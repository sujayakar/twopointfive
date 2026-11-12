use super::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{string::String, vec, vec::Vec};

/// When key k (0..3) of a code is pressed, on a verb `dur` long.
pub fn work_key_beat(k: f64, dur: f64) -> f64 {
    let work = WORK;
    work.key_lead + (k + 0.5) * (dur - work.key_lead) / 4.0
}

#[cfg(test)]
mod tests {
    use super::*;

    /// His fingers go down the code in order, after the hand has got there, and the last is done before the verb is.
    #[test]
    fn a_codes_keys_fall_in_order_inside_the_verb() {
        for dur in [0.7, 1.3, 2.45] {
            let beats: Vec<f64> = (0..4).map(|k| work_key_beat(k as f64, dur)).collect();
            assert!(beats[0] > WORK.key_lead, "dur {dur}: the first key at {}", beats[0]);
            assert!(beats.windows(2).all(|b| b[1] > b[0]), "dur {dur}: {beats:?}");
            assert!(beats[3] < dur, "dur {dur}: the last key at {}", beats[3]);
        }
    }
}

/// The rate (1/s) Sam's hold layer lets go at once the controller has: slow out of a choke run to its end while he stands idle.
pub(super) fn hold_out_rate(g: &crate::game::Game, last: crate::character::HoldPose, speed: f64, busy: bool) -> f64 {
    out_rate(last, speed, busy, g.m.character.hold_show, &g.k.hold)
}

/// hold_out_rate's choice, over what it reads of the game.
fn out_rate(last: crate::character::HoldPose, speed: f64, busy: bool, show: crate::character::CharacterModHOLDSHOW, hold: &crate::knobs::HoldK) -> f64 {
    if busy || last.variant == Some(crate::character::HoldVariant::Gun) { return 16.0; }
    let slow = if last.phase == crate::character::HoldPhase::Choke && last.t > 0.9 * hold.secs.choke && last.beats.unwrap_or(show.beats) {
        hold.beats.after
    } else {
        16.0
    };
    if slow == 16.0 { 16.0 } else { v3::lerp(slow, 16.0, v3::smoothstep(0.1, 0.6, m::abs(speed))) }
}

/// A goggle nod shown as a lift of the head: as big less a little whichever way the nod went.
pub(super) fn nod_lift(n: f64) -> f64 {
    2.0 * crate::character::DEGR - m::hyp2(n, 2.0 * crate::character::DEGR)
}

/// Model-axis yaw: + turns to the figure's left.
pub(super) fn q_yaw(a: f64) -> Quat {
    v3::quat::axis_angle(crate::character::ay(), a)
}

/// One step toward `target`; w is about 6 / settle time.
pub(super) fn spring_step(this: &mut crate::character::Spring, target: f64, w: f64, dt: f64) -> f64 {
    let y = this.x - target;
    let j = this.v + y * w;
    let e = m::exp(-w * dt);
    this.x = target + (y + j * dt) * e;
    this.v = (this.v - j * w * dt) * e;
    this.x
}

/// Catmull-Rom through (time, value) keys, flat at the first and last key, clamped outside.
pub(super) fn curve(t: f64, k: &[[f64; 2]]) -> f64 {
    let n = k.len();
    if t <= k[0][0] { return k[0][1]; }
    if t >= k[n - 1][0] { return k[n - 1][1]; }
    let mut i = 0;
    while t > k[i + 1][0] { i += 1; }
    let (t0, p0, t1, p1) = (k[i][0], k[i][1], k[i + 1][0], k[i + 1][1]);
    let h = t1 - t0;
    let u = (t - t0) / h;
    let m0 = if i > 0 { (p1 - k[i - 1][1]) / (t1 - k[i - 1][0]) } else { 0.0 };
    let m1 = if i + 2 < n { (k[i + 2][1] - p0) / (k[i + 2][0] - t0) } else { 0.0 };
    let u2 = u * u;
    let u3 = u2 * u;
    (2.0 * u3 - 3.0 * u2 + 1.0) * p0 + (u3 - 2.0 * u2 + u) * h * m0 + (3.0 * u2 - 2.0 * u3) * p1 + (u3 - u2) * h * m1
}

/// `p` with the difference between two poses laid on it by w, on the joints of `only`.
pub(super) fn add_pose_delta(poses: &mut crate::anim_skeleton::Poses, p: crate::anim_skeleton::Pose, from: crate::anim_skeleton::Pose, to: crate::anim_skeleton::Pose, w: f64, only: &[f32]) {
    if w <= 0.0 { return; }
    let nr = poses.rows();
    for i in 0..poses.nodes {
        if only[i] as f64 <= 0.0 { continue; }
        // (read before written: p may be `from`)
        let (a, b) = (joint_of(poses.of(from), nr, i), joint_of(poses.of(to), nr, i));
        let pc = poses.of_mut(p);
        let q = delta_joint(joint_of(pc, nr, i), a, b, w);
        for c in 0..7 { pc[c * nr + i] = q[c]; }
    }
}

/// A joint's origin and rotation (the first seven columns, `nr` rows apart), read into doubles.
fn joint_of(cols: &[f32], nr: usize, i: usize) -> [f64; 7] {
    let mut j = [0.0; 7];
    for c in 0..7 { j[c] = cols[c * nr + i] as f64; }
    j
}

/// add_pose_delta's sums for one joint: in doubles, stored as singles once.
fn delta_joint(p: [f64; 7], a: [f64; 7], b: [f64; 7], w: f64) -> [f32; 7] {
    let mut o = [0.0f32; 7];
    for c in 0..3 { o[c] = (p[c] + (b[c] - a[c]) * w) as f32; }
    let (ax, ay, az, aw) = (-a[3], -a[4], -a[5], a[6]);
    let (bx, by, bz, bw) = (b[3], b[4], b[5], b[6]);
    let mut dx = aw * bx + ax * bw + ay * bz - az * by;
    let mut dy = aw * by - ax * bz + ay * bw + az * bx;
    let mut dz = aw * bz + ax * by - ay * bx + az * bw;
    let mut dw = aw * bw - ax * bx - ay * by - az * bz;
    if dw < 0.0 { dx = -dx; dy = -dy; dz = -dz; dw = -dw; }
    dx *= w; dy *= w; dz *= w; dw = 1.0 + (dw - 1.0) * w;
    let (px, py, pz, pw) = (p[3], p[4], p[5], p[6]);
    let x = pw * dx + px * dw + py * dz - pz * dy;
    let y = pw * dy - px * dz + py * dw + pz * dx;
    let z = pw * dz + px * dy - py * dx + pz * dw;
    let ww = pw * dw - px * dx - py * dy - pz * dz;
    let l = js::or(m::hyp4(x, y, z, ww), 1.0);
    o[3] = (x / l) as f32;
    o[4] = (y / l) as f32;
    o[5] = (z / l) as f32;
    o[6] = (ww / l) as f32;
    o
}

/// Smooth 0 to 1 to 0 window: rises over [a,b], falls over [c,d].
pub(super) fn bump(t: f64, a: f64, b: f64, c_2: f64, d: f64) -> f64 {
    v3::smoothstep(a, b, t) * (1.0 - v3::smoothstep(c_2, d, t))
}

/// A carriage flattened into spring channels (indices: LC).
fn long_flat(s: crate::character::LongShape) -> Vec<f64> {
    vec![s.side, s.up, s.fwd, s.pitch, s.aim_k, s.yaw, s.blade, s.lean, s.head_yaw, s.head_pitch, s.head_roll, s.elbow_r[0], s.elbow_r[1], s.elbow_r[2], s.elbow_l[0], s.elbow_l[1], s.elbow_l[2], s.pole_w, s.sway, s.anchor, s.steady]
}

/// The carriage's spring targets for the posture shown: LONG toward LONG_CROUCH by the crouch weight.
pub(super) fn long_targets(stance: crate::character::LongStance, crouch_w: f64) -> Vec<f64> {
    let mut a = long_flat(crate::character::long(stance));
    if crouch_w <= 1e-4 { return a; }
    let b = long_flat(crate::character::long_crouch(stance));
    let w = v3::clamp(crouch_w, 0.0, 1.0);
    for i in 0..a.len() {
        a[i] += (b[i] - a[i]) * w;
    }
    a
}

#[cfg(test)]
mod tests_2 {
    use super::*;
    use crate::character::{HoldPhase, HoldPose, HoldVariant, LongStance};

    const EPS: f64 = 1e-9;

    #[test]
    fn a_nod_shows_as_a_lift_of_the_head_the_same_whichever_way_it_went() {
        assert!(m::abs(nod_lift(0.0)) < EPS, "no nod: no lift");
        assert_eq!(nod_lift(0.3), nod_lift(-0.3));
        assert!(nod_lift(0.5) < nod_lift(0.1) && nod_lift(0.1) < 0.0, "the further he nods, the more it shows: {} {}", nod_lift(0.5), nod_lift(0.1));
    }

    const KEYS: &[[f64; 2]] = &[[0.0, 0.1], [0.13, 1.7], [0.2, -0.4], [0.55, 0.9], [1.1, 0.0]];

    #[test]
    fn a_curve_goes_through_its_keys_and_stands_still_outside_them() {
        for k in KEYS { assert!(m::abs(curve(k[0], KEYS) - k[1]) < EPS, "at {}", k[0]); }
        assert_eq!(curve(-1.0, KEYS), 0.1);
        assert_eq!(curve(9.0, KEYS), 0.0);
        // nothing snaps between keys, and it leaves its first key and comes to its last with no speed
        let step = 1e-4;
        let mut t = 0.0;
        while t < 1.1 {
            assert!(m::abs(curve(t + step, KEYS) - curve(t, KEYS)) < step * 60.0, "a jump at {t}");
            t += step;
        }
        assert!(m::abs(curve(step, KEYS) - curve(0.0, KEYS)) < 1e-5);
        assert!(m::abs(curve(1.1 - step, KEYS)) < 1e-5);
    }

    #[test]
    fn a_bump_rises_holds_at_one_and_falls_inside_its_window() {
        let b = |t: f64| bump(t, 0.1, 0.3, 0.6, 0.95);
        for t in [-1.0, 0.1, 0.95, 2.0] { assert_eq!(b(t), 0.0, "outside at {t}"); }
        for t in [0.3, 0.45, 0.6] { assert_eq!(b(t), 1.0, "at the top at {t}"); }
        assert!(b(0.15) < b(0.2) && b(0.2) < b(0.25), "it rises");
        assert!(b(0.7) > b(0.8) && b(0.8) > b(0.9), "it falls");
    }

    /// Each channel is where a layer reads it, and low he takes the crouch's table. (The crouch's table is the same table at this writing: no shape differs, so the weight moves nothing yet.)
    #[test]
    fn a_carriage_is_read_by_its_channels_and_goes_to_the_crouch_table_when_he_is_low() {
        let c = lc();
        for st in [LongStance::PatrolLong, LongStance::LowLong, LongStance::Shoulder, LongStance::PortLong] {
            let (up, down) = (crate::character::long(st), crate::character::long_crouch(st));
            let (a, b) = (long_targets(st, 0.0), long_targets(st, 1.0));
            assert_eq!((a[c.side as usize], a[c.up as usize], a[c.fwd as usize], a[c.pitch as usize], a[c.yaw as usize], a[c.lean as usize]), (up.side, up.up, up.fwd, up.pitch, up.yaw, up.lean), "{st:?}");
            assert_eq!((a[c.elbow_r as usize + 1], a[c.elbow_l as usize + 2]), (up.elbow_r[1], up.elbow_l[2]), "{st:?}");
            assert_eq!((b[c.side as usize], b[c.fwd as usize], b[c.lean as usize]), (down.side, down.fwd, down.lean), "{st:?}");
        }
    }

    fn turn(a: f64) -> [f64; 7] { let q = v3::quat::axis_angle([0.0, 1.0, 0.0], a); [0.0, 0.0, 0.0, q[0], q[1], q[2], q[3]] }
    fn yaw_of(j: [f32; 7]) -> f64 { 2.0 * m::atan2(j[4] as f64, j[6] as f64) }

    /// A difference of two poses laid on a third: the look layers add a reach to whatever he is doing.
    #[test]
    fn the_difference_of_two_poses_is_laid_on_by_its_weight() {
        let (mut p, mut a, mut b) = (turn(0.5), turn(0.0), turn(0.8));
        p[0] = 1.0;
        a[1] = 2.0;
        b[1] = 2.5;
        let at = |w: f64| delta_joint(p, a, b, w);
        let none = at(0.0);
        assert!(m::abs(none[0] as f64 - 1.0) < 1e-6 && m::abs(yaw_of(none) - 0.5) < 1e-6, "no weight: the pose as it was: {none:?}");
        let whole = at(1.0);
        assert!(m::abs(whole[1] as f64 - 0.5) < 1e-6, "the move is added: {whole:?}");
        assert!(m::abs(yaw_of(whole) - 1.3) < 1e-5, "and the turn: {}", yaw_of(whole));
        let half = at(0.5);
        assert!(m::abs(half[1] as f64 - 0.25) < 1e-6, "half of it");
        assert!(m::abs(yaw_of(half) - 0.9) < 0.01, "and about half the turn: {}", yaw_of(half));
        let l = half[3..7].iter().map(|x| (*x as f64) * (*x as f64)).sum::<f64>();
        assert!(m::abs(l - 1.0) < 1e-5, "a turn stays a turn: {l}");
        // the two poses' rotations with opposite signs are the same rotation: the short way, not round the long
        let mut flipped = b;
        for c in 3..7 { flipped[c] = -flipped[c]; }
        assert!(m::abs(yaw_of(delta_joint(p, a, flipped, 0.5)) - yaw_of(half)) < 1e-5, "{}", yaw_of(delta_joint(p, a, flipped, 0.5)));
    }

    /// After a choke run to its end he lets the body go slowly while he stands there; moving, busy, or with a gun, he lets go at once.
    /// A FAILURE MEANS: a choke ends in a snap back to idle, or a man walks off with his arms still round nobody.
    #[test]
    fn he_lets_go_after_a_choke_slowly_only_while_he_stands_there() {
        let hold = crate::knobs::HoldK::page();
        let show = crate::character::CharacterModHOLDSHOW { beats: false };
        let at_once = 16.0;
        let pose = |phase, t, beats, variant| HoldPose { phase, t, beats, variant, ..HoldPose::default() };
        let rate = |last, speed, busy| out_rate(last, speed, busy, show, &hold);
        let end_of_choke = 0.95 * hold.secs.choke;
        let done = pose(HoldPhase::Choke, end_of_choke, Some(true), None);
        assert_eq!(rate(done, 0.0, false), hold.beats.after, "a choke run to its end, standing still");
        assert!(hold.beats.after < at_once, "the slow rate is slower than at once");
        let (a, b, c) = (rate(done, 0.2, false), rate(done, 0.35, false), rate(done, 0.5, false));
        assert!(hold.beats.after < a && a < b && b < c && c < at_once, "it quickens as he walks off: {a} {b} {c}");
        assert_eq!(rate(done, 2.0, false), at_once, "walking");
        assert_eq!(rate(done, -0.35, false), b, "a speed either way is a speed");
        assert_eq!(rate(pose(HoldPhase::Choke, end_of_choke, None, None), 0.0, false), at_once, "with the beats' show off");
        assert_eq!(rate(pose(HoldPhase::Choke, 0.5 * hold.secs.choke, Some(true), None), 0.0, false), at_once, "a choke broken off early");
        assert_eq!(rate(pose(HoldPhase::Held, 0.0, None, Some(HoldVariant::Gun)), 0.0, false), at_once, "held at gunpoint");
        assert_eq!(rate(done, 0.0, true), at_once, "busy with something else");
    }
}
