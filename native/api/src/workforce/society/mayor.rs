//! Prefeitura: contas públicas, impostos, indicadores, fases do plano diretor e relatório.
//!
//! A cada turno o prefeito:
//! 1. cobra o **imposto** da fase sobre a produção e as vendas do turno na sua área, da conta da
//!    companhia mais próxima; o que faltar vira **dívida** (cobrada antes, no turno seguinte);
//! 2. paga a **manutenção** dos poços da área (sem dinheiro, os poços deixam de contar);
//! 3. mede a vila (população, emprego, PIB, IDH, Gini, bairros, custo de vida) e decide a fase;
//! 4. aponta o pior problema.
//! O salário do prefeito e a assistência às famílias saem da conta pública no turno das casas.
use super::*;
use std::collections::HashSet;

/// Imposto da fase sobre produção e vendas, em %.
pub(crate) fn tax_rate(phase: i32) -> i32 {
    match phase { 1 => 0, 2 => 8, 3 => 12, _ => 16 }
}

/// Verba inicial da prefeitura nova (o "estado" funda a cidade): paga o prefeito antes dos impostos.
pub(crate) const GRANT: i32 = 300;
/// Manutenção de cada poço por turno, em meias moedas (1 moeda a cada 2 turnos).
const WELL_UPKEEP_HALVES: i32 = 1;

/// Exigências para chegar a cada fase: (moradores, famílias, IDH, PIB/dia, emprego %, Gini máx., bairros).
pub(crate) const PHASES: [(i32, i32, i32, i32, i32, i32, i32); 3] = [
    (12, 3, 40, 0, 0, 100, 0),
    (36, 9, 55, 400, 60, 100, 0),
    (80, 20, 70, 0, 0, 45, 3),
];

/// Indicadores de uma área (prefeitura ou bairro).
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct Indicators {
    pub population: i32,
    pub adults: i32,
    pub kids: i32,
    pub families: i32,
    pub homeless: i32,
    pub employed: i32,
    pub employment: i32,
    pub idh: i32,
    pub gini: i32,
    pub hungry: i32,
    pub no_sanitation: i32,
    pub no_power: i32,
    pub no_commerce: i32,
    pub crowded: i32,
    pub smog: i32,
    pub unlicensed: i32,
    pub districts: i32,
    pub company_owed: i32,
}

/// Gini (centésimos) da renda por pessoa, cada grupo com seu peso: (renda por pessoa, pessoas).
pub(crate) fn gini(groups: &[(f64, f64)]) -> i32 {
    let total: f64 = groups.iter().map(|(_, people)| people).sum();
    let mean = groups.iter().map(|(income, people)| income * people).sum::<f64>() / total.max(1.0);
    if total <= 0.0 || mean <= 0.0 {
        return 0;
    }
    // Ordenado, a soma das diferenças sai em O(n log n) em vez de O(n²).
    let mut sorted = groups.to_vec();
    sorted.sort_by(|a, b| a.0.total_cmp(&b.0));
    let (mut below_people, mut below_income, mut sum) = (0.0, 0.0, 0.0);
    for (income, people) in &sorted {
        sum += people * (income * below_people - below_income);
        below_people += people;
        below_income += income * people;
    }
    (sum / (total * total * mean) * 100.0).round() as i32
}

/// Bairros: grupos de casas a até `DISTRICT_RADIUS` blocos (cada casa numa componente).
pub(crate) fn districts(homes: &[Home]) -> Vec<Vec<usize>> {
    let mut group = vec![usize::MAX; homes.len()];
    let mut out = Vec::new();
    for start in 0..homes.len() {
        if group[start] != usize::MAX {
            continue;
        }
        let id = out.len();
        let mut members = vec![start];
        group[start] = id;
        let mut index = 0;
        while index < members.len() {
            let at = homes[members[index]].pos;
            for other in 0..homes.len() {
                if group[other] == usize::MAX && within(homes[other].pos, at, DISTRICT_RADIUS) {
                    group[other] = id;
                    members.push(other);
                }
            }
            index += 1;
        }
        out.push(members);
    }
    out
}

pub(crate) fn measure(homes: &[Home], homeless: i32) -> Indicators {
    let mut out = Indicators { homeless, ..Indicators::default() };
    let mut weighted = 0.0;
    let mut groups = Vec::new();
    let lived: Vec<Home> = homes.iter().filter(|home| home.residents > 0).cloned().collect();
    for home in &lived {
        out.population += home.residents;
        out.adults += home.adults;
        out.kids += home.kids;
        out.families += 1;
        out.employed += home.employed;
        weighted += f64::from(home.idh()) * f64::from(home.residents);
        groups.push((f64::from(home.income) / f64::from(home.residents), f64::from(home.residents)));
        out.hungry += i32::from(home.hungry);
        out.no_sanitation += i32::from(!home.sanitation);
        out.no_power += i32::from(!home.power);
        out.no_commerce += i32::from(!home.commerce);
        out.crowded += i32::from(home.crowded);
        out.smog += i32::from(home.smog);
        out.company_owed += i32::from(home.owed);
    }
    out.unlicensed = homes.iter().filter(|home| !home.licensed).count() as i32;
    out.districts = districts(&lived).len() as i32;
    // Sem-teto: IDH mínimo (5 em cada dimensão) e renda zero.
    let people = out.population + homeless;
    weighted += 5.0 * f64::from(homeless);
    if homeless > 0 {
        groups.push((0.0, f64::from(homeless)));
    }
    out.population = people;
    out.idh = if people == 0 { 0 } else { (weighted / f64::from(people)).round() as i32 };
    out.gini = gini(&groups);
    out.employment = if out.adults == 0 { 0 } else { out.employed * 100 / out.adults };
    out
}

/// A próxima fase que estes números alcançam (a partir da atual, uma por vez).
pub(crate) fn next_phase(phase: i32, numbers: &Indicators, gdp: i32) -> i32 {
    let Some((residents, families, idh, need_gdp, employment, gini, districts)) = PHASES.get((phase - 1) as usize).copied() else {
        return phase;
    };
    let ok = numbers.population - numbers.homeless >= residents && numbers.families >= families && numbers.idh >= idh
        && gdp >= need_gdp && numbers.employment >= employment && numbers.gini <= gini && numbers.districts >= districts;
    if ok { phase + 1 } else { phase }
}

/// Pedido do prefeito: (código, número), o pior problema primeiro. 0 = tudo certo.
/// 12 dívida de impostos, 1 salários da companhia atrasados, 11 cofre público vazio, 2 fome,
/// 3 sem-teto, 4 saneamento, 5 comércio, 6 energia, 7 desemprego, 8 poluição, 9 lotação, 10 meta.
pub(crate) fn request(numbers: &Indicators, debt: i32, broke: bool, phase: i32) -> (i32, i32) {
    let unemployed = numbers.adults - numbers.employed;
    let issues = [
        (12, debt), (1, numbers.company_owed), (11, i32::from(broke)), (2, numbers.hungry), (3, numbers.homeless),
        (4, numbers.no_sanitation), (5, numbers.no_commerce), (6, if phase >= 2 { numbers.no_power } else { 0 }),
        (7, unemployed), (8, numbers.smog), (9, numbers.crowded),
    ];
    if let Some(issue) = issues.into_iter().find(|(_, value)| *value > 0) {
        return issue;
    }
    if phase < 4 { (10, phase + 1) } else { (0, 0) }
}

fn homeless_near(host: &dyn Host, dimension: &str, pos: [i32; 3], now: u64) -> i32 {
    let housed: HashSet<u32> = housed(dimension, now).into_iter().collect();
    let at = center(pos);
    let radius = f64::from(HALL_RADIUS);
    production::ranch::villager_positions(host, dimension).iter()
        .filter(|(id, spot)| (spot[0] - at[0]).abs() <= radius && (spot[2] - at[2]).abs() <= radius && !housed.contains(id))
        .count() as i32
}

/// Ciclo da prefeitura (a cada ciclo do posto). Contas, medição e fase só uma vez por turno.
pub(crate) fn hall_cycle(host: &mut dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity, time: i64) {
    let now = defense::clock();
    let dimension = host.dimension().to_owned();
    // O governo segue de noite e enquanto o prefeito anda; para sem prefeito, ausente ou com fome.
    let staffed = worker(entity).is_some() && !matches!(number(entity, "Status"), NO_WORKER | ABSENT | NO_FOOD);
    let mayor = worker(entity).and_then(cached_entity);
    // Conta pública: a memória manda; ao reabrir o mundo, volta do NBT. A primeira vez traz a verba.
    if !entity.nbt.contains_key(nbt_key("Public")) {
        set_number(entity, "Public", GRANT);
    }
    let mut coins = ledger::balance(true, &dimension, pos).unwrap_or_else(|| number(entity, "Public"));
    coins = coins.saturating_add(ledger::take_emeralds(entity, 1));
    let close = ledger::turn_edge(pos, time) || !entity.nbt.contains_key(nbt_key("DPop"));
    let flow = ledger::touch(true, &dimension, pos, coins, now, close);
    let phase = number(entity, "Phase").max(1);
    if close {
        // 1. Imposto sobre o turno, mais a dívida.
        let rate = tax_rate(phase);
        let due = (flow[ledger::MADE] + flow[ledger::SOLD]) * rate / 100 + number(entity, "Debt");
        let paid = ledger::pay(false, &dimension, pos, i32::MAX / 4, due, ledger::TAXES, now);
        coins += paid;
        set_number(entity, "Debt", due - paid);
        // 2. Manutenção dos poços da área.
        let wells = wells_near(&dimension, pos, HALL_RADIUS, now);
        let upkeep = (wells * WELL_UPKEEP_HALVES + number(entity, "UpkeepHalf")) / 2;
        set_number(entity, "UpkeepHalf", (wells * WELL_UPKEEP_HALVES + number(entity, "UpkeepHalf")) % 2);
        let maintained = coins >= upkeep;
        coins -= upkeep.min(coins);
        set_number(entity, "Maintained", i32::from(maintained));
        // Médias diárias das contas.
        for (key, value) in [("DMade", flow[ledger::MADE]), ("DSold", flow[ledger::SOLD]), ("DTax", paid),
            ("DWages", flow[ledger::WAGES]), ("DUpkeep", upkeep), ("DAssist", flow[ledger::ASSIST])] {
            set_number(entity, key, ledger::daily(number(entity, key), value));
        }
        ledger::touch(true, &dimension, pos, coins, now, false);
        // 3. Medição e fase.
        let homes = homes_near(&dimension, pos, HALL_RADIUS, now);
        let homeless = homeless_near(host, &dimension, pos, now);
        let numbers = measure(&homes, homeless);
        let gdp = number(entity, "DMade") + number(entity, "DSold");
        let promoted = if staffed { next_phase(phase, &numbers, gdp) } else { phase };
        if promoted > phase {
            set_number(entity, "Phase", promoted);
            notify(host, pos, &format!("veio.mayor.phase{promoted}"));
            host.play_sound("minecraft:ui.toast.challenge_complete", center(pos), 4, 0.8, 1.0);
        }
        let phase = promoted;
        // Licença suspensa: IDH bem abaixo do que a fase exigiu, ou dívida maior que um dia de imposto.
        let required = if phase >= 2 { PHASES[(phase - 2) as usize].2 } else { 0 };
        let debt = number(entity, "Debt");
        let suspended = (numbers.population > 0 && numbers.idh + 5 < required)
            || debt > number(entity, "DTax").max(COINS_PER_EMERALD);
        let (code, value) = request(&numbers, debt, coins < WAGE_PER_TURN, phase);
        for (key, number) in [
            ("DPop", numbers.population), ("DFamilies", numbers.families), ("DEmploy", numbers.employment),
            ("DIdh", numbers.idh), ("DGini", numbers.gini), ("DRequest", code), ("DRequestN", value),
            ("DSuspended", i32::from(suspended)),
        ] {
            set_number(entity, key, number.max(0));
        }
    }
    set_number(entity, "Public", coins);
    let phase = number(entity, "Phase").max(1);
    upsert(&HALLS, Hall {
        dimension: dimension.clone(), pos, phase, suspended: number(entity, "DSuspended") != 0, staffed, mayor,
        maintained: number(entity, "Maintained") != 0 || !entity.nbt.contains_key(nbt_key("Maintained")),
        seen: now,
    }, |known| known.dimension == dimension && known.pos == pos, |known| now.saturating_sub(known.seen) <= STALE);
}

/// Os 13 valores do painel da prefeitura.
pub(crate) fn hall_data(entity: &ModBlockEntity) -> [i32; 13] {
    let phase = number(entity, "Phase").max(1);
    [number(entity, "Status"), phase, number(entity, "DPop"), number(entity, "DFamilies"), number(entity, "DEmploy"),
        number(entity, "DIdh"), number(entity, "DGini"), number(entity, "DMade") + number(entity, "DSold"),
        number(entity, "Public"), tax_rate(phase), number(entity, "Debt"),
        number(entity, "DRequest") * 1000 + number(entity, "DRequestN").min(999), number(entity, "DSuspended")]
}

pub(crate) fn decimal(hundredths: i32) -> String {
    format!("{},{:02}", hundredths / 100, hundredths % 100)
}

/// Tela da prefeitura: fase, indicadores, contas públicas e o pedido.
pub(crate) fn draw_screen(host: &dyn Host, event: &mut horizon_client::mod_api::ScreenDrawEvent<'_>) {
    let [x, y] = event.origin;
    let read = |index: usize| event.data.get(index).copied().unwrap_or(0);
    let tr = |key: &str, args: &[&dyn std::fmt::Display]| Text::tr(key, args).plain();
    let status = read(0).clamp(0, presentation::STATUS_KEYS.len() as i32 - 1) as usize;
    let request = read(11);
    let suspended = read(12) != 0;
    let lines = [
        (tr(&format!("veio.status.{}", presentation::STATUS_KEYS[status]), &[]), presentation::text_color(status)),
        (tr(&format!("veio.mayor.phase_name{}", read(1).clamp(1, 4)), &[]), 0x1f4fa8),
        (tr("veio.mayor.population", &[&read(2), &read(3), &read(4)]), presentation::INK),
        (tr("veio.mayor.idh", &[&decimal(read(5)), &decimal(read(6))]), if read(5) < 50 { 0x9a5a00 } else { 0x1e7a1e }),
        (tr("veio.mayor.gdp", &[&read(7), &read(9)]), presentation::INK),
        (tr("veio.mayor.public", &[&read(8), &read(10)]), if read(10) > 0 { 0xaa1f1f } else { presentation::INK }),
        (if suspended { tr("veio.mayor.suspended", &[]) } else { tr(&format!("veio.mayor.request{}", request / 1000), &[&(request % 1000)]) },
            if suspended || !matches!(request / 1000, 0 | 10) { 0xaa1f1f } else { 0x1e7a1e }),
    ];
    for (index, (line, color)) in lines.iter().enumerate() {
        host.text(event.batch, line, [x + 8.0, y + 22.0 + index as f32 * 11.0], *color, false);
    }
}

/// Relatório no chat (clique com livro na prefeitura).
pub(crate) fn report_to_chat(host: &mut dyn Host, pos: [i32; 3]) -> Option<Flow> {
    let mut state = None;
    host.change_mod_block_entity(pos, &mut |entity| {
        if entity.id == TOWN_HALL {
            state = Some((worker(entity).is_some(), number(entity, "Phase").max(1),
                [number(entity, "Public"), number(entity, "DTax"), number(entity, "DWages"), number(entity, "DUpkeep"),
                    number(entity, "DAssist"), number(entity, "Debt")],
                [number(entity, "DMade"), number(entity, "DSold")]));
        }
    });
    let (staffed, phase, public, [made, sold]) = state?;
    let at = center(pos);
    let dimension = host.dimension().to_owned();
    let Some(player) = host.server_players().into_iter()
        .filter(|player| player.dimension == dimension)
        .find(|player| (0..3).map(|axis| (player.position[axis] - at[axis]).powi(2)).sum::<f64>() < 144.0)
        .map(|player| player.uuid) else { return Some(Flow::Cancel) };
    if !staffed {
        host.tell(player, Text::tr("veio.mayor.need_mayor", &[]));
        return Some(Flow::Cancel);
    }
    let now = defense::clock();
    let homes = homes_near(&dimension, pos, HALL_RADIUS, now);
    let numbers = measure(&homes, homeless_near(host, &dimension, pos, now));
    host.tell(player, Text::tr(&format!("veio.mayor.phase_name{phase}"), &[]));
    host.tell(player, Text::tr("veio.mayor.report.people", &[&numbers.population, &numbers.adults, &numbers.kids,
        &numbers.families, &numbers.homeless]));
    host.tell(player, Text::tr("veio.mayor.report.economy", &[&(made + sold), &made, &sold, &numbers.employment,
        &cost_of_living(&dimension, pos, now)]));
    host.tell(player, Text::tr("veio.mayor.report.public", &[&public[0], &tax_rate(phase), &public[1], &public[2],
        &public[3], &public[4], &public[5]]));
    host.tell(player, Text::tr("veio.mayor.report.idh", &[&decimal(numbers.idh), &decimal(numbers.gini)]));
    // Bairros, cada um com sua população e IDH.
    let lived: Vec<Home> = homes.iter().filter(|home| home.residents > 0).cloned().collect();
    for (index, members) in districts(&lived).iter().enumerate().take(6) {
        let group: Vec<Home> = members.iter().map(|member| lived[*member].clone()).collect();
        let part = measure(&group, 0);
        let [x, _, z] = group[0].pos;
        host.tell(player, Text::tr("veio.mayor.report.district", &[&(index + 1), &format!("{x} {z}"), &part.population,
            &decimal(part.idh)]));
    }
    // Todos os problemas, não só o pior.
    let unemployed = numbers.adults - numbers.employed;
    for (code, value) in [(12, public[5]), (1, numbers.company_owed), (11, i32::from(public[0] < WAGE_PER_TURN)),
        (2, numbers.hungry), (3, numbers.homeless), (4, numbers.no_sanitation), (5, numbers.no_commerce),
        (6, if phase >= 2 { numbers.no_power } else { 0 }), (7, unemployed), (8, numbers.smog), (9, numbers.crowded)] {
        if value > 0 {
            host.tell(player, Text::tr(&format!("veio.mayor.request{code}"), &[&value]));
        }
    }
    if let Some((residents, families, idh, gdp, employment, gini, districts)) = PHASES.get((phase - 1) as usize).copied() {
        host.tell(player, Text::tr("veio.mayor.report.next", &[&(phase + 1), &residents, &families, &decimal(idh), &gdp,
            &employment, &decimal(gini), &districts]));
    }
    // Global: todas as prefeituras da dimensão.
    let halls: Vec<[i32; 3]> = HALLS.lock().map(|halls| halls.iter()
        .filter(|hall| hall.dimension == dimension && now.saturating_sub(hall.seen) <= STALE)
        .map(|hall| hall.pos).collect()).unwrap_or_default();
    if halls.len() > 1 {
        let total = measure(&homes_near(&dimension, pos, i32::MAX / 4, now), 0);
        host.tell(player, Text::tr("veio.mayor.report.global", &[&halls.len(), &total.population, &decimal(total.idh)]));
    }
    Some(Flow::Cancel)
}

/// Custo de vida da área: o índice de preços da comida das lojas, em %.
pub(crate) fn cost_of_living(dimension: &str, pos: [i32; 3], now: u64) -> i32 {
    let shops = shops_near(dimension, pos, HALL_RADIUS, now);
    if shops.is_empty() { 100 } else { shops.iter().map(|shop| shop.prices).sum::<i32>() / shops.len() as i32 }
}
