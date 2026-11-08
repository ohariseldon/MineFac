//! Substituto mínimo de `horizon_client::mod_api` para compilar e testar o Veio sem rede.
//!
//! As assinaturas foram copiadas do Horizon `66f3981f522b24c250b21523a70080965b940424`
//! (`native-hooks.json`, `tools/native-hook-fragments/` e os `native_*` de `crates/horizon-client`).
//! Só existem aqui os tipos e métodos que o Veio usa. Quando o Veio passar a usar outro método
//! do Host ou outro gancho, copie a assinatura real para cá. A compilação pelo launcher continua
//! sendo a referência.
#![allow(clippy::all, dead_code, unused_variables)]

pub mod mod_api {
    use std::collections::BTreeMap;
    use std::sync::Arc;

    // ---------- NBT ----------
    #[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct JavaString(pub Vec<u16>);
    impl From<&str> for JavaString {
        fn from(value: &str) -> Self {
            Self(value.encode_utf16().collect())
        }
    }
    impl JavaString {
        pub fn to_string_lossy(&self) -> String {
            String::from_utf16_lossy(&self.0)
        }
    }
    #[derive(Clone, Debug, PartialEq)]
    pub enum Tag {
        End,
        Byte(i8),
        Short(i16),
        Int(i32),
        Long(i64),
        Float(u32),
        Double(u64),
        ByteArray(Vec<u8>),
        String(JavaString),
        List(Vec<Tag>),
        Compound(Compound),
        IntArray(Vec<i32>),
        LongArray(Vec<i64>),
    }
    pub type Compound = BTreeMap<JavaString, Tag>;

    // ---------- Itens ----------
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct WrittenBook {
        pub title: String,
        pub author: String,
        pub generation: u8,
        pub pages: Vec<String>,
    }
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct StackData {
        pub written: Option<WrittenBook>,
    }
    #[derive(Clone, Debug, PartialEq)]
    pub struct ItemStack {
        pub item: Arc<str>,
        pub count: u32,
        pub damage: u32,
        pub extra: Option<Box<StackData>>,
    }
    impl ItemStack {
        pub fn new(item: &str, count: u32) -> Self {
            Self { item: item.into(), count, damage: 0, extra: None }
        }
    }

    // ---------- Geometria ----------
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub enum Direction {
        Down,
        Up,
        North,
        South,
        West,
        East,
    }
    impl Direction {
        pub const ALL: [Self; 6] =
            [Self::Down, Self::Up, Self::North, Self::South, Self::West, Self::East];
        pub const fn opposite(self) -> Self {
            match self {
                Self::Down => Self::Up,
                Self::Up => Self::Down,
                Self::North => Self::South,
                Self::South => Self::North,
                Self::West => Self::East,
                Self::East => Self::West,
            }
        }
        pub const fn normal(self) -> [i32; 3] {
            match self {
                Self::Down => [0, -1, 0],
                Self::Up => [0, 1, 0],
                Self::North => [0, 0, -1],
                Self::South => [0, 0, 1],
                Self::West => [-1, 0, 0],
                Self::East => [1, 0, 0],
            }
        }
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
    pub struct BlockStateId(pub u32);

    // ---------- Texto ----------
    #[derive(Clone, Debug, PartialEq)]
    pub struct Text {
        plain: String,
    }
    impl Text {
        pub fn literal(s: impl Into<String>) -> Self {
            Self { plain: s.into() }
        }
        /// O substituto não tem traduções: devolve a chave, como o jogo faz com chaves ausentes.
        pub fn tr(key: &str, args: &[&dyn std::fmt::Display]) -> Self {
            let _ = args;
            Self { plain: key.to_owned() }
        }
        pub fn plain(&self) -> String {
            self.plain.clone()
        }
    }

    // ---------- Blocos com entidade ----------
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct ModBlockEntityFields {
        pub id: String,
        pub nbt: Compound,
        pub items: Vec<Option<ItemStack>>,
        pub data: Vec<i32>,
    }
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct ModBlockEntity {
        fields: ModBlockEntityFields,
    }
    impl std::ops::Deref for ModBlockEntity {
        type Target = ModBlockEntityFields;
        fn deref(&self) -> &ModBlockEntityFields {
            &self.fields
        }
    }
    impl std::ops::DerefMut for ModBlockEntity {
        fn deref_mut(&mut self) -> &mut ModBlockEntityFields {
            &mut self.fields
        }
    }
    #[derive(Clone, Debug, Default, PartialEq, Eq)]
    pub struct ModContainer {
        pub slots: usize,
        pub faces: [Vec<usize>; 6],
        pub inputs: Vec<usize>,
        pub outputs: Vec<usize>,
        pub menu: Option<String>,
    }
    #[derive(Clone, Debug, Default, PartialEq, Eq)]
    pub struct ModEnergyBuffer {
        pub capacity: i32,
        pub max_insert: i32,
        pub max_extract: i32,
        pub insert_faces: [bool; 6],
        pub extract_faces: [bool; 6],
    }

    // ---------- Telas e desenho ----------
    #[derive(Clone, Debug, Default)]
    pub struct NativeScreen {
        pub width: f32,
        pub height: f32,
        pub slots: Vec<Option<[f32; 2]>>,
        pub inventory_y: f32,
        pub inventory_x: f32,
        pub own_labels: bool,
        pub own_background: bool,
    }
    #[derive(Default)]
    pub struct HudBatch;
    // Offline render probe only: command traces let tests verify cached/GPU
    // movement and export geometry; none of these fields belong to the real API.
    #[derive(Clone, Debug)]
    pub enum DrawOp {
        Push, Pop, Still, Translate([f32;3]), Scale([f32;3]),
        Rotate([f32;3], f32, [f32;3]), Spin([f32;3], [f32;3], f32, f32),
    }
    #[derive(Default)]
    pub struct BlockEntityDraw {
        pub partial: f32,
        pub game_time: i64,
        pub light: u32,
        pub trace: Vec<DrawOp>,
        pub kept: bool,
    }
    impl BlockEntityDraw {
        pub fn keep(&mut self) { self.kept = true; }
        /// Só no substituto: o desenho pediu `keep` (o jogo o reaproveitaria).
        pub fn preview_kept(&self) -> bool { self.kept }
        pub fn spin(&mut self, axis: [f32;3], pivot: [f32;3], rate: f32, phase: f32) { self.trace.push(DrawOp::Spin(axis,pivot,rate,phase)); }
        pub fn still(&mut self) { self.trace.push(DrawOp::Still); }
        pub fn push(&mut self) { self.trace.push(DrawOp::Push); }
        pub fn pop(&mut self) { self.trace.push(DrawOp::Pop); }
        pub fn translate(&mut self, by: [f32;3]) { self.trace.push(DrawOp::Translate(by)); }
        pub fn rotate_about(&mut self, axis: [f32;3], angle: f32, pivot: [f32;3]) { self.trace.push(DrawOp::Rotate(axis,angle,pivot)); }
        pub fn scale(&mut self, by: [f32;3]) { self.trace.push(DrawOp::Scale(by)); }
    }
    #[derive(Clone, Copy, Debug)]
    pub struct CubeFace<'a> {
        pub sprite: &'a str,
        pub uv: Option<[f32; 4]>,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum DisplayContext {
        None,
        ThirdPersonLeftHand,
        ThirdPersonRightHand,
        FirstPersonLeftHand,
        FirstPersonRightHand,
        Head,
        Gui,
        Ground,
        Fixed,
        OnShelf,
    }

    // ---------- Partículas ----------
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub enum ParticleType {
        Smoke,
        Flame,
        ElectricSpark,
        HappyVillager,
        Composter,
        Crit,
        LargeSmoke,
        CampfireCosySmoke,
        Cloud,
        Lava,
    }
    #[derive(Clone, Debug)]
    pub struct ParticleOptions {
        pub ty: ParticleType,
    }
    impl ParticleOptions {
        pub fn simple(ty: ParticleType) -> Self {
            Self { ty }
        }
    }

    // ---------- Handles ----------
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct EntityRef {
        id: u32,
    }
    impl EntityRef {
        pub fn new(id: u32) -> Self {
            Self { id }
        }
        pub fn id(self) -> u32 {
            self.id
        }
        pub fn position(self, host: &dyn Host) -> Option<[f64; 3]> {
            host.entity_position(self)
        }
    }
    /// `mod_api::LivingRef` (Horizon 66f3981): uma entidade viva, de `Host::living`.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct LivingRef(EntityRef);
    impl LivingRef {
        pub fn new(entity: EntityRef) -> Self {
            Self(entity)
        }
        pub fn entity(self) -> EntityRef {
            self.0
        }
        pub fn id(self) -> u32 {
            self.0.id()
        }
    }
    /// `horizon_world::player_inventory::EquipmentSlot`, reexportado por `mod_api`.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum EquipmentSlot { MainHand, OffHand, Feet, Legs, Chest, Head, Body, Saddle }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct LevelRef {
        dimension: &'static str,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct NativeWorldEpoch(u64);
    impl NativeWorldEpoch {
        pub fn new(n: u64) -> Self { Self(n) }
        pub fn get(self) -> u64 { self.0 }
    }
    impl LevelRef {
        pub fn epoch(self) -> NativeWorldEpoch { NativeWorldEpoch::new(1) }
        pub fn new(dimension: &'static str) -> Self {
            Self { dimension }
        }
        pub fn game_time(self, host: &dyn Host) -> Option<i64> {
            host.level_game_time(self)
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Flow {
        Continue,
        Cancel,
    }

    // ---------- Host: métodos com o nome e a assinatura do jogo ----------
    /// Each method's default panics with its name, as in the generated trait.
    pub trait Host {
        fn mod_block_entity(&self, pos: [i32; 3]) -> Option<ModBlockEntity> { unimplemented!("mod_block_entity") }
        fn block(&self, pos: [i32; 3]) -> String {
            unimplemented!("block")
        }
        fn block_property(&self, pos: [i32; 3], key: &str) -> Option<String> {
            unimplemented!("block_property")
        }
        fn set_block_property(&mut self, pos: [i32; 3], key: &str, value: &str) -> bool {
            unimplemented!("set_block_property")
        }
        fn held(&self) -> Option<(String, u32, bool)> {
            unimplemented!("held")
        }
        fn level(&self, dimension: &str) -> Option<LevelRef> {
            unimplemented!("level")
        }
        fn level_is_loaded(&self, level: LevelRef, pos: [i32; 3]) -> bool { unimplemented!("level_is_loaded") }
        fn level_block(&self, level: LevelRef, pos: [i32; 3]) -> Option<String> { unimplemented!("level_block") }
        fn level_game_time(&self, level: LevelRef) -> Option<i64> {
            unimplemented!("level_game_time")
        }
        fn dimension(&self) -> &'static str {
            unimplemented!("dimension")
        }
        fn neighbor_signal(&mut self, dimension: &str, pos: [i32; 3]) -> i32 {
            unimplemented!("neighbor_signal")
        }
        fn light(&self, pos: [i32; 3]) -> (u8, u8) {
            unimplemented!("light")
        }
        fn weather(&self) -> (f32, f32) {
            unimplemented!("weather")
        }
        fn day_time(&self) -> i64 {
            unimplemented!("day_time")
        }
        fn insert_item(&mut self, pos: [i32; 3], face: Direction, stack: ItemStack) -> Option<ItemStack> {
            unimplemented!("insert_item")
        }
        fn move_items(&mut self, dimension: &str, from: [i32; 3], from_face: Direction,
            to: [i32; 3], to_face: Direction, item: Option<&str>, max: u32) -> u32 {
            unimplemented!("move_items")
        }
        fn change_mod_block_entity(
            &mut self,
            pos: [i32; 3],
            change: &mut dyn FnMut(&mut ModBlockEntity),
        ) -> bool {
            unimplemented!("change_mod_block_entity")
        }
        fn block_entity_sync(&mut self, pos: [i32; 3]) {
            unimplemented!("block_entity_sync")
        }
        fn entity(&self, id: u32) -> Option<EntityRef> {
            unimplemented!("entity")
        }
        fn entity_position(&self, entity: EntityRef) -> Option<[f64; 3]> {
            unimplemented!("entity_position")
        }
        fn kill_mob(&mut self, dimension: &str, id: u32) -> bool {
            unimplemented!("kill_mob")
        }
        fn set_mob_health(&mut self, dimension: &str, id: u32, health: f32) -> bool {
            unimplemented!("set_mob_health")
        }
        fn play_sound(&mut self, sound: &str, at: [f64; 3], source: usize, volume: f32, pitch: f32) -> bool {
            unimplemented!("play_sound")
        }
        fn send_particles(
            &mut self,
            options: &ParticleOptions,
            state: Option<BlockStateId>,
            at: [f64; 3],
            spread: [f64; 3],
            speed: f64,
            count: i32,
        ) -> bool {
            unimplemented!("send_particles")
        }
        fn fill(&self, batch: &mut HudBatch, rect: [f32; 4], argb: u32) {
            unimplemented!("fill")
        }
        fn text(&self, batch: &mut HudBatch, text: &str, at: [f32; 2], rgb: u32, shadow: bool) {
            unimplemented!("text")
        }
        fn text_width(&self, text: &str) -> f32 {
            unimplemented!("text_width")
        }
        fn draw_cube(
            &self,
            draw: &mut BlockEntityDraw,
            from: [f32; 3],
            to: [f32; 3],
            faces: [Option<CubeFace<'_>>; 6],
            tint: u32,
        ) -> bool {
            unimplemented!("draw_cube")
        }
        fn draw_item(
            &self,
            draw: &mut BlockEntityDraw,
            stack: &ItemStack,
            display: DisplayContext,
            owner_yaw: Option<f32>,
            seed: i32,
        ) -> bool {
            unimplemented!("draw_item")
        }
        fn block_state_property(&self, state: BlockStateId, key: &str) -> Option<&str> {
            unimplemented!("block_state_property")
        }
        fn draw_quad(
            &self,
            draw: &mut BlockEntityDraw,
            sprite: &str,
            corners: [[f32; 3]; 4],
            uv: [f32; 4],
            tint: u32,
        ) -> bool {
            unimplemented!("draw_quad")
        }
        fn summon(&mut self, dimension: &str, kind: &str, pos: [f64; 3], nbt: Compound) -> Option<u32> {
            unimplemented!("summon")
        }
        fn surface_heights(
            &self,
            dimension: &str,
            columns: &[(i32, i32)],
            heightmap: NativeHeightmap,
        ) -> Result<Vec<Option<i32>>, String> {
            unimplemented!("surface_heights")
        }
        fn server_players(&self) -> Vec<NativeServerPlayer> {
            unimplemented!("server_players")
        }
        fn action_bar(&mut self, uuid: u128, line: Text) -> bool {
            unimplemented!("action_bar")
        }
        fn tell(&mut self, uuid: u128, line: Text) -> bool {
            unimplemented!("tell")
        }
        fn take_items(&mut self, item: &str, count: u32) -> u32 {
            unimplemented!("take_items")
        }
        fn give_items(&mut self, item: &str, count: u32) -> u32 {
            unimplemented!("give_items")
        }
        fn sneaking(&self) -> bool {
            unimplemented!("sneaking")
        }
        fn edit_blocks(&mut self, dimension: &str, edit: &NativeBlockEdit) -> Result<u64, String> {
            unimplemented!("edit_blocks")
        }
        /// `native_world_columns::Game::native_read_columns`: `below` blocos abaixo e `above` a
        /// partir da altura de cada coluna pelo `heightmap`, de baixo para cima.
        fn read_columns(
            &self,
            dimension: &str,
            columns: &[(i32, i32)],
            heightmap: NativeHeightmap,
            range: (u32, u32),
        ) -> Result<NativeColumnBlocks, String> {
            unimplemented!("read_columns")
        }
        /// `native_server::Game::native_world_dir`: a pasta do mundo aberto (ao lado do
        /// `level.dat`); None sem mundo.
        fn world_dir(&self) -> Option<std::path::PathBuf> {
            unimplemented!("world_dir")
        }
        fn entity_nbt(&mut self, entity: EntityRef) -> Option<Compound> {
            unimplemented!("entity_nbt")
        }
        fn entity_merge_nbt(&mut self, entity: EntityRef, nbt: Compound) -> bool {
            unimplemented!("entity_merge_nbt")
        }
        fn item_entities(&self, dimension: &str, min: [f64; 3], max: [f64; 3]) -> Vec<NativeItemEntity> {
            unimplemented!("item_entities")
        }
        fn set_item_entity(&mut self, dimension: &str, id: u32, stack: ItemStack) -> bool {
            unimplemented!("set_item_entity")
        }
        /// `native_server::Game::native_run_command`: roda como o console do servidor (nível 4,
        /// sem saída). `false` sem servidor (cliente de outro servidor) ou se o comando falhar.
        fn run_command(&mut self, line: &str) -> bool {
            unimplemented!("run_command")
        }
        /// `native_hooks::Game::native_window`: este `Game` é o da janela, não um servidor.
        fn window(&self) -> bool {
            false
        }
        /// `native_server::Game::native_acting_player`: o jogador cuja ação o gancho ouve. API 21.
        fn player(&self) -> Option<NativePlayer> {
            unimplemented!("player")
        }
        /// `native_biome_colors::Game::native_refresh_biome_colors` (API 107, cliente).
        fn refresh_biome_colors(&mut self) {
            unimplemented!("refresh_biome_colors")
        }
        fn acting_player(&self) -> Option<NativeServerPlayer> {
            unimplemented!("acting_player")
        }
        /// `native_inventories::Game::native_send_map`: 128 x 128 cores de mapa para um jogador.
        /// API 132.
        fn send_map(&mut self, uuid: u128, map_id: i32, colors: &[u8]) -> bool {
            unimplemented!("send_map")
        }
        /// `native_living_refs::Game::native_living`: a entidade como ser vivo.
        fn living(&self, entity: EntityRef) -> Option<LivingRef> {
            unimplemented!("living")
        }
        /// `native_living_refs::Game::native_living_set_equipment`: põe `stack` (None esvazia) no
        /// slot de equipamento, como `/item replace entity`.
        fn living_set_equipment(&mut self, living: LivingRef, slot: EquipmentSlot, stack: Option<ItemStack>) -> bool {
            unimplemented!("living_set_equipment")
        }
    }

    /// `horizon_client::first_person::Hand`, reexportado em `mod_api`.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Hand {
        MainHand,
        OffHand,
    }

    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct NativeBlockRun {
        pub x: i32,
        pub z: i32,
        pub y: i32,
        pub height: u32,
        pub surface: Option<NativeHeightmap>,
        pub block: u32,
        pub only: Option<u32>,
    }
    #[derive(Clone, Debug, PartialEq, Default)]
    pub struct NativeBlockEdit {
        pub palette: Vec<String>,
        pub filters: Vec<String>,
        pub runs: Vec<NativeBlockRun>,
    }
    /// `native_world_columns::NativeColumnBlocks`.
    #[derive(Clone, Debug, PartialEq, Default)]
    pub struct NativeColumnBlocks {
        pub palette: Vec<String>,
        pub bases: Vec<Option<i32>>,
        pub blocks: Vec<u16>,
        pub tints: Vec<u32>,
    }
    #[derive(Clone, Debug)]
    pub struct NativeItemEntity {
        pub id: u32,
        pub position: [f64; 3],
        pub stack: ItemStack,
        pub pickup_delay: u32,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum NativeHeightmap {
        WorldSurface,
        MotionBlocking,
        MotionBlockingNoLeaves,
        OceanFloor,
    }
    /// The window's player (`native_player`), on the client.
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct NativePlayer {
        pub x: f64,
        pub y: f64,
        pub z: f64,
        pub yaw: f32,
        pub pitch: f32,
        pub dimension: &'static str,
    }

    /// A block outline drawn over the world (`native_visuals`).
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct NativeBlockRim {
        pub pos: [i32; 3],
        pub argb: u32,
        pub width: f32,
    }

    /// A camera-local visual light, with no gameplay light or shadow writes.
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct NativeColoredLight {
        pub at: [f64; 3],
        pub rgb: u32,
        pub radius: f32,
        pub intensity: f32,
    }

    /// `native_visuals(rims, lights)`: a world frame's rims and coloured lights (API 46).
    pub struct VisualsEvent<'a> {
        pub rims: &'a mut Vec<NativeBlockRim>,
        pub lights: &'a mut Vec<NativeColoredLight>,
    }

    /// `horizon_client_render::biome::BiomeColors` (API 107).
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
    pub struct BiomeColors {
        pub grass: u32,
        pub foliage: u32,
        pub dry_foliage: u32,
        pub water: u32,
    }

    /// `native_biome_colors(biome, colors)` (API 107).
    pub struct BiomeColorsEvent<'a> {
        pub biome: &'a str,
        pub colors: &'a mut BiomeColors,
    }

    /// `horizon_client_render::fog::FogType`.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum FogType {
        Atmospheric,
        Water,
        Lava,
        PowderSnow,
    }

    /// `horizon_client_render::fog::FogData`.
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct FogData {
        pub color: [f32; 4],
        pub environmental_start: f32,
        pub environmental_end: f32,
        pub render_distance_start: f32,
        pub render_distance_end: f32,
        pub sky_end: f32,
        pub cloud_end: f32,
    }

    /// `native_render_fog(partial, camera, fog_type, fog)`.
    pub struct RenderFogEvent<'a> {
        pub partial: f32,
        pub camera: [f64; 3],
        pub fog_type: FogType,
        pub fog: &'a mut FogData,
    }

    /// `native_shader_params(partial, params)`: `frame.mods` of the mod's shader pack (API 84).
    pub struct ShaderParamsEvent<'a> {
        pub partial: f32,
        pub params: &'a mut [[f32; 4]; 8],
    }

    #[derive(Clone, Debug, Default)]
    pub struct NativeServerPlayer {
        pub uuid: u128,
        pub name: String,
        pub permission: u8,
        pub dimension: String,
        pub position: [f64; 3],
        pub game_mode: i32,
        pub health: f32,
    }

    // ---------- Eventos dos ganchos usados ----------
    #[non_exhaustive]
    pub struct BlockEntityEvent<'a> {
        pub block: &'a str,
    }
    #[non_exhaustive]
    pub struct BlockEntityTicksEvent<'a> {
        pub block: &'a str,
        pub ticks: bool,
    }
    #[non_exhaustive]
    pub struct BlockEntityContainerEvent<'a> {
        pub block: &'a str,
    }
    #[non_exhaustive]
    pub struct ContainerAcceptsEvent<'a> {
        pub pos: [i32; 3],
        pub entity: &'a ModBlockEntity,
        pub slot: usize,
        pub stack: &'a ItemStack,
        pub face: Option<Direction>,
    }
    #[non_exhaustive]
    pub struct RightClickBlockEvent {
        pub pos: [i32; 3],
        pub face: [i32; 3],
    }
    #[non_exhaustive]
    pub struct BlockEntityEnergyEvent<'a> {
        pub block: &'a str,
    }
    #[non_exhaustive]
    pub struct BlockEntityTickEvent<'a> {
        pub pos: [i32; 3],
        pub entity: &'a mut ModBlockEntity,
    }
    #[non_exhaustive]
    pub struct ScreenEvent<'a> {
        pub block: &'a str,
    }
    #[non_exhaustive]
    pub struct ScreenDrawEvent<'a> {
        pub block: &'a str,
        pub data: &'a [i32],
        pub batch: &'a mut HudBatch,
        pub origin: [f32; 2],
    }
    impl<'a> ScreenDrawEvent<'a> {
        /// Só no substituto: monta o evento para gerar prévias do painel.
        pub fn preview(block: &'a str, data: &'a [i32], batch: &'a mut HudBatch) -> Self {
            Self { block, data, batch, origin: [0.0, 0.0] }
        }
    }
    /// `screen_click` (API 20): `replace`, o retorno é o botão.
    #[non_exhaustive]
    pub struct ScreenClickEvent<'a> {
        pub block: &'a str,
        pub data: &'a [i32],
        pub at: [f32; 2],
        pub right: bool,
    }
    /// `menu_button` (API 20), no servidor.
    #[non_exhaustive]
    pub struct MenuButtonEvent<'a> {
        pub pos: [i32; 3],
        pub entity: &'a mut ModBlockEntity,
        pub id: i32,
    }
    #[non_exhaustive]
    pub struct ScreenTooltipEvent<'a> {
        pub block: &'a str,
        pub data: &'a [i32],
        pub at: [f32; 2],
        pub lines: &'a mut Vec<String>,
    }
    #[non_exhaustive]
    pub struct TooltipEvent<'a> {
        pub stack: &'a ItemStack,
        pub lines: &'a mut Vec<(Text, u32)>,
    }
    #[non_exhaustive]
    pub struct MenuSlotRulesEvent<'a> {
        pub menu: &'a str,
        pub slot: usize,
        pub stack: &'a ItemStack,
        pub may_place: bool,
        pub max_stack: u32,
    }
    #[non_exhaustive]
    pub struct BlockEntityUpdateTagEvent<'a> {
        pub pos: [i32; 3],
        pub entity: &'a ModBlockEntity,
    }
    #[non_exhaustive]
    pub struct BlockEntitySubmitEvent<'a> {
        pub pos: [i32; 3],
        pub block: &'a str,
        pub state: BlockStateId,
        pub entity: &'a ModBlockEntity,
        pub draw: &'a mut BlockEntityDraw,
    }
    impl<'a> BlockEntitySubmitEvent<'a> {
        pub fn new(pos: [i32;3], block: &'a str, entity: &'a ModBlockEntity, draw: &'a mut BlockEntityDraw) -> Self {
            Self {pos,block,state:BlockStateId(0),entity,draw}
        }
        /// Só no substituto: monta o evento para medições do quadro.
        pub fn preview(pos: [i32; 3], block: &'a str, state: BlockStateId, entity: &'a ModBlockEntity,
            draw: &'a mut BlockEntityDraw) -> Self {
            Self { pos, block, state, entity, draw }
        }
    }
    #[non_exhaustive]
    pub struct EntityTickPreEvent<'a> {
        pub dimension: &'a str,
        pub entity: u32,
        pub kind: &'a str,
    }
    impl<'a> EntityTickPreEvent<'a> {
        /// Só no substituto: monta o evento para medições.
        pub fn preview(dimension: &'a str, entity: u32, kind: &'a str) -> Self {
            Self { dimension, entity, kind }
        }
    }
    #[non_exhaustive]
    pub struct TickEvent {}
    impl TickEvent {
        /// Só no substituto: monta o evento para medições.
        pub fn preview() -> Self {
            Self {}
        }
    }
    #[non_exhaustive]
    pub struct ServerAboutToStartEvent { pub dedicated: bool }
    #[non_exhaustive]
    pub struct PlayerLoggedInEvent<'a> {
        pub player: u128,
        pub name: &'a str,
    }
    /// `native_hooks::Game::native_chunk_load` (API 57).
    #[non_exhaustive]
    pub struct ChunkLoadEvent<'a> {
        pub dimension: &'a str,
        pub chunk: [i32; 2],
        pub new: bool,
    }
    /// `merchant::ItemCost` (Horizon 66f3981): o Veio só lê `stack`.
    #[derive(Clone, Debug)]
    pub struct ItemCost {
        pub stack: ItemStack,
    }
    /// `merchant::MerchantOffer` (Horizon 66f3981), reexportado em `mod_api`.
    #[derive(Clone, Debug)]
    pub struct MerchantOffer {
        pub base_cost_a: ItemCost,
        pub cost_b: Option<ItemCost>,
        pub result: ItemStack,
        pub uses: i32,
        pub max_uses: i32,
        pub reward_exp: bool,
        pub special_price_diff: i32,
        pub demand: i32,
        pub price_multiplier: f32,
        pub xp: i32,
    }
    /// `trade_with_villager` (API 310): só escuta; a troca já foi feita.
    #[non_exhaustive]
    pub struct TradeWithVillagerEvent<'a> {
        pub merchant: u32,
        pub kind: &'a str,
        pub offer: &'a MerchantOffer,
    }
    #[non_exhaustive]
    pub struct PlayerRespawnedEvent<'a> {
        pub player: u128,
        pub name: &'a str,
    }
    #[non_exhaustive]
    pub struct PlayerSpawnPointSetEvent<'a> {
        pub player: u128,
        pub name: &'a str,
        pub dimension: &'a str,
        pub pos: [i32; 3],
        pub cause: &'a str,
    }
    #[non_exhaustive]
    pub struct PlayerBedEnterEvent<'a> {
        pub player: u128,
        pub name: &'a str,
        pub pos: [i32; 3],
    }
    #[non_exhaustive]
    pub struct LivingEntityUseItemFinishEvent<'a> {
        pub stack: &'a ItemStack,
        pub entity: Option<i32>,
        pub hand: Hand,
        pub duration: i32,
        pub result: &'a mut Option<ItemStack>,
    }
    #[non_exhaustive]
    pub struct RegisterGoalsEvent<'a> {
        pub mob: &'a mut crate::mobs::Mob,
    }
    #[non_exhaustive]
    pub struct BlockEntityRenderBoundingBoxEvent<'a> {
        pub pos: [i32; 3],
        pub block: &'a str,
        pub state: BlockStateId,
        pub entity: &'a ModBlockEntity,
        pub bounds: &'a mut [f32; 6],
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct NativeCommand { pub usage: String, pub level: u8 }
    impl NativeCommand {
        pub fn new(usage: &str) -> Self { Self { usage: usage.to_owned(), level: 0 } }
    }
    #[non_exhaustive]
    pub struct PlaceBlockEvent<'a> { pub stack: &'a ItemStack, pub pos: [i32; 3], pub state: BlockStateId }
    #[non_exhaustive]
    pub struct PlayerLoggedOutEvent<'a> { pub player: u128, pub name: &'a str }
    #[non_exhaustive]
    pub struct RegisterCommandsEvent<'a> { pub commands: &'a mut Vec<NativeCommand> }
    #[non_exhaustive]
    pub struct PerformCommandEvent<'a> { pub player: &'a NativeServerPlayer, pub command: &'a str }

    pub struct InventoryMoveItemEvent<'a> { pub source: [i32; 3], pub destination: [i32; 3], pub item: &'a str }

    // ---------- Ganchos ----------
    pub trait Hooks: Send + Sync {
        fn server_about_to_start(&self, host: &mut dyn Host, event: &mut ServerAboutToStartEvent) {}
        fn inventory_move_item(&self, host: &dyn Host, event: &mut InventoryMoveItemEvent<'_>) -> Flow { Flow::Continue }
        fn place_block(&self, host: &mut dyn Host, event: &mut PlaceBlockEvent<'_>) {}
        fn player_logged_out(&self, host: &mut dyn Host, event: &mut PlayerLoggedOutEvent<'_>) {}
        fn register_commands(&self, host: &mut dyn Host, event: &mut RegisterCommandsEvent<'_>) {}
        fn perform_command(&self, host: &mut dyn Host, event: &mut PerformCommandEvent<'_>) -> Flow { Flow::Continue }
        fn block_entity(&self, host: &mut dyn Host, event: &mut BlockEntityEvent<'_>) -> Option<(String, Compound)> {
            None
        }
        fn block_entity_ticks(&self, host: &dyn Host, event: &mut BlockEntityTicksEvent<'_>) {}
        fn block_entity_container(
            &self,
            host: &dyn Host,
            event: &mut BlockEntityContainerEvent<'_>,
        ) -> Option<ModContainer> {
            None
        }
        fn container_accepts(&self, host: &dyn Host, event: &mut ContainerAcceptsEvent<'_>) -> Flow {
            Flow::Continue
        }
        fn right_click_block(&self, host: &mut dyn Host, event: &mut RightClickBlockEvent) -> Flow {
            Flow::Continue
        }
        fn block_entity_energy(
            &self,
            host: &dyn Host,
            event: &mut BlockEntityEnergyEvent<'_>,
        ) -> Option<ModEnergyBuffer> {
            None
        }
        fn block_entity_tick(&self, host: &mut dyn Host, event: &mut BlockEntityTickEvent<'_>) {}
        fn screen(&self, host: &dyn Host, event: &mut ScreenEvent<'_>) -> Option<NativeScreen> {
            None
        }
        fn screen_draw(&self, host: &dyn Host, event: &mut ScreenDrawEvent<'_>) {}
        fn screen_tooltip(&self, host: &dyn Host, event: &mut ScreenTooltipEvent<'_>) {}
        fn screen_click(&self, host: &dyn Host, event: &mut ScreenClickEvent<'_>) -> Option<i32> {
            None
        }
        fn menu_button(&self, host: &mut dyn Host, event: &mut MenuButtonEvent<'_>) {}
        fn tooltip(&self, host: &dyn Host, event: &mut TooltipEvent<'_>) {}
        fn menu_slot_rules(&self, event: &mut MenuSlotRulesEvent<'_>) {}
        fn block_entity_update_tag(&self, event: &mut BlockEntityUpdateTagEvent<'_>) -> Option<Compound> {
            None
        }
        fn block_entity_submit(&self, host: &dyn Host, event: &mut BlockEntitySubmitEvent<'_>) {}
        fn visuals(&self, host: &dyn Host, event: &mut VisualsEvent<'_>) {}
        fn shader_params(&self, host: &dyn Host, event: &mut ShaderParamsEvent<'_>) {}
        fn biome_colors(&self, host: &dyn Host, event: &mut BiomeColorsEvent<'_>) {}
        fn render_fog(&self, host: &dyn Host, event: &mut RenderFogEvent<'_>) {}
        fn entity_tick_pre(&self, host: &mut dyn Host, event: &mut EntityTickPreEvent<'_>) -> Flow {
            Flow::Continue
        }
        fn tick(&self, host: &mut dyn Host, event: &mut TickEvent) {}
        fn chunk_load(&self, host: &mut dyn Host, event: &mut ChunkLoadEvent<'_>) {}
        fn player_logged_in(&self, host: &mut dyn Host, event: &mut PlayerLoggedInEvent<'_>) {}
        fn player_respawned(&self, host: &mut dyn Host, event: &mut PlayerRespawnedEvent<'_>) {}
        fn trade_with_villager(&self, host: &mut dyn Host, event: &mut TradeWithVillagerEvent<'_>) {}
        fn player_spawn_point_set(&self, host: &mut dyn Host, event: &mut PlayerSpawnPointSetEvent<'_>) -> Flow {
            Flow::Continue
        }
        fn player_bed_enter(&self, host: &mut dyn Host, event: &mut PlayerBedEnterEvent<'_>) -> Flow {
            Flow::Continue
        }
        fn living_entity_use_item_finish(&self, host: &mut dyn Host, event: &mut LivingEntityUseItemFinishEvent<'_>) {}
        fn register_goals(&self, event: &mut RegisterGoalsEvent<'_>) {}
        fn block_entity_render_bounding_box(
            &self,
            host: &dyn Host,
            event: &mut BlockEntityRenderBoundingBoxEvent<'_>,
        ) {
        }
    }
}

/// Mobs e objetivos (`horizon_client::mobs`), só os campos que o Veio lê.
pub mod mobs {
    use std::sync::Arc;

    #[derive(Clone, Copy, Debug, Default, PartialEq)]
    pub struct Vec3 {
        pub x: f64,
        pub y: f64,
        pub z: f64,
    }
    #[derive(Clone, Debug, Default)]
    pub struct MobType {
        pub id: String,
    }
    pub struct MobWorld<'w> {
        _world: std::marker::PhantomData<&'w ()>,
    }
    impl<'w> MobWorld<'w> {
        /// Offline fixture, not a Horizon API method.
        pub fn preview() -> Self { Self { _world: std::marker::PhantomData } }
        pub fn fixture() -> Self { Self { _world: std::marker::PhantomData } }
    }
    #[derive(Default)]
    pub struct Navigation {
        pub done: bool,
    }
    impl Navigation {
        pub fn stop(&mut self) { self.done = true; }
        pub fn is_done(&self) -> bool {
            self.done
        }
    }
    #[derive(Default)]
    pub struct Ai {
        pub goals: goals::GoalSelector,
        pub targets: goals::GoalSelector,
        pub nav: Navigation,
    }
    /// Nome antigo usado nos testes de caravana.
    pub type Extra = MobExtra;
    pub struct Mob {
        pub rider: Option<()>,
        pub id: u32,
        pub kind: Arc<MobType>,
        pub position: Vec3,
        /// `Mob.velocity` e `Mob.on_climbable` do Horizon: a subida por escada de mão (`workforce`).
        pub velocity: Vec3,
        pub on_climbable: bool,
        pub ai: Ai,
        /// No jogo, `MobExtra` tem muito mais; o Veio só chega ao aldeão por `villager::ext`.
        pub extra: MobExtra,
    }
    #[derive(Default)]
    pub struct MobExtra {
        pub villager: Option<Box<villager::Ext>>,
        pub rider: Option<u32>,
        pub leash: Option<()>,
        pub leash_pending: Option<()>,
    }

    /// `mobs::villager` (Horizon 66f3981): só o acesso ao cérebro que o Veio usa.
    pub mod villager {
        use super::Mob;
        #[derive(Default)]
        pub struct Ext {
            /// O cérebro do aldeão, criado no primeiro tick.
            pub brain: Option<Box<super::villager_brain::Brain>>,
        }
        pub fn ext(m: &Mob) -> Option<&Ext> {
            m.extra.villager.as_deref()
        }
        pub fn ext_mut(m: &mut Mob) -> &mut Ext {
            m.extra.villager.get_or_insert_with(Box::default)
        }
    }

    /// `mobs::villager_brain` (Horizon 66f3981): memórias e atividade que o Veio lê e escreve.
    pub mod villager_brain {
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub enum Act { Core, Idle, Work, Play, Rest, Meet, Panic, Raid, PreRaid, Hide }
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum Tracker {
            Block([i32; 3]),
            Point([f64; 3]),
        }
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub struct WalkTarget {
            pub target: Tracker,
            pub speed: f32,
            pub close: i32,
        }
        #[derive(Debug)]
        pub struct Slot<T> {
            v: Option<T>,
        }
        impl<T> Default for Slot<T> {
            fn default() -> Self {
                Self { v: None }
            }
        }
        impl<T> Slot<T> {
            pub fn set(&mut self, v: T) {
                self.v = Some(v);
            }
            pub fn clear(&mut self) {
                self.v = None;
            }
            pub fn get(&self) -> Option<&T> {
                self.v.as_ref()
            }
        }
        #[derive(Debug, Default)]
        pub struct Memories {
            pub walk_target: Slot<WalkTarget>,
            pub hurt_by: Slot<i64>,
            /// No Horizon é `Slot<Who>`; o Veio só chama `clear`.
            pub hurt_by_entity: Slot<u32>,
            pub nearest_hostile: Slot<u32>,
        }
        #[derive(Debug)]
        pub struct Brain {
            pub mem: Memories,
            pub activity: Act,
        }
        impl Brain {
            /// Só no substituto.
            pub fn preview(activity: Act) -> Self {
                Self { mem: Memories::default(), activity }
            }
        }
    }
    impl Mob {
        pub fn nav_move_to(&mut self, w: &MobWorld, at: [f64; 3], speed: f64) -> bool {
            let _ = (w, at, speed);
            true
        }
    }

    /// `mobs::Species` (Horizon 66f3981): só as espécies que o Veio cita.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Species {
        Villager,
        IronGolem,
    }

    pub mod goals {
        use super::{Mob, MobWorld, Species};
        /// `goals::Prey` (Horizon 66f3981): o que um objetivo de alvo caça.
        #[derive(Clone, Debug)]
        pub enum Prey {
            Player,
            Species(Vec<Species>),
        }
        /// `goals::NearestTarget` (`NearestAttackableTargetGoal`, Horizon 66f3981).
        #[derive(Clone, Debug)]
        pub struct NearestTarget {
            pub prey: Prey,
            pub must_see: bool,
        }
        impl NearestTarget {
            pub fn new(prey: Prey, must_see: bool) -> Self {
                Self { prey, must_see }
            }
        }
        impl Goal for NearestTarget {
            fn flags(&self) -> u8 {
                TARGET
            }
            fn can_use(&mut self, _m: &mut Mob, _cx: &mut Cx) -> bool {
                false
            }
            fn box_clone(&self) -> Box<dyn Goal> {
                Box::new(self.clone())
            }
        }
        pub const MOVE: u8 = 1;
        pub const LOOK: u8 = 2;
        pub const JUMP: u8 = 4;
        pub const TARGET: u8 = 8;
        pub struct Cx<'a, 'w> {
            pub w: &'a MobWorld<'w>,
        }
        pub trait Goal: std::fmt::Debug {
            fn flags(&self) -> u8;
            fn set_start_delay(&mut self, _ticks: i32) {}
            fn can_use(&mut self, m: &mut Mob, cx: &mut Cx) -> bool;
            fn can_continue(&mut self, m: &mut Mob, cx: &mut Cx) -> bool {
                self.can_use(m, cx)
            }
            fn interruptable(&self) -> bool {
                true
            }
            fn start(&mut self, _m: &mut Mob, _cx: &mut Cx) {}
            fn stop(&mut self, _m: &mut Mob, _cx: &mut Cx) {}
            fn every_tick(&self) -> bool {
                false
            }
            fn tick(&mut self, _m: &mut Mob, _cx: &mut Cx) {}
            fn box_clone(&self) -> Box<dyn Goal>;
        }
        #[derive(Default)]
        pub struct GoalSelector {
            pub added: Vec<(i32, Box<dyn Goal>)>,
        }
        impl GoalSelector {
            pub fn add(&mut self, priority: i32, goal: impl Goal + 'static) {
                self.added.push((priority, Box::new(goal)));
            }
        }
    }
}

/// `horizon_client::merchant` (Horizon 66f3981): o tipo do custo de uma oferta.
pub mod merchant {
    pub use crate::mod_api::{ItemCost, MerchantOffer};
}
