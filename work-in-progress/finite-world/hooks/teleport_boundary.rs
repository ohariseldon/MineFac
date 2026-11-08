fn finite_teleport_boundary(game:&mut crate::game::Game<'_>,at:[f64;3])->std::ops::ControlFlow<()> {
    if let Some(b)=horizon_world::veio_map::for_world(&game.world_root,game.dim.id()) && !b.contains(at[0],at[2],2.0){return std::ops::ControlFlow::Break(());}
    std::ops::ControlFlow::Continue(())
}
