//! Postes de energia: ligam-se sozinhos aos vizinhos por fios aéreos e alimentam as máquinas
//! da sua área, como os postes do Factorio.
//!
//! Custo: cada poste guarda em NBT a lista de vizinhos (`Links`) e de blocos de energia da área
//! (`Area`), recalculadas a cada 40 ticks. No tick comum só percorre essas listas. Os postes
//! se encontram por um registro em memória dividido em chunks.
use super::*;
use horizon_client::mod_api::{BlockEntitySubmitEvent, Tag};
use std::collections::HashMap;
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
/// No máximo seis fios por poste, para a malha não virar uma teia.
const MAX_LINKS: usize = 6;
const REFRESH: i64 = 40;

pub(super) fn tier(block: &str) -> Option<&'static Tier> {
    match block {
        POLE => Some(&WOOD),
        STEEL_POLE => Some(&STEEL),
        _ => None,
    }
}

type Chunks = HashMap<(i32, i32), Vec<([i32; 3], bool)>>;
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
            all.push((dimension.to_owned(), HashMap::new()));
            all.len() - 1
        }
    };
    let list = all[index].1.entry(chunk(pos)).or_default();
    match list.iter_mut().find(|(at, _)| *at == pos) {
        Some(entry) => entry.1 = steel,
        None => list.push((pos, steel)),
    }
}

fn forget(dimension: &str, pos: [i32; 3]) {
    if let Ok(mut all) = POLES.lock()
        && let Some((_, chunks)) = all.iter_mut().find(|(name, _)| name == dimension)
        && let Some(list) = chunks.get_mut(&chunk(pos))
    {
        list.retain(|(at, _)| *at != pos);
    }
}

/// Postes registrados a até `radius` blocos (pela caixa de chunks).
fn nearby(dimension: &str, pos: [i32; 3], radius: i32) -> Vec<([i32; 3], bool)> {
    let Ok(all) = POLES.lock() else { return Vec::new() };
    let Some((_, chunks)) = all.iter().find(|(name, _)| name == dimension) else { return Vec::new() };
    let (cx, cz) = chunk(pos);
    let span = (radius >> 4) + 1;
    let mut found = Vec::new();
    for x in cx - span..=cx + span {
        for z in cz - span..=cz + span {
            if let Some(list) = chunks.get(&(x, z)) {
                found.extend(list.iter().filter(|(at, _)| *at != pos).copied());
            }
        }
    }
    found
}

fn distance2(a: [i32; 3], b: [i32; 3]) -> i32 {
    (0..3).map(|axis| (a[axis] - b[axis]).pow(2)).sum()
}

fn reach(steel: bool) -> i32 {
    if steel { STEEL.reach } else { WOOD.reach }
}

/// Os até seis postes mais próximos de `pos` ao alcance do mais curto dos dois.
fn nearest(pos: [i32; 3], steel: bool, candidates: &[([i32; 3], bool)]) -> Vec<[i32; 3]> {
    let mut list: Vec<(i32, [i32; 3])> = candidates.iter()
        .filter(|(at, other)| distance2(pos, *at) <= reach(steel).min(reach(*other)).pow(2))
        .map(|(at, _)| (distance2(pos, *at), *at))
        .collect();
    list.sort_unstable();
    list.into_iter().take(MAX_LINKS).map(|(_, at)| at).collect()
}

/// Ligação simétrica: A e B se ligam se um estiver entre os seis mais próximos do outro.
/// Os dois lados calculam o mesmo resultado a partir do mesmo registro.
pub(super) fn links(pos: [i32; 3], steel: bool, around: impl Fn([i32; 3]) -> Vec<([i32; 3], bool)>) -> Vec<[i32; 3]> {
    let mine = around(pos);
    let own = nearest(pos, steel, &mine);
    let mut result: Vec<[i32; 3]> = mine.iter()
        .filter(|(other, other_steel)| {
            own.contains(other) || nearest(*other, *other_steel, &around(*other)).contains(&pos)
        })
        .map(|(other, _)| *other)
        .collect();
    result.sort_unstable();
    result
}

fn offsets(entity: &ModBlockEntity, key: &str) -> Vec<[i32; 3]> {
    match entity.nbt.get(&key.into()) {
        Some(Tag::IntArray(values)) => values.chunks_exact(3).map(|v| [v[0], v[1], v[2]]).collect(),
        _ => Vec::new(),
    }
}

fn set_offsets(entity: &mut ModBlockEntity, key: &str, list: &[[i32; 3]]) -> bool {
    let flat: Vec<i32> = list.iter().flatten().copied().collect();
    if matches!(entity.nbt.get(&key.into()), Some(Tag::IntArray(old)) if *old == flat) {
        return false;
    }
    entity.nbt.insert(key.into(), Tag::IntArray(flat));
    true
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

pub(super) fn tick(host: &mut dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity, time: i64) -> bool {
    let Some(own) = tier(&entity.id) else { return false };
    let steel = entity.id == STEEL_POLE;
    let dimension = host.dimension();
    let mut changed = false;
    if (time + i64::from((pos[0] ^ pos[2]) & 31)).rem_euclid(REFRESH) == 0 || !entity.nbt.contains_key(&"Links".into()) {
        register(dimension, pos, steel);
        let reader: &dyn Host = host;
        // Confere os candidatos no mundo: um poste quebrado sai do registro.
        let mut gone = Vec::new();
        let candidates: Vec<([i32; 3], bool)> = nearby(dimension, pos, 2 * STEEL.reach).into_iter()
            .filter(|(at, steel)| {
                let alive = reader.block(*at) == if *steel { STEEL_POLE } else { POLE };
                if !alive { gone.push(*at); }
                alive
            })
            .collect();
        for at in gone {
            forget(dimension, at);
        }
        let around = |center: [i32; 3]| candidates.iter().chain([&(pos, steel)])
            .filter(|(at, _)| *at != center && distance2(*at, center) <= (2 * STEEL.reach).pow(2))
            .copied().collect::<Vec<_>>();
        let linked: Vec<[i32; 3]> = links(pos, steel, around).into_iter()
            .map(|at| [at[0] - pos[0], at[1] - pos[1], at[2] - pos[2]]).collect();
        changed |= set_offsets(entity, "Links", &linked);
        let mut area = Vec::new();
        for dx in -own.area..=own.area {
            for dz in -own.area..=own.area {
                for dy in -1..=1 {
                    if (dx, dy, dz) != (0, 0, 0) && powered(&reader.block(add(pos, [dx, dy, dz]))) {
                        area.push([dx, dy, dz]);
                    }
                }
            }
        }
        set_offsets(entity, "Area", &area);
    }
    let limit = own.capacity;
    let mut energy = number(entity, "energy").min(limit);
    let area = offsets(entity, "Area");
    let start = number(entity, "Port") as usize;
    for i in 0..area.len() {
        let target = add(pos, area[(start + i) % area.len()]);
        host.change_mod_block_entity(target, &mut |other| {
            if tier(&other.id).is_some() {
                return; // Postes se ligam por fio, não pela área.
            }
            let capacity = capacity(&other.id);
            let stored = number(other, "energy").min(capacity);
            let flow = if needs_power(&other.id) {
                energy.min(own.rate).min(capacity - stored).max(0)
            } else if matches!(other.id.as_str(), GENERATOR | SOLAR | GEOTHERMAL) {
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
    if !area.is_empty() {
        set_number(entity, "Port", ((start + 1) % area.len()) as i32);
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
