//! Escolher o trabalhador clicando nele, e perguntar a um aldeão onde ele trabalha.
//!
//! O clique com esmeralda no posto contrata o aldeão livre mais próximo (até 8 blocos). Quando
//! não há ninguém tão perto, o posto fica **esperando a escolha** por 30 s: o jogador clica no
//! aldeão que quiser, a até 32 blocos do posto, ainda com a esmeralda na mão, e ele é contratado.
//! Sem a espera, clicar num aldeão com esmeralda continua abrindo o comércio vanilla.
//!
//! Agachado e de mão vazia, clicar num aldeão mostra na barra de ação para qual posto ele
//! trabalha e o estado do posto, ou que ele está livre.
use super::*;

/// Tempo da espera pela escolha, no relógio do mod (30 s).
pub(super) const PICK_TIME: u64 = 600;
/// Distância máxima entre o aldeão escolhido e o posto.
pub(super) const PICK_RADIUS: f64 = 32.0;

struct Pick {
    player: u128,
    dimension: String,
    post: [i32; 3],
    until: u64,
}

static PICKS: Mutex<Vec<Pick>> = Mutex::new(Vec::new());

/// O posto `post` espera o jogador escolher um aldeão. Uma espera por jogador.
pub(super) fn arm(player: u128, dimension: &str, post: [i32; 3], now: u64) {
    let Ok(mut picks) = PICKS.lock() else { return };
    picks.retain(|pick| pick.player != player && pick.until > now);
    if picks.len() < 64 {
        picks.push(Pick { player, dimension: dimension.to_owned(), post, until: now + PICK_TIME });
    }
}

fn armed(player: u128, dimension: &str, now: u64) -> Option<[i32; 3]> {
    PICKS.lock().ok()?.iter()
        .find(|pick| pick.player == player && pick.dimension == dimension && pick.until > now)
        .map(|pick| pick.post)
}

fn disarm(player: u128) {
    if let Ok(mut picks) = PICKS.lock() {
        picks.retain(|pick| pick.player != player);
    }
}

/// Mensagem de contratação: o que o trabalhador vai fazer e, se o posto está sem comida, que
/// ela é o próximo passo.
pub(super) fn hired_key(block: &str, has_food: bool) -> &'static str {
    if !has_food && matches!(block, FARM_POST | HAULER_POST | FOREMAN_DESK | SHEPHERD_POST | BARRACKS) {
        return "veio.worker.hired.hungry";
    }
    match block {
        FARM_POST => "veio.worker.hired",
        HAULER_POST => "veio.worker.hired.hauler",
        FOREMAN_DESK => "veio.worker.hired.foreman",
        SHEPHERD_POST => "veio.worker.hired.shepherd",
        _ => "veio.worker.hired.other",
    }
}

/// O posto tem comida para o trabalhador?
pub(super) fn has_food(entity: &ModBlockEntity) -> bool {
    food_slots(&entity.id).any(|slot| entity.items.get(slot).is_some_and(Option::is_some))
        || number(entity, "Ration") >= TASK_COST
}

/// Clique de um jogador num aldeão. `Some(Flow::Cancel)` quando o Veio tomou o clique.
pub(crate) fn interact(host: &mut dyn Host, entity: u32, held: Option<&str>) -> Option<Flow> {
    let player = host.acting_player()?;
    let dimension = host.dimension();
    let now = defense::clock();
    if held == Some(WAGE) && let Some(post) = armed(player.uuid, dimension, now) {
        disarm(player.uuid);
        hire_picked(host, player.uuid, post, entity, now);
        return Some(Flow::Cancel);
    }
    if held.is_none() && host.sneaking() {
        describe(host, player.uuid, entity, now);
        return Some(Flow::Cancel);
    }
    None
}

fn hire_picked(host: &mut dyn Host, player: u128, post: [i32; 3], id: u32, now: u64) {
    let dimension = host.dimension();
    let block = host.block(post);
    if !is_post(&block) {
        host.action_bar(player, Text::tr("veio.worker.pick.gone", &[]));
        return;
    }
    let mut state = None;
    host.change_mod_block_entity(post, &mut |entity| {
        if is_post(&entity.id) {
            state = Some((worker(entity).is_some(), entity.id == SITE && site::refuses_hire(entity)));
        }
    });
    match state {
        None => {
            host.action_bar(player, Text::tr("veio.worker.pick.gone", &[]));
            return;
        }
        Some((true, _)) => {
            host.action_bar(player, Text::tr("veio.worker.staffed", &[]));
            return;
        }
        Some((_, true)) => {
            host.action_bar(player, Text::tr("veio.site.no_staff", &[]));
            return;
        }
        _ => {}
    }
    let Some(handle) = host.entity(id) else { return };
    let at = handle.position(host);
    let Some(nbt) = host.entity_nbt(handle) else { return };
    let Some(uuid) = uuid_of(&nbt) else { return };
    if !adult(&nbt) {
        host.action_bar(player, Text::tr("veio.worker.child", &[]));
        return;
    }
    if claimed_elsewhere(uuid, dimension, post, now) {
        host.action_bar(player, Text::tr("veio.worker.pick.taken", &[]));
        return;
    }
    if !at.is_some_and(|at| near(at, post_center(post), PICK_RADIUS)) {
        host.action_bar(player, Text::tr("veio.worker.pick.far", &[&(PICK_RADIUS as i32)]));
        return;
    }
    if host.take_items(WAGE, 1) == 0 {
        return;
    }
    let mut hired = false;
    let mut food = true;
    host.change_mod_block_entity(post, &mut |entity| {
        hired = assign(dimension, post, entity, (uuid, id), now);
        food = has_food(entity);
    });
    if !hired {
        host.give_items(WAGE, 1);
        host.action_bar(player, Text::tr("veio.worker.pick.taken", &[]));
        return;
    }
    host.action_bar(player, Text::tr(hired_key(&block, food), &[]));
    host.play_sound("minecraft:entity.villager.yes", post_center(post), 4, 0.8, 1.0);
    if let Some(at) = at {
        host.send_particles(&ParticleOptions::simple(ParticleType::HappyVillager), None,
            [at[0], at[1] + 1.8, at[2]], [0.3, 0.3, 0.3], 0.0, 8);
    }
}

/// Barra de ação: onde o aldeão trabalha e o estado do posto, ou que ele está livre.
fn describe(host: &mut dyn Host, player: u128, id: u32, now: u64) {
    let dimension = host.dimension();
    let post = employers(dimension, now).into_iter().find(|(entity, _)| *entity == id).map(|(_, post)| post);
    let Some(post) = post else {
        let child = host.entity(id).and_then(|handle| host.entity_nbt(handle)).is_some_and(|nbt| !adult(&nbt));
        host.action_bar(player, Text::tr(if child { "veio.worker.child" } else { "veio.worker.free" }, &[]));
        return;
    };
    let mut status = 0;
    host.change_mod_block_entity(post, &mut |entity| status = number(entity, "Status"));
    let [x, y, z] = post;
    host.action_bar(player, Text::tr(works_key(status), &[&x, &y, &z]));
}

/// Frase para o estado do posto: os problemas têm frase própria, o resto é "trabalhando".
pub(super) fn works_key(status: i32) -> &'static str {
    match status {
        NO_FOOD => "veio.worker.works.no_food",
        PATH => "veio.worker.works.path",
        3 => "veio.worker.works.blocked",
        RESTING => "veio.worker.works.rest",
        NO_TASK | IDLE_WORK | production::NO_ANIMALS => "veio.worker.works.idle",
        NO_WAGE => "veio.worker.works.no_wage",
        _ => "veio.worker.works",
    }
}

#[cfg(test)]
pub(super) fn reset() {
    if let Ok(mut picks) = PICKS.lock() {
        picks.clear();
    }
}
