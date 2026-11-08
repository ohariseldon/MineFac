//! Observers for transport implementations outside conveyor output dispatch.
//! Hoppers announce attempts, so defer credit until a mod inventory really changes.
use super::*;
const MAX_PENDING: usize = 128;
static PENDING: AtomicBool = AtomicBool::new(false);

pub(super) struct Pending {
    dimension: String,
    from: Pos,
    to: Pos,
    watch: Pos,
    before: u32,
    incoming: bool,
    plate: bool,
    at: u64,
}
impl Pending {
    fn item(&self) -> &'static str { if self.plate { PLATE } else { "minecraft:raw_copper" } }
    fn accepted(&self, after: Option<u32>, now: u64) -> u32 {
        if now.saturating_sub(self.at) > 1 { return 0; }
        let Some(after) = after else { return 0; };
        if self.incoming { after.saturating_sub(self.before).min(1) }
        else { self.before.saturating_sub(after).min(1) }
    }
}

fn count(entity: &ModBlockEntity, item: &str) -> u32 {
    entity.items.iter().flatten().filter(|s| is_plain(s, item)).map(|s| s.count).sum()
}
fn inventory(host: &dyn Host, pos: Pos) -> Option<[u32; 2]> {
    let e = host.mod_block_entity(pos)?;
    Some([count(&e, "minecraft:raw_copper"), count(&e, PLATE)])
}

pub(in super::super) fn before_transfer(host: &dyn Host) { flush(host, None); }
pub(in super::super) fn before_machine(host: &dyn Host, pos: Pos, entity: &ModBlockEntity) { flush(host, Some((pos, entity))); }

pub(in super::super) fn flush(host: &dyn Host, current: Option<(Pos, &ModBlockEntity)>) {
    if !PENDING.load(Ordering::Relaxed) { return; }
    let now = SERVER_CLOCK.load(Ordering::Relaxed);
    let dimension = host.dimension();
    let Ok(mut slot) = SESSION.lock() else { return; };
    let Some(s) = slot.as_mut() else { PENDING.store(false, Ordering::Relaxed); return; };
    let pending = std::mem::take(&mut s.pending);
    for receipt in pending {
        if receipt.dimension != dimension {
            if now.saturating_sub(receipt.at) <= 1 { s.pending.push(receipt); }
            continue;
        }
        let after = if let Some((_, e)) = current.filter(|(pos, _)| *pos == receipt.watch) {
            Some(count(e, receipt.item()))
        } else { host.mod_block_entity(receipt.watch).map(|e| count(&e, receipt.item())) };
        let moved = receipt.accepted(after, now);
        if moved > 0 { record(s, host, receipt.from, receipt.to, receipt.item(), moved); }
    }
    PENDING.store(!s.pending.is_empty(), Ordering::Relaxed);
}

pub(in super::super) fn hopper_attempt(host: &dyn Host, from: Pos, to: Pos, item: &str) {
    if !FLOW_ACTIVE.load(Ordering::Relaxed) || !matches!(item, "minecraft:raw_copper" | PLATE) { return; }
    // The previous attempt has finished before this new attempt starts.
    flush(host, None);
    let dimension = host.dimension();
    let Ok(mut slot) = SESSION.lock() else { return; };
    let Some(s) = slot.as_mut() else { return; };
    let plate = item == PLATE;
    let incoming = !plate;
    let watch = if incoming { to } else { from };
    let interested = s.players.values().any(|p| p.dimension == dimension && matches!(p.step, 5 | 6)
        && p.blocks[PRESS_SLOT] == Some(watch) && (incoming || p.output_credit > 0));
    if !interested || s.pending.len() >= MAX_PENDING { return; }
    let Some(e) = host.mod_block_entity(watch) else { return; };
    s.pending.push(Pending { dimension: dimension.to_owned(), from, to, watch,
        before: count(&e, item), incoming, plate, at: SERVER_CLOCK.load(Ordering::Relaxed) });
    PENDING.store(true, Ordering::Relaxed);
}

/// Preserve the arm's original sided, filtered atomic move. Deltas identify the
/// actual item even when an unfiltered arm skipped a refused source slot.
#[allow(clippy::too_many_arguments)]
pub(in super::super) fn move_items(host: &mut dyn Host, dimension: &str, from: Pos, from_face: Direction,
    to: Pos, to_face: Direction, item: Option<&str>, max: u32) -> u32 {
    if !FLOW_ACTIVE.load(Ordering::Relaxed) {
        return host.move_items(dimension, from, from_face, to, to_face, item, max);
    }
    before_transfer(host);
    let source_before = inventory(host, from);
    let target_before = inventory(host, to);
    let moved = host.move_items(dimension, from, from_face, to, to_face, item, max);
    if moved == 0 { return 0; }
    let source_after = inventory(host, from);
    let target_after = inventory(host, to);
    let mut budget = moved;
    for (index, id) in ["minecraft:raw_copper", PLATE].into_iter().enumerate() {
        let taken = source_before.zip(source_after).map_or(0, |(a,b)| a[index].saturating_sub(b[index]));
        let put = target_before.zip(target_after).map_or(0, |(a,b)| b[index].saturating_sub(a[index]));
        let n = taken.max(put).min(budget);
        if n > 0 { transferred(host, from, to, id, n); budget -= n; }
    }
    moved
}

/// Cargo belongs to the post, not its public input/output slots. Picking up is
/// not a stock delivery; keep its evidence until the worker actually unloads.
pub(in super::super) fn worker_pickup(host: &dyn Host, post: Pos, from: Pos, item: &str, count: u32) {
    if count == 0 || item != PLATE || !FLOW_ACTIVE.load(Ordering::Relaxed) { return; }
    if let Ok(mut slot) = SESSION.lock() && let Some(s) = slot.as_mut() {
        for p in s.players.values_mut().filter(|p| p.dimension == host.dimension()) {
            p.worker_pickup(post, from, item, count, SERVER_CLOCK.load(Ordering::Relaxed));
        }
    }
}
pub(in super::super) fn worker_delivery(host: &dyn Host, post: Pos, to: Pos, item: &str, count: u32) {
    if count == 0 || !FLOW_ACTIVE.load(Ordering::Relaxed) || !matches!(item, "minecraft:raw_copper" | PLATE) { return; }
    let storage = item == PLATE && is_storage(&host.block(to));
    if let Ok(mut slot) = SESSION.lock() && let Some(s) = slot.as_mut() {
        for p in s.players.values_mut().filter(|p| p.dimension == host.dimension()) {
            p.worker_delivery(post, to, item, count, storage, SERVER_CLOCK.load(Ordering::Relaxed));
        }
    }
}
pub(in super::super) fn worker_unload(dimension: &str, post: Pos, load: &[(String, u32)]) {
    if !FLOW_ACTIVE.load(Ordering::Relaxed) { return; }
    if let Ok(mut slot) = SESSION.lock() && let Some(s) = slot.as_mut() {
        for p in s.players.values_mut().filter(|p| p.dimension == dimension) {
            for (item, count) in load {
                if item == PLATE { p.worker_delivery(post, post, item, *count, true, SERVER_CLOCK.load(Ordering::Relaxed)); }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn hopper_attempts_need_real_inventory_delta_and_expire_before_later_activity() {
        let incoming = Pending { dimension: "minecraft:overworld".into(), from: [0;3], to: [1;3], watch: [1;3],
            before: 4, incoming: true, plate: false, at: 20 };
        assert_eq!(incoming.accepted(Some(4), 20), 0); // refused/cancelled
        assert_eq!(incoming.accepted(None, 20), 0); // unloaded/replaced
        assert_eq!(incoming.accepted(Some(5), 21), 1);
        assert_eq!(incoming.accepted(Some(6), 22), 0); // unrelated later change
        let outgoing = Pending { incoming: false, plate: true, ..incoming };
        assert_eq!(outgoing.accepted(Some(4), 20), 0);
        assert_eq!(outgoing.accepted(Some(3), 20), 1);
        assert_eq!(outgoing.accepted(Some(0), 20), 1); // never mint more than one attempt
    }
}

#[cfg(test)]
mod integration {
    use super::*;
    struct World { blocks: HashMap<Pos, ModBlockEntity>, reject: bool }
    impl Host for World {
        fn dimension(&self) -> &'static str { "minecraft:overworld" }
        fn block(&self, pos: Pos) -> String { self.blocks.get(&pos).map_or("minecraft:air".into(), |e| e.id.clone()) }
        fn mod_block_entity(&self, pos: Pos) -> Option<ModBlockEntity> {
            self.blocks.get(&pos).filter(|e| e.id.starts_with("veio:")).cloned()
        }
        fn move_items(&mut self, _: &str, from: Pos, _: Direction, to: Pos, _: Direction, filter: Option<&str>, max: u32) -> u32 {
            if self.reject || max == 0 { return 0; }
            let source = self.blocks.get(&from).unwrap();
            let slot = source.items.iter().enumerate().find(|(index,s)|
                (source.id != PRESS || *index == 1) && s.as_ref().is_some_and(|s| filter.is_none_or(|id| id == &*s.item))
            ).map(|(i,_)| i);
            let Some(slot) = slot else { return 0; };
            let mut one = source.items[slot].clone().unwrap(); one.count = 1;
            let target = self.blocks.get_mut(&to).unwrap();
            if !can_add(&target.items[0], &one.item, 1) { return 0; }
            add(&mut target.items[0], &one.item, 1);
            consume(&mut self.blocks.get_mut(&from).unwrap().items[slot], 1);
            1
        }
    }
    fn entity(id: &str, slots: usize) -> ModBlockEntity {
        let mut e = ModBlockEntity::default(); e.id = id.into(); e.items = vec![None; slots]; e
    }
    fn progress() -> (u32,u32,u32) {
        let guard = SESSION.lock().unwrap(); let p = &guard.as_ref().unwrap().players[&10]; (p.fed,p.made,p.stored)
    }
    // Run separately: the registry intentionally remains active during the harness.
    #[test]
    #[ignore = "run mixed_transport_harness alone with --ignored --test-threads=1"]
    fn mixed_transport_harness_confirms_arms_hoppers_workers_and_blocked_retries_without_belts() {
        let press = [2,64,0]; let input = [0,64,0]; let hopper = [2,65,0]; let chest = [4,64,0]; let post = [6,64,0];
        let mut p = Progress { step: 5, dimension: "minecraft:overworld".into(), ..Default::default() };
        p.blocks[PRESS_SLOT] = Some(press);
        *SESSION.lock().unwrap() = Some(Session { root: PathBuf::new(), players: HashMap::from([(10,p)]), pending: vec![] });
        ACTIVE.store(true, Ordering::Relaxed); FLOW_ACTIVE.store(true, Ordering::Relaxed); SERVER_CLOCK.store(100, Ordering::Relaxed);
        let mut world = World { blocks: HashMap::from([
            (press, entity(PRESS,2)), (input,entity("minecraft:chest",2)), (hopper,entity("minecraft:hopper",5)),
            (chest,entity("minecraft:chest",2)), (post,entity("veio:hauler_post",7))]), reject: false };
        world.blocks.get_mut(&input).unwrap().items[0] = Some(ItemStack::new("minecraft:raw_copper",4));
        let faces = (Direction::East,Direction::West);
        assert_eq!(move_items(&mut world,"minecraft:overworld",input,faces.0,press,faces.1,None,1),1);
        assert_eq!(progress().0,1);
        world.reject = true;
        assert_eq!(move_items(&mut world,"minecraft:overworld",input,faces.0,press,faces.1,None,1),0);
        assert_eq!(progress().0,1);
        world.reject = false;
        hopper_attempt(&world,hopper,press,"minecraft:raw_copper");
        before_transfer(&world); // Cancelled: inventories did not change.
        assert_eq!(progress().0,1);
        hopper_attempt(&world,hopper,press,"minecraft:raw_copper");
        add(&mut world.blocks.get_mut(&press).unwrap().items[0],"minecraft:raw_copper",1);
        // A ticking mod entity is temporarily detached: use its authoritative snapshot.
        let mut machine = world.blocks.remove(&press).unwrap();
        before_machine(&world,press,&machine);
        assert_eq!(progress().0,2);
        machine.items[0] = None; machine.items[1] = Some(ItemStack::new(PLATE,2)); set_number(&mut machine,"Produced",2);
        super::super::observe("minecraft:overworld",press,&machine,0,0,100);
        assert_eq!(progress().1,2);
        SESSION.lock().unwrap().as_mut().unwrap().players.get_mut(&10).unwrap().step = 6;
        // Actual worker supply accepted by the press, not merely carried toward it.
        worker_delivery(&world,post,press,"minecraft:raw_copper",0);
        assert_eq!(progress().0,2);
        worker_delivery(&world,post,press,"minecraft:raw_copper",2);
        machine.items[1] = Some(ItemStack::new(PLATE,4)); set_number(&mut machine,"Produced",4);
        super::super::observe("minecraft:overworld",press,&machine,2,0,100);
        world.blocks.insert(press,machine);
        hopper_attempt(&world,press,hopper,PLATE);
        before_transfer(&world); // Full/cancelled destination: no source decrement.
        assert_eq!(progress().2,0);
        hopper_attempt(&world,press,hopper,PLATE);
        consume(&mut world.blocks.get_mut(&press).unwrap().items[1],1);
        add(&mut world.blocks.get_mut(&hopper).unwrap().items[0],PLATE,1);
        before_transfer(&world);
        assert_eq!(progress().2,1);
        assert_eq!(move_items(&mut world,"minecraft:overworld",press,faces.0,chest,faces.1,None,1),1);
        assert_eq!(progress().2,2);
        for expected in [3,4] {
            consume(&mut world.blocks.get_mut(&press).unwrap().items[1],1);
            worker_pickup(&world,post,press,PLATE,1);
            assert_eq!(progress().2,expected-1);
            worker_delivery(&world,post,post,PLATE,0); // Failed unload retains cargo evidence.
            assert_eq!(progress().2,expected-1);
            add(&mut world.blocks.get_mut(&post).unwrap().items[4],PLATE,1);
            worker_unload("minecraft:overworld",post,&[(PLATE.into(),1)]);
            assert_eq!(progress().2,expected);
        }
        worker_unload("minecraft:overworld",post,&[(PLATE.into(),8)]);
        assert_eq!(progress().2,4);
        assert!(SESSION.lock().unwrap().as_ref().unwrap().players[&10].ready());
        *SESSION.lock().unwrap() = None;
        ACTIVE.store(false,Ordering::Relaxed); FLOW_ACTIVE.store(false,Ordering::Relaxed); PENDING.store(false,Ordering::Relaxed);
    }
}
