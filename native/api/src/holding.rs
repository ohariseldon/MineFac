//! Holding: a companhia do jogador vista como um grupo de empresas (divisões).
//!
//! Cada bloco do Veio pertence a uma divisão: mineração, manufatura, energia, logística, agro,
//! extrativismo (madeira e folhas), urbanização (casas e saneamento), comércio (lojas) e gestão e
//! segurança. A prefeitura é pública e fica de fora.
//!
//! Contas por empresa: a **receita** é o atacado que as lojas pagam pelos itens daquela empresa
//! (`item_division`); os **gastos** são os salários dos trabalhadores dos postos dela e o imposto
//! retido na fonte. Lucro = receita - gastos, em média do dia.
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
pub(super) const TABS: i32 = 11;
pub(super) const MINING: u8 = 1;
pub(super) const MANUFACTURING: u8 = 2;
pub(super) const ENERGY: u8 = 3;
pub(super) const LOGISTICS: u8 = 4;
pub(super) const AGRO: u8 = 5;
pub(super) const FORESTRY: u8 = 6;
pub(super) const URBAN: u8 = 7;
pub(super) const COMMERCE: u8 = 8;
pub(super) const MANAGEMENT: u8 = 9;
/// Aba do comércio exterior (não é uma empresa: reputação, fluxos e contratos).
pub(super) const FOREIGN: i32 = 10;
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
        workforce::FOREMAN_DESK | workforce::society::trade::PORT | workforce::society::trade::STATION => LOGISTICS,
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
    /// Receita e gastos do turno e do dia, por divisão.
    revenue_turn: [i32; TABS as usize],
    cost_turn: [i32; TABS as usize],
    revenue_day: [i32; TABS as usize],
    cost_day: [i32; TABS as usize],
    /// Exportações do turno e do dia.
    export_turn: i32,
    export_day: i32,
    rolled: i64,
}

/// Uma venda ao mercado externo (a receita já foi lançada na empresa do item).
pub(super) fn exported(dimension: &str, coins: i32) {
    with_world(dimension, |world| world.export_turn += coins.max(0));
}

/// Exportações do dia nesta dimensão.
pub(super) fn exports_day(dimension: &str) -> i32 {
    with_world(dimension, |world| world.export_day).unwrap_or(0)
}

/// A empresa que produz um item (quem recebe o atacado dele).
pub(super) fn item_division(item: &str) -> u8 {
    let name = item.split(':').nth(1).unwrap_or(item);
    if name.ends_with("_log") || name.ends_with("_wood") || name.ends_with("_planks") || name.ends_with("_sapling")
        || name.ends_with("_leaves") || name == "stick" || name == "charcoal"
    {
        FORESTRY
    } else if name.starts_with("raw_") || name.ends_with("_ore") || matches!(name, "coal" | "cobblestone" | "stone"
        | "gravel" | "sand" | "flint" | "quartz" | "diamond" | "redstone" | "lapis_lazuli" | "clay_ball")
    {
        MINING
    } else if production::pantry::value(item).is_some() || name.ends_with("_seeds") || name.ends_with("_wool")
        || matches!(name, "wheat" | "leather" | "feather" | "egg" | "honey_bottle" | "sugar_cane" | "hay_block" | "milk_bucket")
        || item == "veio:flour" || item == "veio:biomass"
    {
        AGRO
    } else {
        MANUFACTURING
    }
}

/// Receita (bruta) e gastos de uma empresa neste turno.
pub(super) fn book(dimension: &str, division: u8, revenue: i32, cost: i32) {
    let division = (division as usize).min(TABS as usize - 1);
    with_world(dimension, |world| {
        world.revenue_turn[division] += revenue;
        world.cost_turn[division] += cost;
    });
}

/// A empresa do bloco em `pos` (pelo último registro), ou 0.
pub(super) fn division_at(dimension: &str, pos: [i32; 3]) -> u8 {
    with_world(dimension, |world| world.units.get(&pos).map_or(0, |unit| unit.division)).unwrap_or(0)
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
        let daily = workforce::society::ledger::daily;
        world.export_day = daily(world.export_day, world.export_turn);
        world.export_turn = 0;
        for division in 0..TABS as usize {
            world.day[division] = daily(world.day[division], world.turn[division]);
            world.revenue_day[division] = daily(world.revenue_day[division], world.revenue_turn[division]);
            world.cost_day[division] = daily(world.cost_day[division], world.cost_turn[division]);
            world.turn[division] = 0;
            world.revenue_turn[division] = 0;
            world.cost_turn[division] = 0;
        }
    });
}

/// Ativo: trabalhando, andando para a tarefa, zona ativa, guardando, casa habitada, carregado.
pub(super) fn active(status: i32) -> bool {
    matches!(status, 1 | 8 | workforce::WALKING | workforce::ZONE_ACTIVE | workforce::STORING
        | workforce::society::HOUSED)
}

/// Os números de uma divisão: [unidades, ativas, trabalhadores, produção/dia, estado parado mais
/// comum, quantas nele, receita/dia, gastos/dia].
pub(super) fn summary(dimension: &str, division: u8) -> [i32; 8] {
    let now = defense::clock();
    with_world(dimension, |world| {
        let mut out = [0; 8];
        out[6] = world.revenue_day[division as usize];
        out[7] = world.cost_day[division as usize];
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
pub(super) fn refresh(dimension: &str, pos: [i32; 3], entity: &mut ModBlockEntity, time: i64) {
    let tab = number(entity, "Tab").clamp(0, TABS - 1);
    if tab == 0 {
        set_number(entity, "DExport", exports_day(dimension));
        return;
    }
    if tab == FOREIGN {
        // [reputação, camada, exporta/dia, importa/dia] e três contratos em dois números cada:
        // produto x 1000 + dias, pedido x 10000 + feito.
        let day = time.div_euclid(workforce::society::DAY);
        let ([reputation, tier, exports, imports], contracts) = workforce::society::trade::summary(dimension, day);
        let mut values = vec![reputation, tier, exports, imports];
        for slot in 0..3 {
            match contracts.get(slot) {
                Some(&[product, done, need, days]) => values.extend([product * 1000 + days.min(999), need * 10000 + done.min(9999)]),
                None => values.extend([-1, 0]),
            }
        }
        entity.nbt.insert("HData".into(), Tag::IntArray(values));
        return;
    }
    let [units, active, workers, made, status, count, revenue, cost] = summary(dimension, tab as u8);
    let [a, b, c] = extras(dimension, pos, tab as u8);
    entity.nbt.insert("HData".into(), Tag::IntArray(vec![units, active, workers, made, status, count, a, b, c, revenue, cost]));
}

/// Valores do painel numa aba de divisão: [unidades, ativas, trabalhadores, produção, estado,
/// quantos, extra1, extra2, extra3, receita/dia, gastos/dia].
pub(super) fn data(entity: &ModBlockEntity) -> [i32; 11] {
    let mut out = [0; 11];
    if let Some(Tag::IntArray(values)) = entity.nbt.get(nbt_key("HData")) {
        for (slot, value) in out.iter_mut().zip(values) {
            *slot = *value;
        }
    }
    out
}

/// Clique nas setas do cabeçalho ou, na visão geral, na linha da companhia (saque).
pub(super) fn click(at: [f32; 2], tab: i32) -> Option<i32> {
    let row = (20.0..32.0).contains(&at[1]);
    if row && (5.0..20.0).contains(&at[0]) {
        Some(PREVIOUS)
    } else if row && (162.0..180.0).contains(&at[0]) {
        Some(NEXT)
    } else if tab == 0 && (64.0..76.0).contains(&at[1]) && (5.0..160.0).contains(&at[0]) {
        Some(workforce::society::ledger::CASH_OUT)
    } else {
        None
    }
}

/// O botão chegou ao servidor: troca a aba e recalcula na hora, ou saca uma esmeralda.
pub(super) fn press(dimension: &str, pos: [i32; 3], entity: &mut ModBlockEntity, id: i32, time: i64) {
    if id == workforce::society::ledger::CASH_OUT {
        workforce::society::ledger::cash_out(dimension, pos, entity, defense::clock());
        return;
    }
    let step = match id { PREVIOUS => TABS - 1, NEXT => 1, _ => return };
    set_number(entity, "Tab", (number(entity, "Tab") + step).rem_euclid(TABS));
    refresh(dimension, pos, entity, time);
}

/// Uma linha por divisão no relatório do centro de controle.
pub(super) fn report_lines(dimension: &str) -> Vec<Text> {
    (1..FOREIGN as u8).filter_map(|division| {
        let [units, active, workers, made, _, _, revenue, cost] = summary(dimension, division);
        (units > 0 || revenue > 0).then(|| Text::tr("veio.holding.report", &[&Text::tr(&format!("veio.holding.tab{division}"), &[]).plain(),
            &units, &active, &workers, &made, &revenue, &cost, &(revenue - cost)]))
    }).collect()
}
