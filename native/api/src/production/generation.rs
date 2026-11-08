//! Geração de energia: gerador a combustível, geotérmico, solar, biomassa, vapor, roda d'água e
//! cata-vento. A distribuição (cabos, postes) fica em `lib.rs`/`power.rs`, com a logística.
//!
//! Escada de energia, do começo ao fim:
//! - roda d'água (8 FE/t por face com água corrente, até 32) e cata-vento (8 FE/t no nível do mar,
//!   sobe com a altitude e com chuva, até 32): grátis, limpos e presos ao terreno;
//! - gerador a combustível e a biomassa (40 FE/t; a biomassa polui menos e vem da lavoura);
//! - caldeira a vapor (120 FE/t, o dobro de energia por combustível, precisa de água ao lado);
//! - geotérmico (80 FE/t com lava) e painéis solares, que escalam em quantidade.
use super::*;
use horizon_client::mod_api::{BlockEntitySubmitEvent, CubeFace};

/// FE por tick e FE por tick de queima vanilla de cada queimador.
pub(crate) fn burner(block: &str) -> Option<(i32, i32)> {
    Some(match block {
        GENERATOR => (40, 10),
        BIOMASS_GENERATOR => (40, 10),
        STEAM_ENGINE => (120, 20),
        _ => return None,
    })
}

/// Valor do combustível em ticks de fornalha vanilla, pela tabela do queimador.
pub(crate) fn fuel_ticks(block: &str, stack: &ItemStack) -> Option<i32> {
    if stack.damage != 0 || stack.extra.is_some() {
        return None;
    }
    let table = if block == BIOMASS_GENERATOR { data::FUELS_BIOMASS_GENERATOR } else { data::FUELS_GENERATOR };
    table.iter().find(|fuel| fuel.items.contains(&stack.item.as_ref())).map(|fuel| fuel.ticks)
}

const SENSE_EVERY: i32 = 40;
/// FE/t por face lateral com água corrente.
pub(crate) const WATER_FACE: i32 = 8;
/// Piso do cata-vento com céu aberto, no nível do mar.
pub(crate) const WIND_FLOOR: i32 = 8;

/// Água corrente encostada nas faces laterais (roda d'água) ou água parada (caldeira).
fn water_sides(host: &dyn Host, [x, y, z]: [i32; 3], flowing: bool) -> i32 {
    [[x + 1, y, z], [x - 1, y, z], [x, y, z + 1], [x, y, z - 1]].into_iter()
        .filter(|&side| {
            host.block(side) == "minecraft:water"
                && host.block_property(side, "level").is_some_and(|level| (level != "0") == flowing)
        })
        .count() as i32
}

/// FE por tick do cata-vento: cresce com a altitude acima do mar; chuva e tempestade ajudam.
pub(crate) fn wind_output(y: i32, open_sky: bool, rain: f32, thunder: f32, gust: f32) -> i32 {
    if !open_sky {
        return 0;
    }
    let altitude = ((y - 62) as f32 / 3.0).clamp(0.0, 24.0);
    let base = altitude.max(WIND_FLOOR as f32);
    let weather = 1.0 + 0.25 * rain + 0.35 * thunder;
    ((base * weather * gust).round() as i32).min(32)
}

/// Produz energia neste tick e devolve o novo estoque. Não depende do relógio do jogo: os
/// sensores (água, vento) são relidos por uma contagem própria, `Sense`, a cada 40 ticks.
fn power_id(id: &str) -> &'static str {
    match id {
        GENERATOR => GENERATOR,
        BIOMASS_GENERATOR => BIOMASS_GENERATOR,
        STEAM_ENGINE => STEAM_ENGINE,
        GEOTHERMAL => GEOTHERMAL,
        SOLAR => SOLAR,
        WATER_WHEEL => WATER_WHEEL,
        WIND_TURBINE => WIND_TURBINE,
        HYDRO_TURBINE => HYDRO_TURBINE,
        _ => "",
    }
}

fn solar_output(host: &dyn Host, [x, y, z]: [i32; 3]) -> i32 {
    let daylight = host.day_time().rem_euclid(24000);
    let open_sky = host.light([x, y + 1, z]).1 == 15;
    if host.dimension() != "minecraft:overworld" || !open_sky || daylight >= 12000 {
        return 0;
    }
    let (rain, thunder) = host.weather();
    let sun = (daylight as f32 * std::f32::consts::PI / 12000.0).sin().max(0.0);
    (16.0 * sun * (1.0 - 0.6 * rain - 0.4 * thunder).max(0.0)) as i32
}

pub(crate) fn produce(host: &mut dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity, energy: i32, limit: i32) -> i32 {
    let mut energy = energy;
    let id = power_id(&entity.id);
    let senses = matches!(id, STEAM_ENGINE | WATER_WHEEL | WIND_TURBINE | SOLAR);
    let sense = if senses {
        let countdown = number(entity, "Sense");
        let due = countdown == 0 || !entity.nbt.contains_key(nbt_key("Info"));
        set_number(entity, "Sense", if due { SENSE_EVERY - 1 + ((pos[0] ^ pos[2]) & 7) } else { countdown - 1 });
        due
    } else {
        false
    };
    match id {
        GENERATOR | BIOMASS_GENERATOR | STEAM_ENGINE => {
            let (rate, per_tick) = burner(id).expect("queimador");
            if id == STEAM_ENGINE && sense {
                set_number(entity, "Info", water_sides(host, pos, false));
            }
            // A caldeira usa a água parada encostada ou, sem ela, a que chega pelo cano.
            let piped = id == STEAM_ENGINE && number(entity, "Info") == 0;
            let has_water = id != STEAM_ENGINE || !piped || number(entity, "Water") >= water::STEAM_WATER;
            if energy > limit - rate || !has_water {
                return energy;
            }
            let mut burn = number(entity, "Burn");
            if burn == 0
                && let Some(ticks) = entity.items[0].as_ref().and_then(|stack| fuel_ticks(id, stack))
            {
                consume(&mut entity.items[0], 1);
                burn = (ticks * per_tick / rate).max(1);
                set_number(entity, "BurnTotal", burn);
            }
            if burn > 0 {
                energy += rate;
                set_number(entity, "Burn", burn - 1);
                if piped {
                    set_number(entity, "Water", number(entity, "Water") - water::STEAM_WATER);
                }
            }
        }
        HYDRO_TURBINE => {
            energy = water::turbine(host, pos, entity, energy, limit);
        }
        GEOTHERMAL if energy <= limit - 80 => {
            let mut burn = number(entity, "Burn");
            let bucket_room = entity.items[1].as_ref().is_none_or(|stack|
                is_plain(stack, "minecraft:bucket") && stack.count < 16);
            if burn == 0 && bucket_room
                && entity.items[0].as_ref().is_some_and(|s| is_plain(s, "minecraft:lava_bucket"))
            {
                consume(&mut entity.items[0], 1);
                add(&mut entity.items[1], "minecraft:bucket", 1);
                burn = 800;
                set_number(entity, "BurnTotal", burn);
            }
            if burn > 0 {
                energy += 80;
                set_number(entity, "Burn", burn - 1);
            }
        }
        SOLAR => {
            if sense {
                set_number(entity, "Info", solar_output(host, pos));
            }
            energy = (energy + number(entity, "Info")).min(limit);
        }
        WATER_WHEEL => {
            if sense {
                set_number(entity, "Info", WATER_FACE * water_sides(host, pos, true));
            }
            energy = (energy + number(entity, "Info")).min(limit);
        }
        WIND_TURBINE => {
            let time = i64::from(number(entity, "Gust").wrapping_add(1));
            set_number(entity, "Gust", time as i32 % 100_000);
            if sense {
                let [x, y, z] = pos;
                let open_sky = host.light([x, y + 2, z]).1 == 15 && host.dimension() == "minecraft:overworld";
                let (rain, thunder) = host.weather();
                // Rajadas lentas e diferentes em cada lugar.
                let gust = 0.85 + 0.3 * ((time as f32 / 900.0) + (x ^ z) as f32).sin().abs();
                set_number(entity, "Info", wind_output(y, open_sky, rain, thunder, gust));
            }
            energy = (energy + number(entity, "Info")).min(limit);
        }
        _ => {}
    }
    energy
}

/// Estado de uma fonte sem nenhuma energia guardada.
pub(crate) fn empty_status(entity: &ModBlockEntity) -> i32 {
    match entity.id.as_str() {
        STEAM_ENGINE if number(entity, "Info") == 0 && number(entity, "Water") < water::STEAM_WATER => NO_WATER,
        HYDRO_TURBINE => NO_WATER,
        GENERATOR | GEOTHERMAL | BIOMASS_GENERATOR | STEAM_ENGINE if number(entity, "Burn") == 0 => 6,
        SOLAR => 7,
        WATER_WHEEL => NO_WATER,
        WIND_TURBINE => NO_WIND,
        _ => 5,
    }
}

const PLANKS: [Option<CubeFace<'static>>; 6] = [Some(CubeFace { sprite: "minecraft:block/oak_planks", uv: None }); 6];
const SAIL: [Option<CubeFace<'static>>; 6] = [Some(CubeFace { sprite: "minecraft:block/white_wool", uv: None }); 6];

/// Eixo horizontal das peças giratórias: a roda gira de lado para a frente do bloco.
fn sideways(facing: &str) -> [f32; 3] {
    if matches!(facing, "east" | "west") { [0.0, 0.0, 1.0] } else { [1.0, 0.0, 0.0] }
}

pub(crate) fn draw(host: &dyn Host, event: &mut BlockEntitySubmitEvent<'_>) -> bool {
    // Só a roda e o cata-vento: as outras máquinas passam por aqui a cada quadro.
    if !matches!(event.block, WATER_WHEEL | WIND_TURBINE) { return false; }
    let facing = host.block_state_property(event.state, "facing").unwrap_or("south");
    let working = number(event.entity, "Info") > 0;
    match event.block {
        WATER_WHEEL => {
            let axis = sideways(facing);
            event.draw.spin(axis, [0.5, 0.5, 0.5], if working { 4.0 } else { 0.0 }, 0.0);
            for paddle in 0..8 {
                event.draw.push();
                event.draw.rotate_about(axis, paddle as f32 * 45.0, [0.5, 0.5, 0.5]);
                let (from, to) = if axis[0] == 1.0 {
                    ([3.0, 7.5, 8.0], [13.0, 8.5, 15.5])
                } else {
                    ([8.0, 7.5, 3.0], [15.5, 8.5, 13.0])
                };
                host.draw_cube(event.draw, from, to, PLANKS, 0xffffff);
                event.draw.pop();
            }
            event.draw.still();
        }
        WIND_TURBINE => {
            // O cubo das pás fica na frente da nacele, a 1,75 bloco de altura.
            let (axis, pivot) = match facing {
                "north" => ([0.0, 0.0, 1.0], [0.5, 1.75, 0.3125]),
                "east" => ([1.0, 0.0, 0.0], [0.6875, 1.75, 0.5]),
                "west" => ([1.0, 0.0, 0.0], [0.3125, 1.75, 0.5]),
                _ => ([0.0, 0.0, 1.0], [0.5, 1.75, 0.6875]),
            };
            let speed = (number(event.entity, "Info") as f32 * 0.6).min(18.0);
            event.draw.spin(axis, pivot, speed, 0.0);
            let center = pivot.map(|value| value * 16.0);
            for blade in 0..3 {
                event.draw.push();
                event.draw.rotate_about(axis, blade as f32 * 120.0, pivot);
                let (from, to) = if axis[2] == 1.0 {
                    ([center[0] - 1.0, center[1], center[2] - 0.25], [center[0] + 1.0, center[1] + 13.0, center[2] + 0.25])
                } else {
                    ([center[0] - 0.25, center[1], center[2] - 1.0], [center[0] + 0.25, center[1] + 13.0, center[2] + 1.0])
                };
                host.draw_cube(event.draw, from, to, SAIL, 0xffffff);
                event.draw.pop();
            }
            event.draw.still();
        }
        _ => return false,
    }
    event.draw.keep();
    true
}

/// Área desenhada além do bloco: pás do cata-vento.
pub(crate) fn bounds(block: &str, bounds: &mut [f32; 6]) {
    if block == WIND_TURBINE {
        bounds[0] = bounds[0].min(-1.0);
        bounds[2] = bounds[2].min(-1.0);
        bounds[3] = bounds[3].max(2.0);
        bounds[4] = bounds[4].max(3.0);
        bounds[5] = bounds[5].max(2.0);
    }
}
