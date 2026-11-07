//! Posto do pastor (setor rancho): o trabalhador cuida dos animais a até `RADIUS` blocos.
//!
//! - Recolhe do chão ovos, penas, couro, lã e outros produtos de animais (os mesmos do rancho).
//! - Tosquia ovelhas adultas.
//! - Dá a ração do slot 1 a animais adultos que a comem, um de cada vez, para reproduzir até
//!   `ranch::CAP` por espécie. Só começa se houver dois adultos prontos da mesma espécie.
//!
//! Com uma **zona de cercado** (`zones`, marco a até 3 blocos da borda), ele trabalha no cercado
//! inteiro em vez do raio fixo, e a ração do slot do marco define o cercado: só ela é usada, e só
//! as espécies que a comem são criadas (trigo: vacas, ovelhas, cabras; cenoura: porcos, coelhos;
//! sementes: galinhas).
//!
//! Tudo exige o aldeão ao lado do animal ou do item. Não usa energia; ao contrário do rancho,
//! não ordenha nem abate. Animais vêm do registro do rancho (`entity_tick_pre`), lendo no
//! máximo `MAX_READS` NBT por ciclo.
use super::*;
use production::ranch;

pub(super) const RADIUS: f64 = 6.0;
const MAX_READS: usize = 12;
const SHEEP: u8 = 1;
/// Ações guardadas em `Action` enquanto o trabalhador anda.
const GATHER: i32 = 1;
const SHEAR: i32 = 2;
const FEED: i32 = 3;
const FEED_SLOT: usize = 1;

fn spot_of(at: [f64; 3]) -> [i32; 3] {
    [at[0].floor() as i32, at[1].floor() as i32, at[2].floor() as i32]
}

fn in_range(at: [f64; 3], post: [i32; 3], zone: Option<&zones::Zone>) -> bool {
    match zone {
        Some(zone) => zones::contains(zone, at),
        None => (at[0] - f64::from(post[0]) - 0.5).hypot(at[2] - f64::from(post[2]) - 0.5) <= RADIUS
            && (at[1] - f64::from(post[1])).abs() <= 3.0,
    }
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

fn animals(host: &mut dyn Host, pos: [i32; 3], zone: Option<&zones::Zone>) -> Vec<Seen> {
    let mut list = Vec::new();
    for (id, kind) in ranch::recent(host.dimension()) {
        if kind == ranch::VILLAGER || list.len() >= MAX_READS {
            continue;
        }
        let Some(handle) = host.entity(id) else { continue };
        let Some(at) = handle.position(host).filter(|at| in_range(*at, pos, zone)) else { continue };
        let Some(nbt) = host.entity_nbt(handle) else { continue };
        list.push(Seen {
            id, kind, at,
            adult: ranch::tag_int(&nbt, "Age") >= 0,
            in_love: ranch::tag_int(&nbt, "InLove") > 0,
            sheared: ranch::tag_int(&nbt, "Sheared") != 0,
            color: ranch::tag_int(&nbt, "Color"),
        });
    }
    list
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
pub(super) fn plan(host: &mut dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity, now: u64) -> Result<Plan, i32> {
    let room = entity.items[outputs(&entity.id)].to_vec();
    let dimension = host.dimension();
    let zone = zones::zone_for(dimension, pos, zones::Kind::Pen, now);
    set_pos(entity, "ZoneAt", zone.as_ref().map(|zone| zone.pos));
    // 1. Produtos no chão.
    let (middle, reach) = zone.as_ref().map_or((pos, RADIUS), |zone| (zone.pos, f64::from(zone.side / 2)));
    let (min, max) = ([f64::from(middle[0]) - reach, f64::from(middle[1]) - 2.0, f64::from(middle[2]) - reach],
        [f64::from(middle[0]) + reach + 1.0, f64::from(middle[1]) + 3.0, f64::from(middle[2]) + reach + 1.0]);
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
    let herd = animals(host, pos, zone.as_ref());
    // 2. Ovelhas para tosquiar.
    for sheep in herd.iter().filter(|animal| animal.kind == SHEEP && animal.adult && !animal.sheared) {
        if farm::drops_fit(&room, &[(wool(sheep.color).as_str(), 3)]) {
            return Ok(choose(entity, SHEAR, sheep.id, sheep.at));
        }
        full = true;
    }
    // 3. Ração: só quando dois adultos da mesma espécie podem cruzar.
    let wanted = zone.as_ref().and_then(|zone| zone.filter.clone());
    let feed = entity.items[FEED_SLOT].as_ref().filter(|stack| plain(stack)).map(|stack| stack.item.to_string())
        .filter(|item| wanted.as_ref().is_none_or(|wanted| wanted == item));
    if let Some(item) = feed {
        for &kind in ranch::feeds(&item) {
            let population = herd.iter().filter(|animal| animal.kind == kind).count();
            let ready: Vec<&Seen> = herd.iter().filter(|animal| animal.kind == kind && animal.adult && !animal.in_love).collect();
            let loving = herd.iter().any(|animal| animal.kind == kind && animal.in_love);
            // Um já comeu: alimenta o par dele. Senão, precisa de dois prontos.
            if population < ranch::CAP && !ready.is_empty() && (loving || ready.len() >= 2) {
                return Ok(choose(entity, FEED, ready[0].id, ready[0].at));
            }
        }
    }
    set_target(entity, None);
    Err(if full { 3 } else if herd.is_empty() { production::NO_ANIMALS } else { IDLE_WORK })
}

/// Executa a ação ao lado do alvo. Devolve a carga (vazia ao alimentar) ou `None` se o alvo
/// sumiu, já foi atendido ou se afastou.
pub(super) fn perform(host: &mut dyn Host, spot: [i32; 3], entity: &mut ModBlockEntity, now: u64) -> Option<Vec<(String, u32)>> {
    let done = act(host, spot, entity, now);
    if done.is_some() && let Some(marker) = get_pos(entity, "ZoneAt") {
        zones::credit(host, marker);
    }
    done
}

fn act(host: &mut dyn Host, spot: [i32; 3], entity: &mut ModBlockEntity, _now: u64) -> Option<Vec<(String, u32)>> {
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
