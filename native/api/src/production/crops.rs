//! Semeadora, lâmpada de cultivo e plantas altas (cana, cacto, bambu, abóbora, melancia).
//!
//! Plantar e remover blocos usa `Host::edit_blocks`, que reenvia o chunk inteiro aos jogadores.
//! Por isso as mudanças ficam numa fila no NBT (`Edits`) e saem juntas, no máximo uma vez por
//! segundo por máquina.
use super::*;
use horizon_client::mod_api::{NativeBlockEdit, NativeBlockRun};

/// Blocos que a produção coloca. A fila guarda o índice nesta lista.
pub(crate) const PALETTE: [&str; 9] = [
    "minecraft:air", "minecraft:wheat", "minecraft:carrots", "minecraft:potatoes", "minecraft:beetroots",
    "minecraft:melon_stem", "minecraft:pumpkin_stem", "minecraft:nether_wart", "minecraft:torchflower_crop",
];
pub(crate) const AIR: i32 = 0;
const FLUSH_EVERY: i64 = 20;
const MAX_QUEUE: usize = 16;
const PLANT_COST: i32 = 20;
const GROW_COST: i32 = 50;

/// Semente → (índice do bloco plantado, solo exigido).
pub(crate) fn planted(item: &str) -> Option<(i32, &'static str)> {
    Some(match item {
        "minecraft:wheat_seeds" => (1, "minecraft:farmland"),
        "minecraft:carrot" => (2, "minecraft:farmland"),
        "minecraft:potato" => (3, "minecraft:farmland"),
        "minecraft:beetroot_seeds" => (4, "minecraft:farmland"),
        "minecraft:melon_seeds" => (5, "minecraft:farmland"),
        "minecraft:pumpkin_seeds" => (6, "minecraft:farmland"),
        "minecraft:nether_wart" => (7, "minecraft:soul_sand"),
        "minecraft:torchflower_seeds" => (8, "minecraft:farmland"),
        _ => return None,
    })
}

/// Fila de edições pendentes: x, y, z, altura, índice na paleta.
fn queued(entity: &ModBlockEntity) -> Vec<[i32; 5]> {
    match entity.nbt.get(&"Edits".into()) {
        Some(Tag::IntArray(values)) => values.chunks_exact(5).map(|v| [v[0], v[1], v[2], v[3], v[4]]).collect(),
        _ => Vec::new(),
    }
}

fn store_queue(entity: &mut ModBlockEntity, list: &[[i32; 5]]) {
    entity.nbt.insert("Edits".into(), Tag::IntArray(list.iter().flatten().copied().collect()));
}

pub(crate) fn is_queued(entity: &ModBlockEntity, pos: [i32; 3]) -> bool {
    queued(entity).iter().any(|edit| edit[..3] == pos)
}

pub(crate) fn queue_full(entity: &ModBlockEntity) -> bool {
    queued(entity).len() >= MAX_QUEUE
}

pub(crate) fn enqueue(entity: &mut ModBlockEntity, [x, y, z]: [i32; 3], height: i32, block: i32) {
    let mut list = queued(entity);
    list.push([x, y, z, height, block]);
    store_queue(entity, &list);
}

/// Envia a fila de uma vez. Sem nada na fila, não toca no mundo.
pub(crate) fn flush(host: &mut dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity, time: i64) {
    let list = queued(entity);
    if list.is_empty() {
        return;
    }
    let due = (time + i64::from((pos[0] ^ pos[2]) & 15)).rem_euclid(FLUSH_EVERY) == 0;
    if !due && list.len() < MAX_QUEUE {
        return;
    }
    let edit = NativeBlockEdit {
        palette: PALETTE.iter().map(|name| (*name).to_owned()).collect(),
        filters: Vec::new(),
        runs: list.iter().map(|&[x, y, z, height, block]| NativeBlockRun {
            x, z, y, height: height.max(1) as u32, surface: None, block: block as u32, only: None,
        }).collect(),
    };
    let dimension = host.dimension();
    // Se a edição for recusada, a fila é descartada: a varredura encontra as plantas de novo.
    let _ = host.edit_blocks(dimension, &edit);
    store_queue(entity, &[]);
}

/// Semeadora: planta a semente do slot em solo vazio na área 9x9 em volta (três camadas).
pub(crate) fn plant(host: &mut dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity, time: i64) {
    flush(host, pos, entity, time);
    let Some((block, soil)) = entity.items[0].as_ref().and_then(|stack| planted(&stack.item)) else {
        set_number(entity, "Status", NO_SEEDS);
        return;
    };
    if number(entity, "energy") < PLANT_COST {
        set_number(entity, "Status", 5);
        return;
    }
    let mut cursor = number(entity, "Scan");
    let mut planted_now = false;
    for _ in 0..3 {
        if queue_full(entity) || entity.items[0].is_none() {
            break;
        }
        let ground = farm::scan_position(pos, cursor);
        let spot = [ground[0], ground[1] + 1, ground[2]];
        cursor = (cursor + 1) % farm::AREA;
        if is_queued(entity, spot) || host.block(ground) != soil || host.block(spot) != "minecraft:air" {
            continue;
        }
        let energy = number(entity, "energy");
        if energy < PLANT_COST {
            set_number(entity, "Scan", cursor);
            set_number(entity, "Status", 5);
            return;
        }
        set_number(entity, "energy", energy - PLANT_COST);
        consume(&mut entity.items[0], 1);
        enqueue(entity, spot, 1, block);
        set_number(entity, "Total", number(entity, "Total").saturating_add(1));
        planted_now = true;
    }
    set_number(entity, "Scan", cursor);
    set_number(entity, "Progress", cursor);
    set_number(entity, "Duration", farm::AREA);
    remember_activity(entity, time, planted_now);
}

/// Mantém o estado "trabalhando" por 5 segundos depois da última ação.
pub(crate) fn remember_activity(entity: &mut ModBlockEntity, time: i64, acted: bool) {
    let now = (time % i64::from(i32::MAX)) as i32;
    if acted {
        set_number(entity, "BusyAt", now);
    }
    let busy = entity.nbt.contains_key(&"BusyAt".into()) && now.wrapping_sub(number(entity, "BusyAt")) < 100;
    set_number(entity, "Status", if busy { 1 } else { 10 });
}

/// Plantas de caule com idade que a lâmpada também acelera.
fn stem_ripe(block: &str) -> Option<u8> {
    matches!(block, "minecraft:melon_stem" | "minecraft:pumpkin_stem" | "minecraft:torchflower_crop").then_some(
        if block == "minecraft:torchflower_crop" { 1 } else { 7 })
}

/// Lâmpada de cultivo: com energia, adianta um estágio de uma planta da área a cada 4 ticks.
pub(crate) fn grow(host: &mut dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity, time: i64) {
    let cooldown = number(entity, "Cooldown");
    if cooldown > 0 {
        set_number(entity, "Cooldown", cooldown - 1);
        return;
    }
    if number(entity, "energy") < GROW_COST {
        set_number(entity, "Status", 5);
        return;
    }
    let mut cursor = number(entity, "Scan");
    let mut acted = false;
    let mut starved = false;
    for _ in 0..4 {
        // A lâmpada fica acima da lavoura: olha da própria altura até três blocos abaixo.
        let mut target = farm::scan_position(pos, cursor);
        target[1] -= 1;
        cursor = (cursor + 1) % farm::AREA;
        let block = host.block(target);
        let ripe = farm::crop(&block).map(|crop| crop.ripe).or_else(|| stem_ripe(&block));
        let Some(ripe) = ripe else { continue };
        let Some(age) = host.block_property(target, "age").and_then(|age| age.parse::<u8>().ok()) else { continue };
        if age >= ripe {
            continue;
        }
        let energy = number(entity, "energy");
        if energy < GROW_COST {
            starved = true;
            break;
        }
        if host.set_block_property(target, "age", &(age + 1).to_string()) {
            set_number(entity, "energy", energy - GROW_COST);
            set_number(entity, "Total", number(entity, "Total").saturating_add(1));
            set_number(entity, "Cooldown", 4);
            acted = true;
            break;
        }
    }
    set_number(entity, "Scan", cursor);
    if starved {
        set_number(entity, "Status", 5);
    } else {
        remember_activity(entity, time, acted);
    }
}

/// Plantas altas que a colheitadeira corta acima da base, deixando a base para rebrotar.
pub(crate) fn tall(block: &str) -> Option<&'static str> {
    Some(match block {
        "minecraft:sugar_cane" => "minecraft:sugar_cane",
        "minecraft:cactus" => "minecraft:cactus",
        "minecraft:bamboo" => "minecraft:bamboo",
        _ => return None,
    })
}

/// Frutos de caule: só são colhidos se um caule estiver preso a eles.
pub(crate) fn fruit(block: &str) -> Option<(&'static str, u32, &'static str)> {
    Some(match block {
        "minecraft:melon" => ("minecraft:melon_slice", 5, "minecraft:attached_melon_stem"),
        "minecraft:pumpkin" => ("minecraft:pumpkin", 1, "minecraft:attached_pumpkin_stem"),
        _ => return None,
    })
}
