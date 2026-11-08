//! Visão da fábrica inteira e âncora de chunk.
//!
//! Monitor: como as estatísticas de produção do Factorio. Cada máquina com receita (ou broca)
//! se registra a cada tick com o que produz e consome por minuto; o monitor soma tudo, mostra os
//! nove itens de maior fluxo e avisa os jogadores quando há máquinas paradas por saída cheia,
//! falta de energia ou desgaste. O registro vive na memória e esquece máquinas que deixaram de
//! rodar há 200 ticks (chunk descarregado ou bloco quebrado).
//!
//! Âncora: gasta energia para manter o chunk carregado com `forceload`. Sem energia, solta o
//! chunk. Quebrada, deixa de se registrar e o chunk é solto 400 ticks depois. A lista fica em
//! `<mundo>/veio/ancoras.txt`, para soltar também os chunks de âncoras quebradas antes de fechar
//! o jogo.
use super::*;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Mutex;

pub(super) const MONITOR: &str = "veio:factory_monitor";
pub(super) const ANCHOR: &str = "veio:chunk_anchor";
/// Energia por tick da âncora: o preço de uma fábrica que roda longe do jogador.
pub(super) const ANCHOR_POWER: i32 = 32;
/// Itens no painel do monitor.
pub(super) const ROWS: usize = 9;
const FORGET: i64 = 200;
const ALERT_EVERY: i64 = 600;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum Health { Working, Waiting, Problem, Idle }

/// Agrupa os estados do painel em trabalhando, esperando insumo e com problema.
pub(super) fn health(status: i32) -> Health {
    match status {
        1 => Health::Working,
        2 | 4 | 6 | 14 | 16 | 17 => Health::Waiting,
        3 | 5 | 13 => Health::Problem,
        _ => Health::Idle,
    }
}

#[derive(Clone, Debug)]
struct Machine {
    seen: i64,
    status: i32,
    /// (item, itens por minuto): positivos produzem, negativos consomem.
    flows: Vec<(&'static str, i32)>,
}

#[derive(Default)]
struct Registry {
    machines: HashMap<(&'static str, [i32; 3]), Machine>,
    last_alert: i64,
}

static REGISTRY: Mutex<Option<Registry>> = Mutex::new(None);

fn with_registry<T>(change: impl FnOnce(&mut Registry) -> T) -> T {
    let mut guard = REGISTRY.lock().unwrap_or_else(|err| err.into_inner());
    change(guard.get_or_insert_with(Registry::default))
}

/// Itens conhecidos pelo monitor: as entradas e saídas das receitas e o minério das brocas.
/// O índice (1 em diante) viaja nos números do painel.
pub(super) fn items() -> &'static [&'static str] {
    static ITEMS: std::sync::OnceLock<Vec<&'static str>> = std::sync::OnceLock::new();
    ITEMS.get_or_init(|| {
        let mut items: Vec<&'static str> = Vec::new();
        let mut push = |item: &'static str| if !items.contains(&item) { items.push(item); };
        for recipe in production::data::RECIPES {
            for input in recipe.inputs { push(input.items[0]); }
            push(recipe.output.0);
        }
        for ore in ["minecraft:raw_iron", "minecraft:raw_copper", "minecraft:raw_gold", "minecraft:coal",
            "minecraft:redstone", "minecraft:quartz", "minecraft:diamond", "minecraft:lapis_lazuli"] { push(ore); }
        items.truncate(127);
        items
    })
}

fn item_index(item: &str) -> Option<usize> {
    items().iter().position(|known| *known == item)
}

/// Fluxos de uma máquina: a receita ativa vezes os ciclos por minuto medidos (`Rate`).
pub(super) fn flows(entity: &ModBlockEntity) -> Vec<(&'static str, i32)> {
    let cycles = number(entity, "Rate");
    let index = number(entity, "RecipeIndex");
    if let Some(recipe) = usize::try_from(index).ok().and_then(|index| index.checked_sub(1))
        .and_then(|index| production::recipes(&entity.id).nth(index))
    {
        let mut flows: Vec<_> = recipe.inputs.iter().map(|input| (input.items[0], -(input.count as i32) * cycles)).collect();
        flows.push((recipe.output.0, recipe.output.1 as i32 * cycles));
        return flows;
    }
    // Brocas: o que está no slot de saída é o minério do veio.
    if matches!(entity.id.as_str(), DRILL | DRILL_MK2 | production::DRILL_MK3) {
        if let Some(stack) = entity.items.first().and_then(Option::as_ref)
            && let Some(index) = item_index(stack.item.as_ref())
        {
            return vec![(items()[index], cycles)];
        }
    }
    Vec::new()
}

/// Só o servidor força chunks e avisa jogadores; a janela do cliente não responde a esta consulta.
pub(super) fn on_server(host: &dyn Host) -> bool {
    host.surface_heights(host.dimension(), &[], horizon_client::mod_api::NativeHeightmap::MotionBlocking).is_ok()
}

fn tracked(block: &str) -> bool {
    needs_power(block) && block != ANCHOR && block != research::LAB && !inserters::is_arm(block)
        && !matches!(block, TESLA)
}

/// Chamado depois do tick de cada bloco da fábrica.
pub(super) fn observe(dimension: &'static str, pos: [i32; 3], entity: &ModBlockEntity, time: i64) {
    // Uma amostra a cada 20 ticks por máquina, espalhada pela posição: o registro aceita até
    // 200 ticks sem notícia, e as taxas medidas (`Rate`) só mudam a cada minuto.
    if (time + i64::from(pos[0]) + i64::from(pos[2])).rem_euclid(20) != 0 || !tracked(&entity.id) { return; }
    let machine = Machine { seen: time, status: number(entity, "Status"), flows: flows(entity) };
    with_registry(|registry| { registry.machines.insert((dimension, pos), machine); });
}

/// Resumo de uma dimensão: contagem por estado, primeira máquina com problema e os fluxos.
#[derive(Default, Debug, PartialEq)]
pub(super) struct Summary {
    pub working: i32,
    pub waiting: i32,
    pub problems: i32,
    pub problem_at: Option<[i32; 3]>,
    /// (item, produz, consome) por minuto, os de maior fluxo primeiro.
    pub items: Vec<(&'static str, i32, i32)>,
}

fn summarize(registry: &mut Registry, dimension: &str, time: i64) -> Summary {
    registry.machines.retain(|_, machine| time - machine.seen <= FORGET);
    let mut summary = Summary::default();
    let mut totals: HashMap<&'static str, (i32, i32)> = HashMap::new();
    let mut positions: Vec<_> = registry.machines.iter().filter(|((dim, _), _)| *dim == dimension).collect();
    positions.sort_by_key(|((_, pos), _)| *pos);
    for ((_, pos), machine) in positions {
        match health(machine.status) {
            Health::Working => summary.working += 1,
            Health::Waiting => summary.waiting += 1,
            Health::Problem => {
                summary.problems += 1;
                summary.problem_at.get_or_insert(*pos);
            }
            Health::Idle => {}
        }
        for &(item, flow) in &machine.flows {
            let total = totals.entry(item).or_default();
            if flow >= 0 { total.0 += flow; } else { total.1 -= flow; }
        }
    }
    let mut items: Vec<_> = totals.into_iter().filter(|(_, (made, used))| made + used > 0)
        .map(|(item, (made, used))| (item, made, used)).collect();
    items.sort_by(|a, b| (b.1.max(b.2)).cmp(&a.1.max(a.2)).then(a.0.cmp(b.0)));
    summary.items = items;
    summary
}

/// Número do painel: índice do item (7 bits), produção e consumo (12 bits cada, até 4.095/min).
pub(super) fn pack(index: usize, made: i32, used: i32) -> i32 {
    ((index as i32 & 0x7f) << 24) | (made.clamp(0, 4095) << 12) | used.clamp(0, 4095)
}

pub(super) fn unpack(value: i32) -> (usize, i32, i32) {
    (((value >> 24) & 0x7f) as usize, (value >> 12) & 0xfff, value & 0xfff)
}

pub(super) fn tick_monitor(host: &mut dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity, time: i64) {
    if time.rem_euclid(20) != 0 { return; }
    let dimension = host.dimension();
    let (summary, alert) = with_registry(|registry| {
        let summary = summarize(registry, dimension, time);
        let alert = summary.problems > 0 && time - registry.last_alert >= ALERT_EVERY;
        if alert { registry.last_alert = time; }
        (summary, alert)
    });
    set_number(entity, "Status", if summary.working > 0 { 1 } else { 0 });
    for row in 0..ROWS {
        let value = summary.items.get(row)
            .and_then(|&(item, made, used)| item_index(item).map(|index| pack(index + 1, made, used)))
            .unwrap_or(0);
        set_number(entity, &format!("Flow{row}"), value);
    }
    set_number(entity, "Working", summary.working);
    set_number(entity, "Waiting", summary.waiting);
    set_number(entity, "Problems", summary.problems);
    if alert && on_server(host) && let Some([x, y, z]) = summary.problem_at {
        let line = horizon_client::mod_api::Text::tr("veio.monitor.alert", &[&summary.problems, &x, &y, &z]);
        for player in host.server_players() {
            if player.dimension == dimension {
                host.action_bar(player.uuid, line.clone());
            }
        }
    }
    let _ = pos;
}

/// Os 13 números do painel do monitor.
pub(super) fn monitor_data(entity: &ModBlockEntity) -> [i32; presentation::DATA_SLOTS] {
    let mut data = [0; presentation::DATA_SLOTS];
    data[0] = number(entity, "Status");
    for row in 0..ROWS {
        data[1 + row] = number(entity, &format!("Flow{row}"));
    }
    data[10] = number(entity, "Working");
    data[11] = number(entity, "Waiting");
    data[12] = number(entity, "Problems");
    data
}

pub(super) fn draw_monitor(host: &dyn Host, event: &mut horizon_client::mod_api::ScreenDrawEvent<'_>) {
    use horizon_client::mod_api::Text;
    let [x, y] = event.origin;
    let read = |index: usize| event.data.get(index).copied().unwrap_or(0);
    let counts = Text::tr("veio.monitor.counts", &[&read(10), &read(11), &read(12)]).plain();
    host.text(event.batch, &counts, [x + 9.0, y + 24.0], if read(12) > 0 { presentation::text_color(3) } else { presentation::INK }, false);
    host.text(event.batch, &Text::tr("veio.monitor.header", &[]).plain(), [x + 9.0, y + 36.0], presentation::SOFT_INK, false);
    let mut shown = 0;
    for row in 0..ROWS {
        let (index, made, used) = unpack(read(1 + row));
        let Some(item) = index.checked_sub(1).and_then(|index| items().get(index)) else { continue; };
        let line_y = y + 48.0 + shown as f32 * 10.0;
        let mut name = contracts::item_name(item);
        while host.text_width(&name) > 100.0 && name.pop().is_some() {}
        host.text(event.batch, &name, [x + 9.0, line_y], presentation::INK, false);
        let made_text = format!("+{made}");
        let used_text = format!("-{used}");
        host.text(event.batch, &made_text, [x + 140.0 - host.text_width(&made_text), line_y], 0xff2f7d32, false);
        // Consumo maior que a produção pinta de vermelho: a linha está faltando esse item.
        let color = if used > made { 0xffb03030 } else { presentation::SOFT_INK };
        host.text(event.batch, &used_text, [x + 182.0 - host.text_width(&used_text), line_y], color, false);
        shown += 1;
    }
    if shown == 0 {
        host.text(event.batch, &Text::tr("veio.monitor.empty", &[]).plain(), [x + 9.0, y + 48.0], presentation::SOFT_INK, false);
    }
}

// ---------- Âncora de chunk ----------

#[derive(Default)]
struct Anchors {
    world: Option<PathBuf>,
    /// (dimensão, chunk) -> último tick em que uma âncora ligada desse chunk rodou.
    forced: HashMap<(String, [i32; 2]), i64>,
    loaded: bool,
}

static ANCHORS: Mutex<Option<Anchors>> = Mutex::new(None);

fn with_anchors<T>(change: impl FnOnce(&mut Anchors) -> T) -> T {
    let mut guard = ANCHORS.lock().unwrap_or_else(|err| err.into_inner());
    change(guard.get_or_insert_with(Anchors::default))
}

fn anchors_file(world: &std::path::Path) -> PathBuf { world.join("veio").join("ancoras.txt") }

pub(super) fn encode_anchors(forced: &[(String, [i32; 2])]) -> String {
    forced.iter().map(|(dim, [x, z])| format!("{dim} {x} {z}\n")).collect()
}

pub(super) fn decode_anchors(text: &str) -> Vec<(String, [i32; 2])> {
    text.lines().filter_map(|line| {
        let mut words = line.split_whitespace();
        let dim = words.next()?.to_owned();
        let x = words.next()?.parse().ok()?;
        let z = words.next()?.parse().ok()?;
        Some((dim, [x, z]))
    }).collect()
}

fn save_anchors(anchors: &Anchors) {
    let Some(world) = &anchors.world else { return; };
    let path = anchors_file(world);
    if let Some(parent) = path.parent() { let _ = std::fs::create_dir_all(parent); }
    let mut list: Vec<_> = anchors.forced.keys().cloned().collect();
    list.sort();
    let _ = std::fs::write(path, encode_anchors(&list));
}

fn forceload(host: &mut dyn Host, dimension: &str, [cx, cz]: [i32; 2], add: bool) {
    let action = if add { "add" } else { "remove" };
    let (x, z) = (cx * 16, cz * 16);
    let command = if dimension == "minecraft:overworld" {
        format!("forceload {action} {x} {z}")
    } else {
        format!("execute in {dimension} run forceload {action} {x} {z}")
    };
    host.run_command(&command);
}

pub(super) fn chunk_of([x, _, z]: [i32; 3]) -> [i32; 2] { [x.div_euclid(16), z.div_euclid(16)] }

pub(super) fn tick_anchor(host: &mut dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity, time: i64) {
    if !on_server(host) { return; }
    let dimension = host.dimension();
    let chunk = chunk_of(pos);
    let energy = number(entity, "energy");
    let powered = energy >= ANCHOR_POWER;
    if powered {
        set_number(entity, "energy", energy - ANCHOR_POWER);
        set_number(entity, "Status", 1);
    } else {
        set_number(entity, "Status", 5);
    }
    let forced = number(entity, "Forced") != 0;
    if powered {
        let fresh = with_anchors(|anchors| {
            let fresh = anchors.forced.insert((dimension.to_owned(), chunk), time).is_none();
            if fresh { save_anchors(anchors); }
            fresh
        });
        if fresh || !forced {
            forceload(host, dimension, chunk, true);
            set_number(entity, "Forced", 1);
        }
    } else if forced {
        // Espera 5 s sem energia antes de soltar, para um apagão curto não piscar o chunk.
        let since = number(entity, "Unpowered") + 1;
        set_number(entity, "Unpowered", since);
        if since >= 100 {
            set_number(entity, "Forced", 0);
            set_number(entity, "Unpowered", 0);
            with_anchors(|anchors| {
                anchors.forced.remove(&(dimension.to_owned(), chunk));
                save_anchors(anchors);
            });
            forceload(host, dimension, chunk, false);
        }
        return;
    }
    if powered { set_number(entity, "Unpowered", 0); }
}

/// Chamado no tick do mod: troca de mundo e chunks de âncoras que sumiram.
pub(super) fn maintain(host: &mut dyn Host, time: i64) {
    if time.rem_euclid(100) != 0 || !on_server(host) { return; }
    let world = host.world_dir();
    let stale = with_anchors(|anchors| {
        if world.is_some() && world != anchors.world {
            *anchors = Anchors { world: world.clone(), forced: HashMap::new(), loaded: false };
        }
        if !anchors.loaded {
            anchors.loaded = true;
            // Ao abrir o mundo, as âncoras dos chunks forçados voltam a rodar e se registram;
            // as que não rodarem em 400 ticks eram de blocos quebrados.
            let listed = anchors.world.as_ref().and_then(|world| std::fs::read_to_string(anchors_file(world)).ok())
                .map(|text| decode_anchors(&text)).unwrap_or_default();
            for key in listed { anchors.forced.entry(key).or_insert(time); }
        }
        let stale: Vec<_> = anchors.forced.iter().filter(|(_, seen)| time - **seen > 400).map(|(key, _)| key.clone()).collect();
        for key in &stale { anchors.forced.remove(key); }
        if !stale.is_empty() { save_anchors(anchors); }
        stale
    });
    for (dimension, chunk) in stale {
        forceload(host, &dimension, chunk, false);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn monitor_sums_flows_and_counts_problems() {
        let mut registry = Registry::default();
        let machine = |status, flows: Vec<(&'static str, i32)>| Machine { seen: 100, status, flows };
        registry.machines.insert(("minecraft:overworld", [0, 64, 0]), machine(1, vec![("veio:iron_plate", 40)]));
        registry.machines.insert(("minecraft:overworld", [2, 64, 0]), machine(1, vec![("veio:iron_plate", 40)]));
        registry.machines.insert(("minecraft:overworld", [4, 64, 0]), machine(3, vec![("veio:iron_plate", -120), (STEEL, 20)]));
        registry.machines.insert(("minecraft:overworld", [6, 64, 0]), machine(2, vec![]));
        registry.machines.insert(("minecraft:the_nether", [0, 64, 0]), machine(5, vec![]));
        registry.machines.insert(("minecraft:overworld", [9, 64, 9]), Machine { seen: -500, status: 5, flows: vec![] });
        let summary = summarize(&mut registry, "minecraft:overworld", 120);
        assert_eq!((summary.working, summary.waiting, summary.problems), (2, 1, 1));
        assert_eq!(summary.problem_at, Some([4, 64, 0]), "a máquina esquecida não conta");
        assert_eq!(summary.items[0], ("veio:iron_plate", 80, 120), "falta placa: consome mais do que produz");
        assert_eq!(summary.items[1], (STEEL, 20, 0));
        assert_eq!(registry.machines.len(), 5);
    }

    #[test]
    fn panel_numbers_and_anchor_list_round_trip() {
        assert_eq!(unpack(pack(17, 480, 4095)), (17, 480, 4095));
        assert_eq!(unpack(pack(126, 99999, -3)), (126, 4095, 0));
        assert!(items().len() <= 127 && items().contains(&STEEL));
        let list = vec![("minecraft:overworld".to_owned(), [-3, 7]), ("minecraft:the_nether".to_owned(), [0, 0])];
        assert_eq!(decode_anchors(&encode_anchors(&list)), list);
        assert_eq!(decode_anchors("lixo\nminecraft:overworld 1"), vec![]);
        assert_eq!(chunk_of([-1, 64, 31]), [-1, 1]);
        assert_eq!(health(13), Health::Problem);
        assert_eq!(health(2), Health::Waiting);
    }
}
