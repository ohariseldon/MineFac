//! Wiki do Veio dentro do jogo: a tecla J abre uma tela própria (`Host::mod_screen_open`), como
//! um livro de receitas com tudo do jogo. Abas do criativo mais a aba Sistemas; à esquerda a
//! lista com ícones, à direita a página: grade de fabricação, máquina que produz, o que a máquina
//! processa e onde o item é usado. Clicar num ícone abre a página dele; o botão direito volta.
//!
//! Só no cliente e só desenhando: o estado é da janela (aba, página, rolagem, histórico) e as
//! áreas clicáveis são as do último quadro desenhado. Dados em `data.rs`, gerado por
//! `tools/build_wiki.py` a partir das receitas, das abas e dos avanços.
use super::*;
use horizon_client::mod_api::{HudBatch, NativeScreenView, Text};
use std::sync::Mutex;

mod data;
#[cfg(test)]
mod tests;

pub(crate) struct Entry {
    pub id: &'static str,
    pub tab: u8,
    /// Ingredientes da receita de bancada (sem forma: um por espaço da grade).
    pub craft: &'static [&'static str],
    pub makes: u32,
    /// Chave da descrição (vazia nos itens do Minecraft).
    pub text: &'static str,
    /// Título do avanço que libera a receita.
    pub unlock: &'static str,
}

pub(crate) struct Article {
    pub id: &'static str,
    pub icon: &'static str,
}

const MOD: &str = "veio";
const KEY: &str = "wiki";
const DEFAULT_KEY: &str = "key.keyboard.j";
const LIST_W: f32 = 120.0;
const ROW: f32 = 18.0;
const LINE: f32 = 10.0;
const SLOT: f32 = 18.0;

const BACKDROP: u32 = 0xa0000000;
const BORDER: u32 = 0xff15161a;
const PANEL: u32 = 0xff2a2c33;
const HEADER: u32 = 0xff3a3d46;
const ROW_ON: u32 = 0xff4a6fa5;
const ROW_HOVER: u32 = 0xff383b44;
const WHITE: u32 = 0xffffff;
const SOFT: u32 = 0xb8bcc6;
const GOLD: u32 = 0xffcc55;
const GREEN: u32 = 0x7fdc7f;

#[derive(Clone, Copy, PartialEq, Debug)]
enum Action {
    Tab(usize),
    Pick(usize),
    Goto(&'static str),
}

struct Hit {
    rect: [f32; 4],
    action: Option<Action>,
    tip: Option<String>,
}

struct State {
    tab: usize,
    pick: usize,
    list_scroll: usize,
    page_scroll: f32,
    page_max: f32,
    list_right: f32,
    history: Vec<(usize, usize)>,
    hits: Vec<Hit>,
}

static STATE: Mutex<State> = Mutex::new(State {
    tab: 0, pick: 0, list_scroll: 0, page_scroll: 0.0, page_max: 0.0, list_right: 0.0,
    history: Vec::new(), hits: Vec::new(),
});

fn guide_tab() -> usize { data::TABS.len() - 1 }

/// O que a aba lista: índices em `ENTRIES` (abas do criativo) ou em `ARTICLES` (Sistemas).
fn list(tab: usize) -> Vec<usize> {
    if tab == guide_tab() { return (0..data::ARTICLES.len()).collect(); }
    (0..data::ENTRIES.len()).filter(|&index| usize::from(data::ENTRIES[index].tab) == tab).collect()
}

fn locate(id: &str) -> Option<(usize, usize)> {
    let entry = data::ENTRIES.iter().position(|entry| entry.id == id)?;
    let tab = usize::from(data::ENTRIES[entry].tab);
    Some((tab, list(tab).iter().position(|&index| index == entry)?))
}

fn tr(key: &str) -> String { Text::tr(key, &[]).plain() }

fn name(id: &str) -> String { tr(&format!("veio.wiki.name.{}", id.rsplit(':').next().unwrap_or(id))) }

/// Quebra o texto em linhas que cabem em `width`.
fn wrap(host: &dyn Host, text: &str, width: f32) -> Vec<String> {
    let mut lines = Vec::new();
    for paragraph in text.split('\n') {
        let mut line = String::new();
        for word in paragraph.split_whitespace() {
            let candidate = if line.is_empty() { word.to_owned() } else { format!("{line} {word}") };
            if !line.is_empty() && host.text_width(&candidate) > width {
                lines.push(std::mem::replace(&mut line, word.to_owned()));
            } else {
                line = candidate;
            }
        }
        lines.push(line);
    }
    lines
}

fn fit(host: &dyn Host, text: &str, width: f32) -> String {
    if host.text_width(text) <= width { return text.to_owned(); }
    let mut out = String::new();
    for ch in text.chars() {
        if host.text_width(&format!("{out}{ch}…")) > width { break; }
        out.push(ch);
    }
    out + "…"
}

fn inside([x, y, w, h]: [f32; 4], [mx, my]: [f32; 2]) -> bool { mx >= x && mx < x + w && my >= y && my < y + h }

/// Tecla J: abre ou fecha a wiki.
pub(crate) fn key(host: &mut dyn Host, key: &str, pressed: bool) {
    if !pressed || key != host.mod_key(MOD, KEY, DEFAULT_KEY) { return; }
    if host.mod_screen() == Some(MOD) {
        host.mod_screen_close(MOD);
    } else {
        host.mod_screen_open(MOD);
    }
}

/// Clique: botão esquerdo segue a área clicada; o direito volta à página anterior.
pub(crate) fn click(host: &dyn Host, screen: &NativeScreenView<'_>, button: i32) -> bool {
    if host.mod_screen() != Some(MOD) { return false; }
    let Ok(mut state) = STATE.lock() else { return true };
    if button == 1 {
        if let Some((tab, pick)) = state.history.pop() { open(&mut state, tab, pick, false); }
        return true;
    }
    let action = state.hits.iter().rev().find(|hit| inside(hit.rect, screen.mouse)).and_then(|hit| hit.action);
    match action {
        Some(Action::Tab(tab)) => { let current = (state.tab, state.pick); remember(&mut state, current); open(&mut state, tab, 0, true); }
        Some(Action::Pick(pick)) => { let current = (state.tab, state.pick); remember(&mut state, current); let tab = state.tab; open(&mut state, tab, pick, false); }
        Some(Action::Goto(id)) => {
            if let Some((tab, pick)) = locate(id) {
                let current = (state.tab, state.pick);
                remember(&mut state, current);
                open(&mut state, tab, pick, true);
            }
        }
        None => {}
    }
    true
}

fn remember(state: &mut State, at: (usize, usize)) {
    if state.history.last() != Some(&at) { state.history.push(at); }
    if state.history.len() > 32 { state.history.remove(0); }
}

fn open(state: &mut State, tab: usize, pick: usize, reveal: bool) {
    state.tab = tab.min(guide_tab());
    state.pick = pick;
    state.page_scroll = 0.0;
    if reveal { state.list_scroll = pick.saturating_sub(3); }
}

/// Roda do mouse: rola a lista (à esquerda) ou a página.
pub(crate) fn scroll(host: &dyn Host, screen: &NativeScreenView<'_>, amount: f64) -> bool {
    if host.mod_screen() != Some(MOD) { return false; }
    let Ok(mut state) = STATE.lock() else { return true };
    if screen.mouse[0] < state.list_right {
        let rows = list(state.tab).len();
        state.list_scroll = if amount > 0.0 { state.list_scroll.saturating_sub(1) } else { (state.list_scroll + 1).min(rows.saturating_sub(1)) };
    } else {
        state.page_scroll = (state.page_scroll - amount as f32 * 2.0 * LINE).clamp(0.0, state.page_max);
    }
    true
}

/// Desenha a wiki por cima da tela própria do mod.
pub(crate) fn draw(host: &dyn Host, batch: &mut HudBatch, screen: &NativeScreenView<'_>) {
    if host.mod_screen() != Some(MOD) { return; }
    let Ok(mut state) = STATE.lock() else { return };
    let mut hits = Vec::new();
    let [sx, sy, sw, sh] = screen.rect;
    let w = (sw - 16.0).clamp(240.0, 420.0);
    let h = (sh - 16.0).clamp(160.0, 270.0);
    let (x, y) = ((sx + (sw - w) / 2.0).floor(), (sy + (sh - h) / 2.0).floor());
    host.fill(batch, [sx, sy, sw, sh], BACKDROP);
    host.fill(batch, [x - 1.0, y - 1.0, w + 2.0, h + 2.0], BORDER);
    host.fill(batch, [x, y, w, h], PANEL);
    host.fill(batch, [x, y, w, 14.0], HEADER);
    host.text(batch, &tr("veio.wiki.title"), [x + 5.0, y + 3.0], GOLD, true);
    let hint = fit(host, &tr("veio.wiki.hint"), w - 80.0);
    host.text(batch, &hint, [x + w - 4.0 - host.text_width(&hint), y + 3.0], SOFT, false);
    // Abas.
    for (index, (title, icon)) in data::TABS.iter().enumerate() {
        let rect = [x + 4.0 + index as f32 * 24.0, y + 17.0, 22.0, 20.0];
        let on = index == state.tab;
        host.fill(batch, rect, if on { ROW_ON } else if inside(rect, screen.mouse) { ROW_HOVER } else { HEADER });
        host.item(batch, icon, [rect[0] + 3.0, rect[1] + 2.0], 16.0);
        hits.push(Hit { rect, action: Some(Action::Tab(index)), tip: Some(tr(title)) });
    }
    let tab_title = tr(data::TABS[state.tab].0);
    host.text(batch, &tab_title, [x + 8.0 + data::TABS.len() as f32 * 24.0, y + 23.0], WHITE, false);
    // Lista.
    let items = list(state.tab);
    state.pick = state.pick.min(items.len().saturating_sub(1));
    let (list_x, list_y, list_h) = (x + 4.0, y + 40.0, h - 44.0);
    state.list_right = list_x + LIST_W;
    let visible = (list_h / ROW).floor().max(1.0) as usize;
    state.list_scroll = state.list_scroll.min(items.len().saturating_sub(visible));
    host.fill(batch, [list_x, list_y, LIST_W, list_h], BORDER);
    for (row, &index) in items.iter().enumerate().skip(state.list_scroll).take(visible) {
        let rect = [list_x, list_y + (row - state.list_scroll) as f32 * ROW, LIST_W - 4.0, ROW];
        let picked = row == state.pick;
        if picked || inside(rect, screen.mouse) { host.fill(batch, rect, if picked { ROW_ON } else { ROW_HOVER }); }
        let (icon, label) = if state.tab == guide_tab() {
            let article = &data::ARTICLES[index];
            (article.icon, tr(&format!("veio.wiki.article.{}.title", article.id)))
        } else {
            (data::ENTRIES[index].id, name(data::ENTRIES[index].id))
        };
        host.item(batch, icon, [rect[0] + 1.0, rect[1] + 1.0], 16.0);
        host.text(batch, &fit(host, &label, LIST_W - 26.0), [rect[0] + 20.0, rect[1] + 5.0], WHITE, false);
        hits.push(Hit { rect, action: Some(Action::Pick(row)), tip: None });
    }
    if items.len() > visible {
        let bar = list_h * visible as f32 / items.len() as f32;
        let top = list_h * state.list_scroll as f32 / items.len() as f32;
        host.fill(batch, [list_x + LIST_W - 3.0, list_y + top, 2.0, bar], SOFT | 0xff000000);
    }
    // Página.
    let page = Page {
        host, batch, hits: &mut hits, mouse: screen.mouse,
        left: list_x + LIST_W + 6.0, width: w - LIST_W - 14.0, top: list_y, bottom: y + h - 4.0,
        cursor: list_y - state.page_scroll,
    };
    let height = if let Some(&index) = items.get(state.pick) {
        if state.tab == guide_tab() { article_page(page, &data::ARTICLES[index]) } else { entry_page(page, &data::ENTRIES[index]) }
    } else { 0.0 };
    state.page_max = (height - (h - 44.0)).max(0.0);
    state.page_scroll = state.page_scroll.min(state.page_max);
    // Dica do que está sob o cursor.
    if let Some(tip) = hits.iter().rev().find(|hit| inside(hit.rect, screen.mouse)).and_then(|hit| hit.tip.clone()) {
        host.gui_tooltip(batch, &[Text::literal(tip)], screen.mouse);
    }
    state.hits = hits;
}

/// A página em construção: o cursor desce e só desenha o que cabe entre `top` e `bottom`
/// (a tela não recorta, então o que sairia da área não é desenhado).
struct Page<'a> {
    host: &'a dyn Host,
    batch: &'a mut HudBatch,
    hits: &'a mut Vec<Hit>,
    mouse: [f32; 2],
    left: f32,
    width: f32,
    top: f32,
    bottom: f32,
    cursor: f32,
}

impl Page<'_> {
    fn shows(&self, height: f32) -> bool { self.cursor >= self.top && self.cursor + height <= self.bottom }

    fn text(&mut self, text: &str, color: u32) {
        for line in wrap(self.host, text, self.width) {
            if self.shows(LINE) { self.host.text(self.batch, &line, [self.left, self.cursor + 1.0], color, false); }
            self.cursor += LINE;
        }
    }

    fn heading(&mut self, text: &str) {
        self.cursor += 4.0;
        if self.shows(LINE + 2.0) {
            self.host.text(self.batch, text, [self.left, self.cursor + 1.0], GOLD, true);
            self.host.fill(self.batch, [self.left, self.cursor + LINE + 1.0, self.width, 1.0], 0xff5a5d66);
        }
        self.cursor += LINE + 4.0;
    }

    /// Um espaço de inventário com o item (clicável), na linha atual, em `x`.
    fn slot(&mut self, x: f32, y: f32, item: Option<&'static str>, count: u32) {
        if !(y >= self.top && y + SLOT <= self.bottom) { return; }
        self.host.fill(self.batch, [x, y, SLOT, SLOT], 0xff373737);
        self.host.fill(self.batch, [x + 1.0, y + 1.0, SLOT - 1.0, SLOT - 1.0], 0xffffffff);
        self.host.fill(self.batch, [x + 1.0, y + 1.0, SLOT - 2.0, SLOT - 2.0], 0xff8b8b8b);
        let Some(item) = item else { return };
        let rect = [x, y, SLOT, SLOT];
        if inside(rect, self.mouse) { self.host.fill(self.batch, [x + 1.0, y + 1.0, SLOT - 2.0, SLOT - 2.0], 0xffc5c5c5); }
        self.host.item(self.batch, item, [x + 1.0, y + 1.0], 16.0);
        if count > 1 {
            let label = count.to_string();
            self.host.text(self.batch, &label, [x + SLOT - self.host.text_width(&label), y + SLOT - 8.0], WHITE, true);
        }
        let action = locate(item).map(|_| Action::Goto(item));
        self.hits.push(Hit { rect, action, tip: Some(name(item)) });
    }

    /// Uma receita de máquina numa linha: [máquina :] entradas → saída · tempo e energia.
    fn process(&mut self, recipe: &'static production::Recipe, machine: bool) {
        let y = self.cursor;
        let mut x = self.left;
        if machine {
            self.slot(x, y, Some(recipe.machine), 1);
            x += SLOT + 6.0;
        }
        for input in recipe.inputs {
            self.slot(x, y, input.items.first().copied(), input.count);
            x += SLOT + 1.0;
        }
        if self.shows(SLOT) { self.host.text(self.batch, "→", [x + 2.0, y + 5.0], WHITE, false); }
        x += 12.0;
        self.slot(x, y, Some(recipe.output.0), recipe.output.1);
        x += SLOT + 4.0;
        if self.shows(SLOT) {
            let seconds = format!("{:.1}", recipe.ticks as f32 / 20.0).replace('.', ",");
            let label = Text::tr("veio.wiki.per", &[&seconds, &recipe.power]).plain();
            self.host.text(self.batch, &fit(self.host, &label, self.left + self.width - x), [x, y + 5.0], SOFT, false);
        }
        self.cursor += SLOT + 2.0;
    }
}

fn entry_page(mut page: Page<'_>, entry: &'static Entry) -> f32 {
    let start = page.cursor;
    if page.shows(34.0) {
        page.host.item(page.batch, entry.id, [page.left, page.cursor], 32.0);
        page.host.text(page.batch, &name(entry.id), [page.left + 38.0, page.cursor + 6.0], WHITE, true);
        page.host.text(page.batch, &tr(data::TABS[usize::from(entry.tab)].0), [page.left + 38.0, page.cursor + 18.0], SOFT, false);
    }
    page.cursor += 36.0;
    if !entry.unlock.is_empty() {
        let unlock = tr(entry.unlock);
        page.text(&Text::tr("veio.wiki.unlock", &[&unlock]).plain(), GREEN);
    }
    if entry.text.is_empty() {
        page.text(&tr("veio.wiki.vanilla"), SOFT);
    } else {
        // As dicas do "?" vêm em linhas curtas: aqui viram um parágrafo.
        page.text(&tr(entry.text).replace('\n', " "), 0xe6e6e6);
    }
    if !entry.craft.is_empty() {
        page.heading(&tr("veio.wiki.craft"));
        let y = page.cursor;
        for index in 0..9 {
            let (col, row) = ((index % 3) as f32, (index / 3) as f32);
            page.slot(page.left + col * SLOT, y + row * SLOT, entry.craft.get(index).copied(), 1);
        }
        if y + SLOT >= page.top && y + 2.0 * SLOT <= page.bottom {
            page.host.text(page.batch, "→", [page.left + 3.0 * SLOT + 8.0, y + SLOT + 5.0], WHITE, false);
        }
        page.slot(page.left + 3.0 * SLOT + 22.0, y + SLOT, Some(entry.id), entry.makes);
        page.cursor = y + 3.0 * SLOT + 4.0;
    }
    let made: Vec<_> = production::data::RECIPES.iter().filter(|recipe| recipe.output.0 == entry.id).collect();
    if !made.is_empty() {
        page.heading(&tr("veio.wiki.made_in"));
        for recipe in made.into_iter().take(8) { page.process(recipe, true); }
    }
    let processes: Vec<_> = production::data::RECIPES.iter().filter(|recipe| recipe.machine == entry.id).collect();
    if !processes.is_empty() {
        page.heading(&tr("veio.wiki.processes"));
        for recipe in processes.into_iter().take(24) { page.process(recipe, false); }
    }
    let used = used_in(entry.id);
    if !used.is_empty() {
        page.heading(&tr("veio.wiki.used_in"));
        let per_row = ((page.width / (SLOT + 1.0)).floor() as usize).max(1);
        for chunk in used.chunks(per_row) {
            let y = page.cursor;
            for (column, item) in chunk.iter().enumerate() { page.slot(page.left + column as f32 * (SLOT + 1.0), y, Some(*item), 1); }
            page.cursor += SLOT + 1.0;
        }
    }
    page.cursor - start + 4.0
}

/// O que usa o item: receitas de bancada e de máquina que o pedem, sem repetir.
fn used_in(id: &str) -> Vec<&'static str> {
    let mut out: Vec<&'static str> = Vec::new();
    for entry in data::ENTRIES.iter().filter(|entry| entry.craft.contains(&id)) {
        if !out.contains(&entry.id) { out.push(entry.id); }
    }
    for recipe in production::data::RECIPES.iter().filter(|recipe| recipe.inputs.iter().any(|input| input.items.contains(&id))) {
        if !out.contains(&recipe.output.0) { out.push(recipe.output.0); }
    }
    out
}

fn article_page(mut page: Page<'_>, article: &'static Article) -> f32 {
    let start = page.cursor;
    if page.shows(20.0) {
        page.host.item(page.batch, article.icon, [page.left, page.cursor], 16.0);
        page.host.text(page.batch, &tr(&format!("veio.wiki.article.{}.title", article.id)), [page.left + 20.0, page.cursor + 4.0], GOLD, true);
    }
    page.cursor += 20.0;
    for paragraph in tr(&format!("veio.wiki.article.{}.body", article.id)).split('\n') {
        page.text(paragraph, 0xe6e6e6);
        page.cursor += 3.0;
    }
    page.cursor - start
}
