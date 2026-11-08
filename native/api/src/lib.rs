mod ambience;
mod scenery;
mod winter;
mod blueprint;
pub(crate) mod climate;
mod budget;
mod presentation;
mod appearance;
mod motion;
mod soundscape;
mod contracts;
mod freight;
mod logistics;
mod logistics_controls;
mod lift;
mod milestones;
mod inserters;
pub(crate) mod operations;
mod defense;
mod farm;
mod home;
mod pollution;
mod production;
mod power;
mod industry;
mod research;
mod stations;
mod rivers;
mod tutorial;
mod onboarding;
mod upgrades;
mod holding;
mod workforce;
mod wiki;

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
const EXPRESS_BELT: &str = "veio:express_belt";
const TUNNEL: &str = "veio:belt_tunnel";
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
/// Amostragem dos registros de mobs em `entity_tick_pre`.
const OBSERVE_EVERY: u64 = 7;


pub fn hooks() -> Box<dyn Hooks> {
    Box::new(Factory)
}

struct Factory;

impl Hooks for Factory {
    fn server_about_to_start(&self, _host: &mut dyn Host, _event: &mut horizon_client::mod_api::ServerAboutToStartEvent) {
        // A casa, as camas e os bônus em memória pertencem ao mundo que abre.
        // O painel carrega os próprios dados salvos quando começar a funcionar.
        home::reset();
        onboarding::world_changed();
        milestones::census::reset();
        blueprint::reset();
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
        if blueprint::owns(&block) {
            let held = host.held().map(|(item, _, _)| item);
            if let Some(flow) = blueprint::right_click(host, event.pos, &block, held.as_deref()) { return flow; }
        }
        if !is_factory_block(&block) {
            return Flow::Continue;
        }
        // Mão vazia chega como `None`. O quartel usa agachar com a mão vazia, então os postos
        // recebem "" nesse caso; o resto continua exigindo um item na mão.
        let held = host.held().map(|(item, _, _)| item);
        if lift::owns(&block)
            && let Some(flow) = lift::right_click(host, event.pos, held.as_deref().unwrap_or(""))
        {
            return flow;
        }
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
        if stations::is_station(&block) && let Some(channel) = stations::dye_channel(&held) {
            host.change_mod_block_entity(pos, &mut |entity| set_number(entity, "Channel", channel));
            host.play_sound("minecraft:item.dye.use", center(pos), 4, 0.6, 1.0);
            return Flow::Cancel;
        }
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
        if held == upgrades::MK2_KIT {
            let mut tier = 0;
            host.change_mod_block_entity(pos, &mut |entity| tier = number(entity, "Tier"));
            if !upgrades::supports(&block) || tier >= 2 || host.take_items(&held, 1) == 0 {
                return Flow::Continue;
            }
            host.change_mod_block_entity(pos, &mut |entity| set_number(entity, "Tier", 2));
            host.play_sound("minecraft:block.anvil.use", center(pos), 4, 0.6, 1.0);
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
        // Sem módulo, a chave agachada devolve o kit Mk.II.
        let mut tier = 0;
        host.change_mod_block_entity(pos, &mut |entity| tier = number(entity, "Tier"));
        if host.sneaking() && tier >= 2 && upgrades::supports(&block) {
            host.change_mod_block_entity(pos, &mut |entity| set_number(entity, "Tier", 0));
            host.give_items(upgrades::MK2_KIT, 1);
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
        block_entity_tick(host, event.pos, event.entity);
    }

    fn entity_tick_pre(&self, host: &mut dyn Host,
        event: &mut horizon_client::mod_api::EntityTickPreEvent<'_>) -> Flow
    {
        // Registro por amostragem: cada mob entra nos registros (defesa e rancho) em 1 de cada
        // `OBSERVE_EVERY` ticks do relógio do mod, não em todos. As consultas aceitam marcas de até
        // 40-60 ticks, então nada se perde; o número é ímpar para cobrir todos os ids mesmo que o
        // relógio avance de 2 em 2 (janela e servidor no mesmo processo).
        let now = defense::clock();
        // As caravanas acompanham seus animais em todo tick (`freight::caravan`).
        freight::caravan::observe(host, event.dimension, event.entity, event.kind);
        // Visitante que nenhum pedido da agenda conhece sai do mundo (de antes de reabrir, por exemplo).
        if event.kind == workforce::society::agenda::VISITOR && (now + u64::from(event.entity)) % 41 == 0 {
            workforce::society::agenda::visitor_seen(host, event.dimension, event.entity);
        }
        if (now + u64::from(event.entity)) % OBSERVE_EVERY == 0 {
            defense::observe(event.dimension, event.entity, event.kind);
            production::ranch::observe(event.dimension, event.entity, event.kind);
        }
        if event.kind == "minecraft:villager" {
            let now = defense::clock();
            workforce::village::observe(host, event.dimension, event.entity, now);
            workforce::village::scan_homes(host, event.dimension, event.entity, now);
        }
        Flow::Continue
    }

    fn chunk_load(&self, _host: &mut dyn Host, event: &mut horizon_client::mod_api::ChunkLoadEvent<'_>) {
        rivers::chunk_loaded(event.dimension, event.chunk, event.new, defense::clock());
    }

    fn player_logged_in(&self, host: &mut dyn Host, event: &mut horizon_client::mod_api::PlayerLoggedInEvent<'_>) {
        onboarding::logged_in(host);
        workforce::spawn::logged_in(event.player, event.name, defense::clock());
    }

    /// Clique num aldeão: escolher quem contratar para o posto que espera, ou (agachado, mão
    /// vazia) ver onde ele trabalha. Fora disso o comércio vanilla segue.
    /// Também: conversa com o prefeito do tutorial e com o visitante da agenda (em vez da troca).
    fn interact_entity(&self, host: &mut dyn Host, event: &mut horizon_client::mod_api::InteractEntityEvent<'_>) -> Flow {
        if event.kind == onboarding::MAYOR {
            onboarding::talk(host);
            return Flow::Cancel;
        }
        if event.kind == workforce::society::agenda::VISITOR
            && let Ok(id) = u32::try_from(event.entity)
            && workforce::society::agenda::talk(host, id)
        {
            return Flow::Cancel;
        }
        if event.kind != "minecraft:villager" || event.entity < 0 {
            return Flow::Continue;
        }
        let held = event.stack.filter(|stack| stack.count > 0).map(|stack| stack.item.to_string());
        workforce::pick::interact(host, event.entity as u32, held.as_deref()).unwrap_or(Flow::Continue)
    }

    fn trade_with_villager(&self, host: &mut dyn Host, event: &mut horizon_client::mod_api::TradeWithVillagerEvent<'_>) {
        if event.kind == "minecraft:villager" {
            workforce::society::traded(host, event.merchant, event.offer);
        }
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

    fn key(&self, host: &mut dyn Host, event: &mut horizon_client::mod_api::KeyEvent<'_>) {
        wiki::key(host, event.key, event.pressed);
    }

    fn screen_render_post(&self, host: &dyn Host, event: &mut horizon_client::mod_api::ScreenRenderPostEvent<'_>) {
        wiki::draw(host, event.batch, event.screen);
    }

    fn screen_mouse_button_pressed_pre(&self, host: &dyn Host,
        event: &mut horizon_client::mod_api::ScreenMouseButtonPressedPreEvent<'_>) -> Flow
    {
        if wiki::click(host, event.screen, event.button) { Flow::Cancel } else { Flow::Continue }
    }

    fn screen_mouse_scrolled_pre(&self, host: &dyn Host,
        event: &mut horizon_client::mod_api::ScreenMouseScrolledPreEvent<'_>) -> Flow
    {
        if wiki::scroll(host, event.screen, event.scroll[1]) { Flow::Cancel } else { Flow::Continue }
    }

    fn tick(&self, host: &mut dyn Host, _event: &mut horizon_client::mod_api::TickEvent) {
        // No jogo de um jogador só, o gancho roda duas vezes por tick: no servidor interno e na
        // janela (cliente). O Veio é todo do servidor; na janela, o relógio andaria em dobro e
        // fora de ordem, as cotas zerariam no meio do tick e um passo da poluição podia ser
        // gasto no cliente (onde não há jogadores nem `summon`), perdendo a onda da noite.
        if host.window() {
            scenery::client_tick(host);
            return;
        }
        defense::advance_clock();
        soundscape::refresh(host, defense::clock());
        budget::begin_tick(defense::clock());
        power::maintenance(defense::clock());
        workforce::maintenance(defense::clock());
        workforce::world_tick(host, defense::clock());
        rivers::tick(host, defense::clock());
        onboarding::server_tick(host);
        home::tick(host, defense::clock());
        milestones::census::tick(host, defense::clock());
        ambience::maintain(host, defense::clock());
        winter::tick(host, defense::clock());
        if let Some(time) = host.level(host.dimension()).and_then(|level| level.game_time(host)) {
            pollution::update(host, time);
            industry::maintain(host, time);
            stations::maintain(host, time);
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
        if blueprint::owns(event.block) { blueprint::bounds(event.entity, event.bounds); }
        if workforce::zones::is_marker(event.block) {
            workforce::zones::bounds(event.entity, event.bounds);
        }
        if event.block == workforce::SITE {
            workforce::site::bounds(event.entity, event.bounds);
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

    fn screen_click(&self, _host: &dyn Host, event: &mut horizon_client::mod_api::ScreenClickEvent<'_>) -> Option<i32> {
        // Aba da holding; nas outras páginas da vila (Atenção, Obras, Pedidos) não há saque.
        let first = event.data.first().copied().unwrap_or(0);
        let tab = if first < 1000 && (first / 100) % 10 != 0 { -1 } else { first / 1000 };
        if event.block == workforce::TOWN_HALL && !event.right && (3.0..17.0).contains(&event.at[1]) {
            return if (149.0..161.0).contains(&event.at[0]) {
                Some(workforce::society::agenda::TAB_PREVIOUS)
            } else if (162.0..175.0).contains(&event.at[0]) {
                Some(workforce::society::agenda::TAB_NEXT)
            } else {
                None
            };
        }
        (event.block == workforce::CONTROL_CENTER && !event.right).then(|| holding::click(event.at, tab)).flatten()
    }

    fn menu_button(&self, host: &mut dyn Host, event: &mut horizon_client::mod_api::MenuButtonEvent<'_>) {
        // Prefeitura: troca entre os indicadores e a agenda.
        if event.entity.id == workforce::TOWN_HALL
            && matches!(event.id, workforce::society::agenda::TAB_PREVIOUS | workforce::society::agenda::TAB_NEXT)
        {
            set_number(event.entity, "Tab", 1 - number(event.entity, "Tab").clamp(0, 1));
            return;
        }
        if event.entity.id == workforce::CONTROL_CENTER {
            let time = host.level(host.dimension()).and_then(|level| level.game_time(host)).unwrap_or(0);
            holding::press(host.dimension(), event.pos, event.entity, event.id, time);
        }
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
        is_factory_block(&event.entity.id).then(|| if blueprint::owns(&event.entity.id) {
            blueprint::client_tag(event.entity)
        } else {
            event.entity.nbt.clone()
        })
    }

    fn block_entity_submit(&self, host: &dyn Host,
        event: &mut horizon_client::mod_api::BlockEntitySubmitEvent<'_>)
    {
        presentation::draw(host, event);
    }

    /// Luz colorida das máquinas quentes trabalhando, para o pacote de shaders.
    #[allow(elided_lifetimes_in_paths)]
    fn visuals(&self, host: &dyn Host, event: &mut horizon_client::mod_api::VisualsEvent) {
        ambience::visuals(host, event.lights);
    }

    /// Estação, temperatura e fumaça da fábrica para o pacote de shaders (`frame.mods`).
    #[allow(elided_lifetimes_in_paths)]
    fn shader_params(&self, host: &dyn Host, event: &mut horizon_client::mod_api::ShaderParamsEvent) {
        ambience::shader_params(host, event.params);
    }

    /// Grama, folhas e água de cada bioma com as cores da estação (cliente).
    #[allow(elided_lifetimes_in_paths)]
    fn biome_colors(&self, _host: &dyn Host, event: &mut horizon_client::mod_api::BiomeColorsEvent) {
        scenery::biome_colors(event.biome, event.colors);
    }

    /// Neblina das manhãs úmidas e fumaça das fábricas na névoa do jogo (cliente).
    #[allow(elided_lifetimes_in_paths)]
    fn render_fog(&self, host: &dyn Host, event: &mut horizon_client::mod_api::RenderFogEvent) {
        scenery::render_fog(host, event.camera, event.fog_type, event.fog);
    }

}

/// Tick de um bloco do Veio. Fora do gancho para as medições chamarem o mesmo caminho.
fn block_entity_tick(host: &mut dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity) {
    if !is_factory_block(&entity.id) {
        return;
    }
    let Some(time) = host.level(host.dimension()).and_then(|level| level.game_time(host)) else {
        return;
    };
    pollution::update(host, time);
    if power::tick(host, pos, entity, time) {
        host.block_entity_sync(pos);
    }
    let previous = visual_signature(entity);
    let produced = number(entity, "Produced");
    let previous_status = number(entity, "Status");
    let training = onboarding::tracking().then(|| {
        onboarding::before_machine(host, pos, entity);
        (number(entity, "Total"), if entity.id == HUB { number(entity, "ContractStage") } else { 0 })
    });
    tick_factory(host, pos, entity, time);
    if entity.id == freight::DRONE {
        set_number(entity, "FlightVisualAt", stamp(time));
    }
    if let Some((cycles, contract)) = training {
        onboarding::observe_production(host.dimension(), pos, entity, produced, cycles, contract, time);
    }
    milestones::census::observe(host.dimension(), pos, entity, previous_status,
        number(entity, "Produced") - produced, defense::clock());
    industry::observe(host.dimension(), pos, entity, time);
    holding::observe(host.dimension(), pos, entity, time);
    pollution::emit(host, pos, entity, time);
    soundscape::after(host, pos, entity, produced, previous_status, time);
    ambience::observe(host, pos, entity, defense::clock());
    record_performance(entity, produced);
    let cargo = presentation::publish(entity);
    if cargo || published_signature(entity) != previous {
        host.block_entity_sync(pos);
    }
}

fn is_factory_block(block: &str) -> bool {
    matches!(block, DRILL | DRILL_MK2 | PRESS | ASSEMBLER | BELT | FAST_BELT | EXPRESS_BELT | TUNNEL |
        SPLITTER | MERGER | lift::LIFT | CUTTER | INTEGRATOR | GENERATOR | CABLE | SOLAR | BATTERY |
        GEOTHERMAL | CRUSHER | ALLOY | ELECTRONICS | HUB | HARVESTER | TESLA | POLE | STEEL_POLE | inserters::ID | inserters::FAST | inserters::LONG)
        || production::is_machine(block) || workforce::owns(block) || freight::owns(block) || milestones::owns(block) || blueprint::owns(block)
        || matches!(block, research::LAB | industry::MONITOR | industry::ANCHOR) || stations::is_station(block)
}

/// Consumidores: recebem energia por cabos, param com redstone e mostram métricas.
fn needs_power(block: &str) -> bool {
    matches!(block, DRILL | DRILL_MK2 | PRESS | ASSEMBLER | CUTTER | INTEGRATOR |
        CRUSHER | ALLOY | ELECTRONICS | HARVESTER | TESLA | inserters::ID | inserters::FAST | inserters::LONG)
        || production::consumes(block) || block == freight::DRONE || matches!(block, research::LAB | industry::ANCHOR | stations::SENDER)
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
        BELT | FAST_BELT | EXPRESS_BELT | SPLITTER | TUNNEL | lift::LIFT => (1, vec![0], vec![0]),
        MERGER => (2, vec![0, 1], vec![0, 1]),
        GENERATOR => (1, vec![0], vec![]),
        GEOTHERMAL => (2, vec![0], vec![1]),
        SOLAR | BATTERY | TESLA | inserters::ID | inserters::FAST | inserters::LONG => (1, vec![], vec![]),
        CABLE | POLE | STEEL_POLE => (0, vec![], vec![]),
        HUB => (4, vec![0, 1], vec![2, 3]),
        HARVESTER => (4, vec![0], vec![1, 2, 3]),
        research::LAB => (2, vec![0, 1], vec![]),
        industry::MONITOR | industry::ANCHOR => (1, vec![], vec![]),
        stations::SENDER => (4, vec![0, 1, 2, 3], vec![]),
        stations::RECEIVER => (4, vec![], vec![0, 1, 2, 3]),
        _ if freight::owns(block) => return freight::layout(block),
        _ if milestones::owns(block) => milestones::layout(),
        _ if workforce::owns(block) => return workforce::layout(block),
        _ => return production::layout(block),
    })
}

/// Há slot de entrada de itens? Evita alocar o `layout` só para decidir uma curva de esteira.
fn has_item_inputs(block: &str) -> bool {
    match block {
        BELT | FAST_BELT | EXPRESS_BELT | SPLITTER | MERGER | TUNNEL | lift::LIFT | PRESS | ASSEMBLER | CUTTER | CRUSHER |
        INTEGRATOR | ALLOY | ELECTRONICS | GENERATOR | GEOTHERMAL | HUB | HARVESTER | research::LAB | stations::SENDER => true,
        DRILL | DRILL_MK2 | SOLAR | BATTERY | TESLA | CABLE | POLE | STEEL_POLE | inserters::ID | inserters::FAST | inserters::LONG
            | industry::MONITOR | industry::ANCHOR | stations::RECEIVER => false,
        _ if freight::owns(block) => true,
        _ if workforce::is_post(block) || workforce::storage::is_store(block) || milestones::owns(block) => true,
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
        research::LAB | industry::MONITOR | industry::ANCHOR | stations::SENDER => 0..0,
        stations::RECEIVER => 0..4,
        GENERATOR | SOLAR | BATTERY | TESLA | CABLE | POLE | STEEL_POLE | inserters::ID | inserters::FAST | inserters::LONG => 0..0,
        _ if freight::owns(block) => 3..6,
        _ if production::is_machine(block) => production::outputs(block),
        _ if workforce::owns(block) => workforce::outputs(block),
        _ if milestones::owns(block) => 1..2,
        _ => 0..1,
    }
}

fn accepts(block: &str, slot: usize, stack: &ItemStack) -> bool {
    match (block, slot) {
        (HUB, _) => contracts::valid_input(slot, stack),
        (research::LAB, _) => research::accepts(slot, stack),
        (stations::SENDER, 0..4) => stack.damage == 0 && stack.extra.is_none(),
        (stations::RECEIVER, _) => false,
        (milestones::BOARD, _) => milestones::valid_input(slot, stack),
        (BELT | FAST_BELT | EXPRESS_BELT | SPLITTER | TUNNEL | lift::LIFT, 0) => true,
        (MERGER, 0 | 1) => true,
        _ if freight::owns(block) => (slot < 6 || block == freight::caravan::ID && slot == 7 && freight::caravan::feed(&stack.item)) && stack.damage == 0 && stack.extra.is_none(),
        _ if workforce::owns(block) => workforce::accepts(block, slot, stack),
        // Entradas das máquinas de produção, pela tabela de receitas e combustíveis.
        _ => production::accepts(block, slot, stack),
    }
}

/// Chaves NBT lidas todo tick. A primeira vez monta o `JavaString`; as seguintes reaproveitam.
/// Sempre embutida: com o nome literal, o compilador resolve o `match` na compilação e a
/// leitura vira um acesso direto à chave pronta, sem comparar textos a cada tick.
#[inline(always)]
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
        "Age" => key!("Age"),
        "BusyAt" => key!("BusyAt"),
        "Carried" => key!("Carried"),
        "CustomName" => key!("CustomName"),
        "Edits" => key!("Edits"),
        "Fertility" => key!("Fertility"),
        "Water" => key!("Water"),
        "WaterPort" => key!("WaterPort"),
        "Held" => key!("Held"),
        "ScanArea" => key!("ScanArea"),
        "Skipped" => key!("Skipped"),
        "UUID" => key!("UUID"),
        "Worker" => key!("Worker"),
        "count" => key!("count"),
        "id" => key!("id"),
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
        "HaulSearchPending" => key!("HaulSearchPending"),
        "HaulTaskDesk" => key!("HaulTaskDesk"),
        "HaulTaskId" => key!("HaulTaskId"),
        "from" => key!("from"),
        "slot" => key!("slot"),
        "item" => key!("item"),
        "to" => key!("to"),
        "into" => key!("into"),
        "by" => key!("by"),
        "seen" => key!("seen"),

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
        "HeldSince" => key!("HeldSince"),
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
        // Painel do centro, obras da vila e lar do jogador.
        "Rot" => key!("Rot"),
        "Survey" => key!("Survey"),
        "Wait" => key!("Wait"),
        "Cursor" => key!("Cursor"),
        "Footing" => key!("Footing"),
        "Standing" => key!("Standing"),
        "Erected" => key!("Erected"),
        "Research" => key!("Research"),
        "Operating" => key!("Operating"),
        "Techs" => key!("Techs"),
        "RWork" => key!("RWork"),
        "Guests" => key!("Guests"),
        "Probe" => key!("Probe"),
        "Class" => key!("Class"),
        "Taught" => key!("Taught"),
        "Install" => key!("Install"),
        "Mail" => key!("Mail"),
        "Delivered" => key!("Delivered"),
        "Shift" => key!("Shift"),
        "Made" => key!("Made"),
        "Installed" => key!("Installed"),
        "LabScan" => key!("LabScan"),
        "LabFound" => key!("LabFound"),
        "Lab" => key!("Lab"),
        "Prestige" => key!("Prestige"),
        "Treasury" => key!("Treasury"),
        "Morning" => key!("Morning"),
        "TasksAt" => key!("TasksAt"),
        "PopAt" => key!("PopAt"),
        "ReqToday" => key!("ReqToday"),
        "Req0" => key!("Req0"),
        "Req1" => key!("Req1"),
        "Req2" => key!("Req2"),
        "Req3" => key!("Req3"),
        "BanquetDay" => key!("BanquetDay"),
        "DD1" => key!("DD1"),
        "DD2" => key!("DD2"),
        "DD3" => key!("DD3"),
        "DD4" => key!("DD4"),
        "DD5" => key!("DD5"),
        "DD6" => key!("DD6"),
        "DD7" => key!("DD7"),
        "DD8" => key!("DD8"),
        "DD9" => key!("DD9"),
        "DD10" => key!("DD10"),
        "DD11" => key!("DD11"),
        "Page" => key!("Page"),
        "Tick" => key!("Tick"),
        "Market" => key!("Market"),
        "Day" => key!("Day"),
        "LastHour" => key!("LastHour"),
        "Built" => key!("Built"),
        "FestivalDay" => key!("FestivalDay"),
        "Office" => key!("Office"),
        "Todo" => key!("Todo"),
        "Comfort" => key!("Comfort"),
        "Have" => key!("Have"),
        "Decor" => key!("Decor"),
        "Finish" => key!("Finish"),
        "Streak" => key!("Streak"),
        "MealWait" => key!("MealWait"),
        "MealAt" => key!("MealAt"),
        "MealDay" => key!("MealDay"),
        "RewardDay" => key!("RewardDay"),
        "SeenDay" => key!("SeenDay"),
        "Warm" => key!("Warm"),
        "Cook" => key!("Cook"),
        "Menu" => key!("Menu"),
        "Owner" => key!("Owner"),
        "CScan" => key!("CScan"),
        "CHave" => key!("CHave"),
        "CDecor" => key!("CDecor"),
        "CFinish" => key!("CFinish"),
        "Up0" => key!("Up0"),
        "Up1" => key!("Up1"),
        "Up2" => key!("Up2"),
        "DA1" => key!("DA1"),
        "DA2" => key!("DA2"),
        "DA3" => key!("DA3"),
        "DA4" => key!("DA4"),
        "DA5" => key!("DA5"),
        "DA6" => key!("DA6"),
        "DA7" => key!("DA7"),
        "DA8" => key!("DA8"),
        "DA9" => key!("DA9"),
        "DA10" => key!("DA10"),
        "DA11" => key!("DA11"),
        "DB1" => key!("DB1"),
        "DB2" => key!("DB2"),
        "DB3" => key!("DB3"),
        "DB4" => key!("DB4"),
        "DB5" => key!("DB5"),
        "DB6" => key!("DB6"),
        "DB7" => key!("DB7"),
        "DB8" => key!("DB8"),
        "DB9" => key!("DB9"),
        "DB10" => key!("DB10"),
        "DB11" => key!("DB11"),
        "DC1" => key!("DC1"),
        "DC2" => key!("DC2"),
        "DC3" => key!("DC3"),
        "DC4" => key!("DC4"),
        "DC5" => key!("DC5"),
        "DC6" => key!("DC6"),
        "DC7" => key!("DC7"),
        "DC8" => key!("DC8"),
        "DC9" => key!("DC9"),
        "DC10" => key!("DC10"),
        "DC11" => key!("DC11"),
        "Obra0_0" => key!("Obra0_0"),
        "Obra0_1" => key!("Obra0_1"),
        "Obra0_2" => key!("Obra0_2"),
        "Obra1_0" => key!("Obra1_0"),
        "Obra1_1" => key!("Obra1_1"),
        "Obra1_2" => key!("Obra1_2"),
        "Obra2_0" => key!("Obra2_0"),
        "Obra2_1" => key!("Obra2_1"),
        "Obra2_2" => key!("Obra2_2"),
        "Obra3_0" => key!("Obra3_0"),
        "Obra3_1" => key!("Obra3_1"),
        "Obra3_2" => key!("Obra3_2"),
        "Obra4_0" => key!("Obra4_0"),
        "Obra4_1" => key!("Obra4_1"),
        "Obra4_2" => key!("Obra4_2"),
        "energy" => key!("energy"),
        "Company" => key!("Company"),
        "DAssist" => key!("DAssist"),
        "DEmploy" => key!("DEmploy"),
        "DFamilies" => key!("DFamilies"),
        "DGini" => key!("DGini"),
        "DIdh" => key!("DIdh"),
        "DMade" => key!("DMade"),
        "DPop" => key!("DPop"),
        "DRequest" => key!("DRequest"),
        "DRequestN" => key!("DRequestN"),
        "DSold" => key!("DSold"),
        "DSuspended" => key!("DSuspended"),
        "DTax" => key!("DTax"),
        "DUpkeep" => key!("DUpkeep"),
        "DWages" => key!("DWages"),
        "Debt" => key!("Debt"),
        "Demand" => key!("Demand"),
        "Employed" => key!("Employed"),
        "Family" => key!("Family"),
        "Fed" => key!("Fed"),
        "Hungry" => key!("Hungry"),
        "Income" => key!("Income"),
        "Kids" => key!("Kids"),
        "Maintained" => key!("Maintained"),
        "OfferFor" => key!("OfferFor"),
        "OfferSig" => key!("OfferSig"),
        "Owed" => key!("Owed"),
        "OwedPublic" => key!("OwedPublic"),
        "Payroll" => key!("Payroll"),
        "Phase" => key!("Phase"),
        "Present" => key!("Present"),
        "Prices" => key!("Prices"),
        "Public" => key!("Public"),
        "Round" => key!("Round"),
        "Short" => key!("Short"),
        "Sold" => key!("Sold"),
        "SoldToday" => key!("SoldToday"),
        "Taxes" => key!("Taxes"),
        "Till" => key!("Till"),
        "Turned" => key!("Turned"),
        "UpkeepHalf" => key!("UpkeepHalf"),
        "Wallet" => key!("Wallet"),
        "Wealth" => key!("Wealth"),
        "Morale" => key!("Morale"),
        "Agenda" => key!("Agenda"),
        "AgendaDay" => key!("AgendaDay"),
        "HolidayToday" => key!("HolidayToday"),
        "Tier" => key!("Tier"),
        "RecipeIndex" => key!("RecipeIndex"),
        "Tab" => key!("Tab"),
        "HData" => key!("HData"),
        "Cash" => key!("Cash"),
        "Known" => key!("Known"),
        "Payable" => key!("Payable"),
        "Revenue" => key!("Revenue"),
        "DExport" => key!("DExport"),
        "Trade" => key!("Trade"),
        "TradeRecent" => key!("TradeRecent"),
        "BoxHeight" => key!("BoxHeight"),
        "BoxSize" => key!("BoxSize"),
        "CarryInto" => key!("CarryInto"),
        "CarryKind" => key!("CarryKind"),
        "Credit" => key!("Credit"),
        "DesignBlocks" => key!("DesignBlocks"),
        "DesignFacing" => key!("DesignFacing"),
        "DesignFootprint" => key!("DesignFootprint"),
        "DesignHeight" => key!("DesignHeight"),
        "DesignSkipped" => key!("DesignSkipped"),
        "Down" => key!("Down"),
        "Gust" => key!("Gust"),
        "Hired" => key!("Hired"),
        "Linked" => key!("Linked"),
        "Order" => key!("Order"),
        "OrderDone" => key!("OrderDone"),
        "OrderPaid" => key!("OrderPaid"),
        "Searched" => key!("Searched"),
        "Size" => key!("Size"),
        "DBlocked" => key!("DBlocked"),
        "DDamaged" => key!("DDamaged"),
        "DPower" => key!("DPower"),
        "DSetup" => key!("DSetup"),
        "DTotal" => key!("DTotal"),
        _ => rare_nbt_key(name),
    }
}

/// Chave rara: vive até o fim do processo, para o tick seguinte não alocar de novo.
#[cold]
#[inline(never)]
fn rare_nbt_key(name: &str) -> &'static JavaString {
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

#[inline(always)]
pub(crate) fn with_nbt_key<T>(name: &str, read: impl FnOnce(&JavaString) -> T) -> T {
    read(nbt_key(name))
}

/// `JavaString` igual a `value`, sem montar uma `String` no meio.
pub(crate) fn java_eq(text: &JavaString, value: &str) -> bool {
    let mut units = text.0.iter().copied();
    value.encode_utf16().all(|unit| units.next() == Some(unit)) && units.next().is_none()
}

// `number`/`set_number` resolvem a chave no local da chamada; a busca no NBT fica numa
// função só, para não copiar o código do mapa em cada chamada.
#[inline(always)]
fn number(entity: &ModBlockEntity, key: &str) -> i32 {
    number_at(entity, nbt_key(key))
}

fn number_at(entity: &ModBlockEntity, key: &JavaString) -> i32 {
    match entity.nbt.get(key) {
        Some(Tag::Int(value)) => (*value).max(0),
        _ => 0,
    }
}

#[inline(always)]
fn set_number(entity: &mut ModBlockEntity, key: &str, value: i32) {
    set_number_at(entity, nbt_key(key), value);
}

fn set_number_at(entity: &mut ModBlockEntity, key: &JavaString, value: i32) {
    if let Some(tag) = entity.nbt.get_mut(key) {
        let current = match tag { Tag::Int(stored) => (*stored).max(0), _ => 0 };
        if current != value { *tag = Tag::Int(value); }
    } else {
        entity.nbt.insert(key.clone(), Tag::Int(value));
    }
}

/// Vários inteiros do NBT numa passada só pelo mapa, cada um como em `number`. `keys` vem em
/// ordem crescente, a mesma do `BTreeMap`: a passada compara cada chave no máximo uma vez.
fn numbers_sorted<const N: usize>(entity: &ModBlockEntity, keys: &[&JavaString; N]) -> [i32; N] {
    debug_assert!(keys.is_sorted());
    let mut values = [0; N];
    let mut wanted = 0;
    'entries: for (key, tag) in &entity.nbt {
        while wanted < N {
            match keys[wanted].cmp(key) {
                std::cmp::Ordering::Less => wanted += 1,
                std::cmp::Ordering::Equal => {
                    values[wanted] = match tag { Tag::Int(value) => (*value).max(0), _ => 0 };
                    wanted += 1;
                    continue 'entries;
                }
                std::cmp::Ordering::Greater => continue 'entries,
            }
        }
        break;
    }
    values
}

/// O que o desenho no mundo lê do NBT, além da carga (que `presentation::publish` acusa).
/// O braço anima pelo `MovedAt`.
/// Drones e caravanas animam pela fase do voo; máquinas que queimam mostram a chama.
fn visual_signature(entity: &ModBlockEntity) -> (i32, i32, i32, bool) {
    let motion = if matches!(entity.id.as_str(), freight::DRONE | freight::caravan::ID) {
        number(entity, "FlightPhase") * 1_000_000 + number(entity, "Progress") / 4
    } else if entity.id == HUB {
        i32::from(contracts::beacon_selected(entity))
    } else {
        number(entity, if inserters::is_arm(&entity.id) { "MovedAt" } else { "Info" })
    };
    (number(entity, "Status"), number(entity, "Module"), motion, burning(entity))
}

fn burning(entity: &ModBlockEntity) -> bool {
    production::burns(&entity.id) && number(entity, "Burn") > 0
}

/// A mesma assinatura logo depois de `presentation::publish`: o painel acabou de copiar
/// `Status`, `Module` e `Info` para `data[0]`, `data[11]` e `data[12]`, sem novas buscas no NBT.
/// Quem publica outra coisa nessas posições (ou não publica) relê o NBT.
fn published_signature(entity: &ModBlockEntity) -> (i32, i32, i32, bool) {
    let standard = !matches!(entity.id.as_str(), CABLE | POLE | STEEL_POLE | TESLA | workforce::CONTROL_CENTER | workforce::TOWN_HALL)
        && !inserters::is_arm(&entity.id) && !workforce::zones::is_marker(&entity.id)
        && !freight::owns(&entity.id) && !milestones::owns(&entity.id);
    match entity.data.as_slice() {
        [status, .., module, info] if standard && entity.data.len() == presentation::DATA_SLOTS => (*status, *module, *info, burning(entity)),
        _ => visual_signature(entity),
    }
}

/// Hash multiplicativo (o do rustc) para chaves pequenas consultadas todo tick: chunks e
/// posições. Os mapas que o usam só são consultados por chave, nunca percorridos em ordem.
#[derive(Default, Clone, Copy)]
pub(crate) struct FastHasher(u64);
pub(crate) type FastHash = std::hash::BuildHasherDefault<FastHasher>;

impl FastHasher {
    #[inline]
    fn add(&mut self, word: u64) {
        self.0 = (self.0.rotate_left(5) ^ word).wrapping_mul(0x51_7c_c1_b7_27_22_0a_95);
    }
}

impl std::hash::Hasher for FastHasher {
    fn finish(&self) -> u64 { self.0 }
    fn write(&mut self, bytes: &[u8]) {
        let mut chunks = bytes.chunks_exact(8);
        for chunk in &mut chunks { self.add(u64::from_le_bytes(chunk.try_into().expect("8 bytes"))); }
        for byte in chunks.remainder() { self.add(u64::from(*byte)); }
    }
    fn write_u8(&mut self, value: u8) { self.add(u64::from(value)); }
    fn write_u32(&mut self, value: u32) { self.add(u64::from(value)); }
    fn write_i32(&mut self, value: i32) { self.add(u64::from(value as u32)); }
    fn write_u64(&mut self, value: u64) { self.add(value); }
    fn write_usize(&mut self, value: usize) { self.add(value as u64); }
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
    if lift::owns(&entity.id) { return lift::tick(host, pos, entity, time); }
    if blueprint::owns(&entity.id) { return blueprint::tick(host, pos, entity); }
    logistics::transfer_outputs(host, pos, entity, time);
    if entity.id == HUB {
        if contracts::tick(entity) && industry::on_server(host) {
            host.run_command("advancement grant @a only veio:grand_project");
            for player in host.server_players() {
                host.tell(player.uuid, horizon_client::mod_api::Text::tr("veio.project.done", &[]));
            }
        }
        return;
    }
    if entity.id == industry::MONITOR {
        return industry::tick_monitor(host, pos, entity, time);
    }
    if milestones::owns(&entity.id) {
        return milestones::tick(host, pos, entity);
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
    if entity.id == research::LAB {
        return research::tick(host, entity, time);
    }
    if entity.id == industry::ANCHOR {
        return industry::tick_anchor(host, pos, entity, time);
    }
    if stations::is_station(&entity.id) {
        return stations::tick(host, pos, entity, time);
    }
    // Extração, receitas, lavouras, rancho e armazém: módulo de produção.
    production::tick(host, pos, entity, time);
}

fn tick_power(host: &mut dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity) {
    let limit = capacity(&entity.id);
    let mut energy = number(entity, "energy").min(limit);
    // Geração (produção); a distribuição abaixo é da logística.
    let stored = energy;
    energy = production::generation::produce(host, pos, entity, energy, limit);
    soundscape::generated(host, pos, entity, energy > stored);
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
