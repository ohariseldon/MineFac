//! Capítulo 3 do tutorial: a cidade, contada pelo prefeito Otávio.
//!
//! Depois da vila operária (capítulo 2), o prefeito ensina o que faz uma vila virar cidade:
//! serviços (poço, energia, lazer), comércio (loja e a primeira venda), a conta da companhia que
//! paga os salários e a primeira fase do plano diretor. Cada etapa fecha pelo que a própria
//! sociedade registra em volta da vila (casas, lojas, contas e prefeitura), não por blocos
//! guardados pelo tutorial: uma casa com saneamento, uma loja com comerciante e caixa, a conta
//! da companhia, casas com energia e lazer, três famílias e a fase "Vila".
//!
//! Começa ao falar com o prefeito depois do capítulo da vila, ou com `/veiotutorial cidade`.
//! `Progress::city`: 0 não começou, 1..=8 é a etapa 0..=7 em andamento, 9 concluído.
use super::*;
use workforce::society::{self, ledger::Book};

pub(super) const STEPS: u8 = 8;
pub(super) const DONE_CITY: u8 = STEPS + 1;
/// Linhas "faça" de cada etapa (`veio.city.do.<etapa>.<n>`).
pub(super) const DO_LINES: [u8; 8] = [4, 4, 4, 4, 3, 4, 4, 4];
/// Famílias da etapa 7 (a mesma conta da fase Vila).
pub(super) const FAMILIES: i32 = 3;

/// O bloco que cada etapa pede (ficha, livro de receitas e lista de materiais); vazio quando a
/// etapa não pede bloco novo.
pub(super) const PARTS: [(&str, &[&str]); 8] = [
    (society::WELL, &["minecraft:cobblestone", "minecraft:bucket"]),
    (society::SHOP, &["minecraft:barrel", "#minecraft:planks", "minecraft:emerald"]),
    ("", &[]),
    ("", &[]),
    ("veio:pole", &["minecraft:stick", "minecraft:copper_ingot"]),
    (society::agenda::PARK, &["minecraft:oak_sapling", "minecraft:poppy", "minecraft:dandelion", "minecraft:oak_fence", "minecraft:lantern"]),
    (society::HOUSE, &["#minecraft:planks", "minecraft:oak_door", "minecraft:glass_pane"]),
    ("", &[]),
];

/// A etapa (0..8) em andamento, se o capítulo está aberto.
pub(super) fn step(p: &Progress) -> Option<usize> {
    (1..=STEPS).contains(&p.city).then(|| usize::from(p.city - 1))
}

/// O que a vila mostra agora, em volta do lugar dela.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) struct Town {
    pub sanitation: bool,
    pub merchant: bool,
    pub sales: bool,
    pub company: bool,
    pub power: bool,
    pub leisure: bool,
    pub families: i32,
    pub residents: i32,
    /// IDH médio das famílias, em centésimos.
    pub idh: i32,
    pub phase: i32,
}

/// O centro da vila para o capítulo: a prefeitura que governa o jogador, a do capítulo 2, a casa
/// do capítulo 2 ou onde o jogador está.
fn anchor(p: &Progress, player: &NativeServerPlayer, now: u64) -> [i32; 3] {
    let at = player.position.map(|v| v.floor() as i32);
    society::governing(&player.dimension, at, now).map(|hall| hall.pos)
        .or(p.village_blocks[5]).or(p.village_blocks[4]).unwrap_or(at)
}

/// Lê a vila pelos registros da sociedade (casas, lojas, contas e prefeitura).
pub(super) fn survey(dimension: &str, at: [i32; 3], now: u64) -> Town {
    let homes = society::homes_near(dimension, at, society::HALL_RADIUS, now);
    let shops = society::shops_near(dimension, at, society::HALL_RADIUS, now);
    let families: Vec<_> = homes.iter().filter(|home| home.residents > 0).collect();
    let idh = if families.is_empty() { 0 } else { families.iter().map(|home| home.idh()).sum::<i32>() / families.len() as i32 };
    Town {
        sanitation: homes.iter().any(|home| home.sanitation),
        merchant: shops.iter().any(|shop| shop.merchant.is_some()),
        sales: shops.iter().any(|shop| shop.cash > 0),
        company: society::ledger::total(Book::Company, dimension) > 0,
        power: homes.iter().any(|home| home.power),
        leisure: homes.iter().any(|home| home.leisure),
        families: families.len() as i32,
        residents: families.iter().map(|home| home.residents).sum(),
        idh,
        phase: society::governing(dimension, at, now).map_or(0, |hall| hall.phase),
    }
}

/// A etapa está cumprida nesta leitura da vila?
pub(super) fn done(step: usize, town: &Town) -> bool {
    match step {
        0 => town.sanitation,
        1 => town.merchant,
        2 => town.sales,
        3 => town.company,
        4 => town.power,
        5 => town.leisure,
        6 => town.families >= FAMILIES,
        _ => town.phase >= 2,
    }
}

/// Anuncia a etapa: o prefeito conta por quê, a ficha da receita (liberada no livro) e os passos.
fn announce(host: &mut dyn Host, player: &NativeServerPlayer, step: usize) {
    let uuid = player.uuid;
    host.tell(uuid, Text::tr(&format!("veio.city.title.{step}"), &[]));
    village::say(host, uuid, &format!("veio.city.why.{step}"));
    let (block, ingredients) = PARTS[step];
    if !ingredients.is_empty() {
        host.tell(uuid, Text::tr(&format!("veio.city.recipe.{step}"), &[]));
        host.tell(uuid, Text::tr(&format!("veio.city.from.{step}"), &[]));
        if spawn_safe(&player.name) { host.run_command(&format!("recipe give {} {block}", player.name)); }
        host.tell(uuid, Text::tr("veio.onboarding.book", &[]));
    }
    for line in 1..=DO_LINES[step] {
        host.tell(uuid, Text::tr(&format!("veio.city.do.{step}.{line}"), &[]));
    }
}

/// A barra: lista de materiais enquanto o bloco da etapa não foi fabricado; depois o objetivo,
/// com os números da vila que a etapa acompanha.
fn bar(host: &mut dyn Host, player: &NativeServerPlayer, p: &mut Progress, town: &Town) {
    let Some(step) = step(p) else { return };
    let (block, ingredients) = PARTS[step];
    let bit = 1u16 << step;
    if !ingredients.is_empty() && p.city_crafted & bit == 0 {
        if has_item(host, &player.name, block) {
            p.city_crafted |= bit;
            p.dirty = true;
        } else if village::materials(host, player, block, ingredients) {
            return;
        }
    }
    let idh = format!("{},{:02}", town.idh / 100, town.idh % 100);
    let goal = match step {
        6 => Text::tr("veio.city.goal.6", &[&town.families]),
        7 => Text::tr("veio.city.goal.7", &[&town.residents, &town.families, &idh]),
        _ => Text::tr(&format!("veio.city.goal.{step}"), &[]),
    };
    host.action_bar(player.uuid, Text::tr("veio.city.bar", &[&(step + 1), &goal.plain()]));
}

/// Etapa concluída: o prefeito comenta, título, som e experiência.
fn celebrate(host: &mut dyn Host, player: &NativeServerPlayer, finished: usize) {
    village::say(host, player.uuid, &format!("veio.city.done.{finished}"));
    if !spawn_safe(&player.name) { return; }
    let name = &player.name;
    let next = if finished + 1 < usize::from(STEPS) { format!("veio.city.title.{}", finished + 1) } else { "veio.city.end_sub".into() };
    host.run_command(&format!(r#"title {name} subtitle {{"translate":"{next}"}}"#));
    host.run_command(&format!(r#"title {name} title {{"translate":"veio.onboarding.done","color":"gold"}}"#));
    host.run_command(&format!("playsound minecraft:ui.toast.challenge_complete master {name}"));
    host.run_command(&format!("xp add {name} {} points", 30 + 5 * finished));
}

/// Um passo do capítulo, a cada segundo, para um jogador online. `now` é o relógio dos
/// registros da sociedade (`defense::clock`).
pub(super) fn tick(host: &mut dyn Host, player: &NativeServerPlayer, p: &mut Progress, now: u64) {
    let Some(step) = step(p) else { return };
    let town = survey(&player.dimension, anchor(p, player, now), now);
    if done(step, &town) {
        celebrate(host, player, step);
        p.city += 1;
        p.dirty = true;
        if p.city == DONE_CITY {
            finish(host, player);
            host.action_bar(player.uuid, Text::literal(""));
            return;
        }
    }
    if p.hidden { return; }
    if p.city_announced != Some(p.city) {
        if let Some(step) = self::step(p) { announce(host, player, step); }
        p.city_announced = Some(p.city);
    }
    bar(host, player, p, &town);
}

/// Fim do capítulo: o prefeito fecha e aponta o caminho até a Metrópole.
fn finish(host: &mut dyn Host, player: &NativeServerPlayer) {
    for line in 1..=4 { village::say(host, player.uuid, &format!("veio.city.end.{line}")); }
    if spawn_safe(&player.name) {
        let name = &player.name;
        host.run_command(&format!(r#"title {name} subtitle {{"translate":"veio.city.end_sub"}}"#));
        host.run_command(&format!(r#"title {name} title {{"translate":"veio.city.end_title","color":"gold"}}"#));
        host.run_command(&format!("xp add {name} 120 points"));
    }
}

/// Clique no prefeito: começa o capítulo ou repete a etapa.
pub(super) fn talk(host: &mut dyn Host, player: &NativeServerPlayer, p: &mut Progress) {
    match step(p) {
        None => start(host, player, p),
        Some(step) => {
            village::say(host, player.uuid, "veio.village.again");
            announce(host, player, step);
        }
    }
}

/// Começa o capítulo (prefeito ou `/veiotutorial cidade`).
pub(super) fn start(host: &mut dyn Host, player: &NativeServerPlayer, p: &mut Progress) {
    for line in 1..=3 { village::say(host, player.uuid, &format!("veio.city.intro.{line}")); }
    if spawn_safe(&player.name) {
        let name = &player.name;
        host.run_command(&format!(r#"title {name} subtitle {{"translate":"veio.city.start_sub"}}"#));
        host.run_command(&format!(r#"title {name} title {{"translate":"veio.city.start_title","color":"gold"}}"#));
    }
    p.city = 1;
    p.city_announced = None;
    p.city_crafted = 0;
    p.hidden = false;
    p.dirty = true;
}
