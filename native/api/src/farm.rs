//! Colheitadeira: colhe e replanta as plantações do Minecraft ao redor do bloco.
use super::*;
use horizon_client::mod_api::{ParticleOptions, ParticleType};

/// Alcance horizontal: 4 blocos para cada lado formam uma área de 9 x 9.
pub(super) const RADIUS: i32 = 4;
/// Camadas verificadas: uma abaixo, a da máquina e uma acima.
const LAYERS: i32 = 3;
const SIDE: i32 = RADIUS * 2 + 1;
pub(super) const AREA: i32 = SIDE * SIDE * LAYERS;
/// Posições verificadas por tick: uma volta completa a cada 81 ticks (4 s).
const SCANS_PER_TICK: i32 = 3;
pub(super) const HARVEST_COST: i32 = 120;
const GROW_COST: i32 = 40;
const ABSORB_PER_HARVEST: f32 = 3.0;
/// Estágios que uma farinha de osso adianta.
const GROWTH_PER_BONE_MEAL: u8 = 2;

pub(super) struct Crop {
    /// Idade madura (`age` máximo da planta vanilla).
    pub ripe: u8,
    /// Idade depois da colheita: a planta continua no lugar, replantada.
    pub reset: u8,
    pub drops: &'static [(&'static str, u32)],
}

pub(super) fn crop(block: &str) -> Option<Crop> {
    Some(match block {
        "minecraft:wheat" => Crop { ripe: 7, reset: 0, drops: &[("minecraft:wheat", 1), ("minecraft:wheat_seeds", 1)] },
        "minecraft:carrots" => Crop { ripe: 7, reset: 0, drops: &[("minecraft:carrot", 2)] },
        "minecraft:potatoes" => Crop { ripe: 7, reset: 0, drops: &[("minecraft:potato", 2)] },
        "minecraft:beetroots" => Crop { ripe: 3, reset: 0, drops: &[("minecraft:beetroot", 1), ("minecraft:beetroot_seeds", 1)] },
        "minecraft:nether_wart" => Crop { ripe: 3, reset: 0, drops: &[("minecraft:nether_wart", 2)] },
        "minecraft:sweet_berry_bush" => Crop { ripe: 3, reset: 1, drops: &[("minecraft:sweet_berries", 2)] },
        "minecraft:cocoa" => Crop { ripe: 2, reset: 0, drops: &[("minecraft:cocoa_beans", 2)] },
        _ => return None,
    })
}

/// Posição número `index` da varredura, em volta de `pos` e sem incluir a própria máquina.
pub(super) fn scan_position(pos: [i32; 3], index: i32) -> [i32; 3] {
    let index = index.rem_euclid(AREA);
    let layer = index / (SIDE * SIDE);
    let cell = index % (SIDE * SIDE);
    [pos[0] + cell % SIDE - RADIUS, pos[1] + layer - 1, pos[2] + cell / SIDE - RADIUS]
}

/// Cabem todos os produtos nas saídas 1 a 3?
pub(super) fn drops_fit(slots: &[Option<ItemStack>], drops: &[(&str, u32)]) -> bool {
    let mut room: Vec<Option<ItemStack>> = slots.to_vec();
    drops.iter().all(|&(item, count)| store(&mut room, item, count))
}

/// Guarda `count` de `item` nas saídas, juntando pilhas iguais primeiro.
pub(super) fn store(slots: &mut [Option<ItemStack>], item: &str, count: u32) -> bool {
    let mut left = count;
    for slot in slots.iter_mut() {
        if let Some(stack) = slot.as_mut().filter(|stack| is_plain(stack, item)) {
            let moved = left.min(64 - stack.count.min(64));
            stack.count += moved;
            left -= moved;
        }
    }
    for slot in slots.iter_mut() {
        if left == 0 {
            break;
        }
        if slot.is_none() {
            let moved = left.min(64);
            *slot = Some(ItemStack::new(item, moved));
            left -= moved;
        }
    }
    left == 0
}

pub(super) fn tick(host: &mut dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity, time: i64) {
    production::crops::flush(host, pos, entity, time);
    let minimum = if entity.items[0].as_ref().is_some_and(|stack| is_plain(stack, BONE_MEAL)) { GROW_COST } else { HARVEST_COST };
    if number(entity, "energy") < minimum {
        set_number(entity, "Status", 5);
        return;
    }
    if !budget::request(budget::Lane::Terrain, host.dimension(), pos, 12) { return; }
    let mut cursor = number(entity, "Scan");
    let mut acted = false;
    for _ in 0..SCANS_PER_TICK {
        let target = scan_position(pos, cursor);
        match visit(host, target, entity) {
            Visit::Skip => {}
            Visit::Acted => acted = true,
            Visit::Wait(status) => {
                // Mantém o cursor: a mesma planta é tentada de novo quando houver energia ou espaço.
                set_number(entity, "Scan", cursor);
                set_number(entity, "Status", status);
                return;
            }
        }
        cursor = (cursor + 1) % AREA;
    }
    set_number(entity, "Scan", cursor);
    set_number(entity, "Progress", cursor);
    set_number(entity, "Duration", AREA);
    let now = (time % i64::from(i32::MAX)) as i32;
    if acted {
        set_number(entity, "BusyAt", now);
    }
    let busy = entity.nbt.contains_key(nbt_key("BusyAt")) && now.wrapping_sub(number(entity, "BusyAt")) < 100;
    set_number(entity, "Status", if busy { 1 } else { 10 });
}

enum Visit {
    Skip,
    Acted,
    Wait(i32),
}

fn visit(host: &mut dyn Host, target: [i32; 3], entity: &mut ModBlockEntity) -> Visit {
    let block = host.block(target);
    if let Some(item) = production::crops::tall(&block) {
        return visit_tall(host, target, entity, item);
    }
    if let Some(fruit) = production::crops::fruit(&block) {
        return visit_fruit(host, target, entity, fruit);
    }
    let Some(crop) = crop(&block) else { return Visit::Skip };
    let Some(age) = host.block_property(target, "age").and_then(|age| age.parse::<u8>().ok()) else {
        return Visit::Skip;
    };
    let energy = number(entity, "energy");
    if age >= crop.ripe {
        if !drops_fit(&entity.items[1..4], crop.drops) {
            return Visit::Wait(3);
        }
        if energy < HARVEST_COST {
            return Visit::Wait(5);
        }
        if !host.set_block_property(target, "age", &crop.reset.to_string()) {
            return Visit::Skip;
        }
        for &(item, count) in crop.drops {
            store(&mut entity.items[1..4], item, count);
        }
        set_number(entity, "energy", energy - HARVEST_COST);
        set_number(entity, "Produced", number(entity, "Produced").saturating_add(1));
        set_number(entity, "Total", number(entity, "Total").saturating_add(1));
        // Plantas absorvem poluição: a lavoura ajuda a fábrica a passar despercebida.
        pollution::absorb(host.dimension(), target, ABSORB_PER_HARVEST);
        let at = center(target);
        host.send_particles(&ParticleOptions::simple(ParticleType::Composter), None, at, [0.3, 0.2, 0.3], 0.0, 6);
        host.play_sound("minecraft:block.crop.break", at, 4, 0.6, 1.0);
        return Visit::Acted;
    }
    let has_bone_meal = entity.items[0].as_ref().is_some_and(|stack| is_plain(stack, BONE_MEAL));
    if !has_bone_meal {
        return Visit::Skip;
    }
    if energy < GROW_COST {
        return Visit::Wait(5);
    }
    let grown = (age + GROWTH_PER_BONE_MEAL).min(crop.ripe);
    if !host.set_block_property(target, "age", &grown.to_string()) {
        return Visit::Skip;
    }
    consume(&mut entity.items[0], 1);
    set_number(entity, "energy", energy - GROW_COST);
    let at = center(target);
    host.send_particles(&ParticleOptions::simple(ParticleType::HappyVillager), None, at, [0.3, 0.2, 0.3], 0.0, 8);
    host.play_sound("minecraft:item.bone_meal.use", at, 4, 0.5, 1.0);
    Visit::Acted
}

/// Corta cana, cacto ou bambu acima da base. Só a base é visitada: ela tem outro bloco embaixo.
fn visit_tall(host: &mut dyn Host, [x, y, z]: [i32; 3], entity: &mut ModBlockEntity, item: &'static str) -> Visit {
    let plant = host.block([x, y, z]);
    if host.block([x, y - 1, z]) == plant || production::crops::is_queued(entity, [x, y + 1, z]) {
        return Visit::Skip;
    }
    let height = (1..16).take_while(|dy| host.block([x, y + dy, z]) == plant).count() as u32;
    if height == 0 {
        return Visit::Skip;
    }
    collect(host, [x, y + 1, z], height as i32, entity, &[(item, height)])
}

/// Colhe abóbora ou melancia presa a um caule; o caule continua produzindo.
fn visit_fruit(host: &mut dyn Host, [x, y, z]: [i32; 3], entity: &mut ModBlockEntity,
    (item, count, stem): (&'static str, u32, &'static str)) -> Visit
{
    let attached = [[x + 1, y, z], [x - 1, y, z], [x, y, z + 1], [x, y, z - 1]].into_iter()
        .any(|side| host.block(side) == stem);
    if !attached || production::crops::is_queued(entity, [x, y, z]) {
        return Visit::Skip;
    }
    collect(host, [x, y, z], 1, entity, &[(item, count)])
}

/// Guarda os produtos e agenda a remoção dos blocos colhidos.
fn collect(host: &mut dyn Host, from: [i32; 3], height: i32, entity: &mut ModBlockEntity, drops: &[(&'static str, u32)]) -> Visit {
    if production::crops::queue_full(entity) {
        return Visit::Wait(1);
    }
    if !drops_fit(&entity.items[1..4], drops) {
        return Visit::Wait(3);
    }
    let energy = number(entity, "energy");
    if energy < HARVEST_COST {
        return Visit::Wait(5);
    }
    for &(item, count) in drops {
        store(&mut entity.items[1..4], item, count);
    }
    set_number(entity, "energy", energy - HARVEST_COST);
    set_number(entity, "Produced", number(entity, "Produced").saturating_add(1));
    set_number(entity, "Total", number(entity, "Total").saturating_add(1));
    pollution::absorb(host.dimension(), from, ABSORB_PER_HARVEST);
    production::crops::enqueue(entity, from, height, production::crops::AIR);
    host.play_sound("minecraft:block.crop.break", center(from), 4, 0.6, 1.0);
    Visit::Acted
}

fn center([x, y, z]: [i32; 3]) -> [f64; 3] {
    [f64::from(x) + 0.5, f64::from(y) + 0.5, f64::from(z) + 0.5]
}
