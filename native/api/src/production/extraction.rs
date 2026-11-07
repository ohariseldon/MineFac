//! Extratoras sobre jazidas do mundo, com pureza no estilo Satisfactory.
//!
//! A pureza vem do tamanho do veio: quantos blocos da mesma jazida há no quadrado 3x3 logo
//! abaixo da extratora. É recalculada a cada 200 ticks (nove leituras), não a cada tick.
use super::*;

const PURITY_REFRESH: i64 = 200;

/// Tempo base por unidade, em ticks, e FE por tick de cada nível.
pub(crate) fn tier_of(block: &str) -> Option<(i32, i32, i32)> {
    Some(match block {
        DRILL => (1, 40, 6),
        DRILL_MK2 => (2, 20, 12),
        DRILL_MK3 => (3, 10, 30),
        _ => return None,
    })
}

pub(crate) fn node(block: &str) -> Option<&'static Node> {
    data::NODES.iter().find(|node| node.blocks.contains(&block))
}

/// 1 impura (metade da velocidade), 2 normal, 3 pura (dobro).
pub(crate) fn purity(same_blocks: usize) -> i32 {
    match same_blocks {
        0..=2 => 1,
        3..=6 => 2,
        _ => 3,
    }
}

/// Ticks por unidade com a pureza aplicada.
pub(crate) fn duration(base: i32, slow: i32, purity: i32) -> i32 {
    let ticks = base * slow;
    match purity {
        1 => ticks * 2,
        3 => (ticks / 2).max(1),
        _ => ticks,
    }
}

pub(crate) fn tick(host: &mut dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity, time: i64) {
    let Some((tier, base, draw)) = tier_of(&entity.id) else { return };
    let [x, y, z] = pos;
    let below = host.block([x, y - 1, z]);
    let Some(node) = node(&below) else {
        set_number(entity, "Info", 0);
        set_number(entity, "Status", 4);
        return;
    };
    if node.tier > tier {
        set_number(entity, "Status", 12);
        return;
    }
    let stale = (time + i64::from((x ^ z) & 63)).rem_euclid(PURITY_REFRESH) == 0;
    if stale || number(entity, "Info") == 0 {
        let reader: &dyn Host = host;
        let same = (-1..=1).flat_map(|dx| (-1..=1).map(move |dz| (dx, dz)))
            .filter(|&(dx, dz)| node.blocks.contains(&reader.block([x + dx, y - 1, z + dz]).as_str()))
            .count();
        set_number(entity, "Info", purity(same));
    }
    let ticks = duration(base, node.slow, number(entity, "Info"));
    let (ticks, draw) = upgrades::apply(ticks, draw, upgrades::effect(entity));
    set_number(entity, "Duration", ticks);
    let (item, amount) = node.output;
    let produced = upgrades::output_amount(entity, item, amount);
    if !can_add(&entity.items[0], item, produced) {
        set_number(entity, "Status", 3);
        return;
    }
    let energy = number(entity, "energy");
    if energy < draw {
        set_number(entity, "Status", 5);
        return;
    }
    set_number(entity, "energy", energy - draw);
    set_number(entity, "Status", 1);
    let progress = number(entity, "Progress") + 1;
    if progress < ticks {
        set_number(entity, "Progress", progress);
        return;
    }
    add(&mut entity.items[0], item, amount);
    if upgrades::bonus_ready(entity, item) {
        add(&mut entity.items[0], item, amount);
    }
    finish_cycle(entity, produced);
}
