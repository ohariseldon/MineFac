//! Prólogo do tutorial: chegar à vila, entrar em casa, falar com o prefeito e nivelar o terreno
//! da oficina. Só depois disso começam as 8 etapas da fábrica (`onboarding.rs`); no fim delas o
//! jogador volta ao prefeito, que fecha o tutorial e aponta o que vem depois.
//!
//! - **Casa:** a casa do fundador (`crate::home`) fica ao norte do centro da vila. A barra de ação
//!   diz a distância e a direção; ao entrar, o jogo apresenta a cama, o painel e o Lar.
//! - **Prefeito:** um aldeão parado e com nome (`veio:mayor`) na varanda da casa. Clicar nele
//!   (`talk`) dá a missão do momento; é o "o que eu faço agora?" do jogo inteiro.
//! - **Terreno da oficina:** o prefeito escolhe um quadrado de `PLOT` x `PLOT` perto da vila, o
//!   mais plano possível e sem água, e marca a borda com faíscas verdes. O jogador cava o que
//!   sobra (chamas pequenas) e aterra os buracos (faíscas azuis) até o nível do meio. A conta
//!   vem dos mapas de altura do servidor, a cada 2 s, e só enquanto a etapa está aberta.
use super::*;
use workforce::spawn::OVERWORLD;
use horizon_client::mod_api::{NativeHeightmap, ParticleOptions, ParticleType};

/// Tipo do prefeito (`tools/build_content.py`: aldeão parado, que não entra no censo da vila).
pub(crate) const MAYOR: &str = "veio:mayor";
pub(super) const HOME_STAGE: u8 = 0;
pub(super) const MAYOR_STAGE: u8 = 1;
pub(super) const PLOT_STAGE: u8 = 2;
pub(super) const INTRO_DONE: u8 = 3;
/// Lado do terreno da oficina, em blocos.
pub(super) const PLOT: i32 = 11;
const HALF: i32 = PLOT / 2;
/// Colunas que podem ficar fora do nível (um tronco, uma flor alta): o terreno conta como pronto.
pub(super) const SLACK: u32 = 4;
/// Sem casa depois disto (3 min), o prefeito aparece ao lado do jogador.
const HOUSE_WAIT: u64 = 3600;
/// O prefeito fica na varanda: em relação ao painel da casa, dois blocos a oeste da porta.
const PORCH: [i32; 3] = [-2, 0, 5];
/// Distâncias e direções em que o prefeito procura o terreno, a partir do centro da vila.
const RINGS: [i32; 3] = [26, 34, 42];
const DIRECTIONS: [(i32, i32); 8] = [(0, 1), (1, 1), (-1, 1), (1, 0), (-1, 0), (1, -1), (-1, -1), (0, -1)];
/// Perto o bastante para a barra trocar a direção pelo que falta fazer.
const NEAR: f64 = 9.0;
/// Até onde o jogador vê as marcas do terreno.
const MARKS_RANGE: f64 = 48.0;

/// Prefeito nesta sessão, e onde ele fica quando não há casa.
static MAYOR_ID: Mutex<Option<u32>> = Mutex::new(None);
static FALLBACK_SPOT: Mutex<Option<[f64; 3]>> = Mutex::new(None);

/// Situação do terreno: colunas fora do nível, a cavar e a aterrar.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) struct Level { pub left: u32, pub dig: u32, pub fill: u32 }

/// Onde o prefeito fica: na varanda da casa ou, sem casa, ao lado de onde o jogador estava.
fn mayor_spot() -> Option<[f64; 3]> {
    if let Some([x, y, z]) = home::dashboard() {
        return Some([f64::from(x + PORCH[0]) + 0.5, f64::from(y + PORCH[1]), f64::from(z + PORCH[2]) + 0.5]);
    }
    FALLBACK_SPOT.lock().ok().and_then(|spot| *spot)
}

/// "23 blocos ao norte": distância e um dos 8 rumos (norte é -z).
pub(super) fn heading(from: [f64; 3], to: [f64; 3]) -> (i32, &'static str) {
    let (dx, dz) = (to[0] - from[0], to[2] - from[2]);
    let distance = dx.hypot(dz).round() as i32;
    let angle = dx.atan2(-dz).to_degrees().rem_euclid(360.0);
    let index = ((angle + 22.5) / 45.0) as usize % 8;
    (distance, ["n", "ne", "e", "se", "s", "sw", "w", "nw"][index])
}

pub(super) fn direction(from: [f64; 3], to: [f64; 3]) -> (i32, String) {
    let (distance, key) = heading(from, to);
    (distance, Text::tr(&format!("veio.onboarding.dir.{key}"), &[]).plain())
}

fn plot_center(plot: Pos) -> [f64; 3] {
    [f64::from(plot[0]) + 0.5, f64::from(plot[1]), f64::from(plot[2]) + 0.5]
}

fn columns(plot: Pos) -> Vec<(i32, i32)> {
    (-HALF..=HALF).flat_map(|dx| (-HALF..=HALF).map(move |dz| (plot[0] + dx, plot[2] + dz))).collect()
}

/// Conta as colunas fora do nível: chão (`ground`, sem folhas), fundo (`floor`, sem água) e topo
/// (`top`, com folhas). Coluna certa: os três no nível do terreno. Acima: cavar; abaixo ou água:
/// aterrar.
pub(super) fn level(target: i32, ground: &[Option<i32>], floor: &[Option<i32>], top: &[Option<i32>]) -> Option<Level> {
    let mut out = Level::default();
    for ((ground, floor), top) in ground.iter().zip(floor).zip(top) {
        let (ground, floor, top) = ((*ground)?, (*floor)?, (*top)?);
        if ground == target && floor == target && top == target { continue; }
        out.left += 1;
        if ground > target || top > ground { out.dig += 1; } else { out.fill += 1; }
    }
    Some(out)
}

fn survey(host: &dyn Host, plot: Pos) -> Option<(Level, Vec<((i32, i32), i32)>)> {
    let cols = columns(plot);
    let ground = host.surface_heights(OVERWORLD, &cols, NativeHeightmap::MotionBlockingNoLeaves).ok()?;
    let floor = host.surface_heights(OVERWORLD, &cols, NativeHeightmap::OceanFloor).ok()?;
    let top = host.surface_heights(OVERWORLD, &cols, NativeHeightmap::MotionBlocking).ok()?;
    let state = level(plot[1], &ground, &floor, &top)?;
    let wrong = cols.iter().zip(ground.iter().zip(&floor).zip(&top))
        .filter_map(|(col, ((g, f), t))| {
            let (g, f, t) = ((*g)?, (*f)?, (*t)?);
            (!(g == plot[1] && f == plot[1] && t == plot[1])).then_some((*col, g.max(t)))
        }).collect();
    Some((state, wrong))
}

/// Nota de um quadrado candidato (menor é melhor), pelas amostras de altura. Água recusa.
pub(super) fn plot_score(ground: &[i32], floor: &[i32], top: &[i32]) -> Option<i32> {
    if ground.is_empty() || ground.iter().zip(floor).any(|(g, f)| f < g) { return None; }
    let leaves = ground.iter().zip(top).filter(|(g, t)| t > g).count() as i32;
    let range = ground.iter().max()? - ground.iter().min()?;
    Some(range * 4 + leaves * 3)
}

/// O quadrado ocupa a casa (com 3 blocos de folga)?
fn overlaps_house(center: [i32; 2]) -> bool {
    let Some([hx, _, hz]) = home::dashboard() else { return false };
    let (west, north) = (hx - home::DASHBOARD[0] - 3, hz - home::DASHBOARD[2] - 3);
    let (east, south) = (west + home::FOOTPRINT[0] + 6, north + home::FOOTPRINT[1] + 6);
    center[0] + HALF >= west && center[0] - HALF <= east && center[1] + HALF >= north && center[1] - HALF <= south
}

/// Escolhe o terreno da oficina em volta da vila: o quadrado mais plano, sem água, fora da casa.
pub(super) fn choose_plot(host: &dyn Host, around: [i32; 3]) -> Pos {
    let mut best: Option<(i32, [i32; 2])> = None;
    for ring in RINGS {
        for (dx, dz) in DIRECTIONS {
            let center = [around[0] + dx * ring, around[2] + dz * ring];
            if overlaps_house(center) { continue; }
            let samples: Vec<(i32, i32)> = (-HALF..=HALF).step_by(2)
                .flat_map(|sx| (-HALF..=HALF).step_by(2).map(move |sz| (center[0] + sx, center[1] + sz))).collect();
            let read = |map| host.surface_heights(OVERWORLD, &samples, map).ok()
                .and_then(|heights| heights.into_iter().collect::<Option<Vec<i32>>>());
            let (Some(ground), Some(floor), Some(top)) = (read(NativeHeightmap::MotionBlockingNoLeaves),
                read(NativeHeightmap::OceanFloor), read(NativeHeightmap::MotionBlocking)) else { continue };
            let Some(score) = plot_score(&ground, &floor, &top) else { continue };
            if best.is_none_or(|(known, _)| score < known) { best = Some((score, center)); }
        }
    }
    let [x, z] = best.map_or([around[0], around[2] + RINGS[0]], |(_, at)| at);
    // Nível: a altura do meio das colunas do quadrado inteiro (o mínimo de terra a mexer).
    let mut heights: Vec<i32> = host.surface_heights(OVERWORLD, &columns([x, 0, z]), NativeHeightmap::MotionBlockingNoLeaves)
        .ok().map(|list| list.into_iter().flatten().collect()).unwrap_or_default();
    heights.sort_unstable();
    let y = heights.get(heights.len() / 2).copied().unwrap_or(around[1]);
    [x, y, z]
}

/// O prefeito está no mundo? Se não, chega à varanda (ou ao lado do jogador, sem casa).
fn ensure_mayor(host: &mut dyn Host, player: &NativeServerPlayer) {
    if player.dimension != OVERWORLD { return; }
    let known = MAYOR_ID.lock().ok().and_then(|id| *id);
    if known.is_some_and(|id| host.entity(id).is_some()) { return; }
    if host.run_command(&format!("execute if entity @e[type={MAYOR}]")) { return; }
    let spot = mayor_spot().unwrap_or_else(|| {
        let [x, y, z] = player.position;
        let at = [x.floor() + 2.5, y.floor(), z.floor() + 0.5];
        if let Ok(mut spot) = FALLBACK_SPOT.lock() { *spot = Some(at); }
        at
    });
    let name = Text::tr("veio.onboarding.mayor.name", &[]).plain();
    let nbt = Compound::from([
        ("CustomName".into(), Tag::String(name.as_str().into())),
        ("CustomNameVisible".into(), Tag::Byte(1)),
        ("NoAI".into(), Tag::Byte(1)),
        ("Invulnerable".into(), Tag::Byte(1)),
        ("PersistenceRequired".into(), Tag::Byte(1)),
        ("VillagerData".into(), Tag::Compound(Compound::from([
            ("profession".into(), Tag::String("minecraft:cartographer".into())),
            ("type".into(), Tag::String("minecraft:plains".into())),
            ("level".into(), Tag::Int(5)),
        ]))),
    ]);
    if let Some(id) = host.summon(OVERWORLD, MAYOR, spot, nbt) {
        if let Ok(mut known) = MAYOR_ID.lock() { *known = Some(id); }
        host.send_particles(&ParticleOptions::simple(ParticleType::HappyVillager), None,
            [spot[0], spot[1] + 1.0, spot[2]], [0.3, 0.5, 0.3], 0.0, 10);
    }
}

fn title(host: &mut dyn Host, player: &NativeServerPlayer, title: &str, subtitle: &str) {
    if !spawn_safe(&player.name) { return; }
    let name = &player.name;
    host.run_command(&format!(r#"title {name} subtitle {{"translate":"{subtitle}"}}"#));
    host.run_command(&format!(r#"title {name} title {{"translate":"{title}","color":"gold"}}"#));
}

/// Uma fala do prefeito no chat do jogador.
fn say(host: &mut dyn Host, player: u128, key: &str, args: &[&dyn std::fmt::Display]) {
    let name = Text::tr("veio.onboarding.mayor.name", &[]).plain();
    host.tell(player, Text::tr("veio.agenda.says", &[&name, &Text::tr(key, args).plain()]));
}

fn announce(host: &mut dyn Host, player: &NativeServerPlayer, p: &Progress) {
    let uuid = player.uuid;
    match p.intro {
        HOME_STAGE => {
            title(host, player, "veio.onboarding.intro.welcome", "veio.onboarding.intro.welcome_sub");
            for line in 1..=3 { host.tell(uuid, Text::tr(&format!("veio.onboarding.intro.home.{line}"), &[])); }
        }
        MAYOR_STAGE => {
            for line in 1..=3 { host.tell(uuid, Text::tr(&format!("veio.onboarding.intro.house.{line}"), &[])); }
            host.tell(uuid, Text::tr("veio.onboarding.intro.mayor.1", &[]));
        }
        PLOT_STAGE => {
            if let Some([x, y, z]) = p.plot {
                title(host, player, "veio.onboarding.intro.plot_title", "veio.onboarding.intro.plot_sub");
                host.tell(uuid, Text::tr("veio.onboarding.intro.plot.1", &[&x, &y, &z, &PLOT]));
                host.tell(uuid, Text::tr("veio.onboarding.intro.plot.2", &[]));
                host.tell(uuid, Text::tr("veio.onboarding.intro.plot.3", &[]));
            }
        }
        _ => {}
    }
}

/// Um passo do prólogo, a cada segundo, para um jogador online.
pub(super) fn tick(host: &mut dyn Host, player: &NativeServerPlayer, p: &mut Progress, now: u64) {
    let since = *p.intro_since.get_or_insert(now);
    match p.intro {
        HOME_STAGE => {
            if home::in_house(&player.dimension, player.position)
                || (home::dashboard().is_none() && now >= since + HOUSE_WAIT) {
                p.intro = MAYOR_STAGE; p.dirty = true;
                if spawn_safe(&player.name) {
                    host.run_command(&format!("playsound minecraft:block.note_block.chime master {}", player.name));
                }
            }
        }
        MAYOR_STAGE => ensure_mayor(host, player),
        PLOT_STAGE => plot_tick(host, player, p, now),
        _ => {}
    }
    if p.hidden { return; }
    if p.intro_announced != Some(p.intro) {
        announce(host, player, p);
        p.intro_announced = Some(p.intro);
    }
    let bar = match p.intro {
        HOME_STAGE => match home::dashboard() {
            Some([x, y, z]) => {
                let (distance, dir) = direction(player.position, [f64::from(x), f64::from(y), f64::from(z)]);
                Text::tr("veio.onboarding.intro.goal.home", &[&distance, &dir])
            }
            None => Text::tr("veio.onboarding.intro.goal.home_wait", &[]),
        },
        MAYOR_STAGE => mayor_bar(player, "veio.onboarding.intro.goal.mayor"),
        PLOT_STAGE => plot_bar(player, p),
        _ => return,
    };
    host.action_bar(player.uuid, bar);
}

fn mayor_bar(player: &NativeServerPlayer, key: &str) -> Text {
    match mayor_spot() {
        Some(spot) if direction(player.position, spot).0 > 4 => {
            let (distance, dir) = direction(player.position, spot);
            Text::tr(key, &[&distance, &dir])
        }
        _ => Text::tr("veio.onboarding.intro.goal.mayor_here", &[]),
    }
}

fn plot_bar(player: &NativeServerPlayer, p: &Progress) -> Text {
    let Some(plot) = p.plot else { return Text::literal("") };
    let center = plot_center(plot);
    let (distance, dir) = direction(player.position, center);
    if f64::from(distance) > NEAR + f64::from(HALF) {
        return Text::tr("veio.onboarding.intro.goal.plot_far", &[&distance, &dir]);
    }
    match p.plot_level {
        Some(level) => Text::tr("veio.onboarding.intro.goal.plot",
            &[&level.left.saturating_sub(SLACK), &level.dig, &level.fill]),
        None => Text::tr("veio.onboarding.intro.goal.plot_far", &[&distance, &dir]),
    }
}

/// Mede o terreno, desenha as marcas para quem está perto e fecha o prólogo quando ele fica plano.
fn plot_tick(host: &mut dyn Host, player: &NativeServerPlayer, p: &mut Progress, now: u64) {
    let Some(plot) = p.plot else { p.intro = MAYOR_STAGE; return };
    if player.dimension != OVERWORLD || now % 40 != 0 { return; }
    let center = plot_center(plot);
    let near = (player.position[0] - center[0]).hypot(player.position[2] - center[2]) <= MARKS_RANGE;
    if !near { return; }
    let Some((state, wrong)) = survey(host, plot) else { return };
    p.plot_level = Some(state);
    marks(host, plot, &wrong);
    if state.left <= SLACK {
        p.intro = INTRO_DONE; p.dirty = true; p.plot_level = None;
        celebrate_plot(host, player);
    }
}

/// Faíscas verdes na borda; chamas onde cavar e faíscas azuis onde aterrar.
fn marks(host: &mut dyn Host, plot: Pos, wrong: &[((i32, i32), i32)]) {
    let [x, y, z] = plot;
    let edge = |host: &mut dyn Host, at: [f64; 3]| {
        host.send_particles(&ParticleOptions::simple(ParticleType::HappyVillager), None, at, [0.0, 0.2, 0.0], 0.0, 1);
    };
    for step in (-HALF..=HALF).step_by(2) {
        for (cx, cz) in [(x + step, z - HALF), (x + step, z + HALF), (x - HALF, z + step), (x + HALF, z + step)] {
            edge(host, [f64::from(cx) + 0.5, f64::from(y) + 0.3, f64::from(cz) + 0.5]);
        }
    }
    for ((cx, cz), height) in wrong.iter().take(60) {
        let (kind, at_y) = if *height > y { (ParticleType::Flame, *height) } else { (ParticleType::ElectricSpark, y) };
        host.send_particles(&ParticleOptions::simple(kind), None,
            [f64::from(*cx) + 0.5, f64::from(at_y) + 0.2, f64::from(*cz) + 0.5], [0.15, 0.1, 0.15], 0.0, 2);
    }
}

fn celebrate_plot(host: &mut dyn Host, player: &NativeServerPlayer) {
    title(host, player, "veio.onboarding.intro.plot_done", "veio.onboarding.title.0");
    if spawn_safe(&player.name) {
        host.run_command(&format!("playsound minecraft:ui.toast.challenge_complete master {}", player.name));
        host.run_command(&format!("xp add {} 20 points", player.name));
    }
    say(host, player.uuid, "veio.onboarding.mayor.plot_done.1", &[]);
    say(host, player.uuid, "veio.onboarding.mayor.plot_done.2", &[]);
}

/// Barra depois da última etapa da fábrica: voltar ao prefeito.
pub(super) fn outro_bar(host: &mut dyn Host, player: &NativeServerPlayer) {
    host.action_bar(player.uuid, mayor_bar(player, "veio.onboarding.outro.goal"));
}

/// O jogador clicou no prefeito: a fala depende de onde ele está no tutorial.
pub(super) fn talk(host: &mut dyn Host, player: &NativeServerPlayer, p: &mut Progress) {
    let uuid = player.uuid;
    if spawn_safe(&player.name) {
        host.run_command(&format!("playsound minecraft:entity.villager.ambient master {}", player.name));
    }
    match p.intro {
        HOME_STAGE | MAYOR_STAGE => {
            for line in 1..=4 { say(host, uuid, &format!("veio.onboarding.mayor.hello.{line}"), &[]); }
            let around = workforce::spawn::center().map(|(at, _)| at)
                .or_else(|| home::dashboard())
                .unwrap_or([player.position[0].floor() as i32, player.position[1].floor() as i32, player.position[2].floor() as i32]);
            let plot = choose_plot(host, around);
            let (distance, dir) = direction(player.position, plot_center(plot));
            say(host, uuid, "veio.onboarding.mayor.plot", &[&distance, &dir, &PLOT, &PLOT]);
            p.plot = Some(plot); p.intro = PLOT_STAGE; p.dirty = true; p.intro_announced = None;
        }
        PLOT_STAGE => {
            let left = p.plot_level.map_or(PLOT as u32 * PLOT as u32, |level| level.left.saturating_sub(SLACK));
            if let Some(plot) = p.plot {
                let (distance, dir) = direction(player.position, plot_center(plot));
                say(host, uuid, "veio.onboarding.mayor.plot_again", &[&distance, &dir, &left]);
            }
        }
        _ if p.step < DONE => {
            say(host, uuid, "veio.onboarding.mayor.now", &[&Text::tr(&format!("veio.onboarding.title.{}", p.step), &[]).plain()]);
            host.tell(uuid, Text::tr(&format!("veio.onboarding.why.{}", p.step), &[]));
            for line in 1..=DO_LINES.get(usize::from(p.step)).copied().unwrap_or(0) {
                host.tell(uuid, Text::tr(&format!("veio.onboarding.do.{}.{line}", p.step), &[]));
            }
        }
        // Depois da fábrica, o prefeito conduz o capítulo da vila.
        _ if p.outro => super::village::talk(host, player, p),
        _ => {
            for line in 1..=5 { say(host, uuid, &format!("veio.onboarding.mayor.next.{line}"), &[]); }
            say(host, uuid, "veio.village.offer", &[]);
            if !p.outro {
                p.outro = true; p.dirty = true;
                title(host, player, "veio.onboarding.outro.title", "veio.onboarding.outro.sub");
                if spawn_safe(&player.name) {
                    host.run_command(&format!("playsound minecraft:ui.toast.challenge_complete master {}", player.name));
                    host.run_command(&format!("xp add {} 50 points", player.name));
                }
                host.action_bar(uuid, Text::literal(""));
            }
        }
    }
}

/// Outro mundo abriu: os ids e o lugar do prefeito eram do anterior.
pub(super) fn reset() {
    if let Ok(mut id) = MAYOR_ID.lock() { *id = None; }
    if let Ok(mut spot) = FALLBACK_SPOT.lock() { *spot = None; }
}
