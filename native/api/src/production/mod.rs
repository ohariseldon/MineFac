//! Produção: extração, processamento por receitas, geração de energia, lavouras, silvicultura,
//! criação de animais e comida. Mantido pelo agente da nuvem (veja COORDENACAO.md).
//!
//! `lib.rs` consulta este módulo pelas funções públicas abaixo: `is_machine`, `consumes`,
//! `generates`, `layout`, `outputs`, `accepts`, `tick`, `emission`. A logística não precisa
//! conhecer cada máquina: basta `layout`/`outputs` e o NBT `energy`.
use super::*;

mod data;
pub(super) mod extraction;
pub(super) mod generation;
pub(super) mod crops;
pub(super) mod forestry;
pub(super) mod ranch;
pub(super) mod pantry;

pub(super) const DRILL_MK3: &str = "veio:drill_mk3";
pub(super) const ELECTRIC_FURNACE: &str = "veio:electric_furnace";
pub(super) const MILL: &str = "veio:mill";
pub(super) const KITCHEN: &str = "veio:kitchen";
pub(super) const BIOMASS_GENERATOR: &str = "veio:biomass_generator";
pub(super) const WATER_WHEEL: &str = "veio:water_wheel";
pub(super) const WIND_TURBINE: &str = "veio:wind_turbine";
pub(super) const STEAM_ENGINE: &str = "veio:steam_engine";
pub(super) const PLANTER: &str = "veio:planter";
pub(super) const GROW_LAMP: &str = "veio:grow_lamp";
pub(super) const TREE_FARM: &str = "veio:tree_farm";
pub(super) const RANCH: &str = "veio:ranch";
pub(super) const PANTRY: &str = "veio:pantry";
// Itens criados pela produção; as receitas os citam pelo nome em `tools/production.json`.
#[allow(dead_code)]
pub(super) const FLOUR: &str = "veio:flour";
#[allow(dead_code)]
pub(super) const BIOMASS: &str = "veio:biomass";

// Estados exibidos nos painéis, além dos de `presentation::STATUS_KEYS` 0-13.
pub(super) const NO_WATER: i32 = 14;
pub(super) const NO_WIND: i32 = 15;
pub(super) const NO_SEEDS: i32 = 16;
pub(super) const NO_ANIMALS: i32 = 17;

pub(crate) struct Input {
    pub items: &'static [&'static str],
    pub count: u32,
}

pub(crate) struct Recipe {
    pub machine: &'static str,
    pub inputs: &'static [Input],
    pub output: (&'static str, u32),
    pub ticks: i32,
    pub power: i32,
    /// Receita alternativa: o guia a marca como tal.
    #[allow(dead_code)]
    pub alt: bool,
}

pub(crate) struct Node {
    pub blocks: &'static [&'static str],
    pub output: (&'static str, u32),
    pub tier: i32,
    pub slow: i32,
}

pub(crate) struct Fuel {
    pub items: &'static [&'static str],
    /// Ticks da fornalha vanilla, como referência de valor do combustível.
    pub ticks: i32,
}

pub(super) fn is_machine(block: &str) -> bool {
    matches!(block, DRILL_MK3 | ELECTRIC_FURNACE | MILL | KITCHEN | BIOMASS_GENERATOR | WATER_WHEEL |
        WIND_TURBINE | STEAM_ENGINE | PLANTER | GROW_LAMP | TREE_FARM | RANCH | PANTRY)
}

/// Máquinas que consomem energia: recebem pela rede, param com redstone, mostram métricas.
pub(super) fn consumes(block: &str) -> bool {
    matches!(block, DRILL_MK3 | ELECTRIC_FURNACE | MILL | KITCHEN | PLANTER | GROW_LAMP | TREE_FARM | RANCH)
}

/// Fontes de energia deste módulo (as antigas: gerador, geotérmico e solar).
pub(super) fn generates(block: &str) -> bool {
    matches!(block, BIOMASS_GENERATOR | WATER_WHEEL | WIND_TURBINE | STEAM_ENGINE)
}

/// Fontes que queimam combustível: poluem apenas enquanto há queima.
pub(super) fn burns(block: &str) -> bool {
    matches!(block, GENERATOR | GEOTHERMAL | BIOMASS_GENERATOR | STEAM_ENGINE)
}

/// Entrada de itens, sem alocar o `layout`.
pub(super) fn has_inputs(block: &str) -> bool {
    matches!(block, ELECTRIC_FURNACE | MILL | PANTRY | KITCHEN | BIOMASS_GENERATOR | STEAM_ENGINE |
        PLANTER | TREE_FARM | RANCH)
}

pub(super) fn layout(block: &str) -> Option<(usize, Vec<usize>, Vec<usize>)> {
    Some(match block {
        DRILL_MK3 => (1, vec![], vec![0]),
        ELECTRIC_FURNACE | MILL | PANTRY => (2, vec![0], vec![1]),
        KITCHEN => (3, vec![0, 1], vec![2]),
        BIOMASS_GENERATOR | STEAM_ENGINE | PLANTER => (1, vec![0], vec![]),
        WATER_WHEEL | WIND_TURBINE | GROW_LAMP => (1, vec![], vec![]),
        TREE_FARM => (4, vec![0], vec![1, 2, 3]),
        RANCH => (5, vec![0, 1], vec![2, 3, 4]),
        _ => return None,
    })
}

pub(super) fn outputs(block: &str) -> std::ops::Range<usize> {
    match block {
        DRILL_MK3 => 0..1,
        ELECTRIC_FURNACE | MILL | PANTRY => 1..2,
        KITCHEN => 2..3,
        TREE_FARM => 1..4,
        RANCH => 2..5,
        _ => 0..0,
    }
}

/// Máquinas de receita (as da tabela) e extratoras aceitam módulos.
pub(super) fn accepts_modules(block: &str) -> bool {
    matches!(block, DRILL_MK3) || recipes(block).next().is_some()
}

pub(super) fn recipes(block: &str) -> impl Iterator<Item = &'static Recipe> + '_ {
    data::RECIPES.iter().filter(move |recipe| recipe.machine == block)
}

fn matches_input(input: &Input, stack: &ItemStack) -> bool {
    input.items.iter().any(|item| is_plain(stack, item))
}

/// O que cada slot de entrada aceita. Usado pelos menus e pela logística.
pub(super) fn accepts(block: &str, slot: usize, stack: &ItemStack) -> bool {
    match (block, slot) {
        (GENERATOR, 0) => generation::fuel_ticks(GENERATOR, stack).is_some(),
        (BIOMASS_GENERATOR, 0) => generation::fuel_ticks(BIOMASS_GENERATOR, stack).is_some(),
        (STEAM_ENGINE, 0) => generation::fuel_ticks(GENERATOR, stack).is_some(),
        (GEOTHERMAL, 0) => is_plain(stack, "minecraft:lava_bucket"),
        (HARVESTER, 0) => is_plain(stack, BONE_MEAL),
        (PLANTER, 0) => crops::planted(&stack.item).is_some(),
        (TREE_FARM, 0) => forestry::by_sapling(&stack.item).is_some(),
        (RANCH, 0) => ranch::is_feed(&stack.item),
        (RANCH, 1) => is_plain(stack, "minecraft:bucket"),
        (PANTRY, 0) => pantry::value(&stack.item).is_some(),
        _ => recipes(block).any(|recipe| recipe.inputs.get(slot).is_some_and(|input| matches_input(input, stack))),
    }
}

/// Poluição por segundo de trabalho das máquinas deste módulo.
pub(super) fn emission(block: &str) -> f32 {
    match block {
        DRILL_MK3 => 6.0,
        STEAM_ENGINE => 4.0,
        BIOMASS_GENERATOR | ELECTRIC_FURNACE => 2.0,
        MILL | KITCHEN | TREE_FARM => 1.0,
        _ => 0.0,
    }
}

/// Tick das máquinas de produção que consomem energia. Devolve `false` se não for uma delas.
pub(super) fn tick(host: &mut dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity, time: i64) -> bool {
    if matches!(entity.id.as_str(), DRILL | DRILL_MK2 | DRILL_MK3) {
        extraction::tick(host, pos, entity, time);
        return true;
    }
    if entity.id == HARVESTER {
        farm::tick(host, pos, entity, time);
        return true;
    }
    if entity.id == PLANTER {
        crops::plant(host, pos, entity, time);
        return true;
    }
    if entity.id == GROW_LAMP {
        crops::grow(host, pos, entity, time);
        return true;
    }
    if entity.id == TREE_FARM {
        forestry::tick(host, pos, entity, time);
        return true;
    }
    if entity.id == RANCH {
        ranch::tick(host, pos, entity, time);
        return true;
    }
    if entity.id == PANTRY {
        pantry::tick(host, pos, entity, time);
        return true;
    }
    if recipes(&entity.id).next().is_some() {
        process(entity);
        return true;
    }
    false
}

/// Primeira receita da máquina cujas entradas estão todas nos slots.
pub(super) fn find(block: &str, slots: &[Option<ItemStack>]) -> Option<&'static Recipe> {
    recipes(block).find(|recipe| {
        recipe.inputs.iter().enumerate().all(|(slot, input)| {
            slots.get(slot).and_then(Option::as_ref)
                .is_some_and(|stack| matches_input(input, stack) && stack.count >= input.count)
        })
    })
}

/// Motor comum das máquinas de receita: espera ingredientes, espaço e energia; aplica módulos.
pub(super) fn process(entity: &mut ModBlockEntity) {
    let output_slot = output_slots(&entity.id).start;
    let Some(recipe) = find(&entity.id, &entity.items) else {
        set_number(entity, "Status", 2);
        return;
    };
    let (item, amount) = recipe.output;
    let (duration, draw) = upgrades::apply(recipe.ticks, recipe.power, upgrades::effect(entity));
    set_number(entity, "Duration", duration);
    let changed = with_nbt_key("Recipe", |key| {
        !matches!(entity.nbt.get(key), Some(Tag::String(old)) if java_eq(old, item))
    });
    if changed {
        with_nbt_key("Recipe", |key| {
            entity.nbt.insert(key.clone(), Tag::String(item.into()));
        });
        set_number(entity, "Progress", 0);
    }
    let produced = upgrades::output_amount(entity, item, amount);
    if !can_add(&entity.items[output_slot], item, produced) {
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
    if progress < duration {
        set_number(entity, "Progress", progress);
        return;
    }
    for (slot, input) in recipe.inputs.iter().enumerate() {
        consume(&mut entity.items[slot], input.count);
    }
    add(&mut entity.items[output_slot], item, amount);
    if upgrades::bonus_ready(entity, item) {
        add(&mut entity.items[output_slot], item, amount);
    }
    finish_cycle(entity, produced);
}

/// Contadores comuns ao terminar um ciclo de produção.
pub(super) fn finish_cycle(entity: &mut ModBlockEntity, produced: u32) {
    set_number(entity, "Produced", number(entity, "Produced").saturating_add(produced as i32));
    set_number(entity, "Total", number(entity, "Total").saturating_add(1));
    set_number(entity, "Progress", 0);
}

/// Fontes contínuas e a lâmpada não têm ciclo: o painel omite a barra de progresso.
pub(super) fn shows_progress(block: &str) -> bool {
    !matches!(block, WATER_WHEEL | WIND_TURBINE | GROW_LAMP)
}

/// Posição dos slots no painel (largura 194, como os demais painéis do Veio).
pub(super) fn screen_slots(block: &str) -> Option<Vec<Option<[f32; 2]>>> {
    Some(match block {
        DRILL_MK3 | BIOMASS_GENERATOR | STEAM_ENGINE | PLANTER => vec![Some([89.0, 45.0])],
        ELECTRIC_FURNACE | MILL | PANTRY => vec![Some([53.0, 45.0]), Some([125.0, 45.0])],
        KITCHEN => vec![Some([35.0, 45.0]), Some([71.0, 45.0]), Some([143.0, 45.0])],
        WATER_WHEEL | WIND_TURBINE | GROW_LAMP => vec![None],
        TREE_FARM => vec![Some([26.0, 45.0]), Some([98.0, 45.0]), Some([116.0, 45.0]), Some([134.0, 45.0])],
        RANCH => vec![Some([17.0, 45.0]), Some([39.0, 45.0]), Some([116.0, 45.0]), Some([134.0, 45.0]), Some([152.0, 45.0])],
        _ => return None,
    })
}

/// Rótulos dos slots no painel; "" pula o rótulo (saídas agrupadas).
pub(super) fn roles(block: &str) -> Option<&'static [&'static str]> {
    Some(match block {
        DRILL_MK3 => &["output"],
        BIOMASS_GENERATOR | STEAM_ENGINE => &["fuel"],
        PLANTER => &["seeds"],
        KITCHEN => &["input1", "input2", "output"],
        TREE_FARM => &["saplings", "output", "", ""],
        RANCH => &["feed", "bucket", "output", "", ""],
        PANTRY => &["food", "emeralds"],
        WATER_WHEEL | WIND_TURBINE | GROW_LAMP => &[],
        _ => return None,
    })
}

/// Linhas de informação no painel. `info` é o valor que a máquina publica no slot 12.
pub(super) fn panel_lines(block: &str, data: &[i32]) -> Option<Vec<String>> {
    use horizon_client::mod_api::Text;
    let read = |index: usize| data.get(index).copied().unwrap_or(0);
    let info = read(12);
    let tr = |key: &str, args: &[&dyn std::fmt::Display]| Text::tr(key, args).plain();
    Some(match block {
        DRILL | DRILL_MK2 | DRILL_MK3 => vec![
            tr("veio.metric.rate", &[&read(6)]),
            tr(&format!("veio.purity.{}", ["unknown", "impure", "normal", "pure"][info.clamp(0, 3) as usize]), &[]),
        ],
        BIOMASS_GENERATOR => vec![tr("veio.hint.biomass", &[])],
        STEAM_ENGINE => vec![tr("veio.hint.steam", &[])],
        WATER_WHEEL => vec![tr("veio.metric.output", &[&info]), tr("veio.hint.water_wheel", &[])],
        WIND_TURBINE => vec![tr("veio.metric.output", &[&info]), tr("veio.hint.wind", &[])],
        SOLAR => vec![tr("veio.metric.output", &[&info]), tr("veio.hint.solar", &[])],
        PLANTER => vec![tr("veio.metric.planted", &[&read(9)]), tr("veio.hint.planter", &[])],
        GROW_LAMP => vec![tr("veio.metric.grown", &[&read(9)]), tr("veio.hint.grow_lamp", &[])],
        TREE_FARM => vec![tr("veio.metric.trees", &[&read(9)]), tr("veio.hint.tree_farm", &[])],
        RANCH => vec![tr("veio.metric.animals", &[&info]), tr("veio.hint.ranch", &[])],
        PANTRY => vec![
            tr("veio.metric.credit", &[&read(1), &pantry::PRICE]),
            if info >= pantry::VILLAGE_SIZE { tr("veio.pantry.village", &[&info]) } else { tr("veio.pantry.no_village", &[&info]) },
        ],
        _ => return None,
    })
}

/// Peças animadas próprias (roda d'água, pás do cata-vento). Devolve `true` se desenhou.
pub(super) fn draw(host: &dyn Host, event: &mut horizon_client::mod_api::BlockEntitySubmitEvent<'_>) -> bool {
    generation::draw(host, event)
}

#[cfg(test)]
mod tests;
