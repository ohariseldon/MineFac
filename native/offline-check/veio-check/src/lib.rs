//! Compila `native/api/src` contra o substituto local da API do Horizon.
#[path = "../../../api/src/lib.rs"]
mod veio;
pub use veio::hooks;

/// Prévia dos painéis: `cargo test -p veio-check preview -- --ignored` grava
/// `target/screens.jsonl` com os retângulos e textos que o mod desenha.
#[cfg(test)]
mod preview {
    use horizon_client::mod_api::{Host, HudBatch, ScreenDrawEvent};
    use std::cell::RefCell;
    use std::io::Write;

    #[derive(Default)]
    struct Recorder {
        ops: RefCell<Vec<String>>,
    }
    impl Host for Recorder {
        fn fill(&self, _: &mut HudBatch, [x, y, w, h]: [f32; 4], argb: u32) {
            self.ops.borrow_mut().push(format!(r#"{{"fill":[{x},{y},{w},{h}],"argb":{argb}}}"#));
        }
        fn text(&self, _: &mut HudBatch, text: &str, [x, y]: [f32; 2], rgb: u32, _: bool) {
            let text = text.replace('\\', "\\\\").replace('"', "\\\"");
            self.ops.borrow_mut().push(format!(r#"{{"text":"{text}","at":[{x},{y}],"rgb":{rgb}}}"#));
        }
        fn text_width(&self, text: &str) -> f32 {
            text.chars().count() as f32 * 6.0
        }
    }

    #[test]
    #[ignore]
    fn preview() {
        let screens: [(&str, [i32; 10]); 6] = [
            ("veio:press", [1, 18, 30, 1500, 0, 2000, 24, 87, 0, 120]),
            ("veio:alloy_furnace", [2, 0, 120, 600, 0, 2000, 6, 40, 0, 12]),
            ("veio:harvester", [1, 120, 243, 1300, 0, 2000, 14, 60, 0, 52]),
            ("veio:tesla", [11, 0, 40, 18000, 0, 20000, 0, 0, 0, 7]),
            ("veio:generator", [1, 210, 400, 1200, 210, 2000, 0, 0, 0, 0]),
            ("veio:contract_hub", [2, 0, 1, 0, 0, 2000, 0, 0, 4, 3]),
        ];
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../target/screens.jsonl");
        let mut out = std::fs::File::create(&path).unwrap();
        let hooks = crate::hooks();
        for (block, data) in screens {
            let host = Recorder::default();
            let mut batch = HudBatch;
            let mut event = ScreenDrawEvent::preview(block, &data, &mut batch);
            hooks.screen_draw(&host, &mut event);
            writeln!(out, r#"{{"block":"{block}","ops":[{}]}}"#, host.ops.borrow().join(",")).unwrap();
        }
    }
}
