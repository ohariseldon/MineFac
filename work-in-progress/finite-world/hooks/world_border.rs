fn finite_border(game:&crate::game::Game<'_>)->std::ops::ControlFlow<BorderBounds> {
    let Some(b)=horizon_world::veio_map::for_world(&game.world_root,game.dim.id()) else{return std::ops::ControlFlow::Continue(());};
    std::ops::ControlFlow::Break(horizon_world::level::portal_travel::BorderBounds{min_x:b.min,min_z:b.min,max_x:b.max,max_z:b.max})
}
