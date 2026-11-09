//! Prólogo do tutorial, a história do começo: o jogador nasce perto de uma vila, segue a seta até
//! ela, fala com o prefeito, junta os recursos que ele pede, recebe a primeira máquina e nivela o
//! terreno da oficina. Só então começam as 8 etapas da fábrica (`onboarding.rs`); no fim delas o
//! prefeito fecha o tutorial e passa a falar das fases do Quadro da fábrica (os gargalos).
//!
//! - **Seta:** a barra de ação aponta o alvo do momento (vila, prefeito, terreno). Num mundo de um
//!   jogador, a janela guarda para onde o jogador olha (`client_view`) e a seta gira com ele; sem
//!   isso (convidado da LAN), a barra diz o rumo ("ao norte").
//! - **Prefeito:** um aldeão parado e com nome (`veio:mayor`) na praça da vila. Clicar nele
//!   (`talk`) dá a fala do momento; é o "o que eu faço agora?" do jogo inteiro.
//! - **Recursos:** pedregulho e lingotes de ferro e de cobre (picareta, cavernas e fornalha). Com
//!   tudo no inventário, o clique entrega; o prefeito monta o gerador e libera as receitas.
//! - **Terreno da oficina:** quadrado de `PLOT` x `PLOT` perto da vila, o mais plano e sem água,
//!   com a borda marcada (faíscas verdes e contorno na tela); chamas onde cavar, faíscas azuis onde
//!   aterrar. A conta vem dos mapas de altura do servidor, a cada 2 s, só com o jogador perto.
use super::*;
use workforce::spawn::OVERWORLD;
use horizon_client::mod_api::{NativeBlockRim, NativeHeightmap, ParticleOptions, ParticleType};

/// Tipo do prefeito (`tools/build_content.py`: aldeão parado, que não entra no censo da vila).
pub(crate) const MAYOR: &str = "veio:mayor";
pub(super) const VILLAGE_STAGE: u8 = 0;
pub(super) const MAYOR_STAGE: u8 = 1;
pub(super) const GATHER_STAGE: u8 = 2;
pub(super) const PLOT_STAGE: u8 = 3;
pub(super) const INTRO_DONE: u8 = 4;
/// O que o prefeito pede para montar o gerador: (item, quantidade, chave do nome).
pub(super) const RESOURCES: [(&str, u32, &str); 3] = [
    ("minecraft:cobblestone", 16, "block.minecraft.cobblestone"),
    ("minecraft:iron_ingot", 2, "item.minecraft.iron_ingot"),
    ("minecraft:copper_ingot", 2, "item.minecraft.copper_ingot"),
];
/// Receitas que o prefeito libera ao entregar o gerador.
const UNLOCKS: [&str; 3] = [GENERATOR, PRESS, CABLE];
/// Lado do terreno da oficina, em blocos.
pub(super) const PLOT: i32 = 11;
const HALF: i32 = PLOT / 2;
/// Colunas que podem ficar fora do nível (um tronco, uma flor alta): o terreno conta como pronto.
pub(super) const SLACK: u32 = 4;
/// Chegou à vila: a até tantos blocos do centro.
const ARRIVED: f64 = 20.0;
/// Sem vila conhecida depois disto (3 min), o prefeito aparece ao lado do jogador.
const VILLAGE_WAIT: u64 = 3600;
/// Distâncias e direções em que o prefeito procura o terreno, a partir do centro da vila.
const RINGS: [i32; 3] = [26, 34, 42];
const DIRECTIONS: [(i32, i32); 8] = [(0, 1), (1, 1), (-1, 1), (1, 0), (-1, 0), (1, -1), (-1, -1), (0, -1)];
/// Perto o bastante para a barra trocar a seta pelo que falta fazer.
const NEAR: f64 = 9.0;
/// Até onde o jogador vê as marcas do terreno.
const MARKS_RANGE: f64 = 48.0;
/// A visão da janela vale por 2 s e só para quem está a até 2 blocos dela.
const VIEW_FRESH: u64 = 40;

/// Prefeito no mundo aberto: o id da entidade (visto no tick dela ou ao invocar).
static MAYOR_ID: Mutex<Option<u32>> = Mutex::new(None);
/// Onde o jogador da janela está e para onde olha (yaw do Minecraft), e quando foi lido.
static VIEW: Mutex<Option<([f64; 3], f32, u64)>> = Mutex::new(None);
/// Contornos para a janela desenhar (terreno ou prefeito) e quando foram feitos.
static MARKS: Mutex<(Vec<NativeBlockRim>, u64)> = Mutex::new((Vec::new(), 0));

/// Situação do terreno: colunas fora do nível, a cavar e a aterrar.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) struct Level { pub left: u32, pub dig: u32, pub fill: u32 }

fn clock() -> u64 { SERVER_CLOCK.load(Ordering::Relaxed) }

// ---------- Seta e marcas ----------

/// Janela: guarda onde o jogador está e para onde olha (só num mundo de um jogador o servidor vê).
pub(super) fn client_view(host: &dyn Host) {
    let Some(player) = host.player() else { return };
    if let Ok(mut view) = VIEW.lock() { *view = Some(([player.x, player.y, player.z], player.yaw, clock())); }
}

/// Para onde este jogador olha, se a janela é dele e a leitura é recente.
fn yaw_of(player: &NativeServerPlayer) -> Option<f32> {
    let (at, yaw, seen) = (*VIEW.lock().ok()?)?;
    let close = (0..3).all(|axis| (at[axis] - player.position[axis]).abs() < 2.0);
    (close && clock().saturating_sub(seen) <= VIEW_FRESH).then_some(yaw)
}

/// Seta para o alvo, vista por quem olha para `yaw` (0 = sul, 90 = oeste, como no Minecraft).
pub(super) fn arrow(from: [f64; 3], to: [f64; 3], yaw: f32) -> &'static str {
    let (dx, dz) = (to[0] - from[0], to[2] - from[2]);
    let bearing = (-dx).atan2(dz).to_degrees();
    let relative = (bearing - f64::from(yaw)).rem_euclid(360.0);
    ["↑", "↗", "→", "↘", "↓", "↙", "←", "↖"][((relative + 22.5) / 45.0) as usize % 8]
}

/// "23 blocos ao norte": distância e um dos 8 rumos (norte é -z).
pub(super) fn heading(from: [f64; 3], to: [f64; 3]) -> (i32, &'static str) {
    let (dx, dz) = (to[0] - from[0], to[2] - from[2]);
    let distance = dx.hypot(dz).round() as i32;
    let angle = dx.atan2(-dz).to_degrees().rem_euclid(360.0);
    let index = ((angle + 22.5) / 45.0) as usize % 8;
    (distance, ["n", "ne", "e", "se", "s", "sw", "w", "nw"][index])
}

/// Distância e seta (ou rumo por extenso, sem a visão da janela) até `to`.
pub(super) fn pointer(player: &NativeServerPlayer, to: [f64; 3]) -> (i32, String) {
    let (distance, key) = heading(player.position, to);
    let way = match yaw_of(player) {
        Some(yaw) => arrow(player.position, to, yaw).to_owned(),
        None => Text::tr(&format!("veio.onboarding.dir.{key}"), &[]).plain(),
    };
    (distance, way)
}

/// Janela: contornos do alvo do jogador (terreno ou prefeito), enquanto forem recentes.
pub(crate) fn visuals(rims: &mut Vec<NativeBlockRim>) {
    let Ok(marks) = MARKS.lock() else { return };
    if clock().saturating_sub(marks.1) <= 60 { rims.extend(marks.0.iter().copied()); }
}

fn publish_marks(player: &NativeServerPlayer, rims: Vec<NativeBlockRim>) {
    let mine = VIEW.lock().ok().and_then(|view| *view)
        .is_some_and(|(at, _, _)| (0..3).all(|axis| (at[axis] - player.position[axis]).abs() < 2.0));
    if mine && let Ok(mut marks) = MARKS.lock() { *marks = (rims, clock()); }
}

// ---------- Vila e prefeito ----------

fn village() -> Option<[f64; 3]> {
    workforce::spawn::center().map(|([x, y, z], _)| [f64::from(x) + 0.5, f64::from(y), f64::from(z) + 0.5])
}

/// Tick de uma entidade do prefeito: guarda o id e tira uma cópia a mais.
pub(crate) fn mayor_seen(host: &mut dyn Host, dimension: &str, id: u32) {
    let Ok(mut known) = MAYOR_ID.lock() else { return };
    match *known {
        Some(other) if other != id && host.entity(other).is_some() => { host.remove_mob(dimension, id); }
        _ => *known = Some(id),
    }
}

/// Onde o prefeito está: a entidade, ou o lugar salvo quando ela está longe (chunk descarregado).
fn mayor_at(host: &dyn Host, p: &Progress) -> Option<[f64; 3]> {
    let id = MAYOR_ID.lock().ok().and_then(|id| *id);
    id.and_then(|id| host.entity(id)).and_then(|handle| handle.position(host)).or(p.mayor)
}

/// O prefeito chega à praça (o ponto mais baixo e seco a 3 blocos do centro), ou ao lado do
/// jogador sem vila conhecida.
fn summon_mayor(host: &mut dyn Host, player: &NativeServerPlayer, p: &mut Progress) {
    if player.dimension != OVERWORLD { return; }
    let id = MAYOR_ID.lock().ok().and_then(|id| *id);
    if let Some(at) = id.and_then(|id| host.entity(id)).and_then(|handle| handle.position(host)) {
        if p.mayor != Some(at) { p.mayor = Some(at); p.dirty = true; }
        return;
    }
    if host.run_command(&format!("execute if entity @e[type={MAYOR}]")) { return; }
    let spot = match workforce::spawn::center() {
        Some(([x, _, z], _)) => {
            let columns: Vec<(i32, i32)> = [(3, 0), (-3, 0), (0, 3), (0, -3), (2, 2), (-2, 2), (2, -2), (-2, -2)]
                .iter().map(|(dx, dz)| (x + dx, z + dz)).collect();
            let ground = host.surface_heights(OVERWORLD, &columns, NativeHeightmap::MotionBlockingNoLeaves).ok();
            let floor = host.surface_heights(OVERWORLD, &columns, NativeHeightmap::OceanFloor).ok();
            ground.zip(floor).and_then(|(ground, floor)| columns.iter().zip(ground.iter().zip(&floor))
                .filter_map(|(col, (g, f))| (g.is_some() && g == f).then(|| (*col, g.unwrap())))
                .min_by_key(|(_, height)| *height))
                .map(|((cx, cz), y)| [f64::from(cx) + 0.5, f64::from(y), f64::from(cz) + 0.5])
        }
        None => None,
    }.unwrap_or_else(|| {
        let [x, y, z] = player.position;
        [x.floor() + 2.5, y.floor(), z.floor() + 0.5]
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
        p.mayor = Some(spot); p.dirty = true;
        host.send_particles(&ParticleOptions::simple(ParticleType::HappyVillager), None,
            [spot[0], spot[1] + 1.0, spot[2]], [0.3, 0.5, 0.3], 0.0, 10);
    }
}

// ---------- Terreno ----------

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

/// O quadrado ocupa a casa do fundador (com 3 blocos de folga)?
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

// ---------- Falas ----------

fn title(host: &mut dyn Host, player: &NativeServerPlayer, title: &str, subtitle: &str) {
    if !spawn_safe(&player.name) { return; }
    let name = &player.name;
    host.run_command(&format!(r#"title {name} subtitle {{"translate":"{subtitle}"}}"#));
    host.run_command(&format!(r#"title {name} title {{"translate":"{title}","color":"gold"}}"#));
}

fn sound(host: &mut dyn Host, player: &NativeServerPlayer, sound: &str) {
    if spawn_safe(&player.name) { host.run_command(&format!("playsound {sound} master {}", player.name)); }
}

/// Uma fala do prefeito no chat do jogador.
pub(super) fn say(host: &mut dyn Host, player: u128, key: &str, args: &[&dyn std::fmt::Display]) {
    let name = Text::tr("veio.onboarding.mayor.name", &[]).plain();
    host.tell(player, Text::tr("veio.agenda.says", &[&name, &Text::tr(key, args).plain()]));
}

fn announce(host: &mut dyn Host, player: &NativeServerPlayer, p: &Progress) {
    let uuid = player.uuid;
    match p.intro {
        VILLAGE_STAGE => {
            title(host, player, "veio.onboarding.intro.welcome", "veio.onboarding.intro.welcome_sub");
            for line in 1..=2 { host.tell(uuid, Text::tr(&format!("veio.onboarding.intro.village.{line}"), &[])); }
        }
        MAYOR_STAGE => {
            title(host, player, "veio.onboarding.intro.arrived", "veio.onboarding.intro.arrived_sub");
            host.tell(uuid, Text::tr("veio.onboarding.intro.mayor.1", &[]));
        }
        GATHER_STAGE => title(host, player, "veio.onboarding.intro.gather_title", "veio.onboarding.intro.gather_sub"),
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

// ---------- Tick ----------

/// Um passo do prólogo, a cada segundo, para um jogador online.
pub(super) fn tick(host: &mut dyn Host, player: &NativeServerPlayer, p: &mut Progress, now: u64) {
    let since = *p.intro_since.get_or_insert(now);
    match p.intro {
        VILLAGE_STAGE => {
            let arrived = village().is_some_and(|at| (at[0] - player.position[0]).hypot(at[2] - player.position[2]) <= ARRIVED);
            if arrived || (village().is_none() && now >= since + VILLAGE_WAIT) {
                p.intro = MAYOR_STAGE; p.dirty = true;
                sound(host, player, "minecraft:block.bell.use");
            }
        }
        MAYOR_STAGE | GATHER_STAGE => summon_mayor(host, player, p),
        PLOT_STAGE => plot_tick(host, player, p, now),
        _ => {}
    }
    if p.hidden { return; }
    if p.intro_announced != Some(p.intro) {
        announce(host, player, p);
        p.intro_announced = Some(p.intro);
    }
    match p.intro {
        VILLAGE_STAGE => {
            let bar = match village() {
                Some(at) => { let (distance, way) = pointer(player, at); Text::tr("veio.onboarding.intro.goal.village", &[&way, &distance]) }
                None => Text::tr("veio.onboarding.intro.goal.village_wait", &[]),
            };
            host.action_bar(player.uuid, bar);
        }
        MAYOR_STAGE => {
            let bar = mayor_bar(host, player, p, "veio.onboarding.intro.goal.mayor");
            host.action_bar(player.uuid, bar);
        }
        GATHER_STAGE => gather_bar(host, player, p),
        PLOT_STAGE => { let bar = plot_bar(player, p); host.action_bar(player.uuid, bar); }
        _ => {}
    }
}

fn mayor_bar(host: &dyn Host, player: &NativeServerPlayer, p: &Progress, key: &str) -> Text {
    let at = mayor_at(host, p);
    if let Some(at) = at {
        publish_marks(player, vec![NativeBlockRim { pos: at.map(|v| v.floor() as i32), argb: 0xffffc040, width: 3.0 }]);
    }
    match at {
        Some(at) if heading(player.position, at).0 > 4 => {
            let (distance, way) = pointer(player, at);
            Text::tr(key, &[&way, &distance])
        }
        _ => Text::tr("veio.onboarding.intro.goal.mayor_here", &[]),
    }
}

fn has_item(host: &mut dyn Host, name: &str, item: &str) -> bool {
    host.run_command(&format!("execute if items entity {name} container.* {item}"))
}

/// Recursos do prefeito acima da barra: verde o que já tem, vermelho o que falta; com tudo, a seta
/// volta ao prefeito.
fn gather_bar(host: &mut dyn Host, player: &NativeServerPlayer, p: &Progress) {
    if !spawn_safe(&player.name) { return; }
    let mut parts = vec![r#"{"translate":"veio.onboarding.intro.goal.gather","color":"gold"}"#.to_owned()];
    let mut all = true;
    for (item, count, key) in RESOURCES {
        let have = has_item(host, &player.name, item);
        all &= have;
        let (mark, color) = if have { ("✔", "green") } else { ("✘", "red") };
        parts.push(format!(r#"{{"text":"  {mark} {count} ","color":"{color}"}},{{"translate":"{key}","color":"{color}"}}"#));
    }
    if all {
        let bar = mayor_bar(host, player, p, "veio.onboarding.intro.goal.deliver");
        host.action_bar(player.uuid, bar);
        return;
    }
    host.run_command(&format!(r#"title {} actionbar ["",{}]"#, player.name, parts.join(",")));
}

fn plot_bar(player: &NativeServerPlayer, p: &Progress) -> Text {
    let Some(plot) = p.plot else { return Text::literal("") };
    let center = plot_center(plot);
    let (distance, way) = pointer(player, center);
    if f64::from(distance) > NEAR + f64::from(HALF) {
        return Text::tr("veio.onboarding.intro.goal.plot_far", &[&way, &distance]);
    }
    match p.plot_level {
        Some(level) => Text::tr("veio.onboarding.intro.goal.plot",
            &[&level.left.saturating_sub(SLACK), &level.dig, &level.fill]),
        None => Text::tr("veio.onboarding.intro.goal.plot_far", &[&way, &distance]),
    }
}

/// Contorno verde da borda do terreno, para a janela.
fn plot_rims(plot: Pos) -> Vec<NativeBlockRim> {
    let [x, y, z] = plot;
    let mut rims = Vec::new();
    for step in -HALF..=HALF {
        for (cx, cz) in [(x + step, z - HALF), (x + step, z + HALF), (x - HALF, z + step), (x + HALF, z + step)] {
            let pos = [cx, y - 1, cz];
            if !rims.iter().any(|rim: &NativeBlockRim| rim.pos == pos) {
                rims.push(NativeBlockRim { pos, argb: 0xff60e060, width: 2.0 });
            }
        }
    }
    rims
}

/// Mede o terreno, desenha as marcas para quem está perto e fecha o prólogo quando ele fica plano.
fn plot_tick(host: &mut dyn Host, player: &NativeServerPlayer, p: &mut Progress, now: u64) {
    let Some(plot) = p.plot else { p.intro = MAYOR_STAGE; return };
    if player.dimension != OVERWORLD || now % 40 != 0 { return; }
    publish_marks(player, plot_rims(plot));
    let center = plot_center(plot);
    let near = (player.position[0] - center[0]).hypot(player.position[2] - center[2]) <= MARKS_RANGE;
    if !near { return; }
    let Some((state, wrong)) = survey(host, plot) else { return };
    p.plot_level = Some(state);
    particles(host, plot, &wrong);
    if state.left <= SLACK {
        p.intro = INTRO_DONE; p.dirty = true; p.plot_level = None;
        publish_marks(player, Vec::new());
        title(host, player, "veio.onboarding.intro.plot_done", "veio.onboarding.title.0");
        sound(host, player, "minecraft:ui.toast.challenge_complete");
        if spawn_safe(&player.name) { host.run_command(&format!("xp add {} 20 points", player.name)); }
        for line in 1..=3 { say(host, player.uuid, &format!("veio.onboarding.mayor.plot_done.{line}"), &[]); }
    }
}

/// Faíscas verdes na borda; chamas onde cavar e faíscas azuis onde aterrar.
fn particles(host: &mut dyn Host, plot: Pos, wrong: &[((i32, i32), i32)]) {
    let [x, y, z] = plot;
    for step in (-HALF..=HALF).step_by(2) {
        for (cx, cz) in [(x + step, z - HALF), (x + step, z + HALF), (x - HALF, z + step), (x + HALF, z + step)] {
            host.send_particles(&ParticleOptions::simple(ParticleType::HappyVillager), None,
                [f64::from(cx) + 0.5, f64::from(y) + 0.3, f64::from(cz) + 0.5], [0.0, 0.2, 0.0], 0.0, 1);
        }
    }
    for ((cx, cz), height) in wrong.iter().take(60) {
        let (kind, at_y) = if *height > y { (ParticleType::Flame, *height) } else { (ParticleType::ElectricSpark, y) };
        host.send_particles(&ParticleOptions::simple(kind), None,
            [f64::from(*cx) + 0.5, f64::from(at_y) + 0.2, f64::from(*cz) + 0.5], [0.15, 0.1, 0.15], 0.0, 2);
    }
}

/// Barra depois da última etapa da fábrica: voltar ao prefeito.
pub(super) fn outro_bar(host: &mut dyn Host, player: &NativeServerPlayer, p: &Progress) {
    let bar = mayor_bar(host, player, p, "veio.onboarding.outro.goal");
    host.action_bar(player.uuid, bar);
}

// ---------- Conversa ----------

/// Entrega os recursos se o jogador tem todos; senão devolve o que tirou. Devolve o que falta.
fn deliver(host: &mut dyn Host) -> Vec<(u32, &'static str)> {
    let mut taken = Vec::new();
    let mut missing = Vec::new();
    for (item, count, key) in RESOURCES {
        let got = host.take_items(item, count);
        taken.push((item, got));
        if got < count { missing.push((count - got, key)); }
    }
    if !missing.is_empty() {
        for (item, got) in taken { if got > 0 { host.give_items(item, got); } }
    }
    missing
}

/// O jogador clicou no prefeito: a fala depende de onde ele está no tutorial.
pub(super) fn talk(host: &mut dyn Host, player: &NativeServerPlayer, p: &mut Progress) {
    let uuid = player.uuid;
    sound(host, player, "minecraft:entity.villager.ambient");
    match p.intro {
        VILLAGE_STAGE | MAYOR_STAGE => {
            for line in 1..=5 { say(host, uuid, &format!("veio.onboarding.mayor.hello.{line}"), &[]); }
            for (count, key) in RESOURCES.map(|(_, count, key)| (count, key)) {
                say(host, uuid, "veio.onboarding.mayor.need", &[&count, &Text::tr(key, &[]).plain()]);
            }
            for line in 1..=3 { host.tell(uuid, Text::tr(&format!("veio.onboarding.mayor.how.{line}"), &[])); }
            p.intro = GATHER_STAGE; p.dirty = true;
        }
        GATHER_STAGE => {
            let missing = deliver(host);
            if !missing.is_empty() {
                let list = missing.iter().map(|(count, key)| format!("{count} {}", Text::tr(key, &[]).plain()))
                    .collect::<Vec<_>>().join(", ");
                say(host, uuid, "veio.onboarding.mayor.missing", &[&list]);
                return;
            }
            host.give_items(GENERATOR, 1);
            if spawn_safe(&player.name) {
                for recipe in UNLOCKS { host.run_command(&format!("recipe give {} {recipe}", player.name)); }
            }
            title(host, player, "veio.onboarding.intro.machine_title", "veio.onboarding.intro.machine_sub");
            sound(host, player, "minecraft:ui.toast.challenge_complete");
            for line in 1..=2 { say(host, uuid, &format!("veio.onboarding.mayor.machine.{line}"), &[]); }
            let around = workforce::spawn::center().map(|(at, _)| at)
                .unwrap_or(player.position.map(|v| v.floor() as i32));
            let plot = choose_plot(host, around);
            let (distance, way) = pointer(player, plot_center(plot));
            say(host, uuid, "veio.onboarding.mayor.plot", &[&distance, &way, &PLOT, &PLOT]);
            p.plot = Some(plot); p.intro = PLOT_STAGE; p.dirty = true; p.intro_announced = None;
        }
        PLOT_STAGE => {
            let left = p.plot_level.map_or(PLOT as u32 * PLOT as u32, |level| level.left.saturating_sub(SLACK));
            if let Some(plot) = p.plot {
                let (distance, way) = pointer(player, plot_center(plot));
                say(host, uuid, "veio.onboarding.mayor.plot_again", &[&distance, &way, &left]);
            }
        }
        // Capítulo da vila aberto (também pelo comando, antes de acabar a fábrica): ele vem primeiro.
        _ if super::village::step(p).is_some() => super::village::talk(host, player, p),
        _ if p.step < DONE => {
            say(host, uuid, "veio.onboarding.mayor.now", &[&Text::tr(&format!("veio.onboarding.title.{}", p.step), &[]).plain()]);
            say(host, uuid, &format!("veio.onboarding.story.{}", p.step), &[]);
            host.tell(uuid, Text::tr(&format!("veio.onboarding.why.{}", p.step), &[]));
            for line in 1..=DO_LINES.get(usize::from(p.step)).copied().unwrap_or(0) {
                host.tell(uuid, Text::tr(&format!("veio.onboarding.do.{}.{line}", p.step), &[]));
            }
        }
        _ if !p.outro => {
            for line in 1..=5 { say(host, uuid, &format!("veio.onboarding.mayor.next.{line}"), &[]); }
            say(host, uuid, "veio.village.offer", &[]);
            p.outro = true; p.dirty = true;
            title(host, player, "veio.onboarding.outro.title", "veio.onboarding.outro.sub");
            sound(host, player, "minecraft:ui.toast.challenge_complete");
            if spawn_safe(&player.name) {
                host.run_command(&format!("xp add {} 50 points", player.name));
                host.run_command(&format!("recipe give {} {}", player.name, milestones::BOARD));
            }
            host.action_bar(uuid, Text::literal(""));
        }
        // Depois da fábrica, o prefeito conduz o capítulo da vila até o fim.
        _ if p.village < super::village::DONE_VILLAGE => super::village::talk(host, player, p),
        // Depois do tutorial: a fase do Quadro da fábrica e o gargalo que ela vai revelar.
        _ => match milestones::census::board_phase(&player.dimension) {
            None => say(host, uuid, "veio.onboarding.mayor.board.none", &[]),
            Some(phase) => match milestones::PHASES.get(phase) {
                Some(goal) => {
                    let name = Text::tr(&format!("veio.board.phase.{}", goal.key), &[]).plain();
                    say(host, uuid, "veio.onboarding.mayor.board.phase", &[&name, &goal.count, &Text::tr(goal.item_key, &[]).plain()]);
                    if let Some(next) = milestones::PHASES.get(phase + 1) {
                        say(host, uuid, &format!("veio.board.hint.{}", next.key), &[]);
                    }
                    say(host, uuid, "veio.onboarding.mayor.board.grow", &[]);
                }
                None => say(host, uuid, "veio.onboarding.mayor.board.done", &[]),
            },
        },
    }
}

/// Outro mundo abriu: os ids e as marcas eram do anterior.
pub(super) fn reset() {
    if let Ok(mut id) = MAYOR_ID.lock() { *id = None; }
    if let Ok(mut marks) = MARKS.lock() { *marks = (Vec::new(), 0); }
}
