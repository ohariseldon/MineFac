//! Runs the real finite generator with the player's prepared resources.
//! The Python driver supplies the repository path of the planner module.
use horizon_world::{registry::BlockRegistry, worldgen::generator::{GeneratorData, OverworldGenerator}};
use horizon_core::position::ChunkPos;
use std::{fs, path::PathBuf};

// PLANNER_MODULE

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().collect();
    let resources = PathBuf::from(&args[1]);
    let output = PathBuf::from(&args[2]);
    let seed: i64 = args.get(3).map_or(Ok(0), |s| s.parse())?;
    let registry = Box::leak(Box::new(BlockRegistry::from_facts_json(fs::File::open(resources.join("facts.json"))?)?));
    let data=GeneratorData::load(&resources.join("worldgen"), registry)?;
    if args.iter().any(|a|a=="--audit-only") {
        for dimension in [horizon_world::veio_map::Dimension::Overworld,horizon_world::veio_map::Dimension::Nether,horizon_world::veio_map::Dimension::End] {
            let biomes=horizon_world::worldgen::generator::veio_catalog::biomes(&data,"minecraft:normal",dimension.id())?;
            let result=horizon_world::worldgen::generator::veio_resources::audit(&data,dimension,&biomes);
            println!("{}: {result:?}",dimension.id());
            result?;
        }
        return Ok(());
    }
    let plan = if output.exists() { serde_json::from_slice(&fs::read(&output)?)? } else {
        let plan = planner::build(&data, seed)?;
        fs::write(&output, serde_json::to_vec_pretty(&plan)?)?;
        plan
    };
    for (id,dimension) in plan["dimensions"].as_object().unwrap() {
        println!("{id}: {} biome regions, {} structure reservations, {} guaranteed source habitats",dimension["biomes"].as_object().unwrap().len(),dimension["structures"].as_array().unwrap().len(),dimension["resources"]["sources"].as_object().map_or(0,|v|v.len()));
    }
    let seed=plan["seed"].as_i64().unwrap();
    horizon_world::veio_map::activate(output.parent().unwrap(),Some(plan.clone()));
    for dimension in [horizon_world::veio_map::Dimension::Overworld,horizon_world::veio_map::Dimension::Nether,horizon_world::veio_map::Dimension::End] {
        let generator=OverworldGenerator::for_world(data.clone(),seed,"minecraft:normal",dimension.id(),true)?;
        let mut worker=generator.worker();
        let bounds=dimension.bounds();
        let (lo,hi)=bounds.walls();
        for (x,z) in [(lo,0),(hi,0),(0,lo),(0,hi),(lo,lo),(lo,hi),(hi,lo),(hi,hi)] {
            let chunk=worker.generate_proto(ChunkPos::new(x.div_euclid(16),z.div_euclid(16)))?;
            for y in [chunk.terrain.min_y(),64,chunk.terrain.min_y()+chunk.terrain.height()-1] {
                assert_eq!(chunk.block_name(x,y,z),"minecraft:bedrock","wall at {x} {y} {z}");
            }
        }
        let chunk=worker.generate_proto(ChunkPos::new(1_000_000,1_000_000))?;
        assert_eq!(chunk.block_name(16_000_000,64,16_000_000),"minecraft:air");
        println!("{}: eight wall/corner probes and far exterior passed",dimension.id());
        let biome_names=plan["dimensions"][dimension.id()]["biomes"].as_object().unwrap().keys().cloned().collect::<Vec<_>>();
        horizon_world::worldgen::generator::veio_resources::validate(&data,dimension,&biome_names,&plan["dimensions"][dimension.id()]["resources"])?;
        if args.iter().any(|a|a=="--habitats") {
            for (index,(name,proof)) in plan["dimensions"][dimension.id()]["resources"]["sources"].as_object().unwrap().iter().enumerate() {
                let at:[i32;3]=std::array::from_fn(|i|proof["at"][i].as_i64().unwrap() as i32);
                let center=ChunkPos::new(at[0]>>4,at[2]>>4);
                let mut chunks=Vec::new();
                for z in -1..=1 {for x in -1..=1 {chunks.push(worker.generate_proto(ChunkPos::new(center.x+x,center.z+z))?);}}
                let mut region:[_;9]=chunks.try_into().ok().unwrap();
                worker.decorate(&mut region)?;
                assert_eq!(region[4].block_name(at[0],at[1],at[2]),name);
                if name.contains("_ore")||name=="minecraft:ancient_debris" {
                    let mut count=0;
                    for z in -1..=2 {for x in -1..=2 {for y in 0..=3 {count+=usize::from(region[4].block_name(at[0]+x,at[1]+y,at[2]+z)==name);}}}
                    assert_eq!(count,64,"guaranteed vein {name}");
                }
                if index==0 {
                    let air=registry.default_state("minecraft:air").unwrap();
                    region[4].terrain.set_block_raw(at[0],at[1],at[2],air)?;
                    worker.decorate(&mut region)?;
                    assert_eq!(region[4].block_name(at[0],at[1],at[2]),"minecraft:air","a mined habitat must stay mined");
                    let mined=region.into_iter().nth(4).unwrap();
                    let mut neighbours=Vec::new();
                    for z in -1..=1 {for x in 0..=2 {neighbours.push(worker.generate_proto(ChunkPos::new(center.x+x,center.z+z))?);}}
                    let mut neighbours:[_;9]=neighbours.try_into().ok().unwrap();
                    neighbours[3]=mined;
                    worker.decorate(&mut neighbours)?;
                    assert_eq!(neighbours[3].block_name(at[0],at[1],at[2]),"minecraft:air","new neighbouring terrain must preserve mining/building");
                }
                if index%20==0 {println!("{}: verified {} generated habitats",dimension.id(),index+1);}
            }
            println!("{}: every resource habitat exists in decorated native terrain",dimension.id());
        }
        if args.iter().any(|a|a=="--sources") {
            let biomes:Vec<_>=plan["dimensions"][dimension.id()]["biomes"].as_object().unwrap().keys().cloned().collect();
            let catalog=horizon_world::worldgen::generator::veio_catalog::sources(&data,&biomes)?;
            let mut found=std::collections::BTreeMap::new();
            for (biome,sites) in plan["dimensions"][dimension.id()]["biomes"].as_object().unwrap() {
                for at in sites.as_array().unwrap().iter().take(2) {
                    let center=ChunkPos::new(at[0].as_i64().unwrap() as i32>>4,at[2].as_i64().unwrap() as i32>>4);
                    let mut chunks=Vec::new();
                    for z in -1..=1 {for x in -1..=1 {chunks.push(worker.generate_proto(ChunkPos::new(center.x+x,center.z+z))?);}}
                    let mut region: [_;9]=chunks.try_into().ok().unwrap();
                    let stats=worker.decorate(&mut region)?;
                    for chunk in &region {
                        for index in chunk.terrain.filled_sections() {
                            let y=chunk.terrain.min_y()+index as i32*16;
                            let mut states=[0_u32;4096];
                            assert!(chunk.terrain.section_states(y,&mut states));
                            for (i,id) in states.into_iter().enumerate() {
                                let name=&registry.state(horizon_world::registry::BlockStateId(id)).unwrap().name;
                                if catalog.contains_key(name) {
                                    found.entry(name.clone()).or_insert([chunk.position().x*16+(i as i32&15),y+(i as i32>>8),chunk.position().z*16+(i as i32>>4&15)]);
                                }
                            }
                        }
                    }
                    println!("{} {biome}: {}/{} source blocks; failures {}",dimension.id(),found.len(),catalog.len(),stats.failed);
                }
            }
            let missing:Vec<_>=catalog.keys().filter(|k|!found.contains_key(*k)).collect();
            println!("{} missing: {missing:?}",dimension.id());
            fs::write(output.with_extension(dimension.id().replace(':',"-")+"-sources.json"),serde_json::to_vec_pretty(&found)?)?;
        }
    }
    Ok(())
}
