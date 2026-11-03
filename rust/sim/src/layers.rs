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
