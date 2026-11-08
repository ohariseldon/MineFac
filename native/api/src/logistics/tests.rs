use super::*;
use std::cell::Cell;
use std::collections::HashMap;
const COPPER: &str = "minecraft:raw_copper";
const IRON: &str = "minecraft:raw_iron";

#[derive(Default)]
struct Outlet {
    room: usize,
    only: Option<&'static str>,
    received: Vec<String>,
}

impl Host for Outlet {
    fn insert_item(&mut self, pos: [i32; 3], face: Direction, stack: ItemStack) -> Option<ItemStack> {
        assert_eq!(pos, [0, 0, -1]);
        assert_eq!(face, Direction::South);
        if self.received.len() == self.room || self.only.is_some_and(|id| id != stack.item.as_ref()) {
            Some(stack)
        } else {
            assert_eq!(stack.count, 1);
            self.received.push(stack.item.to_string());
            None
        }
    }
}

fn loaded() -> ModBlockEntity {
    let mut entity = ModBlockEntity::default();
    entity.id = MERGER.to_owned();
    entity.items = vec![Some(ItemStack::new(COPPER, 3)), Some(ItemStack::new(IRON, 3))];
    entity
}

#[test]
fn both_inputs_get_turns_even_when_the_outlet_accepts_only_one_per_cycle() {
    let mut entity = loaded();
    let mut outlet = Outlet::default();
    for room in 1..=6 {
        outlet.room = room;
        assert_eq!(merge(&mut outlet, [0; 3], Direction::North, &mut entity, 2), 1);
    }
    assert_eq!(outlet.received, [COPPER, IRON, COPPER, IRON, COPPER, IRON]);
    assert!(entity.items.iter().all(Option::is_none));
}

#[test]
fn full_outlet_preserves_both_buffers_and_the_next_turn() {
    let mut entity = loaded();
    set_number(&mut entity, "NextInput", 1);
    let before = entity.items.clone();
    let mut outlet = Outlet::default();
    assert_eq!(merge(&mut outlet, [0; 3], Direction::North, &mut entity, 2), 0);
    assert_eq!(entity.items, before);
    assert_eq!(number(&entity, "NextInput"), 1);
    outlet.room = 1;
    merge(&mut outlet, [0; 3], Direction::North, &mut entity, 2);
    assert_eq!(outlet.received, [IRON]);
}

#[test]
fn rejected_product_does_not_prevent_the_other_lane_from_delivering() {
    let mut entity = loaded();
    let mut outlet = Outlet { room: 2, only: Some(IRON), ..Default::default() };
    assert_eq!(merge(&mut outlet, [0; 3], Direction::North, &mut entity, 2), 2);
    assert_eq!(entity.items[0].as_ref().unwrap().count, 3);
    assert_eq!(entity.items[1].as_ref().unwrap().count, 1);
    assert_eq!(outlet.received, [IRON, IRON]);
    assert_eq!(number(&entity, "NextInput"), 0);
}

#[test]
fn each_orientation_has_two_distinct_side_inputs_and_no_rear_input() {
    for output in [Direction::North, Direction::East, Direction::South, Direction::West] {
        assert_eq!(input_slot(output, turn(output, false)), Some(0));
        assert_eq!(input_slot(output, turn(output, true)), Some(1));
        for face in [output, output.opposite(), Direction::Up, Direction::Down] {
            assert_eq!(input_slot(output, face), None);
        }
    }
}

#[derive(Default)]
struct RouteWorld {
    signal: i32,
    blocks: HashMap<[i32; 3], String>,
    properties: HashMap<([i32; 3], String), String>,
    delivered: Vec<([i32; 3], ItemStack)>,
    full: bool,
    lookups: Cell<usize>,
    property_writes: usize,
}

impl RouteWorld {
    fn put(&mut self, pos: [i32; 3], block: &str, facing: &str) {
        self.blocks.insert(pos, block.to_owned());
        self.properties.insert((pos, "facing".into()), facing.to_owned());
    }
}

impl Host for RouteWorld {
    fn dimension(&self) -> &'static str { "minecraft:overworld" }
    fn neighbor_signal(&mut self, _dimension: &str, _pos: [i32; 3]) -> i32 { self.signal }
    fn block(&self, pos: [i32; 3]) -> String {
        self.lookups.set(self.lookups.get() + 1);
        self.blocks.get(&pos).cloned().unwrap_or_else(|| "minecraft:air".into())
    }
    fn block_property(&self, pos: [i32; 3], key: &str) -> Option<String> {
        self.properties.get(&(pos, key.into())).cloned()
    }
    fn set_block_property(&mut self, pos: [i32; 3], key: &str, value: &str) -> bool {
        self.property_writes += 1;
        self.properties.insert((pos, key.into()), value.into());
        true
    }
    fn insert_item(&mut self, pos: [i32; 3], _face: Direction, stack: ItemStack) -> Option<ItemStack> {
        if self.full { return Some(stack); }
        self.delivered.push((pos, stack));
        None
    }
}

fn belt() -> ModBlockEntity {
    let mut entity = ModBlockEntity::default();
    entity.id = BELT.into();
    entity.items = vec![Some(ItemStack::new("minecraft:raw_copper", 4))];
    entity
}

#[test]
fn belt_climbs_one_block_and_keeps_items_when_the_upper_outlet_is_full() {
    let mut world = RouteWorld::default();
    world.put([0; 3], BELT, "west");
    world.put([1, 0, 0], "minecraft:stone", "");
    world.put([1, 1, 0], FAST_BELT, "west");
    let mut entity = belt();
    world.full = true;
    transfer_outputs(&mut world, [0; 3], &mut entity, 0);
    assert_eq!(entity.items[0].as_ref().unwrap().count, 4);
    assert_eq!(world.block_property([0; 3], "path").as_deref(), Some("up"));
    world.full = false;
    transfer_outputs(&mut world, [0; 3], &mut entity, 8);
    assert_eq!(world.delivered[0].0, [1, 1, 0]);
    assert_eq!(entity.items[0].as_ref().unwrap().count, 3);
    world.put([0, 1, 0], "minecraft:stone", "");
    assert_eq!(belt_target(&world, [0; 3], Direction::East), [1, 0, 0]);
}

#[test]
fn lower_receiving_belt_builds_the_downhill_ramp() {
    let mut world = RouteWorld::default();
    world.put([0; 3], BELT, "south");
    world.put([0, 1, 1], BELT, "south");
    world.put([0, 0, -1], BELT, "south");
    assert_eq!(belt_target(&world, [0, 1, 1], Direction::North), [0; 3]);
    refresh_path(&mut world, [0; 3], &mut belt());
    assert_eq!(world.block_property([0; 3], "path").as_deref(), Some("down"));
    world.put([0, 1, 0], "minecraft:stone", "");
    assert_eq!(belt_target(&world, [0, 1, 1], Direction::North), [0, 1, 0]);
}

#[test]
fn belt_curves_into_a_farm_post() {
    let mut world = RouteWorld::default();
    world.put([0; 3], BELT, "west");
    world.put([-1, 0, 0], BELT, "west");
    world.put([0, 0, -1], "veio:farm_post", "");
    refresh_path(&mut world, [0; 3], &mut belt());
    assert_eq!(world.block_property([0; 3], "facing").as_deref(), Some("south"));
}

#[test]
fn a_single_incoming_and_outgoing_line_snap_to_a_curve_unless_manually_locked() {
    let mut world = RouteWorld::default();
    world.put([0; 3], BELT, "west");
    world.put([-1, 0, 0], BELT, "west");
    world.put([0, 0, -1], BELT, "south");
    let mut entity = belt();
    refresh_path(&mut world, [0; 3], &mut entity);
    assert_eq!(world.block_property([0; 3], "facing").as_deref(), Some("south"));
    assert_eq!(world.block_property([0; 3], "path").as_deref(), Some("left"));
    world.put([0; 3], BELT, "west");
    set_number(&mut entity, "ManualDirection", 1);
    refresh_path(&mut world, [0; 3], &mut entity);
    assert_eq!(world.block_property([0; 3], "facing").as_deref(), Some("west"));
}

#[test]
fn stable_neighborhood_skips_incoming_scans() {
    let mut world = RouteWorld::default();
    world.put([0; 3], BELT, "west");
    world.put([-1, 0, 0], BELT, "west");
    world.put([1, 0, 0], BELT, "west");
    let mut entity = belt();
    refresh_path(&mut world, [0; 3], &mut entity);
    let writes = world.property_writes;
    let first = world.lookups.get();
    refresh_path(&mut world, [0; 3], &mut entity);
    assert_eq!(world.property_writes, writes, "vizinhança igual: não reescreve path");
    let second = world.lookups.get() - first;
    assert!(second <= 14, "só a assinatura na segunda passagem: {second} consultas");
    assert!(first > second, "a primeira passagem relê entradas; a segunda não: {first} > {second}");
    world.put([1, 1, 0], FAST_BELT, "west");
    refresh_path(&mut world, [0; 3], &mut entity);
    assert!(world.lookups.get() - first - second > second, "vizinho novo relê a topologia");
}

#[test]
fn negative_signatures_also_skip_and_a_rotated_neighbor_invalidates() {
    let mut world = RouteWorld::default();
    world.put([0; 3], BELT, "west");
    world.put([-1, 0, 0], BELT, "west");
    world.put([1, 0, 0], BELT, "west");
    let mut entity = belt();
    refresh_path(&mut world, [0; 3], &mut entity);
    // Uma assinatura negativa guardada vale igual (antes, o piso 0 de `number` a ignorava).
    let sig = neighborhood_signature(&world, [0; 3], "west", 0);
    entity.nbt.insert(nbt_key("PathSig").clone(), Tag::Int(sig));
    let before = world.lookups.get();
    refresh_path(&mut world, [0; 3], &mut entity);
    let cached = world.lookups.get() - before;
    assert!(cached <= 14, "só a assinatura: {cached} consultas");
    // Girar a esteira de trás muda a assinatura: a curva é refeita.
    world.put([-1, 0, 0], BELT, "north");
    assert_ne!(neighborhood_signature(&world, [0; 3], "west", 0), sig);
}

#[test]
fn item_paths_meet_at_the_same_height_and_preserve_the_output_edge() {
    for path in ["straight", "left", "right", "up", "down"] {
        let end = path_point(path, 1.0);
        assert!((end[0] - 0.5).abs() < 0.0001);
        assert!(end[2].abs() < 0.0001);
    }
    assert!((path_point("up", 1.0)[1] - (1.0 + path_point("straight", 0.0)[1])).abs() < 0.0001);
    assert!((path_point("down", 0.0)[1] - (1.0 + path_point("straight", 1.0)[1])).abs() < 0.0001);
    assert_eq!(orient([0.5, 0.0, 0.0], "west"), [1.0, 0.0, 0.5]);
    assert_eq!(orient([0.5, 0.0, 0.0], "east"), [0.0, 0.0, 0.5]);
}

#[derive(Default)]
struct Junction {
    blocked: Option<Direction>,
    received: Vec<(Direction, String, u32)>,
}

impl Host for Junction {
    fn insert_item(&mut self, pos: [i32; 3], face: Direction, stack: ItemStack) -> Option<ItemStack> {
        let output = face.opposite();
        let [dx, _, dz] = output.normal();
        assert_eq!(pos, [dx, 0, dz]);
        if self.blocked == Some(output) { return Some(stack); }
        self.received.push((output, stack.item.to_string(), stack.count));
        None
    }
}

#[test]
fn filtered_splitter_sorts_strictly_and_never_spills_into_the_wrong_lane() {
    let mut entity = ModBlockEntity::default();
    entity.id = SPLITTER.into();
    entity.items = vec![Some(ItemStack::new(COPPER, 3))];
    logistics_controls::set_filter(&mut entity, Some(COPPER));
    let mut world = Junction { blocked: Some(Direction::West), ..Default::default() };
    split(&mut world, [0; 3], Direction::North, &mut entity, 2);
    assert!(world.received.is_empty());
    assert_eq!(entity.items[0].as_ref().unwrap().count, 3);
    world.blocked = None;
    split(&mut world, [0; 3], Direction::North, &mut entity, 2);
    assert!(world.received.iter().all(|(side, _, _)| *side == Direction::West));
    entity.items[0] = Some(ItemStack::new(IRON, 2));
    world.received.clear();
    split(&mut world, [0; 3], Direction::North, &mut entity, 2);
    assert!(world.received.iter().all(|(side, _, _)| *side == Direction::East));
}

#[test]
fn splitter_priority_uses_overflow_and_balanced_mode_still_alternates() {
    let mut entity = ModBlockEntity::default();
    entity.id = SPLITTER.into();
    entity.items = vec![Some(ItemStack::new(COPPER, 8))];
    set_number(&mut entity, "RoutingPriority", 1);
    let mut world = Junction::default();
    split(&mut world, [0; 3], Direction::North, &mut entity, 2);
    assert_eq!(world.received.iter().map(|(side, _, _)| *side).collect::<Vec<_>>(), [Direction::West, Direction::West]);
    world.blocked = Some(Direction::West);
    world.received.clear();
    split(&mut world, [0; 3], Direction::North, &mut entity, 2);
    assert!(world.received.iter().all(|(side, _, _)| *side == Direction::East));
    world.blocked = None;
    world.received.clear();
    set_number(&mut entity, "RoutingPriority", 0);
    split(&mut world, [0; 3], Direction::North, &mut entity, 2);
    assert_ne!(world.received[0].0, world.received[1].0);
}

#[test]
fn merger_priority_drains_the_selected_source_before_the_other() {
    for (priority, expected) in [(1, COPPER), (2, IRON)] {
        let mut entity = loaded();
        set_number(&mut entity, "RoutingPriority", priority);
        let mut outlet = Outlet { room: 4, ..Default::default() };
        merge(&mut outlet, [0; 3], Direction::North, &mut entity, 2);
        assert_eq!(outlet.received, [expected, expected]);
        merge(&mut outlet, [0; 3], Direction::North, &mut entity, 2);
        assert_eq!(outlet.received.len(), 4);
        assert_ne!(outlet.received[2], outlet.received[3]);
    }
}

#[test]
fn redstone_pauses_belts_without_consuming_and_success_counts_the_real_flow() {
    let mut world = RouteWorld::default();
    world.put([0; 3], BELT, "south");
    let mut entity = belt();
    world.signal = 15;
    transfer_outputs(&mut world, [0; 3], &mut entity, 0);
    update_status(&mut entity, 0);
    assert_eq!(number(&entity, "Status"), 9);
    assert_eq!(entity.items[0].as_ref().unwrap().count, 4);
    world.signal = 0;
    transfer_outputs(&mut world, [0; 3], &mut entity, 8);
    assert_eq!(number(&entity, "Produced"), 1);
    assert_eq!(world.delivered.len(), 1);
    assert_eq!(entity.items[0].as_ref().unwrap().count, 3);
}

#[test]
fn an_arriving_item_waiting_for_its_turn_counts_as_moving() {
    let mut entity = belt();
    set_number(&mut entity, "Status", 0);
    set_number(&mut entity, "MovedAt", 0);
    update_status(&mut entity, 1000);
    assert_eq!(number(&entity, "Status"), 1, "acabou de chegar: andando, sem luz laranja");
    update_status(&mut entity, 1004);
    assert_eq!(number(&entity, "Status"), 1, "ainda esperando a vez");
    update_status(&mut entity, 1009);
    assert_eq!(number(&entity, "Status"), 3, "parado de verdade: bloqueada");
    entity.items = vec![None; entity.items.len()];
    update_status(&mut entity, 1010);
    assert_eq!(number(&entity, "Status"), 0);
}
