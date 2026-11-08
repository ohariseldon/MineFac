mod budget;
mod presentation;
mod appearance;
mod contracts;
mod freight;
mod logistics;
mod logistics_controls;
mod inserters;
pub(crate) mod operations;
mod defense;
mod farm;
mod home;
mod pollution;
mod production;
mod power;
mod tutorial;
mod onboarding;
mod upgrades;
mod workforce;

use horizon_client::mod_api::{
    BlockEntityContainerEvent, BlockEntityEvent, BlockEntityTickEvent, BlockEntityTicksEvent,
    Compound, ContainerAcceptsEvent, Direction, Flow, Hooks, Host, ItemStack, ModBlockEntity, ModContainer, ModEnergyBuffer, Tag,
};

// Horizon exports Compound but not its UTF-16 key type. Derive that type from
// the public map instead of requiring an engine patch or a second dependency.
pub(crate) trait CompoundKey { type Key; }
impl<K, V> CompoundKey for std::collections::BTreeMap<K, V> { type Key = K; }
type JavaString = <Compound as CompoundKey>::Key;

const DRILL: &str = "veio:drill";
const PRESS: &str = "veio:press";
const ASSEMBLER: &str = "veio:assembler";
const BELT: &str = "veio:belt";
const FAST_BELT: &str = "veio:fast_belt";
const SPLITTER: &str = "veio:splitter";
const MERGER: &str = "veio:merger";
const CUTTER: &str = "veio:cutter";
const INTEGRATOR: &str = "veio:integrator";
const PLATE: &str = "veio:copper_plate";
const GEAR: &str = "veio:gear";
const FRAME: &str = "veio:frame";
const MODULE: &str = "veio:module";
const GENERATOR: &str = "veio:generator";
const CABLE: &str = "veio:cable";
const SOLAR: &str = "veio:solar";
const BATTERY: &str = "veio:battery";
const GEOTHERMAL: &str = "veio:geothermal";
const CRUSHER: &str = "veio:crusher";
const ALLOY: &str = "veio:alloy_furnace";
const ELECTRONICS: &str = "veio:electronics";
const DRILL_MK2: &str = "veio:drill_mk2";
const STEEL: &str = "veio:steel_plate";
const CIRCUIT: &str = "veio:circuit";
const WRENCH: &str = "veio:wrench";
const HUB: &str = "veio:contract_hub";
const HARVESTER: &str = "veio:harvester";
const TESLA: &str = "veio:tesla";
const BONE_MEAL: &str = "minecraft:bone_meal";
const POLE: &str = "veio:pole";
const STEEL_POLE: &str = "veio:steel_pole";
const REPAIR_KIT: &str = "veio:repair_kit";
/// Cada kit de reparo recupera metade da integridade.
const REPAIR: i32 = 50;


pub fn hooks() -> Box<dyn Hooks> {
    Box::new(Factory)
}

struct Factory;

impl Hooks for Factory {
    fn server_about_to_start(&self, _host: &mut dyn Host, _event: &mut horizon_client::mod_api::ServerAboutToStartEvent) {
        // A casa, as camas e os bônus em memória pertencem ao mundo que abre.
        // O painel carrega os próprios dados salvos quando começar a funcionar.
        home::reset();
    }

    fn block_entity(
        &self,
        _host: &mut dyn Host,
        event: &mut BlockEntityEvent<'_>,
    ) -> Option<(String, Compound)> {
        is_factory_block(event.block).then(|| (event.block.to_owned(), Compound::default()))
    }

    fn block_entity_ticks(&self, _host: &dyn Host, event: &mut BlockEntityTicksEvent<'_>) {
        if is_factory_block(event.block) {
            event.ticks = true;
        }
    }

    fn block_entity_container(
        &self,
        _host: &dyn Host,
        event: &mut BlockEntityContainerEvent<'_>,
    ) -> Option<ModContainer> {
        let (slots, mut inputs, mut outputs) = layout(event.block)?;
        if freight::owns(event.block) {
            inputs = (0..6).collect();
            if event.block == freight::caravan::ID { inputs.push(7); }
            // Hoppers load trains from dispatch stock; workers use received stock.
            outputs = if event.block == freight::RAIL { (0..3).collect() } else { (3..6).collect() };
        }
        Some(ModContainer {
            slots,
            faces: std::array::from_fn(|_| (0..slots).collect()),
            inputs,
            outputs,
            menu: (slots > 0).then(|| event.block.to_owned()),
        })
    }

    fn container_accepts(&self, host: &dyn Host, event: &mut ContainerAcceptsEvent<'_>) -> Flow {
        if operations::accepts_delivery(host, event) {
            Flow::Continue
        } else {
            Flow::Cancel
        }
    }

    fn right_click_block(&self, host: &mut dyn Host,
        event: &mut horizon_client::mod_api::RightClickBlockEvent) -> Flow
    {
        let block = host.block(event.pos);
        if let Some(flow) = freight::right_click(host, event.pos, &block) { return flow; }
        onboarding::select(host, event.pos, &block, true);
        if let Some(flow) = logistics_controls::right_click(host, event.pos, &block) { return flow; }
        if !is_factory_block(&block) {
            return Flow::Continue;
        }
        // Mão vazia chega como `None`. O quartel usa agachar com a mão vazia, então os postos
        // recebem "" nesse caso; o resto continua exigindo um item na mão.
        let held = host.held().map(|(item, _, _)| item);
        if workforce::owns(&block)
            && let Some(flow) = workforce::right_click(host, event.pos, held.as_deref().unwrap_or(""))
        {
            return flow;
        }
        let Some(held) = held else { return Flow::Continue };
        let pos = event.pos;
        let mut damage = 0;
        let mut module = 0;
        host.change_mod_block_entity(pos, &mut |entity| {
            damage = number(entity, "Damage");
            module = number(entity, "Module");
        });
        if held == REPAIR_KIT {
            if damage == 0 || host.take_items(REPAIR_KIT, 1) == 0 {
                return Flow::Continue;
            }
            host.change_mod_block_entity(pos, &mut |entity| {
                set_number(entity, "Damage", (number(entity, "Damage") - REPAIR).max(0));
            });
            host.play_sound("minecraft:block.anvil.use", center(pos), 4, 0.5, 1.4);
            return Flow::Cancel;
        }
        if let Some(index) = upgrades::index(&held) {
            if !upgrades::supports(&block) || module != 0 || host.take_items(&held, 1) == 0 {
                return Flow::Continue;
            }
            host.change_mod_block_entity(pos, &mut |entity| set_number(entity, "Module", index));
            host.play_sound("minecraft:block.smithing_table.use", center(pos), 4, 0.6, 1.2);
            return Flow::Cancel;
        }
        if held != WRENCH {
            return Flow::Continue;
        }
        if host.sneaking() && module > 0 {
            host.change_mod_block_entity(pos, &mut |entity| set_number(entity, "Module", 0));
            host.give_items(upgrades::MODULES[(module - 1) as usize], 1);
            return Flow::Cancel;
        }
        let facing = host.block_property(pos, "facing");
        if logistics::is_belt(&block) && host.sneaking() {
            host.change_mod_block_entity(pos, &mut |entity| set_number(entity, "ManualDirection", 0));
            return Flow::Cancel;
        }
        let next = match facing.as_deref() {
            Some("north") => "east",
            Some("east") => "south",
            Some("south") => "west",
            Some("west") => "north",
            _ => return Flow::Continue,
        };
        if host.set_block_property(pos, "facing", next) {
            if logistics::is_belt(&block) {
                host.change_mod_block_entity(pos, &mut |entity| set_number(entity, "ManualDirection", 1));
            }
            Flow::Cancel
        }
        else { Flow::Continue }
    }

    fn block_entity_energy(
        &self,
        _host: &dyn Host,
        event: &mut horizon_client::mod_api::BlockEntityEnergyEvent<'_>,
    ) -> Option<ModEnergyBuffer> {
        if !needs_power(event.block) && !is_power_block(event.block) && power::tier(event.block).is_none() {
            return None;
        }
        Some(ModEnergyBuffer {
            capacity: capacity(event.block),
            max_insert: 80,
            max_extract: 80,
            insert_faces: [!matches!(event.block, GENERATOR | SOLAR | GEOTHERMAL); 6],
            extract_faces: [is_power_block(event.block) || power::tier(event.block).is_some(); 6],
        })
    }

    fn block_entity_tick(&self, host: &mut dyn Host, event: &mut BlockEntityTickEvent<'_>) {
        if !is_factory_block(&event.entity.id) {
            return;
        }
        let Some(time) = host.level(host.dimension()).and_then(|level| level.game_time(host)) else {
            return;
        };
        pollution::update(host, time);
        if power::tick(host, event.pos, event.entity, time) {
            host.block_entity_sync(event.pos);
        }
        onboarding::before_machine(host, event.pos, event.entity);
        let previous = visual_signature(event.entity);
        let produced = number(event.entity, "Produced");
        let tutorial_cycles = number(event.entity, "Total");
        let tutorial_contract = if event.entity.id == HUB { number(event.entity, "ContractStage") } else { 0 };
        tick_factory(host, event.pos, event.entity, time);
        onboarding::observe_production(host.dimension(), event.pos, event.entity, produced, tutorial_cycles, tutorial_contract, time);
        pollution::emit(host, event.pos, event.entity, time);
        record_performance(event.entity, produced);
        presentation::publish(event.entity);
        if visual_signature(event.entity) != previous {
            host.block_entity_sync(event.pos);
        }
    }

    fn entity_tick_pre(&self, host: &mut dyn Host,
        event: &mut horizon_client::mod_api::EntityTickPreEvent<'_>) -> Flow
    {
        freight::caravan::observe(host, event.dimension, event.entity, event.kind);
        defense::observe(event.dimension, event.entity, event.kind);
        production::ranch::observe(event.dimension, event.entity, event.kind);
        if event.kind == "minecraft:villager" {
            let now = defense::clock();
            workforce::village::observe(host, event.dimension, event.entity, now);
            workforce::village::scan_homes(host, event.dimension, event.entity, now);
        }
        Flow::Continue
    }

    fn player_logged_in(&self, host: &mut dyn Host, event: &mut horizon_client::mod_api::PlayerLoggedInEvent<'_>) {
        onboarding::logged_in(host);
        workforce::spawn::logged_in(event.player, event.name, defense::clock());
    }

    fn player_respawned(&self, _host: &mut dyn Host, event: &mut horizon_client::mod_api::PlayerRespawnedEvent<'_>) {
        workforce::spawn::respawned(event.player, event.name);
    }

    /// Casa do fundador: a primeira cama é a da casa, e só mais uma fora dela.
    fn player_spawn_point_set(&self, host: &mut dyn Host, event: &mut horizon_client::mod_api::PlayerSpawnPointSetEvent<'_>) -> Flow {
        home::spawn_point(host, event.player, event.dimension, event.pos, event.cause)
    }

    fn player_bed_enter(&self, host: &mut dyn Host, event: &mut horizon_client::mod_api::PlayerBedEnterEvent<'_>) -> Flow {
        home::bed_enter(host, event.player, event.pos)
    }

    /// Comida do jogador: refeições e banquetes aceleram as máquinas (`home::food`).
    fn living_entity_use_item_finish(&self, host: &mut dyn Host, event: &mut horizon_client::mod_api::LivingEntityUseItemFinishEvent<'_>) {
        if event.entity.is_some() {
            return;
        }
        if let Some(player) = host.acting_player() {
            home::food::eaten(player.uuid, &player.name, &player.dimension, player.position, &event.stack.item, defense::clock());
        }
    }

    fn tick(&self, host: &mut dyn Host, _event: &mut horizon_client::mod_api::TickEvent) {
        // No jogo de um jogador só, o gancho roda duas vezes por tick: no servidor interno e na
        // janela (cliente). O Veio é todo do servidor; na janela, o relógio andaria em dobro e
        // fora de ordem, as cotas zerariam no meio do tick e um passo da poluição podia ser
        // gasto no cliente (onde não há jogadores nem `summon`), perdendo a onda da noite.
        if host.window() {
            return;
        }
        defense::advance_clock();
        budget::begin_tick(defense::clock());
        power::maintenance(defense::clock());
        pollution::note_players(host, defense::clock());
        pollution::harm_villagers(host, defense::clock());
        workforce::maintenance(defense::clock());
        workforce::spawn::tick(host, defense::clock());
        onboarding::server_tick(host);
        home::tick(host, defense::clock());
        if let Some(time) = host.level(host.dimension()).and_then(|level| level.game_time(host)) {
            pollution::update(host, time);
        }
    }

    fn inventory_move_item(&self, host: &dyn Host, event: &mut horizon_client::mod_api::InventoryMoveItemEvent<'_>) -> Flow {
        onboarding::hopper_attempt(host, event.source, event.destination, event.item);
        Flow::Continue
    }

    fn place_block(&self, host: &mut dyn Host, event: &mut horizon_client::mod_api::PlaceBlockEvent<'_>) {
        onboarding::select(host, event.pos, &host.block(event.pos), false);
    }

    fn player_logged_out(&self, host: &mut dyn Host, event: &mut horizon_client::mod_api::PlayerLoggedOutEvent<'_>) {
        onboarding::logout(host, event.player);
    }

    fn register_commands(&self, _host: &mut dyn Host, event: &mut horizon_client::mod_api::RegisterCommandsEvent<'_>) {
        for action in ["ocultar", "mostrar", "reiniciar"] {
            event.commands.push(horizon_client::mod_api::NativeCommand::new(&format!("veiotutorial {action}")));
        }
    }

    fn perform_command(&self, host: &mut dyn Host, event: &mut horizon_client::mod_api::PerformCommandEvent<'_>) -> Flow {
        if onboarding::command(host, event.player, event.command) { Flow::Cancel } else { Flow::Continue }
    }

    fn register_goals(&self, event: &mut horizon_client::mod_api::RegisterGoalsEvent<'_>) {
        pollution::register_goals(event.mob);
        workforce::register_goals(event.mob);
        freight::caravan::register_goals(event.mob);
    }

    fn block_entity_render_bounding_box(&self, _host: &dyn Host,
        event: &mut horizon_client::mod_api::BlockEntityRenderBoundingBoxEvent<'_>)
    {
        if power::tier(event.block).is_some() {
            power::bounds(event.entity, event.bounds);
        }
        production::generation::bounds(event.block, event.bounds);
        if event.block == freight::DRONE { freight::bounds(event.entity, event.bounds); }
        if inserters::is_arm(event.block) { *event.bounds = [-1.5, 0.0, -1.5, 2.5, 1.0, 2.5]; }
        if workforce::zones::is_marker(event.block) {
            workforce::zones::bounds(event.entity, event.bounds);
        }
        if logistics::is_belt(event.block) {
            event.bounds[4] = event.bounds[4].max(1.6);
        }
    }

    fn screen(&self, _host: &dyn Host, event: &mut horizon_client::mod_api::ScreenEvent<'_>)
        -> Option<horizon_client::mod_api::NativeScreen>
    {
        presentation::screen(event.block)
    }

    fn screen_draw(&self, host: &dyn Host, event: &mut horizon_client::mod_api::ScreenDrawEvent<'_>) {
        presentation::screen_draw(host, event);
    }

    fn screen_tooltip(&self, _host: &dyn Host, event: &mut horizon_client::mod_api::ScreenTooltipEvent<'_>) {
        tutorial::screen_tooltip(event);
    }

    fn tooltip(&self, _host: &dyn Host, event: &mut horizon_client::mod_api::TooltipEvent<'_>) {
        tutorial::tooltip(event);
    }

    fn menu_slot_rules(&self, event: &mut horizon_client::mod_api::MenuSlotRulesEvent<'_>) {
        let Some((slots, _, _)) = layout(event.menu) else {
            return;
        };
        if event.slot < slots {
            event.may_place &= accepts(event.menu, event.slot, event.stack);
        }
    }

    fn block_entity_update_tag(
        &self,
        event: &mut horizon_client::mod_api::BlockEntityUpdateTagEvent<'_>,
    ) -> Option<Compound> {
        is_factory_block(&event.entity.id).then(|| event.entity.nbt.clone())
    }

    fn block_entity_submit(&self, host: &dyn Host,
        event: &mut horizon_client::mod_api::BlockEntitySubmitEvent<'_>)
    {
        presentation::draw(host, event);
    }

}

fn is_factory_block(block: &str) -> bool {
    matches!(block, DRILL | DRILL_MK2 | PRESS | ASSEMBLER | BELT | FAST_BELT |
        SPLITTER | MERGER | CUTTER | INTEGRATOR | GENERATOR | CABLE | SOLAR | BATTERY |
        GEOTHERMAL | CRUSHER | ALLOY | ELECTRONICS | HUB | HARVESTER | TESLA | POLE | STEEL_POLE | inserters::ID | inserters::FAST | inserters::LONG)
        || production::is_machine(block) || workforce::owns(block) || freight::owns(block)
}

/// Consumidores: recebem energia por cabos, param com redstone e mostram métricas.
fn needs_power(block: &str) -> bool {
    matches!(block, DRILL | DRILL_MK2 | PRESS | ASSEMBLER | CUTTER | INTEGRATOR |
        CRUSHER | ALLOY | ELECTRONICS | HARVESTER | TESLA | inserters::ID | inserters::FAST | inserters::LONG)
        || production::consumes(block) || block == freight::DRONE
}

/// Produzem, guardam ou conduzem energia.
fn is_power_block(block: &str) -> bool {
    matches!(block, GENERATOR | CABLE | SOLAR | BATTERY | GEOTHERMAL) || production::generates(block)
}

/// Fontes de energia: a rede puxa delas, mas não as abastece. Interface com a logística.
fn is_generator(block: &str) -> bool {
    matches!(block, GENERATOR | SOLAR | GEOTHERMAL) || production::generates(block)
}

fn capacity(block: &str) -> i32 {
    match block {
        BATTERY => 30000,
        freight::DRONE => 8000,
        TESLA => defense::CAPACITY,
        _ => power::tier(block).map_or(2000, |tier| tier.capacity),
    }
}

/// Slots, entradas automáticas e saídas automáticas de cada máquina.
fn layout(block: &str) -> Option<(usize, Vec<usize>, Vec<usize>)> {
    Some(match block {
        DRILL | DRILL_MK2 => (1, vec![], vec![0]),
        PRESS | ASSEMBLER | CUTTER | CRUSHER => (2, vec![0], vec![1]),
        INTEGRATOR | ALLOY | ELECTRONICS => (3, vec![0, 1], vec![2]),
        BELT | FAST_BELT | SPLITTER => (1, vec![0], vec![0]),
        MERGER => (2, vec![0, 1], vec![0, 1]),
        GENERATOR => (1, vec![0], vec![]),
        GEOTHERMAL => (2, vec![0], vec![1]),
        SOLAR | BATTERY | TESLA | inserters::ID | inserters::FAST | inserters::LONG => (1, vec![], vec![]),
        CABLE | POLE | STEEL_POLE => (0, vec![], vec![]),
        HUB => (4, vec![0, 1], vec![2, 3]),
        HARVESTER => (4, vec![0], vec![1, 2, 3]),
        _ if freight::owns(block) => return freight::layout(block),
        _ if workforce::owns(block) => return workforce::layout(block),
        _ => return production::layout(block),
    })
}

/// Há slot de entrada de itens? Evita alocar o `layout` só para decidir uma curva de esteira.
fn has_item_inputs(block: &str) -> bool {
    match block {
        BELT | FAST_BELT | SPLITTER | MERGER | PRESS | ASSEMBLER | CUTTER | CRUSHER |
        INTEGRATOR | ALLOY | ELECTRONICS | GENERATOR | GEOTHERMAL | HUB | HARVESTER => true,
        DRILL | DRILL_MK2 | SOLAR | BATTERY | TESLA | CABLE | POLE | STEEL_POLE | inserters::ID | inserters::FAST | inserters::LONG => false,
        _ if freight::owns(block) => true,
        _ if workforce::is_post(block) || workforce::storage::is_store(block) => true,
        _ => production::has_inputs(block),
    }
}

/// Slots que o bocal da máquina esvazia.
fn output_slots(block: &str) -> std::ops::Range<usize> {
    match block {
        MERGER => 0..2,
        PRESS | ASSEMBLER | CUTTER | CRUSHER | GEOTHERMAL => 1..2,
        INTEGRATOR | ALLOY | ELECTRONICS => 2..3,
        HUB => 2..4,
        HARVESTER => 1..4,
        GENERATOR | SOLAR | BATTERY | TESLA | CABLE | POLE | STEEL_POLE | inserters::ID | inserters::FAST | inserters::LONG => 0..0,
        _ if freight::owns(block) => 3..6,
        _ if production::is_machine(block) => production::outputs(block),
        _ if workforce::owns(block) => workforce::outputs(block),
        _ => 0..1,
    }
}

fn accepts(block: &str, slot: usize, stack: &ItemStack) -> bool {
    match (block, slot) {
        (HUB, _) => contracts::valid_input(slot, stack),
        (BELT | FAST_BELT | SPLITTER, 0) => true,
        (MERGER, 0 | 1) => true,
        _ if freight::owns(block) => (slot < 6 || block == freight::caravan::ID && slot == 7 && freight::caravan::feed(&stack.item)) && stack.damage == 0 && stack.extra.is_none(),
        _ if workforce::owns(block) => workforce::accepts(block, slot, stack),
        // Entradas das máquinas de produção, pela tabela de receitas e combustíveis.
        _ => production::accepts(block, slot, stack),
    }
}

/// Chaves NBT lidas todo tick. A primeira vez monta o `JavaString`; as seguintes reaproveitam.
fn nbt_key(name: &str) -> &'static JavaString {
    #[cfg(test)] perf::key_read();
    macro_rules! key {
        ($lit:literal) => {{
            static KEY: std::sync::OnceLock<JavaString> = std::sync::OnceLock::new();
            KEY.get_or_init(|| JavaString::from($lit))
        }};
    }
    match name {
        "Absent" => key!("Absent"),
        "Action" => key!("Action"),
        "Admin" => key!("Admin"),
        "Area" => key!("Area"),
        "Bonus" => key!("Bonus"),
        "BonusItem" => key!("BonusItem"),
        "Burn" => key!("Burn"),
        "BurnTotal" => key!("BurnTotal"),
        "ContractStage" => key!("ContractStage"),
        "FlightTarget" => key!("FlightTarget"),
        "FlightDelta" => key!("FlightDelta"),
        "FlightPhase" => key!("FlightPhase"),
        "FreightExport" => key!("FreightExport"),
        "FreightImport" => key!("FreightImport"),
        "FreightChannel" => key!("FreightChannel"),
        "FreightManualImport" => key!("FreightManualImport"),
        "CaravanAnimal" => key!("CaravanAnimal"),
        "CaravanEntity" => key!("CaravanEntity"),
        "CaravanProbe" => key!("CaravanProbe"),
        "CaravanLastPos" => key!("CaravanLastPos"),
        "CaravanStall" => key!("CaravanStall"),


        "Cooldown" => key!("Cooldown"),
        "Damage" => key!("Damage"),
        "DisplayCount" => key!("DisplayCount"),
        "DisplayItem" => key!("DisplayItem"),
        "Duration" => key!("Duration"),
        "Herd" => key!("Herd"),
        "Home" => key!("Home"),
        "HomeBeds" => key!("HomeBeds"),
        "Info" => key!("Info"),
        "Level" => key!("Level"),
        "LogisticsFilter" => key!("LogisticsFilter"),
        "LogisticsPaused" => key!("LogisticsPaused"),
        "Links" => key!("Links"),
        "LocalSmog" => key!("LocalSmog"),
        "ManualDirection" => key!("ManualDirection"),
        "MeterTicks" => key!("MeterTicks"),
        "MeterActive" => key!("MeterActive"),
        "MeterItems" => key!("MeterItems"),
        "MeterReady" => key!("MeterReady"),
        "Swing" => key!("Swing"),
        "RoutingPriority" => key!("RoutingPriority"),
        "AreaScan" => key!("AreaScan"),
        "TopologyPending" => key!("TopologyPending"),
        "PlanningPending" => key!("PlanningPending"),
        "TerrainPending" => key!("TerrainPending"),
        "AreaFound" => key!("AreaFound"),
        "Posts" => key!("Posts"),
        "Queue" => key!("Queue"),
        "DWorkers" => key!("DWorkers"),
        "DWorking" => key!("DWorking"),
        "DWaiting" => key!("DWaiting"),
        "DHungry" => key!("DHungry"),
        "DFields" => key!("DFields"),
        "DPens" => key!("DPens"),
        "DStore" => key!("DStore"),
        "DQueue" => key!("DQueue"),
        "DGoal1" => key!("DGoal1"),
        "DGoal2" => key!("DGoal2"),
        "DGoal3" => key!("DGoal3"),
        "Side" => key!("Side"),
        "Module" => key!("Module"),
        "MovedAt" => key!("MovedAt"),
        "NextInput" => key!("NextInput"),
        "Organize" => key!("Organize"),
        "PathReady" => key!("PathReady"),
        "PathSig" => key!("PathSig"),
        "PreferRight" => key!("PreferRight"),
        "Pick" => key!("Pick"),
        "Port" => key!("Port"),
        "Produced" => key!("Produced"),
        "Progress" => key!("Progress"),
        "Queued" => key!("Queued"),
        "Rate" => key!("Rate"),
        "Role" => key!("Role"),
        "Ration" => key!("Ration"),
        "Recipe" => key!("Recipe"),
        "Refresh" => key!("Refresh"),
        "Scan" => key!("Scan"),
        "Sense" => key!("Sense"),
        "SmogTicks" => key!("SmogTicks"),
        "Status" => key!("Status"),
        "TargetMob" => key!("TargetMob"),
        "TaskId" => key!("TaskId"),
        "Total" => key!("Total"),
        "Unlocked" => key!("Unlocked"),
        "Utilization" => key!("Utilization"),
        "Villagers" => key!("Villagers"),
        "Walk" => key!("Walk"),
        "Work" => key!("Work"),
        "energy" => key!("energy"),
        _ => {
            // Chave rara: vive até o fim do processo, para o tick seguinte não alocar de novo.
            #[cfg(test)] perf::fallback_lock();
            use std::collections::HashMap;
            use std::sync::Mutex;
            static EXTRA: Mutex<Option<HashMap<String, &'static JavaString>>> = Mutex::new(None);
            let mut extra = EXTRA.lock().unwrap_or_else(|err| err.into_inner());
            let extra = extra.get_or_insert_with(HashMap::new);
            if let Some(key) = extra.get(name) {
                return key;
            }
            let key: &'static JavaString = Box::leak(Box::new(JavaString::from(name)));
            extra.insert(name.to_owned(), key);
            key
        }
    }
}

pub(crate) fn with_nbt_key<T>(name: &str, read: impl FnOnce(&JavaString) -> T) -> T {
    read(nbt_key(name))
}

/// `JavaString` igual a `value`, sem montar uma `String` no meio.
pub(crate) fn java_eq(text: &JavaString, value: &str) -> bool {
    let mut units = text.0.iter().copied();
    value.encode_utf16().all(|unit| units.next() == Some(unit)) && units.next().is_none()
}

fn number(entity: &ModBlockEntity, key: &str) -> i32 {
    with_nbt_key(key, |key| match entity.nbt.get(key) {
        Some(Tag::Int(value)) => (*value).max(0),
        _ => 0,
    })
}

fn set_number(entity: &mut ModBlockEntity, key: &str, value: i32) {
    with_nbt_key(key, |key| {
        if let Some(tag) = entity.nbt.get_mut(key) {
            let current = match tag { Tag::Int(stored) => (*stored).max(0), _ => 0 };
            if current != value { *tag = Tag::Int(value); }
        } else {
            entity.nbt.insert(key.clone(), Tag::Int(value));
        }
    });
}

fn visual_signature(entity: &ModBlockEntity) -> (i32, i32, i32, Option<ItemStack>) {
    // A carga desenhada no mundo é só de esteira, divisor e unificador; o braço anima pelo `MovedAt`.
    let cargo = if matches!(entity.id.as_str(), BELT | FAST_BELT | SPLITTER | MERGER) {
        logistics::display_stack(entity).cloned()
    } else {
        None
    };
    (number(entity, "Status"), number(entity, "Module"), if matches!(entity.id.as_str(), freight::DRONE | freight::caravan::ID) { number(entity, "FlightPhase") * 1_000_000 + number(entity, "Progress") / 4 }
        else { number(entity, if inserters::is_arm(&entity.id) { "MovedAt" } else { "Info" }) }, cargo)
}

fn center([x, y, z]: [i32; 3]) -> [f64; 3] {
    [f64::from(x) + 0.5, f64::from(y) + 0.5, f64::from(z) + 0.5]
}

fn stamp(time: i64) -> i32 {
    (time % i64::from(i32::MAX)) as i32
}

fn tick_factory(host: &mut dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity, time: i64) {
    if freight::owns(&entity.id) { freight::tick(host, pos, entity, time); return; }
    if inserters::is_arm(&entity.id) { return inserters::tick(host, pos, entity, time); }
    logistics::transfer_outputs(host, pos, entity, time);
    if entity.id == HUB {
        contracts::tick(entity);
        return;
    }
    if power::tier(&entity.id).is_some() {
        return;
    }
    if pollution::emission(&entity.id) > 0.0 && pollution::wear(host, pos, entity) {
        set_number(entity, "Status", 13);
        return;
    }
    if logistics::update_status(entity, time) {
        return;
    }
    if is_power_block(&entity.id) {
        tick_power(host, pos, entity);
        return;
    }
    if workforce::owns(&entity.id) {
        // Trabalhadores, zonas e armazéns: sem energia; o custo é a comida do aldeão.
        return workforce::tick(host, pos, entity, time);
    }
    if needs_power(&entity.id) && host.neighbor_signal(host.dimension(), pos) > 0 {
        set_number(entity, "Status", 9);
        return;
    }
    if entity.id == TESLA {
        if time.rem_euclid(20) == 0 {
            set_number(entity, "LocalSmog", pollution::local(host.dimension(), pos) as i32);
        }
        return defense::tick(host, pos, entity);
    }
    // Extração, receitas, lavouras, rancho e armazém: módulo de produção.
    production::tick(host, pos, entity, time);
}

fn tick_power(host: &mut dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity) {
    let limit = capacity(&entity.id);
    let mut energy = number(entity, "energy").min(limit);
    // Geração (produção); a distribuição abaixo é da logística.
    energy = production::generation::produce(host, pos, entity, energy, limit);
    // A rede só empurra. Sem carga, as seis consultas a vizinhos não movem energia.
    if energy > 0 {
        let offset = number(entity, "Port") as usize % 6;
        for i in 0..6 {
            let face = Direction::ALL[(i + offset) % 6];
            let delta = face.normal();
            let target = std::array::from_fn(|axis| pos[axis] + delta[axis]);
            host.change_mod_block_entity(target, &mut |other| {
                let storage = matches!(other.id.as_str(), CABLE | BATTERY) || power::tier(&other.id).is_some();
                if !needs_power(&other.id) && !storage {
                    return;
                }
                let other_limit = capacity(&other.id);
                let stored = number(other, "energy").min(other_limit);
                // Equalize charge percentages, so large batteries can charge and discharge.
                let offered = if storage {
                    ((i64::from(energy) * i64::from(other_limit)
                        - i64::from(stored) * i64::from(limit))
                        / i64::from(limit + other_limit)).clamp(0, 80) as i32
                } else { energy.min(80) };
                let accepted = offered.min(other_limit - stored);
                if accepted > 0 {
                    set_number(other, "energy", stored + accepted);
                    energy -= accepted;
                }
            });
        }
        set_number(entity, "Port", ((offset + 1) % 6) as i32);
    }
    set_number(entity, "energy", energy);
    let status = if energy == limit { 8 }
        else if energy == 0 { production::generation::empty_status(entity) }
        else { 1 };
    set_number(entity, "Status", status);
}

fn record_performance(entity: &mut ModBlockEntity, previous_produced: i32) {
    if !needs_power(&entity.id) { return; }
    let ticks = number(entity, "MeterTicks") + 1;
    let active = number(entity, "MeterActive") + i32::from(number(entity, "Status") == 1);
    let items = number(entity, "MeterItems") + (number(entity, "Produced") - previous_produced).max(0);
    if number(entity, "MeterReady") == 0 || ticks >= 1200 {
        set_number(entity, "Rate", items * 1200 / ticks);
        set_number(entity, "Utilization", active * 100 / ticks);
    }
    if ticks >= 1200 {
        set_number(entity, "MeterReady", 1);
        set_number(entity, "MeterTicks", 0);
        set_number(entity, "MeterActive", 0);
        set_number(entity, "MeterItems", 0);
    } else {
        set_number(entity, "MeterTicks", ticks);
        set_number(entity, "MeterActive", active);
        set_number(entity, "MeterItems", items);
    }
}

fn ingredients_ready(slots: &[Option<ItemStack>], recipe: &[(&str, u32)]) -> bool {
    slots.len() >= recipe.len() && slots.iter().zip(recipe).all(|(slot, &(item, count))| {
        slot.as_ref().is_some_and(|stack| is_plain(stack, item) && stack.count >= count)
    })
}

fn is_plain(stack: &ItemStack, item: &str) -> bool {
    stack.item.as_ref() == item && stack.damage == 0 && stack.extra.is_none()
}

fn can_add(slot: &Option<ItemStack>, item: &str, count: u32) -> bool {
    slot.as_ref()
        .is_none_or(|stack| is_plain(stack, item) && stack.count + count <= 64)
}

fn add(slot: &mut Option<ItemStack>, item: &str, count: u32) {
    if let Some(stack) = slot {
        stack.count += count;
    } else {
        *slot = Some(ItemStack::new(item, count));
    }
}

fn consume(slot: &mut Option<ItemStack>, count: u32) {
    if let Some(stack) = slot {
        stack.count -= count.min(stack.count);
        if stack.count == 0 {
            *slot = None;
        }
    }
}


#[cfg(test)]
mod tests;
#[cfg(test)]
mod perf;
