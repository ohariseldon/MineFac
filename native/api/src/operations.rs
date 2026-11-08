//! Shared, read-only workstation contract for logistics and future workers.
//! No assignment state, global registry, recipe simulation or polling lives here.
// Snapshot consumers will be added by the parallel workforce implementation.
#![allow(dead_code)]
use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Sector { Extraction, Industry, Farming, Food, Power, Logistics, Projects, Defense, Workforce }

impl Sector {
    pub(crate) fn translation_key(self) -> &'static str {
        match self {
            Self::Extraction => "veio.operation.sector.extraction", Self::Industry => "veio.operation.sector.industry",
            Self::Farming => "veio.operation.sector.farming", Self::Food => "veio.operation.sector.food",
            Self::Power => "veio.operation.sector.power", Self::Logistics => "veio.operation.sector.logistics",
            Self::Projects => "veio.operation.sector.projects", Self::Defense => "veio.operation.sector.defense",
            Self::Workforce => "veio.operation.sector.workforce",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// `Storage`: celeiro e silo. Não é posto de trabalho nem transporte; a fila do capataz os usa
/// como destino das sobras e origem de estoque.
pub(crate) enum StationKind { Workstation, Transport, Infrastructure, Storage }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Profile {
    pub sector: Sector,
    /// A workstation can be considered by task adapters. This grants no worker assignment.
    pub kind: StationKind,
}

pub(crate) fn profile(block: &str) -> Option<Profile> {
    use production::*;
    let sector = match block {
        DRILL | DRILL_MK2 | DRILL_MK3 => Sector::Extraction,
        PRESS | ASSEMBLER | CUTTER | INTEGRATOR | CRUSHER | ALLOY | ELECTRONICS | ELECTRIC_FURNACE
            | workforce::HAULER_POST => Sector::Industry,
        workforce::FOREMAN_DESK | workforce::CONTROL_CENTER => Sector::Workforce,
        HARVESTER | PLANTER | GROW_LAMP | TREE_FARM | RANCH | workforce::FARM_POST | workforce::SHEPHERD_POST => Sector::Farming,
        MILL | KITCHEN | PANTRY => Sector::Food,
        HUB => Sector::Projects,
        TESLA => Sector::Defense,
        BELT | FAST_BELT | SPLITTER | MERGER | inserters::ID | inserters::FAST | inserters::LONG => Sector::Logistics,
        _ if workforce::storage::is_store(block) => return Some(Profile { sector: Sector::Logistics, kind: StationKind::Storage }),
        _ if is_power_block(block) || power::tier(block).is_some() => Sector::Power,
        _ => return None,
    };
    let kind = if sector == Sector::Logistics { StationKind::Transport }
        else if matches!(block, CABLE | POLE | STEEL_POLE | BATTERY | SOLAR | WATER_WHEEL | WIND_TURBINE | GROW_LAMP) {
            StationKind::Infrastructure
        } else { StationKind::Workstation };
    Some(Profile { sector, kind })
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// `Worker`: the post's own villager is missing or not hired (`workforce`). No service is
/// suggested for it; hiring stays an explicit player action.
pub(crate) enum Blockage { None, Input, Output, Power, Fuel, Resource, Environment, Control, Damage, Worker, Unknown }

pub(crate) fn blockage(status: i32) -> Blockage {
    match status {
        0 | 1 | 8 | 11 | 23 | 24 | 26 | 27 | 28 => Blockage::None,
        2 | 16 | 21 | 25 => Blockage::Input,
        3 => Blockage::Output,
        4 | 12 => Blockage::Resource,
        5 => Blockage::Power,
        6 => Blockage::Fuel,
        7 | 10 | 14 | 15 | 17 | 20 | 22 => Blockage::Environment,
        18 | 19 => Blockage::Worker,
        9 => Blockage::Control,
        13 => Blockage::Damage,
        _ => Blockage::Unknown,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Service {
    CheckInputs, CollectOutputs, Refuel, Repair, InspectPower, InspectResource, InspectEnvironment, InspectControl,
}

#[derive(Debug)]
pub(crate) struct SlotView<'a> {
    pub index: usize,
    pub stack: Option<&'a ItemStack>,
}

#[derive(Debug)]
pub(crate) struct StationView<'a> {
    pub block: &'a str,
    pub profile: Profile,
    pub status: i32,
    pub blockage: Blockage,
    pub inputs: Vec<SlotView<'a>>,
    pub outputs: Vec<SlotView<'a>>,
    /// None for mechanical transport and stations without an energy buffer.
    pub energy: Option<(i32, i32)>,
    /// Opportunities to inspect, not assigned jobs or permission to bypass controls.
    pub services: Vec<Service>,
}

/// Inspect only a loaded entity already obtained from the host. The caller owns
/// its world/dimension/position identity and must revalidate before any mutation.
pub(crate) fn inspect(entity: &ModBlockEntity) -> Option<StationView<'_>> {
    let profile = profile(&entity.id)?;
    let (_, inputs, outputs) = layout(&entity.id)?;
    let slots = |indices: Vec<usize>| indices.into_iter().map(|index| SlotView {
        index, stack: entity.items.get(index).and_then(Option::as_ref),
    }).collect::<Vec<_>>();
    let inputs = slots(inputs);
    let outputs = slots(outputs);
    let status = number(entity, "Status");
    let blockage = blockage(status);
    let mut services = Vec::new();
    if profile.kind == StationKind::Workstation {
        // Transport buffers must stay under logistics control, not become haul jobs.
        if outputs.iter().any(|slot| slot.stack.is_some()) { services.push(Service::CollectOutputs); }
        let service = match blockage {
            Blockage::Input => Some(Service::CheckInputs),
            Blockage::Fuel => Some(Service::Refuel),
            Blockage::Damage => Some(Service::Repair),
            Blockage::Power => Some(Service::InspectPower),
            Blockage::Resource => Some(Service::InspectResource),
            Blockage::Environment => Some(Service::InspectEnvironment),
            Blockage::Control => Some(Service::InspectControl),
            _ => None,
        };
        if let Some(service) = service { services.push(service); }
    }
    let energy = (needs_power(&entity.id) || is_power_block(&entity.id) || power::tier(&entity.id).is_some())
        .then(|| (number(entity, "energy"), capacity(&entity.id)));
    Some(StationView { block: &entity.id, profile, status, blockage, inputs, outputs, energy, services })
}

/// Material compatibility for planning. Does not check space, reach or side access.
pub(crate) fn accepts_material(entity: &ModBlockEntity, slot: usize, stack: &ItemStack) -> bool {
    let compatible = if entity.id == HUB { contracts::accepts(entity, slot, stack) }
        else { accepts(&entity.id, slot, stack) };
    if !compatible { return false; }
    // Preserve the slot fence, allocating its layout only for accepted candidates.
    layout(&entity.id).is_some_and(|(_, inputs, _)| inputs.contains(&slot))
}

/// Live delivery validation, shared with host insertion. The host remains responsible
/// for stack limits and moving items; an inspection is never an inventory transaction.
pub(crate) fn accepts_delivery(host: &dyn Host, event: &ContainerAcceptsEvent<'_>) -> bool {
    accepts_material(event.entity, event.slot, event.stack)
        && (event.entity.id != MERGER || logistics::accepts_merger(host, event))
}

pub(crate) fn advice_key(status: i32) -> String {
    let name = usize::try_from(status).ok().and_then(|index| presentation::STATUS_KEYS.get(index))
        .copied().unwrap_or("unknown");
    format!("veio.operation.advice.{name}")
}

#[cfg(test)]
mod tests;
