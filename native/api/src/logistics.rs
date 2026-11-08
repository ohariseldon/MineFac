//! Item transport. Production declares the slots; logistics drains their outputs.
//! Recipe ingredients and energy generation belong to production.
use super::*;

pub(super) fn transfer_outputs(host: &mut dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity, time: i64) {
    let [x, _, z] = pos;
    // No transfer or topology operation can run outside this existing phase.
    if time.rem_euclid(8) != i64::from((x ^ z) & 1) * 4 { return; }
    // A esteira já leu a orientação ao revisar o caminho.
    let belt_output = is_belt(&entity.id).then(|| refresh_path(host, pos, entity));
    if transport(&entity.id) {
        let paused = host.neighbor_signal(host.dimension(), pos) > 0;
        set_number(entity, "LogisticsPaused", i32::from(paused));
        if paused { return; }
    }
    let outputs = output_slots(&entity.id);
    if entity.items.get(outputs.clone()).is_some_and(|slots| slots.iter().any(Option::is_some))
        && let Some(facing) = belt_output.unwrap_or_else(|| output_direction(host, pos))
    {
        if entity.id == MERGER {
            let moved = merge(host, pos, facing, entity, 2);
            if moved > 0 { record_moved(entity, moved, time); }
        } else if entity.id == SPLITTER {
            let before = entity.items[0].as_ref().map_or(0, |stack| stack.count);
            split(host, pos, facing, entity, 2);
            if entity.items[0].as_ref().map_or(0, |stack| stack.count) < before {
                let moved = before - entity.items[0].as_ref().map_or(0, |stack| stack.count);
                record_moved(entity, moved, time);
            }
        } else if entity.id == TUNNEL {
            // Entrada: entrega direto à saída, até 6 blocos à frente, por baixo do que houver.
            let target = if tunnel_is_exit(host, pos, facing, 0) { adjacent(pos, facing, 0) }
                else { tunnel_exit(host, pos, facing).unwrap_or_else(|| adjacent(pos, facing, 0)) };
            if let Some(stack) = entity.items[0].as_ref() {
                let moved = send_to(host, pos, target, facing, stack, 2);
                if moved > 0 {
                    consume(&mut entity.items[0], moved);
                    record_moved(entity, moved, time);
                }
            }
        } else {
            let batch = match entity.id.as_str() { EXPRESS_BELT => 4, FAST_BELT => 2, _ => 1 };
            for slot in outputs {
                if let Some(stack) = entity.items[slot].as_ref() {
                    let moved = if is_belt(&entity.id) {
                        let target = belt_target(host, pos, facing);
                        send_to(host, pos, target, facing, stack, batch)
                    } else { send(host, pos, facing, stack, batch) };
                    if moved > 0 {
                        consume(&mut entity.items[slot], moved);
                        record_moved(entity, moved, time);
                    }
                }
            }
        }
    }
}

pub(super) fn transport(block: &str) -> bool { matches!(block, BELT | FAST_BELT | EXPRESS_BELT | SPLITTER | MERGER | TUNNEL) }

/// Alcance do túnel de esteira: a saída fica a até 6 blocos da entrada, na mesma direção.
pub(super) const TUNNEL_REACH: i32 = 6;

fn same_tunnel(host: &dyn Host, at: [i32; 3], facing: Direction) -> bool {
    host.block(at) == TUNNEL && output_direction(host, at) == Some(facing)
}

/// Primeiro túnel à frente, com a mesma direção: a saída desta entrada.
pub(super) fn tunnel_exit(host: &dyn Host, pos: [i32; 3], facing: Direction) -> Option<[i32; 3]> {
    let [dx, _, dz] = facing.normal();
    (1..=TUNNEL_REACH).map(|step| [pos[0] + dx * step, pos[1], pos[2] + dz * step])
        .find(|&at| same_tunnel(host, at, facing))
}

/// Saída é o túnel cujo par anterior (o primeiro atrás, até 6 blocos) é uma entrada. Os túneis
/// alternam entrada e saída ao longo da linha, como no Factorio.
pub(super) fn tunnel_is_exit(host: &dyn Host, pos: [i32; 3], facing: Direction, depth: u8) -> bool {
    if depth >= 6 { return false; }
    let [dx, _, dz] = facing.normal();
    (1..=TUNNEL_REACH).map(|step| [pos[0] - dx * step, pos[1], pos[2] - dz * step])
        .find(|&at| same_tunnel(host, at, facing))
        .is_some_and(|behind| !tunnel_is_exit(host, behind, facing, depth + 1))
}

fn record_moved(entity: &mut ModBlockEntity, count: u32, time: i64) {
    set_number(entity, "MovedAt", stamp(time));
    if transport(&entity.id) { set_number(entity, "Produced", number(entity, "Produced") + count as i32); }
}

/// Um item que acaba de chegar a uma esteira vazia espera a vez de andar (até `BLOCKED_AFTER`
/// ticks): isso conta como andando, não como bloqueada. Assim a luz não pisca laranja a cada
/// item e a esteira não manda duas atualizações de bloco por item.
const BLOCKED_AFTER: i32 = 8;

pub(super) fn update_status(entity: &mut ModBlockEntity, time: i64) -> bool {
    if matches!(entity.id.as_str(), BELT | FAST_BELT | EXPRESS_BELT | SPLITTER | MERGER | TUNNEL) {
        let empty = entity.items.iter().all(Option::is_none);
        let now = stamp(time);
        if !empty && number(entity, "Status") == 0 {
            set_number(entity, "HeldSince", now);
        }
        let since = number(entity, "MovedAt").max(number(entity, "HeldSince"));
        let status = if number(entity, "LogisticsPaused") != 0 { 9 }
            else if empty { 0 }
            else if now - since > BLOCKED_AFTER { 3 }
            else { 1 };
        set_number(entity, "Status", status);
        return true;
    }
    false
}

/// Esteira comum que, neste tick, não faria nada além de redesenhar a carga (como uma
/// esteira inativa do Factorio): fora da fase de transferência e num estado que
/// `update_status` manteria igual. Vazia e parada (0), travada com carga (3) ou desligada por
/// redstone (9, que só muda na fase). O painel já foi publicado num tick completo antes.
/// Divisores, unificadores e túneis têm configuração ou destino próprio: sempre completos.
pub(super) fn quiet_belt(entity: &ModBlockEntity, pos: [i32; 3], time: i64) -> bool {
    if !matches!(entity.id.as_str(), BELT | FAST_BELT | EXPRESS_BELT) { return false; }
    if time.rem_euclid(8) == i64::from((pos[0] ^ pos[2]) & 1) * 4 { return false; }
    if entity.data.len() != super::presentation::DATA_SLOTS { return false; }
    let empty = entity.items.iter().all(Option::is_none);
    let paused = number(entity, "LogisticsPaused") != 0;
    match number(entity, "Status") {
        0 => empty && !paused,
        3 => !empty && !paused,
        9 => paused,
        _ => false,
    }
}

pub(super) fn display_stack(entity: &ModBlockEntity) -> Option<&ItemStack> {
    if entity.id == MERGER {
        entity.items.iter().find_map(Option::as_ref)
    } else {
        entity.items.first().and_then(Option::as_ref)
    }
}

/// Input buffers stay separate so an upstream line cannot fill both lanes.
pub(super) fn input_slot(output: Direction, face: Direction) -> Option<usize> {
    if face == turn(output, false) { Some(0) }
    else if face == turn(output, true) { Some(1) }
    else { None }
}

pub(super) fn accepts_merger(host: &dyn Host, event: &ContainerAcceptsEvent<'_>) -> bool {
    if event.slot >= 2 { return false; }
    let Some(face) = event.face else { return true; };
    output_direction(host, event.pos).is_some_and(|output| input_slot(output, face) == Some(event.slot))
}

/// The next successful transfer favors the other input. A blocked product
/// stays in its own buffer while an accepted product can use the outlet.
fn merge(host: &mut dyn Host, pos: [i32; 3], facing: Direction, entity: &mut ModBlockEntity, batch: u32) -> u32 {
    let mut next = (number(entity, "NextInput") & 1) as usize;
    let priority = logistics_controls::priority(entity);
    let mut total = 0;
    for _ in 0..batch {
        let mut sent = false;
        let first = match priority { 1 => 0, 2 => 1, _ => next };
        for slot in [first, 1 - first] {
            let Some(stack) = entity.items.get(slot).and_then(Option::as_ref) else { continue; };
            let moved = send(host, pos, facing, stack, 1);
            if moved == 0 { continue; }
            consume(&mut entity.items[slot], moved);
            next = 1 - slot;
            total += moved;
            sent = true;
            break;
        }
        if !sent { break; }
    }
    set_number(entity, "NextInput", next as i32);
    total
}

#[cfg(test)]
mod tests;

pub(super) fn split(
    host: &mut dyn Host,
    pos: [i32; 3],
    facing: Direction,
    entity: &mut ModBlockEntity,
    batch: u32,
) {
    let was_right = with_nbt_key("PreferRight", |key| matches!(entity.nbt.get(key), Some(Tag::Byte(1))));
    let mut prefer_right = was_right;
    let priority = logistics_controls::priority(entity);
    let filtered = logistics_controls::has_filter(entity);
    for _ in 0..batch {
        let Some(stack) = entity.items[0].as_ref() else {
            break;
        };
        let right = if filtered {
            !logistics_controls::allows(entity, stack.item.as_ref())
        } else { match priority { 1 => false, 2 => true, _ => prefer_right } };
        let preferred = turn(facing, right);
        let moved = send(host, pos, preferred, stack, 1);
        if moved != 0 {
            prefer_right = !prefer_right;
            consume(&mut entity.items[0], moved);
        } else {
            // A sorting filter never spills into the wrong lane.
            if filtered { break; }
            let moved = send(host, pos, preferred.opposite(), stack, 1);
            if moved == 0 {
                break;
            }
            consume(&mut entity.items[0], moved);
        }
    }
    if was_right != prefer_right {
        with_nbt_key("PreferRight", |key| {
            entity.nbt.insert(key.clone(), Tag::Byte(i8::from(prefer_right)));
        });
    }
}

pub(super) fn send(host: &mut dyn Host, pos: [i32; 3], facing: Direction, stack: &ItemStack, batch: u32) -> u32 {
    send_to(host, pos, adjacent(pos, facing, 0), facing, stack, batch)
}

fn send_to(host: &mut dyn Host, from: [i32; 3], target: [i32; 3], facing: Direction, stack: &ItemStack, batch: u32) -> u32 {
    onboarding::before_transfer(host);
    let mut offered = stack.clone();
    let count = stack.count.min(batch);
    offered.count = count;
    let returned = host.insert_item(target, facing.opposite(), offered);
    let moved = count - returned.map_or(0, |remainder| remainder.count);
    onboarding::transferred(host, from, target, &stack.item, moved);
    moved
}

pub(super) fn is_belt(block: &str) -> bool { matches!(block, BELT | FAST_BELT | EXPRESS_BELT) }

fn adjacent(pos: [i32; 3], direction: Direction, dy: i32) -> [i32; 3] {
    let [dx, _, dz] = direction.normal();
    [pos[0] + dx, pos[1] + dy, pos[2] + dz]
}

fn air(block: &str) -> bool {
    matches!(block, "minecraft:air" | "minecraft:cave_air" | "minecraft:void_air")
}

/// Only belt-to-belt links change height. Machines keep their existing ports.
pub(super) fn belt_target(host: &dyn Host, pos: [i32; 3], output: Direction) -> [i32; 3] {
    let flat = adjacent(pos, output, 0);
    let block = host.block(flat);
    if !air(&block) {
        if !is_factory_block(&block) && is_belt(&host.block(adjacent(pos, output, 1)))
            && air(&host.block([pos[0], pos[1] + 1, pos[2]])) {
            return adjacent(pos, output, 1);
        }
        return flat;
    }
    let lower = adjacent(pos, output, -1);
    if is_belt(&host.block(lower)) { lower } else { flat }
}

fn incoming(host: &dyn Host, pos: [i32; 3], side: Direction) -> Option<i32> {
    for dy in [0, 1, -1] {
        let other = adjacent(pos, side, dy);
        let block = host.block(other);
        if !is_factory_block(&block) { continue; }
        let Some(output) = output_direction(host, other) else { continue; };
        if block == SPLITTER {
            if dy == 0 && (turn(output, false) == side.opposite() || turn(output, true) == side.opposite()) {
                return Some(0);
            }
            continue;
        }
        if output != side.opposite() { continue; }
        let target = if is_belt(&block) { belt_target(host, other, output) }
            else { adjacent(other, output, 0) };
        if target == pos && !output_slots(&block).is_empty() { return Some(dy); }
    }
    None
}

const HORIZONTAL: [Direction; 4] = [Direction::North, Direction::East, Direction::South, Direction::West];

/// Assinatura da vizinhança: blocos ao redor, orientação dos vizinhos do Veio, a própria
/// orientação e a trava da chave. Quando não muda, a esteira não relê entradas nem reescreve
/// `path`/`facing`. A orientação dos vizinhos entra porque as entradas dependem dela: girar
/// uma esteira vizinha com a chave refaz a curva desta.
fn neighborhood_signature(host: &dyn Host, pos: [i32; 3], facing: &str, manual: i32) -> i32 {
    let mut hash: u32 = 2166136261;
    let mix = |hash: &mut u32, text: &str| {
        for byte in text.as_bytes() {
            *hash ^= u32::from(*byte);
            *hash = hash.wrapping_mul(16777619);
        }
        *hash ^= 0xff;
        *hash = hash.wrapping_mul(16777619);
    };
    mix(&mut hash, facing);
    hash ^= manual as u32;
    hash = hash.wrapping_mul(16777619);
    mix(&mut hash, &host.block([pos[0], pos[1] + 1, pos[2]]));
    for side in HORIZONTAL {
        for dy in [0, 1, -1] {
            let at = adjacent(pos, side, dy);
            let block = host.block(at);
            mix(&mut hash, &block);
            if is_factory_block(&block) {
                mix(&mut hash, &host.block_property(at, "facing").unwrap_or_default());
            }
        }
    }
    hash as i32
}

/// Revisa curva e rampa da esteira. Devolve a saída como ficou (a mesma que
/// `output_direction` leria agora), para a transferência não reler a orientação.
fn refresh_path(host: &mut dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity) -> Option<Direction> {
    let facing = host.block_property(pos, "facing").unwrap_or_default();
    let mut output = output_of(&facing)?;
    let manual = number(entity, "ManualDirection");
    let sig = neighborhood_signature(host, pos, &facing, manual);
    // A assinatura pode ser negativa: compara o valor guardado, sem o piso 0 de `number`.
    let same = with_nbt_key("PathSig", |key| matches!(entity.nbt.get(key), Some(Tag::Int(stored)) if *stored == sig));
    if number(entity, "PathReady") == 1 && same {
        return Some(output);
    }
    let inputs: Vec<_> = HORIZONTAL.into_iter().filter_map(|side| incoming(host, pos, side).map(|dy| (side, dy))).collect();
    // Snap a corner only when one line feeds us and one existing line leaves us.
    // Empty space never overrides the direction chosen while building a new line.
    if number(entity, "ManualDirection") == 0 && inputs.len() == 1 {
        let candidates: Vec<_> = HORIZONTAL.into_iter().filter(|side| *side != inputs[0].0)
            .filter(|side| {
                let target = belt_target(host, pos, *side);
                let block = host.block(target);
                (is_belt(&block) && output_direction(host, target) != Some(side.opposite()))
                    || (target[1] == pos[1] && !is_belt(&block) && has_item_inputs(&block))
            }).collect();
        if candidates.len() == 1 && candidates[0] != output {
            output = candidates[0];
            let facing = match output.opposite() {
                Direction::North => "north", Direction::South => "south",
                Direction::East => "east", Direction::West => "west", _ => unreachable!(),
            };
            host.set_block_property(pos, "facing", facing);
        }
    }
    let target = belt_target(host, pos, output);
    let rear = incoming(host, pos, output.opposite());
    let path = if target[1] > pos[1] { "up" }
        else if rear == Some(1) { "down" }
        else if rear.is_some() { "straight" }
        else if incoming(host, pos, turn(output, false)).is_some() { "left" }
        else if incoming(host, pos, turn(output, true)).is_some() { "right" }
        else { "straight" };
    if host.block_property(pos, "path").as_deref() != Some(path) {
        host.set_block_property(pos, "path", path);
    }
    let facing_now = host.block_property(pos, "facing").unwrap_or_default();
    let stored = if facing_now == facing { sig } else { neighborhood_signature(host, pos, &facing_now, manual) };
    set_number(entity, "PathSig", stored);
    set_number(entity, "PathReady", 1);
    output_of(&facing_now)
}

/// Coordinates follow the same path used by the generated belt models.
pub(super) fn path_point(path: &str, t: f32) -> [f32; 3] {
    let t = t.clamp(0.0, 1.0);
    let a = t * std::f32::consts::FRAC_PI_2;
    match path {
        "left" => [0.5 * a.sin(), 0.1875, 0.5 * a.cos()],
        "right" => [1.0 - 0.5 * a.sin(), 0.1875, 0.5 * a.cos()],
        "up" => [0.5, 0.1875 + t, 1.0 - t],
        "down" => [0.5, 1.1875 - t, 1.0 - t],
        _ => [0.5, 0.1875, 1.0 - t],
    }
}

pub(super) fn orient([x, y, z]: [f32; 3], facing: &str) -> [f32; 3] {
    match facing {
        "west" => [1.0 - z, y, x], "north" => [1.0 - x, y, 1.0 - z],
        "east" => [z, y, 1.0 - x], _ => [x, y, z],
    }
}

pub(super) fn turn(facing: Direction, right: bool) -> Direction {
    let clockwise = match facing {
        Direction::North => Direction::East,
        Direction::East => Direction::South,
        Direction::South => Direction::West,
        Direction::West => Direction::North,
        _ => unreachable!("factory blocks face horizontally"),
    };
    if right { clockwise } else { clockwise.opposite() }
}

pub(super) fn output_direction(host: &dyn Host, pos: [i32; 3]) -> Option<Direction> {
    output_of(&host.block_property(pos, "facing")?)
}

fn output_of(facing: &str) -> Option<Direction> {
    // Glazed terracotta faces the player on placement. Belts run away from them.
    Some(match facing {
        "north" => Direction::South,
        "south" => Direction::North,
        "west" => Direction::East,
        "east" => Direction::West,
        _ => return None,
    })
}

