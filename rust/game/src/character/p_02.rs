//! src/game/character.ts lines 325 to 693
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// Writes the stalk's polish in the hold's table: up = the page's numbers. Down (`up` false) is the off branch of settled rows: dropped. (character.ts holdPolish)
pub fn hold_polish(g: &mut crate::game::Game, up: bool) {
    if !up { return; }
    let u = crate::knobs::page_hold();
    let h = &mut g.k.hold;
    h.way_shape = u.way_shape; h.catch_shape = u.catch_shape; h.hit_shape = u.hit_shape; h.lay_ahead = u.lay_ahead;
    h.lunge.dip = u.lunge.dip; h.lunge.back = u.lunge.back; h.sway.m = u.sway.m; h.sway.twist = u.sway.twist;
    h.apart.side = u.apart.side; h.apart.tip = u.apart.tip; h.yank.arch = u.yank.arch;
}

/// The rate (1/s) Sam's hold layer lets go at once the controller has: slow out of a kill or a choke run to its end while he stands idle. (character.ts holdOutRate)
pub fn hold_out_rate(g: &mut crate::game::Game, last: crate::character::HoldPose, speed: f64, busy: bool) -> f64 {
    out_rate(last, speed, busy, g.m.character.hold_show, &g.k.hold)
}

/// hold_out_rate's choice, over what it reads of the game.
fn out_rate(last: crate::character::HoldPose, speed: f64, busy: bool, show: crate::character::CharacterModHOLDSHOW, hold: &crate::knobs::HoldK) -> f64 {
    if busy || last.variant == Some(crate::character::HoldVariant::Gun) { return 16.0; }
    let slow = if last.phase == crate::character::HoldPhase::Kill && last.kill.unwrap_or(show.kill) {
        hold.kill.after
    } else if last.phase == crate::character::HoldPhase::Choke && last.t > 0.9 * hold.secs.choke && last.beats.unwrap_or(show.beats) {
        hold.beats.after
    } else {
        16.0
    };
    if slow == 16.0 { 16.0 } else { v3::lerp(slow, 16.0, v3::smoothstep(0.1, 0.6, m::abs(speed))) }
}

/// A goggle nod shown as a lift of the head: as big less a little whichever way the nod went. (character.ts nodLift)
pub fn nod_lift(n: f64) -> f64 {
    2.0 * crate::character::DEGR - m::hyp2(n, 2.0 * crate::character::DEGR)
}

/// Model-axis yaw: + turns to the figure's left. (character.ts qYaw)
pub fn q_yaw(a: f64) -> Quat {
    v3::quat::axis_angle(crate::character::ay(), a)
}

/// A critically damped follower at `x`, at rest. (character.ts Spring)
pub fn spring_new(x: Option<f64>) -> crate::character::Spring {
    crate::character::Spring { v: 0.0, x: x.unwrap_or(0.0) }
}

/// One step toward `target`; w is about 6 / settle time. (character.ts Spring.step)
pub fn spring_step(this: &mut crate::character::Spring, target: f64, w: f64, dt: f64) -> f64 {
    let y = this.x - target;
    let j = this.v + y * w;
    let e = m::exp(-w * dt);
    this.x = target + (y + j * dt) * e;
    this.v = (this.v - j * w * dt) * e;
    this.x
}

/// Catmull-Rom through (time, value) keys, flat at the first and last key, clamped outside. (character.ts curve)
pub fn curve(t: f64, k: &[[f64; 2]]) -> f64 {
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

/// A clip's first keys replayed into `out`: the slots are found once per clip, by sampling into a pose of NaNs. (character.ts sampleAt0)
pub fn sample_at0(g: &mut crate::game::Game, clip: ClipId, out: &mut crate::anim_skeleton::Pose) {
    let stale = match g.m.character.at0.get(clip) {
        None => true,
        Some(e) => e.rows != out.rows as f64,
    };
    if stale {
        let mut p = crate::anim_skeleton::pose_new(out.num_nodes as f64);
        p.cols.fill(f32::NAN);
        crate::anim_skeleton::sample_clip(&g.tab.clip[clip], 0.0, false, &mut p);
        let mut at: Vec<i32> = Vec::new();
        for i in 0..p.cols.len() {
            if !p.cols[i].is_nan() { at.push(i as i32); }
        }
        let v: Vec<f32> = at.iter().map(|&i| p.cols[i as usize]).collect();
        g.m.character.at0.set(clip, crate::character::CharacterModAT0Value { rows: p.rows as f64, at, v });
    }
    let e = g.m.character.at0.get(clip).unwrap();
    for k in 0..e.at.len() {
        out.cols[e.at[k] as usize] = e.v[k];
    }
}

/// `p` with the difference between two poses laid on it by w, on the joints of `only`. (character.ts addPoseDelta)
pub fn add_pose_delta(p: &mut crate::anim_skeleton::Pose, from: &crate::anim_skeleton::Pose, to: &crate::anim_skeleton::Pose, w: f64, only: &[f32]) {
    if w <= 0.0 { return; }
    let (n, nr) = (p.num_nodes, p.rows);
    delta_onto(&mut p.cols, &from.cols, &to.cols, n, nr, w, only);
}

/// add_pose_delta's sums over the columns (rows `nr` apart), as the TypeScript does them: singles read into doubles, stored as singles once.
fn delta_onto(pc: &mut [f32], ac: &[f32], bc: &[f32], n: usize, nr: usize, w: f64, only: &[f32]) {
    for i in 0..n {
        if only[i] as f64 <= 0.0 { continue; }
        let mut o = i;
        for _c in 0..3 {
            pc[o] = (pc[o] as f64 + (bc[o] as f64 - ac[o] as f64) * w) as f32;
            o += nr;
        }
        let (ax, ay, az, aw) = (-(ac[3 * nr + i] as f64), -(ac[4 * nr + i] as f64), -(ac[5 * nr + i] as f64), ac[6 * nr + i] as f64);
        let (bx, by, bz, bw) = (bc[3 * nr + i] as f64, bc[4 * nr + i] as f64, bc[5 * nr + i] as f64, bc[6 * nr + i] as f64);
        let mut dx = aw * bx + ax * bw + ay * bz - az * by;
        let mut dy = aw * by - ax * bz + ay * bw + az * bx;
        let mut dz = aw * bz + ax * by - ay * bx + az * bw;
        let mut dw = aw * bw - ax * bx - ay * by - az * bz;
        if dw < 0.0 { dx = -dx; dy = -dy; dz = -dz; dw = -dw; }
        dx *= w; dy *= w; dz *= w; dw = 1.0 + (dw - 1.0) * w;
        let (px, py, pz, pw) = (pc[3 * nr + i] as f64, pc[4 * nr + i] as f64, pc[5 * nr + i] as f64, pc[6 * nr + i] as f64);
        let x = pw * dx + px * dw + py * dz - pz * dy;
        let y = pw * dy - px * dz + py * dw + pz * dx;
        let z = pw * dz + px * dy - py * dx + pz * dw;
        let ww = pw * dw - px * dx - py * dy - pz * dz;
        let l = js::or(m::hyp4(x, y, z, ww), 1.0);
        pc[3 * nr + i] = (x / l) as f32;
        pc[4 * nr + i] = (y / l) as f32;
        pc[5 * nr + i] = (z / l) as f32;
        pc[6 * nr + i] = (ww / l) as f32;
    }
}

/// Smooth 0 to 1 to 0 window: rises over [a,b], falls over [c,d]. (character.ts bump)
pub fn bump(t: f64, a: f64, b: f64, c_2: f64, d: f64) -> f64 {
    v3::smoothstep(a, b, t) * (1.0 - v3::smoothstep(c_2, d, t))
}

/// A copy of a carriage. (character.ts cloneShape)
pub fn clone_shape(s: crate::character::LongShape) -> crate::character::LongShape {
    crate::character::LongShape { elbow_r: s.elbow_r, elbow_l: s.elbow_l, ..s }
}

/// A carriage flattened into spring channels (CharacterAnimator.lg.ch; indices: LC). (character.ts longFlat)
pub fn long_flat(s: crate::character::LongShape) -> Vec<f64> {
    vec![s.side, s.up, s.fwd, s.pitch, s.aim_k, s.yaw, s.blade, s.lean, s.head_yaw, s.head_pitch, s.head_roll, s.elbow_r[0], s.elbow_r[1], s.elbow_r[2], s.elbow_l[0], s.elbow_l[1], s.elbow_l[2], s.pole_w, s.sway, s.anchor, s.steady]
}

/// The carriage's spring targets for the posture shown: LONG toward LONG_CROUCH by the crouch weight. (character.ts longTargets)
pub fn long_targets(stance: crate::character::LongStance, crouch_w: f64) -> Vec<f64> {
    let mut a = long_flat(*crate::character::long().get(String::from(stance.as_str())).unwrap());
    if crouch_w <= 1e-4 { return a; }
    let b = long_flat(*crate::character::long_crouch().get(String::from(stance.as_str())).unwrap());
    let w = v3::clamp(crouch_w, 0.0, 1.0);
    for i in 0..a.len() {
        a[i] += (b[i] - a[i]) * w;
    }
    a
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::character::{HoldPhase, HoldPose, HoldVariant, LongStance};

    // The TypeScript's own bits: its functions, copied from character.ts by line, under bun with EXACT.on.
    const NOD: &[(u64, u64)] = &[(0x0000000000000000, 0x0000000000000000), (0x8000000000000000, 0x0000000000000000), (0x3fa1de69ad42c3ca, 0xbf8d9a1fcb98ebfc), (0xbfb999999999999a, 0xbfb22dc1b9e829a0), (0x3fe0000000000000, 0xbfddd8079749b907), (0x3ff6e6127ff76b8e, 0xbff658d69835d7dd), (0xc00199999999999a, 0xc00152adacf59275)];
    const SPRING: &[(u64, u64, u64, u64, u64)] = &[(0x3ff0000000000000, 0x4030000000000000, 0x3f91111111111111, 0x3fd4893f273c0608, 0x40024cf2e201ac9e), (0x3ff0000000000000, 0x4030000000000000, 0x3f91111111111111, 0x3fd7b3896ce3dde2, 0x400c08ab87ba8709), (0xbfd999999999999a, 0x4023000000000000, 0x3f81111111111111, 0x3fd9480a8d2417a6, 0x4003914eeb88eedd), (0x3fd0000000000000, 0x403c000000000000, 0x3fa0e5604189374c, 0x3fd9237a997ac267, 0xbff6a5dba049cfef), (0x3fd0000000000000, 0x4018000000000000, 0x0000000000000000, 0x3fd9237a997ac267, 0xbff6a5dba049cfef), (0x4004000000000000, 0x400a666666666666, 0x3fe0000000000000, 0x3ff4aa99afec0db1, 0x40030ab7b1f2586a)];
    const KEYS: &[[f64; 2]] = &[[0.0, 0.1], [0.13, 1.7], [0.2, -0.4], [0.55, 0.9], [1.1, 0.0]];
    const CURVE: &[(u64, u64)] = &[(0xbff0000000000000, 0x3fb999999999999a), (0x0000000000000000, 0x3fb999999999999a), (0x3fa999999999999a, 0x3fe50ae219ae7b19), (0x3fc0a3d70a3d70a4, 0x3ffb333333333333), (0x3fc5c28f5c28f5c3, 0x3fdb510de1d8e414), (0x3fc999999999999a, 0xbfd999999999999a), (0x3fd554c985f06f69, 0xbfb6d7f614bd6f44), (0x3fe199999999999a, 0x3feccccccccccccd), (0x3feccccccccccccd, 0x3fd2a0716272483a), (0x3ff199999999999a, 0x0000000000000000), (0x4000000000000000, 0x0000000000000000)];
    const BUMP: &[(u64, u64)] = &[(0xbfb999999999999a, 0x0000000000000000), (0x3fb999999999999a, 0x0000000000000000), (0x3fc999999999999a, 0x3fe0000000000001), (0x3fd51eb851eb851f, 0x3ff0000000000000), (0x3fe0000000000000, 0x3ff0000000000000), (0x3fe6b851eb851eb8, 0x3fe8811d1271a62a), (0x3feccccccccccccd, 0x3fac5c8c509b3de0), (0x3ff3333333333333, 0x0000000000000000)];
    const FLAT_PATROLLONG: &[u64] = &[0xbfcf5c28f5c28f5c, 0xbfc47ae147ae147b, 0x3fc0a3d70a3d70a4, 0xc045000000000000, 0x0000000000000000, 0x4034000000000000, 0xc020000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0xbfdccccccccccccd, 0xbff0000000000000, 0xbfe0000000000000, 0x3fd6666666666666, 0xbff0000000000000, 0xbfd3333333333333, 0x3fe4cccccccccccd, 0x3ff0000000000000, 0x0000000000000000, 0x3fe0000000000000];
    const TARGETS_PATROLLONG: &[u64] = &[0xbfcf5c28f5c28f5c, 0xbfc47ae147ae147b, 0x3fc0a3d70a3d70a4, 0xc045000000000000, 0x0000000000000000, 0x4034000000000000, 0xc020000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0xbfdccccccccccccd, 0xbff0000000000000, 0xbfe0000000000000, 0x3fd6666666666666, 0xbff0000000000000, 0xbfd3333333333333, 0x3fe4cccccccccccd, 0x3ff0000000000000, 0x0000000000000000, 0x3fe0000000000000];
    const FLAT_LOWLONG: &[u64] = &[0xbfc51eb851eb851f, 0xbfb5c28f5c28f5c3, 0x3fc999999999999a, 0xc044000000000000, 0x3fa999999999999a, 0x4038000000000000, 0xc024000000000000, 0x4000000000000000, 0x0000000000000000, 0x4000000000000000, 0x0000000000000000, 0xbfe199999999999a, 0xbff0000000000000, 0xbfd6666666666666, 0x3fd999999999999a, 0xbff0000000000000, 0xbfb999999999999a, 0x3fe8000000000000, 0x3fd0000000000000, 0x3fa999999999999a, 0x3ff0000000000000];
    const TARGETS_LOWLONG: &[u64] = &[0xbfc51eb851eb851f, 0xbfb5c28f5c28f5c3, 0x3fc999999999999a, 0xc044000000000000, 0x3fa999999999999a, 0x4038000000000000, 0xc024000000000000, 0x4000000000000000, 0x0000000000000000, 0x4000000000000000, 0x0000000000000000, 0xbfe199999999999a, 0xbff0000000000000, 0xbfd6666666666666, 0x3fd999999999999a, 0xbff0000000000000, 0xbfb999999999999a, 0x3fe8000000000000, 0x3fd0000000000000, 0x3fa999999999999a, 0x3ff0000000000000];
    const FLAT_SHOULDER: &[u64] = &[0xbfc3333333333333, 0xbf8eb851eb851eb8, 0x3fd051eb851eb852, 0xc000000000000000, 0x3ff0000000000000, 0x0000000000000000, 0xc042000000000000, 0x4010000000000000, 0xc024000000000000, 0x4018000000000000, 0x401c000000000000, 0xbfe999999999999a, 0xbff0000000000000, 0xbfc999999999999a, 0x3fe3333333333333, 0xbff0000000000000, 0x3fd6666666666666, 0x3fe3333333333333, 0x3fb999999999999a, 0x3fe0000000000000, 0x3ff0000000000000];
    const TARGETS_SHOULDER: &[u64] = &[0xbfc3333333333333, 0xbf8eb851eb851eb8, 0x3fd051eb851eb852, 0xc000000000000000, 0x3ff0000000000000, 0x0000000000000000, 0xc042000000000000, 0x4010000000000000, 0xc024000000000000, 0x4018000000000000, 0x401c000000000000, 0xbfe999999999999a, 0xbff0000000000000, 0xbfc999999999999a, 0x3fe3333333333333, 0xbff0000000000000, 0x3fd6666666666666, 0x3fe3333333333333, 0x3fb999999999999a, 0x3fe0000000000000, 0x3ff0000000000000];
    const FLAT_PORTLONG: &[u64] = &[0xbfbd70a3d70a3d71, 0xbfc3333333333333, 0x3fc851eb851eb852, 0x4049000000000000, 0x0000000000000000, 0x4041800000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0xbfe6666666666666, 0xbfe999999999999a, 0xbfd3333333333333, 0x3feccccccccccccd, 0xbfe3333333333333, 0x3fb999999999999a, 0x3fe8000000000000, 0x3fe0000000000000, 0x0000000000000000, 0x3fe6666666666666];
    const TARGETS_PORTLONG: &[u64] = &[0xbfbd70a3d70a3d71, 0xbfc3333333333333, 0x3fc851eb851eb852, 0x4049000000000000, 0x0000000000000000, 0x4041800000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0xbfe6666666666666, 0xbfe999999999999a, 0xbfd3333333333333, 0x3feccccccccccccd, 0xbfe3333333333333, 0x3fb999999999999a, 0x3fe8000000000000, 0x3fe0000000000000, 0x0000000000000000, 0x3fe6666666666666];
    const DELTA_P: [u32; 50] = [0x3e9f8179, 0x3f30c908, 0xbefa9df3, 0x00000000, 0x00000000, 0x3f49d1f2, 0x3d084063, 0xbf4c03b7, 0x00000000, 0x00000000, 0x3f0a55b9, 0xbf279549, 0xbebe4d12, 0x00000000, 0x00000000, 0x3ef80d1b, 0x3f23a93d, 0x3e4f53fc, 0x00000000, 0x00000000, 0x3f1f3ea3, 0xbf2279f4, 0x3f19ac66, 0x00000000, 0x00000000, 0x3e3f1e94, 0xbedda096, 0xbf190244, 0x00000000, 0x00000000, 0x3f160a21, 0x3d088706, 0xbefb929f, 0x00000000, 0x00000000, 0x3f800000, 0x3f800000, 0x3f800000, 0x00000000, 0x00000000, 0x3f800000, 0x3f800000, 0x3f800000, 0x00000000, 0x00000000, 0x3f800000, 0x3f800000, 0x3f800000, 0x00000000, 0x00000000];
    const DELTA_A: [u32; 50] = [0x3f3684ef, 0x3e89360e, 0xbf4832bb, 0x00000000, 0x00000000, 0x3f30c908, 0xbefa9df3, 0xbf107ea2, 0x00000000, 0x00000000, 0x3d084063, 0xbf4c03b7, 0x3e3039ee, 0x00000000, 0x00000000, 0x3f313ce6, 0xbd741d59, 0xbf000fe0, 0x00000000, 0x00000000, 0x3ec6dfb8, 0xbf163dab, 0x3ec929bd, 0x00000000, 0x00000000, 0x3f189b2e, 0xbe6e5839, 0xbf0a7b3b, 0x00000000, 0x00000000, 0x3df61b63, 0xbf45f0bb, 0xbf0cdcef, 0x00000000, 0x00000000, 0x3f800000, 0x3f800000, 0x3f800000, 0x00000000, 0x00000000, 0x3f800000, 0x3f800000, 0x3f800000, 0x00000000, 0x00000000, 0x3f800000, 0x3f800000, 0x3f800000, 0x00000000, 0x00000000];
    const DELTA_B: [u32; 50] = [0x3f18b86e, 0xbf1afe3f, 0xbee18f90, 0x00000000, 0x00000000, 0xbe0139ab, 0xbf446340, 0x3ea5d397, 0x00000000, 0x00000000, 0xbf3ba17b, 0xbe64e5ad, 0x3f4a607a, 0x00000000, 0x00000000, 0xbf09d154, 0xbf053c7b, 0x3ee0f3ba, 0x00000000, 0x00000000, 0xbeb8d0b4, 0x3eea03b3, 0xbf1bbd86, 0x00000000, 0x00000000, 0x3f0a960d, 0x3ef1d222, 0xbf09427e, 0x00000000, 0x00000000, 0xbf091a26, 0xbf0b8cb1, 0x3ec5e32e, 0x00000000, 0x00000000, 0x3f800000, 0x3f800000, 0x3f800000, 0x00000000, 0x00000000, 0x3f800000, 0x3f800000, 0x3f800000, 0x00000000, 0x00000000, 0x3f800000, 0x3f800000, 0x3f800000, 0x00000000, 0x00000000];
    const DELTA_OUT: [u32; 50] = [0x3e751c41, 0x3f30c908, 0xbe8e380d, 0x00000000, 0x00000000, 0x3e905ddb, 0x3d084063, 0xbe7c161e, 0x00000000, 0x00000000, 0x3d85cacc, 0xbf279549, 0x3c4083eb, 0x00000000, 0x00000000, 0x3e9845ea, 0x3f23a93d, 0xbe112cac, 0x00000000, 0x00000000, 0x3e4fc5d9, 0xbf2279f4, 0x3f75f4de, 0x00000000, 0x00000000, 0xbee3ec93, 0xbedda096, 0xbe40f878, 0x00000000, 0x00000000, 0x3f51e3d2, 0x3d088706, 0xbe157ef2, 0x00000000, 0x00000000, 0x3f800000, 0x3f800000, 0x3f800000, 0x00000000, 0x00000000, 0x3f800000, 0x3f800000, 0x3f800000, 0x00000000, 0x00000000, 0x3f800000, 0x3f800000, 0x3f800000, 0x00000000, 0x00000000];
    const OUT_KILL_SHOWN: [u64; 6] = [0x401c000000000000, 0x401c000000000000, 0x40245604189374bc, 0x402c1cac083126e9, 0x4030000000000000, 0x4030000000000000];
    const OUT_KILL_OFF: [u64; 6] = [0x4030000000000000, 0x4030000000000000, 0x4030000000000000, 0x4030000000000000, 0x4030000000000000, 0x4030000000000000];
    const OUT_CHOKE_END: [u64; 6] = [0x4004cccccccccccd, 0x4004cccccccccccd, 0x401d4467381d7dc0, 0x402a36113404ea4a, 0x4030000000000000, 0x4030000000000000];
    const OUT_CHOKE_SHOW_OFF: [u64; 6] = [0x4030000000000000, 0x4030000000000000, 0x4030000000000000, 0x4030000000000000, 0x4030000000000000, 0x4030000000000000];
    const OUT_CHOKE_EARLY: [u64; 6] = [0x4030000000000000, 0x4030000000000000, 0x4030000000000000, 0x4030000000000000, 0x4030000000000000, 0x4030000000000000];
    const OUT_GUN: [u64; 6] = [0x4030000000000000, 0x4030000000000000, 0x4030000000000000, 0x4030000000000000, 0x4030000000000000, 0x4030000000000000];
    const OUT_BUSY: [u64; 6] = [0x4030000000000000, 0x4030000000000000, 0x4030000000000000, 0x4030000000000000, 0x4030000000000000, 0x4030000000000000];

    fn f(b: u64) -> f64 { f64::from_bits(b) }

    #[test]
    fn nod_lift_bits() {
        for &(n, r) in NOD { assert_eq!(nod_lift(f(n)).to_bits(), r); }
    }

    #[test]
    fn spring_bits() {
        let mut s = spring_new(Some(0.3));
        assert_eq!(s.v, 0.0);
        for &(t, w, dt, r, v) in SPRING {
            assert_eq!(spring_step(&mut s, f(t), f(w), f(dt)).to_bits(), r);
            assert_eq!(s.v.to_bits(), v);
        }
        assert_eq!(spring_new(None).x, 0.0);
    }

    #[test]
    fn curve_bits() {
        for &(t, r) in CURVE { assert_eq!(curve(f(t), KEYS).to_bits(), r); }
    }

    #[test]
    fn bump_bits() {
        for &(t, r) in BUMP { assert_eq!(bump(f(t), 0.1, 0.3, 0.6, 0.95).to_bits(), r); }
    }

    #[test]
    fn the_long_table_and_its_targets_bits() {
        let rows: [(LongStance, &[u64], &[u64]); 4] = [
            (LongStance::PatrolLong, FLAT_PATROLLONG, TARGETS_PATROLLONG),
            (LongStance::LowLong, FLAT_LOWLONG, TARGETS_LOWLONG),
            (LongStance::Shoulder, FLAT_SHOULDER, TARGETS_SHOULDER),
            (LongStance::PortLong, FLAT_PORTLONG, TARGETS_PORTLONG),
        ];
        for (st, flat, targets) in rows {
            let got: Vec<u64> = long_targets(st, 0.0).iter().map(|x| x.to_bits()).collect();
            assert_eq!(got, flat);
            let got: Vec<u64> = long_targets(st, 0.37).iter().map(|x| x.to_bits()).collect();
            assert_eq!(got, targets);
        }
    }

    #[test]
    fn pose_delta_bits() {
        let cols = |c: &[u32; 50]| -> Vec<f32> { c.iter().map(|&b| f32::from_bits(b)).collect() };
        let mut p = cols(&DELTA_P);
        delta_onto(&mut p, &cols(&DELTA_A), &cols(&DELTA_B), 3, 5, 0.62, &[1.0, 0.0, 0.25]);
        let got: Vec<u32> = p.iter().map(|x| x.to_bits()).collect();
        assert_eq!(got, DELTA_OUT);
    }

    #[test]
    fn the_polish_numbers_are_the_pages() {
        let u = crate::character::hold_polish_up();
        let h = crate::knobs::page_hold();
        assert_eq!((u.way_shape, u.catch_shape, u.hit_shape), (h.way_shape, h.catch_shape, h.hit_shape));
        let (a, b): ([f64; 8], [f64; 8]) = (
            [u.lay_ahead, u.dip, u.back, u.m, u.twist, u.side, u.tip, u.arch],
            [h.lay_ahead, h.lunge.dip, h.lunge.back, h.sway.m, h.sway.twist, h.apart.side, h.apart.tip, h.yank.arch],
        );
        for i in 0..8 { assert_eq!(a[i].to_bits(), b[i].to_bits()); }
    }

    /// The kept slots replay into `out` and leave the rest of it as it stands; no sample is taken (the clip is never read).
    #[test]
    fn sample_at0_replays_the_kept_slots() {
        let mut g = crate::game::Game::blank();
        let clip = ClipId(0);
        let mut out = crate::anim_skeleton::Pose { num_nodes: 2, rows: 4, cols: vec![9.0; 40] };
        g.m.character.at0.set(clip, crate::character::CharacterModAT0Value { rows: 4.0, at: vec![1, 7, 39], v: vec![0.25, -1.5, 3.0] });
        sample_at0(&mut g, clip, &mut out);
        let mut want = vec![9.0f32; 40];
        want[1] = 0.25; want[7] = -1.5; want[39] = 3.0;
        assert_eq!(out.cols, want);
    }

    #[test]
    fn add_pose_delta_goes_by_the_pose_and_not_for_no_weight() {
        let cols = |c: &[u32; 50]| -> Vec<f32> { c.iter().map(|&b| f32::from_bits(b)).collect() };
        let pose = |c: &[u32; 50]| crate::anim_skeleton::Pose { num_nodes: 3, rows: 5, cols: cols(c) };
        let (mut p, a, b) = (pose(&DELTA_P), pose(&DELTA_A), pose(&DELTA_B));
        add_pose_delta(&mut p, &a, &b, 0.0, &[1.0, 0.0, 0.25]);
        assert_eq!(p.cols.iter().map(|x| x.to_bits()).collect::<Vec<u32>>(), DELTA_P);
        add_pose_delta(&mut p, &a, &b, 0.62, &[1.0, 0.0, 0.25]);
        assert_eq!(p.cols.iter().map(|x| x.to_bits()).collect::<Vec<u32>>(), DELTA_OUT);
    }

    #[test]
    fn hold_out_rate_bits() {
        let hold = crate::knobs::page_hold();
        let show = crate::character::CharacterModHOLDSHOW { kill: true, beats: false };
        let speeds = [0.0, 0.1, 0.3, -0.45, 0.6, 2.0];
        let pose = |phase, t, kill, beats, variant| HoldPose { phase, t, kill, beats, variant, ..HoldPose::zeroed() };
        let cases: [(HoldPose, bool, &[u64; 6]); 7] = [
            (pose(HoldPhase::Kill, 0.0, None, None, None), false, &OUT_KILL_SHOWN),
            (pose(HoldPhase::Kill, 0.0, Some(false), None, None), false, &OUT_KILL_OFF),
            (pose(HoldPhase::Choke, 2.5, None, Some(true), None), false, &OUT_CHOKE_END),
            (pose(HoldPhase::Choke, 2.5, None, None, None), false, &OUT_CHOKE_SHOW_OFF),
            (pose(HoldPhase::Choke, 2.3, None, Some(true), None), false, &OUT_CHOKE_EARLY),
            (pose(HoldPhase::Kill, 0.0, Some(true), None, Some(HoldVariant::Gun)), false, &OUT_GUN),
            (pose(HoldPhase::Kill, 0.0, Some(true), None, None), true, &OUT_BUSY),
        ];
        for (last, busy, want) in cases {
            for i in 0..6 { assert_eq!(out_rate(last, speeds[i], busy, show, &hold).to_bits(), want[i]); }
        }
    }
}
