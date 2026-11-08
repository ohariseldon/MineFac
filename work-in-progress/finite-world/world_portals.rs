// Bounds are applied to the target, candidates and final body independently.
use super::*;
use std::{cell::Cell, ops::ControlFlow};
thread_local! {
    static PLANNING:Cell<bool>=const{Cell::new(false)};
    static SEARCHING:Cell<bool>=const{Cell::new(false)};
    static EXIT:Cell<bool>=const{Cell::new(false)};
    static ARRIVALS:Cell<bool>=const{Cell::new(false)};
}
pub(super) fn plan(game:&Game<'_>,entry:BlockPos,at:Vec3,width:f32,height:f32,reset:bool)->ControlFlow<Plan> {
    if PLANNING.get() || horizon_world::veio_map::for_world(&game.world_root,game.dim.id()).is_none(){return ControlFlow::Continue(());}
    PLANNING.set(true); let mut plan=game.portal_plan(entry,at,width,height,reset); PLANNING.set(false);
    let b=horizon_world::veio_map::Dimension::of(plan.dim.id()).unwrap().bounds();
    let scale=if game.dim==Dim::Nether {8.0}else{0.125};
    let [x,z]=b.clamp(at.x*scale,at.z*scale,horizon_world::veio_map::PORTAL_MARGIN).expect("finite portal target");
    plan.approximate=BlockPos::new(x.floor() as i32,at.y.floor() as i32,z.floor() as i32);
    ControlFlow::Break(plan)
}
pub(super) fn blocks(game:&Game<'_>,center:BlockPos,radius:i32)->ControlFlow<Vec<BlockPos>> {
    let Some(b)=horizon_world::veio_map::for_world(&game.world_root,game.dim.id()) else{return ControlFlow::Continue(());};
    if SEARCHING.get(){return ControlFlow::Continue(());}
    SEARCHING.set(true);let mut found=game.portal_blocks(center,radius);SEARCHING.set(false);
    found.retain(|p|b.contains(p.x as f64,p.z as f64,horizon_world::veio_map::PORTAL_MARGIN));
    ControlFlow::Break(found)
}
pub(super) fn exit(game:&mut Game<'_>,plan:&Plan)->ControlFlow<Option<FoundRectangle>> {
    let Some(b)=horizon_world::veio_map::for_world(&game.world_root,plan.dim.id()) else{return ControlFlow::Continue(());};
    if EXIT.get(){return ControlFlow::Continue(());}
    EXIT.set(true);let found=game.nether_exit_portal(plan);EXIT.set(false);
    let safe=found.filter(|rect| {
        let corner=rect.min_corner;
        let behaviors=game.res.behaviours();
        let x_axis=behaviors.portal_axis(game.world.block(corner.x,corner.y,corner.z)).unwrap_or(true);
        let far=if x_axis{[corner.x+rect.axis1_size,corner.z]}else{[corner.x,corner.z+rect.axis1_size]};
        if !b.contains(corner.x as f64-1.0,corner.z as f64-1.0,1.0) || !b.contains(far[0] as f64+1.0,far[1] as f64+1.0,1.0){return false;}
        let at=game.arrival(plan,*rect).0;
        let half=f64::from(plan.width)*0.5;
        if !at.y.is_finite() || !b.contains(at.x,at.z,half+1.0){return false;}
        let y0=at.y.floor() as i32;
        let y1=(at.y+f64::from(plan.height)-0.001).floor() as i32;
        if y0<game.world.bottom_y()+1 || y1>=game.world.top_y(){return false;}
        for y in y0..=y1 {
            for z in (at.z-half+0.001).floor() as i32..=(at.z+half-0.001).floor() as i32 {
                for x in (at.x-half+0.001).floor() as i32..=(at.x+half-0.001).floor() as i32 {
                    let state=game.world.block(x,y,z);
                    if behaviors.fluid_kind(state)==horizon_world::fluid::FluidKind::Lava || behaviors.collision_shape(state).is_some_and(|s|!s.is_empty()){return false;}
                }
            }
        }
        let floor=game.world.block(at.x.floor() as i32,y0-1,at.z.floor() as i32);
        behaviors.collision_shape(floor).is_some_and(|s|!s.is_empty()) && behaviors.fluid_kind(floor)!=horizon_world::fluid::FluidKind::Lava
    });
    ControlFlow::Break(safe)
}

/// The engine otherwise discards a non-player entity when no exit exists.
/// Preserve its complete saved entity tree and restore it in the source level.
pub(super) fn resolve(game:&mut Game<'_>)->ControlFlow<()> {
    if ARRIVALS.get() || horizon_world::veio_map::for_world(&game.world_root,game.dim.id()).is_none(){return ControlFlow::Continue(());}
    let current=game.dim;
    let mut restore=Vec::new();
    let pending=std::mem::take(&mut game.portals.arrivals);
    for arrival in pending {
        let p=arrival.plan;
        let ready=!p.end && p.dim==current && (-1..=1).all(|z|(-1..=1).all(|x|game.streaming.is_lit((p.approximate.x>>4)+x,(p.approximate.z>>4)+z)));
        if ready && game.nether_exit_portal(&p).is_none(){restore.push(arrival);}else{game.portals.arrivals.push(arrival);}
    }
    ARRIVALS.set(true);game.resolve_arrivals();ARRIVALS.set(false);
    for mut arrival in restore {
        let source=if arrival.plan.dim==Dim::Nether{Dim::Overworld}else{Dim::Nether};
        let Some(at)=horizon_client::entity_nbt::position(&arrival.tag) else{game.portals.arrivals.push(arrival);continue;};
        if game.enter_level(None,source).is_err(){game.portals.arrivals.push(arrival);continue;}
        if let Tag::Compound(c)=&mut arrival.tag {c.insert("PortalCooldown".into(),Tag::Int(100));}
        super::super::persist::mark_fresh(&mut arrival.tag);
        let key=((at.x.floor() as i32)>>4,(at.z.floor() as i32)>>4);
        game.place_portal_ticket(key);
        game.world.loaded_entities.push((key,vec![arrival.tag]));
        let _=game.enter_level(None,current);
        game.portals.dirty=true;
    }
    ControlFlow::Break(())
}
