//! Quartel: um aldeão treinado defende a vila.
//!
//! Um prédio, um soldado. Agachar com a mão vazia troca a classe; com alguém
//! contratado, a troca gasta uma esmeralda do slot de salário. Comida no slot
//! da esquerda: cada golpe custa uma tarefa. Sem comida, ninguém sai do posto.
//!
//! Milícia chega antes e mata em três golpes. O guarda é curto e tanque: os
//! monstros vão nele antes dos outros aldeões, e ele aguenta o dobro. O arqueiro
//! atira de longe. O mago mata em dois golpes e fere um segundo monstro perto.
use super::*;
use horizon_client::mod_api::{ParticleOptions, ParticleType};

pub(super) const BARRACKS: &str = "veio:barracks";
const PATROL: f64 = 18.0;
const TAUNT: f64 = 20.0;
const FRESH: u64 = 40;
const SWING: i32 = 20;
const SPLASH: f64 = 5.0;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Class {
    pub role: i32,
    pub reach: f64,
    pub blows: i32,
    pub life: i32,
    pub splash: bool,
}

pub(crate) fn class(role: i32) -> Class {
    match role {
        2 => Class { role: 2, reach: 5.0, blows: 5, life: 8, splash: false },
        3 => Class { role: 3, reach: 16.0, blows: 4, life: 4, splash: false },
        4 => Class { role: 4, reach: 12.0, blows: 2, life: 4, splash: true },
        _ => Class { role: 1, reach: 8.0, blows: 3, life: 4, splash: false },
    }
}

pub(super) fn hint(role: i32) -> &'static str {
    match role {
        2 => "veio.hint.barracks.guard",
        3 => "veio.hint.barracks.archer",
        4 => "veio.hint.barracks.mage",
        _ => "veio.hint.barracks.militia",
    }
}

fn name_key(role: i32) -> &'static str {
    match role {
        2 => "veio.barracks.guard",
        3 => "veio.barracks.archer",
        4 => "veio.barracks.mage",
        _ => "veio.barracks.militia",
    }
}

fn next_role(role: i32) -> i32 {
    match role {
        1 => 2,
        2 => 3,
        3 => 4,
        _ => 1,
    }
}

#[derive(Clone, Copy)]
struct Guard {
    id: u32,
    role: i32,
    at: [f64; 3],
    seen: u64,
}

static GUARDS: Mutex<Vec<Guard>> = Mutex::new(Vec::new());
static WOUNDS: Mutex<Vec<(u32, i32)>> = Mutex::new(Vec::new());

fn remember(id: u32, role: i32, at: [f64; 3], seen: u64) {
    let Ok(mut guards) = GUARDS.lock() else { return };
    if let Some(guard) = guards.iter_mut().find(|guard| guard.id == id) {
        *guard = Guard { id, role, at, seen };
        return;
    }
    if guards.len() >= 64 {
        let now = seen;
        guards.retain(|guard| now.saturating_sub(guard.seen) <= FRESH);
    }
    if guards.len() >= 64 {
        guards.remove(0);
    }
    guards.push(Guard { id, role, at, seen });
}

/// O defensor que o monstro deve perseguir: guarda dentro de 20 blocos, senão o soldado mais perto.
pub(crate) fn shield(at: [f64; 3]) -> Option<(u32, [f64; 3])> {
    let now = defense::clock();
    let guards = GUARDS.lock().ok()?;
    let fresh: Vec<(u32, i32, [f64; 3])> = guards.iter()
        .filter(|guard| now.saturating_sub(guard.seen) <= FRESH)
        .map(|guard| (guard.id, guard.role, guard.at))
        .collect();
    pick(at, &fresh)
}

pub(crate) fn pick(at: [f64; 3], guards: &[(u32, i32, [f64; 3])]) -> Option<(u32, [f64; 3])> {
    let distance = |pos: [f64; 3]| (pos[0] - at[0]).hypot(pos[2] - at[2]);
    let tanks = guards.iter().filter(|(_, role, pos)| *role == 2 && distance(*pos) <= TAUNT);
    if let Some(guard) = tanks.min_by(|a, b| distance(a.2).total_cmp(&distance(b.2))) {
        return Some((guard.0, guard.2));
    }
    guards.iter()
        .filter(|(_, _, pos)| distance(*pos) <= TAUNT)
        .min_by(|a, b| distance(a.2).total_cmp(&distance(b.2)))
        .map(|guard| (guard.0, guard.2))
}

/// Vida da classe, se este aldeão está registrado como defensor agora.
pub(crate) fn toughness(id: u32) -> Option<i32> {
    GUARDS.lock().ok()?.iter().find(|guard| guard.id == id).map(|guard| class(guard.role).life)
}

/// `true` quando o golpe mata.
fn wound(id: u32, need: i32) -> bool {
    let Ok(mut wounds) = WOUNDS.lock() else { return false };
    let hits = if let Some((_, hits)) = wounds.iter_mut().find(|(seen, _)| *seen == id) {
        *hits += 1;
        *hits
    } else {
        if wounds.len() >= 128 {
            wounds.drain(0..64);
        }
        wounds.push((id, 1));
        1
    };
    if hits >= need {
        wounds.retain(|(seen, _)| *seen != id);
        true
    } else {
        false
    }
}

fn take_wage(entity: &mut ModBlockEntity) -> bool {
    let Some(stack) = entity.items.get(1).and_then(Option::as_ref) else { return false };
    if stack.item.as_ref() != WAGE || stack.extra.is_some() || stack.damage != 0 || stack.count == 0 {
        return false;
    }
    consume(&mut entity.items[1], 1);
    true
}

/// Agachar com a mão vazia: escolhe a próxima classe. Treinar quem já trabalha gasta uma esmeralda.
pub(super) fn cycle(host: &mut dyn Host, pos: [i32; 3]) -> Option<Flow> {
    let mut trained = false;
    let mut paid = true;
    let mut role = 1;
    host.change_mod_block_entity(pos, &mut |entity| {
        trained = worker(entity).is_some();
        if trained && !take_wage(entity) {
            paid = false;
            return;
        }
        role = next_role(number(entity, "Role"));
        set_number(entity, "Role", role);
        set_number(entity, "Rate", role);
    });
    if trained && !paid {
        notify(host, pos, "veio.barracks.need_emerald");
        return Some(Flow::Cancel);
    }
    notify(host, pos, name_key(role));
    host.play_sound("minecraft:entity.villager.yes", post_center(pos), 4, 0.9, 1.1);
    Some(Flow::Cancel)
}

fn strike(host: &mut dyn Host, dimension: &str, id: u32, at: [f64; 3], kind: Class) -> bool {
    let killed = wound(id, kind.blows);
    if killed {
        host.kill_mob(dimension, id);
    }
    let sound = if kind.splash {
        "minecraft:entity.evoker.cast_spell"
    } else if kind.reach >= 12.0 {
        "minecraft:entity.arrow.shoot"
    } else {
        "minecraft:entity.player.attack.crit"
    };
    host.play_sound(sound, at, 6, 0.7, 1.0);
    host.send_particles(&ParticleOptions::simple(ParticleType::Crit), None,
        [at[0], at[1] + 1.0, at[2]], [0.2, 0.3, 0.2], 0.1, 6);
    killed
}

/// Persegue um hostil perto do quartel e golpeia. Sem alvo, volta para a porta.
pub(super) fn fight(host: &mut dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity, id: u32, at: [f64; 3], now: u64) -> i32 {
    let role = match number(entity, "Role") {
        0 => 1,
        other => other,
    };
    let kind = class(role);
    set_number(entity, "Role", kind.role);
    set_number(entity, "Rate", kind.role);
    remember(id, kind.role, at, defense::clock());
    let home = post_center(pos);
    let dimension = host.dimension().to_owned();
    let candidates = defense::recent(&dimension);
    let mut nearby = Vec::new();
    {
        let reader: &dyn Host = host;
        for mob in candidates {
            let Some(spot) = reader.entity(mob).and_then(|entity| entity.position(reader)) else { continue };
            if (spot[0] - home[0]).hypot(spot[2] - home[2]) <= PATROL {
                nearby.push((mob, spot));
            }
        }
    }
    let target = nearby.iter()
        .min_by(|a, b| {
            let left = (a.1[0] - at[0]).hypot(a.1[2] - at[2]);
            let right = (b.1[0] - at[0]).hypot(b.1[2] - at[2]);
            left.total_cmp(&right)
        })
        .copied();
    let Some((mob, spot)) = target else {
        order(id, home, now);
        if !near(at, home, 1.6) {
            return walk(entity, None);
        }
        set_number(entity, "Walk", 0);
        return IDLE_WORK;
    };
    order(id, spot, now);
    if !near(at, spot, kind.reach) {
        return walk(entity, None);
    }
    set_number(entity, "Walk", 0);
    let wait = number(entity, "Cooldown");
    if wait > 0 {
        set_number(entity, "Cooldown", (wait - EVERY as i32).max(0));
        return WALKING;
    }
    set_number(entity, "Cooldown", SWING);
    let killed = strike(host, &dimension, mob, spot, kind);
    if kind.splash {
        if let Some((other, other_at)) = nearby.iter().find(|(other, other_at)| {
            *other != mob && (other_at[0] - spot[0]).hypot(other_at[2] - spot[2]) <= SPLASH
        }) {
            if strike(host, &dimension, *other, *other_at, kind) {
                set_number(entity, "Produced", number(entity, "Produced").saturating_add(1));
            }
        }
    }
    if killed {
        set_number(entity, "Produced", number(entity, "Produced").saturating_add(1));
    }
    finish_task(entity);
    1
}
