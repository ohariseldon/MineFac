//! Centro de controle: o painel de gestão da vila operária.
//!
//! Um aldeão contratado (o administrador) fica no centro. Enquanto ele está lá, o painel junta o
//! que os postos, zonas e armazéns a até `RADIUS` blocos publicam nos registros em memória:
//! quantos trabalhadores há, quantos trabalham, quantos esperam e por quê, zonas, ocupação dos
//! armazéns, tamanho das filas e o progresso de até três **metas** de estoque.
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
/// O administrador gasta comida a cada `CHARGE_EVERY` atualizações do painel (cerca de 2 min).
const CHARGE_EVERY: i32 = 12;
const REFRESH_EVERY: i32 = 10;
const STALE: u64 = 200;
const REPORT_LINES: usize = 8;

pub(crate) fn accepts(slot: usize, stack: &ItemStack) -> bool {
    plain(stack) && if slot == 0 { is_food(&stack.item) } else { GOAL_SLOTS.contains(&slot) }
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
    pub seen: u64,
}

static POSTS: Mutex<Vec<PostInfo>> = Mutex::new(Vec::new());
static POSTS_PENDING: AtomicBool = AtomicBool::new(false);

/// Chamado pelos postos a cada ciclo.
pub(crate) fn report(dimension: &str, pos: [i32; 3], entity: &ModBlockEntity, now: u64) {
    let info = PostInfo {
        dimension: dimension.to_owned(), pos, block: entity.id.clone(), status: number(entity, "Status"),
        staffed: worker(entity).is_some(), queued: number(entity, "Queued"), seen: now,
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

/// Esperando por algo que o jogador ou o capataz precisa resolver.
pub(crate) fn waiting(status: i32) -> bool {
    matches!(status, 3 | NO_WORKER | ABSENT | NO_FOOD | PATH | NO_WAGE)
}

/// O administrador fica no centro.
pub(crate) fn plan(_host: &mut dyn Host, pos: [i32; 3], _entity: &mut ModBlockEntity, _now: u64) -> Result<Plan, i32> {
    Ok(Plan::Visit { spot: pos, reach: POST_REACH })
}

/// Com o administrador ao lado, atualiza o painel. A cada `CHARGE_EVERY` atualizações devolve
/// uma tarefa concluída (gasta comida); nas outras, nada.
pub(crate) fn perform(host: &mut dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity, now: u64) -> Option<Vec<(String, u32)>> {
    set_number(entity, "Admin", 1);
    let round = number(entity, "Refresh") + 1;
    set_number(entity, "Refresh", round % (REFRESH_EVERY * CHARGE_EVERY));
    if round % REFRESH_EVERY == 1 || !entity.nbt.contains_key(nbt_key("DWorkers")) {
        refresh(host.dimension(), pos, entity, now);
        // Com administrador, o centro de controle é o centro da vila: spawn e renascimento aqui.
        spawn::control_center(host, pos);
    }
    (round % (REFRESH_EVERY * CHARGE_EVERY) == 0).then(Vec::new)
}

fn goals(entity: &ModBlockEntity) -> Vec<Option<(String, u32)>> {
    GOAL_SLOTS.map(|slot| entity.items.get(slot).and_then(Option::as_ref)
        .map(|stack| (stack.item.to_string(), stack.count * META_UNIT))).collect()
}

/// Recalcula os números do painel a partir dos registros.
pub(crate) fn refresh(dimension: &str, pos: [i32; 3], entity: &mut ModBlockEntity, now: u64) {
    let posts: Vec<PostInfo> = posts_near(dimension, pos, RADIUS, now).into_iter().filter(|post| post.pos != pos).collect();
    let zones = zones::zones_near(dimension, pos, RADIUS, now);
    let stores = storage::stores_near(dimension, pos, RADIUS, now);
    let workers = posts.iter().filter(|post| post.staffed).count();
    let working = posts.iter().filter(|post| post.staffed && matches!(post.status, 1 | WALKING)).count();
    let waiting = posts.iter().filter(|post| waiting(post.status)).count();
    let hungry = posts.iter().filter(|post| post.status == NO_FOOD).count();
    let (used, total) = stores.iter().fold((0, 0), |(u, t), store| (u + store.used, t + store.total));
    let stored = |item: &str| stores.iter().flat_map(|store| &store.items).filter(|(known, _)| known == item).map(|(_, count)| count).sum::<u32>();
    let progress: Vec<i32> = goals(entity).into_iter().map(|goal| goal.map_or(-1, |(item, target)| {
        (stored(&item).min(target) * 100 / target.max(1)) as i32
    })).collect();
    for (key, value) in [
        ("DWorkers", workers as i32), ("DWorking", working as i32), ("DWaiting", waiting as i32), ("DHungry", hungry as i32),
        ("DFields", zones.iter().filter(|zone| zone.kind == zones::Kind::Field).count() as i32),
        ("DPens", zones.iter().filter(|zone| zone.kind == zones::Kind::Pen).count() as i32),
        ("DStore", if total == 0 { -1 } else { (used * 100 / total) as i32 }),
        ("DQueue", posts.iter().map(|post| post.queued).sum()),
        ("DGoal1", progress[0]), ("DGoal2", progress[1]), ("DGoal3", progress[2]),
    ] {
        entity.nbt.insert(key.into(), Tag::Int(value));
    }
}

fn signed(entity: &ModBlockEntity, key: &str) -> i32 {
    match entity.nbt.get(nbt_key(key)) { Some(Tag::Int(value)) => *value, _ => 0 }
}

/// Os 13 valores do painel do centro.
pub(crate) fn data(entity: &ModBlockEntity) -> [i32; 13] {
    let keys = ["DWorkers", "DWorking", "DWaiting", "DHungry", "DFields", "DPens", "DStore", "DQueue",
        "DGoal1", "DGoal2", "DGoal3"];
    std::array::from_fn(|index| match index {
        0 => number(entity, "Status"), 12 => number(entity, "Info"), _ => signed(entity, keys[index - 1]),
    })
}

const BAR: u32 = 0xff2b2b2b;

/// Tela do centro: números da vila e barras das metas.
pub(crate) fn draw_screen(host: &dyn Host, event: &mut horizon_client::mod_api::ScreenDrawEvent<'_>) {
    let [x, y] = event.origin;
    let read = |index: usize| event.data.get(index).copied().unwrap_or(0);
    let tr = |key: &str, args: &[&dyn std::fmt::Display]| Text::tr(key, args).plain();
    let status = read(0).clamp(0, presentation::STATUS_KEYS.len() as i32 - 1) as usize;
    let lines = [
        (tr(&format!("veio.status.{}", presentation::STATUS_KEYS[status]), &[]), presentation::text_color(status)),
        (tr("veio.center.workers", &[&read(1), &read(2)]), presentation::INK),
        (tr("veio.center.waiting", &[&read(3), &read(4)]), if read(3) > 0 { 0x9a5a00 } else { presentation::INK }),
        (tr("veio.center.zones", &[&read(5), &read(6)]), presentation::INK),
        (if read(7) < 0 { tr("veio.center.no_storage", &[&read(8)]) } else { tr("veio.center.storage", &[&read(7), &read(8)]) },
            presentation::INK),
    ];
    for (index, (line, color)) in lines.iter().enumerate() {
        host.text(event.batch, line, [x + 8.0, y + 22.0 + index as f32 * 11.0], *color, false);
    }
    for goal in 0..3 {
        let top = y + 82.0 + goal as f32 * 18.0;
        let percent = read(9 + goal);
        let left = x + 38.0;
        host.fill(event.batch, [left, top + 4.0, 100.0, 7.0], BAR);
        if percent > 0 {
            let color = if percent >= 100 { 0xff3fae3f } else { 0xffe0a020 };
            host.fill(event.batch, [left + 1.0, top + 5.0, 98.0 * percent.min(100) as f32 / 100.0, 5.0], color);
        }
        let label = if percent < 0 { tr("veio.center.goal_empty", &[]) } else { format!("{percent}%") };
        host.text(event.batch, &label, [left + 104.0, top + 3.0], presentation::SOFT_INK, false);
    }
}

pub(crate) fn screen_slots() -> Vec<Option<[f32; 2]>> {
    vec![Some([170.0, 82.0]), Some([17.0, 82.0]), Some([17.0, 100.0]), Some([17.0, 118.0])]
}

/// Relatório no chat para o jogador mais próximo.
pub(crate) fn report_to_chat(host: &mut dyn Host, pos: [i32; 3]) -> Option<Flow> {
    let mut admin = None;
    let mut targets = Vec::new();
    host.change_mod_block_entity(pos, &mut |entity| {
        if entity.id == CONTROL_CENTER {
            admin = Some(worker(entity).is_some() && number(entity, "Admin") == 1);
            targets = goals(entity);
        }
    });
    let admin = admin?;
    let at = center(pos);
    let dimension = host.dimension();
    let Some(player) = host.server_players().into_iter()
        .filter(|player| player.dimension == dimension)
        .min_by(|a, b| distance2(a.position, at).total_cmp(&distance2(b.position, at)))
        .filter(|player| distance2(player.position, at) < 12.0 * 12.0)
        .map(|player| player.uuid) else { return Some(Flow::Cancel) };
    if !admin {
        host.tell(player, Text::tr("veio.center.need_admin", &[]));
        return Some(Flow::Cancel);
    }
    let now = defense::clock();
    let posts: Vec<PostInfo> = posts_near(dimension, pos, RADIUS, now).into_iter().filter(|post| post.pos != pos).collect();
    let working = posts.iter().filter(|post| post.staffed && matches!(post.status, 1 | WALKING)).count();
    host.tell(player, Text::tr("veio.report.title", &[&posts.len(), &working]));
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
    Some(Flow::Cancel)
}

fn distance2(a: [f64; 3], b: [f64; 3]) -> f64 {
    (0..3).map(|axis| (a[axis] - b[axis]).powi(2)).sum()
}

fn format_pos([x, y, z]: [i32; 3]) -> String {
    format!("{x} {y} {z}")
}
