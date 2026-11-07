//! Mesa do capataz (coordenação): um aldeão contratado cuida de até `MAX_POSTS` postos de
//! trabalho num raio de `RADIUS` blocos.
//!
//! - Leva comida da mesa ao posto cujo trabalhador está ficando sem.
//! - Leva uma esmeralda da mesa ao posto que ficou sem trabalhador (aldeão morto, removido,
//!   dispensado) e contrata ali o aldeão adulto livre mais próximo.
//!
//! O capataz não faz o trabalho dos postos nem escolhe as tarefas deles: cada posto continua
//! com seu adaptador e suas reservas. Ele também precisa de comida (dos mesmos slots da mesa).
use super::*;

pub(super) const RADIUS: i32 = 12;
/// Uma fileira do 25×25 por ciclo; a volta (1875 blocos) leva 75 ciclos.
pub(super) const SCAN: i32 = 25;
pub(super) const MAX_POSTS: usize = 6;
/// Abaixo disto (comida guardada no posto e pontos do trabalhador), o capataz reabastece.
pub(super) const LOW_FOOD: u32 = 4;
const FOOD_BATCH: u32 = 8;
const EMERALD_SLOT: usize = 3;

fn managed(block: &str) -> bool {
    is_post(block) && block != FOREMAN_DESK
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
    let Some(posts) = scan_blocks(host, pos, entity, RADIUS, SCAN, MAX_POSTS, &managed) else {
        return Err(KEEP);
    };
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
            let spare = if number(entity, "Ration") > 0 { stack.count } else { stack.count.saturating_sub(1) };
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
