//! Comércio exterior, no estilo das conexões externas de Cities: Skylines.
//!
//! **O problema:** a vila é pequena. Sem o mundo lá fora, a indústria satura a demanda local, a moeda
//! fica presa (só cresce com esmeraldas depositadas, inclusive as mineradas) e os preços locais podem
//! subir ou cair sem limite. **A solução** tem quatro partes:
//!
//! 1. **Conexões externas** físicas. O *porto de carga* só opera sobre mar ou rio largo
//!    (`MIN_WATER` blocos de água em volta); a *estação de carga* só na fronteira, a pelo menos
//!    `FRONTIER` blocos da sede da companhia, das prefeituras e do centro da vila. Cada uma tem
//!    capacidade por viagem, frete e horário: levar a carga até lá é um problema de logística.
//! 2. **Bolsa mundial.** Cotação = preço base x complexidade (1x a 4x) x índice do dia
//!    (determinístico, 80% a 120%) x saturação (o que você exportou nos últimos dias derruba o preço;
//!    o mercado se recupera em alguns dias).
//! 3. **Contratos e reputação** (como o elevador espacial). O mundo pede lotes com prazo e paga 50% a
//!    mais. Cumprir sobe a reputação, que abre mercados de produtos mais complexos (camadas 2 a 4).
//! 4. **Importação automática.** Quando falta comida nas lojas e há conexão, o comerciante importa a
//!    comida que mais vende, a preço mundial + frete + tarifa: a moeda sai da economia (a tarifa fica
//!    na prefeitura). Com conexão, o preço local nunca passa da paridade de importação (teto); abaixo
//!    da paridade de exportação (piso), compensa mais exportar. É isso que segura a inflação.
//!
//! A moeda de fora entra e sai pelas bordas do `ledger`, então a identidade contábil continua valendo.
//! Os acontecimentos (contratos, reputação) viram avisos no chat pelo centro de controle (`drain`).
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
/// Prêmio dos contratos sobre a cotação, em %.
pub(crate) const PREMIUM: i32 = 50;
/// Estados do painel das conexões.
pub(crate) const NO_COMPANY: i32 = 32;
pub(crate) const NO_WATER: i32 = 33;
pub(crate) const NOT_FRONTIER: i32 = 34;
pub(crate) const LOCKED_MARKET: i32 = 35;
const MAX_CONTRACTS: usize = 3;
/// Reputação a partir da qual cada camada abre (componentes, eletrônicos, módulos).
pub(crate) const TIER_AT: [i32; 3] = [1, 3, 6];

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

/// Comida que as lojas podem importar (a que a loja mais vende, ou pão).
pub(crate) const STAPLE: &str = "minecraft:bread";

pub(crate) fn product(item: &str) -> Option<usize> {
    PRODUCTS.iter().position(|(known, _, _, _)| *known == item)
}

/// Fator da camada de complexidade, em %.
fn tier_factor(tier: i32) -> i32 {
    match tier { 0 => 100, 1 => 150, 2 => 200, 3 => 300, _ => 400 }
}

/// Camada mais alta que o mundo compra com esta reputação.
pub(crate) fn unlocked_tier(reputation: i32) -> i32 {
    1 + TIER_AT.iter().filter(|at| reputation >= **at).count() as i32
}

/// Reputação que abre a próxima camada, ou `None` se todas estão abertas.
pub(crate) fn next_unlock(reputation: i32) -> Option<i32> {
    TIER_AT.iter().copied().find(|at| reputation < *at)
}

/// Índice da bolsa no dia, em %: oscila entre 80 e 120, igual para todos (determinístico).
pub(crate) fn index(product: usize, day: i64) -> i32 {
    let mut n = (product as u64 + 1).wrapping_mul(0x9E37_79B9_7F4A_7C15) ^ (day as u64).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    n ^= n >> 29;
    n = n.wrapping_mul(0x94D0_49BB_1331_11EB);
    n ^= n >> 32;
    80 + (n % 41) as i32
}

/// Valor de referência de um produto (índice 100%), em centésimos de moeda. É também a âncora do
/// preço local nas lojas: o mesmo bem vale o mesmo dentro e fora, antes de oferta e procura.
pub(crate) fn reference_cents(product: usize) -> i32 {
    let (_, _, base, tier) = PRODUCTS[product];
    base * tier_factor(tier)
}

/// Preço mundial cheio do dia (sem saturação), em centésimos de moeda.
pub(crate) fn world_cents(product: usize, day: i64) -> i32 {
    reference_cents(product) * index(product, day) / 100
}

/// Unidades por dia que o mundo absorve antes de o preço cair à metade. Volumes maiores para
/// matéria-prima e processados, menores para eletrônicos e módulos (mercados de nicho).
pub(crate) fn absorb(tier: i32) -> i32 {
    match tier { 0 => 256, 1 | 2 => 512, 3 => 256, _ => 128 }
}

/// Pressão sobre o mercado à vista: metade da média recente mais o que já foi vendido hoje.
fn pressure(trade: &Trade, product: usize) -> i32 {
    trade.recent[product] / 2 + trade.today[product]
}

/// Piso da saturação: o mundo sempre paga pelo menos isto do preço cheio, em % (valor de sucata).
pub(crate) const SCRAP: i32 = 25;

/// Preço de uma unidade, em centésimos, com `pressure` unidades já despejadas no mercado à vista.
pub(crate) fn spot_cents(product: usize, day: i64, pressure: i32) -> i32 {
    let tier = PRODUCTS[product].3;
    let full = world_cents(product, day);
    (full * absorb(tier) / (absorb(tier) + pressure.max(0))).max(full * SCRAP / 100)
}

fn saturated(product: usize, day: i64, pressure: i32) -> i32 {
    (spot_cents(product, day, pressure) / 100).max(1)
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct Contract {
    pub product: usize,
    pub need: i32,
    pub done: i32,
    pub due: i64,
}

/// Acontecimentos para avisar no chat.
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum News {
    Offered(usize, i32, i64),
    Fulfilled(usize, i32),
    Lost(usize),
    Opened(i32),
}

#[derive(Clone, Debug, Default)]
struct Trade {
    loaded: bool,
    reputation: i32,
    /// Camada mais alta já aberta: mercado conquistado não fecha por um contrato perdido.
    best: i32,
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
    news: Vec<News>,
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

/// Camada aberta agora: a da reputação, nunca abaixo da melhor já conquistada.
fn open_tier(trade: &Trade) -> i32 {
    unlocked_tier(trade.reputation).max(trade.best)
}

/// Muda a reputação e anota quando uma camada nova abre (a camada aberta nunca volta).
fn reputation(trade: &mut Trade, delta: i32) {
    let before = open_tier(trade);
    // Guarda a camada aberta antes de mexer: cair de reputação não fecha mercado.
    trade.best = trade.best.max(before);
    trade.reputation = (trade.reputation + delta).max(0);
    trade.best = trade.best.max(open_tier(trade));
    if trade.best > before {
        trade.news.push(News::Opened(trade.best));
    }
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
    let lost: Vec<usize> = trade.contracts.iter().filter(|contract| contract.due < day).map(|contract| contract.product).collect();
    trade.contracts.retain(|contract| contract.due >= day);
    for product in &lost {
        trade.news.push(News::Lost(*product));
    }
    reputation(trade, -(lost.len() as i32));
    while trade.contracts.len() < MAX_CONTRACTS {
        let taken: Vec<usize> = trade.contracts.iter().map(|contract| contract.product).collect();
        let contract = issue(open_tier(trade), trade.issued, day, &taken);
        trade.issued += 1;
        trade.news.push(News::Offered(contract.product, contract.need, contract.due - day));
        trade.contracts.push(contract);
    }
}

/// Um contrato novo: produto da camada mais alta liberada (um em três, uma abaixo), que ainda não
/// esteja em outro contrato; lote menor quanto mais complexo; prazo de 3 a 4 dias.
pub(crate) fn issue(top: i32, number: i32, day: i64, taken: &[usize]) -> Contract {
    let tier = if number % 3 == 2 { (top - 1).max(0) } else { top };
    let mut options: Vec<usize> = (0..PRODUCTS.len()).filter(|index| PRODUCTS[*index].3 == tier && !taken.contains(index)).collect();
    if options.is_empty() {
        options = (0..PRODUCTS.len()).filter(|index| PRODUCTS[*index].3 <= top && !taken.contains(index)).collect();
    }
    let pick = options[(number.unsigned_abs() as usize * 7 + day.unsigned_abs() as usize) % options.len()];
    let need = (256 / (tier + 1) / 16 * 16).max(16) + (number % 4) * 16;
    Contract { product: pick, need, done: 0, due: day + 3 + i64::from(number % 2) }
}

/// Cotação de uma unidade agora, em moedas, ou `None` se o mundo ainda não compra o produto.
pub(crate) fn quote(dimension: &str, item: &str, day: i64) -> Option<i32> {
    let product = product(item)?;
    with_trade(dimension, |trade| {
        (PRODUCTS[product].3 <= open_tier(trade))
            .then(|| saturated(product, day, pressure(trade, product)))
    })
}

/// O mundo já compra este item (camada liberada)?
pub(crate) fn open_market(dimension: &str, item: &str) -> bool {
    product(item).is_some_and(|product| with_trade(dimension, |trade| PRODUCTS[product].3 <= open_tier(trade)))
}

/// Paridade de importação: o que custa trazer uma unidade de fora (preço mundial + frete + tarifa).
pub(crate) fn import_price(item: &str, day: i64) -> Option<i32> {
    let product = product(item)?;
    Some((world_cents(product, day) * (100 + IMPORT_FREIGHT + IMPORT_TARIFF) / 10_000).max(1))
}

/// Tendência da bolsa: índice de hoje menos o de ontem, em pontos.
pub(crate) fn trend(product: usize, day: i64) -> i32 {
    index(product, day) - index(product, day - 1)
}

/// Há conexão externa operando nesta dimensão?
pub(crate) fn connected(dimension: &str, now: u64) -> bool {
    CONNECTIONS.lock().is_ok_and(|all| all.iter().any(|(dim, _, seen)| dim == dimension && now.saturating_sub(*seen) <= STALE))
}

/// Exporta `count` unidades: cada unidade empurra o preço; as que cabem num contrato levam o
/// prêmio; imposto na fonte; moeda nova na companhia. Devolve o bruto (0 se o mundo não compra).
pub(crate) fn export(dimension: &str, at: [i32; 3], item: &str, count: u32, day: i64, now: u64) -> i32 {
    let Some(product) = product(item) else { return 0 };
    if count == 0 || !ledger::company_open(dimension, now) {
        return 0;
    }
    // Uma trava só: cotação, saturação, contratos e reputação juntos.
    let total = with_trade(dimension, |trade| {
        if PRODUCTS[product].3 > open_tier(trade) {
            return 0;
        }
        // Em centésimos de moeda. O contrato é um comprador à parte: paga o preço cheio do dia mais o
        // prêmio e não derruba o mercado à vista. O que sobra vai à vista, com saturação.
        let mut cents = 0;
        for _ in 0..count {
            if let Some(contract) = trade.contracts.iter_mut().find(|contract| contract.product == product && contract.done < contract.need) {
                contract.done += 1;
                cents += world_cents(product, day) * (100 + PREMIUM) / 100;
            } else {
                cents += spot_cents(product, day, pressure(trade, product));
                trade.today[product] += 1;
            }
        }
        let total = cents / 100;
        // Contratos cumpridos: reputação +1 cada e saem da lista (o próximo vem no dia seguinte).
        let done: Vec<Contract> = trade.contracts.iter().filter(|contract| contract.done >= contract.need).cloned().collect();
        trade.contracts.retain(|contract| contract.done < contract.need);
        for contract in &done {
            let premium = world_cents(contract.product, day) * contract.need * PREMIUM / 10_000;
            trade.news.push(News::Fulfilled(contract.product, premium));
        }
        reputation(trade, done.len() as i32);
        trade.exports_turn += total;
        total
    });
    if total <= 0 {
        return 0;
    }
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

/// Carrega (uma vez) e guarda no centro de controle reputação, contratos e a saturação recente.
/// Formato `Trade`: [versão, reputação, emitidos, contratos (produto, pedido, feito, prazo)...] e
/// `TradeRecent`: pares (produto, unidades) só dos produtos com pressão.
pub(crate) fn persist(dimension: &str, entity: &mut ModBlockEntity, time: i64) {
    const VERSION: i32 = 3;
    with_trade(dimension, |trade| {
        if !trade.loaded {
            trade.loaded = true;
            if let Some(Tag::IntArray(values)) = entity.nbt.get(nbt_key("Trade")) {
                // 0.23: [reputação, emitidos, contratos...]; 0.24: [2, reputação, emitidos, contratos...];
                // 0.25: [3, reputação, emitidos, melhor camada, contratos...].
                let (body, best) = match values.first() {
                    Some(&3) if values.len() >= 4 => (&values[1..], Some(values[3])),
                    Some(&2) => (&values[1..], None),
                    _ => (&values[..], None),
                };
                if body.len() >= 2 {
                    trade.reputation = body[0].max(0);
                    trade.issued = body[1].max(0);
                    trade.best = best.unwrap_or(0).max(unlocked_tier(trade.reputation));
                    let skip = if best.is_some() { 3 } else { 2 };
                    trade.contracts = body[skip..].chunks_exact(4)
                        .filter(|chunk| (chunk[0] as usize) < PRODUCTS.len())
                        .map(|chunk| Contract { product: chunk[0] as usize, need: chunk[1], done: chunk[2], due: i64::from(chunk[3]) })
                        .collect();
                }
            }
            if let Some(Tag::IntArray(values)) = entity.nbt.get(nbt_key("TradeRecent")) {
                for pair in values.chunks_exact(2) {
                    if let Some(slot) = trade.recent.get_mut(pair[0] as usize) {
                        *slot = pair[1].max(0);
                    }
                }
            }
            // O dia salvo não é guardado: hoje recompleta os contratos que faltarem, sem avisos.
            new_day(trade, time.div_euclid(DAY));
            trade.news.retain(|news| !matches!(news, News::Offered(..)));
        }
        new_day(trade, time.div_euclid(DAY));
        let mut values = vec![VERSION, trade.reputation, trade.issued, trade.best];
        for contract in &trade.contracts {
            values.extend([contract.product as i32, contract.need, contract.done, contract.due as i32]);
        }
        entity.nbt.insert("Trade".into(), Tag::IntArray(values));
        let recent: Vec<i32> = (0..PRODUCTS.len())
            .filter(|product| trade.recent[*product] + trade.today[*product] > 0)
            // O dia de hoje entra na média como numa virada de dia.
            .flat_map(|product| [product as i32, (trade.recent[product] * 6 + trade.today[product] * 4) / 10])
            .collect();
        entity.nbt.insert("TradeRecent".into(), Tag::IntArray(recent));
    });
}

/// Os avisos acumulados, para o centro de controle mandar ao chat.
pub(crate) fn drain(dimension: &str) -> Vec<News> {
    with_trade(dimension, |trade| std::mem::take(&mut trade.news))
}

/// O aviso no chat, no idioma do jogador.
pub(crate) fn news_text(news: &News) -> Text {
    let name = |product: usize| Text::tr(PRODUCTS[product].1, &[]).plain();
    match news {
        News::Offered(product, need, days) => Text::tr("veio.trade.news.offered", &[&need, &name(*product), &days, &PREMIUM]),
        News::Fulfilled(product, premium) => Text::tr("veio.trade.news.fulfilled", &[&name(*product), &premium]),
        News::Lost(product) => Text::tr("veio.trade.news.lost", &[&name(*product)]),
        News::Opened(tier) => Text::tr(&format!("veio.trade.news.opened{tier}"), &[]),
    }
}

/// Números da aba "Comércio exterior": [reputação, camada, exporta/dia, importa/dia, próxima
/// reputação (0 se tudo aberto)] e até três contratos (produto, feito, pedido, dias).
pub(crate) fn summary(dimension: &str, day: i64) -> ([i32; 5], Vec<[i32; 4]>) {
    with_trade(dimension, |trade| {
        ([trade.reputation, open_tier(trade), trade.exports_day, trade.imports_day,
            next_unlock(trade.reputation).unwrap_or(0)],
            trade.contracts.iter().map(|contract| [contract.product as i32, contract.done, contract.need,
                (contract.due - day).max(0) as i32]).collect())
    })
}

/// As melhores cotações de hoje entre os produtos abertos: (produto, preço, tendência).
pub(crate) fn board(dimension: &str, day: i64, count: usize) -> Vec<(usize, i32, i32)> {
    let mut quotes: Vec<(usize, i32, i32)> = with_trade(dimension, |trade| {
        let tier = open_tier(trade);
        (0..PRODUCTS.len()).filter(|product| PRODUCTS[*product].3 <= tier)
            .map(|product| (product, saturated(product, day, pressure(trade, product)), trend(product, day)))
            .collect()
    });
    quotes.sort_by(|a, b| b.1.cmp(&a.1));
    quotes.truncate(count);
    quotes
}

/// Reputação e contratos (testes).
#[cfg(test)]
pub(crate) fn state(dimension: &str) -> (i32, Vec<Contract>) {
    with_trade(dimension, |trade| (trade.reputation, trade.contracts.clone()))
}

#[cfg(test)]
pub(crate) fn set_reputation(dimension: &str, value: i32) {
    with_trade(dimension, |trade| {
        trade.reputation = value;
        trade.best = trade.best.max(open_tier(trade));
    });
}

/// Camada aberta agora (testes).
#[cfg(test)]
pub(crate) fn tier_open(dimension: &str) -> i32 {
    with_trade(dimension, |trade| open_tier(trade))
}

// ---------- Conexões externas ----------

/// Capacidade (pilhas por viagem), turnos entre viagens e frete (base, por pilha). O frete é por
/// pilha, não por valor: pesa em matéria-prima (64 minérios de ferro valem 128) e quase nada em
/// componentes (64 engrenagens valem 896). O navio é barato e lento; o trem, caro e frequente.
pub(crate) fn schedule(block: &str) -> (usize, i64, i32, i32) {
    if block == PORT { (SLOTS, 2, 4, 6) } else { (6, 1, 2, 12) }
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

/// A estação está na fronteira: longe da sede da companhia, de toda prefeitura e do centro da vila.
fn at_frontier(dimension: &str, pos: [i32; 3], now: u64) -> bool {
    let far = |at: [i32; 3]| (at[0] - pos[0]).abs().max((at[2] - pos[2]).abs()) >= FRONTIER;
    let halls_far = HALLS.lock().is_ok_and(|halls| halls.iter()
        .filter(|hall| hall.dimension == dimension && now.saturating_sub(hall.seen) <= STALE).all(|hall| far(hall.pos)));
    let village_far = dimension != spawn::OVERWORLD || spawn::center().is_none_or(|(at, _)| far(at));
    halls_far && village_far && ledger::company_positions(dimension, now).into_iter().all(far)
}

/// Tick de uma conexão externa: confere o lugar, registra-se e despacha no horário.
pub(crate) fn tick(host: &mut dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity, time: i64) {
    let phase = (time + i64::from((pos[0] ^ pos[2]) & 63)).rem_euclid(TURN);
    if phase % 20 != 0 {
        return;
    }
    let dimension = host.dimension();
    let now = defense::clock();
    let port = entity.id == PORT;
    let (capacity, every, freight_base, freight_stack) = schedule(&entity.id);
    // O lugar: água (porto, conferida uma vez por dia) ou fronteira (estação).
    let place_ok = if port {
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
    // Painel: segundos até a partida, capacidade e frete máximo.
    set_number(entity, "Info", (departs_in / 20) as i32);
    set_number(entity, "Progress", capacity as i32);
    set_number(entity, "Utilization", freight_base + freight_stack * capacity as i32);
    let today = time.div_euclid(DAY) as i32;
    if number(entity, "Day") != today {
        set_number(entity, "Day", today);
        set_number(entity, "Rate", 0);
        set_number(entity, "Total", 0);
    }
    if !place_ok || !company {
        set_number(entity, "Status", if !company { NO_COMPANY } else if port { NO_WATER } else { NOT_FRONTIER });
        return;
    }
    upsert(&CONNECTIONS, (dimension.to_owned(), pos, now), |(dim, at, _)| dim == dimension && *at == pos,
        |(_, _, seen)| now.saturating_sub(*seen) <= STALE);
    let cargo: Vec<&ItemStack> = entity.items.iter().flatten().filter(|stack| product(&stack.item).is_some()).collect();
    let sellable = cargo.iter().any(|stack| open_market(dimension, &stack.item));
    let waiting = if cargo.is_empty() { ZONE_ACTIVE } else if sellable { STORING } else { LOCKED_MARKET };
    if phase != 0 || turn.rem_euclid(every) != 0 {
        set_number(entity, "Status", waiting);
        return;
    }
    // Partida: até `capacity` pilhas que o mundo compra; o frete sai da companhia (para fora).
    let day = time.div_euclid(DAY);
    let (mut stacks, mut units, mut coins) = (0, 0, 0);
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
    if stacks == 0 {
        set_number(entity, "Status", waiting);
        return;
    }
    let freight = freight_base + freight_stack * stacks as i32;
    let paid = ledger::take(Book::Company, dimension, pos, ledger::ANYWHERE, freight, ledger::REDEEM, now);
    ledger::border(dimension, -paid);
    set_number(entity, "Rate", number(entity, "Rate").saturating_add(units));
    set_number(entity, "Total", number(entity, "Total").saturating_add(coins - paid));
    set_number(entity, "Status", 1);
    // A partida: buzina do navio ou trem, fumaça e um aviso para quem está perto.
    let at = center(pos);
    let (sound, pitch) = if port { ("minecraft:item.goat_horn.sound.0", 0.6) } else { ("minecraft:entity.minecart.riding", 1.0) };
    host.play_sound(sound, at, 4, 1.0, pitch);
    host.send_particles(&ParticleOptions::simple(ParticleType::LargeSmoke), None, [at[0], at[1] + 1.5, at[2]],
        [0.6, 0.4, 0.6], 0.02, 12);
    host.send_particles(&ParticleOptions::simple(ParticleType::HappyVillager), None, [at[0], at[1] + 1.2, at[2]],
        [0.4, 0.3, 0.4], 0.0, 6);
    let key = if port { "veio.trade.departed.ship" } else { "veio.trade.departed.train" };
    for player in host.server_players() {
        if player.dimension == dimension && (player.position[0] - at[0]).hypot(player.position[2] - at[2]) <= 48.0 {
            host.action_bar(player.uuid, Text::tr(key, &[&units, &(coins - paid)]));
        }
    }
}
