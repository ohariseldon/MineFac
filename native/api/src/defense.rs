//! Torre Tesla: abate monstros hostis por perto usando energia da fábrica.
//!
//! O Horizon conta cada mob que vai processar o tick (`entity_tick_pre`). A torre guarda os
//! hostis vistos recentemente em cada dimensão e, no tick do próprio bloco, escolhe o mais
//! próximo dentro do alcance.
use super::*;
use horizon_client::mod_api::{ParticleOptions, ParticleType};
use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::atomic::{AtomicU64, Ordering};

pub(super) const RANGE: f64 = 10.0;
pub(super) const SHOT_COST: i32 = 2400;
pub(super) const COOLDOWN: i32 = 40;
pub(super) const CAPACITY: i32 = 20000;
/// Um mob conta como presente se foi visto nas últimas 40 contagens do relógio do mod.
const FRESH: u64 = 40;
const FORGET: u64 = 400;
const MAX_TRACKED: usize = 4096;

static CLOCK: AtomicU64 = AtomicU64::new(0);
static SEEN: Mutex<Vec<(String, HashMap<u32, u64>)>> = Mutex::new(Vec::new());

/// Monstros que atacam o jogador por conta própria. Chefes ficam de fora.
pub(super) fn hostile(kind: &str) -> bool {
    matches!(kind,
        "minecraft:zombie" | "minecraft:husk" | "minecraft:drowned" | "minecraft:zombie_villager"
        | "minecraft:skeleton" | "minecraft:stray" | "minecraft:bogged" | "minecraft:wither_skeleton"
        | "minecraft:creeper" | "minecraft:spider" | "minecraft:cave_spider" | "minecraft:witch"
        | "minecraft:slime" | "minecraft:magma_cube" | "minecraft:phantom" | "minecraft:silverfish"
        | "minecraft:endermite" | "minecraft:pillager" | "minecraft:vindicator" | "minecraft:evoker"
        | "minecraft:ravager" | "minecraft:vex" | "minecraft:blaze" | "minecraft:ghast"
        | "minecraft:guardian" | "minecraft:zoglin" | "minecraft:piglin_brute" | "minecraft:breeze"
        | "minecraft:creaking" | "minecraft:hoglin" | "veio:raider")
}

/// Relógio do mod: conta os ticks do jogo (janela e servidor) para envelhecer registros.
pub(super) fn clock() -> u64 {
    CLOCK.load(Ordering::Relaxed)
}

pub(super) fn advance_clock() {
    CLOCK.fetch_add(1, Ordering::Relaxed);
}

pub(super) fn observe(dimension: &str, id: u32, kind: &str) {
    if !hostile(kind) {
        return;
    }
    let now = CLOCK.load(Ordering::Relaxed);
    let Ok(mut seen) = SEEN.lock() else { return };
    if let Some((_, mobs)) = seen.iter_mut().find(|(name, _)| name == dimension) {
        if mobs.len() >= MAX_TRACKED && !mobs.contains_key(&id) {
            mobs.clear();
        }
        mobs.insert(id, now);
    } else {
        seen.push((dimension.to_owned(), HashMap::from([(id, now)])));
    }
}

/// Hostis vistos há pouco nesta dimensão. Esquece os antigos.
pub(super) fn recent(dimension: &str) -> Vec<u32> {
    let now = CLOCK.load(Ordering::Relaxed);
    let Ok(mut seen) = SEEN.lock() else { return Vec::new() };
    let Some((_, mobs)) = seen.iter_mut().find(|(name, _)| name == dimension) else {
        return Vec::new();
    };
    mobs.retain(|_, stamp| now.saturating_sub(*stamp) <= FORGET);
    mobs.iter().filter(|(_, stamp)| now.saturating_sub(**stamp) <= FRESH).map(|(id, _)| *id).collect()
}

fn forget(dimension: &str, id: u32) {
    if let Ok(mut seen) = SEEN.lock()
        && let Some((_, mobs)) = seen.iter_mut().find(|(name, _)| name == dimension)
    {
        mobs.remove(&id);
    }
}

/// O alvo hostil mais próximo dentro do alcance, com sua posição.
pub(super) fn nearest(
    origin: [f64; 3],
    candidates: &[u32],
    position: impl Fn(u32) -> Option<[f64; 3]>,
    gone: &mut Vec<u32>,
) -> Option<(u32, [f64; 3])> {
    let mut best: Option<(u32, [f64; 3], f64)> = None;
    for &id in candidates {
        let Some(at) = position(id) else {
            gone.push(id);
            continue;
        };
        let distance = (0..3).map(|axis| (at[axis] - origin[axis]).powi(2)).sum::<f64>();
        if distance <= RANGE * RANGE && best.is_none_or(|(_, _, closest)| distance < closest) {
            best = Some((id, at, distance));
        }
    }
    best.map(|(id, at, _)| (id, at))
}

pub(super) fn tick(host: &mut dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity) {
    let cooldown = number(entity, "Cooldown");
    if cooldown > 0 {
        set_number(entity, "Cooldown", cooldown - 1);
        set_number(entity, "Progress", COOLDOWN - cooldown);
        set_number(entity, "Duration", COOLDOWN);
        return;
    }
    set_number(entity, "Progress", 0);
    let energy = number(entity, "energy");
    if energy < SHOT_COST {
        set_number(entity, "Status", 5);
        return;
    }
    let dimension = host.dimension();
    let candidates = recent(dimension);
    let coil = [f64::from(pos[0]) + 0.5, f64::from(pos[1]) + 1.4, f64::from(pos[2]) + 0.5];
    let mut gone = Vec::new();
    let target = {
        let reader: &dyn Host = host;
        nearest(coil, &candidates, |id| reader.entity(id).and_then(|entity| entity.position(reader)), &mut gone)
    };
    for id in gone {
        forget(dimension, id);
    }
    let Some((id, at)) = target else {
        set_number(entity, "Status", 11);
        return;
    };
    forget(dimension, id);
    if !host.kill_mob(dimension, id) {
        return;
    }
    set_number(entity, "energy", energy - SHOT_COST);
    set_number(entity, "Cooldown", COOLDOWN);
    set_number(entity, "Produced", number(entity, "Produced").saturating_add(1));
    set_number(entity, "Total", number(entity, "Total").saturating_add(1));
    set_number(entity, "Status", 1);
    let hit = [at[0], at[1] + 0.9, at[2]];
    let spark = ParticleOptions::simple(ParticleType::ElectricSpark);
    for step in 0..=8 {
        let t = f64::from(step) / 8.0;
        let point = std::array::from_fn(|axis| coil[axis] + (hit[axis] - coil[axis]) * t);
        host.send_particles(&spark, None, point, [0.05, 0.05, 0.05], 0.0, 3);
    }
    host.play_sound("minecraft:entity.lightning_bolt.impact", hit, 4, 0.5, 1.7);
}
