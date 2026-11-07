//! Ground freight uses a real, explicitly assigned animal. Cargo stays in the
//! terminal's escrow until the animal physically reaches the receiving terminal.
use super::*;
use horizon_client::mobs::{Mob, goals::{self, Cx, Goal}};

pub(crate) const ID: &str = "veio:caravan_station";
pub(super) const RANGE: f64 = 512.0;
const FEED: usize = 7;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Kind { Horse, Donkey, Pig }
impl Kind {
    fn of(id: &str) -> Option<Self> { match id {
        "minecraft:horse" => Some(Self::Horse),
        "minecraft:donkey" | "minecraft:mule" => Some(Self::Donkey),
        "minecraft:pig" => Some(Self::Pig), _ => None,
    } }
    pub(super) fn capacity(self) -> u32 { match self { Self::Horse => 16, Self::Donkey => 64, Self::Pig => 8 } }
    fn speed(self) -> f64 { match self { Self::Horse => 1.1, Self::Donkey => 0.75, Self::Pig => 0.5 } }
    fn food(self, item: &str) -> bool { match self {
        Self::Pig => matches!(item, "minecraft:carrot" | "minecraft:potato" | "minecraft:beetroot"),
        _ => matches!(item, "minecraft:wheat" | "minecraft:apple" | "minecraft:hay_block"),
    } }
}
pub(crate) fn feed(item: &str) -> bool { Kind::Horse.food(item) || Kind::Pig.food(item) }

#[derive(Clone)]
struct Animal { epoch: u64, dimension: String, id: u32, kind: Kind, at: [f64; 3], seen: u64 }
#[derive(Clone)]
struct Claim { epoch: u64, uuid: [i32; 4], station: [i32; 3], seen: u64 }
static ANIMALS: Mutex<Vec<Animal>> = Mutex::new(Vec::new());
static CLAIMS: Mutex<Vec<Claim>> = Mutex::new(Vec::new());

pub(crate) fn observe(host: &dyn Host, dimension: &str, id: u32, kind: &str) {
    let Some(kind) = Kind::of(kind) else { return; };
    let now = defense::clock();
    if (now + u64::from(id)) % 10 != 0 { return; }
    let Some(epoch) = host.level(dimension).map(|level| level.epoch().get()) else { return; };
    let Some(at) = host.entity(id).and_then(|handle| handle.position(host)) else { return; };
    if let Ok(mut animals) = ANIMALS.lock() {
        animals.retain(|a| a.epoch == epoch && now.saturating_sub(a.seen) <= 80);
        if let Some(animal) = animals.iter_mut().find(|a| a.id == id && a.dimension == dimension) {
            animal.at = at; animal.seen = now;
        } else if animals.len() < 2048 {
            animals.push(Animal { epoch, dimension: dimension.into(), id, kind, at, seen: now });
        }
    }
}

fn uuid(entity: &ModBlockEntity) -> Option<[i32; 4]> {
    match entity.nbt.get(nbt_key("CaravanAnimal")) {
        Some(Tag::IntArray(v)) if v.len() == 4 => Some([v[0], v[1], v[2], v[3]]), _ => None,
    }
}
fn uuid_of(nbt: &Compound) -> Option<[i32; 4]> {
    match nbt.get(&"UUID".into()) {
        Some(Tag::IntArray(v)) if v.len() == 4 => Some([v[0], v[1], v[2], v[3]]), _ => None,
    }
}
pub(crate) fn saddled(nbt: &Compound) -> bool {
    production::ranch::tag_int(nbt, "Saddle") != 0
        || matches!(nbt.get(&"equipment".into()), Some(Tag::Compound(eq)) if eq.contains_key(&"saddle".into()))
}
fn free(nbt: &Compound, kind: Kind) -> bool {
    let number = |key| production::ranch::tag_int(nbt, key);
    if number("Age") < 0 || number("DeathTime") > 0
        || matches!(nbt.get(&"Health".into()), Some(Tag::Float(bits)) if f32::from_bits(*bits) <= 0.0)
        || nbt.contains_key(&"Leash".into()) || nbt.contains_key(&"leash".into()) || number("Leashed") != 0
        || matches!(nbt.get(&"Passengers".into()), Some(Tag::List(v)) if !v.is_empty()) { return false; }
    let saddle = saddled(nbt);
    match kind {
        Kind::Horse => number("Tame") != 0 && saddle,
        Kind::Donkey => number("Tame") != 0 && number("ChestedHorse") != 0,
        Kind::Pig => saddle,
    }
}
fn claimed(epoch: u64, animal: [i32; 4], station: [i32; 3]) -> bool {
    let now = defense::clock();
    let Ok(mut claims) = CLAIMS.lock() else { return false; };
    claims.retain(|claim| claim.epoch == epoch && now.saturating_sub(claim.seen) <= 80);
    if claims.iter().any(|c| c.uuid == animal && c.station != station) { return false; }
    if let Some(c) = claims.iter_mut().find(|c| c.uuid == animal) { c.seen = now; }
    else { claims.push(Claim { epoch, uuid: animal, station, seen: now }); }
    true
}

fn candidates(epoch: u64, dimension: &str) -> Vec<Animal> {
    ANIMALS.lock().map(|animals| animals.iter().filter(|a| a.epoch == epoch && a.dimension == dimension
        && defense::clock().saturating_sub(a.seen) <= 80).cloned().collect()).unwrap_or_default()
}

/// A lead assigns, rather than consumes or silently taking a passing animal.
pub(super) fn assign(host: &mut dyn Host, pos: [i32; 3], release: bool) {
    let Some(epoch) = host.level(host.dimension()).map(|l| l.epoch().get()) else { return; };
    if release {
        let mut cancel = None;
        host.change_mod_block_entity(pos, &mut |station| {
            if let Some(cargo) = station.items[CARGO].as_ref() {
                let item = cargo.item.to_string(); let count = cargo.count;
                if !farm::drops_fit(&station.items[..3], &[(&item, count)]) { set_number(station, "Status", 3); return; }
                farm::store(&mut station.items[..3], &item, count); station.items[CARGO] = None;
            }
            set_number(station, "FlightPhase", 0);
            if let Some(old) = uuid(station) {
                cancel = Some((number(station, "CaravanEntity"), old));
                if let Ok(mut claims) = CLAIMS.lock() { claims.retain(|c| !(c.epoch == epoch && c.uuid == old && c.station == pos)); }
            }
            station.nbt.remove(nbt_key("CaravanAnimal"));
            set_number(station, "Status", 29);
        });
        if let Some((id, wanted)) = cancel {
            if let Ok(id) = u32::try_from(id) {
                if let Some(handle) = host.entity(id) {
                    if host.entity_nbt(handle).as_ref().and_then(uuid_of) == Some(wanted) { workforce::cancel(id); }
                }
            }
        }
        return;
    }
    let mut nearby = candidates(epoch, host.dimension());
    nearby.retain(|a| (0..3).map(|i| (a.at[i] - center(pos)[i]).powi(2)).sum::<f64>() <= 64.0);
    nearby.sort_by(|a, b| a.at.iter().zip(center(pos)).map(|(v, p)| (v-p).powi(2)).sum::<f64>()
        .total_cmp(&b.at.iter().zip(center(pos)).map(|(v,p)| (v-p).powi(2)).sum::<f64>()));
    for animal in nearby.into_iter().take(4) {
        let Some(handle) = host.entity(animal.id) else { continue; };
        let Some(nbt) = host.entity_nbt(handle) else { continue; };
        let Some(id) = uuid_of(&nbt).filter(|_| free(&nbt, animal.kind)) else { continue; };
        let mut bound = false;
        host.change_mod_block_entity(pos, &mut |station| {
            if number(station, "FlightPhase") == 0 && uuid(station).is_none() && claimed(epoch, id, pos) {
                station.nbt.insert(nbt_key("CaravanAnimal").clone(), Tag::IntArray(id.to_vec()));
                set_number(station, "CaravanEntity", animal.id as i32);
                set_number(station, "Status", 0); bound = true;
            }
        });
        if bound { return; }
    }
}

fn locate(host: &mut dyn Host, pos: [i32; 3], entity: &ModBlockEntity, epoch: u64) -> Option<(Animal, bool)> {
    let wanted = uuid(entity)?;
    let all = candidates(epoch, host.dimension());
    // Bounded round-robin lookup survives runtime entity IDs changing after reload.
    let start = number(entity, "CaravanProbe") as usize;
    for a in all.iter().cycle().skip(start % all.len().max(1)).take(all.len().min(4)) {
        let Some(handle) = host.entity(a.id) else { continue; };
        let Some(nbt) = host.entity_nbt(handle) else { continue; };
        if uuid_of(&nbt) == Some(wanted) && claimed(epoch, wanted, pos) { return Some((a.clone(), free(&nbt, a.kind))); }
    }
    None
}

pub(super) fn tick(host: &mut dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity, partner: Option<&Endpoint>, epoch: u64) {
    let Some((animal, available)) = locate(host, pos, entity, epoch) else {
        set_number(entity, "CaravanProbe", number(entity, "CaravanProbe").wrapping_add(4).max(0));
        set_number(entity, "Status", 29); return;
    };
    set_number(entity, "CaravanEntity", animal.id as i32);
    set_number(entity, "Rate", animal.kind.capacity() as i32);
    if !available { workforce::cancel(animal.id); set_number(entity, "Status", 31); return; }
    let phase = number(entity, "FlightPhase");
    let to = if phase == 1 { target(entity).unwrap_or(pos) } else { pos };
    if host.neighbor_signal(host.dimension(), pos) > 0 {
        workforce::cancel(animal.id); set_number(entity, "Status", 9); return;
    }
    let goal = center(to);
    if animal.at.iter().zip(goal).map(|(a,b)| (a-b).powi(2)).sum::<f64>() > 3.0_f64.powi(2) {
        let at: Vec<i32> = animal.at.iter().map(|v| v.floor() as i32).collect();
        let unchanged = matches!(entity.nbt.get(nbt_key("CaravanLastPos")), Some(Tag::IntArray(old)) if *old == at);
        let stalled = if unchanged { number(entity, "CaravanStall").saturating_add(1) } else { 0 };
        entity.nbt.insert(nbt_key("CaravanLastPos").clone(), Tag::IntArray(at));
        set_number(entity, "CaravanStall", stalled);
        workforce::order(animal.id, goal, defense::clock());
        set_number(entity, "Status", if stalled >= 30 { 22 } else { 23 }); return;
    }
    workforce::cancel(animal.id);
    set_number(entity, "CaravanStall", 0);
    if phase == 2 {
        if let Some(cargo) = entity.items[CARGO].as_ref() {
            let item = cargo.item.to_string(); let count = cargo.count;
            if !farm::drops_fit(&entity.items[..3], &[(&item, count)]) { set_number(entity, "Status", 3); return; }
            farm::store(&mut entity.items[..3], &item, count); entity.items[CARGO] = None;
        }
        set_number(entity, "FlightPhase", 0);
        return;
    }
    if phase == 1 {
        let Some(cargo) = entity.items[CARGO].take() else { set_number(entity, "FlightPhase", 2); return; };
        let count = cargo.count;
        let mut valid = false;
        host.change_mod_block_entity(to, &mut |destination| {
            valid = destination.id == ID && channel(destination) == channel(entity)
                && text(destination, "FreightImport").as_deref() == Some(&cargo.item);
        });
        if valid { entity.items[CARGO] = host.insert_item(to, Direction::Up, cargo); }
        else { entity.items[CARGO] = Some(cargo); set_number(entity, "Status", 28); return; }
        let moved = count - entity.items[CARGO].as_ref().map_or(0, |s| s.count);
        set_number(entity, "Produced", number(entity, "Produced").saturating_add(moved as i32));
        if entity.items[CARGO].is_some() { set_number(entity, "Status", 3); }
        else { set_number(entity, "FlightPhase", 2); set_number(entity, "Total", number(entity, "Total").saturating_add(1)); }
        return;
    }
    let Some(partner) = partner.filter(|p| distance(pos, p.pos) <= RANGE) else { set_number(entity, "Status", 28); return; };
    let Some(item) = export(entity) else { set_number(entity, "Status", 2); return; };
    if partner.export.as_deref() == Some(&item) { set_number(entity, "Status", 28); return; }
    let Some(slot) = (0..3).find(|&i| entity.items[i].as_ref().is_some_and(|s| is_plain(s, &item))) else {
        set_number(entity, "Status", 2); return;
    };
    if entity.items[CARGO].is_some() { set_number(entity, "FlightPhase", 2); return; }
    let food = entity.items[FEED].as_ref().is_some_and(|s| animal.kind.food(&s.item) && s.extra.is_none() && s.damage == 0);
    if !food { set_number(entity, "Status", 30); return; }
    let count = entity.items[slot].as_ref().unwrap().count.min(animal.kind.capacity());
    entity.items[CARGO] = Some(ItemStack::new(&item, count)); consume(&mut entity.items[slot], count);
    consume(&mut entity.items[FEED], 1);
    entity.nbt.insert(nbt_key("FlightTarget").clone(), Tag::IntArray(partner.pos.to_vec()));
    set_number(entity, "FlightPhase", 1);
    workforce::order(animal.id, center(partner.pos), defense::clock());
    set_number(entity, "Status", 23);
}

#[derive(Clone, Debug, Default)]
struct Travel { cooldown: i32 }
impl Goal for Travel {
    fn flags(&self) -> u8 { goals::MOVE | goals::LOOK }
    fn can_use(&mut self, mob: &mut Mob, _: &mut Cx) -> bool {
        mob.rider.is_none() && mob.extra.rider.is_none() && mob.extra.leash.is_none() && mob.extra.leash_pending.is_none()
            && workforce::order_for(mob.id, defense::clock()).is_some()
    }
    fn stop(&mut self, mob: &mut Mob, _: &mut Cx) { mob.ai.nav.stop(); self.cooldown = 0; }
    fn tick(&mut self, mob: &mut Mob, cx: &mut Cx) {
        let Some(goal) = workforce::order_for(mob.id, defense::clock()) else { return; };
        self.cooldown -= 1;
        if self.cooldown > 0 && !mob.ai.nav.is_done() { return; }
        let start = [mob.position.x, mob.position.y, mob.position.z];
        let length = (start[0]-goal[0]).hypot(start[2]-goal[2]);
        let scale = if length > 16.0 { 16.0 / length } else { 1.0 };
        let waypoint = std::array::from_fn(|i| start[i] + (goal[i]-start[i]) * scale);
        let speed = Kind::of(&mob.kind.id).map_or(0.5, Kind::speed);
        mob.nav_move_to(cx.w, waypoint, speed); self.cooldown = 20;
    }
    fn box_clone(&self) -> Box<dyn Goal> { Box::new(self.clone()) }
}
pub(crate) fn register_goals(mob: &mut Mob) {
    if Kind::of(&mob.kind.id).is_some() { mob.ai.goals.add(1, Travel::default()); }
}

#[cfg(test)]
mod tests;
