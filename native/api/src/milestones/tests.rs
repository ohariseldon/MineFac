//! Quadro da fábrica: entregas, recompensas, pedido do dia, censo e alertas.
use super::*;
use std::cell::{Cell, RefCell};
use horizon_client::mod_api::NativeServerPlayer;

#[test]
fn opening_another_world_clears_daily_claims_and_machine_census() {
    let mut machine = ModBlockEntity::default();
    machine.id = PRESS.into();
    set_number(&mut machine, "Status", 13);
    census::observe("veio:previous-world", [0, 64, 0], &machine, 0, 0, 10);
    assert!(census::claim_order("veio:previous-world", 2));
    assert!(!census::claim_order("veio:previous-world", 2));
    assert_eq!(census::summary("veio:previous-world", [0, 64, 0], 32, 10).total, 1);
    census::reset();
    assert_eq!(census::summary("veio:previous-world", [0, 64, 0], 32, 10).total, 0);
    assert!(census::claim_order("veio:previous-world", 2));
}

struct Field {
    day: Cell<i64>,
    players: Vec<NativeServerPlayer>,
    told: RefCell<Vec<(u128, String)>>,
    bar: RefCell<Vec<(u128, String)>>,
}

impl Field {
    fn new() -> Self {
        Self { day: Cell::new(1000), players: vec![NativeServerPlayer {
            uuid: 7, name: "Ana".into(), permission: 0, dimension: "veio:board-test".into(),
            position: [3.0, 64.0, 3.0], game_mode: 0, health: 20.0,
        }], told: RefCell::default(), bar: RefCell::default() }
    }
}

impl Host for Field {
    fn dimension(&self) -> &'static str { "veio:board-test" }
    fn day_time(&self) -> i64 { self.day.get() }
    fn server_players(&self) -> Vec<NativeServerPlayer> { self.players.clone() }
    fn tell(&mut self, uuid: u128, line: Text) -> bool { self.told.borrow_mut().push((uuid, line.plain())); true }
    fn action_bar(&mut self, uuid: u128, line: Text) -> bool { self.bar.borrow_mut().push((uuid, line.plain())); true }
    fn play_sound(&mut self, _: &str, _: [f64; 3], _: usize, _: f32, _: f32) -> bool { true }
}

fn board() -> ModBlockEntity {
    let mut entity = ModBlockEntity::default();
    entity.id = BOARD.to_owned();
    entity.items = vec![None; 2];
    entity
}

#[test]
fn every_phase_and_order_is_reachable_and_fits_one_slot() {
    for phase in &PHASES {
        assert!(phase.count <= 64 && phase.reward.1 <= 64, "{}", phase.key);
        assert!(valid_input(0, &ItemStack::new(phase.item, 1)));
    }
    for order in ORDERS { assert!(order.3 <= 64 && order.4 <= 64 && order.0 <= PHASES.len()); }
    for phase in 0..=PHASES.len() {
        for day in 0..30 {
            let order = ORDERS[order_for(day, phase)];
            assert!(order.0 <= phase && order.0 + 1 >= phase, "fase {phase}, dia {day}: pedido da fase {}", order.0);
        }
    }
    assert!(!valid_input(1, &ItemStack::new(PLATE, 1)), "a saída não recebe entregas");
}

#[test]
fn delivering_the_phase_stack_pays_the_reward_and_advances() {
    let mut host = Field::new();
    let mut entity = board();
    entity.items[0] = Some(ItemStack::new(PLATE, 10));
    tick(&mut host, [0, 64, 0], &mut entity);
    assert_eq!((number(&entity, "Delivered"), number(&entity, "Status")), (10, 1));
    assert!(entity.items[0].is_none());
    entity.items[0] = Some(ItemStack::new(PLATE, 10));
    tick(&mut host, [0, 64, 0], &mut entity);
    assert_eq!(number(&entity, "Phase"), 1);
    assert_eq!(entity.items[1], Some(ItemStack::new(BELT, 32)));
    assert_eq!(number(&entity, "Delivered"), 0);
    assert_eq!(host.told.borrow().len(), 2, "conclusão e próximo gargalo, para quem está perto");
    // As 4 chapas restantes não servem à fase 1 (engrenagens); só ao pedido do dia, se ele pedir chapas.
    assert!(accepts(&entity, 0, &ItemStack::new(GEAR, 1)));
    assert!(!accepts(&entity, 1, &ItemStack::new(GEAR, 1)));
}

#[test]
fn a_full_reward_slot_holds_the_phase_without_losing_deliveries() {
    let mut host = Field::new();
    let mut entity = board();
    set_number(&mut entity, "Phase", 1);
    entity.items[1] = Some(ItemStack::new("minecraft:dirt", 64));
    entity.items[0] = Some(ItemStack::new(GEAR, 40));
    tick(&mut host, [0, 64, 0], &mut entity);
    assert_eq!((number(&entity, "Phase"), number(&entity, "Delivered"), number(&entity, "Status")), (1, 32, 3));
    assert_eq!(entity.items[0], Some(ItemStack::new(GEAR, 8)), "o excedente fica na entrada");
    entity.items[1] = None;
    tick(&mut host, [0, 64, 0], &mut entity);
    assert_eq!(number(&entity, "Phase"), 2);
    assert_eq!(entity.items[1], Some(ItemStack::new("veio:permit_workshop", 1)));
}

#[test]
fn the_daily_order_pays_once_and_resets_at_dawn() {
    let mut host = Field::new();
    let mut entity = board();
    set_number(&mut entity, "Phase", PHASES.len() as i32);
    tick(&mut host, [0, 64, 0], &mut entity);
    let order = ORDERS[number(&entity, "Order") as usize];
    entity.items[0] = Some(ItemStack::new(order.1, order.3));
    tick(&mut host, [0, 64, 0], &mut entity);
    tick(&mut host, [0, 64, 0], &mut entity);
    assert_eq!(entity.items[1], Some(ItemStack::new(EMERALD, order.4)), "paga uma vez");
    assert_eq!(number(&entity, "OrderPaid"), 1);
    entity.items[0] = Some(ItemStack::new(order.1, 1));
    tick(&mut host, [0, 64, 0], &mut entity);
    assert_eq!(number(&entity, "Status"), 3, "pedido cheio: o item fica para retirar");
    host.day.set(DAY + 10);
    entity.items[0] = None;
    tick(&mut host, [0, 64, 0], &mut entity);
    assert_eq!((number(&entity, "OrderDone"), number(&entity, "OrderPaid"), number(&entity, "Day")), (0, 0, 2));
}

#[test]
fn a_second_board_does_not_collect_the_same_daily_order() {
    let mut host = Field::new();
    let (mut first, mut second) = (board(), board());
    for entity in [&mut first, &mut second] {
        set_number(entity, "Phase", PHASES.len() as i32);
        tick(&mut host, [0, 64, 0], entity);
        let order = ORDERS[number(entity, "Order") as usize];
        entity.items[0] = Some(ItemStack::new(order.1, order.3));
        tick(&mut host, [0, 64, 0], entity);
    }
    assert_eq!(number(&first, "OrderPaid"), 1);
    assert_eq!((number(&second, "OrderPaid"), second.items[1].clone()), (2, None));
}

#[test]
fn the_census_reads_bottlenecks_and_the_first_hour_checklist() {
    let mut drill = ModBlockEntity::default();
    drill.id = DRILL.into();
    set_number(&mut drill, "Status", 5);
    census::observe("veio:board-test", [5, 64, 0], &drill, 0, 0, 1);
    let mut press = ModBlockEntity::default();
    press.id = PRESS.into();
    set_number(&mut press, "Status", 1);
    census::observe("veio:board-test", [6, 64, 0], &press, 0, 1, 1);
    let early = census::summary("veio:board-test", [0, 64, 0], RADIUS, 50);
    assert_eq!((early.total, early.working, early.power, early.setup), (2, 1, 0, 2 | 4), "sem energia há pouco: ainda não é alerta");
    let late = census::summary("veio:board-test", [0, 64, 0], RADIUS, 250);
    assert_eq!(late.power, 1);
    assert_eq!(census::summary("veio:board-test", [500, 64, 0], RADIUS, 250).total, 0, "fora do raio");
    assert_eq!(census::summary("minecraft:the_nether", [0, 64, 0], RADIUS, 250).total, 0, "outra dimensão");
}

#[test]
fn alerts_say_it_once_then_remind_after_a_minute() {
    let mut host = Field::new();
    let mut drill = ModBlockEntity::default();
    drill.id = DRILL.into();
    set_number(&mut drill, "Status", 5);
    census::observe("veio:board-test", [5, 64, 0], &drill, 0, 0, 0);
    // Tick a tick, como no jogo: a máquina parada se registra a cada 100 ticks na sua fase.
    for now in 1..1400 {
        census::observe("veio:board-test", [5, 64, 0], &drill, 5, 0, now);
        census::tick(&mut host, now);
    }
    assert_eq!(host.bar.borrow().len(), 1, "o mesmo gargalo não repete antes de um minuto");
    for now in 1400..=1600 {
        census::observe("veio:board-test", [5, 64, 0], &drill, 5, 0, now);
        census::tick(&mut host, now);
    }
    assert_eq!(host.bar.borrow().len(), 2);
}

#[test]
fn dawn_announces_the_order_and_dusk_sums_the_day() {
    let mut host = Field::new();
    let mut entity = board();
    tick(&mut host, [0, 64, 0], &mut entity);
    census::tick(&mut host, 20);
    assert!(host.told.borrow().is_empty(), "carregar o mundo não anuncia");
    host.day.set(DAY + 100);
    census::tick(&mut host, 40);
    assert_eq!(host.told.borrow().len(), 1, "amanhecer");
    host.day.set(DAY + 12500);
    census::tick(&mut host, 60);
    census::tick(&mut host, 80);
    assert_eq!(host.told.borrow().len(), 3, "anoitecer: produção e pedido, uma vez");
}
