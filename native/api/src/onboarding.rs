//! Server-authoritative workshop tutorial. Observe real production and accepted
//! transfers; inventory contents and client clicks never complete production goals.
//! Successful transfers carry bounded, short-lived provenance through the line.
use super::*;
use horizon_client::mod_api::{NativeServerPlayer, Text};
use std::{collections::HashMap, path::{Path, PathBuf}, sync::{Mutex, atomic::{AtomicBool, AtomicU64, Ordering}}};

type Pos = [i32; 3];
const DONE: u8 = 16;
const GENERATOR_SLOT: usize = 0;
const PRESS_SLOT: usize = 1;
const ASSEMBLER_SLOT: usize = 2;
const DRILL_SLOT: usize = 3;
const HUB_SLOT: usize = 4;
const MAX_TRACE: usize = 128;
// Workers may walk and wait with a load. No short conveyor-speed deadline.
const TRACE_TTL: u64 = 6000;
/// Linhas "faça" de cada etapa (`veio.onboarding.do.<etapa>.<n>`), na ordem.
const DO_LINES: [u8; 17] = [3, 4, 4, 3, 3, 4, 3, 4, 3, 3, 3, 3, 3, 3, 3, 3, 3];
/// O que fabricar em cada etapa: o produto e os ingredientes da receita (sem repetir). A ficha
/// da etapa (texto gerado das receitas em `build_onboarding.py`) explica a receita; a lista de
/// materiais acima da barra de itens marca o que o jogador já tem. Nada é dado de graça.
const CRAFT: [&[(&str, &[&str])]; 16] = [
    &[(GENERATOR, &["minecraft:furnace", "minecraft:iron_ingot", "minecraft:copper_ingot"])],
    &[(PRESS, &["minecraft:copper_ingot", "minecraft:iron_ingot", "minecraft:piston", "minecraft:smooth_stone"]),
      (CABLE, &["minecraft:copper_ingot", "minecraft:stick"])],
    &[("minecraft:raw_copper", &[])],
    &[(ASSEMBLER, &[PLATE, "minecraft:piston", "minecraft:crafting_table"])],
    &[(DRILL, &["minecraft:copper_ingot", "minecraft:iron_ingot", "minecraft:piston"])],
    &[(BELT, &["minecraft:copper_ingot", "minecraft:stick"])],
    &[("minecraft:chest", &["#minecraft:planks"])],
    &[(HUB, &["minecraft:chest", GEAR, "minecraft:iron_ingot", "minecraft:copper_ingot"])],
    // Os sistemas: Quadro, fase, outra extratora, divisor, postes, pedido do dia, Lar, centro.
    // Contratar e os postos de trabalho ficam com o capítulo da vila (`village.rs`).
    &[(milestones::BOARD, &["minecraft:chest", "minecraft:copper_ingot", "minecraft:stick"])],
    &[],
    &[(DRILL, &["minecraft:copper_ingot", "minecraft:iron_ingot", "minecraft:piston"])],
    &[(SPLITTER, &[GEAR, BELT, "minecraft:redstone"])],
    &[(POLE, &["minecraft:stick", "minecraft:copper_ingot"])],
    &[],
    &[],
    &[],
];
/// Etapas dos sistemas (8 a 15), na ordem.
const BOARD_STEP: u8 = 8;
const PHASE_STEP: u8 = 9;
const DRILL_STEP: u8 = 10;
const SPLIT_STEP: u8 = 11;
const POLE_STEP: u8 = 12;
const ORDER_STEP: u8 = 13;
const MEAL_STEP: u8 = 14;
const CENTER_STEP: u8 = 15;
/// Chapas da primeira fase do Quadro (`milestones::PHASES[0]`).
const PHASE_COUNT: u32 = 16;
/// Coisas que só acontecem uma vez, guardadas em `Progress::events`.
const EVENT_MEAL: u32 = 1;
const EVENT_CENTER: u32 = 2;
const EVENT_SPLIT: u32 = 4;
/// As 16 etapas em 4 atos de 4: cada ato tem nome, título na tela e fala do prefeito.
const ACT_STEPS: u8 = 4;
/// Empacou: depois deste tempo na mesma etapa (relógio do tutorial, 3 min), o prefeito dá uma
/// dica do erro mais comum. Uma vez por etapa.
const HINT_AFTER: u64 = 3600;
/// Longe disto (blocos) do lugar da etapa, a barra diz para onde ir.
const FAR: i32 = 24;

fn craft_bit(step: usize, part: usize) -> u32 { 1 << (step * 2 + part) }

/// Dicas de uma vez só, na hora em que fazem sentido (como as do Factorio).
const TIP_FACES: u16 = 1;
const TIP_BELT: u16 = 2;
const TIPS: [(u16, &str); 2] = [(TIP_FACES, "faces"), (TIP_BELT, "belt")];
static ACTIVE: AtomicBool = AtomicBool::new(false);
static FLOW_ACTIVE: AtomicBool = AtomicBool::new(false);
static SERVER_CLOCK: AtomicU64 = AtomicU64::new(0);
static SESSION: Mutex<Option<Session>> = Mutex::new(None);

#[derive(Clone, Copy, Debug)]
struct Token { count: u32, seen: u64 }

#[derive(Default, Debug)]
struct Progress {
    step: u8,
    hidden: bool,
    dimension: String,
    blocks: [Option<Pos>; 5],
    powered: bool,
    plates: u32,
    gears: u32,
    copper: u32,
    fed: u32,
    made: u32,
    stored: u32,
    delivered: bool,
    /// Produtos da etapa já fabricados (`craft_bit`) e dicas já vistas (bits), salvos.
    crafted: u32,
    tips: u16,
    pending_tips: u16,
    /// Prólogo (`intro`): casa, prefeito e terreno; `plot` é o centro do terreno e o nível dele;
    /// `outro` marca a última conversa com o prefeito. Salvos.
    intro: u8,
    plot: Option<Pos>,
    outro: bool,
    /// Onde o prefeito ficou (para a seta quando ele está longe, em chunk descarregado). Salvo.
    mayor: Option<[f64; 3]>,
    /// Sistemas (etapas 8 a 15): o Quadro e o último poste colocados, as extratoras novas e o
    /// que aconteceu uma vez (`EVENT_*`). Salvos. `poles` (não salvo) conta os postes da etapa.
    board: Option<Pos>,
    pole: Option<Pos>,
    drills: u32,
    events: u32,
    poles: u32,
    /// Festas do capítulo da vila (`festival`): qual está marcada (0 nenhuma) e o dia dela. Salvos.
    festival: u8,
    festival_day: u32,
    /// O favor do primeiro ato (`favors.rs`): a prensa vem do Seu Bento depois do campo da Dona
    /// Cida. Salvo; os pés de trigo contados são só da sessão.
    favor: u8,
    favor_crops: u32,
    /// Dica do prefeito: a etapa que está contando (fábrica, ou 100 + etapa da vila), desde
    /// quando, e se a dica já saiu. Não salvo.
    hint_for: Option<u16>,
    hint_since: u64,
    hinted: bool,
    /// Lido do mundo a cada segundo, não salvo: chapas entregues, fase e pedido do dia do
    /// Quadro, e o poste com energia.
    board_delivered: u32,
    board_phase: usize,
    order_done: u32,
    order_paid: bool,
    pole_powered: bool,
    intro_announced: Option<u8>,
    intro_since: Option<u64>,
    plot_level: Option<intro::Level>,
    /// Capítulo da vila (`village`): etapa (0 não começou, 1..=8, 9 fim), os blocos do jogador
    /// (posto, marco, carregador, capataz, casa, prefeitura), o que já fabricou e a base das
    /// colheitas. Salvos; a contagem e o anúncio, não.
    village: u8,
    village_blocks: [Option<Pos>; 6],
    village_crafted: u16,
    village_base: i32,
    village_count: i32,
    village_announced: Option<u8>,
    /// Capítulo da cidade (`city`): etapa (0 não começou, 1..=8, 9 fim) e o que já fabricou. Salvos.
    city: u8,
    city_crafted: u16,
    city_announced: Option<u8>,
    /// Quando a etapa da cidade começou (relógio da sociedade) e se o prefeito já deu a dica.
    city_since: Option<u64>,
    city_hinted: bool,
    /// O número que a etapa da cidade acompanha, para o "plim" quando ele sobe.
    city_metric: Option<i32>,
    // Unsaved transient evidence must not survive a reload or manual intervention.
    input_credit: u32,
    output_credit: u32,
    raw: HashMap<Pos, Token>,
    finished: HashMap<Pos, Token>,
    cargo: HashMap<Pos, Token>,
    announced: Option<u8>,
    status: i32,
    last_status_notice: Option<i32>,
    next_status_notice: u64,
    dirty: bool,
    save_warned: bool,
}

impl Progress {
    fn placed(&mut self, dimension: &str, pos: Pos, block: &str) {
        let slot = match block {
            GENERATOR => GENERATOR_SLOT, PRESS => PRESS_SLOT, ASSEMBLER => ASSEMBLER_SLOT,
            DRILL | DRILL_MK2 | production::DRILL_MK3 => DRILL_SLOT, HUB => HUB_SLOT,
            _ => return,
        };
        // A newly placed block at the same coordinates is a different machine.
        self.blocks[slot] = None;
        self.bind(dimension, pos, block);
    }

    fn bind(&mut self, dimension: &str, pos: Pos, block: &str) {
        let slot = match block {
            GENERATOR => GENERATOR_SLOT, PRESS => PRESS_SLOT, ASSEMBLER => ASSEMBLER_SLOT,
            DRILL | DRILL_MK2 | production::DRILL_MK3 => DRILL_SLOT, HUB => HUB_SLOT,
            _ => return,
        };
        if self.dimension != dimension {
            // Evidence from separate dimensions cannot form a production line.
            self.blocks = [None; 5];
            self.dimension = dimension.to_owned();
            self.clear_flow();
        }
        if self.blocks[slot] != Some(pos) {
            self.blocks[slot] = Some(pos);
            if slot == PRESS_SLOT { self.powered = false; self.plates = 0; self.clear_flow(); }
            if slot == ASSEMBLER_SLOT { self.gears = 0; }
            if slot == DRILL_SLOT { self.copper = 0; self.clear_flow(); }
            if slot == HUB_SLOT { self.delivered = false; }
            self.dirty = true;
        }
    }

    /// Blocos dos sistemas (etapas 8 a 15): colocar o Quadro, um posto ou um poste, ou abrir um
    /// Quadro ou posto que já existe, ou o centro de controle na etapa dele.
    fn systems(&mut self, pos: Pos, block: &str, clicked: bool) {
        let systems = self.step >= BOARD_STEP;
        match block {
            milestones::BOARD if !clicked || (systems && self.board.is_none()) => {
                if self.board != Some(pos) { self.board = Some(pos); self.board_delivered = 0; self.dirty = true; }
            }
            DRILL | DRILL_MK2 | production::DRILL_MK3 if !clicked && self.step == DRILL_STEP => {
                self.drills = (self.drills + 1).min(64); self.dirty = true;
            }
            SPLITTER if !clicked && self.step == SPLIT_STEP => { self.events |= EVENT_SPLIT; self.dirty = true; }
            POLE | STEEL_POLE if !clicked => {
                self.poles = (self.poles + 1).min(64); self.pole = Some(pos); self.pole_powered = false; self.dirty = true;
            }
            workforce::CONTROL_CENTER if clicked && self.step == CENTER_STEP => {
                self.events |= EVENT_CENTER; self.dirty = true;
            }
            _ => {}
        }
    }

    fn clear_flow(&mut self) {
        self.input_credit = 0; self.output_credit = 0;
        self.fed = 0; self.made = 0; self.stored = 0;
        self.raw.clear(); self.finished.clear(); self.cargo.clear();
    }

    fn ready(&self) -> bool {
        match self.step {
            0 => self.blocks[GENERATOR_SLOT].is_some(),
            1 => self.powered,
            2 => self.plates >= 4,
            3 => self.gears >= 1,
            4 => self.copper >= 1,
            5 => self.fed >= 2 && self.made >= 1,
            6 => self.stored >= 4,
            7 => self.delivered,
            BOARD_STEP => self.board.is_some(),
            PHASE_STEP => self.board_phase >= 1,
            DRILL_STEP => self.drills >= 1,
            SPLIT_STEP => self.events & EVENT_SPLIT != 0,
            POLE_STEP => self.poles >= 2 && self.pole_powered,
            ORDER_STEP => self.order_paid,
            MEAL_STEP => self.events & EVENT_MEAL != 0,
            CENTER_STEP => self.events & EVENT_CENTER != 0,
            _ => false,
        }
    }

    #[cfg(test)]
    fn observe(&mut self, pos: Pos, entity: &ModBlockEntity, before: i32, contract_before: i32) {
        self.observe_counts(pos, entity, before, contract_before, None);
    }

    fn observe_counts(&mut self, pos: Pos, entity: &ModBlockEntity, before: i32, contract_before: i32, previous_cycles: Option<i32>) {
        let products = number(entity, "Produced").saturating_sub(before).clamp(0, 64) as u32;
        let slot = match self.step { 3 => ASSEMBLER_SLOT, 4 => DRILL_SLOT, _ => PRESS_SLOT };
        if self.blocks[slot] == Some(pos) { self.status = number(entity, "Status"); }
        if self.blocks[PRESS_SLOT] == Some(pos) && entity.id == PRESS {
            if self.step == 1 {
                self.powered = number(entity, "energy") > 0 || (number(entity, "Status") == 1
                    && (number(entity, "Progress") > 0 || products > 0));
            }
            let copper_plate = entity.items.get(1).and_then(Option::as_ref).is_some_and(|s| is_plain(s, PLATE));
            if copper_plate && products > 0 {
                if self.step == 2 { self.plates = (self.plates + products).min(4); self.dirty = true; }
                if matches!(self.step, 5 | 6) {
                    let cycles = previous_cycles.map_or(products, |old| number(entity, "Total").saturating_sub(old).clamp(0, 64) as u32);
                    let inputs = cycles.min(self.input_credit);
                    self.input_credit -= inputs;
                    let certified = if cycles == 0 { 0 } else { products * inputs / cycles };
                    self.output_credit = (self.output_credit + certified).min(64);
                    self.made = (self.made + certified).min(64);
                    self.dirty |= certified > 0;
                }
            }
        }
        if self.step == 3 && self.blocks[ASSEMBLER_SLOT] == Some(pos) && entity.id == ASSEMBLER
            && entity.items.get(1).and_then(Option::as_ref).is_some_and(|s| is_plain(s, GEAR)) && products > 0 {
            self.gears = 1; self.dirty = true;
        }
        if self.step == 4 && self.blocks[DRILL_SLOT] == Some(pos)
            && matches!(entity.id.as_str(), DRILL | DRILL_MK2 | production::DRILL_MK3)
            && entity.items.first().and_then(Option::as_ref).is_some_and(|s| is_plain(s, "minecraft:raw_copper")) && products > 0 {
            self.copper = 1; self.dirty = true;
        }
        if self.step == 7 && self.blocks[HUB_SLOT] == Some(pos) && entity.id == HUB
            && contract_before == 0 && number(entity, "ContractStage") == 1 {
            self.delivered = true; self.dirty = true;
        }
    }

    fn transfer(&mut self, from: Pos, to: Pos, item: &str, count: u32, chest: bool, now: u64) {
        if !matches!(self.step, 5 | 6) { return; }
        if item == "minecraft:raw_copper" {
            // A stocked chest, hopper or worker post is a valid automatic
            // supplier. The observer reports actual accepted transfers only.
            if self.blocks[PRESS_SLOT] == Some(to) {
                self.input_credit = (self.input_credit + count).min(64);
                self.fed = (self.fed + count).min(64);
                self.dirty |= count > 0;
                return;
            }
            let certified = if self.blocks[DRILL_SLOT] == Some(from) { count }
                else { take_trace(&mut self.raw, from, count, now) };
            if certified == 0 { return; }
            put_trace(&mut self.raw, to, certified, now);
        } else if item == PLATE {
            let certified = self.take_output(from, count, now);
            if certified == 0 { return; }
            if chest { self.stored = (self.stored + certified).min(4); self.dirty = true; }
            else { put_trace(&mut self.finished, to, certified, now); }
        }
    }

    fn take_output(&mut self, from: Pos, count: u32, now: u64) -> u32 {
        if self.blocks[PRESS_SLOT] == Some(from) {
            let n = count.min(self.output_credit); self.output_credit -= n; n
        } else { take_trace(&mut self.finished, from, count, now) }
    }

    fn worker_pickup(&mut self, post: Pos, from: Pos, item: &str, count: u32, now: u64) {
        if !matches!(self.step, 5 | 6) || item != PLATE { return; }
        let n = self.take_output(from, count, now);
        if n > 0 { put_trace(&mut self.cargo, post, n, now); }
    }

    fn worker_delivery(&mut self, post: Pos, to: Pos, item: &str, count: u32, storage: bool, now: u64) {
        if !matches!(self.step, 5 | 6) { return; }
        if item == "minecraft:raw_copper" {
            self.transfer(post, to, item, count, storage, now);
        } else if item == PLATE {
            let n = take_trace(&mut self.cargo, post, count, now);
            if n == 0 { return; }
            if storage { self.stored = (self.stored + n).min(4); self.dirty = true; }
            else { put_trace(&mut self.finished, to, n, now); }
        }
    }

    fn encode(&self) -> String {
        let mut text = format!("veio-onboarding-9\n{} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}\n{}\n",
            self.step, u8::from(self.hidden), u8::from(self.powered), self.plates, self.gears,
            self.copper, self.fed, self.made, self.stored, u8::from(self.delivered), self.crafted, self.tips,
            self.intro, u8::from(self.outro), self.village, self.village_crafted, self.village_base.max(0),
            self.city, self.city_crafted, self.drills, self.events, self.festival, self.festival_day, self.favor, self.dimension);
        for pos in self.blocks.iter().chain([&self.plot]).chain(self.village_blocks.iter()) {
            if let Some([x,y,z]) = pos { text.push_str(&format!("{x} {y} {z}\n")); }
            else { text.push_str("-\n"); }
        }
        match self.mayor {
            Some([x, y, z]) => text.push_str(&format!("{x} {y} {z}\n")),
            None => text.push_str("-\n"),
        }
        for pos in [&self.board, &self.pole] {
            if let Some([x,y,z]) = pos { text.push_str(&format!("{x} {y} {z}\n")); }
            else { text.push_str("-\n"); }
        }
        text
    }

    fn decode(text: &str) -> Option<Self> {
        let mut lines = text.lines();
        // v4: o começo novo (prefeito salvo); v5: mais o capítulo da vila (3 números e 6 blocos);
        // v6: mais o capítulo da cidade (2 números); v7: mais as etapas dos sistemas da fábrica
        // (2 números; Quadro e poste depois do prefeito).
        let (fields, version) = match lines.next()? {
            "veio-onboarding-1" => (10, 1), "veio-onboarding-2" => (12, 2), "veio-onboarding-3" => (14, 3),
            "veio-onboarding-4" => (14, 4), "veio-onboarding-5" => (17, 5), "veio-onboarding-6" => (19, 6),
            "veio-onboarding-7" => (21, 7), "veio-onboarding-8" => (23, 8), "veio-onboarding-9" => (24, 9), _ => return None,
        };
        let version4 = version >= 4;
        let values: Vec<u32> = lines.next()?.split_whitespace().map(str::parse).collect::<Result<_,_>>().ok()?;
        if values.len() != fields || values[0] > u32::from(DONE) { return None; }
        let (crafted, tips) = if fields >= 12 { (values[10], values[11] as u16) } else { (0, 0) };
        let (drills, events) = if version >= 7 { (values[19].min(64), values[20]) } else { (0, 0) };
        // v8: as festas do capítulo da vila (qual está marcada e o dia).
        let (festival, festival_day) = if version >= 8 { ((values[21] as u8).min(festival::DONE), values[22]) } else { (0, 0) };
        // Saves de antes do prólogo: quem já começou a fábrica não volta para a casa e o prefeito.
        // A v3 não tinha a vila nem os recursos: 0 casa, 1 prefeito, 2 terreno, 3 pronto.
        let (intro, outro) = if version4 { ((values[12] as u8).min(intro::INTRO_DONE), values[13] != 0) }
            else if fields == 14 { ([intro::VILLAGE_STAGE, intro::MAYOR_STAGE, intro::PLOT_STAGE, intro::INTRO_DONE]
                [usize::from((values[12] as u8).min(3))], values[13] != 0) }
            else if values[0] > 0 || crafted != 0 { (intro::INTRO_DONE, values[0] == u32::from(DONE)) }
            else { (0, false) };
        // v9: o favor do primeiro ato. Antes dele o prefeito já dava as receitas da prensa e dos
        // cabos: quem passou do pedido de recursos não faz o favor.
        let favor = if version >= 9 { (values[23] as u8).min(favors::FAVOR_DONE) }
            else if intro >= intro::PLOT_STAGE { favors::FAVOR_DONE } else { 0 };
        let dimension = lines.next()?.to_owned();
        if dimension.len() > 128 { return None; }
        let parse = |line: &str| -> Option<Option<Pos>> {
            if line == "-" { return Some(None); }
            let coords: Vec<i32> = line.split_whitespace().map(str::parse).collect::<Result<_,_>>().ok()?;
            Some(Some(coords.try_into().ok()?))
        };
        let mut blocks = [None; 5];
        for block in &mut blocks { *block = parse(lines.next()?)?; }
        let plot = if fields >= 14 { parse(lines.next()?)? } else { None };
        let mut village_blocks = [None; 6];
        let (village, village_crafted, village_base) = if version >= 5 {
            for block in &mut village_blocks { *block = parse(lines.next()?)?; }
            ((values[14] as u8).min(village::DONE_VILLAGE), values[15] as u16, values[16] as i32)
        } else { (0, 0, 0) };
        let (city, city_crafted) = if version >= 6 { ((values[17] as u8).min(city::DONE_CITY), values[18] as u16) } else { (0, 0) };
        let mayor = if version4 { lines.next().and_then(|line| {
            let coords: Vec<f64> = line.split_whitespace().map(str::parse).collect::<Result<_,_>>().ok()?;
            coords.try_into().ok()
        }) } else { None };
        let mut more = [None; 2];
        if version >= 7 { for pos in &mut more { *pos = parse(lines.next()?)?; } }
        let [board, pole] = more;
        // Até a v6 a fábrica acabava na etapa 8. Quem já abriu o capítulo da vila (ou da cidade)
        // fica nele (a fábrica conta como pronta); os outros seguem para as etapas dos sistemas e
        // voltam ao prefeito no fim das 16.
        let mut step = values[0] as u8;
        let mut outro = outro;
        if version <= 6 && step == 8 {
            if village > 0 || city > 0 { step = DONE; outro = true; } else { outro = false; }
        }
        // Sem o terreno salvo, o prefeito escolhe outro.
        let intro = if intro == intro::PLOT_STAGE && plot.is_none() { intro::MAYOR_STAGE } else { intro };
        Some(Self { step, hidden: values[1] != 0, powered: values[2] != 0,
            plates: values[3].min(4), gears: values[4].min(1), copper: values[5].min(1),
            fed: values[6].min(64), made: values[7].min(64), stored: values[8].min(4),
            delivered: values[9] != 0, crafted, tips, dimension, blocks, intro, plot, outro, mayor,
            village, village_blocks, village_crafted, village_base, city, city_crafted, board, pole, drills, events,
            festival, festival_day, favor, poles: u32::from(pole.is_some()), ..Self::default() })
    }
}

fn take_trace(trace: &mut HashMap<Pos, Token>, from: Pos, count: u32, now: u64) -> u32 {
    if count == 0 { return 0; }
    let Some(token) = trace.get_mut(&from) else { return 0; };
    let n = if now.saturating_sub(token.seen) <= TRACE_TTL { token.count.min(count) } else { 0 };
    token.count -= n;
    if token.count == 0 || n == 0 { trace.remove(&from); }
    n
}

fn put_trace(trace: &mut HashMap<Pos, Token>, to: Pos, count: u32, now: u64) {
    trace.retain(|_, token| now.saturating_sub(token.seen) <= TRACE_TTL);
    if trace.len() >= MAX_TRACE && !trace.contains_key(&to) { return; }
    let token = trace.entry(to).or_insert(Token { count: 0, seen: now });
    token.count = (token.count + count).min(64); token.seen = now;
}

pub(super) fn tracking() -> bool { ACTIVE.load(Ordering::Relaxed) }

pub(super) fn visible_goal(uuid: u128) -> bool {
    SESSION.lock().ok().and_then(|slot| slot.as_ref().and_then(|session|
        session.players.get(&uuid).map(|progress| (progress.step < DONE || progress.intro < intro::INTRO_DONE || !progress.outro
            || village::step(progress).is_some() || city::step(progress).is_some())
            && !progress.hidden)))
        .unwrap_or(false)
}

struct Session { root: PathBuf, players: HashMap<u128, Progress>, pending: Vec<transport::Pending> }

fn session(host: &dyn Host, slot: &mut Option<Session>) -> bool {
    let Some(world) = host.world_dir() else {
        if let Some(old) = slot.as_mut() {
            for (uuid, p) in &mut old.players {
                if p.dirty { let _ = save(&old.root, *uuid, p); }
            }
        }
        *slot = None; ACTIVE.store(false, Ordering::Relaxed); FLOW_ACTIVE.store(false, Ordering::Relaxed);
        return false;
    };
    let root = world.join("veio").join("tutorial");
    if slot.as_ref().is_none_or(|s| s.root != root) {
        if let Some(old) = slot.as_mut() {
            for (uuid, p) in &mut old.players {
                if p.dirty { let _ = save(&old.root, *uuid, p); }
            }
        }
        *slot = Some(Session { root, players: HashMap::new(), pending: Vec::new() });
    }
    true
}

fn path(root: &Path, uuid: u128) -> PathBuf { root.join(format!("{uuid:032x}.txt")) }

fn load(root: &Path, uuid: u128) -> Progress {
    std::fs::read_to_string(path(root, uuid)).ok().and_then(|text| Progress::decode(&text)).unwrap_or_default()
}

fn save(root: &Path, uuid: u128, progress: &mut Progress) -> std::io::Result<()> {
    std::fs::create_dir_all(root)?;
    let target = path(root, uuid);
    let temp = target.with_extension("tmp");
    std::fs::write(&temp, progress.encode())?;
    std::fs::rename(temp, target)?;
    progress.dirty = false; progress.save_warned = false;
    Ok(())
}

/// Call only after successful placement, or when deliberately selecting an existing
/// machine by opening it. No nearest-player guessing; another player's factory
/// cannot complete your workshop unless you explicitly select that machine.
pub(super) fn select(host: &dyn Host, pos: Pos, block: &str, clicked: bool) {
    let tutorial = matches!(block, GENERATOR | PRESS | ASSEMBLER | DRILL | DRILL_MK2 | production::DRILL_MK3 | HUB);
    let tip = tip_for(block).filter(|_| !clicked);
    let part = !clicked && CRAFT.iter().any(|parts| parts.iter().any(|(target, _)| *target == block));
    let village_block = !clicked && village::slot(block).is_some();
    let system = system_block(block);
    if !tutorial && tip.is_none() && !part && !village_block && !system { return; }
    let Some(player) = host.acting_player() else { return; };
    let Ok(mut slot) = SESSION.lock() else { return; };
    if !session(host, &mut slot) { return; }
    let s = slot.as_mut().unwrap();
    let progress = s.players.entry(player.uuid).or_insert_with(|| load(&s.root, player.uuid));
    if let Some(bit) = tip && progress.tips & bit == 0 { progress.pending_tips |= bit; }
    if village_block { village::placed(progress, &player.dimension, pos, block); }
    // Colocou o produto da etapa: já foi fabricado (a lista de materiais não volta).
    if part && let Some(parts) = CRAFT.get(usize::from(progress.step)) {
        for (index, (target, _)) in parts.iter().enumerate() {
            if *target == block { progress.crafted |= craft_bit(usize::from(progress.step), index); progress.dirty = true; }
        }
    }
    if system && progress.step < DONE { progress.systems(pos, block, clicked); }
    if !tutorial || progress.step == DONE { return; }
    if clicked { progress.bind(&player.dimension, pos, block); }
    else { progress.placed(&player.dimension, pos, block); }
    if clicked && block == PRESS {
        // Opening the input/output permits manual changes: wait for fresh
        // automated receipts and cycles before certifying more outputs.
        progress.input_credit = 0; progress.output_credit = 0;
    }
    ACTIVE.store(true, Ordering::Relaxed);
}

/// Blocos das etapas dos sistemas (8 a 15).
fn system_block(block: &str) -> bool {
    matches!(block, milestones::BOARD | POLE | STEEL_POLE | DRILL | DRILL_MK2 | production::DRILL_MK3 | SPLITTER
        | workforce::CONTROL_CENTER)
}

/// Refeição no Lar (`workforce::home`): conta para a etapa da casa.
pub(super) fn ate_at_home(uuid: u128) {
    if let Ok(mut slot) = SESSION.lock() && let Some(s) = slot.as_mut()
        && let Some(p) = s.players.get_mut(&uuid) && p.step == MEAL_STEP && p.events & EVENT_MEAL == 0 {
        p.events |= EVENT_MEAL; p.dirty = true;
    }
}

/// O que as etapas dos sistemas leem do mundo, uma vez por segundo: o Quadro (fase e pedido do
/// dia) e o último poste. Só lê o bloco da etapa atual.
fn look(host: &mut dyn Host, player: &NativeServerPlayer, p: &mut Progress) {
    if player.dimension != p.dimension && !p.dimension.is_empty() { return; }
    match p.step {
        PHASE_STEP => {
            if let Some(board) = p.board && let Some(e) = host.mod_block_entity(board) && e.id == milestones::BOARD {
                p.board_phase = number(&e, "Phase").max(0) as usize;
                p.board_delivered = (number(&e, "Delivered").max(0) as u32).min(PHASE_COUNT);
            } else if let Some(phase) = milestones::census::board_phase(&player.dimension) {
                // Quadro longe (chunk descarregado) ou outro Quadro da vila: vale a fase dele.
                p.board_phase = p.board_phase.max(phase);
            }
        }
        ORDER_STEP => if let Some(board) = p.board && let Some(e) = host.mod_block_entity(board) && e.id == milestones::BOARD {
            // Pago a este Quadro (1) ou a outro Quadro da vila no mesmo dia (2).
            p.order_done = number(&e, "OrderDone").max(0) as u32;
            p.order_paid = number(&e, "OrderPaid") >= 1;
        },
        POLE_STEP => if let Some(pole) = p.pole && let Some(e) = host.mod_block_entity(pole) {
            p.pole_powered = matches!(e.id.as_str(), POLE | STEEL_POLE) && number(&e, "energy") > 0;
        },
        _ => {}
    }
}

/// A dica que colocar este bloco desperta: esteira (direção e carga) ou máquina (entrada e saída).
fn tip_for(block: &str) -> Option<u16> {
    if logistics::is_belt(block) { return Some(TIP_BELT); }
    (is_factory_block(block) && !logistics::transport(block) && !output_slots(block).is_empty()
        && has_item_inputs(block)).then_some(TIP_FACES)
}

/// Anuncia a etapa: título, por quê, a ficha (receita e de onde vem cada material), o livro de
/// receitas com a receita liberada, e os passos numerados.
fn announce(host: &mut dyn Host, player: &NativeServerPlayer, p: &mut Progress) {
    let step = usize::from(p.step);
    host.tell(player.uuid, Text::tr(&format!("veio.onboarding.title.{step}"), &[]));
    host.tell(player.uuid, Text::tr(&format!("veio.onboarding.why.{step}"), &[]));
    if p.step % ACT_STEPS == 0 && p.step < DONE {
        host.tell(player.uuid, Text::tr(&format!("veio.onboarding.act.{}.line", p.step / ACT_STEPS), &[]));
    }
    intro::say(host, player.uuid, &format!("veio.onboarding.story.{step}"), &[]);
    if let Some(parts) = CRAFT.get(step).filter(|parts| !parts.is_empty()) {
        for (index, (target, _)) in parts.iter().enumerate() {
            if target.starts_with("veio:") || *target == "minecraft:chest" {
                host.tell(player.uuid, Text::tr(&format!("veio.onboarding.recipe.{step}.{index}"), &[]));
                if target.starts_with("veio:") && spawn_safe(&player.name) {
                    host.run_command(&format!("recipe give {} {target}", player.name));
                }
            }
        }
        host.tell(player.uuid, Text::tr(&format!("veio.onboarding.from.{step}"), &[]));
        if parts.iter().any(|(target, _)| target.starts_with("veio:")) {
            host.tell(player.uuid, Text::tr("veio.onboarding.book", &[]));
        }
    }
    for line in 1..=DO_LINES.get(step).copied().unwrap_or(0) {
        host.tell(player.uuid, Text::tr(&format!("veio.onboarding.do.{step}.{line}"), &[]));
    }
    if step == 0 { host.tell(player.uuid, Text::tr("veio.wiki.open", &[])); }
}

fn name_key(item: &str) -> String {
    format!("veio.onboarding.name.{}", item.rsplit(':').next().unwrap_or(item))
}

fn has_item(host: &mut dyn Host, name: &str, item: &str) -> bool {
    host.run_command(&format!("execute if items entity {name} container.* {item}"))
}

/// Contagem do objetivo da etapa (o que aparece como x/N na barra).
fn goal_count(p: &Progress) -> u32 {
    match p.step { 2 => p.plates, 3 => p.gears, 4 => p.copper, 5 => p.fed.min(2), 6 => p.stored,
        PHASE_STEP => p.board_delivered, DRILL_STEP => p.drills.min(1), POLE_STEP => p.poles.min(2),
        ORDER_STEP => p.order_done, _ => 0 }
}

/// Lista de materiais acima da barra de itens, enquanto o produto da etapa não foi fabricado:
/// "Fabrique: Prensa  ✔ lingote de cobre  ✘ pistão ...", verde o que já tem, vermelho o que falta.
/// Devolve se mostrou (então o objetivo comum espera).
fn checklist(host: &mut dyn Host, player: &NativeServerPlayer, p: &mut Progress) -> bool {
    let step = usize::from(p.step);
    let Some(parts) = CRAFT.get(step) else { return false };
    if goal_count(p) > 0 || !spawn_safe(&player.name) { return false; }
    for (index, (target, ingredients)) in parts.iter().enumerate() {
        let bit = craft_bit(step, index);
        if p.crafted & bit != 0 { continue; }
        if has_item(host, &player.name, target) { p.crafted |= bit; p.dirty = true; continue; }
        let mut line = vec![format!(r#"{{"translate":"veio.onboarding.make","with":[{{"translate":"{}"}}],"color":"gold"}}"#, name_key(target))];
        for item in *ingredients {
            let (mark, color) = if has_item(host, &player.name, item) { ("✔", "green") } else { ("✘", "red") };
            line.push(format!(r#"{{"text":"  {mark} ","color":"{color}"}},{{"translate":"{}","color":"{color}"}}"#, name_key(item)));
        }
        host.run_command(&format!(r#"title {} actionbar ["",{}]"#, player.name, line.join(",")));
        return true;
    }
    false
}

/// Etapa concluída: título na tela, som de conquista e um pouco de experiência.
fn celebrate(host: &mut dyn Host, player: &NativeServerPlayer, step: u8) {
    if !spawn_safe(&player.name) { return; }
    let name = &player.name;
    if step % ACT_STEPS == 0 {
        // Fim de um ato: fogos e o nome do próximo ato (ou o fim da fábrica).
        let act = step / ACT_STEPS;
        let (title, subtitle) = if step < DONE { (format!("veio.onboarding.act.{act}"), format!("veio.onboarding.act.{act}.sub")) }
            else { ("veio.onboarding.act.end".to_owned(), "veio.onboarding.act.end.sub".to_owned()) };
        host.run_command(&format!(r#"title {name} subtitle {{"translate":"{subtitle}"}}"#));
        host.run_command(&format!(r#"title {name} title {{"translate":"{title}","color":"gold"}}"#));
        fireworks(host, name);
    } else {
        host.run_command(&format!(r#"title {name} subtitle {{"translate":"veio.onboarding.title.{step}"}}"#));
        host.run_command(&format!(r#"title {name} title {{"translate":"veio.onboarding.done","color":"gold"}}"#));
    }
    host.run_command(&format!("execute as {name} at @s run playsound minecraft:ui.toast.challenge_complete master @s"));
    host.run_command(&format!("xp add {name} {} points", 10 + 5 * u32::from(step)));
}

/// Fogos de artifício em volta do jogador: o fim de um ato ou de um capítulo.
pub(super) fn fireworks(host: &mut dyn Host, name: &str) {
    if !spawn_safe(name) { return; }
    for (dx, dz, color) in [(2, 0, 16766720), (-2, 1, 3066993), (0, -2, 15158332)] {
        host.run_command(&format!(concat!("execute at {} run summon minecraft:firework_rocket ~{} ~1 ~{} ",
            r#"{{LifeTime:22,FireworksItem:{{id:"minecraft:firework_rocket",count:1,components:{{"minecraft:fireworks":"#,
            r#"{{flight_duration:1,explosions:[{{shape:"star",colors:[I;{}],has_twinkle:true}}]}}}}}}}}"#), name, dx, dz, color));
    }
}

/// Dica do prefeito quando o jogador fica muito tempo na mesma etapa (`key`): uma vez.
fn hint(host: &mut dyn Host, player: &NativeServerPlayer, p: &mut Progress, now: u64, key: u16, text: &str) {
    if p.hint_for != Some(key) { p.hint_for = Some(key); p.hint_since = now; p.hinted = false; return; }
    if p.hinted || p.hidden || now.saturating_sub(p.hint_since) < HINT_AFTER { return; }
    p.hinted = true;
    intro::say(host, player.uuid, "veio.onboarding.hint.head", &[]);
    intro::say(host, player.uuid, text, &[]);
}

/// Para onde ir na etapa: a casa (Lar e centro de controle) nas duas últimas, o terreno da
/// oficina nas outras.
fn target(p: &Progress) -> Option<[f64; 3]> {
    if matches!(p.step, MEAL_STEP | CENTER_STEP) {
        return home::dashboard().map(|[x, y, z]| [f64::from(x) + 0.5, f64::from(y), f64::from(z) + 0.5]);
    }
    p.plot.map(intro::plot_center)
}

fn spawn_safe(name: &str) -> bool { workforce::spawn::safe_name(name) }

#[cfg(test)]
pub(super) fn observe(dimension: &str, pos: Pos, entity: &ModBlockEntity, before: i32, contract_before: i32, time: i64) {
    observe_inner(dimension, pos, entity, before, contract_before, time, None);
}

pub(super) fn observe_production(dimension: &str, pos: Pos, entity: &ModBlockEntity, before: i32, previous_cycles: i32, contract_before: i32, time: i64) {
    observe_inner(dimension, pos, entity, before, contract_before, time, Some(previous_cycles));
}

fn observe_inner(dimension: &str, pos: Pos, entity: &ModBlockEntity, before: i32, contract_before: i32, time: i64, previous_cycles: Option<i32>) {
    if !ACTIVE.load(Ordering::Relaxed) { return; }
    if !matches!(entity.id.as_str(), PRESS | ASSEMBLER | DRILL | DRILL_MK2 | production::DRILL_MK3 | HUB) { return; }
    if entity.id == HUB {
        if number(entity, "ContractStage") == contract_before { return; }
    } else if number(entity, "Produced") == before && time.rem_euclid(20) != 0 { return; }
    if let Ok(mut session) = SESSION.lock() && let Some(s) = session.as_mut() {
        for p in s.players.values_mut().filter(|p| p.dimension == dimension && p.step < DONE) {
            p.observe_counts(pos, entity, before, contract_before, previous_cycles);
        }
    }
}

/// Only accepted quantities reach this observer; refused/blocked transfers never
/// generate credit. Trace both raw input and the plates made from that input.
pub(super) fn transferred(host: &dyn Host, from: Pos, to: Pos, item: &str, count: u32) {
    if count == 0 || !FLOW_ACTIVE.load(Ordering::Relaxed) || !matches!(item, "minecraft:raw_copper" | PLATE) { return; }
    if let Ok(mut session) = SESSION.lock() && let Some(s) = session.as_mut() {
        record(s, host, from, to, item, count);
    }
}

pub(super) fn tracks_flow() -> bool { FLOW_ACTIVE.load(Ordering::Relaxed) }

fn is_storage(block: &str) -> bool {
    matches!(block, "minecraft:chest" | "minecraft:trapped_chest" | "minecraft:barrel" | "minecraft:hopper"
        | "veio:barn" | "veio:granary" | "veio:hauler_post")
}

fn record(s: &mut Session, host: &dyn Host, from: Pos, to: Pos, item: &str, count: u32) {
    let dimension = host.dimension();
    let mut chest = None;
        for p in s.players.values_mut().filter(|p| p.dimension == dimension && matches!(p.step, 5 | 6)) {
            let is_chest = if item == PLATE {
                if !(p.blocks[PRESS_SLOT] == Some(from) && p.output_credit > 0) && !p.finished.contains_key(&from) { continue; }
                *chest.get_or_insert_with(|| is_storage(&host.block(to)))
            } else { false };
            p.transfer(from, to, item, count, is_chest, SERVER_CLOCK.load(Ordering::Relaxed));
        }
}

/// `tick` fires in both the window and the integrated server. Never let the
/// client's empty player list clear the authoritative session or advance its clock.
pub(super) fn server_tick(host: &mut dyn Host) {
    if host.window() { intro::client_view(host); return; }
    let now = SERVER_CLOCK.fetch_add(1, Ordering::Relaxed).wrapping_add(1);
    transport::flush(host, None);
    tick(host, now);
}

fn tick(host: &mut dyn Host, now: u64) {
    if now % 20 != 0 { return; }
    let online = host.server_players();
    let Ok(mut slot) = SESSION.lock() else { return; };
    if !session(host, &mut slot) { return; }
    let s = slot.as_mut().unwrap();
    for player in &online {
        let p = s.players.entry(player.uuid).or_insert_with(|| load(&s.root, player.uuid));
        // Prólogo primeiro: casa, prefeito e terreno. A fábrica só começa depois dele.
        if p.intro < intro::INTRO_DONE {
            intro::tick(host, player, p, now);
            if p.dirty && (now % 100 == 0 || p.intro == intro::INTRO_DONE) && save(&s.root, player.uuid, p).is_err() && !p.save_warned {
                host.tell(player.uuid, Text::tr("veio.onboarding.save_failed", &[])); p.save_warned = true;
            }
            continue;
        }
        // Capítulo da vila: enquanto está aberto, ele fala e ocupa a barra.
        if city::step(p).is_some() {
            city::tick(host, player, p, defense::clock());
            if p.dirty && now % 100 == 0 && save(&s.root, player.uuid, p).is_err() && !p.save_warned {
                host.tell(player.uuid, Text::tr("veio.onboarding.save_failed", &[])); p.save_warned = true;
            }
            continue;
        }
        if p.village > 0 && p.festival < festival::DONE { festival::tick(host, player, p); }
        if let Some(step) = village::step(p) {
            village::tick(host, player, p, now);
            if village::step(p) == Some(step) { hint(host, player, p, now, 100 + step as u16, &format!("veio.village.hint.{step}")); }
            else if p.village == village::DONE_VILLAGE { fireworks(host, &player.name); }
            if p.dirty && now % 100 == 0 && save(&s.root, player.uuid, p).is_err() && !p.save_warned {
                host.tell(player.uuid, Text::tr("veio.onboarding.save_failed", &[])); p.save_warned = true;
            }
            continue;
        }
        if !p.hidden && (BOARD_STEP..DONE).contains(&p.step) { look(host, player, p); }
        // Exactly one transition per update. Never skip an unperformed exercise.
        if (player.dimension == p.dimension || p.step >= BOARD_STEP) && p.ready() {
            p.step += 1; p.dirty = true; p.status = 0; p.last_status_notice = None;
            celebrate(host, player, p.step);
            if p.step == DONE { host.action_bar(player.uuid, Text::literal("")); }
        }
        if !p.hidden && p.announced != Some(p.step) {
            announce(host, player, p);
            p.announced = Some(p.step);
        }
        if !p.hidden && p.pending_tips != 0 {
            for (bit, key) in TIPS {
                if p.pending_tips & bit != 0 && p.tips & bit == 0 {
                    host.tell(player.uuid, Text::tr(&format!("veio.onboarding.tip.{key}"), &[]));
                    p.tips |= bit;
                    p.dirty = true;
                }
            }
            p.pending_tips = 0;
        }
        if !p.hidden && p.step == 1 && p.favor < favors::FAVOR_DONE {
            favors::tick(host, player, p, now);
        } else if !p.hidden && p.step < DONE {
            let count = goal_count(p);
            if !checklist(host, player, p) {
                let goal = Text::tr(&format!("veio.onboarding.goal.{}", p.step), &[&count]);
                let far = target(p).map(|at| intro::pointer(player, at)).filter(|(distance, _)| *distance > FAR);
                host.action_bar(player.uuid, match far {
                    Some((distance, way)) => Text::tr("veio.onboarding.far", &[&goal.plain(), &way, &distance]),
                    None => goal,
                });
            }
            hint(host, player, p, now, u16::from(p.step), &format!("veio.onboarding.hint.{}", p.step));
            if matches!(p.status, 3 | 5 | 9) {
                if p.last_status_notice != Some(p.status) && now >= p.next_status_notice {
                    host.tell(player.uuid, Text::tr(&format!("veio.onboarding.blocked.{}", p.status), &[]));
                    p.last_status_notice = Some(p.status); p.next_status_notice = now + 200;
                }
            } else { p.last_status_notice = None; }
        }
        if !p.hidden && p.step == DONE && !p.outro { intro::outro_bar(host, player, p); }
        if p.dirty && (now % 100 == 0 || p.ready() || p.step == DONE) && save(&s.root, player.uuid, p).is_err() && !p.save_warned {
            host.tell(player.uuid, Text::tr("veio.onboarding.save_failed", &[])); p.save_warned = true;
        }
    }
    // Offline players must neither gain progress nor retain volatile flow tokens.
    s.players.retain(|uuid, p| {
        if online.iter().any(|player| player.uuid == *uuid) { true }
        else { if p.dirty { let _ = save(&s.root, *uuid, p); } false }
    });
    ACTIVE.store(s.players.values().any(|p| p.step < DONE), Ordering::Relaxed);
    FLOW_ACTIVE.store(s.players.values().any(|p| matches!(p.step, 5 | 6)), Ordering::Relaxed);
}

pub(super) fn logged_in(host: &dyn Host) {
    if let Ok(mut slot) = SESSION.lock() { session(host, &mut slot); }
}

pub(super) fn logout(_host: &dyn Host, uuid: u128) {
    let Ok(mut slot) = SESSION.lock() else { return; };
    // The engine may already have detached world_dir while shutting down.
    // The session retains the verified save root for this final flush.
    let Some(s) = slot.as_mut() else { return; };
    if let Some(p) = s.players.get_mut(&uuid) && p.dirty { let _ = save(&s.root, uuid, p); }
    s.players.remove(&uuid);
}

pub(super) fn command(host: &mut dyn Host, player: &NativeServerPlayer, command: &str) -> bool {
    let action = match command.trim() {
        "veiotutorial ocultar" => 0, "veiotutorial mostrar" => 1, "veiotutorial reiniciar" => 2, "veiotutorial vila" => 3, "veiotutorial cidade" => 4,
        _ => return false,
    };
    let Ok(mut slot) = SESSION.lock() else { return true; };
    if !session(host, &mut slot) { return true; }
    let s = slot.as_mut().unwrap();
    let p = s.players.entry(player.uuid).or_insert_with(|| load(&s.root, player.uuid));
    if action >= 3 {
        // Capítulo da vila ou da cidade direto (quem já conhece o anterior): pula o prólogo.
        if p.intro < intro::INTRO_DONE { p.intro = intro::INTRO_DONE; }
        if action == 3 { village::start(host, player, p); }
        else {
            // A cidade fecha o capítulo da vila, se estava aberto, para os dois não disputarem a barra.
            if village::step(p).is_some() { p.village = village::DONE_VILLAGE; }
            city::start(host, player, p);
        }
        if save(&s.root, player.uuid, p).is_err() { host.tell(player.uuid, Text::tr("veio.onboarding.save_failed", &[])); }
        ACTIVE.store(true, Ordering::Relaxed);
        return true;
    }
    match action { 0 => { p.hidden = true; }, 1 => { p.hidden = false; p.announced = None; p.village_announced = None; p.city_announced = None; },
        // Refazer não repete as dicas já vistas.
        _ => { *p = Progress { tips: p.tips, ..Progress::default() }; } }
    if action == 1 { p.intro_announced = None; }
    p.dirty = true;
    if save(&s.root, player.uuid, p).is_err() { host.tell(player.uuid, Text::tr("veio.onboarding.save_failed", &[])); }
    host.tell(player.uuid, Text::tr(if action == 0 { "veio.onboarding.hidden" } else { "veio.onboarding.shown" }, &[]));
    if action == 0 { host.action_bar(player.uuid, Text::literal("")); }
    ACTIVE.store(true, Ordering::Relaxed);
    true
}

/// Clique no prefeito (`intro::MAYOR`): a fala do momento para quem clicou. `true` se o Veio
/// tomou o clique.
pub(super) fn talk(host: &mut dyn Host) -> bool {
    let Some(player) = host.acting_player() else { return true };
    let Ok(mut slot) = SESSION.lock() else { return true };
    if !session(host, &mut slot) { return true; }
    let s = slot.as_mut().unwrap();
    let p = s.players.entry(player.uuid).or_insert_with(|| load(&s.root, player.uuid));
    intro::talk(host, &player, p);
    if p.dirty && save(&s.root, player.uuid, p).is_err() {
        host.tell(player.uuid, Text::tr("veio.onboarding.save_failed", &[]));
    }
    true
}

/// Clique no ferreiro ou na lavradora do favor. `true` se o Veio tomou o clique (senão a troca
/// vanilla segue).
pub(super) fn neighbor_talk(host: &mut dyn Host, kind: &str) -> bool {
    let Some(who) = favors::who(kind) else { return false };
    let Some(player) = host.acting_player() else { return true };
    let Ok(mut slot) = SESSION.lock() else { return true };
    if !session(host, &mut slot) { return true; }
    let s = slot.as_mut().unwrap();
    let p = s.players.entry(player.uuid).or_insert_with(|| load(&s.root, player.uuid));
    let took = favors::talk(host, &player, p, who);
    if p.dirty && save(&s.root, player.uuid, p).is_err() {
        host.tell(player.uuid, Text::tr("veio.onboarding.save_failed", &[]));
    }
    took
}

/// Outro mundo vai abrir: esquece o prefeito e os vizinhos do mundo anterior.
pub(super) fn world_changed() { intro::reset(); favors::reset(); }

mod intro;
mod festival;
mod favors;
pub(super) use favors::{seen as neighbor_seen, FARMER, SMITH};
mod barks;
pub(super) use barks::villager_seen;
mod village;
mod city;
pub(super) use intro::{mayor_seen, visuals, MAYOR};
#[cfg(test)]
mod tests;
mod transport;
pub(super) use transport::{before_machine, before_transfer, hopper_attempt, move_items, worker_delivery, worker_pickup, worker_unload};
