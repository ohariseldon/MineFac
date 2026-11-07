//! Armazenagem da fazenda: celeiro (27 espaços, qualquer item) e silo (18 espaços, só grãos,
//! sementes, ração e comida).
//!
//! Funilaria e trabalhadores: funis podem pôr e tirar itens de todos os slots, como num baú. O
//! bocal não esvazia o armazém numa esteira. A fila do capataz usa os armazéns como último
//! destino das sobras e como origem para abastecer máquinas, a mesa e a ração do pastor.
//!
//! Cada armazém publica o que guarda em `STORES` (memória), lido pelo centro de controle.
use super::*;
use std::sync::atomic::{AtomicBool, Ordering};

pub(crate) const BARN: &str = "veio:barn";
pub(crate) const SILO: &str = "veio:silo";
const STALE: u64 = 200;

pub(crate) fn is_store(block: &str) -> bool {
    matches!(block, BARN | SILO)
}

pub(crate) fn slots(block: &str) -> usize {
    match block {
        BARN => 27,
        SILO => 18,
        _ => 0,
    }
}

pub(crate) fn layout(block: &str) -> Option<(usize, Vec<usize>, Vec<usize>)> {
    let size = slots(block);
    (size > 0).then(|| (size, (0..size).collect(), (0..size).collect()))
}

/// Grãos, sementes, ração e comida: o que o silo aceita.
pub(crate) fn grain(item: &str) -> bool {
    is_food(item) || production::ranch::is_feed(item) || production::crops::planted(item).is_some()
        || item.ends_with("_seeds") || matches!(item, "veio:flour" | "veio:biomass" | "minecraft:hay_block"
            | "minecraft:sugar_cane" | "minecraft:cocoa_beans" | "minecraft:nether_wart")
}

pub(crate) fn accepts(block: &str, slot: usize, stack: &ItemStack) -> bool {
    slot < slots(block) && plain(stack) && (block == BARN || grain(&stack.item))
}

/// Armazém registrado: espaços usados e o total de cada item.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Store {
    pub dimension: String,
    pub pos: [i32; 3],
    pub used: u32,
    pub total: u32,
    pub items: Vec<(String, u32)>,
    pub seen: u64,
}

static STORES: Mutex<Vec<Store>> = Mutex::new(Vec::new());
static STORES_PENDING: AtomicBool = AtomicBool::new(false);

pub(crate) fn stores_near(dimension: &str, at: [i32; 3], radius: i32, now: u64) -> Vec<Store> {
    if !STORES_PENDING.load(Ordering::Acquire) {
        return Vec::new();
    }
    STORES.lock().map(|mut stores| {
        stores.retain(|store| now.saturating_sub(store.seen) <= STALE);
        if stores.is_empty() {
            STORES_PENDING.store(false, Ordering::Release);
            return Vec::new();
        }
        stores.iter()
            .filter(|store| store.dimension == dimension
                && (0..3).all(|axis| (store.pos[axis] - at[axis]).abs() <= radius))
            .cloned().collect()
    }).unwrap_or_default()
}

pub(crate) fn contents(entity: &ModBlockEntity) -> Vec<(String, u32)> {
    let mut items: Vec<(String, u32)> = Vec::new();
    for stack in entity.items.iter().flatten() {
        match items.iter_mut().find(|(item, _)| *item == *stack.item) {
            Some((_, count)) => *count += stack.count,
            None => items.push((stack.item.to_string(), stack.count)),
        }
    }
    items
}

pub(crate) fn tick(host: &mut dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity, time: i64) {
    if (time + i64::from((pos[0] ^ pos[2]) & 15)).rem_euclid(20) != 0 {
        return;
    }
    let used = entity.items.iter().filter(|slot| slot.is_some()).count() as u32;
    let total = slots(&entity.id) as u32;
    set_number(entity, "Rate", used as i32);
    set_number(entity, "Utilization", total as i32);
    set_number(entity, "Status", if used == total { 3 } else { STORING });
    let store = Store {
        dimension: host.dimension().to_owned(), pos, used, total, items: contents(entity), seen: defense::clock(),
    };
    if let Ok(mut stores) = STORES.lock() {
        let now = store.seen;
        if let Some(known) = stores.iter_mut().find(|known| known.dimension == store.dimension && known.pos == pos) {
            *known = store;
        } else {
            stores.retain(|known| now.saturating_sub(known.seen) <= STALE);
            stores.push(store);
        }
        STORES_PENDING.store(true, Ordering::Release);
    }
}

pub(crate) fn panel_lines(block: &str, data: &[i32]) -> Option<Vec<String>> {
    if !is_store(block) {
        return None;
    }
    let read = |index: usize| data.get(index).copied().unwrap_or(0);
    let tr = |key: &str, args: &[&dyn std::fmt::Display]| Text::tr(key, args).plain();
    Some(vec![tr("veio.metric.storage", &[&read(6), &read(7)]), tr(&format!("veio.hint.{}", &block[5..]), &[])])
}

/// Grade de 9 colunas; as linhas começam abaixo do estado.
pub(crate) fn screen_slots(block: &str) -> Option<Vec<Option<[f32; 2]>>> {
    let size = slots(block);
    (size > 0).then(|| (0..size).map(|index| Some([17.0 + (index % 9) as f32 * 18.0, 36.0 + (index / 9) as f32 * 18.0])).collect())
}
