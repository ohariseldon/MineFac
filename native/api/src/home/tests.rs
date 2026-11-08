use super::*;
use horizon_client::mod_api::{NativeServerPlayer, ModBlockEntity};
use std::collections::HashMap;

/// Os testes daqui mexem no estado global da casa: um de cada vez. Os relógios começam em
/// `T0`, longe do relógio dos outros testes, para o bônus nunca vazar para eles.
static SERIAL: Mutex<()> = Mutex::new(());
const T0: u64 = 50_000_000;

#[derive(Default)]
struct World {
    blocks: HashMap<[i32; 3], String>,
    /// Altura do terreno; colunas com teto têm a altura do telhado.
    ground: i32,
    roofs: Vec<[i32; 4]>,
    players: Vec<NativeServerPlayer>,
    commands: Vec<String>,
    told: Vec<(u128, String)>,
    bars: Vec<(u128, String)>,
    maps: Vec<(u128, i32, usize)>,
    acting: Option<NativeServerPlayer>,
}

impl World {
    fn top(&self, x: i32, z: i32) -> i32 {
        self.roofs.iter().find(|[x0, z0, x1, z1]| (*x0..=*x1).contains(&x) && (*z0..=*z1).contains(&z))
            .map_or(self.ground, |_| self.ground + 7)
    }
}

impl Host for World {
    fn dimension(&self) -> &'static str {
        OVERWORLD
    }
    fn block(&self, pos: [i32; 3]) -> String {
        self.blocks.get(&pos).cloned().unwrap_or_else(|| {
            if pos[1] == self.ground - 1 { "minecraft:grass_block".into() } else { "minecraft:air".into() }
        })
    }
    fn surface_heights(&self, _: &str, columns: &[(i32, i32)], _: NativeHeightmap) -> Result<Vec<Option<i32>>, String> {
        Ok(columns.iter().map(|(x, z)| Some(self.top(*x, *z))).collect())
    }
    fn server_players(&self) -> Vec<NativeServerPlayer> {
        self.players.clone()
    }
    fn run_command(&mut self, line: &str) -> bool {
        self.commands.push(line.to_owned());
        true
    }
    fn tell(&mut self, uuid: u128, line: Text) -> bool {
        self.told.push((uuid, line.plain()));
        true
    }
    fn action_bar(&mut self, uuid: u128, line: Text) -> bool {
        self.bars.push((uuid, line.plain()));
        true
    }
    fn send_map(&mut self, uuid: u128, map_id: i32, colors: &[u8]) -> bool {
        self.maps.push((uuid, map_id, colors.len()));
        true
    }
    fn acting_player(&self) -> Option<NativeServerPlayer> {
        self.acting.clone()
    }
}

fn player(uuid: u128, name: &str, at: [f64; 3]) -> NativeServerPlayer {
    NativeServerPlayer { uuid, name: name.to_owned(), dimension: OVERWORLD.to_owned(), position: at, ..Default::default() }
}

fn panel() -> ModBlockEntity {
    let mut entity = ModBlockEntity::default();
    entity.id = workforce::CONTROL_CENTER.into();
    entity
}

/// Constrói a casa perto de um jogador em terreno plano e devolve o painel.
fn built(world: &mut World) -> [i32; 3] {
    world.players = vec![player(41, "Founder", [1000.5, 70.0, 1000.5])];
    tick(world, T0);
    tick(world, T0 + BUILD_WAIT + CHECK_EVERY as u64);
    let call = world.commands.iter().find(|line| line.contains(FUNCTION)).expect("a casa foi construída").clone();
    let numbers: Vec<i32> = call.split(' ').filter_map(|word| word.parse().ok()).collect();
    let [x, y, z] = <[i32; 3]>::try_from(&numbers[..3]).unwrap();
    [x + DASHBOARD[0], y + DASHBOARD[1], z + DASHBOARD[2]]
}

#[test]
fn the_founder_house_is_built_once_on_flat_ground_with_its_panel_bed_and_mural() {
    let _serial = SERIAL.lock().unwrap_or_else(|err| err.into_inner());
    reset();
    let mut world = World { ground: 70, ..World::default() };
    let dashboard = built(&mut world);
    let joined = world.commands.join("\n");
    assert!(joined.contains(GUARD), "confere a marca do mundo antes de construir\n{joined}");
    assert!(joined.contains(&format!("execute in {OVERWORLD} positioned")), "{joined}");
    assert!(joined.contains("data modify storage veio:home built set value 1b"));
    assert_eq!(dashboard[1], 70, "piso no nível do terreno (altura 70: topo em 69, piso em 69 + 1)");
    let calls = world.commands.len();
    tick(&mut world, T0 + BUILD_WAIT + 200);
    assert_eq!(world.commands.len(), calls, "uma casa só");
    assert!(world.told.iter().any(|(uuid, _)| *uuid == 41), "o jogador é avisado onde fica a casa");

    // A planta gerada tem o painel, a cama e o mural onde o Rust os procura.
    let plan = include_str!("../../../../content/data/veio/function/home/build.mcfunction");
    let [dx, dy, dz] = DASHBOARD;
    assert!(plan.contains(&format!("setblock ~{dx} ~{dy} ~{dz} veio:control_center[facing=west]")));
    let [bx, by, bz] = HOUSE_BED;
    assert!(plan.contains(&format!("setblock ~{bx} ~{by} ~{bz} minecraft:red_bed[facing=south,part=head]")));
    for map in 0..mural::MAPS as i32 {
        assert!(plan.contains(&format!("\"minecraft:map_id\":{}", mural::MAP_ID + map)), "moldura do mapa {map}");
    }
    let bed = [dashboard[0] - DASHBOARD[0] + bx, dashboard[1] - DASHBOARD[1] + by, dashboard[2] - DASHBOARD[2] + bz];
    assert!((bed[2] - dashboard[2]).abs() <= 3 && (bed[0] - dashboard[0]).abs() <= 1, "cama ao lado do painel");
}

#[test]
fn site_avoids_water_and_slopes() {
    let _serial = SERIAL.lock().unwrap_or_else(|err| err.into_inner());
    struct Lake;
    impl Host for Lake {
        fn surface_heights(&self, _: &str, columns: &[(i32, i32)], map: NativeHeightmap) -> Result<Vec<Option<i32>>, String> {
            // Um lago a oeste (x < 0): a água fica 3 acima do fundo. Encosta ao sul de z = -20.
            Ok(columns.iter().map(|(x, z)| {
                let ground = if *z > -20 { 70 + (z + 20) } else { 70 };
                Some(if *x < 0 && map == NativeHeightmap::MotionBlocking { ground + 3 } else { ground })
            }).collect())
        }
    }
    let [x, y, z] = site(&Lake, [0, 70, 0]);
    assert_eq!(x, 0, "fora do lago, o mais perto do centro");
    assert_eq!(z, -29, "a fileira mais perto está na encosta; a seguinte é plana");
    assert_eq!(y, 69, "piso no topo do terreno");
}

#[test]
fn being_home_with_the_panel_speeds_up_machines_and_food_adds_on_top() {
    let _serial = SERIAL.lock().unwrap_or_else(|err| err.into_inner());
    reset();
    let mut world = World { ground: 70, ..World::default() };
    let dashboard = built(&mut world);
    let mut entity = panel();
    let now = T0 + 10_000;
    // Ainda não é a casa: nenhum outro painel vira casa sozinho.
    let mut other = panel();
    center_tick(&mut world, [dashboard[0] + 40, 70, dashboard[2]], &mut other, 0, now);
    assert!(home_number(&other) == 0);
    center_tick(&mut world, dashboard, &mut entity, 0, now);
    assert_eq!(home_number(&entity), 1, "o painel colocado pela casa é o da casa");
    assert_eq!(super::dashboard(), Some(dashboard));

    // Na varanda, sem teto: não conta.
    let origin = [dashboard[0] - DASHBOARD[0], dashboard[2] - DASHBOARD[2]];
    world.roofs = vec![[origin[0] + 1, origin[1] + 1, origin[0] + 11, origin[1] + 9]];
    let inside = [f64::from(dashboard[0]) - 3.5, 70.0, f64::from(dashboard[2]) + 0.5];
    world.players = vec![player(41, "Founder", [f64::from(origin[0]) + 6.5, 70.0, f64::from(origin[1]) + 10.5])];
    center_tick(&mut world, dashboard, &mut entity, 20, now);
    assert_eq!(time_factor(now), 1.0);
    // Dentro, perto do painel, debaixo do telhado.
    world.players = vec![player(41, "Founder", inside)];
    center_tick(&mut world, dashboard, &mut entity, 40, now);
    assert!((time_factor(now + 10) - HOME_TIME).abs() < 1e-6);
    assert!(world.bars.iter().any(|(uuid, _)| *uuid == 41), "aviso ao entrar");
    assert_eq!(time_factor(now + WINDOW + 1), 1.0, "sem nova verificação o bônus acaba");
    assert_eq!(time_factor(5), 1.0, "relógio de outra sessão não herda o bônus");
    let duration = upgrades::apply(100, 10, upgrades::Effect { time: HOME_TIME, power: 1.0, pollution: 1.0, bonus: 0 }).0;
    assert_eq!(duration, 87, "ciclo de 100 ticks passa a 87: 15% mais produtos por minuto");

    // Banquete em casa: dura o dobro e se soma ao bônus da casa.
    let feast = food::eaten(41, "Founder", OVERWORLD, inside, "minecraft:rabbit_stew", now);
    assert_eq!(feast, Some(food::FEAST_TICKS * 2));
    assert!((time_factor(now + 10) - HOME_TIME * food::FEAST_TIME).abs() < 1e-6);
    // Uma refeição fora de casa, depois, não encurta o banquete.
    assert_eq!(food::eaten(41, "Founder", OVERWORLD, [0.0, 70.0, 0.0], "minecraft:bread", now), Some(food::MEAL_TICKS));
    assert!((food::time_factor(now + food::FEAST_TICKS * 2 - 1) - food::FEAST_TIME).abs() < 1e-6);
    assert_eq!(food::time_factor(now + food::FEAST_TICKS * 2 + 1), 1.0, "a refeição já tinha acabado");
    assert_eq!(food::eaten(41, "Founder", OVERWORLD, inside, "minecraft:apple", now), None, "lanche não dá bônus");
    tick(&mut world, now + 1);
    assert!(world.commands.iter().any(|line| line.starts_with("effect give Founder minecraft:haste")));
    assert!(world.commands.iter().any(|line| line.starts_with("effect give Founder minecraft:regeneration")));
    reset();
}

fn home_number(entity: &ModBlockEntity) -> i32 {
    number(entity, "Home")
}

#[test]
fn only_two_beds_and_the_first_one_is_at_home() {
    let _serial = SERIAL.lock().unwrap_or_else(|err| err.into_inner());
    reset();
    let mut world = World { ground: 70, ..World::default() };
    let dashboard = built(&mut world);
    world.told.clear();
    let mut entity = panel();
    center_tick(&mut world, dashboard, &mut entity, 0, T0);
    let house_bed = [dashboard[0], dashboard[1], dashboard[2] + 3];
    let far = [dashboard[0] + 200, 64, dashboard[2]];
    let farther = [dashboard[0] - 300, 64, dashboard[2]];

    assert_eq!(spawn_point(&world, 41, OVERWORLD, far, "bed"), Flow::Cancel, "a primeira cama é a da casa");
    assert_eq!(bed_enter(&world, 41, far), Flow::Cancel);
    assert_eq!(spawn_point(&world, 41, OVERWORLD, far, "command"), Flow::Continue, "comandos não contam");
    assert!(!has_bed(41));
    assert_eq!(spawn_point(&world, 41, OVERWORLD, house_bed, "bed"), Flow::Continue);
    assert!(has_bed(41));
    world.blocks.insert(far, "minecraft:blue_bed".into());
    assert_eq!(spawn_point(&world, 41, OVERWORLD, far, "bed"), Flow::Continue, "a segunda, fora");
    assert_eq!(bed_enter(&world, 41, [far[0], far[1], far[2] + 1]), Flow::Continue, "a outra metade da mesma cama");
    world.blocks.insert(farther, "minecraft:white_bed".into());
    assert_eq!(spawn_point(&world, 41, OVERWORLD, farther, "bed"), Flow::Cancel, "terceira cama: não");
    assert_eq!(bed_enter(&world, 41, farther), Flow::Cancel);
    // Quebrou a de fora: a nova pode ser marcada.
    world.blocks.remove(&far);
    assert_eq!(spawn_point(&world, 41, OVERWORLD, farther, "bed"), Flow::Continue);
    assert_eq!(bed_enter(&world, 41, house_bed), Flow::Continue, "a da casa continua valendo");
    tick(&mut world, T0 + 1);
    assert!(world.told.iter().filter(|(uuid, _)| *uuid == 41).count() >= 4, "recusas e camas marcadas são explicadas no chat");

    // As camas ficam no NBT do painel e voltam ao reabrir o mundo.
    center_tick(&mut world, dashboard, &mut entity, 1, T0);
    let saved = entity.nbt.clone();
    reset();
    let mut reopened = panel();
    reopened.nbt = saved;
    center_tick(&mut world, dashboard, &mut reopened, 0, T0);
    assert!(has_bed(41));
    assert_eq!(bed_enter(&world, 41, farther), Flow::Continue);
    let beds = vec![Beds { player: 7 << 70, home: Some([1, -2, 3]), away: Some(("minecraft:the_nether".into(), [-4, 5, -6])) },
        Beds { player: 9, home: None, away: None }];
    assert_eq!(beds_round_trip(&beds), beds);
    reset();
}

#[test]
fn the_mural_maps_seen_terrain_marks_the_factory_and_reaches_players_at_home() {
    let _serial = SERIAL.lock().unwrap_or_else(|err| err.into_inner());
    reset();
    let mut world = World { ground: 70, ..World::default() };
    let center = [0, 70, 0];
    world.blocks.insert([-128, 69, -128], "veio:drill".into());
    world.blocks.insert([-127, 69, -128], "minecraft:water".into());
    world.players = vec![player(41, "Founder", [2.5, 70.0, 0.5]), player(42, "Away", [500.0, 70.0, 0.0])];
    mural::step(&mut world, center, 0);
    assert_eq!(world.maps.len(), mural::MAPS, "quatro mapas para quem está em casa");
    assert!(world.maps.iter().all(|(uuid, _, len)| *uuid == 41 && *len == 128 * 128));
    assert_eq!(mural::pixel(0, 0, 0), Some(15 * 4 + 1), "a máquina do Veio em laranja");
    assert_eq!(mural::pixel(0, 1, 0), Some(12 * 4 + 1), "água");
    assert_eq!(mural::pixel(0, 2, 0), Some(4 + 1), "grama");
    assert_eq!(mural::pixel(1, 0, 0), Some(4 + 1), "o mapa do nordeste também");
    assert_eq!(mural::pixel(0, 0, 1), Some(0), "linha ainda não lida: papel");
    mural::step(&mut world, center, 20);
    assert_eq!(world.maps.len(), mural::MAPS, "não reenvia a cada verificação");
    world.players.push(player(43, "Visitor", [-3.5, 70.0, 4.5]));
    mural::step(&mut world, center, 40);
    assert_eq!(world.maps.iter().filter(|(uuid, _, _)| *uuid == 43).count(), mural::MAPS, "quem chega recebe na hora");
    mural::step(&mut world, center, 200);
    assert_eq!(world.maps.len(), 4 * mural::MAPS, "de tempos em tempos, todos recebem de novo");
    assert_eq!(mural::shade(71, Some(70)), 2);
    assert_eq!(mural::shade(69, Some(70)), 0);
    assert_eq!(mural::color("minecraft:oak_leaves"), 7);
    assert_eq!(mural::color("minecraft:dirt_path"), 10);
    reset();
}
