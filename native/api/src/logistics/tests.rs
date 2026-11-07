use super::*;

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
