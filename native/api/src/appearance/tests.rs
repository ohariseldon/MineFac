//! Exercise the actual render callbacks, not a copy of the machine drawings.
use super::*;
use std::cell::RefCell;
use horizon_client::mod_api::{BlockEntityDraw, BlockStateId, DrawOp};
#[derive(Default)]
struct Probe { cubes: RefCell<Vec<String>> }
impl Host for Probe {
    fn block_state_property(&self, _: BlockStateId, property: &str) -> Option<&'static str> {
        Some(if property == "facing" { "south" } else { "straight" })
    }
    fn draw_cube(&self, draw: &mut BlockEntityDraw, lo: [f32;3], hi: [f32;3], _: [Option<CubeFace<'_>>;6], color: u32) -> bool {
        let operations: Vec<String> = draw.trace.iter().map(|op| match op {
            DrawOp::Push => "[\"push\"]".into(), DrawOp::Pop => "[\"pop\"]".into(), DrawOp::Still => "[\"still\"]".into(),
            DrawOp::Translate(p) => format!("[\"translate\",{p:?}]"), DrawOp::Scale(p) => format!("[\"scale\",{p:?}]"),
            DrawOp::Rotate(a,r,p) => format!("[\"rotate\",{a:?},{r},{p:?}]"),
            DrawOp::Spin(a,p,r,f) => format!("[\"spin\",{a:?},{p:?},{r},{f}]"),
        }).collect();
        self.cubes.borrow_mut().push(format!("{{\"lo\":{lo:?},\"hi\":{hi:?},\"color\":{color},\"ops\":[{}]}}",operations.join(",")));
        true
    }
}
const MACHINES: &[&str] = &["drill","drill_mk2","drill_mk3","press","assembler","integrator","cutter","crusher",
    "alloy_furnace","electric_furnace","electronics","mill","kitchen","harvester","planter","tree_farm","ranch",
    "generator","biomass_generator","steam_engine","geothermal","wind_turbine","water_wheel","drone_port","belt","fast_belt","splitter","merger","inserter","fast_inserter","long_inserter"];
fn entity(block: &str) -> ModBlockEntity {
    let mut entity=ModBlockEntity::default(); entity.id=block.into();
    entity.items=vec![None;layout(block).unwrap().0];
    for (key,value) in [("Status",1),("Burn",100),("Info",16),("FlightPhase",1),("Duration",40),("MovedAt",0)] { set_number(&mut entity,key,value); }
    entity.nbt.insert(nbt_key("FlightTarget").clone(),Tag::IntArray(vec![0,0,0]));
    entity
}
fn draw(block: &str, entity: &ModBlockEntity, time: i64) -> (BlockEntityDraw, Vec<String>) {
    let host=Probe::default(); let mut draw=BlockEntityDraw::default(); draw.game_time=time; draw.partial=0.25;
    presentation::draw(&host, &mut BlockEntitySubmitEvent::new([0;3],block,entity,&mut draw));
    (draw,host.cubes.into_inner())
}
#[test]
fn every_mechanical_machine_moves_and_pause_states_do_not_animate() {
    let mut previews=Vec::new();
    for &name in MACHINES {
        let block=format!("veio:{name}"); let mut machine=entity(&block);
        let (active,first)=draw(&block,&machine,0);
        let (_,later)=draw(&block,&machine,9);
        let spins=active.trace.iter().any(|op| matches!(op,DrawOp::Spin(_,_,speed,_) if *speed != 0.0));
        assert!(spins || first!=later, "{name} is still static");
        // For natural wind/water, rotation intentionally follows the available
        // wind/water even if a full buffer is not accepting power.
        if !matches!(name,"wind_turbine"|"water_wheel") {
            for status in [3,5,9,13] {
                set_number(&mut machine,"Status",status);
                let (stopped,a)=draw(&block,&machine,0); let (_,b)=draw(&block,&machine,9);
                assert_eq!(a,b,"{name} moves while stopped ({status})");
                assert!(!stopped.trace.iter().any(|op| matches!(op,DrawOp::Spin(_,_,speed,_) if *speed!=0.0)),"{name} rotor still spins");
            }
        }
        set_number(&mut machine,"Status",1);
        let period=if matches!(name,"electronics"|"ranch"|"alloy_furnace"|"electric_furnace") {48} else {32};
        let frames: Vec<String>=(0..8).map(|i| {let time=i*period/8;let (_,cubes)=draw(&block,&machine,time);format!("{{\"time\":{},\"cubes\":[{}]}}",time as f32+0.25,cubes.join(","))}).collect();
        previews.push(format!("{{\"name\":\"{name}\",\"frames\":[{}]}}",frames.join(",")));
    }
    if let Ok(path)=std::env::var("VEIO_MOTION_PREVIEW") { std::fs::write(path,format!("[{}]",previews.join(","))).unwrap(); }
}
#[test]
fn stopped_burners_and_static_furniture_keep_cached_draws() {
    let mut generator=entity(GENERATOR);set_number(&mut generator,"Burn",0);
    let (draw,_) = draw(GENERATOR,&generator,0);
    assert!(draw.kept);
    assert!(!draw.trace.iter().any(|op| matches!(op,DrawOp::Spin(_,_,rate,_) if *rate!=0.0)));
    for name in ["pantry","barn","silo","battery","foreman_desk","control_center"] {
        let block=format!("veio:{name}"); let e=entity(&block);
        let (cached,_) = self::draw(&block,&e,0); assert!(cached.kept,"{name}");
    }
}
