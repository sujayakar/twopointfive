use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// One step of longTuck's `to`: t[i] toward v by w.
fn tuck_to(t: &mut Vec<f64>, w: f64, i: usize, v: f64) {
    t[i] += (v - t[i]) * w;
}

/// A long carriage's targets pulled toward WALL_TUCK.long by w (0‥1), in place.
pub(super) fn long_tuck(t: &mut Vec<f64>, gun_wall: f64) {
    let w = v3::clamp(gun_wall, 0.0, 1.0);
    let c = lc();
    let wall = wall_tuck();
    let tl = &wall.long;
    tuck_to(t, w, c.side as usize, tl.side);
    tuck_to(t, w, c.up as usize, tl.up);
    tuck_to(t, w, c.fwd as usize, tl.fwd);
    tuck_to(t, w, c.pitch as usize, tl.pitch);
    tuck_to(t, w, c.yaw as usize, tl.yaw);
    tuck_to(t, w, c.aim_k as usize, tl.aim_k);
    for k in 0..3 {
        tuck_to(t, w, c.elbow_r as usize + k, tl.elbow_r[k]);
        tuck_to(t, w, c.elbow_l as usize + k, tl.elbow_l[k]);
    }
    t[c.fwd as usize] -= v3::clamp(gun_wall - 1.0, 0.0, 1.0) * wall.over;
}

/// A read of an ArrayLike of singles, as a double: past its end is undefined, which is NaN in the sum.
fn q_at(a: &[f32], n: f64) -> f64 {
    a.get(n as usize).map_or(m::NAN, |x| *x as f64)
}

/// The angle between the rotation at a[i] (its four numbers `s` apart) and the one at b[k‥k+3].
pub(super) fn q_ang(a: &[f32], i: f64, s: f64, b: &[f32], k: f64) -> f64 {
    let d = m::abs(q_at(a, i) * q_at(b, k) + q_at(a, i + s) * q_at(b, k + 1.0) + q_at(a, i + 2.0 * s) * q_at(b, k + 2.0) + q_at(a, i + 3.0 * s) * q_at(b, k + 3.0));
    if d >= 1.0 { 0.0 } else { 2.0 * m::acos(d) }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn near(a: f64, b: f64, what: &str) { assert!(m::abs(a - b) < 1e-5, "{what}: {a} against {b}"); }

    /// A clip's rotations and a pose's are laid out differently: `s` is how far apart a's four numbers are.
    #[test]
    fn the_angle_between_two_rotations_is_the_turn_from_one_to_the_other() {
        let still = [0.0f32, 0.0, 0.0, 1.0];
        near(q_ang(&still, 0.0, 1.0, &still, 0.0), 0.0, "no turn");
        near(q_ang(&[0.0, 0.0, 0.0, -1.0], 0.0, 1.0, &still, 0.0), 0.0, "the same rotation with its sign flipped is no turn");
        let quarter = [0.0f32, core::f32::consts::FRAC_1_SQRT_2, 0.0, core::f32::consts::FRAC_1_SQRT_2];
        near(q_ang(&quarter, 0.0, 1.0, &still, 0.0), m::PI / 2.0, "a quarter turn");
        let spaced = [9.0f32, 0.0, 9.0, quarter[1], 9.0, 0.0, 9.0, quarter[3]];
        near(q_ang(&spaced, 1.0, 2.0, &still, 0.0), m::PI / 2.0, "the numbers read two apart");
    }

    #[test]
    fn a_gun_at_a_wall_goes_to_the_tuck_by_how_near_it_is_and_then_draws_back() {
        let (c, wall) = (lc(), wall_tuck());
        let tl = &wall.long;
        let start: Vec<f64> = (0..21).map(|i| 0.1 * (i as f64) - 0.7).collect();
        let run = |w: f64| { let mut t = start.clone(); long_tuck(&mut t, w); t };
        assert_eq!(run(0.0), start, "no wall: the carriage stands");
        let (pitch, fwd, elbow, lean) = (c.pitch as usize, c.fwd as usize, c.elbow_r as usize, c.lean as usize);
        let full = run(1.0);
        near(full[pitch], tl.pitch, "at the wall, the tuck's pitch");
        near(full[elbow + 1], tl.elbow_r[1], "and its elbow");
        near(run(0.5)[pitch], (start[pitch] + tl.pitch) / 2.0, "half way there, half way to it");
        assert_eq!(full[lean], start[lean], "what the tuck does not name stays");
        // pushed past it, only the reach goes on changing, and only by the table's most
        let past = run(1.5);
        near(past[fwd], tl.fwd - 0.5 * wall.over, "half way past: half the draw back");
        near(run(9.0)[fwd], tl.fwd - wall.over, "no further than the most");
        near(past[pitch], tl.pitch, "the rest stands at the tuck");
    }
}
