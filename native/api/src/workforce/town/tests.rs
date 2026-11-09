//! Testes da cidade pronta, do piso de população e do casal fundador.
use super::*;
use horizon_client::mod_api::{BlockStateId, EntityRef, EquipmentSlot, Flow, LivingRef, NativeServerPlayer};
use std::collections::HashMap;

const PLAN: &str = include_str!("../../../../../content/data/veio/function/town/build.mcfunction");
const HOUSE: &str = include_str!("../../../../../content/data/veio/function/home/build.mcfunction");

/// Uma linha `fill`/`setblock` da função: caixa relativa (mín, máx) e o bloco.
fn boxes(plan: &str) -> Vec<([i32; 3], [i32; 3], String)> {
    plan.lines().filter(|line| line.starts_with("fill ") || line.starts_with("setblock ")).map(|line| {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let n = |text: &str| text.trim_start_matches('~').parse::<i32>().unwrap_or(0);
        if parts[0] == "setblock" {
            let at = [n(parts[1]), n(parts[2]), n(parts[3])];
            (at, at, parts[4].to_owned())
        } else {
            let a = [n(parts[1]), n(parts[2]), n(parts[3])];
            let b = [n(parts[4]), n(parts[5]), n(parts[6])];
            ([a[0].min(b[0]), a[1].min(b[1]), a[2].min(b[2])], [a[0].max(b[0]), a[1].max(b[1]), a[2].max(b[2])], parts[7].to_owned())
        }
    }).collect()
}

#[test]
fn the_town_is_premade_with_lots_beds_and_a_home_for_everyone() {
    let lines = boxes(PLAN);
    // Sobrescreve tudo: o primeiro passo limpa o volume inteiro da cidade, do chão a 40 acima.
    let cleared: i64 = lines.iter().filter(|(_, _, block)| block == "minecraft:air")
        .map(|(a, b, _)| i64::from(b[0] - a[0] + 1) * i64::from(b[1] - a[1] + 1) * i64::from(b[2] - a[2] + 1)).sum();
    let side = i64::from(2 * HALF + 1);
    assert_eq!(cleared, side * side * 41, "todo o ar da cidade, nenhum galho ou copa sobra");
    for (a, b, block) in &lines {
        let volume = (b[0] - a[0] + 1) * (b[1] - a[1] + 1) * (b[2] - a[2] + 1);
        assert!(volume <= 32768, "fill acima do limite do jogo: {a:?} {b:?} {block}");
        assert!((-HALF..=HALF).contains(&a[0]) && (-HALF..=HALF).contains(&b[2]), "dentro do perímetro");
    }
    assert!(!PLAN.contains("place structure"), "a vila vanilla não é usada");
    assert!(PLAN.contains("minecraft:bell"), "a praça tem o sino, ponto de encontro dos aldeões");
    for station in ["minecraft:composter", "minecraft:smoker", "minecraft:cartography_table"] {
        assert!(PLAN.contains(station), "barraca do mercado com {station}");
    }
    // Camas e moradores.
    assert!(BEDS >= STARTING_VILLAGERS + 6, "camas para os 20 e espaço para crescer");
    assert_eq!(PLAN.matches("_bed[facing=").filter(|_| true).count() / 2, BEDS);
    assert!(SPOTS.len() >= STARTING_VILLAGERS);
    assert!(SPOTS.iter().all(|spot| spot[0].abs() < HALF && spot[2].abs() < HALF));
    // O lote do fundador é do tamanho da casa e fica livre acima do chão (a casa vem depois).
    let lot = data::FOUNDER_LOT;
    assert_eq!(lot[1], -1, "piso da casa no nível do chão");
    let (west, north) = (lot[0], lot[2]);
    let (east, south) = (west + super::super::super::home::FOOTPRINT[0] - 1, north + super::super::super::home::FOOTPRINT[1] - 1);
    for (a, b, block) in &lines {
        let overlaps = a[0] <= east && b[0] >= west && a[2] <= south && b[2] >= north;
        assert!(!(overlaps && b[1] >= 0 && block != "minecraft:air"), "nada construído no lote da casa: {a:?} {block}");
    }
    assert!(south < -8 && west + super::super::super::home::FOOTPRINT[0] / 2 == 0, "ao norte da praça, centrada, de frente para ela");
    assert!(HOUSE.contains("veio:home[facing=south]") && HOUSE.contains("veio:control_center[facing=west]"));
    // A oficina do tutorial cabe no lote do distrito industrial, livre acima do chão por dentro.
    let [wx, _, wz] = data::WORKSHOP;
    for (a, b, block) in &lines {
        let inside = a[0] <= wx + 5 && b[0] >= wx - 5 && a[2] <= wz + 5 && b[2] >= wz - 5;
        assert!(!(inside && b[1] >= 0 && block != "minecraft:air"), "terreno da oficina livre: {a:?} {block}");
    }
    // O centro da praça fica livre (renascimento e prefeito).
    for (a, b, block) in &lines {
        let center = a[0] <= 1 && b[0] >= -1 && a[2] <= 1 && b[2] >= -1;
        assert!(!(center && b[1] >= 0 && block != "minecraft:air"), "centro da praça livre: {a:?} {block}");
    }
}

#[test]
fn the_floor_is_two_and_lost_towns_get_families() {
    use Arrival::{Couple, Family};
    let nothing = Arrival::None;
    assert_eq!(arrival(0, 20, None, 20, None, false), Couple, "zerou: casal na hora, de noite também");
    assert_eq!(arrival(1, 20, Some(0), 20, Some(10), false), Couple, "um só não tem filhos");
    assert_eq!(arrival(2, 20, Some(30), 20, None, true), Family, "abaixo da metade do recorde: família");
    assert_eq!(arrival(2, 20, Some(30), 20, None, false), nothing, "imigrante chega de dia");
    assert_eq!(arrival(2, 20, Some(30), 20, Some(FAMILY_EVERY - 1), true), nothing, "uma família por dia");
    assert_eq!(arrival(2, 20, Some(4), 20, None, true), nothing, "sem cama, ninguém se muda");
    assert_eq!(arrival(9, 20, Some(30), 10, None, true), nothing, "sem teto, ninguém se muda");
    assert_eq!(arrival(10, 20, Some(30), 20, None, true), nothing, "metade do recorde não é queda");
    assert_eq!(arrival(3, 6, Some(30), 20, None, true), nothing, "cidade que nunca cresceu não caiu");
}

#[test]
fn the_town_remembers_its_center_and_record() {
    let text = encode([120, 64, -300], 87);
    assert_eq!(decode(&text), Some(([120, 64, -300], 87)));
    assert_eq!(decode("peak 4\n"), None, "sem centro, sem cidade");
}

#[test]
fn command_uuids_use_the_hyphenated_form() {
    assert_eq!(founders::uuid_text([0x0102_0304, 0x0506_0708, -1, 0x0a0b_0c0d]),
        "01020304-0506-0708-ffff-ffff0a0b0c0d");
}

#[test]
fn founders_need_care_only_when_hurt_or_unarmed() {
    use founders::{care, Care, DOWN_AT, RECOVER};
    assert_eq!(care(Some(20.0), None, true, 0), Care::Nothing);
    assert_eq!(care(Some(20.0), None, false, 0), Care::Arm, "sem totem: arma");
    assert_eq!(care(Some(DOWN_AT), None, true, 0), Care::KnockDown);
    assert_eq!(care(Some(1.0), None, true, 0), Care::KnockDown, "o totem segurou o golpe fatal");
    assert_eq!(care(None, Some(100), true, 100 + RECOVER - 1), Care::Nothing, "ainda desacordado");
    assert_eq!(care(None, Some(100), true, 100 + RECOVER), Care::WakeUp);
}

// ---------- Um mundo para a cidade ----------

#[derive(Default)]
struct World {
    mobs: HashMap<u32, ([f64; 3], Compound)>,
    hands: HashMap<u32, Option<String>>,
    commands: Vec<String>,
    told: Vec<String>,
    players: Vec<NativeServerPlayer>,
    bag: HashMap<String, u32>,
    next: u32,
    day: i64,
}

impl World {
    fn villager(&mut self, id: u32, at: [f64; 3], nbt: Compound) {
        let mut nbt = nbt;
        nbt.entry("UUID".into()).or_insert(Tag::IntArray(vec![1, 2, 3, id as i32]));
        nbt.entry("Health".into()).or_insert(Tag::Float(20f32.to_bits()));
        self.mobs.insert(id, (at, nbt));
        production::ranch::observe(OVERWORLD, id, VILLAGER);
    }
    fn life(&self, id: u32) -> f32 {
        match self.mobs[&id].1.get(&"Health".into()) { Some(Tag::Float(bits)) => f32::from_bits(*bits), _ => -1.0 }
    }
    fn flag(&self, id: u32, key: &str) -> Option<i8> {
        match self.mobs[&id].1.get(&key.into()) { Some(Tag::Byte(value)) => Some(*value), _ => None }
    }
}

impl Host for World {
    fn dimension(&self) -> &'static str { OVERWORLD }
    fn world_dir(&self) -> Option<std::path::PathBuf> { None }
    fn run_command(&mut self, line: &str) -> bool { self.commands.push(line.to_owned()); true }
    fn server_players(&self) -> Vec<NativeServerPlayer> { self.players.clone() }
    fn acting_player(&self) -> Option<NativeServerPlayer> { self.players.first().cloned() }
    fn tell(&mut self, _: u128, line: Text) -> bool { self.told.push(line.plain()); true }
    fn day_time(&self) -> i64 { self.day }
    fn entity(&self, id: u32) -> Option<EntityRef> { self.mobs.contains_key(&id).then(|| EntityRef::new(id)) }
    fn entity_position(&self, entity: EntityRef) -> Option<[f64; 3]> { self.mobs.get(&entity.id()).map(|(at, _)| *at) }
    fn entity_nbt(&mut self, entity: EntityRef) -> Option<Compound> { self.mobs.get(&entity.id()).map(|(_, nbt)| nbt.clone()) }
    fn entity_merge_nbt(&mut self, entity: EntityRef, nbt: Compound) -> bool {
        let Some((_, data)) = self.mobs.get_mut(&entity.id()) else { return false };
        data.extend(nbt);
        true
    }
    fn set_mob_health(&mut self, _: &str, id: u32, health: f32) -> bool {
        let Some((_, data)) = self.mobs.get_mut(&id) else { return false };
        data.insert("Health".into(), Tag::Float(health.to_bits()));
        true
    }
    fn living(&self, entity: EntityRef) -> Option<LivingRef> { self.mobs.contains_key(&entity.id()).then(|| LivingRef::new(entity)) }
    fn living_set_equipment(&mut self, living: LivingRef, slot: EquipmentSlot, stack: Option<ItemStack>) -> bool {
        assert_eq!(slot, EquipmentSlot::OffHand, "o totem vai na mão secundária");
        self.hands.insert(living.id(), stack.map(|stack| stack.item.to_string()));
        true
    }
    fn summon(&mut self, _: &str, _: &str, at: [f64; 3], nbt: Compound) -> Option<u32> {
        self.next += 1;
        let id = 70_000 + self.next;
        self.villager(id, at, nbt);
        Some(id)
    }
    fn take_items(&mut self, item: &str, count: u32) -> u32 {
        let have = self.bag.entry(item.to_owned()).or_default();
        let taken = count.min(*have);
        *have -= taken;
        taken
    }
    fn send_particles(&mut self, _: &ParticleOptions, _: Option<BlockStateId>, _: [f64; 3], _: [f64; 3], _: f64, _: i32) -> bool { true }
    fn surface_heights(&self, _: &str, columns: &[(i32, i32)], _: horizon_client::mod_api::NativeHeightmap) -> Result<Vec<Option<i32>>, String> {
        Ok(columns.iter().map(|_| Some(70)).collect())
    }
    fn block(&self, _: [i32; 3]) -> String { "minecraft:air".into() }
    fn block_property(&self, _: [i32; 3], _: &str) -> Option<String> { None }
}

fn player(at: [f64; 3]) -> NativeServerPlayer {
    NativeServerPlayer { uuid: 5, name: "Mayor".into(), dimension: OVERWORLD.to_owned(), position: at, ..Default::default() }
}

/// Primeiro tick em que o aldeão `id` é conferido (a fase de cada um vem do id).
fn turn(id: u32, from: u64, every: u64) -> u64 {
    (from..from + every).find(|now| (now + u64::from(id)).is_multiple_of(every)).unwrap()
}

#[test]
fn the_town_is_born_with_twenty_people_and_the_founding_couple() {
    reset();
    founders::reset();
    let mut world = World::default();
    let site = [40_000, 70, 40_000];
    assert!(build(&mut world, site, 0));
    assert!(world.commands[0].ends_with("positioned 40000 70 40000 run function veio:town/build"));
    assert_eq!(world.mobs.len(), STARTING_VILLAGERS);
    let kids = world.mobs.values().filter(|(_, nbt)| matches!(nbt.get(&"Age".into()), Some(Tag::Int(age)) if *age < 0)).count();
    assert_eq!(kids, 4, "quatro crianças entre os moradores");
    let tagged: Vec<u32> = world.mobs.iter().filter(|(_, (_, nbt))| founders::tagged(nbt)).map(|(id, _)| *id).collect();
    assert_eq!(tagged.len(), 2);
    assert_eq!(founders::count(), 2);
    assert!(tagged.iter().all(|id| world.hands[id].as_deref() == Some(founders::TOTEM)), "cada fundador com seu totem");
    assert_eq!(founder_lot(), Some([40_000 - 7, 69, 40_000 - 23]));
    assert_eq!(workshop_lot(), Some([40_000 + data::WORKSHOP[0], 70, 40_000 + data::WORKSHOP[2]]));
    assert_eq!(spawn::center(), Some((site, false)), "a praça vale como centro mesmo depois de reabrir");
    // Os moradores nascem dentro das casas, perto das camas.
    for (at, _) in world.mobs.values() {
        assert!((at[0] - 40_000.0).abs() < f64::from(HALF) && (at[2] - 40_000.0).abs() < f64::from(HALF));
    }
    spawn::reset();
    reset();
    founders::reset();
}

#[test]
fn founders_fall_wounded_go_to_the_refuge_and_wake_with_food_or_time() {
    reset();
    founders::reset();
    let mut world = World::default();
    world.players = vec![player([50_000.0, 70.0, 50_000.0])];
    assert!(build(&mut world, [50_000, 70, 50_000], 0));
    let id = *world.mobs.iter().find(|(_, (_, nbt))| founders::tagged(nbt)).map(|(id, _)| id).unwrap();
    // Machucado, mas não o bastante.
    world.set_mob_health(OVERWORLD, id, 12.0);
    let now = turn(id, 100, founders::CHECK);
    founders::observe(&mut world, OVERWORLD, id, now);
    assert!(!founders::is_down(id));
    // Golpe forte: cai ferido, sem IA e invulnerável, e é levado ao lote da prefeitura.
    world.set_mob_health(OVERWORLD, id, 4.0);
    let now = turn(id, now + 1, founders::CHECK);
    founders::observe(&mut world, OVERWORLD, id, now);
    assert!(founders::is_down(id));
    assert_eq!((world.flag(id, "NoAI"), world.flag(id, "Invulnerable")), (Some(1), Some(1)));
    assert!(world.life(id) > 0.0);
    let [rx, ry, rz] = refuge().unwrap();
    assert!(world.commands.iter().any(|line| line.ends_with(&format!("tp 00000001-0000-0002-0000-0003{:08x} {rx}.5 {ry} {rz}.5", id))),
        "{:?}", world.commands);
    assert!(world.told.iter().any(|line| line.contains("veio.founder.down")));
    // Clique sem comida: não acorda, explica; o clique não vira troca nem contratação.
    assert_eq!(founders::interact(&mut world, id, Some("minecraft:stick")), Some(Flow::Cancel));
    assert!(founders::is_down(id));
    assert!(world.told.iter().any(|line| line.contains("veio.founder.resting")));
    // Com pão na mão (e no inventário), acorda na hora com a vida cheia e outro totem.
    world.bag.insert("minecraft:bread".into(), 1);
    world.hands.clear();
    assert_eq!(founders::interact(&mut world, id, Some("minecraft:bread")), Some(Flow::Cancel));
    assert!(!founders::is_down(id));
    assert_eq!(world.bag["minecraft:bread"], 0, "a comida é gasta");
    assert_eq!((world.flag(id, "NoAI"), world.flag(id, "Invulnerable")), (Some(0), Some(0)));
    assert_eq!(world.life(id), founders::HEALTH);
    assert_eq!(world.hands[&id].as_deref(), Some(founders::TOTEM));
    // De pé, o clique segue normal (contratar, trocar).
    assert_eq!(founders::interact(&mut world, id, Some("minecraft:bread")), None);
    // Sem ninguém por perto, acorda sozinho depois de um tempo.
    world.set_mob_health(OVERWORLD, id, 2.0);
    let down = turn(id, now + 1, founders::CHECK);
    founders::observe(&mut world, OVERWORLD, id, down);
    assert!(founders::is_down(id));
    founders::observe(&mut world, OVERWORLD, id, turn(id, down + 1, founders::CHECK));
    assert!(founders::is_down(id), "ainda se recuperando");
    founders::observe(&mut world, OVERWORLD, id, turn(id, down + founders::RECOVER, founders::CHECK));
    assert!(!founders::is_down(id));
    assert!(world.told.iter().any(|line| line.contains("veio.founder.up")));
    spawn::reset();
    reset();
    founders::reset();
}

#[test]
fn founders_are_recognized_after_reopening_the_world() {
    founders::reset();
    let mut world = World::default();
    let tags = Tag::List(vec![Tag::String(founders::TAG.into())]);
    world.villager(81_001, [0.5, 70.0, 0.5], Compound::from([("Tags".into(), tags.clone()), ("NoAI".into(), Tag::Byte(1))]));
    world.villager(81_002, [1.5, 70.0, 0.5], Compound::new());
    world.villager(81_003, [2.5, 70.0, 0.5], Compound::from([("Tags".into(), tags)]));
    // Sem a cidade conhecida, ninguém é lido.
    for id in [81_001, 81_002, 81_003] {
        founders::observe(&mut world, OVERWORLD, id, turn(id, 0, founders::SCAN));
    }
    assert_eq!(founders::count(), 0);
    founders::start_scan(0);
    for id in [81_001, 81_002, 81_003] {
        founders::observe(&mut world, OVERWORLD, id, turn(id, 0, founders::SCAN));
    }
    assert_eq!(founders::count(), 2, "os dois marcados, e só eles");
    assert!(founders::is_down(81_001), "quem estava desacordado volta como ferido");
    assert!(!founders::is_down(81_003));
    assert_eq!(world.hands.get(&81_003).cloned().flatten().as_deref(), Some(founders::TOTEM));
    founders::reset();
}

#[test]
fn an_empty_town_gets_a_couple_and_a_fallen_one_gets_families() {
    reset();
    founders::reset();
    let mut world = World::default();
    let site = [60_000, 70, 60_000];
    assert!(build(&mut world, site, 0));
    world.players = vec![player([60_000.5, 70.0, 60_010.5])];
    world.day = 1000;
    // Uma invasão levou todo mundo (os fundadores também: o totem falhou).
    for id in world.mobs.keys().copied().collect::<Vec<_>>() {
        world.mobs.remove(&id);
    }
    founders::reset();
    world.next = 100;
    tick(&mut world, CHECK_EVERY);
    assert_eq!(world.mobs.len(), 2, "o piso: um casal chega na hora");
    assert_eq!(founders::count(), 2, "e herda o papel de fundador");
    assert!(world.told.iter().any(|line| line.contains("veio.town.refugees")));
    assert!(recovering());
    // Uma cidade de 5 (recorde 20): uma família por dia, de dia.
    for index in 0..3 {
        world.villager(90_000 + index, [60_000.5 + f64::from(index), 70.0, 60_005.5], Compound::new());
    }
    world.told.clear();
    tick(&mut world, 2 * CHECK_EVERY);
    assert_eq!(world.mobs.len(), 8, "dois adultos e uma criança");
    assert!(world.told.iter().any(|line| line.contains("veio.town.immigrants")));
    tick(&mut world, 3 * CHECK_EVERY);
    assert_eq!(world.mobs.len(), 8, "a próxima família só amanhã");
    assert_eq!(peak(), STARTING_VILLAGERS);
    spawn::reset();
    reset();
    founders::reset();
}
