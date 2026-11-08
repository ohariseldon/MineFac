use super::*;
use std::{cell::Cell,ops::ControlFlow};
thread_local!{static ENTERING:Cell<bool>=const{Cell::new(false)};}
pub(super) fn border(game:&mut Game<'_>){
    if let Some(b)=horizon_world::veio_map::for_world(&game.world_root,game.dim.id()) {
        if game.border.center()!=[0.0,0.0]{game.border.set_center([0.0,0.0]);}
        if game.border.size()!=b.max-b.min{game.border.set_size(b.max-b.min);}
    }
}
pub(super) fn enter(game:&mut Game<'_>,host:Option<&mut LanHost>,target:Dim)->ControlFlow<Result<()>>{
    if ENTERING.get() || horizon_world::veio_map::for_world(&game.world_root,target.id()).is_none(){return ControlFlow::Continue(());}
    ENTERING.set(true);let result=game.enter_level(host,target);ENTERING.set(false);
    if result.is_ok(){border(game);}
    ControlFlow::Break(result)
}
