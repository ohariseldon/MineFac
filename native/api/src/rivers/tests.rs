//! Testes do traçado dos rios num terreno sintético, sem o jogo.
use super::*;

/// Terreno por função de altura; o mar cobre o que fica abaixo do nível 63.
struct Synth {
    height: fn(i32, i32) -> f64,
    /// Raio, em chunks, do que está carregado em volta da origem.
    loaded: i32,
    extra: HashSet<(i32, i32)>,
    old: HashSet<(i32, i32)>,
    built: HashSet<(i32, i32)>,
    reads: usize,
}

impl Synth {
    fn new(height: fn(i32, i32) -> f64, loaded: i32) -> Self {
        Synth { height, loaded, extra: HashSet::new(), old: HashSet::new(), built: HashSet::new(), reads: 0 }
    }
    fn ground(&self, x: i32, z: i32) -> i32 {
        (self.height)(x, z).floor() as i32
    }
}

impl Terrain for Synth {
    fn read(&mut self, chunk: (i32, i32)) -> Option<ChunkCols> {
        let inside = chunk.0.abs() <= self.loaded && chunk.1.abs() <= self.loaded;
        if !inside && !self.extra.contains(&chunk) {
            return None;
        }
        self.reads += 1;
        let mut cols = Box::new([Col { ground: 0, top: 0, water: None, blocked: false, ours: false }; 256]);
        for (index, col) in cols.iter_mut().enumerate() {
            let x = chunk.0 * 16 + (index % 16) as i32;
            let z = chunk.1 * 16 + (index / 16) as i32;
            let ground = self.ground(x, z);
            let water = (ground < SEA).then_some(SEA);
            *col = Col { ground, top: water.unwrap_or(ground), water, blocked: self.built.contains(&(x, z)), ours: false };
        }
        Some(cols)
    }
    fn fresh(&self, chunk: (i32, i32)) -> bool {
        !self.old.contains(&chunk)
    }
}

/// Montanha cônica com pico de 140 no centro e mar em volta, com ondulação para criar
/// degraus, ravinas e buracos.
fn cone(x: i32, z: i32) -> f64 {
    let (x, z) = (f64::from(x), f64::from(z));
    let distance = (x * x + z * z).sqrt();
    140.0 - distance * 0.45 + (x * 0.11).sin() * 2.5 + (z * 0.07).cos() * 2.0
}

/// Vale em V que desce para +x: as encostas em z caem forte para o fundo, em z = 0.
fn valley(x: i32, z: i32) -> f64 {
    120.0 - f64::from(x) * 0.25 + f64::from(z.abs()) * 0.6
}

fn run(network: &mut Network, terrain: &mut Synth, rounds: usize) {
    for _ in 0..rounds {
        if network.fronts.is_empty() {
            break;
        }
        network.advance(terrain, 64);
    }
}

/// O rio de `id` a partir da nascente, seguindo `next`.
fn course(network: &Network, start: (i32, i32)) -> Vec<((i32, i32), Cell)> {
    let mut out = Vec::new();
    let mut cursor = Some(start);
    while let Some(point) = cursor {
        let Some(cell) = network.cells.get(&point) else { break };
        out.push((point, *cell));
        cursor = cell.next;
        assert!(out.len() < 20_000, "ciclo no rio");
    }
    out
}

#[test]
fn blocks_are_classified_from_the_top_down() {
    assert_eq!(classify("minecraft:grass_block"), Kind::Ground);
    assert_eq!(classify("minecraft:oak_log"), Kind::Plant);
    assert_eq!(classify("minecraft:short_grass"), Kind::Plant);
    assert_eq!(classify("minecraft:dirt_path"), Kind::Built);
    assert_eq!(classify("minecraft:oak_planks"), Kind::Built);
    assert_eq!(classify("veio:press"), Kind::Built);
    assert_eq!(classify("minecraft:sandstone"), Kind::Ground);
    assert_eq!(classify("minecraft:ice"), Kind::Water);
    // Árvore sobre grama: o chão fica sob o tronco, o topo na copa.
    let tree = [Kind::Ground, Kind::Ground, Kind::Plant, Kind::Plant, Kind::Plant, Kind::Air];
    assert_eq!(column(60, &tree), Col { ground: 61, top: 64, water: None, blocked: false, ours: false });
    let lake = [Kind::Ground, Kind::Water, Kind::Water, Kind::Air];
    assert_eq!(column(60, &lake), Col { ground: 60, top: 62, water: Some(62), blocked: false, ours: false });
}

#[test]
fn channels_widen_and_deepen_with_flow_and_levels_drop_in_real_steps() {
    let mut last = (0, 0);
    for flow in 1..60 {
        let now = shape(flow);
        assert!(now.0 >= last.0 && now.1 >= last.1, "nunca estreita: {flow}");
        last = now;
    }
    assert_eq!(shape(1), (0, 1), "nascente: um bloco de largura");
    assert!(shape(25).0 >= 3 && shape(25).1 >= 3, "rio grande: {:?}", shape(25));
    assert_eq!(next_level(90, 95), 90, "terreno mais alto: corta, a água não sobe");
    assert_eq!(next_level(90, 89), 87, "desce em degrau de pelo menos 3");
    assert_eq!(next_level(90, 70), 68, "cachoeira");
    assert_eq!(next_level(66, 64), SEA, "nunca abaixo do mar");
}

#[test]
fn a_spring_on_the_mountain_runs_downhill_with_falls_and_reaches_the_sea() {
    let mut terrain = Synth::new(cone, 24);
    let mut network = Network::default();
    let start = (6, -4);
    let ground = terrain.ground(start.0, start.1);
    network.spring(&mut terrain, start.0, start.1, ground);
    run(&mut network, &mut terrain, 400);
    assert!(network.fronts.is_empty(), "a frente terminou");
    let river = course(&network, start);
    assert!(river.len() > 120, "o rio percorre a encosta: {}", river.len());
    for pair in river.windows(2) {
        assert!(pair[1].1.level <= pair[0].1.level, "a água só desce");
        let (a, b) = (pair[0].0, pair[1].0);
        assert!((a.0 - b.0).abs() <= 1 && (a.1 - b.1).abs() <= 1, "passos vizinhos");
    }
    let (end, last) = river.last().copied().unwrap();
    let distance = f64::from(end.0 * end.0 + end.1 * end.1).sqrt();
    assert!(last.level <= SEA + 1 || terrain.ground(end.0, end.1) <= SEA + 2, "desaguou: {last:?} a {distance}");
    let falls: Vec<i32> = river.windows(2).map(|pair| pair[0].1.level - pair[1].1.level).filter(|&drop| drop > 0).collect();
    assert!(falls.iter().all(|&drop| drop >= HOLD + 1 || river.iter().any(|(_, cell)| cell.level == SEA)), "degraus de 3+: {falls:?}");
    assert!(falls.len() >= 8, "muitas quedas: {falls:?}");
    assert!(river.last().unwrap().1.flow > river[0].1.flow, "a vazão cresce com a distância");
    assert!(!network.edits.is_empty(), "o leito foi escavado");
    let water_runs = network.edits.values().flatten().filter(|run| run.4 == WATER).count();
    assert!(water_runs >= river.len());
}

#[test]
fn nearby_springs_join_into_one_bigger_river() {
    let mut terrain = Synth::new(valley, 20);
    let mut network = Network::default();
    for start in [(0, -24), (6, 26)] {
        let ground = terrain.ground(start.0, start.1);
    network.spring(&mut terrain, start.0, start.1, ground);
    }
    run(&mut network, &mut terrain, 600);
    assert!(network.fronts.is_empty());
    let first = course(&network, (0, -24));
    let second = course(&network, (6, 26));
    let shared: HashSet<(i32, i32)> = first.iter().map(|(at, _)| *at).collect();
    let joined = second.iter().any(|(at, _)| shared.contains(at)) || first.iter().any(|(at, _)| second.iter().any(|(other, _)| other == at));
    assert!(joined, "os dois córregos se encontram");
    let biggest = network.cells.values().map(|cell| cell.flow).max().unwrap();
    assert!(biggest >= 3, "a jusante soma as nascentes e a distância: {biggest}");
    let mouth = first.last().unwrap().1.flow.max(second.last().unwrap().1.flow);
    assert!(mouth > 2, "na foz o rio é maior que cada córrego: {mouth}");
}

#[test]
fn fronts_wait_for_unexplored_chunks_and_continue_later() {
    let mut terrain = Synth::new(cone, 1);
    let mut network = Network::default();
    let ground = terrain.ground(6, -4);
    network.spring(&mut terrain, 6, -4, ground);
    run(&mut network, &mut terrain, 50);
    assert_eq!(network.fronts.len(), 1, "esperando o chunk vizinho");
    let waiting = network.fronts[0].travelled;
    run(&mut network, &mut terrain, 50);
    assert_eq!(network.fronts[0].travelled, waiting, "parado enquanto o chunk não existe");
    terrain.loaded = 24;
    run(&mut network, &mut terrain, 400);
    assert!(network.fronts.is_empty(), "explorado, o rio segue até o mar");
}

#[test]
fn rivers_go_around_buildings_and_never_dig_old_terrain() {
    let mut terrain = Synth::new(valley, 20);
    // Uma vila no fundo do vale, logo abaixo da nascente.
    for x in 10..18 {
        for z in -3..=3 {
            terrain.built.insert((x, z));
        }
    }
    // E terreno antigo mais adiante.
    for cz in -20..=20 {
        terrain.old.insert((6, cz));
    }
    let mut network = Network::default();
    let ground = terrain.ground(0, 0);
    network.spring(&mut terrain, 0, 0, ground);
    run(&mut network, &mut terrain, 400);
    for (x, z) in network.cells.keys() {
        assert!(!terrain.built.contains(&(*x, *z)), "o rio contorna a vila");
    }
    for runs in network.edits.values() {
        for run in runs {
            assert!(!terrain.built.contains(&(run.0, run.1)), "nada escavado na vila: {run:?}");
            assert!(!terrain.old.contains(&chunk_of(run.0, run.1)), "nada escavado em terreno antigo: {run:?}");
        }
    }
    assert!(network.cells.keys().all(|&(x, _)| x < 6 * 16 + 16), "o rio para no terreno antigo");
}

#[test]
fn springs_rise_only_on_high_fresh_ground_and_keep_their_distance() {
    let mut terrain = Synth::new(cone, 12);
    let mut network = Network::default();
    for cx in -12..=12 {
        for cz in -12..=12 {
            assert!(network.scan(&mut terrain, (cx, cz)));
        }
    }
    assert!(!network.springs.is_empty(), "a montanha tem nascentes");
    for &(x, z) in &network.springs {
        assert!(terrain.ground(x, z) >= SPRING_MIN - 2, "nascente no alto: {x} {z}");
    }
    for (i, a) in network.springs.iter().enumerate() {
        for b in &network.springs[i + 1..] {
            assert!((a.0 - b.0).abs() >= SPRING_SPACING || (a.1 - b.1).abs() >= SPRING_SPACING, "{a:?} {b:?}");
        }
    }
    // Fora da área carregada, a procura espera.
    assert!(!network.scan(&mut terrain, (40, 40)));
}

#[test]
fn the_network_survives_a_restart() {
    let mut terrain = Synth::new(valley, 20);
    let mut network = Network::default();
    let ground = terrain.ground(0, -24);
    network.spring(&mut terrain, 0, -24, ground);
    run(&mut network, &mut terrain, 3);
    let text = network.save_text();
    let loaded = Network::load_text(&text);
    assert_eq!(loaded.next_id, network.next_id);
    assert_eq!(loaded.springs, network.springs);
    assert_eq!(loaded.cells, network.cells);
    assert_eq!(loaded.fronts.len(), network.fronts.len());
    let (a, b) = (&loaded.fronts[0], &network.fronts[0]);
    assert_eq!((a.id, a.x, a.z, a.level, a.base, a.travelled, a.last), (b.id, b.x, b.z, b.level, b.base, b.travelled, b.last));
    assert!(Network::load_text("lixo\nC 1 2\nF x").cells.is_empty(), "linhas ruins são ignoradas");
}

#[test]
fn carving_edits_are_grouped_by_chunk_with_the_shared_palette() {
    let mut terrain = Synth::new(cone, 4);
    let mut network = Network::default();
    network.carve(&mut terrain, 15, 8, 120, 1, 2);
    let mut chunks = HashSet::new();
    while let Some((chunk, edit)) = network.take_edit() {
        assert!(chunks.insert(chunk));
        assert_eq!(edit.palette.len(), PALETTE.len());
        assert_eq!(edit.filters, vec!["minecraft:air".to_owned()]);
        assert!(edit.runs.iter().all(|run| chunk_of(run.x, run.z) == chunk && (run.block as usize) < PALETTE.len()));
    }
    assert_eq!(chunks.len(), 2, "o canal na borda toca dois chunks");
    assert!(network.edits.is_empty() && network.edit_order.is_empty());
}
