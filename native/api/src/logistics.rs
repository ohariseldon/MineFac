//! Item transport. Production declares the slots; logistics drains their outputs.
//! Recipe ingredients and energy generation belong to production.
use super::*;

pub(super) fn transfer_outputs(host: &mut dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity, time: i64) {
    let [x, _, z] = pos;
    let outputs = output_slots(&entity.id);
    if time.rem_euclid(8) == i64::from((x ^ z) & 1) * 4
        && entity.items.get(outputs.clone()).is_some_and(|slots| slots.iter().any(Option::is_some))
        && let Some(facing) = output_direction(host, pos)
    {
        if entity.id == SPLITTER {
            let before = entity.items[0].as_ref().map_or(0, |stack| stack.count);
            split(host, pos, facing, entity, 2);
            if entity.items[0].as_ref().map_or(0, |stack| stack.count) < before {
                set_number(entity, "MovedAt", stamp(time));
            }
        } else {
            let batch = if entity.id == FAST_BELT { 2 } else { 1 };
            for slot in outputs {
                if let Some(stack) = entity.items[slot].as_ref() {
                    let moved = send(host, pos, facing, stack, batch);
                    if moved > 0 {
                        consume(&mut entity.items[slot], moved);
                        set_number(entity, "MovedAt", stamp(time));
                    }
                }
            }
        }
    }
}

pub(super) fn update_status(entity: &mut ModBlockEntity, time: i64) -> bool {
    if matches!(entity.id.as_str(), BELT | FAST_BELT | SPLITTER) {
        let status = if entity.items[0].is_none() { 0 }
            else if stamp(time) - number(entity, "MovedAt") > 8 { 3 }
            else { 1 };
        set_number(entity, "Status", status);
        return true;
    }
    false
}

pub(super) fn split(
    host: &mut dyn Host,
    pos: [i32; 3],
    facing: Direction,
    entity: &mut ModBlockEntity,
    batch: u32,
) {
    let key = "PreferRight".into();
    let was_right = matches!(entity.nbt.get(&key), Some(Tag::Byte(1)));
    let mut prefer_right = was_right;
    for _ in 0..batch {
        let Some(stack) = entity.items[0].as_ref() else {
            break;
        };
        let preferred = turn(facing, prefer_right);
        let moved = send(host, pos, preferred, stack, 1);
        if moved != 0 {
            prefer_right = !prefer_right;
            consume(&mut entity.items[0], moved);
        } else {
            let moved = send(host, pos, preferred.opposite(), stack, 1);
            if moved == 0 {
                break;
            }
            consume(&mut entity.items[0], moved);
        }
    }
    if was_right != prefer_right {
        entity.nbt.insert(key, Tag::Byte(i8::from(prefer_right)));
    }
}

pub(super) fn send(host: &mut dyn Host, pos: [i32; 3], facing: Direction, stack: &ItemStack, batch: u32) -> u32 {
    let [x, y, z] = pos;
    let [dx, _, dz] = facing.normal();
    let mut offered = stack.clone();
    let count = stack.count.min(batch);
    offered.count = count;
    let returned = host.insert_item([x + dx, y, z + dz], facing.opposite(), offered);
    count - returned.map_or(0, |remainder| remainder.count)
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
    // Glazed terracotta faces the player on placement. Belts run away from them.
    Some(match host.block_property(pos, "facing")?.as_str() {
        "north" => Direction::South,
        "south" => Direction::North,
        "west" => Direction::East,
        "east" => Direction::West,
        _ => return None,
    })
}

