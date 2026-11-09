//! Capítulo 5 do tutorial: a grande indústria (`frentes/HISTORIA.md`).
//!
//! Para atender a vila, a fábrica precisa de aço, circuitos e da extratora Mk.II; produzir mais
//! polui mais, e a poluição traz as ondas de Saqueadores. O capítulo leva a fábrica até a
//! eletrônica e monta a defesa que o conselho do capítulo 2 prometeu, com os gargalos dela:
//! quartel (soldado e comida), torre Tesla carregada, acumulador para a noite e kit de reparo.
//!
//! As etapas fecham pelo que o jogo registra: a fase do Quadro da fábrica, os blocos que o
//! jogador coloca (Mk.II, quartel, Tesla, acumulador), o estado deles (soldado, comida, energia),
//! o aviso de poluição e o kit no inventário.
//! `Progress::forge`: 0 não começou, 1..=8 em andamento, 9 concluído.
use super::*;
use chapter::Spec;

pub(super) const SPEC: Spec = Spec {
    key: "forge",
    steps: 8,
    do_lines: &[4, 4, 4, 4, 4, 4, 4, 3],
    parts: &[
        ("", &[]),
        ("", &[]),
        (DRILL_MK2, &[DRILL, "veio:steel_plate", "veio:circuit", "veio:module"]),
        ("", &[]),
        (workforce::BARRACKS, &["minecraft:iron_ingot", "minecraft:emerald", "minecraft:smooth_stone"]),
        (TESLA, &["veio:circuit", "veio:steel_plate", "minecraft:lightning_rod", "minecraft:copper_block"]),
        (BATTERY, &["veio:steel_plate", "minecraft:redstone", "veio:wire"]),
        ("veio:repair_kit", &["veio:iron_plate", "veio:gear"]),
    ],
    intro: 3,
    end: 4,
    xp: 40,
    // Bento (o ferreiro, filho de mineiro), Mestre Lumen (o vidreiro), Dona Cida, Seu Anselmo.
    voices: &["veio.favor.smith.name", "veio.agenda.lights.visitor", "veio.favor.smith.name", "veio.favor.farmer.name",
        "veio.agenda.founding.visitor", "veio.favor.smith.name", "", "veio.favor.smith.name"],
};

/// Fases do Quadro da fábrica concluídas que as primeiras etapas pedem: Oficina e Aço (3),
/// Eletrônica (4).
pub(super) const STEEL_PHASE: usize = 3;
pub(super) const ELECTRONICS_PHASE: usize = 4;

pub(super) fn step(p: &Progress) -> Option<usize> { chapter::step(&SPEC, p.forge) }

/// Onde o capítulo guarda cada bloco: Mk.II, quartel, Tesla, acumulador.
pub(super) fn slot(block: &str) -> Option<usize> {
    Some(match block {
        DRILL_MK2 => 0,
        workforce::BARRACKS => 1,
        TESLA => 2,
        BATTERY => 3,
        _ => return None,
    })
}

pub(super) fn placed(p: &mut Progress, dimension: &str, pos: Pos, block: &str) {
    let Some(index) = slot(block) else { return };
    if step(p).is_none() || dimension != workforce::spawn::OVERWORLD { return; }
    p.forge_blocks[index] = Some(pos);
    p.dirty = true;
}

/// O que o jogo mostra agora para o capítulo.
#[derive(Clone, Debug, Default, PartialEq)]
pub(super) struct Works {
    /// Fases do Quadro concluídas (o Quadro mais adiantado da dimensão).
    pub phase: Option<usize>,
    pub mk2: bool,
    /// Quartel: colocado, com soldado, com comida.
    pub barracks: bool,
    pub soldier: bool,
    pub rations: bool,
    /// Tesla colocada e a energia guardada nela; acumulador colocado e carregado.
    pub tesla: Option<i32>,
    pub battery: Option<i32>,
    /// O chunk mais poluído, se o aviso já veio e quantas ondas vieram.
    pub peak: i32,
    pub warned: bool,
    pub waves: u32,
    pub kit: bool,
}

fn read(host: &mut dyn Host, pos: Option<Pos>, block: &str) -> Option<ModBlockEntity> {
    let pos = pos?;
    let mut out = None;
    host.change_mod_block_entity(pos, &mut |entity| if entity.id == block { out = Some(entity.clone()); });
    out
}

pub(super) fn survey(host: &mut dyn Host, player: &NativeServerPlayer, p: &mut Progress) -> Works {
    let dimension = player.dimension.as_str();
    let (peak, warned, waves) = pollution::alarm(dimension);
    let mut works = Works { phase: milestones::census::board_phase(dimension), peak: peak.round() as i32, warned, waves, ..Works::default() };
    for (index, block) in [DRILL_MK2, workforce::BARRACKS, TESLA, BATTERY].into_iter().enumerate() {
        let entity = read(host, p.forge_blocks[index], block);
        if entity.is_none() && p.forge_blocks[index].is_some() { p.forge_blocks[index] = None; p.dirty = true; }
        let Some(entity) = entity else { continue };
        match index {
            0 => works.mk2 = true,
            1 => {
                works.barracks = true;
                works.soldier = workforce::worker(&entity).is_some();
                works.rations = entity.items.first().is_some_and(Option::is_some);
            }
            2 => works.tesla = Some(number(&entity, "energy")),
            _ => works.battery = Some(number(&entity, "energy")),
        }
    }
    if step(p) == Some(7) && spawn_safe(&player.name) { works.kit = has_item(host, &player.name, "veio:repair_kit"); }
    works
}

pub(super) fn done(step: usize, works: &Works) -> bool {
    let phase = works.phase.unwrap_or(0);
    match step {
        0 => phase >= STEEL_PHASE,
        1 => phase >= ELECTRONICS_PHASE,
        2 => works.mk2,
        3 => works.warned,
        4 => works.barracks && works.soldier && works.rations,
        5 => works.tesla.is_some_and(|energy| energy >= defense::SHOT_COST),
        6 => works.battery.is_some_and(|energy| energy > 0),
        _ => works.kit,
    }
}

/// A barra da etapa: lista ✔/✘ quando há várias condições, texto quando há uma.
fn show(host: &mut dyn Host, player: &NativeServerPlayer, p: &mut Progress, step: usize, works: &Works) {
    let mut crafted = p.forge_crafted;
    let shown = chapter::materials(host, player, &SPEC, step, &mut crafted);
    if crafted != p.forge_crafted { p.forge_crafted = crafted; p.dirty = true; }
    if shown { return; }
    let phase = works.phase.unwrap_or(0);
    match step {
        0 => chapter::checklist(host, player, &SPEC, step, &[
            (works.phase.is_some(), "veio.forge.check.board", vec![]),
            (phase >= 2, "veio.forge.check.workshop", vec![]),
            (phase >= STEEL_PHASE, "veio.forge.check.steel", vec![]),
        ]),
        1 => chapter::bar(host, player, &SPEC, step, &Text::tr("veio.forge.goal.1", &[]).plain()),
        3 => chapter::bar(host, player, &SPEC, step,
            &Text::tr("veio.forge.goal.3", &[&works.peak, &(pollution::NOTICE as i32)]).plain()),
        4 => chapter::checklist(host, player, &SPEC, step, &[
            (works.barracks, "veio.forge.check.barracks", vec![]),
            (works.soldier, "veio.forge.check.soldier", vec![]),
            (works.rations, "veio.forge.check.rations", vec![]),
        ]),
        5 => {
            let energy = works.tesla.unwrap_or(0);
            chapter::checklist(host, player, &SPEC, step, &[
                (works.tesla.is_some(), "veio.forge.check.tesla", vec![]),
                (energy >= defense::SHOT_COST, "veio.forge.check.charge",
                    vec![(energy / defense::SHOT_COST).to_string(), (defense::CAPACITY / defense::SHOT_COST).to_string()]),
            ]);
        }
        6 => chapter::checklist(host, player, &SPEC, step, &[
            (works.battery.is_some(), "veio.forge.check.battery", vec![]),
            (works.battery.is_some_and(|energy| energy > 0), "veio.forge.check.stored", vec![works.battery.unwrap_or(0).to_string()]),
        ]),
        _ => chapter::bar(host, player, &SPEC, step, &Text::tr(&format!("veio.forge.goal.{step}"), &[]).plain()),
    }
}

/// Um passo do capítulo, a cada segundo.
pub(super) fn tick(host: &mut dyn Host, player: &NativeServerPlayer, p: &mut Progress) {
    let Some(current) = step(p) else { return };
    let works = survey(host, player, p);
    if done(current, &works) {
        chapter::celebrate(host, player, &SPEC, current);
        p.forge += 1;
        p.dirty = true;
        if p.forge == SPEC.done() { chapter::finish(host, player, &SPEC); return; }
    }
    if p.hidden { return; }
    let Some(current) = step(p) else { return };
    if p.forge_announced != Some(p.forge) {
        chapter::announce(host, player, &SPEC, current);
        p.forge_announced = Some(p.forge);
    }
    // Uma onda que vem no meio do capítulo vira lição: o prefeito comenta uma vez.
    if works.waves > 0 && p.forge_flags & WAVE_SEEN == 0 {
        p.forge_flags |= WAVE_SEEN;
        p.dirty = true;
        village::say(host, player.uuid, "veio.forge.wave");
    }
    show(host, player, p, current, &works);
}

/// Bits de `Progress::forge_flags`.
pub(super) const WAVE_SEEN: u8 = 1;

pub(super) fn talk(host: &mut dyn Host, player: &NativeServerPlayer, p: &mut Progress) {
    match step(p) {
        None => start(host, player, p),
        Some(current) => {
            village::say(host, player.uuid, "veio.village.again");
            chapter::announce(host, player, &SPEC, current);
        }
    }
}

pub(super) fn start(host: &mut dyn Host, player: &NativeServerPlayer, p: &mut Progress) {
    chapter::open(host, player, &SPEC);
    p.forge = 1;
    p.forge_announced = None;
    p.forge_crafted = 0;
    p.forge_blocks = [None; 4];
    p.hidden = false;
    p.dirty = true;
}
