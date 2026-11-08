//! Comércio exterior, no estilo das conexões externas de Cities: Skylines.
//!
//! **O problema:** a vila é pequena. Sem o mundo lá fora, a indústria satura a demanda local, a moeda
//! fica presa (só cresce com esmeraldas depositadas) e os preços locais podem subir ou cair sem
//! limite. **A solução** tem quatro partes:
//!
//! 1. **Conexões externas** físicas. O *porto de carga* só opera sobre mar ou rio largo
//!    (`MIN_WATER` blocos de água em volta); a *estação de carga* só na fronteira, a pelo menos
//!    `FRONTIER` blocos da sede da companhia e das prefeituras. Cada uma tem capacidade por viagem,
//!    frete e horário: levar a carga até lá (esteiras, carregadores) é um problema de logística.
//! 2. **Bolsa mundial.** Cada produto tem uma cotação: preço base x complexidade (1x a 4x) x um índice
//!    que oscila dia a dia (determinístico, sem guardar estado) x saturação (o que você exportou nos
//!    últimos dias derruba o preço). A aba "Comércio exterior" do centro de controle mostra tudo.
//! 3. **Contratos e reputação** (como o elevador espacial). O mundo pede lotes com prazo e paga um
//!    prêmio. Cumprir sobe a reputação, que abre mercados de produtos mais complexos (camadas 2 a 4).
//! 4. **Importação automática.** Quando falta comida nas lojas e há conexão, o comerciante importa a
//!    preço mundial + frete + tarifa: a moeda sai da economia (a tarifa fica na prefeitura). Com
//!    conexão, o preço local nunca passa da paridade de importação (teto); abaixo da paridade de
//!    exportação (piso), compensa mais exportar. É isso que segura a inflação.
//!
//! A moeda de fora entra e sai pelas bordas do `ledger`, então a identidade contábil continua valendo.
use super::*;
use ledger::Book;

pub(crate) const PORT: &str = "veio:export_dock";
pub(crate) const STATION: &str = "veio:freight_station";
pub(crate) const SLOTS: usize = 9;
pub(crate) const MIN_WATER: i32 = 20;
pub(crate) const FRONTIER: i32 = 96;
/// Importação: preço mundial + 20% de frete + 10% de tarifa (da prefeitura).
pub(crate) const IMPORT_FREIGHT: i32 = 20;
pub(crate) const IMPORT_TARIFF: i32 = 10;
/// Estados do painel das conexões.
pub(crate) const NO_WATER: i32 = 33;
pub(crate) const NOT_FRONTIER: i32 = 34;
pub(crate) const NO_COMPANY: i32 = 32;
const MAX_CONTRACTS: usize = 3;

/// Produtos do mercado mundial: (item, chave de nome, valor base em moedas, camada de complexidade).
pub(crate) const PRODUCTS: [(&str, &str, i32, i32); 34] = [
    ("minecraft:raw_iron", "item.minecraft.raw_iron", 2, 0),
    ("minecraft:raw_copper", "item.minecraft.raw_copper", 1, 0),
    ("minecraft:raw_gold", "item.minecraft.raw_gold", 4, 0),
    ("minecraft:coal", "item.minecraft.coal", 2, 0),
    ("minecraft:quartz", "item.minecraft.quartz", 2, 0),
    ("minecraft:redstone", "item.minecraft.redstone", 2, 0),
    ("minecraft:diamond", "item.minecraft.diamond", 40, 0),
    ("minecraft:oak_log", "block.minecraft.oak_log", 1, 0),
    ("minecraft:spruce_log", "block.minecraft.spruce_log", 1, 0),
    ("minecraft:birch_log", "block.minecraft.birch_log", 1, 0),
    ("minecraft:wheat", "item.minecraft.wheat", 1, 0),
    ("minecraft:potato", "item.minecraft.potato", 1, 0),
    ("minecraft:carrot", "item.minecraft.carrot", 1, 0),
    ("minecraft:leather", "item.minecraft.leather", 2, 0),
    ("minecraft:white_wool", "block.minecraft.white_wool", 1, 1),
    ("minecraft:iron_ingot", "item.minecraft.iron_ingot", 3, 1),
    ("minecraft:copper_ingot", "item.minecraft.copper_ingot", 2, 1),
    ("minecraft:gold_ingot", "item.minecraft.gold_ingot", 6, 1),
    ("minecraft:bread", "item.minecraft.bread", 6, 1),
    ("minecraft:cooked_beef", "item.minecraft.cooked_beef", 8, 1),
    ("minecraft:paper", "item.minecraft.paper", 1, 1),
    ("minecraft:book", "item.minecraft.book", 4, 1),
    ("veio:iron_plate", "item.veio.iron_plate", 4, 1),
    ("veio:copper_plate", "item.veio.copper_plate", 3, 1),
    ("veio:flour", "item.veio.flour", 1, 1),
    ("veio:gear", "item.veio.gear", 7, 2),
    ("veio:wire", "item.veio.wire", 4, 2),
    ("veio:frame", "item.veio.frame", 8, 2),
    ("veio:silicon", "item.veio.silicon", 5, 2),
    ("veio:steel_plate", "item.veio.steel_plate", 9, 2),
    ("veio:circuit", "item.veio.circuit", 14, 3),
    ("veio:module", "item.veio.module", 20, 3),
    ("veio:repair_kit", "item.veio.repair_kit", 12, 3),
    ("veio:speed_module", "item.veio.speed_module", 40, 4),
];

pub(crate) fn product(item: &str) -> Option<usize> {
    PRODUCTS.iter().position(|(known, _, _, _)| *known == item)
}

/// Fator da camada de complexidade, em %.
fn tier_factor(tier: i32) -> i32 {
    match tier { 0 => 100, 1 => 150, 2 => 200, 3 => 300, _ => 400 }
}

/// Camada mais alta que o mundo compra com esta reputação.
pub(crate) fn unlocked_tier(reputation: i32) -> i32 {
    match reputation { ..=0 => 1, 1..=2 => 2, 3..=5 => 3, _ => 4 }
}

/// Índice da bolsa no dia, em %: oscila entre 80 e 120, igual para todos (determinístico).
pub(crate) fn index(product: usize, day: i64) -> i32 {
    let mut n = (product as u64 + 1).wrapping_mul(0x9E37_79B9_7F4A_7C15) ^ (day as u64).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    n ^= n >> 29;
    n = n.wrapping_mul(0x94D0_49BB_1331_11EB);
    n ^= n >> 32;
    80 + (n % 41) as i32
}

/// Unidades que o mundo absorve por dia a preço cheio (matéria-prima satura antes).
fn absorb(tier: i32) -> i32 {
    if tier == 0 { 32 } else { 64 }
}

/// Estado do comércio exterior da dimensão: reputação, contratos e exportações recentes.
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct Contract {
    pub product: usize,
    pub need: i32,
    pub done: i32,
    pub due: i64,
}

#[derive(Clone, Debug, Default)]
struct Trade {
    loaded: bool,
    reputation: i32,
    issued: i32,
    contracts: Vec<Contract>,
    /// Exportações recentes por produto (média de alguns dias) e as do dia corrente.
    recent: Vec<i32>,
    today: Vec<i32>,
    day: i64,
    /// Exportação e importação do turno e do dia, em moedas.
    exports_turn: i32,
    imports_turn: i32,
    exports_day: i32,
    imports_day: i32,
    rolled: i64,
}

static TRADES: Mutex<Vec<(String, Trade)>> = Mutex::new(Vec::new());
/// Conexões externas operando: (dimensão, posição, relógio).
static CONNECTIONS: Mutex<Vec<(String, [i32; 3], u64)>> = Mutex::new(Vec::new());

fn with_trade<T>(dimension: &str, change: impl FnOnce(&mut Trade) -> T) -> T {
    let mut trades = TRADES.lock().unwrap_or_else(|poison| poison.into_inner());
    let index = match trades.iter().position(|(name, _)| name == dimension) {
        Some(index) => index,
        None => {
            // Dia e turno inválidos: o primeiro dia sempre emite contratos.
            trades.push((dimension.to_owned(), Trade { day: -1, rolled: -1, ..Trade::default() }));
            trades.len() - 1
        }
    };
    let trade = &mut trades[index].1;
    if trade.recent.len() != PRODUCTS.len() {
        trade.recent.resize(PRODUCTS.len(), 0);
        trade.today.resize(PRODUCTS.len(), 0);
    }
    change(trade)
}

/// Vira o dia: as exportações do dia entram na média (os mercados se recuperam devagar), os
/// contratos vencidos caem (reputação -1) e novos são emitidos.
fn new_day(trade: &mut Trade, day: i64) {
    if trade.day == day {
        return;
    }
    for product in 0..PRODUCTS.len() {
        trade.recent[product] = (trade.recent[product] * 6 + trade.today[product] * 4) / 10;
        trade.today[product] = 0;
    }
    trade.day = day;
    let before = trade.contracts.len();
    trade.contracts.retain(|contract| contract.due >= day);
    trade.reputation = (trade.reputation - (before - trade.contracts.len()) as i32).max(0);
    while trade.contracts.len() < MAX_CONTRACTS {
        let contract = issue(trade.reputation, trade.issued, day);
        trade.issued += 1;
        trade.contracts.push(contract);
    }
}

/// Um contrato novo: produto da camada mais alta liberada (às vezes uma abaixo), lote e prazo.
pub(crate) fn issue(reputation: i32, number: i32, day: i64) -> Contract {
    let top = unlocked_tier(reputation);
    let tier = if number % 3 == 2 { (top - 1).max(0) } else { top };
    let options: Vec<usize> = (0..PRODUCTS.len()).filter(|index| PRODUCTS[*index].3 == tier).collect();
    let pick = options[(number as usize * 7 + day as usize) % options.len()];
    let need = (256 / (tier + 1) / 16 * 16).max(16) + (number % 4) * 16;
    Contract { product: pick, need, done: 0, due: day + 3 + i64::from(number % 2) }
}

/// Cotação de uma unidade agora, em moedas, ou `None` se o mundo ainda não compra o produto.
pub(crate) fn quote(dimension: &str, item: &str, day: i64) -> Option<i32> {
    let product = product(item)?;
    let (_, _, base, tier) = PRODUCTS[product];
    with_trade(dimension, |trade| {
        if tier > unlocked_tier(trade.reputation) {
            return None;
        }
        let pressure = trade.recent[product] + trade.today[product];
        let full = base * tier_factor(tier) / 100 * index(product, day) / 100;
        Some((full * absorb(tier) / (absorb(tier) + pressure)).max(1))
    })
}

/// Paridade de importação: o que custa trazer uma unidade de fora (preço mundial + frete + tarifa).
pub(crate) fn import_price(item: &str, day: i64) -> Option<i32> {
    let product = product(item)?;
    let (_, _, base, tier) = PRODUCTS[product];
    let world = base * tier_factor(tier) / 100 * index(product, day) / 100;
    Some((world * (100 + IMPORT_FREIGHT + IMPORT_TARIFF) / 100).max(1))
}

/// Há conexão externa operando nesta dimensão?
pub(crate) fn connected(dimension: &str, now: u64) -> bool {
    CONNECTIONS.lock().is_ok_and(|all| all.iter().any(|(dim, _, seen)| dim == dimension && now.saturating_sub(*seen) <= STALE))
}

/// Exporta `count` unidades: cotação do momento (cada unidade empurra o preço), contratos
/// primeiro (com prêmio de 50%), imposto na fonte, moeda nova na companhia. Devolve o bruto.
pub(crate) fn export(dimension: &str, at: [i32; 3], item: &str, count: u32, day: i64, now: u64) -> i32 {
    let Some(product) = product(item) else { return 0 };
    if count == 0 || !ledger::company_open(dimension, now) || quote(dimension, item, day).is_none() {
        return 0;
    }
    let mut gross = 0;
    let mut bonus = 0;
    for _ in 0..count {
        let price = quote(dimension, item, day).unwrap_or(1);
        gross += price;
        with_trade(dimension, |trade| {
            trade.today[product] += 1;
            if let Some(contract) = trade.contracts.iter_mut().find(|contract| contract.product == product && contract.done < contract.need) {
                contract.done += 1;
                bonus += price / 2;
            }
        });
    }
    // Contratos cumpridos: reputação +1 e saem da lista (o próximo vem no dia seguinte).
    with_trade(dimension, |trade| {
        let before = trade.contracts.len();
        trade.contracts.retain(|contract| contract.done < contract.need);
        trade.reputation += (before - trade.contracts.len()) as i32;
        trade.exports_turn += gross + bonus;
    });
    let total = gross + bonus;
    ledger::border(dimension, total);
    ledger::exported(dimension, total);
    let tax = ledger::withhold(dimension, at, total, now);
    ledger::put(Book::Company, dimension, at, ledger::ANYWHERE, total - tax, ledger::RECEIVED, now);
    super::super::super::holding::book(dimension, super::super::super::holding::item_division(item), total, tax);
    super::super::super::holding::exported(dimension, total);
    ledger::record(dimension, at, total, ledger::EXPORTS, now);
    total
}

/// Importa `count` unidades para a loja em `shop`, pagas pelo caixa do comerciante: a tarifa fica na
/// prefeitura, o resto (preço mundial + frete) sai da economia. Devolve quantas vieram.
pub(crate) fn import(dimension: &str, shop: [i32; 3], item: &str, count: u32, day: i64, now: u64) -> u32 {
    let Some(unit) = import_price(item, day) else { return 0 };
    if !connected(dimension, now) {
        return 0;
    }
    let cash = ledger::balance(Book::Shop, dimension, shop).unwrap_or(0);
    let units = count.min((cash / unit).max(0) as u32);
    if units == 0 {
        return 0;
    }
    let paid = ledger::take(Book::Shop, dimension, shop, 0, unit * units as i32, ledger::REDEEM, now);
    let tariff = paid * IMPORT_TARIFF / (100 + IMPORT_FREIGHT + IMPORT_TARIFF);
    // A tarifa fica na prefeitura que governa a loja; sem prefeitura, sai com o resto.
    let tariff = match governing(dimension, shop, now) {
        Some(hall) if ledger::put(Book::Public, dimension, hall.pos, 0, tariff, ledger::TAXES, now) => tariff,
        _ => 0,
    };
    ledger::border(dimension, -(paid - tariff));
    with_trade(dimension, |trade| trade.imports_turn += paid);
    units
}

/// Fecha o turno do comércio exterior (exportação e importação do dia).
pub(crate) fn roll(dimension: &str, time: i64) {
    let turn = time.div_euclid(TURN);
    with_trade(dimension, |trade| {
        new_day(trade, time.div_euclid(DAY));
        if trade.rolled == turn {
            return;
        }
        trade.rolled = turn;
        trade.exports_day = ledger::daily(trade.exports_day, trade.exports_turn);
        trade.imports_day = ledger::daily(trade.imports_day, trade.imports_turn);
        trade.exports_turn = 0;
        trade.imports_turn = 0;
    });
}

/// Carrega (uma vez) e guarda no centro de controle a reputação e os contratos.
pub(crate) fn persist(dimension: &str, entity: &mut ModBlockEntity, time: i64) {
    with_trade(dimension, |trade| {
        if !trade.loaded {
            trade.loaded = true;
            if let Some(Tag::IntArray(values)) = entity.nbt.get(nbt_key("Trade"))
                && values.len() >= 2
            {
                trade.reputation = values[0];
                trade.issued = values[1];
                trade.contracts = values[2..].chunks_exact(4)
                    .filter(|chunk| (chunk[0] as usize) < PRODUCTS.len())
                    .map(|chunk| Contract { product: chunk[0] as usize, need: chunk[1], done: chunk[2], due: i64::from(chunk[3]) })
                    .collect();
                // O dia salvo não é guardado: hoje recompleta os contratos que faltarem.
            }
        }
        new_day(trade, time.div_euclid(DAY));
        let mut values = vec![trade.reputation, trade.issued];
        for contract in &trade.contracts {
            values.extend([contract.product as i32, contract.need, contract.done, contract.due as i32]);
        }
        entity.nbt.insert("Trade".into(), Tag::IntArray(values));
    });
}

/// Números da aba "Comércio exterior": [reputação, camada, exporta/dia, importa/dia] e até três
/// contratos (produto, feito, pedido, dias).
pub(crate) fn summary(dimension: &str, day: i64) -> ([i32; 4], Vec<[i32; 4]>) {
    with_trade(dimension, |trade| {
        ([trade.reputation, unlocked_tier(trade.reputation), trade.exports_day, trade.imports_day],
            trade.contracts.iter().map(|contract| [contract.product as i32, contract.done, contract.need,
                (contract.due - day).max(0) as i32]).collect())
    })
}

/// Reputação e contratos (testes).
#[cfg(test)]
pub(crate) fn state(dimension: &str) -> (i32, Vec<Contract>) {
    with_trade(dimension, |trade| (trade.reputation, trade.contracts.clone()))
}

#[cfg(test)]
pub(crate) fn set_reputation(dimension: &str, reputation: i32) {
    with_trade(dimension, |trade| trade.reputation = reputation);
}

// ---------- Conexões externas ----------

/// Capacidade (pilhas por viagem), turnos entre viagens e frete (base, por pilha).
fn schedule(block: &str) -> (usize, i64, i32, i32) {
    if block == PORT { (SLOTS, 2, 4, 1) } else { (6, 1, 2, 2) }
}

/// Blocos de água em volta (7x7, no nível do bloco e abaixo).
fn water_around(host: &dyn Host, pos: [i32; 3]) -> i32 {
    let mut water = 0;
    for dx in -3..=3 {
        for dz in -3..=3 {
            for dy in [-1, 0] {
                water += i32::from(host.block([pos[0] + dx, pos[1] + dy, pos[2] + dz]) == "minecraft:water");
            }
        }
    }
    water
}

/// A estação está na fronteira: longe da sede da companhia e de toda prefeitura.
fn at_frontier(dimension: &str, pos: [i32; 3], now: u64) -> bool {
    let far = |at: [i32; 3]| (at[0] - pos[0]).abs().max((at[2] - pos[2]).abs()) >= FRONTIER;
    let halls_far = HALLS.lock().is_ok_and(|halls| halls.iter()
        .filter(|hall| hall.dimension == dimension && now.saturating_sub(hall.seen) <= STALE).all(|hall| far(hall.pos)));
    halls_far && ledger::company_positions(dimension, now).into_iter().all(far)
}

/// Tick de uma conexão externa: confere o lugar, registra-se e despacha no horário.
pub(crate) fn tick(host: &mut dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity, time: i64) {
    let phase = (time + i64::from((pos[0] ^ pos[2]) & 63)).rem_euclid(TURN);
    if phase % 20 != 0 {
        return;
    }
    let dimension = host.dimension();
    let now = defense::clock();
    let (capacity, every, freight_base, freight_stack) = schedule(&entity.id);
    // O lugar: água (porto, conferida a cada dia) ou fronteira (estação).
    let place_ok = if entity.id == PORT {
        if !entity.nbt.contains_key(nbt_key("Water")) || time.rem_euclid(DAY) < 20 {
            set_number(entity, "Water", water_around(host, pos));
        }
        number(entity, "Water") >= MIN_WATER
    } else {
        at_frontier(dimension, pos, now)
    };
    let company = ledger::company_open(dimension, now);
    let turn = time.div_euclid(TURN);
    let departs_in = (every - turn.rem_euclid(every) - 1) * TURN + (TURN - phase);
    set_number(entity, "Info", (departs_in / 20) as i32);
    set_number(entity, "Utilization", freight_base + freight_stack * capacity as i32);
    if !place_ok || !company {
        set_number(entity, "Status", if !company { NO_COMPANY } else if entity.id == PORT { NO_WATER } else { NOT_FRONTIER });
        return;
    }
    upsert(&CONNECTIONS, (dimension.to_owned(), pos, now), |(dim, at, _)| dim == dimension && *at == pos,
        |(_, _, seen)| now.saturating_sub(*seen) <= STALE);
    let today = time.div_euclid(DAY) as i32;
    if number(entity, "Day") != today {
        set_number(entity, "Day", today);
        set_number(entity, "Rate", 0);
        set_number(entity, "Total", 0);
    }
    let loaded = entity.items.iter().flatten().any(|stack| product(&stack.item).is_some());
    if phase != 0 || turn.rem_euclid(every) != 0 {
        set_number(entity, "Status", if loaded { STORING } else { ZONE_ACTIVE });
        return;
    }
    // Partida: até `capacity` pilhas que o mundo compra; o frete sai da companhia (para fora).
    let day = time.div_euclid(DAY);
    let mut stacks = 0;
    let mut units = 0;
    let mut coins = 0;
    for slot in 0..entity.items.len().min(SLOTS) {
        if stacks >= capacity {
            break;
        }
        let Some(stack) = entity.items[slot].as_ref().filter(|stack| stack.extra.is_none() && stack.damage == 0) else { continue };
        let (item, count) = (stack.item.to_string(), stack.count);
        let gross = export(dimension, pos, &item, count, day, now);
        if gross > 0 {
            entity.items[slot] = None;
            stacks += 1;
            units += count as i32;
            coins += gross;
        }
    }
    if stacks > 0 {
        let freight = freight_base + freight_stack * stacks as i32;
        let paid = ledger::take(Book::Company, dimension, pos, ledger::ANYWHERE, freight, ledger::REDEEM, now);
        ledger::border(dimension, -paid);
        set_number(entity, "Rate", number(entity, "Rate").saturating_add(units));
        set_number(entity, "Total", number(entity, "Total").saturating_add(coins - paid));
        let sound = if entity.id == PORT { "minecraft:block.bell.use" } else { "minecraft:entity.minecart.riding" };
        host.play_sound(sound, center(pos), 4, 0.8, 0.7);
        host.send_particles(&ParticleOptions::simple(ParticleType::HappyVillager), None,
            [f64::from(pos[0]) + 0.5, f64::from(pos[1]) + 1.2, f64::from(pos[2]) + 0.5], [0.4, 0.3, 0.4], 0.0, 6);
    }
    set_number(entity, "Status", if stacks > 0 { 1 } else { ZONE_ACTIVE });
}
