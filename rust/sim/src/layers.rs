//! THE ANIMATOR'S LAYERS (src/game/character.ts CharacterAnimator) — the steps of a character's update that pose him, moved here one at a time behind the
//! row RUST.walk. A layer is the TypeScript method of the same name, statement for statement and operand for operand: it reads and writes the pose WHERE IT
//! LIVES (the game's pool, poses.rs; the questions of skeleton.rs are plain calls here, with no crossing), reads its knobs and the rig's named nodes from
//! knobs.rs, and is handed the numbers of the animator it needs on the bench's `args`, in the order its comment gives. What a layer keeps from one tick to
//! the next goes into the game's block with the layer that owns it; a layer that owns nothing (the trunk) only needs the pose.
//!
//! A layer answers `true` when it ran; `false` = not a pose of this pool, or a role with no node (nothing written): the host then runs its own.
use crate::knobs::{k, on, role};
use crate::knobs::{GAIT_AIM_LEAN, GAIT_BACK, GAIT_CHIN, GAIT_LEAN, GAIT_SLOUCH_ROLL, GAIT_SLOUCH_SWAY, HEAD, MOVES_ON, MOVES_RECOIL, NECK, SPINE1, SPINE2, SPINE3};
use crate::maths::{cos, sin};
use crate::single::S;
use crate::skeleton::{bench, parent_rot, rotate_model, with_pose, Rig};

const PI: f64 = 3.141592653589793;
/// character.ts DEGR
const DEGR: f64 = PI / 180.0;

#[inline(always)]
fn abs(x: f64) -> f64 {
    f64::from_bits(x.to_bits() & 0x7fff_ffff_ffff_ffff)
}
/// math/vec.ts clamp, smoothstep
#[inline(always)]
fn clamp(x: f64, a: f64, b: f64) -> f64 {
    if x < a {
        a
    } else if x > b {
        b
    } else {
        x
    }
}
#[inline(always)]
fn smoothstep(a: f64, b: f64, x: f64) -> f64 {
    let t = clamp((x - a) / (b - a), 0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}
/// quat.axisAngle about the model's X, Y, Z (character.ts qPitch, qYaw, qRoll): the products with the axis's noughts are written out, since −0 is a number too
#[inline(always)]
fn q_pitch(a: f64) -> [f64; 4] {
    let s = sin(a / 2.0);
    [1.0 * s, 0.0 * s, 0.0 * s, cos(a / 2.0)]
}
#[inline(always)]
fn q_yaw(a: f64) -> [f64; 4] {
    let s = sin(a / 2.0);
    [0.0 * s, 1.0 * s, 0.0 * s, cos(a / 2.0)]
}
#[inline(always)]
fn q_roll(a: f64) -> [f64; 4] {
    let s = sin(a / 2.0);
    [0.0 * s, 0.0 * s, 1.0 * s, cos(a / 2.0)]
}
/// quat.mul
#[inline(always)]
fn q_mul(a: &[f64; 4], b: &[f64; 4]) -> [f64; 4] {
    let (ax, ay, az, aw, bx, by, bz, bw) = (a[0], a[1], a[2], a[3], b[0], b[1], b[2], b[3]);
    [aw * bx + ax * bw + ay * bz - az * by, aw * by - ax * bz + ay * bw + az * bx, aw * bz + ax * by - ay * bx + az * bw, aw * bw - ax * bx - ay * by - az * bz]
}
/// CharacterAnimator.turn: node i (and its subtree) about model axes, in place
#[inline(always)]
fn turn(d: &mut [S], rig: &Rig, i: usize, q: [f64; 4]) {
    let par = parent_rot(rig, d, i);
    rotate_model(d, i, &q, &par);
}
/// do these roles have nodes of a rig of `n` nodes
fn cast(n: usize, roles: &[usize]) -> bool {
    roles.iter().all(|&r| role(r) < n)
}

/// CharacterAnimator.mvTrunk — the trunk's share of MOVES, straight into the pose. args: 0 w · 1 o.g · 2 o.u · 3 o.st · 4 aimPass (1 or 0) · 5 phase ·
/// 6 speed · 7 mvs.rock · 8 mvs.sho · 9 mvs.head
pub fn mv_trunk(g: usize, at: usize) -> bool {
    let a = bench().args;
    let (w, og, ou, ost, aim_pass, phase, speed, rock, sho, head) = (a[0], a[1], a[2], a[3], a[4] != 0.0, a[5], a[6], a[7], a[8], a[9]);
    with_pose(g, at, |d, rig, n| {
        if !cast(n, &[SPINE1, SPINE2, SPINE3, NECK, HEAD]) {
            return false;
        }
        if w < 0.003 {
            return true;
        }
        let (s1, s2, s3, nk, hd) = (role(SPINE1), role(SPINE2), role(SPINE3), role(NECK), role(HEAD));
        let (mut lean, mut chin, mut sway, mut roll);
        if aim_pass {
            lean = k(GAIT_AIM_LEAN) * ost + k(GAIT_LEAN) * 0.5 * og - k(GAIT_BACK) * ou;
            chin = k(GAIT_CHIN) * ou;
            (sway, roll) = (0.0, 0.0);
        } else {
            lean = k(GAIT_LEAN) * og;
            let ph = 2.0 * PI * phase;
            let mw = smoothstep(0.2, 0.8, abs(speed));
            sway = k(GAIT_SLOUCH_SWAY) * ou * mw * sin(ph);
            roll = k(GAIT_SLOUCH_ROLL) * ou * mw * cos(ph);
            chin = 0.0;
        }
        lean *= DEGR * w;
        chin *= DEGR * w;
        sway *= DEGR * w;
        roll *= DEGR * w;
        if abs(lean) > 1e-5 {
            turn(d, rig, s1, q_pitch(lean * 0.45));
            turn(d, rig, s2, q_pitch(lean * 0.35));
            turn(d, rig, s3, q_pitch(lean * 0.2));
            turn(d, rig, nk, q_pitch(-lean * 0.4));
            turn(d, rig, hd, q_pitch(-lean * 0.6));
        }
        if abs(sway) + abs(roll) > 1e-5 {
            let q = |kk: f64| q_mul(&q_yaw(sway * kk), &q_roll(roll * kk));
            turn(d, rig, s2, q(0.45));
            turn(d, rig, s3, q(0.55));
            turn(d, rig, nk, q_yaw(-sway * 0.4));
            turn(d, rig, hd, q_yaw(-sway * 0.4));
        }
        if chin > 1e-5 {
            turn(d, rig, nk, q_pitch(-chin * 0.4));
            turn(d, rig, hd, q_pitch(-chin * 0.6));
        }
        if on(MOVES_ON) && on(MOVES_RECOIL) && (rock > 1e-4 || sho > 1e-4) {
            let q = |kk: f64| q_mul(&q_yaw(-sho * DEGR * w * kk), &q_pitch(-rock * DEGR * w * kk));
            turn(d, rig, s2, q(0.4));
            turn(d, rig, s3, q(0.6));
            // the head keeps most of its line, and is jarred back off the stock
            let hq = |kk: f64| q_mul(&q_yaw(sho * 0.6 * DEGR * w * kk), &q_pitch((rock * 0.6 - head) * DEGR * w * kk));
            turn(d, rig, nk, hq(0.4));
            turn(d, rig, hd, hq(0.6));
        }
        true
    })
    .unwrap_or(false)
}
