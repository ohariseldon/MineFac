//! Laboratório de pesquisa: a progressão pede produção, não um item só.
//!
//! Cada pesquisa pede centenas de peças entregues no laboratório, que as consome devagar com
//! energia. Concluída, ela concede um avanço (`veio:research/<id>`) que libera as receitas do
//! ramo seguinte. O progresso é do mundo, não do bloco: vários laboratórios somam a mesma
//! pesquisa, e quebrar um não perde nada. Fica em `<mundo>/veio/pesquisa.txt`.
//!
//! Depois das quatro, o laboratório entra na **pesquisa contínua**, como a pesquisa infinita
//! do Factorio: quatro linhas sem fim, cada nível mais caro que o anterior e com um bônus
//! permanente para a fábrica inteira. A peça posta na primeira entrada escolhe a linha. É o
//! destino das engrenagens, molduras, circuitos e processadores quando a árvore acaba: quanto
//! maior o fluxo, mais rápido a fábrica toda melhora.
use super::*;
use std::path::PathBuf;
use std::sync::Mutex;
use std::sync::atomic::{AtomicU32, Ordering};

pub(super) const LAB: &str = "veio:research_lab";
/// Energia por tick enquanto consome.
pub(super) const POWER: i32 = 24;
/// Um item de cada entrada a cada tantos ticks: 2 por segundo por entrada.
const EVERY: i64 = 10;

pub(super) struct Research {
    pub id: &'static str,
    pub inputs: [(&'static str, u32); 2],
}

pub(super) const RESEARCH: [Research; 4] = [
    Research { id: "metallurgy", inputs: [(STEEL, 128), (GEAR, 128)] },
    Research { id: "electronics", inputs: [(CIRCUIT, 128), (FRAME, 128)] },
    Research { id: "automation", inputs: [("veio:motor", 64), ("veio:processor", 64)] },
    Research { id: "scale", inputs: [("veio:automation_core", 32), (STEEL, 256)] },
];

/// Uma linha da pesquisa contínua: o pacote do primeiro nível. O nível `n` (de 0) pede
/// `n + 1` vezes isso, então a procura cresce junto com a fábrica.
pub(super) struct Line {
    pub id: &'static str,
    pub inputs: [(&'static str, u32); 2],
}

/// Usinagem: máquinas de receita mais rápidas. Perfuração: extratoras mais rápidas.
/// Rede: menos energia por tick em todas. Precisão: produto extra em todas.
pub(super) const LINES: [Line; 4] = [
    Line { id: "machining", inputs: [(GEAR, 256), (STEEL, 64)] },
    Line { id: "drilling", inputs: [(FRAME, 128), ("veio:motor", 16)] },
    Line { id: "grid", inputs: [(CIRCUIT, 128), ("veio:wire", 256)] },
    Line { id: "precision", inputs: [("veio:processor", 32), (MODULE, 64)] },
];
pub(super) const MACHINING: usize = 0;
pub(super) const DRILLING: usize = 1;
pub(super) const GRID: usize = 2;
pub(super) const PRECISION: usize = 3;
/// Por nível: 5% mais ritmo (Usinagem, Perfuração) ou 5% menos energia por produto (Rede),
/// com retorno decrescente: o fator é `100 / (100 + 5 × nível)` e nunca chega a zero.
pub(super) const STEP: u32 = 5;
/// Precisão: 2% de produto extra por nível, até 50%.
pub(super) const PRECISION_STEP: i32 = 2;
pub(super) const PRECISION_CAP: i32 = 50;

#[derive(Default)]
struct State {
    world: Option<PathBuf>,
    checked: i64,
    stage: usize,
    delivered: [u32; 2],
    /// Pesquisa contínua: nível e entregas de cada linha.
    levels: [u32; 4],
    line_delivered: [[u32; 2]; 4],
    dirty: bool,
}

static STATE: Mutex<State> = Mutex::new(State {
    world: None, checked: i64::MIN, stage: 0, delivered: [0; 2], levels: [0; 4], line_delivered: [[0; 2]; 4], dirty: false,
});

/// Níveis lidos pelas máquinas todo tick, sem trancar o estado.
static LEVELS: [AtomicU32; 4] = [AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0)];

fn publish_levels(levels: [u32; 4]) {
    for (slot, level) in LEVELS.iter().zip(levels) { slot.store(level, Ordering::Relaxed); }
}

pub(super) fn level(line: usize) -> u32 {
    LEVELS.get(line).map_or(0, |level| level.load(Ordering::Relaxed))
}

/// O pacote de um nível: `n + 1` vezes o primeiro.
pub(super) fn cost(line: usize, level: u32) -> [u32; 2] {
    let Some(line) = LINES.get(line) else { return [0; 2] };
    line.inputs.map(|(_, base)| base.saturating_mul(level.saturating_add(1)))
}

/// Fator de tempo (ou de energia) de um nível: 1, 0,952, 0,909... 0,5 no nível 20.
pub(super) fn factor(level: u32) -> f32 {
    100.0 / (100.0 + (STEP * level.min(10_000)) as f32)
}

/// Produto extra da Precisão, em pontos percentuais.
pub(super) fn precision_bonus(level: u32) -> i32 {
    (level.min(1_000) as i32 * PRECISION_STEP).min(PRECISION_CAP)
}

/// Os bônus da pesquisa contínua sobre o efeito de uma máquina. `extractor` escolhe entre
/// Usinagem (receitas) e Perfuração (extratoras).
pub(super) fn boost(effect: upgrades::Effect, extractor: bool) -> upgrades::Effect {
    boost_with(effect, extractor, [level(0), level(1), level(2), level(3)])
}

pub(super) fn boost_with(mut effect: upgrades::Effect, extractor: bool, levels: [u32; 4]) -> upgrades::Effect {
    if levels == [0; 4] { return effect; }
    effect.time *= factor(levels[if extractor { DRILLING } else { MACHINING }]);
    effect.power *= factor(levels[GRID]);
    effect.bonus += precision_bonus(levels[PRECISION]);
    effect
}

/// Ritmo a mais, em %, que um nível dá: 5, 10, 15... (o fator é o inverso).
pub(super) fn speed_percent(level: u32) -> u32 { STEP * level }

/// A linha escolhida pela peça da primeira entrada (ou, vazia, da segunda).
pub(super) fn line_for(slots: &[Option<ItemStack>]) -> Option<usize> {
    for index in 0..2 {
        let Some(stack) = slots.get(index).and_then(Option::as_ref) else { continue };
        if let Some(line) = LINES.iter().position(|line| is_plain(stack, line.inputs[index].0)) {
            return Some(line);
        }
    }
    None
}

/// No painel (slot 8), a etapa da árvore; na pesquisa contínua, `4 + linha + 4 × nível`.
pub(super) fn pack_stage(line: usize, level: u32) -> i32 {
    (RESEARCH.len() as u32 + line as u32 + LINES.len() as u32 * level.min(100_000)) as i32
}

/// `(linha, nível)` quando o número do painel é da pesquisa contínua.
pub(super) fn unpack_stage(value: i32) -> Option<(usize, u32)> {
    let rest = (value as u32).checked_sub(RESEARCH.len() as u32).filter(|_| value >= 0)?;
    Some(((rest % LINES.len() as u32) as usize, rest / LINES.len() as u32))
}

fn with_state<T>(change: impl FnOnce(&mut State) -> T) -> T {
    let mut state = STATE.lock().unwrap_or_else(|err| err.into_inner());
    change(&mut state)
}

fn file(world: &std::path::Path) -> PathBuf { world.join("veio").join("pesquisa.txt") }

/// O que fica salvo em `pesquisa.txt`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) struct Saved {
    pub stage: usize,
    pub delivered: [u32; 2],
    pub levels: [u32; 4],
    pub line_delivered: [[u32; 2]; 4],
}

/// Texto salvo: `estágio entregue0 entregue1`, e depois, para cada linha contínua,
/// `nível entregue0 entregue1`. Arquivos antigos (só os três primeiros) continuam valendo.
pub(super) fn encode(saved: &Saved) -> String {
    let mut text = format!("{} {} {}", saved.stage, saved.delivered[0], saved.delivered[1]);
    for (level, got) in saved.levels.iter().zip(saved.line_delivered) {
        text.push_str(&format!(" {level} {} {}", got[0], got[1]));
    }
    text.push('\n');
    text
}

pub(super) fn decode(text: &str) -> Saved {
    let mut numbers = text.split_whitespace().map(|word| word.parse::<u32>().unwrap_or(0));
    let mut next = || numbers.next().unwrap_or(0);
    let stage = (next() as usize).min(RESEARCH.len());
    let delivered = [next(), next()];
    let mut saved = Saved { stage, delivered, ..Saved::default() };
    for line in 0..LINES.len() {
        saved.levels[line] = next();
        saved.line_delivered[line] = [next(), next()];
    }
    saved
}

fn save(state: &mut State) {
    if !state.dirty { return; }
    let Some(world) = &state.world else { return; };
    let path = file(world);
    if let Some(parent) = path.parent() { let _ = std::fs::create_dir_all(parent); }
    let saved = Saved { stage: state.stage, delivered: state.delivered, levels: state.levels, line_delivered: state.line_delivered };
    if std::fs::write(&path, encode(&saved)).is_ok() {
        state.dirty = false;
    }
}

/// Troca de mundo: salva o anterior e lê o novo. Conferido a cada 100 ticks.
fn sync_world(host: &dyn Host, time: i64) {
    with_state(|state| {
        if state.world.is_some() && time - state.checked < 100 { return; }
        state.checked = time;
        let world = host.world_dir();
        if world.is_none() || world == state.world { return; }
        save(state);
        let saved = world.as_ref()
            .and_then(|world| std::fs::read_to_string(file(world)).ok())
            .map_or_else(Saved::default, |text| decode(&text));
        publish_levels(saved.levels);
        *state = State {
            world, checked: time, stage: saved.stage, delivered: saved.delivered,
            levels: saved.levels, line_delivered: saved.line_delivered, dirty: false,
        };
    });
}

pub(super) fn current() -> Option<&'static Research> {
    RESEARCH.get(with_state(|state| state.stage))
}

pub(super) fn accepts(slot: usize, stack: &ItemStack) -> bool {
    if slot >= 2 { return false; }
    match current() {
        Some(research) => is_plain(stack, research.inputs[slot].0),
        // Pesquisa contínua: cada entrada aceita a peça dela em qualquer linha.
        None => LINES.iter().any(|line| is_plain(stack, line.inputs[slot].0)),
    }
}

/// Entrega um item por entrada, se houver e faltar. Devolve quais entradas receberam.
/// Separado do tick para os testes.
pub(super) fn deliver(stage: usize, delivered: &mut [u32; 2], slots: &mut [Option<ItemStack>]) -> [bool; 2] {
    let Some(research) = RESEARCH.get(stage) else { return [false; 2] };
    deliver_pack(research.inputs, delivered, slots)
}

/// O mesmo para um pacote qualquer (a árvore ou um nível da pesquisa contínua).
pub(super) fn deliver_pack(pack: [(&'static str, u32); 2], delivered: &mut [u32; 2], slots: &mut [Option<ItemStack>]) -> [bool; 2] {
    let mut took = [false; 2];
    for (index, &(item, need)) in pack.iter().enumerate() {
        let Some(slot) = slots.get_mut(index) else { continue };
        if delivered[index] < need && slot.as_ref().is_some_and(|stack| is_plain(stack, item)) {
            consume(slot, 1);
            delivered[index] += 1;
            took[index] = true;
        }
    }
    took
}

pub(super) fn done(stage: usize, delivered: [u32; 2]) -> bool {
    RESEARCH.get(stage).is_some_and(|research| {
        research.inputs.iter().zip(delivered).all(|(&(_, need), got)| got >= need)
    })
}

pub(super) fn tick(host: &mut dyn Host, entity: &mut ModBlockEntity, time: i64) {
    // Só no servidor: a janela do cliente não concede avanços nem soma entregas.
    if host.surface_heights(host.dimension(), &[], horizon_client::mod_api::NativeHeightmap::MotionBlocking).is_err() {
        return;
    }
    sync_world(host, time);
    let (stage, delivered) = with_state(|state| (state.stage, state.delivered));
    set_number(entity, "ContractStage", stage as i32);
    set_number(entity, "Total", delivered[0] as i32);
    set_number(entity, "Info", delivered[1] as i32);
    // Barra do painel: tudo o que já foi entregue sobre o total da pesquisa.
    let goal = RESEARCH.get(stage).map_or(1, |research| research.inputs.iter().map(|&(_, need)| need).sum::<u32>());
    set_number(entity, "Progress", (delivered[0] + delivered[1]) as i32);
    set_number(entity, "Duration", goal as i32);
    if stage >= RESEARCH.len() {
        tick_continuous(host, entity, time);
        return;
    }
    let energy = number(entity, "energy");
    if energy < POWER {
        set_number(entity, "Status", 5);
        return;
    }
    let ready = entity.items.iter().take(2).enumerate().any(|(index, slot)| {
        slot.as_ref().is_some_and(|stack| is_plain(stack, RESEARCH[stage].inputs[index].0))
            && delivered[index] < RESEARCH[stage].inputs[index].1
    });
    if !ready {
        set_number(entity, "Status", 2);
        return;
    }
    set_number(entity, "energy", energy - POWER);
    set_number(entity, "Status", 1);
    if time.rem_euclid(EVERY) != 0 { return; }
    let finished = with_state(|state| {
        if state.stage != stage { return None; }
        let took = deliver(stage, &mut state.delivered, &mut entity.items);
        if took.iter().any(|&took| took) {
            state.dirty = true;
            set_number(entity, "Produced", number(entity, "Produced") + took.iter().filter(|&&t| t).count() as i32);
        }
        if !done(stage, state.delivered) {
            if time.rem_euclid(600) == 0 { save(state); }
            return None;
        }
        state.stage += 1;
        state.delivered = [0, 0];
        state.dirty = true;
        save(state);
        Some(RESEARCH[stage].id)
    });
    if let Some(id) = finished {
        host.run_command(&format!("advancement grant @a only veio:research/{id}"));
                for player in host.server_players() {
            host.tell(player.uuid, horizon_client::mod_api::Text::tr("veio.research.done",
                &[&horizon_client::mod_api::Text::tr(&format!("veio.research.{id}"), &[]).plain()]));
        }
    }
}

/// O pacote de um nível de uma linha.
pub(super) fn line_pack(line: usize, level: u32) -> [(&'static str, u32); 2] {
    let need = cost(line, level);
    [(LINES[line].inputs[0].0, need[0]), (LINES[line].inputs[1].0, need[1])]
}

/// Pesquisa contínua: a peça da entrada escolhe a linha; sem peça, fica a última do bloco.
fn tick_continuous(host: &mut dyn Host, entity: &mut ModBlockEntity, time: i64) {
    let remembered = unpack_stage(number(entity, "ContractStage")).map_or(MACHINING, |(line, _)| line);
    let line = line_for(&entity.items).unwrap_or(remembered);
    let (level, delivered) = with_state(|state| (state.levels[line], state.line_delivered[line]));
    let pack = line_pack(line, level);
    set_number(entity, "ContractStage", pack_stage(line, level));
    set_number(entity, "Total", delivered[0] as i32);
    set_number(entity, "Info", delivered[1] as i32);
    set_number(entity, "Progress", (delivered[0] + delivered[1]).min(i32::MAX as u32) as i32);
    set_number(entity, "Duration", (pack[0].1 + pack[1].1).max(1).min(i32::MAX as u32) as i32);
    let energy = number(entity, "energy");
    if energy < POWER {
        set_number(entity, "Status", 5);
        return;
    }
    let ready = entity.items.iter().take(2).enumerate().any(|(index, slot)| {
        slot.as_ref().is_some_and(|stack| is_plain(stack, pack[index].0)) && delivered[index] < pack[index].1
    });
    if !ready {
        set_number(entity, "Status", 2);
        return;
    }
    set_number(entity, "energy", energy - POWER);
    set_number(entity, "Status", 1);
    if time.rem_euclid(EVERY) != 0 { return; }
    let reached = with_state(|state| {
        if state.stage < RESEARCH.len() || state.levels[line] != level { return None; }
        let took = deliver_pack(pack, &mut state.line_delivered[line], &mut entity.items);
        if took.iter().any(|&took| took) {
            state.dirty = true;
            set_number(entity, "Produced", number(entity, "Produced") + took.iter().filter(|&&t| t).count() as i32);
        }
        let got = state.line_delivered[line];
        if pack.iter().zip(got).any(|(&(_, need), got)| got < need) {
            if time.rem_euclid(600) == 0 { save(state); }
            return None;
        }
        state.levels[line] += 1;
        state.line_delivered[line] = [0, 0];
        state.dirty = true;
        publish_levels(state.levels);
        save(state);
        Some(state.levels[line])
    });
    if let Some(level) = reached {
        use horizon_client::mod_api::Text;
        let name = Text::tr(&format!("veio.research.line.{}", LINES[line].id), &[]).plain();
        let effect = effect_text(line, level);
        for player in host.server_players() {
            host.tell(player.uuid, Text::tr("veio.research.level_up", &[&name, &level, &effect]));
        }
    }
}

/// O bônus total de uma linha num nível, em texto ("+15% de ritmo nas máquinas").
pub(super) fn effect_text(line: usize, level: u32) -> String {
    use horizon_client::mod_api::Text;
    let id = LINES.get(line).map_or("machining", |line| line.id);
    let value = if line == PRECISION { precision_bonus(level) as u32 } else { speed_percent(level) };
    Text::tr(&format!("veio.research.line.{id}.effect"), &[&value]).plain()
}

/// Linhas do painel: pesquisa atual, o que falta de cada entrada e o que ela libera. A barra
/// de progresso do painel comum mostra o total entregue.
pub(super) fn panel_lines(data: &[i32]) -> Vec<String> {
    use horizon_client::mod_api::Text;
    let read = |index: usize| data.get(index).copied().unwrap_or(0);
    let stage = read(8).max(0) as usize;
    let Some(research) = RESEARCH.get(stage) else {
        let (line, level) = unpack_stage(read(8)).unwrap_or((MACHINING, 0));
        let name = Text::tr(&format!("veio.research.line.{}", LINES[line].id), &[]).plain();
        let need = line_pack(line, level).iter().enumerate().map(|(index, &(item, need))| {
            format!("{}/{need} {}", read(if index == 0 { 9 } else { 12 }).max(0), contracts::item_name(item))
        }).collect::<Vec<_>>().join(" · ");
        return vec![
            Text::tr("veio.research.line.title", &[&name, &(level + 1)]).plain(),
            need,
            Text::tr("veio.research.line.now", &[&effect_text(line, level), &effect_text(line, level + 1)]).plain(),
            Text::tr("veio.research.line.switch", &[]).plain(),
        ];
    };
    let name = Text::tr(&format!("veio.research.{}", research.id), &[]).plain();
    let title = Text::tr("veio.research.title", &[&(stage + 1), &RESEARCH.len(), &name]).plain();
    let need = research.inputs.iter().enumerate().map(|(index, &(item, need))| {
        format!("{}/{need} {}", read(if index == 0 { 9 } else { 12 }).max(0), contracts::item_name(item))
    }).collect::<Vec<_>>().join(" · ");
    vec![title, need, Text::tr(&format!("veio.research.{}.unlocks", research.id), &[]).plain()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn research_consumes_one_of_each_until_the_goal_and_saves_as_text() {
        let mut delivered = [0, 0];
        let mut slots = vec![Some(ItemStack::new(STEEL, 64)), Some(ItemStack::new(GEAR, 2))];
        assert_eq!(deliver(0, &mut delivered, &mut slots), [true, true]);
        assert_eq!(deliver(0, &mut delivered, &mut slots), [true, true]);
        assert_eq!(deliver(0, &mut delivered, &mut slots), [true, false], "acabaram as engrenagens");
        assert_eq!(delivered, [3, 2]);
        assert!(slots[1].is_none());
        assert!(!done(0, delivered));
        let mut wrong = vec![Some(ItemStack::new(GEAR, 5)), None];
        assert_eq!(deliver(0, &mut delivered, &mut wrong), [false, false], "cada entrada tem seu item");
        let mut full = [128, 128];
        assert!(done(0, full));
        let mut more = vec![Some(ItemStack::new(STEEL, 1)), None];
        assert_eq!(deliver(0, &mut full, &mut more), [false, false], "não consome além da meta");
        let saved = Saved { stage: 2, delivered: [17, 40], ..Saved::default() };
        assert_eq!(decode(&encode(&saved)), saved);
        assert_eq!(decode("garbage"), Saved::default());
        assert_eq!(decode("99 1 1").stage, RESEARCH.len());
    }

    #[test]
    fn each_research_needs_the_previous_tier() {
        // Metalurgia antes de circuitos; o núcleo de automação só existe depois da fabricadora.
        let order: Vec<_> = RESEARCH.iter().map(|research| research.id).collect();
        assert_eq!(order, ["metallurgy", "electronics", "automation", "scale"]);
        assert!(RESEARCH.iter().all(|research| research.inputs.iter().all(|&(_, need)| need >= 32)));
    }

    #[test]
    fn continuous_research_grows_with_each_level_and_reads_old_saves() {
        // O nível n pede n + 1 vezes o pacote: a procura de engrenagens não acaba.
        assert_eq!(cost(MACHINING, 0), [256, 64]);
        assert_eq!(cost(MACHINING, 3), [1024, 256]);
        // Retorno decrescente: o fator nunca chega a zero.
        assert_eq!(factor(0), 1.0);
        assert!((factor(20) - 0.5).abs() < 1e-6);
        assert!(factor(1_000_000) > 0.0);
        assert_eq!(precision_bonus(3), 6);
        assert_eq!(precision_bonus(400), PRECISION_CAP);
        // Arquivo da 0.37 (só a árvore) abre com tudo zerado na contínua.
        let old = decode("4 0 0\n");
        assert_eq!((old.stage, old.levels, old.line_delivered), (4, [0; 4], [[0; 2]; 4]));
        let saved = Saved { stage: 4, delivered: [0, 0], levels: [3, 0, 7, 1], line_delivered: [[10, 2], [0, 0], [5, 5], [1, 0]] };
        assert_eq!(decode(&encode(&saved)), saved);
        // A etapa do painel guarda linha e nível juntos.
        for line in 0..LINES.len() {
            for level in [0, 1, 9, 250] {
                assert_eq!(unpack_stage(pack_stage(line, level)), Some((line, level)));
            }
        }
        assert_eq!(unpack_stage(2), None, "etapa da árvore");
    }

    #[test]
    fn the_part_in_the_lab_picks_the_line_and_each_line_has_its_own_parts() {
        let first: Vec<_> = LINES.iter().map(|line| line.inputs[0].0).collect();
        let second: Vec<_> = LINES.iter().map(|line| line.inputs[1].0).collect();
        for items in [&first, &second] {
            for (index, item) in items.iter().enumerate() {
                assert!(!items[index + 1..].contains(item), "{item} em duas linhas na mesma entrada");
            }
        }
        assert_eq!(line_for(&[Some(ItemStack::new(GEAR, 3)), None]), Some(MACHINING));
        assert_eq!(line_for(&[Some(ItemStack::new(CIRCUIT, 3)), Some(ItemStack::new(STEEL, 1))]), Some(GRID));
        assert_eq!(line_for(&[None, Some(ItemStack::new(MODULE, 1))]), Some(PRECISION));
        assert_eq!(line_for(&[None, None]), None);
        let mut delivered = [0, 0];
        let mut slots = vec![Some(ItemStack::new(FRAME, 2)), Some(ItemStack::new("veio:motor", 1))];
        assert_eq!(deliver_pack(line_pack(DRILLING, 0), &mut delivered, &mut slots), [true, true]);
        assert_eq!(deliver_pack(line_pack(DRILLING, 0), &mut delivered, &mut slots), [true, false]);
        assert_eq!(delivered, [2, 1]);
    }

    #[test]
    fn levels_speed_up_machines_and_save_power() {
        let base = upgrades::Effect { time: 1.0, power: 1.0, pollution: 1.0, bonus: 0 };
        assert_eq!(boost_with(base, false, [0; 4]), base);
        // Usinagem 20 dobra o ritmo das receitas e não mexe nas extratoras.
        let recipe = boost_with(base, false, [20, 0, 0, 0]);
        assert!((recipe.time - 0.5).abs() < 1e-6);
        assert_eq!(boost_with(base, true, [20, 0, 0, 0]).time, 1.0);
        assert!((boost_with(base, true, [0, 20, 0, 0]).time - 0.5).abs() < 1e-6);
        // Rede corta energia por tick; Precisão soma ao módulo de produtividade.
        let both = boost_with(upgrades::effect_of(3), false, [0, 0, 20, 5]);
        assert!((both.power - 0.7).abs() < 1e-5);
        assert_eq!(both.bonus, 30);
        assert_eq!(both.pollution, upgrades::effect_of(3).pollution, "a poluição não muda");
    }
}
