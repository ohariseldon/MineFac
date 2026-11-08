//! Elevador de carga: a alternativa às escadas para levar itens entre andares.
//!
//! Empilhe elevadores numa coluna. Cada bloco passa um item a cada `STEP` ticks ao vizinho de
//! cima (ou de baixo, se o elevador estiver em descida). O último da coluna entrega pela frente,
//! como uma esteira: numa esteira, máquina, baú ou outro elevador. Entra item por qualquer face:
//! esteiras, braços, máquinas e funis alimentam o pé da coluna. Sem energia, como as esteiras;
//! um sinal de redstone pausa o bloco.
//!
//! Chave com o jogador agachado: alterna subida e descida (`Down`). Chave normal gira a saída.
use super::*;

pub(crate) const LIFT: &str = "veio:cargo_lift";
/// Um item a cada 4 ticks: 5 itens/s, 300/min, a vazão de uma esteira rápida.
pub(crate) const STEP: i64 = 4;

pub(crate) fn owns(block: &str) -> bool { block == LIFT }

pub(crate) fn descending(entity: &ModBlockEntity) -> bool { number(entity, "Down") == 1 }

/// Para onde o item vai agora: o próximo elevador da coluna ou, no fim dela, a frente.
pub(crate) fn next(host: &dyn Host, pos: [i32; 3], down: bool) -> Option<Direction> {
    let vertical = if down { Direction::Down } else { Direction::Up };
    let [_, dy, _] = vertical.normal();
    if host.block([pos[0], pos[1] + dy, pos[2]]) == LIFT { Some(vertical) } else { logistics::output_direction(host, pos) }
}

pub(crate) fn tick(host: &mut dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity, time: i64) {
    // O painel mostra o sentido pelo slot de dados 12 (`Info`).
    set_number(entity, "Info", i32::from(descending(entity)));
    if host.neighbor_signal(host.dimension(), pos) > 0 {
        set_number(entity, "Status", 9);
        return;
    }
    let Some(stack) = entity.items[0].clone() else {
        set_number(entity, "Status", 0);
        return;
    };
    // Fase pela altura: blocos vizinhos da coluna não disputam o mesmo tick.
    if (time + i64::from(pos[1])).rem_euclid(STEP) != 0 { return; }
    let Some(direction) = next(host, pos, descending(entity)) else { return };
    let moved = send(host, pos, direction, &stack);
    if moved > 0 {
        consume(&mut entity.items[0], moved);
        set_number(entity, "Produced", number(entity, "Produced").saturating_add(moved as i32));
        set_number(entity, "Status", 1);
    } else {
        set_number(entity, "Status", 3);
    }
}

/// Entrega um item ao vizinho na direção dada, inclusive para cima e para baixo.
fn send(host: &mut dyn Host, pos: [i32; 3], direction: Direction, stack: &ItemStack) -> u32 {
    let normal = direction.normal();
    let target = std::array::from_fn(|axis| pos[axis] + normal[axis]);
    let returned = host.insert_item(target, direction.opposite(), ItemStack { count: 1, ..stack.clone() });
    1 - returned.map_or(0, |rest| rest.count.min(1))
}

/// Chave + agachado: inverte o sentido. Devolve `None` para outros cliques.
pub(crate) fn right_click(host: &mut dyn Host, pos: [i32; 3], held: &str) -> Option<Flow> {
    if held != WRENCH || !host.sneaking() { return None; }
    let mut down = false;
    host.change_mod_block_entity(pos, &mut |entity| {
        down = !descending(entity);
        set_number(entity, "Down", i32::from(down));
    });
    host.play_sound("minecraft:block.chain.place", center(pos), 4, 0.6, if down { 0.8 } else { 1.2 });
    Some(Flow::Cancel)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[derive(Default)]
    struct Shaft { blocks: HashMap<[i32; 3], (String, ModBlockEntity)>, facing: HashMap<[i32; 3], String> }

    impl Shaft {
        fn lift(&mut self, pos: [i32; 3], facing: &str) {
            let mut entity = ModBlockEntity::default();
            entity.id = LIFT.into();
            entity.items = vec![None];
            self.blocks.insert(pos, (LIFT.into(), entity));
            self.facing.insert(pos, facing.into());
        }
        fn chest(&mut self, pos: [i32; 3]) {
            let mut entity = ModBlockEntity::default();
            entity.id = "minecraft:chest".into();
            entity.items = vec![None];
            self.blocks.insert(pos, ("minecraft:chest".into(), entity));
        }
        fn run(&mut self, ticks: i64) {
            for time in 0..ticks {
                let positions: Vec<[i32; 3]> = self.blocks.iter().filter(|(_, (id, _))| id == LIFT).map(|(p, _)| *p).collect();
                for pos in positions {
                    let mut entity = self.blocks[&pos].1.clone();
                    tick(self, pos, &mut entity, time);
                    self.blocks.get_mut(&pos).unwrap().1 = entity;
                }
            }
        }
        fn count(&self, pos: [i32; 3]) -> u32 { self.blocks[&pos].1.items[0].as_ref().map_or(0, |s| s.count) }
    }

    impl Host for Shaft {
        fn dimension(&self) -> &'static str { "veio:lift-test" }
        fn block(&self, pos: [i32; 3]) -> String { self.blocks.get(&pos).map_or("minecraft:air".into(), |(id, _)| id.clone()) }
        fn block_property(&self, pos: [i32; 3], key: &str) -> Option<String> { (key == "facing").then(|| self.facing.get(&pos).cloned()).flatten() }
        fn neighbor_signal(&mut self, _: &str, _: [i32; 3]) -> i32 { 0 }
        fn play_sound(&mut self, _: &str, _: [f64; 3], _: usize, _: f32, _: f32) -> bool { true }
        fn insert_item(&mut self, pos: [i32; 3], _: Direction, stack: ItemStack) -> Option<ItemStack> {
            let Some((_, entity)) = self.blocks.get_mut(&pos) else { return Some(stack) };
            if !can_add(&entity.items[0], &stack.item, stack.count) { return Some(stack); }
            add(&mut entity.items[0], &stack.item, stack.count);
            None
        }
    }

    #[test]
    fn a_column_carries_items_up_and_out_the_front_at_fast_belt_speed() {
        let mut shaft = Shaft::default();
        for y in 0..4 { shaft.lift([0, y, 0], "north"); }
        shaft.chest([0, 3, 1]); // facing north: saída ao sul, como as esteiras
        shaft.blocks.get_mut(&[0, 0, 0]).unwrap().1.items[0] = Some(ItemStack::new(PLATE, 40));
        shaft.run(100);
        let delivered = shaft.count([0, 3, 1]);
        assert!((20..=25).contains(&delivered), "5 itens/s: {delivered} em 5 s");
        shaft.run(200);
        let delivered = shaft.count([0, 3, 1]);
        assert_eq!(delivered, 40);
        assert_eq!(delivered + (0..4).map(|y| shaft.count([0, y, 0])).sum::<u32>(), 40, "nenhum item some");
    }

    #[test]
    fn descending_lifts_carry_down_and_a_full_target_holds_without_loss() {
        let mut shaft = Shaft::default();
        for y in 0..3 {
            shaft.lift([0, y, 0], "north");
            set_number(&mut shaft.blocks.get_mut(&[0, y, 0]).unwrap().1, "Down", 1);
        }
        shaft.chest([0, 0, 1]);
        shaft.blocks.get_mut(&[0, 0, 1]).unwrap().1.items[0] = Some(ItemStack::new(PLATE, 60));
        shaft.blocks.get_mut(&[0, 2, 0]).unwrap().1.items[0] = Some(ItemStack::new(PLATE, 10));
        shaft.run(400);
        assert_eq!(shaft.count([0, 0, 1]), 64, "o baú enche");
        assert_eq!((0..3).map(|y| shaft.count([0, y, 0])).sum::<u32>(), 6, "o resto espera na coluna");
        assert_eq!(number(&shaft.blocks[&[0, 0, 0]].1, "Status"), 3);
    }
}
