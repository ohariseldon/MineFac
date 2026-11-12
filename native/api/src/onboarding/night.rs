//! A noite: o medo da vila desde o primeiro dia (`frentes/HISTORIA.md`).
//!
//! Duas partes:
//!
//! - **A primeira noite** acontece onde o jogador estiver no tutorial, no primeiro anoitecer. A vila
//!   se recolhe; o prefeito (ou um morador, se o jogador ainda não falou com ele) avisa, a barra
//!   aponta a casa, e o resto do tutorial espera até o amanhecer. Sobreviver à noite é comemorado.
//! - **O capítulo 2, "A noite"**, vem depois da fábrica, pelo prefeito Otávio (ou
//!   `/veiotutorial noite`): luz na vila, a muralha com portão e uma noite dormida em segurança.
//!   As tochas e os blocos da muralha contam quando o jogador os coloca em volta do centro da vila.
//!
//! Em todo anoitecer depois disso, o sino da vila toca para o jogador e uma frase curta lembra
//! que é hora de voltar (sem repetir o capítulo nem encher o chat).
use super::*;

/// `Progress::night_first`: 0 ainda não anoiteceu no tutorial, 1 primeira noite em curso, 2 passou.
pub(super) const FIRST_DONE: u8 = 2;
pub(super) const STEPS: u8 = 6;
pub(super) const DONE_NIGHT: u8 = STEPS + 1;
pub(super) const DO_LINES: [u8; 6] = [4, 4, 3, 4, 4, 3];
/// Blocos da muralha em volta da fábrica, e a que distância do gerador eles contam.
pub(super) const FACTORY_WALL: u16 = 32;
pub(super) const FACTORY_RING: (i32, i32) = (4, 24);
/// Falas do conselho de defesa (`veio.night.council.<n>`).
const COUNCIL_LINES: u8 = 8;
/// Bits de `Progress::night_flags`.
pub(super) const GATE: u8 = 1;
pub(super) const SLEPT: u8 = 2;
pub(super) const SEEN: u8 = 4;
pub(super) const FACTORY_GATE: u8 = 8;
pub(super) const COUNCIL: u8 = 16;
/// Luzes e blocos de muralha que o capítulo pede.
pub(super) const LIGHTS: u16 = 8;
pub(super) const WALL: u16 = 64;
/// Luzes contam a até isto do centro da vila; a muralha, entre `RING.0` e `RING.1` blocos dele.
pub(super) const LIGHT_REACH: i32 = 40;
pub(super) const RING: (i32, i32) = (12, 64);
/// Frases do anoitecer que se alternam (`veio.night.dusk.<n>`).
const DUSK_LINES: u64 = 4;

/// A etapa (0..3) do capítulo em andamento, se ele está aberto.
pub(super) fn step(p: &Progress) -> Option<usize> {
    (1..=STEPS).contains(&p.night).then(|| usize::from(p.night - 1))
}

/// Luz que conta para a vila.
pub(super) fn is_light(block: &str) -> bool {
    matches!(block, "minecraft:torch" | "minecraft:wall_torch" | "minecraft:lantern" | "minecraft:soul_lantern"
        | "minecraft:soul_torch" | "minecraft:soul_wall_torch" | "minecraft:campfire" | "minecraft:glowstone"
        | "minecraft:sea_lantern" | "minecraft:redstone_lamp" | "minecraft:jack_o_lantern" | "minecraft:shroomlight")
}

/// Bloco de muralha: muros, cercas e pedra de construção.
pub(super) fn is_wall(block: &str) -> bool {
    let name = block.strip_prefix("minecraft:").unwrap_or("");
    name.ends_with("_wall") || name.ends_with("_fence") || name.ends_with("_bricks") || name.ends_with("_planks")
        || matches!(name, "cobblestone" | "stone" | "mossy_cobblestone" | "cobbled_deepslate" | "deepslate_bricks"
            | "polished_andesite" | "polished_diorite" | "polished_granite" | "smooth_stone" | "iron_bars")
}

/// Portão da muralha: portão de cerca ou porta.
pub(super) fn is_gate(block: &str) -> bool {
    block.starts_with("minecraft:") && (block.ends_with("_fence_gate") || block.ends_with("_door"))
}

/// O centro da vila: o que o spawn conhece, a casa do fundador ou nenhum.
fn center() -> Option<[i32; 3]> {
    workforce::spawn::center().map(|(at, _)| at).or_else(home::dashboard)
}

fn horizontal(a: [i32; 3], b: [i32; 3]) -> i32 { (a[0] - b[0]).abs().max((a[2] - b[2]).abs()) }

/// A oficina: o gerador do tutorial, o terreno ou, sem eles, o centro da vila.
pub(super) fn factory(p: &Progress) -> Option<[i32; 3]> {
    p.blocks[GENERATOR_SLOT].or(p.plot).or_else(center)
}

/// O jogador colocou um bloco: conta luz, muralha e portão em volta da vila e da fábrica, com o
/// capítulo aberto.
pub(super) fn placed(p: &mut Progress, dimension: &str, pos: Pos, block: &str) {
    if step(p).is_none() || dimension != workforce::spawn::OVERWORLD { return; }
    if let Some(center) = center() {
        let distance = horizontal(pos, center);
        if is_light(block) && distance <= LIGHT_REACH {
            p.night_lights = (p.night_lights + 1).min(LIGHTS);
            p.dirty = true;
        } else if (RING.0..=RING.1).contains(&distance) {
            if is_gate(block) { p.night_flags |= GATE; p.dirty = true; }
            else if is_wall(block) { p.night_wall = (p.night_wall + 1).min(WALL); p.dirty = true; }
        }
    }
    // A muralha da fábrica (etapa 5): perto do gerador. Um bloco pode valer para as duas.
    if step(p) == Some(4) && let Some(at) = factory(p) {
        let distance = horizontal(pos, at);
        if (FACTORY_RING.0..=FACTORY_RING.1).contains(&distance) {
            if is_gate(block) { p.night_flags |= FACTORY_GATE; p.dirty = true; }
            else if is_wall(block) { p.night_factory_wall = (p.night_factory_wall + 1).min(FACTORY_WALL); p.dirty = true; }
        }
    }
}

/// Deitou na cama à noite: vale para a etapa da noite segura.
pub(super) fn slept(p: &mut Progress) {
    if step(p) == Some(2) { p.night_flags |= SLEPT; p.dirty = true; }
}

/// Abriu uma máquina que polui: viu o número no painel (etapa do preço da fábrica).
pub(super) fn opened(p: &mut Progress, block: &str) {
    if step(p) == Some(3) && pollution::emission(block) > 0.0 { p.night_flags |= SEEN; p.dirty = true; }
}

pub(super) fn ready(step: usize, p: &Progress) -> bool {
    match step {
        0 => p.night_lights >= LIGHTS,
        1 => p.night_wall >= WALL && p.night_flags & GATE != 0,
        2 => p.night_flags & SLEPT != 0,
        3 => p.night_flags & SEEN != 0,
        4 => p.night_factory_wall >= FACTORY_WALL && p.night_flags & FACTORY_GATE != 0,
        _ => p.night_flags & COUNCIL != 0,
    }
}

/// O conselho de defesa: o plano do prefeito para quando a fábrica crescer, com o que cada defesa
/// custa e onde ela falha.
fn council(host: &mut dyn Host, player: &NativeServerPlayer, p: &mut Progress) {
    for line in 1..=COUNCIL_LINES { village::say(host, player.uuid, &format!("veio.night.council.{line}")); }
    p.night_flags |= COUNCIL;
    p.dirty = true;
}

fn speaker(p: &Progress) -> &'static str {
    // Antes de conhecer Otávio, quem avisa é um morador.
    if p.intro > intro::MAYOR_STAGE { "veio.onboarding.mayor.name" } else { "veio.night.villager" }
}

fn say(host: &mut dyn Host, p: &Progress, player: u128, key: &str) {
    let name = Text::tr(speaker(p), &[]).plain();
    host.tell(player, Text::tr("veio.agenda.says", &[&name, &Text::tr(key, &[]).plain()]));
}

fn sound(host: &mut dyn Host, player: &NativeServerPlayer, sound: &str) {
    if spawn_safe(&player.name) {
        host.run_command(&format!("execute as {0} at @s run playsound {sound} master @s", player.name));
    }
}

fn title(host: &mut dyn Host, player: &NativeServerPlayer, title: &str, subtitle: &str) {
    if !spawn_safe(&player.name) { return; }
    let name = &player.name;
    host.run_command(&format!(r#"title {name} subtitle {{"translate":"{subtitle}"}}"#));
    host.run_command(&format!(r#"title {name} title {{"translate":"{title}","color":"gold"}}"#));
}

/// A noite, antes de qualquer outra parte do tutorial. Devolve `true` quando ela tomou a vez
/// (a primeira noite em curso): o resto do tutorial espera o amanhecer.
pub(super) fn overlay(host: &mut dyn Host, player: &NativeServerPlayer, p: &mut Progress) -> bool {
    if player.dimension != workforce::spawn::OVERWORLD { return false; }
    let dark = workforce::night(host);
    let dusk = dark && !p.was_night;
    p.was_night = dark;
    match p.night_first {
        0 if dark => {
            p.night_first = 1;
            p.dirty = true;
            title(host, player, "veio.night.first_title", "veio.night.first_sub");
            sound(host, player, "minecraft:block.bell.use");
            for line in 1..=3 { say(host, p, player.uuid, &format!("veio.night.first.{line}")); }
        }
        1 if !dark => {
            p.night_first = FIRST_DONE;
            p.dirty = true;
            title(host, player, "veio.night.dawn_title", "veio.night.dawn_sub");
            sound(host, player, "minecraft:ui.toast.challenge_complete");
            if spawn_safe(&player.name) { host.run_command(&format!("xp add {} 15 points", player.name)); }
            say(host, p, player.uuid, "veio.night.dawn");
            return false;
        }
        FIRST_DONE if dusk && !p.hidden => {
            // Todo anoitecer: o sino da vila e uma frase curta, só para quem está fora de casa.
            if !home::in_house(&player.dimension, player.position) {
                sound(host, player, "minecraft:block.bell.use");
                let line = host.day_time().div_euclid(24_000).rem_euclid(DUSK_LINES as i64) + 1;
                host.tell(player.uuid, Text::tr(&format!("veio.night.dusk.{line}"), &[]));
            }
            return false;
        }
        _ => {}
    }
    if p.night_first != 1 || p.hidden { return p.night_first == 1; }
    // Primeira noite: a barra leva para casa e, dentro dela, pede para dormir.
    let bar = if home::in_house(&player.dimension, player.position) {
        Text::tr("veio.night.goal.inside", &[])
    } else if let Some([x, y, z]) = home::dashboard() {
        let (distance, way) = intro::pointer(player, [f64::from(x) + 0.5, f64::from(y), f64::from(z) + 0.5]);
        Text::tr("veio.night.goal.home", &[&distance, &way])
    } else {
        Text::tr("veio.night.goal.shelter", &[])
    };
    host.action_bar(player.uuid, bar);
    true
}

fn announce(host: &mut dyn Host, player: &NativeServerPlayer, p: &Progress, step: usize) {
    host.tell(player.uuid, Text::tr(&format!("veio.night.title.{step}"), &[]));
    say(host, p, player.uuid, &format!("veio.night.why.{step}"));
    for line in 1..=DO_LINES[step] {
        host.tell(player.uuid, Text::tr(&format!("veio.night.do.{step}.{line}"), &[]));
    }
    host.tell(player.uuid, Text::tr(&format!("veio.night.more.{step}"), &[]));
}

/// Um passo do capítulo, a cada segundo.
pub(super) fn tick(host: &mut dyn Host, player: &NativeServerPlayer, p: &mut Progress) {
    let Some(current) = step(p) else { return };
    if ready(current, p) {
        say(host, p, player.uuid, &format!("veio.night.done.{current}"));
        sound(host, player, "minecraft:ui.toast.challenge_complete");
        if spawn_safe(&player.name) { host.run_command(&format!("xp add {} {} points", player.name, 25 + 5 * current)); }
        p.night += 1;
        p.dirty = true;
        if p.night == DONE_NIGHT {
            title(host, player, "veio.night.end_title", "veio.night.end_sub");
            for line in 1..=3 { say(host, p, player.uuid, &format!("veio.night.end.{line}")); }
            host.action_bar(player.uuid, Text::literal(""));
            return;
        }
        title(host, player, "veio.onboarding.done", &format!("veio.night.title.{}", current + 1));
    }
    if p.hidden { return; }
    let Some(current) = step(p) else { return };
    if p.night_announced != Some(p.night) {
        announce(host, player, p, current);
        p.night_announced = Some(p.night);
    }
    let gate = |bit: u8| Text::tr(if p.night_flags & bit != 0 { "veio.night.gate_yes" } else { "veio.night.gate_no" }, &[]).plain();
    let goal = match current {
        0 => Text::tr("veio.night.goal.0", &[&p.night_lights]),
        1 => Text::tr("veio.night.goal.1", &[&p.night_wall, &gate(GATE)]),
        2 => Text::tr(if workforce::night(host) { "veio.night.goal.2_night" } else { "veio.night.goal.2_day" }, &[]),
        3 => {
            // O medidor ao vivo: a poluição do chunk da oficina contra o aviso e as ondas.
            let level = factory(p).map_or(0, |at| pollution::local(&player.dimension, at).round() as i32);
            Text::tr("veio.night.goal.3", &[&level, &(pollution::NOTICE as i32), &(pollution::RAID as i32)])
        }
        4 => Text::tr("veio.night.goal.4", &[&p.night_factory_wall, &gate(FACTORY_GATE)]),
        _ => Text::tr("veio.night.goal.5", &[]),
    };
    host.action_bar(player.uuid, Text::tr("veio.night.bar", &[&(current + 1), &goal.plain()]));
}

/// Clique no prefeito: começa o capítulo ou repete a etapa.
pub(super) fn talk(host: &mut dyn Host, player: &NativeServerPlayer, p: &mut Progress) {
    match step(p) {
        None => start(host, player, p),
        // O conselho de defesa acontece na conversa com o prefeito.
        Some(5) => council(host, player, p),
        Some(current) => {
            village::say(host, player.uuid, "veio.village.again");
            announce(host, player, p, current);
        }
    }
}

/// Começa o capítulo (prefeito ou `/veiotutorial noite`).
pub(super) fn start(host: &mut dyn Host, player: &NativeServerPlayer, p: &mut Progress) {
    title(host, player, "veio.night.start_title", "veio.night.start_sub");
    for line in 1..=2 { village::say(host, player.uuid, &format!("veio.night.intro.{line}")); }
    p.night = 1;
    p.night_announced = None;
    p.night_lights = 0;
    p.night_wall = 0;
    p.night_factory_wall = 0;
    p.night_flags = 0;
    p.hidden = false;
    p.dirty = true;
}
