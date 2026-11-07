//! Posto do pastor (setor rancho): o trabalhador cuida dos animais a até `RADIUS` blocos.
//!
//! - Recolhe do chão ovos, penas, couro, lã e outros produtos de animais (os mesmos do rancho).
//! - Tosquia ovelhas adultas.
//! - Dá a ração do slot 1 a animais adultos que a comem, um de cada vez, para reproduzir até
//!   `ranch::CAP` por espécie. Só começa se houver dois adultos prontos da mesma espécie.
//!
//! Tudo exige o aldeão ao lado do animal ou do item. Não usa energia; ao contrário do rancho,
//! não ordenha nem abate. Animais vêm do registro do rancho (`entity_tick_pre`), lendo no
//! máximo `MAX_READS` NBT por ciclo.
use super::*;
use production::ranch;

pub(super) const RADIUS: f64 = 6.0;
/// NBT de animais lidos por ciclo, em rodízio (o mesmo teto do `locate` dos aldeões).
pub(super) const MAX_READS: usize = 4;
const SHEEP: u8 = 1;
/// Ações guardadas em `Action` enquanto o trabalhador anda.
const GATHER: i32 = 1;
const SHEAR: i32 = 2;
const FEED: i32 = 3;
const FEED_SLOT: usize = 1;

fn spot_of(at: [f64; 3]) -> [i32; 3] {
    [at[0].floor() as i32, at[1].floor() as i32, at[2].floor() as i32]
}

fn in_range(at: [f64; 3], post: [i32; 3]) -> bool {
    (at[0] - f64::from(post[0]) - 0.5).hypot(at[2] - f64::from(post[2]) - 0.5) <= RADIUS
        && (at[1] - f64::from(post[1])).abs() <= 3.0
}

struct Seen {
    id: u32,
    kind: u8,
    at: [f64; 3],
    adult: bool,
    in_love: bool,
    sheared: bool,
    color: i32,
}

/// Animais no raio (sem NBT) e a fatia cujo NBT foi lido neste ciclo.
fn animals(host: &mut dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity) -> (Vec<(u32, u8, [f64; 3])>, Vec<Seen>) {
    let mut nearby: Vec<(u32, u8, [f64; 3])> = Vec::new();
    for (id, kind) in ranch::recent(host.dimension()) {
        if kind == ranch::VILLAGER {
            continue;
        }
        let Some(handle) = host.entity(id) else { continue };
        let Some(at) = handle.position(host).filter(|at| in_range(*at, pos)) else { continue };
        nearby.push((id, kind, at));
    }
    if nearby.is_empty() {
        return (nearby, Vec::new());
    }
    let start = number(entity, "Herd") as usize % nearby.len();
    set_number(entity, "Herd", (start + MAX_READS) as i32);
    let mut list = Vec::new();
    for step in 0..MAX_READS.min(nearby.len()) {
        let (id, kind, at) = nearby[(start + step) % nearby.len()];
        let Some(handle) = host.entity(id) else { continue };
        let Some(nbt) = host.entity_nbt(handle) else { continue };
        list.push(Seen {
            id, kind, at,
            adult: ranch::tag_int(&nbt, "Age") >= 0,
            in_love: ranch::tag_int(&nbt, "InLove") > 0,
            sheared: ranch::tag_int(&nbt, "Sheared") != 0,
            color: ranch::tag_int(&nbt, "Color"),
        });
    }
    (nearby, list)
}

/// Se o alvo atual ainda precisa da ação, só atualiza a posição — o animal anda.
fn follow(host: &mut dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity) -> Option<Plan> {
    let id = number(entity, "TargetMob") as u32;
    let action = number(entity, "Action");
    if id == 0 || action == 0 || action == GATHER {
        return None;
    }
    let handle = host.entity(id)?;
    let at = handle.position(host).filter(|at| in_range(*at, pos))?;
    let nbt = host.entity_nbt(handle)?;
    match action {
        SHEAR if ranch::tag_int(&nbt, "Sheared") == 0 && ranch::tag_int(&nbt, "Age") >= 0 => {
            Some(choose(entity, SHEAR, id, at))
        }
        FEED if ranch::tag_int(&nbt, "InLove") == 0 && ranch::tag_int(&nbt, "Age") >= 0
            && entity.items[FEED_SLOT].as_ref().is_some_and(|stack| plain(stack)) =>
        {
            Some(choose(entity, FEED, id, at))
        }
        _ => None,
    }
}

fn wool(color: i32) -> String {
    format!("minecraft:{}_wool", ranch::WOOL[color.clamp(0, 15) as usize])
}

fn choose(entity: &mut ModBlockEntity, action: i32, id: u32, at: [f64; 3]) -> Plan {
    set_number(entity, "Action", action);
    set_number(entity, "TargetMob", id as i32);
    // O alvo anda: a posição muda sem zerar o tempo de caminhada.
    let spot = spot_of(at);
    set_pos(entity, "Target", Some(spot));
    Plan::Visit { spot, reach: REACH }
}

/// Próxima tarefa do pastor. `Err` traz o estado de espera do painel.
pub(super) fn plan(host: &mut dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity, _now: u64) -> Result<Plan, i32> {
    let room = entity.items[outputs(&entity.id)].to_vec();
    let dimension = host.dimension();
    // 1. Produtos no chão.
    let (min, max) = ([f64::from(pos[0]) - RADIUS, f64::from(pos[1]) - 2.0, f64::from(pos[2]) - RADIUS],
        [f64::from(pos[0]) + RADIUS + 1.0, f64::from(pos[1]) + 3.0, f64::from(pos[2]) + RADIUS + 1.0]);
    let mut full = false;
    for drop in host.item_entities(dimension, min, max) {
        if !ranch::collectable(&drop.stack.item) || drop.stack.extra.is_some() {
            continue;
        }
        if farm::drops_fit(&room, &[(drop.stack.item.as_ref(), 1)]) {
            return Ok(choose(entity, GATHER, drop.id, drop.position));
        }
        full = true;
    }
    if let Some(plan) = follow(host, pos, entity) {
        return Ok(plan);
    }
    let (nearby, herd) = animals(host, pos, entity);
    // 2. Ovelhas para tosquiar.
    for sheep in herd.iter().filter(|animal| animal.kind == SHEEP && animal.adult && !animal.sheared) {
        if farm::drops_fit(&room, &[(wool(sheep.color).as_str(), 3)]) {
            return Ok(choose(entity, SHEAR, sheep.id, sheep.at));
        }
        full = true;
    }
    // 3. Ração: só quando dois adultos da mesma espécie podem cruzar.
    let feed = entity.items[FEED_SLOT].as_ref().filter(|stack| plain(stack)).map(|stack| stack.item.to_string());
    if let Some(item) = feed {
        for &kind in ranch::feeds(&item) {
            let population = nearby.iter().filter(|(_, seen, _)| *seen == kind).count();
            let ready: Vec<&Seen> = herd.iter().filter(|animal| animal.kind == kind && animal.adult && !animal.in_love).collect();
            let loving = herd.iter().any(|animal| animal.kind == kind && animal.in_love);
            // Um já comeu: alimenta o par dele. Senão, precisa de dois prontos.
            if population < ranch::CAP && !ready.is_empty() && (loving || ready.len() >= 2) {
                return Ok(choose(entity, FEED, ready[0].id, ready[0].at));
            }
        }
    }
    set_target(entity, None);
    Err(if full { 3 } else if nearby.is_empty() { production::NO_ANIMALS } else { IDLE_WORK })
}

/// Executa a ação ao lado do alvo. Devolve a carga (vazia ao alimentar) ou `None` se o alvo
/// sumiu, já foi atendido ou se afastou.
pub(super) fn perform(host: &mut dyn Host, spot: [i32; 3], entity: &mut ModBlockEntity, _now: u64) -> Option<Vec<(String, u32)>> {
    let id = number(entity, "TargetMob") as u32;
    let action = number(entity, "Action");
    set_number(entity, "Action", 0);
    let goal = [f64::from(spot[0]) + 0.5, f64::from(spot[1]), f64::from(spot[2]) + 0.5];
    let dimension = host.dimension();
    let room = entity.items[outputs(&entity.id)].to_vec();
    if action == GATHER {
        let min = [goal[0] - 2.0, goal[1] - 2.0, goal[2] - 2.0];
        let max = [goal[0] + 2.0, goal[1] + 2.0, goal[2] + 2.0];
        let drop = host.item_entities(dimension, min, max).into_iter().find(|drop| drop.id == id)?;
        let item = drop.stack.item.to_string();
        let count = (1..=drop.stack.count).rev().find(|count| farm::drops_fit(&room, &[(item.as_str(), *count)]))?;
        let mut rest = drop.stack.clone();
        rest.count -= count;
        return host.set_item_entity(dimension, id, rest).then(|| vec![(item, count)]);
    }
    let handle = host.entity(id)?;
    let at = handle.position(host)?;
    if !near(at, goal, POST_REACH) {
        return None;
    }
    let nbt = host.entity_nbt(handle)?;
    match action {
        SHEAR if ranch::tag_int(&nbt, "Sheared") == 0 && ranch::tag_int(&nbt, "Age") >= 0 => {
            let wool = wool(ranch::tag_int(&nbt, "Color"));
            let count = 1 + id % 3;
            if !farm::drops_fit(&room, &[(wool.as_str(), count)])
                || !host.entity_merge_nbt(handle, Compound::from([("Sheared".into(), Tag::Byte(1))]))
            {
                return None;
            }
            host.play_sound("minecraft:entity.sheep.shear", at, 4, 0.8, 1.0);
            Some(vec![(wool, count)])
        }
        FEED if ranch::tag_int(&nbt, "InLove") == 0 && ranch::tag_int(&nbt, "Age") >= 0 => {
            let item = entity.items[FEED_SLOT].as_ref().map(|stack| stack.item.to_string())?;
            let kind = ranch::recent(dimension).into_iter().find(|(seen, _)| *seen == id).map(|(_, kind)| kind)?;
            if !ranch::feeds(&item).contains(&kind)
                || !host.entity_merge_nbt(handle, Compound::from([("InLove".into(), Tag::Int(600))]))
            {
                return None;
            }
            consume(&mut entity.items[FEED_SLOT], 1);
            host.send_particles(&ParticleOptions::simple(ParticleType::HappyVillager), None, [at[0], at[1] + 1.0, at[2]], [0.3, 0.3, 0.3], 0.0, 4);
            Some(Vec::new())
        }
        _ => None,
    }
}
