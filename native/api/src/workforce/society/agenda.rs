//! Agenda da vila: calendário, feriados, eventos de domingo e obras cívicas, trazidos ao prefeito
//! por visitantes. Desenho em `frentes/AGENDA.md`.
//!
//! **Calendário.** Usa as estações do clima (`climate`): o ano tem 4 estações de 6 dias. O 6º dia
//! de cada estação é **domingo**. Há 6 **feriados** fixos por ano (Dia da Fundação, Festa das
//! Flores, São João, Festa da Colheita, Festa das Luzes e Virada do Ano). No feriado a vila **para
//! de trabalhar** (os postos descansam, menos o quartel) e os aldeões se reúnem na praça (ou na
//! prefeitura). Nos domingos sem feriado há um evento menor (feira, baile ou torneio de pesca).
//!
//! **Visitantes e pedidos.** `LEAD` dias antes de um feriado ou domingo, chega à prefeitura um
//! visitante (`veio:visitor`, com nome) que representa o evento e pede ao prefeito o que precisa.
//! O prefeito vira o pedido uma missão para o jogador: entregar os materiais no espaço *Agenda* da
//! prefeitura. De tempos em tempos chega também um pedido de **obra cívica** (parque, praça ou
//! monumento) para um bairro sem lazer: a missão é construir o bloco cívico lá (a receita usa os
//! materiais). Clicar no visitante mostra a conversa no chat.
//!
//! **Efeitos.** Evento preparado: o **ânimo** da vila sobe no dia (feriado +20 a +25, domingo +8).
//! Feriado sem preparo: a vila para do mesmo jeito, mas o ânimo cai (-10). Obra cívica: lazer
//! permanente para as casas a até `AMENITY_RADIUS` blocos, +10 de ânimo. O ânimo (0 a 100, volta
//! devagar para 50) multiplica o IDH das famílias de 0,9x a 1,1x; lazer entra nos serviços do IDH.
//! A prefeitura reembolsa à companhia o valor do que foi entregue (eventos) ou paga a obra
//! (cívica), com a conta pública: o imposto volta como gasto público, sem criar moeda.
//!
//! **Custo.** Tudo roda no ciclo do prefeito; o trabalho pesado só na virada do dia e no turno.
use super::*;
use ledger::Book;

pub(crate) const PARK: &str = "veio:park";
pub(crate) const PLAZA: &str = "veio:plaza";
pub(crate) const MONUMENT: &str = "veio:monument";
pub(crate) const VISITOR: &str = "veio:visitor";
/// Espaço da prefeitura onde o jogador entrega os materiais da agenda.
pub(crate) const AGENDA_SLOT: usize = 2;
/// Casas a até esta distância de um parque, praça ou monumento têm lazer.
pub(crate) const AMENITY_RADIUS: i32 = 32;
/// Dias de antecedência com que o visitante chega.
pub(crate) const LEAD: i64 = 2;
/// Uma obra cívica é pedida a cada `CIVIC_EVERY` dias, se não houver outra aberta.
pub(crate) const CIVIC_EVERY: i64 = 4;
pub(crate) const CIVIC_DAYS: i64 = 6;
pub(crate) const MAX_PROPOSALS: usize = 3;
/// Estado do painel dos postos no feriado.
pub(crate) const HOLIDAY: i32 = 44;
/// Botões das abas da prefeitura.
pub(crate) const TAB_PREVIOUS: i32 = 110;
pub(crate) const TAB_NEXT: i32 = 111;
const NEUTRAL: i32 = 50;
const DAY: i64 = super::DAY;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Kind { Holiday, Weekend, Civic }

/// Um evento do catálogo: chave de texto, tipo, data (estação e dia, para feriados), visitante
/// (profissão para a roupa), materiais e ânimo; obra cívica: o bloco e o pagamento.
pub(crate) struct Event {
    pub key: &'static str,
    pub kind: Kind,
    pub season: usize,
    pub day: i64,
    pub profession: &'static str,
    pub needs: [(&'static str, u32); 3],
    pub cheer: i32,
    pub amenity: &'static str,
    pub fee: i32,
}

const NO_NEEDS: [(&str, u32); 3] = [("", 0), ("", 0), ("", 0)];

pub(crate) const EVENTS: [Event; 12] = [
    Event { key: "founding", kind: Kind::Holiday, season: climate::SPRING, day: 1, profession: "minecraft:cleric",
        needs: [("minecraft:cake", 2), ("minecraft:bread", 32), ("minecraft:white_banner", 4)], cheer: 25, amenity: "", fee: 0 },
    Event { key: "flowers", kind: Kind::Holiday, season: climate::SPRING, day: 4, profession: "minecraft:farmer",
        needs: [("minecraft:poppy", 16), ("minecraft:dandelion", 16), ("minecraft:honey_bottle", 4)], cheer: 20, amenity: "", fee: 0 },
    Event { key: "saint_john", kind: Kind::Holiday, season: climate::SUMMER, day: 3, profession: "minecraft:shepherd",
        needs: [("minecraft:baked_potato", 24), ("minecraft:campfire", 2), ("minecraft:cookie", 16)], cheer: 25, amenity: "", fee: 0 },
    Event { key: "harvest", kind: Kind::Holiday, season: climate::AUTUMN, day: 4, profession: "minecraft:farmer",
        needs: [("minecraft:wheat", 48), ("minecraft:pumpkin_pie", 8), ("minecraft:bread", 16)], cheer: 25, amenity: "", fee: 0 },
    Event { key: "lights", kind: Kind::Holiday, season: climate::WINTER, day: 3, profession: "minecraft:librarian",
        needs: [("minecraft:lantern", 16), ("minecraft:candle", 8), ("minecraft:glass", 16)], cheer: 20, amenity: "", fee: 0 },
    Event { key: "new_year", kind: Kind::Holiday, season: climate::WINTER, day: 6, profession: "minecraft:nitwit",
        needs: [("minecraft:firework_rocket", 16), ("minecraft:cake", 1), ("minecraft:cookie", 16)], cheer: 25, amenity: "", fee: 0 },
    Event { key: "fair", kind: Kind::Weekend, season: 0, day: 6, profession: "minecraft:leatherworker",
        needs: [("minecraft:white_wool", 16), ("minecraft:paper", 16), ("minecraft:bread", 16)], cheer: 8, amenity: "", fee: 0 },
    Event { key: "dance", kind: Kind::Weekend, season: 0, day: 6, profession: "minecraft:toolsmith",
        needs: [("minecraft:note_block", 2), ("minecraft:lantern", 4), ("minecraft:cookie", 16)], cheer: 8, amenity: "", fee: 0 },
    Event { key: "fishing", kind: Kind::Weekend, season: 0, day: 6, profession: "minecraft:fisherman",
        needs: [("minecraft:fishing_rod", 4), ("minecraft:cooked_cod", 16), ("minecraft:bread", 8)], cheer: 8, amenity: "", fee: 0 },
    Event { key: "park", kind: Kind::Civic, season: 0, day: 0, profession: "minecraft:farmer",
        needs: NO_NEEDS, cheer: 10, amenity: PARK, fee: 240 },
    Event { key: "plaza", kind: Kind::Civic, season: 0, day: 0, profession: "minecraft:mason",
        needs: NO_NEEDS, cheer: 10, amenity: PLAZA, fee: 420 },
    Event { key: "monument", kind: Kind::Civic, season: 0, day: 0, profession: "minecraft:mason",
        needs: NO_NEEDS, cheer: 12, amenity: MONUMENT, fee: 720 },
];

const WEEKENDS: [usize; 3] = [6, 7, 8];
const CIVICS: [usize; 3] = [9, 10, 11];

pub(crate) fn is_amenity(block: &str) -> bool {
    matches!(block, PARK | PLAZA | MONUMENT)
}

/// O que o espaço *Agenda* aceita: qualquer material de evento.
pub(crate) fn wanted(item: &str) -> bool {
    EVENTS.iter().any(|event| event.needs.iter().any(|(need, count)| *count > 0 && *need == item))
}

// ---------- Calendário ----------

/// Dia absoluto do mundo.
pub(crate) fn day_of(day_time: i64) -> i64 {
    day_time.div_euclid(DAY)
}

fn season_of(day: i64) -> usize {
    climate::season(day * DAY + 1)
}

fn season_day_of(day: i64) -> i64 {
    climate::season_day(day * DAY + 1)
}

fn year_of(day: i64) -> i64 {
    day.div_euclid(climate::SEASON_DAYS * 4)
}

/// O dia 6 de cada estação é domingo.
pub(crate) fn sunday(day: i64) -> bool {
    season_day_of(day) == climate::SEASON_DAYS
}

/// O evento do calendário neste dia: o feriado, ou o evento de domingo (que muda a cada semana).
pub(crate) fn scheduled(day: i64) -> Option<usize> {
    let (season, date) = (season_of(day), season_day_of(day));
    if let Some(index) = EVENTS.iter().position(|event| event.kind == Kind::Holiday && event.season == season && event.day == date) {
        return Some(index);
    }
    sunday(day).then(|| WEEKENDS[(year_of(day) * 4 + season as i64).rem_euclid(3) as usize])
}

/// Feriado neste dia (a vila para)?
pub(crate) fn is_holiday(day: i64) -> bool {
    scheduled(day).is_some_and(|event| EVENTS[event].kind == Kind::Holiday)
}

/// O próximo feriado a partir de `day`: (evento, dias até ele).
pub(crate) fn next_holiday(day: i64) -> (usize, i64) {
    (0..climate::SEASON_DAYS * 4).find_map(|ahead| {
        scheduled(day + ahead).filter(|event| EVENTS[*event].kind == Kind::Holiday).map(|event| (event, ahead))
    }).unwrap_or((0, 0))
}

// ---------- Pedidos ----------

/// Um pedido aberto na prefeitura.
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct Proposal {
    pub event: usize,
    pub due: i64,
    pub have: [u32; 3],
    /// Bairro da obra cívica.
    pub target: [i32; 3],
    pub prepared: bool,
}

impl Proposal {
    /// Primeiro material que falta: (índice, quanto tem, quanto pede).
    pub(crate) fn missing(&self) -> Option<(usize, u32, u32)> {
        EVENTS[self.event].needs.iter().enumerate()
            .find(|(index, (_, count))| self.have[*index] < *count)
            .map(|(index, (_, count))| (index, self.have[index], *count))
    }
}

pub(crate) fn proposals(entity: &ModBlockEntity) -> Vec<Proposal> {
    let Some(Tag::IntArray(values)) = entity.nbt.get(nbt_key("Agenda")) else { return Vec::new() };
    values.chunks_exact(9).filter(|chunk| (chunk[0] as usize) < EVENTS.len()).map(|chunk| Proposal {
        event: chunk[0] as usize,
        due: i64::from(chunk[1]),
        have: [chunk[2].max(0) as u32, chunk[3].max(0) as u32, chunk[4].max(0) as u32],
        target: [chunk[5], chunk[6], chunk[7]],
        prepared: chunk[8] & 1 != 0,
    }).collect()
}

fn set_proposals(entity: &mut ModBlockEntity, list: &[Proposal]) {
    let values: Vec<i32> = list.iter().flat_map(|proposal| [proposal.event as i32, proposal.due as i32,
        proposal.have[0] as i32, proposal.have[1] as i32, proposal.have[2] as i32,
        proposal.target[0], proposal.target[1], proposal.target[2], i32::from(proposal.prepared)]).collect();
    entity.nbt.insert("Agenda".into(), Tag::IntArray(values));
}

/// Leva o que está no espaço *Agenda* para os pedidos abertos (o mais próximo da data primeiro).
/// Devolve os pedidos que ficaram prontos agora. O que nenhum pedido usa fica no espaço.
pub(crate) fn deliver(entity: &mut ModBlockEntity) -> Vec<Proposal> {
    let Some(stack) = entity.items.get(AGENDA_SLOT).and_then(Option::as_ref)
        .filter(|stack| stack.extra.is_none() && stack.damage == 0).cloned() else { return Vec::new() };
    let mut list = proposals(entity);
    list.sort_by_key(|proposal| proposal.due);
    let mut left = stack.count;
    let mut ready = Vec::new();
    for proposal in list.iter_mut().filter(|proposal| !proposal.prepared) {
        for (index, (need, count)) in EVENTS[proposal.event].needs.iter().enumerate() {
            if *count == 0 || *need != stack.item.as_ref() || left == 0 {
                continue;
            }
            let moved = (count - proposal.have[index].min(*count)).min(left);
            proposal.have[index] += moved;
            left -= moved;
        }
        if EVENTS[proposal.event].kind != Kind::Civic && proposal.missing().is_none() {
            proposal.prepared = true;
            ready.push(proposal.clone());
        }
    }
    if left < stack.count {
        consume(&mut entity.items[AGENDA_SLOT], stack.count - left);
        set_proposals(entity, &list);
    }
    ready
}

/// Valor de referência do que um evento pediu (o reembolso da prefeitura).
pub(crate) fn value(event: usize) -> i32 {
    EVENTS[event].needs.iter().filter(|(_, count)| *count > 0)
        .map(|(item, count)| market::base_price(item).unwrap_or(2) * *count as i32).sum()
}

// ---------- Lazer (parques, praças, monumentos) ----------

static AMENITIES: Mutex<Vec<(String, [i32; 3], &'static str, u64)>> = Mutex::new(Vec::new());

/// Tick dos blocos cívicos: registram-se como lazer da vila.
pub(crate) fn tick_amenity(host: &mut dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity, time: i64) {
    if (time + i64::from((pos[0] ^ pos[2]) & 63)).rem_euclid(100) != 0 {
        return;
    }
    let dimension = host.dimension();
    let now = defense::clock();
    let kind: &'static str = match entity.id.as_str() { PARK => PARK, PLAZA => PLAZA, _ => MONUMENT };
    upsert(&AMENITIES, (dimension.to_owned(), pos, kind, now), |(dim, at, _, _)| dim == dimension && *at == pos,
        |(_, _, _, seen)| now.saturating_sub(*seen) <= STALE);
    let homes = homes_near(dimension, pos, AMENITY_RADIUS, now).len() as i32;
    set_number(entity, "Rate", homes);
    set_number(entity, "Status", ZONE_ACTIVE);
}

/// Há lazer (parque, praça ou monumento) a até `AMENITY_RADIUS` blocos de `at`?
pub(crate) fn leisure_near(dimension: &str, at: [i32; 3], now: u64) -> bool {
    AMENITIES.lock().is_ok_and(|list| list.iter().any(|(dim, pos, _, seen)| dim == dimension
        && now.saturating_sub(*seen) <= STALE && within(*pos, at, AMENITY_RADIUS)))
}

fn amenity_near(dimension: &str, at: [i32; 3], kind: &str, now: u64) -> Option<[i32; 3]> {
    AMENITIES.lock().ok()?.iter().find(|(dim, pos, known, seen)| dim == dimension && *known == kind
        && now.saturating_sub(*seen) <= STALE && within(*pos, at, AMENITY_RADIUS)).map(|(_, pos, _, _)| *pos)
}

/// Onde a vila se reúne no feriado: a praça mais perto da prefeitura, ou a própria prefeitura.
fn gathering_spot(dimension: &str, hall: [i32; 3], now: u64) -> [i32; 3] {
    AMENITIES.lock().ok().and_then(|list| list.iter()
        .filter(|(dim, pos, kind, seen)| dim == dimension && *kind == PLAZA && now.saturating_sub(*seen) <= STALE
            && within(*pos, hall, HALL_RADIUS))
        .min_by_key(|(_, pos, _, _)| (pos[0] - hall[0]).abs() + (pos[2] - hall[2]).abs())
        .map(|(_, pos, _, _)| *pos)).unwrap_or(hall)
}

// ---------- Visitantes ----------

/// Visitantes vivos: (dimensão, prefeitura, evento, id da entidade).
static VISITORS: Mutex<Vec<(String, [i32; 3], usize, u32)>> = Mutex::new(Vec::new());

fn visitor_of(dimension: &str, hall: [i32; 3], event: usize) -> Option<u32> {
    VISITORS.lock().ok()?.iter().find(|(dim, at, known, _)| dim == dimension && *at == hall && *known == event).map(|(_, _, _, id)| *id)
}

/// Chamado no tick de cada visitante: um visitante que nenhum pedido conhece (de antes de reabrir
/// o mundo, ou de um pedido já resolvido) sai do mundo.
pub(crate) fn visitor_seen(host: &mut dyn Host, dimension: &str, id: u32) {
    let known = VISITORS.lock().is_ok_and(|list| list.iter().any(|(dim, _, _, known)| dim == dimension && *known == id));
    if !known {
        host.remove_mob(dimension, id);
    }
}

fn send_away(host: &mut dyn Host, dimension: &str, hall: [i32; 3], event: usize) {
    if let Some(id) = visitor_of(dimension, hall, event) {
        host.remove_mob(dimension, id);
    }
    if let Ok(mut list) = VISITORS.lock() {
        list.retain(|(dim, at, known, _)| !(dim == dimension && *at == hall && *known == event));
    }
}

/// O visitante do pedido está na prefeitura? Se não, chega (ao lado da porta, parado).
fn ensure_visitor(host: &mut dyn Host, dimension: &str, hall: [i32; 3], event: usize, slot: usize) {
    if let Some(id) = visitor_of(dimension, hall, event)
        && host.entity(id).is_some()
    {
        return;
    }
    if let Ok(mut list) = VISITORS.lock() {
        list.retain(|(dim, at, known, _)| !(dim == dimension && *at == hall && *known == event));
    }
    let offset = [[2.5, 0.0, 0.5], [-1.5, 0.0, 0.5], [0.5, 0.0, 2.5]][slot % 3];
    let at = [f64::from(hall[0]) + offset[0], f64::from(hall[1]), f64::from(hall[2]) + offset[2]];
    let name = Text::tr(&format!("veio.agenda.{}.visitor", EVENTS[event].key), &[]).plain();
    let nbt = Compound::from([
        ("CustomName".into(), Tag::String(name.as_str().into())),
        ("CustomNameVisible".into(), Tag::Byte(1)),
        ("NoAI".into(), Tag::Byte(1)),
        ("Invulnerable".into(), Tag::Byte(1)),
        ("PersistenceRequired".into(), Tag::Byte(1)),
        ("Silent".into(), Tag::Byte(1)),
        ("VillagerData".into(), Tag::Compound(Compound::from([
            ("profession".into(), Tag::String(EVENTS[event].profession.into())),
            ("type".into(), Tag::String("minecraft:plains".into())),
            ("level".into(), Tag::Int(2)),
        ]))),
    ]);
    if let Some(id) = host.summon(dimension, VISITOR, at, nbt)
        && let Ok(mut list) = VISITORS.lock()
    {
        list.push((dimension.to_owned(), hall, event, id));
        host.send_particles(&ParticleOptions::simple(ParticleType::HappyVillager), None, [at[0], at[1] + 1.0, at[2]],
            [0.3, 0.5, 0.3], 0.0, 8);
    }
}

/// O jogador clicou num visitante: a conversa vai ao chat dele. `true` se era um visitante.
pub(crate) fn talk(host: &mut dyn Host, id: u32) -> bool {
    let dimension = host.dimension();
    let Some((hall, event)) = VISITORS.lock().ok().and_then(|list| list.iter()
        .find(|(dim, _, _, known)| dim == dimension && *known == id).map(|(_, hall, event, _)| (*hall, *event))) else {
        return false;
    };
    let Some(player) = host.acting_player() else { return true };
    let key = EVENTS[event].key;
    let name = Text::tr(&format!("veio.agenda.{key}.visitor"), &[]).plain();
    let mut state = None;
    host.change_mod_block_entity(hall, &mut |entity| {
        if entity.id == TOWN_HALL {
            state = proposals(entity).into_iter().find(|proposal| proposal.event == event);
        }
    });
    let Some(proposal) = state else { return true };
    let say = |line: Text| Text::tr("veio.agenda.says", &[&name, &line.plain()]);
    host.tell(player.uuid, say(Text::tr(&format!("veio.agenda.{key}.hello"), &[])));
    if proposal.prepared {
        host.tell(player.uuid, say(Text::tr(&format!("veio.agenda.{key}.thanks"), &[])));
        return true;
    }
    host.tell(player.uuid, say(Text::tr(&format!("veio.agenda.{key}.ask"), &[])));
    host.tell(player.uuid, need_text(&proposal, day_of(host.day_time())));
    true
}

/// O que falta, em uma linha.
pub(crate) fn need_text(proposal: &Proposal, today: i64) -> Text {
    let days = (proposal.due - today).max(0);
    let event = &EVENTS[proposal.event];
    if event.kind == Kind::Civic {
        let [x, _, z] = proposal.target;
        return Text::tr("veio.agenda.need_civic", &[&Text::tr(&format!("block.veio.{}", &event.amenity[5..]), &[]).plain(),
            &format!("{x} {z}"), &days]);
    }
    match proposal.missing() {
        Some((index, have, need)) => Text::tr("veio.agenda.need", &[&have, &need, &item_name(event.needs[index].0), &days]),
        None => Text::tr("veio.agenda.ready", &[&days]),
    }
}

fn item_name(item: &str) -> String {
    let name = item.split(':').nth(1).unwrap_or(item);
    let block = matches!(name, "white_banner" | "poppy" | "dandelion" | "campfire" | "lantern" | "candle" | "glass"
        | "white_wool" | "note_block" | "cake");
    Text::tr(&format!("{}.minecraft.{name}", if block { "block" } else { "item" }), &[]).plain()
}

// ---------- Ciclo da prefeitura ----------

/// Avisa todos os jogadores da dimensão.
fn announce(host: &mut dyn Host, line: &Text) {
    let dimension = host.dimension();
    let players: Vec<u128> = host.server_players().into_iter().filter(|player| player.dimension == dimension)
        .map(|player| player.uuid).collect();
    for player in players {
        host.tell(player, line.clone());
    }
}

fn adjust_morale(entity: &mut ModBlockEntity, delta: i32) {
    let morale = if entity.nbt.contains_key(nbt_key("Morale")) { number(entity, "Morale") } else { NEUTRAL };
    set_number(entity, "Morale", (morale + delta).clamp(0, 100));
}

pub(crate) fn morale(entity: &ModBlockEntity) -> i32 {
    if entity.nbt.contains_key(nbt_key("Morale")) { number(entity, "Morale") } else { NEUTRAL }
}

/// Um bairro sem lazer: a casa com moradores mais longe de qualquer parque, praça ou monumento.
fn district_without_leisure(dimension: &str, hall: [i32; 3], now: u64) -> Option<[i32; 3]> {
    homes_near(dimension, hall, HALL_RADIUS, now).into_iter()
        .filter(|home| home.residents > 0 && !leisure_near(dimension, home.pos, now))
        .max_by_key(|home| home.residents)
        .map(|home| home.pos)
}

/// A virada do dia: o evento de hoje acontece (ou não), pedidos vencidos saem, o ânimo volta
/// devagar para 50 e chegam os pedidos novos.
fn new_day(host: &mut dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity, today: i64, now: u64) {
    let dimension = host.dimension();
    let mut list = proposals(entity);
    // Ânimo volta 3 pontos por dia para o neutro.
    let morale = morale(entity);
    adjust_morale(entity, (NEUTRAL - morale).clamp(-3, 3));
    // O evento de hoje.
    if let Some(event) = scheduled(today) {
        let prepared = list.iter().any(|proposal| proposal.event == event && proposal.due == today && proposal.prepared);
        let info = &EVENTS[event];
        let name = Text::tr(&format!("veio.agenda.{}.name", info.key), &[]).plain();
        if prepared {
            adjust_morale(entity, info.cheer);
            announce(host, &Text::tr(if info.kind == Kind::Holiday { "veio.agenda.holiday_great" } else { "veio.agenda.sunday_great" }, &[&name]));
        } else if info.kind == Kind::Holiday {
            adjust_morale(entity, -10);
            announce(host, &Text::tr("veio.agenda.holiday_poor", &[&name]));
        }
    }
    // Pedidos vencidos: eventos de hoje ou antes saem; obra cívica não feita no prazo, -5.
    let mut gone = Vec::new();
    list.retain(|proposal| {
        let expired = if EVENTS[proposal.event].kind == Kind::Civic { proposal.due < today } else { proposal.due <= today };
        if expired {
            gone.push(proposal.clone());
        }
        !expired
    });
    for proposal in &gone {
        if EVENTS[proposal.event].kind == Kind::Civic && !proposal.prepared {
            adjust_morale(entity, -5);
        }
        send_away(host, dimension, pos, proposal.event);
    }
    // Pedidos novos: eventos até `LEAD` dias à frente; uma obra cívica a cada `CIVIC_EVERY` dias.
    for ahead in 1..=LEAD {
        let day = today + ahead;
        if let Some(event) = scheduled(day)
            && list.len() < MAX_PROPOSALS
            && !list.iter().any(|proposal| proposal.event == event && proposal.due == day)
        {
            list.push(Proposal { event, due: day, ..Proposal::default() });
            let name = Text::tr(&format!("veio.agenda.{}.visitor", EVENTS[event].key), &[]).plain();
            announce(host, &Text::tr("veio.agenda.arrived", &[&name]));
        }
    }
    if list.len() < MAX_PROPOSALS
        && today.rem_euclid(CIVIC_EVERY) == i64::from((pos[0] ^ pos[2]) & 3)
        && !list.iter().any(|proposal| EVENTS[proposal.event].kind == Kind::Civic)
        && let Some(target) = district_without_leisure(dimension, pos, now)
    {
        let event = CIVICS[(today / CIVIC_EVERY).rem_euclid(3) as usize];
        list.push(Proposal { event, due: today + CIVIC_DAYS, target, ..Proposal::default() });
        let name = Text::tr(&format!("veio.agenda.{}.visitor", EVENTS[event].key), &[]).plain();
        announce(host, &Text::tr("veio.agenda.arrived", &[&name]));
    }
    set_proposals(entity, &list);
}

/// A prefeitura reembolsa a companhia (conta pública -> companhia), até o que tiver.
fn reimburse(dimension: &str, hall: [i32; 3], amount: i32, now: u64) -> i32 {
    let paid = ledger::take(Book::Public, dimension, hall, 0, amount, ledger::WAGES, now);
    if paid > 0 && !ledger::put(Book::Company, dimension, hall, ledger::ANYWHERE, paid, ledger::RECEIVED, now) {
        // Sem companhia, o dinheiro volta ao cofre: nada some.
        ledger::put(Book::Public, dimension, hall, 0, paid, ledger::RECEIVED, now);
        return 0;
    }
    paid
}

/// Ciclo da agenda na prefeitura (a cada ciclo do posto).
pub(crate) fn cycle(host: &mut dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity, time: i64, staffed: bool) {
    let dimension = host.dimension();
    let now = defense::clock();
    let today = day_of(host.day_time());
    if !staffed {
        return;
    }
    if !entity.nbt.contains_key(nbt_key("AgendaDay")) || number(entity, "AgendaDay") != today as i32 {
        set_number(entity, "AgendaDay", today as i32);
        new_day(host, pos, entity, today, now);
    }
    // Entregas: o que fica pronto é reembolsado pela prefeitura e o visitante agradece.
    for proposal in deliver(entity) {
        let paid = reimburse(dimension, pos, value(proposal.event), now);
        let name = Text::tr(&format!("veio.agenda.{}.visitor", EVENTS[proposal.event].key), &[]).plain();
        announce(host, &Text::tr("veio.agenda.prepared", &[&name, &paid]));
    }
    // Obras cívicas: o bloco no bairro cumpre o pedido; a prefeitura paga a obra.
    let mut list = proposals(entity);
    let mut changed = false;
    for proposal in list.iter_mut().filter(|proposal| !proposal.prepared && EVENTS[proposal.event].kind == Kind::Civic) {
        let event = &EVENTS[proposal.event];
        if amenity_near(dimension, proposal.target, event.amenity, now).is_some() {
            proposal.prepared = true;
            changed = true;
            let paid = reimburse(dimension, pos, event.fee, now);
            adjust_morale(entity, event.cheer);
            let name = Text::tr(&format!("veio.agenda.{}.visitor", event.key), &[]).plain();
            announce(host, &Text::tr("veio.agenda.civic_done", &[&name, &paid]));
        }
    }
    if changed {
        set_proposals(entity, &list);
    }
    // Visitantes: um por pedido aberto, ao lado da prefeitura (no turno).
    if ledger::turn_edge(pos, time) {
        for (slot, proposal) in list.iter().enumerate() {
            if proposal.prepared && EVENTS[proposal.event].kind == Kind::Civic {
                send_away(host, dimension, pos, proposal.event);
            } else {
                ensure_visitor(host, dimension, pos, proposal.event, slot);
            }
        }
    }
    // Feriado: de dia, os aldeões vão para a praça (ou a prefeitura) comemorar.
    if is_holiday(today) && !night(host) && (time + i64::from(pos[0] & 7)).rem_euclid(200) == 0 {
        let spot = post_center(gathering_spot(dimension, pos, now));
        let at = center(pos);
        let radius = f64::from(HALL_RADIUS);
        let villagers: Vec<u32> = production::ranch::villager_positions(host, dimension).iter()
            .filter(|(_, spot)| (spot[0] - at[0]).abs() <= radius && (spot[2] - at[2]).abs() <= radius)
            .map(|(id, _)| *id).take(24).collect();
        for id in villagers {
            if order_for(id, now).is_none() {
                order(id, spot, now + 220);
            }
        }
        host.play_sound("minecraft:block.note_block.flute", post_center(gathering_spot(dimension, pos, now)), 4, 0.7,
            0.8 + (time.rem_euclid(5) as f32) * 0.1);
    }
    set_number(entity, "HolidayToday", i32::from(is_holiday(today)));
}

/// Os 13 valores do painel na aba *Agenda*.
pub(crate) fn data(entity: &ModBlockEntity, today: i64) -> [i32; 13] {
    let (next, ahead) = next_holiday(today);
    let kind = if is_holiday(today) { 2 } else if sunday(today) { 1 } else { 0 };
    let mut values = [0; 13];
    values[0] = number(entity, "Status") + 1000;
    values[1] = season_of(today) as i32 * 10 + season_day_of(today) as i32;
    values[2] = kind;
    values[3] = morale(entity);
    values[4] = next as i32;
    values[5] = ahead as i32;
    for (slot, proposal) in proposals(entity).iter().take(3).enumerate() {
        let (index, have, need) = proposal.missing().unwrap_or((0, 0, 0));
        let days = (proposal.due - today).clamp(0, 15) as i32;
        values[6 + slot * 2] = proposal.event as i32 + 1;
        values[7 + slot * 2] = (have.min(255) as i32) | ((need.min(255) as i32) << 8) | ((index as i32) << 16)
            | (days << 18) | (i32::from(proposal.prepared) << 22);
    }
    values[12] = number(entity, "Company");
    values
}

/// As linhas da aba *Agenda*.
pub(crate) fn lines(data: &[i32]) -> Vec<(String, u32)> {
    let read = |index: usize| data.get(index).copied().unwrap_or(0);
    let tr = |key: &str, args: &[&dyn std::fmt::Display]| Text::tr(key, args).plain();
    let season = tr(&format!("veio.season.{}", (read(1) / 10).clamp(0, 3)), &[]);
    let kind = tr(&format!("veio.agenda.daykind{}", read(2).clamp(0, 2)), &[]);
    let mut out = vec![
        (tr("veio.agenda.today", &[&season, &(read(1) % 10), &kind]), presentation::INK),
        (tr("veio.agenda.morale", &[&read(3)]), if read(3) >= NEUTRAL { 0x1e7a1e } else { 0x9a5a00 }),
        (tr("veio.agenda.next", &[&tr(&format!("veio.agenda.{}.name", EVENTS[read(4).clamp(0, 11) as usize].key), &[]), &read(5)]),
            0x1f4fa8),
    ];
    for slot in 0..3 {
        let event = read(6 + slot * 2);
        if event <= 0 || event as usize > EVENTS.len() {
            continue;
        }
        let info = &EVENTS[event as usize - 1];
        let packed = read(7 + slot * 2);
        let (have, need, index, days, ready) = (packed & 255, (packed >> 8) & 255, ((packed >> 16) & 3) as usize, (packed >> 18) & 15,
            packed & (1 << 22) != 0);
        let name = tr(&format!("veio.agenda.{}.name", info.key), &[]);
        let line = if ready {
            tr("veio.agenda.line_ready", &[&name, &days])
        } else if info.kind == Kind::Civic {
            tr("veio.agenda.line_civic", &[&name, &days])
        } else {
            tr("veio.agenda.line", &[&name, &have, &need, &item_name(info.needs[index].0), &days])
        };
        out.push((line, if ready { 0x1e7a1e } else { presentation::INK }));
    }
    out
}

/// Feriado hoje na prefeitura que governa `at`: os postos descansam.
pub(crate) fn holiday(dimension: &str, at: [i32; 3], day_time: i64, now: u64) -> bool {
    is_holiday(day_of(day_time)) && governing(dimension, at, now).is_some()
}

/// Relatório (livro): o calendário e os pedidos abertos.
pub(crate) fn report(entity: &ModBlockEntity, today: i64) -> Vec<Text> {
    let mut out = Vec::new();
    for (line, _) in lines(&data(entity, today)) {
        out.push(Text::literal(&line));
    }
    for proposal in proposals(entity) {
        out.push(need_text(&proposal, today));
    }
    out
}
