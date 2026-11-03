//! THE ANIMATOR'S LAYERS (src/game/character.ts CharacterAnimator) — the steps of a character's update that pose him, moved here one at a time behind the
//! row RUST.walk. A layer is the TypeScript method of the same name, statement for statement and operand for operand: it reads and writes the pose WHERE IT
//! LIVES (the game's pool, poses.rs; the questions of skeleton.rs are plain calls here, with no crossing), reads its knobs and the rig's named nodes from
//! knobs.rs, and is handed the numbers of the animator it needs on the bench's `args`, in the order its comment gives. What a layer keeps from one tick to
//! the next goes into the game's block with the layer that owns it; a layer that owns nothing (the trunk) only needs the pose.
//!
//! A layer answers 0 = not a pose of this pool, or a role with no node (nothing written: the host then runs its own) · else RAN, with a bit for each list of
//! nodes it ran forward on the bench (`ran_list`): those nodes' rotations and origins are the host's to read back, since the TypeScript that follows may
//! read the run-forward bones a layer left (PoseFK is kept from one layer to the next).
//!
//! A LAYER THAT KEEPS STATE between ticks (the breath's phase, a spring, where a foot was put down) is handed it as doubles after its own numbers (from the sixty-fourth double) on the
//! bench's `args`, in an order its comment gives, and leaves it there as it now stands: the layer's COLUMNS of one character. Who keeps the columns between
//! ticks is not the layer's business (today the TypeScript animator's own fields, which every save already carries).
use crate::knobs::*;
use crate::maths::{acos, asin, atan2, cos, exp, hypot2, hypot3, hypot4, sin};
use prim::{floor, sqrt};
use crate::single::{get, put, S};
use crate::skeleton::{ask_blend, ask_mask, ask_sample, bench, fk_run, parent_rot, rotate_local, rotate_model, set_model_rot, set_unit, two_bone_ik, with_pose, Fk, Rig};

/// a layer ran
pub const RAN: u32 = 1;
/// …and left ALL of the run-forward bones on the bench current for the pose (a leg or an arm solved: skeleton.rs two_bone_ik)
pub const RAN_ALL: u32 = 2;
/// …and ran list `i` of knobs.rs forward on the bench
pub const fn ran_list(i: usize) -> u32 {
    4 << i
}

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
/// Math.min, Math.max, Math.sign as JavaScript has them (no number in, no number out; the two noughts in their order)
#[inline(always)]
fn min(a: f64, b: f64) -> f64 {
    if a != a || b != b {
        f64::NAN
    } else if a < b || (a == b && a.is_sign_negative()) {
        a
    } else {
        b
    }
}
#[inline(always)]
fn max(a: f64, b: f64) -> f64 {
    if a != a || b != b {
        f64::NAN
    } else if a > b || (a == b && b.is_sign_negative()) {
        a
    } else {
        b
    }
}
#[inline(always)]
fn sign(x: f64) -> f64 {
    if x > 0.0 {
        1.0
    } else if x < 0.0 {
        -1.0
    } else {
        x
    }
}
/// Math.round (halves go up; the noughts keep their sign as JavaScript's do)
#[inline(always)]
fn round(x: f64) -> f64 {
    if !(abs(x) < 4503599627370496.0) || x == 0.0 {
        return x;
    }
    let f = floor(x);
    let r = if x - f >= 0.5 { f + 1.0 } else { f };
    if r == 0.0 && x < 0.0 {
        -0.0
    } else {
        r
    }
}
/// Number.isFinite
#[inline(always)]
fn finite(x: f64) -> bool {
    abs(x) < f64::INFINITY
}
/// math/vec.ts lerp, damp, wrapAngle
#[inline(always)]
fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + (b - a) * t
}
#[inline(always)]
fn damp(a: f64, b: f64, rate: f64, dt: f64) -> f64 {
    lerp(a, b, 1.0 - exp(-rate * dt))
}
#[inline(always)]
fn wrap_angle(a: f64) -> f64 {
    let mut a = (a + PI) % (2.0 * PI);
    if a <= 0.0 {
        a += 2.0 * PI;
    }
    a - PI
}
/// character.ts Spring.step on two doubles of a layer's state: the value, then its speed
#[inline(always)]
fn spring(st: &mut [f64], i: usize, target: f64, w: f64, dt: f64) {
    let (x, v) = (st[i], st[i + 1]);
    let y = x - target;
    let j = v + y * w;
    let e = exp(-w * dt);
    st[i] = target + (y + j * dt) * e;
    st[i + 1] = (v - j * w * dt) * e;
}
/// quat.rotate
#[inline(always)]
fn q_rotate(q: &[f64; 4], v: &[f64; 3]) -> [f64; 3] {
    let (qx, qy, qz, qw) = (q[0], q[1], q[2], q[3]);
    let (tx, ty, tz) = (2.0 * (qy * v[2] - qz * v[1]), 2.0 * (qz * v[0] - qx * v[2]), 2.0 * (qx * v[1] - qy * v[0]));
    [v[0] + qw * tx + (qy * tz - qz * ty), v[1] + qw * ty + (qz * tx - qx * tz), v[2] + qw * tz + (qx * ty - qy * tx)]
}
/// v3.normalize
#[inline(always)]
fn normalize(a: [f64; 3]) -> [f64; 3] {
    let l = hypot3(a[0], a[1], a[2]);
    let l = if l == 0.0 || l != l { 1.0 } else { l };
    [a[0] / l, a[1] / l, a[2] / l]
}
const AX: [f64; 3] = [1.0, 0.0, 0.0];
const AY: [f64; 3] = [0.0, 1.0, 0.0];
/// CharacterAnimator.kneePole: the knee steered over the toes (the foot bone's Y runs along the foot)
fn knee_pole(foot_rot: &[f64; 4]) -> [f64; 3] {
    let f = q_rotate(foot_rot, &AY);
    let h = normalize([f[0], 0.0, f[2] + 1e-6]);
    normalize([h[0] * 0.25, 0.5, h[2] * 0.25 + 0.75])
}
/// CharacterAnimator.shift: node i's origin moved by a model-space offset
fn shift(d: &mut [S], rig: &Rig, i: usize, off: &[f64; 3]) {
    let p = parent_rot(rig, d, i);
    let l = q_rotate(&[-p[0], -p[1], -p[2], p[3]], off);
    let o = i * 10;
    let (x, y, z) = (get(d, o) + l[0], get(d, o + 1) + l[1], get(d, o + 2) + l[2]);
    put(d, o, x);
    put(d, o + 1, y);
    put(d, o + 2, z);
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
/// quat.axisAngle about any axis
#[inline(always)]
fn axis_angle(ax: &[f64; 3], a: f64) -> [f64; 4] {
    let s = sin(a / 2.0);
    [ax[0] * s, ax[1] * s, ax[2] * s, cos(a / 2.0)]
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
pub fn mv_trunk(g: usize, at: usize) -> u32 {
    let a = bench().args;
    let (w, og, ou, ost, aim_pass, phase, speed, rock, sho, head) = (a[0], a[1], a[2], a[3], a[4] != 0.0, a[5], a[6], a[7], a[8], a[9]);
    with_pose(g, at, |d, rig, n, _fk| {
        if !cast(n, &[SPINE1, SPINE2, SPINE3, NECK, HEAD]) {
            return 0;
        }
        if w < 0.003 {
            return RAN;
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
        RAN
    })
    .unwrap_or(0)
}

/// CharacterAnimator.longBody — the long gun, trunk half: the run's pumping of the shoulder line taken back out down the spine (about the model's upright, or
/// by the sprint's share about the trunk's own line), the upper spine bladed toward the gun, the lean, the head set to the stock. args: 0 w · 1 run ·
/// 2 sprint · 3 longSteady · the carriage's channels as they stand: 4 blade · 5 lean · 6 headYaw · 7 headPitch · 8 headRoll
pub fn long_body(g: usize, at: usize) -> u32 {
    let a = bench().args;
    let (w, run, sprint, long_steady, c_blade, c_lean, c_hy, c_hp, c_hr) = (a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7], a[8]);
    with_pose(g, at, |d, rig, n, fk| {
        if !cast(n, &[SPINE1, SPINE2, SPINE3, NECK, HEAD, SHOULDER_L, SHOULDER_R, UPPER_ARM_L, UPPER_ARM_R]) {
            return 0;
        }
        if w < 0.003 {
            return RAN;
        }
        let (s1, s2, s3, nk, hd) = (role(SPINE1), role(SPINE2), role(SPINE3), role(NECK), role(HEAD));
        let mut ran = RAN;
        let hold = run * clamp(long_steady, 0.0, 1.0) * k(LONG_STEADY_RUN_YAW) * w;
        if hold > 0.003 {
            fk.part(rig, d, list(FK_ARMS));
            ran |= ran_list(FK_ARMS);
            let (a, b) = (fk.at(role(UPPER_ARM_L)), fk.at(role(UPPER_ARM_R)));
            // (by the SPRINT's share of the pose: a jog is held as it always was, to the bit)
            let lv = if on(MOVES_ON) && on(MOVES_STEADY) && on(MOVES_STEADY_RUN) { clamp(sprint, 0.0, 1.0) } else { 0.0 };
            if lv > 0.0 {
                // … about the TRUNK'S OWN LINE: spine1 → between the clavicle roots
                let (ca, cb, sp1) = (fk.at(role(SHOULDER_L)), fk.at(role(SHOULDER_R)), fk.at(s1));
                let u = [ca[0] + (cb[0] - ca[0]) * 0.5 - sp1[0], ca[1] + (cb[1] - ca[1]) * 0.5 - sp1[1], ca[2] + (cb[2] - ca[2]) * 0.5 - sp1[2]];
                let ul = hypot3(u[0], u[1], u[2]);
                if ul > 1e-4 {
                    let inv = 1.0 / ul;
                    let un = [u[0] * inv, u[1] * inv, u[2] * inv];
                    let s = [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
                    let m = -(s[0] * un[0] + s[1] * un[1] + s[2] * un[2]);
                    let sp = [s[0] + un[0] * m, s[1] + un[1] * m, s[2] + un[2] * m];
                    let rf = [1.0 - un[0] * un[0], -un[0] * un[1], -un[0] * un[2]];
                    let cr = [rf[1] * sp[2] - rf[2] * sp[1], rf[2] * sp[0] - rf[0] * sp[2], rf[0] * sp[1] - rf[1] * sp[0]];
                    // the twist about it off square, the share of it to take out
                    let ty = atan2(cr[0] * un[0] + cr[1] * un[1] + cr[2] * un[2], rf[0] * sp[0] + rf[1] * sp[1] + rf[2] * sp[2]) * hold * lv;
                    if abs(ty) > 1e-4 {
                        turn(d, rig, s1, axis_angle(&un, -ty * 0.3));
                        turn(d, rig, s2, axis_angle(&un, -ty * 0.35));
                        turn(d, rig, s3, axis_angle(&un, -ty * 0.35));
                        turn(d, rig, nk, axis_angle(&un, ty));
                    }
                }
            }
            if lv < 1.0 {
                // the shoulder line's yaw off square (+ = turned to the figure's left), the share of it to take out
                let sy = atan2(b[2] - a[2], a[0] - b[0]) * hold * (1.0 - lv);
                if abs(sy) > 1e-4 {
                    turn(d, rig, s1, q_yaw(-sy * 0.3));
                    turn(d, rig, s2, q_yaw(-sy * 0.35));
                    turn(d, rig, s3, q_yaw(-sy * 0.35));
                }
            }
        }
        let (blade, lean) = (c_blade * DEGR * w, c_lean * DEGR * w);
        let (hy, hp, hr) = (c_hy * DEGR * w, c_hp * DEGR * w, c_hr * DEGR * w);
        if abs(blade) > 1e-5 {
            turn(d, rig, s2, q_yaw(blade * 0.4));
            turn(d, rig, s3, q_yaw(blade * 0.6));
            turn(d, rig, nk, q_yaw(-blade * 0.5));
            turn(d, rig, hd, q_yaw(-blade * 0.5));
        }
        if abs(lean) > 1e-5 {
            turn(d, rig, s1, q_pitch(lean * 0.45));
            turn(d, rig, s2, q_pitch(lean * 0.35));
            turn(d, rig, s3, q_pitch(lean * 0.2));
            turn(d, rig, nk, q_pitch(-lean * 0.4));
            turn(d, rig, hd, q_pitch(-lean * 0.6));
        }
        if abs(hy) + abs(hp) + abs(hr) > 1e-5 {
            turn(d, rig, nk, q_mul(&q_yaw(hy * 0.4), &q_pitch(hp * 0.4)));
            turn(d, rig, hd, q_mul(&q_roll(hr), &q_mul(&q_yaw(hy * 0.6), &q_pitch(hp * 0.6))));
        }
        ran
    })
    .unwrap_or(0)
}

/// character.ts mvOn: a row of MOVES, under its master switch
#[inline(always)]
fn mv_on(i: usize) -> bool {
    on(MOVES_ON) && on(i)
}
/// CharacterAnimator.mvHips: the pelvis moved by `off` (model space) with both feet held where the pose under it has them (the legs of `fk` current on entry;
/// all of it on exit)
fn mv_hips(d: &mut [S], rig: &Rig, fk: &mut Fk, off: &[f64; 3]) {
    let (foot_l, foot_r) = (role(FOOT_L), role(FOOT_R));
    let (fl, fr, rl, rr) = (fk.at(foot_l), fk.at(foot_r), fk.rot(foot_l), fk.rot(foot_r));
    shift(d, rig, role(HIPS), off);
    fk.part(rig, d, list(FK_LEGS));
    two_bone_ik(rig, d, fk.rot, fk.pos, role(THIGH_L), role(SHIN_L), foot_l, &fl, Some(&knee_pole(&rl)), 0.0, Some(&rl), false);
    two_bone_ik(rig, d, fk.rot, fk.pos, role(THIGH_R), role(SHIN_R), foot_r, &fr, Some(&knee_pole(&rr)), 0.0, Some(&rr), true);
}
const LEG_ROLES: [usize; 7] = [HIPS, THIGH_L, SHIN_L, FOOT_L, THIGH_R, SHIN_R, FOOT_R];

/// CharacterAnimator.mvPre — before the stances and the arms: the recoil's sums bled off (each at its own rate, by training), the muzzle's dip past a friend's
/// back, and the pelvis's share of how a side carries itself: a trained man with a long gun sinks onto bent knees when he moves and most of the walk's bob is
/// taken out, the feet held where the cycle has them.
/// args: 0 dt · 1 moveW · 2 standW · 3 trained · 4 longGun (1 or 0) · 5 dip · 6 the long gun's weight (lg.w.x) · 7 upperW.aim; then from 64 its columns:
/// the recoil's back · climb · yaw · rock · sho · head · glide (x, v) · hipY · the dip's spring (x, v) · what the trunk layer is handed: g · u · st · live
/// (1 or 0) · the dip shown · sightsLift
pub fn mv_pre(g: usize, at: usize) -> u32 {
    const COLS: usize = 17;
    const BACK: usize = 0;
    const CLIMB: usize = 1;
    const YAW: usize = 2;
    const ROCK: usize = 3;
    const SHO: usize = 4;
    const HEAD: usize = 5;
    const GLIDE: usize = 6;
    const HIP_Y: usize = 8;
    const DIP: usize = 9;
    const O_G: usize = 11;
    const O_U: usize = 12;
    const O_ST: usize = 13;
    const O_LIVE: usize = 14;
    const DIP_SHOWN: usize = 15;
    const SIGHTS_LIFT: usize = 16;
    let mut a = [0.0f64; L_IN + COLS];
    a.copy_from_slice(&bench().args[..L_IN + COLS]);
    let (dt, move_w, stand_w, trained, long_gun, dip, lg_w, aim) = (a[0], a[1], a[2], a[3], a[4] != 0.0, a[5], a[6], a[7]);
    let st = &mut a[L_IN..];
    let code = with_pose(g, at, |d, rig, n, fk| {
        if !cast(n, &LEG_ROLES) {
            return 0;
        }
        let t = clamp(trained, 0.0, 1.0);
        let kk = |p0: usize, p1: usize| lerp(k(p0), k(p1), t);
        let sum = st[BACK] + st[CLIMB] + abs(st[YAW]) + st[ROCK] + st[SHO] + st[HEAD];
        if sum > 0.0 {
            if sum < 1e-4 {
                for i in [BACK, CLIMB, YAW, ROCK, SHO, HEAD] {
                    st[i] = 0.0;
                }
            } else {
                let (eb, ec, er) = (exp(-dt / kk(RECOIL_BACK_TAU_0, RECOIL_BACK_TAU_1)), exp(-dt / kk(RECOIL_CLIMB_TAU_0, RECOIL_CLIMB_TAU_1)), exp(-dt / kk(RECOIL_ROCK_TAU_0, RECOIL_ROCK_TAU_1)));
                st[BACK] *= eb;
                st[CLIMB] *= ec;
                st[YAW] *= ec;
                st[ROCK] *= er;
                st[SHO] *= er;
                st[HEAD] *= er;
            }
        }
        // MOVES.entry: the muzzle down past a friend's back, and up again the moment he is clear of it
        let want = if mv_on(MOVES_ENTRY) && long_gun { clamp(dip, 0.0, 1.0) } else { 0.0 };
        if want > 0.0 || st[DIP] != 0.0 {
            let rate = if want > st[DIP] { k(DIP_RATE_IN) } else { k(DIP_RATE_OUT) };
            spring(st, DIP, want, rate, dt);
            if want == 0.0 && st[DIP] < 0.01 {
                st[DIP] = 0.0;
                st[DIP + 1] = 0.0;
            }
        }
        st[DIP_SHOWN] = clamp(st[DIP], 0.0, 1.0);
        st[SIGHTS_LIFT] = 0.0;
        st[O_G] = 0.0;
        st[O_U] = 0.0;
        st[O_ST] = 0.0;
        let mut ran = RAN;
        if mv_on(MOVES_GAIT) {
            let (lgw, tw) = (clamp(lg_w, 0.0, 1.0), smoothstep(0.5, 1.0, trained));
            spring(st, GLIDE, tw * move_w * lgw, 9.0, dt);
            st[O_G] = clamp(st[GLIDE], 0.0, 1.0);
            st[O_U] = (1.0 - smoothstep(0.0, 0.5, trained)) * lgw * stand_w;
            st[O_ST] = tw * lgw * stand_w;
            let drop = k(GAIT_DROP) * st[O_G] + k(GAIT_AIM_DROP) * st[O_ST] * aim * (1.0 - st[O_G]);
            if drop > 2e-4 || st[O_G] > 0.003 {
                fk.part(rig, d, list(FK_LEGS));
                let hy = fk.at(role(HIPS))[1];
                st[HIP_Y] = if finite(st[HIP_Y]) { st[HIP_Y] + (hy - st[HIP_Y]) * (1.0 - exp(-dt / 0.35)) } else { hy };
                let flat = clamp((st[HIP_Y] - hy) * k(GAIT_BOB) * st[O_G], -0.03, 0.03);
                mv_hips(d, rig, fk, &[0.0, flat - drop, 0.0]);
                ran |= RAN_ALL;
            } else {
                st[HIP_Y] = f64::NAN;
            }
        }
        st[O_LIVE] = if st[O_G] > 0.003 || st[O_U] > 0.003 || st[O_ST] > 0.003 || st[ROCK] > 1e-4 || st[SHO] > 1e-4 { 1.0 } else { 0.0 };
        ran
    })
    .unwrap_or(0);
    if code != 0 {
        bench().args[L_IN..L_IN + COLS].copy_from_slice(&a[L_IN..]);
    }
    code
}

/// CharacterAnimator.stanceBody — the stance, body half: the pelvis bladed toward the wall and settled onto soft knees over feet put back exactly where the
/// locomotion has them, the upper spine countering, a lean spread hips-first with the head levelled back, the stance's own head yaw and chin.
/// args: 0 w · 1 speed · 2 stand · the stance's springs as they stand: 3 blade · 4 squat · 5 lean · 6 headPitch · 7 headYaw · 8 stackPressSm
pub fn stance_body(g: usize, at: usize) -> u32 {
    let a = bench().args;
    let (w, speed, stand, s_blade, s_squat, s_lean, s_hp, s_hy, press) = (a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7], a[8]);
    with_pose(g, at, |d, rig, n, fk| {
        if !cast(n, &LEG_ROLES) || !cast(n, &[SPINE1, SPINE2, SPINE3, NECK, HEAD]) {
            return 0;
        }
        let (hips, s1, s2, s3, nk, hd) = (role(HIPS), role(SPINE1), role(SPINE2), role(SPINE3), role(NECK), role(HEAD));
        let (foot_l, foot_r) = (role(FOOT_L), role(FOOT_R));
        let blade = s_blade * DEGR * w * (1.0 - stand) * lerp(1.0, 0.5, smoothstep(0.3, 1.3, speed));
        let drop = max(0.0, s_squat) * w * (1.0 - stand);
        let replant = abs(blade) > 1e-4 || drop > 1e-4;
        let mut ran = RAN;
        let (mut fl, mut fr, mut rl, mut rr) = ([0.0; 3], [0.0; 3], [0.0; 4], [0.0; 4]);
        if replant {
            fk.part(rig, d, list(FK_LEGS));
            ran |= ran_list(FK_LEGS);
            (fl, fr, rl, rr) = (fk.at(foot_l), fk.at(foot_r), fk.rot(foot_l), fk.rot(foot_r));
        }
        if abs(blade) > 1e-4 {
            turn(d, rig, hips, q_yaw(blade));
            turn(d, rig, s1, q_yaw(-blade * 0.25));
            turn(d, rig, s2, q_yaw(-blade * 0.25));
            turn(d, rig, s3, q_yaw(-blade * 0.15));
        }
        if drop > 1e-4 {
            shift(d, rig, hips, &[0.0, -drop, 0.0]);
            turn(d, rig, hips, q_pitch(drop * 40.0 * DEGR));
        }
        let lean = s_lean * DEGR * w * (1.0 - 0.5 * stand);
        turn(d, rig, s1, q_pitch(lean * 0.45));
        turn(d, rig, s2, q_pitch(lean * 0.35));
        turn(d, rig, s3, q_pitch(lean * 0.2));
        let (hp, hy, unl) = (s_hp * DEGR * w, s_hy * DEGR * w, lean + drop * 40.0 * DEGR);
        turn(d, rig, nk, q_mul(&q_yaw(hy * 0.4), &q_pitch(-unl * 0.4 + hp * 0.4)));
        turn(d, rig, hd, q_mul(&q_yaw(hy * 0.6), &q_pitch(-unl * 0.6 + hp * 0.6)));
        if replant {
            fk.part(rig, d, list(FK_LEGS));
            two_bone_ik(rig, d, fk.rot, fk.pos, role(THIGH_L), role(SHIN_L), foot_l, &fl, Some(&knee_pole(&rl)), 0.6, Some(&rl), false);
            two_bone_ik(rig, d, fk.rot, fk.pos, role(THIGH_R), role(SHIN_R), foot_r, &fr, Some(&knee_pole(&rr)), 0.6, Some(&rr), true);
            ran |= RAN_ALL;
        }
        if press != 0.0 {
            shift(d, rig, hips, &[-press * w * (1.0 - stand), 0.0, 0.0]);
        }
        ran
    })
    .unwrap_or(0)
}

/// CharacterAnimator.jogShape — the run clip as the game shows it: its bounce flattened toward the mean by phase, and at a jog its stride warped shorter at
/// the feet about the gait's middle. `a`: 0 ph · the clip's pelvis height at the two samples about the phase: 1, 2 · 3 its mean · 4 runK · 5 jogMid
fn jog_shape(g: usize, at: usize, a: &[f64]) -> u32 {
    let (ph, y0, y1, mean, run_k, mid) = (a[0], a[1], a[2], a[3], a[4], a[5]);
    with_pose(g, at, |d, rig, n, fk| {
        if !cast(n, &LEG_ROLES) {
            return 0;
        }
        let fi = ph * 64.0;
        let fr = fi - floor(fi);
        let hy = y0 * (1.0 - fr) + y1 * fr;
        shift(d, rig, role(HIPS), &[0.0, -(hy - mean) * (1.0 - k(BOB_KEEP)), 0.0]);
        if !(run_k < 0.999) {
            return RAN;
        }
        fk.part(rig, d, list(FK_LEGS));
        let (foot_l, foot_r) = (role(FOOT_L), role(FOOT_R));
        let (fl, fr, rl, rr) = (fk.at(foot_l), fk.at(foot_r), fk.rot(foot_l), fk.rot(foot_r));
        let tgt = |f: &[f64; 3]| -> [f64; 3] { [f[0], f[1] * lerp(1.0, run_k, 0.5), mid + (f[2] - mid) * run_k] };
        two_bone_ik(rig, d, fk.rot, fk.pos, role(THIGH_L), role(SHIN_L), foot_l, &tgt(&fl), Some(&knee_pole(&rl)), 0.0, Some(&rl), false);
        two_bone_ik(rig, d, fk.rot, fk.pos, role(THIGH_R), role(SHIN_R), foot_r, &tgt(&fr), Some(&knee_pole(&rr)), 0.0, Some(&rr), true);
        RAN | RAN_ALL
    })
    .unwrap_or(0)
}

/// CharacterAnimator.sprintShape — the sprint clip as the game shows it: its stride warped at the ankles about the gait's middle, heights eased half way with
/// it about the ankles' floor, never an ankle so low its toe goes through the floor. `a`: 0 sprintMid · 1 sprintFloorY
fn sprint_shape(g: usize, at: usize, a: &[f64]) -> u32 {
    let (mid, y0) = (a[0], a[1]);
    with_pose(g, at, |d, rig, n, fk| {
        if !cast(n, &LEG_ROLES) || !cast(n, &[TOE_L, TOE_R]) {
            return 0;
        }
        let kk = k(SPRINT_WARP_STRIDE);
        if kk > 0.999 {
            return RAN;
        }
        fk.part(rig, d, list(FK_LEGS));
        let ky = lerp(1.0, kk, 0.5);
        let (foot_l, foot_r) = (role(FOOT_L), role(FOOT_R));
        let (fl, fr, rl, rr) = (fk.at(foot_l), fk.at(foot_r), fk.rot(foot_l), fk.rot(foot_r));
        let (tl0, tr0) = (fk.at(role(TOE_L))[1] - fl[1], fk.at(role(TOE_R))[1] - fr[1]);
        let tgt = |f: &[f64; 3], toe_off: f64| -> [f64; 3] {
            let mut y = y0 + (f[1] - y0) * ky;
            if y + toe_off < k(TOE_FLOOR_Y) {
                y = k(TOE_FLOOR_Y) - toe_off;
            }
            [f[0], y, mid + (f[2] - mid) * kk]
        };
        two_bone_ik(rig, d, fk.rot, fk.pos, role(THIGH_L), role(SHIN_L), foot_l, &tgt(&fl, tl0), Some(&knee_pole(&rl)), 0.0, Some(&rl), false);
        two_bone_ik(rig, d, fk.rot, fk.pos, role(THIGH_R), role(SHIN_R), foot_r, &tgt(&fr, tr0), Some(&knee_pole(&rr)), 0.0, Some(&rr), true);
        RAN | RAN_ALL
    })
    .unwrap_or(0)
}

/// a clip sampled into a pose as it stands (sampleClip with no copy first: the nodes the clip does not write keep what the pose held)
fn clip_at(g: usize, at: usize, nodes: usize, clip: f64, t: f64) -> bool {
    clip >= 0.0 && ask_sample(g, at, nodes, clip as usize, t, true)
}
/// CharacterAnimator.sampleJog — the jog clip at phase `ph` (its time `t`), its SEAM closed: the clip's last key is not its first, and each joint that
/// jumps there is carried half the jump back at the cycle's end and half forward at its start (the rig's table JOG_SEAM, six numbers a joint: where its
/// floats start in a pose · 0 an origin, 1 a rotation · the jump: an offset, or an axis and, last, its angle)
fn sample_jog(g: usize, at: usize, nodes: usize, clip: f64, t: f64, ph: f64) -> bool {
    if !clip_at(g, at, nodes, clip, t) {
        return false;
    }
    let kk = ph - floor(ph) - 0.5;
    with_pose(g, at, |d, _rig, _n, _fk| {
        for s in nums(JOG_SEAM).chunks_exact(6) {
            let o = s[0] as usize;
            if o + 4 > d.len() {
                continue;
            }
            if s[1] == 0.0 {
                let (x, y, z) = (get(d, o) + s[2] * kk, get(d, o + 1) + s[3] * kk, get(d, o + 2) + s[4] * kk);
                put(d, o, x);
                put(d, o + 1, y);
                put(d, o + 2, z);
            } else {
                let q = q_mul(&[get(d, o), get(d, o + 1), get(d, o + 2), get(d, o + 3)], &axis_angle(&[s[2], s[3], s[4]], s[5] * kk));
                put(d, o, q[0]);
                put(d, o + 1, q[1]);
                put(d, o + 2, q[2]);
                put(d, o + 3, q[3]);
            }
        }
    })
    .is_some()
}
/// CharacterAnimator.standPose — the standing locomotion: the idle, and over it the cycle (the walk, the jog laid on it, or the jog and the sprint laid
/// together), each run shaped as the game shows it before the blend. The cycle's own poses are left in the scratch poses for the layers that read them.
/// args: 0 ph · 1 wIdle · 2 wWalk · 3 wJog · 4 wSprint · 5 the idle's clock · the clips' numbers and times: 6 the idle · 7 the jog · 8 its time · 9 the
/// sprint · 10 its time · 11 the walk · 12 its time · jogShape's numbers: 13 ph again‥18 · sprintShape's: 19, 20
pub fn stand_pose(g: usize, at: usize, at_b: usize, at_c: usize) -> u32 {
    let mut a = [0.0f64; 21];
    a.copy_from_slice(&bench().args[..21]);
    let (ph, w_idle, w_walk, w_jog, w_sprint, idle_t) = (a[0], a[1], a[2], a[3], a[4], a[5]);
    let Some(n) = with_pose(g, at, |_d, _rig, n, _fk| n) else {
        return 0;
    };
    let moving = w_walk + w_jog + w_sprint > 0.001;
    let mut code = RAN;
    // (the idle is not sampled when the cycle's pose is about to replace all of it)
    if !(moving && 1.0 - w_idle >= 1.0) && !clip_at(g, at, n, a[6], idle_t) {
        return 0;
    }
    if moving {
        let mut shaped = |c: u32| {
            code |= c;
            c != 0
        };
        if w_sprint > 0.001 {
            if !sample_jog(g, at_b, n, a[7], a[8], ph) || !shaped(jog_shape(g, at_b, &a[13..19])) {
                return 0;
            }
            if !clip_at(g, at_c, n, a[9], a[10]) || !shaped(sprint_shape(g, at_c, &a[19..21])) || !ask_blend(g, at_b, at_c, w_sprint, false, at_b, n) {
                return 0;
            }
        } else {
            if !clip_at(g, at_b, n, a[11], a[12]) {
                return 0;
            }
            if w_jog > 0.001 && (!sample_jog(g, at_c, n, a[7], a[8], ph) || !shaped(jog_shape(g, at_c, &a[13..19])) || !ask_blend(g, at_b, at_c, w_jog, false, at_b, n)) {
                return 0;
            }
        }
        if !ask_blend(g, at, at_b, 1.0 - w_idle, false, at, n) {
            return 0;
        }
    }
    code
}
/// CharacterAnimator.crouchPose — the crouch over the stand pose: its idle, the crouch walk's cycle laid on it by how fast he goes, and that laid over the
/// pose by the crouch shown. args: 0 wMove · 1 the crouch shown · 2 the idle's clock · 3 the crouched idle's number · 4 the crouch walk's · 5 its time
pub fn crouch_pose(g: usize, at: usize, at_b: usize, at_c: usize) -> u32 {
    let mut a = [0.0f64; 6];
    a.copy_from_slice(&bench().args[..6]);
    let (w_move, crouch) = (a[0], a[1]);
    let Some(n) = with_pose(g, at, |_d, _rig, n, _fk| n) else {
        return 0;
    };
    if !(w_move >= 1.0) && !clip_at(g, at_b, n, a[3], a[2]) {
        return 0;
    }
    if w_move > 0.001 && (!clip_at(g, at_c, n, a[4], a[5]) || !ask_blend(g, at_b, at_c, w_move, false, at_b, n)) {
        return 0;
    }
    if !ask_blend(g, at, at_b, crouch, false, at, n) {
        return 0;
    }
    RAN
}

#[inline(always)]
fn dist3(a: &[f64; 3], b: &[f64; 3]) -> f64 {
    hypot3(a[0] - b[0], a[1] - b[1], a[2] - b[2])
}
/// CharacterAnimator.softKnees — SOFT_KNEE: each leg re-solved toward its own ankle with the hip→ankle reach eased below full extension. args: 0 w
pub fn soft_knees(g: usize, at: usize) -> u32 {
    let w = bench().args[0];
    with_pose(g, at, |d, rig, n, fk| {
        if !cast(n, &LEG_ROLES) {
            return 0;
        }
        fk.part(rig, d, list(FK_LEGS));
        let mut cur = false;
        for (thigh, shin, foot) in [(role(THIGH_L), role(SHIN_L), role(FOOT_L)), (role(THIGH_R), role(SHIN_R), role(FOOT_R))] {
            let (ph, pk, pa) = (fk.at(thigh), fk.at(shin), fk.at(foot));
            let (l, dd) = (dist3(&ph, &pk) + dist3(&pk, &pa), dist3(&ph, &pa));
            let r = dd / l;
            if r <= k(SOFT_KNEE_FROM) {
                continue;
            }
            let span = k(SOFT_KNEE_TO) - k(SOFT_KNEE_FROM);
            let x = (r - k(SOFT_KNEE_FROM)) / span;
            let r_eased = k(SOFT_KNEE_FROM) + span * (1.0 - exp(-x));
            let r_new = r + (r_eased - r) * w;
            let ar = fk.rot(foot);
            let tgt = [ph[0] + (pa[0] - ph[0]) * r_new / r, ph[1] + (pa[1] - ph[1]) * r_new / r, ph[2] + (pa[2] - ph[2]) * r_new / r];
            two_bone_ik(rig, d, fk.rot, fk.pos, thigh, shin, foot, &tgt, Some(&knee_pole(&ar)), 0.0, Some(&ar), cur);
            cur = true;
        }
        RAN | if cur { RAN_ALL } else { ran_list(FK_LEGS) }
    })
    .unwrap_or(0)
}

/// character.ts bump
#[inline(always)]
fn bump(t: f64, a: f64, b: f64, c: f64, d: f64) -> f64 {
    smoothstep(a, b, t) * (1.0 - smoothstep(c, d, t))
}
/// CharacterAnimator.runPlant — the run's stance feet: through each foot's stance window of the sampled phase the toe is put on the floor and carried back at
/// the ground's speed, the offset it is left holding bled away up the swing.
/// args: 0 ph · 1 s · 2 dt · 3 wRun · 4 wSprint · 5 reverse (1 or 0) · the mid-stances: 6 the jog's left · 7 the sprint's left · 8 the jog's right ·
/// 9 the sprint's right; then from 64 its columns: the left foot's dev · pz · the right's dev · pz
pub fn run_plant(g: usize, at: usize) -> u32 {
    const COLS: usize = 4;
    let mut a = [0.0f64; L_IN + COLS];
    a.copy_from_slice(&bench().args[..L_IN + COLS]);
    let (ph, s, dt, w_run, w_sprint, reverse) = (a[0], a[1], a[2], a[3], a[4], a[5] != 0.0);
    let mids = [lerp(a[6], a[7], w_sprint), lerp(a[8], a[9], w_sprint)];
    let st = &mut a[L_IN..];
    let code = with_pose(g, at, |d, rig, n, fk| {
        if !cast(n, &LEG_ROLES) || !cast(n, &[TOE_L, TOE_R]) {
            return 0;
        }
        let dz = if reverse { 1.0 } else { -1.0 } * s * dt;
        fk.part(rig, d, list(FK_LEGS));
        let legs = [(role(THIGH_L), role(SHIN_L), role(FOOT_L), role(TOE_L)), (role(THIGH_R), role(SHIN_R), role(FOOT_R), role(TOE_R))];
        let mut cur = false;
        for i in 0..2 {
            let (thigh, shin, foot, toe) = legs[i];
            let (dev, pz) = (i * 2, i * 2 + 1);
            let mut x = ph - mids[i];
            x -= round(x);
            let w = bump(x, k(RUN_LINGER_ON_0), k(RUN_LINGER_ON_1), k(RUN_LINGER_OFF_0), k(RUN_LINGER_OFF_1)) * w_run;
            let tp = fk.at(toe);
            let miss = if finite(st[pz]) { dz - (tp[2] - st[pz]) } else { 0.0 };
            st[pz] = tp[2];
            st[dev] = clamp((st[dev] + if abs(miss) < 0.2 { miss } else { 0.0 } * w) * exp(-(1.0 - w) * dt / max(k(RUN_LINGER_BLEED), 1e-3)), -0.25, 0.25);
            if w < 1e-3 && abs(st[dev]) < 5e-4 {
                st[dev] = 0.0;
                continue;
            }
            let (ap, ar) = (fk.at(foot), fk.rot(foot));
            two_bone_ik(rig, d, fk.rot, fk.pos, thigh, shin, foot, &[ap[0], ap[1] - (tp[1] - k(TOE_FLOOR_Y)) * w, ap[2] + st[dev]], Some(&knee_pole(&ar)), 0.0, Some(&ar), cur);
            cur = true;
        }
        RAN | if cur { RAN_ALL } else { ran_list(FK_LEGS) }
    })
    .unwrap_or(0);
    if code != 0 {
        bench().args[L_IN..L_IN + COLS].copy_from_slice(&a[L_IN..]);
    }
    code
}

/// CharacterAnimator.sneakBody — deliberate movement and the crouch's centre of gravity by pace, the legs and the trunk: the stride warped at the feet about
/// the gait's middle, both feet and the pelvis riding the surge, the pelvis lower and tipped, the torso further over with the head level back out of it.
/// args: 0 Dl · 1 pace · 2 soft · 3 surge · 4 dsh · 5 upperW.aim · 6 gait.mid
pub fn sneak_body(g: usize, at: usize) -> u32 {
    let a = bench().args;
    let (dl, pace, soft, surge, dsh, aim, mid) = (a[0], a[1], a[2], a[3], a[4], a[5], a[6]);
    with_pose(g, at, |d, rig, n, fk| {
        if !cast(n, &LEG_ROLES) || !cast(n, &[SPINE1, SPINE2, SPINE3, NECK, HEAD]) {
            return 0;
        }
        let kk = (1.0 + k(SNEAK_STRIDE) * dl) * (1.0 + k(SNEAK_PACE_STRIDE) * pace) * (1.0 + k(DASH_STRIDE) * dsh);
        let drop = k(SNEAK_DROP) * dl + k(SNEAK_PACE_DROP) * pace + k(SNEAK_SOFT_DROP) * soft + k(DASH_DROP) * dsh;
        let tip = drop * 25.0 * DEGR;
        let lean = (k(SNEAK_LEAN) * dl + k(SNEAK_PACE_LEAN) * pace + k(DASH_LEAN) * dsh) * DEGR * (1.0 - aim);
        let mut ran = RAN;
        if kk > 1.0005 || drop > 2e-4 || abs(surge) > 2e-4 {
            fk.part(rig, d, list(FK_LEGS));
            let (foot_l, foot_r) = (role(FOOT_L), role(FOOT_R));
            let (fl, fr, rl, rr) = (fk.at(foot_l), fk.at(foot_r), fk.rot(foot_l), fk.rot(foot_r));
            let tgt = |f: &[f64; 3]| -> [f64; 3] { [f[0], f[1], mid + (f[2] - mid) * kk + surge] };
            shift(d, rig, role(HIPS), &[0.0, -drop, surge]);
            if tip > 1e-5 {
                turn(d, rig, role(HIPS), q_pitch(tip));
            }
            fk.part(rig, d, list(FK_LEGS));
            two_bone_ik(rig, d, fk.rot, fk.pos, role(THIGH_L), role(SHIN_L), foot_l, &tgt(&fl), Some(&knee_pole(&rl)), 0.0, Some(&rl), false);
            two_bone_ik(rig, d, fk.rot, fk.pos, role(THIGH_R), role(SHIN_R), foot_r, &tgt(&fr), Some(&knee_pole(&rr)), 0.0, Some(&rr), true);
            ran |= RAN_ALL;
        }
        if lean + tip > 1e-5 {
            turn(d, rig, role(SPINE1), q_pitch(lean * 0.45));
            turn(d, rig, role(SPINE2), q_pitch(lean * 0.35));
            turn(d, rig, role(SPINE3), q_pitch(lean * 0.2));
            let unl = lean + tip;
            turn(d, rig, role(NECK), q_pitch(-unl * 0.4));
            turn(d, rig, role(HEAD), q_pitch(-unl * 0.6));
        }
        ran
    })
    .unwrap_or(0)
}

/// CharacterAnimator.steadyLayer — MOVES.steady: this frame's swing of the crouch cycle (its own sample, the pose at `at_c`, against its own mean) taken out
/// by MV.steady's shares: the pelvis's roll at the hips with the chest handed the same turn back, a little of the hips' sideways travel, both ankles put back
/// nearer his middle and the knees swung in over them; at setting 2 the pelvis ridden up by as much as both legs can pay out.
/// args: 0 w · 1 base · 2 sunk · the cycle's mean: 3 its pelvis roll · 4 its hips' sideways set · 5 steadyUp
pub fn steady_layer(g: usize, at: usize, at_c: usize) -> u32 {
    let a = bench().args;
    let (w, base, sunk, mean_roll, mean_x, steady_up) = (a[0], a[1], a[2], a[3], a[4], a[5]);
    let roll_now = |fk: &Fk| -> f64 {
        let (a, b) = (fk.at(role(THIGH_L)), fk.at(role(THIGH_R)));
        atan2(a[1] - b[1], a[0] - b[0])
    };
    // this frame's swing: the cycle's sample off the cycle's mean
    let Some(Some((d_roll, d_x))) = with_pose(g, at_c, |d, rig, n, fk| {
        if !cast(n, &LEG_ROLES) || !cast(n, &[SPINE3]) {
            return None;
        }
        fk.part(rig, d, list(FK_LEGS));
        Some((roll_now(fk) - mean_roll, fk.at(role(HIPS))[0] - mean_x))
    }) else {
        return 0;
    };
    with_pose(g, at, |d, rig, _n, fk| {
        let (hips, foot_l, foot_r) = (role(HIPS), role(FOOT_L), role(FOOT_R));
        let legs = [(role(THIGH_L), role(SHIN_L), foot_l), (role(THIGH_R), role(SHIN_R), foot_r)];
        fk.part(rig, d, list(FK_LEGS));
        let (fl, fr, rl, rr) = (fk.at(foot_l), fk.at(foot_r), fk.rot(foot_l), fk.rot(foot_r));
        let (tl, tr, sl, sr) = (fk.at(legs[0].0), fk.at(legs[1].0), fk.at(legs[0].1), fk.at(legs[1].1));
        // each knee's set off its hip, and its height, as the cycle has them
        let (ol, or) = ([sl[0] - tl[0], sl[1] - tl[1], sl[2] - tl[2]], [sr[0] - tr[0], sr[1] - tr[1], sr[2] - tr[2]]);
        let (yl, yr) = (sl[1], sr[1]);
        let (a, sx) = (d_roll * k(STEADY_ROLL) * w, d_x * k(STEADY_SWAY) * w);
        if abs(a) > 1e-6 {
            turn(d, rig, hips, q_roll(-a));
            turn(d, rig, role(SPINE3), q_roll(a));
        }
        if abs(sx) > 1e-6 {
            shift(d, rig, hips, &[-sx, 0.0, 0.0]);
        }
        let kx = 1.0 - (1.0 - k(STEADY_FEET)) * w * base;
        fk.part(rig, d, list(FK_LEGS));
        let (al, ar) = ([fl[0] * kx, fl[1], fl[2]], [fr[0] * kx, fr[1], fr[2]]);
        if steady_up > 0.0 {
            let mut up = steady_up;
            for (i, at) in [al, ar].iter().enumerate() {
                let (hip, knee, foot) = legs[i];
                let h = fk.at(hip);
                let len = (dist3(&h, &fk.at(knee)) + dist3(&fk.at(knee), &fk.at(foot))) * k(STEADY_REACH);
                let (hx, hy, hz) = (h[0] - at[0], h[1] - at[1], h[2] - at[2]);
                let q = len * len - hx * hx - hz * hz;
                up = min(up, if q > 0.0 { sqrt(q) - hy } else { 0.0 });
            }
            if up > 1e-5 {
                shift(d, rig, hips, &[0.0, up, 0.0]);
                fk.part(rig, d, list(FK_LEGS));
            }
        }
        let (kn, ko) = (k(STEADY_KNEE) * w * base, k(STEADY_KNEE_OUT) * w * sunk);
        let (hl, hr) = (smoothstep(0.10, 0.22, yl), smoothstep(0.10, 0.22, yr));
        let (kl, kr) = (1.0 - kn * lerp(k(STEADY_KNEE_LOW), 1.0, hl) + ko * (1.0 - hl), 1.0 - kn * lerp(k(STEADY_KNEE_LOW), 1.0, hr) + ko * (1.0 - hr));
        let (pl, pr) = ([ol[0] * kl, ol[1], ol[2]], [or[0] * kr, or[1], or[2]]);
        two_bone_ik(rig, d, fk.rot, fk.pos, legs[0].0, legs[0].1, foot_l, &al, Some(&pl), 1.0, Some(&rl), false);
        two_bone_ik(rig, d, fk.rot, fk.pos, legs[1].0, legs[1].1, foot_r, &ar, Some(&pr), 1.0, Some(&rr), true);
        RAN | RAN_ALL
    })
    .unwrap_or(0)
}

#[inline(always)]
fn add3(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}
#[inline(always)]
fn sub3(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}
#[inline(always)]
fn mad3(a: [f64; 3], b: [f64; 3], s: f64) -> [f64; 3] {
    [a[0] + b[0] * s, a[1] + b[1] * s, a[2] + b[2] * s]
}
#[inline(always)]
fn conj(q: [f64; 4]) -> [f64; 4] {
    [-q[0], -q[1], -q[2], q[3]]
}
/// `!x` of a number: nothing or no number
#[inline(always)]
fn falsy(x: f64) -> bool {
    x == 0.0 || x != x
}
/// CharacterAnimator.untwist: where a point must be posed below spine3 to END UP at `h` after the chest twist still to come (the run-forward bones current)
fn untwist(fk: &Fk, tw_y: f64, tw_p: f64, h: [f64; 3]) -> [f64; 3] {
    if falsy(tw_y) && falsy(tw_p) {
        return h;
    }
    let (e1, e2) = (q_yaw(tw_y * 0.25), q_yaw(tw_y * 0.35));
    let mut e3 = q_yaw(tw_y * 0.4);
    if !falsy(tw_p) {
        let right = q_rotate(&e3, &AX);
        e3 = q_mul(&axis_angle(&right, tw_p), &e3);
    }
    let (ie1, ie2, ie3) = (conj(e1), conj(e2), conj(e3));
    let (p1, p2, p3) = (fk.at(role(SPINE1)), fk.at(role(SPINE2)), fk.at(role(SPINE3)));
    let p2t = add3(q_rotate(&e1, &sub3(p2, p1)), p1);
    let p3t = add3(q_rotate(&e2, &sub3(add3(q_rotate(&e1, &sub3(p3, p1)), p1), p2t)), p2t);
    let x1 = add3(q_rotate(&ie3, &sub3(h, p3t)), p3t);
    let x2 = add3(q_rotate(&ie2, &sub3(x1, p2t)), p2t);
    add3(q_rotate(&ie1, &sub3(x2, p1)), p1)
}
/// CharacterAnimator.lookAt: the neck and head turned by `w` of the way from where the eyes look to `target` (model space); runs the pose forward first
fn look_at(d: &mut [S], rig: &Rig, fk: &mut Fk, gaze_local: &[f64; 3], target: [f64; 3], w: f64, roll: f64) {
    fk_run(rig, d, fk.rot, fk.pos);
    let (nk, hd) = (role(NECK), role(HEAD));
    let hr = fk.rot(hd);
    let gaze = q_rotate(&hr, gaze_local);
    let eyes = mad3(mad3(fk.at(hd), q_rotate(&hr, &AY), 0.1), gaze, 0.08);
    let want = normalize(sub3(target, eyes));
    let yaw_e = clamp(wrap_angle(atan2(want[0], want[2]) - atan2(gaze[0], gaze[2])), -0.7, 0.7);
    let pitch_e = clamp(asin(clamp(gaze[1], -1.0, 1.0)) - asin(clamp(want[1], -1.0, 1.0)), -0.6, 0.8);
    let look = |kk: f64| q_mul(&q_yaw(yaw_e * kk * w), &q_pitch(pitch_e * kk * w));
    turn(d, rig, nk, look(0.4));
    turn(d, rig, hd, if !falsy(roll) { q_mul(&q_roll(roll), &look(0.6)) } else { look(0.6) });
}

/// CharacterAnimator.creepBody — the close-in mode's carriage, the body: the pelvis sunk and set back with the feet put back where the gait has them, the heft
/// of his stride read off the feet (his shoulders round against the step and over the foot he is on), the spine bowed forward hips first with the head
/// levelled back out of it and turned onto what he creeps at.
/// args: 0 e · 1 free · 2 cr · 3 gw · 4 dt · 5 twY · 6 twP · 7 lowS · 8‥10 lowShape · 11‥16 heftShape · 17 speed · 18 the watch's weight (wt.w) ·
/// 19 creepAt is there (1 or 0) · 20‥22 creepAt (world) · 23 bodyYawIn · 24‥26 posIn · 27‥29 gazeLocal; then from 64 its columns, the heft's springs:
/// t · s · sv · a · av · w · wv · v · vv. It answers on args 0, 1: the arms' share of his stride, and his surge (what creepArms is handed).
pub fn creep_body(g: usize, at: usize) -> u32 {
    const COLS: usize = 9;
    const T: usize = 0;
    const HS_: usize = 1;
    const HA: usize = 3;
    const HW: usize = 5;
    const HV: usize = 7;
    let mut a = [0.0f64; L_IN + COLS];
    a.copy_from_slice(&bench().args[..L_IN + COLS]);
    let (e, free, cr, gw, dt, tw_y, tw_p, lo, speed, wt_w, at_there, yin) = (a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7], a[17], a[18], a[19] != 0.0, a[23]);
    let (ls, hsh) = ([a[8], a[9], a[10]], [a[11], a[12], a[13], a[14], a[15], a[16]]);
    let (creep_at, p0, gaze_local) = ([a[20], a[21], a[22]], [a[24], a[25], a[26]], [a[27], a[28], a[29]]);
    let mut out = (0.0, 0.0);
    let st = &mut a[L_IN..];
    let code = with_pose(g, at, |d, rig, n, fk| {
        if !cast(n, &LEG_ROLES) || !cast(n, &[SPINE1, SPINE2, SPINE3, NECK, HEAD]) {
            return 0;
        }
        let (hips, s1, s2, s3, nk, hd) = (role(HIPS), role(SPINE1), role(SPINE2), role(SPINE3), role(NECK), role(HEAD));
        let (foot_l, foot_r) = (role(FOOT_L), role(FOOT_R));
        let (mut hs, mut ha, mut hw, mut hg) = (0.0, 0.0, 0.0, 0.0);
        fk.part(rig, d, list(FK_LEGS));
        let (fl, fr, rl, rr) = (fk.at(foot_l), fk.at(foot_r), fk.rot(foot_l), fk.rot(foot_r));
        if gw > 0.0 {
            // CREEP.heft: read off the feet as they are under him, each a beat behind the last
            let h = min(dt, k(HEFT_DT));
            st[T] += dt;
            let step = |st: &mut [f64], i: usize, to: f64, w0: f64, w1: f64| {
                let (x, v) = (st[i], st[i + 1]);
                let nv = v + (w0 * w0 * (to - x) - 2.0 * w1 * w0 * v) * h;
                st[i] = x + nv * h;
                st[i + 1] = nv;
            };
            step(st, HS_, clamp((fl[2] - fr[2]) / k(HEFT_STRIDE), -1.0, 1.0), k(HEFT_TRUNK_0), k(HEFT_TRUNK_1));
            let to = st[HS_];
            step(st, HA, to, k(HEFT_ARM_0), k(HEFT_ARM_1));
            step(st, HW, clamp((fr[1] - fl[1]) / k(HEFT_LIFT), -1.0, 1.0), k(HEFT_BODY_0), k(HEFT_BODY_1));
            step(st, HV, clamp(abs(speed) / k(HEFT_RUN), 0.0, 1.0), k(HEFT_GO_0), k(HEFT_GO_1));
            hs = st[HS_] * gw;
            ha = st[HA] * gw;
            hw = st[HW] * gw;
            hg = (clamp(abs(speed) / k(HEFT_RUN), 0.0, 1.0) - st[HV]) * gw * hsh[5];
        }
        shift(d, rig, hips, &[hw * hsh[2], -(lerp(k(CREEP_DROP_0), k(CREEP_DROP_1), cr) + ls[0] * lo) * e - abs(hs) * hsh[4], -(lerp(k(CREEP_BACK_0), k(CREEP_BACK_1), cr) + ls[1] * lo) * e * free]);
        fk.part(rig, d, list(FK_LEGS));
        two_bone_ik(rig, d, fk.rot, fk.pos, role(THIGH_L), role(SHIN_L), foot_l, &fl, Some(&knee_pole(&rl)), 0.0, Some(&rl), false);
        two_bone_ik(rig, d, fk.rot, fk.pos, role(THIGH_R), role(SHIN_R), foot_r, &fr, Some(&knee_pole(&rr)), 0.0, Some(&rr), true);
        let lean = (lerp(k(CREEP_LEAN_0), k(CREEP_LEAN_1), cr) + ls[2] * lo + hsh[3] * gw - hg * 9.0 + if gw > 0.0 { k(HEFT_BREATH) * gw * sin(st[T] * PI * 2.0 * k(HEFT_BREATH_HZ)) } else { 0.0 }) * DEGR * e * free;
        if gw > 0.0 {
            // the shoulders come round against the step and over the foot he is on; the head stays where it was looking
            let (tw, rl) = (hs * hsh[0] * DEGR, hw * hsh[0] * 0.3 * DEGR);
            turn(d, rig, s1, q_mul(&q_yaw(tw * 0.25), &q_roll(rl * 0.3)));
            turn(d, rig, s2, q_mul(&q_yaw(tw * 0.35), &q_roll(rl * 0.35)));
            turn(d, rig, s3, q_mul(&q_yaw(tw * 0.4), &q_roll(rl * 0.35)));
            turn(d, rig, nk, q_mul(&q_yaw(-tw * 0.5), &q_roll(-rl * 0.5)));
            turn(d, rig, hd, q_mul(&q_yaw(-tw * 0.5), &q_roll(-rl * 0.5)));
        }
        if lean > 1e-5 {
            turn(d, rig, s1, q_pitch(lean * 0.4));
            turn(d, rig, s2, q_pitch(lean * 0.35));
            turn(d, rig, s3, q_pitch(lean * 0.25));
            turn(d, rig, nk, q_pitch(-lean * 0.4));
            turn(d, rig, hd, q_pitch(-lean * 0.5));
        }
        let lw = k(CREEP_LOOK) * e * free * (1.0 - clamp(wt_w, 0.0, 1.0));
        if at_there && lw > 0.003 {
            fk_run(rig, d, fk.rot, fk.pos);
            // (toModelP: the world point in the model's frame)
            let (cy, sy) = (cos(yin), sin(yin));
            let (dx, dz) = (creep_at[0] - p0[0], creep_at[2] - p0[2]);
            let m = [dx * cy - dz * sy, creep_at[1] - p0[1], dx * sy + dz * cy];
            let target = untwist(fk, tw_y, tw_p, m);
            look_at(d, rig, fk, &gaze_local, target, lw, 0.0);
        }
        out = (ha, hg);
        RAN | RAN_ALL
    })
    .unwrap_or(0);
    if code != 0 {
        let b = bench();
        b.args[L_IN..L_IN + COLS].copy_from_slice(&a[L_IN..]);
        b.args[0] = out.0;
        b.args[1] = out.1;
    }
    code
}

// the life layer's columns (after its own numbers): the breath and the body's mass, then the foot lock, then a foot (two of them)
/// character.ts curve: one hand-animated channel of a one-shot — Catmull-Rom through (time, value) keys, flat at the first and the last key, clamped outside
fn curve(t: f64, k: &[[f64; 2]]) -> f64 {
    let n = k.len();
    if t <= k[0][0] {
        return k[0][1];
    }
    if t >= k[n - 1][0] {
        return k[n - 1][1];
    }
    let mut i = 0;
    while t > k[i + 1][0] {
        i += 1;
    }
    let (t0, p0, t1, p1) = (k[i][0], k[i][1], k[i + 1][0], k[i + 1][1]);
    let h = t1 - t0;
    let u = (t - t0) / h;
    let m0 = if i > 0 { (p1 - k[i - 1][1]) / (t1 - k[i - 1][0]) } else { 0.0 };
    let m1 = if i + 2 < n { (k[i + 2][1] - p0) / (k[i + 2][0] - t0) } else { 0.0 };
    let u2 = u * u;
    let u3 = u2 * u;
    (2.0 * u3 - 3.0 * u2 + 1.0) * p0 + (u3 - 2.0 * u2 + u) * h * m0 + (3.0 * u2 - 2.0 * u3) * p1 + (u3 - u2) * h * m1
}
/// character.ts hermite: a cubic segment with its end slopes given
#[allow(clippy::too_many_arguments)]
fn hermite(t: f64, t0: f64, p0: f64, m0: f64, t1: f64, p1: f64, m1: f64) -> f64 {
    let h = t1 - t0;
    let u = clamp((t - t0) / h, 0.0, 1.0);
    let u2 = u * u;
    let u3 = u2 * u;
    (2.0 * u3 - 3.0 * u2 + 1.0) * p0 + (u3 - 2.0 * u2 + u) * h * m0 + (3.0 * u2 - 2.0 * u3) * p1 + (u3 - u2) * h * m1
}
/// quat.nlerp: the shorter way, then made unit
fn nlerp(a: &[f64; 4], b: &[f64; 4], t: f64) -> [f64; 4] {
    let s = if a[0] * b[0] + a[1] * b[1] + a[2] * b[2] + a[3] * b[3] < 0.0 { -1.0 } else { 1.0 };
    let (x, y, z, w) = (lerp(a[0], b[0] * s, t), lerp(a[1], b[1] * s, t), lerp(a[2], b[2] * s, t), lerp(a[3], b[3] * s, t));
    let l = or_one(hypot4(x, y, z, w));
    [x / l, y / l, z / l, w / l]
}
/// `x || 1` of a length: 1 where it is nothing or no number
#[inline(always)]
fn or_one(l: f64) -> f64 {
    if l == 0.0 || l != l {
        1.0
    } else {
        l
    }
}
/// quat.slerp
fn slerp(a: &[f64; 4], b: &[f64; 4], t: f64) -> [f64; 4] {
    let mut d = a[0] * b[0] + a[1] * b[1] + a[2] * b[2] + a[3] * b[3];
    let mut s = 1.0;
    if d < 0.0 {
        d = -d;
        s = -1.0;
    }
    if d > 0.9995 {
        return nlerp(a, b, t);
    }
    let th = acos(d);
    let sn = sin(th);
    let (wa, wb) = (sin((1.0 - t) * th) / sn, (sin(t * th) / sn) * s);
    [a[0] * wa + b[0] * wb, a[1] * wa + b[1] * wb, a[2] * wa + b[2] * wb, a[3] * wa + b[3] * wb]
}
/// quat.fromTo: the shortest turn that takes unit a onto unit b
fn from_to(a: &[f64; 3], b: &[f64; 3]) -> [f64; 4] {
    let d = a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
    if d < -0.9999 {
        let ax = if abs(a[0]) < 0.9 { [1.0, 0.0, 0.0] } else { [0.0, 1.0, 0.0] };
        let c = [a[1] * ax[2] - a[2] * ax[1], a[2] * ax[0] - a[0] * ax[2], a[0] * ax[1] - a[1] * ax[0]];
        let l = or_one(hypot3(c[0], c[1], c[2]));
        return [c[0] / l, c[1] / l, c[2] / l, 0.0];
    }
    let c = [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];
    let w = 1.0 + d;
    let l = or_one(hypot4(c[0], c[1], c[2], w));
    [c[0] / l, c[1] / l, c[2] / l, w / l]
}

/// CharacterAnimator.toesUp — a crouched man's toes kept out of the floor: the foot joint alone turned about the ankle until the toe is back on the floor,
/// laid away from the knee along the shin's own heading unless the foot already points somewhere along the ground. The ankle is not moved.
/// args: 0 w (the crouch shown)
pub fn toes_up(g: usize, at: usize) -> u32 {
    let w = bench().args[0];
    with_pose(g, at, |d, rig, n, fk| {
        if !cast(n, &[SHIN_L, FOOT_L, TOE_L, SHIN_R, FOOT_R, TOE_R]) {
            return 0;
        }
        fk_run(rig, d, fk.rot, fk.pos);
        let floor_y = k(TOE_FLOOR_Y);
        for (shin, foot, toe) in [(role(SHIN_L), role(FOOT_L), role(TOE_L)), (role(SHIN_R), role(FOOT_R), role(TOE_R))] {
            let (a, t) = (fk.at(foot), fk.at(toe));
            if t[1] >= floor_y - 1e-4 {
                continue;
            }
            let dv = sub3(t, a);
            let (l, dy) = (hypot3(dv[0], dv[1], dv[2]), floor_y - a[1]);
            if l < 1e-4 || abs(dy) >= l {
                continue;
            }
            let kn = fk.at(shin);
            let (mut hx, mut hz) = (dv[0] / l, dv[2] / l);
            let (bx, bz) = (a[0] - kn[0], a[2] - kn[2]);
            let bl = hypot2(bx, bz);
            if bl > 1e-4 {
                let own = hypot2(hx, hz);
                let kb = 0.35 * (1.0 - own) * (1.0 - own);
                hx += bx / bl * kb;
                hz += bz / bl * kb;
            }
            let hl = hypot2(hx, hz);
            if hl < 1e-4 {
                continue;
            }
            let r = sqrt(l * l - dy * dy);
            let (to, il) = ([hx / hl * r, dy, hz / hl * r], 1.0 / l);
            let q = slerp(&[0.0, 0.0, 0.0, 1.0], &from_to(&[dv[0] * il, dv[1] * il, dv[2] * il], &[to[0] * il, to[1] * il, to[2] * il]), clamp(w, 0.0, 1.0));
            let par = rig.parent[foot];
            let up = if par >= 0 { fk.rot(par as usize) } else { [0.0, 0.0, 0.0, 1.0] };
            set_model_rot(d, foot, &q_mul(&q, &fk.rot(foot)), &up);
        }
        RAN | RAN_ALL
    })
    .unwrap_or(0)
}
/// v3.lerp
#[inline(always)]
fn lerp3(a: &[f64; 3], b: &[f64; 3], t: f64) -> [f64; 3] {
    [lerp(a[0], b[0], t), lerp(a[1], b[1], t), lerp(a[2], b[2], t)]
}

/// CharacterAnimator.kickBody — the kick's body half: the pelvis's shift, dip, drive, lean back, counter-yaw and hike as overlapping curves, the spine and
/// the head countering so the eyes stay on the door, the swing foot flown along chamber, strike, recoil and plant by IK, the other foot held from the first
/// frame. Built on the scratch pose at `at_c` from the pose at `at`, and laid on that one by w. The left leg's kick is the same kick in a mirror.
/// args: 0 t · 1 w · 2 the left leg kicks (1 or 0) · the idle's feet: 3‥5 the left's spot · 6‥9 its sole's rotation · 10‥12 the right's spot · 13‥16 its
/// sole's rotation; then from 64 its columns, where the kick found the feet: 0 they are set (1 or 0) · 1‥3 the left's spot · 4‥7 its rotation · 8‥10 the
/// right's spot · 11‥14 its rotation (set by the kick's first tick)
pub fn kick_body(g: usize, at: usize, at_c: usize) -> u32 {
    const COLS: usize = 15;
    const ROLES: [usize; 12] = [HIPS, SPINE1, SPINE2, SPINE3, NECK, HEAD, THIGH_L, SHIN_L, FOOT_L, THIGH_R, SHIN_R, FOOT_R];
    let mut a = [0.0f64; L_IN + COLS];
    a.copy_from_slice(&bench().args[..L_IN + COLS]);
    let (t, w, left) = (a[0], a[1], a[2] != 0.0);
    let m = if left { -1.0 } else { 1.0 };
    // when the sole meets the door, and when the foot is back on the floor (seconds into the kick)
    let (kick_impact, kick_plant) = (k(KICK_IMPACT), k(KICK_PLANT));
    let (f_l, f_rl, f_r, f_rr) = ([a[3], a[4], a[5]], [a[6], a[7], a[8], a[9]], [a[10], a[11], a[12]], [a[13], a[14], a[15], a[16]]);
    // where the kick found the feet: taken off the pose on its first tick
    let st = &mut a[L_IN..];
    let Some(Some(n)) = with_pose(g, at, |d, rig, n, fk| {
        if !cast(n, &ROLES) {
            return None;
        }
        fk_run(rig, d, fk.rot, fk.pos);
        if st[0] == 0.0 {
            let (pl, rl, pr, rr) = (fk.at(role(FOOT_L)), fk.rot(role(FOOT_L)), fk.at(role(FOOT_R)), fk.rot(role(FOOT_R)));
            st[0] = 1.0;
            st[1..4].copy_from_slice(&pl);
            st[4..8].copy_from_slice(&rl);
            st[8..11].copy_from_slice(&pr);
            st[11..15].copy_from_slice(&rr);
        }
        Some(n)
    }) else {
        return 0;
    };
    let (k_l, k_rl, k_r, k_rr) = ([st[1], st[2], st[3]], [st[4], st[5], st[6], st[7]], [st[8], st[9], st[10]], [st[11], st[12], st[13], st[14]]);
    // the swing leg and the planted one; their idle spots and sole rotations; where the kick found them
    let (th_s, sh_s, ft_s, th_p, sh_p, ft_p) =
        if left { (role(THIGH_L), role(SHIN_L), role(FOOT_L), role(THIGH_R), role(SHIN_R), role(FOOT_R)) } else { (role(THIGH_R), role(SHIN_R), role(FOOT_R), role(THIGH_L), role(SHIN_L), role(FOOT_L)) };
    let (fs, fsr, fp, fpr) = if left { (f_l, f_rl, f_r, f_rr) } else { (f_r, f_rr, f_l, f_rl) };
    let (ks, kp, kpr) = if left { (k_l, k_r, k_rr) } else { (k_r, k_l, k_rl) };
    // pC.copy(pA)
    if !ask_blend(g, at, at, 1.0, false, at_c, n) {
        return 0;
    }
    let code = with_pose(g, at_c, |d, rig, _n, fk| {
        let (hips, s1, s2, s3, nk, hd) = (role(HIPS), role(SPINE1), role(SPINE2), role(SPINE3), role(NECK), role(HEAD));
        // the pelvis
        let sx = m * curve(t, &[[0.0, 0.0], [0.22, 0.05], [0.5, 0.045], [kick_plant, 0.03], [1.0, 0.0]]);
        let sy = curve(t, &[[0.0, 0.0], [0.2, -0.02], [0.38, -0.01], [0.55, -0.02], [kick_plant, -0.03], [1.0, 0.0]]);
        let sz = curve(t, &[[0.0, 0.0], [0.16, -0.015], [0.38, 0.09], [0.46, 0.04], [0.6, -0.06], [kick_plant, -0.12], [0.86, -0.07], [1.0, 0.0]]);
        let tilt = curve(t, &[[0.0, 0.0], [0.16, 4.0], [0.4, -19.0], [0.56, -9.0], [kick_plant, -2.0], [0.86, 1.5], [1.0, 0.0]]) * DEGR;
        let hyaw = m * curve(t, &[[0.0, 0.0], [0.2, -3.0], [0.4, 9.0], [0.6, 4.0], [0.9, 0.0]]) * DEGR;
        let roll = m * curve(t, &[[0.0, 0.0], [0.25, -5.0], [0.5, -4.0], [kick_plant, -1.0], [0.9, 0.0]]) * DEGR;
        shift(d, rig, hips, &[sx, sy, sz]);
        turn(d, rig, hips, q_mul(&q_yaw(hyaw), &q_mul(&q_roll(roll), &q_pitch(tilt))));
        // the spine and the head
        let lay = curve(t, &[[0.0, 0.0], [0.2, 3.0], [0.39, -8.0], [0.47, -3.5], [0.6, -2.5], [0.85, 0.0]]) * DEGR;
        turn(d, rig, s1, q_mul(&q_yaw(-hyaw * 0.3), &q_mul(&q_roll(-roll * 0.4), &q_pitch(lay))));
        turn(d, rig, s2, q_mul(&q_yaw(-hyaw * 0.3), &q_mul(&q_roll(-roll * 0.4), &q_pitch(lay * 0.7))));
        turn(d, rig, s3, q_yaw(-hyaw * 0.2));
        turn(d, rig, nk, q_pitch(-(tilt + lay * 1.7) * 0.45));
        turn(d, rig, hd, q_pitch(-(tilt + lay * 1.7) * 0.55));
        fk_run(rig, d, fk.rot, fk.pos);
        // the swing foot: a moving origin (its own spot to the idle's) and offsets off it
        let o = lerp3(&ks, &fs, smoothstep(0.0, 0.3, t));
        let down = if t < kick_plant { 1.0 } else { 0.0 };
        let cx = -0.07 * m - fs[0];
        let ox = down * curve(t, &[[0.0, 0.0], [0.12, cx * 0.15], [0.26, cx * 0.6], [0.38, cx], [0.5, cx * 0.8], [0.64, cx * 0.3], [kick_plant, 0.0], [1.0, 0.0]]);
        let (tc, ti) = (0.26, kick_impact);
        let oy = down
            * (if t < tc {
                curve(t, &[[0.0, 0.0], [0.08, 0.03], [0.17, 0.14], [tc, 0.4], [0.32, 0.635]])
            } else if t < ti {
                hermite(t, tc, 0.4, 3.3, ti, 0.76, 0.5)
            } else {
                curve(t, &[[ti, 0.76], [0.43, 0.75], [0.5, 0.58], [0.62, 0.24], [kick_plant, 0.0], [1.0, 0.0]])
            });
        let oz = down
            * (if t < tc {
                curve(t, &[[0.0, 0.0], [0.1, 0.04], [0.17, 0.13], [tc, 0.3], [0.32, 0.745]])
            } else if t < ti {
                hermite(t, tc, 0.3, 4.1, ti, 1.02, 9.0)
            } else {
                curve(t, &[[ti, 1.02], [0.43, 1.0], [0.5, 0.68], [0.62, 0.26], [kick_plant, 0.0], [1.0, 0.0]])
            });
        let lead = if left { f_l[2] - f_r[2] } else { 0.0 };
        let target = [o[0] + ox, max(o[1] + oy, fs[1]), o[2] + oz - lead * bump(t, 0.0, tc, 0.5, kick_plant)];
        let knee_up = [-0.04 * m, 0.5, 1.0];
        two_bone_ik(rig, d, fk.rot, fk.pos, th_s, sh_s, ft_s, &target, Some(&knee_up), bump(t, 0.04, 0.2, 0.62, 0.8), None, false);
        // its ankle, then its sole levelled onto the idle's for the plant
        rotate_local(d, ft_s, &axis_angle(&AX, curve(t, &[[0.0, 0.0], [0.2, 14.0], [0.34, -16.0], [0.42, -18.0], [0.58, 4.0], [0.68, 0.0]]) * DEGR));
        let flat = smoothstep(0.6, kick_plant, t);
        if flat > 0.0 {
            let (sh, q) = (fk.rot(sh_s), ft_s * 10 + 3);
            let own = [get(d, q), get(d, q + 1), get(d, q + 2), get(d, q + 3)];
            set_model_rot(d, ft_s, &nlerp(&q_mul(&sh, &own), &fsr, flat), &sh);
        }
        // the planted foot: held, and set down on the idle's spot in one short low step if the kick found it off it
        let e = smoothstep(0.0, 0.24, t);
        let far = clamp(hypot2(kp[0] - fp[0], kp[2] - fp[2]) / 0.2, 0.0, 1.0);
        let mut pin = lerp3(&kp, &fp, e);
        pin[1] += 0.05 * sin(PI * e) * far;
        fk_run(rig, d, fk.rot, fk.pos);
        two_bone_ik(rig, d, fk.rot, fk.pos, th_p, sh_p, ft_p, &pin, Some(&knee_pole(&fpr)), 0.6 * smoothstep(0.0, 0.15, t), Some(&nlerp(&kpr, &fpr, e)), false);
        RAN | RAN_ALL
    })
    .unwrap_or(0);
    if code == 0 || !ask_blend(g, at, at_c, w, false, at, n) {
        return 0;
    }
    bench().args[L_IN..L_IN + COLS].copy_from_slice(&a[L_IN..]);
    code
}

/// v3.dot, v3.scale; quat.normalize; a node's own rotation in a pose, read and put
#[inline(always)]
fn dot3(a: &[f64; 3], b: &[f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
#[inline(always)]
fn scale3(a: [f64; 3], s: f64) -> [f64; 3] {
    [a[0] * s, a[1] * s, a[2] * s]
}
#[inline(always)]
fn q_unit(q: [f64; 4]) -> [f64; 4] {
    let l = or_one(hypot4(q[0], q[1], q[2], q[3]));
    [q[0] / l, q[1] / l, q[2] / l, q[3] / l]
}
#[inline(always)]
fn own_rot(d: &[S], i: usize) -> [f64; 4] {
    let o = i * 10 + 3;
    [get(d, o), get(d, o + 1), get(d, o + 2), get(d, o + 3)]
}
#[inline(always)]
fn set_own_rot(d: &mut [S], i: usize, q: &[f64; 4]) {
    let o = i * 10 + 3;
    put(d, o, q[0]);
    put(d, o + 1, q[1]);
    put(d, o + 2, q[2]);
    put(d, o + 3, q[3]);
}
/// a foot's place and its sole's rotation (model space)
type Sole = ([f64; 3], [f64; 4]);
/// …and of the hold's own step: where the knee points as well
type Tread = ([f64; 3], [f64; 4], [f64; 3]);

/// A CLIP A LAYER SAMPLES FOR ITSELF (`p.copy(rest); sampleClip(clip, t, looped, p)`): the rest pose copied into a scratch pose of the same character and the
/// clip sampled over it. `clip` is the module's number for it, handed with the layer's own numbers (skeleton.ts clipNo: the host asks a layer only when the
/// module holds every clip it samples); false = no such clip or pose
fn sample_on_rest(g: usize, at_rest: usize, at_out: usize, nodes: usize, clip: f64, t: f64, looped: bool) -> bool {
    clip >= 0.0 && ask_blend(g, at_rest, at_rest, 1.0, false, at_out, nodes) && ask_sample(g, at_out, nodes, clip as usize, t, looped)
}

/// CharacterAnimator.wallBody — the wall hold's body: the spine blended toward the standing idle's and then measured and put (the chest square to the
/// wall but for a counter-turn toward the lead side, the spine's line leant onto the plaster, a crouch's pelvis tilt taken back at the pelvis, the peek's
/// own numbers taking over as it comes in), the pelvis carried back onto the wall and along it onto the lead foot, the head turned along the wall, and the
/// feet planted by IK: parked on the hold's own spots, on the move on the hold's own step or the turned cycle narrowed toward the wall, each sole kept
/// where it stood in the world through a take until its turn to step.
/// `at_c`: the standing idle sampled over the rest pose (its spine, and its pelvis) · `at_b`: the crouched idle sampled the same way, read only under the
/// hold's own step (gw > 1e-4): the host samples both until the clip sampler is the module's.
/// args: 0 w · 1 crouch · the hold's springs: 2 side · 3 side's speed · 4 peek · 5 park · 6 upperW.aim · 7 wallYaw · 8 phase · 9 wallHit ·
/// 10 aimYawOffset · 11 lookYawExtra · 12‥15 chestRestInv · 16‥18 gazeLocal · 19 wallTake · 20 the stance is the wall (1 or 0) · 21 bodyYawIn ·
/// 22‥24 posIn · 25 wallTakeFeet is there (1 or 0) · 26‥28 its left foot's spot (world) · 29 its yaw · 30‥32 its right's · 33 its yaw · the idle's
/// feet: 34‥37 the left sole's rotation · 38 its height · 39‥42 the right's · 43 its height · 44 idleT · 45 the idle clip's number · 46 the crouched
/// idle's (sampled here into the scratch poses `at_c` and `at_b`, over the rest pose `at_rest`); then from 64 its columns: 0 wallGait (the step clock) ·
/// 1 wtkHold · the take's feet (wtk): 2 they are set (1 or 0) · 3‥5 the left's spot (world) · 6 its yaw · 7‥9 the right's · 10 its yaw · 11 which
/// steps first (0 the left) · 12 the take's clock when last seen
pub fn wall_body(g: usize, at: usize, at_c: usize, at_b: usize, at_rest: usize) -> u32 {
    const COLS: usize = 13;
    const ROLES: [usize; 16] = [HIPS, SPINE1, SPINE2, SPINE3, NECK, HEAD, SHOULDER_L, SHOULDER_R, UPPER_ARM_L, UPPER_ARM_R, THIGH_L, SHIN_L, FOOT_L, THIGH_R, SHIN_R, FOOT_R];
    let mut a = [0.0f64; L_IN + COLS];
    a.copy_from_slice(&bench().args[..L_IN + COLS]);
    let (w, cr, s, side_v, pk, park) = (a[0], clamp(a[1], 0.0, 1.0), clamp(a[2], -1.0, 1.0), a[3], clamp(a[4], 0.0, 1.0), clamp(a[5], 0.0, 1.0));
    let (aim_w, ny, phase, wall_hit, aim_yaw, lk) = (a[6], a[7], a[8], a[9], a[10], a[11]);
    let (chest_rest_inv, gaze_local) = ([a[12], a[13], a[14], a[15]], [a[16], a[17], a[18]]);
    let (wall_take, on_wall, yin, p0) = (a[19], a[20] != 0.0, a[21], [a[22], a[23], a[24]]);
    let take_feet = if a[25] != 0.0 { Some(([a[26], a[27], a[28]], a[29], [a[30], a[31], a[32]], a[33])) } else { None };
    let (i_rl, i_ly, i_rr, i_ry) = ([a[34], a[35], a[36], a[37]], a[38], [a[39], a[40], a[41], a[42]], a[43]);
    let e = pk * pk * (3.0 - 2.0 * pk);
    // the wall's normal (into the room) and his right along the wall, model space
    let n = [sin(ny), 0.0, cos(ny)];
    let tr = [-n[2], 0.0, n[0]];
    let root_off = k(WALL_ROOT_OFF);
    // the sidle's own step: its share of the feet, and of the pelvis on the move
    let g_share = if truthy(WS_ON) && truthy(WS_POSE) { lerp(k(WG_SHARE_0), k(WG_SHARE_1), cr) } else { 0.0 };
    let gw = g_share * (1.0 - park) * w;
    // the idles underneath, sampled here: the standing one's spine and pelvis, the crouched one's pelvis
    let (idle_t, idle, crouch_idle) = (a[44], a[45], a[46]);
    let Some(nodes) = with_pose(g, at, |_d, _rig, nodes, _fk| nodes) else {
        return 0;
    };
    if !sample_on_rest(g, at_rest, at_c, nodes, idle, idle_t, true) {
        return 0;
    }
    let Some(Some(under)) = with_pose(g, at_c, |d, _rig, nodes, _fk| {
        if !cast(nodes, &ROLES) {
            return None;
        }
        Some([SPINE1, SPINE2, SPINE3, NECK, HEAD, SHOULDER_L, SHOULDER_R, HIPS].map(|r| own_rot(d, role(r))))
    }) else {
        return 0;
    };
    let hips_low = if gw > 1e-4 {
        if !sample_on_rest(g, at_rest, at_b, nodes, crouch_idle, idle_t, true) {
            return 0;
        }
        let Some(q) = with_pose(g, at_b, |d, _rig, _nodes, _fk| own_rot(d, role(HIPS))) else {
            return 0;
        };
        q
    } else {
        [0.0; 4]
    };
    let st = &mut a[L_IN..];
    let code = with_pose(g, at, |d, rig, _nodes, fk| {
        let (hips, s1, s2, s3, nk, hd) = (role(HIPS), role(SPINE1), role(SPINE2), role(SPINE3), role(NECK), role(HEAD));
        let (foot_l, foot_r) = (role(FOOT_L), role(FOOT_R));
        // what is underneath: the clip's feet
        fk_run(rig, d, fk.rot, fk.pos);
        let (f_l, f_r, r_l, r_r) = (fk.at(foot_l), fk.at(foot_r), fk.rot(foot_l), fk.rot(foot_r));
        // 1. the spine toward the standing idle's, joint by joint
        for (j, (i, share)) in [(s1, 0.9), (s2, 0.9), (s3, 0.9), (nk, 0.85), (hd, 0.85), (role(SHOULDER_L), 0.7), (role(SHOULDER_R), 0.7)].into_iter().enumerate() {
            let q = nlerp(&own_rot(d, i), &under[j], share * w);
            set_own_rot(d, i, &q);
        }
        // 2. the pelvis
        if park > 0.995 || w < 0.01 {
            st[0] = k(WALL_GAIT_REST);
        }
        if gw > 1e-4 {
            let q = nlerp(&own_rot(d, hips), &nlerp(&under[7], &hips_low, cr), gw);
            set_own_rot(d, hips, &q);
        }
        fk_run(rig, d, fk.rot, fk.pos);
        let hp = fk.at(hips);
        let back_off = lerp(lerp(k(WP_BACK_0), k(WP_BACK_1), cr), k(WP_BACK_2), 1.0 - park);
        let out = dot3(&hp, &n) - (-root_off + back_off);
        let drop = lerp(k(WP_DROP_0), k(WP_DROP_1), cr) + 0.006 * (1.0 - park) * (1.0 - g_share) * (1.0 - cos(phase * 4.0 * PI)) + k(WS_DIP) * wall_hit;
        shift(d, rig, hips, &add3(add3(scale3(n, -out * w), [0.0, -drop * w, 0.0]), scale3(tr, s * k(WP_PEEK_SHIFT) * e * w)));
        if gw > 1e-4 {
            let lvl = lerp(k(WG_HIPS_0), k(WG_HIPS_1), cr) - lerp(k(WG_BOB_0), k(WG_BOB_1), cr) * 0.5 * (1.0 + cos((st[0] - 0.05) * 4.0 * PI));
            shift(d, rig, hips, &add3([0.0, (lvl - hp[1]) * gw, 0.0], scale3(tr, -dot3(&hp, &tr) * gw)));
        }
        turn(d, rig, hips, q_mul(&axis_angle(&n, s * 3.0 * DEGR * e * w), &q_yaw(ny * 0.33 * w)));
        // 3. the spine measured and put, twice over
        let chest_yaw_of = |fk: &Fk| {
            let l = q_rotate(&q_mul(&fk.rot(s3), &chest_rest_inv), &AX);
            atan2(-l[2], l[0])
        };
        let shoulder_yaw_of = |fk: &Fk| {
            let (a, b) = (fk.at(role(UPPER_ARM_L)), fk.at(role(UPPER_ARM_R)));
            atan2(b[2] - a[2], a[0] - b[0])
        };
        let ax_n = [n[2], 0.0, -n[0]];
        let mut hip_share = 0.0;
        let twist = aim_yaw * (1.0 - w * (1.0 - aim_w));
        let (turn_deg, lean_deg, gaze_deg) = (lerp(k(WP_CHEST_TURN), k(WP_PEEK_TURN), e), lerp(lerp(k(WP_LEAN_0), k(WP_LEAN_1), cr), k(WP_PEEK_TILT), e), lerp(k(WP_GAZE), k(WP_PEEK_GAZE), e));
        for _pass in 0..2 {
            fk_run(rig, d, fk.rot, fk.pos);
            let dy = wrap_angle(ny - s * turn_deg * DEGR - (twist - aim_yaw * aim_w) - lerp(shoulder_yaw_of(fk), chest_yaw_of(fk), e)) * w;
            turn(d, rig, s1, q_yaw(dy * 0.3));
            turn(d, rig, s2, q_yaw(dy * 0.35));
            turn(d, rig, s3, q_yaw(dy * 0.35));
            fk_run(rig, d, fk.rot, fk.pos);
            let dv = sub3(fk.at(nk), fk.at(s1));
            let (tilt_n, tilt_t) = (atan2(dot3(&dv, &n), dv[1]), atan2(dot3(&dv, &tr), dv[1]));
            let (d_n, d_t) = ((lean_deg * DEGR - tilt_n) * w, (s * k(WP_PEEK_LEAN) * DEGR * e - tilt_t) * w);
            let hs = clamp(d_n, -0.45 * cr - hip_share, 0.45 * cr - hip_share);
            hip_share += hs;
            if abs(hs) > 1e-5 {
                turn(d, rig, hips, axis_angle(&ax_n, hs));
            }
            let rest = d_n - hs;
            turn(d, rig, s1, q_mul(&axis_angle(&n, d_t * 0.5), &axis_angle(&ax_n, rest * 0.6)));
            turn(d, rig, s2, q_mul(&axis_angle(&n, d_t * 0.3), &axis_angle(&ax_n, rest * 0.4)));
            turn(d, rig, s3, axis_angle(&n, d_t * 0.2));
        }
        // 4. the head
        fk_run(rig, d, fk.rot, fk.pos);
        {
            let hw = w * (1.0 - aim_w);
            let gz = q_rotate(&fk.rot(hd), &gaze_local);
            let gy = atan2(gz[0], gz[2]);
            let chest_yaw = chest_yaw_of(fk);
            let mut want = ny - s * gaze_deg * DEGR - twist;
            want = chest_yaw - lk + clamp(wrap_angle(want + lk - chest_yaw), -80.0 * DEGR, 80.0 * DEGR);
            let dyaw = wrap_angle(want - gy) * hw;
            let el = asin(clamp(gz[1], -1.0, 1.0));
            let dpitch = (el + (k(WP_HEAD_PITCH) - k(WS_NOD) * wall_hit) * DEGR) * hw;
            let gh = normalize([gz[0], 0.0, gz[2] + 1e-6]);
            let ax_p = [gh[2], 0.0, -gh[0]];
            let unroll = -s * k(WP_PEEK_LEAN) * DEGR * e * 0.5 * hw;
            let q = |share: f64| q_mul(&axis_angle(&n, unroll * share), &q_mul(&q_yaw(dyaw * share), &axis_angle(&ax_p, dpitch * share)));
            turn(d, rig, nk, q(0.4));
            turn(d, rig, hd, q(0.6));
        }
        // 5. the feet
        let u = (1.0 + s) / 2.0;
        // a parked spot in the wall's frame for one role of one foot, and its toes' yaw
        let spot = |lead: bool, right: bool| -> ([f64; 3], f64) {
            let (stand, low) = if lead {
                ([k(WP_LEAD_0), k(WP_LEAD_1), k(WP_LEAD_2)], [k(WP_LEAD_C_0), k(WP_LEAD_C_1), k(WP_LEAD_C_2)])
            } else {
                ([k(WP_TRAIL_0), k(WP_TRAIL_1), k(WP_TRAIL_2)], [k(WP_TRAIL_C_0), k(WP_TRAIL_C_1), k(WP_TRAIL_C_2)])
            };
            let av = lerp3(&stand, &low, cr);
            let sg = if right == lead { 1.0 } else { -1.0 };
            (add3(scale3(tr, sg * av[0]), scale3(n, av[1] - root_off)), -sg * av[2] * DEGR)
        };
        let mv = (1.0 - park) * w * (1.0 - g_share);
        let wall_gait = st[0];
        // the hold's own step: where this sole is on its track at the step clock's phase, its rotation, and where its knee points
        let gait_foot = |right: bool, r_idle: &[f64; 4], y_idle: f64| -> Tread {
            let (sl, duty) = (lerp(k(WG_STEP_0), k(WG_STEP_1), cr), k(WG_DUTY));
            let half = duty * sl;
            let f = (wall_gait + if right { 0.5 } else { 0.0 }) % 1.0;
            let (along, up);
            if f < duty {
                along = half * (1.0 - 2.0 * f / duty);
                up = 0.0;
            } else {
                let q = (f - duty) / (1.0 - duty);
                let q2 = q * q;
                let q3 = q2 * q;
                let m = -2.0 * sl * (1.0 - duty);
                along = (2.0 * q3 - 3.0 * q2 + 1.0) * -half + (q3 - 2.0 * q2 + q) * m + (-2.0 * q3 + 3.0 * q2) * half + (q3 - q2) * m;
                up = sin(PI * q);
            }
            let heel = lerp(k(WG_HEEL_0), k(WG_HEEL_1), cr) * DEGR * smoothstep(0.1, 1.0, -along * clamp(s * 2.0, -1.0, 1.0) / half);
            let lw = if right { u } else { 1.0 - u };
            let off = lerp(lerp(k(WG_FAR_0), k(WG_FAR_1), cr), lerp(k(WG_NEAR_0), k(WG_NEAR_1), cr), lw);
            let yaw_abs = ny - s * lerp(k(WG_TOE_0), k(WG_TOE_1), cr) * DEGR * (0.5 + 0.5 * lw);
            let h = [sin(yaw_abs), 0.0, cos(yaw_abs)];
            const BALL: f64 = 0.12;
            let al = along + s * lerp(k(WG_AHEAD_0), k(WG_AHEAD_1), cr) + (if right { 1.0 } else { -1.0 }) * lerp(k(WG_APART_0), k(WG_APART_1), cr);
            let p = [
                tr[0] * al + n[0] * (off - root_off) + h[0] * BALL * (1.0 - cos(heel)),
                y_idle + lerp(k(WG_LIFT_0), k(WG_LIFT_1), cr) * up + BALL * sin(heel),
                tr[2] * al + n[2] * (off - root_off) + h[2] * BALL * (1.0 - cos(heel)),
            ];
            let fwd = q_rotate(r_idle, &AY);
            let yaw0 = atan2(fwd[0], fwd[2]);
            let r = q_unit(q_mul(&axis_angle(&[h[2], 0.0, -h[0]], heel), &q_mul(&q_yaw(wrap_angle(yaw_abs - yaw0)), r_idle)));
            let kr = lerp(k(WG_KNEE_ROOM_0), k(WG_KNEE_ROOM_1), cr);
            let dn = normalize([n[0] * kr, 0.0, (1.0 - kr) + n[2] * kr]);
            (p, r, normalize([h[0] * 0.25 + dn[0] * 0.75, 0.5, h[2] * 0.25 + dn[2] * 0.75]))
        };
        let (g_l, g_r) = if g_share > 0.0 { (Some(gait_foot(false, &i_rl, i_ly)), Some(gait_foot(true, &i_rr, i_ry))) } else { (None, None) };
        // (a short take ends before the pose's weight is whole: the soles stay where they stepped to)
        let taking = wall_take >= 0.0 && wall_take < 1.0;
        if taking {
            st[1] = 1.0;
        } else if st[1] != 0.0 && (w >= 0.98 || !on_wall) {
            st[1] = 0.0;
        }
        let wtk_hold = st[1] != 0.0;
        let foot_target = |right: bool, f_clip0: &[f64; 3], r_clip0: &[f64; 4], r_idle: &[f64; 4], y_idle: f64, full: bool| -> Sole {
            let gf = if right { &g_r } else { &g_l };
            let (f_clip, r_clip) = match gf {
                Some(gf) => {
                    let gk = g_share * w;
                    (lerp3(f_clip0, &gf.0, gk), nlerp(r_clip0, &gf.1, gk))
                }
                None => (*f_clip0, *r_clip0),
            };
            let lw = if right { u } else { 1.0 - u };
            let ((lp, ly), (tp, ty)) = (spot(true, right), spot(false, right));
            let mut p = lerp3(&tp, &lp, lw);
            let yaw_rel = lerp(ty, ly, lw);
            let on_toes = cr * (1.0 - lw) * k(WP_TRAIL_HEEL);
            p[1] = lerp(y_idle, f_clip[1], on_toes) + 0.04 * (1.0 - s * s) * (if (side_v > 0.0) == right { 1.0 } else { 0.25 }) + 0.08 * park * (1.0 - park) * (0.4 + 0.6 * lw);
            let base = nlerp(r_idle, &r_clip, on_toes);
            let fwd = q_rotate(&base, &AY);
            let yaw0 = atan2(fwd[0], fwd[2]);
            let r = q_unit(q_mul(&q_yaw(wrap_angle(ny + yaw_rel - yaw0)), &base));
            let share = if full { 1.0 } else { park * (if wtk_hold { 1.0 } else { w }) };
            let mut q = lerp3(&f_clip, &p, share);
            if mv > 1e-4 {
                let off_c = dot3(&q, &n) + root_off;
                q = mad3(q, n, (k(WP_SLIDE_LINE) + (off_c - root_off) * k(WP_SLIDE_NARROW) - off_c) * mv);
            }
            let into = -root_off + 0.11 - dot3(&q, &n);
            (if into > 0.0 { mad3(q, n, into * w) } else { q }, nlerp(&r_clip, &r, share))
        };
        let (mut t_l, mut t_r) = (foot_target(false, &f_l, &r_l, &i_rl, i_ly, false), foot_target(true, &f_r, &r_r, &i_rr, i_ry, false));
        if taking {
            // the take: each sole stays where it stood in the WORLD until its turn, then steps to the hold's spot in a low arc
            let (tk, cy, sy) = (wall_take, cos(yin), sin(yin));
            let to_world = |m: &[f64; 3]| [p0[0] + m[0] * cy + m[2] * sy, p0[1] + m[1], p0[2] - m[0] * sy + m[2] * cy];
            let foot_yaw = |r: &[f64; 4]| {
                let f = q_rotate(r, &AY);
                atan2(f[0], f[2])
            };
            let (s_l, s_r) = (foot_target(false, &f_l, &r_l, &i_rl, i_ly, true), foot_target(true, &f_r, &r_r, &i_rr, i_ry, true));
            if st[2] == 0.0 || tk < st[12] {
                let ((lp, lyaw), (rp, ryaw)) = match take_feet {
                    Some(t) => ((t.0, t.1), (t.2, t.3)),
                    None => ((to_world(&f_l), yin + foot_yaw(&r_l)), (to_world(&f_r), yin + foot_yaw(&r_r))),
                };
                st[2] = 1.0;
                st[3..6].copy_from_slice(&lp);
                st[6] = lyaw;
                st[7..10].copy_from_slice(&rp);
                st[10] = ryaw;
                st[11] = if dist_xz(&f_l, &s_l.0) >= dist_xz(&f_r, &s_r.0) { 0.0 } else { 1.0 };
            }
            st[12] = tk;
            let step = |from: [f64; 3], from_yaw: f64, sp: &Sole, r_clip: &[f64; 4], t0: f64, t1: f64| -> Sole {
                let q = clamp((tk - t0) / (t1 - t0), 0.0, 1.0);
                let ease = q * q * (3.0 - 2.0 * q);
                let (wx, wz) = (from[0] - p0[0], from[2] - p0[2]);
                let mut m = [wx * cy - wz * sy, from[1] - p0[1], wx * sy + wz * cy];
                let (dx, dz) = (m[0] - sp.0[0], m[2] - sp.0[2]);
                let (dl, mx) = (hypot2(dx, dz), k(WS_STEP_TRAIL));
                if t0 > 0.0 && dl > mx {
                    m[0] = sp.0[0] + dx * mx / dl;
                    m[2] = sp.0[2] + dz * mx / dl;
                }
                let mut p = lerp3(&m, &sp.0, ease);
                p[1] += k(WS_STEP_LIFT) * sin(PI * q);
                (p, nlerp(&q_unit(q_mul(&q_yaw(wrap_angle(from_yaw - yin - foot_yaw(r_clip))), r_clip)), &sp.1, ease))
            };
            // (the further foot goes at once, the other waits, then closes as the back lands)
            let (early, late) = ((0.0, 0.58), (0.42, 1.0));
            let (tl, trr) = if st[11] == 0.0 { (early, late) } else { (late, early) };
            t_l = step([st[3], st[4], st[5]], st[6], &s_l, &r_l, tl.0, tl.1);
            t_r = step([st[7], st[8], st[9]], st[10], &s_r, &r_r, trr.0, trr.1);
        } else if st[2] != 0.0 {
            st[2..13].fill(0.0);
        }
        fk_run(rig, d, fk.rot, fk.pos);
        let pole_of = |t: &Sole, gf: &Option<Tread>| match gf {
            Some(gf) if gw > 1e-4 => normalize(lerp3(&knee_pole(&t.1), &gf.2, gw)),
            _ => knee_pole(&t.1),
        };
        two_bone_ik(rig, d, fk.rot, fk.pos, role(THIGH_L), role(SHIN_L), foot_l, &t_l.0, Some(&pole_of(&t_l, &g_l)), 0.6, Some(&t_l.1), false);
        two_bone_ik(rig, d, fk.rot, fk.pos, role(THIGH_R), role(SHIN_R), foot_r, &t_r.0, Some(&pole_of(&t_r, &g_r)), 0.6, Some(&t_r.1), true);
        RAN | RAN_ALL
    })
    .unwrap_or(0);
    if code != 0 {
        bench().args[L_IN..L_IN + COLS].copy_from_slice(&a[L_IN..]);
    }
    code
}

/// WATCH's columns (the state the slip and the look share): 0 the slip's weight (sw) · 1, 2 the man's bearing, level, in the model (sx, sz) · 3 the arm
/// that has him (0 the left) · 4 the lean's share (sl) · 5 the look's weight (w) · 6 its yaw · 7 its pitch
const WT_COLS: usize = 8;
const WT_SW: usize = 0;
const WT_SX: usize = 1;
const WT_SZ: usize = 2;
const WT_SIDE: usize = 3;
const WT_SL: usize = 4;
const WT_W: usize = 5;
const WT_YAW: usize = 6;
const WT_PITCH: usize = 7;

/// CharacterAnimator.watchSlip — he gives a man room as he passes, the body's share: by how near the man is the pelvis comes lower and off his side over
/// feet left where they were, and the trunk tips away from him with the head brought level back out of it.
/// args: 0 dt · 1 free · 2 a man is there (1 or 0) · 3‥5 where, in the model, through the twist's inverse · 6 slipK · 7 crouch · 8 slipLean; then from 64
/// WATCH's columns
pub fn watch_slip(g: usize, at: usize) -> u32 {
    let mut a = [0.0f64; L_IN + WT_COLS];
    a.copy_from_slice(&bench().args[..L_IN + WT_COLS]);
    let (dt, free, there, m, slip_k, crouch, slip_lean) = (a[0], a[1], a[2] != 0.0, [a[3], a[4], a[5]], a[6], a[7], a[8]);
    let st = &mut a[L_IN..];
    let code = with_pose(g, at, |d, rig, n, fk| {
        if !cast(n, &LEG_ROLES) || !cast(n, &[SPINE1, SPINE2, SPINE3, NECK, HEAD]) {
            return 0;
        }
        let s_want = if there { clamp(slip_k, 0.0, 1.0) * free * clamp(crouch, 0.0, 1.0) } else { 0.0 };
        st[WT_SW] = if abs(s_want - st[WT_SW]) < 0.003 { s_want } else { damp(st[WT_SW], s_want, k(WATCH_SLIP_RATE), dt) };
        if there {
            let l = hypot2(m[0], m[2]);
            if l > 1e-4 {
                st[WT_SX] = m[0] / l;
                st[WT_SZ] = m[2] / l;
            }
            if if st[WT_SIDE] == 0.0 { st[WT_SX] < -0.2 } else { st[WT_SX] > 0.2 } {
                st[WT_SIDE] = if st[WT_SX] > 0.0 { 0.0 } else { 1.0 };
            }
            let lw = clamp(slip_lean, 0.0, 1.0);
            st[WT_SL] = if abs(lw - st[WT_SL]) < 0.003 { lw } else { damp(st[WT_SL], lw, k(WATCH_SLIP_RATE), dt) };
        }
        let mut ran = RAN;
        let (sw, sx, sl) = (st[WT_SW], st[WT_SX], st[WT_SL]);
        if sw > 0.003 {
            let kk = sw * sw * (3.0 - 2.0 * sw);
            let (kl, kd) = (kk * sl, kk * (1.0 + k(WATCH_SQUEEZE) * (1.0 - sl)));
            let (hips, foot_l, foot_r) = (role(HIPS), role(FOOT_L), role(FOOT_R));
            fk.part(rig, d, list(FK_LEGS));
            let (f_l, f_r, r_l, r_r) = (fk.at(foot_l), fk.at(foot_r), fk.rot(foot_l), fk.rot(foot_r));
            shift(d, rig, hips, &[-sx * k(WATCH_AWAY) * kl, -k(WATCH_DROP) * kd, 0.0]);
            fk.part(rig, d, list(FK_LEGS));
            two_bone_ik(rig, d, fk.rot, fk.pos, role(THIGH_L), role(SHIN_L), foot_l, &f_l, Some(&knee_pole(&r_l)), 0.0, Some(&r_l), false);
            two_bone_ik(rig, d, fk.rot, fk.pos, role(THIGH_R), role(SHIN_R), foot_r, &f_r, Some(&knee_pole(&r_r)), 0.0, Some(&r_r), true);
            ran |= RAN_ALL;
            let roll = sx * k(WATCH_LEAN) * DEGR * kl;
            turn(d, rig, role(SPINE1), q_roll(roll * 0.4));
            turn(d, rig, role(SPINE2), q_roll(roll * 0.35));
            turn(d, rig, role(SPINE3), q_roll(roll * 0.25));
            turn(d, rig, role(NECK), q_roll(-roll * 0.4));
            turn(d, rig, role(HEAD), q_roll(-roll * 0.6));
        }
        ran
    })
    .unwrap_or(0);
    if code != 0 {
        bench().args[L_IN..L_IN + WT_COLS].copy_from_slice(&a[L_IN..]);
    }
    code
}

/// CharacterAnimator.watchLook — he watches a man: the head goes to the point (the chest a little, the neck, the head), within what a neck does of the
/// chest's line; the angle follows the point, and the whole comes and goes by its weight.
/// args: 0 dt · 1 free · 2 a man is there (1 or 0) · 3‥5 where, in the model, through the twist's inverse · 6 watchK · 7‥9 gazeLocal; then from 64
/// WATCH's columns
pub fn watch_look(g: usize, at: usize) -> u32 {
    let mut a = [0.0f64; L_IN + WT_COLS];
    a.copy_from_slice(&bench().args[..L_IN + WT_COLS]);
    let (dt, free, there, m, watch_k, gaze_local) = (a[0], a[1], a[2] != 0.0, [a[3], a[4], a[5]], a[6], [a[7], a[8], a[9]]);
    let st = &mut a[L_IN..];
    let code = with_pose(g, at, |d, rig, n, fk| {
        if !cast(n, &[SPINE3, NECK, HEAD]) {
            return 0;
        }
        let (s3, nk, hd) = (role(SPINE3), role(NECK), role(HEAD));
        let w_want = if there { clamp(watch_k, 0.0, 1.0) * free } else { 0.0 };
        st[WT_W] = if abs(w_want - st[WT_W]) < 0.003 { w_want } else { damp(st[WT_W], w_want, k(WATCH_W_RATE), dt) };
        let mut ran = RAN;
        if there && w_want > 0.0 {
            fk_run(rig, d, fk.rot, fk.pos);
            ran |= RAN_ALL;
            let hr = fk.rot(hd);
            let gaze = q_rotate(&hr, &gaze_local);
            let eyes = mad3(mad3(fk.at(hd), q_rotate(&hr, &AY), 0.1), gaze, 0.08);
            let want = normalize(sub3(m, eyes));
            let yaw_e = clamp(wrap_angle(atan2(want[0], want[2]) - atan2(gaze[0], gaze[2])), -k(WATCH_YAW) * DEGR, k(WATCH_YAW) * DEGR);
            let pitch_e = clamp(asin(clamp(gaze[1], -1.0, 1.0)) - asin(clamp(want[1], -1.0, 1.0)), -k(WATCH_UP) * DEGR, k(WATCH_DOWN) * DEGR);
            if st[WT_W] < 0.01 {
                st[WT_YAW] = yaw_e;
                st[WT_PITCH] = pitch_e;
            } else {
                st[WT_YAW] = damp(st[WT_YAW], yaw_e, k(WATCH_RATE), dt);
                st[WT_PITCH] = damp(st[WT_PITCH], pitch_e, k(WATCH_RATE), dt);
            }
        } else if st[WT_W] <= 0.0 {
            st[WT_YAW] = 0.0;
            st[WT_PITCH] = 0.0;
        }
        if st[WT_W] > 0.003 {
            let (yw, pw) = (st[WT_YAW] * st[WT_W], st[WT_PITCH] * st[WT_W]);
            let (sh0, sh1, sh2) = (k(WATCH_SHARE_0), k(WATCH_SHARE_1), k(WATCH_SHARE_2));
            let rest = sh1 + sh2;
            turn(d, rig, s3, q_yaw(yw * sh0));
            turn(d, rig, nk, q_mul(&q_yaw(yw * sh1), &q_pitch(pw * sh1 / rest)));
            turn(d, rig, hd, q_mul(&q_yaw(yw * sh2), &q_pitch(pw * sh2 / rest)));
        }
        ran
    })
    .unwrap_or(0);
    if code != 0 {
        bench().args[L_IN..L_IN + WT_COLS].copy_from_slice(&a[L_IN..]);
    }
    code
}

/// a curve read where the TypeScript keeps it: a key is two knobs (its time, its value)
fn curve_k<const N: usize>(t: f64, at: &[[usize; 2]; N]) -> f64 {
    let mut keys = [[0.0f64; 2]; N];
    for (key, ix) in keys.iter_mut().zip(at) {
        *key = [k(ix[0]), k(ix[1])];
    }
    curve(t, &keys)
}
/// the flinch's envelope over its own length, the hit's over seconds, the start's over its own length (character.ts MV_FLINCH_ENV, MV_HIT_ENV, MV_STARTLE.env)
const FLINCH_KEYS: [[usize; 2]; 5] = [[FLINCH_ENV_0_0, FLINCH_ENV_0_1], [FLINCH_ENV_1_0, FLINCH_ENV_1_1], [FLINCH_ENV_2_0, FLINCH_ENV_2_1], [FLINCH_ENV_3_0, FLINCH_ENV_3_1], [FLINCH_ENV_4_0, FLINCH_ENV_4_1]];
const HIT_KEYS: [[usize; 2]; 6] = [[HIT_ENV_0_0, HIT_ENV_0_1], [HIT_ENV_1_0, HIT_ENV_1_1], [HIT_ENV_2_0, HIT_ENV_2_1], [HIT_ENV_3_0, HIT_ENV_3_1], [HIT_ENV_4_0, HIT_ENV_4_1], [HIT_ENV_5_0, HIT_ENV_5_1]];
const START_KEYS: [[usize; 2]; 5] = [[START_ENV_0_0, START_ENV_0_1], [START_ENV_1_0, START_ENV_1_1], [START_ENV_2_0, START_ENV_2_1], [START_ENV_3_0, START_ENV_3_1], [START_ENV_4_0, START_ENV_4_1]];
/// the columns of mv_post_body (after its own numbers): the clocks and springs of the reactions, then what it hands the arms that follow it
const MP_COLS: usize = 27;
const MP_FL_T: usize = 0;
const MP_FL_K: usize = 1;
const MP_ST_T: usize = 2;
const MP_HT_T: usize = 3;
const MP_DZ: usize = 4;
const MP_DZ_SD: usize = 6;
const MP_DZ_T: usize = 7;
const MP_CF: usize = 8;
const MP_CF_T: usize = 10;
const MP_HU: usize = 11;
const MP_HU_T: usize = 13;
const MP_SK: usize = 14;
const MP_STW: usize = 16;
const MP_STN: usize = 17;
const MP_DZW: usize = 18;
const MP_GUN: usize = 19;
const MP_GUN_P: usize = 20;
const MP_GUN_R: usize = 23;

/// CharacterAnimator.mvPostBody — after the stances and the arms, the body's share of the reactions: the flinch, the start, the hit, the blinded hunch, the
/// clutch's fold, a hurt man's hunch and heavy step, a shaken man's ducked head and the toss's wind-up, summed into one turn of the trunk, one of the head and
/// one move of the pelvis over feet held where they are. The arms that follow (the start's, the blinded hand, the held gun, the toss) are the host's.
/// args: 0 dt · 1 gate · 2 trained · 3 the long gun's weight (lg.w.x) · 4 flKTo · 5‥7 the flinch's direction · 8‥10 the start's · 11‥13 the hit's · 14 the
/// hit's side · 15 blinded (1 or 0) · 16 by how much · 17‥19 where from (model, flat) · 20 clutch · 21 the fold is on (1 or 0) · 22 foldToo (1 or 0) ·
/// 23 hurt · 24 the side he favours · 25 upperW.aim · 26 speed · 27 phase · 28 shaken · 29 the toss's clock · 30 its release; then from 64 its columns:
/// 0 flT · 1 flK · 2 stT · 3 htT · 4 the dazzle's spring (x, v) · 6 dzSd · 7 dzT · 8 the fold's spring (x, v) · 10 cfT · 11 hurt's spring (x, v) · 13 huT ·
/// 14 shaken's spring (x, v) · and what the arms are handed: 16 the start's weight · 17 its side · 18 the blinded weight · 19 a gun held on its line
/// (1 or 0) · 20‥22 the trigger hand's place · 23‥26 its rotation
pub fn mv_post_body(g: usize, at: usize) -> u32 {
    let mut a = [0.0f64; L_IN + MP_COLS];
    a.copy_from_slice(&bench().args[..L_IN + MP_COLS]);
    let (dt, gate, trained, lg_w, fl_k_to) = (a[0], a[1], a[2], a[3], a[4]);
    let (fl_d, st_d, ht_d, ht_side) = ([a[5], a[6], a[7]], [a[8], a[9], a[10]], [a[11], a[12], a[13]], a[14]);
    let (blind, blind_k, dz_d) = (a[15] != 0.0, a[16], [a[17], a[18], a[19]]);
    let (clutch, fold_on, fold_too, hurt, hurt_side, aim, speed, phase, shaken, ts, rel) = (a[20], a[21] != 0.0, a[22] != 0.0, a[23], a[24], a[25], a[26], a[27], a[28], a[29], a[30]);
    let st = &mut a[L_IN..];
    let code = with_pose(g, at, |d, rig, n, fk| {
        if !cast(n, &LEG_ROLES) || !cast(n, &[SPINE1, SPINE2, SPINE3, NECK, HEAD, HAND_R]) {
            return 0;
        }
        let t = clamp(trained, 0.0, 1.0);
        let kk = |p0: usize, p1: usize| lerp(k(p0), k(p1), t);
        let (mut pitch, mut yaw, mut roll, mut hp, mut hy, mut drop, mut sx, mut sz) = (0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64);
        if st[MP_FL_T] >= 0.0 {
            st[MP_FL_K] += (fl_k_to - st[MP_FL_K]) * min(1.0, dt * 12.0);
            let dur = kk(FLINCH_DUR_0, FLINCH_DUR_1);
            let e = curve_k(st[MP_FL_T] / dur, &FLINCH_KEYS) * kk(FLINCH_AMP_0, FLINCH_AMP_1) * st[MP_FL_K] * gate;
            let (sd, lat) = (if fl_d[0] >= 0.0 { 1.0 } else { -1.0 }, min(1.0, abs(fl_d[0]) * 1.6));
            pitch += k(FLINCH_PITCH) * e * (0.65 + 0.35 * max(0.0, fl_d[2]));
            yaw -= sd * k(FLINCH_AWAY) * e * lat;
            roll += sd * 3.0 * e * lat;
            hp += k(FLINCH_HEAD) * e;
            hy -= sd * k(FLINCH_AWAY) * e * lat;
            drop += k(FLINCH_DROP) * e;
            sx -= fl_d[0] * k(FLINCH_SIDE) * e;
            sz -= fl_d[2] * k(FLINCH_SIDE) * e;
            st[MP_FL_T] += dt;
            if st[MP_FL_T] > dur {
                st[MP_FL_T] = -1.0;
            }
        }
        // the start: reared back off a thing in front, turned toward the side it is on; a thing behind him he comes round to
        let (mut stw, mut stn) = (0.0f64, 1.0f64);
        if st[MP_ST_T] >= 0.0 {
            let (dur, lw, dv) = (kk(START_DUR_0, START_DUR_1), clamp(lg_w, 0.0, 1.0), st_d);
            let amp = kk(START_AMP_0, START_AMP_1);
            let e = curve_k(st[MP_ST_T] / dur, &START_KEYS) * lerp(amp, max(amp, k(START_GUN_AMP)), lw) * gate;
            let (bh, sd) = (clamp(-dv[2], 0.0, 1.0), if dv[0] >= 0.0 { 1.0 } else { -1.0 });
            let lat = if bh > 0.0 { 1.0 } else { min(1.0, abs(dv[0]) * 1.6) };
            pitch -= k(START_REAR) * (1.0 + k(START_GUN_REAR) * lw) * e * dv[2] * (if dv[2] < 0.0 { k(START_BEHIND_PITCH) } else { 1.0 });
            yaw += sd * k(START_TURN) * (1.0 + k(START_BEHIND_TURN) * bh) * e * lat;
            roll -= sd * k(START_LEAN) * e * lat;
            hp -= k(START_HEAD) * e * max(0.3, dv[2]);
            hy += sd * k(START_HEAD_TURN) * (1.0 + k(START_BEHIND_HEAD) * bh) * e * lat;
            drop += k(START_DROP) * e;
            sx -= dv[0] * k(START_BACK) * (1.0 + k(START_GUN_BACK) * lw) * e;
            sz -= dv[2] * k(START_BACK) * (1.0 + k(START_GUN_BACK) * lw) * e;
            stw = e;
            stn = sd;
            st[MP_ST_T] += dt;
            if st[MP_ST_T] > dur {
                st[MP_ST_T] = -1.0;
            }
        }
        if st[MP_HT_T] >= 0.0 {
            let (dv, e) = (ht_d, curve_k(st[MP_HT_T], &HIT_KEYS) * gate);
            let fz = if dv[2] <= 0.0 { 1.0 } else { -1.0 };
            let (tp, ty) = (dv[2] * k(HIT_PITCH) * e, ht_side * fz * k(HIT_YAW) * e * (0.4 + 0.6 * abs(dv[2])));
            pitch += tp;
            yaw += ty;
            roll += (ht_side * fz * k(HIT_ROLL) * abs(dv[2]) - dv[0] * k(HIT_ROLL) * 2.0) * e;
            hp -= tp * 0.5;
            hy -= ty * 0.5;
            drop += k(HIT_DROP) * abs(e);
            sx += dv[0] * k(HIT_SHOVE) * e;
            sz += dv[2] * k(HIT_SHOVE) * e;
            st[MP_HT_T] += dt;
            if st[MP_HT_T] > k(HIT_DUR) {
                st[MP_HT_T] = -1.0;
            }
        }
        // blinded: the side he turns away to is chosen once, as it goes off
        let dz_t = if blind && mv_on(MOVES_DAZZLE) { clamp(blind_k, 0.0, 1.0) } else { 0.0 };
        let rate = if dz_t > st[MP_DZ] { k(DAZZLE_RATE_IN) } else { k(DAZZLE_RATE_OUT) };
        spring(st, MP_DZ, dz_t, rate, dt);
        if dz_t == 0.0 && st[MP_DZ] < 0.02 {
            st[MP_DZ] = 0.0;
            st[MP_DZ + 1] = 0.0;
        }
        let dzw = clamp(st[MP_DZ], 0.0, 1.0) * kk(DAZZLE_AMP_0, DAZZLE_AMP_1) * gate;
        if st[MP_DZ] <= 0.0 {
            st[MP_DZ_SD] = 0.0;
        } else if st[MP_DZ_SD] == 0.0 {
            st[MP_DZ_SD] = if dz_d[0] >= 0.0 { 1.0 } else { -1.0 };
        }
        if dzw > 0.003 {
            st[MP_DZ_T] += dt;
            let sd = if falsy(st[MP_DZ_SD]) { 1.0 } else { st[MP_DZ_SD] };
            pitch += k(DAZZLE_HUNCH) * dzw;
            yaw -= sd * k(DAZZLE_AWAY) * dzw;
            roll += sd * k(DAZZLE_ROLL) * dzw;
            hp += k(DAZZLE_HEAD) * dzw;
            hy -= sd * k(DAZZLE_HEAD_AWAY) * dzw;
            drop += k(DAZZLE_DROP) * dzw;
            sx += (-dz_d[0] * k(DAZZLE_BACK) + k(DAZZLE_SWAY_X) * sin(st[MP_DZ_T] * 3.1)) * dzw;
            sz += (-dz_d[2] * k(DAZZLE_BACK) + k(DAZZLE_SWAY_Z) * sin(st[MP_DZ_T] * 2.3 + 1.0)) * dzw;
        } else {
            st[MP_DZ_T] = 0.0;
        }
        // the clutch's fold: under the blinded hunch it gives way by the hunch's own weight
        let want = if clutch > 0.0 && fold_on { clamp(clutch, 0.0, 1.0) * (1.0 - clamp(dzw, 0.0, 1.0)) } else { 0.0 };
        if want > 0.0 || st[MP_CF] != 0.0 {
            spring(st, MP_CF, want, k(FOLD_RATE), dt);
            if want == 0.0 && st[MP_CF] < 0.01 {
                st[MP_CF] = 0.0;
                st[MP_CF + 1] = 0.0;
            }
        }
        let cfw = clamp(st[MP_CF], 0.0, 1.0) * gate;
        if cfw > 0.003 {
            st[MP_CF_T] += dt;
            let s = sin(st[MP_CF_T] * 2.0 * PI * k(FOLD_HZ));
            pitch += (if fold_too { k(FOLD_PITCH_TOO) } else { k(FOLD_PITCH) }) * cfw;
            roll += k(FOLD_SWAY) * s * cfw;
            hp += k(FOLD_HEAD) * cfw;
            drop += k(FOLD_DROP) * cfw;
            sx += 0.012 * s * cfw;
        } else {
            st[MP_CF_T] = 0.0;
        }
        // a man's condition: hurt, then shaken; both give way under the fold and the blinded hunch
        let (want, under) = (clamp(hurt, 0.0, 1.0), (1.0 - clamp(dzw, 0.0, 1.0)) * (1.0 - cfw));
        if want > 0.0 || st[MP_HU] != 0.0 {
            spring(st, MP_HU, want, k(HURT_RATE), dt);
            if want == 0.0 && st[MP_HU] < 0.01 {
                st[MP_HU] = 0.0;
                st[MP_HU + 1] = 0.0;
            }
        }
        let hu = clamp(st[MP_HU], 0.0, 1.0) * gate * under;
        if hu > 0.003 {
            let (sd, aim_w) = (hurt_side, clamp(aim, 0.0, 1.0));
            let (brace, mw) = (1.0 - k(HURT_BRACE) * aim_w, smoothstep(0.15, 0.7, abs(speed)));
            let on_it = max(0.0, -sd * sin(2.0 * PI * phase));
            let step = on_it * on_it * mw;
            st[MP_HU_T] += dt;
            let br = sin(st[MP_HU_T] * 2.0 * PI * k(HURT_HZ)) * (1.0 - mw);
            pitch += (k(HURT_PITCH) + k(HURT_STEP_PITCH) * step) * brace * hu;
            roll -= sd * (k(HURT_LEAN) + k(HURT_STEP_ROLL) * step + k(HURT_SWAY) * br) * hu;
            yaw += sd * k(HURT_STEP_YAW) * step * brace * hu;
            hp += (k(HURT_HEAD) * (1.0 - k(HURT_HEAD_BRACE) * aim_w) + k(HURT_STEP_HEAD) * step) * hu;
            drop += (k(HURT_DROP) + k(HURT_STEP_DROP) * step) * hu;
            sx += sd * k(HURT_STEP_SIDE) * step * hu;
        } else {
            st[MP_HU_T] = 0.0;
        }
        let want = clamp(shaken, 0.0, 1.0);
        if want > 0.0 || st[MP_SK] != 0.0 {
            let rate = if want > st[MP_SK] { k(SHAKEN_RATE_IN) } else { k(SHAKEN_RATE_OUT) };
            spring(st, MP_SK, want, rate, dt);
            if want == 0.0 && st[MP_SK] < 0.01 {
                st[MP_SK] = 0.0;
                st[MP_SK + 1] = 0.0;
            }
        }
        let sk = clamp(st[MP_SK], 0.0, 1.0) * gate * under;
        if sk > 0.003 {
            pitch += k(SHAKEN_PITCH) * (1.0 - k(SHAKEN_BRACE) * clamp(aim, 0.0, 1.0)) * sk;
            hp += k(SHAKEN_HEAD) * sk;
            drop += k(SHAKEN_DROP) * sk;
        }
        // the toss's body English: the left shoulder back with the hand, then through with it; a dip of the knees under the swing
        if ts >= 0.0 {
            let td = rel + k(TOSS_AFTER);
            let wind = curve(ts, &[[0.0, 0.0], [rel * 0.8, 1.0], [rel + 0.03, 0.2], [rel + 0.2, -1.0], [rel + 0.4, -0.6], [td, 0.0]]) * gate;
            yaw += 9.0 * wind;
            pitch += 5.0 * max(0.0, -wind) + 2.0 * max(0.0, wind);
            drop += 0.02 * abs(wind);
        }
        let mut ran = RAN;
        if drop > 2e-4 || abs(sx) + abs(sz) > 2e-4 {
            fk_run(rig, d, fk.rot, fk.pos);
            mv_hips(d, rig, fk, &[sx, -drop, sz]);
            ran |= RAN_ALL;
        }
        // a blinded man's gun stays on the line it had: where the trigger hand is before the trunk turns away
        st[MP_GUN] = 0.0;
        if dzw > 0.003 && lg_w > 0.02 {
            fk_run(rig, d, fk.rot, fk.pos);
            ran |= RAN_ALL;
            let hand = role(HAND_R);
            st[MP_GUN] = 1.0;
            st[MP_GUN_P..MP_GUN_P + 3].copy_from_slice(&fk.at(hand));
            st[MP_GUN_R..MP_GUN_R + 4].copy_from_slice(&fk.rot(hand));
        }
        if abs(pitch) + abs(yaw) + abs(roll) > 1e-3 {
            let q = |ky: f64, kp: f64| q_mul(&q_yaw(yaw * DEGR * ky), &q_mul(&q_roll(roll * DEGR * ky), &q_pitch(pitch * DEGR * kp)));
            turn(d, rig, role(SPINE1), q(0.3, 0.4));
            turn(d, rig, role(SPINE2), q(0.35, 0.35));
            turn(d, rig, role(SPINE3), q(0.35, 0.25));
        }
        if abs(hp) + abs(hy) > 1e-3 {
            let q = |kq: f64| q_mul(&q_yaw(hy * DEGR * kq), &q_pitch(hp * DEGR * kq));
            turn(d, rig, role(NECK), q(0.4));
            turn(d, rig, role(HEAD), q(0.6));
        }
        st[MP_STW] = stw;
        st[MP_STN] = stn;
        st[MP_DZW] = dzw;
        ran
    })
    .unwrap_or(0);
    if code != 0 {
        bench().args[L_IN..L_IN + MP_COLS].copy_from_slice(&a[L_IN..]);
    }
    code
}

/// character.ts turnStep: a turn's speed toward what is left of it (`err`), never faster than it can stop in and never carried past the point. Answers how
/// far it goes this step and the speed it has after.
fn turn_step(w: f64, err: f64, w_max: f64, acc: f64, dt: f64) -> (f64, f64) {
    let a = abs(err);
    if a < 1e-5 {
        return (err, 0.0);
    }
    let s = if err < 0.0 { -1.0 } else { 1.0 };
    let wt = s * min(w_max, sqrt(2.0 * acc * a));
    let mut nw = w + clamp(wt - w, -acc * dt, acc * dt);
    if s * nw > s * wt {
        nw = wt;
    }
    let d = nw * dt;
    if s * d >= a {
        (err, s * min(w_max, a / dt))
    } else {
        (d, nw)
    }
}
/// Character.turnWeigh — TURN: bodyYaw and aimYaw as his AI left them are where he WANTS to face; he comes round to them as a mass (his feet the short way
/// round, the gun about his waist through his front), and the two are rewritten with where he has got to. A RULE OF THE CHARACTER'S, NOT OF HIS POSE: it reads
/// no pose and is asked with none, through the layers' bench.
/// args: 0 dt · 1‥2 where he stands (x, z) · 3 the handling of what he carries (turn.h) · 4 how drilled he is (turn.k); then from 64 its columns: 0 a turn
/// stands (1 or 0) · 1 body · 2 aim · 3 the body's speed · 4 the gun's · 5 the body wished · 6 the aim wished · 7‥8 where he stood when it was weighed ·
/// 9 the neck's lead · 10 bodyYaw · 11 aimYaw
pub fn turn_weigh() -> u32 {
    const COLS: usize = 12;
    const THERE: usize = 0;
    const BODY: usize = 1;
    const AIM: usize = 2;
    const WB: usize = 3;
    const WA: usize = 4;
    const IB: usize = 5;
    const IA: usize = 6;
    const X: usize = 7;
    const Z: usize = 8;
    const LEAD: usize = 9;
    const BODY_YAW: usize = 10;
    const AIM_YAW: usize = 11;
    let mut a = [0.0f64; L_IN + COLS];
    a.copy_from_slice(&bench().args[..L_IN + COLS]);
    let (dt, px, pz, handling, drilled) = (a[0], a[1], a[2], a[3], a[4]);
    let st = &mut a[L_IN..];
    let (body_yaw, aim_yaw) = (st[BODY_YAW], st[AIM_YAW]);
    if st[THERE] == 0.0 || hypot2(px - st[X], pz - st[Z]) > k(TURN_JUMP) || !finite(body_yaw + aim_yaw) {
        st[THERE] = 1.0;
        st[BODY] = body_yaw;
        st[AIM] = aim_yaw;
        st[WB] = 0.0;
        st[WA] = 0.0;
        st[IB] = body_yaw;
        st[IA] = aim_yaw;
        st[X] = px;
        st[Z] = pz;
    } else {
        if body_yaw != st[BODY] {
            st[IB] = body_yaw;
        }
        if aim_yaw != st[AIM] {
            st[IA] = aim_yaw;
        }
        let (h, kk) = (clamp(handling, 0.2, 1.5), lerp(k(TURN_GREEN), 1.0, clamp(drilled, 0.0, 1.0)));
        let hb = lerp(1.0, h, k(TURN_BODY_SHARE));
        let sb = turn_step(st[WB], wrap_angle(st[IB] - st[BODY]), k(TURN_BODY) * hb, k(TURN_BODY_ACC) * hb * kk, dt);
        st[BODY] = wrap_angle(st[BODY] + sb.0);
        st[WB] = sb.1;
        let r0 = clamp(wrap_angle(st[AIM] - st[BODY]), -1.9, 1.9);
        let r1 = clamp(wrap_angle(st[IA] - st[IB]) + wrap_angle(st[IB] - st[BODY]), -1.9, 1.9);
        let sa = turn_step(st[WA], r1 - r0, k(TURN_AIM) * h, k(TURN_AIM_ACC) * h * kk, dt);
        let rr = clamp(r0 + sa.0, -1.9, 1.9);
        st[WA] = if rr == r0 + sa.0 { sa.1 } else { 0.0 };
        st[AIM] = wrap_angle(st[BODY] + rr);
        st[X] = px;
        st[Z] = pz;
        st[BODY_YAW] = st[BODY];
        st[AIM_YAW] = st[AIM];
        let (ld, was) = (clamp(wrap_angle(st[IA] - st[AIM]), -k(TURN_LEAD), k(TURN_LEAD)), st[LEAD]);
        st[LEAD] = damp(was, ld, if abs(ld) < abs(was) { k(TURN_LEAD_RATE) * 2.5 } else { k(TURN_LEAD_RATE) }, dt);
    }
    bench().args[L_IN..L_IN + COLS].copy_from_slice(&a[L_IN..]);
    RAN
}

/// CharacterAnimator.leanBody — a lean round an edge: the trunk rolled about the figure's fore-aft line over the three spine joints, the arms and the gun with
/// it, and the neck and head brought part of the way back so the eyes stay near level.
/// args: 0 dt · 1 the lean asked (−1‥1); then from 64 its column: the lean as shown (eased)
pub fn lean_body(g: usize, at: usize) -> u32 {
    let (dt, lean) = (bench().args[0], bench().args[1]);
    let mut sm = bench().args[L_IN];
    let code = with_pose(g, at, |d, rig, n, _fk| {
        if !cast(n, &[SPINE1, SPINE2, SPINE3, NECK, HEAD]) {
            return 0;
        }
        let want = clamp(lean, -1.0, 1.0);
        sm += (want - sm) * min(1.0, k(LEAN_RATE) * dt);
        if want == 0.0 && abs(sm) < 0.01 {
            sm = 0.0;
        }
        let r = sm * k(LEAN_DEG) * DEGR;
        if r != 0.0 {
            turn(d, rig, role(SPINE1), q_roll(r * 0.3));
            turn(d, rig, role(SPINE2), q_roll(r * 0.35));
            turn(d, rig, role(SPINE3), q_roll(r * 0.35));
            turn(d, rig, role(NECK), q_roll(-r * k(LEAN_LEVEL) * 0.5));
            turn(d, rig, role(HEAD), q_roll(-r * k(LEAN_LEVEL) * 0.5));
        }
        RAN
    })
    .unwrap_or(0);
    if code != 0 {
        bench().args[L_IN] = sm;
    }
    code
}

/// CharacterAnimator.actPose — a one-shot action's clip laid over the finished pose (the animator keeps which clip, its clock and its weight), and a man asleep
/// in his chair slumped over it.
/// args: 0 the weight · 1 the clip's number · 2 its clock · 3 it loops (1 or 0) · 4 dt · 5 the slump asked; then from 64 its column: the slump as shown
pub fn act_pose(g: usize, at: usize, at_b: usize, at_rest: usize) -> u32 {
    let mut a = [0.0f64; 6];
    a.copy_from_slice(&bench().args[..6]);
    let (w, clip, t, looped, dt, slump) = (a[0], a[1], a[2], a[3] != 0.0, a[4], a[5]);
    let mut slump_w = bench().args[L_IN];
    let Some(Some(n)) = with_pose(g, at, |_d, _rig, n, _fk| if cast(n, &[SPINE3, NECK, HEAD]) { Some(n) } else { None }) else {
        return 0;
    };
    if !sample_on_rest(g, at_rest, at_b, n, clip, t, looped) || !ask_blend(g, at, at_b, w, false, at, n) {
        return 0;
    }
    let mut code = RAN;
    if slump > 0.0 || slump_w > 0.0 {
        let ds = slump - slump_w;
        slump_w += if ds > 0.0 { min(ds, dt / 1.6) } else { max(ds, -dt / 0.35) };
        let s = slump_w * w;
        if s > 1e-4 {
            let br = sin(t * 1.75);
            code = with_pose(g, at, |d, rig, _n, fk| {
                fk_run(rig, d, fk.rot, fk.pos);
                turn(d, rig, role(SPINE3), q_pitch((10.0 + 1.5 * br) * DEGR * s));
                turn(d, rig, role(NECK), q_pitch(17.0 * DEGR * s));
                turn(d, rig, role(HEAD), q_mul(&q_pitch((25.0 + br) * DEGR * s), &q_roll(8.0 * DEGR * s)));
                RAN | RAN_ALL
            })
            .unwrap_or(0);
        }
    }
    if code != 0 {
        bench().args[L_IN] = slump_w;
    }
    code
}

/// CharacterAnimator.woundBody — the wounded man's pose over the finished one: his clip (the animator keeps its clock: the way down, then a slow breath across
/// the hold) laid over every node and, once he is most of the way down, his head brought up to look at the wound.
/// args: 0 the weight · 1 the clip's number · 2 its time · 3 how far down he is · 4 the head's pitch for this pose (°)
pub fn wound_body(g: usize, at: usize, at_b: usize, at_rest: usize) -> u32 {
    let mut a = [0.0f64; 5];
    a.copy_from_slice(&bench().args[..5]);
    let (w, clip, t, down, head) = (a[0], a[1], a[2], a[3], a[4]);
    let Some(Some(n)) = with_pose(g, at, |_d, _rig, n, _fk| if cast(n, &[NECK, HEAD]) { Some(n) } else { None }) else {
        return 0;
    };
    if !sample_on_rest(g, at_rest, at_b, n, clip, t, false) || !ask_blend(g, at, at_b, w, false, at, n) {
        return 0;
    }
    if down > 0.003 && head != 0.0 {
        return with_pose(g, at, |d, rig, _n, fk| {
            fk_run(rig, d, fk.rot, fk.pos);
            turn(d, rig, role(NECK), q_pitch(head * 0.45 * DEGR * down));
            turn(d, rig, role(HEAD), q_pitch(head * 0.55 * DEGR * down));
            RAN | RAN_ALL
        })
        .unwrap_or(0);
    }
    RAN
}

/// CharacterAnimator.gazeKeep — the gaze stabiliser: the head kept level and steady while the body works under it. Measured off the finished pose (the head's
/// chain alone), it works out the pitch the neck and head take toward the gait's own eye line and the yaw wobble they take out against a half-second
/// reference of itself. It turns nothing: the host pushes the two twists.
/// args: 0 dt · 1 the weight wanted · 2 the crouch shown · 3 the sprint's share · 4 the goggle nod as posed · 5 hurt's weight (mvs.hu.x) · 6 shaken's
/// (mvs.sk.x) · 7‥9 gazeLocal; then from 64 its columns: 0 its weight · 1 the pitch it holds · 2 the yaw's slow reference · 3 that reference is set (1 or 0)
/// · and, out only, 4 the yaw it takes out this frame
pub fn gaze_keep(g: usize, at: usize) -> u32 {
    const COLS: usize = 5;
    const W: usize = 0;
    const PITCH: usize = 1;
    const REF: usize = 2;
    const INIT: usize = 3;
    const FIX: usize = 4;
    let mut a = [0.0f64; L_IN + COLS];
    a.copy_from_slice(&bench().args[..L_IN + COLS]);
    let (dt, want_w, crouch, sprint, nod, hu, sk, gaze_local) = (a[0], a[1], a[2], a[3], a[4], a[5], a[6], [a[7], a[8], a[9]]);
    let st = &mut a[L_IN..];
    st[FIX] = 0.0;
    let code = with_pose(g, at, |d, rig, n, fk| {
        if !cast(n, &[NECK, HEAD]) {
            return 0;
        }
        st[W] = damp(st[W], want_w, 10.0, dt);
        if st[W] > 0.01 {
            fk.part(rig, d, list(FK_HEAD));
            let gz = q_rotate(&fk.rot(role(HEAD)), &gaze_local);
            let (el, gyaw) = (asin(clamp(gz[1], -1.0, 1.0)) + nod, atan2(gz[0], gz[2]));
            let bow = (k(HURT_PITCH) + k(HURT_HEAD)) * clamp(hu, 0.0, 1.0) + (k(SHAKEN_PITCH) + k(SHAKEN_HEAD)) * clamp(sk, 0.0, 1.0);
            let target_el = -(k(GAZE_STAND) + (k(GAZE_CROUCH) - k(GAZE_STAND)) * crouch + (k(GAZE_SPRINT) - k(GAZE_STAND)) * sprint * (1.0 - crouch) + bow) * DEGR;
            let want_pitch = clamp((el - target_el) * k(GAZE_KEEP), -30.0 * DEGR, 30.0 * DEGR);
            st[PITCH] = damp(st[PITCH], want_pitch, 25.0, dt);
            if st[INIT] == 0.0 {
                st[REF] = gyaw;
                st[INIT] = 1.0;
            }
            st[REF] += wrap_angle(gyaw - st[REF]) * clamp(dt / 0.5, 0.0, 1.0);
            st[FIX] = clamp(-wrap_angle(gyaw - st[REF]) * k(GAZE_YAW_DAMP), -12.0 * DEGR, 12.0 * DEGR);
            RAN | ran_list(FK_HEAD)
        } else {
            st[INIT] = 0.0;
            st[PITCH] = damp(st[PITCH], 0.0, 10.0, dt);
            RAN
        }
    })
    .unwrap_or(0);
    if code != 0 {
        bench().args[L_IN..L_IN + COLS].copy_from_slice(&a[L_IN..]);
    }
    code
}

// ================================================================ THE ARMS
// An arm is posed on a COPY of the pose (a second pose of the pool, the animator's pB) and laid back through a mask: the copy is made with `ask_blend` at
// weight 1 (singles copied, none worked out), the limbs are solved on it, the mask is written from the named joints (`ask_mask`: skeleton.ts buildMask) and
// the copy is blended in. The run-forward bones are then the COPY's, as the TypeScript leaves them.

/// both arms from the collar bones down (CharacterAnimator.armsMask)
fn arms_mask() {
    ask_mask(&[(role(SHOULDER_L), 1.0, true), (role(SHOULDER_R), 1.0, true)]);
}
const ARM_ROLES: [usize; 9] = [SPINE3, SHOULDER_L, SHOULDER_R, UPPER_ARM_L, UPPER_ARM_R, FOREARM_L, FOREARM_R, HAND_L, HAND_R];

/// CharacterAnimator.sneakArms — the sneak's arms: hanging hands drawn in toward the thighs, or the two-hand carry brought down and back toward the lap, the
/// support hand rigid with wherever the gun hand got to.
/// args: 0 armsIn · 1 crouch · 2 the upper body at ease (its weight)
pub fn sneak_arms(g: usize, at: usize, at_b: usize) -> u32 {
    let (arms_in, crouch, at_ease) = (bench().args[0], bench().args[1], bench().args[2]);
    let Some(Some(n)) = with_pose(g, at, |_d, _rig, n, _fk| if cast(n, &ARM_ROLES) { Some(n) } else { None }) else {
        return 0;
    };
    if !ask_blend(g, at, at, 1.0, false, at_b, n) {
        return 0;
    }
    let code = with_pose(g, at_b, |d, rig, _n, fk| {
        fk.part(rig, d, list(FK_ARMS));
        let (cr, r) = (clamp(crouch, 0.0, 1.0), clamp(at_ease, 0.0, 1.0));
        let kk = lerp(0.5, 1.0, cr);
        let (hand_r, hand_l) = (role(HAND_R), role(HAND_L));
        let (pr0, pl0, rr0, rl0) = (fk.at(hand_r), fk.at(hand_l), fk.rot(hand_r), fk.rot(hand_l));
        let gun_in = lerp3(&[k(SNEAK_GUN_IN_STAND_0), k(SNEAK_GUN_IN_STAND_1), k(SNEAK_GUN_IN_STAND_2)], &[k(SNEAK_GUN_IN_0), k(SNEAK_GUN_IN_1), k(SNEAK_GUN_IN_2)], cr);
        let gun_r = add3(pr0, gun_in);
        // (the right hand lives at −x: in = toward the middle, never past it)
        let free_r = [pr0[0] + min(k(SNEAK_HAND_IN), max(0.0, -pr0[0])) * kk, pr0[1] - 0.02 * kk, pr0[2] - 0.03 * kk];
        let free_l = [pl0[0] - min(k(SNEAK_HAND_IN), max(0.0, pl0[0])) * kk, pl0[1] - 0.02 * kk, pl0[2] - 0.03 * kk];
        let pole_w = lerp(0.35, 0.7, r);
        two_bone_ik(rig, d, fk.rot, fk.pos, role(UPPER_ARM_R), role(FOREARM_R), hand_r, &lerp3(&free_r, &gun_r, r), Some(&lerp3(&[-0.05, 0.4, -1.0], &[-0.12, -1.0, -0.5], r)), pole_w, Some(&rr0), false);
        let gun_l = add3(fk.at(hand_r), sub3(pl0, pr0));
        two_bone_ik(rig, d, fk.rot, fk.pos, role(UPPER_ARM_L), role(FOREARM_L), hand_l, &lerp3(&free_l, &gun_l, r), Some(&lerp3(&[0.05, 0.4, -1.0], &[0.2, -1.0, -0.35], r)), pole_w, Some(&rl0), true);
        RAN | RAN_ALL
    })
    .unwrap_or(0);
    if code == 0 {
        return 0;
    }
    arms_mask();
    if !ask_blend(g, at, at_b, clamp(arms_in, 0.0, 1.0), true, at, n) {
        return 0;
    }
    code
}

/// v3.cross
#[inline(always)]
fn cross3(a: &[f64; 3], b: &[f64; 3]) -> [f64; 3] {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}
/// v3.len
#[inline(always)]
fn len3(a: &[f64; 3]) -> f64 {
    hypot3(a[0], a[1], a[2])
}
/// CharacterAnimator.gripTurn: the rotation that carries the aim clip's grip (bore `g0`, gun hand at model rotation `rr0`) onto the wanted bore `gd` — the
/// shortest arc, then unrolled about the bore so the pistol's top strap (`grip_up`, in the hand) faces `up_ref` as nearly as the bore allows
fn grip_turn(g0: &[f64; 3], rr0: &[f64; 4], gd: &[f64; 3], up_ref: &[f64; 3], grip_up: &[f64; 3]) -> [f64; 4] {
    let mut turn = from_to(g0, gd);
    let up0 = q_rotate(&q_mul(&turn, rr0), grip_up);
    let (want, cur) = (sub3(*up_ref, scale3(*gd, dot3(up_ref, gd))), sub3(up0, scale3(*gd, dot3(&up0, gd))));
    if len3(&want) > 1e-4 && len3(&cur) > 1e-4 {
        let (wn, cn) = (normalize(want), normalize(cur));
        turn = q_mul(&axis_angle(gd, atan2(dot3(&cross3(&cn, &wn), gd), dot3(&cn, &wn))), &turn);
    }
    turn
}
/// CharacterAnimator.carryArms — both arms onto the pistol: the gun hand at the chest + the stance's [side, up, fwd], the muzzle at its pitch and level in
/// roll, the support hand keeping the exact hold it has in the aim clip (both hands moved by one rigid transform of the clip's grip); elbows swivelled down
/// and in by the tuck; a little sway, and during a kick a pump of the gun toward the chest. Solved on a copy seeded with the aim clip's arms (`at_aim`).
/// args: 0 w · 1 the kick's clock · 2 idleT · the stance's springs as they stand: 3 fwd · 4 up · 5 side · 6 pitch · 7 blade · 8 tuck · 9‥11 gripUp
pub fn carry_arms(g: usize, at: usize, at_b: usize, at_aim: usize) -> u32 {
    let mut a = [0.0f64; 12];
    a.copy_from_slice(&bench().args[..12]);
    let (w, kick_t, t, st_fwd, st_up, st_side, st_pitch, st_blade, tuck, grip_up) = (a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7], a[8], [a[9], a[10], a[11]]);
    let Some(Some(n)) = with_pose(g, at, |_d, _rig, n, _fk| if cast(n, &ARM_ROLES) && cast(n, &[MIDDLE1_R]) { Some(n) } else { None }) else {
        return 0;
    };
    // the body as it is, the aim clip's arms
    arms_mask();
    if !ask_blend(g, at, at, 1.0, false, at_b, n) || !ask_blend(g, at_b, at_aim, 1.0, true, at_b, n) {
        return 0;
    }
    let code = with_pose(g, at_b, |d, rig, _n, fk| {
        fk.part(rig, d, list(FK_ARMS));
        let (hand_r, hand_l) = (role(HAND_R), role(HAND_L));
        let (chest, pr0, pl0, rr0) = (fk.at(role(SPINE3)), fk.at(hand_r), fk.at(hand_l), fk.rot(hand_r));
        // the clip's bore axis as posed on this spine
        let g0 = normalize(sub3(fk.at(role(MIDDLE1_R)), pr0));
        // where the gun goes: sway (a slow figure of eight, millimetres) and the kick's pump
        let pump = if kick_t >= 0.0 { 0.05 * bump(kick_t, 0.15, 0.36, 0.5, 0.85) } else { 0.0 };
        let (fwd, up, side) = (st_fwd - pump, st_up + 0.004 * sin(t * 2.1) + 0.02 * pump, st_side + 0.005 * sin(t * 1.3 + 0.6));
        let (pitch, yaw) = ((st_pitch + 0.8 * sin(t * 1.7 + 1.1)) * DEGR + pump * 4.0, 0.6 * sin(t * 1.1) * DEGR + st_blade * DEGR * 0.2);
        let gd = [sin(yaw) * cos(pitch), sin(pitch), cos(yaw) * cos(pitch)];
        let turn = grip_turn(&g0, &rr0, &gd, &AY, &grip_up);
        let pr = [chest[0] + side, chest[1] + up, chest[2] + fwd];
        // the support wrist relative to the gun wrist, turned with the gun
        let grip = q_rotate(&turn, &sub3(pl0, pr0));
        two_bone_ik(rig, d, fk.rot, fk.pos, role(UPPER_ARM_R), role(FOREARM_R), hand_r, &pr, Some(&[-0.12 - 0.6 * (1.0 - tuck), -1.0, -0.5]), 0.4 + 0.55 * tuck, Some(&q_mul(&turn, &rr0)), false);
        // (from where the gun hand actually got to: the hands must not part; the support wrist's rotation read as the solve left the bones)
        let (pl, rl0) = (add3(fk.at(hand_r), grip), fk.rot(hand_l));
        two_bone_ik(rig, d, fk.rot, fk.pos, role(UPPER_ARM_L), role(FOREARM_L), hand_l, &pl, Some(&[0.2 + 0.6 * (1.0 - tuck), -1.0, -0.35]), 0.4 + 0.5 * tuck, Some(&q_mul(&turn, &rl0)), true);
        RAN | RAN_ALL
    })
    .unwrap_or(0);
    if code == 0 {
        return 0;
    }
    arms_mask();
    if !ask_blend(g, at, at_b, w, true, at, n) {
        return 0;
    }
    code
}

// ─────────────────────────────── THE HANDS ───────────────────────────────

/// THE HAND PASS'S COLUMNS (character.ts hn.base, hn.note), after every layer's own numbers and columns: each side's fifteen digit joints as the locomotion
/// left them this frame (the thumb's three, then each finger's three, a rotation each), then each side's five notes (how near a layer's hand came to that,
/// digit by digit; no layer yet = infinity). A layer that shapes a hand is handed them and hands the notes back.
const HANDS_AT: usize = 128;
const HAND_DIGITS: usize = 15;
struct Hands {
    base: [f64; 2 * HAND_DIGITS * 4],
    note: [f64; 10],
    /// the same joints in the rest pose (the straight hand every shape is curled from)
    rest: [[f64; 4]; 2 * HAND_DIGITS],
}
fn digits(side: usize) -> &'static [u32] {
    list(if side == 1 { DIGITS_R } else { DIGITS_L })
}
/// the columns as handed, and the rest pose's digits (`at_rest`, a pose of the same character); None = the rig has no such hands
fn hands_in(g: usize, at_rest: usize) -> Option<Hands> {
    if digits(0).len() != HAND_DIGITS || digits(1).len() != HAND_DIGITS {
        return None;
    }
    let mut h = Hands { base: [0.0; 120], note: [0.0; 10], rest: [[0.0; 4]; 30] };
    h.base.copy_from_slice(&bench().args[HANDS_AT..HANDS_AT + 120]);
    h.note.copy_from_slice(&bench().args[HANDS_AT + 120..HANDS_AT + 130]);
    let rest = &mut h.rest;
    with_pose(g, at_rest, |d, _rig, n, _fk| {
        for side in 0..2 {
            for (i, &node) in digits(side).iter().enumerate() {
                let o = node as usize * 10 + 3;
                if (node as usize) < n {
                    rest[side * HAND_DIGITS + i] = [get(d, o), get(d, o + 1), get(d, o + 2), get(d, o + 3)];
                }
            }
        }
    })?;
    Some(h)
}
fn hands_out(h: &Hands) {
    bench().args[HANDS_AT + 120..HANDS_AT + 130].copy_from_slice(&h.note);
}
/// what a Float32Array keeps of a double
#[inline(always)]
fn stored(v: f64) -> f64 {
    let mut s = [S::ZERO];
    put(&mut s, 0, v);
    get(&s, 0)
}
/// character.ts qAng: the angle between a stored rotation and one of four doubles
#[inline(always)]
fn q_ang(d: &[S], o: usize, b: &[f64], k: usize) -> f64 {
    let dot = abs(get(d, o) * b[k] + get(d, o + 1) * b[k + 1] + get(d, o + 2) * b[k + 2] + get(d, o + 3) * b[k + 3]);
    if dot >= 1.0 {
        0.0
    } else {
        2.0 * acos(dot)
    }
}
/// full-fist flexion per knuckle (°), and the thumb's, joint by joint
const FIST: [f64; 3] = [72.0, 92.0, 62.0];
const THUMB_FIST: [f64; 3] = [30.0, 25.0, 45.0];
/// CharacterAnimator.hand — curl the four fingers (0 straight … 1 fist) and set the thumb (0 alongside … 1 folded over the fist) of one hand, from the rest
/// (straight) hand; then the hand pass's note: a layer has set this hand, and how far its shape stands from where the locomotion left each digit.
fn hand(d: &mut [S], h: &mut Hands, side: usize, curl: &[f64; 4], thumb: f64, spread: f64) {
    let (ns, at) = (digits(side), side * HAND_DIGITS);
    for fi in 0..4 {
        for j in 0..3 {
            let mut q = q_pitch(curl[fi] * FIST[j] * DEGR);
            if j == 0 && !falsy(spread) {
                // fan about the knuckle's palm-normal axis
                q = q_mul(&q_roll((fi as f64 - 1.2) * spread * (if side == 1 { 1.0 } else { -1.0 }) * 9.0 * DEGR), &q);
            }
            let k = 3 + fi * 3 + j;
            set_unit(d, ns[k] as usize, &q_mul(&h.rest[at + k], &q));
        }
    }
    for j in 0..3 {
        set_unit(d, ns[j] as usize, &q_mul(&h.rest[at + j], &q_pitch(thumb * THUMB_FIST[j] * DEGR)));
    }
    if on(MOVES_ON) {
        let b = &h.base[side * 60..side * 60 + 60];
        for g in 0..5 {
            let k = g * 3;
            let far = (q_ang(d, ns[k] as usize * 10 + 3, b, k * 4) + q_ang(d, ns[k + 1] as usize * 10 + 3, b, k * 4 + 4) + q_ang(d, ns[k + 2] as usize * 10 + 3, b, k * 4 + 8)) / 3.0;
            h.note[side * 5 + g] = stored(min(h.note[side * 5 + g], far));
        }
    }
}
/// quat.fromBasis: the rotation whose axes are x, y, z
fn from_basis(x: &[f64; 3], y: &[f64; 3], z: &[f64; 3]) -> [f64; 4] {
    let (m00, m10, m20, m01, m11, m21, m02, m12, m22) = (x[0], x[1], x[2], y[0], y[1], y[2], z[0], z[1], z[2]);
    let tr = m00 + m11 + m22;
    q_unit(if tr > 0.0 {
        let s = sqrt(tr + 1.0) * 2.0;
        [(m21 - m12) / s, (m02 - m20) / s, (m10 - m01) / s, 0.25 * s]
    } else if m00 > m11 && m00 > m22 {
        let s = sqrt(1.0 + m00 - m11 - m22) * 2.0;
        [0.25 * s, (m01 + m10) / s, (m02 + m20) / s, (m21 - m12) / s]
    } else if m11 > m22 {
        let s = sqrt(1.0 + m11 - m00 - m22) * 2.0;
        [(m01 + m10) / s, 0.25 * s, (m12 + m21) / s, (m02 - m20) / s]
    } else {
        let s = sqrt(1.0 + m22 - m00 - m11) * 2.0;
        [(m02 + m20) / s, (m12 + m21) / s, 0.25 * s, (m10 - m01) / s]
    })
}
/// CharacterAnimator.handRot: the model rotation of a hand whose fingers point along `f` with the back of the hand to `b` (made square to f); `fix` is that
/// side's calibration from the rest pose (the animator's handFix, handed in with the layer's numbers)
fn hand_rot(fix: &[f64; 4], f: [f64; 3], b: [f64; 3]) -> [f64; 4] {
    let ff = normalize(f);
    let bb = normalize(mad3(b, ff, -dot3(&b, &ff)));
    q_mul(&from_basis(&cross3(&ff, &bb), &ff, &bb), fix)
}

/// CharacterAnimator.wallArms — the wall hold's arms: both palms laid on the face beside the hips, fingers down and out along it, the elbows where the pose
/// says; mended (WALLSNAP), the wrists hang beside the hips, a resting hand on the plaster with the thumb beside the index.
/// args: 0 w · 1 crouch · 2 side · 3 peek · 4 the wall's yaw · 5 the hold's gait · 6 park · 7‥10 the left hand's fix · 11‥14 the right's
pub fn wall_arms(g: usize, at: usize, at_b: usize, at_rest: usize) -> u32 {
    let mut a = [0.0f64; 15];
    a.copy_from_slice(&bench().args[..15]);
    let Some(Some(n)) = with_pose(g, at, |_d, _rig, n, _fk| if cast(n, &ARM_ROLES) && cast(n, &[HIPS]) { Some(n) } else { None }) else {
        return 0;
    };
    let Some(mut h) = hands_in(g, at_rest) else {
        return 0;
    };
    if !ask_blend(g, at, at, 1.0, false, at_b, n) {
        return 0;
    }
    let hh = &mut h;
    let code = with_pose(g, at_b, |d, rig, _n, fk| {
        let (w_cr, s, pk) = (clamp(a[1], 0.0, 1.0), clamp(a[2], -1.0, 1.0), clamp(a[3], 0.0, 1.0));
        let e = pk * pk * (3.0 - 2.0 * pk);
        // the wall's normal (into the room) and his right along the wall, model space
        let nn = [sin(a[4]), 0.0, cos(a[4])];
        let tr = [-nn[2], 0.0, nn[0]];
        fk_run(rig, d, fk.rot, fk.pos);
        let mend = truthy(WS_ON) && truthy(WS_POSE);
        let hips = fk.at(role(HIPS));
        let (hx, hy, hz) = (hips[0], hips[1] + if mend { lerp(k(WM_UP_0), k(WM_UP_1), w_cr) } else { lerp(k(WH_UP), k(WH_UP_CROUCH), w_cr) }, hips[2]);
        let along0 = if mend {
            lerp(k(WM_ALONG_0), k(WM_ALONG_1), w_cr) - k(WM_SWAY) * cos(a[5] * 2.0 * PI) * (1.0 - clamp(a[6], 0.0, 1.0)) * lerp(k(WG_SHARE_0), k(WG_SHARE_1), w_cr)
        } else {
            lerp(k(WH_ALONG), k(WH_ALONG_CROUCH), w_cr)
        };
        let (e_out, e_room, e_down) = if mend { (k(WM_ELBOW_OUT), k(WM_ELBOW_ROOM), k(WM_ELBOW_DOWN)) } else { (k(WE_OUT), k(WE_ROOM), k(WE_DOWN)) };
        let splay = if mend { k(WM_SPLAY) } else { k(WH_OUT) };
        for side in 0..2usize {
            // his right is +tr: the right hand goes that way, the left the other
            let sg = if side == 1 { 1.0 } else { -1.0 };
            let along = along0 - k(WH_PEEK_IN) * e * (if side == 1 { 1.0 + s } else { 1.0 - s }) / 2.0;
            let mut p = [hx + tr[0] * sg * along, hy, hz + tr[2] * sg * along];
            // carried square onto the plane, the wrist proud of it
            let onto = (-k(WALL_ROOT_OFF) + k(WH_OFF)) - dot3(&p, &nn);
            p[0] += nn[0] * onto;
            p[2] += nn[2] * onto;
            let fix = [a[7 + side * 4], a[8 + side * 4], a[9 + side * 4], a[10 + side * 4]];
            let r = hand_rot(&fix, [tr[0] * sg * splay, -1.0, tr[2] * sg * splay], nn);
            let pole = [tr[0] * sg * e_out + nn[0] * e_room, -e_down, tr[2] * sg * e_out + nn[2] * e_room];
            let (up, fore, hnd) = if side == 1 { (role(UPPER_ARM_R), role(FOREARM_R), role(HAND_R)) } else { (role(UPPER_ARM_L), role(FOREARM_L), role(HAND_L)) };
            two_bone_ik(rig, d, fk.rot, fk.pos, up, fore, hnd, &p, Some(&pole), 0.9, Some(&r), false);
            if mend {
                hand(d, hh, side, &[k(WM_CURL_0), k(WM_CURL_1), k(WM_CURL_2), k(WM_CURL_3)], k(WM_THUMB), k(WM_FAN));
                // the thumb beside the index, out of the plaster
                turn(d, rig, digits(side)[0] as usize, axis_angle(&q_rotate(&fk.rot(hnd), &AY), sg * k(WM_THUMB_IN) * DEGR));
            } else {
                hand(d, hh, side, &[0.05, 0.06, 0.08, 0.12], 0.1, 0.5);
            }
        }
        RAN | RAN_ALL
    })
    .unwrap_or(0);
    if code == 0 {
        return 0;
    }
    arms_mask();
    if !ask_blend(g, at, at_b, a[0], true, at, n) {
        return 0;
    }
    hands_out(&h);
    code
}

/// CharacterAnimator.trunkFrame: the trunk's own left, up and forward, off the run-forward bones
fn trunk_frame(fk: &Fk) -> ([f64; 3], [f64; 3], [f64; 3]) {
    let up = normalize(sub3(fk.at(role(SPINE3)), fk.at(role(HIPS))));
    let l = sub3(fk.at(role(UPPER_ARM_L)), fk.at(role(UPPER_ARM_R)));
    let left = normalize(mad3(l, up, -dot3(&l, &up)));
    (left, up, cross3(&left, &up))
}
/// CharacterAnimator.woundLimbs — the limbs the wound's clip does not pose as a hurt man's: the support hand pressed to the wound (the ribs under the gun arm,
/// or the top of the near thigh) and, on his back, that leg's knee drawn up off the floor and the gun hand in on his belly.
/// args: 0 w · 1 he is on his back (1 or 0) · 2 the wound is at the ribs (1 or 0) · 3‥6 the left hand's fix · 7‥10 the right's
pub fn wound_limbs(g: usize, at: usize, at_b: usize, at_rest: usize) -> u32 {
    let mut a = [0.0f64; 11];
    a.copy_from_slice(&bench().args[..11]);
    let (w, back, ribs) = (a[0], a[1] != 0.0, a[2] != 0.0);
    let Some(Some(n)) = with_pose(g, at, |_d, _rig, n, _fk| if cast(n, &ARM_ROLES) && cast(n, &[HIPS, SPINE1, THIGH_L, SHIN_L, FOOT_L]) { Some(n) } else { None }) else {
        return 0;
    };
    let Some(mut h) = hands_in(g, at_rest) else {
        return 0;
    };
    if !ask_blend(g, at, at, 1.0, false, at_b, n) {
        return 0;
    }
    let hh = &mut h;
    let code = with_pose(g, at_b, |d, rig, _n, fk| {
        fk_run(rig, d, fk.rot, fk.pos);
        if back {
            let (hp, ft) = (fk.at(role(THIGH_L)), fk.at(role(FOOT_L)));
            let to = [hp[0] + (ft[0] - hp[0]) * 0.56, ft[1] + 0.02, hp[2] + (ft[2] - hp[2]) * 0.56];
            two_bone_ik(rig, d, fk.rot, fk.pos, role(THIGH_L), role(SHIN_L), role(FOOT_L), &to, Some(&[0.0, 1.0, 0.0]), 1.0, None, false);
        }
        let (left, up, fwd) = trunk_frame(fk);
        let tk = |o: [f64; 3], x: f64, y: f64, z: f64| mad3(mad3(mad3(o, left, x), up, y), fwd, z);
        let p = if ribs {
            tk(fk.at(role(SPINE1)), k(WOUND_AT_0), k(WOUND_AT_1), k(WOUND_AT_2))
        } else {
            let (th, sh, t) = (fk.at(role(THIGH_L)), fk.at(role(SHIN_L)), if back { 0.5 } else { 0.42 });
            mad3(mad3([lerp(th[0], sh[0], t), lerp(th[1], sh[1], t), lerp(th[2], sh[2], t)], fwd, 0.085), left, 0.02)
        };
        let (fix_l, fix_r) = ([a[3], a[4], a[5], a[6]], [a[7], a[8], a[9], a[10]]);
        let r = hand_rot(&fix_l, mad3(scale3(left, -1.0), up, -0.25), mad3(fwd, up, 0.2));
        two_bone_ik(rig, d, fk.rot, fk.pos, role(UPPER_ARM_L), role(FOREARM_L), role(HAND_L), &p, Some(&[0.6, -0.7, -0.3]), 0.7, Some(&r), false);
        hand(d, hh, 0, &[0.3, 0.34, 0.38, 0.42], 0.3, 0.2);
        if back {
            let (to, pole) = (tk(fk.at(role(SPINE1)), -0.07, -0.04, 0.13), mad3(scale3(left, -1.0), up, -0.6));
            let r = hand_rot(&fix_r, mad3(left, up, -0.2), fwd);
            two_bone_ik(rig, d, fk.rot, fk.pos, role(UPPER_ARM_R), role(FOREARM_R), role(HAND_R), &to, Some(&pole), 0.8, Some(&r), false);
            hand(d, hh, 1, &[0.25, 0.3, 0.34, 0.38], 0.25, 0.2);
        }
        RAN | RAN_ALL
    })
    .unwrap_or(0);
    if code == 0 || !ask_blend(g, at, at_b, w, false, at, n) {
        return 0;
    }
    hands_out(&h);
    code
}

/// LC of character.ts: the long gun's channels, in the order the animator keeps them
const LC_SIDE: usize = 0;
const LC_UP: usize = 1;
const LC_FWD: usize = 2;
const LC_PITCH: usize = 3;
const LC_AIM_K: usize = 4;
const LC_YAW: usize = 5;
const LC_ELBOW_R: usize = 11;
const LC_ELBOW_L: usize = 14;
const LC_POLE_W: usize = 17;
const LC_SWAY: usize = 18;
const LC_ANCHOR: usize = 19;
const LC_STEADY: usize = 20;
const LC_COUNT: usize = 21;
/// CharacterAnimator.longArms — the long gun's arms: the trigger hand's wrist at the carriage's point off the chest joint (or, by its anchor, off the gun
/// shoulder), swung about the shoulders with the aim's share of the pitch, holding the grip as the aim clip holds the pistol's turned onto the wanted bore;
/// the support hand solved onto the gun's grip point from where the trigger hand got to. Should the support arm fall short the whole gun is brought back by
/// the shortfall (capped) and both re-solved, and that hold-back is kept and let out over half a second. Solved on a copy seeded with the aim clip's arms.
/// args: 0 w · 1 the kick's clock · 2 dt · 3 it yields to a hand on a door (1 or 0) · 4 this pass keeps what it holds over (1 or 0) · 5 idleT · 6 upperW.aim
/// · 7 aimPitch · 8 shootT · 9 longSteady · 10 trained · the recoil's sums: 11 yaw · 12 climb · 13 back · 14 the dip shown · 15 the reload carry's weight ·
/// 16‥18 its point · 19, 20 the door hands' weights (left, right) · 21‥23 gripUp · 24‥27 the left hand's fix · the gun's grip: 28‥30 the support wrist ·
/// 31‥33 its fingers · 34‥36 the back of that hand · 37‥57 the carriage's channels (LC); then from 64 its columns: 0, 1 the anchor's two poles · 2‥4 the
/// hold-back · 5 the sights' lift; and the hand pass's columns
pub fn long_arms(g: usize, at: usize, at_b: usize, at_aim: usize, at_rest: usize) -> u32 {
    const COLS: usize = 6;
    let mut a = [0.0f64; L_IN + COLS];
    a.copy_from_slice(&bench().args[..L_IN + COLS]);
    let (w, kick_t, dt, yield_door, keep, t, aim, aim_pitch, shoot_t) = (a[0], a[1], a[2], a[3] != 0.0, a[4] != 0.0, a[5], a[6], a[7], a[8]);
    let (long_steady, trained, mvs_yaw, mvs_climb, mvs_back, dip_w, rl_k) = (a[9], a[10], a[11], a[12], a[13], a[14], a[15]);
    let (rl_at, dl, dr) = ([a[16], a[17], a[18]], a[19], a[20]);
    let (grip_up, fix_l) = ([a[21], a[22], a[23]], [a[24], a[25], a[26], a[27]]);
    let (g_wrist, g_fingers, g_back) = ([a[28], a[29], a[30]], [a[31], a[32], a[33]], [a[34], a[35], a[36]]);
    let mut ch = [0.0f64; LC_COUNT];
    ch.copy_from_slice(&a[37..37 + LC_COUNT]);
    let Some(Some(n)) = with_pose(g, at, |_d, _rig, n, _fk| if cast(n, &ARM_ROLES) && cast(n, &[MIDDLE1_R]) { Some(n) } else { None }) else {
        return 0;
    };
    let Some(mut h) = hands_in(g, at_rest) else {
        return 0;
    };
    // the body as it is, the aim clip's clavicles, trigger grip and fingers to start from
    arms_mask();
    if !ask_blend(g, at, at, 1.0, false, at_b, n) || !ask_blend(g, at_b, at_aim, 1.0, true, at_b, n) {
        return 0;
    }
    let (hh, st) = (&mut h, &mut a[L_IN..]);
    let code = with_pose(g, at_b, |d, rig, _n, fk| {
        fk.part(rig, d, list(FK_ARMS));
        let (hand_r, hand_l) = (role(HAND_R), role(HAND_L));
        let (chest, pr0, rr0) = (fk.at(role(SPINE3)), fk.at(hand_r), fk.rot(hand_r));
        // the clip's bore as posed on this spine
        let g0 = normalize(sub3(fk.at(role(MIDDLE1_R)), pr0));
        // the bore, model space
        let sway = ch[LC_SWAY];
        let rcl = mv_on(MOVES_RECOIL);
        let yaw = (ch[LC_YAW] + 0.6 * sway * sin(t * 1.1) + if rcl { mvs_yaw } else { 0.0 }) * DEGR;
        let pitch0 = clamp((ch[LC_PITCH] + 0.8 * sway * sin(t * 1.7 + 1.1) + if rcl { mvs_climb } else { 0.0 }) * DEGR + aim_pitch * (ch[LC_AIM_K] - 0.35 * aim), -1.35, 1.35);
        // (only ever down, and only from a gun that is about level)
        let dip_k = if dip_w > 0.0 { dip_w * (1.0 - smoothstep(0.3, 0.6, pitch0)) } else { 0.0 };
        let pitch = if dip_k > 0.0 { lerp(pitch0, min(pitch0, k(DIP_PITCH) * DEGR), dip_k) } else { pitch0 };
        if dip_k > 0.0 && w > 0.99 && mv_on(MOVES_ENTRY_EYES) {
            st[5] = pitch0 - pitch;
        }
        let cp = cos(pitch);
        let dd = [sin(yaw) * cp, sin(pitch), cos(yaw) * cp];
        let rr = q_mul(&grip_turn(&g0, &rr0, &dd, &AY, &grip_up), &rr0);
        // the gun's own frame about that bore: y up out of the receiver, x to the figure's left
        let mut yw = mad3(AY, dd, -dd[1]);
        yw = if len3(&yw) > 1e-4 { normalize(yw) } else { [cos(yaw), 0.0, -sin(yaw)] };
        let xw = cross3(&yw, &dd);
        let in_gun = |p: &[f64; 3]| -> [f64; 3] { [xw[0] * p[0] + yw[0] * p[1] + dd[0] * p[2], xw[1] * p[0] + yw[1] * p[1] + dd[1] * p[2], xw[2] * p[0] + yw[2] * p[1] + dd[2] * p[2]] };
        // the trigger hand: the carriage's point off the chest joint, pumped toward the chest by a kick, pulled back along the bore by a shot
        let kick = if shoot_t >= 0.0 {
            if shoot_t < 0.035 {
                shoot_t / 0.035
            } else {
                max(0.0, 1.0 - (shoot_t - 0.035) / 0.2)
            }
        } else {
            0.0
        };
        let pump = if kick_t >= 0.0 { 0.04 * bump(kick_t, 0.15, 0.36, 0.5, 0.85) } else { 0.0 };
        let ka = clamp(ch[LC_ANCHOR], 0.0, 1.0);
        let sh = fk.at(role(UPPER_ARM_R));
        let mut anchor = [lerp(chest[0], sh[0], ka), lerp(chest[1], sh[1], ka), lerp(chest[2], sh[2], ka)];
        anchor[0] = max(anchor[0], chest[0] - k(LONG_ANCHOR_SIDE_MAX));
        // steadied against the gait: the anchor's side-to-side followed through a two-pole low-pass
        {
            let (mut l0, mut l1) = (st[0], st[1]);
            if !finite(l0) || !finite(l1) {
                l0 = anchor[0];
                l1 = anchor[0];
            } else {
                let kk = 1.0 - exp(-dt / max(1e-3, k(LONG_STEADY_TAU)));
                l0 += (anchor[0] - l0) * kk;
                l1 += (l0 - l1) * kk;
            }
            if keep {
                st[0] = l0;
                st[1] = l1;
            }
            let iso = clamp(ch[LC_STEADY], 0.0, 1.0) * clamp(long_steady, 0.0, 1.0);
            if iso > 1e-3 {
                anchor[0] = lerp(anchor[0], l1, iso);
            }
        }
        let mut off = [
            anchor[0] - chest[0] + ch[LC_SIDE] + 0.004 * sway * sin(t * 1.3 + 0.6),
            anchor[1] - chest[1] + ch[LC_UP] + 0.003 * sway * sin(t * 2.1),
            anchor[2] - chest[2] + ch[LC_FWD] - pump,
        ];
        if rl_k > 0.0 {
            // (the reload carry's own point)
            off[0] += (rl_at[0] - off[0]) * rl_k;
            off[1] += (rl_at[1] - off[1]) * rl_k;
            off[2] += (rl_at[2] - off[2]) * rl_k;
        }
        {
            // the swing's pivot: between the shoulders … the gun shoulder itself
            let (py, pz) = (lerp(k(LONG_PIVOT_Y), anchor[1] - chest[1], ka), lerp(k(LONG_PIVOT_Z), anchor[2] - chest[2], ka));
            let (th, ry, rz) = (aim_pitch * ch[LC_AIM_K], off[1] - py, off[2] - pz);
            off[1] = py + ry * cos(th) + rz * sin(th);
            off[2] = pz - ry * sin(th) + rz * cos(th);
            // less the twist's tilt of the whole trunk about the chest joint
            let (ph, oy, oz) = (-0.35 * aim_pitch * aim, off[1], off[2]);
            off[1] = oy * cos(ph) + oz * sin(ph);
            off[2] = -oy * sin(ph) + oz * cos(ph);
        }
        let mut pr = add3(chest, off);
        if dip_k > 0.0 {
            pr[1] -= k(DIP_DOWN) * dip_k;
            pr[2] -= k(DIP_BACK) * dip_k;
        }
        if rcl {
            if mvs_back > 1e-5 {
                pr = mad3(pr, dd, -mvs_back);
            }
        } else if kick > 0.0 {
            pr = mad3(pr, dd, -k(LONG_KICK_BACK) * kick);
        }
        let mut pole_r = [ch[LC_ELBOW_R], ch[LC_ELBOW_R + 1], ch[LC_ELBOW_R + 2]];
        let mut pole_l = [ch[LC_ELBOW_L], ch[LC_ELBOW_L + 1], ch[LC_ELBOW_L + 2]];
        let pw = clamp(ch[LC_POLE_W], 0.0, 1.0);
        if mv_on(MOVES_GAIT) {
            // the elbows by training, shouldered
            let (tw, uw) = (smoothstep(0.5, 1.0, trained) * aim, (1.0 - smoothstep(0.0, 0.5, trained)) * aim);
            pole_r[0] += k(GAIT_ELBOW_IN) * tw;
            pole_l[0] -= k(GAIT_ELBOW_IN) * 0.8 * tw;
            pole_r[1] += k(GAIT_WING) * uw;
            pole_r[0] -= 0.25 * uw;
        }
        let rl = hand_rot(&fix_l, in_gun(&g_fingers), in_gun(&g_back));
        let (up_r, fore_r, up_l, fore_l) = (role(UPPER_ARM_R), role(FOREARM_R), role(UPPER_ARM_L), role(FOREARM_L));
        // → how far the support hand fell short of its point (nought when it reached)
        let solve = |d: &mut [S], fk: &mut Fk, to: [f64; 3]| -> [f64; 3] {
            two_bone_ik(rig, d, fk.rot, fk.pos, up_r, fore_r, hand_r, &to, Some(&pole_r), pw, Some(&rr), false);
            // from where the trigger hand actually got to
            let pl = add3(fk.at(hand_r), in_gun(&g_wrist));
            two_bone_ik(rig, d, fk.rot, fk.pos, up_l, fore_l, hand_l, &pl, Some(&pole_l), pw, Some(&rl), true);
            sub3(fk.at(hand_l), pl)
        };
        // out of the support arm's reach: the gun comes back to where it can be held, at once, and is let out again over half a second
        let mut pull = [st[2], st[3], st[4]];
        let m = solve(d, fk, add3(pr, pull));
        if len3(&m) > 5e-4 {
            // still short with what is held: take up the rest now (capped), and pose there
            pull = [pull[0] + m[0], pull[1] + m[1], pull[2] + m[2]];
            let l = len3(&pull);
            if l > k(LONG_PULL_MAX) {
                let kk = k(LONG_PULL_MAX) / l;
                pull = [pull[0] * kk, pull[1] * kk, pull[2] * kk];
            }
            solve(d, fk, add3(pr, pull));
        } else if keep {
            // reached: let a little out for next frame
            let kk = exp(-2.0 * dt);
            pull = [pull[0] * kk, pull[1] * kk, pull[2] * kk];
        }
        if keep {
            st[2] = pull[0];
            st[3] = pull[1];
            st[4] = pull[2];
        }
        // the support hand cupped round the handguard
        hand(d, hh, 0, &[0.5, 0.55, 0.6, 0.68], 0.35, 0.0);
        RAN | RAN_ALL
    })
    .unwrap_or(0);
    if code == 0 {
        return 0;
    }
    if yield_door {
        // arm by arm, short of whatever a hand on a door still holds
        let (wl, wr) = (w * (1.0 - clamp(dl, 0.0, 1.0)), w * (1.0 - clamp(dr, 0.0, 1.0)));
        ask_mask(&[(role(SHOULDER_L), 1.0, true)]);
        if !ask_blend(g, at, at_b, wl, true, at, n) {
            return 0;
        }
        ask_mask(&[(role(SHOULDER_R), 1.0, true)]);
        if !ask_blend(g, at, at_b, wr, true, at, n) {
            return 0;
        }
    } else {
        arms_mask();
        if !ask_blend(g, at, at_b, w, true, at, n) {
            return 0;
        }
    }
    bench().args[L_IN..L_IN + COLS].copy_from_slice(&a[L_IN..]);
    hands_out(&h);
    code
}

/// character.ts curve3: the scalar curve's spline for a keyed vector, one search for the segment and the same Hermite weights for each component
fn curve3(t: f64, k: &[(f64, [f64; 3])]) -> [f64; 3] {
    let n = k.len();
    if t <= k[0].0 {
        return k[0].1;
    }
    if t >= k[n - 1].0 {
        return k[n - 1].1;
    }
    let mut i = 0;
    while t > k[i + 1].0 {
        i += 1;
    }
    let (t0, t1, p0, p1) = (k[i].0, k[i + 1].0, k[i].1, k[i + 1].1);
    let h = t1 - t0;
    let u = (t - t0) / h;
    let u2 = u * u;
    let u3 = u2 * u;
    let (a, b, c, d) = (2.0 * u3 - 3.0 * u2 + 1.0, (u3 - 2.0 * u2 + u) * h, 3.0 * u2 - 2.0 * u3, (u3 - u2) * h);
    let mut out = [0.0f64; 3];
    for (j, o) in out.iter_mut().enumerate() {
        let m0 = if i > 0 { (p1[j] - k[i - 1].1[j]) / (t1 - k[i - 1].0) } else { 0.0 };
        let m1 = if i + 2 < n { (k[i + 2].1[j] - p0[j]) / (k[i + 2].0 - t0) } else { 0.0 };
        *o = a * p0[j] + b * m0 + c * p1[j] + d * m1;
    }
    out
}

/// a hand that left its grip and comes back to it: its model turn eased from `rot0` (the grip it had) to the one the solve gave it, by `wr`
fn wrist_ease(d: &mut [S], rig: &Rig, fk: &mut Fk, hand_n: usize, rot0: &[f64; 4], wr: f64) {
    if wr < 0.999 {
        fk_run(rig, d, fk.rot, fk.pos);
        let (q, par) = (slerp(rot0, &fk.rot(hand_n), wr), parent_rot(rig, d, hand_n));
        set_model_rot(d, hand_n, &q, &par);
    }
}

/// CharacterAnimator.mvPostArms — the reactions' arms, after their body: the start's shoulders and elbows on the pose at `at`; then, each solved on the
/// scratch pose at `at_b` and laid back through its own mask, a blinded man's gun held on the line it had, his hand flat over his eyes, and the toss of the
/// support hand along its keyed path. args: 0 dt · 1 the gate · 2 how much of a long gun is up · 3 the toss's release · 4 the dazzle's clock · 5‥7 the gaze in
/// the head's frame · 8‥11 the left hand's fix · 12 the start's weight · 13 its side · 14 the dazzle's weight · 15 a gun to hold · 16‥18 its point ·
/// 19‥22 its turn. Columns: 0 the toss's clock · 1 how much of the left arm the reactions hold; then the hand pass's.
pub fn mv_post_arms(g: usize, at: usize, at_b: usize, at_rest: usize) -> u32 {
    const COLS: usize = 2;
    let mut a = [0.0f64; L_IN + COLS];
    a.copy_from_slice(&bench().args[..L_IN + COLS]);
    let (dt, gate, lg_w, rel, dz_t) = (a[0], a[1], a[2], a[3], a[4]);
    let (gaze, fix_l) = ([a[5], a[6], a[7]], [a[8], a[9], a[10], a[11]]);
    let (stw, stn, dzw, gun) = (a[12], a[13], a[14], a[15] != 0.0);
    let (gun_p, gun_r) = ([a[16], a[17], a[18]], [a[19], a[20], a[21], a[22]]);
    let (ts, td) = (a[L_IN], rel + k(TOSS_AFTER));
    let mut arm_l = a[L_IN + 1];
    let Some(Some(n)) = with_pose(g, at, |_d, _rig, n, _fk| if cast(n, &ARM_ROLES) && cast(n, &[HEAD]) { Some(n) } else { None }) else {
        return 0;
    };
    let Some(mut h) = hands_in(g, at_rest) else {
        return 0;
    };
    let (up_l, fore_l, hand_l, sh_l) = (role(UPPER_ARM_L), role(FOREARM_L), role(HAND_L), role(SHOULDER_L));
    let mut code = RAN;
    if stw > 0.003 {
        // the start's arms: shoulders up, elbows wide and forearms half raised by what a long gun leaves of them
        let aa = stw * (1.0 - clamp(lg_w, 0.0, 1.0));
        with_pose(g, at, |d, rig, _n, _fk| {
            for (sh, up, fo, sg) in [(sh_l, role(UPPER_ARM_L), role(FOREARM_L), 1.0), (role(SHOULDER_R), role(UPPER_ARM_R), role(FOREARM_R), -1.0)] {
                let far = sg * stn < 0.0;
                turn(d, rig, sh, q_roll(sg * k(START_SHRUG) * DEGR * stw));
                if aa > 0.003 {
                    turn(d, rig, up, q_roll(sg * k(if far { START_WIDE_1 } else { START_WIDE_0 }) * DEGR * aa));
                    turn(d, rig, fo, q_pitch(-k(if far { START_RAISE_1 } else { START_RAISE_0 }) * DEGR * aa));
                }
            }
        });
    }
    if gun {
        // a blinded man's gun stays on the line it had, by how much of a long gun he has up
        if !ask_blend(g, at, at, 1.0, false, at_b, n) {
            return 0;
        }
        with_pose(g, at_b, |d, rig, _n, fk| {
            fk_run(rig, d, fk.rot, fk.pos);
            two_bone_ik(rig, d, fk.rot, fk.pos, role(UPPER_ARM_R), role(FOREARM_R), role(HAND_R), &gun_p, Some(&[-0.3, -1.0, -0.4]), 0.6, Some(&gun_r), false);
        });
        ask_mask(&[(role(UPPER_ARM_R), smoothstep(0.003, 0.15, dzw) * smoothstep(0.02, 0.6, lg_w), true)]);
        if !ask_blend(g, at, at_b, 1.0, true, at, n) {
            return 0;
        }
        code |= RAN_ALL;
    }
    if dzw > 0.003 {
        // blinded: the support hand goes flat over the eyes, wrist by the near eye, fingers across to the far one
        if !ask_blend(g, at, at, 1.0, false, at_b, n) {
            return 0;
        }
        let hh = &mut h;
        with_pose(g, at_b, |d, rig, _n, fk| {
            turn(d, rig, sh_l, q_roll(9.0 * DEGR));
            fk_run(rig, d, fk.rot, fk.pos);
            let hr = fk.rot(role(HEAD));
            let (gz, up) = (q_rotate(&hr, &gaze), q_rotate(&hr, &AY));
            let left = cross3(&up, &gz);
            let eyes = mad3(mad3(fk.at(role(HEAD)), up, 0.1), gz, 0.08);
            let to = mad3(mad3(mad3(eyes, gz, k(DAZZLE_HAND_OUT)), left, 0.04), up, 0.006 * sin(dz_t * 4.3) - k(DAZZLE_HAND_DROP));
            let pole = [k(DAZZLE_HAND_POLE_0), k(DAZZLE_HAND_POLE_1), k(DAZZLE_HAND_POLE_2)];
            let r = hand_rot(&fix_l, mad3(scale3(left, -1.0), up, k(DAZZLE_HAND_TILT)), mad3(gz, up, k(DAZZLE_HAND_ROLL)));
            two_bone_ik(rig, d, fk.rot, fk.pos, up_l, fore_l, hand_l, &to, Some(&pole), 0.85, Some(&r), false);
            hand(d, hh, 0, &[0.22, 0.28, 0.34, 0.4], 0.3, 0.3);
        });
        let wz = smoothstep(0.08, 0.6, dzw);
        ask_mask(&[(sh_l, wz, true)]);
        if !ask_blend(g, at, at_b, 1.0, true, at, n) {
            return 0;
        }
        arm_l = max(arm_l, wz);
        code |= RAN_ALL;
    }
    let mut ts_out = ts;
    if ts >= 0.0 {
        // the toss: the support hand off the gun, down beside the hip, through and up, back to the gun (a path off the shoulder; its ends are the live hand)
        if !ask_blend(g, at, at, 1.0, false, at_b, n) {
            return 0;
        }
        let hh = &mut h;
        with_pose(g, at_b, |d, rig, _n, fk| {
            fk_run(rig, d, fk.rot, fk.pos);
            let sh = fk.at(up_l);
            let (cur, rot0) = (sub3(fk.at(hand_l), sh), fk.rot(hand_l));
            // (a quick release winds up less)
            let wk = clamp(rel / 0.3, 0.5, 1.0);
            let back = lerp3(&cur, &[0.07, -0.47, -0.1], wk);
            // (on the way down the hand swings out past the pouches on his front, not through them)
            let mut mid = lerp3(&cur, &back, 0.5);
            mid[0] += 0.08 * wk;
            mid[2] += 0.06 * wk;
            let off = curve3(
                ts,
                &[
                    (0.0, cur),
                    (rel * 0.36, mid),
                    (rel * 0.72, back),
                    (rel, [0.04, -0.43, 0.15]),
                    (rel + 0.1, [-0.02, -0.2, 0.45]),
                    (rel + 0.22, [-0.04, 0.0, 0.5]),
                    (rel + 0.4, [-0.03, -0.05, 0.46]),
                    (td, cur),
                ],
            );
            let fing = curve3(ts, &[(0.0, [0.0, -0.7, 0.7]), (rel * 0.72, [0.0, -1.0, 0.1]), (rel, [0.0, -0.8, 0.6]), (rel + 0.1, [0.0, -0.15, 1.0]), (rel + 0.22, [-0.05, 0.3, 1.0]), (td, [0.0, -0.2, 1.0])]);
            let bk = curve3(ts, &[(0.0, [0.5, -0.6, -0.5]), (rel, [0.3, -0.5, -0.8]), (rel + 0.12, [0.25, -1.0, -0.2]), (td, [0.3, -1.0, 0.0])]);
            let r = hand_rot(&fix_l, fing, bk);
            two_bone_ik(rig, d, fk.rot, fk.pos, up_l, fore_l, hand_l, &add3(sh, off), Some(&[0.7, -0.45, -0.55]), 0.8, Some(&r), false);
            let (open, recup) = (smoothstep(rel - 0.02, rel + 0.04, ts), smoothstep(rel + 0.35, td, ts));
            let c = |grip: f64, flat: f64, cup: f64| lerp(lerp(grip, flat, open), cup, recup);
            hand(d, hh, 0, &[c(0.6, 0.05, 0.5), c(0.65, 0.08, 0.55), c(0.7, 0.1, 0.6), c(0.78, 0.14, 0.68)], c(0.7, 0.1, 0.35), 0.25 * open * (1.0 - recup));
            // the wrist's turn eases from and back to the grip it had on the gun at the ends (the path does the same for its place)
            wrist_ease(d, rig, fk, hand_l, &rot0, bump(ts, 0.0, 0.08, td - 0.22, td - 0.04));
        });
        let hold = bump(ts, 0.0, 0.05, td - 0.2, td);
        arm_l = max(arm_l, hold);
        ask_mask(&[(sh_l, hold * gate, true), (sh_l, 0.0, false)]);
        if !ask_blend(g, at, at_b, 1.0, true, at, n) {
            return 0;
        }
        ts_out = ts + dt;
        if ts_out > td {
            ts_out = -1.0;
        }
        code |= RAN_ALL;
    }
    let b = bench();
    b.args[L_IN] = ts_out;
    b.args[L_IN + 1] = arm_l;
    hands_out(&h);
    code
}

/// three knobs in a row as a vector
fn kv(i: &[usize; 3]) -> [f64; 3] {
    [k(i[0]), k(i[1]), k(i[2])]
}

/// CharacterAnimator.watchArm — the slip's arm: the one on the man's side goes out low toward him, the hand open. Solved on the scratch pose at `at_b` and
/// laid back through that shoulder's mask. args: 0, 1 what nothing else has of the left and the right arm · 2 the slip's weight · 3 its lean · 4 its side
/// (0 the left) · 5, 6 the line to the man · 7‥10, 11‥14 the hands' fixes. Columns: the hand pass's.
pub fn watch_arm(g: usize, at: usize, at_b: usize, at_rest: usize) -> u32 {
    let mut a = [0.0f64; 15];
    a.copy_from_slice(&bench().args[..15]);
    let (wl, wr, sw, sl, left, sx, sz) = (a[0], a[1], a[2], a[3], a[4] == 0.0, a[5], a[6]);
    let Some(Some(n)) = with_pose(g, at, |_d, _rig, n, _fk| if cast(n, &ARM_ROLES) { Some(n) } else { None }) else {
        return 0;
    };
    if !(sw > 0.003) {
        return RAN;
    }
    let kl = sw * sw * (3.0 - 2.0 * sw) * sl;
    let sgn = if left { 1.0 } else { -1.0 };
    let w_arm = (if left { wl } else { wr }) * kl * k(WATCH_ARM_W) * smoothstep(k(WATCH_ARM_SIDE_0), k(WATCH_ARM_SIDE_1), sgn * sx);
    if !(w_arm > 0.003) {
        return RAN;
    }
    let Some(mut h) = hands_in(g, at_rest) else {
        return 0;
    };
    if !ask_blend(g, at, at, 1.0, false, at_b, n) {
        return 0;
    }
    let (side, fix) = if left { (0, [a[7], a[8], a[9], a[10]]) } else { (1, [a[11], a[12], a[13], a[14]]) };
    let (up, fore, hand_n, sh_n) = if left { (role(UPPER_ARM_L), role(FOREARM_L), role(HAND_L), role(SHOULDER_L)) } else { (role(UPPER_ARM_R), role(FOREARM_R), role(HAND_R), role(SHOULDER_R)) };
    let hh = &mut h;
    with_pose(g, at_b, |d, rig, _n, fk| {
        fk_run(rig, d, fk.rot, fk.pos);
        let sh = fk.at(up);
        let to = [sh[0] + sx * k(WATCH_ARM_OUT), sh[1] - k(WATCH_ARM_DOWN), sh[2] + sz * k(WATCH_ARM_OUT)];
        let r = hand_rot(&fix, normalize([sx * 0.75, -0.55, sz * 0.75]), [-sx * 0.3, 1.0, -sz * 0.3]);
        two_bone_ik(rig, d, fk.rot, fk.pos, up, fore, hand_n, &to, Some(&[sgn * 0.5, -0.3, -0.8]), 0.6, Some(&r), false);
        hand(d, hh, side, &[0.12, 0.15, 0.18, 0.22], 0.15, 0.35);
    });
    ask_mask(&[(sh_n, w_arm, true)]);
    if !ask_blend(g, at, at_b, 1.0, true, at, n) {
        return 0;
    }
    hands_out(&h);
    RAN | RAN_ALL
}

/// CharacterAnimator.knifeRot: the right hand's turn from where the blade is to point and which way the back of the hand faces (squared to the blade here)
fn knife_rot(fix_r: &[f64; 4], blade: [f64; 3], back: [f64; 3]) -> [f64; 4] {
    let t = normalize(blade);
    let b = normalize(mad3(back, t, -dot3(&back, &t)));
    hand_rot(fix_r, cross3(&t, &b), b)
}
/// CharacterAnimator.sheathFist: where the right wrist goes, the fist's turn and the elbow's pole with the fist closed on the sheathed handle, in the chest
/// joint's frame off the rest pose's (`rc_inv`: the rest chest's turn, inverted). `pull` m back down the knife's line, `off` m further off the chest.
fn sheath_fist(fk: &Fk, rc_inv: &[f64; 4], fix_r: &[f64; 4], crouch: f64, pull: f64, off: f64) -> ([f64; 3], [f64; 4], [f64; 3]) {
    let s3 = role(SPINE3);
    let q = q_mul(&fk.rot(s3), rc_inv);
    let t = normalize(q_rotate(&q, &kv(&[SHEATH_ALONG_0, SHEATH_ALONG_1, SHEATH_ALONG_2])));
    let b0 = q_rotate(&q, &kv(&[SHEATH_OUT_0, SHEATH_OUT_1, SHEATH_OUT_2]));
    let b = normalize(mad3(b0, t, -dot3(&b0, &t)));
    let f = cross3(&t, &b);
    let h = mad3(mad3(add3(fk.at(s3), q_rotate(&q, &kv(&[SHEATH_AT_0, SHEATH_AT_1, SHEATH_AT_2]))), t, -pull), b, off);
    let to = mad3(mad3(mad3(h, f, -k(KNIFE_GRIP_FWD)), b, k(KNIFE_GRIP_PALM)), t, -k(KNIFE_GRIP_SIDE));
    let pole = mad3(normalize(q_rotate(&q, &kv(&[REACH_POLE_0, REACH_POLE_1, REACH_POLE_2]))), AY, lerp(k(REACH_POLE_UP_0), k(REACH_POLE_UP_1), clamp(crouch, 0.0, 1.0)));
    (to, knife_rot(fix_r, t, b), pole)
}

/// CREEP.L / CREEP.R of character.ts: a hand of the close-in carriage, by where its numbers are among the knobs
struct CreepHand {
    at: [usize; 3],
    at_c: [usize; 3],
    pole: [usize; 3],
    fing: [usize; 3],
    back: [usize; 3],
    curl: usize,
    thumb: usize,
    spread: usize,
}
const CREEP_L: CreepHand = CreepHand {
    at: [CREEP_L_AT_0, CREEP_L_AT_1, CREEP_L_AT_2],
    at_c: [CREEP_L_AT_C_0, CREEP_L_AT_C_1, CREEP_L_AT_C_2],
    pole: [CREEP_L_POLE_0, CREEP_L_POLE_1, CREEP_L_POLE_2],
    fing: [CREEP_L_FING_0, CREEP_L_FING_1, CREEP_L_FING_2],
    back: [CREEP_L_BACK_0, CREEP_L_BACK_1, CREEP_L_BACK_2],
    curl: CREEP_L_CURL,
    thumb: CREEP_L_THUMB,
    spread: CREEP_L_SPREAD,
};
const CREEP_R: CreepHand = CreepHand {
    at: [CREEP_R_AT_0, CREEP_R_AT_1, CREEP_R_AT_2],
    at_c: [CREEP_R_AT_C_0, CREEP_R_AT_C_1, CREEP_R_AT_C_2],
    pole: [CREEP_R_POLE_0, CREEP_R_POLE_1, CREEP_R_POLE_2],
    fing: [CREEP_R_FING_0, CREEP_R_FING_1, CREEP_R_FING_2],
    back: [CREEP_R_BACK_0, CREEP_R_BACK_1, CREEP_R_BACK_2],
    curl: CREEP_R_CURL,
    thumb: CREEP_R_THUMB,
    spread: CREEP_R_SPREAD,
};

/// CharacterAnimator.creepArms — the close-in carriage's arms, after its body: the shoulders rounded and the hands brought up ahead of the hips in the
/// shoulders' frame, let out wide and low by `ws`, alive with a slow working of the fingers when the distance is dealt, and the knife hand taken up across
/// the chest to hover off the hilt as he closes. Solved on the scratch pose at `at_b`, laid back joint by joint down each arm. args: 0 w · 1, 2 what the
/// layer has of the left and the right arm · 3 the eased weight · 4 the crouch, clamped · 5 how close (−1: not dealt) · 6 that, eased · 7 wide · 8 the arms'
/// share of his stride · 9 his surge · 10 the layer's clock · 11 a thing in the right fist · 12 no knife · 13 the crouch · 14 the stride's swing m ·
/// 15‥18 the wide shape · 19 a wide pole fed · 20‥22 it · 23‥26, 27‥30 the hands' fixes · 31‥34 the rest chest's turn, inverted. Columns: the hand pass's.
pub fn creep_arms(g: usize, at: usize, at_b: usize, at_rest: usize) -> u32 {
    let mut a = [0.0f64; 35];
    a.copy_from_slice(&bench().args[..35]);
    let (w, wl, wr, e, cr, ck, cke, ws, ha, hg, cl_t) = (a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7], a[8], a[9], a[10]);
    let (held, no_knife, crouch, swing) = (a[11] != 0.0, a[12] != 0.0, a[13], a[14]);
    let wsh = [a[15], a[16], a[17], a[18]];
    let wide_pole = if a[19] != 0.0 { [a[20], a[21], a[22]] } else { kv(&[CREEP_WIDE_POLE_0, CREEP_WIDE_POLE_1, CREEP_WIDE_POLE_2]) };
    let (fix_l, fix_r, rc_inv) = ([a[23], a[24], a[25], a[26]], [a[27], a[28], a[29], a[30]], [a[31], a[32], a[33], a[34]]);
    let Some(Some(n)) = with_pose(g, at, |_d, _rig, n, _fk| if cast(n, &ARM_ROLES) { Some(n) } else { None }) else {
        return 0;
    };
    let Some(mut h) = hands_in(g, at_rest) else {
        return 0;
    };
    if !ask_blend(g, at, at, 1.0, false, at_b, n) {
        return 0;
    }
    let (on_l, on_r) = (wl > 0.003, wr > 0.003);
    let (w_up, w_fore, w_hand) = (smoothstep(0.0, 0.75, w), smoothstep(0.08, 0.88, w), smoothstep(0.16, 1.0, w));
    let dealt = ck >= 0.0;
    let lw = wl * if dealt { lerp(k(CLOSE_LEFT), 1.0, ws) } else { 1.0 };
    let hh = &mut h;
    with_pose(g, at_b, |d, rig, _n, fk| {
        let rnd = k(CREEP_ROUND) * DEGR * e;
        if on_l {
            turn(d, rig, role(SHOULDER_L), q_yaw(-rnd));
        }
        if on_r {
            turn(d, rig, role(SHOULDER_R), q_yaw(rnd));
        }
        fk_run(rig, d, fk.rot, fk.pos);
        let (sl, sr) = (fk.at(role(UPPER_ARM_L)), fk.at(role(UPPER_ARM_R)));
        let (mut fx, mut fz) = (-(sl[2] - sr[2]), sl[0] - sr[0]);
        {
            let l = or_one(hypot2(fx, fz));
            fx /= l;
            fz /= l;
        }
        // (left, up, forward) off the shoulder line, as model axes
        let sq = |v: [f64; 3]| -> [f64; 3] { [fz * v[0] + fx * v[2], v[1], fz * v[2] - fx * v[0]] };
        // the hands alive: a slow working of the fingers and a drift of the wrists on the layer's own clock (two rates a hand, never in step)
        let lt = cl_t * PI * 2.0 * k(CLOSE_FLEX);
        let fl_l = if dealt { k(CLOSE_FLEX_AMP) * (0.6 * sin(lt) + 0.4 * sin(lt * 1.63 + 1.1)) } else { 0.0 };
        let fl_r = if dealt { k(CLOSE_FLEX_AMP) * (0.6 * sin(lt * 1.21 + 2.3) + 0.4 * sin(lt * 0.71 + 0.4)) } else { 0.0 };
        let drift = |ph: f64| -> [f64; 3] {
            if dealt {
                [k(CLOSE_DRIFT) * sin(lt * 0.83 + ph), k(CLOSE_DRIFT) * sin(lt * 1.17 + ph * 1.7), k(CLOSE_DRIFT) * 0.6 * sin(lt * 0.61 + ph * 0.6)]
            } else {
                [0.0, 0.0, 0.0]
            }
        };
        let (wd_fing, wd_back) = (kv(&[CREEP_WIDE_FING_0, CREEP_WIDE_FING_1, CREEP_WIDE_FING_2]), kv(&[CREEP_WIDE_BACK_0, CREEP_WIDE_BACK_1, CREEP_WIDE_BACK_2]));
        if on_l {
            let hd = &CREEP_L;
            let (mut pole, mut fing, mut back, mut cu, mut th, mut sp) = (kv(&hd.pole), kv(&hd.fing), kv(&hd.back), k(hd.curl), k(hd.thumb), k(hd.spread));
            // (the drift at its full size through the part of this arm the layer has)
            let mut to = add3(
                mad3(lerp3(&kv(&hd.at), &kv(&hd.at_c), cr), kv(&[CLOSE_LEAD_0, CLOSE_LEAD_1, CLOSE_LEAD_2]), cke),
                scale3(drift(0.0), if dealt { 1.0 / max(0.38, lerp(k(CLOSE_LEFT), 1.0, ws)) } else { 1.0 }),
            );
            if ws > 0.0 {
                // out to its side, and still
                let s = -ha * swing;
                to = lerp3(&to, &[wsh[0] + s * k(HEFT_OUT), -wsh[1] + abs(s) * k(HEFT_UP), wsh[2] + wsh[3] + s - hg * 0.1], ws);
                pole = lerp3(&pole, &wide_pole, ws);
                fing = lerp3(&fing, &wd_fing, ws);
                back = lerp3(&back, &wd_back, ws);
                cu = lerp(cu, k(CREEP_WIDE_CURL), ws);
                th = lerp(th, k(CREEP_WIDE_THUMB), ws);
                sp = lerp(sp, k(CREEP_WIDE_SPREAD), ws);
            }
            let r = hand_rot(&fix_l, sq(fing), sq(back));
            two_bone_ik(rig, d, fk.rot, fk.pos, role(UPPER_ARM_L), role(FOREARM_L), role(HAND_L), &add3(sl, sq(to)), Some(&sq(pole)), 0.8, Some(&r), false);
            hand(d, hh, 0, &[cu + fl_l, cu + 0.05 + fl_l * 0.9, cu + 0.1 + fl_l * 0.8, cu + 0.15 + fl_l * 0.7], th + fl_l * 0.5, sp);
        }
        if on_r {
            let hd = &CREEP_R;
            let mir = |v: [f64; 3]| -> [f64; 3] { [-v[0], v[1], v[2]] };
            let (mut a0, mut p0, mut fing, mut back) = (add3(lerp3(&kv(&hd.at), &kv(&hd.at_c), cr), drift(2.1)), kv(&hd.pole), kv(&hd.fing), kv(&hd.back));
            let (mut curl, mut thb, mut spr) = (k(hd.curl), k(hd.thumb), k(hd.spread) * (1.0 - cke));
            if ws > 0.0 {
                // (the left's mirror)
                let s = ha * swing;
                a0 = lerp3(&a0, &[-wsh[0] - s * k(HEFT_OUT), -wsh[1] + abs(s) * k(HEFT_UP), wsh[2] - wsh[3] + s - hg * 0.1], ws);
                p0 = lerp3(&p0, &mir(wide_pole), ws);
                fing = lerp3(&fing, &mir(wd_fing), ws);
                back = lerp3(&back, &mir(wd_back), ws);
                curl = lerp(curl, k(CREEP_WIDE_CURL), ws);
                thb = lerp(thb, k(CREEP_WIDE_THUMB), ws);
                spr = lerp(spr, k(CREEP_WIDE_SPREAD), ws);
            }
            let (mut to, mut rot, mut pole) = (add3(sr, sq(a0)), hand_rot(&fix_r, sq(fing), sq(back)), sq(p0));
            if dealt && !held && !no_knife {
                // the knife hand by distance: from its guard up across the chest to hover off the hilt
                let (s_at, s_rot, s_pole) = sheath_fist(fk, &rc_inv, &fix_r, crouch, 0.0, k(CLOSE_HOVER));
                // (the travel half-eased: the hand must be seen to go with every step)
                let cp = cke + (ck - cke) * 0.5;
                to = add3(mad3(lerp3(&to, &s_at, cp), sq(kv(&[CLOSE_BOW_0, CLOSE_BOW_1, CLOSE_BOW_2])), sin(PI * cke)), scale3(sq(drift(2.1)), cke));
                rot = slerp(&rot, &s_rot, cke);
                pole = lerp3(&pole, &s_pole, cke);
                curl = lerp(k(hd.curl), k(CLOSE_CURL), cke);
            }
            two_bone_ik(rig, d, fk.rot, fk.pos, role(UPPER_ARM_R), role(FOREARM_R), role(HAND_R), &to, Some(&pole), 0.8, Some(&rot), false);
            if !held {
                hand(d, hh, 1, &[curl + fl_r, curl + 0.05 + fl_r * 0.9, curl + 0.1 + fl_r * 0.8, curl + 0.15 + fl_r * 0.7], thb + fl_r * 0.5, spr);
            }
        }
    });
    // (an arm the layer has none of is left out of the mask: a node no rig has is skipped)
    let (l, r) = (|i: usize| if on_l { role(i) } else { usize::MAX }, |i: usize| if on_r { role(i) } else { usize::MAX });
    ask_mask(&[
        (l(SHOULDER_L), w_up * lw, false),
        (l(UPPER_ARM_L), w_up * lw, false),
        (l(FOREARM_L), w_fore * lw, false),
        (l(HAND_L), w_hand * lw, true),
        (r(SHOULDER_R), w_up * wr, false),
        (r(UPPER_ARM_R), w_up * wr, false),
        (r(FOREARM_R), w_fore * wr, false),
        (r(HAND_R), w_hand * wr, true),
    ]);
    if !ask_blend(g, at, at_b, 1.0, true, at, n) {
        return 0;
    }
    hands_out(&h);
    RAN | RAN_ALL
}

/// the support arm from the upper arm down (the collar bone stays the clip's), or both arms (character.ts armMask)
fn arm_mask(both: bool) {
    ask_mask(&[(role(UPPER_ARM_L), 1.0, true), (if both { role(UPPER_ARM_R) } else { usize::MAX }, 1.0, true)]);
}

/// CharacterAnimator.clutchArm — a man knocked about and still up: the support hand off the gun and pressed to his ribs. Solved on the scratch pose at
/// `at_b`, laid on the pose by w through the support arm's mask. args: 0 w · 1‥4 the left hand's fix. Columns: the hand pass's.
pub fn clutch_arm(g: usize, at: usize, at_b: usize, at_rest: usize) -> u32 {
    let mut a = [0.0f64; 5];
    a.copy_from_slice(&bench().args[..5]);
    let (w, fix_l) = (a[0], [a[1], a[2], a[3], a[4]]);
    let Some(Some(n)) = with_pose(g, at, |_d, _rig, n, _fk| if cast(n, &ARM_ROLES) && cast(n, &[HIPS, SPINE1]) { Some(n) } else { None }) else {
        return 0;
    };
    let Some(mut h) = hands_in(g, at_rest) else {
        return 0;
    };
    if !ask_blend(g, at, at, 1.0, false, at_b, n) {
        return 0;
    }
    let hh = &mut h;
    with_pose(g, at_b, |d, rig, _n, fk| {
        fk_run(rig, d, fk.rot, fk.pos);
        let (left, up, fwd) = trunk_frame(fk);
        let p = mad3(mad3(mad3(fk.at(role(SPINE1)), left, k(CLUTCH_AT_0)), up, k(CLUTCH_AT_1)), fwd, k(CLUTCH_AT_2));
        let r = hand_rot(&fix_l, mad3(scale3(left, -1.0), up, -0.25), mad3(fwd, up, 0.2));
        two_bone_ik(rig, d, fk.rot, fk.pos, role(UPPER_ARM_L), role(FOREARM_L), role(HAND_L), &p, Some(&kv(&[CLUTCH_POLE_0, CLUTCH_POLE_1, CLUTCH_POLE_2])), 0.7, Some(&r), false);
        hand(d, hh, 0, &[0.3, 0.34, 0.38, 0.42], 0.3, 0.2);
    });
    arm_mask(false);
    if !ask_blend(g, at, at_b, w, true, at, n) {
        return 0;
    }
    hands_out(&h);
    RAN | RAN_ALL
}

/// CharacterAnimator.boundArms — kept down: the wrists crossed in the small of the back, palms out (kind 1), or the fingers laced on the crown with the
/// elbows wide, in the head's own frame (kind 2). Both arms, solved on the scratch pose at `at_b` and laid on the pose by w. args: 0 w · 1 the kind ·
/// 2‥4 the gaze in the head's frame · 5‥8, 9‥12 the hands' fixes. Columns: the hand pass's.
pub fn bound_arms(g: usize, at: usize, at_b: usize, at_rest: usize) -> u32 {
    let mut a = [0.0f64; 13];
    a.copy_from_slice(&bench().args[..13]);
    let (w, back, gaze) = (a[0], a[1] == 1.0, [a[2], a[3], a[4]]);
    let (fix_l, fix_r) = ([a[5], a[6], a[7], a[8]], [a[9], a[10], a[11], a[12]]);
    let Some(Some(n)) = with_pose(g, at, |_d, _rig, n, _fk| if cast(n, &ARM_ROLES) && cast(n, &[HIPS, HEAD]) { Some(n) } else { None }) else {
        return 0;
    };
    let Some(mut h) = hands_in(g, at_rest) else {
        return 0;
    };
    if !ask_blend(g, at, at, 1.0, false, at_b, n) {
        return 0;
    }
    let hh = &mut h;
    with_pose(g, at_b, |d, rig, _n, fk| {
        fk_run(rig, d, fk.rot, fk.pos);
        let (left, up, fwd) = trunk_frame(fk);
        let tk = |o: [f64; 3], x: f64, y: f64, z: f64| mad3(mad3(mad3(o, left, x), up, y), fwd, z);
        let tv = |x: f64, y: f64, z: f64| tk([0.0, 0.0, 0.0], x, y, z);
        let (up_l, fore_l, hand_l, up_r, fore_r, hand_r) = (role(UPPER_ARM_L), role(FOREARM_L), role(HAND_L), role(UPPER_ARM_R), role(FOREARM_R), role(HAND_R));
        if back {
            // wrists crossed in the small of the back, palms out, elbows back and a little out
            let o = fk.at(role(HIPS));
            let (b_up, b_out, b_behind) = (k(BOUND_BACK_UP), k(BOUND_BACK_OUT), k(BOUND_BACK_BEHIND));
            let (to, r) = (tk(o, -b_out * 0.4, b_up, -b_behind), hand_rot(&fix_l, tv(-0.75, -0.6, -0.1), tv(0.0, 0.0, 1.0)));
            two_bone_ik(rig, d, fk.rot, fk.pos, up_l, fore_l, hand_l, &to, Some(&tv(0.8, 0.1, -0.6)), 0.8, Some(&r), false);
            let (to, r) = (tk(o, b_out * 0.4, b_up + 0.02, -b_behind - 0.02), hand_rot(&fix_r, tv(0.75, -0.6, -0.1), tv(0.0, 0.0, 1.0)));
            two_bone_ik(rig, d, fk.rot, fk.pos, up_r, fore_r, hand_r, &to, Some(&tv(-0.8, 0.1, -0.6)), 0.8, Some(&r), false);
            // a loose half fist each
            let curl = [k(BOUND_BACK_CURL_0), k(BOUND_BACK_CURL_1), k(BOUND_BACK_CURL_2), k(BOUND_BACK_CURL_3)];
            hand(d, hh, 0, &curl, k(BOUND_BACK_THUMB), 0.0);
            hand(d, hh, 1, &curl, k(BOUND_BACK_THUMB), 0.0);
        } else {
            // fingers laced on the crown, elbows wide: in the head's own frame, up its crown and forward along its gaze
            let hr = fk.rot(role(HEAD));
            let hu = q_rotate(&hr, &AY);
            let hf0 = q_rotate(&hr, &gaze);
            let hf = normalize(mad3(hf0, hu, -dot3(&hf0, &hu)));
            let hl = cross3(&hu, &hf);
            let o = mad3(mad3(fk.at(role(HEAD)), hu, k(BOUND_HEAD_UP)), hf, -k(BOUND_HEAD_BEHIND));
            let hk = |x: f64, y: f64, z: f64| mad3(mad3(scale3(hl, x), hu, y), hf, z);
            let (out, elbow) = (k(BOUND_HEAD_OUT), k(BOUND_HEAD_ELBOW));
            let r = hand_rot(&fix_l, hk(-0.84, 0.5, 0.04), hk(0.2, 0.75, -0.6));
            two_bone_ik(rig, d, fk.rot, fk.pos, up_l, fore_l, hand_l, &mad3(o, hl, out), Some(&tv(1.0, -0.15, elbow)), 0.9, Some(&r), false);
            let r = hand_rot(&fix_r, hk(0.84, 0.5, 0.04), hk(-0.2, 0.75, -0.6));
            two_bone_ik(rig, d, fk.rot, fk.pos, up_r, fore_r, hand_r, &mad3(o, hl, -out), Some(&tv(-1.0, -0.15, elbow)), 0.9, Some(&r), false);
            // open hands, the fingers a little apart, and bent down at the knuckles onto the crown (an angle a side)
            let curl = [k(BOUND_HEAD_CURL_0), k(BOUND_HEAD_CURL_1), k(BOUND_HEAD_CURL_2), k(BOUND_HEAD_CURL_3)];
            hand(d, hh, 0, &curl, k(BOUND_HEAD_THUMB), 0.25);
            hand(d, hh, 1, &curl, k(BOUND_HEAD_THUMB), 0.25);
            for (sd, kn) in [(0, BOUND_HEAD_KNUCKLE_0), (1, BOUND_HEAD_KNUCKLE_1)] {
                let ns = digits(sd);
                for fi in 0..4 {
                    let i = ns[3 + fi * 3] as usize;
                    set_unit(d, i, &q_mul(&own_rot(d, i), &axis_angle(&AX, k(kn) * DEGR)));
                }
            }
        }
    });
    arm_mask(true);
    if !ask_blend(g, at, at_b, w, true, at, n) {
        return 0;
    }
    hands_out(&h);
    RAN | RAN_ALL
}

/// where a layer is handed a path of keys (after the hand pass's columns), seven doubles a key, and how many keys it may be
const KEYS_AT: usize = 264;
const KEY: usize = 7;
const KEYS_MAX: usize = 48;
/// a keyed path being put together: (time, vector) knots for curve3
struct Path {
    n: usize,
    k: [(f64, [f64; 3]); KEYS_MAX + 3],
}
impl Path {
    fn new() -> Path {
        Path { n: 0, k: [(0.0, [0.0; 3]); KEYS_MAX + 3] }
    }
    fn of(keys: &[(f64, [f64; 3])]) -> Path {
        let mut p = Path::new();
        for &(t, v) in keys {
            p.push(t, v);
        }
        p
    }
    fn push(&mut self, t: f64, v: [f64; 3]) {
        self.k[self.n] = (t, v);
        self.n += 1;
    }
    fn at(&self, t: f64) -> [f64; 3] {
        curve3(t, &self.k[..self.n])
    }
}

/// the gun's frame where the trigger hand has it now, off the run-forward bones: the wrist, the bore (wrist to the middle knuckle), up out of the receiver
/// and across to his left. The support hand works on the weapon as drawn.
struct GunFrame {
    wrist: [f64; 3],
    bore: [f64; 3],
    up: [f64; 3],
    left: [f64; 3],
}
impl GunFrame {
    fn of(fk: &Fk) -> GunFrame {
        let wrist = fk.at(role(HAND_R));
        let bore = normalize(sub3(fk.at(role(MIDDLE1_R)), wrist));
        let y = mad3(AY, bore, -bore[1]);
        let up = if len3(&y) > 1e-4 { normalize(y) } else { [0.0, 0.0, -1.0] };
        GunFrame { wrist, bore, up, left: cross3(&up, &bore) }
    }
    fn dir(&self, p: &[f64; 3]) -> [f64; 3] {
        let (x, y, d) = (&self.left, &self.up, &self.bore);
        [x[0] * p[0] + y[0] * p[1] + d[0] * p[2], x[1] * p[0] + y[1] * p[1] + d[1] * p[2], x[2] * p[0] + y[2] * p[1] + d[2] * p[2]]
    }
    fn at(&self, p: &[f64; 3]) -> [f64; 3] {
        add3(self.wrist, self.dir(p))
    }
}
/// a reload's beat as the animator keeps it (rlB), at args 3‥9: its length, the magazine out, the fresh one in, the handle run, whether it is run at all,
/// a magazine kept (−1: dropped) and when the hand is at the fresh ones
struct Beat {
    dur: f64,
    out: f64,
    in_: f64,
    rack: f64,
    pull: bool,
    keep: f64,
    fresh: f64,
}
fn beat(a: &[f64]) -> Beat {
    Beat { dur: a[3], out: a[4], in_: a[5], rack: a[6], pull: a[7] != 0.0, keep: a[8], fresh: a[9] }
}

/// CharacterAnimator.reloadArm (and its feedArm) — the long gun's magazine change, the support hand's: to the well, the empty stripped, a fresh one
/// fetched and seated, the handle run, back to the fore-end; an untrained man's lands on the lip, is backed off and found, and is slapped home. Or, for a
/// gun fed its own way, the hand through the stations it is handed. Solved on the scratch pose at `at_b` in the gun's frame as the trigger hand has it, laid
/// on the pose by w through the support arm's mask. args: 0 t · 1 w · 2 trained · 3‥9 the beat · 10 the hip's pouches by the hips' own line · 11‥14 the
/// left hand's fix · 15 how many stations (−1: the magazine change) · 16‥33 the gun's marks (the well, a hand's width under it, the handle, the handle
/// back, the fingers' and the back of the hand's lines on the handle); from 264 the stations, seven a key: its time · at the pouch · its point · how the
/// hand is on it (0 round a thing, 1 flat on top, 2 into the pouch) · its grip. Columns: the hand pass's.
pub fn reload_arm(g: usize, at: usize, at_b: usize, at_rest: usize) -> u32 {
    let mut a = [0.0f64; 34];
    a.copy_from_slice(&bench().args[..34]);
    let (t, w, tr, hip_line) = (a[0], a[1], a[2], a[10] != 0.0);
    let (bt, fix_l) = (beat(&a), [a[11], a[12], a[13], a[14]]);
    let fed = a[15];
    let v = |i: usize| [a[i], a[i + 1], a[i + 2]];
    let (m_mag, m_strip, m_handle, m_pull, m_hf, m_hb) = (v(16), v(19), v(22), v(25), v(28), v(31));
    let stations = if fed >= 0.0 { fed as usize } else { 0 };
    if stations > KEYS_MAX {
        return 0;
    }
    let mut keys = [[0.0f64; KEY]; KEYS_MAX];
    for (i, key) in keys.iter_mut().enumerate().take(stations) {
        key.copy_from_slice(&bench().args[KEYS_AT + i * KEY..KEYS_AT + (i + 1) * KEY]);
    }
    let Some(Some(n)) = with_pose(g, at, |_d, _rig, n, _fk| if cast(n, &ARM_ROLES) && cast(n, &[MIDDLE1_R, HIPS, SPINE2, THIGH_L, THIGH_R]) { Some(n) } else { None }) else {
        return 0;
    };
    let Some(mut h) = hands_in(g, at_rest) else {
        return 0;
    };
    if !ask_blend(g, at, at, 1.0, false, at_b, n) {
        return 0;
    }
    let hh = &mut h;
    let posed = with_pose(g, at_b, |d, rig, _n, fk| {
        fk_run(rig, d, fk.rot, fk.pos);
        let gun = GunFrame::of(fk);
        let hand_l = role(HAND_L);
        let (cur, rot0) = (fk.at(hand_l), fk.rot(hand_l));
        let (left, up, fwd) = trunk_frame(fk);
        let tk = |o: [f64; 3], p: [f64; 3]| mad3(mad3(mad3(o, left, p[0]), up, p[1]), fwd, p[2]);
        // the hip's pouches: off the pelvis in the trunk's frame, or by the hips' own line where a bladed chest would carry them round with it
        let hips = fk.at(role(HIPS));
        let (hl, hf) = if hip_line {
            let l = sub3(fk.at(role(THIGH_L)), fk.at(role(THIGH_R)));
            let hl = normalize(mad3(l, up, -dot3(&l, &up)));
            (hl, cross3(&hl, &up))
        } else {
            (left, fwd)
        };
        let hk = |p: [f64; 3]| mad3(mad3(mad3(hips, hl, p[0]), up, p[1]), hf, p[2]);
        let pouch = if tr > 0.5 { tk(fk.at(role(SPINE2)), kv(&[RELOAD_POUCH_CHEST_0, RELOAD_POUCH_CHEST_1, RELOAD_POUCH_CHEST_2])) } else { hk(kv(&[RELOAD_POUCH_HIP_0, RELOAD_POUCH_HIP_1, RELOAD_POUCH_HIP_2])) };
        let (r_pole, pole_w) = (kv(&[RELOAD_POLE_0, RELOAD_POLE_1, RELOAD_POLE_2]), k(RELOAD_POLE_W));
        let (up_l, fore_l) = (role(UPPER_ARM_L), role(FOREARM_L));
        let (o, i, r, dur) = (bt.out, bt.in_, bt.rack, bt.dur);
        // the hand: round a thing from its front (fingers across it, palm toward him) · fingers down into the pouch
        let (mf, mb) = (gun.dir(&[-1.0, 0.15, 0.1]), gun.dir(&[0.1, -0.35, 1.0]));
        let (pf, pb) = (mad3(mad3(scale3(up, -0.9), left, -0.3), fwd, 0.1), mad3(fwd, left, 0.3));
        if fed >= 0.0 {
            // a gun fed its own way: the hand through its stations, and back to where it was
            if stations < 2 {
                return false;
            }
            // …or flat on top of the gun (fingers across it, the back of the hand up)
            let (tf, tb) = (gun.dir(&[-1.0, 0.0, 0.15]), gun.dir(&[0.1, 1.0, 0.1]));
            let (mut p, mut f, mut b) = (Path::of(&[(0.0, cur)]), Path::new(), Path::new());
            for key in keys.iter().take(stations) {
                let how = key[5];
                p.push(key[0], if key[1] != 0.0 { pouch } else { gun.at(&[key[2], key[3], key[4]]) });
                f.push(key[0], if how == 1.0 { tf } else if how == 2.0 { pf } else { mf });
                b.push(key[0], if how == 1.0 { tb } else if how == 2.0 { pb } else { mb });
            }
            // (the hand rests where its last work was until the weapon is nearly done, then back to the gun)
            let end = keys[stations - 1][0];
            if end + 0.1 < dur - 0.3 {
                let last = p.k[p.n - 1].1;
                p.push(dur - 0.3, last);
            }
            p.push(max(dur - 0.03, end + 0.02), cur);
            let rot = hand_rot(&fix_l, f.at(t), b.at(t));
            two_bone_ik(rig, d, fk.rot, fk.pos, up_l, fore_l, hand_l, &p.at(t), Some(&r_pole), pole_w, Some(&rot), false);
            let (first, last) = (&keys[0], &keys[stations - 1]);
            let mut gr = if t <= first[0] { first[6] * smoothstep(first[0] - 0.12, first[0], t) } else { last[6] * (1.0 - smoothstep(end, end + 0.12, t)) };
            for j in 0..stations - 1 {
                let (ka, kb) = (&keys[j], &keys[j + 1]);
                if t > ka[0] && t <= kb[0] {
                    gr = lerp(ka[6], kb[6], (t - ka[0]) / (kb[0] - ka[0]));
                    break;
                }
            }
            let c = |kk: f64| lerp(0.3, kk, gr);
            hand(d, hh, 0, &[c(0.62), c(0.68), c(0.72), c(0.78)], lerp(0.3, 0.6, gr), 0.1 * (1.0 - gr));
            wrist_ease(d, rig, fk, hand_l, &rot0, bump(t, 0.0, 0.12, dur - 0.26, dur - 0.05));
            return true;
        }
        let half = (o + i) * 0.5;
        let (mag, strip, hd, pl) = (gun.at(&m_mag), gun.at(&m_strip), gun.at(&m_handle), gun.at(&m_pull));
        // the empty comes straight down out of the well and is let go there; the fresh one comes up from under. An untrained man's lands on the lip beside
        // the well, is backed off, then found; seated, he slaps it home; and the handle let go, the hand flies on back.
        let wob = k(RELOAD_FUMBLE) * (1.0 - tr);
        let fum = wob > 0.004;
        let hp = if fum { half - 0.1 } else { half };
        let slap = fum && bt.pull && r - i >= 0.4;
        let fl = if fum { (1.0 - tr) * max(bump(t, i - 0.36, i - 0.22, i - 0.09, i + 0.02), if slap { bump(t, i + 0.07, i + 0.15, i + 0.19, i + 0.27) } else { 0.0 }) } else { 0.0 };
        let hit = mad3(mad3(mad3(mag, gun.up, -0.035), gun.left, wob * 1.2), gun.bore, wob * 1.6);
        let off = if fum { tk(lerp3(&hit, &strip, 0.65), scale3(kv(&[FLARE_OFF_0, FLARE_OFF_1, FLARE_OFF_2]), 1.0 - tr)) } else { hit };
        let tap_p = if slap { tk(mad3(mag, gun.up, -k(RELOAD_TAP) * (1.0 - tr)), scale3(kv(&[FLARE_TAP_0, FLARE_TAP_1, FLARE_TAP_2]), 1.0 - tr)) } else { mag };
        let over = mad3(mad3(pl, gun.bore, -(k(RELOAD_FLING) + 0.1 * (1.0 - tr))), gun.left, 0.04 + 0.05 * (1.0 - tr));
        // (a magazine with rounds in it goes to the dump pouch on the hip before the hand goes for a fresh one)
        let (kp, fr) = (bt.keep, bt.fresh);
        let dump = if kp >= 0.0 { hk(kv(&[KIT_MAG_DUMP_0, KIT_MAG_DUMP_1, KIT_MAG_DUMP_2])) } else { pouch };
        let mut p = Path::of(&[(0.0, cur), (max(0.1, o - 0.2), mag), (o, mag), (o + 0.12, strip)]);
        if kp >= 0.0 {
            p.push((o + 0.05 + kp) * 0.5, hk(kv(&[KIT_MAG_VIA_0, KIT_MAG_VIA_1, KIT_MAG_VIA_2])));
            p.push(kp - 0.07, dump);
            p.push(kp + 0.05, dump);
            p.push(fr, pouch);
            p.push(fr + 0.1, pouch);
        } else {
            p.push(hp, pouch);
            p.push(hp + 0.12, pouch);
        }
        if fum {
            p.push(max(hp + 0.13, i - 0.3), hit);
            p.push(i - 0.14, off);
        } else {
            p.push(i - 0.16, strip);
        }
        p.push(i, mag);
        p.push(i + 0.08, mag);
        if bt.pull {
            if fum && r - i >= 0.4 {
                p.push(i + 0.17, tap_p);
                p.push(i + 0.25, mag);
            }
            p.push(r - 0.1, hd);
            p.push(r, hd);
            p.push(r + 0.08, pl);
            p.push(r + 0.17, over);
        } else {
            p.push(i + 0.42, cur);
        }
        p.push(dur - 0.03, cur);
        // the hand: round the magazine from its front, fingers down into the pouch, hooked on the handle
        let (hf_d, hb_d) = (gun.dir(&m_hf), gun.dir(&m_hb));
        // (the hand is down at the pouches from the first of them to the last)
        let (hq, hr) = if kp >= 0.0 { (kp - 0.07, fr + 0.1) } else { (hp, hp + 0.12) };
        let lines = |a0: [f64; 3], a1: [f64; 3], a2: [f64; 3]| {
            let mut q = Path::of(&[(max(0.1, o - 0.2), a0), (o + 0.12, a0), (hq, a1), (hr, a1), (i - 0.24, a0), (i + 0.08, a0)]);
            if bt.pull {
                q.push(r - 0.1, a2);
                q.push(r + 0.16, a2);
            }
            q.at(t)
        };
        let rot = hand_rot(&fix_l, lines(mf, pf, hf_d), lines(mb, pb, hb_d));
        let pole = if fl > 0.0 { lerp3(&r_pole, &kv(&[FLARE_POLE_0, FLARE_POLE_1, FLARE_POLE_2]), fl) } else { r_pole };
        two_bone_ik(rig, d, fk.rot, fk.pos, up_l, fore_l, hand_l, &p.at(t), Some(&pole), pole_w, Some(&rot), false);
        let grip = if kp >= 0.0 { max(bump(t, o - 0.12, o - 0.02, kp - 0.02, kp + 0.06), bump(t, fr, fr + 0.08, i + 0.04, i + 0.12)) } else { bump(t, o - 0.12, o - 0.02, i + 0.04, i + 0.12) };
        let hook = if bt.pull { bump(t, r - 0.14, r - 0.04, r + 0.14, r + 0.22) } else { 0.0 };
        let c = |kk: f64| lerp(0.3, kk, max(grip, hook));
        hand(d, hh, 0, &[c(0.62), c(0.68), c(0.72), c(0.78)], lerp(0.3, 0.6, grip), 0.1 * (1.0 - grip));
        wrist_ease(d, rig, fk, hand_l, &rot0, bump(t, 0.0, 0.12, dur - 0.26, dur - 0.05));
        true
    });
    if posed == Some(true) {
        arm_mask(false);
        if !ask_blend(g, at, at_b, w, true, at, n) {
            return 0;
        }
        hands_out(&h);
    }
    RAN | RAN_ALL
}

/// CharacterAnimator.pistolReloadArm — a pistol's magazine change, the support hand's, over the reload clip's pose: to the butt as the magazine drops
/// free, to the belt for a fresh one (a magazine kept goes to the hip first), seated with the heel of the palm, and over the top to run the slide. The
/// trigger hand, the gun and the body stay the clip's. args: 0 t · 1 w · 3‥9 the beat · 11‥14 the left hand's fix. Columns: the hand pass's.
pub fn pistol_reload_arm(g: usize, at: usize, at_b: usize, at_rest: usize) -> u32 {
    let mut a = [0.0f64; 15];
    a.copy_from_slice(&bench().args[..15]);
    let (t, w, bt, fix_l) = (a[0], a[1], beat(&a), [a[11], a[12], a[13], a[14]]);
    let Some(Some(n)) = with_pose(g, at, |_d, _rig, n, _fk| if cast(n, &ARM_ROLES) && cast(n, &[MIDDLE1_R, HIPS]) { Some(n) } else { None }) else {
        return 0;
    };
    let Some(mut h) = hands_in(g, at_rest) else {
        return 0;
    };
    if !ask_blend(g, at, at, 1.0, false, at_b, n) {
        return 0;
    }
    let hh = &mut h;
    with_pose(g, at_b, |d, rig, _n, fk| {
        fk_run(rig, d, fk.rot, fk.pos);
        let gun = GunFrame::of(fk);
        let hand_l = role(HAND_L);
        let (cur, rot0) = (fk.at(hand_l), fk.rot(hand_l));
        let (left, up, fwd) = trunk_frame(fk);
        let hips = fk.at(role(HIPS));
        let tk = |p: [f64; 3]| mad3(mad3(mad3(hips, left, p[0]), up, p[1]), fwd, p[2]);
        let (pouch, dump) = (tk(kv(&[KIT_MAG_POUCH_0, KIT_MAG_POUCH_1, KIT_MAG_POUCH_2])), tk(kv(&[KIT_MAG_DUMP_0, KIT_MAG_DUMP_1, KIT_MAG_DUMP_2])));
        let (o, i, r, dur, kp, fr) = (bt.out, bt.in_, bt.rack, bt.dur, bt.keep, bt.fresh);
        let (well, strip) = (gun.at(&kv(&[KIT_MAG_WELL_0, KIT_MAG_WELL_1, KIT_MAG_WELL_2])), gun.at(&kv(&[KIT_MAG_STRIP_0, KIT_MAG_STRIP_1, KIT_MAG_STRIP_2])));
        let (sl, pl) = (gun.at(&kv(&[KIT_MAG_SLIDE_0, KIT_MAG_SLIDE_1, KIT_MAG_SLIDE_2])), gun.at(&kv(&[KIT_MAG_PULL_0, KIT_MAG_PULL_1, KIT_MAG_PULL_2])));
        // (seated: the heel of the palm drives it the last centimetre)
        let seat = mad3(well, gun.up, 0.012);
        let mut p = Path::of(&[(0.0, cur), (max(0.08, o - 0.14), well), (o, well), (o + 0.1, strip)]);
        if kp >= 0.0 {
            p.push((o + 0.04 + kp) * 0.5, tk(kv(&[KIT_MAG_VIA_0, KIT_MAG_VIA_1, KIT_MAG_VIA_2])));
            p.push(kp - 0.06, dump);
            p.push(kp + 0.05, dump);
        }
        p.push(fr, pouch);
        p.push(fr + 0.09, pouch);
        p.push(i - 0.13, strip);
        p.push(i, seat);
        p.push(i + 0.07, well);
        if bt.pull {
            p.push(r - 0.09, sl);
            p.push(r, sl);
            p.push(r + 0.07, pl);
            p.push(r + 0.15, mad3(pl, gun.up, 0.03));
        } else {
            p.push(i + 0.3, cur);
        }
        p.push(dur - 0.03, cur);
        let (wf, wb) = (gun.dir(&kv(&[KIT_MAG_WF_0, KIT_MAG_WF_1, KIT_MAG_WF_2])), gun.dir(&kv(&[KIT_MAG_WB_0, KIT_MAG_WB_1, KIT_MAG_WB_2])));
        let (sf, sb) = (gun.dir(&kv(&[KIT_MAG_SF_0, KIT_MAG_SF_1, KIT_MAG_SF_2])), gun.dir(&kv(&[KIT_MAG_SB_0, KIT_MAG_SB_1, KIT_MAG_SB_2])));
        let (pf, pb) = (mad3(mad3(scale3(up, -0.9), left, -0.25), fwd, 0.15), mad3(fwd, left, 0.4));
        let (hq, hr) = (if kp >= 0.0 { kp - 0.06 } else { fr }, fr + 0.09);
        let lines = |a0: [f64; 3], a1: [f64; 3], a2: [f64; 3]| {
            let mut q = Path::of(&[(max(0.08, o - 0.14), a0), (o + 0.1, a0), (hq, a1), (hr, a1), (i - 0.2, a0), (i + 0.07, a0)]);
            if bt.pull {
                q.push(r - 0.09, a2);
                q.push(r + 0.14, a2);
            }
            q.at(t)
        };
        let rot = hand_rot(&fix_l, lines(wf, pf, sf), lines(wb, pb, sb));
        two_bone_ik(rig, d, fk.rot, fk.pos, role(UPPER_ARM_L), role(FOREARM_L), hand_l, &p.at(t), Some(&kv(&[KIT_MAG_POLE_0, KIT_MAG_POLE_1, KIT_MAG_POLE_2])), k(KIT_MAG_POLE_W), Some(&rot), false);
        let grip = max(if kp >= 0.0 { bump(t, o - 0.08, o, kp - 0.02, kp + 0.06) } else { 0.0 }, bump(t, fr, fr + 0.08, i - 0.02, i + 0.06));
        let pinch = if bt.pull { bump(t, r - 0.1, r - 0.02, r + 0.08, r + 0.16) } else { 0.0 };
        let c = |kk: f64| lerp(0.25, kk, max(grip, pinch));
        hand(d, hh, 0, &[c(0.6), c(0.66), c(0.7), c(0.76)], lerp(0.3, 0.65, max(grip, pinch)), 0.1 * (1.0 - grip));
        wrist_ease(d, rig, fk, hand_l, &rot0, bump(t, 0.0, 0.1, dur - 0.22, dur - 0.05));
    });
    arm_mask(false);
    if !ask_blend(g, at, at_b, w, true, at, n) {
        return 0;
    }
    hands_out(&h);
    RAN | RAN_ALL
}

/// character.ts qAng between two tables of doubles
#[inline(always)]
fn q_ang_of(a: &[f64], i: usize, b: &[f64], k: usize) -> f64 {
    let dot = abs(a[i] * b[k] + a[i + 1] * b[k + 1] + a[i + 2] * b[k + 2] + a[i + 3] * b[k + 3]);
    if dot >= 1.0 {
        0.0
    } else {
        2.0 * acos(dot)
    }
}
/// character.ts qTurnBy: the stored rotation at `o` turned on by the share f of a further local rotation, in place
fn q_turn_by(d: &mut [S], o: usize, q: [f64; 4], f: f64) {
    let [mut x, mut y, mut z, mut w] = q;
    if w < 0.0 {
        x = -x;
        y = -y;
        z = -z;
        w = -w;
    }
    let th = acos(min(1.0, w));
    let (mut sc, mut cw) = (f, 1.0);
    if th > 1e-4 {
        sc = sin(f * th) / sin(th);
        cw = cos(f * th);
    }
    x *= sc;
    y *= sc;
    z *= sc;
    let (qx, qy, qz, qw) = (get(d, o), get(d, o + 1), get(d, o + 2), get(d, o + 3));
    put(d, o, qw * x + qx * cw + qy * z - qz * y);
    put(d, o + 1, qw * y - qx * z + qy * cw + qz * x);
    put(d, o + 2, qw * z + qx * y - qy * x + qz * cw);
    put(d, o + 3, qw * cw - qx * x - qy * y - qz * z);
}
/// character.ts qTurnTo: the stored rotation at `o` turned the share f of the way to the local rotation at t[k‥k+3], in place
fn q_turn_to(d: &mut [S], o: usize, t: &[f64], k: usize, f: f64) {
    let (cx, cy, cz, cw) = (-get(d, o), -get(d, o + 1), -get(d, o + 2), get(d, o + 3));
    // conj(q) · target
    let q = [
        cw * t[k] + cx * t[k + 3] + cy * t[k + 2] - cz * t[k + 1],
        cw * t[k + 1] - cx * t[k + 2] + cy * t[k + 3] + cz * t[k],
        cw * t[k + 2] + cx * t[k + 1] - cy * t[k] + cz * t[k + 3],
        cw * t[k + 3] - cx * t[k] - cy * t[k + 1] - cz * t[k + 2],
    ];
    q_turn_by(d, o, q, f);
}

/// where the hand pass's own tables are handed (the room a path of keys has), a man's and his rig's: the library's fist on each digit joint, a side
/// after the other (120) · the turn from it to his relaxed hand (120) · how far a digit of the crouch's own hand stands from the fist (10) · his relaxed
/// thumb (24) · the trigger finger laid straight (12)
const HT_FIST: usize = KEYS_AT;
const HT_OPEN: usize = HT_FIST + 120;
const HT_OWN: usize = HT_OPEN + 120;
const HT_THUMB: usize = HT_OWN + 10;
const HT_TRIG: usize = HT_THUMB + 24;
const HT_END: usize = HT_TRIG + 12;

/// CharacterAnimator.handsPass — the hand pass, after every other layer and on the digit joints alone: a hand the library keyed as a fist and no layer has
/// shaped is opened to the man's relaxed hand, by how much of it is the fist, how free the hand is and how little a layer made of it; the thumb of a hand
/// that is not the fist goes to the relaxed thumb; and a drilled man's trigger finger lies straight along the frame until he fires. args: 0 dt · 1, 2 how
/// much of the left and the right hand is closed round something · 3 how much of a gun the right hand has · 4, 5 a thing in the left, the right fist ·
/// 6 trained. Columns: 0, 1 the grips, eased · 2 seconds since he fired · 3 the finger off the trigger · 4 the gun, eased; the hand pass's (where the
/// locomotion left every digit, and what the layers noted); and its tables.
pub fn hands_pass(g: usize, at: usize) -> u32 {
    const COLS: usize = 5;
    let mut a = [0.0f64; L_IN + COLS];
    a.copy_from_slice(&bench().args[..L_IN + COLS]);
    let (dt, full, gun_r, held, trained) = (a[0], [a[1], a[2]], a[3], [a[4] != 0.0, a[5] != 0.0], a[6]);
    let mut base = [0.0f64; 120];
    let mut note = [0.0f64; 10];
    base.copy_from_slice(&bench().args[HANDS_AT..HANDS_AT + 120]);
    note.copy_from_slice(&bench().args[HANDS_AT + 120..HANDS_AT + 130]);
    let mut tb = [0.0f64; HT_END - HT_FIST];
    tb.copy_from_slice(&bench().args[HT_FIST..HT_END]);
    let (fist, open, own, thumb, trig) = (&tb[..120], &tb[HT_OPEN - HT_FIST..HT_OWN - HT_FIST], &tb[HT_OWN - HT_FIST..HT_THUMB - HT_FIST], &tb[HT_THUMB - HT_FIST..HT_TRIG - HT_FIST], &tb[HT_TRIG - HT_FIST..]);
    if digits(0).len() != HAND_DIGITS || digits(1).len() != HAND_DIGITS {
        return 0;
    }
    let st = &mut a[L_IN..];
    let far = k(HAND_FAR) * DEGR;
    let code = with_pose(g, at, |d, _rig, n, _fk| {
        if digits(0).iter().chain(digits(1)).any(|&i| i as usize >= n) {
            return 0;
        }
        for sd in 0..2 {
            st[sd] = damp(st[sd], if held[sd] { 1.0 } else { 0.0 }, 14.0, dt);
            let free = (1.0 - clamp(full[sd], 0.0, 1.0)) * (1.0 - st[sd]);
            if free <= 0.003 {
                continue;
            }
            let (ns, b, fs, op, nt, ow) = (digits(sd), &base[sd * 60..sd * 60 + 60], &fist[sd * 60..sd * 60 + 60], &open[sd * 60..sd * 60 + 60], &note[sd * 5..sd * 5 + 5], &own[sd * 5..sd * 5 + 5]);
            for dg in 0..5 {
                let kk = dg * 12;
                let (o0, o1, o2) = (ns[dg * 3] as usize * 10 + 3, ns[dg * 3 + 1] as usize * 10 + 3, ns[dg * 3 + 2] as usize * 10 + 3);
                // how much of the hand under the layers is the library's fist (none of it, crouched: that clip's hand is its own)
                let fist_k = 1.0 - clamp((q_ang_of(b, kk, fs, kk) + q_ang_of(b, kk + 4, fs, kk + 4) + q_ang_of(b, kk + 8, fs, kk + 8)) / 3.0 / (ow[dg] * 0.85), 0.0, 1.0);
                if fist_k <= 0.003 && dg > 0 {
                    continue;
                }
                // …less what a layer has made of it
                let moved = (q_ang(d, o0, b, kk) + q_ang(d, o1, b, kk + 4) + q_ang(d, o2, b, kk + 8)) / 3.0;
                let noted = nt[dg] < f64::INFINITY;
                let dd = if noted { nt[dg] } else { far };
                let lay = 1.0 - if dd < 0.03 { if noted { 1.0 } else { 0.0 } } else { clamp(moved / dd, 0.0, 1.0) };
                let f = fist_k * free * lay;
                if f > 0.003 {
                    q_turn_by(d, o0, [op[kk], op[kk + 1], op[kk + 2], op[kk + 3]], f);
                    q_turn_by(d, o1, [op[kk + 4], op[kk + 5], op[kk + 6], op[kk + 7]], f);
                    q_turn_by(d, o2, [op[kk + 8], op[kk + 9], op[kk + 10], op[kk + 11]], f);
                }
                if dg == 0 {
                    // the thumb of a hand that is not the fist: to the relaxed hand's thumb
                    let (t, th) = ((1.0 - fist_k) * free * lay * k(HAND_THUMB_OWN), &thumb[sd * 12..sd * 12 + 12]);
                    if t > 0.003 {
                        q_turn_to(d, o0, th, 0, t);
                        q_turn_to(d, o1, th, 4, t);
                        q_turn_to(d, o2, th, 8, t);
                    }
                }
            }
        }
        // the trigger hand's index, a drilled man's: straight along the frame until he fires, on the trigger while he does and for a moment after
        st[2] += dt;
        let off_t = if st[2] < k(HAND_TRIG_HOLD) { 0.0 } else { 1.0 };
        st[3] = damp(st[3], off_t, if off_t < st[3] { 40.0 } else { 9.0 }, dt);
        // (eased: the caller's gate for a throw, a reload or a strike is a step)
        st[4] = damp(st[4], clamp(gun_r, 0.0, 1.0), 18.0, dt);
        let tr = st[4] * smoothstep(k(HAND_TRIG_TRAINED), k(HAND_TRIG_TRAINED) + 0.1, trained) * st[3];
        if tr > 0.003 {
            let ns = digits(1);
            for j in 0..3 {
                q_turn_to(d, ns[3 + j] as usize * 10 + 3, trig, j * 4, tr);
            }
        }
        RAN
    })
    .unwrap_or(0);
    if code != 0 {
        bench().args[L_IN..L_IN + COLS].copy_from_slice(&a[L_IN..]);
    }
    code
}

/// quat.yaw: about the model's up, the noughts plain
#[inline(always)]
fn yaw_of(a: f64) -> [f64; 4] {
    [0.0, sin(a / 2.0), 0.0, cos(a / 2.0)]
}
/// a twist the skeleton lays on after the layers: the yaw, then the pitch about the level axis the yaw has turned
fn twist_of(y: f64, p: f64) -> [f64; 4] {
    let mut q = yaw_of(y);
    if p != 0.0 {
        q = q_mul(&axis_angle(&q_rotate(&q, &AX), p), &q);
    }
    q
}
/// the head as it will be DRAWN (the neck's and the head's twists of last frame over the run-forward bones): where the joint is and its three axes
struct HeadDrawn {
    at: [f64; 3],
    x: [f64; 3],
    y: [f64; 3],
    z: [f64; 3],
}
impl HeadDrawn {
    fn of(fk: &Fk, tw: &[f64; 4], rest_inv: &[f64; 4]) -> HeadDrawn {
        let tn = twist_of(tw[0], tw[1]);
        let hr = q_mul(&twist_of(tw[2], tw[3]), &q_mul(&tn, &q_mul(&fk.rot(role(HEAD)), rest_inv)));
        let nk = fk.at(role(NECK));
        HeadDrawn { at: add3(nk, q_rotate(&tn, &sub3(fk.at(role(HEAD)), nk))), x: q_rotate(&hr, &[1.0, 0.0, 0.0]), y: q_rotate(&hr, &[0.0, 1.0, 0.0]), z: q_rotate(&hr, &[0.0, 0.0, 1.0]) }
    }
    /// a direction of the head's frame, in the model's
    fn dir(&self, p: &[f64; 3]) -> [f64; 3] {
        let (x, y, z) = (&self.x, &self.y, &self.z);
        [x[0] * p[0] + y[0] * p[1] + z[0] * p[2], x[1] * p[0] + y[1] * p[1] + z[1] * p[2], x[2] * p[0] + y[2] * p[1] + z[2] * p[2]]
    }
}
/// the support arm's mask, joint by joint: the shoulder and the upper arm, the forearm, the hand and all its digits
fn left_arm_mask(w_up: f64, w_fore: f64, w_hand: f64) {
    ask_mask(&[(role(SHOULDER_L), w_up, false), (role(UPPER_ARM_L), w_up, false), (role(FOREARM_L), w_fore, false), (role(HAND_L), w_hand, true)]);
}
/// what a support hand brought to the head is posed from
struct HeadHand {
    e: f64,
    shrug: f64,
    to: [f64; 3],
    pole: [f64; 3],
    fingers: [f64; 3],
    back: [f64; 3],
    curl: [f64; 4],
    thumb: f64,
}
/// the support hand to a place in the head's frame as it will be drawn (a smoke, the goggles): solved on the scratch pose at `at_b`, laid on the arm by e.
/// `tail` = the twists, the head's rest inverse, the left hand's fix (12 doubles).
fn head_hand(g: usize, at: usize, at_b: usize, at_rest: usize, p: &HeadHand, tail: &[f64]) -> u32 {
    let (tw, hri, fix_l) = ([tail[0], tail[1], tail[2], tail[3]], [tail[4], tail[5], tail[6], tail[7]], [tail[8], tail[9], tail[10], tail[11]]);
    let Some(Some((n, hd))) = with_pose(g, at, |d, rig, n, fk| {
        if !(cast(n, &ARM_ROLES) && cast(n, &[NECK, HEAD])) {
            return None;
        }
        fk_run(rig, d, fk.rot, fk.pos);
        Some((n, HeadDrawn::of(fk, &tw, &hri)))
    }) else {
        return 0;
    };
    let Some(mut h) = hands_in(g, at_rest) else {
        return 0;
    };
    if !ask_blend(g, at, at, 1.0, false, at_b, n) {
        return 0;
    }
    let hh = &mut h;
    with_pose(g, at_b, |d, rig, _n, fk| {
        turn(d, rig, role(SHOULDER_L), q_roll(p.shrug * DEGR));
        fk_run(rig, d, fk.rot, fk.pos);
        let r = hand_rot(&fix_l, hd.dir(&p.fingers), hd.dir(&p.back));
        two_bone_ik(rig, d, fk.rot, fk.pos, role(UPPER_ARM_L), role(FOREARM_L), role(HAND_L), &add3(hd.at, hd.dir(&p.to)), Some(&p.pole), 0.85, Some(&r), false);
        hand(d, hh, 0, &p.curl, p.thumb, 0.0);
    });
    left_arm_mask(p.e, p.e, p.e);
    if !ask_blend(g, at, at_b, 1.0, true, at, n) {
        return 0;
    }
    hands_out(&h);
    RAN | RAN_ALL
}
/// CharacterAnimator.smokePose — a smoke's arm: the left wrist to a place in front of the mouth. args: 0 how far up the arm has got · 1‥4 last frame's
/// twists (the neck's yaw and pitch, the head's) · 5‥8 the head's rest inverse · 9‥12 the left hand's fix. Columns: the hand pass's.
pub fn smoke_pose(g: usize, at: usize, at_b: usize, at_rest: usize) -> u32 {
    let mut a = [0.0f64; 13];
    a.copy_from_slice(&bench().args[..13]);
    let x = a[0];
    let p = HeadHand {
        e: x * x * (3.0 - 2.0 * x),
        shrug: k(SMOKE_SHRUG),
        to: kv(&[SMOKE_AT_0, SMOKE_AT_1, SMOKE_AT_2]),
        pole: kv(&[SMOKE_POLE_0, SMOKE_POLE_1, SMOKE_POLE_2]),
        fingers: kv(&[SMOKE_FINGERS_0, SMOKE_FINGERS_1, SMOKE_FINGERS_2]),
        back: kv(&[SMOKE_BACK_0, SMOKE_BACK_1, SMOKE_BACK_2]),
        curl: [k(SMOKE_CURL_0), k(SMOKE_CURL_1), k(SMOKE_CURL_2), k(SMOKE_CURL_3)],
        thumb: k(SMOKE_THUMB),
    };
    head_hand(g, at, at_b, at_rest, &p, &a[1..])
}
/// CharacterAnimator.nvPose — a goggle flip's arm: the left hand at the goggles, wherever between up and down they are. args: 0 how much of the arm the
/// flip has · 1 where the goggles are (0 up ‥ 1 down) · 2‥5 last frame's twists · 6‥9 the head's rest inverse · 10‥13 the left hand's fix. Columns: the
/// hand pass's.
pub fn nv_pose(g: usize, at: usize, at_b: usize, at_rest: usize) -> u32 {
    let mut a = [0.0f64; 14];
    a.copy_from_slice(&bench().args[..14]);
    let p = HeadHand {
        e: a[0],
        shrug: k(NV_SHRUG),
        to: lerp3(&kv(&[NV_UP_0, NV_UP_1, NV_UP_2]), &kv(&[NV_DOWN_0, NV_DOWN_1, NV_DOWN_2]), a[1]),
        pole: kv(&[NV_POLE_0, NV_POLE_1, NV_POLE_2]),
        fingers: kv(&[NV_FINGERS_0, NV_FINGERS_1, NV_FINGERS_2]),
        back: kv(&[NV_BACK_0, NV_BACK_1, NV_BACK_2]),
        curl: [k(NV_CURL_0), k(NV_CURL_1), k(NV_CURL_2), k(NV_CURL_3)],
        thumb: k(NV_THUMB),
    };
    head_hand(g, at, at_b, at_rest, &p, &a[2..])
}

/// the support arm laid on shoulder first, then the elbow, then the hand, and off in the same order (the signals' and the tells' envelopes)
fn staggered(t: f64, tin: f64, tout: f64, e: f64) -> (f64, f64, f64) {
    let seg = |lag: f64| bump(t, lag, lag + tin, e - tout + lag * 0.6 - 0.06, e - 0.06 + lag * 0.6);
    (seg(0.0), seg(0.05), seg(0.09))
}
/// CharacterAnimator.signalLayer — a left-hand signal: the arm solved against the body as it stands and laid on shoulder first. args: 0 the signal's
/// clock · 1 which (0 hold, 1 go, 2 rally, 3 squeeze, 4 aside) · 2 how long it lasts · 3 the side an aside sends a man to · 4‥7 the left hand's fix.
/// Columns: the hand pass's.
pub fn signal_layer(g: usize, at: usize, at_b: usize, at_rest: usize) -> u32 {
    let mut a = [0.0f64; 8];
    a.copy_from_slice(&bench().args[..8]);
    let (t, kind, dur, side, fix_l) = (a[0], a[1], a[2], a[3], [a[4], a[5], a[6], a[7]]);
    let Some(Some(n)) = with_pose(g, at, |d, rig, n, _fk| {
        if !(cast(n, &ARM_ROLES) && cast(n, &[SPINE2, NECK, HEAD])) {
            return None;
        }
        if kind == 1.0 {
            // body English on the chop: the upper spine goes a few degrees with the hand, the eyes stay on where it sends people
            let ft = curve(t, &[[0.22, 0.0], [0.46, 1.0], [0.62, 0.5], [0.9, 0.0]]);
            if ft > 1e-3 {
                let q = |kk: f64| q_mul(&q_yaw(-3.5 * DEGR * ft * kk), &q_mul(&q_roll(-2.0 * DEGR * ft * kk), &q_pitch(2.5 * DEGR * ft * kk)));
                turn(d, rig, role(SPINE2), q(0.4));
                turn(d, rig, role(SPINE3), q(0.6));
                turn(d, rig, role(NECK), q(-0.3));
                turn(d, rig, role(HEAD), q(-0.4));
            }
        }
        Some(n)
    }) else {
        return 0;
    };
    let Some(mut h) = hands_in(g, at_rest) else {
        return 0;
    };
    if !ask_blend(g, at, at, 1.0, false, at_b, n) {
        return 0;
    }
    let hh = &mut h;
    with_pose(g, at_b, |d, rig, _n, fk| {
        fk_run(rig, d, fk.rot, fk.pos);
        let sh = fk.at(role(UPPER_ARM_L));
        let (to, r, pole, clav, curl, thumb, spread): ([f64; 3], [f64; 4], [f64; 3], f64, [f64; 4], f64, f64);
        if kind == 0.0 {
            // the fist snaps up beside the head, palm forward
            let settle = 0.006 * sin(t * 5.5) * bump(t, 0.3, 0.4, 0.6, 0.7);
            to = [sh[0] + 0.14, sh[1] + 0.19 + settle, sh[2] + 0.16];
            r = hand_rot(&fix_l, [0.12, 1.0, 0.12], [0.0, 0.05, -1.0]);
            (pole, clav, curl, thumb, spread) = ([1.0, -0.4, 0.35], 8.0, [1.0, 1.0, 1.0, 1.0], 1.0, 0.0);
        } else if kind == 1.0 {
            // the knife hand comes up by the ear, chops over and down to point the way, holds, returns
            let hx = curve(t, &[[0.0, 0.1], [0.28, 0.1], [0.46, 0.04], [0.95, 0.04]]);
            let hy = curve(t, &[[0.0, 0.2], [0.28, 0.22], [0.36, 0.2], [0.46, -0.02], [0.54, 0.015], [0.95, 0.01]]);
            let hz = curve(t, &[[0.0, 0.1], [0.28, 0.08], [0.38, 0.3], [0.46, 0.43], [0.95, 0.42]]);
            to = [sh[0] + hx, sh[1] + hy, sh[2] + hz];
            let chop = smoothstep(0.28, 0.46, t);
            r = hand_rot(&fix_l, lerp3(&[0.1, 1.0, 0.25], &[0.05, -0.18, 1.0], chop), lerp3(&[1.0, 0.0, -0.2], &[0.6, 0.8, 0.1], chop));
            (pole, clav, curl, thumb, spread) = ([1.0, -0.35, 0.1], 7.0 * (1.0 - 0.6 * chop), [0.05, 0.08, 0.12, 0.18], 0.3, 0.0);
        } else if kind == 3.0 {
            // the stack's "ready": the hand onto the shoulder of the man ahead, two squeezes, and back
            let sq = bump(t, 0.3, 0.36, 0.4, 0.47) + bump(t, 0.52, 0.58, 0.62, 0.69);
            to = [sh[0] - 0.1, sh[1] + 0.04 - 0.012 * sq, sh[2] + 0.5 + 0.008 * sq];
            r = hand_rot(&fix_l, [-0.15, -0.3, 1.0], [0.1, 1.0, 0.3]);
            (pole, clav, curl, thumb, spread) = ([0.8, -0.6, -0.1], 3.0, [0.3 + 0.4 * sq, 0.34 + 0.4 * sq, 0.38 + 0.4 * sq, 0.42 + 0.4 * sq], 0.15 + 0.5 * sq, 0.2);
        } else if kind == 4.0 {
            // "off my line": the open hand to chest height, palm the way he wants the man gone, two pushes
            let push = bump(t, 0.22, 0.36, 0.42, 0.56) + bump(t, 0.6, 0.72, 0.78, 0.94);
            if side >= 0.0 {
                to = [sh[0] + 0.02 + 0.4 * push, sh[1] - 0.12 + 0.05 * push, sh[2] + 0.3 - 0.14 * push];
                r = hand_rot(&fix_l, [0.2 + 0.35 * push, 0.8, 0.55], [-1.0, 0.1, 0.2]);
                pole = [0.9, -0.7, -0.1];
            } else {
                to = [sh[0] - 0.02 - 0.3 * push, sh[1] - 0.1 + 0.04 * push, sh[2] + 0.36 + 0.04 * push];
                r = hand_rot(&fix_l, [-0.2 - 0.3 * push, 0.8, 0.55], [1.0, 0.1, 0.2]);
                pole = [0.8, -0.8, 0.0];
            }
            (clav, curl, thumb, spread) = (3.0, [0.04, 0.06, 0.08, 0.12], 0.25, 0.3);
        } else {
            // the open hand goes up over the head and circles, the forearm coning from the elbow, then comes down
            let circ = bump(t, 0.16, 0.32, dur - 0.42, dur - 0.26);
            let ph = (t - 0.16) * 2.2 * PI * 2.0;
            to = [sh[0] + 0.08 + 0.12 * cos(ph) * circ, sh[1] + 0.36 - 0.02 * circ * (1.0 - cos(ph * 2.0)), sh[2] + 0.08 + 0.12 * sin(ph) * circ];
            r = hand_rot(&fix_l, [0.35 * cos(ph) * circ, 1.0, 0.1 + 0.35 * sin(ph) * circ], [-0.2, 0.0, -1.0]);
            (pole, clav, curl, thumb, spread) = ([1.0, 0.05, 0.3], 14.0, [0.1, 0.15, 0.2, 0.3], 0.2, 0.6);
        }
        if !falsy(clav) {
            turn(d, rig, role(SHOULDER_L), q_roll(clav * DEGR));
        }
        fk_run(rig, d, fk.rot, fk.pos);
        two_bone_ik(rig, d, fk.rot, fk.pos, role(UPPER_ARM_L), role(FOREARM_L), role(HAND_L), &to, Some(&pole), 0.85, Some(&r), false);
        hand(d, hh, 0, &curl, thumb, spread);
    });
    let (w_up, w_fore, w_hand) = staggered(t, 0.2, 0.26, dur);
    left_arm_mask(w_up, w_fore, w_hand);
    if !ask_blend(g, at, at_b, 1.0, true, at, n) {
        return 0;
    }
    hands_out(&h);
    RAN | RAN_ALL
}
/// CharacterAnimator.tellLayer — an idle tell: a look at the watch (the forearm level across the chest, the eyes down onto it), or a hand round to the
/// back of the neck with the chest lifted into the stretch. args: 0 the tell's clock · 1 which (0 the watch, 1 the neck) · 2 how long it lasts · 3‥5 the
/// gaze in the head's frame · 6‥9 the left hand's fix. Columns: 0 how much of the arm the tell has, answered; the hand pass's.
pub fn tell_layer(g: usize, at: usize, at_b: usize, at_rest: usize) -> u32 {
    let mut a = [0.0f64; 10];
    a.copy_from_slice(&bench().args[..10]);
    let (t, watch, dur, gaze_local, fix_l) = (a[0], a[1] == 0.0, a[2], [a[3], a[4], a[5]], [a[6], a[7], a[8], a[9]]);
    let (tin, tout) = if watch { (0.2, 0.28) } else { (0.26, 0.34) };
    // the body's share (the head, the chest): one window over the whole tell
    let env = bump(t, 0.0, tin, dur - tout, dur);
    let Some(Some(n)) = with_pose(g, at, |_d, _rig, n, _fk| if cast(n, &ARM_ROLES) && cast(n, &[SPINE2, NECK, HEAD]) { Some(n) } else { None }) else {
        return 0;
    };
    if env <= 1e-4 {
        bench().args[L_IN] = 0.0;
        return RAN;
    }
    let Some(mut h) = hands_in(g, at_rest) else {
        return 0;
    };
    let mut wrist = [0.0f64; 3];
    with_pose(g, at, |d, rig, _n, fk| {
        fk_run(rig, d, fk.rot, fk.pos);
        if watch {
            // the wrist's spot first, off the shoulder as the body has it now, then the eyes onto the watch on the back of it
            let sh = fk.at(role(UPPER_ARM_L));
            wrist = [sh[0] - 0.21, sh[1] - 0.21, sh[2] + 0.21];
            look_at(d, rig, fk, &gaze_local, [wrist[0] + 0.03, wrist[1] + 0.02, wrist[2] + 0.01], 0.9 * env, 0.0);
        } else {
            // the stretch: the chest up a few degrees, the head forward and tipped away from the working hand
            turn(d, rig, role(SPINE2), q_pitch(-1.6 * DEGR * env));
            turn(d, rig, role(SPINE3), q_pitch(-1.8 * DEGR * env));
            turn(d, rig, role(NECK), q_mul(&q_pitch(5.0 * DEGR * env), &q_roll(-3.0 * DEGR * env)));
            turn(d, rig, role(HEAD), q_mul(&q_pitch(9.0 * DEGR * env), &q_mul(&q_roll(-6.0 * DEGR * env), &q_yaw(-4.0 * DEGR * env))));
        }
    });
    if !ask_blend(g, at, at, 1.0, false, at_b, n) {
        return 0;
    }
    let hh = &mut h;
    with_pose(g, at_b, |d, rig, _n, fk| {
        fk_run(rig, d, fk.rot, fk.pos);
        if watch {
            turn(d, rig, role(SHOULDER_L), q_roll(3.0 * DEGR * env));
            fk_run(rig, d, fk.rot, fk.pos);
            let r = hand_rot(&fix_l, [-1.0, 0.14, 0.12], [0.05, 0.9, -0.42]);
            two_bone_ik(rig, d, fk.rot, fk.pos, role(UPPER_ARM_L), role(FOREARM_L), role(HAND_L), &wrist, Some(&[1.0, -0.55, 0.18]), 0.85, Some(&r), false);
            hand(d, hh, 0, &[0.35, 0.4, 0.45, 0.5], 0.4, 0.0);
        } else {
            // the scapula comes up with the raised elbow; the hand works up and down the back of the neck
            turn(d, rig, role(SHOULDER_L), q_roll(12.0 * DEGR * env));
            fk_run(rig, d, fk.rot, fk.pos);
            let nk = fk.at(role(NECK));
            let rub = 0.018 * sin((t - tin) * 2.0 * PI * 2.5) * bump(t, tin, tin + 0.15, dur - tout - 0.1, dur - tout);
            let to = [nk[0] + 0.085, nk[1] + 0.035 + rub, nk[2] - 0.075];
            let r = hand_rot(&fix_l, [-0.55, 0.35, -0.75], [0.45, 0.55, 0.55]);
            two_bone_ik(rig, d, fk.rot, fk.pos, role(UPPER_ARM_L), role(FOREARM_L), role(HAND_L), &to, Some(&[0.75, 0.35, 0.8]), 0.9, Some(&r), false);
            hand(d, hh, 0, &[0.5, 0.55, 0.6, 0.65], 0.5, 0.0);
        }
    });
    let (w_up, w_fore, w_hand) = staggered(t, tin, tout, dur);
    left_arm_mask(w_up, w_fore, w_hand);
    if !ask_blend(g, at, at_b, 1.0, true, at, n) {
        return 0;
    }
    hands_out(&h);
    bench().args[L_IN] = w_up;
    RAN | RAN_ALL
}

/// CharacterAnimator.torchUpLayer — the flashlight brought up in the gun hand: the right wrist to a spot ahead of the shoulder, the elbow down and a
/// little out, by e. Solved on the scratch pose at `at_b`, laid on the right arm. args: 0 e · 1‥4 the right hand's fix.
pub fn torch_up(g: usize, at: usize, at_b: usize) -> u32 {
    let mut a = [0.0f64; 5];
    a.copy_from_slice(&bench().args[..5]);
    let (e, fix_r) = (a[0], [a[1], a[2], a[3], a[4]]);
    let Some(Some(n)) = with_pose(g, at, |_d, _rig, n, _fk| if cast(n, &ARM_ROLES) { Some(n) } else { None }) else {
        return 0;
    };
    if !ask_blend(g, at, at, 1.0, false, at_b, n) {
        return 0;
    }
    with_pose(g, at_b, |d, rig, _n, fk| {
        fk_run(rig, d, fk.rot, fk.pos);
        let (sh, live0, rot0) = (fk.at(role(UPPER_ARM_R)), fk.at(role(HAND_R)), fk.rot(role(HAND_R)));
        let wrist = [sh[0] - 0.07, sh[1] + 0.04, sh[2] + 0.22];
        let r = nlerp(&rot0, &hand_rot(&fix_r, [0.0, 0.92, 0.38], [-1.0, 0.1, 0.0]), e);
        two_bone_ik(rig, d, fk.rot, fk.pos, role(UPPER_ARM_R), role(FOREARM_R), role(HAND_R), &lerp3(&live0, &wrist, e), Some(&[-0.3, -1.0, 0.0]), e, Some(&r), false);
    });
    ask_mask(&[(role(SHOULDER_R), 1.0, true)]);
    if !ask_blend(g, at, at_b, smoothstep(0.0, 0.25, e), true, at, n) {
        return 0;
    }
    RAN | RAN_ALL
}
/// CharacterAnimator.radioPose — a hand to the radio: the left hand up to the handset at his left collar, the head turned and tipped to it as far as his
/// gun is down. args: 0 w · 1 the head's share · 2‥5 the left hand's fix. Columns: the hand pass's.
pub fn radio_pose(g: usize, at: usize, at_b: usize, at_rest: usize) -> u32 {
    let mut a = [0.0f64; 6];
    a.copy_from_slice(&bench().args[..6]);
    let (w, hw, fix_l) = (a[0], a[1], [a[2], a[3], a[4], a[5]]);
    let Some(Some(n)) = with_pose(g, at, |d, rig, n, _fk| {
        if !(cast(n, &ARM_ROLES) && cast(n, &[NECK, HEAD])) {
            return None;
        }
        if hw > 0.003 {
            turn(d, rig, role(NECK), q_mul(&q_yaw(9.0 * DEGR * hw), &q_pitch(5.0 * DEGR * hw)));
            turn(d, rig, role(HEAD), q_mul(&q_roll(-5.0 * DEGR * hw), &q_mul(&q_yaw(13.0 * DEGR * hw), &q_pitch(8.0 * DEGR * hw))));
        }
        Some(n)
    }) else {
        return 0;
    };
    let Some(mut h) = hands_in(g, at_rest) else {
        return 0;
    };
    if !ask_blend(g, at, at, 1.0, false, at_b, n) {
        return 0;
    }
    let hh = &mut h;
    with_pose(g, at_b, |d, rig, _n, fk| {
        turn(d, rig, role(SHOULDER_L), q_roll(7.0 * DEGR * w));
        fk_run(rig, d, fk.rot, fk.pos);
        let (sh, live0, rot0) = (fk.at(role(UPPER_ARM_L)), fk.at(role(HAND_L)), fk.rot(role(HAND_L)));
        // under and ahead of the collar on that side: the fingers come up onto the handset
        let wrist = [sh[0] - 0.035, sh[1] - 0.115, sh[2] + 0.15];
        let r = nlerp(&rot0, &hand_rot(&fix_l, [-0.4, 0.82, -0.4], [0.3, 0.3, 0.9]), w);
        two_bone_ik(rig, d, fk.rot, fk.pos, role(UPPER_ARM_L), role(FOREARM_L), role(HAND_L), &lerp3(&live0, &wrist, w), Some(&[0.75, -1.0, 0.1]), w, Some(&r), false);
        hand(d, hh, 0, &[0.4, 0.5, 0.6, 0.65], 0.2, 0.0);
    });
    ask_mask(&[(role(SHOULDER_L), 1.0, true)]);
    if !ask_blend(g, at, at_b, smoothstep(0.0, 0.25, w), true, at, n) {
        return 0;
    }
    hands_out(&h);
    RAN | RAN_ALL
}

/// CharacterAnimator.lockpickLayer — at a lock: the crouch's rounded spine sat up toward it, the shoulders up, the head onto it, and both hands at the
/// keyway (the pick raking, the wrench hand under it giving as a pin sets). args: 0 w · 1 the work's clock · 2‥4 the lock, model space · 5‥7 the gaze in
/// the head's frame · 8‥11, 12‥15 the hands' fixes, left then right. Columns: the hand pass's.
pub fn lockpick_layer(g: usize, at: usize, at_b: usize, at_rest: usize) -> u32 {
    let mut a = [0.0f64; 16];
    a.copy_from_slice(&bench().args[..16]);
    let (w, t, lock, gaze_local, fix_l, fix_r) = (a[0], a[1], [a[2], a[3], a[4]], [a[5], a[6], a[7]], [a[8], a[9], a[10], a[11]], [a[12], a[13], a[14], a[15]]);
    let Some(Some(n)) = with_pose(g, at, |d, rig, n, fk| {
        if !(cast(n, &ARM_ROLES) && cast(n, &[SPINE1, SPINE2, NECK, HEAD])) {
            return None;
        }
        // the body, by w: the torso sits up toward the lock (the neck's crane comes off first), the shoulders up, then the head onto it
        let ext = 28.0 * DEGR * w;
        turn(d, rig, role(SPINE1), q_pitch(-ext * 0.36));
        turn(d, rig, role(SPINE2), q_pitch(-ext * 0.39));
        turn(d, rig, role(SPINE3), q_pitch(-ext * 0.25));
        turn(d, rig, role(NECK), q_pitch(ext * 0.45));
        turn(d, rig, role(HEAD), q_pitch(ext * 0.55));
        turn(d, rig, role(SHOULDER_L), q_roll(10.0 * DEGR * w));
        turn(d, rig, role(SHOULDER_R), q_roll(-10.0 * DEGR * w));
        look_at(d, rig, fk, &gaze_local, lock, w, 9.0 * DEGR * w);
        Some(n)
    }) else {
        return 0;
    };
    let Some(mut h) = hands_in(g, at_rest) else {
        return 0;
    };
    if !ask_blend(g, at, at, 1.0, false, at_b, n) {
        return 0;
    }
    let hh = &mut h;
    with_pose(g, at_b, |d, rig, _n, fk| {
        fk_run(rig, d, fk.rot, fk.pos);
        // an uneven rhythm: a rake whose rate wobbles, a quicker jiggle that comes and goes, and now and then a pin sets (a pause, a twist of the wrench)
        let rake = sin(t * 2.0 * PI * 2.3 + 1.4 * sin(t * 2.1));
        let jig = sin(t * 2.0 * PI * 3.6) * (0.5 + 0.5 * sin(t * 0.9 + 2.0));
        let set_ev = smoothstep(0.7, 1.0, sin(t * 1.7) * sin(t * 0.61 + 1.0));
        let work = 1.0 - 0.8 * set_ev;
        let pick = [lock[0] - 0.04, lock[1] - 0.035 + 0.004 * jig * work, lock[2] - 0.135 + 0.01 * rake * work];
        let wrench = [lock[0] + 0.05, lock[1] - 0.075, lock[2] - 0.125];
        let roll_r = (11.0 * rake * work + 4.0 * jig * work - 8.0 * set_ev) * DEGR;
        let rot_r = hand_rot(&fix_r, [-0.1, 0.15, 1.0], q_rotate(&q_roll(roll_r), &normalize([-0.6, 1.0, 0.0])));
        let roll_l = (3.0 * sin(t * 2.0 * PI * 0.55) + 7.0 * set_ev) * DEGR;
        let f_l = normalize([-0.4, 0.5, 1.0]);
        let rot_l = hand_rot(&fix_l, f_l, q_rotate(&axis_angle(&f_l, roll_l), &normalize([0.35, -1.0, 0.55])));
        two_bone_ik(rig, d, fk.rot, fk.pos, role(UPPER_ARM_R), role(FOREARM_R), role(HAND_R), &pick, Some(&[-0.45, -1.0, -0.45]), 0.75, Some(&rot_r), false);
        two_bone_ik(rig, d, fk.rot, fk.pos, role(UPPER_ARM_L), role(FOREARM_L), role(HAND_L), &wrench, Some(&[0.3, -1.0, -0.45]), 0.75, Some(&rot_l), true);
        hand(d, hh, 1, &[0.55, 0.7, 0.9, 0.95], 0.75, 0.0);
        hand(d, hh, 0, &[0.75, 0.8, 0.85, 0.85], 0.6, 0.0);
    });
    arms_mask();
    if !ask_blend(g, at, at_b, w, true, at, n) {
        return 0;
    }
    hands_out(&h);
    RAN | RAN_ALL
}
/// CharacterAnimator.haulLayer — both hands on a body: the trunk bowed toward the grip and tipped off a load, the eyes a stride beyond the hands, and
/// each wrist to its spot through the inverse of the chest twist still to come. args: 0 w · 1, 2 the twist's yaw and pitch · 3 the bow · 4 the right
/// hand's share · 5 the look · 6 the tilt · 7 the grip · 8‥10, 11‥13 the left and the right spot, model space · 14‥16 the gaze in the head's frame ·
/// 17‥20, 21‥24 the hands' fixes, left then right. Columns: the hand pass's.
pub fn haul_layer(g: usize, at: usize, at_b: usize, at_rest: usize) -> u32 {
    let mut a = [0.0f64; 25];
    a.copy_from_slice(&bench().args[..25]);
    let (w, tw_y, tw_p, rw, look, grip) = (a[0], a[1], a[2], a[4], a[5], a[7]);
    let (bow, tilt) = (a[3] * w, a[6] * w);
    let (gaze_local, fix_l, fix_r) = ([a[14], a[15], a[16]], [a[17], a[18], a[19], a[20]], [a[21], a[22], a[23], a[24]]);
    let Some(Some((n, tl, tr))) = with_pose(g, at, |d, rig, n, fk| {
        if !(cast(n, &ARM_ROLES) && cast(n, &[SPINE1, SPINE2, NECK, HEAD])) {
            return None;
        }
        if bow > 1e-4 {
            turn(d, rig, role(SPINE1), q_pitch(bow * 0.4));
            turn(d, rig, role(SPINE2), q_pitch(bow * 0.35));
            turn(d, rig, role(SPINE3), q_pitch(bow * 0.25));
            turn(d, rig, role(NECK), q_pitch(-bow * 0.3));
            turn(d, rig, role(HEAD), q_pitch(-bow * 0.3));
        }
        if abs(tilt) > 1e-4 {
            // the load on his left shoulder: trunk and head tipped off it
            turn(d, rig, role(SPINE2), q_roll(tilt * 0.5));
            turn(d, rig, role(SPINE3), q_roll(tilt * 0.5));
            turn(d, rig, role(NECK), q_roll(tilt * 0.8));
            turn(d, rig, role(HEAD), q_roll(tilt * 0.8));
        }
        fk_run(rig, d, fk.rot, fk.pos);
        let (tl, tr) = (untwist(fk, tw_y, tw_p, [a[8], a[9], a[10]]), untwist(fk, tw_y, tw_p, [a[11], a[12], a[13]]));
        if look > 0.01 {
            let mut to = lerp3(&tl, &tr, 0.5 * rw);
            to[2] += 0.45;
            look_at(d, rig, fk, &gaze_local, to, w * look, 0.0);
        }
        Some((n, tl, tr))
    }) else {
        return 0;
    };
    let Some(mut h) = hands_in(g, at_rest) else {
        return 0;
    };
    if !ask_blend(g, at, at, 1.0, false, at_b, n) {
        return 0;
    }
    let hh = &mut h;
    with_pose(g, at_b, |d, rig, _n, fk| {
        fk_run(rig, d, fk.rot, fk.pos);
        let c = clamp(grip, 0.0, 1.0);
        two_bone_ik(rig, d, fk.rot, fk.pos, role(UPPER_ARM_R), role(FOREARM_R), role(HAND_R), &tr, Some(&[-0.7, -0.25, -0.65]), 0.75, Some(&hand_rot(&fix_r, [0.12, -0.55, 0.83], [-0.45, 1.0, 0.5])), false);
        two_bone_ik(rig, d, fk.rot, fk.pos, role(UPPER_ARM_L), role(FOREARM_L), role(HAND_L), &tl, Some(&[0.7, -0.25, -0.65]), 0.75, Some(&hand_rot(&fix_l, [-0.12, -0.55, 0.83], [0.45, 1.0, 0.5])), true);
        hand(d, hh, 1, &[c * 0.85, c * 0.9, c * 0.95, c], c * 0.7, 0.0);
        hand(d, hh, 0, &[c * 0.85, c * 0.9, c * 0.95, c], c * 0.7, 0.0);
    });
    if rw >= 0.999 {
        arms_mask();
        if !ask_blend(g, at, at_b, w, true, at, n) {
            return 0;
        }
    } else {
        ask_mask(&[(role(SHOULDER_L), 1.0, true)]);
        if !ask_blend(g, at, at_b, w, true, at, n) {
            return 0;
        }
        if rw > 0.003 {
            ask_mask(&[(role(SHOULDER_R), 1.0, true)]);
            if !ask_blend(g, at, at_b, w * rw, true, at, n) {
                return 0;
            }
        }
    }
    hands_out(&h);
    RAN | RAN_ALL
}

/// CharacterAnimator.swapPose — the gun hand's errand between the flashlight's ring at the front of the hip and the sidearm on the thigh: the wrist
/// from wherever the pose has it down onto the one, across to the other, and back. The pose as it stood is kept at `at_sw` (what was in the hand is
/// drawn off it). args: 0 e · 1 how far across · 2 to the weapon (1) or the flashlight (0) · 3, 4 the twist's yaw and pitch · 5‥8 the right hand's fix.
/// Columns: the hand pass's.
pub fn swap_pose(g: usize, at: usize, at_b: usize, at_rest: usize, at_sw: usize) -> u32 {
    let mut a = [0.0f64; 9];
    a.copy_from_slice(&bench().args[..9]);
    let (e, across, to_weapon, tw_y, tw_p, fix_r) = (a[0], a[1], a[2] != 0.0, a[3], a[4], [a[5], a[6], a[7], a[8]]);
    let Some(Some((n, to))) = with_pose(g, at, |d, rig, n, fk| {
        if !(cast(n, &ARM_ROLES) && cast(n, &[HIPS, THIGH_R, SPINE1, SPINE2])) {
            return None;
        }
        fk_run(rig, d, fk.rot, fk.pos);
        let (hp, th) = (fk.at(role(HIPS)), fk.at(role(THIGH_R)));
        // the wrist over the flashlight's ring at the front of the right hip · over the sidearm's grip on the thigh
        let (belt, hol) = (untwist(fk, tw_y, tw_p, [hp[0] - 0.13, hp[1] + 0.1, hp[2] + 0.12]), untwist(fk, tw_y, tw_p, [th[0] - 0.1, th[1] + 0.06, th[2] + 0.01]));
        let (first, second) = if to_weapon { (belt, hol) } else { (hol, belt) };
        Some((n, lerp3(&first, &second, across)))
    }) else {
        return 0;
    };
    let Some(mut h) = hands_in(g, at_rest) else {
        return 0;
    };
    if !ask_blend(g, at, at, 1.0, false, at_b, n) {
        return 0;
    }
    let hh = &mut h;
    with_pose(g, at_b, |d, rig, _n, fk| {
        fk_run(rig, d, fk.rot, fk.pos);
        let (live0, rot0) = (fk.at(role(HAND_R)), fk.rot(role(HAND_R)));
        // (the shoulder drops a little with the reach down)
        turn(d, rig, role(SHOULDER_R), q_roll(4.0 * DEGR * e));
        fk_run(rig, d, fk.rot, fk.pos);
        // fingers down onto it, the back of the hand out to his right; the elbow back
        let r = nlerp(&rot0, &hand_rot(&fix_r, [0.05, -1.0, 0.15], [-0.75, 0.0, 0.6]), e);
        two_bone_ik(rig, d, fk.rot, fk.pos, role(UPPER_ARM_R), role(FOREARM_R), role(HAND_R), &lerp3(&live0, &to, e), Some(&[-0.5, 0.15, -1.0]), e, Some(&r), false);
        hand(d, hh, 1, &[0.72, 0.78, 0.82, 0.85], 0.5, 0.0);
    });
    if !ask_blend(g, at, at, 1.0, false, at_sw, n) {
        return 0;
    }
    ask_mask(&[(role(SHOULDER_R), 1.0, true)]);
    if !ask_blend(g, at, at_b, smoothstep(0.0, 0.25, e), true, at, n) {
        return 0;
    }
    hands_out(&h);
    RAN | RAN_ALL
}

/// CharacterAnimator.stealthLayer — the crouch's own arms: a free man low has his hands down ahead of his knees, never a held frame (the weight goes
/// foot to foot at a stand, the hands drift), swinging with the crouch walk, the left leading its foot in the close-in mode, the elbows flaring as he
/// sinks and the arms rising after him. args: 0 dt · 1 wanted · 2 the crouch shown · 3 how much he moves · 4, 5 the twist's yaw and pitch · 6 the
/// close-in carriage shown · 7 the ready pose's weight · 8 the wide arms' share · 9 hands down · 10‥13, 14‥17 the hands' fixes, left then right.
/// Columns: 0 the weight's clock · 1 its own clock · 2 the arms' crouch · 3 the flare · 4 the lead · 5 where the left foot was · 6 the swing; the hand
/// pass's.
pub fn stealth_layer(g: usize, at: usize, at_b: usize, at_rest: usize) -> u32 {
    const COLS: usize = 7;
    let mut a = [0.0f64; L_IN + COLS];
    a.copy_from_slice(&bench().args[..L_IN + COLS]);
    let (dt, want, crouch, tw_y, tw_p, cs, ready_w, wide_s, hands_down) = (a[0], a[1] != 0.0, a[2], a[4], a[5], a[6], a[7], a[8], a[9]);
    let wm = clamp(a[3], 0.0, 1.0);
    let fixes = [[a[10], a[11], a[12], a[13]], [a[14], a[15], a[16], a[17]]];
    let Some(Some(n)) = with_pose(g, at, |_d, _rig, n, _fk| if cast(n, &ARM_ROLES) && cast(n, &LEG_ROLES) && cast(n, &[SPINE1, SPINE2]) { Some(n) } else { None }) else {
        return 0;
    };
    let (st, _) = a[L_IN..].split_at_mut(COLS);
    let out = |st: &[f64], code: u32| {
        bench().args[L_IN..L_IN + COLS].copy_from_slice(st);
        code
    };
    let (p_rise, p_idle, p_walk, p_creep) = (on(SP_RISE), on(SP_IDLE), on(SP_WALK), on(SP_CREEP));
    st[0] = clamp(st[0] + if want { dt / k(STL_IN_SECS) } else { -dt / k(STL_OUT_SECS) }, 0.0, 1.0);
    let w_a = st[0] * st[0] * (3.0 - 2.0 * st[0]);
    // the arms' own crouch: down with him at once (the elbows flaring for as long as he is still sinking), up after him
    let dc = crouch - st[2];
    if !p_rise || dc >= 0.0 {
        st[3] += ((if p_rise && dt > 0.0 { clamp(dc / dt * 0.35, 0.0, 1.0) } else { 0.0 }) - st[3]) * min(1.0, dt * 12.0);
        st[2] = if p_rise { min(crouch, st[2] + dt * k(STL_RISE_DOWN)) } else { crouch };
    } else {
        st[2] = max(crouch, st[2] - dt / max(0.05, k(SP_RISE_SECS)));
        st[3] += (0.0 - st[3]) * min(1.0, dt * 12.0);
    }
    st[1] += dt;
    let up = st[2] * st[2] * (3.0 - 2.0 * st[2]);
    let mix = lerp(if p_idle { 1.0 } else { 0.0 }, if p_walk { 1.0 } else { 0.0 }, wm);
    let w = w_a * up * max(mix, if p_creep { cs } else { 0.0 });
    if st[0] <= 0.003 || up <= 0.003 {
        st[4] = 0.0;
        return out(st, RAN);
    }
    let t = st[1];
    let code = with_pose(g, at, |d, rig, _n, fk| {
        fk.part(rig, d, list(FK_LEGS));
        // at a stand: never a held frame, the weight goes foot to foot with the feet where they were
        let sway = if p_idle { w_a * up * (1.0 - wm) * (1.0 - cs) * crouch } else { 0.0 };
        if !(sway > 0.003) {
            return RAN | ran_list(FK_LEGS);
        }
        let (foot_l, foot_r) = (role(FOOT_L), role(FOOT_R));
        let (fl, fr, rl, rr) = (fk.at(foot_l), fk.at(foot_r), fk.rot(foot_l), fk.rot(foot_r));
        let ph = t * 2.0 * PI / max(0.5, k(SP_SWAY_SECS));
        shift(d, rig, role(HIPS), &[k(SP_SWAY_M) * sway * sin(ph), 0.0, k(SP_SWAY_M) * 0.35 * sway * sin(ph * 2.0 + 0.6)]);
        fk.part(rig, d, list(FK_LEGS));
        two_bone_ik(rig, d, fk.rot, fk.pos, role(THIGH_L), role(SHIN_L), foot_l, &fl, Some(&knee_pole(&rl)), 0.0, Some(&rl), false);
        two_bone_ik(rig, d, fk.rot, fk.pos, role(THIGH_R), role(SHIN_R), foot_r, &fr, Some(&knee_pole(&rr)), 0.0, Some(&rr), true);
        RAN | RAN_ALL
    })
    .unwrap_or(0);
    if code == 0 {
        return 0;
    }
    if w <= 0.003 {
        st[4] = 0.0;
        return out(st, code);
    }
    let Some(mut h) = hands_in(g, at_rest) else {
        return 0;
    };
    if !ask_blend(g, at, at, 1.0, false, at_b, n) {
        return 0;
    }
    let hh = &mut h;
    let mut arm_w = [0.0f64; 2];
    let aw = &mut arm_w;
    with_pose(g, at_b, |d, rig, _n, fk| {
        let rnd = k(STL_ROUND) * DEGR * w;
        turn(d, rig, role(SHOULDER_L), q_yaw(-rnd));
        turn(d, rig, role(SHOULDER_R), q_yaw(rnd));
        fk_run(rig, d, fk.rot, fk.pos);
        let (kl, kr) = (fk.at(role(SHIN_L)), fk.at(role(SHIN_R)));
        let (k_hi, k_z) = (max(kl[1], kr[1]), max(kl[2], kr[2]));
        let (fz_l, fz_r) = (fk.at(role(FOOT_L))[2], fk.at(role(FOOT_R))[2]);
        // the walk: + = the right foot ahead, so the LEFT hand comes forward
        st[6] += ((if p_walk { clamp((fz_r - fz_l) / k(STL_WALK_STRIDE), -1.0, 1.0) * wm } else { 0.0 }) - st[6]) * min(1.0, dt * k(STL_WALK_RATE));
        let s = st[6];
        // the close-in mode: the free hand leads its own foot
        let vz = if dt > 0.0 { (fz_l - st[5]) / dt } else { 0.0 };
        st[5] = fz_l;
        let tgt = if p_creep && cs > 0.003 { if vz > 0.15 { 1.0 } else { clamp((fz_l + 0.25) / 0.55, 0.0, 1.0) } } else { 0.0 };
        st[4] += (tgt - st[4]) * min(1.0, dt * k(STL_CREEP_RATE));
        let (lt, dr) = (t * 2.0 * PI / max(0.5, k(SP_SWAY_SECS)), k(SP_DRIFT_M) * (1.0 - wm));
        let (wf, wb) = (kv(&[STL_WALK_FWD_0, STL_WALK_FWD_1, STL_WALK_FWD_2]), kv(&[STL_WALK_BACK_0, STL_WALK_BACK_1, STL_WALK_BACK_2]));
        let (pole, fing, back) = (kv(&[STL_IDLE_POLE_0, STL_IDLE_POLE_1, STL_IDLE_POLE_2]), kv(&[STL_IDLE_FING_0, STL_IDLE_FING_1, STL_IDLE_FING_2]), kv(&[STL_IDLE_BACK_0, STL_IDLE_BACK_1, STL_IDLE_BACK_2]));
        for side in 0..2usize {
            let (sg, kn, sf) = if side == 0 { (1.0, kl, 0.0) } else { (-1.0, kr, 1.0) };
            let a_ = sg * s;
            let (up_n, fo_n, ha_n) = if side == 0 { (role(UPPER_ARM_L), role(FOREARM_L), role(HAND_L)) } else { (role(UPPER_ARM_R), role(FOREARM_R), role(HAND_R)) };
            // (the knife hand is the close-in mode's; with his arms out the left is the carriage's too)
            let w_arm = w * (1.0 - ready_w) * (if side == 0 { if p_creep { 1.0 - cs * wide_s * clamp(hands_down, 0.0, 1.0) } else { 1.0 - cs } } else { 1.0 - cs });
            if w_arm <= 0.003 {
                continue;
            }
            let (s0, fo, ha) = (fk.at(up_n), fk.at(fo_n), fk.at(ha_n));
            let len = (dist3(&fo, &s0) + dist3(&ha, &fo)) * 0.97;
            let (d1, d2, d3) = (dr * sin(lt * 0.83 + sf * 2.1), dr * sin(lt * 1.31 + sf * 1.3 + 0.7), dr * sin(lt * 0.57 + sf * 0.9 + 1.9));
            let mut to = untwist(fk, tw_y, tw_p, [kn[0] + sg * k(STL_IDLE_OUT) + d1, max(k(STL_IDLE_FLOOR), k_hi - k(STL_IDLE_BELOW)) + d2, max(kn[2] + k(STL_IDLE_FWD), k_z + k(STL_IDLE_LEAD)) + d3]);
            if a_ > 0.0 {
                to = lerp3(&to, &untwist(fk, tw_y, tw_p, [kn[0] * 0.6 + sg * wf[0], max(k(STL_IDLE_FLOOR), k_hi + wf[1]), k_z + wf[2]]), a_);
            } else if a_ < 0.0 {
                to = lerp3(&to, &[s0[0] + sg * wb[0], s0[1] + wb[1], s0[2] + wb[2]], -a_);
            }
            if side == 0 && cs > 0.003 && p_creep {
                let behind = [s0[0] + wb[0], s0[1] + wb[1] + 0.06, s0[2] + wb[2] * k(STL_CREEP_SWING)];
                let ahead = untwist(fk, tw_y, tw_p, [kl[0] * 0.5 + k(STL_CREEP_AT_0), max(k(STL_CREEP_FLOOR), k_hi + k(STL_CREEP_AT_1)), k_z + k(STL_CREEP_AT_2)]);
                to = lerp3(&to, &lerp3(&behind, &ahead, st[4]), cs);
            }
            // (beyond the arm: reached toward, never locked out)
            let (dx, dy, dz) = (to[0] - s0[0], to[1] - s0[1], to[2] - s0[2]);
            let dd = hypot3(dx, dy, dz);
            if dd > len {
                to = [s0[0] + dx * len / dd, s0[1] + dy * len / dd, s0[2] + dz * len / dd];
            }
            // (the back arm's elbow goes behind him, not out)
            let (flare, bk) = (st[3] * k(STL_WALK_FLARE_POLE), max(0.0, -a_) * k(STL_WALK_TUCK));
            let r = hand_rot(&fixes[side], [sg * fing[0], fing[1], fing[2]], [sg * back[0], back[1], back[2]]);
            two_bone_ik(rig, d, fk.rot, fk.pos, up_n, fo_n, ha_n, &to, Some(&[sg * (pole[0] + flare - bk), pole[1] + flare * 0.5 + bk * 0.4, pole[2] - bk]), 0.8, Some(&r), false);
            let c = k(STL_IDLE_CURL);
            hand(d, hh, side, &[c, c + 0.04, c + 0.08, c + 0.12], k(STL_IDLE_THUMB), k(STL_IDLE_SPREAD));
            aw[side] = w_arm;
        }
    });
    let l = |i: usize| if arm_w[0] > 0.003 { role(i) } else { usize::MAX };
    let r = |i: usize| if arm_w[1] > 0.003 { role(i) } else { usize::MAX };
    ask_mask(&[
        (l(SHOULDER_L), arm_w[0], false),
        (l(UPPER_ARM_L), arm_w[0], false),
        (l(FOREARM_L), arm_w[0], false),
        (l(HAND_L), arm_w[0], true),
        (r(SHOULDER_R), arm_w[1], false),
        (r(UPPER_ARM_R), arm_w[1], false),
        (r(FOREARM_R), arm_w[1], false),
        (r(HAND_R), arm_w[1], true),
    ]);
    if !ask_blend(g, at, at_b, 1.0, true, at, n) {
        return 0;
    }
    hands_out(&h);
    out(st, RAN | RAN_ALL)
}

/// character.ts nodLift: a goggle nod shown as a lift of the head, as big less a little whichever way the nod went, and smooth through nothing
#[inline(always)]
fn nod_lift(n: f64) -> f64 {
    2.0 * DEGR - hypot2(n, 2.0 * DEGR)
}
/// CharacterAnimator.nvTip: a goggle nod's tip of the HEAD joint (+ = chin down) about the level axis square to where he faces, `head` being the head
/// as it will be drawn (nvHead), so the gaze stays in its own upright plane
fn nv_tip(d: &mut [S], head: &[f64; 4], rest_inv: &[f64; 4], nod: f64) {
    let hd = role(HEAD);
    let gh = q_mul(head, rest_inv);
    let z = q_rotate(&gh, &[0.0, 0.0, 1.0]);
    let l = hypot2(z[0], z[2]);
    let ax = if l > 1e-4 { [z[2] / l, 0.0, -z[0] / l] } else { q_rotate(&gh, &[1.0, 0.0, 0.0]) };
    set_unit(d, hd, &q_mul(&own_rot(d, hd), &axis_angle(&q_rotate(&conj(*head), &ax), nod)));
}
/// CharacterAnimator.cablePose — the look under a door: on the pose at `at` (a copy of the finished pose, which is then MADE the pose) he shuffles in
/// and sinks with his feet where they were, the trunk folds, the head bows at the foot of the leaf, a goggle nod under way is posed on it, and the
/// hands go low: the right wrist at the gap, the left by its knee. args: 0 e · 1 the head's e · 2 the lock work's weight · 3 the lock's distance ·
/// 4 the nod · 5‥7 the gaze in the head's frame · 8‥11 last frame's twists · 12‥15 the head's rest inverse · 16‥19, 20‥23 the hands' fixes, left then
/// right. Columns: 0 the nod's way, settled once a nod (−1 not yet, 0 chin down, 1 a lift) · 1 the nod as posed; the hand pass's.
pub fn cable_pose(g: usize, at: usize, at_b: usize, at_rest: usize) -> u32 {
    const COLS: usize = 2;
    let mut a = [0.0f64; L_IN + COLS];
    a.copy_from_slice(&bench().args[..L_IN + COLS]);
    let (e, eh, lp, lock_z, nod) = (a[0], a[1], a[2], a[3], a[4]);
    let (gaze_local, tw, hri) = ([a[5], a[6], a[7]], [a[8], a[9], a[10], a[11]], [a[12], a[13], a[14], a[15]]);
    let (fix_l, fix_r) = ([a[16], a[17], a[18], a[19]], [a[20], a[21], a[22], a[23]]);
    let mut st = [a[L_IN], a[L_IN + 1]];
    let sr = &mut st;
    let Some(Some(n)) = with_pose(g, at, |d, rig, n, fk| {
        if !(cast(n, &ARM_ROLES) && cast(n, &LEG_ROLES) && cast(n, &[SPINE1, SPINE2, NECK, HEAD])) {
            return None;
        }
        let (hips, foot_l, foot_r, hd) = (role(HIPS), role(FOOT_L), role(FOOT_R), role(HEAD));
        // the shuffle in, all of him, then the sink over feet put back where they were
        shift(d, rig, hips, &[0.0, 0.0, k(SP_CABLE_STEP) * e]);
        fk.part(rig, d, list(FK_LEGS));
        let (fl, fr, rl, rr) = (fk.at(foot_l), fk.at(foot_r), fk.rot(foot_l), fk.rot(foot_r));
        shift(d, rig, hips, &[0.0, -k(STL_CABLE_SINK) * e, 0.0]);
        fk.part(rig, d, list(FK_LEGS));
        two_bone_ik(rig, d, fk.rot, fk.pos, role(THIGH_L), role(SHIN_L), foot_l, &fl, Some(&knee_pole(&rl)), 0.0, Some(&rl), false);
        two_bone_ik(rig, d, fk.rot, fk.pos, role(THIGH_R), role(SHIN_R), foot_r, &fr, Some(&knee_pole(&rr)), 0.0, Some(&rr), true);
        let bow = (k(STL_CABLE_UNDO) * lp + k(SP_CABLE_BOW)) * DEGR * e;
        turn(d, rig, role(SPINE1), q_pitch(bow * 0.36));
        turn(d, rig, role(SPINE2), q_pitch(bow * 0.39));
        turn(d, rig, role(SPINE3), q_pitch(bow * 0.25));
        // (on the way out the head is level again before the back is)
        turn(d, rig, role(NECK), q_pitch(-bow * 0.25 * (1.0 - eh)));
        turn(d, rig, hd, q_pitch(-bow * 0.3 * (1.0 - eh)));
        turn(d, rig, role(SHOULDER_L), q_yaw(-k(STL_CABLE_ROUND) * DEGR * e));
        turn(d, rig, role(SHOULDER_R), q_yaw(k(STL_CABLE_ROUND) * DEGR * e));
        fk_run(rig, d, fk.rot, fk.pos);
        let gap = [0.0, 0.04, max(lock_z + 0.05, fk.at(hd)[2] + k(STL_CABLE_AHEAD))];
        look_at(d, rig, fk, &gaze_local, gap, eh, 0.0);
        if on(SP_NOD) && nod != 0.0 {
            // a nod that starts with the look's head mostly in is a lift of the head off the gap and back; one the look came in over stays the
            // chin-down nod it began as, kept short of straight down. Which is settled once a nod.
            if sr[0] < 0.0 {
                sr[0] = if eh > 0.3 { 1.0 } else { 0.0 };
            }
            let lift = sr[0] > 0.0;
            let mut tip = if lift { nod_lift(nod) } else { nod };
            fk_run(rig, d, fk.rot, fk.pos);
            let head = q_mul(&twist_of(tw[2], tw[3]), &q_mul(&twist_of(tw[0], tw[1]), &fk.rot(hd)));
            if !lift && nod > 0.0 {
                tip = min(nod, max(0.0, k(STL_CABLE_NOD_MAX) * DEGR - asin(clamp(-q_rotate(&q_mul(&head, &hri), &[0.0, 0.0, 1.0])[1], -1.0, 1.0))));
            }
            nv_tip(d, &head, &hri, tip);
            sr[1] = tip;
        }
        Some(n)
    }) else {
        return 0;
    };
    let Some(mut h) = hands_in(g, at_rest) else {
        return 0;
    };
    if !ask_blend(g, at, at, 1.0, false, at_b, n) {
        return 0;
    }
    let hh = &mut h;
    with_pose(g, at_b, |d, rig, _n, fk| {
        fk_run(rig, d, fk.rot, fk.pos);
        let kl = fk.at(role(SHIN_L));
        let (rr, rl) = (hand_rot(&fix_r, [0.05, -0.35, 0.94], [-0.3, 0.9, 0.3]), hand_rot(&fix_l, [-0.15, -0.85, 0.5], [0.45, 0.25, 0.85]));
        two_bone_ik(rig, d, fk.rot, fk.pos, role(UPPER_ARM_R), role(FOREARM_R), role(HAND_R), &[k(STL_CABLE_GAP_0), k(STL_CABLE_GAP_1), lock_z - k(STL_CABLE_SHORT)], Some(&[-0.6, 0.1, -0.75]), 0.8, Some(&rr), false);
        two_bone_ik(rig, d, fk.rot, fk.pos, role(UPPER_ARM_L), role(FOREARM_L), role(HAND_L), &[kl[0] + k(STL_CABLE_KNEE_0), kl[1] + k(STL_CABLE_KNEE_1), kl[2] + k(STL_CABLE_KNEE_2)], Some(&[0.8, 0.0, -0.6]), 0.8, Some(&rl), false);
        hand(d, hh, 1, &[0.5, 0.6, 0.7, 0.75], 0.5, 0.0);
        hand(d, hh, 0, &[0.2, 0.24, 0.28, 0.32], 0.15, 0.3);
    });
    arms_mask();
    if !ask_blend(g, at, at_b, e, true, at, n) {
        return 0;
    }
    hands_out(&h);
    bench().args[L_IN..L_IN + COLS].copy_from_slice(&st);
    RAN | RAN_ALL
}

/// CharacterAnimator.takenLayer — the man a takedown is done to, still on his feet: head, trunk and (hauled back) his arms, pitch only. Its own
/// numbers: 0 the seconds past the reaction's delay · 1 the seconds since he was let go (under 0 = held) · 2 the style (0 arched back, 1 hunched,
/// 2 the head snapped back)
pub fn taken_layer(g: usize, at: usize) -> u32 {
    let (u, off, style) = {
        let a = &bench().args;
        (a[0], a[1], a[2])
    };
    with_pose(g, at, |d, rig, n, _fk| {
        if !(cast(n, &ARM_ROLES) && cast(n, &[SPINE1, SPINE2, NECK, HEAD])) {
            return 0;
        }
        let out = (1.0 - smoothstep(0.6, k(TAKEN_SECS), u)) * if off < 0.0 { 1.0 } else { 1.0 - smoothstep(0.0, 0.2, off) };
        if style == 0.0 {
            // hauled back onto the knife man's shoulder: the whole back bowed, the head right over, the arms flung out in front of him
            let (a, r) = (smoothstep(0.0, 0.1, u) * out, smoothstep(0.02, 0.16, u) * out);
            turn(d, rig, role(SPINE1), q_pitch(-8.0 * a * DEGR));
            turn(d, rig, role(SPINE2), q_pitch(-10.0 * a * DEGR));
            turn(d, rig, role(SPINE3), q_pitch(-10.0 * a * DEGR));
            turn(d, rig, role(NECK), q_pitch(-10.0 * a * DEGR));
            turn(d, rig, role(HEAD), q_pitch(-16.0 * a * DEGR));
            turn(d, rig, role(UPPER_ARM_L), q_pitch(-38.0 * r * DEGR));
            turn(d, rig, role(UPPER_ARM_R), q_pitch(-34.0 * r * DEGR));
            turn(d, rig, role(FOREARM_L), q_pitch(-30.0 * r * DEGR));
            turn(d, rig, role(FOREARM_R), q_pitch(-36.0 * r * DEGR));
        } else if style == 1.0 {
            // struck on the base of the skull: the chin snaps to the chest, the shoulders come up round it, the trunk follows
            let (s, b) = (smoothstep(0.0, 0.06, u) * out, smoothstep(0.0, 0.2, u) * out);
            turn(d, rig, role(SPINE2), q_pitch(8.0 * b * DEGR));
            turn(d, rig, role(SPINE3), q_pitch(8.0 * b * DEGR));
            turn(d, rig, role(NECK), q_pitch(16.0 * s * DEGR));
            turn(d, rig, role(HEAD), q_pitch(26.0 * s * DEGR));
            turn(d, rig, role(SHOULDER_L), q_roll(10.0 * s * DEGR));
            turn(d, rig, role(SHOULDER_R), q_roll(-10.0 * s * DEGR));
        } else {
            // the heel of a hand under the chin: the head goes back, the trunk after it
            let (s, b) = (smoothstep(0.0, 0.05, u) * out, smoothstep(0.0, 0.18, u) * out);
            turn(d, rig, role(SPINE2), q_pitch(-4.5 * b * DEGR));
            turn(d, rig, role(SPINE3), q_pitch(-4.5 * b * DEGR));
            turn(d, rig, role(NECK), q_pitch(-12.0 * s * DEGR));
            turn(d, rig, role(HEAD), q_pitch(-18.0 * s * DEGR));
        }
        RAN
    })
    .unwrap_or(0)
}
/// CharacterAnimator.blowLegs — a takedown's own move, the legs: by contact the rear foot is drawn in under him and the pelvis comes up and over
/// the feet until the shorter leg is near straight; let go after the move's end. Its own numbers: 0 the move's clock · 1 when it lands · 2 its
/// end · 3 the seconds he stays down after it
pub fn blow_legs(g: usize, at: usize) -> u32 {
    let (t, hit, end, tail) = {
        let a = &bench().args;
        (a[0], a[1], a[2], a[3])
    };
    let u = smoothstep(0.0, 0.85 * hit, t);
    let w = u * (1.0 - smoothstep(hit + 0.12, end + if tail > 0.0 { 0.05 } else { 0.0 }, t));
    with_pose(g, at, |d, rig, n, fk| {
        if !cast(n, &LEG_ROLES) {
            return 0;
        }
        if w < 0.003 {
            return RAN;
        }
        fk.part(rig, d, list(FK_LEGS));
        let (mut f_l, mut f_r) = (fk.at(role(FOOT_L)), fk.at(role(FOOT_R)));
        let (r_l, r_r) = (fk.rot(role(FOOT_L)), fk.rot(role(FOOT_R)));
        let left_back = f_l[2] < f_r[2];
        let (mut back, front) = if left_back { (f_l, f_r) } else { (f_r, f_l) };
        let dz = front[2] - back[2] - k(BLOW_LEGS_GAP);
        if dz > 0.0 {
            // (…and its heel comes down as it lands)
            back[2] += dz * w;
            back[1] += (front[1] - back[1]) * w * 0.8 + k(BLOW_LEGS_LIFT) * 4.0 * u * (1.0 - u) * min(1.0, dz / 0.25);
        }
        if left_back {
            f_l = back;
        } else {
            f_r = back;
        }
        let hp = fk.at(role(HIPS));
        let hz = ((f_l[2] + f_r[2]) / 2.0 - hp[2]) * k(BLOW_LEGS_FWD) * w;
        let mut up = k(BLOW_LEGS_UP);
        for (th, sh, ft, f) in [(role(THIGH_L), role(SHIN_L), role(FOOT_L), f_l), (role(THIGH_R), role(SHIN_R), role(FOOT_R), f_r)] {
            let (a, b, c) = (fk.at(th), fk.at(sh), fk.at(ft));
            let len = (dist3(&a, &b) + dist3(&b, &c)) * k(BLOW_LEGS_STRAIGHT);
            let (hx, hzz) = (a[0] - f[0], a[2] + hz - f[2]);
            let q = len * len - hx * hx - hzz * hzz;
            up = min(up, if q > 0.0 { sqrt(q) - (a[1] - f[1]) } else { 0.0 });
        }
        shift(d, rig, role(HIPS), &[0.0, max(0.0, up) * w, hz]);
        fk.part(rig, d, list(FK_LEGS));
        two_bone_ik(rig, d, fk.rot, fk.pos, role(THIGH_L), role(SHIN_L), role(FOOT_L), &f_l, Some(&knee_pole(&r_l)), 0.0, Some(&r_l), false);
        two_bone_ik(rig, d, fk.rot, fk.pos, role(THIGH_R), role(SHIN_R), role(FOOT_R), &f_r, Some(&knee_pole(&r_r)), 0.0, Some(&r_r), true);
        RAN | RAN_ALL
    })
    .unwrap_or(0)
}
/// CharacterAnimator.blowTrunk — a takedown's own move, the trunk's share (every one is delivered by the body, the arm its last link). Its own
/// numbers: 0 the move's clock · 1 when it lands · 2 its end · 3 the style (0 chop, 1 palm heel, 2 hand over the mouth, 3 slash, 4 the rest) ·
/// 4 how far he is down over the man after it
pub fn blow_trunk(g: usize, at: usize) -> u32 {
    let (t, h, e, style, tl) = {
        let a = &bench().args;
        (a[0], a[1], a[2], a[3], a[4])
    };
    let out = 1.0 - smoothstep(e - 0.2, e, t);
    let pt;
    let mut yw = 0.0;
    if style == 0.0 {
        pt = -10.0 * bump(t, 0.05, 0.5 * h, 0.75 * h, h) + 40.0 * smoothstep(0.75 * h, h + 0.14, t) * max(out, 0.55 * tl);
        yw = -16.0 * bump(t, 0.0, 0.5 * h, 0.72 * h, h) + 18.0 * smoothstep(0.72 * h, h + 0.04, t) * out;
    } else if style == 1.0 {
        pt = 14.0 * bump(t, 0.0, 0.45 * h, 0.7 * h, h) - 14.0 * bump(t, 0.8 * h, h + 0.08, e - 0.14, e) + 14.0 * tl;
        yw = -14.0 * bump(t, 0.0, 0.45 * h, 0.7 * h, h) + 24.0 * smoothstep(0.7 * h, h + 0.06, t) * out;
    } else if style == 2.0 {
        pt = 8.0 * bump(t, 0.0, 0.12, h - 0.06, h) - 8.0 * bump(t, h - 0.04, h + 0.08, h + 0.2, e) + 34.0 * tl;
    } else if style == 3.0 {
        pt = -8.0 * smoothstep(h, h + 0.1, t) * out + 12.0 * tl;
        yw = 14.0 * bump(t, 0.0, 0.5 * h, 0.7 * h, h) - 26.0 * smoothstep(0.7 * h, h + 0.08, t) * out;
    } else {
        pt = 5.0 * bump(t, 0.0, 0.5 * h, h, h + 0.12) + 12.0 * tl;
    }
    with_pose(g, at, |d, rig, n, _fk| {
        if !cast(n, &[SPINE2, SPINE3, NECK]) {
            return 0;
        }
        if falsy(pt) && falsy(yw) {
            return RAN;
        }
        let q = q_mul(&q_yaw(yw * 0.5 * DEGR), &q_pitch(pt * 0.5 * DEGR));
        turn(d, rig, role(SPINE2), q);
        turn(d, rig, role(SPINE3), q);
        turn(d, rig, role(NECK), q_pitch(-pt * 0.5 * DEGR));
        RAN
    })
    .unwrap_or(0)
}
/// CharacterAnimator.blowArms — a takedown's own move, the arms, by IK on a copy of the pose (the knife styles' right arm is the knife's own): the
/// chop's fist cocked behind the ear and hammered down, the palm heel driven up and through with the arm locked out; the left flat on his back
/// (chop), over his mouth (the knife from behind) or thrown back and down (the rest). The eyes stay on the mark. Its own numbers: 0 the move's
/// clock · 1 when it lands · 2 its end · 3 the seconds he stays down after it · 4 the style (blow_trunk's) · 5 how free of an aim the arms are ·
/// 6, 7 the twist · 8‥10 where it lands · 11 a second point? · 12‥14 where the free hand goes · 15 his speed · 16‥18 the gaze · 19‥22, 23‥26 the
/// hands' fixes. Its answer (column 0): the layer's weight, which the hand pass reads
pub fn blow_arms(g: usize, at: usize, at_b: usize, at_rest: usize) -> u32 {
    let mut a = [0.0f64; 27];
    a.copy_from_slice(&bench().args[..27]);
    let (t, h, e, tail, style, free, tw_y, tw_p, has2, speed) = (a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7], a[11] != 0.0, a[15]);
    let (at1, at2, gaze_local, fix_l, fix_r) = ([a[8], a[9], a[10]], [a[12], a[13], a[14]], [a[16], a[17], a[18]], [a[19], a[20], a[21], a[22]], [a[23], a[24], a[25], a[26]]);
    let (chop, palm, mouth) = (style == 0.0, style == 1.0, style == 2.0);
    let x = e + tail;
    let w_all = smoothstep(0.0, 0.5 * h, t) * (1.0 - smoothstep(e - 0.16, e, t)) * free;
    let Some(Some(n)) = with_pose(g, at, |_d, _rig, n, _fk| if cast(n, &ARM_ROLES) && cast(n, &[SPINE1, SPINE2, NECK, HEAD]) { Some(n) } else { None }) else {
        return 0;
    };
    if w_all <= 0.0 && !(mouth && t < x) {
        bench().args[L_IN] = 0.0;
        return RAN;
    }
    let Some(mut hs) = hands_in(g, at_rest) else {
        return 0;
    };
    if !ask_blend(g, at, at, 1.0, false, at_b, n) {
        return 0;
    }
    let hh = &mut hs;
    let Some((tgt, right, w_l)) = with_pose(g, at_b, |d, rig, _n, fk| {
        fk_run(rig, d, fk.rot, fk.pos);
        let tgt = untwist(fk, tw_y, tw_p, at1);
        let t2 = if has2 { Some(untwist(fk, tw_y, tw_p, at2)) } else { None };
        let (sl, sr) = (fk.at(role(UPPER_ARM_L)), fk.at(role(UPPER_ARM_R)));
        let (er, el) = (fk.at(role(FOREARM_R)), fk.at(role(FOREARM_L)));
        let arm_r = dist3(&sr, &er) + dist3(&er, &fk.at(role(HAND_R)));
        let arm_l = dist3(&sl, &el) + dist3(&el, &fk.at(role(HAND_L)));
        let cap = |s: [f64; 3], p: [f64; 3], arm: f64| -> [f64; 3] {
            let o = sub3(p, s);
            let (l, c) = (len3(&o), k(BLOW_REACH) * arm);
            if l > c {
                mad3(s, o, c / l)
            } else {
                p
            }
        };
        // the arm thrown out straight along a line
        let lock = |s: [f64; 3], dv: [f64; 3], arm: f64| -> [f64; 3] { mad3(s, normalize(dv), k(BLOW_REACH) * arm) };
        let right = (chop || palm) && w_all > 0.0;
        if right {
            // (when, where the wrist is, the line wrist → knuckles, the back of the hand, the elbow's pole)
            type Key = (f64, [f64; 3], [f64; 3], [f64; 3], [f64; 3]);
            let thru = sub3(tgt, sr);
            let last: Key = (e - 0.14, lock(sr, [thru[0] * 0.5, max(thru[1], 0.0) + 0.4, max(thru[2], 0.1) * 0.8], arm_r), [0.0, 0.8, -0.5], [0.0, -0.5, -0.8], [-0.5, -0.8, -0.2]);
            let (keys, nk): ([Key; 5], usize) = if chop {
                (
                    [
                        (0.5 * h, add3(sr, [0.02, 0.12, -0.14]), [0.2, 0.3, -0.9], [-1.0, 0.0, 0.0], [-0.7, 1.0, -0.4]),
                        (0.72 * h, add3(sr, [0.03, 0.13, -0.17]), [0.2, 0.3, -0.9], [-1.0, 0.0, 0.0], [-0.7, 1.0, -0.4]),
                        (h, cap(sr, tgt, arm_r), [0.1, -0.45, 0.9], [-1.0, 0.2, 0.0], [-0.8, 0.1, -0.3]),
                        (h + 0.1, cap(sr, add3(tgt, [0.0, -0.32, 0.06]), arm_r), [0.1, -0.8, 0.6], [-1.0, 0.2, 0.0], [-0.8, -0.2, -0.4]),
                        (h + 0.22, cap(sr, add3(tgt, [0.0, -0.5, 0.1]), arm_r), [0.1, -0.9, 0.4], [-1.0, 0.2, 0.0], [-0.8, -0.2, -0.5]),
                    ],
                    5,
                )
            } else {
                (
                    [
                        (0.5 * h, add3(sr, [0.16, -0.34, 0.2]), [0.0, 0.9, 0.3], [0.0, 0.3, -0.9], [-0.4, -0.7, -0.5]),
                        (h, cap(sr, tgt, arm_r), [0.0, 0.9, -0.3], [0.0, -0.3, -0.9], [-0.5, -0.8, -0.2]),
                        (h + 0.09, lock(sr, [thru[0] * 0.6, max(thru[1], 0.0) + 0.34, max(thru[2], 0.1) * 0.9], arm_r), [0.0, 0.8, -0.5], [0.0, -0.5, -0.8], [-0.5, -0.8, -0.2]),
                        last,
                        last,
                    ],
                    4,
                )
            };
            let mut i = 0;
            while i < nk - 1 && t >= keys[i + 1].0 {
                i += 1;
            }
            let j = if i + 1 < nk { i + 1 } else { nk - 1 };
            let (ka, kb) = (keys[i], keys[j]);
            let u = if j == i || t <= ka.0 { 0.0 } else { clamp((t - ka.0) / (kb.0 - ka.0), 0.0, 1.0) };
            // (into the hit it only gathers speed)
            let ez = if kb.0 == h { u * u } else { u * u * (3.0 - 2.0 * u) };
            let rot = slerp(&hand_rot(&fix_r, ka.2, ka.3), &hand_rot(&fix_r, kb.2, kb.3), ez);
            two_bone_ik(rig, d, fk.rot, fk.pos, role(UPPER_ARM_R), role(FOREARM_R), role(HAND_R), &lerp3(&ka.1, &kb.1, ez), Some(&lerp3(&ka.4, &kb.4, ez)), 0.85, Some(&rot), false);
            if chop {
                hand(d, hh, 1, &[k(BLOW_FIST_0), k(BLOW_FIST_1), k(BLOW_FIST_2), k(BLOW_FIST_3)], 0.6, 0.0);
            } else {
                hand(d, hh, 1, &[k(BLOW_OPEN_0), k(BLOW_OPEN_1), k(BLOW_OPEN_2), k(BLOW_OPEN_3)], 0.2, 0.0);
            }
        }
        let mut w_l = 0.0;
        if let (true, Some(t2)) = (chop, t2) {
            w_l = bump(t, 0.04, 0.5 * h, h - 0.02, h + 0.15) * free;
            fk_run(rig, d, fk.rot, fk.pos);
            let rot = hand_rot(&fix_l, [0.0, 1.0, 0.2], [0.0, 0.2, -1.0]);
            two_bone_ik(rig, d, fk.rot, fk.pos, role(UPPER_ARM_L), role(FOREARM_L), role(HAND_L), &cap(sl, t2, arm_l), Some(&[0.6, -0.7, -0.2]), 0.8, Some(&rot), false);
            hand(d, hh, 0, &[k(BLOW_OPEN_0), k(BLOW_OPEN_1), k(BLOW_OPEN_2), k(BLOW_OPEN_3)], 0.2, 0.0);
        } else if let (true, Some(t2)) = (mouth, t2) {
            let (c, dn) = if tail > 0.0 { (x - 0.5, x - 0.2) } else { (e - 0.18, e - 0.04) };
            w_l = bump(t, 0.03, 0.14, c, dn) * free * if tail > 0.0 && t > e { 1.0 - smoothstep(k(BLOW_WALK_0), k(BLOW_WALK_1), speed) } else { 1.0 };
            fk_run(rig, d, fk.rot, fk.pos);
            let rot = hand_rot(&fix_l, [-1.0, 0.0, 0.1], [0.1, 0.0, 1.0]);
            two_bone_ik(rig, d, fk.rot, fk.pos, role(UPPER_ARM_L), role(FOREARM_L), role(HAND_L), &cap(sl, t2, arm_l), Some(&[0.8, -0.5, 0.0]), 0.8, Some(&rot), false);
            hand(d, hh, 0, &[k(BLOW_CUP_0), k(BLOW_CUP_1), k(BLOW_CUP_2), k(BLOW_CUP_3)], 0.3, 0.0);
        } else if !chop && !mouth {
            w_l = bump(t, 0.4 * h, h, e - 0.2, e) * free;
            fk_run(rig, d, fk.rot, fk.pos);
            let rot = hand_rot(&fix_l, [0.15, -0.7, -0.7], [1.0, 0.2, 0.1]);
            two_bone_ik(rig, d, fk.rot, fk.pos, role(UPPER_ARM_L), role(FOREARM_L), role(HAND_L), &lock(sl, [0.16, -0.72, -0.68], arm_l), Some(&[0.5, 0.2, -0.8]), 0.6, Some(&rot), false);
            hand(d, hh, 0, &[k(BLOW_LOOSE_0), k(BLOW_LOOSE_1), k(BLOW_LOOSE_2), k(BLOW_LOOSE_3)], 0.3, 0.0);
        }
        (tgt, right, w_l)
    }) else {
        return 0;
    };
    let side = |on: bool, r: usize| if on { role(r) } else { usize::MAX };
    let left = w_l > 0.0;
    ask_mask(&[
        (side(right, SHOULDER_R), w_all, false),
        (side(right, UPPER_ARM_R), w_all, false),
        (side(right, FOREARM_R), w_all, false),
        (side(right, HAND_R), w_all, true),
        (side(left, SHOULDER_L), w_l, false),
        (side(left, UPPER_ARM_L), w_l, false),
        (side(left, FOREARM_L), w_l, false),
        (side(left, HAND_L), w_l, true),
    ]);
    if !ask_blend(g, at, at_b, 1.0, true, at, n) {
        return 0;
    }
    if w_all > 0.0 {
        with_pose(g, at, |d, rig, _n, fk| look_at(d, rig, fk, &gaze_local, tgt, 0.6 * w_all, 0.0));
    }
    hands_out(&hs);
    bench().args[L_IN] = max(w_all, w_l);
    RAN | RAN_ALL
}
/// CharacterAnimator.knifePose — the knife's blow, the right arm over the punch clip's pose, by IK on a copy: to the hilt, the knife drawn down
/// out of the sheath, through the kind's keys (eased key to key, the wrist's spots off the right shoulder, turned onto the man), back to the
/// sheath's mouth and home; then the arm goes back to what the pose under it has. The clock is the caller's. Its own numbers: 0 the key time
/// (0 on the hilt) · 1 the seconds to the hilt · 2 the clip's length · 3 the blow's clock · 4 how far on the hilt the hand was · 5 what the arm
/// goes back to · 6, 7 the twist · 8 a man to turn onto? · 9‥11 where he is · 12 the crouch · 13‥16 the right hand's fix · 17‥20 the sheath's
/// rest turn undone · 21 the keys' end · 22 how many keys; the keys where a path of keys rides, thirteen doubles each (when, the wrist, the
/// blade, the knuckles, the elbow's pole)
pub fn knife_pose(g: usize, at: usize, at_b: usize, at_rest: usize) -> u32 {
    const KEY: usize = 13;
    let mut a = [0.0f64; 23];
    a.copy_from_slice(&bench().args[..23]);
    let (t, t0, total, kb_t, kb_r0, under, tw_y, tw_p, has_at, crouch, end) = (a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7], a[8] != 0.0, a[12], a[21]);
    let (rd_at, fix_r, rc_inv) = ([a[9], a[10], a[11]], [a[13], a[14], a[15], a[16]], [a[17], a[18], a[19], a[20]]);
    let nk = a[22] as usize;
    if nk > 4 {
        return 0;
    }
    let mut keys = [[0.0f64; KEY]; 4];
    for (i, key) in keys.iter_mut().enumerate().take(nk) {
        key.copy_from_slice(&bench().args[KEYS_AT + i * KEY..KEYS_AT + (i + 1) * KEY]);
    }
    let Some(Some(n)) = with_pose(g, at, |_d, _rig, n, _fk| if cast(n, &ARM_ROLES) { Some(n) } else { None }) else {
        return 0;
    };
    let Some(mut hs) = hands_in(g, at_rest) else {
        return 0;
    };
    if !ask_blend(g, at, at, 1.0, false, at_b, n) {
        return 0;
    }
    let hh = &mut hs;
    with_pose(g, at_b, |d, rig, _n, fk| {
        fk_run(rig, d, fk.rot, fk.pos);
        let (sl, sr, er) = (fk.at(role(UPPER_ARM_L)), fk.at(role(UPPER_ARM_R)), fk.at(role(FOREARM_R)));
        let arm = dist3(&sr, &er) + dist3(&er, &fk.at(role(HAND_R)));
        let mut az = 0.0;
        if has_at {
            let gp = untwist(fk, tw_y, tw_p, rd_at);
            let (mx, mz) = ((sl[0] + sr[0]) / 2.0, (sl[2] + sr[2]) / 2.0);
            if hypot2(gp[0] - mx, gp[2] - mz) > 0.05 {
                az = clamp(atan2(gp[0] - mx, gp[2] - mz), -k(KB_AZ), k(KB_AZ));
            }
        }
        let (ca, sa) = (cos(az), sin(az));
        // (left, up, forward) turned az toward his left → model axes
        let fr = |v: [f64; 3]| -> [f64; 3] { [v[0] * ca + v[2] * sa, v[1], v[2] * ca - v[0] * sa] };
        // 0 hilt · 1 clear of the sheath · 2‥ the keys · then clear · hilt
        let stop = |i: usize| -> ([f64; 3], [f64; 4], [f64; 3]) {
            if i == 0 || i >= nk + 3 {
                return sheath_fist(fk, &rc_inv, &fix_r, crouch, 0.0, 0.0);
            }
            if i == 1 || i == nk + 2 {
                return sheath_fist(fk, &rc_inv, &fix_r, crouch, k(KB_CLEAR_0), k(KB_CLEAR_1));
            }
            let q = &keys[i - 2];
            let mut o = fr([q[1], q[2], q[3]]);
            let (l, cap) = (len3(&o), k(KB_REACH) * arm);
            if l > cap {
                o = scale3(o, cap / l);
            }
            let bl = fr([q[4], q[5], q[6]]);
            (add3(sr, o), knife_rot(&fix_r, bl, cross3(&fr([q[7], q[8], q[9]]), &bl)), fr([q[10], q[11], q[12]]))
        };
        let mut ts = [0.0f64; 8];
        ts[1] = k(KB_T_CLEAR);
        for i in 0..nk {
            ts[2 + i] = keys[i][0];
        }
        ts[nk + 2] = end - k(KB_T_CLEAR);
        ts[nk + 3] = end;
        let mut i = 0;
        while i < nk + 2 && t >= ts[i + 1] {
            i += 1;
        }
        let (sa_, sb_) = (stop(i), stop(if i + 1 < nk + 3 { i + 1 } else { nk + 3 }));
        let ez = if i >= nk + 3 || t <= 0.0 { 0.0 } else { smoothstep(ts[i], ts[i + 1], t) };
        two_bone_ik(rig, d, fk.rot, fk.pos, role(UPPER_ARM_R), role(FOREARM_R), role(HAND_R), &lerp3(&sa_.0, &sb_.0, ez), Some(&lerp3(&sa_.2, &sb_.2, ez)), 0.9, Some(&slerp(&sa_.1, &sb_.1, ez)), false);
        hand(d, hh, 1, &[k(REACH_CURL_0), k(REACH_CURL_1), k(REACH_CURL_2), k(REACH_CURL_3)], k(REACH_THUMB), 0.0);
    });
    let w = lerp(under, 1.0, max(kb_r0, smoothstep(0.0, if falsy(t0) { 1e-6 } else { t0 }, kb_t)) * (1.0 - smoothstep(total - k(KB_FADE), total, kb_t)));
    ask_mask(&[(role(SHOULDER_R), w, false), (role(UPPER_ARM_R), w, false), (role(FOREARM_R), w, false), (role(HAND_R), w, true)]);
    if !ask_blend(g, at, at_b, 1.0, true, at, n) {
        return 0;
    }
    hands_out(&hs);
    RAN | RAN_ALL
}
#[inline(always)]
fn ease(c: &mut [f64], i: usize, v: f64, q: f64) {
    c[i] += (v - c[i]) * q;
}
fn ease3(c: &mut [f64], i: usize, v: [f64; 3], q: f64) {
    ease(c, i, v[0], q);
    ease(c, i + 1, v[1], q);
    ease(c, i + 2, v[2], q);
}
#[inline(always)]
fn get3(c: &[f64], i: usize) -> [f64; 3] {
    [c[i], c[i + 1], c[i + 2]]
}
/// CharacterAnimator.readyLayer — the ready pose, a small layer: the trunk bowed toward the man (and bladed for the guard) straight into the
/// pose by the weight, then the arms by IK on a copy in the shoulders' frame as it now stands, blended in shoulder → elbow → hand: the left
/// always, the right by the kind's share, or on the knife's hilt by the reach's clock. Every number is eased in its columns. Its own numbers:
/// 0 the weight · 1 still asked for? · 2 dt · 3, 4 the twist · 5 a man? · 6‥8 where he is · 9 the grab? · 10 how far up the reach goes · 11 both
/// hands bare? · 12 the way in has a shape? · 13 how much of the reach he is fed · 14 the reach's clock · 15 the right hand's share out ·
/// 16 how close (under 0 = not dealt) · 17 the close-in clock · 18 a knife for the right hand? · 19 the crouch · 20‥23, 24‥27 the hands' fixes ·
/// 28‥31 the sheath's rest turn undone. Its table for the call, where a path of keys rides: the kind's lean, blade and clavicle (degrees), a
/// reach? with its eight numbers, a fixed spot, then the left hand's pole, fingers, back, curl, thumb and spread and the right hand's place,
/// pole, fingers, back, curl and thumb. Its columns: the thirty-six eased numbers, the left arm's wait, the hand on the hilt shown, live?
pub fn ready_layer(g: usize, at: usize, at_b: usize, at_rest: usize) -> u32 {
    const R_L: usize = 0;
    const R_R: usize = 3;
    const L_POLE: usize = 6;
    const R_POLE: usize = 9;
    const L_FING: usize = 12;
    const L_BACK: usize = 15;
    const R_FING: usize = 18;
    const R_BACK: usize = 21;
    const LEAN: usize = 24;
    const BLADE: usize = 25;
    const W_R: usize = 26;
    const L_CURL: usize = 27;
    const L_THUMB: usize = 28;
    const L_SPREAD: usize = 29;
    const R_CURL: usize = 30;
    const R_THUMB: usize = 31;
    const CLAV: usize = 32;
    const INTO: usize = 33;
    const UP: usize = 35;
    const CL_L: usize = 36;
    const REACH_SHOWN: usize = 37;
    const LIVE: usize = 38;
    const N: usize = 39;
    let mut a = [0.0f64; 32];
    a.copy_from_slice(&bench().args[..32]);
    let mut tb = [0.0f64; 41];
    tb.copy_from_slice(&bench().args[KEYS_AT..KEYS_AT + 41]);
    let mut cs = [0.0f64; N];
    cs.copy_from_slice(&bench().args[L_IN..L_IN + N]);
    let (w, live, dt, tw_y, tw_p, has_wp, grab, ready_up, bare, way_shape) = (a[0], a[1] != 0.0, a[2], a[3], a[4], a[5] != 0.0, a[9] != 0.0, a[10], a[11] != 0.0, a[12] != 0.0);
    let (rd_a, u, rd_out_r, close_k, cl_t, knife_on, crouch) = (a[13], a[14], a[15], a[16], a[17], a[18] != 0.0, a[19]);
    let (wp, fix_l, fix_r, rc_inv) = ([a[6], a[7], a[8]], [a[20], a[21], a[22], a[23]], [a[24], a[25], a[26], a[27]], [a[28], a[29], a[30], a[31]]);
    let rd_live = cs[LIVE] != 0.0;
    let kq = if !rd_live {
        1.0
    } else if live {
        1.0 - exp(-k(READY_RATE) * dt)
    } else {
        0.0
    };
    let e = w * w * (3.0 - 2.0 * w);
    let c = &mut cs;
    // ---- the trunk: which way the man is from the chest (flat, within a radian of dead ahead), the bow toward him, the guard's blade
    let Some(Some(n)) = with_pose(g, at, |d, rig, n, fk| {
        if !(cast(n, &ARM_ROLES) && cast(n, &[SPINE1, SPINE2, NECK, HEAD])) {
            return None;
        }
        fk_run(rig, d, fk.rot, fk.pos);
        let mut aa = 0.0;
        if has_wp {
            let (gp, o) = (untwist(fk, tw_y, tw_p, wp), fk.at(role(SPINE3)));
            aa = clamp(atan2(gp[0] - o[0], gp[2] - o[2]), -1.0, 1.0);
        }
        ease(c, INTO, sin(aa), kq);
        ease(c, INTO + 1, cos(aa), kq);
        ease(c, LEAN, tb[0] * DEGR, kq);
        ease(c, BLADE, tb[1] * DEGR, kq);
        ease(c, CLAV, tb[2] * DEGR, kq);
        if !rd_live {
            // (never snapped: it comes in from the reach as it was, at its own rate)
            c[UP] = 0.0;
        }
        if ready_up > 0.0 || c[UP] > 1e-4 {
            // (the grab alone: a pistol that is up keeps its aim and a seated man is under him)
            let to = if ready_up > 0.0 && has_wp && grab { ready_up * smoothstep(k(READY_UP_OVER_0), k(READY_UP_OVER_1), untwist(fk, tw_y, tw_p, wp)[1] - fk.at(role(UPPER_ARM_L))[1]) } else { 0.0 };
            ease(c, UP, to, if live { 1.0 - exp(-k(READY_UP_RATE) * dt) } else { 0.0 });
            if to == 0.0 && c[UP] < 1e-4 {
                c[UP] = 0.0;
            }
        }
        let (lean, blade, hi) = (c[LEAN] * e, c[BLADE] * e, c[UP] * e);
        // (bowing toward +Z is + about +X = up × Z)
        let ax = normalize(cross3(&AY, &[c[INTO], 0.0, c[INTO + 1] + 1e-6]));
        if lean > 1e-4 {
            turn(d, rig, role(SPINE2), axis_angle(&ax, lean * 0.45));
            turn(d, rig, role(SPINE3), axis_angle(&ax, lean * 0.55));
            turn(d, rig, role(NECK), axis_angle(&ax, -lean * 0.45));
            turn(d, rig, role(HEAD), axis_angle(&ax, -lean * 0.55));
        }
        if hi > 1e-4 {
            // (up out of the crouch's bow toward the man who stands over him)
            let r = hi * k(READY_UP_RISE) * DEGR;
            turn(d, rig, role(SPINE1), axis_angle(&ax, -r * 0.3));
            turn(d, rig, role(SPINE2), axis_angle(&ax, -r * 0.35));
            turn(d, rig, role(SPINE3), axis_angle(&ax, -r * 0.35));
            turn(d, rig, role(NECK), axis_angle(&ax, r * k(READY_UP_LOOK) * 0.45));
            turn(d, rig, role(HEAD), axis_angle(&ax, r * k(READY_UP_LOOK) * 0.55));
        }
        if blade > 1e-4 {
            // (− yaw swings his left shoulder toward +Z)
            turn(d, rig, role(SPINE1), q_yaw(-blade * 0.25));
            turn(d, rig, role(SPINE2), q_yaw(-blade * 0.35));
            turn(d, rig, role(SPINE3), q_yaw(-blade * 0.4));
            turn(d, rig, role(NECK), q_yaw(blade * 0.4));
            turn(d, rig, role(HEAD), q_yaw(blade * 0.6));
        }
        Some(n)
    }) else {
        return 0;
    };
    let Some(mut hs) = hands_in(g, at_rest) else {
        return 0;
    };
    // ---- the arms, on a copy: the left clavicle forward with a reach, then the shoulders' frame as it stands
    if !ask_blend(g, at, at, 1.0, false, at_b, n) {
        return 0;
    }
    let hh = &mut hs;
    let Some(w_rb) = with_pose(g, at_b, |d, rig, _n, fk| {
        fk_run(rig, d, fk.rot, fk.pos);
        if c[CLAV] * e > 1e-4 {
            turn(d, rig, role(SHOULDER_L), q_yaw(-c[CLAV] * e));
            fk_run(rig, d, fk.rot, fk.pos);
        }
        let (sl, sr) = (fk.at(role(UPPER_ARM_L)), fk.at(role(UPPER_ARM_R)));
        // forward, square to the shoulder line (his left = (fz, 0, −fx))
        let (mut fx, mut fz) = (-(sl[2] - sr[2]), sl[0] - sr[0]);
        let l = hypot2(fx, fz);
        let l = if falsy(l) { 1.0 } else { l };
        fx /= l;
        fz /= l;
        if live {
            let el = fk.at(role(FOREARM_L));
            let arm = dist3(&sl, &el) + dist3(&el, &fk.at(role(HAND_L)));
            let (mut az, mut h, mut up) = (0.0, 0.0, 0.0);
            // (left, up, forward) in the shoulders' frame turned t toward his left → model axes
            let frame = |v: [f64; 3], t: f64| -> [f64; 3] {
                let (ct, st) = (cos(t), sin(t));
                let (l, f) = (v[0] * ct + v[2] * st, v[2] * ct - v[0] * st);
                [fz * l + fx * f, v[1], fz * f - fx * l]
            };
            if has_wp {
                // where the man is off the left shoulder: level run, height, bearing off square (+ = his left)
                let dd = sub3(untwist(fk, tw_y, tw_p, wp), sl);
                let (pa, pb) = (dd[0] * fx + dd[2] * fz, dd[0] * fz - dd[2] * fx);
                h = hypot2(pa, pb);
                up = dd[1];
                if h > 0.05 {
                    az = clamp(atan2(pb, pa), -k(READY_AZ_0), k(READY_AZ_1));
                }
            }
            let o_l = if tb[3] != 0.0 {
                let (frac, r0, r1, dy0, dy1, bias) = (tb[4], tb[5], tb[6], tb[7], tb[8], tb[9]);
                if !has_wp {
                    h = tb[10];
                    up = tb[11];
                }
                let q = c[UP];
                let lift = if bare && way_shape { smoothstep(k(RB_WAY_LIFT_0), k(RB_WAY_LIFT_1), rd_a) } else { 1.0 };
                let mut o = if bare {
                    // (out ahead of his own shoulder, never over his head)
                    let y = lerp(k(RB_WAY_LOW), clamp(k(RB_L_FRAC) * up + bias, dy0, k(RB_L_DY)), lift);
                    let z = lerp(k(RB_WAY_NEAR), clamp(frac * h, k(RB_L_R_0), max(k(RB_L_R_0), min(k(RB_L_R_1), h - k(RB_L_SHORT)))), lift);
                    frame([0.0, y, z], az)
                } else {
                    frame([0.0, clamp(frac * up + bias, dy0, lerp(dy1, k(READY_UP_DY), q)), clamp(frac * h, lerp(r0, k(READY_UP_R_0), q), lerp(r1, k(READY_UP_R_1), q))], az)
                };
                let (l, cap) = (len3(&o), k(READY_REACH) * arm);
                if l > cap {
                    o = scale3(o, cap / l);
                }
                o
            } else {
                // (a fixed spot is still turned to face him: the guard's lead hand is out toward the man)
                frame([tb[12], tb[13], tb[14]], az)
            };
            ease3(c, R_L, o_l, kq);
            ease3(c, L_POLE, frame([tb[15], tb[16], tb[17]], az), kq);
            let fing = if c[UP] > 1e-4 { [tb[18], tb[19] + k(READY_UP_FING) * c[UP], tb[20]] } else { [tb[18], tb[19], tb[20]] };
            ease3(c, L_FING, frame(fing, az), kq);
            ease3(c, L_BACK, frame([tb[21], tb[22], tb[23]], az), kq);
            ease(c, L_CURL, tb[24], kq);
            ease(c, L_THUMB, tb[25], kq);
            ease(c, L_SPREAD, tb[26], kq);
            if bare {
                // (the right hand's numbers snap when it had no share: it comes in from the arm as it is, by the weight)
                let kr = if c[W_R] < 1e-3 { 1.0 } else { kq };
                ease3(c, R_R, frame([tb[27], tb[28], tb[29]], az), kr);
                ease3(c, R_POLE, frame([tb[30], tb[31], tb[32]], az), kr);
                ease3(c, R_FING, frame([tb[33], tb[34], tb[35]], az), kr);
                ease3(c, R_BACK, frame([tb[36], tb[37], tb[38]], az), kr);
                ease(c, R_CURL, tb[39], kr);
                ease(c, R_THUMB, tb[40], kr);
            }
            ease(c, W_R, if bare { 1.0 } else { 0.0 }, kq);
        } else if !rd_live {
            c[W_R] = 0.0;
        }
        let rot = hand_rot(&fix_l, get3(c, L_FING), get3(c, L_BACK));
        two_bone_ik(rig, d, fk.rot, fk.pos, role(UPPER_ARM_L), role(FOREARM_L), role(HAND_L), &add3(sl, get3(c, R_L)), Some(&get3(c, L_POLE)), 0.85, Some(&rot), false);
        let q = c[L_CURL];
        hand(d, hh, 0, &[q, q + 0.04, q + 0.08, q + 0.12], c[L_THUMB], c[L_SPREAD]);
        // (the right hand comes off his ribs for the last stretch only; never under a hand that is on a hilt)
        let w_rb = if u > 0.0 { 0.0 } else { c[W_R] * rd_out_r * if way_shape { smoothstep(k(RB_WAY_RIGHT_0), k(RB_WAY_RIGHT_1), rd_a) } else { 1.0 } };
        if w_rb > 1e-3 {
            let rot = hand_rot(&fix_r, get3(c, R_FING), get3(c, R_BACK));
            two_bone_ik(rig, d, fk.rot, fk.pos, role(UPPER_ARM_R), role(FOREARM_R), role(HAND_R), &add3(sr, get3(c, R_R)), Some(&get3(c, R_POLE)), 0.85, Some(&rot), false);
            let q = c[R_CURL];
            hand(d, hh, 1, &[q, q + 0.04, q + 0.08, q + 0.12], c[R_THUMB], 0.0);
        }
        // ---- the right hand on the knife: the fist closes on the sheathed handle; the knife stays where it is
        if u > 0.0 {
            let (s_at, s_rot, s_pole) = sheath_fist(fk, &rc_inv, &fix_r, crouch, 0.0, 0.0);
            two_bone_ik(rig, d, fk.rot, fk.pos, role(UPPER_ARM_R), role(FOREARM_R), role(HAND_R), &s_at, Some(&s_pole), 0.9, Some(&s_rot), false);
            if close_k >= 0.0 {
                // (the fist on the hilt is alive: the fingers let go a little, little finger first, and close again; the thumb shifts with them)
                let ph = (cl_t % k(CLOSE_REGRIP)) / k(CLOSE_REGRIP);
                let gg = |dv: f64| bump(ph, 0.55 + dv, 0.66 + dv, 0.72 + dv, 0.86 + dv);
                hand(d, hh, 1, &[k(REACH_CURL_0) - 0.14 * gg(0.06), k(REACH_CURL_1) - 0.2 * gg(0.04), k(REACH_CURL_2) - 0.26 * gg(0.02), k(REACH_CURL_3) - 0.3 * gg(0.0)], k(REACH_THUMB) - 0.25 * gg(0.03), 0.0);
            } else {
                hand(d, hh, 1, &[k(REACH_CURL_0), k(REACH_CURL_1), k(REACH_CURL_2), k(REACH_CURL_3)], k(REACH_THUMB), 0.0);
            }
        }
        w_rb
    }) else {
        return 0;
    };
    // staggered, as the signals: the clavicle / upper arm lead, the forearm and the hand a little behind — in, and out in the same order
    let (w_up, w_fore, w_hand) = (smoothstep(0.0, 0.75, w), smoothstep(0.08, 0.88, w), smoothstep(0.16, 1.0, w));
    // the left arm's move for the man waits for the right hand to be on the hilt
    let mut lg = 1.0;
    if close_k >= 0.0 && knife_on {
        if !rd_live {
            c[CL_L] = 0.0;
        }
        c[CL_L] = clamp(c[CL_L] + if live && u >= k(CLOSE_LEFT_AT) { dt / (k(CLOSE_LEFT_IN) * (1.0 + k(READY_UP_SLOW) * c[UP])) } else { -dt / k(CLOSE_LEFT_OUT) }, 0.0, 1.0);
        lg = c[CL_L] * c[CL_L] * (3.0 - 2.0 * c[CL_L]);
    }
    c[REACH_SHOWN] = smoothstep(0.16, 1.0, u);
    let right = u > 0.0 || w_rb > 1e-3;
    let (r_up, r_fore, r_hand) = if u > 0.0 { (smoothstep(0.0, 0.75, u), smoothstep(0.08, 0.88, u), c[REACH_SHOWN]) } else { (w_up * w_rb, w_fore * w_rb, w_hand * w_rb) };
    let side = |r: usize| if right { role(r) } else { usize::MAX };
    ask_mask(&[
        (role(SHOULDER_L), w_up * lg, false),
        (role(UPPER_ARM_L), w_up * lg, false),
        (role(FOREARM_L), w_fore * lg, false),
        (role(HAND_L), w_hand * lg, true),
        (side(SHOULDER_R), r_up, false),
        (side(UPPER_ARM_R), r_up, false),
        (side(FOREARM_R), r_fore, false),
        (side(HAND_R), r_hand, true),
    ]);
    if !ask_blend(g, at, at_b, 1.0, true, at, n) {
        return 0;
    }
    hands_out(&hs);
    cs[LIVE] = 1.0;
    bench().args[L_IN..L_IN + N].copy_from_slice(&cs);
    RAN | RAN_ALL
}
/// CharacterAnimator.untwistDir: a direction taken back through the chest twist still to come
fn untwist_dir(tw_y: f64, tw_p: f64, dv: [f64; 3]) -> [f64; 3] {
    if falsy(tw_y) && falsy(tw_p) {
        return dv;
    }
    let (e1, e2) = (q_yaw(tw_y * 0.25), q_yaw(tw_y * 0.35));
    let mut e3 = q_yaw(tw_y * 0.4);
    if !falsy(tw_p) {
        let right = q_rotate(&e3, &AX);
        e3 = q_mul(&axis_angle(&right, tw_p), &e3);
    }
    q_rotate(&q_mul(&conj(e1), &q_mul(&conj(e2), &conj(e3))), &dv)
}
/// rigProps.ts Frame: a place and three axes
#[derive(Clone, Copy)]
struct Frame {
    o: [f64; 3],
    x: [f64; 3],
    y: [f64; 3],
    z: [f64; 3],
}
/// an Adj as a layer is handed it: is there one, its offset along the parent's axes, its turn in degrees about the parent's x, then y, then z
type Adj = (bool, [f64; 3], [f64; 3]);
/// rigProps.ts frameFrom
fn frame_from(o: [f64; 3], along: [f64; 3], up_hint: [f64; 3]) -> Frame {
    let z = normalize(along);
    let mut y = sub3(up_hint, scale3(z, dot3(&up_hint, &z)));
    if len3(&y) < 1e-3 {
        y = sub3([1.0, 0.0, 0.0], scale3(z, z[0]));
    }
    let y = normalize(y);
    Frame { o, x: cross3(&y, &z), y, z }
}
/// rigProps.ts applyAdj: the child frame = the parent's moved by the offset and turned about the parent's x, then y, then z
fn apply_adj(f: &Frame, adj: &Adj) -> Frame {
    if !adj.0 {
        return *f;
    }
    let (fo, fx, fy, fz, a) = (f.o, f.x, f.y, f.z, adj.1);
    let o = [((fo[0] + fx[0] * a[0]) + fy[0] * a[1]) + fz[0] * a[2], ((fo[1] + fx[1] * a[0]) + fy[1] * a[1]) + fz[1] * a[2], ((fo[2] + fx[2] * a[0]) + fy[2] * a[1]) + fz[2] * a[2]];
    let dg = PI / 180.0;
    let (mut x, mut y, mut z) = (f.x, f.y, f.z);
    for (axis, deg) in [(f.x, adj.2[0]), (f.y, adj.2[1]), (f.z, adj.2[2])] {
        let ang = deg * dg;
        if !(abs(ang) < 1e-6) {
            let q = axis_angle(&axis, ang);
            x = q_rotate(&q, &x);
            y = q_rotate(&q, &y);
            z = q_rotate(&q, &z);
        }
    }
    Frame { o, x, y, z }
}
/// rigProps.ts unAdj: applyAdj backwards — the parent frame whose child by `adj` is g
fn un_adj(g: &Frame, adj: &Adj) -> Frame {
    if !adj.0 {
        return *g;
    }
    // the child's axes in the parent's own coordinates
    let i = apply_adj(&Frame { o: [0.0; 3], x: [1.0, 0.0, 0.0], y: [0.0, 1.0, 0.0], z: [0.0, 0.0, 1.0] }, &(true, [0.0; 3], adj.2));
    let ax = |j: usize| -> [f64; 3] { [g.x[0] * i.x[j] + g.y[0] * i.y[j] + g.z[0] * i.z[j], g.x[1] * i.x[j] + g.y[1] * i.y[j] + g.z[1] * i.z[j], g.x[2] * i.x[j] + g.y[2] * i.y[j] + g.z[2] * i.z[j]] };
    let (x, y, z, a) = (ax(0), ax(1), ax(2), adj.1);
    Frame { o: [g.o[0] - x[0] * a[0] - y[0] * a[1] - z[0] * a[2], g.o[1] - x[1] * a[0] - y[1] * a[1] - z[1] * a[2], g.o[2] - x[2] * a[0] - y[2] * a[1] - z[2] * a[2]], x, y, z }
}
/// rigProps.ts holsterFrameOf: the thigh holster's group frame off three joints and the body's forward, in whatever one space they are given in;
/// `h` = how far down the thigh, out from the pelvis and forward
fn holster_frame_of(th: [f64; 3], kn: [f64; 3], hp: [f64; 3], fwd: [f64; 3], h: [f64; 3], adj: &Adj) -> Frame {
    let down = normalize(sub3(kn, th));
    let mut outward = sub3(th, hp);
    outward = sub3(outward, scale3(down, dot3(&outward, &down)));
    outward[1] = 0.0;
    let outward = if len3(&outward) > 1e-3 { normalize(outward) } else { cross3(&fwd, &[0.0, 1.0, 0.0]) };
    apply_adj(&frame_from(mad3(mad3(mad3(th, down, h[0]), outward, h[1]), fwd, h[2]), down, fwd), adj)
}
/// where a wrist is, how the hand lies and (for a place a thing lives at) where the elbow points
type Spot = ([f64; 3], [f64; 4], Option<[f64; 3]>);
/// CharacterAnimator.kitLayer — the kit change's frame: the right hand's errand between what was in it and what will be, both arms by IK on a
/// copy of the pose, whole from the upper arm down. The path begins on the hand as the pose under it had it when the change began and ends on
/// the hand of the pose under it now. The clock is the caller's. Its own numbers: 0 the clock · 1 away by · 2 there by · 3 its length · 4, 5 how
/// far stowed and rolled it began · 6 the same thing back up? · 7 a rifle let down in front? · 8, 9 what goes and what comes (0 nothing,
/// 1 pistol, 2 rifle, 3 can, 4 knife) · 10, 11 the twist · 12 the crouch · 13‥16 the right hand's fix · 17‥20 the sheath's rest turn undone ·
/// 21 a model place for the right hand as it began? · 22‥24 it · 25‥27 the hand off its shoulder in the trunk's frame · 28 its lie by
/// directions? · 29‥31, 32‥34 the fingers' line and the back of the hand · 35‥38 its turn on the chest's · 39 a model place for the left? ·
/// 40‥42 it · 43‥45 off its shoulder · 46‥49 its turn on the chest's. Its table for the call, where a path of keys rides: the hand on a rifle
/// let down in front (wrist, fingers, back), the holster's place down the thigh, out and forward, its adj and the pistol group's (each:
/// is there one, offset, degrees). Its columns, written: what the hand holds, how far stowed, how far rolled, the wrist's lie? and it
pub fn kit_layer(g: usize, at: usize, at_b: usize, at_rest: usize) -> u32 {
    let mut a = [0.0f64; 50];
    a.copy_from_slice(&bench().args[..50]);
    let mut tb = [0.0f64; 26];
    tb.copy_from_slice(&bench().args[KEYS_AT..KEYS_AT + 26]);
    let (t, t_a, t_r, dur, s0, roll0, re, front, from, to, tw_y, tw_p, crouch) = (a[0], a[1], a[2], a[3], a[4], a[5], a[6] != 0.0, a[7] != 0.0, a[8], a[9], a[10], a[11], a[12]);
    let (fix_r, rc_inv) = ([a[13], a[14], a[15], a[16]], [a[17], a[18], a[19], a[20]]);
    let (has_mr, mr, cap_r, has_mf, mf, mb, cap_rq) = (a[21] != 0.0, [a[22], a[23], a[24]], [a[25], a[26], a[27]], a[28] != 0.0, [a[29], a[30], a[31]], [a[32], a[33], a[34]], [a[35], a[36], a[37], a[38]]);
    let (has_ml, ml, cap_l, cap_lq) = (a[39] != 0.0, [a[40], a[41], a[42]], [a[43], a[44], a[45]], [a[46], a[47], a[48], a[49]]);
    let (sling_at, sling_fing, sling_back, holster) = ([tb[0], tb[1], tb[2]], [tb[3], tb[4], tb[5]], [tb[6], tb[7], tb[8]], [tb[9], tb[10], tb[11]]);
    let (h_adj, p_adj): (Adj, Adj) = ((tb[12] != 0.0, [tb[13], tb[14], tb[15]], [tb[16], tb[17], tb[18]]), (tb[19] != 0.0, [tb[20], tb[21], tb[22]], [tb[23], tb[24], tb[25]]));
    const PISTOL: f64 = 1.0;
    const RIFLE: f64 = 2.0;
    const CAN: f64 = 3.0;
    let Some(Some(n)) = with_pose(g, at, |_d, _rig, n, _fk| if cast(n, &ARM_ROLES) && cast(n, &LEG_ROLES) && cast(n, &[SPINE2]) { Some(n) } else { None }) else {
        return 0;
    };
    let Some(mut hs) = hands_in(g, at_rest) else {
        return 0;
    };
    if !ask_blend(g, at, at, 1.0, false, at_b, n) {
        return 0;
    }
    let hh = &mut hs;
    let Some((hold, stow, roll, lie, left, join)) = with_pose(g, at_b, |d, rig, _n, fk| {
        fk_run(rig, d, fk.rot, fk.pos);
        let (left, up, fwd) = trunk_frame(fk);
        let q3 = fk.rot(role(SPINE3));
        let dir_t = |q: [f64; 3]| -> [f64; 3] { [left[0] * q[0] + up[0] * q[1] + fwd[0] * q[2], left[1] * q[0] + up[1] * q[1] + fwd[1] * q[2], left[2] * q[0] + up[2] * q[1] + fwd[2] * q[2]] };
        let (s_r, s_l, elbow_u) = (fk.at(role(UPPER_ARM_R)), fk.at(role(UPPER_ARM_L)), fk.at(role(FOREARM_R)));
        let u_r: Spot = (fk.at(role(HAND_R)), fk.rot(role(HAND_R)), None);
        let u_l: Spot = (fk.at(role(HAND_L)), fk.rot(role(HAND_L)), None);
        let (la, lb) = (len3(&sub3(elbow_u, s_r)), len3(&sub3(u_r.0, elbow_u)));
        let rifle = from == RIFLE || to == RIFLE;
        let un = |p: [f64; 3]| untwist(fk, tw_y, tw_p, p);
        let und = |dv: [f64; 3]| untwist_dir(tw_y, tw_p, dv);
        let st: Spot = (if has_mr { un(mr) } else { add3(s_r, dir_t(cap_r)) }, if has_mf { hand_rot(&fix_r, und(mf), und(mb)) } else { q_mul(&q3, &cap_rq) }, None);
        // where the right wrist is, how the hand lies and where the elbow points with the thing in the fist at the place it lives
        let spot = |it: f64| -> Spot {
            if it == RIFLE && front {
                return (add3(fk.at(role(SPINE2)), dir_t(sling_at)), hand_rot(&fix_r, dir_t(sling_fing), dir_t(sling_back)), Some(dir_t(kv(&[KIT_FRONT_POLE_0, KIT_FRONT_POLE_1, KIT_FRONT_POLE_2]))));
            }
            if it == RIFLE {
                let (p, f, b) = (kv(&[KIT_APEX_AT_0, KIT_APEX_AT_1, KIT_APEX_AT_2]), kv(&[KIT_APEX_FING_0, KIT_APEX_FING_1, KIT_APEX_FING_2]), kv(&[KIT_APEX_BACK_0, KIT_APEX_BACK_1, KIT_APEX_BACK_2]));
                return (add3(fk.at(role(UPPER_ARM_R)), dir_t(p)), hand_rot(&fix_r, dir_t(f), dir_t(b)), Some(dir_t(kv(&[KIT_APEX_POLE_0, KIT_APEX_POLE_1, KIT_APEX_POLE_2]))));
            }
            if it == CAN {
                let (p, f, b) = (kv(&[KIT_BELT_AT_0, KIT_BELT_AT_1, KIT_BELT_AT_2]), kv(&[KIT_BELT_FING_0, KIT_BELT_FING_1, KIT_BELT_FING_2]), kv(&[KIT_BELT_BACK_0, KIT_BELT_BACK_1, KIT_BELT_BACK_2]));
                return (un(add3(fk.at(role(HIPS)), dir_t(p))), hand_rot(&fix_r, und(dir_t(f)), und(dir_t(b))), Some(dir_t(kv(&[KIT_BELT_POLE_0, KIT_BELT_POLE_1, KIT_BELT_POLE_2]))));
            }
            if it == PISTOL {
                // the hand that holds the gun as it stands in the holster
                let p = un_adj(&holster_frame_of(fk.at(role(THIGH_R)), fk.at(role(SHIN_R)), fk.at(role(HIPS)), [0.0, 0.0, 1.0], holster, &h_adj), &p_adj);
                return (un(p.o), hand_rot(&fix_r, und(p.z), und(scale3(p.x, -1.0))), Some(dir_t(kv(&[KIT_HOLSTER_POLE_0, KIT_HOLSTER_POLE_1, KIT_HOLSTER_POLE_2]))));
            }
            let (p, r, pl) = sheath_fist(fk, &rc_inv, &fix_r, crouch, 0.0, 0.0);
            (p, r, Some(pl))
        };
        let arc_of = |it: f64| -> [f64; 3] {
            if it == PISTOL {
                kv(&[KIT_ARC_PISTOL_0, KIT_ARC_PISTOL_1, KIT_ARC_PISTOL_2])
            } else if it == RIFLE {
                kv(&[KIT_ARC_RIFLE_0, KIT_ARC_RIFLE_1, KIT_ARC_RIFLE_2])
            } else if it == CAN {
                kv(&[KIT_ARC_CAN_0, KIT_ARC_CAN_1, KIT_ARC_CAN_2])
            } else {
                kv(&[KIT_ARC_KNIFE_0, KIT_ARC_KNIFE_1, KIT_ARC_KNIFE_2])
            }
        };
        // (the rifle's home: over the shoulder onto his back, or let down on its sling in front)
        let back = rifle && !front;
        let rarc = if front { kv(&[KIT_FRONT_ARC_0, KIT_FRONT_ARC_1, KIT_FRONT_ARC_2]) } else { arc_of(RIFLE) };
        let (over, cross) = (kv(&[KIT_OVER_0, KIT_OVER_1, KIT_OVER_2]), kv(&[KIT_CROSS_0, KIT_CROSS_1, KIT_CROSS_2]));
        let e3 = |u: f64| -> f64 {
            let u = clamp(u, 0.0, 1.0);
            u * u * (3.0 - 2.0 * u)
        };
        // (the elbow of a rifle errand: half of it at a steady rate, so it does not all move with the hand at the top of its speed)
        let eh = |u: f64| -> f64 {
            let u = clamp(u, 0.0, 1.0);
            0.5 * (u + u * u * (3.0 - 2.0 * u))
        };
        let mix = |sa: &Spot, sb: &Spot, e: f64, arc: Option<[f64; 3]>, via: Option<[f64; 4]>| -> Spot {
            let p = lerp3(&sa.0, &sb.0, e);
            (
                match arc {
                    Some(ar) => mad3(p, dir_t(ar), sin(PI * e)),
                    None => p,
                },
                match via {
                    Some(v) => slerp(&slerp(&sa.1, &v, e), &slerp(&v, &sb.1, e), e),
                    None => slerp(&sa.1, &sb.1, e),
                },
                None,
            )
        };
        // (the empty hand going over the shoulder leads with its fingers)
        let top = if back { Some(hand_rot(&fix_r, dir_t(kv(&[KIT_TOP_FING_0, KIT_TOP_FING_1, KIT_TOP_FING_2])), dir_t(kv(&[KIT_TOP_BACK_0, KIT_TOP_BACK_1, KIT_TOP_BACK_2])))) } else { None };
        let ap = clamp(if front { k(KIT_FRONT_HAND_AT) } else { k(KIT_APEX_AT) }, 0.05, 0.95);
        let sl = clamp(if front { k(KIT_FRONT_SLING_AT) } else { k(KIT_SLING_AT) }, 0.05, ap);
        let (has_from, has_to) = (from != 0.0, to != 0.0);
        let (mut hold, mut stow, mut roll, mut join, mut free) = (0.0, 0.0, 0.0, 0.0, 0.0);
        let (mut pole_w, grip): (f64, f64);
        let mut pole: Option<[f64; 3]>;
        let mut tg: Spot;
        if has_from && t < t_a {
            // the item in the fist to where it lives
            let (it, u) = (from, t / t_a);
            let hu = if it == RIFLE { clamp(u / ap, 0.0, 1.0) } else { u };
            let (e, s) = (e3(hu), spot(it));
            tg = mix(&st, &s, e, Some(if it == RIFLE { rarc } else { arc_of(it) }), None);
            pole = s.2;
            pole_w = 0.85 * if rifle { eh(hu) } else { e3(hu * 1.6) };
            hold = it;
            grip = 1.0;
            roll = max(smoothstep(0.0, if it == RIFLE { k(KIT_ROLL_IN) } else { 0.35 }, hu), roll0);
            stow = max(
                s0,
                if it == RIFLE {
                    e3((u - sl) / (1.0 - sl))
                } else if it == PISTOL {
                    smoothstep(0.65, 1.0, u)
                } else {
                    0.0
                },
            );
        } else if has_to && t < t_r {
            // the empty hand to where the next one lives
            let u = (t - t_a) / max(1e-3, t_r - t_a);
            let e = e3(u);
            let sa = if has_from { spot(from) } else { st };
            let s = spot(to);
            let arc = if back {
                if has_from {
                    over
                } else {
                    kv(&[KIT_UP_0, KIT_UP_1, KIT_UP_2])
                }
            } else if to == RIFLE {
                rarc
            } else if has_from {
                cross
            } else {
                arc_of(to)
            };
            tg = mix(&sa, &s, e, Some(arc), top);
            pole = s.2;
            pole_w = 0.85
                * if has_from {
                    1.0
                } else if rifle {
                    eh(u)
                } else {
                    e3(u * 1.6)
                };
            if rifle && has_from {
                // (shoulder ↔ hip: the elbow goes with the hand, not ahead of it)
                if let (Some(pa), Some(pb)) = (sa.2, pole) {
                    pole = Some(lerp3(&pa, &pb, eh(u)));
                }
            }
            grip = smoothstep(0.65, 1.0, u);
        } else if has_to {
            // …and up with it into the pose underneath
            let (it, u) = (to, (t - t_r) / max(1e-3, dur - t_r));
            let s = if re { st } else { spot(it) };
            let hu = if it == RIFLE && !re { clamp((u - (1.0 - ap)) / ap, 0.0, 1.0) } else { u };
            let e = e3(hu);
            tg = mix(&s, &u_r, e, if re { None } else { Some(if it == RIFLE { rarc } else { arc_of(it) }) }, None);
            pole = s.2;
            pole_w = 0.85 * (1.0 - if rifle { eh(hu) } else { e3(hu * 1.25) });
            hold = it;
            grip = 1.0;
            join = smoothstep(k(KIT_JOIN), 1.0, hu);
            roll = if re { roll0 * (1.0 - smoothstep(0.4, 1.0, hu)) } else { 1.0 - smoothstep(if it == RIFLE { 1.0 - k(KIT_ROLL_IN) } else { 0.65 }, 1.0, hu) };
            free = e;
            stow = if re {
                s0 * (1.0 - smoothstep(0.0, 0.6, u))
            } else if it == RIFLE {
                1.0 - e3(u / (1.0 - sl))
            } else if it == PISTOL {
                1.0 - smoothstep(0.0, 0.35, u)
            } else {
                0.0
            };
        } else {
            // nothing next: the emptied hand back to the pose underneath
            let u = (t - t_a) / max(1e-3, dur - t_a);
            let e = e3(u);
            let sa = if has_from { spot(from) } else { st };
            tg = mix(&sa, &u_r, e, if back && from == RIFLE { Some(over) } else { None }, if from == RIFLE { top } else { None });
            pole = sa.2;
            pole_w = 0.85 * (1.0 - e);
            grip = 1.0 - smoothstep(0.0, 0.45, u);
            free = e;
        }
        if let (true, Some(pl)) = (rifle, pole) {
            // a rifle's errand: the elbow exactly where its pole says, the pole itself led over from where the pose underneath has the elbow
            let (ax, eb) = (sub3(u_r.0, s_r), sub3(elbow_u, s_r));
            let al = len3(&ax);
            let pu = if al > 1e-4 { mad3(eb, ax, -dot3(&eb, &ax) / (al * al)) } else { eb };
            let pu = if len3(&pu) > 1e-3 { normalize(pu) } else { normalize(pl) };
            let (wv, pn) = (clamp(pole_w / 0.85, 0.0, 1.0), normalize(pl));
            let m = lerp3(&pu, &pn, wv);
            pole = Some(if len3(&m) > 1e-3 { normalize(m) } else { pn });
            pole_w = 1.0;
        }
        // what an arm can do, by how much of the hand is the errand's and not the pose's under it — none on the first frames nor as it lands
        let own = if rifle { clamp(t / 0.12, 0.0, 1.0) * (1.0 - free * free) } else { 0.0 };
        let mut lie: Option<[f64; 4]> = None;
        if own > 0.0 {
            let r_min = sqrt(max(0.0, la * la + lb * lb - 2.0 * la * lb * cos(k(KIT_ELBOW_MIN) * PI / 180.0))) * own;
            let dv = sub3(tg.0, s_r);
            let dl = len3(&dv);
            if dl < r_min {
                tg.0 = mad3(s_r, if dl > 1e-4 { scale3(dv, 1.0 / dl) } else { dir_t([-0.4, 0.2, 0.9]) }, r_min);
            }
        }
        // the support hand, with a gun coming: off whatever it had to wait in front of him, onto the gun late
        let gun_next = to == PISTOL || to == RIFLE;
        let s_l0: Spot = (if has_ml { un(ml) } else { add3(s_l, dir_t(cap_l)) }, q_mul(&q3, &cap_lq), None);
        let mut tl: Option<Spot> = None;
        if gun_next {
            let rest: Spot = (add3(s_l, dir_t(kv(&[KIT_REST_0, KIT_REST_1, KIT_REST_2]))), s_l0.1, None);
            tl = Some(mix(&mix(&s_l0, &rest, e3(t / 0.22), None, None), &u_l, join, None, None));
        } else if t < k(KIT_LET_GO) {
            // …and no gun coming: it lets go and falls away, not snapped off
            join = e3(t / k(KIT_LET_GO));
            tl = Some(mix(&s_l0, &u_l, join, None, None));
        }
        let pw = if pole.is_some() { pole_w } else { 0.0 };
        two_bone_ik(rig, d, fk.rot, fk.pos, role(UPPER_ARM_R), role(FOREARM_R), role(HAND_R), &tg.0, pole.as_ref(), pw, Some(&tg.1), false);
        if own > 0.0 {
            // the wrist: the hand's line brought back toward its forearm's where the path asked more of it than a wrist bends
            let fa = sub3(fk.at(role(HAND_R)), fk.at(role(FOREARM_R)));
            let (fl, fg) = (len3(&fa), q_rotate(&q_mul(&tg.1, &conj(fix_r)), &[0.0, 1.0, 0.0]));
            if fl > 1e-4 {
                let fd = scale3(fa, 1.0 / fl);
                let (ang, kn) = (acos(clamp(dot3(&fd, &fg), -1.0, 1.0)), k(KIT_WRIST_KNEE) * PI / 180.0);
                let mx = max(kn + 0.01, k(KIT_WRIST_MAX) * PI / 180.0);
                if ang > kn {
                    let want = kn + (mx - kn) * (1.0 - exp(-(ang - kn) / (mx - kn)));
                    let ax = cross3(&fg, &fd);
                    let al = len3(&ax);
                    if al > 1e-4 {
                        let lim = q_mul(&axis_angle(&scale3(ax, 1.0 / al), (ang - want) * own), &tg.1);
                        two_bone_ik(rig, d, fk.rot, fk.pos, role(UPPER_ARM_R), role(FOREARM_R), role(HAND_R), &tg.0, pole.as_ref(), pw, Some(&lim), false);
                        lie = Some(q_mul(&conj(lim), &tg.1));
                    }
                }
            }
        }
        let curl = [lerp(k(KIT_OPEN_0), k(KIT_FIST_0), grip), lerp(k(KIT_OPEN_1), k(KIT_FIST_1), grip), lerp(k(KIT_OPEN_2), k(KIT_FIST_2), grip), lerp(k(KIT_OPEN_3), k(KIT_FIST_3), grip)];
        hand(d, hh, 1, &curl, lerp(0.25, 0.6, grip), 0.1 * (1.0 - grip));
        if let Some(sp) = tl {
            two_bone_ik(rig, d, fk.rot, fk.pos, role(UPPER_ARM_L), role(FOREARM_L), role(HAND_L), &sp.0, None, 0.0, Some(&sp.1), false);
        }
        (hold, stow, roll, lie, tl.is_some(), join)
    }) else {
        return 0;
    };
    // (the support hand's fingers go back to the pose's as it joins)
    let side = |r: usize| if left { role(r) } else { usize::MAX };
    ask_mask(&[(role(UPPER_ARM_R), 1.0, true), (side(UPPER_ARM_L), 1.0, true), (side(HAND_L), 1.0 - join, true), (side(HAND_L), 1.0, false)]);
    if !ask_blend(g, at, at_b, 1.0, true, at, n) {
        return 0;
    }
    hands_out(&hs);
    let o = &mut bench().args[L_IN..L_IN + 8];
    o[0] = hold;
    o[1] = stow;
    o[2] = roll;
    o[3] = if lie.is_some() { 1.0 } else { 0.0 };
    o[4..8].copy_from_slice(&lie.unwrap_or([0.0; 4]));
    RAN | RAN_ALL
}
/// the chest twist still to come (yaw spread over spine1‥3, pitch at spine3), run backwards: its three turns undone about the spine's joints
/// as they stood on the pose run forward when this was made (CharacterAnimator.untwist, doorLayer's pivots)
#[derive(Clone, Copy)]
struct Untwist {
    ident: bool,
    ie1: [f64; 4],
    ie2: [f64; 4],
    ie3: [f64; 4],
    i_all: [f64; 4],
    p1: [f64; 3],
    p2t: [f64; 3],
    p3t: [f64; 3],
}
impl Untwist {
    /// `short`: no twist = the point itself (untwist); the door hand's own construction has no such way round
    fn at(fk: &Fk, tw_y: f64, tw_p: f64, short: bool) -> Untwist {
        let (e1, e2) = (q_yaw(tw_y * 0.25), q_yaw(tw_y * 0.35));
        let mut e3 = q_yaw(tw_y * 0.4);
        if !falsy(tw_p) {
            let right = q_rotate(&e3, &AX);
            e3 = q_mul(&axis_angle(&right, tw_p), &e3);
        }
        let (p1, p2, p3) = (fk.at(role(SPINE1)), fk.at(role(SPINE2)), fk.at(role(SPINE3)));
        let p2t = add3(q_rotate(&e1, &sub3(p2, p1)), p1);
        let p3t = add3(q_rotate(&e2, &sub3(add3(q_rotate(&e1, &sub3(p3, p1)), p1), p2t)), p2t);
        Untwist { ident: short && falsy(tw_y) && falsy(tw_p), ie1: conj(e1), ie2: conj(e2), ie3: conj(e3), i_all: conj(q_mul(&e3, &q_mul(&e2, &e1))), p1, p2t, p3t }
    }
    /// where to pose a point for it to end up at `h`
    fn un(&self, h: [f64; 3]) -> [f64; 3] {
        if self.ident {
            return h;
        }
        let x1 = add3(q_rotate(&self.ie3, &sub3(h, self.p3t)), self.p3t);
        let x2 = add3(q_rotate(&self.ie2, &sub3(x1, self.p2t)), self.p2t);
        add3(q_rotate(&self.ie1, &sub3(x2, self.p1)), self.p1)
    }
    /// a direction (the door hand's: the whole twist undone at once)
    fn dir(&self, dv: [f64; 3]) -> [f64; 3] {
        q_rotate(&self.i_all, &dv)
    }
}
/// the door hand's fingers on the leaf, model space: up and ~25° out along the face, away from his middle on that arm's side
fn door_fingers(tw: &Untwist, n0: &[f64; 3], side: usize) -> [f64; 3] {
    let mut t0 = cross3(&AY, n0);
    if len3(&t0) < 1e-4 {
        t0 = [1.0, 0.0, 0.0];
    }
    t0 = normalize(t0);
    if (if side == 1 { -1.0 } else { 1.0 }) * t0[0] < 0.0 {
        t0 = scale3(t0, -1.0);
    }
    tw.dir(normalize(add3(scale3(AY, 0.9), scale3(t0, 0.42))))
}
/// (side → its clavicle, upper arm, forearm and hand)
fn arm_of(side: usize) -> (usize, usize, usize, usize) {
    if side == 1 {
        (role(SHOULDER_R), role(UPPER_ARM_R), role(FOREARM_R), role(HAND_R))
    } else {
        (role(SHOULDER_L), role(UPPER_ARM_L), role(FOREARM_L), role(HAND_L))
    }
}
/// CharacterAnimator.doorLayer — a hand on a door: the spine bowed into a long reach, then the arm that is up by IK on a copy, its target
/// travelling with the eased weight from wherever the pose beneath has the wrist to the spot on the leaf, the palm flat on the face. Its own
/// numbers: 0 dt · 1 how much he means it · 2 the right hand free for it? · 3, 4 the twist · 5 which hand (−1 not settled) · 6 a leaf a
/// stride off too? · 7‥9 out of the face toward him · 10‥12 the spot · 13‥16, 17‥20 the hands' fixes. Its columns: the left and right
/// weights' springs (value, speed), in reach?, the right hand?
pub fn door_layer(g: usize, at: usize, at_b: usize, at_rest: usize) -> u32 {
    let mut a = [0.0f64; 21];
    a.copy_from_slice(&bench().args[..21]);
    let mut cs = [0.0f64; 6];
    cs.copy_from_slice(&bench().args[L_IN..L_IN + 6]);
    let (dt, intent, right_free, tw_y, tw_p, hand_no, reach) = (a[0], a[1], a[2] != 0.0, a[3], a[4], a[5], a[6] != 0.0);
    let (p0, fix) = ([a[10], a[11], a[12]], [[a[13], a[14], a[15], a[16]], [a[17], a[18], a[19], a[20]]]);
    let (live, w_was) = (intent > 0.003, clamp(max(cs[0], cs[2]), 0.0, 1.0));
    if live && hand_no >= 0.0 {
        cs[5] = if hand_no == 1.0 && right_free { 1.0 } else { 0.0 };
    }
    let right = cs[5] != 0.0;
    let arm = if right { 1 } else { 0 };
    let n0 = normalize([a[7], a[8], a[9]]);
    let c = &mut cs;
    let Some(Some((n, tw, f, p_tgt, w_l, w_r))) = with_pose(g, at, |d, rig, n, fk| {
        if !(cast(n, &ARM_ROLES) && cast(n, &[SPINE1, SPINE2, NECK, HEAD])) {
            return None;
        }
        // ---- the lean into a long reach, off the un-leaned pose: the spine bows toward the door by last frame's weight; the head levelled back out of it
        fk_run(rig, d, fk.rot, fk.pos);
        let mut tw = Untwist::at(fk, tw_y, tw_p, false);
        let (nn, f, ua0) = (tw.dir(n0), door_fingers(&tw, &n0, arm), arm_of(arm).1);
        let wrist_of = |p: [f64; 3]| mad3(mad3(p, nn, 0.035), f, -0.08);
        let lean = 8.0 * DEGR * smoothstep(0.4, 0.55, dist3(&fk.at(ua0), &wrist_of(tw.un(p0)))) * w_was;
        if lean > 1e-4 {
            // (bowing toward +Z is + about +X = up × Z)
            let ax = cross3(&AY, &normalize([-n0[0], 0.0, -n0[2] + 1e-6]));
            turn(d, rig, role(SPINE1), axis_angle(&ax, lean * 0.4));
            turn(d, rig, role(SPINE2), axis_angle(&ax, lean * 0.35));
            turn(d, rig, role(SPINE3), axis_angle(&ax, lean * 0.25));
            turn(d, rig, role(NECK), axis_angle(&ax, -lean * 0.45));
            turn(d, rig, role(HEAD), axis_angle(&ax, -lean * 0.55));
            fk_run(rig, d, fk.rot, fk.pos);
            tw = Untwist::at(fk, tw_y, tw_p, false);
        }
        let p_tgt = tw.un(p0);
        // ---- which arm is up, and can it reach: the wrist spot within what the arm spans once leaned and ahead of it; else the hand stays off
        let (mut tgt_l, mut tgt_r) = (0.0, 0.0);
        if live && hand_no >= 0.0 {
            let (sh, wr) = (fk.at(ua0), wrist_of(p_tgt));
            let (dd, ahead, far) = (dist3(&sh, &wr), wr[2] - sh[2], if reach { 0.45 } else { 0.0 });
            let ok = if c[4] != 0.0 { dd < 0.585 + far && dd > 0.16 && ahead > 0.02 } else { dd < 0.56 + far && dd > 0.2 && ahead > 0.06 };
            c[4] = if ok { 1.0 } else { 0.0 };
            if ok {
                if right {
                    tgt_r = intent;
                } else {
                    tgt_l = intent;
                }
            }
        } else {
            c[4] = 0.0;
        }
        // ≈ 0.15 s in, 0.25 s out
        let rate = |t: f64, x: f64| if t > x { 40.0 } else { 26.0 };
        let (rl, rr) = (rate(tgt_l, c[0]), rate(tgt_r, c[2]));
        spring(c, 0, tgt_l, rl, dt);
        spring(c, 2, tgt_r, rr, dt);
        Some((n, tw, f, p_tgt, clamp(c[0], 0.0, 1.0), clamp(c[2], 0.0, 1.0)))
    }) else {
        return 0;
    };
    let Some(mut hs) = hands_in(g, at_rest) else {
        return 0;
    };
    let nn = tw.dir(n0);
    // ---- the arm(s): the target travels with the eased weight, the hand's turn, the elbow's swivel and the clavicle's reach riding the same ease
    for side in 0..2usize {
        let e = if side == 1 { w_r } else { w_l };
        if e < 0.003 {
            continue;
        }
        let fs = if side == arm { f } else { door_fingers(&tw, &n0, side) };
        let wrist = mad3(mad3(p_tgt, nn, 0.035), fs, -0.08);
        if !ask_blend(g, at, at, 1.0, false, at_b, n) {
            return 0;
        }
        let hh = &mut hs;
        with_pose(g, at_b, |d, rig, _n, fk| {
            let (sg, (sh_n, ua, fa, hd)) = (if side == 1 { 1.0 } else { -1.0 }, arm_of(side));
            fk_run(rig, d, fk.rot, fk.pos);
            let (live0, rot0) = (fk.at(hd), fk.rot(hd));
            let far = smoothstep(0.38, 0.54, dist3(&fk.at(ua), &wrist)) * e;
            if far > 1e-3 {
                // the shoulder reaches too: clavicle forward and a touch up
                turn(d, rig, sh_n, q_mul(&q_yaw(sg * 10.0 * DEGR * far), &q_roll(-sg * 3.0 * DEGR * far)));
                fk_run(rig, d, fk.rot, fk.pos);
            }
            // elbow down, out to the arm's side, a shade back; back of the hand toward him = palm flat on the face he pushes
            let rot = nlerp(&rot0, &hand_rot(&fix[side], fs, nn), e);
            two_bone_ik(rig, d, fk.rot, fk.pos, ua, fa, hd, &lerp3(&live0, &wrist, e), Some(&[-sg * 0.4, -1.0, -0.35]), e, Some(&rot), false);
            // open, the fingers a little apart, and the thumb up into the palm's plane
            hand(d, hh, side, &[0.1, 0.1, 0.15, 0.2], 0.1, 0.35);
            turn(d, rig, digits(side)[0] as usize, axis_angle(&q_rotate(&fk.rot(hd), &AY), sg * 40.0 * DEGR * e));
        });
        ask_mask(&[(arm_of(side).0, 1.0, true)]);
        if !ask_blend(g, at, at_b, smoothstep(0.0, 0.25, e), true, at, n) {
            return 0;
        }
    }
    hands_out(&hs);
    bench().args[L_IN..L_IN + 6].copy_from_slice(&cs);
    RAN | RAN_ALL
}
/// what a hand at work does: the wrist's spot, where the fingers point and the back of the hand faces, the fingers themselves, and whether
/// the spot is on his own body
#[derive(Clone, Copy)]
struct Shape {
    wrist: [f64; 3],
    f: [f64; 3],
    b: [f64; 3],
    curl: [f64; 4],
    thumb: f64,
    spread: f64,
    body: bool,
}
fn shape_mix(a: Shape, b: Shape, k: f64) -> Shape {
    if k <= 0.0 {
        return a;
    }
    if k >= 1.0 {
        return b;
    }
    Shape {
        wrist: lerp3(&a.wrist, &b.wrist, k),
        f: lerp3(&a.f, &b.f, k),
        b: lerp3(&a.b, &b.b, k),
        curl: [lerp(a.curl[0], b.curl[0], k), lerp(a.curl[1], b.curl[1], k), lerp(a.curl[2], b.curl[2], k), lerp(a.curl[3], b.curl[3], k)],
        thumb: lerp(a.thumb, b.thumb, k),
        spread: lerp(a.spread, b.spread, k),
        body: b.body,
    }
}
/// the thing a man works at, as his hands see it: the spot, out of its face, its right and its up (the twist still to come undone), where
/// the shoulders will be once he has bowed in, the verb and its clock
struct Work {
    p_t: [f64; 3],
    n_t: [f64; 3],
    r_t: [f64; 3],
    u_t: [f64; 3],
    sh_full: Option<[[f64; 3]; 2]>,
    un: Untwist,
    t: f64,
    dur: f64,
    kind: f64,
    dir: f64,
    main: usize,
    two: bool,
    done: bool,
    keys: [[f64; 2]; 4],
    nk: usize,
}
impl Work {
    fn at(&self, x: f64, y: f64, off: f64) -> [f64; 3] {
        mad3(mad3(mad3(self.p_t, self.r_t, x), self.u_t, y), self.n_t, off)
    }
    /// a palm on the face (the door hand's numbers)
    fn flat(&self, side: usize, x: f64, y: f64, off: f64) -> Shape {
        let f = normalize(add3(scale3(self.u_t, 0.9), scale3(self.r_t, (if side == 1 { 1.0 } else { -1.0 }) * 0.42)));
        Shape { wrist: mad3(mad3(self.at(x, y, off), self.n_t, 0.035), f, -0.08), f, b: self.n_t, curl: [0.1, 0.1, 0.15, 0.2], thumb: 0.1, spread: 0.35, body: false }
    }
    /// the same push from further off: the arm long, the fingers' pads on it from a little above
    fn tips(&self, side: usize, x: f64, y: f64, off: f64) -> Shape {
        let f = normalize(add3(scale3(self.n_t, -1.0), scale3(self.u_t, -0.3)));
        let b = normalize(add3(self.u_t, scale3(self.r_t, (if side == 1 { 1.0 } else { -1.0 }) * 0.25)));
        Shape { wrist: mad3(self.at(x, y, off), f, -0.17), f, b, curl: [0.12, 0.12, 0.16, 0.2], thumb: 0.25, spread: 0.3, body: false }
    }
    /// palm flat where the shoulder is near enough for it, else the long arm
    fn push_on(&self, fk: &Fk, side: usize, x: f64, y: f64, off: f64) -> Shape {
        let a = self.flat(side, x, y, off);
        let sh = match self.sh_full {
            Some(s) => s[side],
            None => fk.at(arm_of(side).1),
        };
        shape_mix(a, self.tips(side, x, y, off), smoothstep(0.48, 0.56, dist3(&sh, &a.wrist)))
    }
    /// a forefinger's tip at a spot, the wrist behind and a little under it
    fn poke(&self, side: usize, x: f64, y: f64, off: f64, curl: [f64; 4], thumb: f64) -> Shape {
        let f = normalize(add3(scale3(self.n_t, -1.0), scale3(self.u_t, 0.25)));
        let b = normalize(add3(self.u_t, scale3(self.r_t, if side == 1 { 0.3 } else { -0.3 })));
        Shape { wrist: mad3(self.at(x, y, off), f, -k(WORK_TIP_LEN)), f, b, curl, thumb, spread: 0.0, body: false }
    }
    /// what a hand does, by the verb (0 keys, 1 charge, 2 lever, 3 take); `fk` as it stands
    fn shape(&self, fk: &Fk, side: usize) -> Shape {
        let (sg, lead, t, dur) = (if side == 1 { 1.0 } else { -1.0 }, side == self.main, self.t, self.dur);
        if self.kind == 0.0 {
            let (mut kx, mut ky, mut press) = (self.keys[0][0], self.keys[0][1], 0.0);
            for i in 0..4usize {
                // (when key i of a code is pressed, on a verb `dur` long)
                let tk = k(WORK_KEY_LEAD) + (i as f64 + 0.5) * (dur - k(WORK_KEY_LEAD)) / 4.0;
                let key = self.keys[i % self.nk];
                if i != 0 {
                    let gl = smoothstep(tk - 0.19, tk - 0.06, t);
                    kx = lerp(kx, key[0], gl);
                    ky = lerp(ky, key[1], gl);
                }
                press += bump(t, tk - 0.07, tk - 0.005, tk + 0.03, tk + 0.1);
            }
            return self.poke(side, kx, ky, k(WORK_KEY_OFF) + k(WORK_HOVER) * (1.0 - press), [0.03, 0.8, 0.88, 0.92], 0.8);
        }
        if self.kind == 1.0 {
            let u = t / dur;
            let rub = 0.014 * sin(t * 6.5) * bump(u, 0.12, 0.25, 0.5, 0.6);
            if !lead {
                // the other hand holds it to the face
                return self.push_on(fk, side, sg * 0.05, -0.025 - rub, 0.022);
            }
            let press = self.push_on(fk, side, if self.two { sg * 0.05 } else { 0.0 }, 0.01 + rub, 0.022);
            // …then the fingers at the cap on its top edge
            return shape_mix(press, self.poke(side, 0.012 * sin(t * 9.0), 0.1, 0.03, [0.25, 0.35, 0.85, 0.9], 0.15), smoothstep(0.6, 0.72, u));
        }
        if self.kind == 2.0 {
            let thr = smoothstep(dur - 0.26, dur - 0.1, t);
            let jig = if dur > 1.2 { bump(t, 0.45, 0.55, 0.62, 0.8) + bump(t, 1.0, 1.1, 1.17, 1.35) } else { 0.0 };
            let gp = self.at(0.0, self.dir * (k(WORK_LEVER) * (1.0 - 2.0 * thr) - 0.012 * jig), 0.05);
            let f = normalize(add3(scale3(self.n_t, -1.0), scale3(self.u_t, -0.2)));
            // a fist over the handle from above
            return Shape { wrist: mad3(mad3(gp, self.n_t, 0.075), self.u_t, 0.03), f, b: self.u_t, curl: [0.7, 0.75, 0.75, 0.75], thumb: 0.55, spread: 0.0, body: false };
        }
        let u = clamp(t / dur, 0.0, 1.0);
        let out = 0.05 + 0.17 * smoothstep(0.55, 0.96, u) + 0.004 * sin(t * 13.0) * bump(u, 0.15, 0.25, 0.9, 1.0);
        let (gp, gr) = (self.at(if self.two { sg * 0.045 } else { 0.0 }, 0.0, out), smoothstep(0.12, 0.22, u) * 0.6);
        // fingers in, palms toward each other, thumbs up
        let b = normalize(add3(scale3(self.r_t, sg * 0.85), scale3(self.u_t, 0.5)));
        let hold = Shape { wrist: mad3(gp, self.n_t, 0.085), f: scale3(self.n_t, -1.0), b, curl: [gr, gr, gr + 0.05, gr + 0.1], thumb: 0.45, spread: 0.1, body: false };
        if !self.done || !lead {
            return hold;
        }
        // put away: the hand carries it down to the pouch at the front of that hip
        let hp = fk.at(role(HIPS));
        let hip = self.un.un([hp[0] - sg * 0.17, hp[1] + 0.08, hp[2] + 0.1]);
        shape_mix(hold, Shape { wrist: hip, f: [0.0, -1.0, 0.2], b: [-sg * 0.9, 0.0, 0.3], curl: [0.6, 0.6, 0.65, 0.7], thumb: 0.45, spread: 0.0, body: true }, smoothstep(dur, dur + 0.3, t))
    }
}
/// CharacterAnimator.workLayer — hands at work on a thing: the body bowed to a low thing and leaned into a long reach, the eyes onto it,
/// then each hand that is up by IK on a copy, its target travelling with the eased weight. Its own numbers: 0 dt · 1 live? · 2 the right
/// hand free for it? · 3, 4 the twist · 5, 6 the verb's clock and length · 7 the verb · 8 a lever's way · 9‥11 the spot · 12‥14 out of its
/// face · 15‥17 the gaze · 18‥21, 22‥25 the hands' fixes · 26 how many keys; the keys where a path of keys rides, two doubles each. Its
/// columns: the left and right weights' springs (value, speed), in reach? left and right, the lead hand, both hands?, the weight shown
pub fn work_layer(g: usize, at: usize, at_b: usize, at_rest: usize) -> u32 {
    let mut a = [0.0f64; 27];
    a.copy_from_slice(&bench().args[..27]);
    let mut cs = [0.0f64; 9];
    cs.copy_from_slice(&bench().args[L_IN..L_IN + 9]);
    let (dt, live, r_free, tw_y, tw_p, t, dur, kind, dir) = (a[0], a[1] != 0.0, a[2] != 0.0, a[3], a[4], a[5], a[6], a[7], a[8]);
    let (p0, gaze_local, fix) = ([a[9], a[10], a[11]], [a[15], a[16], a[17]], [[a[18], a[19], a[20], a[21]], [a[22], a[23], a[24], a[25]]]);
    let nk = a[26] as usize;
    if nk == 0 || nk > 4 {
        return 0;
    }
    let mut keys = [[0.0f64; 2]; 4];
    for (i, key) in keys.iter_mut().enumerate().take(nk) {
        key.copy_from_slice(&bench().args[KEYS_AT + i * 2..KEYS_AT + i * 2 + 2]);
    }
    let (w_was, done) = (clamp(max(cs[0], cs[2]), 0.0, 1.0), t > dur);
    if live && !done {
        // (which hands: settled while the verb runs, kept while it comes away)
        cs[6] = if r_free { 1.0 } else { 0.0 };
        cs[7] = if (kind == 1.0 || kind == 3.0) && r_free { 1.0 } else { 0.0 };
    }
    let (main, two) = (if cs[6] != 0.0 { 1usize } else { 0 }, cs[7] != 0.0);
    let n0 = normalize([a[12], a[13], a[14]]);
    // ---- the body: bowed to a low thing, leaned into a long reach (by last frame's weight; the head levelled back out of both), the eyes onto it
    let ax = cross3(&AY, &normalize([-n0[0], 0.0, -n0[2] + 1e-6]));
    let Some(Some((n, bow1, lean1))) = with_pose(g, at, |d, rig, n, fk| {
        if !(cast(n, &ARM_ROLES) && cast(n, &[HIPS, SPINE1, SPINE2, NECK, HEAD])) {
            return None;
        }
        fk_run(rig, d, fk.rot, fk.pos);
        let bow1 = 30.0 * DEGR * smoothstep(0.3, 0.8, (fk.at(role(UPPER_ARM_L))[1] + fk.at(role(UPPER_ARM_R))[1]) / 2.0 - p0[1]);
        let lean1 = 14.0 * DEGR * smoothstep(0.42, 0.66, dist3(&fk.at(arm_of(main).1), &mad3(p0, n0, 0.12)));
        Some((n, bow1, lean1))
    }) else {
        return 0;
    };
    let body = |d: &mut [S], rig: &Rig, bow: f64, lean: f64| {
        let q = |sh: f64| q_mul(&axis_angle(&ax, lean * sh), &q_pitch(bow * sh));
        turn(d, rig, role(SPINE1), q(0.4));
        turn(d, rig, role(SPINE2), q(0.35));
        turn(d, rig, role(SPINE3), q(0.25));
        turn(d, rig, role(NECK), q(-0.4));
        turn(d, rig, role(HEAD), q(-0.4));
    };
    // where the shoulders WILL be once he has bowed and leaned in: what the reach is asked of while the hands are still coming up
    let mut sh_full: Option<[[f64; 3]; 2]> = None;
    if live && !done && w_was < 0.97 && bow1 + lean1 > 1e-4 {
        if !ask_blend(g, at, at, 1.0, false, at_b, n) {
            return 0;
        }
        sh_full = with_pose(g, at_b, |d, rig, _n, fk| {
            body(d, rig, bow1, lean1);
            fk_run(rig, d, fk.rot, fk.pos);
            [fk.at(role(UPPER_ARM_L)), fk.at(role(UPPER_ARM_R))]
        });
    }
    let ease = w_was * if done { smoothstep(dur + 0.3, dur, t) } else { 1.0 };
    let (bow, lean) = (bow1 * ease, lean1 * ease);
    let Some(mut hs) = hands_in(g, at_rest) else {
        return 0;
    };
    let c = &mut cs;
    let Some((wk, w_l, w_r)) = with_pose(g, at, |d, rig, _n, fk| {
        if bow + lean > 1e-4 {
            body(d, rig, bow, lean);
        }
        if sh_full.is_some() || bow + lean > 1e-4 {
            fk_run(rig, d, fk.rot, fk.pos);
        }
        let un = Untwist::at(fk, tw_y, tw_p, true);
        let p_t = un.un(p0);
        let n_t = normalize(sub3(un.un(add3(p0, n0)), p_t));
        // the thing's right as he sees it (model −X is his right)
        let r0 = cross3(&AY, &n0);
        let r0 = if len3(&r0) < 1e-4 { [-1.0, 0.0, 0.0] } else { normalize(r0) };
        let r_t = normalize(sub3(un.un(add3(p0, r0)), p_t));
        let wk = Work { p_t, n_t, r_t, u_t: cross3(&n_t, &r_t), sh_full, un, t, dur, kind, dir, main, two, done, keys, nk };
        if !done {
            look_at(d, rig, fk, &gaze_local, wk.at(0.0, 0.0, 0.0), 0.75 * w_was, 0.0);
        }
        fk_run(rig, d, fk.rot, fk.pos);
        // ---- which arms are up, and can each reach (asked inside 0.57 m of the shoulder joint and ahead of it, let go past 0.6; the put-away is on his own body)
        let (mut tgt_l, mut tgt_r) = (0.0, 0.0);
        if live {
            for side in 0..2usize {
                // (one hand's verb: the lead hand alone; the other hand lets go once the thing is out)
                if (!two || done) && side != main {
                    continue;
                }
                let (sh, sp) = (fk.at(arm_of(side).1), wk.shape(fk, side));
                let (mut dd, mut ahead) = (dist3(&sh, &sp.wrist), sp.wrist[2] - sh[2]);
                if let Some(sf) = sh_full {
                    let df = dist3(&sf[side], &sp.wrist);
                    if df < dd {
                        dd = df;
                        ahead = max(ahead, sp.wrist[2] - sf[side][2]);
                    }
                }
                let ok = sp.body || if c[4 + side] != 0.0 { dd < 0.6 && dd > 0.14 && ahead > 0.0 } else { dd < 0.57 && dd > 0.18 && ahead > 0.04 };
                c[4 + side] = if ok { 1.0 } else { 0.0 };
                if ok {
                    if side == 1 {
                        tgt_r = 1.0;
                    } else {
                        tgt_l = 1.0;
                    }
                }
            }
        }
        let rate = |tg: f64, x: f64| if tg > x { 26.0 } else { 22.0 };
        let (rl, rr) = (rate(tgt_l, c[0]), rate(tgt_r, c[2]));
        spring(c, 0, tgt_l, rl, dt);
        spring(c, 2, tgt_r, rr, dt);
        let (w_l, w_r) = (clamp(c[0], 0.0, 1.0), clamp(c[2], 0.0, 1.0));
        c[8] = if main == 1 { w_r } else { w_l };
        (wk, w_l, w_r)
    }) else {
        return 0;
    };
    for side in 0..2usize {
        let e = if side == 1 { w_r } else { w_l };
        if e < 0.003 {
            continue;
        }
        // (the shape off the run-forward as it stands: the pose's own for the first hand, the first hand's copy for the second)
        let Some(sp) = with_pose(g, at, |_d, _rig, _n, fk| wk.shape(fk, side)) else {
            return 0;
        };
        if !ask_blend(g, at, at, 1.0, false, at_b, n) {
            return 0;
        }
        let hh = &mut hs;
        with_pose(g, at_b, |d, rig, _n, fk| {
            let (sg, (sh_n, ua, fa, hd)) = (if side == 1 { 1.0 } else { -1.0 }, arm_of(side));
            fk_run(rig, d, fk.rot, fk.pos);
            let (live0, rot0) = (fk.at(hd), fk.rot(hd));
            let far = smoothstep(0.38, 0.54, dist3(&fk.at(ua), &sp.wrist)) * e;
            if far > 1e-3 {
                // the shoulder reaches too (the door hand's turn)
                turn(d, rig, sh_n, q_mul(&q_yaw(sg * 10.0 * DEGR * far), &q_roll(-sg * 3.0 * DEGR * far)));
                fk_run(rig, d, fk.rot, fk.pos);
            }
            // elbow down and out
            let rot = nlerp(&rot0, &hand_rot(&fix[side], sp.f, sp.b), e);
            two_bone_ik(rig, d, fk.rot, fk.pos, ua, fa, hd, &lerp3(&live0, &sp.wrist, e), Some(&[-sg * 0.45, -1.0, -0.3]), e, Some(&rot), false);
            hand(d, hh, side, &sp.curl, sp.thumb, sp.spread);
        });
        ask_mask(&[(arm_of(side).0, 1.0, true)]);
        if !ask_blend(g, at, at_b, smoothstep(0.0, 0.25, e), true, at, n) {
            return 0;
        }
    }
    hands_out(&hs);
    bench().args[L_IN..L_IN + 9].copy_from_slice(&cs);
    RAN | RAN_ALL
}
const L_IN: usize = 64;
const BREATH: usize = 0;
const EXERT: usize = 1;
const PREV_S: usize = 2;
const ACCEL: usize = 3;
const MASS: usize = 4;
const MASS_V: usize = 6;
const STILL_T: usize = 8;
const SHIFT_T: usize = 9;
const SHIFT_SIDE: usize = 10;
const SHIFT: usize = 11;
const YAW_PREV: usize = 13;
const YAW_RATE: usize = 14;
const PREV_POS: usize = 15;
const ROOT_V: usize = 18;
const SHIFT_N: usize = 19;
const F_W: usize = 20;
const F_NEXT: usize = 21;
const F_L: usize = 22;
const F_R: usize = 31;
// …and within a foot
const FP: usize = 0;
const FYAW: usize = 3;
const FSWING: usize = 4;
const FFROM: usize = 5;
const FFROM_YAW: usize = 8;
const L_COLS: usize = 40;
#[inline(always)]
fn v3_of(st: &[f64], i: usize) -> [f64; 3] {
    [st[i], st[i + 1], st[i + 2]]
}
#[inline(always)]
fn v3_to(st: &mut [f64], i: usize, v: &[f64; 3]) {
    st[i] = v[0];
    st[i + 1] = v[1];
    st[i + 2] = v[2];
}
#[inline(always)]
fn dist_xz(a: &[f64; 3], b: &[f64; 3]) -> f64 {
    hypot2(a[0] - b[0], a[2] - b[2])
}

/// CharacterAnimator.lifeLayer — breathing, the body's mass on starts and stops, the weight shifting foot to foot at rest, and the standing foot lock: the feet
/// planted in the world while he stands, each stepping across in a lifted arc when the body has turned or shifted out from over it.
/// args: 0 dt · 1 s · 2 crouchPose · 3 w · 4 wF · 5 bodyYawIn · 6‥8 posIn · 9 crT; then from 64 the columns above, in and out:
/// breath · exert · prevS · accel · mass (x, v) · massV (x, v) · stillT · shiftT · shiftSide · shift (x, v) · yawPrev · yawRate · prevPos (3) · rootV · shiftN ·
/// feet.w · feet.next · the left foot, the right: p (3) · yaw · swing · from (3) · fromYaw
pub fn life_layer(g: usize, at: usize) -> u32 {
    let mut a = [0.0f64; L_IN + L_COLS];
    a.copy_from_slice(&bench().args[..L_IN + L_COLS]);
    let (dt, s, crouch_pose, w, w_f, yin, cr_t) = (a[0], a[1], a[2], a[3], a[4], a[5], a[9]);
    let p0 = [a[6], a[7], a[8]];
    let st = &mut a[L_IN..];
    let code = with_pose(g, at, |d, rig, n, fk| {
        if !cast(n, &[HIPS, SPINE1, SPINE2, SPINE3, NECK, SHOULDER_L, SHOULDER_R, THIGH_L, SHIN_L, FOOT_L, THIGH_R, SHIN_R, FOOT_R]) {
            return 0;
        }
        // clocks
        if !finite(st[YAW_PREV]) {
            st[YAW_PREV] = yin;
        }
        let yr = if dt > 0.0 { wrap_angle(yin - st[YAW_PREV]) / dt } else { 0.0 };
        st[YAW_PREV] = yin;
        st[YAW_RATE] = damp(st[YAW_RATE], clamp(yr, -8.0, 8.0), 8.0, dt);
        // the root's OWN planar motion: more than half a metre in a frame is a teleport — the feet re-lock where they land
        let mut teleported = false;
        if !finite(st[PREV_POS]) {
            v3_to(st, PREV_POS, &p0);
        } else {
            let dd = hypot2(p0[0] - st[PREV_POS], p0[2] - st[PREV_POS + 2]);
            teleported = dd > 0.5;
            st[ROOT_V] = if dt > 0.0 && !teleported {
                damp(st[ROOT_V], min(dd / dt, 8.0), 14.0, dt)
            } else if teleported {
                0.0
            } else {
                st[ROOT_V]
            };
            v3_to(st, PREV_POS, &p0);
        }
        let hb = truthy(HARDBREATH_ON);
        st[EXERT] = clamp(st[EXERT] + if s > 3.2 { dt / if hb { k(HARDBREATH_CHEST_UP) } else { 4.0 } } else { -dt / if hb { k(HARDBREATH_CHEST_DOWN) } else { 8.0 } }, 0.0, 1.0);
        let b_rate = lerp(k(LIFE_BREATH_RATE), if hb { k(HARDBREATH_CHEST_RATE) } else { k(LIFE_BREATH_RATE_RUN) }, st[EXERT]);
        let b_amp = lerp(k(LIFE_BREATH_AMP), if hb { k(HARDBREATH_CHEST_AMP) } else { k(LIFE_BREATH_AMP_RUN) }, st[EXERT]) * lerp(1.0, 0.35, clamp(s / 2.0, 0.0, 1.0)) * k(LIFE_ON);
        st[BREATH] = (st[BREATH] + dt * b_rate * 2.0 * PI) % (2.0 * PI);
        let accel = if dt > 0.0 { (s - st[PREV_S]) / dt } else { 0.0 };
        st[PREV_S] = s;
        st[ACCEL] = damp(st[ACCEL], clamp(accel, -12.0, 12.0), 10.0, dt);
        let t = clamp(-st[ACCEL] * k(LIFE_MASS_GAIN), -k(LIFE_MASS_BACK), k(LIFE_MASS_FWD)) * k(LIFE_ON);
        spring(st, MASS, t, 9.0, dt);
        let t = clamp(-abs(st[ACCEL]) * k(LIFE_MASS_DIP), -k(LIFE_MASS_DIP_MAX), 0.0) * k(LIFE_ON);
        spring(st, MASS_V, t, 10.0, dt);
        st[STILL_T] = if s < 0.12 && abs(st[YAW_RATE]) < 0.5 { st[STILL_T] + dt } else { 0.0 };
        let resting = if st[STILL_T] > k(LIFE_REST_AFTER) && k(LIFE_ON) > 0.0 && !(cr_t > 0.5) { 1.0 } else { 0.0 };
        if resting != 0.0 {
            st[SHIFT_T] -= dt;
            if st[SHIFT_T] <= 0.0 {
                st[SHIFT_SIDE] = if st[SHIFT_SIDE] > 0.0 { -1.0 } else { 1.0 };
                st[SHIFT_N] += 1.0;
                st[SHIFT_T] = k(LIFE_SHIFT_MIN) + ((st[SHIFT_N] * 0.618034) % 1.0) * k(LIFE_SHIFT_SPAN);
            }
        }
        let t = resting * st[SHIFT_SIDE];
        spring(st, SHIFT, t, if resting != 0.0 { k(LIFE_SHIFT_IN) } else { k(LIFE_SHIFT_OUT) }, dt);
        if w_f < 0.003 {
            st[F_W] = 0.0;
        }
        if w < 0.003 && w_f < 0.003 {
            return RAN;
        }
        // remember the feet, pose the body, put the feet back
        let (hips, s1, s2, s3, nk) = (role(HIPS), role(SPINE1), role(SPINE2), role(SPINE3), role(NECK));
        let (foot_l, foot_r) = (role(FOOT_L), role(FOOT_R));
        let legs = list(FK_LEGS);
        fk.part(rig, d, legs);
        let ran = RAN | ran_list(FK_LEGS);
        let (fl, fr, rl, rr) = (fk.at(foot_l), fk.at(foot_r), fk.rot(foot_l), fk.rot(foot_r));
        if w >= 0.003 {
            let (br, br2) = (sin(st[BREATH]), sin(st[BREATH] - 0.6));
            let sh = st[SHIFT] * w * min(1.0, k(LIFE_ON));
            let side = k(LIFE_SHIFT_TRAVEL) * sh * (1.0 - 0.5 * crouch_pose);
            let off = [side, st[MASS_V] * w + 0.004 * b_amp * br * w - abs(sh) * 0.006, st[MASS] * w * (1.0 - 0.4 * crouch_pose)];
            shift(d, rig, hips, &off);
            if abs(sh) > 1e-3 {
                turn(d, rig, hips, q_roll(-sh * k(LIFE_HIP_DROP) * DEGR));
                turn(d, rig, s2, q_roll(sh * k(LIFE_SPINE_COUNTER) * DEGR));
                turn(d, rig, nk, q_roll(sh * k(LIFE_NECK_COUNTER) * DEGR));
            }
            let tip = clamp(st[ACCEL] * k(LIFE_TIP_GAIN), -k(LIFE_TIP_BACK), k(LIFE_TIP_FWD)) * DEGR * w * (1.0 - 0.5 * crouch_pose) * k(LIFE_ON);
            if abs(tip) > 1e-4 {
                turn(d, rig, s1, q_pitch(tip * 0.5));
                turn(d, rig, s2, q_pitch(tip * 0.3));
                turn(d, rig, nk, q_pitch(-tip * 0.5));
            }
            let bp = b_amp * w * DEGR;
            turn(d, rig, s2, q_pitch(-0.5 * bp * br));
            turn(d, rig, s3, q_pitch(-0.4 * bp * br2));
            turn(d, rig, nk, q_pitch(0.5 * bp * br2));
            turn(d, rig, role(SHOULDER_L), q_roll(1.0 * 0.9 * bp * br2));
            turn(d, rig, role(SHOULDER_R), q_roll(-1.0 * 0.9 * bp * br2));
            fk.part(rig, d, legs);
        }
        // ---- feet: planted in the WORLD while he stands, released into the cycle's feet as he moves off
        let standing = if max(s, st[ROOT_V]) < 0.22 && w_f > 0.5 && k(FEET_ON) > 0.5 && !(cr_t > 0.0 && cr_t < 1.0) { 1.0 } else { 0.0 };
        let w_was = st[F_W];
        st[F_W] = damp(st[F_W], standing, if standing != 0.0 { 12.0 } else { 16.0 + 40.0 * st[ROOT_V] }, dt);
        if w < 0.003 && st[F_W] <= 0.01 {
            return ran;
        }
        let (cy, sy) = (cos(yin), sin(yin));
        let to_world = |m: &[f64; 3]| -> [f64; 3] { [p0[0] + m[0] * cy + m[2] * sy, p0[1] + m[1], p0[2] - m[0] * sy + m[2] * cy] };
        let to_model = |wp: &[f64; 3]| -> [f64; 3] {
            let (dx, dz) = (wp[0] - p0[0], wp[2] - p0[2]);
            [dx * cy - dz * sy, wp[1] - p0[1], dx * sy + dz * cy]
        };
        let foot_yaw = |r: &[f64; 4]| -> f64 {
            let f = q_rotate(r, &AY);
            atan2(f[0], f[2])
        };
        let (mut tl, mut tr, mut ql, mut qr) = (fl, fr, rl, rr);
        if st[F_W] > 0.01 {
            let want_p = [to_world(&fl), to_world(&fr)];
            let want_yaw = [yin + foot_yaw(&rl), yin + foot_yaw(&rr)];
            const FOOT: [usize; 2] = [F_L, F_R];
            let relock = w_was <= 0.01 || teleported || dist_xz(&v3_of(st, F_L + FP), &want_p[0]) > 0.45 || dist_xz(&v3_of(st, F_R + FP), &want_p[1]) > 0.45;
            if relock {
                for i in 0..2 {
                    v3_to(st, FOOT[i] + FP, &want_p[i]);
                    st[FOOT[i] + FYAW] = want_yaw[i];
                    st[FOOT[i] + FSWING] = -1.0;
                }
            }
            let (swing, lift, dist, turn_at, pivot, lag_cap) = (k(FEET_SWING), k(FEET_LIFT), k(FEET_DIST), k(FEET_TURN_DEG) * DEGR, k(FEET_PIVOT), k(FEET_LAG_CAP_DEG) * DEGR);
            let far_of = |st: &[f64], i: usize| -> bool { dist_xz(&v3_of(st, FOOT[i] + FP), &want_p[i]) > dist || abs(wrap_angle(st[FOOT[i] + FYAW] - want_yaw[i])) > turn_at };
            for i in 0..2 {
                let (f, o) = (FOOT[i], FOOT[1 - i]);
                if st[f + FSWING] < 0.0 {
                    let mut err_yaw = wrap_angle(want_yaw[i] - st[f + FYAW]);
                    if abs(err_yaw) > lag_cap {
                        st[f + FYAW] += sign(err_yaw) * (abs(err_yaw) - lag_cap);
                        err_yaw = sign(err_yaw) * lag_cap;
                    }
                    if abs(err_yaw) > turn_at {
                        st[f + FYAW] += sign(err_yaw) * min(abs(err_yaw) - turn_at, pivot * dt);
                    }
                    let my_turn = st[F_NEXT] == i as f64;
                    if far_of(st, i) && st[o + FSWING] < 0.0 && (my_turn || !far_of(st, 1 - i)) {
                        st[f + FSWING] = 0.0;
                        let p = v3_of(st, f + FP);
                        v3_to(st, f + FFROM, &p);
                        st[f + FFROM_YAW] = st[f + FYAW];
                        st[F_NEXT] = (1 - i) as f64;
                    }
                }
                if st[f + FSWING] >= 0.0 {
                    st[f + FSWING] += dt / swing;
                    let e = smoothstep(0.0, 1.0, min(1.0, st[f + FSWING]));
                    let from = v3_of(st, f + FFROM);
                    let mut p = [lerp(from[0], want_p[i][0], e), lerp(from[1], want_p[i][1], e), lerp(from[2], want_p[i][2], e)];
                    p[1] = want_p[i][1] + sin(PI * min(1.0, st[f + FSWING])) * lift;
                    v3_to(st, f + FP, &p);
                    st[f + FYAW] = st[f + FFROM_YAW] + wrap_angle(want_yaw[i] - st[f + FFROM_YAW]) * e;
                    if st[f + FSWING] >= 1.0 {
                        st[f + FSWING] = -1.0;
                        v3_to(st, f + FP, &want_p[i]);
                        st[f + FYAW] = want_yaw[i];
                    }
                }
            }
            let mk = |st: &[f64], f: usize, rest_p: &[f64; 3], rest_r: &[f64; 4]| -> ([f64; 3], [f64; 4]) {
                let mp = to_model(&v3_of(st, f + FP));
                let fw = st[F_W];
                let p = [lerp(rest_p[0], mp[0], fw), lerp(rest_p[1], mp[1], fw), lerp(rest_p[2], mp[2], fw)];
                let dyaw = wrap_angle((st[f + FYAW] - yin) - foot_yaw(rest_r)) * fw;
                (p, q_mul(&q_yaw(dyaw), rest_r))
            };
            (tl, ql) = mk(st, F_L, &fl, &rl);
            (tr, qr) = mk(st, F_R, &fr, &rr);
        }
        // put the feet back: the knee steered over the toes only as far as the feet are LOCKED
        let pole_w = if cr_t > 0.0 { 0.6 * st[F_W] * (1.0 - smoothstep(0.0, 1.0, cr_t)) } else { 0.6 * st[F_W] };
        two_bone_ik(rig, d, fk.rot, fk.pos, role(THIGH_L), role(SHIN_L), foot_l, &tl, Some(&knee_pole(&ql)), pole_w, Some(&ql), false);
        two_bone_ik(rig, d, fk.rot, fk.pos, role(THIGH_R), role(SHIN_R), foot_r, &tr, Some(&knee_pole(&qr)), pole_w, Some(&qr), true);
        ran | RAN_ALL
    })
    .unwrap_or(0);
    if code != 0 {
        bench().args[L_IN..L_IN + L_COLS].copy_from_slice(&a[L_IN..]);
    }
    code
}
