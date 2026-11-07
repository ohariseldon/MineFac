//! Powered transfer through Horizon's atomic, sided inventory operation.
use super::*;

pub(super) const ID: &str = "veio:inserter";
pub(super) const COST: i32 = 40;
const INTERVAL: i64 = 20;
const SWING: i32 = 10;

pub(super) fn tick(host: &mut dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity, time: i64) {
    set_number(entity, "Duration", SWING);
    if host.neighbor_signal(host.dimension(), pos) > 0 {
        set_number(entity, "Status", 9);
        return;
    }
    let swing = number(entity, "Swing");
    if swing > 0 {
        if number(entity, "Status") == 9 {
            set_number(entity, "MovedAt", stamp(time - i64::from(SWING - swing)));
        }
        set_number(entity, "Swing", swing - 1);
        set_number(entity, "Progress", SWING - swing + 1);
        set_number(entity, "Status", 1);
        return;
    }
    set_number(entity, "Progress", 0);
    if number(entity, "energy") < COST {
        set_number(entity, "Status", 5);
        return;
    }
    set_number(entity, "Status", 0);
    let phase = (i64::from(pos[0]) * 7 + i64::from(pos[1]) * 3 + i64::from(pos[2]) * 11).rem_euclid(INTERVAL);
    if time.rem_euclid(INTERVAL) != phase { return; }
    let Some(output) = logistics::output_direction(host, pos) else { return; };
    let [dx, _, dz] = output.normal();
    let from = [pos[0] - dx, pos[1], pos[2] - dz];
    let to = [pos[0] + dx, pos[1], pos[2] + dz];
    // Refused items stay at the source; no temporary hand stack can be lost.
    let moved = host.move_items(host.dimension(), from, output, to, output.opposite(), None, 1);
    if moved > 0 {
        set_number(entity, "energy", number(entity, "energy") - COST);
        set_number(entity, "Produced", number(entity, "Produced") + moved as i32);
        set_number(entity, "MovedAt", stamp(time));
        set_number(entity, "Swing", SWING);
        set_number(entity, "Status", 1);
    }
}

pub(super) fn draw(host: &dyn Host, event: &mut horizon_client::mod_api::BlockEntitySubmitEvent<'_>) {
    use horizon_client::mod_api::CubeFace;
    let facing = host.block_state_property(event.state, "facing").unwrap_or("south");
    let yaw = match facing { "west" => 90.0, "north" => 180.0, "east" => 270.0, _ => 0.0 };
    let active = number(event.entity, "Status") == 1;
    let paused = number(event.entity, "Status") == 9 && number(event.entity, "Swing") > 0;
    let elapsed = (event.draw.game_time - i64::from(number(event.entity, "MovedAt"))).rem_euclid(i64::from(i32::MAX));
    let t = if active { (elapsed as f32 + event.draw.partial).clamp(0.0, SWING as f32) / SWING as f32 }
        else if paused { number(event.entity, "Progress") as f32 / SWING as f32 }
        else { 0.0 };
    let end = if active || paused { 20.0 - 24.0 * t } else { 8.0 };
    let faces = [Some(CubeFace { sprite: "veio:gui/material", uv: None }); 6];
    event.draw.push();
    event.draw.rotate_about([0.0, 1.0, 0.0], -yaw, [0.5; 3]);
    host.draw_cube(event.draw, [2.0, 4.0, 5.0], [4.0, 4.5, 7.0], faces,
        presentation::lamp_color(number(event.entity, "Status") as usize));
    host.draw_cube(event.draw, [6.8, 8.0, end.min(8.0)], [9.2, 9.4, end.max(8.0) + 1.0], faces, 0xd8aa42);
    host.draw_cube(event.draw, [5.5, 7.0, end - 1.0], [10.5, 9.0, end + 1.0], faces, 0x454c55);
    event.draw.pop();
    if !active { event.draw.keep(); }
}

#[cfg(test)]
mod tests;
