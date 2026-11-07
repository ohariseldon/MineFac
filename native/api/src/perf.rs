//! Reproducible offline allocation/work benchmarks. Never compiled into the game.
use super::*;
use std::{alloc::{GlobalAlloc, Layout, System}, cell::Cell, hint::black_box, time::Instant};

#[derive(Clone, Copy, Default)]
struct Counts { allocations: usize, bytes: usize, keys: usize, fallback: usize }
thread_local! {
    static ACTIVE: Cell<bool> = const { Cell::new(false) };
    static COUNTS: Cell<Counts> = const { Cell::new(Counts { allocations: 0, bytes: 0, keys: 0, fallback: 0 }) };
}
struct Meter;
#[global_allocator]
static ALLOCATOR: Meter = Meter;

fn count(update: impl FnOnce(&mut Counts)) {
    if ACTIVE.try_with(Cell::get).unwrap_or(false) {
        let _ = COUNTS.try_with(|cell| { let mut counts = cell.get(); update(&mut counts); cell.set(counts); });
    }
}
unsafe impl GlobalAlloc for Meter {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        count(|c| { c.allocations += 1; c.bytes += layout.size(); });
        unsafe { System.alloc(layout) }
    }
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        count(|c| { c.allocations += 1; c.bytes += layout.size(); });
        unsafe { System.alloc_zeroed(layout) }
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) { unsafe { System.dealloc(ptr, layout) } }
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, size: usize) -> *mut u8 {
        count(|c| { c.allocations += 1; c.bytes += size; });
        unsafe { System.realloc(ptr, layout, size) }
    }
}
pub(super) fn key_read() { count(|c| c.keys += 1); }
pub(super) fn fallback_lock() { count(|c| c.fallback += 1); }

fn measure(name: &str, runs: usize, mut run: impl FnMut()) {
    for _ in 0..4 { run(); }
    COUNTS.with(|c| c.set(Counts::default()));
    ACTIVE.with(|c| c.set(true));
    let start = Instant::now();
    for _ in 0..runs { run(); }
    let elapsed = start.elapsed();
    ACTIVE.with(|c| c.set(false));
    let c = COUNTS.with(Cell::get);
    println!("PERF {{\"scenario\":\"{name}\",\"runs\":{runs},\"ns\":{},\"allocations\":{},\"allocated_bytes\":{},\"nbt_keys\":{},\"fallback_locks\":{}}}",
        elapsed.as_nanos(), c.allocations, c.bytes, c.keys, c.fallback);
}

#[test]
fn stable_publication_and_network_ticks_allocate_nothing() {
    let mut arms = node(inserters::ID);
    logistics_controls::set_filter(&mut arms, Some(PLATE));
    let mut cable = node(CABLE);
    let mut console = node(workforce::CONTROL_CENTER);
    let mut zone = node(workforce::zones::PEN_MARKER);
    let mut machine = node(PRESS);
    let mut pole = node(POLE);
    pole.nbt.insert("Area".into(), Tag::IntArray(vec![1,0,0,2,0,0]));
    pole.nbt.insert("Links".into(), Tag::IntArray(vec![3,0,0]));
    let mut grid = Grid { targets: vec![node(PRESS), node(BATTERY), node(POLE)], callbacks: 0 };
    let mut step = || {
        logistics_controls::publish(&mut arms);
        presentation::publish(&mut cable);
        presentation::publish(&mut console);
        presentation::publish(&mut zone);
        record_performance(&mut machine, 0);
        power::tick(&mut grid, [0; 3], &mut pole, 1);
    };
    for _ in 0..16 { step(); }
    COUNTS.with(|c| c.set(Counts::default()));
    ACTIVE.with(|c| c.set(true));
    for _ in 0..64 { step(); }
    ACTIVE.with(|c| c.set(false));
    let counts = COUNTS.with(Cell::get);
    assert_eq!(counts.allocations, 0);
    assert_eq!(counts.fallback, 0);
}

fn node(id: &str) -> ModBlockEntity {
    let mut n = ModBlockEntity::default();
    n.id = id.into();
    n.items = vec![None; layout(id).map_or(0, |(n, _, _)| n)];
    set_number(&mut n, "Status", 1);
    set_number(&mut n, "energy", 500);
    n
}

struct Grid { targets: Vec<ModBlockEntity>, callbacks: usize }
impl Host for Grid {
    fn dimension(&self) -> &'static str { "veio:performance" }
    fn change_mod_block_entity(&mut self, pos: [i32; 3], change: &mut dyn FnMut(&mut ModBlockEntity)) -> bool {
        self.callbacks += 1;
        let Some(n) = usize::try_from(pos[0] - 1).ok().and_then(|i| self.targets.get_mut(i)) else { return false; };
        change(n); true
    }
}

struct Factory { nodes: std::collections::HashMap<[i32; 3], ModBlockEntity>, queries: usize }
impl Host for Factory {
    fn dimension(&self) -> &'static str { "veio:performance" }
    fn block(&self, pos: [i32; 3]) -> String {
        self.nodes.get(&pos).map_or_else(|| "minecraft:air".into(), |n| n.id.clone())
    }
    fn change_mod_block_entity(&mut self, pos: [i32; 3], change: &mut dyn FnMut(&mut ModBlockEntity)) -> bool {
        self.queries += 1;
        if let Some(n) = self.nodes.get_mut(&pos) { change(n); true } else { false }
    }
}

#[test]
#[ignore = "explicit performance run; run alone with --release --test-threads=1 --nocapture"]
fn profile_scale() {
    let mut wires: Vec<_> = (0..2000).map(|_| node(CABLE)).collect();
    measure("publish_2000_cables_20_ticks", 20, || {
        for n in &mut wires { presentation::publish(black_box(n)); }
    });

    let mut arm = node(inserters::ID);
    logistics_controls::set_filter(&mut arm, Some(PLATE));
    measure("configured_arm_publication", 20000, || { logistics_controls::publish(black_box(&mut arm)); });

    let mut machine = node(PRESS);
    measure("working_machine_metrics", 20000, || { record_performance(black_box(&mut machine), 0); });

    let stone = ItemStack::new("minecraft:stone", 1);
    measure("rejected_queue_material", 30000, || { black_box(operations::accepts_material(&machine, 0, &stone)); });

    let mut poles: Vec<_> = (0..256).map(|_| {
        let mut n = node(POLE);
        n.nbt.insert("Area".into(), Tag::IntArray(vec![1,0,0, 2,0,0]));
        n.nbt.insert("Links".into(), Tag::IntArray(vec![3,0,0, -3,0,0]));
        n
    }).collect();
    let mut grid = Grid { targets: vec![node(PRESS), node(BATTERY), node(POLE)], callbacks: 0 };
    measure("256_poles_cached_network_40_ticks", 40, || {
        for n in &mut poles { black_box(power::tick(&mut grid, [0; 3], n, 1)); }
    });
    let energy: i32 = poles.iter().chain(grid.targets.iter()).map(|n| number(n, "energy")).sum();
    println!("CHECK power_energy={energy} callbacks={}", grid.callbacks);

    let candidates: Vec<_> = (-4..=4).flat_map(|x| (-4..=4).map(move |z| ([x,0,z], false)))
        .filter(|(p, _)| *p != [0; 3]).collect();
    measure("dense_pole_links", 100, || {
        black_box(power::links([0; 3], false, |pos| candidates.iter().copied().filter(move |(p, _)| *p != pos)));
    });

    let mut factory = Factory { nodes: Default::default(), queries: 0 };
    let mut positions = Vec::new();
    for i in 0..96 {
        let pos = [i % 12 - 6, 0, i / 12 - 4];
        if pos == [0; 3] { continue; }
        let mut n = node(if i % 3 == 0 { PRESS } else { ASSEMBLER });
        if i % 3 == 0 { n.items[1] = Some(ItemStack::new(PLATE, 32)); }
        factory.nodes.insert(pos, n);
        positions.push(pos);
    }
    let mut desk = node(workforce::FOREMAN_DESK);
    desk.nbt.insert("Area".into(), Tag::IntArray(positions.iter().flatten().copied().collect()));
    measure("foreman_95_node_queue", 40, || {
        workforce::profile_organize(&mut factory, [0; 3], &mut desk, 1);
    });
    println!("CHECK queue_tasks={} source_reads={}", number(&desk, "Queued"), factory.queries);
}
