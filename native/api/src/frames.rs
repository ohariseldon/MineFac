//! Máquinas grandes: o bloco da máquina é o núcleo, e ao ser colocado monta em volta a
//! estrutura da sua planta (`tools/machine_frames.py`).
//!
//! - O núcleo fica na fileira da frente, com a frente livre: painel, luz de estado, entrada
//!   pela frente e energia pela área do poste continuam nele, como antes.
//! - A estrutura é feita de peças sem inventário (`veio:structure`) e de escotilhas: a de saída
//!   (`veio:hatch_out`, atrás) recebe o que o núcleo produz para esteiras, braços e funis
//!   pegarem; a de entrada (`veio:hatch_in`, nas laterais) passa ao núcleo o que recebe.
//! - Colocar exige a planta livre e chão firme embaixo de toda ela: sem isso a máquina não é
//!   posta e os blocos no caminho ficam contornados de vermelho por alguns segundos.
//! - Quebrar qualquer parte desmonta a máquina inteira e devolve o núcleo (como quebrar o
//!   núcleo). A estrutura não sai em explosões; se faltar uma peça, o núcleo a repõe.
//! - Máquinas de mundos antigos (colocadas antes, sem `Frame`) seguem compactas e funcionando.
use super::*;
use horizon_client::mod_api::{NativeBlockEdit, NativeBlockRim, NativeBlockRun, Text};
use std::sync::Mutex;
use std::time::{Duration, Instant};
mod data;

pub(super) const STRUCTURE: &str = "veio:structure";
pub(super) const HATCH_IN: &str = "veio:hatch_in";
pub(super) const HATCH_OUT: &str = "veio:hatch_out";
/// Itens que uma escotilha guarda de cada vez (o menor tamanho de pilha das saídas).
const HATCH_STACK: u32 = 16;
/// Ticks entre as passagens de itens das escotilhas.
const HATCH_EVERY: i64 = 4;
/// Ticks entre as conferências da estrutura pelo núcleo.
const REPAIR_EVERY: u64 = 200;
const CREATIVE: i32 = 1;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Part { Piece(&'static str), HatchIn, HatchOut }

pub(super) struct Cell { pub at: [i32; 3], pub part: Part }

pub(super) struct Layout {
    pub tier: &'static str,
    /// Largura, altura e profundidade, em blocos.
    pub size: [i32; 3],
    pub cells: &'static [Cell],
}

pub(super) fn layout(block: &str) -> Option<&'static Layout> {
    data::layout(block)
}

/// Peça ou escotilha de uma estrutura.
pub(super) fn is_part(block: &str) -> bool {
    matches!(block, STRUCTURE | HATCH_IN | HATCH_OUT)
}

pub(super) fn is_hatch(block: &str) -> bool {
    matches!(block, HATCH_IN | HATCH_OUT)
}

/// Gira um deslocamento da planta (olhando para o sul) para a direção da máquina, como o
/// `y` dos blockstates gira os modelos.
fn rotate([x, y, z]: [i32; 3], facing: &str) -> [i32; 3] {
    match facing {
        "west" => [-z, y, x],
        "north" => [-x, y, -z],
        "east" => [z, y, -x],
        _ => [x, y, z],
    }
}

fn offset(core: [i32; 3], at: [i32; 3], facing: &str) -> [i32; 3] {
    let [dx, dy, dz] = rotate(at, facing);
    [core[0] + dx, core[1] + dy, core[2] + dz]
}

/// As células da planta no mundo, com o que vai em cada uma.
pub(super) fn cells<'a>(core: [i32; 3], facing: &'a str, layout: &'static Layout) -> impl Iterator<Item = ([i32; 3], Part)> + 'a {
    layout.cells.iter().map(move |cell| (offset(core, cell.at, facing), cell.part))
}

/// O estado de bloco de uma parte, como os comandos o escrevem.
fn state(part: Part, facing: &str, tier: &str) -> String {
    match part {
        Part::Piece(piece) => format!("{STRUCTURE}[facing={facing},piece={piece}]"),
        Part::HatchIn => format!("{HATCH_IN}[facing={facing},tier={tier}]"),
        Part::HatchOut => format!("{HATCH_OUT}[facing={facing},tier={tier}]"),
    }
}

fn part_block(part: Part) -> &'static str {
    match part { Part::Piece(_) => STRUCTURE, Part::HatchIn => HATCH_IN, Part::HatchOut => HATCH_OUT }
}

/// Some ao montar a máquina: ar, plantas rasteiras e neve fina.
pub(super) fn replaceable(block: &str) -> bool {
    matches!(block, "minecraft:air" | "minecraft:cave_air" | "minecraft:void_air" | "minecraft:short_grass"
        | "minecraft:tall_grass" | "minecraft:fern" | "minecraft:large_fern" | "minecraft:dead_bush"
        | "minecraft:snow" | "minecraft:dandelion" | "minecraft:poppy" | "minecraft:blue_orchid"
        | "minecraft:allium" | "minecraft:azure_bluet" | "minecraft:oxeye_daisy" | "minecraft:cornflower"
        | "minecraft:lily_of_the_valley" | "minecraft:red_tulip" | "minecraft:orange_tulip"
        | "minecraft:white_tulip" | "minecraft:pink_tulip" | "minecraft:sunflower" | "minecraft:lilac"
        | "minecraft:rose_bush" | "minecraft:peony" | "minecraft:bush" | "minecraft:leaf_litter"
        | "minecraft:short_dry_grass" | "minecraft:tall_dry_grass" | "minecraft:firefly_bush"
        | "minecraft:wildflowers" | "minecraft:pink_petals")
}

/// Chão que segura uma máquina: nada de buraco, água, lava ou folhas.
pub(super) fn firm(block: &str) -> bool {
    !replaceable(block) && !block.ends_with("_leaves")
        && !matches!(block, "minecraft:water" | "minecraft:lava" | "minecraft:bubble_column" | "minecraft:powder_snow")
}

/// Posições que impedem a máquina em `core`: células ocupadas e buracos embaixo da planta.
/// `with_core`: confere também a célula do núcleo (antes de colocá-lo).
pub(super) fn obstacles(host: &dyn Host, core: [i32; 3], facing: &str, layout: &'static Layout, with_core: bool) -> Vec<[i32; 3]> {
    let mut blocked = Vec::new();
    let mut check = |at: [i32; 3], ground: bool, own: bool| {
        if own && !replaceable(&host.block(at)) {
            blocked.push(at);
        }
        if ground {
            let below = [at[0], at[1] - 1, at[2]];
            if !firm(&host.block(below)) {
                blocked.push(below);
            }
        }
    };
    check(core, true, with_core);
    for (at, _) in cells(core, facing, layout) {
        check(at, at[1] == core[1], true);
    }
    blocked
}

/// A máquina olha para quem a põe: o lado de `target` mais perto do jogador.
fn facing_towards(target: [i32; 3], player: [f64; 3]) -> &'static str {
    let dx = player[0] - (f64::from(target[0]) + 0.5);
    let dz = player[2] - (f64::from(target[2]) + 0.5);
    if dx.abs() > dz.abs() {
        if dx > 0.0 { "east" } else { "west" }
    } else if dz > 0.0 { "south" } else { "north" }
}

// ---------- Contorno dos blocos no caminho (mostrado pela janela) ----------

/// No jogo de um jogador só o servidor e a janela rodam no mesmo processo: o servidor guarda
/// aqui o que barrou a última máquina e a janela contorna esses blocos.
static REFUSED: Mutex<Option<(Instant, Vec<[i32; 3]>)>> = Mutex::new(None);
const SHOW_REFUSED: Duration = Duration::from_secs(6);

fn remember_refused(blocked: &[[i32; 3]]) {
    if let Ok(mut refused) = REFUSED.lock() {
        *refused = Some((Instant::now(), blocked.iter().copied().take(64).collect()));
    }
}

pub(super) fn visuals(rims: &mut Vec<NativeBlockRim>) {
    let Ok(mut refused) = REFUSED.lock() else { return };
    let Some((when, blocked)) = refused.as_ref() else { return };
    if when.elapsed() > SHOW_REFUSED {
        *refused = None;
        return;
    }
    rims.extend(blocked.iter().map(|&pos| NativeBlockRim { pos, argb: 0xffe8322a, width: 3.0 }));
}

fn tell(host: &mut dyn Host, line: Text) {
    if let Some(player) = host.acting_player() {
        host.action_bar(player.uuid, line);
    }
}

fn refuse(host: &mut dyn Host, layout: &Layout, blocked: &[[i32; 3]]) {
    remember_refused(blocked);
    let [w, h, d] = layout.size;
    tell(host, Text::tr("veio.frame.blocked", &[&w, &d, &h, &blocked.len()]));
}

// ---------- Colocar ----------

/// Antes de pôr uma máquina grande: recusa se a planta não cabe ali.
pub(super) fn use_item(host: &mut dyn Host, item: &str, pos: [i32; 3], face: Direction) -> Flow {
    let Some(layout) = layout(item) else { return Flow::Continue };
    let clicked = host.block(pos);
    // Clicar num bloco do Veio abre o painel dele; nada é posto.
    if clicked.starts_with("veio:") && !host.sneaking() {
        return Flow::Continue;
    }
    let target = if replaceable(&clicked) {
        pos
    } else {
        let [dx, dy, dz] = face.normal();
        [pos[0] + dx, pos[1] + dy, pos[2] + dz]
    };
    let Some(player) = host.acting_player() else { return Flow::Continue };
    let facing = facing_towards(target, player.position);
    let blocked = obstacles(host, target, facing, layout, true);
    if blocked.is_empty() {
        return Flow::Continue;
    }
    refuse(host, layout, &blocked);
    Flow::Cancel
}

/// Depois de pôr: monta a estrutura (ou, se a direção final não couber, desfaz e devolve).
pub(super) fn placed(host: &mut dyn Host, item: &str, pos: [i32; 3]) {
    let Some(layout) = layout(item) else { return };
    if host.block(pos) != item {
        return;
    }
    let facing = host.block_property(pos, "facing").unwrap_or_else(|| "south".to_owned());
    let blocked = obstacles(host, pos, &facing, layout, false);
    if !blocked.is_empty() {
        set_blocks(host, &[pos], "minecraft:air");
        if !host.acting_player().is_some_and(|player| player.game_mode == CREATIVE) {
            host.give_items(item, 1);
        }
        refuse(host, layout, &blocked);
        return;
    }
    build(host, pos, &facing, layout, false);
    host.change_mod_block_entity(pos, &mut |entity| set_number(entity, "Frame", 1));
    host.play_sound("minecraft:block.smithing_table.use", center(pos), 4, 0.8, 0.8);
    tell(host, Text::tr("veio.frame.built", &[&layout.size[0], &layout.size[2], &layout.size[1]]));
}

/// Põe as partes da planta. `only_missing`: só onde não há nada (a reposição pelo núcleo).
fn build(host: &mut dyn Host, core: [i32; 3], facing: &str, layout: &'static Layout, only_missing: bool) -> usize {
    let mut palette: Vec<String> = Vec::new();
    let mut runs = Vec::new();
    for (at, part) in cells(core, facing, layout) {
        if only_missing {
            let here = host.block(at);
            if here == part_block(part) || !replaceable(&here) {
                continue;
            }
        }
        let wanted = state(part, facing, layout.tier);
        let index = palette.iter().position(|known| *known == wanted).unwrap_or_else(|| {
            palette.push(wanted);
            palette.len() - 1
        });
        runs.push(NativeBlockRun { x: at[0], y: at[1], z: at[2], height: 1, surface: None, block: index as u32, only: None });
    }
    let count = runs.len();
    if count > 0 {
        let _ = host.edit_blocks(host.dimension(), &NativeBlockEdit { palette, filters: Vec::new(), runs });
    }
    count
}

fn set_blocks(host: &mut dyn Host, positions: &[[i32; 3]], block: &str) {
    if positions.is_empty() {
        return;
    }
    let runs = positions.iter().map(|at| NativeBlockRun { x: at[0], y: at[1], z: at[2], height: 1, surface: None, block: 0, only: None }).collect();
    let _ = host.edit_blocks(host.dimension(), &NativeBlockEdit { palette: vec![block.to_owned()], filters: Vec::new(), runs });
}

// ---------- Achar o núcleo de uma parte ----------

/// A máquina em `core` (montada) tem uma parte em `pos`?
fn owns_cell(host: &dyn Host, core: [i32; 3], pos: [i32; 3]) -> bool {
    let block = host.block(core);
    let Some(layout) = layout(&block) else { return false };
    let facing = host.block_property(core, "facing").unwrap_or_else(|| "south".to_owned());
    cells(core, &facing, layout).any(|(at, _)| at == pos)
}

/// Procura o núcleo dono da parte em `pos`: as plantas vão até 4 blocos para os lados e para
/// trás e até 4 para cima do núcleo.
pub(super) fn find_core(host: &dyn Host, pos: [i32; 3]) -> Option<[i32; 3]> {
    for dy in 0..=4 {
        for dx in -4..=4 {
            for dz in -4..=4 {
                let core = [pos[0] + dx, pos[1] - dy, pos[2] + dz];
                if layout(&host.block(core)).is_some() && owns_cell(host, core, pos) {
                    return Some(core);
                }
            }
        }
    }
    None
}

/// O núcleo, quando `pos` é uma escotilha; senão a própria posição.
pub(super) fn owner(host: &dyn Host, pos: [i32; 3]) -> [i32; 3] {
    if is_hatch(&host.block(pos)) { find_core(host, pos).unwrap_or(pos) } else { pos }
}

fn cached_core(host: &dyn Host, pos: [i32; 3], entity: &ModBlockEntity) -> Option<[i32; 3]> {
    let has = number(entity, "CoreSet") == 1;
    let core = [signed(entity, "CoreX"), signed(entity, "CoreY"), signed(entity, "CoreZ")];
    (has && owns_cell(host, core, pos)).then_some(core)
}

fn signed(entity: &ModBlockEntity, key: &str) -> i32 {
    with_nbt_key(key, |key| match entity.nbt.get(key) { Some(Tag::Int(value)) => *value, _ => 0 })
}

fn set_signed(entity: &mut ModBlockEntity, key: &str, value: i32) {
    with_nbt_key(key, |key| { entity.nbt.insert(key.clone(), Tag::Int(value)); });
}

// ---------- Quebrar ----------

/// Quebrar uma parte ou um núcleo montado: a máquina toda sai junta.
pub(super) fn destroy(host: &mut dyn Host, pos: [i32; 3]) {
    let block = host.block(pos);
    if is_part(&block) {
        if let Some(core) = find_core(host, pos) {
            dismantle(host, core, Some(pos));
            // O núcleo sai como se o jogador o quebrasse: com o próprio item.
            host.run_command(&format!("setblock {} {} {} minecraft:air destroy", core[0], core[1], core[2]));
        }
        return;
    }
    if layout(&block).is_some() {
        dismantle(host, pos, None);
    }
}

/// Começar a quebrar uma parte solta (sem núcleo) a tira na hora.
pub(super) fn left_click(host: &mut dyn Host, pos: [i32; 3]) {
    if is_part(&host.block(pos)) && find_core(host, pos).is_none() {
        set_blocks(host, &[pos], "minecraft:air");
    }
}

/// Tira as partes da máquina em `core` (menos `keep`, que o jogador está quebrando). O que
/// estiver nas escotilhas vai para o jogador.
fn dismantle(host: &mut dyn Host, core: [i32; 3], keep: Option<[i32; 3]>) {
    let block = host.block(core);
    let Some(layout) = layout(&block) else { return };
    let facing = host.block_property(core, "facing").unwrap_or_else(|| "south".to_owned());
    let mut gone = Vec::new();
    let mut loot: Vec<ItemStack> = Vec::new();
    for (at, part) in cells(core, &facing, layout) {
        if host.block(at) != part_block(part) {
            continue;
        }
        if matches!(part, Part::HatchIn | Part::HatchOut) {
            host.change_mod_block_entity(at, &mut |entity| {
                loot.extend(entity.items.iter_mut().filter_map(Option::take));
            });
        }
        if Some(at) != keep {
            gone.push(at);
        }
    }
    for stack in loot {
        host.give_items(&stack.item, stack.count);
    }
    set_blocks(host, &gone, "minecraft:air");
}

/// Explosões não levam a estrutura (o núcleo segue a regra de sempre).
pub(super) fn explosion(host: &dyn Host, blocks: &mut Vec<[i32; 3]>) {
    blocks.retain(|at| !is_part(&host.block(*at)));
}

/// Clique com a mão vazia numa parte: diz de que máquina é e onde fica o painel.
pub(super) fn right_click(host: &mut dyn Host, pos: [i32; 3], empty_hand: bool) -> Option<Flow> {
    if !empty_hand {
        return None;
    }
    let core = find_core(host, pos)?;
    let name = format!("block.{}", host.block(core).replace(':', "."));
    tell(host, Text::tr("veio.frame.part", &[&Text::tr(&name, &[]).plain()]));
    Some(Flow::Cancel)
}

/// A chave não gira uma máquina montada: a estrutura ficaria torta.
pub(super) fn fixed(host: &mut dyn Host, pos: [i32; 3], block: &str) -> bool {
    if layout(block).is_none() {
        return false;
    }
    let mut framed = false;
    host.change_mod_block_entity(pos, &mut |entity| framed = number(entity, "Frame") == 1);
    if framed {
        tell(host, Text::tr("veio.frame.fixed", &[]));
    }
    framed
}

// ---------- Ticks ----------

/// Núcleo montado: de tempos em tempos repõe peças que faltam (explosão, obra, cópia).
pub(super) fn core_tick(host: &mut dyn Host, pos: [i32; 3], entity: &ModBlockEntity, clock: u64) {
    let phase = (pos[0].wrapping_mul(7) ^ pos[2].wrapping_mul(13) ^ pos[1]) as u64;
    if clock.wrapping_add(phase) % REPAIR_EVERY != 0 {
        return;
    }
    let Some(layout) = layout(&entity.id) else { return };
    if number(entity, "Frame") != 1 {
        return;
    }
    let facing = host.block_property(pos, "facing").unwrap_or_else(|| "south".to_owned());
    build(host, pos, &facing, layout, true);
}

/// Escotilhas: a de entrada passa ao núcleo o que recebeu; a de saída puxa o que ele fez.
pub(super) fn hatch_tick(host: &mut dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity, time: i64) {
    let phase = i64::from(pos[0] * 3 + pos[1] * 5 + pos[2] * 7).rem_euclid(HATCH_EVERY);
    if time.rem_euclid(HATCH_EVERY) != phase {
        return;
    }
    let core = match cached_core(host, pos, entity) {
        Some(core) => core,
        None => {
            let Some(core) = find_core(host, pos) else { return };
            set_number(entity, "CoreSet", 1);
            set_signed(entity, "CoreX", core[0]);
            set_signed(entity, "CoreY", core[1]);
            set_signed(entity, "CoreZ", core[2]);
            core
        }
    };
    if entity.id == HATCH_IN {
        if let Some(stack) = entity.items.first_mut().and_then(Option::take) {
            let rest = host.insert_item(core, Direction::Up, stack);
            entity.items[0] = rest.filter(|stack| stack.count > 0);
        }
        return;
    }
    let Some(slot) = entity.items.first_mut() else { return };
    let held = slot.as_ref().map_or(0, |stack| stack.count);
    if held >= HATCH_STACK {
        return;
    }
    host.change_mod_block_entity(core, &mut |machine| {
        let outputs = output_slots(&machine.id);
        for index in outputs {
            let Some(Some(made)) = machine.items.get_mut(index) else { continue };
            let room = HATCH_STACK - slot.as_ref().map_or(0, |stack| stack.count);
            if room == 0 { break; }
            match slot {
                Some(stack) if !(stack.item == made.item && stack.damage == made.damage && stack.extra == made.extra) => continue,
                _ => {}
            }
            let moved = made.count.min(room);
            match slot {
                Some(stack) => stack.count += moved,
                None => *slot = Some(ItemStack { count: moved, ..made.clone() }),
            }
            made.count -= moved;
            if made.count == 0 {
                machine.items[index] = None;
            }
        }
    });
}

/// A escotilha de entrada só aceita o que o núcleo usaria.
pub(super) fn hatch_accepts(host: &dyn Host, pos: [i32; 3], entity: &ModBlockEntity, stack: &ItemStack) -> bool {
    let Some(core) = cached_core(host, pos, entity).or_else(|| find_core(host, pos)) else { return false };
    let block = host.block(core);
    layout_slots(&block).is_some_and(|inputs| inputs.iter().any(|&slot| accepts(&block, slot, stack)))
}

fn layout_slots(block: &str) -> Option<Vec<usize>> {
    super::layout(block).map(|(_, inputs, _)| inputs)
}

#[cfg(test)]
mod tests;
