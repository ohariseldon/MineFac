//! Rancho: reproduz animais com ração, tosquia ovelhas, ordenha, recolhe ovos, penas e couro e
//! mantém cada espécie num teto de população (o excedente vira comida).
//!
//! Animais e aldeões são vistos por `entity_tick_pre` e guardados num registro em memória por
//! dimensão. O rancho age a cada 100 ticks e lê o NBT de no máximo 24 animais por vez.
use super::*;
use horizon_client::mod_api::{Compound, Tag};
use std::collections::HashMap;
use std::sync::Mutex;

pub(crate) const KINDS: [&str; 8] = [
    "minecraft:cow", "minecraft:sheep", "minecraft:pig", "minecraft:chicken", "minecraft:rabbit",
    "minecraft:goat", "minecraft:mooshroom", "minecraft:villager",
];
const VILLAGER: u8 = 7;
const RADIUS: f64 = 6.0;
/// Adultos por espécie que o rancho mantém; acima disso, abate o excedente.
pub(crate) const CAP: usize = 10;
const EVERY: i64 = 100;
const MAX_READS: usize = 24;
const FRESH: u64 = 60;
const FORGET: u64 = 600;

/// Por dimensão: id → (índice em `KINDS`, última vez visto).
type Seen = Vec<(String, HashMap<u32, (u8, u64)>)>;
static SEEN: Mutex<Seen> = Mutex::new(Vec::new());

pub(crate) fn observe(dimension: &str, id: u32, kind: &str) {
    let Some(index) = KINDS.iter().position(|known| *known == kind) else { return };
    let now = defense::clock();
    let Ok(mut seen) = SEEN.lock() else { return };
    if let Some((_, mobs)) = seen.iter_mut().find(|(name, _)| name == dimension) {
        if mobs.len() >= 4096 && !mobs.contains_key(&id) {
            mobs.clear();
        }
        mobs.insert(id, (index as u8, now));
    } else {
        seen.push((dimension.to_owned(), HashMap::from([(id, (index as u8, now))])));
    }
}

/// Ids e espécies vistos há pouco nesta dimensão; esquece os antigos.
pub(crate) fn recent(dimension: &str) -> Vec<(u32, u8)> {
    let now = defense::clock();
    let Ok(mut seen) = SEEN.lock() else { return Vec::new() };
    let Some((_, mobs)) = seen.iter_mut().find(|(name, _)| name == dimension) else { return Vec::new() };
    mobs.retain(|_, (_, stamp)| now.saturating_sub(*stamp) <= FORGET);
    mobs.iter().filter(|(_, (_, stamp))| now.saturating_sub(*stamp) <= FRESH).map(|(id, (kind, _))| (*id, *kind)).collect()
}

/// Quantos aldeões há a até `radius` blocos.
pub(crate) fn villagers_near(host: &dyn Host, center: [f64; 3], radius: f64) -> i32 {
    recent(host.dimension()).into_iter()
        .filter(|(_, kind)| *kind == VILLAGER)
        .filter(|(id, _)| host.entity(*id).and_then(|entity| entity.position(host))
            .is_some_and(|at| within(at, center, radius, radius)))
        .count() as i32
}

fn within(at: [f64; 3], center: [f64; 3], radius: f64, height: f64) -> bool {
    (at[0] - center[0]).abs() <= radius && (at[2] - center[2]).abs() <= radius && (at[1] - center[1]).abs() <= height
}

/// Ração: qual espécie cada item faz reproduzir.
pub(crate) fn feeds(item: &str) -> &'static [u8] {
    match item {
        "minecraft:wheat" => &[0, 1, 5, 6],
        "minecraft:carrot" => &[2, 4],
        "minecraft:potato" | "minecraft:beetroot" => &[2],
        "minecraft:wheat_seeds" | "minecraft:melon_seeds" | "minecraft:pumpkin_seeds" | "minecraft:beetroot_seeds" => &[3],
        "minecraft:dandelion" => &[4],
        _ => &[],
    }
}

pub(crate) fn is_feed(item: &str) -> bool {
    !feeds(item).is_empty()
}

/// Produtos de animais que o rancho recolhe do chão em volta.
fn collectable(item: &str) -> bool {
    matches!(item, "minecraft:egg" | "minecraft:brown_egg" | "minecraft:blue_egg" | "minecraft:feather"
        | "minecraft:leather" | "minecraft:beef" | "minecraft:porkchop" | "minecraft:chicken" | "minecraft:mutton"
        | "minecraft:rabbit" | "minecraft:rabbit_hide" | "minecraft:rabbit_foot")
        || item.ends_with("_wool")
}

const WOOL: [&str; 16] = ["white", "orange", "magenta", "light_blue", "yellow", "lime", "pink", "gray",
    "light_gray", "cyan", "purple", "blue", "brown", "green", "red", "black"];

fn tag_int(nbt: &Compound, key: &str) -> i32 {
    match nbt.get(&key.into()) {
        Some(Tag::Int(value)) => *value,
        Some(Tag::Byte(value)) => i32::from(*value),
        Some(Tag::Short(value)) => i32::from(*value),
        _ => 0,
    }
}

pub(crate) struct Animal {
    pub id: u32,
    pub kind: u8,
    pub adult: bool,
    pub in_love: bool,
    pub sheared: bool,
    pub color: i32,
    pub named: bool,
}

/// Decisões de um ciclo, a partir dos animais por perto e da ração disponível.
#[derive(Debug, Default, PartialEq)]
pub(crate) struct Plan {
    pub breed: Vec<u32>,
    pub shear: Vec<(u32, i32)>,
    pub milk: bool,
    pub cull: Option<u32>,
}

pub(crate) fn plan(animals: &[Animal], feed: Option<(&str, u32)>, has_bucket: bool) -> Plan {
    let mut plan = Plan::default();
    let adults = |kind: u8| animals.iter().filter(move |animal| animal.kind == kind && animal.adult);
    if let Some((item, count)) = feed
        && count >= 2
    {
        for &kind in feeds(item) {
            let population = animals.iter().filter(|animal| animal.kind == kind).count();
            let ready: Vec<u32> = adults(kind).filter(|animal| !animal.in_love).map(|animal| animal.id).take(2).collect();
            if population < CAP && ready.len() == 2 {
                plan.breed = ready;
                break;
            }
        }
    }
    plan.shear = adults(1).filter(|sheep| !sheep.sheared).take(2).map(|sheep| (sheep.id, sheep.color)).collect();
    plan.milk = has_bucket && [0u8, 5, 6].iter().any(|&kind| adults(kind).next().is_some());
    for kind in 0..VILLAGER {
        let grown: Vec<&Animal> = adults(kind).collect();
        if grown.len() > CAP && let Some(extra) = grown.iter().find(|animal| !animal.named) {
            plan.cull = Some(extra.id);
            break;
        }
    }
    plan
}

pub(crate) fn tick(host: &mut dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity, time: i64) {
    if (time + i64::from((pos[0] ^ pos[2]) & 31)).rem_euclid(EVERY) != 0 {
        return;
    }
    let center = [f64::from(pos[0]) + 0.5, f64::from(pos[1]), f64::from(pos[2]) + 0.5];
    let dimension = host.dimension();
    let mut animals = Vec::new();
    for (id, kind) in recent(dimension) {
        if kind == VILLAGER || animals.len() >= MAX_READS {
            continue;
        }
        let Some(handle) = host.entity(id) else { continue };
        let Some(at) = handle.position(host) else { continue };
        if !within(at, center, RADIUS, 3.0) {
            continue;
        }
        let Some(nbt) = host.entity_nbt(handle) else { continue };
        animals.push(Animal {
            id, kind,
            adult: tag_int(&nbt, "Age") >= 0,
            in_love: tag_int(&nbt, "InLove") > 0,
            sheared: tag_int(&nbt, "Sheared") != 0,
            color: tag_int(&nbt, "Color"),
            named: nbt.contains_key(&"CustomName".into()),
        });
    }
    set_number(entity, "Info", animals.len() as i32);
    gather(host, pos, entity);
    if animals.is_empty() {
        set_number(entity, "Status", NO_ANIMALS);
        return;
    }
    let feed = entity.items[0].as_ref().map(|stack| (stack.item.to_string(), stack.count));
    let has_bucket = entity.items[1].as_ref().is_some_and(|stack| is_plain(stack, "minecraft:bucket"));
    let plan = plan(&animals, feed.as_ref().map(|(item, count)| (item.as_str(), *count)), has_bucket);
    let mut energy = number(entity, "energy");
    let mut acted = false;
    if plan.breed.len() == 2 && spend(40, &mut energy) {
        for id in &plan.breed {
            if let Some(handle) = host.entity(*id) {
                host.entity_merge_nbt(handle, Compound::from([("InLove".into(), Tag::Int(600))]));
            }
        }
        consume(&mut entity.items[0], 2);
        acted = true;
    }
    for (id, color) in plan.shear {
        let wool = format!("minecraft:{}_wool", WOOL[color.clamp(0, 15) as usize]);
        let count = 1 + id % 3;
        if !farm::drops_fit(&entity.items[2..5], &[(wool.as_str(), count)]) || !spend(20, &mut energy) {
            break;
        }
        if let Some(handle) = host.entity(id)
            && host.entity_merge_nbt(handle, Compound::from([("Sheared".into(), Tag::Byte(1))]))
        {
            farm::store(&mut entity.items[2..5], &wool, count);
            acted = true;
        }
    }
    if plan.milk && farm::drops_fit(&entity.items[2..5], &[("minecraft:milk_bucket", 1)]) && spend(10, &mut energy) {
        consume(&mut entity.items[1], 1);
        farm::store(&mut entity.items[2..5], "minecraft:milk_bucket", 1);
        acted = true;
    }
    if let Some(id) = plan.cull
        && spend(60, &mut energy)
        && host.kill_mob(dimension, id)
    {
        acted = true;
    }
    set_number(entity, "energy", energy);
    if acted {
        set_number(entity, "Produced", number(entity, "Produced").saturating_add(1));
    }
    crops::remember_activity(entity, time, acted);
}

fn spend(cost: i32, energy: &mut i32) -> bool {
    if *energy < cost {
        return false;
    }
    *energy -= cost;
    true
}

/// Recolhe do chão os produtos dos animais da área.
fn gather(host: &mut dyn Host, [x, y, z]: [i32; 3], entity: &mut ModBlockEntity) {
    let dimension = host.dimension();
    let min = [f64::from(x) - RADIUS, f64::from(y) - 2.0, f64::from(z) - RADIUS];
    let max = [f64::from(x) + RADIUS + 1.0, f64::from(y) + 3.0, f64::from(z) + RADIUS + 1.0];
    for drop in host.item_entities(dimension, min, max) {
        if !collectable(&drop.stack.item) || drop.stack.extra.is_some() {
            continue;
        }
        let mut room = 0;
        for slot in &entity.items[2..5] {
            room += match slot {
                None => 64,
                Some(stack) if is_plain(stack, &drop.stack.item) => 64 - stack.count.min(64),
                _ => 0,
            };
        }
        let taken = drop.stack.count.min(room);
        if taken == 0 {
            continue;
        }
        let mut rest = drop.stack.clone();
        rest.count -= taken;
        if host.set_item_entity(dimension, drop.id, rest) {
            farm::store(&mut entity.items[2..5], &drop.stack.item, taken);
        }
    }
}
