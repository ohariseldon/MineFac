//! Canteiro de obras: onde um prédio da vila (refeitório, escola, ferraria, mercado, biblioteca,
//! correio, estalagem, prefeitura) sai do papel.
//!
//! 1. **Planta.** O jogador põe o canteiro onde quer o prédio; a borda da planta aparece no chão
//!    e a entrada fica virada para onde o canteiro olha. A chave (sem agachar) troca a planta.
//!    O canteiro fica dentro do prédio, no lugar marcado na planta.
//! 2. **Pagamento.** A obra precisa estar paga no centro de controle a até 64 blocos (materiais no
//!    slot Obra, como antes) e ainda não estar de pé na vila nem em outro canteiro.
//! 3. **Terreno.** O canteiro confere a área: terra, pedra, grama, árvores e água saem; baús,
//!    camas, portas, máquinas e outros blocos do jogador param a obra até serem tirados.
//! 4. **Construção.** Um mestre de obras contratado (esmeralda, comida no slot) anda até cada
//!    parte e assenta `BATCH` blocos por vez, de baixo para cima; enfeites e móveis vão por último.
//!    Antes, o chão embaixo do piso é completado com pedregulho onde houver buraco.
//! 5. **De pé.** Pronto, o mestre é dispensado, o prédio passa a contar para a vila e o efeito da
//!    obra vale. Lugares reservados (lojas na praça, a prefeitura do prefeito) piscam até o
//!    jogador pôr o bloco do Veio ali.
//!
//! Um prédio só vale enquanto o canteiro existir (é o coração da obra). Tudo lido aos poucos:
//! a conferência lê `SURVEY` blocos por ciclo pela cota de trabalhadores do orçamento global.
use super::*;
use horizon_client::mod_api::{BlockEntitySubmitEvent, CubeFace, NativeBlockEdit, NativeBlockRun};
use std::sync::atomic::{AtomicBool, Ordering};

pub(crate) const SITE: &str = "veio:site";
/// Blocos assentados por tarefa do mestre de obras.
pub(crate) const BATCH: usize = 10;
/// Blocos lidos por ciclo na conferência do terreno e procurando o próximo bloco a assentar.
pub(crate) const SURVEY: i32 = 48;
/// Espera (ciclos de 10 ticks) para conferir de novo um terreno ocupado.
const RECHECK: i32 = 20;
const STALE: u64 = 400;
/// Alcance do mestre de obras: anda até perto de cada parte do prédio.
const BUILD_REACH: f64 = 5.0;

// Fases do canteiro (`Phase`).
pub(crate) const PLAN: i32 = 0;
pub(crate) const SURVEYING: i32 = 1;
pub(crate) const BUILDING: i32 = 2;
pub(crate) const DONE: i32 = 3;

// Estados do painel (`presentation::STATUS_KEYS`).
pub(crate) const SITE_PLAN: i32 = 36;
pub(crate) const SITE_BLOCKED: i32 = 37;
pub(crate) const SITE_BUILDING: i32 = 38;
pub(crate) const SITE_DONE: i32 = 39;

/// Por que a obra espera na fase de planta (`Utilization`).
pub(crate) const NO_CENTER: i32 = 1;
pub(crate) const NOT_PAID: i32 = 2;
pub(crate) const ALREADY: i32 = 3;
pub(crate) const ELSEWHERE: i32 = 4;

/// Slots: 0 comida (mestre de obras ou funcionário), 1 entrada do funcionário, 2 produto.
pub(crate) fn accepts(slot: usize, stack: &ItemStack) -> bool {
    plain(stack) && match slot {
        0 => is_food(&stack.item),
        1 => staff::accepts_input(stack),
        _ => false,
    }
}

/// O prédio pronto não tem funcionário para contratar (praça, prefeitura)?
pub(crate) fn refuses_hire(entity: &ModBlockEntity) -> bool {
    phase(entity) == DONE && !staff::staffed(plan(entity).role)
}

// ---------- Registro dos prédios ----------

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Building {
    pub dimension: String,
    pub pos: [i32; 3],
    pub bit: u32,
    /// Prédio terminado.
    pub standing: bool,
    /// De pé e funcionando (funcionário trabalhando, lojas ou prefeitura no lugar).
    pub operating: bool,
    /// Pesquisas concluídas (só o centro de pesquisa).
    pub research: u32,
    pub seen: u64,
}

/// Prédios em volta de um ponto, em bits de `projects`.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) struct Near {
    /// De pé e funcionando: o efeito da obra vale.
    pub operating: u32,
    /// Em obras (planta aprovada, conferência ou construção).
    pub busy: u32,
    /// Terminados, funcionando ou não.
    pub erected: u32,
    /// Pesquisas dos centros de pesquisa de pé.
    pub research: u32,
}

static BUILDINGS: Mutex<Vec<Building>> = Mutex::new(Vec::new());
static PENDING: AtomicBool = AtomicBool::new(false);

fn publish(building: Building) {
    if let Ok(mut list) = BUILDINGS.lock() {
        let now = building.seen;
        if let Some(known) = list.iter_mut().find(|known| known.dimension == building.dimension && known.pos == building.pos) {
            *known = building;
        } else {
            list.retain(|known| now.saturating_sub(known.seen) <= STALE);
            list.push(building);
        }
        PENDING.store(true, Ordering::Release);
    }
}

fn forget(dimension: &str, pos: [i32; 3]) {
    if let Ok(mut list) = BUILDINGS.lock() {
        list.retain(|known| !(known.dimension == dimension && known.pos == pos));
    }
}

/// Prédios a até `radius` blocos de `at`, sem contar o canteiro em `except`.
pub(crate) fn near(dimension: &str, at: [i32; 3], radius: i32, now: u64, except: Option<[i32; 3]>) -> Near {
    if !PENDING.load(Ordering::Acquire) {
        return Near::default();
    }
    BUILDINGS.lock().map(|list| list.iter()
        .filter(|known| known.dimension == dimension && now.saturating_sub(known.seen) <= STALE && Some(known.pos) != except
            && (0..3).all(|axis| (known.pos[axis] - at[axis]).abs() <= radius))
        .fold(Near::default(), |mut near, known| {
            if known.standing {
                near.erected |= known.bit;
                near.research |= known.research;
                if known.operating {
                    near.operating |= known.bit;
                }
            } else {
                near.busy |= known.bit;
            }
            near
        }))
        .unwrap_or_default()
}

/// Canteiros (de qualquer fase) a até `radius` blocos: as ruínas da vila e as obras do jogador.
pub(crate) fn sites_near(dimension: &str, at: [i32; 3], radius: i32, now: u64) -> Vec<Building> {
    if !PENDING.load(Ordering::Acquire) {
        return Vec::new();
    }
    BUILDINGS.lock().map(|list| list.iter()
        .filter(|known| known.dimension == dimension && now.saturating_sub(known.seen) <= STALE
            && (0..3).all(|axis| (known.pos[axis] - at[axis]).abs() <= radius))
        .cloned().collect()).unwrap_or_default()
}

// ---------- Estado ----------

pub(crate) fn plan_index(entity: &ModBlockEntity) -> usize {
    number(entity, "Rate") as usize % blueprints::BLUEPRINTS.len()
}

fn plan(entity: &ModBlockEntity) -> &'static blueprints::Blueprint {
    &blueprints::BLUEPRINTS[plan_index(entity)]
}

fn phase(entity: &ModBlockEntity) -> i32 {
    number(entity, "ContractStage")
}

fn set_phase(entity: &mut ModBlockEntity, phase: i32) {
    set_number(entity, "ContractStage", phase);
}

fn rot(entity: &ModBlockEntity) -> i32 {
    number(entity, "Rot")
}

/// Blocos assentados e total (para o painel).
fn publish_progress(entity: &mut ModBlockEntity) {
    let total = plan(entity).cells.len() as i32;
    set_number(entity, "Duration", total);
    set_number(entity, "Progress", number(entity, "Cursor").min(total));
}

/// Chave sem agachar: troca a planta (antes de a construção começar).
pub(crate) fn cycle(host: &mut dyn Host, pos: [i32; 3]) -> Option<Flow> {
    let facing = host.block_property(pos, "facing");
    let mut result = None;
    host.change_mod_block_entity(pos, &mut |entity| {
        if entity.id != SITE {
            return;
        }
        if phase(entity) >= BUILDING {
            result = Some(false);
            return;
        }
        let next = (plan_index(entity) + 1) % blueprints::BLUEPRINTS.len();
        set_number(entity, "Rate", next as i32);
        set_number(entity, "Rot", blueprints::rotation(facing.as_deref()));
        set_phase(entity, PLAN);
        set_number(entity, "Survey", 0);
        publish_progress(entity);
        result = Some(true);
    });
    let changed = result?;
    host.block_entity_sync(pos);
    if changed {
        let mut key = String::new();
        host.change_mod_block_entity(pos, &mut |entity| key = plan(entity).key.to_owned());
        notify_name(host, pos, &key);
        host.play_sound("minecraft:item.book.page_turn", center(pos), 4, 0.6, 1.1);
    } else {
        notify(host, pos, "veio.site.started");
    }
    Some(Flow::Cancel)
}

fn notify_name(host: &mut dyn Host, pos: [i32; 3], key: &str) {
    if let Some(player) = center::nearest_player(host, pos, 12.0) {
        host.action_bar(player, Text::tr("veio.site.picked", &[&Text::tr(&format!("veio.project.{key}.name"), &[]).plain()]));
    }
}

/// Tick do canteiro (a cada 10 ticks, na fase do posto). Planta, pagamento, terreno e, durante a
/// construção, o ciclo normal do posto com o mestre de obras.
pub(crate) fn tick(host: &mut dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity, time: i64, now: u64) {
    let dimension = host.dimension();
    if !entity.nbt.contains_key(nbt_key("Rot")) {
        set_number(entity, "Rot", blueprints::rotation(host.block_property(pos, "facing").as_deref()));
    }
    publish_progress(entity);
    let blueprint = plan(entity);
    let bit = blueprints::project_bit(blueprint);
    let due = (time + i64::from((pos[0] ^ pos[2]) & 7)).rem_euclid(10) == 0;
    let building = |standing: bool, operating: bool, research: u32| Building {
        dimension: dimension.to_owned(), pos, bit, standing, operating, research, seen: now,
    };
    if phase(entity) == BUILDING {
        if due {
            publish(building(false, false, 0));
        }
        step(host, pos, entity, time, now);
        return;
    }
    if phase(entity) == DONE {
        let staffed = staff::staffed(blueprint.role);
        if staffed {
            step(host, pos, entity, time, now);
        }
        if !due {
            return;
        }
        reserved(host, pos, entity, now);
        let operating = operating(entity, blueprint);
        let research = if blueprint.role == "scientist" { staff::known(entity) } else { 0 };
        publish(building(true, operating, research));
        set_number(entity, "Operating", i32::from(operating));
        if !staffed {
            set_number(entity, "Status", SITE_DONE);
        }
        staff::visitors(host, pos, entity, blueprint, rot(entity), operating, now);
        return;
    }
    if !due {
        return;
    }
    match phase(entity) {
        SURVEYING => {
            publish(building(false, false, 0));
            survey(host, pos, entity);
        }
        _ => {
            forget(dimension, pos);
            let reason = waiting_for(dimension, pos, bit, now);
            set_number(entity, "Utilization", reason);
            set_number(entity, "Status", SITE_PLAN);
            if reason == 0 {
                set_phase(entity, SURVEYING);
                set_number(entity, "Survey", 0);
                set_number(entity, "Wait", 0);
            }
        }
    }
}

/// O prédio pronto funciona? Funcionário trabalhando; na praça, ao menos 2 lojas no lugar; na
/// prefeitura, o bloco da prefeitura.
fn operating(entity: &ModBlockEntity, blueprint: &blueprints::Blueprint) -> bool {
    let empty = number(entity, "Utilization").max(0) as usize;
    match blueprint.role {
        "market" => blueprint.reserved.len().saturating_sub(empty) >= 2,
        "council" => empty == 0,
        role if staff::staffed(role) => staff::on_duty(entity),
        _ => true,
    }
}

/// Por que a obra ainda não pode começar (0: pode).
fn waiting_for(dimension: &str, pos: [i32; 3], bit: u32, now: u64) -> i32 {
    let Some(village) = projects::nearest(dimension, pos, center::RADIUS, now) else { return NO_CENTER };
    if village.funded & bit == 0 {
        return NOT_PAID;
    }
    let found = near(dimension, village.pos, center::RADIUS, now, Some(pos));
    if found.erected & bit != 0 {
        ALREADY
    } else if found.busy & bit != 0 {
        ELSEWHERE
    } else {
        0
    }
}

/// Confere o terreno aos poucos. Achou bloco do jogador: para e mostra onde.
fn survey(host: &mut dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity) {
    let wait = number(entity, "Wait");
    if wait > 0 {
        set_number(entity, "Wait", wait - 1);
        if wait > 1 {
            return;
        }
        set_number(entity, "Survey", 0);
    }
    if !budget::request(budget::Lane::Workforce, host.dimension(), pos, SURVEY as u32) {
        return;
    }
    let blueprint = plan(entity);
    let rot = rot(entity);
    let mut cursor = number(entity, "Survey") as usize;
    for _ in 0..SURVEY {
        let Some(&cell) = blueprint.cells.get(cursor) else { break };
        let (spot, index) = blueprints::decode(cell);
        let at = blueprints::world(blueprint, pos, rot, spot);
        let found = host.block(at);
        if at != pos && blueprints::protected(&found) && found != blueprints::PALETTE[index] {
            let offset = |axis: usize| (at[axis] - pos[axis]).clamp(-127, 127) + 128;
            set_number(entity, "Utilization", offset(0) | (offset(1) << 8) | (offset(2) << 16));
            set_number(entity, "Status", SITE_BLOCKED);
            set_number(entity, "Wait", RECHECK);
            return;
        }
        cursor += 1;
    }
    set_number(entity, "Survey", cursor as i32);
    set_number(entity, "Status", SITE_PLAN);
    if cursor >= blueprint.cells.len() {
        set_phase(entity, BUILDING);
        set_number(entity, "Cursor", 0);
        set_number(entity, "Utilization", 0);
        set_number(entity, "Status", if worker(entity).is_some() { SITE_BUILDING } else { NO_WORKER });
    }
}

/// O próximo bloco que ainda não está como na planta, a partir de `Cursor`. Lê no máximo
/// `SURVEY` blocos; o que já está certo é pulado.
fn next_cell(host: &dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity) -> Option<usize> {
    let blueprint = plan(entity);
    let rot = rot(entity);
    let mut cursor = number(entity, "Cursor") as usize;
    for _ in 0..SURVEY {
        let &cell = blueprint.cells.get(cursor)?;
        let (spot, index) = blueprints::decode(cell);
        let at = blueprints::world(blueprint, pos, rot, spot);
        if at == pos || (host.block(at) == blueprints::PALETTE[index] && blueprints::PROPS[index].is_empty()) {
            cursor += 1;
            continue;
        }
        set_number(entity, "Cursor", cursor as i32);
        return Some(cursor);
    }
    set_number(entity, "Cursor", cursor as i32);
    (cursor < blueprint.cells.len()).then_some(cursor)
}

/// Tarefa do mestre de obras: ir até a próxima parte do prédio.
pub(crate) fn plan_task(host: &mut dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity, now: u64) -> Result<Plan, i32> {
    let blueprint = plan(entity);
    if phase(entity) == DONE {
        return staff::plan(host, pos, entity, blueprint, rot(entity), now);
    }
    let Some(index) = next_cell(host, pos, entity) else {
        if number(entity, "Cursor") as usize >= blueprint.cells.len() {
            finish(host, pos, entity);
            return Err(SITE_DONE);
        }
        return Err(KEEP);
    };
    let (spot, _) = blueprints::decode(blueprint.cells[index]);
    let at = blueprints::world(blueprint, pos, rot(entity), spot);
    let ground = [at[0], pos[1], at[2]];
    // Parte que ele não alcança andando (dentro de paredes já erguidas): trabalha do canteiro.
    if skipped(entity).contains(&ground) {
        return Ok(Plan::Visit { spot: pos, reach: BUILD_REACH });
    }
    set_target(entity, Some(ground));
    Ok(Plan::Visit { spot: ground, reach: BUILD_REACH })
}

/// O mestre de obras chegou: assenta até `BATCH` blocos e conta uma tarefa (gasta comida).
pub(crate) fn perform(host: &mut dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity) -> Option<Vec<(String, u32)>> {
    if phase(entity) == DONE {
        return staff::perform(host, pos, entity, plan(entity));
    }
    if number(entity, "Footing") == 0 {
        footing(host, pos, entity);
        set_number(entity, "Footing", 1);
    }
    let blueprint = plan(entity);
    let rot = rot(entity);
    let mut cursor = number(entity, "Cursor") as usize;
    let mut palette: Vec<String> = Vec::new();
    let mut runs = Vec::new();
    let mut props: Vec<([i32; 3], usize)> = Vec::new();
    let mut reads = 0;
    while runs.len() < BATCH && reads < SURVEY {
        let Some(&cell) = blueprint.cells.get(cursor) else { break };
        cursor += 1;
        reads += 1;
        let (spot, index) = blueprints::decode(cell);
        let at = blueprints::world(blueprint, pos, rot, spot);
        let block = blueprints::PALETTE[index];
        if at == pos || (host.block(at) == block && blueprints::PROPS[index].is_empty()) {
            continue;
        }
        let slot = palette.iter().position(|known| known == block).unwrap_or_else(|| {
            palette.push(block.to_owned());
            palette.len() - 1
        });
        runs.push(NativeBlockRun { x: at[0], y: at[1], z: at[2], height: 1, surface: None, block: slot as u32, only: None });
        if !blueprints::PROPS[index].is_empty() {
            props.push((at, index));
        }
    }
    if !runs.is_empty() {
        let placed = runs.len() as i32;
        if host.edit_blocks(host.dimension(), &NativeBlockEdit { palette, filters: Vec::new(), runs }).is_err() {
            return None;
        }
        for (at, index) in props {
            for (key, value) in blueprints::PROPS[index] {
                host.set_block_property(at, key, &blueprints::turned(key, value, rot));
            }
        }
        set_number(entity, "Total", number(entity, "Total").saturating_add(placed));
        let spot = center(pos);
        host.play_sound("minecraft:block.wood.place", spot, 4, 0.5, 0.9);
    }
    set_number(entity, "Cursor", cursor as i32);
    publish_progress(entity);
    Some(Vec::new())
}

/// Completa o chão embaixo do piso: pedregulho onde houver ar, água ou plantas, até 3 blocos.
fn footing(host: &mut dyn Host, pos: [i32; 3], entity: &ModBlockEntity) {
    let blueprint = plan(entity);
    let rot = rot(entity);
    let runs: Vec<NativeBlockRun> = blueprint.cells.iter()
        .map(|cell| blueprints::decode(*cell))
        .filter(|(spot, index)| spot[1] == 0 && *index != 0)
        .map(|(spot, _)| {
            let at = blueprints::world(blueprint, pos, rot, spot);
            NativeBlockRun { x: at[0], y: at[1] - 3, z: at[2], height: 3, surface: None, block: 0, only: Some(0) }
        })
        .collect();
    if runs.is_empty() {
        return;
    }
    let _ = host.edit_blocks(host.dimension(), &NativeBlockEdit {
        palette: vec!["minecraft:cobblestone".to_owned()],
        filters: vec!["#minecraft:replaceable".to_owned()],
        runs,
    });
}

/// Prédio pronto: o mestre é dispensado e a vila fica sabendo.
fn finish(host: &mut dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity) {
    let dimension = host.dimension();
    set_phase(entity, DONE);
    set_target(entity, None);
    dismiss(dimension, pos, entity);
    set_number(entity, "Status", SITE_DONE);
    let key = plan(entity).key;
    let bit = blueprints::project_bit(plan(entity));
    publish(Building { dimension: dimension.to_owned(), pos, bit, standing: true, operating: false, research: 0, seen: defense::clock() });
    let at = center(pos);
    host.play_sound("minecraft:ui.toast.challenge_complete", at, 4, 0.9, 1.0);
    host.send_particles(&ParticleOptions::simple(ParticleType::HappyVillager), None, [at[0], at[1] + 2.0, at[2]], [3.0, 2.0, 3.0], 0.0, 60);
    let line = Text::tr("veio.site.done", &[&Text::tr(&format!("veio.project.{key}.name"), &[]).plain(),
        &Text::tr(&format!("veio.project.{key}.effect"), &[]).plain()]);
    let players: Vec<u128> = host.server_players().into_iter()
        .filter(|player| player.dimension == dimension && center::distance2(player.position, at) < 64.0 * 64.0)
        .map(|player| player.uuid).collect();
    let role = plan(entity).role;
    let next = Text::tr(&format!("veio.site.next.{role}"), &[]);
    for player in players {
        host.tell(player, line.clone());
        host.tell(player, next.clone());
    }
}

/// Lugares reservados ainda vazios: conta e marca com partículas.
fn reserved(host: &mut dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity, now: u64) {
    let blueprint = plan(entity);
    if blueprint.reserved.is_empty() {
        set_number(entity, "Utilization", 0);
        return;
    }
    let rot = rot(entity);
    let empty: Vec<[i32; 3]> = blueprint.reserved.iter()
        .map(|spot| blueprints::world(blueprint, pos, rot, *spot))
        .filter(|at| host.block(*at) != blueprint.reserved_block)
        .collect();
    set_number(entity, "Utilization", empty.len() as i32);
    if (now / 10) % 4 == 0 {
        for at in empty {
            host.send_particles(&ParticleOptions::simple(ParticleType::HappyVillager), None,
                [f64::from(at[0]) + 0.5, f64::from(at[1]) + 0.5, f64::from(at[2]) + 0.5], [0.2, 0.3, 0.2], 0.0, 4);
        }
    }
}

// ---------- Painel ----------

/// Nome da planta escolhida (o índice vem do painel).
pub(crate) fn plan_name(index: i32) -> String {
    let blueprint = &blueprints::BLUEPRINTS[index.max(0) as usize % blueprints::BLUEPRINTS.len()];
    Text::tr(&format!("veio.project.{}.name", blueprint.key), &[]).plain()
}

pub(crate) fn panel_lines(data: &[i32]) -> Vec<String> {
    let read = |index: usize| data.get(index).copied().unwrap_or(0);
    let tr = |key: &str, args: &[&dyn std::fmt::Display]| Text::tr(key, args).plain();
    let blueprint = &blueprints::BLUEPRINTS[read(6).max(0) as usize % blueprints::BLUEPRINTS.len()];
    let name = tr(&format!("veio.project.{}.name", blueprint.key), &[]);
    let [w, h, d] = blueprint.size;
    let first = tr("veio.site.plan", &[&name, &w, &d, &h]);
    let (progress, total) = (read(1), read(2).max(1));
    let second = match (read(8), read(0)) {
        (DONE, _) if read(7) > 0 => tr("veio.site.reserved", &[&read(7), &tr(&format!("block.{}", blueprint.reserved_block.replace(':', ".")), &[])]),
        (DONE, _) => tr("veio.site.standing", &[]),
        (BUILDING, _) => tr("veio.site.progress", &[&(progress * 100 / total), &read(9)]),
        (_, SITE_BLOCKED) => {
            let value = read(7);
            tr("veio.site.blocker", &[&((value & 255) - 128), &(((value >> 8) & 255) - 128), &(((value >> 16) & 255) - 128)])
        }
        (SURVEYING, _) => tr("veio.site.surveying", &[]),
        _ => tr(&format!("veio.site.wait.{}", read(7).clamp(0, 4)), &[]),
    };
    let third = if read(8) == BUILDING { tr("veio.metric.ration", &[&read(12)]) } else { tr("veio.hint.site", &[]) };
    vec![first, second, third]
}

// ---------- Desenho: a borda da planta no chão ----------

const LINE: [Option<CubeFace<'static>>; 6] = [Some(CubeFace { sprite: "veio:gui/material", uv: None }); 6];

pub(crate) fn draw(host: &dyn Host, event: &mut BlockEntitySubmitEvent<'_>) -> bool {
    if event.block != SITE {
        return false;
    }
    if phase(event.entity) >= BUILDING {
        return false;
    }
    let blueprint = plan(event.entity);
    let (low, high) = blueprints::bounds(blueprint, [0, 0, 0], rot(event.entity));
    let (x0, z0) = (low[0] as f32 * 16.0, low[2] as f32 * 16.0);
    let (x1, z1) = ((high[0] + 1) as f32 * 16.0, (high[2] + 1) as f32 * 16.0);
    let color = if number(event.entity, "Status") == SITE_BLOCKED { 0xc0392b } else { 0x2f80d0 };
    let y = [0.3, 1.1];
    for (from, to) in [
        ([x0, y[0], z0], [x1, y[1], z0 + 0.8]),
        ([x0, y[0], z1 - 0.8], [x1, y[1], z1]),
        ([x0, y[0], z0], [x0 + 0.8, y[1], z1]),
        ([x1 - 0.8, y[0], z0], [x1, y[1], z1]),
    ] {
        host.draw_cube(event.draw, from, to, LINE, color);
    }
    // Seta da entrada: do canteiro até a borda da frente.
    let (dx, dz) = blueprints::turn(0, 1, rot(event.entity));
    let reach = (blueprint.size[2] - blueprint.site[2]) as f32 * 16.0;
    let (ex, ez) = (8.0 + dx as f32 * reach, 8.0 + dz as f32 * reach);
    let (ax, az) = (ex.min(8.0) - 0.6, ez.min(8.0) - 0.6);
    let (bx, bz) = (ex.max(8.0) + 0.6, ez.max(8.0) + 0.6);
    host.draw_cube(event.draw, [ax, 0.3, az], [bx, 1.3, bz], LINE, 0xf1c40f);
    event.draw.keep();
    true
}

/// Área de desenho: cobre a planta inteira.
pub(crate) fn bounds(entity: &ModBlockEntity, bounds: &mut [f32; 6]) {
    let (low, high) = blueprints::bounds(plan(entity), [0, 0, 0], rot(entity));
    bounds[0] = bounds[0].min(low[0] as f32 - 0.2);
    bounds[2] = bounds[2].min(low[2] as f32 - 0.2);
    bounds[3] = bounds[3].max(high[0] as f32 + 1.2);
    bounds[5] = bounds[5].max(high[2] as f32 + 1.2);
}
