fn finite_write_boundary(world:&crate::world::World,x:i32,z:i32)->std::ops::ControlFlow<crate::Result<()>>{
    if horizon_world::veio_map::session().is_none(){return std::ops::ControlFlow::Continue(());}
    let dimension=if world.fast_lava{horizon_world::veio_map::Dimension::Nether}else{horizon_world::veio_map::Dimension::Overworld};
    let b=dimension.bounds();
    if !b.column(x,z)||b.shell(x,z){return std::ops::ControlFlow::Break(Err("finite world boundary".into()));}
    std::ops::ControlFlow::Continue(())
}
