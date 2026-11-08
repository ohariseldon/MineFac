//! O cenário no cliente, pelo motor e não só pelo shader: funciona até no visual Leve, sem pacote.
//!
//! - **Cores das estações:** a grama, as folhas, a serrapilheira e a água de cada bioma mudam
//!   com a estação (`biome_colors`). Primavera verde-clara, verão cheio, outono dourado e
//!   avermelhado, inverno pardo e acinzentado. O jogo refaz o terreno com as cores novas: só
//!   quando a mistura das estações anda um quarto (umas 4 vezes por virada), nunca por quadro.
//! - **Neblina:** a névoa do jogo (`render_fog`) fecha nas manhãs de outono e primavera, mais nos
//!   vales, e ganha a cor parda da fumaça perto de fábricas poluídas.
use super::{ambience, climate};
use horizon_client::mod_api::{BiomeColors, FogData, FogType, Host};
use std::sync::Mutex;

/// Pesos das estações em uso nas cores dos biomas, e o passo em que foram calculados.
static IN_USE: Mutex<Option<([f32; 4], i64)>> = Mutex::new(None);

/// Passo da mistura: um quarto da virada de estação. Cada passo novo refaz as cores.
fn step_of(weights: [f32; 4]) -> i64 {
    weights.iter().enumerate().map(|(i, w)| ((w * 4.0).round() as i64) << (i * 4)).sum()
}

/// Tick da janela (cliente): relê a estação a cada 5 segundos e refaz as cores quando o passo muda.
pub(crate) fn client_tick(host: &mut dyn Host) {
    static TICKS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    let clock = TICKS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    if clock % 100 != 0 {
        return;
    }
    let Some(player) = host.player() else { return };
    if player.dimension != "minecraft:overworld" {
        return;
    }
    let weights = climate::season_weights(host.day_time());
    let step = step_of(weights);
    let changed = {
        let mut in_use = IN_USE.lock().unwrap_or_else(|poison| poison.into_inner());
        let changed = in_use.is_none_or(|(_, old)| old != step);
        if changed {
            *in_use = Some((weights.map(|w| (w * 4.0).round() / 4.0), step));
        }
        changed
    };
    if changed {
        host.refresh_biome_colors();
    }
}

fn rgb(color: u32) -> [f32; 3] {
    [((color >> 16) & 0xff) as f32, ((color >> 8) & 0xff) as f32, (color & 0xff) as f32]
}

fn pack([r, g, b]: [f32; 3]) -> u32 {
    let c = |v: f32| v.round().clamp(0.0, 255.0) as u32;
    (c(r) << 16) | (c(g) << 8) | c(b)
}

fn mix(a: [f32; 3], b: u32, t: f32) -> [f32; 3] {
    let b = rgb(b);
    [a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t, a[2] + (b[2] - a[2]) * t]
}

/// A cor de uma estação a partir da do bioma: (primavera, verão, outono, inverno).
fn seasonal(base: u32, weights: [f32; 4], targets: [(u32, f32); 4]) -> u32 {
    let base = rgb(base);
    let mut out = [0.0; 3];
    for (w, (target, amount)) in weights.iter().zip(targets) {
        let c = mix(base, target, amount);
        for k in 0..3 {
            out[k] += c[k] * w;
        }
    }
    pack(out)
}

/// Biomas que não mudam: os nevados e gelados (sempre inverno), os quentes (sem outono) e
/// os de fora do mundo de cima.
pub(crate) fn keeps_its_colours(biome: &str) -> bool {
    const FIXED: [&str; 14] = ["snowy", "frozen", "ice", "desert", "badlands", "jungle", "mangrove", "savanna",
        "mushroom", "deep_dark", "lush", "dripstone", "nether", "end"];
    let name = biome.strip_prefix("minecraft:").unwrap_or(biome);
    FIXED.iter().any(|part| name.contains(part)) || name.starts_with("the_") || name.contains("warm")
}

/// As cores de `biome` nesta estação.
pub(crate) fn recolour(biome: &str, colors: &mut BiomeColors, weights: [f32; 4]) {
    if keeps_its_colours(biome) {
        return;
    }
    colors.grass = seasonal(colors.grass, weights,
        [(0x7cc94a, 0.22), (0x5f9e35, 0.08), (0xb8a24c, 0.50), (0x8e977c, 0.58)]);
    colors.foliage = seasonal(colors.foliage, weights,
        [(0x6fcf4a, 0.22), (0x4f8a2a, 0.10), (0xd27a28, 0.62), (0x857a58, 0.60)]);
    colors.dry_foliage = seasonal(colors.dry_foliage, weights,
        [(0x9a7a4a, 0.10), (0x9a7a4a, 0.0), (0xa8582a, 0.45), (0x7a6a56, 0.40)]);
    colors.water = seasonal(colors.water, weights,
        [(0x4c7a88, 0.18), (0x3d8fd6, 0.10), (0x3f6f8c, 0.15), (0x3f5d74, 0.35)]);
}

/// Gancho `biome_colors` do cliente.
pub(crate) fn biome_colors(biome: &str, colors: &mut BiomeColors) {
    let Some((weights, _)) = *IN_USE.lock().unwrap_or_else(|poison| poison.into_inner()) else { return };
    recolour(biome, colors, weights);
}

/// Quanto de neblina há agora, 0..1: manhãs úmidas (outono e primavera mais), vales, fumaça.
pub(crate) fn mist(day_time: i64, camera_y: f64, smog: f32) -> f32 {
    let t = day_time.rem_euclid(24000) as f32;
    // O sol nasce em 0: a neblina vem antes do amanhecer e se desfaz até umas 9h (3.000).
    let dawn = if t >= 22500.0 { (t - 22500.0) / 1500.0 } else { 1.0 - ((t - 500.0) / 3000.0).clamp(0.0, 1.0) };
    let w = climate::season_weights(day_time);
    let humid = 0.70 * w[climate::SPRING] + 0.20 * w[climate::SUMMER] + 1.0 * w[climate::AUTUMN] + 0.55 * w[climate::WINTER];
    // Nos vales e beiras de rio ela assenta; no alto do morro, some.
    let valley = (1.0 - ((camera_y as f32 - 64.0) / 40.0)).clamp(0.0, 1.0);
    (dawn * humid * (0.35 + 0.45 * valley) + smog * 0.75).clamp(0.0, 0.9)
}

/// Gancho `render_fog` do cliente.
pub(crate) fn render_fog(host: &dyn Host, camera: [f64; 3], fog_type: FogType, fog: &mut FogData) {
    if fog_type != FogType::Atmospheric {
        return;
    }
    let Some(player) = host.player() else { return };
    if player.dimension != "minecraft:overworld" {
        return;
    }
    let pos = [camera[0].floor() as i32, camera[1].floor() as i32, camera[2].floor() as i32];
    let smog = ambience::smog(player.dimension, pos);
    let amount = mist(host.day_time(), camera[1], smog);
    thicken(fog, amount, smog);
}

/// Aproxima a névoa do jogo: o fim desce do horizonte até ~28 blocos na neblina mais densa,
/// e a cor vai para o branco-azulado da manhã ou o pardo da fumaça.
pub(crate) fn thicken(fog: &mut FogData, amount: f32, smog: f32) {
    if amount <= 0.01 {
        return;
    }
    let far = fog.render_distance_end.max(32.0);
    let end = far + (28.0 - far) * amount;
    fog.environmental_end = fog.environmental_end.min(end);
    fog.environmental_start = fog.environmental_start.min(end * (0.35 - 0.3 * amount));
    let tint = if smog > amount * 0.5 { [0.50, 0.46, 0.39] } else { [0.78, 0.81, 0.85] };
    let k = amount * 0.6;
    for i in 0..3 {
        fog.color[i] = (fog.color[i] + (tint[i] * fog.color[i].max(0.25).min(1.0) / 0.85 - fog.color[i]) * k).clamp(0.0, 1.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn summer() -> [f32; 4] { [0.0, 1.0, 0.0, 0.0] }
    fn autumn() -> [f32; 4] { [0.0, 0.0, 1.0, 0.0] }
    fn winter() -> [f32; 4] { [0.0, 0.0, 0.0, 1.0] }

    #[test]
    fn autumn_turns_leaves_warm_and_winter_dulls_the_grass() {
        let plains = BiomeColors { grass: 0x91bd59, foliage: 0x77ab2f, dry_foliage: 0xa0a069, water: 0x3f76e4 };
        let mut fall = plains;
        recolour("minecraft:plains", &mut fall, autumn());
        let [r, g, _] = rgb(fall.foliage);
        assert!(r > g, "folhas do outono puxam para o laranja: {:06x}", fall.foliage);
        let mut cold = plains;
        recolour("minecraft:forest", &mut cold, winter());
        let [r, g, b] = rgb(cold.grass);
        let [r0, g0, b0] = rgb(plains.grass);
        assert!((g - r) < (g0 - r0) && (g - b) < (g0 - b0), "inverno tira o verde da grama");
        let mut snowy = plains;
        recolour("minecraft:snowy_plains", &mut snowy, autumn());
        assert_eq!(snowy, plains, "bioma nevado não muda");
        let mut hot = plains;
        recolour("minecraft:savanna_plateau", &mut hot, winter());
        assert_eq!(hot, plains);
    }

    #[test]
    fn summer_barely_changes_a_biome() {
        let plains = BiomeColors { grass: 0x91bd59, foliage: 0x77ab2f, dry_foliage: 0xa0a069, water: 0x3f76e4 };
        let mut same = plains;
        recolour("minecraft:plains", &mut same, summer());
        let close = |a: u32, b: u32| rgb(a).iter().zip(rgb(b)).all(|(x, y)| (x - y).abs() < 30.0);
        assert!(close(same.grass, plains.grass) && close(same.foliage, plains.foliage));
    }

    #[test]
    fn colour_steps_change_about_four_times_per_turn() {
        let day = 24000;
        let mut steps = std::collections::BTreeSet::new();
        for t in (0..climate::SEASON_DAYS * 4 * day).step_by(2000) {
            steps.insert(step_of(climate::season_weights(t)));
        }
        assert!(steps.len() <= 4 * 5, "poucas reconstruções por ano: {}", steps.len());
    }

    #[test]
    fn mist_settles_on_autumn_mornings_in_the_valleys() {
        let autumn_dawn = 2 * climate::SEASON_DAYS * 24000 + 3 * 24000 + 600;
        let autumn_noon = autumn_dawn + 5400;
        assert!(mist(autumn_dawn, 64.0, 0.0) > 0.6);
        assert!(mist(autumn_noon, 64.0, 0.0) < 0.05);
        assert!(mist(autumn_dawn, 140.0, 0.0) < mist(autumn_dawn, 64.0, 0.0), "no alto, menos");
        assert!(mist(autumn_noon, 64.0, 1.0) > 0.5, "fumaça da fábrica a qualquer hora");
        let mut fog = FogData { color: [0.7, 0.8, 1.0, 1.0], environmental_start: 300.0, environmental_end: 400.0,
            render_distance_start: 180.0, render_distance_end: 192.0, sky_end: 192.0, cloud_end: 192.0 };
        thicken(&mut fog, 0.8, 0.0);
        assert!(fog.environmental_end < 70.0 && fog.environmental_start < fog.environmental_end);
    }
}
