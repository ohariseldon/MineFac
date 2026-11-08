//! Geometry-specific indicators and working parts; all anchors come from the
//! same source as the block models and their oriented collision shapes.
use super::*;
use horizon_client::mod_api::{BlockEntitySubmitEvent, CubeFace};
mod data;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Motion { None, Drill, Press, Saw, Roller, Turntable }
#[derive(Clone, Copy)]
struct Profile { lamp: [f32; 6], badge: [f32; 3], motion: Motion, center: [f32; 3], marker: [f32; 3] }

pub(super) fn marker(block: &str) -> Option<[f32; 3]> {
    data::profile(block).map(|profile| profile.marker.map(|v| v / 16.0))
}

const MATERIAL: [Option<CubeFace<'static>>; 6] = [Some(CubeFace { sprite: "veio:gui/material", uv: None }); 6];

/// Returns whether an active CPU-driven stroke needs a new draw each frame.
/// Rotations use Horizon's cached GPU spin and static machines remain cached.
pub(super) fn draw(host: &dyn Host, event: &mut BlockEntitySubmitEvent<'_>) -> Option<bool> {
    let profile = data::profile(event.block)?;
    let active = number(event.entity, "Status") == 1;
    let facing = host.block_state_property(event.state, "facing").unwrap_or("south");
    let yaw = match facing { "west" => -90.0, "north" => 180.0, "east" => 90.0, _ => 0.0 };
    event.draw.push();
    event.draw.rotate_about([0.0, 1.0, 0.0], yaw, [0.5; 3]);
    let [x0, y0, z0, x1, y1, z1] = profile.lamp;
    host.draw_cube(event.draw, [x0, y0, z0], [x1, y1, z1], MATERIAL,
        presentation::lamp_color(number(event.entity, "Status") as usize));
    let module = number(event.entity, "Module");
    if (1..=3).contains(&module) {
        let [x, y, z] = profile.badge;
        host.draw_cube(event.draw, [x, y, z], [x+2.0, y+2.0, z+0.15], MATERIAL,
            [0x3f7fff, 0x3fbf3f, 0xff8a1f][module as usize - 1]);
    }
    let pivot = profile.center.map(|v| v / 16.0);
    match profile.motion {
        Motion::None => {}
        Motion::Press => {
            let time = event.draw.game_time.rem_euclid(10000) as f32 + event.draw.partial;
            let head = if active { 9.0 + (time * 0.2).sin() * 2.0 } else { 11.0 };
            host.draw_cube(event.draw, [7.0, head+2.0, 7.0], [9.0, 14.0, 9.0], MATERIAL, 0x8d9698);
            host.draw_cube(event.draw, [5.0, head, 5.0], [11.0, head+2.0, 11.0], MATERIAL, 0x555d61);
        }
        Motion::Drill => {
            event.draw.spin([0.0, 1.0, 0.0], pivot, if active { 9.0 } else { 0.0 }, 0.0);
            host.draw_cube(event.draw, [7.0, 3.0, 7.0], [9.0, 13.0, 9.0], MATERIAL, 0x969ea1);
            for y in [3.0, 5.0, 7.0] {
                host.draw_cube(event.draw, [5.5, y, 6.5], [10.5, y+0.6, 9.5], MATERIAL, 0x535c60);
            }
            event.draw.still();
        }
        Motion::Saw => {
            event.draw.spin([0.0, 0.0, 1.0], pivot, if active { 16.0 } else { 0.0 }, 0.0);
            host.draw_cube(event.draw, [4.0, 8.0, 7.5], [12.0, 10.0, 8.5], MATERIAL, 0xa8b0b2);
            host.draw_cube(event.draw, [7.0, 5.0, 7.5], [9.0, 13.0, 8.5], MATERIAL, 0xa8b0b2);
            host.draw_cube(event.draw, [5.0, 6.0, 7.5], [11.0, 12.0, 8.5], MATERIAL, 0x879194);
            event.draw.still();
        }
        Motion::Roller => {
            let [x, y, z] = profile.center;
            event.draw.spin([1.0, 0.0, 0.0], pivot, if active { 10.0 } else { 0.0 }, 0.0);
            host.draw_cube(event.draw, [x-4.0, y-1.0, z-1.0], [x+4.0, y+1.0, z+1.0], MATERIAL, 0x6a7275);
            for px in [5.0, 8.0, 11.0] {
                host.draw_cube(event.draw, [px, y-2.0, z-1.5], [px+0.8, y+2.0, z+1.5], MATERIAL, 0x959d9f);
            }
            event.draw.still();
        }
        Motion::Turntable => {
            let [x, y, z] = profile.center;
            event.draw.spin([0.0, 1.0, 0.0], pivot, if active { 5.0 } else { 0.0 }, 0.0);
            host.draw_cube(event.draw, [x-3.0, y, z-3.0], [x+3.0, y+0.6, z+3.0], MATERIAL, 0x6c777b);
            host.draw_cube(event.draw, [x-1.0, y+0.6, z-1.0], [x+1.0, y+1.5, z+1.0], MATERIAL, 0xb17c50);
            event.draw.still();
        }
    }
    event.draw.pop();
    Some(active && profile.motion == Motion::Press)
}
