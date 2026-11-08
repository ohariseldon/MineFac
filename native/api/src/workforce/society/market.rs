//! Lojas e preços locais.
//!
//! Cada loja tem os seus preços: o preço base (pontos de comida, ou a tabela de bens) vezes um
//! fator de 50% a 200% por item, guardado no NBT da loja (`Prices`). Uma vez por turno o fator anda
//! um quarto do caminho até o alvo, que vem da **cobertura** do estoque: estoque / procura média
//! do dia (`Demand`, média móvel das unidades vendidas). Um dia de cobertura = 100%; menos estoque
//! que procura encarece, estoque parado barateia aos poucos (até 70%). Famílias que queriam comprar
//! e não acharam (`Short`) empurram a comida para cima. Como a procura é a das casas em volta,
//! cada vila tem os seus preços.
//!
//! **O comerciante é independente.** O que entra no estoque (à mão, funil, carregador) é a
//! indústria vendendo no atacado: a loja deve `WHOLESALE`% do preço local à companhia (`Payable`,
//! por divisão) e paga com o caixa, primeiro o mais antigo. Sem caixa, fica fiado. Cada venda
//! (varejo e atacado) tem o imposto da fase retido na fonte para a prefeitura. O comerciante
//! recebe o salário do caixa da própria loja.
//!
//! O jogador compra trocando com o comerciante: as ofertas saem do estoque com o preço local e
//! são reescritas só quando o estoque ou os preços mudam. As esmeraldas que ele paga entram na
//! economia (borda) e vão para o caixa da loja.
use super::*;
use horizon_client::mod_api::MerchantOffer;
use ledger::Book;

pub(crate) const STOCK_SLOTS: std::ops::Range<usize> = 1..7;
/// Preço de atacado, em % do preço local.
pub(crate) const WHOLESALE: i32 = 75;
/// Margem do comerciante sobre o custo de importar: o teto do preço local é esse custo + margem,
/// para que importar nunca dê prejuízo à loja.
pub(crate) const IMPORT_MARGIN: i32 = 10;
const FLOOR: i32 = 50;
const CEILING: i32 = 200;
const IDLE_FLOOR: i32 = 70;
/// Itens lembrados no NBT de preços (os do estoque e os procurados há pouco).
const REMEMBERED: usize = 12;

/// Preço base de uma unidade: comida vale os pontos, bens a tabela.
///
/// **Âncora única:** produtos da bolsa valem localmente o mesmo valor de referência do mercado
/// mundial (base x complexidade), então pão é pão dentro e fora da vila. Comida fora da bolsa vale
/// os pontos de comida x 1,5 se for preparada (o mesmo fator dos processados) ou x 1 se for crua;
/// bens de conforto, a tabela `GOODS`.
pub(crate) fn base_price(item: &str) -> Option<i32> {
    if let Some(product) = trade::product(item) {
        return Some((trade::reference_cents(product) / 100).max(1));
    }
    let name = item.split(':').nth(1).unwrap_or(item);
    let prepared = name.starts_with("cooked_") || name.starts_with("baked_") || name.ends_with("_pie")
        || name.ends_with("_stew") || name.ends_with("_soup") || matches!(name, "cookie" | "cake" | "golden_carrot" | "golden_apple");
    production::pantry::value(item)
        .map(|value| (value as i32 * if prepared { 150 } else { 100 } / 100).max(1))
        .or_else(|| goods(item).map(|(price, _)| price))
}

fn int_map(entity: &ModBlockEntity, key: &str) -> Vec<(String, i32)> {
    match entity.nbt.get(nbt_key(key)) {
        Some(Tag::Compound(map)) => map.iter()
            .filter_map(|(item, value)| match value { Tag::Int(value) => Some((item.to_string_lossy(), *value)), _ => None })
            .collect(),
        _ => Vec::new(),
    }
}

fn set_int_map(entity: &mut ModBlockEntity, key: &str, values: &[(String, i32)]) {
    let map: Compound = values.iter().map(|(item, value)| (item.as_str().into(), Tag::Int(*value))).collect();
    entity.nbt.insert(key.into(), Tag::Compound(map));
}

fn read(map: &[(String, i32)], item: &str) -> Option<i32> {
    map.iter().find(|(known, _)| known == item).map(|(_, value)| *value)
}

fn bump(map: &mut Vec<(String, i32)>, item: &str, amount: i32) {
    match map.iter_mut().find(|(known, _)| known == item) {
        Some(entry) => entry.1 = entry.1.saturating_add(amount),
        None => map.push((item.to_owned(), amount)),
    }
}

/// Fator de preço do item nesta loja, em %.
pub(crate) fn factor(entity: &ModBlockEntity, item: &str) -> i32 {
    read(&int_map(entity, "Prices"), item).unwrap_or(100).clamp(FLOOR, CEILING)
}

/// Preço local de uma unidade, em moedas (mínimo 1).
pub(crate) fn local_price(entity: &ModBlockEntity, item: &str) -> Option<i32> {
    base_price(item).map(|base| (base * factor(entity, item) / 100).max(1))
}

/// O alvo do fator: `cover` dias de estoque frente à procura.
pub(crate) fn target(current: i32, stock: u32, demand_day: i32, short: bool) -> i32 {
    let target = if demand_day <= 0 {
        // Ninguém compra: barateia devagar.
        (current - 5).max(IDLE_FLOOR)
    } else {
        let cover = f64::from(stock) / f64::from(demand_day);
        (100.0 / cover.max(0.25).sqrt()).round() as i32
    };
    let target = if short { target + 25 } else { target };
    target.clamp(FLOOR, CEILING)
}

/// Uma vez por turno: atualiza procura e preços.
/// `ceiling`: o maior fator que cada item pode ter (paridade de importação), quando há conexão.
fn update_prices(entity: &mut ModBlockEntity, ceiling: &dyn Fn(&str) -> Option<i32>) {
    let stock = stock(entity);
    let sold = int_map(entity, "Sold");
    let mut demand = int_map(entity, "Demand");
    let mut prices = int_map(entity, "Prices");
    let short = number(entity, "Short") > 0;
    // Procura: média móvel de um dia das unidades vendidas por turno.
    let mut items: Vec<String> = stock.iter().map(|(item, _)| item.clone()).collect();
    for (item, _) in sold.iter().chain(demand.iter()) {
        if !items.contains(item) {
            items.push(item.clone());
        }
    }
    let mut next_demand = Vec::new();
    let mut next_prices = Vec::new();
    for item in items {
        let day = ledger::daily(read(&demand, &item).unwrap_or(0), read(&sold, &item).unwrap_or(0));
        let have = read(&stock.iter().map(|(item, count)| (item.clone(), *count as i32)).collect::<Vec<_>>(), &item)
            .unwrap_or(0) as u32;
        let current = read(&prices, &item).unwrap_or(100);
        let goal = target(current, have, day, short && is_food(&item));
        let next = current + (goal - current) / 4;
        // Com conexão externa, o preço local não passa do custo de importar.
        let next = ceiling(&item).map_or(next, |cap| next.min(cap));
        if have > 0 || day > 0 {
            next_demand.push((item.clone(), day));
            next_prices.push((item, next.clamp(FLOOR, CEILING)));
        }
    }
    next_demand.truncate(REMEMBERED);
    next_prices.truncate(REMEMBERED);
    demand = next_demand;
    prices = next_prices;
    set_int_map(entity, "Demand", &demand);
    set_int_map(entity, "Prices", &prices);
    set_int_map(entity, "Sold", &[]);
    set_number(entity, "Short", 0);
}

/// Teto do fator de preço de um item, em %, quando há conexão externa: o custo de importar mais a
/// margem do comerciante, já contando o imposto que sai na venda. Assim importar nunca dá prejuízo.
pub(crate) fn import_ceiling(item: &str, day: i64, tax: i32) -> Option<i32> {
    let base = base_price(item)?;
    let cost = trade::import_price(item, day)?;
    Some((cost * (100 + IMPORT_MARGIN) * 100 / ((100 - tax) * base.max(1))).clamp(FLOOR, CEILING))
}

/// Índice de preços da comida da loja (média dos fatores), em %.
fn food_index(entity: &ModBlockEntity) -> i32 {
    let prices: Vec<i32> = stock(entity).iter().filter(|(item, _)| is_food(item))
        .map(|(item, _)| factor(entity, item)).collect();
    if prices.is_empty() { 100 } else { prices.iter().sum::<i32>() / prices.len() as i32 }
}

pub(crate) fn stock(entity: &ModBlockEntity) -> Vec<(String, u32)> {
    let mut items: Vec<(String, u32)> = Vec::new();
    for stack in entity.items.get(STOCK_SLOTS).into_iter().flatten().flatten() {
        if stack.extra.is_some() || base_price(&stack.item).is_none() {
            continue;
        }
        match items.iter_mut().find(|(item, _)| *item == *stack.item) {
            Some((_, count)) => *count += stack.count,
            None => items.push((stack.item.to_string(), stack.count)),
        }
    }
    items
}

/// O que a loja deve à companhia, por divisão da holding.
fn payable(entity: &ModBlockEntity) -> [i32; holding_tabs()] {
    let mut out = [0; holding_tabs()];
    if let Some(Tag::IntArray(values)) = entity.nbt.get(nbt_key("Payable")) {
        for (slot, value) in out.iter_mut().zip(values) {
            *slot = (*value).max(0);
        }
    }
    out
}

const fn holding_tabs() -> usize {
    super::super::super::holding::TABS as usize
}

fn set_payable(entity: &mut ModBlockEntity, values: &[i32; holding_tabs()]) {
    entity.nbt.insert("Payable".into(), Tag::IntArray(values.to_vec()));
}

/// Unidades que saíram do estoque por venda (não contam como entrega nova no próximo ciclo).
fn forget_units(entity: &mut ModBlockEntity, item: &str, units: u32) {
    let mut known = int_map(entity, "Known");
    bump(&mut known, item, -(units as i32));
    known.retain(|(_, count)| *count > 0);
    set_int_map(entity, "Known", &known);
}

/// Entregas novas no estoque viram dívida de atacado com a companhia.
fn receive_deliveries(dimension: &str, pos: [i32; 3], entity: &mut ModBlockEntity, now: u64) {
    let goods = stock(entity);
    let known = int_map(entity, "Known");
    let mut owed = payable(entity);
    let mut delivered = 0;
    for (item, count) in &goods {
        let new = *count as i32 - read(&known, item).unwrap_or(0);
        if new > 0 && let Some(price) = local_price(entity, item) {
            let value = price * WHOLESALE / 100 * new;
            let division = super::super::super::holding::item_division(item) as usize;
            owed[division] = owed[division].saturating_add(value);
            delivered += value;
        }
    }
    set_int_map(entity, "Known", &goods.iter().map(|(item, count)| (item.clone(), *count as i32)).collect::<Vec<_>>());
    set_payable(entity, &owed);
    ledger::record(dimension, pos, delivered, ledger::MADE, now);
}

/// Paga o atacado com o caixa: imposto retido na fonte, o resto para a companhia. Sem conta da
/// companhia, nada sai do caixa.
fn settle(dimension: &str, pos: [i32; 3], entity: &mut ModBlockEntity, now: u64) {
    let mut owed = payable(entity);
    for division in 0..owed.len() {
        if owed[division] == 0 {
            continue;
        }
        let cash = ledger::balance(Book::Shop, dimension, pos).unwrap_or(0);
        let amount = owed[division].min(cash);
        if amount <= 0 {
            break;
        }
        // Sem conta da companhia, nada sai do caixa: dinheiro nunca some no caminho.
        if !ledger::company_open(dimension, now) {
            break;
        }
        let paid = ledger::take(Book::Shop, dimension, pos, 0, amount, ledger::WAGES, now);
        let tax = ledger::withhold(dimension, pos, paid, now);
        ledger::put(Book::Company, dimension, pos, ledger::ANYWHERE, paid - tax, ledger::RECEIVED, now);
        super::super::super::holding::book(dimension, division as u8, paid, tax);
        owed[division] -= paid;
    }
    set_payable(entity, &owed);
}

/// Recebe uma venda no varejo: imposto retido na fonte, o resto no caixa da loja.
fn receive_sale(dimension: &str, pos: [i32; 3], amount: i32, now: u64) {
    let tax = ledger::withhold(dimension, pos, amount, now);
    ledger::put(Book::Shop, dimension, pos, 0, amount - tax, ledger::SOLD, now);
    ledger::record(dimension, pos, amount, ledger::SOLD, now);
}

/// Uma família compra na loja em `shop`, a preço local, até `budget` moedas e `max_units` unidades.
/// Devolve (item, unidades, custo); a família tira o custo da carteira. `wanted_units` a mais do que
/// conseguiu vira falta na loja.
pub(crate) fn buy(host: &mut dyn Host, shop: [i32; 3], wanted: fn(&str) -> bool, budget: i32, max_units: u32,
    wanted_units: u32, now: u64) -> Vec<(String, u32, i32)>
{
    let mut bought: Vec<(String, u32, i32)> = Vec::new();
    host.change_mod_block_entity(shop, &mut |entity| {
        if entity.id != SHOP {
            return;
        }
        let mut spent = 0;
        let mut units = 0;
        let mut sold = int_map(entity, "Sold");
        // Uma unidade por vez, rodando entre os itens: a família leva variedade.
        let mut progress = true;
        while progress && units < max_units {
            progress = false;
            for slot in STOCK_SLOTS {
                let Some(stack) = entity.items[slot].as_ref() else { continue };
                if !wanted(&stack.item) || stack.extra.is_some() || units >= max_units {
                    continue;
                }
                let item = stack.item.to_string();
                let Some(cost) = local_price(entity, &item) else { continue };
                if spent + cost > budget {
                    continue;
                }
                consume(&mut entity.items[slot], 1);
                spent += cost;
                units += 1;
                progress = true;
                bump(&mut sold, &item, 1);
                match bought.iter_mut().find(|(known, _, _)| *known == item) {
                    Some(entry) => { entry.1 += 1; entry.2 += cost; }
                    None => bought.push((item, 1, cost)),
                }
            }
        }
        for (item, count, _) in &bought {
            forget_units(entity, item, *count);
        }
        set_int_map(entity, "Sold", &sold);
        if wanted_units > units {
            set_number(entity, "Short", number(entity, "Short").saturating_add((wanted_units - units) as i32));
        }
        set_number(entity, "SoldToday", number(entity, "SoldToday").saturating_add(spent));
    });
    let spent: i32 = bought.iter().map(|(_, _, cost)| cost).sum();
    if spent > 0 {
        let dimension = host.dimension().to_owned();
        receive_sale(&dimension, shop, spent, now);
    }
    bought
}

/// Lote e preço em esmeraldas de um item ao preço `unit`: o maior lote que cabe em 1 esmeralda
/// (até 16 unidades); item que custa mais que uma esmeralda vai sozinho, arredondado para cima.
pub(crate) fn bundle_at(unit: i32) -> (u32, u32) {
    let unit = unit.max(1) as u32;
    let emerald = COINS_PER_EMERALD as u32;
    if unit > emerald {
        return (1, unit.div_ceil(emerald));
    }
    ((emerald / unit).clamp(1, 16), 1)
}

#[cfg(test)]
pub(crate) fn bundle(item: &str) -> Option<(u32, u32)> {
    base_price(item).map(bundle_at)
}

/// As ofertas do comerciante: uma por item do estoque, ao preço local.
pub(crate) fn offers(entity: &ModBlockEntity, open: bool) -> Tag {
    let recipes: Vec<Tag> = if !open { Vec::new() } else { stock(entity).iter().filter_map(|(item, count)| {
        let (lot, emeralds) = bundle_at(local_price(entity, item)?);
        let uses = (*count / lot) as i32;
        (uses > 0).then(|| Tag::Compound(Compound::from([
            ("buy".into(), Tag::Compound(Compound::from([("id".into(), Tag::String(WAGE.into())), ("count".into(), Tag::Int(emeralds as i32))]))),
            ("sell".into(), Tag::Compound(Compound::from([("id".into(), Tag::String(item.as_str().into())), ("count".into(), Tag::Int(lot as i32))]))),
            ("maxUses".into(), Tag::Int(uses)),
            ("xp".into(), Tag::Int(0)),
            ("rewardExp".into(), Tag::Byte(0)),
        ])))
    }).collect() };
    Tag::Compound(Compound::from([("Recipes".into(), Tag::List(recipes))]))
}

/// Assinatura do que o jogador vê na troca: itens, quantidades, preços e se está aberta.
fn signature(entity: &ModBlockEntity, open: bool) -> i32 {
    let mut hash: u32 = u32::from(open);
    for (item, count) in stock(entity) {
        for byte in item.bytes() {
            hash = hash.wrapping_mul(31).wrapping_add(u32::from(byte));
        }
        hash = hash.wrapping_mul(31).wrapping_add(count);
        hash = hash.wrapping_mul(31).wrapping_add(factor(entity, &item) as u32);
    }
    (hash & 0x3fff_ffff) as i32
}

/// Ciclo da loja: caixa, entregas e atacado, preços (uma vez por turno), ofertas do comerciante,
/// painel e registro.
pub(crate) fn shop_cycle(host: &mut dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity, time: i64) {
    let now = defense::clock();
    let dimension: &'static str = host.dimension();
    let merchant = worker(entity).and_then(cached_entity);
    let open = merchant.is_some() && number(entity, "Status") == 1 && number(entity, "Present") == 1;
    if !matches!(number(entity, "Status"), 1 | WALKING) {
        set_number(entity, "Present", 0);
    }
    ledger::open(Book::Shop, dimension, pos, number(entity, "Cash"), now, false);
    receive_deliveries(dimension, pos, entity, now);
    settle(dimension, pos, entity, now);
    if !entity.nbt.contains_key(nbt_key("Prices")) {
        set_int_map(entity, "Prices", &[]);
    } else if ledger::turn_edge(pos, time) {
        let day = time.div_euclid(DAY);
        // Falta de comida e conexão externa: o comerciante importa a comida que mais vende (ou pão).
        // A moeda sai da economia; importado não é entrega da indústria, então não vira atacado.
        let short = number(entity, "Short");
        if short > 0 && trade::connected(dimension, now) {
            let item = int_map(entity, "Demand").into_iter()
                .filter(|(item, _)| is_food(item) && trade::product(item).is_some())
                .max_by_key(|(_, demand)| *demand)
                .map_or_else(|| trade::STAPLE.to_owned(), |(item, _)| item);
            let room_in = |stack: &Option<ItemStack>| stack.as_ref().map_or(64, |stack| {
                if stack.item.as_ref() == item && stack.extra.is_none() { 64 - stack.count.min(64) } else { 0 }
            });
            let room: u32 = STOCK_SLOTS.map(|slot| room_in(&entity.items[slot])).sum();
            let units = trade::import(dimension, pos, &item, (short as u32).min(16).min(room), day, now);
            let mut left = units;
            for slot in STOCK_SLOTS {
                let put = room_in(&entity.items[slot]).min(left);
                if put > 0 {
                    add(&mut entity.items[slot], &item, put);
                    left -= put;
                }
            }
            let mut known = int_map(entity, "Known");
            bump(&mut known, &item, units as i32);
            set_int_map(entity, "Known", &known);
        }
        let connected = trade::connected(dimension, now);
        let tax = governing(dimension, pos, now).map_or(0, |hall| mayor::tax_rate(hall.phase));
        update_prices(entity, &|item| connected.then(|| import_ceiling(item, day, tax)).flatten());
    }
    // Vendas do dia: zera no começo de cada dia.
    let today = time.div_euclid(DAY) as i32;
    if number(entity, "Day") != today {
        set_number(entity, "Day", today);
        set_number(entity, "SoldToday", 0);
    }
    let sign = signature(entity, open);
    if let Some(id) = merchant
        && (sign != number(entity, "OfferSig") || number(entity, "OfferFor") as u32 != id)
        && let Some(handle) = host.entity(id)
        && (host.entity_merge_nbt(handle, Compound::from([("Offers".into(), offers(entity, open))])) || !open)
    {
        set_number(entity, "OfferSig", sign);
        set_number(entity, "OfferFor", id as i32);
    }
    let cash = ledger::balance(Book::Shop, dimension, pos).unwrap_or(0);
    set_number(entity, "Cash", cash);
    let goods = stock(entity);
    let index = food_index(entity);
    set_number(entity, "Rate", goods.iter().map(|(_, count)| *count as i32).sum());
    set_number(entity, "Utilization", index);
    set_number(entity, "Total", number(entity, "SoldToday"));
    set_number(entity, "Info", cash);
    set_number(entity, "Duration", payable(entity).iter().sum());
    upsert(&SHOPS, Shop { dimension: dimension.to_owned(), pos, merchant, open, prices: index, cash, seen: now },
        |known| known.dimension == dimension && known.pos == pos, |known| now.saturating_sub(known.seen) <= STALE);
}

/// O jogador trocou com um aldeão. Se for um comerciante do Veio, o item sai do estoque, a venda
/// conta como procura, e as esmeraldas pagas entram na economia, no caixa da loja.
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
        forget_units(entity, &item, count - left);
        let mut sold = int_map(entity, "Sold");
        bump(&mut sold, &item, count as i32);
        set_int_map(entity, "Sold", &sold);
        set_number(entity, "SoldToday", number(entity, "SoldToday").saturating_add(paid));
        // Força a atualização das ofertas no próximo ciclo.
        set_number(entity, "OfferSig", -1);
        moved = true;
    });
    if moved {
        ledger::border(&dimension, paid);
        receive_sale(&dimension, shop, paid, now);
    }
}
