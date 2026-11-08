//! Eight cyclic key poses, interpolated on the render clock (never server ticks).
//! Rotating parts use GPU spin instead of quantizing a turn into eight jumps.
pub(super) const STROKE: [f32; 8] = [1.0, 1.0, 0.94, 0.55, 0.0, 0.0, 0.55, 0.94];
pub(super) const SWEEP: [f32; 8] = [0.0, 0.15, 0.5, 0.85, 1.0, 0.85, 0.5, 0.15];

pub(super) fn sample(keys: &[f32; 8], ticks: i64, partial: f32, period: i64) -> f32 {
    let phase = ((ticks.rem_euclid(period) as f32 + partial).rem_euclid(period as f32)) * 8.0 / period as f32;
    let index = phase.floor() as usize;
    let t = phase.fract();
    let eased = t*t*(3.0-2.0*t);
    keys[index] + (keys[(index+1)%8]-keys[index])*eased
}

pub(super) fn active(block: &str, entity: &super::ModBlockEntity) -> bool {
    super::number(entity, "Status") == 1 && (!super::production::burns(block) || super::number(entity, "Burn") > 0)
}

/// Eight poses along a one-way transfer, including both endpoints.
pub(super) fn travel(t: f32) -> f32 {
    const KEYS: [f32; 8] = [0.0,0.05,0.18,0.38,0.62,0.82,0.95,1.0];
    let scaled=t.clamp(0.0,1.0)*7.0;
    let i=(scaled.floor() as usize).min(6);
    let u=scaled-i as f32;
    KEYS[i]+(KEYS[i+1]-KEYS[i])*u*u*(3.0-2.0*u)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn transfer_pose_path_is_monotone_and_clamps_to_both_endpoints() {
        assert_eq!(travel(-1.0),0.0); assert_eq!(travel(2.0),1.0);
        for step in 0..100 { assert!(travel(step as f32/100.0)<=travel((step+1) as f32/100.0)); }
    }
    #[test]
    fn cyclic_pose_boundaries_are_continuous_and_partial_ticks_interpolate() {
        for keys in [&STROKE, &SWEEP] {
            for tick in -40..80 {
                let start = sample(keys, tick, 0.9999, 32);
                let end = sample(keys, tick+1, 0.0, 32);
                assert!((start-end).abs() < 0.0002);
                for partial in [0.0,0.25,0.5,0.75] { assert!((0.0..=1.0).contains(&sample(keys,tick,partial,32))); }
            }
        }
        assert_ne!(sample(&SWEEP, 1, 0.0,32), sample(&SWEEP, 1,0.5,32));
    }
}
