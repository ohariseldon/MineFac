//! Posto agrícola: o trabalhador colhe as plantações maduras num 9x9 em volta do posto
//! (mesma área da colheitadeira), deixando a planta replantada, e leva o produto ao posto.
//!
//! Plantas: as de `farm::crop` (trigo, cenoura, batata, beterraba, fungo do Nether, bagas e
//! cacau). Não usa energia: o custo é a comida do trabalhador.
use super::*;

/// Posições examinadas por ciclo do posto: uma fileira do 9×9. A volta completa (243 blocos)
/// leva 27 ciclos (270 ticks), no mesmo espírito da colheitadeira (3 blocos/tick).
pub(super) const SCANS: i32 = 9;
const ABSORB_PER_HARVEST: f32 = 3.0;

fn ripe(host: &dyn Host, spot: [i32; 3]) -> Option<farm::Crop> {
    let crop = farm::crop(&host.block(spot))?;
    let age = host.block_property(spot, "age")?.parse::<u8>().ok()?;
    (age >= crop.ripe).then_some(crop)
}

/// Próxima planta madura a colher. `Err` traz o estado de espera do painel; `KEEP` mantém o
/// estado atual enquanto a primeira volta da busca não termina.
pub(super) fn plan(host: &mut dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity, now: u64) -> Result<Plan, i32> {
    let room = outputs(&entity.id);
    let dimension = host.dimension();
    let visit = |spot| Plan::Visit { spot, reach: REACH };
    if let Some(spot) = target(entity) {
        if let Some(crop) = ripe(host, spot) {
            return if farm::drops_fit(&entity.items[room], crop.drops) { Ok(visit(spot)) } else { Err(3) };
        }
        set_target(entity, None);
    }
    let avoid = skipped(entity);
    let mut cursor = number(entity, "Scan");
    for _ in 0..SCANS {
        let spot = area_position(pos, farm::RADIUS, cursor);
        cursor = (cursor + 1) % farm::AREA;
        if cursor == 0 {
            clear_skipped(entity);
        }
        if avoid.contains(&spot) || reserved_by_other(dimension, spot, pos, now) {
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
        return Ok(visit(spot));
    }
    set_number(entity, "Scan", cursor);
    let searched = (number(entity, "Searched") + SCANS).min(farm::AREA);
    set_number(entity, "Searched", searched);
    Err(if searched >= farm::AREA { NO_TASK } else { KEEP })
}

/// Colhe a planta ao lado do trabalhador. Devolve o que ele passa a carregar, ou `None` se a
/// planta não estiver mais madura ou o produto não couber no posto.
pub(super) fn perform(host: &mut dyn Host, spot: [i32; 3], entity: &mut ModBlockEntity) -> Option<Vec<(String, u32)>> {
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
