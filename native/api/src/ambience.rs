//! Imersão: o que o mundo mostra da fábrica, sem custo na simulação.
//!
//! - Máquinas quentes trabalhando (fornos, geradores, caldeira, geotérmico, cozinha) soltam
//!   fumaça ou vapor pela chaminé e, no jogo de um jogador só, acendem uma luz colorida no
//!   pacote de shaders (`visuals`), com uma oscilação leve de chama.
//! - O pacote de shaders recebe por quadro a estação, a temperatura do ar e a fumaça industrial
//!   em volta da câmera (`shader_params`, lidos como `frame.mods[0]`).
//!
//! Custos: cada máquina quente se registra uma vez por segundo (um `HashMap`, no máximo 2.048
//! entradas); a fumaça só sai perto de jogadores e no máximo a cada 2 segundos por máquina; a
//! luz escolhe as 8 mais próximas em 32 blocos. Nada é salvo: o registro recomeça ao abrir o mundo.
//! Num servidor dedicado o cliente não vê o registro do servidor: lá ficam só a fumaça e o vapor.
use super::*;
use horizon_client::mod_api::{NativeColoredLight, ParticleOptions, ParticleType};
use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};

#[derive(Clone, Copy)]
pub(crate) struct Heat {
    /// Cor da luz, 0xRRGGBB.
    pub rgb: u32,
    /// Alcance da luz, em blocos.
    pub radius: f32,
    /// O que sai por cima enquanto trabalha.
    pub exhaust: Exhaust,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Exhaust { Smoke, Steam, Embers, None }

pub(crate) fn heat(block: &str) -> Option<Heat> {
    let (rgb, radius, exhaust) = match block {
        GENERATOR | production::BIOMASS_GENERATOR => (0xff8a3a, 5.0, Exhaust::Smoke),
        production::STEAM_ENGINE => (0xff9040, 5.0, Exhaust::Steam),
        GEOTHERMAL => (0xff5a1a, 6.5, Exhaust::Embers),
        ALLOY => (0xff7a2a, 6.0, Exhaust::Smoke),
        production::ELECTRIC_FURNACE => (0xff9a55, 5.0, Exhaust::None),
        production::KITCHEN => (0xffb060, 3.5, Exhaust::Steam),
        // Névoa de tinta saindo da cabine de pintura.
        production::PAINT_BOOTH => (0xc890ff, 3.0, Exhaust::Steam),
        TESLA => (0x8cc4ff, 6.0, Exhaust::None),
        _ => return None,
    };
    Some(Heat { rgb, radius, exhaust })
}

#[derive(Clone, Copy)]
struct Seen { rgb: u32, radius: f32, tick: u64 }

#[derive(Default)]
struct Registry {
    hot: HashMap<(&'static str, [i32; 3]), Seen, FastHash>,
    listeners: Vec<(String, [f64; 3])>,
    listeners_at: Option<u64>,
    pruned_at: u64,
}

const MAX_HOT: usize = 2048;
/// Uma máquina sem notícia há 3 segundos parou ou descarregou.
const STALE: u64 = 60;
/// Distância de jogador para soltar fumaça: além disto ninguém vê.
const PLUME_RANGE: f64 = 40.0;

static REGISTRY: std::sync::LazyLock<Mutex<Registry>> = std::sync::LazyLock::new(Default::default);

fn registry<T>(run: impl FnOnce(&mut Registry) -> T) -> T {
    run(&mut REGISTRY.lock().unwrap_or_else(|poison| poison.into_inner()))
}

/// Defasagem por posição: as máquinas não fazem a conta todas no mesmo tick.
fn stagger([x, y, z]: [i32; 3]) -> u64 {
    (x.wrapping_mul(73_856_093) ^ y.wrapping_mul(19_349_663) ^ z.wrapping_mul(83_492_791)) as u32 as u64
}

/// Tick do mundo: relê os jogadores a cada segundo e esquece máquinas paradas.
pub(super) fn maintain(host: &dyn Host, clock: u64) {
    let refresh = registry(|r| r.listeners_at.is_none_or(|at| clock.saturating_sub(at) >= 20 || clock < at));
    if !refresh { return; }
    let listeners: Vec<_> = host.server_players().into_iter().map(|p| (p.dimension, p.position)).collect();
    registry(|r| {
        r.listeners = listeners;
        r.listeners_at = Some(clock);
        if clock.saturating_sub(r.pruned_at) >= 100 || clock < r.pruned_at {
            r.hot.retain(|_, seen| clock.saturating_sub(seen.tick) <= STALE && seen.tick <= clock);
            r.pruned_at = clock;
        }
    });
}

/// Depois do tick de uma máquina: registra o calor e solta a fumaça, uma vez por segundo.
pub(super) fn observe(host: &mut dyn Host, pos: [i32; 3], entity: &ModBlockEntity, clock: u64) {
    let Some(heat) = heat(&entity.id) else { return };
    let offset = stagger(pos);
    if (clock + offset) % 20 != 0 { return; }
    let working = number(entity, "Status") == 1;
    let dimension = host.dimension();
    let center = [f64::from(pos[0]) + 0.5, f64::from(pos[1]) + 0.5, f64::from(pos[2]) + 0.5];
    let near = registry(|r| {
        let key = (dimension, pos);
        if working {
            if r.hot.len() < MAX_HOT || r.hot.contains_key(&key) {
                r.hot.insert(key, Seen { rgb: heat.rgb, radius: heat.radius, tick: clock });
            }
        } else {
            r.hot.remove(&key);
        }
        working && r.listeners.iter().any(|(d, p)| d == dimension
            && (0..3).map(|i| (p[i] - center[i]).powi(2)).sum::<f64>() <= PLUME_RANGE * PLUME_RANGE)
    });
    // Fumaça a cada 2 segundos: metade das vezes em que a máquina se registra.
    if !near || (clock + offset) % 40 != 0 { return; }
    let top = [center[0], f64::from(pos[1]) + 1.05, center[2]];
    match heat.exhaust {
        Exhaust::Smoke => {
            host.send_particles(&ParticleOptions::simple(ParticleType::CampfireCosySmoke), None, top, [0.08, 0.05, 0.08], 0.01, 1);
        }
        Exhaust::Steam => {
            host.send_particles(&ParticleOptions::simple(ParticleType::Cloud), None, top, [0.12, 0.05, 0.12], 0.02, 2);
        }
        Exhaust::Embers => {
            host.send_particles(&ParticleOptions::simple(ParticleType::Lava), None, top, [0.15, 0.0, 0.15], 0.0, 1);
            host.send_particles(&ParticleOptions::simple(ParticleType::LargeSmoke), None, top, [0.1, 0.05, 0.1], 0.01, 1);
        }
        Exhaust::None => {}
    }
}

/// Até `MAX_LIGHTS` luzes das máquinas quentes perto da câmera, as mais próximas primeiro.
pub(crate) const MAX_LIGHTS: usize = 8;
const LIGHT_RANGE: f64 = 32.0;

pub(crate) fn lights_near(dimension: &str, eye: [f64; 3], clock: u64, seconds: f32) -> Vec<NativeColoredLight> {
    // Só as 8 mais próximas ficam guardadas: num pátio com mil máquinas, nada de lista grande por quadro.
    let mut found: Vec<(f64, [i32; 3], Seen)> = Vec::with_capacity(MAX_LIGHTS + 1);
    registry(|r| {
        for ((d, pos), seen) in &r.hot {
            if *d != dimension || clock.saturating_sub(seen.tick) > STALE { continue; }
            let distance = (0..3).map(|i| (f64::from(pos[i]) + 0.5 - eye[i]).powi(2)).sum::<f64>();
            if distance > LIGHT_RANGE * LIGHT_RANGE { continue; }
            let entry = (distance, *pos, *seen);
            let at = found.partition_point(|a| a.0.total_cmp(&entry.0).then(a.1.cmp(&entry.1)).is_lt());
            if at < MAX_LIGHTS {
                found.insert(at, entry);
                found.truncate(MAX_LIGHTS);
            }
        }
    });
    found.into_iter().map(|(_, pos, seen)| {
        // Oscilação de chama: duas senoides lentas com fase própria de cada máquina.
        let phase = (stagger(pos) % 6283) as f32 / 1000.0;
        let flicker = 0.88 + 0.08 * (seconds * 7.3 + phase).sin() + 0.04 * (seconds * 17.0 + phase * 2.0).sin();
        NativeColoredLight {
            at: [f64::from(pos[0]) + 0.5, f64::from(pos[1]) + 0.9, f64::from(pos[2]) + 0.5],
            rgb: seen.rgb,
            radius: seen.radius,
            intensity: if seen.rgb == 0x8cc4ff { 0.7 } else { 0.9 * flicker },
        }
    }).collect()
}

fn seconds() -> f32 {
    static START: std::sync::OnceLock<std::time::Instant> = std::sync::OnceLock::new();
    START.get_or_init(std::time::Instant::now).elapsed().as_secs_f32() % 3600.0
}

/// Gancho de quadro do cliente: as luzes das máquinas.
pub(super) fn visuals(host: &dyn Host, lights: &mut Vec<NativeColoredLight>) {
    let Some(player) = host.player() else { return };
    lights.extend(lights_near(player.dimension, [player.x, player.y + 1.6, player.z], defense::clock(), seconds()));
}

/// Fumaça industrial em volta da câmera, 0..1, relida no máximo 4 vezes por segundo.
pub(crate) fn smog(dimension: &str, pos: [i32; 3]) -> f32 {
    static AT: AtomicU64 = AtomicU64::new(0);
    static VALUE: AtomicU32 = AtomicU32::new(0);
    let now = (seconds() * 1000.0) as u64 + 1;
    let last = AT.load(Ordering::Relaxed);
    if now.saturating_sub(last) >= 250 || now < last {
        AT.store(now, Ordering::Relaxed);
        let local = pollution::local(dimension, pos);
        VALUE.store(smog_amount(local).to_bits(), Ordering::Relaxed);
    }
    f32::from_bits(VALUE.load(Ordering::Relaxed))
}

/// Do nível de poluição do chunk à densidade da fumaça no ar: nada abaixo de um quarto do
/// aviso, tudo no nível de ataque.
pub(crate) fn smog_amount(local: f32) -> f32 {
    let start = pollution::NOTICE * 0.25;
    ((local - start) / (pollution::RAID - start)).clamp(0.0, 1.0)
}

/// Os valores do pacote de shaders: `frame.mods[0]` = (fase do ano, temperatura / 50, fumaça, 1).
pub(super) fn shader_params(host: &dyn Host, params: &mut [[f32; 4]; 8]) {
    let Some(player) = host.player() else { return };
    let day_time = host.day_time();
    let (rain, _) = host.weather();
    let y = player.y.floor() as i32;
    let pos = [player.x.floor() as i32, y, player.z.floor() as i32];
    let temperature = climate::temperature(day_time, y, rain, player.dimension);
    params[0] = [climate::season_phase(day_time), temperature / 50.0, smog(player.dimension, pos), 1.0];
    // x: o mod já pinta a grama e as folhas pela estação (`scenery`); o pacote só varia por bloco.
    params[1] = [1.0, 0.0, 0.0, 0.0];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_working_hot_machines_light_up_and_the_nearest_win() {
        registry(|r| r.hot.clear());
        let dim = "veio:ambience-test";
        for i in 0..12 {
            registry(|r| r.hot.insert((dim, [i * 2, 64, 0]), Seen { rgb: 0xff8a3a, radius: 5.0, tick: 100 }));
        }
        registry(|r| r.hot.insert(("minecraft:the_nether", [0, 64, 0]), Seen { rgb: 0xff8a3a, radius: 5.0, tick: 100 }));
        let lights = lights_near(dim, [0.5, 65.0, 0.5], 110, 0.0);
        assert_eq!(lights.len(), MAX_LIGHTS);
        assert!(lights.windows(2).all(|w| w[0].at[0] <= w[1].at[0]), "mais próximas primeiro");
        assert!(lights.iter().all(|l| l.intensity > 0.0 && l.intensity <= 1.0));
        assert!(lights_near(dim, [0.5, 65.0, 0.5], 100 + STALE + 1, 0.0).is_empty(), "parada há 3 s apaga");
        assert!(lights_near(dim, [500.0, 65.0, 0.5], 110, 0.0).is_empty(), "longe demais");
        registry(|r| r.hot.clear());
    }

    #[test]
    fn hot_machines_are_the_ones_that_burn_or_heat() {
        assert_eq!(heat(GENERATOR).unwrap().exhaust, Exhaust::Smoke);
        assert_eq!(heat(production::STEAM_ENGINE).unwrap().exhaust, Exhaust::Steam);
        assert!(heat(PRESS).is_none() && heat(BELT).is_none());
    }

    #[test]
    fn smog_starts_below_the_notice_and_fills_at_raid_level() {
        assert_eq!(smog_amount(0.0), 0.0);
        assert_eq!(smog_amount(pollution::NOTICE * 0.25), 0.0);
        assert!(smog_amount(pollution::NOTICE) > 0.2);
        assert_eq!(smog_amount(pollution::RAID * 2.0), 1.0);
    }
}
