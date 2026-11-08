//! Laboratório de pesquisa: a progressão pede produção, não um item só.
//!
//! Cada pesquisa pede centenas de peças entregues no laboratório, que as consome devagar com
//! energia. Concluída, ela concede um avanço (`veio:research/<id>`) que libera as receitas do
//! ramo seguinte. O progresso é do mundo, não do bloco: vários laboratórios somam a mesma
//! pesquisa, e quebrar um não perde nada. Fica em `<mundo>/veio/pesquisa.txt`.
use super::*;
use std::path::PathBuf;
use std::sync::Mutex;

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

#[derive(Default)]
struct State {
    world: Option<PathBuf>,
    checked: i64,
    stage: usize,
    delivered: [u32; 2],
    dirty: bool,
}

static STATE: Mutex<State> = Mutex::new(State { world: None, checked: i64::MIN, stage: 0, delivered: [0; 2], dirty: false });

fn with_state<T>(change: impl FnOnce(&mut State) -> T) -> T {
    let mut state = STATE.lock().unwrap_or_else(|err| err.into_inner());
    change(&mut state)
}

fn file(world: &std::path::Path) -> PathBuf { world.join("veio").join("pesquisa.txt") }

/// Texto salvo: `estágio entregue0 entregue1`.
pub(super) fn encode(stage: usize, delivered: [u32; 2]) -> String {
    format!("{stage} {} {}\n", delivered[0], delivered[1])
}

pub(super) fn decode(text: &str) -> (usize, [u32; 2]) {
    let mut numbers = text.split_whitespace().map(|word| word.parse::<u32>().unwrap_or(0));
    let stage = numbers.next().unwrap_or(0) as usize;
    let delivered = [numbers.next().unwrap_or(0), numbers.next().unwrap_or(0)];
    (stage.min(RESEARCH.len()), delivered)
}

fn save(state: &mut State) {
    if !state.dirty { return; }
    let Some(world) = &state.world else { return; };
    let path = file(world);
    if let Some(parent) = path.parent() { let _ = std::fs::create_dir_all(parent); }
    if std::fs::write(&path, encode(state.stage, state.delivered)).is_ok() {
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
        let (stage, delivered) = world.as_ref()
            .and_then(|world| std::fs::read_to_string(file(world)).ok())
            .map_or((0, [0, 0]), |text| decode(&text));
        *state = State { world, checked: time, stage, delivered, dirty: false };
    });
}

pub(super) fn current() -> Option<&'static Research> {
    RESEARCH.get(with_state(|state| state.stage))
}

pub(super) fn accepts(slot: usize, stack: &ItemStack) -> bool {
    slot < 2 && current().is_some_and(|research| is_plain(stack, research.inputs[slot].0))
}

/// Entrega um item por entrada, se houver e faltar. Devolve quais entradas receberam.
/// Separado do tick para os testes.
pub(super) fn deliver(stage: usize, delivered: &mut [u32; 2], slots: &mut [Option<ItemStack>]) -> [bool; 2] {
    let Some(research) = RESEARCH.get(stage) else { return [false; 2] };
    let mut took = [false; 2];
    for (index, &(item, need)) in research.inputs.iter().enumerate() {
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
        set_number(entity, "Status", 0);
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

/// Linhas do painel: pesquisa atual, o que falta de cada entrada e o que ela libera. A barra
/// de progresso do painel comum mostra o total entregue.
pub(super) fn panel_lines(data: &[i32]) -> Vec<String> {
    use horizon_client::mod_api::Text;
    let read = |index: usize| data.get(index).copied().unwrap_or(0);
    let stage = read(8).max(0) as usize;
    let Some(research) = RESEARCH.get(stage) else {
        return vec![Text::tr("veio.research.complete", &[]).plain()];
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
        assert_eq!(decode(&encode(2, [17, 40])), (2, [17, 40]));
        assert_eq!(decode("garbage"), (0, [0, 0]));
        assert_eq!(decode("99 1 1").0, RESEARCH.len());
    }

    #[test]
    fn each_research_needs_the_previous_tier() {
        // Metalurgia antes de circuitos; o núcleo de automação só existe depois da fabricadora.
        let order: Vec<_> = RESEARCH.iter().map(|research| research.id).collect();
        assert_eq!(order, ["metallurgy", "electronics", "automation", "scale"]);
        assert!(RESEARCH.iter().all(|research| research.inputs.iter().all(|&(_, need)| need >= 32)));
    }
}
