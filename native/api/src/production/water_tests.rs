//! Testes da água encanada, do aspersor e da turbina.
use super::*;
use std::collections::HashMap;

/// Mundo mínimo: blocos com propriedades e máquinas do Veio por posição.
#[derive(Default)]
struct Plot {
    blocks: HashMap<[i32; 3], (String, Vec<(String, String)>)>,
    machines: HashMap<[i32; 3], ModBlockEntity>,
    rain: f32,
}

impl Plot {
    fn put(&mut self, pos: [i32; 3], block: &str, props: &[(&str, &str)]) {
        self.blocks.insert(pos, (block.to_owned(),
            props.iter().map(|(k, v)| ((*k).to_owned(), (*v).to_owned())).collect()));
    }
    fn water(&mut self, pos: [i32; 3]) {
        self.put(pos, "minecraft:water", &[("level", "0")]);
    }
}

impl Host for Plot {
    fn dimension(&self) -> &'static str { "veio:test_water" }
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
    fn change_mod_block_entity(&mut self, pos: [i32; 3], change: &mut dyn FnMut(&mut ModBlockEntity)) -> bool {
        match self.machines.get_mut(&pos) {
            Some(entity) => { change(entity); true }
            None => false,
        }
    }
    fn light(&self, _: [i32; 3]) -> (u8, u8) { (0, 15) }
    fn weather(&self) -> (f32, f32) { (self.rain, 0.0) }
}

fn entity(id: &str) -> ModBlockEntity {
    let mut entity = ModBlockEntity::default();
    entity.id = id.to_owned();
    entity.items = vec![None; super::super::super::layout(id).map_or(0, |(slots, _, _)| slots)];
    entity
}

#[test]
fn pump_fills_from_still_water_and_pipes_carry_it_to_a_sprinkler() {
    let mut plot = Plot::default();
    plot.water([0, 63, 0]);
    plot.water([1, 64, 1]); // não encosta: não conta
    plot.machines.insert([1, 64, 0], entity(PIPE));
    plot.machines.insert([2, 64, 0], entity(PIPE));
    plot.machines.insert([3, 64, 0], entity(SPRINKLER));
    let mut pump_entity = entity(PUMP);
    for _ in 0..200 {
        set_number(&mut pump_entity, "energy", 2000);
        pump(&mut plot, [0, 64, 0], &mut pump_entity);
        for pos in [[1, 64, 0], [2, 64, 0]] {
            let mut pipe_entity = plot.machines.remove(&pos).unwrap();
            pipe(&mut plot, pos, &mut pipe_entity);
            plot.machines.insert(pos, pipe_entity);
        }
    }
    assert_eq!(number(&pump_entity, "Info"), pump_rate(1), "uma fonte encostada (o poço abaixo)");
    let sprinkler_water = number(&plot.machines[&[3, 64, 0]], "Water");
    assert!(sprinkler_water > 2000, "a água chegou ao aspersor: {sprinkler_water}");
    assert!(sprinkler_water <= capacity(SPRINKLER));
    assert!(number(&pump_entity, "Total") > 0);
}

#[test]
fn pump_without_water_or_power_says_why() {
    let mut plot = Plot::default();
    let mut dry = entity(PUMP);
    set_number(&mut dry, "energy", 2000);
    pump(&mut plot, [0, 64, 0], &mut dry);
    assert_eq!(number(&dry, "Status"), NO_WATER);
    plot.water([1, 64, 0]);
    let mut unpowered = entity(PUMP);
    pump(&mut plot, [0, 64, 0], &mut unpowered);
    assert_eq!(number(&unpowered, "Status"), 5);
    assert_eq!(number(&unpowered, "Water"), 0);
}

#[test]
fn pipes_level_out_and_never_push_back_into_the_pump() {
    assert_eq!(share(1000, 1000, 0, 1000, true), 200, "até o limite por face");
    assert_eq!(share(600, 1000, 400, 1000, true), 100, "metade da diferença");
    assert_eq!(share(400, 1000, 600, 1000, true), 0, "não sobe para o mais cheio");
    assert_eq!(share(50, 1000, 3000, 4000, false), 50, "consumidor recebe o que houver");
    assert!(!receives(PUMP));
    assert!(receives(PIPE) && receives(SPRINKLER) && receives(STEAM_ENGINE));
}

/// Lavoura 9x9 de trigo em volta do aspersor, na camada abaixo dele.
fn field(plot: &mut Plot, crop: &str) {
    for dx in -4..=4 {
        for dz in -4..=4 {
            if dx == 0 && dz == 0 { continue; }
            plot.put([dx, 63, dz], "minecraft:farmland", &[("moisture", "0")]);
            plot.put([dx, 64, dz], crop, &[("age", "0")]);
        }
    }
}

fn ages(plot: &Plot) -> u32 {
    plot.blocks.iter().filter(|(pos, _)| pos[1] == 64)
        .filter_map(|(_, (_, props))| props.iter().find(|(k, _)| k == "age"))
        .map(|(_, v)| v.parse::<u32>().unwrap()).sum()
}

fn run_sprinkler(plot: &mut Plot, sprinkler: &mut ModBlockEntity, ticks: i64, refill: bool) {
    for time in 0..ticks {
        if refill { set_number(sprinkler, "Water", capacity(SPRINKLER)); }
        budget::begin_tick(time as u64 + 10_000);
        sprinkle(plot, [0, 64, 0], sprinkler, time);
    }
}

#[test]
fn sprinkler_keeps_farmland_wet_and_fed_crops_grow_faster_than_watered_ones() {
    let mut fed_plot = Plot::default();
    field(&mut fed_plot, "minecraft:wheat");
    let mut fed = entity(SPRINKLER);
    fed.items[0] = Some(ItemStack::new(BONE_MEAL, 64));
    run_sprinkler(&mut fed_plot, &mut fed, 81 * 40, true);

    let mut watered_plot = Plot::default();
    field(&mut watered_plot, "minecraft:wheat");
    let mut watered = entity(SPRINKLER);
    run_sprinkler(&mut watered_plot, &mut watered, 81 * 40, true);

    assert!(fed_plot.blocks.values().filter(|(id, _)| id == "minecraft:farmland")
        .all(|(_, props)| props[0].1 == "7"), "terra arada úmida sem poça no meio");
    let (fed_growth, watered_growth) = (ages(&fed_plot), ages(&watered_plot));
    assert!(watered_growth > 0, "só com água também cresce, devagar");
    assert!(fed_growth > watered_growth * 2, "adubo e água: {fed_growth} contra {watered_growth}");
    assert!(fed.items[0].as_ref().unwrap().count < 64, "o adubo é consumido");
}

#[test]
fn sprinkler_without_water_does_nothing_and_rain_halves_the_soil_water() {
    let mut plot = Plot::default();
    field(&mut plot, "minecraft:carrots");
    let mut dry = entity(SPRINKLER);
    run_sprinkler(&mut plot, &mut dry, 200, false);
    assert_eq!(number(&dry, "Status"), NO_WATER);
    assert_eq!(ages(&plot), 0);

    let soil_only = |rain: f32| {
        let mut plot = Plot { rain, ..Plot::default() };
        for dx in -4..=4 {
            for dz in -4..=4 {
                plot.put([dx, 63, dz], "minecraft:farmland", &[("moisture", "0")]);
            }
        }
        let mut sprinkler = entity(SPRINKLER);
        set_number(&mut sprinkler, "Water", 4000);
        for time in 0..81 {
            budget::begin_tick(time as u64 + 20_000);
            sprinkle(&mut plot, [0, 64, 0], &mut sprinkler, time);
        }
        4000 - number(&sprinkler, "Water")
    };
    let (dry_cost, rain_cost) = (soil_only(0.0), soil_only(1.0));
    assert_eq!(dry_cost, 81 * SOIL_WATER, "uma passada pela área");
    assert_eq!(rain_cost, dry_cost / 2);
}

#[test]
fn crop_needs_follow_real_demand_and_fertilizers_are_accepted() {
    let (wheat, _, _) = needs("minecraft:wheat").unwrap();
    let (carrot, _, _) = needs("minecraft:carrots").unwrap();
    let (potato, _, _) = needs("minecraft:potatoes").unwrap();
    assert!(carrot < wheat && wheat < potato, "cenoura < trigo < batata em água por estágio");
    let (beet, beet_food, beet_ripe) = needs("minecraft:beetroots").unwrap();
    assert_eq!(beet_ripe, 3);
    assert!(beet * i32::from(beet_ripe) > potato * 7 / 2 && beet_food * 3 > 30, "beterraba: sede e solo exigente");
    assert!(needs("minecraft:nether_wart").is_none(), "fungo do Nether não usa irrigação");
    assert!(super::super::accepts(SPRINKLER, 0, &ItemStack::new(BONE_MEAL, 1)));
    assert!(super::super::accepts(SPRINKLER, 0, &ItemStack::new(BIOMASS, 1)));
    assert!(!super::super::accepts(SPRINKLER, 0, &ItemStack::new("minecraft:dirt", 1)));
}

#[test]
fn hydro_power_grows_with_head_reservoir_and_river_flow() {
    assert_eq!(hydro_power(2, 40, 0), 0, "queda pequena demais");
    assert_eq!(hydro_power(10, 5, 0), 0, "poça não é represa");
    assert_eq!(hydro_power(10, 40, 0), 60);
    assert_eq!(hydro_power(10, 20, 0), 30, "metade do espelho, metade da potência");
    assert_eq!(hydro_power(10, 40, 4), 120, "vazão do rio dobra");
    assert_eq!(hydro_power(32, 40, 12), HYDRO_MAX);
    assert!(hydro_power(20, 40, 0) > hydro_power(10, 40, 0));
}

#[test]
fn turbine_measures_the_dam_behind_it() {
    let mut plot = Plot::default();
    // Turbina em (0, 64, 0) virada para o sul: represa ao norte, saída ao sul.
    plot.put([0, 64, 0], HYDRO_TURBINE, &[("facing", "south")]);
    for y in 64..74 {
        for dx in -4..=4 {
            for dz in -8..=-1 {
                plot.water([dx, y, dz]);
            }
        }
    }
    let (head, surface) = measure_dam(&plot, [0, 64, 0]);
    assert_eq!(head, 10);
    assert!(surface >= 40, "espelho: {surface}");
    // Água também do lado de baixo: conta só a diferença de nível.
    for y in 64..68 {
        plot.water([0, y, 1]);
    }
    assert_eq!(measure_dam(&plot, [0, 64, 0]).0, 6);
    let mut turbine_entity = entity(HYDRO_TURBINE);
    let energy = turbine(&mut plot, [0, 64, 0], &mut turbine_entity, 0, 2000);
    assert_eq!(energy, hydro_power(6, surface, 0));
    assert_eq!(number(&turbine_entity, "Head"), 6);
}
