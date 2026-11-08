//! Kit visual das telas do Veio: o mesmo painel cinza do Minecraft, com cabeçalho que diz o que
//! o bloco faz, faixa de estado com cor, palco do processo (entrada → máquina → saída), cartões de
//! números e uma linha que explica como e por quê. Cada família (indústria, energia, logística,
//! campo, vila, comércio...) tem sua cor e seus ícones, desenhados em `veio:gui/ui`
//! (`tools/build_ui_atlas.py`).
//!
//! As coordenadas são relativas ao canto do painel; `Pen` soma a origem. Quando o atlas falta
//! (`Host::sprite` devolve falso), as peças essenciais (seta, chama, barras) caem para retângulos.
use super::*;
use horizon_client::mod_api::{HudBatch, ScreenDrawEvent, Text};

// Gerado com todos os ícones do atlas; nem todos têm uso ainda.
#[allow(dead_code)]
pub(crate) mod icons;
pub(crate) mod spec;
pub(crate) mod screens;
pub(crate) mod custom;


#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Icon {
    pub u: f32,
    pub v: f32,
    pub w: f32,
    pub h: f32,
}

pub(crate) const ATLAS: &str = "veio:gui/ui";

// Paleta do painel vanilla.
pub(crate) const PANEL: u32 = 0xffc6c6c6;
pub(crate) const LIGHT: u32 = 0xffffffff;
pub(crate) const SHADOW: u32 = 0xff555555;
pub(crate) const EDGE: u32 = 0xff000000;
pub(crate) const SLOT: u32 = 0xff8b8b8b;
pub(crate) const SLOT_DARK: u32 = 0xff373737;
// Tintas de texto (RGB, sem alfa: `Host::text` é opaco).
pub(crate) const INK: u32 = 0x404040;
pub(crate) const SOFT: u32 = 0x5c5c5c;
pub(crate) const FAINT: u32 = 0x7c7c7c;
pub(crate) const GOOD: u32 = 0x1e7a1e;
pub(crate) const WARN: u32 = 0x9a5a00;
pub(crate) const BAD: u32 = 0xaa1f1f;
pub(crate) const INFO: u32 = 0x1f4fa8;

/// Largura e altura padrão das telas do Veio; o inventário começa em `INVENTORY_Y`.
pub(crate) const WIDTH: f32 = 194.0;
pub(crate) const HEIGHT: f32 = 236.0;
pub(crate) const INVENTORY_Y: f32 = 154.0;
pub(crate) const INVENTORY_X: f32 = 17.0;
/// Faixas verticais do layout padrão.
pub(crate) const STATUS_Y: f32 = 30.0;
pub(crate) const STAGE_Y: f32 = 44.0;
pub(crate) const CARDS_Y: f32 = 103.0;
pub(crate) const FOOTER_Y: f32 = 125.0;
/// Botão de ajuda no canto direito do cabeçalho.
pub(crate) const HELP: [f32; 4] = [177.0, 4.0, 12.0, 12.0];

/// Mistura duas cores RGB (`t` = 0 fica em `a`, 1 em `b`), mantendo o alfa de `a`.
pub(crate) fn mix(a: u32, b: u32, t: f32) -> u32 {
    let t = t.clamp(0.0, 1.0);
    let channel = |shift: u32| {
        let x = ((a >> shift) & 255) as f32;
        let y = ((b >> shift) & 255) as f32;
        ((x + (y - x) * t).round() as u32) << shift
    };
    (a & 0xff00_0000) | channel(16) | channel(8) | channel(0)
}

/// Cor opaca para `fill` a partir de um RGB.
pub(crate) const fn solid(rgb: u32) -> u32 { 0xff00_0000 | (rgb & 0x00ff_ffff) }

/// Lápis das telas: desenha em coordenadas do painel.
pub(crate) struct Pen<'a> {
    pub host: &'a dyn Host,
    pub batch: &'a mut HudBatch,
    pub origin: [f32; 2],
    /// Tempo do mundo em ticks, para as animações; 0 sem relógio.
    pub tick: i64,
    /// Hora do dia (0 a 24000), para o sol e a lua.
    pub day: i64,
}

impl<'a> Pen<'a> {
    pub(crate) fn new(host: &'a dyn Host, batch: &'a mut HudBatch, origin: [f32; 2]) -> Self {
        let (tick, day) = host.clock().map_or((0, 6000), |clock| (clock.game_time, clock.day_time));
        Self { host, batch, origin, tick, day }
    }

    pub(crate) fn from_event<'e: 'a>(host: &'a dyn Host, event: &'a mut ScreenDrawEvent<'e>) -> Self {
        let origin = event.origin;
        Self::new(host, &mut *event.batch, origin)
    }

    pub(crate) fn fill(&mut self, x: f32, y: f32, w: f32, h: f32, argb: u32) {
        if w <= 0.0 || h <= 0.0 { return; }
        let [ox, oy] = self.origin;
        self.host.fill(self.batch, [ox + x, oy + y, w, h], argb);
    }

    pub(crate) fn quad(&mut self, points: [[f32; 2]; 4], argb: u32) {
        let [ox, oy] = self.origin;
        self.host.quad(self.batch, points.map(|[x, y]| [ox + x, oy + y]), argb);
    }

    pub(crate) fn text(&mut self, text: &str, x: f32, y: f32, rgb: u32) {
        if text.is_empty() { return; }
        let [ox, oy] = self.origin;
        self.host.text(self.batch, text, [ox + x, oy + y], rgb, false);
    }

    /// Texto com a sombra do Minecraft: para letras claras sobre fundo escuro.
    pub(crate) fn shadow_text(&mut self, text: &str, x: f32, y: f32, rgb: u32) {
        if text.is_empty() { return; }
        let [ox, oy] = self.origin;
        self.host.text(self.batch, text, [ox + x, oy + y], rgb, true);
    }

    pub(crate) fn width(&self, text: &str) -> f32 { self.host.text_width(text) }

    pub(crate) fn text_right(&mut self, text: &str, right: f32, y: f32, rgb: u32) {
        let width = self.width(text);
        self.text(text, right - width, y, rgb);
    }

    pub(crate) fn text_center(&mut self, text: &str, center: f32, y: f32, rgb: u32) {
        let width = self.width(text);
        self.text(text, (center - width / 2.0).round(), y, rgb);
    }

    /// Corta com reticências o que não cabe em `width`.
    pub(crate) fn fit(&self, text: &str, width: f32) -> String {
        if self.width(text) <= width { return text.to_owned(); }
        let mut cut = text.to_owned();
        while !cut.is_empty() && self.width(&format!("{cut}…")) > width { cut.pop(); }
        format!("{}…", cut.trim_end())
    }

    /// Quebra por palavras em linhas de até `width` (e nas quebras do próprio texto).
    pub(crate) fn wrap(&self, text: &str, width: f32) -> Vec<String> {
        let mut lines = Vec::new();
        for paragraph in text.split('\n') {
            let mut line = String::new();
            for word in paragraph.split(' ').filter(|word| !word.is_empty()) {
                let candidate = if line.is_empty() { word.to_owned() } else { format!("{line} {word}") };
                if !line.is_empty() && self.width(&candidate) > width {
                    lines.push(std::mem::take(&mut line));
                    line = word.to_owned();
                } else {
                    line = candidate;
                }
            }
            lines.push(line);
        }
        lines
    }

    /// Escreve `text` quebrado em até `max` linhas a partir de `y`; a última leva reticências.
    pub(crate) fn paragraph(&mut self, text: &str, x: f32, y: f32, width: f32, max: usize, rgb: u32) -> usize {
        let mut lines = self.wrap(text, width);
        if lines.len() > max {
            let rest = lines[max - 1..].join(" ");
            lines.truncate(max - 1);
            lines.push(self.fit(&rest, width));
        }
        for (index, line) in lines.iter().enumerate() {
            self.text(line, x, y + index as f32 * 9.0, rgb);
        }
        lines.len()
    }

    /// Moldura em relevo: clara em cima e à esquerda quando saliente, escura quando rebaixada.
    pub(crate) fn bevel(&mut self, x: f32, y: f32, w: f32, h: f32, raised: bool, fill: u32) {
        let (top, bottom) = if raised { (LIGHT, SHADOW) } else { (SLOT_DARK, LIGHT) };
        self.fill(x, y, w, h, fill);
        self.fill(x, y, w - 1.0, 1.0, top);
        self.fill(x, y, 1.0, h - 1.0, top);
        self.fill(x + 1.0, y + h - 1.0, w - 1.0, 1.0, bottom);
        self.fill(x + w - 1.0, y + 1.0, 1.0, h - 1.0, bottom);
    }

    /// Slot vanilla com o item em (`x`, `y`).
    pub(crate) fn slot(&mut self, x: f32, y: f32) {
        self.bevel(x - 1.0, y - 1.0, 18.0, 18.0, false, SLOT);
    }

    /// Slot grande de resultado, como o da bancada, com o item em (`x`, `y`).
    pub(crate) fn result_slot(&mut self, x: f32, y: f32, accent: u32) {
        let _ = accent;
        self.bevel(x - 5.0, y - 5.0, 26.0, 26.0, false, SLOT);
    }

    /// Ícone-fantasma num slot vazio, como as peças da mesa de ferraria.
    pub(crate) fn ghost(&mut self, item: &str, x: f32, y: f32) {
        let [ox, oy] = self.origin;
        if self.host.item(self.batch, item, [ox + x, oy + y], 16.0) {
            self.fill(x, y, 16.0, 16.0, 0xa08b8b8b);
        }
    }

    /// Ícone de item em qualquer tamanho (16 é o do slot).
    pub(crate) fn item(&mut self, item: &str, x: f32, y: f32, size: f32) -> bool {
        let [ox, oy] = self.origin;
        self.host.item(self.batch, item, [ox + x, oy + y], size)
    }

    /// Ícone do atlas no tamanho natural; falso sem o atlas.
    pub(crate) fn icon(&mut self, icon: Icon, x: f32, y: f32) -> bool {
        let [ox, oy] = self.origin;
        self.host.sprite(self.batch, ATLAS, [icon.u, icon.v, icon.w, icon.h], [ox + x, oy + y, icon.w, icon.h])
    }

    /// Ícone centrado em (`cx`, `cy`).
    pub(crate) fn icon_at_center(&mut self, icon: Icon, cx: f32, cy: f32) -> bool {
        self.icon(icon, (cx - icon.w / 2.0).round(), (cy - icon.h / 2.0).round())
    }

    /// A parte esquerda (`horizontal`) ou de baixo do ícone, na fração `part`: o enchimento da
    /// seta e da chama.
    pub(crate) fn icon_part(&mut self, icon: Icon, x: f32, y: f32, part: f32, horizontal: bool) -> bool {
        let part = part.clamp(0.0, 1.0);
        if part <= 0.0 { return true; }
        let [ox, oy] = self.origin;
        if horizontal {
            let w = (icon.w * part).ceil();
            self.host.sprite(self.batch, ATLAS, [icon.u, icon.v, w, icon.h], [ox + x, oy + y, w, icon.h])
        } else {
            let h = (icon.h * part).ceil();
            let top = icon.h - h;
            self.host.sprite(self.batch, ATLAS, [icon.u, icon.v + top, icon.w, h], [ox + x, oy + y + top, icon.w, h])
        }
    }

    /// Barra rebaixada com preenchimento proporcional e brilho em cima.
    pub(crate) fn bar(&mut self, x: f32, y: f32, w: f32, h: f32, fraction: f32, color: u32) {
        self.bevel(x, y, w, h, false, 0xff2b2b2b);
        let inner = ((w - 2.0) * fraction.clamp(0.0, 1.0)).round();
        if inner > 0.0 {
            self.fill(x + 1.0, y + 1.0, inner, h - 2.0, solid(color));
            self.fill(x + 1.0, y + 1.0, inner, 1.0, 0x50ffffff);
            self.fill(x + 1.0, y + h - 2.0, inner, 1.0, 0x30000000);
        }
    }

    /// Seta de progresso 22x15, vazia e enchendo da esquerda.
    pub(crate) fn arrow(&mut self, x: f32, y: f32, fraction: f32) {
        if self.icon(icons::ARROW_EMPTY, x, y) {
            self.icon_part(icons::ARROW_FULL, x, y, fraction, true);
            return;
        }
        self.fill(x, y + 5.0, 14.0, 5.0, SLOT);
        self.fill(x, y + 5.0, 14.0 * fraction.clamp(0.0, 1.0), 5.0, LIGHT);
        for step in 0..8 {
            let s = step as f32;
            let color = if fraction * 22.0 > 14.0 + s { LIGHT } else { SLOT };
            self.fill(x + 14.0 + s, y + s, 1.0, 15.0 - 2.0 * s, color);
        }
    }

    /// Chama da queima 13x13, enchendo de baixo para cima.
    pub(crate) fn burn(&mut self, x: f32, y: f32, fraction: f32) {
        if self.icon(icons::BURN_EMPTY, x, y) {
            self.icon_part(icons::BURN_FULL, x, y, fraction, false);
            return;
        }
        self.fill(x + 3.0, y, 7.0, 13.0, SLOT);
        let h = 13.0 * fraction.clamp(0.0, 1.0);
        self.fill(x + 3.0, y + 13.0 - h, 7.0, h, 0xffe8761c);
    }

    /// Lâmpada de estado com brilho, 7x7.
    pub(crate) fn lamp(&mut self, x: f32, y: f32, rgb: u32) {
        self.fill(x, y, 7.0, 7.0, EDGE);
        self.fill(x + 1.0, y + 1.0, 5.0, 5.0, solid(rgb));
        self.fill(x + 1.0, y + 1.0, 2.0, 2.0, 0x90ffffff);
        self.fill(x + 1.0, y + 5.0, 5.0, 1.0, 0x50000000);
    }

    /// Botão vanilla saliente com um rótulo curto.
    pub(crate) fn button(&mut self, x: f32, y: f32, w: f32, h: f32, label: &str, hover: bool) {
        self.fill(x, y, w, h, EDGE);
        let face = if hover { 0xffb8c4e8 } else { 0xffa8a8a8 };
        self.bevel(x + 1.0, y + 1.0, w - 2.0, h - 2.0, true, face);
        let width = self.width(label);
        self.text(label, (x + (w - width) / 2.0).round(), y + (h - 8.0) / 2.0, INK);
    }

    /// Onde está o mouse, relativo ao painel.
    pub(crate) fn mouse(&self) -> [f32; 2] {
        let [mx, my] = self.host.cursor();
        [mx - self.origin[0], my - self.origin[1]]
    }

    pub(crate) fn hovering(&self, x: f32, y: f32, w: f32, h: f32) -> bool {
        let [mx, my] = self.mouse();
        (x..x + w).contains(&mx) && (y..y + h).contains(&my)
    }
}

/// Tradução simples.
pub(crate) fn tr(key: &str, args: &[&dyn std::fmt::Display]) -> String { Text::tr(key, args).plain() }

/// Tradução que some quando a chave não existe (o jogo devolve a própria chave).
pub(crate) fn tr_opt(key: &str, args: &[&dyn std::fmt::Display]) -> Option<String> {
    let text = Text::tr(key, args).plain();
    (!text.is_empty() && text != key).then_some(text)
}

/// Número com separador de milhar no estilo brasileiro (1.500).
pub(crate) fn thousands(value: i64) -> String {
    let digits = value.unsigned_abs().to_string();
    let mut out = String::new();
    for (index, c) in digits.chars().enumerate() {
        if index > 0 && (digits.len() - index) % 3 == 0 { out.push('.'); }
        out.push(c);
    }
    if value < 0 { format!("-{out}") } else { out }
}

/// Segundos como `m:ss` (ou `s s` abaixo de um minuto).
pub(crate) fn clock_text(seconds: i32) -> String {
    let seconds = seconds.max(0);
    if seconds < 60 { format!("{seconds} s") } else { format!("{}:{:02}", seconds / 60, seconds % 60) }
}

/// Nome traduzido de um item ou bloco.
pub(crate) fn item_name(item: &str) -> String {
    let kind = if item.starts_with("veio:") && is_factory_block(item) { "block" }
        else if item.starts_with("minecraft:") && vanilla_block(item) { "block" } else { "item" };
    tr(&format!("{kind}.{}", item.replace(':', ".")), &[])
}

/// Itens do Minecraft citados nas telas que são blocos (a chave de tradução é `block.`).
fn vanilla_block(item: &str) -> bool {
    let name = &item[10..];
    name.ends_with("_log") || name.ends_with("_wool") || name.ends_with("_sapling") || name.ends_with("_planks")
        || matches!(name, "cobblestone" | "gravel" | "sand" | "hay_block" | "chest" | "barrel" | "dirt" | "stone"
            | "torch" | "furnace" | "crafting_table" | "short_grass" | "fern" | "kelp" | "cactus" | "bamboo" | "vine")
}
