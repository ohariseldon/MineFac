use super::*;
use std::{cell::Cell,ops::ControlFlow};
thread_local!{static ENTERING:Cell<bool>=const{Cell::new(false)};}
pub(crate) fn border(game:&mut Game<'_>){
    if let Some(b)=horizon_world::veio_map::for_world(&game.world_root,game.dim.id()) {
        if game.border.center()!=[0.0,0.0]{game.border.set_center([0.0,0.0]);}
        if game.border.size()!=b.max-b.min{game.border.set_size(b.max-b.min);}
    }
}
// An authoritative check also covers spectator flight and invalid positions
// restored from a finite save. In a guest's turn the engine publishes this
// corrected body back to that guest with its normal teleport packet.
pub(crate) fn player(game:&mut Game<'_>) {
    let Some(b)=horizon_world::veio_map::for_world(&game.world_root,game.dim.id()) else{return;};
    border(game);
    let Some(p)=game.player.as_mut() else{return;};
    let half=f64::from(p.dimensions.width)*0.5+0.001;
    let at=b.clamp(p.position.x,p.position.z,half).unwrap_or([0.0,0.0]);
    if at!=[p.position.x,p.position.z] {
        p.position.x=at[0];p.position.z=at[1];
        p.velocity.x=0.0;p.velocity.z=0.0;p.teleported=true;
    }
}
pub(super) fn enter(game:&mut Game<'_>,host:Option<&mut LanHost>,target:Dim)->ControlFlow<Result<()>>{
    if ENTERING.get() || horizon_world::veio_map::for_world(&game.world_root,target.id()).is_none(){return ControlFlow::Continue(());}
    ENTERING.set(true);let result=game.enter_level(host,target);ENTERING.set(false);
    if result.is_ok(){border(game);}
    ControlFlow::Break(result)
}
