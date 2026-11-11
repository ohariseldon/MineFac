//! Poluição industrial e ataques noturnos, no espírito do Factorio, mas sem quebrar o Minecraft.
//!
//! - Máquinas trabalhando emitem poluição no chunk em que estão. A cada 64 ticks a poluição se
//!   espalha aos chunks vizinhos e uma parte se dissipa.
//! - O primeiro aviso chega pelo chat quando um chunk passa de `NOTICE`. Ataques só começam um dia
//!   inteiro depois desse aviso, e só à noite, com alerta ao anoitecer.
//! - Uma onda chega à noite pela borda da fábrica, junta e de um lado (a direção muda a cada noite),
//!   com o toque de corneta. Ela traz **saqueadores** (`veio:raider`, zumbi com faro de 40 blocos),
//!   **arqueiros** (esqueletos, a partir da 3ª noite de ataque) e **brutos**
//!   (`veio:raider_brute`, zumbi grande e lento, a partir da 5ª). Todos queimam ao sol.
//! - O combate é o do Minecraft: saqueadores e brutos atacam jogadores, aldeões e golens como
//!   zumbis; arqueiros atiram neles. Esqueletos e aranhas da noite também passam a caçar aldeões
//!   (alvo vanilla `NearestTarget`), com dano de verdade: o aldeão foge, golens defendem.
//! - Sem ninguém por perto para atacar, saqueadores golpeiam a máquina poluidora mais próxima
//!   (quatro golpes param a máquina, até um kit de reparo). O Bruto vai primeiro às máquinas e
//!   bate o dobro.
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
const BRUTE: &str = "veio:raider_brute";
const ARCHER: &str = "minecraft:skeleton";
/// O Bruto derruba uma máquina em dois golpes.
pub(super) const BRUTE_BLOW: i32 = 50;
/// Noites de ataque a partir das quais a onda traz arqueiros e brutos.
pub(super) const ARCHERS_FROM: u32 = 3;
pub(super) const BRUTES_FROM: u32 = 5;
/// A onda nasce a esta distância da máquina-alvo.
const ARRIVAL: f64 = 26.0;

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

/// Noites de ataque já vividas nesta dimensão (a onda de hoje incluída).
fn waves_so_far(dimension: &str) -> u32 {
    with_world(dimension, |world| world.waves).unwrap_or(1)
}

fn act(host: &mut dyn Host, dimension: &str, plan: Plan, time: i64) {
    let (anchor, key) = match plan {
        Plan::Notice(anchor) => (anchor, "veio.raid.notice"),
        Plan::Warning(anchor) => (anchor, "veio.raid.warning"),
        Plan::Wave(anchor, size) => {
            if spawn_wave(host, dimension, anchor, size, waves_so_far(dimension), time) == 0 {
                with_world(dimension, |world| {
                    world.wave_day = -1;
                    world.waves = world.waves.saturating_sub(1);
                });
                return;
            }
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

/// Quem vem numa onda de `size` na noite de ataque `waves`: (saqueadores, arqueiros, brutos).
pub(super) fn wave_mix(size: u32, waves: u32) -> (u32, u32, u32) {
    let archers = if waves >= ARCHERS_FROM { size / 4 } else { 0 };
    let brutes = if waves >= BRUTES_FROM { (size / 6).max(1) } else { 0 };
    (size - archers - brutes, archers, brutes)
}

fn spawn_wave(host: &mut dyn Host, dimension: &str, anchor: [i32; 3], size: u32, waves: u32, time: i64) -> u32 {
    let factory = [f64::from(anchor[0]) + 0.5, f64::from(anchor[1]), f64::from(anchor[2]) + 0.5];
    // Uma frente: um arco de 90 graus do lado do Acampamento dos Saqueadores (`camp_angle`), que
    // oscila um pouco a cada noite. Os saqueadores farejam o jogador a 40 blocos; quem está na
    // fábrica os vê chegar.
    let night = time.div_euclid(DAY);
    let angle = match camp_angle() {
        Some(camp) => camp + (night.rem_euclid(5) as f64 - 2.0) * 0.3,
        None => (night.rem_euclid(97) as f64) * 2.399,
    };
    let (raiders, archers, brutes) = wave_mix(size, waves);
    let kinds = std::iter::repeat_n(RAIDER, raiders as usize)
        .chain(std::iter::repeat_n(ARCHER, archers as usize))
        .chain(std::iter::repeat_n(BRUTE, brutes as usize));
    let mut first = None;
    let mut spawned = 0;
    for (index, kind) in kinds.enumerate() {
        let spread = (index as f64 / f64::from(size.max(1)) - 0.5) * std::f64::consts::FRAC_PI_2;
        let x = factory[0] + (angle + spread).cos() * ARRIVAL;
        let z = factory[2] + (angle + spread).sin() * ARRIVAL;
        let Ok(heights) = host.surface_heights(dimension, &[(x.floor() as i32, z.floor() as i32)], NativeHeightmap::MotionBlocking) else { continue };
        let Some(Some(y)) = heights.first().copied() else { continue };
        let at = [x, f64::from(y), z];
        if host.summon(dimension, kind, at, Compound::default()).is_some() {
            first.get_or_insert(at);
            spawned += 1;
        }
    }
    if let Some(at) = first {
        host.play_sound("minecraft:event.raid.horn", at, 16, 4.0, 0.7);
    }
    spawned
}

/// De que lado da vila fica o Acampamento dos Saqueadores (radianos; 0 é leste, e cresce para o
/// sul). Fixo para cada vila: as ondas sempre vêm de lá. No mapa fixo, o acampamento tem lugar na
/// planta (`docs/mapa/costa-oeste`); num mundo aleatório, o lado sai do centro da vila.
pub(super) fn camp_angle() -> Option<f64> {
    let ([x, _, z], _) = workforce::spawn::center()?;
    let hash = (x as i64).wrapping_mul(73_856_093) ^ (z as i64).wrapping_mul(19_349_663);
    Some((hash.rem_euclid(360) as f64).to_radians())
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

/// Objetivo do saqueador: andar até a máquina poluidora e golpeá-la. Para o saqueador fica abaixo
/// dos ataques do zumbi, então jogadores, aldeões e golens por perto vêm primeiro; o Bruto o
/// tem acima: máquinas primeiro.
#[derive(Clone, Debug, Default)]
pub(super) struct Sabotage {
    target: Option<[i32; 3]>,
    cooldown: i32,
    blow: i32,
    /// Chamadas até procurar de novo uma máquina (meio segundo a um segundo).
    wait: u8,
}

impl Sabotage {
    fn new(blow: i32) -> Self {
        Self { blow, ..Self::default() }
    }
}

impl Goal for Sabotage {
    fn flags(&self) -> u8 {
        goals::MOVE
    }
    fn can_use(&mut self, m: &mut Mob, _cx: &mut Cx) -> bool {
        // A lista de máquinas tem trava: um saqueador não a consulta em todo tick.
        if self.wait > 0 {
            self.wait -= 1;
            return false;
        }
        self.wait = 19;
        self.target = target_near([m.position.x, m.position.y, m.position.z]);
        self.target.is_some()
    }
    fn can_continue(&mut self, _m: &mut Mob, _cx: &mut Cx) -> bool {
        if self.wait > 0 {
            self.wait -= 1;
            return self.target.is_some();
        }
        self.wait = 19;
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
                hit([x, y, z], if self.blow > 0 { self.blow } else { BLOW });
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

/// Mobs noturnos que não caçam aldeões no vanilla e passam a caçar. Zumbis, saqueadores e
/// pillagers já caçam.
pub(super) fn villager_hunter(kind: &str) -> bool {
    matches!(kind, "minecraft:skeleton" | "minecraft:stray" | "minecraft:bogged" | "minecraft:spider" | "minecraft:cave_spider")
}

pub(super) fn register_goals(mob: &mut Mob) {
    use horizon_client::mobs::Species;
    use horizon_client::mobs::goals::{NearestTarget, Prey};
    match mob.kind.id.as_str() {
        // Abaixo do ataque corpo a corpo do zumbi (2) e acima do passeio (7).
        RAIDER => mob.ai.goals.add(3, Sabotage::new(BLOW)),
        // Acima do ataque: o Bruto só briga quando não há máquina poluidora perto.
        BRUTE => mob.ai.goals.add(1, Sabotage::new(BRUTE_BLOW)),
        kind if villager_hunter(kind) => {
            // O alvo vanilla (`NearestAttackableTargetGoal`): o ataque do próprio mob (arco,
            // mordida) faz o resto, com dano de verdade. Depois de jogador e golem.
            mob.ai.targets.add(3, NearestTarget::new(Prey::Species(vec![Species::Villager]), true));
        }
        _ => {}
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

#[cfg(test)]
pub(super) fn seed_for_test(dimension: &str, cells: &[((i32, i32), f32)], anchor: [i32; 3], seen: i64, noticed_at: Option<i64>) {
    with_world(dimension, |world| {
        *world = World {
            name: dimension.to_owned(), cells: cells.iter().copied().collect(), anchors: HashMap::from([(anchor, seen)]),
            noticed_at, warned_day: -1, wave_day: -1, ..World::default()
        };
    });
}

/// Para o tutorial: o chunk mais poluído da dimensão, se o aviso já veio e quantas ondas vieram.
pub(super) fn alarm(dimension: &str) -> (f32, bool, u32) {
    with_world(dimension, |world| (world.cells.values().copied().fold(0.0, f32::max), world.noticed_at.is_some(), world.waves))
        .unwrap_or((0.0, false, 0))
}

#[cfg(test)]
pub(super) fn waves_for_test(dimension: &str) -> u32 {
    with_world(dimension, |world| world.waves).unwrap_or(0)
}
