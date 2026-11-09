//! Capítulo 2 do tutorial: a vila operária, contada pelo prefeito Otávio.
//!
//! Depois da fábrica, o prefeito ensina a contratar, a construir postos, a demarcar zonas, a
//! organizar a vila com carregador e capataz e a começar a sociedade (casa e prefeitura). Cada
//! etapa só fecha com o resultado no mundo, lido do bloco que o jogador colocou: posto colocado,
//! trabalhador contratado (`Worker`), marco perto do posto, colheitas entregues (`Produced`),
//! família na casa (`Rate`) e funcionário na prefeitura.
//!
//! Começa ao falar com o prefeito depois do tutorial da fábrica, ou com `/veiotutorial vila`.
//! `Progress::village`: 0 não começou, 1..=8 é a etapa 0..=7 em andamento, 9 concluído.
use super::*;
use horizon_client::mod_api::{ParticleOptions, ParticleType};

pub(super) const STEPS: u8 = 8;
pub(super) const DONE_VILLAGE: u8 = STEPS + 1;
/// Linhas "faça" de cada etapa (`veio.village.do.<etapa>.<n>`).
pub(super) const DO_LINES: [u8; 8] = [4, 4, 4, 3, 4, 4, 4, 4];
/// Colheitas entregues no posto para fechar a etapa da primeira colheita.
pub(super) const HARVESTS: i32 = 4;
/// O marco conta se o posto está até aqui do centro dele (meia zona de 9x9 + `zones::LINK`).
pub(super) const MARKER_REACH: i32 = 4 + workforce::zones::LINK;

/// Bloco que cada etapa pede para colocar, com os ingredientes da receita (a ficha e a lista de
/// materiais). A etapa da colheita reaproveita o posto do lavrador.
pub(super) const PARTS: [(&str, &[&str]); 8] = [
    (workforce::FARM_POST, &["minecraft:composter", "minecraft:iron_hoe", "minecraft:emerald", "minecraft:barrel"]),
    (workforce::FARM_POST, &[]),
    (workforce::zones::FIELD_MARKER, &["minecraft:oak_fence", "minecraft:wheat_seeds", "minecraft:stick"]),
    (workforce::FARM_POST, &[]),
    (workforce::HAULER_POST, &["minecraft:chest", "minecraft:hopper", "minecraft:emerald", "veio:iron_plate"]),
    (workforce::FOREMAN_DESK, &["minecraft:lectern", "minecraft:emerald", "minecraft:bell"]),
    (workforce::society::HOUSE, &["#minecraft:planks", "minecraft:oak_door", "minecraft:glass_pane"]),
    (workforce::society::TOWN_HALL, &["minecraft:bell", "minecraft:lectern", "minecraft:emerald", "minecraft:stone_bricks"]),
];

/// Onde o capítulo guarda cada bloco colocado: posto, marco, carregador, capataz, casa, prefeitura.
pub(super) fn slot(block: &str) -> Option<usize> {
    Some(match block {
        workforce::FARM_POST => 0,
        workforce::zones::FIELD_MARKER => 1,
        workforce::HAULER_POST => 2,
        workforce::FOREMAN_DESK => 3,
        workforce::society::HOUSE => 4,
        workforce::society::TOWN_HALL => 5,
        _ => return None,
    })
}

/// A etapa (0..8) em andamento, se o capítulo está aberto.
pub(super) fn step(p: &Progress) -> Option<usize> {
    (1..=STEPS).contains(&p.village).then(|| usize::from(p.village - 1))
}

/// O bloco que a etapa acompanha (para a barra e as faíscas).
fn focus(step: usize) -> usize {
    match step { 0 | 1 | 3 => 0, 2 => 1, 4 => 2, 5 => 3, 6 => 4, _ => 5 }
}

/// O jogador colocou um bloco da vila: guarda onde (o último de cada tipo vale).
pub(super) fn placed(p: &mut Progress, dimension: &str, pos: Pos, block: &str) {
    let Some(index) = slot(block) else { return };
    if step(p).is_none() || dimension != OVERWORLD_DIM { return; }
    p.village_blocks[index] = Some(pos);
    p.dirty = true;
}

const OVERWORLD_DIM: &str = workforce::spawn::OVERWORLD;

/// O que o bloco guardado mostra agora: (é do tipo certo, tem trabalhador, entregas, moradores).
fn read(host: &mut dyn Host, pos: Pos, block: &str) -> Option<(bool, i32, i32)> {
    let mut out = None;
    host.change_mod_block_entity(pos, &mut |entity| {
        if entity.id == block {
            out = Some((workforce::worker(entity).is_some(), number(entity, "Produced"), number(entity, "Rate")));
        }
    });
    out
}

/// Confere a etapa no mundo. Bloco que sumiu (quebrado, trocado) é esquecido.
pub(super) fn ready(host: &mut dyn Host, p: &mut Progress) -> bool {
    let Some(step) = step(p) else { return false };
    let mut look = |p: &mut Progress, index: usize| -> Option<(bool, i32, i32)> {
        let pos = p.village_blocks[index]?;
        let block = [workforce::FARM_POST, workforce::zones::FIELD_MARKER, workforce::HAULER_POST,
            workforce::FOREMAN_DESK, workforce::society::HOUSE, workforce::society::TOWN_HALL][index];
        let state = read(host, pos, block);
        if state.is_none() { p.village_blocks[index] = None; p.dirty = true; }
        state
    };
    match step {
        0 => look(p, 0).is_some(),
        1 => look(p, 0).is_some_and(|(hired, _, _)| hired),
        2 => {
            let post = look(p, 0);
            let marker = look(p, 1);
            match (post, marker, p.village_blocks[0], p.village_blocks[1]) {
                (Some(_), Some(_), Some(a), Some(b)) => (a[0] - b[0]).abs().max((a[2] - b[2]).abs()) <= MARKER_REACH && (a[1] - b[1]).abs() <= 1,
                _ => false,
            }
        }
        3 => look(p, 0).is_some_and(|(_, produced, _)| {
            p.village_count = (produced - p.village_base).clamp(0, HARVESTS);
            p.village_count >= HARVESTS
        }),
        4 => look(p, 2).is_some_and(|(hired, _, _)| hired),
        5 => look(p, 3).is_some_and(|(hired, _, _)| hired),
        6 => look(p, 4).is_some_and(|(_, _, residents)| { p.village_count = residents; residents >= 1 }),
        _ => look(p, 5).is_some_and(|(hired, _, _)| hired),
    }
}

/// Uma fala do prefeito.
pub(super) fn say(host: &mut dyn Host, player: u128, key: &str) {
    let name = Text::tr("veio.onboarding.mayor.name", &[]).plain();
    host.tell(player, Text::tr("veio.agenda.says", &[&name, &Text::tr(key, &[]).plain()]));
}

/// Anuncia a etapa: o prefeito conta por quê, a ficha da receita (liberada no livro de receitas)
/// e os passos numerados.
pub(super) fn announce(host: &mut dyn Host, player: &NativeServerPlayer, step: usize) {
    let uuid = player.uuid;
    host.tell(uuid, Text::tr(&format!("veio.village.title.{step}"), &[]));
    say(host, uuid, &format!("veio.village.why.{step}"));
    let (block, ingredients) = PARTS[step];
    if !ingredients.is_empty() {
        host.tell(uuid, Text::tr(&format!("veio.village.recipe.{step}"), &[]));
        host.tell(uuid, Text::tr(&format!("veio.village.from.{step}"), &[]));
        if spawn_safe(&player.name) { host.run_command(&format!("recipe give {} {block}", player.name)); }
        host.tell(uuid, Text::tr("veio.onboarding.book", &[]));
    }
    for line in 1..=DO_LINES[step] {
        host.tell(uuid, Text::tr(&format!("veio.village.do.{step}.{line}"), &[]));
    }
}

/// A barra da etapa: lista de materiais enquanto o bloco não foi fabricado; depois o objetivo,
/// com distância e rumo até o bloco quando o jogador está longe dele.
pub(super) fn bar(host: &mut dyn Host, player: &NativeServerPlayer, p: &mut Progress) {
    let Some(step) = step(p) else { return };
    let (block, ingredients) = PARTS[step];
    let bit = 1u16 << step;
    if !ingredients.is_empty() && p.village_crafted & bit == 0 && p.village_blocks[slot(block).unwrap_or(0)].is_none() {
        if has_item(host, &player.name, block) {
            p.village_crafted |= bit;
            p.dirty = true;
        } else if materials(host, player, block, ingredients) {
            return;
        }
    }
    let goal = Text::tr(&format!("veio.village.goal.{step}"), &[&p.village_count]).plain();
    let text = match p.village_blocks[focus(step)] {
        Some(pos) => {
            let target = [f64::from(pos[0]) + 0.5, f64::from(pos[1]), f64::from(pos[2]) + 0.5];
            let (distance, dir) = intro::pointer(player, target);
            if distance > 12 { Text::tr("veio.village.far", &[&goal, &distance, &dir]) } else { Text::literal(goal) }
        }
        None => Text::literal(goal),
    };
    let mut bar = Text::tr("veio.village.bar", &[&(step + 1), &text.plain()]).plain();
    // A festa marcada soma a contagem: "Festa da Colheita em 2 dias".
    if let Some(when) = super::festival::countdown(host, p) { bar = format!("{bar} · {}", when.plain()); }
    host.action_bar(player.uuid, Text::literal(&bar));
}

/// Faíscas no lugar da etapa: em volta do posto, a área onde o marco pode ficar (etapa da zona),
/// e sobre o bloco que espera ação (contratar, colheita).
pub(super) fn marks(host: &mut dyn Host, player: &NativeServerPlayer, p: &Progress, now: u64) {
    let Some(step) = step(p) else { return };
    if now % 40 != 0 || player.dimension != OVERWORLD_DIM { return; }
    // Na etapa da zona, o centro das faíscas é o posto (onde o marco pode ficar em volta).
    let Some(pos) = p.village_blocks[if step == 2 { 0 } else { focus(step) }] else { return };
    let center = [f64::from(pos[0]) + 0.5, f64::from(pos[1]), f64::from(pos[2]) + 0.5];
    if (player.position[0] - center[0]).hypot(player.position[2] - center[2]) > 40.0 { return; }
    if step == 2 {
        // Onde o marco pode ficar: o quadrado em volta do posto.
        let reach = MARKER_REACH;
        for offset in (-reach..=reach).step_by(2) {
            for (dx, dz) in [(offset, -reach), (offset, reach), (-reach, offset), (reach, offset)] {
                host.send_particles(&ParticleOptions::simple(ParticleType::HappyVillager), None,
                    [center[0] + f64::from(dx), center[1] + 0.3, center[2] + f64::from(dz)], [0.0, 0.2, 0.0], 0.0, 1);
            }
        }
    } else {
        host.send_particles(&ParticleOptions::simple(ParticleType::HappyVillager), None,
            [center[0], center[1] + 1.4, center[2]], [0.3, 0.3, 0.3], 0.0, 4);
    }
}

/// Etapa concluída: o prefeito comenta, título, som e experiência.
pub(super) fn celebrate(host: &mut dyn Host, player: &NativeServerPlayer, finished: usize) {
    say(host, player.uuid, &format!("veio.village.done.{finished}"));
    if !spawn_safe(&player.name) { return; }
    let name = &player.name;
    let next = if finished + 1 < usize::from(STEPS) { format!("veio.village.title.{}", finished + 1) } else { "veio.village.end_sub".into() };
    host.run_command(&format!(r#"title {name} subtitle {{"translate":"{next}"}}"#));
    host.run_command(&format!(r#"title {name} title {{"translate":"veio.onboarding.done","color":"gold"}}"#));
    host.run_command(&format!("execute as {name} at @s run playsound minecraft:ui.toast.challenge_complete master @s"));
    host.run_command(&format!("xp add {name} {} points", 20 + 5 * finished));
}

/// Um passo do capítulo, a cada segundo, para um jogador online.
pub(super) fn tick(host: &mut dyn Host, player: &NativeServerPlayer, p: &mut Progress, now: u64) {
    let Some(step) = step(p) else { return };
    if player.dimension == OVERWORLD_DIM && ready(host, p) {
        celebrate(host, player, step);
        p.village += 1;
        p.village_count = 0;
        p.dirty = true;
        // A colheita conta do que o posto já tinha entregue quando a etapa começou.
        if p.village == 4 && let Some(pos) = p.village_blocks[0] && let Some((_, produced, _)) = read(host, pos, workforce::FARM_POST) {
            p.village_base = produced;
        }
        if p.village == DONE_VILLAGE {
            finish(host, player);
            host.action_bar(player.uuid, Text::literal(""));
            return;
        }
    }
    if p.hidden { return; }
    if p.village_announced != Some(p.village) {
        if let Some(step) = step_of(p.village) { announce(host, player, step); }
        p.village_announced = Some(p.village);
    }
    marks(host, player, p, now);
    bar(host, player, p);
}

fn step_of(village: u8) -> Option<usize> { (1..=STEPS).contains(&village).then(|| usize::from(village - 1)) }

/// Fim do capítulo: o prefeito fecha e aponta os próximos passos da vila.
fn finish(host: &mut dyn Host, player: &NativeServerPlayer) {
    for line in 1..=4 { say(host, player.uuid, &format!("veio.village.end.{line}")); }
    if spawn_safe(&player.name) {
        let name = &player.name;
        host.run_command(&format!(r#"title {name} subtitle {{"translate":"veio.village.end_sub"}}"#));
        host.run_command(&format!(r#"title {name} title {{"translate":"veio.village.end_title","color":"gold"}}"#));
        host.run_command(&format!("xp add {name} 80 points"));
    }
}

/// Clique no prefeito depois da fábrica: começa o capítulo, repete a etapa ou fecha.
pub(super) fn talk(host: &mut dyn Host, player: &NativeServerPlayer, p: &mut Progress) {
    match p.village {
        0 => start(host, player, p),
        _ => {
            if let Some(step) = step(p) {
                say(host, player.uuid, "veio.village.again");
                announce(host, player, step);
            }
        }
    }
}

/// Começa o capítulo (prefeito ou `/veiotutorial vila`).
pub(super) fn start(host: &mut dyn Host, player: &NativeServerPlayer, p: &mut Progress) {
    for line in 1..=3 { say(host, player.uuid, &format!("veio.village.intro.{line}")); }
    if spawn_safe(&player.name) {
        let name = &player.name;
        host.run_command(&format!(r#"title {name} subtitle {{"translate":"veio.village.start_sub"}}"#));
        host.run_command(&format!(r#"title {name} title {{"translate":"veio.village.start_title","color":"gold"}}"#));
    }
    p.village = 1;
    p.village_announced = None;
    p.village_blocks = [None; 6];
    p.village_crafted = 0;
    p.village_count = 0;
    p.hidden = false;
    p.dirty = true;
}

/// Lista de materiais do bloco da etapa (a mesma do capítulo da fábrica).
pub(super) fn materials(host: &mut dyn Host, player: &NativeServerPlayer, block: &str, ingredients: &[&str]) -> bool {
    if !spawn_safe(&player.name) { return false; }
    let mut line = vec![format!(r#"{{"translate":"veio.onboarding.make","with":[{{"translate":"{}"}}],"color":"gold"}}"#, name_key(block))];
    for item in ingredients {
        let (mark, color) = if has_item(host, &player.name, item) { ("✔", "green") } else { ("✘", "red") };
        line.push(format!(r#"{{"text":"  {mark} ","color":"{color}"}},{{"translate":"{}","color":"{color}"}}"#, name_key(item)));
    }
    host.run_command(&format!(r#"title {} actionbar ["",{}]"#, player.name, line.join(",")))
}
