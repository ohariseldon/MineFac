// Builds a certificate against the real generator before a new world opens.
use horizon_world::{registry::BlockRegistry, worldgen::generator::{GeneratorData, GeneratorWorker, OverworldGenerator}};
use horizon_core::position::ChunkPos;
use horizon_world::veio_map::{Dimension, PORTAL_MARGIN};
use serde_json::{Value, json};
use std::{collections::{BTreeMap, BTreeSet}, path::Path};

fn error(e: impl std::fmt::Display) -> String { e.to_string() }

pub fn derive(seed: i64, attempt: u64) -> i64 {
    if attempt == 0 { return seed; }
    let mut v = (seed as u64).wrapping_add(attempt.wrapping_mul(0x9e3779b97f4a7c15));
    v = (v ^ (v >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    v = (v ^ (v >> 27)).wrapping_mul(0x94d049bb133111eb);
    (v ^ (v >> 31)) as i64
}

fn witness(worker: &mut GeneratorWorker, name: &str, at: [i32; 3], cave: bool) -> bool {
    // The whole 48x48 region, rather than one rare biome cell, must be present.
    let mut samples = Vec::new();
    for dz in [-24, -12, 0, 12, 24] {
        for dx in [-24, -12, 0, 12, 24] {
            samples.push([(at[0] + dx) >> 2, at[1] >> 2, (at[2] + dz) >> 2]);
        }
    }
    if worker.noise_biomes(&samples).iter().any(|b| b.as_deref() != Some(name)) { return false; }
    if !cave {
        let Some(surface) = worker.surface_sample(at[0], at[2]) else { return false; };
        let quart = [at[0] >> 2, (surface.height - 1) >> 2, at[2] >> 2];
        if worker.noise_biomes(&[quart])[0].as_deref() != Some(name) { return false; }
        // An ocean witness needs real water, not only an ocean biome id.
        if name.contains("ocean") && surface.water.is_none() { return false; }
    }
    true
}

fn scan(worker: &mut GeneratorWorker, dimension: Dimension, required: &[String]) -> Result<BTreeMap<String, Vec<[i32; 3]>>, String> {
    let bounds = dimension.bounds();
    let caves: BTreeSet<&str> = required.iter().filter(|b| b.ends_with("_caves") || b.ends_with(":deep_dark")).map(String::as_str).collect();
    let ys: &[i32] = match dimension { Dimension::Overworld => &[64, 128, 192, 32, 0, -32, -48], Dimension::Nether => &[32, 64, 96], Dimension::End => &[64] };
    let mut found: BTreeMap<String, Vec<[i32; 3]>> = BTreeMap::new();
    let limit = bounds.max as i32 - 256;
    // Two interleaved grids; neither creates or loads gameplay chunks.
    for stride in [128, 64] {
        for z in (-limit..limit).step_by(stride) {
            let mut positions = Vec::new();
            for x in (-limit..limit).step_by(stride) {
                for &y in ys { positions.push([x, y, z]); }
            }
            let quarts: Vec<_> = positions.iter().map(|p| [p[0] >> 2, p[1] >> 2, p[2] >> 2]).collect();
            let answers = worker.noise_biomes(&quarts);
            for (at, answer) in positions.into_iter().zip(answers) {
                let Some(name) = answer else { continue; };
                if !required.contains(&name) || found.get(&name).is_some_and(|v| v.len() >= 8) { continue; }
                let cave = caves.contains(name.as_str());
                if cave && at[1] > 40 || !cave && at[1] < 32 { continue; }
                if witness(worker, &name, at, cave) {
                    let sites = found.entry(name).or_default();
                    if sites.iter().all(|p| (p[0]-at[0]).abs().max((p[2]-at[2]).abs()) >= 256) { sites.push(at); }
                }
            }
            if required.iter().all(|b| found.get(b).is_some_and(|v| !v.is_empty())) { break; }
        }
        if required.iter().all(|b| found.get(b).is_some_and(|v| !v.is_empty())) { break; }
    }
    let missing: Vec<_> = required.iter().filter(|b| !found.contains_key(*b)).cloned().collect();
    if !missing.is_empty() { return Err(format!("{}: missing usable biomes: {}", dimension.id(), missing.join(", "))); }
    Ok(found)
}

fn reserve(worker: &mut GeneratorWorker, dimension: Dimension, names: &BTreeMap<String, Vec<String>>, biomes: &BTreeMap<String, Vec<[i32; 3]>>) -> Result<Vec<Value>, String> {
    let mut reserved: Vec<Value> = Vec::new();
    for (name, allowed) in names {
        let mut result = None;
        'search: for biome in allowed {
            for &origin in biomes.get(biome).into_iter().flatten() {
                for radius in [0_i32, 4, 8, 12, 16, 24] {
                    for (dx, dz) in [(radius, 0), (-radius, 0), (0, radius), (0, -radius), (radius, radius), (-radius, -radius)] {
                        let pos = ChunkPos::new((origin[0] >> 4) + dx, (origin[2] >> 4) + dz);
                        let x = pos.x * 16 + 8;
                        let z = pos.z * 16 + 8;
                        if !dimension.bounds().contains(x as f64, z as f64, 128.0) { continue; }
                        if reserved.iter().any(|r| {
                            let a = r["chunk"][0].as_i64().unwrap() as i32;
                            let b = r["chunk"][1].as_i64().unwrap() as i32;
                            (a-pos.x).abs().max((b-pos.z).abs()) < 12
                        }) { continue; }
                        let Some(start) = worker.structure_start(name, pos) else { continue; };
                        let bounds = start.bounding_box;
                        if !dimension.bounds().contains_box(bounds.min, bounds.max, PORTAL_MARGIN) { continue; }
                        // An End city must include a ship, so elytra remain obtainable.
                        let pieces = format!("{:?}", start.save());
                        if name == "minecraft:end_city" && !pieces.contains("ship") { continue; }
                        if reserved.iter().any(|r| [0,2].iter().all(|&a| bounds.min[a] <= r["max"][a].as_i64().unwrap() as i32 + 16 && bounds.max[a] + 16 >= r["min"][a].as_i64().unwrap() as i32)) { continue; }
                        result = Some(json!({"name":name,"chunk":[pos.x,pos.z],"min":bounds.min,"max":bounds.max}));
                        break 'search;
                    }
                }
            }
        }
        reserved.push(result.ok_or_else(|| format!("{}: no safe reservation for {name}", dimension.id()))?);
    }
    Ok(reserved)
}

pub fn build(resources: &Path, registry: &'static BlockRegistry, requested_seed: i64) -> Result<Value, String> {
    let data = GeneratorData::load(&resources.join("worldgen"), registry).map_err(error)?;
    let mut catalogs = Vec::new();
    for dimension in [Dimension::Overworld, Dimension::Nether, Dimension::End] {
        let biomes = horizon_world::worldgen::generator::veio_catalog::biomes(&data, "minecraft:normal", dimension.id())?;
        let structures = horizon_world::worldgen::generator::veio_catalog::structures(&data, &biomes)?;
        catalogs.push((dimension, biomes, structures));
    }
    let mut last = String::new();
    for attempt in 0..64 {
        let seed = derive(requested_seed, attempt);
        eprintln!("Veio: checking finite world candidate {attempt} (seed {seed})");
        let mut dimensions = serde_json::Map::new();
        let outcome = (|| {
            for (dimension, required, structures) in &catalogs {
                let generator = OverworldGenerator::for_world(data.clone(), seed, "minecraft:normal", dimension.id(), true).map_err(error)?;
                let mut worker = generator.worker();
                let biomes = scan(&mut worker, *dimension, required)?;
                let reserved = reserve(&mut worker, *dimension, structures, &biomes)?;
                dimensions.insert(dimension.id().to_owned(), json!({"biomes":biomes,"structures":reserved}));
            }
            Ok::<(),String>(())
        })();
        match outcome {
            Ok(()) => return Ok(json!({"format":horizon_world::veio_map::FORMAT,"side":15000,"requested_seed":requested_seed,"seed":seed,"data_version":horizon_world::DATA_VERSION,"dimensions":dimensions})),
            Err(e) => { eprintln!("Veio: candidate rejected: {e}"); last = e; }
        }
    }
    Err(format!("finite world validation failed after 64 candidates: {last}"))
}
