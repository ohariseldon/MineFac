//! Spawn no centro da vila.
//!
//! - **Mundo novo / jogador novo:** no login, o Veio procura uma vila a até `SEARCH` blocos do
//!   jogador pelos aldeões que já registra (`ranch::villager_positions`); o centro é a média dos
//!   aldeões em volta daquele que tem mais vizinhos (onde a vila se reúne). O jogador vai para lá,
//!   e o spawn do mundo e o dele ficam nesse ponto. Sem vila em `WAIT` ticks, o Veio escolhe um
//!   terreno plano, seco e sem árvores a até `SITE_REACH` blocos (`flat_site`), tira as árvores
//!   que sobrarem em volta, gera uma vila de planície ali (`/place structure`) e leva o jogador.
//! - **Centro de controle:** com administrador, ele vira o centro da vila: o spawn do mundo e o de
//!   todos os jogadores passam para cima dele, e quem renasce volta para lá.
//!
//! Tudo por `Host::run_command` (console do servidor). Cada jogador é marcado com a tag
//! `veio_village` depois da primeira vez; os comandos de chegada só valem para quem não tem a tag,
//! então voltar ao mundo não muda o spawn de ninguém. Só no Overworld.
use super::*;
use horizon_client::mod_api::NativeHeightmap;
use std::sync::atomic::{AtomicBool, Ordering};

pub(crate) const OVERWORLD: &str = "minecraft:overworld";
pub(crate) const SEARCH: f64 = 96.0;
/// Espera pelos aldeões (os chunks em volta precisam carregar) antes de gerar uma vila.
pub(crate) const WAIT: u64 = 600;
pub(crate) const MIN_VILLAGERS: usize = 3;
const NEIGHBORS: f64 = 16.0;
/// Até onde procurar terreno para a vila nova (dentro da distância em que os chunks já carregaram).
pub(crate) const SITE_REACH: i32 = 96;
/// O núcleo da vila: 7 x 7 colunas a cada 8 blocos (48 x 48 blocos).
const SITE_HALF: i32 = 24;
const SITE_STEP: i32 = 8;
/// Bom o bastante: no máximo 3 blocos de desnível e 2 colunas com copa de árvore.
const GOOD_SITE: i32 = 3 * 4 + 2 * 3;
/// Área limpa de árvores em volta do centro (64 x 64), em blocos de 16 x 16 x 32 por comando.
const CLEAR_HALF: i32 = 32;
const TAG: &str = "veio_village";

struct Arrival {
    player: u128,
    name: String,
    deadline: u64,
    placed: bool,
}

static ARRIVALS: Mutex<Vec<Arrival>> = Mutex::new(Vec::new());
/// Quem renasceu e vai para o centro no próximo tick: `run_command` falha dentro do gancho de
/// renascimento, que roda com o estado do jogador trocado.
static RESPAWNS: Mutex<Vec<String>> = Mutex::new(Vec::new());
static RESPAWNING: AtomicBool = AtomicBool::new(false);
static ARRIVING: AtomicBool = AtomicBool::new(false);
/// Centro conhecido e se veio de um centro de controle.
static CENTER: Mutex<Option<([i32; 3], bool)>> = Mutex::new(None);

/// Nomes de jogador vão para dentro de comandos: só letras, números e `_`.
pub(crate) fn safe_name(name: &str) -> bool {
    (1..=16).contains(&name.len()) && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
}

pub(crate) fn center() -> Option<([i32; 3], bool)> {
    CENTER.lock().ok().and_then(|center| *center)
}

pub(crate) fn logged_in(player: u128, name: &str, now: u64) {
    if !safe_name(name) {
        return;
    }
    if let Ok(mut arrivals) = ARRIVALS.lock() {
        arrivals.retain(|arrival| arrival.player != player);
        arrivals.push(Arrival { player, name: name.to_owned(), deadline: now + WAIT, placed: false });
        ARRIVING.store(true, Ordering::Release);
    }
}

/// Quem renasce volta ao próprio lar (`home.rs`). Sem lar e sem cama (a da casa do fundador ou a
/// de fora, ver `crate::home`), volta ao centro da vila; com cama, o jogo leva o jogador para ela.
/// O teleporte sai no tick seguinte: dentro do gancho o jogo ainda reposiciona o jogador.
pub(crate) fn respawned(player: u128, name: &str) {
    let lar = home::home_of(name).is_some();
    if safe_name(name)
        && (lar || (center().is_some() && !super::super::home::has_bed(player)))
        && let Ok(mut respawns) = RESPAWNS.lock()
    {
        respawns.retain(|known| known != name);
        respawns.push(name.to_owned());
        RESPAWNING.store(true, Ordering::Release);
    }
}

fn send_respawns(host: &mut dyn Host) {
    if !RESPAWNING.swap(false, Ordering::AcqRel) {
        return;
    }
    let names = RESPAWNS.lock().map(|mut list| std::mem::take(&mut *list)).unwrap_or_default();
    for name in names {
        if let Some(home) = home::home_of(&name) {
            let [x, y, z] = home.pos;
            host.run_command(&format!("execute in {} run tp {name} {x} {} {z}", home.dimension, y + 1));
        } else if let Some(([x, y, z], _)) = center() {
            host.run_command(&format!("execute in {OVERWORLD} run tp {name} {x} {y} {z}"));
        }
    }
}

/// Centro da vila em volta de `around`: a média dos aldeões perto daquele com mais vizinhos.
pub(crate) fn find_village(host: &dyn Host, around: [f64; 3]) -> Option<[i32; 3]> {
    let near: Vec<[f64; 3]> = production::ranch::villager_positions(host, OVERWORLD).iter().copied()
        .map(|(_, at)| at)
        .filter(|at| (at[0] - around[0]).hypot(at[2] - around[2]) <= SEARCH)
        .collect();
    if near.len() < MIN_VILLAGERS {
        return None;
    }
    let close = |a: &[f64; 3], b: &[f64; 3]| (a[0] - b[0]).hypot(a[2] - b[2]) <= NEIGHBORS;
    let hub = near.iter().max_by_key(|a| near.iter().filter(|b| close(a, b)).count())?;
    let group: Vec<&[f64; 3]> = near.iter().filter(|b| close(hub, b)).collect();
    let mean = |axis: usize| group.iter().map(|at| at[axis]).sum::<f64>() / group.len() as f64;
    let (x, z) = (mean(0).floor() as i32, mean(2).floor() as i32);
    let y = host.surface_heights(OVERWORLD, &[(x, z)], NativeHeightmap::MotionBlockingNoLeaves).ok()
        .and_then(|heights| heights.first().copied().flatten())
        .unwrap_or(mean(1).floor() as i32);
    Some([x, y, z])
}

/// Nota de um terreno para a vila (menor é melhor), pelas alturas das colunas: chão sólido
/// (`floor`), chão com água (`ground`, sem folhas) e topo com folhas (`top`). Água recusa o lugar;
/// desnível e copas de árvore pesam.
pub(crate) fn site_score(floor: &[i32], ground: &[i32], top: &[i32]) -> Option<i32> {
    if floor.is_empty() || floor.len() != ground.len() || floor.len() != top.len() { return None; }
    if floor.iter().zip(ground).any(|(floor, ground)| ground > floor) { return None; }
    let leaves = ground.iter().zip(top).filter(|(ground, top)| top > ground).count() as i32;
    let range = floor.iter().max()? - floor.iter().min()?;
    Some(range * 4 + leaves * 3)
}

fn site_columns(x: i32, z: i32) -> Vec<(i32, i32)> {
    let steps = (-SITE_HALF..=SITE_HALF).step_by(SITE_STEP as usize);
    steps.clone().flat_map(|dx| steps.clone().map(move |dz| (x + dx, z + dz))).collect()
}

fn heights(host: &dyn Host, columns: &[(i32, i32)], map: NativeHeightmap) -> Option<Vec<i32>> {
    host.surface_heights(OVERWORLD, columns, map).ok()?.into_iter().collect()
}

/// O terreno mais plano e sem árvores perto de `around`: anéis de 16 em 16 blocos, 8 direções,
/// o primeiro bom o bastante ou o melhor visto. Coluna em chunk não carregado descarta o lugar.
/// Devolve o centro, a altura do chão e se ainda há copas para tirar.
pub(crate) fn flat_site(host: &dyn Host, around: [i32; 2]) -> Option<([i32; 3], bool)> {
    let mut best: Option<(i32, [i32; 3])> = None;
    for ring in (0..=SITE_REACH).step_by(16) {
        let points: Vec<[i32; 2]> = if ring == 0 { vec![around] } else {
            [(1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1), (0, -1), (1, -1)].iter()
                .map(|(dx, dz)| [around[0] + dx * ring, around[1] + dz * ring]).collect()
        };
        for [x, z] in points {
            let columns = site_columns(x, z);
            let (Some(floor), Some(ground), Some(top)) = (heights(host, &columns, NativeHeightmap::OceanFloor),
                heights(host, &columns, NativeHeightmap::MotionBlockingNoLeaves),
                heights(host, &columns, NativeHeightmap::MotionBlocking)) else { continue };
            let Some(score) = site_score(&floor, &ground, &top) else { continue };
            let mut sorted = floor.clone();
            sorted.sort_unstable();
            let at = [x, sorted[sorted.len() / 2], z];
            if best.is_none_or(|(known, _)| score < known) { best = Some((score, at)); }
            if score <= GOOD_SITE { return Some((at, score > 0)); }
        }
    }
    best.map(|(score, at)| (at, score > 0))
}

/// Tira folhas e troncos de árvore em volta do centro, antes de gerar a vila.
fn clear_trees(host: &mut dyn Host, [x, y, z]: [i32; 3]) {
    for dx in (-CLEAR_HALF..CLEAR_HALF).step_by(16) {
        for dz in (-CLEAR_HALF..CLEAR_HALF).step_by(16) {
            let (x0, z0) = (x + dx, z + dz);
            for tag in ["minecraft:leaves", "minecraft:logs"] {
                host.run_command(&format!("execute in {OVERWORLD} run fill {x0} {} {z0} {} {} {} minecraft:air replace #{tag}",
                    y - 1, x0 + 15, y + 30, z0 + 15));
            }
        }
    }
}

/// Quem chega nasce perto da vila, não dentro dela: a `APPROACH` blocos do centro, do lado de
/// onde o jogador estava, e o tutorial aponta a vila (`onboarding::intro`). Quem já está entre
/// `NEAR_MIN` e `SEARCH` blocos fica onde está.
pub(crate) const APPROACH: i32 = 44;
const NEAR_MIN: f64 = 28.0;

/// Onde o jogador aparece: `None` se já está perto da vila; senão um chão seco e sem árvore a
/// `APPROACH` blocos do centro, começando pelo rumo do jogador; sem terreno lido, o centro.
pub(crate) fn approach(host: &dyn Host, center: [i32; 3], from: [f64; 3]) -> Option<[i32; 3]> {
    let (dx, dz) = (from[0] - f64::from(center[0]), from[2] - f64::from(center[2]));
    let distance = dx.hypot(dz);
    if (NEAR_MIN..=SEARCH).contains(&distance) {
        return None;
    }
    // Rumos em 8 direções, do mais perto do jogador ao mais longe (sem rumo: o sul, longe da casa).
    let angle = if distance < 1.0 { std::f64::consts::FRAC_PI_2 } else { dz.atan2(dx) };
    let mut headings: Vec<f64> = (0..8).map(|k| f64::from(k) * std::f64::consts::FRAC_PI_4).collect();
    let gap = |a: f64| { let d = (a - angle).rem_euclid(std::f64::consts::TAU); d.min(std::f64::consts::TAU - d) };
    headings.sort_by(|a, b| gap(*a).total_cmp(&gap(*b)));
    let points: Vec<(i32, i32)> = headings.iter()
        .map(|a| (center[0] + (a.cos() * f64::from(APPROACH)).round() as i32, center[2] + (a.sin() * f64::from(APPROACH)).round() as i32))
        .collect();
    if let (Some(floor), Some(ground), Some(top)) = (heights(host, &points, NativeHeightmap::OceanFloor),
        heights(host, &points, NativeHeightmap::MotionBlockingNoLeaves), heights(host, &points, NativeHeightmap::MotionBlocking))
    {
        for (index, (x, z)) in points.iter().enumerate() {
            if floor[index] == ground[index] && top[index] == ground[index] {
                return Some([*x, ground[index], *z]);
            }
        }
    }
    Some(center)
}

/// Fixa os spawns na vila e leva o jogador para perto dela, só se ele ainda não tem a tag.
fn settle(host: &mut dyn Host, name: &str, [x, y, z]: [i32; 3], from: [f64; 3]) -> bool {
    let only_new = format!("execute in {OVERWORLD} as {name} unless entity @s[tag={TAG}] run");
    let ok = host.run_command(&format!("{only_new} setworldspawn {x} {y} {z}"));
    if !ok {
        return false;
    }
    host.run_command(&format!("{only_new} spawnpoint @s {x} {y} {z}"));
    host.run_command(&format!("{only_new} tellraw @s {{\"translate\":\"veio.spawn.village\",\"color\":\"gold\"}}"));
    if let Some([ax, ay, az]) = approach(host, [x, y, z], from) {
        host.run_command(&format!("{only_new} tp @s {ax} {ay} {az}"));
    }
    host.run_command(&format!("tag {name} add {TAG}"));
    true
}

/// Chamado no tick do jogo. Sai na primeira comparação quando ninguém está chegando.
pub(crate) fn tick(host: &mut dyn Host, now: u64) {
    send_respawns(host);
    if !ARRIVING.load(Ordering::Acquire) || !now.is_multiple_of(20) {
        return;
    }
    let players = host.server_players();
    // A janela também chama o tick e pode não ver os jogadores do servidor: espera o servidor.
    if players.is_empty() {
        return;
    }
    let Ok(mut arrivals) = ARRIVALS.lock().map(|mut list| std::mem::take(&mut *list)) else { return };
    let mut waiting = Vec::new();
    for mut arrival in arrivals.drain(..) {
        let Some(player) = players.iter().find(|player| player.uuid == arrival.player) else {
            // Ainda não listado (ou saiu): espera até o prazo; no próximo login tenta de novo.
            if now < arrival.deadline + WAIT {
                waiting.push(arrival);
            }
            continue;
        };
        if player.dimension != OVERWORLD {
            waiting.push(arrival);
            continue;
        }
        // Centro já conhecido (a vila do anfitrião ou o centro de controle): quem entra na LAN
        // vai para lá, mesmo longe. Sem isso, o convidado ganharia outra vila onde apareceu.
        let known = center().map(|(at, _)| at);
        let found = known.or_else(|| find_village(host, player.position));
        if let Some(at) = found {
            if settle(host, &arrival.name, at, player.position) {
                if let Ok(mut center) = CENTER.lock()
                    && center.is_none()
                {
                    *center = Some((at, false));
                }
                continue;
            }
            waiting.push(arrival);
            continue;
        }
        if now < arrival.deadline {
            waiting.push(arrival);
            continue;
        }
        let name = arrival.name.clone();
        let only_new = format!("execute in {OVERWORLD} as {name} unless entity @s[tag={TAG}] at @s run");
        if !arrival.placed {
            // Nenhuma vila perto: escolhe terreno plano e sem árvores, limpa e gera a vila lá.
            arrival.placed = true;
            let around = [player.position[0].floor() as i32, player.position[2].floor() as i32];
            if let Some((site, trees)) = flat_site(host, around) {
                if trees { clear_trees(host, site); }
                let [x, y, z] = site;
                if host.run_command(&format!("execute in {OVERWORLD} run place structure minecraft:village_plains {x} {y} {z}"))
                    && settle(host, &name, site, player.position)
                {
                    if let Ok(mut center) = CENTER.lock() && center.is_none() {
                        *center = Some((site, false));
                    }
                    continue;
                }
            } else {
                host.run_command(&format!("{only_new} place structure minecraft:village_plains ~ ~ ~"));
            }
            arrival.deadline = now + WAIT;
            waiting.push(arrival);
        } else {
            host.run_command(&format!("{only_new} tellraw @s {{\"translate\":\"veio.spawn.none\",\"color\":\"yellow\"}}"));
            host.run_command(&format!("tag {name} add {TAG}"));
        }
    }
    if let Ok(mut list) = ARRIVALS.lock() {
        list.extend(waiting);
        ARRIVING.store(!list.is_empty(), Ordering::Release);
    }
}

/// O centro de controle com administrador vira o centro da vila (só no Overworld). Muda os spawns
/// uma vez por posição.
pub(crate) fn control_center(host: &mut dyn Host, [x, y, z]: [i32; 3]) {
    if host.dimension() != OVERWORLD {
        return;
    }
    let at = [x, y + 1, z];
    if center() == Some((at, true)) {
        return;
    }
    let [cx, cy, cz] = at;
    if host.run_command(&format!("execute in {OVERWORLD} run setworldspawn {cx} {cy} {cz}")) {
        host.run_command(&format!("execute in {OVERWORLD} run spawnpoint @a {cx} {cy} {cz}"));
        if let Ok(mut center) = CENTER.lock() {
            *center = Some((at, true));
        }
    }
}

#[cfg(test)]
pub(crate) fn reset() {
    if let Ok(mut center) = CENTER.lock() {
        *center = None;
    }
    if let Ok(mut arrivals) = ARRIVALS.lock() {
        arrivals.clear();
    }
    if let Ok(mut respawns) = RESPAWNS.lock() {
        respawns.clear();
    }
}
