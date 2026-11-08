//! Comida do jogador: o que ele come muda o dia de trabalho dele e o ritmo das máquinas.
//!
//! - **Lanche** (cru, fruta, biscoito): só a fome do jogo.
//! - **Refeição** (carne ou peixe assados, pão, batata assada, torta): pressa I e máquinas com ciclo
//!   `MEAL_TIME` (5% mais produção) por 5 minutos.
//! - **Banquete** (ensopados e sopas, cenoura dourada): pressa I, um pouco de regeneração e máquinas
//!   com ciclo `FEAST_TIME` (10% mais produção) por 8 minutos.
//!
//! Comer dentro de casa dobra a duração. Os bônus de vários jogadores não se somam: vale o maior.
//! A comida da cozinha elétrica entra aqui sem regra nova (pão, torta, sopas).
use super::*;
use std::sync::atomic::{AtomicU64, Ordering};

pub(crate) const MEAL: &[&str] = &[
    "minecraft:cooked_beef", "minecraft:cooked_porkchop", "minecraft:cooked_chicken", "minecraft:cooked_mutton",
    "minecraft:cooked_rabbit", "minecraft:cooked_cod", "minecraft:cooked_salmon", "minecraft:bread",
    "minecraft:baked_potato", "minecraft:pumpkin_pie",
];
pub(crate) const FEAST: &[&str] = &[
    "minecraft:rabbit_stew", "minecraft:mushroom_stew", "minecraft:beetroot_soup", "minecraft:golden_carrot",
];
pub(crate) const MEAL_TICKS: u64 = 6_000;
pub(crate) const FEAST_TICKS: u64 = 9_600;
pub(crate) const MEAL_TIME: f32 = 0.95;
pub(crate) const FEAST_TIME: f32 = 0.91;
const LONGEST: u64 = FEAST_TICKS * 2;

static MEAL_UNTIL: AtomicU64 = AtomicU64::new(0);
static FEAST_UNTIL: AtomicU64 = AtomicU64::new(0);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Tier {
    Snack,
    Meal,
    Feast,
}

pub(crate) fn tier(item: &str) -> Tier {
    if FEAST.contains(&item) {
        Tier::Feast
    } else if MEAL.contains(&item) {
        Tier::Meal
    } else {
        Tier::Snack
    }
}

pub(crate) fn time_factor(now: u64) -> f32 {
    if super::active(FEAST_UNTIL.load(Ordering::Relaxed), now, LONGEST) {
        FEAST_TIME
    } else if super::active(MEAL_UNTIL.load(Ordering::Relaxed), now, LONGEST) {
        MEAL_TIME
    } else {
        1.0
    }
}

/// O jogador terminou de comer `item` em `at`. Devolve quanto o bônus dura, em ticks.
pub(crate) fn eaten(player: u128, name: &str, dimension: &str, at: [f64; 3], item: &str, now: u64) -> Option<u64> {
    let (until, base, key) = match tier(item) {
        Tier::Snack => return None,
        Tier::Meal => (&MEAL_UNTIL, MEAL_TICKS, "veio.food.meal"),
        Tier::Feast => (&FEAST_UNTIL, FEAST_TICKS, "veio.food.feast"),
    };
    let home = super::in_house(dimension, at);
    let ticks = if home { base * 2 } else { base };
    // Vale a refeição que termina por último (mas nunca uma que viria de outra sessão).
    let current = until.load(Ordering::Relaxed);
    if !super::active(current, now, LONGEST) || current < now + ticks {
        until.store(now + ticks, Ordering::Relaxed);
    }
    let minutes = (ticks / 1200).to_string();
    super::tell(player, key, vec![minutes]);
    if home {
        super::tell(player, "veio.food.home", Vec::new());
    }
    if spawn::safe_name(name) {
        let seconds = ticks / 20;
        super::command(format!("effect give {name} minecraft:haste {seconds} 0 true"));
        if tier(item) == Tier::Feast {
            super::command(format!("effect give {name} minecraft:regeneration 8 0 true"));
        }
    }
    Some(ticks)
}

pub(crate) fn reset() {
    MEAL_UNTIL.store(0, Ordering::Relaxed);
    FEAST_UNTIL.store(0, Ordering::Relaxed);
}
