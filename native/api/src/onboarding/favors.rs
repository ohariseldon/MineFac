//! O primeiro ato começa com favores, não com tudo pronto: o prefeito monta o gerador, mas a
//! prensa está com o **Seu Bento**, o ferreiro, que tem uma sobrando. Ele só larga dela se você
//! ajudar a **Dona Cida**, a lavradora: ela não dá conta de plantar e o pão da vila está caro.
//! Você planta um campo de trigo ao lado da casa dela, colhe e entrega; ela passa a vender pão
//! mais barato (a troca dela melhora) e o Seu Bento cumpre a palavra: prensa, cabos e as receitas.
//!
//! O favor ocupa a etapa 2 ("Ligar uma máquina") até a prensa chegar. Os dois vizinhos são
//! aldeões parados com nome (`veio:smith`, `veio:farmer`, `tools/build_content.py`), em lugares
//! fixos a partir do centro da vila, para a seta achá-los mesmo com o chunk descarregado.
use super::*;
use workforce::spawn::OVERWORLD;
use horizon_client::mod_api::NativeHeightmap;

pub(crate) const SMITH: &str = "veio:smith";
pub(crate) const FARMER: &str = "veio:farmer";

/// Etapas do favor (`Progress::favor`).
pub(super) const NOT_STARTED: u8 = 0;
pub(super) const ASK_SMITH: u8 = 1;
pub(super) const ASK_FARMER: u8 = 2;
pub(super) const PLANT: u8 = 3;
pub(super) const HARVEST: u8 = 4;
pub(super) const BACK_TO_SMITH: u8 = 5;
pub(super) const FAVOR_DONE: u8 = 6;
/// Pés de trigo plantados perto da Dona Cida para o campo contar.
pub(super) const CROPS: u32 = 12;
/// Trigo colhido que ela pede.
pub(super) const WHEAT: u32 = 12;
/// Até onde da Dona Cida o trigo conta como o campo dela.
const FIELD: i32 = 10;
/// Cabos que o Seu Bento entrega junto com a prensa.
const CABLES: u32 = 16;
/// Perto disto a barra troca a seta pelo pedido.
const NEAR: f64 = 6.0;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum Who { Smith, Farmer }

impl Who {
    fn kind(self) -> &'static str { match self { Who::Smith => SMITH, Who::Farmer => FARMER } }
    fn key(self) -> &'static str { match self { Who::Smith => "smith", Who::Farmer => "farmer" } }
    /// Onde cada um mora, a partir do centro da vila: o ferreiro a leste, a lavradora a oeste.
    fn offsets(self) -> [(i32, i32); 4] {
        match self {
            Who::Smith => [(8, 0), (8, 3), (8, -3), (7, 6)],
            Who::Farmer => [(-8, 0), (-8, 3), (-8, -3), (-7, -6)],
        }
    }
    fn villager(self) -> (&'static str, &'static str) {
        match self { Who::Smith => ("minecraft:toolsmith", "minecraft:plains"), Who::Farmer => ("minecraft:farmer", "minecraft:savanna") }
    }
}

/// Ids das entidades no mundo aberto (vistos no tick delas ou ao invocar).
static IDS: Mutex<[Option<u32>; 2]> = Mutex::new([None, None]);

fn slot(who: Who) -> usize { match who { Who::Smith => 0, Who::Farmer => 1 } }

pub(super) fn who(kind: &str) -> Option<Who> {
    match kind { SMITH => Some(Who::Smith), FARMER => Some(Who::Farmer), _ => None }
}

/// Tick de um vizinho: guarda o id e tira cópias a mais.
pub(crate) fn seen(host: &mut dyn Host, dimension: &str, kind: &str, id: u32) {
    let Some(who) = who(kind) else { return };
    let Ok(mut ids) = IDS.lock() else { return };
    match ids[slot(who)] {
        Some(other) if other != id && host.entity(other).is_some() => { host.remove_mob(dimension, id); }
        _ => ids[slot(who)] = Some(id),
    }
}

pub(super) fn reset() { if let Ok(mut ids) = IDS.lock() { *ids = [None, None]; } }

/// O lugar fixo de cada um: o chão mais baixo e seco entre as colunas dele.
fn home(host: &dyn Host, who: Who) -> Option<[f64; 3]> {
    let ([x, _, z], _) = workforce::spawn::center()?;
    let columns: Vec<(i32, i32)> = who.offsets().iter().map(|(dx, dz)| (x + dx, z + dz)).collect();
    let ground = host.surface_heights(OVERWORLD, &columns, NativeHeightmap::MotionBlockingNoLeaves).ok()?;
    let floor = host.surface_heights(OVERWORLD, &columns, NativeHeightmap::OceanFloor).ok()?;
    columns.iter().zip(ground.iter().zip(&floor))
        .filter_map(|(col, (g, f))| (g.is_some() && g == f).then(|| (*col, g.unwrap())))
        .min_by_key(|(_, height)| *height)
        .map(|((cx, cz), y)| [f64::from(cx) + 0.5, f64::from(y), f64::from(cz) + 0.5])
}

/// Onde a pessoa está agora: a entidade, ou o lugar fixo dela.
pub(super) fn at(host: &dyn Host, who: Who) -> Option<[f64; 3]> {
    let id = IDS.lock().ok().and_then(|ids| ids[slot(who)]);
    id.and_then(|id| host.entity(id)).and_then(|entity| entity.position(host)).or_else(|| home(host, who))
}

/// Os dois chegam à vila quando o favor começa (e ficam depois dele).
fn summon(host: &mut dyn Host, player: &NativeServerPlayer, who: Who) {
    if player.dimension != OVERWORLD { return; }
    let id = IDS.lock().ok().and_then(|ids| ids[slot(who)]);
    if id.and_then(|id| host.entity(id)).is_some() { return; }
    if host.run_command(&format!("execute if entity @e[type={}]", who.kind())) { return; }
    let Some(spot) = home(host, who) else { return };
    let name = Text::tr(&format!("veio.favor.{}.name", who.key()), &[]).plain();
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
            ("level".into(), Tag::Int(2)),
        ]))),
    ]);
    if let Some(id) = host.summon(OVERWORLD, who.kind(), spot, nbt) {
        if let Ok(mut ids) = IDS.lock() { ids[slot(who)] = Some(id); }
    }
}

/// Uma fala do vizinho no chat (o nome dele e a frase).
fn say(host: &mut dyn Host, player: u128, who: Who, key: &str, args: &[&dyn std::fmt::Display]) {
    let name = Text::tr(&format!("veio.favor.{}.name", who.key()), &[]).plain();
    host.tell(player, Text::tr("veio.agenda.says", &[&name, &Text::tr(key, args).plain()]));
}

fn react(host: &mut dyn Host, who: Who, happy: bool) {
    let (particle, sound) = if happy { ("minecraft:heart", "minecraft:entity.villager.celebrate") }
        else { ("minecraft:angry_villager", "minecraft:entity.villager.no") };
    let kind = who.kind();
    host.run_command(&format!("execute at @e[type={kind},limit=1] run particle {particle} ~ ~2.1 ~ 0.3 0.2 0.3 0 5"));
    host.run_command(&format!("execute at @e[type={kind},limit=1] run playsound {sound} neutral @a[distance=..16] ~ ~ ~"));
}

/// Pés de trigo (de qualquer idade) perto da Dona Cida.
pub(super) fn crops(host: &dyn Host, near: [f64; 3]) -> u32 {
    let [cx, cy, cz] = near.map(|v| v.floor() as i32);
    let mut count = 0;
    for dx in -FIELD..=FIELD { for dz in -FIELD..=FIELD { for dy in -3..=2 {
        if host.block([cx + dx, cy + dy, cz + dz]) == "minecraft:wheat" { count += 1; }
    } } }
    count
}

/// Quem o favor manda procurar agora.
fn wanted(favor: u8) -> Option<Who> {
    match favor {
        ASK_SMITH | BACK_TO_SMITH => Some(Who::Smith),
        ASK_FARMER | PLANT | HARVEST => Some(Who::Farmer),
        _ => None,
    }
}

/// A cada segundo, na etapa 2 antes da prensa: o prefeito manda ao ferreiro, a barra aponta a
/// pessoa e conta o campo.
pub(super) fn tick(host: &mut dyn Host, player: &NativeServerPlayer, p: &mut Progress, now: u64) {
    if p.favor == NOT_STARTED {
        for line in 1..=2 { intro::say(host, player.uuid, &format!("veio.favor.start.{line}"), &[]); }
        p.favor = ASK_SMITH; p.dirty = true;
    }
    summon(host, player, Who::Smith);
    if p.favor >= ASK_FARMER { summon(host, player, Who::Farmer); }
    if p.favor == PLANT && now % 100 == 0 && let Some(near) = at(host, Who::Farmer) {
        p.favor_crops = crops(host, near);
        if p.favor_crops >= CROPS {
            say(host, player.uuid, Who::Farmer, "veio.favor.farmer.planted", &[&WHEAT]);
            react(host, Who::Farmer, true);
            p.favor = HARVEST; p.dirty = true;
        }
    }
    let Some(who) = wanted(p.favor) else { return };
    let goal = Text::tr(&format!("veio.favor.goal.{}", p.favor), &[&p.favor_crops.min(CROPS)]);
    let bar = match at(host, who).map(|spot| intro::pointer(player, spot)) {
        Some((distance, way)) if f64::from(distance) > NEAR => Text::tr("veio.onboarding.far", &[&goal.plain(), &way, &distance]),
        _ => goal,
    };
    host.action_bar(player.uuid, Text::tr("veio.favor.bar", &[&bar.plain()]));
}

/// Clique no ferreiro ou na lavradora. `Some(true)`: o Veio tomou o clique (conversa);
/// `Some(false)`: segue a troca vanilla (a Dona Cida vendendo pão depois do favor).
pub(super) fn talk(host: &mut dyn Host, player: &NativeServerPlayer, p: &mut Progress, who: Who) -> bool {
    let uuid = player.uuid;
    host.run_command(&format!("execute at @e[type={},limit=1] run playsound minecraft:entity.villager.ambient neutral @a[distance=..16] ~ ~ ~", who.kind()));
    match (who, p.favor) {
        (Who::Smith, NOT_STARTED | ASK_SMITH) => {
            for line in 1..=4 { say(host, uuid, who, &format!("veio.favor.smith.ask.{line}"), &[]); }
            p.favor = ASK_FARMER; p.dirty = true;
            summon(host, player, Who::Farmer);
        }
        (Who::Farmer, ASK_FARMER) => {
            for line in 1..=3 { say(host, uuid, who, &format!("veio.favor.farmer.ask.{line}"), &[&CROPS]); }
            host.give_items("minecraft:wheat_seeds", 16);
            host.give_items("minecraft:bone_meal", 32);
            host.give_items("minecraft:stone_hoe", 1);
            host.tell(uuid, Text::tr("veio.favor.farmer.gift", &[]));
            p.favor = PLANT; p.dirty = true;
        }
        (Who::Farmer, PLANT) => say(host, uuid, who, "veio.favor.farmer.plant", &[&p.favor_crops.min(CROPS), &CROPS]),
        (Who::Farmer, HARVEST) => {
            let got = host.take_items("minecraft:wheat", WHEAT);
            if got < WHEAT {
                if got > 0 { host.give_items("minecraft:wheat", got); }
                say(host, uuid, who, "veio.favor.farmer.more", &[&(WHEAT - got)]);
                react(host, who, false);
                return true;
            }
            for line in 1..=3 { say(host, uuid, who, &format!("veio.favor.farmer.thanks.{line}"), &[]); }
            react(host, who, true);
            cheaper_bread(host);
            p.favor = BACK_TO_SMITH; p.dirty = true;
        }
        (Who::Smith, BACK_TO_SMITH) => {
            for line in 1..=3 { say(host, uuid, who, &format!("veio.favor.smith.thanks.{line}"), &[]); }
            react(host, who, true);
            host.give_items(PRESS, 1);
            host.give_items(CABLE, CABLES);
            if workforce::spawn::safe_name(&player.name) {
                for recipe in [PRESS, CABLE] { host.run_command(&format!("recipe give {} {recipe}", player.name)); }
                host.run_command(&format!("xp add {} 30 points", player.name));
                host.run_command(&format!(r#"title {} subtitle {{"translate":"veio.favor.done.sub"}}"#, player.name));
                host.run_command(&format!(r#"title {} title {{"translate":"veio.favor.done","color":"gold"}}"#, player.name));
                host.run_command(&format!("execute as {} at @s run playsound minecraft:ui.toast.challenge_complete master @s", player.name));
            }
            p.favor = FAVOR_DONE; p.dirty = true;
        }
        // Depois do favor, a Dona Cida vende pão (a troca vanilla abre) e o Seu Bento comenta.
        (Who::Farmer, FAVOR_DONE) => { say(host, uuid, who, "veio.favor.farmer.after", &[]); return false; }
        (Who::Smith, FAVOR_DONE) => say(host, uuid, who, "veio.favor.smith.after", &[]),
        (Who::Farmer, _) => say(host, uuid, who, "veio.favor.farmer.wait", &[]),
        (Who::Smith, _) => say(host, uuid, who, "veio.favor.smith.wait", &[]),
    }
    true
}

/// Com o trigo do campo novo, a Dona Cida vende pão por menos: 6 por esmeralda (antes eram 2).
fn cheaper_bread(host: &mut dyn Host) {
    host.run_command(&format!(concat!("execute as @e[type={},limit=1] run data merge entity @s ",
        r#"{{Offers:{{Recipes:[{{buy:{{id:"minecraft:emerald",count:1}},sell:{{id:"minecraft:bread",count:6}},"#,
        r#"maxUses:16,rewardExp:0b,xp:1}}]}}}}"#), FARMER));
}
