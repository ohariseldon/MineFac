//! Censo das máquinas e ritmo do dia.
//!
//! Cada máquina que consome ou gera energia se registra quando muda de estado e, de resto, a
//! cada `REPORT_EVERY` ticks (fase espalhada pela posição): uma trava por mudança, não por tick.
//! O quadro lê o censo a cada segundo; o tick global avisa jogadores a cada `ALERT_EVERY` ticks
//! e anuncia amanhecer e anoitecer. Nada aqui lê o mundo.
use super::*;
use std::collections::HashMap;

const STALE: u64 = 400;
const REPORT_EVERY: u64 = 100;
const ALERT_EVERY: u64 = 100;
const ALERT_RADIUS: i32 = 32;
/// Repetir o mesmo aviso só depois de um minuto; um problema novo avisa na hora.
const REMIND: u64 = 1200;
/// Falta de energia ou combustível por 10 s vira alerta; saída cheia, só depois de 1 min,
/// porque uma linha equilibrada para quando o estoque enche.
const POWER_GRACE: u64 = 200;
const BLOCKED_GRACE: u64 = 1200;
const BOARD_REACH: i32 = 96;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Kind { Power, Drill, Press, Assembler, Other }

#[derive(Clone, Copy, Debug)]
struct Machine { kind: Kind, status: i32, since: u64, seen: u64 }

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct BoardInfo { pub phase: usize, pub order: usize, pub done: u32, pub paid: bool, pub seen: u64 }

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) struct Summary {
    pub working: u32, pub power: u32, pub fuel: u32, pub blocked: u32, pub damaged: u32, pub total: u32,
    /// Primeira hora: 1 energia, 2 extratora, 4 prensa, 8 montador.
    pub setup: u32,
}

impl Summary {
    pub(crate) fn stuck(&self) -> u32 { self.power + self.fuel + self.blocked + self.damaged }
}

type Key = (&'static str, [i32; 3]);
/// Itens produzidos desde o carregamento; atômico para a produção não disputar a trava.
static PRODUCED: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
fn produced() -> u64 { PRODUCED.load(std::sync::atomic::Ordering::Relaxed) }

#[derive(Default)]
struct Census {
    machines: HashMap<Key, Machine>,
    boards: HashMap<Key, BoardInfo>,
    /// `PRODUCED` na virada do último amanhecer.
    produced_at_dawn: u64,
    day: Option<i64>,
    dusk_day: i64,
    alerts: HashMap<u128, (u32, u64)>,
    /// Último dia em que cada dimensão recebeu o pagamento do pedido: um por dia, não um por quadro.
    paid_day: HashMap<&'static str, i64>,
}

#[cfg(not(test))]
fn with_census<T>(run: impl FnOnce(&mut Census) -> T) -> T {
    static STATE: std::sync::LazyLock<std::sync::Mutex<Census>> = std::sync::LazyLock::new(Default::default);
    run(&mut STATE.lock().unwrap_or_else(|poisoned| poisoned.into_inner()))
}

#[cfg(test)]
fn with_census<T>(run: impl FnOnce(&mut Census) -> T) -> T {
    thread_local! { static STATE: std::cell::RefCell<Census> = Default::default(); }
    STATE.with(|state| run(&mut state.borrow_mut()))
}

fn kind(block: &str) -> Option<Kind> {
    if !(needs_power(block) || is_generator(block)) { return None; }
    Some(match block {
        DRILL | DRILL_MK2 | production::DRILL_MK3 => Kind::Drill,
        PRESS => Kind::Press,
        ASSEMBLER => Kind::Assembler,
        _ if is_generator(block) => Kind::Power,
        _ => Kind::Other,
    })
}

/// Chamado depois do tick de cada bloco da fábrica.
pub(crate) fn observe(dimension: &'static str, pos: [i32; 3], entity: &ModBlockEntity, previous: i32, produced: i32, now: u64) {
    if produced > 0 { PRODUCED.fetch_add(produced as u64, std::sync::atomic::Ordering::Relaxed); }
    let Some(kind) = kind(&entity.id) else { return };
    let status = number(entity, "Status");
    let phase = (pos[0] ^ pos[2]).rem_euclid(REPORT_EVERY as i32) as u64;
    if status == previous && (now + phase) % REPORT_EVERY != 0 {
        return;
    }
    with_census(|census| {
        let key = (dimension, pos);
        match census.machines.get_mut(&key) {
            Some(known) => {
                if known.status != status || known.kind != kind { *known = Machine { kind, status, since: now, seen: now }; }
                else { known.seen = now; }
            }
            None => {
                census.machines.retain(|_, machine| now.saturating_sub(machine.seen) <= STALE);
                census.machines.insert(key, Machine { kind, status, since: now, seen: now });
            }
        }
    });
}

/// O primeiro quadro que fecha o pedido do dia na dimensão recebe; os outros, não.
pub(crate) fn claim_order(dimension: &'static str, day: i64) -> bool {
    with_census(|census| {
        let paid = census.paid_day.get(dimension) == Some(&day);
        if !paid { census.paid_day.insert(dimension, day); }
        !paid
    })
}

pub(crate) fn report_board(dimension: &'static str, pos: [i32; 3], info: BoardInfo) {
    with_census(|census| {
        if !census.boards.contains_key(&(dimension, pos)) {
            census.boards.retain(|_, board| info.seen.saturating_sub(board.seen) <= STALE);
        }
        census.boards.insert((dimension, pos), info);
    });
}

fn near(a: [i32; 3], b: [i32; 3], radius: i32) -> bool { (0..3).all(|axis| (a[axis] - b[axis]).abs() <= radius) }

fn summarize<'a>(machines: impl Iterator<Item = &'a Machine>, now: u64) -> Summary {
    let mut seen = Summary::default();
    for machine in machines {
        seen.total += 1;
        let held = now.saturating_sub(machine.since);
        match machine.status {
            1 => seen.working += 1,
            5 if held >= POWER_GRACE => seen.power += 1,
            6 if held >= POWER_GRACE => seen.fuel += 1,
            3 if held >= BLOCKED_GRACE => seen.blocked += 1,
            13 => seen.damaged += 1,
            _ => {}
        }
        seen.setup |= match machine.kind { Kind::Power => 1, Kind::Drill => 2, Kind::Press => 4, Kind::Assembler => 8, Kind::Other => 0 };
    }
    seen
}

pub(crate) fn summary(dimension: &str, at: [i32; 3], radius: i32, now: u64) -> Summary {
    with_census(|census| summarize(census.machines.iter()
        .filter(|((dim, pos), machine)| *dim == dimension && near(*pos, at, radius) && now.saturating_sub(machine.seen) <= STALE)
        .map(|(_, machine)| machine), now))
}

/// Tick global: alertas e ritmo do dia.
pub(crate) fn tick(host: &mut dyn Host, now: u64) {
    if now.is_multiple_of(ALERT_EVERY) { alert(host, now); }
    if now.is_multiple_of(20) { rhythm(host, now); }
}

fn to_block(position: [f64; 3]) -> [i32; 3] { position.map(|v| v.floor() as i32) }

fn alert(host: &mut dyn Host, now: u64) {
    let players = host.server_players();
    let mut messages = Vec::new();
    with_census(|census| {
        census.machines.retain(|_, machine| now.saturating_sub(machine.seen) <= STALE);
        for player in &players {
            let at = to_block(player.position);
            let seen = summarize(census.machines.iter()
                .filter(|((dim, pos), _)| *dim == player.dimension && near(*pos, at, ALERT_RADIUS))
                .map(|(_, machine)| machine), now);
            if seen.stuck() == 0 {
                census.alerts.remove(&player.uuid);
                continue;
            }
            let signature = seen.power << 24 | seen.fuel << 16 | seen.blocked << 8 | seen.damaged;
            let due = census.alerts.get(&player.uuid)
                .is_none_or(|&(last, at)| last != signature || now.saturating_sub(at) >= REMIND);
            if due {
                census.alerts.insert(player.uuid, (signature, now));
                messages.push((player.uuid, seen));
            }
        }
    });
    for (uuid, seen) in messages {
        host.action_bar(uuid, Text::tr("veio.alert.near", &[&seen.power, &seen.fuel, &seen.blocked, &seen.damaged]));
    }
}

/// Amanhecer: pedido do dia. Anoitecer: balanço do dia. Só para quem tem um quadro por perto.
fn rhythm(host: &mut dyn Host, now: u64) {
    let time = host.day_time();
    let (day, hour) = (time.div_euclid(DAY), time.rem_euclid(DAY));
    let event = with_census(|census| {
        let previous = census.day.replace(day);
        census.boards.retain(|_, board| now.saturating_sub(board.seen) <= STALE);
        match previous {
            Some(known) if day > known => {
                census.produced_at_dawn = produced();
                Some(true)
            }
            Some(_) if hour >= 12000 && census.dusk_day != day => {
                census.dusk_day = day;
                Some(false)
            }
            None => {
                // Mundo recém-carregado: começa a contar sem anunciar; à noite, não repete o anoitecer.
                census.produced_at_dawn = produced();
                if hour >= 12000 { census.dusk_day = day; }
                None
            }
            _ => None,
        }
    });
    let Some(dawn) = event else { return };
    for player in host.server_players() {
        let at = to_block(player.position);
        let Some((board_pos, board)) = with_census(|census| census.boards.iter()
            .filter(|((dim, pos), _)| *dim == player.dimension && near(*pos, at, BOARD_REACH))
            .min_by_key(|((_, pos), _)| (0..3).map(|axis| (pos[axis] - at[axis]).abs()).sum::<i32>())
            .map(|((_, pos), board)| (*pos, *board))) else { continue };
        if dawn {
            let order = ORDERS[order_for(day, board.phase)];
            host.tell(player.uuid, Text::tr("veio.day.dawn", &[&(day + 1), &order.3, &Text::tr(order.2, &[]).plain(), &order.4]));
            continue;
        }
        let made = produced().saturating_sub(with_census(|census| census.produced_at_dawn));
        host.tell(player.uuid, Text::tr("veio.day.dusk", &[&(day + 1), &made]));
        let order = ORDERS[board.order];
        host.tell(player.uuid, if board.paid { Text::tr("veio.day.dusk_paid", &[]) }
            else { Text::tr("veio.day.dusk_order", &[&board.done.min(order.3), &order.3, &Text::tr(order.2, &[]).plain()]) });
        let seen = summary(player.dimension.as_str(), board_pos, RADIUS, now);
        if seen.stuck() > 0 {
            host.tell(player.uuid, Text::tr("veio.day.dusk_issue", &[&(seen.power + seen.fuel), &seen.blocked, &seen.damaged]));
        }
    }
}
