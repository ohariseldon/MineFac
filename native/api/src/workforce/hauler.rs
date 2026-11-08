//! Posto do carregador (indústria): o trabalhador abastece as máquinas próximas com os insumos
//! do posto e recolhe os produtos delas para as saídas do posto.
//!
//! As máquinas vêm da interface de postos do Codex (`operations`): só estações do tipo
//! `Workstation` são atendidas, nunca esteiras, divisores, unificadores, postes, cabos ou outros
//! postos de trabalho. Entradas seguem `operations::accepts_material` (receitas, combustíveis e o
//! pedido atual da central). Saídas são as do `layout` da máquina.
//!
//! Prioridade: tarefas da fila de uma mesa do capataz próxima (`foreman::claim_task`); máquina
//! parada por falta de insumo ou combustível; depois produtos prontos; depois completar entradas
//! até `TOP_UP` itens.
use super::*;
use operations::{Blockage, StationKind};

/// Área atendida: 13x13 em volta do posto, em três camadas.
pub(super) const RADIUS: i32 = 6;
/// Uma fileira do 13×13 por ciclo; a volta (507 blocos) leva 32 ciclos.
pub(super) const SCAN: i32 = 16;
const MAX_STATIONS: usize = 12;
/// Máquinas inspecionadas por ciclo depois da primeira volta. 12 máquinas ociosas
/// levam 4 ciclos (2 s) para uma volta de inspeção, em vez de 12 `inspect` a cada 10 ticks.
pub(super) const LOOKS: usize = 3;
/// Itens levados ou recolhidos por viagem.
pub(super) const BATCH: u32 = 16;
/// O carregador mantém cada entrada com até esta quantidade.
pub(super) const TOP_UP: u32 = 16;

pub(super) fn station(block: &str) -> bool {
    !is_post(block) && operations::profile(block).is_some_and(|profile| profile.kind == StationKind::Workstation)
}

/// O que o carregador vê numa máquina.
#[derive(Debug, Default)]
struct Look {
    hungry: bool,
    /// (slot do estoque do posto, slot de entrada da máquina, quantidade)
    feed: Option<(usize, usize, u32)>,
    /// Há produto pronto que cabe no posto.
    output: bool,
    output_blocked: bool,
}

fn look(host: &mut dyn Host, spot: [i32; 3], stock: &[(usize, ItemStack)], room: &[Option<ItemStack>], batch: u32) -> Option<Look> {
    let mut seen = None;
    host.change_mod_block_entity(spot, &mut |machine| {
        let machine: &ModBlockEntity = machine;
        if !station(&machine.id) {
            return;
        }
        let Some(view) = operations::inspect(machine) else { return };
        let mut look = Look { hungry: matches!(view.blockage, Blockage::Input | Blockage::Fuel), ..Look::default() };
        for slot in &view.outputs {
            let Some(stack) = slot.stack.filter(|stack| plain(stack)) else { continue };
            if farm::drops_fit(room, &[(stack.item.as_ref(), 1)]) {
                look.output = true;
            } else {
                look.output_blocked = true;
            }
        }
        look.feed = stock.iter().find_map(|(from, stack)| view.inputs.iter().find_map(|slot| {
            let current = match slot.stack {
                None => 0,
                Some(present) if is_plain(present, &stack.item) => present.count,
                Some(_) => return None,
            };
            (current < TOP_UP.max(batch) && operations::accepts_material(machine, slot.index, stack))
                .then(|| (*from, slot.index, (TOP_UP.max(batch) - current).min(stack.count).min(batch)))
        }));
        seen = Some(look);
    });
    seen
}

/// Próxima viagem. `Err` traz o estado de espera do painel.
pub(super) fn plan(host: &mut dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity, now: u64) -> Result<Plan, i32> {
    let dimension = host.dimension();
    // A fila do capataz vem antes das tarefas próprias.
    match foreman::search_task(host, pos, entity, now) {
        foreman::TaskSearch::Found(task) => {
            if let Plan::Fetch { from, .. } = &task { set_target(entity, Some(*from)); }
            return Ok(task);
        }
        foreman::TaskSearch::Pending => return Err(KEEP),
        foreman::TaskSearch::Empty => {},
    }
    let Some(stations) = scan_blocks(host, pos, entity, RADIUS, SCAN, MAX_STATIONS, &station) else {
        return Err(KEEP);
    };
    set_number(entity, "Rate", stations.len() as i32);
    if stations.is_empty() {
        set_target(entity, None);
        return Err(IDLE_WORK);
    }
    let stock: Vec<(usize, ItemStack)> = stock_slots(&entity.id)
        .filter_map(|slot| entity.items[slot].clone().filter(plain).map(|stack| (slot, stack)))
        .collect();
    let room = entity.items[outputs(&entity.id)].to_vec();
    let avoid = skipped(entity);
    // Ferraria (obra da vila): viagens maiores.
    let batch = projects::haul_batch(projects::at(dimension, pos, now));
    let mut blocked = false;
    let mut topping = None;
    let mut inspected = 0;
    for spot in visiting_order(entity, &stations) {
        if avoid.contains(&spot) || reserved_by_other(dimension, spot, pos, now) {
            continue;
        }
        inspected += 1;
        let Some(look) = look(host, spot, &stock, &room, batch) else { continue };
        blocked |= look.output_blocked;
        let bring = |(from, into, count)| Plan::Bring { from, count, to: spot, into, kind: Carry::Supply };
        if look.hungry && let Some(feed) = look.feed {
            set_target(entity, Some(spot));
            return Ok(bring(feed));
        }
        if look.output {
            set_target(entity, Some(spot));
            return Ok(Plan::Visit { spot, reach: POST_REACH });
        }
        if topping.is_none() {
            topping = look.feed.map(|feed| (spot, bring(feed)));
        }
        if inspected >= LOOKS {
            break;
        }
    }
    if let Some((spot, plan)) = topping {
        set_target(entity, Some(spot));
        return Ok(plan);
    }
    rotate(entity, stations.len());
    set_target(entity, None);
    Err(if blocked { 3 } else { IDLE_WORK })
}

/// Recolhe os produtos da máquina ao lado do trabalhador, até `2 x BATCH` e o que couber no posto.
pub(super) fn perform(host: &mut dyn Host, spot: [i32; 3], entity: &mut ModBlockEntity) -> Option<Vec<(String, u32)>> {
    let batch = projects::haul_batch(projects::at(host.dimension(), spot, defense::clock()));
    let mut room = entity.items[outputs(&entity.id)].to_vec();
    let mut taken: Vec<(String, u32)> = Vec::new();
    host.change_mod_block_entity(spot, &mut |machine| {
        if !station(&machine.id) {
            return;
        }
        // `layout` da raiz (todas as máquinas); o de `workforce` só conhece os postos.
        let Some((_, _, slots)) = super::super::layout(&machine.id) else { return };
        let mut budget = batch * 2;
        for index in slots {
            let Some(stack) = machine.items.get(index).and_then(Option::as_ref).filter(|stack| plain(stack)) else { continue };
            let item = stack.item.to_string();
            let Some(count) = (1..=stack.count.min(budget)).rev().find(|count| farm::drops_fit(&room, &[(item.as_str(), *count)])) else {
                continue;
            };
            farm::store(&mut room, &item, count);
            consume(&mut machine.items[index], count);
            budget -= count;
            match taken.iter_mut().find(|(known, _)| *known == item) {
                Some((_, total)) => *total += count,
                None => taken.push((item, count)),
            }
            if budget == 0 {
                break;
            }
        }
    });
    if !taken.is_empty() {
        host.play_sound("minecraft:entity.item.pickup", center(spot), 4, 0.4, 0.9);
    }
    (!taken.is_empty()).then_some(taken)
}
