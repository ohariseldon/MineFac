//! Foco: o centro de controle manda o jogador até o lugar que precisa dele.
//!
//! Com uma **bússola**, clique no centro de controle (ou no lar a partir do nível 3, o
//! escritório). O jogador recebe a pendência mais urgente da página Atenção; cliques seguidos
//! passam para a próxima. Enquanto o foco dura:
//!
//! - a barra de ação mostra o que é, a distância e a direção (N, NE, L...);
//! - partículas verdes marcam o bloco;
//! - perto dele, a barra explica o que fazer ali (a mesma dica do painel);
//! - quando o problema some (o posto voltou a trabalhar, a zona cresceu, o armazém esvaziou), o
//!   jogador ouve um sinal e recebe a próxima pendência da mesma vila, até a vila ficar em ordem.
//!
//! Agachar com a bússola no centro limpa o foco. Um foco vence em 10 minutos. Só lê os registros
//! em memória dos postos, zonas e armazéns: não varre blocos nem entidades.
use super::*;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

pub(crate) const COMPASS: &str = "minecraft:compass";
/// Ticks do relógio entre duas atualizações da barra de ação.
const EVERY: u64 = 10;
/// Distância (horizontal) em que o jogador "chegou".
const ARRIVED: f64 = 3.5;
/// Perto assim, um posto que sumiu do registro foi removido (o chunk está carregado).
const SURE: f64 = 16.0;
const EXPIRE: u64 = 12_000;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Focus {
    pub player: u128,
    pub dimension: String,
    pub center: [i32; 3],
    pub issue: center::Issue,
    pub index: usize,
    pub since: u64,
}

static FOCUS: Mutex<Vec<Focus>> = Mutex::new(Vec::new());
static ACTIVE: AtomicBool = AtomicBool::new(false);
static LAST: AtomicU64 = AtomicU64::new(0);

pub(crate) fn current(player: u128) -> Option<Focus> {
    FOCUS.lock().ok()?.iter().find(|focus| focus.player == player).cloned()
}

fn set(focus: Focus) {
    if let Ok(mut list) = FOCUS.lock() {
        list.retain(|known| known.player != focus.player);
        list.push(focus);
        ACTIVE.store(true, Ordering::Release);
    }
}

pub(crate) fn clear(player: u128) {
    if let Ok(mut list) = FOCUS.lock() {
        list.retain(|known| known.player != player);
        ACTIVE.store(!list.is_empty(), Ordering::Release);
    }
}

/// "Lavrador: trabalhador sem comida"
pub(crate) fn describe(issue: &center::Issue) -> String {
    format!("{}: {}", Text::tr(&format!("veio.center.kind.{}", issue.kind), &[]).plain(), center::problem_text(issue.code))
}

/// Bússola no centro de controle.
pub(crate) fn from_center(host: &mut dyn Host, pos: [i32; 3]) -> Option<Flow> {
    let mut staffed = None;
    host.change_mod_block_entity(pos, &mut |entity| {
        if entity.id == CONTROL_CENTER {
            staffed = Some(worker(entity).is_some());
        }
    });
    let staffed = staffed?;
    let Some(player) = center::nearest_player(host, pos, 12.0) else { return Some(Flow::Cancel) };
    if !staffed {
        host.tell(player, Text::tr("veio.center.need_admin", &[]));
        return Some(Flow::Cancel);
    }
    if host.sneaking() {
        clear(player);
        host.action_bar(player, Text::tr("veio.focus.cleared", &[]));
        return Some(Flow::Cancel);
    }
    choose(host, player, pos, defense::clock());
    host.play_sound("minecraft:item.lodestone_compass.lock", center(pos), 4, 0.7, 1.0);
    Some(Flow::Cancel)
}

/// Escolhe a próxima pendência da vila do centro em `origin` para o jogador. `false` se a vila
/// está em ordem.
pub(crate) fn choose(host: &mut dyn Host, player: u128, origin: [i32; 3], now: u64) -> bool {
    let dimension = host.dimension();
    let list = center::issues(dimension, origin, now);
    if list.is_empty() {
        clear(player);
        host.tell(player, Text::tr("veio.focus.none", &[]));
        return false;
    }
    let index = current(player).filter(|focus| focus.center == origin && focus.dimension == dimension)
        .map_or(0, |focus| (focus.index + 1) % list.len());
    let issue = list[index];
    set(Focus { player, dimension: dimension.to_owned(), center: origin, issue, index, since: now });
    host.tell(player, Text::tr("veio.focus.set", &[&(index + 1), &list.len(), &describe(&issue), &center::format_pos(issue.pos)]));
    true
}

/// O problema da pendência sumiu? `None`: ainda não dá para saber (posto longe e sem notícia).
fn resolved(dimension: &str, issue: &center::Issue, player_near: bool, now: u64) -> Option<bool> {
    match issue.code {
        center::ZONE_READY => zones::zones_near(dimension, issue.pos, 0, now).first().map(|zone| {
            !zones::requirement(zone.unlocked).is_some_and(|(_, work, villagers)| zone.work >= work && zone.villagers >= villagers)
        }),
        center::STORE_FULL => storage::stores_near(dimension, issue.pos, 0, now).first()
            .map(|store| store.used * 10 < store.total * 9),
        code => match center::post_at(dimension, issue.pos, now) {
            Some(post) => Some(post.status != code),
            None => player_near.then_some(true),
        },
    }
}

/// Chamado no tick do jogo. Sai na primeira comparação quando ninguém tem foco.
pub(crate) fn tick(host: &mut dyn Host, now: u64) {
    if !ACTIVE.load(Ordering::Acquire) || now.saturating_sub(LAST.load(Ordering::Relaxed)) < EVERY {
        return;
    }
    let players = host.server_players();
    if players.is_empty() {
        // Lado da janela: o servidor faz o trabalho.
        return;
    }
    LAST.store(now, Ordering::Relaxed);
    let Ok(list) = FOCUS.lock().map(|list| list.clone()) else { return };
    let pulse = (now / EVERY) % 2 == 0;
    for focus in list {
        if now.saturating_sub(focus.since) > EXPIRE {
            clear(focus.player);
            host.action_bar(focus.player, Text::tr("veio.focus.expired", &[]));
            continue;
        }
        let Some(player) = players.iter().find(|player| player.uuid == focus.player) else { continue };
        if player.dimension != focus.dimension {
            continue;
        }
        let target = center(focus.issue.pos);
        let (dx, dz) = (target[0] - player.position[0], target[2] - player.position[2]);
        let distance = dx.hypot(dz);
        if resolved(&focus.dimension, &focus.issue, distance <= SURE, now) == Some(true) {
            host.tell(focus.player, Text::tr("veio.focus.resolved", &[&describe(&focus.issue)]));
            host.play_sound("minecraft:entity.experience_orb.pickup", player.position, 7, 0.8, 1.2);
            clear(focus.player);
            let next = center::issues(&focus.dimension, focus.center, now);
            if let Some(issue) = next.first() {
                set(Focus { issue: *issue, index: 0, since: now, ..focus.clone() });
                host.tell(focus.player, Text::tr("veio.focus.next", &[&next.len(), &describe(issue)]));
            } else {
                host.tell(focus.player, Text::tr("veio.focus.all_clear", &[]));
            }
            continue;
        }
        if distance > ARRIVED {
            host.action_bar(focus.player, Text::tr("veio.focus.bar", &[&describe(&focus.issue),
                &(distance.round() as i32), &center::direction_text(dx, dz)]));
        } else {
            host.action_bar(focus.player, Text::tr("veio.focus.here", &[&center::advice(focus.issue.code)]));
        }
        if pulse {
            host.send_particles(&ParticleOptions::simple(ParticleType::HappyVillager), None,
                [target[0], target[1] + 1.6, target[2]], [0.15, 0.9, 0.15], 0.0, 12);
        }
    }
}
