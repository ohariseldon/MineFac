//! Inter-zone logistics. Workers plan outbound stock and consume received stock;
//! hoppers exchange rail cargo, while a drone port owns its cargo until delivery.
use super::*;
use horizon_client::mod_api::{BlockEntitySubmitEvent, CubeFace, Text};
use std::sync::Mutex;

pub(super) const RAIL: &str = "veio:freight_station";
pub(super) const DRONE: &str = "veio:drone_port";
pub(super) const CARD: &str = "veio:route_card";
pub(super) const RANGE: i32 = 64;
const CARGO: usize = 6;
const STALE: u64 = 100;

pub(super) fn owns(block: &str) -> bool { matches!(block, RAIL | DRONE) }
pub(super) fn layout(block: &str) -> Option<(usize, Vec<usize>, Vec<usize>)> {
    owns(block).then(|| (if block == DRONE { 7 } else { 6 }, vec![0, 1, 2], vec![3, 4, 5]))
}

fn text(entity: &ModBlockEntity, key: &str) -> Option<String> {
    match entity.nbt.get(nbt_key(key)) {
        Some(Tag::String(s)) if !java_eq(s, "") => Some(s.to_string_lossy()),
        _ => None,
    }
}
fn set_text(entity: &mut ModBlockEntity, key: &str, value: Option<&str>) {
    if let Some(value) = value { entity.nbt.insert(nbt_key(key).clone(), Tag::String(value.into())); }
    else { entity.nbt.remove(nbt_key(key)); }
}
pub(super) fn export(entity: &ModBlockEntity) -> Option<String> { text(entity, "FreightExport") }
pub(super) fn channel(entity: &ModBlockEntity) -> i32 { number(entity, "FreightChannel").clamp(0, 16) }

/// Planning never treats received stock as a request to send it straight back.
pub(super) fn accepts_plan(entity: &ModBlockEntity, slot: usize, stack: &ItemStack) -> bool {
    slot < 3 && export(entity).is_some_and(|item| is_plain(stack, &item))
}
pub(super) fn accepts_delivery(entity: &ModBlockEntity, slot: usize, stack: &ItemStack) -> bool {
    let item = if slot < 3 { export(entity) }
        else if slot < 6 { text(entity, "FreightImport") } else { None };
    item.is_some_and(|item| is_plain(stack, &item))
}

#[derive(Clone, Debug)]
struct Endpoint { epoch: u64, dimension: String, pos: [i32; 3], kind: String, channel: i32, export: Option<String>, seen: u64 }
static PORTS: Mutex<Vec<Endpoint>> = Mutex::new(Vec::new());

fn register(endpoint: Endpoint) -> Option<Endpoint> {
    let mut ports = PORTS.lock().ok()?;
    ports.retain(|old| old.epoch == endpoint.epoch && endpoint.seen.saturating_sub(old.seen) <= STALE
        && !(old.dimension == endpoint.dimension && old.pos == endpoint.pos));
    ports.push(endpoint.clone());
    if endpoint.channel == 0 { return None; }
    let mut others = ports.iter().filter(|other| other.epoch == endpoint.epoch
        && other.dimension == endpoint.dimension && other.kind == endpoint.kind
        && other.channel == endpoint.channel && other.pos != endpoint.pos);
    let partner = others.next()?.clone();
    // Ambiguous routes stop rather than choosing a possibly wrong factory.
    others.next().is_none().then_some(partner)
}

pub(super) fn right_click(host: &mut dyn Host, pos: [i32; 3], block: &str) -> Option<Flow> {
    if !owns(block) { return None; }
    let held = host.held().map(|(item, _, _)| item);
    if held.as_deref() != Some(CARD) && !host.sneaking()
        && (held.is_none() || held.as_deref() == Some(WRENCH)) { return None; }
    let backwards = host.sneaking();
    host.change_mod_block_entity(pos, &mut |entity| {
        if number(entity, "FlightPhase") != 0 {
            if backwards && held.as_deref() == Some(WRENCH) {
                if number(entity, "FlightPhase") == 1 {
                    let remaining = number(entity, "Duration") - number(entity, "Progress");
                    set_number(entity, "Progress", remaining.max(0));
                    set_number(entity, "FlightPhase", 2);
                }
            }
            return;
        }
        if held.as_deref() == Some(CARD) {
            let next = (channel(entity) + if backwards { 16 } else { 1 }) % 17;
            set_number(entity, "FreightChannel", next);
            set_number(entity, "Info", next);
            set_text(entity, "FreightImport", None);
            set_number(entity, "FreightManualImport", 0);
        } else if backwards {
            set_text(entity, "FreightExport", held.as_deref().filter(|item| *item != WRENCH));
        } else {
            set_text(entity, "FreightImport", held.as_deref());
            set_number(entity, "FreightManualImport", 1);
        }
    });
    host.block_entity_sync(pos);
    Some(Flow::Cancel)
}

fn distance(a: [i32; 3], b: [i32; 3]) -> f64 {
    (0..3).map(|i| (f64::from(a[i]) - f64::from(b[i])).powi(2)).sum::<f64>().sqrt()
}
fn target(entity: &ModBlockEntity) -> Option<[i32; 3]> {
    match entity.nbt.get(nbt_key("FlightTarget")) {
        Some(Tag::IntArray(v)) if v.len() == 3 => Some([v[0], v[1], v[2]]),
        _ => None,
    }
}

/// Check the small swept craft volume before advancing; never force a chunk.
fn clear_step(host: &dyn Host, level: horizon_client::mod_api::LevelRef, from: [i32; 3], to: [i32; 3], fraction: f64) -> bool {
    let base: [f64; 3] = std::array::from_fn(|i| f64::from(from[i]) + f64::from(to[i] - from[i]) * fraction);
    for x in [0.1, 0.9] { for y in [3.0, 3.5] { for z in [0.1, 0.9] {
        let at = [(base[0] + x).floor() as i32, (base[1] + y).floor() as i32, (base[2] + z).floor() as i32];
        if !host.level_is_loaded(level, at) || !host.level_block(level, at).is_some_and(|id|
            matches!(id.as_str(), "minecraft:air" | "minecraft:cave_air" | "minecraft:void_air")) { return false; }
    } } }
    true
}

pub(super) fn tick(host: &mut dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity, time: i64) {
    let phase = number(entity, "FlightPhase");
    // No global scan: a loaded endpoint announces itself once per second.
    let due = (time + i64::from((pos[0] ^ pos[2]) & 15)).rem_euclid(20) == 0;
    if !due && phase == 0 { return; }
    let Some(level) = host.level(host.dimension()) else { return; };
    let epoch = level.epoch().get();
    let partner = if due {
        let partner = register(Endpoint { epoch, dimension: host.dimension().into(), pos,
            kind: entity.id.clone(), channel: channel(entity), export: export(entity), seen: defense::clock() });
        if number(entity, "FreightManualImport") == 0 {
            if let Some(other) = partner.as_ref() { set_text(entity, "FreightImport", other.export.as_deref()); }
            else if entity.id == DRONE { set_text(entity, "FreightImport", None); }
        }
        partner
    } else { None };
    set_number(entity, "Info", channel(entity));
    if entity.id == RAIL {
        set_number(entity, "Status", if partner.is_some() || text(entity, "FreightImport").is_some() { 27 } else { 28 });
        return;
    }
    if host.neighbor_signal(host.dimension(), pos) > 0 {
        set_number(entity, "Status", 9);
        return;
    }
    if phase > 0 {
        let duration = number(entity, "Duration").max(1);
        let progress = number(entity, "Progress");
        if progress < duration {
            let Some(to) = target(entity) else { set_number(entity, "Status", 22); return; };
            let fraction = f64::from(progress + 1) / f64::from(duration);
            let fraction = if phase == 2 { 1.0 - fraction } else { fraction };
            if !clear_step(host, level, pos, to, fraction) { set_number(entity, "Status", 22); return; }
            set_number(entity, "Progress", progress + 1);
            set_number(entity, "Status", 1);
            return;
        }
        if phase == 2 {
            if let Some(cargo) = entity.items[CARGO].as_ref() {
                let item = cargo.item.to_string();
                let count = cargo.count;
                if !farm::drops_fit(&entity.items[..3], &[(&item, count)]) {
                    set_number(entity, "Status", 3); return;
                }
                farm::store(&mut entity.items[..3], &item, count);
                entity.items[CARGO] = None;
            }
            set_number(entity, "FlightPhase", 0);
            set_number(entity, "Progress", 0);
            return;
        }
        if entity.items[CARGO].is_none() {
            set_number(entity, "FlightPhase", 2);
            set_number(entity, "Progress", 0);
            return;
        }
        let Some(to) = target(entity) else { set_number(entity, "Status", 2); return; };
        // Revalidate the actual destination, not just an announcement or saved entity ID.
        let mut valid = false;
        host.change_mod_block_entity(to, &mut |destination| {
            valid = destination.id == DRONE && channel(destination) == channel(entity)
                && text(destination, "FreightImport") == entity.items[CARGO].as_ref().map(|s| s.item.to_string());
        });
        if !valid { set_number(entity, "Status", 28); return; }
        let cargo = entity.items[CARGO].take().unwrap();
        let count = cargo.count;
        entity.items[CARGO] = host.insert_item(to, Direction::Up, cargo);
        let delivered = count - entity.items[CARGO].as_ref().map_or(0, |s| s.count);
        set_number(entity, "Produced", number(entity, "Produced").saturating_add(delivered as i32));
        if entity.items[CARGO].is_some() { set_number(entity, "Status", 3); }
        else {
            set_number(entity, "FlightPhase", 2);
            set_number(entity, "Progress", 0);
            set_number(entity, "Total", number(entity, "Total").saturating_add(1));
        }
        return;
    }
    if entity.items[CARGO].is_some() {
        set_number(entity, "FlightPhase", 2);
        set_number(entity, "Duration", 1);
        set_number(entity, "Progress", 1);
        return;
    }
    let Some(partner) = partner.filter(|other| distance(pos, other.pos) <= f64::from(RANGE)) else {
        set_number(entity, "Status", 28); return;
    };
    let Some(item) = export(entity) else { set_number(entity, "Status", 2); return; };
    // Distinct goods per direction prevent an automatic A -> B -> A feedback loop.
    if partner.export.as_deref() == Some(&item) { set_number(entity, "Status", 28); return; }
    let Some(slot) = (0..3).find(|&i| entity.items[i].as_ref().is_some_and(|s| is_plain(s, &item))) else {
        set_number(entity, "Status", 2); return;
    };
    let length = distance(pos, partner.pos).ceil() as i32;
    let cost = 800 + length * 32; // Paid once for both legs; stalled unloading is free.
    if number(entity, "energy") < cost { set_number(entity, "Status", 5); return; }
    if !clear_step(host, level, pos, partner.pos, 0.0) { set_number(entity, "Status", 22); return; }
    let count = entity.items[slot].as_ref().unwrap().count.min(16);
    entity.items[CARGO] = Some(ItemStack::new(&item, count));
    consume(&mut entity.items[slot], count);
    set_number(entity, "energy", number(entity, "energy") - cost);
    entity.nbt.insert(nbt_key("FlightTarget").clone(), Tag::IntArray(partner.pos.to_vec()));
    entity.nbt.insert(nbt_key("FlightDelta").clone(), Tag::IntArray((0..3).map(|i| partner.pos[i] - pos[i]).collect()));
    set_number(entity, "FlightPhase", 1);
    set_number(entity, "Progress", 0);
    set_number(entity, "Duration", 20 + length * 2);
    set_number(entity, "Status", 1);
}

pub(super) fn screen_slots(block: &str) -> Option<Vec<Option<[f32; 2]>>> {
    owns(block).then(|| {
        let mut slots = vec![Some([26.0, 42.0]), Some([44.0, 42.0]), Some([62.0, 42.0]),
            Some([116.0, 42.0]), Some([134.0, 42.0]), Some([152.0, 42.0])];
        if block == DRONE { slots.push(Some([89.0, 42.0])); }
        slots
    })
}
pub(super) fn panel_lines(block: &str, data: &[i32]) -> Option<Vec<String>> {
    owns(block).then(|| vec![
        Text::tr("veio.freight.channel", &[&data.get(12).copied().unwrap_or(0)]).plain(),
        Text::tr(if block == RAIL { "veio.freight.rail" } else { match data.get(8).copied().unwrap_or(0) {
            1 => "veio.freight.outbound", 2 => "veio.freight.return", _ => "veio.freight.ready",
        } }, &[]).plain(),
    ])
}
pub(super) fn roles(block: &str) -> Option<&'static [&'static str]> {
    match block {
        RAIL => Some(&["dispatch", "", "", "received", "", ""]),
        DRONE => Some(&["dispatch", "", "", "received", "", "", "inflight"]),
        _ => None,
    }
}

/// The visible drone is an animation of the port's durable flight, not a loose
/// entity holding a second copy of the cargo. No target or inventory is teleported.
pub(super) fn draw(host: &dyn Host, event: &mut BlockEntitySubmitEvent<'_>) {
    if let Some(item) = export(event.entity) {
        event.draw.push(); event.draw.translate([0.5, 1.02, 0.5]);
        event.draw.rotate_about([1.0, 0.0, 0.0], 90.0, [0.0; 3]); event.draw.scale([0.35; 3]);
        host.draw_item(event.draw, &ItemStack::new(&item, 1), horizon_client::mod_api::DisplayContext::Fixed, None, 0);
        event.draw.pop();
    }
    if event.block != DRONE { event.draw.keep(); return; }
    let phase = number(event.entity, "FlightPhase");
    let to = target(event.entity).unwrap_or(event.pos);
    let duration = number(event.entity, "Duration").max(1) as f32;
    let progress = number(event.entity, "Progress") as f32;
    let fraction = if phase == 1 { (progress / duration).min(1.0) }
        else if phase == 2 { 1.0 - (progress / duration).min(1.0) } else { 0.0 };
    let faces = [Some(CubeFace { sprite: "minecraft:block/iron_block", uv: None }); 6];
    event.draw.push();
    event.draw.translate([
        (to[0] - event.pos[0]) as f32 * fraction,
        (to[1] - event.pos[1]) as f32 * fraction + if phase > 0 { 2.0 } else { 0.0 },
        (to[2] - event.pos[2]) as f32 * fraction,
    ]);
    host.draw_cube(event.draw, [5.0, 17.0, 5.0], [11.0, 20.0, 11.0], faces, 0x47b9ce);
    for (x, z) in [(2.0, 2.0), (11.0, 2.0), (2.0, 11.0), (11.0, 11.0)] {
        host.draw_cube(event.draw, [x, 20.0, z], [x + 3.0, 20.5, z + 3.0], faces, 0x454c55);
    }
    if let Some(cargo) = event.entity.items.get(CARGO).and_then(Option::as_ref) {
        event.draw.push(); event.draw.translate([0.5, 0.95, 0.5]); event.draw.scale([0.3; 3]);
        host.draw_item(event.draw, cargo, horizon_client::mod_api::DisplayContext::Fixed, None, 0);
        event.draw.pop();
    }
    event.draw.pop();
    if phase == 0 { event.draw.keep(); }
}

pub(super) fn bounds(entity: &ModBlockEntity, bounds: &mut [f32; 6]) {
    bounds[4] = bounds[4].max(2.5);
    if number(entity, "FlightPhase") == 0 { return; }
    bounds[4] = bounds[4].max(3.5);
    if let Some(Tag::IntArray(delta)) = entity.nbt.get(nbt_key("FlightDelta")) {
        if delta.len() == 3 {
            for i in 0..3 { bounds[i] = bounds[i].min(delta[i] as f32); bounds[i + 3] = bounds[i + 3].max(delta[i] as f32 + 3.5); }
        }
    }
}

#[cfg(test)]
mod tests;
