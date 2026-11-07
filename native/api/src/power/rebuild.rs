//! Resumable neighbor collection, nearest-six ranking and atomic publication.
use super::*;
use std::ops::Bound::{Excluded, Unbounded};

const SPAN: i32 = ((2 * STEEL.reach) >> 4) + 1;
const SIDE: i32 = SPAN * 2 + 1;
const CELLS: usize = (SIDE * SIDE) as usize;
const QUANTUM: u32 = 64;
type Key = (&'static str, [i32; 3]);
type Node = ([i32; 3], bool);
#[cfg(test)]
thread_local! { static TEST_WORK: std::cell::Cell<u64> = const { std::cell::Cell::new(0) }; }

fn cell(pos: [i32; 3], index: usize) -> (i32, i32) {
    let (x, z) = chunk(pos);
    (x + index as i32 / SIDE - SPAN, z + index as i32 % SIDE - SPAN)
}

fn versions(dimension: &str, pos: [i32; 3]) -> [u64; CELLS] {
    let Ok(all) = POLES.lock() else { return [0; CELLS] };
    let chunks = all.iter().find(|(name, _)| name == dimension).map(|(_, chunks)| chunks);
    std::array::from_fn(|index| chunks.and_then(|chunks| chunks.get(&cell(pos, index))).map_or(0, |cell| cell.revision))
}

fn next(dimension: &str, pos: [i32; 3], index: usize, after: Option<[i32; 3]>) -> Option<Node> {
    let all = POLES.lock().ok()?;
    let (_, chunks) = all.iter().find(|(name, _)| name == dimension)?;
    let nodes = &chunks.get(&cell(pos, index))?.nodes;
    let entry = match after {
        Some(after) => nodes.range((Excluded(after), Unbounded)).next(),
        None => nodes.first_key_value(),
    }?;
    Some((*entry.0, *entry.1))
}

#[derive(Clone, Copy)]
enum Phase { Collect, Rank, Publish, Done }

struct Build {
    steel: bool,
    revisions: [u64; CELLS],
    phase: Phase,
    cell: usize,
    after: Option<[i32; 3]>,
    nodes: Vec<Node>,
    own: Vec<(i32, [i32; 3])>, // At most six entries; no full candidate sort.
    target: usize,
    peer: usize,
    before: usize,
    result: BTreeMap<[i32; 3], bool>,
    published: Option<[i32; 3]>,
    output: Vec<i32>,
    same: bool,
    seen: u64,
    polled: i64,
    #[cfg(test)]
    operations: u64,
}

impl Build {
    fn new(dimension: &str, pos: [i32; 3], steel: bool, time: i64) -> Self {
        Self { steel, revisions: versions(dimension, pos), phase: Phase::Collect,
            cell: 0, after: None, nodes: Vec::new(), own: Vec::with_capacity(MAX_LINKS),
            target: 0, peer: 0, before: 0, result: BTreeMap::new(), published: None,
            output: Vec::new(), same: true, seen: defense::clock(), polled: time,
            #[cfg(test)] operations: 0 }
    }

    fn remember(&mut self, pos: [i32; 3], node: Node) {
        let (at, steel) = node;
        let distance = distance2(pos, at);
        if distance <= reach(self.steel).min(reach(steel)).pow(2) {
            let ranked = (distance, at);
            let index = self.own.partition_point(|entry| *entry < ranked);
            if index < MAX_LINKS {
                if self.own.len() == MAX_LINKS { self.own.pop(); }
                self.own.insert(index, ranked);
            }
        }
        self.nodes.push(node);
    }

    // Each iteration handles one registry candidate/empty chunk, one rank pair,
    // or one output endpoint. No loop over an entire dense neighborhood.
    fn run(&mut self, host: &dyn Host, pos: [i32; 3], entity: &ModBlockEntity, units: u32) -> bool {
        for _ in 0..units {
            #[cfg(test)] {
                self.operations += 1;
                TEST_WORK.with(|work| work.set(work.get() + 1));
            }
            match self.phase {
                Phase::Collect => {
                    if self.cell == CELLS { self.phase = Phase::Rank; continue; }
                    if let Some(node @ (at, steel)) = next(host.dimension(), pos, self.cell, self.after) {
                        self.after = Some(at);
                        if at == pos || distance2(pos, at) > (2 * STEEL.reach).pow(2) { continue; }
                        if host.block(at) == if steel { STEEL_POLE } else { POLE } {
                            self.remember(pos, node);
                        } else { forget(host.dimension(), at); }
                    } else {
                        self.cell += 1;
                        self.after = None;
                    }
                }
                Phase::Rank => {
                    let Some(&(center, steel)) = self.nodes.get(self.target) else {
                        self.phase = Phase::Publish; continue;
                    };
                    let target_distance = distance2(center, pos);
                    let own = self.own.iter().any(|(_, at)| *at == center);
                    if own || self.peer == self.nodes.len() {
                        if own || self.before < MAX_LINKS { self.result.insert(center, steel); }
                        self.target += 1; self.peer = 0; self.before = 0;
                        continue;
                    }
                    if target_distance > reach(steel).min(reach(self.steel)).pow(2) {
                        self.target += 1; self.peer = 0; self.before = 0; continue;
                    }
                    let (at, other_steel) = self.nodes[self.peer];
                    self.peer += 1;
                    if at == center { continue; }
                    let distance = distance2(center, at);
                    if distance <= reach(steel).min(reach(other_steel)).pow(2)
                        && (distance, at) < (target_distance, pos) {
                        self.before += 1;
                        if self.before == MAX_LINKS {
                            self.target += 1; self.peer = 0; self.before = 0;
                        }
                    }
                }
                Phase::Publish => {
                    let at = match self.published {
                        Some(after) => self.result.range((Excluded(after), Unbounded)).next().map(|(at, steel)| (*at, *steel)),
                        None => self.result.first_key_value().map(|(at, steel)| (*at, *steel)),
                    };
                    let Some((at, steel)) = at else {
                        self.same &= matches!(entity.nbt.get(nbt_key("Links")), Some(Tag::IntArray(old)) if old.len() == self.output.len());
                        self.phase = Phase::Done;
                        return true;
                    };
                    // Do not publish a removed/replaced endpoint from an old snapshot.
                    if host.block(at) != if steel { STEEL_POLE } else { POLE } {
                        forget(host.dimension(), at);
                        self.revisions = [u64::MAX; CELLS];
                        return false;
                    }
                    let offset = std::array::from_fn::<_, 3, _>(|axis| at[axis] - pos[axis]);
                    self.same &= matches!(entity.nbt.get(nbt_key("Links")), Some(Tag::IntArray(old))
                        if old.get(self.output.len()..self.output.len() + 3) == Some(offset.as_slice()));
                    self.output.extend_from_slice(&offset);
                    self.published = Some(at);
                }
                Phase::Done => return true,
            }
        }
        false
    }
}

#[derive(Default)]
struct Pool { jobs: BTreeMap<Key, Option<Build>>, cursor: Option<Key> }

#[cfg(not(test))]
fn with_pool<T>(run: impl FnOnce(&mut Pool) -> T) -> T {
    static POOL: std::sync::LazyLock<Mutex<Pool>> = std::sync::LazyLock::new(|| Mutex::new(Pool::default()));
    run(&mut POOL.lock().unwrap_or_else(|error| error.into_inner()))
}
#[cfg(test)]
fn with_pool<T>(run: impl FnOnce(&mut Pool) -> T) -> T {
    thread_local! { static POOL: std::cell::RefCell<Pool> = Default::default(); }
    POOL.with(|pool| run(&mut pool.borrow_mut()))
}

pub(super) fn maintenance(now: u64) {
    with_pool(|pool| {
        // Bounded cleanup: never traverse every in-flight build in a single tick.
        for _ in 0..8 {
            let key = match pool.cursor {
                Some(after) => pool.jobs.range((Excluded(after), Unbounded)).next().map(|(key, _)| *key),
                None => pool.jobs.first_key_value().map(|(key, _)| *key),
            };
            let Some(key) = key else { pool.cursor = None; break; };
            pool.cursor = Some(key);
            if pool.jobs.get(&key).is_some_and(|slot| slot.as_ref().is_none_or(|job| now < job.seen || now.saturating_sub(job.seen) > 20)) {
                pool.jobs.remove(&key);
            }
        }
    });
}

pub(super) fn poll(host: &dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity, steel: bool, time: i64) -> bool {
    let key = (host.dimension(), pos);
    let allowed = budget::request(budget::Lane::Topology, key.0, pos, QUANTUM);
    let job = with_pool(|pool| {
        let reset = pool.jobs.get(&key).and_then(Option::as_ref).is_some_and(|job|
            job.steel != steel || time < job.polled || time.saturating_sub(job.polled) > 2);
        if reset && let Some(slot) = pool.jobs.get_mut(&key) { *slot = None; }
        if let Some(job) = pool.jobs.get_mut(&key).and_then(Option::as_mut) { job.seen = defense::clock(); job.polled = time; }
        // Keep the map node allocated while the job runs outside the mutex.
        if allowed { pool.jobs.entry(key).or_default().take() } else { None }
    });
    if !allowed { return false; }
    let mut job = job.unwrap_or_else(|| {
        // Saved pending work has no in-memory job after restarting the game.
        register(key.0, pos, steel);
        Build::new(key.0, pos, steel, time)
    });
    if job.revisions != versions(key.0, pos) { job = Build::new(key.0, pos, steel, time); }
    let complete = job.run(host, pos, entity, QUANTUM);
    if job.revisions != versions(key.0, pos) {
        with_pool(|pool| { *pool.jobs.entry(key).or_default() = Some(Build::new(key.0, pos, steel, time)); });
        return false;
    }
    if complete {
        with_pool(|pool| { pool.jobs.remove(&key); });
        set_number(entity, "TopologyPending", 0);
        let changed = !job.same;
        if changed {
            let value = Tag::IntArray(job.output);
            if let Some(tag) = entity.nbt.get_mut(nbt_key("Links")) { *tag = value; }
            else { entity.nbt.insert(nbt_key("Links").clone(), value); }
        }
        return changed;
    }
    with_pool(|pool| { *pool.jobs.entry(key).or_default() = Some(job); });
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    struct Grid { dimension: &'static str, nodes: BTreeMap<[i32; 3], bool>, reads: Cell<usize> }
    impl Host for Grid {
        fn dimension(&self) -> &'static str { self.dimension }
        fn block(&self, pos: [i32; 3]) -> String {
            self.reads.set(self.reads.get() + 1);
            self.nodes.get(&pos).map_or("minecraft:air", |steel| if *steel { STEEL_POLE } else { POLE }).into()
        }
    }

    fn expected(grid: &Grid, pos: [i32; 3], steel: bool) -> Vec<i32> {
        let snapshot: Vec<_> = grid.nodes.iter().filter(|(at, _)| **at != pos && distance2(**at, pos) <= (2 * STEEL.reach).pow(2))
            .map(|(at, steel)| (*at, *steel)).collect();
        links(pos, steel, |center| snapshot.iter().copied().chain(std::iter::once((pos, steel)))
            .filter(|(at, _)| *at != center && distance2(*at, center) <= (2 * STEEL.reach).pow(2)).collect::<Vec<_>>())
            .into_iter().flat_map(|at| (0..3).map(move |axis| at[axis] - pos[axis])).collect()
    }

    fn fixture(dimension: &'static str) -> Grid {
        Grid { dimension, reads: Cell::new(0), nodes: (-5i32..=5).flat_map(|x| (-5i32..=5)
            .map(move |z| ([x, (x + z).rem_euclid(3) + 64, z], (x - z) % 3 == 0))).collect() }
    }

    #[test]
    fn simultaneous_dense_rebuilds_share_the_global_work_limit_and_all_finish() {
        let grid = fixture("veio:shared-topology-quanta");
        let mut poles: Vec<_> = grid.nodes.iter().map(|(&pos, &steel)| {
            register(grid.dimension, pos, steel);
            let mut entity = ModBlockEntity::default();
            entity.id = if steel { STEEL_POLE } else { POLE }.into();
            set_number(&mut entity, "TopologyPending", 1);
            (pos, steel, entity)
        }).collect();
        let mut finished = false;
        for tick in 0..2000 {
            budget::begin_tick(tick);
            let before = TEST_WORK.with(Cell::get);
            grid.reads.set(0);
            for (pos, steel, entity) in &mut poles {
                if number(entity, "TopologyPending") != 0 { poll(&grid, *pos, entity, *steel, tick as i64); }
            }
            assert!(TEST_WORK.with(Cell::get) - before <= 4096);
            assert!(grid.reads.get() <= 4096);
            if poles.iter().all(|(_, _, entity)| number(entity, "TopologyPending") == 0) { finished = true; break; }
        }
        assert!(finished);
        for (pos, steel, entity) in &poles {
            assert!(matches!(entity.nbt.get(nbt_key("Links")), Some(Tag::IntArray(output)) if *output == expected(&grid, *pos, *steel)));
        }
    }

    #[test]
    fn tiny_steps_match_previous_solver_for_dense_mixed_tiers_ties_and_boundary_ranges() {
        let mut grid = fixture("veio:rank-quanta");
        grid.nodes.insert([16, 64, 0], true);
        grid.nodes.insert([17, 64, 0], true);
        grid.nodes.insert([32, 64, 0], false);
        for (pos, steel) in [([0, 64, 0], false), ([0, 64, 0], true), ([8, 64, 0], true), ([-8, 64, 0], false)] {
            let mut job = Build::new(grid.dimension, pos, steel, 0);
            job.phase = Phase::Rank;
            for (&at, &tier) in &grid.nodes {
                if at != pos && distance2(at, pos) <= (2 * STEEL.reach).pow(2) { job.remember(pos, (at, tier)); }
            }
            let entity = ModBlockEntity::default();
            let mut finished = false;
            for _ in 0..100000 {
                let before = job.operations;
                grid.reads.set(0);
                finished = job.run(&grid, pos, &entity, 3);
                assert!(job.operations - before <= 3);
                assert!(grid.reads.get() <= 3);
                if finished { break; }
            }
            assert!(finished);
            assert_eq!(job.output, expected(&grid, pos, steel));
        }
    }

    #[test]
    fn real_collection_is_bounded_and_mid_build_removal_never_publishes_stale_endpoint() {
        let mut grid = fixture("veio:collect-quanta");
        let pos = [0, 64, 0];
        for (&at, &steel) in &grid.nodes { register(grid.dimension, at, steel); }
        register(grid.dimension, pos, true);
        grid.nodes.insert(pos, true);
        let removed = [1, 65, 0];
        let mut entity = ModBlockEntity::default();
        entity.id = STEEL_POLE.into();
        entity.nbt.insert(nbt_key("Links").clone(), Tag::IntArray(vec![7, 0, 0]));
        set_number(&mut entity, "TopologyPending", 1);
        let mut finished = false;
        for tick in 0..10000 {
            budget::begin_tick(tick);
            grid.reads.set(0);
            let changed = poll(&grid, pos, &mut entity, true, tick as i64);
            assert!(grid.reads.get() <= QUANTUM as usize);
            if tick == 1 { grid.nodes.remove(&removed); }
            if number(&entity, "TopologyPending") == 0 { assert!(changed); finished = true; break; }
            assert!(matches!(entity.nbt.get(nbt_key("Links")), Some(Tag::IntArray(old)) if old == &[7, 0, 0]));
        }
        assert!(finished);
        assert!(matches!(entity.nbt.get(nbt_key("Links")), Some(Tag::IntArray(result)) if *result == expected(&grid, pos, true)));
        assert!(!offsets(&entity, "Links").any(|offset| add(pos, offset) == removed));
    }

    #[test]
    fn chunk_revision_change_restarts_partial_work_and_unloaded_jobs_are_reclaimed() {
        let mut grid = fixture("veio:revision-quanta");
        let pos = [0, 64, 0];
        for (&at, &steel) in &grid.nodes { register(grid.dimension, at, steel); }
        let mut entity = ModBlockEntity::default();
        entity.id = STEEL_POLE.into();
        set_number(&mut entity, "TopologyPending", 1);
        budget::begin_tick(0);
        poll(&grid, pos, &mut entity, true, 0);
        grid.nodes.insert([0, 64, 1], true);
        register(grid.dimension, [0, 64, 1], true);
        for time in 1..10000 {
            budget::begin_tick(time);
            poll(&grid, pos, &mut entity, true, time as i64);
            if number(&entity, "TopologyPending") == 0 { break; }
        }
        assert_eq!(number(&entity, "TopologyPending"), 0);
        assert!(matches!(entity.nbt.get(nbt_key("Links")), Some(Tag::IntArray(result)) if *result == expected(&grid, pos, true)));
        let job = Build::new(grid.dimension, pos, true, 0);
        with_pool(|pool| { pool.jobs.insert((grid.dimension, pos), Some(job)); });
        maintenance(defense::clock() + 21);
        assert!(with_pool(|pool| pool.jobs.is_empty()));
    }
}
