//! Máquinas grandes: plantas, colocação (chão plano e livre), montagem, escotilhas e desmonte.
use super::*;
use horizon_client::mod_api::NativeServerPlayer;
use std::collections::HashMap;

#[derive(Default)]
struct World {
    blocks: HashMap<[i32; 3], (String, Vec<(String, String)>)>,
    entities: HashMap<[i32; 3], ModBlockEntity>,
    said: Vec<String>,
    given: Vec<(String, u32)>,
    commands: Vec<String>,
    player: [f64; 3],
}

impl World {
    fn flat(size: i32) -> Self {
        let mut world = World { player: [0.5, 64.0, 10.5], ..World::default() };
        for x in -size..=size {
            for z in -size..=size {
                world.put([x, 63, z], "minecraft:grass_block", &[]);
            }
        }
        world
    }
    fn put(&mut self, pos: [i32; 3], block: &str, props: &[(&str, &str)]) {
        self.blocks.insert(pos, (block.to_owned(), props.iter().map(|(k, v)| ((*k).to_owned(), (*v).to_owned())).collect()));
        self.entities.remove(&pos);
        if is_hatch(block) || layout(block).is_some() {
            let mut entity = ModBlockEntity::default();
            entity.id = block.to_owned();
            entity.items = vec![None; super::super::layout(block).map_or(0, |(slots, _, _)| slots)];
            self.entities.insert(pos, entity);
        }
    }
    fn count(&self, block: &str) -> usize {
        self.blocks.values().filter(|(id, _)| id == block).count()
    }
}

impl Host for World {
    fn dimension(&self) -> &'static str { "minecraft:overworld" }
    fn block(&self, pos: [i32; 3]) -> String {
        self.blocks.get(&pos).map_or_else(|| "minecraft:air".to_owned(), |(id, _)| id.clone())
    }
    fn block_property(&self, pos: [i32; 3], key: &str) -> Option<String> {
        self.blocks.get(&pos)?.1.iter().find(|(k, _)| k == key).map(|(_, v)| v.clone())
    }
    fn edit_blocks(&mut self, _: &str, edit: &NativeBlockEdit) -> Result<u64, String> {
        for run in &edit.runs {
            let spec = &edit.palette[run.block as usize];
            let (id, props) = match spec.split_once('[') {
                Some((id, rest)) => (id, rest.trim_end_matches(']').split(',')
                    .filter_map(|pair| pair.split_once('=')).collect::<Vec<_>>()),
                None => (spec.as_str(), Vec::new()),
            };
            if id == "minecraft:air" {
                self.blocks.remove(&[run.x, run.y, run.z]);
                self.entities.remove(&[run.x, run.y, run.z]);
            } else {
                self.put([run.x, run.y, run.z], id, &props);
            }
        }
        Ok(edit.runs.len() as u64)
    }
    fn change_mod_block_entity(&mut self, pos: [i32; 3], change: &mut dyn FnMut(&mut ModBlockEntity)) -> bool {
        self.entities.get_mut(&pos).map(|entity| change(entity)).is_some()
    }
    fn insert_item(&mut self, pos: [i32; 3], _: Direction, mut stack: ItemStack) -> Option<ItemStack> {
        let entity = self.entities.get_mut(&pos)?;
        let (_, inputs, _) = super::super::layout(&entity.id)?;
        for slot in inputs {
            match &mut entity.items[slot] {
                None => { entity.items[slot] = Some(stack); return None; }
                Some(held) if held.item == stack.item => { held.count += stack.count; stack.count = 0; return None; }
                _ => {}
            }
        }
        Some(stack)
    }
    fn acting_player(&self) -> Option<NativeServerPlayer> {
        Some(NativeServerPlayer { uuid: 1, name: "Luiz".into(), permission: 4, dimension: "minecraft:overworld".into(),
            position: self.player, game_mode: 0, health: 20.0 })
    }
    fn action_bar(&mut self, _: u128, line: Text) -> bool { self.said.push(line.plain()); true }
    fn give_items(&mut self, item: &str, count: u32) -> u32 { self.given.push((item.to_owned(), count)); count }
    fn play_sound(&mut self, _: &str, _: [f64; 3], _: usize, _: f32, _: f32) -> bool { true }
    fn sneaking(&self) -> bool { false }
    fn run_command(&mut self, line: &str) -> bool {
        self.commands.push(line.to_owned());
        let parts: Vec<&str> = line.split(' ').collect();
        if parts[0] == "setblock" {
            let at = [parts[1].parse().unwrap(), parts[2].parse().unwrap(), parts[3].parse().unwrap()];
            self.blocks.remove(&at);
            self.entities.remove(&at);
        }
        true
    }
}

/// Coloca a máquina como o jogo faz: o gancho antes, o bloco, e o gancho depois.
fn place(world: &mut World, item: &str, ground: [i32; 3]) -> Flow {
    let flow = use_item(world, item, ground, Direction::Up);
    if matches!(flow, Flow::Cancel) {
        return flow;
    }
    let at = [ground[0], ground[1] + 1, ground[2]];
    let facing = facing_towards(at, world.player);
    world.put(at, item, &[("facing", facing)]);
    placed(world, item, at);
    flow
}

#[test]
fn every_layout_is_a_real_footprint_with_a_free_front_and_an_outlet() {
    for (block, outputs) in [("veio:press", true), ("veio:drill", true), ("veio:drill_mk3", true), ("veio:steam_engine", false),
        ("veio:fabricator", true), ("veio:generator", false), ("veio:cutter", true), ("veio:electronics", true)] {
        let layout = layout(block).expect(block);
        let [w, h, d] = layout.size;
        assert!(w * d >= 4 && h >= 1, "{block}: precisa ocupar mais de um bloco");
        assert!(layout.cells.iter().all(|cell| cell.at[2] <= 0 && cell.at[1] >= 0), "{block}: só para trás e para cima");
        assert!(!layout.cells.iter().any(|cell| cell.at == [0, 0, 1] || cell.at == [0, 0, 0]), "{block}: a frente do núcleo fica livre");
        let out = layout.cells.iter().filter(|cell| cell.part == Part::HatchOut).count();
        assert_eq!(out > 0, outputs, "{block}: escotilha de saída");
        // A boca da escotilha de saída (atrás) dá para fora da planta.
        for cell in layout.cells.iter().filter(|cell| cell.part == Part::HatchOut) {
            let behind = [cell.at[0], 0, cell.at[2] - 1];
            assert!(!layout.cells.iter().any(|other| other.at == behind), "{block}: saída tapada");
        }
    }
    let sizes: std::collections::BTreeSet<_> = ["veio:drill", "veio:drill_mk3", "veio:press", "veio:steam_engine", "veio:fabricator"]
        .iter().map(|block| layout(block).unwrap().size).collect();
    assert_eq!(sizes.len(), 5, "formatos diferentes");
    assert_eq!(layout("veio:steam_engine").unwrap().size, [3, 2, 5]);
    assert!(layout("veio:belt").is_none() && layout("veio:pole").is_none(), "logística segue de um bloco");
}

#[test]
fn rotation_keeps_the_structure_behind_the_front() {
    let layout = layout("veio:press").unwrap();
    for (facing, behind) in [("south", [0, 0, -1]), ("north", [0, 0, 1]), ("west", [1, 0, 0]), ("east", [-1, 0, 0])] {
        let cells: Vec<_> = cells([0; 3], facing, layout).map(|(at, _)| at).collect();
        assert!(cells.contains(&behind), "{facing}");
        let front = [-behind[0], 0, -behind[2]];
        assert!(!cells.contains(&front), "{facing}: a frente fica livre");
    }
}

#[test]
fn placing_on_flat_free_ground_builds_the_whole_machine() {
    let mut world = World::flat(8);
    assert!(matches!(place(&mut world, "veio:press", [0, 63, 0]), Flow::Continue));
    let layout = layout("veio:press").unwrap();
    let parts = world.count(STRUCTURE) + world.count(HATCH_IN) + world.count(HATCH_OUT);
    assert_eq!(parts, layout.cells.len());
    assert_eq!(world.block_property([0, 64, -1], "facing").as_deref(), Some("south"), "peças giradas como o núcleo");
    assert_eq!(number(&world.entities[&[0, 64, 0]], "Frame"), 1);
    assert!(world.said.iter().any(|line| line.contains("veio.frame.built")), "{:?}", world.said);
}

#[test]
fn blocked_or_uneven_ground_refuses_and_outlines_the_problem() {
    let mut world = World::flat(8);
    world.put([1, 64, -1], "minecraft:stone", &[]);
    assert!(matches!(place(&mut world, "veio:press", [0, 63, 0]), Flow::Cancel));
    assert_eq!(world.count("veio:press"), 0);
    assert!(world.said.last().unwrap().contains("veio.frame.blocked"));
    assert!(REFUSED.lock().unwrap().as_ref().unwrap().1.contains(&[1, 64, -1]));
    // Um buraco embaixo da planta também barra: é preciso aplainar o terreno.
    let mut world = World::flat(8);
    world.blocks.remove(&[1, 63, -2]);
    assert!(matches!(place(&mut world, "veio:press", [0, 63, 0]), Flow::Cancel));
    world.put([1, 63, -2], "minecraft:dirt", &[]);
    assert!(matches!(place(&mut world, "veio:press", [0, 63, 0]), Flow::Continue));
    // Grama alta e flores somem sob a máquina.
    let mut world = World::flat(8);
    world.put([0, 64, -1], "minecraft:short_grass", &[]);
    assert!(matches!(place(&mut world, "veio:press", [0, 63, 0]), Flow::Continue));
}

#[test]
fn a_direction_that_does_not_fit_after_placing_is_undone_and_refunded() {
    let mut world = World::flat(8);
    world.put([0, 64, -1], "minecraft:stone", &[]);
    world.put([0, 64, 0], "veio:press", &[("facing", "south")]);
    placed(&mut world, "veio:press", [0, 64, 0]);
    assert_eq!(world.count("veio:press"), 0);
    assert_eq!(world.given, vec![("veio:press".to_owned(), 1)]);
}

#[test]
fn breaking_any_part_takes_the_whole_machine_and_returns_what_was_inside() {
    let mut world = World::flat(8);
    place(&mut world, "veio:press", [0, 63, 0]);
    let out = cells([0, 64, 0], "south", layout("veio:press").unwrap())
        .find(|(_, part)| *part == Part::HatchOut).unwrap().0;
    world.entities.get_mut(&out).unwrap().items[0] = Some(ItemStack::new("veio:iron_plate", 3));
    let roof = [0, 66, -1];
    assert_eq!(world.block(roof), STRUCTURE);
    destroy(&mut world, roof);
    assert_eq!(world.commands, vec!["setblock 0 64 0 minecraft:air destroy".to_owned()]);
    assert_eq!(world.count(HATCH_OUT), 0);
    assert_eq!(world.count(STRUCTURE), 1, "só a peça que o jogador quebra fica para o jogo tirar");
    assert!(world.given.contains(&("veio:iron_plate".to_owned(), 3)));
    // Quebrar o núcleo leva a estrutura junto.
    let mut world = World::flat(8);
    place(&mut world, "veio:drill", [0, 63, 0]);
    destroy(&mut world, [0, 64, 0]);
    assert_eq!(world.count(STRUCTURE) + world.count(HATCH_OUT), 0);
}

#[test]
fn hatches_carry_items_between_the_belts_and_the_core() {
    let mut world = World::flat(8);
    place(&mut world, "veio:integrator", [0, 63, 0]);
    let all: Vec<_> = cells([0, 64, 0], "south", layout("veio:integrator").unwrap()).collect();
    let input = all.iter().find(|(_, part)| *part == Part::HatchIn).unwrap().0;
    let output = all.iter().find(|(_, part)| *part == Part::HatchOut).unwrap().0;
    // Entrada: o que chega à escotilha vai para o núcleo.
    let mut hatch = world.entities.remove(&input).unwrap();
    hatch.items[0] = Some(ItemStack::new("veio:copper_plate", 5));
    for time in 0..8 { hatch_tick(&mut world, input, &mut hatch, time); }
    assert_eq!(hatch.items[0], None);
    assert_eq!(world.entities[&[0, 64, 0]].items[0], Some(ItemStack::new("veio:copper_plate", 5)));
    assert_eq!(number(&hatch, "CoreSet"), 1, "guarda onde está o núcleo");
    // Saída: o produto do núcleo passa à escotilha, até 16 por vez.
    world.entities.get_mut(&[0, 64, 0]).unwrap().items[2] = Some(ItemStack::new("veio:circuit", 20));
    let mut hatch = world.entities.remove(&output).unwrap();
    for time in 0..8 { hatch_tick(&mut world, output, &mut hatch, time); }
    assert_eq!(hatch.items[0], Some(ItemStack::new("veio:circuit", HATCH_STACK)));
    assert_eq!(world.entities[&[0, 64, 0]].items[2], Some(ItemStack::new("veio:circuit", 4)));
}

#[test]
fn the_core_restores_missing_pieces_and_the_wrench_does_not_twist_it() {
    let mut world = World::flat(8);
    place(&mut world, "veio:assembler", [0, 63, 0]);
    let before = world.count(STRUCTURE);
    world.blocks.remove(&[-1, 64, -1]);
    let core = world.entities[&[0, 64, 0]].clone();
    for clock in 0..REPAIR_EVERY { core_tick(&mut world, [0, 64, 0], &core, clock); }
    assert_eq!(world.count(STRUCTURE), before);
    assert!(fixed(&mut world, [0, 64, 0], "veio:assembler"));
    // Uma máquina de mundo antigo, sem estrutura, segue girando com a chave.
    world.put([5, 64, 5], "veio:assembler", &[("facing", "south")]);
    assert!(!fixed(&mut world, [5, 64, 5], "veio:assembler"));
}

#[test]
fn explosions_spare_the_structure() {
    let mut world = World::flat(8);
    place(&mut world, "veio:generator", [0, 63, 0]);
    let mut blocks = vec![[0, 64, 0], [1, 64, 0], [0, 63, 0]];
    explosion(&world, &mut blocks);
    assert_eq!(blocks, vec![[0, 64, 0], [0, 63, 0]]);
}
