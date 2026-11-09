//! Módulos de melhoria: um por máquina, colocado com o botão direito.
//! Cada um resolve um gargalo e cria outro, como no Factorio.
use super::*;

pub(super) const SPEED: &str = "veio:speed_module";
pub(super) const EFFICIENCY: &str = "veio:efficiency_module";
pub(super) const PRODUCTIVITY: &str = "veio:productivity_module";
pub(super) const MODULES: [&str; 3] = [SPEED, EFFICIENCY, PRODUCTIVITY];
/// Kit Mk.II: troca o motor de uma máquina de receita ou extratora. Fica ao lado do módulo.
pub(super) const MK2_KIT: &str = "veio:mk2_kit";
/// No painel (slot 11), somado ao número do módulo quando a máquina é Mk.II.
pub(super) const MK2_FLAG: i32 = 16;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct Effect {
    /// Multiplica a duração do ciclo.
    pub time: f32,
    /// Multiplica a energia por tick.
    pub power: f32,
    /// Multiplica a poluição.
    pub pollution: f32,
    /// Porcentagem de produto extra.
    pub bonus: i32,
}

const NONE: Effect = Effect { time: 1.0, power: 1.0, pollution: 1.0, bonus: 0 };

/// Velocidade: 40% menos tempo de ciclo, gasta 80% mais por tick (8% mais por produto) e polui 50% mais.
/// Eficiência: 40% menos energia e metade da poluição.
/// Produtividade: um lote extra a cada cinco ciclos, ciclos 15% mais longos e 40% mais energia.
pub(super) fn effect_of(module: i32) -> Effect {
    match module {
        1 => Effect { time: 0.6, power: 1.8, pollution: 1.5, bonus: 0 },
        2 => Effect { time: 1.0, power: 0.6, pollution: 0.5, bonus: 0 },
        3 => Effect { time: 1.15, power: 1.4, pollution: 1.2, bonus: 20 },
        _ => NONE,
    }
}

/// O módulo da máquina e, por cima, a casa e a comida do jogador (`home::time_factor`), que só
/// encurtam o ciclo: a energia por tick fica igual, então cada produto sai mais barato.
pub(super) fn effect(entity: &ModBlockEntity) -> Effect {
    effect_as(entity, false)
}

/// O mesmo, com a pesquisa contínua (`research::boost`): `extractor` troca a Usinagem pela
/// Perfuração no ritmo.
pub(super) fn effect_as(entity: &ModBlockEntity, extractor: bool) -> Effect {
    let mut effect = research::boost(with_tier(effect_of(number(entity, "Module")), number(entity, "Tier")), extractor);
    effect.time *= home::time_factor(defense::clock());
    effect
}

/// Mk.II: ciclo na metade com o dobro de energia por tick (a mesma energia por produto) e
/// 25% mais poluição. Soma-se ao módulo: uma máquina rende o que duas rendiam.
pub(super) fn with_tier(effect: Effect, tier: i32) -> Effect {
    if tier < 2 { return effect; }
    Effect { time: effect.time * 0.5, power: effect.power * 2.0, pollution: effect.pollution * 1.25, ..effect }
}

pub(super) fn index(item: &str) -> Option<i32> {
    MODULES.iter().position(|module| *module == item).map(|index| index as i32 + 1)
}

/// Máquinas de receita e extratoras aceitam módulos.
pub(super) fn supports(block: &str) -> bool {
    matches!(block, DRILL | DRILL_MK2 | PRESS | ASSEMBLER | CUTTER | INTEGRATOR | CRUSHER | ALLOY | ELECTRONICS)
        || production::accepts_modules(block)
}

pub(super) fn apply(duration: i32, draw: i32, effect: Effect) -> (i32, i32) {
    (((duration as f32 * effect.time).round() as i32).max(1), ((draw as f32 * effect.power).round() as i32).max(1))
}

/// Frações pertencem ao produto: não financiar netherita com ciclos de pedra.
fn accumulated(entity: &ModBlockEntity, item: &str) -> i32 {
    with_nbt_key("BonusItem", |key| {
        if matches!(entity.nbt.get(key), Some(Tag::String(old)) if java_eq(old, item)) {
            number(entity, "Bonus").clamp(0, 99)
        } else { 0 }
    })
}

/// Reserva espaço para o lote inteiro, incluindo o bônus devido neste ciclo.
/// Consulta pura: esperar por saída ou energia não pode gastar frações.
pub(super) fn output_amount(entity: &ModBlockEntity, item: &str, amount: u32) -> u32 {
    let bonus = effect(entity).bonus;
    if bonus > 0 && accumulated(entity, item) + bonus >= 100 { amount * 2 }
    else { amount }
}

/// Só chamar ao concluir um ciclo cuja saída inteira já cabe.
pub(super) fn bonus_ready(entity: &mut ModBlockEntity, item: &str) -> bool {
    let bonus = effect(entity).bonus;
    if bonus == 0 { return false; }
    let total = accumulated(entity, item) + bonus;
    with_nbt_key("BonusItem", |key| {
        if !matches!(entity.nbt.get(key), Some(Tag::String(old)) if java_eq(old, item)) {
            entity.nbt.insert(key.clone(), Tag::String(item.into()));
        }
    });
    set_number(entity, "Bonus", total % 100);
    total >= 100
}
