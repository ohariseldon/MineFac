//! Testes dos trabalhadores: atribuição exclusiva, ciclo físico de colheita, conservação de
//! itens, interrupções e restauração depois de reabrir o mundo.
//!
//! Cada teste usa sua própria dimensão, UUIDs e ids, porque os registros são globais.
use super::*;
use horizon_client::mod_api::{BlockStateId, EntityRef};
use std::cell::Cell;

const POST: [i32; 3] = [0, 64, 0];
/// Fora do alcance de entrega do posto: colher exige ir até lá.
const WHEAT: [i32; 3] = [3, 64, 3];

#[derive(Default)]
struct Village {
    dimension: &'static str,
    crops: HashMap<[i32; 3], (String, u8)>,
    villagers: HashMap<u32, ([f64; 3], Uuid, i32)>,
    looks: Cell<u32>,
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
        self.crops.get(&pos).map_or_else(|| "minecraft:air".to_owned(), |(id, _)| id.clone())
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
        self.villagers.contains_key(&id).then(|| EntityRef::new(id))
    }
    fn entity_position(&self, entity: EntityRef) -> Option<[f64; 3]> {
        self.villagers.get(&entity.id()).map(|(at, _, _)| *at)
    }
    fn entity_nbt(&mut self, entity: EntityRef) -> Option<Compound> {
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
    let mut entity = ModBlockEntity::default();
    entity.id = FARM_POST.to_owned();
    entity.items = vec![None; 4];
    entity
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

/// A varredura incremental não encontra a planta no primeiro ciclo; avança até o predicado.
fn step_until(
    village: &mut Village,
    entity: &mut ModBlockEntity,
    mut now: u64,
    pred: impl Fn(u64, i32, &ModBlockEntity) -> bool,
) -> (u64, i32) {
    let mut status = number(entity, "Status");
    let limit = farm::AREA / farmhand::SCANS + 8;
    for _ in 0..limit {
        status = cycle(village, entity, now);
        if pred(now, status, entity) {
            return (now, status);
        }
        now += 1;
    }
    panic!("posto não alcançou o estado esperado (último status {status}, agora {now})");
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

    let (now, status) = step_until(&mut village, &mut entity, 1, |_, _, entity| target(entity) == Some(WHEAT));
    assert_eq!(status, WALKING);
    assert_eq!(entity.items[0], None, "comeu o pão");
    assert_eq!(number(&entity, "Ration"), 6);
    assert_eq!(number(&entity, "Info"), 3, "pão vale 3 colheitas");
    assert_eq!(order_for(9301, now), Some(goal(WHEAT)), "o aldeão recebeu a ordem de ir até o trigo");
    assert_eq!(village.age(WHEAT), 7, "longe da planta, nada é colhido");

    village.walk(9301, WHEAT);
    let (now, status) = step_until(&mut village, &mut entity, now + 1, |_, _, entity| !carried(entity).is_empty());
    assert_eq!(status, 1);
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

    let (idle, status) = step_until(&mut village, &mut entity, now + 3, |_, status, _| status == NO_TASK);
    assert_eq!(status, NO_TASK, "uma volta sem nada maduro");
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
    let (now, status) = step_until(&mut village, &mut entity, 1, |_, status, _| status == 3);
    assert_eq!(status, 3, "destino cheio");
    assert_eq!(village.age(WHEAT), 7, "a planta continua no campo");
    assert_eq!(number(&entity, "Ration"), 10, "sem trabalho, sem consumo");
    entity.items[3] = None;
    assert_eq!(cycle(&mut village, &mut entity, now + 1), 3, "trigo e semente precisam de dois espaços");
    entity.items[2] = None;
    let (_, status) = step_until(&mut village, &mut entity, now + 2, |_, _, entity| {
        number(entity, "Total") >= 1 || !carried(entity).is_empty()
    });
    assert_eq!(status, 1);
    assert_eq!(village.age(WHEAT), 0);
}

#[test]
fn blocked_path_gives_up_on_that_crop() {
    let mut village = Village::new("veio:test_work_path");
    village.villager(9601, [-3.5, 64.0, -3.5], [6, 0, 0, 1]);
    let mut entity = post();
    set_number(&mut entity, "Ration", 10);
    assert!(hire_at(&mut village, &mut entity, POST, 0));
    let (found, status) = step_until(&mut village, &mut entity, 1, |_, _, entity| target(entity) == Some(WHEAT));
    assert_eq!(status, WALKING);
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
    assert_eq!(
        entity.nbt.get(&"Skipped".into()),
        Some(&Tag::IntArray(WHEAT.to_vec())),
        "a planta inalcançável entra na lista desta volta",
    );
    for step in 1..=8 {
        assert_ne!(
            target(&entity),
            Some(WHEAT),
            "a planta inalcançável fica de lado nesta volta",
        );
        assert_ne!(cycle(&mut village, &mut entity, now + step), WALKING);
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
    let (now, _) = step_until(&mut village, &mut entity, 1, |_, _, entity| carried(entity).len() == 2);
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
    let (now, status) = step_until(&mut village, &mut reopened, 1, |_, _, entity| target(entity) == Some(WHEAT));
    assert_eq!(status, WALKING);
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

#[test]
fn idle_farmhand_scans_one_row_per_cycle() {
    let mut village = Village::new("veio:test_work_scan");
    village.crops.clear();
    village.villager(10201, [1.5, 64.0, 1.5], [11, 0, 0, 1]);
    let mut entity = post();
    set_number(&mut entity, "Ration", 10);
    assert!(hire_at(&mut village, &mut entity, POST, 0));
    village.looks.set(0);
    cycle(&mut village, &mut entity, 1);
    assert_eq!(village.looks.get(), farmhand::SCANS as u32, "uma fileira, não a camada inteira");
    village.looks.set(0);
    cycle(&mut village, &mut entity, 2);
    assert_eq!(village.looks.get(), farmhand::SCANS as u32);
}

#[test]
fn raw_wheat_does_not_pay_a_full_harvest() {
    let mut village = Village::new("veio:test_work_wheat");
    village.villager(10301, [3.5, 64.0, 3.5], [12, 0, 0, 1]);
    let mut entity = post();
    entity.items[0] = Some(ItemStack::new("minecraft:wheat", 1));
    assert!(hire_at(&mut village, &mut entity, POST, 0));
    assert_eq!(cycle(&mut village, &mut entity, 1), NO_FOOD);
    assert_eq!(number(&entity, "Ration"), 1, "comeu o trigo e ainda não cobre a colheita");
    assert_eq!(number(&entity, "Info"), 0);
    assert_eq!(village.age(WHEAT), 7, "sem pontos suficientes, a planta fica");
    entity.items[0] = Some(ItemStack::new("minecraft:wheat", 1));
    let (_, status) = step_until(&mut village, &mut entity, 2, |_, _, entity| !carried(entity).is_empty());
    assert_eq!(status, 1);
    assert_eq!(village.age(WHEAT), 0);
    assert_eq!(number(&entity, "Ration"), 0);
}

#[test]
fn two_carrots_cover_one_harvest() {
    let mut village = Village::new("veio:test_work_carrot");
    village.crops.insert(WHEAT, ("minecraft:carrots".into(), 7));
    village.villager(10401, [3.5, 64.0, 3.5], [13, 0, 0, 1]);
    let mut entity = post();
    entity.items[0] = Some(ItemStack::new("minecraft:carrot", 2));
    assert!(hire_at(&mut village, &mut entity, POST, 0));
    let (_, status) = step_until(&mut village, &mut entity, 1, |_, _, entity| !carried(entity).is_empty());
    assert_eq!(status, 1);
    assert_eq!(entity.items[0], None, "as duas cenouras pagaram a colheita");
    assert_eq!(number(&entity, "Ration"), 0);
    assert_eq!(village.age(WHEAT), 0);
}
