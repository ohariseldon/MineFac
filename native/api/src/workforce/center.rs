//! Centro de controle: o painel executivo da vila operária.
//!
//! Um aldeão contratado (o administrador) fica no centro. Com ele contratado, o painel junta o
//! que os postos, zonas, armazéns e aldeões a até `RADIUS` blocos publicam nos registros em
//! memória e mostra três páginas (a chave troca a página):
//!
//! 1. **Resumo:** aldeões e teto da vila, trabalhadores e esperas, comida guardada (em tarefas),
//!    zonas e quantas já podem crescer, armazéns, fila e até três **metas** de estoque.
//! 2. **Atenção:** as pendências da vila em ordem de urgência, com distância e direção. Passe o
//!    cursor numa linha para saber o que fazer. Com uma **bússola** no centro, o jogador recebe o
//!    **foco** (`focus.rs`): a barra de ação o leva até a pendência e explica o que fazer lá.
//! 3. **Obras:** melhorias da vila (`projects.rs`). Os materiais entram pelo slot **Obra**.
//! 4. **Pedidos:** prestígio, caixa, decreto e os pedidos do dia (`requests.rs`).
//!
//! Toda manhã, quem está na vila recebe um **boletim** no chat: tarefas de ontem, população,
//! pedidos atendidos, os pedidos de hoje e as pendências.
//!
//! Metas: um item num slot de meta pede `quantidade x META_UNIT` daquele item guardados nos
//! armazéns da área. Ex.: 4 pães = meta de 64 pães.
//!
//! Clicar no centro com um livro manda ao chat um relatório com os postos que precisam de atenção
//! (posição e motivo), as zonas e o que falta para o próximo nível, os armazéns e as metas.
use super::*;
use std::sync::atomic::{AtomicBool, Ordering};

pub(crate) const CONTROL_CENTER: &str = "veio:control_center";
pub(crate) const RADIUS: i32 = 64;
pub(crate) const META_UNIT: u32 = 16;
pub(crate) const GOAL_SLOTS: std::ops::Range<usize> = 1..4;
pub(crate) const PAGES: i32 = 4;
/// O administrador gasta comida a cada `CHARGE_EVERY` atualizações do painel (cerca de 2 min).
const CHARGE_EVERY: i32 = 12;
const REFRESH_EVERY: i32 = 10;
const STALE: u64 = 200;
const REPORT_LINES: usize = 8;
/// Pendências guardadas na página Atenção.
pub(crate) const MAX_ISSUES: usize = 10;
/// Pendências que não vêm de um posto.
pub(crate) const ZONE_READY: i32 = 40;
pub(crate) const STORE_FULL: i32 = 41;

pub(crate) fn accepts(slot: usize, stack: &ItemStack) -> bool {
    plain(stack) && match slot {
        0 => is_food(&stack.item),
        projects::SITE => projects::material(&stack.item) || requests::wanted(&stack.item),
        projects::DECREE => projects::decree_of(&stack.item).is_some(),
        society::ledger::COMPANY_VAULT => stack.item.as_ref() == WAGE,
        society::ledger::COMPANY_CASHOUT => false,
        _ => GOAL_SLOTS.contains(&slot),
    }
}

/// Um posto como o centro o vê.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct PostInfo {
    pub dimension: String,
    pub pos: [i32; 3],
    pub block: String,
    pub status: i32,
    pub staffed: bool,
    /// Tarefas na fila (mesa do capataz).
    pub queued: i32,
    /// Tarefas feitas desde sempre (para o boletim da manhã).
    pub total: i32,
    pub seen: u64,
}

static POSTS: Mutex<Vec<PostInfo>> = Mutex::new(Vec::new());
static POSTS_PENDING: AtomicBool = AtomicBool::new(false);

/// Chamado pelos postos a cada ciclo.
pub(crate) fn report(dimension: &str, pos: [i32; 3], entity: &ModBlockEntity, now: u64) {
    let info = PostInfo {
        dimension: dimension.to_owned(), pos, block: entity.id.clone(), status: number(entity, "Status"),
        staffed: worker(entity).is_some(), queued: number(entity, "Queued"), total: number(entity, "Total"), seen: now,
    };
    if let Ok(mut posts) = POSTS.lock() {
        if let Some(known) = posts.iter_mut().find(|known| known.dimension == dimension && known.pos == pos) {
            *known = info;
        } else {
            posts.retain(|known| now.saturating_sub(known.seen) <= STALE);
            posts.push(info);
        }
        POSTS_PENDING.store(true, Ordering::Release);
    }
}

pub(crate) fn posts_near(dimension: &str, at: [i32; 3], radius: i32, now: u64) -> Vec<PostInfo> {
    if !POSTS_PENDING.load(Ordering::Acquire) {
        return Vec::new();
    }
    // Leitura sem efeito colateral: entradas vencidas saem nas escritas (`register`/`report`).
    POSTS.lock().map(|posts| {
        if posts.is_empty() {
            POSTS_PENDING.store(false, Ordering::Release);
            return Vec::new();
        }
        posts.iter()
            .filter(|post| post.dimension == dimension && now.saturating_sub(post.seen) <= STALE
                && (0..3).all(|axis| (post.pos[axis] - at[axis]).abs() <= radius))
            .cloned().collect()
    }).unwrap_or_default()
}

/// O posto em `pos`, se ainda estiver publicando.
pub(crate) fn post_at(dimension: &str, pos: [i32; 3], now: u64) -> Option<PostInfo> {
    posts_near(dimension, pos, 0, now).into_iter().next()
}

/// Esperando por algo que o jogador ou o capataz precisa resolver.
pub(crate) fn waiting(status: i32) -> bool {
    matches!(status, 3 | NO_WORKER | ABSENT | NO_FOOD | PATH | NO_WAGE | site::SITE_BLOCKED)
}

/// Ordem de urgência: sem comida e ausente param o trabalho que já existia.
fn urgency(code: i32) -> usize {
    [NO_FOOD, ABSENT, NO_WORKER, NO_WAGE, PATH, site::SITE_BLOCKED, 3, ZONE_READY, STORE_FULL].iter()
        .position(|known| *known == code).unwrap_or(9)
}

/// Tipo do bloco de uma pendência, para o nome curto no painel (4 bits).
pub(crate) fn kind_of(block: &str) -> i32 {
    match block {
        FARM_POST => 1,
        HAULER_POST => 2,
        FOREMAN_DESK => 3,
        SHEPHERD_POST => 4,
        CONTROL_CENTER => 5,
        FIELD_MARKER => 6,
        PEN_MARKER => 7,
        storage::BARN => 8,
        storage::SILO => 9,
        SITE => 10,
        _ => 0,
    }
}

/// Algo na vila que precisa do jogador.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Issue {
    /// Estado do posto (`waiting`) ou `ZONE_READY` / `STORE_FULL`.
    pub code: i32,
    pub kind: i32,
    pub pos: [i32; 3],
}

/// Pendências em volta do centro em `at`, da mais urgente para a menos urgente e, entre iguais,
/// da mais perto para a mais longe.
pub(crate) fn issues(dimension: &str, at: [i32; 3], now: u64) -> Vec<Issue> {
    let mut list: Vec<Issue> = posts_near(dimension, at, RADIUS, now).into_iter()
        .filter(|post| post.pos != at && waiting(post.status))
        .map(|post| Issue { code: post.status, kind: kind_of(&post.block), pos: post.pos })
        .collect();
    for zone in zones::zones_near(dimension, at, RADIUS, now) {
        if zones::requirement(zone.unlocked).is_some_and(|(_, work, villagers)| zone.work >= work && zone.villagers >= villagers) {
            list.push(Issue { code: ZONE_READY, kind: if zone.kind == zones::Kind::Pen { 7 } else { 6 }, pos: zone.pos });
        }
    }
    for store in storage::stores_near(dimension, at, RADIUS, now) {
        if store.total > 0 && store.used * 10 >= store.total * 9 {
            list.push(Issue { code: STORE_FULL, kind: if store.total == 18 { 9 } else { 8 }, pos: store.pos });
        }
    }
    let distance = |pos: [i32; 3]| (0..3).map(|axis| i64::from(pos[axis] - at[axis]).pow(2)).sum::<i64>();
    list.sort_by_key(|issue| (urgency(issue.code), distance(issue.pos)));
    list
}

/// Pendência num inteiro, relativa ao centro: código (6 bits), tipo (4), dx e dz (8 cada) e dy (5).
pub(crate) fn pack(issue: &Issue, origin: [i32; 3]) -> i32 {
    let dx = (issue.pos[0] - origin[0]).clamp(-128, 127) + 128;
    let dz = (issue.pos[2] - origin[2]).clamp(-128, 127) + 128;
    let dy = (issue.pos[1] - origin[1]).clamp(-16, 15) + 16;
    (issue.code & 63) | ((issue.kind & 15) << 6) | (dx << 10) | (dz << 18) | (dy << 26)
}

/// (código, tipo, dx, dy, dz)
pub(crate) fn unpack(value: i32) -> (i32, i32, i32, i32, i32) {
    (value & 63, (value >> 6) & 15, ((value >> 10) & 255) - 128, ((value >> 26) & 31) - 16, ((value >> 18) & 255) - 128)
}

/// Direção na rosa dos ventos (norte = -z): índice 0-7 a partir do norte, no sentido horário.
pub(crate) fn direction(dx: f64, dz: f64) -> usize {
    let angle = dx.atan2(-dz).to_degrees().rem_euclid(360.0);
    ((angle + 22.5) / 45.0) as usize % 8
}

pub(crate) fn direction_text(dx: f64, dz: f64) -> String {
    Text::tr(&format!("veio.dir.{}", direction(dx, dz)), &[]).plain()
}

/// Nome curto do problema de uma pendência.
pub(crate) fn problem_text(code: i32) -> String {
    match code {
        ZONE_READY => Text::tr("veio.center.problem.zone_ready", &[]).plain(),
        STORE_FULL => Text::tr("veio.center.problem.store_full", &[]).plain(),
        code => Text::tr(&format!("veio.status.{}", presentation::STATUS_KEYS.get(code as usize).copied().unwrap_or("idle")), &[]).plain(),
    }
}

/// O que fazer ao chegar numa pendência.
pub(crate) fn advice(code: i32) -> String {
    match code {
        ZONE_READY => Text::tr("veio.focus.advice.zone_ready", &[]).plain(),
        STORE_FULL => Text::tr("veio.focus.advice.store_full", &[]).plain(),
        code => Text::tr(&operations::advice_key(code), &[]).plain(),
    }
}

/// O administrador fica no centro.
pub(crate) fn plan(_host: &mut dyn Host, pos: [i32; 3], _entity: &mut ModBlockEntity, _now: u64) -> Result<Plan, i32> {
    Ok(Plan::Visit { spot: pos, reach: POST_REACH })
}

/// Com o administrador ao lado: marca a presença e, de tempos em tempos, fixa o centro da vila.
/// A cada `CHARGE_EVERY` atualizações devolve uma tarefa concluída (gasta comida); nas outras, nada.
pub(crate) fn perform(host: &mut dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity, _now: u64) -> Option<Vec<(String, u32)>> {
    set_number(entity, "Admin", 1);
    let round = number(entity, "Refresh") + 1;
    set_number(entity, "Refresh", round % (REFRESH_EVERY * CHARGE_EVERY));
    if round % REFRESH_EVERY == 1 {
        // Com administrador, o centro de controle é o centro da vila: spawn e renascimento aqui.
        spawn::control_center(host, pos);
    }
    (round % (REFRESH_EVERY * CHARGE_EVERY) == 0).then(Vec::new)
}

/// Tick do centro, na fase do posto (a cada 10 ticks): com administrador contratado, recebe as
/// entregas do slot de obra (pedidos, depois obras, sobra de esmeraldas no caixa), faz a rotina da
/// manhã, atualiza o painel (também à noite) e publica os efeitos.
pub(crate) fn tick(host: &mut dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity, now: u64) {
    let dimension = host.dimension();
    if worker(entity).is_none() {
        projects::withdraw_village(dimension, pos);
        return;
    }
    let day = day_counter(host, entity);
    let perks = projects::active(entity, day);
    if !entity.nbt.contains_key(nbt_key("Morning")) {
        // Primeira vez: a vila já começa com pedidos; o boletim espera a próxima manhã.
        requests::morning(entity, pos, day, perks);
        set_number(entity, "Morning", day + 1);
        mark_morning(host, pos, entity, now);
    } else if number(entity, "Morning") != day + 1 {
        set_number(entity, "Morning", day + 1);
        morning(host, pos, entity, day, now);
    }
    let (earned, finished) = requests::deliver(entity, perks);
    if finished > 0 {
        // Recompensa paga pela prefeitura (economia fechada); sem caixa público, só o prestígio.
        let paid = reward(dimension, pos, entity, earned as i32 * society::COINS_PER_EMERALD) / society::COINS_PER_EMERALD;
        set_number(entity, "ReqToday", number(entity, "ReqToday") + finished);
        let up = projects::gain(entity, requests::PRESTIGE * finished);
        celebrate(host, pos, Text::tr("veio.request.done", &[&finished, &paid]), up, entity);
    }
    let mut built = projects::deliver(entity, day);
    if let Some(stack) = entity.items.get(projects::SITE).and_then(Option::as_ref).filter(|stack| plain(stack) && is_plain(stack, WAGE)) {
        // Esmeraldas que nenhuma obra pediu: caixa da vila.
        let count = stack.count;
        consume(&mut entity.items[projects::SITE], count);
        add_coins(dimension, pos, entity, count as i32 * society::COINS_PER_EMERALD);
    }
    let (paid, used) = projects::pay_with(entity, day, (coins(entity) / society::COINS_PER_EMERALD) as u32);
    add_coins(dimension, pos, entity, -(used as i32) * society::COINS_PER_EMERALD);
    built.extend(paid);
    for project in built {
        let key = projects::PROJECTS[project].key;
        let line = Text::tr("veio.project.done", &[&Text::tr(&format!("veio.project.{key}.name"), &[]).plain(),
            &Text::tr(&format!("veio.project.{key}.effect"), &[]).plain()]);
        celebrate(host, pos, line, false, entity);
    }
    let round = number(entity, "Tick") + 1;
    set_number(entity, "Tick", round % 4);
    if round % 4 == 1 || !entity.nbt.contains_key(nbt_key("DA1")) {
        // Prédios de pé e funcionando: o efeito de uma obra paga só vale assim.
        let found = site::near(dimension, pos, RADIUS, now, None);
        set_number(entity, "Standing", found.operating as i32);
        set_number(entity, "Erected", found.erected as i32);
        set_number(entity, "Research", found.research as i32);
        refresh(host, pos, entity, now);
    }
    let (villagers, cap) = split(signed(entity, "DA7"));
    projects::publish(projects::Village {
        dimension: dimension.to_owned(), pos, perks: projects::active(entity, day),
        waiting: signed(entity, "DB1"), villagers, cap, funded: projects::built(entity), requests: requests::open(entity), seen: now,
    });
}

/// Guarda os números de hoje para o boletim de amanhã.
fn mark_morning(host: &dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity, now: u64) {
    let tasks: i32 = posts_near(host.dimension(), pos, RADIUS, now).iter().map(|post| post.total).sum();
    set_number(entity, "TasksAt", tasks);
    set_number(entity, "PopAt", split(signed(entity, "DA7")).0);
    set_number(entity, "ReqToday", 0);
}

/// Rotina da manhã: juros, viajante da estalagem, prestígio de uma vila em ordem, pedidos novos e
/// o boletim para quem está na vila.
fn morning(host: &mut dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity, day: i32, now: u64) {
    let dimension = host.dimension();
    let perks = projects::active(entity, day);
    let tasks: i32 = posts_near(dimension, pos, RADIUS, now).iter().map(|post| post.total).sum();
    let yesterday = (tasks - number(entity, "TasksAt")).max(0);
    let (villagers, cap) = split(signed(entity, "DA7"));
    let growth = villagers - number(entity, "PopAt");
    let served = number(entity, "ReqToday");
    let interest = projects::interest(perks, (coins(entity) / society::COINS_PER_EMERALD) as u32);
    let interest = reward(dimension, pos, entity, interest as i32 * society::COINS_PER_EMERALD) / society::COINS_PER_EMERALD;
    let calm = issues(dimension, pos, now).is_empty();
    let mut up = false;
    if calm {
        up |= projects::gain(entity, 3);
    }
    let mut traveler = false;
    if perks & projects::INN != 0 && villagers < cap {
        let at = center(pos);
        traveler = host.summon(dimension, VILLAGER, [at[0] + 1.5, at[1] + 0.5, at[2] + 1.5], Compound::new()).is_some();
    }
    requests::morning(entity, pos, day, projects::active(entity, day));
    mark_morning(host, pos, entity, now);
    let tr = |key: &str, args: &[&dyn std::fmt::Display]| Text::tr(key, args);
    let mut lines = vec![
        tr("veio.bulletin.title", &[&(day + 1), &tier_name(projects::tier(entity))]),
        tr("veio.bulletin.yesterday", &[&yesterday, &villagers, &cap, &format!("{growth:+}"), &served]),
    ];
    if interest > 0 || traveler {
        lines.push(tr("veio.bulletin.extra", &[&interest, &if traveler { 1 } else { 0 }]));
    }
    let wishes: Vec<String> = (0..requests::SLOTS).filter_map(|index| requests::get(entity, index))
        .filter(|request| !request.done())
        .map(|request| format!("{} {}", request.need - request.have, Text::tr(requests::POOL[request.wish].name, &[]).plain()))
        .collect();
    if !wishes.is_empty() {
        lines.push(tr("veio.bulletin.requests", &[&wishes.join(", ")]));
    }
    let open = signed(entity, "DB1");
    lines.push(if calm { tr("veio.bulletin.calm", &[]) } else { tr("veio.bulletin.issues", &[&open]) });
    let at = center(pos);
    let players: Vec<u128> = host.server_players().into_iter()
        .filter(|player| player.dimension == dimension && distance2(player.position, at) < f64::from(RADIUS * RADIUS))
        .map(|player| player.uuid).collect();
    for player in players {
        for line in &lines {
            host.tell(player, line.clone());
        }
        if up {
            host.tell(player, tr("veio.prestige.up", &[&tier_name(projects::tier(entity))]));
        }
    }
}

pub(crate) fn tier_name(tier: i32) -> String {
    Text::tr(&format!("veio.prestige.tier.{}", tier.clamp(0, projects::MAX_TIER)), &[]).plain()
}

/// Algo bom aconteceu: som, partículas e aviso a quem está perto (e, se subiu, o novo prestígio).
fn celebrate(host: &mut dyn Host, pos: [i32; 3], line: Text, tier_up: bool, entity: &ModBlockEntity) {
    let at = center(pos);
    host.play_sound("minecraft:ui.toast.challenge_complete", at, 4, 0.8, 1.0);
    host.send_particles(&ParticleOptions::simple(ParticleType::HappyVillager), None,
        [at[0], at[1] + 1.0, at[2]], [1.5, 0.8, 1.5], 0.0, 30);
    let dimension = host.dimension();
    let players: Vec<u128> = host.server_players().into_iter()
        .filter(|player| player.dimension == dimension && distance2(player.position, at) < 48.0 * 48.0)
        .map(|player| player.uuid).collect();
    for player in players {
        host.tell(player, line.clone());
        if tier_up {
            host.tell(player, Text::tr("veio.prestige.up", &[&tier_name(projects::tier(entity))]));
        }
    }
}

/// Prestígio vindo de fora do centro (lar: banquete, dia em ordem).
pub(crate) fn add_prestige(host: &mut dyn Host, pos: [i32; 3], points: i32) {
    let mut up = None;
    host.change_mod_block_entity(pos, &mut |entity| {
        if entity.id == CONTROL_CENTER {
            up = Some((projects::gain(entity, points), projects::tier(entity)));
        }
    });
    if let Some((true, tier)) = up {
        let at = center(pos);
        let dimension = host.dimension();
        let players: Vec<u128> = host.server_players().into_iter()
            .filter(|player| player.dimension == dimension && distance2(player.position, at) < f64::from(RADIUS * RADIUS))
            .map(|player| player.uuid).collect();
        for player in players {
            host.tell(player, Text::tr("veio.prestige.up", &[&tier_name(tier)]));
        }
    }
}

/// Esmeraldas do caixa do centro em `pos`: tira até `count`. Devolve quanto saiu.
pub(crate) fn take_treasury(host: &mut dyn Host, pos: [i32; 3], count: u32) -> u32 {
    let dimension = host.dimension();
    let mut taken = 0;
    host.change_mod_block_entity(pos, &mut |entity| {
        if entity.id == CONTROL_CENTER {
            let have = society::ledger::balance(society::ledger::Book::Company, dimension, pos).unwrap_or_else(|| coins(entity));
            let emeralds = ((have / society::COINS_PER_EMERALD).max(0) as u32).min(count);
            add_coins(dimension, pos, entity, -(emeralds as i32) * society::COINS_PER_EMERALD);
            taken = emeralds;
        }
    });
    taken
}

pub(crate) fn return_treasury(host: &mut dyn Host, pos: [i32; 3], count: u32) {
    let dimension = host.dimension();
    host.change_mod_block_entity(pos, &mut |entity| {
        if entity.id == CONTROL_CENTER {
            add_coins(dimension, pos, entity, count as i32 * society::COINS_PER_EMERALD);
        }
    });
}

/// Agachar + esmeralda no centro: saca até 16 esmeraldas do caixa.
pub(crate) fn cash_out(host: &mut dyn Host, pos: [i32; 3]) -> Option<Flow> {
    let player = nearest_player(host, pos, 12.0);
    let taken = take_treasury(host, pos, 16);
    if taken > 0 {
        host.give_items(WAGE, taken);
        host.play_sound("minecraft:entity.experience_orb.pickup", center(pos), 4, 0.6, 1.2);
    }
    if let Some(player) = player {
        host.action_bar(player, Text::tr(if taken > 0 { "veio.treasury.out" } else { "veio.treasury.empty" }, &[&taken]));
    }
    Some(Flow::Cancel)
}

fn goals(entity: &ModBlockEntity) -> Vec<Option<(String, u32)>> {
    GOAL_SLOTS.map(|slot| entity.items.get(slot).and_then(Option::as_ref)
        .map(|stack| (stack.item.to_string(), stack.count * META_UNIT))).collect()
}

const PAGE_KEYS: [[&str; 11]; 4] = [
    ["DA1", "DA2", "DA3", "DA4", "DA5", "DA6", "DA7", "DA8", "DA9", "DA10", "DA11"],
    ["DB1", "DB2", "DB3", "DB4", "DB5", "DB6", "DB7", "DB8", "DB9", "DB10", "DB11"],
    ["DC1", "DC2", "DC3", "DC4", "DC5", "DC6", "DC7", "DC8", "DC9", "DC10", "DC11"],
    ["DD1", "DD2", "DD3", "DD4", "DD5", "DD6", "DD7", "DD8", "DD9", "DD10", "DD11"],
];

fn pair(high: usize, low: usize) -> i32 {
    ((high.min(32767) as i32) << 16) | low.min(65535) as i32
}

pub(crate) fn split(value: i32) -> (i32, i32) {
    (value >> 16, value & 0xffff)
}

/// Recalcula as quatro páginas do painel a partir dos registros e do censo de aldeões.
pub(crate) fn refresh(host: &dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity, now: u64) {
    let dimension = host.dimension();
    let posts: Vec<PostInfo> = posts_near(dimension, pos, RADIUS, now).into_iter().filter(|post| post.pos != pos).collect();
    let zones = zones::zones_near(dimension, pos, RADIUS, now);
    let stores = storage::stores_near(dimension, pos, RADIUS, now);
    let workers = posts.iter().filter(|post| post.staffed).count();
    let working = posts.iter().filter(|post| post.staffed && matches!(post.status, 1 | WALKING)).count();
    let waiting = posts.iter().filter(|post| waiting(post.status)).count();
    let hungry = posts.iter().filter(|post| post.status == NO_FOOD).count();
    let (used, total) = stores.iter().fold((0, 0), |(u, t), store| (u + store.used, t + store.total));
    let stored = |item: &str| stores.iter().flat_map(|store| &store.items).filter(|(known, _)| known == item).map(|(_, count)| count).sum::<u32>();
    let food: u32 = stores.iter().flat_map(|store| &store.items)
        .filter_map(|(item, count)| production::pantry::value(item).map(|value| value * count)).sum();
    let progress: Vec<i32> = goals(entity).into_iter().map(|goal| goal.map_or(-1, |(item, target)| {
        (stored(&item).min(target) * 100 / target.max(1)) as i32
    })).collect();
    let middle = center(pos);
    let villagers = production::ranch::villager_positions(host, dimension).iter()
        .filter(|(_, at)| (0..3).all(|axis| (at[axis] - middle[axis]).abs() <= village::COUNT_RADIUS))
        .count();
    let cap = village::cap(dimension, pos, now);
    let list = issues(dimension, pos, now);
    let ready = list.iter().filter(|issue| issue.code == ZONE_READY).count();
    let overview = [
        pair(workers, working), pair(waiting, hungry),
        pair(zones.iter().filter(|zone| zone.kind == zones::Kind::Field).count(), zones.iter().filter(|zone| zone.kind == zones::Kind::Pen).count()),
        ready as i32,
        pair(posts.iter().map(|post| post.queued as usize).sum(), if total == 0 { 0 } else { (used * 100 / total) as usize + 1 }),
        super::super::holding::exports_day(dimension), pair(villagers, cap), (food / TASK_COST as u32).min(99_999) as i32,
        progress[0], progress[1], progress[2],
    ];
    let mut attention = [0; 11];
    attention[0] = list.len() as i32;
    for (index, issue) in list.iter().take(MAX_ISSUES).enumerate() {
        attention[index + 1] = pack(issue, pos);
    }
    let day = number(entity, "Day");
    let mut works = [0; 11];
    for (index, slot) in works.iter_mut().take(projects::PROJECTS.len()).enumerate() {
        *slot = projects::packed(entity, index, day);
    }
    let perks = projects::active(entity, day);
    works[projects::PROJECTS.len()] = projects::prestige(entity);
    let mut wishes = [0; 11];
    wishes[0] = projects::prestige(entity);
    wishes[1] = projects::decree_index(perks) | (requests::open_slots(perks) as i32) << 4 | (perks as i32 & projects::POST_OFFICE as i32);
    wishes[2] = coins(entity) / society::COINS_PER_EMERALD;
    for index in 0..requests::SLOTS {
        wishes[3 + index] = requests::get(entity, index).as_ref().map_or(0, requests::pack);
    }
    wishes[7] = number(entity, "ReqToday");
    for (page, values) in [overview, attention, works, wishes].iter().enumerate() {
        for (key, value) in PAGE_KEYS[page].iter().zip(values) {
            set_signed(entity, key, *value);
        }
    }
}

fn signed(entity: &ModBlockEntity, key: &str) -> i32 {
    match entity.nbt.get(nbt_key(key)) { Some(Tag::Int(value)) => *value, _ => 0 }
}

fn set_signed(entity: &mut ModBlockEntity, key: &str, value: i32) {
    if signed(entity, key) != value || !entity.nbt.contains_key(nbt_key(key)) {
        entity.nbt.insert(nbt_key(key).clone(), Tag::Int(value));
    }
}

/// Página da vila aberta (só vale na aba 0 da holding).
pub(crate) fn page(entity: &ModBlockEntity) -> i32 {
    number(entity, "Page") % PAGES
}

/// Visões do painel: as `PAGES` páginas da vila e depois as divisões da holding (abas 1 a 9).
pub(crate) const VIEWS: i32 = PAGES + super::super::holding::TABS - 1;

/// Índice da visão aberta (0 a `VIEWS - 1`).
pub(crate) fn view(entity: &ModBlockEntity) -> i32 {
    let tab = number(entity, "Tab").clamp(0, super::super::holding::TABS - 1);
    if tab == 0 { page(entity) } else { PAGES - 1 + tab }
}

/// Anda `step` visões (setas da tela ou chave).
pub(crate) fn step_view(entity: &mut ModBlockEntity, step: i32) {
    let next = (view(entity) + step).rem_euclid(VIEWS);
    if next < PAGES {
        set_number(entity, "Tab", 0);
        set_number(entity, "Page", next);
    } else {
        set_number(entity, "Tab", next - PAGES + 1);
        set_number(entity, "Page", 0);
    }
}

/// Chave sem agachar: próxima visão.
pub(crate) fn next_page(host: &mut dyn Host, pos: [i32; 3]) -> Option<Flow> {
    let dimension = host.dimension();
    let time = host.level(dimension).and_then(|level| level.game_time(host)).unwrap_or(0);
    let mut found = false;
    host.change_mod_block_entity(pos, &mut |entity| {
        if entity.id == CONTROL_CENTER {
            step_view(entity, 1);
            super::super::holding::refresh(dimension, pos, entity, time);
            found = true;
        }
    });
    if !found {
        return None;
    }
    host.block_entity_sync(pos);
    host.play_sound("minecraft:item.book.page_turn", center(pos), 4, 0.6, 1.0);
    Some(Flow::Cancel)
}

/// Os 13 valores do painel do centro. O valor 0 leva o estado (0-99), a página da vila (centenas)
/// e a aba da holding (milhares); o 12 é o caixa da companhia. Nas abas de divisão, 1 a 9 são os
/// valores da holding; na aba 0, os 11 valores da página da vila.
pub(crate) fn data(entity: &ModBlockEntity) -> [i32; 13] {
    let tab = number(entity, "Tab").clamp(0, super::super::holding::TABS - 1);
    let page = if tab == 0 { page(entity) } else { 0 };
    let status = number(entity, "Status").min(99) + page * 100 + tab * 1000;
    if tab > 0 {
        let holding = super::super::holding::data(entity);
        return std::array::from_fn(|index| match index {
            0 => status, 1..=11 => holding[index - 1], 12 => number(entity, "Company"), _ => 0,
        });
    }
    std::array::from_fn(|index| match index {
        0 => status, 12 => number(entity, "Company"), _ => signed(entity, PAGE_KEYS[page as usize][index - 1]),
    })
}

/// Caixa da companhia (moedas) visto do centro.
pub(crate) fn coins(entity: &ModBlockEntity) -> i32 {
    number(entity, "Company")
}

/// Esmeraldas de verdade entram (`delta > 0`) ou saem (`delta < 0`) do caixa da companhia: é uma
/// borda da economia fechada (`society::ledger::border`). Devolve quanto mudou.
pub(crate) fn add_coins(dimension: &str, pos: [i32; 3], entity: &mut ModBlockEntity, delta: i32) -> i32 {
    use society::ledger::{self, Book};
    let now = defense::clock();
    ledger::open(Book::Company, dimension, pos, coins(entity), now, false);
    let changed = if delta >= 0 {
        if ledger::put(Book::Company, dimension, pos, 0, delta, ledger::REDEEM, now) { delta } else { 0 }
    } else {
        -ledger::take(Book::Company, dimension, pos, 0, -delta, ledger::REDEEM, now)
    };
    ledger::border(dimension, changed);
    set_number(entity, "Company", ledger::balance(Book::Company, dimension, pos).unwrap_or_else(|| coins(entity)));
    changed
}

/// Recompensa da vila (pedidos, rendimento da Prefeitura): sai do caixa público da prefeitura que
/// governa o centro e entra na companhia. Sem prefeitura ou sem saldo, paga o que houver (ou nada).
/// Devolve as moedas pagas.
pub(crate) fn reward(dimension: &str, pos: [i32; 3], entity: &mut ModBlockEntity, coins_wanted: i32) -> i32 {
    use society::ledger::{self, Book};
    let now = defense::clock();
    let Some(hall) = society::governing(dimension, pos, now) else { return 0 };
    ledger::open(Book::Company, dimension, pos, coins(entity), now, false);
    let paid = ledger::take(Book::Public, dimension, hall.pos, 0, coins_wanted, ledger::ASSIST, now);
    if paid > 0 && !ledger::put(Book::Company, dimension, pos, 0, paid, ledger::RECEIVED, now) {
        ledger::put(Book::Public, dimension, hall.pos, 0, paid, ledger::RECEIVED, now);
        return 0;
    }
    set_number(entity, "Company", ledger::balance(Book::Company, dimension, pos).unwrap_or_else(|| coins(entity)));
    paid
}

const BAR: u32 = 0xff2b2b2b;
const ROW: f32 = 9.5;
const TOP: f32 = 33.0;
/// Linhas de texto por página (cabem antes dos slots).
const ROWS: usize = 6;
/// Linha dos slots de meta e de comida.
const SLOTS_Y: f32 = 89.0;
const WARN: u32 = 0x9a5a00;
const GOOD: u32 = 0x1e7a1e;

/// Corta o texto para caber em `width` pixels.
pub(crate) fn fit(host: &dyn Host, text: &str, width: f32) -> String {
    if host.text_width(text) <= width {
        return text.to_owned();
    }
    let mut cut: String = text.to_owned();
    while !cut.is_empty() && host.text_width(&format!("{cut}…")) > width {
        cut.pop();
    }
    format!("{}…", cut.trim_end())
}

fn issue_line(value: i32) -> String {
    let (code, kind, dx, _, dz) = unpack(value);
    let distance = f64::from(dx).hypot(f64::from(dz)).round() as i32;
    Text::tr("veio.center.issue", &[&Text::tr(&format!("veio.center.kind.{kind}"), &[]).plain(), &problem_text(code),
        &distance, &direction_text(f64::from(dx), f64::from(dz))]).plain()
}

/// Linha da página Obras.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ProjectRow {
    Project(usize),
    /// Obras prontas, numa linha só.
    Built(usize),
}

/// Ordem das linhas da página Obras: em andamento (ou festa ativa), liberadas, bloqueadas e, por
/// fim, uma linha com as prontas. Cabem 6 linhas.
pub(crate) fn project_rows(data: &[i32]) -> Vec<ProjectRow> {
    let read = |index: usize| data.get(index + 1).copied().unwrap_or(0);
    let state = |index: usize| projects::unpack(read(index));
    let count = projects::PROJECTS.len();
    let up = |index: usize| projects::is_standing(read(index));
    let built: Vec<usize> = (0..count).filter(|index| up(*index)).collect();
    let mut rows: Vec<usize> = (0..count).filter(|index| !up(*index)).collect();
    // Pagas esperando o canteiro, em andamento, liberadas e bloqueadas.
    rows.sort_by_key(|index| {
        let (percent, paid, _, _, days, locked, tier) = state(*index);
        (if paid { 0 } else if locked { 3 } else if percent > 0 || days > 0 { 1 } else { 2 }, tier, *index)
    });
    let room = if built.is_empty() { ROWS - 1 } else { ROWS - 2 };
    let mut out: Vec<ProjectRow> = rows.into_iter().take(room).map(ProjectRow::Project).collect();
    if !built.is_empty() {
        out.push(ProjectRow::Built(built.len()));
    }
    out
}

fn project_line(project: usize, value: i32) -> (String, u32) {
    let info = &projects::PROJECTS[project];
    let name = Text::tr(&format!("veio.project.{}.name", info.key), &[]).plain();
    let (percent, done, index, left, days, locked, tier) = projects::unpack(value);
    if done && projects::is_standing(value) {
        return (Text::tr("veio.center.project.done", &[&name]).plain(), GOOD);
    }
    if done && projects::is_erected(value) {
        return (Text::tr("veio.center.project.idle", &[&name]).plain(), WARN);
    }
    if done {
        return (Text::tr("veio.center.project.paid", &[&name]).plain(), WARN);
    }
    if days > 0 {
        return (Text::tr("veio.center.project.active", &[&name, &days]).plain(), GOOD);
    }
    if locked {
        return (Text::tr("veio.center.project.locked", &[&name, &tier_name(tier)]).plain(), presentation::SOFT_INK);
    }
    let need = &info.needs[index.min(2)];
    (Text::tr("veio.center.project.progress", &[&name, &percent, &left, &Text::tr(need.name, &[]).plain()]).plain(),
        if percent > 0 { WARN } else { presentation::INK })
}

fn decree_name(index: i32) -> String {
    Text::tr(&format!("veio.decree.{}", index.clamp(0, 3)), &[]).plain()
}

fn request_line(value: i32, post_office: u32) -> (String, u32) {
    let Some(request) = requests::unpack(value) else {
        return (Text::tr("veio.request.empty", &[]).plain(), presentation::SOFT_INK);
    };
    let name = Text::tr(requests::POOL[request.wish].name, &[]).plain();
    let reward = requests::reward(request.wish, post_office);
    if request.done() {
        (Text::tr("veio.request.line_done", &[&request.need, &name]).plain(), GOOD)
    } else {
        (Text::tr("veio.request.line", &[&request.have, &request.need, &name, &reward]).plain(),
            if request.have > 0 { WARN } else { presentation::INK })
    }
}

/// Tela do centro: a página escolhida, as metas (no resumo) e as dicas de uso.
pub(crate) fn draw_screen(host: &dyn Host, event: &mut horizon_client::mod_api::ScreenDrawEvent<'_>) {
    let [x, y] = event.origin;
    let read = |index: usize| event.data.get(index).copied().unwrap_or(0);
    let tr = |key: &str, args: &[&dyn std::fmt::Display]| Text::tr(key, args).plain();
    let tab = (read(0) / 1000).clamp(0, super::super::holding::TABS - 1);
    let page = ((read(0) / 100) % 10).clamp(0, PAGES - 1);
    let status = (read(0) % 100).clamp(0, presentation::STATUS_KEYS.len() as i32 - 1) as usize;
    let index = if tab == 0 { page } else { PAGES - 1 + tab };
    // Cabeçalho: < nome da visão (n/total) > — as setas trocam de visão (clique), como a chave.
    host.text(event.batch, "<", [x + 8.0, y + 22.0], 0x1f4fa8, false);
    host.text(event.batch, ">", [x + 166.0, y + 22.0], 0x1f4fa8, false);
    let name = if tab == 0 { tr(&format!("veio.center.page.{page}"), &[]) } else { tr(&format!("veio.holding.tab{tab}"), &[]) };
    let title = fit(host, &format!("{name} ({}/{VIEWS})", index + 1), 140.0);
    host.text(event.batch, &title, [x + 22.0, y + 22.0], 0x1f4fa8, false);
    if tab > 0 {
        let lines: Vec<(String, u32)> = if tab == super::super::holding::FOREIGN {
            let mut lines = vec![
                (if read(5) > 0 { tr("veio.trade.reputation", &[&read(1), &read(2), &read(5)]) } else { tr("veio.trade.reputation_max", &[&read(1)]) },
                    0x1f4fa8),
                (tr("veio.trade.flows", &[&read(3), &read(4), &(read(3) - read(4))]), if read(3) >= read(4) { 0x1e7a1e } else { 0xaa1f1f }),
            ];
            for slot in 0..3 {
                let (product, need) = (read(6 + slot * 2), read(7 + slot * 2));
                if product < 0 {
                    continue;
                }
                let Some(&(_, key, _, _)) = society::trade::PRODUCTS.get((product / 1000) as usize) else { continue };
                lines.push((tr("veio.trade.contract", &[&tr(key, &[]), &(need % 10000), &(need / 10000), &(product % 1000)]),
                    presentation::INK));
            }
            lines
        } else {
            let stopped = read(5).clamp(0, presentation::STATUS_KEYS.len() as i32 - 1) as usize;
            vec![
                (tr("veio.holding.units", &[&read(1), &read(2), &read(3)]), presentation::INK),
                (tr(&format!("veio.holding.made{tab}"), &[&read(4)]), presentation::INK),
                (tr("veio.holding.profit", &[&read(10), &read(11), &(read(10) - read(11))]),
                    if read(10) >= read(11) { GOOD } else { 0xaa1f1f }),
                (if read(6) > 0 {
                    tr("veio.holding.stopped", &[&tr(&format!("veio.status.{}", presentation::STATUS_KEYS[stopped]), &[]), &read(6)])
                } else { tr("veio.holding.all_good", &[]) }, if read(6) > 0 { WARN } else { GOOD }),
                (tr(&format!("veio.holding.extra{tab}"), &[&read(7), &read(8), &read(9)]), presentation::SOFT_INK),
            ]
        };
        for (index, (line, color)) in lines.iter().enumerate() {
            host.text(event.batch, &fit(host, line, 176.0), [x + 8.0, y + TOP + index as f32 * ROW], *color, false);
        }
        return;
    }
    let mut lines: Vec<(String, u32)> = vec![(tr(&format!("veio.status.{}", presentation::STATUS_KEYS[status]), &[]), presentation::text_color(status))];
    match page {
        0 => {
            let (workers, working) = split(read(1));
            let (waiting, _hungry) = split(read(2));
            let (fields, pens) = split(read(3));
            let (villagers, cap) = split(read(7));
            let (queued, filled) = split(read(5));
            let storage = filled - 1;
            lines.push((tr("veio.center.population", &[&villagers, &cap, &read(8)]), if read(8) < 16 { WARN } else { presentation::INK }));
            lines.push((tr("veio.center.crew", &[&workers, &working, &waiting]), if waiting > 0 { WARN } else { presentation::INK }));
            // Linha da companhia: clicar saca uma esmeralda (`holding::click`).
            lines.push((tr("veio.center.company", &[&read(12), &read(6)]), 0x1f4fa8));
            lines.push((tr("veio.center.zones_ready", &[&fields, &pens, &read(4)]), if read(4) > 0 { GOOD } else { presentation::INK }));
            lines.push((if storage < 0 { tr("veio.center.no_storage", &[&queued]) } else { tr("veio.center.storage", &[&storage, &queued]) },
                if storage >= 90 { WARN } else { presentation::INK }));
        }
        1 => {
            let count = read(1);
            lines[0] = if count == 0 { (tr("veio.center.no_issues", &[]), GOOD) } else { (tr("veio.center.issues", &[&count]), WARN) };
            for index in 0..MAX_ISSUES.min(ROWS - 1) {
                let value = read(index + 2);
                if value != 0 {
                    lines.push((issue_line(value), presentation::INK));
                }
            }
        }
        2 => {
            let points = read(projects::PROJECTS.len() + 1);
            lines[0] = (tr("veio.center.projects", &[&tier_name(projects::tier_of(points))]), presentation::INK);
            for row in project_rows(event.data) {
                lines.push(match row {
                    ProjectRow::Project(index) => project_line(index, read(index + 1)),
                    ProjectRow::Built(count) => (tr("veio.center.project.built", &[&count]), GOOD),
                });
            }
        }
        _ => {
            let points = read(1);
            let tier = projects::tier_of(points);
            lines[0] = match projects::TIERS.get(tier as usize + 1) {
                Some(next) => (tr("veio.center.prestige", &[&tier_name(tier), &points, next, &read(8)]), presentation::INK),
                None => (tr("veio.center.prestige_max", &[&tier_name(tier), &points, &read(8)]), GOOD),
            };
            let flags = read(2);
            lines.push((tr("veio.center.treasury", &[&(read(12) / society::COINS_PER_EMERALD), &decree_name(flags & 15)]), presentation::INK));
            let slots = ((flags >> 4) & 7) as usize;
            let post_office = (flags as u32) & projects::POST_OFFICE;
            for index in 0..slots.min(requests::SLOTS) {
                lines.push(request_line(read(4 + index), post_office));
            }
        }
    }
    for (index, (line, color)) in lines.iter().enumerate().take(ROWS) {
        host.text(event.batch, &fit(host, line, 176.0), [x + 8.0, y + TOP + index as f32 * ROW], *color, false);
    }
    if page == 0 {
        for goal in 0..3 {
            let top = y + SLOTS_Y + goal as f32 * 18.0;
            let percent = read(9 + goal);
            let left = x + 38.0;
            host.fill(event.batch, [left, top + 4.0, 70.0, 7.0], BAR);
            if percent > 0 {
                let color = if percent >= 100 { 0xff3fae3f } else { 0xffe0a020 };
                host.fill(event.batch, [left + 1.0, top + 5.0, 68.0 * percent.min(100) as f32 / 100.0, 5.0], color);
            }
            let label = if percent < 0 { tr("veio.center.goal_empty", &[]) } else { format!("{percent}%") };
            host.text(event.batch, &label, [left + 74.0, top + 3.0], presentation::SOFT_INK, false);
        }
    } else {
        let hints = match page {
            1 => ["veio.center.hint.compass", "veio.center.hint.hover"],
            2 => ["veio.center.hint.site", "veio.center.hint.hover"],
            _ => ["veio.center.hint.deliver", "veio.center.hint.cash"],
        };
        for (index, key) in hints.iter().chain(["veio.center.hint.page"].iter()).enumerate() {
            host.text(event.batch, &fit(host, &tr(key, &[]), 108.0), [x + 38.0, y + SLOTS_Y + 3.0 + index as f32 * 11.0], presentation::SOFT_INK, false);
        }
    }
}

/// Dica ao passar o cursor por uma linha do painel. `true` se tratou o evento.
pub(crate) fn tooltip(event: &mut horizon_client::mod_api::ScreenTooltipEvent<'_>) -> bool {
    let [x, y] = event.at;
    if !(8.0..186.0).contains(&x) || !(TOP..TOP + ROWS as f32 * ROW).contains(&y) {
        return false;
    }
    let row = ((y - TOP) / ROW) as usize;
    let read = |index: usize| event.data.get(index).copied().unwrap_or(0);
    let tr = |key: &str, args: &[&dyn std::fmt::Display]| Text::tr(key, args).plain();
    if read(0) / 1000 > 0 {
        return false;
    }
    let page = ((read(0) / 100) % 10).clamp(0, PAGES - 1);
    if row == 0 && page == 0 {
        event.lines.push(tr(&operations::advice_key(read(0) % 100), &[]));
        return true;
    }
    let mut lines: Vec<String> = Vec::new();
    match page {
        0 => {
            let key = ["", "population", "workers", "company", "zones", "storage"].get(row).copied().unwrap_or("");
            if key.is_empty() {
                return false;
            }
            lines.push(tr(&format!("veio.center.tip.{key}"), &[]));
        }
        1 => {
            if row == 0 {
                event.lines.push(tr("veio.center.hint.compass", &[]));
                return true;
            }
            let value = read(row + 1);
            if value == 0 {
                return false;
            }
            let (code, _, dx, dy, dz) = unpack(value);
            lines.push(issue_line(value));
            lines.push(advice(code));
            lines.push(tr("veio.center.tip.offset", &[&dx, &dy, &dz]));
        }
        2 => {
            let rows = project_rows(event.data);
            match row.checked_sub(1).and_then(|index| rows.get(index)) {
                None => lines.push(tr("veio.center.tip.prestige", &[])),
                Some(ProjectRow::Built(_)) => {
                    for (index, info) in projects::PROJECTS.iter().enumerate() {
                        if projects::is_standing(read(index + 1)) {
                            lines.push(format!("{}: {}", tr(&format!("veio.project.{}.name", info.key), &[]),
                                tr(&format!("veio.project.{}.effect", info.key), &[]).replace('\n', " ")));
                        }
                    }
                }
                Some(ProjectRow::Project(index)) => {
                    let info = &projects::PROJECTS[*index];
                    lines.push(tr(&format!("veio.project.{}.name", info.key), &[]));
                    lines.push(tr(&format!("veio.project.{}.effect", info.key), &[]));
                    let list: Vec<String> = info.needs.iter().map(|need| format!("{} {}", need.count, tr(need.name, &[]))).collect();
                    lines.push(tr("veio.project.needs", &[&list.join(", ")]));
                    if info.tier > 0 {
                        lines.push(tr("veio.project.tier", &[&tier_name(info.tier)]));
                    }
                    if !info.repeat {
                        lines.push(tr("veio.project.building", &[]));
                    }
                    if info.repeat {
                        lines.push(tr("veio.project.repeat", &[]));
                    }
                }
            }
        }
        _ => match row {
            0 => lines.push(tr("veio.center.tip.prestige", &[])),
            1 => {
                lines.push(tr("veio.center.tip.treasury", &[]));
                lines.push(tr(&format!("veio.decree.{}.effect", (read(2) & 15).clamp(0, 3)), &[]));
            }
            _ => {
                let slots = ((read(2) >> 4) & 7) as usize;
                let index = row - 2;
                if index >= slots.min(requests::SLOTS) {
                    lines.push(tr("veio.center.tip.served", &[]));
                } else if let Some(request) = requests::unpack(read(4 + index)) {
                    let reward = requests::reward(request.wish, (read(2) as u32) & projects::POST_OFFICE);
                    lines.push(tr("veio.request.tip", &[&reward, &requests::PRESTIGE]));
                    lines.push(tr("veio.request.tip_keep", &[]));
                } else {
                    lines.push(tr("veio.request.tip_keep", &[]));
                }
            }
        },
    }
    event.lines.extend(lines.iter().flat_map(|line| line.lines().map(str::to_owned)));
    true
}

pub(crate) fn screen_slots() -> Vec<Option<[f32; 2]>> {
    vec![Some([170.0, SLOTS_Y]), Some([17.0, SLOTS_Y]), Some([17.0, SLOTS_Y + 18.0]), Some([17.0, SLOTS_Y + 36.0]),
        // Cofre e saque lado a lado; obra embaixo; decreto ao lado da comida.
        Some([170.0, SLOTS_Y + 18.0]), Some([152.0, SLOTS_Y + 18.0]),
        Some([170.0, SLOTS_Y + 36.0]), Some([152.0, SLOTS_Y])]
}

/// Relatório no chat para o jogador mais próximo.
pub(crate) fn report_to_chat(host: &mut dyn Host, pos: [i32; 3]) -> Option<Flow> {
    let mut admin = None;
    let mut targets = Vec::new();
    let mut company = None;
    host.change_mod_block_entity(pos, &mut |entity| {
        if entity.id == CONTROL_CENTER {
            admin = Some(worker(entity).is_some() && number(entity, "Admin") == 1);
            targets = goals(entity);
            company = Some(society::ledger::company_report(entity));
        }
    });
    let admin = admin?;
    let Some(player) = nearest_player(host, pos, 12.0) else { return Some(Flow::Cancel) };
    if !admin {
        host.tell(player, Text::tr("veio.center.need_admin", &[]));
        return Some(Flow::Cancel);
    }
    let dimension = host.dimension();
    let now = defense::clock();
    let posts: Vec<PostInfo> = posts_near(dimension, pos, RADIUS, now).into_iter().filter(|post| post.pos != pos).collect();
    let working = posts.iter().filter(|post| post.staffed && matches!(post.status, 1 | WALKING)).count();
    host.tell(player, Text::tr("veio.report.title", &[&posts.len(), &working]));
    if let Some(company) = company {
        host.tell(player, company);
    }
    for line in super::super::holding::report_lines(dimension) {
        host.tell(player, line);
    }
    // Exportação: os itens guardados que hoje rendem mais no mercado externo (por unidade).
    let day = host.level(dimension).and_then(|level| level.game_time(host)).unwrap_or(0).div_euclid(society::DAY);
    let mut best: Vec<(i32, String)> = Vec::new();
    for store in storage::stores_near(dimension, pos, RADIUS, now) {
        for (item, _) in store.items {
            if !best.iter().any(|(_, known)| *known == item)
                && let Some(price) = society::trade::quote(dimension, &item, day)
            {
                best.push((price, item));
            }
        }
    }
    best.sort_by(|a, b| b.0.cmp(&a.0));
    for (price, item) in best.into_iter().take(3) {
        host.tell(player, Text::tr("veio.export.best", &[&item, &price]));
    }
    // Bolsa do dia: as cinco melhores cotações abertas, com a tendência.
    for (product, price, trend) in society::trade::board(dimension, day, 5) {
        let arrow = if trend > 0 { "+" } else if trend < 0 { "-" } else { "=" };
        host.tell(player, Text::tr("veio.trade.board", &[&Text::tr(society::trade::PRODUCTS[product].1, &[]).plain(),
            &price, &format!("{arrow}{}", trend.abs())]));
    }
    for post in posts.iter().filter(|post| waiting(post.status)).take(REPORT_LINES) {
        let key = format!("veio.report.issue.{}", presentation::STATUS_KEYS.get(post.status as usize).copied().unwrap_or("idle"));
        host.tell(player, Text::tr(&key, &[&format_pos(post.pos)]));
    }
    for zone in zones::zones_near(dimension, pos, RADIUS, now) {
        let kind = if zone.kind == zones::Kind::Pen { "veio.report.pen" } else { "veio.report.field" };
        let next = zones::requirement(zone.unlocked).map_or_else(
            || Text::tr("veio.zone.max", &[]),
            |(cost, work, villagers)| Text::tr("veio.report.next", &[&cost, &zone.work, &work, &zone.villagers, &villagers]));
        host.tell(player, Text::tr(kind, &[&format_pos(zone.pos), &zone.level, &zone.unlocked, &zone.side, &zone.side]));
        host.tell(player, next);
    }
    let stores = storage::stores_near(dimension, pos, RADIUS, now);
    let (used, total) = stores.iter().fold((0, 0), |(u, t), store| (u + store.used, t + store.total));
    host.tell(player, Text::tr("veio.report.storage", &[&stores.len(), &used, &total]));
    for (index, goal) in targets.into_iter().enumerate() {
        if let Some((item, target)) = goal {
            let have: u32 = stores.iter().flat_map(|store| &store.items).filter(|(known, _)| *known == item).map(|(_, count)| count).sum();
            host.tell(player, Text::tr("veio.report.goal", &[&(index + 1), &item, &have.min(target), &target]));
        }
    }
    host.tell(player, Text::tr("veio.report.compass", &[]));
    Some(Flow::Cancel)
}

/// Jogador mais próximo do bloco, a até `reach` blocos, na mesma dimensão.
pub(crate) fn nearest_player(host: &dyn Host, pos: [i32; 3], reach: f64) -> Option<u128> {
    nearest_server_player(host, pos, reach).map(|player| player.uuid)
}

pub(crate) fn nearest_server_player(host: &dyn Host, pos: [i32; 3], reach: f64) -> Option<horizon_client::mod_api::NativeServerPlayer> {
    let at = center(pos);
    let dimension = host.dimension();
    host.server_players().into_iter()
        .filter(|player| player.dimension == dimension && distance2(player.position, at) < reach * reach)
        .min_by(|a, b| distance2(a.position, at).total_cmp(&distance2(b.position, at)))
}

pub(crate) fn distance2(a: [f64; 3], b: [f64; 3]) -> f64 {
    (0..3).map(|axis| (a[axis] - b[axis]).powi(2)).sum()
}

pub(crate) fn format_pos([x, y, z]: [i32; 3]) -> String {
    format!("{x} {y} {z}")
}
