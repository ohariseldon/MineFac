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
        let screens: [(&str, [i32; 13]); 20] = [
            ("veio:kitchen", [1, 30, 80, 1500, 0, 2000, 12, 70, 0, 40, 0, 2, 0]),
            ("veio:drill_mk3", [1, 4, 5, 1800, 0, 2000, 240, 98, 0, 900, 0, 1, 3]),
            ("veio:ranch", [1, 0, 1, 900, 0, 2000, 3, 40, 0, 12, 0, 0, 7]),
            ("veio:pantry", [1, 42, 60, 0, 0, 2000, 0, 0, 0, 5, 0, 0, 4]),
            ("veio:wind_turbine", [1, 0, 1, 1200, 0, 2000, 0, 0, 0, 0, 0, 0, 16]),
            ("veio:steam_engine", [14, 0, 266, 0, 0, 2000, 0, 0, 0, 0, 30, 0, 0]),
            ("veio:farm_post", [23, 0, 1, 0, 0, 2000, 0, 0, 0, 12, 0, 0, 5]),
            ("veio:hauler_post", [24, 0, 1, 0, 0, 2000, 3, 0, 0, 40, 0, 0, 7]),
            ("veio:foreman_desk", [25, 0, 1, 0, 0, 2000, 4, 3, 0, 9, 0, 0, 11]),
            ("veio:shepherd_post", [17, 0, 1, 0, 0, 2000, 0, 0, 0, 6, 0, 0, 4]),
            ("veio:control_center", [1, 786441, 131073, 196609, 1, 54, 4, 2687038, 96, 62, 100, -1, 0]),
            ("veio:control_center", [1, 4, 1106661461, 1182388371, 1117954322, 1099532712, 0, 0, 0, 0, 0, 0, 1]),
            ("veio:control_center", [1, 228, 228, 8192, 8754, 524288, 8401182, 8404992, 20979712, 29364224, 3, 180, 2]),
            ("veio:control_center", [1, 180, 66, 37, 82945, 98699, 211, 1550, 2, 0, 0, 0, 3]),
            ("veio:site", [34, 512, 1271, 0, 0, 0, 0, 0, 2, 380, 0, 0, 12]),
            ("veio:site", [33, 0, 1271, 0, 0, 0, 3, 128 | (129 << 8) | (126 << 16), 1, 0, 0, 0, 0]),
            ("veio:home", [1, 72, 95, 2, 215, 5, 24, 40, 97, 3, 3, 141630, 0]),
            ("veio:field_marker", [11, 2, 2, 13, 140, 128, 14, 12, 16, 140, 0, 0, 0]),
            ("veio:barn", [0, 0, 1, 0, 0, 2000, 12, 27, 0, 0, 0, 0, 0]),
            ("veio:silo", [3, 0, 1, 0, 0, 2000, 18, 18, 0, 0, 0, 0, 0]),
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
