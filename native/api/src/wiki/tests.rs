use super::*;
use std::cell::{Cell, RefCell};

/// Janela de mentira: guarda o que foi desenhado e se a tela do mod está aberta.
#[derive(Default)]
struct Window {
    open: Cell<bool>,
    icons: RefCell<Vec<(String, [f32; 2])>>,
    texts: RefCell<Vec<String>>,
    tips: RefCell<Vec<String>>,
}

impl Host for Window {
    fn fill(&self, _: &mut HudBatch, _: [f32; 4], _: u32) {}
    fn text(&self, _: &mut HudBatch, text: &str, _: [f32; 2], _: u32, _: bool) { self.texts.borrow_mut().push(text.to_owned()); }
    fn text_width(&self, text: &str) -> f32 { text.chars().count() as f32 * 6.0 }
    fn item(&self, _: &mut HudBatch, item: &str, at: [f32; 2], _: f32) -> bool {
        self.icons.borrow_mut().push((item.to_owned(), at));
        true
    }
    fn gui_tooltip(&self, _: &mut HudBatch, lines: &[Text], _: [f32; 2]) {
        self.tips.borrow_mut().extend(lines.iter().map(Text::plain));
    }
    fn mod_screen_open(&mut self, mod_id: &str) -> bool { assert_eq!(mod_id, "veio"); self.open.set(true); true }
    fn mod_screen_close(&mut self, _: &str) { self.open.set(false); }
    fn mod_screen(&self) -> Option<&str> { self.open.get().then_some("veio") }
    fn mod_key(&self, _: &str, _: &str, default: &str) -> String { default.to_owned() }
}

fn view(mouse: [f32; 2]) -> NativeScreenView<'static> {
    NativeScreenView { id: "mod", menu: None, title: None, rect: [0.0, 0.0, 480.0, 300.0], slots: 0, mouse, partial_tick: 0.0 }
}

fn frame(window: &Window, mouse: [f32; 2]) {
    window.icons.borrow_mut().clear();
    window.texts.borrow_mut().clear();
    window.tips.borrow_mut().clear();
    draw(window, &mut HudBatch::default(), &view(mouse));
}

fn center_of(target: impl Fn(&Hit) -> bool) -> [f32; 2] {
    let state = STATE.lock().unwrap();
    let hit = state.hits.iter().find(|hit| target(hit)).expect("área clicável");
    [hit.rect[0] + hit.rect[2] / 2.0, hit.rect[1] + hit.rect[3] / 2.0]
}

fn page_id() -> &'static str {
    let state = STATE.lock().unwrap();
    data::ENTRIES[list(state.tab)[state.pick]].id
}

/// Tudo num teste só: o estado da wiki é um só para a janela.
#[test]
fn the_wiki_opens_with_j_shows_recipes_and_navigates_between_items() {
    let mut window = Window::default();
    key(&mut window, "key.keyboard.k", true);
    assert!(!window.open.get());
    key(&mut window, "key.keyboard.j", true);
    assert!(window.open.get(), "J abre a wiki");
    // Todas as abas e páginas desenham sem passar da área.
    for tab in 0..data::TABS.len() {
        for pick in 0..list(tab).len() {
            {
                let mut state = STATE.lock().unwrap();
                state.tab = tab;
                state.pick = pick;
                state.page_scroll = 0.0;
            }
            frame(&window, [0.0, 0.0]);
            let state = STATE.lock().unwrap();
            assert!(state.hits.iter().all(|hit| hit.rect[1] >= 0.0 && hit.rect[1] + hit.rect[3] <= 300.0));
        }
    }
    // A prensa: a grade da bancada com os ingredientes e o que ela processa.
    {
        let mut state = STATE.lock().unwrap();
        let (tab, pick) = locate("veio:press").unwrap();
        state.tab = tab;
        state.pick = pick;
        state.history.clear();
    }
    frame(&window, [0.0, 0.0]);
    let icons: Vec<String> = window.icons.borrow().iter().map(|(item, _)| item.clone()).collect();
    for item in ["minecraft:copper_ingot", "minecraft:iron_ingot", "minecraft:piston", "minecraft:smooth_stone", "veio:copper_plate"] {
        assert!(icons.iter().any(|icon| icon == item), "{item} na página da prensa: {icons:?}");
    }
    assert!(window.texts.borrow().iter().any(|text| text.contains("veio.wiki.processes")));
    // Clicar na chapa de cobre (produto da prensa) abre a página dela; o botão direito volta.
    let plate = center_of(|hit| hit.action == Some(Action::Goto("veio:copper_plate")));
    frame(&window, plate);
    assert!(window.tips.borrow().iter().any(|tip| tip.contains("veio.wiki.name.copper_plate")), "dica com o nome");
    assert!(click(&window, &view(plate), 0));
    assert_eq!(page_id(), "veio:copper_plate");
    frame(&window, [0.0, 0.0]);
    assert!(window.texts.borrow().iter().any(|text| text.contains("veio.wiki.made_in")), "a chapa mostra onde é feita");
    assert!(click(&window, &view([0.0, 0.0]), 1));
    assert_eq!(page_id(), "veio:press");
    // Item do Minecraft sem página: só a dica, o clique não navega.
    let ingot = center_of(|hit| hit.tip.as_deref() == Some("veio.wiki.name.copper_ingot"));
    assert!(click(&window, &view(ingot), 0));
    assert_eq!(page_id(), "veio:press");
    // Abas e rolagem da lista.
    let systems = center_of(|hit| hit.action == Some(Action::Tab(guide_tab())));
    click(&window, &view(systems), 0);
    frame(&window, [0.0, 0.0]);
    assert!(window.texts.borrow().iter().any(|text| text.contains("veio.wiki.article.start.title")));
    let list_point = [STATE.lock().unwrap().list_right - 10.0, 100.0];
    scroll(&window, &view(list_point), -1.0);
    assert_eq!(STATE.lock().unwrap().list_scroll, 1);
    scroll(&window, &view(list_point), 1.0);
    assert_eq!(STATE.lock().unwrap().list_scroll, 0);
    // Com a wiki fechada, os ganchos de tela não tomam o clique.
    key(&mut window, "key.keyboard.j", true);
    assert!(!window.open.get());
    assert!(!click(&window, &view(systems), 0));
    assert!(!scroll(&window, &view(systems), 1.0));
}

#[test]
fn every_item_and_recipe_in_the_wiki_has_a_page_or_a_name() {
    for entry in data::ENTRIES {
        assert!(locate(entry.id).is_some(), "{} sem página", entry.id);
        assert!(entry.craft.len() <= 9, "{}: a grade tem 9 espaços", entry.id);
        assert!(entry.craft.is_empty() || entry.makes > 0);
    }
    assert!(locate("veio:press").is_some() && locate("minecraft:copper_ingot").is_none());
    // Usado em: a chapa de cobre entra na engrenagem, na fiação e no montador.
    let used = used_in("veio:copper_plate");
    for item in ["veio:gear", "veio:wire", "veio:assembler"] {
        assert!(used.contains(&item), "{item} usa chapa de cobre: {used:?}");
    }
}
