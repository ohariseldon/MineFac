//! Produção: extração, processamento por receitas, geração de energia, lavouras, silvicultura,
//! criação de animais e comida. Mantido pelo agente da nuvem (veja COORDENACAO.md).
//!
//! `lib.rs` consulta este módulo pelas funções públicas abaixo: `is_machine`, `consumes`,
//! `generates`, `layout`, `outputs`, `accepts`, `tick`, `emission`. A logística não precisa
//! conhecer cada máquina: basta `layout`/`outputs` e o NBT `energy`.
use super::*;

pub(super) mod data;
pub(super) mod extraction;
pub(super) mod generation;
pub(super) mod crops;
pub(super) mod forestry;
pub(super) mod ranch;
pub(super) mod pantry;
pub(crate) mod water;

pub(super) use water::{HYDRO_TURBINE, PIPE, PUMP, SPRINKLER};

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
/// Fabricadora: três entradas, para os produtos de topo (núcleo de automação, kit Mk.II).
pub(super) const FABRICATOR: &str = "veio:fabricator";
/// Fabricante: quatro entradas, saída e subproduto, receita automática ou fixada no painel.
pub(super) const MANUFACTURER: &str = "veio:manufacturer";
/// `Info` do fabricante: receita atual + `PICK_SHIFT` × receita fixada (0 = automática).
pub(super) const PICK_SHIFT: i32 = 64;
/// Botões do painel do fabricante (`menu_button`).
pub(super) const PICK_PREVIOUS: i32 = 1;
pub(super) const PICK_NEXT: i32 = 2;
/// Misturador de tintas: corante + argila viram latas de tinta (e tintas misturadas).
pub(super) const PAINT_MIXER: &str = "veio:paint_mixer";
/// Cabine de pintura: peça + lata de tinta viram peças pintadas (e lã, terracota e vidro).
pub(super) const PAINT_BOOTH: &str = "veio:paint_booth";
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
    /// Nome estável da receita; o painel do fabricante a mostra por `veio.recipe.<id>`.
    pub id: &'static str,
    pub machine: &'static str,
    pub inputs: &'static [Input],
    pub output: (&'static str, u32),
    /// Segunda saída (escória, sobras). Só em máquinas com dois slots de saída: o gerador confere.
    /// Se ela não tiver para onde ir, a máquina para, como no Satisfactory.
    pub byproduct: Option<(&'static str, u32)>,
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
        WIND_TURBINE | STEAM_ENGINE | PLANTER | GROW_LAMP | TREE_FARM | RANCH | PANTRY |
        PUMP | PIPE | SPRINKLER | HYDRO_TURBINE | FABRICATOR | MANUFACTURER | PAINT_MIXER | PAINT_BOOTH)
}

/// Máquinas que consomem energia: recebem pela rede, param com redstone, mostram métricas.
pub(super) fn consumes(block: &str) -> bool {
    matches!(block, DRILL_MK3 | ELECTRIC_FURNACE | MILL | KITCHEN | PLANTER | GROW_LAMP | TREE_FARM | RANCH | PUMP | FABRICATOR | MANUFACTURER
        | PAINT_MIXER | PAINT_BOOTH)
}

/// Fontes de energia deste módulo (as antigas: gerador, geotérmico e solar).
pub(super) fn generates(block: &str) -> bool {
    matches!(block, BIOMASS_GENERATOR | WATER_WHEEL | WIND_TURBINE | STEAM_ENGINE | HYDRO_TURBINE)
}

/// Fontes que queimam combustível: poluem apenas enquanto há queima.
pub(super) fn burns(block: &str) -> bool {
    matches!(block, GENERATOR | GEOTHERMAL | BIOMASS_GENERATOR | STEAM_ENGINE)
}

/// Entrada de itens, sem alocar o `layout`.
pub(super) fn has_inputs(block: &str) -> bool {
    matches!(block, ELECTRIC_FURNACE | MILL | PANTRY | KITCHEN | BIOMASS_GENERATOR | STEAM_ENGINE |
        PLANTER | TREE_FARM | RANCH | SPRINKLER | FABRICATOR | MANUFACTURER | PAINT_MIXER | PAINT_BOOTH)
}

pub(super) fn layout(block: &str) -> Option<(usize, Vec<usize>, Vec<usize>)> {
    Some(match block {
        DRILL_MK3 => (1, vec![], vec![0]),
        ELECTRIC_FURNACE | MILL | PANTRY => (2, vec![0], vec![1]),
        KITCHEN | PAINT_MIXER | PAINT_BOOTH => (3, vec![0, 1], vec![2]),
        FABRICATOR => (4, vec![0, 1, 2], vec![3]),
        BIOMASS_GENERATOR | STEAM_ENGINE | PLANTER | SPRINKLER => (1, vec![0], vec![]),
        WATER_WHEEL | WIND_TURBINE | GROW_LAMP | PUMP | HYDRO_TURBINE => (1, vec![], vec![]),
        PIPE => (0, vec![], vec![]),
        TREE_FARM => (4, vec![0], vec![1, 2, 3]),
        RANCH => (5, vec![0, 1], vec![2, 3, 4]),
        MANUFACTURER => (6, vec![0, 1, 2, 3], vec![4, 5]),
        _ => return None,
    })
}

pub(super) fn outputs(block: &str) -> std::ops::Range<usize> {
    match block {
        DRILL_MK3 => 0..1,
        ELECTRIC_FURNACE | MILL | PANTRY => 1..2,
        KITCHEN | PAINT_MIXER | PAINT_BOOTH => 2..3,
        FABRICATOR => 3..4,
        TREE_FARM => 1..4,
        RANCH => 2..5,
        MANUFACTURER => 4..6,
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
        (SPRINKLER, 0) => stack.damage == 0 && stack.extra.is_none() && water::fertilizer(&stack.item).is_some(),
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
        // A cabine solta solvente: polui como uma fornalha.
        BIOMASS_GENERATOR | ELECTRIC_FURNACE | FABRICATOR | MANUFACTURER | PAINT_BOOTH => 2.0,
        MILL | KITCHEN | TREE_FARM | PAINT_MIXER => 1.0,
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
    match entity.id.as_str() {
        PUMP => { water::pump(host, pos, entity); return true; }
        PIPE => { water::pipe(host, pos, entity); return true; }
        SPRINKLER => { water::sprinkle(host, pos, entity, time); return true; }
        _ => {}
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

/// Todas as entradas da receita estão nos slots, na quantidade pedida.
fn ready(recipe: &Recipe, slots: &[Option<ItemStack>]) -> bool {
    recipe.inputs.iter().enumerate().all(|(slot, input)| {
        slots.get(slot).and_then(Option::as_ref)
            .is_some_and(|stack| matches_input(input, stack) && stack.count >= input.count)
    })
}

/// Sem receita completa: (índice da receita, entrada que falta) da receita com mais entradas
/// já atendidas entre as que usam algo que está nas entradas. `None` se nada nas entradas serve.
pub(super) fn closest(block: &str, slots: &[Option<ItemStack>]) -> Option<(usize, usize)> {
    let mut best: Option<(usize, usize, usize)> = None;
    for (index, recipe) in recipes(block).enumerate() {
        let mut started = false;
        let mut ready = 0;
        let mut missing = None;
        for (slot, input) in recipe.inputs.iter().enumerate() {
            let present = slots.get(slot).and_then(Option::as_ref).filter(|stack| matches_input(input, stack));
            started |= present.is_some();
            if present.is_some_and(|stack| stack.count >= input.count) {
                ready += 1;
            } else if missing.is_none() {
                missing = Some(slot);
            }
        }
        let Some(missing) = missing else { continue };
        if started && best.is_none_or(|(_, _, known)| ready > known) {
            best = Some((index, missing, ready));
        }
    }
    best.map(|(index, slot, _)| (index, slot))
}

/// Linha do painel quando faltam ingredientes: o item e a quantidade por ciclo que faltam na
/// receita mais próxima (`data[8]` receita, `data[12]` entrada). Só texto, para qualquer painel.
pub(super) fn missing_line(block: &str, data: &[i32]) -> Option<String> {
    use horizon_client::mod_api::Text;
    let read = |index: usize| data.get(index).copied().unwrap_or(0);
    let slot = read(12).rem_euclid(PICK_SHIFT);
    let (item, count) = missing_input(block, read(0), read(8), slot)?;
    Some(Text::tr("veio.todo.missing", &[&count, &contracts::item_name(item), &slot]).plain())
}

/// (item, quantidade por ciclo) que falta, se o estado for "faltam ingredientes".
pub(super) fn missing_input(block: &str, status: i32, recipe: i32, slot: i32) -> Option<(&'static str, u32)> {
    if status != 2 || slot <= 0 || block == HUB {
        return None;
    }
    let recipe = recipes(block).nth(usize::try_from(recipe).ok()?.checked_sub(1)?)?;
    let input = recipe.inputs.get(usize::try_from(slot - 1).ok()?)?;
    Some((input.items.first()?, input.count))
}

/// Receita automática: entre as que estão prontas, a de mais ingredientes. Assim "engrenagem +
/// chapa" faz uma coisa e "engrenagem + chapa + fiação" faz outra, sem a menor roubar a maior.
/// Em empate vale a ordem do `production.json`.
pub(super) fn find(block: &str, slots: &[Option<ItemStack>]) -> Option<&'static Recipe> {
    recipes(block).filter(|recipe| ready(recipe, slots))
        .fold(None, |best: Option<&'static Recipe>, recipe| match best {
            Some(best) if best.inputs.len() >= recipe.inputs.len() => Some(best),
            _ => Some(recipe),
        })
}

/// Receita fixada pelo jogador (NBT `Pick`: 0 = automática, n = n-ésima receita da máquina).
/// Novas receitas de uma máquina entram no fim da lista dela para não mudar as escolhas salvas.
pub(super) fn picked(block: &str, pick: i32) -> Option<&'static Recipe> {
    usize::try_from(pick).ok().filter(|pick| *pick > 0).and_then(|pick| recipes(block).nth(pick - 1))
}

/// Posição (1, 2...) da receita na lista da máquina, para o painel.
fn position(block: &str, recipe: &Recipe) -> i32 {
    recipes(block).position(|known| std::ptr::eq(known, recipe)).map_or(0, |index| index as i32 + 1)
}

/// Parte do `Info` que guarda a receita fixada (só no fabricante): `Pick` × `PICK_SHIFT`. A parte
/// de baixo (`Info % PICK_SHIFT`) continua sendo a entrada que falta, como nas outras máquinas.
fn pick_info(block: &str, pick: i32) -> i32 {
    if block == MANUFACTURER { pick.max(0) * PICK_SHIFT } else { 0 }
}

/// Primeira entrada da receita que ainda não está atendida.
fn first_missing(recipe: &Recipe, slots: &[Option<ItemStack>]) -> Option<usize> {
    recipe.inputs.iter().enumerate().position(|(slot, input)| {
        !slots.get(slot).and_then(Option::as_ref).is_some_and(|stack| matches_input(input, stack) && stack.count >= input.count)
    })
}

/// Motor comum das máquinas de receita: espera ingredientes, espaço e energia; aplica módulos.
pub(super) fn process(entity: &mut ModBlockEntity) {
    let outputs = output_slots(&entity.id);
    let output_slot = outputs.start;
    let pick = number(entity, "Pick");
    let chosen = if pick > 0 {
        picked(&entity.id, pick).filter(|recipe| ready(recipe, &entity.items))
    } else {
        find(&entity.id, &entity.items)
    };
    let Some(recipe) = chosen else {
        set_number(entity, "Status", 2);
        // A receita mais próxima do que está nas entradas: o painel mostra as taxas dela e
        // o que falta (`Info` = entrada que falta, a partir de 1; 0 = entradas vazias).
        // Receita fixada: o painel mostra ela e o que falta nela. Senão, a mais próxima do que
        // está nas entradas (`Info` = entrada que falta, a partir de 1; 0 = entradas vazias).
        let target = match picked(&entity.id, pick) {
            Some(recipe) => Some((position(&entity.id, recipe) as usize - 1, first_missing(recipe, &entity.items).unwrap_or(0))),
            None => closest(&entity.id, &entity.items),
        };
        match target {
            Some((index, slot)) => {
                set_number(entity, "RecipeIndex", index as i32 + 1);
                set_number(entity, "Info", slot as i32 + 1 + pick_info(&entity.id, pick));
            }
            None => set_number(entity, "Info", pick_info(&entity.id, pick)),
        }
        return;
    };
    set_number(entity, "Info", pick_info(&entity.id, pick));
    let byproduct_slot = (outputs.len() > 1).then_some(output_slot + 1);
    let (item, amount) = recipe.output;
    let (duration, draw) = upgrades::apply(recipe.ticks, recipe.power, upgrades::effect(entity));
    set_number(entity, "Duration", duration);
    // Índice da receita ativa (1 em diante) para o painel mostrar entrada e saída por minuto.
    let index = recipes(&entity.id).position(|other| std::ptr::eq(other, recipe)).map_or(0, |index| index as i32 + 1);
    set_number(entity, "RecipeIndex", index);
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
    let byproduct_full = match (recipe.byproduct, byproduct_slot) {
        (Some((extra, count)), Some(slot)) => !can_add(&entity.items[slot], extra, count),
        _ => false,
    };
    if !can_add(&entity.items[output_slot], item, produced) || byproduct_full {
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
    if let (Some((extra, count)), Some(slot)) = (recipe.byproduct, byproduct_slot) {
        add(&mut entity.items[slot], extra, count);
    }
    if upgrades::bonus_ready(entity, item) {
        add(&mut entity.items[output_slot], item, amount);
    }
    finish_cycle(entity, produced);
}

/// Itens por minuto de cada slot da receita `index` (1 em diante) com ciclo de `duration` ticks:
/// as entradas na ordem dos slots e, por último, a saída com o bônus de produtividade. Como no
/// Satisfactory, o painel escreve o número acima de cada slot.
pub(super) fn rates_per_minute(block: &str, index: i32, duration: i32, bonus: i32) -> Option<Vec<f32>> {
    let recipe = recipes(block).nth(usize::try_from(index).ok()?.checked_sub(1)?)?;
    let cycles = 1200.0 / duration.max(1) as f32;
    let mut rates: Vec<f32> = recipe.inputs.iter().map(|input| input.count as f32 * cycles).collect();
    rates.push(recipe.output.1 as f32 * cycles * (1.0 + bonus as f32 / 100.0));
    Some(rates)
}

/// Contadores comuns ao terminar um ciclo de produção.
pub(super) fn finish_cycle(entity: &mut ModBlockEntity, produced: u32) {
    set_number(entity, "Produced", number(entity, "Produced").saturating_add(produced as i32));
    set_number(entity, "Total", number(entity, "Total").saturating_add(1));
    set_number(entity, "Progress", 0);
}

/// Fontes contínuas e a lâmpada não têm ciclo: o painel omite a barra de progresso.
pub(super) fn shows_progress(block: &str) -> bool {
    !matches!(block, WATER_WHEEL | WIND_TURBINE | GROW_LAMP | PUMP | PIPE | HYDRO_TURBINE)
}

/// Posição dos slots no painel (largura 194, como os demais painéis do Veio).
pub(super) fn screen_slots(block: &str) -> Option<Vec<Option<[f32; 2]>>> {
    Some(match block {
        DRILL_MK3 | BIOMASS_GENERATOR | STEAM_ENGINE | PLANTER | SPRINKLER => vec![Some([89.0, 45.0])],
        PUMP | HYDRO_TURBINE => vec![None],
        ELECTRIC_FURNACE | MILL | PANTRY => vec![Some([53.0, 45.0]), Some([125.0, 45.0])],
        KITCHEN | PAINT_MIXER | PAINT_BOOTH => vec![Some([35.0, 45.0]), Some([71.0, 45.0]), Some([143.0, 45.0])],
        FABRICATOR => vec![Some([26.0, 45.0]), Some([53.0, 45.0]), Some([80.0, 45.0]), Some([143.0, 45.0])],
        WATER_WHEEL | WIND_TURBINE | GROW_LAMP => vec![None],
        TREE_FARM => vec![Some([26.0, 45.0]), Some([98.0, 45.0]), Some([116.0, 45.0]), Some([134.0, 45.0])],
        RANCH => vec![Some([17.0, 45.0]), Some([39.0, 45.0]), Some([116.0, 45.0]), Some([134.0, 45.0]), Some([152.0, 45.0])],
        MANUFACTURER => vec![Some([13.0, 45.0]), Some([31.0, 45.0]), Some([49.0, 45.0]), Some([67.0, 45.0]),
            Some([125.0, 45.0]), Some([157.0, 45.0])],
        _ => return None,
    })
}

/// Rótulos dos slots no painel; "" pula o rótulo (saídas agrupadas).
pub(super) fn roles(block: &str) -> Option<&'static [&'static str]> {
    Some(match block {
        DRILL_MK3 => &["output"],
        BIOMASS_GENERATOR | STEAM_ENGINE => &["fuel"],
        PLANTER => &["seeds"],
        SPRINKLER => &["fertilizer"],
        PUMP | HYDRO_TURBINE => &[],
        KITCHEN | PAINT_MIXER | PAINT_BOOTH => &["input1", "input2", "output"],
        FABRICATOR => &["input1", "input2", "input3", "output"],
        TREE_FARM => &["saplings", "output", "", ""],
        RANCH => &["feed", "bucket", "output", "", ""],
        PANTRY => &["food", "emeralds"],
        MANUFACTURER => &["", "", "", "", "output", "byproduct"],
        WATER_WHEEL | WIND_TURBINE | GROW_LAMP => &[],
        _ => return None,
    })
}

/// Linhas de informação no painel. `info` é o valor que a máquina publica no slot 12.
/// Máquinas que medem a estação e a temperatura do ar (guardadas em "Climate").
pub(crate) fn feels_weather(block: &str) -> bool {
    matches!(block, SOLAR | WIND_TURBINE | WATER_WHEEL | HYDRO_TURBINE | SPRINKLER)
}

pub(super) fn panel_lines(block: &str, data: &[i32]) -> Option<Vec<String>> {
    use horizon_client::mod_api::Text;
    let read = |index: usize| data.get(index).copied().unwrap_or(0);
    let info = read(12);
    let tr = |key: &str, args: &[&dyn std::fmt::Display]| Text::tr(key, args).plain();
    // Estação e temperatura que a máquina mediu; 0 enquanto não mediu.
    let climate = || -> String {
        let packed = read(4);
        if packed <= 0 { return tr("veio.climate.unknown", &[]); }
        let (season, celsius) = super::climate::unpack(packed);
        let name = tr(&format!("veio.season.{season}"), &[]);
        let key = if block == WATER_WHEEL && (celsius as f32) < super::climate::FREEZE { "veio.climate.frozen" }
            else if block == SPRINKLER && celsius < 3 { "veio.climate.frost" }
            else { "veio.climate" };
        tr(key, &[&name, &celsius])
    };
    Some(match block {
        DRILL | DRILL_MK2 | DRILL_MK3 => vec![
            tr("veio.metric.rate", &[&read(6)]),
            tr(&format!("veio.purity.{}", ["unknown", "impure", "normal", "pure", "rich"][info.clamp(0, 4) as usize]), &[]),
        ],
        BIOMASS_GENERATOR => vec![tr("veio.hint.biomass", &[])],
        STEAM_ENGINE => vec![tr("veio.metric.water", &[&read(8), &water::capacity(STEAM_ENGINE)]), tr("veio.hint.steam", &[])],
        PUMP => vec![tr("veio.metric.pumping", &[&info]),
            tr(if read(8) < 0 { "veio.metric.lava" } else { "veio.metric.water" }, &[&read(8).abs(), &water::capacity(PUMP)]),
            tr("veio.hint.pump", &[])],
        SPRINKLER => vec![tr("veio.metric.fertility", &[&info]), tr("veio.metric.grown", &[&read(9)]), climate()],
        HYDRO_TURBINE => vec![tr("veio.metric.output", &[&info]), tr("veio.metric.head", &[&read(8)]), climate()],
        WATER_WHEEL => vec![tr("veio.metric.output", &[&info]), climate(), tr("veio.hint.water_wheel", &[])],
        WIND_TURBINE => vec![tr("veio.metric.output", &[&info]), climate(), tr("veio.hint.wind", &[])],
        SOLAR => vec![tr("veio.metric.output", &[&info]), climate(), tr("veio.hint.solar", &[])],
        PLANTER => vec![tr("veio.metric.planted", &[&read(9)]), tr("veio.hint.planter", &[])],
        GROW_LAMP => vec![tr("veio.metric.grown", &[&read(9)]), tr("veio.hint.grow_lamp", &[])],
        TREE_FARM => vec![tr("veio.metric.trees", &[&read(9)]), tr("veio.hint.tree_farm", &[])],
        RANCH => vec![tr("veio.metric.animals", &[&info]), tr("veio.hint.ranch", &[])],
        // Automática e parada: nenhuma receita rodando, mesmo que `RecipeIndex` lembre a última.
        MANUFACTURER => manufacturer_lines(if read(0) == 1 { read(8) } else { 0 }, info.div_euclid(PICK_SHIFT), read(6)),
        PANTRY => vec![
            tr("veio.metric.credit", &[&read(1), &pantry::PRICE]),
            if info >= pantry::VILLAGE_SIZE { tr("veio.pantry.village", &[&info]) } else { tr("veio.pantry.no_village", &[&info]) },
        ],
        _ => return None,
    })
}

/// Painel do fabricante: a receita (com as setas para trocar) e a vazão.
fn manufacturer_lines(current: i32, pick: i32, rate: i32) -> Vec<String> {
    use horizon_client::mod_api::Text;
    let name = |position: i32| picked(MANUFACTURER, position)
        .map(|recipe| Text::tr(&format!("veio.recipe.{}", recipe.id), &[]).plain());
    let recipe = match (pick, name(current)) {
        (0, Some(running)) => Text::tr("veio.pick.auto", &[&running]).plain(),
        (0, None) => Text::tr("veio.pick.auto_idle", &[]).plain(),
        (_, running) => Text::tr("veio.pick.fixed", &[&running.or_else(|| name(pick)).unwrap_or_default()]).plain(),
    };
    vec![
        format!("< {recipe} >"),
        Text::tr("veio.metric.rate", &[&rate]).plain(),
    ]
}

/// Região clicável da linha da receita: metade esquerda volta, metade direita avança.
pub(super) fn click(block: &str, at: [f32; 2]) -> Option<i32> {
    (block == MANUFACTURER && (106.0..119.0).contains(&at[1]) && (6.0..188.0).contains(&at[0]))
        .then(|| if at[0] < 97.0 { PICK_PREVIOUS } else { PICK_NEXT })
}

/// O botão chegou ao servidor: passa à receita anterior ou seguinte (0 = automática).
pub(super) fn press(entity: &mut ModBlockEntity, id: i32) {
    if entity.id != MANUFACTURER {
        return;
    }
    let choices = recipes(MANUFACTURER).count() as i32 + 1;
    let step = match id { PICK_PREVIOUS => choices - 1, PICK_NEXT => 1, _ => return };
    let pick = (number(entity, "Pick").max(0) + step).rem_euclid(choices);
    set_number(entity, "Pick", pick);
    set_number(entity, "Progress", 0);
    if pick > 0 {
        set_number(entity, "RecipeIndex", pick);
    }
    set_number(entity, "Info", pick * PICK_SHIFT);
}

/// Peças animadas próprias (roda d'água, pás do cata-vento). Devolve `true` se desenhou.
pub(super) fn draw(host: &dyn Host, event: &mut horizon_client::mod_api::BlockEntitySubmitEvent<'_>) -> bool {
    generation::draw(host, event)
}

#[cfg(test)]
mod tests;
#[cfg(test)]
mod balance_tests;
