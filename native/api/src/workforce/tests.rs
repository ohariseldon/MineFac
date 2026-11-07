//! Testes dos trabalhadores: atribuição exclusiva, ciclo físico de colheita, conservação de
//! itens, interrupções e restauração depois de reabrir o mundo.
//!
//! Cada teste usa sua própria dimensão, UUIDs e ids, porque os registros são globais.
use super::*;
use horizon_client::mod_api::{BlockStateId, EntityRef, NativeItemEntity};
use std::cell::Cell;

const POST: [i32; 3] = [0, 64, 0];
/// Fora do alcance de entrega do posto: colher exige ir até lá.
const WHEAT: [i32; 3] = [3, 64, 3];
/// Primeira volta do carregador (507 / 16) com folga.
const HAULER_SWEEP: u64 = 48;
/// Primeira volta do capataz (1875 / 25) com folga.
const FOREMAN_SWEEP: u64 = 90;

#[derive(Default)]
struct Village {
    dimension: &'static str,
    crops: HashMap<[i32; 3], (String, u8)>,
    villagers: HashMap<u32, ([f64; 3], Uuid, i32)>,
    /// Outros blocos com entidade: máquinas e postos atendidos.
    stations: HashMap<[i32; 3], ModBlockEntity>,
    animals: HashMap<u32, ([f64; 3], Compound)>,
    drops: Vec<NativeItemEntity>,
    looks: Cell<u32>,
    edits: Cell<u32>,
}

impl Village {
    fn new(dimension: &'static str) -> Self {
        let mut village = Self { dimension, ..Self::default() };
        village.crops.insert(WHEAT, ("minecraft:wheat".into(), 7));
        village
    }
    fn villager(&mut self, id: u32, at: [f64; 3], uuid: Uuid) {
        self.villagers.insert(id, (at, uuid, 0));
        production::ranch::observe(self.dimension, id, VILLAGER);
    }
    fn walk(&mut self, id: u32, to: [i32; 3]) {
        self.villagers.get_mut(&id).unwrap().0 = [f64::from(to[0]) + 0.5, f64::from(to[1]), f64::from(to[2]) + 0.5];
    }
    fn age(&self, pos: [i32; 3]) -> u8 {
        self.crops[&pos].1
    }
}

impl Host for Village {
    fn dimension(&self) -> &'static str {
        self.dimension
    }
    fn block(&self, pos: [i32; 3]) -> String {
        self.looks.set(self.looks.get() + 1);
        if let Some(station) = self.stations.get(&pos) {
            return station.id.clone();
        }
        self.crops.get(&pos).map_or_else(|| "minecraft:air".to_owned(), |(id, _)| id.clone())
    }
    fn change_mod_block_entity(&mut self, pos: [i32; 3], change: &mut dyn FnMut(&mut ModBlockEntity)) -> bool {
        self.edits.set(self.edits.get() + 1);
        let Some(station) = self.stations.get_mut(&pos) else { return false };
        change(station);
        true
    }
    fn block_property(&self, pos: [i32; 3], key: &str) -> Option<String> {
        (key == "age").then(|| self.crops.get(&pos).map(|(_, age)| age.to_string())).flatten()
    }
    fn set_block_property(&mut self, pos: [i32; 3], key: &str, value: &str) -> bool {
        let Some((_, age)) = self.crops.get_mut(&pos) else { return false };
        let value: u8 = value.parse().unwrap();
        if key != "age" || *age == value { return false; }
        *age = value;
        true
    }
    fn entity(&self, id: u32) -> Option<EntityRef> {
        (self.villagers.contains_key(&id) || self.animals.contains_key(&id)).then(|| EntityRef::new(id))
    }
    fn entity_position(&self, entity: EntityRef) -> Option<[f64; 3]> {
        self.villagers.get(&entity.id()).map(|(at, _, _)| *at)
            .or_else(|| self.animals.get(&entity.id()).map(|(at, _)| *at))
    }
    fn entity_merge_nbt(&mut self, entity: EntityRef, nbt: Compound) -> bool {
        let Some((_, data)) = self.animals.get_mut(&entity.id()) else { return false };
        data.extend(nbt);
        true
    }
    fn item_entities(&self, _: &str, min: [f64; 3], max: [f64; 3]) -> Vec<NativeItemEntity> {
        self.drops.iter().filter(|drop| (0..3).all(|axis| (min[axis]..=max[axis]).contains(&drop.position[axis]))).cloned().collect()
    }
    fn set_item_entity(&mut self, _: &str, id: u32, stack: ItemStack) -> bool {
        let Some(index) = self.drops.iter().position(|drop| drop.id == id) else { return false };
        if stack.count == 0 { self.drops.remove(index); } else { self.drops[index].stack = stack; }
        true
    }
    fn entity_nbt(&mut self, entity: EntityRef) -> Option<Compound> {
        if let Some((_, data)) = self.animals.get(&entity.id()) {
            return Some(data.clone());
        }
        let (_, uuid, age) = self.villagers.get(&entity.id())?;
        Some(Compound::from([("UUID".into(), Tag::IntArray(uuid.to_vec())), ("Age".into(), Tag::Int(*age))]))
    }
    fn send_particles(&mut self, _: &ParticleOptions, _: Option<BlockStateId>, _: [f64; 3], _: [f64; 3], _: f64, _: i32) -> bool {
        true
    }
    fn play_sound(&mut self, _: &str, _: [f64; 3], _: usize, _: f32, _: f32) -> bool {
        true
    }
}

fn post() -> ModBlockEntity {
    block(FARM_POST)
}

fn block(id: &str) -> ModBlockEntity {
    let mut entity = ModBlockEntity::default();
    entity.id = id.to_owned();
    entity.items = vec![None; super::super::layout(id).map_or(0, |(slots, _, _)| slots)];
    entity
}

/// Roda ciclos de um posto em `pos` (na fase dele) até `done` ou `limit` ciclos.
fn cycles_at(village: &mut Village, pos: [i32; 3], entity: &mut ModBlockEntity, from: u64, limit: u64,
    done: impl Fn(&ModBlockEntity) -> bool) -> u64
{
    let phase = i64::from((pos[0] ^ pos[2]) & 7);
    for now in from..from + limit {
        let time = (EVERY - phase).rem_euclid(EVERY) + now as i64 * EVERY;
        step(village, pos, entity, time, now);
        if done(entity) {
            return now;
        }
    }
    panic!("condição não atingida em {limit} ciclos; estado {}", number(entity, "Status"));
}

fn hire_at(village: &mut Village, entity: &mut ModBlockEntity, pos: [i32; 3], now: u64) -> bool {
    let Some(found) = candidate(village, pos, now) else { return false };
    assign(village.dimension, pos, entity, found, now)
}

/// Roda um ciclo do posto (fase 0: x ^ z = 0 em `POST`).
fn cycle(village: &mut Village, entity: &mut ModBlockEntity, now: u64) -> i32 {
    step(village, POST, entity, 0, now);
    number(entity, "Status")
}

fn outputs_of(entity: &ModBlockEntity) -> Vec<(String, u32)> {
    let mut found: Vec<(String, u32)> = entity.items[1..4].iter().flatten()
        .map(|stack| (stack.item.to_string(), stack.count)).collect();
    found.sort();
    found
}

fn goal(pos: [i32; 3]) -> [f64; 3] {
    [f64::from(pos[0]) + 0.5, f64::from(pos[1]), f64::from(pos[2]) + 0.5]
}

#[test]
fn villager_works_for_one_post_only_until_dismissed() {
    let mut village = Village::new("veio:test_work_exclusive");
    village.villager(9101, [3.5, 64.0, 3.5], [1, 0, 0, 1]);
    let mut first = post();
    let mut second = post();
    let other = [4, 64, 0];
    assert!(hire_at(&mut village, &mut first, POST, 0));
    assert_eq!(worker(&first), Some([1, 0, 0, 1]));
    assert!(!hire_at(&mut village, &mut first, POST, 0), "um posto, um trabalhador");
    assert!(!hire_at(&mut village, &mut second, other, 0), "o aldeão já está atribuído");
    assert!(!claim([1, 0, 0, 1], village.dimension, other, 10));
    assert!(dismiss(village.dimension, POST, &mut first));
    assert_eq!(worker(&first), None);
    assert_eq!(number(&first, "Status"), NO_WORKER);
    assert!(hire_at(&mut village, &mut second, other, 20), "dispensado, pode ir para outro posto");
}

#[test]
fn babies_and_far_villagers_are_not_hired() {
    let mut village = Village::new("veio:test_work_candidates");
    village.villager(9201, [20.5, 64.0, 0.5], [2, 0, 0, 1]);
    village.villager(9202, [2.5, 64.0, 2.5], [2, 0, 0, 2]);
    village.villagers.get_mut(&9202).unwrap().2 = -24000;
    let mut entity = post();
    assert!(!hire_at(&mut village, &mut entity, POST, 0));
    assert_eq!(worker(&entity), None);
}

#[test]
fn farmhand_walks_harvests_and_carries_the_crop_home() {
    let mut village = Village::new("veio:test_work_cycle");
    village.villager(9301, [5.5, 64.0, 5.5], [3, 0, 0, 1]);
    let mut entity = post();
    entity.items[0] = Some(ItemStack::new("minecraft:bread", 1));
    assert!(hire_at(&mut village, &mut entity, POST, 0));

    let now = cycles_at(&mut village, POST, &mut entity, 1, 40, |entity| target(entity) == Some(WHEAT));
    assert_eq!(number(&entity, "Status"), WALKING);
    assert_eq!(entity.items[0], None, "comeu o pão");
    assert_eq!(number(&entity, "Ration"), 6);
    assert_eq!(number(&entity, "Info"), 3, "pão vale 3 colheitas");
    assert_eq!(order_for(9301, now), Some(goal(WHEAT)), "o aldeão recebeu a ordem de ir até o trigo");
    assert_eq!(village.age(WHEAT), 7, "longe da planta, nada é colhido");

    village.walk(9301, WHEAT);
    let now = cycles_at(&mut village, POST, &mut entity, now + 1, 5, |entity| !carried(entity).is_empty());
    assert_eq!(village.age(WHEAT), 0, "colhido e replantado");
    assert_eq!(number(&entity, "Ration"), 4);
    assert_eq!(number(&entity, "Info"), 2);
    assert!(outputs_of(&entity).is_empty(), "o produto ainda está com o trabalhador");
    assert_eq!(carried(&entity).len(), 2);
    assert_eq!(order_for(9301, now), Some(goal(POST)), "volta ao posto");

    assert_eq!(cycle(&mut village, &mut entity, now + 1), WALKING, "precisa chegar ao posto para entregar");
    village.walk(9301, [1, 64, 0]);
    assert_eq!(cycle(&mut village, &mut entity, now + 2), 1);
    assert!(carried(&entity).is_empty());
    assert_eq!(outputs_of(&entity), vec![("minecraft:wheat".into(), 1), ("minecraft:wheat_seeds".into(), 1)]);
    assert_eq!(number(&entity, "Total"), 1);

    let idle = cycles_at(&mut village, POST, &mut entity, now + 3, 40, |entity| number(entity, "Status") == NO_TASK);
    assert_eq!(order_for(9301, idle), None, "sem tarefa, o aldeão volta ao comportamento vanilla");
}

#[test]
fn hungry_worker_waits_without_touching_the_field() {
    let mut village = Village::new("veio:test_work_food");
    village.villager(9401, [3.5, 64.0, 3.5], [4, 0, 0, 1]);
    let mut entity = post();
    assert!(hire_at(&mut village, &mut entity, POST, 0));
    entity.items[0] = Some(ItemStack::new("minecraft:stone", 1));
    assert_eq!(cycle(&mut village, &mut entity, 1), NO_FOOD);
    assert_eq!(village.age(WHEAT), 7);
    assert_eq!(order_for(9401, 1), None);
    assert!(!accepts(FARM_POST, 0, &ItemStack::new("minecraft:stone", 1)));
    assert!(accepts(FARM_POST, 0, &ItemStack::new("minecraft:baked_potato", 1)));
}

#[test]
fn full_post_stops_before_harvesting() {
    let mut village = Village::new("veio:test_work_full");
    village.villager(9501, [3.5, 64.0, 3.5], [5, 0, 0, 1]);
    let mut entity = post();
    set_number(&mut entity, "Ration", 10);
    for slot in 1..4 {
        entity.items[slot] = Some(ItemStack::new("minecraft:cobblestone", 64));
    }
    assert!(hire_at(&mut village, &mut entity, POST, 0));
    let now = cycles_at(&mut village, POST, &mut entity, 1, 40, |entity| number(entity, "Status") == 3);
    assert_eq!(village.age(WHEAT), 7, "a planta continua no campo");
    assert_eq!(number(&entity, "Ration"), 10, "sem trabalho, sem consumo");
    entity.items[3] = None;
    assert_eq!(cycle(&mut village, &mut entity, now + 1), 3, "trigo e semente precisam de dois espaços");
    entity.items[2] = None;
    cycles_at(&mut village, POST, &mut entity, now + 2, 5, |entity| number(entity, "Total") >= 1 || !carried(entity).is_empty());
    assert_eq!(village.age(WHEAT), 0);
}

#[test]
fn blocked_path_gives_up_on_that_crop() {
    let mut village = Village::new("veio:test_work_path");
    village.villager(9601, [-3.5, 64.0, -3.5], [6, 0, 0, 1]);
    let mut entity = post();
    set_number(&mut entity, "Ration", 10);
    assert!(hire_at(&mut village, &mut entity, POST, 0));
    let found = cycles_at(&mut village, POST, &mut entity, 1, 40, |entity| target(entity) == Some(WHEAT));
    let mut status = WALKING;
    let mut now = found;
    for _ in 0..(WALK_LIMIT as u64 / EVERY as u64) {
        now += 1;
        status = cycle(&mut village, &mut entity, now);
    }
    assert_eq!(status, PATH);
    assert_eq!(target(&entity), None);
    assert_eq!(village.age(WHEAT), 7, "sem chegar, nada é colhido");
    assert_eq!(number(&entity, "Ration"), 10);
    assert_eq!(entity.nbt.get(&"Skipped".into()), Some(&Tag::IntArray(WHEAT.to_vec())));
    for step in 1..=8 {
        assert_ne!(target(&entity), Some(WHEAT));
        assert_ne!(cycle(&mut village, &mut entity, now + step), WALKING, "a planta inalcançável fica de lado nesta volta");
        assert_eq!(entity.nbt.get(&"Skipped".into()), Some(&Tag::IntArray(WHEAT.to_vec())));
    }
}

#[test]
fn carried_crop_survives_dismissal_until_there_is_room() {
    let mut village = Village::new("veio:test_work_carry");
    village.villager(9701, [3.5, 64.0, 3.5], [7, 0, 0, 1]);
    let mut entity = post();
    set_number(&mut entity, "Ration", 10);
    assert!(hire_at(&mut village, &mut entity, POST, 0));
    let now = cycles_at(&mut village, POST, &mut entity, 1, 40, |entity| carried(entity).len() == 2);
    assert_eq!(carried(&entity).len(), 2);
    // O jogador enche o posto e dispensa o trabalhador antes da entrega.
    for slot in 1..4 {
        entity.items[slot] = Some(ItemStack::new("minecraft:cobblestone", 64));
    }
    assert!(dismiss(village.dimension, POST, &mut entity));
    assert_eq!(carried(&entity).len(), 2, "nada some");
    assert_eq!(cycle(&mut village, &mut entity, now + 1), NO_WORKER);
    entity.items[1] = None;
    entity.items[2] = None;
    cycle(&mut village, &mut entity, now + 2);
    assert!(carried(&entity).is_empty());
    assert_eq!(outputs_of(&entity)[1..], [("minecraft:wheat".into(), 1), ("minecraft:wheat_seeds".into(), 1)]);
}

#[test]
fn reopened_world_finds_the_worker_by_uuid() {
    let mut village = Village::new("veio:test_work_reload");
    village.villager(9801, [5.5, 64.0, 5.5], [8, 0, 0, 1]);
    let mut entity = post();
    set_number(&mut entity, "Ration", 10);
    assert!(hire_at(&mut village, &mut entity, POST, 0));
    // Salvar e reabrir: o NBT do posto fica; o aldeão volta com outro id de sessão.
    let saved = entity.nbt.clone();
    village.villagers.remove(&9801);
    village.villager(9802, [5.5, 64.0, 5.5], [8, 0, 0, 1]);
    let mut reopened = post();
    reopened.nbt = saved;
    let now = cycles_at(&mut village, POST, &mut reopened, 1, 40, |entity| target(entity) == Some(WHEAT));
    assert_eq!(number(&reopened, "Status"), WALKING);
    assert_eq!(order_for(9802, now), Some(goal(WHEAT)), "reconhecido pelo UUID");
    assert_eq!(number(&reopened, "Absent"), 0);
}

#[test]
fn missing_worker_is_released_after_a_while() {
    let mut village = Village::new("veio:test_work_absent");
    village.villager(9901, [3.5, 64.0, 3.5], [9, 0, 0, 1]);
    let mut entity = post();
    assert!(hire_at(&mut village, &mut entity, POST, 0));
    village.villagers.remove(&9901);
    assert_eq!(cycle(&mut village, &mut entity, 1), ABSENT);
    assert_eq!(worker(&entity), Some([9, 0, 0, 1]), "ausência curta mantém a atribuição");
    let mut status = ABSENT;
    for now in 2..=(ABSENT_LIMIT as u64 / EVERY as u64) {
        status = cycle(&mut village, &mut entity, now);
    }
    assert_eq!(status, NO_WORKER);
    assert_eq!(worker(&entity), None);
    assert!(claim([9, 0, 0, 1], village.dimension, [8, 64, 8], 700), "liberado para outro posto");
}

#[test]
fn stale_claim_from_unloaded_post_loses_to_the_live_one() {
    let dimension = "veio:test_work_stale";
    let uuid = [10, 0, 0, 1];
    let mut village = Village::new(dimension);
    village.villager(10001, [3.5, 64.0, 3.5], uuid);
    let mut old = post();
    assert!(hire_at(&mut village, &mut old, POST, 0));
    // O posto antigo fica descarregado; depois do prazo, outro posto contrata o mesmo aldeão.
    let mut new = post();
    assert!(!claim(uuid, dimension, [5, 64, 5], CLAIM_STALE));
    assert!(assign(dimension, [5, 64, 5], &mut new, (uuid, 10001), CLAIM_STALE + 1));
    // O antigo volta a carregar e percebe que perdeu o trabalhador.
    assert_eq!(cycle(&mut village, &mut old, CLAIM_STALE + 2), NO_WORKER);
    assert_eq!(worker(&old), None);
    assert_eq!(worker(&new), Some(uuid));
}

#[test]
fn work_goal_only_runs_with_a_fresh_order() {
    order(10101, [1.5, 64.0, 1.5], 0);
    assert_eq!(order_for(10101, ORDER_FRESH), Some([1.5, 64.0, 1.5]));
    assert_eq!(order_for(10101, ORDER_FRESH + 1), None, "ordem vencida: comportamento vanilla");
    cancel(10101);
    assert_eq!(order_for(10101, 0), None);
}

const PRESS_AT: [i32; 3] = [3, 64, 0];

#[test]
fn hauler_feeds_a_hungry_press_and_brings_the_plates_back() {
    let mut village = Village::new("veio:test_work_hauler");
    village.crops.clear();
    let mut press = block(PRESS);
    set_number(&mut press, "Status", 2);
    village.stations.insert(PRESS_AT, press);
    village.villager(11001, [5.5, 64.0, 5.5], [11, 0, 0, 1]);
    let mut hauler = block(HAULER_POST);
    set_number(&mut hauler, "Ration", 10);
    hauler.items[1] = Some(ItemStack::new("minecraft:raw_copper", 10));
    hauler.items[2] = Some(ItemStack::new("minecraft:dirt", 5));
    assert!(hire_at(&mut village, &mut hauler, POST, 0));

    // Varre a área; depois vai buscar o minério no posto.
    let now = cycles_at(&mut village, POST, &mut hauler, 1, HAULER_SWEEP, |post| target(post) == Some(PRESS_AT));
    assert_eq!(order_for(11001, now), Some(goal(POST)), "primeiro pega o insumo no posto");
    assert!(carried(&hauler).is_empty());
    village.walk(11001, [1, 64, 0]);
    cycle(&mut village, &mut hauler, now + 1);
    assert_eq!(hauler.items[1], None, "pegou o minério");
    assert_eq!(carried(&hauler), vec![("minecraft:raw_copper".to_owned(), 10)]);
    assert_eq!(carry_to(&hauler), Some(PRESS_AT));
    // Ao lado da prensa (2 blocos), entrega.
    cycle(&mut village, &mut hauler, now + 2);
    assert_eq!(village.stations[&PRESS_AT].items[0], Some(ItemStack::new("minecraft:raw_copper", 10)));
    assert!(carried(&hauler).is_empty());
    assert_eq!(number(&hauler, "Total"), 1);
    assert_eq!(number(&hauler, "Ration"), 8);
    assert_eq!(hauler.items[2], Some(ItemStack::new("minecraft:dirt", 5)), "terra não serve à prensa");

    // A prensa trabalhou: o carregador recolhe as chapas e leva ao posto.
    let press = village.stations.get_mut(&PRESS_AT).unwrap();
    press.items[1] = Some(ItemStack::new(PLATE, 6));
    set_number(press, "Status", 1);
    cycle(&mut village, &mut hauler, now + 3);
    assert_eq!(village.stations[&PRESS_AT].items[1], None);
    assert_eq!(carried(&hauler), vec![(PLATE.to_owned(), 6)]);
    cycle(&mut village, &mut hauler, now + 4);
    assert_eq!(hauler.items[4], Some(ItemStack::new(PLATE, 6)));
    assert_eq!(number(&hauler, "Total"), 2);
}

#[test]
fn hauler_leaves_belts_and_other_posts_alone() {
    let mut village = Village::new("veio:test_work_hauler_scope");
    village.crops.clear();
    let mut belt = block(BELT);
    belt.items[0] = Some(ItemStack::new(PLATE, 3));
    village.stations.insert([2, 64, 0], belt);
    let mut farm = block(FARM_POST);
    farm.items[1] = Some(ItemStack::new("minecraft:wheat", 3));
    village.stations.insert([0, 64, 3], farm);
    village.villager(11101, [1.5, 64.0, 1.5], [11, 1, 0, 1]);
    let mut hauler = block(HAULER_POST);
    set_number(&mut hauler, "Ration", 10);
    assert!(hire_at(&mut village, &mut hauler, POST, 0));
    let now = cycles_at(&mut village, POST, &mut hauler, 1, HAULER_SWEEP, |post| number(post, "Status") == IDLE_WORK);
    assert_eq!(order_for(11101, now), None, "sem tarefa, comportamento vanilla");
    assert_eq!(village.stations[&[2, 64, 0]].items[0], Some(ItemStack::new(PLATE, 3)));
    assert_eq!(village.stations[&[0, 64, 3]].items[1], Some(ItemStack::new("minecraft:wheat", 3)));
}

#[test]
fn two_haulers_never_serve_the_same_machine_at_once() {
    let mut village = Village::new("veio:test_work_reserve");
    village.crops.clear();
    let mut press = block(PRESS);
    press.items[1] = Some(ItemStack::new(PLATE, 6));
    set_number(&mut press, "Status", 1);
    village.stations.insert(PRESS_AT, press);
    let other = [0, 64, 4];
    village.villager(11201, [6.5, 64.0, -5.5], [12, 0, 0, 1]);
    village.villager(11202, [6.5, 64.0, 9.5], [12, 0, 0, 2]);
    let (mut first, mut second) = (block(HAULER_POST), block(HAULER_POST));
    set_number(&mut first, "Ration", 10);
    set_number(&mut second, "Ration", 10);
    assert!(assign(village.dimension, POST, &mut first, ([12, 0, 0, 1], 11201), 0));
    assert!(assign(village.dimension, other, &mut second, ([12, 0, 0, 2], 11202), 0));
    let now = cycles_at(&mut village, POST, &mut first, 1, HAULER_SWEEP, |post| target(post) == Some(PRESS_AT));
    cycles_at(&mut village, other, &mut second, now, HAULER_SWEEP, |post| number(post, "Status") == IDLE_WORK);
    assert_eq!(target(&second), None, "a prensa já está reservada para o primeiro");
    assert_eq!(order_for(11202, now + 1), None);
}

const DESK: [i32; 3] = [8, 64, 8];
const MANAGED: [i32; 3] = [12, 64, 8];

#[test]
fn foreman_feeds_a_hungry_post_and_rehires_when_it_is_empty() {
    let mut village = Village::new("veio:test_work_foreman");
    village.crops.clear();
    village.villager(11301, [9.5, 64.0, 9.5], [13, 0, 0, 1]);
    village.villager(11302, [13.5, 64.0, 9.5], [13, 0, 0, 2]);
    village.villager(11303, [14.5, 64.0, 7.5], [13, 0, 0, 3]);
    let mut farm = block(FARM_POST);
    assert!(assign(village.dimension, MANAGED, &mut farm, ([13, 0, 0, 2], 11302), 0));
    village.stations.insert(MANAGED, farm);
    let mut desk = block(FOREMAN_DESK);
    desk.items[0] = Some(ItemStack::new("minecraft:bread", 10));
    desk.items[3] = Some(ItemStack::new(WAGE, 1));
    assert!(assign(village.dimension, DESK, &mut desk, ([13, 0, 0, 1], 11301), 0));

    // Comida: o capataz come um pão, pega oito e leva ao posto.
    let now = cycles_at(&mut village, DESK, &mut desk, 1, FOREMAN_SWEEP, |desk| !carried(desk).is_empty());
    assert_eq!(number(&desk, "Ration"), 6, "comeu um pão; a comida só é gasta ao concluir a tarefa");
    assert_eq!(carried(&desk), vec![("minecraft:bread".to_owned(), 8)]);
    assert_eq!(desk.items[0], Some(ItemStack::new("minecraft:bread", 1)));
    village.walk(11301, [11, 64, 8]);
    cycles_at(&mut village, DESK, &mut desk, now + 1, 3, |desk| carried(desk).is_empty());
    assert_eq!(village.stations[&MANAGED].items[0], Some(ItemStack::new("minecraft:bread", 8)));

    // O lavrador some: o capataz leva a esmeralda e contrata outro aldeão perto do posto.
    let managed = village.stations.get_mut(&MANAGED).unwrap();
    assert!(dismiss(village.dimension, MANAGED, managed));
    village.walk(11301, [9, 64, 8]);
    let now = cycles_at(&mut village, DESK, &mut desk, now + 5, 10, |desk| !carried(desk).is_empty());
    assert_eq!(carry_kind(&desk), Carry::Wage);
    assert_eq!(desk.items[3], None);
    village.walk(11301, [11, 64, 8]);
    cycles_at(&mut village, DESK, &mut desk, now + 1, 3, |desk| carried(desk).is_empty());
    let hired = worker(&village.stations[&MANAGED]);
    assert!(hired == Some([13, 0, 0, 2]) || hired == Some([13, 0, 0, 3]), "contratou um aldeão livre: {hired:?}");
    assert_ne!(hired, Some([13, 0, 0, 1]), "o capataz não contrata a si mesmo");
    assert_eq!(number(&desk, "Hired"), 1);
}

#[test]
fn foreman_without_emeralds_says_so_and_keeps_the_emerald_if_nobody_is_free() {
    let mut village = Village::new("veio:test_work_foreman_wage");
    village.crops.clear();
    village.villager(11401, [9.5, 64.0, 9.5], [14, 0, 0, 1]);
    village.stations.insert(MANAGED, block(FARM_POST));
    let mut desk = block(FOREMAN_DESK);
    desk.items[0] = Some(ItemStack::new("minecraft:bread", 4));
    assert!(assign(village.dimension, DESK, &mut desk, ([14, 0, 0, 1], 11401), 0));
    let now = cycles_at(&mut village, DESK, &mut desk, 1, FOREMAN_SWEEP, |desk| number(desk, "Status") == NO_WAGE);
    // Com esmeralda, mas sem aldeão livre perto do posto: a esmeralda volta à mesa.
    desk.items[3] = Some(ItemStack::new(WAGE, 1));
    village.walk(11401, [9, 64, 8]);
    let now = cycles_at(&mut village, DESK, &mut desk, now + 1, 5, |desk| !carried(desk).is_empty());
    village.walk(11401, [11, 64, 8]);
    cycle_at(&mut village, DESK, &mut desk, now + 1);
    assert_eq!(carried(&desk), vec![(WAGE.to_owned(), 1)], "ninguém contratado: continua com a esmeralda");
    assert_eq!(carry_to(&desk), None, "volta para a mesa");
    village.walk(11401, [9, 64, 8]);
    cycle_at(&mut village, DESK, &mut desk, now + 2);
    assert_eq!(desk.items[3], Some(ItemStack::new(WAGE, 1)));
    assert_eq!(worker(&village.stations[&MANAGED]), None);
}

fn cycle_at(village: &mut Village, pos: [i32; 3], entity: &mut ModBlockEntity, now: u64) {
    let phase = i64::from((pos[0] ^ pos[2]) & 7);
    step(village, pos, entity, (EVERY - phase).rem_euclid(EVERY) + now as i64 * EVERY, now);
}

#[test]
fn supply_returns_to_the_stock_when_the_machine_no_longer_takes_it() {
    let mut village = Village::new("veio:test_work_return");
    village.crops.clear();
    let mut press = block(PRESS);
    set_number(&mut press, "Status", 2);
    village.stations.insert(PRESS_AT, press);
    village.villager(11501, [1.5, 64.0, 0.5], [15, 0, 0, 1]);
    let mut hauler = block(HAULER_POST);
    set_number(&mut hauler, "Ration", 10);
    hauler.items[1] = Some(ItemStack::new("minecraft:raw_copper", 10));
    assert!(hire_at(&mut village, &mut hauler, POST, 0));
    let now = cycles_at(&mut village, POST, &mut hauler, 1, HAULER_SWEEP, |post| !carried(post).is_empty());
    // Enquanto ele anda, o jogador põe ferro bruto na prensa.
    village.stations.get_mut(&PRESS_AT).unwrap().items[0] = Some(ItemStack::new("minecraft:raw_iron", 64));
    cycle(&mut village, &mut hauler, now + 1);
    assert_eq!(carried(&hauler), vec![("minecraft:raw_copper".to_owned(), 10)], "nada sumiu");
    assert_eq!(carry_to(&hauler), None, "volta ao posto");
    assert_eq!(number(&hauler, "Total"), 0, "viagem sem entrega não conta");
    cycle(&mut village, &mut hauler, now + 2);
    assert_eq!(hauler.items[1], Some(ItemStack::new("minecraft:raw_copper", 10)), "de volta ao estoque");
    assert!(hauler.items[4..].iter().all(Option::is_none), "insumo não vira produto");
}

// ---------- Fila de tarefas do capataz ----------

const HAULER_AT: [i32; 3] = [10, 64, 10];
const MILL_AT: [i32; 3] = [8, 64, 12];

/// Mesa com capataz contratado e comida, num mundo sem plantas.
fn desk_with_foreman(village: &mut Village, villager: u32, uuid: Uuid) -> ModBlockEntity {
    village.crops.clear();
    village.villager(villager, [9.5, 64.0, 8.5], uuid);
    let mut desk = block(FOREMAN_DESK);
    set_number(&mut desk, "Ration", 50);
    assert!(assign(village.dimension, DESK, &mut desk, (uuid, villager), 0));
    desk
}

#[test]
fn queue_moves_harvested_wheat_from_the_farm_post_to_the_mill() {
    let mut village = Village::new("veio:test_work_queue");
    let mut desk = desk_with_foreman(&mut village, 12001, [20, 0, 0, 1]);
    desk.items[0] = Some(ItemStack::new("minecraft:bread", foreman::DESK_FOOD));
    let mut farm = block(FARM_POST);
    farm.items[1] = Some(ItemStack::new("minecraft:wheat", 10));
    village.stations.insert(MANAGED, farm);
    village.stations.insert(MILL_AT, block(production::MILL));
    let now = cycles_at(&mut village, DESK, &mut desk, 1, 100, |desk| !foreman::queue(desk).is_empty());
    let task = foreman::queue(&desk).remove(0);
    assert_eq!((task.from, task.slot, task.item.as_str(), task.count, task.to, task.into),
        (MANAGED, 1, "minecraft:wheat", 10, MILL_AT, 0));
    village.stations.insert(DESK, desk);

    // Um carregador perto pega a tarefa antes das próprias.
    village.villager(12002, [10.5, 64.0, 11.5], [20, 0, 0, 2]);
    let mut hauler = block(HAULER_POST);
    set_number(&mut hauler, "Ration", 10);
    assert!(assign(village.dimension, HAULER_AT, &mut hauler, ([20, 0, 0, 2], 12002), now));
    cycle_at(&mut village, HAULER_AT, &mut hauler, now + 1);
    assert_eq!(order_for(12002, now + 1), Some(goal(MANAGED)), "vai buscar no lavrador");
    assert_eq!(foreman::queue(&village.stations[&DESK])[0].by.map(|(by, _)| by), Some(HAULER_AT));
    village.walk(12002, [11, 64, 8]);
    cycle_at(&mut village, HAULER_AT, &mut hauler, now + 2);
    assert_eq!(carried(&hauler), vec![("minecraft:wheat".to_owned(), 10)]);
    assert_eq!(village.stations[&MANAGED].items[1], None);
    assert!(foreman::queue(&village.stations[&DESK]).is_empty(), "tarefa sai da fila ao pegar os itens");
    village.walk(12002, [8, 64, 11]);
    cycle_at(&mut village, HAULER_AT, &mut hauler, now + 3);
    assert_eq!(village.stations[&MILL_AT].items[0], Some(ItemStack::new("minecraft:wheat", 10)));
    assert!(carried(&hauler).is_empty());
}

#[test]
fn queue_brings_pantry_emeralds_to_the_desk_and_frees_abandoned_tasks() {
    let mut village = Village::new("veio:test_work_queue_desk");
    let mut desk = desk_with_foreman(&mut village, 12101, [21, 0, 0, 1]);
    let mut pantry = block(production::PANTRY);
    pantry.items[1] = Some(ItemStack::new(WAGE, 3));
    village.stations.insert(MILL_AT, pantry);
    let now = cycles_at(&mut village, DESK, &mut desk, 1, 100, |desk| !foreman::queue(desk).is_empty());
    let task = foreman::queue(&desk).remove(0);
    assert_eq!((task.item.as_str(), task.count, task.to, task.into), (WAGE, 3, DESK, 3), "esmeraldas vão para o slot da mesa");
    village.stations.insert(DESK, desk);
    let first = foreman::claim_task(&mut village, HAULER_AT, now);
    assert!(matches!(first, Some(Plan::Fetch { .. })));
    assert_eq!(foreman::claim_task(&mut village, [10, 64, 6], now), None, "já pega por outro carregador");
    // A mesa continua ativa e mantém a tarefa marcada enquanto a marca vale.
    let mut desk = village.stations.remove(&DESK).unwrap();
    for later in now + 150..now + 154 {
        cycle_at(&mut village, DESK, &mut desk, later);
    }
    assert!(foreman::queue(&desk).iter().any(|task| task.by.is_some()), "tarefa pega continua na fila");
    village.stations.insert(DESK, desk);
    // O primeiro carregador sumiu: a marca vence e a tarefa fica livre.
    assert!(matches!(foreman::claim_task(&mut village, [10, 64, 6], now + 201), Some(Plan::Fetch { .. })));
}

// ---------- Pastor ----------

fn animal(village: &mut Village, id: u32, kind: &str, at: [f64; 3], nbt: &[(&str, Tag)]) {
    let data = nbt.iter().map(|(key, value)| ((*key).into(), value.clone())).collect();
    village.animals.insert(id, (at, data));
    production::ranch::observe(village.dimension, id, kind);
}

fn shepherd_with_worker(village: &mut Village, villager: u32, uuid: Uuid, at: [f64; 3]) -> ModBlockEntity {
    village.crops.clear();
    village.villager(villager, at, uuid);
    let mut post = block(SHEPHERD_POST);
    set_number(&mut post, "Ration", 10);
    assert!(hire_at(village, &mut post, POST, 0));
    post
}

#[test]
fn shepherd_picks_up_eggs_and_shears_sheep_beside_them() {
    let mut village = Village::new("veio:test_work_shepherd");
    let mut post = shepherd_with_worker(&mut village, 13001, [30, 0, 0, 1], [5.5, 64.0, 5.5]);
    village.drops.push(NativeItemEntity { id: 13050, position: [3.5, 64.0, 3.5], stack: ItemStack::new("minecraft:egg", 2), pickup_delay: 0 });
    assert_eq!(cycle(&mut village, &mut post, 1), WALKING);
    assert_eq!(village.drops.len(), 1, "longe do ovo, nada é recolhido");
    village.walk(13001, [3, 64, 3]);
    cycle(&mut village, &mut post, 2);
    assert!(village.drops.is_empty());
    assert_eq!(carried(&post), vec![("minecraft:egg".to_owned(), 2)]);
    village.walk(13001, [1, 64, 0]);
    cycle(&mut village, &mut post, 3);
    assert_eq!(post.items[2], Some(ItemStack::new("minecraft:egg", 2)));

    animal(&mut village, 13003, "minecraft:sheep", [-2.5, 64.0, 1.5], &[("Age", Tag::Int(0)), ("Color", Tag::Byte(14))]);
    assert_eq!(cycle(&mut village, &mut post, 4), WALKING);
    village.walk(13001, [-2, 64, 1]);
    cycle(&mut village, &mut post, 5);
    assert_eq!(village.animals[&13003].1.get(&"Sheared".into()), Some(&Tag::Byte(1)));
    assert_eq!(carried(&post), vec![("minecraft:red_wool".to_owned(), 1 + 13003 % 3)]);
    assert_eq!(number(&post, "Total"), 2);
}

#[test]
fn shepherd_feeds_a_breeding_pair_and_then_stops() {
    let mut village = Village::new("veio:test_work_shepherd_feed");
    let mut post = shepherd_with_worker(&mut village, 13101, [31, 0, 0, 1], [2.5, 64.0, 2.5]);
    post.items[1] = Some(ItemStack::new("minecraft:wheat", 5));
    animal(&mut village, 13102, "minecraft:cow", [2.5, 64.0, 2.5], &[("Age", Tag::Int(0))]);
    animal(&mut village, 13103, "minecraft:cow", [3.5, 64.0, 2.5], &[("Age", Tag::Int(0))]);
    cycle(&mut village, &mut post, 1);
    cycle(&mut village, &mut post, 2);
    let in_love = |village: &Village, id| village.animals[&id].1.get(&"InLove".into()) == Some(&Tag::Int(600));
    assert!(in_love(&village, 13102) && in_love(&village, 13103), "os dois comeram");
    assert_eq!(post.items[1], Some(ItemStack::new("minecraft:wheat", 3)));
    assert_eq!(cycle(&mut village, &mut post, 3), IDLE_WORK, "par pronto: não gasta mais ração");
    assert_eq!(post.items[1], Some(ItemStack::new("minecraft:wheat", 3)));
}

#[test]
fn shepherd_does_not_start_feeding_a_lonely_animal() {
    let mut village = Village::new("veio:test_work_shepherd_alone");
    let mut post = shepherd_with_worker(&mut village, 13201, [32, 0, 0, 1], [2.5, 64.0, 2.5]);
    post.items[1] = Some(ItemStack::new("minecraft:wheat", 5));
    animal(&mut village, 13202, "minecraft:cow", [2.5, 64.0, 2.5], &[("Age", Tag::Int(0))]);
    assert_eq!(cycle(&mut village, &mut post, 1), IDLE_WORK);
    assert_eq!(post.items[1], Some(ItemStack::new("minecraft:wheat", 5)));
}

#[test]
fn idle_farmhand_scans_one_row_per_cycle() {
    let mut village = Village::new("veio:test_work_scan");
    village.crops.clear();
    village.villager(11601, [1.5, 64.0, 1.5], [16, 0, 0, 1]);
    let mut entity = post();
    set_number(&mut entity, "Ration", 10);
    assert!(hire_at(&mut village, &mut entity, POST, 0));
    village.looks.set(0);
    cycle(&mut village, &mut entity, 1);
    assert_eq!(village.looks.get(), farmhand::SCANS as u32, "uma fileira, não a camada inteira");
}

#[test]
fn raw_wheat_does_not_pay_a_full_harvest() {
    let mut village = Village::new("veio:test_work_wheat");
    village.villager(11701, [3.5, 64.0, 3.5], [17, 0, 0, 1]);
    let mut entity = post();
    entity.items[0] = Some(ItemStack::new("minecraft:wheat", 1));
    assert!(hire_at(&mut village, &mut entity, POST, 0));
    assert_eq!(cycle(&mut village, &mut entity, 1), NO_FOOD);
    assert_eq!(number(&entity, "Ration"), 1, "comeu o trigo e ainda não cobre a colheita");
    assert_eq!(number(&entity, "Info"), 0);
    assert_eq!(village.age(WHEAT), 7, "sem pontos suficientes, a planta fica");
    entity.items[0] = Some(ItemStack::new("minecraft:wheat", 1));
    cycles_at(&mut village, POST, &mut entity, 2, 40, |entity| !carried(entity).is_empty());
    assert_eq!(village.age(WHEAT), 0);
    assert_eq!(number(&entity, "Ration"), 0);
}

#[test]
fn idle_hauler_scans_one_row_per_cycle() {
    let mut village = Village::new("veio:test_work_hauler_scan");
    village.crops.clear();
    village.villager(11801, [1.5, 64.0, 1.5], [18, 0, 0, 1]);
    let mut hauler = block(HAULER_POST);
    set_number(&mut hauler, "Ration", 10);
    assert!(hire_at(&mut village, &mut hauler, POST, 0));
    village.looks.set(0);
    cycle(&mut village, &mut hauler, 1);
    assert_eq!(village.looks.get(), hauler::SCAN as u32);
}

#[test]
fn idle_foreman_scans_one_row_per_cycle() {
    let mut village = Village::new("veio:test_work_foreman_scan");
    village.crops.clear();
    village.villager(11901, [9.5, 64.0, 9.5], [19, 0, 0, 1]);
    let mut desk = block(FOREMAN_DESK);
    set_number(&mut desk, "Ration", 10);
    assert!(assign(village.dimension, DESK, &mut desk, ([19, 0, 0, 1], 11901), 0));
    village.looks.set(0);
    cycle_at(&mut village, DESK, &mut desk, 1);
    assert_eq!(village.looks.get(), foreman::SCAN as u32);
}

#[test]
fn queue_reads_each_network_block_once_per_refresh() {
    let mut village = Village::new("veio:test_work_queue_once");
    village.crops.clear();
    village.villager(13501, [9.5, 64.0, 8.5], [35, 0, 0, 1]);
    for i in 0..8 {
        village.stations.insert([8 + i, 64, 9], block(PRESS));
    }
    let mut desk = block(FOREMAN_DESK);
    set_number(&mut desk, "Ration", 10);
    assert!(assign(village.dimension, DESK, &mut desk, ([35, 0, 0, 1], 13501), 0));
    let found = cycles_at(&mut village, DESK, &mut desk, 1, 100, |desk| get_list(desk, "Posts").is_some());
    village.edits.set(0);
    cycles_at(&mut village, DESK, &mut desk, found + 1, 5, |desk| number(desk, "Organize") == 0);
    assert_eq!(village.edits.get(), 8, "uma leitura por máquina, não fontes e destinos em passadas separadas");
}

#[test]
fn idle_foreman_inspects_a_few_posts_per_cycle() {
    let mut village = Village::new("veio:test_work_foreman_looks");
    village.crops.clear();
    village.villager(13601, [9.5, 64.0, 8.5], [36, 0, 0, 1]);
    for i in 0..6 {
        village.stations.insert([9 + i, 64, 8], block(FARM_POST));
    }
    let mut desk = block(FOREMAN_DESK);
    set_number(&mut desk, "Ration", 10);
    assert!(assign(village.dimension, DESK, &mut desk, ([36, 0, 0, 1], 13601), 0));
    let found = cycles_at(&mut village, DESK, &mut desk, 1, 100, |desk| {
        get_list(desk, "Posts").is_some_and(|posts| posts.len() >= 6)
    });
    village.edits.set(0);
    village.looks.set(0);
    cycle_at(&mut village, DESK, &mut desk, found + 1);
    assert_eq!(village.looks.get(), foreman::SCAN as u32, "não relê a Área para achar os postos");
    assert_eq!(village.edits.get(), foreman::LOOKS as u32, "não inspeciona os 6 postos no mesmo ciclo");
}
