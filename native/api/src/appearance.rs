//! Geometry-specific indicators and working parts; all anchors come from the
//! same source as the block models and their oriented collision shapes.
use super::*;
use horizon_client::mod_api::{BlockEntitySubmitEvent, CubeFace};
mod data;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Motion { None, Drill, Press, Saw, Roller, Turntable, Motor, Heat, Scanner, Mixer, Seeder, Mill, Harvester, Feed }
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
    let active = motion::active(event.block, event.entity);
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
    let time = event.draw.game_time.wrapping_add(i64::from(event.pos[0])*7 + i64::from(event.pos[2])*11);
    let stroke = if active { motion::sample(&motion::STROKE, time, event.draw.partial, 32) } else { 1.0 };
    let sweep = if active { motion::sample(&motion::SWEEP, time, event.draw.partial, 48) } else { 0.0 };
    match profile.motion {
        Motion::None => {}
        Motion::Press => {
            let head = 7.3 + stroke * 3.7;
            host.draw_cube(event.draw, [7.0, head+2.0, 7.0], [9.0, 14.0, 9.0], MATERIAL, 0x8d9698);
            host.draw_cube(event.draw, [5.0, head, 5.0], [11.0, head+2.0, 11.0], MATERIAL, 0x555d61);
        }
        Motion::Drill => {
            event.draw.spin([0.0, 1.0, 0.0], pivot, if active { match event.block { DRILL_MK2 => 12.0, production::DRILL_MK3 => 15.0, _ => 9.0 } } else { 0.0 }, 0.0);
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
            let [x,y,z] = profile.center;
            for (offset,direction) in [(-1.2,1.0),(1.2,-1.0)] {
                let axis=[1.0,0.0,0.0]; let center=[x/16.0,y/16.0,(z+offset)/16.0];
                event.draw.spin(axis,center,if active { direction*10.0 } else { 0.0 },0.0);
                host.draw_cube(event.draw,[x-4.0,y-0.7,z+offset-0.7],[x+4.0,y+0.7,z+offset+0.7],MATERIAL,0x6a7275);
                for px in [5.0,8.0,11.0] {
                    host.draw_cube(event.draw,[px,y-1.0,z+offset-0.3],[px+0.8,y+1.0,z+offset+0.3],MATERIAL,0x959d9f);
                }
                event.draw.still();
            }
        }
        Motion::Turntable | Motion::Mill | Motion::Seeder => {
            let [x, y, z] = profile.center;
            event.draw.spin([0.0, 1.0, 0.0], pivot, if active { if event.block == INTEGRATOR { -5.0 } else if profile.motion == Motion::Mill { 3.0 } else { 5.0 } } else { 0.0 }, 0.0);
            host.draw_cube(event.draw, [x-3.0, y, z-3.0], [x+3.0, y+0.6, z+3.0], MATERIAL, 0x6c777b);
            if profile.motion == Motion::Seeder {
                for angle in [0.0, 120.0, 240.0] {
                    event.draw.push(); event.draw.rotate_about([0.0,1.0,0.0], angle, pivot);
                    host.draw_cube(event.draw, [x+1.5,y+0.6,z-0.6], [x+3.0,y+1.2,z+0.6], MATERIAL, 0x9e794b);
                    event.draw.pop();
                }
            }
            host.draw_cube(event.draw, [x-1.0, y+0.6, z-1.0], [x+1.0, y+1.5, z+1.0], MATERIAL, 0xb17c50);
            event.draw.still();
        }
        Motion::Motor => {
            event.draw.spin([0.0,0.0,1.0], pivot, if active { 6.0 } else { 0.0 }, 0.0);
            for angle in [0.0,90.0,180.0,270.0] {
                event.draw.push(); event.draw.rotate_about([0.0,0.0,1.0], angle, pivot);
                host.draw_cube(event.draw,[7.4,8.0,14.3],[8.6,10.8,14.7],MATERIAL,0x9ca6a8);
                event.draw.pop();
            }
            event.draw.still();
        }
        Motion::Heat => {
            let rise = sweep*1.0;
            for y in [4.0,5.5,7.0] {
                host.draw_cube(event.draw,[5.0,y+rise,15.3],[11.0,y+0.7+rise,15.6],MATERIAL,
                    if active { 0xb27643 } else { 0x50585b });
            }
        }
        Motion::Scanner => {
            host.draw_cube(event.draw,[3.0,11.2,7.0],[13.0,11.7,8.0],MATERIAL,0x59656b);
            let x=4.0+6.0*sweep;
            host.draw_cube(event.draw,[x,9.2,8.0],[x+1.5,11.5,10.5],MATERIAL,0x91a1a3);
            host.draw_cube(event.draw,[x,9.15,9.0],[x+1.5,9.4,10.5],MATERIAL,if active { 0x60b4a7 } else { 0x465255 });
        }
        Motion::Mixer => {
            event.draw.spin([0.0,1.0,0.0], pivot, if active { 5.0 } else { 0.0 }, 0.0);
            host.draw_cube(event.draw,[4.7,10.0,7.7],[5.3,12.8,8.3],MATERIAL,0x91999c);
            host.draw_cube(event.draw,[3.5,10.3,7.6],[6.5,10.8,8.4],MATERIAL,0xab8560);
            event.draw.still();
        }
        Motion::Harvester => {
            event.draw.spin([1.0,0.0,0.0], pivot, if active { 7.0 } else { 0.0 }, 0.0);
            host.draw_cube(event.draw,[4.0,7.5,8.5],[12.0,8.5,9.5],MATERIAL,0x66745c);
            for angle in [0.0,90.0,180.0,270.0] {
                event.draw.push(); event.draw.rotate_about([1.0,0.0,0.0], angle,pivot);
                host.draw_cube(event.draw,[4.0,8.6,8.5],[12.0,9.4,9.5],MATERIAL,0x9d9e7c);
                event.draw.pop();
            }
            event.draw.still();
        }
        Motion::Feed => {
            let lift=sweep*1.5;
            for x in [5.0,7.5,10.0] {
                host.draw_cube(event.draw,[x,8.0+lift,10.0],[x+0.7,9.5+lift,10.6],MATERIAL,0x92744c);
            }
        }
    }
    event.draw.pop();
    Some(active && matches!(profile.motion, Motion::Press | Motion::Heat | Motion::Scanner | Motion::Feed))
}

#[cfg(test)]
mod tests;
