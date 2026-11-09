//! Cidade pronta: a vila inicial do Veio, desenhada e gerada pelo mod (não mais a vila vanilla).
//!
//! - **Geração:** num mundo novo, `spawn` escolhe o terreno e chama `build`, que roda a função
//!   `veio:town/build` (gerada por `tools/build_town.py`). A função limpa todo o volume da cidade
//!   (copas, galhos, água rasa, morros baixos), nivela o chão e constrói praça, avenidas, ruas,
//!   comércio, casas, lotes reservados, lavouras e a cerca. Depois nascem `STARTING_VILLAGERS`
//!   aldeões nas casas, dois deles o casal fundador (`founders`).
//! - **Lotes reservados:** a casa do jogador (`home.rs`) vai para o lote do fundador, de frente
//!   para a praça; a oficina do tutorial vai para o lote do distrito industrial; o lote da
//!   prefeitura é o refúgio do casal fundador.
//! - **Piso e recuperação:** a cada minuto, com um jogador por perto (os chunks da cidade
//!   carregados), conta a população. Abaixo de 2 chega um casal na hora; abaixo da metade do
//!   recorde, uma família de imigrantes por dia, se houver cama e teto. Enquanto a cidade está
//!   abaixo da metade do recorde, os nascimentos ficam mais prováveis (`recovering`).
//!
//! O centro da cidade e o recorde de população ficam em `veio/town.txt`, na pasta do mundo.
use super::*;
use super::town_data as data;
use horizon_client::mod_api::{ParticleOptions, ParticleType, Text};
use spawn::OVERWORLD;
use std::path::{Path, PathBuf};

pub(crate) use data::{HALF, STARTING_VILLAGERS};
#[cfg(test)]
pub(crate) use data::{BEDS, SPOTS};

pub(crate) const FUNCTION: &str = "veio:town/build";
/// O piso: com menos aldeões que isto, chega um casal na hora.
pub(crate) const FLOOR: usize = 2;
/// A cada quanto a população é medida (1 min).
pub(crate) const CHECK_EVERY: u64 = 1200;
/// No máximo uma família de imigrantes por dia do jogo.
pub(crate) const FAMILY_EVERY: u64 = 24000;
/// Abaixo de `peak / RECOVERY_SHARE`, a cidade está se recuperando.
pub(crate) const RECOVERY_SHARE: usize = 2;
/// Recorde mínimo para falar em recuperação (uma cidade que nunca cresceu não "caiu").
pub(crate) const RECOVERY_MIN_PEAK: usize = 8;
/// Quem conta como morador: a até tantos blocos do centro, na horizontal.
pub(crate) const RADIUS: f64 = HALF as f64 + 20.0;
/// Só mede com um jogador a até tantos blocos do centro (senão os chunks podem estar fechados).
const PLAYER_NEAR: f64 = 96.0;
/// Crianças entre os moradores iniciais (as últimas casas da lista).
const STARTING_KIDS: usize = 4;

#[derive(Default)]
struct State {
    world: Option<PathBuf>,
    center: Option<[i32; 3]>,
    peak: usize,
    population: Option<usize>,
    last_family: Option<u64>,
    recovering: bool,
    dirty: bool,
}

#[cfg(not(test))]
static STATE: Mutex<State> = Mutex::new(State {
    world: None, center: None, peak: 0, population: None, last_family: None, recovering: false, dirty: false,
});

// Nos testes, cada teste (uma thread) tem a sua cidade: os testes rodam em paralelo.
#[cfg(test)]
thread_local! {
    static STATE: Mutex<State> = Mutex::new(State::default());
}

#[cfg(not(test))]
fn with_state<T>(f: impl FnOnce(&mut State) -> T) -> T {
    f(&mut STATE.lock().unwrap_or_else(|err| err.into_inner()))
}

#[cfg(test)]
fn with_state<T>(f: impl FnOnce(&mut State) -> T) -> T {
    STATE.with(|state| f(&mut state.lock().unwrap_or_else(|err| err.into_inner())))
}

// ---------- Lugares ----------

fn offset(center: [i32; 3], local: [i32; 3]) -> [i32; 3] {
    [center[0] + local[0], center[1] + local[1], center[2] + local[2]]
}

/// Centro da praça da cidade do Veio deste mundo (y no primeiro ar acima do chão).
pub(crate) fn center() -> Option<[i32; 3]> {
    with_state(|state| state.center)
}

/// Canto noroeste do lote da casa do jogador, no nível do piso.
pub(crate) fn founder_lot() -> Option<[i32; 3]> {
    center().map(|at| offset(at, data::FOUNDER_LOT))
}

/// Centro do lote da oficina (distrito industrial), no nível do primeiro ar.
pub(crate) fn workshop_lot() -> Option<[i32; 3]> {
    center().map(|at| offset(at, data::WORKSHOP))
}

/// Onde o casal fundador se recupera (o lote da prefeitura).
pub(crate) fn refuge() -> Option<[i32; 3]> {
    center().map(|at| offset(at, data::REFUGE))
}

/// A cidade perdeu mais da metade do recorde: nascimentos mais prováveis.
pub(crate) fn recovering() -> bool {
    with_state(|state| state.recovering)
}

// ---------- Construção ----------

/// Gera a cidade com o centro da praça em `site` e faz nascer os moradores. `false` se o
/// comando foi recusado (sem servidor ou função ausente).
pub(crate) fn build(host: &mut dyn Host, site: [i32; 3], now: u64) -> bool {
    let [x, y, z] = site;
    if !host.run_command(&format!("execute in {OVERWORLD} positioned {x} {y} {z} run function {FUNCTION}")) {
        return false;
    }
    with_state(|state| {
        state.center = Some(site);
        state.peak = STARTING_VILLAGERS;
        state.population = Some(STARTING_VILLAGERS);
        state.dirty = true;
    });
    for (index, spot) in data::SPOTS.iter().take(STARTING_VILLAGERS).enumerate() {
        let at = offset(site, *spot);
        let at = [f64::from(at[0]) + 0.5, f64::from(at[1]), f64::from(at[2]) + 0.5];
        if index < FOUNDERS_AT_START {
            founders::summon(host, OVERWORLD, at, index, now);
        } else {
            let kid = index >= STARTING_VILLAGERS - STARTING_KIDS;
            summon_villager(host, at, kid);
        }
    }
    founders::start_scan(now);
    save_now(host);
    true
}

/// Os dois primeiros lugares (os mais perto da praça) são do casal fundador.
const FOUNDERS_AT_START: usize = 2;

fn summon_villager(host: &mut dyn Host, at: [f64; 3], kid: bool) -> Option<u32> {
    let mut nbt = Compound::from([
        ("PersistenceRequired".into(), Tag::Byte(1)),
        ("VillagerData".into(), Tag::Compound(Compound::from([
            ("profession".into(), Tag::String("minecraft:none".into())),
            ("type".into(), Tag::String("minecraft:plains".into())),
            ("level".into(), Tag::Int(1)),
        ]))),
    ]);
    if kid {
        nbt.insert("Age".into(), Tag::Int(-24000));
    }
    host.summon(OVERWORLD, VILLAGER, at, nbt)
}

// ---------- Piso, imigrantes e recuperação ----------

/// Tick global: carrega o estado do mundo e, a cada minuto, mede a cidade.
pub(crate) fn tick(host: &mut dyn Host, now: u64) {
    if !now.is_multiple_of(20) {
        return;
    }
    load(host);
    if now.is_multiple_of(CHECK_EVERY) {
        check(host, now);
    }
    if now.is_multiple_of(600) && with_state(|state| state.dirty) {
        save_now(host);
    }
}

/// O que fazer com a população medida: nada, um casal (piso) ou uma família (recuperação).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Arrival { None, Couple, Family }

/// Regra do piso e da imigração. `beds` são as camas contadas perto do centro (se já contadas),
/// `cap` o teto de desenvolvimento, `since` ticks desde a última família.
pub(crate) fn arrival(population: usize, peak: usize, beds: Option<usize>, cap: usize, since: Option<u64>, daytime: bool) -> Arrival {
    if population < FLOOR {
        return Arrival::Couple;
    }
    let low = peak >= RECOVERY_MIN_PEAK && population * RECOVERY_SHARE < peak;
    let room = beds.is_none_or(|beds| beds >= population + 3) && population + 3 <= cap.min(village::MAX_CAP);
    let rested = since.is_none_or(|since| since >= FAMILY_EVERY);
    if low && room && rested && daytime { Arrival::Family } else { Arrival::None }
}

fn check(host: &mut dyn Host, now: u64) {
    let Some(center) = center() else { return };
    let here = [f64::from(center[0]), f64::from(center[1]), f64::from(center[2])];
    let near = |at: &[f64; 3]| (at[0] - here[0]).abs() <= PLAYER_NEAR && (at[2] - here[2]).abs() <= PLAYER_NEAR;
    let players: Vec<u128> = host.server_players().into_iter()
        .filter(|player| player.dimension == OVERWORLD && near(&player.position))
        .map(|player| player.uuid)
        .collect();
    if players.is_empty() {
        return;
    }
    let population = production::ranch::villager_positions(host, OVERWORLD).iter()
        .filter(|(_, at)| (at[0] - here[0]).abs() <= RADIUS && (at[2] - here[2]).abs() <= RADIUS)
        .count();
    let (peak, since) = with_state(|state| {
        if population > state.peak {
            state.peak = population;
            state.dirty = true;
        }
        state.population = Some(population);
        let low = state.peak >= RECOVERY_MIN_PEAK && population * RECOVERY_SHARE < state.peak;
        state.recovering = low || population < FLOOR;
        (state.peak, state.last_family.map(|last| now.saturating_sub(last)))
    });
    let daytime = host.day_time().rem_euclid(24000) < 12000;
    let beds = village::homes(OVERWORLD, center);
    let cap = village::cap(OVERWORLD, center, now);
    let gate = offset(center, data::GATE);
    let gate = [f64::from(gate[0]) + 0.5, f64::from(gate[1]), f64::from(gate[2]) + 0.5];
    match arrival(population, peak, beds, cap, since, daytime) {
        Arrival::None => {}
        Arrival::Couple => {
            // O piso: dois adultos chegam pelo portão. Se o casal fundador se perdeu, eles herdam.
            for index in 0..2 {
                let at = [gate[0] + index as f64, gate[1], gate[2]];
                if founders::count() < 2 {
                    founders::summon(host, OVERWORLD, at, index, now);
                } else {
                    summon_villager(host, at, false);
                }
            }
            welcome(host, &players, gate, "veio.town.refugees");
        }
        Arrival::Family => {
            for index in 0..3 {
                let at = [gate[0] + index as f64 - 1.0, gate[1], gate[2]];
                if index < 2 && founders::count() < 2 {
                    founders::summon(host, OVERWORLD, at, index, now);
                } else {
                    summon_villager(host, at, index == 2);
                }
            }
            with_state(|state| state.last_family = Some(now));
            welcome(host, &players, gate, "veio.town.immigrants");
        }
    }
}

fn welcome(host: &mut dyn Host, players: &[u128], at: [f64; 3], key: &str) {
    host.send_particles(&ParticleOptions::simple(ParticleType::HappyVillager), None,
        [at[0], at[1] + 1.0, at[2]], [1.0, 0.6, 1.0], 0.0, 16);
    for player in players {
        host.tell(*player, Text::tr(key, &[]));
    }
}

// ---------- Arquivo do mundo ----------

fn file(world: &Path) -> PathBuf {
    world.join("veio").join("town.txt")
}

pub(crate) fn encode(center: [i32; 3], peak: usize) -> String {
    format!("center {} {} {}\npeak {peak}\n", center[0], center[1], center[2])
}

pub(crate) fn decode(text: &str) -> Option<([i32; 3], usize)> {
    let mut center = None;
    let mut peak = 0;
    for line in text.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        match parts.as_slice() {
            ["center", x, y, z] => center = Some([x.parse().ok()?, y.parse().ok()?, z.parse().ok()?]),
            ["peak", n] => peak = n.parse().ok()?,
            _ => {}
        }
    }
    center.map(|center| (center, peak))
}

fn load(host: &dyn Host) {
    let world = host.world_dir();
    let found = with_state(|state| {
        if world == state.world {
            return false;
        }
        *state = State { world: world.clone(), ..State::default() };
        let Some((center, peak)) = world.as_deref().and_then(|dir| std::fs::read_to_string(file(dir)).ok())
            .and_then(|text| decode(&text)) else { return false };
        state.center = Some(center);
        state.peak = peak;
        true
    });
    if found {
        founders::start_scan(defense::clock());
    }
}

fn save_now(host: &dyn Host) {
    let world = host.world_dir();
    with_state(|state| {
        if state.world.is_none() {
            state.world = world.clone();
        }
        let (Some(dir), Some(center)) = (world, state.center) else { return };
        let path = file(&dir);
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        if std::fs::write(&path, encode(center, state.peak)).is_ok() {
            state.dirty = false;
        }
    });
}

#[cfg(test)]
pub(crate) fn reset() {
    with_state(|state| *state = State::default());
}

#[cfg(test)]
pub(crate) fn peak() -> usize {
    with_state(|state| state.peak)
}

#[cfg(test)]
mod tests;
