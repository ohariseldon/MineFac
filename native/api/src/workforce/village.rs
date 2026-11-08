//! Crescimento das vilas: natalidade de aldeões, com um teto que cresce junto com a vila.
//!
//! - **Teto:** `BASE_CAP` (30, o triplo de uma vila grande do Minecraft) mais o desenvolvimento a
//!   até `DEVELOPMENT_RADIUS` blocos: cada posto com trabalhador soma 3, cada nível de zona soma 5,
//!   cada armazém soma 2 e um centro de controle com administrador soma 20. Máximo `MAX_CAP` (150).
//!   Assim uma vila só chega a 150 depois de virar uma fábrica grande; bases espalhadas em vários
//!   lugares têm cada uma o seu teto.
//! - **Natalidade:** a cada `CHECK` ticks (cerca de 5 min, fase espalhada pelo id), cada aldeão
//!   adulto tem uma chance em `CHANCE` de ter um filho, se houver pelo menos `MIN_ADULTS` aldeões a
//!   até `COUNT_RADIUS` blocos e menos que o teto.
//!
//! O nascimento é decidido no tick do aldeão (`observe`) e o bebê aparece no tick seguinte de um
//! bloco do Veio na mesma dimensão (`flush`), fora da iteração de entidades do jogo. As contagens
//! usam o censo de posições (`ranch::villager_positions`) e os registros de postos, zonas e
//! armazéns, sem varrer blocos.
use super::*;
use std::sync::atomic::{AtomicBool, Ordering};

pub(crate) const BASE_CAP: usize = 30;
pub(crate) const MAX_CAP: usize = 150;
pub(crate) const COUNT_RADIUS: f64 = 48.0;
pub(crate) const DEVELOPMENT_RADIUS: i32 = 64;
/// Cerca de 5 min; ímpar para que todo id caia na fase mesmo com o relógio avançando de 2 em 2.
pub(crate) const CHECK: u64 = 5999;
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

/// Teto de população da vila em volta de `at`.
pub(crate) fn cap(dimension: &str, at: [i32; 3], now: u64) -> usize {
    let posts = center::posts_near(dimension, at, DEVELOPMENT_RADIUS, now);
    let staffed = posts.iter().filter(|post| post.staffed).count();
    let control = posts.iter().any(|post| post.staffed && post.block == CONTROL_CENTER);
    let levels: i32 = zones::zones_near(dimension, at, DEVELOPMENT_RADIUS, now).iter().map(|zone| zone.level).sum();
    let stores = storage::stores_near(dimension, at, DEVELOPMENT_RADIUS, now).len();
    let points = staffed * 3 + levels.max(0) as usize * 5 + stores * 2 + if control { 20 } else { 0 }
        + projects::extra_cap(projects::at(dimension, at, now));
    (BASE_CAP + points).min(MAX_CAP)
}

/// Chamado no tick de cada aldeão. Quase sempre sai na primeira comparação.
pub(crate) fn observe(host: &mut dyn Host, dimension: &str, id: u32, now: u64) {
    // A chance mais alta (festa) é 1/2: quem não passa nela não passa em nenhuma.
    let draw = roll(id, now);
    if (now + u64::from(id)) % CHECK != 0 || draw % 2 != 0 {
        return;
    }
    let Some(handle) = host.entity(id) else { return };
    let Some(at) = handle.position(host) else { return };
    let spot = [at[0].floor() as i32, at[1].floor() as i32, at[2].floor() as i32];
    // Escola e festa (obras da vila) aumentam a chance; sem obras, 1 em `CHANCE`.
    // Racionamento (decreto): ninguém nasce.
    let Some(chance) = projects::birth_chance(projects::at(dimension, spot, now)) else { return };
    if draw % chance != 0 {
        return;
    }
    let adult = host.entity_nbt(handle).is_some_and(|nbt| production::ranch::tag_int(&nbt, "Age") >= 0);
    if !adult {
        return;
    }
    let limit = cap(dimension, spot, now);
    let near = production::ranch::villager_positions(host, dimension).iter().copied()
        .filter(|(_, other)| (0..3).all(|axis| (other[axis] - at[axis]).abs() <= COUNT_RADIUS))
        .take(limit)
        .count();
    if near < MIN_ADULTS || near >= limit {
        return;
    }
    // O prefeito: teto da fase, licença em dia e vaga em casa com alvará.
    if !society::birth_allowed(dimension, spot, near, now) {
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
