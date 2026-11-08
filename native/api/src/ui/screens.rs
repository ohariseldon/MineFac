//! As telas do Veio montadas com o kit: onde fica cada slot e como cada tipo de palco é
//! desenhado. Todas têm o mesmo esqueleto, de cima para baixo:
//!
//! - cabeçalho: o bloco num quadro com a cor da família, o nome e uma linha do que ele faz;
//! - faixa de estado: lâmpada, o estado em palavras e os equipamentos (Mk.II, módulo, fumaça);
//! - palco: o processo desenhado (entradas → máquina → saída), com ícones-fantasma nos slots
//!   vazios, taxas por minuto, barra de energia à direita e cenas animadas pelo relógio do mundo;
//! - cartões: os números que importam, cada um com ícone, valor e nome (a dica explica);
//! - rodapé: como e por quê, ou o conselho quando algo trava.
use super::*;
use super::spec::{Kind, Spec};
use horizon_client::mod_api::{ScreenDrawEvent, ScreenTooltipEvent};

/// Índice da máscara de slots vazios nos dados do menu (`presentation::publish`).
pub(crate) const EMPTY: usize = 13;
/// Linha dos slots no palco.
const ROW: f32 = 63.0;
/// Barra de energia e tanque, na borda direita do palco.
const ENERGY: [f32; 4] = [176.0, 60.0, 8.0, 37.0];
const TANK: [f32; 4] = [165.0, 60.0, 8.0, 37.0];
/// Centro das composições do palco (à esquerda da barra de energia) e sem ela.
const CENTER: f32 = 88.0;
const MIDDLE: f32 = 97.0;
const STATUS: [f32; 4] = [6.0, STATUS_Y, 182.0, 12.0];
const CARD_H: f32 = 20.0;

/// O que a tela sabe do bloco: dados do menu e a ficha (`spec`).
pub(crate) struct View<'a> {
    pub block: &'a str,
    pub name: &'a str,
    pub data: &'a [i32],
    pub spec: Spec,
}

impl<'a> View<'a> {
    pub(crate) fn new(block: &'a str, data: &'a [i32]) -> Option<Self> {
        let spec = spec::of(block)?;
        Some(Self { block, name: block.strip_prefix("veio:").unwrap_or(block), data, spec })
    }

    pub(crate) fn read(&self, index: usize) -> i32 { self.data.get(index).copied().unwrap_or(0) }

    pub(crate) fn status(&self) -> usize {
        (self.read(0).max(0) as usize).min(presentation::STATUS_KEYS.len() - 1)
    }

    fn working(&self) -> bool { self.read(0) == 1 }

    /// O slot está vazio? Sem a máscara (dados ainda não chegaram), diz que não: nada de fantasma
    /// por baixo de um item de verdade.
    pub(crate) fn empty(&self, slot: usize) -> bool {
        slot < 15 && self.data.get(EMPTY).is_some_and(|mask| mask >> slot & 1 == 1)
    }

    fn accent(&self) -> u32 { self.spec.family.accent() }

    fn progress(&self) -> f32 { self.read(1).max(0) as f32 / self.read(2).max(1) as f32 }

    fn powered(&self) -> bool { needs_power(self.block) || is_power_block(self.block) }

    fn energy(&self) -> (i32, i32) {
        let limit = self.read(5).max(capacity(self.block)).max(1);
        (self.read(3).clamp(0, limit), limit)
    }

    fn effect(&self) -> upgrades::Effect {
        let module = self.read(11) & !upgrades::MK2_FLAG;
        upgrades::with_tier(upgrades::effect_of(module), if self.read(11) & upgrades::MK2_FLAG != 0 { 2 } else { 0 })
    }

    /// Algo trava (falta insumo, energia, trabalhador...): o rodapé troca a explicação pelo
    /// conselho. Esperas normais (noite, vigia, a caminho) não contam.
    fn problem(&self) -> bool {
        matches!(self.status(), 2..=6 | 9 | 12..=19 | 21 | 22 | 25 | 30..=35 | 37 | 40..=43)
    }

    fn key(&self, group: &str) -> String { format!("veio.ui.{group}.{}", self.name) }
}

// ---------- Posições dos slots ----------

/// Linha de produção centrada em `center`: entradas, seta e o slot grande do resultado.
struct Line {
    inputs: Vec<[f32; 2]>,
    arrow: [f32; 2],
    output: [f32; 2],
}

fn line(inputs: usize, center: f32) -> Line {
    let n = inputs as f32;
    let x0 = (center - (n * 18.0 + 62.0) / 2.0).round();
    let arrow = [x0 + n * 18.0 + 6.0, ROW + 1.0];
    Line {
        inputs: (0..inputs).map(|index| [x0 + 1.0 + index as f32 * 18.0, ROW]).collect(),
        arrow,
        output: [arrow[0] + 35.0, ROW],
    }
}

/// Fileira de `count` slots começando em `x`.
fn row(x: f32, y: f32, count: usize) -> impl Iterator<Item = Option<[f32; 2]>> {
    (0..count).map(move |index| Some([x + index as f32 * 18.0, y]))
}

/// Extratora: a jazida, a seta e o resultado.
const DEPOSIT: [f32; 4] = [34.0, 50.0, 44.0, 42.0];
const EXTRACT_ARROW: [f32; 2] = [86.0, ROW + 1.0];
const EXTRACT_OUT: [f32; 2] = [121.0, ROW];
/// Queimadores: chama em cima, combustível embaixo, seta e o que sai.
const FUEL: [f32; 2] = [36.0, 74.0];
const FLAME: [f32; 2] = [38.0, 54.0];
const BURN_ARROW: [f32; 2] = [62.0, ROW + 1.0];
const BURN_OUT: [f32; 2] = [92.0, ROW];
/// Postos e terminais: grupos de três.
const TERMINAL_Y: f32 = 58.0;

/// Central de contratos: duas entregas, a seta e as duas recompensas.
const CONTRACT_SLOTS: [[f32; 2]; 4] = [[20.0, ROW], [66.0, ROW], [120.0, ROW], [142.0, ROW]];
const CONTRACT_ARROW: f32 = 90.0;
/// Quadro: a entrega, a seta e a recompensa no slot grande; à direita, o que a fase libera.
const BOARD_IN: [f32; 2] = [46.0, ROW];
const BOARD_ARROW: f32 = 70.0;
const BOARD_OUT: [f32; 2] = [104.0, ROW];

pub(crate) fn slots(block: &str) -> Option<Vec<Option<[f32; 2]>>> {
    use production::*;
    let spec = spec::of(block)?;
    let hidden = || vec![None];
    Some(match spec.kind {
        Kind::Process => {
            let layout = line(output_slots(block).start, CENTER);
            layout.inputs.into_iter().map(Some).chain([Some(layout.output)]).collect()
        }
        Kind::Extract => vec![Some(EXTRACT_OUT)],
        Kind::Burner if block == GEOTHERMAL => vec![Some(FUEL), Some([BURN_OUT[0], 79.0])],
        Kind::Burner => vec![Some(FUEL)],
        Kind::Passive | Kind::Battery | Kind::Arm | Kind::Tesla | Kind::Anchor => hidden(),
        Kind::Conveyor => vec![Some([89.0, ROW])],
        Kind::Router if block == MERGER => vec![Some([52.0, 52.0]), Some([52.0, 75.0])],
        Kind::Router => vec![Some([52.0, ROW])],
        Kind::Grow => match block {
            PLANTER | SPRINKLER => vec![Some([34.0, ROW])],
            HARVESTER | TREE_FARM => std::iter::once(Some([18.0, ROW])).chain(row(120.0, ROW, 3)).collect(),
            _ => hidden(),
        },
        Kind::Ranch => vec![Some([18.0, 53.0]), Some([18.0, 75.0]), Some([120.0, ROW]), Some([138.0, ROW]), Some([156.0, ROW])],
        Kind::Storage => {
            let count = workforce::storage::slots(block);
            let top = if count > 18 { 46.0 } else { 55.0 };
            (0..count).map(|index| Some([INVENTORY_X + (index % 9) as f32 * 18.0, top + (index / 9) as f32 * 18.0])).collect()
        }
        Kind::Post => match block {
            workforce::FARM_POST => std::iter::once(Some([40.0, ROW])).chain(row(102.0, ROW, 3)).collect(),
            workforce::HAULER_POST => std::iter::once(Some([18.0, ROW])).chain(row(44.0, ROW, 3)).chain(row(130.0, ROW, 3)).collect(),
            workforce::FOREMAN_DESK => row(24.0, ROW, 3).chain([Some([155.0, ROW])]).collect(),
            workforce::SHEPHERD_POST => vec![Some([30.0, ROW]), Some([52.0, ROW])].into_iter().chain(row(112.0, ROW, 3)).collect(),
            workforce::BARRACKS => vec![Some([40.0, ROW]), Some([140.0, ROW])],
            workforce::SITE => vec![Some([24.0, ROW]), Some([54.0, ROW]), Some([104.0, ROW])],
            _ => return None,
        },
        Kind::Zone => vec![Some([24.0, ROW])],
        Kind::Port => row(INVENTORY_X, 82.0, workforce::society::trade::SLOTS).collect(),
        Kind::Terminal => {
            let mut slots: Vec<_> = row(11.0, TERMINAL_Y, 3).chain(row(120.0, TERMINAL_Y, 3)).collect();
            if block != freight::RAIL { slots.push(Some([84.0, 80.0])); }
            if block == freight::caravan::ID { slots.push(Some([29.0, 80.0])); }
            slots
        }
        Kind::Station => row(if block == stations::SENDER { STATION_SEND } else { STATION_RECEIVE }, ROW, 4).collect(),
        Kind::Dwelling => match block {
            workforce::society::HOUSE => row(104.0, ROW, 3).collect(),
            workforce::SHOP => std::iter::once(Some([13.0, ROW])).chain(row(41.0, ROW, 6)).chain([Some([163.0, ROW])]).collect(),
            _ => vec![Some([46.0, ROW])],
        },
        Kind::Pantry => {
            let layout = line(1, MIDDLE);
            vec![Some(layout.inputs[0]), Some(layout.output)]
        }
        Kind::Lab => vec![Some([16.0, ROW]), Some([56.0, ROW])],
        Kind::Contract => CONTRACT_SLOTS.iter().copied().map(Some).collect(),
        Kind::Board => vec![Some(BOARD_IN), Some(BOARD_OUT)],
        Kind::Custom => return None,
    })
}

// ---------- Desenho ----------

/// Desenha a tela inteira. Devolve falso para blocos fora do kit.
pub(crate) fn draw(host: &dyn Host, event: &mut ScreenDrawEvent<'_>) -> bool {
    let Some(view) = View::new(event.block, event.data) else { return false; };
    if view.spec.kind == Kind::Custom { return false; }
    let Some(slots) = slots(event.block) else { return false; };
    let mut pen = Pen::from_event(host, event);
    frame(&mut pen);
    header(&mut pen, &view, true);
    status_strip(&mut pen, &view);
    backdrop(&mut pen, &view);
    for slot in slots.iter().flatten() {
        pen.slot(slot[0], slot[1]);
    }
    stage(&mut pen, &view, &slots);
    if view.powered() { energy_bar(&mut pen, &view); }
    let cards = cards(&view);
    draw_cards(&mut pen, &cards);
    footer(&mut pen, &view);
    inventory(&mut pen);
    true
}

/// Fundo do painel e a moldura preta de cantos recortados, como as telas de contêiner.
pub(crate) fn frame(pen: &mut Pen<'_>) {
    pen.fill(1.0, 0.0, WIDTH - 2.0, HEIGHT, EDGE);
    pen.fill(0.0, 1.0, WIDTH, HEIGHT - 2.0, EDGE);
    pen.bevel(1.0, 1.0, WIDTH - 2.0, HEIGHT - 2.0, true, PANEL);
}

/// Inventário do jogador com o rótulo vanilla.
pub(crate) fn inventory(pen: &mut Pen<'_>) {
    pen.text(&tr("container.inventory", &[]), INVENTORY_X, INVENTORY_Y - 11.0, INK);
    for row in 0..3 {
        for column in 0..9 {
            pen.slot(INVENTORY_X + column as f32 * 18.0, INVENTORY_Y + row as f32 * 18.0);
        }
    }
    for column in 0..9 {
        pen.slot(INVENTORY_X + column as f32 * 18.0, INVENTORY_Y + 58.0);
    }
}

/// Cabeçalho: o bloco num quadro da cor da família, o nome, a legenda e o botão de ajuda. A
/// versão compacta (telas próprias) tem uma linha só.
pub(crate) fn header(pen: &mut Pen<'_>, view: &View<'_>, full: bool) {
    let accent = view.accent();
    let title = item_name(view.block);
    help_button(pen);
    // Como as telas vanilla: o item e o nome, sem moldura; embaixo, em cinza, o que a máquina faz.
    let _ = accent;
    if full {
        pen.item(view.block, 7.0, 6.0, 16.0);
        pen.text(&pen.fit(&title, 144.0), 28.0, 6.0, INK);
        let tag = tr_opt(&view.key("tag"), &[]).unwrap_or_else(|| tr(view.spec.family.key(), &[]));
        let tag = pen.fit(&tag, 146.0);
        pen.text(&tag, 28.0, 16.0, SOFT);
    } else {
        pen.item(view.block, 6.0, 4.0, 12.0);
        pen.text(&pen.fit(&title, 150.0), 21.0, 6.0, INK);
    }
}

pub(crate) fn help_button(pen: &mut Pen<'_>) {
    let [x, y, w, h] = HELP;
    let hover = pen.hovering(x, y, w, h);
    pen.button(x, y, w, h, "", hover);
    if !pen.icon(icons::INFO, x + 1.5, y + 1.5) {
        pen.text("?", x + 4.0, y + 2.0, INK);
    }
}

/// Linha das telas próprias: ícone numa coluna à esquerda e o texto ajustado à largura.
pub(crate) fn info_row(pen: &mut Pen<'_>, icon: Icon, text: &str, x: f32, y: f32, width: f32, rgb: u32) {
    pen.icon_at_center(icon, x + 6.0, y + 3.5);
    let text = pen.fit(text, width - 16.0);
    pen.text(&text, x + 14.0, y, rgb);
}

/// Cor do estado: a do texto vanilla escurecido e a do fundo da faixa.
fn tone(status: usize) -> u32 { presentation::text_color(status) }

pub(crate) fn status_strip(pen: &mut Pen<'_>, view: &View<'_>) {
    let status = view.status();
    let color = tone(status);
    let [x, y, w, _] = STATUS;
    pen.lamp(x + 3.0, y + 2.5, presentation::lamp_color(status));
    let chips = chips(view);
    let mut right = x + w - 3.0;
    for (text, rgb, icon) in chips.iter().rev() {
        let width = pen.width(text);
        right -= width;
        pen.text(text, right, y + 2.0, *rgb);
        if let Some(icon) = icon {
            right -= icon.w + 2.0;
            pen.icon(*icon, right, y + 6.0 - (icon.h / 2.0).round());
        }
        right -= 5.0;
    }
    let (label, color) = match view.spec.kind {
        // O quadro diz em que fase a fábrica está; o estado da máquina não interessa ali.
        Kind::Board => match milestones::PHASES.get(view.read(1).max(0) as usize) {
            Some(current) => (tr("veio.board.phase", &[&(view.read(1) + 1), &milestones::PHASES.len(),
                &tr(&format!("veio.board.phase.{}", current.key), &[])]), INK),
            None => (tr("veio.ui.board.done", &[]), GOOD),
        },
        _ => (tr(&format!("veio.status.{}", presentation::STATUS_KEYS[status]), &[]), color),
    };
    let label = pen.fit(&label, right - (x + 13.0) - 2.0);
    pen.text(&label, x + 13.0, y + 2.0, color);
}

/// Equipamento da máquina, à direita da faixa: Mk.II, módulo, fumaça e integridade.
fn chips(view: &View<'_>) -> Vec<(String, u32, Option<Icon>)> {
    let mut chips = Vec::new();
    if !matches!(view.spec.kind, Kind::Process | Kind::Extract | Kind::Burner | Kind::Grow | Kind::Ranch | Kind::Tesla | Kind::Lab) {
        return chips;
    }
    let module = view.read(11) & !upgrades::MK2_FLAG;
    if view.read(11) & upgrades::MK2_FLAG != 0 { chips.push((tr("veio.mk2", &[]), 0x6a50b8, None)); }
    if (1..=3).contains(&module) {
        let name = ["speed", "efficiency", "productivity"][module as usize - 1];
        chips.push((tr(&format!("veio.ui.module.{name}"), &[]), [0x2f5fd0, 0x2a8a3a, 0xc0661a][module as usize - 1], Some(icons::GEAR)));
    }
    if view.read(10) > 0 { chips.push((format!("{}%", (100 - view.read(10)).max(0)), BAD, Some(icons::HEART))); }
    chips
}

fn footer(pen: &mut Pen<'_>, view: &View<'_>) {
    let (text, color) = if view.problem() {
        (tr(&operations::advice_key(view.status() as i32), &[]), tone(view.status()))
    } else if view.spec.kind == Kind::Board {
        // Primeira fase: a lista de montagem. Depois, o próximo gargalo; no fim, os pedidos.
        match milestones::PHASES.get(view.read(1).max(0) as usize) {
            Some(_) if view.read(1) <= 0 => return board_setup(pen, view),
            Some(current) => (tr(&format!("veio.board.hint.{}", current.key), &[]), SOFT),
            None => (tr("veio.board.done", &[]), SOFT),
        }
    } else if view.spec.kind == Kind::Contract && contract_of(view).2 {
        (tr("veio.contract.optional_beacon", &[]), SOFT)
    } else {
        (tr_opt(&view.key("how"), &[]).unwrap_or_default(), SOFT)
    };
    pen.paragraph(&text, 7.0, FOOTER_Y, 180.0, 2, color);
}

// ---------- Palcos ----------

/// O que fica por baixo dos slots (a esteira sob o item).
fn backdrop(pen: &mut Pen<'_>, view: &View<'_>) {
    if view.spec.kind == Kind::Conveyor && view.block != lift::LIFT {
        let speed = match view.block { EXPRESS_BELT => 4, FAST_BELT => 8, _ => 16 };
        if view.block == TUNNEL {
            belt_strip(pen, ROW + 2.0, 12.0, 60.0, view.working(), speed);
            belt_strip(pen, ROW + 2.0, 118.0, 166.0, view.working(), speed);
            pen.fill(60.0, ROW - 4.0, 58.0, 24.0, 0xff5a5a5a);
            pen.fill(62.0, ROW - 2.0, 54.0, 20.0, 0xff2a2a2a);
        } else {
            belt_strip(pen, ROW + 2.0, 12.0, 166.0, view.working(), speed);
        }
    }
}

fn stage(pen: &mut Pen<'_>, view: &View<'_>, slots: &[Option<[f32; 2]>]) {
    match view.spec.kind {
        Kind::Process => process(pen, view),
        Kind::Extract => extract(pen, view),
        Kind::Burner => burner(pen, view),
        Kind::Passive => passive(pen, view),
        Kind::Battery => battery(pen, view),
        Kind::Conveyor => conveyor(pen, view),
        Kind::Router => router(pen, view),
        Kind::Arm => arm(pen, view),
        Kind::Grow => grow(pen, view, slots),
        Kind::Ranch => ranch(pen, view, slots),
        Kind::Storage => storage(pen, view, slots),
        Kind::Post => post(pen, view, slots),
        Kind::Zone => zone(pen, view),
        Kind::Port => port(pen, view),
        Kind::Terminal => terminal(pen, view, slots),
        Kind::Station => station(pen, view, slots),
        Kind::Dwelling => dwelling(pen, view, slots),
        Kind::Pantry => pantry(pen, view),
        Kind::Tesla => tesla(pen, view),
        Kind::Anchor => anchor(pen, view),
        Kind::Lab => lab(pen, view, slots),
        Kind::Contract => contract(pen, view),
        Kind::Board => board(pen, view),
        Kind::Custom => {}
    }
}

/// Contrato em vigor e a fase do grande projeto, se for uma.
fn contract_of(view: &View<'_>) -> (&'static contracts::Contract, Option<usize>, bool) {
    let stage = view.read(8).max(0) as usize;
    let optional_export = stage == 3 && view.read(12) == 0;
    let deal = if optional_export { &contracts::EXPORT } else { contracts::for_stage(stage) };
    (deal, contracts::phase(stage), optional_export)
}

/// Central de contratos, como a mesa de ferraria: o que o contrato pede aparece apagado nos
/// slots da esquerda, com a quantidade; a recompensa, apagada nos da direita.
fn contract(pen: &mut Pen<'_>, view: &View<'_>) {
    let (deal, phase, _) = contract_of(view);
    let title = pen.fit(&tr(deal.title, &[]), 176.0);
    pen.text_center(&title, CENTER, STAGE_Y + 3.0, INK);
    let (mut got_total, mut need_total) = (0, 0);
    for (index, &(item, count)) in deal.inputs.iter().enumerate() {
        let [x, y] = CONTRACT_SLOTS[index];
        if view.empty(index) {
            pen.ghost(item, x, y);
            if count > 1 && phase.is_none() { ghost_count(pen, count, x, y); }
        }
        need_total += count as i32;
        // No grande projeto as peças entram aos poucos: quanto já foi de quanto.
        if phase.is_some() {
            let got = view.read(if index == 0 { 9 } else { 12 }).clamp(0, count as i32);
            got_total += got;
            caption(pen, &format!("{got}/{count}"), x + 8.0, y + 19.0);
        }
    }
    let fraction = if phase.is_some() { got_total as f32 / need_total.max(1) as f32 }
        else if view.working() { (pen.tick % 40) as f32 / 40.0 } else { 0.0 };
    pen.arrow(CONTRACT_ARROW, ROW + 1.0, fraction);
    for (index, &(item, count)) in deal.rewards.iter().enumerate() {
        let [x, y] = CONTRACT_SLOTS[2 + index];
        if view.empty(2 + index) {
            pen.ghost(item, x, y);
            if count > 1 { ghost_count(pen, count, x, y); }
        }
    }
}

fn contract_cards(view: &View<'_>) -> Vec<Card> {
    let stage = view.read(8).max(0) as usize;
    let (_, phase, _) = contract_of(view);
    let total = contracts::CONTRACTS.len();
    if let Some(phase) = phase {
        let count = contracts::PHASES.len();
        vec![card(icons::STAR, format!("{}/{count}", phase + 1), "phase", tr("veio.project.stage", &[&(phase + 1), &count]))]
    } else if stage < total {
        vec![card(icons::SCROLL, format!("{}/{total}", stage + 1), "contract", tr("veio.contract.stage", &[&(stage + 1), &total]))]
    } else {
        vec![card(icons::SHIP, thousands(view.read(9) as i64), "exports", tr("veio.contract.exports", &[&view.read(9)]))]
    }
}

/// O que o quadro pede agora: a entrega da fase ou, com as fases feitas, o pedido do dia.
fn board_goal(view: &View<'_>) -> (&'static str, i32, i32, (&'static str, u32), &'static [&'static str]) {
    match milestones::PHASES.get(view.read(1).max(0) as usize) {
        Some(current) => (current.item, current.count as i32, view.read(2).clamp(0, current.count as i32), current.reward, current.unlocks),
        None => {
            let order = board_order(view);
            (order.1, order.3 as i32, view.read(4).clamp(0, order.3 as i32), (EMERALD_ITEM, order.4), &[])
        }
    }
}

fn board_order(view: &View<'_>) -> (usize, &'static str, &'static str, u32, u32) {
    milestones::ORDERS[(view.read(3).max(0) as usize).min(milestones::ORDERS.len() - 1)]
}

/// Quadro da fábrica: a peça da fase entra, a recompensa sai e, à direita, as máquinas que a
/// fase libera, como a prévia da mesa de ferraria.
fn board(pen: &mut Pen<'_>, view: &View<'_>) {
    let (item, count, done, (reward, amount), unlocks) = board_goal(view);
    let [x, y] = BOARD_IN;
    if view.empty(0) { pen.ghost(item, x, y); }
    caption(pen, &format!("{done}/{count}"), x + 8.0, y + 19.0);
    pen.arrow(BOARD_ARROW, ROW + 1.0, done as f32 / count.max(1) as f32);
    let [ox, oy] = BOARD_OUT;
    pen.result_slot(ox, oy, view.accent());
    if view.empty(1) {
        pen.ghost(reward, ox, oy);
        if amount > 1 { ghost_count(pen, amount, ox, oy); }
    }
    if unlocks.is_empty() { return; }
    let shown = unlocks.len().min(3);
    let left = 136.0;
    caption(pen, &tr("veio.ui.label.unlocks", &[]), left + shown as f32 * 17.0 / 2.0, ROW - 11.0);
    for (index, key) in unlocks.iter().take(shown).enumerate() {
        // "block.veio.assembler" -> "veio:assembler"
        let id = key.split_once('.').map_or(*key, |(_, rest)| rest).replacen('.', ":", 1);
        pen.item(&id, left + index as f32 * 17.0, y, 16.0);
    }
}

fn board_cards(view: &View<'_>) -> Vec<Card> {
    let order = board_order(view);
    let done = view.read(4).clamp(0, order.3 as i32);
    let order_text = tr("veio.board.order", &[&view.read(6).max(1), &done, &order.3, &tr(order.2, &[])]);
    let mut cards = Vec::new();
    if (view.read(1).max(0) as usize) < milestones::PHASES.len() {
        cards.push(Card { item: Some(order.1), ..card(icons::SCROLL, format!("{done}/{}", order.3), "order", order_text) });
    }
    let (paid_key, rgb) = match view.read(5) { 1 => ("veio.board.order_paid", GOOD), 2 => ("veio.board.order_elsewhere", FAINT), _ => ("veio.board.order_reward", INK) };
    cards.push(Card { rgb, ..card(icons::EMERALD, format!("{}", order.4), "prize", tr(paid_key, &[&order.4])) });
    let (stuck, total) = (view.read(8) + view.read(9) + view.read(10), view.read(12));
    let hint = if total == 0 { tr("veio.board.no_machines", &[]) }
        else if stuck == 0 { tr("veio.board.ok", &[&view.read(7), &total]) }
        else { tr("veio.board.issues", &[&view.read(8), &view.read(9), &view.read(10)]) };
    cards.push(Card { rgb: if stuck > 0 { BAD } else { INK }, ..card(icons::FACTORY, format!("{}/{}", view.read(7), total), "machines", hint) });
    cards
}

/// Primeira hora: o que já existe perto do quadro, marcado com um visto.
fn board_setup(pen: &mut Pen<'_>, view: &View<'_>) {
    let setup = view.read(11);
    let mut x = 7.0;
    let mut y = FOOTER_Y;
    for (bit, key) in ["veio.board.setup.power", "veio.board.setup.drill", "veio.board.setup.press", "veio.board.setup.assembler"].iter().enumerate() {
        let label = tr(key, &[]);
        let width = 10.0 + pen.width(&label);
        if x > 7.0 && x + width > 187.0 { x = 7.0; y += 10.0; }
        let ready = setup & (1 << bit) != 0;
        pen.icon_at_center(if ready { icons::CHECK } else { icons::CROSS }, x + 4.0, y + 4.0);
        pen.text(&label, x + 10.0, y, if ready { GOOD } else { FAINT });
        x += width + 8.0;
    }
}

/// Alterna entre opções a cada segundo e meio, como os fantasmas da mesa de ferraria.
fn cycle<'b>(items: &[&'b str], tick: i64) -> Option<&'b str> {
    (!items.is_empty()).then(|| items[(tick.max(0) / 30) as usize % items.len()])
}

/// Ícone da ação sobre a seta: balança enquanto a máquina trabalha.
fn verb(pen: &mut Pen<'_>, view: &View<'_>, cx: f32, cy: f32) {
    let bob = if view.working() { ((pen.tick / 5) % 2) as f32 } else { 0.0 };
    pen.icon_at_center(view.spec.verb, cx, cy - bob);
}

/// Texto pequeno centrado, apagado.
fn caption(pen: &mut Pen<'_>, text: &str, cx: f32, y: f32) {
    pen.text_center(text, cx, y, SOFT);
}

fn seconds_text(ticks: i32) -> String {
    let tenths = (ticks.max(0) + 1) / 2;
    if tenths % 10 == 0 || tenths >= 100 { format!("{} s", (tenths + 5) / 10) } else { format!("{},{} s", tenths / 10, tenths % 10) }
}

fn per_minute(rate: f32) -> String { format!("{}/min", presentation::per_minute_text(rate)) }

/// Receita mostrada: a ativa ou, antes do primeiro ciclo, uma de cada vez.
fn shown_recipe(view: &View<'_>, tick: i64) -> Option<&'static production::Recipe> {
    let index = view.read(8);
    if index > 0 { return spec::recipe(view.block, index - 1); }
    let count = production::recipes(view.block).count();
    if count == 0 { return None; }
    spec::recipe(view.block, ((tick.max(0) / 60) as usize % count) as i32)
}

fn process(pen: &mut Pen<'_>, view: &View<'_>) {
    let inputs = output_slots(view.block).start;
    let layout = line(inputs, CENTER);
    let recipe = shown_recipe(view, pen.tick);
    let duration = if view.read(2) > 1 { view.read(2) } else { recipe.map_or(20, |recipe| recipe.ticks) };
    let cycles = 1200.0 / duration.max(1) as f32;
    let bonus = 1.0 + view.effect().bonus as f32 / 100.0;
    let accent = view.accent();
    for (index, at) in layout.inputs.iter().enumerate() {
        let Some(input) = recipe.and_then(|recipe| recipe.inputs.get(index)) else { continue; };
        if view.empty(index) {
            if let Some(item) = cycle(input.items, pen.tick) {
                pen.ghost(item, at[0], at[1]);
                if input.count > 1 { ghost_count(pen, input.count, at[0], at[1]); }
            }
        }
        let rate = input.count as f32 * cycles;
        let text = if inputs > 1 { presentation::per_minute_text(rate) } else { per_minute(rate) };
        caption(pen, &text, at[0] + 8.0, ROW - 10.0);
    }
    pen.result_slot(layout.output[0], layout.output[1], accent);
    if let Some(recipe) = recipe {
        if view.empty(inputs) { pen.ghost(recipe.output.0, layout.output[0], layout.output[1]); }
        let rate = per_minute(recipe.output.1 as f32 * cycles * bonus);
        pen.text_center(&rate, layout.output[0] + 8.0, ROW - 15.0, INK);
        consumers(pen, recipe.output.0, layout.output[0] + 8.0, ROW + 23.0);
    }
    let [ax, ay] = layout.arrow;
    verb(pen, view, ax + 11.0, ROW - 7.0);
    pen.arrow(ax, ay, view.progress());
    caption(pen, &seconds_text(duration), ax + 11.0, ROW + 19.0);
}

/// Quantas unidades a receita pede, no canto do fantasma.
fn ghost_count(pen: &mut Pen<'_>, count: u32, x: f32, y: f32) {
    let text = count.to_string();
    let width = pen.width(&text);
    pen.shadow_text(&text, x + 17.0 - width, y + 9.0, 0xc8c8c8);
}

/// "Vai para": até três máquinas que usam o produto.
fn consumers(pen: &mut Pen<'_>, item: &str, cx: f32, y: f32) {
    let machines = spec::consumers(item);
    if machines.is_empty() { return; }
    let shown = machines.len().min(3);
    let width = 9.0 + shown as f32 * 11.0;
    let x0 = (cx - width / 2.0).round();
    pen.text("→", x0 - 1.0, y + 1.0, FAINT);
    for (index, machine) in machines.iter().take(shown).enumerate() {
        pen.item(machine, x0 + 9.0 + index as f32 * 11.0, y, 10.0);
    }
}

/// Retângulos dos ícones de "vai para" (para a dica).
fn consumer_boxes(item: &str, cx: f32, y: f32) -> Vec<([f32; 4], &'static str)> {
    let machines = spec::consumers(item);
    let shown = machines.len().min(3);
    let x0 = (cx - (9.0 + shown as f32 * 11.0) / 2.0).round();
    machines.into_iter().take(shown).enumerate()
        .map(|(index, machine)| ([x0 + 9.0 + index as f32 * 11.0, y, 10.0, 10.0], machine)).collect()
}

fn extract(pen: &mut Pen<'_>, view: &View<'_>) {
    let [x, y, w, h] = DEPOSIT;
    let purity = view.read(12).clamp(0, 4);
    // Corte do terreno: terra em cima, pedra embaixo e os veios, mais densos quanto mais pura.
    pen.bevel(x, y, w, h, false, 0xff6f6f6f);
    pen.fill(x + 1.0, y + 1.0, w - 2.0, 8.0, 0xff7a5a3a);
    pen.fill(x + 1.0, y + 1.0, w - 2.0, 2.0, 0xff5f8a3a);
    let ore = [0xff9a9a9a, 0xffb07a4a, 0xffc8a060, 0xffe0c070, 0xfff0d880][purity as usize];
    let veins = [0, 3, 6, 9, 13][purity as usize];
    for index in 0..veins {
        let vx = x + 3.0 + ((index * 17) % 37) as f32;
        let vy = y + 12.0 + ((index * 11) % 26) as f32;
        pen.fill(vx, vy, 3.0, 2.0, ore);
        pen.fill(vx + 1.0, vy - 1.0, 1.0, 1.0, 0x60ffffff);
    }
    // A broca desce e sobe enquanto trabalha.
    let depth = if view.working() { ((pen.tick % 20) as f32 / 20.0 * 6.0).round() } else { 0.0 };
    let bit = x + w / 2.0;
    pen.fill(bit - 2.0, y - 4.0, 4.0, 10.0 + depth, 0xff3a3a3a);
    pen.quad([[bit - 3.0, y + 6.0 + depth], [bit + 3.0, y + 6.0 + depth], [bit + 1.0, y + 11.0 + depth], [bit - 1.0, y + 11.0 + depth]], 0xffd0d0d0);
    pen.arrow(EXTRACT_ARROW[0], EXTRACT_ARROW[1], view.progress());
    verb(pen, view, EXTRACT_ARROW[0] + 11.0, ROW - 7.0);
    pen.result_slot(EXTRACT_OUT[0], EXTRACT_OUT[1], view.accent());
    if view.read(6) > 0 {
        pen.text_center(&per_minute(view.read(6) as f32), EXTRACT_OUT[0] + 8.0, ROW - 15.0, INK);
    }
    caption(pen, &seconds_text(view.read(2)), EXTRACT_ARROW[0] + 11.0, ROW + 19.0);
}

fn burner(pen: &mut Pen<'_>, view: &View<'_>) {
    let burning = view.read(4) > 0;
    let burn = if view.read(2) > 0 { view.read(4).max(0) as f32 / view.read(2).max(1) as f32 } else { 0.0 };
    if view.empty(0) {
        if let Some(item) = view.spec.ghosts.first() { pen.ghost(item, FUEL[0], FUEL[1]); }
    }
    pen.burn(FLAME[0], FLAME[1], burn);
    pen.arrow(BURN_ARROW[0], BURN_ARROW[1], if burning { (pen.tick % 20) as f32 / 20.0 } else { 0.0 });
    verb(pen, view, BURN_ARROW[0] + 11.0, ROW - 7.0);
    let output = power_output(view);
    let color = if burning { INK } else { FAINT };
    let [ox, oy] = BURN_OUT;
    pen.icon(icons::BOLT, ox, oy - 4.0);
    pen.text(&format!("+{output} FE/t"), ox + 11.0, oy - 1.0, color);
    if view.block != GEOTHERMAL {
        let text = if burning { tr("veio.ui.burn.left", &[&seconds_text(view.read(4).max(0))]) } else { tr("veio.ui.burn.out", &[]) };
        let text = pen.fit(&text, TANK[0] - 4.0 - ox);
        pen.text(&text, ox, oy + 12.0, SOFT);
    }
    if production::water::holds(view.block) || view.block == GEOTHERMAL {
        tank(pen, view);
    }
}

/// FE/t que o bloco gera agora (zero quando parado).
fn power_output(view: &View<'_>) -> i32 {
    match view.block {
        GEOTHERMAL => if view.read(4) > 0 { 80 } else { 0 },
        _ => match production::generation::burner(view.block) {
            Some((rate, _)) => if view.read(4) > 0 { rate } else { 0 },
            None => view.read(12).max(0),
        },
    }
}

/// Tanque de água ou lava, ao lado da energia.
fn tank(pen: &mut Pen<'_>, view: &View<'_>) {
    let limit = production::water::capacity(view.block).max(1);
    let lava = view.read(8) < 0 || view.block == GEOTHERMAL;
    let stored = view.read(8).abs().clamp(0, limit);
    let [x, y, w, h] = TANK;
    pen.icon_at_center(if lava { icons::LAVA } else { icons::DROP }, x + w / 2.0, y - 7.0);
    vertical(pen, x, y, w, h, stored as f32 / limit as f32, if lava { 0xe0561a } else { 0x3f76e4 });
}

/// Barra vertical que enche de baixo para cima.
fn vertical(pen: &mut Pen<'_>, x: f32, y: f32, w: f32, h: f32, fraction: f32, rgb: u32) {
    pen.bevel(x, y, w, h, false, 0xff2b2b2b);
    let inner = ((h - 2.0) * fraction.clamp(0.0, 1.0)).round();
    if inner > 0.0 {
        pen.fill(x + 1.0, y + h - 1.0 - inner, w - 2.0, inner, solid(rgb));
        pen.fill(x + 1.0, y + h - 1.0 - inner, w - 2.0, 1.0, 0x60ffffff);
        pen.fill(x + w - 2.0, y + h - 1.0 - inner, 1.0, inner, 0x30000000);
    }
}

fn energy_bar(pen: &mut Pen<'_>, view: &View<'_>) {
    let (energy, limit) = view.energy();
    let [x, y, w, h] = ENERGY;
    pen.icon_at_center(icons::BOLT, x + w / 2.0, y - 7.0);
    vertical(pen, x, y, w, h, energy as f32 / limit as f32, 0xe0a020);
}

/// Céu do palco: azul de dia, escuro de noite, com chão embaixo.
fn sky(pen: &mut Pen<'_>, x: f32, y: f32, w: f32, h: f32, night: bool) {
    let (top, bottom) = if night { (0xff1a2340, 0xff2e3a60) } else { (0xff6fa8e8, 0xffb8d8f4) };
    let bands = 6;
    for band in 0..bands {
        let t = band as f32 / (bands - 1) as f32;
        let band_h = (h / bands as f32).ceil();
        pen.fill(x, y + band as f32 * band_h, w, band_h.min(y + h - (y + band as f32 * band_h)), mix(top, bottom, t));
    }
}

fn ground(pen: &mut Pen<'_>, x: f32, y: f32, w: f32) {
    pen.fill(x, y, w, 2.0, 0xff5f9a3a);
    pen.fill(x, y + 2.0, w, 6.0, 0xff7a5a3a);
}

/// Hélice de `blades` pás girando em volta de (`cx`, `cy`).
fn rotor(pen: &mut Pen<'_>, cx: f32, cy: f32, radius: f32, blades: usize, angle: f32, rgb: u32) {
    for blade in 0..blades {
        let a = angle + blade as f32 * std::f32::consts::TAU / blades as f32;
        let (s, c) = a.sin_cos();
        let (ps, pc) = (a + 1.5708).sin_cos();
        let tip = [cx + c * radius, cy + s * radius];
        pen.quad([[cx + pc * 1.2, cy + ps * 1.2], [tip[0] + pc * 1.6, tip[1] + ps * 1.6],
            [tip[0] - pc * 1.6, tip[1] - ps * 1.6], [cx - pc * 1.2, cy - ps * 1.2]], solid(rgb));
    }
    pen.fill(cx - 1.5, cy - 1.5, 3.0, 3.0, 0xff404040);
}

fn passive(pen: &mut Pen<'_>, view: &View<'_>) {
    use production::*;
    let output = view.read(12).max(0);
    let (max, label) = match view.block {
        SOLAR => (16, "veio.ui.max.solar"),
        WIND_TURBINE => (32, "veio.ui.max.wind"),
        WATER_WHEEL => (32, "veio.ui.max.water_wheel"),
        _ => (water::HYDRO_MAX, "veio.ui.max.hydro"),
    };
    let [x, y, w, h] = [12.0, 48.0, 76.0, 46.0];
    let night = view.block == SOLAR && output == 0 && view.status() == 7;
    sky(pen, x, y, w, h - 8.0, night);
    ground(pen, x, y + h - 8.0, w);
    let spin = (pen.tick as f32 + 0.0) * output as f32 * 0.004;
    match view.block {
        SOLAR => {
            let day = pen.day.rem_euclid(24000) as f32;
            let t = (day / 12000.0).min(1.0);
            let sx = x + 6.0 + t * (w - 12.0);
            let sy = y + 20.0 - (t * std::f32::consts::PI).sin() * 15.0;
            pen.icon_at_center(if night { icons::MOON } else { icons::SUN }, sx, sy);
            // Painel inclinado no chão.
            pen.quad([[x + 22.0, y + 37.0], [x + 54.0, y + 37.0], [x + 50.0, y + 29.0], [x + 26.0, y + 29.0]], 0xff24324a);
            pen.fill(x + 27.0, y + 31.0, 22.0, 1.0, 0xff5a7ab0);
            pen.fill(x + 25.0, y + 34.0, 26.0, 1.0, 0xff5a7ab0);
        }
        WIND_TURBINE => {
            pen.fill(x + 37.0, y + 12.0, 2.0, 26.0, 0xffe8e8e8);
            rotor(pen, x + 38.0, y + 12.0, 11.0, 3, spin, 0xf4f4f4);
            let drift = (pen.tick % 60) as f32 / 60.0 * w;
            for gust in 0..2 {
                let gx = x + (drift + gust as f32 * w / 2.0) % (w - 8.0);
                pen.fill(gx, y + 6.0 + gust as f32 * 14.0, 6.0, 1.0, 0x90ffffff);
            }
        }
        _ => {
            // Água correndo embaixo e a roda (ou a turbina) girando.
            pen.fill(x, y + h - 14.0, w, 6.0, 0xff3f76e4);
            for wave in 0..5 {
                let wx = x + ((pen.tick / 2 + wave * 17) % w as i64) as f32;
                pen.fill(wx, y + h - 13.0, 4.0, 1.0, 0x90ffffff);
            }
            pen.fill(x + 37.0, y + 30.0, 2.0, 2.0, 0xff5a4030);
            rotor(pen, x + 38.0, y + 24.0, 12.0, if view.block == WATER_WHEEL { 8 } else { 3 }, spin,
                if view.block == WATER_WHEEL { 0x8a6a3a } else { 0x9aa0a8 });
        }
    }
    // Quanto gera agora e o teto do bloco.
    let ox = 98.0;
    pen.icon(icons::BOLT, ox, 50.0);
    pen.text(&format!("+{output} FE/t"), ox + 11.0, 52.0, if output > 0 { INK } else { FAINT });
    pen.bar(ox, 64.0, 68.0, 7.0, output as f32 / max as f32, 0xe0a020);
    let lines = pen.paragraph(&tr(label, &[&max]), ox, 75.0, 70.0, 2, SOFT);
    if view.block == HYDRO_TURBINE && lines < 2 {
        pen.text(&pen.fit(&tr("veio.ui.head", &[&view.read(8)]), 70.0), ox, 84.0, SOFT);
    }
}

fn battery(pen: &mut Pen<'_>, view: &View<'_>) {
    let (energy, limit) = view.energy();
    let fraction = energy as f32 / limit as f32;
    let [x, y, w, h] = [30.0, 54.0, 112.0, 32.0];
    // Pilha grande deitada: corpo, polo e a carga em segmentos.
    pen.fill(x - 1.0, y - 1.0, w + 2.0, h + 2.0, EDGE);
    pen.bevel(x, y, w, h, false, 0xff2b2b2b);
    pen.fill(x + w + 1.0, y + 10.0, 5.0, 12.0, EDGE);
    pen.fill(x + w + 1.0, y + 11.0, 4.0, 10.0, 0xff8a8a8a);
    let segments = 10;
    let lit = (fraction * segments as f32).ceil() as i32;
    for segment in 0..segments {
        let sx = x + 3.0 + segment as f32 * ((w - 6.0) / segments as f32);
        let color = if segment < lit { mix(0xff3fbf5f, 0xffe0c040, 1.0 - fraction) } else { 0xff3a3a3a };
        pen.fill(sx, y + 3.0, (w - 6.0) / segments as f32 - 2.0, h - 6.0, color);
        if segment < lit { pen.fill(sx, y + 3.0, (w - 6.0) / segments as f32 - 2.0, 2.0, 0x50ffffff); }
    }
    let percent = format!("{}%", (fraction * 100.0).round() as i32);
    let width = pen.width(&percent);
    pen.fill(x + (w - width) / 2.0 - 3.0, y + 11.0, width + 6.0, 11.0, 0xc0000000);
    pen.shadow_text(&percent, (x + (w - width) / 2.0).round(), y + 13.0, 0xffffff);
}

/// Esteira desenhada atravessando o palco, com as ripas andando.
fn belt_strip(pen: &mut Pen<'_>, y: f32, x0: f32, x1: f32, moving: bool, speed: i64) {
    pen.fill(x0, y, x1 - x0, 12.0, 0xff3a3a3a);
    pen.fill(x0, y, x1 - x0, 1.0, 0xff6a6a6a);
    pen.fill(x0, y + 11.0, x1 - x0, 1.0, 0xff1a1a1a);
    let shift = if moving { (pen.tick % speed) as f32 / speed as f32 * 8.0 } else { 0.0 };
    let mut sx = x0 + shift;
    while sx < x1 - 1.0 {
        pen.fill(sx, y + 2.0, 1.0, 8.0, 0xff5a5a5a);
        sx += 8.0;
    }
}

fn conveyor(pen: &mut Pen<'_>, view: &View<'_>) {
    let moving = view.working();
    match view.block {
        lift::LIFT => {
            let down = view.read(12) == 1;
            pen.fill(76.0, 47.0, 42.0, 50.0, 0xff3a3a3a);
            let shift = if moving { (pen.tick % 16) as f32 / 16.0 * 8.0 } else { 0.0 };
            let mut sy = 48.0 + if down { shift } else { 8.0 - shift };
            while sy < 96.0 {
                pen.fill(77.0, sy, 40.0, 1.0, 0xff5a5a5a);
                sy += 8.0;
            }
            // Seta grande do sentido ao lado da esteira vertical.
            let accent = solid(view.accent());
            pen.fill(127.0, 56.0, 4.0, 30.0, accent);
            if down {
                pen.quad([[121.0, 84.0], [137.0, 84.0], [129.0, 94.0], [129.0, 94.0]], accent);
            } else {
                pen.quad([[121.0, 58.0], [137.0, 58.0], [129.0, 48.0], [129.0, 48.0]], accent);
            }
        }
        TUNNEL => caption(pen, &tr("veio.ui.tunnel", &[&logistics::TUNNEL_REACH]), 89.0, ROW + 24.0),
        _ => {}
    }
    if view.read(6) > 0 || view.block != lift::LIFT {
        pen.text_center(&per_minute(view.read(6) as f32), 97.0, ROW - 13.0, INK);
    }
}

fn router(pen: &mut Pen<'_>, view: &View<'_>) {
    let info = view.read(12);
    let sorted = info & 4 != 0;
    let priority = info & 3;
    let accent = solid(view.accent());
    let lit = |side: i32| if priority == 0 || priority == side { accent } else { 0xff8a8a8a };
    if view.block == MERGER {
        // Duas entradas que se juntam numa saída.
        pen.fill(70.0, 59.0, 28.0, 3.0, lit(1));
        pen.fill(70.0, 82.0, 28.0, 3.0, lit(2));
        pen.fill(96.0, 59.0, 3.0, 26.0, accent);
        pen.arrow(102.0, ROW + 1.0, if view.working() { (pen.tick % 20) as f32 / 20.0 } else { 0.0 });
        pen.text("A", 42.0, 56.0, INK);
        pen.text("B", 42.0, 79.0, INK);
    } else {
        // Uma entrada que se divide em duas saídas.
        pen.fill(70.0, ROW + 6.0, 20.0, 3.0, accent);
        pen.fill(88.0, 52.0, 3.0, 37.0, accent);
        pen.fill(88.0, 52.0, 26.0, 3.0, lit(1));
        pen.fill(88.0, 86.0, 26.0, 3.0, lit(2));
        pen.text("A", 118.0, 50.0, INK);
        pen.text("B", 118.0, 84.0, INK);
    }
    let mode = if sorted { tr("veio.ui.route.sorted", &[]) }
        else { tr(["veio.ui.route.balanced", "veio.ui.route.left", "veio.ui.route.right"][priority.min(2) as usize], &[]) };
    let x = if view.block == MERGER { 156.0 } else { 150.0 };
    pen.text_center(&pen.fit(&mode, 58.0), x, ROW + 4.0, INK);
}

fn arm(pen: &mut Pen<'_>, view: &View<'_>) {
    // Baú de origem, o braço e o de destino: o braço vai e volta enquanto trabalha.
    for (cx, label) in [(36.0, "veio.ui.arm.from"), (136.0, "veio.ui.arm.to")] {
        pen.bevel(cx - 14.0, 58.0, 28.0, 24.0, true, 0xffa0702a);
        pen.fill(cx - 13.0, 66.0, 26.0, 2.0, 0xff6a4a1a);
        pen.fill(cx - 2.0, 64.0, 4.0, 6.0, 0xffd8c070);
        caption(pen, &tr(label, &[]), cx, 85.0);
    }
    let t = if view.working() { ((pen.tick % 40) as f32 / 20.0 - 1.0).abs() } else { 0.0 };
    let reach = if view.block == inserters::LONG { 1.0 } else { 0.85 };
    let base = [89.0, 86.0];
    let hand = [50.0 + t * 72.0 * reach + (1.0 - reach) * 36.0, 58.0 + (t * std::f32::consts::PI).sin() * -6.0];
    let elbow = [(base[0] + hand[0]) / 2.0, 52.0];
    segment(pen, base, elbow, 3.0, solid(view.accent()));
    segment(pen, elbow, hand, 2.0, solid(view.accent()));
    pen.fill(base[0] - 5.0, base[1] - 2.0, 10.0, 6.0, 0xff3a3a3a);
    pen.fill(hand[0] - 3.0, hand[1] - 1.0, 6.0, 3.0, 0xff3a3a3a);
    let filter = view.read(12) & 4 != 0;
    let text = tr(if filter { "veio.ui.arm.filtered" } else { "veio.ui.arm.any" }, &[]);
    caption(pen, &text, 86.0, 93.0);
}

/// Segmento grosso entre dois pontos.
fn segment(pen: &mut Pen<'_>, from: [f32; 2], to: [f32; 2], thickness: f32, argb: u32) {
    let (dx, dy) = (to[0] - from[0], to[1] - from[1]);
    let length = (dx * dx + dy * dy).sqrt().max(0.01);
    let (nx, ny) = (-dy / length * thickness / 2.0, dx / length * thickness / 2.0);
    pen.quad([[from[0] + nx, from[1] + ny], [to[0] + nx, to[1] + ny], [to[0] - nx, to[1] - ny], [from[0] - nx, from[1] - ny]], argb);
}

/// Canteiro: terra arada e plantas crescendo em fileira.
fn field(pen: &mut Pen<'_>, x: f32, y: f32, w: f32, growing: bool, tree: bool) {
    let tick = pen.tick;
    pen.fill(x, y + 22.0, w, 6.0, 0xff5a3a22);
    for furrow in 0..(w as i32 / 4) {
        pen.fill(x + furrow as f32 * 4.0, y + 22.0, 2.0, 1.0, 0xff3f2814);
    }
    let plants = (w / 9.0) as i32;
    for plant in 0..plants {
        let px = x + 3.0 + plant as f32 * 9.0;
        let age = if growing { ((tick / 40 + plant as i64 * 3) % 8) as f32 } else { 4.0 };
        if tree {
            let height = 6.0 + age * 1.5;
            pen.fill(px + 2.0, y + 22.0 - height, 2.0, height, 0xff6a4a2a);
            pen.fill(px - 1.0, y + 18.0 - height, 8.0, 7.0, 0xff3f8a2e);
            pen.fill(px, y + 17.0 - height, 6.0, 1.0, 0xff5aaa3a);
        } else {
            let height = 3.0 + age * 2.0;
            pen.fill(px + 2.0, y + 22.0 - height, 1.0, height, 0xff5f9a3a);
            pen.fill(px, y + 22.0 - height * 0.6, 2.0, 1.0, 0xff5f9a3a);
            pen.fill(px + 3.0, y + 22.0 - height * 0.8, 2.0, 1.0, 0xff5f9a3a);
            if age >= 7.0 { pen.fill(px + 1.0, y + 21.0 - height, 3.0, 3.0, 0xffd9b84a); }
        }
    }
}

fn grow(pen: &mut Pen<'_>, view: &View<'_>, slots: &[Option<[f32; 2]>]) {
    use production::*;
    let working = view.working();
    let ghost = |pen: &mut Pen<'_>, slot: usize| {
        if let (Some(Some([x, y])), Some(item)) = (slots.get(slot), view.spec.ghosts.get(slot)) {
            if view.empty(slot) && !item.is_empty() { pen.ghost(item, *x, *y); }
        }
    };
    match view.block {
        PLANTER | SPRINKLER => {
            ghost(pen, 0);
            pen.arrow(58.0, ROW + 1.0, if working { (pen.tick % 30) as f32 / 30.0 } else { 0.0 });
            verb(pen, view, 69.0, ROW - 7.0);
            field(pen, 88.0, 54.0, 64.0, working, false);
            if view.block == SPRINKLER {
                for drop in 0..4 {
                    let fall = if working { ((pen.tick / 3 + drop * 5) % 14) as f32 } else { 6.0 };
                    pen.fill(96.0 + drop as f32 * 14.0, 50.0 + fall, 1.0, 2.0, 0xff3f76e4);
                }
                tank(pen, view);
            }
        }
        GROW_LAMP => {
            pen.fill(70.0, 47.0, 40.0, 5.0, 0xff3a3a3a);
            pen.fill(72.0, 52.0, 36.0, 2.0, if working { 0xfff0e080 } else { 0xff8a8a7a });
            if working {
                for ray in 0..5 {
                    pen.fill(74.0 + ray as f32 * 8.0, 55.0, 1.0, 18.0, 0x50fff0a0);
                }
            }
            field(pen, 52.0, 62.0, 76.0, working, false);
        }
        PUMP => {
            let limit = water::capacity(PUMP).max(1);
            let lava = view.read(8) < 0;
            let stored = view.read(8).abs().clamp(0, limit);
            pen.fill(20.0, 78.0, 40.0, 14.0, if lava { 0xffe0561a } else { 0xff3f76e4 });
            for wave in 0..3 {
                let wx = 20.0 + ((pen.tick / 3 + wave * 13) % 36) as f32;
                pen.fill(wx, 79.0, 4.0, 1.0, 0x90ffffff);
            }
            pen.fill(36.0, 52.0, 8.0, 26.0, 0xff5a5a5a);
            pen.arrow(56.0, 52.0, if working { (pen.tick % 20) as f32 / 20.0 } else { 0.0 });
            pen.icon(if lava { icons::LAVA } else { icons::DROP }, 88.0, 50.0);
            pen.text(&tr("veio.ui.pump.rate", &[&view.read(12)]), 98.0, 52.0, INK);
            pen.bar(88.0, 66.0, 76.0, 7.0, stored as f32 / limit as f32, if lava { 0xe0561a } else { 0x3f76e4 });
            pen.text(&format!("{} / {} mB", thousands(stored as i64), thousands(limit as i64)), 88.0, 77.0, SOFT);
        }
        _ => {
            // Colhedora e silvicultura: insumo, a área e o que sai.
            ghost(pen, 0);
            field(pen, 40.0, 54.0, 50.0, working, view.block == TREE_FARM);
            pen.arrow(94.0, ROW + 1.0, if working { (pen.tick % 40) as f32 / 40.0 } else { 0.0 });
            verb(pen, view, 105.0, ROW - 7.0);
            group_label(pen, if view.block == TREE_FARM { "veio.ui.label.products" } else { "veio.ui.label.harvest" }, 120.0, 3);
        }
    }
}

/// Rótulo pequeno acima de um grupo de `count` slots que começa em `x`.
fn group_label(pen: &mut Pen<'_>, key: &str, x: f32, count: usize) {
    let center = x + count as f32 * 9.0 - 1.0;
    let text = tr(key, &[]);
    let text = pen.fit(&text, (count as f32 * 18.0 + 10.0).max(40.0));
    caption(pen, &text, center, ROW - 11.0);
}

fn ranch(pen: &mut Pen<'_>, view: &View<'_>, slots: &[Option<[f32; 2]>]) {
    for slot in 0..2 {
        if let (Some(Some([x, y])), Some(item)) = (slots.get(slot), view.spec.ghosts.get(slot)) {
            if view.empty(slot) { pen.ghost(item, *x, *y); }
        }
    }
    // Pasto com os animais (um ícone por animal, até seis).
    pen.fill(42.0, 52.0, 48.0, 36.0, 0xff7ab048);
    pen.fill(42.0, 52.0, 48.0, 1.0, 0xff9ad068);
    for post in 0..5 {
        pen.fill(42.0 + post as f32 * 11.5, 84.0, 2.0, 6.0, 0xff8a6a3a);
    }
    pen.fill(42.0, 85.0, 48.0, 1.0, 0xff8a6a3a);
    let animals = view.read(12).clamp(0, 6);
    for animal in 0..animals {
        let step = if view.working() { ((pen.tick / 10 + animal as i64) % 2) as f32 } else { 0.0 };
        pen.icon(icons::COW, 44.0 + (animal % 3) as f32 * 15.0 + step, 56.0 + (animal / 3) as f32 * 13.0);
    }
    pen.arrow(94.0, ROW + 1.0, view.progress());
    group_label(pen, "veio.ui.label.products", 120.0, 3);
}

fn storage(pen: &mut Pen<'_>, view: &View<'_>, slots: &[Option<[f32; 2]>]) {
    // Silo: grãos e ração; celeiro: tudo. A tampa de madeira em cima e o nível de ocupação.
    let used = view.read(6).max(0);
    let total = view.read(7).max(slots.len() as i32).max(1);
    if slots.len() <= 18 {
        let text = tr("veio.ui.storage.used", &[&used, &total]);
        pen.text(&text, INVENTORY_X, 46.0, INK);
        pen.bar(110.0, 46.0, 70.0, 7.0, used as f32 / total as f32, view.accent());
    }
}

fn post(pen: &mut Pen<'_>, view: &View<'_>, slots: &[Option<[f32; 2]>]) {
    for (slot, item) in view.spec.ghosts.iter().enumerate() {
        if let Some(Some([x, y])) = slots.get(slot) {
            if view.empty(slot) && !item.is_empty() { pen.ghost(item, *x, *y); }
        }
    }
    let ration_left = view.read(12).max(0);
    let food_at = slots.first().copied().flatten().unwrap_or([40.0, ROW]);
    // O trabalhador sobre a comida, com a ração que resta (o capataz fica no meio da mesa).
    if view.block != workforce::FOREMAN_DESK {
        pen.icon_at_center(icons::PERSON, food_at[0] + 8.0, ROW - 8.0);
    }
    match view.block {
        workforce::FARM_POST | workforce::SHEPHERD_POST | workforce::HAULER_POST | workforce::SITE => {
            ration(pen, ration_left, food_at[0] + 8.0, ROW + 20.0);
            let (arrow, label, outputs) = match view.block {
                workforce::FARM_POST => (70.0, "veio.ui.label.harvest", 102.0),
                workforce::SHEPHERD_POST => (80.0, "veio.ui.label.products", 112.0),
                workforce::HAULER_POST => (102.0, "veio.ui.label.collected", 130.0),
                _ => (74.0, "veio.ui.label.leftover", 104.0),
            };
            pen.arrow(arrow, ROW + 1.0, if view.working() { (pen.tick % 40) as f32 / 40.0 } else { 0.0 });
            verb(pen, view, arrow + 11.0, ROW - 7.0);
            let count = if view.block == workforce::SITE { 1 } else { 3 };
            group_label(pen, label, outputs, count);
            if view.block == workforce::HAULER_POST { group_label(pen, "veio.ui.label.supply", 44.0, 3); }
            if view.block == workforce::SHEPHERD_POST { group_label(pen, "veio.guide.role.feed", 52.0, 1); }
            if view.block == workforce::SITE {
                group_label(pen, "veio.ui.label.supply", 54.0, 1);
                site(pen, view);
            }
        }
        workforce::FOREMAN_DESK => {
            group_label(pen, "veio.ui.label.food", 24.0, 3);
            ration(pen, ration_left, 50.0, ROW + 20.0);
            pen.icon_at_center(icons::PERSON, 104.0, 56.0);
            pen.icon_at_center(icons::SCROLL, 118.0, 56.0);
            caption(pen, &tr("veio.ui.foreman.posts", &[&view.read(6)]), 112.0, 66.0);
            caption(pen, &tr("veio.ui.foreman.queue", &[&view.read(7)]), 112.0, 76.0);
            caption(pen, &tr("veio.guide.role.wage", &[]), 163.0, ROW + 19.0);
        }
        _ => {
            // Quartel: o soldado, as baixas e o salário.
            ration(pen, ration_left, food_at[0] + 8.0, ROW + 20.0);
            pen.icon_at_center(icons::SHIELD, 98.0, 56.0);
            let role = tr(&format!("veio.ui.soldier.{}", view.read(6).clamp(1, 4)), &[]);
            caption(pen, &role, 98.0, 66.0);
            caption(pen, &tr("veio.ui.kills", &[&view.read(9)]), 98.0, 76.0);
            caption(pen, &tr("veio.guide.role.wage", &[]), 148.0, ROW + 19.0);
        }
    }
}

/// Comida que resta ao trabalhador: o pão e quantas tarefas ela paga.
fn ration(pen: &mut Pen<'_>, tasks: i32, cx: f32, y: f32) {
    let text = tasks.max(0).to_string();
    let width = icons::BREAD.w + 2.0 + pen.width(&text);
    let x = (cx - width / 2.0).round();
    pen.icon(icons::BREAD, x, y);
    pen.text(&text, x + icons::BREAD.w + 2.0, y, if tasks > 0 { SOFT } else { BAD });
}

/// Canteiro de obras: a planta subindo do chão conforme os blocos assentados.
fn site(pen: &mut Pen<'_>, view: &View<'_>) {
    let total = view.read(2).max(1);
    let done = view.read(1).clamp(0, total);
    let fraction = done as f32 / total as f32;
    let [x, y, w, h] = [128.0, 48.0, 52.0, 40.0];
    pen.fill(x, y + h, w, 2.0, 0xff7a5a3a);
    // Andaime: contorno pontilhado da planta; o que já subiu, cheio.
    let built = (h * fraction).round();
    pen.fill(x + 4.0, y + h - built, w - 8.0, built, 0xffb08a5a);
    for step in (0..(h as i32)).step_by(4) {
        pen.fill(x + 4.0, y + step as f32, 1.0, 2.0, 0xff6a6a6a);
        pen.fill(x + w - 5.0, y + step as f32, 1.0, 2.0, 0xff6a6a6a);
    }
    for step in (0..(w as i32 - 8)).step_by(4) {
        pen.fill(x + 4.0 + step as f32, y, 2.0, 1.0, 0xff6a6a6a);
    }
    caption(pen, &format!("{}%", (fraction * 100.0) as i32), x + w / 2.0, y + h + 3.0);
}

fn zone(pen: &mut Pen<'_>, view: &View<'_>) {
    if view.empty(0) {
        if let Some(item) = view.spec.ghosts.first() { pen.ghost(item, 24.0, ROW); }
    }
    caption(pen, &tr(if view.read(12) == 1 { "veio.guide.role.feed" } else { "veio.guide.role.seeds" }, &[]), 32.0, ROW + 19.0);
    // Mapa da área: o marco no centro e os anéis de cada nível (o ativo na cor da família).
    let level = view.read(1).clamp(1, 4);
    let unlocked = view.read(2).clamp(1, 4);
    let [cx, cy] = [80.0, 72.0];
    for ring in (1..=4).rev() {
        let half = 4.0 + ring as f32 * 5.0;
        let color = if ring == level { solid(view.accent()) } else if ring <= unlocked { 0xff9a9a9a } else { 0x30000000 };
        pen.fill(cx - half, cy - half, half * 2.0, half * 2.0, color);
        pen.fill(cx - half + 1.0, cy - half + 1.0, half * 2.0 - 2.0, half * 2.0 - 2.0, mix(solid(view.accent()), PANEL, 0.8 - ring as f32 * 0.08));
    }
    pen.icon_at_center(view.spec.verb, cx, cy);
    let side = view.read(3);
    let x = 112.0;
    pen.text(&tr("veio.ui.zone.size", &[&side, &side]), x, 52.0, INK);
    pen.text(&tr("veio.ui.zone.level", &[&level, &unlocked]), x, 63.0, SOFT);
    if view.read(8) > 0 {
        let next = tr("veio.ui.zone.next", &[&view.read(8)]);
        pen.text(&next, x, 76.0, SOFT);
        let width = pen.width(&next);
        pen.icon(icons::EMERALD, x + width + 2.0, 75.0);
    } else {
        pen.text(&tr("veio.ui.zone.max", &[]), x, 76.0, GOOD);
    }
}

/// Porto e estação de carga: a cena do navio (ou do trem) chegando para a próxima partida.
fn port(pen: &mut Pen<'_>, view: &View<'_>) {
    let station = view.block == workforce::society::trade::STATION;
    let period = if station { 60 } else { 120 };
    let left = view.read(12).clamp(0, period);
    let [x, y, w, h] = [7.0, 45.0, 180.0, 32.0];
    sky(pen, x, y, w, h - 10.0, false);
    let dock = x + w - 44.0;
    let travel = (left as f32 / period as f32).clamp(0.0, 1.0);
    if station {
        ground(pen, x, y + h - 10.0, w);
        pen.fill(x, y + h - 12.0, w, 1.0, 0xff6a6a6a);
        let mut tie = x;
        while tie < x + w {
            pen.fill(tie, y + h - 11.0, 2.0, 1.0, 0xff5a4030);
            tie += 6.0;
        }
        pen.fill(dock, y + h - 18.0, 40.0, 6.0, 0xff8a8a8a);
        let train_x = dock - 6.0 - travel * (w - 50.0);
        pen.icon(icons::TRAIN, train_x, y + h - 21.0);
        pen.icon(icons::TRAIN, train_x - 15.0, y + h - 21.0);
    } else {
        pen.fill(x, y + h - 10.0, w, 10.0, 0xff3f76e4);
        for wave in 0..8 {
            let wx = x + ((pen.tick / 4 + wave * 23) % (w as i64 - 4)) as f32;
            pen.fill(wx, y + h - 8.0 + (wave % 2) as f32 * 4.0, 4.0, 1.0, 0x90ffffff);
        }
        pen.fill(dock + 10.0, y + h - 13.0, 34.0, 4.0, 0xff8a6a3a);
        pen.fill(dock + 14.0, y + h - 9.0, 2.0, 9.0, 0xff6a4a2a);
        pen.fill(dock + 36.0, y + h - 9.0, 2.0, 9.0, 0xff6a4a2a);
        let bob = ((pen.tick / 20) % 2) as f32;
        let ship_x = dock - 4.0 - travel * (w - 50.0);
        pen.icon(icons::SHIP, ship_x, y + h - 19.0 + bob);
    }
    // Caixas no cais: uma por pilha que vai no próximo navio (slots ocupados, até a capacidade).
    let capacity = view.read(1).clamp(0, 9);
    let filled = (0..workforce::society::trade::SLOTS).filter(|&slot| !view.empty(slot)).count() as i32;
    let stacks = if view.data.len() > EMPTY { filled.min(capacity) } else { 0 };
    for crate_ in 0..stacks.min(6) {
        let cx = dock + 12.0 + (crate_ % 3) as f32 * 8.0;
        let cy = y + h - 19.0 - (crate_ / 3) as f32 * 6.0 - if station { 5.0 } else { 0.0 };
        pen.bevel(cx, cy, 7.0, 6.0, true, 0xffb08850);
    }
    let text = tr("veio.ui.port.departs", &[&clock_text(left)]);
    let width = pen.width(&text);
    pen.fill(x + 2.0, y + 1.0, width + 6.0, 10.0, 0x90000000);
    pen.shadow_text(&text, x + 5.0, y + 2.0, 0xffffff);
    let hold = tr("veio.ui.port.hold", &[&stacks, &capacity]);
    let width = pen.width(&hold);
    pen.fill(x + w - width - 8.0, y + 1.0, width + 6.0, 10.0, 0x90000000);
    pen.shadow_text(&hold, x + w - width - 5.0, y + 2.0, 0xffffff);
}

fn terminal(pen: &mut Pen<'_>, view: &View<'_>, slots: &[Option<[f32; 2]>]) {
    group_label_at(pen, "veio.guide.role.dispatch", 11.0, 3, TERMINAL_Y - 11.0);
    group_label_at(pen, "veio.guide.role.received", 120.0, 3, TERMINAL_Y - 11.0);
    // A rota: trilho, céu ou estrada, e o veículo indo e voltando.
    let [x0, x1, y] = [68.0, 114.0, TERMINAL_Y + 6.0];
    let channel = view.read(12);
    let route = if (1..=16).contains(&channel) { dye_color(channel as usize - 1) } else { 0xff8a8a8a };
    match view.block {
        freight::RAIL => {
            pen.fill(x0, y + 4.0, x1 - x0, 1.0, 0xff6a6a6a);
            let mut tie = x0;
            while tie < x1 { pen.fill(tie, y + 5.0, 2.0, 1.0, 0xff5a4030); tie += 5.0; }
        }
        _ => {
            let mut dash = x0;
            while dash < x1 { pen.fill(dash, y + 4.0, 3.0, 1.0, route); dash += 6.0; }
        }
    }
    pen.fill(x0 - 2.0, y - 6.0, 3.0, 12.0, route);
    pen.fill(x1 - 1.0, y - 6.0, 3.0, 12.0, route);
    let phase = view.read(8);
    let t = view.progress().clamp(0.0, 1.0);
    let position = match phase { 1 => t, 2 => 1.0 - t, _ => 0.0 };
    let icon = view.spec.verb;
    let vx = x0 + position * (x1 - x0 - icon.w);
    let lift = if view.block == freight::DRONE && phase > 0 { -4.0 } else { 0.0 };
    pen.icon(icon, vx.round(), y - icon.h + 4.0 + lift);
    if view.block != freight::RAIL {
        let key = if view.block == freight::DRONE { "veio.guide.role.inflight" } else { "veio.guide.role.cargo" };
        pen.text(&tr(key, &[]), 104.0, 84.0, SOFT);
    }
    if view.block == freight::caravan::ID {
        if let Some(Some([fx, fy])) = slots.get(7) {
            if view.empty(7) { pen.ghost(WHEAT_ITEM, *fx, *fy); }
            pen.text(&tr("veio.guide.role.feed", &[]), fx + 20.0, fy + 4.0, SOFT);
        }
    }
}

const WHEAT_ITEM: &str = "minecraft:wheat";
const EMERALD_ITEM: &str = "minecraft:emerald";

fn group_label_at(pen: &mut Pen<'_>, key: &str, x: f32, count: usize, y: f32) {
    let center = x + count as f32 * 9.0 - 1.0;
    let text = tr(key, &[]);
    let text = pen.fit(&text, count as f32 * 18.0 + 10.0);
    pen.text_center(&text, center, y, SOFT);
}

/// Cor de cada corante, para mostrar o canal.
fn dye_color(index: usize) -> u32 {
    const COLORS: [u32; 16] = [0xf0f0f0, 0xf9801d, 0xc74ebd, 0x3ab3da, 0xfed83d, 0x80c71f, 0xf38baa, 0x474f52,
        0x9d9d97, 0x169c9c, 0x8932b8, 0x3c44aa, 0x835432, 0x5e7c16, 0xb02e26, 0x1d1d21];
    solid(COLORS[index.min(15)])
}

fn station(pen: &mut Pen<'_>, view: &View<'_>, _slots: &[Option<[f32; 2]>]) {
    let sender = view.block == stations::SENDER;
    let channel = view.read(8).clamp(0, 15) as usize;
    let color = dye_color(channel);
    let moving = if view.working() { (pen.tick % 30) as f32 / 30.0 } else { 0.0 };
    // Envio: os slots, a seta e a bandeira do canal. Descarga: a bandeira, a seta e os slots.
    let (slots_x, arrow_x, flag_x) = if sender { (STATION_SEND, 108.0, 140.0) } else { (STATION_RECEIVE, 46.0, 18.0) };
    group_label(pen, if sender { "veio.ui.label.to_send" } else { "veio.ui.label.arrived" }, slots_x, 4);
    pen.arrow(arrow_x, ROW + 1.0, moving);
    pen.fill(flag_x, 50.0, 2.0, 34.0, 0xff5a4030);
    pen.fill(flag_x + 2.0, 50.0, 14.0, 9.0, color);
    pen.fill(flag_x + 2.0, 50.0, 14.0, 1.0, 0x60ffffff);
}

const STATION_SEND: f32 = 30.0;
const STATION_RECEIVE: f32 = 74.0;

fn dwelling(pen: &mut Pen<'_>, view: &View<'_>, slots: &[Option<[f32; 2]>]) {
    for (slot, item) in view.spec.ghosts.iter().enumerate() {
        if let Some(Some([x, y])) = slots.get(slot) {
            if view.empty(slot) && !item.is_empty() { pen.ghost(item, *x, *y); }
        }
    }
    match view.block {
        workforce::society::HOUSE => {
            // A casa com a família: uma pessoa por morador, corações pelo bem-estar.
            let [x, y] = [18.0, 50.0];
            pen.quad([[x, y + 16.0], [x + 26.0, y], [x + 52.0, y + 16.0], [x + 26.0, y + 16.0]], 0xffa03a2a);
            pen.fill(x + 4.0, y + 16.0, 44.0, 26.0, 0xffd8c8a0);
            pen.fill(x + 21.0, y + 28.0, 10.0, 14.0, 0xff6a4a2a);
            pen.fill(x + 8.0, y + 21.0, 8.0, 7.0, 0xff8ab8e8);
            pen.fill(x + 36.0, y + 21.0, 8.0, 7.0, 0xff8ab8e8);
            // Um rosto por morador, na janela de cima.
            for person in 0..view.read(6).clamp(0, 4) {
                pen.icon(icons::PERSON, x + 2.0 + person as f32 * 12.0, y + 43.0);
            }
            group_label(pen, "veio.ui.label.pantry", 104.0, 3);
            let need = view.read(1);
            let text = tr(&format!("veio.house.need{need}"), &[]);
            let color = if need == 0 { GOOD } else { WARN };
            pen.paragraph(&text, 104.0, ROW + 20.0, 76.0, 2, color);
        }
        workforce::SHOP => {
            group_label(pen, "veio.ui.label.food", 13.0, 1);
            group_label(pen, "veio.ui.label.stock", 41.0, 6);
            group_label(pen, "veio.ui.label.cash", 163.0, 1);
            caption(pen, &tr("veio.ui.shop.prices", &[&view.read(7)]), 94.0, ROW + 20.0);
        }
        _ => {
            // Poço: balde descendo e subindo.
            caption(pen, &tr("veio.guide.role.bucket", &[]), 54.0, ROW + 19.0);
            let [x, y] = [96.0, 50.0];
            pen.fill(x, y + 22.0, 40.0, 20.0, 0xff8a8a8a);
            pen.fill(x + 2.0, y + 24.0, 36.0, 4.0, 0xff3f76e4);
            pen.fill(x + 2.0, y, 3.0, 22.0, 0xff6a4a2a);
            pen.fill(x + 35.0, y, 3.0, 22.0, 0xff6a4a2a);
            pen.quad([[x - 3.0, y + 2.0], [x + 20.0, y - 6.0], [x + 43.0, y + 2.0], [x + 20.0, y + 2.0]], 0xffa03a2a);
            let rope = 6.0 + ((pen.tick % 80) as f32 / 40.0 - 1.0).abs() * 12.0;
            pen.fill(x + 19.0, y + 2.0, 1.0, rope, 0xffc8b080);
            pen.fill(x + 16.0, y + 2.0 + rope, 7.0, 5.0, 0xff9a9a9a);
        }
    }
}

fn pantry(pen: &mut Pen<'_>, view: &View<'_>) {
    let layout = line(1, MIDDLE);
    let [fx, fy] = layout.inputs[0];
    if view.empty(0) { pen.ghost(BREAD_ITEM, fx, fy); }
    pen.result_slot(layout.output[0], layout.output[1], view.accent());
    if view.empty(1) { pen.ghost("minecraft:emerald", layout.output[0], layout.output[1]); }
    let credit = view.read(1).max(0) as f32 / production::pantry::PRICE.max(1) as f32;
    pen.arrow(layout.arrow[0], layout.arrow[1], credit);
    verb(pen, view, layout.arrow[0] + 11.0, ROW - 7.0);
    caption(pen, &tr("veio.ui.pantry.credit", &[&view.read(1), &production::pantry::PRICE]), layout.arrow[0] + 11.0, ROW + 19.0);
    let villagers = view.read(12).clamp(0, 9);
    for villager in 0..villagers.min(6) {
        pen.icon(icons::PERSON, 12.0 + (villager % 3) as f32 * 11.0, 50.0 + (villager / 3) as f32 * 11.0);
    }
}

const BREAD_ITEM: &str = "minecraft:bread";

fn tesla(pen: &mut Pen<'_>, view: &View<'_>) {
    // A bobina e os raios quando dispara; o alcance como um anel.
    let [cx, cy] = [60.0, 70.0];
    pen.fill(cx - 3.0, cy - 10.0, 6.0, 26.0, 0xffb87333);
    for coil in 0..5 {
        pen.fill(cx - 6.0, cy - 8.0 + coil as f32 * 5.0, 12.0, 2.0, 0xffd89050);
    }
    pen.fill(cx - 8.0, cy - 18.0, 16.0, 8.0, 0xff9aa0a8);
    pen.fill(cx - 10.0, cy + 16.0, 20.0, 4.0, 0xff5a5a5a);
    if view.working() && (pen.tick / 3) % 2 == 0 {
        for bolt in 0..3 {
            let angle = bolt as f32 * 2.1 + (pen.tick / 3) as f32;
            let to = [cx + angle.cos() * 30.0, cy - 14.0 + angle.sin() * 14.0];
            segment(pen, [cx, cy - 14.0], [(cx + to[0]) / 2.0 + 4.0, (cy - 14.0 + to[1]) / 2.0 - 3.0], 1.0, 0xffbfefff);
            segment(pen, [(cx + to[0]) / 2.0 + 4.0, (cy - 14.0 + to[1]) / 2.0 - 3.0], to, 1.0, 0xffbfefff);
        }
    }
    let smog = view.read(12).max(0);
    let limit = pollution::RAID as i32;
    pen.icon(icons::SMOKE, 96.0, 50.0);
    pen.text(&pen.fit(&tr("veio.ui.tesla.smog", &[]), 58.0), 110.0, 50.0, INK);
    pen.bar(96.0, 61.0, 70.0, 7.0, smog as f32 / limit as f32, 0x8a7a6a);
    pen.text(&format!("{} / {}", thousands(smog as i64), thousands(limit as i64)), 96.0, 72.0, SOFT);
}

fn anchor(pen: &mut Pen<'_>, view: &View<'_>) {
    // O chunk visto de cima: 16x16 com o bloco no centro, aceso enquanto carregado.
    let on = view.read(0) == 1;
    let [x, y, size] = [44.0, 48.0, 48.0];
    for row in 0..4 {
        for column in 0..4 {
            let cell = size / 4.0;
            let shade = if (row + column) % 2 == 0 { 0xff7ab048 } else { 0xff6aa03c };
            pen.fill(x + column as f32 * cell, y + row as f32 * cell, cell, cell, if on { shade } else { mix(shade, 0xff404040, 0.6) });
        }
    }
    let pulse = if on { ((pen.tick % 40) as f32 / 40.0 * 4.0).round() } else { 0.0 };
    pen.fill(x - pulse, y - pulse, size + pulse * 2.0, 1.0, solid(view.accent()));
    pen.fill(x - pulse, y + size + pulse - 1.0, size + pulse * 2.0, 1.0, solid(view.accent()));
    pen.fill(x - pulse, y - pulse, 1.0, size + pulse * 2.0, solid(view.accent()));
    pen.fill(x + size + pulse - 1.0, y - pulse, 1.0, size + pulse * 2.0, solid(view.accent()));
    pen.icon_at_center(icons::LOCK, x + size / 2.0, y + size / 2.0);
    let state = tr(if on { "veio.ui.anchor.on" } else { "veio.ui.anchor.off" }, &[]);
    let lines = pen.paragraph(&state, 104.0, 52.0, 66.0, 2, if on { GOOD } else { BAD });
    pen.text(&tr("veio.ui.anchor.cost", &[&industry::ANCHOR_POWER]), 104.0, 56.0 + lines as f32 * 9.0, SOFT);
}

fn lab(pen: &mut Pen<'_>, view: &View<'_>, slots: &[Option<[f32; 2]>]) {
    let stage = view.read(8).max(0) as usize;
    let Some(research) = research::RESEARCH.get(stage) else {
        pen.text(&tr("veio.research.complete", &[]), 20.0, 70.0, GOOD);
        return;
    };
    let delivered = [view.read(9).max(0), view.read(12).max(0)];
    for (index, &(item, need)) in research.inputs.iter().enumerate() {
        let Some(Some([x, y])) = slots.get(index) else { continue; };
        if view.empty(index) { pen.ghost(item, *x, *y); }
        caption(pen, &format!("{}/{need}", delivered[index]), x + 8.0, ROW + 19.0);
    }
    pen.arrow(80.0, ROW + 1.0, if view.working() { (pen.tick % 40) as f32 / 40.0 } else { 0.0 });
    verb(pen, view, 91.0, ROW - 7.0);
    let name = tr(&format!("veio.research.{}", research.id), &[]);
    pen.text(&pen.fit(&name, 64.0), 106.0, 52.0, INK);
    let total: u32 = research.inputs.iter().map(|(_, need)| need).sum();
    let got: u32 = research.inputs.iter().zip(delivered).map(|((_, need), got)| (got as u32).min(*need)).sum();
    pen.bar(106.0, 64.0, 64.0, 7.0, got as f32 / total.max(1) as f32, view.accent());
    pen.text(&pen.fit(&tr("veio.ui.lab.stage", &[&(stage + 1), &research::RESEARCH.len()]), 64.0), 106.0, 75.0, SOFT);
}

// ---------- Cartões ----------

pub(crate) struct Card {
    pub icon: Icon,
    /// Item no lugar do ícone (o pedido do dia, por exemplo).
    pub item: Option<&'static str>,
    pub value: String,
    pub label: String,
    pub rgb: u32,
    /// Dica ao passar o mouse.
    pub hint: String,
}

fn card(icon: Icon, value: String, label: &str, hint: String) -> Card {
    Card { icon, item: None, value, label: tr(&format!("veio.ui.card.{label}"), &[]), rgb: INK, hint }
}

pub(crate) fn cards(view: &View<'_>) -> Vec<Card> {
    use production::*;
    let read = |index: usize| view.read(index);
    let metric = |key: &str, args: &[&dyn std::fmt::Display]| tr(&format!("veio.metric.{key}"), args);
    let rate = || card(icons::CLOCK, per_minute(read(6) as f32), "rate", metric("rate", &[&read(6)]));
    let uptime = || card(icons::GAUGE, format!("{}%", read(7).clamp(0, 100)), "uptime",
        if read(2) > 1 { metric("utilization_max", &[&read(7), &presentation::cycles_per_minute(read(2))]) } else { metric("utilization", &[&read(7)]) });
    let energy = || {
        let (energy, limit) = view.energy();
        card(icons::BATTERY, thousands(energy as i64), "energy", tr("veio.energy", &[&energy, &limit]))
    };
    let emission = || {
        let effect = view.effect();
        let per_minute = (pollution::emission(view.block) * effect.pollution * 60.0).round() as i32;
        (per_minute > 0).then(|| card(icons::SMOKE, format!("{per_minute}/min"), "smog", metric("emission", &[&per_minute])))
    };
    let mut cards = match view.spec.kind {
        Kind::Process => {
            let mut cards = vec![rate(), uptime()];
            cards.push(emission().unwrap_or_else(|| card(icons::CHECK, thousands(read(9) as i64), "cycles", tr("veio.ui.hint.cycles", &[]))));
            cards
        }
        Kind::Extract => {
            let purity = read(12).clamp(0, 4) as usize;
            let names = ["unknown", "impure", "normal", "pure", "rich"];
            vec![rate(), card(icons::PICKAXE, tr(&format!("veio.ui.purity.{}", names[purity]), &[]), "deposit",
                tr(&format!("veio.purity.{}", names[purity]), &[])), uptime()]
        }
        Kind::Burner => {
            let mut cards = vec![card(icons::BOLT, format!("{}", power_output(view)), "fe_tick", tr("veio.ui.hint.fe_tick", &[])), energy()];
            if let Some(smog) = emission() { cards.push(smog); }
            if view.block == STEAM_ENGINE {
                cards.push(card(icons::DROP, thousands(read(8).max(0) as i64), "water", metric("water", &[&read(8), &water::capacity(STEAM_ENGINE)])));
            }
            cards.truncate(3);
            cards
        }
        Kind::Passive => {
            let mut cards = vec![card(icons::BOLT, format!("{}", read(12).max(0)), "fe_tick", tr("veio.ui.hint.fe_tick", &[]))];
            if view.block == HYDRO_TURBINE {
                cards.push(card(icons::DROP, format!("{}", read(8)), "head", metric("head", &[&read(8)])));
            }
            cards.push(energy());
            cards
        }
        Kind::Battery => {
            let (stored, limit) = view.energy();
            vec![energy(), card(icons::BATTERY, thousands(limit as i64), "capacity", tr("veio.energy", &[&stored, &limit])),
                card(icons::HOURGLASS, format!("{}%", stored * 100 / limit.max(1)), "charge", tr("veio.hint.battery", &[]))]
        }
        Kind::Conveyor => {
            let mut cards = vec![rate()];
            let speed = match view.block { EXPRESS_BELT => 3, FAST_BELT => 2, _ => 1 };
            if view.block == lift::LIFT {
                cards.push(card(icons::CART, tr(if read(12) == 1 { "veio.ui.lift.down" } else { "veio.ui.lift.up" }, &[]), "direction", tr("veio.hint.lift", &[])));
            } else {
                cards.push(card(icons::CART, format!("{speed}x"), "speed", tr(&format!("veio.ui.hint.belt{speed}"), &[])));
            }
            cards
        }
        Kind::Router => vec![rate()],
        Kind::Arm => vec![rate(), energy()],
        Kind::Grow => match view.block {
            PLANTER => vec![card(icons::SEEDS, thousands(read(9) as i64), "planted", metric("planted", &[&read(9)])), energy()],
            GROW_LAMP | SPRINKLER => vec![card(icons::SAPLING, thousands(read(9) as i64), "grown", metric("grown", &[&read(9)])), energy()],
            TREE_FARM => vec![card(icons::TREE, thousands(read(9) as i64), "trees", metric("trees", &[&read(9)])), energy()],
            HARVESTER => vec![card(icons::WHEAT, per_minute(read(6) as f32), "harvests", metric("harvests", &[&read(6)])), energy()],
            _ => vec![card(icons::DROP, format!("{}", read(12)), "pumping", metric("pumping", &[&read(12)])), energy()],
        },
        Kind::Ranch => vec![card(icons::COW, format!("{}", read(12)), "animals", metric("animals", &[&read(12)])), energy()],
        Kind::Storage => {
            let total = read(7).max(1);
            vec![card(icons::CHEST, format!("{}/{}", read(6), total), "slots", metric("storage", &[&read(6), &total]))]
        }
        Kind::Post => match view.block {
            workforce::FARM_POST => vec![card(icons::WHEAT, thousands(read(9) as i64), "tasks", metric("tasks", &[&read(9)]))],
            workforce::HAULER_POST => vec![card(icons::CART, thousands(read(9) as i64), "trips", metric("trips", &[&read(9)]))],
            workforce::SHEPHERD_POST => vec![card(icons::COW, thousands(read(9) as i64), "chores", metric("chores", &[&read(9)]))],
            workforce::FOREMAN_DESK => vec![card(icons::SCROLL, format!("{}", read(6)), "posts", metric("posts", &[&read(6), &read(7)]))],
            workforce::SITE => {
                let plan = workforce::site::panel_lines(view.data).into_iter().next().unwrap_or_default();
                vec![card(icons::HOUSE, workforce::site::plan_name(read(6)), "plan", plan),
                    card(icons::HAMMER, format!("{}/{}", read(1), read(2)), "blocks", tr("veio.ui.hint.blocks", &[]))]
            }
            _ => vec![card(icons::SHIELD, thousands(read(9) as i64), "kills", metric("kills", &[&read(9)]))],
        },
        Kind::Zone => vec![card(icons::PERSON, format!("{}/{}", read(6), read(7)), "villagers", tr("veio.ui.hint.zone_villagers", &[])),
            card(icons::HAMMER, format!("{}/{}", read(4), read(5)), "work", tr("veio.ui.hint.zone_work", &[]))],
        Kind::Port => {
            let coins = read(9);
            vec![card(icons::HOURGLASS, clock_text(read(12)), "departs", tr("veio.dock.next", &[&read(12), &read(1), &read(7)])),
                card(icons::CHEST, format!("{}", read(6)), "sold_today", tr("veio.ui.hint.sold_today", &[])),
                Card { rgb: if coins < 0 { BAD } else if coins > 0 { GOOD } else { INK }, ..card(icons::EMERALD, signed(coins), "profit_today", tr("veio.ui.hint.profit_today", &[])) }]
        }
        Kind::Terminal => {
            let channel = read(12);
            let color = if (1..=16).contains(&channel) { tr(&format!("color.minecraft.{}", stations::DYES[channel as usize - 1]), &[]) } else { tr("veio.freight.disabled", &[]) };
            let mut cards = vec![card(icons::FLAG, color, "route", tr("veio.ui.hint.route", &[]))];
            if view.block == freight::caravan::ID {
                cards.push(card(icons::CHEST, format!("{}", read(6)), "per_trip", tr("veio.freight.capacity", &[&read(6)])));
            } else {
                cards.push(card(icons::CHECK, thousands(read(9) as i64), "trips_done", tr("veio.ui.hint.trips_done", &[])));
            }
            cards
        }
        Kind::Station => {
            let channel = tr(&format!("color.minecraft.{}", stations::DYES[read(8).clamp(0, 15) as usize]), &[]);
            let mut cards = vec![card(icons::FLAG, channel, "channel", tr("veio.hint.unloading", &[]))];
            if view.block == stations::SENDER {
                cards.push(card(icons::CLOCK, per_minute(read(12) as f32), "rate", tr("veio.station.rate", &[&read(12)])));
            }
            cards.push(card(icons::CART, thousands(read(9).max(0) as i64), "transit", tr("veio.ui.hint.transit", &[&stations::CAPACITY])));
            cards
        }
        Kind::Dwelling => match view.block {
            workforce::society::HOUSE => vec![
                card(icons::PEOPLE, format!("{}/{}", read(6), read(7)), "residents", tr("veio.house.residents", &[&read(6), &read(7), &workforce::society::mayor::decimal(read(12))])),
                card(icons::HEART, workforce::society::mayor::decimal(read(12)), "idh", tr("veio.ui.hint.idh", &[])),
                card(icons::EMERALD, thousands(read(9) as i64), "wallet", tr("veio.house.wallet", &[&read(9), &read(2)]))],
            workforce::SHOP => vec![
                card(icons::EMERALD, thousands(read(9) as i64), "sales", tr("veio.shop.sales", &[&read(9), &read(12)])),
                card(icons::CHEST, thousands(read(6) as i64), "stock", tr("veio.shop.stock", &[&read(6), &read(7)])),
                if read(2) > 1 { Card { rgb: WARN, ..card(icons::SCROLL, thousands(read(2) as i64), "owes", tr("veio.shop.owes", &[&read(2)])) } }
                else { card(icons::EMERALD, thousands(read(12) as i64), "cash", tr("veio.ui.hint.cash", &[])) }],
            _ => vec![card(icons::HOUSE, format!("{}", workforce::society::WELL_RADIUS), "radius", tr("veio.hint.well", &[]))],
        },
        Kind::Pantry => vec![card(icons::PERSON, format!("{}", read(12)), "villagers",
            if read(12) >= pantry::VILLAGE_SIZE { tr("veio.pantry.village", &[&read(12)]) } else { tr("veio.pantry.no_village", &[&read(12)]) }),
            card(icons::EMERALD, thousands(read(9) as i64), "sold", tr("veio.ui.hint.pantry_sold", &[]))],
        Kind::Tesla => vec![card(icons::SHIELD, thousands(read(9) as i64), "kills", metric("kills", &[&read(9)])), energy()],
        Kind::Anchor => vec![energy()],
        Kind::Lab => vec![energy()],
        Kind::Contract => contract_cards(view),
        Kind::Board => board_cards(view),
        Kind::Custom => Vec::new(),
    };
    cards.truncate(3);
    cards
}

fn signed(value: i32) -> String {
    if value > 0 { format!("+{}", thousands(value as i64)) } else { thousands(value as i64) }
}

/// Posição dos cartões: dividem a largura do painel.
fn card_boxes(count: usize) -> Vec<[f32; 4]> {
    if count == 0 { return Vec::new(); }
    let gap = 3.0;
    let width = ((182.0 - gap * (count - 1) as f32) / count as f32).floor();
    (0..count).map(|index| [6.0 + index as f32 * (width + gap), CARDS_Y, width, CARD_H]).collect()
}

fn draw_cards(pen: &mut Pen<'_>, cards: &[Card]) {
    for (card, [x, y, w, h]) in cards.iter().zip(card_boxes(cards.len())) {
        // Sem caixa: número e nome soltos no painel, um traço fino entre eles.
        if pen.hovering(x, y, w, h) { pen.fill(x, y, w, h, 0x18ffffff); }
        if x > 7.0 { pen.fill(x - 2.0, y + 2.0, 1.0, h - 4.0, 0xffa8a8a8); }
        // Ícone e número na primeira linha; o nome embaixo, na largura toda.
        let icon_w = card.icon.w.min(12.0);
        let drawn = card.item.is_some_and(|item| pen.item(item, x + 3.0, y, 12.0));
        if !drawn {
            pen.icon(card.icon, x + 3.0 + ((12.0 - icon_w) / 2.0).round(), y + 2.0 + ((8.0 - card.icon.h.min(10.0)) / 2.0).round().max(-1.0));
        }
        let value = pen.fit(&card.value, w - 20.0);
        pen.text(&value, x + 17.0, y + 2.0, card.rgb);
        let label = pen.fit(&card.label, w - 6.0);
        pen.text(&label, x + 3.0, y + 11.0, FAINT);
    }
}

// ---------- Dicas ----------

/// Dica do painel do kit. Devolve `true` quando tratou o ponto.
pub(crate) fn tooltip(event: &mut ScreenTooltipEvent<'_>) -> bool {
    let Some(view) = View::new(event.block, event.data) else { return false; };
    let [x, y] = event.at;
    let inside = |[rx, ry, rw, rh]: [f32; 4]| (rx..rx + rw).contains(&x) && (ry..ry + rh).contains(&y);
    if inside(HELP) {
        help_lines(view.block, event.lines);
        return true;
    }
    if view.spec.kind == Kind::Custom { return false; }
    if view.spec.kind == Kind::Board && inside(STATUS) {
        if let Some(current) = milestones::PHASES.get(view.read(1).max(0) as usize) {
            let done = view.read(2).clamp(0, current.count as i32);
            event.lines.push(tr("veio.board.deliver", &[&done, &current.count, &tr(current.item_key, &[])]));
            event.lines.push(tr("veio.board.reward", &[&current.reward.1, &tr(current.reward_key, &[])]));
            let unlocks: Vec<String> = current.unlocks.iter().map(|key| tr(key, &[])).collect();
            event.lines.push(tr("veio.board.unlocks", &[&unlocks.join(", ")]));
        } else {
            event.lines.push(tr("veio.board.done", &[]));
        }
        return true;
    }
    if inside(STATUS) {
        event.lines.push(tr(&format!("veio.status.{}", presentation::STATUS_KEYS[view.status()]), &[]));
        event.lines.push(tr(&operations::advice_key(view.status() as i32), &[]));
        return true;
    }
    if view.powered() && inside([ENERGY[0] - 2.0, ENERGY[1] - 14.0, ENERGY[2] + 4.0, ENERGY[3] + 14.0]) {
        let (energy, limit) = view.energy();
        event.lines.push(tr("veio.energy", &[&energy, &limit]));
        event.lines.push(tr(if is_power_block(view.block) { "veio.ui.hint.energy_out" } else { "veio.ui.hint.energy_in" }, &[]));
        return true;
    }
    let cards = cards(&view);
    for (card, rect) in cards.iter().zip(card_boxes(cards.len())) {
        if inside(rect) {
            event.lines.push(format!("{} {}", card.value, card.label));
            event.lines.push(card.hint.clone());
            return true;
        }
    }
    if view.spec.kind == Kind::Process {
        let layout = line(output_slots(view.block).start, CENTER);
        if let Some(recipe) = shown_recipe(&view, 0) {
            for (rect, machine) in consumer_boxes(recipe.output.0, layout.output[0] + 8.0, ROW + 23.0) {
                if inside(rect) {
                    event.lines.push(tr("veio.ui.hint.goes_to", &[&item_name(machine)]));
                    return true;
                }
            }
        }
    }
    if let Some(slots) = slots(view.block) {
        for (index, slot) in slots.iter().enumerate() {
            if let Some([sx, sy]) = slot
                && inside([*sx, *sy, 16.0, 16.0])
            {
                if let Some(text) = tr_opt(&format!("veio.slothelp.{}.{index}", view.name), &[]) {
                    event.lines.push(text);
                }
                return true;
            }
        }
    }
    false
}

/// Ajuda do botão "?": setor, o que faz e a receita do bloco.
pub(crate) fn help_lines(block: &str, lines: &mut Vec<String>) {
    let name = block.strip_prefix("veio:").unwrap_or(block);
    lines.push(item_name(block));
    if let Some(spec) = spec::of(block) { lines.push(tr(spec.family.key(), &[])); }
    for prefix in ["veio.help", "veio.guide.make"] {
        if let Some(text) = tr_opt(&format!("{prefix}.{name}"), &[]) {
            lines.extend(text.lines().map(str::to_owned));
        }
    }
    lines.push(tr("veio.guide.tip", &[]));
}
