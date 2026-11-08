// Small survival habitats, derived from the version's reachable feature graph.
// They supplement normal generation rather than relying on a rare ore/plant
// happening to roll inside a finite area. Only new generation uses these stamps.
use super::*;
use crate::{block_survival::SurvivalWorld,block_collision::{DynamicCollisionWorld,ShulkerCollisionState},dynamic_support::DynamicSupportWorld,fluid::FluidState};
use crate::veio_map::Dimension;
use horizon_core::geometry::VoxelShape;
use serde_json::json;

struct Habitat<'a> {
    data:&'a GeneratorData,
    blocks:BTreeMap<[i32;3],BlockStateId>,
    light:i32,
}
impl DynamicSupportWorld for Habitat<'_> {
    type Error=String;
    fn shulker_is_closed(&mut self,_:[i32;3])->Result<Option<bool>,String>{Ok(None)}
    fn moving_piston_shape(&mut self,_:[i32;3])->Result<Option<VoxelShape>,String>{Ok(None)}
}
impl DynamicCollisionWorld for Habitat<'_> {
    fn shulker_collision_state(&mut self,_:[i32;3])->Result<Option<ShulkerCollisionState>,String>{Ok(None)}
}
impl SurvivalWorld for Habitat<'_> {
    fn fluid_state(&mut self,p:[i32;3])->Result<FluidState,String>{let id=self.block_state(p)?;FluidState::from_block(self.data.registry.state(id).unwrap()).map_err(|e|e.to_string())}
    fn raw_brightness(&mut self,_:[i32;3],_:i32)->Result<i32,String>{Ok(self.light)}
    fn block_state(&mut self,p:[i32;3])->Result<BlockStateId,String>{Ok(self.blocks.get(&p).copied().unwrap_or_else(||self.data.registry.default_state("minecraft:air").unwrap()))}
}
fn state(data:&GeneratorData,name:&str)->Result<BlockStateId,String>{data.registry.default_state(name).ok_or_else(||format!("missing habitat block {name}"))}
fn with(data:&GeneratorData,id:BlockStateId,properties:&[(&str,&str)])->BlockStateId {
    let mut s=data.registry.state(id).unwrap().clone();
    for &(key,value) in properties {if s.properties.contains_key(key){s.properties.insert(key.into(),value.into());}}
    data.registry.id(&s).unwrap_or(id)
}
fn survives(data:&GeneratorData,world:&mut Habitat<'_>,id:BlockStateId,at:[i32;3])->Result<bool,String>{
    let b=data.feature_resources.as_ref().and_then(|r|r.blocks.as_ref()).ok_or("missing survival rules")?;
    b.survival.can_survive(world,id,at).map_err(|e|e.to_string())
}

// Search support arrangements with the real engine's survival rules. The
// winning arrangement is saved, not reconstructed from a future rule table.
fn habitat(data:&GeneratorData,name:&str,at:[i32;3],dark:bool,dimension:Dimension)->Result<(BTreeMap<[i32;3],BlockStateId>,BlockStateId),String> {
    let default=state(data,name)?;
    let variants=[default,with(data,default,&[("north","true")]),with(data,default,&[("hanging","false")]),with(data,default,&[("waterlogged","true")])];
    let air=state(data,"minecraft:air")?;
    let water=state(data,"minecraft:water")?;
    let [x,y,z]=at;
    let base=match dimension {Dimension::Nether=>"minecraft:netherrack",Dimension::End=>"minecraft:end_stone",Dimension::Overworld=>if dark{"minecraft:stone"}else{"minecraft:grass_block"}};
    for soil in [base,"minecraft:mycelium","minecraft:sand","minecraft:soul_sand","minecraft:crimson_nylium","minecraft:warped_nylium","minecraft:moss_block","minecraft:clay","minecraft:mud","minecraft:farmland","minecraft:stone","minecraft:end_stone","minecraft:dripstone_block","minecraft:sulfur","minecraft:jungle_log"] {
      let soil=with(data,state(data,soil)?,&[("moisture","7")]);
      for wet in [false,true] {
       if wet && dimension==Dimension::Nether {continue;}
       if !wet && (name.contains("coral")||name.contains("seagrass")||name.contains("kelp")||name=="minecraft:sea_pickle"){continue;}
       for support in [0,1,2] {
        for &raw in &variants {
         let id=with(data,raw,&[("distance","1"),("persistent","true"),("berries","true")]);
         let mut world=Habitat{data,blocks:BTreeMap::new(),light:if dark{0}else{15}};
         // A contained 5x5 floor, space for growth, and an open approach.
         for dz in -2..=2 {for dx in -2..=2 {
            world.blocks.insert([x+dx,y-2,z+dz],state(data,base)?);
            for dy in -1..=4 {world.blocks.insert([x+dx,y+dy,z+dz],if dy<0{soil}else if wet && dy<=1{water}else{air});}
         }}
         if !wet && dimension!=Dimension::Nether {world.blocks.insert([x+1,y-1,z],water);}
         if support==1 {world.blocks.insert([x,y+1,z],soil);}
         if support==2 {world.blocks.insert([x,y,z-1],soil);}
         if name.ends_with("_leaves") {
            let log=name.replace("_leaves","_log");
            let log=state(data,&log).or_else(|_|state(data,"minecraft:oak_log"))?;
            world.blocks.insert([x,y-1,z],log);
         }
         if name=="minecraft:cocoa" {world.blocks.insert([x,y,z+1],state(data,"minecraft:jungle_log")?);}
         if name=="minecraft:lily_pad" {world.blocks.insert([x,y-1,z],water);}
         if name=="minecraft:cactus_flower" {
            for dz in -2..=2 {for dx in -2..=2 {world.blocks.insert([x+dx,y-1,z+dz],air);}}
            world.blocks.insert([x,y-2,z],state(data,"minecraft:sand")?);
            world.blocks.insert([x,y-1,z],state(data,"minecraft:cactus")?);
         }
         if name.starts_with("minecraft:attached_") {
            let fruit=if name.contains("melon"){"minecraft:melon"}else{"minecraft:pumpkin"};
            world.blocks.insert([x,y,z-1],state(data,fruit)?);
         }
         if name=="minecraft:chorus_plant" {world.blocks.insert([x,y-1,z],state(data,"minecraft:end_stone")?);}
         if name=="minecraft:cave_vines_plant" {world.blocks.insert([x,y+1,z],soil);world.blocks.insert([x,y-1,z],state(data,"minecraft:cave_vines")?);}
         if name=="minecraft:kelp_plant" {world.blocks.insert([x,y-1,z],soil);world.blocks.insert([x,y+1,z],state(data,"minecraft:kelp")?);}
         if name=="minecraft:weeping_vines_plant" {world.blocks.insert([x,y+1,z],soil);world.blocks.insert([x,y-1,z],state(data,"minecraft:weeping_vines")?);}
         if name=="minecraft:twisting_vines_plant" {world.blocks.insert([x,y+1,z],state(data,"minecraft:twisting_vines")?);}
         if name=="minecraft:big_dripleaf_stem" {world.blocks.insert([x,y+1,z],with(data,state(data,"minecraft:big_dripleaf")?,&[("waterlogged","true")]));}
         let s=data.registry.state(id).unwrap();
         let mut checked=vec![(at,id)];
         if s.properties.get("half").is_some_and(|v|v=="lower") && name!="minecraft:pitcher_crop" {
            let upper=with(data,id,&[("half","upper")]);world.blocks.insert([x,y+1,z],upper);checked.push(([x,y+1,z],upper));
         }
         world.blocks.insert(at,id);
         if name.contains("_ore")||name=="minecraft:ancient_debris" {
            for dz in -1..=2 {for dx in -1..=2 {for dy in 0..=3 {world.blocks.insert([x+dx,y+dy,z+dz],id);}}}
         }
         if checked.iter().all(|&(p,s)|survives(data,&mut world,s,p).unwrap_or(false)) {
            return Ok((world.blocks,id));
         }
        }
       }
      }
    }
    Err(format!("no stable survival habitat for {name}"))
}

pub fn catalog(data:&GeneratorData,dimension:Dimension,names:&[String])->Result<BTreeMap<String,Vec<String>>,String> {
    let mut sources=super::veio_catalog::sources(data,names)?;
    fn surface(value:&Value,data:&GeneratorData,seen:&mut BTreeSet<String>,found:&mut BTreeSet<String>)->Result<(),String> {
        match value {
            Value::Object(o)=>for v in o.values(){surface(v,data,seen,found)?;},
            Value::Array(a)=>for v in a{surface(v,data,seen,found)?;},
            Value::String(s)=>{
                if data.registry.default_state(s).is_some() && !matches!(s.as_str(),"minecraft:air"|"minecraft:bedrock"){found.insert(s.clone());}
                if seen.insert(s.clone()) && let Some(raw)=data.rules.get(s){surface(&serde_json::from_str::<Value>(raw).map_err(|e|e.to_string())?,data,seen,found)?;}
            },_=>{}
        }Ok(())
    }
    let preset:Value=serde_json::from_str(data.world_presets.get("minecraft:normal").ok_or("missing normal preset")?).map_err(|e|e.to_string())?;
    let setting=preset["dimensions"][dimension.id()]["generator"]["settings"].as_str().ok_or("missing noise settings")?;
    let setting:Value=serde_json::from_str(data.settings.get(setting).ok_or("missing noise settings")?).map_err(|e|e.to_string())?;
    let mut found=BTreeSet::new();
    for field in ["default_block","default_fluid","surface_rule"] {surface(&setting[field],data,&mut BTreeSet::new(),&mut found)?;}
    for name in found {sources.entry(name).or_insert_with(||names.to_vec());}
    // These algorithms select their palette internally, without a block-state
    // field in JSON. Bind their actual registered source blocks as well.
    let implicit:&[(&str,&str)]=match dimension {
        Dimension::End=>&[("minecraft:end_highlands","minecraft:chorus_flower")],
        Dimension::Nether=>&[("minecraft:warped_forest","minecraft:warped_stem"),("minecraft:warped_forest","minecraft:warped_wart_block"),("minecraft:warped_forest","minecraft:shroomlight"),("minecraft:crimson_forest","minecraft:crimson_stem"),("minecraft:nether_wastes","minecraft:nether_quartz_ore"),("minecraft:nether_wastes","minecraft:ancient_debris"),("minecraft:nether_wastes","minecraft:nether_wart")],
        Dimension::Overworld=>&[("minecraft:warm_ocean","minecraft:sea_pickle"),("minecraft:deep_dark","minecraft:sculk"),("minecraft:deep_dark","minecraft:sculk_shrieker"),("minecraft:deep_dark","minecraft:sculk_catalyst"),("minecraft:jungle","minecraft:cocoa"),("minecraft:flower_forest","minecraft:bee_nest")],
    };
    for &(biome,name) in implicit {state(data,name)?;if !names.iter().any(|b|b==biome){return Err(format!("missing implicit source biome {biome}"));}sources.entry(name.into()).or_default().push(biome.into());}
    if dimension==Dimension::Overworld {
        if let Some(crops)=data.block_tags.get("minecraft:maintains_farmland") {
            for name in crops {if data.registry.default_state(name).is_some(){sources.entry(name.clone()).or_insert_with(||vec!["minecraft:plains".into()]);}}
        }
        for i in 0..data.registry.len() {
            let name=&data.registry.state(BlockStateId(i as u32)).unwrap().name;
            if name.starts_with("minecraft:") && !name.contains("dead_") && !name.contains("wall_") && (name.ends_with("_coral")||name.ends_with("_coral_block")||name.ends_with("_coral_fan")) {sources.entry(name.clone()).or_insert_with(||vec!["minecraft:warm_ocean".into()]);}
        }
    }
    Ok(sources)
}

pub fn build(data:&Arc<GeneratorData>,worker:&mut GeneratorWorker,dimension:Dimension,biomes:&BTreeMap<String,Vec<[i32;3]>>,structures:&[Value],seed:i64)->Result<Value,String> {
    let names:Vec<_>=biomes.keys().cloned().collect();
    let sources=catalog(data,dimension,&names)?;
    let mut used=BTreeSet::new();
    let mut palette:Vec<BlockStateId>=Vec::new();
    let mut patches=Vec::new();
    let mut proofs=serde_json::Map::new();
    for (index,(name,allowed)) in sources.iter().enumerate() {
        let mut found=None;
        'site: for i in 0..allowed.len() {
            let biome=&allowed[i.wrapping_add(seed as usize).wrapping_add(index)%allowed.len()];
            for origin in biomes.get(biome).into_iter().flatten() {
              for radius in 0_i32..=16 {
               for (dx,dz) in [(radius,0),(-radius,0),(0,radius),(0,-radius),(radius,radius),(-radius,-radius),(radius,-radius),(-radius,radius)] {
                let pos=ChunkPos::new((origin[0]>>4)+dx,(origin[2]>>4)+dz);
                if used.contains(&(pos.x,pos.z)) || !dimension.bounds().contains((pos.x*16+8) as f64,(pos.z*16+8) as f64,64.0){continue;}
                let (x,z)=(pos.x*16+8,pos.z*16+8);
                if structures.iter().any(|s|x>=s["min"][0].as_i64().unwrap() as i32-16 && x<=s["max"][0].as_i64().unwrap() as i32+16 && z>=s["min"][2].as_i64().unwrap() as i32-16 && z<=s["max"][2].as_i64().unwrap() as i32+16){continue;}
                let Some(surface)=worker.surface_sample(x,z) else{continue;};
                let underground=name.contains("_ore")||name=="minecraft:ancient_debris"||biome.ends_with("_caves")||biome.ends_with(":deep_dark");
                let submerged=name.contains("coral")||name.contains("seagrass")||name.contains("kelp")||name=="minecraft:sea_pickle";
                let y=if underground {if dimension==Dimension::Nether{24}else if name.starts_with("minecraft:deepslate_"){-40}else{origin[1].min(surface.height-12).clamp(-48,32)}}else if dimension==Dimension::Nether{origin[1].clamp(20,100)}else if !submerged{surface.water.map_or(surface.height,|(h,_)|h.max(surface.height))}else{surface.height};
                let (bottom,top)=if dimension==Dimension::Overworld{(-64,320)}else{(0,256)};
                if y<bottom+2||y+4>=top{continue;}
                if worker.noise_biomes(&[[x>>2,(y-1)>>2,z>>2]])[0].as_deref()!=Some(biome){continue;}
                let at=[x,y,z];
                let (blocks,id)=habitat(data,name,at,underground||dimension==Dimension::Nether,dimension)?;
                found=Some((pos,at,blocks,id,biome.clone(),underground||dimension==Dimension::Nether));break 'site;
               }
              }
            }
        }
        let (pos,at,blocks,id,biome,dark)=found.ok_or_else(||format!("{}: no safe habitat site for {name}",dimension.id()))?;
        used.insert((pos.x,pos.z));
        let mut entries=Vec::new();
        for (p,s) in blocks {
            let idx=match palette.iter().position(|v|*v==s){Some(i)=>i,None=>{palette.push(s);palette.len()-1}};
            entries.push(json!([p[0],p[1],p[2],idx]));
        }
        proofs.insert(name.clone(),json!({"at":at,"biome":biome,"state":data.registry.state(id).unwrap().name,"light":if dark{0}else{15}}));
        patches.push(json!({"chunk":[pos.x,pos.z],"source":name,"at":at,"blocks":entries}));
    }
    let palette:Vec<_>=palette.into_iter().map(|id|{let s=data.registry.state(id).unwrap();json!({"name":s.name,"properties":s.properties})}).collect();
    Ok(json!({"palette":palette,"patches":patches,"sources":proofs}))
}

pub fn audit(data:&GeneratorData,dimension:Dimension,names:&[String])->Result<(),String> {
    let mut missing=Vec::new();
    for name in catalog(data,dimension,names)?.keys() {
        if let Err(e)=habitat(data,name,[8,64,8],dimension==Dimension::Nether,dimension){missing.push(e);}
    }
    if missing.is_empty(){Ok(())}else{Err(missing.join("; "))}
}

pub type Patches=BTreeMap<(i32,i32),Vec<([i32;3],BlockStateId)>>;
pub fn decode(data:&GeneratorData,value:&Value)->Result<Patches,String> {
    let palette:Vec<_>=value["palette"].as_array().ok_or("missing habitat palette")?.iter().map(|s|{
        let name=s["name"].as_str().ok_or("invalid habitat state")?.to_owned();
        let properties=serde_json::from_value(s["properties"].clone()).map_err(|e|e.to_string())?;
        data.registry.id(&crate::chunk::BlockState{name,properties}).ok_or_else(||"unknown habitat state".to_owned())
    }).collect::<Result<_,String>>()?;
    let mut result=Patches::new();
    for p in value["patches"].as_array().ok_or("missing resource habitats")? {
        let pos=(p["chunk"][0].as_i64().ok_or("invalid habitat chunk")? as i32,p["chunk"][1].as_i64().ok_or("invalid habitat chunk")? as i32);
        let list=result.entry(pos).or_default();
        for b in p["blocks"].as_array().ok_or("missing habitat blocks")? {
            let at=[0,1,2].map(|i|b[i].as_i64().and_then(|v|i32::try_from(v).ok()));
            let [Some(x),Some(y),Some(z)]=at else{return Err("invalid habitat coordinate".into());};
            if (x>>4,z>>4)!=pos{return Err("habitat crosses its reserved chunk".into());}
            let id=palette.get(b[3].as_u64().ok_or("invalid habitat palette index")? as usize).ok_or("invalid habitat palette index")?;
            list.push(([x,y,z],*id));
        }
    }
    Ok(result)
}

pub fn validate(data:&GeneratorData,dimension:Dimension,names:&[String],value:&Value)->Result<Patches,String> {
    let patches=decode(data,value)?;
    let expected=catalog(data,dimension,names)?;
    let sources=value["sources"].as_object().ok_or("missing source certificate")?;
    if sources.len()!=expected.len(){return Err("incomplete source certificate".into());}
    let (bottom,top)=if dimension==Dimension::Overworld {(-64,320)}else{(0,256)};
    for blocks in patches.values() {
        for &(p,_) in blocks {if p[1]<bottom||p[1]>=top||!dimension.bounds().contains(p[0] as f64,p[2] as f64,32.0){return Err("resource habitat beyond build bounds".into());}}
    }
    for (name,allowed) in expected {
        let proof=&value["sources"][&name];
        if !proof["biome"].as_str().is_some_and(|b|allowed.iter().any(|v|v==b)){return Err(format!("invalid source biome for {name}"));}
        let p:Vec<i32>=proof["at"].as_array().ok_or("missing resource coordinate")?.iter().map(|v|v.as_i64().and_then(|n|i32::try_from(n).ok()).ok_or("invalid resource coordinate")).collect::<Result<_,_>>()?;
        let at:[i32;3]=p.try_into().map_err(|_|"invalid resource coordinate")?;
        let blocks=patches.get(&(at[0]>>4,at[2]>>4)).ok_or("missing resource habitat")?;
        let light=proof["light"].as_i64().filter(|v|matches!(v,0|15)).ok_or("invalid habitat lighting")? as i32;
        let mut world=Habitat{data,blocks:blocks.iter().copied().collect(),light};
        let id=world.block_state(at)?;
        if data.registry.state(id).is_none_or(|s|s.name!=name){return Err(format!("missing guaranteed source {name}"));}
        if !survives(data,&mut world,id,at)? {return Err(format!("unstable guaranteed source {name}"));}
    }
    Ok(patches)
}

pub fn apply(chunk:&mut ProtoChunk,patches:&Patches)->Result<(),GeneratorError> {
    let p=chunk.position();let Some(blocks)=patches.get(&(p.x,p.z)) else{return Ok(());};
    for &(at,id) in blocks {
        chunk.terrain.set_block_raw(at[0],at[1],at[2],id).map_err(stage("resource habitat"))?;
        chunk.block_entities.retain(|(p,_)|p!=&at);
        let name=&chunk.data.registry.state(id).unwrap().name;
        if matches!(name.as_str(),"minecraft:beehive"|"minecraft:bee_nest") {
            chunk.block_entities.retain(|(p,_)|p!=&at);
            chunk.block_entities.push((at,super::super::tree::beehive_occupants(&[0,0,0])));
        }
        if name=="minecraft:suspicious_sand" {
            use crate::nbt::Tag;
            chunk.block_entities.retain(|(p,_)|p!=&at);
            let tag=Tag::compound([("id",Tag::String("minecraft:brushable_block".into())),("LootTable",Tag::String("minecraft:archaeology/desert_well".into())),("LootTableSeed",Tag::Long((i64::from(at[0])<<32)^i64::from(at[2])))]);
            if let Tag::Compound(tag)=tag {chunk.block_entities.push((at,tag));}
        }
    }
    chunk.terrain.prime_heightmaps(&[HeightmapType::MotionBlocking,HeightmapType::MotionBlockingNoLeaves,HeightmapType::OceanFloor,HeightmapType::WorldSurface]).map_err(stage("resource heightmaps"))?;
    Ok(())
}
