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

pub(super) const STATUS_KEYS: [&str; 14] = [
    "idle", "working", "input", "blocked", "ore", "power", "fuel", "sun", "charged", "redstone",
    "crops", "watch", "tier", "damaged",
];

pub(super) fn screen(block: &str) -> Option<NativeScreen> {
    let slots = match block {
        DRILL | DRILL_MK2 | BELT | FAST_BELT | SPLITTER | GENERATOR => vec![Some([89.0, 45.0])],
        PRESS | ASSEMBLER | CUTTER | CRUSHER | GEOTHERMAL => vec![Some([53.0, 45.0]), Some([125.0, 45.0])],
        MERGER => vec![Some([53.0, 45.0]), Some([125.0, 45.0])],
        INTEGRATOR | ALLOY | ELECTRONICS => vec![Some([35.0, 45.0]), Some([71.0, 45.0]), Some([143.0, 45.0])],
        SOLAR | BATTERY | TESLA => vec![None],
        HUB => vec![Some([26.0, 45.0]), Some([62.0, 45.0]), Some([116.0, 45.0]), Some([152.0, 45.0])],
        HARVESTER => vec![Some([26.0, 45.0]), Some([98.0, 45.0]), Some([116.0, 45.0]), Some([134.0, 45.0])],
        _ => return None,
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
    let status = read(0).clamp(0, STATUS_KEYS.len() as i32 - 1) as usize;
    let status_text = Text::tr(&format!("veio.status.{}", STATUS_KEYS[status]), &[]).plain();
    lamp_icon(host, event, [9.0, 24.0], status);
    host.text(event.batch, &status_text, [x + 18.0, y + 24.0], text_color(status), false);
    let progress = read(1).max(0) as f32 / read(2).max(1) as f32;
    // Seta entre a última entrada e a primeira saída; nas demais máquinas, uma barra.
    let first_output = output_slots(event.block).start;
    let positions = &machine_slots;
    if first_output > 0 && first_output < positions.len() && event.block != HARVESTER {
        arrow(host, event, positions[first_output - 1][0] + 16.0, positions[first_output][0], progress);
    } else if !matches!(event.block, BATTERY | CABLE | SOLAR) {
        gauge(host, event, [10.0, 75.0], 174.0, progress, 0xff3fae3f);
    }
    let limit = read(5).max(capacity(event.block)).max(1);
    let energy = read(3).clamp(0, limit);
    if !matches!(event.block, BELT | FAST_BELT | SPLITTER | MERGER) {
        let energy_text = Text::tr("veio.energy", &[&energy, &limit]).plain();
        host.text(event.batch, &energy_text, [x + 10.0, y + 85.0], INK, false);
        gauge(host, event, [10.0, 95.0], 174.0, energy as f32 / limit as f32, 0xffe0a020);
    }
    let lines: Vec<String> = match event.block {
        MERGER => vec![Text::tr("veio.hint.merger", &[]).plain()],
        GENERATOR => vec![Text::tr("veio.hint.fuel", &[]).plain()],
        GEOTHERMAL => vec![Text::tr("veio.hint.lava", &[]).plain()],
        SOLAR => vec![Text::tr("veio.hint.solar", &[]).plain()],
        BATTERY => vec![Text::tr("veio.hint.battery", &[]).plain()],
        BELT | FAST_BELT | SPLITTER => vec![Text::tr("veio.hint.belt", &[]).plain()],
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
    };
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
    let (progress, duration) = match entity.id.as_str() {
        GENERATOR | GEOTHERMAL => (number(entity, "Burn"), number(entity, "BurnTotal").max(1)),
        _ => (number(entity, "Progress"), number(entity, "Duration").max(1)),
    };
    entity.data = vec![number(entity, "Status"), progress, duration,
        number(entity, "energy"), number(entity, "Burn"), capacity(&entity.id), number(entity, "Rate"),
        number(entity, "Utilization"), number(entity, "ContractStage"), number(entity, "Total"),
        number(entity, "Damage"), number(entity, "Module"), number(entity, "LocalSmog")];
    debug_assert_eq!(entity.data.len(), DATA_SLOTS);
    let stack = logistics::display_stack(entity);
    let name = stack.map_or("", |stack| stack.item.as_ref()).to_owned();
    let count = stack.map_or(0, |stack| stack.count);
    entity.nbt.insert("DisplayItem".into(), Tag::String(name.as_str().into()));
    entity.nbt.insert("DisplayCount".into(), Tag::Int(count as i32));
}

/// Cor das luzes no mundo: fortes, para ler de longe.
pub(super) fn lamp_color(status: usize) -> u32 {
    match status {
        1 => 0x5dff5d,
        3 => 0xffaa00,
        2 | 10 | 11 => 0xffe14d,
        4..=6 | 12 | 13 => 0xff4040,
        7 => 0xd9b84a,
        8 => 0x55aaff,
        9 => 0xcc66ff,
        _ => 0x8a8a8a,
    }
}

/// Cor do texto de estado sobre o painel cinza.
pub(super) fn text_color(status: usize) -> u32 {
    match status {
        1 => 0x1e7a1e,
        3 => 0x9a5a00,
        2 | 10 | 11 => 0x6b5a00,
        4..=6 | 12 | 13 => 0xaa1f1f,
        7 => 0x7a6420,
        8 => 0x1f4fa8,
        9 => 0x7a2fa0,
        _ => INK,
    }
}

const FLAT: [Option<CubeFace<'static>>; 6] = [Some(CubeFace { sprite: "veio:gui/material", uv: None }); 6];

pub(super) fn draw(host: &dyn Host, event: &mut BlockEntitySubmitEvent<'_>) {
    if !is_factory_block(event.block) {
        return;
    }
    if power::tier(event.block).is_some() {
        return power::draw(host, event);
    }
    let status = (number(event.entity, "Status") as usize).min(STATUS_KEYS.len() - 1);
    let lamp = lamp_color(status);
    let (lamp_from, lamp_to, lamps) = match event.block {
        CABLE => ([7.0, 4.0, 7.0], [9.0, 4.4, 9.0], 1),
        SOLAR => ([7.0, 4.0, 0.2], [9.0, 5.0, 0.8], 1),
        BELT | FAST_BELT | SPLITTER | MERGER => ([0.2, 4.0, 7.0], [1.8, 4.4, 9.0], 1),
        TESLA => ([7.0, 3.5, 1.6], [9.0, 4.5, 2.0], 4),
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
        let t = ((event.draw.game_time.rem_euclid(speed) as f32
            + event.draw.partial - phase as f32).rem_euclid(speed as f32)) / speed as f32;
        // Ripas em movimento deixam a direção legível mesmo sem carga.
        for index in 0..4 {
            let offset = ((index as f32 / 4.0 + t).fract() - 0.5) * 12.0;
            event.draw.push();
            event.draw.translate([normal[0] * offset / 16.0, 0.0, normal[1] * offset / 16.0]);
            let (from, to) = if normal[0] == 0.0 {
                ([2.0, 3.0, 7.5], [14.0, 3.15, 8.5])
            } else {
                ([7.5, 3.0, 2.0], [8.5, 3.15, 14.0])
            };
            host.draw_cube(event.draw, from, to, FLAT, 0x3a3a3a);
            event.draw.pop();
        }
        if let Some(Tag::String(item)) = event.entity.nbt.get(&"DisplayItem".into()) {
            let name = item.to_string_lossy();
            if !name.is_empty() {
                let distance = if status == 3 { 0.0 } else { t - 0.5 };
                event.draw.push();
                let height = if event.block == MERGER { 0.34 } else { 0.25 };
                event.draw.translate([0.5 + normal[0] * distance, height, 0.5 + normal[1] * distance]);
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
        } else if status == 1 && needs_power(event.block) {
            // Rotor no teto: gira enquanto a máquina trabalha.
            event.draw.spin([0.0, 1.0, 0.0], [0.5, 1.0, 0.5], 6.0, 0.0);
            host.draw_cube(event.draw, [4.0, 16.0, 7.25], [12.0, 16.5, 8.75], FLAT, 0x9a9a9a);
            event.draw.still();
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
