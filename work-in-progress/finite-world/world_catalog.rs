// Version-derived coverage queries for the finite-world planner.
use super::*;
use std::collections::BTreeMap;

/// Read the actual resolved biome source, including the version's quantized
/// preset. This deliberately does not maintain a second list of vanilla biomes.
pub fn biomes(data: &GeneratorData, preset: &str, dimension: &str) -> Result<Vec<String>, String> {
    let raw = data.world_presets.get(preset).ok_or("missing world preset")?;
    let preset: Value = serde_json::from_str(raw).map_err(|e| e.to_string())?;
    let stem = match data.dimensions.get(dimension) {
        Some(raw) => serde_json::from_str::<Value>(raw).map_err(|e| e.to_string())?,
        None => preset["dimensions"][dimension].clone(),
    };
    let source = data.source_catalog.decode(&stem["generator"]["biome_source"].to_string())
        .map_err(|e| e.to_string())?.resolve(&data.presets).map_err(|e| e.to_string())?;
    Ok(source.possible_biomes().into_iter().map(str::to_owned).collect())
}

/// Every structure whose biome predicate intersects this dimension. Values are
/// its allowed biomes, expanded by the engine's own tag resolver.
pub fn structures(data: &Arc<GeneratorData>, dimension_biomes: &[String]) -> Result<BTreeMap<String, Vec<String>>, String> {
    let possible: BTreeSet<BiomeId> = dimension_biomes.iter().filter_map(|b| data.biome_id(b)).collect();
    let state = super::super::generator_structures::StructureState::new(data, &possible, Arc::new(std::sync::OnceLock::new()))
        .map_err(|e| e.to_string())?.ok_or("structure resources unavailable")?;
    Ok(state.registry.structures.iter().filter_map(|s| {
        let allowed: Vec<String> = s.biomes.intersection(&possible).map(|id| data.biome_name(*id).to_owned()).collect();
        (!allowed.is_empty()).then(|| (s.name.clone(), allowed))
    }).collect())
}

/// Source blocks mentioned by the actual reachable feature graph. A block
/// provider can reference another provider, configured feature or placed
/// feature; walk those edges instead of keeping a frozen list of ores/plants.
pub fn sources(data: &GeneratorData, biomes: &[String]) -> Result<BTreeMap<String, Vec<String>>, String> {
    fn visit(value:&Value, feature_ref:bool,data:&GeneratorData, seen:&mut BTreeSet<String>, found:&mut BTreeSet<String>) -> Result<(),String> {
        match value {
            Value::Array(a) => for v in a {visit(v,feature_ref,data,seen,found)?;},
            Value::Object(o) => for (key,v) in o {visit(v,key=="feature"||key=="features"||(feature_ref && key=="values"),data,seen,found)?;},
            Value::String(s) => {
                if feature_ref && let Some(r)=&data.feature_resources {
                    if !seen.insert(format!("feature:{s}")){return Ok(());}
                    for map in [&r.placed,&r.features] {
                        if let Some(raw)=map.get(s) {visit(&serde_json::from_str::<Value>(raw).map_err(|e|e.to_string())?,false,data,seen,found)?;return Ok(());}
                    }
                    if let Some(raw)=r.feature_tags.get(s.trim_start_matches('#')) {visit(&serde_json::from_str::<Value>(raw).map_err(|e|e.to_string())?,true,data,seen,found)?;return Ok(());}
                    return Err(format!("unresolved finite-world feature {s}"));
                }
                // These are world infrastructure, not obtainable resources.
                if data.registry.default_state(s).is_some() && !matches!(s.as_str(),"minecraft:air"|"minecraft:cave_air"|"minecraft:void_air"|"minecraft:bedrock"|"minecraft:end_gateway"|"minecraft:fire"|"minecraft:soul_fire") {found.insert(s.clone());}
                if seen.insert(format!("provider:{s}")) && data.registry.default_state(s).is_none() && let Some(r)=&data.feature_resources && let Some(raw)=r.state_providers.get(s) {
                    visit(&serde_json::from_str::<Value>(raw).map_err(|e|e.to_string())?,false,data,seen,found)?;
                }
            },
            _=>{},
        }
        Ok(())
    }
    let mut result:BTreeMap<String,Vec<String>>=BTreeMap::new();
    for biome in biomes {
        let id=data.biome_id(biome).ok_or("unknown catalog biome")?;
        let raw:Value=serde_json::from_str(data.biome_json(id)).map_err(|e|e.to_string())?;
        let mut found=BTreeSet::new();
        visit(&raw["features"],true,data,&mut BTreeSet::new(),&mut found)?;
        for name in found {result.entry(name).or_default().push(biome.clone());}
    }
    Ok(result)
}

/// Bastion start layouts have distinct survival loot and mob sources. Derive
/// all four (or a pack's replacements) from the real start pool.
pub fn variants(data:&GeneratorData,name:&str)->Result<Vec<String>,String> {
    if name!="minecraft:bastion_remnant" {return Ok(vec![String::new()]);}
    let r=data.structure_resources.as_ref().ok_or("missing structures")?;
    let s:Value=serde_json::from_str(r.structures.get(name).ok_or("missing bastion")?).map_err(|e|e.to_string())?;
    let pool=s["start_pool"].as_str().ok_or("missing bastion start pool")?;
    let pool:Value=serde_json::from_str(r.pools.get(pool).ok_or("missing bastion variants")?).map_err(|e|e.to_string())?;
    let variants:Vec<_>=pool["elements"].as_array().ok_or("empty bastion variants")?.iter().filter_map(|e|e["element"]["location"].as_str().map(str::to_owned)).collect();
    if variants.is_empty(){return Err("missing bastion variant templates".into());}
    Ok(variants)
}

/// Stable digest of the resolved catalog, independent of Rust's HashMap seed.
/// A changed data pack or engine catalog must never silently change a saved
/// world's guaranteed reservations on reopening.
pub fn fingerprint(data:&GeneratorData) -> String {
    let mut hash=0xcbf29ce484222325_u64;
    let mut add=|v:&str| {for byte in v.bytes().chain([0]) {hash^=u64::from(byte);hash=hash.wrapping_mul(0x100000001b3);}};
    add(&crate::DATA_VERSION.to_string());
    for map in [&data.noises,&data.densities,&data.rules,&data.settings,&data.presets,&data.world_presets,&data.dimensions,&data.dimension_types] {for (id,raw) in map {add(id);add(raw);}}
    for raw in &data.biome_json {add(raw);}
    for (id,tag) in &data.block_tags {add(id);for v in tag {add(v);}}
    for (id,tag) in &data.biome_tags {add(id);for v in tag {add(v);}}
    if let Some(r)=&data.feature_resources {for map in [&r.features,&r.placed,&r.state_providers,&r.feature_tags] {for (id,raw) in map {add(id);add(raw);}}}
    if let Some(r)=&data.structure_resources {for map in [&r.structures,&r.sets,&r.pools] {for (id,raw) in map {add(id);add(raw);}}}
    format!("{hash:016x}")
}
