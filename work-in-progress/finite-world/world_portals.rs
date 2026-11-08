// Bounds are applied to the target, candidates and final body independently.
use super::*;
use std::{cell::Cell, ops::ControlFlow};
use std::cell::RefCell;
use crate::game::Resources;
thread_local! {
    static PLANNING:Cell<bool>=const{Cell::new(false)};
    static SEARCHING:Cell<bool>=const{Cell::new(false)};
    static EXIT:Cell<bool>=const{Cell::new(false)};
    static ARRIVALS:Cell<bool>=const{Cell::new(false)};
    static EXCLUDED:RefCell<Vec<([i32;3],[i32;3])>>=const{RefCell::new(Vec::new())};
}
pub(super) fn plan(game:&Game<'_>,entry:BlockPos,at:Vec3,width:f32,height:f32,reset:bool)->ControlFlow<Plan> {
    if PLANNING.get() || horizon_world::veio_map::for_world(&game.world_root,game.dim.id()).is_none(){return ControlFlow::Continue(());}
    PLANNING.set(true); let mut plan=game.portal_plan(entry,at,width,height,reset); PLANNING.set(false);
    let b=horizon_world::veio_map::Dimension::of(plan.dim.id()).unwrap().bounds();
    let scale=if game.dim==Dim::Nether {8.0}else{0.125};
    let [x,z]=b.clamp(at.x*scale,at.z*scale,horizon_world::veio_map::PORTAL_TARGET_MARGIN).expect("finite portal target");
    plan.approximate=BlockPos::new(x.floor() as i32,at.y.floor() as i32,z.floor() as i32);
    ControlFlow::Break(plan)
}
pub(super) fn blocks(game:&Game<'_>,center:BlockPos,radius:i32)->ControlFlow<Vec<BlockPos>> {
    let Some(b)=horizon_world::veio_map::for_world(&game.world_root,game.dim.id()) else{return ControlFlow::Continue(());};
    if SEARCHING.get(){return ControlFlow::Continue(());}
    SEARCHING.set(true);let mut found=game.portal_blocks(center,radius);SEARCHING.set(false);
    found.retain(|p|b.contains(p.x as f64,p.z as f64,1.0) && EXCLUDED.with_borrow(|v|!v.iter().any(|(lo,hi)|p.x>=lo[0]&&p.x<=hi[0]&&p.y>=lo[1]&&p.y<=hi[1]&&p.z>=lo[2]&&p.z<=hi[2])));
    ControlFlow::Break(found)
}
pub(super) fn exit(game:&mut Game<'_>,plan:&Plan)->ControlFlow<Option<FoundRectangle>> {
    let Some(b)=horizon_world::veio_map::for_world(&game.world_root,plan.dim.id()) else{return ControlFlow::Continue(());};
    if EXIT.get(){return ControlFlow::Continue(());}
    EXIT.set(true);
    let mut result=None;
    let mut previous=None;
    for _ in 0..16 {
      let Some(rect)=game.nether_exit_portal(plan) else{break;};
      let safe=(|| {
        let corner=rect.min_corner;
        let behaviors=game.res.behaviours();
        let x_axis=behaviors.portal_axis(game.world.block(corner.x,corner.y,corner.z)).unwrap_or(true);
        let far=if x_axis{[corner.x+rect.axis1_size,corner.z]}else{[corner.x,corner.z+rect.axis1_size]};
        if !b.contains(corner.x as f64-1.0,corner.z as f64-1.0,1.0) || !b.contains(far[0] as f64+1.0,far[1] as f64+1.0,1.0){return false;}
        let at=game.arrival(plan,rect).0;
        let half=f64::from(plan.width)*0.5;
        if !at.y.is_finite() || !b.contains(at.x,at.z,half+1.0){return false;}
        let y0=at.y.floor() as i32;
        let y1=(at.y+f64::from(plan.height)-0.001).floor() as i32;
        if y0<game.world.bottom_y()+1 || y1>=game.world.top_y(){return false;}
        for y in y0..=y1 {
            for z in (at.z-half+0.001).floor() as i32..=(at.z+half-0.001).floor() as i32 {
                for x in (at.x-half+0.001).floor() as i32..=(at.x+half-0.001).floor() as i32 {
                    let state=game.world.block(x,y,z);
                    if behaviors.fluid_kind(state).same_family(horizon_world::fluid::FluidKind::Lava) || behaviors.collision_shape(state).is_some_and(|s|!s.is_empty()){return false;}
                }
            }
        }
        let floor=game.world.block(at.x.floor() as i32,y0-1,at.z.floor() as i32);
        behaviors.collision_shape(floor).is_some_and(|s|!s.is_empty()) && !behaviors.fluid_kind(floor).same_family(horizon_world::fluid::FluidKind::Lava)
      })();
      if safe {result=Some(rect);break;}
      if !game.streaming.is_lit(rect.min_corner.x>>4,rect.min_corner.z>>4){break;}
      let key=(rect.min_corner.x,rect.min_corner.y,rect.min_corner.z,rect.axis1_size,rect.axis2_size);
      if previous==Some(key){break;}previous=Some(key);
      // Reject the complete unsafe rectangle, then try another existing
      // portal or the engine's normal safe-platform creation.
      let p=rect.min_corner;
      let x_axis=game.res.behaviours().portal_axis(game.world.block(p.x,p.y,p.z)).unwrap_or(true);
      EXCLUDED.with_borrow_mut(|v|v.push(([p.x,p.y,p.z],[p.x+if x_axis{rect.axis1_size-1}else{0},p.y+rect.axis2_size-1,p.z+if x_axis{0}else{rect.axis1_size-1}])));
    }
    EXIT.set(false);EXCLUDED.with_borrow_mut(Vec::clear);
    ControlFlow::Break(result)
}

pub(crate) fn waiting(game:&Game<'_>,plan:&Plan)->bool {
    let radius=if plan.dim==Dim::Nether{16}else{128};
    let border=game.border_bounds();
    let portals=game.portal_blocks(plan.approximate,radius);
    closest_portal(&portals,plan.approximate,plan.dim==Dim::Nether,&border,if plan.dim==Dim::Nether{0}else{-4},if plan.dim==Dim::Nether{15}else{19})
        .is_some_and(|p|!(-1..=1).all(|z|(-1..=1).all(|x|game.streaming.is_lit((p.x>>4)+x,(p.z>>4)+z))))
}

pub(crate) fn for_rider(mut plan:Plan)->Plan {plan.width=(plan.width+1.2).max(2.0);plan.height+=1.8;plan}

/// The origin travels inside the entity's saved compound, including across a
/// restart while destination chunks load. Strip it when the arrival succeeds.
pub(super) fn mark_origins(game:&mut Game<'_>) {
    if horizon_world::veio_map::for_world(&game.world_root,game.dim.id()).is_none(){return;}
    fn size(res:&Resources,tag:&Tag,origin:Vec3,width:&mut f32,height:&mut f32) {
        let at=horizon_client::entity_nbt::position(tag).unwrap_or(origin);
        let kind=horizon_client::save::text(tag.get("id"));
        let (w,h)=kind.as_deref().and_then(|id|res.mob_types().get(id)).map_or((0.6,1.8),|t|(t.width,t.height));
        *width=width.max((2.0*(at.x-origin.x).abs().max((at.z-origin.z).abs())+w) as f32);
        *height=height.max((at.y-origin.y+h) as f32);
        if let Some(Tag::List(riders))=tag.get("Passengers"){for rider in riders{size(res,rider,origin,width,height);}}
    }
    for arrival in &mut game.portals.arrivals {
        if arrival.tag.get("VeioPortalOrigin").is_some(){continue;}
        if let Some(at)=horizon_client::entity_nbt::position(&arrival.tag) {
            size(&game.res,&arrival.tag,at,&mut arrival.plan.width,&mut arrival.plan.height);
        }
        if let Tag::Compound(c)=&mut arrival.tag {c.insert("VeioPortalOrigin".into(),Tag::String(game.dim.id().into()));}
    }
}

/// The engine otherwise discards a non-player entity when no exit exists.
/// Preserve its complete saved entity tree and restore it in the source level.
pub(super) fn resolve(game:&mut Game<'_>)->ControlFlow<()> {
    if ARRIVALS.get() || horizon_world::veio_map::for_world(&game.world_root,game.dim.id()).is_none(){return ControlFlow::Continue(());}
    let current=game.dim;
    let mut restore=Vec::new();
    let mut waiting_arrivals=Vec::new();
    let pending=std::mem::take(&mut game.portals.arrivals);
    for mut arrival in pending {
        let p=arrival.plan;
        let ready=!p.end && p.dim==current && (-1..=1).all(|z|(-1..=1).all(|x|game.streaming.is_lit((p.approximate.x>>4)+x,(p.approximate.z>>4)+z)));
        if ready && game.nether_exit_portal(&p).is_none(){
            if waiting(game,&p){waiting_arrivals.push(arrival);}else{restore.push(arrival);}
        }else{
            if (ready||p.end)&&let Tag::Compound(c)=&mut arrival.tag {c.remove(&"VeioPortalOrigin".into());}
            game.portals.arrivals.push(arrival);
        }
    }
    ARRIVALS.set(true);game.resolve_arrivals();ARRIVALS.set(false);
    game.portals.arrivals.extend(waiting_arrivals);
    for mut arrival in restore {
        let source=horizon_client::save::text(arrival.tag.get("VeioPortalOrigin")).map(|id|Dim::from_id(&id)).unwrap_or_else(||if arrival.plan.dim==Dim::Nether{Dim::Overworld}else{Dim::Nether});
        let Some(at)=horizon_client::entity_nbt::position(&arrival.tag) else{game.portals.arrivals.push(arrival);continue;};
        if game.enter_level(None,source).is_err(){game.portals.arrivals.push(arrival);continue;}
        if let Tag::Compound(c)=&mut arrival.tag {c.insert("PortalCooldown".into(),Tag::Int(100));c.remove(&"VeioPortalOrigin".into());}
        super::super::persist::mark_fresh(&mut arrival.tag);
        let key=((at.x.floor() as i32)>>4,(at.z.floor() as i32)>>4);
        game.place_portal_ticket(key);
        game.world.loaded_entities.push((key,vec![arrival.tag]));
        let _=game.enter_level(None,current);
        game.portals.dirty=true;
    }
    ControlFlow::Break(())
}
