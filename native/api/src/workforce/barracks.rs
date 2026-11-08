//! Quartel: um aldeão treinado defende a vila.
//!
//! Um prédio, um soldado. Agachar com uma esmeralda na mão troca a classe; com alguém
//! contratado, a troca gasta uma esmeralda do slot de salário. Comida no slot da esquerda:
//! cada golpe custa uma tarefa. Sem comida, ninguém sai do posto.
//!
//! O soldado fica de plantão também à noite e não foge de medo (`super::duty`). Os golpes tiram
//! vida de verdade do monstro (`Host::set_mob_health`); no último, ele morre e solta o saque.
//!
//! - **Milícia:** chega rápido, 7 de dano.
//! - **Guarda:** curto, 5 de dano, mas se cura a cada golpe: é o tanque da vila.
//! - **Arqueiro:** atira de longe (16 blocos), 5 de dano.
//! - **Mago:** 10 de dano e metade disso num segundo monstro perto do alvo.
use super::*;
use horizon_client::mod_api::{ParticleOptions, ParticleType};

pub(super) const BARRACKS: &str = "veio:barracks";
const PATROL: f64 = 18.0;
const SWING: i32 = 20;
const SPLASH: f64 = 5.0;
/// Vida que o guarda recupera a cada golpe.
pub(crate) const GUARD_HEAL: f32 = 3.0;
/// Vida de um aldeão (vanilla).
const VILLAGER_LIFE: f32 = 20.0;
/// Sem vida legível no NBT, o golpe conta como uma fração destes pontos.
const FALLBACK_LIFE: f32 = 20.0;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Class {
    pub role: i32,
    pub reach: f64,
    pub damage: f32,
    pub heal: f32,
    pub splash: bool,
}

pub(crate) fn class(role: i32) -> Class {
    match role {
        2 => Class { role: 2, reach: 5.0, damage: 5.0, heal: GUARD_HEAL, splash: false },
        3 => Class { role: 3, reach: 16.0, damage: 5.0, heal: 0.0, splash: false },
        4 => Class { role: 4, reach: 12.0, damage: 10.0, heal: 0.0, splash: true },
        _ => Class { role: 1, reach: 8.0, damage: 7.0, heal: 0.0, splash: false },
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

/// Golpes que já acertaram cada monstro, para quando o NBT não diz a vida.
static WOUNDS: Mutex<Vec<(u32, f32)>> = Mutex::new(Vec::new());

/// Vida atual de uma entidade, lida do NBT (`Health`).
fn health(host: &mut dyn Host, id: u32) -> Option<f32> {
    let handle = host.entity(id)?;
    match host.entity_nbt(handle)?.get(&"Health".into())? {
        Tag::Float(bits) => Some(f32::from_bits(*bits)),
        Tag::Double(bits) => Some(f64::from_bits(*bits) as f32),
        _ => None,
    }
}

/// Vida depois de levar `damage`, com o contador como reserva quando não há vida legível.
pub(crate) fn after_blow(current: Option<f32>, id: u32, damage: f32) -> f32 {
    if let Some(life) = current {
        return life - damage;
    }
    let Ok(mut wounds) = WOUNDS.lock() else { return 0.0 };
    let taken = if let Some((_, taken)) = wounds.iter_mut().find(|(seen, _)| *seen == id) {
        *taken += damage;
        *taken
    } else {
        if wounds.len() >= 128 {
            wounds.drain(0..64);
        }
        wounds.push((id, damage));
        damage
    };
    let left = FALLBACK_LIFE - taken;
    if left <= 0.0 {
        wounds.retain(|(seen, _)| *seen != id);
    }
    left
}

fn take_wage(entity: &mut ModBlockEntity) -> bool {
    let Some(stack) = entity.items.get(1).and_then(Option::as_ref) else { return false };
    if stack.item.as_ref() != WAGE || stack.extra.is_some() || stack.damage != 0 || stack.count == 0 {
        return false;
    }
    consume(&mut entity.items[1], 1);
    true
}

/// Agachar com uma esmeralda na mão: escolhe a próxima classe. Treinar quem já trabalha gasta uma esmeralda.
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

/// Um golpe de `damage`. `true` quando mata.
fn strike(host: &mut dyn Host, dimension: &str, id: u32, at: [f64; 3], kind: Class, damage: f32) -> bool {
    let current = health(host, id);
    let left = after_blow(current, id, damage);
    let killed = if left <= 0.0 {
        host.kill_mob(dimension, id)
    } else {
        host.set_mob_health(dimension, id, left);
        false
    };
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

/// O guarda se recupera a cada golpe (a comida do posto paga).
fn heal(host: &mut dyn Host, dimension: &str, id: u32, amount: f32) {
    if amount <= 0.0 {
        return;
    }
    if let Some(life) = health(host, id)
        && life > 0.0
        && life < VILLAGER_LIFE
    {
        host.set_mob_health(dimension, id, (life + amount).min(VILLAGER_LIFE));
    }
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
    duty(id, now);
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
    let killed = strike(host, &dimension, mob, spot, kind, kind.damage);
    heal(host, &dimension, id, kind.heal);
    if kind.splash
        && let Some((other, other_at)) = nearby.iter().find(|(other, other_at)| {
            *other != mob && (other_at[0] - spot[0]).hypot(other_at[2] - spot[2]) <= SPLASH
        })
        && strike(host, &dimension, *other, *other_at, kind, kind.damage / 2.0)
    {
        set_number(entity, "Produced", number(entity, "Produced").saturating_add(1));
    }
    if killed {
        set_number(entity, "Produced", number(entity, "Produced").saturating_add(1));
    }
    finish_task(entity, 0);
    1
}
