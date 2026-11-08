//! Comércio exterior: o porto de exportação vende ao mercado externo.
//!
//! - **Porto** (`DOCK`, empresa de logística): 9 espaços que aceitam qualquer item com preço
//!   mundial. A cada turno (1 min) sai um navio e vende tudo o que está nele.
//! - **Preço mundial** = valor do item x fator da complexidade (`tier`): matéria-prima 1x,
//!   processado 1,5x, componente 2x, avançado 3x, módulo 4x. Quanto mais elaborado, mais vale.
//! - **Saturação:** o mercado externo absorve `ABSORB` unidades de cada item por dia ao preço cheio;
//!   depois, cada unidade vale menos (`absorb / (absorb + vendidas hoje)`). Matéria-prima satura
//!   na metade. Exportar variado e com valor agregado rende mais.
//! - **Moeda de fora:** a venda entra na conta da companhia como moeda nova (borda de entrada, como a
//!   esmeralda minerada que você deposita), com o imposto da fase retido na fonte. A receita vai para
//!   a empresa da holding que faz o item.
//!
//! Outras frentes (praça do mercado, missões, novos produtos) usam `world_price` e `sell`.
use super::*;
use ledger::Book;
use std::collections::HashMap;

pub(crate) const DOCK: &str = "veio:export_dock";
pub(crate) const DOCK_SLOTS: usize = 9;
/// Unidades de cada item que o mercado externo absorve por dia ao preço cheio.
pub(crate) const ABSORB: u32 = 64;
/// Estado do painel: falta o cofre da companhia (centro de controle) para receber.
pub(crate) const NO_COMPANY: i32 = 32;

/// Valor base de exportação (moedas por unidade) e a camada de complexidade (0 a 4).
fn table(item: &str) -> Option<(i32, i32)> {
    let name = item.split(':').nth(1).unwrap_or(item);
    let veio = item.starts_with("veio:");
    Some(match (veio, name) {
        // Avançado e módulos.
        (true, "speed_module" | "efficiency_module" | "productivity_module") => (40, 4),
        (true, "module") => (20, 3),
        (true, "circuit") => (14, 3),
        (true, "repair_kit") => (12, 3),
        // Componentes.
        (true, "steel_plate") => (9, 2),
        (true, "gear") => (7, 2),
        (true, "frame") => (8, 2),
        (true, "wire") => (4, 2),
        (true, "silicon") => (5, 2),
        // Processados.
        (true, "iron_plate") => (4, 1),
        (true, "copper_plate") => (3, 1),
        (true, "flour") => (1, 1),
        (false, "iron_ingot") => (3, 1),
        (false, "copper_ingot") => (2, 1),
        (false, "gold_ingot") => (6, 1),
        (false, "glass" | "paper" | "white_wool" | "charcoal") => (1, 1),
        (false, "book" | "lantern" | "candle") => (4, 1),
        // Matéria-prima.
        (false, "raw_iron") => (2, 0),
        (false, "raw_copper") => (1, 0),
        (false, "raw_gold") => (4, 0),
        (false, "coal" | "redstone" | "lapis_lazuli" | "quartz" | "leather") => (2, 0),
        (false, "diamond") => (40, 0),
        (false, "wheat" | "potato" | "carrot" | "beetroot" | "sugar_cane" | "feather" | "egg") => (1, 0),
        _ if name.ends_with("_log") => (1, 0),
        // Comida pronta: o valor de comida, como processado.
        _ => {
            let value = production::pantry::value(item)? as i32;
            (value.max(1), 1)
        }
    })
}

/// Fator da camada de complexidade, em %.
fn tier_factor(tier: i32) -> i32 {
    match tier { 0 => 100, 1 => 150, 2 => 200, 3 => 300, _ => 400 }
}

/// Preço mundial cheio de uma unidade, em moedas.
pub(crate) fn world_price(item: &str) -> Option<i32> {
    // A esmeralda é a moeda: não se exporta.
    if item == WAGE {
        return None;
    }
    table(item).map(|(value, tier)| (value * tier_factor(tier) / 100).max(1))
}

pub(crate) fn exportable(item: &str) -> bool {
    world_price(item).is_some()
}

/// Vendido hoje ao mercado externo, por dimensão e item: (dia, item -> unidades).
static WORLD: Mutex<Vec<(String, i64, HashMap<String, u32>)>> = Mutex::new(Vec::new());

/// Quanto rendem `count` unidades agora, já com a saturação do dia. Anota a venda.
fn quote_and_sell(dimension: &str, item: &str, count: u32, day: i64) -> i32 {
    let Some(price) = world_price(item) else { return 0 };
    let absorb = if table(item).is_some_and(|(_, tier)| tier == 0) { ABSORB / 2 } else { ABSORB };
    let Ok(mut world) = WORLD.lock() else { return 0 };
    let index = match world.iter().position(|(name, _, _)| name == dimension) {
        Some(index) => index,
        None => { world.push((dimension.to_owned(), day, HashMap::new())); world.len() - 1 }
    };
    let entry = &mut world[index];
    if entry.1 != day {
        entry.1 = day;
        entry.2.clear();
    }
    let sold = entry.2.entry(item.to_owned()).or_insert(0);
    let mut total = 0i64;
    for _ in 0..count {
        total += i64::from(price) * i64::from(absorb) * 100 / i64::from(absorb + *sold);
        *sold += 1;
    }
    (total / 100) as i32
}

/// Preço de uma unidade agora (com a saturação do dia), sem vender. Para painéis e relatórios.
pub(crate) fn quote(dimension: &str, item: &str, day: i64) -> Option<i32> {
    let price = world_price(item)?;
    let absorb = if table(item).is_some_and(|(_, tier)| tier == 0) { ABSORB / 2 } else { ABSORB };
    let sold = WORLD.lock().ok().and_then(|world| world.iter().find(|(name, at, _)| name == dimension && *at == day)
        .and_then(|(_, _, sold)| sold.get(item).copied())).unwrap_or(0);
    Some((price * absorb as i32 / (absorb + sold) as i32).max(1))
}

/// Vende ao mercado externo em `at`: imposto na fonte para a prefeitura, o resto para a companhia,
/// receita na empresa do item. Devolve o bruto (0 sem conta da companhia: nada é vendido).
pub(crate) fn sell(dimension: &str, at: [i32; 3], item: &str, count: u32, time: i64, now: u64) -> i32 {
    if count == 0 || !ledger::company_open(dimension, now) {
        return 0;
    }
    let gross = quote_and_sell(dimension, item, count, time.div_euclid(DAY));
    if gross <= 0 {
        return 0;
    }
    // Moeda de fora entra na economia.
    ledger::border(dimension, gross);
    ledger::exported(dimension, gross);
    let tax = ledger::withhold(dimension, at, gross, now);
    ledger::put(Book::Company, dimension, at, ledger::ANYWHERE, gross - tax, ledger::RECEIVED, now);
    let division = super::super::super::holding::item_division(item);
    super::super::super::holding::book(dimension, division, gross, tax);
    super::super::super::holding::exported(dimension, gross);
    ledger::record(dimension, at, gross, ledger::EXPORTS, now);
    gross
}

/// Tick do porto: a cada turno sai um navio com tudo o que está nele.
pub(crate) fn tick(host: &mut dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity, time: i64) {
    let phase = (time + i64::from((pos[0] ^ pos[2]) & 63)).rem_euclid(TURN);
    if phase % 20 != 0 {
        return;
    }
    let dimension = host.dimension();
    let now = defense::clock();
    let today = time.div_euclid(DAY) as i32;
    if number(entity, "Day") != today {
        set_number(entity, "Day", today);
        set_number(entity, "Rate", 0);
        set_number(entity, "Total", 0);
    }
    let loaded = entity.items.iter().flatten().any(|stack| exportable(&stack.item));
    // Segundos até o próximo navio.
    set_number(entity, "Info", ((TURN - phase) / 20) as i32);
    if phase != 0 {
        set_number(entity, "Status", if !ledger::company_open(dimension, now) { NO_COMPANY } else if loaded { STORING } else { 0 });
        return;
    }
    let mut units = 0;
    let mut coins = 0;
    for slot in 0..entity.items.len().min(DOCK_SLOTS) {
        let Some(stack) = entity.items[slot].as_ref().filter(|stack| stack.extra.is_none() && stack.damage == 0) else { continue };
        let item = stack.item.to_string();
        let count = stack.count;
        let gross = sell(dimension, pos, &item, count, time, now);
        if gross > 0 {
            entity.items[slot] = None;
            units += count as i32;
            coins += gross;
        }
    }
    if units > 0 {
        set_number(entity, "Rate", number(entity, "Rate").saturating_add(units));
        set_number(entity, "Total", number(entity, "Total").saturating_add(coins));
        host.play_sound("minecraft:block.bell.use", center(pos), 4, 0.8, 0.7);
        host.send_particles(&ParticleOptions::simple(ParticleType::HappyVillager), None,
            [f64::from(pos[0]) + 0.5, f64::from(pos[1]) + 1.2, f64::from(pos[2]) + 0.5], [0.4, 0.3, 0.4], 0.0, 6);
    }
    set_number(entity, "Status", if !ledger::company_open(dimension, now) { NO_COMPANY } else if units > 0 { 1 } else { 0 });
}
