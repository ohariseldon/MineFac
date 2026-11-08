fn finite_break_boundary(game:&crate::game::Game<'_>,pos:[i32;3])->std::ops::ControlFlow<Result<()>>{
    if let Some(b)=horizon_world::veio_map::for_world(&game.world_root,game.dim.id()) && (!b.column(pos[0],pos[2])||b.shell(pos[0],pos[2])){return std::ops::ControlFlow::Break(Ok(()));}
    std::ops::ControlFlow::Continue(())
}
