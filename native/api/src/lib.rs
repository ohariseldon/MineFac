mod presentation;
mod contracts;
mod tutorial;

use horizon_client::mod_api::{
    BlockEntityContainerEvent, BlockEntityEvent, BlockEntityTickEvent, BlockEntityTicksEvent,
    Compound, ContainerAcceptsEvent, Direction, Flow, Hooks, Host, ItemStack, ModBlockEntity, ModContainer, ModEnergyBuffer, Tag,
};

const DRILL: &str = "veio:drill";
const PRESS: &str = "veio:press";
const ASSEMBLER: &str = "veio:assembler";
const BELT: &str = "veio:belt";
const FAST_BELT: &str = "veio:fast_belt";
const SPLITTER: &str = "veio:splitter";
const CUTTER: &str = "veio:cutter";
const INTEGRATOR: &str = "veio:integrator";
const COPPER: &str = "minecraft:raw_copper";
const IRON: &str = "minecraft:raw_iron";
const PLATE: &str = "veio:copper_plate";
const IRON_PLATE: &str = "veio:iron_plate";
const GEAR: &str = "veio:gear";
const FRAME: &str = "veio:frame";
const MODULE: &str = "veio:module";
const GENERATOR: &str = "veio:generator";
const CABLE: &str = "veio:cable";
const SOLAR: &str = "veio:solar";
const BATTERY: &str = "veio:battery";
const GEOTHERMAL: &str = "veio:geothermal";
const CRUSHER: &str = "veio:crusher";
const ALLOY: &str = "veio:alloy_furnace";
const ELECTRONICS: &str = "veio:electronics";
const DRILL_MK2: &str = "veio:drill_mk2";
const STEEL: &str = "veio:steel_plate";
const SILICON: &str = "veio:silicon";
const WIRE: &str = "veio:wire";
const CIRCUIT: &str = "veio:circuit";
const WRENCH: &str = "veio:wrench";
const HUB: &str = "veio:contract_hub";


pub fn hooks() -> Box<dyn Hooks> {
    Box::new(Factory)
}

struct Factory;

impl Hooks for Factory {
    fn block_entity(
        &self,
        _host: &mut dyn Host,
        event: &mut BlockEntityEvent<'_>,
    ) -> Option<(String, Compound)> {
        is_factory_block(event.block).then(|| (event.block.to_owned(), Compound::default()))
    }

    fn block_entity_ticks(&self, _host: &dyn Host, event: &mut BlockEntityTicksEvent<'_>) {
        if is_factory_block(event.block) {
            event.ticks = true;
        }
    }

    fn block_entity_container(
        &self,
        _host: &dyn Host,
        event: &mut BlockEntityContainerEvent<'_>,
    ) -> Option<ModContainer> {
        let (slots, inputs, outputs) = match event.block {
            DRILL | DRILL_MK2 => (1, vec![], vec![0]),
            PRESS | ASSEMBLER | CUTTER | CRUSHER => (2, vec![0], vec![1]),
            INTEGRATOR | ALLOY | ELECTRONICS => (3, vec![0, 1], vec![2]),
            BELT | FAST_BELT | SPLITTER => (1, vec![0], vec![0]),
            GENERATOR => (1, vec![0], vec![]),
            GEOTHERMAL => (2, vec![0], vec![1]),
            SOLAR | BATTERY => (1, vec![], vec![]),
            CABLE => (0, vec![], vec![]),
            HUB => (4, vec![0, 1], vec![2, 3]),
            _ => return None,
        };
        Some(ModContainer {
            slots,
            faces: std::array::from_fn(|_| (0..slots).collect()),
            inputs,
            outputs,
            menu: (slots > 0).then(|| event.block.to_owned()),
        })
    }

    fn container_accepts(&self, _host: &dyn Host, event: &mut ContainerAcceptsEvent<'_>) -> Flow {
        if event.entity.id == HUB {
            return if contracts::accepts(event.entity, event.slot, event.stack) { Flow::Continue } else { Flow::Cancel };
        }
        if accepts(&event.entity.id, event.slot, event.stack) {
            Flow::Continue
        } else {
            Flow::Cancel
        }
    }

    fn right_click_block(&self, host: &mut dyn Host,
        event: &mut horizon_client::mod_api::RightClickBlockEvent) -> Flow
    {
        if !host.held().is_some_and(|held| held.0 == WRENCH) {
            return Flow::Continue;
        }
        let block = host.block(event.pos);
        if !is_factory_block(&block) {
            return Flow::Continue;
        }
        let facing = host.block_property(event.pos, "facing");
        let next = match facing.as_deref() {
            Some("north") => "east",
            Some("east") => "south",
            Some("south") => "west",
            Some("west") => "north",
            _ => return Flow::Continue,
        };
        if host.set_block_property(event.pos, "facing", next) { Flow::Cancel }
        else { Flow::Continue }
    }

    fn block_entity_energy(
        &self,
        _host: &dyn Host,
        event: &mut horizon_client::mod_api::BlockEntityEnergyEvent<'_>,
    ) -> Option<ModEnergyBuffer> {
        if !needs_power(event.block) && !matches!(event.block, GENERATOR | CABLE | SOLAR | BATTERY | GEOTHERMAL) {
            return None;
        }
        Some(ModEnergyBuffer {
            capacity: capacity(event.block),
            max_insert: 80,
            max_extract: 80,
            insert_faces: [!matches!(event.block, GENERATOR | SOLAR | GEOTHERMAL); 6],
            extract_faces: [matches!(event.block, GENERATOR | CABLE | SOLAR | BATTERY | GEOTHERMAL); 6],
        })
    }

    fn block_entity_tick(&self, host: &mut dyn Host, event: &mut BlockEntityTickEvent<'_>) {
        if !is_factory_block(&event.entity.id) {
            return;
        }
        let Some(time) = host.level(host.dimension()).and_then(|level| level.game_time(host)) else {
            return;
        };
        let previous = visual_signature(event.entity);
        let produced = number(event.entity, "Produced");
        tick_factory(host, event.pos, event.entity, time);
        record_performance(event.entity, produced);
        presentation::publish(event.entity);
        if visual_signature(event.entity) != previous {
            host.block_entity_sync(event.pos);
        }
    }

    fn screen(&self, _host: &dyn Host, event: &mut horizon_client::mod_api::ScreenEvent<'_>)
        -> Option<horizon_client::mod_api::NativeScreen>
    {
        presentation::screen(event.block)
    }

    fn screen_draw(&self, host: &dyn Host, event: &mut horizon_client::mod_api::ScreenDrawEvent<'_>) {
        presentation::screen_draw(host, event);
    }

    fn screen_tooltip(&self, _host: &dyn Host, event: &mut horizon_client::mod_api::ScreenTooltipEvent<'_>) {
        tutorial::screen_tooltip(event);
    }

    fn tooltip(&self, _host: &dyn Host, event: &mut horizon_client::mod_api::TooltipEvent<'_>) {
        tutorial::tooltip(event);
    }

    fn menu_slot_rules(&self, event: &mut horizon_client::mod_api::MenuSlotRulesEvent<'_>) {
        if !is_factory_block(event.menu) {
            return;
        }
        let slots = match event.menu {
            INTEGRATOR | ALLOY | ELECTRONICS => 3,
            PRESS | ASSEMBLER | CUTTER | CRUSHER | GEOTHERMAL => 2,
            CABLE => 0,
            HUB => 4,
            _ => 1,
        };
        if event.slot < slots {
            event.may_place &= accepts(event.menu, event.slot, event.stack);
        }
    }

    fn block_entity_update_tag(
        &self,
        event: &mut horizon_client::mod_api::BlockEntityUpdateTagEvent<'_>,
    ) -> Option<Compound> {
        is_factory_block(&event.entity.id).then(|| event.entity.nbt.clone())
    }

    fn block_entity_submit(&self, host: &dyn Host,
        event: &mut horizon_client::mod_api::BlockEntitySubmitEvent<'_>)
    {
        presentation::draw(host, event);
    }

}

fn is_factory_block(block: &str) -> bool {
    matches!(block, DRILL | DRILL_MK2 | PRESS | ASSEMBLER | BELT | FAST_BELT |
        SPLITTER | CUTTER | INTEGRATOR | GENERATOR | CABLE | SOLAR | BATTERY |
        GEOTHERMAL | CRUSHER | ALLOY | ELECTRONICS | HUB)
}

fn needs_power(block: &str) -> bool {
    matches!(block, DRILL | DRILL_MK2 | PRESS | ASSEMBLER | CUTTER | INTEGRATOR |
        CRUSHER | ALLOY | ELECTRONICS)
}

fn capacity(block: &str) -> i32 {
    if block == BATTERY { 30000 } else { 2000 }
}

fn ore_product(block: &str) -> Option<&'static str> {
    Some(match block {
        "minecraft:copper_ore" | "minecraft:deepslate_copper_ore" => COPPER,
        "minecraft:iron_ore" | "minecraft:deepslate_iron_ore" => IRON,
        "minecraft:gold_ore" | "minecraft:deepslate_gold_ore" => "minecraft:raw_gold",
        "minecraft:coal_ore" | "minecraft:deepslate_coal_ore" => "minecraft:coal",
        "minecraft:redstone_ore" | "minecraft:deepslate_redstone_ore" => "minecraft:redstone",
        "minecraft:nether_quartz_ore" => "minecraft:quartz",
        _ => return None,
    })
}

fn is_fuel(stack: &ItemStack) -> bool {
    is_plain(stack, "minecraft:coal") || is_plain(stack, "minecraft:charcoal")
}

fn accepts(block: &str, slot: usize, stack: &ItemStack) -> bool {
    match (block, slot) {
        (HUB, _) => contracts::valid_input(slot, stack),
        (GENERATOR, 0) => is_fuel(stack),
        (PRESS, 0) => [COPPER, IRON, "minecraft:raw_gold"].iter().any(|id| is_plain(stack, id)),
        (ASSEMBLER, 0) => is_plain(stack, PLATE),
        (CUTTER, 0) => is_plain(stack, IRON_PLATE) || is_plain(stack, PLATE),
        (CRUSHER, 0) => is_plain(stack, "minecraft:quartz"),
        (ALLOY, 0) => is_plain(stack, IRON_PLATE),
        (ALLOY, 1) => is_fuel(stack),
        (ELECTRONICS, 0) => is_plain(stack, WIRE),
        (ELECTRONICS, 1) => is_plain(stack, SILICON),
        (GEOTHERMAL, 0) => is_plain(stack, "minecraft:lava_bucket"),
        (INTEGRATOR, 0) => is_plain(stack, FRAME),
        (INTEGRATOR, 1) => is_plain(stack, GEAR),
        (BELT | FAST_BELT | SPLITTER, 0) => true,
        _ => false,
    }
}

fn number(entity: &ModBlockEntity, key: &str) -> i32 {
    match entity.nbt.get(&key.into()) {
        Some(Tag::Int(value)) => (*value).max(0),
        _ => 0,
    }
}

fn set_number(entity: &mut ModBlockEntity, key: &str, value: i32) {
    if number(entity, key) != value {
        entity.nbt.insert(key.into(), Tag::Int(value));
    }
}

fn visual_signature(entity: &ModBlockEntity) -> (i32, Option<ItemStack>) {
    (number(entity, "Status"), entity.items.first().cloned().flatten())
}

fn tick_factory(host: &mut dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity, time: i64) {
    let output_slot = match entity.id.as_str() {
        PRESS | ASSEMBLER | CUTTER | CRUSHER | GEOTHERMAL => 1,
        INTEGRATOR | ALLOY | ELECTRONICS | HUB => 2,
        _ => 0,
    };
    let [x, y, z] = pos;
    let logistics = matches!(entity.id.as_str(), BELT | FAST_BELT | SPLITTER);
    if time.rem_euclid(8) == i64::from((x ^ z) & 1) * 4
        && let Some(stack) = entity.items.get(output_slot).and_then(Option::as_ref).or_else(|| {
            if entity.id == HUB { entity.items.get(3).and_then(Option::as_ref) } else { None }
        })
        && let Some(facing) = output_direction(host, pos)
        && !matches!(entity.id.as_str(), GENERATOR | SOLAR | BATTERY)
    {
        if entity.id == HUB {
            for slot in 2..4 {
                if let Some(stack) = entity.items[slot].as_ref() {
                    let moved = send(host, pos, facing, stack, 1);
                    if moved > 0 { consume(&mut entity.items[slot], moved); }
                }
            }
        } else if entity.id == SPLITTER {
            let before = entity.items[0].as_ref().map_or(0, |stack| stack.count);
            split(host, pos, facing, entity, 2);
            if entity.items[0].as_ref().map_or(0, |stack| stack.count) < before {
                set_number(entity, "MovedAt", (time % i64::from(i32::MAX)) as i32);
            }
        } else {
            let batch = if entity.id == FAST_BELT { 2 } else { 1 };
            let moved = send(host, pos, facing, stack, batch);
            if moved > 0 {
                consume(&mut entity.items[output_slot], moved);
                set_number(entity, "MovedAt", (time % i64::from(i32::MAX)) as i32);
            }
        }
    }
    if entity.id == HUB {
        contracts::tick(entity);
        return;
    }
    if logistics {
        let status = if entity.items[0].is_none() { 0 }
            else if (time % i64::from(i32::MAX)) as i32 - number(entity, "MovedAt") > 8 { 3 }
            else { 1 };
        set_number(entity, "Status", status);
        return;
    }
    if matches!(entity.id.as_str(), GENERATOR | CABLE | SOLAR | BATTERY | GEOTHERMAL) {
        tick_power(host, pos, entity);
        return;
    }
    if needs_power(&entity.id) && host.neighbor_signal(host.dimension(), pos) > 0 {
        set_number(entity, "Status", 9);
        return;
    }
    let (recipe, output, duration, draw): (&[(&str, u32)], &str, i32, i32) = match entity.id.as_str() {
        DRILL | DRILL_MK2 => {
            let Some(ore) = ore_product(&host.block([x, y - 1, z])) else {
                set_number(entity, "Status", 4);
                return;
            };
            let advanced = entity.id == DRILL_MK2;
            (&[], ore, if advanced { 20 } else { 40 }, if advanced { 12 } else { 6 })
        }
        PRESS => {
            if entity.items[0].as_ref().is_some_and(|s| is_plain(s, IRON)) {
                (&[(IRON, 1)], IRON_PLATE, 30, 8)
            } else if entity.items[0].as_ref().is_some_and(|s| is_plain(s, "minecraft:raw_gold")) {
                (&[("minecraft:raw_gold", 1)], "minecraft:gold_ingot", 40, 10)
            } else {
                (&[(COPPER, 1)], PLATE, 30, 8)
            }
        }
        ASSEMBLER => (&[(PLATE, 2)], GEAR, 40, 12),
        CUTTER => {
            if entity.items[0].as_ref().is_some_and(|s| is_plain(s, PLATE)) {
                (&[(PLATE, 1)], WIRE, 20, 6)
            } else {
                (&[(IRON_PLATE, 1)], FRAME, 30, 8)
            }
        }
        CRUSHER => (&[("minecraft:quartz", 1)], SILICON, 50, 10),
        ALLOY => {
            let fuel = if entity.items[1].as_ref().is_some_and(|s| is_plain(s, "minecraft:charcoal")) {
                "minecraft:charcoal"
            } else { "minecraft:coal" };
            (&[(IRON_PLATE, 2), (fuel, 1)], STEEL, 120, 16)
        }
        ELECTRONICS => (&[(WIRE, 2), (SILICON, 1)], CIRCUIT, 60, 12),
        INTEGRATOR => (&[(FRAME, 1), (GEAR, 1)], MODULE, 60, 16),
        _ => return,
    };
    set_number(entity, "Duration", duration);
    let recipe_key = "Recipe".into();
    let recipe_changed = !matches!(entity.nbt.get(&recipe_key), Some(Tag::String(old)) if old.to_string_lossy() == output);
    if recipe_changed {
        entity.nbt.insert(recipe_key, Tag::String(output.into()));
        set_number(entity, "Progress", 0);
    }
    if !can_add(&entity.items[output_slot], output) {
        set_number(entity, "Status", 3);
        return;
    }
    if !ingredients_ready(&entity.items, recipe) {
        set_number(entity, "Status", 2);
        return;
    }
    let energy = number(entity, "energy");
    if energy < draw {
        set_number(entity, "Status", 5);
        return;
    }
    set_number(entity, "energy", energy - draw);
    set_number(entity, "Status", 1);
    let progress = number(entity, "Progress") + 1;
    if progress >= duration {
        if matches!(entity.id.as_str(), DRILL | DRILL_MK2) {
            add_one(&mut entity.items[0], output);
        } else {
            process(&mut entity.items, recipe, output);
        }
        let produced = number(entity, "Produced").saturating_add(1);
        set_number(entity, "Produced", produced);
        set_number(entity, "Progress", 0);
    } else {
        set_number(entity, "Progress", progress);
    }
}

fn tick_power(host: &mut dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity) {
    let limit = capacity(&entity.id);
    let mut energy = number(entity, "energy").min(limit);
    match entity.id.as_str() {
        GENERATOR if energy <= limit - 40 => {
            let mut burn = number(entity, "Burn");
            if burn == 0 && entity.items[0].as_ref().is_some_and(is_fuel) {
                consume(&mut entity.items[0], 1);
                burn = 400;
            }
            if burn > 0 {
                energy += 40;
                set_number(entity, "Burn", burn - 1);
            }
        }
        GEOTHERMAL if energy <= limit - 80 => {
            let mut burn = number(entity, "Burn");
            let bucket_room = entity.items[1].as_ref().is_none_or(|stack|
                is_plain(stack, "minecraft:bucket") && stack.count < 16);
            if burn == 0 && bucket_room
                && entity.items[0].as_ref().is_some_and(|s| is_plain(s, "minecraft:lava_bucket"))
            {
                consume(&mut entity.items[0], 1);
                add_one(&mut entity.items[1], "minecraft:bucket");
                burn = 800;
            }
            if burn > 0 {
                energy += 80;
                set_number(entity, "Burn", burn - 1);
            }
        }
        SOLAR => {
            let daylight = host.day_time().rem_euclid(24000);
            let [x, y, z] = pos;
            let open_sky = host.light([x, y + 1, z]).1 == 15;
            if host.dimension() == "minecraft:overworld" && open_sky && daylight < 12000 {
                let (rain, thunder) = host.weather();
                let sun = (daylight as f32 * std::f32::consts::PI / 12000.0).sin().max(0.0);
                let output = (16.0 * sun * (1.0 - 0.6 * rain - 0.4 * thunder).max(0.0)) as i32;
                energy = (energy + output).min(limit);
            }
        }
        _ => {}
    }
    let offset = number(entity, "Port") as usize % 6;
    for i in 0..6 {
        let face = Direction::ALL[(i + offset) % 6];
        let delta = face.normal();
        let target = std::array::from_fn(|axis| pos[axis] + delta[axis]);
        host.change_mod_block_entity(target, &mut |other| {
            if !needs_power(&other.id) && !matches!(other.id.as_str(), CABLE | BATTERY) {
                return;
            }
            let other_limit = capacity(&other.id);
            let stored = number(other, "energy").min(other_limit);
            // Equalize charge percentages, so large batteries can charge and discharge.
            let offered = if matches!(other.id.as_str(), CABLE | BATTERY) {
                ((i64::from(energy) * i64::from(other_limit)
                    - i64::from(stored) * i64::from(limit))
                    / i64::from(limit + other_limit)).clamp(0, 80) as i32
            } else { energy.min(80) };
            let accepted = offered.min(other_limit - stored);
            if accepted > 0 {
                set_number(other, "energy", stored + accepted);
                energy -= accepted;
            }
        });
    }
    set_number(entity, "Port", ((offset + 1) % 6) as i32);
    set_number(entity, "energy", energy);
    let status = if energy == limit { 8 }
        else if matches!(entity.id.as_str(), GENERATOR | GEOTHERMAL) && energy == 0 && number(entity, "Burn") == 0 { 6 }
        else if entity.id == SOLAR && energy == 0 { 7 }
        else if energy > 0 { 1 } else { 5 };
    set_number(entity, "Status", status);
}

fn record_performance(entity: &mut ModBlockEntity, previous_produced: i32) {
    if !needs_power(&entity.id) { return; }
    let ticks = number(entity, "MeterTicks") + 1;
    let active = number(entity, "MeterActive") + i32::from(number(entity, "Status") == 1);
    let items = number(entity, "MeterItems") + (number(entity, "Produced") - previous_produced).max(0);
    if number(entity, "MeterReady") == 0 || ticks >= 1200 {
        set_number(entity, "Rate", items * 1200 / ticks);
        set_number(entity, "Utilization", active * 100 / ticks);
    }
    if ticks >= 1200 {
        set_number(entity, "MeterReady", 1);
        set_number(entity, "MeterTicks", 0);
        set_number(entity, "MeterActive", 0);
        set_number(entity, "MeterItems", 0);
    } else {
        set_number(entity, "MeterTicks", ticks);
        set_number(entity, "MeterActive", active);
        set_number(entity, "MeterItems", items);
    }
}

fn ingredients_ready(slots: &[Option<ItemStack>], recipe: &[(&str, u32)]) -> bool {
    slots.len() >= recipe.len() && slots.iter().zip(recipe).all(|(slot, &(item, count))| {
        slot.as_ref().is_some_and(|stack| is_plain(stack, item) && stack.count >= count)
    })
}

fn split(
    host: &mut dyn Host,
    pos: [i32; 3],
    facing: Direction,
    entity: &mut ModBlockEntity,
    batch: u32,
) {
    let key = "PreferRight".into();
    let was_right = matches!(entity.nbt.get(&key), Some(Tag::Byte(1)));
    let mut prefer_right = was_right;
    for _ in 0..batch {
        let Some(stack) = entity.items[0].as_ref() else {
            break;
        };
        let preferred = turn(facing, prefer_right);
        let moved = send(host, pos, preferred, stack, 1);
        if moved != 0 {
            prefer_right = !prefer_right;
            consume(&mut entity.items[0], moved);
        } else {
            let moved = send(host, pos, preferred.opposite(), stack, 1);
            if moved == 0 {
                break;
            }
            consume(&mut entity.items[0], moved);
        }
    }
    if was_right != prefer_right {
        entity.nbt.insert(key, Tag::Byte(i8::from(prefer_right)));
    }
}

fn send(host: &mut dyn Host, pos: [i32; 3], facing: Direction, stack: &ItemStack, batch: u32) -> u32 {
    let [x, y, z] = pos;
    let [dx, _, dz] = facing.normal();
    let mut offered = stack.clone();
    let count = stack.count.min(batch);
    offered.count = count;
    let returned = host.insert_item([x + dx, y, z + dz], facing.opposite(), offered);
    count - returned.map_or(0, |remainder| remainder.count)
}

fn turn(facing: Direction, right: bool) -> Direction {
    let clockwise = match facing {
        Direction::North => Direction::East,
        Direction::East => Direction::South,
        Direction::South => Direction::West,
        Direction::West => Direction::North,
        _ => unreachable!("factory blocks face horizontally"),
    };
    if right { clockwise } else { clockwise.opposite() }
}

fn output_direction(host: &dyn Host, pos: [i32; 3]) -> Option<Direction> {
    // Glazed terracotta faces the player on placement. Belts run away from them.
    Some(match host.block_property(pos, "facing")?.as_str() {
        "north" => Direction::South,
        "south" => Direction::North,
        "west" => Direction::East,
        "east" => Direction::West,
        _ => return None,
    })
}

fn is_plain(stack: &ItemStack, item: &str) -> bool {
    stack.item.as_ref() == item && stack.damage == 0 && stack.extra.is_none()
}

fn can_add(slot: &Option<ItemStack>, item: &str) -> bool {
    slot.as_ref()
        .is_none_or(|stack| is_plain(stack, item) && stack.count < 64)
}

fn add_one(slot: &mut Option<ItemStack>, item: &str) {
    if let Some(stack) = slot {
        stack.count += 1;
    } else {
        *slot = Some(ItemStack::new(item, 1));
    }
}

fn consume(slot: &mut Option<ItemStack>, count: u32) {
    if let Some(stack) = slot {
        stack.count -= count;
        if stack.count == 0 {
            *slot = None;
        }
    }
}

fn process(slots: &mut [Option<ItemStack>], recipe: &[(&str, u32)], output: &str) {
    if slots.len() <= recipe.len() {
        return;
    }
    let (inputs, outputs) = slots.split_at_mut(recipe.len());
    let output_slot = &mut outputs[0];
    if !can_add(output_slot, output) || !ingredients_ready(inputs, recipe) {
        return;
    }
    for (slot, &(_, count)) in inputs.iter_mut().zip(recipe) {
        consume(slot, count);
    }
    add_one(output_slot, output);
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integrator_waits_for_both_inputs_and_preserves_them_when_output_is_full() {
        let mut slots = vec![Some(ItemStack::new(FRAME, 2)), None, None];
        process(&mut slots, &[(FRAME, 1), (GEAR, 1)], MODULE);
        assert_eq!(slots[0].as_ref().unwrap().count, 2);
        assert!(slots[2].is_none());
        slots[1] = Some(ItemStack::new(GEAR, 3));
        slots[2] = Some(ItemStack::new(MODULE, 64));
        let blocked = slots.clone();
        process(&mut slots, &[(FRAME, 1), (GEAR, 1)], MODULE);
        assert_eq!(slots, blocked);
        slots[2] = None;
        process(&mut slots, &[(FRAME, 1), (GEAR, 1)], MODULE);
        assert_eq!(slots[0].as_ref().unwrap().count, 1);
        assert_eq!(slots[1].as_ref().unwrap().count, 2);
        assert_eq!(slots[2], Some(ItemStack::new(MODULE, 1)));
    }

    #[test]
    fn menus_fence_outputs_and_route_integrator_inputs() {
        assert!(accepts(INTEGRATOR, 0, &ItemStack::new(FRAME, 1)));
        assert!(!accepts(INTEGRATOR, 0, &ItemStack::new(GEAR, 1)));
        assert!(accepts(INTEGRATOR, 1, &ItemStack::new(GEAR, 1)));
        assert!(!accepts(INTEGRATOR, 2, &ItemStack::new(MODULE, 1)));
        assert!(!accepts(DRILL, 0, &ItemStack::new(COPPER, 1)));
        assert!(accepts(GENERATOR, 0, &ItemStack::new("minecraft:charcoal", 1)));
        assert!(!accepts(GENERATOR, 0, &ItemStack::new("minecraft:stone", 1)));
    }
    #[test]
    fn generator_conserves_energy_and_pauses_fuel_when_buffers_are_full() {
        struct Grid { cable: ModBlockEntity }
        impl Host for Grid {
            fn change_mod_block_entity(
                &mut self,
                pos: [i32; 3],
                change: &mut dyn FnMut(&mut ModBlockEntity),
            ) -> bool {
                if pos != [1, 0, 0] { return false; }
                change(&mut self.cable);
                true
            }
        }
        let mut generator = ModBlockEntity::default();
        generator.id = GENERATOR.to_owned();
        generator.items = vec![Some(ItemStack::new("minecraft:coal", 2))];
        let mut cable = ModBlockEntity::default();
        cable.id = CABLE.to_owned();
        let mut grid = Grid { cable };
        tick_power(&mut grid, [0, 0, 0], &mut generator);
        assert_eq!(number(&generator, "energy") + number(&grid.cable, "energy"), 40);
        assert_eq!(generator.items[0].as_ref().unwrap().count, 1);
        let remaining_burn = number(&generator, "Burn");
        set_number(&mut generator, "energy", 2000);
        set_number(&mut grid.cable, "energy", 2000);
        tick_power(&mut grid, [0, 0, 0], &mut generator);
        assert_eq!(number(&generator, "Burn"), remaining_burn);
        assert_eq!(generator.items[0].as_ref().unwrap().count, 1);
        assert_eq!(number(&generator, "energy") + number(&grid.cable, "energy"), 4000);
    }

}
