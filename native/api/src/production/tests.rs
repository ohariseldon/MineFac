//! Testes da produção: receitas, pureza, geração, lavouras, silvicultura, rancho e armazém.
use super::*;
use horizon_client::mod_api::{BlockStateId, NativeBlockEdit, ParticleOptions};
use std::collections::HashMap;

#[test]
fn exhausted_global_terrain_budget_preserves_inputs_energy_and_cursor_then_resumes() {
    use std::cell::Cell;
    struct Probe { reads: Cell<usize> }
    impl Host for Probe {
        fn dimension(&self) -> &'static str { "veio:terrain-budget-test" }
        fn block(&self, _: [i32; 3]) -> String {
            self.reads.set(self.reads.get() + 1);
            "minecraft:air".into()
        }
    }
    for id in [HARVESTER, PLANTER, TREE_FARM, GROW_LAMP] {
        let mut host = Probe { reads: Cell::new(0) };
        let mut entity = machine(id);
        if id == PLANTER { entity.items[0] = Some(ItemStack::new("minecraft:wheat_seeds", 8)); }
        budget::begin_tick(0);
        assert!(budget::request(budget::Lane::Terrain, host.dimension(), [999; 3], 384));
        tick(&mut host, [0; 3], &mut entity, 0);
        assert_eq!(host.reads.get(), 0, "{id}");
        assert_eq!(number(&entity, "energy"), 2000);
        assert_eq!(number(&entity, "Scan"), 0);
        if id == PLANTER { assert_eq!(entity.items[0].as_ref().unwrap().count, 8); }
        budget::begin_tick(1);
        tick(&mut host, [0; 3], &mut entity, 1);
        assert!(host.reads.get() > 0, "{id} must resume after admission");
    }
}

#[test]
fn unpowered_field_machines_do_no_world_scan_and_resume_when_funded() {
    use std::cell::Cell;
    struct Probe { reads: Cell<usize> }
    impl Host for Probe {
        fn dimension(&self) -> &'static str { "veio:budget-fixture" }
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
    fn day_time(&self) -> i64 {
        // Fim da primavera, meio-dia: uns 20 °C, sem geada nem calor extra.
        4 * 24000 + 6000
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
        let (_, inputs, outputs) = super::super::layout(recipe.machine).unwrap();
        assert!(recipe.inputs.len() <= inputs.len() && recipe.ticks > 0 && recipe.power > 0, "{}", recipe.id);
        assert!(recipe.byproduct.is_none() || outputs.len() > 1, "{}: sem slot de subproduto", recipe.id);
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

#[test]
fn productivity_reserves_the_bonus_without_consuming_inputs_or_power_while_blocked() {
    let mut press = machine(PRESS);
    set_number(&mut press, "Module", 3);
    press.items[0] = Some(ItemStack::new("minecraft:raw_copper", 5));
    // Ciclo da receita com o módulo, lido da própria receita (o balanceamento muda os ticks).
    let recipe = find(PRESS, &press.items).unwrap();
    let cycle = upgrades::apply(recipe.ticks, recipe.power, upgrades::effect_of(3)).0 as usize;
    run(&mut press, 4 * cycle);
    assert_eq!(press.items[1], Some(ItemStack::new(PLATE, 4)));
    assert_eq!(number(&press, "Bonus"), 80);
    press.items[1] = Some(ItemStack::new(PLATE, 63));
    let before = press.items.clone();
    let energy = number(&press, "energy");
    process(&mut press);
    assert_eq!(number(&press, "Status"), 3);
    assert_eq!(number(&press, "energy"), energy);
    assert_eq!(press.items, before);
    assert_eq!(number(&press, "Bonus"), 80);
    assert_eq!(number(&press, "Progress"), 0);
    press.items[1].as_mut().unwrap().count = 62;
    run(&mut press, cycle);
    assert_eq!(press.items[1], Some(ItemStack::new(PLATE, 64)));
    assert!(press.items[0].is_none());
    assert_eq!(number(&press, "Produced"), 6);
    assert_eq!(number(&press, "Total"), 5);
    assert_eq!(number(&press, "Bonus"), 0);
}

#[test]
fn cheap_cycles_cannot_be_banked_for_a_different_expensive_product() {
    let mut furnace = machine(ELECTRIC_FURNACE);
    set_number(&mut furnace, "Module", 3);
    furnace.items[0] = Some(ItemStack::new("minecraft:oak_log", 4));
    run(&mut furnace, 4 * 92);
    assert_eq!(number(&furnace, "Bonus"), 80);
    furnace.items[1] = None;
    furnace.items[0] = Some(ItemStack::new("minecraft:ancient_debris", 1));
    run(&mut furnace, 460);
    assert_eq!(furnace.items[1], Some(ItemStack::new("minecraft:netherite_scrap", 1)));
    assert_eq!(number(&furnace, "Bonus"), 20);
    furnace.items[1] = None;
    furnace.items[0] = Some(ItemStack::new("minecraft:oak_log", 1));
    run(&mut furnace, 92);
    assert_eq!(furnace.items[1], Some(ItemStack::new("minecraft:charcoal", 1)));
    assert_eq!(number(&furnace, "Bonus"), 20);
}

#[test]
fn extraction_rewards_scale_across_tiers_and_preserve_the_vein() {
    for id in [DRILL, DRILL_MK2, DRILL_MK3] {
        for (blocks, purity) in [(1, 1), (4, 2), (9, 3)] {
            let mut world = World::default();
            // O bloco central deve estar presente também na jazida impura.
            let positions = [[0, 63, 0], [-1, 63, 0], [1, 63, 0], [0, 63, -1],
                [0, 63, 1], [-1, 63, -1], [-1, 63, 1], [1, 63, -1], [1, 63, 1]];
            for pos in positions.into_iter().take(blocks) { world.put(pos, "minecraft:copper_ore"); }
            let mut drill = machine(id);
            set_number(&mut drill, "Module", 3);
            let (_, base, draw) = extraction::tier_of(id).unwrap();
            let (ticks, _) = upgrades::apply(extraction::duration(base, 1, purity), draw, upgrades::effect(&drill));
            for time in 0..ticks * 5 {
                set_number(&mut drill, "energy", 2000);
                extraction::tick(&mut world, [0, 64, 0], &mut drill, i64::from(time));
            }
            assert_eq!(drill.items[0], Some(ItemStack::new("minecraft:raw_copper", 6)), "{id}, pureza {purity}");
            assert_eq!(number(&drill, "Produced"), 6);
            assert_eq!(number(&drill, "Total"), 5);
            assert_eq!(world.blocks.len(), blocks, "extração não esgota a jazida");
        }
    }
}

#[test]
fn batch_recipes_report_items_instead_of_cycles() {
    let mut kitchen = machine(KITCHEN);
    kitchen.items[0] = Some(ItemStack::new(FLOUR, 2));
    kitchen.items[1] = Some(ItemStack::new("minecraft:cocoa_beans", 1));
    for _ in 0..80 {
        set_number(&mut kitchen, "energy", 2000);
        let before = number(&kitchen, "Produced");
        process(&mut kitchen);
        super::super::record_performance(&mut kitchen, before);
    }
    assert_eq!(number(&kitchen, "Rate"), 120, "8 biscoitos em 4 segundos");
    assert_eq!(number(&kitchen, "Produced"), 8);
    assert_eq!(number(&kitchen, "Total"), 1);
}


#[test]
fn old_untyped_bonus_is_not_awarded_to_a_new_product() {
    let mut furnace = machine(ELECTRIC_FURNACE);
    set_number(&mut furnace, "Module", 3);
    set_number(&mut furnace, "Bonus", 90); // save antigo, sem BonusItem
    furnace.items[0] = Some(ItemStack::new("minecraft:ancient_debris", 1));
    run(&mut furnace, 460);
    assert_eq!(furnace.items[1], Some(ItemStack::new("minecraft:netherite_scrap", 1)));
    assert_eq!(number(&furnace, "Bonus"), 20);
}

#[test]
fn a_full_extractor_keeps_the_earned_bonus_until_transport_makes_room() {
    let mut world = World::default();
    world.put([0, 63, 0], "minecraft:copper_ore");
    let mut drill = machine(DRILL);
    set_number(&mut drill, "Module", 3);
    for time in 0..92 * 4 {
        set_number(&mut drill, "energy", 2000);
        extraction::tick(&mut world, [0, 64, 0], &mut drill, time);
    }
    assert_eq!(number(&drill, "Bonus"), 80);
    drill.items[0] = Some(ItemStack::new("minecraft:raw_copper", 63));
    let energy = number(&drill, "energy");
    extraction::tick(&mut world, [0, 64, 0], &mut drill, 400);
    assert_eq!(number(&drill, "Status"), 3);
    assert_eq!(number(&drill, "energy"), energy);
    assert_eq!(number(&drill, "Bonus"), 80);
    drill.items[0].as_mut().unwrap().count = 62;
    for time in 401..493 {
        set_number(&mut drill, "energy", 2000);
        extraction::tick(&mut world, [0, 64, 0], &mut drill, time);
    }
    assert_eq!(drill.items[0], Some(ItemStack::new("minecraft:raw_copper", 64)));
    assert_eq!(number(&drill, "Produced"), 6);
}

#[test]
fn a_ranch_never_treats_saddled_transport_pigs_as_surplus_meat() {
    use horizon_client::mod_api::EntityRef;
    struct RidingPigs;
    impl Host for RidingPigs {
        fn dimension(&self) -> &'static str { "veio:protected_transport_pigs" }
        fn entity(&self, id: u32) -> Option<EntityRef> { Some(EntityRef::new(id)) }
        fn entity_position(&self, _: EntityRef) -> Option<[f64; 3]> { Some([0.5, 64.0, 0.5]) }
        fn entity_nbt(&mut self, _: EntityRef) -> Option<Compound> {
            Some(Compound::from([("Age".into(), Tag::Int(0)), ("Saddle".into(), Tag::Byte(1))]))
        }
        fn item_entities(&self, _: &str, _: [f64; 3], _: [f64; 3]) -> Vec<horizon_client::mod_api::NativeItemEntity> { Vec::new() }
        fn kill_mob(&mut self, _: &str, _: u32) -> bool { panic!("a saddled pig must not be slaughtered") }
    }
    let mut host = RidingPigs;
    for id in 200100..200112 { ranch::observe(host.dimension(), id, "minecraft:pig"); }
    let mut pen = machine(RANCH);
    ranch::tick(&mut host, [0, 64, 0], &mut pen, 0);
    assert_eq!(number(&pen, "Info"), 0, "mounts do not enter the farm's breeding/culling pool");
    assert_eq!(number(&pen, "Status"), NO_ANIMALS);
}

// ---------- Cores: pigmento, tinta, peça pintada e bens finais ----------

fn stocked(id: &str, items: &[(usize, &str, u32)]) -> ModBlockEntity {
    let mut entity = machine(id);
    for &(slot, item, count) in items {
        entity.items[slot] = Some(ItemStack::new(item, count));
    }
    entity
}

#[test]
fn manufacturer_makes_more_from_a_fuller_mix() {
    // Engrenagem + chapa: uma placa reforçada.
    let mut plain = stocked(MANUFACTURER, &[(0, "veio:iron_plate", 4), (1, GEAR, 2)]);
    run(&mut plain, 120);
    assert_eq!(plain.items[4], Some(ItemStack::new("veio:reinforced_plate", 1)));
    // Engrenagem + chapa + fiação: a receita maior vence e rende o dobro.
    let mut stitched = stocked(MANUFACTURER, &[(0, "veio:iron_plate", 4), (1, GEAR, 2), (2, "veio:wire", 4)]);
    run(&mut stitched, 160);
    assert_eq!(stitched.items[4], Some(ItemStack::new("veio:reinforced_plate", 2)));
    assert!(stitched.items[2].is_none(), "a fiação foi usada");
}

#[test]
fn byproduct_goes_to_its_slot_and_stops_the_line_when_full() {
    let mut steel = stocked(MANUFACTURER, &[(0, "veio:iron_plate", 3), (1, "minecraft:coal", 2), (2, "minecraft:sand", 1)]);
    run(&mut steel, 160);
    assert_eq!(steel.items[4], Some(ItemStack::new("veio:steel_plate", 2)));
    assert_eq!(steel.items[5], Some(ItemStack::new("minecraft:gravel", 1)));
    // Sobra entupida: nada é consumido, a máquina avisa saída bloqueada.
    let mut jammed = stocked(MANUFACTURER, &[(0, "veio:iron_plate", 3), (1, "minecraft:coal", 2), (2, "minecraft:sand", 1),
        (5, "minecraft:gravel", 64)]);
    run(&mut jammed, 200);
    assert!(jammed.items[4].is_none());
    assert_eq!(jammed.items[0].as_ref().unwrap().count, 3);
    assert_eq!(number(&jammed, "Status"), 3);
}

#[test]
fn locked_recipe_waits_for_its_own_ingredients() {
    let mut entity = stocked(MANUFACTURER, &[(0, "veio:iron_plate", 4), (1, GEAR, 2)]);
    let motor = recipes(MANUFACTURER).position(|recipe| recipe.id == "motor").unwrap() as i32 + 1;
    for _ in 0..motor {
        press(&mut entity, PICK_NEXT);
    }
    assert_eq!(number(&entity, "Pick"), motor);
    run(&mut entity, 200);
    assert!(entity.items[4].is_none(), "fixada no motor, não faz placa reforçada");
    assert_eq!(number(&entity, "Status"), 2);
    assert_eq!(number(&entity, "RecipeIndex"), motor);
    assert_eq!(number(&entity, "Info"), 1 + motor * PICK_SHIFT, "falta a entrada 1 da receita fixada");
    // Voltar até a automática destrava.
    for _ in 0..motor {
        press(&mut entity, PICK_PREVIOUS);
    }
    assert_eq!(number(&entity, "Pick"), 0);
    run(&mut entity, 120);
    assert_eq!(entity.items[4], Some(ItemStack::new("veio:reinforced_plate", 1)));
    // Antes da automática vem a última receita: o ciclo dá a volta.
    press(&mut entity, PICK_PREVIOUS);
    assert_eq!(number(&entity, "Pick"), recipes(MANUFACTURER).count() as i32);
}

#[test]
fn manufacturer_slots_route_each_ingredient_to_one_place() {
    let gear = ItemStack::new(GEAR, 1);
    let accepted: Vec<usize> = (0..6).filter(|slot| accepts(MANUFACTURER, *slot, &gear)).collect();
    assert_eq!(accepted, vec![1]);
    assert!(!accepts(MANUFACTURER, 4, &ItemStack::new("veio:reinforced_plate", 1)), "saída só para retirada");
    assert!(accepts(MANUFACTURER, 2, &ItemStack::new("minecraft:cherry_planks", 1)));
    assert_eq!(click(MANUFACTURER, [20.0, 110.0]), Some(PICK_PREVIOUS));
    assert_eq!(click(MANUFACTURER, [150.0, 110.0]), Some(PICK_NEXT));
    assert_eq!(click(MANUFACTURER, [150.0, 60.0]), None);
    assert_eq!(click(KITCHEN, [150.0, 110.0]), None);
}

#[test]
fn a_red_plate_goes_from_flower_to_dye_to_paint_to_part() {
    // Moinho: papoula vira corante.
    let mut mill = stocked(MILL, &[(0, "minecraft:poppy", 1)]);
    run(&mut mill, 30);
    assert_eq!(mill.items[1], Some(ItemStack::new("minecraft:red_dye", 2)));
    // Misturador: 2 corantes + 1 argila = 2 latas.
    let mut mixer = stocked(PAINT_MIXER, &[(0, "minecraft:red_dye", 2), (1, "minecraft:clay_ball", 1)]);
    run(&mut mixer, 70);
    assert_eq!(mixer.items[2], Some(ItemStack::new("veio:red_paint", 2)));
    // Cabine: 4 chapas + 1 lata = 4 chapas vermelhas.
    let mut booth = stocked(PAINT_BOOTH, &[(0, "veio:iron_plate", 4), (1, "veio:red_paint", 1)]);
    run(&mut booth, 50);
    assert_eq!(booth.items[2], Some(ItemStack::new("veio:red_plate", 4)));
    assert!(booth.items[0].is_none() && booth.items[1].is_none());
}

#[test]
fn paints_mix_into_secondary_colors_and_paint_vanilla_blocks() {
    let mut mixer = stocked(PAINT_MIXER, &[(0, "veio:blue_paint", 1), (1, "veio:yellow_paint", 1)]);
    run(&mut mixer, 50);
    assert_eq!(mixer.items[2], Some(ItemStack::new("veio:green_paint", 2)), "azul + amarelo = verde");
    let mut booth = stocked(PAINT_BOOTH, &[(0, "minecraft:white_wool", 8), (1, "veio:green_paint", 1)]);
    run(&mut booth, 70);
    assert_eq!(booth.items[2], Some(ItemStack::new("minecraft:green_wool", 8)));
}

#[test]
fn the_deep_chain_and_end_goods_need_painted_parts() {
    let needs = |output: &str, part: &str| data::RECIPES.iter().filter(|recipe| recipe.output.0 == output)
        .all(|recipe| recipe.inputs.iter().any(|input| input.items.contains(&part)));
    assert!(needs("veio:mk2_kit", "veio:black_gear"), "kit Mk.II com engrenagens pretas");
    assert!(needs("veio:automation_core", "veio:blue_steel_plate"), "núcleo com aço azul");
    assert!(needs("veio:wall_clock", "veio:white_gear") && needs("veio:wall_clock", "veio:black_gear"));
    assert!(needs("veio:road_sign", "veio:yellow_plate"));
    assert!(needs("veio:bicycle", "veio:red_plate") && needs("veio:bicycle", "veio:black_gear"));
    // Tudo isso tem preço no mercado e o relógio e a bicicleta são bens de conforto das famílias.
    for item in ["veio:black_paint", "veio:black_gear", "veio:wall_clock", "veio:road_sign", "veio:bicycle"] {
        assert!(crate::veio::workforce::society::trade::product(item).is_some(), "{item}");
    }
    assert!(crate::veio::workforce::society::goods("veio:wall_clock").is_some());
}

#[test]
fn every_ingredient_has_one_slot_in_the_machines_that_share_them() {
    // A esteira entrega no primeiro slot que aceita: o mesmo item em dois slots travaria a receita.
    for machine in [PAINT_MIXER, PAINT_BOOTH, KITCHEN, FABRICATOR, "veio:integrator"] {
        let mut slot_of: Vec<(&str, usize)> = Vec::new();
        for recipe in recipes(machine) {
            for (slot, input) in recipe.inputs.iter().enumerate() {
                for item in input.items {
                    match slot_of.iter().find(|(known, _)| known == item) {
                        Some((_, known)) => assert_eq!(*known, slot, "{machine}: {item} em dois slots"),
                        None => slot_of.push((item, slot)),
                    }
                }
            }
        }
    }
}

#[test]
fn advanced_goods_build_on_painted_parts_and_sell() {
    use crate::veio::workforce::society::{goods, trade};
    let made_from = |output: &str| data::RECIPES.iter().find(|recipe| recipe.output.0 == output)
        .map(|recipe| recipe.inputs.iter().flat_map(|input| input.items.iter().copied()).collect::<Vec<_>>())
        .unwrap_or_default();
    assert!(made_from("veio:motorcycle").contains(&"veio:bicycle"), "a moto nasce da bicicleta");
    assert!(made_from("veio:electric_car").contains(&"veio:blue_steel_plate"));
    assert!(made_from("veio:helper_robot").contains(&"veio:white_gear"));
    assert!(made_from("veio:radio").contains(&"veio:yellow_plate"));
    for item in ["veio:electric_fan", "veio:radio", "veio:sewing_machine", "veio:television", "veio:refrigerator",
        "veio:motorcycle", "veio:electric_car", "veio:helper_robot"] {
        assert!(!made_from(item).is_empty(), "{item} tem receita");
        assert!(trade::product(item).is_some(), "{item} tem preço");
    }
    // Eletrodomésticos são conforto das famílias; veículos e robô, só exportação.
    assert!(goods("veio:refrigerator").is_some() && goods("veio:television").is_some());
    assert!(goods("veio:electric_car").is_none() && goods("veio:helper_robot").is_none());
    // Uma geladeira sai de uma fabricadora abastecida.
    let mut fabricator = stocked(FABRICATOR, &[(0, "veio:motor", 1), (1, "veio:circuit", 2), (2, "veio:blue_steel_plate", 2)]);
    run(&mut fabricator, 260);
    assert_eq!(fabricator.items[3], Some(ItemStack::new("veio:refrigerator", 1)));
}
