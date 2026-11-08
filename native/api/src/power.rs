//! Postes de energia: ligam-se sozinhos aos vizinhos por fios aéreos e alimentam as máquinas
//! da sua área, como os postes do Factorio.
//!
//! Custo: cada poste guarda em NBT a lista de vizinhos (`Links`) e de blocos de energia da área
//! (`Area`). A cada 40 ticks solicita a reconstrução dos fios e a varredura da área.
//! Ambas avançam em etapas sob cotas globais; listas completas continuam em uso.
//! A transferência percorre as listas completas. Os postes
//! se encontram por um registro em memória dividido em chunks.
use super::*;
use horizon_client::mod_api::{BlockEntitySubmitEvent, Tag};
use std::collections::{BTreeMap, HashMap};
mod rebuild;
use std::sync::Mutex;

pub(super) struct Tier {
    /// Alcance dos fios, em blocos.
    pub reach: i32,
    /// Raio da área alimentada em volta do poste (2 = 5x5).
    pub area: i32,
    /// FE por tick em cada fio e em cada máquina.
    pub rate: i32,
    pub capacity: i32,
    /// Altura do isolador, em blocos, onde o fio se prende.
    pub top: f32,
    pub wire: u32,
}

pub(super) const WOOD: Tier = Tier { reach: 8, area: 2, rate: 120, capacity: 1000, top: 1.40, wire: 0x2d2a26 };
pub(super) const STEEL: Tier = Tier { reach: 16, area: 3, rate: 400, capacity: 4000, top: 1.90, wire: 0xb5643c };
/// Cada poste escolhe seis vizinhos; a união simétrica pode conter mais fios.
const MAX_LINKS: usize = 6;
const REFRESH: i64 = 40;
/// Limit world lookups for area discovery; energy transfer still runs every tick.
const AREA_READS_PER_TICK: i32 = 8;

pub(super) fn tier(block: &str) -> Option<&'static Tier> {
    match block {
        POLE => Some(&WOOD),
        STEEL_POLE => Some(&STEEL),
        _ => None,
    }
}

#[derive(Default)]
struct PoleChunk { nodes: BTreeMap<[i32; 3], bool>, revision: u64 }
type Chunks = HashMap<(i32, i32), PoleChunk, FastHash>;
/// Postes conhecidos por dimensão e chunk. `true` marca o poste de aço.
static POLES: Mutex<Vec<(String, Chunks)>> = Mutex::new(Vec::new());

fn chunk([x, _, z]: [i32; 3]) -> (i32, i32) {
    (x >> 4, z >> 4)
}

fn register(dimension: &str, pos: [i32; 3], steel: bool) {
    let Ok(mut all) = POLES.lock() else { return };
    let index = match all.iter().position(|(name, _)| name == dimension) {
        Some(index) => index,
        None => {
            all.push((dimension.to_owned(), HashMap::default()));
            all.len() - 1
        }
    };
    let cell = all[index].1.entry(chunk(pos)).or_default();
    if cell.nodes.insert(pos, steel) != Some(steel) {
        cell.revision = cell.revision.wrapping_add(1);
    }
}

fn forget(dimension: &str, pos: [i32; 3]) {
    if let Ok(mut all) = POLES.lock()
        && let Some((_, chunks)) = all.iter_mut().find(|(name, _)| name == dimension)
        && let Some(cell) = chunks.get_mut(&chunk(pos))
        && cell.nodes.remove(&pos).is_some()
    {
        cell.revision = cell.revision.wrapping_add(1);
    }
}

/// Há um poste (de madeira ou aço) a até `radius` blocos de `pos`? Lê só os chunks em volta.
pub(super) fn pole_near(dimension: &str, pos: [i32; 3], radius: i32) -> bool {
    let Ok(all) = POLES.lock() else { return false };
    let Some((_, chunks)) = all.iter().find(|(name, _)| name == dimension) else { return false };
    let (cx, cz) = chunk(pos);
    let span = (radius >> 4) + 1;
    (cx - span..=cx + span).any(|x| (cz - span..=cz + span).any(|z| chunks.get(&(x, z))
        .is_some_and(|cell| cell.nodes.keys().any(|at| distance2(*at, pos) <= radius * radius))))
}

fn distance2(a: [i32; 3], b: [i32; 3]) -> i32 {
    (0..3).map(|axis| (a[axis] - b[axis]).pow(2)).sum()
}

fn reach(steel: bool) -> i32 {
    if steel { STEEL.reach } else { WOOD.reach }
}

/// Os até seis postes mais próximos de `pos` ao alcance do mais curto dos dois.
#[cfg(test)]
fn nearest(pos: [i32; 3], steel: bool, candidates: &[([i32; 3], bool)]) -> Vec<[i32; 3]> {
    let mut list: Vec<(i32, [i32; 3])> = candidates.iter()
        .filter(|(at, other)| distance2(pos, *at) <= reach(steel).min(reach(*other)).pow(2))
        .map(|(at, _)| (distance2(pos, *at), *at))
        .collect();
    list.sort_unstable();
    list.into_iter().take(MAX_LINKS).map(|(_, at)| at).collect()
}

#[cfg(test)]
mod optimization_tests {
    use super::*;

    #[test]
    fn thousand_poles_obey_one_global_area_quota_and_all_finish() {
        use std::cell::Cell;
        struct Probe { reads: Cell<usize> }
        impl Host for Probe {
            fn dimension(&self) -> &'static str { "veio:global-budget-test" }
            fn block(&self, _: [i32; 3]) -> String {
                self.reads.set(self.reads.get() + 1);
                "minecraft:air".into()
            }
        }
        let host = Probe { reads: Cell::new(0) };
        let mut poles: Vec<_> = (0..1000).map(|_| ModBlockEntity::default()).collect();
        let mut total = 0;
        for tick in 0..220 {
            budget::begin_tick(tick);
            host.reads.set(0);
            for (index, pole) in poles.iter_mut().enumerate() {
                scan_area(&host, [index as i32, 0, 0], pole, WOOD.area, tick == 0);
            }
            assert!(host.reads.get() <= 512, "global quota exceeded in tick {tick}");
            total += host.reads.get();
        }
        assert_eq!(total, 74000);
        assert!(poles.iter().all(|pole| number(pole, "AreaScan") == 0 && pole.nbt.contains_key(nbt_key("Area"))));
    }

    #[test]
    fn incremental_area_bounds_peak_reads_and_publishes_only_complete_results() {
        use std::cell::Cell;
        struct Probe { reads: Cell<usize> }
        impl Host for Probe {
            fn dimension(&self) -> &'static str { "veio:budget-fixture" }
            fn block(&self, _: [i32; 3]) -> String {
                self.reads.set(self.reads.get() + 1);
                PRESS.into()
            }
        }
        for radius in [WOOD.area, STEEL.area] {
            let host = Probe { reads: Cell::new(0) };
            let mut pole = ModBlockEntity::default();
            let old = vec![99, 0, 0];
            pole.nbt.insert(nbt_key("Area").clone(), Tag::IntArray(old));
            let total = (radius * 2 + 1).pow(2) * 3;
            let ticks = (total + AREA_READS_PER_TICK - 1) / AREA_READS_PER_TICK;
            for tick in 0..ticks {
                host.reads.set(0);
                scan_area(&host, [0; 3], &mut pole, radius, tick == 0);
                assert!(host.reads.get() <= AREA_READS_PER_TICK as usize);
                if tick + 1 < ticks { assert_eq!(offsets(&pole, "Area").collect::<Vec<_>>(), [[99, 0, 0]]); }
                // Persisted partial work resumes after saving/loading the entity.
                pole = pole.clone();
            }
            let expected: Vec<_> = (-radius..=radius).flat_map(|x| (-radius..=radius)
                .flat_map(move |z| (-1..=1).map(move |y| [x, y, z])))
                .filter(|at| *at != [0; 3]).collect();
            assert_eq!(offsets(&pole, "Area").collect::<Vec<_>>(), expected);
            assert_eq!(number(&pole, "AreaScan"), 0);
            host.reads.set(0);
            scan_area(&host, [0; 3], &mut pole, radius, false);
            assert_eq!(host.reads.get(), 0);
        }
    }

    #[test]
    fn area_refresh_keeps_delivering_power_and_removes_stale_consumers_when_complete() {
        struct Grid { consumer: ModBlockEntity }
        impl Host for Grid {
            fn dimension(&self) -> &'static str { "veio:incremental-power-test" }
            fn block(&self, _: [i32; 3]) -> String { "minecraft:air".into() }
            fn change_mod_block_entity(&mut self, pos: [i32; 3], change: &mut dyn FnMut(&mut ModBlockEntity)) -> bool {
                if pos != [1, 0, 0] { return false; }
                change(&mut self.consumer);
                true
            }
        }
        let mut pole = ModBlockEntity::default();
        pole.id = POLE.into();
        set_number(&mut pole, "energy", 200);
        pole.nbt.insert(nbt_key("Area").clone(), Tag::IntArray(vec![1, 0, 0]));
        let mut host = Grid { consumer: ModBlockEntity::default() };
        host.consumer.id = PRESS.into();
        budget::begin_tick(0);
        assert!(budget::request(budget::Lane::Topology, host.dimension(), [999; 3], 4096));
        assert!(budget::request(budget::Lane::Area, host.dimension(), [999; 3], 512));
        tick(&mut host, [0; 3], &mut pole, 0);
        assert_eq!(number(&host.consumer, "energy"), WOOD.rate);
        assert_eq!(number(&pole, "energy") + number(&host.consumer, "energy"), 200);
        assert!(number(&pole, "AreaScan") > 0);
        assert_eq!(number(&pole, "TopologyPending"), 1);
        // The mock world reports air: the old entry must disappear after the scan.
        for time in 1..=10 {
            budget::begin_tick(time as u64);
            tick(&mut host, [0; 3], &mut pole, time);
        }
        assert_eq!(offsets(&pole, "Area").len(), 0);
        assert_eq!(number(&pole, "TopologyPending"), 0);
        assert_eq!(number(&pole, "energy") + number(&host.consumer, "energy"), 200);
    }

    #[test]
    fn ranked_connections_match_original_sorted_selection_with_ties_and_mixed_tiers() {
        for seed in 0..8 {
            let nodes: Vec<_> = (-4i32..=4).flat_map(|x| (-4i32..=4).map(move |z| {
                ([x * (seed % 3 + 1), (x + z).rem_euclid(3), z * (seed % 2 + 1)], (x + z + seed) % 3 == 0)
            })).collect();
            for &(pos, steel) in nodes.iter().step_by(3) {
                let around = |center| nodes.iter().copied().filter(move |(at, _)| *at != center);
                let mine: Vec<_> = around(pos).collect();
                let own = nearest(pos, steel, &mine);
                let mut expected: Vec<_> = mine.iter().filter(|(at, tier)| {
                    own.contains(at) || nearest(*at, *tier, &around(*at).collect::<Vec<_>>()).contains(&pos)
                }).map(|(at, _)| *at).collect();
                expected.sort_unstable();
                assert_eq!(links(pos, steel, around), expected);
            }
        }
    }

    #[test]
    fn borrowed_area_keeps_wrap_order_and_handles_empty_or_partial_arrays() {
        struct Trace { order: Vec<[i32; 3]> }
        impl Host for Trace {
            fn dimension(&self) -> &'static str { "veio:power-regression" }
            fn change_mod_block_entity(&mut self, pos: [i32; 3], _: &mut dyn FnMut(&mut ModBlockEntity)) -> bool {
                self.order.push(pos); false
            }
        }
        let mut pole = ModBlockEntity::default();
        pole.id = POLE.into();
        pole.nbt.insert("Area".into(), Tag::IntArray(vec![1,0,0,2,0,0,3,0,0,99]));
        pole.nbt.insert("Links".into(), Tag::IntArray(Vec::new()));
        set_number(&mut pole, "Port", 2);
        let mut trace = Trace { order: Vec::new() };
        tick(&mut trace, [0; 3], &mut pole, 1);
        assert_eq!(trace.order, [[3,0,0],[1,0,0],[2,0,0]]);
        assert_eq!(number(&pole, "Port"), 0);
        pole.nbt.insert("Area".into(), Tag::IntArray(Vec::new()));
        trace.order.clear();
        tick(&mut trace, [0; 3], &mut pole, 2);
        assert!(trace.order.is_empty());
    }
}

// Membership in the nearest-six set needs a rank, not a sorted allocation.
#[cfg(test)]
fn among_nearest(center: [i32; 3], steel: bool, target: [i32; 3], target_steel: bool,
    peers: impl IntoIterator<Item = ([i32; 3], bool)>) -> bool
{
    let target_distance = distance2(center, target);
    if target_distance > reach(steel).min(reach(target_steel)).pow(2) { return false; }
    let mut before = 0;
    for (at, other_steel) in peers {
        let distance = distance2(center, at);
        if distance <= reach(steel).min(reach(other_steel)).pow(2) && (distance, at) < (target_distance, target) {
            before += 1;
            if before == MAX_LINKS { return false; }
        }
    }
    true
}

/// Symmetric union of the nearest-six selections, with the original ordering.
#[cfg(test)]
pub(super) fn links<I>(pos: [i32; 3], steel: bool, around: impl Fn([i32; 3]) -> I) -> Vec<[i32; 3]>
where I: IntoIterator<Item = ([i32; 3], bool)> {
    let mine: Vec<_> = around(pos).into_iter().collect();
    let own = nearest(pos, steel, &mine);
    let mut result: Vec<[i32; 3]> = mine.iter()
        .filter(|(other, other_steel)| {
            own.contains(other) || among_nearest(*other, *other_steel, pos, steel, around(*other))
        })
        .map(|(other, _)| *other)
        .collect();
    result.sort_unstable();
    result
}

fn offsets<'a>(entity: &'a ModBlockEntity, key: &str) -> impl ExactSizeIterator<Item = [i32; 3]> + Clone + 'a {
    let values: &[i32] = match entity.nbt.get(nbt_key(key)) {
        Some(Tag::IntArray(values)) => values,
        _ => &[],
    };
    values.chunks_exact(3).map(|v| [v[0], v[1], v[2]])
}


fn add([x, y, z]: [i32; 3], [dx, dy, dz]: [i32; 3]) -> [i32; 3] {
    [x + dx, y + dy, z + dz]
}

/// Quanto passar de A para B para igualar a porcentagem de carga (negativo: de B para A).
pub(super) fn balance(energy: i32, capacity: i32, other: i32, other_capacity: i32, rate: i32) -> i32 {
    ((i64::from(energy) * i64::from(other_capacity) - i64::from(other) * i64::from(capacity))
        / i64::from((capacity + other_capacity).max(1))).clamp(-i64::from(rate), i64::from(rate)) as i32
}

/// O poste aceita receber de outro bloco de energia ou entregar a ele.
fn powered(block: &str) -> bool {
    needs_power(block) || is_power_block(block) || tier(block).is_some()
}

fn scan_area(host: &dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity, radius: i32, due: bool) {
    let side = radius * 2 + 1;
    let total = side * side * 3;
    let mut cursor = number(entity, "AreaScan") - 1;
    if cursor < 0 || cursor >= total {
        if !due && entity.nbt.contains_key(nbt_key("Area")) { return; }
        cursor = 0;
        match entity.nbt.get_mut(nbt_key("AreaFound")) {
            Some(Tag::IntArray(pending)) => pending.clear(),
            _ => { entity.nbt.insert(nbt_key("AreaFound").clone(), Tag::IntArray(Vec::new())); }
        }
    }
    if !budget::request(budget::Lane::Area, host.dimension(), pos, AREA_READS_PER_TICK as u32) {
        set_number(entity, "AreaScan", cursor + 1);
        return;
    }
    // Keep the previous complete area active until all cells have been checked.
    // The scratch buffer is reused on subsequent refreshes, including after load.
    let Some(Tag::IntArray(pending)) = entity.nbt.get_mut(nbt_key("AreaFound")) else {
        set_number(entity, "AreaScan", 0);
        return;
    };
    let end = (cursor + AREA_READS_PER_TICK).min(total);
    for cell in cursor..end {
        let offset = [cell / (side * 3) - radius, cell % 3 - 1, (cell / 3) % side - radius];
        if offset != [0; 3] && powered(&host.block(add(pos, offset))) {
            pending.extend_from_slice(&offset);
        }
    }
    if end < total {
        set_number(entity, "AreaScan", end + 1);
        return;
    }
    let complete = std::mem::take(pending);
    let previous = if let Some(area) = entity.nbt.get_mut(nbt_key("Area")) {
        std::mem::replace(area, Tag::IntArray(complete))
    } else {
        entity.nbt.insert(nbt_key("Area").clone(), Tag::IntArray(complete));
        Tag::IntArray(Vec::new())
    };
    let mut scratch = match previous { Tag::IntArray(values) => values, _ => Vec::new() };
    scratch.clear();
    *entity.nbt.get_mut(nbt_key("AreaFound")).expect("area scratch exists") = Tag::IntArray(scratch);
    set_number(entity, "AreaScan", 0);
}

pub(super) fn maintenance(now: u64) { rebuild::maintenance(now); }

pub(super) fn tick(host: &mut dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity, time: i64) -> bool {
    let Some(own) = tier(&entity.id) else { return false };
    let steel = entity.id == STEEL_POLE;
    let dimension = host.dimension();
    let mut changed = false;
    let due = (time + i64::from((pos[0] ^ pos[2]) & 31)).rem_euclid(REFRESH) == 0;
    if due || (!entity.nbt.contains_key(nbt_key("Links")) && number(entity, "TopologyPending") == 0) {
        register(dimension, pos, steel);
        set_number(entity, "TopologyPending", 1);
    }
    if number(entity, "TopologyPending") != 0 {
        changed |= rebuild::poll(host, pos, entity, steel, time);
    }
    scan_area(host, pos, entity, own.area, due);
    let limit = own.capacity;
    let mut energy = number(entity, "energy").min(limit);
    let area = offsets(entity, "Area");
    let area_len = area.len();
    let start = number(entity, "Port") as usize % area_len.max(1);
    for offset in area.clone().skip(start).chain(area.take(start)) {
        let target = add(pos, offset);
        host.change_mod_block_entity(target, &mut |other| {
            if tier(&other.id).is_some() {
                return; // Postes se ligam por fio, não pela área.
            }
            let capacity = capacity(&other.id);
            let stored = number(other, "energy").min(capacity);
            let flow = if needs_power(&other.id) {
                energy.min(own.rate).min(capacity - stored).max(0)
            } else if is_generator(&other.id) {
                balance(energy, limit, stored, capacity, own.rate).min(0)
            } else if matches!(other.id.as_str(), CABLE | BATTERY) {
                balance(energy, limit, stored, capacity, own.rate)
            } else {
                0
            };
            let flow = flow.clamp(-stored.min(limit - energy), energy.min(capacity - stored));
            if flow != 0 {
                set_number(other, "energy", stored + flow);
                energy -= flow;
            }
        });
    }
    if area_len > 0 {
        set_number(entity, "Port", ((start + 1) % area_len) as i32);
    }
    // Cada fio é atendido só pela ponta "menor", uma vez por tick.
    for offset in offsets(entity, "Links").into_iter().filter(|offset| *offset > [0, 0, 0]) {
        host.change_mod_block_entity(add(pos, offset), &mut |other| {
            let Some(far) = tier(&other.id) else { return };
            let stored = number(other, "energy").min(far.capacity);
            let flow = balance(energy, limit, stored, far.capacity, own.rate.min(far.rate))
                .clamp(-stored.min(limit - energy), energy.min(far.capacity - stored));
            if flow != 0 {
                set_number(other, "energy", stored + flow);
                energy -= flow;
            }
        });
    }
    set_number(entity, "energy", energy);
    set_number(entity, "Status", if energy > 0 { 1 } else { 5 });
    changed
}

/// Fios com curvatura entre o isolador deste poste e o dos vizinhos "maiores".
pub(super) fn draw(host: &dyn Host, event: &mut BlockEntitySubmitEvent<'_>) {
    let Some(own) = tier(event.block) else { return };
    for offset in offsets(event.entity, "Links").into_iter().filter(|offset| *offset > [0, 0, 0]) {
        let far = if host.block(add(event.pos, offset)) == STEEL_POLE { &STEEL } else { &WOOD };
        let start = [0.5, own.top, 0.5];
        let end = [offset[0] as f32 + 0.5, offset[1] as f32 + far.top, offset[2] as f32 + 0.5];
        wire(host, event, start, end, own.wire);
    }
    // Nada aqui depende do tempo: o jogo guarda o desenho até os fios mudarem.
    event.draw.keep();
}

pub(super) fn sag_points(start: [f32; 3], end: [f32; 3]) -> Vec<[f32; 3]> {
    let length = (0..3).map(|axis| (end[axis] - start[axis]).powi(2)).sum::<f32>().sqrt();
    let segments = ((length * 2.0) as usize).clamp(6, 32);
    let sag = (length * 0.05).min(1.0);
    (0..=segments).map(|step| {
        let t = step as f32 / segments as f32;
        let mut point: [f32; 3] = std::array::from_fn(|axis| start[axis] + (end[axis] - start[axis]) * t);
        point[1] -= sag * 4.0 * t * (1.0 - t);
        point
    }).collect()
}

fn wire(host: &dyn Host, event: &mut BlockEntitySubmitEvent<'_>, start: [f32; 3], end: [f32; 3], tint: u32) {
    const HALF: f32 = 0.025;
    let points = sag_points(start, end);
    let [dx, _, dz] = std::array::from_fn::<f32, 3, _>(|axis| end[axis] - start[axis]);
    let flat = (dx * dx + dz * dz).sqrt().max(0.001);
    // Duas fitas cruzadas formam um fio visível de todos os lados.
    let across = [-dz / flat * HALF, 0.0, dx / flat * HALF];
    let up = [0.0, HALF, 0.0];
    for pair in points.windows(2) {
        let (a, b) = (pair[0], pair[1]);
        for side in [across, up] {
            let corners = [plus(a, side, 1.0), plus(b, side, 1.0), plus(b, side, -1.0), plus(a, side, -1.0)];
            let back = [corners[3], corners[2], corners[1], corners[0]];
            for quad in [corners, back] {
                host.draw_quad(event.draw, "veio:gui/material", quad, [0.0, 0.0, 1.0, 1.0], tint);
            }
        }
    }
}

fn plus(point: [f32; 3], by: [f32; 3], sign: f32) -> [f32; 3] {
    std::array::from_fn(|axis| point[axis] + by[axis] * sign)
}

/// Área desenhada, para o jogo não esconder fios longos quando o poste sai da tela.
pub(super) fn bounds(entity: &ModBlockEntity, bounds: &mut [f32; 6]) {
    for [x, y, z] in offsets(entity, "Links") {
        let point = [x as f32 + 0.5, y as f32 + 2.0, z as f32 + 0.5];
        for axis in 0..3 {
            bounds[axis] = bounds[axis].min(point[axis] - 1.0);
            bounds[axis + 3] = bounds[axis + 3].max(point[axis] + 1.0);
        }
    }
}
