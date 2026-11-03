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
//! A LAYER THAT KEEPS STATE between ticks (the breath's phase, a spring, where a foot was put down) is handed it as doubles after its own numbers (from the thirty-second double) on the
//! bench's `args`, in an order its comment gives, and leaves it there as it now stands: the layer's COLUMNS of one character. Who keeps the columns between
//! ticks is not the layer's business (today the TypeScript animator's own fields, which every save already carries).
use crate::knobs::*;
use crate::maths::{acos, asin, atan2, cos, exp, hypot2, hypot3, hypot4, sin};
use prim::{floor, sqrt};
use crate::single::{get, put, S};
use crate::skeleton::{ask_blend, bench, fk_run, parent_rot, rotate_local, rotate_model, set_model_rot, two_bone_ik, with_pose, Fk, Rig};

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
/// args: 0 dt · 1 moveW · 2 standW · 3 trained · 4 longGun (1 or 0) · 5 dip · 6 the long gun's weight (lg.w.x) · 7 upperW.aim; then from 32 its columns:
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
/// the feet about the gait's middle. args: 0 ph · the clip's pelvis height at the two samples about the phase: 1, 2 · 3 its mean · 4 runK · 5 jogMid
pub fn jog_shape(g: usize, at: usize) -> u32 {
    let a = bench().args;
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
/// it about the ankles' floor, never an ankle so low its toe goes through the floor. args: 0 sprintMid · 1 sprintFloorY
pub fn sprint_shape(g: usize, at: usize) -> u32 {
    let a = bench().args;
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
/// 9 the sprint's right; then from 32 its columns: the left foot's dev · pz · the right's dev · pz
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
/// 19 creepAt is there (1 or 0) · 20‥22 creepAt (world) · 23 bodyYawIn · 24‥26 posIn · 27‥29 gazeLocal; then from 32 its columns, the heft's springs:
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
/// sole's rotation; then from 32 its columns, where the kick found the feet: 0 they are set (1 or 0) · 1‥3 the left's spot · 4‥7 its rotation · 8‥10 the
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

const L_IN: usize = 32;
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
/// args: 0 dt · 1 s · 2 crouchPose · 3 w · 4 wF · 5 bodyYawIn · 6‥8 posIn · 9 crT; then from 32 the columns above, in and out:
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
