// Per-world opt-in, saved before the first chunk is generated.
use super::*;
use std::{cell::Cell, fs, io::Write, ops::ControlFlow};
use horizon_world::veio_map as finite;
use horizon_world::worldgen::generator::{GeneratorData,veio_catalog};
use crate::game::world_loading;
thread_local! { static PREPARING: Cell<bool> = const { Cell::new(false) }; }
struct Preparing;
impl Drop for Preparing { fn drop(&mut self) { PREPARING.set(false); } }

fn validate(data:&std::sync::Arc<GeneratorData>,plan: &serde_json::Value) -> std::result::Result<(), String> {
    if plan["format"].as_u64() != Some(finite::FORMAT) || plan["side"].as_u64() != Some(15_000)
        || plan["data_version"].as_i64() != Some(i64::from(horizon_world::DATA_VERSION)) || plan["seed"].as_i64().is_none() {
        return Err("incompatible finite world certificate".to_owned());
    }
    for dimension in [finite::Dimension::Overworld,finite::Dimension::Nether,finite::Dimension::End] {
        let d=&plan["dimensions"][dimension.id()];
        if d["biomes"].as_object().is_none_or(|v|v.is_empty()) || d["structures"].as_array().is_none_or(|v|v.is_empty()) { return Err("incomplete finite world certificate".to_owned()); }
        let expected=veio_catalog::biomes(data,"minecraft:normal",dimension.id())?;
        if expected.len()!=d["biomes"].as_object().unwrap().len() || expected.iter().any(|name|d["biomes"][name].as_array().is_none_or(|v|v.is_empty())) {return Err("incomplete biome certificate".into());}
        for name in veio_catalog::structures(data,&expected)?.keys() {
            for variant in veio_catalog::variants(data,name)? {
                if !d["structures"].as_array().unwrap().iter().any(|s|s["name"].as_str()==Some(name)&&s["variant"].as_str()==Some(&variant)) {return Err(format!("missing reserved structure {name} {variant}"));}
            }
        }
        horizon_world::worldgen::generator::veio_resources::validate(data,dimension,&expected,&d["resources"])?;
        for s in d["structures"].as_array().unwrap() {
            if s["name"].as_str().is_none() || s["chunk"].as_array().is_none_or(|v|v.len()!=2 || v.iter().any(|n|n.as_i64().is_none())) {return Err("invalid structure reservation".to_owned());}
            let array=|key:&str|->Option<[i32;3]> { let v=s[key].as_array()?; if v.len()!=3{return None;} Some([i32::try_from(v[0].as_i64()?).ok()?,i32::try_from(v[1].as_i64()?).ok()?,i32::try_from(v[2].as_i64()?).ok()?]) };
            let (Some(min),Some(max))=(array("min"),array("max")) else {return Err("invalid structure bounds".to_owned());};
            if !dimension.bounds().contains_box(min,max,finite::PORTAL_MARGIN) {return Err("structure beyond finite world boundary".to_owned());}
        }
    }
    Ok(())
}

pub(super) fn prepare(request:&world_loading::WorldRequest,res:&Resources,pack:&PackResources,options:&Options,settings:&Settings,upgrade:&horizon_world::datafix::progress::UpgradeProgress)->ControlFlow<Result<world_loading::PreparedWorld>> {
    if PREPARING.get() { return ControlFlow::Continue(()); }
    let root=&request.save_root;
    finite::activate(root,None);
    let outcome=(||->Result<Option<serde_json::Value>> {
        let file=root.join(finite::FILE);
        if !file.try_exists()? && finite::existing(root)? { return Ok(None); }
        // Use the engine's OS-backed lock. A crash releases it automatically;
        // no stale planning.lock can strand a half-created world.
        let _lock=horizon_client::save::SessionLock::acquire(root)?;
        let overrides=if let Some(create)=&request.create {
            creation_worldgen_overrides(create.pack_dir.as_deref(),&options.resources,&create.data_packs,res.biomes())
        } else {
            let features=Level::read(root).map_or_else(Vec::new,|l|l.enabled_features);
            let biomes=packed_biomes(root,&options.resources,res.biomes(),&features);
            world_worldgen_overrides(root,&options.resources,&biomes,&features)
        };
        let data=GeneratorData::load_with(&options.resources.join("worldgen"),pack.assets.registry,&overrides)?;
        let catalog=veio_catalog::fingerprint(&data);
        if file.try_exists()? {
            let plan:serde_json::Value=serde_json::from_slice(&fs::read(&file)?)?;
            validate(&data,&plan).map_err(|e|format!("{}: {e}",file.display()))?;
            if plan["catalog"].as_str()!=Some(&catalog) {return Err(horizon_client::lang::get("veio.world.catalog_changed").into());}
            if let Some(level)=Level::read(root) && Some(level.seed)!=plan["seed"].as_i64() {return Err("finite world seed mismatch; save left unchanged".into());}
            return Ok(Some(plan));
        }
        if finite::existing(root)? { return Ok(None); }
        // A complete finite world uses the normal generator. Preserve existing
        // worlds with other presets; new incompatible presets fail explicitly.
        if request.create.as_ref().is_some_and(|o|o.world_type!=horizon_client::save::WorldPreset::Normal || !o.structures) {
            return Err(horizon_client::lang::get("veio.world.requires_normal").into());
        }
        let dir=file.parent().expect("certificate parent"); fs::create_dir_all(dir)?;
        let seed=request.seed.unwrap_or_else(||std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0,|d|d.as_nanos() as i64));
        let mut plan=crate::game::veio_plan::build(&data,seed)
            .map_err(|e|format!("{}\n{e}",horizon_client::lang::get("veio.world.generation_failed")))?;
        plan["catalog"]=catalog.into();
        validate(&data,&plan)?;
        let nonce=std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)?.as_nanos();
        let temporary=dir.join(format!("world-plan.{}.{nonce}.tmp",std::process::id()));
        struct Temporary(PathBuf);
        impl Drop for Temporary {fn drop(&mut self){let _=fs::remove_file(&self.0);}}
        let _temporary=Temporary(temporary.clone());
        let mut output=fs::OpenOptions::new().write(true).create_new(true).open(&temporary)?;
        output.write_all(&serde_json::to_vec_pretty(&plan)?)?; output.sync_all()?; drop(output);
        fs::rename(&temporary,&file)?;
        Ok(Some(plan))
    })();
    let plan=match outcome { Ok(Some(plan))=>plan,Ok(None)=>return ControlFlow::Continue(()),Err(e)=>return ControlFlow::Break(Err(e)) };
    let seed=plan["seed"].as_i64().unwrap();
    finite::activate(root,Some(plan));
    if Level::read(root).is_some() { return ControlFlow::Continue(()); }
    let next=world_loading::WorldRequest{save_root:root.clone(),folder:request.folder.clone(),name:request.name.clone(),seed:Some(seed),creative:request.creative,create:request.create.clone()};
    PREPARING.set(true); let _guard=Preparing;
    ControlFlow::Break(next.prepare(res,pack,options,settings,upgrade))
}
