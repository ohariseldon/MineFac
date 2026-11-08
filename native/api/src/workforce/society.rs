//! Sociedade da vila: casas e famílias, salário, consumo, lojas, prefeito e indicadores.
//!
//! Desenho completo em `frentes/SOCIEDADE.md`. Resumo:
//! - **Casa** (`HOUSE`): até `CAPACITY` moradores (aldeões a até `HOME_RADIUS` blocos, cada um numa
//!   casa só). A cada turno (`TURN` ticks do mundo) recebe o salário dos moradores empregados, come
//!   da despensa (3 espaços), gasta conforto e compra o que falta na loja mais próxima.
//! - **Loja** (`SHOP`): posto com comerciante. Estoque em 6 espaços, caixa em moedas que vira
//!   esmeralda no último espaço. Vende às casas e ao jogador (tela de troca do comerciante).
//! - **Prefeitura** (`TOWN_HALL`): posto com prefeito. Cofre (esmeraldas viram moedas) que paga os
//!   salários, indicadores (população, emprego, PIB, IDH, Gini), fases do plano diretor, alvarás de
//!   casa, pedidos e os limites de zona e de nascimentos.
//! - **Poço** (`WELL`): saneamento para as casas; enche baldes.
//!
//! Custo: casas e prefeituras trabalham uma vez por turno (1 min) e leem só registros em memória e
//! o censo de posições; a loja só reescreve as ofertas do comerciante quando o estoque muda.
use super::*;
use horizon_client::mod_api::MerchantOffer;
use std::sync::atomic::{AtomicBool, Ordering};

pub(crate) const HOUSE: &str = "veio:house";
pub(crate) const SHOP: &str = "veio:shop";
pub(crate) const TOWN_HALL: &str = "veio:town_hall";
pub(crate) const WELL: &str = "veio:well";

/// Turno da sociedade: 1 min (1/20 do dia do Minecraft).
pub(crate) const TURN: i64 = 1200;
pub(crate) const TURNS_PER_DAY: i64 = 24000 / TURN;
pub(crate) const DAY: i64 = 24000;
/// Moedas por esmeralda (o mesmo preço da despensa: 60 pontos de comida).
pub(crate) const COINS_PER_EMERALD: i32 = 60;
/// Salário por turno de cada morador empregado.
pub(crate) const WAGE_PER_TURN: i32 = 2;
/// Valor de produção de cada tarefa concluída por um trabalhador (PIB).
pub(crate) const TASK_VALUE: i32 = 2;
/// Comida por morador por turno (20 pontos por dia: pouco mais de 3 pães).
pub(crate) const FOOD_PER_TURN: i32 = 1;
/// Conforto gasto por morador por turno; bens repõem.
pub(crate) const COMFORT_PER_TURN: i32 = 1;
/// Custo de vida por morador por dia (comida + bens), base do índice de renda.
pub(crate) const LIVING_COST: i32 = 30;

pub(crate) const CAPACITY: usize = 4;
pub(crate) const HOME_RADIUS: f64 = 10.0;
pub(crate) const PANTRY_SLOTS: usize = 3;
pub(crate) const WELL_RADIUS: i32 = 24;
pub(crate) const POWER_RADIUS: i32 = 12;
pub(crate) const SHOP_RADIUS: i32 = 40;
pub(crate) const HALL_RADIUS: i32 = 128;
/// Casas a até `CROWD_RADIUS` blocos: mais de `CROWD_LIMIT` deixam a casa apertada.
pub(crate) const CROWD_RADIUS: i32 = 16;
pub(crate) const CROWD_LIMIT: usize = 6;
/// Casas a até `DISTRICT_RADIUS` blocos formam um bairro.
pub(crate) const DISTRICT_RADIUS: i32 = 24;
/// Um recado de compra mantém o morador indo à loja por este tempo (relógio do mod).
const ERRAND: u64 = 400;
const STALE: u64 = 3000;
const STOCK_SLOTS: std::ops::Range<usize> = 1..7;
const TILL: usize = 7;
const EVENTS_MAX: usize = 4096;

/// Bens de consumo (conforto): (item, preço em moedas, pontos de conforto).
pub(crate) const GOODS: [(&str, i32, i32); 12] = [
    ("minecraft:white_wool", 4, 8), ("minecraft:leather", 6, 12), ("minecraft:paper", 2, 4),
    ("minecraft:book", 10, 20), ("minecraft:candle", 4, 8), ("minecraft:torch", 1, 2),
    ("minecraft:glass_bottle", 2, 4), ("minecraft:flower_pot", 3, 6), ("minecraft:honey_bottle", 5, 10),
    ("minecraft:painting", 8, 16), ("minecraft:lantern", 6, 12), ("minecraft:feather", 1, 2),
];

pub(crate) fn is_society_block(block: &str) -> bool {
    matches!(block, HOUSE | WELL)
}

pub(crate) fn goods(item: &str) -> Option<(i32, i32)> {
    GOODS.iter().find(|(known, _, _)| *known == item).map(|(_, price, comfort)| (*price, *comfort))
}

/// Preço em moedas de uma unidade na loja: comida vale os pontos, bens a tabela.
pub(crate) fn price(item: &str) -> Option<i32> {
    production::pantry::value(item).map(|value| value as i32).or_else(|| goods(item).map(|(price, _)| price))
}

pub(crate) fn layout(block: &str) -> Option<(usize, Vec<usize>, Vec<usize>)> {
    Some(match block {
        HOUSE => (PANTRY_SLOTS, (0..PANTRY_SLOTS).collect(), vec![]),
        WELL => (1, vec![0], vec![0]),
        SHOP => (8, (0..7).collect(), vec![TILL]),
        TOWN_HALL => (2, vec![0, 1], vec![]),
        _ => return None,
    })
}

pub(crate) fn accepts(block: &str, slot: usize, stack: &ItemStack) -> bool {
    match (block, slot) {
        (HOUSE, 0..PANTRY_SLOTS) => is_food(&stack.item),
        (WELL, 0) => stack.item.as_ref() == "minecraft:bucket",
        (SHOP, 0) => is_food(&stack.item),
        (SHOP, 1..7) => price(&stack.item).is_some(),
        (TOWN_HALL, 0) => is_food(&stack.item),
        (TOWN_HALL, 1) => stack.item.as_ref() == WAGE,
        _ => false,
    }
}

// ---------- Registros ----------

/// Uma casa como a prefeitura a vê.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Home {
    pub dimension: String,
    pub pos: [i32; 3],
    /// Tick do mundo em que a casa apareceu (ordem dos alvarás).
    pub built: i64,
    pub licensed: bool,
    pub residents: i32,
    pub adults: i32,
    pub kids: i32,
    pub employed: i32,
    /// Renda do último dia (média móvel), em moedas.
    pub income: i32,
    pub hungry: bool,
    pub sanitation: bool,
    pub power: bool,
    pub commerce: bool,
    pub crowded: bool,
    pub smog: bool,
    /// Índices em centésimos: alimentação, moradia, serviços, renda.
    pub food: i32,
    pub housing: i32,
    pub services: i32,
    pub wealth: i32,
    pub seen: u64,
}

impl Home {
    /// IDH da família, em centésimos: média geométrica das quatro dimensões (piso de 0,05).
    pub(crate) fn idh(&self) -> i32 {
        let parts = [self.food, self.housing, self.services, self.wealth];
        let product: f64 = parts.iter().map(|part| (f64::from(*part) / 100.0).max(0.05)).product();
        (product.powf(0.25) * 100.0).round() as i32
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Shop {
    pub dimension: String,
    pub pos: [i32; 3],
    pub merchant: Option<u32>,
    pub open: bool,
    pub stock: Vec<(String, u32)>,
    pub seen: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Hall {
    pub dimension: String,
    pub pos: [i32; 3],
    pub phase: i32,
    pub suspended: bool,
    pub staffed: bool,
    pub seen: u64,
}

static HOMES: Mutex<Vec<Home>> = Mutex::new(Vec::new());
static SHOPS: Mutex<Vec<Shop>> = Mutex::new(Vec::new());
static HALLS: Mutex<Vec<Hall>> = Mutex::new(Vec::new());
static WELLS: Mutex<Vec<(String, [i32; 3], u64)>> = Mutex::new(Vec::new());
/// Cofre de cada prefeitura, em moedas (a fonte é a memória; o NBT guarda uma cópia).
static TREASURY: Mutex<Vec<(String, [i32; 3], i32)>> = Mutex::new(Vec::new());
/// Morador -> casa (dimensão, posição, relógio).
static RESIDENTS: Mutex<Vec<(u32, String, [i32; 3], u64)>> = Mutex::new(Vec::new());
/// Produção e comércio: (dimensão, posição, tick do mundo, moedas, comércio?).
static EVENTS: Mutex<Vec<(String, [i32; 3], i64, i32, bool)>> = Mutex::new(Vec::new());
static ACTIVE: AtomicBool = AtomicBool::new(false);

fn within(a: [i32; 3], b: [i32; 3], radius: i32) -> bool {
    (a[0] - b[0]).abs() <= radius && (a[2] - b[2]).abs() <= radius && (a[1] - b[1]).abs() <= radius.max(16)
}

fn upsert<T: Clone>(list: &Mutex<Vec<T>>, item: T, same: impl Fn(&T) -> bool, fresh: impl Fn(&T) -> bool) {
    let Ok(mut list) = list.lock() else { return };
    if let Some(known) = list.iter_mut().find(|known| same(known)) {
        *known = item;
    } else {
        list.retain(|known| fresh(known));
        list.push(item);
    }
    ACTIVE.store(true, Ordering::Release);
}

pub(crate) fn homes_near(dimension: &str, at: [i32; 3], radius: i32, now: u64) -> Vec<Home> {
    if !ACTIVE.load(Ordering::Acquire) {
        return Vec::new();
    }
    HOMES.lock().map(|homes| homes.iter()
        .filter(|home| home.dimension == dimension && now.saturating_sub(home.seen) <= STALE && within(home.pos, at, radius))
        .cloned().collect()).unwrap_or_default()
}

fn shops_near(dimension: &str, at: [i32; 3], radius: i32, now: u64) -> Vec<Shop> {
    if !ACTIVE.load(Ordering::Acquire) {
        return Vec::new();
    }
    SHOPS.lock().map(|shops| shops.iter()
        .filter(|shop| shop.dimension == dimension && now.saturating_sub(shop.seen) <= STALE && within(shop.pos, at, radius))
        .cloned().collect()).unwrap_or_default()
}

fn well_near(dimension: &str, at: [i32; 3], now: u64) -> bool {
    WELLS.lock().is_ok_and(|wells| wells.iter().any(|(dim, pos, seen)| dim == dimension
        && now.saturating_sub(*seen) <= STALE && within(*pos, at, WELL_RADIUS)))
}

/// A prefeitura que governa `at`: a mais próxima com prefeito, a até `HALL_RADIUS` blocos.
pub(crate) fn governing(dimension: &str, at: [i32; 3], now: u64) -> Option<Hall> {
    if !ACTIVE.load(Ordering::Acquire) {
        return None;
    }
    let halls = HALLS.lock().ok()?;
    halls.iter()
        .filter(|hall| hall.dimension == dimension && hall.staffed && now.saturating_sub(hall.seen) <= STALE
            && within(hall.pos, at, HALL_RADIUS))
        .min_by_key(|hall| (hall.pos[0] - at[0]).abs() + (hall.pos[2] - at[2]).abs())
        .cloned()
}

fn treasury(dimension: &str, hall: [i32; 3]) -> Option<i32> {
    TREASURY.lock().ok()?.iter().find(|(dim, pos, _)| dim == dimension && *pos == hall).map(|(_, _, coins)| *coins)
}

fn set_treasury(dimension: &str, hall: [i32; 3], coins: i32) {
    if let Ok(mut all) = TREASURY.lock() {
        match all.iter_mut().find(|(dim, pos, _)| dim == dimension && *pos == hall) {
            Some(entry) => entry.2 = coins,
            None => all.push((dimension.to_owned(), hall, coins)),
        }
    }
}

/// Tira até `amount` moedas do cofre; devolve quanto pagou.
fn withdraw(dimension: &str, hall: [i32; 3], amount: i32) -> i32 {
    let Some(coins) = treasury(dimension, hall) else { return 0 };
    let paid = amount.min(coins).max(0);
    set_treasury(dimension, hall, coins - paid);
    paid
}

/// Registra produção (tarefas) ou comércio (vendas), para o PIB.
pub(crate) fn record(dimension: &str, pos: [i32; 3], time: i64, coins: i32, commerce: bool) {
    if coins <= 0 {
        return;
    }
    if let Ok(mut events) = EVENTS.lock() {
        if events.len() >= EVENTS_MAX {
            events.retain(|(_, _, at, _, _)| time.saturating_sub(*at) <= DAY);
            if events.len() >= EVENTS_MAX {
                events.drain(0..EVENTS_MAX / 4);
            }
        }
        events.push((dimension.to_owned(), pos, time, coins, commerce));
    }
}

/// PIB do último dia em volta de `at`: (produção, comércio).
pub(crate) fn output(dimension: &str, at: [i32; 3], radius: i32, time: i64) -> (i32, i32) {
    EVENTS.lock().map(|events| events.iter()
        .filter(|(dim, pos, when, _, _)| dim == dimension && time.saturating_sub(*when) <= DAY && *when <= time
            && within(*pos, at, radius))
        .fold((0, 0), |(made, sold), (_, _, _, coins, commerce)| if *commerce { (made, sold + coins) } else { (made + coins, sold) }))
        .unwrap_or((0, 0))
}

/// Reivindica o aldeão como morador desta casa. `false` se ele já mora em outra.
fn settle(id: u32, dimension: &str, pos: [i32; 3], now: u64) -> bool {
    let Ok(mut residents) = RESIDENTS.lock() else { return false };
    if let Some(entry) = residents.iter_mut().find(|(known, _, _, _)| *known == id) {
        let mine = entry.1 == dimension && entry.2 == pos;
        if !mine && now.saturating_sub(entry.3) <= STALE {
            return false;
        }
        *entry = (id, dimension.to_owned(), pos, now);
        return true;
    }
    if residents.len() >= 2048 {
        residents.retain(|(_, _, _, seen)| now.saturating_sub(*seen) <= STALE);
    }
    residents.push((id, dimension.to_owned(), pos, now));
    true
}

fn leave(dimension: &str, pos: [i32; 3], keep: &[u32]) {
    if let Ok(mut residents) = RESIDENTS.lock() {
        residents.retain(|(id, dim, at, _)| !(dim == dimension && *at == pos && !keep.contains(id)));
    }
}

/// Moradores registrados em volta de `at` (para contar os sem-teto).
fn housed(dimension: &str, now: u64) -> Vec<u32> {
    RESIDENTS.lock().map(|residents| residents.iter()
        .filter(|(_, dim, _, seen)| dim == dimension && now.saturating_sub(*seen) <= STALE)
        .map(|(id, _, _, _)| *id).collect()).unwrap_or_default()
}

// ---------- Plano diretor ----------

/// Exigências para chegar a cada fase: (moradores, famílias, IDH, PIB/dia, emprego %, Gini máx., bairros).
pub(crate) const PHASES: [(i32, i32, i32, i32, i32, i32, i32); 3] = [
    (12, 3, 40, 0, 0, 100, 0),
    (36, 9, 55, 400, 60, 100, 0),
    (80, 20, 70, 0, 0, 45, 3),
];

pub(crate) fn zone_limit(phase: i32) -> i32 {
    match phase { 1 => 2, 2 => 3, _ => 4 }
}

pub(crate) fn population_limit(phase: i32) -> usize {
    match phase { 1 => 30, 2 => 60, 3 => 100, _ => 150 }
}

pub(crate) fn permits(phase: i32) -> usize {
    match phase { 1 => 4, 2 => 10, 3 => 24, _ => 48 }
}

/// O padrão de casa exigido na fase: saneamento (2+), energia (3+), comércio (4).
fn meets_standard(phase: i32, sanitation: bool, power: bool, commerce: bool) -> bool {
    (phase < 2 || sanitation) && (phase < 3 || power) && (phase < 4 || commerce)
}

/// Até que nível uma zona em `at` pode subir agora. Sem prefeitura: fase 1.
pub(crate) fn zone_cap(dimension: &str, at: [i32; 3], now: u64) -> (i32, bool) {
    match governing(dimension, at, now) {
        Some(hall) => (zone_limit(hall.phase), hall.suspended),
        None => (zone_limit(1), false),
    }
}

/// Pode nascer mais um aldeão em `at`, com `near` aldeões em volta?
pub(crate) fn birth_allowed(dimension: &str, at: [i32; 3], near: usize, now: u64) -> bool {
    match governing(dimension, at, now) {
        None => near < population_limit(1),
        Some(hall) => {
            if hall.suspended || near >= population_limit(hall.phase) {
                return false;
            }
            // Com prefeito, bebê só nasce onde há vaga em casa com alvará.
            homes_near(dimension, at, HALL_RADIUS, now).iter()
                .any(|home| home.licensed && (home.residents as usize) < CAPACITY)
        }
    }
}

// ---------- Casa ----------

fn pantry_points(entity: &ModBlockEntity) -> i32 {
    entity.items.iter().take(PANTRY_SLOTS).flatten()
        .map(|stack| production::pantry::value(&stack.item).unwrap_or(0) as i32 * stack.count as i32).sum()
}

fn variety(entity: &ModBlockEntity) -> i32 {
    let mut kinds: Vec<&str> = entity.items.iter().take(PANTRY_SLOTS).flatten().map(|stack| stack.item.as_ref()).collect();
    kinds.sort_unstable();
    kinds.dedup();
    kinds.len() as i32
}

/// Come `need` pontos da despensa. Devolve o que faltou.
fn eat_pantry(entity: &mut ModBlockEntity, need: i32) -> i32 {
    let mut fed = number(entity, "Fed");
    while fed < need {
        let Some(slot) = (0..PANTRY_SLOTS).find(|slot| entity.items[*slot].as_ref()
            .is_some_and(|stack| production::pantry::value(&stack.item).is_some())) else { break };
        let value = entity.items[slot].as_ref().and_then(|stack| production::pantry::value(&stack.item)).unwrap_or(0);
        consume(&mut entity.items[slot], 1);
        fed += value as i32;
    }
    let missing = (need - fed).max(0);
    set_number(entity, "Fed", (fed - need).max(0));
    missing
}

/// Põe `count` de `item` na despensa; devolve quantos couberam.
fn stock_pantry(entity: &mut ModBlockEntity, item: &str, count: u32) -> u32 {
    let mut left = count;
    for slot in 0..PANTRY_SLOTS {
        while left > 0 && can_add(&entity.items[slot], item, 1) {
            add(&mut entity.items[slot], item, 1);
            left -= 1;
        }
    }
    count - left
}

/// Compra da loja em `shop`: tira do estoque e põe o valor no caixa. Devolve o que comprou.
fn buy(host: &mut dyn Host, shop: [i32; 3], wanted: impl Fn(&str) -> bool, budget: i32, max_units: u32, time: i64) -> Vec<(String, u32, i32)> {
    let mut bought: Vec<(String, u32, i32)> = Vec::new();
    host.change_mod_block_entity(shop, &mut |entity| {
        if entity.id != SHOP {
            return;
        }
        let mut spent = 0;
        let mut units = 0;
        // Uma unidade por vez, rodando entre os itens: a família leva variedade.
        let mut progress = true;
        while progress && units < max_units {
            progress = false;
            for slot in STOCK_SLOTS {
                let Some(stack) = entity.items[slot].as_ref() else { continue };
                if !wanted(&stack.item) || stack.extra.is_some() {
                    continue;
                }
                let Some(cost) = price(&stack.item) else { continue };
                if spent + cost > budget || units >= max_units {
                    continue;
                }
                let item = stack.item.to_string();
                consume(&mut entity.items[slot], 1);
                spent += cost;
                units += 1;
                progress = true;
                match bought.iter_mut().find(|(known, _, _)| *known == item) {
                    Some(entry) => { entry.1 += 1; entry.2 += cost; }
                    None => bought.push((item, 1, cost)),
                }
            }
        }
        if spent > 0 {
            credit_till(entity, spent);
            set_number(entity, "SoldToday", number(entity, "SoldToday").saturating_add(spent));
        }
    });
    let spent: i32 = bought.iter().map(|(_, _, cost)| cost).sum();
    if spent > 0 {
        let dimension = host.dimension();
        record(dimension, shop, time, spent, true);
    }
    bought
}

/// Moedas no caixa; a cada `COINS_PER_EMERALD`, uma esmeralda no espaço do caixa.
fn credit_till(entity: &mut ModBlockEntity, coins: i32) {
    let mut till = number(entity, "Till").saturating_add(coins);
    while till >= COINS_PER_EMERALD && can_add(&entity.items[TILL], WAGE, 1) {
        add(&mut entity.items[TILL], WAGE, 1);
        till -= COINS_PER_EMERALD;
    }
    set_number(entity, "Till", till);
}

fn signed(entity: &ModBlockEntity, key: &str) -> i32 {
    match entity.nbt.get(nbt_key(key)) { Some(Tag::Int(value)) => *value, _ => 0 }
}

fn set_signed(entity: &mut ModBlockEntity, key: &str, value: i32) {
    entity.nbt.insert(key.into(), Tag::Int(value));
}

/// Estados extras do painel.
pub(crate) const HOUSED: i32 = 29;
pub(crate) const NO_PERMIT: i32 = 30;
pub(crate) const EMPTY_HOME: i32 = 31;

/// Necessidade mais urgente da casa (dica do painel): 0 nada, 1 fome, 2 renda, 3 saneamento,
/// 4 energia, 5 comércio, 6 ar, 7 lotação, 8 sem alvará, 9 sem prefeitura.
fn worst_need(home: &Home, governed: bool) -> i32 {
    if !home.licensed { return 8; }
    if home.residents == 0 { return 0; }
    if home.hungry { return 1; }
    if !governed { return 9; }
    if home.wealth < 50 { return 2; }
    if !home.sanitation { return 3; }
    if !home.commerce { return 5; }
    if !home.power { return 4; }
    if home.smog { return 6; }
    if home.crowded { return 7; }
    0
}

pub(crate) fn tick_block(host: &mut dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity, time: i64) {
    let phase = (time + i64::from((pos[0] ^ pos[2]) & 63)).rem_euclid(TURN);
    match entity.id.as_str() {
        WELL => {
            if phase % 20 == 0 {
                let dimension = host.dimension().to_owned();
                let now = defense::clock();
                upsert(&WELLS, (dimension.clone(), pos, now), |(dim, at, _)| *dim == dimension && *at == pos,
                    |(_, _, seen)| now.saturating_sub(*seen) <= STALE);
                // Balde vazio vira balde d'água (um por vez: o d'água não empilha).
                if entity.items[0].as_ref().is_some_and(|stack| stack.item.as_ref() == "minecraft:bucket" && stack.count == 1) {
                    entity.items[0] = Some(ItemStack::new("minecraft:water_bucket", 1));
                }
                set_number(entity, "Status", ZONE_ACTIVE);
            }
        }
        HOUSE => {
            if !entity.nbt.contains_key(nbt_key("Built")) {
                set_signed(entity, "Built", time.clamp(0, i64::from(i32::MAX)) as i32);
            }
            if phase == 0 || !entity.nbt.contains_key(nbt_key("Turned")) {
                set_number(entity, "Turned", 1);
                house_turn(host, pos, entity, time);
            }
        }
        _ => {}
    }
}

/// Um turno da família.
pub(crate) fn house_turn(host: &mut dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity, time: i64) {
    let dimension = host.dimension().to_owned();
    let now = defense::clock();
    let built = i64::from(signed(entity, "Built"));
    let at = center(pos);
    let hall = governing(&dimension, pos, now);
    let phase = hall.as_ref().map_or(1, |hall| hall.phase);
    // Serviços em volta.
    let sanitation = well_near(&dimension, pos, now);
    let power = power::pole_near(&dimension, pos, POWER_RADIUS);
    let shops = shops_near(&dimension, pos, SHOP_RADIUS, now);
    let commerce = !shops.is_empty();
    let neighbors = homes_near(&dimension, pos, CROWD_RADIUS, now).iter().filter(|home| home.pos != pos).count();
    let crowded = neighbors > CROWD_LIMIT;
    let smog = pollution::local(&dimension, pos) > 400.0;
    // Alvará: ordem de construção entre as casas da mesma prefeitura (ou em volta, sem prefeitura).
    let anchor = hall.as_ref().map_or(pos, |hall| hall.pos);
    let earlier = homes_near(&dimension, anchor, HALL_RADIUS, now).iter()
        .filter(|home| home.pos != pos && home.licensed && (home.built, home.pos) < (built, pos))
        .count();
    let licensed = earlier < permits(phase) && meets_standard(phase, sanitation, power, commerce);
    // Moradores: aldeões perto, cada um em uma casa só.
    let mut residents: Vec<u32> = Vec::new();
    if licensed {
        let close: Vec<u32> = production::ranch::villager_positions(host, &dimension).iter()
            .filter(|(_, spot)| near(*spot, at, HOME_RADIUS))
            .map(|(id, _)| *id).collect();
        for id in close {
            if residents.len() >= CAPACITY {
                break;
            }
            if settle(id, &dimension, pos, now) {
                residents.push(id);
            }
        }
    }
    leave(&dimension, pos, &residents);
    let mut kids = 0;
    for id in &residents {
        let child = host.entity(*id).and_then(|handle| host.entity_nbt(handle))
            .is_some_and(|nbt| production::ranch::tag_int(&nbt, "Age") < 0);
        kids += i32::from(child);
    }
    let adults = residents.len() as i32 - kids;
    let employed = residents.iter().filter(|id| is_employed(**id, now)).count() as i32;
    // Salário pago pelo cofre da prefeitura.
    let owed = employed * WAGE_PER_TURN + number(entity, "Owed");
    let paid = hall.as_ref().map_or(0, |hall| withdraw(&dimension, hall.pos, owed));
    set_number(entity, "Owed", if hall.is_some() { (owed - paid).min(employed * WAGE_PER_TURN * TURNS_PER_DAY as i32) } else { 0 });
    let mut wallet = number(entity, "Wallet").saturating_add(paid);
    let income = number(entity, "Income");
    let income = income - income / TURNS_PER_DAY as i32 + paid;
    set_number(entity, "Income", income);
    // Comida: cada morador come; o que falta vira fome.
    let people = residents.len() as i32;
    let missing = eat_pantry(entity, people * FOOD_PER_TURN);
    let hungry_turns = if missing > 0 { number(entity, "Hungry") + 1 } else { (number(entity, "Hungry") - 1).max(0) };
    set_number(entity, "Hungry", hungry_turns.min(TURNS_PER_DAY as i32));
    // Conforto: bens comprados repõem.
    let mut comfort = (number(entity, "Comfort") - people * COMFORT_PER_TURN).max(0);
    // Compras de dia, na loja aberta mais próxima.
    let day = !night(host);
    let target = people * FOOD_PER_TURN * TURNS_PER_DAY as i32;
    let mut errand = None;
    if day && people > 0 && let Some(shop) = shops.iter().filter(|shop| shop.open)
        .min_by_key(|shop| (shop.pos[0] - pos[0]).abs() + (shop.pos[2] - pos[2]).abs())
    {
        let have = pantry_points(entity);
        if have < target / 2 && wallet > 0 {
            let room = (0..PANTRY_SLOTS).map(|slot| entity.items[slot].as_ref().map_or(64, |stack| 64 - stack.count)).sum::<u32>();
            let bought = buy(host, shop.pos, is_food, wallet, room.min(16), time);
            for (item, count, cost) in bought {
                let kept = stock_pantry(entity, &item, count);
                wallet -= cost * kept as i32 / count.max(1) as i32;
                errand = Some(shop.pos);
            }
        }
        if comfort < people * 10 && wallet > 0 {
            for (item, count, cost) in buy(host, shop.pos, |item| goods(item).is_some(), wallet / 2, 2, time) {
                comfort += goods(&item).map_or(0, |(_, points)| points) * count as i32;
                wallet -= cost;
                errand = Some(shop.pos);
            }
        }
    }
    set_number(entity, "Comfort", comfort.min(people.max(1) * 40));
    set_number(entity, "Wallet", wallet.max(0));
    // Um morador sem emprego vai até a loja (só visual: a compra já foi feita).
    if let Some(shop) = errand
        && let Some(id) = residents.iter().find(|id| !is_employed(**id, now) && order_for(**id, now).is_none())
    {
        order(*id, post_center(shop), now + ERRAND);
    }
    // Índices.
    let hungry = hungry_turns > 2;
    let food_index = if people == 0 { 100 } else {
        let fed = 100 - hungry_turns * 100 / TURNS_PER_DAY as i32;
        fed * (70 + 10 * variety(entity).min(3)) / 100
    };
    let mut housing = if licensed { 100 } else { 0 };
    if crowded { housing = housing * 70 / 100; }
    if smog { housing = housing * if pollution::local(&dimension, pos) > 1200.0 { 30 } else { 60 } / 100; }
    let services = (i32::from(sanitation) + i32::from(power) + i32::from(commerce)) * 100 / 3;
    let comfort_index = if people == 0 { 100 } else { (comfort * 100 / (people * 10)).min(100) };
    let wealth = if people == 0 { 100 } else { (income * 100 / (people * LIVING_COST)).min(100) * (50 + comfort_index / 2) / 100 };
    let home = Home {
        dimension: dimension.clone(), pos, built, licensed, residents: people, adults, kids, employed, income, hungry,
        sanitation, power, commerce, crowded, smog, food: food_index.clamp(0, 100), housing, services,
        wealth: wealth.clamp(0, 100), seen: now,
    };
    let need = worst_need(&home, hall.is_some());
    let idh = home.idh();
    upsert(&HOMES, home, |known| known.dimension == dimension && known.pos == pos,
        |known| now.saturating_sub(known.seen) <= STALE);
    // Painel: estado, moradores, capacidade, IDH, carteira, despensa e a necessidade.
    set_number(entity, "Status", if !licensed { NO_PERMIT } else if people == 0 { EMPTY_HOME } else { HOUSED });
    set_number(entity, "Rate", people);
    set_number(entity, "Utilization", CAPACITY as i32);
    set_number(entity, "Info", idh);
    set_number(entity, "Total", number(entity, "Wallet"));
    set_number(entity, "Duration", pantry_points(entity));
    set_number(entity, "Progress", need);
}

// ---------- Emprego ----------

/// O aldeão `id` trabalha num posto agora?
pub(crate) fn is_employed(id: u32, now: u64) -> bool {
    employed_ids(now).contains(&id)
}

// ---------- Loja ----------

/// O comerciante fica na loja.
pub(crate) fn plan(pos: [i32; 3]) -> Result<Plan, i32> {
    Ok(Plan::Visit { spot: pos, reach: POST_REACH })
}

/// Com o comerciante ou o prefeito no posto. A cada `CHARGE_EVERY` voltas, uma tarefa (comida).
pub(crate) fn perform(entity: &mut ModBlockEntity) -> Option<Vec<(String, u32)>> {
    const CHARGE_EVERY: i32 = 60;
    set_number(entity, "Present", 1);
    let round = (number(entity, "Round") + 1) % CHARGE_EVERY;
    set_number(entity, "Round", round);
    (round == 0).then(Vec::new)
}

fn stock(entity: &ModBlockEntity) -> Vec<(String, u32)> {
    let mut items: Vec<(String, u32)> = Vec::new();
    for stack in entity.items[STOCK_SLOTS].iter().flatten() {
        if stack.extra.is_some() || price(&stack.item).is_none() {
            continue;
        }
        match items.iter_mut().find(|(item, _)| *item == *stack.item) {
            Some((_, count)) => *count += stack.count,
            None => items.push((stack.item.to_string(), stack.count)),
        }
    }
    items
}

/// Lote e preço em esmeraldas de um item para o jogador: o menor lote que vale 1 esmeralda (até 16).
pub(crate) fn bundle(item: &str) -> Option<(u32, u32)> {
    let unit = price(item)?.max(1) as u32;
    let count = (COINS_PER_EMERALD as u32).div_ceil(unit).clamp(1, 16);
    let emeralds = (count * unit).div_ceil(COINS_PER_EMERALD as u32).max(1);
    Some((count, emeralds))
}

/// As ofertas do comerciante a partir do estoque.
pub(crate) fn offers(stock: &[(String, u32)]) -> Tag {
    let recipes: Vec<Tag> = stock.iter().filter_map(|(item, count)| {
        let (lot, emeralds) = bundle(item)?;
        let uses = (*count / lot) as i32;
        (uses > 0).then(|| Tag::Compound(Compound::from([
            ("buy".into(), Tag::Compound(Compound::from([("id".into(), Tag::String(WAGE.into())), ("count".into(), Tag::Int(emeralds as i32))]))),
            ("sell".into(), Tag::Compound(Compound::from([("id".into(), Tag::String(item.as_str().into())), ("count".into(), Tag::Int(lot as i32))]))),
            ("maxUses".into(), Tag::Int(uses)),
            ("xp".into(), Tag::Int(0)),
            ("rewardExp".into(), Tag::Byte(0)),
        ])))
    }).collect();
    Tag::Compound(Compound::from([("Recipes".into(), Tag::List(recipes))]))
}

fn stock_signature(stock: &[(String, u32)], open: bool) -> i32 {
    let mut hash: u32 = u32::from(open);
    for (item, count) in stock {
        for byte in item.bytes() {
            hash = hash.wrapping_mul(31).wrapping_add(u32::from(byte));
        }
        hash = hash.wrapping_mul(31).wrapping_add(*count);
    }
    (hash & 0x3fff_ffff) as i32
}

/// Turno da loja (a cada ciclo do posto): registra, publica e atualiza as ofertas do comerciante.
pub(crate) fn shop_cycle(host: &mut dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity, time: i64) {
    let now = defense::clock();
    let dimension = host.dimension().to_owned();
    let merchant = worker(entity).and_then(cached_entity);
    let open = merchant.is_some() && number(entity, "Status") == 1 && number(entity, "Present") == 1;
    if !matches!(number(entity, "Status"), 1 | WALKING) {
        set_number(entity, "Present", 0);
    }
    let goods = stock(entity);
    // Vendas do dia: zera no começo de cada dia.
    let today = time.div_euclid(DAY) as i32;
    if number(entity, "Day") != today {
        set_number(entity, "Day", today);
        set_number(entity, "SoldToday", 0);
    }
    let signature = stock_signature(&goods, open);
    if let Some(id) = merchant
        && (signature != number(entity, "OfferSig") || number(entity, "OfferFor") as u32 != id)
        && let Some(handle) = host.entity(id)
    {
        let list = if open { offers(&goods) } else { offers(&[]) };
        if host.entity_merge_nbt(handle, Compound::from([("Offers".into(), list)])) || !open {
            set_number(entity, "OfferSig", signature);
            set_number(entity, "OfferFor", id as i32);
        }
    }
    set_number(entity, "Rate", goods.iter().map(|(_, count)| *count as i32).sum());
    set_number(entity, "Total", number(entity, "SoldToday"));
    set_number(entity, "Info", number(entity, "Till"));
    upsert(&SHOPS, Shop { dimension: dimension.clone(), pos, merchant, open, stock: goods, seen: now },
        |known| known.dimension == dimension && known.pos == pos, |known| now.saturating_sub(known.seen) <= STALE);
}

/// O jogador trocou com um aldeão. Se for um comerciante do Veio, o item sai do estoque da loja e
/// as esmeraldas entram no caixa.
pub(crate) fn traded(host: &mut dyn Host, merchant: u32, offer: &MerchantOffer) {
    let now = defense::clock();
    let dimension = host.dimension().to_owned();
    let Some(shop) = SHOPS.lock().ok().and_then(|shops| shops.iter()
        .find(|shop| shop.merchant == Some(merchant) && shop.dimension == dimension && now.saturating_sub(shop.seen) <= STALE)
        .map(|shop| shop.pos)) else { return };
    let item = offer.result.item.to_string();
    let count = offer.result.count;
    let paid = offer.base_cost_a.stack.count as i32 * COINS_PER_EMERALD;
    let mut moved = false;
    host.change_mod_block_entity(shop, &mut |entity| {
        if entity.id != SHOP {
            return;
        }
        let mut left = count;
        for slot in STOCK_SLOTS {
            if left == 0 {
                break;
            }
            if entity.items[slot].as_ref().is_some_and(|stack| stack.item.as_ref() == item && stack.extra.is_none()) {
                let taken = left.min(entity.items[slot].as_ref().map_or(0, |stack| stack.count));
                consume(&mut entity.items[slot], taken);
                left -= taken;
            }
        }
        credit_till(entity, paid);
        set_number(entity, "SoldToday", number(entity, "SoldToday").saturating_add(paid));
        // Força a atualização das ofertas no próximo ciclo.
        set_number(entity, "OfferSig", -1);
        moved = true;
    });
    if moved {
        let time = host.level(&dimension).and_then(|level| level.game_time(host)).unwrap_or(0);
        record(&dimension, shop, time, paid, true);
    }
}

// ---------- Prefeitura ----------

/// Indicadores de uma área (prefeitura ou bairro).
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct Indicators {
    pub population: i32,
    pub adults: i32,
    pub kids: i32,
    pub families: i32,
    pub homeless: i32,
    pub employed: i32,
    pub employment: i32,
    pub idh: i32,
    pub gini: i32,
    pub hungry: i32,
    pub no_sanitation: i32,
    pub no_power: i32,
    pub no_commerce: i32,
    pub crowded: i32,
    pub smog: i32,
    pub unlicensed: i32,
    pub districts: i32,
}

/// Gini (centésimos) da renda por pessoa, cada grupo com seu peso: (renda por pessoa, pessoas).
pub(crate) fn gini(groups: &[(f64, f64)]) -> i32 {
    let total: f64 = groups.iter().map(|(_, people)| people).sum();
    let mean = groups.iter().map(|(income, people)| income * people).sum::<f64>() / total.max(1.0);
    if total <= 0.0 || mean <= 0.0 {
        return 0;
    }
    let mut sum = 0.0;
    for (a, wa) in groups {
        for (b, wb) in groups {
            sum += wa * wb * (a - b).abs();
        }
    }
    (sum / (2.0 * total * total * mean) * 100.0).round() as i32
}

/// Bairros: grupos de casas a até `DISTRICT_RADIUS` blocos (cada casa numa componente).
pub(crate) fn districts(homes: &[Home]) -> Vec<Vec<usize>> {
    let mut group = vec![usize::MAX; homes.len()];
    let mut out = Vec::new();
    for start in 0..homes.len() {
        if group[start] != usize::MAX {
            continue;
        }
        let id = out.len();
        let mut members = vec![start];
        group[start] = id;
        let mut index = 0;
        while index < members.len() {
            let at = homes[members[index]].pos;
            for other in 0..homes.len() {
                if group[other] == usize::MAX && within(homes[other].pos, at, DISTRICT_RADIUS) {
                    group[other] = id;
                    members.push(other);
                }
            }
            index += 1;
        }
        out.push(members);
    }
    out
}

pub(crate) fn measure(homes: &[Home], homeless: i32) -> Indicators {
    let mut out = Indicators { homeless, ..Indicators::default() };
    let mut weighted = 0.0;
    let mut groups = Vec::new();
    for home in homes.iter().filter(|home| home.residents > 0) {
        out.population += home.residents;
        out.adults += home.adults;
        out.kids += home.kids;
        out.families += 1;
        out.employed += home.employed;
        weighted += f64::from(home.idh()) * f64::from(home.residents);
        groups.push((f64::from(home.income) / f64::from(home.residents), f64::from(home.residents)));
        out.hungry += i32::from(home.hungry);
        out.no_sanitation += i32::from(!home.sanitation);
        out.no_power += i32::from(!home.power);
        out.no_commerce += i32::from(!home.commerce);
        out.crowded += i32::from(home.crowded);
        out.smog += i32::from(home.smog);
    }
    out.unlicensed = homes.iter().filter(|home| !home.licensed).count() as i32;
    out.districts = districts(&homes.iter().filter(|home| home.residents > 0).cloned().collect::<Vec<_>>()).len() as i32;
    // Sem-teto: IDH mínimo (5 em cada dimensão) e renda zero.
    let people = out.population + homeless;
    weighted += 5.0 * f64::from(homeless);
    if homeless > 0 {
        groups.push((0.0, f64::from(homeless)));
    }
    out.population = people;
    out.idh = if people == 0 { 0 } else { (weighted / f64::from(people)).round() as i32 };
    out.gini = gini(&groups);
    out.employment = if out.adults == 0 { 0 } else { out.employed * 100 / out.adults };
    out
}

/// A próxima fase que estes números alcançam (a partir da atual, uma por vez).
pub(crate) fn next_phase(phase: i32, numbers: &Indicators, gdp: i32) -> i32 {
    let index = (phase - 1) as usize;
    let Some((residents, families, idh, need_gdp, employment, gini, districts)) = PHASES.get(index).copied() else {
        return phase;
    };
    let ok = numbers.population - numbers.homeless >= residents && numbers.families >= families && numbers.idh >= idh
        && gdp >= need_gdp && numbers.employment >= employment && numbers.gini <= gini && numbers.districts >= districts;
    if ok { phase + 1 } else { phase }
}

/// Pedido do prefeito: (código, número). 0 = tudo certo, 10 = meta da próxima fase.
pub(crate) fn request(numbers: &Indicators, owed: bool, phase: i32) -> (i32, i32) {
    let unemployed = numbers.adults - numbers.employed;
    if owed { return (1, 0); }
    if numbers.hungry > 0 { return (2, numbers.hungry); }
    if numbers.homeless > 0 { return (3, numbers.homeless); }
    if numbers.no_sanitation > 0 { return (4, numbers.no_sanitation); }
    if numbers.no_commerce > 0 { return (5, numbers.no_commerce); }
    if phase >= 2 && numbers.no_power > 0 { return (6, numbers.no_power); }
    if unemployed > 0 { return (7, unemployed); }
    if numbers.smog > 0 { return (8, numbers.smog); }
    if numbers.crowded > 0 { return (9, numbers.crowded); }
    if phase < 4 { return (10, phase + 1); }
    (0, 0)
}

/// Turno do prefeito: cofre, indicadores, fase, pedido. Chamado a cada ciclo do posto; o cálculo
/// pesado só roda uma vez por turno.
pub(crate) fn hall_cycle(host: &mut dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity, time: i64) {
    let now = defense::clock();
    let dimension = host.dimension().to_owned();
    // O governo segue de noite e enquanto o prefeito anda; para sem prefeito, ausente ou com fome.
    let staffed = worker(entity).is_some() && !matches!(number(entity, "Status"), NO_WORKER | ABSENT | NO_FOOD);
    // Cofre: a memória manda; ao reabrir o mundo, volta do NBT.
    let mut coins = treasury(&dimension, pos).unwrap_or_else(|| number(entity, "Treasury"));
    if let Some(stack) = entity.items[1].as_ref().filter(|stack| stack.item.as_ref() == WAGE && stack.extra.is_none()) {
        coins = coins.saturating_add(stack.count as i32 * COINS_PER_EMERALD);
        entity.items[1] = None;
    }
    set_treasury(&dimension, pos, coins);
    set_number(entity, "Treasury", coins);
    let phase = number(entity, "Phase").max(1);
    set_number(entity, "Phase", phase);
    let turn = (time + i64::from((pos[0] ^ pos[2]) & 63)).rem_euclid(TURN) < EVERY || !entity.nbt.contains_key(nbt_key("DPop"));
    if turn {
        let homes = homes_near(&dimension, pos, HALL_RADIUS, now);
        let housed = housed(&dimension, now);
        let at = center(pos);
        let radius = f64::from(HALL_RADIUS);
        let homeless = production::ranch::villager_positions(host, &dimension).iter()
            .filter(|(id, spot)| (spot[0] - at[0]).abs() <= radius && (spot[2] - at[2]).abs() <= radius && !housed.contains(id))
            .count() as i32;
        let numbers = measure(&homes, homeless);
        let (made, sold) = output(&dimension, pos, HALL_RADIUS, time);
        let gdp = made + sold;
        let owed = homes.iter().any(|home| home.employed > 0) && coins < WAGE_PER_TURN;
        let promoted = if staffed { next_phase(phase, &numbers, gdp) } else { phase };
        if promoted > phase {
            set_number(entity, "Phase", promoted);
            notify(host, pos, &format!("veio.mayor.phase{promoted}"));
            host.play_sound("minecraft:ui.toast.challenge_complete", center(pos), 4, 0.8, 1.0);
        }
        let phase = promoted;
        // Licença suspensa: o IDH caiu bem abaixo do que a fase atual exigiu.
        let required = if phase >= 2 { PHASES[(phase - 2) as usize].2 } else { 0 };
        let suspended = numbers.population > 0 && numbers.idh + 5 < required;
        let (code, value) = request(&numbers, owed, phase);
        for (key, number) in [
            ("DPop", numbers.population), ("DAdults", numbers.adults), ("DKids", numbers.kids),
            ("DFamilies", numbers.families), ("DEmploy", numbers.employment), ("DIdh", numbers.idh),
            ("DGini", numbers.gini), ("DGdp", gdp), ("DRequest", code), ("DRequestN", value),
            ("DSuspended", i32::from(suspended)), ("DHomeless", numbers.homeless), ("DDistricts", numbers.districts),
            ("DMade", made), ("DSold", sold),
        ] {
            set_signed(entity, key, number);
        }
    }
    let phase = number(entity, "Phase").max(1);
    upsert(&HALLS, Hall {
        dimension: dimension.clone(), pos, phase, suspended: number(entity, "DSuspended") != 0, staffed, seen: now,
    }, |known| known.dimension == dimension && known.pos == pos, |known| now.saturating_sub(known.seen) <= STALE);
}

/// Os 13 valores do painel da prefeitura.
pub(crate) fn hall_data(entity: &ModBlockEntity) -> [i32; 13] {
    [number(entity, "Status"), number(entity, "Phase").max(1), signed(entity, "DPop"), signed(entity, "DAdults"),
        signed(entity, "DKids"), signed(entity, "DFamilies"), signed(entity, "DEmploy"), signed(entity, "DIdh"),
        signed(entity, "DGini"), signed(entity, "DGdp"), number(entity, "Treasury"),
        signed(entity, "DRequest") * 1000 + signed(entity, "DRequestN").clamp(0, 999), signed(entity, "DSuspended")]
}

fn decimal(hundredths: i32) -> String {
    format!("{},{:02}", hundredths / 100, hundredths % 100)
}

/// Tela da prefeitura: a fase e os indicadores.
pub(crate) fn draw_screen(host: &dyn Host, event: &mut horizon_client::mod_api::ScreenDrawEvent<'_>) {
    let [x, y] = event.origin;
    let read = |index: usize| event.data.get(index).copied().unwrap_or(0);
    let tr = |key: &str, args: &[&dyn std::fmt::Display]| Text::tr(key, args).plain();
    let status = read(0).clamp(0, presentation::STATUS_KEYS.len() as i32 - 1) as usize;
    let phase = read(1).clamp(1, 4);
    let request = read(11);
    let suspended = read(12) != 0;
    let lines = [
        (tr(&format!("veio.status.{}", presentation::STATUS_KEYS[status]), &[]), presentation::text_color(status)),
        (tr(&format!("veio.mayor.phase_name{phase}"), &[]), 0x1f4fa8),
        (tr("veio.mayor.population", &[&read(2), &read(3), &read(4), &read(5)]), presentation::INK),
        (tr("veio.mayor.employment", &[&read(6)]), presentation::INK),
        (tr("veio.mayor.idh", &[&decimal(read(7)), &decimal(read(8))]), if read(7) < 50 { 0x9a5a00 } else { 0x1e7a1e }),
        (tr("veio.mayor.gdp", &[&read(9), &read(10)]), presentation::INK),
        (if suspended { tr("veio.mayor.suspended", &[]) } else { tr(&format!("veio.mayor.request{}", request / 1000), &[&(request % 1000)]) },
            if suspended || request / 1000 != 0 { 0xaa1f1f } else { 0x1e7a1e }),
    ];
    for (index, (line, color)) in lines.iter().enumerate() {
        host.text(event.batch, line, [x + 8.0, y + 22.0 + index as f32 * 11.0], *color, false);
    }
}

/// Relatório no chat (clique com livro na prefeitura).
pub(crate) fn report_to_chat(host: &mut dyn Host, pos: [i32; 3]) -> Option<Flow> {
    let mut state = None;
    host.change_mod_block_entity(pos, &mut |entity| {
        if entity.id == TOWN_HALL {
            state = Some((worker(entity).is_some(), number(entity, "Phase").max(1), number(entity, "Treasury"),
                signed(entity, "DMade"), signed(entity, "DSold")));
        }
    });
    let (staffed, phase, coins, made, sold) = state?;
    let at = center(pos);
    let dimension = host.dimension().to_owned();
    let Some(player) = host.server_players().into_iter()
        .filter(|player| player.dimension == dimension)
        .filter(|player| (0..3).map(|axis| (player.position[axis] - at[axis]).powi(2)).sum::<f64>() < 144.0)
        .map(|player| player.uuid).next() else { return Some(Flow::Cancel) };
    if !staffed {
        host.tell(player, Text::tr("veio.mayor.need_mayor", &[]));
        return Some(Flow::Cancel);
    }
    let now = defense::clock();
    let homes = homes_near(&dimension, pos, HALL_RADIUS, now);
    let housed = housed(&dimension, now);
    let radius = f64::from(HALL_RADIUS);
    let homeless = production::ranch::villager_positions(host, &dimension).iter()
        .filter(|(id, spot)| (spot[0] - at[0]).abs() <= radius && (spot[2] - at[2]).abs() <= radius && !housed.contains(id))
        .count() as i32;
    let numbers = measure(&homes, homeless);
    host.tell(player, Text::tr(&format!("veio.mayor.phase_name{phase}"), &[]));
    host.tell(player, Text::tr("veio.mayor.report.people", &[&numbers.population, &numbers.adults, &numbers.kids,
        &numbers.families, &numbers.homeless]));
    host.tell(player, Text::tr("veio.mayor.report.economy", &[&(made + sold), &made, &sold, &coins, &numbers.employment]));
    host.tell(player, Text::tr("veio.mayor.report.idh", &[&decimal(numbers.idh), &decimal(numbers.gini)]));
    // Bairros, cada um com sua população e IDH.
    let lived: Vec<Home> = homes.iter().filter(|home| home.residents > 0).cloned().collect();
    for (index, members) in districts(&lived).iter().enumerate().take(6) {
        let group: Vec<Home> = members.iter().map(|member| lived[*member].clone()).collect();
        let part = measure(&group, 0);
        let [x, _, z] = group[0].pos;
        host.tell(player, Text::tr("veio.mayor.report.district", &[&(index + 1), &format!("{x} {z}"), &part.population,
            &decimal(part.idh)]));
    }
    // Todos os problemas, não só o pior.
    let owed = homes.iter().any(|home| home.employed > 0) && coins < WAGE_PER_TURN;
    let unemployed = numbers.adults - numbers.employed;
    for (code, value) in [(1, i32::from(owed)), (2, numbers.hungry), (3, numbers.homeless), (4, numbers.no_sanitation),
        (5, numbers.no_commerce), (6, if phase >= 2 { numbers.no_power } else { 0 }), (7, unemployed), (8, numbers.smog),
        (9, numbers.crowded)] {
        if value > 0 {
            host.tell(player, Text::tr(&format!("veio.mayor.request{code}"), &[&value]));
        }
    }
    if let Some((residents, families, idh, gdp, employment, gini, districts)) = PHASES.get((phase - 1) as usize).copied() {
        host.tell(player, Text::tr("veio.mayor.report.next", &[&(phase + 1), &residents, &families, &decimal(idh), &gdp,
            &employment, &decimal(gini), &districts]));
    }
    // Global: todas as prefeituras da dimensão.
    let halls: Vec<[i32; 3]> = HALLS.lock().map(|halls| halls.iter()
        .filter(|hall| hall.dimension == dimension && now.saturating_sub(hall.seen) <= STALE)
        .map(|hall| hall.pos).collect()).unwrap_or_default();
    if halls.len() > 1 {
        let time = host.level(&dimension).and_then(|level| level.game_time(host)).unwrap_or(0);
        let all = homes_near(&dimension, pos, i32::MAX / 4, now);
        let total = measure(&all, 0);
        let gdp: i32 = halls.iter().map(|hall| { let (m, s) = output(&dimension, *hall, HALL_RADIUS, time); m + s }).sum();
        host.tell(player, Text::tr("veio.mayor.report.global", &[&halls.len(), &total.population, &gdp, &decimal(total.idh)]));
    }
    Some(Flow::Cancel)
}

/// Lista de empregados vivos (ids de entidade), lida do registro de atribuições.
fn employed_ids(now: u64) -> Vec<u32> {
    super::employed_entities(now)
}

/// Painel padrão da casa, da loja e do poço.
pub(crate) fn panel_lines(block: &str, data: &[i32]) -> Option<Vec<String>> {
    let read = |index: usize| data.get(index).copied().unwrap_or(0);
    let tr = |key: &str, args: &[&dyn std::fmt::Display]| Text::tr(key, args).plain();
    match block {
        HOUSE => Some(vec![
            tr("veio.house.residents", &[&read(6), &read(7), &decimal(read(12))]),
            tr("veio.house.wallet", &[&read(9), &read(2)]),
            tr(&format!("veio.house.need{}", read(1)), &[]),
        ]),
        SHOP => Some(vec![
            tr("veio.shop.sales", &[&read(9), &read(12)]),
            tr("veio.shop.stock", &[&read(6)]),
            tr("veio.hint.shop", &[]),
        ]),
        WELL => Some(vec![tr("veio.hint.well", &[])]),
        _ => None,
    }
}

pub(crate) fn screen_slots(block: &str) -> Option<Vec<Option<[f32; 2]>>> {
    Some(match block {
        HOUSE => vec![Some([62.0, 45.0]), Some([80.0, 45.0]), Some([98.0, 45.0])],
        WELL => vec![Some([89.0, 45.0])],
        SHOP => vec![Some([17.0, 45.0]), Some([44.0, 45.0]), Some([62.0, 45.0]), Some([80.0, 45.0]),
            Some([98.0, 45.0]), Some([116.0, 45.0]), Some([134.0, 45.0]), Some([160.0, 45.0])],
        TOWN_HALL => vec![Some([170.0, 82.0]), Some([170.0, 104.0])],
        _ => return None,
    })
}

pub(crate) fn roles(block: &str) -> Option<&'static [&'static str]> {
    Some(match block {
        HOUSE => &["food", "", ""],
        WELL => &[""],
        SHOP => &["food", "supply", "", "", "", "", "", "output"],
        TOWN_HALL => &["food", "wage"],
        _ => return None,
    })
}
