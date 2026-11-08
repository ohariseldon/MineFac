//! Água encanada, irrigação e hidrelétrica.
//!
//! A água das máquinas fica no NBT `Water`, em mB (mil por balde), como a energia fica em
//! `energy`. A bomba puxa de água parada encostada nela e empurra pelos canos; o cano iguala
//! o nível com os canos vizinhos e entrega tudo o que cabe a quem usa água (aspersor e
//! caldeira). A turbina não guarda água: mede a represa atrás dela (coluna de água, espelho
//! d'água e vazão do rio) e gera energia pela queda.
//!
//! Escolhas de realismo, sem refazer a lavoura vanilla:
//! - cada cultura gasta água e adubo por estágio, nas proporções da demanda real (batata e
//!   beterraba pedem mais água que cenoura; trigo e beterraba esgotam mais o solo);
//! - o aspersor mantém a terra arada úmida em 9x9 sem a poça no meio e, com água e adubo,
//!   adianta estágios; só com água, adianta bem menos; a chuva economiza a água da irrigação;
//! - a turbina gera `queda × espelho d'água × vazão`, como P = ρ·g·Q·h.
use super::*;

pub(crate) const PUMP: &str = "veio:pump";
pub(crate) const PIPE: &str = "veio:pipe";
pub(crate) const SPRINKLER: &str = "veio:sprinkler";
pub(crate) const HYDRO_TURBINE: &str = "veio:hydro_turbine";

/// mB que um cano passa a cada vizinho por tick.
const FLOW_PER_FACE: i32 = 200;
/// mB/t que cada bloco de água parada encostado rende à bomba, até quatro.
pub(crate) const PUMP_PER_SOURCE: i32 = 25;
const PUMP_SOURCES: i32 = 4;
/// FE/t da bomba enquanto bombeia.
pub(crate) const PUMP_POWER: i32 = 6;
/// mB por tick de queima da caldeira alimentada por cano.
pub(crate) const STEAM_WATER: i32 = 5;
const SENSE_EVERY: i32 = 40;

/// Blocos que guardam água no NBT `Water`.
pub(crate) fn holds(block: &str) -> bool {
    matches!(block, PUMP | PIPE | SPRINKLER | STEAM_ENGINE | GEOTHERMAL)
}

/// Lava encanada: a mesma bomba, encostada em lava parada, puxa 5 mB/t por fonte, e os canos a
/// levam até o gerador geotérmico (1.000 mB valem um balde). Cada cano leva um fluido por vez
/// (`Fluid` 1 é lava); vazio, aceita qualquer um.
pub(crate) const LAVA_PER_SOURCE: i32 = 5;
pub(crate) const LAVA_BUCKET: i32 = 1000;

pub(crate) fn is_lava(entity: &ModBlockEntity) -> bool {
    number(entity, "Fluid") == 1
}

/// Valor do painel: mB guardados, negativos quando o fluido é lava.
pub(crate) fn panel_amount(entity: &ModBlockEntity) -> i32 {
    let stored = number(entity, "Water");
    if is_lava(entity) { -stored } else { stored }
}

/// Capacidade do reservatório interno, em mB.
pub(crate) fn capacity(block: &str) -> i32 {
    match block {
        PUMP => 4000,
        PIPE => 1000,
        SPRINKLER => 4000,
        STEAM_ENGINE => 8000,
        GEOTHERMAL => 2000,
        _ => 0,
    }
}

/// Quem recebe de canos e bombas, por fluido. A bomba não recebe: ela só puxa do mundo.
fn receives(block: &str, lava: bool) -> bool {
    if lava { matches!(block, PIPE | GEOTHERMAL) } else { matches!(block, PIPE | SPRINKLER | STEAM_ENGINE) }
}

/// Blocos sem energia, que mostram o nível de água no lugar da carga.
pub(crate) fn without_power(block: &str) -> bool {
    matches!(block, PIPE | SPRINKLER)
}

fn water(entity: &ModBlockEntity) -> i32 {
    number(entity, "Water").min(capacity(&entity.id))
}

/// Empurra água aos vizinhos: entre canos iguala o nível; a quem consome, entrega o que couber.
fn push(host: &mut dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity) {
    let limit = capacity(&entity.id).max(1);
    let mut stored = water(entity);
    if stored == 0 {
        return;
    }
    let lava = is_lava(entity);
    let offset = number(entity, "WaterPort") as usize % 6;
    for i in 0..6 {
        if stored == 0 {
            break;
        }
        let face = Direction::ALL[(i + offset) % 6];
        let delta = face.normal();
        let target = std::array::from_fn(|axis| pos[axis] + delta[axis]);
        host.change_mod_block_entity(target, &mut |other| {
            let theirs = water(other);
            // Fluidos não se misturam: um cano com água não recebe lava, e vice-versa.
            if !receives(&other.id, lava) || (theirs > 0 && is_lava(other) != lava) {
                return;
            }
            let other_limit = capacity(&other.id);
            let offered = share(stored, limit, theirs, other_limit, other.id == PIPE);
            let accepted = offered.min(other_limit - theirs).max(0);
            if accepted > 0 {
                set_number(other, "Water", theirs + accepted);
                set_number(other, "Fluid", i32::from(lava));
                stored -= accepted;
            }
        });
    }
    set_number(entity, "WaterPort", ((offset + 1) % 6) as i32);
    set_number(entity, "Water", stored);
    if stored == 0 {
        set_number(entity, "Fluid", 0);
    }
}

/// Quanto oferecer a um vizinho. Entre canos, a diferença de nível proporcional (a água
/// corre do mais cheio para o mais vazio); a quem consome, tudo até o limite por face.
pub(crate) fn share(mine: i32, limit: i32, theirs: i32, other_limit: i32, pipe: bool) -> i32 {
    if pipe {
        ((i64::from(mine) * i64::from(other_limit) - i64::from(theirs) * i64::from(limit))
            / i64::from((limit + other_limit).max(1))).clamp(0, i64::from(FLOW_PER_FACE)) as i32
    } else {
        mine.min(FLOW_PER_FACE)
    }
}

/// Blocos de água parada encostados (seis faces). A bomba aceita poço (água abaixo) e margem.
fn still_water_around(host: &dyn Host, [x, y, z]: [i32; 3]) -> i32 {
    [[x + 1, y, z], [x - 1, y, z], [x, y, z + 1], [x, y, z - 1], [x, y - 1, z], [x, y + 1, z]].into_iter()
        .filter(|&side| host.block(side) == "minecraft:water"
            && host.block_property(side, "level").is_none_or(|level| level == "0"))
        .count() as i32
}

/// Lava parada encostada (seis faces).
fn still_lava_around(host: &dyn Host, [x, y, z]: [i32; 3]) -> i32 {
    [[x + 1, y, z], [x - 1, y, z], [x, y, z + 1], [x, y, z - 1], [x, y - 1, z], [x, y + 1, z]].into_iter()
        .filter(|&side| host.block(side) == "minecraft:lava"
            && host.block_property(side, "level").is_none_or(|level| level == "0"))
        .count() as i32
}

/// Vazão da bomba em mB/t e o fluido: água quando houver, senão lava, bem mais lenta.
pub(crate) fn pump_source(water_sources: i32, lava_sources: i32) -> (i32, bool) {
    if water_sources > 0 { (pump_rate(water_sources), false) }
    else { (lava_sources.clamp(0, PUMP_SOURCES) * LAVA_PER_SOURCE, lava_sources > 0) }
}

/// Vazão da bomba em mB/t pelo número de fontes encostadas.
pub(crate) fn pump_rate(sources: i32) -> i32 {
    sources.clamp(0, PUMP_SOURCES) * PUMP_PER_SOURCE
}

fn sense_due(entity: &mut ModBlockEntity, pos: [i32; 3]) -> bool {
    let countdown = number(entity, "Sense");
    let due = countdown == 0 || !entity.nbt.contains_key(nbt_key("Info"));
    set_number(entity, "Sense", if due { SENSE_EVERY - 1 + ((pos[0] ^ pos[2]) & 7) } else { countdown - 1 });
    due
}

/// Bomba: com energia, enche o tanque com a água parada encostada e empurra pelos canos.
pub(crate) fn pump(host: &mut dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity) {
    if sense_due(entity, pos) {
        let water_sources = still_water_around(host, pos);
        let lava_sources = if water_sources == 0 { still_lava_around(host, pos) } else { 0 };
        let (rate, lava) = pump_source(water_sources, lava_sources);
        set_number(entity, "Info", rate);
        set_number(entity, "PumpLava", i32::from(lava));
    }
    let rate = number(entity, "Info");
    let limit = capacity(PUMP);
    let mut stored = water(entity);
    let lava = number(entity, "PumpLava") == 1;
    if is_lava(entity) != lava {
        // Trocou de fonte: o resto do tanque se perde, para não misturar.
        stored = 0;
        set_number(entity, "Fluid", i32::from(lava));
    }
    let energy = number(entity, "energy");
    let status = if rate == 0 {
        NO_WATER
    } else if stored >= limit {
        3
    } else if energy < PUMP_POWER {
        5
    } else {
        set_number(entity, "energy", energy - PUMP_POWER);
        set_number(entity, "Water", (stored + rate).min(limit));
        set_number(entity, "Fluid", i32::from(lava));
        set_number(entity, "Total", number(entity, "Total").saturating_add(rate));
        1
    };
    if status != 1 { set_number(entity, "Water", stored); }
    set_number(entity, "Status", status);
    push(host, pos, entity);
}

/// Cano: só conduz. "Trabalhando" enquanto leva água.
pub(crate) fn pipe(host: &mut dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity) {
    super::super::joints::refresh(host, pos, entity, &super::super::joints::PIPE_FACES, super::super::joints::pipe_connects);
    let before = water(entity);
    push(host, pos, entity);
    set_number(entity, "Status", if before == 0 { 0 } else { 1 });
}

/// O que a cultura pede a cada estágio: (água em mB, adubo em pontos, idade madura).
/// Proporções da demanda hídrica e da extração de nutrientes reais, em escala de jogo.
pub(crate) fn needs(block: &str) -> Option<(i32, i32, u8)> {
    Some(match block {
        "minecraft:wheat" => (60, 6, 7),
        "minecraft:carrots" => (45, 4, 7),
        "minecraft:potatoes" => (70, 6, 7),
        "minecraft:beetroots" => (150, 14, 3),
        "minecraft:melon_stem" => (90, 7, 7),
        "minecraft:pumpkin_stem" => (80, 7, 7),
        "minecraft:torchflower_crop" => (60, 5, 1),
        _ => return None,
    })
}

/// Pontos de adubo de cada item do slot.
pub(crate) fn fertilizer(item: &str) -> Option<i32> {
    Some(match item {
        BONE_MEAL => 40,
        BIOMASS => 20,
        _ => return None,
    })
}

/// Chance, em um a cada N passadas (4 s cada), de a cultura avançar um estágio.
pub(crate) const FED_ONE_IN: u32 = 10;
pub(crate) const WATERED_ONE_IN: u32 = 30;
const MAX_FERTILITY: i32 = 400;
/// mB de evaporação por terra arada visitada; metade na chuva.
pub(crate) const SOIL_WATER: i32 = 2;
const SCANS_PER_TICK: i32 = 3;

/// Sorteio estável por posição e tempo, sem estado de gerador aleatório.
pub(crate) fn roll([x, y, z]: [i32; 3], time: i64, one_in: u32) -> bool {
    let mut h = (x as i64 as u64).wrapping_mul(0x9e37_79b9_7f4a_7c15)
        ^ (z as i64 as u64).wrapping_mul(0xc2b2_ae3d_27d4_eb4f)
        ^ (y as i64 as u64).wrapping_mul(0x1656_67b1_9e37_79f9)
        ^ (time as u64).wrapping_mul(0x27d4_eb2f_1656_67c5);
    h ^= h >> 29;
    h = h.wrapping_mul(0xbf58_476d_1ce4_e5b9);
    h ^= h >> 32;
    h % u64::from(one_in.max(1)) == 0
}

/// Aspersor: irriga a terra arada de 9x9 em três camadas e, com água e adubo, faz as culturas
/// crescerem. Sem energia: a pressão vem da bomba.
pub(crate) fn sprinkle(host: &mut dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity, time: i64) {
    let mut fertility = number(entity, "Fertility").min(MAX_FERTILITY);
    if fertility <= MAX_FERTILITY / 2
        && let Some(points) = entity.items[0].as_ref().and_then(|stack| fertilizer(&stack.item))
    {
        consume(&mut entity.items[0], 1);
        fertility += points;
    }
    let mut stored = water(entity);
    if stored < SOIL_WATER {
        set_number(entity, "Fertility", fertility);
        set_number(entity, "Info", fertility);
        set_number(entity, "Status", NO_WATER);
        return;
    }
    if !budget::request(budget::Lane::Terrain, host.dimension(), pos, 12) {
        return;
    }
    let raining = host.weather().0 > 0.5 && host.light([pos[0], pos[1] + 1, pos[2]]).1 == 15;
    let soil_cost = if raining { SOIL_WATER / 2 } else { SOIL_WATER };
    let mut cursor = number(entity, "Scan");
    let mut acted = false;
    for _ in 0..SCANS_PER_TICK {
        let ground = farm::scan_position(pos, cursor);
        cursor = (cursor + 1) % farm::AREA;
        if host.block(ground) != "minecraft:farmland" || stored < soil_cost {
            continue;
        }
        stored -= soil_cost;
        host.set_block_property(ground, "moisture", "7");
        let crop = [ground[0], ground[1] + 1, ground[2]];
        let block = host.block(crop);
        let Some((thirst, hunger, ripe)) = needs(&block) else { continue };
        let Some(age) = host.block_property(crop, "age").and_then(|age| age.parse::<u8>().ok()) else { continue };
        if age >= ripe || stored < thirst {
            continue;
        }
        let fed = fertility >= hunger;
        if !roll(crop, time, if fed { FED_ONE_IN } else { WATERED_ONE_IN }) {
            continue;
        }
        if host.set_block_property(crop, "age", &(age + 1).to_string()) {
            stored -= thirst;
            if fed {
                fertility -= hunger;
            }
            set_number(entity, "Total", number(entity, "Total").saturating_add(1));
            acted = true;
        }
    }
    set_number(entity, "Water", stored);
    set_number(entity, "Fertility", fertility);
    set_number(entity, "Info", fertility);
    set_number(entity, "Scan", cursor);
    set_number(entity, "Progress", cursor);
    set_number(entity, "Duration", farm::AREA);
    crops::remember_activity(entity, time, acted);
    if number(entity, "Status") == 10 {
        // Sem cultura para adiantar, mas irrigando: segue "trabalhando".
        set_number(entity, "Status", 1);
    }
}

/// Potência da turbina em FE/t: `queda × espelho × vazão`. Abaixo de 3 blocos de queda ou de
/// 12 blocos de espelho d'água não gira. Até 40 blocos de espelho contam; a vazão do rio (em
/// nascentes somadas) multiplica até 4 vezes.
pub(crate) fn hydro_power(head: i32, surface: i32, flow: i32) -> i32 {
    if head < 3 || surface < 12 {
        return 0;
    }
    let area = surface.min(40) as f32 / 40.0;
    let discharge = 1.0 + flow.clamp(0, 12) as f32 / 4.0;
    ((6.0 * head.min(32) as f32 * area * discharge).round() as i32).min(HYDRO_MAX)
}
pub(crate) const HYDRO_MAX: i32 = 320;

fn facing_normal(facing: &str) -> [i32; 3] {
    match facing {
        "north" => [0, 0, -1],
        "east" => [1, 0, 0],
        "west" => [-1, 0, 0],
        _ => [0, 0, 1],
    }
}

fn still(host: &dyn Host, pos: [i32; 3]) -> bool {
    host.block(pos) == "minecraft:water" && host.block_property(pos, "level").is_none_or(|level| level == "0")
}

/// Altura da coluna de água parada a partir de `pos`, para cima.
fn column(host: &dyn Host, [x, y, z]: [i32; 3]) -> i32 {
    (0..32).take_while(|&dy| still(host, [x, y + dy, z])).count() as i32
}

/// Mede a represa: a entrada é o bocal (atrás), a saída é a frente. Devolve (queda, espelho).
pub(crate) fn measure_dam(host: &dyn Host, pos: [i32; 3]) -> (i32, i32) {
    let facing = host.block_property(pos, "facing").unwrap_or_else(|| "south".to_owned());
    let [nx, _, nz] = facing_normal(&facing);
    let intake = [pos[0] - nx, pos[1], pos[2] - nz];
    let outlet = [pos[0] + nx, pos[1], pos[2] + nz];
    let upstream = column(host, intake);
    if upstream == 0 {
        return (0, 0);
    }
    let downstream = column(host, outlet);
    let head = upstream - downstream;
    let top = intake[1] + upstream - 1;
    let mut surface = 0;
    for dx in -3..=3 {
        for dz in -3..=3 {
            let at = [intake[0] + dx - nx * 3, top, intake[2] + dz - nz * 3];
            if still(host, at) && host.block([at[0], top + 1, at[2]]) != "minecraft:water" {
                surface += 1;
            }
        }
    }
    (head, surface)
}

/// Geração da turbina, relida a cada 2 segundos. Publica a potência em `Info`.
pub(crate) fn turbine(host: &mut dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity, energy: i32, limit: i32) -> i32 {
    if sense_due(entity, pos) {
        let (head, surface) = measure_dam(host, pos);
        let flow = super::super::rivers::flow_near(host.dimension(), pos, 10);
        set_number(entity, "Head", head.max(0));
        set_number(entity, "Info", hydro_power(head, surface, flow));
    }
    (energy + number(entity, "Info")).min(limit)
}

#[cfg(test)]
#[path = "water_tests.rs"]
mod tests;
