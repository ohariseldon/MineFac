//! Compila `native/api/src` contra o substituto local da API do Horizon.
#[path = "../../../api/src/lib.rs"]
mod veio;
pub use veio::hooks;

/// Prévia dos painéis: `python tools/preview_screens.py` escreve os casos e as traduções e roda
/// `cargo test -p veio-check preview -- --ignored`, que grava `target/screens.jsonl` com os
/// retângulos, textos, sprites e ícones que o mod desenha. Sem os arquivos do script, usa
/// alguns casos fixos e as chaves sem tradução.
#[cfg(test)]
mod preview {
    use horizon_client::mod_api::{Host, HudBatch, ScreenDrawEvent};
    use std::cell::RefCell;
    use std::io::Write;

    #[derive(Default)]
    struct Recorder {
        ops: RefCell<Vec<String>>,
        cursor: [f32; 2],
        tick: i64,
    }

    fn quote(text: &str) -> String {
        let mut out = String::from("\"");
        for c in text.chars() {
            match c {
                '"' => out.push_str("\\\""),
                '\\' => out.push_str("\\\\"),
                '\n' => out.push_str("\\n"),
                c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
                c => out.push(c),
            }
        }
        out.push('"');
        out
    }

    /// Larguras da fonte padrão do Minecraft (avanço com o espaço entre letras).
    pub(super) fn glyph_width(c: char) -> f32 {
        match c {
            ' ' => 4.0,
            '!' | '\'' | ',' | '.' | ':' | ';' | '|' | 'i' | '·' => 2.0,
            '`' | 'l' | 'í' | 'ì' => 3.0,
            '"' | '(' | ')' | '*' | 'I' | 'Í' | '[' | ']' | 't' | '{' | '}' => 4.0,
            '<' | '>' | 'f' | 'k' => 5.0,
            '@' | '~' => 7.0,
            '—' | '→' | '←' | '⚡' => 8.0,
            _ => 6.0,
        }
    }

    impl Host for Recorder {
        fn fill(&self, _: &mut HudBatch, [x, y, w, h]: [f32; 4], argb: u32) {
            self.ops.borrow_mut().push(format!(r#"{{"fill":[{x},{y},{w},{h}],"argb":{argb}}}"#));
        }
        fn text(&self, _: &mut HudBatch, text: &str, [x, y]: [f32; 2], rgb: u32, shadow: bool) {
            self.ops.borrow_mut().push(format!(r#"{{"text":{},"at":[{x},{y}],"rgb":{rgb},"shadow":{shadow}}}"#, quote(text)));
        }
        fn text_width(&self, text: &str) -> f32 {
            text.chars().map(glyph_width).sum()
        }
        fn sprite(&self, _: &mut HudBatch, name: &str, src: [f32; 4], dst: [f32; 4]) -> bool {
            self.ops.borrow_mut().push(format!(r#"{{"sprite":{},"src":{src:?},"dst":{dst:?}}}"#, quote(name)));
            true
        }
        fn item(&self, _: &mut HudBatch, item: &str, [x, y]: [f32; 2], size: f32) -> bool {
            self.ops.borrow_mut().push(format!(r#"{{"item":{},"at":[{x},{y}],"size":{size}}}"#, quote(item)));
            true
        }
        fn quad(&self, _: &mut HudBatch, points: [[f32; 2]; 4], argb: u32) {
            let points: Vec<String> = points.iter().map(|[x, y]| format!("[{x},{y}]")).collect();
            self.ops.borrow_mut().push(format!(r#"{{"quad":[{}],"argb":{argb}}}"#, points.join(",")));
        }
        fn cursor(&self) -> [f32; 2] {
            self.cursor
        }
        fn clock(&self) -> Option<horizon_client::mod_api::NativeClock> {
            Some(horizon_client::mod_api::NativeClock { game_time: self.tick, day_time: (self.tick + 6000).rem_euclid(24000) })
        }
    }

    /// `chave\x1fvalor\x1e...`, escrito pelo script a partir de `lang/*.json`.
    fn language(path: &std::path::Path) -> Option<std::collections::HashMap<String, String>> {
        let raw = std::fs::read_to_string(path).ok()?;
        Some(raw.split('\x1e').filter_map(|record| record.split_once('\x1f'))
            .map(|(key, value)| (key.to_owned(), value.to_owned())).collect())
    }

    /// Uma linha por caso: `rótulo bloco d0 d1 ... [@mx,my] [tN] [slot=item*n]` (mouse para a
    /// dica, o tempo do mundo para as animações; os itens nos slots o script pinta por cima).
    fn cases(path: &std::path::Path) -> Option<Vec<(String, String, Vec<i32>, [f32; 2], i64)>> {
        let raw = std::fs::read_to_string(path).ok()?;
        Some(raw.lines().filter(|line| !line.trim().is_empty() && !line.starts_with('#')).map(|line| {
            let mut words = line.split_whitespace();
            let label = words.next().unwrap().to_owned();
            let block = words.next().unwrap().to_owned();
            let mut data = Vec::new();
            let mut mouse = [-100.0, -100.0];
            let mut tick = 0;
            for word in words {
                if let Some((mx, my)) = word.strip_prefix('@').and_then(|at| at.split_once(',')) {
                    mouse = [mx.parse().unwrap(), my.parse().unwrap()];
                } else if let Some(time) = word.strip_prefix('t') {
                    tick = time.parse().unwrap();
                } else if !word.contains('=') {
                    data.push(word.parse::<i64>().unwrap() as i32);
                }
            }
            (label, block, data, mouse, tick)
        }).collect())
    }

    #[test]
    #[ignore]
    fn preview() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../target");
        let lang = language(&root.join("preview_lang.txt"));
        horizon_client::mod_api::preview_language(lang);
        let fallback = vec![
            ("kitchen".to_owned(), "veio:kitchen".to_owned(), vec![1, 30, 80, 1500, 0, 2000, 12, 70, 0, 40, 0, 2, 0, 0], [-100.0, -100.0], 0),
            ("barn".to_owned(), "veio:barn".to_owned(), vec![0, 0, 1, 0, 0, 2000, 12, 27, 0, 0, 0, 0, 0, 0], [-100.0, -100.0], 0),
            ("manufacturer".to_owned(), "veio:manufacturer".to_owned(), vec![1, 90, 240, 1500, 0, 2000, 4, 80, 5, 18, 0, 0, 5 * 64, 0], [-100.0, -100.0], 0),
        ];
        let screens = cases(&root.join("preview_cases.txt")).unwrap_or(fallback);
        let path = root.join("screens.jsonl");
        let mut out = std::fs::File::create(&path).unwrap();
        let hooks = crate::hooks();
        for (label, block, data, mouse, tick) in screens {
            let host = Recorder { cursor: mouse, tick, ..Recorder::default() };
            let mut batch = HudBatch;
            let mut event = ScreenDrawEvent::preview(&block, &data, &mut batch);
            hooks.screen_draw(&host, &mut event);
            let (size, slots) = {
                let mut event = horizon_client::mod_api::ScreenEvent::preview(&block);
                hooks.screen(&host, &mut event).map_or(([0.0, 0.0], String::new()), |screen| {
                    let slots: Vec<String> = screen.slots.iter()
                        .map(|slot| slot.map_or("null".to_owned(), |[x, y]| format!("[{x},{y}]"))).collect();
                    ([screen.width, screen.height], slots.join(","))
                })
            };
            let mut tooltip = Vec::new();
            if mouse[0] >= 0.0 {
                let mut event = horizon_client::mod_api::ScreenTooltipEvent::preview(&block, &data, mouse, &mut tooltip);
                hooks.screen_tooltip(&host, &mut event);
            }
            let tooltip: Vec<String> = tooltip.iter().map(|line| quote(line)).collect();
            writeln!(out, r#"{{"label":{},"block":{},"size":{size:?},"slots":[{slots}],"mouse":{mouse:?},"tooltip":[{}],"ops":[{}]}}"#,
                quote(&label), quote(&block), tooltip.join(","), host.ops.borrow().join(",")).unwrap();
        }
        horizon_client::mod_api::preview_language(None);
    }
}
