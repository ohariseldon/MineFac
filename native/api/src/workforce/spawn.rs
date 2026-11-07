//! Spawn no centro da vila.
//!
//! - **Mundo novo / jogador novo:** no login, o Veio procura uma vila a até `SEARCH` blocos do
//!   jogador pelos aldeões que já registra (`ranch::villager_positions`); o centro é a média dos
//!   aldeões em volta daquele que tem mais vizinhos (onde a vila se reúne). O jogador vai para lá,
//!   e o spawn do mundo e o dele ficam nesse ponto. Sem vila em `WAIT` ticks, o Veio gera uma vila de
//!   planície onde o jogador está (`/place structure`) e procura de novo.
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
const TAG: &str = "veio_village";

struct Arrival {
    player: u128,
    name: String,
    deadline: u64,
    placed: bool,
}

static ARRIVALS: Mutex<Vec<Arrival>> = Mutex::new(Vec::new());
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

/// Quem renasce volta ao centro da vila (mesmo com cama em outro lugar).
pub(crate) fn respawned(host: &mut dyn Host, name: &str) {
    if let Some(([x, y, z], _)) = center()
        && safe_name(name)
    {
        host.run_command(&format!("execute in {OVERWORLD} run tp {name} {x} {y} {z}"));
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

/// Leva o jogador ao centro e fixa os spawns, só se ele ainda não tem a tag.
fn settle(host: &mut dyn Host, name: &str, [x, y, z]: [i32; 3]) -> bool {
    let only_new = format!("execute in {OVERWORLD} as {name} unless entity @s[tag={TAG}] run");
    let ok = host.run_command(&format!("{only_new} setworldspawn {x} {y} {z}"));
    if !ok {
        return false;
    }
    host.run_command(&format!("{only_new} spawnpoint @s {x} {y} {z}"));
    host.run_command(&format!("{only_new} tellraw @s {{\"translate\":\"veio.spawn.village\",\"color\":\"gold\"}}"));
    host.run_command(&format!("{only_new} tp @s {x} {y} {z}"));
    host.run_command(&format!("tag {name} add {TAG}"));
    true
}

/// Chamado no tick do jogo. Sai na primeira comparação quando ninguém está chegando.
pub(crate) fn tick(host: &mut dyn Host, now: u64) {
    if !ARRIVING.load(Ordering::Acquire) || now % 20 != 0 {
        return;
    }
    let players = host.server_players();
    let Ok(mut arrivals) = ARRIVALS.lock().map(|mut list| std::mem::take(&mut *list)) else { return };
    let mut waiting = Vec::new();
    for mut arrival in arrivals.drain(..) {
        let Some(player) = players.iter().find(|player| player.uuid == arrival.player) else {
            // Saiu antes de chegar; tenta de novo no próximo login.
            continue;
        };
        if player.dimension != OVERWORLD {
            waiting.push(arrival);
            continue;
        }
        let known = center().filter(|(_, control)| *control).map(|(at, _)| at);
        let found = known.or_else(|| find_village(host, player.position));
        if let Some(at) = found {
            if settle(host, &arrival.name, at) {
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
            // Nenhuma vila perto: gera uma onde o jogador está e procura de novo.
            host.run_command(&format!("{only_new} place structure minecraft:village_plains ~ ~ ~"));
            arrival.placed = true;
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
}
