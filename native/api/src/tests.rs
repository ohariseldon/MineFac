//! Testes de regras do Veio. Rodam com `python tools/check_rust.py`, sem abrir o jogo.
use super::*;
use horizon_client::mod_api::{BlockStateId, ParticleOptions};
use std::collections::HashMap;

fn entity(id: &str, slots: usize) -> ModBlockEntity {
    let mut entity = ModBlockEntity::default();
    entity.id = id.to_owned();
    entity.items = vec![None; slots];
    entity
}

#[test]
fn integrator_waits_for_both_inputs_and_preserves_them_when_output_is_full() {
    let mut slots = vec![Some(ItemStack::new(FRAME, 2)), None, None];
    process(&mut slots, &[(FRAME, 1), (GEAR, 1)], MODULE, 1);
    assert_eq!(slots[0].as_ref().unwrap().count, 2);
    assert!(slots[2].is_none());
    slots[1] = Some(ItemStack::new(GEAR, 3));
    slots[2] = Some(ItemStack::new(MODULE, 64));
    let blocked = slots.clone();
    process(&mut slots, &[(FRAME, 1), (GEAR, 1)], MODULE, 1);
    assert_eq!(slots, blocked);
    slots[2] = None;
    process(&mut slots, &[(FRAME, 1), (GEAR, 1)], MODULE, 1);
    assert_eq!(slots[0].as_ref().unwrap().count, 1);
    assert_eq!(slots[1].as_ref().unwrap().count, 2);
    assert_eq!(slots[2], Some(ItemStack::new(MODULE, 1)));
}

#[test]
fn crusher_keeps_multi_item_output_within_one_stack() {
    let mut slots = vec![Some(ItemStack::new("minecraft:bone", 5)), Some(ItemStack::new(BONE_MEAL, 61))];
    process(&mut slots, &[("minecraft:bone", 1)], BONE_MEAL, 4);
    assert_eq!(slots[0].as_ref().unwrap().count, 5, "4 farinhas não cabem em 61 de 64");
    slots[1] = Some(ItemStack::new(BONE_MEAL, 60));
    process(&mut slots, &[("minecraft:bone", 1)], BONE_MEAL, 4);
    assert_eq!(slots[1].as_ref().unwrap().count, 64);
    assert!(accepts(CRUSHER, 0, &ItemStack::new("minecraft:cobblestone", 1)));
    assert!(!accepts(CRUSHER, 0, &ItemStack::new("minecraft:stone", 1)));
}

#[test]
fn menus_fence_outputs_and_route_inputs() {
    assert!(accepts(INTEGRATOR, 0, &ItemStack::new(FRAME, 1)));
    assert!(!accepts(INTEGRATOR, 0, &ItemStack::new(GEAR, 1)));
    assert!(accepts(INTEGRATOR, 1, &ItemStack::new(GEAR, 1)));
    assert!(!accepts(INTEGRATOR, 2, &ItemStack::new(MODULE, 1)));
    assert!(!accepts(DRILL, 0, &ItemStack::new(COPPER, 1)));
    assert!(accepts(GENERATOR, 0, &ItemStack::new("minecraft:charcoal", 1)));
    assert!(!accepts(GENERATOR, 0, &ItemStack::new("minecraft:stone", 1)));
    assert!(accepts(HARVESTER, 0, &ItemStack::new(BONE_MEAL, 1)));
    assert!(!accepts(HARVESTER, 1, &ItemStack::new("minecraft:wheat", 1)), "saídas só para retirada");
    assert!(!accepts(TESLA, 0, &ItemStack::new("minecraft:arrow", 1)));
}

#[test]
fn generator_burns_vanilla_fuels_in_proportion_to_furnace_time() {
    let fuel = |item: &str| fuel_ticks(&ItemStack::new(item, 1));
    assert_eq!(fuel("minecraft:coal"), Some(1600));
    assert_eq!(fuel("minecraft:oak_log"), Some(300));
    assert_eq!(fuel("minecraft:stripped_birch_wood"), Some(300));
    assert_eq!(fuel("minecraft:cherry_planks"), Some(300));
    assert_eq!(fuel("minecraft:coal_block"), Some(16000));
    assert_eq!(fuel("minecraft:crimson_planks"), None, "madeira do Nether não queima");
    assert_eq!(fuel("minecraft:warped_stem"), None);
    assert_eq!(fuel("minecraft:cobblestone"), None);
    // Carvão rende os mesmos 16.000 FE da versão 0.6.
    assert_eq!(1600 * FE_PER_VANILLA_TICK, 16000);
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
    let mut generator = entity(GENERATOR, 1);
    generator.items = vec![Some(ItemStack::new("minecraft:coal", 2))];
    let mut grid = Grid { cable: entity(CABLE, 0) };
    tick_power(&mut grid, [0, 0, 0], &mut generator);
    assert_eq!(number(&generator, "energy") + number(&grid.cable, "energy"), 40);
    assert_eq!(generator.items[0].as_ref().unwrap().count, 1);
    assert_eq!(number(&generator, "BurnTotal"), 400);
    let remaining_burn = number(&generator, "Burn");
    set_number(&mut generator, "energy", 2000);
    set_number(&mut grid.cable, "energy", 2000);
    tick_power(&mut grid, [0, 0, 0], &mut generator);
    assert_eq!(number(&generator, "Burn"), remaining_burn);
    assert_eq!(generator.items[0].as_ref().unwrap().count, 1);
    assert_eq!(number(&generator, "energy") + number(&grid.cable, "energy"), 4000);
}

#[test]
fn gems_need_the_advanced_extractor() {
    assert_eq!(ore_product("minecraft:deepslate_diamond_ore"), Some(("minecraft:diamond", true)));
    assert_eq!(ore_product("minecraft:lapis_ore"), Some(("minecraft:lapis_lazuli", false)));
    assert_eq!(ore_product("minecraft:stone"), None);
}

/// Um pedaço de mundo com plantações: blocos e a propriedade `age`.
#[derive(Default)]
struct Field {
    blocks: HashMap<[i32; 3], (String, u8)>,
    particles: usize,
}

impl Host for Field {
    fn block(&self, pos: [i32; 3]) -> String {
        self.blocks.get(&pos).map_or_else(|| "minecraft:air".to_owned(), |(id, _)| id.clone())
    }
    fn block_property(&self, pos: [i32; 3], key: &str) -> Option<String> {
        (key == "age").then(|| self.blocks.get(&pos).map(|(_, age)| age.to_string())).flatten()
    }
    fn set_block_property(&mut self, pos: [i32; 3], key: &str, value: &str) -> bool {
        let Some((_, age)) = self.blocks.get_mut(&pos) else { return false };
        let value: u8 = value.parse().unwrap();
        if key != "age" || *age == value { return false; }
        *age = value;
        true
    }
    fn send_particles(&mut self, _: &ParticleOptions, _: Option<BlockStateId>, _: [f64; 3], _: [f64; 3], _: f64, _: i32) -> bool {
        self.particles += 1;
        true
    }
    fn play_sound(&mut self, _: &str, _: [f64; 3], _: usize, _: f32, _: f32) -> bool {
        true
    }
}

fn sweep(field: &mut Field, harvester: &mut ModBlockEntity) {
    for time in 0..(farm::AREA as i64) {
        farm::tick(field, [0, 64, 0], harvester, time);
    }
}

#[test]
fn harvester_collects_ripe_crops_and_replants_them() {
    let mut field = Field::default();
    field.blocks.insert([2, 64, 1], ("minecraft:wheat".into(), 7));
    field.blocks.insert([-3, 64, 4], ("minecraft:carrots".into(), 7));
    field.blocks.insert([1, 63, 0], ("minecraft:potatoes".into(), 3));
    field.blocks.insert([9, 64, 0], ("minecraft:wheat".into(), 7));
    let mut harvester = entity(HARVESTER, 4);
    set_number(&mut harvester, "energy", 2000);
    sweep(&mut field, &mut harvester);
    assert_eq!(field.blocks[&[2, 64, 1]].1, 0, "trigo maduro volta a ser broto");
    assert_eq!(field.blocks[&[-3, 64, 4]].1, 0);
    assert_eq!(field.blocks[&[1, 63, 0]].1, 3, "sem farinha de osso, a batata verde fica");
    assert_eq!(field.blocks[&[9, 64, 0]].1, 7, "fora do alcance de 4 blocos");
    let mut found: Vec<(String, u32)> = harvester.items[1..4].iter().flatten()
        .map(|stack| (stack.item.to_string(), stack.count)).collect();
    found.sort();
    assert_eq!(found, vec![
        ("minecraft:carrot".into(), 2), ("minecraft:wheat".into(), 1), ("minecraft:wheat_seeds".into(), 1),
    ]);
    assert_eq!(number(&harvester, "energy"), 2000 - 2 * farm::HARVEST_COST);
}

#[test]
fn harvester_uses_bone_meal_and_waits_for_room() {
    let mut field = Field::default();
    field.blocks.insert([1, 64, 1], ("minecraft:beetroots".into(), 0));
    let mut harvester = entity(HARVESTER, 4);
    harvester.items[0] = Some(ItemStack::new(BONE_MEAL, 1));
    set_number(&mut harvester, "energy", 2000);
    sweep(&mut field, &mut harvester);
    assert_eq!(field.blocks[&[1, 64, 1]].1, 2);
    assert!(harvester.items[0].is_none());
    field.blocks.insert([1, 64, 1], ("minecraft:beetroots".into(), 3));
    for slot in 1..4 {
        harvester.items[slot] = Some(ItemStack::new("minecraft:cobblestone", 64));
    }
    sweep(&mut field, &mut harvester);
    assert_eq!(field.blocks[&[1, 64, 1]].1, 3, "sem espaço, não colhe nem perde produto");
    assert_eq!(number(&harvester, "Status"), 3);
}

#[test]
fn harvester_store_merges_stacks_before_using_empty_slots() {
    let mut slots = vec![Some(ItemStack::new("minecraft:carrot", 63)), None, None];
    assert!(farm::store(&mut slots, "minecraft:carrot", 2));
    assert_eq!(slots[0].as_ref().unwrap().count, 64);
    assert_eq!(slots[1], Some(ItemStack::new("minecraft:carrot", 1)));
    assert!(!farm::drops_fit(&[Some(ItemStack::new("minecraft:stone", 64))], &[("minecraft:carrot", 1)]));
}

#[test]
fn tesla_picks_the_closest_hostile_in_range() {
    let positions: HashMap<u32, [f64; 3]> = HashMap::from([
        (1, [8.0, 64.0, 0.0]),
        (2, [3.0, 64.0, 2.0]),
        (3, [30.0, 64.0, 0.0]),
    ]);
    let mut gone = Vec::new();
    let target = defense::nearest([0.0, 64.0, 0.0], &[1, 2, 3, 4], |id| positions.get(&id).copied(), &mut gone);
    assert_eq!(target.map(|(id, _)| id), Some(2));
    assert_eq!(gone, vec![4], "mob que não existe mais é esquecido");
    let far = defense::nearest([0.0, 64.0, 0.0], &[3], |id| positions.get(&id).copied(), &mut gone);
    assert!(far.is_none());
    assert!(defense::hostile("minecraft:zombie"));
    assert!(!defense::hostile("minecraft:villager"));
    assert!(!defense::hostile("minecraft:wither"), "chefes ficam de fora");
}

#[test]
fn contract_hub_keeps_exporting_after_the_last_project() {
    let mut hub = entity(HUB, 4);
    set_number(&mut hub, "ContractStage", contracts::CONTRACTS.len() as i32);
    hub.items[0] = Some(ItemStack::new(CIRCUIT, 8));
    hub.items[1] = Some(ItemStack::new(STEEL, 8));
    contracts::tick(&mut hub);
    contracts::tick(&mut hub);
    assert_eq!(number(&hub, "ContractStage") as usize, contracts::CONTRACTS.len());
    assert_eq!(number(&hub, "Total"), 2);
    assert_eq!(hub.items[2], Some(ItemStack::new("minecraft:emerald", 8)));
    assert!(hub.items[0].is_none() && hub.items[1].is_none());
}
