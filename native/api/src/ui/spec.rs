//! O que cada tela é: família (cor e setor), tipo de palco, ícone da ação e o item-fantasma de
//! cada slot de entrada. Os textos ficam em `veio.ui.tag.<bloco>` (o que faz, uma linha) e
//! `veio.ui.how.<bloco>` (como e por quê, até duas linhas), gerados por
//! `tools/build_ui_text.py`.
use super::icons;
use super::Icon;
use super::super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Family {
    Industry,
    Extraction,
    Energy,
    Logistics,
    Farming,
    Food,
    Storage,
    Workforce,
    Society,
    Trade,
    Management,
    Defense,
}

impl Family {
    /// Cor da família: fio do cabeçalho, moldura do ícone, enchimentos e o fundo do palco.
    pub(crate) fn accent(self) -> u32 {
        match self {
            Family::Industry => 0xc8662a,
            Family::Extraction => 0x9a7440,
            Family::Energy => 0xd8a018,
            Family::Logistics => 0x3a72c8,
            Family::Farming => 0x4e9a2e,
            Family::Food => 0xc08a2a,
            Family::Storage => 0x9a6a34,
            Family::Workforce => 0x2a8a7a,
            Family::Society => 0xb04a36,
            Family::Trade => 0x2a7ea0,
            Family::Management => 0x6a50b8,
            Family::Defense => 0xa82a2a,
        }
    }

    pub(crate) fn key(self) -> &'static str {
        match self {
            Family::Industry => "veio.ui.family.industry",
            Family::Extraction => "veio.ui.family.extraction",
            Family::Energy => "veio.ui.family.energy",
            Family::Logistics => "veio.ui.family.logistics",
            Family::Farming => "veio.ui.family.farming",
            Family::Food => "veio.ui.family.food",
            Family::Storage => "veio.ui.family.storage",
            Family::Workforce => "veio.ui.family.workforce",
            Family::Society => "veio.ui.family.society",
            Family::Trade => "veio.ui.family.trade",
            Family::Management => "veio.ui.family.management",
            Family::Defense => "veio.ui.family.defense",
        }
    }
}

/// Como o palco da tela é montado.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Kind {
    /// Receitas: entradas, seta e o resultado.
    Process,
    /// Extratora: a jazida embaixo e a saída.
    Extract,
    /// Queima combustível para gerar energia.
    Burner,
    /// Sol, vento ou água: gera sem combustível.
    Passive,
    Battery,
    /// Esteiras, túnel e elevador.
    Conveyor,
    /// Divisor e unificador.
    Router,
    /// Braços de transferência.
    Arm,
    /// Plantadeira, colhedora, silvicultura, lâmpada, irrigador e bomba.
    Grow,
    Ranch,
    /// Celeiro e silo: a grade inteira.
    Storage,
    /// Postos com trabalhador: comida, insumos e o que ele produz.
    Post,
    /// Marcadores de lavoura e cercado.
    Zone,
    /// Porto e estação de carga do comércio exterior.
    Port,
    /// Terminais de ferrovia, drone e caravana.
    Terminal,
    /// Estações de envio e descarga.
    Station,
    /// Casa, loja e poço.
    Dwelling,
    Pantry,
    Tesla,
    Anchor,
    Lab,
    /// Central de contratos: o pedido entra, a recompensa sai.
    Contract,
    /// Quadro da fábrica: a entrega da fase e o que ela libera.
    Board,
    /// Telas com desenho próprio (centro, prefeitura, lar, monitor).
    Custom,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct Spec {
    pub family: Family,
    pub kind: Kind,
    /// Ícone da ação, sobre a seta do palco.
    pub verb: Icon,
    /// Item-fantasma de cada slot (vazio = sem fantasma), na ordem dos slots do menu.
    pub ghosts: &'static [&'static str],
}

const fn spec(family: Family, kind: Kind, verb: Icon, ghosts: &'static [&'static str]) -> Spec {
    Spec { family, kind, verb, ghosts }
}

const BREAD: &str = "minecraft:bread";
const EMERALD: &str = "minecraft:emerald";
const WHEAT: &str = "minecraft:wheat";

pub(crate) fn of(block: &str) -> Option<Spec> {
    use Family::{Industry, Extraction, Energy, Logistics, Farming, Food, Workforce, Society, Trade, Management, Defense};
    use Kind::*;
    use production::*;
    use workforce::{storage, zones, society, center, home, site};
    Some(match block {
        PRESS => spec(Industry, Process, icons::HAMMER, &[]),
        ASSEMBLER => spec(Industry, Process, icons::GEAR, &[]),
        CUTTER => spec(Industry, Process, icons::SAW, &[]),
        CRUSHER => spec(Industry, Process, icons::CRUSH, &[]),
        INTEGRATOR => spec(Industry, Process, icons::WRENCH, &[]),
        ALLOY => spec(Industry, Process, icons::FLAME, &[]),
        ELECTRONICS => spec(Industry, Process, icons::CHIP, &[]),
        ELECTRIC_FURNACE => spec(Industry, Process, icons::FLAME, &[]),
        FABRICATOR => spec(Industry, Process, icons::FACTORY, &[]),
        MILL => spec(Food, Process, icons::CRUSH, &[]),
        KITCHEN => spec(Food, Process, icons::POT, &[]),
        DRILL | DRILL_MK2 | DRILL_MK3 => spec(Extraction, Extract, icons::PICKAXE, &[]),
        GENERATOR => spec(Energy, Burner, icons::FLAME, &["minecraft:coal"]),
        BIOMASS_GENERATOR => spec(Energy, Burner, icons::LEAF, &["veio:biomass"]),
        STEAM_ENGINE => spec(Energy, Burner, icons::DROP, &["minecraft:coal"]),
        GEOTHERMAL => spec(Energy, Burner, icons::LAVA, &["minecraft:lava_bucket"]),
        SOLAR => spec(Energy, Passive, icons::SUN, &[]),
        WIND_TURBINE => spec(Energy, Passive, icons::WIND, &[]),
        WATER_WHEEL | HYDRO_TURBINE => spec(Energy, Passive, icons::DROP, &[]),
        BATTERY => spec(Energy, Battery, icons::BATTERY, &[]),
        BELT | FAST_BELT | EXPRESS_BELT | TUNNEL | lift::LIFT => spec(Logistics, Conveyor, icons::ARROW_FULL, &[]),
        SPLITTER | MERGER => spec(Logistics, Router, icons::ARROW_FULL, &[]),
        inserters::ID | inserters::FAST | inserters::LONG => spec(Logistics, Arm, icons::PICKUP, &[]),
        PLANTER => spec(Farming, Grow, icons::SEEDS, &["minecraft:wheat_seeds"]),
        GROW_LAMP => spec(Farming, Grow, icons::SUN, &[]),
        HARVESTER => spec(Farming, Grow, icons::WHEAT, &[BONE_MEAL]),
        TREE_FARM => spec(Farming, Grow, icons::TREE, &["minecraft:oak_sapling"]),
        SPRINKLER => spec(Farming, Grow, icons::DROP, &[BONE_MEAL]),
        PUMP => spec(Farming, Grow, icons::DROP, &[]),
        RANCH => spec(Farming, Ranch, icons::COW, &[WHEAT, "minecraft:bucket"]),
        PANTRY => spec(Food, Pantry, icons::BREAD, &[BREAD, EMERALD]),
        storage::BARN | storage::SILO => spec(Family::Storage, Kind::Storage, icons::CHEST, &[]),
        workforce::FARM_POST => spec(Workforce, Post, icons::WHEAT, &[BREAD]),
        workforce::HAULER_POST => spec(Workforce, Post, icons::CART, &[BREAD]),
        workforce::FOREMAN_DESK => spec(Workforce, Post, icons::SCROLL, &[BREAD, BREAD, BREAD, EMERALD]),
        workforce::SHEPHERD_POST => spec(Workforce, Post, icons::COW, &[BREAD, WHEAT]),
        workforce::BARRACKS => spec(Defense, Post, icons::SHIELD, &[BREAD, EMERALD]),
        zones::FIELD_MARKER => spec(Farming, Zone, icons::WHEAT, &["minecraft:wheat_seeds"]),
        zones::PEN_MARKER => spec(Farming, Zone, icons::COW, &[WHEAT]),
        society::trade::PORT => spec(Trade, Port, icons::SHIP, &[]),
        society::trade::STATION => spec(Trade, Port, icons::TRAIN, &[]),
        freight::RAIL => spec(Trade, Terminal, icons::TRAIN, &[]),
        freight::DRONE => spec(Trade, Terminal, icons::DRONE, &[]),
        freight::caravan::ID => spec(Trade, Terminal, icons::LLAMA, &[]),
        stations::SENDER | stations::RECEIVER => spec(Logistics, Station, icons::FLAG, &[]),
        society::HOUSE => spec(Society, Dwelling, icons::HOUSE, &[BREAD, BREAD, BREAD]),
        society::SHOP => spec(Society, Dwelling, icons::EMERALD, &[BREAD]),
        society::WELL => spec(Society, Dwelling, icons::DROP, &["minecraft:bucket"]),
        TESLA => spec(Defense, Tesla, icons::BOLT, &[]),
        industry::ANCHOR => spec(Management, Anchor, icons::LOCK, &[]),
        research::LAB => spec(Management, Lab, icons::FLASK, &[]),
        center::CONTROL_CENTER => spec(Management, Custom, icons::CHART, &[]),
        society::TOWN_HALL => spec(Society, Custom, icons::CROWN, &[]),
        home::HOME => spec(Society, Custom, icons::HOUSE, &[]),
        site::SITE => spec(Society, Post, icons::HAMMER, &[BREAD]),
        milestones::BOARD => spec(Management, Board, icons::STAR, &[]),
        HUB => spec(Management, Contract, icons::SCROLL, &[]),
        industry::MONITOR => spec(Management, Custom, icons::CHART, &[]),
        _ => return None,
    })
}

/// Receita ativa da máquina (a última usada; a primeira antes de qualquer ciclo).
pub(crate) fn recipe(block: &str, index: i32) -> Option<&'static production::Recipe> {
    let mut recipes = production::recipes(block);
    let count = production::recipes(block).count();
    if count == 0 { return None; }
    recipes.nth((index.max(0) as usize).min(count - 1))
}

/// Máquinas que usam `item` como ingrediente: o "vai para" da saída.
pub(crate) fn consumers(item: &str) -> Vec<&'static str> {
    let mut machines: Vec<&'static str> = Vec::new();
    for recipe in production::data::RECIPES.iter() {
        if recipe.inputs.iter().any(|input| input.items.contains(&item)) && !machines.contains(&recipe.machine) {
            machines.push(recipe.machine);
        }
    }
    machines
}
