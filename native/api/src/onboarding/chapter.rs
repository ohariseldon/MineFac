//! O que os capítulos do prefeito têm em comum (a partir do 5): anunciar a etapa, a ficha da
//! receita, a lista de materiais, a barra (texto ou lista ✔/✘), a comemoração, a abertura e o fim.
//!
//! Cada capítulo descreve só o que é dele (`Spec`) e decide quando a etapa fecha. As chaves de
//! texto seguem `veio.<key>.<parte>.<etapa>` (título, por quê, ficha, de onde vem, passos, saiba
//! mais, comentário ao concluir), mais `intro.<n>`, `end.<n>`, `start_title`, `start_sub`,
//! `end_title`, `end_sub` e `bar` ("Capítulo %s/%s | %s").
use super::*;

pub(super) struct Spec {
    pub key: &'static str,
    pub steps: u8,
    pub do_lines: &'static [u8],
    /// Bloco que a etapa pede fabricar e os ingredientes (vazio: a etapa não pede bloco novo).
    pub parts: &'static [(&'static str, &'static [&'static str])],
    pub intro: u8,
    pub end: u8,
    /// Experiência da primeira etapa; cada etapa seguinte dá 5 a mais.
    pub xp: u32,
    /// Quem da vila comenta cada etapa (chave do nome; vazio: só o prefeito). A fala é
    /// `veio.<key>.voice.<etapa>`; a do prefeito, `veio.<key>.story.<etapa>`.
    pub voices: &'static [&'static str],
}

impl Spec {
    pub(super) fn done(&self) -> u8 { self.steps + 1 }
}

/// A etapa (0..steps) em andamento, se o capítulo está aberto (`value` 1..=steps).
pub(super) fn step(spec: &Spec, value: u8) -> Option<usize> {
    (1..=spec.steps).contains(&value).then(|| usize::from(value - 1))
}

fn title(host: &mut dyn Host, player: &NativeServerPlayer, title: &str, subtitle: &str) {
    if !spawn_safe(&player.name) { return; }
    let name = &player.name;
    host.run_command(&format!(r#"title {name} subtitle {{"translate":"{subtitle}"}}"#));
    host.run_command(&format!(r#"title {name} title {{"translate":"{title}","color":"gold"}}"#));
}

fn sound(host: &mut dyn Host, player: &NativeServerPlayer, sound: &str) {
    if spawn_safe(&player.name) {
        host.run_command(&format!("execute as {0} at @s run playsound {sound} master @s", player.name));
    }
}

/// Anuncia a etapa: título, o prefeito conta por quê, a ficha (com a receita no livro), os passos
/// e o "saiba mais".
pub(super) fn announce(host: &mut dyn Host, player: &NativeServerPlayer, spec: &Spec, step: usize) {
    let (uuid, key) = (player.uuid, spec.key);
    host.tell(uuid, Text::tr(&format!("veio.{key}.title.{step}"), &[]));
    story(host, uuid, spec, step);
    village::say(host, uuid, &format!("veio.{key}.why.{step}"));
    let (block, ingredients) = spec.parts[step];
    if !ingredients.is_empty() {
        host.tell(uuid, Text::tr(&format!("veio.{key}.recipe.{step}"), &[]));
        host.tell(uuid, Text::tr(&format!("veio.{key}.from.{step}"), &[]));
        if spawn_safe(&player.name) { host.run_command(&format!("recipe give {} {block}", player.name)); }
        host.tell(uuid, Text::tr("veio.onboarding.book", &[]));
    }
    for line in 1..=spec.do_lines[step] {
        host.tell(uuid, Text::tr(&format!("veio.{key}.do.{step}.{line}"), &[]));
    }
    host.tell(uuid, Text::tr(&format!("veio.{key}.more.{step}"), &[]));
}

/// A história da etapa: o prefeito liga a etapa ao que a vila vive, e um morador comenta.
pub(super) fn story(host: &mut dyn Host, uuid: u128, spec: &Spec, step: usize) {
    let key = spec.key;
    village::say(host, uuid, &format!("veio.{key}.story.{step}"));
    if let Some(who) = spec.voices.get(step).copied().filter(|who| !who.is_empty()) {
        let name = Text::tr(who, &[]).plain();
        host.tell(uuid, Text::tr("veio.agenda.says", &[&name, &Text::tr(&format!("veio.{key}.voice.{step}"), &[]).plain()]));
    }
}

/// A lista de materiais do bloco da etapa, enquanto ele não foi fabricado nem está no inventário.
/// Devolve `true` se ocupou a barra.
pub(super) fn materials(host: &mut dyn Host, player: &NativeServerPlayer, spec: &Spec, step: usize, crafted: &mut u16) -> bool {
    let (block, ingredients) = spec.parts[step];
    let bit = 1u16 << step;
    if ingredients.is_empty() || *crafted & bit != 0 { return false; }
    if has_item(host, &player.name, block) { *crafted |= bit; return false; }
    village::materials(host, player, block, ingredients)
}

/// A barra com um texto: "Capítulo n/total | texto".
pub(super) fn bar(host: &mut dyn Host, player: &NativeServerPlayer, spec: &Spec, step: usize, text: &str) {
    host.action_bar(player.uuid, Text::tr(&format!("veio.{}.bar", spec.key), &[&(step + 1), &text.to_owned()]));
}

fn json(text: &str) -> String { text.replace('\\', "\\\\").replace('"', "\\\"") }

/// A barra com a lista ✔/✘ das condições da etapa: (cumprida, chave, argumentos).
pub(super) fn checklist(host: &mut dyn Host, player: &NativeServerPlayer, spec: &Spec, step: usize,
    checks: &[(bool, &str, Vec<String>)])
{
    if !spawn_safe(&player.name) { return; }
    let mut parts = vec![format!(r#"{{"translate":"veio.{}.bar_prefix","with":["{}"],"color":"gold"}}"#, spec.key, step + 1)];
    for (ok, key, args) in checks {
        let (mark, color) = if *ok { ("✔", "green") } else { ("✘", "red") };
        let with = args.iter().map(|arg| format!(r#""{}""#, json(arg))).collect::<Vec<_>>().join(",");
        parts.push(format!(r#"{{"text":"  {mark} ","color":"{color}"}},{{"translate":"{key}","with":[{with}],"color":"{color}"}}"#));
    }
    host.run_command(&format!(r#"title {} actionbar ["",{}]"#, player.name, parts.join(",")));
}

/// Etapa concluída: o prefeito comenta, título com a próxima, som e experiência.
pub(super) fn celebrate(host: &mut dyn Host, player: &NativeServerPlayer, spec: &Spec, finished: usize) {
    village::say(host, player.uuid, &format!("veio.{}.done.{finished}", spec.key));
    let next = if finished + 1 < usize::from(spec.steps) { format!("veio.{}.title.{}", spec.key, finished + 1) }
        else { format!("veio.{}.end_sub", spec.key) };
    title(host, player, "veio.onboarding.done", &next);
    sound(host, player, "minecraft:ui.toast.challenge_complete");
    intro::react(host, intro::Mood::Happy);
    if spawn_safe(&player.name) {
        host.run_command(&format!("xp add {} {} points", player.name, spec.xp + 5 * finished as u32));
    }
}

/// Abre o capítulo: falas de abertura e título.
pub(super) fn open(host: &mut dyn Host, player: &NativeServerPlayer, spec: &Spec) {
    for line in 1..=spec.intro { village::say(host, player.uuid, &format!("veio.{}.intro.{line}", spec.key)); }
    title(host, player, &format!("veio.{}.start_title", spec.key), &format!("veio.{}.start_sub", spec.key));
}

/// Fecha o capítulo: falas finais, título e a experiência do fim.
pub(super) fn finish(host: &mut dyn Host, player: &NativeServerPlayer, spec: &Spec) {
    for line in 1..=spec.end { village::say(host, player.uuid, &format!("veio.{}.end.{line}", spec.key)); }
    title(host, player, &format!("veio.{}.end_title", spec.key), &format!("veio.{}.end_sub", spec.key));
    if spawn_safe(&player.name) { host.run_command(&format!("xp add {} {} points", player.name, spec.xp * 4)); }
    // Fim de capítulo é festa na praça: fogos em volta do jogador e o prefeito comemorando.
    fireworks(host, &player.name);
    intro::react(host, intro::Mood::Happy);
    host.action_bar(player.uuid, Text::literal(""));
}
