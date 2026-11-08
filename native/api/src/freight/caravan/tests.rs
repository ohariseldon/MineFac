use super::*;
use horizon_client::mod_api::{EntityRef, LevelRef};
use std::collections::HashMap;

static SERIAL: Mutex<()> = Mutex::new(());
const A: [i32; 3] = [0, 64, 0];
const B: [i32; 3] = [64, 64, 0];
const RAW: &str = "minecraft:raw_copper";
const UUID: [i32; 4] = [77, 88, 99, 100];

#[derive(Default)]
struct World {
    nodes: HashMap<[i32; 3], ModBlockEntity>,
    mobs: HashMap<u32, ([f64; 3], Compound)>,
}
impl Host for World {
    fn dimension(&self) -> &'static str { "veio:caravan_test" }
    fn level(&self, _: &str) -> Option<LevelRef> { Some(LevelRef::new(self.dimension())) }
    fn entity(&self, id: u32) -> Option<EntityRef> { self.mobs.contains_key(&id).then(|| EntityRef::new(id)) }
    fn entity_position(&self, entity: EntityRef) -> Option<[f64; 3]> { self.mobs.get(&entity.id()).map(|m| m.0) }
    fn entity_nbt(&mut self, entity: EntityRef) -> Option<Compound> { self.mobs.get(&entity.id()).map(|m| m.1.clone()) }
    fn neighbor_signal(&mut self, _: &str, _: [i32; 3]) -> i32 { 0 }
    fn change_mod_block_entity(&mut self, pos: [i32; 3], f: &mut dyn FnMut(&mut ModBlockEntity)) -> bool {
        if let Some(node) = self.nodes.get_mut(&pos) { f(node); true } else { false }
    }
    fn insert_item(&mut self, pos: [i32; 3], _: Direction, mut stack: ItemStack) -> Option<ItemStack> {
        if let Some(node) = self.nodes.get_mut(&pos) {
            for i in 3..6 {
                if !accepts_delivery(node, i, &stack) { continue; }
                let room = match node.items[i].as_ref() {
                    None => 64, Some(s) if is_plain(s, &stack.item) => 64 - s.count, _ => 0,
                };
                let count = room.min(stack.count);
                if count > 0 { add(&mut node.items[i], &stack.item, count); stack.count -= count; }
                if stack.count == 0 { return None; }
            }
        }
        Some(stack)
    }
}
fn station() -> ModBlockEntity {
    let mut station = ModBlockEntity::default(); station.id = ID.into(); station.items = vec![None; 8];
    set_number(&mut station, "FreightChannel", 1);
    station.nbt.insert(nbt_key("CaravanAnimal").clone(), Tag::IntArray(UUID.to_vec()));
    set_text(&mut station, "FreightExport", Some(RAW));
    station
}
fn animal_nbt(kind: Kind) -> Compound {
    let mut nbt = Compound::from([("UUID".into(), Tag::IntArray(UUID.to_vec())), ("Age".into(), Tag::Int(0))]);
    nbt.insert("Tame".into(), Tag::Byte(1));
    nbt.insert("Saddle".into(), Tag::Byte(1));
    if kind == Kind::Donkey { nbt.insert("ChestedHorse".into(), Tag::Byte(1)); }
    nbt
}
fn seen(world: &mut World, id: u32, kind: Kind, at: [f64; 3]) {
    world.mobs.entry(id).and_modify(|m| m.0 = at).or_insert((at, animal_nbt(kind)));
    let mut animals = ANIMALS.lock().unwrap();
    animals.retain(|a| a.id != id);
    animals.push(Animal { epoch: 1, dimension: world.dimension().into(), id, kind, at, seen: defense::clock() });
}
fn setup(kind: Kind) -> (World, ModBlockEntity, Endpoint) {
    ANIMALS.lock().unwrap().clear(); CLAIMS.lock().unwrap().clear();
    let mut world = World::default(); seen(&mut world, 9090, kind, center(A));
    let mut a = station(); a.items[0] = Some(ItemStack::new(RAW, 64));
    a.items[FEED] = Some(ItemStack::new(if kind == Kind::Pig { "minecraft:carrot" } else { "minecraft:wheat" }, 2));
    let mut b = station(); b.nbt.remove(nbt_key("CaravanAnimal"));
    set_text(&mut b, "FreightExport", Some(PLATE)); set_text(&mut b, "FreightImport", Some(RAW));
    world.nodes.insert(B, b);
    let partner = Endpoint { epoch: 1, dimension: world.dimension().into(), pos: B, kind: ID.into(),
        channel: 1, export: Some(PLATE.into()), seen: defense::clock() };
    (world, a, partner)
}
fn received(world: &World) -> u32 { world.nodes[&B].items[3..6].iter().flatten().map(|s| s.count).sum() }

#[test]
fn species_have_different_roles_and_only_deliver_after_physical_arrival() {
    let _guard = SERIAL.lock().unwrap();
    for (kind, count) in [(Kind::Horse, 16), (Kind::Donkey, 64), (Kind::Pig, 8)] {
        let (mut world, mut a, partner) = setup(kind);
        tick(&mut world, A, &mut a, Some(&partner), 1);
        assert_eq!(a.items[CARGO].as_ref().unwrap().count, count);
        assert_eq!(a.items[FEED].as_ref().unwrap().count, 1);
        assert_eq!(received(&world), 0);
        assert_eq!(workforce::order_for(9090, defense::clock()), Some(center(B)));
        for _ in 0..10 { tick(&mut world, A, &mut a, Some(&partner), 1); }
        assert_eq!(received(&world), 0, "a timer cannot replace real travel");
        seen(&mut world, 9090, kind, center(B));
        tick(&mut world, A, &mut a, Some(&partner), 1);
        assert_eq!(received(&world), count);
        assert_eq!(number(&a, "Total"), 1);
        assert!(a.items[CARGO].is_none());
        assert_eq!(a.items[FEED].as_ref().unwrap().count, 1, "return does not eat again");
        tick(&mut world, A, &mut a, Some(&partner), 1);
        assert_eq!(workforce::order_for(9090, defense::clock()), Some(center(A)));
        seen(&mut world, 9090, kind, center(A));
        tick(&mut world, A, &mut a, Some(&partner), 1);
        assert_eq!(number(&a, "FlightPhase"), 0);
    }
    assert!(Kind::Horse.speed() > Kind::Donkey.speed());
    assert!(Kind::Donkey.speed() > Kind::Pig.speed());
}

#[test]
fn save_rebinds_by_uuid_and_never_delivers_a_second_copy() {
    let _guard = SERIAL.lock().unwrap();
    let (mut world, mut a, partner) = setup(Kind::Donkey);
    tick(&mut world, A, &mut a, Some(&partner), 1);
    let mut saved = station(); saved.nbt = a.nbt.clone(); saved.items = a.items.clone();
    world.mobs.remove(&9090);
    seen(&mut world, 9191, Kind::Donkey, center(B));
    tick(&mut world, A, &mut saved, Some(&partner), 1);
    tick(&mut world, A, &mut saved, Some(&partner), 1);
    assert_eq!(received(&world), 64);
    assert_eq!(number(&saved, "Total"), 1);
}

#[test]
fn missing_animal_wrong_food_and_occupied_mount_never_take_more_cargo() {
    let _guard = SERIAL.lock().unwrap();
    let (mut world, mut a, partner) = setup(Kind::Pig);
    a.items[FEED] = Some(ItemStack::new("minecraft:wheat", 2));
    tick(&mut world, A, &mut a, Some(&partner), 1);
    assert_eq!(number(&a, "Status"), super::super::NO_FEED);
    assert!(a.items[CARGO].is_none());
    a.items[FEED] = Some(ItemStack::new("minecraft:carrot", 2));
    world.mobs.get_mut(&9090).unwrap().1.insert("Passengers".into(), Tag::List(vec![Tag::Compound(Compound::default())]));
    tick(&mut world, A, &mut a, Some(&partner), 1);
    assert_eq!(number(&a, "Status"), super::super::ANIMAL_BUSY);
    assert!(a.items[CARGO].is_none());
    world.mobs.get_mut(&9090).unwrap().1.remove(&"Passengers".into());
    tick(&mut world, A, &mut a, Some(&partner), 1);
    world.mobs.clear();
    tick(&mut world, A, &mut a, Some(&partner), 1);
    assert_eq!(number(&a, "Status"), super::super::NO_ANIMAL);
    assert_eq!(a.items[CARGO].as_ref().unwrap().count, 8);
    assert_eq!(a.items[FEED].as_ref().unwrap().count, 1);
    assert_eq!(received(&world), 0);
}

#[test]
fn one_animal_cannot_serve_two_stations_and_wild_or_baby_animals_are_not_selected() {
    let _guard = SERIAL.lock().unwrap();
    let (mut world, _a, _partner) = setup(Kind::Donkey);
    let mut first = station(); first.nbt.remove(nbt_key("CaravanAnimal"));
    let mut other = station(); other.nbt.remove(nbt_key("CaravanAnimal"));
    world.nodes.insert(A, first); world.nodes.insert([2, 64, 0], other);
    assign(&mut world, A, false);
    assign(&mut world, [2, 64, 0], false);
    assert_eq!(uuid(&world.nodes[&A]), Some(UUID));
    assert_eq!(uuid(&world.nodes[&[2, 64, 0]]), None);
    let mut nbt = animal_nbt(Kind::Donkey);
    nbt.insert("Age".into(), Tag::Int(-10)); assert!(!free(&nbt, Kind::Donkey));
    nbt.insert("Age".into(), Tag::Int(0)); nbt.insert("Tame".into(), Tag::Byte(0));
    assert!(!free(&nbt, Kind::Donkey));
    nbt.insert("Tame".into(), Tag::Byte(1)); nbt.remove(&"ChestedHorse".into());
    assert!(!free(&nbt, Kind::Donkey));
    let mut leashed = animal_nbt(Kind::Donkey);
    leashed.insert("leash".into(), Tag::Compound(Compound::default()));
    assert!(!free(&leashed, Kind::Donkey), "current Horizon leash schema is lowercase");
}

#[test]
fn partial_unloading_and_emergency_release_preserve_all_goods() {
    let _guard = SERIAL.lock().unwrap();
    let (mut world, mut a, partner) = setup(Kind::Donkey);
    tick(&mut world, A, &mut a, Some(&partner), 1);
    for i in 3..6 { world.nodes.get_mut(&B).unwrap().items[i] = Some(ItemStack::new(RAW, 64)); }
    world.nodes.get_mut(&B).unwrap().items[3].as_mut().unwrap().count = 54;
    seen(&mut world, 9090, Kind::Donkey, center(B));
    tick(&mut world, A, &mut a, Some(&partner), 1);
    assert_eq!(a.items[CARGO].as_ref().unwrap().count, 54);
    assert_eq!(number(&a, "Produced"), 10);
    world.nodes.insert(A, a);
    assign(&mut world, A, true);
    let a = &world.nodes[&A];
    assert_eq!(a.items[0].as_ref().unwrap().count, 54);
    assert!(a.items[CARGO].is_none());
    assert!(uuid(a).is_none());
    assert_eq!(received(&world), 192);
}

#[test]
fn companion_saddles_are_recognized_in_both_supported_nbt_forms() {
    let mut nbt = Compound::from([("Saddle".into(), Tag::Byte(1))]);
    assert!(saddled(&nbt));
    nbt.clear();
    nbt.insert("equipment".into(), Tag::Compound(Compound::from([("saddle".into(), Tag::Compound(Compound::default()))])));
    assert!(saddled(&nbt));
    assert!(!saddled(&Compound::default()));
}


#[test]
fn a_player_mounting_the_animal_takes_priority_over_the_transport_goal() {
    use horizon_client::mobs::{Ai, Extra, MobType, MobWorld, Vec3};
    let mut mob = Mob { id: 98002, rider: None, extra: Extra::default(),
        kind: std::sync::Arc::new(MobType { id: "minecraft:horse".into() }), position: Vec3::default(), velocity: Vec3::default(), on_climbable: false, ai: Ai::default() };
    let world = MobWorld::fixture(); let mut cx = Cx { w: &world }; let mut travel = Travel::default();
    workforce::order(mob.id, [64.0, 64.0, 0.0], defense::clock());
    assert!(travel.can_use(&mut mob, &mut cx));
    mob.rider = Some(());
    assert!(!travel.can_use(&mut mob, &mut cx));
    mob.ai.nav.done = false; travel.stop(&mut mob, &mut cx);
    assert!(mob.ai.nav.is_done());
    workforce::cancel(mob.id);
}
