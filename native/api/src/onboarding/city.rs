//! Capítulo 3 do tutorial: a cidade, contada pelo prefeito Otávio.
//!
//! Depois da vila operária (capítulo 2), o prefeito ensina o que faz uma vila virar cidade:
//! serviços (poço, energia, lazer), comércio (loja e a primeira venda), o dinheiro (a conta da
//! companhia que paga os salários e a conta pública que vive do imposto), a agenda (uma festa
//! pedida por um visitante) e a primeira fase do plano diretor.
//!
//! Cada etapa fecha pelo que a própria sociedade registra em volta da vila (casas, lojas, contas,
//! prefeitura e os pedidos da agenda), não por blocos guardados pelo tutorial. A barra diz o que
//! falta com os números da vila (casas atendidas, famílias, o pedido aberto, a parte mais fraca
//! do IDH) e aponta para a loja ou a prefeitura quando a etapa depende delas. Se o jogador fica
//! parado numa etapa, o prefeito percebe e dá uma dica.
//!
//! Começa ao falar com o prefeito depois do capítulo da vila, ou com `/veiotutorial cidade`.
//! `Progress::city`: 0 não começou, 1..=10 é a etapa 0..=9 em andamento, 11 concluído.
use super::*;
use workforce::society::{self, agenda, ledger::Book};

pub(super) const STEPS: u8 = 10;
pub(super) const DONE_CITY: u8 = STEPS + 1;
/// Linhas "faça" de cada etapa (`veio.city.do.<etapa>.<n>`).
pub(super) const DO_LINES: [u8; 10] = [4, 4, 4, 4, 3, 4, 4, 4, 4, 4];
/// Famílias da etapa do bairro (a mesma conta da fase Vila).
pub(super) const FAMILIES: i32 = 3;
/// Parado na mesma etapa por isto (20 min), a barra avisa, discreta, que o prefeito tem uma dica.
/// Ele não fala sozinho: a dica sai quando o jogador clica nele.
pub(super) const STUCK: u64 = 24_000;
/// Metas da fase Vila (as mesmas de `mayor::PHASES[0]`).
const VILLAGE_RESIDENTS: i32 = 12;
const VILLAGE_IDH: i32 = 40;

/// Mais longe que isto da loja ou da prefeitura, a barra mostra a seta.
const FAR: i32 = 12;
/// Depois de uma etapa concluída, a próxima espera isto (3 s) para não chegar junto com a festa.
const BREATH: u64 = 60;

/// O bloco que cada etapa pede (ficha, livro de receitas e lista de materiais); vazio quando a
/// etapa não pede bloco novo.
pub(super) const PARTS: [(&str, &[&str]); 10] = [
    (society::WELL, &["minecraft:cobblestone", "minecraft:bucket"]),
    (society::SHOP, &["minecraft:barrel", "#minecraft:planks", "minecraft:emerald"]),
    ("", &[]),
    ("", &[]),
    ("veio:pole", &["minecraft:stick", "minecraft:copper_ingot"]),
    (agenda::PARK, &["minecraft:oak_sapling", "minecraft:poppy", "minecraft:dandelion", "minecraft:oak_fence", "minecraft:lantern"]),
    (society::HOUSE, &["#minecraft:planks", "minecraft:oak_door", "minecraft:glass_pane"]),
    ("", &[]),
    ("", &[]),
    ("", &[]),
];

/// A etapa (0..10) em andamento, se o capítulo está aberto.
pub(super) fn step(p: &Progress) -> Option<usize> {
    (1..=STEPS).contains(&p.city).then(|| usize::from(p.city - 1))
}

/// O que a vila mostra agora, em volta do lugar dela.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(super) struct Town {
    /// Casas com moradores, e quantas delas têm saneamento, energia e lazer.
    pub homes: i32,
    pub sanitized: i32,
    pub powered: i32,
    pub entertained: i32,
    pub merchant: bool,
    pub sales: bool,
    pub company: bool,
    pub public: bool,
    pub families: i32,
    pub residents: i32,
    /// IDH médio das famílias, em centésimos, e a dimensão mais fraca (0 comida, 1 moradia,
    /// 2 serviços, 3 renda).
    pub idh: i32,
    pub weakest: Option<usize>,
    pub phase: i32,
    /// Pedido da agenda já preparado e o texto do pedido aberto mais próximo.
    pub prepared: bool,
    pub request: Option<String>,
    /// Onde apontar: a loja (sem comerciante ou sem venda) e a prefeitura.
    pub shop: Option<[i32; 3]>,
    pub hall: Option<[i32; 3]>,
    /// O que a primeira venda pede ao mesmo tempo: comerciante no balcão, dia e famílias com
    /// dinheiro na carteira.
    pub open_shop: bool,
    pub day: bool,
    pub wallets: bool,
    /// Saldos das contas, em moedas.
    pub company_coins: i64,
    pub public_coins: i64,
    /// A prefeitura tem funcionário (a fase só sobe com ele).
    pub staffed: bool,
    /// A próxima festa do calendário (evento) e em quantos dias o visitante dela chega.
    pub next_visitor: Option<(usize, i64)>,
}

/// O centro da vila para o capítulo: a prefeitura que governa o jogador, a do capítulo 2, a casa
/// do capítulo 2 ou onde o jogador está.
fn anchor(p: &Progress, player: &NativeServerPlayer, now: u64) -> [i32; 3] {
    let at = player.position.map(|v| v.floor() as i32);
    society::governing(&player.dimension, at, now).map(|hall| hall.pos)
        .or(p.village_blocks[5]).or(p.village_blocks[4]).unwrap_or(at)
}

/// Lê a vila pelos registros da sociedade (casas, lojas, contas, prefeitura e agenda).
pub(super) fn survey(host: &mut dyn Host, dimension: &str, at: [i32; 3], now: u64) -> Town {
    let homes = society::homes_near(dimension, at, society::HALL_RADIUS, now);
    let shops = society::shops_near(dimension, at, society::HALL_RADIUS, now);
    let families: Vec<_> = homes.iter().filter(|home| home.residents > 0).collect();
    let count = |test: &dyn Fn(&society::Home) -> bool| families.iter().filter(|home| test(home)).count() as i32;
    let idh = if families.is_empty() { 0 } else { families.iter().map(|home| home.idh()).sum::<i32>() / families.len() as i32 };
    let weakest = (!families.is_empty()).then(|| {
        let average = |part: fn(&society::Home) -> i32| families.iter().map(|home| part(home)).sum::<i32>();
        let parts = [average(|h| h.food), average(|h| h.housing), average(|h| h.services), average(|h| h.wealth)];
        (0..4).min_by_key(|&index| parts[index]).unwrap_or(0)
    });
    let hall = society::governing(dimension, at, now);
    let day_time = host.day_time();
    let today = agenda::day_of(day_time);
    let next_visitor = (0..=12).find_map(|ahead| agenda::scheduled(today + ahead).map(|event| (event, (ahead - agenda::LEAD).max(0))));
    // A agenda fica no bloco da prefeitura: o pedido preparado e o aberto mais próximo.
    let (mut prepared, mut request) = (false, None);
    if let Some(hall) = &hall {
        host.change_mod_block_entity(hall.pos, &mut |entity| {
            if entity.id != society::TOWN_HALL { return; }
            let open = agenda::proposals(entity);
            prepared = open.iter().any(|proposal| proposal.prepared);
            request = open.iter().filter(|proposal| !proposal.prepared).min_by_key(|proposal| proposal.due)
                .map(|proposal| agenda::need_text(proposal, today).plain());
        });
    }
    let shop = shops.iter().find(|shop| shop.merchant.is_none() || shop.cash <= 0).or(shops.first()).map(|shop| shop.pos);
    let company_coins = society::ledger::total(Book::Company, dimension);
    let public_coins = society::ledger::total(Book::Public, dimension);
    Town {
        open_shop: shops.iter().any(|shop| shop.open),
        day: !workforce::night(host),
        wallets: families.iter().any(|home| home.wallet > 0),
        company_coins,
        public_coins,
        staffed: hall.as_ref().is_some_and(|hall| hall.staffed),
        next_visitor,
        homes: families.len() as i32,
        sanitized: count(&|home| home.sanitation),
        powered: count(&|home| home.power),
        entertained: count(&|home| home.leisure),
        merchant: shops.iter().any(|shop| shop.merchant.is_some()),
        sales: shops.iter().any(|shop| shop.cash > 0),
        company: company_coins > 0,
        public: public_coins > 0,
        families: families.len() as i32,
        residents: families.iter().map(|home| home.residents).sum(),
        idh,
        weakest,
        phase: hall.as_ref().map_or(0, |hall| hall.phase),
        prepared,
        request,
        shop,
        hall: hall.map(|hall| hall.pos),
    }
}

/// A etapa está cumprida nesta leitura da vila?
pub(super) fn done(step: usize, town: &Town) -> bool {
    match step {
        0 => town.sanitized > 0,
        1 => town.merchant,
        2 => town.sales,
        3 => town.company,
        4 => town.powered > 0,
        5 => town.entertained > 0,
        6 => town.families >= FAMILIES,
        7 => town.public,
        8 => town.prepared,
        _ => town.phase >= 2,
    }
}

/// O objetivo da etapa com os números da vila, e para onde apontar.
pub(super) fn goal(step: usize, town: &Town) -> (Text, Option<[i32; 3]>) {
    let idh = format!("{},{:02}", town.idh / 100, town.idh % 100);
    match step {
        0 => (Text::tr("veio.city.goal.0", &[&town.sanitized, &town.homes]), None),
        1 | 2 => (Text::tr(&format!("veio.city.goal.{step}"), &[]), town.shop),
        4 => (Text::tr("veio.city.goal.4", &[&town.powered, &town.homes]), None),
        5 => (Text::tr("veio.city.goal.5", &[&town.entertained, &town.homes]), None),
        6 => (Text::tr("veio.city.goal.6", &[&town.families]), None),
        3 => (Text::tr("veio.city.goal.3", &[&town.company_coins]), None),
        7 => (Text::tr("veio.city.goal.7", &[&town.public_coins]), town.hall),
        8 => match (&town.request, town.next_visitor) {
            (Some(request), _) => (Text::tr("veio.city.goal.8", &[request]), town.hall),
            (None, Some((event, days))) => {
                let name = Text::tr(&format!("veio.agenda.{}.name", agenda::EVENTS[event].key), &[]).plain();
                (Text::tr("veio.city.goal.8_wait", &[&days, &name]), town.hall)
            }
            (None, None) => (Text::tr("veio.city.goal.8_soon", &[]), town.hall),
        },
        9 => {
            let weak = town.weakest.map_or(String::new(), |index| Text::tr(&format!("veio.city.weak.{index}"), &[]).plain());
            (Text::tr("veio.city.goal.9", &[&town.residents, &town.families, &idh, &weak]), town.hall)
        }
        _ => (Text::tr(&format!("veio.city.goal.{step}"), &[]), None),
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
    host.tell(uuid, Text::tr(&format!("veio.city.more.{step}"), &[]));
}

/// As condições que a etapa pede ao mesmo tempo, para a lista ✔/✘ da barra: (cumprida, chave,
/// argumentos). Só nas etapas em que várias coisas precisam estar certas juntas.
pub(super) fn checks(step: usize, town: &Town) -> Vec<(bool, &'static str, Vec<String>)> {
    match step {
        2 => vec![
            (town.open_shop, "veio.city.check.open", vec![]),
            (town.day, "veio.city.check.day", vec![]),
            (town.company, "veio.city.check.wages", vec![]),
            (town.wallets, "veio.city.check.wallets", vec![]),
        ],
        9 => vec![
            (town.staffed, "veio.city.check.clerk", vec![]),
            (town.residents >= VILLAGE_RESIDENTS, "veio.city.check.residents", vec![town.residents.to_string()]),
            (town.families >= FAMILIES, "veio.city.check.families", vec![town.families.to_string()]),
            (town.idh >= VILLAGE_IDH, "veio.city.check.idh", vec![format!("{},{:02}", town.idh / 100, town.idh % 100)]),
        ],
        _ => Vec::new(),
    }
}

/// O número que a etapa acompanha (casas atendidas, famílias, moradores): quando sobe, um "plim".
pub(super) fn metric(step: usize, town: &Town) -> Option<i32> {
    match step {
        0 => Some(town.sanitized),
        4 => Some(town.powered),
        5 => Some(town.entertained),
        6 => Some(town.families),
        9 => Some(town.residents),
        _ => None,
    }
}

fn json(text: &str) -> String { text.replace('\\', "\\\\").replace('"', "\\\"") }

/// A barra: lista de materiais enquanto o bloco da etapa não foi fabricado; depois o objetivo
/// (ou a lista ✔/✘ das condições), a seta até a loja ou a prefeitura quando longe, e o aviso
/// discreto de que o prefeito tem uma dica.
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
    let (goal, target) = goal(step, town);
    let mut tail = String::new();
    if let Some(pos) = target {
        let (distance, way) = intro::pointer(player, [f64::from(pos[0]) + 0.5, f64::from(pos[1]), f64::from(pos[2]) + 0.5]);
        if distance > FAR { tail = format!(" · {distance} {way}"); }
    }
    let checks = checks(step, town);
    if checks.is_empty() || !spawn_safe(&player.name) {
        let mut line = goal.plain() + &tail;
        if p.city_hinted { line += &Text::tr("veio.city.hint_ready", &[]).plain(); }
        host.action_bar(player.uuid, Text::tr("veio.city.bar", &[&(step + 1), &line]));
        return;
    }
    let mut parts = vec![format!(r#"{{"translate":"veio.city.bar_prefix","with":["{}"],"color":"gold"}}"#, step + 1)];
    for (ok, key, args) in checks {
        let (mark, color) = if ok { ("✔", "green") } else { ("✘", "red") };
        let with = args.iter().map(|arg| format!(r#""{}""#, json(arg))).collect::<Vec<_>>().join(",");
        parts.push(format!(r#"{{"text":"  {mark} ","color":"{color}"}},{{"translate":"{key}","with":[{with}],"color":"{color}"}}"#));
    }
    if step == 9 && let Some(index) = town.weakest {
        parts.push(format!(r#"{{"translate":"veio.city.weak.{index}","color":"yellow"}}"#));
    }
    if !tail.is_empty() { parts.push(format!(r#"{{"text":"{}","color":"gray"}}"#, json(&tail))); }
    if p.city_hinted { parts.push(r#"{"translate":"veio.city.hint_ready","color":"aqua"}"#.into()); }
    host.run_command(&format!(r#"title {} actionbar ["",{}]"#, player.name, parts.join(",")));
}

/// Um número da etapa subiu (mais uma casa atendida, mais uma família): um "plim" discreto, sem chat.
fn progress_sound(host: &mut dyn Host, player: &NativeServerPlayer, p: &mut Progress, step: usize, town: &Town) {
    let Some(value) = metric(step, town) else { return };
    if p.city_metric.is_some_and(|before| value > before) && spawn_safe(&player.name) {
        host.run_command(&format!("execute as {0} at @s run playsound minecraft:block.note_block.bell master @s ~ ~ ~ 0.6 1.6", player.name));
    }
    p.city_metric = Some(value);
}

/// Parado numa etapa por muito tempo: a barra avisa que o prefeito tem uma dica. Ele só fala
/// quando o jogador vai até ele.
fn hint(p: &mut Progress, now: u64) {
    let since = *p.city_since.get_or_insert(now);
    if now >= since + STUCK { p.city_hinted = true; }
}

/// Etapa concluída: o prefeito comenta, título, som e experiência.
fn celebrate(host: &mut dyn Host, player: &NativeServerPlayer, finished: usize) {
    village::say(host, player.uuid, &format!("veio.city.done.{finished}"));
    if !spawn_safe(&player.name) { return; }
    let name = &player.name;
    let next = if finished + 1 < usize::from(STEPS) { format!("veio.city.title.{}", finished + 1) } else { "veio.city.end_sub".into() };
    host.run_command(&format!(r#"title {name} subtitle {{"translate":"{next}"}}"#));
    host.run_command(&format!(r#"title {name} title {{"translate":"veio.onboarding.done","color":"gold"}}"#));
    host.run_command(&format!("execute as {name} at @s run playsound minecraft:ui.toast.challenge_complete master @s"));
    host.run_command(&format!("xp add {name} {} points", 30 + 5 * finished));
}

/// Um passo do capítulo, a cada segundo, para um jogador online. `now` é o relógio dos
/// registros da sociedade (`defense::clock`).
pub(super) fn tick(host: &mut dyn Host, player: &NativeServerPlayer, p: &mut Progress, now: u64) {
    let Some(step) = step(p) else { return };
    let at = anchor(p, player, now);
    let town = survey(host, &player.dimension, at, now);
    if done(step, &town) {
        celebrate(host, player, step);
        p.city += 1;
        p.city_since = Some(now);
        p.city_hinted = false;
        p.city_metric = None;
        p.dirty = true;
        if p.city == DONE_CITY {
            finish(host, player);
            host.action_bar(player.uuid, Text::literal(""));
            return;
        }
    }
    if p.hidden { return; }
    let Some(step) = self::step(p) else { return };
    if p.city_announced != Some(p.city) {
        if p.city_since.is_some_and(|since| now < since + BREATH) { return; }
        announce(host, player, step);
        p.city_announced = Some(p.city);
    }
    hint(p, now);
    progress_sound(host, player, p, step, &town);
    bar(host, player, p, &town);
}

/// Fim do capítulo: o prefeito fecha e aponta o caminho até a Metrópole.
fn finish(host: &mut dyn Host, player: &NativeServerPlayer) {
    for line in 1..=5 { village::say(host, player.uuid, &format!("veio.city.end.{line}")); }
    if spawn_safe(&player.name) {
        let name = &player.name;
        host.run_command(&format!(r#"title {name} subtitle {{"translate":"veio.city.end_sub"}}"#));
        host.run_command(&format!(r#"title {name} title {{"translate":"veio.city.end_title","color":"gold"}}"#));
        host.run_command(&format!("xp add {name} 150 points"));
    }
}

/// Clique no prefeito: começa o capítulo ou repete a etapa (com a dica, se já passou do tempo).
pub(super) fn talk(host: &mut dyn Host, player: &NativeServerPlayer, p: &mut Progress) {
    match step(p) {
        None => start(host, player, p),
        Some(step) => {
            village::say(host, player.uuid, "veio.village.again");
            announce(host, player, step);
            // A dica vem quando o jogador pede: ao clicar, sempre, e o aviso da barra some.
            village::say(host, player.uuid, &format!("veio.city.hint.{step}"));
            if step == 9 {
                let now = defense::clock();
                let town = survey(host, &player.dimension, anchor(p, player, now), now);
                if let Some(index) = town.weakest { village::say(host, player.uuid, &format!("veio.city.fix.{index}")); }
            }
            p.city_hinted = false;
            p.city_since = None;
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
    p.city_since = None;
    p.city_hinted = false;
    p.city_metric = None;
    p.hidden = false;
    p.dirty = true;
}
