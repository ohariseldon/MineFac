//! Casa do fundador: a missão de controle e a casa do jogador num lugar só.
//!
//! - **Construção:** uma vez por mundo, assim que se sabe onde fica o centro da vila (ou depois de
//!   `BUILD_WAIT` ticks), o Veio procura um terreno plano ao norte do centro e roda a função
//!   `veio:home/build` (gerada por `tools/build_house.py`). A marca `veio:home built` no
//!   armazenamento de comandos do mundo impede uma segunda casa. A casa já vem com o centro de
//!   controle (o painel), o Lar das obras no lugar da lareira (`workforce::home`), sofá, cama ao
//!   lado do painel e o mural de mapas.
//! - **Em casa:** quem entra (perto do painel, debaixo do telhado) recebe um aviso. A refeição e
//!   os bônus de casa são do Lar; a casa em si não acelera as máquinas.
//! - **Duas camas:** a primeira cama de cada jogador é uma da casa; depois ele pode marcar só mais
//!   uma fora. Para trocar a de fora, quebra a antiga.
//! - **Comida:** `food` (refeições fora de casa dão bônus ao jogador e às máquinas).
//! - **Mural:** `mural` (mapa das redondezas já vistas nas molduras da parede).
//!
//! O painel da casa guarda `Home = 1` e as camas (`HomeBeds`) no próprio NBT, então nada disso
//! se perde ao reabrir o mundo. Os valores lidos a cada tick de máquina são atômicos.
use super::*;
use super::workforce::spawn::{self, OVERWORLD};
use horizon_client::mod_api::{NativeHeightmap, Text};
use std::sync::Mutex;
use std::sync::atomic::{AtomicU64, Ordering};

pub(crate) mod food;
pub(crate) mod mural;

pub(crate) const FUNCTION: &str = "veio:home/build";
/// Planta da casa (ver `tools/build_house.py`): colunas em x e z a partir do canto noroeste.
pub(crate) const FOOTPRINT: [i32; 2] = [13, 11];
/// O painel e a cabeceira da cama da casa, relativos ao canto noroeste no nível do piso.
pub(crate) const DASHBOARD: [i32; 3] = [10, 1, 5];
#[cfg(test)]
pub(crate) const HOUSE_BED: [i32; 3] = [10, 1, 8];
/// Dentro de casa: até `REACH` blocos do painel na horizontal e `REACH_Y` na vertical.
pub(crate) const REACH: i32 = 9;
pub(crate) const REACH_Y: i32 = 4;
/// Quem está em casa conta até `WINDOW` ticks depois da última verificação.
const WINDOW: u64 = 60;
const CHECK_EVERY: i64 = 20;
/// Sem centro da vila conhecido, a casa sai perto do primeiro jogador depois disto.
const BUILD_WAIT: u64 = 2 * spawn::WAIT + 200;
const RETRY: u64 = 200;
const ATTEMPTS: u32 = 5;
pub(crate) const GUARD: &str = "execute unless data storage veio:home built";
const MARK: &str = "data modify storage veio:home built set value 1b";

/// Camas de um jogador: a da casa e a de fora (dimensão e posição).
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct Beds {
    pub player: u128,
    pub home: Option<[i32; 3]>,
    pub away: Option<(String, [i32; 3])>,
}

#[derive(Default)]
struct State {
    /// Painel da casa (Overworld).
    dashboard: Option<[i32; 3]>,
    /// Painel que a função acabou de colocar e que ainda não ticou.
    pending: Option<[i32; 3]>,
    beds: Vec<Beds>,
    beds_loaded: bool,
    beds_dirty: bool,
    inside: Vec<u128>,
    built: bool,
    attempts: u32,
    first_seen: Option<u64>,
    next_try: u64,
    /// Mensagens e comandos de ganchos onde o jogador não pode ser alcançado; saem no tick.
    messages: Vec<(u128, &'static str, Vec<String>)>,
    commands: Vec<String>,
}

static STATE: Mutex<State> = Mutex::new(State {
    dashboard: None, pending: None, beds: Vec::new(), beds_loaded: false, beds_dirty: false,
    inside: Vec::new(), built: false, attempts: 0, first_seen: None, next_try: 0,
    messages: Vec::new(), commands: Vec::new(),
});
static HOME_UNTIL: AtomicU64 = AtomicU64::new(0);

fn state() -> std::sync::MutexGuard<'static, State> {
    STATE.lock().unwrap_or_else(|err| err.into_inner())
}

pub(crate) fn tell(player: u128, key: &'static str, args: Vec<String>) {
    state().messages.push((player, key, args));
}

pub(crate) fn command(line: String) {
    state().commands.push(line);
}

/// Algo vale até `until`, mas só se `until` estiver no máximo `span` ticks à frente: um relógio
/// reiniciado (mundo reaberto) não herda bônus de outra sessão.
pub(crate) fn active(until: u64, now: u64, span: u64) -> bool {
    until > now && until - now <= span
}

/// Multiplicador da duração do ciclo das máquinas: a comida de fora de casa. Lido a cada ciclo,
/// sem trava. Em casa, quem dá refeição e bônus é o Lar (`workforce::home`), que vem na casa.
pub(crate) fn time_factor(now: u64) -> f32 {
    food::time_factor(now)
}

/// Alguém do lado de dentro da casa do fundador há pouco.
#[cfg(test)]
pub(crate) fn someone_home(now: u64) -> bool {
    active(HOME_UNTIL.load(Ordering::Relaxed), now, WINDOW)
}

pub(crate) fn dashboard() -> Option<[i32; 3]> {
    state().dashboard
}

fn near(dashboard: [i32; 3], at: [i32; 3]) -> bool {
    (at[0] - dashboard[0]).abs() <= REACH && (at[2] - dashboard[2]).abs() <= REACH && (at[1] - dashboard[1]).abs() <= REACH_Y
}

/// Uma posição do Overworld dentro da área da casa (sem conferir o telhado).
pub(crate) fn in_house(dimension: &str, at: [f64; 3]) -> bool {
    dimension == OVERWORLD && dashboard().is_some_and(|home| near(home, block_of(at)))
}

fn block_of(at: [f64; 3]) -> [i32; 3] {
    [at[0].floor() as i32, at[1].floor() as i32, at[2].floor() as i32]
}

// ---------- Construção ----------

/// Canto noroeste da casa perto de `around`: o terreno mais plano, sem água nem árvores, ao norte do
/// centro (a porta, no sul, fica de frente para a vila). `y` é o nível do piso.
pub(crate) fn site(host: &dyn Host, around: [i32; 3]) -> [i32; 3] {
    let [cx, cy, cz] = around;
    let fallback = [cx - FOOTPRINT[0] / 2, cy - 1, cz - 23];
    let candidates: Vec<[i32; 2]> = [-18, -24, -30].iter()
        .flat_map(|dz| [0, -6, 6, -12, 12].map(|dx| [cx + dx - FOOTPRINT[0] / 2, cz + dz - FOOTPRINT[1] / 2]))
        .collect();
    let samples: Vec<(i32, i32)> = candidates.iter()
        .flat_map(|[x, z]| (1..=11).step_by(2).flat_map(move |dx| (1..=9).step_by(2).map(move |dz| (x + dx, z + dz))))
        .collect();
    let per = samples.len() / candidates.len();
    let (Ok(top), Ok(ground)) = (
        host.surface_heights(OVERWORLD, &samples, NativeHeightmap::MotionBlocking),
        host.surface_heights(OVERWORLD, &samples, NativeHeightmap::OceanFloor),
    ) else {
        return fallback;
    };
    let mut best: Option<(i32, [i32; 3])> = None;
    for (index, [x, z]) in candidates.iter().enumerate() {
        let range = index * per..(index + 1) * per;
        let Some(mut heights) = ground[range.clone()].iter().copied().collect::<Option<Vec<i32>>>() else { continue };
        // Água ou folhas por cima do chão: lago, rio ou árvore.
        let covered = top[range].iter().zip(&heights).filter(|(top, ground)| top.is_none_or(|top| top != **ground)).count();
        heights.sort_unstable();
        let spread = heights[heights.len() - 1] - heights[0];
        let score = spread * 4 + covered as i32 * 6 + index as i32 / 5;
        let floor = heights[heights.len() / 2] - 1;
        if best.is_none_or(|(known, _)| score < known) {
            best = Some((score, [*x, floor, *z]));
        }
    }
    best.map_or(fallback, |(_, at)| at)
}

/// Chamado no tick global: entrega mensagens e, uma vez por mundo, constrói a casa.
pub(crate) fn tick(host: &mut dyn Host, now: u64) {
    let (messages, commands) = {
        let mut state = state();
        (std::mem::take(&mut state.messages), std::mem::take(&mut state.commands))
    };
    for (player, key, args) in messages {
        let args: Vec<&dyn std::fmt::Display> = args.iter().map(|arg| arg as &dyn std::fmt::Display).collect();
        host.tell(player, Text::tr(key, &args));
    }
    for line in commands {
        host.run_command(&line);
    }
    if now.is_multiple_of(CHECK_EVERY as u64) {
        build(host, now);
    }
}

fn build(host: &mut dyn Host, now: u64) {
    let mut state = state();
    if state.built || now < state.next_try {
        return;
    }
    let players = host.server_players();
    let Some(first) = players.iter().find(|player| player.dimension == OVERWORLD) else { return };
    let since = *state.first_seen.get_or_insert(now);
    let center = spawn::center().map(|(at, _)| at);
    if center.is_none() && now < since + BUILD_WAIT {
        return;
    }
    if !host.run_command(GUARD) {
        // Este mundo já tem a casa.
        state.built = true;
        return;
    }
    let around = center.unwrap_or_else(|| block_of(first.position));
    let [x, y, z] = site(host, around);
    if host.run_command(&format!("execute in {OVERWORLD} positioned {x} {y} {z} run function {FUNCTION}")) {
        host.run_command(MARK);
        state.built = true;
        let dashboard = [x + DASHBOARD[0], y + DASHBOARD[1], z + DASHBOARD[2]];
        state.pending = Some(dashboard);
        let at = format!("{} {} {}", dashboard[0], dashboard[1], dashboard[2]);
        for player in players.iter().filter(|player| player.dimension == OVERWORLD) {
            state.messages.push((player.uuid, "veio.home.built", vec![at.clone()]));
        }
    } else {
        // Chunks ainda não carregados ou comando recusado: tenta de novo algumas vezes.
        state.attempts += 1;
        state.built = state.attempts >= ATTEMPTS;
        state.next_try = now + RETRY;
    }
}

// ---------- O painel da casa ----------

/// Chamado no tick de todo centro de controle. Só o da casa faz alguma coisa.
pub(crate) fn center_tick(host: &mut dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity, time: i64, now: u64) {
    if host.dimension() != OVERWORLD || !adopt(host, pos, entity) {
        return;
    }
    if time.rem_euclid(CHECK_EVERY) == 0 {
        presence(host, pos, now);
    }
    mural::step(host, pos, time);
}

/// Se este painel é o da casa: registra a posição, carrega e grava as camas.
fn adopt(host: &dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity) -> bool {
    let mut state = state();
    if number(entity, "Home") != 1 {
        let placed = state.pending == Some(pos);
        // O painel da casa foi quebrado e posto de volta perto do lugar antigo.
        let replaced = state.dashboard.is_some_and(|old| old != pos && near(old, pos) && (old[1] - pos[1]).abs() <= 1
            && host.block(old) != workforce::CONTROL_CENTER);
        if !placed && !replaced {
            return false;
        }
        set_number(entity, "Home", 1);
        state.pending = None;
        state.beds_dirty = true;
    }
    if state.dashboard != Some(pos) {
        state.dashboard = Some(pos);
    }
    if !state.beds_loaded {
        let saved = with_nbt_key("HomeBeds", |key| match entity.nbt.get(key) {
            Some(Tag::String(text)) => decode(&text.to_string_lossy()),
            _ => Vec::new(),
        });
        for beds in saved {
            if !state.beds.iter().any(|known| known.player == beds.player) {
                state.beds.push(beds);
            }
        }
        state.beds_loaded = true;
    }
    if state.beds_dirty {
        let text = encode(&state.beds);
        with_nbt_key("HomeBeds", |key| entity.nbt.insert(key.clone(), Tag::String(text.as_str().into())));
        state.beds_dirty = false;
    }
    true
}

/// Quem está em casa: perto do painel e com telhado acima da cabeça.
fn presence(host: &mut dyn Host, pos: [i32; 3], now: u64) {
    let players: Vec<_> = host.server_players().into_iter()
        .filter(|player| player.dimension == OVERWORLD && near(pos, block_of(player.position)))
        .collect();
    let columns: Vec<(i32, i32)> = players.iter().map(|player| {
        let [x, _, z] = block_of(player.position);
        (x, z)
    }).collect();
    let heights = if columns.is_empty() { Vec::new() } else {
        host.surface_heights(OVERWORLD, &columns, NativeHeightmap::MotionBlocking).unwrap_or_default()
    };
    let inside: Vec<u128> = players.iter().enumerate()
        .filter(|(index, player)| heights.get(*index).copied().flatten().is_some_and(|top| top > block_of(player.position)[1] + 1))
        .map(|(_, player)| player.uuid)
        .collect();
    if !inside.is_empty() {
        HOME_UNTIL.store(now + WINDOW, Ordering::Relaxed);
    }
    let before = std::mem::replace(&mut state().inside, inside.clone());
    for player in inside.iter().filter(|player| !before.contains(player)) {
        host.action_bar(*player, Text::tr("veio.home.inside", &[]));
    }
    for player in before.iter().filter(|player| !inside.contains(player)) {
        host.action_bar(*player, Text::tr("veio.home.left", &[]));
    }
}

// ---------- Camas ----------

fn same_bed(a: [i32; 3], b: [i32; 3]) -> bool {
    (0..3).all(|axis| (a[axis] - b[axis]).abs() <= 1)
}

/// Pode usar a cama em `pos`? `register` grava a escolha (ao mudar o ponto de renascimento).
/// `Err` traz a chave da mensagem. Sem casa conhecida, não há regra.
pub(crate) fn bed_allowed(host: &dyn Host, player: u128, dimension: &str, pos: [i32; 3], register: bool) -> Result<(), &'static str> {
    let mut state = state();
    let Some(home) = state.dashboard else { return Ok(()) };
    let index = match state.beds.iter().position(|beds| beds.player == player) {
        Some(index) => index,
        None => {
            state.beds.push(Beds { player, ..Beds::default() });
            state.beds.len() - 1
        }
    };
    let beds = state.beds[index].clone();
    let changed = if dimension == OVERWORLD && near(home, pos) {
        let changed = beds.home.is_none_or(|known| !same_bed(known, pos));
        if register && changed {
            state.beds[index].home = Some(pos);
        }
        changed
    } else {
        if beds.home.is_none() {
            return Err("veio.home.bed_first");
        }
        if let Some((known_dimension, known)) = &beds.away {
            let same = known_dimension == dimension && same_bed(*known, pos);
            // A cama de fora ainda existe? Só dá para ver na dimensão que está ticando agora.
            let standing = known_dimension != host.dimension() || host.block(*known).ends_with("_bed");
            if !same && standing {
                return Err("veio.home.bed_limit");
            }
            if same {
                return Ok(());
            }
        }
        if register {
            state.beds[index].away = Some((dimension.to_owned(), pos));
        }
        true
    };
    if register && changed {
        state.beds_dirty = true;
    }
    Ok(())
}

pub(crate) fn has_bed(player: u128) -> bool {
    state().beds.iter().any(|beds| beds.player == player && (beds.home.is_some() || beds.away.is_some()))
}

/// Ponto de renascimento mudando: a regra das duas camas vale para camas.
pub(crate) fn spawn_point(host: &dyn Host, player: u128, dimension: &str, pos: [i32; 3], cause: &str) -> Flow {
    if cause != "bed" {
        return Flow::Continue;
    }
    let home_before = state().beds.iter().find(|beds| beds.player == player).and_then(|beds| beds.home);
    match bed_allowed(host, player, dimension, pos, true) {
        Ok(()) => {
            let beds = state().beds.iter().find(|beds| beds.player == player).cloned().unwrap_or_default();
            if beds.home.is_some_and(|home| same_bed(home, pos)) && home_before.is_none_or(|old| !same_bed(old, pos)) {
                tell(player, "veio.home.bed_home", Vec::new());
            } else if beds.away.as_ref().is_some_and(|(_, away)| same_bed(*away, pos)) {
                tell(player, "veio.home.bed_away", Vec::new());
            }
            Flow::Continue
        }
        Err(key) => {
            tell(player, key, Vec::new());
            Flow::Cancel
        }
    }
}

/// Deitar: só nas duas camas permitidas.
pub(crate) fn bed_enter(host: &dyn Host, player: u128, pos: [i32; 3]) -> Flow {
    let dimension = host.dimension();
    match bed_allowed(host, player, dimension, pos, false) {
        Ok(()) => Flow::Continue,
        Err(key) => {
            tell(player, key, Vec::new());
            Flow::Cancel
        }
    }
}

fn encode(beds: &[Beds]) -> String {
    let pos = |[x, y, z]: [i32; 3]| format!("{x},{y},{z}");
    beds.iter().map(|beds| {
        let home = beds.home.map_or_else(|| "-".to_owned(), pos);
        let away = beds.away.as_ref().map_or_else(|| "-".to_owned(), |(dimension, at)| format!("{dimension}@{}", pos(*at)));
        format!("{:032x}|{home}|{away}", beds.player)
    }).collect::<Vec<_>>().join(";")
}

fn decode(text: &str) -> Vec<Beds> {
    let pos = |text: &str| -> Option<[i32; 3]> {
        let parts: Vec<i32> = text.split(',').map(str::parse).collect::<Result<_, _>>().ok()?;
        <[i32; 3]>::try_from(parts).ok()
    };
    text.split(';').filter_map(|entry| {
        let mut fields = entry.split('|');
        let player = u128::from_str_radix(fields.next()?, 16).ok()?;
        let home = pos(fields.next()?);
        let away = fields.next()?.split_once('@').and_then(|(dimension, at)| Some((dimension.to_owned(), pos(at)?)));
        Some(Beds { player, home, away })
    }).collect()
}

pub(crate) fn reset() {
    *state() = State::default();
    HOME_UNTIL.store(0, Ordering::Relaxed);
    food::reset();
    mural::reset();
}

#[cfg(test)]
pub(crate) fn beds_round_trip(beds: &[Beds]) -> Vec<Beds> {
    decode(&encode(beds))
}

#[cfg(test)]
mod tests;
