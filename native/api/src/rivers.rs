//! Rios com nascente: água que brota no alto de algumas montanhas, desce pelo caminho mais
//! baixo, junta-se a outros córregos, fica mais larga e funda e termina no mar ou num lago.
//!
//! O Horizon não lê worldgen de data pack e o gancho de geração substitui o terreno inteiro.
//! Para manter o terreno vanilla da seed, os rios são traçados por cima dele, logo depois de o
//! chunk ser gerado: `chunk_load` com `new` marca o chunk como recém-gerado, e o tick lê as
//! colunas (`read_columns`), escolhe nascentes, avança as frentes de cada rio e escava o leito
//! com `edit_blocks`. Só chunks recém-gerados são escavados: construções e mundos antigos
//! ficam intactos, e terreno com blocos feitos pelo homem (vilas) é contornado.
//!
//! O traçado segue a descida mais íngreme. Num buraco sem saída, procura a passagem com o
//! menor corte até um ponto mais baixo (no máximo 8 blocos de corte); sem passagem, vira lagoa.
//! O nível da água só desce, em degraus de pelo menos 3 blocos: daí as quedas e cachoeiras
//! onde dá para represar. A vazão conta nascentes somadas mais um tanto pela distância
//! percorrida (a chuva da bacia); largura e profundidade crescem com ela.
//!
//! As frentes esperam quando o próximo chunk ainda não existe e continuam quando o jogador
//! explora. O estado fica em `<mundo>/veio/rios.txt`.
use horizon_client::mod_api::{Host, NativeBlockEdit, NativeBlockRun, NativeHeightmap};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::path::PathBuf;
use std::sync::Mutex;

pub(crate) const OVERWORLD: &str = "minecraft:overworld";
/// Nível do mar do Overworld.
pub(crate) const SEA: i32 = 63;
/// Altura mínima do terreno para brotar uma nascente.
pub(crate) const SPRING_MIN: i32 = 100;
/// Distância mínima entre nascentes.
const SPRING_SPACING: i32 = 40;
/// Ticks (do relógio do mod) em que um chunk recém-gerado ainda pode ser escavado.
pub(crate) const FRESH_WINDOW: u64 = 6000;
/// Blocos lidos abaixo da superfície: atravessa árvores até o chão.
const READ_BELOW: u32 = 28;
/// Corte máximo em terreno mais alto que a água (passagem por uma crista).
pub(crate) const MAX_CUT: i32 = 8;
/// Profundidade extra do canal a cada descida: as quedas têm pelo menos `HOLD + 1` blocos.
pub(crate) const HOLD: i32 = 2;
/// Alcance da busca por passagem num buraco.
const BREACH: i32 = 24;
/// Blocos percorridos que somam uma unidade de vazão.
pub(crate) const RUN_PER_FLOW: i32 = 48;
const STEPS_PER_TICK: usize = 48;
const SCANS_PER_TICK: usize = 2;
const EDIT_CHUNKS_PER_TICK: usize = 2;
const SAVE_EVERY: u64 = 1200;
const CHECK_WORLD_EVERY: u64 = 100;
const CACHE_LIMIT: usize = 1536;
/// Um rio para depois de tantos blocos sem achar o mar (planaltos sem fim).
const MAX_TRAVEL: i32 = 6000;

/// Uma coluna do terreno como o traçado a vê.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Col {
    /// Bloco de chão mais alto (sem árvores nem plantas).
    pub ground: i32,
    /// Bloco mais alto de qualquer tipo (copas, troncos).
    pub top: i32,
    /// Superfície de água natural acima do chão.
    pub water: Option<i32>,
    /// Construção (vila, blocos de mod): o rio contorna.
    pub blocked: bool,
    /// Canal escavado por um rio do Veio: a água dele não é foz, e `ground` segue o terreno
    /// original, para o traçado não confundir o próprio leito com uma descida.
    pub ours: bool,
}

pub(crate) type ChunkCols = Box<[Col; 256]>;

/// O que o traçado precisa do mundo. No jogo, `HostTerrain`; nos testes, um terreno sintético.
pub(crate) trait Terrain {
    /// Colunas do chunk, ou `None` se ele não estiver carregado.
    fn read(&mut self, chunk: (i32, i32)) -> Option<ChunkCols>;
    /// Chunk gerado há pouco, que pode ser escavado.
    fn fresh(&self, chunk: (i32, i32)) -> bool;
}

/// Classe de um bloco lido de cima para baixo.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Kind { Air, Water, Plant, Built, Ground }

pub(crate) fn classify(name: &str) -> Kind {
    let name = name.strip_prefix("minecraft:").unwrap_or(name);
    if matches!(name, "air" | "cave_air" | "void_air") {
        return Kind::Air;
    }
    if matches!(name, "water" | "bubble_column" | "ice" | "kelp" | "kelp_plant" | "seagrass" | "tall_seagrass") {
        return Kind::Water;
    }
    if name == "lava" {
        return Kind::Built; // Lagos de lava: o rio desvia.
    }
    if name.contains(':') {
        return Kind::Built; // Blocos de mod, inclusive as máquinas do Veio.
    }
    const BUILT: [&str; 44] = [
        "planks", "cobblestone", "bricks", "glass", "wool", "_bed", "door", "fence", "torch", "lantern",
        "crafting", "furnace", "chest", "barrel", "bell", "hay_block", "dirt_path", "farmland", "composter",
        "lectern", "smoker", "cartography", "fletching", "loom", "stonecutter", "grindstone", "smithing",
        "cauldron", "brewing", "carpet", "stairs", "slab", "_wall", "trapdoor", "ladder", "sign", "banner",
        "pane", "glazed", "rail", "bars", "anvil", "cut_sandstone", "smooth_sandstone",
    ];
    if BUILT.iter().any(|part| name.contains(part)) {
        return Kind::Built;
    }
    const GROUND: [&str; 30] = [
        "stone", "dirt", "grass_block", "sand", "gravel", "clay", "deepslate", "granite", "diorite",
        "andesite", "tuff", "terracotta", "podzol", "mycelium", "snow_block", "packed_ice", "blue_ice",
        "mud", "moss_block", "calcite", "_ore", "netherrack", "basalt", "dripstone", "obsidian", "bedrock",
        "powder_snow", "amethyst", "magma", "soul_s",
    ];
    if GROUND.iter().any(|part| name.contains(part)) {
        return Kind::Ground;
    }
    Kind::Plant
}

/// Uma coluna a partir dos blocos lidos, de baixo para cima, a partir de `bottom`.
pub(crate) fn column(bottom: i32, blocks: &[Kind]) -> Col {
    let mut top = None;
    let mut water = None;
    for (index, kind) in blocks.iter().enumerate().rev() {
        let y = bottom + index as i32;
        match kind {
            Kind::Air => continue,
            Kind::Water => {
                top.get_or_insert(y);
                water.get_or_insert(y);
            }
            Kind::Plant => {
                top.get_or_insert(y);
            }
            Kind::Built | Kind::Ground => {
                return Col { ground: y, top: top.unwrap_or(y), water, blocked: *kind == Kind::Built, ours: false };
            }
        }
    }
    let ground = bottom - 1;
    Col { ground, top: top.unwrap_or(ground), water, blocked: false, ours: false }
}

impl Col {
    /// Água natural (mar, lago, rio vanilla) com a superfície até `level`.
    fn outlet(&self, level: i32) -> bool {
        !self.ours && self.water.is_some_and(|surface| surface <= level)
    }
}

/// Raio do canal (0 = um bloco de largura) e profundidade pela vazão.
pub(crate) fn shape(flow: i32) -> (i32, i32) {
    let root = f64::from(flow.max(1)).sqrt();
    let radius = (root.floor() as i32 - 1).clamp(0, 4);
    let depth = (1 + (root / 1.5).floor() as i32).clamp(1, 4);
    (radius, depth)
}

fn chunk_of(x: i32, z: i32) -> (i32, i32) {
    (x.div_euclid(16), z.div_euclid(16))
}

fn index_in_chunk(x: i32, z: i32) -> usize {
    (x.rem_euclid(16) + z.rem_euclid(16) * 16) as usize
}

fn hash(a: i32, b: i32, salt: u64) -> u64 {
    let mut h = (a as i64 as u64).wrapping_mul(0x9e37_79b9_7f4a_7c15)
        ^ (b as i64 as u64).wrapping_mul(0xc2b2_ae3d_27d4_eb4f) ^ salt;
    h ^= h >> 31;
    h = h.wrapping_mul(0xbf58_476d_1ce4_e5b9);
    h ^ (h >> 29)
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Front {
    pub id: u32,
    pub x: i32,
    pub z: i32,
    pub level: i32,
    /// Vazão vinda de cima: nascentes e afluentes.
    pub base: i32,
    pub travelled: i32,
    pub last: (i32, i32),
    /// Passagem planejada por uma crista.
    pub path: VecDeque<(i32, i32)>,
    recent: VecDeque<(i32, i32)>,
}

impl Front {
    pub(crate) fn flow(&self) -> i32 {
        self.base + self.travelled / RUN_PER_FLOW
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Cell {
    pub id: u32,
    pub level: i32,
    pub flow: i32,
    pub radius: i32,
    pub next: Option<(i32, i32)>,
}

/// O que um passo da frente fez.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Step {
    Moved,
    /// O chunk ainda não existe: espera o jogador explorar.
    Wait((i32, i32)),
    /// Chegou ao mar, a um lago ou a outro rio; ou parou em terreno antigo ou numa lagoa.
    Done,
}

/// A rede de rios de um mundo.
#[derive(Default)]
pub(crate) struct Network {
    pub next_id: u32,
    pub springs: Vec<(i32, i32)>,
    pub fronts: Vec<Front>,
    pub cells: HashMap<(i32, i32), Cell>,
    cache: HashMap<(i32, i32), ChunkCols>,
    /// Escavações por chunk, na ordem em que foram decididas.
    pub edits: HashMap<(i32, i32), Vec<(i32, i32, i32, u32, u32, Option<u32>)>>,
    pub edit_order: VecDeque<(i32, i32)>,
    pub dirty: bool,
}

/// Paleta das escavações. A ordem é a do índice gravado nas edições.
pub(crate) const PALETTE: [&str; 8] = [
    "minecraft:air", "minecraft:water", "minecraft:gravel", "minecraft:sand", "minecraft:stone",
    "minecraft:dirt", "minecraft:grass_block", "minecraft:mossy_cobblestone",
];
const AIR: u32 = 0;
const WATER: u32 = 1;
const GRAVEL: u32 = 2;
const SAND: u32 = 3;
const STONE: u32 = 4;
const DIRT: u32 = 5;
const GRASS: u32 = 6;
const MOSSY: u32 = 7;
/// Filtro "só onde for ar": vedar cavernas sob o leito.
const ONLY_AIR: u32 = 0;
pub(crate) const FILTERS: [&str; 1] = ["minecraft:air"];

enum Look { Unloaded((i32, i32)), Old, Col(Col) }

impl Network {
    fn look(&mut self, terrain: &mut dyn Terrain, x: i32, z: i32) -> Look {
        let chunk = chunk_of(x, z);
        if !self.cache.contains_key(&chunk) {
            match terrain.read(chunk) {
                Some(cols) => {
                    if self.cache.len() >= CACHE_LIMIT && self.edits.is_empty() {
                        self.cache.clear();
                    }
                    self.cache.insert(chunk, cols);
                }
                None => return Look::Unloaded(chunk),
            }
        }
        let col = self.cache[&chunk][index_in_chunk(x, z)];
        if terrain.fresh(chunk) { Look::Col(col) } else { Look::Old }
    }

    fn set_col(&mut self, x: i32, z: i32, col: Col) {
        if let Some(cols) = self.cache.get_mut(&chunk_of(x, z)) {
            cols[index_in_chunk(x, z)] = col;
        }
    }

    fn queue(&mut self, x: i32, z: i32, y: i32, height: i32, block: u32, only: Option<u32>) {
        if height <= 0 {
            return;
        }
        let chunk = chunk_of(x, z);
        let list = self.edits.entry(chunk).or_default();
        if list.is_empty() {
            self.edit_order.push_back(chunk);
        }
        list.push((x, z, y, height as u32, block, only));
    }

    /// Escava o canal em volta de `(x, z)` com a água no nível `level`, e levanta margens
    /// onde o terreno em volta é mais baixo que a água.
    pub(crate) fn carve(&mut self, terrain: &mut dyn Terrain, x: i32, z: i32, level: i32, radius: i32, depth: i32) {
        let bed = level - depth;
        let reach = radius + 1;
        let near_sea = level <= SEA + 6;
        for dz in -reach..=reach {
            for dx in -reach..=reach {
                let distance = f64::from(dx * dx + dz * dz).sqrt();
                let (cx, cz) = (x + dx, z + dz);
                let Look::Col(col) = self.look(terrain, cx, cz) else { continue };
                if col.blocked {
                    continue;
                }
                if distance <= f64::from(radius) + 0.5 {
                    let top = col.top.max(col.ground);
                    if top > level {
                        self.queue(cx, cz, level + 1, (top - level).min(28), AIR, None);
                    }
                    self.queue(cx, cz, bed + 1, depth, WATER, None);
                    self.queue(cx, cz, bed, 1, if near_sea { SAND } else { GRAVEL }, None);
                    if col.ground < bed - 1 {
                        // Encosta mais baixa que o leito: aterro por baixo, como um dique.
                        self.queue(cx, cz, col.ground + 1, bed - 1 - col.ground, DIRT, None);
                    } else {
                        // Veda cavernas logo abaixo do leito.
                        self.queue(cx, cz, bed - 2, 2, STONE, Some(ONLY_AIR));
                    }
                    self.set_col(cx, cz, Col { top: level, water: Some(level), ours: true, ..col });
                } else if distance <= f64::from(radius) + 1.5 && col.water.is_none() && col.top < level {
                    // Margem: o terreno mais baixo que a água vira barranco até o nível dela.
                    let height = level - col.top;
                    self.queue(cx, cz, col.top + 1, height - 1, DIRT, None);
                    self.queue(cx, cz, level, 1, GRASS, None);
                    // O traçado segue lendo o chão original (o barranco à frente é o caminho da
                    // queda); o topo sobe para o próximo corte remover o barranco.
                    self.set_col(cx, cz, Col { top: level.max(col.top), ..col });
                }
            }
        }
        self.dirty = true;
    }

    fn other_river(&self, id: u32, x: i32, z: i32, reach: i32, level: i32) -> Option<(i32, i32)> {
        let mut best: Option<((i32, i32), i32)> = None;
        for dz in -reach..=reach {
            for dx in -reach..=reach {
                let at = (x + dx, z + dz);
                if let Some(cell) = self.cells.get(&at)
                    && cell.id != id && cell.level <= level
                    && best.is_none_or(|(_, old)| cell.level < old)
                {
                    best = Some((at, cell.level));
                }
            }
        }
        best.map(|(at, _)| at)
    }

    fn own(&self, id: u32, x: i32, z: i32) -> bool {
        self.cells.get(&(x, z)).is_some_and(|cell| cell.id == id)
    }

    /// Junta a vazão de um afluente ao rio que passa por `at`, até a foz dele.
    fn join(&mut self, terrain: &mut dyn Terrain, at: (i32, i32), added: i32) {
        let mut cursor = Some(at);
        let mut guard = 0;
        let mut target = None;
        while let Some(point) = cursor {
            guard += 1;
            if guard > 20_000 {
                break;
            }
            let Some(cell) = self.cells.get_mut(&point) else { break };
            cell.flow += added;
            target = Some(cell.id);
            let (radius, depth) = shape(cell.flow);
            let (level, wider) = (cell.level, radius > cell.radius);
            if wider {
                cell.radius = radius;
            }
            cursor = cell.next;
            if wider {
                self.carve(terrain, point.0, point.1, level, radius, depth);
            }
        }
        if let Some(id) = target
            && let Some(front) = self.fronts.iter_mut().find(|front| front.id == id)
        {
            front.base += added;
        }
        self.dirty = true;
    }

    /// Nova nascente brotando em `(x, z)`, no chão `ground`.
    pub(crate) fn spring(&mut self, terrain: &mut dyn Terrain, x: i32, z: i32, ground: i32) {
        self.next_id += 1;
        let id = self.next_id;
        self.springs.push((x, z));
        self.cells.insert((x, z), Cell { id, level: ground, flow: 1, radius: 0, next: None });
        self.carve(terrain, x, z, ground, 0, 1);
        // Pedras com musgo em volta da mina, para o jogador reconhecê-la.
        for (dx, dz) in [(1, 1), (-1, -1), (1, -1), (-1, 1)] {
            if let Look::Col(col) = self.look(terrain, x + dx, z + dz)
                && !col.blocked && col.water.is_none() && !col.ours && col.ground >= ground
            {
                self.queue(x + dx, z + dz, ground + 1, 1, MOSSY, Some(ONLY_AIR));
            }
        }
        let mut recent = VecDeque::new();
        recent.push_back((x, z));
        self.fronts.push(Front { id, x, z, level: ground, base: 1, travelled: 0, last: (0, 0), path: VecDeque::new(), recent });
    }

    fn near_spring(&self, x: i32, z: i32) -> bool {
        self.springs.iter().any(|&(sx, sz)| (sx - x).abs() < SPRING_SPACING && (sz - z).abs() < SPRING_SPACING)
    }

    /// Procura nascente num chunk recém-gerado. Devolve `false` se o chunk não está carregado.
    pub(crate) fn scan(&mut self, terrain: &mut dyn Terrain, chunk: (i32, i32)) -> bool {
        if !terrain.fresh(chunk) {
            return true;
        }
        let Some(cols) = terrain.read(chunk) else { return false };
        self.cache.insert(chunk, cols.clone());
        let land = cols.iter().enumerate().filter(|(_, col)| !col.blocked && col.water.is_none());
        let Some(peak) = land.clone().map(|(_, col)| col.ground).max() else { return true };
        if peak < SPRING_MIN {
            return true;
        }
        let chance = (20 + (peak - SPRING_MIN) / 2).clamp(20, 50) as u64;
        if hash(chunk.0, chunk.1, 0x5be1_1a) % 100 >= chance {
            return true;
        }
        // A mina brota na encosta, um pouco abaixo do pico.
        let (index, col) = land.filter(|(_, col)| col.ground <= peak - 2).max_by_key(|(_, col)| col.ground)
            .unwrap_or_else(|| cols.iter().enumerate().max_by_key(|(_, col)| col.ground).expect("256 colunas"));
        let x = chunk.0 * 16 + (index % 16) as i32;
        let z = chunk.1 * 16 + (index / 16) as i32;
        if self.near_spring(x, z) || self.other_river(0, x, z, 6, i32::MAX).is_some() {
            return true;
        }
        self.spring(terrain, x, z, col.ground);
        true
    }

    /// Um passo da frente `index`.
    pub(crate) fn step(&mut self, terrain: &mut dyn Terrain, index: usize) -> Step {
        let front = self.fronts[index].clone();
        let (x, z, level, id) = (front.x, front.z, front.level, front.id);
        let (radius, depth) = shape(front.flow());
        if front.travelled >= MAX_TRAVEL {
            return Step::Done;
        }
        // Chegou à água de outro rio, lago ou mar?
        if let Some(at) = self.other_river(id, x, z, radius + 1, level) {
            let flow = front.flow();
            self.join(terrain, at, flow);
            if let Some(cell) = self.cells.get_mut(&(x, z)) {
                cell.next = Some(at);
            }
            return Step::Done;
        }
        let planned = front.path.front().copied();
        let next = if let Some(point) = planned {
            match self.look(terrain, point.0, point.1) {
                Look::Unloaded(chunk) => return Step::Wait(chunk),
                Look::Old => return Step::Done,
                Look::Col(col) => (point, col),
            }
        } else {
            let mut best: Option<((i32, i32), Col, i64)> = None;
            for (dx, dz) in [(1, 0), (-1, 0), (0, 1), (0, -1), (1, 1), (1, -1), (-1, 1), (-1, -1)] {
                let at = (x + dx, z + dz);
                if self.own(id, at.0, at.1) || excluded(&front.recent, (x, z), at) {
                    continue;
                }
                let col = match self.look(terrain, at.0, at.1) {
                    Look::Unloaded(chunk) => return Step::Wait(chunk),
                    Look::Old => continue,
                    Look::Col(col) => col,
                };
                if col.blocked {
                    continue;
                }
                if col.outlet(level) {
                    return Step::Done; // Foz: mar, lago ou rio do próprio mundo.
                }
                // Mais baixo primeiro; empate: seguir a direção, depois um sorteio estável.
                let straight = i64::from(dx * front.last.0 + dz * front.last.1);
                let score = i64::from(col.ground) * 64 - straight * 8 - (hash(at.0, at.1, 7) % 8) as i64;
                if best.is_none_or(|(_, _, old)| score < old) {
                    best = Some((at, col, score));
                }
            }
            match best {
                Some((at, col, _)) if col.ground <= level + 1 => (at, col),
                _ => match self.breach(terrain, &front) {
                    Ok(path) => {
                        let first = path[0];
                        // O primeiro ponto é o deste passo; a frente guarda o resto do caminho.
                        self.fronts[index].path = path.into_iter().skip(1).collect();
                        let Look::Col(col) = self.look(terrain, first.0, first.1) else { return Step::Done };
                        (first, col)
                    }
                    Err(Some(chunk)) => return Step::Wait(chunk),
                    Err(None) => return Step::Done, // Lagoa: nada mais baixo por perto.
                },
            }
        };
        let (at, col) = next;
        // Os chunks que o canal toca precisam existir.
        for (dx, dz) in [(-1, -1), (1, -1), (-1, 1), (1, 1)] {
            let edge = (at.0 + dx * (radius + 1), at.1 + dz * (radius + 1));
            if let Look::Unloaded(chunk) = self.look(terrain, edge.0, edge.1) {
                return Step::Wait(chunk);
            }
        }
        if col.outlet(level) {
            return Step::Done;
        }
        let new_level = next_level(level, col.ground);
        let flow = front.flow() + i32::from((front.travelled + 1) % RUN_PER_FLOW == 0);
        let (radius, depth) = if flow == front.flow() { (radius, depth) } else { shape(flow) };
        self.carve(terrain, at.0, at.1, new_level, radius, depth);
        if let Some(cell) = self.cells.get_mut(&(x, z)) {
            cell.next = Some(at);
        }
        self.cells.insert(at, Cell { id, level: new_level, flow, radius, next: None });
        let front = &mut self.fronts[index];
        if planned.is_some() {
            front.path.pop_front();
        }
        front.last = (at.0 - x, at.1 - z);
        front.x = at.0;
        front.z = at.1;
        front.level = new_level;
        front.travelled += 1;
        front.recent.push_back(at);
        if front.recent.len() > 12 {
            front.recent.pop_front();
        }
        if new_level <= SEA && col.ground < SEA {
            return Step::Done; // No nível do mar e abaixo dele: desaguou.
        }
        Step::Moved
    }

    /// Passagem com o menor corte até um ponto mais baixo que a água (ou à água de outro rio).
    /// `Err(Some(chunk))`: falta carregar esse chunk; `Err(None)`: não há saída.
    fn breach(&mut self, terrain: &mut dyn Terrain, front: &Front) -> Result<Vec<(i32, i32)>, Option<(i32, i32)>> {
        use std::cmp::Reverse;
        let (x0, z0, level) = (front.x, front.z, front.level);
        let mut heap = BinaryHeap::new();
        let mut from: HashMap<(i32, i32), (i32, i32)> = HashMap::new();
        let mut best: HashMap<(i32, i32), (i32, i32)> = HashMap::new();
        let mut done: HashSet<(i32, i32)> = HashSet::new();
        let mut missing = None;
        heap.push(Reverse((0, 0, x0, z0)));
        while let Some(Reverse((cut, distance, x, z))) = heap.pop() {
            if !done.insert((x, z)) {
                continue;
            }
            if (x, z) != (x0, z0) {
                let goal = match self.look(terrain, x, z) {
                    Look::Col(col) => (col.ground < level && !col.ours) || col.outlet(level),
                    _ => false,
                } || self.cells.get(&(x, z)).is_some_and(|cell| cell.id != front.id && cell.level <= level);
                if goal {
                    let mut path = vec![(x, z)];
                    while let Some(&back) = from.get(path.last().expect("caminho")) {
                        if back == (x0, z0) {
                            break;
                        }
                        path.push(back);
                    }
                    path.reverse();
                    return Ok(path);
                }
            }
            for (dx, dz) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let at = (x + dx, z + dz);
                if (at.0 - x0).abs() > BREACH || (at.1 - z0).abs() > BREACH || done.contains(&at) || self.own(front.id, at.0, at.1) {
                    continue;
                }
                let col = match self.look(terrain, at.0, at.1) {
                    Look::Unloaded(chunk) => {
                        missing.get_or_insert(chunk);
                        continue;
                    }
                    Look::Old => continue,
                    Look::Col(col) => col,
                };
                if col.blocked {
                    continue;
                }
                let step_cut = (col.ground - level).max(0);
                if step_cut > MAX_CUT {
                    continue;
                }
                let key = (cut.max(step_cut), distance + 1);
                if best.get(&at).is_none_or(|&old| key < old) {
                    best.insert(at, key);
                    from.insert(at, (x, z));
                    heap.push(Reverse((key.0, key.1, at.0, at.1)));
                }
            }
        }
        Err(missing)
    }

    /// Avança as frentes, no máximo `budget` passos no total. Devolve os chunks esperados.
    pub(crate) fn advance(&mut self, terrain: &mut dyn Terrain, budget: usize) {
        let mut steps = 0;
        let mut index = 0;
        let mut progressed = false;
        while steps < budget && !self.fronts.is_empty() {
            if index >= self.fronts.len() {
                if !progressed {
                    break;
                }
                index = 0;
                progressed = false;
            }
            match self.step(terrain, index) {
                Step::Moved => {
                    progressed = true;
                    index += 1;
                }
                Step::Wait(_) => index += 1,
                Step::Done => {
                    self.fronts.remove(index);
                    self.dirty = true;
                }
            }
            steps += 1;
        }
    }

    /// Escavações de um chunk prontas para `edit_blocks`.
    pub(crate) fn take_edit(&mut self) -> Option<((i32, i32), NativeBlockEdit)> {
        let chunk = self.edit_order.pop_front()?;
        let runs = self.edits.remove(&chunk).unwrap_or_default();
        Some((chunk, NativeBlockEdit {
            palette: PALETTE.iter().map(|name| (*name).to_owned()).collect(),
            filters: FILTERS.iter().map(|name| (*name).to_owned()).collect(),
            runs: runs.into_iter().map(|(x, z, y, height, block, only)| NativeBlockRun {
                x, z, y, height, surface: None, block, only,
            }).collect(),
        }))
    }

    /// Maior vazão de rio a até `reach` blocos.
    pub(crate) fn flow_near(&self, x: i32, z: i32, reach: i32) -> i32 {
        let mut best = 0;
        for dz in -reach..=reach {
            for dx in -reach..=reach {
                if let Some(cell) = self.cells.get(&(x + dx, z + dz)) {
                    best = best.max(cell.flow);
                }
            }
        }
        best
    }

    pub(crate) fn save_text(&self) -> String {
        use std::fmt::Write;
        let mut out = format!("veio-rios 1\nN {}\n", self.next_id);
        for (x, z) in &self.springs {
            let _ = writeln!(out, "S {x} {z}");
        }
        for front in &self.fronts {
            let _ = writeln!(out, "F {} {} {} {} {} {} {} {}", front.id, front.x, front.z, front.level, front.base,
                front.travelled, front.last.0, front.last.1);
        }
        for ((x, z), cell) in &self.cells {
            let next = cell.next.map_or_else(|| "- -".to_owned(), |(nx, nz)| format!("{nx} {nz}"));
            let _ = writeln!(out, "C {x} {z} {} {} {} {} {next}", cell.id, cell.level, cell.flow, cell.radius);
        }
        out
    }

    pub(crate) fn load_text(text: &str) -> Self {
        let mut network = Network::default();
        for line in text.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let int = |i: usize| parts.get(i).and_then(|value| value.parse::<i32>().ok());
            match parts.first().copied() {
                Some("N") => network.next_id = int(1).unwrap_or(0).max(0) as u32,
                Some("S") => if let (Some(x), Some(z)) = (int(1), int(2)) { network.springs.push((x, z)) },
                Some("F") => if let (Some(id), Some(x), Some(z), Some(level), Some(base), Some(travelled), Some(lx), Some(lz)) =
                    (int(1), int(2), int(3), int(4), int(5), int(6), int(7), int(8))
                {
                    network.fronts.push(Front { id: id.max(0) as u32, x, z, level, base, travelled, last: (lx, lz),
                        path: VecDeque::new(), recent: VecDeque::from([(x, z)]) });
                },
                Some("C") => if let (Some(x), Some(z), Some(id), Some(level), Some(flow), Some(radius)) =
                    (int(1), int(2), int(3), int(4), int(5), int(6))
                {
                    let next = int(7).zip(int(8));
                    network.cells.insert((x, z), Cell { id: id.max(0) as u32, level, flow, radius, next });
                },
                _ => {}
            }
        }
        network
    }
}

/// O nível da água no próximo bloco: igual em terreno mais alto (o rio corta), mais baixo em
/// degraus de pelo menos `HOLD + 1` quando o terreno desce. Nunca abaixo do mar.
pub(crate) fn next_level(level: i32, ground: i32) -> i32 {
    if ground >= level {
        level
    } else {
        (ground - HOLD).max(SEA).min(level)
    }
}

/// Sem voltar para perto do trecho recente: o rio muda de direção no máximo 45 graus por bloco.
fn excluded(recent: &VecDeque<(i32, i32)>, current: (i32, i32), at: (i32, i32)) -> bool {
    recent.iter().any(|&point| point != current && (point.0 - at.0).abs() <= 1 && (point.1 - at.1).abs() <= 1)
}

/// O terreno real, pelas colunas do Horizon.
struct HostTerrain<'a> {
    host: &'a dyn Host,
    fresh: &'a HashMap<(i32, i32), u64>,
    now: u64,
}

impl Terrain for HostTerrain<'_> {
    fn read(&mut self, (cx, cz): (i32, i32)) -> Option<ChunkCols> {
        let columns: Vec<(i32, i32)> = (0..256).map(|i| (cx * 16 + i % 16, cz * 16 + i / 16)).collect();
        let read = self.host.read_columns(OVERWORLD, &columns, NativeHeightmap::MotionBlockingNoLeaves, (READ_BELOW, 1)).ok()?;
        let per = (READ_BELOW + 1) as usize;
        if read.bases.len() != 256 || read.blocks.len() < 256 * per {
            return None;
        }
        let kinds: Vec<Kind> = read.palette.iter().map(|name| classify(name)).collect();
        let mut cols = Box::new([Col { ground: 0, top: 0, water: None, blocked: false, ours: false }; 256]);
        let mut blocks = Vec::with_capacity(per);
        for (index, col) in cols.iter_mut().enumerate() {
            let base = read.bases[index]?;
            blocks.clear();
            blocks.extend(read.blocks[index * per..(index + 1) * per].iter()
                .map(|&block| kinds.get(block as usize).copied().unwrap_or(Kind::Air)));
            *col = column(base - READ_BELOW as i32, &blocks);
        }
        Some(cols)
    }

    fn fresh(&self, chunk: (i32, i32)) -> bool {
        self.fresh.get(&chunk).is_some_and(|&born| self.now.saturating_sub(born) <= FRESH_WINDOW)
    }
}

#[derive(Default)]
struct State {
    world: Option<PathBuf>,
    network: Network,
    fresh: HashMap<(i32, i32), u64>,
    scan: VecDeque<((i32, i32), u8)>,
    checked: u64,
    saved: u64,
    ticked: u64,
}

static STATE: Mutex<Option<State>> = Mutex::new(None);

fn with_state<T>(work: impl FnOnce(&mut State) -> T) -> T {
    let mut state = STATE.lock().unwrap_or_else(|err| err.into_inner());
    work(state.get_or_insert_with(State::default))
}

/// `chunk_load`: anota os chunks do Overworld que acabaram de ser gerados.
pub(crate) fn chunk_loaded(dimension: &str, chunk: [i32; 2], new: bool, now: u64) {
    if !new || dimension != OVERWORLD {
        return;
    }
    with_state(|state| {
        state.fresh.insert((chunk[0], chunk[1]), now);
        state.scan.push_back(((chunk[0], chunk[1]), 0));
    });
}

fn file(world: &std::path::Path) -> PathBuf {
    world.join("veio").join("rios.txt")
}

fn save(state: &State) {
    let Some(world) = &state.world else { return };
    let path = file(world);
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let temp = path.with_extension("tmp");
    if std::fs::write(&temp, state.network.save_text()).is_ok() {
        let _ = std::fs::rename(&temp, &path);
    }
}

/// Tick do servidor: troca de mundo, procura de nascentes, frentes, escavações e salvamento.
pub(crate) fn tick(host: &mut dyn Host, now: u64) {
    // Só no servidor, com o Overworld sendo simulado: a janela do cliente não lê colunas.
    if host.dimension() != OVERWORLD || host.surface_heights(OVERWORLD, &[], NativeHeightmap::MotionBlocking).is_err() {
        return;
    }
    let edits = with_state(|state| {
        if state.world.is_none() || now.saturating_sub(state.checked) >= CHECK_WORLD_EVERY {
            state.checked = now;
            let world = host.world_dir();
            if world != state.world && world.is_some() {
                if state.network.dirty {
                    save(state);
                }
                let network = world.as_ref().and_then(|dir| std::fs::read_to_string(file(dir)).ok())
                    .map(|text| Network::load_text(&text)).unwrap_or_default();
                state.world = world;
                state.network = network;
                // Fica só o que chegou depois do último tick: os chunks do mundo que abriu.
                let since = state.ticked.min(now);
                state.fresh.retain(|_, born| *born >= since);
            }
        }
        if state.world.is_none() {
            return Vec::new();
        }
        state.ticked = now;
        state.fresh.retain(|_, born| now.saturating_sub(*born) <= FRESH_WINDOW);
        let State { network, fresh, scan, .. } = state;
        let mut terrain = HostTerrain { host: &*host, fresh, now };
        for _ in 0..SCANS_PER_TICK {
            let Some((chunk, tries)) = scan.pop_front() else { break };
            if !network.scan(&mut terrain, chunk) && tries < 40 {
                scan.push_back((chunk, tries + 1));
            }
        }
        network.advance(&mut terrain, STEPS_PER_TICK);
        let mut edits = Vec::new();
        for _ in 0..EDIT_CHUNKS_PER_TICK {
            match network.take_edit() {
                Some(edit) => edits.push(edit),
                None => break,
            }
        }
        if network.dirty && now.saturating_sub(state.saved) >= SAVE_EVERY {
            save(state);
            state.network.dirty = false;
            state.saved = now;
        }
        edits
    });
    for (_, edit) in edits {
        let _ = host.edit_blocks(OVERWORLD, &edit);
    }
}

/// Vazão do rio mais forte perto de `pos` (0 fora dos rios), para a turbina.
pub(crate) fn flow_near(dimension: &str, pos: [i32; 3], reach: i32) -> i32 {
    if dimension != OVERWORLD {
        return 0;
    }
    with_state(|state| state.network.flow_near(pos[0], pos[2], reach))
}

#[cfg(test)]
mod tests;
