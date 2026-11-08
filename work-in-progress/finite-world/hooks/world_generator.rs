fn finite_generator(data:std::sync::Arc<crate::worldgen::generator::GeneratorData>,seed:i64,preset:&str,dimension:&str,structures:bool,saved:Option<&serde_json::Value>)->std::ops::ControlFlow<Result<crate::worldgen::generator::OverworldGenerator,crate::worldgen::generator::GeneratorError>> {
    crate::worldgen::generator::veio_finite::construct(data,seed,preset,dimension,structures,saved)
}
