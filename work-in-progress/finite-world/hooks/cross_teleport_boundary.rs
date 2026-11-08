fn finite_cross_teleport(game:&crate::game::Game<'_>,dimension:crate::game::Dim,pos:[f64;3])->std::ops::ControlFlow<Result<bool,horizon_client::command::CommandError>>{
    if let Some(b)=horizon_world::veio_map::for_world(&game.world_root,dimension.id()) && !b.contains(pos[0],pos[2],2.0){return std::ops::ControlFlow::Break(Err(crate::game::commands::err("veio.world.outside",&[])));}
    std::ops::ControlFlow::Continue(())
}
