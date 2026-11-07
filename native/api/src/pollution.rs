//! Poluição industrial e ataques noturnos, no espírito do Factorio, mas sem quebrar o Minecraft.
//!
//! - Máquinas trabalhando emitem poluição no chunk em que estão. A cada 64 ticks a poluição se
//!   espalha aos chunks vizinhos e uma parte se dissipa.
//! - O primeiro aviso chega pelo chat quando um chunk passa de `NOTICE`. Ataques só começam um dia
//!   inteiro depois desse aviso, e só à noite, com alerta ao anoitecer.
//! - Saqueadores (`veio:raider`, um zumbi) andam até a máquina poluidora mais próxima e a danificam.
//!   Nada é destruído: a máquina para até receber um kit de reparo. Eles queimam ao sol.
//!
//! O estado fica em memória e recomeça ao reabrir o mundo; a poluição volta em poucos minutos.
use super::*;
use horizon_client::mobs::Mob;
use horizon_client::mobs::goals::{self, Cx, Goal};
use horizon_client::mod_api::{Compound, NativeHeightmap, ParticleOptions, ParticleType, Text};
use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};

/// Unidades de poluição. Um chunk em equilíbrio guarda cerca de 53 vezes a emissão por segundo.
pub(super) const NOTICE: f32 = 400.0;
pub(super) const RAID: f32 = 1200.0;
const STEP: i64 = 64;
const ABSORB: f32 = 0.02;
const SPREAD: f32 = 0.04;
const SPREAD_ABOVE: f32 = 20.0;
const DAY: i64 = 24000;
const MAX_WAVE: u32 = 8;
const RAIDER: &str = "veio:raider";

/// Emissão por segundo de trabalho.
pub(super) fn emission(block: &str) -> f32 {
    match block {
        GENERATOR => 6.0,
        DRILL | CRUSHER | GEOTHERMAL => 2.0,
        DRILL_MK2 | ALLOY => 4.0,
        PRESS | ASSEMBLER | CUTTER | INTEGRATOR | ELECTRONICS => 1.0,
        _ => 0.0,
    }
}

#[derive(Default)]
struct World {
    name: String,
    cells: HashMap<(i32, i32), f32>,
    /// Máquinas que emitiram há pouco e quando: os alvos dos saqueadores.
    anchors: HashMap<[i32; 3], i64>,
    updated: i64,
    noticed_at: Option<i64>,
    warned_day: i64,
    wave_day: i64,
    waves: u32,
}

static WORLDS: Mutex<Vec<World>> = Mutex::new(Vec::new());
static SABOTAGE: Mutex<Vec<([i32; 3], i32)>> = Mutex::new(Vec::new());
static SABOTAGE_PENDING: AtomicBool = AtomicBool::new(false);

fn with_world<T>(dimension: &str, change: impl FnOnce(&mut World) -> T) -> Option<T> {
    let mut worlds = WORLDS.lock().ok()?;
    let index = match worlds.iter().position(|world| world.name == dimension) {
        Some(index) => index,
        None => {
            worlds.push(World { name: dimension.to_owned(), warned_day: -1, wave_day: -1, ..World::default() });
            worlds.len() - 1
        }
    };
    Some(change(&mut worlds[index]))
}

fn chunk([x, _, z]: [i32; 3]) -> (i32, i32) {
    (x >> 4, z >> 4)
}

pub(super) fn local(dimension: &str, pos: [i32; 3]) -> f32 {
    with_world(dimension, |world| world.cells.get(&chunk(pos)).copied().unwrap_or(0.0)).unwrap_or(0.0)
}

/// Multiplicador do módulo instalado sobre a poluição.
fn module_factor(entity: &ModBlockEntity) -> f32 {
    upgrades::effect(entity).pollution
}

/// Conta os ticks de trabalho e entrega a poluição ao chunk uma vez por segundo.
pub(super) fn emit(host: &dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity, time: i64) {
    let rate = emission(&entity.id);
    if rate <= 0.0 {
        return;
    }
    let working = match entity.id.as_str() {
        GENERATOR | GEOTHERMAL => number(entity, "Burn") > 0,
        _ => number(entity, "Status") == 1,
    };
    let ticks = number(entity, "SmogTicks") + i32::from(working);
    if (time + i64::from(pos[0] ^ pos[2])).rem_euclid(20) != 0 {
        set_number(entity, "SmogTicks", ticks);
        return;
    }
    set_number(entity, "SmogTicks", 0);
    if ticks == 0 {
        return;
    }
    let amount = rate * ticks as f32 / 20.0 * module_factor(entity);
    with_world(host.dimension(), |world| {
        *world.cells.entry(chunk(pos)).or_default() += amount;
        world.anchors.insert(pos, time);
    });
}

/// Plantações absorvem poluição: cada colheita limpa um pouco o chunk.
pub(super) fn absorb(dimension: &str, pos: [i32; 3], amount: f32) {
    with_world(dimension, |world| {
        if let Some(cell) = world.cells.get_mut(&chunk(pos)) {
            *cell = (*cell - amount).max(0.0);
        }
    });
}

/// Espalha e dissipa: um passo a cada 64 ticks por dimensão.
pub(super) fn diffuse(cells: &HashMap<(i32, i32), f32>) -> HashMap<(i32, i32), f32> {
    let mut next = HashMap::with_capacity(cells.len());
    for (&(x, z), &amount) in cells {
        let mut keep = amount * (1.0 - ABSORB);
        if amount > SPREAD_ABOVE {
            let share = amount * SPREAD / 4.0;
            keep -= share * 4.0;
            for neighbor in [(x + 1, z), (x - 1, z), (x, z + 1), (x, z - 1)] {
                *next.entry(neighbor).or_insert(0.0) += share;
            }
        }
        *next.entry((x, z)).or_insert(0.0) += keep;
    }
    next.retain(|_, amount| *amount >= 0.5);
    next
}

/// Chamado no tick das máquinas; só um deles por dimensão faz o passo.
pub(super) fn update(host: &mut dyn Host, time: i64) {
    if time.rem_euclid(STEP) != 0 {
        return;
    }
    let dimension = host.dimension();
    let day_time = host.day_time();
    let plan = with_world(dimension, |world| {
        if world.updated == time {
            return None;
        }
        world.updated = time;
        world.cells = diffuse(&world.cells);
        world.anchors.retain(|_, seen| time - *seen < 1200);
        if dimension != "minecraft:overworld" {
            return None;
        }
        plan_raid(world, time, day_time)
    }).flatten();
    if let Some(plan) = plan {
        act(host, dimension, plan, time);
    }
}

#[derive(Debug, PartialEq)]
pub(super) enum Plan {
    Notice([i32; 3]),
    Warning([i32; 3]),
    Wave([i32; 3], u32),
}

/// Decide avisos e ondas a partir do chunk mais poluído.
fn plan_raid(world: &mut World, time: i64, day_time: i64) -> Option<Plan> {
    let (&hot, &level) = world.cells.iter().max_by(|a, b| a.1.total_cmp(b.1))?;
    let anchor = *world.anchors.keys().min_by_key(|pos| {
        let (x, z) = chunk(**pos);
        (x - hot.0).abs() + (z - hot.1).abs()
    })?;
    let day = day_time.div_euclid(DAY);
    let hour = day_time.rem_euclid(DAY);
    if level >= NOTICE && world.noticed_at.is_none() {
        world.noticed_at = Some(time);
        return Some(Plan::Notice(anchor));
    }
    let ready = world.noticed_at.is_some_and(|at| time - at >= DAY);
    if level < RAID || !ready || world.wave_day == day {
        return None;
    }
    if (11500..13000).contains(&hour) && world.warned_day != day {
        world.warned_day = day;
        return Some(Plan::Warning(anchor));
    }
    if (14000..20000).contains(&hour) {
        world.wave_day = day;
        world.waves += 1;
        return Some(Plan::Wave(anchor, wave_size(level, world.waves)));
    }
    None
}

/// Duas criaturas na primeira onda; cresce com a poluição e com as noites de ataque.
pub(super) fn wave_size(level: f32, waves: u32) -> u32 {
    let extra = ((level - RAID).max(0.0) / 600.0) as u32;
    (2 + waves.saturating_sub(1) / 2 + extra).min(MAX_WAVE)
}

fn act(host: &mut dyn Host, dimension: &str, plan: Plan, time: i64) {
    let (anchor, key) = match plan {
        Plan::Notice(anchor) => (anchor, "veio.raid.notice"),
        Plan::Warning(anchor) => (anchor, "veio.raid.warning"),
        Plan::Wave(anchor, size) => {
            spawn_wave(host, dimension, anchor, size, time);
            (anchor, "veio.raid.arrived")
        }
    };
    let center = [f64::from(anchor[0]), f64::from(anchor[1]), f64::from(anchor[2])];
    for player in host.server_players() {
        let near = player.dimension == dimension
            && (0..3).map(|axis| (player.position[axis] - center[axis]).powi(2)).sum::<f64>() < 160.0 * 160.0;
        if !near {
            continue;
        }
        if key == "veio.raid.notice" {
            host.tell(player.uuid, Text::tr(key, &[]));
            host.tell(player.uuid, Text::tr("veio.raid.notice_help", &[]));
        } else {
            host.action_bar(player.uuid, Text::tr(key, &[]));
        }
    }
}

fn spawn_wave(host: &mut dyn Host, dimension: &str, anchor: [i32; 3], size: u32, time: i64) {
    // Direção que muda a cada noite; os saqueadores chegam a 28 blocos da fábrica.
    let angle = (time.rem_euclid(7919) as f64) * 0.0123;
    for index in 0..size {
        let spread = f64::from(index) * 0.35;
        let x = anchor[0] + ((angle + spread).cos() * 28.0) as i32;
        let z = anchor[2] + ((angle + spread).sin() * 28.0) as i32;
        let Ok(heights) = host.surface_heights(dimension, &[(x, z)], NativeHeightmap::MotionBlocking) else { continue };
        let Some(Some(y)) = heights.first().copied() else { continue };
        host.summon(dimension, RAIDER, [f64::from(x) + 0.5, f64::from(y), f64::from(z) + 0.5], Compound::default());
    }
}

/// Golpes dos saqueadores nesta máquina desde o último tick.
pub(super) fn take_hits(pos: [i32; 3]) -> i32 {
    if !SABOTAGE_PENDING.load(Ordering::Relaxed) {
        return 0;
    }
    let Ok(mut queue) = SABOTAGE.lock() else { return 0 };
    let mut total = 0;
    queue.retain(|(at, damage)| {
        let hit = *at == pos;
        if hit { total += damage; }
        !hit
    });
    if queue.is_empty() {
        SABOTAGE_PENDING.store(false, Ordering::Relaxed);
    }
    total
}

fn hit(pos: [i32; 3], damage: i32) {
    if let Ok(mut queue) = SABOTAGE.lock() {
        if queue.len() < 256 {
            queue.push((pos, damage));
        }
        SABOTAGE_PENDING.store(true, Ordering::Relaxed);
    }
}

/// Aplica os golpes recebidos. Devolve `true` se a máquina está parada por dano.
pub(super) fn wear(host: &mut dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity) -> bool {
    let hits = take_hits(pos);
    if hits > 0 {
        set_number(entity, "Damage", (number(entity, "Damage") + hits).min(100));
        let at = [f64::from(pos[0]) + 0.5, f64::from(pos[1]) + 0.8, f64::from(pos[2]) + 0.5];
        host.send_particles(&ParticleOptions::simple(ParticleType::Crit), None, at, [0.3, 0.3, 0.3], 0.1, 8);
        host.play_sound("minecraft:entity.zombie.attack_iron_door", at, 5, 0.6, 1.2);
    }
    number(entity, "Damage") >= 100
}

/// Alvo de um saqueador: a máquina poluidora mais próxima a até 48 blocos.
fn target_near(at: [f64; 3]) -> Option<[i32; 3]> {
    let worlds = WORLDS.lock().ok()?;
    worlds.iter().flat_map(|world| world.anchors.keys())
        .map(|pos| {
            let distance = (0..3).map(|axis| (f64::from(pos[axis]) + 0.5 - at[axis]).powi(2)).sum::<f64>();
            (distance, *pos)
        })
        .filter(|(distance, _)| *distance < 48.0 * 48.0)
        .min_by(|a, b| a.0.total_cmp(&b.0))
        .map(|(_, pos)| pos)
}

fn still_polluting(pos: [i32; 3]) -> bool {
    WORLDS.lock().is_ok_and(|worlds| worlds.iter().any(|world| world.anchors.contains_key(&pos)))
}

/// Objetivo do saqueador: andar até a máquina e golpeá-la. Fica abaixo do ataque ao jogador,
/// então um jogador por perto continua sendo a prioridade do zumbi.
#[derive(Clone, Debug, Default)]
pub(super) struct Sabotage {
    target: Option<[i32; 3]>,
    cooldown: i32,
}

impl Goal for Sabotage {
    fn flags(&self) -> u8 {
        goals::MOVE
    }
    fn can_use(&mut self, m: &mut Mob, _cx: &mut Cx) -> bool {
        self.target = target_near([m.position.x, m.position.y, m.position.z]);
        self.target.is_some()
    }
    fn can_continue(&mut self, _m: &mut Mob, _cx: &mut Cx) -> bool {
        self.target.is_some_and(still_polluting)
    }
    fn start(&mut self, m: &mut Mob, cx: &mut Cx) {
        if let Some([x, y, z]) = self.target {
            m.nav_move_to(cx.w, [f64::from(x) + 0.5, f64::from(y), f64::from(z) + 0.5], 1.0);
        }
    }
    fn tick(&mut self, m: &mut Mob, cx: &mut Cx) {
        let Some([x, y, z]) = self.target else { return };
        let goal = [f64::from(x) + 0.5, f64::from(y), f64::from(z) + 0.5];
        let close = (m.position.x - goal[0]).powi(2) + (m.position.z - goal[2]).powi(2) < 2.4 * 2.4
            && (m.position.y - goal[1]).abs() < 2.0;
        self.cooldown -= 1;
        if close {
            if self.cooldown <= 0 {
                hit([x, y, z], 8);
                self.cooldown = 20;
            }
        } else if m.ai.nav.is_done() && self.cooldown <= 0 {
            m.nav_move_to(cx.w, goal, 1.0);
            self.cooldown = 20;
        }
    }
    fn box_clone(&self) -> Box<dyn Goal> {
        Box::new(self.clone())
    }
}

pub(super) fn register_goals(mob: &mut Mob) {
    if mob.kind.id == RAIDER {
        // Abaixo do ataque corpo a corpo do zumbi (2) e acima do passeio (7).
        mob.ai.goals.add(3, Sabotage::default());
    }
}

#[cfg(test)]
pub(super) fn plan_for_test(cells: &[((i32, i32), f32)], anchor: [i32; 3], noticed_at: Option<i64>, time: i64, day_time: i64) -> Option<Plan> {
    let mut world = World {
        cells: cells.iter().copied().collect(),
        anchors: HashMap::from([(anchor, time)]),
        noticed_at,
        warned_day: -1,
        wave_day: -1,
        ..World::default()
    };
    plan_raid(&mut world, time, day_time)
}
