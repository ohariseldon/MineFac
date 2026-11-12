//! Os pedidos da vila: a vila está em declínio e cada morador que sobrou tem um problema. Cada
//! pedido é uma porta para uma área do jogo, feita à mão, como no começo do Minecraft:
//!
//! - **O telhado** (o prefeito e a Dona Zefa, a oleira): argila, fornalha, tijolo e construção.
//! - **A vendinha** (o Seu Zé): pão e lã para a prateleira e a primeira venda (comércio).
//! - **O rebanho** (o Seu Tonho e a Mimosa, a última vaca): atrair, criar e ovelhas (pecuária).
//! - **A carvoaria** (o Seu Lino): lenha, carvão vegetal e a roda d'água (energia).
//! - **A forja** (o Seu Bento, depois do favor da prensa): ferro à mão e a chapa na prensa.
//!
//! Nível 1 é o telhado, que o prefeito pede no primeiro encontro. Enquanto ele está aberto, o
//! prefeito vai contando, um por conversa, quem mais precisa de ajuda (nível 2). Com todos
//! atendidos, "a vila respira" (nível 3). Nada disso trava o tutorial da fábrica: são pedidos
//! paralelos, na ordem que o jogador quiser, e `/veiotutorial pedidos` mostra o caderno.
//!
//! Os moradores são aldeões parados com nome (`veio:potter`, `veio:grocer`, `veio:cowherd`,
//! `veio:collier`), em lugares fixos a partir do centro da vila, como os do favor
//! (`favors.rs`). O estado vai no save do tutorial, numa linha própria (`pedidos ...`).
use super::*;
use favors::Who;
use horizon_client::mod_api::NativeHeightmap;
use workforce::spawn::OVERWORLD;

pub(crate) const POTTER: &str = "veio:potter";
pub(crate) const GROCER: &str = "veio:grocer";
pub(crate) const COWHERD: &str = "veio:cowherd";
pub(crate) const COLLIER: &str = "veio:collier";

/// Quem pede: os quatro moradores novos e o Seu Bento (do favor).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum Person { Potter, Grocer, Cowherd, Collier, Smith }

const PEOPLE: [Person; 4] = [Person::Potter, Person::Grocer, Person::Cowherd, Person::Collier];

impl Person {
    pub(super) fn kind(self) -> &'static str {
        match self { Person::Potter => POTTER, Person::Grocer => GROCER, Person::Cowherd => COWHERD,
            Person::Collier => COLLIER, Person::Smith => favors::SMITH }
    }
    fn name_key(self) -> &'static str {
        match self { Person::Potter => "veio.errand.potter.name", Person::Grocer => "veio.errand.grocer.name",
            Person::Cowherd => "veio.errand.cowherd.name", Person::Collier => "veio.errand.collier.name",
            Person::Smith => "veio.favor.smith.name" }
    }
    /// Onde cada um mora, a partir do centro da vila (o ferreiro e a lavradora ficam a leste e a
    /// oeste, `favors.rs`): a oleira ao sul, a vendinha a sudeste, o pasto a sudoeste, a
    /// carvoaria a nordeste.
    fn offsets(self) -> [(i32, i32); 4] {
        match self {
            Person::Potter => [(0, 11), (3, 11), (-3, 11), (0, 13)],
            Person::Grocer => [(9, 9), (11, 8), (8, 11), (11, 11)],
            Person::Cowherd => [(-12, 10), (-13, 7), (-10, 12), (-14, 11)],
            Person::Collier => [(11, -9), (13, -7), (9, -11), (14, -10)],
            Person::Smith => [(8, 0), (8, 3), (8, -3), (7, 6)],
        }
    }
    /// Na vila do Veio: na frente da casa ou da oficina de cada um (`town_plan`).
    fn planned(self) -> &'static [(i32, i32); 4] {
        match self {
            Person::Potter => &workforce::town::POTTER, Person::Grocer => &workforce::town::GROCER,
            Person::Cowherd => &workforce::town::COWHERD, Person::Collier => &workforce::town::COLLIER,
            Person::Smith => &workforce::town::SMITH,
        }
    }
    fn villager(self) -> (&'static str, &'static str) {
        match self {
            Person::Potter => ("minecraft:mason", "minecraft:desert"),
            Person::Grocer => ("minecraft:butcher", "minecraft:plains"),
            Person::Cowherd => ("minecraft:leatherworker", "minecraft:savanna"),
            Person::Collier => ("minecraft:armorer", "minecraft:taiga"),
            Person::Smith => ("minecraft:toolsmith", "minecraft:plains"),
        }
    }
    fn slot(self) -> usize {
        match self { Person::Potter => 0, Person::Grocer => 1, Person::Cowherd => 2, Person::Collier => 3, Person::Smith => 4 }
    }
}

pub(super) fn person(kind: &str) -> Option<Person> {
    match kind { POTTER => Some(Person::Potter), GROCER => Some(Person::Grocer), COWHERD => Some(Person::Cowherd),
        COLLIER => Some(Person::Collier), _ => None }
}

/// O que uma tarefa do pedido pede.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum Task {
    /// Conhecer quem pede (a primeira conversa).
    Meet,
    /// Entregar na conversa: qualquer um destes itens, somando a quantidade.
    Bring(&'static [&'static str], u32),
    /// Animais desta espécie (`ranch::KINDS`) perto de quem pede.
    Herd(u8, u32),
    /// Blocos de tijolo colocados perto do prefeito (o telhado do coreto).
    Build(u32),
    /// Colocar este bloco em qualquer lugar.
    Place(&'static str),
    /// Uma troca com quem pede.
    Trade,
}

pub(super) struct Errand {
    pub key: &'static str,
    pub who: Person,
    pub tasks: &'static [Task],
    /// Falas de cada tarefa: (ao começar, ao terminar).
    pub lines: &'static [(u8, u8)],
    pub xp: u32,
}

const LOGS: &[&str] = &["minecraft:oak_log", "minecraft:spruce_log", "minecraft:birch_log", "minecraft:jungle_log",
    "minecraft:acacia_log", "minecraft:dark_oak_log", "minecraft:mangrove_log", "minecraft:cherry_log"];
const WOOL: &[&str] = &["minecraft:white_wool", "minecraft:light_gray_wool", "minecraft:gray_wool", "minecraft:black_wool",
    "minecraft:brown_wool", "minecraft:red_wool", "minecraft:orange_wool", "minecraft:yellow_wool", "minecraft:lime_wool",
    "minecraft:green_wool", "minecraft:cyan_wool", "minecraft:light_blue_wool", "minecraft:blue_wool",
    "minecraft:purple_wool", "minecraft:magenta_wool", "minecraft:pink_wool"];
/// As "telhas": a família do tijolo.
const BRICKS: &[&str] = &["minecraft:bricks", "minecraft:brick_stairs", "minecraft:brick_slab", "minecraft:brick_wall"];
const COW: u8 = 0;
const SHEEP: u8 = 1;

pub(super) const ROOF: usize = 0;
pub(super) const SHOP: usize = 1;
pub(super) const HERD: usize = 2;
pub(super) const COAL: usize = 3;
pub(super) const FORGE: usize = 4;
pub(super) const COUNT: usize = 5;

pub(super) const ERRANDS: [Errand; COUNT] = [
    Errand { key: "roof", who: Person::Potter, xp: 40,
        tasks: &[Task::Meet, Task::Bring(&["minecraft:brick"], 16), Task::Build(8)],
        lines: &[(3, 0), (3, 3), (2, 4)] },
    Errand { key: "shop", who: Person::Grocer, xp: 50,
        tasks: &[Task::Meet, Task::Bring(&["minecraft:bread"], 6), Task::Bring(WOOL, 8), Task::Trade],
        lines: &[(4, 0), (3, 2), (3, 3), (3, 4)] },
    Errand { key: "herd", who: Person::Cowherd, xp: 50,
        tasks: &[Task::Meet, Task::Herd(COW, 2), Task::Herd(COW, 4), Task::Herd(SHEEP, 2)],
        lines: &[(3, 0), (3, 2), (3, 2), (3, 4)] },
    Errand { key: "coal", who: Person::Collier, xp: 50,
        tasks: &[Task::Meet, Task::Bring(LOGS, 16), Task::Bring(&["minecraft:charcoal"], 8), Task::Place(production::WATER_WHEEL)],
        lines: &[(4, 0), (3, 2), (3, 3), (3, 4)] },
    Errand { key: "forge", who: Person::Smith, xp: 50,
        tasks: &[Task::Meet, Task::Bring(&["minecraft:iron_ingot"], 8), Task::Bring(&["veio:iron_plate"], 4)],
        lines: &[(3, 0), (3, 3), (3, 4)] },
];

impl Errand {
    pub(super) fn done(&self) -> u8 { self.tasks.len() as u8 + 1 }
}

/// A tarefa em andamento (`state` 1..=tarefas), se o pedido está aberto.
pub(super) fn task(index: usize, state: u8) -> Option<usize> {
    let errand = &ERRANDS[index];
    (1..errand.done()).contains(&state).then(|| usize::from(state - 1))
}

/// Bits de `Progress::errand_flags`: o prefeito já contou deste pedido (0..5), a Mimosa chegou,
/// a vila respira, a roda d'água foi colocada, houve uma troca na vendinha.
pub(super) const MIMOSA: u16 = 1 << 8;
pub(super) const REVIVED: u16 = 1 << 9;
pub(super) const WHEEL: u16 = 1 << 10;
pub(super) const TRADED: u16 = 1 << 11;
/// O prefeito vai contar do telhado logo depois do primeiro encontro (a conversa já é longa).
pub(super) const PENDING: u16 = 1 << 12;
/// Quanto depois do primeiro encontro (relógio do tutorial, 10 s).
const PAUSE: u64 = 200;
fn told(index: usize) -> u16 { 1 << index }

/// Até onde do prefeito o tijolo conta como telhado do coreto.
const ROOF_REACH: i32 = 10;
/// Até onde do Seu Tonho o animal conta como do rebanho dele.
const PASTURE: f64 = 12.0;
/// Perto disto a barra mostra o pedido de quem está ao lado.
const NEAR: f64 = 8.0;
/// Até onde o jogador vê o brilho de quem tem um pedido para ele.
const GLOW: f64 = 48.0;

/// O pedido está disponível? A forja só depois do favor da prensa.
pub(super) fn available(index: usize, p: &Progress) -> bool {
    index != FORGE || p.favor >= favors::FAVOR_DONE
}

// ---------- Moradores no mundo ----------

static IDS: Mutex<[Option<u32>; 4]> = Mutex::new([None; 4]);

/// Tick de um morador: guarda o id e tira cópias a mais.
pub(crate) fn seen(host: &mut dyn Host, dimension: &str, kind: &str, id: u32) {
    let Some(who) = person(kind) else { return };
    let Ok(mut ids) = IDS.lock() else { return };
    match ids[who.slot()] {
        Some(other) if other != id && host.entity(other).is_some() => { host.remove_mob(dimension, id); }
        _ => ids[who.slot()] = Some(id),
    }
}

pub(super) fn reset() { if let Ok(mut ids) = IDS.lock() { *ids = [None; 4]; } }

/// Os moradores dos pedidos no mundo: (tipo, id, onde trabalham).
pub(super) fn residents(host: &dyn Host) -> Vec<(&'static str, Option<u32>, Option<[f64; 3]>)> {
    let ids = IDS.lock().map(|ids| *ids).unwrap_or([None; 4]);
    PEOPLE.into_iter().map(|who| (who.kind(), ids[who.slot()], home(host, who))).collect()
}

/// O lugar fixo de cada um: o chão mais baixo e seco entre as colunas dele.
fn home(host: &dyn Host, who: Person) -> Option<[f64; 3]> {
    let ([x, _, z], _) = workforce::spawn::center()?;
    let columns: Vec<(i32, i32)> = workforce::town::spots(host, who.planned())
        .unwrap_or_else(|| who.offsets().iter().map(|(dx, dz)| (x + dx, z + dz)).collect());
    let ground = host.surface_heights(OVERWORLD, &columns, NativeHeightmap::MotionBlockingNoLeaves).ok()?;
    let floor = host.surface_heights(OVERWORLD, &columns, NativeHeightmap::OceanFloor).ok()?;
    columns.iter().zip(ground.iter().zip(&floor))
        .filter_map(|(col, (g, f))| (g.is_some() && g == f).then(|| (*col, g.unwrap())))
        .min_by_key(|(_, height)| *height)
        .map(|((cx, cz), y)| [f64::from(cx) + 0.5, f64::from(y), f64::from(cz) + 0.5])
}

/// Onde a pessoa está agora: a entidade, ou o lugar fixo dela.
pub(super) fn at(host: &dyn Host, who: Person) -> Option<[f64; 3]> {
    if who == Person::Smith { return favors::at(host, Who::Smith); }
    let id = IDS.lock().ok().and_then(|ids| ids[who.slot()]);
    id.and_then(|id| host.entity(id)).and_then(|entity| entity.position(host)).or_else(|| home(host, who))
}

fn summon(host: &mut dyn Host, who: Person) {
    if who == Person::Smith { return; }
    let id = IDS.lock().ok().and_then(|ids| ids[who.slot()]);
    if id.and_then(|id| host.entity(id)).is_some() { return; }
    if host.run_command(&format!("execute if entity @e[type={}]", who.kind())) { return; }
    let Some(spot) = home(host, who) else { return };
    let name = Text::tr(who.name_key(), &[]).plain();
    let (profession, biome) = who.villager();
    let nbt = Compound::from([
        ("CustomName".into(), Tag::String(name.as_str().into())),
        ("CustomNameVisible".into(), Tag::Byte(1)),
        ("NoAI".into(), Tag::Byte(1)),
        ("Invulnerable".into(), Tag::Byte(1)),
        ("PersistenceRequired".into(), Tag::Byte(1)),
        ("VillagerData".into(), Tag::Compound(Compound::from([
            ("profession".into(), Tag::String(profession.into())),
            ("type".into(), Tag::String(biome.into())),
            ("level".into(), Tag::Int(1)),
        ]))),
    ]);
    if let Some(id) = host.summon(OVERWORLD, who.kind(), spot, nbt) {
        if let Ok(mut ids) = IDS.lock() { ids[who.slot()] = Some(id); }
    }
}

/// A Mimosa, a última vaca da vila, chega ao pasto do Seu Tonho.
/// `true` quando ela já está lá (ou acabou de chegar); sem o pasto conhecido, tenta de novo depois.
fn mimosa(host: &mut dyn Host) -> bool {
    if host.run_command("execute if entity @e[type=minecraft:cow,name=Mimosa]") { return true; }
    let Some([x, y, z]) = at(host, Person::Cowherd) else { return false };
    let nbt = Compound::from([
        ("CustomName".into(), Tag::String("Mimosa".into())),
        ("PersistenceRequired".into(), Tag::Byte(1)),
    ]);
    host.summon(OVERWORLD, "minecraft:cow", [x + 2.0, y, z + 1.0], nbt).is_some()
}

// ---------- Falas ----------

fn say(host: &mut dyn Host, player: u128, who: Person, key: &str, args: &[&dyn std::fmt::Display]) {
    let name = Text::tr(who.name_key(), &[]).plain();
    host.tell(player, Text::tr("veio.agenda.says", &[&name, &Text::tr(key, args).plain()]));
}

fn react(host: &mut dyn Host, who: Person, happy: bool) {
    let (particle, sound) = if happy { ("minecraft:heart", "minecraft:entity.villager.celebrate") }
        else { ("minecraft:angry_villager", "minecraft:entity.villager.no") };
    let kind = who.kind();
    host.run_command(&format!("execute at @e[type={kind},limit=1] run particle {particle} ~ ~2.1 ~ 0.3 0.2 0.3 0 5"));
    host.run_command(&format!("execute at @e[type={kind},limit=1] run playsound {sound} neutral @a[distance=..16] ~ ~ ~"));
}

/// Quanto falta da tarefa: (feito, pedido).
fn count(task: Task, p: &Progress, index: usize) -> (u32, u32) {
    match task {
        Task::Bring(_, n) | Task::Herd(_, n) => (p.errand_counts[index].min(n), n),
        Task::Build(n) => (u32::from(p.errand_built).min(n), n),
        _ => (0, 1),
    }
}

/// O texto do que a tarefa pede, com a contagem.
fn goal(index: usize, current: usize, p: &Progress) -> Text {
    let errand = &ERRANDS[index];
    let (have, need) = count(errand.tasks[current], p, index);
    Text::tr(&format!("veio.errand.{}.goal.{current}", errand.key), &[&have, &need])
}

/// Abre a tarefa: o morador explica o que precisa e por quê.
fn ask(host: &mut dyn Host, player: &NativeServerPlayer, p: &mut Progress, index: usize, current: usize) {
    let errand = &ERRANDS[index];
    for line in 1..=errand.lines[current].0 {
        say(host, player.uuid, errand.who, &format!("veio.errand.{}.ask.{current}.{line}", errand.key), &[]);
    }
    if current > 0 { host.tell(player.uuid, Text::tr("veio.errand.task", &[&goal(index, current, p).plain()])); }
    // O que cada tarefa precisa ao começar: a Mimosa, a tesoura, a receita da roda d'água, as
    // trocas da vendinha.
    match (index, current) {
        (HERD, 1) if p.errand_flags & MIMOSA == 0 && mimosa(host) => p.errand_flags |= MIMOSA,
        (HERD, 3) => { host.give_items("minecraft:shears", 1); host.tell(player.uuid, Text::tr("veio.errand.herd.gift", &[])); }
        (COAL, 3) => if spawn_safe(&player.name) {
            host.run_command(&format!("recipe give {} {}", player.name, production::WATER_WHEEL));
            host.tell(player.uuid, Text::tr("veio.onboarding.book", &[]));
        },
        (SHOP, 3) => trades(host, Person::Grocer),
        _ => {}
    }
    p.errand_counts[index] = 0;
    p.dirty = true;
}

/// A tarefa terminou: agradece, dá o que prometeu e abre a próxima (ou fecha o pedido).
fn advance(host: &mut dyn Host, player: &NativeServerPlayer, p: &mut Progress, index: usize, finished: usize) {
    let errand = &ERRANDS[index];
    for line in 1..=errand.lines[finished].1 {
        say(host, player.uuid, errand.who, &format!("veio.errand.{}.done.{finished}.{line}", errand.key), &[]);
    }
    if errand.lines[finished].1 > 0 { react(host, errand.who, true); }
    match (index, finished) {
        // A Dona Zefa molda as telhas com o tijolo que você assou.
        (ROOF, 1) => { host.give_items("minecraft:brick_stairs", 8); host.tell(player.uuid, Text::tr("veio.errand.roof.gift", &[])); }
        (COAL, 2) => { host.give_items("minecraft:charcoal", 8); host.tell(player.uuid, Text::tr("veio.errand.coal.gift", &[])); }
        _ => {}
    }
    p.errands[index] += 1;
    p.errand_counts[index] = 0;
    p.dirty = true;
    match task(index, p.errands[index]) {
        Some(next) => ask(host, player, p, index, next),
        None => close(host, player, p, index),
    }
}

/// O pedido inteiro foi atendido: título, experiência, as trocas e o que o sistema do jogo faz
/// por esse morador mais tarde.
fn close(host: &mut dyn Host, player: &NativeServerPlayer, p: &mut Progress, index: usize) {
    let errand = &ERRANDS[index];
    match index {
        ROOF => {
            for line in 1..=3 { intro::say(host, player.uuid, &format!("veio.errand.roof.mayor.{line}"), &[]); }
            intro::react(host, intro::Mood::Happy);
            trades(host, Person::Potter);
        }
        HERD => {
            host.give_items("minecraft:lead", 2);
            if spawn_safe(&player.name) {
                for recipe in ["veio:pen_marker", "veio:shepherd_post"] {
                    host.run_command(&format!("recipe give {} {recipe}", player.name));
                }
            }
            trades(host, Person::Cowherd);
        }
        COAL => trades(host, Person::Collier),
        FORGE => trades(host, Person::Smith),
        _ => {}
    }
    host.tell(player.uuid, Text::tr(&format!("veio.errand.{}.next", errand.key), &[]));
    if spawn_safe(&player.name) {
        let name = &player.name;
        host.run_command(&format!(r#"title {name} subtitle {{"translate":"veio.errand.{}.title"}}"#, errand.key));
        host.run_command(&format!(r#"title {name} title {{"translate":"veio.errand.done","color":"gold"}}"#));
        host.run_command(&format!("execute as {name} at @s run playsound minecraft:ui.toast.challenge_complete master @s"));
        host.run_command(&format!("xp add {name} {} points", errand.xp));
    }
    if ERRANDS.iter().enumerate().all(|(i, e)| p.errands[i] == e.done()) && p.errand_flags & REVIVED == 0 {
        p.errand_flags |= REVIVED;
        // A vila respira: fonte com água, coreto com telhado, as casas em obra prontas.
        workforce::town::advance(2);
        for line in 1..=4 { intro::say(host, player.uuid, &format!("veio.errand.revival.{line}"), &[]); }
        intro::react(host, intro::Mood::Happy);
        if spawn_safe(&player.name) {
            let name = &player.name;
            host.run_command(&format!(r#"title {name} subtitle {{"translate":"veio.errand.revival.sub"}}"#));
            host.run_command(&format!(r#"title {name} title {{"translate":"veio.errand.revival.title","color":"gold"}}"#));
            host.run_command(&format!("xp add {name} 100 points"));
        }
        fireworks(host, &player.name);
    }
    p.dirty = true;
}

/// As trocas vanilla de cada morador depois do pedido (a vendinha: já na última tarefa). A vila
/// compra o que ela precisa de você e vende o que ela faz.
fn trades(host: &mut dyn Host, who: Person) {
    let offer = |buy: &str, buy_count: u32, sell: &str, sell_count: u32| format!(
        r#"{{buy:{{id:"{buy}",count:{buy_count}}},sell:{{id:"{sell}",count:{sell_count}}},maxUses:16,rewardExp:0b,xp:1}}"#);
    let offers = match who {
        Person::Potter => vec![offer("minecraft:clay_ball", 10, "minecraft:emerald", 1), offer("minecraft:emerald", 1, "minecraft:bricks", 4)],
        Person::Grocer => vec![offer("minecraft:wheat", 20, "minecraft:emerald", 1), offer("minecraft:bread", 6, "minecraft:emerald", 1),
            offer("minecraft:white_wool", 12, "minecraft:emerald", 1)],
        Person::Cowherd => vec![offer("minecraft:emerald", 1, "minecraft:lead", 1), offer("minecraft:wheat", 16, "minecraft:emerald", 1)],
        Person::Collier => vec![offer("minecraft:emerald", 1, "minecraft:charcoal", 8), offer("minecraft:oak_log", 16, "minecraft:emerald", 1)],
        Person::Smith => vec![offer("minecraft:iron_ingot", 4, "minecraft:emerald", 1)],
    };
    host.run_command(&format!("execute as @e[type={},limit=1] run data merge entity @s {{Offers:{{Recipes:[{}]}}}}",
        who.kind(), offers.join(",")));
}

// ---------- Conversa ----------

/// Primeiro encontro com o prefeito: os moradores aparecem e, uns segundos depois, ele conta da
/// vila em declínio e pede o telhado.
pub(super) fn queue(host: &mut dyn Host, p: &mut Progress) {
    if p.errand_flags & told(ROOF) != 0 { return; }
    p.errand_flags |= PENDING;
    p.dirty = true;
    for who in PEOPLE { summon(host, who); }
}

/// O prefeito apresenta a vila em declínio e pede o telhado.
pub(super) fn introduce(host: &mut dyn Host, player: &NativeServerPlayer, p: &mut Progress) {
    if p.errand_flags & told(ROOF) != 0 { return; }
    for line in 1..=2 { intro::say(host, player.uuid, &format!("veio.errand.decline.{line}"), &[]); }
    hint(host, player, p, ROOF);
    for who in PEOPLE { summon(host, who); }
}

/// O prefeito conta de um morador que precisa de ajuda: onde ele está e o que aflige.
fn hint(host: &mut dyn Host, player: &NativeServerPlayer, p: &mut Progress, index: usize) {
    let errand = &ERRANDS[index];
    let (distance, way) = at(host, errand.who).map_or((0, String::new()), |spot| intro::pointer(player, spot));
    intro::say(host, player.uuid, &format!("veio.errand.{}.hint", errand.key), &[&distance, &way]);
    p.errand_flags |= told(index);
    p.dirty = true;
}

/// Numa conversa com o prefeito depois do primeiro encontro: no máximo uma sugestão por vez,
/// de quem ele ainda não contou, na ordem da história.
pub(super) fn suggest(host: &mut dyn Host, player: &NativeServerPlayer, p: &mut Progress) {
    let next = [ROOF, SHOP, HERD, COAL, FORGE].into_iter()
        .find(|&index| available(index, p) && p.errands[index] == 0 && p.errand_flags & told(index) == 0);
    if let Some(index) = next { hint(host, player, p, index); }
}

/// Clique num morador. `true`: o Veio tomou o clique; `false`: abre a troca vanilla.
pub(super) fn talk(host: &mut dyn Host, player: &NativeServerPlayer, p: &mut Progress, who: Person) -> bool {
    let Some(index) = ERRANDS.iter().position(|errand| errand.who == who) else { return false };
    host.run_command(&format!("execute at @e[type={},limit=1] run playsound minecraft:entity.villager.ambient neutral @a[distance=..16] ~ ~ ~", who.kind()));
    let errand = &ERRANDS[index];
    if !available(index, p) {
        say(host, player.uuid, who, &format!("veio.errand.{}.busy", errand.key), &[]);
        return true;
    }
    if p.errands[index] == 0 {
        // A primeira conversa: quem pede se apresenta e conta o problema.
        p.errands[index] = 1;
        p.errand_flags |= told(index);
        ask(host, player, p, index, 0);
        advance(host, player, p, index, 0);
        return true;
    }
    let Some(current) = task(index, p.errands[index]) else {
        say(host, player.uuid, who, &format!("veio.errand.{}.after", errand.key), &[]);
        // Depois do pedido, cada morador tem as suas trocas.
        return false;
    };
    match errand.tasks[current] {
        Task::Bring(items, need) => {
            let mut got = 0;
            for item in items { if got < need { got += host.take_items(item, need - got); } }
            if got < need {
                // Entrega parcial: devolve e diz quanto falta.
                let mut back = got;
                for item in items {
                    if back == 0 { break; }
                    let given = host.give_items(item, back);
                    back = back.saturating_sub(given);
                    if given > 0 { break; }
                }
                say(host, player.uuid, who, &format!("veio.errand.{}.more.{current}", errand.key), &[&(need - got), &need]);
                react(host, who, false);
                return true;
            }
            advance(host, player, p, index, current);
        }
        // Na vendinha, a última tarefa é vender: a troca abre.
        Task::Trade => {
            say(host, player.uuid, who, &format!("veio.errand.{}.wait.{current}", errand.key), &[]);
            return false;
        }
        _ => {
            let (have, need) = count(errand.tasks[current], p, index);
            say(host, player.uuid, who, &format!("veio.errand.{}.wait.{current}", errand.key), &[&have, &need]);
        }
    }
    true
}

// ---------- Mundo ----------

/// Colocou um bloco: tijolo perto do prefeito (telhado) ou a roda d'água (carvoaria).
pub(super) fn placed(p: &mut Progress, dimension: &str, pos: Pos, block: &str) {
    if dimension != OVERWORLD { return; }
    if BRICKS.contains(&block) && task(ROOF, p.errands[ROOF]) == Some(2) && let Some(mayor) = p.mayor {
        let close = (pos[0] - mayor[0].floor() as i32).abs() <= ROOF_REACH && (pos[2] - mayor[2].floor() as i32).abs() <= ROOF_REACH
            && (pos[1] - mayor[1].floor() as i32).abs() <= ROOF_REACH;
        if close { p.errand_built = p.errand_built.saturating_add(1).min(64); p.dirty = true; }
    }
    if block == production::WATER_WHEEL && task(COAL, p.errands[COAL]) == Some(3) { p.errand_flags |= WHEEL; p.dirty = true; }
}

pub(super) fn placement(block: &str) -> bool { BRICKS.contains(&block) || block == production::WATER_WHEEL }

/// Uma troca com um morador dos pedidos: a primeira venda na vendinha.
pub(super) fn traded(p: &mut Progress, who: Person) {
    if who == Person::Grocer && task(SHOP, p.errands[SHOP]) == Some(3) { p.errand_flags |= TRADED; p.dirty = true; }
}

/// Animais desta espécie perto de um ponto, pelo registro do rancho.
fn animals(host: &dyn Host, kind: u8, near: [f64; 3]) -> u32 {
    production::ranch::recent(OVERWORLD).iter().filter(|(_, index)| *index == kind)
        .filter_map(|(id, _)| host.entity(*id).and_then(|entity| entity.position(host)))
        .filter(|spot| (spot[0] - near[0]).hypot(spot[2] - near[2]) <= PASTURE && (spot[1] - near[1]).abs() <= 6.0)
        .count() as u32
}

/// A cada segundo: confere o que se faz no mundo (rebanho, telhado, roda d'água, a venda), põe
/// os moradores na vila, o brilho em quem tem algo para você e, perto de alguém, o pedido na barra.
pub(super) fn tick(host: &mut dyn Host, player: &NativeServerPlayer, p: &mut Progress, now: u64) {
    if player.dimension != OVERWORLD || p.intro < intro::GATHER_STAGE { return; }
    if p.errand_flags & PENDING != 0 {
        if p.errand_due == 0 { p.errand_due = now + PAUSE; }
        if now >= p.errand_due {
            p.errand_flags &= !PENDING;
            introduce(host, player, p);
        }
    }
    if now % 100 == 0 {
        for who in PEOPLE { summon(host, who); }
        if task(HERD, p.errands[HERD]).is_some() && p.errand_flags & MIMOSA == 0 && mimosa(host) { p.errand_flags |= MIMOSA; p.dirty = true; }
    }
    for index in 0..COUNT {
        let Some(current) = task(index, p.errands[index]) else { continue };
        let finished = match ERRANDS[index].tasks[current] {
            Task::Herd(kind, need) => {
                if now % 100 == 0 && let Some(near) = at(host, Person::Cowherd) {
                    p.errand_counts[index] = animals(host, kind, near);
                }
                p.errand_counts[index] >= need
            }
            Task::Build(need) => u32::from(p.errand_built) >= need,
            Task::Place(_) => p.errand_flags & WHEEL != 0,
            Task::Trade => p.errand_flags & TRADED != 0,
            _ => false,
        };
        if finished { advance(host, player, p, index, current); }
    }
    let mut bar = None;
    for index in 0..COUNT {
        if !available(index, p) { continue; }
        let state = p.errands[index];
        let open = task(index, state);
        // Só brilha quem tem algo: um pedido aberto, ou alguém de quem o prefeito já contou.
        if open.is_none() && !(state == 0 && p.errand_flags & told(index) != 0) { continue; }
        let who = ERRANDS[index].who;
        let Some(spot) = at(host, who) else { continue };
        let distance = (spot[0] - player.position[0]).hypot(spot[2] - player.position[2]);
        if distance <= GLOW && now % 40 == 0 && spawn_safe(&player.name) {
            host.run_command(&format!("particle minecraft:happy_villager {:.1} {:.1} {:.1} 0.3 0.3 0.3 0 3 normal {}",
                spot[0], spot[1] + 2.3, spot[2], player.name));
        }
        if distance <= NEAR && bar.is_none() {
            let name = Text::tr(who.name_key(), &[]).plain();
            let text = match open {
                Some(current) => goal(index, current, p).plain(),
                None => Text::tr("veio.errand.meet", &[]).plain(),
            };
            bar = Some(Text::tr("veio.errand.bar", &[&name, &text]));
        }
    }
    if let Some(bar) = bar && !p.hidden { host.action_bar(player.uuid, bar); }
}

/// O caderno de pedidos (`/veiotutorial pedidos`): cada pedido, quem pede, onde e o que falta.
pub(super) fn book(host: &mut dyn Host, player: &NativeServerPlayer, p: &Progress) {
    host.tell(player.uuid, Text::tr("veio.errand.book.title", &[]));
    for (index, errand) in ERRANDS.iter().enumerate() {
        let title = Text::tr(&format!("veio.errand.{}.title", errand.key), &[]).plain();
        let name = Text::tr(errand.who.name_key(), &[]).plain();
        let line = match task(index, p.errands[index]) {
            _ if p.errands[index] == errand.done() => Text::tr("veio.errand.book.done", &[&title]),
            _ if !available(index, p) => Text::tr("veio.errand.book.later", &[&title, &name]),
            Some(current) => {
                let (distance, way) = at(host, errand.who).map_or((0, String::new()), |spot| intro::pointer(player, spot));
                Text::tr("veio.errand.book.open", &[&title, &goal(index, current, p).plain(), &name, &distance, &way])
            }
            None => {
                let (distance, way) = at(host, errand.who).map_or((0, String::new()), |spot| intro::pointer(player, spot));
                Text::tr("veio.errand.book.new", &[&title, &name, &distance, &way])
            }
        };
        host.tell(player.uuid, line);
    }
    let done = ERRANDS.iter().enumerate().filter(|(i, e)| p.errands[*i] == e.done()).count();
    let level = if done == COUNT { 3 } else if p.errands[ROOF] == ERRANDS[ROOF].done() { 2 } else { 1 };
    host.tell(player.uuid, Text::tr("veio.errand.book.level", &[&level, &done, &COUNT]));
}
