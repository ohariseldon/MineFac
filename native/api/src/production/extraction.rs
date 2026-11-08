//! Extratoras sobre jazidas do mundo, com pureza no estilo Satisfactory.
//!
//! Minérios: a pureza vem do veio conectado em 3D a partir do bloco logo abaixo da extratora
//! (faces, não diagonais; minério comum e de ardósia são a mesma jazida). A busca para em
//! `VEIN_LIMIT` blocos: no máximo 97 leituras, a cada 200 ticks e dentro da cota de terreno.
//! Pedra, areia e demais jazidas infinitas continuam contando o quadrado 3x3 abaixo dela.
//! Referência: frentes/JAZIDAS-CONECTADAS.md e frentes/PROGRESSAO-FASES.md.
use super::*;
use std::collections::{HashSet, VecDeque};

const PURITY_REFRESH: i64 = 200;
/// A classe industrial começa em 16 blocos conectados; contar além disso não muda nada.
pub(crate) const VEIN_LIMIT: usize = 16;
/// Peso na cota de terreno, o mesmo de uma tentativa da silvicultora.
const VEIN_COST: u32 = 24;
const FACES: [[i32; 3]; 6] = [[1, 0, 0], [-1, 0, 0], [0, 1, 0], [0, -1, 0], [0, 0, 1], [0, 0, -1]];

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

/// 1 impura (metade da velocidade), 2 normal, 3 pura (dobro), 4 industrial (quádruplo).
/// O 3x3 conta no máximo 9 blocos: só um veio conectado chega à classe industrial.
pub(crate) fn purity(same_blocks: usize) -> i32 {
    match same_blocks {
        0..=2 => 1,
        3..=6 => 2,
        7..=15 => 3,
        _ => 4,
    }
}

/// Ticks por unidade com a pureza aplicada.
pub(crate) fn duration(base: i32, slow: i32, purity: i32) -> i32 {
    let ticks = base * slow;
    match purity {
        1 => ticks * 2,
        3 => (ticks / 2).max(1),
        // Arredonda para cima: a Mk.III (10 ticks) fica em 3, 400/min, nunca mais que 4x.
        4 => ((ticks + 3) / 4).max(1),
        _ => ticks,
    }
}

/// A jazida industrial puxa o dobro de potência: o FE por unidade fica igual ao da pura.
/// O ganho é vazão por máquina, não energia de graça.
pub(crate) fn rich_draw(draw: i32, purity: i32) -> i32 {
    if purity >= 4 { draw * 2 } else { draw }
}

/// Minérios formam veios; pedra, areia e afins são terreno contínuo e não iniciam busca.
pub(crate) fn is_vein(node: &Node) -> bool {
    node.blocks.iter().any(|block| block.ends_with("_ore") || *block == "minecraft:ancient_debris")
}

/// Diamante, esmeralda e detritos ficam no dobro até medirmos jazidas maiores em partida.
pub(crate) fn class_cap(node: &Node) -> i32 {
    if node.tier >= 2 { 3 } else { 4 }
}

/// Blocos da mesma jazida ligados por faces ao `start`, até `VEIN_LIMIT`.
/// Chunk não carregado lê como ar: a leitura subestima e nunca carrega terreno.
pub(crate) fn connected(reader: &dyn Host, start: [i32; 3], node: &Node) -> usize {
    if !node.blocks.contains(&reader.block(start).as_str()) {
        return 0;
    }
    let mut queue = VecDeque::from([start]);
    let mut seen = HashSet::from([start]);
    let mut count = 1;
    while let Some([x, y, z]) = queue.pop_front() {
        for [dx, dy, dz] in FACES {
            if count >= VEIN_LIMIT {
                return count;
            }
            let next = [x + dx, y + dy, z + dz];
            if seen.insert(next) && node.blocks.contains(&reader.block(next).as_str()) {
                count += 1;
                queue.push_back(next);
            }
        }
    }
    count
}

/// Pureza medida: veio conectado para minérios, 3x3 abaixo para o resto.
fn measure(reader: &dyn Host, [x, y, z]: [i32; 3], node: &Node) -> i32 {
    if is_vein(node) {
        return purity(connected(reader, [x, y - 1, z], node)).min(class_cap(node));
    }
    let same = (-1..=1).flat_map(|dx| (-1..=1).map(move |dz| (dx, dz)))
        .filter(|&(dx, dz)| node.blocks.contains(&reader.block([x + dx, y - 1, z + dz]).as_str()))
        .count();
    purity(same)
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
    if (stale || number(entity, "Info") == 0)
        && (!is_vein(node) || budget::request(budget::Lane::Terrain, host.dimension(), pos, VEIN_COST))
    {
        let reader: &dyn Host = host;
        set_number(entity, "Info", measure(reader, pos, node));
    }
    let draw = rich_draw(draw, number(entity, "Info"));
    let ticks = duration(base, node.slow, number(entity, "Info"));
    let (ticks, draw) = upgrades::apply(ticks, draw, upgrades::effect(entity));
    set_number(entity, "Duration", ticks);
    let (item, amount) = node.output;
    if !can_add(&entity.items[0], item, amount) {
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
    if upgrades::bonus_ready(entity) && can_add(&entity.items[0], item, amount) {
        add(&mut entity.items[0], item, amount);
    }
    finish_cycle(entity);
}
