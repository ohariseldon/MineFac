//! Estações de carga e descarga: a produção de um posto distante continua chegando à base.
//!
//! Como no Satisfactory, a fábrica longe do jogador não precisa estar carregada. A estação de
//! carga recebe itens das esteiras e os manda para o canal dela (a cor do corante); a estação de
//! descarga do mesmo canal os entrega na base. Enquanto o posto está carregado, a carga mede
//! quantos itens de cada tipo manda por minuto. Quando o chunk descarrega, ela continua mandando
//! nesse ritmo, de forma virtual: supõe-se que o posto siga como estava, o mesmo regime estável
//! que o Satisfactory simula. Ao voltar, o posto retoma de onde parou, sem recuperar duas vezes.
//!
//! O canal guarda até `CAPACITY` itens em trânsito: com a descarga cheia ou longe, o canal enche e
//! a carga para, como uma esteira travada. O estado fica em `<mundo>/veio/estacoes.txt`.
use super::*;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Mutex;

pub(super) const SENDER: &str = "veio:cargo_station";
pub(super) const RECEIVER: &str = "veio:unloading_station";
/// Itens em trânsito por canal.
pub(super) const CAPACITY: u32 = 4096;
/// Itens por envio, a cada 8 ticks: 1 por tick, 1.200 por minuto.
pub(super) const BATCH: u32 = 8;
/// Energia por item enviado.
pub(super) const POWER_PER_ITEM: i32 = 10;
/// Janela de medição do ritmo, em ticks.
const WINDOW: i64 = 1200;
/// Sem rodar há tanto tempo, a carga está descarregada e passa a mandar de forma virtual.
const UNLOADED_AFTER: i64 = 100;

pub(super) const DYES: [&str; 16] = ["white", "orange", "magenta", "light_blue", "yellow", "lime", "pink", "gray",
    "light_gray", "cyan", "purple", "blue", "brown", "green", "red", "black"];

pub(super) fn is_station(block: &str) -> bool { matches!(block, SENDER | RECEIVER) }

pub(super) fn dye_channel(item: &str) -> Option<i32> {
    let color = item.strip_prefix("minecraft:")?.strip_suffix("_dye")?;
    DYES.iter().position(|dye| *dye == color).map(|index| index as i32)
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(super) struct Sender {
    pub channel: i32,
    pub seen: i64,
    /// Itens por tick medidos na última janela completa.
    pub rates: Vec<(String, f64)>,
    window: Vec<(String, u32)>,
    window_start: i64,
    /// Até onde a produção virtual já foi somada, e as frações que sobraram.
    virtual_at: i64,
    carry: Vec<(String, f64)>,
}

#[derive(Default, Debug, PartialEq)]
pub(super) struct Network {
    pub senders: HashMap<(String, [i32; 3]), Sender>,
    /// Itens em trânsito por canal.
    pub pools: HashMap<i32, Vec<(String, u32)>>,
}

fn add_to(list: &mut Vec<(String, u32)>, item: &str, count: u32) {
    if let Some(entry) = list.iter_mut().find(|(known, _)| known == item) { entry.1 += count; }
    else { list.push((item.to_owned(), count)); }
}

impl Network {
    pub fn in_transit(&self, channel: i32) -> u32 {
        self.pools.get(&channel).map_or(0, |pool| pool.iter().map(|(_, count)| count).sum())
    }

    /// Quanto o canal ainda aceita.
    pub fn room(&self, channel: i32) -> u32 { CAPACITY.saturating_sub(self.in_transit(channel)) }

    /// Uma carga carregada rodou: registra o canal, encerra a janela de medição e, se ela estava
    /// descarregada, para a produção virtual.
    pub fn touch(&mut self, key: (String, [i32; 3]), channel: i32, time: i64) -> &mut Sender {
        let sender = self.senders.entry(key).or_insert_with(|| Sender { window_start: time, ..Sender::default() });
        if sender.channel != channel {
            *sender = Sender { channel, window_start: time, ..Sender::default() };
        }
        sender.seen = time;
        sender.virtual_at = time;
        sender.carry.clear();
        if time - sender.window_start >= WINDOW {
            let span = (time - sender.window_start) as f64;
            sender.rates = sender.window.iter().map(|(item, count)| (item.clone(), f64::from(*count) / span)).collect();
            sender.window.clear();
            sender.window_start = time;
        }
        sender
    }

    /// Envio real de uma carga carregada; devolve quanto entrou no canal.
    pub fn send(&mut self, key: (String, [i32; 3]), channel: i32, time: i64, item: &str, count: u32) -> u32 {
        let accepted = count.min(self.room(channel));
        if accepted == 0 { return 0; }
        add_to(self.pools.entry(channel).or_default(), item, accepted);
        let sender = self.touch(key, channel, time);
        add_to(&mut sender.window, item, accepted);
        accepted
    }

    /// Retira até `count` itens do canal, na ordem em que chegaram os tipos.
    pub fn take(&mut self, channel: i32, mut accept: impl FnMut(&str, u32) -> u32) -> u32 {
        let Some(pool) = self.pools.get_mut(&channel) else { return 0 };
        let mut total = 0;
        for (item, count) in pool.iter_mut() {
            let taken = accept(item, *count).min(*count);
            *count -= taken;
            total += taken;
        }
        pool.retain(|(_, count)| *count > 0);
        total
    }

    /// Produção virtual das cargas descarregadas até `time`, no ritmo medido e com o canal
    /// segurando o excesso.
    pub fn simulate(&mut self, time: i64) {
        let mut adds: Vec<(i32, String, u32)> = Vec::new();
        for sender in self.senders.values_mut() {
            if time - sender.seen <= UNLOADED_AFTER || sender.rates.is_empty() { continue; }
            let elapsed = (time - sender.virtual_at).max(0) as f64;
            sender.virtual_at = time;
            for (item, rate) in &sender.rates {
                let carry = match sender.carry.iter_mut().find(|(known, _)| known == item) {
                    Some(entry) => entry,
                    None => { sender.carry.push((item.clone(), 0.0)); sender.carry.last_mut().unwrap() }
                };
                carry.1 += rate * elapsed;
                let whole = carry.1.floor();
                if whole >= 1.0 {
                    carry.1 -= whole;
                    adds.push((sender.channel, item.clone(), whole.min(f64::from(CAPACITY)) as u32));
                }
            }
        }
        for (channel, item, count) in adds {
            let accepted = count.min(self.room(channel));
            if accepted > 0 { add_to(self.pools.entry(channel).or_default(), &item, accepted); }
        }
    }

    /// Esquece cargas que não rodam há muito e nunca mediram nada (blocos quebrados cedo).
    pub fn forget_idle(&mut self, time: i64) {
        self.senders.retain(|_, sender| !sender.rates.is_empty() || time - sender.seen <= WINDOW * 2);
    }

    /// Texto salvo: uma linha por carga (`S dim x y z canal visto item ritmo ...`) e por canal
    /// (`P canal item quantidade ...`).
    pub fn encode(&self) -> String {
        let mut lines = Vec::new();
        let mut senders: Vec<_> = self.senders.iter().collect();
        senders.sort_by(|a, b| a.0.cmp(b.0));
        for ((dim, [x, y, z]), sender) in senders {
            let rates: String = sender.rates.iter().map(|(item, rate)| format!(" {item} {rate}")).collect();
            lines.push(format!("S {dim} {x} {y} {z} {} {}{rates}", sender.channel, sender.seen));
        }
        let mut pools: Vec<_> = self.pools.iter().filter(|(_, pool)| !pool.is_empty()).collect();
        pools.sort_by_key(|(channel, _)| **channel);
        for (channel, pool) in pools {
            let items: String = pool.iter().map(|(item, count)| format!(" {item} {count}")).collect();
            lines.push(format!("P {channel}{items}"));
        }
        lines.iter().map(|line| format!("{line}\n")).collect()
    }

    pub fn decode(text: &str) -> Network {
        let mut network = Network::default();
        for line in text.lines() {
            let words: Vec<&str> = line.split_whitespace().collect();
            match words.first() {
                Some(&"S") if words.len() >= 7 => {
                    let (Ok(x), Ok(y), Ok(z), Ok(channel), Ok(seen)) = (words[2].parse(), words[3].parse(),
                        words[4].parse(), words[5].parse::<i32>(), words[6].parse::<i64>()) else { continue };
                    let rates = words[7..].chunks_exact(2)
                        .filter_map(|pair| Some((pair[0].to_owned(), pair[1].parse::<f64>().ok()?))).collect();
                    network.senders.insert((words[1].to_owned(), [x, y, z]), Sender {
                        channel: channel.clamp(0, 15), seen, rates, window_start: seen, virtual_at: seen, ..Sender::default() });
                }
                Some(&"P") if words.len() >= 2 => {
                    let Ok(channel) = words[1].parse::<i32>() else { continue };
                    let pool = network.pools.entry(channel).or_default();
                    for pair in words[2..].chunks_exact(2) {
                        if let Ok(count) = pair[1].parse::<u32>() { add_to(pool, pair[0], count.min(CAPACITY)); }
                    }
                }
                _ => {}
            }
        }
        network
    }
}

#[derive(Default)]
struct State {
    world: Option<PathBuf>,
    network: Network,
    loaded: bool,
}

static STATE: Mutex<Option<State>> = Mutex::new(None);

fn with_state<T>(change: impl FnOnce(&mut State) -> T) -> T {
    let mut guard = STATE.lock().unwrap_or_else(|err| err.into_inner());
    change(guard.get_or_insert_with(State::default))
}

fn file(world: &std::path::Path) -> PathBuf { world.join("veio").join("estacoes.txt") }

fn save(state: &State) {
    let Some(world) = &state.world else { return };
    let path = file(world);
    if let Some(parent) = path.parent() { let _ = std::fs::create_dir_all(parent); }
    let _ = std::fs::write(path, state.network.encode());
}

/// Tick do mod: troca de mundo, produção virtual e gravação.
pub(super) fn maintain(host: &mut dyn Host, time: i64) {
    if time.rem_euclid(20) != 0 || !industry::on_server(host) { return; }
    let world = host.world_dir();
    with_state(|state| {
        if world.is_some() && world != state.world {
            if state.loaded { save(state); }
            *state = State { world: world.clone(), network: Network::default(), loaded: false };
        }
        if !state.loaded && state.world.is_some() {
            state.loaded = true;
            state.network = state.world.as_ref().and_then(|world| std::fs::read_to_string(file(world)).ok())
                .map(|text| Network::decode(&text)).unwrap_or_default();
        }
        state.network.simulate(time);
        if time.rem_euclid(1200) == 0 {
            state.network.forget_idle(time);
            save(state);
        }
    });
}

pub(super) fn tick(host: &mut dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity, time: i64) {
    // Tudo a cada 8 ticks, como as esteiras: o resto do tempo a estação não custa nada.
    if time.rem_euclid(8) != 0 || !industry::on_server(host) { return; }
    let channel = number(entity, "Channel").clamp(0, 15);
    let key = (host.dimension().to_owned(), pos);
    if entity.id == SENDER {
        let energy = number(entity, "energy");
        let (sent, transit, rate) = with_state(|state| {
            let network = &mut state.network;
            network.touch(key.clone(), channel, time);
            let mut sent = 0;
            {
                let mut budget = BATCH.min((energy / POWER_PER_ITEM).max(0) as u32);
                for slot in entity.items.iter_mut() {
                    if budget == 0 { break; }
                    let Some(stack) = slot.as_ref().filter(|stack| stack.damage == 0 && stack.extra.is_none()) else { continue };
                    let item = stack.item.to_string();
                    let moved = network.send(key.clone(), channel, time, &item, stack.count.min(budget));
                    if moved > 0 {
                        consume(slot, moved);
                        budget -= moved;
                        sent += moved;
                    }
                }
            }
            let rate = network.senders.get(&key).map_or(0.0, |sender| sender.rates.iter().map(|(_, rate)| rate).sum::<f64>());
            (sent, network.in_transit(channel), (rate * 1200.0).round() as i32)
        });
        set_number(entity, "energy", energy - sent as i32 * POWER_PER_ITEM);
        if sent > 0 { set_number(entity, "Produced", number(entity, "Produced") + sent as i32); }
        let has_items = entity.items.iter().any(Option::is_some);
        let status = if sent > 0 { 1 } else if !has_items { 2 } else if transit >= CAPACITY { 3 }
            else if energy < POWER_PER_ITEM { 5 } else { number(entity, "Status") };
        set_number(entity, "Status", status);
        set_number(entity, "Total", transit as i32);
        set_number(entity, "Info", rate);
    } else {
        let mut received = 0;
        let transit = with_state(|state| {
            {
                received = state.network.take(channel, |item, count| {
                    let mut taken = 0;
                    for slot in entity.items.iter_mut() {
                        let room = match slot {
                            None => 64,
                            Some(stack) if is_plain(stack, item) => 64u32.saturating_sub(stack.count),
                            _ => 0,
                        };
                        let amount = room.min(count - taken).min(BATCH * 2);
                        if amount > 0 { add(slot, item, amount); taken += amount; }
                        if taken == count { break; }
                    }
                    taken
                });
            }
            state.network.in_transit(channel)
        });
        if received > 0 { set_number(entity, "Produced", number(entity, "Produced") + received as i32); }
        let full = entity.items.iter().all(|slot| slot.as_ref().is_some_and(|stack| stack.count >= 64));
        let status = if received > 0 { 1 } else if full && transit > 0 { 3 } else if transit == 0 { 2 } else { number(entity, "Status") };
        set_number(entity, "Status", status);
        set_number(entity, "Total", transit as i32);
    }
    set_number(entity, "ContractStage", channel);
}

/// Linhas do painel das duas estações.
pub(super) fn panel_lines(block: &str, data: &[i32]) -> Vec<String> {
    use horizon_client::mod_api::Text;
    let read = |index: usize| data.get(index).copied().unwrap_or(0);
    let channel = Text::tr(&format!("color.minecraft.{}", DYES[read(8).clamp(0, 15) as usize]), &[]).plain();
    let first = Text::tr("veio.station.channel", &[&channel, &read(9), &CAPACITY]).plain();
    if block == SENDER {
        vec![first, Text::tr("veio.station.rate", &[&read(12)]).plain(), Text::tr("veio.hint.cargo", &[]).plain()]
    } else {
        vec![first, Text::tr("veio.hint.unloading", &[]).plain()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn key(x: i32) -> (String, [i32; 3]) { ("minecraft:overworld".to_owned(), [x, 64, 0]) }

    #[test]
    fn unloaded_outpost_keeps_shipping_at_the_measured_rate_until_the_channel_fills() {
        let mut network = Network::default();
        // Carregado: 60 placas em 1.200 ticks, uma a cada 20 ticks.
        for tick in (0..=1200).step_by(20) {
            if tick < 1200 { network.send(key(0), 3, tick, "veio:iron_plate", 1); }
        }
        network.touch(key(0), 3, 1200);
        let rate = network.senders[&key(0)].rates[0].1;
        assert!((rate - 60.0 / 1200.0).abs() < 1e-9, "60 itens por 1.200 ticks: {rate}");
        assert_eq!(network.in_transit(3), 60);
        // A base retira tudo.
        assert_eq!(network.take(3, |_, count| count), 60);
        // Descarregado: nada antes de 100 ticks; depois, no ritmo medido.
        network.simulate(1290);
        assert_eq!(network.in_transit(3), 0, "ainda conta como carregado");
        network.simulate(1200 + 2400);
        assert_eq!(network.in_transit(3), 120, "2.400 ticks a 0,05 por tick");
        network.simulate(1200 + 2400 + 10_000_000);
        assert_eq!(network.in_transit(3), CAPACITY, "canal cheio segura a carga");
        assert_eq!(network.room(3), 0);
        // Voltou a carregar: a produção virtual para.
        network.touch(key(0), 3, 20_000_000);
        network.take(3, |_, count| count);
        network.simulate(20_000_050);
        assert_eq!(network.in_transit(3), 0);
    }

    #[test]
    fn receiver_takes_what_fits_and_the_network_survives_saving() {
        let mut network = Network::default();
        network.send(key(0), 1, 0, "minecraft:coal", 30);
        network.send(key(0), 1, 0, "veio:steel_plate", 5);
        let mut room = 20;
        let taken = network.take(1, |_, count| { let t = count.min(room); room -= t; t });
        assert_eq!(taken, 20);
        assert_eq!(network.in_transit(1), 15);
        network.senders.get_mut(&key(0)).unwrap().rates = vec![("minecraft:coal".to_owned(), 0.25)];
        let decoded = Network::decode(&network.encode());
        assert_eq!(decoded.in_transit(1), 15);
        assert_eq!(decoded.senders[&key(0)].rates, vec![("minecraft:coal".to_owned(), 0.25)]);
        assert_eq!(decoded.senders[&key(0)].channel, 1);
        assert_eq!(Network::decode("lixo\nS x\nP y").in_transit(0), 0);
        assert_eq!(dye_channel("minecraft:red_dye"), Some(14));
        assert_eq!(dye_channel("minecraft:light_blue_dye"), Some(3));
        assert_eq!(dye_channel("minecraft:redstone"), None);
    }

    /// Custo da simulação: `cargo test --release -- --ignored stations_cost --nocapture`.
    #[test]
    #[ignore]
    fn stations_cost() {
        let mut network = Network::default();
        for x in 0..1000 {
            network.senders.insert(key(x), Sender { channel: x % 16, seen: 0, virtual_at: 0,
                rates: vec![("veio:iron_plate".to_owned(), 0.5), ("veio:gear".to_owned(), 0.1)], ..Sender::default() });
        }
        let start = std::time::Instant::now();
        for step in 1..=100 { network.simulate(1000 + step * 20); }
        println!("1000 estações descarregadas: {:?} por passo de 20 ticks", start.elapsed() / 100);
    }
}
