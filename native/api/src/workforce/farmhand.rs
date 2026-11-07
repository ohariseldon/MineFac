//! Posto agrícola: o trabalhador colhe as plantações maduras num 9x9 em volta do posto
//! (mesma área da colheitadeira), deixando a planta replantada, e leva o produto ao posto.
//!
//! Plantas: as de `farm::crop` (trigo, cenoura, batata, beterraba, fungo do Nether, bagas e
//! cacau). Não usa energia: o custo é a comida do trabalhador.
use super::*;

/// Posições examinadas por ciclo do posto: uma camada inteira; a volta leva 3 ciclos (30 ticks).
const SCANS: i32 = 81;
/// Plantas recusadas por caminho obstruído, esquecidas a cada volta completa.
const MAX_SKIPPED: usize = 8;
const ABSORB_PER_HARVEST: f32 = 3.0;

fn ripe(host: &dyn Host, spot: [i32; 3]) -> Option<farm::Crop> {
    let crop = farm::crop(&host.block(spot))?;
    let age = host.block_property(spot, "age")?.parse::<u8>().ok()?;
    (age >= crop.ripe).then_some(crop)
}

fn skipped(entity: &ModBlockEntity) -> Vec<[i32; 3]> {
    match entity.nbt.get(&"Skipped".into()) {
        Some(Tag::IntArray(values)) => values.chunks_exact(3).map(|v| [v[0], v[1], v[2]]).collect(),
        _ => Vec::new(),
    }
}

/// Lembra uma planta inalcançável até a próxima volta da varredura.
pub(super) fn skip(entity: &mut ModBlockEntity, spot: [i32; 3]) {
    let mut list = skipped(entity);
    if list.len() >= MAX_SKIPPED {
        list.remove(0);
    }
    list.push(spot);
    entity.nbt.insert("Skipped".into(), Tag::IntArray(list.into_iter().flatten().collect()));
}

/// Posição `index` da varredura: primeiro a camada do posto, depois a de baixo e a de cima.
fn scan_position(pos: [i32; 3], index: i32) -> [i32; 3] {
    let index = index.rem_euclid(farm::AREA);
    let [x, _, z] = farm::scan_position(pos, index % 81);
    [x, pos[1] + [0, -1, 1][(index / 81) as usize], z]
}

/// Próxima planta madura a colher. `Err` traz o estado de espera do painel; `KEEP` mantém o
/// estado atual enquanto a primeira volta da busca não termina.
pub(super) fn next(host: &mut dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity) -> Result<[i32; 3], i32> {
    let room = outputs(&entity.id);
    if let Some(spot) = target(entity) {
        if let Some(crop) = ripe(host, spot) {
            return if farm::drops_fit(&entity.items[room], crop.drops) { Ok(spot) } else { Err(3) };
        }
        set_target(entity, None);
    }
    let avoid = skipped(entity);
    let mut cursor = number(entity, "Scan");
    for _ in 0..SCANS {
        let spot = scan_position(pos, cursor);
        cursor = (cursor + 1) % farm::AREA;
        if cursor == 0 {
            entity.nbt.remove(&"Skipped".into());
        }
        if avoid.contains(&spot) {
            continue;
        }
        let Some(crop) = ripe(host, spot) else { continue };
        if !farm::drops_fit(&entity.items[room.clone()], crop.drops) {
            // Mantém o cursor nesta planta: ela é a primeira quando houver espaço.
            set_number(entity, "Scan", (cursor - 1).rem_euclid(farm::AREA));
            return Err(3);
        }
        set_number(entity, "Scan", cursor);
        set_number(entity, "Searched", 0);
        set_target(entity, Some(spot));
        return Ok(spot);
    }
    set_number(entity, "Scan", cursor);
    let searched = (number(entity, "Searched") + SCANS).min(farm::AREA);
    set_number(entity, "Searched", searched);
    Err(if searched >= farm::AREA { NO_TASK } else { KEEP })
}

/// Colhe a planta ao lado do trabalhador. Devolve o que ele passa a carregar, ou `None` se a
/// planta não estiver mais madura ou o produto não couber no posto.
pub(super) fn perform(host: &mut dyn Host, spot: [i32; 3], entity: &ModBlockEntity) -> Option<Vec<(String, u32)>> {
    let crop = ripe(host, spot)?;
    if !farm::drops_fit(&entity.items[outputs(&entity.id)], crop.drops)
        || !host.set_block_property(spot, "age", &crop.reset.to_string())
    {
        return None;
    }
    pollution::absorb(host.dimension(), spot, ABSORB_PER_HARVEST);
    let at = center(spot);
    host.send_particles(&ParticleOptions::simple(ParticleType::Composter), None, at, [0.3, 0.2, 0.3], 0.0, 6);
    host.play_sound("minecraft:block.crop.break", at, 4, 0.6, 1.0);
    Some(crop.drops.iter().map(|&(item, count)| (item.to_owned(), count)).collect())
}
