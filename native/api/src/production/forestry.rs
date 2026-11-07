//! Silvicultora: planta mudas numa grade 3x3 em volta (espaço de 3 blocos entre árvores) e
//! derruba as árvores que crescerem, replantando a mesma espécie.
//!
//! Um tronco é cortado de uma vez com `edit_blocks`, junto das folhas em volta (filtro
//! `#minecraft:leaves`), então o custo por árvore é uma edição.
use super::*;
use horizon_client::mod_api::{NativeBlockEdit, NativeBlockRun};

const FELL_COST: i32 = 200;
const PLANT_COST: i32 = 20;
const MIN_TRUNK: usize = 4;
/// Vagas de plantio: os oito vizinhos de uma grade com passo 3.
pub(crate) const SPOTS: [(i32, i32); 8] = [(-3, -3), (0, -3), (3, -3), (-3, 0), (3, 0), (-3, 3), (0, 3), (3, 3)];

pub(crate) struct Species {
    pub sapling: &'static str,
    pub log: &'static str,
    pub apples: bool,
}

const SPECIES: [Species; 6] = [
    Species { sapling: "minecraft:oak_sapling", log: "minecraft:oak_log", apples: true },
    Species { sapling: "minecraft:birch_sapling", log: "minecraft:birch_log", apples: false },
    Species { sapling: "minecraft:spruce_sapling", log: "minecraft:spruce_log", apples: false },
    Species { sapling: "minecraft:jungle_sapling", log: "minecraft:jungle_log", apples: false },
    Species { sapling: "minecraft:acacia_sapling", log: "minecraft:acacia_log", apples: false },
    Species { sapling: "minecraft:cherry_sapling", log: "minecraft:cherry_log", apples: false },
];

pub(crate) fn by_sapling(item: &str) -> Option<&'static Species> {
    SPECIES.iter().find(|species| species.sapling == item)
}

fn by_log(block: &str) -> Option<&'static Species> {
    SPECIES.iter().find(|species| species.log == block)
}

pub(crate) fn fertile(block: &str) -> bool {
    matches!(block, "minecraft:dirt" | "minecraft:grass_block" | "minecraft:podzol" | "minecraft:coarse_dirt"
        | "minecraft:rooted_dirt" | "minecraft:moss_block" | "minecraft:mud")
}

/// Produtos de uma árvore: troncos, gravetos, a muda extra e, às vezes, maçã.
pub(crate) fn drops(species: &Species, trunk: u32, felled_before: i32) -> Vec<(&'static str, u32)> {
    let mut list = vec![(species.log, trunk), ("minecraft:stick", 2), (species.sapling, 1)];
    if species.apples && felled_before % 4 == 3 {
        list.push(("minecraft:apple", 1));
    }
    list
}

pub(crate) fn tick(host: &mut dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity, time: i64) {
    // Uma vaga a cada 5 ticks: a volta completa leva 2 segundos.
    if (time + i64::from((pos[0] ^ pos[2]) & 7)).rem_euclid(5) != 0 {
        return;
    }
    if number(entity, "energy") < PLANT_COST {
        set_number(entity, "Status", 5);
        return;
    }
    let index = number(entity, "Scan") as usize % SPOTS.len();
    let (dx, dz) = SPOTS[index];
    let base = [pos[0] + dx, pos[1], pos[2] + dz];
    let block = host.block(base);
    let acted = if let Some(species) = by_log(&block) {
        match fell(host, base, entity, species) {
            Ok(acted) => acted,
            Err(status) => {
                set_number(entity, "Status", status);
                return;
            }
        }
    } else if block == "minecraft:air" && fertile(&host.block([base[0], base[1] - 1, base[2]])) {
        plant(host, base, entity)
    } else {
        false
    };
    set_number(entity, "Scan", ((index + 1) % SPOTS.len()) as i32);
    set_number(entity, "Progress", index as i32);
    set_number(entity, "Duration", SPOTS.len() as i32);
    if !acted && entity.items[0].is_none() && number(entity, "Total") == 0 {
        set_number(entity, "Status", NO_SEEDS);
    } else {
        crops::remember_activity(entity, time, acted);
    }
}

fn plant(host: &mut dyn Host, base: [i32; 3], entity: &mut ModBlockEntity) -> bool {
    let Some(species) = entity.items[0].as_ref().and_then(|stack| by_sapling(&stack.item)) else { return false };
    let energy = number(entity, "energy");
    if energy < PLANT_COST {
        return false;
    }
    let edit = NativeBlockEdit {
        palette: vec![species.sapling.to_owned()],
        filters: Vec::new(),
        runs: vec![NativeBlockRun { x: base[0], y: base[1], z: base[2], height: 1, surface: None, block: 0, only: None }],
    };
    if host.edit_blocks(host.dimension(), &edit).is_err() {
        return false;
    }
    consume(&mut entity.items[0], 1);
    set_number(entity, "energy", energy - PLANT_COST);
    true
}

/// Derruba a árvore crescida em `base`. `Err` traz o estado de espera.
fn fell(host: &mut dyn Host, base: [i32; 3], entity: &mut ModBlockEntity, species: &'static Species) -> Result<bool, i32> {
    let [x, y, z] = base;
    let trunk = (0..20).take_while(|dy| host.block([x, y + dy, z]) == species.log).count();
    if trunk < MIN_TRUNK {
        return Ok(false);
    }
    let felled = number(entity, "Total");
    let products = drops(species, trunk as u32, felled);
    if !farm::drops_fit(&entity.items[1..4], &products) {
        return Err(3);
    }
    let energy = number(entity, "energy");
    if energy < FELL_COST {
        return Err(5);
    }
    let top = y + trunk as i32;
    let mut runs = vec![
        // A base vira muda da mesma espécie; o resto do tronco some.
        NativeBlockRun { x, y, z, height: 1, surface: None, block: 1, only: None },
        NativeBlockRun { x, y: y + 1, z, height: trunk as u32 - 1, surface: None, block: 0, only: None },
    ];
    for dx in -2..=2 {
        for dz in -2..=2 {
            runs.push(NativeBlockRun { x: x + dx, y: top - 4, z: z + dz, height: 8, surface: None, block: 0, only: Some(0) });
        }
    }
    let edit = NativeBlockEdit {
        palette: vec!["minecraft:air".to_owned(), species.sapling.to_owned()],
        filters: vec!["#minecraft:leaves".to_owned()],
        runs,
    };
    if host.edit_blocks(host.dimension(), &edit).is_err() {
        return Ok(false);
    }
    for (item, count) in products {
        farm::store(&mut entity.items[1..4], item, count);
    }
    set_number(entity, "energy", energy - FELL_COST);
    set_number(entity, "Produced", number(entity, "Produced").saturating_add(1));
    set_number(entity, "Total", felled.saturating_add(1));
    host.play_sound("minecraft:block.wood.break", [f64::from(x) + 0.5, f64::from(y) + 1.0, f64::from(z) + 0.5], 4, 0.8, 0.9);
    Ok(true)
}
