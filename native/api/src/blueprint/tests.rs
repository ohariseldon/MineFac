//! Copiadora e marco: copiar, girar, cobrar, recusar sem cobrar e guardar no NBT.
use super::*;
use horizon_client::mod_api::NativeServerPlayer;
use std::cell::RefCell;

#[derive(Default)]
struct Site {
    blocks: HashMap<[i32; 3], (String, Vec<(String, String)>)>,
    entities: HashMap<[i32; 3], ModBlockEntity>,
    inventory: HashMap<String, u32>,
    told: RefCell<Vec<String>>,
    edits: usize,
}

impl Site {
    fn put(&mut self, at: [i32; 3], block: &str, props: &[(&str, &str)]) {
        self.blocks.insert(at, (block.into(), props.iter().map(|(k, v)| ((*k).into(), (*v).into())).collect()));
        if block.starts_with("veio:") {
            let mut entity = ModBlockEntity::default();
            entity.id = block.into();
            self.entities.insert(at, entity);
        }
    }
    fn prop(&self, at: [i32; 3], key: &str) -> Option<String> {
        self.blocks.get(&at)?.1.iter().find(|(k, _)| k == key).map(|(_, v)| v.clone())
    }
}

impl Host for Site {
    fn dimension(&self) -> &'static str { "veio:blueprint-test" }
    fn block(&self, at: [i32; 3]) -> String { self.blocks.get(&at).map_or("minecraft:air".into(), |(id, _)| id.clone()) }
    fn block_property(&self, at: [i32; 3], key: &str) -> Option<String> { self.prop(at, key) }
    fn set_block_property(&mut self, at: [i32; 3], key: &str, value: &str) -> bool {
        let Some((_, props)) = self.blocks.get_mut(&at) else { return false };
        props.retain(|(k, _)| k != key);
        props.push((key.into(), value.into()));
        true
    }
    fn change_mod_block_entity(&mut self, at: [i32; 3], change: &mut dyn FnMut(&mut ModBlockEntity)) -> bool {
        self.entities.get_mut(&at).map(change).is_some()
    }
    fn take_items(&mut self, item: &str, count: u32) -> u32 {
        let have = self.inventory.entry(item.into()).or_default();
        let got = (*have).min(count);
        *have -= got;
        got
    }
    fn give_items(&mut self, item: &str, count: u32) -> u32 { *self.inventory.entry(item.into()).or_default() += count; count }
    fn edit_blocks(&mut self, _: &str, edit: &NativeBlockEdit) -> Result<u64, String> {
        self.edits += 1;
        for run in &edit.runs { self.put([run.x, run.y, run.z], &edit.palette[run.block as usize].clone(), &[]); }
        Ok(edit.runs.len() as u64)
    }
    fn server_players(&self) -> Vec<NativeServerPlayer> {
        vec![NativeServerPlayer { uuid: 1, dimension: "veio:blueprint-test".into(), ..Default::default() }]
    }
    fn tell(&mut self, _: u128, line: Text) -> bool { self.told.borrow_mut().push(line.plain()); true }
    fn play_sound(&mut self, _: &str, _: [f64; 3], _: usize, _: f32, _: f32) -> bool { true }
    fn sneaking(&self) -> bool { true }
}

/// Uma linha pequena acima da copiadora (em 0,64,0, virada ao norte): esteira, prensa,
/// escada de mão subindo ao segundo andar, um elevador e um minério que não entra na cópia.
fn workshop() -> Site {
    let mut site = Site::default();
    site.put([0, 64, 0], DESIGNER, &[("facing", "north")]);
    site.put([1, 65, 0], "veio:belt", &[("facing", "east")]);
    site.put([2, 65, 0], "veio:press", &[("facing", "east")]);
    for y in 65..68 { site.put([0, y, 2], "minecraft:ladder", &[("facing", "north")]); }
    site.put([-1, 67, 0], "minecraft:oak_planks", &[]);
    site.put([-2, 65, -1], "veio:cargo_lift", &[("facing", "south")]);
    site.entities.get_mut(&[-2, 65, -1]).map(|e| set_number(e, "Down", 1));
    site.put([0, 66, -2], "minecraft:diamond_ore", &[]);
    site
}

#[test]
fn copying_keeps_blocks_states_and_settings_but_skips_ores() {
    let mut site = workshop();
    let design = capture(&mut site, [0, 64, 0], 0);
    assert_eq!(design.cells.len(), 7);
    assert_eq!(design.skipped, 1, "o minério fica de fora");
    assert_eq!((design.footprint, design.height, design.facing), (5, 3, 0));
    assert!(design.config.contains(&([-2, 1, -1], "Down".into(), Setting::Int(1))));
    let cost = costs(&design);
    assert_eq!(cost.get("minecraft:ladder"), Some(&3));
    assert_eq!(cost.get("veio:press"), Some(&1));
}

#[test]
fn the_design_survives_a_save_and_load() {
    let mut site = workshop();
    let design = capture(&mut site, [0, 64, 0], 0);
    let mut entity = ModBlockEntity::default();
    entity.id = DESIGNER.into();
    save(&mut entity, &design);
    assert_eq!(load(&entity), Some(design));
    assert!(!client_tag(&entity).contains_key(&"DesignCells".into()), "o projeto não vai para os clientes");
}

#[test]
fn rotation_turns_positions_facings_and_axes_clockwise() {
    assert_eq!(rotate([1, 2, 0], 1), [0, 2, 1], "leste vira sul");
    assert_eq!(rotate([0, 0, -1], 1), [1, 0, 0], "norte vira leste");
    assert_eq!(rotate([3, 1, 2], 4), [3, 1, 2]);
    assert_eq!(rotate_prop("facing", "north", 1), "east");
    assert_eq!(rotate_prop("facing", "up", 1), "up");
    assert_eq!(rotate_prop("axis", "x", 3), "z");
    assert_eq!(rotate_prop("rotation", "14", 1), "2");
}

#[test]
fn pasting_charges_the_player_rotates_and_restores_settings() {
    let mut site = workshop();
    let design = capture(&mut site, [0, 64, 0], 0);
    site.put([40, 64, 0], MARKER, &[("facing", "east")]);
    for (item, count) in costs(&design) { site.inventory.insert(item, count + 1); }
    let turns = (facing_index(Some("east")) + 4 - design.facing) % 4;
    assert_eq!(build(&mut site, [40, 64, 0], &design, turns), Ok(7));
    assert!(site.inventory.values().all(|left| *left == 1), "cobra exatamente o orçamento");
    // A prensa, 2 blocos a leste da copiadora, nasce 2 blocos ao sul do marco, virada ao sul.
    assert_eq!(site.block([40, 65, 2]), "veio:press");
    assert_eq!(site.prop([40, 65, 2], "facing").as_deref(), Some("south"));
    assert_eq!(site.block([38, 67, 0]), "minecraft:ladder");
    assert_eq!(number(&site.entities[&[41, 65, -2]], "Down"), 1, "o elevador continua descendo");
}

#[test]
fn missing_material_or_space_refuses_without_charging() {
    let mut site = workshop();
    let design = capture(&mut site, [0, 64, 0], 0);
    site.inventory.insert("minecraft:ladder".into(), 3);
    let refusal = build(&mut site, [40, 64, 0], &design, 0);
    assert!(matches!(refusal, Err(Refusal::Missing(ref list)) if list.iter().any(|(item, _)| item == "veio:press")));
    assert_eq!(site.inventory["minecraft:ladder"], 3, "o que foi tirado volta");
    assert_eq!(site.edits, 0);
    for (item, count) in costs(&design) { site.inventory.insert(item, count); }
    site.put([41, 65, 0], "minecraft:stone", &[]);
    assert_eq!(build(&mut site, [40, 64, 0], &design, 0), Err(Refusal::Blocked([41, 65, 0])));
    assert_eq!(site.inventory["veio:press"], 1, "obstáculo: nada é cobrado");
}

#[test]
fn the_marker_finds_the_design_and_builds_on_sneak_wrench() {
    let mut site = workshop();
    assert_eq!(right_click(&mut site, [0, 64, 0], DESIGNER, Some(PAPER)), Some(Flow::Cancel));
    site.put([0, 64, 30], MARKER, &[("facing", "north")]);
    let mut marker = site.entities[&[0, 64, 30]].clone();
    tick(&mut site, [0, 64, 30], &mut marker);
    assert_eq!((number(&marker, "Linked"), number(&marker, "BoxSize"), number(&marker, "BoxHeight")), (1, 5, 3));
    for item in ["veio:belt", "veio:press", "minecraft:ladder", "minecraft:oak_planks", "veio:cargo_lift"] {
        site.inventory.insert(item.into(), 64);
    }
    assert_eq!(right_click(&mut site, [0, 64, 30], MARKER, Some(WRENCH)), Some(Flow::Cancel));
    assert_eq!(site.block([2, 65, 30]), "veio:press");
    assert_eq!(right_click(&mut site, [0, 64, 30], MARKER, Some("minecraft:stick")), None, "outros itens seguem o fluxo comum");
}

#[test]
fn ores_portals_and_fluids_are_not_part_of_a_build() {
    for block in ["minecraft:iron_ore", "minecraft:water", "minecraft:nether_portal", "minecraft:bedrock", DESIGNER, MARKER] {
        assert!(!copyable(block), "{block}");
    }
    for block in ["minecraft:ladder", "minecraft:oak_stairs", "veio:cargo_lift", "minecraft:glass"] {
        assert!(copyable(block), "{block}");
    }
    let door = State { block: "minecraft:oak_door".into(), props: vec![("half".into(), "upper".into())] };
    assert_eq!(cost_item(&door), None, "a porta é paga uma vez");
    assert_eq!(cost_item(&State { block: "minecraft:wall_torch".into(), props: vec![] }).as_deref(), Some("minecraft:torch"));
}
