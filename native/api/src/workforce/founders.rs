//! Casal fundador: os dois moradores que garantem que a cidade nunca zera.
//!
//! - **Quem são:** dois aldeões comuns (contam no censo, trabalham e têm filhos), com a marca
//!   `veio_founder` e nome: a família do prefeito, Helena (a esposa) e Joaquim (o filho mais
//!   velho). Nascem com a cidade (`town::build`); se um deles se perder, os próximos imigrantes
//!   adultos herdam o papel.
//! - **Feridos, não mortos:** cada um segura um totem da imortalidade na mão secundária (que não
//!   cai) e o Veio confere a vida a cada `CHECK` ticks. Com a vida em `DOWN_AT` ou menos (ou
//!   depois do totem), o fundador cai **ferido e desacordado**: fica sem IA e invulnerável, é
//!   levado ao lote da prefeitura (o refúgio) e os jogadores perto são avisados.
//! - **Recuperação:** acorda sozinho depois de `RECOVER` ticks (2 min), ou na hora se um jogador
//!   der comida a ele (botão direito com comida na mão). Acorda com a vida cheia e outro totem.
//!
//! Depois de reabrir o mundo, os ids mudam: o Veio reconhece os fundadores pela marca, lendo o
//! NBT de cada aldeão uma vez a cada `SCAN` ticks, só durante `SCAN_WINDOW` ticks e só enquanto
//! faltar algum. Um fundador que ainda estava desacordado (sem IA) volta a contar como ferido.
use super::*;
use horizon_client::mod_api::{EquipmentSlot, Flow, ParticleOptions, ParticleType, Text};
use std::sync::atomic::{AtomicU64, Ordering};

pub(crate) const TAG: &str = "veio_founder";
pub(crate) const TOTEM: &str = "minecraft:totem_of_undying";
/// Vida em que o fundador cai (de 20).
pub(crate) const DOWN_AT: f32 = 6.0;
pub(crate) const HEALTH: f32 = 20.0;
/// Vida com que fica enquanto está desacordado.
const DOWN_HEALTH: f32 = 2.0;
/// Tempo desacordado sem comida: 2 min.
pub(crate) const RECOVER: u64 = 2400;
/// A vida é conferida a cada tantos ticks.
pub(crate) const CHECK: u64 = 10;
/// Reconhecimento depois de reabrir o mundo: um aldeão a cada tantos ticks...
pub(crate) const SCAN: u64 = 97;
/// ...durante este tempo depois de a cidade ser conhecida.
pub(crate) const SCAN_WINDOW: u64 = 6000;
/// Nomes: chaves de tradução, na ordem em que nascem.
const NAMES: [&str; 2] = ["veio.founder.name.0", "veio.founder.name.1"];

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Founder {
    pub id: u32,
    pub dimension: String,
    pub name: String,
    pub down_since: Option<u64>,
    pub armed: bool,
}

#[cfg(not(test))]
static FOUNDERS: Mutex<Vec<Founder>> = Mutex::new(Vec::new());
#[cfg(not(test))]
static SCAN_UNTIL: AtomicU64 = AtomicU64::new(0);

// Nos testes, cada teste (uma thread) tem os seus fundadores.
#[cfg(test)]
thread_local! {
    static FOUNDERS: &'static Mutex<Vec<Founder>> = Box::leak(Box::new(Mutex::new(Vec::new())));
    static SCAN_UNTIL: &'static AtomicU64 = Box::leak(Box::new(AtomicU64::new(0)));
}

#[cfg(not(test))]
fn founders() -> std::sync::MutexGuard<'static, Vec<Founder>> {
    FOUNDERS.lock().unwrap_or_else(|err| err.into_inner())
}

#[cfg(test)]
fn founders() -> std::sync::MutexGuard<'static, Vec<Founder>> {
    FOUNDERS.with(|list| list.lock().unwrap_or_else(|err| err.into_inner()))
}

#[cfg(not(test))]
fn scan_until() -> &'static AtomicU64 {
    &SCAN_UNTIL
}

#[cfg(test)]
fn scan_until() -> &'static AtomicU64 {
    SCAN_UNTIL.with(|until| *until)
}

/// Fundadores conhecidos nesta sessão.
pub(crate) fn count() -> usize {
    founders().len()
}

pub(crate) fn is_founder(id: u32) -> bool {
    founders().iter().any(|founder| founder.id == id)
}

#[cfg(test)]
pub(crate) fn is_down(id: u32) -> bool {
    founders().iter().any(|founder| founder.id == id && founder.down_since.is_some())
}

/// Começa (ou recomeça) a procurar os fundadores entre os aldeões.
pub(crate) fn start_scan(now: u64) {
    scan_until().store(now + SCAN_WINDOW, Ordering::Relaxed);
}

/// Faz nascer um fundador em `at`. `index` escolhe o nome (0 = Helena, 1 = Joaquim).
pub(crate) fn summon(host: &mut dyn Host, dimension: &str, at: [f64; 3], index: usize, now: u64) -> Option<u32> {
    let name = Text::tr(NAMES[index % NAMES.len()], &[]).plain();
    let nbt = Compound::from([
        ("CustomName".into(), Tag::String(name.as_str().into())),
        ("PersistenceRequired".into(), Tag::Byte(1)),
        ("Tags".into(), Tag::List(vec![Tag::String(TAG.into())])),
        ("HandDropChances".into(), Tag::List(vec![Tag::Float(0f32.to_bits()), Tag::Float(0f32.to_bits())])),
        ("VillagerData".into(), Tag::Compound(Compound::from([
            ("profession".into(), Tag::String("minecraft:none".into())),
            ("type".into(), Tag::String("minecraft:plains".into())),
            ("level".into(), Tag::Int(1)),
        ]))),
    ]);
    let id = host.summon(dimension, VILLAGER, at, nbt)?;
    {
        let mut list = founders();
        list.retain(|founder| founder.id != id);
        if list.len() < 2 {
            list.push(Founder { id, dimension: dimension.to_owned(), name, down_since: None, armed: false });
        }
    }
    arm(host, id, now);
    Some(id)
}

/// Chamado no tick de cada aldeão. Quase sempre sai na primeira comparação.
pub(crate) fn observe(host: &mut dyn Host, dimension: &str, id: u32, now: u64) {
    let tick = now.wrapping_add(u64::from(id));
    if tick.is_multiple_of(CHECK) && is_founder(id) {
        check(host, dimension, id, now);
        return;
    }
    if !tick.is_multiple_of(SCAN) || now >= scan_until().load(Ordering::Relaxed) || count() >= 2 || is_founder(id) {
        return;
    }
    let Some(handle) = host.entity(id) else { return };
    let Some(nbt) = host.entity_nbt(handle) else { return };
    if !tagged(&nbt) {
        return;
    }
    let name = match nbt.get(&"CustomName".into()) {
        Some(Tag::String(text)) => text.to_string_lossy(),
        _ => Text::tr(NAMES[count() % NAMES.len()], &[]).plain(),
    };
    let asleep = matches!(nbt.get(&"NoAI".into()), Some(Tag::Byte(1)));
    founders().push(Founder { id, dimension: dimension.to_owned(), name, down_since: asleep.then_some(now), armed: asleep });
    if !asleep {
        arm(host, id, now);
    }
}

pub(crate) fn tagged(nbt: &Compound) -> bool {
    matches!(nbt.get(&"Tags".into()), Some(Tag::List(tags))
        if tags.iter().any(|tag| matches!(tag, Tag::String(text) if text.to_string_lossy() == TAG)))
}

fn health(host: &mut dyn Host, id: u32) -> Option<f32> {
    let handle = host.entity(id)?;
    match host.entity_nbt(handle)?.get(&"Health".into())? {
        Tag::Float(bits) => Some(f32::from_bits(*bits)),
        Tag::Double(bits) => Some(f64::from_bits(*bits) as f32),
        _ => None,
    }
}

/// O que fazer com um fundador nesta conferência.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Care { Nothing, KnockDown, WakeUp, Arm }

pub(crate) fn care(health: Option<f32>, down_since: Option<u64>, armed: bool, now: u64) -> Care {
    match down_since {
        Some(since) if now.saturating_sub(since) >= RECOVER => Care::WakeUp,
        Some(_) => Care::Nothing,
        None if health.is_some_and(|life| life <= DOWN_AT) => Care::KnockDown,
        None if !armed => Care::Arm,
        None => Care::Nothing,
    }
}

fn check(host: &mut dyn Host, dimension: &str, id: u32, now: u64) {
    let Some(founder) = founders().iter().find(|founder| founder.id == id).cloned() else { return };
    let life = if founder.down_since.is_none() { health(host, id) } else { None };
    match care(life, founder.down_since, founder.armed, now) {
        Care::Nothing => {}
        Care::Arm => arm(host, id, now),
        Care::KnockDown => knock_down(host, dimension, &founder, now),
        Care::WakeUp => wake_up(host, dimension, &founder, now),
    }
}

/// Totem na mão secundária: um golpe fatal deixa o fundador com vida, e o Veio o derruba.
fn arm(host: &mut dyn Host, id: u32, _now: u64) {
    let Some(handle) = host.entity(id) else { return };
    let Some(living) = host.living(handle) else { return };
    if host.living_set_equipment(living, EquipmentSlot::OffHand, Some(ItemStack::new(TOTEM, 1))) {
        host.entity_merge_nbt(handle, Compound::from([(
            "HandDropChances".into(),
            Tag::List(vec![Tag::Float(0f32.to_bits()), Tag::Float(0f32.to_bits())]),
        )]));
        if let Some(founder) = founders().iter_mut().find(|founder| founder.id == id) {
            founder.armed = true;
        }
    }
}

fn set_down(id: u32, down: Option<u64>) {
    if let Some(founder) = founders().iter_mut().find(|founder| founder.id == id) {
        founder.down_since = down;
        if down.is_none() {
            founder.armed = false;
        }
    }
}

/// Formato de UUID que os comandos aceitam, a partir dos quatro inteiros do NBT.
pub(crate) fn uuid_text(ints: [i32; 4]) -> String {
    let n = ints.iter().fold(0u128, |acc, part| (acc << 32) | u128::from(*part as u32));
    let hex = format!("{n:032x}");
    format!("{}-{}-{}-{}-{}", &hex[0..8], &hex[8..12], &hex[12..16], &hex[16..20], &hex[20..32])
}

fn uuid(host: &mut dyn Host, id: u32) -> Option<[i32; 4]> {
    let handle = host.entity(id)?;
    match host.entity_nbt(handle)?.get(&"UUID".into())? {
        Tag::IntArray(values) if values.len() == 4 => Some([values[0], values[1], values[2], values[3]]),
        _ => None,
    }
}

fn knock_down(host: &mut dyn Host, dimension: &str, founder: &Founder, now: u64) {
    let Some(handle) = host.entity(founder.id) else { return };
    host.entity_merge_nbt(handle, Compound::from([
        ("NoAI".into(), Tag::Byte(1)),
        ("Invulnerable".into(), Tag::Byte(1)),
    ]));
    host.set_mob_health(dimension, founder.id, DOWN_HEALTH);
    set_down(founder.id, Some(now));
    let refuge = town::refuge();
    if let (Some([x, y, z]), Some(ints)) = (refuge, uuid(host, founder.id)) {
        host.run_command(&format!("execute in {dimension} run tp {} {x}.5 {y} {z}.5", uuid_text(ints)));
    }
    let at = refuge.map(|[x, y, z]| [f64::from(x) + 0.5, f64::from(y), f64::from(z) + 0.5])
        .or_else(|| host.entity(founder.id).and_then(|handle| handle.position(host)));
    if let Some(at) = at {
        host.send_particles(&ParticleOptions::simple(ParticleType::Cloud), None, [at[0], at[1] + 1.0, at[2]],
            [0.3, 0.3, 0.3], 0.0, 6);
        tell_near(host, dimension, at, Text::tr("veio.founder.down", &[&founder.name]));
    }
}

fn wake_up(host: &mut dyn Host, dimension: &str, founder: &Founder, now: u64) {
    let Some(handle) = host.entity(founder.id) else { return };
    host.entity_merge_nbt(handle, Compound::from([
        ("NoAI".into(), Tag::Byte(0)),
        ("Invulnerable".into(), Tag::Byte(0)),
    ]));
    host.set_mob_health(dimension, founder.id, HEALTH);
    set_down(founder.id, None);
    arm(host, founder.id, now);
    if let Some(at) = host.entity(founder.id).and_then(|handle| handle.position(host)) {
        host.send_particles(&ParticleOptions::simple(ParticleType::HappyVillager), None, [at[0], at[1] + 1.0, at[2]],
            [0.3, 0.4, 0.3], 0.0, 10);
        tell_near(host, dimension, at, Text::tr("veio.founder.up", &[&founder.name]));
    }
}

fn tell_near(host: &mut dyn Host, dimension: &str, at: [f64; 3], line: Text) {
    let near: Vec<u128> = host.server_players().into_iter()
        .filter(|player| player.dimension == dimension
            && (player.position[0] - at[0]).abs() <= 96.0 && (player.position[2] - at[2]).abs() <= 96.0)
        .map(|player| player.uuid)
        .collect();
    for player in near {
        host.tell(player, line.clone());
    }
}

/// Clique num fundador. Desacordado: com comida na mão, ele acorda na hora (a comida é gasta);
/// sem comida, o jogador ouve como ajudar. `None` deixa o clique seguir (contratar, trocar).
pub(crate) fn interact(host: &mut dyn Host, id: u32, held: Option<&str>) -> Option<Flow> {
    let founder = founders().iter().find(|founder| founder.id == id && founder.down_since.is_some()).cloned()?;
    let player = host.acting_player();
    match held.filter(|item| is_food(item)) {
        Some(item) if host.take_items(item, 1) == 1 => {
            let dimension = founder.dimension.clone();
            wake_up(host, &dimension, &founder, defense::clock());
        }
        _ => {
            if let Some(player) = player {
                host.tell(player.uuid, Text::tr("veio.founder.resting", &[&founder.name]));
            }
        }
    }
    Some(Flow::Cancel)
}

#[cfg(test)]
pub(crate) fn reset() {
    founders().clear();
    scan_until().store(0, Ordering::Relaxed);
}

#[cfg(test)]
pub(crate) fn known() -> Vec<Founder> {
    founders().clone()
}
