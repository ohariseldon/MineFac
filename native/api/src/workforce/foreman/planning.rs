//! Resumable queue preparation. Never owns items or mutates an inventory.
use super::*;
use std::collections::BTreeMap;
use std::ops::Bound::{Excluded, Unbounded};

const QUANTUM: u32 = 64;
const COMMIT_UNITS: u32 = (MAX_QUEUE * 2 + 1) as u32;
const _: () = assert!(COMMIT_UNITS <= QUANTUM);
type Key = (&'static str, [i32; 3]);

struct Node {
    pos: [i32; 3],
    entity: ModBlockEntity,
    inputs: Vec<usize>,
    outputs: Vec<usize>,
    store: bool,
}
impl Node {
    fn new(pos: [i32; 3], live: &ModBlockEntity) -> Self {
        // Planning needs inventory and the current HUB contract, not energy,
        // world edits, worker paths or arbitrary machine NBT.
        let mut entity = ModBlockEntity::default();
        entity.id = live.id.clone();
        entity.items = live.items.clone();
        if entity.id == HUB && let Some(stage) = live.nbt.get(nbt_key("ContractStage")) {
            entity.nbt.insert(nbt_key("ContractStage").clone(), stage.clone());
        }
        // The parallel freight branch uses this filter in accepts_plan. Keep
        // its small configuration when that branch is integrated.
        if matches!(entity.id.as_str(), "veio:freight_station" | "veio:drone_port" | "veio:caravan_station")
            && let Some(export) = live.nbt.get(nbt_key("FreightExport")) {
            entity.nbt.insert(nbt_key("FreightExport").clone(), export.clone());
        }
        let inputs = queue_inputs(&entity.id);
        let outputs = queue_outputs(&entity.id);
        let store = storage::is_store(&entity.id);
        Self { pos, entity, inputs, outputs, store }
    }
}
struct Source { pos: [i32; 3], slot: usize, stack: ItemStack, store: bool }
#[derive(Clone, Copy)]
enum Phase { Collect, Sources, Plan, Commit }

struct Job {
    network: Vec<[i32; 3]>,
    collect: usize,
    desk: Node,
    regular: Vec<Node>,
    stores: Vec<Node>,
    posts: Vec<[i32; 3]>,
    pinned: Vec<Task>,
    sources: Vec<Option<Source>>,
    drafts: Vec<Task>,
    phase: Phase,
    node: usize,
    slot: usize,
    source: usize,
    worker: Option<Uuid>,
    seen: u64,
}

impl Job {
    fn new(pos: [i32; 3], entity: &ModBlockEntity, network: Vec<[i32; 3]>, now: u64) -> Self {
        Self { network, collect: 0, desk: Node::new(pos, entity), regular: Vec::new(),
            stores: Vec::new(), posts: Vec::new(),
            pinned: queue(entity).into_iter().filter(|task| fresh(task, now)).collect(),
            sources: Vec::new(), drafts: Vec::new(), phase: Phase::Collect, node: 0, slot: 0,
            source: 0, worker: worker(entity), seen: now }
    }

    fn run(&mut self, host: &mut dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity, now: u64, units: u32) -> bool {
        let mut left = units;
        while left > 0 {
            if matches!(self.phase, Phase::Commit) {
                // Up to eight proposals: validate and publish in one bounded tail,
                // so claims/completions cannot interleave across simulation ticks.
                if left < COMMIT_UNITS { return false; }
                #[cfg(test)] work(COMMIT_UNITS);
                self.publish(host, pos, entity, now);
                return true;
            }
            left -= 1;
            #[cfg(test)] work(1);
            match self.phase {
                Phase::Collect => {
                    let Some(&spot) = self.network.get(self.collect) else {
                        self.phase = Phase::Sources; self.node = 0; self.slot = 0; continue;
                    };
                    self.collect += 1;
                    let mut found = None;
                    host.change_mod_block_entity(spot, &mut |node| { found = Some(Node::new(spot, node)); });
                    if let Some(node) = found {
                        if managed(&node.entity.id) && self.posts.len() < MAX_POSTS { self.posts.push(spot); }
                        if node.store { self.stores.push(node); } else { self.regular.push(node); }
                    }
                }
                Phase::Sources => {
                    let node = if self.node < self.regular.len() { self.regular.get(self.node) }
                        else { self.stores.get(self.node - self.regular.len()) };
                    let Some(node) = node else {
                        self.phase = Phase::Plan; self.node = 0; self.slot = 0; self.source = 0; continue;
                    };
                    let Some(&slot) = node.outputs.get(self.slot) else { self.node += 1; self.slot = 0; continue; };
                    self.slot += 1;
                    if self.pinned.iter().any(|task| task.from == node.pos && task.slot == slot) { continue; }
                    if let Some(stack) = node.entity.items.get(slot).and_then(Option::as_ref).filter(|stack| plain(stack)) {
                        self.sources.push(Some(Source { pos: node.pos, slot, stack: stack.clone(), store: node.store }));
                    }
                }
                Phase::Plan => {
                    if self.pinned.len() + self.drafts.len() >= MAX_QUEUE { self.phase = Phase::Commit; continue; }
                    let sink = if self.node == 0 { Some(&self.desk) }
                        else if self.node <= self.regular.len() { self.regular.get(self.node - 1) }
                        else { self.stores.get(self.node - 1 - self.regular.len()) };
                    let Some(sink) = sink else { self.phase = Phase::Commit; continue; };
                    let Some(&into) = sink.inputs.get(self.slot) else {
                        self.node += 1; self.slot = 0; self.source = 0; continue;
                    };
                    if self.source == 0 {
                        if self.node != 0 && reserved_by_other(host.dimension(), sink.pos, pos, now) {
                            self.node += 1; self.slot = 0; continue;
                        }
                        let tasks = self.pinned.iter().chain(self.drafts.iter());
                        let occupied = tasks.clone().any(|task| task.to == sink.pos && (sink.store || task.into == into));
                        let enough_food = sink.entity.id == FOREMAN_DESK && food_slots(&sink.entity.id).contains(&into)
                            && food_slots(&sink.entity.id).filter_map(|slot| sink.entity.items[slot].as_ref()).map(|stack| stack.count).sum::<u32>() >= DESK_FOOD;
                        if occupied || enough_food { self.slot += 1; continue; }
                    }
                    if self.source == self.sources.len() { self.slot += 1; self.source = 0; continue; }
                    let index = self.source;
                    self.source += 1;
                    let Some(source) = &self.sources[index] else { continue; };
                    if source.pos == sink.pos || (sink.store && source.store) { continue; }
                    let current = sink.entity.items.get(into).and_then(Option::as_ref);
                    if !current.is_none_or(|present| is_plain(present, &source.stack.item) && present.count < top_up(&sink.entity.id))
                        || !operations::accepts_material(&sink.entity, into, &source.stack) { continue; }
                    let count = source.stack.count.min(hauler::BATCH).min(top_up(&sink.entity.id) - current.map_or(0, |stack| stack.count));
                    self.drafts.push(Task { id: 0, from: source.pos, slot: source.slot, item: source.stack.item.to_string(),
                        count, to: sink.pos, into, by: None });
                    self.sources[index] = None;
                    self.slot += 1; self.source = 0;
                }
                Phase::Commit => unreachable!(),
            }
        }
        false
    }

    fn publish(&mut self, host: &mut dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity, now: u64) {
        // Read live claims, not the pinned snapshot. A completed job must never
        // reappear, and a task claimed during preparation must keep its owner.
        let mut tasks: Vec<Task> = queue(entity).into_iter().filter(|task| fresh(task, now)).collect();
        let mut next_id = number(entity, "TaskId");
        for proposal in &self.drafts {
            if tasks.len() >= MAX_QUEUE { break; }
            if tasks.iter().any(|task| task.from == proposal.from && task.slot == proposal.slot)
                || reserved_by_other(host.dimension(), proposal.to, pos, now) { continue; }
            let mut task = proposal.clone();
            let mut available = 0;
            let mut from_store = false;
            host.change_mod_block_entity(task.from, &mut |node| {
                from_store = storage::is_store(&node.id);
                if queue_outputs(&node.id).contains(&task.slot)
                    && let Some(stack) = node.items.get(task.slot).and_then(Option::as_ref)
                    && is_plain(stack, &task.item) { available = stack.count; }
            });
            task.count = task.count.min(available);
            if task.count == 0 { continue; }
            let validate = |sink: &ModBlockEntity| -> u32 {
                if (from_store && storage::is_store(&sink.id)) || !queue_inputs(&sink.id).contains(&task.into) { return 0; }
                if tasks.iter().any(|other| other.to == task.to && (storage::is_store(&sink.id) || other.into == task.into)) { return 0; }
                if sink.id == FOREMAN_DESK && food_slots(&sink.id).contains(&task.into)
                    && food_slots(&sink.id).filter_map(|slot| sink.items[slot].as_ref()).map(|stack| stack.count).sum::<u32>() >= DESK_FOOD { return 0; }
                if !operations::accepts_material(sink, task.into, &ItemStack::new(&task.item, 1)) { return 0; }
                match sink.items.get(task.into) {
                    Some(None) => task.count.min(top_up(&sink.id)),
                    Some(Some(stack)) if is_plain(stack, &task.item) => task.count.min(top_up(&sink.id).saturating_sub(stack.count)),
                    _ => 0,
                }
            };
            let accepted = if task.to == pos { validate(entity) } else {
                let mut count = 0;
                host.change_mod_block_entity(task.to, &mut |sink| { count = validate(sink); });
                count
            };
            if accepted == 0 { continue; }
            task.count = accepted;
            next_id = next_id.checked_add(1).unwrap_or(1);
            while tasks.iter().any(|other| other.id == next_id) { next_id = next_id.checked_add(1).unwrap_or(1); }
            task.id = next_id;
            tasks.push(task);
        }
        set_number(entity, "TaskId", next_id);
        set_queue(entity, &tasks);
        set_list(entity, "Posts", &self.posts);
        set_number(entity, "Queued", tasks.len() as i32);
        set_number(entity, "Utilization", tasks.len() as i32);
        set_number(entity, "PlanningPending", 0);
    }
}

fn fresh(task: &Task, now: u64) -> bool { task.by.is_some_and(|(_, seen)| now.saturating_sub(seen) <= TASK_STALE) }

#[derive(Default)]
struct Pool { jobs: BTreeMap<Key, Option<Job>>, cursor: Option<Key> }
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

pub(super) fn start(dimension: &'static str, pos: [i32; 3], entity: &mut ModBlockEntity, network: Vec<[i32; 3]>, now: u64) {
    with_pool(|pool| { pool.jobs.insert((dimension, pos), Some(Job::new(pos, entity, network, now))); });
    set_number(entity, "PlanningPending", 1);
}

pub(super) fn advance(host: &mut dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity, now: u64) {
    let key = (host.dimension(), pos);
    let allowed = budget::request(budget::Lane::Planning, key.0, pos, QUANTUM);
    let job = with_pool(|pool| {
        if let Some(slot) = pool.jobs.get_mut(&key) {
            if slot.as_ref().is_some_and(|job| job.worker != worker(entity) || now < job.seen || now.saturating_sub(job.seen) > 20) { *slot = None; }
            if let Some(job) = slot.as_mut() { job.seen = now; }
        }
        if allowed { pool.jobs.entry(key).or_default().take() } else { None }
    });
    if !allowed { return; }
    let mut job = job.unwrap_or_else(|| Job::new(pos, entity, get_list(entity, "Area").unwrap_or_default(), now));
    if job.run(host, pos, entity, now, QUANTUM) { with_pool(|pool| { pool.jobs.remove(&key); }); }
    else { with_pool(|pool| { *pool.jobs.entry(key).or_default() = Some(job); }); }
}

pub(super) fn maintenance(now: u64) {
    with_pool(|pool| {
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

#[cfg(test)]
thread_local! { static TEST_WORK: std::cell::Cell<u64> = const { std::cell::Cell::new(0) }; }
#[cfg(test)]
fn work(units: u32) { TEST_WORK.with(|work| work.set(work.get() + u64::from(units))); }

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone)]
    struct Factory { dimension: &'static str, nodes: BTreeMap<[i32; 3], ModBlockEntity>, reads: usize }
    impl Host for Factory {
        fn dimension(&self) -> &'static str { self.dimension }
        fn block(&self, pos: [i32; 3]) -> String { self.nodes.get(&pos).map_or("minecraft:air", |node| node.id.as_str()).into() }
        fn change_mod_block_entity(&mut self, pos: [i32; 3], change: &mut dyn FnMut(&mut ModBlockEntity)) -> bool {
            self.reads += 1;
            let Some(node) = self.nodes.get_mut(&pos) else { return false; };
            change(node); true
        }
    }
    fn node(id: &str) -> ModBlockEntity {
        let mut node = ModBlockEntity::default();
        node.id = id.into();
        node.items = vec![None; super::super::super::super::layout(id).map_or(0, |(size, _, _)| size)];
        node
    }
    fn fixture(dimension: &'static str, count: i32) -> (Factory, ModBlockEntity, Vec<[i32; 3]>) {
        let mut factory = Factory { dimension, nodes: BTreeMap::new(), reads: 0 };
        let network: Vec<_> = (1..=count).map(|index| [index, 64, 0]).collect();
        for (index, &pos) in network.iter().enumerate() {
            let mut station = node(if index % 3 == 0 { PRESS } else { ASSEMBLER });
            if index % 3 == 0 { station.items[1] = Some(ItemStack::new(PLATE, 32)); }
            factory.nodes.insert(pos, station);
        }
        let mut desk = node(FOREMAN_DESK);
        desk.items[0] = Some(ItemStack::new("minecraft:bread", DESK_FOOD));
        set_list(&mut desk, "Area", &network);
        set_number(&mut desk, "TaskId", 7);
        (factory, desk, network)
    }
    fn draft() -> Task {
        Task { id: 0, from: [1, 64, 0], slot: 1, item: PLATE.into(), count: 8,
            to: [2, 64, 0], into: 0, by: None }
    }

    #[test]
    fn reduced_snapshot_keeps_parallel_freight_filter_without_copying_unrelated_state() {
        let mut live = ModBlockEntity::default();
        live.id = "veio:drone_port".into();
        live.nbt.insert("FreightExport".into(), Tag::String(PLATE.into()));
        live.nbt.insert("UnrelatedState".into(), Tag::IntArray(vec![1; 1024]));
        let snapshot = Node::new([0; 3], &live);
        assert_eq!(snapshot.entity.nbt.len(), 1);
        assert!(matches!(snapshot.entity.nbt.get(nbt_key("FreightExport")), Some(Tag::String(item)) if java_eq(item, PLATE)));
    }

    #[test]
    fn stable_factories_match_the_previous_planner_with_hub_contracts_and_storage_priority() {
        for stage in [0, 1, 3] {
            let (mut factory, mut desk, network) = fixture("veio:planner-equivalence", 24);
            let mut hub = node(HUB);
            set_number(&mut hub, "ContractStage", stage);
            factory.nodes.insert(network[1], hub);
            let mut barn = node(storage::BARN);
            barn.items[5] = Some(ItemStack::new("minecraft:wheat", 20));
            barn.items[26] = Some(ItemStack::new(STEEL, 16));
            factory.nodes.insert(network[3], barn);
            factory.nodes.get_mut(&network[0]).unwrap().items[1] = Some(ItemStack::new(STEEL, 32));
            let mut shepherd = node(SHEPHERD_POST);
            shepherd.items[2] = Some(ItemStack::new("minecraft:wheat", 12));
            factory.nodes.insert(network[4], shepherd);
            set_number(&mut desk, "Organize", ORGANIZE_EVERY - 1);
            let mut reference = desk.clone();
            let mut reference_world = factory.clone();
            reference_organize(&mut reference_world, [0, 64, 0], &mut reference, 1);
            let mut job = Job::new([0, 64, 0], &desk, network, 1);
            let mut done = false;
            // Enough to fit the bounded publication tail, but not a whole plan.
            for _ in 0..10000 {
                let before = TEST_WORK.with(std::cell::Cell::get);
                done = job.run(&mut factory, [0, 64, 0], &mut desk, 1, COMMIT_UNITS);
                assert!(TEST_WORK.with(std::cell::Cell::get) - before <= u64::from(COMMIT_UNITS));
                if done { break; }
            }
            assert!(done);
            assert_eq!(queue(&desk), queue(&reference));
            assert_eq!(get_list(&desk, "Posts"), get_list(&reference, "Posts"));
            assert_eq!(number(&desk, "TaskId"), number(&reference, "TaskId"));
            assert_eq!(number(&desk, "Queued"), number(&reference, "Queued"));
            assert_eq!(factory.nodes[&[1,64,0]].items[1].as_ref().unwrap().count, 32);
        }
    }

    #[test]
    fn many_large_plans_share_the_quota_and_keep_the_old_queue_until_done() {
        let (mut world, desk, network) = fixture("veio:planner-global-limit", 95);
        // Every source is incompatible with every sink: exercise long pair searches.
        for station in world.nodes.values_mut() {
            *station = node(PRESS);
            station.items[1] = Some(ItemStack::new(PLATE, 32));
        }
        let mut desks: Vec<_> = (0..64).map(|i| {
            let mut table = desk.clone();
            let mut old = draft(); old.id = 6;
            set_queue(&mut table, &[old]);
            start(world.dimension, [i, 64, 100], &mut table, network.clone(), 0);
            table
        }).collect();
        let mut finished = false;
        for tick in 0..2000 {
            budget::begin_tick(tick);
            let before = TEST_WORK.with(std::cell::Cell::get);
            let reads = world.reads;
            for (i, table) in desks.iter_mut().enumerate() {
                if number(table, "PlanningPending") == 0 { continue; }
                let start = TEST_WORK.with(std::cell::Cell::get);
                advance(&mut world, [i as i32, 64, 100], table, tick);
                assert!(TEST_WORK.with(std::cell::Cell::get) - start <= u64::from(QUANTUM));
                if number(table, "PlanningPending") != 0 { assert_eq!(queue(table)[0].id, 6); }
                else { assert!(queue(table).is_empty()); }
            }
            assert!(TEST_WORK.with(std::cell::Cell::get) - before <= 2048);
            assert!(world.reads - reads <= 2048);
            if desks.iter().all(|table| number(table, "PlanningPending") == 0) { finished = true; break; }
        }
        assert!(finished);
        assert_eq!(world.reads, 64 * 95);
        assert!(desks.iter().all(|table| number(table, "TaskId") == 7));
    }

    #[test]
    fn publication_preserves_new_claims_and_does_not_resurrect_completed_pins() {
        let (mut world, mut desk, network) = fixture("veio:planner-live-claims", 4);
        let pos = [0, 64, 0];
        let mut job = Job::new(pos, &desk, network.clone(), 1);
        job.drafts.push(draft());
        job.phase = Phase::Commit;
        let mut claimed = draft(); claimed.id = 99; claimed.by = Some(([5,64,5], 2));
        set_queue(&mut desk, &[claimed.clone()]);
        set_number(&mut desk, "TaskId", 99);
        assert!(job.run(&mut world, pos, &mut desk, 2, QUANTUM));
        assert_eq!(queue(&desk), [claimed]);
        let mut job = Job::new(pos, &desk, network, 2);
        assert_eq!(job.pinned.len(), 1);
        set_queue(&mut desk, &[]); // The hauler took the material while we planned.
        job.phase = Phase::Commit;
        assert!(job.run(&mut world, pos, &mut desk, 3, QUANTUM));
        assert!(queue(&desk).is_empty());
        assert_eq!(world.nodes[&[1,64,0]].items[1].as_ref().unwrap().count, 32);
    }

    #[test]
    fn publication_rechecks_current_materials_counts_and_contracts() {
        let (mut world, mut desk, network) = fixture("veio:planner-live-inputs", 4);
        let pos = [0,64,0];
        let mut job = Job::new(pos, &desk, network.clone(), 1);
        job.drafts.push(draft()); job.phase = Phase::Commit;
        world.nodes.get_mut(&[1,64,0]).unwrap().items[1].as_mut().unwrap().count = 3;
        let reads = world.reads;
        assert!(!job.run(&mut world, pos, &mut desk, 2, COMMIT_UNITS - 1));
        assert_eq!(world.reads, reads);
        assert!(queue(&desk).is_empty());
        assert!(job.run(&mut world, pos, &mut desk, 2, QUANTUM));
        assert_eq!(queue(&desk)[0].count, 3);
        assert_eq!(world.nodes[&[1,64,0]].items[1].as_ref().unwrap().count, 3);
        set_queue(&mut desk, &[]);
        let mut job = Job::new(pos, &desk, network, 2);
        job.drafts.push(draft()); job.phase = Phase::Commit;
        let mut hub = node(HUB);
        set_number(&mut hub, "ContractStage", 1); // Now wants steel, not copper plates.
        world.nodes.insert([2,64,0], hub);
        assert!(job.run(&mut world, pos, &mut desk, 3, QUANTUM));
        assert!(queue(&desk).is_empty());
    }

    #[test]
    fn partial_job_resets_for_a_new_worker_and_restarts_after_memory_cleanup() {
        let (mut world, mut desk, network) = fixture("veio:planner-lifecycle", 95);
        set_worker(&mut desk, Some([1,2,3,4]));
        let pos = [0,64,0];
        start(world.dimension, pos, &mut desk, network, 0);
        advance(&mut world, pos, &mut desk, 0);
        set_worker(&mut desk, Some([9,8,7,6]));
        advance(&mut world, pos, &mut desk, 1);
        assert!(with_pool(|pool| pool.jobs.get(&(world.dimension, pos)).and_then(Option::as_ref)
            .is_some_and(|job| job.worker == Some([9,8,7,6]) && job.collect <= QUANTUM as usize)));
        maintenance(22);
        assert!(with_pool(|pool| pool.jobs.is_empty()));
        assert_eq!(number(&desk, "PlanningPending"), 1);
        for now in 23..1000 {
            advance(&mut world, pos, &mut desk, now);
            if number(&desk, "PlanningPending") == 0 { break; }
        }
        assert_eq!(number(&desk, "PlanningPending"), 0);
    }
}
