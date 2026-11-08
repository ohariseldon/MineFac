use super::*;
use horizon_client::mod_api::{
    BlockEntitySubmitEvent, CubeFace, DisplayContext, NativeScreen, ScreenDrawEvent, Text,
};

// Cores do painel no estilo das telas de baú e fornalha do Minecraft.
const PANEL: u32 = 0xffc6c6c6;
const LIGHT: u32 = 0xffffffff;
const SHADOW: u32 = 0xff555555;
const EDGE: u32 = 0xff000000;
const SLOT: u32 = 0xff8b8b8b;
const SLOT_DARK: u32 = 0xff373737;
/// Texto principal das telas vanilla.
pub(super) const INK: u32 = 0x404040;
pub(super) const SOFT_INK: u32 = 0x5a5a5a;
const WIDTH: f32 = 194.0;
const HEIGHT: f32 = 236.0;

pub(super) const STATUS_KEYS: [&str; 29] = [
    "idle", "working", "input", "blocked", "ore", "power", "fuel", "sun", "charged", "redstone",
    "crops", "watch", "tier", "damaged", "water", "wind", "seeds", "animals",
    // Postos de trabalho (`workforce`).
    "no_worker", "absent", "no_task", "no_food", "path", "walking", "idle_work", "no_wage",
    // Zonas e armazéns.
    "zone", "storing",
    // Lavrador e pastor à noite.
    "resting",
];

pub(super) fn screen(block: &str) -> Option<NativeScreen> {
    let slots = match block {
        DRILL | DRILL_MK2 | BELT | FAST_BELT | SPLITTER | GENERATOR => vec![Some([89.0, 45.0])],
        PRESS | ASSEMBLER | CUTTER | CRUSHER | GEOTHERMAL => vec![Some([53.0, 45.0]), Some([125.0, 45.0])],
        MERGER => vec![Some([53.0, 45.0]), Some([125.0, 45.0])],
        INTEGRATOR | ALLOY | ELECTRONICS => vec![Some([35.0, 45.0]), Some([71.0, 45.0]), Some([143.0, 45.0])],
        SOLAR | BATTERY | TESLA | inserters::ID | inserters::FAST | inserters::LONG => vec![None],
        HUB => vec![Some([26.0, 45.0]), Some([62.0, 45.0]), Some([116.0, 45.0]), Some([152.0, 45.0])],
        HARVESTER => vec![Some([26.0, 45.0]), Some([98.0, 45.0]), Some([116.0, 45.0]), Some([134.0, 45.0])],
        _ if workforce::owns(block) => workforce::screen_slots(block)?,
        _ => production::screen_slots(block)?,
    };
    Some(NativeScreen {
        width: WIDTH,
        height: HEIGHT,
        slots,
        inventory_y: 154.0,
        inventory_x: 17.0,
        own_labels: true,
        own_background: true,
    })
}

/// Moldura em relevo: claro em cima e à esquerda, sombra embaixo e à direita.
fn bevel(host: &dyn Host, event: &mut ScreenDrawEvent<'_>, [x, y, w, h]: [f32; 4], raised: bool, fill: u32) {
    let (top, bottom) = if raised { (LIGHT, SHADOW) } else { (SLOT_DARK, LIGHT) };
    host.fill(event.batch, [x, y, w, h], fill);
    host.fill(event.batch, [x, y, w - 1.0, 1.0], top);
    host.fill(event.batch, [x, y, 1.0, h - 1.0], top);
    host.fill(event.batch, [x + 1.0, y + h - 1.0, w - 1.0, 1.0], bottom);
    host.fill(event.batch, [x + w - 1.0, y + 1.0, 1.0, h - 1.0], bottom);
}

fn slot(host: &dyn Host, event: &mut ScreenDrawEvent<'_>, [sx, sy]: [f32; 2]) {
    let [x, y] = event.origin;
    bevel(host, event, [x + sx - 1.0, y + sy - 1.0, 18.0, 18.0], false, SLOT);
}

/// Barra rebaixada com preenchimento proporcional.
fn gauge(host: &dyn Host, event: &mut ScreenDrawEvent<'_>, at: [f32; 2], width: f32, fraction: f32, color: u32) {
    let [x, y] = event.origin;
    bevel(host, event, [x + at[0], y + at[1], width, 7.0], false, 0xff2b2b2b);
    let inner = (width - 2.0) * fraction.clamp(0.0, 1.0);
    if inner > 0.0 {
        host.fill(event.batch, [x + at[0] + 1.0, y + at[1] + 1.0, inner, 5.0], color);
        host.fill(event.batch, [x + at[0] + 1.0, y + at[1] + 1.0, inner, 1.0], 0x40ffffff);
    }
}

/// Seta de progresso entre entrada e saída, como na fornalha.
fn arrow(host: &dyn Host, event: &mut ScreenDrawEvent<'_>, from: f32, to: f32, fraction: f32) {
    let [x, y] = event.origin;
    let length = (to - from - 6.0).max(6.0);
    let left = x + from + 3.0;
    let top = y + 50.0;
    let done = length * fraction.clamp(0.0, 1.0);
    host.fill(event.batch, [left, top, length - 4.0, 6.0], SLOT);
    for step in 0..4 {
        let s = step as f32;
        host.fill(event.batch, [left + length - 4.0 + s, top - 2.0 + s, 1.0, 10.0 - 2.0 * s], SLOT);
    }
    if done > 0.0 {
        host.fill(event.batch, [left, top, done.min(length - 4.0), 6.0], LIGHT);
        for step in 0..4 {
            let s = step as f32;
            if done > length - 4.0 + s {
                host.fill(event.batch, [left + length - 4.0 + s, top - 2.0 + s, 1.0, 10.0 - 2.0 * s], LIGHT);
            }
        }
    }
}

pub(super) fn screen_draw(host: &dyn Host, event: &mut ScreenDrawEvent<'_>) {
    if !is_factory_block(event.block) {
        return;
    }
    let [x, y] = event.origin;
    // Painel com contorno preto e cantos recortados, como o fundo das telas de contêiner.
    host.fill(event.batch, [x + 1.0, y, WIDTH - 2.0, HEIGHT], EDGE);
    host.fill(event.batch, [x, y + 1.0, WIDTH, HEIGHT - 2.0], EDGE);
    bevel(host, event, [x + 1.0, y + 1.0, WIDTH - 2.0, HEIGHT - 2.0], true, PANEL);
    let layout = screen(event.block).expect("factory screen");
    let machine_slots: Vec<[f32; 2]> = layout.slots.iter().flatten().copied().collect();
    for &position in &machine_slots {
        slot(host, event, position);
    }
    for row in 0..3 {
        for column in 0..9 {
            slot(host, event, [17.0 + column as f32 * 18.0, 154.0 + row as f32 * 18.0]);
        }
    }
    for column in 0..9 {
        slot(host, event, [17.0 + column as f32 * 18.0, 212.0]);
    }

    let read = |index: usize| event.data.get(index).copied().unwrap_or(0);
    let title = Text::tr(&format!("block.{}", event.block.replace(':', ".")), &[]).plain();
    host.text(event.batch, &title, [x + 8.0, y + 7.0], INK, false);
    tutorial::draw_labels(host, event);
    host.text(event.batch, &Text::tr("container.inventory", &[]).plain(), [x + 17.0, y + 143.0], INK, false);
    if event.block == HUB {
        contracts::draw_screen(host, event);
        return;
    }
    if event.block == workforce::CONTROL_CENTER {
        workforce::center::draw_screen(host, event);
        return;
    }
    let status = read(0).clamp(0, STATUS_KEYS.len() as i32 - 1) as usize;
    let status_text = Text::tr(&format!("veio.status.{}", STATUS_KEYS[status]), &[]).plain();
    lamp_icon(host, event, [9.0, 24.0], status);
    host.text(event.batch, &status_text, [x + 18.0, y + 24.0], text_color(status), false);
    let progress = read(1).max(0) as f32 / read(2).max(1) as f32;
    // Seta entre a última entrada e a primeira saída; nas demais máquinas, uma barra.
    let first_output = output_slots(event.block).start;
    let positions = &machine_slots;
    let post = workforce::owns(event.block);
    if first_output > 0 && first_output < positions.len() && event.block != HARVESTER && !post {
        arrow(host, event, positions[first_output - 1][0] + 16.0, positions[first_output][0], progress);
    } else if !matches!(event.block, BATTERY | CABLE | SOLAR) && production::shows_progress(event.block) && !post {
        gauge(host, event, [10.0, 75.0], 174.0, progress, 0xff3fae3f);
    }
    let limit = read(5).max(capacity(event.block)).max(1);
    let energy = read(3).clamp(0, limit);
    if !matches!(event.block, BELT | FAST_BELT | SPLITTER | MERGER) && event.block != production::PANTRY && !post {
        let energy_text = Text::tr("veio.energy", &[&energy, &limit]).plain();
        host.text(event.batch, &energy_text, [x + 10.0, y + 85.0], INK, false);
        gauge(host, event, [10.0, 95.0], 174.0, energy as f32 / limit as f32, 0xffe0a020);
    }
    let lines: Vec<String> = if let Some(lines) = production::panel_lines(event.block, event.data)
        .or_else(|| workforce::panel_lines(event.block, event.data)) { lines } else { match event.block {
        inserters::ID | inserters::FAST | inserters::LONG => vec![
            Text::tr(if read(0) == 0 { "veio.hint.inserter_wait" } else if event.block == inserters::FAST { "veio.hint.fast_inserter" } else if event.block == inserters::LONG { "veio.hint.long_inserter" } else { "veio.hint.inserter" }, &[]).plain(),
            Text::tr("veio.metric.rate", &[&read(6)]).plain(),
            Text::tr(if read(12) & 4 != 0 { "veio.routing.filtered" } else { "veio.routing.any" }, &[]).plain(),
        ],
        SPLITTER | MERGER => vec![
            Text::tr(if read(12) & 4 != 0 { "veio.routing.sorted" } else { ["veio.routing.balanced", "veio.routing.left", "veio.routing.right"][(read(12) & 3).min(2) as usize] }, &[]).plain(),
            Text::tr("veio.metric.rate", &[&read(6)]).plain(),
        ],
        GENERATOR => vec![Text::tr("veio.hint.fuel", &[]).plain()],
        GEOTHERMAL => vec![Text::tr("veio.hint.lava", &[]).plain()],
        SOLAR => vec![Text::tr("veio.hint.solar", &[]).plain()],
        BATTERY => vec![Text::tr("veio.hint.battery", &[]).plain()],
        BELT | FAST_BELT => vec![Text::tr("veio.hint.belt", &[]).plain(), Text::tr("veio.metric.rate", &[&read(6)]).plain()],
        TESLA => vec![
            Text::tr("veio.metric.kills", &[&read(9)]).plain(),
            Text::tr("veio.hint.tesla", &[]).plain(),
            Text::tr("veio.metric.pollution", &[&read(12), &(read(12) * 100 / pollution::RAID as i32)]).plain(),
        ],
        HARVESTER => vec![
            Text::tr("veio.metric.harvests", &[&read(6)]).plain(),
            Text::tr("veio.hint.harvester", &[]).plain(),
        ],
        _ => vec![
            Text::tr("veio.metric.rate", &[&read(6)]).plain(),
            Text::tr("veio.metric.utilization", &[&read(7)]).plain(),
        ],
    } };
    let mut lines = lines;
    // Terceira linha: módulo, emissão e integridade, só quando houver algo a dizer.
    let mut extra = Vec::new();
    let module = read(11);
    if (1..=3).contains(&module) {
        extra.push(Text::tr(&format!("veio.module.{}", ["speed", "efficiency", "productivity"][module as usize - 1]), &[]).plain());
    }
    let emission = pollution::emission(event.block) * upgrades::effect_of(module).pollution * 60.0;
    if emission > 0.0 {
        extra.push(Text::tr("veio.metric.emission", &[&(emission.round() as i32)]).plain());
    }
    if read(10) > 0 {
        extra.push(Text::tr("veio.metric.integrity", &[&(100 - read(10)).max(0)]).plain());
    }
    if !extra.is_empty() && lines.len() < 3 {
        lines.push(extra.join(" · "));
    }
    for (index, line) in lines.iter().enumerate() {
        host.text(event.batch, line, [x + 10.0, y + 108.0 + index as f32 * 11.0], SOFT_INK, false);
    }
}

fn lamp_icon(host: &dyn Host, event: &mut ScreenDrawEvent<'_>, [lx, ly]: [f32; 2], status: usize) {
    let [x, y] = event.origin;
    host.fill(event.batch, [x + lx, y + ly, 6.0, 6.0], EDGE);
    host.fill(event.batch, [x + lx + 1.0, y + ly + 1.0, 4.0, 4.0], 0xff000000 | lamp_color(status));
    host.fill(event.batch, [x + lx + 1.0, y + ly + 1.0, 2.0, 1.0], 0x80ffffff);
}

/// Valores enviados ao painel. Os menus em `data/veio/horizon/menu` declaram o mesmo número
/// (`tools/check_project.py` confere).
pub(super) const DATA_SLOTS: usize = 13;

pub(super) fn publish(entity: &mut ModBlockEntity) {
    // These blocks have no menu; rendering and electricity read their NBT directly.
    if matches!(entity.id.as_str(), CABLE | POLE | STEEL_POLE) { return; }
    logistics_controls::publish(entity);
    let values = if entity.id == workforce::CONTROL_CENTER {
        workforce::center::data(entity)
    } else if workforce::zones::is_marker(&entity.id) {
        workforce::zones::data(entity)
    } else {
        let (progress, duration) = match entity.id.as_str() {
            GENERATOR | GEOTHERMAL => (number(entity, "Burn"), number(entity, "BurnTotal").max(1)),
            _ => (number(entity, "Progress"), number(entity, "Duration").max(1)),
        };
        [number(entity, "Status"), progress, duration,
            number(entity, "energy"), number(entity, "Burn"), capacity(&entity.id), number(entity, "Rate"),
            number(entity, "Utilization"), number(entity, "ContractStage"), number(entity, "Total"),
            number(entity, "Damage"), number(entity, "Module"),
            // Slot 12: poluição local na torre Tesla; na central, a seleção do farol opcional
            // (estágio e contador ficam nos slots 8 e 9); nas outras, a leitura própria
            // (pureza, FE/t gerado, animais, aldeões).
            if entity.id == TESLA { number(entity, "LocalSmog") }
            else if entity.id == HUB { i32::from(contracts::beacon_selected(entity)) }
            else { number(entity, "Info") }]
    };
    debug_assert_eq!(values.len(), DATA_SLOTS);
    if entity.data.len() != DATA_SLOTS {
        entity.data.resize(DATA_SLOTS, 0);
    }
    entity.data.copy_from_slice(&values);
    if !matches!(entity.id.as_str(), BELT | FAST_BELT | SPLITTER | MERGER) {
        return;
    }
    let count = logistics::display_stack(entity).map_or(0, |stack| stack.count) as i32;
    let same = {
        let name = logistics::display_stack(entity).map_or("", |stack| stack.item.as_ref());
        with_nbt_key("DisplayItem", |key| {
            matches!(entity.nbt.get(key), Some(Tag::String(old)) if java_eq(old, name))
        }) && number(entity, "DisplayCount") == count
    };
    if !same {
        let name = logistics::display_stack(entity).map_or(String::new(), |stack| stack.item.as_ref().to_owned());
        with_nbt_key("DisplayItem", |key| {
            entity.nbt.insert(key.clone(), Tag::String(name.as_str().into()));
        });
        set_number(entity, "DisplayCount", count);
    }
}

/// Cor das luzes no mundo: fortes, para ler de longe.
pub(super) fn lamp_color(status: usize) -> u32 {
    match status {
        1 | 23 => 0x5dff5d,
        3 | 22 => 0xffaa00,
        2 | 10 | 11 | 16 | 17 | 18 | 20 | 21 | 24 | 25 => 0xffe14d,
        4..=6 | 12..=15 | 19 => 0xff4040,
        7 => 0xd9b84a,
        8 => 0x55aaff,
        9 => 0xcc66ff,
        _ => 0x8a8a8a,
    }
}

/// Cor do texto de estado sobre o painel cinza.
pub(super) fn text_color(status: usize) -> u32 {
    match status {
        1 | 23 => 0x1e7a1e,
        3 | 22 => 0x9a5a00,
        2 | 10 | 11 | 16 | 17 | 18 | 20 | 21 | 24 | 25 => 0x6b5a00,
        4..=6 | 12..=15 | 19 => 0xaa1f1f,
        7 => 0x7a6420,
        8 | 28 => 0x1f4fa8,
        9 => 0x7a2fa0,
        _ => INK,
    }
}

const FLAT: [Option<CubeFace<'static>>; 6] = [Some(CubeFace { sprite: "veio:gui/material", uv: None }); 6];

fn machine_accent(block: &str) -> u32 {
    match block {
        PRESS | production::ELECTRIC_FURNACE => 0xc46a32,
        ASSEMBLER | ELECTRONICS => 0x3f8f9a,
        CUTTER | CRUSHER => 0xc0c4c8,
        INTEGRATOR | ALLOY => 0xd8a04a,
        GENERATOR | production::STEAM_ENGINE => 0x6a4030,
        _ => 0x8d93a0,
    }
}

fn output_mark(facing: &str) -> ([f32; 3], [f32; 3]) {
    match facing {
        "north" => ([6.0, 16.0, 0.0], [10.0, 16.7, 1.6]),
        "west" => ([0.0, 16.0, 6.0], [1.6, 16.7, 10.0]),
        "east" => ([14.4, 16.0, 6.0], [16.0, 16.7, 10.0]),
        _ => ([6.0, 16.0, 14.4], [10.0, 16.7, 16.0]),
    }
}

pub(super) fn draw(host: &dyn Host, event: &mut BlockEntitySubmitEvent<'_>) {
    if !is_factory_block(event.block) {
        return;
    }
    if !inserters::is_arm(event.block) { logistics_controls::draw_filter(host, event); }
    if power::tier(event.block).is_some() {
        return power::draw(host, event);
    }
    if inserters::is_arm(event.block) { return inserters::draw(host, event); }
    if production::draw(host, event) || workforce::zones::draw(host, event) {
        return;
    }
    let status = (number(event.entity, "Status") as usize).min(STATUS_KEYS.len() - 1);
    let lamp = lamp_color(status);
    let (lamp_from, lamp_to, lamps) = match event.block {
        CABLE => ([7.0, 4.0, 7.0], [9.0, 4.4, 9.0], 1),
        SOLAR => ([7.0, 4.0, 0.2], [9.0, 5.0, 0.8], 1),
        BELT | FAST_BELT | SPLITTER | MERGER => ([0.2, 4.0, 7.0], [1.8, 4.4, 9.0], 1),
        TESLA => ([7.0, 3.5, 1.6], [9.0, 4.5, 2.0], 4),
        production::GROW_LAMP => ([7.0, 13.0, 3.6], [9.0, 14.0, 4.0], 4),
        _ => ([7.0, 11.0, 0.6], [9.0, 12.5, 1.0], 4),
    };
    for side in 0..lamps {
        event.draw.push();
        event.draw.rotate_about([0.0, 1.0, 0.0], side as f32 * 90.0, [0.5; 3]);
        host.draw_cube(event.draw, lamp_from, lamp_to, FLAT, lamp);
        event.draw.pop();
    }
    if matches!(event.block, BELT | FAST_BELT | SPLITTER | MERGER) {
        let facing = host.block_state_property(event.state, "facing").unwrap_or("north");
        let normal = match facing {
            "north" => [0.0, 1.0],
            "south" => [0.0, -1.0],
            "east" => [-1.0, 0.0],
            "west" => [1.0, 0.0],
            _ => [0.0, 0.0],
        };
        let phase = ((event.pos[0] ^ event.pos[2]) & 1) * 4;
        let speed = if event.block == FAST_BELT { 4 } else { 8 };
        let t = if matches!(status, 3 | 9) { 0.5 } else { ((event.draw.game_time.rem_euclid(speed) as f32
            + event.draw.partial - phase as f32).rem_euclid(speed as f32)) / speed as f32 };
        let path = host.block_state_property(event.state, "path").unwrap_or("straight");
        // Slats and cargo follow the same centerline as the static geometry.
        for index in 0..4 {
            let u = (index as f32 / 4.0 + t).fract();
            let point = logistics::orient(logistics::path_point(path, u), facing);
            let next = logistics::orient(logistics::path_point(path, (u + 0.01).min(1.0)), facing);
            let across_z = (next[0] - point[0]).abs() > (next[2] - point[2]).abs();
            let [px, py, pz] = point.map(|n| n * 16.0);
            let (from, to) = if across_z {
                ([px - 0.4, py + 0.04, pz - 5.0], [px + 0.4, py + 0.18, pz + 5.0])
            } else {
                ([px - 5.0, py + 0.04, pz - 0.4], [px + 5.0, py + 0.18, pz + 0.4])
            };
            host.draw_cube(event.draw, from, to, FLAT, 0x3a3a3a);
        }
        if let Some(Tag::String(item)) = event.entity.nbt.get(&"DisplayItem".into()) {
            let name = item.to_string_lossy();
            if !name.is_empty() {
                let u = if matches!(status, 3 | 9) { 0.5 } else { t };
                event.draw.push();
                let point = if logistics::is_belt(event.block) {
                    let mut point = logistics::orient(logistics::path_point(path, u), facing);
                    point[1] += 0.0625;
                    point
                } else {
                    [0.5 + normal[0] * (u - 0.5), if event.block == MERGER { 0.34 } else { 0.25 }, 0.5 + normal[1] * (u - 0.5)]
                };
                event.draw.translate(point);
                event.draw.rotate_about([1.0, 0.0, 0.0], 90.0, [0.0; 3]);
                event.draw.scale([0.4; 3]);
                host.draw_item(event.draw, &ItemStack::new(&name, 1), DisplayContext::Fixed, None, 0);
                event.draw.pop();
            }
        }
    } else {
        if event.block == TESLA {
            // Anel em volta da bobina: gira devagar vigiando e rápido ao disparar.
            let rate = match status { 1 => 18.0, 11 => 4.0, _ => 0.0 };
            event.draw.spin([0.0, 1.0, 0.0], [0.5, 0.5, 0.5], rate, 0.0);
            for side in 0..4 {
                event.draw.push();
                event.draw.rotate_about([0.0, 1.0, 0.0], side as f32 * 90.0, [0.5; 3]);
                host.draw_cube(event.draw, [4.0, 9.5, 3.5], [12.0, 10.5, 4.5], FLAT,
                    if status == 1 { 0x9fe8ff } else { 0xb87333 });
                event.draw.pop();
            }
            event.draw.still();
        } else if needs_power(event.block) && event.block != production::GROW_LAMP {
            // Cruz no teto: parada quando ociosa, girando enquanto trabalha. A marca na borda
            // aponta a saída, para a linha se ler de cima.
            let working = status == 1;
            let color = machine_accent(event.block);
            event.draw.spin([0.0, 1.0, 0.0], [0.5, 1.0, 0.5], if working { 8.0 } else { 0.0 }, 0.0);
            host.draw_cube(event.draw, [6.4, 16.0, 6.4], [9.6, 17.4, 9.6], FLAT, 0x2e2e2e);
            host.draw_cube(event.draw, [1.5, 16.5, 7.15], [14.5, 17.15, 8.85], FLAT, color);
            host.draw_cube(event.draw, [7.15, 16.5, 1.5], [8.85, 17.15, 14.5], FLAT, color);
            event.draw.still();
            let facing = host.block_state_property(event.state, "facing").unwrap_or("south");
            let (from, to) = output_mark(facing);
            host.draw_cube(event.draw, from, to, FLAT, if working { color } else { 0x6a6a6a });
        }
        let module = number(event.entity, "Module");
        if module > 0 {
            // Selo colorido no canto da tampa: azul velocidade, verde eficiência, laranja produtividade.
            let color = [0x3f7fff, 0x3fbf3f, 0xff8a1f][(module as usize - 1).min(2)];
            host.draw_cube(event.draw, [12.5, 16.0, 12.5], [14.5, 16.6, 14.5], FLAT, color);
        }
        // Sem nada que dependa do tempo: o jogo reaproveita este desenho até o estado mudar.
        event.draw.keep();
    }
}
