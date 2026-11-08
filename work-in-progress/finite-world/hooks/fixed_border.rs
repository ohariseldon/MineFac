fn finite_fixed_border(game:&crate::game::Game<'_>)->std::ops::ControlFlow<R<i32>> {
    if horizon_world::veio_map::for_world(&game.world_root,game.dim.id()).is_some() {
        return std::ops::ControlFlow::Break(Err(crate::game::commands::err("veio.world.fixed_border",&[])));
    }
    std::ops::ControlFlow::Continue(())
}
