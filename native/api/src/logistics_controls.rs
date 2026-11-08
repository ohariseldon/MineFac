//! Shared configuration for item routing. No items are consumed as filter samples.
use super::*;

pub(super) fn filter(entity: &ModBlockEntity) -> Option<String> {
    with_nbt_key("LogisticsFilter", |key| match entity.nbt.get(key) {
        Some(Tag::String(value)) => {
            let value = value.to_string_lossy();
            (!value.is_empty()).then_some(value)
        }
        _ => None,
    })
}

pub(super) fn has_filter(entity: &ModBlockEntity) -> bool {
    with_nbt_key("LogisticsFilter", |key| matches!(entity.nbt.get(key), Some(Tag::String(value)) if !value.0.is_empty()))
}

/// `true` quando não há filtro ou quando o item é o escolhido. Sem montar `String`.
pub(super) fn allows(entity: &ModBlockEntity, item: &str) -> bool {
    with_nbt_key("LogisticsFilter", |key| match entity.nbt.get(key) {
        Some(Tag::String(value)) if !value.0.is_empty() => java_eq(value, item),
        _ => true,
    })
}

pub(super) fn priority(entity: &ModBlockEntity) -> usize {
    match number(entity, "RoutingPriority") { 1 => 1, 2 => 2, _ => 0 }
}

pub(super) fn set_filter(entity: &mut ModBlockEntity, item: Option<&str>) {
    if let Some(item) = item.filter(|item| !item.is_empty()) {
        with_nbt_key("LogisticsFilter", |key| {
            entity.nbt.insert(key.clone(), Tag::String(item.into()));
        });
    } else {
        with_nbt_key("LogisticsFilter", |key| {
            entity.nbt.remove(key);
        });
    }
    publish(entity);
}

pub(super) fn cycle_priority(entity: &mut ModBlockEntity) {
    if entity.id == SPLITTER && has_filter(entity) {
        set_filter(entity, None);
        set_number(entity, "RoutingPriority", 0);
    } else {
        set_number(entity, "RoutingPriority", ((priority(entity) + 1) % 3) as i32);
    }
    publish(entity);
}

pub(super) fn publish(entity: &mut ModBlockEntity) {
    if configurable(&entity.id) {
        set_number(entity, "Info", priority(entity) as i32 + if has_filter(entity) { 4 } else { 0 });
    }
}

fn configurable(block: &str) -> bool {
    inserters::is_arm(block) || matches!(block, SPLITTER | MERGER)
}

/// Prancheta: copia filtro e prioridade de um braço, divisor ou unificador e cola em outros,
/// como o copiar e colar configurações do Factorio. Agachado copia; em pé cola.
pub(super) const CLIPBOARD: &str = "veio:clipboard";

/// Configuração copiada: (filtro, prioridade). Uma por mundo aberto, compartilhada.
static COPIED: std::sync::Mutex<Option<(Option<String>, i32)>> = std::sync::Mutex::new(None);

pub(super) fn copy(entity: &ModBlockEntity) -> (Option<String>, i32) {
    (filter(entity), number(entity, "RoutingPriority"))
}

/// Cola o que couber no bloco: o unificador não tem filtro, os braços não têm prioridade.
pub(super) fn paste(entity: &mut ModBlockEntity, (item, priority): &(Option<String>, i32)) {
    if entity.id != MERGER {
        set_filter(entity, item.as_deref());
    }
    if matches!(entity.id.as_str(), SPLITTER | MERGER) {
        set_number(entity, "RoutingPriority", *priority);
    }
    publish(entity);
}

fn clipboard(host: &mut dyn Host, pos: [i32; 3]) -> Flow {
    let mut copied = COPIED.lock().unwrap_or_else(|err| err.into_inner());
    if host.sneaking() {
        host.change_mod_block_entity(pos, &mut |entity| *copied = Some(copy(entity)));
        host.play_sound("minecraft:item.book.page_turn", center(pos), 4, 0.6, 1.0);
    } else if let Some(settings) = copied.as_ref() {
        if host.change_mod_block_entity(pos, &mut |entity| paste(entity, settings)) {
            host.block_entity_sync(pos);
            host.play_sound("minecraft:block.note_block.hat", center(pos), 4, 0.4, 1.5);
        }
    }
    Flow::Cancel
}

pub(super) fn right_click(host: &mut dyn Host, pos: [i32; 3], block: &str) -> Option<Flow> {
    if configurable(block) && host.held().is_some_and(|(item, _, _)| item == CLIPBOARD) {
        return Some(clipboard(host, pos));
    }
    if !configurable(block) || !host.sneaking() { return None; }
    let held = host.held().map(|(item, _, _)| item);
    let change = match held.as_deref() {
        Some(WRENCH) if matches!(block, SPLITTER | MERGER) => 0,
        Some(WRENCH) => 1,
        None => 1,
        Some(_) if block != MERGER => 2,
        _ => return None,
    };
    let changed = host.change_mod_block_entity(pos, &mut |entity| {
        match change {
            0 => cycle_priority(entity),
            1 => set_filter(entity, None),
            _ => set_filter(entity, held.as_deref()),
        }
    });
    if changed {
        host.block_entity_sync(pos);
        host.play_sound("minecraft:block.note_block.hat", center(pos), 4, 0.4, 1.2);
    }
    Some(Flow::Cancel)
}

pub(super) fn draw_filter(host: &dyn Host, event: &mut horizon_client::mod_api::BlockEntitySubmitEvent<'_>) {
    if !configurable(event.block) { return; }
    if matches!(event.block, SPLITTER | MERGER) {
        use horizon_client::mod_api::CubeFace;
        let facing = host.block_state_property(event.state, "facing").unwrap_or("south");
        let yaw = match facing { "west" => -90.0, "north" => -180.0, "east" => -270.0, _ => 0.0 };
        let mode = priority(event.entity);
        let sorted = has_filter(event.entity);
        let faces = [Some(CubeFace { sprite: "veio:gui/material", uv: None }); 6];
        event.draw.push();
        event.draw.rotate_about([0.0, 1.0, 0.0], yaw, [0.5; 3]);
        for (side, x) in [(1, 1.0), (2, 13.0)] {
            let color = if sorted { if side == 1 { 0x59c65a } else { 0x49b5de } }
                else if mode == side { 0xffc857 } else if mode == 0 { 0x59c65a } else { 0x555d66 };
            host.draw_cube(event.draw, [x, 5.0, 6.0], [x + 2.0, 5.5, 10.0], faces, color);
        }
        event.draw.pop();
    }
    let item = with_nbt_key("LogisticsFilter", |key| match event.entity.nbt.get(key) {
        Some(Tag::String(value)) if !value.0.is_empty() => Some(value),
        _ => None,
    });
    let Some(item) = item else { return; };
    use horizon_client::mod_api::DisplayContext;
    event.draw.push();
    event.draw.translate([0.5, if inserters::is_arm(event.block) { 0.65 } else { 0.45 }, 0.5]);
    event.draw.rotate_about([1.0, 0.0, 0.0], 90.0, [0.0; 3]);
    event.draw.scale([0.25; 3]);
    presentation::with_drawn_item(item, |stack| { host.draw_item(event.draw, stack, DisplayContext::Fixed, None, 0); });
    event.draw.pop();
}

#[cfg(test)]
mod tests {
    use super::*;
    struct ConfigHost { entity: ModBlockEntity, held: Option<String>, syncs: usize }
    impl Host for ConfigHost {
        fn held(&self) -> Option<(String, u32, bool)> { self.held.as_ref().map(|s| (s.clone(), 7, false)) }
        fn sneaking(&self) -> bool { true }
        fn change_mod_block_entity(&mut self, _pos: [i32; 3], change: &mut dyn FnMut(&mut ModBlockEntity)) -> bool {
            change(&mut self.entity); true
        }
        fn block_entity_sync(&mut self, _pos: [i32; 3]) { self.syncs += 1; }
        fn play_sound(&mut self, _sound: &str, _at: [f64; 3], _source: usize, _volume: f32, _pitch: f32) -> bool { true }
    }
    #[test]
    fn clipboard_copies_filter_and_priority_to_compatible_blocks() {
        let mut splitter = ModBlockEntity::default();
        splitter.id = SPLITTER.to_owned();
        set_filter(&mut splitter, Some("minecraft:iron_ingot"));
        set_number(&mut splitter, "RoutingPriority", 2);
        let settings = copy(&splitter);
        let mut arm = ModBlockEntity::default();
        arm.id = inserters::ID.to_owned();
        paste(&mut arm, &settings);
        assert_eq!(filter(&arm).as_deref(), Some("minecraft:iron_ingot"));
        assert_eq!(number(&arm, "RoutingPriority"), 0, "braço não tem prioridade");
        let mut merger = ModBlockEntity::default();
        merger.id = MERGER.to_owned();
        paste(&mut merger, &settings);
        assert_eq!((filter(&merger), priority(&merger)), (None, 2));
    }

    #[test]
    fn copy_and_clear_filter_never_consume_the_sample_or_put_it_in_inventory() {
        let mut entity = ModBlockEntity::default();
        entity.id = inserters::ID.into();
        entity.items = vec![None];
        let mut host = ConfigHost { entity, held: Some(PLATE.into()), syncs: 0 };
        assert!(right_click(&mut host, [0; 3], inserters::ID).is_some());
        assert_eq!(host.held().unwrap().1, 7);
        assert_eq!(filter(&host.entity).as_deref(), Some(PLATE));
        assert!(host.entity.items[0].is_none());
        assert_eq!(number(&host.entity, "Info"), 4);
        host.held = None;
        right_click(&mut host, [0; 3], inserters::ID);
        assert!(filter(&host.entity).is_none());
        assert_eq!(host.syncs, 2);
    }
    #[test]
    fn priority_cycles_and_filter_removal_survive_saved_nbt() {
        let mut entity = ModBlockEntity::default();
        entity.id = SPLITTER.into();
        cycle_priority(&mut entity);
        assert_eq!(priority(&entity), 1);
        cycle_priority(&mut entity);
        assert_eq!(priority(&entity), 2);
        set_filter(&mut entity, Some(PLATE));
        let mut restored = ModBlockEntity::default();
        restored.id = SPLITTER.into(); restored.nbt = entity.nbt.clone();
        assert_eq!(filter(&restored).as_deref(), Some(PLATE));
        cycle_priority(&mut restored);
        assert!(filter(&restored).is_none());
        assert_eq!(priority(&restored), 0);
    }
}
