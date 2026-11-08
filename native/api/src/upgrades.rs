//! Módulos de melhoria: um por máquina, colocado com o botão direito.
//! Cada um resolve um gargalo e cria outro, como no Factorio.
use super::*;

pub(super) const SPEED: &str = "veio:speed_module";
pub(super) const EFFICIENCY: &str = "veio:efficiency_module";
pub(super) const PRODUCTIVITY: &str = "veio:productivity_module";
pub(super) const MODULES: [&str; 3] = [SPEED, EFFICIENCY, PRODUCTIVITY];

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

/// Velocidade: 40% mais rápido, gasta 80% mais por tick (8% mais por produto) e polui 50% mais.
/// Eficiência: 40% menos energia e metade da poluição.
/// Produtividade: um produto extra a cada dez ciclos, ciclos 15% mais longos e 40% mais energia.
pub(super) fn effect_of(module: i32) -> Effect {
    match module {
        1 => Effect { time: 0.6, power: 1.8, pollution: 1.5, bonus: 0 },
        2 => Effect { time: 1.0, power: 0.6, pollution: 0.5, bonus: 0 },
        3 => Effect { time: 1.15, power: 1.4, pollution: 1.2, bonus: 10 },
        _ => NONE,
    }
}

/// O módulo da máquina e, por cima, a casa e a comida do jogador (`home::time_factor`), que só
/// encurtam o ciclo: a energia por tick fica igual, então cada produto sai mais barato.
pub(super) fn effect(entity: &ModBlockEntity) -> Effect {
    let mut effect = effect_of(number(entity, "Module"));
    effect.time *= home::time_factor(defense::clock());
    effect
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

/// Acumula o bônus de produtividade; devolve `true` quando sai um produto extra.
pub(super) fn bonus_ready(entity: &mut ModBlockEntity) -> bool {
    let effect = effect(entity);
    if effect.bonus == 0 {
        return false;
    }
    let bonus = number(entity, "Bonus") + effect.bonus;
    if bonus >= 100 {
        set_number(entity, "Bonus", bonus - 100);
        true
    } else {
        set_number(entity, "Bonus", bonus);
        false
    }
}
