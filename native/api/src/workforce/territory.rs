//! Território das obras: o jogador não quebra nem constrói dentro das ruínas da vila e das obras
//! em construção ou de pé (a planta mais `site::MARGIN` blocos, publicada pelo canteiro).
//!
//! - **Livre:** terra, grama, flores, folhas, areia, cascalho e neve (o chão natural) e, em modo
//!   criativo, tudo.
//! - **Ruína à espera:** os blocos que o mestre de obras não derruba (baús, camas, máquinas...)
//!   podem sair, senão a conferência do terreno pararia para sempre.
//! - **Usar item:** comida, ferramentas de uso (esmeralda, chave, livro, bússola...) e o bloco do
//!   lugar reservado (loja na praça, prefeitura) passam; abrir portas, baús e blocos do Veio sem
//!   agachar também. O resto (pôr blocos, baldes, isqueiro, enxada...) é recusado.
//! - **Explosões** não levam os blocos protegidos.
//!
//! Os ganchos saem na primeira leitura quando nenhum canteiro publicou território.
use super::*;
use super::site::Guard;

/// Modo de jogo criativo (`NativeServerPlayer::game_mode`).
const CREATIVE: i32 = 1;

/// Chão e plantas naturais: o jogador mexe à vontade, mesmo dentro do território.
pub(crate) fn natural(block: &str) -> bool {
    let Some(name) = block.strip_prefix("minecraft:") else { return false };
    if name.contains("pot") || name.contains("sandstone") || name.contains("carpet") {
        return false;
    }
    const PLAIN: [&str; 36] = ["grass_block", "dirt", "coarse_dirt", "podzol", "rooted_dirt", "mycelium", "mud", "sand",
        "red_sand", "gravel", "snow", "snow_block", "powder_snow", "ice", "short_grass", "grass", "tall_grass", "fern",
        "large_fern", "dead_bush", "sweet_berry_bush", "vine", "lily_pad", "sugar_cane", "dandelion", "poppy",
        "blue_orchid", "allium", "azure_bluet", "oxeye_daisy", "cornflower", "lily_of_the_valley", "sunflower", "lilac",
        "rose_bush", "peony"];
    PLAIN.contains(&name) || name.ends_with("_leaves") || name.ends_with("_tulip") || name.ends_with("_sapling")
        || name.ends_with("_mushroom") || matches!(name, "torchflower" | "pink_petals" | "wildflowers" | "bush" | "firefly_bush")
}

/// Comida que o jogador come com o botão direito (não é bloco).
fn edible(item: &str) -> bool {
    // O bolo vale como comida, mas é um bloco: no território, não vai ao chão.
    if item == "minecraft:cake" {
        return false;
    }
    if is_food(item) {
        return true;
    }
    let Some(name) = item.strip_prefix("minecraft:") else { return false };
    name.starts_with("cooked_") || name.ends_with("_stew") || name.ends_with("_soup") || matches!(name,
        "apple" | "golden_apple" | "enchanted_golden_apple" | "melon_slice" | "sweet_berries" | "glow_berries"
        | "chorus_fruit" | "carrot" | "golden_carrot" | "potato" | "baked_potato" | "poisonous_potato" | "beetroot"
        | "bread" | "cookie" | "pumpkin_pie" | "dried_kelp" | "honey_bottle" | "beef" | "porkchop" | "chicken"
        | "mutton" | "rabbit" | "cod" | "salmon" | "tropical_fish" | "pufferfish" | "rotten_flesh" | "spider_eye")
}

/// Itens que só se usam (não põem nem mudam blocos): passam no território.
pub(crate) fn harmless(item: &str, reserved: &str) -> bool {
    if item.is_empty() || item == "minecraft:air" || edible(item) || (!reserved.is_empty() && item == reserved) {
        return true;
    }
    if matches!(item, WAGE | WRENCH | "veio:clipboard" | "veio:route_card" | "veio:repair_kit") || upgrades::index(item).is_some() {
        return true;
    }
    let Some(name) = item.strip_prefix("minecraft:") else { return false };
    name.ends_with("_sword") || name.ends_with("_pickaxe") || name.ends_with("_helmet") || name.ends_with("_chestplate")
        || name.ends_with("_leggings") || name.ends_with("_boots") || name.ends_with("potion") || matches!(name,
        "compass" | "recovery_compass" | "clock" | "book" | "writable_book" | "written_book" | "enchanted_book" | "paper"
        | "map" | "filled_map" | "name_tag" | "lead" | "spyglass" | "goat_horn" | "bow" | "crossbow" | "trident"
        | "shield" | "ender_pearl" | "snowball" | "egg" | "fishing_rod" | "milk_bucket" | "firework_rocket"
        | "totem_of_undying" | "elytra" | "experience_bottle" | "glass_bottle" | "stick" | "emerald")
}

/// Blocos que se abrem ou usam com o botão direito: o clique sem agachar não põe nada.
fn interactive(block: &str) -> bool {
    block.starts_with("veio:") || blueprints::protected(block) || block.ends_with("_trapdoor") || block.ends_with("_fence_gate")
        || block.ends_with("_button") || block == "minecraft:lever"
}

/// O jogador pode quebrar `block` em `at`, dentro do território `guard`?
pub(crate) fn may_break(guard: &Guard, at: [i32; 3], block: &str, waiting: bool) -> bool {
    if natural(block) {
        return true;
    }
    // O canteiro do jogador (não o da ruína) sai: o prédio deixa de valer.
    if at == guard.site {
        return !guard.fixed;
    }
    // Ruína à espera da obra: o que o mestre de obras não derruba precisa poder sair.
    waiting && blueprints::protected(block)
}

fn creative(host: &dyn Host) -> bool {
    host.acting_player().is_some_and(|player| player.game_mode == CREATIVE)
}

fn warn(host: &mut dyn Host) {
    if let Some(player) = host.acting_player() {
        host.action_bar(player.uuid, Text::tr("veio.site.protected", &[]));
    }
}

/// A ruína ainda espera a obra (não começou a construir)?
fn waiting(host: &mut dyn Host, guard: &Guard) -> bool {
    let mut phase = None;
    host.change_mod_block_entity(guard.site, &mut |entity| {
        if entity.id == SITE {
            phase = Some(number(entity, "ContractStage"));
        }
    });
    phase.is_none_or(|phase| phase < site::BUILDING)
}

/// Quebrar (e começar a quebrar) um bloco: `destroy_block` e `left_click_block`.
pub(crate) fn destroy(host: &mut dyn Host, pos: [i32; 3], finished: bool) -> Flow {
    if !site::any_territory() {
        return Flow::Continue;
    }
    let dimension = host.dimension();
    let Some(guard) = site::guard_at(dimension, pos, defense::clock()) else { return Flow::Continue };
    if creative(host) {
        if finished && pos == guard.site {
            site::removed(dimension, pos);
        }
        return Flow::Continue;
    }
    let block = host.block(pos);
    let waiting = guard.fixed && waiting(host, &guard);
    if may_break(&guard, pos, &block, waiting) {
        if finished && pos == guard.site {
            site::removed(dimension, pos);
        }
        return Flow::Continue;
    }
    warn(host);
    Flow::Cancel
}

/// Usar um item num bloco (`use_item_on`): pôr blocos e mudar o terreno são recusados.
pub(crate) fn use_item(host: &mut dyn Host, item: &str, pos: [i32; 3]) -> Flow {
    if !site::any_territory() {
        return Flow::Continue;
    }
    let Some(guard) = site::guard_at(host.dimension(), pos, defense::clock()) else { return Flow::Continue };
    if harmless(item, guard.reserved) || creative(host) || (interactive(&host.block(pos)) && !host.sneaking()) {
        return Flow::Continue;
    }
    warn(host);
    Flow::Cancel
}

/// Explosão: tira da lista os blocos protegidos (o chão natural ainda voa).
pub(crate) fn explosion(host: &dyn Host, dimension: &str, blocks: &mut Vec<[i32; 3]>) {
    if !site::any_territory() || blocks.is_empty() {
        return;
    }
    let areas = site::territories(dimension, defense::clock());
    if areas.is_empty() {
        return;
    }
    let inside = |at: &[i32; 3]| areas.iter().any(|(low, high)| (0..3).all(|axis| low[axis] <= at[axis] && at[axis] <= high[axis]));
    blocks.retain(|at| !inside(at) || natural(&host.block(*at)));
}
