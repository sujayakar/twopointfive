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

/// Is the stagger's fold up: `too` is the figure's own foldToo.
pub(super) fn fold_on(g: &Game, too: Option<bool>) -> bool {
    let too = too.unwrap_or(false);
    g.k.moves.clutch > 0.5 && (g.k.moves.fold > 0.5 || (g.k.moves.fold == 0.0 && too))
}

/// One part of the motion pass: on?
pub(super) fn mv_on(g: &Game, k: crate::character::MvOnK) -> bool {
    use crate::character::MvOnK;
    let t = &g.k.moves;
    let v = match k {
        MvOnK::On => return true,
        MvOnK::Reload => t.reload,
        MvOnK::Fold => t.fold,
        MvOnK::Steady => t.steady,
        MvOnK::Toss => t.toss,
        MvOnK::Entry => t.entry,
        MvOnK::Flinch => t.flinch,
        MvOnK::Fall => t.fall,
        MvOnK::Squeeze => t.squeeze,
        MvOnK::Snap => t.snap,
        MvOnK::Recoil => t.recoil,
        MvOnK::Dazzle => t.dazzle,
        MvOnK::Gait => t.gait,
        MvOnK::Bound => t.bound,
        MvOnK::Clutch => t.clutch,
        MvOnK::EntryEyes => t.entry_eyes,
        MvOnK::SteadyRun => t.steady_run,
    };
    v > 0.5
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

/// MOVES.entry's lane: something (dx, dz) from a man stands before his muzzle.
pub fn in_dip_lane(dx: f64, dz: f64, gun_yaw: f64) -> bool {
    let l = MV2.dip.lane;
    let fx = m::sin(gun_yaw);
    let fz = m::cos(gun_yaw);
    let along = dx * fx + dz * fz;
    along > l[0] && along < l[1] && m::abs(dx * fz - dz * fx) < l[2]
}

#[cfg(test)]
mod tests {
    use super::*;

    // The expected bits are the TypeScript's own, run under bun with EXACT.on, over Float32Arrays as every caller's are.
    #[test]
    fn q_ang_has_the_typescripts_bits() {
        let a = [0.1f32, 0.0, 0.2, 0.0, 0.3, 0.0, 0.9273618495495703, 0.0];
        let b = [0.5f32, -0.5, 0.5, 0.5];
        assert_eq!(q_ang(&a, 0.0, 2.0, &b, 0.0).to_bits(), 0x3fff1a4dbe042acd);
        let c = [9.0f32, 0.2, 0.4, 0.4, 0.7999999999999999];
        let e = [7.0f32, 0.1, 0.7, 0.1, 0.6999999999999999];
        assert_eq!(q_ang(&c, 1.0, 1.0, &e, 1.0).to_bits(), 0x3fecdd9f8f922ea1);
        assert_eq!(q_ang(&[0.0, 0.0, 0.0, 1.0], 0.0, 1.0, &[0.0, 0.0, 0.0, 1.0], 0.0).to_bits(), 0);
        assert_eq!(q_ang(&[0.0, 0.0, 0.0, -0.5], 0.0, 1.0, &[0.1, 0.2, 0.3, 0.9], 0.0).to_bits(), 0x4001aa1c6937ac15);
    }

    #[test]
    fn long_tuck_has_the_typescripts_bits() {
        let run = |gun_wall: f64| -> Vec<u64> {
            let mut t: Vec<f64> = (0..21).map(|i| 0.1 * (i as f64) - 0.7).collect();
            long_tuck(&mut t, gun_wall);
            t.iter().map(|x| x.to_bits()).collect()
        };
        let part: [u64; 21] = [0xbfe010cb295e9e1b, 0xbfddfe5c91d14e3c, 0xbfcf8ef34d6a161e, 0xc03c5f3b645a1cab, 0xbfc83126e978d4fc, 0x4015b22d0e560418, 0xbfb9999999999990, 0x3ca0000000000000, 0x3fb99999999999a0, 0x3fc999999999999c, 0x3fd3333333333334, 0xbf9a1cac083126d0, 0xbfac28f5c28f5c10, 0x3fcb126e978d4fe2, 0x3fe5374bc6a7ef9f, 0x3fc126e978d4fdf4, 0x3fde5e353f7ced93, 0x3ff0000000000001, 0x3ff199999999999a, 0x3ff3333333333334, 0x3ff4cccccccccccd];
        assert_eq!(run(0.37), part);
        let over: [u64; 21] = [0xbfc51eb851eb8520, 0xbfcf5c28f5c28f5c, 0x3fbced916872b020, 0xc053000000000000, 0x0000000000000000, 0x402e000000000000, 0xbfb9999999999990, 0x3ca0000000000000, 0x3fb99999999999a0, 0x3fc999999999999c, 0x3fd3333333333334, 0xbfe8000000000000, 0xbff0000000000000, 0xbfdccccccccccccc, 0x3fe3333333333333, 0xbff0000000000000, 0xbfd0000000000000, 0x3ff0000000000001, 0x3ff199999999999a, 0x3ff3333333333334, 0x3ff4cccccccccccd];
        assert_eq!(run(1.6), over);
    }

    #[test]
    fn q_ang_past_the_end_is_nan() {
        assert!(q_ang(&[0.0], 0.0, 1.0, &[0.0, 0.0, 0.0, 1.0], 0.0).is_nan());
    }

    #[test]
    fn dip_lane_answers_as_the_typescript() {
        let cases = [
            (0.0, 1.0, 0.0, true),
            (0.3, 1.0, 0.2, true),
            (0.34, 1.0, 0.0, false),
            (1.0, 1.0, 0.7853981633974483, true),
            (0.0, 0.25, 0.0, false),
            (0.0, 1.9, 0.0, false),
            (-1.0, 0.5, -1.5707963267948966, false),
        ];
        for (dx, dz, y, want) in cases {
            assert_eq!(in_dip_lane(dx, dz, y), want);
        }
    }

}
