//! Mural da casa: quatro mapas (2 x 2) nas molduras da parede oeste, 256 x 256 blocos em volta do
//! painel, um bloco por pixel.
//!
//! O painel da casa lê uma linha de 256 colunas a cada `SCAN_EVERY` ticks (uma consulta de altura
//! e 256 leituras de bloco), então o mural inteiro se refaz em cerca de 25 segundos. Só entra o que
//! está carregado, ou seja, o que alguém já viu: colunas fora dos chunks carregados guardam o último
//! pixel conhecido, e o que nunca foi visto fica com o papel do mapa. Blocos do Veio aparecem em
//! laranja e a casa em vermelho.
//!
//! Os pixels vivem só na memória (64 KiB); ao reabrir o mundo, o mural se refaz com o que estiver
//! carregado. As imagens vão aos jogadores perto da casa por `Host::send_map` (o servidor não
//! guarda esses mapas), assim que eles chegam e depois a cada `RESEND` ticks.
use super::*;

pub(crate) const MAP_ID: i32 = 1_970_000;
pub(crate) const MAPS: usize = 4;
const SIDE: i32 = 256;
const MAP: usize = 128 * 128;
const SCAN_EVERY: i64 = 2;
const RESEND: i64 = 200;
/// Distância do painel em que o jogador recebe o mural.
const VIEW: f64 = 24.0;

// Cores de mapa (`MapColor`), id vezes 4 mais o tom.
const NONE: u8 = 0;
const GRASS: u8 = 1;
const SAND: u8 = 2;
const WOOL: u8 = 3;
const FIRE: u8 = 4;
const ICE: u8 = 5;
const PLANT: u8 = 7;
const SNOW: u8 = 8;
const CLAY: u8 = 9;
const DIRT: u8 = 10;
const STONE: u8 = 11;
const WATER: u8 = 12;
const WOOD: u8 = 13;
const ORANGE: u8 = 15;
const YELLOW: u8 = 18;
const LIGHT_GRAY: u8 = 22;
const RED: u8 = 28;
const GOLD: u8 = 30;
const PODZOL: u8 = 34;

/// A cor do bloco no topo de uma coluna.
pub(crate) fn color(block: &str) -> u8 {
    if block.starts_with("veio:") {
        return ORANGE;
    }
    let name = block.strip_prefix("minecraft:").unwrap_or(block);
    match name {
        "air" | "void_air" | "cave_air" => NONE,
        "water" | "bubble_column" | "seagrass" | "tall_seagrass" | "kelp" | "kelp_plant" => WATER,
        "lava" | "magma_block" => FIRE,
        "ice" | "packed_ice" | "blue_ice" | "frosted_ice" => ICE,
        "snow" | "snow_block" | "powder_snow" => SNOW,
        "grass_block" => GRASS,
        "podzol" => PODZOL,
        "sand" | "sandstone" | "smooth_sandstone" | "cut_sandstone" | "birch_planks" => SAND,
        "clay" => CLAY,
        "dirt" | "coarse_dirt" | "rooted_dirt" | "farmland" | "dirt_path" | "mud" | "packed_mud" | "jungle_planks" => DIRT,
        "hay_block" | "bee_nest" => YELLOW,
        "bricks" | "red_sand" | "red_sandstone" => RED,
        "bell" | "gold_block" => GOLD,
        "glass" | "glass_pane" => LIGHT_GRAY,
        _ if name.ends_with("leaves") || name.ends_with("sapling") || name.contains("vine") || name == "cactus" => PLANT,
        _ if name.ends_with("_bed") => RED,
        _ if name.ends_with("wool") || name.ends_with("carpet") => WOOL,
        _ if name.contains("oak") || name.contains("spruce") || name.contains("log") || name.contains("planks")
            || name.contains("fence") || name.contains("door") || name.contains("barrel") || name.contains("chest")
            || name.contains("bookshelf") || name.contains("composter") || name.contains("lectern") => WOOD,
        _ => STONE,
    }
}

/// Tom pela altura da coluna ao norte, como nos mapas do jogo: mais alta clareia, mais baixa escurece.
pub(crate) fn shade(height: i32, north: Option<i32>) -> u8 {
    match north {
        Some(north) if height > north => 2,
        Some(north) if height < north => 0,
        _ => 1,
    }
}

struct Mural {
    center: [i32; 3],
    pixels: Vec<u8>,
    row: i32,
    north: Vec<Option<i32>>,
    viewers: Vec<u128>,
}

static MURAL: Mutex<Option<Mural>> = Mutex::new(None);

fn mural() -> std::sync::MutexGuard<'static, Option<Mural>> {
    MURAL.lock().unwrap_or_else(|err| err.into_inner())
}

/// Lê uma linha e manda o mural a quem está perto. Chamado pelo painel da casa.
pub(crate) fn step(host: &mut dyn Host, pos: [i32; 3], time: i64) {
    let mut guard = mural();
    let mural = guard.get_or_insert_with(|| Mural {
        center: pos, pixels: vec![NONE; MAPS * MAP], row: 0, north: vec![None; SIDE as usize], viewers: Vec::new(),
    });
    if mural.center != pos {
        *mural = Mural { center: pos, pixels: vec![NONE; MAPS * MAP], row: 0, north: vec![None; SIDE as usize], viewers: Vec::new() };
    }
    if time.rem_euclid(SCAN_EVERY) == 0 {
        scan(host, mural);
    }
    if time.rem_euclid(super::CHECK_EVERY) == 0 {
        send(host, mural, time.rem_euclid(RESEND) == 0);
    }
}

fn scan(host: &dyn Host, mural: &mut Mural) {
    let [cx, _, cz] = mural.center;
    let (left, z) = (cx - SIDE / 2, cz - SIDE / 2 + mural.row);
    let columns: Vec<(i32, i32)> = (0..SIDE).map(|dx| (left + dx, z)).collect();
    let Ok(heights) = host.surface_heights(host.dimension(), &columns, NativeHeightmap::MotionBlocking) else { return };
    let row = mural.row as usize;
    for (index, height) in heights.iter().enumerate() {
        let Some(height) = *height else {
            mural.north[index] = None;
            continue;
        };
        let x = left + index as i32;
        let base = if (x - cx).abs() <= 1 && (z - cz).abs() <= 1 {
            RED
        } else {
            color(&host.block([x, height - 1, z]))
        };
        let pixel = if base == NONE { NONE } else { base * 4 + shade(height, mural.north[index]) };
        let map = (row / 128) * 2 + index / 128;
        mural.pixels[map * MAP + (row % 128) * 128 + index % 128] = pixel;
        mural.north[index] = Some(height);
    }
    mural.row = (mural.row + 1) % SIDE;
    if mural.row == 0 {
        mural.north.iter_mut().for_each(|north| *north = None);
    }
}

fn send(host: &mut dyn Host, mural: &mut Mural, everyone: bool) {
    let [cx, cy, cz] = mural.center.map(f64::from);
    let near: Vec<u128> = host.server_players().into_iter()
        .filter(|player| player.dimension == OVERWORLD)
        .filter(|player| (player.position[0] - cx).hypot(player.position[2] - cz) <= VIEW && (player.position[1] - cy).abs() <= VIEW)
        .map(|player| player.uuid)
        .collect();
    for player in &near {
        if everyone || !mural.viewers.contains(player) {
            for map in 0..MAPS {
                host.send_map(*player, MAP_ID + map as i32, &mural.pixels[map * MAP..(map + 1) * MAP]);
            }
        }
    }
    mural.viewers = near;
}

pub(crate) fn reset() {
    *mural() = None;
}

#[cfg(test)]
pub(crate) fn pixel(map: usize, x: usize, z: usize) -> Option<u8> {
    mural().as_ref().map(|mural| mural.pixels[map * MAP + z * 128 + x])
}
