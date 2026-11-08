//! Holding: a companhia do jogador vista como um grupo de empresas (divisões).
//!
//! Cada bloco do Veio pertence a uma divisão: mineração, manufatura, energia, logística, agro,
//! extrativismo (madeira e folhas), urbanização (casas e saneamento), comércio (lojas) e gestão e
//! segurança. A prefeitura é pública e fica de fora.
//!
//! Custo: cada bloco se registra uma vez a cada `SAMPLE` ticks (fase espalhada pela posição), com
//! estado, se tem trabalhador e o aumento do seu contador de produção. A produção vira média do dia
//! por divisão no turno da sociedade. O centro de controle soma a aba escolhida a cada
//! `REFRESH` ciclos, nunca por tick.
use super::*;
use horizon_client::mod_api::Text;
use std::collections::HashMap;
use std::sync::Mutex;

/// Abas do centro de controle: 0 é a visão geral; 1 a 9, as divisões.
pub(super) const TABS: i32 = 10;
pub(super) const MINING: u8 = 1;
pub(super) const MANUFACTURING: u8 = 2;
pub(super) const ENERGY: u8 = 3;
pub(super) const LOGISTICS: u8 = 4;
pub(super) const AGRO: u8 = 5;
pub(super) const FORESTRY: u8 = 6;
pub(super) const URBAN: u8 = 7;
pub(super) const COMMERCE: u8 = 8;
pub(super) const MANAGEMENT: u8 = 9;
const SAMPLE: i64 = 100;
/// Botões das setas da aba (enviados pelo clique na tela, recebidos no servidor).
pub(super) const PREVIOUS: i32 = 100;
pub(super) const NEXT: i32 = 101;

/// A divisão de um bloco, ou `None` (prefeitura, blocos sem divisão).
pub(super) fn division(block: &str) -> Option<u8> {
    use operations::Sector;
    Some(match block {
        production::TREE_FARM => FORESTRY,
        workforce::society::HOUSE | workforce::society::WELL => URBAN,
        workforce::SHOP => COMMERCE,
        workforce::TOWN_HALL => return None,
        workforce::FOREMAN_DESK => LOGISTICS,
        _ => match operations::profile(block)?.sector {
            Sector::Extraction => MINING,
            Sector::Industry | Sector::Projects => MANUFACTURING,
            Sector::Power => ENERGY,
            Sector::Logistics => LOGISTICS,
            Sector::Farming | Sector::Food => AGRO,
            Sector::Defense | Sector::Workforce => MANAGEMENT,
        },
    })
}

#[derive(Clone, Copy, Debug, Default)]
struct Unit {
    division: u8,
    status: i32,
    staffed: bool,
    counter: i32,
    seen: u64,
}

#[derive(Default)]
struct World {
    units: HashMap<[i32; 3], Unit, FastHash>,
    /// Produção deste turno e média do dia, por divisão.
    turn: [i32; TABS as usize],
    day: [i32; TABS as usize],
    rolled: i64,
}

static WORLDS: Mutex<Vec<(String, World)>> = Mutex::new(Vec::new());
const STALE: u64 = 1200;

fn with_world<T>(dimension: &str, change: impl FnOnce(&mut World) -> T) -> Option<T> {
    let mut worlds = WORLDS.lock().ok()?;
    let index = match worlds.iter().position(|(name, _)| name == dimension) {
        Some(index) => index,
        None => {
            worlds.push((dimension.to_owned(), World::default()));
            worlds.len() - 1
        }
    };
    Some(change(&mut worlds[index].1))
}

/// Chamado no tick de cada bloco do Veio. Quase sempre sai na primeira comparação.
pub(super) fn observe(dimension: &str, pos: [i32; 3], entity: &ModBlockEntity, time: i64) {
    if (time + i64::from((pos[0] * 7 ^ pos[2] * 13) & 127)).rem_euclid(SAMPLE) != 0 {
        return;
    }
    let Some(division) = division(&entity.id) else { return };
    // Casas guardam a carteira em `Total` e lojas as vendas do dia: o que elas fazem vem dos
    // registros da sociedade, não do contador.
    let counter = if matches!(division, URBAN | COMMERCE) { 0 } else { number(entity, "Produced").saturating_add(number(entity, "Total")) };
    let unit = Unit {
        division, status: number(entity, "Status"), staffed: workforce::worker(entity).is_some(), counter,
        seen: defense::clock(),
    };
    with_world(dimension, |world| {
        let previous = world.units.insert(pos, unit);
        if let Some(previous) = previous
            && previous.division == division
        {
            world.turn[division as usize] += (counter - previous.counter).max(0);
        }
    });
}

/// Fecha o turno: a produção do turno entra na média do dia. Uma vez por turno, mesmo com vários
/// centros de controle.
pub(super) fn roll(dimension: &str, time: i64) {
    let turn = time.div_euclid(workforce::society::TURN);
    with_world(dimension, |world| {
        if world.rolled == turn {
            return;
        }
        world.rolled = turn;
        for division in 0..TABS as usize {
            world.day[division] = workforce::society::ledger::daily(world.day[division], world.turn[division]);
            world.turn[division] = 0;
        }
    });
}

/// Ativo: trabalhando, andando para a tarefa, zona ativa, guardando, casa habitada, carregado.
pub(super) fn active(status: i32) -> bool {
    matches!(status, 1 | 8 | workforce::WALKING | workforce::ZONE_ACTIVE | workforce::STORING
        | workforce::society::HOUSED)
}

/// Os números de uma divisão: [unidades, ativas, trabalhadores, produção/dia, estado parado mais
/// comum, quantas nele].
pub(super) fn summary(dimension: &str, division: u8) -> [i32; 6] {
    let now = defense::clock();
    with_world(dimension, |world| {
        let mut out = [0; 6];
        let mut stopped: Vec<(i32, i32)> = Vec::new();
        for unit in world.units.values().filter(|unit| unit.division == division && now.saturating_sub(unit.seen) <= STALE) {
            out[0] += 1;
            out[1] += i32::from(active(unit.status));
            out[2] += i32::from(unit.staffed);
            if !active(unit.status) && unit.status != 0 {
                match stopped.iter_mut().find(|(status, _)| *status == unit.status) {
                    Some(entry) => entry.1 += 1,
                    None => stopped.push((unit.status, 1)),
                }
            }
        }
        out[3] = world.day[division as usize];
        if let Some((status, count)) = stopped.into_iter().max_by_key(|(_, count)| *count) {
            out[4] = status;
            out[5] = count;
        }
        out
    }).unwrap_or_default()
}

/// Números extras de algumas divisões, dos registros da sociedade e das zonas.
fn extras(dimension: &str, pos: [i32; 3], division: u8) -> [i32; 3] {
    let now = defense::clock();
    const ANYWHERE: i32 = i32::MAX / 4;
    match division {
        URBAN => {
            let homes = workforce::society::homes_near(dimension, pos, ANYWHERE, now);
            [homes.len() as i32, homes.iter().map(|home| home.residents).sum(), homes.iter().filter(|home| !home.licensed).count() as i32]
        }
        COMMERCE => {
            let shops = workforce::society::shops_near(dimension, pos, ANYWHERE, now);
            let open = shops.iter().filter(|shop| shop.open).count() as i32;
            let index = if shops.is_empty() { 100 } else { shops.iter().map(|shop| shop.prices).sum::<i32>() / shops.len() as i32 };
            [shops.len() as i32, open, index]
        }
        AGRO => {
            let zones = workforce::zones::zones_near(dimension, pos, ANYWHERE, now);
            let fields = zones.iter().filter(|zone| zone.kind == workforce::zones::Kind::Field).count() as i32;
            [fields, zones.len() as i32 - fields, zones.iter().map(|zone| zone.level).sum()]
        }
        _ => [0; 3],
    }
}

/// Recalcula a aba escolhida no centro de controle (`HData`).
pub(super) fn refresh(dimension: &str, pos: [i32; 3], entity: &mut ModBlockEntity) {
    let tab = number(entity, "Tab").clamp(0, TABS - 1);
    if tab == 0 {
        return;
    }
    let [units, active, workers, made, status, count] = summary(dimension, tab as u8);
    let [a, b, c] = extras(dimension, pos, tab as u8);
    entity.nbt.insert("HData".into(), Tag::IntArray(vec![units, active, workers, made, status, count, a, b, c]));
}

/// Valores do painel numa aba de divisão: [unidades, ativas, trabalhadores, produção, estado,
/// quantos, extra1, extra2, extra3].
pub(super) fn data(entity: &ModBlockEntity) -> [i32; 9] {
    let mut out = [0; 9];
    if let Some(Tag::IntArray(values)) = entity.nbt.get(nbt_key("HData")) {
        for (slot, value) in out.iter_mut().zip(values) {
            *slot = *value;
        }
    }
    out
}

/// Clique nas setas do cabeçalho (coordenadas da tela).
pub(super) fn click(at: [f32; 2]) -> Option<i32> {
    let row = (20.0..32.0).contains(&at[1]);
    if row && (5.0..20.0).contains(&at[0]) {
        Some(PREVIOUS)
    } else if row && (162.0..180.0).contains(&at[0]) {
        Some(NEXT)
    } else {
        None
    }
}

/// O botão chegou ao servidor: troca a aba e recalcula na hora.
pub(super) fn press(dimension: &str, pos: [i32; 3], entity: &mut ModBlockEntity, id: i32) {
    // As setas passam pelas páginas da vila (aba 0) e depois pelas divisões.
    let step = match id { PREVIOUS => -1, NEXT => 1, _ => return };
    workforce::center::step_view(entity, step);
    refresh(dimension, pos, entity);
}

/// Uma linha por divisão no relatório do centro de controle.
pub(super) fn report_lines(dimension: &str) -> Vec<Text> {
    (1..TABS as u8).filter_map(|division| {
        let [units, active, workers, made, _, _] = summary(dimension, division);
        (units > 0).then(|| Text::tr("veio.holding.report", &[&Text::tr(&format!("veio.holding.tab{division}"), &[]).plain(),
            &units, &active, &workers, &made]))
    }).collect()
}
