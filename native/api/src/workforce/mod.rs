//! Trabalhadores: aldeões que o jogador contrata para um posto de trabalho.
//!
//! Conceitos (veja `frentes/CLOUD.md`):
//! - **Trabalhador**: um aldeão adulto, identificado pelo UUID salvo no NBT do posto. O id de
//!   entidade (`u32`) só vale durante a sessão e fica num registro em memória.
//! - **Posto**: bloco que guarda a atribuição, a comida do trabalhador e o que ele produz.
//! - **Tarefa**: um lugar onde o trabalhador precisa estar e o que acontece quando chega.
//!   Cada tipo de posto define as suas em um submódulo (`farmhand.rs` é o primeiro).
//! - **Atribuição**: exclusiva. Um aldeão pertence a no máximo um posto; o registro `CLAIMS`
//!   recusa a segunda reivindicação enquanto a primeira continuar sendo renovada.
//!
//! O trabalho é físico: o posto manda o aldeão andar (objetivo `Work`, registrado nos aldeões)
//! e só colhe quando ele está ao lado da planta. O produto viaja com ele até o posto. Aldeões
//! sem ordem recente seguem o comportamento vanilla, inclusive comércio.
use super::*;
use horizon_client::mobs::Mob;
use horizon_client::mobs::goals::{self, Cx, Goal};
use horizon_client::mod_api::{ParticleOptions, ParticleType, Text};
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;

pub(super) mod farmhand;

pub(super) const FARM_POST: &str = "veio:farm_post";
pub(super) const VILLAGER: &str = "minecraft:villager";
/// Contratar custa uma esmeralda, paga só quando a contratação dá certo.
pub(super) const WAGE: &str = "minecraft:emerald";

// Estados do painel, depois de `production` (14-17).
pub(super) const NO_WORKER: i32 = 18;
pub(super) const ABSENT: i32 = 19;
pub(super) const NO_TASK: i32 = 20;
pub(super) const NO_FOOD: i32 = 21;
pub(super) const PATH: i32 = 22;
pub(super) const WALKING: i32 = 23;
/// Resposta de uma tarefa que ainda está procurando: o painel mantém o estado anterior.
pub(super) const KEEP: i32 = -1;

/// Pontos de comida gastos em cada colheita. Trigo cru vale 1 e não paga o ciclo;
/// pão vale 6 e paga três colheitas.
pub(super) const TASK_COST: i32 = 2;
/// O posto age a cada 10 ticks, com a fase espalhada pela posição.
const EVERY: i64 = 10;
/// Aldeões a até esta distância do posto podem ser contratados por ele.
pub(super) const HIRE_RADIUS: f64 = 8.0;
/// Procura o trabalhador nesta distância quando o id da sessão não é conhecido.
const SEARCH_RADIUS: f64 = 32.0;
/// NBT de aldeões lidos por ciclo ao procurar o trabalhador.
const PROBES: usize = 4;
/// Ticks de caminhada antes de desistir de um alvo: caminho obstruído.
pub(super) const WALK_LIMIT: i32 = 200;
/// Ticks sem encontrar o trabalhador antes de desfazer a atribuição (morto ou removido).
pub(super) const ABSENT_LIMIT: i32 = 6000;
/// Distância horizontal para trabalhar numa planta e para entregar no posto.
pub(super) const REACH: f64 = 1.8;
pub(super) const POST_REACH: f64 = 2.5;
/// Uma reivindicação que o posto não renova nesse tempo perde a validade (posto removido ou
/// descarregado). Medido no relógio do mod (`defense::clock`).
const CLAIM_STALE: u64 = 400;
/// Uma ordem de caminhada vale por esse tempo; o posto a renova a cada ciclo.
const ORDER_FRESH: u64 = 60;

pub(super) type Uuid = [i32; 4];

pub(super) fn is_post(block: &str) -> bool {
    block == FARM_POST
}

pub(super) fn layout(block: &str) -> Option<(usize, Vec<usize>, Vec<usize>)> {
    // Slot 0: comida do trabalhador. Slots 1-3: produção entregue.
    is_post(block).then(|| (4, vec![0], vec![1, 2, 3]))
}

pub(super) fn outputs(block: &str) -> std::ops::Range<usize> {
    if is_post(block) { 1..4 } else { 0..0 }
}

pub(super) fn accepts(block: &str, slot: usize, stack: &ItemStack) -> bool {
    is_post(block) && slot == 0 && stack.extra.is_none() && production::pantry::value(&stack.item).is_some()
}

// ---------- Registro de atribuições ----------

struct Claim {
    dimension: String,
    post: [i32; 3],
    entity: Option<u32>,
    seen: u64,
}

static CLAIMS: Mutex<Vec<(Uuid, Claim)>> = Mutex::new(Vec::new());

/// O aldeão já trabalha para outro posto vivo?
pub(super) fn claimed_elsewhere(uuid: Uuid, dimension: &str, post: [i32; 3], now: u64) -> bool {
    let Ok(claims) = CLAIMS.lock() else { return true };
    claims.iter().any(|(id, claim)| *id == uuid
        && !(claim.dimension == dimension && claim.post == post)
        && now.saturating_sub(claim.seen) <= CLAIM_STALE)
}

/// Reivindica ou renova o aldeão para este posto. `false` se outro posto vivo já o tem.
pub(super) fn claim(uuid: Uuid, dimension: &str, post: [i32; 3], now: u64) -> bool {
    let Ok(mut claims) = CLAIMS.lock() else { return false };
    if let Some((_, claim)) = claims.iter_mut().find(|(id, _)| *id == uuid) {
        let mine = claim.dimension == dimension && claim.post == post;
        if !mine && now.saturating_sub(claim.seen) <= CLAIM_STALE {
            return false;
        }
        if !mine {
            *claim = Claim { dimension: dimension.to_owned(), post, entity: None, seen: now };
        }
        claim.seen = now;
        return true;
    }
    // Limpa reivindicações vencidas antes de crescer.
    claims.retain(|(_, claim)| now.saturating_sub(claim.seen) <= CLAIM_STALE);
    claims.push((uuid, Claim { dimension: dimension.to_owned(), post, entity: None, seen: now }));
    true
}

/// Libera o aldeão, se ele pertencer a este posto.
pub(super) fn release(uuid: Uuid, dimension: &str, post: [i32; 3]) {
    if let Ok(mut claims) = CLAIMS.lock() {
        claims.retain(|(id, claim)| !(*id == uuid && claim.dimension == dimension && claim.post == post));
    }
}

fn cached_entity(uuid: Uuid) -> Option<u32> {
    CLAIMS.lock().ok()?.iter().find(|(id, _)| *id == uuid).and_then(|(_, claim)| claim.entity)
}

fn remember_entity(uuid: Uuid, entity: Option<u32>) {
    if let Ok(mut claims) = CLAIMS.lock()
        && let Some((_, claim)) = claims.iter_mut().find(|(id, _)| *id == uuid)
    {
        claim.entity = entity;
    }
}

// ---------- Ordens de caminhada ----------

static ORDERS: Mutex<Option<HashMap<u32, ([f64; 3], u64)>>> = Mutex::new(None);
/// Atalho para o objetivo `Work`: aldeões ociosos não disputam o mutex quando nenhum
/// posto tem ordem viva.
static ORDERS_PENDING: AtomicBool = AtomicBool::new(false);

/// Manda o aldeão `entity` andar até `target`.
pub(super) fn order(entity: u32, target: [f64; 3], now: u64) {
    if let Ok(mut orders) = ORDERS.lock() {
        let orders = orders.get_or_insert_with(HashMap::new);
        if orders.len() >= 1024 && !orders.contains_key(&entity) {
            orders.retain(|_, (_, issued)| now.saturating_sub(*issued) <= ORDER_FRESH);
        }
        orders.insert(entity, (target, now));
        ORDERS_PENDING.store(true, Ordering::Release);
    }
}

/// Retira a ordem: o aldeão volta ao comportamento vanilla.
pub(super) fn cancel(entity: u32) {
    if let Ok(mut orders) = ORDERS.lock()
        && let Some(orders) = orders.as_mut()
    {
        orders.remove(&entity);
        if orders.is_empty() {
            ORDERS_PENDING.store(false, Ordering::Release);
        }
    }
}

/// Destino atual do aldeão, se o posto renovou a ordem há pouco.
pub(super) fn order_for(entity: u32, now: u64) -> Option<[f64; 3]> {
    if !ORDERS_PENDING.load(Ordering::Acquire) {
        return None;
    }
    let orders = ORDERS.lock().ok()?;
    let (target, issued) = orders.as_ref()?.get(&entity)?;
    (now.saturating_sub(*issued) <= ORDER_FRESH).then_some(*target)
}

/// Objetivo dos aldeões: obedecer à ordem do posto. Sem ordem, não faz nada e o cérebro
/// vanilla continua no comando.
#[derive(Clone, Debug, Default)]
pub(super) struct Work {
    target: Option<[f64; 3]>,
    cooldown: i32,
}

impl Goal for Work {
    fn flags(&self) -> u8 {
        goals::MOVE | goals::LOOK
    }
    fn can_use(&mut self, m: &mut Mob, _cx: &mut Cx) -> bool {
        self.target = order_for(m.id, defense::clock());
        self.target.is_some()
    }
    fn start(&mut self, m: &mut Mob, cx: &mut Cx) {
        self.cooldown = 0;
        self.tick(m, cx);
    }
    fn tick(&mut self, m: &mut Mob, cx: &mut Cx) {
        let Some(goal) = self.target else { return };
        self.cooldown -= 1;
        let close = (m.position.x - goal[0]).hypot(m.position.z - goal[2]) < REACH * 0.8
            && (m.position.y - goal[1]).abs() < 2.0;
        // A ordem pode mudar de alvo; refaz o caminho quando ele termina ou a cada segundo.
        if !close && (m.ai.nav.is_done() || self.cooldown <= 0) {
            m.nav_move_to(cx.w, goal, 0.6);
            self.cooldown = 20;
        }
    }
    fn box_clone(&self) -> Box<dyn Goal> {
        Box::new(self.clone())
    }
}

pub(super) fn register_goals(mob: &mut Mob) {
    if mob.kind.id == VILLAGER {
        mob.ai.goals.add(1, Work::default());
    }
}

// ---------- NBT do posto ----------

pub(super) fn worker(entity: &ModBlockEntity) -> Option<Uuid> {
    match entity.nbt.get(&"Worker".into()) {
        Some(Tag::IntArray(values)) if values.len() == 4 => Some([values[0], values[1], values[2], values[3]]),
        _ => None,
    }
}

fn set_worker(entity: &mut ModBlockEntity, uuid: Option<Uuid>) {
    match uuid {
        Some(uuid) => { entity.nbt.insert("Worker".into(), Tag::IntArray(uuid.to_vec())); }
        None => { entity.nbt.remove(&"Worker".into()); }
    }
    for key in ["Target", "Walk", "Absent"] {
        entity.nbt.remove(&key.into());
    }
}

pub(super) fn target(entity: &ModBlockEntity) -> Option<[i32; 3]> {
    match entity.nbt.get(&"Target".into()) {
        Some(Tag::IntArray(values)) if values.len() == 3 => Some([values[0], values[1], values[2]]),
        _ => None,
    }
}

pub(super) fn set_target(entity: &mut ModBlockEntity, pos: Option<[i32; 3]>) {
    match pos {
        Some(pos) => { entity.nbt.insert("Target".into(), Tag::IntArray(pos.to_vec())); }
        None => { entity.nbt.remove(&"Target".into()); }
    }
    entity.nbt.remove(&"Walk".into());
}

/// Há carga a entregar? Evita montar o `Vec` de `Carried` em todo ciclo ocioso.
fn has_load(entity: &ModBlockEntity) -> bool {
    matches!(entity.nbt.get(&"Carried".into()), Some(Tag::List(list)) if !list.is_empty())
}

/// Itens que o trabalhador carrega do campo até o posto. Ficam no NBT do posto: salvar,
/// descarregar o chunk ou trocar de trabalhador não os perde.
pub(super) fn carried(entity: &ModBlockEntity) -> Vec<(String, u32)> {
    let Some(Tag::List(list)) = entity.nbt.get(&"Carried".into()) else { return Vec::new() };
    list.iter().filter_map(|tag| {
        let Tag::Compound(item) = tag else { return None };
        let Some(Tag::String(id)) = item.get(&"id".into()) else { return None };
        let Some(Tag::Int(count)) = item.get(&"count".into()) else { return None };
        (*count > 0).then(|| (id.to_string_lossy(), *count as u32))
    }).collect()
}

pub(super) fn set_carried(entity: &mut ModBlockEntity, items: &[(String, u32)]) {
    if items.is_empty() {
        entity.nbt.remove(&"Carried".into());
        return;
    }
    let list = items.iter().map(|(id, count)| Tag::Compound(Compound::from([
        ("id".into(), Tag::String(id.as_str().into())),
        ("count".into(), Tag::Int(*count as i32)),
    ]))).collect();
    entity.nbt.insert("Carried".into(), Tag::List(list));
}

/// Entrega o que foi carregado nas saídas, se tudo couber.
pub(super) fn deliver(entity: &mut ModBlockEntity) -> bool {
    let load = carried(entity);
    if load.is_empty() {
        return true;
    }
    let range = outputs(&entity.id);
    let drops: Vec<(&str, u32)> = load.iter().map(|(id, count)| (id.as_str(), *count)).collect();
    if !farm::drops_fit(&entity.items[range.clone()], &drops) {
        return false;
    }
    for (id, count) in &drops {
        farm::store(&mut entity.items[range.clone()], id, *count);
    }
    set_carried(entity, &[]);
    set_number(entity, "Produced", number(entity, "Produced").saturating_add(1));
    true
}

fn uuid_of(nbt: &Compound) -> Option<Uuid> {
    match nbt.get(&"UUID".into()) {
        Some(Tag::IntArray(values)) if values.len() == 4 => Some([values[0], values[1], values[2], values[3]]),
        _ => None,
    }
}

fn adult(nbt: &Compound) -> bool {
    !matches!(nbt.get(&"Age".into()), Some(Tag::Int(age)) if *age < 0)
}

fn post_center([x, y, z]: [i32; 3]) -> [f64; 3] {
    [f64::from(x) + 0.5, f64::from(y), f64::from(z) + 0.5]
}

pub(super) fn near(at: [f64; 3], goal: [f64; 3], reach: f64) -> bool {
    (at[0] - goal[0]).hypot(at[2] - goal[2]) <= reach && (at[1] - goal[1]).abs() <= 2.0
}

// ---------- Contratar e dispensar ----------

/// Aldeão adulto e livre mais próximo do posto: (UUID, id na sessão).
pub(super) fn candidate(host: &mut dyn Host, pos: [i32; 3], now: u64) -> Option<(Uuid, u32)> {
    let dimension = host.dimension();
    let center = post_center(pos);
    let mut nearby: Vec<(f64, u32)> = production::ranch::recent_villagers(dimension).into_iter()
        .filter_map(|id| {
            let at = host.entity(id)?.position(host)?;
            near(at, center, HIRE_RADIUS).then(|| ((at[0] - center[0]).hypot(at[2] - center[2]), id))
        })
        .collect();
    nearby.sort_by(|a, b| a.0.total_cmp(&b.0));
    nearby.into_iter().find_map(|(_, id)| {
        let nbt = host.entity_nbt(host.entity(id)?)?;
        let uuid = uuid_of(&nbt)?;
        (adult(&nbt) && !claimed_elsewhere(uuid, dimension, pos, now)).then_some((uuid, id))
    })
}

/// Atribui o aldeão ao posto. `false` se outro posto o reivindicou nesse meio-tempo.
pub(super) fn assign(dimension: &str, pos: [i32; 3], entity: &mut ModBlockEntity, (uuid, id): (Uuid, u32), now: u64) -> bool {
    if worker(entity).is_some() || !claim(uuid, dimension, pos, now) {
        return false;
    }
    remember_entity(uuid, Some(id));
    set_worker(entity, Some(uuid));
    set_number(entity, "Status", WALKING);
    true
}

/// Dispensa o trabalhador: a ordem é retirada e o aldeão volta ao comportamento vanilla.
/// O que ele carregava entra no posto (agora ou quando houver espaço).
pub(super) fn dismiss(dimension: &str, pos: [i32; 3], entity: &mut ModBlockEntity) -> bool {
    let Some(uuid) = worker(entity) else { return false };
    if let Some(id) = cached_entity(uuid) {
        cancel(id);
    }
    release(uuid, dimension, pos);
    set_worker(entity, None);
    deliver(entity);
    set_number(entity, "Status", NO_WORKER);
    true
}

/// Clique com esmeralda contrata; agachado com a chave, dispensa.
pub(super) fn right_click(host: &mut dyn Host, pos: [i32; 3], held: &str) -> Option<Flow> {
    let wrench = held == WRENCH && host.sneaking();
    if held != WAGE && !wrench {
        return None;
    }
    let dimension = host.dimension();
    let now = defense::clock();
    let mut staffed = None;
    host.change_mod_block_entity(pos, &mut |entity| {
        if is_post(&entity.id) {
            staffed = Some(worker(entity).is_some());
        }
    });
    let staffed = staffed?;
    if wrench {
        if !staffed {
            return None;
        }
        host.change_mod_block_entity(pos, &mut |entity| { dismiss(dimension, pos, entity); });
        notify(host, pos, "veio.worker.dismissed");
        host.play_sound("minecraft:entity.villager.no", center(pos), 4, 0.6, 1.0);
        return Some(Flow::Cancel);
    }
    if staffed {
        notify(host, pos, "veio.worker.staffed");
        return Some(Flow::Cancel);
    }
    let Some(found) = candidate(host, pos, now) else {
        notify(host, pos, "veio.worker.none");
        return Some(Flow::Cancel);
    };
    if host.take_items(WAGE, 1) == 0 {
        return Some(Flow::Cancel);
    }
    let mut hired = false;
    host.change_mod_block_entity(pos, &mut |entity| hired = assign(dimension, pos, entity, found, now));
    if !hired {
        // Ninguém foi contratado: devolve a esmeralda.
        host.give_items(WAGE, 1);
        notify(host, pos, "veio.worker.none");
        return Some(Flow::Cancel);
    }
    notify(host, pos, "veio.worker.hired");
    host.play_sound("minecraft:entity.villager.yes", center(pos), 4, 0.8, 1.0);
    host.send_particles(&ParticleOptions::simple(ParticleType::HappyVillager), None,
        [f64::from(pos[0]) + 0.5, f64::from(pos[1]) + 1.2, f64::from(pos[2]) + 0.5], [0.3, 0.3, 0.3], 0.0, 8);
    Some(Flow::Cancel)
}

/// Mensagem na barra de ação do jogador mais próximo do posto.
fn notify(host: &mut dyn Host, pos: [i32; 3], key: &str) {
    let at = center(pos);
    let dimension = host.dimension();
    let nearest = host.server_players().into_iter()
        .filter(|player| player.dimension == dimension)
        .map(|player| ((0..3).map(|axis| (player.position[axis] - at[axis]).powi(2)).sum::<f64>(), player.uuid))
        .filter(|(distance, _)| *distance < 12.0 * 12.0)
        .min_by(|a, b| a.0.total_cmp(&b.0));
    if let Some((_, uuid)) = nearest {
        host.action_bar(uuid, Text::tr(key, &[]));
    }
}

// ---------- Ciclo do posto ----------

/// Encontra o trabalhador na sessão: pelo registro ou lendo o UUID dos aldeões perto do posto.
fn locate(host: &mut dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity, uuid: Uuid) -> Option<(u32, [f64; 3])> {
    if let Some(id) = cached_entity(uuid) {
        if let Some(at) = host.entity(id).and_then(|handle| handle.position(host)) {
            return Some((id, at));
        }
        remember_entity(uuid, None);
    }
    let center = post_center(pos);
    let nearby: Vec<u32> = production::ranch::recent_villagers(host.dimension()).into_iter()
        .filter(|id| host.entity(*id).and_then(|handle| handle.position(host))
            .is_some_and(|at| (at[0] - center[0]).hypot(at[2] - center[2]) <= SEARCH_RADIUS
                && (at[1] - center[1]).abs() <= SEARCH_RADIUS))
        .collect();
    if nearby.is_empty() {
        return None;
    }
    // Lê poucos NBT por ciclo, em rodízio.
    let start = number(entity, "Probe") as usize % nearby.len();
    for step in 0..PROBES.min(nearby.len()) {
        let id = nearby[(start + step) % nearby.len()];
        let Some(handle) = host.entity(id) else { continue };
        if host.entity_nbt(handle).as_ref().and_then(uuid_of) == Some(uuid) {
            remember_entity(uuid, Some(id));
            set_number(entity, "Probe", 0);
            return handle.position(host).map(|at| (id, at));
        }
    }
    set_number(entity, "Probe", ((start + PROBES) % nearby.len()) as i32);
    None
}

/// Tick do posto. O relógio `now` vem de `defense::clock` (registros em memória); `time` é o
/// tempo do mundo (fase do ciclo).
pub(super) fn tick(host: &mut dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity, time: i64) {
    let now = defense::clock();
    step(host, pos, entity, time, now);
}

fn publish_ration(entity: &mut ModBlockEntity) {
    set_number(entity, "Info", number(entity, "Ration") / TASK_COST);
}

pub(super) fn step(host: &mut dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity, time: i64, now: u64) {
    publish_ration(entity);
    if (time + i64::from((pos[0] ^ pos[2]) & 7)).rem_euclid(EVERY) != 0 {
        return;
    }
    let Some(uuid) = worker(entity) else {
        // Sem trabalhador, o que ficou carregado entra assim que houver espaço.
        deliver(entity);
        set_number(entity, "Status", NO_WORKER);
        return;
    };
    let dimension = host.dimension();
    if !claim(uuid, dimension, pos, now) {
        // Outro posto ficou com este aldeão enquanto este estava descarregado.
        set_worker(entity, None);
        deliver(entity);
        set_number(entity, "Status", NO_WORKER);
        return;
    }
    let Some((id, at)) = locate(host, pos, entity, uuid) else {
        let absent = number(entity, "Absent") + EVERY as i32;
        if absent >= ABSENT_LIMIT {
            // Morto, removido ou longe demais por muito tempo: libera o posto.
            release(uuid, dimension, pos);
            set_worker(entity, None);
            deliver(entity);
            set_number(entity, "Status", NO_WORKER);
        } else {
            set_number(entity, "Absent", absent);
            set_number(entity, "Status", ABSENT);
        }
        return;
    };
    set_number(entity, "Absent", 0);
    let status = work(host, pos, entity, id, at, now);
    set_number(entity, "Status", status);
    publish_ration(entity);
}

/// Uma etapa do trabalho com o aldeão encontrado em `at`. Devolve o estado do painel.
fn work(host: &mut dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity, id: u32, at: [f64; 3], now: u64) -> i32 {
    let home = post_center(pos);
    // 1. Carregando: leva ao posto antes de qualquer outra tarefa.
    if has_load(entity) {
        order(id, home, now);
        if !near(at, home, POST_REACH) {
            return walk(entity, None);
        }
        set_number(entity, "Walk", 0);
        if !deliver(entity) {
            return 3;
        }
        host.play_sound("minecraft:entity.villager.work_farmer", home, 4, 0.5, 1.0);
        return 1;
    }
    // 2. Comida: cada tarefa gasta `TASK_COST` pontos; o trabalhador come do slot 0 até cobrir.
    if !feed(entity, TASK_COST) {
        cancel(id);
        set_target(entity, None);
        return NO_FOOD;
    }
    // 3. Tarefa: o submódulo do posto escolhe e executa.
    let job = match entity.id.as_str() {
        FARM_POST => farmhand::next(host, pos, entity),
        _ => Err(NO_TASK),
    };
    let spot = match job {
        Ok(spot) => spot,
        Err(KEEP) => {
            cancel(id);
            return number(entity, "Status");
        }
        Err(status) => {
            cancel(id);
            return status;
        }
    };
    let goal = [f64::from(spot[0]) + 0.5, f64::from(spot[1]), f64::from(spot[2]) + 0.5];
    order(id, goal, now);
    if !near(at, goal, REACH) {
        return walk(entity, Some(spot));
    }
    match farmhand::perform(host, spot, entity) {
        Some(load) => {
            set_carried(entity, &load);
            set_number(entity, "Ration", number(entity, "Ration") - TASK_COST);
            set_number(entity, "Total", number(entity, "Total").saturating_add(1));
            set_target(entity, None);
            order(id, home, now);
            1
        }
        // A planta mudou enquanto ele andava: procura outra no próximo ciclo.
        None => {
            set_target(entity, None);
            1
        }
    }
}

/// Conta o tempo de caminhada; acima do limite, desiste do alvo (caminho obstruído).
fn walk(entity: &mut ModBlockEntity, spot: Option<[i32; 3]>) -> i32 {
    let walked = number(entity, "Walk") + EVERY as i32;
    if walked < WALK_LIMIT {
        set_number(entity, "Walk", walked);
        return WALKING;
    }
    if let Some(spot) = spot {
        farmhand::skip(entity, spot);
        set_target(entity, None);
    } else {
        // Voltando ao posto com carga: continua tentando, mas avisa.
        set_number(entity, "Walk", walked);
    }
    PATH
}

/// Come do slot 0 até `Ration` cobrir `need` pontos, ou falha se a comida acabar.
fn feed(entity: &mut ModBlockEntity, need: i32) -> bool {
    while number(entity, "Ration") < need {
        if !eat(entity) {
            return false;
        }
    }
    true
}

/// O trabalhador come um item do slot de comida. Valor = pontos de `production.json`.
pub(super) fn eat(entity: &mut ModBlockEntity) -> bool {
    let Some(stack) = entity.items[0].as_ref() else { return false };
    let Some(value) = production::pantry::value(&stack.item).filter(|_| stack.extra.is_none()) else { return false };
    consume(&mut entity.items[0], 1);
    set_number(entity, "Ration", number(entity, "Ration") + value as i32);
    true
}

/// Linhas do painel do posto.
pub(super) fn panel_lines(block: &str, data: &[i32]) -> Option<Vec<String>> {
    if !is_post(block) {
        return None;
    }
    let read = |index: usize| data.get(index).copied().unwrap_or(0);
    let tr = |key: &str, args: &[&dyn std::fmt::Display]| Text::tr(key, args).plain();
    Some(vec![
        tr("veio.metric.tasks", &[&read(9)]),
        tr("veio.metric.ration", &[&read(12)]),
        tr("veio.hint.farm_post", &[]),
    ])
}

pub(super) fn screen_slots(block: &str) -> Option<Vec<Option<[f32; 2]>>> {
    is_post(block).then(|| vec![Some([26.0, 45.0]), Some([98.0, 45.0]), Some([116.0, 45.0]), Some([134.0, 45.0])])
}

pub(super) fn roles(block: &str) -> Option<&'static [&'static str]> {
    is_post(block).then_some(&["food", "output", "", ""][..])
}

#[cfg(test)]
mod tests;
