use super::*;

#[test]
fn fast_material_rejection_keeps_the_original_layout_and_contract_fences() {
    let blocks = [DRILL, DRILL_MK2, PRESS, ASSEMBLER, CUTTER, INTEGRATOR, CRUSHER, ALLOY,
        ELECTRONICS, GENERATOR, GEOTHERMAL, HARVESTER, HUB, BELT, FAST_BELT, SPLITTER, MERGER,
        CABLE, SOLAR, BATTERY, TESLA, inserters::ID, inserters::FAST, inserters::LONG,
        production::PLANTER, production::TREE_FARM, production::RANCH, production::PANTRY,
        production::MILL, production::KITCHEN, production::ELECTRIC_FURNACE, production::STEAM_ENGINE,
        production::BIOMASS_GENERATOR, "veio:farm_post", "veio:hauler_post", "veio:shepherd_post",
        "veio:foreman_desk", "veio:barn", "veio:silo", "veio:control_center"];
    let materials = [PLATE, GEAR, FRAME, STEEL, CIRCUIT, BONE_MEAL, "minecraft:raw_copper",
        "minecraft:coal", "minecraft:lava_bucket", "minecraft:bucket", "minecraft:wheat",
        "minecraft:carrot", "minecraft:wheat_seeds", "minecraft:oak_sapling", "minecraft:emerald",
        "minecraft:stone", "minecraft:shears"];
    for block in blocks {
        let mut entity = ModBlockEntity::default();
        entity.id = block.into();
        for stage in [0, 1, 4] {
            set_number(&mut entity, "ContractStage", stage);
            let size = layout(block).map_or(0, |(n, _, _)| n);
            for slot in (0..size + 2).chain(std::iter::once(usize::MAX)) {
                for item in materials {
                    let stack = ItemStack::new(item, 1);
                    let expected = layout(block).is_some_and(|(_, inputs, _)| inputs.contains(&slot))
                        && if block == HUB { contracts::accepts(&entity, slot, &stack) } else { accepts(block, slot, &stack) };
                    assert_eq!(accepts_material(&entity, slot, &stack), expected, "{block} / {slot} / {item}");
                }
            }
        }
    }
}

fn station(block: &str) -> ModBlockEntity {
    let mut entity = ModBlockEntity::default();
    entity.id = block.into();
    entity.items = vec![None; layout(block).map_or(0, |(size, _, _)| size)];
    entity
}

#[test]
fn transport_buffers_are_not_worker_haul_jobs() {
    let mut entity = station(MERGER);
    entity.items[0] = Some(ItemStack::new(PLATE, 8));
    set_number(&mut entity, "Status", 3);
    let view = inspect(&entity).unwrap();
    assert_eq!(view.profile.kind, StationKind::Transport);
    assert_eq!(view.blockage, Blockage::Output);
    assert!(view.services.is_empty());
    assert!(view.energy.is_none());
    assert_eq!(view.inputs.len(), 2);
}

#[test]
fn current_contract_filters_materials_without_leaking_output_slots() {
    let mut entity = station(HUB);
    assert!(accepts_material(&entity, 0, &ItemStack::new(PLATE, 24)));
    assert!(!accepts_material(&entity, 0, &ItemStack::new(STEEL, 16)));
    set_number(&mut entity, "ContractStage", 1);
    assert!(!accepts_material(&entity, 0, &ItemStack::new(PLATE, 24)));
    assert!(accepts_material(&entity, 0, &ItemStack::new(STEEL, 16)));
    assert!(!accepts_material(&entity, 2, &ItemStack::new(BELT, 64)));
}

#[test]
fn station_snapshot_preserves_inputs_and_distinguishes_power_from_supply() {
    let mut entity = station(PRESS);
    entity.items[0] = Some(ItemStack::new("minecraft:raw_copper", 3));
    entity.items[1] = Some(ItemStack::new(PLATE, 2));
    set_number(&mut entity, "Status", 5);
    let before = entity.items.clone();
    let view = inspect(&entity).unwrap();
    assert_eq!(view.inputs[0].index, 0);
    assert_eq!(view.inputs[0].stack.unwrap().count, 3);
    assert_eq!(view.outputs[0].index, 1);
    assert!(view.services.contains(&Service::InspectPower));
    assert!(view.services.contains(&Service::CollectOutputs));
    assert!(!view.services.contains(&Service::CheckInputs));
    assert_eq!(entity.items, before);
}

#[test]
fn idle_empty_energy_does_not_invent_a_power_failure_or_a_worker_assignment() {
    let entity = station(production::PLANTER);
    let view = inspect(&entity).unwrap();
    assert_eq!(view.profile.sector, Sector::Farming);
    assert_eq!(view.blockage, Blockage::None);
    assert!(view.services.is_empty());
    assert_eq!(view.energy, Some((0, capacity(production::PLANTER))));
}

#[test]
fn redstone_is_a_control_to_inspect_and_unknown_states_stay_unknown() {
    let mut entity = station(INTEGRATOR);
    set_number(&mut entity, "Status", 9);
    assert_eq!(inspect(&entity).unwrap().services, [Service::InspectControl]);
    set_number(&mut entity, "Status", 200);
    assert_eq!(inspect(&entity).unwrap().blockage, Blockage::Unknown);
    assert_eq!(advice_key(-1), "veio.operation.advice.unknown");
    assert_eq!(advice_key(200), "veio.operation.advice.unknown");
    assert!(profile("minecraft:villager").is_none());
}

#[test]
fn extractor_and_infrastructure_do_not_accept_materials_in_output_or_dummy_slots() {
    assert!(!accepts_material(&station(DRILL), 0, &ItemStack::new(PLATE, 1)));
    assert!(!accepts_material(&station(SOLAR), 0, &ItemStack::new(PLATE, 1)));
    assert_eq!(inspect(&station(SOLAR)).unwrap().profile.kind, StationKind::Infrastructure);
}

#[test]
fn farmhand_post_is_a_farming_station_and_worker_states_suggest_no_hiring() {
    let mut entity = station(workforce::FARM_POST);
    assert_eq!(profile(workforce::FARM_POST), Some(Profile { sector: Sector::Farming, kind: StationKind::Workstation }));
    assert!(accepts_material(&entity, 0, &ItemStack::new("minecraft:bread", 1)));
    assert!(!accepts_material(&entity, 1, &ItemStack::new("minecraft:bread", 1)));
    set_number(&mut entity, "Status", workforce::NO_WORKER);
    let view = inspect(&entity).unwrap();
    assert_eq!(view.blockage, Blockage::Worker);
    assert!(view.services.is_empty());
    assert!(view.energy.is_none(), "o posto não usa FE");
    set_number(&mut entity, "Status", workforce::NO_FOOD);
    assert_eq!(inspect(&entity).unwrap().services, [Service::CheckInputs]);
    for status in 0..presentation::STATUS_KEYS.len() as i32 {
        assert_ne!(advice_key(status), "veio.operation.advice.unknown", "estado {status} sem dica");
    }
}
