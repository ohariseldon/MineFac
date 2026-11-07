//! Poluição industrial e ataques noturnos, no espírito do Factorio, mas sem quebrar o Minecraft.
//!
//! - Máquinas trabalhando emitem poluição no chunk em que estão. A cada 64 ticks a poluição se
//!   espalha aos chunks vizinhos e uma parte se dissipa.
//! - O primeiro aviso chega pelo chat quando um chunk passa de `NOTICE`. Ataques só começam um dia
//!   inteiro depois desse aviso, e só à noite, com alerta ao anoitecer.
//! - Saqueadores (`veio:raider`, um zumbi) vêm à noite atrás de quem está na fábrica. Se ninguém
//!   os enfrenta, golpeiam a máquina mais próxima: quatro golpes e ela para, até um kit de reparo.
//!   Eles queimam ao sol.
//!
//! O estado fica em memória e recomeça ao reabrir o mundo; a poluição volta em poucos minutos.
use super::*;
use horizon_client::mobs::Mob;
use horizon_client::mobs::goals::{self, Cx, Goal};
use horizon_client::mod_api::{Compound, NativeHeightmap, ParticleOptions, ParticleType, Text};
use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, AtomicI64, Ordering};

/// Unidades de poluição. Um chunk em equilíbrio guarda cerca de 53 vezes a emissão por segundo.
pub(super) const NOTICE: f32 = 400.0;
pub(super) const RAID: f32 = 1200.0;
const STEP: i64 = 64;
const ABSORB: f32 = 0.02;
const SPREAD: f32 = 0.04;
const SPREAD_ABOVE: f32 = 20.0;
const DAY: i64 = 24000;
const MAX_WAVE: u32 = 12;
const RAIDER: &str = "veio:raider";
/// Dano de um golpe. Quatro golpes (4 s) param a máquina.
pub(super) const BLOW: i32 = 25;
/// O saqueador corre até o jogador neste raio; mais perto, o ataque vanilla assume.
const CHASE: f64 = 40.0;
const HAND: f64 = 2.8;

/// Emissão por segundo de trabalho.
pub(super) fn emission(block: &str) -> f32 {
    match block {
        GENERATOR => 6.0,
        DRILL | CRUSHER | GEOTHERMAL => 2.0,
        DRILL_MK2 | ALLOY => 4.0,
        PRESS | ASSEMBLER | CUTTER | INTEGRATOR | ELECTRONICS => 1.0,
        _ => production::emission(block),
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
/// Um passo de difusão por tempo de jogo, por dimensão. O atalho é por dimensão: Overworld e
/// Nether não compartilham o mesmo relógio de passo.
static STEP_OVERWORLD: AtomicI64 = AtomicI64::new(i64::MIN);
static STEP_NETHER: AtomicI64 = AtomicI64::new(i64::MIN);
static STEP_END: AtomicI64 = AtomicI64::new(i64::MIN);

fn step_slot(dimension: &str) -> Option<&'static AtomicI64> {
    match dimension {
        "minecraft:overworld" => Some(&STEP_OVERWORLD),
        "minecraft:the_nether" => Some(&STEP_NETHER),
        "minecraft:the_end" => Some(&STEP_END),
        _ => None,
    }
}

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
        id if production::burns(id) => number(entity, "Burn") > 0,
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

/// Chamado no tick do mundo e no das máquinas. `world.updated` garante um passo por dimensão.
pub(super) fn update(host: &mut dyn Host, time: i64) {
    if time.rem_euclid(STEP) != 0 {
        return;
    }
    let dimension = host.dimension();
    // A primeira máquina (ou o tick do mundo) faz o passo; as outras não disputam o mutex.
    if let Some(slot) = step_slot(dimension)
        && slot.swap(time, Ordering::AcqRel) == time
    {
        return;
    }
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

/// Quatro criaturas na primeira onda; cresce com a poluição e com as noites, até doze.
pub(super) fn wave_size(level: f32, waves: u32) -> u32 {
    let extra = ((level - RAID).max(0.0) / 400.0) as u32;
    (4 + waves.saturating_sub(1) + extra).min(MAX_WAVE)
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
    let factory = [f64::from(anchor[0]) + 0.5, f64::from(anchor[1]), f64::from(anchor[2]) + 0.5];
    let player = host.server_players().into_iter()
        .filter(|player| player.dimension == dimension)
        .map(|player| {
            let distance = (0..3).map(|axis| (player.position[axis] - factory[axis]).powi(2)).sum::<f64>();
            (distance, player.position)
        })
        .filter(|(distance, _)| *distance < 64.0 * 64.0)
        .min_by(|a, b| a.0.total_cmp(&b.0))
        .map(|(_, at)| at);
    // Na cara de quem está na fábrica; senão, a 18 blocos da máquina, para chegar rápido.
    let (center, radius) = player.map_or((factory, 18.0), |at| (at, 14.0));
    let angle = (time.rem_euclid(7919) as f64) * 0.0123;
    host.play_sound("minecraft:event.raid.horn", center, 16, 1.2, 0.7);
    for index in 0..size {
        let spread = f64::from(index) * (std::f64::consts::TAU / f64::from(size.max(1)));
        let x = center[0] + ((angle + spread).cos() * radius);
        let z = center[2] + ((angle + spread).sin() * radius);
        let Ok(heights) = host.surface_heights(dimension, &[(x.floor() as i32, z.floor() as i32)], NativeHeightmap::MotionBlocking) else { continue };
        let Some(Some(y)) = heights.first().copied() else { continue };
        host.summon(dimension, RAIDER, [x, f64::from(y), z], Compound::default());
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
                hit([x, y, z], BLOW);
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

static PLAYERS: Mutex<Vec<(String, [f64; 3])>> = Mutex::new(Vec::new());
static VILLAGERS: Mutex<Vec<(u32, [f64; 3])>> = Mutex::new(Vec::new());
static STRIKES: Mutex<Vec<u32>> = Mutex::new(Vec::new());
static WOUNDS: Mutex<Vec<(u32, i32)>> = Mutex::new(Vec::new());
/// Golpes até matar um aldeão. O posto fica sem trabalhador.
pub(super) const VILLAGER_BLOWS: i32 = 4;

/// Posições de quem está no mundo, para os saqueadores caçarem sem consultar o host no objetivo.
pub(super) fn note_players(host: &dyn Host) {
    let list = host.server_players().into_iter().map(|player| (player.dimension, player.position)).collect();
    if let Ok(mut players) = PLAYERS.lock() {
        *players = list;
    }
    let people = production::ranch::villager_positions(host, "minecraft:overworld");
    if let Ok(mut villagers) = VILLAGERS.lock() {
        villagers.clear();
        villagers.extend(people.iter().copied());
    }
}

fn nearest_villager(at: [f64; 3]) -> Option<(u32, [f64; 3])> {
    if let Some(guard) = workforce::barracks::shield(at) {
        return Some(guard);
    }
    VILLAGERS.lock().ok()?.iter()
        .map(|(id, pos)| ((pos[0] - at[0]).hypot(pos[2] - at[2]), *id, *pos))
        .filter(|(distance, _, _)| *distance <= 28.0)
        .min_by(|a, b| a.0.total_cmp(&b.0))
        .map(|(_, id, pos)| (id, pos))
}

fn player_in_melee(at: [f64; 3]) -> bool {
    PLAYERS.lock().is_ok_and(|players| players.iter().any(|(dimension, pos)| {
        dimension == "minecraft:overworld" && (pos[0] - at[0]).hypot(pos[2] - at[2]) <= 6.0
    }))
}

fn strike(id: u32) {
    if let Ok(mut queue) = STRIKES.lock()
        && queue.len() < 64
    {
        queue.push(id);
    }
}

/// Aplica os golpes nos aldeões. No quarto, o aldeão morre e o posto dele fica vazio.
pub(super) fn harm_villagers(host: &mut dyn Host) {
    let queue = STRIKES.lock().map(|mut queue| std::mem::take(&mut *queue)).unwrap_or_default();
    if queue.is_empty() {
        return;
    }
    let mut dead = Vec::new();
    let mut hurt = Vec::new();
    if let Ok(mut wounds) = WOUNDS.lock() {
        for id in queue {
            let hits = if let Some((_, hits)) = wounds.iter_mut().find(|(seen, _)| *seen == id) {
                *hits += 1;
                *hits
            } else {
                wounds.push((id, 1));
                1
            };
            if hits >= workforce::barracks::toughness(id).unwrap_or(VILLAGER_BLOWS) {
                wounds.retain(|(seen, _)| *seen != id);
                dead.push(id);
            } else {
                hurt.push(id);
            }
        }
    }
    let place = |id: u32| VILLAGERS.lock().ok().and_then(|list| list.iter().find(|(seen, _)| *seen == id).map(|(_, at)| *at));
    for id in hurt {
        if let Some(at) = place(id) {
            host.play_sound("minecraft:entity.villager.hurt", at, 6, 0.8, 1.0);
        }
    }
    for id in dead {
        let at = place(id).unwrap_or([0.0, 64.0, 0.0]);
        if host.kill_mob("minecraft:overworld", id) {
            host.play_sound("minecraft:entity.villager.death", at, 8, 1.0, 1.0);
            for player in host.server_players() {
                if player.dimension == "minecraft:overworld"
                    && (player.position[0] - at[0]).hypot(player.position[2] - at[2]) < 64.0
                {
                    host.action_bar(player.uuid, Text::tr("veio.raid.villager_down", &[]));
                }
            }
        }
    }
}

fn nearest_player(at: [f64; 3]) -> Option<[f64; 3]> {
    PLAYERS.lock().ok()?.iter()
        .filter(|(dimension, _)| dimension == "minecraft:overworld")
        .map(|(_, pos)| {
            let distance = (pos[0] - at[0]).hypot(pos[2] - at[2]);
            (distance, *pos)
        })
        .filter(|(distance, _)| *distance <= CHASE && *distance > HAND)
        .min_by(|a, b| a.0.total_cmp(&b.0))
        .map(|(_, pos)| pos)
}

/// Corre até o jogador. Perto dele, sai de cena e o ataque do zumbi acerta.
#[derive(Clone, Debug, Default)]
struct Hunt {
    goal: Option<[f64; 3]>,
}

impl Goal for Hunt {
    fn flags(&self) -> u8 {
        goals::MOVE
    }
    fn can_use(&mut self, m: &mut Mob, _cx: &mut Cx) -> bool {
        self.goal = nearest_player([m.position.x, m.position.y, m.position.z]);
        self.goal.is_some()
    }
    fn start(&mut self, m: &mut Mob, cx: &mut Cx) {
        if let Some(goal) = self.goal {
            m.nav_move_to(cx.w, goal, 1.25);
        }
    }
    fn tick(&mut self, m: &mut Mob, cx: &mut Cx) {
        let Some(goal) = self.goal else { return };
        if m.ai.nav.is_done() {
            m.nav_move_to(cx.w, goal, 1.25);
        }
    }
    fn box_clone(&self) -> Box<dyn Goal> {
        Box::new(self.clone())
    }
}

/// Zumbi, esqueleto, aranha e saqueador vão até o aldeão e batem. O jogador em luta corpo a corpo vem primeiro.
#[derive(Clone, Debug, Default)]
struct Prey {
    id: u32,
    at: [f64; 3],
    cooldown: i32,
}

impl Goal for Prey {
    fn flags(&self) -> u8 {
        goals::MOVE
    }
    fn can_use(&mut self, m: &mut Mob, _cx: &mut Cx) -> bool {
        let here = [m.position.x, m.position.y, m.position.z];
        if player_in_melee(here) {
            return false;
        }
        if let Some((id, at)) = nearest_villager(here) {
            self.id = id;
            self.at = at;
            true
        } else {
            false
        }
    }
    fn start(&mut self, m: &mut Mob, cx: &mut Cx) {
        m.nav_move_to(cx.w, self.at, 1.15);
    }
    fn tick(&mut self, m: &mut Mob, cx: &mut Cx) {
        let close = (m.position.x - self.at[0]).hypot(m.position.z - self.at[2]) < 2.2
            && (m.position.y - self.at[1]).abs() < 2.0;
        self.cooldown -= 1;
        if close {
            if self.cooldown <= 0 {
                strike(self.id);
                self.cooldown = 20;
            }
        } else if m.ai.nav.is_done() && self.cooldown <= 0 {
            m.nav_move_to(cx.w, self.at, 1.15);
            self.cooldown = 10;
        }
    }
    fn box_clone(&self) -> Box<dyn Goal> {
        Box::new(self.clone())
    }
}

fn night_hunter(kind: &str) -> bool {
    matches!(kind,
        "minecraft:zombie" | "minecraft:husk" | "minecraft:drowned" | "minecraft:zombie_villager"
        | "minecraft:skeleton" | "minecraft:stray" | "minecraft:bogged"
        | "minecraft:spider" | "minecraft:cave_spider"
        | "minecraft:pillager" | "minecraft:vindicator" | "veio:raider")
}

pub(super) fn register_goals(mob: &mut Mob) {
    if mob.kind.id == RAIDER {
        // 1: caça o jogador. 2: aldeão, e o zumbi bate no jogador quando chega. 3: máquina.
        mob.ai.goals.add(1, Hunt::default());
        mob.ai.goals.add(2, Prey::default());
        mob.ai.goals.add(3, Sabotage::default());
    } else if night_hunter(&mob.kind.id) {
        mob.ai.goals.add(3, Prey::default());
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
