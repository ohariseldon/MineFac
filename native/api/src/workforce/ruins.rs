//! Ruínas: prédios da vila caídos em volta do centro, achados na chegada.
//!
//! Quando o primeiro jogador chega a um mundo novo e o centro da vila é achado (`spawn::tick`),
//! o Veio escolhe lugares num anel em volta do centro (`RING`), em terreno firme e plano:
//! - a altura `MotionBlockingNoLeaves` igual à `OceanFloor` em toda a planta (sem água nem lavoura
//!   irrigada) e desnível de no máximo `SLOPE` blocos (casas e árvores ficam de fora);
//! - sem sobrepor outra ruína, com `site::MARGIN` blocos de folga.
//!
//! Em cada lugar vai um canteiro (`veio:site`, virado para o centro) só onde o bloco é
//! substituível. No primeiro tick o canteiro pega a planta escolhida (`claim`), fica com a planta
//! fixa (`Fixed`) e carimba a ruína (`stamp`): as camadas de baixo da planta, um subconjunto
//! determinístico, sem móveis (blocos com propriedades), sem blocos protegidos e sem tocar no que
//! o jogador ou a vila já têm ali. O jogador paga a obra no centro e o mestre de obras reconstrói.
//!
//! Tudo acontece uma vez por mundo: `<mundo>/veio/ruins.txt` guarda que as ruínas já foram postas
//! e quais canteiros ainda não pegaram a planta (chunk descarregado, mundo fechado no meio).
//! Mundos antigos (o primeiro dia já passou) não ganham ruínas.
use super::*;
use horizon_client::mod_api::{NativeBlockEdit, NativeBlockRun, NativeHeightmap};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};

/// Distâncias (em blocos, no plano) do centro da vila aos canteiros. O anel do projeto é de 48 a
/// 80 blocos; os canteiros ficam até 62 para o centro de controle no meio da vila (a até
/// `center::RADIUS`) alcançar a obra.
pub(crate) const RING: [i32; 3] = [50, 56, 62];
/// Direções tentadas em volta do centro.
pub(crate) const ANGLES: i32 = 16;
/// Desnível máximo do terreno sob a planta.
pub(crate) const SLOPE: i32 = 3;
/// Ruínas por vila.
pub(crate) const COUNT: usize = 3;
/// Só mundos novos: até o fim do primeiro dia.
pub(crate) const FRESH: i64 = 24000;
/// Chance (em 100) de cada camada da planta sobrar na ruína, de baixo para cima.
const KEEP: [u64; 4] = [90, 70, 45, 20];
/// Blocos que se apoiam em outros: não sobram na ruína.
const ATTACHED: [&str; 12] = ["lantern", "torch", "carpet", "potted_", "_bed", "_door", "bell", "candle", "campfire",
    "flower_pot", "lectern", "azalea_leaves"];
const FILE: &str = "ruins.txt";
const HEADER: &str = "veio-ruins 1";

/// Uma ruína escolhida: o canteiro, a planta (`blueprints::BLUEPRINTS`) e o giro.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Ruin {
    pub dimension: String,
    pub pos: [i32; 3],
    pub plan: usize,
    pub rot: i32,
}

#[derive(Default)]
struct State {
    world: Option<PathBuf>,
    synced: bool,
    pending: Vec<Ruin>,
}

static STATE: Mutex<State> = Mutex::new(State { world: None, synced: false, pending: Vec::new() });
/// Há canteiros à espera da planta (ou o arquivo do mundo ainda não foi lido).
static ACTIVE: AtomicBool = AtomicBool::new(true);

/// Mundo novo: esquece o que veio do anterior.
pub(crate) fn reset() {
    if let Ok(mut state) = STATE.lock() {
        *state = State::default();
    }
    ACTIVE.store(true, Ordering::Release);
}

/// Plantas das ruínas: obras sem exigência de prestígio que têm prédio, começando num ponto
/// que depende do lugar da vila.
pub(crate) fn plans(seed: u64) -> Vec<usize> {
    let all: Vec<usize> = (0..blueprints::BLUEPRINTS.len())
        .filter(|index| projects::PROJECTS.iter()
            .any(|project| project.key == blueprints::BLUEPRINTS[*index].key && project.tier == 0 && !project.repeat))
        .collect();
    if all.is_empty() {
        return all;
    }
    let start = (seed % all.len() as u64) as usize;
    (0..all.len().min(COUNT)).map(|step| all[(start + step) % all.len()]).collect()
}

fn mix(mut value: u64) -> u64 {
    value = value.wrapping_add(0x9e37_79b9_7f4a_7c15);
    value = (value ^ (value >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    value = (value ^ (value >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    value ^ (value >> 31)
}

pub(crate) fn seed(center: [i32; 3]) -> u64 {
    mix(((center[0] as u32 as u64) << 32) | center[2] as u32 as u64)
}

/// Giro para a entrada da planta ficar virada para o centro.
pub(crate) fn facing_center(pos: [i32; 3], center: [i32; 3]) -> i32 {
    let (dx, dz) = (center[0] - pos[0], center[2] - pos[2]);
    if dx.abs() > dz.abs() {
        if dx > 0 { 3 } else { 1 }
    } else if dz > 0 {
        0
    } else {
        2
    }
}

const FACINGS: [&str; 4] = ["south", "west", "north", "east"];

fn overlaps(a: ([i32; 3], [i32; 3]), b: ([i32; 3], [i32; 3])) -> bool {
    a.0[0] <= b.1[0] && b.0[0] <= a.1[0] && a.0[2] <= b.1[2] && b.0[2] <= a.1[2]
}

/// Colunas lidas sob a planta: uma grade de 2 em 2 com as bordas; a do canteiro primeiro.
fn columns(low: [i32; 3], high: [i32; 3], site: [i32; 3]) -> Vec<(i32, i32)> {
    let axis = |from: i32, to: i32| {
        let mut values: Vec<i32> = (from..=to).step_by(2).collect();
        if values.last() != Some(&to) {
            values.push(to);
        }
        values
    };
    let mut out = vec![(site[0], site[2])];
    for x in axis(low[0], high[0]) {
        for z in axis(low[2], high[2]) {
            if (x, z) != (site[0], site[2]) {
                out.push((x, z));
            }
        }
    }
    out
}

/// Terreno firme e plano sob a planta? Devolve a altura do canteiro (primeiro bloco livre).
fn ground(host: &dyn Host, dimension: &str, columns: &[(i32, i32)]) -> Option<i32> {
    let top = host.surface_heights(dimension, columns, NativeHeightmap::MotionBlockingNoLeaves).ok()?;
    let floor = host.surface_heights(dimension, columns, NativeHeightmap::OceanFloor).ok()?;
    if top.len() != columns.len() || floor.len() != columns.len() {
        return None;
    }
    let mut heights = Vec::with_capacity(columns.len());
    for (a, b) in top.iter().zip(&floor) {
        let (Some(a), Some(b)) = (a, b) else { return None };
        if a != b {
            return None;
        }
        heights.push(*a);
    }
    let (low, high) = (heights.iter().min()?, heights.iter().max()?);
    (high - low <= SLOPE).then(|| heights[0])
}

/// Escolhe os lugares das ruínas em volta de `center`. Determinístico para o mesmo terreno.
pub(crate) fn choose(host: &dyn Host, dimension: &str, center: [i32; 3]) -> Vec<Ruin> {
    let seed = seed(center);
    let wanted = plans(seed);
    let base = (seed >> 8) as i32 & (ANGLES - 1);
    let mut taken: Vec<([i32; 3], [i32; 3])> = Vec::new();
    let mut out = Vec::new();
    for (order, &plan_index) in wanted.iter().enumerate() {
        let plan = &blueprints::BLUEPRINTS[plan_index];
        // Cada ruína começa num lado diferente do anel.
        let first = base + order as i32 * ANGLES / wanted.len().max(1) as i32;
        'search: for step in 0..ANGLES {
            let angle = f64::from((first + step).rem_euclid(ANGLES)) * std::f64::consts::TAU / f64::from(ANGLES);
            for radius in RING {
                let x = center[0] + (f64::from(radius) * angle.cos()).round() as i32;
                let z = center[2] + (f64::from(radius) * angle.sin()).round() as i32;
                let at = [x, center[1], z];
                let rot = facing_center(at, center);
                let (low, high) = site::territory(plan, at, rot);
                if taken.iter().any(|other| overlaps(*other, (low, high))) {
                    continue;
                }
                let inner = blueprints::bounds(plan, at, rot);
                let Some(y) = ground(host, dimension, &columns(inner.0, inner.1, at)) else { continue };
                taken.push((low, high));
                out.push(Ruin { dimension: dimension.to_owned(), pos: [x, y, z], plan: plan_index, rot });
                break 'search;
            }
        }
    }
    out
}

/// Blocos da ruína: camadas de baixo da planta, um subconjunto que depende só do lugar. Sem ar,
/// sem móveis e sem blocos protegidos (baús, camas, portas...).
pub(crate) fn ruin_cells(plan: &blueprints::Blueprint, pos: [i32; 3], rot: i32) -> Vec<([i32; 3], usize)> {
    let seed = seed(pos) ^ pos[1] as u32 as u64;
    plan.cells.iter().enumerate().filter_map(|(order, cell)| {
        let (spot, index) = blueprints::decode(*cell);
        let layer = spot[1] as usize;
        let keep = *KEEP.get(layer)?;
        let block = blueprints::PALETTE[index];
        if index == 0 || !blueprints::PROPS[index].is_empty() || blueprints::protected(block)
            || ATTACHED.iter().any(|part| block.contains(part))
        {
            return None;
        }
        (mix(seed ^ order as u64) % 100 < keep).then(|| (blueprints::world(plan, pos, rot, spot), index))
    }).collect()
}

/// Carimba a ruína em volta do canteiro em `pos`. Não mexe no canteiro nem em blocos protegidos.
pub(crate) fn stamp(host: &mut dyn Host, pos: [i32; 3], plan: &blueprints::Blueprint, rot: i32) {
    let mut palette: Vec<String> = Vec::new();
    let mut runs = Vec::new();
    for (at, index) in ruin_cells(plan, pos, rot) {
        if at == pos || blueprints::protected(&host.block(at)) {
            continue;
        }
        let block = blueprints::PALETTE[index];
        let slot = palette.iter().position(|known| known == block).unwrap_or_else(|| {
            palette.push(block.to_owned());
            palette.len() - 1
        });
        runs.push(NativeBlockRun { x: at[0], y: at[1], z: at[2], height: 1, surface: None, block: slot as u32, only: None });
    }
    if !runs.is_empty() {
        let _ = host.edit_blocks(host.dimension(), &NativeBlockEdit { palette, filters: Vec::new(), runs });
    }
}

// ---------- Arquivo do mundo ----------

fn file(world: &std::path::Path) -> PathBuf {
    world.join("veio").join(FILE)
}

fn encode(pending: &[Ruin]) -> String {
    let mut text = format!("{HEADER}\n");
    for ruin in pending {
        let [x, y, z] = ruin.pos;
        text.push_str(&format!("{} {x} {y} {z} {} {}\n", ruin.dimension, blueprints::BLUEPRINTS[ruin.plan].key, ruin.rot));
    }
    text
}

fn decode(text: &str) -> Option<Vec<Ruin>> {
    let mut lines = text.lines();
    if lines.next()? != HEADER {
        return None;
    }
    Some(lines.filter_map(|line| {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let [dimension, x, y, z, key, rot] = parts.as_slice() else { return None };
        let plan = blueprints::BLUEPRINTS.iter().position(|plan| plan.key == *key)?;
        Some(Ruin { dimension: (*dimension).to_owned(), pos: [x.parse().ok()?, y.parse().ok()?, z.parse().ok()?],
            plan, rot: rot.parse::<i32>().ok()?.rem_euclid(4) })
    }).collect())
}

fn save(world: &std::path::Path, pending: &[Ruin]) {
    let path = file(world);
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let temp = path.with_extension("tmp");
    if std::fs::write(&temp, encode(pending)).is_ok() {
        let _ = std::fs::rename(temp, path);
    }
}

/// Lê o arquivo do mundo. `None`: as ruínas ainda não foram postas neste mundo.
fn load(world: &std::path::Path) -> Option<Vec<Ruin>> {
    std::fs::read_to_string(file(world)).ok().and_then(|text| decode(&text))
}

fn sync(state: &mut State, world: PathBuf) -> bool {
    if state.synced && state.world.as_ref() == Some(&world) {
        return true;
    }
    let found = load(&world);
    state.pending = found.clone().unwrap_or_default();
    state.world = Some(world);
    state.synced = true;
    ACTIVE.store(!state.pending.is_empty(), Ordering::Release);
    found.is_some()
}

/// Chegada ao centro da vila `center` (só a primeira vez por mundo): escolhe e põe as ruínas.
pub(crate) fn arrive(host: &mut dyn Host, dimension: &str, center: [i32; 3]) {
    let Some(world) = host.world_dir() else { return };
    let Ok(mut state) = STATE.lock() else { return };
    if sync(&mut state, world.clone()) {
        return;
    }
    let fresh = host.level(dimension).and_then(|level| level.game_time(host)).is_some_and(|time| time <= FRESH);
    if !fresh {
        // Mundo que já existia antes das ruínas: nada muda, e nem vai mudar depois.
        save(&world, &[]);
        return;
    }
    let chosen = choose(host, dimension, center);
    let mut placed = Vec::new();
    for ruin in chosen {
        let [x, y, z] = ruin.pos;
        let command = format!("execute in {dimension} if block {x} {y} {z} #minecraft:replaceable run setblock {x} {y} {z} {}[facing={}] replace",
            site::SITE, FACINGS[ruin.rot.rem_euclid(4) as usize]);
        if host.run_command(&command) {
            placed.push(ruin);
        }
    }
    if !placed.is_empty() {
        host.run_command("tellraw @a {\"translate\":\"veio.spawn.ruins\",\"color\":\"gold\"}");
    }
    save(&world, &placed);
    ACTIVE.store(!placed.is_empty(), Ordering::Release);
    state.pending = placed;
}

/// Primeiro tick de um canteiro: se a vila o pôs como ruína, devolve (planta, giro) e o tira da
/// lista. Sem ruína à espera, sai na primeira leitura.
pub(crate) fn claim(host: &dyn Host, dimension: &str, pos: [i32; 3]) -> Option<(usize, i32)> {
    if !ACTIVE.load(Ordering::Acquire) {
        return None;
    }
    let mut state = STATE.lock().ok()?;
    if !state.synced {
        let world = host.world_dir()?;
        sync(&mut state, world);
    }
    let index = state.pending.iter().position(|ruin| ruin.dimension == dimension && ruin.pos == pos)?;
    let ruin = state.pending.remove(index);
    if let Some(world) = state.world.clone() {
        save(&world, &state.pending);
    }
    ACTIVE.store(!state.pending.is_empty(), Ordering::Release);
    Some((ruin.plan, ruin.rot))
}
