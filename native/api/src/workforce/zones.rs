//! Zonas de trabalho: marcos que definem a área de uma lavoura ou de um cercado.
//!
//! - O marco fica no centro da zona. A área é um quadrado de `SIDES[nível]` blocos, na camada
//!   do marco, na de baixo e na de cima.
//! - Postos do tipo certo (lavrador para lavoura, pastor para cercado) a até `LINK` blocos da
//!   borda trabalham na zona inteira em vez da área fixa em volta do posto. Vários postos podem
//!   usar a mesma zona; as reservas evitam que dois aldeões peguem o mesmo alvo.
//! - Níveis: 9x9, 13x13, 17x17 e 21x21 (cerca de cinco vezes uma horta de vila). Cada nível novo
//!   custa esmeraldas e exige trabalho feito na zona e aldeões na vila (`requirement`). O jogador
//!   escolhe o tamanho ativo entre os níveis liberados com a chave.
//! - O slot do marco é um filtro: na lavoura, a semente que os lavradores plantam; no cercado, a
//!   ração que o pastor usa (e assim as espécies que ele cria).
//!
//! Os marcos se registram em `ZONES` (memória) a cada segundo; os postos consultam o registro, sem
//! varrer blocos para achar a zona.
use super::*;
use horizon_client::mod_api::{BlockEntitySubmitEvent, CubeFace};
use std::sync::atomic::{AtomicBool, Ordering};

pub(crate) const FIELD_MARKER: &str = "veio:field_marker";
pub(crate) const PEN_MARKER: &str = "veio:pen_marker";
/// Lado da zona em cada nível.
pub(crate) const SIDES: [i32; 4] = [9, 13, 17, 21];
/// Para liberar o nível 2, 3 e 4: esmeraldas, tarefas feitas na zona e aldeões a até 48 blocos.
pub(crate) const COST: [u32; 3] = [8, 16, 32];
pub(crate) const WORK: [i32; 3] = [32, 128, 384];
pub(crate) const VILLAGERS: [i32; 3] = [6, 12, 20];
const VILLAGE_RADIUS: f64 = 48.0;
/// Postos a até esta distância fora da borda usam a zona.
pub(crate) const LINK: i32 = 3;
const STALE: u64 = 200;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Kind {
    Field,
    Pen,
}

pub(crate) fn is_marker(block: &str) -> bool {
    matches!(block, FIELD_MARKER | PEN_MARKER)
}

fn kind_of(block: &str) -> Kind {
    if block == PEN_MARKER { Kind::Pen } else { Kind::Field }
}

pub(crate) fn layout(block: &str) -> Option<(usize, Vec<usize>, Vec<usize>)> {
    // Um slot de filtro, sem automação.
    is_marker(block).then(|| (1, vec![], vec![]))
}

pub(crate) fn accepts(block: &str, slot: usize, stack: &ItemStack) -> bool {
    slot == 0 && stack.extra.is_none() && match block {
        FIELD_MARKER => production::crops::planted(&stack.item).is_some(),
        PEN_MARKER => production::ranch::is_feed(&stack.item),
        _ => false,
    }
}

pub(crate) fn unlocked(entity: &ModBlockEntity) -> i32 {
    number(entity, "Unlocked").clamp(1, SIDES.len() as i32)
}

pub(crate) fn level(entity: &ModBlockEntity) -> i32 {
    number(entity, "Level").clamp(1, unlocked(entity))
}

pub(crate) fn side(entity: &ModBlockEntity) -> i32 {
    SIDES[level(entity) as usize - 1]
}

fn filter(entity: &ModBlockEntity) -> Option<String> {
    entity.items.first().and_then(Option::as_ref).map(|stack| stack.item.to_string())
}

/// Zona registrada.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Zone {
    pub dimension: String,
    pub pos: [i32; 3],
    pub kind: Kind,
    pub level: i32,
    pub unlocked: i32,
    pub side: i32,
    pub work: i32,
    pub villagers: i32,
    pub filter: Option<String>,
    pub seen: u64,
}

static ZONES: Mutex<Vec<Zone>> = Mutex::new(Vec::new());
static ZONES_PENDING: AtomicBool = AtomicBool::new(false);

fn register(zone: Zone) {
    if let Ok(mut zones) = ZONES.lock() {
        let now = zone.seen;
        if let Some(known) = zones.iter_mut().find(|known| known.dimension == zone.dimension && known.pos == zone.pos) {
            *known = zone;
        } else {
            zones.retain(|known| now.saturating_sub(known.seen) <= STALE);
            zones.push(zone);
        }
        ZONES_PENDING.store(true, Ordering::Release);
    }
}

/// Zonas vivas nesta dimensão a até `radius` blocos de `at`.
pub(crate) fn zones_near(dimension: &str, at: [i32; 3], radius: i32, now: u64) -> Vec<Zone> {
    if !ZONES_PENDING.load(Ordering::Acquire) {
        return Vec::new();
    }
    // Leitura sem efeito colateral: entradas vencidas saem nas escritas (`register`/`report`).
    ZONES.lock().map(|zones| {
        if zones.is_empty() {
            ZONES_PENDING.store(false, Ordering::Release);
            return Vec::new();
        }
        zones.iter()
            .filter(|zone| zone.dimension == dimension && now.saturating_sub(zone.seen) <= STALE
                && (0..3).all(|axis| (zone.pos[axis] - at[axis]).abs() <= radius))
            .cloned().collect()
    }).unwrap_or_default()
}

/// A zona do tipo `kind` que o posto em `post` usa: a mais próxima cuja borda esteja a até
/// `LINK` blocos dele, na mesma altura (até 3 blocos).
pub(crate) fn zone_for(dimension: &str, post: [i32; 3], kind: Kind, now: u64) -> Option<Zone> {
    zones_near(dimension, post, SIDES[SIDES.len() - 1] / 2 + LINK, now).into_iter()
        .filter(|zone| zone.kind == kind && (zone.pos[1] - post[1]).abs() <= 3
            && (zone.pos[0] - post[0]).abs() <= zone.side / 2 + LINK
            && (zone.pos[2] - post[2]).abs() <= zone.side / 2 + LINK)
        .min_by_key(|zone| (zone.pos[0] - post[0]).abs() + (zone.pos[2] - post[2]).abs())
}

/// A posição está dentro da zona (horizontalmente, e até 3 blocos de altura)?
pub(crate) fn contains(zone: &Zone, at: [f64; 3]) -> bool {
    let half = f64::from(zone.side / 2) + 0.5;
    (at[0] - f64::from(zone.pos[0]) - 0.5).abs() <= half && (at[2] - f64::from(zone.pos[2]) - 0.5).abs() <= half
        && (at[1] - f64::from(zone.pos[1])).abs() <= 3.0
}

/// Conta uma tarefa feita na zona (colheita, plantio, tosquia...). Libera níveis.
pub(crate) fn credit(host: &mut dyn Host, marker: [i32; 3]) {
    host.change_mod_block_entity(marker, &mut |entity| {
        if is_marker(&entity.id) {
            set_number(entity, "Work", number(entity, "Work").saturating_add(1));
        }
    });
}

/// O que falta para o próximo nível: (esmeraldas, tarefas, aldeões); `None` no nível máximo.
pub(crate) fn requirement(unlocked: i32) -> Option<(u32, i32, i32)> {
    let next = unlocked as usize - 1;
    (next < COST.len()).then(|| (COST[next], WORK[next], VILLAGERS[next]))
}

pub(crate) fn tick(host: &mut dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity, time: i64) {
    let phase = (time + i64::from((pos[0] ^ pos[2]) & 15)).rem_euclid(20);
    if phase != 0 && entity.nbt.contains_key(nbt_key("Villagers")) {
        return;
    }
    if (time + i64::from(pos[0] & 15)).rem_euclid(200) == 0 || !entity.nbt.contains_key(nbt_key("Villagers")) {
        let center = [f64::from(pos[0]) + 0.5, f64::from(pos[1]), f64::from(pos[2]) + 0.5];
        set_number(entity, "Villagers", production::ranch::villagers_near(host, center, VILLAGE_RADIUS));
    }
    if !entity.nbt.contains_key(nbt_key("Unlocked")) {
        set_number(entity, "Unlocked", 1);
        set_number(entity, "Level", 1);
    }
    set_number(entity, "Status", ZONE_ACTIVE);
    register(Zone {
        dimension: host.dimension().to_owned(),
        pos,
        kind: kind_of(&entity.id),
        level: level(entity),
        unlocked: unlocked(entity),
        side: side(entity),
        work: number(entity, "Work"),
        villagers: number(entity, "Villagers"),
        filter: filter(entity),
        seen: defense::clock(),
    });
}

/// Dados do painel do marco (13 valores, como os demais painéis).
pub(crate) fn data(entity: &ModBlockEntity) -> [i32; 13] {
    let (cost, work, villagers) = requirement(unlocked(entity)).map_or((0, 0, 0), |(c, w, v)| (c as i32, w, v));
    [number(entity, "Status"), level(entity), unlocked(entity), side(entity), number(entity, "Work"), work,
        number(entity, "Villagers"), villagers, cost, number(entity, "Work"), 0, 0, i32::from(entity.id == PEN_MARKER)]
}

pub(crate) fn panel_lines(block: &str, data: &[i32]) -> Option<Vec<String>> {
    if !is_marker(block) {
        return None;
    }
    let read = |index: usize| data.get(index).copied().unwrap_or(0);
    let tr = |key: &str, args: &[&dyn std::fmt::Display]| Text::tr(key, args).plain();
    let next = if read(8) == 0 {
        tr("veio.zone.max", &[])
    } else {
        tr("veio.zone.next", &[&read(8), &read(4), &read(5), &read(6), &read(7)])
    };
    Some(vec![tr("veio.zone.size", &[&read(1), &read(2), &read(3), &read(3)]), next,
        tr(if block == PEN_MARKER { "veio.hint.pen_marker" } else { "veio.hint.field_marker" }, &[])])
}

/// Esmeralda libera o próximo nível; chave escolhe o tamanho ativo. Devolve `None` se o item não
/// for de zona (o clique segue para o resto do Veio).
pub(crate) fn right_click(host: &mut dyn Host, pos: [i32; 3], held: &str) -> Option<Flow> {
    let mut state = None;
    host.change_mod_block_entity(pos, &mut |entity| {
        if is_marker(&entity.id) {
            state = Some((unlocked(entity), level(entity), number(entity, "Work"), number(entity, "Villagers")));
        }
    });
    let (unlocked, level, work, villagers) = state?;
    if held == WRENCH && !host.sneaking() {
        let next = level % unlocked + 1;
        host.change_mod_block_entity(pos, &mut |entity| set_number(entity, "Level", next));
        host.block_entity_sync(pos);
        notify(host, pos, &format!("veio.zone.level{next}"));
        return Some(Flow::Cancel);
    }
    if held != WAGE {
        return None;
    }
    let Some((cost, need_work, need_villagers)) = requirement(unlocked) else {
        notify(host, pos, "veio.zone.max");
        return Some(Flow::Cancel);
    };
    if work < need_work {
        notify(host, pos, "veio.zone.need_work");
        return Some(Flow::Cancel);
    }
    if villagers < need_villagers {
        notify(host, pos, "veio.zone.need_villagers");
        return Some(Flow::Cancel);
    }
    let taken = host.take_items(WAGE, cost);
    if taken < cost {
        if taken > 0 {
            host.give_items(WAGE, taken);
        }
        notify(host, pos, "veio.zone.need_emeralds");
        return Some(Flow::Cancel);
    }
    host.change_mod_block_entity(pos, &mut |entity| {
        set_number(entity, "Unlocked", unlocked + 1);
        set_number(entity, "Level", unlocked + 1);
    });
    host.block_entity_sync(pos);
    host.play_sound("minecraft:entity.player.levelup", center(pos), 4, 0.6, 1.0);
    notify(host, pos, &format!("veio.zone.level{}", unlocked + 1));
    Some(Flow::Cancel)
}

/// Área de desenho do marco: cobre a borda da zona.
pub(crate) fn bounds(entity: &ModBlockEntity, bounds: &mut [f32; 6]) {
    let half = (side(entity) / 2) as f32;
    bounds[0] = bounds[0].min(-half - 0.2);
    bounds[2] = bounds[2].min(-half - 0.2);
    bounds[3] = bounds[3].max(half + 1.2);
    bounds[5] = bounds[5].max(half + 1.2);
    bounds[4] = bounds[4].max(1.5);
}

const LINE: [Option<CubeFace<'static>>; 6] = [Some(CubeFace { sprite: "veio:gui/material", uv: None }); 6];

/// Borda da zona no chão e estacas nos cantos: verde na lavoura, marrom no cercado.
pub(crate) fn draw(host: &dyn Host, event: &mut BlockEntitySubmitEvent<'_>) -> bool {
    if !is_marker(event.block) {
        return false;
    }
    let half = (side(event.entity) / 2) as f32 * 16.0;
    let (low, high) = (-half, half + 16.0);
    let color = if event.block == PEN_MARKER { 0x8a5a2b } else { 0x4caf3f };
    let y = [0.5, 1.3];
    for (from, to) in [
        ([low, y[0], low], [high, y[1], low + 0.8]),
        ([low, y[0], high - 0.8], [high, y[1], high]),
        ([low, y[0], low], [low + 0.8, y[1], high]),
        ([high - 0.8, y[0], low], [high, y[1], high]),
    ] {
        host.draw_cube(event.draw, from, to, LINE, color);
    }
    for (x, z) in [(low, low), (low, high - 2.0), (high - 2.0, low), (high - 2.0, high - 2.0)] {
        host.draw_cube(event.draw, [x, 0.0, z], [x + 2.0, 12.0, z + 2.0], LINE, color);
    }
    event.draw.keep();
    true
}
