use super::*;

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
    impl Host for Client { fn window(&self) -> bool { true } }
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
    }
    let unique = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
    let root = std::env::temp_dir().join(format!("veio-tutorial-lifecycle-{}-{unique}", std::process::id()));
    let other_world = root.with_extension("other");
    let player = NativeServerPlayer { uuid: 10, dimension: "minecraft:overworld".to_owned(), ..Default::default() };
    let mut host = Server { root: root.clone(), player: player.clone(), bars: vec![], messages: vec![] };
    tick(&mut host, 20);
    assert!(host.bars.last().unwrap().contains("goal.0"));
    select(&host, GEN, GENERATOR, false);
    tick(&mut host, 40);
    assert!(host.bars.last().unwrap().contains("goal.1"));
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
    assert!(host.bars.last().unwrap().contains("goal.0"));
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
    assert!(!other_world.exists());
}
