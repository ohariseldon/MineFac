//! Progressão em fases: razões inteiras entre máquinas, energia por linha e veios conectados.
//! Números e justificativas em frentes/PROGRESSAO-FASES.md.
use super::*;
use std::cell::Cell;
use std::collections::HashMap;

const MINUTE: i32 = 1200;

fn recipe(machine: &str, input: &str) -> &'static Recipe {
    data::RECIPES.iter()
        .find(|r| r.machine == machine && !r.alt && r.inputs[0].items.contains(&input))
        .unwrap_or_else(|| panic!("{machine} <- {input}"))
}

/// Itens por minuto que a receita entrega e quanto ela consome do primeiro ingrediente.
fn rates(r: &Recipe) -> (i32, i32) {
    let cycles = MINUTE / r.ticks;
    assert_eq!(MINUTE % r.ticks, 0, "{} precisa de ciclos inteiros por minuto", r.machine);
    (cycles * r.output.1 as i32, cycles * r.inputs[0].count as i32)
}

/// Itens por minuto de uma extratora sem módulo em minério comum.
fn drill_rate(block: &str, purity: i32) -> i32 {
    let (_, base, _) = extraction::tier_of(block).unwrap();
    MINUTE / extraction::duration(base, 1, purity)
}

#[test]
fn the_basic_unit_is_thirty_per_minute() {
    assert_eq!(drill_rate(DRILL, 2), 30, "Mk.I normal");
    for (machine, input) in [("veio:press", "minecraft:raw_copper"), ("veio:press", "minecraft:raw_iron"),
        ("veio:cutter", "veio:iron_plate"), ("veio:crusher", "minecraft:quartz")] {
        assert_eq!(rates(recipe(machine, input)), (30, 30), "{machine} <- {input}: uma extratora, uma máquina");
    }
    // Cada classe de jazida e cada nível de extratora vale um número inteiro de unidades.
    assert_eq!([1, 2, 3, 4].map(|p| drill_rate(DRILL, p)), [15, 30, 60, 120]);
    assert_eq!([1, 2, 3, 4].map(|p| drill_rate(DRILL_MK2, p)), [30, 60, 120, 240]);
    assert_eq!([1, 2, 3, 4].map(|p| drill_rate(DRILL_MK3, p)), [60, 120, 240, 400]);
}

#[test]
fn chains_close_with_whole_machine_counts() {
    let press = rates(recipe("veio:press", "minecraft:raw_copper")).0;
    let (gears, gear_plates) = rates(recipe("veio:assembler", "veio:copper_plate"));
    assert_eq!((gears, gear_plates / press), (30, 2), "2 prensas : 1 montador");
    let (wire, wire_plates) = rates(recipe("veio:cutter", "veio:copper_plate"));
    assert_eq!((wire, wire_plates / press), (60, 1), "1 prensa : 1 cortadora de fiação");
    let (steel, steel_plates) = rates(recipe("veio:alloy_furnace", "veio:iron_plate"));
    assert_eq!((steel, steel_plates / press), (15, 1), "1 extratora de ferro : 1 prensa : 1 forno de ligas");
    let (modules, frames) = rates(recipe("veio:integrator", "veio:frame"));
    assert_eq!((modules, frames), (15, 15), "2 integradores : 1 cortadora de moldura : 1 montador");
    let electronics = recipe("veio:electronics", "veio:wire");
    let (circuits, circuit_wire) = rates(electronics);
    let silicon = MINUTE / electronics.ticks * electronics.inputs[1].count as i32;
    assert_eq!((circuits, circuit_wire, silicon), (15, 30, 15), "2 montadores de circuito : 1 cortadora : 1 triturador");
    assert_eq!(2 * circuit_wire, wire, "a cortadora de fiação alimenta exatamente dois");
}

#[test]
fn each_phase_line_fits_its_power_source() {
    let power = |machine: &str, input: &str| recipe(machine, input).power;
    let drill = extraction::tier_of(DRILL).unwrap().2;
    let generator = generation::burner(GENERATOR).unwrap().0;
    let steam = generation::burner(STEAM_ENGINE).unwrap().0;
    let water_wheel = 4 * generation::WATER_FACE;
    // Primeira fábrica: 1 extratora, 1 prensa, 1 montador cabem numa roda d'água.
    let starter = drill + power("veio:press", "minecraft:raw_copper") + power("veio:assembler", "veio:copper_plate");
    assert!(starter <= water_wheel, "{starter} > {water_wheel}");
    // Linha de engrenagens (30/min): gargalo de energia, exatamente um gerador.
    let gears = 2 * drill + 2 * power("veio:press", "minecraft:raw_copper") + power("veio:assembler", "veio:copper_plate");
    assert!(gears > water_wheel && gears <= generator, "{gears}");
    // Linha de aço (15/min) com o carvão dela: outro gerador.
    let steel = 2 * drill + power("veio:press", "minecraft:raw_iron") + power("veio:alloy_furnace", "veio:iron_plate");
    assert_eq!(steel, generator);
    // Linha de módulos (30/min): pede a caldeira a vapor desbloqueada pelo aço.
    let modules = 3 * drill + 3 * power("veio:press", "minecraft:raw_iron") + power("veio:assembler", "veio:copper_plate")
        + power("veio:cutter", "veio:iron_plate") + 2 * power("veio:integrator", "veio:frame");
    assert!(modules > 2 * generator && modules <= steam, "{modules}");
    // Linha de circuitos (30/min): duas linhas de gerador ou uma caldeira.
    let circuits = 2 * drill + power("veio:press", "minecraft:raw_copper") + power("veio:cutter", "veio:copper_plate")
        + power("veio:crusher", "minecraft:quartz") + 2 * power("veio:electronics", "veio:wire");
    assert!(circuits > generator && circuits <= steam, "{circuits}");
}

#[test]
fn milestones_need_a_running_line_not_a_single_item() {
    // Cada salto de fase pede uma pilha que só uma linha completa entrega em 1 a 3 minutos.
    let stock = |path: &str| -> i64 {
        let text = match path {
            "engineering" => include_str!("../../../../content/data/veio/advancement/engineering.json"),
            "steel" => include_str!("../../../../content/data/veio/advancement/steel.json"),
            _ => include_str!("../../../../content/data/veio/advancement/circuits.json"),
        };
        let start = text.find("\"min\": ").expect(path) + 7;
        text[start..].split(|c: char| !c.is_ascii_digit()).next().unwrap().parse().unwrap()
    };
    for (path, per_minute) in [("engineering", 15), ("steel", 15), ("circuits", 15)] {
        let minutes = stock(path) as f64 / per_minute as f64;
        assert!((1.0..=3.0).contains(&minutes), "{path}: {minutes} min com uma linha");
    }
}

struct Vein {
    blocks: HashMap<[i32; 3], &'static str>,
    reads: Cell<usize>,
}

impl Vein {
    fn of(block: &'static str, positions: impl IntoIterator<Item = [i32; 3]>) -> Self {
        Self { blocks: positions.into_iter().map(|p| (p, block)).collect(), reads: Cell::new(0) }
    }
}

impl Host for Vein {
    fn dimension(&self) -> &'static str { "veio:vein-test" }
    fn block(&self, pos: [i32; 3]) -> String {
        self.reads.set(self.reads.get() + 1);
        self.blocks.get(&pos).copied().unwrap_or("minecraft:air").to_owned()
    }
}

fn node_of(block: &str) -> &'static Node { extraction::node(block).unwrap() }

fn drill_on(world: &mut Vein, id: &str, ticks: i64) -> ModBlockEntity {
    let mut drill = ModBlockEntity::default();
    drill.id = id.to_owned();
    drill.items = vec![None; super::super::layout(id).map_or(1, |(slots, _, _)| slots)];
    for time in 0..ticks {
        set_number(&mut drill, "energy", 2000);
        extraction::tick(world, [0, 64, 0], &mut drill, time);
    }
    drill
}

#[test]
fn a_vertical_bent_vein_is_one_deposit_and_diagonals_do_not_count() {
    let copper = node_of("minecraft:copper_ore");
    let mut world = Vein::of("minecraft:copper_ore", [[0, 0, 0], [0, -1, 0], [0, -2, 0], [1, -2, 0], [2, -2, 0], [2, -2, 1]]);
    world.blocks.insert([0, -3, 0], "minecraft:deepslate_copper_ore");
    world.blocks.insert([1, 1, 1], "minecraft:copper_ore");
    world.blocks.insert([0, 1, 0], "minecraft:iron_ore");
    assert_eq!(extraction::connected(&world, [0, 0, 0], copper), 7, "ardósia conta; diagonal e ferro não");
    assert_eq!(extraction::purity(7), 3);
}

#[test]
fn the_search_stops_at_the_useful_limit_and_stone_keeps_the_cheap_square() {
    let copper = node_of("minecraft:copper_ore");
    let world = Vein::of("minecraft:copper_ore", (0..1000).map(|x| [x, 0, 0]));
    assert_eq!(extraction::connected(&world, [0, 0, 0], copper), extraction::VEIN_LIMIT);
    assert!(world.reads.get() <= 1 + 6 * extraction::VEIN_LIMIT, "{} leituras", world.reads.get());
    assert!(!extraction::is_vein(node_of("minecraft:stone")));
    let mut quarry = Vein::of("minecraft:stone", (-20..=20).flat_map(|x| (-20..=20).map(move |z| [x, 63, z])));
    let drill = drill_on(&mut quarry, DRILL, 1);
    assert_eq!(number(&drill, "Info"), 3, "pedreira: 3x3 cheio, sem busca no terreno inteiro");
    assert_eq!(quarry.reads.get(), 10, "bloco abaixo + nove do quadrado");
}

#[test]
fn an_industrial_vein_quadruples_speed_at_double_power() {
    let mut world = Vein::of("minecraft:iron_ore", (0..20).map(|depth| [0, 63 - depth, 0]));
    let drill = drill_on(&mut world, DRILL, 40);
    assert_eq!(number(&drill, "Info"), 4, "coluna de 20: classe industrial");
    assert_eq!(number(&drill, "Duration"), 10);
    assert_eq!(drill.items[0].as_ref().map(|s| s.count), Some(4), "120/min na Mk.I");
    let draw = extraction::tier_of(DRILL).unwrap().2;
    assert_eq!(extraction::rich_draw(draw, 4) * 10, draw * 20, "FE por minério igual ao da jazida pura");
}

#[test]
fn rare_ores_stay_capped_at_double() {
    let mut world = Vein::of("minecraft:diamond_ore", (0..20).map(|depth| [0, 63 - depth, 0]));
    let drill = drill_on(&mut world, DRILL_MK2, 1);
    assert_eq!(number(&drill, "Info"), 3);
}

#[test]
fn a_denied_terrain_quota_keeps_the_last_class_and_retries_later() {
    let mut world = Vein::of("minecraft:copper_ore", (0..20).map(|depth| [0, 63 - depth, 0]));
    let mut drill = drill_on(&mut world, DRILL, 1);
    assert_eq!(number(&drill, "Info"), 4);
    world.blocks.retain(|pos, _| pos[1] >= 62);
    budget::begin_tick(10_000);
    assert!(budget::request(budget::Lane::Terrain, world.dimension(), [999; 3], 384));
    let before = world.reads.get();
    // Fase escolhida para cair na releitura periódica: (time + (x ^ z) & 63) % 200 == 0.
    extraction::tick(&mut world, [0, 64, 0], &mut drill, 200);
    assert_eq!(number(&drill, "Info"), 4, "sem cota, mantém a última classe medida");
    assert_eq!(world.reads.get() - before, 1, "só o bloco abaixo");
    budget::begin_tick(10_001);
    extraction::tick(&mut world, [0, 64, 0], &mut drill, 400);
    assert_eq!(number(&drill, "Info"), 1, "veio partido: dois blocos, impura");
}
