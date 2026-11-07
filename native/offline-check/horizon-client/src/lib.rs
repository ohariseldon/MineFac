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
    #[derive(Default)]
    pub struct BlockEntityDraw {
        pub partial: f32,
        pub game_time: i64,
        pub light: u32,
    }
    impl BlockEntityDraw {
        pub fn push(&mut self) {}
        pub fn pop(&mut self) {}
        pub fn translate(&mut self, by: [f32; 3]) {}
        pub fn rotate_about(&mut self, axis: [f32; 3], degrees: f32, pivot: [f32; 3]) {}
        pub fn scale(&mut self, by: [f32; 3]) {}
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
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct LevelRef {
        dimension: &'static str,
    }
    impl LevelRef {
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
    #[non_exhaustive]
    pub struct EntityTickPreEvent<'a> {
        pub dimension: &'a str,
        pub entity: u32,
        pub kind: &'a str,
    }
    #[non_exhaustive]
    pub struct TickEvent {}

    // ---------- Ganchos ----------
    pub trait Hooks: Send + Sync {
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
        fn tooltip(&self, host: &dyn Host, event: &mut TooltipEvent<'_>) {}
        fn menu_slot_rules(&self, event: &mut MenuSlotRulesEvent<'_>) {}
        fn block_entity_update_tag(&self, event: &mut BlockEntityUpdateTagEvent<'_>) -> Option<Compound> {
            None
        }
        fn block_entity_submit(&self, host: &dyn Host, event: &mut BlockEntitySubmitEvent<'_>) {}
        fn entity_tick_pre(&self, host: &mut dyn Host, event: &mut EntityTickPreEvent<'_>) -> Flow {
            Flow::Continue
        }
        fn tick(&self, host: &mut dyn Host, event: &mut TickEvent) {}
    }
}
