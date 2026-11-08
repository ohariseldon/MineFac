fn finite_world_entry(request:&crate::game::world_loading::WorldRequest,res:&crate::game::Resources,pack:&crate::game::PackResources,options:&crate::game::Options,settings:&crate::game::Settings,upgrade:&horizon_world::datafix::progress::UpgradeProgress)->std::ops::ControlFlow<Result<PreparedWorld>> {
    crate::game::world_loading::veio_entry::prepare(request,res,pack,options,settings,upgrade)
}
