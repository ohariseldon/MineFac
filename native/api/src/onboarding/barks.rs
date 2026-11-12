//! Os aldeões comuns falam pouco: um resmungo curto quando o jogador passa perto, sobre o que
//! está acontecendo com ele (procurar o prefeito, a fábrica, os postos, a festa que vem). Quem
//! conversa de verdade é o prefeito; os outros só comentam.
use super::*;
use std::sync::Mutex;

/// Perto disto (blocos) o aldeão comenta.
const NEAR: f64 = 6.0;
/// Um comentário por jogador a cada 40 s (relógio do tutorial) no máximo.
const GAP: u64 = 800;
/// Falas por assunto (`veio.villager.<assunto>.<n>`).
const LINES: u32 = 4;

static LAST: Mutex<Vec<(u128, u64)>> = Mutex::new(Vec::new());

/// O assunto do momento para o jogador.
pub(super) fn topic(host: &dyn Host, p: &Progress) -> &'static str {
    if p.intro < intro::GATHER_STAGE { "mayor" }
    else if p.intro < intro::PLOT_STAGE { "gather" }
    else if p.intro < intro::INTRO_DONE { "plot" }
    else if festival::soon(host, p) { "festival" }
    else if p.step == 1 && p.favor < favors::FAVOR_DONE { "favor" }
    else if p.step < DONE { "factory" }
    else if village::step(p).is_some() { "village" }
    // Fora dos capítulos: os pedidos da vila em aberto, ou a vila que voltou a respirar.
    else if p.errand_flags & errands::REVIVED != 0 { "revived" }
    else if (0..errands::COUNT).any(|index| errands::task(index, p.errands[index]).is_some()) { "errands" }
    else { "free" }
}

/// Chamado pelo tick dos aldeões (amostrado): comenta para o jogador perto, se faz tempo.
pub(crate) fn villager_seen(host: &mut dyn Host, dimension: &str, id: u32, now: u64) {
    let Some(at) = host.entity(id).and_then(|entity| entity.position(host)) else { return };
    let Some(player) = host.server_players().into_iter().find(|player| player.dimension == dimension
        && (0..3).map(|axis| (player.position[axis] - at[axis]).powi(2)).sum::<f64>() <= NEAR * NEAR) else { return };
    {
        let Ok(mut last) = LAST.lock() else { return };
        if last.iter().any(|(uuid, when)| *uuid == player.uuid && now.saturating_sub(*when) < GAP) { return; }
        last.retain(|(uuid, when)| *uuid != player.uuid && now.saturating_sub(*when) < GAP);
        if last.len() >= 64 { return; }
        last.push((player.uuid, now));
    }
    let topic = {
        let Ok(slot) = SESSION.lock() else { return };
        match slot.as_ref().and_then(|s| s.players.get(&player.uuid)) {
            Some(p) if p.hidden => return,
            Some(p) => topic(host, p),
            None => "free",
        }
    };
    let line = (now / GAP + u64::from(id)) % u64::from(LINES) + 1;
    let who = Text::tr("veio.villager.who", &[]).plain();
    host.tell(player.uuid, Text::tr("veio.villager.says", &[&who, &Text::tr(&format!("veio.villager.{topic}.{line}"), &[]).plain()]));
    host.play_sound("minecraft:entity.villager.ambient", at, 6, 0.6, 1.0);
}
