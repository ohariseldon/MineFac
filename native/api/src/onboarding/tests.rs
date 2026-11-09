use super::*;

#[test]
#[ignore = "Uses the global onboarding session; run separately with --test-threads=1"]
fn board_alerts_yield_to_the_visible_tutorial_goal() {
    use horizon_client::mod_api::NativeServerPlayer;
    struct Player { bars: usize }
    impl Host for Player {
        fn server_players(&self) -> Vec<NativeServerPlayer> {
            vec![NativeServerPlayer { uuid: 91001, name: "Tutorial".into(), permission: 0,
                dimension: "veio:tutorial-board".into(), position: [0.0, 64.0, 0.0],
                game_mode: 0, health: 20.0 }]
        }
        fn day_time(&self) -> i64 { 1000 }
        fn action_bar(&mut self, _: u128, _: Text) -> bool { self.bars += 1; true }
    }
    let mut player = Player { bars: 0 };
    *SESSION.lock().unwrap() = Some(Session { root: PathBuf::new(),
        players: HashMap::from([(91001, Progress::default())]), pending: Vec::new() });
    super::super::milestones::census::reset();
    let mut press = ModBlockEntity::default(); press.id = PRESS.into();
    set_number(&mut press, "Status", 13);
    super::super::milestones::census::observe("veio:tutorial-board", [0, 64, 0], &press, 0, 0, 0);
    assert!(visible_goal(91001));
    super::super::milestones::census::tick(&mut player, 100);
    assert_eq!(player.bars, 0, "The tutorial keeps its single action-bar goal");
    SESSION.lock().unwrap().as_mut().unwrap().players.get_mut(&91001).unwrap().hidden = true;
    super::super::milestones::census::tick(&mut player, 200);
    assert_eq!(player.bars, 1, "Factory alerts become available when the tutorial is hidden");
    *SESSION.lock().unwrap() = None;
}

const GEN: Pos = [0, 64, 0];
const PRESS_AT: Pos = [2, 64, 0];
const DRILL_AT: Pos = [8, 64, 0];
const BELT_AT: Pos = [7, 64, 0];
const OUT: Pos = [3, 64, 0];
const CHEST: Pos = [4, 64, 0];

fn machine(id: &str, output: Option<(&str, u32)>, produced: i32) -> ModBlockEntity {
    let mut e = ModBlockEntity::default();
    e.id = id.to_owned();
    e.items = if id == PRESS || id == ASSEMBLER { vec![None, None] } else { vec![None] };
    let last = e.items.len() - 1;
    e.items[last] = output.map(|(item, count)| ItemStack::new(item, count));
    set_number(&mut e, "Produced", produced);
    e
}

fn line(step: u8) -> Progress {
    let mut p = Progress::default();
    p.bind("minecraft:overworld", GEN, GENERATOR);
    p.bind("minecraft:overworld", PRESS_AT, PRESS);
    p.bind("minecraft:overworld", DRILL_AT, DRILL);
    p.step = step;
    p
}

#[test]
fn a_machine_in_inventory_or_an_empty_machine_cannot_complete_production() {
    let mut p = Progress { step: 2, ..Progress::default() };
    let press = machine(PRESS, Some((PLATE, 64)), 200);
    p.observe(PRESS_AT, &press, 200, 0);
    assert!(!p.ready()); // No selected installation, and no new cycles.
    p.bind("minecraft:overworld", PRESS_AT, PRESS);
    p.observe(PRESS_AT, &press, 200, 0);
    assert_eq!(p.plates, 0); // Existing stock is not new production.
}

#[test]
fn only_selected_powered_press_and_new_copper_cycles_count() {
    let mut p = line(1);
    let mut press = machine(PRESS, Some((PLATE, 4)), 40);
    p.observe(PRESS_AT, &press, 40, 0);
    assert!(!p.ready());
    set_number(&mut press, "energy", 50);
    p.observe([99,64,0], &press, 40, 0);
    assert!(!p.ready());
    p.observe(PRESS_AT, &press, 40, 0);
    assert!(p.ready());
    p.step = 2;
    p.observe(PRESS_AT, &press, 39, 0);
    assert_eq!(p.plates, 1); // One cycle, even with four items in the output.
    let iron = machine(PRESS, Some(("veio:iron_plate", 20)), 50);
    p.observe(PRESS_AT, &iron, 40, 0);
    assert_eq!(p.plates, 1);
    p.observe(PRESS_AT, &press, 37, 0);
    assert!(p.ready());
}

#[test]
fn manual_press_input_and_old_plates_do_not_prove_automatic_work() {
    let mut p = line(5);
    let press = machine(PRESS, Some((PLATE, 10)), 10);
    p.observe(PRESS_AT, &press, 0, 0); // Player filled the machine; no accepted transport event.
    p.transfer(PRESS_AT, OUT, PLATE, 10, false, 2);
    p.transfer(OUT, CHEST, PLATE, 10, true, 3);
    assert_eq!((p.fed, p.made, p.stored), (0,0,0));
    assert!(!p.ready());
}

#[test]
fn automatic_delivery_from_a_player_stocked_buffer_is_valid_partial_automation() {
    let mut p = line(5);
    p.transfer(BELT_AT, PRESS_AT, "minecraft:raw_copper", 4, false, 1);
    p.observe(PRESS_AT, &machine(PRESS, Some((PLATE,4)),4), 0, 0);
    assert!(p.ready());
    p.step = 6;
    p.transfer(PRESS_AT, CHEST, PLATE, 4, true, 2);
    assert!(p.ready());
}

#[test]
fn real_extraction_transport_processing_and_storage_complete_the_line() {
    let mut p = line(5);
    p.transfer(DRILL_AT, BELT_AT, "minecraft:raw_copper", 4, false, 10);
    p.transfer(BELT_AT, PRESS_AT, "minecraft:raw_copper", 4, false, 11);
    assert!(!p.ready()); // Actual receipt alone is not a completed press cycle.
    let press = machine(PRESS, Some((PLATE, 4)), 4);
    p.observe(PRESS_AT, &press, 0, 0);
    assert!(p.ready());
    p.step = 6;
    p.transfer(PRESS_AT, OUT, PLATE, 4, false, 12);
    assert!(!p.ready()); // A belt alone is not a chest delivery.
    p.transfer(OUT, CHEST, PLATE, 4, true, 13);
    assert!(p.ready());
    assert_eq!(p.stored, 4);
}

#[test]
fn rejected_deliveries_and_replayed_output_cannot_mint_plate_credit() {
    let mut p = line(5);
    p.transfer(BELT_AT, PRESS_AT, "minecraft:raw_copper", 0, false, 1);
    assert_eq!(p.fed, 0);
    p.transfer(BELT_AT, PRESS_AT, "minecraft:raw_copper", 2, false, 2);
    p.observe(PRESS_AT, &machine(PRESS, Some((PLATE,2)),2), 0, 0);
    p.step = 6;
    p.transfer(PRESS_AT, CHEST, PLATE, 2, true, 3);
    p.transfer(PRESS_AT, CHEST, PLATE, 2, true, 4);
    assert_eq!(p.stored, 2);
    assert!(!p.ready());
}

#[test]
fn stale_evidence_expires_and_trace_memory_is_bounded() {
    let mut p = line(5);
    p.output_credit = 4;
    p.transfer(PRESS_AT, OUT, PLATE, 4, false, 1);
    p.transfer(OUT, CHEST, PLATE, 4, true, TRACE_TTL + 2);
    assert_eq!(p.stored, 0);
    for x in 0..1024 { put_trace(&mut p.raw, [x,80,10], 1, 300); }
    assert_eq!(p.raw.len(), MAX_TRACE);
}

#[test]
fn replacing_a_selected_machine_resets_unfinished_line_evidence() {
    let mut p = line(6);
    p.fed = 8; p.made = 8; p.stored = 3; p.input_credit = 4;
    p.bind("minecraft:overworld", [20,64,0], PRESS);
    assert_eq!((p.step, p.fed, p.made, p.stored, p.input_credit), (6,0,0,0,0));
    p.bind("minecraft:the_nether", PRESS_AT, PRESS);
    assert!(p.blocks[DRILL_SLOT].is_none());
}

#[test]
fn replacing_a_machine_at_the_same_position_invalidates_pending_flow() {
    let mut p = line(6);
    p.fed = 4; p.input_credit = 4; p.output_credit = 2; p.stored = 3;
    p.placed("minecraft:overworld", PRESS_AT, PRESS);
    assert_eq!((p.fed, p.input_credit, p.output_credit, p.stored), (0,0,0,0));
    assert_eq!(p.blocks[PRESS_SLOT], Some(PRESS_AT));
}

#[test]
fn a_press_that_spent_its_last_energy_on_a_real_cycle_is_powered() {
    let mut p = line(1);
    let mut press = machine(PRESS, Some((PLATE, 1)), 1);
    set_number(&mut press, "Status", 1);
    p.observe(PRESS_AT, &press, 0, 0);
    assert!(p.ready());
}

#[test]
fn project_requires_the_real_first_delivery_in_the_selected_hub() {
    let mut p = line(7);
    let at = [30,64,0];
    p.bind("minecraft:overworld", at, HUB);
    let mut hub = machine(HUB, None, 0);
    set_number(&mut hub, "ContractStage", 1);
    p.observe(at, &hub, 0, 1);
    assert!(!p.ready()); // An already completed hub does not count.
    p.observe([31,64,0], &hub, 0, 0);
    assert!(!p.ready());
    p.observe(at, &hub, 0, 0);
    assert!(p.ready());
}

#[test]
fn save_roundtrip_preserves_progress_but_never_restores_transient_credit() {
    let mut p = line(6);
    p.hidden = true; p.stored = 3; p.fed = 8; p.made = 4;
    p.input_credit = 8; p.output_credit = 4;
    put_trace(&mut p.raw, BELT_AT, 8, 1);
    let loaded = Progress::decode(&p.encode()).unwrap();
    assert_eq!((loaded.step, loaded.hidden, loaded.stored), (6, true, 3));
    assert_eq!(loaded.blocks, p.blocks);
    assert_eq!((loaded.fed, loaded.made), (8, 4));
    assert_eq!((loaded.input_credit, loaded.output_credit), (0, 0));
    assert!(loaded.raw.is_empty());
    assert!(Progress::decode("bad save").is_none());
    // O que já foi fabricado e as dicas sobrevivem ao save; um save antigo (v1) ainda abre.
    p.crafted = 0b101; p.tips = TIP_BELT;
    let loaded = Progress::decode(&p.encode()).unwrap();
    assert_eq!((loaded.crafted, loaded.tips), (0b101, TIP_BELT));
    let old = p.encode().replace("veio-onboarding-5", "veio-onboarding-1").replacen(" 5 2 0 0 0 0 0\n", "\n", 1);
    assert_eq!(Progress::decode(&old).map(|old| (old.step, old.crafted)), Some((6, 0)));
    assert!(Progress::decode(&p.encode().replace("6 1", "99 1")).is_none());
}

#[test]
fn separate_players_do_not_share_production_or_delivery_evidence() {
    let mut alice = line(5);
    let bob = line(5);
    alice.transfer(DRILL_AT, BELT_AT, "minecraft:raw_copper", 2, false, 1);
    alice.transfer(BELT_AT, PRESS_AT, "minecraft:raw_copper", 2, false, 2);
    assert_eq!((alice.fed, bob.fed), (2,0));
}

#[test]
fn worker_cargo_only_counts_on_accepted_delivery_and_survives_long_walks() {
    let mut p = line(6);
    let post = [10,64,10];
    p.output_credit = 4;
    p.worker_pickup(post, PRESS_AT, PLATE, 4, 1);
    assert_eq!(p.stored, 0); // In transit, not in stock.
    p.worker_delivery(post, post, PLATE, 0, true, 400);
    assert_eq!(p.stored, 0); // Full destination, all cargo retained.
    p.worker_delivery(post, CHEST, PLATE, 2, true, 500);
    assert_eq!(p.stored, 2); // 25-second walk, no conveyor deadline.
    p.worker_delivery(post, post, PLATE, 2, true, 600);
    assert_eq!(p.stored, 4); // Leftovers returned to the post.
    p.worker_delivery(post, post, PLATE, 4, true, 601);
    assert_eq!(p.stored, 4); // Retrying/replaying unload cannot duplicate evidence.
}

#[test]
fn arbitrary_worker_loads_do_not_certify_plates_and_failed_supply_gives_no_credit() {
    let mut p = line(5);
    let post = [10,64,10];
    p.worker_delivery(post, PRESS_AT, "minecraft:raw_copper", 0, false, 1);
    assert_eq!(p.fed, 0);
    p.worker_pickup(post, [20,64,20], PLATE, 8, 2); // Old/player-stocked plates.
    p.worker_delivery(post, post, PLATE, 8, true, 3);
    assert_eq!(p.stored, 0);
    p.worker_delivery(post, PRESS_AT, "minecraft:raw_copper", 2, false, 4);
    p.observe(PRESS_AT, &machine(PRESS, Some((PLATE,2)),2), 0, 0);
    assert!(p.ready());
}

#[test]
fn client_window_tick_never_reads_the_server_world_or_advances_its_clock() {
    struct Client;
    impl Host for Client {
        fn window(&self) -> bool { true }
        fn player(&self) -> Option<horizon_client::mod_api::NativePlayer> { None }
    }
    // All other Host methods panic: a client tick must not call any of them.
    let before = SERVER_CLOCK.load(Ordering::Relaxed);
    server_tick(&mut Client);
    assert_eq!(SERVER_CLOCK.load(Ordering::Relaxed), before);
}

#[test]
fn saving_twice_atomically_replaces_the_file_and_failures_keep_it_dirty() {
    let unique = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
    let root = std::env::temp_dir().join(format!("veio-tutorial-{}-{unique}", std::process::id()));
    let mut p = line(2);
    p.plates = 2;
    save(&root, 10, &mut p).unwrap();
    assert!(!p.dirty);
    assert_eq!(load(&root, 10).plates, 2);
    p.plates = 3; p.dirty = true;
    save(&root, 10, &mut p).unwrap();
    assert_eq!(load(&root, 10).plates, 3);
    assert!(!path(&root, 10).with_extension("tmp").exists());
    // A file is deliberately used where a directory would have to exist.
    p.dirty = true;
    assert!(save(&path(&root, 10), 11, &mut p).is_err());
    assert!(p.dirty);
    std::fs::remove_file(path(&root, 10)).unwrap();
    std::fs::remove_dir(root).unwrap();
}

/// Exercises the process-wide hook session alone so unrelated parallel logistics
/// tests do not observe the intentionally active tutorial registry.
#[test]
#[ignore = "run the lifecycle harness alone with --ignored --test-threads=1"]
fn server_lifecycle_displays_one_goal_saves_rejoins_and_isolates_worlds() {
    struct Server {
        root: PathBuf,
        player: NativeServerPlayer,
        bars: Vec<String>,
        messages: Vec<String>,
        /// Itens no inventário do jogador (para `execute if items`); `None`: tem de tudo.
        inventory: Option<Vec<&'static str>>,
    }
    impl Host for Server {
        fn world_dir(&self) -> Option<PathBuf> { Some(self.root.clone()) }
        fn server_players(&self) -> Vec<NativeServerPlayer> { vec![self.player.clone()] }
        fn acting_player(&self) -> Option<NativeServerPlayer> { Some(self.player.clone()) }
        fn action_bar(&mut self, uuid: u128, text: Text) -> bool {
            assert_eq!(uuid, self.player.uuid); self.bars.push(text.plain()); true
        }
        fn tell(&mut self, uuid: u128, text: Text) -> bool {
            assert_eq!(uuid, self.player.uuid); self.messages.push(text.plain()); true
        }
        fn run_command(&mut self, line: &str) -> bool {
            self.messages.push(line.to_owned());
            match (&self.inventory, line.strip_prefix("execute if items entity Tutorial container.* ")) {
                (Some(items), Some(item)) => items.contains(&item),
                _ => true,
            }
        }
    }
    let unique = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
    let root = std::env::temp_dir().join(format!("veio-tutorial-lifecycle-{}-{unique}", std::process::id()));
    let other_world = root.with_extension("other");
    let player = NativeServerPlayer { uuid: 10, name: "Tutorial".into(), dimension: "minecraft:overworld".to_owned(), ..Default::default() };
    let mut host = Server { root: root.clone(), player: player.clone(), bars: vec![], messages: vec![],
        inventory: Some(vec!["minecraft:iron_ingot"]) };
    // O prólogo (casa, prefeito, terreno) já foi feito neste mundo; ele tem teste próprio.
    save(&root.join("veio/tutorial"), player.uuid, &mut Progress { intro: intro::INTRO_DONE,
        dimension: "minecraft:overworld".into(), ..Progress::default() }).unwrap();
    tick(&mut host, 20);
    // Sem o gerador: a lista de materiais ocupa a barra (ferro já tem, o resto falta).
    assert!(host.bars.is_empty());
    let list = host.messages.iter().find(|line| line.starts_with("title Tutorial actionbar")).unwrap().clone();
    assert!(list.contains(r#"✔ ","color":"green"},{"translate":"veio.onboarding.name.iron_ingot""#), "{list}");
    assert!(list.contains(r#"✘ ","color":"red"},{"translate":"veio.onboarding.name.furnace""#), "{list}");
    // Com o gerador fabricado, volta o objetivo.
    host.inventory = None;
    tick(&mut host, 40);
    assert!(host.bars.last().unwrap().contains("goal.0"));
    // A etapa chega em passos curtos.
    assert!(host.messages.iter().any(|line| line.contains("title.0")));
    assert!(host.messages.iter().any(|line| line.contains("do.0.3")));
    // A ficha da receita, o livro de receitas e a lista de materiais (nada é dado de graça).
    assert!(host.messages.iter().any(|line| line.contains("recipe.0.0")));
    assert!(host.messages.iter().any(|line| line == "recipe give Tutorial veio:generator"), "{:?}", host.messages);
    assert!(host.messages.iter().any(|line| line.starts_with("title Tutorial actionbar") && line.contains("name.furnace")));
    assert!(!host.messages.iter().any(|line| line.starts_with("give ")));
    select(&host, GEN, GENERATOR, false);
    tick(&mut host, 40);
    assert!(host.bars.last().unwrap().contains("goal.1"));
    // Etapa concluída: título, som de conquista e experiência.
    assert!(host.messages.iter().any(|line| line == "playsound minecraft:ui.toast.challenge_complete master Tutorial"));
    assert!(host.messages.iter().any(|line| line.starts_with("xp add Tutorial ")));
    select(&host, PRESS_AT, PRESS, false);
    let mut press = machine(PRESS, None, 0);
    set_number(&mut press, "energy", 20);
    observe("minecraft:overworld", PRESS_AT, &press, 0, 0, 60);
    tick(&mut host, 60);
    assert!(host.bars.last().unwrap().contains("goal.2"));
    let press = machine(PRESS, Some((PLATE, 4)), 4);
    observe("minecraft:overworld", PRESS_AT, &press, 0, 0, 80);
    tick(&mut host, 80);
    assert!(host.bars.last().unwrap().contains("goal.3"));
    logout(&host, player.uuid);
    tick(&mut host, 100);
    assert!(host.bars.last().unwrap().contains("goal.3"));
    assert!(command(&mut host, &player, "veiotutorial ocultar"));
    let before = host.bars.len();
    tick(&mut host, 120);
    assert_eq!(host.bars.len(), before);
    assert!(command(&mut host, &player, "veiotutorial mostrar"));
    tick(&mut host, 140);
    assert!(host.bars.last().unwrap().contains("goal.3"));
    host.root = other_world.clone();
    logged_in(&host);
    tick(&mut host, 160);
    assert!(host.bars.last().unwrap().contains("intro.goal.village"), "mundo novo começa pela vila");
    host.root = root.clone();
    logged_in(&host);
    tick(&mut host, 180);
    assert!(host.bars.last().unwrap().contains("goal.3"));
    logout(&host, player.uuid);
    *SESSION.lock().unwrap() = None;
    ACTIVE.store(false, Ordering::Relaxed); FLOW_ACTIVE.store(false, Ordering::Relaxed);
    let save_root = root.join("veio/tutorial");
    std::fs::remove_file(path(&save_root, player.uuid)).unwrap();
    std::fs::remove_dir(&save_root).unwrap();
    std::fs::remove_dir(root.join("veio")).unwrap();
    std::fs::remove_dir(root).unwrap();
    // No outro mundo o gerador ainda não existe; o progresso de lá fica separado deste.
    // (Lá o jogador ainda está no prólogo, sem nada para salvar: o arquivo pode nem existir.)
    let other = std::fs::read_to_string(path(&other_world.join("veio/tutorial"), player.uuid)).ok()
        .and_then(|text| Progress::decode(&text)).unwrap_or_default();
    assert_eq!((other.step, other.intro), (0, intro::VILLAGE_STAGE));
    let _ = std::fs::remove_dir_all(&other_world);
}

#[test]
fn productivity_certifies_bonus_items_without_spending_two_input_credits() {
    let mut p = line(5);
    p.transfer(BELT_AT, PRESS_AT, "minecraft:raw_copper", 1, false, 1);
    let mut press = machine(PRESS, Some((PLATE, 2)), 2);
    set_number(&mut press, "Total", 1);
    p.observe_counts(PRESS_AT, &press, 0, 0, Some(0));
    assert_eq!((p.input_credit, p.output_credit, p.made), (0, 2, 2));
    p.transfer(BELT_AT, PRESS_AT, "minecraft:raw_copper", 1, false, 2);
    press.items[1] = Some(ItemStack::new(PLATE, 3));
    set_number(&mut press, "Produced", 3); set_number(&mut press, "Total", 2);
    p.observe_counts(PRESS_AT, &press, 2, 0, Some(1));
    assert_eq!((p.input_credit, p.output_credit, p.made), (0, 3, 3));
    assert!(p.ready());
}

// ---------- Prólogo: casa, prefeito e terreno ----------

#[test]
fn headings_point_the_right_way() {
    use intro::heading;
    assert_eq!(heading([0.0, 64.0, 0.0], [0.0, 64.0, -20.0]), (20, "n"));
    assert_eq!(heading([0.0, 64.0, 0.0], [10.0, 64.0, 10.0]).1, "se");
    assert_eq!(heading([0.0, 64.0, 0.0], [-30.0, 64.0, 0.0]), (30, "w"));
    assert_eq!(heading([5.0, 64.0, 5.0], [5.0, 64.0, 25.0]).1, "s");
}

#[test]
fn level_counts_what_to_dig_and_what_to_fill() {
    use intro::{level, Level};
    let flat = vec![Some(64); 4];
    assert_eq!(level(64, &flat, &flat, &flat), Some(Level::default()));
    // Um morro, um buraco, água (fundo abaixo do chão) e uma copa de árvore por cima.
    let ground = vec![Some(66), Some(63), Some(64), Some(64)];
    let floor = vec![Some(66), Some(63), Some(62), Some(64)];
    let top = vec![Some(66), Some(63), Some(64), Some(70)];
    assert_eq!(level(64, &ground, &floor, &top), Some(Level { left: 4, dig: 2, fill: 2 }));
    // Coluna em chunk não carregado: não dá para medir agora.
    assert_eq!(level(64, &[None], &[Some(64)], &[Some(64)]), None);
}

#[test]
fn the_plot_avoids_water_and_prefers_flat_ground() {
    use intro::plot_score;
    assert_eq!(plot_score(&[64, 64], &[64, 64], &[64, 64]), Some(0));
    assert!(plot_score(&[64, 64], &[61, 64], &[64, 64]).is_none(), "lago recusa");
    assert!(plot_score(&[64, 67], &[64, 67], &[64, 67]) > plot_score(&[64, 65], &[64, 65], &[64, 70]));
}

#[test]
fn old_saves_skip_the_prologue_once_the_factory_started_and_v3_keeps_the_plot() {
    let old = "veio-onboarding-2\n3 0 1 4 0 0 0 0 0 0 0 0\nminecraft:overworld\n-\n-\n-\n-\n-\n";
    let p = Progress::decode(old).unwrap();
    assert_eq!((p.step, p.intro, p.outro), (3, intro::INTRO_DONE, false));
    let fresh = "veio-onboarding-2\n0 0 0 0 0 0 0 0 0 0 0 0\nminecraft:overworld\n-\n-\n-\n-\n-\n";
    assert_eq!(Progress::decode(fresh).unwrap().intro, intro::VILLAGE_STAGE);
    let finished = "veio-onboarding-1\n8 0 1 4 1 1 2 1 4 1\nminecraft:overworld\n-\n-\n-\n-\n-\n";
    assert!(Progress::decode(finished).unwrap().outro, "quem já terminou não precisa voltar ao prefeito");
    let mut p = Progress { intro: intro::PLOT_STAGE, plot: Some([30, 65, -4]), dimension: "minecraft:overworld".into(), ..Progress::default() };
    p = Progress::decode(&p.encode()).unwrap();
    assert_eq!((p.intro, p.plot), (intro::PLOT_STAGE, Some([30, 65, -4])));
    // Sem o terreno, o prefeito escolhe de novo.
    let lost = p.encode().replace("30 65 -4\n", "-\n");
    assert_eq!(Progress::decode(&lost).unwrap().intro, intro::MAYOR_STAGE);
}

#[test]
fn the_mayor_asks_for_resources_gives_the_generator_and_a_level_plot_starts_the_factory() {
    use horizon_client::mod_api::NativeHeightmap;
    struct Land { player: NativeServerPlayer, hill: Option<[i32; 2]>, chat: Vec<String>, sparks: usize,
        bag: HashMap<String, u32> }
    impl Host for Land {
        fn surface_heights(&self, _: &str, columns: &[(i32, i32)], _: NativeHeightmap) -> Result<Vec<Option<i32>>, String> {
            Ok(columns.iter().map(|(x, z)| Some(if self.hill == Some([*x, *z]) { 70 } else { 64 })).collect())
        }
        fn acting_player(&self) -> Option<NativeServerPlayer> { Some(self.player.clone()) }
        fn run_command(&mut self, line: &str) -> bool { self.chat.push(line.to_owned()); true }
        fn tell(&mut self, _: u128, text: Text) -> bool { self.chat.push(text.plain()); true }
        fn action_bar(&mut self, _: u128, _: Text) -> bool { true }
        fn entity(&self, _: u32) -> Option<horizon_client::mod_api::EntityRef> { None }
        fn take_items(&mut self, item: &str, count: u32) -> u32 {
            let have = self.bag.entry(item.to_owned()).or_default();
            let n = (*have).min(count); *have -= n; n
        }
        fn give_items(&mut self, item: &str, count: u32) -> u32 { *self.bag.entry(item.to_owned()).or_default() += count; count }
        fn send_particles(&mut self, _: &horizon_client::mod_api::ParticleOptions, _: Option<horizon_client::mod_api::BlockStateId>,
            _: [f64; 3], _: [f64; 3], _: f64, _: i32) -> bool { self.sparks += 1; true }
    }
    let player = NativeServerPlayer { uuid: 77, name: "Fundador".into(), dimension: "minecraft:overworld".into(),
        position: [0.0, 64.0, 0.0], ..Default::default() };
    let mut land = Land { player: player.clone(), hill: None, chat: vec![], sparks: 0, bag: HashMap::new() };
    let mut p = Progress { intro: intro::MAYOR_STAGE, ..Progress::default() };
    // Primeira conversa: a história e o pedido.
    intro::talk(&mut land, &player, &mut p);
    assert_eq!(p.intro, intro::GATHER_STAGE);
    assert!(land.chat.iter().any(|line| line.contains("mayor.how.1")), "{:?}", land.chat);
    // Faltando um lingote: nada some do inventário.
    land.bag.insert("minecraft:cobblestone".into(), 20);
    land.bag.insert("minecraft:iron_ingot".into(), 2);
    land.bag.insert("minecraft:copper_ingot".into(), 1);
    intro::talk(&mut land, &player, &mut p);
    assert_eq!(p.intro, intro::GATHER_STAGE);
    assert_eq!((land.bag["minecraft:cobblestone"], land.bag["minecraft:copper_ingot"]), (20, 1), "devolve o que tirou");
    // Com tudo: entrega, ganha o gerador e as receitas, e o terreno é escolhido.
    land.bag.insert("minecraft:copper_ingot".into(), 3);
    intro::talk(&mut land, &player, &mut p);
    assert_eq!(p.intro, intro::PLOT_STAGE);
    assert_eq!(land.bag[GENERATOR], 1);
    assert_eq!((land.bag["minecraft:cobblestone"], land.bag["minecraft:copper_ingot"]), (4, 1));
    assert!(land.chat.iter().any(|line| line == "recipe give Fundador veio:press"));
    let plot = p.plot.expect("o prefeito escolheu o terreno");
    assert_eq!(plot[1], 64, "nível do meio do terreno");
    // Um bloco a mais no meio do terreno: ainda não conta, mas cabe na folga.
    land.hill = Some([plot[0], plot[2]]);
    land.player.position = [f64::from(plot[0]), 64.0, f64::from(plot[2])];
    let near = land.player.clone();
    intro::tick(&mut land, &near, &mut p, 40);
    assert_eq!(p.intro, intro::INTRO_DONE, "sobra 1 coluna, dentro da folga de {}", intro::SLACK);
    assert!(land.sparks > 0, "as marcas aparecem para quem está perto");
    assert!(land.chat.iter().any(|line| line.contains("plot_done")));
    // Depois: perguntar ao prefeito repete a etapa atual da fábrica.
    land.chat.clear();
    intro::talk(&mut land, &near, &mut p);
    assert!(land.chat.iter().any(|line| line.contains("why.0")), "{:?}", land.chat);
    // Fim da fábrica: a última conversa fecha o tutorial uma vez e libera o Quadro da fábrica.
    p.step = DONE;
    intro::talk(&mut land, &near, &mut p);
    assert!(p.outro);
    assert!(land.chat.iter().any(|line| line.contains("outro.title")));
    assert!(land.chat.iter().any(|line| line == "recipe give Fundador veio:factory_board"));
    let calls = land.chat.len();
    intro::talk(&mut land, &near, &mut p);
    assert!(!land.chat[calls..].iter().any(|line| line.contains("xp add")), "a recompensa sai uma vez só");
}

/// Mundo de mentira para o capítulo da vila: blocos do Veio por posição.
#[derive(Default)]
struct Town { entities: HashMap<Pos, ModBlockEntity>, said: Vec<String>, commands: Vec<String>, bars: Vec<String> }

impl Host for Town {
    fn change_mod_block_entity(&mut self, pos: [i32; 3], change: &mut dyn FnMut(&mut ModBlockEntity)) -> bool {
        match self.entities.get_mut(&pos) { Some(entity) => { change(entity); true } None => false }
    }
    fn tell(&mut self, _: u128, text: Text) -> bool { self.said.push(text.plain()); true }
    fn action_bar(&mut self, _: u128, text: Text) -> bool { self.bars.push(text.plain()); true }
    fn run_command(&mut self, line: &str) -> bool {
        self.commands.push(line.to_owned());
        // Inventário vazio: a lista de materiais mostra tudo em vermelho.
        !line.starts_with("execute if items")
    }
    fn send_particles(&mut self, _: &horizon_client::mod_api::ParticleOptions, _: Option<horizon_client::mod_api::BlockStateId>, _: [f64; 3], _: [f64; 3], _: f64, _: i32) -> bool { true }
}

fn town_block(id: &str) -> ModBlockEntity {
    let mut entity = ModBlockEntity::default();
    entity.id = id.into();
    entity
}

fn hire(entity: &mut ModBlockEntity) { entity.nbt.insert("Worker".into(), Tag::IntArray(vec![1, 2, 3, 4])); }

#[test]
fn the_village_chapter_checks_each_step_in_the_world() {
    const POST: Pos = [0, 64, 0];
    const MARKER: Pos = [5, 64, 3];
    let mut town = Town::default();
    let mut p = Progress { intro: intro::INTRO_DONE, step: DONE, outro: true, village: 1, ..Progress::default() };
    // 1: o posto do lavrador colocado (só no Overworld e com o capítulo aberto).
    assert!(!village::ready(&mut town, &mut p));
    village::placed(&mut p, "minecraft:the_nether", POST, workforce::FARM_POST);
    assert_eq!(p.village_blocks[0], None);
    village::placed(&mut p, workforce::spawn::OVERWORLD, POST, workforce::FARM_POST);
    town.entities.insert(POST, town_block(workforce::FARM_POST));
    assert!(village::ready(&mut town, &mut p));
    // 2: contratar.
    p.village = 2;
    assert!(!village::ready(&mut town, &mut p));
    hire(town.entities.get_mut(&POST).unwrap());
    assert!(village::ready(&mut town, &mut p));
    // 3: o marco tem de ficar perto do posto.
    p.village = 3;
    village::placed(&mut p, workforce::spawn::OVERWORLD, [20, 64, 0], workforce::zones::FIELD_MARKER);
    town.entities.insert([20, 64, 0], town_block(workforce::zones::FIELD_MARKER));
    assert!(!village::ready(&mut town, &mut p), "marco longe do posto");
    village::placed(&mut p, workforce::spawn::OVERWORLD, MARKER, workforce::zones::FIELD_MARKER);
    town.entities.insert(MARKER, town_block(workforce::zones::FIELD_MARKER));
    assert!(village::ready(&mut town, &mut p));
    // 4: colheitas novas, contadas a partir do que o posto já tinha.
    p.village = 4;
    p.village_base = 10;
    set_number(town.entities.get_mut(&POST).unwrap(), "Produced", 12);
    assert!(!village::ready(&mut town, &mut p));
    assert_eq!(p.village_count, 2);
    set_number(town.entities.get_mut(&POST).unwrap(), "Produced", 14);
    assert!(village::ready(&mut town, &mut p));
    // Quebrar o posto: o capítulo esquece o bloco.
    town.entities.remove(&POST);
    assert!(!village::ready(&mut town, &mut p));
    assert_eq!(p.village_blocks[0], None);
    // 7: a casa precisa de morador.
    p.village = 7;
    village::placed(&mut p, workforce::spawn::OVERWORLD, [30, 64, 30], workforce::society::HOUSE);
    town.entities.insert([30, 64, 30], town_block(workforce::society::HOUSE));
    assert!(!village::ready(&mut town, &mut p));
    set_number(town.entities.get_mut(&[30, 64, 30]).unwrap(), "Rate", 2);
    assert!(village::ready(&mut town, &mut p));
    // 8: a prefeitura com funcionário.
    p.village = 8;
    village::placed(&mut p, workforce::spawn::OVERWORLD, [40, 64, 40], workforce::society::TOWN_HALL);
    let mut hall = town_block(workforce::society::TOWN_HALL);
    hire(&mut hall);
    town.entities.insert([40, 64, 40], hall);
    assert!(village::ready(&mut town, &mut p));
    // O progresso do capítulo sobrevive ao save.
    p.village_crafted = 0b101;
    let loaded = Progress::decode(&p.encode()).unwrap();
    assert_eq!((loaded.village, loaded.village_crafted, loaded.village_base), (8, 0b101, 10));
    assert_eq!(loaded.village_blocks, p.village_blocks);
    // Fora do capítulo, colocar blocos não é registrado.
    let mut idle = Progress::default();
    village::placed(&mut idle, workforce::spawn::OVERWORLD, POST, workforce::FARM_POST);
    assert_eq!(idle.village_blocks[0], None);
}

#[test]
fn the_mayor_opens_the_village_chapter_and_celebrates_each_step() {
    let mut town = Town::default();
    let player = NativeServerPlayer { uuid: 7, name: "Ana".into(), dimension: workforce::spawn::OVERWORLD.into(),
        position: [0.0, 64.0, 0.0], ..Default::default() };
    let mut p = Progress { intro: intro::INTRO_DONE, step: DONE, outro: true, ..Progress::default() };
    // Depois da fábrica, falar com o prefeito abre o capítulo.
    intro::talk(&mut town, &player, &mut p);
    assert_eq!(p.village, 1);
    // As falas do prefeito vão pelo formato "Nome: fala" (o substituto mostra só a chave dele).
    let says = |town: &Town| town.said.iter().filter(|line| *line == "veio.agenda.says").count();
    assert_eq!(says(&town), 3, "três falas de abertura");
    assert!(town.commands.iter().any(|line| line.contains("veio.village.start_title")));
    // Primeiro segundo: a etapa chega com a fala, a ficha, a receita no livro e os passos.
    village::tick(&mut town, &player, &mut p, 40);
    assert!(town.said.iter().any(|line| line.contains("veio.village.title.0")));
    assert_eq!(says(&town), 4, "o prefeito conta o porquê");
    assert!(town.said.iter().any(|line| line.contains("veio.village.recipe.0")));
    assert!(town.said.iter().any(|line| line.contains("veio.village.do.0.4")));
    assert!(town.commands.iter().any(|line| line == "recipe give Ana veio:farm_post"));
    let list = town.commands.iter().find(|line| line.starts_with("title Ana actionbar")).expect("lista de materiais");
    assert!(list.contains("veio.onboarding.name.composter") && list.contains("✘"));
    // O posto colocado fecha a etapa: comentário do prefeito, título e experiência.
    village::placed(&mut p, workforce::spawn::OVERWORLD, [0, 64, 0], workforce::FARM_POST);
    town.entities.insert([0, 64, 0], town_block(workforce::FARM_POST));
    town.commands.clear();
    village::tick(&mut town, &player, &mut p, 60);
    assert_eq!(p.village, 2);
    assert!(says(&town) >= 6, "comenta a etapa e conta a próxima");
    assert!(town.commands.iter().any(|line| line.starts_with("xp add Ana")));
    assert!(town.said.iter().any(|line| line.contains("veio.village.title.1")), "anuncia a etapa seguinte");
    assert!(town.bars.last().unwrap().contains("veio.village.bar"));
    // Clicar no prefeito no meio do capítulo repete a etapa.
    town.said.clear();
    intro::talk(&mut town, &player, &mut p);
    assert!(town.said.iter().any(|line| line.contains("veio.village.title.1")));
    assert!(town.said.iter().any(|line| line.contains("veio.village.do.1.3")));
    // Última etapa concluída: o prefeito fecha o capítulo.
    p.village = 8;
    let mut hall = town_block(workforce::society::TOWN_HALL);
    hire(&mut hall);
    town.entities.insert([9, 64, 9], hall);
    village::placed(&mut p, workforce::spawn::OVERWORLD, [9, 64, 9], workforce::society::TOWN_HALL);
    village::tick(&mut town, &player, &mut p, 80);
    assert_eq!(p.village, village::DONE_VILLAGE);
    assert!(town.commands.iter().any(|line| line.contains("veio.village.end_title")));
    assert!(village::step(&p).is_none());
}

#[test]
fn the_arrow_turns_with_the_player() {
    use intro::arrow;
    let (from, south) = ([0.0, 64.0, 0.0], [0.0, 64.0, 30.0]);
    assert_eq!(arrow(from, south, 0.0), "↑", "olhando para o sul, o alvo ao sul fica à frente");
    assert_eq!(arrow(from, south, 180.0), "↓");
    // Olhando para o oeste (yaw 90), o sul fica à esquerda.
    assert_eq!(arrow(from, south, 90.0), "←");
    assert_eq!(arrow(from, south, -90.0), "→");
    assert_eq!(arrow(from, [30.0, 64.0, 30.0], 0.0), "↖", "sudeste, olhando para o sul: à frente e à esquerda");
}
