//! Fábrica média simulada: o mesmo `block_entity_tick` do jogo, num mundo de mentira.
//! Mede o Veio (µs por tick, operações no Host, alocações) e imprime um resumo do estado final:
//! se uma otimização mudar o resultado do jogo, o resumo muda.
//! `cargo test --offline --release -p veio-check bench_factory -- --ignored --nocapture`
use super::*;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

const DIMENSION: &str = "veio:bench_factory";
const LINES: i32 = 16;
const TICKS: i64 = 4000;

#[derive(Default, Clone, Copy)]
struct Ops { block: usize, property: usize, set_property: usize, change: usize, insert: usize, sync: usize, signal: usize }

#[derive(Default)]
struct World {
    time: i64,
    blocks: HashMap<[i32; 3], String>,
    properties: HashMap<([i32; 3], &'static str), String>,
    index: HashMap<[i32; 3], usize>,
    order: Vec<[i32; 3]>,
    nodes: Vec<ModBlockEntity>,
    ops: std::cell::Cell<Ops>,
    /// Sincronias por tipo de bloco, para achar quem inunda a rede.
    syncs: Vec<usize>,
    ticking: usize,
}

impl World {
    fn count(&self, change: impl FnOnce(&mut Ops)) {
        let mut ops = self.ops.get();
        change(&mut ops);
        self.ops.set(ops);
    }
    fn place(&mut self, pos: [i32; 3], id: &str, facing: Option<&str>) -> &mut ModBlockEntity {
        self.blocks.insert(pos, id.to_owned());
        if let Some(facing) = facing { self.properties.insert((pos, "facing"), facing.to_owned()); }
        let mut entity = ModBlockEntity::default();
        entity.id = id.to_owned();
        entity.items = vec![None; layout(id).map_or(0, |(slots, _, _)| slots)];
        self.index.insert(pos, self.nodes.len());
        self.order.push(pos);
        self.nodes.push(entity);
        self.syncs.push(0);
        self.nodes.last_mut().unwrap()
    }
    fn tick(&mut self) {
        self.time += 1;
        defense::advance_clock();
        budget::begin_tick(defense::clock());
        power::maintenance(defense::clock());
        for i in 0..self.order.len() {
            let pos = self.order[i];
            self.ticking = i;
            let mut entity = std::mem::take(&mut self.nodes[i]);
            block_entity_tick(self, pos, &mut entity);
            self.nodes[i] = entity;
        }
    }
    /// Resumo do estado: blocos, propriedades, NBT, itens e painel de cada máquina.
    fn digest(&self) -> u64 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        let mut order = self.order.clone();
        order.sort_unstable();
        for pos in order {
            let entity = &self.nodes[self.index[&pos]];
            pos.hash(&mut hasher);
            format!("{:?}{:?}{:?}{:?}", entity.id, entity.nbt, entity.items, entity.data).hash(&mut hasher);
        }
        let mut properties: Vec<_> = self.properties.iter().collect();
        properties.sort();
        format!("{properties:?}").hash(&mut hasher);
        hasher.finish()
    }
}

impl Host for World {
    fn dimension(&self) -> &'static str { DIMENSION }
    fn level(&self, dimension: &str) -> Option<horizon_client::mod_api::LevelRef> {
        (dimension == DIMENSION).then(|| horizon_client::mod_api::LevelRef::new(DIMENSION))
    }
    fn level_game_time(&self, _: horizon_client::mod_api::LevelRef) -> Option<i64> { Some(self.time) }
    fn day_time(&self) -> i64 { 6000 }
    fn light(&self, _: [i32; 3]) -> (u8, u8) { (0, 15) }
    fn weather(&self) -> (f32, f32) { (0.0, 0.0) }
    fn neighbor_signal(&mut self, _: &str, _: [i32; 3]) -> i32 { self.count(|o| o.signal += 1); 0 }
    fn block(&self, pos: [i32; 3]) -> String {
        self.count(|o| o.block += 1);
        self.blocks.get(&pos).cloned().unwrap_or_else(|| "minecraft:air".into())
    }
    fn block_property(&self, pos: [i32; 3], key: &str) -> Option<String> {
        self.count(|o| o.property += 1);
        let key: &'static str = match key { "facing" => "facing", "path" => "path", _ => return None };
        self.properties.get(&(pos, key)).cloned()
    }
    fn set_block_property(&mut self, pos: [i32; 3], key: &str, value: &str) -> bool {
        self.count(|o| o.set_property += 1);
        let key: &'static str = match key { "facing" => "facing", "path" => "path", _ => return false };
        self.properties.insert((pos, key), value.to_owned());
        true
    }
    fn change_mod_block_entity(&mut self, pos: [i32; 3], change: &mut dyn FnMut(&mut ModBlockEntity)) -> bool {
        self.count(|o| o.change += 1);
        let Some(&i) = self.index.get(&pos) else { return false };
        if self.nodes[i].id.is_empty() { return false; }
        change(&mut self.nodes[i]);
        true
    }
    fn insert_item(&mut self, pos: [i32; 3], _: Direction, stack: ItemStack) -> Option<ItemStack> {
        self.count(|o| o.insert += 1);
        let Some(&i) = self.index.get(&pos) else { return Some(stack) };
        let entity = &mut self.nodes[i];
        let Some((_, inputs, _)) = layout(&entity.id) else { return Some(stack) };
        for slot in inputs {
            if accepts(&entity.id, slot, &stack) && can_add(&entity.items[slot], stack.item.as_ref(), stack.count) {
                add(&mut entity.items[slot], stack.item.as_ref(), stack.count);
                return None;
            }
        }
        Some(stack)
    }
    fn block_entity_sync(&mut self, _: [i32; 3]) {
        self.count(|o| o.sync += 1);
        if let Some(count) = self.syncs.get_mut(self.ticking) { *count += 1; }
    }
    fn send_particles(&mut self, _: &horizon_client::mod_api::ParticleOptions, _: Option<horizon_client::mod_api::BlockStateId>,
        _: [f64; 3], _: [f64; 3], _: f64, _: i32) -> bool { true }
    fn play_sound(&mut self, _: &str, _: [f64; 3], _: usize, _: f32, _: f32) -> bool { true }
}

/// 16 linhas: extratora (comum ou Mk.III) → 8 esteiras → prensa → 8 esteiras → montadora (que enche e para).
/// Cada linha tem postes de madeira e um gerador a carvão; ao lado, 64 cabos e 16 painéis solares.
fn build() -> World {
    let mut world = World::default();
    for line in 0..LINES {
        let z = line * 6;
        for dx in -1..=1 { for dz in -1..=1 {
            world.blocks.insert([dx, -1, z + dz], "minecraft:copper_ore".into());
        } }
        // Metade das linhas com a extratora Mk.III: produz mais que a esteira leva, a linha enche.
        world.place([0, 0, z], if line % 2 == 0 { DRILL } else { production::DRILL_MK3 }, Some("west"));
        for x in 1..=8 { world.place([x, 0, z], BELT, Some("west")); }
        world.place([9, 0, z], PRESS, Some("west"));
        for x in 10..=17 { world.place([x, 0, z], if line % 2 == 0 { BELT } else { FAST_BELT }, Some("west")); }
        world.place([18, 0, z], ASSEMBLER, Some("west"));
        for x in [0, 5, 10, 15, 19] { world.place([x, 0, z + 2], POLE, None); }
        world.place([-1, 0, z + 2], GENERATOR, Some("west")).items[0] = Some(ItemStack::new("minecraft:coal", 64));
    }
    for x in 0..64 { world.place([x, 0, -10], CABLE, None); }
    for x in 0..16 { world.place([x, 0, -14], SOLAR, None); }
    world
}

#[test]
#[ignore = "medição explícita; rode sozinha com --release --nocapture"]
fn bench_factory() {
    let mut world = build();
    let entities = world.order.len();
    // Aquecimento: postes ligados, áreas varridas, curvas resolvidas, esteiras cheias.
    for _ in 0..600 { world.tick(); }
    world.ops.set(Ops::default());
    world.syncs.iter_mut().for_each(|count| *count = 0);
    COUNTS.with(|c| c.set(Counts::default()));
    ACTIVE.with(|c| c.set(true));
    let started = std::time::Instant::now();
    for _ in 0..TICKS { world.tick(); }
    let elapsed = started.elapsed();
    ACTIVE.with(|c| c.set(false));
    let counts = COUNTS.with(Cell::get);
    let ops = world.ops.get();
    let per = |n: usize| n as f64 / TICKS as f64;
    eprintln!("fábrica: {entities} blocos: {:.1} µs por tick do jogo (Veio + mundo simulado)",
        elapsed.as_secs_f64() * 1e6 / TICKS as f64);
    eprintln!("  por tick: {:.0} alocações, {:.0} block(), {:.0} block_property(), {:.1} set_block_property(), {:.0} change_mod_block_entity(), {:.1} insert_item(), {:.1} sync, {:.0} neighbor_signal()",
        per(counts.allocations), per(ops.block), per(ops.property), per(ops.set_property), per(ops.change), per(ops.insert), per(ops.sync), per(ops.signal));
    let mut kinds = std::collections::BTreeMap::<&str, usize>::new();
    for (entity, count) in world.nodes.iter().zip(&world.syncs) { *kinds.entry(entity.id.as_str()).or_default() += count; }
    eprintln!("  sync por tick: {}", kinds.iter().map(|(id, n)| format!("{id} {:.2}", per(*n))).collect::<Vec<_>>().join(", "));
    let produced: i32 = world.nodes.iter().filter(|n| n.id == ASSEMBLER).map(|n| n.items[1].as_ref().map_or(0, |s| s.count as i32)).sum();
    eprintln!("CHECK factory_digest={:016x} gears={produced}", world.digest());
}
