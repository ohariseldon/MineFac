//! Copiadora e marco de obra: copiar e colar fábricas, como o Blueprint Designer do Satisfactory.
//!
//! **Copiadora** (`DESIGNER`): o volume acima dela é a prancheta, um quadrado de `SIZES[n]`
//! blocos centrado nela e até `HEIGHT` de altura, desenhado no mundo por uma moldura.
//! - Papel na mão + clique: copia o que está na prancheta (blocos, orientação e ajustes
//!   de filtro, prioridade e sentido; nunca o conteúdo de inventários).
//! - Chave + agachado: muda o tamanho. Chave: gira, como as outras máquinas.
//! - Mão vazia: resumo no chat.
//!
//! **Marco de obra** (`MARKER`): fica onde a cópia vai nascer, no lugar equivalente ao da
//! copiadora, e usa a copiadora mais próxima com projeto (até `LINK` blocos). A diferença de
//! orientação entre o marco e a copiadora gira a obra.
//! - Mão vazia: orçamento no chat (materiais, tamanho e obstáculos).
//! - Chave + agachado: constrói, pagando com o inventário de quem clicou. Falta material ou
//!   espaço: nada é cobrado e o chat diz o que falta.
//!
//! Fábricas verticais: escadas de mão, andares e elevadores de carga são copiados como qualquer
//! outro bloco.
use super::*;
use horizon_client::mod_api::{NativeBlockEdit, NativeBlockRun, Text};
use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;

pub(crate) const DESIGNER: &str = "veio:blueprint_designer";
pub(crate) const MARKER: &str = "veio:build_marker";
pub(crate) const SIZES: [i32; 4] = [5, 9, 15, 21];
pub(crate) const HEIGHT: i32 = 24;
/// Teto de blocos de um projeto: mantém o NBT da copiadora e uma obra num tamanho seguro.
pub(crate) const MAX_BLOCKS: usize = 6000;
const LINK: i32 = 256;
const STALE: u64 = 400;
const PAPER: &str = "minecraft:paper";
/// Propriedades de estado copiadas. As conexões (cercas, vidros, curvas das esteiras) o jogo refaz.
const PROPS: [&str; 9] = ["facing", "axis", "half", "type", "shape", "hinge", "face", "part", "rotation"];
/// Ajustes das máquinas Veio copiados com a obra.
const CONFIG_INTS: [&str; 3] = ["RoutingPriority", "Down", "ManualDirection"];
const CONFIG_STRINGS: [&str; 1] = ["LogisticsFilter"];
const FACINGS: [&str; 4] = ["north", "east", "south", "west"];

pub(crate) fn owns(block: &str) -> bool { block == DESIGNER || block == MARKER }

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) struct State { pub block: String, pub props: Vec<(String, String)> }

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Setting { Int(i32), Text(String) }

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct Design {
    pub palette: Vec<State>,
    /// Deslocamento a partir da copiadora (y ≥ 1) e índice na paleta.
    pub cells: Vec<([i32; 3], u32)>,
    pub config: Vec<([i32; 3], String, Setting)>,
    /// Orientação da copiadora na cópia, 0 norte, 1 leste, 2 sul, 3 oeste.
    pub facing: u8,
    pub footprint: i32,
    pub height: i32,
    /// Blocos que a copiadora não copia (minério, fluidos, portais) e excedentes do teto.
    pub skipped: u32,
}

// ---------- O que se copia e quanto custa ----------

/// Blocos que não fazem parte de uma obra: terreno especial, fluidos, portais e blocos técnicos.
pub(crate) fn copyable(block: &str) -> bool {
    if let Some(name) = block.strip_prefix("veio:") { return !owns(block) && !name.is_empty(); }
    let Some(name) = block.strip_prefix("minecraft:") else { return false };
    const DENY: [&str; 22] = ["air", "cave_air", "void_air", "water", "lava", "fire", "soul_fire", "bedrock",
        "spawner", "trial_spawner", "vault", "barrier", "light", "structure_void", "structure_block", "jigsaw",
        "moving_piston", "piston_head", "bubble_column", "end_gateway", "budding_amethyst", "ancient_debris"];
    !(DENY.contains(&name) || name.ends_with("_ore") || name.contains("portal") || name.contains("command_block")
        || name.ends_with("_crop") || matches!(name, "wheat" | "carrots" | "potatoes" | "beetroots" | "nether_wart"
            | "melon_stem" | "pumpkin_stem" | "attached_melon_stem" | "attached_pumpkin_stem" | "cocoa" | "frosted_ice"))
}

/// Item que paga o bloco. `None`: a outra metade de um bloco duplo (porta, cama, planta alta).
pub(crate) fn cost_item(state: &State) -> Option<String> {
    let prop = |key: &str| state.props.iter().find(|(k, _)| k == key).map(|(_, v)| v.as_str());
    let block = state.block.as_str();
    if prop("half") == Some("upper") && (block.ends_with("_door") || matches!(block, "minecraft:tall_grass"
        | "minecraft:large_fern" | "minecraft:sunflower" | "minecraft:lilac" | "minecraft:rose_bush" | "minecraft:peony")) {
        return None;
    }
    if block.ends_with("_bed") && prop("part") == Some("head") { return None; }
    Some(match block {
        "minecraft:wall_torch" => "minecraft:torch".into(),
        "minecraft:soul_wall_torch" => "minecraft:soul_torch".into(),
        "minecraft:redstone_wall_torch" => "minecraft:redstone_torch".into(),
        "minecraft:redstone_wire" => "minecraft:redstone".into(),
        "minecraft:tripwire" => "minecraft:string".into(),
        "minecraft:farmland" | "minecraft:dirt_path" => "minecraft:dirt".into(),
        _ if block.ends_with("_wall_sign") => block.replace("_wall_sign", "_sign"),
        _ if block.ends_with("_wall_hanging_sign") => block.replace("_wall_hanging_sign", "_hanging_sign"),
        _ if block.ends_with("_wall_banner") => block.replace("_wall_banner", "_banner"),
        _ => block.to_owned(),
    })
}

pub(crate) fn costs(design: &Design) -> BTreeMap<String, u32> {
    let mut total = BTreeMap::new();
    for (_, index) in &design.cells {
        if let Some(item) = cost_item(&design.palette[*index as usize]) { *total.entry(item).or_insert(0) += 1; }
    }
    total
}

/// Espaço livre para a obra: ar e vegetação rasteira.
pub(crate) fn replaceable(block: &str) -> bool {
    matches!(block, "minecraft:air" | "minecraft:cave_air" | "minecraft:short_grass" | "minecraft:grass"
        | "minecraft:fern" | "minecraft:dead_bush" | "minecraft:snow" | "minecraft:tall_grass" | "minecraft:large_fern")
}

// ---------- Copiar ----------

pub(crate) fn facing_index(value: Option<&str>) -> u8 {
    FACINGS.iter().position(|f| Some(*f) == value).unwrap_or(0) as u8
}

pub(crate) fn capture(host: &mut dyn Host, pos: [i32; 3], size: usize) -> Design {
    let footprint = SIZES[size.min(SIZES.len() - 1)];
    let radius = footprint / 2;
    let mut design = Design { footprint, facing: facing_index(host.block_property(pos, "facing").as_deref()), ..Design::default() };
    let mut index: HashMap<State, u32> = HashMap::new();
    for dy in 1..=HEIGHT {
        for dz in -radius..=radius {
            for dx in -radius..=radius {
                let at = [pos[0] + dx, pos[1] + dy, pos[2] + dz];
                let block = host.block(at);
                if block.ends_with(":air") { continue; }
                if !copyable(&block) || design.cells.len() >= MAX_BLOCKS {
                    design.skipped += 1;
                    continue;
                }
                let props = PROPS.iter()
                    .filter_map(|key| host.block_property(at, key).map(|value| ((*key).to_owned(), value)))
                    .collect();
                let state = State { block: block.clone(), props };
                let next = index.len() as u32;
                let slot = *index.entry(state.clone()).or_insert_with(|| { design.palette.push(state); next });
                design.cells.push(([dx, dy, dz], slot));
                design.height = design.height.max(dy);
                if block.starts_with("veio:") {
                    let offset = [dx, dy, dz];
                    host.change_mod_block_entity(at, &mut |entity| {
                        for key in CONFIG_INTS {
                            if entity.nbt.contains_key(&key.into()) && number(entity, key) != 0 {
                                design.config.push((offset, key.into(), Setting::Int(number(entity, key))));
                            }
                        }
                        for key in CONFIG_STRINGS {
                            if let Some(Tag::String(value)) = entity.nbt.get(&key.into()) && !value.0.is_empty() {
                                design.config.push((offset, key.into(), Setting::Text(String::from_utf16_lossy(&value.0))));
                            }
                        }
                    });
                }
            }
        }
    }
    design
}

// ---------- Guardar no NBT da copiadora ----------

pub(crate) fn save(entity: &mut ModBlockEntity, design: &Design) {
    let palette = design.palette.iter().map(|state| {
        let props: Vec<String> = state.props.iter().map(|(k, v)| format!("{k}={v}")).collect();
        Tag::String(format!("{}|{}", state.block, props.join(",")).as_str().into())
    }).collect();
    entity.nbt.insert("DesignPalette".into(), Tag::List(palette));
    entity.nbt.insert("DesignCells".into(), Tag::IntArray(design.cells.iter()
        .flat_map(|([x, y, z], i)| [*x, *y, *z, *i as i32]).collect()));
    entity.nbt.insert("DesignConfig".into(), Tag::List(design.config.iter().map(|(offset, key, value)| {
        let value = match value { Setting::Int(v) => format!("i{v}"), Setting::Text(t) => format!("s{t}") };
        Tag::String(format!("{},{},{}|{key}|{value}", offset[0], offset[1], offset[2]).as_str().into())
    }).collect()));
    for (key, value) in [("DesignFacing", i32::from(design.facing)), ("DesignFootprint", design.footprint),
        ("DesignHeight", design.height), ("DesignSkipped", design.skipped as i32), ("DesignBlocks", design.cells.len() as i32)] {
        set_number(entity, key, value);
    }
}

fn text(tag: &Tag) -> Option<String> {
    match tag { Tag::String(value) => Some(String::from_utf16_lossy(&value.0)), _ => None }
}

pub(crate) fn load(entity: &ModBlockEntity) -> Option<Design> {
    let Some(Tag::IntArray(cells)) = entity.nbt.get(&"DesignCells".into()) else { return None };
    let Some(Tag::List(palette)) = entity.nbt.get(&"DesignPalette".into()) else { return None };
    let palette: Vec<State> = palette.iter().filter_map(text).map(|line| {
        let (block, props) = line.split_once('|').unwrap_or((&line, ""));
        State { block: block.to_owned(), props: props.split(',').filter_map(|pair| pair.split_once('='))
            .map(|(k, v)| (k.to_owned(), v.to_owned())).collect() }
    }).collect();
    let cells: Vec<([i32; 3], u32)> = cells.chunks_exact(4)
        .filter(|c| (c[3] as usize) < palette.len()).map(|c| ([c[0], c[1], c[2]], c[3] as u32)).collect();
    let config = match entity.nbt.get(&"DesignConfig".into()) {
        Some(Tag::List(list)) => list.iter().filter_map(text).filter_map(|line| {
            let mut parts = line.splitn(3, '|');
            let offset: Vec<i32> = parts.next()?.split(',').filter_map(|v| v.parse().ok()).collect();
            let key = parts.next()?.to_owned();
            let raw = parts.next()?;
            let value = if let Some(v) = raw.strip_prefix('i') { Setting::Int(v.parse().ok()?) } else { Setting::Text(raw.get(1..)?.to_owned()) };
            (offset.len() == 3).then(|| ([offset[0], offset[1], offset[2]], key, value))
        }).collect(),
        _ => Vec::new(),
    };
    Some(Design { palette, cells, config, facing: number(entity, "DesignFacing").clamp(0, 3) as u8,
        footprint: number(entity, "DesignFootprint"), height: number(entity, "DesignHeight"),
        skipped: number(entity, "DesignSkipped") as u32 })
}

/// O projeto não viaja para os clientes: só o necessário para desenhar a moldura.
pub(crate) fn client_tag(entity: &ModBlockEntity) -> Compound {
    let mut nbt = entity.nbt.clone();
    for key in ["DesignPalette", "DesignCells", "DesignConfig"] { nbt.remove(&key.into()); }
    nbt
}

// ---------- Colar ----------

/// Gira um deslocamento `turns` quartos de volta no sentido horário (norte → leste).
pub(crate) fn rotate([x, y, z]: [i32; 3], turns: u8) -> [i32; 3] {
    match turns % 4 { 0 => [x, y, z], 1 => [-z, y, x], 2 => [-x, y, -z], _ => [z, y, -x] }
}

pub(crate) fn rotate_prop(key: &str, value: &str, turns: u8) -> String {
    match key {
        "facing" if FACINGS.contains(&value) => FACINGS[(facing_index(Some(value)) as usize + turns as usize) % 4].into(),
        "axis" if turns % 2 == 1 && value != "y" => if value == "x" { "z".into() } else { "x".into() },
        "rotation" => value.parse::<u32>().map_or(value.into(), |r| ((r + 4 * u32::from(turns)) % 16).to_string()),
        _ => value.into(),
    }
}

#[derive(Debug, PartialEq)]
pub(crate) enum Refusal { NoDesign, Blocked([i32; 3]), Missing(Vec<(String, u32)>) }

/// Primeiro obstáculo da obra, se houver.
pub(crate) fn obstacle(host: &dyn Host, origin: [i32; 3], design: &Design, turns: u8) -> Option<[i32; 3]> {
    design.cells.iter().map(|(offset, _)| add3(origin, rotate(*offset, turns)))
        .find(|at| !replaceable(&host.block(*at)))
}

fn add3(a: [i32; 3], b: [i32; 3]) -> [i32; 3] { [a[0] + b[0], a[1] + b[1], a[2] + b[2]] }

/// Cobra do inventário de quem clicou e constrói. Nada muda se faltar material ou espaço.
pub(crate) fn build(host: &mut dyn Host, origin: [i32; 3], design: &Design, turns: u8) -> Result<usize, Refusal> {
    if design.cells.is_empty() { return Err(Refusal::NoDesign); }
    if let Some(at) = obstacle(host, origin, design, turns) { return Err(Refusal::Blocked(at)); }
    let mut taken = Vec::new();
    let mut missing = Vec::new();
    for (item, need) in costs(design) {
        let got = host.take_items(&item, need);
        if got < need { missing.push((item.clone(), need - got)); }
        taken.push((item, got));
    }
    if !missing.is_empty() {
        for (item, got) in taken { if got > 0 { host.give_items(&item, got); } }
        return Err(Refusal::Missing(missing));
    }
    let mut names: Vec<String> = Vec::new();
    let mut ids: HashMap<&str, u32> = HashMap::new();
    let mut runs = Vec::with_capacity(design.cells.len());
    for (offset, index) in &design.cells {
        let block = design.palette[*index as usize].block.as_str();
        let id = *ids.entry(block).or_insert_with(|| { names.push(block.to_owned()); names.len() as u32 - 1 });
        let [x, y, z] = add3(origin, rotate(*offset, turns));
        runs.push(NativeBlockRun { x, y, z, height: 1, surface: None, block: id, only: None });
    }
    if host.edit_blocks(host.dimension(), &NativeBlockEdit { palette: names, filters: Vec::new(), runs }).is_err() {
        for (item, got) in taken { host.give_items(&item, got); }
        return Err(Refusal::NoDesign);
    }
    for (offset, index) in &design.cells {
        let at = add3(origin, rotate(*offset, turns));
        for (key, value) in &design.palette[*index as usize].props {
            host.set_block_property(at, key, &rotate_prop(key, value, turns));
        }
    }
    let dimension = host.dimension();
    for (offset, key, value) in &design.config {
        let at = add3(origin, rotate(*offset, turns));
        if !apply(host, at, key, value) { queue(dimension, at, key, value.clone()); }
    }
    Ok(design.cells.len())
}

fn apply(host: &mut dyn Host, at: [i32; 3], key: &str, value: &Setting) -> bool {
    host.change_mod_block_entity(at, &mut |entity| match value {
        Setting::Int(v) => set_number(entity, key, *v),
        Setting::Text(t) => { entity.nbt.insert(key.into(), Tag::String(t.as_str().into())); }
    })
}

// ---------- Registro em memória ----------

type Key = (&'static str, [i32; 3]);

#[derive(Default)]
struct Registry {
    designs: HashMap<Key, (Arc<Design>, u64)>,
    /// Ajustes que esperam a máquina recém-colocada ganhar sua entidade.
    pending: Vec<(&'static str, [i32; 3], String, Setting, u8)>,
}

#[cfg(not(test))]
fn with_registry<T>(run: impl FnOnce(&mut Registry) -> T) -> T {
    static STATE: std::sync::LazyLock<std::sync::Mutex<Registry>> = std::sync::LazyLock::new(Default::default);
    run(&mut STATE.lock().unwrap_or_else(|poisoned| poisoned.into_inner()))
}

#[cfg(test)]
fn with_registry<T>(run: impl FnOnce(&mut Registry) -> T) -> T {
    thread_local! { static STATE: std::cell::RefCell<Registry> = Default::default(); }
    STATE.with(|state| run(&mut state.borrow_mut()))
}

fn queue(dimension: &'static str, at: [i32; 3], key: &str, value: Setting) {
    with_registry(|registry| registry.pending.push((dimension, at, key.to_owned(), value, 0)));
}

fn register(dimension: &'static str, pos: [i32; 3], design: Arc<Design>, now: u64) {
    with_registry(|registry| {
        registry.designs.retain(|_, (_, seen)| now.saturating_sub(*seen) <= STALE);
        registry.designs.insert((dimension, pos), (design, now));
    });
}

/// Copiadora com projeto mais próxima do marco.
pub(crate) fn nearest(dimension: &str, at: [i32; 3], now: u64) -> Option<([i32; 3], Arc<Design>)> {
    with_registry(|registry| registry.designs.iter()
        .filter(|((dim, pos), (_, seen))| *dim == dimension && now.saturating_sub(*seen) <= STALE
            && (0..3).all(|axis| (pos[axis] - at[axis]).abs() <= LINK))
        .min_by_key(|((_, pos), _)| (0..3).map(|axis| (pos[axis] - at[axis]).abs()).sum::<i32>())
        .map(|((_, pos), (design, _))| (*pos, design.clone())))
}

// ---------- Tick, cliques e painel ----------

pub(crate) fn tick(host: &mut dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity) {
    let now = defense::clock();
    let dimension = host.dimension();
    if entity.id == DESIGNER {
        let known = with_registry(|registry| registry.designs.get_mut(&(dimension, pos)).map(|(_, seen)| *seen = now).is_some());
        if !known && let Some(design) = load(entity) { register(dimension, pos, Arc::new(design), now); }
        set_number(entity, "Status", if entity.nbt.contains_key(&"DesignCells".into()) { 1 } else { 0 });
        return;
    }
    // Marco: segue a copiadora mais próxima e termina ajustes pendentes.
    if (now + ((pos[0] ^ pos[2]) & 15) as u64).is_multiple_of(20) || number(entity, "Linked") == 0 {
        match nearest(dimension, pos, now) {
            Some((source, design)) => {
                let turns = turns(host, pos, &design);
                for (key, value) in [("Linked", 1), ("BoxSize", design.footprint), ("BoxHeight", design.height.max(1)),
                    ("Turns", i32::from(turns)), ("DesignBlocks", design.cells.len() as i32)] {
                    set_number(entity, key, value);
                }
                entity.nbt.insert("Source".into(), Tag::IntArray(source.to_vec()));
            }
            None => set_number(entity, "Linked", 0),
        }
    }
    let ready = with_registry(|registry| {
        let (ready, waiting): (Vec<_>, Vec<_>) = registry.pending.drain(..).partition(|(dim, at, ..)| *dim == dimension
            && (0..3).all(|axis| (at[axis] - pos[axis]).abs() <= SIZES[SIZES.len() - 1] && at[1] >= pos[1]));
        registry.pending = waiting;
        ready
    });
    for (dim, at, key, value, tries) in ready {
        if !apply(host, at, &key, &value) && tries < 40 { with_registry(|r| r.pending.push((dim, at, key, value, tries + 1))); }
    }
    set_number(entity, "Status", if number(entity, "Linked") == 1 { 1 } else { 2 });
}

fn turns(host: &dyn Host, pos: [i32; 3], design: &Design) -> u8 {
    (facing_index(host.block_property(pos, "facing").as_deref()) + 4 - design.facing) % 4
}

fn tell_nearest(host: &mut dyn Host, pos: [i32; 3], line: Text) {
    let at = center(pos);
    let dimension = host.dimension();
    if let Some(player) = host.server_players().into_iter()
        .filter(|player| player.dimension == dimension)
        .min_by(|a, b| distance2(a.position, at).total_cmp(&distance2(b.position, at)))
    {
        host.tell(player.uuid, line);
    }
}

fn distance2(a: [f64; 3], b: [f64; 3]) -> f64 { (0..3).map(|axis| (a[axis] - b[axis]).powi(2)).sum() }

/// Nome traduzível de um item: blocos com `block.*`, o resto com `item.*`.
fn item_name(item: &str) -> String {
    let (namespace, name) = item.split_once(':').unwrap_or(("minecraft", item));
    let as_item = matches!(name, "redstone" | "string");
    let key = format!("{}.{namespace}.{name}", if as_item { "item" } else { "block" });
    Text::tr(&key, &[]).plain()
}

/// Cliques tratados pela copiadora e pelo marco. `None`: segue o tratamento comum (girar).
pub(crate) fn right_click(host: &mut dyn Host, pos: [i32; 3], block: &str, held: Option<&str>) -> Option<Flow> {
    let sneaking = host.sneaking();
    let now = defense::clock();
    let dimension = host.dimension();
    match (block, held) {
        (DESIGNER, Some(PAPER)) => {
            let mut size = 0;
            host.change_mod_block_entity(pos, &mut |entity| size = number(entity, "Size") as usize);
            let design = capture(host, pos, size);
            host.change_mod_block_entity(pos, &mut |entity| save(entity, &design));
            let blocks = design.cells.len();
            let materials = costs(&design).len();
            register(dimension, pos, Arc::new(design.clone()), now);
            host.play_sound("minecraft:item.book.page_turn", center(pos), 4, 0.8, 1.0);
            tell_nearest(host, pos, Text::tr("veio.blueprint.copied", &[&blocks, &materials, &design.footprint, &design.height]));
            if design.skipped > 0 { tell_nearest(host, pos, Text::tr("veio.blueprint.skipped", &[&design.skipped])); }
            Some(Flow::Cancel)
        }
        (DESIGNER, Some(WRENCH)) if sneaking => {
            let mut size = 0;
            host.change_mod_block_entity(pos, &mut |entity| {
                size = (number(entity, "Size") + 1) % SIZES.len() as i32;
                set_number(entity, "Size", size);
            });
            tell_nearest(host, pos, Text::tr("veio.blueprint.size", &[&SIZES[size as usize], &HEIGHT]));
            Some(Flow::Cancel)
        }
        (DESIGNER, None) => {
            let mut summary = None;
            host.change_mod_block_entity(pos, &mut |entity| {
                summary = Some((SIZES[(number(entity, "Size") as usize).min(SIZES.len() - 1)], number(entity, "DesignBlocks")));
            });
            let (size, blocks) = summary?;
            tell_nearest(host, pos, Text::tr("veio.blueprint.designer", &[&size, &HEIGHT, &blocks]));
            Some(Flow::Cancel)
        }
        (MARKER, None) => {
            let Some((source, design)) = nearest(dimension, pos, now) else {
                tell_nearest(host, pos, Text::tr("veio.blueprint.no_design", &[&LINK]));
                return Some(Flow::Cancel);
            };
            let turns = turns(host, pos, &design);
            tell_nearest(host, pos, Text::tr("veio.blueprint.budget", &[&source.map(|v| v.to_string()).join(" "),
                &design.cells.len(), &design.footprint, &design.height, &(u32::from(turns) * 90)]));
            let mut list: Vec<(String, u32)> = costs(&design).into_iter().collect();
            list.sort_by(|a, b| b.1.cmp(&a.1));
            for (item, count) in list.iter().take(10) {
                tell_nearest(host, pos, Text::tr("veio.blueprint.cost", &[count, &item_name(item)]));
            }
            if list.len() > 10 { tell_nearest(host, pos, Text::tr("veio.blueprint.more", &[&(list.len() - 10)])); }
            let line = match obstacle(host, pos, &design, turns) {
                Some(at) => Text::tr("veio.blueprint.blocked", &[&at.map(|v| v.to_string()).join(" ")]),
                None => Text::tr("veio.blueprint.clear", &[]),
            };
            tell_nearest(host, pos, line);
            Some(Flow::Cancel)
        }
        (MARKER, Some(WRENCH)) if sneaking => {
            let Some((_, design)) = nearest(dimension, pos, now) else {
                tell_nearest(host, pos, Text::tr("veio.blueprint.no_design", &[&LINK]));
                return Some(Flow::Cancel);
            };
            let turns = turns(host, pos, &design);
            let line = match build(host, pos, &design, turns) {
                Ok(blocks) => {
                    host.play_sound("minecraft:block.anvil.use", center(pos), 4, 0.8, 0.8);
                    Text::tr("veio.blueprint.built", &[&blocks])
                }
                Err(Refusal::Blocked(at)) => Text::tr("veio.blueprint.blocked", &[&at.map(|v| v.to_string()).join(" ")]),
                Err(Refusal::Missing(list)) => {
                    let parts: Vec<String> = list.iter().take(6).map(|(item, n)| format!("{n} {}", item_name(item))).collect();
                    Text::tr("veio.blueprint.missing", &[&parts.join(", ")])
                }
                Err(Refusal::NoDesign) => Text::tr("veio.blueprint.no_design", &[&LINK]),
            };
            tell_nearest(host, pos, line);
            Some(Flow::Cancel)
        }
        _ => None,
    }
}

// ---------- Moldura no mundo ----------

const EDGE: f32 = 0.6;
const SIDE: Option<horizon_client::mod_api::CubeFace<'static>> =
    Some(horizon_client::mod_api::CubeFace { sprite: "minecraft:block/white_concrete", uv: None });
const FRAME: [Option<horizon_client::mod_api::CubeFace<'static>>; 6] = [SIDE, SIDE, SIDE, SIDE, SIDE, SIDE];

/// Caixa da moldura em blocos, relativa ao bloco: (mín, máx).
pub(crate) fn frame(entity: &ModBlockEntity) -> Option<([f32; 3], [f32; 3])> {
    let (size, height) = if entity.id == DESIGNER {
        (SIZES[(number(entity, "Size") as usize).min(SIZES.len() - 1)], HEIGHT)
    } else if number(entity, "Linked") == 1 {
        (number(entity, "BoxSize").max(1), number(entity, "BoxHeight").max(1))
    } else { return None };
    let radius = (size / 2) as f32;
    Some(([-radius, 1.0, -radius], [radius + 1.0, 1.0 + height as f32, radius + 1.0]))
}

pub(crate) fn bounds(entity: &ModBlockEntity, bounds: &mut [f32; 6]) {
    if let Some((from, to)) = frame(entity) {
        for axis in 0..3 {
            bounds[axis] = bounds[axis].min(from[axis]);
            bounds[axis + 3] = bounds[axis + 3].max(to[axis]);
        }
    }
}

/// Doze arestas finas: ciano na copiadora, verde no marco.
pub(crate) fn draw(host: &dyn Host, event: &mut horizon_client::mod_api::BlockEntitySubmitEvent<'_>) -> bool {
    if !owns(event.block) { return false; }
    let Some((from, to)) = frame(event.entity) else { return true };
    let [a, b] = [from.map(|v| v * 16.0), to.map(|v| v * 16.0)];
    let tint = if event.block == DESIGNER { 0x40d0e0 } else { 0x60e060 };
    for axis in 0..3 {
        let (u, v) = ((axis + 1) % 3, (axis + 2) % 3);
        for (cu, cv) in [(a[u], a[v]), (b[u], a[v]), (a[u], b[v]), (b[u], b[v])] {
            let mut lo = [0.0; 3];
            let mut hi = [0.0; 3];
            lo[axis] = a[axis];
            hi[axis] = b[axis];
            lo[u] = cu - EDGE / 2.0;
            hi[u] = cu + EDGE / 2.0;
            lo[v] = cv - EDGE / 2.0;
            hi[v] = cv + EDGE / 2.0;
            host.draw_cube(event.draw, lo, hi, FRAME, tint);
        }
    }
    true
}

#[cfg(test)]
mod tests;
