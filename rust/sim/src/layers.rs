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
//! A LAYER THAT KEEPS STATE between ticks (the breath's phase, a spring, where a foot was put down) is handed it as doubles after its own numbers on the
//! bench's `args`, in an order its comment gives, and leaves it there as it now stands: the layer's COLUMNS of one character. Who keeps the columns between
//! ticks is not the layer's business (today the TypeScript animator's own fields, which every save already carries).
use crate::knobs::*;
use crate::maths::{atan2, cos, exp, hypot2, hypot3, sin};
use crate::single::{get, put, S};
use crate::skeleton::{bench, parent_rot, rotate_model, two_bone_ik, with_pose, Rig};

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

// the life layer's columns (after its own sixteen numbers): the breath and the body's mass, then the foot lock, then a foot (two of them)
const L_IN: usize = 16;
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
/// args: 0 dt · 1 s · 2 crouchPose · 3 w · 4 wF · 5 bodyYawIn · 6‥8 posIn · 9 crT; then from 16 the columns above, in and out:
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
