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
//! O jogador compra trocando com o comerciante: as ofertas saem do estoque com o preço local e
//! são reescritas só quando o estoque ou os preços mudam.
use super::*;
use horizon_client::mod_api::MerchantOffer;

pub(crate) const STOCK_SLOTS: std::ops::Range<usize> = 1..7;
pub(crate) const TILL: usize = 7;
const FLOOR: i32 = 50;
const CEILING: i32 = 200;
const IDLE_FLOOR: i32 = 70;
/// Itens lembrados no NBT de preços (os do estoque e os procurados há pouco).
const REMEMBERED: usize = 12;

/// Preço base de uma unidade: comida vale os pontos, bens a tabela.
pub(crate) fn base_price(item: &str) -> Option<i32> {
    production::pantry::value(item).map(|value| value as i32).or_else(|| goods(item).map(|(price, _)| price))
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
fn update_prices(entity: &mut ModBlockEntity) {
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

/// Moedas no caixa; a cada `COINS_PER_EMERALD`, uma esmeralda no espaço do caixa.
pub(crate) fn credit_till(entity: &mut ModBlockEntity, coins: i32) {
    let mut till = number(entity, "Till").saturating_add(coins);
    while till >= COINS_PER_EMERALD && can_add(&entity.items[TILL], WAGE, 1) {
        add(&mut entity.items[TILL], WAGE, 1);
        till -= COINS_PER_EMERALD;
    }
    set_number(entity, "Till", till);
}

/// Uma família compra na loja em `shop`, a preço local, até `budget` moedas e `max_units` unidades.
/// Devolve (item, unidades, custo). `wanted_units` a mais do que conseguiu vira falta na loja.
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
        set_int_map(entity, "Sold", &sold);
        if wanted_units > units {
            set_number(entity, "Short", number(entity, "Short").saturating_add((wanted_units - units) as i32));
        }
        if spent > 0 {
            credit_till(entity, spent);
            set_number(entity, "SoldToday", number(entity, "SoldToday").saturating_add(spent));
        }
    });
    let spent: i32 = bought.iter().map(|(_, _, cost)| cost).sum();
    let dimension = host.dimension().to_owned();
    ledger::record(&dimension, shop, spent, true, now);
    bought
}

/// Lote e preço em esmeraldas de um item ao preço `unit`: o menor lote que vale 1 esmeralda (até 16).
pub(crate) fn bundle_at(unit: i32) -> (u32, u32) {
    let unit = unit.max(1) as u32;
    let count = (COINS_PER_EMERALD as u32).div_ceil(unit).clamp(1, 16);
    let emeralds = (count * unit).div_ceil(COINS_PER_EMERALD as u32).max(1);
    (count, emeralds)
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

/// Ciclo da loja: preços (uma vez por turno), ofertas do comerciante, painel e registro.
pub(crate) fn shop_cycle(host: &mut dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity, time: i64) {
    let now = defense::clock();
    let dimension = host.dimension().to_owned();
    let merchant = worker(entity).and_then(cached_entity);
    let open = merchant.is_some() && number(entity, "Status") == 1 && number(entity, "Present") == 1;
    if !matches!(number(entity, "Status"), 1 | WALKING) {
        set_number(entity, "Present", 0);
    }
    if !entity.nbt.contains_key(nbt_key("Prices")) {
        set_int_map(entity, "Prices", &[]);
    } else if ledger::turn_edge(pos, time) {
        update_prices(entity);
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
    {
        if host.entity_merge_nbt(handle, Compound::from([("Offers".into(), offers(entity, open))])) || !open {
            set_number(entity, "OfferSig", sign);
            set_number(entity, "OfferFor", id as i32);
        }
    }
    let goods = stock(entity);
    let index = food_index(entity);
    set_number(entity, "Rate", goods.iter().map(|(_, count)| *count as i32).sum());
    set_number(entity, "Utilization", index);
    set_number(entity, "Total", number(entity, "SoldToday"));
    set_number(entity, "Info", number(entity, "Till"));
    upsert(&SHOPS, Shop { dimension: dimension.clone(), pos, merchant, open, prices: index, seen: now },
        |known| known.dimension == dimension && known.pos == pos, |known| now.saturating_sub(known.seen) <= STALE);
}

/// O jogador trocou com um aldeão. Se for um comerciante do Veio, o item sai do estoque, a venda
/// conta como procura e as esmeraldas entram no caixa.
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
        let mut sold = int_map(entity, "Sold");
        bump(&mut sold, &item, count as i32);
        set_int_map(entity, "Sold", &sold);
        credit_till(entity, paid);
        set_number(entity, "SoldToday", number(entity, "SoldToday").saturating_add(paid));
        // Força a atualização das ofertas no próximo ciclo.
        set_number(entity, "OfferSig", -1);
        moved = true;
    });
    if moved {
        ledger::record(&dimension, shop, paid, true, now);
    }
}
