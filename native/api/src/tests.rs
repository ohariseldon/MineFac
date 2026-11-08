//! Testes de regras do Veio. Rodam com `python tools/check_rust.py`, sem abrir o jogo.
use super::*;
use horizon_client::mod_api::{BlockStateId, ParticleOptions};
use std::collections::HashMap;

#[test]
fn numeric_updates_preserve_legacy_tag_and_clamping_semantics() {
    let mut n = ModBlockEntity::default();
    n.nbt.insert("energy".into(), Tag::Int(-7));
    set_number(&mut n, "energy", 0);
    assert!(matches!(n.nbt.get(nbt_key("energy")), Some(Tag::Int(-7))));
    n.nbt.insert("Status".into(), Tag::String("legacy".into()));
    set_number(&mut n, "Status", 0);
    assert!(matches!(n.nbt.get(nbt_key("Status")), Some(Tag::String(_))));
    set_number(&mut n, "Status", 3);
    set_number(&mut n, "energy", 120);
    assert_eq!(number(&n, "Status"), 3);
    assert_eq!(number(&n, "energy"), 120);
    let count = n.nbt.len();
    set_number(&mut n, "energy", 121);
    assert_eq!(n.nbt.len(), count);
}

fn entity(id: &str, slots: usize) -> ModBlockEntity {
    let mut entity = ModBlockEntity::default();
    entity.id = id.to_owned();
    entity.items = vec![None; slots];
    entity
}

#[test]
fn integrator_waits_for_both_inputs_and_preserves_them_when_output_is_full() {
    let run = |entity: &mut ModBlockEntity, ticks: usize| {
        for _ in 0..ticks {
            set_number(entity, "energy", 2000);
            production::process(entity);
        }
    };
    let mut integrator = entity(INTEGRATOR, 3);
    integrator.items[0] = Some(ItemStack::new(FRAME, 2));
    run(&mut integrator, 60);
    assert_eq!(integrator.items[0].as_ref().unwrap().count, 2);
    assert!(integrator.items[2].is_none());
    assert_eq!(number(&integrator, "Status"), 2, "falta a engrenagem");
    integrator.items[1] = Some(ItemStack::new(GEAR, 3));
    integrator.items[2] = Some(ItemStack::new(MODULE, 64));
    let blocked = integrator.items.clone();
    run(&mut integrator, 60);
    assert_eq!(integrator.items, blocked);
    assert_eq!(number(&integrator, "Status"), 3);
    integrator.items[2] = None;
    run(&mut integrator, 60);
    assert_eq!(integrator.items[0].as_ref().unwrap().count, 1);
    assert_eq!(integrator.items[1].as_ref().unwrap().count, 2);
    assert_eq!(integrator.items[2], Some(ItemStack::new(MODULE, 1)));
}

#[test]
fn crusher_keeps_multi_item_output_within_one_stack() {
    let mut crusher = entity(CRUSHER, 2);
    crusher.items[0] = Some(ItemStack::new("minecraft:bone", 5));
    crusher.items[1] = Some(ItemStack::new(BONE_MEAL, 61));
    for _ in 0..40 {
        set_number(&mut crusher, "energy", 2000);
        production::process(&mut crusher);
    }
    assert_eq!(crusher.items[0].as_ref().unwrap().count, 5, "4 farinhas não cabem em 61 de 64");
    crusher.items[1] = Some(ItemStack::new(BONE_MEAL, 60));
    for _ in 0..30 {
        set_number(&mut crusher, "energy", 2000);
        production::process(&mut crusher);
    }
    assert_eq!(crusher.items[1].as_ref().unwrap().count, 64);
    assert!(accepts(CRUSHER, 0, &ItemStack::new("minecraft:cobblestone", 1)));
    assert!(!accepts(CRUSHER, 0, &ItemStack::new("minecraft:stone", 1)));
}

#[test]
fn menus_fence_outputs_and_route_inputs() {
    assert!(accepts(INTEGRATOR, 0, &ItemStack::new(FRAME, 1)));
    assert!(!accepts(INTEGRATOR, 0, &ItemStack::new(GEAR, 1)));
    assert!(accepts(INTEGRATOR, 1, &ItemStack::new(GEAR, 1)));
    assert!(!accepts(INTEGRATOR, 2, &ItemStack::new(MODULE, 1)));
    assert!(!accepts(DRILL, 0, &ItemStack::new("minecraft:raw_copper", 1)));
    assert!(accepts(GENERATOR, 0, &ItemStack::new("minecraft:charcoal", 1)));
    assert!(!accepts(GENERATOR, 0, &ItemStack::new("minecraft:stone", 1)));
    assert!(accepts(HARVESTER, 0, &ItemStack::new(BONE_MEAL, 1)));
    assert!(!accepts(HARVESTER, 1, &ItemStack::new("minecraft:wheat", 1)), "saídas só para retirada");
    assert!(!accepts(TESLA, 0, &ItemStack::new("minecraft:arrow", 1)));
}

#[test]
fn generator_burns_vanilla_fuels_in_proportion_to_furnace_time() {
    let fuel = |item: &str| production::generation::fuel_ticks(GENERATOR, &ItemStack::new(item, 1));
    assert_eq!(fuel("minecraft:coal"), Some(1600));
    assert_eq!(fuel("minecraft:oak_log"), Some(300));
    assert_eq!(fuel("minecraft:stripped_birch_log"), Some(300));
    assert_eq!(fuel("minecraft:coal_block"), Some(16000));
    assert_eq!(fuel("minecraft:crimson_stem"), None, "madeira do Nether não queima");
    assert_eq!(fuel("minecraft:cobblestone"), None);
    assert_eq!(production::generation::fuel_ticks(production::BIOMASS_GENERATOR, &ItemStack::new(production::BIOMASS, 1)), Some(600));
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
fn idle_cables_skip_neighbor_lookups() {
    struct Probe { visits: usize, neighbor: ModBlockEntity }
    impl Host for Probe {
        fn change_mod_block_entity(
            &mut self,
            _pos: [i32; 3],
            change: &mut dyn FnMut(&mut ModBlockEntity),
        ) -> bool {
            self.visits += 1;
            change(&mut self.neighbor);
            true
        }
    }
    let mut probe = Probe { visits: 0, neighbor: entity(CABLE, 0) };
    let mut cable = entity(CABLE, 0);
    tick_power(&mut probe, [0, 0, 0], &mut cable);
    assert_eq!(probe.visits, 0, "cabo vazio não consulta vizinhos");
    set_number(&mut cable, "energy", 80);
    tick_power(&mut probe, [0, 0, 0], &mut cable);
    assert_eq!(probe.visits, 6, "com carga, percorre as seis faces");
}

#[test]
fn gems_need_the_advanced_extractor() {
    let diamond = production::extraction::node("minecraft:deepslate_diamond_ore").unwrap();
    assert_eq!((diamond.output.0, diamond.tier), ("minecraft:diamond", 2));
    assert_eq!(production::extraction::node("minecraft:lapis_ore").unwrap().tier, 1);
    assert!(production::extraction::node("minecraft:dirt").is_none());
}

/// Um pedaço de mundo com plantações: blocos e a propriedade `age`.
#[derive(Default)]
struct Field {
    blocks: HashMap<[i32; 3], (String, u8)>,
    particles: usize,
}

impl Host for Field {
    fn dimension(&self) -> &'static str {
        "veio:test_field"
    }
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
    let export = contracts::CONTRACTS.len() + contracts::PHASES.len();
    set_number(&mut hub, "ContractStage", export as i32);
    hub.items[0] = Some(ItemStack::new(CIRCUIT, 8));
    hub.items[1] = Some(ItemStack::new(STEEL, 8));
    contracts::tick(&mut hub);
    contracts::tick(&mut hub);
    assert_eq!(number(&hub, "ContractStage") as usize, export);
    assert_eq!(number(&hub, "Total"), 2);
    assert_eq!(hub.items[2], Some(ItemStack::new("minecraft:emerald", 8)));
    assert!(hub.items[0].is_none() && hub.items[1].is_none());
}

#[test]
fn grand_project_collects_slowly_and_pays_each_phase() {
    let mut hub = entity(HUB, 4);
    set_number(&mut hub, "ContractStage", contracts::CONTRACTS.len() as i32);
    hub.items[0] = Some(ItemStack::new(STEEL, 64));
    hub.items[1] = Some(ItemStack::new(MODULE, 3));
    assert!(!contracts::tick(&mut hub));
    assert_eq!((number(&hub, "Total"), number(&hub, "Info")), (4, 3), "4 por tick, o que houver");
    set_number(&mut hub, "Total", 199);
    set_number(&mut hub, "Info", 100);
    assert!(!contracts::tick(&mut hub));
    assert_eq!(number(&hub, "ContractStage") as usize, contracts::CONTRACTS.len() + 1, "fase 1 fechada");
    assert_eq!(hub.items[0].as_ref().map(|stack| stack.count), Some(59), "não leva além da meta");
    assert_eq!(hub.items[2], Some(ItemStack::new("minecraft:emerald", 32)));
    assert_eq!(number(&hub, "Total"), 0);
    set_number(&mut hub, "ContractStage", (contracts::CONTRACTS.len() + 3) as i32);
    hub.items = vec![Some(ItemStack::new("veio:automation_core", 1)), Some(ItemStack::new("veio:mk2_kit", 1)), None, None];
    set_number(&mut hub, "Total", 199);
    set_number(&mut hub, "Info", 49);
    assert!(contracts::tick(&mut hub), "a última fase concede o avanço");
    assert!(contracts::valid_input(0, &ItemStack::new("veio:processor", 1)));
}

#[test]
fn poles_link_symmetrically_within_the_shorter_reach() {
    // Uma fileira de postes de madeira a cada 6 blocos e um de aço longe.
    let poles: Vec<([i32; 3], bool)> = vec![
        ([0, 64, 0], false), ([6, 64, 0], false), ([12, 64, 0], false), ([30, 64, 0], true), ([42, 64, 0], true),
    ];
    let around = |center: [i32; 3]| poles.iter().filter(|(at, _)| *at != center).copied().collect::<Vec<_>>();
    let links = |pos: [i32; 3], steel: bool| power::links(pos, steel, around);
    assert_eq!(links([0, 64, 0], false), vec![[6, 64, 0]]);
    assert_eq!(links([6, 64, 0], false), vec![[0, 64, 0], [12, 64, 0]]);
    assert_eq!(links([12, 64, 0], false), vec![[6, 64, 0]], "madeira não alcança 18 blocos");
    assert_eq!(links([30, 64, 0], true), vec![[42, 64, 0]], "aço com aço chega a 12 blocos");
    for (pos, steel) in &poles {
        for other in links(*pos, *steel) {
            let other_steel = poles.iter().find(|(at, _)| *at == other).unwrap().1;
            assert!(links(other, other_steel).contains(pos), "ligação de mão única entre {pos:?} e {other:?}");
        }
    }
}

#[test]
fn pole_balance_moves_energy_toward_the_emptier_buffer() {
    assert_eq!(power::balance(1000, 1000, 0, 1000, 400), 400, "limitado pela vazão");
    assert_eq!(power::balance(200, 1000, 0, 1000, 400), 100);
    assert_eq!(power::balance(0, 1000, 30000, 30000, 120), -120, "acumulador cheio alimenta o poste");
    assert_eq!(power::balance(500, 1000, 1000, 2000, 400), 0, "mesma porcentagem");
}

#[test]
fn wires_sag_between_their_ends() {
    let points = power::sag_points([0.5, 1.4, 0.5], [8.5, 1.4, 0.5]);
    assert_eq!(points.first(), Some(&[0.5, 1.4, 0.5]));
    assert_eq!(points.last().map(|p| (p[0], p[1])), Some((8.5, 1.4)));
    let middle = points[points.len() / 2];
    assert!(middle[1] < 1.1, "o meio do fio fica mais baixo: {middle:?}");
}

#[test]
fn pollution_spreads_and_fades_without_growing() {
    let mut cells = HashMap::from([((0, 0), 1000.0_f32)]);
    let total = |cells: &HashMap<(i32, i32), f32>| cells.values().sum::<f32>();
    let before = total(&cells);
    cells = pollution::diffuse(&cells);
    assert!(cells[&(1, 0)] > 0.0 && cells[&(0, -1)] > 0.0, "vizinhos recebem parte");
    assert!((total(&cells) - before * 0.98).abs() < 0.5, "só 2% somem por passo");
    for _ in 0..400 {
        cells = pollution::diffuse(&cells);
    }
    assert!(total(&cells) < 1.0, "sem emissão, tudo se dissipa");
}

#[test]
fn raids_need_a_notice_a_day_of_grace_and_the_night() {
    use pollution::Plan;
    let anchor = [3, 64, 3];
    let hot = |level: f32| [((0, 0), level)];
    // Primeiro contato: só o aviso, mesmo com muita poluição e de noite.
    assert_eq!(pollution::plan_for_test(&hot(5000.0), anchor, None, 1000, 15000), Some(Plan::Notice(anchor)));
    // Menos de um dia depois do aviso, nada acontece.
    assert_eq!(pollution::plan_for_test(&hot(5000.0), anchor, Some(1000), 20000, 15000), None);
    // Depois da carência: alerta ao anoitecer e onda à noite.
    assert_eq!(pollution::plan_for_test(&hot(1500.0), anchor, Some(0), 30000, 24000 + 12000), Some(Plan::Warning(anchor)));
    assert_eq!(pollution::plan_for_test(&hot(1500.0), anchor, Some(0), 30000, 24000 + 15000), Some(Plan::Wave(anchor, 4)));
    // De dia ou abaixo do limite, não há ataque.
    assert_eq!(pollution::plan_for_test(&hot(1500.0), anchor, Some(0), 30000, 24000 + 3000), None);
    assert_eq!(pollution::plan_for_test(&hot(900.0), anchor, Some(0), 30000, 24000 + 15000), None);
}

#[test]
fn waves_grow_but_stay_capped() {
    assert_eq!(pollution::wave_size(pollution::RAID, 1), 4);
    assert_eq!(pollution::wave_size(pollution::RAID + 1200.0, 3), 9);
    assert_eq!(pollution::wave_size(1.0e6, 50), 12);
    assert_eq!(100 / pollution::BLOW, 4, "quatro golpes param a máquina");
    assert_eq!(100 / pollution::BRUTE_BLOW, 2, "o Bruto para a máquina em dois golpes");
}

#[test]
fn waves_bring_archers_and_brutes_later() {
    let (raiders, archers, brutes) = pollution::wave_mix(4, 1);
    assert_eq!((raiders, archers, brutes), (4, 0, 0), "a primeira onda é só de saqueadores");
    let (raiders, archers, brutes) = pollution::wave_mix(8, pollution::ARCHERS_FROM);
    assert_eq!((raiders + archers + brutes, archers, brutes), (8, 2, 0), "da 3ª onda vêm arqueiros");
    let (raiders, archers, brutes) = pollution::wave_mix(12, pollution::BRUTES_FROM);
    assert_eq!(raiders + archers + brutes, 12, "o tamanho da onda não muda");
    assert!(brutes >= 1 && archers >= 1 && raiders > brutes, "da 5ª vem o Bruto, ainda com maioria de saqueadores");
    assert!(pollution::villager_hunter("minecraft:skeleton") && pollution::villager_hunter("minecraft:spider"));
    assert!(!pollution::villager_hunter("minecraft:creeper"), "creeper continua atrás do jogador");
}

#[test]
fn modules_trade_one_bottleneck_for_another() {
    let speed = upgrades::effect_of(1);
    let (duration, draw) = upgrades::apply(30, 8, speed);
    assert_eq!((duration, draw), (18, 14));
    assert!(duration * draw > 30 * 8, "velocidade custa mais energia por produto");
    let efficiency = upgrades::effect_of(2);
    assert_eq!(upgrades::apply(30, 8, efficiency), (30, 5));
    let mut press = entity(PRESS, 2);
    set_number(&mut press, "Module", 3);
    let extras = (0..20).filter(|_| upgrades::bonus_ready(&mut press)).count();
    assert_eq!(extras, 2, "produtividade: um extra a cada dez ciclos");
    assert!(!upgrades::supports(TESLA));
}

#[test]
fn panels_publish_as_many_values_as_the_menus_declare() {
    let mut press = entity(PRESS, 2);
    presentation::publish(&mut press);
    assert_eq!(press.data.len(), presentation::DATA_SLOTS);
}

#[test]
fn one_pass_numbers_read_like_number() {
    let mut n = entity(PRESS, 2);
    n.nbt.insert("Burn".into(), Tag::Int(-3));
    n.nbt.insert("Info".into(), Tag::String("x".into()));
    n.nbt.insert("Aaa".into(), Tag::Int(9));
    n.nbt.insert("Zzz".into(), Tag::Int(9));
    set_number(&mut n, "Module", 2);
    set_number(&mut n, "energy", 77);
    let names = ["Burn", "Damage", "Info", "Module", "Status", "energy"];
    let keys = names.map(nbt_key);
    assert_eq!(numbers_sorted(&n, &keys), names.map(|name| number(&n, name)));
    assert_eq!(numbers_sorted(&n, &keys), [0, 0, 0, 2, 0, 77]);
}

#[test]
fn published_signature_matches_the_nbt_for_every_block() {
    let blocks = [DRILL, DRILL_MK2, PRESS, ASSEMBLER, BELT, FAST_BELT, SPLITTER, MERGER, CUTTER, INTEGRATOR,
        GENERATOR, CABLE, SOLAR, BATTERY, GEOTHERMAL, CRUSHER, ALLOY, ELECTRONICS, HUB, HARVESTER, TESLA,
        POLE, STEEL_POLE, inserters::ID, inserters::FAST, inserters::LONG, "veio:drill_mk3", "veio:electric_furnace",
        "veio:mill", "veio:kitchen", "veio:biomass_generator", "veio:water_wheel", "veio:wind_turbine",
        "veio:steam_engine", "veio:planter", "veio:grow_lamp", "veio:tree_farm", "veio:ranch", "veio:pantry",
        "veio:farm_post", "veio:hauler_post", "veio:foreman_desk", "veio:shepherd_post", "veio:barn", "veio:silo",
        "veio:control_center", "veio:field_marker", "veio:pen_marker"];
    for block in blocks {
        assert!(is_factory_block(block), "{block}");
        for seed in 0..4 {
            let mut n = entity(block, layout(block).map_or(0, |(slots, _, _)| slots));
            for (key, value) in [("Status", 1 + seed), ("Module", seed % 3), ("Info", 7 * seed), ("MovedAt", 11 + seed),
                ("LocalSmog", 40 + seed), ("RoutingPriority", seed % 3)] {
                set_number(&mut n, key, value);
            }
            presentation::publish(&mut n);
            assert_eq!(published_signature(&n), visual_signature(&n), "{block}");
        }
    }
}

#[test]
fn belt_cargo_syncs_only_when_the_drawn_item_changes() {
    let mut belt = entity(BELT, 1);
    assert!(!presentation::publish(&mut belt), "vazia antes e depois: nada a redesenhar");
    belt.items[0] = Some(ItemStack::new(PLATE, 3));
    assert!(presentation::publish(&mut belt), "item novo aparece no desenho");
    belt.items[0] = Some(ItemStack::new(PLATE, 2));
    assert!(!presentation::publish(&mut belt), "só a quantidade mudou: o desenho é o mesmo");
    assert_eq!(number(&belt, "DisplayCount"), 2, "o NBT continua com a quantidade");
    belt.items[0] = Some(ItemStack::new(GEAR, 2));
    assert!(presentation::publish(&mut belt));
    belt.items[0] = None;
    assert!(presentation::publish(&mut belt), "a carga some do desenho");
    assert!(matches!(belt.nbt.get(nbt_key("DisplayItem")), Some(Tag::String(name)) if name.0.is_empty()));
}

#[test]
fn workforce_posts_count_as_item_input_for_belt_curves() {
    assert!(has_item_inputs(workforce::FARM_POST));
    assert!(has_item_inputs(workforce::HAULER_POST));
    assert!(has_item_inputs(workforce::FOREMAN_DESK));
    assert!(has_item_inputs(workforce::SHEPHERD_POST));
    assert!(!has_item_inputs(SOLAR));
}

#[test]
fn every_company_block_belongs_to_a_holding_division() {
    use holding::*;
    assert_eq!(division(DRILL), Some(MINING));
    assert_eq!(division(PRESS), Some(MANUFACTURING));
    assert_eq!(division(GENERATOR), Some(ENERGY));
    assert_eq!(division(BELT), Some(LOGISTICS));
    assert_eq!(division(workforce::FARM_POST), Some(AGRO));
    assert_eq!(division(production::TREE_FARM), Some(FORESTRY), "madeira e folhas: extrativismo");
    assert_eq!(division(workforce::society::HOUSE), Some(URBAN));
    assert_eq!(division(workforce::SHOP), Some(COMMERCE));
    assert_eq!(division(workforce::CONTROL_CENTER), Some(MANAGEMENT));
    assert_eq!(division(workforce::TOWN_HALL), None, "a prefeitura é pública, fora da holding");
}

#[test]
fn the_holding_counts_units_workers_and_daily_production_per_division() {
    use holding::*;
    let dimension = "veio:test_holding";
    let mut drill = ModBlockEntity::default();
    drill.id = DRILL.to_owned();
    set_number(&mut drill, "Status", 1);
    set_number(&mut drill, "Produced", 10);
    // Posição com fase 0: (x*7 ^ z*13) & 127 = 0.
    let pos = [0, 64, 0];
    observe(dimension, pos, &drill, 0);
    set_number(&mut drill, "Produced", 25);
    observe(dimension, pos, &drill, 100);
    observe(dimension, pos, &drill, 150);
    roll(dimension, 1200);
    let [units, active, workers, made, _, _, _, _] = summary(dimension, MINING);
    assert_eq!((units, active, workers), (1, 1, 0));
    assert_eq!(made, 15, "15 minérios no turno viram a produção do dia");
    roll(dimension, 1300);
    assert_eq!(summary(dimension, MINING)[3], 15, "um turno só fecha uma vez");
    // As abas do centro de controle: setas no cabeçalho.
    assert_eq!(click([10.0, 25.0], 1), Some(PREVIOUS));
    assert_eq!(click([170.0, 25.0], 1), Some(NEXT));
    assert_eq!(click([90.0, 60.0], 1), None);
    assert_eq!(click([90.0, 70.0], 0), Some(workforce::society::ledger::CASH_OUT), "linha da companhia: saque");
    let mut center = ModBlockEntity::default();
    center.id = workforce::CONTROL_CENTER.to_owned();
    // Primeiro as páginas da vila (aba 0), depois as divisões.
    press(dimension, [0, 64, 0], &mut center, NEXT, 0);
    assert_eq!((number(&center, "Tab"), number(&center, "Page")), (0, 1), "página Atenção");
    for _ in 1..workforce::center::PAGES {
        press(dimension, [0, 64, 0], &mut center, NEXT, 0);
    }
    assert_eq!(number(&center, "Tab"), 1, "aba Mineração");
    assert_eq!(data(&center)[0], 1, "uma extratora na aba");
    press(dimension, [0, 64, 0], &mut center, PREVIOUS, 0);
    assert_eq!((number(&center, "Tab"), number(&center, "Page")), (0, workforce::center::PAGES - 1), "volta à última página");
    for _ in 0..workforce::center::PAGES {
        press(dimension, [0, 64, 0], &mut center, PREVIOUS, 0);
    }
    assert_eq!(number(&center, "Tab"), TABS - 1, "volta pela última aba");
}

#[test]
fn each_good_pays_the_business_that_makes_it() {
    use holding::*;
    assert_eq!(item_division("minecraft:bread"), AGRO);
    assert_eq!(item_division("minecraft:white_wool"), AGRO);
    assert_eq!(item_division("minecraft:oak_log"), FORESTRY);
    assert_eq!(item_division("minecraft:raw_iron"), MINING);
    assert_eq!(item_division("veio:gear"), MANUFACTURING);
}
