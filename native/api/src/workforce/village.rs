//! Crescimento das vilas: natalidade de aldeões até o triplo de uma vila grande do Minecraft.
//!
//! A cada `CHECK` ticks (cerca de 5 min, com fase espalhada pelo id), cada aldeão adulto tem uma
//! chance em `CHANCE` de ter um filho, se houver pelo menos `MIN_ADULTS` aldeões a até `RADIUS`
//! blocos e a vizinhança tiver menos de `CAP`. O vanilla limita a vila pelas camas; aqui o teto é
//! `CAP` (o triplo de uma vila vanilla grande, com cerca de 10).
//!
//! O nascimento é decidido no tick do aldeão (`observe`) e o bebê aparece no tick seguinte de um
//! bloco do Veio na mesma dimensão (`flush`), fora da iteração de entidades do jogo.
use super::*;
use std::sync::atomic::{AtomicBool, Ordering};

pub(crate) const CAP: usize = 30;
pub(crate) const RADIUS: f64 = 32.0;
pub(crate) const CHECK: u64 = 6000;
pub(crate) const CHANCE: u64 = 6;
pub(crate) const MIN_ADULTS: usize = 2;
const MAX_PENDING: usize = 16;

static BIRTHS: Mutex<Vec<(String, [f64; 3])>> = Mutex::new(Vec::new());
static PENDING: AtomicBool = AtomicBool::new(false);

/// Sorteio determinístico a partir do id e do relógio, sem gerador global.
fn roll(id: u32, now: u64) -> u64 {
    let mut n = u64::from(id).wrapping_mul(0x9E37_79B9_7F4A_7C15) ^ now.wrapping_mul(0xBF58_476D_1CE4_E5B9);
    n ^= n >> 31;
    n
}

/// Chamado no tick de cada aldeão. Quase sempre sai na primeira comparação.
pub(crate) fn observe(host: &mut dyn Host, dimension: &str, id: u32, now: u64) {
    if (now + u64::from(id)) % CHECK != 0 || roll(id, now) % CHANCE != 0 {
        return;
    }
    let Some(handle) = host.entity(id) else { return };
    let Some(at) = handle.position(host) else { return };
    let adult = host.entity_nbt(handle).is_some_and(|nbt| production::ranch::tag_int(&nbt, "Age") >= 0);
    if !adult {
        return;
    }
    let near = production::ranch::recent_villagers(dimension).into_iter()
        .filter_map(|other| host.entity(other).and_then(|other| other.position(host)))
        .filter(|other| (0..3).all(|axis| (other[axis] - at[axis]).abs() <= RADIUS))
        .take(CAP)
        .count();
    if near < MIN_ADULTS || near >= CAP {
        return;
    }
    if let Ok(mut births) = BIRTHS.lock()
        && births.len() < MAX_PENDING
    {
        births.push((dimension.to_owned(), at));
        PENDING.store(true, Ordering::Release);
    }
}

/// Faz nascer os bebês pendentes desta dimensão. Chamado no tick dos blocos do Veio.
pub(crate) fn flush(host: &mut dyn Host) {
    if !PENDING.load(Ordering::Acquire) {
        return;
    }
    let dimension = host.dimension();
    let ready: Vec<[f64; 3]> = {
        let Ok(mut births) = BIRTHS.lock() else { return };
        let ready = births.iter().filter(|(dim, _)| dim == dimension).map(|(_, at)| *at).collect();
        births.retain(|(dim, _)| dim != dimension);
        if births.is_empty() {
            PENDING.store(false, Ordering::Release);
        }
        ready
    };
    for at in ready {
        if host.summon(dimension, VILLAGER, at, Compound::from([("Age".into(), Tag::Int(-24000))])).is_some() {
            host.send_particles(&ParticleOptions::simple(ParticleType::HappyVillager), None,
                [at[0], at[1] + 1.0, at[2]], [0.4, 0.4, 0.4], 0.0, 10);
        }
    }
}

#[cfg(test)]
pub(crate) fn pending(dimension: &str) -> usize {
    BIRTHS.lock().map(|births| births.iter().filter(|(dim, _)| dim == dimension).count()).unwrap_or(0)
}
