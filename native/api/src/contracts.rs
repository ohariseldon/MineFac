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
        inputs: [(STEEL, 24), (MODULE, 12)],
        rewards: [("minecraft:rail", 64), ("minecraft:powered_rail", 16)],
    },
    Contract {
        title: "veio.contract.village",
        inputs: [(CIRCUIT, 24), (FRAME, 32)],
        rewards: [("minecraft:hopper", 8), ("minecraft:emerald", 8)],
    },
    Contract {
        title: "veio.contract.beacon",
        inputs: [("minecraft:nether_star", 1), (CIRCUIT, 48)],
        rewards: [("minecraft:beacon", 1), (DRILL_MK2, 2)],
    },
];

/// Grande projeto: a torre de transmissão, em quatro fases, como o elevador espacial do
/// Satisfactory. Cada fase pede centenas de peças entregues aos poucos (o progresso fica em
/// `Total` e `Info`) e paga ao fechar. A última concede o avanço `veio:grand_project`.
pub(super) const PHASES: [Contract; 4] = [
    Contract {
        title: "veio.project.phase1",
        inputs: [(STEEL, 200), (MODULE, 100)],
        rewards: [("minecraft:emerald", 32), ("veio:speed_module", 4)],
    },
    Contract {
        title: "veio.project.phase2",
        inputs: [(CIRCUIT, 300), ("veio:motor", 100)],
        rewards: [("minecraft:emerald", 48), ("veio:productivity_module", 4)],
    },
    Contract {
        title: "veio.project.phase3",
        inputs: [("veio:processor", 150), ("veio:automation_core", 50)],
        rewards: [("minecraft:emerald", 64), ("veio:mk2_kit", 8)],
    },
    Contract {
        title: "veio.project.phase4",
        inputs: [("veio:automation_core", 200), ("veio:mk2_kit", 50)],
        rewards: [("minecraft:nether_star", 2), ("minecraft:beacon", 2)],
    },
];

/// Peças por tick que a central recolhe numa fase do grande projeto.
const PHASE_INTAKE: u32 = 4;

/// Depois dos projetos, a central passa a exportar para os povoados sem fim.
/// Esmeraldas compram com aldeões o que a fábrica não produz.
pub(super) const EXPORT: Contract = Contract {
    title: "veio.contract.export",
    inputs: [(CIRCUIT, 4), (STEEL, 4)],
    rewards: [("minecraft:emerald", 4), ("minecraft:experience_bottle", 2)],
};

pub(super) fn current(entity: &ModBlockEntity) -> Option<&'static Contract> {
    let stage = number(entity, "ContractStage") as usize;
    if stage == 3 && !beacon_selected(entity) { Some(&EXPORT) }
    else { Some(for_stage(stage)) }
}

/// A estrela seleciona o desafio opcional sem mudar o estágio salvo.
pub(super) fn beacon_selected(entity: &ModBlockEntity) -> bool {
    entity.items.first().and_then(Option::as_ref)
        .is_some_and(|stack| is_plain(stack, "minecraft:nether_star"))
}

pub(super) fn for_stage(stage: usize) -> &'static Contract {
    CONTRACTS.get(stage).or_else(|| PHASES.get(stage.wrapping_sub(CONTRACTS.len()))).unwrap_or(&EXPORT)
}

/// Fase do grande projeto (0 a 3) num estágio da central.
pub(super) fn phase(stage: usize) -> Option<usize> {
    stage.checked_sub(CONTRACTS.len()).filter(|phase| *phase < PHASES.len())
}

pub(super) fn valid_input(slot: usize, stack: &ItemStack) -> bool {
    slot < 2 && CONTRACTS.iter().chain(&PHASES).chain([&EXPORT]).any(|contract| is_plain(stack, contract.inputs[slot].0))
}

pub(super) fn accepts(entity: &ModBlockEntity, slot: usize, stack: &ItemStack) -> bool {
    if slot >= 2 { return false; }
    if number(entity, "ContractStage") == 3 && slot == 0 {
        return is_plain(stack, CIRCUIT) || is_plain(stack, "minecraft:nether_star");
    }
    current(entity).is_some_and(|contract| is_plain(stack, contract.inputs[slot].0))
}

/// Uma fase do grande projeto: recolhe aos poucos e paga quando as duas metas fecham.
/// Devolve `true` quando a última fase termina.
pub(super) fn tick_phase(entity: &mut ModBlockEntity, phase: usize) -> bool {
    let contract = &PHASES[phase];
    let mut got = [number(entity, "Total").max(0) as u32, number(entity, "Info").max(0) as u32];
    let mut took = false;
    for (index, &(item, need)) in contract.inputs.iter().enumerate() {
        let slot = &mut entity.items[index];
        let Some(stack) = slot.as_ref().filter(|stack| is_plain(stack, item)) else { continue };
        let amount = stack.count.min(PHASE_INTAKE).min(need.saturating_sub(got[index]));
        if amount > 0 {
            consume(slot, amount);
            got[index] += amount;
            took = true;
        }
    }
    set_number(entity, "Total", got[0] as i32);
    set_number(entity, "Info", got[1] as i32);
    if got.iter().zip(contract.inputs).any(|(got, (_, need))| *got < need) {
        set_number(entity, "Status", if took { 1 } else { 2 });
        return false;
    }
    let outputs_fit = entity.items[2..4].iter().zip(contract.rewards).all(|(slot, (item, count))| {
        slot.as_ref().is_none_or(|stack| is_plain(stack, item) && stack.count <= 64 - count)
    });
    if !outputs_fit {
        set_number(entity, "Status", 3);
        return false;
    }
    for (slot, (item, count)) in entity.items[2..4].iter_mut().zip(contract.rewards) {
        if let Some(stack) = slot { stack.count += count; }
        else { *slot = Some(ItemStack::new(item, count)); }
    }
    set_number(entity, "Total", 0);
    set_number(entity, "Info", 0);
    set_number(entity, "ContractStage", number(entity, "ContractStage") + 1);
    set_number(entity, "Status", 1);
    phase + 1 == PHASES.len()
}

pub(super) fn tick(entity: &mut ModBlockEntity) -> bool {
    if let Some(phase) = phase(number(entity, "ContractStage").max(0) as usize) {
        return tick_phase(entity, phase);
    }
    let Some(contract) = current(entity) else {
        set_number(entity, "Status", 8);
        return false;
    };
    let outputs_fit = entity.items[2..4].iter().zip(contract.rewards).all(|(slot, (item, count))| {
        slot.as_ref().is_none_or(|stack| is_plain(stack, item) && stack.count <= 64 - count)
    });
    if !outputs_fit {
        set_number(entity, "Status", 3);
        return false;
    }
    if !ingredients_ready(&entity.items[..2], &contract.inputs) {
        set_number(entity, "Status", 2);
        return false;
    }
    for (slot, (_, count)) in entity.items[..2].iter_mut().zip(contract.inputs) {
        consume(slot, count);
    }
    for (slot, (item, count)) in entity.items[2..4].iter_mut().zip(contract.rewards) {
        if let Some(stack) = slot { stack.count += count; }
        else { *slot = Some(ItemStack::new(item, count)); }
    }
    let stage = number(entity, "ContractStage");
    if (stage as usize) < CONTRACTS.len() && contract.title != EXPORT.title {
        set_number(entity, "ContractStage", stage + 1);
    } else {
        set_number(entity, "Total", number(entity, "Total").saturating_add(1));
    }
    set_number(entity, "Status", 1);
    false
}

pub(super) fn draw_screen(
    host: &dyn Host,
    event: &mut horizon_client::mod_api::ScreenDrawEvent<'_>,
) {
    use horizon_client::mod_api::Text;
    let [x, y] = event.origin;
    let stage = event.data.get(8).copied().unwrap_or(0).max(0) as usize;
    let optional_export = stage == 3 && event.data.get(12).copied().unwrap_or(0) == 0;
    let contract = if optional_export { &EXPORT } else { for_stage(stage) };
    let title = Text::tr(contract.title, &[]).plain();
    host.text(event.batch, &title, [x + 9.0, y + 24.0], presentation::INK, false);
    let in_phase = phase(stage).is_some();
    for (index, &(item, required)) in contract.inputs.iter().enumerate() {
        let got = event.data.get(if index == 0 { 9 } else { 12 }).copied().unwrap_or(0).max(0);
        let text = if in_phase { format!("{got}/{required} {}", item_name(item)) } else { format!("{required} x {}", item_name(item)) };
        host.text(event.batch, &text, [x + 9.0, y + 76.0 + index as f32 * 11.0], presentation::SOFT_INK, false);
    }
    let status = event.data.first().copied().unwrap_or(0);
    let key = if status == 3 { "veio.status.blocked" }
        else if optional_export { "veio.contract.optional_beacon" }
        else { "veio.contract.deliver" };
    let color = if status == 3 { presentation::text_color(3) } else { presentation::INK };
    host.text(event.batch, &Text::tr(key, &[]).plain(), [x + 9.0, y + 104.0], color, false);
    let progress = if stage < CONTRACTS.len() && !optional_export {
        Text::tr("veio.contract.stage", &[&(stage + 1), &CONTRACTS.len()]).plain()
    } else if let Some(phase) = phase(stage) {
        Text::tr("veio.project.stage", &[&(phase + 1), &PHASES.len()]).plain()
    } else {
        Text::tr("veio.contract.exports", &[&event.data.get(9).copied().unwrap_or(0)]).plain()
    };
    host.text(event.batch, &progress, [x + 9.0, y + 128.0], presentation::SOFT_INK, false);
}

pub(super) fn item_name(item: &str) -> String {
    let kind = if is_factory_block(item) { "block" } else { "item" };
    horizon_client::mod_api::Text::tr(&format!("{kind}.{}", item.replace(':', ".")), &[]).plain()
}
