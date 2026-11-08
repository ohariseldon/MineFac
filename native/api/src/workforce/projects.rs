//! Obras da vila, prestígio e decretos: o centro de controle decide e constrói melhorias que
//! mudam a vila de verdade.
//!
//! **Obras.** O jogador põe os materiais no slot **Obra** do centro (`SITE`). Com o administrador
//! contratado, cada material vai primeiro para os pedidos do dia (`requests.rs`) e depois para a
//! primeira obra liberada que ainda precisa dele; a obra termina quando os três materiais estão
//! completos. Esmeraldas que sobram vão para o **caixa da companhia** (as moedas do centro de
//! controle, `society::ledger`), que também paga as esmeraldas das obras. Obras feitas valem para os postos, zonas e aldeões a até `center::RADIUS` blocos:
//!
//! | Obra | Prestígio | Materiais | Efeito |
//! | --- | --- | --- | --- |
//! | Refeitório | Aldeia | 24 tijolos, 8 chapas de ferro, 1 cozinha | Toda 4ª tarefa dos postos não gasta comida |
//! | Escola | Aldeia | 24 papéis, 6 livros, 16 tijolos | Natalidade 1/3 (em vez de 1/6) e teto +15 |
//! | Ferraria | Aldeia | 8 chapas de aço, 8 engrenagens, 1 bigorna | Carregadores levam 24 itens (recolhem 48) |
//! | Praça do mercado | Aldeia | 16 vidros, 16 chapas de cobre, 8 esmeraldas | Expandir zonas custa 25% menos |
//! | Festa da colheita | Aldeia | 1 bolo, 16 biscoitos, 16 pães | Repetível: hoje e amanhã tarefas sem comida, natalidade 1/2 |
//! | Biblioteca | Vila | 8 estantes, 16 livros, 4 lanternas | Zonas ganham o dobro de trabalho por tarefa |
//! | Correio | Vila | 16 papéis, 4 baús, 8 chapas de cobre | 4 pedidos por dia e +50% de esmeraldas |
//! | Estalagem | Vila próspera | 8 camas brancas, 32 tábuas de carvalho, 4 chapas de aço | Teto +20; um viajante chega toda manhã se houver lugar |
//! | Prefeitura | Cidade operária | 4 blocos de ouro, 1 sino, 8 circuitos | Prestígio +50% e o caixa rende (1 a cada 16, até 8 por dia) |
//! | Centro de pesquisa | Vila | 4 circuitos, 16 vidros, 8 chapas de aço | O cientista pesquisa tecnologias (`staff.rs`) e instala módulos |
//!
//! Cada efeito só vale com o prédio **de pé e funcionando** (`staff.rs`): funcionário contratado
//! e trabalhando, lojas da praça no lugar, bloco da prefeitura posto.
//!
//! **Prestígio.** Pontos no centro (`Prestige`) vindos de pedidos (+10), obras (+25), festas
//! (+10), banquetes no lar (+5), dias em ordem no lar (+2) e manhãs sem pendências (+3). Níveis:
//! Aldeia, Vila (60), Vila próspera (200) e Cidade operária (500). Cada nível soma 5 ao teto da
//! vila, libera obras e pedidos mais ricos.
//!
//! **Decretos.** Um item no slot **Decreto** do centro muda a regra da área (sem gastar o item):
//! - sino — Convocação: postos contratam aldeões a até 16 blocos (em vez de 8);
//! - papel — Racionamento: tarefas custam 1 ponto de comida, mas ninguém nasce;
//! - fardo de feno — Fartura: tarefas custam 3 pontos, as zonas ganham o dobro de trabalho e
//!   nasce mais gente (1/4).
//!
//! As obras ficam no NBT do centro (`Built`, `ObraI_J`, `FestivalDay`); o efeito é publicado num
//! registro em memória (`PERKS`) que os postos consultam uma vez por ciclo, sem varrer blocos.
use super::*;
use std::sync::atomic::{AtomicBool, Ordering};

pub(crate) const CANTEEN: u32 = 1;
pub(crate) const SCHOOL: u32 = 2;
pub(crate) const SMITHY: u32 = 4;
pub(crate) const MARKET: u32 = 8;
pub(crate) const FESTIVAL: u32 = 16;
pub(crate) const LIBRARY: u32 = 32;
pub(crate) const POST_OFFICE: u32 = 64;
pub(crate) const INN: u32 = 128;
pub(crate) const HALL: u32 = 256;
pub(crate) const LAB: u32 = 512;
/// Decretos.
pub(crate) const CALL: u32 = 1 << 10;
pub(crate) const RATION: u32 = 1 << 11;
pub(crate) const PLENTY: u32 = 1 << 12;
/// Nível de prestígio (2 bits).
const TIER_SHIFT: u32 = 16;
const TIER_MASK: u32 = 3 << TIER_SHIFT;
/// Pesquisas do centro de pesquisa (`staff::TECHS`, 5 bits).
const RESEARCH_SHIFT: u32 = 20;

/// Bit da pesquisa `tech` (`staff::AGRONOMY`...) nos efeitos da vila.
pub(crate) const fn research_bit(tech: u32) -> u32 {
    tech << RESEARCH_SHIFT
}

/// Slot de obra (materiais e entregas) no centro de controle.
pub(crate) const SITE: usize = 5;
/// Slot do decreto.
pub(crate) const DECREE: usize = 6;
/// Uma entrada do registro vence nesse tempo sem o centro republicar (a cada 10 ticks).
const STALE: u64 = 400;

/// Pontos para cada nível de prestígio.
pub(crate) const TIERS: [i32; 4] = [0, 60, 200, 500];
pub(crate) const MAX_TIER: i32 = 3;

pub(crate) struct Material {
    pub item: &'static str,
    pub count: u32,
    /// Chave de tradução do nome do item.
    pub name: &'static str,
}

pub(crate) struct Project {
    pub key: &'static str,
    pub bit: u32,
    pub repeat: bool,
    /// Prestígio necessário para começar.
    pub tier: i32,
    pub needs: [Material; 3],
}

const fn m(item: &'static str, count: u32, name: &'static str) -> Material {
    Material { item, count, name }
}

pub(crate) const PROJECTS: [Project; 10] = [
    Project { key: "canteen", bit: CANTEEN, repeat: false, tier: 0, needs: [
        m("minecraft:brick", 24, "item.minecraft.brick"),
        m("veio:iron_plate", 8, "item.veio.iron_plate"),
        m("veio:kitchen", 1, "block.veio.kitchen")] },
    Project { key: "school", bit: SCHOOL, repeat: false, tier: 0, needs: [
        m("minecraft:paper", 24, "item.minecraft.paper"),
        m("minecraft:book", 6, "item.minecraft.book"),
        m("minecraft:brick", 16, "item.minecraft.brick")] },
    Project { key: "smithy", bit: SMITHY, repeat: false, tier: 0, needs: [
        m("veio:steel_plate", 8, "item.veio.steel_plate"),
        m("veio:gear", 8, "item.veio.gear"),
        m("minecraft:anvil", 1, "block.minecraft.anvil")] },
    Project { key: "market", bit: MARKET, repeat: false, tier: 0, needs: [
        m("minecraft:glass", 16, "block.minecraft.glass"),
        m("veio:copper_plate", 16, "item.veio.copper_plate"),
        m("minecraft:emerald", 8, "item.minecraft.emerald")] },
    Project { key: "festival", bit: FESTIVAL, repeat: true, tier: 0, needs: [
        m("minecraft:cake", 1, "block.minecraft.cake"),
        m("minecraft:cookie", 16, "item.minecraft.cookie"),
        m("minecraft:bread", 16, "item.minecraft.bread")] },
    Project { key: "library", bit: LIBRARY, repeat: false, tier: 1, needs: [
        m("minecraft:bookshelf", 8, "block.minecraft.bookshelf"),
        m("minecraft:book", 16, "item.minecraft.book"),
        m("minecraft:lantern", 4, "block.minecraft.lantern")] },
    Project { key: "post_office", bit: POST_OFFICE, repeat: false, tier: 1, needs: [
        m("minecraft:paper", 16, "item.minecraft.paper"),
        m("minecraft:chest", 4, "block.minecraft.chest"),
        m("veio:copper_plate", 8, "item.veio.copper_plate")] },
    Project { key: "inn", bit: INN, repeat: false, tier: 2, needs: [
        m("minecraft:white_bed", 8, "block.minecraft.white_bed"),
        m("minecraft:oak_planks", 32, "block.minecraft.oak_planks"),
        m("veio:steel_plate", 4, "item.veio.steel_plate")] },
    Project { key: "hall", bit: HALL, repeat: false, tier: 3, needs: [
        m("minecraft:gold_block", 4, "block.minecraft.gold_block"),
        m("minecraft:bell", 1, "block.minecraft.bell"),
        m("veio:circuit", 8, "item.veio.circuit")] },
    Project { key: "lab", bit: LAB, repeat: false, tier: 1, needs: [
        m("veio:circuit", 4, "item.veio.circuit"),
        m("minecraft:glass", 16, "block.minecraft.glass"),
        m("veio:steel_plate", 8, "item.veio.steel_plate")] },
];

/// Algum projeto usa este item?
pub(crate) fn material(item: &str) -> bool {
    PROJECTS.iter().any(|project| project.needs.iter().any(|need| need.item == item))
}

/// Itens aceitos como decreto e o efeito de cada um.
pub(crate) fn decree_of(item: &str) -> Option<u32> {
    Some(match item {
        "minecraft:bell" => CALL,
        "minecraft:paper" => RATION,
        "minecraft:hay_block" => PLENTY,
        _ => return None,
    })
}

/// Índice do decreto para o painel (0 = nenhum).
pub(crate) fn decree_index(perks: u32) -> i32 {
    [CALL, RATION, PLENTY].iter().position(|bit| perks & bit != 0).map_or(0, |index| index as i32 + 1)
}

fn key(project: usize, need: usize) -> String {
    format!("Obra{project}_{need}")
}

/// Obras pagas (materiais entregues). O efeito só vale com o prédio de pé (`standing`).
pub(crate) fn built(entity: &ModBlockEntity) -> u32 {
    number(entity, "Built") as u32
}

/// Obras pagas e de pé num canteiro concluído perto do centro (o centro atualiza `Standing`).
pub(crate) fn standing(entity: &ModBlockEntity) -> u32 {
    built(entity) & number(entity, "Standing") as u32
}

pub(crate) fn progress(entity: &ModBlockEntity, project: usize, need: usize) -> u32 {
    number(entity, &key(project, need)) as u32
}

/// Porcentagem entregue da obra (0-100).
pub(crate) fn percent(entity: &ModBlockEntity, project: usize) -> i32 {
    let (have, total) = PROJECTS[project].needs.iter().enumerate().fold((0, 0), |(have, total), (index, need)| {
        (have + progress(entity, project, index).min(need.count), total + need.count)
    });
    (have * 100 / total.max(1)) as i32
}

/// Primeiro material que falta: (índice, quantidade).
pub(crate) fn missing(entity: &ModBlockEntity, project: usize) -> Option<(usize, u32)> {
    PROJECTS[project].needs.iter().enumerate()
        .map(|(index, need)| (index, need.count.saturating_sub(progress(entity, project, index))))
        .find(|(_, left)| *left > 0)
}

/// Dia da festa ativa até (contador de dias do centro, `Day`).
pub(crate) fn festival_until(entity: &ModBlockEntity) -> i32 {
    number(entity, "FestivalDay")
}

// ---------- Prestígio ----------

pub(crate) fn prestige(entity: &ModBlockEntity) -> i32 {
    number(entity, "Prestige")
}

pub(crate) fn tier_of(points: i32) -> i32 {
    TIERS.iter().rposition(|need| points >= *need).unwrap_or(0) as i32
}

pub(crate) fn tier(entity: &ModBlockEntity) -> i32 {
    tier_of(prestige(entity))
}

/// Soma prestígio (+50% com a prefeitura). Devolve `true` se a vila subiu de nível.
pub(crate) fn gain(entity: &mut ModBlockEntity, points: i32) -> bool {
    let before = tier(entity);
    let points = if standing(entity) & HALL != 0 { points + points / 2 } else { points };
    set_number(entity, "Prestige", prestige(entity).saturating_add(points));
    tier(entity) > before
}

pub(crate) fn tier_bits(perks: u32) -> i32 {
    ((perks & TIER_MASK) >> TIER_SHIFT) as i32
}

/// Efeitos ativos deste centro no dia `day`: obras, festa, decreto e nível de prestígio.
pub(crate) fn active(entity: &ModBlockEntity, day: i32) -> u32 {
    let festival = entity.nbt.contains_key(&"FestivalDay".into()) && festival_until(entity) >= day;
    let decree = entity.items.get(DECREE).and_then(Option::as_ref).and_then(|stack| decree_of(&stack.item)).unwrap_or(0);
    standing(entity) | if festival { FESTIVAL } else { 0 } | decree | ((tier(entity) as u32) << TIER_SHIFT)
        | research_bit(number(entity, "Research") as u32 & 31)
}

/// Leva o que está no slot de obra para as obras liberadas. Devolve as obras concluídas.
/// Nada é destruído: o que nenhuma obra precisa fica no slot (esmeraldas vão para o caixa).
pub(crate) fn deliver(entity: &mut ModBlockEntity, day: i32) -> Vec<usize> {
    let mut done = Vec::new();
    let Some(stack) = entity.items.get(SITE).and_then(Option::as_ref).filter(|stack| plain(stack)).cloned() else {
        return done;
    };
    let left = feed(entity, &stack.item, stack.count, day, &mut done);
    let used = stack.count - left;
    if used > 0 {
        consume(&mut entity.items[SITE], used);
    }
    done
}

/// Distribui `count` de `item` pelas obras liberadas. Devolve a sobra.
fn feed(entity: &mut ModBlockEntity, item: &str, count: u32, day: i32, done: &mut Vec<usize>) -> u32 {
    let mut left = count;
    let tier = tier(entity);
    for (index, project) in PROJECTS.iter().enumerate() {
        if left == 0 {
            break;
        }
        if (!project.repeat && built(entity) & project.bit != 0) || project.tier > tier {
            continue;
        }
        for (slot, need) in project.needs.iter().enumerate() {
            if need.item != item {
                continue;
            }
            let have = progress(entity, index, slot);
            let moved = need.count.saturating_sub(have).min(left);
            if moved > 0 {
                set_number(entity, &key(index, slot), (have + moved) as i32);
                left -= moved;
            }
        }
        if missing(entity, index).is_none() {
            complete(entity, index, day);
            done.push(index);
        }
    }
    left
}

/// O caixa paga as esmeraldas que as obras liberadas pedem, até `emeralds`. Devolve as obras
/// concluídas e quantas esmeraldas foram usadas.
pub(crate) fn pay_with(entity: &mut ModBlockEntity, day: i32, emeralds: u32) -> (Vec<usize>, u32) {
    let mut done = Vec::new();
    if emeralds == 0 {
        return (done, 0);
    }
    let left = feed(entity, WAGE, emeralds, day, &mut done);
    (done, emeralds - left)
}

fn complete(entity: &mut ModBlockEntity, project: usize, day: i32) {
    let info = &PROJECTS[project];
    if info.repeat {
        // Festa: vale hoje e amanhã; outra festa durante esta emenda mais um dia.
        let from = if active(entity, day) & FESTIVAL != 0 { festival_until(entity) } else { day };
        set_number(entity, "FestivalDay", from + 1);
        for slot in 0..info.needs.len() {
            set_number(entity, &key(project, slot), 0);
        }
        gain(entity, 10);
    } else {
        set_number(entity, "Built", (built(entity) | info.bit) as i32);
        gain(entity, 25);
    }
}

/// Dados do painel de obras, um valor por obra: porcentagem (7 bits), feita (bit 7), índice do
/// material que falta (2 bits, a partir do 8), quantidade que falta (8 bits, a partir do 10),
/// dias de festa restantes (4 bits, a partir do 18), bloqueada (bit 22) e prestígio exigido
/// (2 bits, a partir do 23).
pub(crate) fn packed(entity: &ModBlockEntity, project: usize, day: i32) -> i32 {
    let info = &PROJECTS[project];
    let done = !info.repeat && built(entity) & info.bit != 0;
    let (index, left) = missing(entity, project).unwrap_or((0, 0));
    let days = if info.repeat && active(entity, day) & FESTIVAL != 0 { (festival_until(entity) - day + 1).clamp(0, 15) } else { 0 };
    let locked = !done && info.tier > tier(entity);
    let up = standing(entity) & info.bit != 0;
    let erected = !up && built(entity) & number(entity, "Erected") as u32 & info.bit != 0;
    percent(entity, project).clamp(0, 100) | (i32::from(done) << 7) | ((index as i32 & 3) << 8)
        | ((left.min(255) as i32) << 10) | (days << 18) | (i32::from(locked) << 22) | (info.tier << 23)
        | (i32::from(up) << 25) | (i32::from(erected) << 26)
}

/// O prédio está de pé mas parado: falta funcionário, lojas ou a prefeitura (bit 26).
pub(crate) fn is_erected(value: i32) -> bool {
    value & (1 << 26) != 0
}

/// O prédio da obra está de pé (bit 25 de `packed`).
pub(crate) fn is_standing(value: i32) -> bool {
    value & (1 << 25) != 0
}

/// (porcentagem, feita, material que falta, quantidade, dias de festa, bloqueada, prestígio exigido)
pub(crate) fn unpack(value: i32) -> (i32, bool, usize, u32, i32, bool, i32) {
    (value & 127, value & 128 != 0, ((value >> 8) & 3) as usize, ((value >> 10) & 255) as u32, (value >> 18) & 15,
        value & (1 << 22) != 0, (value >> 23) & 3)
}

// ---------- Registro dos efeitos ----------

/// Um centro como os postos e o lar o veem.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Village {
    pub dimension: String,
    pub pos: [i32; 3],
    pub perks: u32,
    pub waiting: i32,
    pub villagers: i32,
    pub cap: i32,
    /// Obras pagas (prontas para um canteiro).
    pub funded: u32,
    /// Pedidos ainda abertos.
    pub requests: i32,
    pub seen: u64,
}

static PERKS: Mutex<Vec<Village>> = Mutex::new(Vec::new());
static PENDING: AtomicBool = AtomicBool::new(false);

/// Chamado pelo centro de controle com administrador contratado.
pub(crate) fn publish(village: Village) {
    if let Ok(mut list) = PERKS.lock() {
        let now = village.seen;
        if let Some(known) = list.iter_mut().find(|known| known.dimension == village.dimension && known.pos == village.pos) {
            *known = village;
        } else {
            list.retain(|known| now.saturating_sub(known.seen) <= STALE);
            list.push(village);
        }
        PENDING.store(true, Ordering::Release);
    }
}

/// Centro removido ou sem administrador: os efeitos param.
pub(crate) fn withdraw_village(dimension: &str, pos: [i32; 3]) {
    if let Ok(mut list) = PERKS.lock() {
        list.retain(|known| !(known.dimension == dimension && known.pos == pos));
    }
}

fn within(a: [i32; 3], b: [i32; 3], radius: i32) -> bool {
    (0..3).all(|axis| (a[axis] - b[axis]).abs() <= radius)
}

/// Efeitos das obras que valem em `at`. Quase sempre sai na primeira comparação. Com dois
/// centros em volta, as obras se somam e vale o maior prestígio.
pub(crate) fn at(dimension: &str, at: [i32; 3], now: u64) -> u32 {
    if !PENDING.load(Ordering::Acquire) {
        return 0;
    }
    PERKS.lock().map(|list| list.iter()
        .filter(|known| known.dimension == dimension && now.saturating_sub(known.seen) <= STALE
            && within(known.pos, at, center::RADIUS))
        .fold(0, |bits, known| {
            let tier = (bits & TIER_MASK).max(known.perks & TIER_MASK);
            (bits | known.perks) & !TIER_MASK | tier
        })).unwrap_or(0)
}

/// Centro vivo mais próximo de `at`, a até `radius` blocos.
pub(crate) fn nearest(dimension: &str, at: [i32; 3], radius: i32, now: u64) -> Option<Village> {
    if !PENDING.load(Ordering::Acquire) {
        return None;
    }
    let distance = |pos: [i32; 3]| (0..3).map(|axis| i64::from(pos[axis] - at[axis]).pow(2)).sum::<i64>();
    PERKS.lock().ok()?.iter()
        .filter(|known| known.dimension == dimension && now.saturating_sub(known.seen) <= STALE && within(known.pos, at, radius))
        .min_by_key(|known| distance(known.pos))
        .cloned()
}

// ---------- Efeitos ----------

/// Pontos de comida da tarefa que vai ser feita. Festa: nada; refeitório: toda 4ª é de graça;
/// racionamento: 1; fartura: 3; senão `TASK_COST`. `total` é o contador de tarefas do posto
/// antes desta.
pub(crate) fn food_cost(perks: u32, total: i32) -> i32 {
    let cost = base_food_cost(perks, total);
    if perks & research_bit(staff::NUTRITION) != 0 && cost > 1 { cost - 1 } else { cost }
}

fn base_food_cost(perks: u32, total: i32) -> i32 {
    if perks & FESTIVAL != 0 || (perks & CANTEEN != 0 && (total + 1) % 4 == 0) {
        0
    } else if perks & RATION != 0 {
        1
    } else if perks & PLENTY != 0 {
        TASK_COST + 1
    } else {
        TASK_COST
    }
}

/// Itens por viagem do carregador.
pub(crate) fn haul_batch(perks: u32) -> u32 {
    let batch = if perks & SMITHY != 0 { hauler::BATCH + hauler::BATCH / 2 } else { hauler::BATCH };
    if perks & research_bit(staff::CARGO) != 0 { batch + 8 } else { batch }
}

/// Chance de nascimento (1 em N) por aldeão adulto a cada verificação; `None`: ninguém nasce.
pub(crate) fn birth_chance(perks: u32) -> Option<u64> {
    if perks & RATION != 0 {
        None
    } else if perks & FESTIVAL != 0 {
        Some(2)
    } else if perks & SCHOOL != 0 {
        Some(3)
    } else if perks & PLENTY != 0 {
        Some(4)
    } else {
        Some(village::CHANCE)
    }
}

/// Teto extra de população: escola, estalagem e prestígio.
pub(crate) fn extra_cap(perks: u32) -> usize {
    (if perks & SCHOOL != 0 { 15 } else { 0 }) + (if perks & INN != 0 { 20 } else { 0 }) + 5 * tier_bits(perks) as usize
}

/// Esmeraldas para expandir uma zona: a praça do mercado tira 25% (arredondando para cima).
pub(crate) fn zone_cost(perks: u32, cost: u32) -> u32 {
    if perks & MARKET != 0 { cost - cost / 4 } else { cost }
}

/// Trabalho que uma tarefa conta na zona: biblioteca e fartura dobram (as duas, triplica).
pub(crate) fn zone_credit(perks: u32) -> i32 {
    1 + i32::from(perks & LIBRARY != 0) + i32::from(perks & PLENTY != 0) + i32::from(perks & research_bit(staff::AGRONOMY) != 0)
}

/// Raio de contratação dos postos: a convocação dobra.
pub(crate) fn hire_radius(perks: u32) -> f64 {
    if perks & CALL != 0 { HIRE_RADIUS * 2.0 } else { HIRE_RADIUS }
}

/// Juros do caixa com a prefeitura.
pub(crate) fn interest(perks: u32, treasury: u32) -> u32 {
    if perks & HALL != 0 { (treasury / 16).min(8) } else { 0 }
}

