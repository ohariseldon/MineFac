//! Capítulo 6 do tutorial: a cidade (`frentes/HISTORIA.md`), de Vila a Cidade.
//!
//! Com a indústria de pé, a vila cresce em gente e em serviço: nove famílias, emprego, casas com
//! os quatro serviços, uma obra cívica, o PIB e a fase Cidade. Lê a vila como o capítulo 4
//! (`city::survey`), com as metas da fase Cidade (`mayor::PHASES[1]`).
//! `Progress::metro`: 0 não começou, 1..=6 em andamento, 7 concluído.
use super::*;
use chapter::Spec;
use city::Town;

pub(super) const SPEC: Spec = Spec {
    key: "metro",
    steps: 6,
    do_lines: &[4, 4, 4, 4, 4, 4],
    parts: &[("", &[]), ("", &[]), ("", &[]), ("", &[]), ("", &[]), ("", &[])],
    intro: 3,
    end: 4,
    xp: 60,
};

/// Metas da fase Cidade: moradores, famílias, IDH (centésimos), PIB por dia e emprego (%).
pub(super) const RESIDENTS: i32 = 36;
pub(super) const FAMILIES: i32 = 9;
pub(super) const IDH: i32 = 55;
pub(super) const GDP: i32 = 400;
pub(super) const EMPLOYMENT: i32 = 60;
/// A fase Cidade é a 3 (Povoado 1, Vila 2, Cidade 3).
pub(super) const CITY_PHASE: i32 = 3;

pub(super) fn step(p: &Progress) -> Option<usize> { chapter::step(&SPEC, p.metro) }

fn employment(town: &Town) -> i32 { if town.adults == 0 { 0 } else { town.employed * 100 / town.adults } }

pub(super) fn done(step: usize, town: &Town) -> bool {
    match step {
        0 => town.families >= FAMILIES,
        1 => employment(town) >= EMPLOYMENT,
        2 => town.homes > 0 && town.serviced == town.homes,
        // Obra cívica feita, ou nenhum bairro sem lazer (aí o prefeito nem pede obra).
        3 => town.civic || (town.homes >= FAMILIES && town.entertained == town.homes),
        4 => town.gdp >= GDP,
        _ => town.phase >= CITY_PHASE,
    }
}

/// As condições da etapa, para a lista ✔/✘ (vazia: a barra mostra um texto).
pub(super) fn checks(step: usize, town: &Town) -> Vec<(bool, &'static str, Vec<String>)> {
    let idh = format!("{},{:02}", town.idh / 100, town.idh % 100);
    match step {
        2 => vec![
            (town.sanitized == town.homes && town.homes > 0, "veio.metro.check.water", vec![town.sanitized.to_string(), town.homes.to_string()]),
            (town.powered == town.homes && town.homes > 0, "veio.metro.check.power", vec![town.powered.to_string(), town.homes.to_string()]),
            (town.serviced == town.homes && town.homes > 0, "veio.metro.check.all", vec![town.serviced.to_string(), town.homes.to_string()]),
        ],
        5 => vec![
            (town.staffed, "veio.city.check.clerk", vec![]),
            (town.residents >= RESIDENTS, "veio.metro.check.residents", vec![town.residents.to_string()]),
            (town.families >= FAMILIES, "veio.metro.check.families", vec![town.families.to_string()]),
            (town.idh >= IDH, "veio.metro.check.idh", vec![idh]),
            (town.gdp >= GDP, "veio.metro.check.gdp", vec![town.gdp.to_string()]),
            (employment(town) >= EMPLOYMENT, "veio.metro.check.jobs", vec![employment(town).to_string()]),
        ],
        _ => Vec::new(),
    }
}

/// O texto da barra quando a etapa tem uma condição só.
pub(super) fn goal(step: usize, town: &Town) -> Text {
    match step {
        0 => Text::tr("veio.metro.goal.0", &[&town.families, &town.residents]),
        1 => Text::tr("veio.metro.goal.1", &[&employment(town), &town.employed, &town.adults]),
        3 => match &town.request {
            Some(request) => Text::tr("veio.metro.goal.3", &[request]),
            None => Text::tr("veio.metro.goal.3_wait", &[&town.entertained, &town.homes]),
        },
        4 => Text::tr("veio.metro.goal.4", &[&town.gdp]),
        _ => Text::tr(&format!("veio.metro.goal.{step}"), &[]),
    }
}

/// Um passo do capítulo, a cada segundo. `now` é o relógio dos registros da sociedade.
pub(super) fn tick(host: &mut dyn Host, player: &NativeServerPlayer, p: &mut Progress, now: u64) {
    let Some(current) = step(p) else { return };
    let town = city::survey(host, &player.dimension, city::anchor(p, player, now), now);
    if done(current, &town) {
        chapter::celebrate(host, player, &SPEC, current);
        p.metro += 1;
        p.dirty = true;
        if p.metro == SPEC.done() { chapter::finish(host, player, &SPEC); return; }
    }
    if p.hidden { return; }
    let Some(current) = step(p) else { return };
    if p.metro_announced != Some(p.metro) {
        chapter::announce(host, player, &SPEC, current);
        p.metro_announced = Some(p.metro);
    }
    let checks = checks(current, &town);
    if checks.is_empty() {
        chapter::bar(host, player, &SPEC, current, &goal(current, &town).plain());
    } else {
        chapter::checklist(host, player, &SPEC, current, &checks);
    }
}

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
    p.metro = 1;
    p.metro_announced = None;
    p.hidden = false;
    p.dirty = true;
}
