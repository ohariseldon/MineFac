//! Testes da produção: receitas, pureza, geração, lavouras, silvicultura, rancho e armazém.
use super::*;
use horizon_client::mod_api::{BlockStateId, NativeBlockEdit, ParticleOptions};
use std::collections::HashMap;

#[test]
fn unpowered_field_machines_do_no_world_scan_and_resume_when_funded() {
    use std::cell::Cell;
    struct Probe { reads: Cell<usize> }
    impl Host for Probe {
        fn block(&self, _: [i32; 3]) -> String {
            self.reads.set(self.reads.get() + 1);
            "minecraft:air".into()
        }
    }
    for id in [HARVESTER, PLANTER, TREE_FARM] {
        let mut host = Probe { reads: Cell::new(0) };
        let mut entity = machine(id);
        if id == PLANTER { entity.items[0] = Some(ItemStack::new("minecraft:wheat_seeds", 8)); }
        set_number(&mut entity, "energy", 0);
        for time in 0..120 { tick(&mut host, [0; 3], &mut entity, time); }
        assert_eq!(host.reads.get(), 0, "{id}");
        assert_eq!(number(&entity, "Scan"), 0);
        assert_eq!(number(&entity, "Status"), 5);
        set_number(&mut entity, "energy", 2000);
        tick(&mut host, [0; 3], &mut entity, 120);
        assert!(host.reads.get() > 0, "{id} must wake immediately on its next work phase");
    }
}

fn machine(id: &str) -> ModBlockEntity {
    let mut entity = ModBlockEntity::default();
    entity.id = id.to_owned();
    entity.items = vec![None; super::super::layout(id).map_or(0, |(slots, _, _)| slots)];
    set_number(&mut entity, "energy", 2000);
    entity
}

fn run(entity: &mut ModBlockEntity, ticks: usize) {
    for _ in 0..ticks {
        set_number(entity, "energy", 2000);
        process(entity);
    }
}

/// Bloco com suas propriedades (`age`, `level`...).
type Placed = (String, Vec<(String, String)>);

/// Um pedaço de mundo: blocos com propriedades e as edições em lote recebidas.
#[derive(Default)]
struct World {
    blocks: HashMap<[i32; 3], Placed>,
    edits: Vec<NativeBlockEdit>,
    sky: bool,
}

impl World {
    fn put(&mut self, pos: [i32; 3], block: &str) {
        self.blocks.insert(pos, (block.to_owned(), Vec::new()));
    }
    fn put_with(&mut self, pos: [i32; 3], block: &str, key: &str, value: &str) {
        self.blocks.insert(pos, (block.to_owned(), vec![(key.to_owned(), value.to_owned())]));
    }
}

impl Host for World {
    fn dimension(&self) -> &'static str {
        "veio:test_production"
    }
    fn block(&self, pos: [i32; 3]) -> String {
        self.blocks.get(&pos).map_or_else(|| "minecraft:air".to_owned(), |(id, _)| id.clone())
    }
    fn block_property(&self, pos: [i32; 3], key: &str) -> Option<String> {
        self.blocks.get(&pos)?.1.iter().find(|(k, _)| k == key).map(|(_, v)| v.clone())
    }
    fn set_block_property(&mut self, pos: [i32; 3], key: &str, value: &str) -> bool {
        let Some((_, props)) = self.blocks.get_mut(&pos) else { return false };
        let Some(entry) = props.iter_mut().find(|(k, _)| k == key) else { return false };
        if entry.1 == value { return false; }
        entry.1 = value.to_owned();
        true
    }
    fn edit_blocks(&mut self, _: &str, edit: &NativeBlockEdit) -> Result<u64, String> {
        self.edits.push(edit.clone());
        Ok(edit.runs.len() as u64)
    }
    fn light(&self, _: [i32; 3]) -> (u8, u8) {
        (0, if self.sky { 15 } else { 0 })
    }
    fn weather(&self) -> (f32, f32) {
        (0.0, 0.0)
    }
    fn send_particles(&mut self, _: &ParticleOptions, _: Option<BlockStateId>, _: [f64; 3], _: [f64; 3], _: f64, _: i32) -> bool {
        true
    }
    fn play_sound(&mut self, _: &str, _: [f64; 3], _: usize, _: f32, _: f32) -> bool {
        true
    }
}

#[test]
fn kitchen_prefers_the_recipe_that_uses_every_input() {
    let mut kitchen = machine(KITCHEN);
    kitchen.items[0] = Some(ItemStack::new(FLOUR, 4));
    kitchen.items[1] = Some(ItemStack::new("minecraft:cocoa_beans", 1));
    run(&mut kitchen, 80);
    assert_eq!(kitchen.items[2], Some(ItemStack::new("minecraft:cookie", 8)));
    // Sem cacau, a mesma farinha vira pão.
    run(&mut kitchen, 60);
    assert_eq!(kitchen.items[2], Some(ItemStack::new("minecraft:cookie", 8)), "saída ocupada por biscoitos");
    kitchen.items[2] = None;
    run(&mut kitchen, 60);
    assert_eq!(kitchen.items[2], Some(ItemStack::new("minecraft:bread", 1)));
    assert!(kitchen.items[0].is_none());
}

#[test]
fn alternate_recipes_open_routes_without_the_nether() {
    let mut alloy = machine(ALLOY);
    alloy.items[0] = Some(ItemStack::new("minecraft:sand", 4));
    alloy.items[1] = Some(ItemStack::new("minecraft:charcoal", 1));
    run(&mut alloy, 160);
    assert_eq!(alloy.items[2], Some(ItemStack::new("veio:silicon", 1)));
    assert!(data::RECIPES.iter().any(|recipe| recipe.alt && recipe.output.0 == CIRCUIT));
}

#[test]
fn electric_furnace_turns_any_log_into_charcoal() {
    let mut furnace = machine(ELECTRIC_FURNACE);
    furnace.items[0] = Some(ItemStack::new("minecraft:cherry_log", 2));
    run(&mut furnace, 160);
    assert_eq!(furnace.items[1], Some(ItemStack::new("minecraft:charcoal", 2)));
    assert!(accepts(ELECTRIC_FURNACE, 0, &ItemStack::new("minecraft:beef", 1)));
    assert!(!accepts(ELECTRIC_FURNACE, 0, &ItemStack::new("minecraft:dirt", 1)));
    assert!(!accepts(ELECTRIC_FURNACE, 1, &ItemStack::new("minecraft:beef", 1)), "saída só para retirada");
}

#[test]
fn every_recipe_machine_and_item_is_known() {
    for recipe in data::RECIPES {
        assert!(super::super::layout(recipe.machine).is_some(), "{}", recipe.machine);
        assert!(recipe.inputs.len() <= 2 && recipe.ticks > 0 && recipe.power > 0);
    }
}

#[test]
fn purity_rewards_bigger_veins() {
    assert_eq!(extraction::purity(1), 1);
    assert_eq!(extraction::purity(5), 2);
    assert_eq!(extraction::purity(9), 3);
    assert_eq!(extraction::duration(40, 1, 1), 80);
    assert_eq!(extraction::duration(40, 1, 3), 20);
    assert_eq!(extraction::duration(20, 20, 2), 400, "diamante na Mk.II: 20 segundos");
}

#[test]
fn drill_reads_the_vein_and_respects_tiers() {
    let mut world = World::default();
    for dx in -1..=1 {
        for dz in -1..=1 {
            world.put([dx, 63, dz], "minecraft:copper_ore");
        }
    }
    let mut drill = machine(DRILL);
    for time in 0..40 {
        set_number(&mut drill, "energy", 2000);
        extraction::tick(&mut world, [0, 64, 0], &mut drill, time);
    }
    assert_eq!(number(&drill, "Info"), 3, "veio cheio: jazida pura");
    assert_eq!(drill.items[0], Some(ItemStack::new("minecraft:raw_copper", 2)), "pura: 20 ticks por minério");
    world.put([5, 63, 5], "minecraft:diamond_ore");
    let mut basic = machine(DRILL);
    extraction::tick(&mut world, [5, 64, 5], &mut basic, 0);
    assert_eq!(number(&basic, "Status"), 12);
}

#[test]
fn renewable_power_covers_a_starter_line_without_replacing_steam() {
    assert_eq!(generation::burner(BIOMASS_GENERATOR).unwrap().0, 40);
    assert_eq!(generation::burner(GENERATOR).unwrap().0, 40);
    assert_eq!(emission(BIOMASS_GENERATOR), 2.0);
    assert_eq!(super::super::pollution::emission(GENERATOR), 6.0);
    let line = 2 * 6 + 2 * 8;
    let two_wheels = 2 * 2 * generation::WATER_FACE;
    assert!(two_wheels >= line, "2 rodas com 2 faces cada cobrem 2 extratoras + 2 prensas: {two_wheels} >= {line}");
    assert!(generation::WIND_FLOOR * 4 >= line, "4 cata-ventos no nível do mar cobrem a mesma linha");
    assert!(two_wheels < 120, "água e vento não substituem a caldeira");
}

#[test]
fn wind_grows_with_altitude_and_needs_open_sky() {
    assert_eq!(generation::wind_output(64, true, 0.0, 0.0, 1.0), generation::WIND_FLOOR);
    assert_eq!(generation::wind_output(110, true, 0.0, 0.0, 1.0), 16);
    assert_eq!(generation::wind_output(110, false, 0.0, 0.0, 1.0), 0);
    assert!(generation::wind_output(110, true, 1.0, 1.0, 1.0) > 16, "tempestade ajuda");
    assert_eq!(generation::wind_output(300, true, 1.0, 1.0, 1.2), 32, "teto");
}

#[test]
fn water_wheel_needs_flowing_water_and_steam_needs_still_water() {
    let mut world = World::default();
    world.put_with([1, 64, 0], "minecraft:water", "level", "2");
    world.put_with([-1, 64, 0], "minecraft:water", "level", "0");
    let mut wheel = machine(WATER_WHEEL);
    let energy = generation::produce(&mut world, [0, 64, 0], &mut wheel, 0, 2000);
    assert_eq!(energy, generation::WATER_FACE, "uma face com água corrente");
    let mut steam = machine(STEAM_ENGINE);
    steam.items[0] = Some(ItemStack::new("minecraft:coal", 1));
    let energy = generation::produce(&mut world, [0, 64, 0], &mut steam, 0, 2000);
    assert_eq!(energy, 120);
    assert_eq!(number(&steam, "BurnTotal"), 1600 * 20 / 120, "o dobro de energia por carvão");
    let mut dry = World::default();
    let mut idle = machine(STEAM_ENGINE);
    idle.items[0] = Some(ItemStack::new("minecraft:coal", 1));
    assert_eq!(generation::produce(&mut dry, [0, 64, 0], &mut idle, 0, 2000), 0);
    assert_eq!(generation::empty_status(&idle), NO_WATER);
    assert_eq!(idle.items[0].as_ref().unwrap().count, 1, "sem água, não queima o carvão");
}

#[test]
fn planter_queues_seeds_on_empty_farmland_and_flushes_in_one_edit() {
    let mut world = World::default();
    world.put([1, 63, 1], "minecraft:farmland");
    world.put([2, 63, 1], "minecraft:farmland");
    world.put([3, 63, 1], "minecraft:farmland");
    world.put([3, 64, 1], "minecraft:wheat");
    let mut planter = machine(PLANTER);
    planter.items[0] = Some(ItemStack::new("minecraft:wheat_seeds", 10));
    for time in 1..=farm::AREA as i64 / 3 + 1 {
        crops::plant(&mut world, [0, 64, 0], &mut planter, time);
    }
    assert_eq!(planter.items[0].as_ref().unwrap().count, 8, "duas vagas livres");
    crops::flush(&mut world, [0, 64, 0], &mut planter, 0);
    let runs: Vec<_> = world.edits.iter().flat_map(|edit| edit.runs.iter().map(|run| (run.x, run.y, run.z))).collect();
    assert!(runs.contains(&(1, 64, 1)) && runs.contains(&(2, 64, 1)));
    assert!(world.edits.len() <= 2, "edições agrupadas: {}", world.edits.len());
}

#[test]
fn harvester_cuts_tall_plants_above_the_base_and_attached_fruit_only() {
    let mut world = World::default();
    for dy in 0..3 {
        world.put([1, 64 + dy, 1], "minecraft:sugar_cane");
    }
    world.put([-2, 64, 0], "minecraft:melon");
    world.put([-3, 64, 0], "minecraft:attached_melon_stem");
    world.put([2, 64, -2], "minecraft:pumpkin");
    let mut harvester = machine(HARVESTER);
    for time in 0..farm::AREA as i64 / 3 + 1 {
        set_number(&mut harvester, "energy", 2000);
        farm::tick(&mut world, [0, 64, 0], &mut harvester, time);
    }
    let mut found: Vec<(String, u32)> = harvester.items[1..4].iter().flatten()
        .map(|stack| (stack.item.to_string(), stack.count)).collect();
    found.sort();
    assert_eq!(found, vec![("minecraft:melon_slice".into(), 5), ("minecraft:sugar_cane".into(), 2)]);
    crops::flush(&mut world, [0, 64, 0], &mut harvester, 0);
    let cut = world.edits.iter().flat_map(|edit| edit.runs.clone()).find(|run| (run.x, run.z) == (1, 1)).unwrap();
    assert_eq!((cut.y, cut.height), (65, 2), "a base da cana fica");
}

#[test]
fn tree_farm_fells_grown_trees_and_replants_the_same_species() {
    let mut world = World::default();
    let base = [3, 64, 0];
    world.put([3, 63, 0], "minecraft:grass_block");
    for dy in 0..5 {
        world.put([base[0], base[1] + dy, base[2]], "minecraft:birch_log");
    }
    let mut farm = machine(TREE_FARM);
    for time in 0..40 {
        set_number(&mut farm, "energy", 2000);
        forestry::tick(&mut world, [0, 64, 0], &mut farm, time);
    }
    let edit = world.edits.first().expect("uma edição por árvore");
    assert_eq!(edit.palette, vec!["minecraft:air".to_owned(), "minecraft:birch_sapling".to_owned()]);
    assert_eq!(edit.filters, vec!["#minecraft:leaves".to_owned()]);
    assert_eq!((edit.runs[0].y, edit.runs[0].block), (64, 1), "a base vira muda");
    assert_eq!((edit.runs[1].y, edit.runs[1].height), (65, 4));
    let logs: u32 = farm.items[1..4].iter().flatten().filter(|stack| &*stack.item == "minecraft:birch_log").map(|stack| stack.count).sum();
    assert_eq!(logs, 5);
    assert_eq!(world.edits.len(), 1, "a árvore some do mundo: não há segundo corte do mesmo tronco no teste");
}

fn animal(id: u32, kind: u8) -> ranch::Animal {
    ranch::Animal { id, kind, adult: true, in_love: false, sheared: false, color: 0, named: false }
}

#[test]
fn ranch_breeds_shears_milks_and_keeps_a_population_cap() {
    let cows: Vec<_> = (0..3).map(|id| animal(id, 0)).collect();
    let plan = ranch::plan(&cows, Some(("minecraft:wheat", 4)), true);
    assert_eq!(plan.breed, vec![0, 1]);
    assert!(plan.milk);
    assert_eq!(plan.cull, None);
    let sheep: Vec<_> = (10..12).map(|id| ranch::Animal { color: 14, ..animal(id, 1) }).collect();
    assert_eq!(ranch::plan(&sheep, None, false).shear, vec![(10, 14), (11, 14)]);
    let crowd: Vec<_> = (0..12).map(|id| ranch::Animal { named: id == 0, ..animal(id, 2) }).collect();
    let plan = ranch::plan(&crowd, Some(("minecraft:carrot", 10)), false);
    assert!(plan.breed.is_empty(), "teto de população");
    assert_eq!(plan.cull, Some(1), "animal com nome não é abatido");
}

#[test]
fn pantry_pays_more_for_processed_food_and_villages() {
    assert!(pantry::credit("minecraft:bread", 1, false) > 2 * pantry::credit("minecraft:wheat", 1, false),
        "pão vale mais que o trigo usado");
    assert_eq!(pantry::credit("minecraft:bread", 2, true), 6 * 2 * 150);
    assert!(pantry::value("minecraft:stone").is_none());
}
