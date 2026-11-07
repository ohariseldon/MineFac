use super::*;

pub(super) struct Contract {
    pub title: &'static str,
    pub inputs: [(&'static str, u32); 2],
    pub rewards: [(&'static str, u32); 2],
}

pub(super) const CONTRACTS: [Contract; 4] = [
    Contract {
        title: "veio.contract.workshop",
        inputs: [(PLATE, 24), (GEAR, 8)],
        rewards: [(BELT, 64), (CABLE, 32)],
    },
    Contract {
        title: "veio.contract.railway",
        inputs: [(STEEL, 16), (MODULE, 8)],
        rewards: [("minecraft:rail", 64), ("minecraft:powered_rail", 16)],
    },
    Contract {
        title: "veio.contract.village",
        inputs: [(CIRCUIT, 12), (FRAME, 24)],
        rewards: [("minecraft:hopper", 8), ("minecraft:emerald", 8)],
    },
    Contract {
        title: "veio.contract.beacon",
        inputs: [("minecraft:nether_star", 1), (CIRCUIT, 32)],
        rewards: [("minecraft:beacon", 1), (DRILL_MK2, 2)],
    },
];

pub(super) fn current(entity: &ModBlockEntity) -> Option<&'static Contract> {
    CONTRACTS.get(number(entity, "ContractStage") as usize)
}

pub(super) fn valid_input(slot: usize, stack: &ItemStack) -> bool {
    slot < 2 && CONTRACTS.iter().any(|contract| is_plain(stack, contract.inputs[slot].0))
}

pub(super) fn accepts(entity: &ModBlockEntity, slot: usize, stack: &ItemStack) -> bool {
    slot < 2 && current(entity).is_some_and(|contract| is_plain(stack, contract.inputs[slot].0))
}

pub(super) fn tick(entity: &mut ModBlockEntity) {
    let Some(contract) = current(entity) else {
        set_number(entity, "Status", 8);
        return;
    };
    let outputs_fit = entity.items[2..4].iter().zip(contract.rewards).all(|(slot, (item, count))| {
        slot.as_ref().is_none_or(|stack| is_plain(stack, item) && stack.count <= 64 - count)
    });
    if !outputs_fit {
        set_number(entity, "Status", 3);
        return;
    }
    if !ingredients_ready(&entity.items[..2], &contract.inputs) {
        set_number(entity, "Status", 2);
        return;
    }
    for (slot, (_, count)) in entity.items[..2].iter_mut().zip(contract.inputs) {
        consume(slot, count);
    }
    for (slot, (item, count)) in entity.items[2..4].iter_mut().zip(contract.rewards) {
        if let Some(stack) = slot { stack.count += count; }
        else { *slot = Some(ItemStack::new(item, count)); }
    }
    let stage = number(entity, "ContractStage") + 1;
    set_number(entity, "ContractStage", stage);
    set_number(entity, "Status", if stage as usize == CONTRACTS.len() { 8 } else { 1 });
}

pub(super) fn draw_screen(
    host: &dyn Host,
    event: &mut horizon_client::mod_api::ScreenDrawEvent<'_>,
) {
    let [x, y] = event.origin;
    let stage = event.data.get(8).copied().unwrap_or(0).max(0) as usize;
    let Some(contract) = CONTRACTS.get(stage) else {
        let text = horizon_client::mod_api::Text::tr("veio.contract.completed", &[]).plain();
        host.text(event.batch, &text, [x + 9.0, y + 24.0], 0x61a5e8, false);
        return;
    };
    let title = horizon_client::mod_api::Text::tr(contract.title, &[]).plain();
    host.text(event.batch, &title, [x + 9.0, y + 24.0], 0xdce6ef, false);
    for (index, &(item, required)) in contract.inputs.iter().enumerate() {
        let kind = if is_factory_block(item) { "block" } else { "item" };
        let key = format!("{kind}.{}", item.replace(':', "."));
        let name = horizon_client::mod_api::Text::tr(&key, &[]).plain();
        let text = format!("{required} x {name}");
        host.text(event.batch, &text, [x + 9.0, y + 76.0 + index as f32 * 12.0], 0x91a7bd, false);
    }
    let status = event.data.first().copied().unwrap_or(0);
    let key = if status == 3 { "veio.status.blocked" } else { "veio.contract.deliver" };
    host.text(event.batch, &horizon_client::mod_api::Text::tr(key, &[]).plain(),
        [x + 9.0, y + 106.0], 0xffffbb4a, false);
    let progress = horizon_client::mod_api::Text::tr("veio.contract.stage", &[&(stage + 1), &CONTRACTS.len()]).plain();
    host.text(event.batch, &progress, [x + 9.0, y + 130.0], 0x91a7bd, false);
}
