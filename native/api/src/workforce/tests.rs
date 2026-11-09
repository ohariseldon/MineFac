//! Testes dos trabalhadores: atribuição exclusiva, ciclo físico de colheita, conservação de
//! itens, interrupções e restauração depois de reabrir o mundo.
//!
//! Cada teste usa sua própria dimensão, UUIDs e ids, porque os registros são globais.
use super::*;
use horizon_client::mod_api::{BlockStateId, EntityRef, NativeItemEntity};
use std::cell::Cell;

#[test]
fn active_barracks_keep_the_night_watch_while_civilians_sleep() {
    use horizon_client::mobs::{Ai, Mob, MobType, Vec3, villager, villager_brain::{Act, Brain}};
    let mut village = Village::new("veio:night-watch-integration");
    village.villager(98521, [0.5, 64.0, 0.5], [98, 5, 2, 1]);
    let mut post = block(BARRACKS);
    set_number(&mut post, "Ration", 10);
    assert!(hire_at(&mut village, &mut post, POST, 0));
    village.day = 13000;
    assert_ne!(cycle(&mut village, &mut post, 1), RESTING);
    let mut guard = Mob { id: 98521, rider: None, extra: Default::default(),
        kind: std::sync::Arc::new(MobType { id: VILLAGER.into() }), position: Vec3::default(),
        velocity: Vec3::default(), climbing: false, ai: Ai::default() };
    villager::ext_mut(&mut guard).brain = Some(Box::new(Brain::preview(Act::Rest)));
    let duty = on_duty(guard.id, 1);
    assert!(matches!(steer(&mut guard, [4.0, 64.0, 0.0], duty), Steer::Brain),
        "The active defender must retain its patrol target through the vanilla rest schedule");
    assert!(villager::ext(&guard).unwrap().brain.as_ref().unwrap().mem.walk_target.get().is_some());
}

#[test]
fn worker_discovery_keeps_cached_targets_and_cursor_while_globally_deferred() {
    struct Probe { reads: Cell<usize> }
    impl Host for Probe {
        fn dimension(&self) -> &'static str { "veio:worker-budget-test" }
        fn block(&self, _: [i32; 3]) -> String {
            self.reads.set(self.reads.get() + 1);
            "minecraft:air".into()
        }
    }
    let host = Probe { reads: Cell::new(0) };
    let mut entity = ModBlockEntity::default();
    set_list(&mut entity, "Area", &[[3, 64, 3]]);
    set_number(&mut entity, "AreaScan", 7);
    budget::begin_tick(0);
    assert!(budget::request(budget::Lane::Workforce, host.dimension(), [999; 3], 512));
    let cached = scan_named(&host, POST, &mut entity, "Area", 2, 1, 8, &|_| false, true);
    assert_eq!(cached, Some(vec![[3, 64, 3]]));
    assert_eq!(number(&entity, "AreaScan"), 7);
    assert_eq!(host.reads.get(), 0);
    budget::begin_tick(1);
    scan_named(&host, POST, &mut entity, "Area", 2, 1, 8, &|_| false, true);
    assert_eq!(number(&entity, "AreaScan"), 8);
    assert_eq!(host.reads.get(), 1);
}

const POST: [i32; 3] = [0, 64, 0];
/// Fora do alcance de entrega do posto: colher exige ir até lá.
const WHEAT: [i32; 3] = [3, 64, 3];
/// Primeira volta do carregador (507 / 16) com folga.
const HAULER_SWEEP: u64 = 48;
/// Primeira volta do capataz (1875 / 25) com folga.
const FOREMAN_SWEEP: u64 = 90;

#[test]
fn background_hauler_search_waits_during_rest_hunger_and_delivery() {
    let mut world = Village::new("veio:search-background-guards");
    let mut post = block(HAULER_POST);
    set_worker(&mut post, Some([91,0,0,1]));
    set_number(&mut post, "HaulSearchPending", 1);
    for (time,status) in [(1,RESTING),(2,NO_FOOD)] {
        set_number(&mut post,"Status",status);
        tick(&mut world, POST, &mut post,time);
        assert_eq!(number(&post,"HaulSearchPending"),1);
    }
    set_number(&mut post,"Status",WALKING);
    set_carried(&mut post,&[(PLATE.into(),3)],Some(WHEAT),Carry::Supply,0);
    tick(&mut world,POST,&mut post,3);
    assert_eq!(number(&post,"HaulSearchPending"),1);
    assert_eq!(carried(&post),[(PLATE.into(),3)]);
    set_carried(&mut post,&[],None,Carry::Supply,0);
    tick(&mut world,POST,&mut post,4);
    assert_eq!(number(&post,"HaulSearchPending"),0);
}

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
    /// Esmeraldas no inventário do jogador.
    emeralds: u32,
    summoned: Vec<[f64; 3]>,
    parts: HashMap<[i32; 3], String>,
    notices: Vec<String>,
    commands: Vec<String>,
    /// Hora do dia (0 = manhã).
    day: i64,
    /// Item na mão de cada aldeão e NBT mesclado neles.
    hands: HashMap<u32, Option<String>>,
    villager_nbt: HashMap<u32, Compound>,
    /// Vida dos aldeões que têm uma (`set_mob_health`).
    lives: HashMap<u32, f32>,
    players: Vec<horizon_client::mod_api::NativeServerPlayer>,
    edits: Cell<u32>,
    reads: Cell<u32>,
    /// Mensagens no chat e na barra de ação.
    told: Vec<String>,
    /// Outros itens no inventário do jogador.
    bag: HashMap<String, u32>,
    sneak: bool,
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
        self.parts.get(&pos).cloned().or_else(|| {
            (key == "age").then(|| self.crops.get(&pos).map(|(_, age)| age.to_string())).flatten()
        })
    }
    fn set_block_property(&mut self, pos: [i32; 3], key: &str, value: &str) -> bool {
        if key != "age" { return self.crops.contains_key(&pos); }
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
        if self.villagers.contains_key(&entity.id()) {
            self.villager_nbt.entry(entity.id()).or_default().extend(nbt);
            return true;
        }
        let Some((_, data)) = self.animals.get_mut(&entity.id()) else { return false };
        data.extend(nbt);
        true
    }
    fn living(&self, entity: EntityRef) -> Option<horizon_client::mod_api::LivingRef> {
        self.villagers.contains_key(&entity.id()).then(|| horizon_client::mod_api::LivingRef::new(entity))
    }
    fn living_set_equipment(&mut self, living: horizon_client::mod_api::LivingRef,
        slot: horizon_client::mod_api::EquipmentSlot, stack: Option<ItemStack>) -> bool
    {
        assert_eq!(slot, horizon_client::mod_api::EquipmentSlot::MainHand);
        self.hands.insert(living.id(), stack.map(|stack| stack.item.to_string()));
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
        self.reads.set(self.reads.get() + 1);
        if let Some((_, data)) = self.animals.get(&entity.id()) {
            return Some(data.clone());
        }
        let (_, uuid, age) = self.villagers.get(&entity.id())?;
        let mut nbt = Compound::from([("UUID".into(), Tag::IntArray(uuid.to_vec())), ("Age".into(), Tag::Int(*age))]);
        if let Some(life) = self.lives.get(&entity.id()) {
            nbt.insert("Health".into(), Tag::Float(life.to_bits()));
        }
        Some(nbt)
    }
    fn set_mob_health(&mut self, _: &str, id: u32, health: f32) -> bool {
        if health <= 0.0 {
            return false;
        }
        if let Some((_, data)) = self.animals.get_mut(&id) {
            data.insert("Health".into(), Tag::Float(health.to_bits()));
            return true;
        }
        if self.villagers.contains_key(&id) {
            self.lives.insert(id, health);
            return true;
        }
        false
    }
    fn send_particles(&mut self, _: &ParticleOptions, _: Option<BlockStateId>, _: [f64; 3], _: [f64; 3], _: f64, _: i32) -> bool {
        true
    }
    fn play_sound(&mut self, _: &str, _: [f64; 3], _: usize, _: f32, _: f32) -> bool {
        true
    }
    fn kill_mob(&mut self, _: &str, id: u32) -> bool {
        self.animals.remove(&id).is_some()
    }
    fn remove_mob(&mut self, _: &str, id: u32) -> bool {
        self.animals.remove(&id).is_some() || self.villagers.remove(&id).is_some()
    }
    fn edit_blocks(&mut self, _: &str, edit: &horizon_client::mod_api::NativeBlockEdit) -> Result<u64, String> {
        for run in &edit.runs {
            for dy in 0..run.height as i32 {
                self.crops.insert([run.x, run.y + dy, run.z], (edit.palette[run.block as usize].clone(), 0));
            }
        }
        Ok(edit.runs.len() as u64)
    }
    fn take_items(&mut self, item: &str, count: u32) -> u32 {
        if item != WAGE {
            let have = self.bag.entry(item.to_owned()).or_default();
            let taken = count.min(*have);
            *have -= taken;
            return taken;
        }
        let taken = count.min(self.emeralds);
        self.emeralds -= taken;
        taken
    }
    fn give_items(&mut self, item: &str, count: u32) -> u32 {
        if item == WAGE { self.emeralds += count; }
        count
    }
    fn sneaking(&self) -> bool {
        self.sneak
    }
    fn server_players(&self) -> Vec<horizon_client::mod_api::NativeServerPlayer> {
        self.players.clone()
    }
    fn acting_player(&self) -> Option<horizon_client::mod_api::NativeServerPlayer> {
        self.players.first().cloned()
    }
    fn run_command(&mut self, line: &str) -> bool {
        self.commands.push(line.to_owned());
        true
    }
    fn tell(&mut self, _: u128, line: Text) -> bool {
        self.told.push(line.plain());
        true
    }
    fn action_bar(&mut self, _: u128, line: Text) -> bool {
        self.notices.push(line.plain());
        self.told.push(line.plain());
        true
    }
    fn surface_heights(&self, _: &str, columns: &[(i32, i32)], _: horizon_client::mod_api::NativeHeightmap) -> Result<Vec<Option<i32>>, String> {
        Ok(columns.iter().map(|_| Some(70)).collect())
    }
    fn block_entity_sync(&mut self, _: [i32; 3]) {}
    fn level(&self, _: &str) -> Option<horizon_client::mod_api::LevelRef> {
        None
    }
    /// Sem pasta de mundo: as ruínas (`ruins.rs`) não são postas.
    fn world_dir(&self) -> Option<std::path::PathBuf> {
        None
    }
    fn day_time(&self) -> i64 {
        self.day
    }
    fn summon(&mut self, _: &str, _: &str, at: [f64; 3], _: Compound) -> Option<u32> {
        self.summoned.push(at);
        Some(90000 + self.summoned.len() as u32)
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

// ---------- Zonas, armazéns, centro e vila ----------

/// Marco de zona registrado em `at`, com nível liberado e ativo.
fn marker(village: &mut Village, id: &str, at: [i32; 3], level: i32, filter: Option<&str>) {
    let mut entity = block(id);
    set_number(&mut entity, "Unlocked", level);
    set_number(&mut entity, "Level", level);
    entity.items[0] = filter.map(|item| ItemStack::new(item, 1));
    zones::tick(village, at, &mut entity, 0);
    village.stations.insert(at, entity);
}

#[test]
fn farmhand_in_a_field_zone_plants_tills_and_harvests_beyond_the_old_area() {
    let mut village = Village::new("veio:test_work_zone");
    village.crops.clear();
    // Lavoura nível 2 (13x13) com o marco a 5 blocos do posto.
    marker(&mut village, FIELD_MARKER, [5, 64, 0], 2, None);
    let far = [10, 64, 5];
    village.crops.insert(far, ("minecraft:wheat".into(), 7));
    village.villager(14001, [1.5, 64.0, 1.5], [40, 0, 0, 1]);
    let mut entity = post();
    set_number(&mut entity, "Ration", 20);
    assert!(assign(village.dimension, POST, &mut entity, ([40, 0, 0, 1], 14001), 0));
    let now = cycles_at(&mut village, POST, &mut entity, 1, 80, |post| target(post) == Some(far));
    assert_eq!(get_pos(&entity, "ZoneAt"), Some([5, 64, 0]));
    village.walk(14001, far);
    cycle(&mut village, &mut entity, now + 1);
    assert_eq!(village.age(far), 0, "colheu a 10 blocos do posto, dentro da lavoura");
    village.walk(14001, [1, 64, 0]);
    cycle(&mut village, &mut entity, now + 2);
    assert!(outputs_of(&entity).contains(&("minecraft:wheat_seeds".into(), 1)));

    // Terra arada vazia: planta a semente que colheu.
    let soil = [6, 63, 2];
    village.crops.insert(soil, ("minecraft:farmland".into(), 0));
    let spot = [6, 64, 2];
    let now = cycles_at(&mut village, POST, &mut entity, now + 3, 80, |post| target(post) == Some(spot));
    village.walk(14001, spot);
    cycle(&mut village, &mut entity, now + 1);
    assert_eq!(village.crops.get(&spot).map(|(id, _)| id.as_str()), Some("minecraft:wheat"), "plantou trigo");
    assert!(!outputs_of(&entity).iter().any(|(item, _)| item == "minecraft:wheat_seeds"), "gastou a semente");

    // Grama perto da água vira terra arada.
    let grass = [3, 64, -3];
    village.crops.insert(grass, ("minecraft:grass_block".into(), 0));
    village.crops.insert([3, 64, -1], ("minecraft:water".into(), 0));
    let now = cycles_at(&mut village, POST, &mut entity, now + 2, 80, |post| target(post) == Some(grass));
    village.walk(14001, grass);
    cycle(&mut village, &mut entity, now + 1);
    assert_eq!(village.crops[&grass].0, "minecraft:farmland");
    let work = number(&village.stations[&[5, 64, 0]], "Work");
    assert_eq!(work, 3, "colheita, plantio e aragem contam na zona");
}

#[test]
fn idle_farmhand_in_a_dry_field_scans_one_row() {
    let mut village = Village::new("veio:test_work_zone_dry");
    village.crops.clear();
    marker(&mut village, FIELD_MARKER, [0, 64, 0], 2, None);
    for x in -6..=6 {
        for z in -6..=6 {
            village.crops.insert([x, 64, z], ("minecraft:grass_block".into(), 0));
        }
    }
    village.villager(15001, [1.5, 64.0, 1.5], [50, 0, 0, 1]);
    let mut entity = post();
    set_number(&mut entity, "Ration", 10);
    assert!(assign(village.dimension, POST, &mut entity, ([50, 0, 0, 1], 15001), 0));
    village.looks.set(0);
    cycle(&mut village, &mut entity, 1);
    // Fileira de 9 + ar acima + no máximo 32 blocos de água no primeiro probe.
    assert!(village.looks.get() <= 50, "não sonda água em toda a fileira seca ({})", village.looks.get());
    assert!(village.looks.get() >= farmhand::SCANS as u32);
}

#[test]
fn field_filter_plants_only_the_chosen_seed() {
    let mut village = Village::new("veio:test_work_zone_filter");
    village.crops.clear();
    marker(&mut village, FIELD_MARKER, [2, 64, 0], 1, Some("minecraft:carrot"));
    village.crops.insert([3, 63, 1], ("minecraft:farmland".into(), 0));
    village.villager(14101, [3.5, 64.0, 1.5], [41, 0, 0, 1]);
    let mut entity = post();
    set_number(&mut entity, "Ration", 20);
    entity.items[1] = Some(ItemStack::new("minecraft:wheat_seeds", 4));
    assert!(hire_at(&mut village, &mut entity, POST, 0));
    let now = cycles_at(&mut village, POST, &mut entity, 1, 40, |post| number(post, "Status") == NO_TASK);
    assert_eq!(village.crops.get(&[3, 64, 1]), None, "a lavoura é de cenoura: trigo não é plantado");
    entity.items[2] = Some(ItemStack::new("minecraft:carrot", 4));
    for later in now + 1..now + 41 {
        cycle(&mut village, &mut entity, later);
        if village.crops.contains_key(&[3, 64, 1]) {
            break;
        }
    }
    assert_eq!(village.crops.get(&[3, 64, 1]).map(|(id, _)| id.as_str()), Some("minecraft:carrots"));
    assert_eq!(entity.items[1], Some(ItemStack::new("minecraft:wheat_seeds", 4)), "trigo intacto");
}

#[test]
fn zone_levels_need_work_villagers_and_emeralds_and_the_wrench_picks_the_size() {
    let mut village = Village::new("veio:test_work_zone_level");
    let at = [0, 64, 20];
    marker(&mut village, FIELD_MARKER, at, 1, None);
    village.emeralds = 100;
    assert_eq!(zones::right_click(&mut village, at, WAGE), Some(Flow::Cancel));
    assert_eq!(zones::unlocked(&village.stations[&at]), 1, "sem trabalho na zona, não sobe");
    set_number(village.stations.get_mut(&at).unwrap(), "Work", zones::WORK[0]);
    set_number(village.stations.get_mut(&at).unwrap(), "Villagers", zones::VILLAGERS[0]);
    zones::right_click(&mut village, at, WAGE);
    assert_eq!(zones::unlocked(&village.stations[&at]), 2);
    assert_eq!(zones::side(&village.stations[&at]), 13);
    assert_eq!(village.emeralds, 100 - zones::COST[0]);
    zones::right_click(&mut village, at, WRENCH);
    assert_eq!(zones::side(&village.stations[&at]), 9, "a chave volta ao tamanho menor");
    zones::right_click(&mut village, at, WRENCH);
    assert_eq!(zones::side(&village.stations[&at]), 13);
    // Sem esmeraldas suficientes, nada é gasto.
    set_number(village.stations.get_mut(&at).unwrap(), "Work", 1000);
    set_number(village.stations.get_mut(&at).unwrap(), "Villagers", 50);
    village.emeralds = 3;
    zones::right_click(&mut village, at, WAGE);
    assert_eq!((zones::unlocked(&village.stations[&at]), village.emeralds), (2, 3));
}

#[test]
fn shepherd_works_the_whole_pen_and_follows_its_feed() {
    let mut village = Village::new("veio:test_work_pen");
    village.crops.clear();
    marker(&mut village, PEN_MARKER, [8, 64, 0], 3, Some("minecraft:carrot"));
    village.villager(14201, [13.5, 64.0, 2.5], [42, 0, 0, 1]);
    let mut post = block(SHEPHERD_POST);
    set_number(&mut post, "Ration", 20);
    post.items[1] = Some(ItemStack::new("minecraft:wheat", 8));
    assert!(assign(village.dimension, POST, &mut post, ([42, 0, 0, 1], 14201), 0));
    // Ovelha a 14 blocos do posto, dentro do cercado 17x17.
    animal(&mut village, 14250, "minecraft:sheep", [14.5, 64.0, 2.5], &[("Age", Tag::Int(0)), ("Color", Tag::Byte(0))]);
    animal(&mut village, 14251, "minecraft:cow", [12.5, 64.0, -2.5], &[("Age", Tag::Int(0))]);
    animal(&mut village, 14252, "minecraft:cow", [12.5, 64.0, -3.5], &[("Age", Tag::Int(0))]);
    cycle(&mut village, &mut post, 1);
    assert_eq!(village.animals[&14250].1.get(&"Sheared".into()), Some(&Tag::Byte(1)), "tosquiou dentro do cercado");
    village.walk(14201, [1, 64, 0]);
    cycle(&mut village, &mut post, 2);
    assert_eq!(cycle(&mut village, &mut post, 3), IDLE_WORK, "cercado de cenoura: não dá trigo às vacas");
    assert_eq!(post.items[1], Some(ItemStack::new("minecraft:wheat", 8)));
    assert_eq!(number(&village.stations[&[8, 64, 0]], "Work"), 1);
}

#[test]
fn queue_stores_leftovers_in_the_barn_and_feeds_machines_from_it() {
    let mut village = Village::new("veio:test_work_barn");
    let mut desk = desk_with_foreman(&mut village, 14301, [43, 0, 0, 1]);
    desk.items[0] = Some(ItemStack::new("minecraft:bread", foreman::DESK_FOOD));
    let mut farm = block(FARM_POST);
    farm.items[1] = Some(ItemStack::new("minecraft:white_wool", 10));
    village.stations.insert(MANAGED, farm);
    let barn = [8, 64, 12];
    village.stations.insert(barn, block(storage::BARN));
    village.stations.insert([4, 64, 8], block(production::MILL));
    let now = cycles_at(&mut village, DESK, &mut desk, 1, 100, |desk| !foreman::queue(desk).is_empty());
    let task = foreman::queue(&desk).remove(0);
    assert_eq!((task.from, task.item.as_str(), task.to), (MANAGED, "minecraft:white_wool", barn), "sobra vai ao celeiro");

    // Celeiro com trigo e um moinho vazio: o estoque abastece a máquina.
    village.stations.get_mut(&MANAGED).unwrap().items[1] = None;
    village.stations.get_mut(&barn).unwrap().items[5] = Some(ItemStack::new("minecraft:wheat", 20));
    set_queue_empty(&mut desk);
    cycles_at(&mut village, DESK, &mut desk, now + 1, 8, |desk| foreman::queue(desk).iter().any(|task| task.from == barn));
    let task = foreman::queue(&desk).into_iter().find(|task| task.from == barn).unwrap();
    assert_eq!((task.item.as_str(), task.to, task.into), ("minecraft:wheat", [4, 64, 8], 0));
}

fn set_queue_empty(desk: &mut ModBlockEntity) {
    desk.nbt.remove(&"Queue".into());
}

#[test]
fn silo_only_takes_grain_and_food() {
    assert!(storage::accepts(storage::SILO, 0, &ItemStack::new("minecraft:wheat_seeds", 1)));
    assert!(storage::accepts(storage::SILO, 17, &ItemStack::new("minecraft:bread", 1)));
    assert!(!storage::accepts(storage::SILO, 0, &ItemStack::new("minecraft:iron_ingot", 1)));
    assert!(!storage::accepts(storage::SILO, 18, &ItemStack::new("minecraft:wheat", 1)));
    assert!(storage::accepts(storage::BARN, 26, &ItemStack::new("minecraft:iron_ingot", 1)));
    assert!(storage::layout(storage::BARN).is_some_and(|(slots, inputs, outputs)| slots == 27 && inputs.len() == 27 && outputs.len() == 27));
    assert_eq!(super::super::output_slots(storage::BARN), 0..0, "o bocal não esvazia o celeiro numa esteira");
}

#[test]
fn control_center_dashboard_counts_workers_zones_storage_and_goals() {
    let mut village = Village::new("veio:test_work_center");
    let dimension = village.dimension;
    let at = [100, 64, 100];
    // Postos publicam o estado; armazém e zona se registram.
    let mut busy = block(FARM_POST);
    set_number(&mut busy, "Status", 1);
    busy.nbt.insert("Worker".into(), Tag::IntArray(vec![1, 2, 3, 4]));
    center::report(dimension, [104, 64, 100], &busy, 5);
    let mut hungry = block(HAULER_POST);
    set_number(&mut hungry, "Status", NO_FOOD);
    hungry.nbt.insert("Worker".into(), Tag::IntArray(vec![1, 2, 3, 5]));
    center::report(dimension, [96, 64, 100], &hungry, 5);
    let mut barn = block(storage::BARN);
    barn.items[0] = Some(ItemStack::new("minecraft:bread", 40));
    storage::tick(&mut village, [100, 64, 110], &mut barn, 10);
    marker(&mut village, PEN_MARKER, [110, 64, 100], 1, None);
    let mut console = block(CONTROL_CENTER);
    console.items[1] = Some(ItemStack::new("minecraft:bread", 4));
    center::refresh(&village, at, &mut console, 5);
    let data = center::data(&console);
    assert_eq!(data[12], 0, "página do resumo");
    assert_eq!(center::split(data[1]), (2, 1), "trabalhadores e trabalhando");
    assert_eq!(center::split(data[2]), (1, 1), "esperando e sem comida");
    assert_eq!(center::split(data[3]), (0, 1), "lavouras e cercados");
    assert_eq!(center::split(data[5]), (0, 3 + 1), "fila e ocupação (+1; 0 = sem armazéns)");
    assert_eq!(data[6], 0, "exportação do dia");
    assert_eq!(data[8], 40 * 6 / TASK_COST, "40 pães guardados pagam 120 tarefas");
    assert_eq!(data[9], 62, "meta de 64 pães com 40 guardados");
    assert_eq!(data[10], -1, "slot de meta vazio");
    // Página Atenção: o carregador sem comida, com a direção a partir do centro.
    set_number(&mut console, "Page", 1);
    let data = center::data(&console);
    assert_eq!(data[1], 1, "uma pendência");
    assert_eq!(center::unpack(data[2]), (NO_FOOD, 2, -4, 0, 0), "carregador sem comida 4 blocos a oeste");
}

#[test]
fn villages_grow_with_births_until_the_cap() {
    let mut village = Village::new("veio:test_work_births");
    for (index, id) in (15001..15004).enumerate() {
        village.villager(id, [index as f64, 64.0, 0.0], [50, 0, 0, index as i32]);
    }
    // Procura um instante em que o primeiro aldeão sorteia um nascimento.
    let now = (0..village::CHECK * village::CHANCE * 4)
        .map(|step| step * village::CHECK + (village::CHECK - 15001 % village::CHECK))
        .find(|now| { village::observe(&mut village, "veio:test_work_births", 15001, *now); village::pending("veio:test_work_births") > 0 })
        .expect("algum ciclo sorteia um nascimento");
    assert!(now > 0);
    village::flush(&mut village);
    assert_eq!(village.summoned.len(), 1, "nasceu um bebê");
    assert_eq!(village::pending("veio:test_work_births"), 0);
    // Vila cheia: ninguém nasce.
    for id in 0..village::BASE_CAP as u32 {
        village.villager(15100 + id, [0.5, 64.0, id as f64 * 0.5], [51, 0, 0, id as i32]);
    }
    village.villagers.get_mut(&15001).unwrap().0 = [0.5, 64.0, 0.5];
    village::observe(&mut village, "veio:test_work_births", 15001, now);
    assert_eq!(village::pending("veio:test_work_births"), 0, "acima do teto não há nascimento");
}

#[test]
fn full_houses_stop_births_and_ask_for_more() {
    let dimension = "veio:test_work_houses";
    let mut village = Village::new(dimension);
    village.crops.insert([2, 64, 1], ("minecraft:red_bed".into(), 0));
    village.crops.insert([3, 64, 1], ("minecraft:red_bed".into(), 0));
    village.parts.insert([3, 64, 1], "foot".into());
    village.villager(15200, [0.5, 64.0, 0.5], [52, 0, 0, 9]);
    for now in 0..2_000 {
        village::scan_homes(&village, dimension, 15200, now);
        if village::homes(dimension, [0, 64, 0]).is_some() {
            break;
        }
    }
    assert_eq!(village::homes(dimension, [0, 64, 0]), Some(1), "a cabeceira conta, o pé da cama não");
    for (index, id) in (15201..15204).enumerate() {
        village.villager(id, [index as f64, 64.0, 0.0], [52, 0, 0, index as i32]);
    }
    village.players.push(player(1, "tester", [0.5, 64.0, 0.5]));
    village.players[0].dimension = dimension.to_owned();
    let now = (0..village::CHECK * village::CHANCE * 4)
        .map(|step| step * village::CHECK + (village::CHECK - 15201 % village::CHECK))
        .find(|now| {
            village::observe(&mut village, dimension, 15201, *now);
            !village.notices.is_empty()
        })
        .expect("com uma cama e três aldeões, o jogo pede casas");
    assert_eq!(village::pending(dimension), 0, "sem cama livre, não nasce");
    assert_eq!(village.notices[0], "veio.village.need_houses");
    village.notices.clear();
    // Mais uma cabeceira na volta seguinte: o aviso some e o nascimento volta a ser possível.
    village.crops.insert([4, 64, 1], ("minecraft:white_bed".into(), 0));
    for step in 0..2_000 {
        village::scan_homes(&village, dimension, 15200, now + 1 + step);
        if village::homes(dimension, [0, 64, 0]) == Some(2) {
            break;
        }
    }
    assert_eq!(village::homes(dimension, [0, 64, 0]), Some(2));
}

#[test]
fn idle_hauler_inspects_a_few_stations_per_cycle() {
    let mut village = Village::new("veio:test_work_hauler_looks");
    village.crops.clear();
    village.villager(13401, [1.5, 64.0, 1.5], [34, 0, 0, 1]);
    for i in 0..12 {
        let mut press = block(PRESS);
        set_number(&mut press, "Status", 2);
        village.stations.insert([1 + i % 6, 64, i / 6], press);
    }
    let mut hauler = block(HAULER_POST);
    set_number(&mut hauler, "Ration", 10);
    assert!(hire_at(&mut village, &mut hauler, POST, 0));
    cycles_at(&mut village, POST, &mut hauler, 1, HAULER_SWEEP, |post| number(post, "Rate") >= 12);
    village.edits.set(0);
    cycle(&mut village, &mut hauler, 50);
    assert_eq!(village.edits.get(), hauler::LOOKS as u32, "não inspeciona as 12 máquinas ociosas no mesmo ciclo");
}

#[test]
fn walking_shepherd_follows_the_target_without_rereading_the_herd() {
    let mut village = Village::new("veio:test_work_shepherd_follow");
    let mut post = shepherd_with_worker(&mut village, 13301, [33, 0, 0, 1], [5.5, 64.0, 5.5]);
    for i in 0..8 {
        animal(&mut village, 13310 + i, "minecraft:cow", [1.5, 64.0, 1.5], &[("Age", Tag::Int(0))]);
    }
    animal(&mut village, 13320, "minecraft:sheep", [-2.5, 64.0, 1.5], &[("Age", Tag::Int(0)), ("Color", Tag::Byte(0))]);
    village.reads.set(0);
    cycle(&mut village, &mut post, 1);
    assert!(village.reads.get() <= shepherd::MAX_READS as u32);
    let found = cycles_at(&mut village, POST, &mut post, 2, 12, |entity| number(entity, "Action") == 2);
    village.reads.set(0);
    assert_eq!(cycle(&mut village, &mut post, found + 1), WALKING);
    assert_eq!(village.reads.get(), 1, "só o alvo atual");
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

#[test]
fn village_cap_grows_with_development_up_to_150() {
    let dimension = "veio:test_work_cap";
    let at = [500, 64, 500];
    assert_eq!(village::cap(dimension, at, 5), village::BASE_CAP, "sem desenvolvimento, 30");
    // Cinco postos com trabalhador (+15), um centro com administrador (+3 +20).
    for (index, block_id) in [FARM_POST, FARM_POST, HAULER_POST, SHEPHERD_POST, FOREMAN_DESK, CONTROL_CENTER].iter().enumerate() {
        let mut post = block(block_id);
        post.nbt.insert("Worker".into(), Tag::IntArray(vec![60, index as i32, 0, 1]));
        center::report(dimension, [at[0] + index as i32 * 3, 64, at[2]], &post, 5);
    }
    assert_eq!(village::cap(dimension, at, 5), 30 + 6 * 3 + 20);
    // Muitas zonas no nível máximo: o teto para em 150.
    let mut village = Village::new(dimension);
    for index in 0..12 {
        marker(&mut village, FIELD_MARKER, [at[0] + index * 4, 64, at[2] + 10], 4, None);
    }
    assert_eq!(village::cap(dimension, at, 5), village::MAX_CAP);
    assert_eq!(village::cap(dimension, [at[0] + 500, 64, at[2]], 5), village::BASE_CAP, "outra base, longe, tem o seu teto");
}

// ---------- Spawn no centro da vila ----------

#[test]
fn the_new_village_goes_on_flat_dry_land_without_trees() {
    // Plano e limpo: nota 0. Morro de 9 blocos e copas pesam; água recusa.
    let flat = vec![70; 49];
    assert_eq!(spawn::site_score(&flat, &flat, &flat), Some(0));
    let mut hill = flat.clone();
    hill[10] = 79;
    assert_eq!(spawn::site_score(&hill, &hill, &hill), Some(36));
    let mut canopy = flat.clone();
    canopy[3] = 76;
    canopy[4] = 75;
    assert_eq!(spawn::site_score(&flat, &flat, &canopy), Some(6));
    let mut lake = flat.clone();
    lake[0] = 72;
    assert_eq!(spawn::site_score(&flat, &lake, &lake), None, "água não serve");
}

fn player(uuid: u128, name: &str, at: [f64; 3]) -> horizon_client::mod_api::NativeServerPlayer {
    horizon_client::mod_api::NativeServerPlayer {
        uuid, name: name.to_owned(), dimension: spawn::OVERWORLD.to_owned(), position: at, ..Default::default()
    }
}

#[test]
fn new_players_arrive_at_the_village_center_and_the_control_center_takes_over() {
    spawn::reset();
    let mut village = Village::new(spawn::OVERWORLD);
    village.crops.clear();
    for (index, (x, z)) in [(98.5, 99.5), (101.5, 100.5), (100.5, 102.5), (99.5, 98.5), (102.5, 99.5)].iter().enumerate() {
        village.villager(16001 + index as u32, [*x, 64.0, *z], [70, 0, 0, index as i32]);
    }
    village.villager(16010, [180.5, 64.0, 100.5], [70, 0, 0, 10]);
    spawn::logged_in(7, "Steve", 0);
    spawn::logged_in(8, "bad name;kill", 0);
    // A janela também roda o tick e não vê os jogadores do servidor: nada se perde.
    spawn::tick(&mut village, 20);
    assert!(village.commands.is_empty());
    village.players = vec![player(7, "Steve", [90.0, 64.0, 90.0]), player(9, "Alex", [5000.0, 64.0, 5000.0])];
    spawn::tick(&mut village, 40);
    let joined = village.commands.join("\n");
    assert!(joined.contains("setworldspawn 100 70 100"), "centro da vila: média do grupo, altura do terreno\n{joined}");
    assert!(joined.contains("unless entity @s[tag=veio_village] run tp @s 100 70 100"), "só quem chega pela primeira vez");
    assert!(joined.contains("tag Steve add veio_village"));
    assert!(!joined.contains("bad name"), "nome inseguro não entra em comando");

    // Quem entra longe, com a vila do anfitrião já conhecida, vai para ela e não ganha outra.
    village.commands.clear();
    spawn::logged_in(9, "Alex", 40);
    spawn::tick(&mut village, 60);
    let joined = village.commands.join("\n");
    assert!(joined.contains("tp @s 100 70 100"), "convidado da LAN vai ao centro já conhecido\n{joined}");
    assert!(!joined.contains("place structure"), "não gera uma segunda vila");

    // Sozinho e longe de tudo: espera os chunks, gera uma vila e só depois desiste.
    spawn::reset();
    village.commands.clear();
    village.players = vec![player(9, "Alex", [5000.0, 64.0, 5000.0])];
    spawn::logged_in(9, "Alex", 40);
    spawn::tick(&mut village, 60);
    assert!(village.commands.is_empty(), "ainda esperando os aldeões carregarem");
    spawn::tick(&mut village, 40 + spawn::WAIT + 20);
    // Terreno todo plano no teste: a vila nasce onde o jogador está, e ele já vai para o centro.
    let joined = village.commands.join("\n");
    assert!(joined.contains("place structure minecraft:village_plains 5000 70 5000"), "{joined}");
    assert!(joined.contains("tp @s 5000 70 5000") && joined.contains("tag Alex add veio_village"));
    assert!(!joined.contains("fill"), "sem árvores, nada a limpar");
    assert_eq!(spawn::center(), Some(([5000, 70, 5000], false)));

    // O centro de controle vira o centro da vila; quem renasce volta para lá.
    village.commands.clear();
    spawn::control_center(&mut village, [200, 64, 200]);
    assert_eq!(spawn::center(), Some(([200, 65, 200], true)));
    assert!(village.commands.iter().any(|line| line.ends_with("setworldspawn 200 65 200")));
    assert!(village.commands.iter().any(|line| line.ends_with("spawnpoint @a 200 65 200")));
    let before = village.commands.len();
    spawn::control_center(&mut village, [200, 64, 200]);
    assert_eq!(village.commands.len(), before, "mesmo centro: nada a refazer");
    spawn::respawned(7, "Steve");
    assert_eq!(village.commands.len(), before, "nada de comando dentro do gancho de renascimento");
    spawn::tick(&mut village, 1);
    assert!(village.commands.last().is_some_and(|line| line.ends_with("tp Steve 200 65 200")), "vai no tick seguinte");
    spawn::reset();
}

#[test]
fn hiring_prefers_villagers_without_a_trade() {
    let data = |profession: &str, level: i32| Compound::from([("VillagerData".into(), Tag::Compound(Compound::from([
        ("profession".into(), Tag::String(profession.into())), ("level".into(), Tag::Int(level)),
    ])))]);
    assert_eq!(trade_rank(&data("minecraft:none", 1)), 0);
    assert_eq!(trade_rank(&data("minecraft:nitwit", 1)), 0);
    assert_eq!(trade_rank(&data("minecraft:librarian", 1)), 1);
    assert_eq!(trade_rank(&data("minecraft:librarian", 3)), 2);
    assert_eq!(trade_rank(&Compound::new()), 0, "sem dados de aldeão: livre");
}



#[test]
fn hauler_fills_a_freight_dispatch_and_uses_received_goods_at_local_machines() {
    let mut village = Village::new("veio:test_freight_workers");
    let mut desk = desk_with_foreman(&mut village, 19001, [90, 0, 0, 1]);
    desk.items[0] = Some(ItemStack::new("minecraft:bread", foreman::DESK_FOOD));
    let mut drill = block(DRILL);
    drill.items[0] = Some(ItemStack::new("minecraft:raw_copper", 10));
    village.stations.insert(MANAGED, drill);
    let mut station = block(freight::RAIL);
    station.nbt.insert("FreightExport".into(), Tag::String("minecraft:raw_copper".into()));
    village.stations.insert(MILL_AT, station);
    assert!(hauler::station(freight::RAIL));
    let now = cycles_at(&mut village, DESK, &mut desk, 1, 100, |desk| !foreman::queue(desk).is_empty());
    let task = foreman::queue(&desk).remove(0);
    assert_eq!((task.from, task.to, task.into), (MANAGED, MILL_AT, 0));
    village.stations.insert(DESK, desk);
    village.villager(19002, [10.5, 64.0, 11.5], [90, 0, 0, 2]);
    let mut hauler = block(HAULER_POST);
    set_number(&mut hauler, "Ration", 30);
    assert!(assign(village.dimension, HAULER_AT, &mut hauler, ([90, 0, 0, 2], 19002), now));
    cycle_at(&mut village, HAULER_AT, &mut hauler, now + 1);
    village.walk(19002, [11, 64, 8]);
    cycle_at(&mut village, HAULER_AT, &mut hauler, now + 2);
    assert!(village.stations[&MANAGED].items[0].is_none());
    village.walk(19002, [8, 64, 11]);
    cycle_at(&mut village, HAULER_AT, &mut hauler, now + 3);
    assert_eq!(village.stations[&MILL_AT].items[0], Some(ItemStack::new("minecraft:raw_copper", 10)));
    assert!(carried(&hauler).is_empty());

    // Simulate transport depositing return goods in the received compartment.
    village.stations.get_mut(&MILL_AT).unwrap().items[3] = Some(ItemStack::new(PLATE, 10));
    let assembler = [4, 64, 8];
    village.stations.insert(assembler, block(ASSEMBLER));
    let mut desk = village.stations.remove(&DESK).unwrap();
    set_queue_empty(&mut desk);
    let later = cycles_at(&mut village, DESK, &mut desk, now + 4, 100,
        |desk| foreman::queue(desk).iter().any(|task| task.from == MILL_AT && task.to == assembler));
    let task = foreman::queue(&desk).into_iter().find(|task| task.to == assembler).unwrap();
    assert_eq!((task.from, task.slot, task.item.as_str()), (MILL_AT, 3, PLATE));
    village.stations.insert(DESK, desk);
    cycle_at(&mut village, HAULER_AT, &mut hauler, later + 1);
    cycle_at(&mut village, HAULER_AT, &mut hauler, later + 2);
    village.walk(19002, [4, 64, 9]);
    cycle_at(&mut village, HAULER_AT, &mut hauler, later + 3);
    assert_eq!(village.stations[&assembler].items[0], Some(ItemStack::new(PLATE, 10)));
    assert!(village.stations[&MILL_AT].items[3].is_none());
    assert_eq!(village.stations[&MILL_AT].items[0], Some(ItemStack::new("minecraft:raw_copper", 10)), "dispatch is not grabbed as received stock");
}

#[test]
fn sneaking_with_an_emerald_on_the_barracks_switches_the_class() {
    let mut village = Village::new("veio:test_barracks_click");
    village.stations.insert(POST, block(BARRACKS));
    village.sneak = true;
    // A esmeralda distingue o treino de abrir o painel.
    assert_eq!(right_click(&mut village, POST, WAGE), Some(Flow::Cancel));
    assert_eq!(number(&village.stations[&POST], "Role"), 1, "sem soldado, escolher a classe é de graça");
    village.sneak = false;
    assert_eq!(right_click(&mut village, POST, ""), None, "sem agachar, abre o painel");
}


#[test]
fn worker_orders_reset_navigation_on_cancel_and_refresh_changed_destinations() {
    use horizon_client::mobs::{Ai, Extra, Mob, MobType, MobWorld, Vec3, goals::{Cx, Goal}};
    let mut mob = Mob { id: 98001, rider: None, extra: Extra::default(),
        kind: std::sync::Arc::new(MobType { id: VILLAGER.into() }), position: Vec3::default(), velocity: Vec3::default(), climbing: false, ai: Ai::default() };
    let world = MobWorld::preview(); let mut cx = Cx { w: &world };
    let mut goal = Work::default();
    order(mob.id, [1.0, 64.0, 1.0], defense::clock());
    assert!(goal.can_use(&mut mob, &mut cx));
    goal.cooldown = 15;
    goal.wait = 0;
    order(mob.id, [10.0, 64.0, 10.0], defense::clock());
    for _ in 0..ORDER_POLL { assert!(goal.can_use(&mut mob, &mut cx)); }
    assert_eq!(goal.cooldown, 0);
    assert_eq!(goal.target, Some([10.0, 64.0, 10.0]));
    mob.ai.nav.done = false;
    cancel(mob.id); goal.stop(&mut mob, &mut cx);
    assert!(mob.ai.nav.is_done()); assert!(goal.target.is_none());
}


#[test]
fn shepherd_keeps_riding_pigs_out_of_the_breeding_queue() {
    let mut village = Village::new("veio:saddled_pig_shepherd");
    let mut post = shepherd_with_worker(&mut village, 98011, [98, 0, 0, 11], [2.5, 64.0, 2.5]);
    post.items[1] = Some(ItemStack::new("minecraft:carrot", 5));
    for id in [98012, 98013] {
        animal(&mut village, id, "minecraft:pig", [2.5, 64.0, 2.5], &[("Age", Tag::Int(0)), ("Saddle", Tag::Byte(1))]);
    }
    assert_eq!(cycle(&mut village, &mut post, 1), IDLE_WORK);
    assert_eq!(post.items[1], Some(ItemStack::new("minecraft:carrot", 5)));
    assert!(village.animals.values().all(|(_, nbt)| !nbt.contains_key(&"InLove".into())));
}

// ---------- Medição: vila grande ----------

/// Custo do Veio numa vila grande, por tick do jogo: `entity_tick_pre` de cada aldeão e animal e
/// o objetivo `Work` de cada aldeão (com ordens vivas no mapa). Mede só o código do Veio, no
/// substituto. `cargo test --release -p veio-check bench_large_village -- --ignored --nocapture`
#[test]
#[ignore]
fn bench_large_village() {
    use horizon_client::mobs::{Ai, Mob, MobType, MobWorld, Vec3};
    use horizon_client::mod_api::{EntityTickPreEvent, TickEvent};
    use std::sync::Arc;
    const VILLAGERS: u32 = 150;
    const ANIMALS: u32 = 300;
    const TICKS: u32 = 20000;
    let dimension = "veio:bench_village";
    let mut village = Village::new(dimension);
    village.crops.clear();
    for id in 0..VILLAGERS {
        village.villagers.insert(200_000 + id, ([f64::from(id % 20), 64.0, f64::from(id / 20)], [99, 0, 0, id as i32], 0));
    }
    for id in 0..ANIMALS {
        village.animals.insert(300_000 + id, ([f64::from(id % 30), 64.0, 30.0 + f64::from(id / 30)], Compound::new()));
    }
    let hooks = super::super::hooks();
    let world = MobWorld::preview();
    let kind = Arc::new(MobType { id: VILLAGER.to_owned() });
    let mut mobs: Vec<(Mob, Work)> = (0..VILLAGERS).map(|id| (Mob { rider: None,
        id: 200_000 + id, kind: kind.clone(), position: Vec3 { x: 0.0, y: 64.0, z: 0.0 }, velocity: Vec3::default(), climbing: false, ai: Ai::default(), extra: Default::default(),
    }, Work::default())).collect();
    // Um posto em cada sete aldeões tem ordem viva.
    for id in (0..VILLAGERS).step_by(7) {
        order(200_000 + id, [5.0, 64.0, 5.0], defense::clock());
    }
    let animal_kinds = ["minecraft:cow", "minecraft:sheep", "minecraft:pig", "minecraft:chicken"];
    let started = std::time::Instant::now();
    for _ in 0..TICKS {
        hooks.tick(&mut village, &mut TickEvent::preview());
        for id in 0..VILLAGERS {
            hooks.entity_tick_pre(&mut village, &mut EntityTickPreEvent::preview(dimension, 200_000 + id, VILLAGER));
        }
        for id in 0..ANIMALS {
            hooks.entity_tick_pre(&mut village, &mut EntityTickPreEvent::preview(dimension, 300_000 + id, animal_kinds[(id % 4) as usize]));
        }
        let now = defense::clock();
        for (mob, goal) in &mut mobs {
            let mut cx = Cx { w: &world };
            if goal.can_use(mob, &mut cx) {
                goal.tick(mob, &mut cx);
            }
            // Mantém as ordens vivas, como os postos fazem a cada ciclo.
            if mob.id % 7 == 200_000 % 7 && now % 10 == 0 {
                order(mob.id, [5.0, 64.0, 5.0], now);
            }
        }
    }
    let per_tick = started.elapsed().as_secs_f64() * 1e6 / f64::from(TICKS);
    eprintln!("vila grande: {VILLAGERS} aldeões + {ANIMALS} animais: {per_tick:.1} µs por tick do jogo (Veio)");
}

// ---------- Aldeões guiados pelo cérebro e descanso ----------

#[test]
fn work_goal_hands_the_order_to_the_villager_brain_and_respects_danger() {
    use horizon_client::mobs::{Ai, Mob, MobType, MobWorld, Vec3, villager};
    use horizon_client::mobs::villager_brain::{Act, Brain, Tracker};
    use std::sync::Arc;
    let world = MobWorld::preview();
    let mut mob = Mob { rider: None, id: 17001, kind: Arc::new(MobType { id: VILLAGER.to_owned() }), position: Vec3 { x: 0.0, y: 64.0, z: 0.0 },
        velocity: Vec3::default(), climbing: false, ai: Ai::default(), extra: Default::default() };
    villager::ext_mut(&mut mob).brain = Some(Box::new(Brain::preview(Act::Idle)));
    let mut goal = Work::default();
    order(17001, [5.5, 64.0, 7.5], defense::clock());
    let mut cx = Cx { w: &world };
    assert!(goal.can_use(&mut mob, &mut cx));
    goal.tick(&mut mob, &mut cx);
    let walk = |mob: &Mob| villager::ext(mob).and_then(|ext| ext.brain.as_ref()).and_then(|brain| brain.mem.walk_target.get().copied());
    assert_eq!(walk(&mob).map(|walk| walk.target), Some(Tracker::Block([5, 64, 7])), "o cérebro recebe a ordem como alvo");
    // Em pânico, o objetivo não mexe no cérebro.
    villager::ext_mut(&mut mob).brain.as_mut().unwrap().mem.walk_target.clear();
    villager::ext_mut(&mut mob).brain.as_mut().unwrap().activity = Act::Panic;
    goal.tick(&mut mob, &mut cx);
    assert_eq!(walk(&mob), None, "fugindo de um zumbi, ele foge");
    // Ao parar, o alvo da ordem sai do cérebro.
    villager::ext_mut(&mut mob).brain.as_mut().unwrap().activity = Act::Work;
    goal.tick(&mut mob, &mut cx);
    assert!(walk(&mob).is_some());
    goal.stop(&mut mob, &mut cx);
    assert_eq!(walk(&mob), None);
    cancel(17001);
}

#[test]
fn workers_rest_at_night_and_resume_in_the_morning() {
    let mut village = Village::new("veio:test_work_night");
    village.villager(17101, [5.5, 64.0, 5.5], [80, 0, 0, 1]);
    let mut entity = post();
    set_number(&mut entity, "Ration", 10);
    assert!(hire_at(&mut village, &mut entity, POST, 0));
    village.day = 13000;
    assert_eq!(cycle(&mut village, &mut entity, 1), RESTING);
    assert_eq!(order_for(17101, 1), None, "sem ordem: o aldeão vai dormir");
    assert_eq!(worker(&entity), Some([80, 0, 0, 1]), "continua contratado");
    village.day = 24000 + 1000;
    assert_ne!(cycle(&mut village, &mut entity, 2), RESTING, "de manhã volta a trabalhar");
    let now = cycles_at(&mut village, POST, &mut entity, 3, 40, |post| target(post) == Some(WHEAT));
    assert_eq!(order_for(17101, now), Some(goal(WHEAT)), "e vai até o trigo");
}

#[test]
fn worker_holds_what_it_carries_and_empties_the_hand_after_delivery() {
    let mut village = Village::new("veio:test_work_hand");
    village.villager(17201, [5.5, 64.0, 5.5], [81, 0, 0, 1]);
    let mut entity = post();
    set_number(&mut entity, "Ration", 10);
    assert!(hire_at(&mut village, &mut entity, POST, 0));
    let hand = |village: &Village| village.hands.get(&17201).cloned().flatten();
    cycle(&mut village, &mut entity, 1);
    assert_eq!(village.hands.get(&17201), Some(&None), "primeira vez: a mão é acertada (vazia)");
    village.walk(17201, WHEAT);
    let now = cycles_at(&mut village, POST, &mut entity, 2, 60, |post| !carried(post).is_empty());
    assert_eq!(hand(&village).as_deref(), Some("minecraft:wheat"), "colheu: segura o trigo");
    let chances = village.villager_nbt.get(&17201).and_then(|nbt| nbt.get(&"HandDropChances".into()).cloned());
    assert_eq!(chances, Some(Tag::List(vec![Tag::Float(0), Tag::Float(0)])), "o item da mão não cai na morte");
    village.walk(17201, [1, 64, 0]);
    cycle(&mut village, &mut entity, now + 1);
    assert_eq!(hand(&village), None, "entregou: mão vazia");
    assert!(outputs_of(&entity).contains(&("minecraft:wheat".into(), 1)), "a colheita está no posto, não na mão");
}

#[test]
fn barracks_trains_a_class_and_the_militia_kills_in_three_blows() {
    assert!(barracks::class(3).reach > barracks::class(1).reach, "arqueiro alcança mais que a milícia");
    assert!(barracks::class(2).heal > 0.0 && barracks::class(1).heal == 0.0, "só o guarda se cura");
    assert!(barracks::class(4).damage > barracks::class(1).damage, "o mago bate mais forte");

    let mut village = Village::new("veio:test_barracks");
    village.villager(77022, [2.5, 64.0, 0.5], [77, 0, 0, 1]);
    let mut entity = block(BARRACKS);
    entity.items[0] = Some(ItemStack::new("minecraft:bread", 1));
    entity.items[1] = Some(ItemStack::new("minecraft:emerald", 2));
    assert!(hire_at(&mut village, &mut entity, POST, 0));
    village.stations.insert(POST, entity.clone());
    assert_eq!(barracks::cycle(&mut village, POST), Some(Flow::Cancel));
    assert_eq!(barracks::cycle(&mut village, POST), Some(Flow::Cancel));
    let station = village.stations.get(&POST).unwrap();
    assert_eq!(number(station, "Role"), 2, "milícia e depois guarda");
    assert!(station.items[1].is_none(), "cada troca gasta uma esmeralda");
    assert_eq!(barracks::cycle(&mut village, POST), Some(Flow::Cancel));
    assert_eq!(number(village.stations.get(&POST).unwrap(), "Role"), 2, "sem esmeralda a classe fica");

    // Milícia contra um zumbi de 20 de vida: 7 + 7 + 7.
    set_number(&mut entity, "Role", 1);
    let zombie = Compound::from([("Health".into(), Tag::Float(20f32.to_bits()))]);
    village.animals.insert(77021, ([4.5, 64.0, 0.5], zombie));
    defense::observe(village.dimension, 77021, "minecraft:zombie");
    let life = |village: &Village| match village.animals.get(&77021).and_then(|(_, nbt)| nbt.get(&"Health".into())) {
        Some(Tag::Float(bits)) => Some(f32::from_bits(*bits)),
        _ => None,
    };
    let mut blows = Vec::new();
    for now in 0..7 {
        step(&mut village, POST, &mut entity, now as i64 * EVERY, now);
        if let Some(left) = life(&village)
            && blows.last() != Some(&left)
        {
            blows.push(left);
        }
    }
    assert_eq!(blows, vec![13.0, 6.0], "cada golpe tira 7 de vida de verdade");
    assert!(village.animals.get(&77021).is_none(), "o terceiro golpe mata");
    assert_eq!(number(&entity, "Produced"), 1);
    assert_eq!(entity.items[0], None, "o pão pagou os três golpes");
    assert!(on_duty(77022, 6), "o soldado está de plantão");
}

#[test]
fn barracks_guard_heals_and_stays_on_duty_at_night() {
    let mut village = Village::new("veio:test_barracks_night");
    village.villager(77032, [2.5, 64.0, 0.5], [78, 0, 0, 1]);
    village.lives.insert(77032, 10.0);
    let mut entity = block(BARRACKS);
    entity.items[0] = Some(ItemStack::new("minecraft:bread", 4));
    assert!(hire_at(&mut village, &mut entity, POST, 0));
    set_number(&mut entity, "Role", 2);
    village.day = 13000;
    // Sem NBT de vida, o golpe conta pelo contador: 5 de 20.
    village.animals.insert(77031, ([4.5, 64.0, 0.5], Compound::default()));
    defense::observe(village.dimension, 77031, "minecraft:zombie");
    step(&mut village, POST, &mut entity, 0, 0);
    assert_ne!(number(&entity, "Status"), RESTING, "o quartel não dorme");
    assert_eq!(village.lives.get(&77032), Some(&(10.0 + barracks::GUARD_HEAL)), "o guarda se cura no golpe");
    assert!(village.animals.contains_key(&77031), "um golpe de guarda não mata");
    assert_eq!(barracks::after_blow(None, 77031, 5.0), 10.0, "o contador lembra os golpes");
}

// ---------- Sociedade ----------

fn society_town(dimension: &'static str, seed: i32) -> (Village, ModBlockEntity, ModBlockEntity) {
    const SHOP_POS: [i32; 3] = [20, 64, 0];
    const HALL_POS: [i32; 3] = [60, 64, 0];
    const OFFICE_POS: [i32; 3] = [30, 64, 30];
    let mut village = Village::new(dimension);
    village.day = 1000;
    // Três moradores perto da casa (um deles vira o comerciante) e o prefeito longe dela.
    // Ids próprios por vila: as ordens dos aldeões são globais (por id), e os testes rodam em paralelo.
    static NEXT: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(30000);
    let base = NEXT.fetch_add(10, std::sync::atomic::Ordering::Relaxed);
    village.villager(base + 1, [16.5, 64.0, 1.5], [180, seed, base as i32, 1]);
    village.villager(base + 2, [17.5, 64.0, 0.5], [180, seed, base as i32, 2]);
    village.villager(base + 3, [15.5, 64.0, -1.5], [180, seed, base as i32, 3]);
    village.villager(base + 4, [60.5, 64.0, 1.5], [180, seed, base as i32, 4]);
    let mut shop = block(SHOP);
    assert!(hire_at(&mut village, &mut shop, SHOP_POS, 0));
    let mut hall = block(TOWN_HALL);
    assert!(hire_at(&mut village, &mut hall, HALL_POS, 0));
    society::hall_cycle(&mut village, HALL_POS, &mut hall, 0);
    // A companhia: duas esmeraldas no cofre do centro de controle.
    let mut office = block(CONTROL_CENTER);
    office.items[society::ledger::COMPANY_VAULT] = Some(ItemStack::new(WAGE, 2));
    society::company_cycle(&mut village, OFFICE_POS, &mut office, 0);
    village.stations.insert(OFFICE_POS, office);
    // A indústria entrega 10 pães na loja: atacado a 70% (4 moedas cada), fiado.
    shop.items[1] = Some(ItemStack::new("minecraft:bread", 10));
    set_number(&mut shop, "Status", 1);
    set_number(&mut shop, "Present", 1);
    society::shop_cycle(&mut village, SHOP_POS, &mut shop, 0);
    village.stations.insert(SHOP_POS, shop.clone());
    (village, shop, hall)
}

fn balance(book: society::ledger::Book, village: &Village, pos: [i32; 3]) -> i32 {
    society::ledger::balance(book, village.dimension, pos).unwrap_or(-1)
}

const COMPANY: society::ledger::Book = society::ledger::Book::Company;
const PUBLIC: society::ledger::Book = society::ledger::Book::Public;
const SHOPS: society::ledger::Book = society::ledger::Book::Shop;

#[test]
fn money_circulates_without_appearing_or_vanishing() {
    const HOUSE_POS: [i32; 3] = [16, 64, 0];
    let (mut village, _, _) = society_town("veio:test_society_family", 1);
    let dimension = village.dimension;
    assert_eq!(balance(COMPANY, &village, [30, 64, 30]), 2 * society::COINS_PER_EMERALD, "duas esmeraldas na companhia");
    assert_eq!(balance(PUBLIC, &village, [60, 64, 0]), 0, "a prefeitura só vive de imposto");
    assert_eq!(society::ledger::borders(dimension), (120, 0), "as esmeraldas do cofre entraram na economia");
    let mut house = block(society::HOUSE);
    set_number(&mut house, "Wallet", 100);
    let before = 120 + 100;
    society::house_turn(&mut village, HOUSE_POS, &mut house, 1000);
    assert_eq!(number(&house, "Rate"), 3, "três moradores");
    let pantry: u32 = house.items.iter().flatten().filter(|stack| stack.item.as_ref() == "minecraft:bread").map(|stack| stack.count).sum();
    assert_eq!(pantry, 10, "a família comprou todo o pão");
    // Pão a 9 (valor de referência da bolsa: 6 x 1,5); o comerciante não tinha caixa para o salário.
    assert_eq!(number(&house, "Wallet"), 10, "90 moedas de pão");
    // Varejo: 90, com 4% de imposto retido na fonte.
    assert_eq!(balance(SHOPS, &village, [20, 64, 0]), 87);
    assert_eq!(balance(PUBLIC, &village, [60, 64, 0]), 3);
    // No ciclo seguinte a loja paga o atacado (75%: 6 por pão, 60) à companhia, com 4% retido.
    let mut shop = village.stations.get(&[20, 64, 0]).unwrap().clone();
    society::shop_cycle(&mut village, [20, 64, 0], &mut shop, 10);
    assert_eq!(balance(SHOPS, &village, [20, 64, 0]), 27, "o comerciante fica com a margem");
    assert_eq!(balance(COMPANY, &village, [30, 64, 30]), 120 + 58, "a indústria recebe o atacado líquido");
    assert_eq!(balance(PUBLIC, &village, [60, 64, 0]), 5);
    let after = balance(COMPANY, &village, [30, 64, 30]) + balance(PUBLIC, &village, [60, 64, 0])
        + balance(SHOPS, &village, [20, 64, 0]) + number(&house, "Wallet");
    assert_eq!(after, before, "identidade contábil: nada some, nada aparece");
}

#[test]
fn the_mayor_is_paid_from_public_funds_and_poor_families_get_assistance() {
    let (mut village, _, mut hall) = society_town("veio:test_society_public", 4);
    // Aporte voluntário: 4 esmeraldas na conta pública.
    hall.items[1] = Some(ItemStack::new(WAGE, 4));
    society::hall_cycle(&mut village, [60, 64, 0], &mut hall, 10);
    assert_eq!(balance(PUBLIC, &village, [60, 64, 0]), 240);
    let mut house = block(society::HOUSE);
    society::house_turn(&mut village, [57, 64, 0], &mut house, 1000);
    assert_eq!(number(&house, "Rate"), 1, "o prefeito mora perto da prefeitura");
    // Salário do prefeito e assistência (1 por morador): ambos da conta pública.
    assert_eq!(balance(PUBLIC, &village, [60, 64, 0]), 240 - society::WAGE_PER_TURN - 1);
    assert_eq!(balance(COMPANY, &village, [30, 64, 30]), 120, "a companhia não paga funcionário público");
    assert_eq!(number(&house, "Wallet"), society::WAGE_PER_TURN + 1);
}

#[test]
fn company_workers_are_paid_by_the_company_and_the_cost_lands_on_their_business() {
    let (mut village, _, _) = society_town("veio:test_society_payroll", 5);
    let dimension = village.dimension;
    // Um lavrador contratado que mora numa casa.
    village.villager(18105, [80.5, 64.0, 80.5], [180, 5, 0, 5]);
    let mut farm = block(FARM_POST);
    assert!(hire_at(&mut village, &mut farm, [80, 64, 78], 0));
    // A holding amostra cada bloco uma vez a cada 100 ticks (na fase dele).
    for time in 0..100 {
        super::super::holding::observe(dimension, [80, 64, 78], &farm, time);
    }
    let mut house = block(society::HOUSE);
    society::house_turn(&mut village, [80, 64, 80], &mut house, 1000);
    assert_eq!(number(&house, "Wallet"), society::WAGE_PER_TURN, "salário da companhia");
    assert_eq!(balance(COMPANY, &village, [30, 64, 30]), 120 - society::WAGE_PER_TURN);
    super::super::holding::roll(dimension, 1200);
    let agro = super::super::holding::summary(dimension, super::super::holding::AGRO);
    assert_eq!(agro[7], society::WAGE_PER_TURN, "o salário do lavrador é gasto do agro");
}

#[test]
fn a_player_purchase_brings_emeralds_into_the_economy_with_tax_at_source() {
    let (mut village, shop, _) = society_town("veio:test_society_player", 7);
    let merchant = worker(&shop).and_then(cached_entity).unwrap();
    let offer = horizon_client::mod_api::MerchantOffer {
        base_cost_a: horizon_client::merchant::ItemCost { stack: ItemStack::new(WAGE, 1) },
        cost_b: None, result: ItemStack::new("minecraft:bread", 10), uses: 1, max_uses: 1, reward_exp: false,
        special_price_diff: 0, demand: 0, price_multiplier: 0.0, xp: 0,
    };
    society::traded(&mut village, merchant, &offer);
    assert_eq!(society::ledger::borders(village.dimension), (180, 0), "a esmeralda do jogador entrou na economia");
    assert_eq!(balance(SHOPS, &village, [20, 64, 0]), 58, "60 menos 4% de imposto");
    assert_eq!(balance(PUBLIC, &village, [60, 64, 0]), 2);
}

#[test]
fn prices_follow_local_supply_and_demand() {
    use society::market::target;
    assert_eq!(target(100, 20, 20, false), 100, "um dia de estoque: preço base");
    assert_eq!(target(100, 0, 20, false), 200, "sem estoque e com procura: teto");
    assert_eq!(target(100, 80, 20, false), 50, "quatro dias de estoque: metade");
    assert_eq!(target(100, 50, 0, false), 95, "ninguém compra: barateia devagar");
    assert!(target(100, 20, 20, true) > 100, "famílias sem comida empurram o preço");
    let (mut village, _, _) = society_town("veio:test_society_prices", 6);
    // Uma família leva todo o pão; no turno seguinte a loja reabastecida cobra mais.
    let mut house = block(society::HOUSE);
    set_number(&mut house, "Wallet", 100);
    society::house_turn(&mut village, [16, 64, 0], &mut house, 1000);
    let mut shop = village.stations.get(&[20, 64, 0]).unwrap().clone();
    shop.items[1] = Some(ItemStack::new("minecraft:bread", 4));
    society::shop_cycle(&mut village, [20, 64, 0], &mut shop, 1180);
    assert!(society::market::factor(&shop, "minecraft:bread") > 100, "procura alta e estoque baixo: o pão subiu");
}

#[test]
fn the_merchant_offers_the_stock_at_the_local_price() {
    let (village, shop, _) = society_town("veio:test_society_trade", 2);
    let merchant = worker(&shop).and_then(cached_entity).unwrap();
    let offers = village.villager_nbt.get(&merchant).and_then(|nbt| nbt.get(&"Offers".into()).cloned());
    let Some(Tag::Compound(offers)) = offers else { panic!("o comerciante não recebeu ofertas") };
    let Some(Tag::List(recipes)) = offers.get(&"Recipes".into()) else { panic!("sem receitas") };
    assert_eq!(recipes.len(), 1, "uma oferta por item");
    assert_eq!(society::bundle("minecraft:bread"), Some((6, 1)), "6 pães (54 moedas) por esmeralda");
}

#[test]
fn indicators_measure_families_inequality_and_districts() {
    let home = |x: i32, residents: i32, income: i32| society::Home {
        dimension: "d".into(), pos: [x, 64, 0], built: 0, licensed: true, residents, adults: residents, kids: 0,
        employed: residents / 2, income, hungry: false, owed: false, wallet: 0, sanitation: true, leisure: true, cheer: 50, power: true, commerce: true, crowded: false,
        smog: false, food: 100, housing: 100, services: 100, wealth: 100, seen: 0,
    };
    assert_eq!(home(0, 4, 0).idh(), 100);
    let equal = [home(0, 4, 120), home(10, 4, 120)];
    assert_eq!(society::measure(&equal, 0).gini, 0, "renda igual: Gini zero");
    let unequal = [home(0, 4, 400), home(100, 4, 0)];
    let numbers = society::measure(&unequal, 2);
    assert!(numbers.gini > 40, "renda concentrada: Gini alto ({})", numbers.gini);
    assert_eq!(numbers.districts, 2, "casas a 100 blocos: dois bairros");
    assert_eq!(numbers.population, 10, "oito moradores e dois sem-teto");
    assert!(numbers.idh < 100, "os sem-teto puxam o IDH para baixo");
    assert_eq!(society::request(&numbers, false, 1), (3, 2), "o prefeito pede casa para os sem-teto");
    assert_eq!(society::request(&numbers, true, 1).0, 11, "cofre público vazio vem antes");
    let town: Vec<society::Home> = (0..3).map(|index| home(index * 5, 4, 120)).collect();
    let numbers = society::measure(&town, 0);
    assert_eq!(society::next_phase(1, &numbers, 0), 2, "12 moradores, 3 famílias e IDH alto: vira vila");
    assert_eq!(society::next_phase(2, &numbers, 0), 2, "cidade pede mais gente");
}

#[test]
fn without_a_mayor_zones_stop_at_level_two_and_the_village_at_thirty() {
    let dimension = "veio:test_society_gates";
    assert_eq!(society::zone_cap(dimension, [0, 64, 0], 0), (2, false));
    assert!(society::birth_allowed(dimension, [0, 64, 0], 29, 0));
    assert!(!society::birth_allowed(dimension, [0, 64, 0], 30, 0), "o povoado para em 30");
    assert_eq!(society::zone_limit(4), 4);
    assert_eq!(society::population_limit(4), 150);
}

#[test]
fn the_mayor_phase_sets_the_zone_limit_and_house_permits_need_a_well() {
    let (mut village, _, mut hall) = society_town("veio:test_society_phase", 3);
    let dimension = village.dimension;
    assert_eq!(society::zone_cap(dimension, [30, 64, 0], 1), (2, false), "fase 1 com prefeito");
    set_number(&mut hall, "Phase", 3);
    society::hall_cycle(&mut village, [60, 64, 0], &mut hall, 0);
    assert_eq!(society::zone_cap(dimension, [30, 64, 0], 1).0, 4, "a cidade libera o nível 4");
    // Na fase 3, casa sem poço e sem poste não ganha alvará.
    let mut house = block(society::HOUSE);
    society::house_turn(&mut village, [16, 64, 0], &mut house, 1000);
    assert_eq!(number(&house, "Status"), society::NO_PERMIT);
    assert_eq!(number(&house, "Rate"), 0, "sem alvará, ninguém se muda");
}

// ---------- Obras da vila, painel executivo, foco e lar ----------

fn named_player(dimension: &str, uuid: u128, name: &str, at: [f64; 3]) -> horizon_client::mod_api::NativeServerPlayer {
    horizon_client::mod_api::NativeServerPlayer { uuid, name: name.into(), dimension: dimension.into(), position: at, ..Default::default() }
}

#[test]
fn village_projects_take_site_materials_and_change_the_rules() {
    let mut console = block(CONTROL_CENTER);
    assert_eq!(console.items.len(), 8, "comida, três metas, cofre, saque, obra e decreto");
    assert!(center::accepts(projects::SITE, &ItemStack::new("minecraft:brick", 1)));
    assert!(!center::accepts(projects::SITE, &ItemStack::new("minecraft:dirt", 1)), "o canteiro só aceita materiais de obra");
    // Tijolos vão para o refeitório e depois para a escola; nada some.
    console.items[projects::SITE] = Some(ItemStack::new("minecraft:brick", 50));
    assert!(projects::deliver(&mut console, 0).is_empty());
    assert_eq!(projects::progress(&console, 0, 0), 24);
    assert_eq!(projects::progress(&console, 1, 2), 16);
    assert_eq!(console.items[projects::SITE].as_ref().map(|stack| stack.count), Some(10), "sobra no canteiro");
    assert_eq!(projects::deliver(&mut console, 0), Vec::<usize>::new(), "sobra sem obra que precise fica");
    console.items[projects::SITE] = Some(ItemStack::new("veio:iron_plate", 8));
    projects::deliver(&mut console, 0);
    console.items[projects::SITE] = Some(ItemStack::new("veio:kitchen", 1));
    assert_eq!(projects::deliver(&mut console, 0), vec![0], "refeitório pronto");
    assert_eq!(projects::active(&console, 0) & projects::CANTEEN, 0, "paga, mas sem prédio: ainda sem efeito");
    set_number(&mut console, "Standing", projects::CANTEEN as i32);
    assert_eq!(projects::active(&console, 0) & projects::CANTEEN, projects::CANTEEN, "prédio de pé: efeito ativo");
    let packed = projects::packed(&console, 0, 0);
    assert!(projects::unpack(packed).1, "painel mostra a obra feita");
    assert_eq!(projects::prestige(&console), 25, "obra pronta dá prestígio");
    let (percent, done, index, left, _, locked, _) = projects::unpack(projects::packed(&console, 1, 0));
    assert!(!locked);
    assert_eq!((done, index, left), (false, 0, 24), "escola: faltam 24 papéis");
    assert_eq!(percent, 16 * 100 / 46);
    // Festa: repetível, vale hoje e amanhã e emenda.
    for (item, count) in [("minecraft:cake", 1), ("minecraft:cookie", 16), ("minecraft:bread", 16)] {
        console.items[projects::SITE] = Some(ItemStack::new(item, count));
        projects::deliver(&mut console, 5);
    }
    assert!(projects::active(&console, 5) & projects::FESTIVAL != 0);
    assert!(projects::active(&console, 6) & projects::FESTIVAL != 0);
    assert!(projects::active(&console, 7) & projects::FESTIVAL == 0);
    assert_eq!(projects::percent(&console, 4), 0, "a festa recomeça vazia");
    // Regras.
    assert_eq!(projects::food_cost(0, 3), TASK_COST);
    assert_eq!(projects::food_cost(projects::CANTEEN, 3), 0, "toda 4ª tarefa é de graça");
    assert_eq!(projects::food_cost(projects::CANTEEN, 4), TASK_COST);
    assert_eq!(projects::food_cost(projects::FESTIVAL, 0), 0);
    assert_eq!(projects::food_cost(projects::RATION, 0), 1);
    assert_eq!(projects::food_cost(projects::PLENTY, 0), 3);
    assert_eq!(projects::haul_batch(projects::SMITHY), 24);
    assert_eq!([8, 16, 32].map(|cost| projects::zone_cost(projects::MARKET, cost)), [6, 12, 24]);
    assert_eq!(projects::birth_chance(projects::SCHOOL), Some(3));
    assert_eq!(projects::birth_chance(projects::SCHOOL | projects::FESTIVAL), Some(2));
    assert_eq!(projects::birth_chance(projects::RATION | projects::FESTIVAL), None, "racionamento: ninguém nasce");
}

#[test]
fn canteen_tasks_and_school_reach_the_posts_and_the_village_cap() {
    let dimension = "veio:test_work_projects_reach";
    let mut post = block(FARM_POST);
    set_number(&mut post, "Ration", 8);
    for _ in 0..4 {
        finish_task(&mut post, projects::CANTEEN);
    }
    assert_eq!(number(&post, "Ration"), 2, "quatro tarefas, três pagas");
    let base = village::cap(dimension, [10, 64, 10], 50);
    projects::publish(projects::Village { dimension: dimension.into(), pos: [0, 64, 0], perks: projects::SCHOOL,
        waiting: 0, villagers: 3, cap: 30, funded: 0, requests: 0, seen: 50 });
    assert_eq!(projects::at(dimension, [10, 64, 10], 60), projects::SCHOOL);
    assert_eq!(projects::at(dimension, [200, 64, 10], 60), 0, "longe do centro, sem efeito");
    assert_eq!(village::cap(dimension, [10, 64, 10], 60), base + 15);
    projects::withdraw_village(dimension, [0, 64, 0]);
    assert_eq!(projects::at(dimension, [10, 64, 10], 60), 0, "sem administrador, o efeito para");
}

#[test]
fn attention_page_sorts_packs_and_points_the_way() {
    let dimension = "veio:test_work_attention";
    let at = [300, 64, 300];
    let mut far = block(FARM_POST);
    set_number(&mut far, "Status", NO_WORKER);
    center::report(dimension, [300, 64, 260], &far, 10);
    let mut hungry = block(SHEPHERD_POST);
    set_number(&mut hungry, "Status", NO_FOOD);
    center::report(dimension, [330, 66, 330], &hungry, 10);
    let list = center::issues(dimension, at, 12);
    assert_eq!(list.iter().map(|issue| issue.code).collect::<Vec<_>>(), vec![NO_FOOD, NO_WORKER], "sem comida antes de sem trabalhador");
    let packed = center::pack(&list[0], at);
    assert_eq!(center::unpack(packed), (NO_FOOD, 4, 30, 2, 30));
    let far_issue = center::Issue { code: NO_WORKER, kind: 1, pos: [100, 0, 900] };
    assert_eq!(center::unpack(center::pack(&far_issue, at)), (NO_WORKER, 1, -128, -16, 127), "longe: preso na borda");
    assert_eq!(center::direction(0.0, -5.0), 0, "norte");
    assert_eq!(center::direction(5.0, 0.0), 2, "leste");
    assert_eq!(center::direction(-3.0, 3.0), 5, "sudoeste");
}

#[test]
fn compass_focus_leads_the_player_to_each_issue_until_the_village_is_in_order() {
    let mut village = Village::new("veio:test_work_focus");
    let dimension = village.dimension;
    let hub = [500, 64, 500];
    let mut console = block(CONTROL_CENTER);
    console.nbt.insert("Worker".into(), Tag::IntArray(vec![9, 9, 9, 9]));
    village.stations.insert(hub, console);
    village.players = vec![named_player(dimension, 77, "Bea", [500.5, 65.0, 502.5])];
    let mut hungry = block(FARM_POST);
    set_number(&mut hungry, "Status", NO_FOOD);
    let mut empty = block(HAULER_POST);
    set_number(&mut empty, "Status", NO_WORKER);
    let clock = 2_000_000;
    center::report(dimension, [520, 64, 500], &hungry, clock);
    center::report(dimension, [500, 64, 480], &empty, clock);
    assert_eq!(focus::from_center(&mut village, hub), Some(Flow::Cancel));
    let focus = focus::current(77).expect("foco escolhido");
    assert_eq!(focus.issue.pos, [520, 64, 500], "o mais urgente primeiro");
    // Segundo clique: a próxima pendência.
    focus::choose(&mut village, 77, hub, clock);
    assert_eq!(focus::current(77).unwrap().issue.pos, [500, 64, 480]);
    focus::choose(&mut village, 77, hub, clock);
    assert_eq!(focus::current(77).unwrap().issue.pos, [520, 64, 500], "volta ao começo");
    village.told.clear();
    focus::tick(&mut village, clock + 20);
    assert_eq!(village.told, vec![Text::tr("veio.focus.bar", &[]).plain()], "barra com distância e direção");
    // O jogador chega: a barra explica o que fazer.
    village.players[0].position = [520.5, 64.0, 501.5];
    village.told.clear();
    focus::tick(&mut village, clock + 40);
    assert_eq!(village.told, vec![Text::tr("veio.focus.here", &[&center::advice(NO_FOOD)]).plain()]);
    // Resolvido: vai para a próxima e, no fim, a vila fica em ordem.
    set_number(&mut hungry, "Status", WALKING);
    center::report(dimension, [520, 64, 500], &hungry, clock + 50);
    focus::tick(&mut village, clock + 60);
    assert_eq!(focus::current(77).unwrap().issue.pos, [500, 64, 480], "próxima pendência");
    set_number(&mut empty, "Status", WALKING);
    center::report(dimension, [500, 64, 480], &empty, clock + 70);
    focus::tick(&mut village, clock + 80);
    assert!(focus::current(77).is_none());
    assert_eq!(village.told.last(), Some(&Text::tr("veio.focus.all_clear", &[]).plain()));
}

#[test]
fn home_comfort_reads_vanilla_furniture() {
    let found: Vec<Option<usize>> = ["minecraft:red_bed", "minecraft:oak_door", "minecraft:glass_pane", "minecraft:lantern",
        "minecraft:barrel", "minecraft:smoker", "minecraft:crafting_table", "minecraft:potted_poppy", "minecraft:stone_bricks",
        "minecraft:oak_trapdoor", "minecraft:dirt"].iter().map(|block| home::category(block)).collect();
    assert_eq!(found, vec![Some(0), Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), None, None]);
    assert_eq!(home::comfort(0, 0, 0), 0);
    assert_eq!(home::comfort(0b111_1111, 3, 24), 100, "casa completa");
    assert_eq!(home::comfort(0b1, 1, 12), 15 + 4 + 7, "cama, um enfeite e metade do acabamento");
}

#[test]
fn home_hearth_meals_upgrades_rest_and_daily_order() {
    let mut village = Village::new("veio:test_work_home");
    village.crops.clear();
    let dimension = village.dimension;
    let at = [0, 64, 0];
    village.players = vec![named_player(dimension, 88, "Ana", [1.5, 64.0, 1.5])];
    let mut lar = block(HOME);
    lar.items[home::FUEL] = Some(ItemStack::new("minecraft:coal", 2));
    lar.items[1] = Some(ItemStack::new("minecraft:bread", 3));
    lar.items[2] = Some(ItemStack::new("minecraft:cooked_beef", 1));
    lar.items[3] = Some(ItemStack::new("minecraft:beef", 2));
    assert!(home::accepts(home::FUEL, &ItemStack::new("veio:biomass", 1)));
    assert!(!home::accepts(home::FUEL, &ItemStack::new("minecraft:bread", 1)));
    assert!(home::accepts(home::BUILD, &ItemStack::new("veio:circuit", 1)));
    village.stations.insert(at, lar);
    let mut time = 0;
    let tick = |village: &mut Village, time: &mut i64| {
        let mut lar = village.stations.remove(&at).unwrap();
        home::tick(village, at, &mut lar, *time, 3_000_000);
        village.stations.insert(at, lar);
        *time += 10;
    };
    tick(&mut village, &mut time);
    assert!(village.commands.iter().any(|line| line.contains("spawnpoint Ana 0 65 0")), "{:?}", village.commands);
    assert_eq!(home::home_of("Ana").map(|home| home.pos), Some(at));
    let lar = &village.stations[&at];
    assert_eq!(number(lar, "Status"), 1, "lareira acesa");
    assert_eq!(lar.items[home::FUEL].as_ref().map(|stack| stack.count), Some(1));
    // Refeição em casa: o prato mais valioso, com a lareira acesa.
    village.commands.clear();
    assert_eq!(home::right_click(&mut village, at, home::BOWL), Some(Flow::Cancel));
    assert!(village.commands.iter().any(|line| line.starts_with("effect give Ana minecraft:saturation")));
    assert!(village.commands.iter().any(|line| line.starts_with("effect give Ana minecraft:regeneration")));
    assert!(village.stations[&at].items[2].is_none(), "comeu a carne assada");
    village.commands.clear();
    home::right_click(&mut village, at, home::BOWL);
    assert!(village.commands.is_empty(), "uma refeição a cada 2 minutos");
    // Dia em ordem: comeu, lareira acesa e despensa com 3 itens.
    tick(&mut village, &mut time);
    assert!(village.commands.iter().any(|line| line.starts_with("xp add Ana")), "{:?}", village.commands);
    assert_eq!(number(&village.stations[&at], "Streak"), 1);
    // Melhoria: casa de tijolos.
    for (item, count) in [("minecraft:brick", 24), ("minecraft:glass", 12), ("veio:copper_plate", 8)] {
        village.stations.get_mut(&at).unwrap().items[home::BUILD] = Some(ItemStack::new(item, count));
        tick(&mut village, &mut time);
    }
    assert_eq!(home::level(&village.stations[&at]), 2);
    // Nível 2: a lareira cozinha a carne crua.
    for _ in 0..45 {
        tick(&mut village, &mut time);
    }
    let pantry: Vec<String> = village.stations[&at].items[1..4].iter().flatten().map(|stack| format!("{} {}", stack.item, stack.count)).collect();
    assert!(pantry.iter().any(|line| line.starts_with("minecraft:cooked_beef")), "{pantry:?}");
    // Noite em casa com a lareira acesa: acorda descansado.
    village.day = 13_000;
    tick(&mut village, &mut time);
    village.commands.clear();
    village.day = 100;
    tick(&mut village, &mut time);
    assert!(village.commands.iter().any(|line| line.starts_with("effect give Ana minecraft:speed")), "{:?}", village.commands);
    // Renascer em casa.
    village.commands.clear();
    spawn::respawned(1, "Ana");
    assert!(village.commands.is_empty(), "nada de comando dentro do gancho de renascimento");
    spawn::tick(&mut village, 1);
    assert_eq!(village.commands.last(), Some(&format!("execute in {dimension} run tp Ana 0 65 0")));
    // Painel: oito linhas, com o que falta fazer.
    let data = home::data(&village.stations[&at]);
    assert_eq!(data[3], 2);
    assert!(data[6] & 16 != 0, "ainda há melhoria");
}

#[test]
fn prestige_unlocks_projects_decrees_change_the_rules_and_the_treasury_pays() {
    let mut console = block(CONTROL_CENTER);
    assert_eq!(projects::tier(&console), 0);
    // Biblioteca exige prestígio Vila: os livros vão só para a escola.
    console.items[projects::SITE] = Some(ItemStack::new("minecraft:book", 30));
    projects::deliver(&mut console, 0);
    assert_eq!(projects::progress(&console, 1, 1), 6, "escola");
    assert_eq!(projects::progress(&console, 5, 1), 0, "biblioteca bloqueada");
    assert!(projects::unpack(projects::packed(&console, 5, 0)).5);
    projects::gain(&mut console, 60);
    assert_eq!(projects::tier(&console), 1);
    projects::deliver(&mut console, 0);
    assert_eq!(projects::progress(&console, 5, 1), 16, "liberada com prestígio");
    assert_eq!(console.items[projects::SITE].as_ref().map(|stack| stack.count), Some(8), "sobra fica no slot");
    // Decreto no slot: efeito ativo, item não é gasto.
    assert!(center::accepts(projects::DECREE, &ItemStack::new("minecraft:bell", 1)));
    assert!(!center::accepts(projects::DECREE, &ItemStack::new("minecraft:dirt", 1)));
    console.items[projects::DECREE] = Some(ItemStack::new("minecraft:bell", 1));
    let perks = projects::active(&console, 0);
    assert!(perks & projects::CALL != 0);
    assert_eq!(projects::tier_bits(perks), 1);
    assert_eq!(projects::hire_radius(perks), HIRE_RADIUS * 2.0);
    assert_eq!(projects::extra_cap(perks), 5, "prestígio Vila: teto +5");
    assert_eq!(projects::zone_credit(projects::LIBRARY | projects::PLENTY), 3);
    // Caixa paga as esmeraldas do mercado.
    let (built, used) = projects::pay_with(&mut console, 0, 20);
    assert!(built.is_empty());
    assert_eq!(used, 8);
    assert_eq!(projects::progress(&console, 3, 2), 8);
    assert_eq!(projects::interest(projects::HALL, 200), 8);
    assert_eq!(projects::interest(projects::HALL, 40), 2);
}

#[test]
fn village_requests_fill_each_morning_pay_the_treasury_and_never_lose_items() {
    let mut console = block(CONTROL_CENTER);
    let pos = [40, 64, 40];
    requests::morning(&mut console, pos, 0, 0);
    let open: Vec<requests::Request> = (0..requests::SLOTS).filter_map(|index| requests::get(&console, index)).collect();
    assert_eq!(open.len(), 3, "3 pedidos sem o correio");
    assert!(open.iter().all(|request| requests::POOL[request.wish].tier == 0), "aldeia pede coisas simples");
    let mut wishes: Vec<usize> = open.iter().map(|request| request.wish).collect();
    wishes.dedup();
    assert_eq!(wishes.len(), 3, "sem pedidos repetidos");
    // Entrega parcial e depois completa.
    let first = open[0];
    let item = requests::POOL[first.wish].item;
    console.items[projects::SITE] = Some(ItemStack::new(item, first.need - 1));
    assert_eq!(requests::deliver(&mut console, 0), (0, 0));
    assert!(console.items[projects::SITE].is_none());
    console.items[projects::SITE] = Some(ItemStack::new(item, 5));
    let (earned, done) = requests::deliver(&mut console, 0);
    assert_eq!((earned, done), (requests::POOL[first.wish].reward, 1));
    assert_eq!(console.items[projects::SITE].as_ref().map(|stack| stack.count), Some(4), "sobra fica no slot");
    assert_eq!(requests::open(&console), 2);
    // Manhã: o atendido dá lugar a um novo; os abertos ficam e envelhecem.
    requests::morning(&mut console, pos, 1, projects::POST_OFFICE);
    let after: Vec<requests::Request> = (0..requests::SLOTS).filter_map(|index| requests::get(&console, index)).collect();
    assert_eq!(after.len(), 4, "correio: 4 pedidos");
    assert!(after.iter().all(|request| !request.done()));
    assert_eq!(after.iter().filter(|request| request.age == 1).count(), 2);
    assert_eq!(requests::reward(0, projects::POST_OFFICE), 3, "correio: +50%");
    // Parados por 3 dias sem entrega são trocados.
    requests::morning(&mut console, pos, 2, projects::POST_OFFICE);
    requests::morning(&mut console, pos, 3, projects::POST_OFFICE);
    assert!((0..requests::SLOTS).filter_map(|index| requests::get(&console, index)).all(|request| request.age < 3));
}

#[test]
fn center_morning_sends_a_bulletin_and_delivers_requests() {
    let mut village = Village::new("veio:test_work_bulletin");
    let dimension = village.dimension;
    let pos = [700, 64, 700];
    let mut console = block(CONTROL_CENTER);
    console.nbt.insert("Worker".into(), Tag::IntArray(vec![7, 7, 7, 7]));
    village.players = vec![named_player(dimension, 66, "Caio", [700.5, 65.0, 703.5])];
    village.day = 1000;
    let clock = 3_000_000;
    center::tick(&mut village, pos, &mut console, clock);
    assert_eq!(requests::open(&console), 3, "começa com pedidos");
    assert!(village.told.is_empty(), "sem boletim na primeira vez");
    // Entrega um pedido inteiro pelo slot de obra.
    let request = requests::get(&console, 0).unwrap();
    console.items[projects::SITE] = Some(ItemStack::new(requests::POOL[request.wish].item, request.need));
    center::tick(&mut village, pos, &mut console, clock + 10);
    let reward = requests::POOL[request.wish].reward;
    // Sem prefeitura, a recompensa não tem de onde sair (economia fechada): só o prestígio.
    assert_eq!(projects::progress(&console, 3, 2), 0, "sem caixa público, nada para o mercado");
    assert_eq!(center::coins(&console), 0);
    assert_eq!(projects::prestige(&console), requests::PRESTIGE);
    // Esmeraldas soltas no slot vão para o caixa.
    console.items[projects::SITE] = Some(ItemStack::new(WAGE, 3));
    center::tick(&mut village, pos, &mut console, clock + 20);
    assert!(console.items[projects::SITE].is_none());
    // O mercado pede 8 esmeraldas: o caixa paga o que tiver.
    let _ = reward;
    assert_eq!(projects::progress(&console, 3, 2), 3);
    assert_eq!(center::coins(&console), 0);
    // Manhã seguinte: boletim e prestígio de vila em ordem.
    village.told.clear();
    village.day = 13_000;
    center::tick(&mut village, pos, &mut console, clock + 30);
    village.day = 200;
    center::tick(&mut village, pos, &mut console, clock + 40);
    assert!(village.told.iter().any(|line| line == "veio.bulletin.title"), "{:?}", village.told);
    assert!(village.told.iter().any(|line| line == "veio.bulletin.calm"));
    assert_eq!(projects::prestige(&console), requests::PRESTIGE + 3);
    assert_eq!(number(&console, "ReqToday"), 0, "contagem do dia recomeça");
    // Agachar + esmeralda: saque do caixa.
    let before = village.emeralds;
    let left = (center::coins(&console) / society::COINS_PER_EMERALD) as u32;
    village.stations.insert(pos, console);
    center::cash_out(&mut village, pos);
    assert_eq!(village.emeralds - before, left.min(16));
}

#[test]
fn banquet_brings_free_villagers_home_and_makes_the_host_a_hero() {
    let mut village = Village::new("veio:test_work_banquet");
    village.crops.clear();
    let dimension = village.dimension;
    let at = [900, 64, 900];
    village.players = vec![named_player(dimension, 99, "Davi", [901.5, 64.0, 900.5])];
    village.bag.insert("minecraft:cake".into(), 2);
    for (index, id) in (17001..17009).enumerate() {
        village.villager(id, [905.5 + index as f64, 64.0, 905.5], [70, 0, 0, index as i32]);
    }
    // Um deles trabalha num posto: não é convidado.
    let base = defense::clock();
    assert!(claim([70, 0, 0, 0], dimension, [0, 0, 0], base));
    let mut lar = block(HOME);
    lar.items[home::FUEL] = Some(ItemStack::new("minecraft:coal", 1));
    village.stations.insert(at, lar);
    let tick = |village: &mut Village, time: i64, now: u64| {
        let mut lar = village.stations.remove(&at).unwrap();
        home::tick(village, at, &mut lar, time, now);
        village.stations.insert(at, lar);
    };
    // Lareira apagada: sem banquete.
    assert_eq!(home::right_click(&mut village, at, home::CAKE), Some(Flow::Cancel));
    assert_eq!(village.bag["minecraft:cake"], 2);
    tick(&mut village, 0, base);
    home::right_click(&mut village, at, home::CAKE);
    assert_eq!(village.bag["minecraft:cake"], 1, "o bolo é gasto");
    let guests = home::guests(dimension, at);
    assert_eq!(guests.len(), home::GUESTS);
    assert!(!guests.contains(&17001), "trabalhador não é convidado");
    assert!(village.commands.iter().any(|line| line.contains("hero_of_the_village")), "{:?}", village.commands);
    // Os convidados recebem ordem de ir até a casa.
    tick(&mut village, 10, base + 10);
    assert!(guests.iter().all(|id| order_for(*id, base + 10).is_some()));
    // Uma vez por dia.
    home::right_click(&mut village, at, home::CAKE);
    assert_eq!(village.bag["minecraft:cake"], 1);
    // Acaba: as ordens são retiradas.
    tick(&mut village, 20, base + home::BANQUET_TICKS + 100_000);
    assert!(home::guests(dimension, at).is_empty());
    assert!(guests.iter().all(|id| order_for(*id, base + home::BANQUET_TICKS + 100_000).is_none()));
}

// ---------- Plantas e canteiro de obras ----------

#[test]
fn blueprints_turn_with_the_site_and_keep_one_cell_per_block() {
    for plan in &blueprints::BLUEPRINTS {
        assert!(projects::PROJECTS.iter().any(|project| project.key == plan.key && !project.repeat), "{}: obra com prédio", plan.key);
        let mut seen = std::collections::HashSet::new();
        for cell in plan.cells {
            let (spot, index) = blueprints::decode(*cell);
            assert!(index < blueprints::PALETTE.len());
            assert!((0..3).all(|axis| spot[axis] >= 0 && spot[axis] < plan.size[axis]), "{}: {spot:?}", plan.key);
            assert!(seen.insert(spot), "{}: célula repetida {spot:?}", plan.key);
            assert_ne!(spot, plan.site, "o canteiro não é reconstruído");
        }
    }
    let plan = &blueprints::BLUEPRINTS[0];
    let site = [100, 64, 100];
    // A entrada (última linha da planta) fica do lado para onde o canteiro olha.
    let front = [plan.site[0], 1, plan.size[2] - 1];
    for (facing, rot, dir) in [("south", 0, [0, 1]), ("west", 1, [-1, 0]), ("north", 2, [0, -1]), ("east", 3, [1, 0])] {
        assert_eq!(blueprints::rotation(Some(facing)), rot);
        let at = blueprints::world(plan, site, rot, front);
        assert_eq!([at[0] - site[0], at[2] - site[2]], [dir[0] * (front[2] - plan.site[2]), dir[1] * (front[2] - plan.site[2])], "{facing}");
        assert_eq!(blueprints::world(plan, site, rot, plan.site), site, "o canteiro fica na sua célula");
    }
    assert_eq!(blueprints::turned("facing", "north", 0), "north");
    assert_eq!(blueprints::turned("facing", "south", 1), "west");
    assert_eq!(blueprints::turned("facing", "north", 2), "south");
    assert_eq!(blueprints::turned("facing", "east", 3), "north");
    assert_eq!(blueprints::turned("axis", "x", 1), "z");
    assert_eq!(blueprints::turned("axis", "y", 1), "y");
    assert!(blueprints::protected("minecraft:chest") && blueprints::protected("veio:press"));
    assert!(!blueprints::protected("minecraft:grass_block") && !blueprints::protected(site::SITE));
}

fn funded_village(village: &mut Village, center_at: [i32; 3], bits: u32, now: u64) {
    projects::publish(projects::Village { dimension: village.dimension.into(), pos: center_at, perks: 0, waiting: 0,
        villagers: 10, cap: 40, funded: bits, requests: 0, seen: now });
}

#[test]
fn site_waits_for_payment_stops_on_player_blocks_and_builds_the_canteen() {
    let mut village = Village::new("veio:test_site_build");
    village.crops.clear();
    let dimension = village.dimension;
    let pos = [0, 64, 0];
    let mut site_block = block(SITE);
    let now = defense::clock();
    // Sem centro: espera.
    for time in (0..40).step_by(10) {
        site::tick(&mut village, pos, &mut site_block, time, now);
    }
    assert_eq!(number(&site_block, "Status"), site::SITE_PLAN);
    assert_eq!(number(&site_block, "Utilization"), site::NO_CENTER);
    funded_village(&mut village, [20, 64, 0], 0, now);
    site::tick(&mut village, pos, &mut site_block, 40, now);
    assert_eq!(number(&site_block, "Utilization"), site::NOT_PAID, "obra não paga");
    // Paga: confere o terreno; um baú do jogador dentro da planta para a obra.
    funded_village(&mut village, [20, 64, 0], projects::CANTEEN, now);
    let plan = &blueprints::BLUEPRINTS[0];
    let chest = blueprints::world(plan, pos, 0, [3, 1, 3]);
    village.crops.insert(chest, ("minecraft:chest".into(), 0));
    for time in (50..2000).step_by(10) {
        site::tick(&mut village, pos, &mut site_block, time, now);
        if number(&site_block, "Status") == site::SITE_BLOCKED { break; }
    }
    assert_eq!(number(&site_block, "Status"), site::SITE_BLOCKED);
    let value = number(&site_block, "Utilization");
    assert_eq!([(value & 255) - 128, ((value >> 8) & 255) - 128, ((value >> 16) & 255) - 128],
        [chest[0] - pos[0], 0, chest[2] - pos[2]], "o painel mostra onde está o bloco");
    village.crops.remove(&chest);
    let mut time = 2000;
    while number(&site_block, "ContractStage") != site::BUILDING && time < 8000 {
        site::tick(&mut village, pos, &mut site_block, time, now);
        time += 10;
    }
    assert_eq!(number(&site_block, "ContractStage"), site::BUILDING, "terreno livre: começa a obra");
    // Mestre de obras com pão; ele fica ao lado de cada parte.
    village.villager(19001, [0.5, 64.0, 1.5], [91, 0, 0, 1]);
    site_block.items[0] = Some(ItemStack::new("minecraft:bread", 64));
    assert!(hire_at(&mut village, &mut site_block, pos, now));
    let mut guard = 0;
    while number(&site_block, "ContractStage") != site::DONE && guard < 4000 {
        if let Some(target) = target(&site_block) {
            village.walk(19001, target);
        }
        site::tick(&mut village, pos, &mut site_block, time, now);
        time += 10;
        guard += 1;
    }
    assert_eq!(number(&site_block, "ContractStage"), site::DONE, "o refeitório ficou pronto");
    assert!(worker(&site_block).is_none(), "o mestre de obras foi dispensado");
    // Os blocos estão no mundo, na orientação certa.
    let door = blueprints::world(plan, pos, 0, [5, 1, 8]);
    assert_eq!(village.block(door), "minecraft:spruce_door");
    let chimney_fire = blueprints::world(plan, pos, 0, [1, 11, 0]);
    assert_eq!(village.block(chimney_fire), "minecraft:campfire");
    assert_eq!(village.block(pos), "minecraft:air", "o canteiro não é substituído (no teste ele não está no mapa)");
    // De pé: o centro passa a ver o prédio.
    let found = site::near(dimension, [20, 64, 0], center::RADIUS, now, None);
    assert_eq!(found.erected, projects::CANTEEN);
    assert_eq!(found.operating, 0, "de pé, mas sem cozinheira o refeitório não funciona");
    // Outro canteiro com a mesma planta na mesma vila espera.
    let mut other = block(SITE);
    site::tick(&mut village, [40, 64, 0], &mut other, 0, now);
    assert_eq!(number(&other, "Utilization"), site::ALREADY);
}

fn finished_site(key: &str) -> (ModBlockEntity, &'static blueprints::Blueprint) {
    let index = blueprints::BLUEPRINTS.iter().position(|plan| plan.key == key).unwrap();
    let mut entity = block(SITE);
    set_number(&mut entity, "Rate", index as i32);
    set_number(&mut entity, "Rot", 0);
    set_number(&mut entity, "ContractStage", site::DONE);
    (entity, &blueprints::BLUEPRINTS[index])
}

#[test]
fn every_blueprint_has_a_project_and_staffed_ones_have_furniture() {
    for plan in blueprints::BLUEPRINTS.iter() {
        assert_ne!(blueprints::project_bit(plan), 0, "{} sem obra", plan.key);
        if staff::staffed(plan.role) {
            assert!(!plan.work.is_empty(), "{} sem móveis de trabalho", plan.key);
        }
        if plan.visitors > 0 {
            assert!(!plan.seats.is_empty() && plan.hours[0] < plan.hours[1], "{} sem assentos", plan.key);
        }
        for spot in plan.work.iter().chain(plan.seats) {
            assert!((0..3).all(|axis| spot[axis] >= 0 && spot[axis] < plan.size[axis]), "{}: {spot:?} fora da planta", plan.key);
        }
    }
    let lab = blueprints::BLUEPRINTS.iter().find(|plan| plan.key == "lab").unwrap();
    assert_eq!(lab.role, "scientist");
    assert_eq!(blueprints::project_bit(lab), projects::LAB);
}

#[test]
fn canteen_cook_cooks_and_the_building_starts_operating() {
    let mut village = Village::new("veio:test_staff_cook");
    village.crops.clear();
    village.day = 1000;
    let dimension = village.dimension;
    let pos = [0, 64, 0];
    let now = defense::clock();
    funded_village(&mut village, [20, 64, 0], projects::CANTEEN, now);
    let (mut site_block, _) = finished_site("canteen");
    site::tick(&mut village, pos, &mut site_block, 0, now);
    assert_eq!(site::near(dimension, pos, 8, now, None).operating, 0, "sem cozinheira, parado");
    village.villager(19101, [0.5, 64.0, 1.5], [92, 0, 0, 1]);
    site_block.items[0] = Some(ItemStack::new("minecraft:bread", 64));
    site_block.items[1] = Some(ItemStack::new("minecraft:beef", 8));
    assert!(hire_at(&mut village, &mut site_block, pos, now));
    let mut time = 10;
    for _ in 0..400 {
        if let Some(target) = target(&site_block) {
            village.walk(19101, target);
        }
        site::tick(&mut village, pos, &mut site_block, time, now);
        time += 10;
    }
    let cooked = site_block.items[2].as_ref().expect("a cozinheira cozinhou");
    assert_eq!(cooked.item.as_ref(), "minecraft:cooked_beef");
    assert!(cooked.count >= 3, "cozinhou {} carnes", cooked.count);
    assert_eq!(site::near(dimension, pos, 8, now, None).operating, projects::CANTEEN);
}

#[test]
fn school_calls_children_to_class_and_they_grow() {
    let mut village = Village::new("veio:test_staff_school");
    village.crops.clear();
    village.day = 2000;
    let pos = [0, 64, 0];
    let now = defense::clock();
    funded_village(&mut village, [20, 64, 0], projects::SCHOOL, now);
    let (mut site_block, plan) = finished_site("school");
    village.villager(19201, [0.5, 64.0, 1.5], [93, 0, 0, 1]);
    site_block.items[0] = Some(ItemStack::new("minecraft:bread", 64));
    assert!(hire_at(&mut village, &mut site_block, pos, now));
    village.villager(19202, [3.5, 64.0, 3.5], [94, 0, 0, 1]);
    village.villagers.get_mut(&19202).unwrap().2 = -24000;
    let mut time = 10;
    for _ in 0..30 {
        if let Some(target) = target(&site_block) {
            village.walk(19201, target);
        }
        if let Some(goal) = order_for(19202, now) {
            village.villagers.get_mut(&19202).unwrap().0 = goal;
        }
        site::tick(&mut village, pos, &mut site_block, time, now);
        time += 10;
    }
    assert_eq!(staff::visitors_of(village.dimension, pos), vec![19202], "o aluno foi para a carteira");
    let seat = order_for(19202, now).unwrap();
    assert!(plan.seats.iter().any(|spot| {
        let at = blueprints::world(plan, pos, 0, *spot);
        [f64::from(at[0]) + 0.5, f64::from(at[1]), f64::from(at[2]) + 0.5] == seat
    }));
    assert!(matches!(village.villager_nbt.get(&19202).and_then(|nbt| nbt.get(&"Age".into())), Some(Tag::Int(age)) if *age > -24000),
        "a aula fez o aluno crescer");
    // Fim da aula: o aluno é liberado.
    village.day = 9000;
    site::tick(&mut village, pos, &mut site_block, time, now);
    assert!(order_for(19202, now).is_none());
    assert!(staff::visitors_of(village.dimension, pos).is_empty());
}

#[test]
fn scientist_researches_a_tech_and_the_village_gets_it() {
    let mut entity = finished_site("lab").0;
    assert_eq!(staff::current(&entity), Some(0));
    for item in ["minecraft:bone_meal", "minecraft:wheat_seeds", "minecraft:composter"] {
        entity.items[1] = Some(ItemStack::new(item, 64));
        assert!(staff::accepts_input(entity.items[1].as_ref().unwrap()));
        staff::feed_for_test(&mut entity);
    }
    assert!(staff::tech_missing(&entity, 0).is_none());
    for _ in 0..staff::RESEARCH_WORK {
        staff::research_for_test(&mut entity);
    }
    assert_eq!(staff::known(&entity) & staff::AGRONOMY, staff::AGRONOMY);
    assert_eq!(staff::current(&entity), Some(1));
    let perks = projects::research_bit(staff::AGRONOMY);
    assert_eq!(projects::zone_credit(perks), 2);
    assert_eq!(projects::food_cost(projects::research_bit(staff::NUTRITION), 0), TASK_COST - 1);
}

#[test]
fn village_rewards_come_from_the_town_hall_cash() {
    let (mut village, _, _) = society_town("veio:test_work_reward_public", 9);
    let dimension = village.dimension;
    let (office, hall) = ([30, 64, 30], [60, 64, 0]);
    let mut console = village.stations.remove(&office).unwrap();
    let before = balance(COMPANY, &village, office);
    let public = balance(PUBLIC, &village, hall);
    let (entered, _) = society::ledger::borders(dimension);
    society::ledger::put(society::ledger::Book::Public, dimension, hall, 0, 100, society::ledger::RECEIVED, defense::clock());
    assert_eq!(center::reward(dimension, office, &mut console, 60), 60, "a prefeitura paga");
    assert_eq!(balance(COMPANY, &village, office), before + 60);
    assert_eq!(balance(PUBLIC, &village, hall), public + 40);
    assert_eq!(center::reward(dimension, office, &mut console, 600), public + 40, "paga só o que tem");
    assert_eq!(balance(PUBLIC, &village, hall), 0);
    assert_eq!(center::reward(dimension, office, &mut console, 60), 0, "caixa público vazio: nada");
    assert_eq!(society::ledger::borders(dimension).0, entered, "recompensa não cria moeda");
}

/// Mar em volta de `pos` (7x7, no nível e abaixo).
fn sea(village: &mut Village, pos: [i32; 3]) {
    for dx in -3..=3 {
        for dz in -3..=3 {
            for dy in [-1, 0] {
                let at = [pos[0] + dx, pos[1] + dy, pos[2] + dz];
                if at != pos {
                    village.crops.insert(at, ("minecraft:water".into(), 0));
                }
            }
        }
    }
}

fn economy_total(village: &Village) -> i32 {
    balance(COMPANY, village, [30, 64, 30]) + balance(PUBLIC, village, [60, 64, 0]) + balance(SHOPS, village, [20, 64, 0])
}

#[test]
fn the_world_market_pays_for_complexity_and_reputation_opens_markets() {
    use society::trade::*;
    let dimension = "veio:test_trade_market";
    for day in 0..20 {
        assert!((80..=120).contains(&index(3, day)), "a bolsa oscila entre 80% e 120%");
    }
    assert!(quote(dimension, "veio:gear", 0).is_none(), "sem reputação o mundo não compra componentes");
    assert!(quote(dimension, "veio:iron_plate", 0).is_some(), "processados sempre");
    set_reputation(dimension, 1);
    let gear = quote(dimension, "veio:gear", 0).unwrap();
    let ore = quote(dimension, "minecraft:raw_iron", 0).unwrap();
    assert!(gear >= 5 * ore, "engrenagem vale muito mais que minério ({gear} x {ore})");
    assert!(import_price("minecraft:bread", 0).unwrap() > quote(dimension, "minecraft:bread", 0).unwrap(),
        "importar custa mais que exportar: o preço local fica entre os dois");
    assert_eq!(quote(dimension, WAGE, 0), None, "a esmeralda é a moeda");
    let contract = issue(unlocked_tier(1), 0, 10, &[]);
    assert_eq!(PRODUCTS[contract.product].3, 2, "contratos pedem a camada mais alta liberada");
    assert!(contract.need >= 16 && contract.due >= 13);
}

#[test]
fn a_cargo_port_needs_the_sea_and_ships_on_schedule_with_freight() {
    let (mut village, _, _) = society_town("veio:test_trade_port", 8);
    let dimension = village.dimension;
    let port_pos = [40, 64, 40];
    let mut port = block(society::trade::PORT);
    port.items[0] = Some(ItemStack::new("veio:iron_plate", 16));
    society::tick_block(&mut village, port_pos, &mut port, 0);
    assert_eq!(number(&port, "Status"), society::trade::NO_WATER, "porto em terra não opera");
    sea(&mut village, port_pos);
    port.nbt.remove(&"Water".into());
    let before = economy_total(&village);
    let (entered, left) = society::ledger::borders(dimension);
    // Fase 0 em [40, 64, 40]; o porto parte a cada 2 turnos.
    society::tick_block(&mut village, port_pos, &mut port, 1200);
    assert!(port.items[0].is_some(), "turno ímpar: o navio ainda não saiu");
    society::tick_block(&mut village, port_pos, &mut port, 2400);
    assert!(port.items[0].is_none(), "o navio levou a carga");
    let (entered_after, left_after) = society::ledger::borders(dimension);
    assert!(left_after - left >= 5, "o frete paga o navio, para fora");
    assert_eq!(i64::from(economy_total(&village) - before), (entered_after - entered) - (left_after - left),
        "identidade contábil com o comércio exterior");
    assert!(society::trade::connected(dimension, 1), "o porto é uma conexão externa");
}

#[test]
fn a_freight_station_only_runs_at_the_frontier() {
    let (mut village, _, _) = society_town("veio:test_trade_station", 9);
    let mut near = block(society::trade::STATION);
    society::tick_block(&mut village, [100, 64, 100], &mut near, 0);
    assert_eq!(number(&near, "Status"), society::trade::NOT_FRONTIER, "a 70 blocos da sede não é fronteira");
    let mut far = block(society::trade::STATION);
    society::tick_block(&mut village, [200, 64, 200], &mut far, 0);
    assert_ne!(number(&far, "Status"), society::trade::NOT_FRONTIER, "a 170 blocos, sim");
}

#[test]
fn contracts_pay_a_premium_and_raise_reputation() {
    use society::trade::*;
    let (mut village, _, _) = society_town("veio:test_trade_contracts", 10);
    let dimension = village.dimension;
    roll(dimension, 0);
    let (reputation, contracts) = state(dimension);
    assert_eq!((reputation, contracts.len()), (0, 3), "três contratos no primeiro dia");
    let contract = contracts[0].clone();
    let item = PRODUCTS[contract.product].0;
    let plain = quote(dimension, item, 0).unwrap();
    let paid = export(dimension, [40, 64, 40], item, contract.need as u32, 0, 1);
    assert!(paid > plain * contract.need * 9 / 10, "o contrato paga prêmio");
    assert_eq!(state(dimension).0, 1, "contrato cumprido: reputação +1");
    let _ = &mut village;
}

#[test]
fn shops_import_food_when_short_and_money_leaves_the_economy() {
    let (mut village, _, _) = society_town("veio:test_trade_import", 11);
    let dimension = village.dimension;
    sea(&mut village, [40, 64, 40]);
    let mut port = block(society::trade::PORT);
    society::tick_block(&mut village, [40, 64, 40], &mut port, 0);
    // A família leva o pão; na volta, a loja está vazia e a falta fica anotada.
    let mut house = block(society::HOUSE);
    set_number(&mut house, "Wallet", 100);
    society::house_turn(&mut village, [16, 64, 0], &mut house, 1000);
    for slot in 0..3 {
        house.items[slot] = None;
    }
    society::house_turn(&mut village, [16, 64, 0], &mut house, 1000);
    let mut shop = village.stations.get(&[20, 64, 0]).unwrap().clone();
    assert!(number(&shop, "Short") > 0, "faltou comida");
    let before = economy_total(&village) + number(&house, "Wallet");
    let (_, left) = society::ledger::borders(dimension);
    // Turno da loja em [20, 64, 0]: (time + 20) % 1200 < 10.
    society::shop_cycle(&mut village, [20, 64, 0], &mut shop, 1180);
    let bread: u32 = shop.items[1..7].iter().flatten().filter(|stack| stack.item.as_ref() == "minecraft:bread").map(|stack| stack.count).sum();
    assert!(bread > 0, "o comerciante importou pão");
    let (_, left_after) = society::ledger::borders(dimension);
    assert!(left_after > left, "a importação tira moeda da economia");
    assert_eq!(i64::from(economy_total(&village) + number(&house, "Wallet") - before), -(left_after - left),
        "identidade contábil com a importação");
    assert_eq!(number(&shop, "Duration"), 0, "importado não vira dívida com a indústria");
}

#[test]
fn contracts_never_repeat_a_product_and_news_reach_the_chat() {
    use society::trade::*;
    let dimension = "veio:test_trade_news";
    roll(dimension, 0);
    let (_, contracts) = state(dimension);
    let mut products: Vec<usize> = contracts.iter().map(|contract| contract.product).collect();
    products.sort_unstable();
    products.dedup();
    assert_eq!(products.len(), 3, "três contratos, três produtos diferentes");
    let news = drain(dimension);
    assert_eq!(news.iter().filter(|item| matches!(item, News::Offered(..))).count(), 3, "o chat anuncia os contratos");
    assert!(drain(dimension).is_empty(), "cada aviso sai uma vez");
    assert_eq!(unlocked_tier(0), 1);
    assert_eq!(unlocked_tier(TIER_AT[0]), 2, "a primeira reputação abre os componentes");
    assert_eq!(next_unlock(0), Some(TIER_AT[0]));
    assert_eq!(next_unlock(99), None);
    let board = board(dimension, 0, 5);
    assert!(board.windows(2).all(|pair| pair[0].1 >= pair[1].1), "a bolsa vem da maior cotação para a menor");
}

#[test]
fn a_fulfilled_contract_announces_the_premium_and_opens_a_market() {
    use society::trade::*;
    let (mut village, _, _) = society_town("veio:test_trade_open", 12);
    let dimension = village.dimension;
    roll(dimension, 0);
    drain(dimension);
    let (_, contracts) = state(dimension);
    let contract = contracts.iter().find(|contract| PRODUCTS[contract.product].3 <= 1).cloned().unwrap();
    export(dimension, [40, 64, 40], PRODUCTS[contract.product].0, contract.need as u32, 0, 1);
    let news = drain(dimension);
    assert!(news.iter().any(|item| matches!(item, News::Fulfilled(product, premium) if *product == contract.product && *premium > 0)));
    assert!(news.contains(&News::Opened(2)), "reputação 1: o mundo passa a comprar componentes");
    let _ = &mut village;
}

#[test]
fn locked_cargo_waits_at_the_port_and_reputation_survives_a_reload() {
    use society::trade::*;
    let (mut village, _, _) = society_town("veio:test_trade_locked", 13);
    let dimension = village.dimension;
    sea(&mut village, [40, 64, 40]);
    let mut port = block(PORT);
    port.items[0] = Some(ItemStack::new("veio:speed_module", 2));
    society::tick_block(&mut village, [40, 64, 40], &mut port, 2400);
    assert_eq!(number(&port, "Status"), LOCKED_MARKET, "módulos ainda não têm mercado: a carga espera");
    assert!(port.items[0].is_some());
    // Reputação e contratos ficam no centro de controle e voltam ao reabrir o mundo.
    set_reputation(dimension, 4);
    let mut office = block(CONTROL_CENTER);
    persist(dimension, &mut office, 0);
    persist("veio:test_trade_reloaded", &mut office, 0);
    assert_eq!(state("veio:test_trade_reloaded").0, 4);
    assert_eq!(state("veio:test_trade_reloaded").1.len(), state(dimension).1.len());
}

// ---------- Equilíbrio (as regras de `tools/sim_economy.py`, com o código do jogo) ----------

#[test]
fn balance_a_wage_feeds_two_people_at_reference_prices() {
    let bread = society::market::base_price("minecraft:bread").unwrap();
    let food_day = 20 * bread / 6;
    assert_eq!(bread, 9, "pão: valor de referência 6 x 1,5, igual dentro e fora da vila");
    assert!(society::WAGE_PER_TURN * society::TURNS_PER_DAY as i32 >= 2 * (food_day + 10),
        "um salário paga comida e conforto de duas pessoas");
    assert!(society::LIVING_COST >= food_day + 10, "o custo de vida cobre o pão e os bens");
    // Processar compensa: pão (2 trigos) vale muito mais que os 2 trigos.
    assert!(bread >= 4 * 2 * society::market::base_price("minecraft:wheat").unwrap());
}

#[test]
fn balance_importing_never_loses_money_and_the_ceiling_keeps_food_at_home() {
    use society::{market, trade};
    for day in 0..10 {
        for phase in 1..=4 {
            let tax = society::mayor::tax_rate(phase);
            let base = market::base_price("minecraft:bread").unwrap();
            let ceiling = base * market::import_ceiling("minecraft:bread", day, tax).unwrap() / 100;
            let landed = trade::import_price("minecraft:bread", day).unwrap();
            assert!(ceiling * (100 - tax) / 100 >= landed, "fase {phase}, dia {day}: o comerciante cobre a importação");
            // No teto, o atacado local rende tanto quanto exportar sem saturação: a vila não fica sem comida.
            let product = trade::product("minecraft:bread").unwrap();
            let export = trade::world_cents(product, day) * (100 - tax) / 10_000;
            assert!(ceiling * market::WHOLESALE / 100 >= export * 95 / 100, "fase {phase}, dia {day}: atacado no teto x exportação");
        }
    }
}

#[test]
fn balance_freight_weighs_on_raw_goods_and_saturation_has_a_floor() {
    use society::trade::*;
    let (base, stack, stacks, _) = (schedule(STATION).2, schedule(STATION).3, schedule(STATION).0, ());
    let per_unit = (base + stack * stacks as i32) * 100 / (64 * stacks as i32);
    let ore = reference_cents(product("minecraft:raw_iron").unwrap());
    let gear = reference_cents(product("veio:gear").unwrap());
    assert!(per_unit * 100 / ore > 5 * (per_unit * 100 / gear), "o frete pesa mais no minério que na engrenagem");
    let plate = product("veio:iron_plate").unwrap();
    assert!(spot_cents(plate, 0, 1_000_000) >= world_cents(plate, 0) * SCRAP / 100, "piso de sucata");
    assert!(spot_cents(plate, 0, absorb(1)) * 2 <= world_cents(plate, 0) + 1, "no volume de absorção, metade do preço");
}

#[test]
fn balance_contracts_are_a_separate_buyer_and_open_markets_stay_open() {
    use society::trade::*;
    let (mut village, _, _) = society_town("veio:test_balance_contracts", 14);
    let dimension = village.dimension;
    roll(dimension, 0);
    let (_, contracts) = state(dimension);
    let contract = contracts.iter().find(|contract| PRODUCTS[contract.product].3 <= 1).cloned().unwrap();
    let item = PRODUCTS[contract.product].0;
    let before = quote(dimension, item, 0).unwrap();
    export(dimension, [40, 64, 40], item, contract.need as u32, 0, 1);
    assert_eq!(quote(dimension, item, 0).unwrap(), before, "entregar contrato não derruba a cotação à vista");
    assert_eq!(tier_open(dimension), 2, "contrato cumprido abre componentes");
    // Os outros contratos vencem: a reputação cai, mas o mercado aberto continua aberto.
    roll(dimension, 24_000 * 10);
    assert_eq!(state(dimension).0, 0, "contratos perdidos tiram reputação");
    assert_eq!(tier_open(dimension), 2, "mercado conquistado não fecha");
    let _ = &mut village;
}

// ---------- Ruínas e território das obras ----------

/// Terreno de teste para as ruínas e a proteção: alturas por coluna, blocos e um jogador.
struct Terrain {
    dimension: &'static str,
    world: Option<std::path::PathBuf>,
    time: i64,
    /// (MotionBlockingNoLeaves, OceanFloor) de cada coluna.
    ground: fn(i32, i32) -> (Option<i32>, Option<i32>),
    blocks: HashMap<[i32; 3], String>,
    entities: HashMap<[i32; 3], ModBlockEntity>,
    commands: Vec<String>,
    player: Option<horizon_client::mod_api::NativeServerPlayer>,
    sneak: bool,
    bar: Vec<String>,
}

impl Terrain {
    fn new(dimension: &'static str, ground: fn(i32, i32) -> (Option<i32>, Option<i32>)) -> Self {
        Self { dimension, world: None, time: 100, ground, blocks: HashMap::new(), entities: HashMap::new(),
            commands: Vec::new(), player: None, sneak: false, bar: Vec::new() }
    }
}

impl Host for Terrain {
    fn dimension(&self) -> &'static str {
        self.dimension
    }
    fn block(&self, pos: [i32; 3]) -> String {
        self.blocks.get(&pos).cloned().unwrap_or_else(|| "minecraft:air".to_owned())
    }
    fn surface_heights(&self, _: &str, columns: &[(i32, i32)], map: horizon_client::mod_api::NativeHeightmap) -> Result<Vec<Option<i32>>, String> {
        Ok(columns.iter().map(|(x, z)| {
            let (top, floor) = (self.ground)(*x, *z);
            if map == horizon_client::mod_api::NativeHeightmap::OceanFloor { floor } else { top }
        }).collect())
    }
    fn run_command(&mut self, line: &str) -> bool {
        self.commands.push(line.to_owned());
        true
    }
    fn world_dir(&self) -> Option<std::path::PathBuf> {
        self.world.clone()
    }
    fn level(&self, _: &str) -> Option<horizon_client::mod_api::LevelRef> {
        Some(horizon_client::mod_api::LevelRef::new(self.dimension))
    }
    fn level_game_time(&self, _: horizon_client::mod_api::LevelRef) -> Option<i64> {
        Some(self.time)
    }
    fn edit_blocks(&mut self, _: &str, edit: &horizon_client::mod_api::NativeBlockEdit) -> Result<u64, String> {
        for run in &edit.runs {
            for dy in 0..run.height as i32 {
                self.blocks.insert([run.x, run.y + dy, run.z], edit.palette[run.block as usize].clone());
            }
        }
        Ok(edit.runs.len() as u64)
    }
    fn acting_player(&self) -> Option<horizon_client::mod_api::NativeServerPlayer> {
        self.player.clone()
    }
    fn action_bar(&mut self, _: u128, line: Text) -> bool {
        self.bar.push(line.plain());
        true
    }
    fn sneaking(&self) -> bool {
        self.sneak
    }
    fn change_mod_block_entity(&mut self, pos: [i32; 3], change: &mut dyn FnMut(&mut ModBlockEntity)) -> bool {
        self.entities.get_mut(&pos).map(change).is_some()
    }
    fn block_entity_sync(&mut self, _: [i32; 3]) {}
    fn block_property(&self, _: [i32; 3], _: &str) -> Option<String> {
        None
    }
    fn day_time(&self) -> i64 {
        1000
    }
    fn server_players(&self) -> Vec<horizon_client::mod_api::NativeServerPlayer> {
        Vec::new()
    }
}

fn flat(_: i32, _: i32) -> (Option<i32>, Option<i32>) {
    (Some(70), Some(70))
}

/// Lago a leste do centro (água: as duas alturas diferem) e um morro íngreme ao norte.
fn lake_and_hill(x: i32, z: i32) -> (Option<i32>, Option<i32>) {
    if x > 30 {
        (Some(63), Some(58))
    } else if z < -30 {
        (Some(70 + (-z - 30) / 2), Some(70 + (-z - 30) / 2))
    } else {
        (Some(70), Some(70))
    }
}

#[test]
fn ruins_pick_firm_flat_spots_in_the_ring_facing_the_center() {
    let center = [0, 70, 0];
    let host = Terrain::new("veio:test_ruins_choose", flat);
    let chosen = ruins::choose(&host, host.dimension, center);
    assert_eq!(chosen.len(), ruins::COUNT, "terreno plano: todas as ruínas cabem");
    assert_eq!(chosen, ruins::choose(&host, host.dimension, center), "mesmo terreno, mesmas ruínas");
    let mut keys = std::collections::HashSet::new();
    for ruin in &chosen {
        let plan = &blueprints::BLUEPRINTS[ruin.plan];
        assert!(keys.insert(plan.key), "uma ruína de cada prédio");
        let tier = projects::PROJECTS.iter().find(|project| project.key == plan.key).unwrap().tier;
        assert_eq!(tier, 0, "{}: só obras do começo viram ruína", plan.key);
        let distance = f64::from(ruin.pos[0]).hypot(f64::from(ruin.pos[2]));
        assert!((48.0..=80.0).contains(&distance), "{:?} fora do anel", ruin.pos);
        assert_eq!(ruin.pos[1], 70, "o canteiro fica no chão");
        assert_eq!(ruin.rot, ruins::facing_center(ruin.pos, center), "entrada virada para o centro");
        // A entrada da planta aponta para o centro.
        let (dx, dz) = blueprints::turn(0, 1, ruin.rot);
        assert!(dx * -ruin.pos[0].signum() >= 0 && dz * -ruin.pos[2].signum() >= 0, "{:?} rot {}", ruin.pos, ruin.rot);
    }
    // Sem sobrepor, com a folga do território.
    for (index, a) in chosen.iter().enumerate() {
        let (alow, ahigh) = site::territory(&blueprints::BLUEPRINTS[a.plan], a.pos, a.rot);
        for b in &chosen[index + 1..] {
            let (blow, bhigh) = site::territory(&blueprints::BLUEPRINTS[b.plan], b.pos, b.rot);
            assert!(ahigh[0] < blow[0] || bhigh[0] < alow[0] || ahigh[2] < blow[2] || bhigh[2] < alow[2],
                "{:?} e {:?} se sobrepõem", a.pos, b.pos);
        }
    }

    // Água e morro ficam de fora; sem chunk carregado, nada.
    let host = Terrain::new("veio:test_ruins_terrain", lake_and_hill);
    let chosen = ruins::choose(&host, host.dimension, center);
    assert!(!chosen.is_empty());
    for ruin in &chosen {
        let (low, high) = blueprints::bounds(&blueprints::BLUEPRINTS[ruin.plan], ruin.pos, ruin.rot);
        assert!(high[0] <= 30, "{:?} na água", ruin.pos);
        let steep = |z: i32| z < -30 - 2 * ruins::SLOPE;
        assert!(!steep(low[2]) || !steep(high[2]), "{:?} no morro", ruin.pos);
    }
    let host = Terrain::new("veio:test_ruins_unloaded", |_, _| (None, None));
    assert!(ruins::choose(&host, host.dimension, center).is_empty());
}

#[test]
fn ruin_stamp_keeps_low_layers_without_furniture_and_is_deterministic() {
    let plan = blueprints::BLUEPRINTS.iter().find(|plan| plan.key == "canteen").unwrap();
    let pos = [55, 70, 3];
    let cells = ruins::ruin_cells(plan, pos, 1);
    assert!(!cells.is_empty());
    assert_eq!(cells, ruins::ruin_cells(plan, pos, 1), "a mesma ruína sempre");
    let full = plan.cells.iter().filter(|cell| {
        let (spot, index) = blueprints::decode(**cell);
        spot[1] < 4 && index != 0
    }).count();
    assert!(cells.len() < full, "só parte das camadas de baixo sobra");
    for (at, index) in &cells {
        let block = blueprints::PALETTE[*index];
        assert!(blueprints::PROPS[*index].is_empty() && !blueprints::protected(block) && *index != 0, "{block}");
        assert!(at[1] - pos[1] + plan.site[1] < 4, "camada alta em {at:?}");
    }
    // O carimbo não toca no canteiro nem em blocos protegidos que já estão ali.
    let mut host = Terrain::new("veio:test_ruins_stamp", flat);
    let (chest, _) = cells[0];
    host.blocks.insert(chest, "minecraft:chest".into());
    host.blocks.insert(pos, site::SITE.into());
    ruins::stamp(&mut host, pos, plan, 1);
    assert_eq!(host.block(chest), "minecraft:chest");
    assert_eq!(host.block(pos), site::SITE);
    assert!(cells[1..].iter().all(|(at, index)| *at == pos || host.block(*at) == blueprints::PALETTE[*index]));
}

#[test]
fn ruins_are_placed_once_per_new_world_and_their_sites_keep_a_fixed_plan() {
    let unique = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
    let world = std::env::temp_dir().join(format!("veio-ruins-{}-{unique}", std::process::id()));
    let center = [0, 70, 0];
    let mut host = Terrain::new("veio:test_ruins_world", flat);
    host.world = Some(world.clone());
    let dimension = host.dimension;
    ruins::arrive(&mut host, dimension, center);
    let placed: Vec<&String> = host.commands.iter().filter(|line| line.contains("setblock")).collect();
    assert_eq!(placed.len(), ruins::COUNT, "{:?}", host.commands);
    assert!(placed.iter().all(|line| line.contains("if block") && line.contains("#minecraft:replaceable")
        && line.contains("veio:site[facing=")), "só onde o bloco é substituível");
    assert!(host.commands.iter().any(|line| line.contains("veio.spawn.ruins")));
    let expected = ruins::choose(&host, host.dimension, center);
    // De novo (outro jogador, mundo reaberto): nada se repete.
    let before = host.commands.len();
    let dimension = host.dimension;
    ruins::arrive(&mut host, dimension, center);
    assert_eq!(host.commands.len(), before, "as ruínas só aparecem uma vez por mundo");

    // O primeiro tick do canteiro pega a planta escolhida, fixa, e carimba a ruína.
    let ruin = &expected[0];
    let mut entity = block(SITE);
    let now = defense::clock();
    let due = 10 - i64::from((ruin.pos[0] ^ ruin.pos[2]) & 7);
    site::tick(&mut host, ruin.pos, &mut entity, due, now);
    assert_eq!(site::plan_index(&entity), ruin.plan);
    assert_eq!(number(&entity, "Rot"), ruin.rot);
    assert!(site::fixed(&entity));
    let walls = ruins::ruin_cells(&blueprints::BLUEPRINTS[ruin.plan], ruin.pos, ruin.rot);
    assert!(walls.iter().all(|(at, index)| host.block(*at) == blueprints::PALETTE[*index]), "a ruína está no chão");
    // A chave não troca a planta da ruína.
    host.entities.insert(ruin.pos, entity);
    assert_eq!(site::cycle(&mut host, ruin.pos), Some(Flow::Cancel));
    assert_eq!(site::plan_index(&host.entities[&ruin.pos]), ruin.plan);
    // Ruína à espera: o território vale, mas a obra não conta como ocupada na vila.
    let guard = site::guard_at(host.dimension, walls[0].0, now).expect("território da ruína");
    assert!(guard.fixed && guard.site == ruin.pos);
    assert_eq!(site::near(host.dimension, ruin.pos, 8, now, None), site::Near::default());
    // Outro canteiro qualquer não vira ruína.
    let mut other = block(SITE);
    site::tick(&mut host, [500, 70, 500], &mut other, 0, now);
    assert!(!site::fixed(&other));

    // Mundo que já existia (primeiro dia passou): sem ruínas, nem depois.
    let old = std::env::temp_dir().join(format!("veio-ruins-old-{}-{unique}", std::process::id()));
    let mut host = Terrain::new("veio:test_ruins_old", flat);
    host.world = Some(old.clone());
    host.time = ruins::FRESH + 1;
    let dimension = host.dimension;
    ruins::arrive(&mut host, dimension, center);
    host.time = 0;
    let dimension = host.dimension;
    ruins::arrive(&mut host, dimension, center);
    assert!(!host.commands.iter().any(|line| line.contains("setblock")));
    let _ = std::fs::remove_dir_all(world);
    let _ = std::fs::remove_dir_all(old);
}

fn publish_site(host: &mut Terrain, pos: [i32; 3], phase: i32, fixed: bool) -> ModBlockEntity {
    let mut entity = block(SITE);
    set_number(&mut entity, "Rot", 0);
    set_number(&mut entity, "ContractStage", phase);
    if fixed {
        set_number(&mut entity, "Fixed", 1);
    }
    // Ciclo do registro: a cada 10 ticks.
    let time = 10 - i64::from((pos[0] ^ pos[2]) & 7);
    if phase == site::BUILDING {
        // Sem mestre de obras: só publica.
        set_number(&mut entity, "Cursor", 0);
    }
    site::tick(host, pos, &mut entity, time, defense::clock());
    entity
}

#[test]
fn territory_covers_the_plan_plus_margin_only_for_ruins_and_real_works() {
    let plan = &blueprints::BLUEPRINTS[0];
    let pos = [0, 70, 0];
    let (low, high) = site::territory(plan, pos, 0);
    let (inner_low, inner_high) = blueprints::bounds(plan, pos, 0);
    assert_eq!(low, inner_low.map(|value| value - site::MARGIN));
    assert_eq!(high, inner_high.map(|value| value + site::MARGIN));
    assert!(site::guards(true, site::PLAN) && site::guards(false, site::BUILDING) && site::guards(false, site::DONE));
    assert!(!site::guards(false, site::PLAN) && !site::guards(false, site::SURVEYING), "planta do jogador não protege");

    // Canteiro do jogador na planta: nada protegido. Em obra: protegido e conta como ocupado.
    let mut host = Terrain::new("veio:test_territory_registry", flat);
    let now = defense::clock();
    publish_site(&mut host, pos, site::PLAN, false);
    assert!(site::guard_at(host.dimension, [low[0], 70, low[2]], now).is_none());
    publish_site(&mut host, pos, site::DONE, false);
    let corner = site::guard_at(host.dimension, low, now).expect("canto do território");
    assert_eq!(corner, site::Guard { site: pos, fixed: false, reserved: plan.reserved_block });
    assert!(site::guard_at(host.dimension, [high[0] + 1, 70, 0], now).is_none(), "fora da folga");
    assert_eq!(site::near(host.dimension, pos, 8, now, None).erected, blueprints::project_bit(plan));
}

#[test]
fn protection_frees_natural_ground_creative_players_and_food() {
    for natural in ["minecraft:grass_block", "minecraft:dirt", "minecraft:poppy", "minecraft:oak_leaves", "minecraft:sand",
        "minecraft:gravel", "minecraft:snow", "minecraft:short_grass", "minecraft:red_tulip"]
    {
        assert!(territory::natural(natural), "{natural}");
    }
    for built in ["minecraft:bricks", "minecraft:sandstone", "minecraft:flower_pot", "minecraft:oak_planks", "veio:site",
        "minecraft:spruce_door", "minecraft:white_carpet"]
    {
        assert!(!territory::natural(built), "{built}");
    }
    assert!(territory::harmless("minecraft:bread", "") && territory::harmless("minecraft:cooked_beef", ""));
    assert!(territory::harmless(WAGE, "") && territory::harmless("veio:wrench", ""));
    assert!(territory::harmless("veio:shop", "veio:shop"), "a loja vai no lugar reservado da praça");
    for refused in ["minecraft:cobblestone", "minecraft:water_bucket", "minecraft:lava_bucket", "minecraft:flint_and_steel",
        "minecraft:iron_axe", "veio:shop", "veio:press", "minecraft:cake"]
    {
        assert!(!territory::harmless(refused, ""), "{refused}");
    }
    let guard = site::Guard { site: [0, 70, 0], fixed: false, reserved: "" };
    assert!(territory::may_break(&guard, [2, 70, 2], "minecraft:dirt", false));
    assert!(!territory::may_break(&guard, [2, 71, 2], "minecraft:bricks", false));
    assert!(territory::may_break(&guard, [0, 70, 0], SITE, false), "o canteiro do jogador sai");
    let ruin = site::Guard { fixed: true, ..guard };
    assert!(!territory::may_break(&ruin, [0, 70, 0], SITE, true), "o canteiro da ruína fica");
    assert!(territory::may_break(&ruin, [2, 71, 2], "minecraft:chest", true), "baú no caminho da obra sai");
    assert!(!territory::may_break(&ruin, [2, 71, 2], "minecraft:chest", false), "na obra, os móveis ficam");
}

#[test]
fn protection_hooks_cancel_breaking_building_and_explosions_inside_the_grounds() {
    let mut host = Terrain::new("veio:test_territory_hooks", flat);
    let pos = [0, 70, 0];
    let plan = &blueprints::BLUEPRINTS[0];
    let entity = publish_site(&mut host, pos, site::DONE, false);
    host.entities.insert(pos, entity);
    let (low, high) = site::territory(plan, pos, 0);
    let wall = [low[0] + site::MARGIN, 71, low[2] + site::MARGIN];
    host.blocks.insert(wall, "minecraft:bricks".into());
    host.blocks.insert([wall[0], 69, wall[2]], "minecraft:grass_block".into());
    let outside = [high[0] + 2, 71, 0];
    host.blocks.insert(outside, "minecraft:bricks".into());
    host.player = Some(horizon_client::mod_api::NativeServerPlayer { uuid: 5, game_mode: 0, ..Default::default() });

    assert_eq!(territory::destroy(&mut host, wall, false), Flow::Cancel);
    assert_eq!(territory::destroy(&mut host, wall, true), Flow::Cancel);
    assert_eq!(host.bar.last().map(String::as_str), Some("veio.site.protected"));
    assert_eq!(territory::destroy(&mut host, [wall[0], 69, wall[2]], true), Flow::Continue, "grama é livre");
    assert_eq!(territory::destroy(&mut host, outside, true), Flow::Continue, "fora do território");
    assert_eq!(territory::use_item(&mut host, "minecraft:cobblestone", [wall[0], 69, wall[2]]), Flow::Cancel);
    assert_eq!(territory::use_item(&mut host, "minecraft:bread", wall), Flow::Continue, "comer passa");
    assert_eq!(territory::use_item(&mut host, "minecraft:cobblestone", outside), Flow::Continue);
    // Abrir o canteiro com um item na mão passa; agachado (pôr bloco), não.
    host.blocks.insert(pos, SITE.into());
    assert_eq!(territory::use_item(&mut host, "veio:iron_plate", pos), Flow::Continue);
    host.sneak = true;
    assert_eq!(territory::use_item(&mut host, "veio:iron_plate", pos), Flow::Cancel);
    host.sneak = false;
    // Criativo passa em tudo.
    host.player.as_mut().unwrap().game_mode = 1;
    assert_eq!(territory::destroy(&mut host, wall, true), Flow::Continue);
    assert_eq!(territory::use_item(&mut host, "minecraft:lava_bucket", wall), Flow::Continue);
    host.player.as_mut().unwrap().game_mode = 0;
    // Explosão: os tijolos ficam, a grama e o que está fora voam.
    let mut blocks = vec![wall, [wall[0], 69, wall[2]], outside];
    territory::explosion(&host, host.dimension, &mut blocks);
    assert_eq!(blocks, vec![[wall[0], 69, wall[2]], outside]);
    // O jogador tira o próprio canteiro: o prédio sai do registro na hora.
    assert_eq!(territory::destroy(&mut host, pos, true), Flow::Continue);
    assert!(site::guard_at(host.dimension, wall, defense::clock()).is_none());
}

// ---------- Agenda da vila ----------

const DAY_TICKS: i64 = 24_000;

#[test]
fn the_calendar_has_holidays_sundays_and_weekend_events() {
    use society::agenda::*;
    let key = |day: i64| scheduled(day).map(|event| EVENTS[event].key);
    assert_eq!(key(0), Some("founding"), "primavera, dia 1: Dia da Fundação");
    assert_eq!(key(3), Some("flowers"), "primavera, dia 4: Festa das Flores");
    assert!(is_holiday(0) && is_holiday(3));
    assert!(sunday(5) && !is_holiday(5), "primavera, dia 6: domingo comum");
    assert!(matches!(key(5), Some("fair" | "dance" | "fishing")), "domingo tem evento menor");
    assert_eq!(key(8), Some("saint_john"), "verão, dia 3: São João");
    assert_eq!(key(15), Some("harvest"), "outono, dia 4: Festa da Colheita");
    assert_eq!(key(20), Some("lights"), "inverno, dia 3: Festa das Luzes");
    assert_eq!(key(23), Some("new_year"), "inverno, dia 6: Virada do Ano (feriado no lugar do domingo)");
    assert_eq!(key(1), None, "dia útil");
    let holidays = (0..24).filter(|day| is_holiday(*day)).count();
    assert_eq!(holidays, 6, "seis feriados por ano");
    assert_eq!(next_holiday(1), (1, 2), "do dia 2 da primavera, a Festa das Flores é daqui a 2 dias");
}

#[test]
fn a_visitor_brings_a_request_and_delivering_it_prepares_the_holiday() {
    use society::agenda::*;
    let (mut village, _, mut hall) = society_town("veio:test_agenda_request", 21);
    let dimension = village.dimension;
    // Dia 2 da primavera: a Festa das Flores (dia 4) chega com 2 dias de antecedência.
    village.day = DAY_TICKS + 1000;
    hall.items[1] = Some(ItemStack::new(WAGE, 4));
    // Turno da prefeitura em [60, 64, 0]: (time + 60) % 1200 < 10.
    society::hall_cycle(&mut village, [60, 64, 0], &mut hall, 1140);
    let open = proposals(&hall);
    assert_eq!(open.len(), 1);
    assert_eq!(EVENTS[open[0].event].key, "flowers");
    assert_eq!(open[0].due, 3);
    assert_eq!(village.summoned.len(), 1, "a florista chegou à prefeitura");
    // Entrega no espaço Agenda: 16 papoulas, 16 dentes-de-leão e 4 garrafas de mel.
    let public_before = balance(PUBLIC, &village, [60, 64, 0]);
    let company_before = balance(COMPANY, &village, [30, 64, 30]);
    for (item, count) in [("minecraft:poppy", 16), ("minecraft:dandelion", 16), ("minecraft:honey_bottle", 4)] {
        hall.items[AGENDA_SLOT] = Some(ItemStack::new(item, count));
        society::hall_cycle(&mut village, [60, 64, 0], &mut hall, 1150);
    }
    assert!(proposals(&hall)[0].prepared, "festa preparada");
    let paid = public_before - balance(PUBLIC, &village, [60, 64, 0]);
    assert!(paid > 0, "a prefeitura reembolsa o que foi entregue");
    assert_eq!(balance(COMPANY, &village, [30, 64, 30]) - company_before, paid, "o reembolso vai para a companhia");
    // No dia da festa, o ânimo sobe e o pedido sai da lista.
    let before = morale(&hall);
    village.day = 3 * DAY_TICKS + 1000;
    society::hall_cycle(&mut village, [60, 64, 0], &mut hall, 1160);
    // O ânimo volta 3 pontos por dia para 50 (estava abaixo: o Dia da Fundação não foi preparado).
    assert_eq!(morale(&hall), before + 3 + EVENTS[1].cheer, "festa preparada: ânimo +20");
    assert!(proposals(&hall).iter().all(|proposal| proposal.due > 3));
    let _ = dimension;
}

#[test]
fn on_a_holiday_the_village_stops_working_and_an_unprepared_one_lowers_morale() {
    use society::agenda::*;
    let (mut village, _, mut hall) = society_town("veio:test_agenda_holiday", 22);
    // Dia da Fundação (primavera, dia 1 = dia 0), sem preparo.
    village.day = 1000;
    society::hall_cycle(&mut village, [60, 64, 0], &mut hall, 1140);
    assert!(morale(&hall) < 50, "feriado sem festa: ânimo cai");
    let mut farm = block(FARM_POST);
    village.villager(18201, [62.5, 64.0, 2.5], [182, 22, 0, 1]);
    assert!(hire_at(&mut village, &mut farm, [62, 64, 4], 0));
    set_number(&mut farm, "Ration", 10);
    // Fase do posto em [62, 64, 4]: (62 ^ 4) & 7 = 2, então os ciclos caem em 8, 18, 28...
    step(&mut village, [62, 64, 4], &mut farm, 8, 1);
    assert_eq!(number(&farm, "Status"), HOLIDAY, "feriado: o posto descansa");
    village.day = DAY_TICKS + 1000;
    step(&mut village, [62, 64, 4], &mut farm, 18, 2);
    assert_ne!(number(&farm, "Status"), HOLIDAY, "no dia seguinte, volta ao trabalho");
}

#[test]
fn a_civic_request_is_fulfilled_by_building_a_park_and_gives_leisure() {
    use society::agenda::*;
    let (mut village, _, mut hall) = society_town("veio:test_agenda_civic", 23);
    let dimension = village.dimension;
    hall.items[1] = Some(ItemStack::new(WAGE, 10));
    let mut house = block(society::HOUSE);
    society::house_turn(&mut village, [16, 64, 0], &mut house, 1000);
    // Dia útil com obra cívica: (dia % 4) == (60 ^ 0) & 3 = 0 → dia 4 (primavera, dia 5).
    village.day = 4 * DAY_TICKS + 1000;
    society::hall_cycle(&mut village, [60, 64, 0], &mut hall, 1140);
    let civic = proposals(&hall).into_iter().find(|proposal| EVENTS[proposal.event].kind == Kind::Civic).expect("pedido de obra cívica");
    assert_eq!(civic.target, [16, 64, 0], "o bairro sem lazer");
    let block_id = EVENTS[civic.event].amenity;
    let mut amenity = block(block_id);
    // Bloco cívico no bairro (fase 0 em [18, 64, 2]: (18 ^ 2) & 63 = 16; tempo 84).
    society::tick_block(&mut village, [18, 64, 2], &mut amenity, 84);
    assert!(leisure_near(dimension, [16, 64, 0], 1), "o bloco dá lazer ao bairro");
    let company = balance(COMPANY, &village, [30, 64, 30]);
    society::hall_cycle(&mut village, [60, 64, 0], &mut hall, 1150);
    let done = proposals(&hall).into_iter().find(|proposal| proposal.event == civic.event).unwrap();
    assert!(done.prepared, "obra cívica cumprida");
    assert!(balance(COMPANY, &village, [30, 64, 30]) > company, "a prefeitura paga a obra à companhia");
    society::house_turn(&mut village, [16, 64, 0], &mut house, 1000);
    let homes = society::homes_near(dimension, [16, 64, 0], 1, 2);
    assert!(homes[0].leisure, "a casa ganha lazer");
}

#[test]
fn morale_and_leisure_move_the_human_development_index() {
    let home = |leisure: bool, cheer: i32| society::Home {
        dimension: "d".into(), pos: [0, 64, 0], built: 0, licensed: true, residents: 4, adults: 4, kids: 0,
        employed: 2, income: 160, hungry: false, owed: false, wallet: 0, sanitation: true, leisure, cheer, power: true,
        commerce: true, crowded: false, smog: false, food: 80, housing: 80,
        services: if leisure { 100 } else { 75 }, wealth: 80, seen: 0,
    };
    assert!(home(true, 50).idh() > home(false, 50).idh(), "lazer conta nos serviços");
    assert!(home(true, 100).idh() > home(true, 50).idh(), "ânimo alto melhora o IDH");
    assert!(home(true, 0).idh() < home(true, 50).idh(), "ânimo baixo piora o IDH");
    assert!(home(true, 100).idh() <= 100);
}

#[test]
fn with_no_one_near_the_post_waits_for_the_villager_the_player_clicks() {
    pick::reset();
    let mut village = Village::new("veio:test_pick_hire");
    village.stations.insert(POST, post());
    village.players.push(named_player(village.dimension, 7, "Luiz", [0.5, 64.0, 0.5]));
    village.emeralds = 2;
    // Longe demais para a contratação automática (8), perto o bastante para a escolha (32).
    village.villager(12001, [20.5, 64.0, 0.5], [12, 0, 1, 1]);
    assert_eq!(right_click(&mut village, POST, WAGE), Some(Flow::Cancel));
    assert_eq!(worker(&village.stations[&POST]), None);
    assert_eq!(village.emeralds, 2, "sem contratação, a esmeralda fica");
    assert_eq!(village.notices.last().map(String::as_str), Some("veio.worker.none"));

    // Sem a espera, a esmeralda num aldeão é comércio vanilla; com ela, contrata.
    assert_eq!(pick::interact(&mut village, 12001, Some(WAGE)), Some(Flow::Cancel));
    assert_eq!(worker(&village.stations[&POST]), Some([12, 0, 1, 1]));
    assert_eq!(village.emeralds, 1);
    assert_eq!(village.notices.last().map(String::as_str), Some("veio.worker.hired.hungry"), "posto sem comida: o próximo passo é a comida");
    assert_eq!(pick::interact(&mut village, 12001, Some(WAGE)), None, "a espera acabou: volta a ser comércio");

    // Agachado e de mão vazia, o aldeão diz onde trabalha.
    village.sneak = true;
    set_number(village.stations.get_mut(&POST).unwrap(), "Status", NO_FOOD);
    assert_eq!(pick::interact(&mut village, 12001, None), Some(Flow::Cancel));
    assert_eq!(village.notices.last().map(String::as_str), Some("veio.worker.works.no_food"));
    village.sneak = false;
    assert_eq!(pick::interact(&mut village, 12001, None), None, "sem agachar, abre o comércio");
}

#[test]
fn a_picked_villager_must_be_free_adult_and_near_the_post() {
    pick::reset();
    let mut village = Village::new("veio:test_pick_rules");
    village.stations.insert(POST, post());
    village.players.push(named_player(village.dimension, 8, "Luiz", [0.5, 64.0, 0.5]));
    village.emeralds = 3;
    village.villager(12101, [60.5, 64.0, 0.5], [12, 1, 0, 1]);
    village.villager(12102, [20.5, 64.0, 0.5], [12, 1, 0, 2]);
    village.villagers.get_mut(&12102).unwrap().2 = -24000;
    right_click(&mut village, POST, WAGE);
    assert_eq!(pick::interact(&mut village, 12101, Some(WAGE)), Some(Flow::Cancel));
    assert_eq!(village.notices.last().map(String::as_str), Some("veio.worker.pick.far"));
    right_click(&mut village, POST, WAGE);
    assert_eq!(pick::interact(&mut village, 12102, Some(WAGE)), Some(Flow::Cancel));
    assert_eq!(village.notices.last().map(String::as_str), Some("veio.worker.child"));
    assert_eq!(worker(&village.stations[&POST]), None);
    assert_eq!(village.emeralds, 3, "escolha recusada não cobra");
    village.sneak = true;
    assert_eq!(pick::interact(&mut village, 12101, None), Some(Flow::Cancel));
    assert_eq!(village.notices.last().map(String::as_str), Some("veio.worker.free"));
}

#[test]
fn the_post_panel_says_what_to_do_next() {
    let data = |status: i32, rate: i32| { let mut data = vec![0; 13]; data[0] = status; data[6] = rate; data };
    assert_eq!(next_step_key(FARM_POST, &data(NO_WORKER, 0)), Some("veio.todo.hire"));
    assert_eq!(next_step_key(HAULER_POST, &data(NO_FOOD, 0)), Some("veio.todo.food"));
    assert_eq!(next_step_key(HAULER_POST, &data(IDLE_WORK, 0)), Some("veio.todo.hauler_none"));
    assert_eq!(next_step_key(HAULER_POST, &data(IDLE_WORK, 3)), Some("veio.todo.hauler_idle"));
    assert_eq!(next_step_key(FARM_POST, &data(NO_TASK, 0)), Some("veio.todo.farm_idle"));
    assert_eq!(next_step_key(FOREMAN_DESK, &data(NO_WAGE, 0)), Some("veio.todo.wage"));
    assert_eq!(next_step_key(SHEPHERD_POST, &data(production::NO_ANIMALS, 0)), Some("veio.todo.shepherd_idle"));
    assert_eq!(next_step_key(FARM_POST, &data(1, 0)), None, "trabalhando: fica a dica fixa");
    assert_eq!(next_step_key(SHOP, &data(NO_WORKER, 0)), None, "loja e prefeitura têm painel próprio");
    let lines = panel_lines(FARM_POST, &data(NO_FOOD, 0)).unwrap();
    assert_eq!(lines[2], "veio.todo.food");
}

#[test]
fn a_hauler_carries_the_press_plates_on_to_the_assembler() {
    let mut village = Village::new("veio:test_work_chain");
    village.crops.clear();
    let mut press = block(PRESS);
    press.items[1] = Some(ItemStack::new(PLATE, 6));
    set_number(&mut press, "Status", 1);
    village.stations.insert(PRESS_AT, press);
    let assembler_at = [-3, 64, 0];
    let mut assembler = block(ASSEMBLER);
    set_number(&mut assembler, "Status", 2);
    village.stations.insert(assembler_at, assembler);
    village.villager(12201, [0.5, 64.0, 1.5], [12, 2, 0, 1]);
    let mut hauler = block(HAULER_POST);
    set_number(&mut hauler, "Ration", 20);
    assert!(hire_at(&mut village, &mut hauler, POST, 0));
    // Recolhe as chapas da prensa...
    let now = cycles_at(&mut village, POST, &mut hauler, 1, HAULER_SWEEP * 2, |post| target(post) == Some(PRESS_AT));
    village.walk(12201, [2, 64, 0]);
    cycle_at(&mut village, POST, &mut hauler, now + 1);
    assert_eq!(carried(&hauler), vec![(PLATE.to_owned(), 6)]);
    village.walk(12201, [0, 64, 1]);
    cycle_at(&mut village, POST, &mut hauler, now + 2);
    assert_eq!(hauler.items[4], Some(ItemStack::new(PLATE, 6)));
    // ... e leva ao montador parado por falta delas, sem devolver à prensa.
    let now = cycles_at(&mut village, POST, &mut hauler, now + 3, HAULER_SWEEP, |post| carry_to(post) == Some(assembler_at));
    assert_eq!(carried(&hauler), vec![(PLATE.to_owned(), 6)]);
    village.walk(12201, [-2, 64, 0]);
    cycle_at(&mut village, POST, &mut hauler, now + 1);
    assert_eq!(village.stations[&assembler_at].items[0], Some(ItemStack::new(PLATE, 6)));
    assert!(village.stations[&PRESS_AT].items[0].is_none(), "a chapa não volta para a prensa");
}

#[test]
fn a_starved_machine_names_the_missing_input_of_the_closest_recipe() {
    let mut assembler = block(ASSEMBLER);
    assembler.items[0] = Some(ItemStack::new(PLATE, 1));
    production::process(&mut assembler);
    assert_eq!(number(&assembler, "Status"), 2);
    let recipe = number(&assembler, "RecipeIndex");
    assert!(recipe > 0, "taxas da receita mais próxima no painel");
    assert_eq!(number(&assembler, "Info"), 1, "falta a entrada 1 (só 1 de 2 chapas)");
    assert_eq!(production::missing_input(ASSEMBLER, 2, recipe, 1), Some((PLATE, 2)));
    assert_eq!(production::missing_input(ASSEMBLER, 1, recipe, 1), None, "trabalhando: nada a pedir");
    // Entradas vazias: nada a apontar.
    let mut empty = block(ASSEMBLER);
    production::process(&mut empty);
    assert_eq!(number(&empty, "Info"), 0);
}
