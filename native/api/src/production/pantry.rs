//! Armazém do povoado: compra comida da fábrica e paga em esmeraldas.
//! Comida processada vale mais por ingrediente, e uma vila por perto paga 50% a mais.
use super::*;

/// Valor de comida por esmeralda.
pub(crate) const PRICE: i32 = 60;
/// Aldeões a até 48 blocos para contar como povoado.
pub(crate) const VILLAGE_SIZE: i32 = 3;
const VILLAGE_RADIUS: f64 = 48.0;
const EVERY: i64 = 20;
const PER_CYCLE: u32 = 4;

pub(crate) fn value(item: &str) -> Option<u32> {
    data::FOOD.iter().find(|(food, _)| *food == item).map(|(_, value)| *value)
}

/// Crédito ganho por `count` unidades, em centésimos para não perder frações do bônus.
pub(crate) fn credit(item: &str, count: u32, village: bool) -> i32 {
    let base = value(item).unwrap_or(0) * count * 100;
    (if village { base * 3 / 2 } else { base }) as i32
}

pub(crate) fn tick(host: &mut dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity, time: i64) {
    let phase = (time + i64::from((pos[0] ^ pos[2]) & 15)).rem_euclid(200);
    if phase == 0 || !entity.nbt.contains_key(&"Info".into()) {
        let center = [f64::from(pos[0]) + 0.5, f64::from(pos[1]), f64::from(pos[2]) + 0.5];
        set_number(entity, "Info", ranch::villagers_near(host, center, VILLAGE_RADIUS));
    }
    set_number(entity, "Duration", PRICE);
    if phase % EVERY != 0 {
        return;
    }
    let village = number(entity, "Info") >= VILLAGE_SIZE;
    let mut stored = number(entity, "Credit");
    let mut sold = false;
    if stored >= PRICE * 100 && !can_add(&entity.items[1], "minecraft:emerald", 1) {
        set_number(entity, "Status", 3);
        return;
    }
    if let Some(stack) = entity.items[0].as_ref()
        && value(&stack.item).is_some()
        && stack.extra.is_none()
    {
        let count = stack.count.min(PER_CYCLE);
        stored += credit(&stack.item.clone(), count, village);
        consume(&mut entity.items[0], count);
        sold = true;
    }
    let mut blocked = false;
    while stored >= PRICE * 100 {
        if !can_add(&entity.items[1], "minecraft:emerald", 1) {
            blocked = true;
            break;
        }
        add(&mut entity.items[1], "minecraft:emerald", 1);
        stored -= PRICE * 100;
        set_number(entity, "Total", number(entity, "Total").saturating_add(1));
        set_number(entity, "Produced", number(entity, "Produced").saturating_add(1));
    }
    set_number(entity, "Credit", stored);
    set_number(entity, "Progress", stored / 100);
    set_number(entity, "Status", if blocked { 3 } else if sold { 1 } else { 2 });
}
