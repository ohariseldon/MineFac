fn finite_save(save:&crate::save::SaveDir,folder:crate::save::Folder,records:&Vec<((i32,i32),crate::save::Record)>,wait:bool)->std::ops::ControlFlow<crate::Result<()>> {
    let Some(session)=horizon_world::veio_map::session() else{return std::ops::ControlFlow::Continue(());};
    let dimension=[horizon_world::veio_map::Dimension::Overworld,horizon_world::veio_map::Dimension::Nether,horizon_world::veio_map::Dimension::End].into_iter().find(|d|crate::save::dimension_dir(&session.root,d.id())==save.root);
    let Some(dimension)=dimension else{return std::ops::ControlFlow::Continue(());};
    let b=dimension.bounds();
    if records.iter().all(|((x,z),_)|b.chunk(*x,*z,0)){return std::ops::ControlFlow::Continue(());}
    let kept=records.iter().filter(|((x,z),_)|b.chunk(*x,*z,0)).cloned().collect();
    std::ops::ControlFlow::Break(save.queue(folder,kept,wait))
}
