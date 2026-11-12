//! A vila do Veio: o construtor que levanta, no mundo, a vila desenhada em `tools/vila`.
//!
//! - **O plano** (`town_plan.rs`, gerado por `tools/vila/build_village.py`): as peças (casas,
//!   moradores, praça, roça, pasto, prefeitura), os pedaços de rua e os lugares de cada morador
//!   com nome, tudo relativo ao centro da vila (a fonte). Cada peça é uma função do datapack
//!   (`veio:village/<peça>`) que limpa o volume, faz a fundação e põe os blocos e os aldeões.
//! - **Quando:** `found` marca o centro num mundo novo (no lugar de `place structure` de uma vila
//!   do Minecraft). A cada meio segundo, `tick` ergue até `PER_TICK` peças do estágio atual cujo
//!   chão já carregou, depois as ruas. Peça em chunk não carregado espera o jogador chegar perto.
//! - **O piso de cada peça** é a mediana do chão sob as paredes, menos um (o piso entra no lugar da
//!   grama). As trocas de estágio no mesmo lugar (`site`) usam o mesmo piso.
//! - **O acabamento** melhora com a vila: no estágio 1 as casas têm desgaste (uma vidraça
//!   quebrada, teia no beiral, mato junto da parede); o acabamento do estágio 2 conserta e põe
//!   lanterna, jardineira e flores; o do 4, venezianas, banco e arbustos floridos.
//! - **Os estágios** (`advance`) sobem com a história: 2 com os pedidos da vila atendidos, 3 com o
//!   capítulo da vila, 4 com o da cidade, 6 com o da metrópole. Uma troca nunca acontece com
//!   alguém a menos de `UNSEEN` blocos, nem uma peça nasce em cima de um jogador.
//! - **Salvo** em `<mundo>/veio/vila.txt`: o centro, o estágio, as peças e ruas já feitas e o piso de
//!   cada lugar.
use super::*;
use super::town_plan::{FOUNDER, PIECES, PLOT, ROADS};
pub(crate) use super::town_plan::{COLLIER, COWHERD, FARMER, GROCER, MAYOR, POTTER, SMITH};
use horizon_client::mod_api::NativeHeightmap;
use std::path::{Path, PathBuf};

pub(crate) struct Piece {
    pub key: &'static str,
    /// Peças no mesmo lugar (a troca de um estágio para outro) dividem o piso.
    pub site: &'static str,
    /// Onde a função roda (o zero da peça), relativo ao centro.
    pub at: [i32; 2],
    /// As paredes (oeste, norte, leste, sul), relativas ao centro: onde se mede o chão.
    pub core: [i32; 4],
    /// Tudo o que a peça ocupa, com beiral e quintal.
    pub area: [i32; 4],
    pub stage: u8,
    pub until: u8,
    /// Acabamento: só acrescenta blocos ao prédio do mesmo lugar (lanterna, flores, venezianas),
    /// no piso dele. Espera o prédio estar de pé.
    pub overlay: bool,
}

pub(crate) struct Road {
    pub name: &'static str,
    pub stage: u8,
    pub until: u8,
    pub area: [i32; 4],
}

const OVERWORLD: &str = spawn::OVERWORLD;
const EVERY: u64 = 10;
/// Peças por vez (cada uma são centenas de blocos).
const PER_TICK: usize = 3;
const ROADS_PER_TICK: usize = 6;
/// Uma troca de estágio não acontece com alguém a menos disto.
const UNSEEN: f64 = 32.0;
/// Nem nasce uma peça com alguém dentro dela ou a menos disto da borda.
const KEEP_OUT: i32 = 2;
const FILE: &str = "vila.txt";
pub(crate) const MAX_STAGE: u8 = 6;

#[derive(Default)]
struct State {
    world: Option<PathBuf>,
    synced: bool,
    center: Option<[i32; 3]>,
    stage: u8,
    /// Peças feitas e o piso de cada lugar.
    built: Vec<&'static str>,
    floors: Vec<(String, i32)>,
    roads: Vec<&'static str>,
    dirty: bool,
}

static STATE: Mutex<Option<State>> = Mutex::new(None);

fn with<R>(f: impl FnOnce(&mut State) -> R) -> R {
    let mut guard = STATE.lock().unwrap_or_else(|err| err.into_inner());
    f(guard.get_or_insert_with(State::default))
}

// ---------- Salvar ----------

pub(crate) fn save_text(center: [i32; 3], stage: u8, built: &[&str], floors: &[(String, i32)], roads: &[&str]) -> String {
    let mut out = format!("centro {} {} {}\nestagio {stage}\n", center[0], center[1], center[2]);
    for (site, y) in floors {
        out.push_str(&format!("piso {site} {y}\n"));
    }
    for key in built {
        out.push_str(&format!("peca {key}\n"));
    }
    for name in roads {
        out.push_str(&format!("rua {name}\n"));
    }
    out
}

/// Lê o arquivo salvo. Peças e ruas que não estão mais no plano são esquecidas.
pub(crate) fn load_text(text: &str) -> Option<([i32; 3], u8, Vec<&'static str>, Vec<(String, i32)>, Vec<&'static str>)> {
    let (mut center, mut stage, mut built, mut floors, mut roads) = (None, 1u8, Vec::new(), Vec::new(), Vec::new());
    for line in text.lines() {
        let mut parts = line.split_whitespace();
        match parts.next() {
            Some("centro") => {
                let mut n = || parts.next()?.parse::<i32>().ok();
                if let (Some(x), Some(y), Some(z)) = (n(), n(), n()) { center = Some([x, y, z]); }
            }
            Some("estagio") => stage = parts.next().and_then(|s| s.parse().ok()).unwrap_or(1).clamp(1, MAX_STAGE),
            Some("piso") => {
                if let (Some(site), Some(y)) = (parts.next(), parts.next().and_then(|y| y.parse().ok())) {
                    floors.push((site.to_owned(), y));
                }
            }
            Some("peca") => {
                if let Some(piece) = parts.next().and_then(|key| PIECES.iter().find(|p| p.key == key)) { built.push(piece.key); }
            }
            Some("rua") => {
                if let Some(road) = parts.next().and_then(|name| ROADS.iter().find(|r| r.name == name)) { roads.push(road.name); }
            }
            _ => {}
        }
    }
    center.map(|c| (c, stage, built, floors, roads))
}

fn file(world: &Path) -> PathBuf {
    world.join("veio").join(FILE)
}

fn persist(state: &mut State) {
    let (Some(world), Some(center)) = (&state.world, state.center) else { return };
    let path = file(world);
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let temp = path.with_extension("tmp");
    let text = save_text(center, state.stage, &state.built, &state.floors, &state.roads);
    if std::fs::write(&temp, text).is_ok() && std::fs::rename(&temp, &path).is_ok() {
        state.dirty = false;
    }
}

/// Troca de mundo: lê o arquivo do mundo novo (ou começa vazio).
fn sync_world(host: &dyn Host, state: &mut State) {
    let world = host.world_dir();
    if state.synced && world == state.world {
        return;
    }
    if state.dirty { persist(state); }
    *state = State { world: world.clone(), synced: true, stage: 1, ..State::default() };
    if let Some((center, stage, built, floors, roads)) = world.as_deref()
        .and_then(|dir| std::fs::read_to_string(file(dir)).ok())
        .and_then(|text| load_text(&text))
    {
        state.center = Some(center);
        state.stage = stage;
        state.built = built;
        state.floors = floors;
        state.roads = roads;
    }
}

// ---------- Consultas ----------

/// O centro da vila do Veio neste mundo, se ela existe (lido do disco no `tick`).
pub(crate) fn center(_host: &dyn Host) -> Option<[i32; 3]> {
    known_center()
}

pub(crate) fn known_center() -> Option<[i32; 3]> {
    with(|state| state.center)
}

/// Uma coluna (x, z) cai dentro de alguma peça da vila (de qualquer estágio)?
pub(crate) fn occupied(center: [i32; 3], x: i32, z: i32) -> bool {
    let (dx, dz) = (x - center[0], z - center[2]);
    PIECES.iter().any(|p| dx >= p.area[0] - KEEP_OUT && dx <= p.area[2] + KEEP_OUT && dz >= p.area[1] - KEEP_OUT && dz <= p.area[3] + KEEP_OUT)
}

/// Canto noroeste da casa do fundador no plano, com o piso medido no chão; `None` sem vila do Veio
/// ou com o chão ainda não carregado.
pub(crate) fn founder_site(host: &dyn Host) -> Option<[i32; 3]> {
    let center = center(host)?;
    let [w, d] = super::super::home::FOOTPRINT;
    let (x, z) = (center[0] + FOUNDER[0], center[2] + FOUNDER[1]);
    let y = floor(host, [x + 1, z + 1, x + w - 2, z + d - 2])?;
    Some([x, y, z])
}

/// O centro do terreno da oficina no plano.
pub(crate) fn plot(host: &dyn Host) -> Option<[i32; 2]> {
    center(host).map(|c| [c[0] + PLOT[0], c[2] + PLOT[1]])
}

/// Os lugares de um morador com nome (relativos ao centro) viram colunas do mundo.
pub(crate) fn spots(host: &dyn Host, offsets: &[(i32, i32); 4]) -> Option<Vec<(i32, i32)>> {
    center(host).map(|c| offsets.iter().map(|(dx, dz)| (c[0] + dx, c[2] + dz)).collect())
}

/// O piso de uma área (oeste, norte, leste, sul em coordenadas do mundo): a mediana do chão, menos
/// um. `None` se alguma coluna não carregou.
fn floor(host: &dyn Host, [x0, z0, x1, z1]: [i32; 4]) -> Option<i32> {
    let columns: Vec<(i32, i32)> = (x0..=x1).step_by(2).flat_map(|x| (z0..=z1).step_by(2).map(move |z| (x, z))).collect();
    let mut heights: Vec<i32> = host.surface_heights(OVERWORLD, &columns, NativeHeightmap::MotionBlockingNoLeaves).ok()?
        .into_iter().collect::<Option<Vec<i32>>>()?;
    if heights.is_empty() { return None; }
    heights.sort_unstable();
    Some(heights[heights.len() / 2] - 1)
}

// ---------- Mudanças ----------

/// A vila do Veio nasce aqui (o centro é a fonte da praça, `y` o chão). Só uma vez por mundo.
pub(crate) fn found(host: &dyn Host, at: [i32; 3]) -> [i32; 3] {
    with(|state| {
        sync_world(host, state);
        if let Some(known) = state.center { return known; }
        state.center = Some(at);
        state.stage = 1;
        state.dirty = true;
        persist(state);
        at
    })
}

/// A história chegou a um estágio (nunca volta).
pub(crate) fn advance(stage: u8) {
    with(|state| {
        let stage = stage.min(MAX_STAGE);
        if state.center.is_some() && stage > state.stage {
            state.stage = stage;
            state.dirty = true;
        }
    });
}

/// O que deve estar de pé no estágio.
pub(crate) fn wanted(stage: u8) -> impl Iterator<Item = &'static Piece> {
    PIECES.iter().filter(move |p| p.stage <= stage && stage < p.until)
}

fn wanted_roads(stage: u8) -> impl Iterator<Item = &'static Road> {
    ROADS.iter().filter(move |r| r.stage <= stage && stage < r.until)
}

/// Chamado no tick do servidor.
pub(crate) fn tick(host: &mut dyn Host, now: u64) {
    if !now.is_multiple_of(EVERY) {
        return;
    }
    let (center, stage, built, roads_done, floors) = {
        let snapshot = with(|state| {
            sync_world(&*host, state);
            state.center.map(|c| (c, state.stage.max(1), state.built.clone(), state.roads.clone(), state.floors.clone()))
        });
        let Some(snapshot) = snapshot else { return };
        snapshot
    };
    if spawn::center().is_none() {
        spawn::adopt(center);
    }
    let players: Vec<[f64; 3]> = host.server_players().iter().filter(|p| p.dimension == OVERWORLD).map(|p| p.position).collect();
    if players.is_empty() {
        return;
    }
    let [cx, cy, cz] = center;
    let mut made = Vec::new();
    let mut new_floors = Vec::new();
    for piece in wanted(stage).filter(|p| !built.contains(&p.key)) {
        if made.len() >= PER_TICK { break; }
        let [x0, z0, x1, z1] = [cx + piece.area[0], cz + piece.area[1], cx + piece.area[2], cz + piece.area[3]];
        let inside = players.iter().any(|at| {
            let (px, pz) = (at[0].floor() as i32, at[2].floor() as i32);
            px >= x0 - KEEP_OUT && px <= x1 + KEEP_OUT && pz >= z0 - KEEP_OUT && pz <= z1 + KEEP_OUT
        });
        // Troca de estágio (o lugar já tinha outra peça): só longe dos olhos.
        let replaces = PIECES.iter().any(|old| old.site == piece.site && old.key != piece.key && built.contains(&old.key));
        let seen = replaces && players.iter().any(|at| {
            let (mx, mz) = (f64::from(x0 + x1) / 2.0, f64::from(z0 + z1) / 2.0);
            (at[0] - mx).hypot(at[2] - mz) < UNSEEN + f64::from((x1 - x0).max(z1 - z0)) / 2.0
        });
        if inside || seen { continue; }
        let known = floors.iter().chain(new_floors.iter()).find(|(site, _)| site == piece.site).map(|(_, y)| *y);
        let base_up = PIECES.iter().any(|p| p.site == piece.site && !p.overlay && (built.contains(&p.key) || made.contains(&p.key)));
        if piece.overlay && (known.is_none() || !base_up) { continue; }
        let core = [cx + piece.core[0], cz + piece.core[1], cx + piece.core[2], cz + piece.core[3]];
        let Some(y) = known.or_else(|| floor(&*host, core)) else { continue };
        let (x, z) = (cx + piece.at[0], cz + piece.at[1]);
        if host.run_command(&format!("execute in {OVERWORLD} positioned {x} {y} {z} run function veio:village/{}", piece.key)) {
            made.push(piece.key);
            if known.is_none() { new_floors.push((piece.site.to_owned(), y)); }
        }
    }
    // As ruas só depois que as peças do estágio estão de pé (o caminho chega à porta pronta).
    let mut paved = Vec::new();
    let all_built = wanted(stage).all(|p| built.contains(&p.key) || made.contains(&p.key));
    if all_built {
        for road in wanted_roads(stage).filter(|r| !roads_done.contains(&r.name)) {
            if paved.len() >= ROADS_PER_TICK { break; }
            let [x0, z0, x1, z1] = [cx + road.area[0], cz + road.area[1], cx + road.area[2], cz + road.area[3]];
            let line = format!("execute in {OVERWORLD} if loaded {x0} {cy} {z0} if loaded {x1} {cy} {z1} if loaded {x0} {cy} {z1} \
                if loaded {x1} {cy} {z0} positioned {cx} {cy} {cz} run function veio:village/{}", road.name);
            if host.run_command(&line) {
                paved.push(road.name);
            }
        }
    }
    if made.is_empty() && paved.is_empty() {
        with(|state| if state.dirty { persist(state) });
        return;
    }
    with(|state| {
        state.built.extend(made);
        state.roads.extend(paved);
        state.floors.extend(new_floors);
        state.dirty = true;
        persist(state);
    });
}

#[cfg(test)]
pub(crate) fn reset() {
    if let Ok(mut state) = STATE.lock() {
        *state = None;
    }
}
