// Keep players and their vehicles at the source until an exit is verified.
use super::*;
use std::{cell::Cell, ops::ControlFlow, path::PathBuf, sync::Mutex};
struct Pending {root:PathBuf,conn:u32,source:Dim,target:Dim,plan:Plan,from:[f64;3]}
static PENDING:Mutex<Vec<Pending>>=Mutex::new(Vec::new());
thread_local! {static STARTING:Cell<bool>=const{Cell::new(false)};}

pub(super) fn start(game:&mut Game<'_>,host:&mut LanHost,conn:u32,target:Dim,plan:Option<Plan>)->ControlFlow<()> {
    if STARTING.get() || horizon_world::veio_map::for_world(&game.world_root,target.id()).is_none(){return ControlFlow::Continue(());}
    let Some(plan)=plan else{return ControlFlow::Continue(());};
    let Some(guest)=host.hub.guests.iter().find(|g|g.conn==conn)else{return ControlFlow::Break(());};
    let source=game.dim;let from=guest.position;
    if game.enter_level(Some(host),target).is_err(){return ControlFlow::Break(());}
    game.place_portal_ticket((plan.approximate.x>>4,plan.approximate.z>>4));
    let _=game.enter_level(Some(host),source);
    let mut pending=PENDING.lock().unwrap_or_else(|p|p.into_inner());
    pending.retain(|p|p.root==game.world_root && p.conn!=conn);
    pending.push(Pending{root:game.world_root.clone(),conn,source,target,plan,from});
    ControlFlow::Break(())
}

pub(super) fn tick(game:&mut Game<'_>,host:&mut LanHost) {
    if horizon_world::veio_map::for_world(&game.world_root,game.dim.id()).is_none(){return;}
    let current=game.dim;
    let pending=std::mem::take(&mut *PENDING.lock().unwrap_or_else(|p|p.into_inner()));
    let mut waiting=Vec::new();
    for p in pending {
        if p.root!=game.world_root{continue;}
        let Some(guest)=host.hub.guests.iter().find(|g|g.conn==p.conn)else{continue;};
        if guest.dimension!=p.source.id() || guest.position.iter().zip(p.from).any(|(a,b)|(a-b).abs()>5.0){continue;}
        if p.target!=current{waiting.push(p);continue;}
        game.place_portal_ticket((p.plan.approximate.x>>4,p.plan.approximate.z>>4));
        if !(-2..=2).all(|z|(-2..=2).all(|x|game.streaming.is_lit((p.plan.approximate.x>>4)+x,(p.plan.approximate.z>>4)+z))){waiting.push(p);continue;}
        if game.nether_exit_portal(&p.plan).is_none(){
            let text=horizon_client::text::Text::tr("veio.world.portal_blocked",&[]);
            let packet=C::SystemChat(play::SystemChat{content:horizon_client::lan::data::component_nbt(&text.component()),overlay:false});
            host.hub.send(p.conn,&packet);continue;
        }
        if game.enter_level(Some(host),p.source).is_err(){waiting.push(p);continue;}
        STARTING.set(true);game.start_server_trip(host,p.conn,p.target,Some(p.plan));STARTING.set(false);
        let _=game.enter_level(Some(host),current);
    }
    PENDING.lock().unwrap_or_else(|p|p.into_inner()).extend(waiting);
}
