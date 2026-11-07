//! Mesa do capataz (coordenação): um aldeão contratado cuida de até `MAX_POSTS` postos de
//! trabalho num raio de `RADIUS` blocos.
//!
//! - Leva comida da mesa ao posto cujo trabalhador está ficando sem.
//! - Leva uma esmeralda da mesa ao posto que ficou sem trabalhador (aldeão morto, removido,
//!   dispensado) e contrata ali o aldeão adulto livre mais próximo.
//!
//! O capataz não faz o trabalho dos postos nem escolhe as tarefas deles: cada posto continua
//! com seu adaptador e suas reservas. Ele também precisa de comida (dos mesmos slots da mesa).
//!
//! **Fila de tarefas.** Com o capataz presente, a mesa mantém uma fila (`Queue`, no NBT da mesa)
//! de transferências entre os blocos da sua rede (postos e máquinas a até `RADIUS` blocos):
//! produtos prontos de um bloco para a entrada de outro que os aceite, inclusive comida e
//! esmeraldas para a própria mesa e ração para o posto do pastor. Os carregadores perto da mesa
//! pegam tarefas da fila antes das suas (`claim_task`), marcando-as com sua posição; uma marca
//! não renovada vence e a tarefa volta a ficar livre. Ao pegar os itens na origem, o carregador
//! tira a tarefa da fila (`complete`).
use super::*;
use std::sync::Mutex;

pub(super) const RADIUS: i32 = 12;
/// Uma fileira do 25×25 por ciclo; a volta (1875 blocos) leva 75 ciclos.
pub(super) const SCAN: i32 = 25;
pub(super) const MAX_POSTS: usize = 6;
/// Abaixo disto (comida guardada no posto e pontos do trabalhador), o capataz reabastece.
pub(super) const LOW_FOOD: u32 = 4;
const FOOD_BATCH: u32 = 8;
const EMERALD_SLOT: usize = 3;
/// Blocos da rede da mesa: postos atendidos e máquinas.
const MAX_NETWORK: usize = 24;
/// Tarefas na fila.
pub(super) const MAX_QUEUE: usize = 8;
/// A fila é refeita a cada 4 ciclos do capataz (40 ticks).
const ORGANIZE_EVERY: i32 = 4;
/// Uma tarefa marcada por um carregador volta a ficar livre se a marca não for renovada.
const TASK_STALE: u64 = 200;
/// Carregadores a até esta distância da mesa pegam tarefas dela.
pub(super) const DESK_RANGE: f64 = 16.0;
/// Origem e destino precisam estar a até esta distância do posto do carregador.
pub(super) const HAUL_RANGE: f64 = 12.0;
/// A mesa pede comida pela fila quando guarda menos que isto.
pub(super) const DESK_FOOD: u32 = 16;

fn managed(block: &str) -> bool {
    is_post(block) && block != FOREMAN_DESK
}

fn networked(block: &str) -> bool {
    managed(block) || hauler::station(block) || storage::is_store(block)
}

/// Situação de um posto atendido.
struct Look {
    staffed: bool,
    /// Item e quantidade no slot de comida (slot 0).
    food: Option<(String, u32)>,
    ration: i32,
}

fn look(host: &mut dyn Host, spot: [i32; 3]) -> Option<Look> {
    let mut seen = None;
    host.change_mod_block_entity(spot, &mut |post| {
        if managed(&post.id) {
            seen = Some(Look {
                staffed: worker(post).is_some(),
                food: post.items.first().and_then(Option::as_ref).map(|stack| (stack.item.to_string(), stack.count)),
                ration: number(post, "Ration"),
            });
        }
    });
    seen
}

/// Próxima tarefa do capataz. `Err` traz o estado de espera do painel.
pub(super) fn plan(host: &mut dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity, now: u64) -> Result<Plan, i32> {
    let dimension = host.dimension();
    // A rede é varrida por `organize`, que roda antes em cada ciclo.
    let Some(network) = get_list(entity, "Area") else { return Err(KEEP) };
    let posts: Vec<[i32; 3]> = network.into_iter().filter(|spot| managed(&host.block(*spot))).take(MAX_POSTS).collect();
    set_number(entity, "Rate", posts.len() as i32);
    if posts.is_empty() {
        set_target(entity, None);
        return Err(IDLE_WORK);
    }
    let has_wage = entity.items[EMERALD_SLOT].as_ref().is_some_and(|stack| is_plain(stack, WAGE));
    let avoid = skipped(entity);
    let mut needs_wage = false;
    for spot in visiting_order(entity, &posts) {
        if avoid.contains(&spot) || reserved_by_other(dimension, spot, pos, now) {
            continue;
        }
        let Some(post) = look(host, spot) else { continue };
        if !post.staffed {
            if has_wage {
                set_target(entity, Some(spot));
                return Ok(Plan::Bring { from: EMERALD_SLOT, count: 1, to: spot, into: 0, kind: Carry::Wage });
            }
            needs_wage = true;
            continue;
        }
        let stored = post.food.as_ref().map_or(0, |(_, count)| *count);
        if post.ration >= LOW_FOOD as i32 || stored >= LOW_FOOD {
            continue;
        }
        let supply = food_slots(&entity.id).find_map(|from| {
            let stack = entity.items[from].as_ref().filter(|stack| plain(stack) && is_food(&stack.item))?;
            let room = match &post.food {
                None => 64,
                Some((item, count)) if *item == *stack.item => 64 - (*count).min(64),
                Some(_) => 0,
            };
            // Deixa um item para o próprio capataz.
            let spare = if number(entity, "Ration") >= TASK_COST { stack.count } else { stack.count.saturating_sub(1) };
            let count = FOOD_BATCH.min(spare).min(room);
            (count > 0).then_some((from, count))
        });
        if let Some((from, count)) = supply {
            set_target(entity, Some(spot));
            return Ok(Plan::Bring { from, count, to: spot, into: 0, kind: Carry::Supply });
        }
    }
    rotate(entity, posts.len());
    set_target(entity, None);
    Err(if needs_wage { NO_WAGE } else { IDLE_WORK })
}

// ---------- Fila de tarefas ----------

static DESKS: Mutex<Vec<(String, [i32; 3], u64)>> = Mutex::new(Vec::new());

fn register_desk(dimension: &str, pos: [i32; 3], now: u64) {
    if let Ok(mut desks) = DESKS.lock() {
        desks.retain(|(dim, at, seen)| !(dim == dimension && *at == pos) && now.saturating_sub(*seen) <= TASK_STALE);
        desks.push((dimension.to_owned(), pos, now));
    }
}

fn desks_near(dimension: &str, at: [i32; 3], now: u64) -> Vec<[i32; 3]> {
    DESKS.lock().map(|desks| desks.iter()
        .filter(|(dim, desk, seen)| dim == dimension && now.saturating_sub(*seen) <= TASK_STALE
            && distance(*desk, at) <= DESK_RANGE)
        .map(|(_, desk, _)| *desk).collect()).unwrap_or_default()
}

fn distance(a: [i32; 3], b: [i32; 3]) -> f64 {
    (0..3).map(|axis| f64::from(a[axis] - b[axis]).powi(2)).sum::<f64>().sqrt()
}

/// Uma transferência da fila.
#[derive(Clone, Debug, PartialEq)]
pub(super) struct Task {
    pub id: i32,
    pub from: [i32; 3],
    pub slot: usize,
    pub item: String,
    pub count: u32,
    pub to: [i32; 3],
    pub into: usize,
    /// Carregador que a pegou e quando (relógio do mod).
    pub by: Option<([i32; 3], u64)>,
}

fn int_array(tag: Option<&Tag>) -> Option<[i32; 3]> {
    match tag {
        Some(Tag::IntArray(values)) if values.len() == 3 => Some([values[0], values[1], values[2]]),
        _ => None,
    }
}

fn int(tag: Option<&Tag>) -> i32 {
    match tag { Some(Tag::Int(value)) => *value, _ => 0 }
}

pub(super) fn queue(desk: &ModBlockEntity) -> Vec<Task> {
    let Some(Tag::List(list)) = desk.nbt.get(&"Queue".into()) else { return Vec::new() };
    list.iter().filter_map(|tag| {
        let Tag::Compound(task) = tag else { return None };
        let get = |key: &str| task.get(&key.into());
        let Some(Tag::String(item)) = get("item") else { return None };
        Some(Task {
            id: int(get("id")),
            from: int_array(get("from"))?,
            slot: int(get("slot")).max(0) as usize,
            item: item.to_string_lossy(),
            count: int(get("count")).max(0) as u32,
            to: int_array(get("to"))?,
            into: int(get("into")).max(0) as usize,
            by: int_array(get("by")).map(|by| (by, match get("seen") { Some(Tag::Long(seen)) => *seen as u64, _ => 0 })),
        })
    }).collect()
}

fn set_queue(desk: &mut ModBlockEntity, tasks: &[Task]) {
    let list = tasks.iter().map(|task| {
        let mut tag = Compound::from([
            ("id".into(), Tag::Int(task.id)),
            ("from".into(), Tag::IntArray(task.from.to_vec())),
            ("slot".into(), Tag::Int(task.slot as i32)),
            ("item".into(), Tag::String(task.item.as_str().into())),
            ("count".into(), Tag::Int(task.count as i32)),
            ("to".into(), Tag::IntArray(task.to.to_vec())),
            ("into".into(), Tag::Int(task.into as i32)),
        ]);
        if let Some((by, seen)) = task.by {
            tag.insert("by".into(), Tag::IntArray(by.to_vec()));
            tag.insert("seen".into(), Tag::Long(seen as i64));
        }
        Tag::Compound(tag)
    }).collect();
    desk.nbt.insert("Queue".into(), Tag::List(list));
}

/// Quanto cada tipo de destino guarda antes de deixar de pedir.
fn top_up(block: &str) -> u32 {
    match block {
        FOREMAN_DESK | SHEPHERD_POST => 32,
        _ if storage::is_store(block) => 64,
        _ => hauler::TOP_UP,
    }
}

/// Entradas que a fila pode abastecer: as das máquinas; nos postos, só a ração do pastor e a
/// própria mesa. Comida dos trabalhadores é tarefa do capataz; estoque do carregador é do jogador.
fn queue_inputs(block: &str) -> Vec<usize> {
    match block {
        FOREMAN_DESK => vec![0, 1, 2, 3],
        SHEPHERD_POST => vec![1],
        _ if is_post(block) => Vec::new(),
        _ => super::super::layout(block).map(|(_, inputs, _)| inputs).unwrap_or_default(),
    }
}

/// Saídas que a fila pode esvaziar.
fn queue_outputs(block: &str) -> Vec<usize> {
    if is_post(block) {
        outputs(block).collect()
    } else {
        super::super::layout(block).map(|(_, _, outputs)| outputs).unwrap_or_default()
    }
}

/// Varre a rede e, a cada `ORGANIZE_EVERY` ciclos, refaz a fila. Tarefas já pegas por um
/// carregador (e renovadas) são mantidas; as demais são recalculadas do estado atual.
pub(super) fn organize(host: &mut dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity, now: u64) {
    let dimension = host.dimension();
    register_desk(dimension, pos, now);
    let Some(network) = scan_named(host, pos, entity, "Area", RADIUS, SCAN, MAX_NETWORK, &networked, true) else { return };
    let round = (number(entity, "Organize") + 1) % ORGANIZE_EVERY;
    set_number(entity, "Organize", round);
    if round != 0 {
        return;
    }
    let mut tasks: Vec<Task> = queue(entity).into_iter()
        .filter(|task| task.by.is_some_and(|(_, seen)| now.saturating_sub(seen) <= TASK_STALE))
        .collect();
    // Produtos disponíveis: (bloco, slot, item, quantidade). Armazéns vêm por último: produtos
    // novos saem das máquinas antes de mexer no estoque.
    let mut sources: Vec<([i32; 3], usize, String, u32)> = Vec::new();
    let mut stores: Vec<[i32; 3]> = Vec::new();
    for &spot in &network {
        host.change_mod_block_entity(spot, &mut |node| {
            if storage::is_store(&node.id) {
                stores.push(spot);
            }
            for slot in queue_outputs(&node.id) {
                if let Some(stack) = node.items.get(slot).and_then(Option::as_ref).filter(|stack| plain(stack)) {
                    sources.push((spot, slot, stack.item.to_string(), stack.count));
                }
            }
        });
    }
    sources.retain(|(spot, slot, _, _)| !tasks.iter().any(|task| task.from == *spot && task.slot == *slot));
    sources.sort_by_key(|(spot, _, _, _)| stores.contains(spot));
    let mut next_id = number(entity, "TaskId");
    let mut plan_for = |sink: &ModBlockEntity, spot: [i32; 3], tasks: &mut Vec<Task>, sources: &mut Vec<([i32; 3], usize, String, u32)>| {
        // A mesa só pede comida quando está acabando; o resto da comida segue para as máquinas.
        let desk_food: u32 = if sink.id == FOREMAN_DESK {
            food_slots(&sink.id).filter_map(|slot| sink.items[slot].as_ref()).map(|stack| stack.count).sum()
        } else { 0 };
        let store = storage::is_store(&sink.id);
        for into in queue_inputs(&sink.id) {
            if sink.id == FOREMAN_DESK && food_slots(&sink.id).contains(&into) && desk_food >= DESK_FOOD {
                continue;
            }
            // Armazém: uma entrega por rodada, num slot vazio ou com o mesmo item.
            if store && tasks.iter().any(|task| task.to == spot) {
                break;
            }
            if tasks.len() >= MAX_QUEUE || tasks.iter().any(|task| task.to == spot && task.into == into) {
                continue;
            }
            let current = sink.items.get(into).and_then(Option::as_ref);
            let Some(index) = sources.iter().position(|(from, _, item, _)| *from != spot
                && !(store && stores.contains(from))
                && current.is_none_or(|present| is_plain(present, item) && present.count < top_up(&sink.id))
                && operations::accepts_material(sink, into, &ItemStack::new(item, 1))) else { continue };
            let (from, slot, item, available) = sources.remove(index);
            let have = current.map_or(0, |present| present.count);
            let count = available.min(hauler::BATCH).min(top_up(&sink.id) - have);
            next_id += 1;
            tasks.push(Task { id: next_id, from, slot, item, count, to: spot, into, by: None });
        }
    };
    // A própria mesa primeiro: comida e esmeraldas mantêm o resto funcionando.
    plan_for(entity, pos, &mut tasks, &mut sources);
    // Depois as máquinas e os postos; armazéns por último, com o que sobrar.
    let order = network.iter().filter(|spot| !stores.contains(spot)).chain(network.iter().filter(|spot| stores.contains(spot)));
    for &spot in order {
        // Destino com entrega a caminho (reservado por um carregador) não recebe outra.
        if tasks.len() >= MAX_QUEUE || reserved_by_other(dimension, spot, pos, now) {
            continue;
        }
        host.change_mod_block_entity(spot, &mut |sink| plan_for(sink, spot, &mut tasks, &mut sources));
    }
    set_number(entity, "TaskId", next_id);
    set_queue(entity, &tasks);
    set_number(entity, "Queued", tasks.len() as i32);
    // O painel lê o tamanho da fila no slot 7 (`Utilization`), livre fora das máquinas elétricas.
    set_number(entity, "Utilization", tasks.len() as i32);
}

/// Tarefa da fila para o carregador em `hauler`: a que ele já pegou, ou a primeira livre cuja
/// origem e destino estejam ao alcance dele.
pub(super) fn claim_task(host: &mut dyn Host, hauler: [i32; 3], now: u64) -> Option<Plan> {
    let dimension = host.dimension();
    for desk in desks_near(dimension, hauler, now) {
        let mut claimed = None;
        host.change_mod_block_entity(desk, &mut |table| {
            if table.id != FOREMAN_DESK {
                return;
            }
            let mut tasks = queue(table);
            let mine = tasks.iter().position(|task| task.by.is_some_and(|(by, seen)| by == hauler && now.saturating_sub(seen) <= TASK_STALE));
            let free = || tasks.iter().position(|task| task.by.is_none_or(|(_, seen)| now.saturating_sub(seen) > TASK_STALE)
                && distance(task.from, hauler) <= HAUL_RANGE && distance(task.to, hauler) <= HAUL_RANGE);
            let Some(index) = mine.or_else(free) else { return };
            tasks[index].by = Some((hauler, now));
            claimed = Some(tasks[index].clone());
            set_queue(table, &tasks);
        });
        if let Some(task) = claimed {
            return Some(Plan::Fetch { desk, task: task.id, from: task.from, slot: task.slot, item: task.item,
                count: task.count, to: task.to, into: task.into });
        }
    }
    None
}

/// Tira a tarefa da fila: o carregador já pegou os itens (ou não havia mais nada na origem).
pub(super) fn complete(host: &mut dyn Host, desk: [i32; 3], id: i32) {
    host.change_mod_block_entity(desk, &mut |table| {
        if table.id == FOREMAN_DESK {
            let tasks: Vec<Task> = queue(table).into_iter().filter(|task| task.id != id).collect();
            set_queue(table, &tasks);
            set_number(table, "Queued", tasks.len() as i32);
        }
    });
}
