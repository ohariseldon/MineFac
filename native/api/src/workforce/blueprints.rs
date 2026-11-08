//! Plantas das obras: os prédios prontos, desenhados em `tools/blueprints.py` e gerados em
//! `blueprint_data.rs`.
//!
//! Uma planta é lida na orientação "entrada ao sul". O canteiro (`site.rs`) fica na célula `S` da
//! planta; a planta gira para a entrada ficar virada para onde o canteiro olha (`facing`). As
//! propriedades `facing` e `axis` dos blocos giram junto.
use super::*;

pub(crate) use super::blueprint_data::{BLUEPRINTS, PALETTE, PROPS};

pub(crate) struct Blueprint {
    /// Mesma chave da obra em `projects::PROJECTS`.
    pub key: &'static str,
    /// Largura (x), camadas (y) e profundidade (z).
    pub size: [i32; 3],
    /// Célula do canteiro na planta (x, camada, z).
    pub site: [i32; 3],
    /// Células na ordem de construção: x (5 bits), z (5), camada (5) e paleta (bits 15+).
    pub cells: &'static [u32],
    /// Lugares que o jogador completa com um bloco do Veio.
    pub reserved: &'static [[i32; 3]],
    pub reserved_block: &'static str,
    /// Quem trabalha no prédio pronto (`staff.rs`): cook, teacher, smith, librarian, postman,
    /// innkeeper, scientist; market e council funcionam pelos blocos reservados.
    pub role: &'static str,
    /// Móveis onde o funcionário trabalha (ele anda até cada um).
    pub work: &'static [[i32; 3]],
    /// Assentos dos visitantes (alunos, clientes, leitores, conselheiros).
    pub seats: &'static [[i32; 3]],
    /// Visitantes: 0 nenhum, 1 adultos, 2 crianças.
    pub visitors: u8,
    /// Horário das visitas (hora do dia, 0-24000).
    pub hours: [i64; 2],
    pub max_visitors: usize,
}

/// Bit da obra (`projects`) que esta planta constrói.
pub(crate) fn project_bit(plan: &Blueprint) -> u32 {
    projects::PROJECTS.iter().find(|project| project.key == plan.key).map_or(0, |project| project.bit)
}

/// (x, camada, z) e índice da paleta.
pub(crate) fn decode(cell: u32) -> ([i32; 3], usize) {
    ([(cell & 31) as i32, ((cell >> 10) & 31) as i32, ((cell >> 5) & 31) as i32], (cell >> 15) as usize)
}

/// Giro a partir da orientação do canteiro: 0 sul, 1 oeste, 2 norte, 3 leste.
pub(crate) fn rotation(facing: Option<&str>) -> i32 {
    match facing {
        Some("west") => 1,
        Some("north") => 2,
        Some("east") => 3,
        _ => 0,
    }
}

/// Gira um deslocamento horizontal (dx, dz) da planta para o mundo.
pub(crate) fn turn(dx: i32, dz: i32, rot: i32) -> (i32, i32) {
    match rot.rem_euclid(4) {
        1 => (-dz, dx),
        2 => (-dx, -dz),
        3 => (dz, -dx),
        _ => (dx, dz),
    }
}

/// Posição no mundo de uma célula da planta, com o canteiro em `site`.
pub(crate) fn world(plan: &Blueprint, site: [i32; 3], rot: i32, cell: [i32; 3]) -> [i32; 3] {
    let (dx, dz) = turn(cell[0] - plan.site[0], cell[2] - plan.site[2], rot);
    [site[0] + dx, site[1] + cell[1] - plan.site[1], site[2] + dz]
}

/// Cantos (mínimo, máximo) da planta no mundo.
pub(crate) fn bounds(plan: &Blueprint, site: [i32; 3], rot: i32) -> ([i32; 3], [i32; 3]) {
    let a = world(plan, site, rot, [0, 0, 0]);
    let b = world(plan, site, rot, [plan.size[0] - 1, plan.size[1] - 1, plan.size[2] - 1]);
    ([a[0].min(b[0]), a[1].min(b[1]), a[2].min(b[2])], [a[0].max(b[0]), a[1].max(b[1]), a[2].max(b[2])])
}

const DIRECTIONS: [(&str, i32, i32); 4] = [("north", 0, -1), ("south", 0, 1), ("east", 1, 0), ("west", -1, 0)];

/// Valor de uma propriedade depois do giro.
pub(crate) fn turned(key: &str, value: &str, rot: i32) -> String {
    match key {
        "facing" => DIRECTIONS.iter().find(|(name, _, _)| *name == value).map_or_else(|| value.to_owned(), |(_, dx, dz)| {
            let (x, z) = turn(*dx, *dz, rot);
            DIRECTIONS.iter().find(|(_, a, b)| *a == x && *b == z).map_or(value, |(name, _, _)| name).to_owned()
        }),
        "axis" if rot % 2 == 1 => match value { "x" => "z", "z" => "x", other => other }.to_owned(),
        _ => value.to_owned(),
    }
}

/// Blocos que o construtor não derruba: o que o jogador fez ou guarda (baús, camas, portas,
/// máquinas do Veio...). Terra, pedra, grama, árvores e água saem do caminho.
pub(crate) fn protected(block: &str) -> bool {
    if block.starts_with("veio:") {
        return block != site::SITE;
    }
    const KEEP: [&str; 24] = ["chest", "barrel", "shulker", "_bed", "_door", "sign", "furnace", "smoker", "crafting_table",
        "lectern", "anvil", "bell", "spawner", "beacon", "hopper", "dropper", "dispenser", "brewing", "enchanting",
        "jukebox", "bookshelf", "banner", "respawn_anchor", "lodestone"];
    KEEP.iter().any(|part| block.contains(part))
}
