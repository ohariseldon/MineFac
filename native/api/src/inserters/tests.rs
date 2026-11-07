use super::*;

struct World {
    facing: &'static str,
    signal: i32,
    source: u32,
    target: u32,
    accepts: bool,
    attempts: Vec<([i32; 3], Direction, [i32; 3], Direction)>,
}

impl Default for World {
    fn default() -> Self {
        Self { facing: "south", signal: 0, source: 5, target: 0, accepts: true, attempts: Vec::new() }
    }
}

impl Host for World {
    fn dimension(&self) -> &'static str { "minecraft:overworld" }
    fn neighbor_signal(&mut self, _dimension: &str, _pos: [i32; 3]) -> i32 { self.signal }
    fn block_property(&self, _pos: [i32; 3], key: &str) -> Option<String> {
        (key == "facing").then(|| self.facing.into())
    }
    fn move_items(&mut self, dimension: &str, from: [i32; 3], from_face: Direction,
        to: [i32; 3], to_face: Direction, item: Option<&str>, max: u32) -> u32 {
        assert_eq!(dimension, "minecraft:overworld");
        assert!(item.is_none());
        assert_eq!(max, 1);
        self.attempts.push((from, from_face, to, to_face));
        if self.source == 0 || self.target == 64 || !self.accepts { return 0; }
        self.source -= 1;
        self.target += 1;
        1
    }
}

fn arm(energy: i32) -> ModBlockEntity {
    let mut entity = ModBlockEntity::default();
    entity.id = ID.into();
    entity.items = vec![None];
    set_number(&mut entity, "energy", energy);
    entity
}

#[test]
fn accepted_transfer_costs_once_and_does_not_repeat_during_animation() {
    let mut world = World::default();
    let mut entity = arm(100);
    for time in 0..20 { tick(&mut world, [0; 3], &mut entity, time); }
    assert_eq!((world.source, world.target), (4, 1));
    assert_eq!(number(&entity, "energy"), 60);
    assert_eq!(number(&entity, "Produced"), 1);
    assert_eq!(world.attempts.len(), 1);
    assert!(entity.items[0].is_none());
}

#[test]
fn full_filtered_and_empty_endpoints_preserve_items_and_energy() {
    for mut world in [World { target: 64, ..Default::default() },
        World { accepts: false, ..Default::default() }, World { source: 0, ..Default::default() }] {
        let before = world.source + world.target;
        let mut entity = arm(100);
        tick(&mut world, [0; 3], &mut entity, 0);
        assert_eq!(world.source + world.target, before);
        assert_eq!(number(&entity, "energy"), 100);
        assert_eq!(number(&entity, "Produced"), 0);
        assert_eq!(number(&entity, "Status"), 0);
    }
}

#[test]
fn insufficient_power_and_redstone_do_not_touch_inventories() {
    let mut world = World::default();
    let mut entity = arm(COST - 1);
    tick(&mut world, [0; 3], &mut entity, 0);
    assert_eq!(number(&entity, "Status"), 5);
    set_number(&mut entity, "energy", 100);
    world.signal = 15;
    tick(&mut world, [0; 3], &mut entity, 20);
    assert_eq!(number(&entity, "Status"), 9);
    assert!(world.attempts.is_empty());
    assert_eq!(number(&entity, "energy"), 100);
}

#[test]
fn all_four_directions_use_the_correct_opposing_container_faces() {
    for (facing, output) in [("south", Direction::North), ("north", Direction::South),
        ("east", Direction::West), ("west", Direction::East)] {
        let mut world = World { facing, ..Default::default() };
        tick(&mut world, [0; 3], &mut arm(100), 0);
        let [dx, _, dz] = output.normal();
        assert_eq!(world.attempts, [([-dx, 0, -dz], output, [dx, 0, dz], output.opposite())]);
    }
}

#[test]
fn restoring_the_saved_animation_never_replays_an_already_completed_transfer() {
    let mut world = World::default();
    let mut entity = arm(100);
    tick(&mut world, [0; 3], &mut entity, 0);
    let mut restored = arm(0);
    restored.nbt = entity.nbt.clone();
    for time in 1..20 { tick(&mut world, [0; 3], &mut restored, time); }
    assert_eq!(world.attempts.len(), 1);
    assert_eq!((world.source, world.target), (4, 1));
    assert_eq!(number(&restored, "energy"), 60);
    assert_eq!(number(&restored, "Produced"), 1);
}

#[test]
fn arms_are_powered_transport_without_supply_or_haul_slots() {
    let entity = arm(100);
    assert!(needs_power(ID));
    assert!(!has_item_inputs(ID));
    assert!(output_slots(ID).is_empty());
    let view = operations::inspect(&entity).unwrap();
    assert_eq!(view.profile.kind, operations::StationKind::Transport);
    assert!(view.services.is_empty());
    assert!(!accepts(ID, 0, &ItemStack::new(PLATE, 1)));
}
