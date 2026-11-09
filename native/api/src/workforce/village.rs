//! Crescimento das vilas: natalidade de aldeões, com um teto que cresce junto com a vila.
//!
//! - **Teto:** `BASE_CAP` (20, a população com que a cidade nasce) mais o desenvolvimento a até
//!   `DEVELOPMENT_RADIUS` blocos: cada posto com trabalhador soma 3, cada nível de zona soma 5,
//!   cada armazém soma 2 e um centro de controle com administrador soma 20. Máximo `MAX_CAP` (500).
//!   Assim uma cidade só chega a 500 depois de virar uma fábrica grande; bases espalhadas em vários
//!   lugares têm cada uma o seu teto. O piso (nunca menos de 2) é da cidade (`town`).
//! - **Natalidade:** a cada `CHECK` ticks (cerca de 5 min, fase espalhada pelo id), cada aldeão
//!   adulto tem uma chance em `CHANCE` de ter um filho, se houver pelo menos `MIN_ADULTS` aldeões a
//!   até `COUNT_RADIUS` blocos, menos que o teto e uma cama livre. A cama é a casa, como na vila
//!   vanilla: a cabeceira conta, o pé não. Sem a primeira volta das camas, o teto de
//!   desenvolvimento vale sozinho.
//!
//! O nascimento é decidido no tick do aldeão (`observe`) e o bebê aparece no tick seguinte de um
//! bloco do Veio na mesma dimensão (`flush`), fora da iteração de entidades do jogo. As contagens
//! usam o censo de posições (`ranch::villager_positions`) e os registros de postos, zonas e
//! armazéns. As camas são lidas aos poucos, 16 blocos por tick.
use super::*;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

pub(crate) const BASE_CAP: usize = 20;
pub(crate) const MAX_CAP: usize = 500;
pub(crate) const COUNT_RADIUS: f64 = 48.0;
pub(crate) const DEVELOPMENT_RADIUS: i32 = 64;
/// Cerca de 5 min; ímpar para que todo id caia na fase mesmo com o relógio avançando de 2 em 2.
pub(crate) const CHECK: u64 = 5999;
pub(crate) const CHANCE: u64 = 6;
pub(crate) const MIN_ADULTS: usize = 2;
const MAX_PENDING: usize = 16;
/// Camas contadas neste raio, em três camadas. Uma volta leva cerca de 40 s.
const HOUSING_RADIUS: i32 = 32;
const HOUSING_STEP: i32 = 16;
const ASK_EVERY: u64 = 200;

static BIRTHS: Mutex<Vec<(String, [f64; 3])>> = Mutex::new(Vec::new());
static PENDING: AtomicBool = AtomicBool::new(false);
static LAST_SCAN: AtomicU64 = AtomicU64::new(u64::MAX);
static LAST_ASK: AtomicU64 = AtomicU64::new(0);

struct Homes {
    dimension: String,
    origin: [i32; 3],
    cursor: i32,
    found: usize,
    ready: Option<usize>,
}

static HOMES: Mutex<Vec<Homes>> = Mutex::new(Vec::new());

fn bed_head(host: &dyn Host, pos: [i32; 3]) -> bool {
    host.block(pos).ends_with("_bed") && host.block_property(pos, "part").as_deref() != Some("foot")
}

fn housing_spot(origin: [i32; 3], index: i32) -> [i32; 3] {
    let side = HOUSING_RADIUS * 2 + 1;
    let layer = side * side;
    let index = index.rem_euclid(layer * 3);
    let cell = index % layer;
    let y = [0, -1, 1][(index / layer) as usize];
    [origin[0] + cell % side - HOUSING_RADIUS, origin[1] + y, origin[2] + cell / side - HOUSING_RADIUS]
}

/// Uma volta das camas em volta do aldeão `id`. Um passo por tick do jogo: só o primeiro aldeão
/// do tick lê a própria posição; os outros saem na primeira comparação.
pub(crate) fn scan_homes(host: &dyn Host, dimension: &str, id: u32, now: u64) {
    if LAST_SCAN.swap(now, Ordering::Relaxed) == now {
        return;
    }
    let Some(at) = host.entity(id).and_then(|entity| entity.position(host)) else { return };
    let here = [at[0].floor() as i32, at[1].floor() as i32, at[2].floor() as i32];
    let Ok(mut homes) = HOMES.lock() else { return };
    let lap = if let Some(lap) = homes.iter_mut().find(|lap| lap.dimension == dimension) {
        if lap.cursor == 0 && (0..3).any(|axis| (lap.origin[axis] - here[axis]).abs() > HOUSING_RADIUS) {
            *lap = Homes { dimension: dimension.to_owned(), origin: here, cursor: 0, found: 0, ready: None };
        }
        lap
    } else {
        homes.push(Homes { dimension: dimension.to_owned(), origin: here, cursor: 0, found: 0, ready: None });
        homes.last_mut().unwrap()
    };
    let area = (HOUSING_RADIUS * 2 + 1).pow(2) * 3;
    for _ in 0..HOUSING_STEP {
        if bed_head(host, housing_spot(lap.origin, lap.cursor)) {
            lap.found += 1;
        }
        lap.cursor += 1;
        if lap.cursor >= area {
            lap.ready = Some(lap.found);
            lap.found = 0;
            lap.cursor = 0;
            break;
        }
    }
}

/// Camas já contadas perto de `at`. `None` até a primeira volta terminar.
pub(crate) fn homes(dimension: &str, at: [i32; 3]) -> Option<usize> {
    HOMES.lock().ok().and_then(|homes| homes.iter().find(|lap| {
        lap.dimension == dimension && (0..3).all(|axis| (lap.origin[axis] - at[axis]).abs() <= HOUSING_RADIUS)
    }).and_then(|lap| lap.ready))
}

/// Teto efetivo: o desenvolvimento, e as camas quando a volta já terminou.
pub(crate) fn room(development: usize, beds: Option<usize>) -> usize {
    beds.map_or(development, |beds| development.min(beds))
}

/// A população encheu as casas e o desenvolvimento ainda deixaria crescer.
pub(crate) fn wants_houses(population: usize, development: usize, beds: Option<usize>) -> bool {
    beds.is_some_and(|beds| population >= beds && beds < development)
}

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
    // Cidade que perdeu mais da metade do recorde: nascimentos com o dobro da chance.
    let chance = if town::recovering() { (chance / 2).max(1) } else { chance };
    if draw % chance != 0 {
        return;
    }
    let adult = host.entity_nbt(handle).is_some_and(|nbt| production::ranch::tag_int(&nbt, "Age") >= 0);
    if !adult {
        return;
    }
    let development = cap(dimension, spot, now);
    let beds = homes(dimension, spot);
    let limit = room(development, beds);
    let near = production::ranch::villager_positions(host, dimension).iter().copied()
        .filter(|(_, other)| (0..3).all(|axis| (other[axis] - at[axis]).abs() <= COUNT_RADIUS))
        .take(development.max(1))
        .count();
    if wants_houses(near, development, beds) {
        ask_for_houses(host, dimension, at, near, beds.unwrap_or(0), now);
    }
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

fn ask_for_houses(host: &mut dyn Host, dimension: &str, at: [f64; 3], people: usize, beds: usize, now: u64) {
    if now.saturating_sub(LAST_ASK.load(Ordering::Relaxed)) < ASK_EVERY {
        return;
    }
    let Some(player) = host.server_players().into_iter()
        .filter(|player| player.dimension == dimension)
        .min_by(|a, b| {
            let far = |p: &[f64; 3]| (0..3).map(|axis| (p[axis] - at[axis]).powi(2)).sum::<f64>();
            far(&a.position).total_cmp(&far(&b.position))
        })
        .filter(|player| (0..3).all(|axis| (player.position[axis] - at[axis]).abs() <= COUNT_RADIUS))
    else { return };
    LAST_ASK.store(now, Ordering::Relaxed);
    host.action_bar(player.uuid, Text::tr("veio.village.need_houses", &[&people, &beds]));
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
