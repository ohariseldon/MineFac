fn finite_player_border(game:&mut crate::game::Game<'_>)->std::ops::ControlFlow<()> {
    let Some(b)=horizon_world::veio_map::for_world(&game.world_root,game.dim.id()) else{return std::ops::ControlFlow::Continue(());};
    if let Some(player)=game.player.as_mut(){let mut border=horizon_client::worldborder::WorldBorder::default();border.set_center([0.,0.]);border.set_size(b.max-b.min);player.border=Some(border.geometry().clone());}
    std::ops::ControlFlow::Break(())
}
