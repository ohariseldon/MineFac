//! Fábricas verticais: aldeões mudam de andar por escadas de mão.
//!
//! A navegação dos mobs não planeja escadas (como no vanilla). Então o posto planeja:
//! 1. `discover` acha colunas de escada de mão no andar do posto (área do carregador), mede até
//!    onde vão e em que alturas dá para sair delas para um piso. Guarda `Ladders` e `Floors`.
//! 2. `route` troca o destino de uma ordem por um passo intermediário: ir até a escada no
//!    próprio andar, depois o topo (ou o pé) da coluna na altura do alvo.
//! 3. O objetivo `Work` sobe ou desce de fato quando o aldeão está na escada (`climb`).
//! O carregador passa a varrer também os andares que suas escadas alcançam.
use super::*;

pub(super) const LADDER: &str = "minecraft:ladder";
/// Uma coluna mede até 24 blocos para cima e para baixo do andar do posto.
const REACH: i32 = 24;
const MAX_COLUMNS: usize = 4;
/// Um piso por coluna é lido no máximo a cada 20 s; a cota de trabalhadores paga as leituras.
pub(super) const EVERY: u64 = 400;
const MAX_FLOORS: usize = 6;
/// Dentro da coluna: o aldeão está na escada e pode subir.
const ON_COLUMN: f64 = 0.45;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct Column { pub x: i32, pub z: i32, pub bottom: i32, pub top: i32 }

impl Column {
    /// Alturas de pé que a coluna liga: do pé da escada a um acima do último degrau.
    fn covers(&self, y: i32) -> bool { (self.bottom..=self.top + 1).contains(&y) }
}

fn open(block: &str) -> bool {
    block.ends_with(":air") || block.contains("torch") || block.contains("carpet") || block.contains("button")
        || block.contains("pressure_plate") || block.ends_with("_sign") || block.ends_with("rail")
}

fn solid(block: &str) -> bool {
    !open(block) && block != LADDER && !matches!(block, "minecraft:water" | "minecraft:lava")
}

/// Alturas em que se sai da coluna para um piso: ao lado, espaço livre para o aldeão (2 de
/// altura) com chão firme embaixo.
pub(super) fn exits(host: &dyn Host, column: Column) -> Vec<i32> {
    (column.bottom..=column.top + 1).filter(|&y| {
        [[1, 0], [-1, 0], [0, 1], [0, -1]].iter().any(|[dx, dz]| {
            let (x, z) = (column.x + dx, column.z + dz);
            open(&host.block([x, y, z])) && open(&host.block([x, y + 1, z])) && solid(&host.block([x, y - 1, z]))
        })
    }).collect()
}

/// Mede a coluna que passa por `at` (uma escada de mão).
pub(super) fn column(host: &dyn Host, [x, y, z]: [i32; 3]) -> Column {
    let mut bottom = y;
    while bottom > y - REACH && host.block([x, bottom - 1, z]) == LADDER { bottom -= 1; }
    let mut top = y;
    while top < y + REACH && host.block([x, top + 1, z]) == LADDER { top += 1; }
    Column { x, z, bottom, top }
}

pub(super) fn columns(entity: &ModBlockEntity) -> Vec<Column> {
    match entity.nbt.get(&"Ladders".into()) {
        Some(Tag::IntArray(values)) => values.chunks_exact(4)
            .map(|v| Column { x: v[0], z: v[1], bottom: v[2], top: v[3] }).collect(),
        _ => Vec::new(),
    }
}

/// Deslocamentos verticais dos andares que o posto atende, o dele primeiro.
pub(super) fn floors(entity: &ModBlockEntity) -> Vec<i32> {
    match entity.nbt.get(&"Floors".into()) {
        Some(Tag::IntArray(values)) if !values.is_empty() => values.clone(),
        _ => vec![0],
    }
}

/// Procura escadas no andar do posto e mede andares. Roda a cada `EVERY` ticks por posto.
pub(super) fn discover(host: &dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity, radius: i32, now: u64) {
    let phase = (pos[0] ^ pos[2]).rem_euclid(EVERY as i32) as u64;
    if !(now + phase).is_multiple_of(EVERY) { return; }
    let side = radius * 2 + 1;
    if !budget::request(budget::Lane::Workforce, host.dimension(), pos, (side * side) as u32) { return; }
    let mut found: Vec<Column> = Vec::new();
    for dz in -radius..=radius {
        for dx in -radius..=radius {
            let at = [pos[0] + dx, pos[1], pos[2] + dz];
            if found.len() < MAX_COLUMNS && host.block(at) == LADDER {
                found.push(column(host, at));
            }
        }
    }
    let mut offsets = vec![0];
    for column in &found {
        for y in exits(host, *column) {
            let offset = y - pos[1];
            if !offsets.contains(&offset) && offsets.len() < MAX_FLOORS { offsets.push(offset); }
        }
    }
    entity.nbt.insert("Ladders".into(), Tag::IntArray(found.iter().flat_map(|c| [c.x, c.z, c.bottom, c.top]).collect()));
    entity.nbt.insert("Floors".into(), Tag::IntArray(offsets));
}

/// Camadas varridas: cada andar e uma camada abaixo e acima dele.
pub(super) fn layers(entity: &ModBlockEntity) -> Vec<i32> {
    let mut layers = Vec::new();
    for floor in floors(entity) {
        for layer in [floor, floor - 1, floor + 1] {
            if !layers.contains(&layer) { layers.push(layer); }
        }
    }
    layers
}

fn flat(a: [f64; 3], x: f64, z: f64) -> f64 { (a[0] - x).hypot(a[2] - z) }

/// Andar (altura de pé) mais próximo de `y` entre os andares do posto.
fn floor_of(entity: &ModBlockEntity, post_y: i32, y: f64) -> i32 {
    floors(entity).into_iter().map(|offset| post_y + offset)
        .min_by(|a, b| (f64::from(*a) - y).abs().total_cmp(&(f64::from(*b) - y).abs()))
        .unwrap_or(y.floor() as i32)
}

/// Passo da viagem até `goal` saindo de `at`. No mesmo andar, o próprio alvo.
pub(super) fn route(entity: &ModBlockEntity, post_y: i32, at: [f64; 3], goal: [f64; 3]) -> [f64; 3] {
    let (from, to) = (floor_of(entity, post_y, at[1]), floor_of(entity, post_y, goal[1]));
    let ladders = columns(entity);
    let in_reach = |c: &Column| c.covers(from) && c.covers(to);
    let cost = |c: &Column| flat(at, f64::from(c.x) + 0.5, f64::from(c.z) + 0.5) + flat(goal, f64::from(c.x) + 0.5, f64::from(c.z) + 0.5);
    let Some(column) = ladders.iter().filter(|c| in_reach(c)).min_by(|a, b| cost(a).total_cmp(&cost(b))) else { return goal };
    let (cx, cz) = (f64::from(column.x) + 0.5, f64::from(column.z) + 0.5);
    if flat(at, cx, cz) <= ON_COLUMN {
        // Na escada: sobe ou desce até a altura de pé do andar do alvo, depois sai.
        return if (at[1] - f64::from(to)).abs() < 0.5 { goal } else { [cx, f64::from(to), cz] };
    }
    if from == to { goal } else { [cx, f64::from(from), cz] }
}

/// Subida ou descida física. O objetivo `Work` chama antes de entregar o passo ao cérebro:
/// com o aldeão na escada e o passo na mesma coluna, mais alto ou mais baixo, dirige o mob.
pub(super) fn climb(m: &mut horizon_client::mobs::Mob, goal: [f64; 3]) -> bool {
    let (dx, dz) = (goal[0] - m.position.x, goal[2] - m.position.z);
    let dy = goal[1] - m.position.y;
    if dx.hypot(dz) > 0.6 || dy.abs() < 0.5 || !m.on_climbable {
        return false;
    }
    m.velocity.x = dx * 0.2;
    m.velocity.z = dz * 0.2;
    // Os valores do vanilla: 0,2 subindo encostado na escada, queda limitada a 0,15 descendo.
    m.velocity.y = if dy > 0.0 { 0.2 } else { -0.15 };
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use horizon_client::mobs::{Ai, Mob, MobType, Vec3};
    use std::collections::HashMap;
    use std::sync::Arc;

    /// Dois andares: piso de pedra em y 63 e y 67, escada em (3, 64..67, 0) passando pelo furo.
    fn tower() -> HashMap<[i32; 3], &'static str> {
        let mut blocks = HashMap::new();
        for x in -6..=6 {
            for z in -6..=6 {
                blocks.insert([x, 63, z], "minecraft:stone");
                if [x, z] != [3, 0] { blocks.insert([x, 67, z], "minecraft:stone"); }
            }
        }
        for y in 64..=68 { blocks.insert([3, y, 0], LADDER); }
        blocks.insert([4, 68, 1], "minecraft:stone"); // parede de apoio, não bloqueia a saída
        blocks
    }

    struct Tower(HashMap<[i32; 3], &'static str>);
    impl Host for Tower {
        fn dimension(&self) -> &'static str { "veio:ladder-test" }
        fn block(&self, pos: [i32; 3]) -> String { self.0.get(&pos).copied().unwrap_or("minecraft:air").into() }
    }

    fn post() -> ModBlockEntity {
        let mut entity = ModBlockEntity::default();
        entity.id = HAULER_POST.into();
        entity
    }

    #[test]
    fn discovery_measures_the_column_and_finds_both_floors() {
        let host = Tower(tower());
        assert_eq!(column(&host, [3, 64, 0]), Column { x: 3, z: 0, bottom: 64, top: 68 });
        assert_eq!(exits(&host, column(&host, [3, 64, 0])), vec![64, 68]);
        let mut entity = post();
        let phase = (0 ^ 0i32).rem_euclid(EVERY as i32) as u64;
        discover(&host, [0, 64, 0], &mut entity, 6, EVERY - phase);
        assert_eq!(columns(&entity), vec![Column { x: 3, z: 0, bottom: 64, top: 68 }]);
        assert_eq!(floors(&entity), vec![0, 4]);
        assert_eq!(layers(&entity), vec![0, -1, 1, 4, 3, 5], "o carregador varre o andar de cima também");
    }

    #[test]
    fn route_walks_to_the_ladder_climbs_and_steps_off() {
        let host = Tower(tower());
        let mut entity = post();
        discover(&host, [0, 64, 0], &mut entity, 6, EVERY);
        let machine = [-2.5, 68.0, 2.5];
        assert_eq!(route(&entity, 64, [0.5, 64.0, 0.5], machine), [3.5, 64.0, 0.5], "vai à escada no próprio andar");
        assert_eq!(route(&entity, 64, [3.5, 65.3, 0.5], machine), [3.5, 68.0, 0.5], "na escada: sobe até o piso de cima");
        assert_eq!(route(&entity, 64, [3.5, 68.0, 0.5], machine), machine, "chegou ao andar: segue ao alvo");
        assert_eq!(route(&entity, 64, [-2.0, 68.0, 2.0], [0.5, 64.0, 0.5]), [3.5, 68.0, 0.5], "descendo, o mesmo caminho");
        assert_eq!(route(&entity, 64, [0.5, 64.0, 0.5], [2.5, 64.0, 2.5]), [2.5, 64.0, 2.5], "mesmo andar: direto");
        assert_eq!(route(&post(), 64, [0.5, 64.0, 0.5], machine), machine, "sem escada conhecida, nada muda");
    }

    #[test]
    fn climbing_pushes_the_villager_along_the_ladder() {
        let mut mob = Mob { id: 1, kind: Arc::new(MobType { id: VILLAGER.into() }), position: Vec3 { x: 3.5, y: 65.0, z: 0.5 },
            velocity: Vec3::default(), on_climbable: true, ai: Ai::default(), extra: Default::default() };
        assert!(climb(&mut mob, [3.5, 68.0, 0.5]));
        assert_eq!(mob.velocity.y, 0.2);
        assert!(climb(&mut mob, [3.5, 64.0, 0.5]));
        assert_eq!(mob.velocity.y, -0.15);
        mob.on_climbable = false;
        assert!(!climb(&mut mob, [3.5, 68.0, 0.5]), "fora da escada, a navegação cuida");
        mob.on_climbable = true;
        assert!(!climb(&mut mob, [8.5, 68.0, 0.5]), "passo longe da coluna não é subida");
    }
}
