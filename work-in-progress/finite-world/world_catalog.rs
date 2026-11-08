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
