//! À noite a vila se recolhe, e os moradores com nome também: o prefeito e os vizinhos (o favor e
//! os pedidos da vila) são aldeões parados, sem a rotina vanilla de dormir. Ao anoitecer cada um
//! vai para a cama mais perto de onde trabalha (procurada uma vez por mundo, uma cama por pessoa);
//! ao amanhecer volta ao lugar de trabalho. Sem cama por perto, fica onde está.
use super::*;
use std::collections::HashMap;

/// Até onde (blocos) procurar a cama em volta do lugar de trabalho.
const REACH: i32 = 28;
const DEPTH: i32 = 5;

#[derive(Default)]
struct State {
    night: Option<bool>,
    /// Cama de cada morador (tipo da entidade): `None` ainda não procurou; `Some(None)` não achou.
    beds: HashMap<&'static str, Option<[i32; 3]>>,
}

static STATE: Mutex<Option<State>> = Mutex::new(None);

pub(super) fn reset() { if let Ok(mut state) = STATE.lock() { *state = None; } }

/// Os moradores com nome: (tipo, id no mundo, lugar de trabalho).
fn residents(host: &dyn Host) -> Vec<(&'static str, Option<u32>, Option<[f64; 3]>)> {
    let mut list = vec![(intro::MAYOR, intro::mayor_id(), intro::plaza_spot(host))];
    list.extend(favors::residents(host));
    list.extend(errands::residents(host));
    list
}

/// A cama mais perto de `at`, fora as já tomadas. Lê os blocos uma vez por morador por mundo.
pub(super) fn bed_near(host: &dyn Host, at: [f64; 3], taken: &[[i32; 3]]) -> Option<[i32; 3]> {
    let [x, y, z] = at.map(|v| v.floor() as i32);
    let mut best: Option<(i32, [i32; 3])> = None;
    for dy in -DEPTH..=DEPTH {
        for dz in -REACH..=REACH {
            for dx in -REACH..=REACH {
                let pos = [x + dx, y + dy, z + dz];
                if !host.block(pos).ends_with("_bed") { continue; }
                // As duas metades da mesma cama contam como uma.
                if taken.iter().any(|t| (t[0] - pos[0]).abs() <= 1 && (t[2] - pos[2]).abs() <= 1 && t[1] == pos[1]) { continue; }
                let d = dx * dx + dz * dz + 4 * dy * dy;
                if best.is_none_or(|(known, _)| d < known) { best = Some((d, pos)); }
            }
        }
    }
    best.map(|(_, pos)| pos)
}

fn teleport(host: &mut dyn Host, kind: &str, [x, y, z]: [f64; 3]) {
    host.run_command(&format!("execute in {} run tp @e[type={kind},limit=1] {x:.2} {y:.2} {z:.2}", workforce::spawn::OVERWORLD));
}

/// A cada 5 s (relógio do tutorial): no anoitecer, todos para a cama; no amanhecer, ao trabalho.
pub(super) fn tick(host: &mut dyn Host, now: u64) {
    if now % 100 != 0 || workforce::spawn::center().is_none() { return; }
    let night = workforce::night(host);
    let Ok(mut slot) = STATE.lock() else { return };
    let state = slot.get_or_insert_with(State::default);
    if state.night == Some(night) { return; }
    let first = state.night.is_none();
    state.night = Some(night);
    // Abrindo o mundo de dia, ninguém se mexe; abrindo de noite, todos vão para a cama.
    if first && !night { return; }
    let people = residents(host);
    let mut taken: Vec<[i32; 3]> = state.beds.values().flatten().copied().collect();
    for (kind, id, work) in people {
        if id.is_none() { continue; }
        if night {
            let Some(work) = work else { continue };
            let bed = *state.beds.entry(kind).or_insert_with(|| {
                let found = bed_near(host, work, &taken);
                if let Some(pos) = found { taken.push(pos); }
                found
            });
            if let Some([bx, by, bz]) = bed {
                teleport(host, kind, [f64::from(bx) + 0.5, f64::from(by) + 0.6, f64::from(bz) + 0.5]);
            }
        } else if let Some(work) = work {
            teleport(host, kind, work);
        }
    }
}
