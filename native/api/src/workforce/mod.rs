//! Trabalhadores: aldeões que o jogador contrata para um posto de trabalho.
//!
//! Conceitos (veja `frentes/CLOUD.md` e `TRABALHADORES.md`):
//! - **Trabalhador**: um aldeão adulto, identificado pelo UUID salvo no NBT do posto. O id de
//!   entidade (`u32`) só vale durante a sessão e fica num registro em memória.
//! - **Posto**: bloco que guarda a atribuição, a comida do trabalhador e os estoques do setor.
//! - **Tarefa (`Plan`)**: escolhida por um adaptador de setor. `Visit` manda o trabalhador a um
//!   lugar e executa a ação do setor ao chegar; `Bring` faz ele pegar itens no posto e levá-los
//!   a outro bloco. Adaptadores: `farmhand` (lavoura), `hauler` (indústria) e `foreman`
//!   (coordenação de outros postos).
//! - **Atribuição**: exclusiva. Um aldeão pertence a no máximo um posto (`CLAIMS`).
//! - **Reserva**: um alvo (planta, máquina, posto) atendido por um posto não é escolhido por
//!   outro enquanto a reserva for renovada (`RESERVED`). Assim dois trabalhadores não disputam
//!   a mesma tarefa.
//!
//! O trabalho é físico: o posto manda o aldeão andar (objetivo `Work`, registrado nos aldeões)
//! e só age quando ele está ao lado do alvo. Itens viajam com ele (`Carried` no NBT do posto).
//! Aldeões sem ordem recente seguem o comportamento vanilla, inclusive comércio.
use super::*;
use horizon_client::mobs::Mob;
use horizon_client::mobs::goals::{self, Cx, Goal};
use horizon_client::mod_api::{ParticleOptions, ParticleType, Text};
use std::collections::HashMap;

#[cfg(test)]
pub(super) fn profile_organize(host: &mut dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity, now: u64) {
    foreman::organize(host, pos, entity, now);
}
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;

pub(super) mod farmhand;
pub(super) mod foreman;
pub(super) mod hauler;
pub(super) mod shepherd;
pub(super) mod center;
pub(super) mod storage;
pub(super) mod spawn;
pub(super) mod village;
pub(super) mod zones;

pub(super) const FARM_POST: &str = "veio:farm_post";
pub(super) const HAULER_POST: &str = "veio:hauler_post";
pub(super) const FOREMAN_DESK: &str = "veio:foreman_desk";
pub(super) const SHEPHERD_POST: &str = "veio:shepherd_post";
pub(super) const CONTROL_CENTER: &str = center::CONTROL_CENTER;
pub(super) use zones::{FIELD_MARKER, PEN_MARKER};
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
/// Carregador e capataz sem nada a fazer.
pub(super) const IDLE_WORK: i32 = 24;
/// Capataz encontrou posto vazio, mas não tem esmeralda para contratar.
pub(super) const NO_WAGE: i32 = 25;
/// Marco de zona ativo e armazém com espaço.
pub(super) const ZONE_ACTIVE: i32 = 26;
pub(super) const STORING: i32 = 27;
/// Resposta de uma tarefa que ainda está procurando: o painel mantém o estado anterior.
pub(super) const KEEP: i32 = -1;

/// Pontos de comida gastos em cada tarefa concluída (colheita, viagem, recontratação).
/// Trigo cru vale 1 e não paga o ciclo; pão vale 6 e paga três tarefas.
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
/// Distância horizontal para trabalhar numa planta (o aldeão pisa ao lado ou em cima dela).
pub(super) const REACH: f64 = 1.8;
/// Distância horizontal para usar um bloco sólido: posto, máquina.
pub(super) const POST_REACH: f64 = 2.5;
/// Uma reivindicação que o posto não renova nesse tempo perde a validade (posto removido ou
/// descarregado). Medido no relógio do mod (`defense::clock`).
const CLAIM_STALE: u64 = 400;
/// Uma ordem de caminhada vale por esse tempo; o posto a renova a cada ciclo.
const ORDER_FRESH: u64 = 60;
/// Uma reserva de alvo vale por esse tempo; o posto a renova a cada ciclo.
const RESERVE_STALE: u64 = 100;
/// Alvos inalcançáveis lembrados até a próxima volta da varredura.
const MAX_SKIPPED: usize = 8;

pub(super) type Uuid = [i32; 4];

/// Blocos com trabalhador contratado.
pub(super) fn is_post(block: &str) -> bool {
    matches!(block, FARM_POST | HAULER_POST | FOREMAN_DESK | SHEPHERD_POST | CONTROL_CENTER)
}

/// Todos os blocos deste módulo: postos, marcos de zona e armazéns.
pub(super) fn owns(block: &str) -> bool {
    is_post(block) || zones::is_marker(block) || storage::is_store(block)
}

pub(super) fn layout(block: &str) -> Option<(usize, Vec<usize>, Vec<usize>)> {
    Some(match block {
        // Comida; colheita entregue.
        FARM_POST => (4, vec![0], vec![1, 2, 3]),
        // Comida; insumos para as máquinas; produtos recolhidos.
        HAULER_POST => (7, vec![0, 1, 2, 3], vec![4, 5, 6]),
        // Comida (do capataz e dos postos); esmeraldas para contratar.
        FOREMAN_DESK => (4, vec![0, 1, 2, 3], vec![]),
        // Comida; ração para os animais; produtos (lã, ovos, penas).
        SHEPHERD_POST => (5, vec![0, 1], vec![2, 3, 4]),
        // Comida do administrador; três metas (sem automação).
        CONTROL_CENTER => (4, vec![0], vec![]),
        _ => return zones::layout(block).or_else(|| storage::layout(block)),
    })
}

pub(super) fn outputs(block: &str) -> std::ops::Range<usize> {
    match block {
        FARM_POST => 1..4,
        HAULER_POST => 4..7,
        SHEPHERD_POST => 2..5,
        _ => 0..0,
    }
}

/// Slots de onde o próprio trabalhador come.
pub(super) fn food_slots(block: &str) -> std::ops::Range<usize> {
    if block == FOREMAN_DESK { 0..3 } else { 0..1 }
}

/// Estoque de onde saem as entregas (`Bring`) e para onde volta o que não foi usado.
pub(super) fn stock_slots(block: &str) -> std::ops::Range<usize> {
    match block {
        HAULER_POST => 1..4,
        FOREMAN_DESK => 0..4,
        SHEPHERD_POST => 1..2,
        _ => 0..0,
    }
}

fn plain(stack: &ItemStack) -> bool {
    stack.extra.is_none() && stack.damage == 0
}

pub(super) fn is_food(item: &str) -> bool {
    production::pantry::value(item).is_some()
}

pub(super) fn accepts(block: &str, slot: usize, stack: &ItemStack) -> bool {
    plain(stack) && match (block, slot) {
        (FARM_POST | HAULER_POST | SHEPHERD_POST, 0) | (FOREMAN_DESK, 0..=2) => is_food(&stack.item),
        (SHEPHERD_POST, 1) => production::ranch::is_feed(&stack.item),
        (HAULER_POST, 1..=3) => true,
        (FOREMAN_DESK, 3) => stack.item.as_ref() == WAGE,
        (CONTROL_CENTER, _) => center::accepts(slot, stack),
        _ if zones::is_marker(block) => zones::accepts(block, slot, stack),
        _ if storage::is_store(block) => storage::accepts(block, slot, stack),
        _ => false,
    }
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

// ---------- Reservas de alvos ----------

static RESERVED: Mutex<Vec<(String, [i32; 3], [i32; 3], u64)>> = Mutex::new(Vec::new());
static RESERVED_PENDING: AtomicBool = AtomicBool::new(false);

/// Outro posto vivo está atendendo este alvo?
pub(super) fn reserved_by_other(dimension: &str, spot: [i32; 3], owner: [i32; 3], now: u64) -> bool {
    if !RESERVED_PENDING.load(Ordering::Acquire) {
        return false;
    }
    RESERVED.lock().is_ok_and(|list| list.iter().any(|(dim, at, by, seen)|
        dim == dimension && *at == spot && *by != owner && now.saturating_sub(*seen) <= RESERVE_STALE))
}

/// Reserva ou renova o alvo para este posto. `false` se outro posto vivo já o reservou.
pub(super) fn reserve(dimension: &str, spot: [i32; 3], owner: [i32; 3], now: u64) -> bool {
    let Ok(mut list) = RESERVED.lock() else { return false };
    list.retain(|(_, _, _, seen)| now.saturating_sub(*seen) <= RESERVE_STALE);
    if list.iter().any(|(dim, at, by, _)| dim == dimension && *at == spot && *by != owner) {
        RESERVED_PENDING.store(true, Ordering::Release);
        return false;
    }
    // Um posto reserva um alvo por vez.
    list.retain(|(dim, _, by, _)| !(dim == dimension && *by == owner));
    list.push((dimension.to_owned(), spot, owner, now));
    RESERVED_PENDING.store(true, Ordering::Release);
    true
}

/// Solta a reserva deste posto, se houver.
pub(super) fn unreserve(dimension: &str, owner: [i32; 3]) {
    if let Ok(mut list) = RESERVED.lock() {
        list.retain(|(dim, _, by, _)| !(dim == dimension && *by == owner));
        if list.is_empty() {
            RESERVED_PENDING.store(false, Ordering::Release);
        }
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

fn get_pos(entity: &ModBlockEntity, key: &str) -> Option<[i32; 3]> {
    match entity.nbt.get(&key.into()) {
        Some(Tag::IntArray(values)) if values.len() == 3 => Some([values[0], values[1], values[2]]),
        _ => None,
    }
}

fn set_pos(entity: &mut ModBlockEntity, key: &str, pos: Option<[i32; 3]>) {
    match pos {
        Some(pos) => { entity.nbt.insert(key.into(), Tag::IntArray(pos.to_vec())); }
        None => { entity.nbt.remove(&key.into()); }
    }
}

pub(super) fn target(entity: &ModBlockEntity) -> Option<[i32; 3]> {
    get_pos(entity, "Target")
}

pub(super) fn set_target(entity: &mut ModBlockEntity, pos: Option<[i32; 3]>) {
    if target(entity) != pos {
        entity.nbt.remove(&"Walk".into());
    }
    set_pos(entity, "Target", pos);
}

/// Alvos que o trabalhador não alcançou, até a próxima volta da varredura do adaptador.
pub(super) fn skipped(entity: &ModBlockEntity) -> Vec<[i32; 3]> {
    match entity.nbt.get(&"Skipped".into()) {
        Some(Tag::IntArray(values)) => values.chunks_exact(3).map(|v| [v[0], v[1], v[2]]).collect(),
        _ => Vec::new(),
    }
}

pub(super) fn skip(entity: &mut ModBlockEntity, spot: [i32; 3]) {
    let mut list = skipped(entity);
    if list.len() >= MAX_SKIPPED {
        list.remove(0);
    }
    list.push(spot);
    entity.nbt.insert("Skipped".into(), Tag::IntArray(list.into_iter().flatten().collect()));
}

pub(super) fn clear_skipped(entity: &mut ModBlockEntity) {
    entity.nbt.remove(&"Skipped".into());
}

/// O que acontece com a carga do trabalhador.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Carry {
    /// Produto recolhido: vai para as saídas do posto.
    Product = 0,
    /// Insumo ou comida levada a outro bloco; o que sobrar volta ao estoque do posto.
    Supply = 1,
    /// Esmeralda levada a um posto vazio para contratar; volta se ninguém for contratado.
    Wage = 2,
}

pub(super) fn carry_kind(entity: &ModBlockEntity) -> Carry {
    match number(entity, "CarryKind") {
        1 => Carry::Supply,
        2 => Carry::Wage,
        _ => Carry::Product,
    }
}

/// Destino da carga; `None` é o próprio posto.
pub(super) fn carry_to(entity: &ModBlockEntity) -> Option<[i32; 3]> {
    get_pos(entity, "CarryTo")
}

/// Há carga a entregar? Evita montar o `Vec` de `Carried` em todo ciclo ocioso.
fn has_load(entity: &ModBlockEntity) -> bool {
    matches!(entity.nbt.get(&"Carried".into()), Some(Tag::List(list)) if !list.is_empty())
}

/// Itens que o trabalhador carrega. Ficam no NBT do posto: salvar, descarregar o chunk ou
/// trocar de trabalhador não os perde.
pub(super) fn carried(entity: &ModBlockEntity) -> Vec<(String, u32)> {
    let Some(Tag::List(list)) = entity.nbt.get(&"Carried".into()) else { return Vec::new() };
    list.iter().filter_map(|tag| {
        let Tag::Compound(item) = tag else { return None };
        let Some(Tag::String(id)) = item.get(&"id".into()) else { return None };
        let Some(Tag::Int(count)) = item.get(&"count".into()) else { return None };
        (*count > 0).then(|| (id.to_string_lossy(), *count as u32))
    }).collect()
}

pub(super) fn set_carried(entity: &mut ModBlockEntity, items: &[(String, u32)], to: Option<[i32; 3]>, kind: Carry, into: usize) {
    if items.is_empty() {
        for key in ["Carried", "CarryTo", "CarryKind", "CarryInto"] {
            entity.nbt.remove(&key.into());
        }
        return;
    }
    let list = items.iter().map(|(id, count)| Tag::Compound(Compound::from([
        ("id".into(), Tag::String(id.as_str().into())),
        ("count".into(), Tag::Int(*count as i32)),
    ]))).collect();
    entity.nbt.insert("Carried".into(), Tag::List(list));
    set_pos(entity, "CarryTo", to);
    set_number(entity, "CarryKind", kind as i32);
    set_number(entity, "CarryInto", into as i32);
}

/// Guarda a carga no próprio posto, se tudo couber: produtos nas saídas, sobras no estoque.
pub(super) fn deliver(entity: &mut ModBlockEntity) -> bool {
    let load = carried(entity);
    if load.is_empty() {
        return true;
    }
    let kind = carry_kind(entity);
    let range = if kind == Carry::Product { outputs(&entity.id) } else { stock_slots(&entity.id) };
    let mut slots = entity.items.clone();
    for (id, count) in &load {
        // Sobras voltam só aos slots que aceitam o item (a esmeralda não ocupa o lugar da comida).
        let accepted: Vec<usize> = range.clone()
            .filter(|&slot| kind == Carry::Product || accepts(&entity.id, slot, &ItemStack::new(id, 1)))
            .collect();
        let mut part: Vec<Option<ItemStack>> = accepted.iter().map(|&slot| slots[slot].take()).collect();
        let fits = farm::store(&mut part, id, *count);
        for (slot, stack) in accepted.into_iter().zip(part) {
            slots[slot] = stack;
        }
        if !fits {
            return false;
        }
    }
    entity.items = slots;
    set_carried(entity, &[], None, Carry::Product, 0);
    if kind == Carry::Product {
        set_number(entity, "Produced", number(entity, "Produced").saturating_add(1));
    }
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

// ---------- Varredura de blocos ----------

/// Posição `index` de uma área `(2r+1)² x 3`: a camada do posto, depois a de baixo e a de cima.
pub(super) fn area_position(pos: [i32; 3], radius: i32, index: i32) -> [i32; 3] {
    let side = radius * 2 + 1;
    let layer = side * side;
    let index = index.rem_euclid(layer * 3);
    let cell = index % layer;
    [pos[0] + cell % side - radius, pos[1] + [0, -1, 1][(index / layer) as usize], pos[2] + cell / side - radius]
}

fn get_list(entity: &ModBlockEntity, key: &str) -> Option<Vec<[i32; 3]>> {
    match entity.nbt.get(&key.into()) {
        Some(Tag::IntArray(values)) => Some(values.chunks_exact(3).map(|v| [v[0], v[1], v[2]]).collect()),
        _ => None,
    }
}

fn set_list(entity: &mut ModBlockEntity, key: &str, list: &[[i32; 3]]) {
    entity.nbt.insert(key.into(), Tag::IntArray(list.iter().flatten().copied().collect()));
}

/// Varredura incremental dos blocos em volta do posto. Lê `per_cycle` posições por chamada e
/// guarda até `max` blocos aceitos por `wanted`. Devolve a lista da última volta completa
/// (`None` até a primeira terminar). Ao completar uma volta, os alvos pulados são esquecidos.
pub(super) fn scan_blocks(host: &dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity, radius: i32,
    per_cycle: i32, max: usize, wanted: &dyn Fn(&str) -> bool) -> Option<Vec<[i32; 3]>>
{
    scan_named(host, pos, entity, "Area", radius, per_cycle, max, wanted, true)
}

/// Como `scan_blocks`, com as chaves NBT começando por `key` (um posto pode ter mais de uma
/// varredura). `forget` esquece os alvos pulados ao fim de cada volta.
#[allow(clippy::too_many_arguments)]
pub(super) fn scan_named(host: &dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity, key: &str, radius: i32,
    per_cycle: i32, max: usize, wanted: &dyn Fn(&str) -> bool, forget: bool) -> Option<Vec<[i32; 3]>>
{
    let (scan, partial) = (format!("{key}Scan"), format!("{key}Found"));
    let side = radius * 2 + 1;
    let area = side * side * 3;
    let mut cursor = number(entity, &scan) % area;
    let mut found = get_list(entity, &partial).unwrap_or_default();
    for _ in 0..per_cycle {
        let spot = area_position(pos, radius, cursor);
        if spot != pos && found.len() < max && wanted(&host.block(spot)) {
            found.push(spot);
        }
        cursor += 1;
        if cursor >= area {
            cursor = 0;
            set_list(entity, key, &found);
            found.clear();
            if forget {
                clear_skipped(entity);
            }
        }
    }
    set_number(entity, &scan, cursor);
    set_list(entity, &partial, &found);
    get_list(entity, key)
}

/// Ordem de atendimento: o alvo atual primeiro, depois um rodízio que começa em `Pick`.
pub(super) fn visiting_order(entity: &ModBlockEntity, spots: &[[i32; 3]]) -> Vec<[i32; 3]> {
    let start = number(entity, "Pick") as usize % spots.len().max(1);
    let mut order: Vec<[i32; 3]> = target(entity).into_iter().filter(|spot| spots.contains(spot)).collect();
    order.extend((0..spots.len()).map(|step| spots[(start + step) % spots.len()]));
    order
}

/// Avança o rodízio depois de um ciclo sem tarefa.
pub(super) fn rotate(entity: &mut ModBlockEntity, len: usize) {
    set_number(entity, "Pick", ((number(entity, "Pick") as usize + 1) % len.max(1)) as i32);
}

// ---------- Contratar e dispensar ----------

/// Aldeão adulto e livre mais próximo do posto: (UUID, id na sessão).
pub(super) fn candidate(host: &mut dyn Host, pos: [i32; 3], now: u64) -> Option<(Uuid, u32)> {
    let dimension = host.dimension();
    let center = post_center(pos);
    // Posições do censo (até 5 s de idade); a distância exata não é crítica para contratar.
    let mut nearby: Vec<(f64, u32)> = production::ranch::villager_positions(host, dimension).iter().copied()
        .filter(|(_, at)| near(*at, center, HIRE_RADIUS))
        .map(|(id, at)| ((at[0] - center[0]).hypot(at[2] - center[2]), id))
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
    unreserve(dimension, pos);
    set_worker(entity, None);
    deliver(entity);
    set_number(entity, "Status", NO_WORKER);
    true
}

/// Clique com esmeralda contrata; agachado com a chave, dispensa.
pub(super) fn right_click(host: &mut dyn Host, pos: [i32; 3], held: &str) -> Option<Flow> {
    let block = host.block(pos);
    if zones::is_marker(&block) {
        return zones::right_click(host, pos, held);
    }
    if block == CONTROL_CENTER && held == "minecraft:book" {
        return center::report_to_chat(host, pos);
    }
    if !is_post(&block) {
        return None;
    }
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
    let nearby: Vec<u32> = production::ranch::villager_positions(host, host.dimension()).iter().copied()
        .filter(|(_, at)| (at[0] - center[0]).hypot(at[2] - center[2]) <= SEARCH_RADIUS
            && (at[1] - center[1]).abs() <= SEARCH_RADIUS)
        .map(|(id, _)| id)
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
    village::flush(host);
    if zones::is_marker(&entity.id) {
        return zones::tick(host, pos, entity, time);
    }
    if storage::is_store(&entity.id) {
        return storage::tick(host, pos, entity, time);
    }
    let now = defense::clock();
    step(host, pos, entity, time, now);
    if (time + i64::from((pos[0] ^ pos[2]) & 7)).rem_euclid(EVERY) == 0 {
        if entity.id == CONTROL_CENTER && number(entity, "Status") != 1 {
            set_number(entity, "Admin", 0);
        }
        center::report(host.dimension(), pos, entity, now);
    }
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
    if entity.id == FOREMAN_DESK {
        foreman::organize(host, pos, entity, now);
    }
    let status = work(host, pos, entity, id, at, now);
    set_number(entity, "Status", status);
    publish_ration(entity);
}

/// Tarefa escolhida por um adaptador de setor.
#[derive(Clone, Debug, PartialEq)]
pub(super) enum Plan {
    /// Ir até `spot`, a até `reach` blocos, e executar a ação do setor ao chegar.
    Visit { spot: [i32; 3], reach: f64 },
    /// Pegar `count` itens do slot `from` do posto e levá-los ao bloco `to` (slot `into`).
    Bring { from: usize, count: u32, to: [i32; 3], into: usize, kind: Carry },
    /// Tarefa da fila do capataz (`desk`, número `task`): buscar `count` de `item` no slot
    /// `slot` do bloco `from` e levar ao slot `into` do bloco `to`.
    Fetch { desk: [i32; 3], task: i32, from: [i32; 3], slot: usize, item: String, count: u32, to: [i32; 3], into: usize },
}

fn block_goal(spot: [i32; 3]) -> [f64; 3] {
    post_center(spot)
}

/// Uma etapa do trabalho com o aldeão encontrado em `at`. Devolve o estado do painel.
fn work(host: &mut dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity, id: u32, at: [f64; 3], now: u64) -> i32 {
    let home = post_center(pos);
    let dimension = host.dimension();
    // 1. Carregando: termina a entrega antes de qualquer outra tarefa.
    if has_load(entity) {
        let destination = carry_to(entity);
        let goal = destination.map_or(home, block_goal);
        if let Some(spot) = destination {
            reserve(dimension, spot, pos, now);
        }
        order(id, goal, now);
        if !near(at, goal, POST_REACH) {
            return walk(entity, None);
        }
        set_number(entity, "Walk", 0);
        let Some(spot) = destination else {
            if !deliver(entity) {
                return 3;
            }
            host.play_sound("minecraft:entity.villager.work_farmer", home, 4, 0.5, 1.0);
            return 1;
        };
        arrive(host, entity, spot, now);
        unreserve(dimension, pos);
        order(id, home, now);
        return 1;
    }
    // 2. Comida: cada tarefa gasta `TASK_COST` pontos; o trabalhador come até cobrir.
    if !feed(entity, TASK_COST) {
        cancel(id);
        set_target(entity, None);
        unreserve(dimension, pos);
        return NO_FOOD;
    }
    // 3. Tarefa: o adaptador do setor escolhe.
    let plan = match entity.id.as_str() {
        FARM_POST => farmhand::plan(host, pos, entity, now),
        HAULER_POST => hauler::plan(host, pos, entity, now),
        FOREMAN_DESK => foreman::plan(host, pos, entity, now),
        SHEPHERD_POST => shepherd::plan(host, pos, entity, now),
        CONTROL_CENTER => center::plan(host, pos, entity, now),
        _ => Err(NO_TASK),
    };
    let plan = match plan {
        Ok(plan) => plan,
        Err(status) => {
            cancel(id);
            unreserve(dimension, pos);
            return if status == KEEP { number(entity, "Status") } else { status };
        }
    };
    match plan {
        Plan::Visit { spot, reach } => {
            if !reserve(dimension, spot, pos, now) {
                skip(entity, spot);
                set_target(entity, None);
                return number(entity, "Status");
            }
            let goal = block_goal(spot);
            order(id, goal, now);
            if !near(at, goal, reach) {
                return walk(entity, Some(spot));
            }
            let result = match entity.id.as_str() {
                FARM_POST => farmhand::perform(host, spot, entity),
                HAULER_POST => hauler::perform(host, spot, entity),
                SHEPHERD_POST => shepherd::perform(host, spot, entity, now),
                CONTROL_CENTER => center::perform(host, spot, entity, now),
                _ => None,
            };
            set_target(entity, None);
            unreserve(dimension, pos);
            if let Some(load) = result {
                set_carried(entity, &load, None, Carry::Product, 0);
                finish_task(entity);
                order(id, home, now);
            }
            1
        }
        Plan::Bring { from, count, to, into, kind } => {
            if !reserve(dimension, to, pos, now) {
                skip(entity, to);
                set_target(entity, None);
                return number(entity, "Status");
            }
            // Pega no posto: precisa estar ao lado dele.
            order(id, home, now);
            if !near(at, home, POST_REACH) {
                return walk(entity, Some(to));
            }
            let Some(stack) = entity.items.get(from).and_then(Option::as_ref).filter(|stack| plain(stack)) else {
                return 1;
            };
            let item = stack.item.to_string();
            let taken = count.min(stack.count);
            if taken == 0 {
                return 1;
            }
            consume(&mut entity.items[from], taken);
            set_carried(entity, &[(item, taken)], Some(to), kind, into);
            set_number(entity, "Walk", 0);
            order(id, block_goal(to), now);
            WALKING
        }
        Plan::Fetch { desk, task, from, slot, item, count, to, into } => {
            if !reserve(dimension, from, pos, now) {
                skip(entity, from);
                set_target(entity, None);
                return number(entity, "Status");
            }
            let goal = block_goal(from);
            order(id, goal, now);
            if !near(at, goal, POST_REACH) {
                return walk(entity, Some(from));
            }
            // Ao lado da origem: pega o que ainda estiver lá, e só do item combinado.
            let mut taken = 0;
            host.change_mod_block_entity(from, &mut |source| {
                if let Some(stack) = source.items.get(slot).and_then(Option::as_ref)
                    && is_plain(stack, &item)
                {
                    taken = count.min(stack.count);
                    consume(&mut source.items[slot], taken);
                }
            });
            foreman::complete(host, desk, task);
            set_target(entity, None);
            unreserve(dimension, pos);
            if taken == 0 {
                return 1;
            }
            set_carried(entity, &[(item, taken)], Some(to), Carry::Supply, into);
            set_number(entity, "Walk", 0);
            order(id, block_goal(to), now);
            WALKING
        }
    }
}

/// Uma tarefa concluída: gasta comida e conta no painel.
fn finish_task(entity: &mut ModBlockEntity) {
    set_number(entity, "Ration", number(entity, "Ration").saturating_sub(TASK_COST));
    set_number(entity, "Total", number(entity, "Total").saturating_add(1));
}

/// O trabalhador chegou ao destino da carga. O que não for usado volta com ele ao posto.
fn arrive(host: &mut dyn Host, entity: &mut ModBlockEntity, spot: [i32; 3], now: u64) {
    let load = carried(entity);
    let kind = carry_kind(entity);
    let into = number(entity, "CarryInto") as usize;
    match kind {
        Carry::Supply => {
            let before: u32 = load.iter().map(|(_, count)| count).sum();
            let rest: Vec<(String, u32)> = load.into_iter()
                .map(|(item, count)| { let left = insert_into(host, spot, into, &item, count); (item, left) })
                .filter(|(_, left)| *left > 0)
                .collect();
            if rest.iter().map(|(_, count)| count).sum::<u32>() < before {
                finish_task(entity);
                host.play_sound("minecraft:entity.item.pickup", block_goal(spot), 4, 0.4, 1.2);
            }
            set_carried(entity, &rest, None, Carry::Supply, 0);
        }
        Carry::Wage => {
            if hire_for(host, spot, now) {
                finish_task(entity);
                set_number(entity, "Hired", number(entity, "Hired").saturating_add(1));
                set_carried(entity, &[], None, Carry::Product, 0);
            } else {
                set_carried(entity, &load, None, Carry::Wage, 0);
            }
        }
        // Produtos sempre vão ao posto; um destino aqui é um save inconsistente.
        Carry::Product => set_carried(entity, &load, None, Carry::Product, 0),
    }
}

/// Põe `count` de `item` no slot `into` do bloco em `spot`, se ele aceitar. Devolve a sobra.
pub(super) fn insert_into(host: &mut dyn Host, spot: [i32; 3], into: usize, item: &str, count: u32) -> u32 {
    let mut left = count;
    host.change_mod_block_entity(spot, &mut |other| {
        if !operations::accepts_material(other, into, &ItemStack::new(item, left)) {
            return;
        }
        let Some(slot) = other.items.get_mut(into) else { return };
        let room = match slot {
            None => 64,
            Some(stack) if is_plain(stack, item) => 64 - stack.count.min(64),
            Some(_) => 0,
        };
        let moved = left.min(room);
        if moved > 0 {
            add(slot, item, moved);
            left -= moved;
        }
    });
    left
}

/// Contrata o aldeão livre mais próximo do posto em `spot`, que precisa estar sem trabalhador.
fn hire_for(host: &mut dyn Host, spot: [i32; 3], now: u64) -> bool {
    let dimension = host.dimension();
    let mut empty = false;
    host.change_mod_block_entity(spot, &mut |other| empty = is_post(&other.id) && worker(other).is_none());
    if !empty {
        return false;
    }
    let Some(found) = candidate(host, spot, now) else { return false };
    let mut hired = false;
    host.change_mod_block_entity(spot, &mut |other| hired = assign(dimension, spot, other, found, now));
    hired
}

/// Conta o tempo de caminhada; acima do limite, desiste do alvo (caminho obstruído).
fn walk(entity: &mut ModBlockEntity, spot: Option<[i32; 3]>) -> i32 {
    let walked = number(entity, "Walk") + EVERY as i32;
    if walked < WALK_LIMIT {
        set_number(entity, "Walk", walked);
        return WALKING;
    }
    if let Some(spot) = spot {
        skip(entity, spot);
        set_target(entity, None);
    } else {
        // Com carga: continua tentando, mas avisa.
        set_number(entity, "Walk", walked);
    }
    PATH
}

/// Come dos slots de comida até `Ration` cobrir `need` pontos, ou falha se a comida acabar.
fn feed(entity: &mut ModBlockEntity, need: i32) -> bool {
    while number(entity, "Ration") < need {
        if !eat(entity) {
            return false;
        }
    }
    true
}

/// O trabalhador come um item dos slots de comida. Valor = pontos de `production.json`.
pub(super) fn eat(entity: &mut ModBlockEntity) -> bool {
    for slot in food_slots(&entity.id) {
        let Some(stack) = entity.items[slot].as_ref().filter(|stack| plain(stack)) else { continue };
        let Some(value) = production::pantry::value(&stack.item) else { continue };
        consume(&mut entity.items[slot], 1);
        set_number(entity, "Ration", number(entity, "Ration") + value as i32);
        return true;
    }
    false
}

// ---------- Painel ----------

pub(super) fn panel_lines(block: &str, data: &[i32]) -> Option<Vec<String>> {
    let read = |index: usize| data.get(index).copied().unwrap_or(0);
    let tr = |key: &str, args: &[&dyn std::fmt::Display]| Text::tr(key, args).plain();
    let (first, hint) = match block {
        FARM_POST => (tr("veio.metric.tasks", &[&read(9)]), "veio.hint.farm_post"),
        HAULER_POST => (tr("veio.metric.trips", &[&read(9)]), "veio.hint.hauler_post"),
        // O capataz publica os postos atendidos em `Rate` (slot 6), livre fora das máquinas elétricas.
        FOREMAN_DESK => (tr("veio.metric.posts", &[&read(6), &read(7)]), "veio.hint.foreman_desk"),
        SHEPHERD_POST => (tr("veio.metric.chores", &[&read(9)]), "veio.hint.shepherd_post"),
        _ => return zones::panel_lines(block, data).or_else(|| storage::panel_lines(block, data)),
    };
    Some(vec![first, tr("veio.metric.ration", &[&read(12)]), tr(hint, &[])])
}

pub(super) fn screen_slots(block: &str) -> Option<Vec<Option<[f32; 2]>>> {
    Some(match block {
        FARM_POST => vec![Some([26.0, 45.0]), Some([98.0, 45.0]), Some([116.0, 45.0]), Some([134.0, 45.0])],
        HAULER_POST => vec![Some([17.0, 45.0]), Some([44.0, 45.0]), Some([62.0, 45.0]), Some([80.0, 45.0]),
            Some([116.0, 45.0]), Some([134.0, 45.0]), Some([152.0, 45.0])],
        FOREMAN_DESK => vec![Some([44.0, 45.0]), Some([62.0, 45.0]), Some([80.0, 45.0]), Some([134.0, 45.0])],
        SHEPHERD_POST => vec![Some([17.0, 45.0]), Some([39.0, 45.0]), Some([116.0, 45.0]), Some([134.0, 45.0]), Some([152.0, 45.0])],
        CONTROL_CENTER => center::screen_slots(),
        FIELD_MARKER | PEN_MARKER => vec![Some([89.0, 45.0])],
        _ => return storage::screen_slots(block),
    })
}

pub(super) fn roles(block: &str) -> Option<&'static [&'static str]> {
    Some(match block {
        FARM_POST => &["food", "output", "", ""],
        HAULER_POST => &["food", "supply", "", "", "output", "", ""],
        FOREMAN_DESK => &["food", "", "", "wage"],
        SHEPHERD_POST => &["food", "feed", "output", "", ""],
        CONTROL_CENTER => &[],
        FIELD_MARKER => &["seeds"],
        PEN_MARKER => &["feed"],
        _ if storage::is_store(block) => &[],
        _ => return None,
    })
}

#[cfg(test)]
mod tests;
