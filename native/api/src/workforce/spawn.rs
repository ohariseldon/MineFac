//! Spawn no centro da vila.
//!
//! - **Mundo novo / jogador novo:** no login, depois de `TOWN_WAIT` ticks (os chunks em volta
//!   carregarem), o Veio escolhe o terreno mais plano e com menos água a até `SITE_REACH` blocos
//!   (`flat_site`, que confere a área inteira da cidade) e ergue ali a **cidade pronta do Veio**
//!   (`town::build`): ela sobrescreve tudo no seu volume, inclusive árvores e galhos, e nasce
//!   com 20 moradores. O jogador aparece fora da cerca, a `APPROACH` blocos do centro, e o spawn
//!   do mundo e o dele ficam na praça. Vilas vanilla por perto não são mais adotadas.
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
/// Prazo de referência de chegada (a casa do fundador espera o dobro disto pelo centro).
pub(crate) const WAIT: u64 = 600;
/// Espera os chunks em volta carregarem antes de escolher o terreno da cidade.
pub(crate) const TOWN_WAIT: u64 = 100;
/// Tentativas de achar terreno com tudo carregado antes de erguer a cidade onde o jogador está.
const SITE_TRIES: u32 = 6;
/// Até onde procurar terreno para a cidade (perto: a cidade inteira precisa estar carregada).
pub(crate) const SITE_REACH: i32 = 48;
/// A área da cidade, conferida em 9 x 9 colunas.
const SITE_HALF: i32 = town::HALF;
const SITE_STEP: i32 = 11;
/// Bom o bastante: até 3 blocos de desnível, sem água e com pouca mata.
const GOOD_SITE: i32 = 3 * 4 + 8;
const TAG: &str = "veio_village";

struct Arrival {
    player: u128,
    name: String,
    deadline: u64,
    tries: u32,
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

/// Centro da vila: o centro de controle, o centro fixado nesta sessão ou a praça da cidade do Veio
/// (que fica salva no mundo e vale também depois de reabrir).
pub(crate) fn center() -> Option<([i32; 3], bool)> {
    CENTER.lock().ok().and_then(|center| *center).or_else(|| town::center().map(|at| (at, false)))
}

pub(crate) fn logged_in(player: u128, name: &str, now: u64) {
    if !safe_name(name) {
        return;
    }
    if let Ok(mut arrivals) = ARRIVALS.lock() {
        arrivals.retain(|arrival| arrival.player != player);
        arrivals.push(Arrival { player, name: name.to_owned(), deadline: now + TOWN_WAIT, tries: 0 });
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

/// Nota de um terreno para a cidade (menor é melhor), pelas alturas das colunas: chão sólido
/// (`floor`), chão com água (`ground`, sem folhas) e topo com folhas (`top`). A cidade aterra
/// lagoas e limpa a mata, então um pouco de água e de árvore é aceito; mais de um sexto de água
/// (um lago, um rio largo) recusa o lugar. O desnível pesa mais: é terra a mexer.
pub(crate) fn site_score(floor: &[i32], ground: &[i32], top: &[i32]) -> Option<i32> {
    if floor.is_empty() || floor.len() != ground.len() || floor.len() != top.len() { return None; }
    let water = floor.iter().zip(ground).filter(|(floor, ground)| ground > floor).count();
    if water * 6 > floor.len() { return None; }
    let leaves = ground.iter().zip(top).filter(|(ground, top)| top > ground).count() as i32;
    let range = floor.iter().max()? - floor.iter().min()?;
    Some(range * 4 + water as i32 * 6 + leaves)
}

fn site_columns(x: i32, z: i32) -> Vec<(i32, i32)> {
    let steps = (-SITE_HALF..=SITE_HALF).step_by(SITE_STEP as usize);
    steps.clone().flat_map(|dx| steps.clone().map(move |dz| (x + dx, z + dz))).collect()
}

fn heights(host: &dyn Host, columns: &[(i32, i32)], map: NativeHeightmap) -> Option<Vec<i32>> {
    host.surface_heights(OVERWORLD, columns, map).ok()?.into_iter().collect()
}

/// O terreno mais plano e seco perto de `around`: anéis de 16 em 16 blocos, 8 direções, o primeiro
/// bom o bastante ou o melhor visto. Coluna em chunk não carregado descarta o lugar (a cidade
/// inteira precisa estar carregada para sair completa). Devolve o centro (y no primeiro ar acima
/// do chão sólido) e se havia mata ou água a tirar.
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

/// Quem chega nasce perto da cidade, não dentro dela: a `APPROACH` blocos do centro (fora da
/// cerca, que fica a `town::HALF`), do lado de onde o jogador estava, e o tutorial aponta a cidade
/// (`onboarding::intro`). Quem já está entre `NEAR_MIN` e `SEARCH` blocos fica onde está.
pub(crate) const APPROACH: i32 = town::HALF + 12;
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
        // Centro já conhecido (a cidade do anfitrião ou o centro de controle): quem entra na LAN
        // vai para lá, mesmo longe. Sem isso, o convidado ganharia outra cidade onde apareceu.
        if let Some((at, _)) = center() {
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
        // Nenhuma cidade ainda: escolhe o terreno e ergue a cidade pronta do Veio. Sem terreno com
        // tudo carregado, tenta de novo algumas vezes; depois ergue onde o jogador está.
        let around = [player.position[0].floor() as i32, player.position[2].floor() as i32];
        arrival.tries += 1;
        let site = flat_site(host, around).map(|(at, _)| at).or_else(|| (arrival.tries >= SITE_TRIES).then(|| {
            let y = host.surface_heights(OVERWORLD, &[(around[0], around[1])], NativeHeightmap::OceanFloor).ok()
                .and_then(|heights| heights.first().copied().flatten())
                .unwrap_or(player.position[1].floor() as i32);
            [around[0], y, around[1]]
        }));
        let Some(site) = site else {
            arrival.deadline = now + TOWN_WAIT;
            waiting.push(arrival);
            continue;
        };
        if town::build(host, site, now) && settle(host, &arrival.name, site, player.position) {
            if let Ok(mut center) = CENTER.lock() && center.is_none() {
                *center = Some((site, false));
            }
            continue;
        }
        if arrival.tries >= SITE_TRIES * 2 {
            let name = arrival.name.clone();
            host.run_command(&format!("execute in {OVERWORLD} as {name} unless entity @s[tag={TAG}] run tellraw @s {{\"translate\":\"veio.spawn.none\",\"color\":\"yellow\"}}"));
            host.run_command(&format!("tag {name} add {TAG}"));
            continue;
        }
        arrival.deadline = now + TOWN_WAIT;
        waiting.push(arrival);
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
