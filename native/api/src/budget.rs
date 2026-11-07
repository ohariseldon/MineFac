//! Shared, deterministic admission budgets for background factory work.
//! These are work quotas, not a wall-clock/FPS guarantee. Never gate transfers.
use std::collections::{HashMap, VecDeque};

#[derive(Clone, Copy)]
pub(super) enum Lane { Area, Terrain, Topology, Planning, Workforce }
// Independent reservations prevent cheap scans from starving network maintenance.
const LIMITS: [u32; 5] = [512, 384, 4096, 4, 512];
// Area/terrain/topology retry every tick; foremen are called less frequently.
const STALE: [u64; 5] = [2, 2, 2, 20, 20];
type Key = (&'static str, [i32; 3]);

#[derive(Default)]
struct Queue {
    used: u32,
    order: VecDeque<Key>,
    seen: HashMap<Key, u64>,
}

#[derive(Default)]
struct Scheduler {
    tick: Option<u64>,
    queues: [Queue; 5],
}

impl Scheduler {
    fn begin(&mut self, tick: u64) {
        if self.tick == Some(tick) { return; }
        if self.tick.is_some_and(|old| tick < old) { *self = Self::default(); }
        self.tick = Some(tick);
        for (index, queue) in self.queues.iter_mut().enumerate() {
            queue.used = 0;
            // Polling tasks renew their place. Removed/unloaded tasks cannot
            // hold the front forever. There is one entry per pending block,
            // never one per denied call or per skipped simulation tick.
            while let Some(key) = queue.order.front() {
                if queue.seen.get(key).is_some_and(|seen| tick.saturating_sub(*seen) <= STALE[index]) { break; }
                queue.seen.remove(key);
                queue.order.pop_front();
            }
        }
    }

    fn request(&mut self, lane: Lane, key: Key, cost: u32) -> bool {
        // Before the first simulation tick (also in offline fixtures), inactive.
        let Some(tick) = self.tick else { return true; };
        let index = lane as usize;
        if cost == 0 || cost > LIMITS[index] { return false; }
        let queue = &mut self.queues[index];
        if let Some(seen) = queue.seen.get_mut(&key) { *seen = tick; }
        if queue.used + cost <= LIMITS[index] && queue.order.front().is_none_or(|first| *first == key) {
            queue.used += cost;
            if queue.order.front() == Some(&key) {
                queue.order.pop_front();
                queue.seen.remove(&key);
            }
            return true;
        }
        if !queue.seen.contains_key(&key) {
            queue.seen.insert(key, tick);
            queue.order.push_back(key);
        }
        false
    }
}

#[cfg(not(test))]
fn with_scheduler<T>(run: impl FnOnce(&mut Scheduler) -> T) -> T {
    static STATE: std::sync::LazyLock<std::sync::Mutex<Scheduler>> =
        std::sync::LazyLock::new(|| std::sync::Mutex::new(Scheduler::default()));
    // Recover quota state after a panic rather than stopping all future work.
    run(&mut STATE.lock().unwrap_or_else(|poisoned| poisoned.into_inner()))
}

#[cfg(test)]
fn with_scheduler<T>(run: impl FnOnce(&mut Scheduler) -> T) -> T {
    // Each test has its own simulated engine clock and cannot throttle another.
    thread_local! { static STATE: std::cell::RefCell<Scheduler> = Default::default(); }
    STATE.with(|state| run(&mut state.borrow_mut()))
}

pub(super) fn begin_tick(tick: u64) { with_scheduler(|state| state.begin(tick)); }
pub(super) fn request(lane: Lane, dimension: &'static str, pos: [i32; 3], cost: u32) -> bool {
    with_scheduler(|state| state.request(lane, (dimension, pos), cost))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thousand_jobs_share_a_hard_quota_and_all_make_progress_in_fixed_order() {
        let mut state = Scheduler::default();
        let mut served = [0; 1000];
        for tick in 0..100 {
            state.begin(tick);
            for (id, count) in served.iter_mut().enumerate() {
                if state.request(Lane::Area, ("overworld", [id as i32, 0, 0]), 8) { *count += 1; }
            }
            assert!(state.queues[0].used <= 512);
            assert!(state.queues[0].seen.len() <= 1000);
        }
        assert!(served.iter().all(|count| *count > 0));
        assert!(served.iter().max().unwrap() - served.iter().min().unwrap() <= 1);
    }

    #[test]
    fn dimensions_share_limits_same_tick_does_not_refill_and_lanes_do_not_starve() {
        let mut state = Scheduler::default();
        state.begin(10);
        assert!(state.request(Lane::Area, ("overworld", [0; 3]), 512));
        state.begin(10);
        assert!(!state.request(Lane::Area, ("nether", [0; 3]), 8));
        assert!(state.request(Lane::Topology, ("nether", [0; 3]), 1));
        assert!(state.request(Lane::Planning, ("overworld", [0; 3]), 1));
        state.begin(11);
        assert!(state.request(Lane::Area, ("nether", [0; 3]), 8));
    }

    #[test]
    fn unloaded_head_expires_clock_rewind_clears_waiters_and_retries_are_deduplicated() {
        let mut state = Scheduler::default();
        state.begin(1);
        assert!(state.request(Lane::Terrain, ("world", [0; 3]), 384));
        for id in 1..10000 { state.request(Lane::Terrain, ("world", [id, 0, 0]), 8); }
        assert_eq!(state.queues[1].seen.len(), 9999);
        for _ in 0..100 { state.request(Lane::Terrain, ("world", [1, 0, 0]), 8); }
        assert_eq!(state.queues[1].seen.len(), 9999);
        state.begin(62);
        assert!(state.request(Lane::Terrain, ("world", [99999, 0, 0]), 8));
        assert!(state.queues[1].order.is_empty());
        state.begin(0);
        assert!(state.queues.iter().all(|queue| queue.order.is_empty() && queue.used == 0));
        assert!(!state.request(Lane::Terrain, ("world", [0; 3]), 385));
    }
}
