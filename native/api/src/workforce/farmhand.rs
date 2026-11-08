//! Posto agrícola: o trabalhador cuida da lavoura e leva a colheita ao posto.
//!
//! - **Sem zona:** colhe as plantas maduras num 9x9 em volta do posto (como a colheitadeira),
//!   deixando a planta replantada.
//! - **Com zona de lavoura** (`zones`, marco a até 3 blocos da borda): trabalha na zona inteira e,
//!   além de colher, **planta** sementes da saída do posto em solo vazio (terra arada; areia das
//!   almas para fungo do Nether) e **ara** terra e grama com ar em cima e água a até 4 blocos. Se o
//!   marco tiver uma semente no slot, só ela é plantada: cada lavoura pode ter sua cultura.
//!
//! Plantar e arar não gastam comida (só colheitas gastam) e contam como trabalho na zona.
use super::*;
use horizon_client::mod_api::{NativeBlockEdit, NativeBlockRun};
use zones::{Kind, Zone};

/// Posições examinadas por ciclo: uma fileira de 9 (a volta de um 9x9 leva 27 ciclos; um 21x21, 147).
pub(super) const SCANS: i32 = 9;
const ABSORB_PER_HARVEST: f32 = 3.0;
const HARVEST: i32 = 0;
const PLANT: i32 = 1;
const TILL: i32 = 2;
/// Hidratação vanilla: água a até 4 blocos na horizontal.
const WATER_REACH: i32 = 4;

fn ripe(host: &dyn Host, spot: [i32; 3]) -> Option<farm::Crop> {
    let crop = farm::crop(&host.block(spot))?;
    let age = host.block_property(spot, "age")?.parse::<u8>().ok()?;
    (age >= crop.ripe).then_some(crop)
}

/// Slot de saída com uma semente que cresce em `soil`, respeitando o filtro da zona.
fn seed_slot(entity: &ModBlockEntity, soil: &str, filter: Option<&str>) -> Option<(usize, i32)> {
    outputs(&entity.id).find_map(|slot| {
        let stack = entity.items[slot].as_ref().filter(|stack| plain(stack))?;
        if filter.is_some_and(|wanted| wanted != stack.item.as_ref()) {
            return None;
        }
        let (block, needs) = production::crops::planted(&stack.item)?;
        (needs == soil).then_some((slot, block))
    })
}

/// Ar sobre solo de plantio vazio: devolve o solo.
fn plantable(host: &dyn Host, spot: [i32; 3]) -> Option<String> {
    let soil = host.block([spot[0], spot[1] - 1, spot[2]]);
    matches!(soil.as_str(), "minecraft:farmland" | "minecraft:soul_sand").then_some(soil)
}

fn tillable(host: &dyn Host, spot: [i32; 3]) -> bool {
    host.block([spot[0], spot[1] + 1, spot[2]]) == "minecraft:air" && watered(host, spot)
}

fn watered(host: &dyn Host, [x, y, z]: [i32; 3]) -> bool {
    (1..=WATER_REACH).any(|d| [(d, 0), (-d, 0), (0, d), (0, -d), (d, d), (-d, -d), (d, -d), (-d, d)].iter()
        .any(|(dx, dz)| host.block([x + dx, y, z + dz]) == "minecraft:water"))
}

fn zone_of(host: &dyn Host, pos: [i32; 3], now: u64) -> Option<Zone> {
    zones::zone_for(host.dimension(), pos, Kind::Field, now)
}

/// Próxima tarefa. `Err` traz o estado de espera do painel; `KEEP` mantém o estado atual enquanto
/// a primeira volta da busca não termina.
pub(super) fn plan(host: &mut dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity, now: u64) -> Result<Plan, i32> {
    let room = outputs(&entity.id);
    let dimension = host.dimension();
    let zone = zone_of(host, pos, now);
    // Uma fileira de `SCANS` mesmo na zona grande: a área não muda, só a volta demora mais.
    let (center, radius, row) = zone.as_ref().map_or((pos, farm::RADIUS, SCANS), |zone| (zone.pos, zone.side / 2, SCANS));
    let area = (radius * 2 + 1).pow(2) * 3;
    // Zona nova ou de outro tamanho: recomeça a varredura.
    let shape = vec![center[0], center[1], center[2], radius];
    if !matches!(entity.nbt.get(nbt_key("ScanArea")), Some(Tag::IntArray(old)) if *old == shape) {
        entity.nbt.insert("ScanArea".into(), Tag::IntArray(shape));
        set_number(entity, "Scan", 0);
        set_number(entity, "Searched", 0);
    }
    set_pos(entity, "ZoneAt", zone.as_ref().map(|zone| zone.pos));
    let filter = zone.as_ref().and_then(|zone| zone.filter.clone());
    let visit = |spot| Plan::Visit { spot, reach: REACH };
    if let Some(spot) = target(entity) {
        let still = match number(entity, "Action") {
            HARVEST => ripe(host, spot).map(|crop| farm::drops_fit(&entity.items[room.clone()], crop.drops)),
            PLANT => plantable(host, spot).map(|soil| seed_slot(entity, &soil, filter.as_deref()).is_some()),
            _ => tillable(host, spot).then_some(true),
        };
        match still {
            Some(true) => return Ok(visit(spot)),
            Some(false) => return Err(3),
            None => set_target(entity, None),
        }
    }
    let avoid = skipped(entity);
    let mut cursor = number(entity, "Scan") % area;
    let mut plant = None;
    let mut till = None;
    let mut probed_soil = false;
    for _ in 0..row {
        let spot = area_position(center, radius, cursor);
        cursor = (cursor + 1) % area;
        if cursor == 0 {
            clear_skipped(entity);
        }
        if avoid.contains(&spot) || reserved_by_other(dimension, spot, pos, now) {
            continue;
        }
        let block = host.block(spot);
        if let Some(crop) = farm::crop(&block)
            && host.block_property(spot, "age").and_then(|age| age.parse::<u8>().ok()).is_some_and(|age| age >= crop.ripe)
        {
            if !farm::drops_fit(&entity.items[room.clone()], crop.drops) {
                set_number(entity, "Scan", (cursor - 1).rem_euclid(area));
                return Err(3);
            }
            set_number(entity, "Scan", cursor);
            set_number(entity, "Searched", 0);
            set_number(entity, "Action", HARVEST);
            set_target(entity, Some(spot));
            return Ok(visit(spot));
        }
        // Plantar e arar só dentro de uma zona de lavoura.
        if zone.is_none() {
            continue;
        }
        if plant.is_none() && block == "minecraft:air"
            && plantable(host, spot).is_some_and(|soil| seed_slot(entity, &soil, filter.as_deref()).is_some())
        {
            plant = Some(spot);
        } else if till.is_none() && !probed_soil
            && matches!(block.as_str(), "minecraft:grass_block" | "minecraft:dirt")
        {
            // `watered` pode ler 32 blocos; um probe por ciclo, o resto da fileira espera a próxima.
            probed_soil = true;
            if tillable(host, spot) {
                till = Some(spot);
            }
        }
    }
    set_number(entity, "Scan", cursor);
    if let Some((spot, action)) = plant.map(|spot| (spot, PLANT)).or(till.map(|spot| (spot, TILL))) {
        set_number(entity, "Searched", 0);
        set_number(entity, "Action", action);
        set_target(entity, Some(spot));
        return Ok(visit(spot));
    }
    let searched = (number(entity, "Searched") + row).min(area);
    set_number(entity, "Searched", searched);
    Err(if searched >= area { NO_TASK } else { KEEP })
}

/// Executa a tarefa ao lado do alvo. Colheita: devolve o que o trabalhador passa a carregar.
/// Plantar e arar: mudam o mundo e devolvem `None` (não há carga nem gasto de comida).
pub(super) fn perform(host: &mut dyn Host, spot: [i32; 3], entity: &mut ModBlockEntity) -> Option<Vec<(String, u32)>> {
    let zone = get_pos(entity, "ZoneAt");
    let action = number(entity, "Action");
    set_number(entity, "Action", HARVEST);
    let dimension = host.dimension();
    let place = |host: &mut dyn Host, block: &str| host.edit_blocks(dimension, &NativeBlockEdit {
        palette: vec![block.to_owned()],
        filters: Vec::new(),
        runs: vec![NativeBlockRun { x: spot[0], y: spot[1], z: spot[2], height: 1, surface: None, block: 0, only: None }],
    }).is_ok();
    let done = match action {
        PLANT => {
            let soil = plantable(host, spot)?;
            if host.block(spot) != "minecraft:air" {
                return None;
            }
            let filter = zone.and_then(|marker| zones::zones_near(dimension, marker, 0, defense::clock()).into_iter()
                .next().and_then(|zone| zone.filter));
            let (slot, block) = seed_slot(entity, &soil, filter.as_deref())?;
            if !place(host, production::crops::PALETTE[block as usize]) {
                return None;
            }
            consume(&mut entity.items[slot], 1);
            host.play_sound("minecraft:item.crop.plant", center(spot), 4, 0.6, 1.0);
            None
        }
        TILL => {
            if !matches!(host.block(spot).as_str(), "minecraft:grass_block" | "minecraft:dirt") || !tillable(host, spot)
                || !place(host, "minecraft:farmland")
            {
                return None;
            }
            host.play_sound("minecraft:item.hoe.till", center(spot), 4, 0.6, 1.0);
            None
        }
        _ => {
            let crop = ripe(host, spot)?;
            if !farm::drops_fit(&entity.items[outputs(&entity.id)], crop.drops)
                || !host.set_block_property(spot, "age", &crop.reset.to_string())
            {
                return None;
            }
            pollution::absorb(dimension, spot, ABSORB_PER_HARVEST);
            let at = center(spot);
            host.send_particles(&ParticleOptions::simple(ParticleType::Composter), None, at, [0.3, 0.2, 0.3], 0.0, 6);
            host.play_sound("minecraft:block.crop.break", at, 4, 0.6, 1.0);
            Some(crop.drops.iter().map(|&(item, count)| (item.to_owned(), count)).collect())
        }
    };
    if let Some(marker) = zone {
        zones::credit(host, marker);
    }
    done
}
