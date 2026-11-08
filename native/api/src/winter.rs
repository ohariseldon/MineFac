//! Neve e gelo do inverno: com chuva abaixo de zero, a neve cobre aos poucos o chão, as folhas e
//! a grama perto dos jogadores, e os lagos congelam; na primavera tudo derrete e volta a ser
//! como era.
//!
//! O clima de `climate.rs` não sabe o bioma, e o vanilla só neva nos biomas frios. Aqui a neve
//! cai em qualquer bioma temperado quando o nosso inverno esfria, e só onde nós a pusemos ela
//! derrete: cada bloco coberto ou congelado fica anotado em `<mundo>/veio/inverno.txt`, então
//! a neve dos biomas nevados do vanilla nunca é tocada. Biomas quentes (deserto, savana,
//! badlands, selva) ficam de fora pela cor da grama que o jogo informa da coluna.
//!
//! Custo: a cada meio segundo, 6 colunas sorteadas por jogador num raio de 40 blocos (uma
//! leitura pequena de colunas) e uma edição em lote com o que mudou; o degelo trata até 24
//! blocos anotados por vez, só perto de jogadores (chunks carregados).
use super::climate;
use horizon_client::mod_api::{Host, NativeBlockEdit, NativeBlockRun, NativeHeightmap};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

const OVERWORLD: &str = "minecraft:overworld";
/// Ticks entre passadas: meio segundo.
const EVERY: u64 = 10;
const COLUMNS_PER_PLAYER: usize = 6;
const RADIUS: i32 = 40;
/// Blocos anotados que derretem por passada.
const THAW_PER_PASS: usize = 24;
/// Distância de um jogador até onde o degelo mexe (chunks carregados).
const THAW_RANGE: i32 = 112;
/// Teto de blocos anotados: um mundo enorme no inverno não cresce o arquivo sem fim.
const MAX_RECORDS: usize = 60_000;
/// Chuva mínima (0..1) para nevar.
const PRECIPITATION: f32 = 0.2;
/// Abaixo disto a neve acumula; acima de `MELT` ela derrete.
const SNOW_BELOW: f32 = 0.0;
const MELT: f32 = 2.0;
/// Mais frio que isto, a neve engrossa (2 camadas) e a água parada vira gelo.
const DEEP_COLD: f32 = -4.0;
const FREEZE_WATER: f32 = -2.0;
const SAVE_EVERY: u64 = 1200;

/// O que nós mudamos num bloco, para desfazer no degelo.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Mark {
    /// Camada de neve posta sobre o bloco de baixo (que era grama, se `grass`).
    Snow { grass: bool },
    /// Água parada que virou gelo.
    Ice,
}

#[derive(Default)]
struct State {
    world: Option<PathBuf>,
    marks: HashMap<[i32; 3], Mark>,
    dirty: bool,
    saved_at: u64,
}

static STATE: Mutex<Option<State>> = Mutex::new(None);

fn with_state<T>(run: impl FnOnce(&mut State) -> T) -> T {
    let mut guard = STATE.lock().unwrap_or_else(|poison| poison.into_inner());
    run(guard.get_or_insert_with(State::default))
}

/// Blocos sobre os quais a neve pousa: chão natural e copas. Construções e máquinas, não.
pub(crate) fn snowable(name: &str) -> bool {
    let name = name.strip_prefix("minecraft:").unwrap_or(name);
    if name.contains(':') {
        return false;
    }
    matches!(name, "grass_block" | "dirt" | "coarse_dirt" | "podzol" | "rooted_dirt" | "mycelium" | "stone"
        | "andesite" | "diorite" | "granite" | "gravel" | "moss_block" | "mud" | "packed_mud" | "clay"
        | "tuff" | "calcite" | "deepslate" | "cobblestone" | "mossy_cobblestone")
        || name.ends_with("_leaves")
}

/// Pela cor da grama do bioma na coluna (0xRRGGBB): savana, deserto e badlands são amarelados,
/// a selva é verde vivo. Nesses a neve não cai.
pub(crate) fn hot_biome(grass: u32) -> bool {
    let r = (grass >> 16) & 0xff;
    let g = (grass >> 8) & 0xff;
    r >= 0xa8 || (g >= 0xc4 && r < 0x70)
}

/// O que fazer numa coluna: `top` o bloco mais alto que barra o movimento, `above` o de cima.
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Change {
    /// Pôr neve em `top + 1` (e marcar a grama de baixo como nevada).
    Cover { grass: bool },
    /// Engrossar a neve de `top + 1` de `from` para `from + 1` camadas.
    Thicken { from: u8 },
    /// Congelar a água parada em `top`.
    Freeze,
}

pub(crate) fn decide(top: &str, above: &str, temperature: f32, rain: f32) -> Option<Change> {
    if rain < PRECIPITATION || temperature >= SNOW_BELOW {
        return None;
    }
    let top = top.strip_prefix("minecraft:").unwrap_or(top);
    let above = above.strip_prefix("minecraft:").unwrap_or(above);
    if top == "water" {
        return (temperature < FREEZE_WATER && above == "air").then_some(Change::Freeze);
    }
    if above == "snow" {
        return (temperature < DEEP_COLD).then_some(Change::Thicken { from: 1 });
    }
    if above == "air" && snowable(top) {
        return Some(Change::Cover { grass: top == "grass_block" });
    }
    None
}

const PALETTE: [&str; 7] = [
    "minecraft:snow[layers=1]", "minecraft:snow[layers=2]", "minecraft:ice", "minecraft:grass_block[snowy=true]",
    "minecraft:air", "minecraft:water", "minecraft:grass_block[snowy=false]",
];
const SNOW_1: u32 = 0;
const SNOW_2: u32 = 1;
const ICE: u32 = 2;
const GRASS_SNOWY: u32 = 3;
const AIR: u32 = 4;
const WATER: u32 = 5;
const GRASS_BARE: u32 = 6;
const FILTERS: [&str; 6] = [
    "minecraft:air", "minecraft:snow[layers=1]", "minecraft:water[level=0]", "minecraft:grass_block",
    "minecraft:snow", "minecraft:ice",
];
const ONLY_AIR: u32 = 0;
const ONLY_SNOW_1: u32 = 1;
const ONLY_SOURCE: u32 = 2;
const ONLY_GRASS: u32 = 3;
const ONLY_SNOW: u32 = 4;
const ONLY_ICE: u32 = 5;

fn run(pos: [i32; 3], block: u32, only: u32) -> NativeBlockRun {
    NativeBlockRun { x: pos[0], z: pos[2], y: pos[1], height: 1, surface: None, block, only: Some(only) }
}

/// Sorteio barato e repetível por passada e jogador.
fn scatter(seed: u64, index: usize) -> (i32, i32) {
    let mut h = seed ^ (index as u64).wrapping_mul(0x9e37_79b9_7f4a_7c15);
    h ^= h >> 33;
    h = h.wrapping_mul(0xff51_afd7_ed55_8ccd);
    h ^= h >> 33;
    let span = (RADIUS * 2 + 1) as u64;
    (((h % span) as i32) - RADIUS, (((h >> 20) % span) as i32) - RADIUS)
}

fn file(world: &Path) -> PathBuf {
    world.join("veio").join("inverno.txt")
}

pub(crate) fn save_text(marks: &HashMap<[i32; 3], Mark>) -> String {
    let mut lines: Vec<String> = marks.iter().map(|(p, mark)| {
        let kind = match mark { Mark::Snow { grass: true } => "g", Mark::Snow { grass: false } => "s", Mark::Ice => "i" };
        format!("{kind} {} {} {}", p[0], p[1], p[2])
    }).collect();
    lines.sort();
    lines.join("\n")
}

pub(crate) fn load_text(text: &str) -> HashMap<[i32; 3], Mark> {
    text.lines().filter_map(|line| {
        let mut parts = line.split_whitespace();
        let mark = match parts.next()? { "g" => Mark::Snow { grass: true }, "s" => Mark::Snow { grass: false }, "i" => Mark::Ice, _ => return None };
        let mut n = || parts.next()?.parse::<i32>().ok();
        Some(([n()?, n()?, n()?], mark))
    }).collect()
}

fn save(state: &mut State) {
    let Some(world) = &state.world else { return };
    let path = file(world);
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let temp = path.with_extension("tmp");
    if std::fs::write(&temp, save_text(&state.marks)).is_ok() && std::fs::rename(&temp, &path).is_ok() {
        state.dirty = false;
    }
}

/// Tick do servidor.
pub(crate) fn tick(host: &mut dyn Host, now: u64) {
    if now % EVERY != 0 || host.dimension() != OVERWORLD {
        return;
    }
    let players: Vec<[i32; 3]> = host.server_players().into_iter()
        .filter(|p| p.dimension == OVERWORLD)
        .map(|p| [p.position[0].floor() as i32, p.position[1].floor() as i32, p.position[2].floor() as i32])
        .take(8)
        .collect();
    let world = host.world_dir();
    let rain = host.weather().0;
    let day_time = host.day_time();
    let mut runs = Vec::new();
    with_state(|state| {
        if world != state.world && world.is_some() {
            if state.dirty { save(state); }
            state.marks = world.as_deref().and_then(|dir| std::fs::read_to_string(file(dir)).ok())
                .map(|text| load_text(&text)).unwrap_or_default();
            state.world = world.clone();
        }
        if state.world.is_none() || players.is_empty() {
            return;
        }
        // Cobrir: colunas sorteadas perto de cada jogador.
        let probe = climate::temperature(day_time, players[0][1].max(64), rain, OVERWORLD);
        if rain >= PRECIPITATION && probe < SNOW_BELOW + 6.0 && state.marks.len() < MAX_RECORDS {
            let columns: Vec<(i32, i32)> = players.iter().enumerate().flat_map(|(i, p)| {
                (0..COLUMNS_PER_PLAYER).map(move |k| {
                    let (dx, dz) = scatter(now ^ ((i as u64) << 48), k);
                    (p[0] + dx, p[2] + dz)
                })
            }).collect();
            if let Ok(read) = host.read_columns(OVERWORLD, &columns, NativeHeightmap::MotionBlocking, (1, 1)) {
                for (i, &(x, z)) in columns.iter().enumerate() {
                    let (Some(Some(base)), Some(&top), Some(&above)) =
                        (read.bases.get(i), read.blocks.get(i * 2), read.blocks.get(i * 2 + 1)) else { continue };
                    if read.tints.get(i * 3).is_some_and(|&grass| hot_biome(grass)) {
                        continue;
                    }
                    let name = |index: u16| read.palette.get(index as usize).map_or("minecraft:air", String::as_str);
                    let y = *base;
                    let air = climate::temperature(day_time, y + 1, rain, OVERWORLD);
                    let at = [x, y + 1, z];
                    match decide(name(top), name(above), air, rain) {
                        Some(Change::Cover { grass }) => {
                            runs.push(run(at, SNOW_1, ONLY_AIR));
                            if grass { runs.push(run([x, y, z], GRASS_SNOWY, ONLY_GRASS)); }
                            state.marks.insert(at, Mark::Snow { grass });
                            state.dirty = true;
                        }
                        Some(Change::Thicken { .. }) => {
                            // Só a neve que nós pusemos engrossa; a de bioma nevado fica como está.
                            if matches!(state.marks.get(&at), Some(Mark::Snow { .. })) {
                                runs.push(run(at, SNOW_2, ONLY_SNOW_1));
                            }
                        }
                        Some(Change::Freeze) => {
                            runs.push(run([x, y, z], ICE, ONLY_SOURCE));
                            state.marks.insert([x, y, z], Mark::Ice);
                            state.dirty = true;
                        }
                        None => {}
                    }
                }
            }
        }
        // Degelo: o que nós cobrimos perto dos jogadores, quando o ar esquenta.
        if probe > MELT && !state.marks.is_empty() {
            let near: Vec<([i32; 3], Mark)> = state.marks.iter()
                .filter(|(p, _)| players.iter().any(|q| (p[0] - q[0]).abs() <= THAW_RANGE && (p[2] - q[2]).abs() <= THAW_RANGE))
                .filter(|(p, _)| climate::temperature(day_time, p[1], 0.0, OVERWORLD) > MELT)
                .take(THAW_PER_PASS)
                .map(|(p, m)| (*p, *m))
                .collect();
            for (p, mark) in near {
                match mark {
                    Mark::Snow { grass } => {
                        runs.push(run(p, AIR, ONLY_SNOW));
                        if grass { runs.push(run([p[0], p[1] - 1, p[2]], GRASS_BARE, ONLY_GRASS)); }
                    }
                    Mark::Ice => runs.push(run(p, WATER, ONLY_ICE)),
                }
                state.marks.remove(&p);
                state.dirty = true;
            }
        }
        if state.dirty && now.saturating_sub(state.saved_at) >= SAVE_EVERY {
            state.saved_at = now;
            save(state);
        }
    });
    if runs.is_empty() {
        return;
    }
    let edit = NativeBlockEdit {
        palette: PALETTE.iter().map(|s| (*s).to_owned()).collect(),
        filters: FILTERS.iter().map(|s| (*s).to_owned()).collect(),
        runs,
    };
    let _ = host.edit_blocks(OVERWORLD, &edit);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snow_covers_natural_ground_only_in_cold_rain() {
        assert_eq!(decide("minecraft:grass_block", "minecraft:air", -3.0, 1.0), Some(Change::Cover { grass: true }));
        assert_eq!(decide("minecraft:oak_leaves", "minecraft:air", -1.0, 0.8), Some(Change::Cover { grass: false }));
        assert_eq!(decide("minecraft:grass_block", "minecraft:air", 3.0, 1.0), None, "acima de zero chove");
        assert_eq!(decide("minecraft:grass_block", "minecraft:air", -5.0, 0.0), None, "sem precipitação");
        assert_eq!(decide("veio:conveyor", "minecraft:air", -5.0, 1.0), None, "máquinas ficam limpas");
        assert_eq!(decide("minecraft:oak_planks", "minecraft:air", -5.0, 1.0), None, "construções também");
        assert_eq!(decide("minecraft:water", "minecraft:air", -1.0, 1.0), None, "água só congela no frio forte");
        assert_eq!(decide("minecraft:water", "minecraft:air", -6.0, 1.0), Some(Change::Freeze));
        assert_eq!(decide("minecraft:dirt", "minecraft:snow", -6.0, 1.0), Some(Change::Thicken { from: 1 }));
    }

    #[test]
    fn hot_biomes_are_left_out_by_their_grass_colour() {
        assert!(hot_biome(0xbfb755), "savana e deserto");
        assert!(hot_biome(0x59c93c), "selva");
        assert!(!hot_biome(0x91bd59), "planície");
        assert!(!hot_biome(0x86b783), "taiga");
    }

    #[test]
    fn marks_round_trip_through_the_world_file() {
        let mut marks = HashMap::new();
        marks.insert([1, 70, -3], Mark::Snow { grass: true });
        marks.insert([-8, 63, 12], Mark::Ice);
        marks.insert([5, 90, 5], Mark::Snow { grass: false });
        assert_eq!(load_text(&save_text(&marks)), marks);
    }

    #[test]
    fn scattered_columns_stay_within_the_radius() {
        for k in 0..200 {
            let (dx, dz) = scatter(12345 + k as u64, k);
            assert!(dx.abs() <= RADIUS && dz.abs() <= RADIUS);
        }
    }
}
