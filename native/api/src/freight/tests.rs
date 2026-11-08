use super::*;
use horizon_client::mod_api::LevelRef;
use std::collections::HashMap;

static SERIAL: Mutex<()> = Mutex::new(());
const A: [i32; 3] = [0, 64, 0];
const B: [i32; 3] = [16, 64, 0];
const RAW: &str = "minecraft:raw_copper";

fn port(kind: &str, item: Option<&str>) -> ModBlockEntity {
    let mut port = ModBlockEntity::default();
    port.id = kind.into();
    port.items = vec![None; layout(kind).unwrap().0];
    set_number(&mut port, "FreightChannel", 1);
    set_number(&mut port, "energy", 8000);
    set_text(&mut port, "FreightExport", item);
    port
}

#[derive(Default)]
struct World { nodes: HashMap<[i32; 3], ModBlockEntity>, held: Option<String>, paused: bool, obstructed: bool, unloaded: bool, normal: bool }
impl Host for World {
    fn dimension(&self) -> &'static str { "veio:freight_test" }
    fn level(&self, _: &str) -> Option<LevelRef> { Some(LevelRef::new(self.dimension())) }
    fn level_is_loaded(&self, _: LevelRef, _: [i32; 3]) -> bool { !self.unloaded }
    fn level_block(&self, _: LevelRef, _: [i32; 3]) -> Option<String> { Some(if self.obstructed { "minecraft:stone" } else { "minecraft:air" }.into()) }
    fn neighbor_signal(&mut self, _: &str, _: [i32; 3]) -> i32 { i32::from(self.paused) }
    fn held(&self) -> Option<(String, u32, bool)> { self.held.clone().map(|s| (s, 1, false)) }
    fn sneaking(&self) -> bool { !self.normal }
    fn block_entity_sync(&mut self, _: [i32; 3]) {}
    fn change_mod_block_entity(&mut self, at: [i32; 3], f: &mut dyn FnMut(&mut ModBlockEntity)) -> bool {
        if let Some(node) = self.nodes.get_mut(&at) { f(node); true } else { false }
    }
    fn insert_item(&mut self, at: [i32; 3], _: Direction, mut stack: ItemStack) -> Option<ItemStack> {
        if let Some(node) = self.nodes.get_mut(&at) {
            for slot in 0..6 {
                if !accepts_delivery(node, slot, &stack) { continue; }
                let room = match node.items[slot].as_ref() {
                    None => 64,
                    Some(old) if is_plain(old, &stack.item) => 64 - old.count,
                    _ => 0,
                };
                let count = room.min(stack.count);
                if count > 0 { add(&mut node.items[slot], &stack.item, count); stack.count -= count; }
                if stack.count == 0 { return None; }
            }
        }
        Some(stack)
    }
}

fn pair() -> (World, ModBlockEntity) {
    PORTS.lock().unwrap().clear();
    let mut world = World::default();
    let mut a = port(DRONE, Some(RAW));
    let mut b = port(DRONE, Some(PLATE));
    tick(&mut world, B, &mut b, 0);
    world.nodes.insert(B, b);
    tick(&mut world, A, &mut a, 0);
    let mut b = world.nodes.remove(&B).unwrap();
    tick(&mut world, B, &mut b, 0);
    world.nodes.insert(B, b);
    (world, a)
}

fn advance(world: &mut World, a: &mut ModBlockEntity, ticks: i64) {
    for t in 21..21 + ticks { tick(world, A, a, t); }
}

fn received(world: &World) -> u32 {
    world.nodes[&B].items[3..6].iter().flatten().map(|s| s.count).sum()
}

#[test]
fn dispatch_and_received_stock_have_different_jobs() {
    let _guard = SERIAL.lock().unwrap();
    let (world, a) = pair();
    assert!(accepts_plan(&a, 0, &ItemStack::new(RAW, 1)));
    assert!(!accepts_plan(&a, 3, &ItemStack::new(PLATE, 1)));
    assert!(accepts_delivery(&a, 3, &ItemStack::new(PLATE, 1)));
    assert!(!accepts_delivery(&a, 3, &ItemStack::new(RAW, 1)));
    assert!(!accepts_delivery(&a, CARGO, &ItemStack::new(RAW, 1)));
    assert_eq!(super::super::output_slots(RAIL), 3..6);
    assert_eq!(received(&world), 0);
}

#[test]
fn saved_flight_delivers_once_and_pays_for_both_legs_once() {
    let _guard = SERIAL.lock().unwrap();
    let (mut world, mut a) = pair();
    a.items[0] = Some(ItemStack::new(RAW, 16));
    tick(&mut world, A, &mut a, 20);
    assert_eq!(a.items[CARGO], Some(ItemStack::new(RAW, 16)));
    assert!(a.items[0].is_none());
    assert_eq!(received(&world), 0, "cargo cannot arrive at departure");
    let energy = number(&a, "energy");
    assert_eq!(energy, 8000 - 800 - 16 * 32);
    let mut restored = port(DRONE, None);
    restored.nbt = a.nbt.clone(); restored.items = a.items.clone();
    advance(&mut world, &mut restored, 160);
    assert_eq!(received(&world), 16);
    assert!(restored.items[CARGO].is_none());
    assert_eq!(number(&restored, "Total"), 1);
    assert_eq!(number(&restored, "Produced"), 16);
    assert_eq!(number(&restored, "energy"), energy);
    assert_eq!(number(&restored, "FlightPhase"), 0);
}

#[test]
fn full_or_unloaded_destination_keeps_cargo_and_partial_delivery_conserves_it() {
    let _guard = SERIAL.lock().unwrap();
    let (mut world, mut a) = pair();
    a.items[0] = Some(ItemStack::new(RAW, 16));
    tick(&mut world, A, &mut a, 20);
    let mut b = world.nodes.remove(&B).unwrap();
    advance(&mut world, &mut a, 80);
    assert_eq!(a.items[CARGO].as_ref().unwrap().count, 16);
    for slot in 3..6 { b.items[slot] = Some(ItemStack::new(RAW, 64)); }
    world.nodes.insert(B, b);
    advance(&mut world, &mut a, 1);
    assert_eq!(number(&a, "Status"), 3);
    world.nodes.get_mut(&B).unwrap().items[3].as_mut().unwrap().count -= 10;
    advance(&mut world, &mut a, 1);
    assert_eq!(a.items[CARGO].as_ref().unwrap().count, 6);
    assert_eq!(number(&a, "Produced"), 10);
    world.nodes.get_mut(&B).unwrap().items[3] = None;
    advance(&mut world, &mut a, 1);
    assert!(a.items[CARGO].is_none());
    assert_eq!(number(&a, "Produced"), 16);
    assert_eq!(received(&world), 134);
}

#[test]
fn insufficient_energy_and_redstone_never_remove_stock() {
    let _guard = SERIAL.lock().unwrap();
    let (mut world, mut a) = pair();
    a.items[0] = Some(ItemStack::new(RAW, 16));
    set_number(&mut a, "energy", 1);
    tick(&mut world, A, &mut a, 20);
    assert_eq!(number(&a, "Status"), 5);
    assert_eq!(a.items[0].as_ref().unwrap().count, 16);
    world.paused = true;
    set_number(&mut a, "energy", 8000);
    tick(&mut world, A, &mut a, 40);
    assert_eq!(number(&a, "Status"), 9);
    assert!(a.items[CARGO].is_none());
}

#[test]
fn same_goods_in_both_directions_or_ambiguous_channel_do_not_launch() {
    let _guard = SERIAL.lock().unwrap();
    let (mut world, mut a) = pair();
    let now = defense::clock();
    register(Endpoint { epoch: 1, dimension: world.dimension().into(), pos: B,
        kind: DRONE.into(), channel: 1, export: Some(RAW.into()), seen: now });
    a.items[0] = Some(ItemStack::new(RAW, 16));
    tick(&mut world, A, &mut a, 20);
    assert!(a.items[CARGO].is_none());
    register(Endpoint { epoch: 1, dimension: world.dimension().into(), pos: [32, 64, 0],
        kind: DRONE.into(), channel: 1, export: Some(PLATE.into()), seen: now });
    tick(&mut world, A, &mut a, 40);
    assert!(a.items[CARGO].is_none());
}

#[test]
fn cancelling_a_flight_recovers_cargo_instead_of_overwriting_it() {
    let _guard = SERIAL.lock().unwrap();
    let (mut world, mut a) = pair();
    a.items[0] = Some(ItemStack::new(RAW, 16));
    tick(&mut world, A, &mut a, 20);
    world.held = Some(WRENCH.into());
    world.nodes.insert(A, a);
    right_click(&mut world, A, DRONE);
    let mut a = world.nodes.remove(&A).unwrap();
    // Return stock must wait if all dispatch slots were refilled during flight.
    for slot in 0..3 { a.items[slot] = Some(ItemStack::new(RAW, 64)); }
    advance(&mut world, &mut a, 60);
    assert_eq!(a.items[CARGO].as_ref().unwrap().count, 16);
    assert_eq!(number(&a, "Status"), 3);
    a.items[0] = None;
    advance(&mut world, &mut a, 1);
    assert!(a.items[CARGO].is_none());
    assert_eq!(a.items[0].as_ref().unwrap().count, 16);
    assert_eq!(received(&world), 0);
}

#[test]
fn endpoint_registry_separates_worlds_dimensions_and_transport_modes() {
    let _guard = SERIAL.lock().unwrap();
    PORTS.lock().unwrap().clear();
    let endpoint = Endpoint { epoch: 100, dimension: "a".into(), pos: A,
        kind: DRONE.into(), channel: 2, export: Some(RAW.into()), seen: 0 };
    assert!(register(endpoint.clone()).is_none());
    let mut other = endpoint.clone(); other.pos = B; other.kind = RAIL.into();
    assert!(register(other.clone()).is_none());
    other.kind = DRONE.into(); other.dimension = "b".into();
    assert!(register(other.clone()).is_none());
    other.dimension = "a".into(); other.epoch = 101;
    assert!(register(other).is_none());
}


#[test]
fn an_obstacle_stops_the_drone_without_spending_or_delivering_the_cargo() {
    let _guard = SERIAL.lock().unwrap();
    let (mut world, mut a) = pair();
    a.items[0] = Some(ItemStack::new(RAW, 16));
    tick(&mut world, A, &mut a, 20);
    let energy = number(&a, "energy");
    world.obstructed = true;
    advance(&mut world, &mut a, 80);
    assert_eq!(number(&a, "Progress"), 0);
    assert_eq!(number(&a, "Status"), 22);
    assert_eq!(a.items[CARGO].as_ref().unwrap().count, 16);
    assert_eq!(number(&a, "energy"), energy);
    assert_eq!(received(&world), 0);
    world.obstructed = false;
    advance(&mut world, &mut a, 160);
    assert_eq!(received(&world), 16);
}


#[test]
fn unloaded_path_waits_without_advancing_a_saved_flight() {
    let _guard = SERIAL.lock().unwrap();
    let (mut world, mut a) = pair();
    a.items[0] = Some(ItemStack::new(RAW, 16));
    tick(&mut world, A, &mut a, 20);
    world.unloaded = true;
    advance(&mut world, &mut a, 80);
    assert_eq!(number(&a, "Progress"), 0);
    assert_eq!(a.items[CARGO].as_ref().unwrap().count, 16);
    assert_eq!(received(&world), 0);
    world.unloaded = false;
    advance(&mut world, &mut a, 160);
    assert_eq!(received(&world), 16);
}

#[test]
fn excessive_range_never_launches_or_consumes_energy() {
    let _guard = SERIAL.lock().unwrap();
    let (mut world, mut a) = pair();
    PORTS.lock().unwrap().retain(|p| p.pos != B);
    register(Endpoint { epoch: 1, dimension: world.dimension().into(), pos: [RANGE + 1, 64, 0],
        kind: DRONE.into(), channel: 1, export: Some(PLATE.into()), seen: defense::clock() });
    a.items[0] = Some(ItemStack::new(RAW, 16));
    tick(&mut world, A, &mut a, 20);
    assert_eq!(number(&a, "energy"), 8000);
    assert!(a.items[CARGO].is_none());
    assert_eq!(a.items[0].as_ref().unwrap().count, 16);
}

#[test]
fn configuration_survives_save_and_never_consumes_a_sample_or_card() {
    let _guard = SERIAL.lock().unwrap();
    let mut world = World::default();
    world.nodes.insert(A, port(RAIL, None));
    world.held = Some(RAW.into());
    right_click(&mut world, A, RAIL);
    assert_eq!(world.held().unwrap().1, 1);
    assert!(world.nodes[&A].items.iter().all(Option::is_none));
    world.held = Some(CARD.into());
    right_click(&mut world, A, RAIL); // Sneaking selects the previous channel.
    let old = &world.nodes[&A];
    let mut restored = port(RAIL, None); restored.nbt = old.nbt.clone();
    assert_eq!(channel(&restored), 0);
    assert_eq!(export(&restored).as_deref(), Some(RAW));
    assert_eq!(world.held().unwrap().1, 1);
}

#[test]
fn a_rail_terminal_keeps_its_receive_filter_when_the_remote_zone_unloads() {
    let _guard = SERIAL.lock().unwrap();
    PORTS.lock().unwrap().clear();
    let mut world = World::default();
    let mut station = port(RAIL, Some(RAW));
    set_text(&mut station, "FreightImport", Some(PLATE));
    tick(&mut world, A, &mut station, 20);
    assert_eq!(text(&station, "FreightImport").as_deref(), Some(PLATE));
    assert!(accepts_delivery(&station, 3, &ItemStack::new(PLATE, 1)));
    assert!(!accepts_plan(&station, 3, &ItemStack::new(PLATE, 1)));
    assert_eq!(number(&station, "Status"), 27);
}


#[test]
fn matching_dyes_connect_two_ports_without_cycling_the_card_or_consuming_dye() {
    let _guard = SERIAL.lock().unwrap();
    let mut world = World::default(); world.normal = true;
    world.nodes.insert(A, port(DRONE, None)); world.nodes.insert(B, port(DRONE, None));
    world.held = Some("minecraft:blue_dye".into());
    right_click(&mut world, A, DRONE); right_click(&mut world, B, DRONE);
    assert_eq!(channel(&world.nodes[&A]), channel(&world.nodes[&B]));
    assert_eq!(channel(&world.nodes[&A]), 12);
    assert_eq!(world.held().unwrap().1, 1);
    assert!(export(&world.nodes[&A]).is_none(), "color setup is not a cargo request");
}

#[test]
fn drone_render_extrapolation_is_bounded_and_freezes_on_pause_or_obstruction() {
    let mut drone = port(DRONE,None);
    for (key,value) in [("Status",1),("FlightPhase",1),("Progress",10),("Duration",40),("FlightVisualAt",100)] { set_number(&mut drone,key,value); }
    assert_eq!(visual_progress(&drone,102,0.5),12.5);
    assert_eq!(visual_progress(&drone,99,0.5),10.5); // a client clock behind the snapshot cannot jump forward
    assert_eq!(visual_progress(&drone,1000,0.5),14.5); // no unlimited dead reckoning
    for status in [3,9,22] {
        set_number(&mut drone,"Status",status);
        assert_eq!(visual_progress(&drone,102,0.5),10.0);
    }
    set_number(&mut drone,"Status",1);
    set_number(&mut drone,"FlightVisualAt",i32::MAX-2);
    assert_eq!(visual_progress(&drone,i64::from(i32::MAX)+1,0.5),13.5);
    drone.nbt.remove(nbt_key("FlightVisualAt"));
    assert_eq!(visual_progress(&drone,102,0.5),10.5); // legacy world has no timestamp
}
