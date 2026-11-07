//! Powered transfer through Horizon's atomic, sided inventory operation.
use super::*;

pub(super) const ID: &str = "veio:inserter";
pub(super) const COST: i32 = 40;
pub(super) const FAST: &str = "veio:fast_inserter";
pub(super) const LONG: &str = "veio:long_inserter";

#[derive(Clone, Copy)]
pub(super) struct Tier { pub interval: i64, pub swing: i32, pub cost: i32, pub reach: i32, pub color: u32 }
pub(super) fn tier(block: &str) -> Option<Tier> {
    match block {
        ID => Some(Tier { interval: 20, swing: 10, cost: COST, reach: 1, color: 0xd8aa42 }),
        FAST => Some(Tier { interval: 8, swing: 4, cost: 64, reach: 1, color: 0x3fd0d8 }),
        LONG => Some(Tier { interval: 20, swing: 10, cost: 60, reach: 2, color: 0xd87142 }),
        _ => None,
    }
}
pub(super) fn is_arm(block: &str) -> bool { tier(block).is_some() }

pub(super) fn tick(host: &mut dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity, time: i64) {
    let Some(tier) = tier(&entity.id) else { return; };
    set_number(entity, "Duration", tier.swing);
    if host.neighbor_signal(host.dimension(), pos) > 0 {
        set_number(entity, "Status", 9);
        return;
    }
    let swing = number(entity, "Swing");
    if swing > 0 {
        if number(entity, "Status") == 9 {
            set_number(entity, "MovedAt", stamp(time - i64::from(tier.swing - swing)));
        }
        set_number(entity, "Swing", swing - 1);
        set_number(entity, "Progress", tier.swing - swing + 1);
        set_number(entity, "Status", 1);
        return;
    }
    set_number(entity, "Progress", 0);
    if number(entity, "energy") < tier.cost {
        set_number(entity, "Status", 5);
        return;
    }
    set_number(entity, "Status", 0);
    let phase = (i64::from(pos[0]) * 7 + i64::from(pos[1]) * 3 + i64::from(pos[2]) * 11).rem_euclid(tier.interval);
    if time.rem_euclid(tier.interval) != phase { return; }
    let Some(output) = logistics::output_direction(host, pos) else { return; };
    let [dx, _, dz] = output.normal();
    let from = [pos[0] - dx * tier.reach, pos[1], pos[2] - dz * tier.reach];
    let to = [pos[0] + dx * tier.reach, pos[1], pos[2] + dz * tier.reach];
    // Refused items stay at the source; no temporary hand stack can be lost.
    let filter = logistics_controls::filter(entity);
    let moved = host.move_items(host.dimension(), from, output, to, output.opposite(), filter.as_deref(), 1);
    if moved > 0 {
        set_number(entity, "energy", number(entity, "energy") - tier.cost);
        set_number(entity, "Produced", number(entity, "Produced") + moved as i32);
        set_number(entity, "MovedAt", stamp(time));
        set_number(entity, "Swing", tier.swing);
        set_number(entity, "Status", 1);
    }
}

pub(super) fn draw(host: &dyn Host, event: &mut horizon_client::mod_api::BlockEntitySubmitEvent<'_>) {
    use horizon_client::mod_api::CubeFace;
    let Some(tier) = tier(event.block) else { return; };
    let facing = host.block_state_property(event.state, "facing").unwrap_or("south");
    let yaw = match facing { "west" => 90.0, "north" => 180.0, "east" => 270.0, _ => 0.0 };
    let active = number(event.entity, "Status") == 1;
    let paused = number(event.entity, "Status") == 9 && number(event.entity, "Swing") > 0;
    let elapsed = (event.draw.game_time - i64::from(number(event.entity, "MovedAt"))).rem_euclid(i64::from(i32::MAX));
    let t = if active { (elapsed as f32 + event.draw.partial).clamp(0.0, tier.swing as f32) / tier.swing as f32 }
        else if paused { number(event.entity, "Progress") as f32 / tier.swing as f32 }
        else { 0.0 };
    let end = if active || paused { 8.0 + 12.0 * tier.reach as f32 * (1.0 - 2.0 * t) } else { 8.0 };
    let faces = [Some(CubeFace { sprite: "veio:gui/material", uv: None }); 6];
    event.draw.push();
    event.draw.rotate_about([0.0, 1.0, 0.0], -yaw, [0.5; 3]);
    host.draw_cube(event.draw, [2.0, 4.0, 5.0], [4.0, 4.5, 7.0], faces,
        presentation::lamp_color(number(event.entity, "Status") as usize));
    host.draw_cube(event.draw, [6.8, 8.0, end.min(8.0)], [9.2, 9.4, end.max(8.0) + 1.0], faces, tier.color);
    host.draw_cube(event.draw, [5.5, 7.0, end - 1.0], [10.5, 9.0, end + 1.0], faces, 0x454c55);
    event.draw.pop();
    logistics_controls::draw_filter(host, event);
    if !active { event.draw.keep(); }
}

#[cfg(test)]
mod tests;
