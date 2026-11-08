// Finite-world geometry and the active world's persisted contract.
// This module is injected into horizon-world so generation and gameplay share
// the same state. An unmarked save is always outside this feature.
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};

pub const FORMAT: u64 = 1;
pub const FILE: &str = "data/veio/world-plan.json";
pub const SIDE: f64 = 15_000.0;
pub const PORTAL_MARGIN: f64 = 24.0;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dimension { Overworld, Nether, End }

impl Dimension {
    pub fn of(id: &str) -> Option<Self> {
        match id {
            "minecraft:overworld" => Some(Self::Overworld),
            "minecraft:the_nether" => Some(Self::Nether),
            "minecraft:the_end" => Some(Self::End),
            _ => None,
        }
    }
    pub fn id(self) -> &'static str {
        match self {
            Self::Overworld => "minecraft:overworld",
            Self::Nether => "minecraft:the_nether",
            Self::End => "minecraft:the_end",
        }
    }
    pub fn bounds(self) -> Bounds {
        let half = if self == Self::Nether { SIDE / 16.0 } else { SIDE / 2.0 };
        Bounds { min: -half, max: half }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Bounds { pub min: f64, pub max: f64 }

impl Bounds {
    pub fn walls(self) -> (i32, i32) { (self.min.ceil() as i32 - 1, self.max.floor() as i32) }
    pub fn contains(self, x: f64, z: f64, margin: f64) -> bool {
        x.is_finite() && z.is_finite() && margin.is_finite() && margin >= 0.0
            && x >= self.min + margin && x < self.max - margin
            && z >= self.min + margin && z < self.max - margin
    }
    pub fn contains_box(self, min: [i32; 3], max: [i32; 3], margin: f64) -> bool {
        self.contains(f64::from(min[0]), f64::from(min[2]), margin)
            && self.contains(f64::from(max[0]) + 1.0, f64::from(max[2]) + 1.0, margin)
    }
    pub fn clamp(self, x: f64, z: f64, margin: f64) -> Option<[f64; 2]> {
        if !x.is_finite() || !z.is_finite() || !margin.is_finite() || margin < 0.0
            || self.min + margin >= self.max - margin { return None; }
        let lo = self.min + margin;
        let hi = self.max - margin - 1.0e-5;
        Some([x.clamp(lo, hi), z.clamp(lo, hi)])
    }
    pub fn shell(self, x: i32, z: i32) -> bool {
        let (lo, hi) = self.walls();
        (x == lo || x == hi) && (lo..=hi).contains(&z)
            || (z == lo || z == hi) && (lo..=hi).contains(&x)
    }
    pub fn column(self, x: i32, z: i32) -> bool {
        let (lo, hi) = self.walls();
        (lo..=hi).contains(&x) && (lo..=hi).contains(&z)
    }
    pub fn chunk(self, x: i32, z: i32, halo: i32) -> bool {
        let (lo, hi) = self.walls();
        let (x, z) = (i64::from(x) * 16, i64::from(z) * 16);
        let (lo, hi) = (i64::from(lo - halo), i64::from(hi + halo));
        x <= hi && x + 15 >= lo && z <= hi && z + 15 >= lo
    }
}

#[derive(Clone, Debug)]
pub struct Session { pub root: PathBuf, pub seed: i64, pub plan: Arc<serde_json::Value> }
static SESSION: RwLock<Option<Arc<Session>>> = RwLock::new(None);

pub fn activate(root: &Path, plan: Option<serde_json::Value>) {
    *SESSION.write().unwrap_or_else(|p| p.into_inner()) = plan.map(|plan| Arc::new(Session { root: root.to_owned(), seed: plan["seed"].as_i64().expect("validated world plan"), plan: Arc::new(plan) }));
}
pub fn session() -> Option<Arc<Session>> {
    SESSION.read().unwrap_or_else(|p| p.into_inner()).clone()
}
pub fn active(seed: i64, dimension: &str) -> Option<Bounds> {
    session().filter(|s| s.seed == seed)?;
    Some(Dimension::of(dimension)?.bounds())
}
pub fn for_world(root: &Path, dimension: &str) -> Option<Bounds> {
    session().filter(|s| s.root == root)?;
    Some(Dimension::of(dimension)?.bounds())
}

/// Detect player data as well as level metadata. Missing/corrupt level.dat in
/// an old world must never turn it into a new finite world.
pub fn existing(root: &Path) -> std::io::Result<bool> {
    for name in ["level.dat", "level.dat_old", "level.dat_mcr", "region", "entities", "playerdata", "DIM-1", "DIM1"] {
        if root.join(name).try_exists()? { return Ok(true); }
    }
    Ok(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn partial_chunks_and_all_corners_preserve_the_exact_overworld() {
        let b = Dimension::Overworld.bounds();
        for x in [-7500, -7499, 0, 7499] {
            for z in [-7500, -7499, 0, 7499] {
                assert!(b.contains(x as f64, z as f64, 0.0));
                assert!(!b.shell(x, z));
                assert!(b.chunk(x.div_euclid(16), z.div_euclid(16), 0));
            }
        }
        for (x, z) in [(-7501, -7501), (-7501, 7500), (7500, -7501), (7500, 7500), (-7501, 0), (7500, 0)] {
            assert!(b.shell(x, z));
        }
        assert!(!b.chunk(-470, 0, 0));
        assert!(!b.chunk(469, 0, 0));
        assert!(b.chunk(-469, 0, 0));
        assert!(b.chunk(468, 0, 0));
    }
    #[test]
    fn portal_targets_stay_inside_both_dimensions_at_every_edge() {
        for source in [Dimension::Overworld, Dimension::Nether] {
            let destination = if source == Dimension::Nether { Dimension::Overworld } else { Dimension::Nether };
            let scale = if source == Dimension::Nether { 8.0 } else { 0.125 };
            let b = source.bounds();
            for x in [b.min, b.min + 1.0, 0.0, b.max - 0.001] {
                for z in [b.min, b.min + 1.0, 0.0, b.max - 0.001] {
                    let [x, z] = destination.bounds().clamp(x * scale, z * scale, PORTAL_MARGIN).unwrap();
                    assert!(destination.bounds().contains(x, z, PORTAL_MARGIN));
                }
            }
        }
        assert!(Dimension::Nether.bounds().clamp(f64::NAN, 0.0, 1.0).is_none());
        assert!(!Dimension::Overworld.bounds().contains(f64::INFINITY, 0.0, 0.0));
    }
    #[test]
    fn fractional_nether_edges_do_not_round_the_world_up() {
        let b = Dimension::Nether.bounds();
        assert!(b.contains(-937.5, 937.499, 0.0));
        assert!(!b.contains(-937.501, 0.0, 0.0));
        assert!(!b.contains(937.5, 0.0, 0.0));
        assert!(b.shell(-938, 0));
        assert!(b.shell(937, 0));
        assert!(!b.column(938, 0));
    }
}
