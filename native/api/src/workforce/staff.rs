//! Os prédios prontos funcionando: funcionário, visitantes e pesquisa.
//!
//! Depois que o mestre de obras termina (`site.rs`), o canteiro vira o posto do prédio. Um aldeão
//! contratado com esmeralda (e comida no slot) trabalha ali, andando entre os móveis da planta:
//!
//! | Prédio | Funcionário | O que faz de verdade |
//! | --- | --- | --- |
//! | Refeitório | cozinheira | Cozinha a carne, o peixe e a batata crus do slot de entrada |
//! | Escola | professor | Dá aula: as crianças da vila vão às carteiras e crescem mais rápido |
//! | Ferraria | ferreiro | 2 chapas de ferro viram 2 kits de reparo |
//! | Biblioteca | bibliotecário | 3 papéis viram 1 livro encadernado |
//! | Correio | carteiro | Faz a ronda pelas casas e postos da vila |
//! | Estalagem | estalajadeiro | Atende o balcão; à noitinha os hóspedes vêm às mesas |
//! | Centro de pesquisa | cientista | Pesquisa tecnologias e instala módulos nas máquinas |
//!
//! A praça do mercado funciona com as lojas (comerciantes da sociedade) nos lugares reservados, e a
//! prefeitura com o bloco da prefeitura (o prefeito). O efeito da obra só vale com o prédio de pé
//! **e funcionando** (funcionário presente, ou lojas e prefeitura no lugar).
//!
//! **Visitantes:** no horário da planta, aldeões livres (adultos ou crianças) andam até os
//! assentos: almoço no refeitório, aula na escola, leitura na biblioteca, reunião do conselho na
//! prefeitura, compras na praça, jantar na estalagem. Usam as mesmas ordens de caminhada dos
//! postos; fora do horário voltam ao vanilla. Ninguém contratado é chamado.
use super::*;
use std::sync::atomic::{AtomicBool, Ordering};

/// Visitas (chegadas ao móvel) entre duas tarefas que gastam comida.
pub(crate) const SHIFT: i32 = 6;
/// Aldeões procurados por ciclo para virar visitantes (leituras de NBT).
const PROBES: usize = 4;
const VISIT_RADIUS: f64 = 32.0;
/// A cada `CLASS_EVERY` ciclos de aula, cada aluno sentado cresce `GROWTH` ticks.
const CLASS_EVERY: i32 = 20;
pub(crate) const GROWTH: i32 = 1200;
/// Alcance do cientista para instalar módulos.
const INSTALL_RADIUS: i32 = 12;

pub(crate) fn staffed(role: &str) -> bool {
    matches!(role, "cook" | "teacher" | "smith" | "librarian" | "postman" | "innkeeper" | "scientist")
}

// ---------- Pesquisa ----------

pub(crate) const AGRONOMY: u32 = 1;
pub(crate) const CARGO: u32 = 2;
pub(crate) const NUTRITION: u32 = 4;
pub(crate) const PEDAGOGY: u32 = 8;
pub(crate) const ENGINEERING: u32 = 16;
/// Tarefas do cientista depois dos materiais entregues.
pub(crate) const RESEARCH_WORK: i32 = 24;

pub(crate) struct Tech {
    pub key: &'static str,
    pub bit: u32,
    pub needs: [projects::Material; 3],
}

const fn m(item: &'static str, count: u32, name: &'static str) -> projects::Material {
    projects::Material { item, count, name }
}

pub(crate) const TECHS: [Tech; 5] = [
    Tech { key: "agronomy", bit: AGRONOMY, needs: [m("minecraft:bone_meal", 16, "item.minecraft.bone_meal"),
        m("minecraft:wheat_seeds", 32, "item.minecraft.wheat_seeds"), m("minecraft:composter", 2, "block.minecraft.composter")] },
    Tech { key: "cargo", bit: CARGO, needs: [m("veio:gear", 8, "item.veio.gear"),
        m("veio:iron_plate", 8, "item.veio.iron_plate"), m("minecraft:chest", 4, "block.minecraft.chest")] },
    Tech { key: "nutrition", bit: NUTRITION, needs: [m("minecraft:bread", 16, "item.minecraft.bread"),
        m("minecraft:cooked_beef", 8, "item.minecraft.cooked_beef"), m("minecraft:baked_potato", 8, "item.minecraft.baked_potato")] },
    Tech { key: "pedagogy", bit: PEDAGOGY, needs: [m("minecraft:book", 16, "item.minecraft.book"),
        m("minecraft:paper", 16, "item.minecraft.paper"), m("minecraft:lantern", 4, "block.minecraft.lantern")] },
    Tech { key: "engineering", bit: ENGINEERING, needs: [m("veio:circuit", 4, "item.veio.circuit"),
        m("veio:module", 2, "item.veio.module"), m("minecraft:redstone", 16, "item.minecraft.redstone")] },
];

fn tech_key(tech: usize, need: usize) -> String {
    format!("Tech{tech}_{need}")
}

pub(crate) fn known(entity: &ModBlockEntity) -> u32 {
    number(entity, "Techs") as u32
}

/// A pesquisa em andamento: a primeira ainda não feita.
pub(crate) fn current(entity: &ModBlockEntity) -> Option<usize> {
    TECHS.iter().position(|tech| known(entity) & tech.bit == 0)
}

pub(crate) fn tech_missing(entity: &ModBlockEntity, tech: usize) -> Option<(usize, u32)> {
    TECHS[tech].needs.iter().enumerate()
        .map(|(index, need)| (index, need.count.saturating_sub(number(entity, &tech_key(tech, index)) as u32)))
        .find(|(_, left)| *left > 0)
}

/// Leva o que está no slot de entrada para a pesquisa em andamento.
fn feed_research(entity: &mut ModBlockEntity) {
    let Some(tech) = current(entity) else { return };
    let Some(stack) = entity.items.get(1).and_then(Option::as_ref).filter(|stack| plain(stack)).cloned() else { return };
    let Some((index, need)) = TECHS[tech].needs.iter().enumerate().find(|(_, need)| need.item == stack.item.as_ref()) else { return };
    let have = number(entity, &tech_key(tech, index)) as u32;
    let moved = need.count.saturating_sub(have).min(stack.count);
    if moved > 0 {
        set_number(entity, &tech_key(tech, index), (have + moved) as i32);
        consume(&mut entity.items[1], moved);
    }
}

/// Uma tarefa do cientista: avança a pesquisa. Devolve a tecnologia concluída agora.
fn research(entity: &mut ModBlockEntity) -> Option<usize> {
    let tech = current(entity)?;
    if tech_missing(entity, tech).is_some() {
        return None;
    }
    let work = number(entity, "RWork") + 1;
    if work < RESEARCH_WORK {
        set_number(entity, "RWork", work);
        return None;
    }
    set_number(entity, "RWork", 0);
    set_number(entity, "Techs", (known(entity) | TECHS[tech].bit) as i32);
    Some(tech)
}

/// Porcentagem da pesquisa em andamento (materiais e trabalho contam metade cada).
pub(crate) fn research_percent(entity: &ModBlockEntity) -> i32 {
    let Some(tech) = current(entity) else { return 100 };
    let (have, total) = TECHS[tech].needs.iter().enumerate().fold((0, 0), |(have, total), (index, need)| {
        (have + (number(entity, &tech_key(tech, index)) as u32).min(need.count), total + need.count)
    });
    (have * 50 / total.max(1)) as i32 + number(entity, "RWork") * 50 / RESEARCH_WORK
}

// ---------- Visitantes ----------

struct Visit {
    dimension: String,
    pos: [i32; 3],
    guests: Vec<(u32, Uuid, usize)>,
}

static VISITS: Mutex<Vec<Visit>> = Mutex::new(Vec::new());
static VISITING: AtomicBool = AtomicBool::new(false);

/// Visitante de outro prédio agora?
fn busy_elsewhere(id: u32, dimension: &str, pos: [i32; 3]) -> bool {
    VISITS.lock().map(|list| list.iter().any(|visit| !(visit.dimension == dimension && visit.pos == pos)
        && visit.guests.iter().any(|(known, _, _)| *known == id))).unwrap_or(false)
}

fn guests(dimension: &str, pos: [i32; 3]) -> Vec<(u32, Uuid, usize)> {
    if !VISITING.load(Ordering::Acquire) {
        return Vec::new();
    }
    VISITS.lock().map(|list| list.iter().find(|visit| visit.dimension == dimension && visit.pos == pos)
        .map(|visit| visit.guests.clone()).unwrap_or_default()).unwrap_or_default()
}

fn set_guests(dimension: &str, pos: [i32; 3], guests: Vec<(u32, Uuid, usize)>) {
    if let Ok(mut list) = VISITS.lock() {
        list.retain(|visit| !(visit.dimension == dimension && visit.pos == pos));
        if !guests.is_empty() {
            list.push(Visit { dimension: dimension.to_owned(), pos, guests });
        }
        VISITING.store(!list.is_empty(), Ordering::Release);
    }
}

#[cfg(test)]
pub(crate) fn visitors_of(dimension: &str, pos: [i32; 3]) -> Vec<u32> {
    guests(dimension, pos).into_iter().map(|(id, _, _)| id).collect()
}

/// Ciclo dos visitantes (a cada 10 ticks, no prédio pronto e funcionando).
pub(crate) fn visitors(host: &mut dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity, plan: &blueprints::Blueprint,
    rot: i32, operating: bool, now: u64)
{
    let dimension = host.dimension();
    let hour = host.day_time().rem_euclid(24000);
    let open = operating && plan.visitors > 0 && hour >= plan.hours[0] && hour < plan.hours[1];
    let mut list = guests(dimension, pos);
    if !open {
        if !list.is_empty() {
            for (id, _, _) in &list {
                cancel(*id);
            }
            set_guests(dimension, pos, Vec::new());
        }
        set_number(entity, "Guests", 0);
        return;
    }
    // Quem foi contratado no meio da visita vai trabalhar.
    list.retain(|(_, uuid, _)| !claimed_elsewhere(*uuid, dimension, pos, now));
    let room = plan.max_visitors.min(plan.seats.len());
    if list.len() < room {
        let center = post_center(pos);
        let mut candidates: Vec<(f64, u32)> = production::ranch::villager_positions(host, dimension).iter().copied()
            .filter(|(id, at)| (at[0] - center[0]).hypot(at[2] - center[2]) <= VISIT_RADIUS && (at[1] - center[1]).abs() <= 8.0
                && !list.iter().any(|(known, _, _)| known == id))
            .map(|(id, at)| ((at[0] - center[0]).hypot(at[2] - center[2]), id))
            .collect();
        candidates.sort_by(|a, b| a.0.total_cmp(&b.0));
        let start = number(entity, "Probe") as usize;
        let count = candidates.len();
        for step in 0..PROBES.min(count) {
            if list.len() >= room {
                break;
            }
            let (_, id) = candidates[(start + step) % count];
            if busy_elsewhere(id, dimension, pos) || order_for(id, now).is_some() {
                continue;
            }
            let Some(nbt) = host.entity(id).and_then(|handle| host.entity_nbt(handle)) else { continue };
            let Some(uuid) = uuid_of(&nbt) else { continue };
            let child = production::ranch::tag_int(&nbt, "Age") < 0;
            if (plan.visitors == 2) != child || claimed_elsewhere(uuid, dimension, pos, now) {
                continue;
            }
            let seat = (0..plan.seats.len()).find(|seat| !list.iter().any(|(_, _, taken)| taken == seat)).unwrap_or(0);
            list.push((id, uuid, seat));
        }
        set_number(entity, "Probe", ((start + PROBES) % count.max(1)) as i32);
    }
    for (id, _, seat) in &list {
        let spot = blueprints::world(plan, pos, rot, plan.seats[*seat]);
        order(*id, post_center(spot), now);
    }
    set_number(entity, "Guests", list.len() as i32);
    // Aula: alunos sentados crescem mais rápido (pedagogia: o dobro).
    if plan.visitors == 2 {
        let round = number(entity, "Class") + 1;
        set_number(entity, "Class", round % CLASS_EVERY);
        if round % CLASS_EVERY == 0 {
            let growth = if projects::at(dimension, pos, now) & projects::research_bit(PEDAGOGY) != 0 { GROWTH * 2 } else { GROWTH };
            let mut graduated = Vec::new();
            for (id, _, seat) in &list {
                let spot = post_center(blueprints::world(plan, pos, rot, plan.seats[*seat]));
                let Some(handle) = host.entity(*id) else { continue };
                if !handle.position(host).is_some_and(|at| near(at, spot, 2.0)) {
                    continue;
                }
                let Some(nbt) = host.entity_nbt(handle) else { continue };
                let age = production::ranch::tag_int(&nbt, "Age");
                let next = (age + growth).min(0);
                host.entity_merge_nbt(handle, Compound::from([("Age".into(), Tag::Int(next))]));
                set_number(entity, "Taught", number(entity, "Taught").saturating_add(1));
                if next >= 0 {
                    graduated.push(*id);
                }
            }
            if !graduated.is_empty() {
                for id in &graduated {
                    cancel(*id);
                }
                list.retain(|(id, _, _)| !graduated.contains(id));
            }
        }
    }
    set_guests(dimension, pos, list);
}

// ---------- Funcionário ----------

/// Próxima tarefa do funcionário: o próximo móvel da planta (o carteiro alterna com a ronda; o
/// cientista vai instalar módulos quando sabe engenharia e há módulo no slot).
pub(crate) fn plan(host: &mut dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity, plan: &blueprints::Blueprint, rot: i32,
    now: u64) -> Result<Plan, i32>
{
    if plan.work.is_empty() {
        return Err(IDLE_WORK);
    }
    let dimension = host.dimension();
    if plan.role == "scientist" && known(entity) & ENGINEERING != 0
        && entity.items.get(1).and_then(Option::as_ref).is_some_and(|stack| upgrades::index(&stack.item).is_some())
        && let Some(machine) = machine_to_upgrade(host, pos, entity)
    {
        set_number(entity, "Install", 1);
        set_target(entity, Some(machine));
        return Ok(Plan::Visit { spot: machine, reach: POST_REACH });
    }
    set_number(entity, "Install", 0);
    let round = number(entity, "Round") + 1;
    set_number(entity, "Round", round % 1024);
    if plan.role == "postman" && round % 2 == 0 {
        let mut stops: Vec<[i32; 3]> = center::posts_near(dimension, pos, center::RADIUS, now).into_iter()
            .map(|post| post.pos).filter(|stop| *stop != pos).collect();
        stops.extend(society::homes_near(dimension, pos, center::RADIUS, now).into_iter().map(|home| home.pos));
        if !stops.is_empty() {
            let stop = stops[(round as usize / 2) % stops.len()];
            set_number(entity, "Mail", 1);
            set_target(entity, Some(stop));
            return Ok(Plan::Visit { spot: stop, reach: POST_REACH });
        }
    }
    set_number(entity, "Mail", 0);
    let spot = blueprints::world(plan, pos, rot, plan.work[round as usize % plan.work.len()]);
    set_target(entity, Some(spot));
    Ok(Plan::Visit { spot, reach: REACH })
}

/// Máquina do Veio perto do laboratório que aceita módulo e ainda não tem um.
fn machine_to_upgrade(host: &mut dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity) -> Option<[i32; 3]> {
    let found = scan_named(host, pos, entity, "Lab", INSTALL_RADIUS, 32, 16, &|block| upgrades::supports(block), false)?;
    found.into_iter().find(|spot| {
        let mut free = false;
        host.change_mod_block_entity(*spot, &mut |machine| free = number(machine, "Module") == 0);
        free
    })
}

/// Chegou ao móvel (ou à máquina, ou à casa da ronda). A cada `SHIFT` chegadas, o trabalho do
/// prédio acontece e conta uma tarefa (gasta comida).
pub(crate) fn perform(host: &mut dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity, plan: &blueprints::Blueprint)
    -> Option<Vec<(String, u32)>>
{
    if number(entity, "Install") == 1 {
        let spot = target(entity)?;
        let module = entity.items.get(1).and_then(Option::as_ref).and_then(|stack| upgrades::index(&stack.item))?;
        let mut done = false;
        host.change_mod_block_entity(spot, &mut |machine| {
            if upgrades::supports(&machine.id) && number(machine, "Module") == 0 {
                set_number(machine, "Module", module);
                done = true;
            }
        });
        if done {
            consume(&mut entity.items[1], 1);
            set_number(entity, "Installed", number(entity, "Installed").saturating_add(1));
            host.play_sound("minecraft:block.smithing_table.use", center(spot), 4, 0.6, 1.2);
            return Some(Vec::new());
        }
        return None;
    }
    if number(entity, "Mail") == 1 {
        set_number(entity, "Delivered", number(entity, "Delivered").saturating_add(1));
        host.play_sound("minecraft:item.book.page_turn", center(target(entity).unwrap_or(pos)), 4, 0.6, 1.0);
        return Some(Vec::new());
    }
    let shift = number(entity, "Shift") + 1;
    set_number(entity, "Shift", shift % SHIFT);
    if shift % SHIFT != 0 {
        return None;
    }
    match plan.role {
        "cook" => {
            if let Some(raw) = entity.items[1].as_ref().filter(|stack| plain(stack)).and_then(|stack| home::cooked(&stack.item))
                && output(entity, raw, 1)
            {
                consume(&mut entity.items[1], 1);
                set_number(entity, "Made", number(entity, "Made").saturating_add(1));
                host.play_sound("minecraft:block.smoker.smoke", center(pos), 4, 0.5, 1.0);
            }
        }
        "smith" => convert(host, pos, entity, "veio:iron_plate", 2, "veio:repair_kit", 2, "minecraft:block.anvil.use"),
        "librarian" => convert(host, pos, entity, "minecraft:paper", 3, "minecraft:book", 1, "minecraft:item.book.put"),
        "scientist" => {
            feed_research(entity);
            if let Some(tech) = research(entity) {
                announce_tech(host, pos, tech);
            }
        }
        _ => {}
    }
    Some(Vec::new())
}

fn output(entity: &mut ModBlockEntity, item: &str, count: u32) -> bool {
    let fits = match entity.items[2].as_ref() {
        None => true,
        Some(stack) => is_plain(stack, item) && stack.count + count <= 64,
    };
    if fits {
        add(&mut entity.items[2], item, count);
    }
    fits
}

#[allow(clippy::too_many_arguments)]
fn convert(host: &mut dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity, input: &str, need: u32, made: &str, count: u32, sound: &str) {
    let enough = entity.items[1].as_ref().is_some_and(|stack| is_plain(stack, input) && stack.count >= need);
    if enough && output(entity, made, count) {
        consume(&mut entity.items[1], need);
        set_number(entity, "Made", number(entity, "Made").saturating_add(count as i32));
        host.play_sound(sound, center(pos), 4, 0.6, 1.0);
    }
}

fn announce_tech(host: &mut dyn Host, pos: [i32; 3], tech: usize) {
    let at = center(pos);
    host.play_sound("minecraft:ui.toast.challenge_complete", at, 4, 0.8, 1.2);
    host.send_particles(&ParticleOptions::simple(ParticleType::ElectricSpark), None, [at[0], at[1] + 1.5, at[2]], [1.0, 1.0, 1.0], 0.1, 30);
    let key = TECHS[tech].key;
    let line = Text::tr("veio.research.lab_done", &[&Text::tr(&format!("veio.research.{key}.name"), &[]).plain(),
        &Text::tr(&format!("veio.research.{key}.effect"), &[]).plain()]);
    let dimension = host.dimension();
    let players: Vec<u128> = host.server_players().into_iter()
        .filter(|player| player.dimension == dimension && center::distance2(player.position, at) < 64.0 * 64.0)
        .map(|player| player.uuid).collect();
    for player in players {
        host.tell(player, line.clone());
    }
}

/// O funcionário está trabalhando (presente, alimentado, sem caminho bloqueado)?
pub(crate) fn on_duty(entity: &ModBlockEntity) -> bool {
    worker(entity).is_some() && !matches!(number(entity, "Status"), NO_WORKER | ABSENT | NO_FOOD | PATH)
}

/// Aceita no slot de entrada o que o funcionário usa.
pub(crate) fn accepts_input(stack: &ItemStack) -> bool {
    let item = stack.item.as_ref();
    home::cooked(item).is_some() || matches!(item, "veio:iron_plate" | "minecraft:paper")
        || upgrades::index(item).is_some() || TECHS.iter().any(|tech| tech.needs.iter().any(|need| need.item == item))
}

#[cfg(test)]
pub(crate) fn feed_for_test(entity: &mut ModBlockEntity) {
    feed_research(entity);
}

#[cfg(test)]
pub(crate) fn research_for_test(entity: &mut ModBlockEntity) -> Option<usize> {
    research(entity)
}
