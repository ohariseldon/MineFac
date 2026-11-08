//! Canos e cabos que se ligam aos vizinhos, como cerca e redstone do vanilla.
//!
//! O modelo é um `multipart`: um núcleo sempre, e um braço por face ligada. Cada face é uma
//! propriedade booleana do bloco (`north`, `up`...), regravada só quando muda. A vizinhança
//! é relida a cada `EVERY` ticks (com fase pela posição, para não acordarem todos juntos) e
//! logo no primeiro tick depois de colocado.
use super::*;

/// Ticks entre releituras dos vizinhos: um cano novo aparece ligado em até 1 s.
const EVERY: i32 = 20;

/// Faces do cano: as seis.
pub(crate) const PIPE_FACES: [Direction; 6] = Direction::ALL;
/// Faces do cabo: ele corre no chão, então liga dos lados e sobe para quem está em cima.
pub(crate) const CABLE_FACES: [Direction; 5] =
    [Direction::Up, Direction::North, Direction::South, Direction::West, Direction::East];

pub(crate) fn name(face: Direction) -> &'static str {
    match face {
        Direction::Down => "down",
        Direction::Up => "up",
        Direction::North => "north",
        Direction::South => "south",
        Direction::West => "west",
        Direction::East => "east",
    }
}

/// Máscara de faces ligadas, na ordem de `faces`.
pub(crate) fn mask(host: &dyn Host, pos: [i32; 3], faces: &[Direction], connects: fn(&str) -> bool) -> i32 {
    faces.iter().enumerate().fold(0, |mask, (bit, face)| {
        let delta = face.normal();
        let target = std::array::from_fn(|axis| pos[axis] + delta[axis]);
        if connects(&host.block(target)) { mask | 1 << bit } else { mask }
    })
}

/// Relê os vizinhos quando é hora e grava só as faces que mudaram.
pub(crate) fn refresh(host: &mut dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity,
    faces: &[Direction], connects: fn(&str) -> bool) {
    // `Joints` guarda a máscara + 1: zero quer dizer "nunca lido" (bloco recém-colocado).
    let stored = number(entity, "Joints") - 1;
    let wait = number(entity, "JointWait");
    if stored >= 0 && wait > 0 {
        set_number(entity, "JointWait", wait - 1);
        return;
    }
    set_number(entity, "JointWait", EVERY - 1 + ((pos[0] * 3 ^ pos[2] * 5 ^ pos[1]) & 7));
    let now = mask(&*host, pos, faces, connects);
    if now == stored {
        return;
    }
    for (bit, face) in faces.iter().enumerate() {
        let on = now & 1 << bit != 0;
        if stored < 0 || (stored & 1 << bit != 0) != on {
            host.set_block_property(pos, name(*face), if on { "true" } else { "false" });
        }
    }
    set_number(entity, "Joints", now + 1);
}

/// Quem o cano alcança: tudo o que guarda água ou lava.
pub(crate) fn pipe_connects(block: &str) -> bool {
    production::water::holds(block)
}

/// Quem o cabo alcança: rede, postes, geradores e consumidores.
pub(crate) fn cable_connects(block: &str) -> bool {
    matches!(block, CABLE | BATTERY) || power::tier(block).is_some() || needs_power(block) || is_generator(block)
}
