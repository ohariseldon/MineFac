//! Quadro da fábrica: metas visíveis, pedido do dia e leitura de gargalos num só painel.
//!
//! - **Fases** (Satisfactory): entregar a pilha da fase no quadro, à mão ou por esteira, devolve a
//!   recompensa. Nas fases 2 a 4 ela é uma licença que concede o avanço da fase (mesmas receitas
//!   de juntar a pilha no inventário). O painel mostra o que falta, o que a fase libera e o
//!   gargalo provável.
//! - **Pedido do dia** (Punch Club): a cada amanhecer, um pedido da vila que vale até o próximo
//!   amanhecer e paga esmeraldas. Escolhido pelo dia e pela fase do quadro.
//! - **Gargalos** (Factorio): o censo (`census`) conta máquinas paradas por energia,
//!   combustível, saída cheia ou dano a até `RADIUS` blocos, e avisa na barra de ação.
//!
//! Primeira hora: na fase 1 o painel vira uma lista de montagem (energia, extratora, prensa,
//! montador) que se marca sozinha quando as máquinas aparecem perto do quadro.
use super::*;
use horizon_client::mod_api::{ScreenDrawEvent, Text};

pub(crate) mod census;

pub(crate) const BOARD: &str = "veio:factory_board";
pub(crate) const RADIUS: i32 = 64;
const EMERALD: &str = "minecraft:emerald";
const DAY: i64 = 24000;
/// O painel relê o censo a cada segundo; a entrega roda todo tick.
const REFRESH: u64 = 20;

pub(crate) struct Phase {
    pub key: &'static str,
    pub item: &'static str,
    pub item_key: &'static str,
    pub count: u32,
    pub reward: (&'static str, u32),
    pub reward_key: &'static str,
    /// O que a recompensa ou a fase seguinte abre, como chaves de tradução.
    pub unlocks: &'static [&'static str],
}

pub(crate) const PHASES: [Phase; 5] = [
    Phase { key: "first", item: PLATE, item_key: "item.veio.copper_plate", count: 16,
        reward: (BELT, 32), reward_key: "block.veio.belt",
        unlocks: &["block.veio.assembler", "block.veio.splitter", "block.veio.contract_hub"] },
    Phase { key: "workshop", item: GEAR, item_key: "item.veio.gear", count: 32,
        reward: ("veio:permit_workshop", 1), reward_key: "item.veio.permit_workshop",
        unlocks: &["item.veio.wrench", "block.veio.crusher", "block.veio.alloy_furnace"] },
    Phase { key: "steel", item: STEEL, item_key: "item.veio.steel_plate", count: 16,
        reward: ("veio:permit_steel", 1), reward_key: "item.veio.permit_steel",
        unlocks: &["block.veio.steam_engine", "block.veio.battery", "block.veio.geothermal"] },
    Phase { key: "electronics", item: CIRCUIT, item_key: "item.veio.circuit", count: 16,
        reward: ("veio:permit_electronics", 1), reward_key: "item.veio.permit_electronics",
        unlocks: &["block.veio.drill_mk2", "block.veio.solar", "block.veio.tesla"] },
    Phase { key: "depth", item: CIRCUIT, item_key: "item.veio.circuit", count: 32,
        reward: ("minecraft:diamond", 2), reward_key: "item.minecraft.diamond",
        unlocks: &["block.veio.drill_mk3"] },
];

/// Pedidos do dia: (fase mínima concluída, item, chave do nome, quantidade, esmeraldas).
pub(crate) const ORDERS: [(usize, &str, &str, u32, u32); 12] = [
    (0, PLATE, "item.veio.copper_plate", 64, 1),
    (0, "veio:iron_plate", "item.veio.iron_plate", 64, 1),
    (1, GEAR, "item.veio.gear", 32, 2),
    (1, "minecraft:bread", "item.minecraft.bread", 16, 2),
    (2, FRAME, "item.veio.frame", 32, 2),
    (2, STEEL, "item.veio.steel_plate", 32, 3),
    (3, MODULE, "item.veio.module", 16, 3),
    (3, "minecraft:cooked_beef", "item.minecraft.cooked_beef", 32, 2),
    (4, CIRCUIT, "item.veio.circuit", 32, 4),
    (4, STEEL, "item.veio.steel_plate", 64, 4),
    (5, MODULE, "item.veio.module", 48, 5),
    (5, CIRCUIT, "item.veio.circuit", 64, 6),
];

/// Pedido do dia para um quadro na fase `phase` (fases concluídas). Só os dois níveis mais
/// recentes entram no sorteio: o pedido acompanha o que a fábrica acabou de aprender.
pub(crate) fn order_for(day: i64, phase: usize) -> usize {
    let floor = phase.saturating_sub(1);
    let pool: Vec<usize> = (0..ORDERS.len()).filter(|&i| (floor..=phase).contains(&ORDERS[i].0)).collect();
    pool[(day.rem_euclid(9973) as usize * 7 + phase) % pool.len()]
}

pub(crate) fn owns(block: &str) -> bool { block == BOARD }

/// Entrada 0, recompensa 1.
pub(crate) fn layout() -> (usize, Vec<usize>, Vec<usize>) { (2, vec![0], vec![1]) }

/// Regra estática (menu): qualquer item de fase ou de pedido.
pub(crate) fn valid_input(slot: usize, stack: &ItemStack) -> bool {
    slot == 0 && (PHASES.iter().any(|phase| is_plain(stack, phase.item)) || ORDERS.iter().any(|order| is_plain(stack, order.1)))
}

/// Regra da entrega automática: só o que a fase ou o pedido atual pede.
pub(crate) fn accepts(entity: &ModBlockEntity, slot: usize, stack: &ItemStack) -> bool {
    if slot != 0 { return false; }
    let phase = phase(entity);
    (phase < PHASES.len() && is_plain(stack, PHASES[phase].item))
        || (number(entity, "OrderDone") < ORDERS[order(entity)].3 as i32 && is_plain(stack, ORDERS[order(entity)].1))
}

fn phase(entity: &ModBlockEntity) -> usize { (number(entity, "Phase") as usize).min(PHASES.len()) }
fn order(entity: &ModBlockEntity) -> usize { (number(entity, "Order") as usize).min(ORDERS.len() - 1) }

pub(crate) fn tick(host: &mut dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity) {
    let day = host.day_time().div_euclid(DAY);
    let now = defense::clock();
    if number(entity, "Day") != (day + 1) as i32 {
        set_number(entity, "Day", (day + 1) as i32);
        set_number(entity, "Order", order_for(day, phase(entity)) as i32);
        set_number(entity, "OrderDone", 0);
        set_number(entity, "OrderPaid", 0);
    }
    let mut delivered = false;
    let mut status = 2;
    if let Some(stack) = entity.items[0].clone() {
        let phase = phase(entity);
        let (item, done_key, need) = if phase < PHASES.len() && is_plain(&stack, PHASES[phase].item) {
            (PHASES[phase].item, "Delivered", PHASES[phase].count)
        } else {
            let order = ORDERS[order(entity)];
            (order.1, "OrderDone", order.3)
        };
        let done = number(entity, done_key) as u32;
        if is_plain(&stack, item) && done < need {
            let take = stack.count.min(need - done);
            consume(&mut entity.items[0], take);
            set_number(entity, done_key, (done + take) as i32);
            delivered = true;
        } else {
            status = 3; // item que nada pede, ou pedido já cheio: retirar
        }
    }
    let phase = phase(entity);
    if phase < PHASES.len() && number(entity, "Delivered") as u32 >= PHASES[phase].count {
        let (item, count) = PHASES[phase].reward;
        if can_add(&entity.items[1], item, count) {
            add(&mut entity.items[1], item, count);
            set_number(entity, "Phase", phase as i32 + 1);
            set_number(entity, "Delivered", 0);
            celebrate(host, pos, Text::tr("veio.board.complete", &[&Text::tr(&format!("veio.board.phase.{}", PHASES[phase].key), &[]).plain()]));
            // Cada fase nova chega com o gargalo que ela vai revelar.
            if let Some(next) = PHASES.get(phase + 1) {
                celebrate_quiet(host, pos, Text::tr(&format!("veio.board.hint.{}", next.key), &[]));
            }
        } else {
            status = 3;
        }
    }
    let order = ORDERS[order(entity)];
    if number(entity, "OrderDone") as u32 >= order.3 && number(entity, "OrderPaid") == 0 {
        if !can_add(&entity.items[1], EMERALD, order.4) {
            status = 3;
        } else if census::claim_order(host.dimension(), day) {
            add(&mut entity.items[1], EMERALD, order.4);
            set_number(entity, "OrderPaid", 1);
            celebrate(host, pos, Text::tr("veio.board.order_complete", &[&order.4]));
        } else {
            // Outro quadro já recebeu hoje: o pedido é da vila, não de cada quadro.
            set_number(entity, "OrderPaid", 2);
        }
    }
    if delivered && status != 3 { status = 1; }
    set_number(entity, "Status", status);
    let dimension = host.dimension();
    if (now + ((pos[0] ^ pos[2]) & 15) as u64).is_multiple_of(REFRESH)
        || !with_nbt_key("DTotal", |key| entity.nbt.contains_key(key)) {
        let seen = census::summary(dimension, pos, RADIUS, now);
        for (key, value) in [("DWorking", seen.working), ("DPower", seen.power + seen.fuel), ("DBlocked", seen.blocked),
            ("DDamaged", seen.damaged), ("DSetup", seen.setup), ("DTotal", seen.total)] {
            set_number(entity, key, value as i32);
        }
    }
    census::report_board(dimension, pos, census::BoardInfo {
        phase: self::phase(entity), order: self::order(entity),
        done: number(entity, "OrderDone") as u32, paid: number(entity, "OrderPaid") >= 1, seen: now,
    });
}

fn celebrate(host: &mut dyn Host, pos: [i32; 3], line: Text) {
    host.play_sound("minecraft:ui.toast.challenge_complete", center(pos), 4, 0.7, 1.0);
    celebrate_quiet(host, pos, line);
}

fn celebrate_quiet(host: &mut dyn Host, pos: [i32; 3], line: Text) {
    let at = center(pos);
    let dimension = host.dimension();
    for player in host.server_players() {
        if player.dimension == dimension && (0..3).map(|axis| (player.position[axis] - at[axis]).powi(2)).sum::<f64>() < 48.0 * 48.0 {
            host.tell(player.uuid, line.clone());
        }
    }
}

/// Os 13 valores do painel.
pub(crate) fn data(entity: &ModBlockEntity) -> [i32; 13] {
    let keys = ["Status", "Phase", "Delivered", "Order", "OrderDone", "OrderPaid", "Day",
        "DWorking", "DPower", "DBlocked", "DDamaged", "DSetup", "DTotal"];
    std::array::from_fn(|index| number(entity, keys[index]))
}

pub(crate) fn screen_slots() -> Vec<Option<[f32; 2]>> { vec![Some([138.0, 120.0]), Some([162.0, 120.0])] }
pub(crate) fn roles() -> &'static [&'static str] { &["input", "output"] }

const GREEN: u32 = 0x2f7a34;
const RED: u32 = 0xa02020;
const MUTED: u32 = 0x8a8a8a;
const BAR: u32 = 0xff2b2b2b;

fn tr(key: &str, args: &[&dyn std::fmt::Display]) -> String { Text::tr(key, args).plain() }

/// Largura útil na altura `row`: o painel inteiro acima dos slots, à esquerda deles abaixo.
fn width_at(row: f32) -> f32 { if row + 8.0 <= 119.0 { 174.0 } else { 120.0 } }

/// Escreve `text` a partir de `row`, quebrando por palavras na largura disponível em cada linha.
fn write(host: &dyn Host, event: &mut ScreenDrawEvent<'_>, row: &mut f32, text: &str, color: u32) {
    let [x, y] = event.origin;
    let mut line = String::new();
    for word in text.split(' ') {
        let candidate = if line.is_empty() { word.to_owned() } else { format!("{line} {word}") };
        if !line.is_empty() && host.text_width(&candidate) > width_at(*row) {
            host.text(event.batch, &line, [x + 10.0, y + *row], color, false);
            *row += 10.0;
            line = word.to_owned();
        } else {
            line = candidate;
        }
    }
    host.text(event.batch, &line, [x + 10.0, y + *row], color, false);
    *row += 10.0;
}

pub(crate) fn draw_screen(host: &dyn Host, event: &mut ScreenDrawEvent<'_>) {
    let [x, y] = event.origin;
    let read = |index: usize| event.data.get(index).copied().unwrap_or(0);
    // Linhas em coordenadas relativas ao painel; `write` soma a origem.
    let mut row = 20.0;
    let phase = (read(1).max(0) as usize).min(PHASES.len());
    if phase < PHASES.len() {
        let current = &PHASES[phase];
        write(host, event, &mut row, &tr("veio.board.phase", &[&(phase + 1), &PHASES.len(), &tr(&format!("veio.board.phase.{}", current.key), &[])]), presentation::INK);
        let done = read(2).clamp(0, current.count as i32);
        write(host, event, &mut row, &tr("veio.board.deliver", &[&done, &current.count, &tr(current.item_key, &[])]), presentation::SOFT_INK);
        host.fill(event.batch, [x + 10.0, y + row, 174.0, 6.0], BAR);
        if done > 0 {
            host.fill(event.batch, [x + 11.0, y + row + 1.0, 172.0 * done as f32 / current.count as f32, 4.0], 0xff3fae3f);
        }
        row += 9.0;
        // Licença: o que a fase abre. Nas outras fases, o que o quadro entrega.
        if current.reward.0.starts_with("veio:permit_") {
            let unlocks: Vec<String> = current.unlocks.iter().map(|key| tr(key, &[])).collect();
            write(host, event, &mut row, &tr("veio.board.unlocks", &[&unlocks.join(", ")]), presentation::SOFT_INK);
        } else {
            write(host, event, &mut row, &tr("veio.board.reward", &[&current.reward.1, &tr(current.reward_key, &[])]), presentation::SOFT_INK);
        }
        if phase == 0 {
            // Primeira hora: lista de montagem que se marca sozinha (verde = já há perto do quadro).
            let setup = read(11);
            let mut at = 10.0;
            for (bit, key) in ["veio.board.setup.power", "veio.board.setup.drill", "veio.board.setup.press", "veio.board.setup.assembler"].iter().enumerate() {
                let label = tr(key, &[]);
                let width = host.text_width(&label);
                if at > 10.0 && at + width > 184.0 { at = 10.0; row += 10.0; }
                host.text(event.batch, &label, [x + at, y + row], if setup & (1 << bit) != 0 { GREEN } else { MUTED }, false);
                at += width + 5.0;
            }
            row += 10.0;
        }
    } else {
        write(host, event, &mut row, &tr("veio.board.done", &[]), GREEN);
    }
    row = f32::max(row, 82.0) + 2.0;
    host.fill(event.batch, [x + 10.0, y + row - 3.0, 174.0, 1.0], 0xff8b8b8b);
    let order = ORDERS[(read(3).max(0) as usize).min(ORDERS.len() - 1)];
    let done = read(4).clamp(0, order.3 as i32);
    write(host, event, &mut row, &tr("veio.board.order", &[&read(6).max(1), &done, &order.3, &tr(order.2, &[])]), presentation::INK);
    let (key, color) = match read(5) {
        1 => ("veio.board.order_paid", GREEN),
        2 => ("veio.board.order_elsewhere", MUTED),
        _ => ("veio.board.order_reward", presentation::SOFT_INK),
    };
    write(host, event, &mut row, &tr(key, &[&order.4]), color);
    let (stuck, total) = (read(8) + read(9) + read(10), read(12));
    let status = if total == 0 { tr("veio.board.no_machines", &[]) }
        else if stuck == 0 { tr("veio.board.ok", &[&read(7), &total]) }
        else { tr("veio.board.issues", &[&read(8), &read(9), &read(10)]) };
    write(host, event, &mut row, &status, if stuck > 0 { RED } else if total == 0 { MUTED } else { GREEN });
}

#[cfg(test)]
mod tests;
