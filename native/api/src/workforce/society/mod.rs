//! Sociedade da vila: casas e famílias, contas da companhia e públicas, lojas com preços locais,
//! prefeito e indicadores. Desenho em `frentes/SOCIEDADE.md`.
//!
//! - **Casa** (`HOUSE`, aqui): até `CAPACITY` moradores (aldeões a até `HOME_RADIUS` blocos, cada
//!   um numa casa só). A cada turno recebe os salários (da companhia ou, para o prefeito, da
//!   prefeitura), come da despensa, gasta conforto e compra o que falta na loja mais próxima.
//! - **Contas** (`ledger`): a da companhia (cofre do centro de controle) e a pública (prefeitura).
//! - **Lojas** (`market`): estoque, preços por oferta e procura, troca com o jogador.
//! - **Prefeito** (`mayor`): impostos, manutenção, assistência, indicadores, fases, relatório.
//! - **Poço** (`WELL`, aqui): saneamento; enche baldes.
//!
//! Custo: casas e prefeituras trabalham uma vez por turno (1 min) com registros em memória e o
//! censo de posições; a loja só reescreve as ofertas do comerciante quando estoque ou preço mudam.
use super::*;
use std::sync::atomic::{AtomicBool, Ordering};

pub(crate) mod ledger;
pub(crate) mod market;
pub(crate) mod mayor;
pub(crate) mod trade;

pub(crate) use ledger::company_cycle;
pub(crate) use market::{shop_cycle, traded};
pub(crate) use mayor::{draw_screen, hall_cycle, hall_data, report_to_chat};
#[cfg(test)]
pub(crate) use market::bundle;
#[cfg(test)]
pub(crate) use mayor::{measure, next_phase, request};

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
/// Comida por morador por turno (20 pontos por dia: pouco mais de 3 pães).
pub(crate) const FOOD_PER_TURN: i32 = 1;
/// Conforto gasto por morador por turno; bens repõem.
pub(crate) const COMFORT_PER_TURN: i32 = 1;
/// Custo de vida por morador por dia (comida + bens), base do índice de renda.
pub(crate) const LIVING_COST: i32 = 30;
/// Assistência pública: 1 moeda por morador por turno às famílias pobres, se o cofre passar disto.
pub(crate) const ASSIST_RESERVE: i32 = 200;

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
pub(crate) const STALE: u64 = 3000;

/// Bens de consumo (conforto): (item, preço base em moedas, pontos de conforto).
pub(crate) const GOODS: [(&str, i32, i32); 12] = [
    ("minecraft:white_wool", 4, 8), ("minecraft:leather", 6, 12), ("minecraft:paper", 2, 4),
    ("minecraft:book", 10, 20), ("minecraft:candle", 4, 8), ("minecraft:torch", 1, 2),
    ("minecraft:glass_bottle", 2, 4), ("minecraft:flower_pot", 3, 6), ("minecraft:honey_bottle", 5, 10),
    ("minecraft:painting", 8, 16), ("minecraft:lantern", 6, 12), ("minecraft:feather", 1, 2),
];

pub(crate) fn is_society_block(block: &str) -> bool {
    matches!(block, HOUSE | WELL | trade::PORT | trade::STATION)
}

pub(crate) fn goods(item: &str) -> Option<(i32, i32)> {
    GOODS.iter().find(|(known, _, _)| *known == item).map(|(_, price, comfort)| (*price, *comfort))
}

fn is_goods(item: &str) -> bool {
    goods(item).is_some()
}

pub(crate) fn layout(block: &str) -> Option<(usize, Vec<usize>, Vec<usize>)> {
    Some(match block {
        HOUSE => (PANTRY_SLOTS, (0..PANTRY_SLOTS).collect(), vec![]),
        WELL => (1, vec![0], vec![0]),
        // Comida do comerciante; estoque; antigo caixa (só retirar).
        SHOP => (8, (0..7).collect(), vec![]),
        TOWN_HALL => (2, vec![0, 1], vec![]),
        trade::PORT | trade::STATION => (trade::SLOTS, (0..trade::SLOTS).collect(), vec![]),
        _ => return None,
    })
}

pub(crate) fn accepts(block: &str, slot: usize, stack: &ItemStack) -> bool {
    match (block, slot) {
        (HOUSE, 0..PANTRY_SLOTS) => is_food(&stack.item),
        (WELL, 0) => stack.item.as_ref() == "minecraft:bucket",
        (SHOP, 0) => is_food(&stack.item),
        (SHOP, 1..7) => market::base_price(&stack.item).is_some(),
        (TOWN_HALL, 0) => is_food(&stack.item),
        (TOWN_HALL, 1) => stack.item.as_ref() == WAGE,
        (trade::PORT | trade::STATION, 0..trade::SLOTS) => trade::product(&stack.item).is_some(),
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
    /// A companhia está devendo salário a esta família.
    pub owed: bool,
    /// Carteira da família, em moedas (para a identidade contábil).
    pub wallet: i32,
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
    /// Índice de preços da comida, em %.
    pub prices: i32,
    /// Caixa do comerciante, em moedas.
    pub cash: i32,
    pub seen: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Hall {
    pub dimension: String,
    pub pos: [i32; 3],
    pub phase: i32,
    pub suspended: bool,
    pub staffed: bool,
    /// O prefeito (funcionário público: salário da conta pública).
    pub mayor: Option<u32>,
    pub seen: u64,
}

static HOMES: Mutex<Vec<Home>> = Mutex::new(Vec::new());
static SHOPS: Mutex<Vec<Shop>> = Mutex::new(Vec::new());
static HALLS: Mutex<Vec<Hall>> = Mutex::new(Vec::new());
static WELLS: Mutex<Vec<(String, [i32; 3], u64)>> = Mutex::new(Vec::new());
/// Morador -> casa (dimensão, posição, relógio).
static RESIDENTS: Mutex<Vec<(u32, String, [i32; 3], u64)>> = Mutex::new(Vec::new());
static ACTIVE: AtomicBool = AtomicBool::new(false);

pub(crate) fn within(a: [i32; 3], b: [i32; 3], radius: i32) -> bool {
    (a[0] - b[0]).abs() <= radius && (a[2] - b[2]).abs() <= radius && (a[1] - b[1]).abs() <= radius.max(16)
}

fn upsert<T>(list: &Mutex<Vec<T>>, item: T, same: impl Fn(&T) -> bool, fresh: impl Fn(&T) -> bool) {
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

/// Numa passada só: casas vizinhas (lotação) e casas com alvará construídas antes desta.
fn neighbors(dimension: &str, pos: [i32; 3], anchor: [i32; 3], built: i64, now: u64) -> (usize, usize) {
    if !ACTIVE.load(Ordering::Acquire) {
        return (0, 0);
    }
    HOMES.lock().map(|homes| homes.iter()
        .filter(|home| home.dimension == dimension && home.pos != pos && now.saturating_sub(home.seen) <= STALE)
        .fold((0, 0), |(crowd, earlier), home| (
            crowd + usize::from(within(home.pos, pos, CROWD_RADIUS)),
            earlier + usize::from(home.licensed && within(home.pos, anchor, HALL_RADIUS) && (home.built, home.pos) < (built, pos)),
        ))).unwrap_or((0, 0))
}

pub(crate) fn shops_near(dimension: &str, at: [i32; 3], radius: i32, now: u64) -> Vec<Shop> {
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

/// Ids dos prefeitos (funcionários públicos) desta dimensão.
fn public_servants(dimension: &str, now: u64) -> Vec<u32> {
    HALLS.lock().map(|halls| halls.iter()
        .filter(|hall| hall.dimension == dimension && now.saturating_sub(hall.seen) <= STALE)
        .filter_map(|hall| hall.mayor).collect()).unwrap_or_default()
}

/// Reivindica o aldeão como morador desta casa. `false` se ele já mora em outra.
fn settle(id: u32, dimension: &str, pos: [i32; 3], now: u64) -> bool {
    let Ok(mut residents) = RESIDENTS.lock() else { return false };
    if let Some(entry) = residents.iter_mut().find(|(known, dim, _, _)| *known == id && dim == dimension) {
        let mine = entry.2 == pos;
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

/// Moradores registrados nesta dimensão (para contar os sem-teto).
pub(crate) fn housed(dimension: &str, now: u64) -> Vec<u32> {
    RESIDENTS.lock().map(|residents| residents.iter()
        .filter(|(_, dim, _, seen)| dim == dimension && now.saturating_sub(*seen) <= STALE)
        .map(|(id, _, _, _)| *id).collect()).unwrap_or_default()
}

// ---------- Plano diretor ----------

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

/// Até que nível uma zona em `at` pode subir agora, e se as licenças estão suspensas.
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
            HOMES.lock().is_ok_and(|homes| homes.iter().any(|home| home.dimension == dimension && home.licensed
                && (home.residents as usize) < CAPACITY && now.saturating_sub(home.seen) <= STALE
                && within(home.pos, at, HALL_RADIUS)))
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
        let Some((slot, value)) = (0..PANTRY_SLOTS.min(entity.items.len())).find_map(|slot| entity.items[slot].as_ref()
            .and_then(|stack| production::pantry::value(&stack.item)).map(|value| (slot, value))) else { break };
        consume(&mut entity.items[slot], 1);
        fed += value as i32;
    }
    set_number(entity, "Fed", (fed - need).max(0));
    (need - fed).max(0)
}

/// Põe `count` de `item` na despensa; devolve quantos couberam.
fn stock_pantry(entity: &mut ModBlockEntity, item: &str, count: u32) -> u32 {
    let mut left = count;
    for slot in 0..PANTRY_SLOTS.min(entity.items.len()) {
        let room = entity.items[slot].as_ref().map_or(64, |stack| {
            if stack.item.as_ref() == item && stack.extra.is_none() { 64 - stack.count.min(64) } else { 0 }
        });
        let put = room.min(left);
        if put > 0 {
            add(&mut entity.items[slot], item, put);
            left -= put;
        }
    }
    count - left
}

fn signed(entity: &ModBlockEntity, key: &str) -> i32 {
    match entity.nbt.get(nbt_key(key)) { Some(Tag::Int(value)) => *value, _ => 0 }
}

/// Estados extras do painel.
pub(crate) const HOUSED: i32 = 29;
pub(crate) const NO_PERMIT: i32 = 30;
pub(crate) const EMPTY_HOME: i32 = 31;

/// Necessidade mais urgente da casa (dica do painel): 0 nada, 1 fome, 2 renda, 3 saneamento,
/// 4 energia, 5 comércio, 6 ar, 7 lotação, 8 sem alvará, 9 sem prefeitura, 10 salário atrasado.
fn worst_need(home: &Home, governed: bool) -> i32 {
    if !home.licensed { return 8; }
    if home.residents == 0 { return 0; }
    if home.hungry { return 1; }
    if home.owed { return 10; }
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
        trade::PORT | trade::STATION => trade::tick(host, pos, entity, time),
        WELL => {
            if phase % 20 == 0 {
                let dimension = host.dimension().to_owned();
                let now = defense::clock();
                upsert(&WELLS, (dimension.clone(), pos, now), |(dim, at, _)| *dim == dimension && *at == pos,
                    |(_, _, seen)| now.saturating_sub(*seen) <= STALE);
                // Balde vazio vira balde d'água (um por vez: o d'água não empilha).
                if entity.items.first().and_then(Option::as_ref)
                    .is_some_and(|stack| stack.item.as_ref() == "minecraft:bucket" && stack.count == 1)
                {
                    entity.items[0] = Some(ItemStack::new("minecraft:water_bucket", 1));
                }
                set_number(entity, "Status", ZONE_ACTIVE);
            }
        }
        HOUSE => {
            if !entity.nbt.contains_key(nbt_key("Built")) {
                entity.nbt.insert("Built".into(), Tag::Int(time.clamp(0, i64::from(i32::MAX)) as i32));
            }
            if phase == 0 || !entity.nbt.contains_key(nbt_key("Turned")) {
                set_number(entity, "Turned", 1);
                house_turn(host, pos, entity, time);
            }
        }
        _ => {}
    }
}

/// Salários do turno, cada um pago por quem emprega: o prefeito pela prefeitura, o comerciante
/// pela própria loja e os demais pela companhia (custo lançado na empresa do posto). O que a
/// companhia não pagar fica devendo (até um dia). Devolve (pago, a companhia deve).
fn wages(dimension: &str, pos: [i32; 3], entity: &mut ModBlockEntity, residents: &[u32], hall: Option<&Hall>, now: u64) -> (i32, bool) {
    use ledger::Book;
    let employers = employers(dimension, now);
    let servants = public_servants(dimension, now);
    let merchants = merchants(dimension, now);
    let mut paid = 0;
    let mut public_jobs = 0;
    let mut company: Vec<u8> = Vec::new();
    let mut employed = 0;
    for id in residents {
        if servants.contains(id) {
            public_jobs += 1;
        } else if let Some((_, shop)) = merchants.iter().find(|(merchant, _)| merchant == id) {
            employed += 1;
            paid += ledger::take(Book::Shop, dimension, *shop, 0, WAGE_PER_TURN, ledger::WAGES, now);
        } else if let Some((_, post)) = employers.iter().find(|(worker, _)| worker == id) {
            company.push(super::super::holding::division_at(dimension, *post));
        }
    }
    let cap = WAGE_PER_TURN * TURNS_PER_DAY as i32;
    let jobs = company.len() as i32;
    let company_due = jobs * WAGE_PER_TURN + number(entity, "Owed");
    let company_paid = ledger::take(Book::Company, dimension, pos, ledger::ANYWHERE, company_due, ledger::WAGES, now);
    // O custo vai para a empresa de cada trabalhador; o atrasado, para a primeira.
    let mut left = company_paid;
    for (index, division) in company.iter().enumerate() {
        let share = if index + 1 == company.len() { left } else { left.min(WAGE_PER_TURN) };
        super::super::holding::book(dimension, *division, 0, share);
        left -= share;
    }
    set_number(entity, "Owed", (company_due - company_paid).min(jobs * cap));
    let public_due = public_jobs * WAGE_PER_TURN + number(entity, "OwedPublic");
    let public_paid = hall.map_or(0, |hall| ledger::take(Book::Public, dimension, hall.pos, 0, public_due, ledger::WAGES, now));
    set_number(entity, "OwedPublic", (public_due - public_paid).min(public_jobs * cap));
    set_number(entity, "Employed", employed + public_jobs + jobs);
    (paid + company_paid + public_paid, company_due > company_paid)
}

/// Comerciantes e a loja de cada um.
fn merchants(dimension: &str, now: u64) -> Vec<(u32, [i32; 3])> {
    SHOPS.lock().map(|shops| shops.iter()
        .filter(|shop| shop.dimension == dimension && now.saturating_sub(shop.seen) <= STALE)
        .filter_map(|shop| shop.merchant.map(|id| (id, shop.pos))).collect()).unwrap_or_default()
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
    let anchor = hall.as_ref().map_or(pos, |hall| hall.pos);
    let (crowd, earlier) = neighbors(&dimension, pos, anchor, built, now);
    let crowded = crowd > CROWD_LIMIT;
    let smog_level = pollution::local(&dimension, pos);
    let smog = smog_level > 400.0;
    let licensed = earlier < permits(phase) && meets_standard(phase, sanitation, power, commerce);
    // Moradores: aldeões perto, cada um em uma casa só.
    let mut residents: Vec<u32> = Vec::new();
    if licensed {
        for (id, spot) in production::ranch::villager_positions(host, &dimension).iter() {
            if residents.len() >= CAPACITY {
                break;
            }
            if near(*spot, at, HOME_RADIUS) && settle(*id, &dimension, pos, now) {
                residents.push(*id);
            }
        }
    }
    leave(&dimension, pos, &residents);
    // Idade: um NBT por morador só quando a família muda (a cada dia, por garantia).
    let family = residents.iter().fold(0u32, |hash, id| hash.wrapping_mul(31).wrapping_add(*id)) as i32 & 0x3fff_ffff;
    let kids = if number(entity, "Family") == family && time.rem_euclid(DAY) >= TURN {
        number(entity, "Kids")
    } else {
        let kids = residents.iter().filter(|id| host.entity(**id).and_then(|handle| host.entity_nbt(handle))
            .is_some_and(|nbt| production::ranch::tag_int(&nbt, "Age") < 0)).count() as i32;
        set_number(entity, "Family", family);
        set_number(entity, "Kids", kids);
        kids
    };
    let people = residents.len() as i32;
    let adults = people - kids;
    let (paid, owed) = wages(&dimension, pos, entity, &residents, hall.as_ref(), now);
    let employed = number(entity, "Employed");
    let mut wallet = number(entity, "Wallet").saturating_add(paid);
    // Assistência pública às famílias pobres, se o cofre da prefeitura tiver folga.
    if let Some(hall) = &hall
        && people > 0 && number(entity, "Wealth") < 40
        && ledger::balance(ledger::Book::Public, &dimension, hall.pos).is_some_and(|coins| coins > ASSIST_RESERVE)
    {
        wallet += ledger::take(ledger::Book::Public, &dimension, hall.pos, 0, people, ledger::ASSIST, now);
    }
    let income = ledger::daily(number(entity, "Income"), wallet - number(entity, "Wallet"));
    set_number(entity, "Income", income);
    // Comida: cada morador come; o que falta vira fome.
    let missing = eat_pantry(entity, people * FOOD_PER_TURN);
    let hungry_turns = if missing > 0 { number(entity, "Hungry") + 1 } else { (number(entity, "Hungry") - 1).max(0) };
    set_number(entity, "Hungry", hungry_turns.min(TURNS_PER_DAY as i32));
    let mut comfort = (number(entity, "Comfort") - people * COMFORT_PER_TURN).max(0);
    // Compras de dia, na loja aberta mais próxima, a preço local.
    let target = people * FOOD_PER_TURN * TURNS_PER_DAY as i32;
    let mut errand = None;
    if !night(host) && people > 0 && let Some(shop) = shops.iter().filter(|shop| shop.open)
        .min_by_key(|shop| (shop.pos[0] - pos[0]).abs() + (shop.pos[2] - pos[2]).abs())
    {
        let have = pantry_points(entity);
        if have < target / 2 {
            // Quer cobrir um dia; a falta (sem dinheiro ou sem estoque) empurra o preço da loja.
            let wanted = ((target - have) / 6).max(1) as u32;
            let room = (0..PANTRY_SLOTS.min(entity.items.len()))
                .map(|slot| entity.items[slot].as_ref().map_or(64, |stack| 64 - stack.count.min(64))).sum::<u32>();
            for (item, count, cost) in market::buy(host, shop.pos, is_food, wallet, room.min(16), wanted, now) {
                stock_pantry(entity, &item, count);
                wallet -= cost;
                errand = Some(shop.pos);
            }
        }
        if comfort < people * 10 && wallet > 0 {
            for (item, count, cost) in market::buy(host, shop.pos, is_goods, wallet / 2, 2, 0, now) {
                comfort += goods(&item).map_or(0, |(_, points)| points) * count as i32;
                wallet -= cost;
                errand = Some(shop.pos);
            }
        }
    }
    set_number(entity, "Comfort", comfort.min(people.max(1) * 40));
    set_number(entity, "Wallet", wallet.max(0));
    // Um morador sem emprego vai até a loja (só visual: a compra já foi feita).
    if let Some(shop) = errand {
        let employed_now = employers(&dimension, now);
        if let Some(id) = residents.iter().find(|id| !employed_now.iter().any(|(known, _)| known == *id) && order_for(**id, now).is_none()) {
            order(*id, post_center(shop), now + ERRAND);
        }
    }
    // Índices.
    let hungry = hungry_turns > 2;
    let food_index = if people == 0 { 100 } else {
        let fed = 100 - hungry_turns * 100 / TURNS_PER_DAY as i32;
        fed * (70 + 10 * variety(entity).min(3)) / 100
    };
    let mut housing = if licensed { 100 } else { 0 };
    if crowded { housing = housing * 70 / 100; }
    if smog { housing = housing * if smog_level > 1200.0 { 30 } else { 60 } / 100; }
    let services = (i32::from(sanitation) + i32::from(power) + i32::from(commerce)) * 100 / 3;
    let comfort_index = if people == 0 { 100 } else { (comfort * 100 / (people * 10)).min(100) };
    let wealth = if people == 0 { 100 } else { (income * 100 / (people * LIVING_COST)).min(100) * (50 + comfort_index / 2) / 100 };
    set_number(entity, "Wealth", wealth.clamp(0, 100));
    let home = Home {
        dimension: dimension.clone(), pos, built, licensed, residents: people, adults, kids, employed, income, hungry,
        owed, wallet: wallet.max(0), sanitation, power, commerce, crowded, smog, food: food_index.clamp(0, 100), housing, services,
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

// ---------- Comerciante e prefeito ----------

/// O comerciante e o prefeito ficam no posto.
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

// ---------- Painéis ----------

/// Painel padrão da casa, da loja e do poço.
pub(crate) fn panel_lines(block: &str, data: &[i32]) -> Option<Vec<String>> {
    let read = |index: usize| data.get(index).copied().unwrap_or(0);
    let tr = |key: &str, args: &[&dyn std::fmt::Display]| Text::tr(key, args).plain();
    match block {
        HOUSE => Some(vec![
            tr("veio.house.residents", &[&read(6), &read(7), &mayor::decimal(read(12))]),
            tr("veio.house.wallet", &[&read(9), &read(2)]),
            tr(&format!("veio.house.need{}", read(1)), &[]),
        ]),
        SHOP => Some(vec![
            tr("veio.shop.sales", &[&read(9), &read(12)]),
            tr("veio.shop.stock", &[&read(6), &read(7)]),
            if read(2) > 1 { tr("veio.shop.owes", &[&read(2)]) } else { tr("veio.hint.shop", &[]) },
        ]),
        WELL => Some(vec![tr("veio.hint.well", &[])]),
        trade::PORT | trade::STATION => Some(vec![
            tr("veio.dock.today", &[&read(6), &read(9)]),
            tr("veio.dock.next", &[&read(12), &read(7)]),
            tr(if block == trade::PORT { "veio.hint.export_dock" } else { "veio.hint.freight_station" }, &[]),
        ]),
        _ => None,
    }
}

pub(crate) fn screen_slots(block: &str) -> Option<Vec<Option<[f32; 2]>>> {
    Some(match block {
        HOUSE => vec![Some([62.0, 45.0]), Some([80.0, 45.0]), Some([98.0, 45.0])],
        WELL => vec![Some([89.0, 45.0])],
        SHOP => vec![Some([17.0, 45.0]), Some([44.0, 45.0]), Some([62.0, 45.0]), Some([80.0, 45.0]),
            Some([98.0, 45.0]), Some([116.0, 45.0]), Some([134.0, 45.0]), Some([160.0, 45.0])],
        TOWN_HALL => vec![Some([170.0, 104.0]), Some([170.0, 122.0])],
        trade::PORT | trade::STATION => (0..trade::SLOTS).map(|index| Some([17.0 + index as f32 * 18.0, 45.0])).collect(),
        _ => return None,
    })
}

pub(crate) fn roles(block: &str) -> Option<&'static [&'static str]> {
    Some(match block {
        HOUSE => &["food", "", ""],
        WELL => &[""],
        SHOP => &["food", "supply", "", "", "", "", "", ""],
        TOWN_HALL => &["food", "wage"],
        trade::PORT | trade::STATION => &["", "", "", "", "", "", "", "", ""],
        _ => return None,
    })
}
