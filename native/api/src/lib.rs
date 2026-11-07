mod presentation;
mod contracts;
mod logistics;
mod defense;
mod farm;
mod pollution;
mod power;
mod tutorial;
mod upgrades;

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
const HARVESTER: &str = "veio:harvester";
const TESLA: &str = "veio:tesla";
const BONE_MEAL: &str = "minecraft:bone_meal";
const POLE: &str = "veio:pole";
const STEEL_POLE: &str = "veio:steel_pole";
const REPAIR_KIT: &str = "veio:repair_kit";
/// Cada kit de reparo recupera metade da integridade.
const REPAIR: i32 = 50;


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
        let (slots, inputs, outputs) = layout(event.block)?;
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
        let Some((held, _, _)) = host.held() else { return Flow::Continue };
        let block = host.block(event.pos);
        if !is_factory_block(&block) {
            return Flow::Continue;
        }
        let pos = event.pos;
        let mut damage = 0;
        let mut module = 0;
        host.change_mod_block_entity(pos, &mut |entity| {
            damage = number(entity, "Damage");
            module = number(entity, "Module");
        });
        if held == REPAIR_KIT {
            if damage == 0 || host.take_items(REPAIR_KIT, 1) == 0 {
                return Flow::Continue;
            }
            host.change_mod_block_entity(pos, &mut |entity| {
                set_number(entity, "Damage", (number(entity, "Damage") - REPAIR).max(0));
            });
            host.play_sound("minecraft:block.anvil.use", center(pos), 4, 0.5, 1.4);
            return Flow::Cancel;
        }
        if let Some(index) = upgrades::index(&held) {
            if !upgrades::supports(&block) || module != 0 || host.take_items(&held, 1) == 0 {
                return Flow::Continue;
            }
            host.change_mod_block_entity(pos, &mut |entity| set_number(entity, "Module", index));
            host.play_sound("minecraft:block.smithing_table.use", center(pos), 4, 0.6, 1.2);
            return Flow::Cancel;
        }
        if held != WRENCH {
            return Flow::Continue;
        }
        if host.sneaking() && module > 0 {
            host.change_mod_block_entity(pos, &mut |entity| set_number(entity, "Module", 0));
            host.give_items(upgrades::MODULES[(module - 1) as usize], 1);
            return Flow::Cancel;
        }
        let facing = host.block_property(pos, "facing");
        let next = match facing.as_deref() {
            Some("north") => "east",
            Some("east") => "south",
            Some("south") => "west",
            Some("west") => "north",
            _ => return Flow::Continue,
        };
        if host.set_block_property(pos, "facing", next) { Flow::Cancel }
        else { Flow::Continue }
    }

    fn block_entity_energy(
        &self,
        _host: &dyn Host,
        event: &mut horizon_client::mod_api::BlockEntityEnergyEvent<'_>,
    ) -> Option<ModEnergyBuffer> {
        if !needs_power(event.block) && !is_power_block(event.block) && power::tier(event.block).is_none() {
            return None;
        }
        Some(ModEnergyBuffer {
            capacity: capacity(event.block),
            max_insert: 80,
            max_extract: 80,
            insert_faces: [!matches!(event.block, GENERATOR | SOLAR | GEOTHERMAL); 6],
            extract_faces: [is_power_block(event.block) || power::tier(event.block).is_some(); 6],
        })
    }

    fn block_entity_tick(&self, host: &mut dyn Host, event: &mut BlockEntityTickEvent<'_>) {
        if !is_factory_block(&event.entity.id) {
            return;
        }
        let Some(time) = host.level(host.dimension()).and_then(|level| level.game_time(host)) else {
            return;
        };
        pollution::update(host, time);
        if power::tick(host, event.pos, event.entity, time) {
            host.block_entity_sync(event.pos);
        }
        let previous = visual_signature(event.entity);
        let produced = number(event.entity, "Produced");
        tick_factory(host, event.pos, event.entity, time);
        pollution::emit(host, event.pos, event.entity, time);
        record_performance(event.entity, produced);
        presentation::publish(event.entity);
        if visual_signature(event.entity) != previous {
            host.block_entity_sync(event.pos);
        }
    }

    fn entity_tick_pre(&self, _host: &mut dyn Host,
        event: &mut horizon_client::mod_api::EntityTickPreEvent<'_>) -> Flow
    {
        defense::observe(event.dimension, event.entity, event.kind);
        Flow::Continue
    }

    fn tick(&self, _host: &mut dyn Host, _event: &mut horizon_client::mod_api::TickEvent) {
        defense::advance_clock();
    }

    fn register_goals(&self, event: &mut horizon_client::mod_api::RegisterGoalsEvent<'_>) {
        pollution::register_goals(event.mob);
    }

    fn block_entity_render_bounding_box(&self, _host: &dyn Host,
        event: &mut horizon_client::mod_api::BlockEntityRenderBoundingBoxEvent<'_>)
    {
        if power::tier(event.block).is_some() {
            power::bounds(event.entity, event.bounds);
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
        let Some((slots, _, _)) = layout(event.menu) else {
            return;
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
        GEOTHERMAL | CRUSHER | ALLOY | ELECTRONICS | HUB | HARVESTER | TESLA | POLE | STEEL_POLE)
}

/// Consumidores: recebem energia por cabos, param com redstone e mostram métricas.
fn needs_power(block: &str) -> bool {
    matches!(block, DRILL | DRILL_MK2 | PRESS | ASSEMBLER | CUTTER | INTEGRATOR |
        CRUSHER | ALLOY | ELECTRONICS | HARVESTER | TESLA)
}

/// Produzem, guardam ou conduzem energia.
fn is_power_block(block: &str) -> bool {
    matches!(block, GENERATOR | CABLE | SOLAR | BATTERY | GEOTHERMAL)
}

fn capacity(block: &str) -> i32 {
    match block {
        BATTERY => 30000,
        TESLA => defense::CAPACITY,
        _ => power::tier(block).map_or(2000, |tier| tier.capacity),
    }
}

/// Slots, entradas automáticas e saídas automáticas de cada máquina.
fn layout(block: &str) -> Option<(usize, Vec<usize>, Vec<usize>)> {
    Some(match block {
        DRILL | DRILL_MK2 => (1, vec![], vec![0]),
        PRESS | ASSEMBLER | CUTTER | CRUSHER => (2, vec![0], vec![1]),
        INTEGRATOR | ALLOY | ELECTRONICS => (3, vec![0, 1], vec![2]),
        BELT | FAST_BELT | SPLITTER => (1, vec![0], vec![0]),
        GENERATOR => (1, vec![0], vec![]),
        GEOTHERMAL => (2, vec![0], vec![1]),
        SOLAR | BATTERY | TESLA => (1, vec![], vec![]),
        CABLE | POLE | STEEL_POLE => (0, vec![], vec![]),
        HUB => (4, vec![0, 1], vec![2, 3]),
        HARVESTER => (4, vec![0], vec![1, 2, 3]),
        _ => return None,
    })
}

/// Slots que o bocal da máquina esvazia.
fn output_slots(block: &str) -> std::ops::Range<usize> {
    match block {
        PRESS | ASSEMBLER | CUTTER | CRUSHER | GEOTHERMAL => 1..2,
        INTEGRATOR | ALLOY | ELECTRONICS => 2..3,
        HUB => 2..4,
        HARVESTER => 1..4,
        GENERATOR | SOLAR | BATTERY | TESLA | CABLE | POLE | STEEL_POLE => 0..0,
        _ => 0..1,
    }
}

/// Jazidas vanilla e o que a extratora entrega. `true` exige a Mk.II.
fn ore_product(block: &str) -> Option<(&'static str, bool)> {
    Some(match block {
        "minecraft:copper_ore" | "minecraft:deepslate_copper_ore" => (COPPER, false),
        "minecraft:iron_ore" | "minecraft:deepslate_iron_ore" => (IRON, false),
        "minecraft:gold_ore" | "minecraft:deepslate_gold_ore" => ("minecraft:raw_gold", false),
        "minecraft:coal_ore" | "minecraft:deepslate_coal_ore" => ("minecraft:coal", false),
        "minecraft:redstone_ore" | "minecraft:deepslate_redstone_ore" => ("minecraft:redstone", false),
        "minecraft:lapis_ore" | "minecraft:deepslate_lapis_ore" => ("minecraft:lapis_lazuli", false),
        "minecraft:nether_quartz_ore" => ("minecraft:quartz", false),
        "minecraft:nether_gold_ore" => ("minecraft:gold_nugget", false),
        "minecraft:diamond_ore" | "minecraft:deepslate_diamond_ore" => ("minecraft:diamond", true),
        "minecraft:emerald_ore" | "minecraft:deepslate_emerald_ore" => ("minecraft:emerald", true),
        _ => return None,
    })
}

/// Pedras preciosas saem 20 vezes mais devagar que minérios comuns.
const GEM_SLOWDOWN: i32 = 20;

/// Combustíveis vanilla do gerador, em ticks de queima da fornalha vanilla.
/// O gerador queima quatro vezes mais rápido e entrega 40 FE/t: 10 FE por tick vanilla.
fn fuel_ticks(stack: &ItemStack) -> Option<i32> {
    if stack.damage != 0 || stack.extra.is_some() {
        return None;
    }
    let item = stack.item.as_ref();
    let name = item.strip_prefix("minecraft:")?;
    let nether_wood = name.contains("crimson") || name.contains("warped");
    Some(match name {
        "coal" | "charcoal" => 1600,
        "coal_block" => 16000,
        "blaze_rod" => 2400,
        "dried_kelp_block" => 4000,
        "stick" => 100,
        "bamboo_block" | "stripped_bamboo_block" | "bamboo_planks" => 300,
        _ if !nether_wood
            && (name.ends_with("_log") || name.ends_with("_wood") || name.ends_with("_planks")) => 300,
        _ => return None,
    })
}

const GENERATOR_RATE: i32 = 40;
const FE_PER_VANILLA_TICK: i32 = 10;

fn is_fuel(stack: &ItemStack) -> bool {
    is_plain(stack, "minecraft:coal") || is_plain(stack, "minecraft:charcoal")
}

/// Receitas do triturador: entrada, produto e quantidade.
const CRUSHER_RECIPES: [(&str, &str, u32); 4] = [
    ("minecraft:quartz", SILICON, 1),
    ("minecraft:bone", BONE_MEAL, 4),
    ("minecraft:cobblestone", "minecraft:gravel", 1),
    ("minecraft:gravel", "minecraft:sand", 1),
];

fn accepts(block: &str, slot: usize, stack: &ItemStack) -> bool {
    match (block, slot) {
        (HUB, _) => contracts::valid_input(slot, stack),
        (GENERATOR, 0) => fuel_ticks(stack).is_some(),
        (PRESS, 0) => [COPPER, IRON, "minecraft:raw_gold"].iter().any(|id| is_plain(stack, id)),
        (ASSEMBLER, 0) => is_plain(stack, PLATE),
        (CUTTER, 0) => is_plain(stack, IRON_PLATE) || is_plain(stack, PLATE),
        (CRUSHER, 0) => CRUSHER_RECIPES.iter().any(|(input, _, _)| is_plain(stack, input)),
        (ALLOY, 0) => is_plain(stack, IRON_PLATE),
        (ALLOY, 1) => is_fuel(stack),
        (ELECTRONICS, 0) => is_plain(stack, WIRE),
        (ELECTRONICS, 1) => is_plain(stack, SILICON),
        (GEOTHERMAL, 0) => is_plain(stack, "minecraft:lava_bucket"),
        (INTEGRATOR, 0) => is_plain(stack, FRAME),
        (INTEGRATOR, 1) => is_plain(stack, GEAR),
        (HARVESTER, 0) => is_plain(stack, BONE_MEAL),
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
    if number(entity, key) != value || !entity.nbt.contains_key(&key.into()) {
        entity.nbt.insert(key.into(), Tag::Int(value));
    }
}

fn visual_signature(entity: &ModBlockEntity) -> (i32, i32, Option<ItemStack>) {
    (number(entity, "Status"), number(entity, "Module"), entity.items.first().cloned().flatten())
}

fn center([x, y, z]: [i32; 3]) -> [f64; 3] {
    [f64::from(x) + 0.5, f64::from(y) + 0.5, f64::from(z) + 0.5]
}

fn stamp(time: i64) -> i32 {
    (time % i64::from(i32::MAX)) as i32
}

/// Ingredientes, produto com quantidade, duração em ticks e FE por tick.
type Recipe<'a> = (&'a [(&'a str, u32)], (&'a str, u32), i32, i32);

fn tick_factory(host: &mut dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity, time: i64) {
    let [x, y, z] = pos;
    logistics::transfer_outputs(host, pos, entity, time);
    if entity.id == HUB {
        contracts::tick(entity);
        return;
    }
    if power::tier(&entity.id).is_some() {
        return;
    }
    if pollution::emission(&entity.id) > 0.0 && pollution::wear(host, pos, entity) {
        set_number(entity, "Status", 13);
        return;
    }
    if logistics::update_status(entity, time) {
        return;
    }
    if is_power_block(&entity.id) {
        tick_power(host, pos, entity);
        return;
    }
    if needs_power(&entity.id) && host.neighbor_signal(host.dimension(), pos) > 0 {
        set_number(entity, "Status", 9);
        return;
    }
    match entity.id.as_str() {
        HARVESTER => return farm::tick(host, pos, entity, time),
        TESLA => {
            if time.rem_euclid(20) == 0 {
                set_number(entity, "LocalSmog", pollution::local(host.dimension(), pos) as i32);
            }
            return defense::tick(host, pos, entity);
        }
        _ => {}
    }
    let output_slot = output_slots(&entity.id).start;
    let (recipe, (output, amount), duration, draw): Recipe<'_> = match entity.id.as_str() {
        DRILL | DRILL_MK2 => {
            let advanced = entity.id == DRILL_MK2;
            let Some((ore, gem)) = ore_product(&host.block([x, y - 1, z])) else {
                set_number(entity, "Status", 4);
                return;
            };
            if gem && !advanced {
                set_number(entity, "Status", 12);
                return;
            }
            let base = if advanced { 20 } else { 40 };
            (&[], (ore, 1), if gem { base * GEM_SLOWDOWN } else { base }, if advanced { 12 } else { 6 })
        }
        PRESS => {
            if entity.items[0].as_ref().is_some_and(|s| is_plain(s, IRON)) {
                (&[(IRON, 1)], (IRON_PLATE, 1), 30, 8)
            } else if entity.items[0].as_ref().is_some_and(|s| is_plain(s, "minecraft:raw_gold")) {
                (&[("minecraft:raw_gold", 1)], ("minecraft:gold_ingot", 1), 40, 10)
            } else {
                (&[(COPPER, 1)], (PLATE, 1), 30, 8)
            }
        }
        ASSEMBLER => (&[(PLATE, 2)], (GEAR, 1), 40, 12),
        CUTTER => {
            if entity.items[0].as_ref().is_some_and(|s| is_plain(s, PLATE)) {
                (&[(PLATE, 1)], (WIRE, 1), 20, 6)
            } else {
                (&[(IRON_PLATE, 1)], (FRAME, 1), 30, 8)
            }
        }
        CRUSHER => {
            let &(input, product, count) = CRUSHER_RECIPES.iter()
                .find(|(input, _, _)| entity.items[0].as_ref().is_some_and(|s| is_plain(s, input)))
                .unwrap_or(&CRUSHER_RECIPES[0]);
            let recipe: &[(&str, u32)] = match input {
                "minecraft:bone" => &[("minecraft:bone", 1)],
                "minecraft:cobblestone" => &[("minecraft:cobblestone", 1)],
                "minecraft:gravel" => &[("minecraft:gravel", 1)],
                _ => &[("minecraft:quartz", 1)],
            };
            (recipe, (product, count), if input == "minecraft:quartz" { 50 } else { 30 }, 10)
        }
        ALLOY => {
            let fuel = if entity.items[1].as_ref().is_some_and(|s| is_plain(s, "minecraft:charcoal")) {
                "minecraft:charcoal"
            } else { "minecraft:coal" };
            (&[(IRON_PLATE, 2), (fuel, 1)], (STEEL, 1), 120, 16)
        }
        ELECTRONICS => (&[(WIRE, 2), (SILICON, 1)], (CIRCUIT, 1), 60, 12),
        INTEGRATOR => (&[(FRAME, 1), (GEAR, 1)], (MODULE, 1), 60, 16),
        _ => return,
    };
    let (duration, draw) = upgrades::apply(duration, draw, upgrades::effect(entity));
    set_number(entity, "Duration", duration);
    let recipe_key = "Recipe".into();
    let recipe_changed = !matches!(entity.nbt.get(&recipe_key), Some(Tag::String(old)) if old.to_string_lossy() == output);
    if recipe_changed {
        entity.nbt.insert(recipe_key, Tag::String(output.into()));
        set_number(entity, "Progress", 0);
    }
    if !can_add(&entity.items[output_slot], output, amount) {
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
            add(&mut entity.items[0], output, amount);
        } else {
            process(&mut entity.items, recipe, output, amount);
        }
        if upgrades::bonus_ready(entity) && can_add(&entity.items[output_slot], output, amount) {
            add(&mut entity.items[output_slot], output, amount);
        }
        let produced = number(entity, "Produced").saturating_add(1);
        set_number(entity, "Produced", produced);
        set_number(entity, "Total", number(entity, "Total").saturating_add(1));
        set_number(entity, "Progress", 0);
    } else {
        set_number(entity, "Progress", progress);
    }
}

fn tick_power(host: &mut dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity) {
    let limit = capacity(&entity.id);
    let mut energy = number(entity, "energy").min(limit);
    match entity.id.as_str() {
        GENERATOR if energy <= limit - GENERATOR_RATE => {
            let mut burn = number(entity, "Burn");
            if burn == 0
                && let Some(ticks) = entity.items[0].as_ref().and_then(fuel_ticks)
            {
                consume(&mut entity.items[0], 1);
                burn = ticks * FE_PER_VANILLA_TICK / GENERATOR_RATE;
                set_number(entity, "BurnTotal", burn);
            }
            if burn > 0 {
                energy += GENERATOR_RATE;
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
            let storage = matches!(other.id.as_str(), CABLE | BATTERY) || power::tier(&other.id).is_some();
            if !needs_power(&other.id) && !storage {
                return;
            }
            let other_limit = capacity(&other.id);
            let stored = number(other, "energy").min(other_limit);
            // Equalize charge percentages, so large batteries can charge and discharge.
            let offered = if storage {
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

fn is_plain(stack: &ItemStack, item: &str) -> bool {
    stack.item.as_ref() == item && stack.damage == 0 && stack.extra.is_none()
}

fn can_add(slot: &Option<ItemStack>, item: &str, count: u32) -> bool {
    slot.as_ref()
        .is_none_or(|stack| is_plain(stack, item) && stack.count + count <= 64)
}

fn add(slot: &mut Option<ItemStack>, item: &str, count: u32) {
    if let Some(stack) = slot {
        stack.count += count;
    } else {
        *slot = Some(ItemStack::new(item, count));
    }
}

fn consume(slot: &mut Option<ItemStack>, count: u32) {
    if let Some(stack) = slot {
        stack.count -= count.min(stack.count);
        if stack.count == 0 {
            *slot = None;
        }
    }
}

fn process(slots: &mut [Option<ItemStack>], recipe: &[(&str, u32)], output: &str, amount: u32) {
    if slots.len() <= recipe.len() {
        return;
    }
    let (inputs, outputs) = slots.split_at_mut(recipe.len());
    let output_slot = &mut outputs[0];
    if !can_add(output_slot, output, amount) || !ingredients_ready(inputs, recipe) {
        return;
    }
    for (slot, &(_, count)) in inputs.iter_mut().zip(recipe) {
        consume(slot, count);
    }
    add(output_slot, output, amount);
}

#[cfg(test)]
mod tests;
