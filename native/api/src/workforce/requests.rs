//! Pedidos da vila: a cada manhã, o centro de controle recebe pedidos de coisas que a vila quer
//! (pão, chapas, vidro, lanternas, circuitos...). O jogador entrega pelo slot **Obra**; cada pedido
//! atendido paga esmeraldas ao **caixa** da vila e 10 pontos de prestígio.
//!
//! - 3 pedidos abertos (4 com o correio). Os pedidos ficam até serem atendidos: nada entregue se
//!   perde. Na manhã seguinte, os atendidos dão lugar a novos; um pedido parado há 3 dias sem
//!   nenhuma entrega é trocado.
//! - A lista depende do prestígio: pedidos mais ricos (engrenagens, aço, circuitos, módulos)
//!   aparecem nas vilas maiores.
//! - Sorteio determinístico pelo dia e pela posição do centro, sem gerador global.
//!
//! Cada pedido ocupa um inteiro no NBT do centro (`Req0`..`Req3`): item da lista (6 bits),
//! quantidade pedida (8), entregue (8) e dias aberto (3). Zero = vago.
use super::*;

pub(crate) struct Wish {
    pub item: &'static str,
    pub name: &'static str,
    pub min: u32,
    pub max: u32,
    /// Esmeraldas pagas ao caixa.
    pub reward: u32,
    /// Prestígio mínimo da vila para pedir.
    pub tier: i32,
}

const fn w(item: &'static str, name: &'static str, min: u32, max: u32, reward: u32, tier: i32) -> Wish {
    Wish { item, name, min, max, reward, tier }
}

pub(crate) const POOL: [Wish; 27] = [
    w("minecraft:bread", "item.minecraft.bread", 8, 16, 2, 0),
    w("veio:iron_plate", "item.veio.iron_plate", 8, 16, 2, 0),
    w("veio:copper_plate", "item.veio.copper_plate", 8, 16, 2, 0),
    w("minecraft:glass", "block.minecraft.glass", 8, 16, 2, 0),
    w("minecraft:brick", "item.minecraft.brick", 12, 24, 2, 0),
    w("minecraft:cooked_beef", "item.minecraft.cooked_beef", 4, 8, 2, 0),
    w("minecraft:white_wool", "block.minecraft.white_wool", 4, 12, 1, 0),
    w("minecraft:egg", "item.minecraft.egg", 8, 16, 1, 0),
    w("minecraft:torch", "block.minecraft.torch", 16, 32, 1, 0),
    w("minecraft:baked_potato", "item.minecraft.baked_potato", 8, 16, 1, 0),
    w("veio:gear", "item.veio.gear", 4, 8, 3, 1),
    w("veio:steel_plate", "item.veio.steel_plate", 4, 8, 4, 1),
    w("veio:wire", "item.veio.wire", 8, 16, 3, 1),
    w("minecraft:cookie", "item.minecraft.cookie", 16, 32, 2, 1),
    w("minecraft:pumpkin_pie", "item.minecraft.pumpkin_pie", 4, 8, 3, 1),
    w("minecraft:lantern", "block.minecraft.lantern", 4, 8, 3, 1),
    w("minecraft:bucket", "item.minecraft.bucket", 2, 4, 2, 1),
    w("minecraft:bookshelf", "block.minecraft.bookshelf", 2, 4, 3, 1),
    w("veio:circuit", "item.veio.circuit", 2, 4, 5, 2),
    w("veio:module", "item.veio.module", 1, 2, 6, 2),
    w("minecraft:cake", "block.minecraft.cake", 1, 2, 5, 2),
    w("minecraft:golden_carrot", "item.minecraft.golden_carrot", 4, 8, 5, 2),
    w("minecraft:rail", "block.minecraft.rail", 16, 32, 3, 2),
    w("veio:silicon", "item.veio.silicon", 4, 8, 4, 2),
    w("minecraft:redstone_lamp", "block.minecraft.redstone_lamp", 2, 4, 4, 3),
    w("veio:speed_module", "item.veio.speed_module", 1, 1, 8, 3),
    w("minecraft:golden_apple", "item.minecraft.golden_apple", 1, 2, 8, 3),
];

pub(crate) const SLOTS: usize = 4;
/// Dias sem nenhuma entrega antes de trocar o pedido.
const STALE_DAYS: i32 = 3;
pub(crate) const PRESTIGE: i32 = 10;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Request {
    pub wish: usize,
    pub need: u32,
    pub have: u32,
    pub age: i32,
}

impl Request {
    pub(crate) fn done(&self) -> bool {
        self.have >= self.need
    }
}

pub(crate) fn pack(request: &Request) -> i32 {
    // +1 no item: zero significa vago.
    ((request.wish as i32 + 1) & 63) | ((request.need.min(255) as i32) << 6) | ((request.have.min(255) as i32) << 14)
        | (request.age.clamp(0, 7) << 22)
}

pub(crate) fn unpack(value: i32) -> Option<Request> {
    let wish = (value & 63) as usize;
    (wish > 0 && wish <= POOL.len()).then(|| Request {
        wish: wish - 1, need: ((value >> 6) & 255) as u32, have: ((value >> 14) & 255) as u32, age: (value >> 22) & 7,
    })
}

fn slot_key(index: usize) -> &'static str {
    ["Req0", "Req1", "Req2", "Req3"][index]
}

pub(crate) fn get(entity: &ModBlockEntity, index: usize) -> Option<Request> {
    match entity.nbt.get(nbt_key(slot_key(index))) {
        Some(Tag::Int(value)) => unpack(*value),
        _ => None,
    }
}

fn put(entity: &mut ModBlockEntity, index: usize, request: Option<Request>) {
    entity.nbt.insert(nbt_key(slot_key(index)).clone(), Tag::Int(request.as_ref().map_or(0, pack)));
}

/// Quantos pedidos a vila mantém: 3, ou 4 com o correio.
pub(crate) fn open_slots(perks: u32) -> usize {
    if perks & projects::POST_OFFICE != 0 { 4 } else { 3 }
}

/// Esmeraldas pagas por um pedido (correio: +50%).
pub(crate) fn reward(wish: usize, perks: u32) -> u32 {
    let base = POOL[wish].reward;
    if perks & projects::POST_OFFICE != 0 { base + base.div_ceil(2) } else { base }
}

pub(crate) fn wanted(item: &str) -> bool {
    POOL.iter().any(|wish| wish.item == item)
}

pub(crate) fn open(entity: &ModBlockEntity) -> i32 {
    (0..SLOTS).filter_map(|index| get(entity, index)).filter(|request| !request.done()).count() as i32
}

fn roll(seed: u64) -> u64 {
    let mut n = seed.wrapping_mul(0x9E37_79B9_7F4A_7C15);
    n ^= n >> 29;
    n = n.wrapping_mul(0xBF58_476D_1CE4_E5B9);
    n ^ (n >> 32)
}

/// Sorteia um pedido que a vila ainda não tem aberto.
fn draw(pos: [i32; 3], day: i32, index: usize, tier: i32, taken: &[usize]) -> Option<Request> {
    let choices: Vec<usize> = (0..POOL.len()).filter(|wish| POOL[*wish].tier <= tier && !taken.contains(wish)).collect();
    if choices.is_empty() {
        return None;
    }
    let seed = (pos[0] as i64 as u64).wrapping_mul(31) ^ (pos[2] as i64 as u64).wrapping_mul(1_000_003)
        ^ (day as u64).wrapping_mul(7919) ^ (index as u64).wrapping_mul(104_729);
    let n = roll(seed);
    let wish = choices[(n % choices.len() as u64) as usize];
    let span = POOL[wish].max - POOL[wish].min + 1;
    // Vilas maiores pedem mais (até a borda de cima da faixa).
    let need = (POOL[wish].min + ((n >> 16) % u64::from(span)) as u32 + tier as u32 * span / 4).min(POOL[wish].max);
    Some(Request { wish, need, have: 0, age: 0 })
}

/// Manhã: atendidos e parados saem, os vagos recebem pedidos novos e os abertos envelhecem.
/// Também usado na primeira vez, para a vila já começar com pedidos.
pub(crate) fn morning(entity: &mut ModBlockEntity, pos: [i32; 3], day: i32, perks: u32) {
    let tier = projects::tier_bits(perks);
    let slots = open_slots(perks);
    for index in 0..SLOTS {
        let current = get(entity, index);
        let keep = current.filter(|request| index < slots && !request.done()
            && !(request.have == 0 && request.age + 1 >= STALE_DAYS))
            .map(|request| Request { age: request.age + 1, ..request });
        put(entity, index, keep);
    }
    for index in 0..slots {
        if get(entity, index).is_none() {
            let taken: Vec<usize> = (0..SLOTS).filter_map(|other| get(entity, other)).map(|request| request.wish).collect();
            let fresh = draw(pos, day, index, tier, &taken);
            put(entity, index, fresh);
        }
    }
}

/// Entrega o que está no slot de obra aos pedidos abertos. Devolve as esmeraldas ganhas e
/// quantos pedidos foram atendidos agora.
pub(crate) fn deliver(entity: &mut ModBlockEntity, perks: u32) -> (u32, i32) {
    let Some(stack) = entity.items.get(projects::SITE).and_then(Option::as_ref).filter(|stack| plain(stack)).cloned() else {
        return (0, 0);
    };
    let mut left = stack.count;
    let (mut earned, mut finished) = (0, 0);
    for index in 0..SLOTS {
        let Some(mut request) = get(entity, index) else { continue };
        if request.done() || POOL[request.wish].item != stack.item.as_ref() || left == 0 {
            continue;
        }
        let moved = (request.need - request.have).min(left);
        request.have += moved;
        left -= moved;
        if request.done() {
            earned += reward(request.wish, perks);
            finished += 1;
        }
        put(entity, index, Some(request));
    }
    let used = stack.count - left;
    if used > 0 {
        consume(&mut entity.items[projects::SITE], used);
    }
    (earned, finished)
}
