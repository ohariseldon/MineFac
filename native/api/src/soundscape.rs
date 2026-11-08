//! Cosmetic spatial feedback. Short samples expire naturally; no persistent loop
//! handles, NBT timers, production gating or inventory changes. Category: Blocks.
use super::*;
mod data;

#[derive(Clone, Copy)]
struct Profile {
    family: u8,
    work: Option<&'static str>,
    finish: Option<&'static str>,
    interval: i64,
    volume: f32,
}

struct Listener { dimension: String, pos: [f64; 3] }
struct Voice { dimension: String, pos: [f64; 3], family: u8, tick: u64 }
#[derive(Default)]
struct Scene {
    epoch: Option<u64>,
    tick: u64,
    refreshed: Option<u64>,
    listeners: Vec<Listener>,
    voices: Vec<Voice>,
}
const RANGE: f64 = 12.0; // Conservative listener prefilter; resources attenuate at 8 blocks.
const TAIL: u64 = 20; // Longest clip 0.9s, including the lowest pitch (0.97).
fn distance(a: [f64; 3], b: [f64; 3]) -> f64 {
    (0..3).map(|i| (a[i] - b[i]).powi(2)).sum()
}
impl Scene {
    fn begin(&mut self, epoch: u64, tick: u64) -> bool {
        if self.epoch != Some(epoch) || tick < self.tick { *self = Self::default(); self.epoch = Some(epoch); }
        self.tick = tick;
        self.voices.retain(|v| tick.saturating_sub(v.tick) < TAIL);
        self.refreshed.is_none_or(|last| tick.saturating_sub(last) >= 10)
    }
    fn admit(&mut self, dimension: &str, pos: [f64; 3], family: u8) -> bool {
        // One representative per family in a small cell, <=2 new voices per
        // dimension/tick and <=4 overlapping sounds near each listener.
        let cell = |p: [f64; 3]| p.map(|n| (n / 8.0).floor() as i32);
        if self.voices.len() >= 256 || self.voices.iter().any(|v| v.dimension == dimension && v.family == family && cell(v.pos) == cell(pos)) {
            return false;
        }
        if self.voices.iter().filter(|v| v.dimension == dimension && v.tick == self.tick).count() >= 2 { return false; }
        let mut audible = false;
        for listener in &self.listeners {
            if listener.dimension != dimension || distance(listener.pos, pos) > RANGE * RANGE { continue; }
            audible = true;
            if self.voices.iter().filter(|v| v.dimension == dimension && distance(v.pos, listener.pos) <= RANGE * RANGE).count() >= 4 { return false; }
        }
        if !audible { return false; }
        self.voices.push(Voice { dimension: dimension.into(), pos, family, tick: self.tick });
        true
    }
}
#[cfg(not(test))]
fn scene<T>(run: impl FnOnce(&mut Scene) -> T) -> T {
    static STATE: std::sync::LazyLock<std::sync::Mutex<Scene>> = std::sync::LazyLock::new(|| Default::default());
    run(&mut STATE.lock().unwrap_or_else(|p| p.into_inner()))
}
#[cfg(test)]
fn scene<T>(run: impl FnOnce(&mut Scene) -> T) -> T {
    thread_local! { static STATE: std::cell::RefCell<Scene> = Default::default(); }
    STATE.with(|s| run(&mut s.borrow_mut()))
}

pub(super) fn refresh(host: &dyn Host, tick: u64) {
    let Some(level) = host.level(host.dimension()) else { return; };
    let needed = scene(|s| s.begin(level.epoch().get(), tick));
    if !needed { return; }
    let listeners = host.server_players().into_iter().map(|p| Listener { dimension: p.dimension, pos: p.position }).collect();
    scene(|s| { s.listeners = listeners; s.refreshed = Some(tick); });
}

fn cue(profile: Profile, status: i32, produced: i32, previous: i32, phase: i64) -> Option<&'static str> {
    if produced > previous {
        if let Some(finish) = profile.finish { return Some(finish); }
    }
    if status == 1 && phase.rem_euclid(profile.interval) == 0 { profile.work } else { None }
}

fn position(pos: [i32; 3], entity: &ModBlockEntity) -> [f64; 3] {
    let mut at = pos.map(|n| f64::from(n) + 0.5);
    if entity.id == freight::DRONE {
        if let Some(Tag::IntArray(to)) = entity.nbt.get(nbt_key("FlightTarget")) {
            if to.len() == 3 {
                let progress = (f64::from(number(entity, "Progress")) / f64::from(number(entity, "Duration").max(1))).clamp(0.0, 1.0);
                let fraction = match number(entity, "FlightPhase") { 1 => progress, 2 => 1.0 - progress, _ => 0.0 };
                for axis in 0..3 { at[axis] += (f64::from(to[axis]) - f64::from(pos[axis])) * fraction; }
                at[1] += 2.75; // same flight path as freight::draw's rotor
            }
        }
    }
    at
}

pub(super) fn after(host: &mut dyn Host, pos: [i32; 3], entity: &ModBlockEntity, previous: i32, time: i64) {
    // Generator Status also means stored energy; only the production hook can
    // tell whether the motor actually generated this tick.
    if is_power_block(&entity.id) { return; }
    let Some(profile) = data::profile(&entity.id) else { return; };
    // Stagger repeated fragments spatially; no start pulse on every status
    // change (that would stutter on fast recipes or intermittent power).
    let hash = i64::from(pos[0]) * 7 + i64::from(pos[1]) * 3 + i64::from(pos[2]) * 11;
    let Some(event) = cue(profile, number(entity, "Status"), number(entity, "Produced"), previous, time.wrapping_add(hash)) else { return; };
    play(host, pos, entity, profile, event, hash);
}

fn play(host: &mut dyn Host, pos: [i32; 3], entity: &ModBlockEntity, profile: Profile, event: &str, hash: i64) {
    if scene(|s| s.listeners.is_empty()) { return; }
    let at = position(pos, entity);
    if scene(|s| s.admit(host.dimension(), at, profile.family)) {
        let pitch = 0.97 + hash.rem_euclid(7) as f32 * 0.01;
        host.play_sound(event, at, 4, profile.volume, pitch);
    }
}

/// Called immediately after generation, before energy distribution. Stored
/// energy alone must never leave an empty burner humming indefinitely.
pub(super) fn generated(host: &mut dyn Host, pos: [i32; 3], entity: &ModBlockEntity, generated: bool) {
    if !generated { return; }
    let Some(profile) = data::profile(&entity.id) else { return; };
    let hash = i64::from(pos[0]) * 7 + i64::from(pos[1]) * 3 + i64::from(pos[2]) * 11;
    let time = scene(|s| s.tick as i64);
    if let Some(event) = cue(profile, 1, 0, 0, time.wrapping_add(hash)) {
        play(host, pos, entity, profile, event, hash);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn listener(x: f64) -> Listener { Listener { dimension: "overworld".into(), pos: [x, 0.0, 0.0] } }
    #[test]
    fn idle_blocked_unpowered_and_paused_are_silent() {
        let p = data::profile("veio:press").unwrap();
        for status in [0, 2, 3, 5, 9, 13] { assert_eq!(cue(p, status, 7, 7, 0), None); }
        assert_eq!(cue(p, 1, 7, 7, 0), p.work);
        assert_eq!(cue(p, 1, 7, 7, 1), None);
        assert_eq!(cue(p, 0, 8, 7, 1), p.finish);
        let arm = data::profile("veio:inserter").unwrap();
        assert_eq!(cue(arm, 1, 0, 0, 0), None);
        assert_eq!(cue(arm, 1, 1, 0, 3), arm.finish);
    }
    #[test]
    fn dense_factories_share_voices_and_expire_without_stop_packets() {
        let mut s = Scene::default(); s.begin(1, 0); s.listeners = vec![listener(4.0)];
        assert!(s.admit("overworld", [0.0; 3], 0));
        assert!(!s.admit("overworld", [1.0; 3], 0));
        assert!(s.admit("overworld", [1.0; 3], 1));
        assert!(!s.admit("overworld", [1.0; 3], 2)); // per-tick cap
        s.begin(1, 1);
        assert!(s.admit("overworld", [1.0; 3], 2));
        assert!(s.admit("overworld", [1.0; 3], 3));
        s.begin(1, 2);
        for family in 4..100 { assert!(!s.admit("overworld", [1.0; 3], family)); }
        s.begin(1, TAIL + 1); assert!(s.voices.is_empty());
        assert!(s.admit("overworld", [0.0; 3], 0));
    }
    #[test]
    fn distant_players_dimensions_and_world_changes_are_isolated() {
        let mut s = Scene::default(); s.begin(1, 100); s.listeners = vec![listener(0.0), listener(100.0)];
        assert!(!s.admit("nether", [0.0; 3], 0));
        assert!(!s.admit("overworld", [50.0; 3], 0));
        assert!(s.admit("overworld", [0.0; 3], 0));
        assert!(s.admit("overworld", [100.0, 0.0, 0.0], 0));
        assert!(s.begin(2, 101)); assert!(s.listeners.is_empty()); assert!(s.voices.is_empty());
        s.listeners = vec![listener(0.0)]; s.begin(2, 0); assert!(s.listeners.is_empty());
    }
    #[test]
    fn emitted_feedback_is_positional_in_blocks_category_and_drone_tracks_flight() {
        #[derive(Default)]
        struct Probe { calls: Vec<(String, [f64; 3], usize, f32, f32)> }
        impl Host for Probe {
            fn dimension(&self) -> &'static str { "overworld" }
            fn play_sound(&mut self, event: &str, at: [f64; 3], category: usize, volume: f32, pitch: f32) -> bool {
                self.calls.push((event.into(), at, category, volume, pitch)); true
            }
        }
        let mut host = Probe::default();
        let mut entity = ModBlockEntity::default(); entity.id = PRESS.into();
        set_number(&mut entity, "Status", 1);
        after(&mut host, [0; 3], &entity, 0, 0);
        assert!(host.calls.is_empty()); // No listener, no sound host calls.
        scene(|s| { s.begin(1, 0); s.listeners = vec![listener(0.0)]; });
        after(&mut host, [0; 3], &entity, 0, 0);
        assert_eq!(host.calls.len(), 1);
        let (event, at, category, volume, pitch) = &host.calls[0];
        assert_eq!(event, "veio:machine.press"); assert_eq!(*at, [0.5; 3]);
        assert_eq!(*category, 4); assert!(*volume <= 0.3); assert!((0.97..=1.03).contains(pitch));
        entity.id = freight::DRONE.into();
        entity.nbt.insert(nbt_key("FlightTarget").clone(), Tag::IntArray(vec![16, 64, 0]));
        set_number(&mut entity, "Duration", 40); set_number(&mut entity, "Progress", 10);
        set_number(&mut entity, "FlightPhase", 1);
        assert_eq!(position([0,64,0], &entity), [4.5,67.25,0.5]);
        set_number(&mut entity, "FlightPhase", 2);
        assert_eq!(position([0,64,0], &entity), [12.5,67.25,0.5]);
    }

    #[test]
    fn stored_energy_does_not_keep_an_empty_generator_humming() {
        #[derive(Default)]
        struct Probe { sounds: usize }
        impl Host for Probe {
            fn change_mod_block_entity(&mut self, _: [i32; 3], _: &mut dyn FnMut(&mut ModBlockEntity)) -> bool { false }
            fn dimension(&self) -> &'static str { "overworld" }
            fn play_sound(&mut self, _: &str, _: [f64; 3], _: usize, _: f32, _: f32) -> bool {
                self.sounds += 1; true
            }
        }
        let mut host = Probe::default();
        scene(|s| { s.begin(1, 0); s.listeners = vec![listener(0.0)]; });
        let mut generator = ModBlockEntity::default(); generator.id = GENERATOR.into();
        generator.items = vec![None; layout(GENERATOR).unwrap().0];
        set_number(&mut generator, "energy", 1000);
        set_number(&mut generator, "Status", 1);
        tick_power(&mut host, [0; 3], &mut generator);
        after(&mut host, [0; 3], &generator, 0, 0);
        assert_eq!(host.sounds, 0);
        add(&mut generator.items[0], "minecraft:coal", 1);
        tick_power(&mut host, [0; 3], &mut generator);
        assert_eq!(host.sounds, 1);
    }

}
