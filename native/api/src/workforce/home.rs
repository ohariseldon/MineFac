//! Lar: a casa do jogador dentro da vila operária.
//!
//! O bloco do lar fica dentro da casa que o jogador constrói. Quem estiver perto quando ele
//! começa a funcionar vira o dono: renasce ali (o centro da vila continua sendo o spawn de quem
//! não tem casa). O lar dá o que fazer em casa, todos os dias:
//!
//! - **Lareira** (slot 0): carvão, carvão vegetal, biomassa, troncos ou tábuas. Acesa, aquece a
//!   casa, solta fumaça e, a partir do nível 2, cozinha a carne, o peixe e a batata crus da despensa.
//! - **Despensa** (slots 1-3): comida. Com uma **tigela** na mão, clique no lar: **refeição em
//!   casa**. Enche a fome e, com a lareira acesa, dá efeitos que crescem com o valor da comida, o
//!   **cardápio variado** (pratos diferentes nas últimas 5 refeições) e o **conforto**. Uma
//!   refeição a cada 2 minutos.
//! - **Conforto** (0-100): o lar lê a casa em volta (9x9, de um bloco abaixo a três acima) aos
//!   poucos e soma cama, porta, janela, luz, baú, fogão, bancada, decoração e acabamento.
//! - **Descanso:** quem passa a noite em casa com a lareira acesa acorda **descansado**.
//! - **Melhorias** (slot 4): materiais da fábrica sobem o nível da casa.
//!   2. Casa de tijolos (24 tijolos, 12 vidros, 8 chapas de cobre): a lareira cozinha; efeitos +50%.
//!   3. Casa elétrica (8 fiações, 2 circuitos, 8 chapas de ferro): **escritório** — o painel mostra a
//!      vila e a bússola no lar dá o foco das pendências, como no centro de controle.
//!   4. Casa de engenheiro (4 chapas de aço, 4 engrenagens, 1 módulo): efeitos mais fortes (pressa II).
//! - **Coisas a fazer:** o painel lista o dia (comer em casa, lareira acesa, despensa abastecida),
//!   o que falta no conforto e na melhoria e, com escritório, as pendências da vila. Um dia em
//!   ordem rende experiência e conta a sequência de dias.
//!
//! Efeitos e renascer usam `Host::run_command` (console do servidor), como o spawn na vila.
use super::*;
use std::sync::atomic::{AtomicBool, Ordering};

pub(crate) const HOME: &str = "veio:home";
pub(crate) const FUEL: usize = 0;
pub(crate) const PANTRY: std::ops::Range<usize> = 1..4;
pub(crate) const BUILD: usize = 4;
pub(crate) const BOWL: &str = "minecraft:bowl";
pub(crate) const CAKE: &str = "minecraft:cake";
/// Convidados de um banquete, quanto tempo ficam e de onde vêm.
pub(crate) const GUESTS: usize = 6;
pub(crate) const BANQUET_TICKS: u64 = 1200;
const INVITE_RADIUS: f64 = 32.0;
/// Lugares em volta do lar onde os convidados ficam.
const SEATS: [[f64; 2]; 6] = [[2.0, 0.0], [-2.0, 0.0], [0.0, 2.0], [0.0, -2.0], [2.0, 2.0], [-2.0, -2.0]];
/// Lado da casa lida: 9x9.
pub(crate) const RADIUS: i32 = 4;
/// Camadas: uma abaixo do lar até três acima.
const LAYERS: [i32; 5] = [0, -1, 1, 2, 3];
/// Blocos lidos por ciclo (10 ticks): a casa inteira em 15 ciclos.
const PER_CYCLE: i32 = 27;
/// Ticks do relógio entre duas refeições.
pub(crate) const MEAL_COOLDOWN: u64 = 2400;
/// Cozinha um item cru a cada `COOK_EVERY` ticks com a lareira acesa (nível 2+).
const COOK_EVERY: i32 = 400;
/// Refeições lembradas para o cardápio variado.
const MENU: usize = 5;
pub(crate) const MAX_LEVEL: i32 = 4;

/// Pontos de cada categoria do conforto (soma 100).
pub(crate) const CATEGORIES: [(&str, i32); 9] = [
    ("bed", 15), ("door", 10), ("window", 10), ("light", 10), ("storage", 10),
    ("stove", 10), ("workshop", 10), ("decor", 10), ("finish", 15),
];
/// Blocos de acabamento para os 15 pontos inteiros.
const FINISH_FULL: i32 = 24;

const fn m(item: &'static str, count: u32, name: &'static str) -> projects::Material {
    projects::Material { item, count, name }
}

/// Materiais para chegar a cada nível (2, 3 e 4).
pub(crate) const LEVELS: [[projects::Material; 3]; 3] = [
    [m("minecraft:brick", 24, "item.minecraft.brick"), m("minecraft:glass", 12, "block.minecraft.glass"),
        m("veio:copper_plate", 8, "item.veio.copper_plate")],
    [m("veio:wire", 8, "item.veio.wire"), m("veio:circuit", 2, "item.veio.circuit"),
        m("veio:iron_plate", 8, "item.veio.iron_plate")],
    [m("veio:steel_plate", 4, "item.veio.steel_plate"), m("veio:gear", 4, "item.veio.gear"),
        m("veio:module", 1, "item.veio.module")],
];

pub(crate) fn is_home(block: &str) -> bool {
    block == HOME
}

/// Ticks de fogo por item na lareira.
pub(crate) fn burn_time(item: &str) -> Option<i32> {
    Some(match item {
        "minecraft:coal" | "minecraft:charcoal" => 4800,
        "minecraft:coal_block" => 48_000,
        "veio:biomass" => 2400,
        "minecraft:dried_kelp_block" => 8000,
        _ if item.ends_with("_log") || item.ends_with("_wood") || item.ends_with("_stem") || item.ends_with("_hyphae") => 1600,
        _ if item.ends_with("_planks") => 600,
        _ => return None,
    })
}

/// Comida crua que a lareira cozinha (nível 2+).
pub(crate) fn cooked(item: &str) -> Option<&'static str> {
    Some(match item {
        "minecraft:beef" => "minecraft:cooked_beef",
        "minecraft:porkchop" => "minecraft:cooked_porkchop",
        "minecraft:chicken" => "minecraft:cooked_chicken",
        "minecraft:mutton" => "minecraft:cooked_mutton",
        "minecraft:rabbit" => "minecraft:cooked_rabbit",
        "minecraft:cod" => "minecraft:cooked_cod",
        "minecraft:salmon" => "minecraft:cooked_salmon",
        "minecraft:potato" => "minecraft:baked_potato",
        "minecraft:kelp" => "minecraft:dried_kelp",
        _ => return None,
    })
}

fn level_material(item: &str) -> bool {
    LEVELS.iter().flatten().any(|need| need.item == item)
}

pub(crate) fn accepts(slot: usize, stack: &ItemStack) -> bool {
    plain(stack) && match slot {
        FUEL => burn_time(&stack.item).is_some(),
        1..=3 => is_food(&stack.item) || cooked(&stack.item).is_some(),
        BUILD => level_material(&stack.item),
        _ => false,
    }
}

/// Categoria de conforto de um bloco da casa.
pub(crate) fn category(block: &str) -> Option<usize> {
    let name = block.strip_prefix("minecraft:").unwrap_or(block);
    Some(if name.ends_with("_bed") {
        0
    } else if name.ends_with("_door") {
        1
    } else if name.contains("glass") {
        2
    } else if name.contains("torch") || name.contains("lantern") || name.contains("candle") || name.contains("froglight")
        || matches!(name, "glowstone" | "shroomlight" | "end_rod" | "redstone_lamp" | "veio:grow_lamp") {
        3
    } else if name.ends_with("chest") || name == "barrel" || name.ends_with("shulker_box") || storage::is_store(block) {
        4
    } else if name.contains("furnace") || matches!(name, "smoker" | "campfire" | "soul_campfire" | "veio:kitchen") {
        5
    } else if matches!(name, "crafting_table" | "smithing_table" | "anvil" | "chipped_anvil" | "damaged_anvil" | "loom"
        | "cartography_table" | "stonecutter" | "grindstone" | "fletching_table" | "crafter") {
        6
    } else if name.starts_with("potted_") || name.ends_with("_carpet") || name.contains("bookshelf") || name.ends_with("_banner")
        || matches!(name, "flower_pot" | "lectern" | "jukebox" | "decorated_pot" | "note_block" | "bell") {
        7
    } else if name.ends_with("_planks") || name.contains("bricks") || name.starts_with("polished_") || name.contains("tiles")
        || name.ends_with("terracotta") || matches!(name, "smooth_stone" | "quartz_block" | "chiseled_stone_bricks") {
        8
    } else {
        return None;
    })
}

/// Pontos de conforto: categorias encontradas, quantos enfeites e quantos blocos de acabamento.
pub(crate) fn comfort(have: i32, decor: i32, finish: i32) -> i32 {
    CATEGORIES.iter().enumerate().map(|(index, (_, points))| match index {
        7 => [0, 4, 7, 10][decor.clamp(0, 3) as usize],
        8 => finish.min(FINISH_FULL) * points / FINISH_FULL,
        _ if have & (1 << index) != 0 => *points,
        _ => 0,
    }).sum()
}

fn scan_position(pos: [i32; 3], index: i32) -> [i32; 3] {
    let side = RADIUS * 2 + 1;
    let layer = side * side;
    let cell = index % layer;
    [pos[0] + cell % side - RADIUS, pos[1] + LAYERS[(index / layer) as usize % LAYERS.len()], pos[2] + cell / side - RADIUS]
}

/// Lê `PER_CYCLE` blocos da casa; ao fim da volta, publica o conforto.
fn scan(host: &dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity) {
    if !budget::request(budget::Lane::Workforce, host.dimension(), pos, PER_CYCLE as u32) {
        return;
    }
    let side = RADIUS * 2 + 1;
    let area = side * side * LAYERS.len() as i32;
    let mut cursor = number(entity, "CScan") % area;
    let (mut have, mut decor, mut finish) = (number(entity, "CHave"), number(entity, "CDecor"), number(entity, "CFinish"));
    for _ in 0..PER_CYCLE {
        let spot = scan_position(pos, cursor);
        if spot != pos && let Some(found) = category(&host.block(spot)) {
            have |= 1 << found;
            match found {
                7 => decor += 1,
                8 => finish += 1,
                _ => {}
            }
        }
        cursor += 1;
        if cursor >= area {
            cursor = 0;
            set_number(entity, "Have", have);
            set_number(entity, "Decor", decor);
            set_number(entity, "Finish", finish);
            set_number(entity, "Comfort", comfort(have, decor, finish));
            (have, decor, finish) = (0, 0, 0);
        }
    }
    set_number(entity, "CScan", cursor);
    set_number(entity, "CHave", have);
    set_number(entity, "CDecor", decor);
    set_number(entity, "CFinish", finish);
}

pub(crate) fn level(entity: &ModBlockEntity) -> i32 {
    number(entity, "Level").clamp(1, MAX_LEVEL)
}

fn up_key(index: usize) -> &'static str {
    ["Up0", "Up1", "Up2"][index]
}

/// Primeiro material que falta para o próximo nível: (índice, quantidade).
pub(crate) fn missing(entity: &ModBlockEntity) -> Option<(usize, u32)> {
    let next = LEVELS.get((level(entity) - 1) as usize)?;
    next.iter().enumerate().map(|(index, need)| (index, need.count.saturating_sub(number(entity, up_key(index)) as u32)))
        .find(|(_, left)| *left > 0)
}

fn upgrade_percent(entity: &ModBlockEntity) -> i32 {
    let Some(next) = LEVELS.get((level(entity) - 1) as usize) else { return 100 };
    let (have, total) = next.iter().enumerate().fold((0, 0), |(have, total), (index, need)| {
        (have + (number(entity, up_key(index)) as u32).min(need.count), total + need.count)
    });
    (have * 100 / total.max(1)) as i32
}

/// Leva os materiais do slot de obra para a melhoria. `true` se a casa subiu de nível.
fn build(entity: &mut ModBlockEntity) -> bool {
    let Some(next) = LEVELS.get((level(entity) - 1) as usize) else { return false };
    let Some(stack) = entity.items.get(BUILD).and_then(Option::as_ref).filter(|stack| plain(stack)).cloned() else { return false };
    let Some((index, need)) = next.iter().enumerate().find(|(_, need)| need.item == stack.item.as_ref()) else { return false };
    let have = number(entity, up_key(index)) as u32;
    let moved = need.count.saturating_sub(have).min(stack.count);
    if moved > 0 {
        set_number(entity, up_key(index), (have + moved) as i32);
        consume(&mut entity.items[BUILD], moved);
    }
    if missing(entity).is_some() {
        return false;
    }
    set_number(entity, "Level", level(entity) + 1);
    for index in 0..3 {
        set_number(entity, up_key(index), 0);
    }
    true
}

fn text(entity: &ModBlockEntity, key: &str) -> String {
    match entity.nbt.get(nbt_key(key)) {
        Some(Tag::String(value)) => value.to_string_lossy(),
        _ => String::new(),
    }
}

fn set_text(entity: &mut ModBlockEntity, key: &str, value: &str) {
    entity.nbt.insert(nbt_key(key).clone(), Tag::String(value.into()));
}

fn pantry_count(entity: &ModBlockEntity) -> u32 {
    PANTRY.filter_map(|slot| entity.items.get(slot).and_then(Option::as_ref))
        .filter(|stack| is_food(&stack.item)).map(|stack| stack.count).sum()
}

fn menu(entity: &ModBlockEntity) -> Vec<String> {
    text(entity, "Menu").split(',').filter(|item| !item.is_empty()).map(str::to_owned).collect()
}

pub(crate) fn variety(entity: &ModBlockEntity) -> i32 {
    let menu = menu(entity);
    menu.iter().enumerate().filter(|(index, item)| !menu[..*index].contains(item)).count() as i32
}

// ---------- Registro dos lares ----------

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Home {
    pub dimension: String,
    pub pos: [i32; 3],
    pub owner: String,
    pub seen: u64,
}

static HOMES: Mutex<Vec<Home>> = Mutex::new(Vec::new());
static HOMES_PENDING: AtomicBool = AtomicBool::new(false);

fn register(home: Home) {
    if let Ok(mut homes) = HOMES.lock() {
        // Um lar por jogador: o mais recente vale.
        homes.retain(|known| known.owner != home.owner && !(known.dimension == home.dimension && known.pos == home.pos));
        homes.push(home);
        HOMES_PENDING.store(true, Ordering::Release);
    }
}

/// Lar do jogador, se ele tem um que publicou há pouco (ou desde que o mundo abriu).
pub(crate) fn home_of(name: &str) -> Option<Home> {
    if !HOMES_PENDING.load(Ordering::Acquire) {
        return None;
    }
    HOMES.lock().ok()?.iter().find(|home| home.owner == name).cloned()
}

// ---------- Tick ----------

/// Fase de 10 ticks, espalhada pela posição.
fn phase(pos: [i32; 3], time: i64) -> bool {
    (time + i64::from((pos[0] ^ pos[2]) & 7)).rem_euclid(10) == 0
}

pub(crate) fn tick(host: &mut dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity, time: i64, now: u64) {
    if !phase(pos, time) {
        return;
    }
    let dimension = host.dimension();
    let day = day_counter(host, entity);
    claim_owner(host, pos, entity);
    let owner = text(entity, "Owner");
    if !owner.is_empty() {
        register(Home { dimension: dimension.to_owned(), pos, owner: owner.clone(), seen: now });
    }
    // Lareira.
    let mut burn = number(entity, "Burn") - 10;
    if burn <= 0 {
        burn = 0;
        if let Some(stack) = entity.items.get(FUEL).and_then(Option::as_ref).filter(|stack| plain(stack))
            && let Some(time) = burn_time(&stack.item)
        {
            consume(&mut entity.items[FUEL], 1);
            burn = time;
            set_number(entity, "BurnTotal", time);
        }
    }
    set_number(entity, "Burn", burn);
    let lit = burn > 0;
    set_number(entity, "Status", if lit { 1 } else { 6 });
    let hour = host.day_time().rem_euclid(24000);
    if lit {
        if hour >= 12000 {
            set_number(entity, "Warm", 1);
        }
        if (time / 10 + i64::from(pos[1])) % 4 == 0 {
            let at = center(pos);
            host.send_particles(&ParticleOptions::simple(ParticleType::Flame), None, [at[0], at[1] + 0.6, at[2]], [0.15, 0.05, 0.15], 0.0, 2);
            host.send_particles(&ParticleOptions::simple(ParticleType::Smoke), None, [at[0], at[1] + 0.9, at[2]], [0.1, 0.2, 0.1], 0.01, 3);
        }
        if level(entity) >= 2 {
            let timer = number(entity, "Cook") + 10;
            if timer >= COOK_EVERY && cook(entity) {
                set_number(entity, "Cook", 0);
            } else {
                set_number(entity, "Cook", timer.min(COOK_EVERY));
            }
        }
    }
    // Casa: conforto e melhoria.
    scan(host, pos, entity);
    if build(entity) {
        let at = center(pos);
        host.play_sound("minecraft:entity.player.levelup", at, 4, 0.7, 1.0);
        host.send_particles(&ParticleOptions::simple(ParticleType::HappyVillager), None, [at[0], at[1] + 1.0, at[2]], [1.0, 0.8, 1.0], 0.0, 20);
        if let Some(player) = center::nearest_player(host, pos, 16.0) {
            let level = level(entity);
            host.tell(player, Text::tr("veio.home.level_up", &[&level, &Text::tr(&format!("veio.home.level.{level}"), &[]).plain()]));
        }
    }
    // Manhã: quem passou a noite em casa com a lareira acesa acorda descansado.
    if number(entity, "SeenDay") != day + 1 {
        let rested = number(entity, "Warm") == 1 && entity.nbt.contains_key(&"SeenDay".into());
        set_number(entity, "SeenDay", day + 1);
        set_number(entity, "Warm", 0);
        if rested {
            rest(host, pos, entity, &owner, day);
        }
    }
    // Dia em ordem: refeição em casa, lareira acesa e despensa abastecida.
    if number(entity, "RewardDay") != day + 1 && number(entity, "MealDay") == day + 1 && lit && pantry_count(entity) >= 3 {
        let streak = if number(entity, "RewardDay") == day { number(entity, "Streak") + 1 } else { 1 };
        set_number(entity, "Streak", streak);
        set_number(entity, "RewardDay", day + 1);
        if spawn::safe_name(&owner) {
            host.run_command(&format!("xp add {owner} {} points", 10 + 5 * streak.min(6)));
        }
        if let Some(player) = center::nearest_player(host, pos, 16.0) {
            host.tell(player, Text::tr("veio.home.day_done_chat", &[&streak]));
        }
        // Uma casa em ordem também conta para a vila.
        if let Some(village) = projects::nearest(dimension, pos, center::RADIUS * 2, now) {
            center::add_prestige(host, village.pos, 2);
        }
    }
    banquet_upkeep(host, pos, now);
    // Escritório (nível 3): resumo da vila mais próxima.
    let office = if level(entity) >= 3 {
        projects::nearest(dimension, pos, center::RADIUS * 2, now).map_or(-1, |village| {
            (village.requests.clamp(0, 7) << 24) | (village.waiting.clamp(0, 255) << 16)
                | (village.villagers.clamp(0, 255) << 8) | village.cap.clamp(0, 255)
        })
    } else { -1 };
    entity.nbt.insert(nbt_key("Office").clone(), Tag::Int(office));
    let todo = todo(entity, day, lit, office);
    set_number(entity, "Todo", todo);
    let cooldown = number(entity, "MealAt") as u64;
    let wait = if cooldown > now && cooldown - now <= MEAL_COOLDOWN { (cooldown - now) / 20 } else { 0 };
    set_number(entity, "MealWait", wait as i32);
}

/// O primeiro jogador que fica perto do lar é o dono; renasce ali.
fn claim_owner(host: &mut dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity) {
    if !text(entity, "Owner").is_empty() {
        return;
    }
    let Some(player) = center::nearest_server_player(host, pos, 8.0) else { return };
    if !spawn::safe_name(&player.name) {
        return;
    }
    set_text(entity, "Owner", &player.name);
    set_number(entity, "Level", 1);
    settle(host, pos, &player.name);
    host.tell(player.uuid, Text::tr("veio.home.claimed", &[]));
}

/// Renascer no lar.
fn settle(host: &mut dyn Host, [x, y, z]: [i32; 3], name: &str) {
    let dimension = host.dimension();
    host.run_command(&format!("execute in {dimension} run spawnpoint {name} {x} {} {z}", y + 1));
}

/// Cozinha um item cru da despensa. `true` se cozinhou.
fn cook(entity: &mut ModBlockEntity) -> bool {
    for slot in PANTRY {
        let Some(stack) = entity.items.get(slot).and_then(Option::as_ref).filter(|stack| plain(stack)) else { continue };
        let Some(result) = cooked(&stack.item) else { continue };
        if stack.count == 1 {
            entity.items[slot] = Some(ItemStack::new(result, 1));
            return true;
        }
        let Some(target) = (PANTRY.start..PANTRY.end).find(|other| match entity.items[*other].as_ref() {
            None => true,
            Some(present) => is_plain(present, result) && present.count < 64,
        }) else { continue };
        consume(&mut entity.items[slot], 1);
        add(&mut entity.items[target], result, 1);
        return true;
    }
    false
}

/// O que falta fazer, em bits: 0 lareira apagada, 1 despensa vazia, 2 refeição de hoje,
/// 3 conforto incompleto, 4 melhoria disponível, 5 pendências na vila (escritório).
pub(crate) fn todo(entity: &ModBlockEntity, day: i32, lit: bool, office: i32) -> i32 {
    let mut bits = 0;
    if !lit {
        bits |= 1;
    }
    if pantry_count(entity) < 3 {
        bits |= 2;
    }
    if number(entity, "MealDay") != day + 1 {
        bits |= 4;
    }
    if number(entity, "Comfort") < 100 {
        bits |= 8;
    }
    if level(entity) < MAX_LEVEL {
        bits |= 16;
    }
    if office > 0 && (office >> 16) & 0x7ff > 0 {
        bits |= 32;
    }
    bits
}

/// Multiplicador dos efeitos: conforto e nível da casa.
fn strength(entity: &ModBlockEntity) -> f64 {
    let comfort = f64::from(number(entity, "Comfort").clamp(0, 100));
    (0.5 + comfort / 100.0) * if level(entity) >= 2 { 1.5 } else { 1.0 }
}

fn effect(host: &mut dyn Host, name: &str, effect: &str, seconds: f64, amplifier: i32) {
    host.run_command(&format!("effect give {name} minecraft:{effect} {} {amplifier} true", seconds.round().max(1.0) as i32));
}

/// Acordar descansado.
fn rest(host: &mut dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity, owner: &str, day: i32) {
    let Some(player) = host.server_players().into_iter().find(|player| player.name == owner) else { return };
    if player.dimension != host.dimension() || center::distance2(player.position, center(pos)) > 12.0 * 12.0 || !spawn::safe_name(owner) {
        return;
    }
    let power = strength(entity);
    effect(host, owner, "regeneration", 20.0, 0);
    effect(host, owner, "speed", 120.0 * power, 0);
    if level(entity) >= MAX_LEVEL {
        effect(host, owner, "haste", 120.0 * power, 0);
    }
    set_number(entity, "RestDay", day + 1);
    host.tell(player.uuid, Text::tr("veio.home.rested", &[&number(entity, "Comfort")]));
}

/// Tigela no lar: refeição em casa; bússola (nível 3): foco da vila; agachar + chave: o lar
/// passa para quem clicou.
pub(crate) fn right_click(host: &mut dyn Host, pos: [i32; 3], held: &str) -> Option<Flow> {
    match held {
        BOWL => meal(host, pos),
        CAKE => banquet(host, pos),
        focus::COMPASS => office(host, pos),
        WRENCH if host.sneaking() => {
            let player = center::nearest_server_player(host, pos, 8.0)?;
            if !spawn::safe_name(&player.name) {
                return Some(Flow::Cancel);
            }
            host.change_mod_block_entity(pos, &mut |entity| set_text(entity, "Owner", &player.name));
            settle(host, pos, &player.name);
            host.tell(player.uuid, Text::tr("veio.home.claimed", &[]));
            Some(Flow::Cancel)
        }
        _ => None,
    }
}

fn office(host: &mut dyn Host, pos: [i32; 3]) -> Option<Flow> {
    let mut home_level = None;
    host.change_mod_block_entity(pos, &mut |entity| if entity.id == HOME { home_level = Some(level(entity)) });
    let home_level = home_level?;
    let player = center::nearest_player(host, pos, 8.0)?;
    if home_level < 3 {
        host.action_bar(player, Text::tr("veio.home.office_locked", &[]));
        return Some(Flow::Cancel);
    }
    let now = defense::clock();
    let Some(village) = projects::nearest(host.dimension(), pos, center::RADIUS * 2, now) else {
        host.action_bar(player, Text::tr("veio.home.no_village", &[]));
        return Some(Flow::Cancel);
    };
    focus::choose(host, player, village.pos, now);
    Some(Flow::Cancel)
}

/// Escolhe o prato: o de maior valor que não está nas últimas refeições; senão, o de maior valor.
fn pick(entity: &ModBlockEntity) -> Option<usize> {
    let recent = menu(entity);
    let value = |slot: usize| entity.items[slot].as_ref().filter(|stack| plain(stack))
        .and_then(|stack| production::pantry::value(&stack.item)).map(|value| value as i32);
    let fresh = |slot: &usize| entity.items[*slot].as_ref().is_some_and(|stack| !recent.iter().any(|item| *item == *stack.item));
    PANTRY.filter(|slot| value(*slot).is_some()).filter(fresh).max_by_key(|slot| value(*slot))
        .or_else(|| PANTRY.filter(|slot| value(*slot).is_some()).max_by_key(|slot| value(*slot)))
}

fn food_name(item: &str) -> String {
    let (space, name) = item.split_once(':').unwrap_or(("minecraft", item));
    let kind = if matches!(name, "cake") { "block" } else { "item" };
    Text::tr(&format!("{kind}.{space}.{name}"), &[]).plain()
}

fn meal(host: &mut dyn Host, pos: [i32; 3]) -> Option<Flow> {
    let player = center::nearest_server_player(host, pos, 6.0)?;
    let now = defense::clock();
    let mut outcome = None;
    host.change_mod_block_entity(pos, &mut |entity| {
        if entity.id != HOME {
            return;
        }
        let ready = number(entity, "MealAt") as u64;
        if ready > now && ready - now <= MEAL_COOLDOWN {
            outcome = Some(Err(((ready - now) / 20) as i32));
            return;
        }
        let Some(slot) = pick(entity) else {
            outcome = Some(Err(-1));
            return;
        };
        let item = entity.items[slot].as_ref().map(|stack| stack.item.to_string()).unwrap_or_default();
        consume(&mut entity.items[slot], 1);
        let mut recent = menu(entity);
        recent.push(item.clone());
        if recent.len() > MENU {
            recent.remove(0);
        }
        set_text(entity, "Menu", &recent.join(","));
        set_number(entity, "MealAt", (now + MEAL_COOLDOWN).min(i32::MAX as u64) as i32);
        set_number(entity, "MealDay", number(entity, "Day") + 1);
        outcome = Some(Ok((item, variety(entity), number(entity, "Burn") > 0, strength(entity), level(entity),
            number(entity, "Comfort"))));
    });
    match outcome? {
        Err(-1) => {
            host.action_bar(player.uuid, Text::tr("veio.home.no_food", &[]));
        }
        Err(wait) => {
            host.action_bar(player.uuid, Text::tr("veio.home.meal_wait_bar", &[&wait]));
        }
        Ok((item, variety, warm, power, level, comfort)) => {
            let name = player.name.clone();
            if spawn::safe_name(&name) {
                // Fome cheia, sempre.
                effect(host, &name, "saturation", 1.0, 0);
                let value = production::pantry::value(&item).unwrap_or(1) as f64;
                if warm {
                    effect(host, &name, "regeneration", (5.0 + value).min(30.0) * power, 0);
                    if variety >= 3 {
                        effect(host, &name, "haste", 180.0 * power, i32::from(level >= MAX_LEVEL));
                    }
                    if variety >= MENU as i32 {
                        effect(host, &name, "resistance", 180.0 * power, 0);
                    }
                }
                match item.as_str() {
                    "minecraft:golden_carrot" => effect(host, &name, "night_vision", 300.0 * power, 0),
                    "minecraft:golden_apple" => {
                        effect(host, &name, "absorption", 120.0 * power, 0);
                        effect(host, &name, "regeneration", 10.0, 1);
                    }
                    _ => {}
                }
            }
            let at = center(pos);
            host.play_sound("minecraft:entity.generic.eat", at, 7, 0.8, 1.0);
            host.send_particles(&ParticleOptions::simple(ParticleType::HappyVillager), None, [at[0], at[1] + 1.0, at[2]], [0.4, 0.3, 0.4], 0.0, 6);
            let key = if warm { "veio.home.meal" } else { "veio.home.meal_cold" };
            host.action_bar(player.uuid, Text::tr(key, &[&food_name(&item), &variety, &comfort]));
        }
    }
    Some(Flow::Cancel)
}

// ---------- Banquete ----------

struct Banquet {
    dimension: String,
    pos: [i32; 3],
    guests: Vec<(u32, Uuid)>,
    until: u64,
}

static BANQUETS: Mutex<Vec<Banquet>> = Mutex::new(Vec::new());
static FEASTING: AtomicBool = AtomicBool::new(false);

/// Bolo no lar (uma vez por dia, lareira acesa): até 6 aldeões livres da vizinhança vêm jantar na
/// sua casa por um minuto. Quem oferece ganha Herói da Vila (descontos no comércio vanilla) e a
/// vila ganha prestígio. Aldeões contratados não são chamados; os convidados andam até lá pelo
/// próprio cérebro (as mesmas ordens dos postos) e voltam à rotina vanilla quando acaba.
fn banquet(host: &mut dyn Host, pos: [i32; 3]) -> Option<Flow> {
    let player = center::nearest_server_player(host, pos, 6.0)?;
    let now = defense::clock();
    let mut state = None;
    host.change_mod_block_entity(pos, &mut |entity| {
        if entity.id == HOME {
            state = Some((number(entity, "Burn") > 0, number(entity, "BanquetDay") == number(entity, "Day") + 1, level(entity)));
        }
    });
    let (lit, done, home_level) = state?;
    if !lit {
        host.action_bar(player.uuid, Text::tr("veio.home.banquet_cold", &[]));
        return Some(Flow::Cancel);
    }
    if done {
        host.action_bar(player.uuid, Text::tr("veio.home.banquet_done", &[]));
        return Some(Flow::Cancel);
    }
    let dimension = host.dimension();
    let at = center(pos);
    let mut near: Vec<(f64, u32)> = production::ranch::villager_positions(host, dimension).iter().copied()
        .filter(|(_, spot)| (spot[0] - at[0]).hypot(spot[2] - at[2]) <= INVITE_RADIUS && (spot[1] - at[1]).abs() <= 8.0)
        .map(|(id, spot)| ((spot[0] - at[0]).hypot(spot[2] - at[2]), id))
        .collect();
    near.sort_by(|a, b| a.0.total_cmp(&b.0));
    let mut guests = Vec::new();
    for (_, id) in near.into_iter().take(GUESTS * 2) {
        let Some(nbt) = host.entity(id).and_then(|handle| host.entity_nbt(handle)) else { continue };
        let Some(uuid) = uuid_of(&nbt) else { continue };
        if adult(&nbt) && !claimed_elsewhere(uuid, dimension, pos, now) {
            guests.push((id, uuid));
            if guests.len() >= GUESTS {
                break;
            }
        }
    }
    if guests.is_empty() {
        host.action_bar(player.uuid, Text::tr("veio.home.banquet_alone", &[]));
        return Some(Flow::Cancel);
    }
    if host.take_items(CAKE, 1) == 0 {
        return Some(Flow::Cancel);
    }
    host.change_mod_block_entity(pos, &mut |entity| set_number(entity, "BanquetDay", number(entity, "Day") + 1));
    let count = guests.len();
    if let Ok(mut list) = BANQUETS.lock() {
        list.retain(|known| known.until > now && !(known.dimension == dimension && known.pos == pos));
        list.push(Banquet { dimension: dimension.to_owned(), pos, guests, until: now + BANQUET_TICKS });
        FEASTING.store(true, Ordering::Release);
    }
    if spawn::safe_name(&player.name) {
        effect(host, &player.name, "hero_of_the_village", 1200.0, i32::from(home_level >= MAX_LEVEL));
    }
    if let Some(village) = projects::nearest(dimension, pos, center::RADIUS * 2, now) {
        center::add_prestige(host, village.pos, 5);
    }
    host.play_sound("minecraft:block.bell.use", at, 4, 0.8, 1.2);
    host.send_particles(&ParticleOptions::simple(ParticleType::HappyVillager), None, [at[0], at[1] + 1.0, at[2]], [1.2, 0.6, 1.2], 0.0, 24);
    host.tell(player.uuid, Text::tr("veio.home.banquet", &[&count]));
    Some(Flow::Cancel)
}

/// Mantém os convidados em volta do lar enquanto o banquete dura.
fn banquet_upkeep(host: &mut dyn Host, pos: [i32; 3], now: u64) {
    if !FEASTING.load(Ordering::Acquire) {
        return;
    }
    let dimension = host.dimension();
    let Ok(mut list) = BANQUETS.lock() else { return };
    let Some(index) = list.iter().position(|known| known.dimension == dimension && known.pos == pos) else { return };
    let at = center(pos);
    if now >= list[index].until {
        for (id, _) in &list[index].guests {
            cancel(*id);
        }
        list.remove(index);
        FEASTING.store(!list.is_empty(), Ordering::Release);
        drop(list);
        host.play_sound("minecraft:entity.villager.celebrate", at, 4, 0.6, 1.0);
        return;
    }
    // Quem foi contratado durante o banquete vai trabalhar.
    list[index].guests.retain(|(_, uuid)| !claimed_elsewhere(*uuid, dimension, pos, now));
    let guests: Vec<u32> = list[index].guests.iter().map(|(id, _)| *id).collect();
    drop(list);
    for (seat, id) in guests.iter().enumerate() {
        let [dx, dz] = SEATS[seat % SEATS.len()];
        order(*id, [at[0] + dx, at[1] - 0.5, at[2] + dz], now);
    }
    if (now / 10) % 4 == 0 {
        for id in guests {
            if let Some(spot) = host.entity(id).and_then(|handle| handle.position(host)) {
                host.send_particles(&ParticleOptions::simple(ParticleType::HappyVillager), None,
                    [spot[0], spot[1] + 2.1, spot[2]], [0.2, 0.2, 0.2], 0.0, 3);
            }
        }
    }
}

/// Convidados de um banquete em andamento no lar em `pos`.
#[cfg(test)]
pub(crate) fn guests(dimension: &str, pos: [i32; 3]) -> Vec<u32> {
    BANQUETS.lock().map(|list| list.iter().filter(|known| known.dimension == dimension && known.pos == pos)
        .flat_map(|known| known.guests.iter().map(|(id, _)| *id)).collect()).unwrap_or_default()
}

// ---------- Painel ----------

/// Os 13 valores do painel do lar.
pub(crate) fn data(entity: &ModBlockEntity) -> [i32; 13] {
    let (index, left) = missing(entity).unwrap_or((0, 0));
    let office = match entity.nbt.get(nbt_key("Office")) { Some(Tag::Int(value)) => *value, _ => -1 };
    [number(entity, "Status"), number(entity, "Comfort"), number(entity, "Have"), level(entity),
        number(entity, "Burn") / 20, pantry_count(entity) as i32, number(entity, "Todo"), upgrade_percent(entity),
        index as i32 | ((left.min(1023) as i32) << 4), number(entity, "Streak"), variety(entity), office, number(entity, "MealWait")]
}

pub(crate) fn screen_slots() -> Vec<Option<[f32; 2]>> {
    vec![Some([17.0, 100.0]), Some([44.0, 100.0]), Some([62.0, 100.0]), Some([80.0, 100.0]), Some([152.0, 100.0])]
}

const ROW: f32 = 10.0;
const TOP: f32 = 20.0;
const WARN: u32 = 0x9a5a00;
const GOOD: u32 = 0x1e7a1e;

fn missing_names(have: i32) -> Vec<String> {
    CATEGORIES.iter().enumerate()
        .filter(|(index, _)| *index < 7 && have & (1 << index) == 0)
        .map(|(_, (key, _))| Text::tr(&format!("veio.home.cat.{key}"), &[]).plain())
        .collect()
}

fn lines(data: &[i32]) -> Vec<(String, u32)> {
    let read = |index: usize| data.get(index).copied().unwrap_or(0);
    let tr = |key: &str, args: &[&dyn std::fmt::Display]| Text::tr(key, args).plain();
    let level = read(3).clamp(1, MAX_LEVEL);
    let todo = read(6);
    let burn = read(4);
    let mut lines = vec![
        if burn > 0 {
            (tr("veio.home.lit", &[&format!("{}:{:02}", burn / 60, burn % 60)]), GOOD)
        } else {
            (tr("veio.home.cold", &[]), WARN)
        },
        (tr("veio.home.comfort", &[&read(1), &level, &tr(&format!("veio.home.level.{level}"), &[])]), presentation::INK),
    ];
    lines.push(if todo & 7 == 0 {
        (tr("veio.home.day_done", &[&read(9)]), GOOD)
    } else {
        let first = [(4, "meal"), (1, "fire"), (2, "pantry")].iter().find(|(bit, _)| todo & bit != 0).map_or("meal", |(_, key)| key);
        (tr("veio.home.today", &[&tr(&format!("veio.home.task.{first}"), &[])]), WARN)
    });
    lines.push(if read(12) > 0 {
        (tr("veio.home.meal_wait", &[&read(12), &read(10)]), presentation::INK)
    } else {
        (tr("veio.home.meal_ready", &[&read(5), &read(10)]), presentation::INK)
    });
    let missing = missing_names(read(2));
    lines.push(if missing.is_empty() {
        (tr("veio.home.cozy", &[]), GOOD)
    } else {
        (tr("veio.home.missing", &[&missing.join(", ")]), WARN)
    });
    lines.push(match LEVELS.get((level - 1) as usize) {
        Some(next) => {
            let (index, left) = ((read(8) & 15) as usize, read(8) >> 4);
            (tr("veio.home.upgrade", &[&(level + 1), &read(7), &left, &tr(next[index.min(2)].name, &[])]), presentation::INK)
        }
        None => (tr("veio.home.max", &[]), GOOD),
    });
    let office = read(11);
    lines.push(if level < 3 {
        (tr("veio.home.office_locked", &[]), presentation::SOFT_INK)
    } else if office < 0 {
        (tr("veio.home.no_village", &[]), presentation::SOFT_INK)
    } else {
        let waiting = (office >> 16) & 255;
        let open = (office >> 24) & 7;
        (tr("veio.home.village", &[&((office >> 8) & 255), &(office & 255), &waiting, &open]), if waiting > 0 { WARN } else { GOOD })
    });
    lines.push((tr("veio.home.hint", &[]), presentation::SOFT_INK));
    lines
}

pub(crate) fn draw_screen(host: &dyn Host, event: &mut horizon_client::mod_api::ScreenDrawEvent<'_>) {
    let [x, y] = event.origin;
    for (index, (line, color)) in lines(event.data).iter().enumerate() {
        host.text(event.batch, &center::fit(host, line, 176.0), [x + 8.0, y + TOP + index as f32 * ROW], *color, false);
    }
}

pub(crate) fn tooltip(event: &mut horizon_client::mod_api::ScreenTooltipEvent<'_>) -> bool {
    let [x, y] = event.at;
    if !(8.0..186.0).contains(&x) || !(TOP..TOP + 8.0 * ROW).contains(&y) {
        return false;
    }
    let read = |index: usize| event.data.get(index).copied().unwrap_or(0);
    let tr = |key: &str, args: &[&dyn std::fmt::Display]| Text::tr(key, args).plain();
    let row = ((y - TOP) / ROW) as usize;
    match row {
        1 | 4 => {
            let have = read(2);
            event.lines.push(tr("veio.home.tip.comfort", &[]));
            for (index, (key, points)) in CATEGORIES.iter().enumerate() {
                let got = if index >= 7 { -1 } else if have & (1 << index) != 0 { *points } else { 0 };
                let name = tr(&format!("veio.home.cat.{key}"), &[]);
                event.lines.push(if got < 0 { format!("  {name}: 0-{points}") } else { format!("  {name}: {got}/{points}") });
            }
        }
        5 => {
            let level = read(3).clamp(1, MAX_LEVEL);
            match LEVELS.get((level - 1) as usize) {
                Some(next) => {
                    event.lines.push(tr(&format!("veio.home.level.{}", level + 1), &[]));
                    event.lines.extend(tr(&format!("veio.home.perk.{}", level + 1), &[]).lines().map(str::to_owned));
                    let list: Vec<String> = next.iter().map(|need| format!("{} {}", need.count, tr(need.name, &[]))).collect();
                    event.lines.push(tr("veio.project.needs", &[&list.join(", ")]));
                    event.lines.push(tr("veio.home.tip.build", &[]));
                }
                None => event.lines.push(tr("veio.home.max", &[])),
            }
        }
        _ => {
            let key = ["fire", "comfort", "today", "meal", "comfort", "build", "office", "hint"][row.min(7)];
            event.lines.extend(tr(&format!("veio.home.tip.{key}"), &[]).lines().map(str::to_owned));
        }
    }
    true
}
