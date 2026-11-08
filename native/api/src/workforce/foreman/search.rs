//! Spatial, resumable queue discovery; existing claims use a bounded direct path.
use super::*;
use std::collections::BTreeMap;
use std::ops::Bound::{Excluded, Included, Unbounded};

const QUANTUM: u32 = 32;
const CLAIM_UNITS: u32 = MAX_QUEUE as u32 + 1;
const _: () = assert!(CLAIM_UNITS <= QUANTUM);
type Key = (&'static str, [i32; 3]);
type DeskKey = (&'static str, i32, i32, [i32; 3]);

#[derive(Default)]
struct Registry { desks: BTreeMap<DeskKey, u64>, sweep: Option<DeskKey> }
#[derive(Clone, Copy)]
struct Bookmark { desk: [i32; 3], id: i32, index: usize }
#[derive(Clone, Copy)]
struct Candidate { id: i32, index: usize }
enum Phase { Region, Tasks { desk: [i32; 3], index: usize, first: Option<Candidate> }, Claim { desk: [i32; 3], candidate: Candidate } }
struct State {
    phase: Phase,
    segment: usize,
    after: Option<[i32; 3]>,
    anchor: Option<(usize, [i32; 3])>,
    bookmark: Option<Bookmark>,
    next_scan: u64,
    seen: u64,
}
impl State {
    fn new(now: u64) -> Self {
        Self { phase: Phase::Region, segment: 0, after: None, anchor: None, bookmark: None, next_scan: 0, seen: now }
    }
    fn reset(&mut self) {
        self.phase = Phase::Region; self.segment = 0;
        self.after = self.anchor.map(|(_, pos)| pos);
    }
    fn load(now: u64, hauler: [i32; 3], saved: Option<&ModBlockEntity>) -> Self {
        let mut state = Self::new(now);
        if let Some(saved) = saved
            && let Some(desk) = int_array(saved.nbt.get(nbt_key("HaulTaskDesk")))
            && let id = number(saved, "HaulTaskId")
            && id > 0 && in_range(desk, hauler, DESK_RANGE) {
            state.bookmark = Some(Bookmark { desk, id, index: 0 });
        }
        state
    }
}
#[derive(Default)]
struct Pool { states: BTreeMap<Key, Option<State>>, sweep: Option<Key> }

#[cfg(not(test))]
fn registry<T>(run: impl FnOnce(&mut Registry) -> T) -> T {
    static REGISTRY: std::sync::LazyLock<Mutex<Registry>> = std::sync::LazyLock::new(|| Mutex::new(Registry::default()));
    run(&mut REGISTRY.lock().unwrap_or_else(|error| error.into_inner()))
}
#[cfg(test)]
fn registry<T>(run: impl FnOnce(&mut Registry) -> T) -> T {
    thread_local! { static REGISTRY: std::cell::RefCell<Registry> = Default::default(); }
    REGISTRY.with(|registry| run(&mut registry.borrow_mut()))
}
#[cfg(not(test))]
fn pool<T>(run: impl FnOnce(&mut Pool) -> T) -> T {
    static POOL: std::sync::LazyLock<Mutex<Pool>> = std::sync::LazyLock::new(|| Mutex::new(Pool::default()));
    run(&mut POOL.lock().unwrap_or_else(|error| error.into_inner()))
}
#[cfg(test)]
fn pool<T>(run: impl FnOnce(&mut Pool) -> T) -> T {
    thread_local! { static POOL: std::cell::RefCell<Pool> = Default::default(); }
    POOL.with(|pool| run(&mut pool.borrow_mut()))
}

fn desk_key(dimension: &'static str, pos: [i32; 3]) -> DeskKey { (dimension, pos[0] >> 4, pos[2] >> 4, pos) }
pub(super) fn register(dimension: &'static str, pos: [i32; 3], now: u64) {
    registry(|registry| { registry.desks.insert(desk_key(dimension, pos), now); });
}
fn recent(dimension: &'static str, pos: [i32; 3], now: u64) -> bool {
    registry(|registry| registry.desks.get(&desk_key(dimension, pos)).is_some_and(|seen| *seen <= now && now.saturating_sub(*seen) <= TASK_STALE))
}
fn forget(dimension: &'static str, pos: [i32; 3]) { registry(|registry| { registry.desks.remove(&desk_key(dimension, pos)); }); }
fn in_range(a: [i32; 3], b: [i32; 3], radius: f64) -> bool {
    (0..3).map(|axis| (f64::from(a[axis]) - f64::from(b[axis])).powi(2)).sum::<f64>() <= radius * radius
}

struct View { id: i32, from: [i32; 3], to: [i32; 3], by: Option<([i32; 3], u64)> }
fn view(tag: &Tag) -> Option<View> {
    let Tag::Compound(tag) = tag else { return None };
    if !matches!(tag.get(nbt_key("item")), Some(Tag::String(_))) { return None; }
    Some(View { id: int(tag.get(nbt_key("id"))), from: int_array(tag.get(nbt_key("from")))?, to: int_array(tag.get(nbt_key("to")))?,
        by: int_array(tag.get(nbt_key("by"))).map(|by| (by, match tag.get(nbt_key("seen")) { Some(Tag::Long(seen)) => *seen as u64, _ => 0 })) })
}
fn mine(task: &View, hauler: [i32; 3], now: u64) -> bool { task.by.is_some_and(|(by, seen)| by == hauler && now.saturating_sub(seen) <= TASK_STALE) }
fn free(task: &View, hauler: [i32; 3], now: u64) -> bool {
    task.by.is_none_or(|(_, seen)| now.saturating_sub(seen) > TASK_STALE)
        && in_range(task.from, hauler, HAUL_RANGE) && in_range(task.to, hauler, HAUL_RANGE)
}

// A claim changes only ownership fields. Never rebuild/clone the whole queue.
fn claim(table: &mut ModBlockEntity, hauler: [i32; 3], now: u64, wanted: Candidate, own_only: bool) -> Option<(Plan, Bookmark)> {
    if table.id != FOREMAN_DESK { return None; }
    let Some(Tag::List(list)) = table.nbt.get_mut(nbt_key("Queue")) else { return None };
    let mut owned = own_only.then(|| list.get(wanted.index).and_then(view)
        .filter(|task| task.id == wanted.id && mine(task, hauler, now)).map(|_| wanted.index)).flatten();
    let mut candidate = None;
    // Bounded critical section: handle shifted indices and give an existing claim
    // priority over a free task even if the queue changed during discovery.
    for (index, tag) in list.iter().take(MAX_QUEUE).enumerate() {
        if owned.is_some() { break; }
        let Some(task) = view(tag) else { continue };
        if mine(&task, hauler, now) && (!own_only || task.id == wanted.id) { owned = Some(index); break; }
        if task.id == wanted.id && free(&task, hauler, now) { candidate = Some(index); }
    }
    let index = if own_only { owned } else { owned.or(candidate) }?;
    let task = decode_task(&list[index])?;
    let Tag::Compound(tag) = &mut list[index] else { return None };
    if int_array(tag.get(nbt_key("by"))) != Some(hauler) {
        tag.insert(nbt_key("by").clone(), Tag::IntArray(hauler.to_vec()));
    }
    if let Some(value) = tag.get_mut(nbt_key("seen")) { *value = Tag::Long(now as i64); }
    else { tag.insert(nbt_key("seen").clone(), Tag::Long(now as i64)); }
    Some((Plan::Fetch { desk: [0; 3], task: task.id, from: task.from, slot: task.slot, item: task.item,
        count: task.count, to: task.to, into: task.into }, Bookmark { desk: [0; 3], id: task.id, index }))
}

fn claim_at(host: &mut dyn Host, desk: [i32; 3], hauler: [i32; 3], now: u64, candidate: Candidate, own_only: bool) -> Option<(Plan, Bookmark)> {
    let mut result = None;
    host.change_mod_block_entity(desk, &mut |table| { result = claim(table, hauler, now, candidate, own_only); });
    if let Some((Plan::Fetch { desk: found, .. }, bookmark)) = &mut result { *found = desk; bookmark.desk = desk; }
    result
}

fn run(host: &mut dyn Host, hauler: [i32; 3], state: &mut State, now: u64) -> TaskSearch {
    let dimension = host.dimension();
    if let Some(bookmark) = state.bookmark {
        // A bounded direct renewal cannot be blocked by idle discovery traffic.
        if in_range(bookmark.desk, hauler, DESK_RANGE)
            && let Some((plan, bookmark)) = claim_at(host, bookmark.desk, hauler, now,
                Candidate { id: bookmark.id, index: bookmark.index }, true) {
            state.bookmark = Some(bookmark);
            return TaskSearch::Found(plan);
        }
        state.bookmark = None;
        state.reset();
    }
    if now < state.next_scan { return TaskSearch::Empty; }
    if !budget::request(budget::Lane::HaulSearch, dimension, hauler, QUANTUM) { return TaskSearch::Pending; }
    let radius = DESK_RANGE as i32;
    let min = (hauler[0].saturating_sub(radius) >> 4, hauler[2].saturating_sub(radius) >> 4);
    let width = (hauler[0].saturating_add(radius) >> 4) - min.0 + 1;
    let depth = (hauler[2].saturating_add(radius) >> 4) - min.1 + 1;
    let cells = (width * depth) as usize;
    let mut left = QUANTUM;
    while left > 0 {
        let cost = if matches!(state.phase, Phase::Claim { .. }) { CLAIM_UNITS } else { 1 };
        if left < cost { break; }
        left -= cost;
        #[cfg(test)] TEST_WORK.with(|work| work.set(work.get() + u64::from(cost)));
        match state.phase {
            Phase::Region => {
                if state.segment >= cells + usize::from(state.anchor.is_some()) {
                    state.reset(); state.next_scan = now.saturating_add(EVERY as u64);
                    return TaskSearch::Empty;
                }
                let cell = (state.anchor.map_or(0, |(cell, _)| cell) + state.segment) % cells;
                let region = (min.0 + cell as i32 / depth, min.1 + cell as i32 % depth);
                let next = registry(|registry| {
                    let lower = match state.after {
                        Some(after) => Excluded((dimension, region.0, region.1, after)),
                        None => Included((dimension, region.0, region.1, [i32::MIN; 3])),
                    };
                    registry.desks.range((lower, Unbounded)).next().filter(|(key, _)|
                        key.0 == dimension && (key.1, key.2) == region).map(|(key, seen)| (key.3, *seen))
                });
                if let Some((desk, seen)) = next {
                    if state.segment == cells && state.anchor.is_some_and(|(_, anchor)| desk > anchor) {
                        state.segment += 1; continue;
                    }
                    state.after = Some(desk);
                    if seen > now || now.saturating_sub(seen) > TASK_STALE { forget(dimension, desk); continue; }
                    if in_range(desk, hauler, DESK_RANGE) { state.phase = Phase::Tasks { desk, index: 0, first: None }; }
                } else { state.segment += 1; state.after = None; }
            }
            Phase::Tasks { desk, index, mut first } => {
                if index >= MAX_QUEUE {
                    state.phase = first.map_or(Phase::Region, |candidate| Phase::Claim { desk, candidate }); continue;
                }
                let mut found = None;
                let mut valid = false;
                host.change_mod_block_entity(desk, &mut |table| {
                    if table.id != FOREMAN_DESK { return; }
                    valid = true;
                    if let Some(Tag::List(list)) = table.nbt.get(nbt_key("Queue")) {
                        found = list.get(index).map(view);
                    }
                });
                if !valid { forget(dimension, desk); state.phase = Phase::Region; continue; }
                match found {
                    None => state.phase = first.map_or(Phase::Region, |candidate| Phase::Claim { desk, candidate }),
                    Some(task) => {
                        if let Some(task) = task {
                            let candidate = Candidate { id: task.id, index };
                            if mine(&task, hauler, now) { state.phase = Phase::Claim { desk, candidate }; continue; }
                            if first.is_none() && free(&task, hauler, now) { first = Some(candidate); }
                        }
                        state.phase = Phase::Tasks { desk, index: index + 1, first };
                    }
                }
            }
            Phase::Claim { desk, candidate } => {
                state.phase = Phase::Region;
                if !recent(dimension, desk, now) { continue; }
                if let Some((plan, bookmark)) = claim_at(host, desk, hauler, now, candidate, false) {
                    let cell = (state.anchor.map_or(0, |(cell, _)| cell) + state.segment) % cells;
                    state.anchor = Some((cell, desk)); state.bookmark = Some(bookmark); state.reset();
                    return TaskSearch::Found(plan);
                }
            }
        }
    }
    TaskSearch::Pending
}

pub(super) fn poll(host: &mut dyn Host, hauler: [i32; 3], now: u64, saved: Option<&ModBlockEntity>) -> TaskSearch {
    let key = (host.dimension(), hauler);
    let mut state = pool(|pool| pool.states.entry(key).or_default().take()).unwrap_or_else(|| State::load(now, hauler, saved));
    if now < state.seen || now.saturating_sub(state.seen) > TASK_STALE { state = State::load(now, hauler, saved); }
    state.seen = now;
    let result = run(host, hauler, &mut state, now);
    pool(|pool| { *pool.states.entry(key).or_default() = Some(state); });
    result
}

pub(super) fn maintenance(now: u64) {
    registry(|registry| {
        for _ in 0..8 {
            let key = match registry.sweep {
                Some(after) => registry.desks.range((Excluded(after), Unbounded)).next().map(|(key, _)| *key),
                None => registry.desks.first_key_value().map(|(key, _)| *key),
            };
            let Some(key) = key else { registry.sweep = None; break; };
            registry.sweep = Some(key);
            if registry.desks.get(&key).is_some_and(|seen| now < *seen || now.saturating_sub(*seen) > TASK_STALE) { registry.desks.remove(&key); }
        }
    });
    pool(|pool| {
        for _ in 0..8 {
            let key = match pool.sweep {
                Some(after) => pool.states.range((Excluded(after), Unbounded)).next().map(|(key, _)| *key),
                None => pool.states.first_key_value().map(|(key, _)| *key),
            };
            let Some(key) = key else { pool.sweep = None; break; };
            pool.sweep = Some(key);
            if pool.states.get(&key).is_some_and(|slot| slot.as_ref().is_none_or(|state| now < state.seen || now.saturating_sub(state.seen) > TASK_STALE)) {
                pool.states.remove(&key);
            }
        }
    });
}

#[cfg(test)]
thread_local! { static TEST_WORK: std::cell::Cell<u64> = const { std::cell::Cell::new(0) }; }

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    struct World { nodes: BTreeMap<[i32; 3], ModBlockEntity>, reads: usize }
    impl Host for World {
        fn dimension(&self) -> &'static str { "veio:desk-search-tests" }
        fn change_mod_block_entity(&mut self, pos: [i32; 3], change: &mut dyn FnMut(&mut ModBlockEntity)) -> bool {
            self.reads += 1;
            let Some(node) = self.nodes.get_mut(&pos) else { return false; };
            change(node); true
        }
    }
    fn task(id: i32) -> Task {
        Task { id, from: [1,64,1], slot: 1, item: PLATE.into(), count: 8, to: [2,64,2], into: 0, by: None }
    }
    fn add(world: &mut World, desk: [i32; 3], tasks: &[Task], now: u64) {
        let mut table = ModBlockEntity::default(); table.id = FOREMAN_DESK.into();
        set_queue(&mut table, tasks); world.nodes.insert(desk, table); register(world.dimension(), desk, now);
    }
    fn empty() -> World { World { nodes: BTreeMap::new(), reads: 0 } }
    fn find(world: &mut World, hauler: [i32; 3], now: u64) -> TaskSearch { poll(world, hauler, now, None) }

    #[test]
    fn many_searches_obey_global_quota_and_all_finish() {
        let mut world = empty();
        for x in -7..=7 { for z in -7..=7 { add(&mut world, [x,70,z], &[], 0); } }
        let mut done = [false; 64];
        for now in 0..200 {
            budget::begin_tick(now);
            let start = TEST_WORK.with(Cell::get); let reads = world.reads;
            for (i, done) in done.iter_mut().enumerate() {
                if *done { continue; }
                let before = TEST_WORK.with(Cell::get);
                *done = matches!(find(&mut world, [i as i32 % 8,64,i as i32 / 8], now), TaskSearch::Empty);
                assert!(TEST_WORK.with(Cell::get) - before <= u64::from(QUANTUM));
            }
            assert!(TEST_WORK.with(Cell::get) - start <= 1024);
            assert!(world.reads - reads <= 1024);
            if done.iter().all(|done| *done) { break; }
        }
        assert!(done.iter().all(|done| *done));
        assert!(world.reads > 6400);
    }

    #[test]
    fn ten_thousand_far_desks_do_not_add_local_work_and_empty_results_are_cached() {
        let mut world = empty();
        for i in 0..10000 { register(world.dimension(), [1000 + i * 16,64,0], 0); }
        let start = TEST_WORK.with(Cell::get);
        assert!(matches!(find(&mut world, [0,64,0], 0), TaskSearch::Empty));
        assert!(TEST_WORK.with(Cell::get) - start <= 10);
        assert_eq!(world.reads, 0);
        let start = TEST_WORK.with(Cell::get);
        assert!(matches!(find(&mut world, [0,64,0], 1), TaskSearch::Empty));
        assert_eq!(TEST_WORK.with(Cell::get), start);
    }

    #[test]
    fn existing_claim_renews_with_one_read_despite_exhausted_discovery_budget() {
        let mut world = empty(); let desk = [1,65,0]; let hauler = [0,64,0];
        add(&mut world, desk, &[task(1)], 0);
        let Some(Tag::List(list)) = world.nodes.get_mut(&desk).unwrap().nbt.get_mut(nbt_key("Queue")) else { panic!() };
        let Tag::Compound(tag) = &mut list[0] else { panic!() };
        tag.insert("ExtensionField".into(), Tag::String("preserved".into()));
        budget::begin_tick(0);
        assert!(matches!(find(&mut world, hauler, 0), TaskSearch::Found(Plan::Fetch { task: 1, .. })));
        budget::begin_tick(1);
        assert!(budget::request(budget::Lane::HaulSearch, world.dimension(), [999;3], 1024));
        let reads = world.reads;
        assert!(matches!(find(&mut world, hauler, 1), TaskSearch::Found(Plan::Fetch { task: 1, .. })));
        assert_eq!(world.reads - reads, 1);
        assert_eq!(queue(&world.nodes[&desk])[0].by, Some((hauler,1)));
        let Some(Tag::List(list)) = world.nodes[&desk].nbt.get(nbt_key("Queue")) else { panic!() };
        assert!(matches!(&list[0], Tag::Compound(tag) if tag.contains_key(&"ExtensionField".into())));
    }

    #[test]
    fn final_claim_rechecks_ownership_and_shifted_indices() {
        let mut world = empty(); let desk = [1,65,0]; let hauler = [0,64,0];
        let mut other = task(1); other.by = Some(([5,64,5],0));
        add(&mut world, desk, &[other.clone()], 0);
        assert!(claim_at(&mut world, desk, hauler, 1, Candidate { id: 1,index: 0 }, false).is_none());
        assert_eq!(queue(&world.nodes[&desk]), [other]);
        set_queue(world.nodes.get_mut(&desk).unwrap(), &[task(9),task(2)]);
        let result = claim_at(&mut world, desk, hauler, 1, Candidate { id: 2,index: 0 }, false);
        assert!(matches!(result, Some((Plan::Fetch { task: 2, .. }, Bookmark { index: 1, .. }))));
        assert_eq!(queue(&world.nodes[&desk])[0].by, None);
        assert!(matches!(claim_at(&mut world, desk, hauler, 2, Candidate { id: 9,index: 0 }, false), Some((Plan::Fetch { task: 2, .. }, _))));
    }

    #[test]
    fn mesas_rotate_after_completion_without_creating_duplicate_claims() {
        let mut world = empty(); let hauler = [0,64,0]; let desks = [[1,65,0],[2,65,0],[3,65,0]];
        for desk in desks { add(&mut world, desk, &[task(1)], 0); }
        for now in 0..6 {
            let TaskSearch::Found(Plan::Fetch { desk, task:id,.. }) = find(&mut world, hauler, now) else { panic!("expected task in round {now}") };
            assert_eq!(desk, desks[now as usize % desks.len()]);
            complete(&mut world, desk, id);
            set_queue(world.nodes.get_mut(&desk).unwrap(), &[task(id + 1)]);
        }
    }

    #[test]
    fn saved_claim_restores_without_a_registry_heartbeat_and_never_resurrects_completion() {
        let mut world = empty(); let desk = [1,65,0]; let hauler = [0,64,0];
        add(&mut world, desk, &[task(1)], 0);
        let mut post = ModBlockEntity::default(); post.id = HAULER_POST.into();
        assert!(matches!(search_task(&mut world, hauler, &mut post, 0), TaskSearch::Found(_)));
        pool(|pool| pool.states.clear()); registry(|registry| registry.desks.clear());
        assert!(matches!(search_task(&mut world, hauler, &mut post, 1), TaskSearch::Found(Plan::Fetch { task: 1, .. })));
        complete(&mut world, desk, 1);
        pool(|pool| pool.states.clear());
        assert!(matches!(search_task(&mut world, hauler, &mut post, 2), TaskSearch::Empty));
        assert!(!post.nbt.contains_key(nbt_key("HaulTaskDesk")));
        assert!(queue(&world.nodes[&desk]).is_empty());
    }

    #[test]
    fn replaced_desks_are_removed_and_registry_cleanup_is_bounded() {
        let mut world = empty(); let desk = [1,65,0]; add(&mut world, desk, &[task(1)], 0);
        world.nodes.get_mut(&desk).unwrap().id = PRESS.into();
        assert!(matches!(find(&mut world, [0,64,0], 0), TaskSearch::Empty));
        assert!(!registry(|registry| registry.desks.contains_key(&desk_key(world.dimension(), desk))));
        for i in 0..20 { register(world.dimension(), [100+i,64,0], 0); }
        maintenance(TASK_STALE + 1);
        assert_eq!(registry(|registry| registry.desks.len()), 12);
        for _ in 0..3 { maintenance(TASK_STALE + 1); }
        assert!(registry(|registry| registry.desks.is_empty()));
        assert!(pool(|pool| pool.states.is_empty()));
    }
}
