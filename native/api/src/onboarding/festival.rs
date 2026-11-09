//! Festas do capítulo da vila, como os eventos do calendário do Stardew Valley: o prefeito marca
//! uma data, a barra conta os dias, e no dia (ao nascer do sol) a festa acontece com o que a vila
//! tiver pronto. Dá urgência sem castigo: festa fraca é remarcada.
//!
//! 1. **Festa da Colheita**, 3 dias depois de abrir o capítulo: pede as 4 primeiras etapas
//!    (posto, contratar, marco e a primeira colheita). Recompensa: pão para os postos.
//! 2. **Visita do Governador**, 4 dias depois: pede o capítulo inteiro (carregador, capataz, casa
//!    e prefeitura). Recompensa: um bolo, que no Lar vira banquete.
use super::*;

pub(super) const HARVEST: u8 = 1;
pub(super) const GOVERNOR: u8 = 2;
pub(super) const DONE: u8 = 3;
/// Um dia do jogo, em ticks.
const DAY: i64 = 24_000;
/// Dias até cada festa e, se ela sair fraca, até a nova data.
const HARVEST_DAYS: u32 = 3;
const GOVERNOR_DAYS: u32 = 4;
const RETRY_DAYS: u32 = 2;

/// Até onde o capítulo precisa ter chegado (`Progress::village`) para a festa sair bonita.
pub(super) fn needs(festival: u8) -> u8 {
    if festival == HARVEST { 5 } else { village::DONE_VILLAGE }
}

pub(super) fn today(host: &dyn Host) -> u32 { host.day_time().div_euclid(DAY).clamp(0, i64::from(u32::MAX)) as u32 }

/// Marca a festa e o prefeito anuncia.
fn schedule(host: &mut dyn Host, player: &NativeServerPlayer, p: &mut Progress, festival: u8, days: u32, retry: bool) {
    p.festival = festival;
    p.festival_day = today(host).saturating_add(days);
    p.dirty = true;
    let key = key(festival);
    if retry {
        intro::say(host, player.uuid, &format!("veio.festival.{key}.retry"), &[&days]);
        return;
    }
    for line in 1..=2 { intro::say(host, player.uuid, &format!("veio.festival.{key}.announce.{line}"), &[&days]); }
    if workforce::spawn::safe_name(&player.name) {
        let name = &player.name;
        host.run_command(&format!(r#"title {name} subtitle {{"translate":"veio.festival.{key}.when","with":["{days}"]}}"#));
        host.run_command(&format!(r#"title {name} title {{"translate":"veio.festival.{key}.name","color":"gold"}}"#));
        host.run_command(&format!("execute as {name} at @s run playsound minecraft:block.bell.use master @s"));
    }
}

fn key(festival: u8) -> &'static str { if festival == HARVEST { "harvest" } else { "governor" } }

/// A cada segundo, com o capítulo da vila aberto (ou acabado, esperando a visita).
pub(super) fn tick(host: &mut dyn Host, player: &NativeServerPlayer, p: &mut Progress) {
    match p.festival {
        0 => schedule(host, player, p, HARVEST, HARVEST_DAYS, false),
        HARVEST | GOVERNOR if today(host) >= p.festival_day => {
            let festival = p.festival;
            let key = key(festival);
            let name = player.name.clone();
            let safe = workforce::spawn::safe_name(&name);
            if p.village >= needs(festival) {
                for line in 1..=2 { intro::say(host, player.uuid, &format!("veio.festival.{key}.good.{line}"), &[]); }
                intro::react(host, intro::Mood::Happy);
                if safe {
                    host.run_command(&format!(r#"title {name} subtitle {{"translate":"veio.festival.{key}.good_sub"}}"#));
                    host.run_command(&format!(r#"title {name} title {{"translate":"veio.festival.{key}.name","color":"gold"}}"#));
                    host.run_command(&format!("execute as {name} at @s run playsound minecraft:entity.player.levelup master @s"));
                    let (item, count, xp) = if festival == HARVEST { ("minecraft:bread", 16, 60) } else { ("minecraft:cake", 1, 120) };
                    host.run_command(&format!("give {name} {item} {count}"));
                    host.run_command(&format!("xp add {name} {xp} points"));
                }
                fireworks(host, &name);
                if festival == HARVEST { schedule(host, player, p, GOVERNOR, GOVERNOR_DAYS, false); }
                else { p.festival = DONE; p.dirty = true; }
            } else {
                intro::say(host, player.uuid, &format!("veio.festival.{key}.weak"), &[]);
                intro::react(host, intro::Mood::Sad);
                schedule(host, player, p, festival, RETRY_DAYS, true);
            }
        }
        _ => {}
    }
}

/// O que a barra do capítulo soma: "Festa da Colheita em 2 dias", "é hoje!", ou nada.
pub(super) fn countdown(host: &dyn Host, p: &Progress) -> Option<Text> {
    if !matches!(p.festival, HARVEST | GOVERNOR) { return None; }
    let left = p.festival_day.saturating_sub(today(host));
    let name = Text::tr(&format!("veio.festival.{}.name", key(p.festival)), &[]).plain();
    Some(Text::tr("veio.festival.countdown", &[&name, &left]))
}

/// A festa marcada para hoje ou amanhã (os aldeões só falam dela).
pub(super) fn soon(host: &dyn Host, p: &Progress) -> bool {
    matches!(p.festival, HARVEST | GOVERNOR) && p.festival_day.saturating_sub(today(host)) <= 1
}
