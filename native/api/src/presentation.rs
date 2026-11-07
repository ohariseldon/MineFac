use super::*;
use horizon_client::mod_api::{
    BlockEntitySubmitEvent, CubeFace, DisplayContext, NativeScreen, ScreenDrawEvent, Text,
};

pub(super) fn screen(block: &str) -> Option<NativeScreen> {
    let slots = match block {
        DRILL | DRILL_MK2 | BELT | FAST_BELT | SPLITTER | GENERATOR => vec![Some([80.0, 45.0])],
        PRESS | ASSEMBLER | CUTTER | CRUSHER | GEOTHERMAL => vec![Some([44.0, 45.0]), Some([116.0, 45.0])],
        INTEGRATOR | ALLOY | ELECTRONICS => vec![Some([26.0, 45.0]), Some([62.0, 45.0]), Some([134.0, 45.0])],
        SOLAR | BATTERY => vec![None],
        HUB => vec![Some([26.0, 45.0]), Some([62.0, 45.0]), Some([116.0, 45.0]), Some([152.0, 45.0])],
        _ => return None,
    };
    Some(NativeScreen {
        width: 194.0,
        height: 236.0,
        slots,
        inventory_y: 154.0,
        inventory_x: 17.0,
        own_labels: true,
        own_background: true,
    })
}

pub(super) fn screen_draw(host: &dyn Host, event: &mut ScreenDrawEvent<'_>) {
    if !is_factory_block(event.block) {
        return;
    }
    let [x, y] = event.origin;
    host.fill(event.batch, [x, y, 194.0, 236.0], 0xff526579);
    host.fill(event.batch, [x + 2.0, y + 2.0, 190.0, 232.0], 0xff141d29);
    host.fill(event.batch, [x + 2.0, y + 2.0, 190.0, 19.0], 0xff263547);
    host.fill(event.batch, [x + 2.0, y + 21.0, 190.0, 1.0], 0xffffbb4a);
    let layout = screen(event.block).expect("factory screen");
    let mut positions: Vec<[f32; 2]> = layout.slots.into_iter().flatten().collect();
    for row in 0..3 {
        for column in 0..9 {
            positions.push([17.0 + column as f32 * 18.0, 154.0 + row as f32 * 18.0]);
        }
    }
    for column in 0..9 {
        positions.push([17.0 + column as f32 * 18.0, 212.0]);
    }
    for [sx, sy] in positions {
        host.fill(event.batch, [x + sx - 1.0, y + sy - 1.0, 18.0, 18.0], 0xff53657a);
        host.fill(event.batch, [x + sx, y + sy, 16.0, 16.0], 0xff0b121d);
    }

    let read = |index| event.data.get(index).copied().unwrap_or(0);
    let title = Text::tr(&format!("block.{}", event.block.replace(':', ".")), &[]).plain();
    host.text(event.batch, &title, [x + 9.0, y + 8.0], 0xdce6ef, false);
    tutorial::draw_labels(host, event);
    if event.block == HUB {
        contracts::draw_screen(host, event);
        host.text(event.batch, &Text::tr("container.inventory", &[]).plain(), [x + 17.0, y + 142.0], 0xdce6ef, false);
        return;
    }
    let status = read(0).clamp(0, 9) as usize;
    let keys = ["idle", "working", "input", "blocked", "ore", "power", "fuel", "sun", "charged", "redstone"];
    let status_text = Text::tr(&format!("veio.status.{}", keys[status]), &[]).plain();
    host.text(event.batch, &status_text, [x + 9.0, y + 24.0], color(status), false);
    let limit = event.data.get(5).copied().unwrap_or_else(|| capacity(event.block)).max(1);
    let energy = read(3).clamp(0, limit);
    let progress = read(1).max(0) as f32 / read(2).max(1) as f32;
    host.fill(event.batch, [x + 10.0, y + 78.0, 174.0, 7.0], 0xff253140);
    host.fill(event.batch, [x + 10.0, y + 78.0, 174.0 * progress.clamp(0.0, 1.0), 7.0], 0xff37bd8e);
    if !matches!(event.block, BELT | FAST_BELT | SPLITTER) {
        host.fill(event.batch, [x + 10.0, y + 104.0, 174.0, 6.0], 0xff253140);
        host.fill(event.batch, [x + 10.0, y + 104.0, 174.0 * energy as f32 / limit as f32, 6.0], 0xffffc857);
        let energy_text = Text::tr("veio.energy", &[&energy, &limit]).plain();
        host.text(event.batch, &energy_text, [x + 10.0, y + 92.0], 0xdce6ef, false);
    }
    let hint = if event.block == GENERATOR { "veio.hint.fuel" }
        else if event.block == GEOTHERMAL { "veio.hint.lava" }
        else if event.block == SOLAR { "veio.hint.solar" }
        else if event.block == BATTERY { "veio.hint.battery" }
        else if matches!(event.block, BELT | FAST_BELT | SPLITTER) { "veio.hint.belt" }
        else { "veio.hint.power" };
    if !needs_power(event.block) {
        host.text(event.batch, &Text::tr(hint, &[]).plain(), [x + 10.0, y + 118.0], 0x91a7bd, false);
    }
    if needs_power(event.block) {
        let rate = Text::tr("veio.metric.rate", &[&read(6)]).plain();
        let utilization = Text::tr("veio.metric.utilization", &[&read(7)]).plain();
        host.text(event.batch, &rate, [x + 10.0, y + 118.0], 0x91a7bd, false);
        host.text(event.batch, &utilization, [x + 10.0, y + 130.0], 0x91a7bd, false);
    }
    host.text(event.batch, &Text::tr("container.inventory", &[]).plain(), [x + 17.0, y + 142.0], 0xdce6ef, false);
}

pub(super) fn publish(entity: &mut ModBlockEntity) {
    let (progress, duration) = match entity.id.as_str() {
        GENERATOR => (number(entity, "Burn"), 400),
        GEOTHERMAL => (number(entity, "Burn"), 800),
        _ => (number(entity, "Progress"), number(entity, "Duration").max(1)),
    };
    entity.data = vec![number(entity, "Status"), progress, duration,
        number(entity, "energy"), number(entity, "Burn"), capacity(&entity.id), number(entity, "Rate"),
        number(entity, "Utilization"), number(entity, "ContractStage")];
    let stack = entity.items.first().and_then(Option::as_ref);
    let name = stack.map_or("", |stack| stack.item.as_ref()).to_owned();
    let count = stack.map_or(0, |stack| stack.count);
    entity.nbt.insert("DisplayItem".into(), Tag::String(name.as_str().into()));
    entity.nbt.insert("DisplayCount".into(), Tag::Int(count as i32));
}

fn color(status: usize) -> u32 {
    match status {
        1 => 0x31b68a,
        3 => 0xe1a64a,
        4..=6 => 0xe15b64,
        7 => 0xb99448,
        8 => 0x61a5e8,
        9 => 0xc48bdf,
        _ => 0x768696,
    }
}

pub(super) fn draw(host: &dyn Host, event: &mut BlockEntitySubmitEvent<'_>) {
    if !is_factory_block(event.block) {
        return;
    }
    let status = number(event.entity, "Status").min(9) as usize;
    let lamp = color(status);
    let faces = [Some(CubeFace { sprite: "veio:gui/material", uv: None }); 6];
    let (lamp_from, lamp_to) = match event.block {
        CABLE => ([7.0, 4.0, 7.0], [9.0, 4.4, 9.0]),
        SOLAR => ([6.5, 4.6, 0.5], [9.5, 5.8, 1.1]),
        BELT | FAST_BELT | SPLITTER => ([0.2, 3.9, 6.5], [1.8, 4.4, 9.5]),
        _ => ([6.5, 9.5, 1.7], [9.5, 11.0, 2.1]),
    };
    let lamps = if matches!(event.block, CABLE | BELT | FAST_BELT | SPLITTER | SOLAR) { 1 } else { 4 };
    for side in 0..lamps {
        event.draw.push();
        event.draw.rotate_about([0.0, 1.0, 0.0], side as f32 * 90.0, [0.5; 3]);
        host.draw_cube(event.draw, lamp_from, lamp_to, faces, lamp);
        event.draw.pop();
    }
    if matches!(event.block, BELT | FAST_BELT | SPLITTER) {
        let facing = host.block_state_property(event.state, "facing").unwrap_or("north");
        let normal = match facing {
            "north" => [0.0, 1.0],
            "south" => [0.0, -1.0],
            "east" => [-1.0, 0.0],
            "west" => [1.0, 0.0],
            _ => [0.0, 0.0],
        };
        let phase = ((event.pos[0] ^ event.pos[2]) & 1) * 4;
        let t = ((event.draw.game_time.rem_euclid(8) as f32
            + event.draw.partial - phase as f32).rem_euclid(8.0)) / 8.0;
        // Moving slats make empty conveyors readable too.
        for index in 0..4 {
            let offset = ((index as f32 / 4.0 + t).fract() - 0.5) * 12.0;
            event.draw.push();
            event.draw.translate([normal[0] * offset / 16.0, 0.0, normal[1] * offset / 16.0]);
            let (from, to) = if normal[0] == 0.0 {
                ([3.0, 3.05, 7.6], [13.0, 3.2, 8.4])
            } else {
                ([7.6, 3.05, 3.0], [8.4, 3.2, 13.0])
            };
            host.draw_cube(event.draw, from, to,
                [Some(CubeFace { sprite: "veio:gui/material", uv: None }); 6], 0x6e7b86);
            event.draw.pop();
        }
        if let Some(Tag::String(item)) = event.entity.nbt.get(&"DisplayItem".into()) {
            let name = item.to_string_lossy();
            if !name.is_empty() {
                let distance = if status == 3 { 0.0 } else { t - 0.5 };
                event.draw.push();
                event.draw.translate([0.5 + normal[0] * distance, 0.28, 0.5 + normal[1] * distance]);
                event.draw.rotate_about([1.0, 0.0, 0.0], 90.0, [0.0; 3]);
                event.draw.scale([0.38; 3]);
                host.draw_item(event.draw, &ItemStack::new(&name, 1), DisplayContext::Fixed, None, 0);
                event.draw.pop();
            }
        }
    } else if status == 1 && needs_power(event.block) {
        event.draw.push();
        let angle = (event.draw.game_time.rem_euclid(60) as f32 + event.draw.partial) * 6.0;
        event.draw.rotate_about([0.0, 1.0, 0.0], angle, [0.5, 0.84, 0.5]);
        host.draw_cube(event.draw, [3.0, 13.0, 7.0], [13.0, 14.0, 9.0],
            [Some(CubeFace { sprite: "veio:gui/material", uv: None }); 6], 0xb8c8d6);
        event.draw.pop();
    }
}
