//! THE ANIMATOR'S LAYERS: the steps of a man's update that pose him, each written once, here. A layer takes his animation as it is (`an`), the numbers the game is
//! tuned by (`tune`) and what only this call knows, reads and writes the pose WHERE IT LIVES (the game's pool, poses.rs) and the rig on the bench (skeleton.rs), and
//! asks the rig's named nodes of roles.rs. What it keeps from one tick to the next it keeps on him.
//!
//! A layer answers 0 = not a pose of this pool, or a role with no node (the caller stops, by name) · else RAN, with a bit for each list of nodes it ran forward on
//! the bench (`ran_list`), whose rotations and origins the caller reads back.
use game_types::character::{KICK_IMPACT, KICK_PLANT, LONG_ANCHOR_SIDE_MAX, LONG_KICK_BACK, LONG_PULL_MAX, READY_RATE, READY_REACH, THROW_DUR, THROW_RELEASE, TOE_FLOOR_Y, TOSS_AFTER, TOSS_LOW, TOSS_TOP, WALL_GAIT_REST, WALL_ROOT_OFF, BIGMOVE, BLOW, BOB, CREEP, FEET, GAZE, GAZE_EL, HAND, KNIFE_BLOW, LEAN_POSE, LIFE, LONG_PIVOT, LONG_STEADY, MV2, MV_FLINCH_ENV, MV_HIT_ENV, MV_STARTLE, REACH, READY_AZ, READY_BARE, READY_UP, RUN_LINGER, SMOKEARM, SNEAK, SOFT_KNEE, SPRINT_WARP, STEALTH, SWIV_REF_L, SWIV_REF_R, TAKEN, TOSS_HANG, WALL_POSE, WORK};
use game_types::rig_props::{Adj as RigAdj, SlingFrontHand, BIGRIG, RIG};
use game_types::character::{Anim, AnimBigTool, AnimBigToolKind, AnimFoot, AnimFootSpot, AnimStep, AnimHdFix, AnimKickFrom, AnimKickLeg, AnimLongGrip, AnimSp, AnimThrLast, AnimWtk, BlowRun, BlowStyle, Character, CharacterTn, DoorPush, HaulPose, HeldPose, HoldPhase, HoldPose, HoldVariant, KitItem, KnifeSlash, MvGait, ReadyKind, ReadyShape, SignalKind, Spring, Stance, TakenStyle, ThrowTables as Tables, WorkKind, WorkPose};
use game_types::gunreload::{FeedKey, FeedKeyAt, FeedKeyHow, FeedMarks};
use game_types::knobs::Knobs;
use crate::roles::*;
use crate::maths::{acos, asin, atan2, cos, exp, hypot2, hypot3, hypot4, pow, sin};
use prim::{floor, sqrt};
use crate::single::{get, put, S};
use crate::skeleton::{NODE, ask_blend as ask_blend_how, ask_mask, ask_sample as ask_sample_how, bench, fk_run, parent_rot, rotate_local, rotate_model, set_model_rot, set_unit, two_bone_ik, with_pose, Fk, Rig};

/// a layer ran
pub const RAN: u32 = 1;
/// …and left ALL of the run-forward bones on the bench current for the pose (a leg or an arm solved: skeleton.rs two_bone_ik)
pub const RAN_ALL: u32 = 2;
/// …and ran list `i` of roles.rs forward on the bench
pub const fn ran_list(i: usize) -> u32 {
    4 << i
}

const PI: f64 = 3.141592653589793;
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
/// one step of a spring the animator keeps
fn spring_step(s: &mut Spring, target: f64, w: f64, dt: f64) {
    let mut d = [s.x, s.v];
    spring(&mut d, 0, target, w, dt);
    (s.x, s.v) = (d[0], d[1]);
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
/// A LAYER'S BLEND AND ITS SAMPLE (skeleton.rs ask_blend, ask_sample) are worked as the tick's knob says: in singles under POSEMATH.singles, as the host's
/// own are (anim/skeleton.ts blendPose, sampleClip read the same switch as they ask)
#[inline(always)]
fn ask_blend(g: usize, a: usize, b: usize, w: f64, masked: bool, out: usize, n: usize, tune: &Knobs) -> bool {
    ask_blend_how(g, a, b, w, masked, out, n, tune.posemath.singles)
}
#[inline(always)]
fn ask_sample(g: usize, at: usize, nodes: usize, c: usize, t: f64, looped: bool, tune: &Knobs) -> bool {
    ask_sample_how(g, at, nodes, c, t, looped, tune.posemath.singles)
}
/// A POSE'S SINGLES, and the one place in this file that says where one lies: node i's channel c (0‥2 its origin, 3‥6 its rotation, 7‥9 its
/// scale). Every layer below reads and writes a pose through these two, so a pose laid out another way is these two lines. A POSE IS TEN COLUMNS, A NODE A
/// ROW (skeleton.rs `rows`): a layer is handed exactly a pose's singles, so its rows are their count over NODE.
#[inline(always)]
fn ch(d: &[S], i: usize, c: usize) -> f64 {
    get(d, c * (d.len() / NODE) + i)
}
#[inline(always)]
fn set_ch(d: &mut [S], i: usize, c: usize, v: f64) {
    put(d, c * (d.len() / NODE) + i, v)
}
/// HOLD.lower's number, or HOLD.beats.lower's while the hold is played in beats
#[inline(always)]
fn lower_k(beats: bool, in_beats: f64, plain: f64) -> f64 {
    if beats {
        in_beats
    } else {
        plain
    }
}
/// CharacterAnimator.shift: node i's origin moved by a model-space offset
fn shift(d: &mut [S], rig: &Rig, i: usize, off: &[f64; 3]) {
    let p = parent_rot(rig, d, i);
    let l = q_rotate(&[-p[0], -p[1], -p[2], p[3]], off);
    let (x, y, z) = (ch(d, i, 0) + l[0], ch(d, i, 1) + l[1], ch(d, i, 2) + l[2]);
    set_ch(d, i, 0, x);
    set_ch(d, i, 1, y);
    set_ch(d, i, 2, z);
}
/// quat.axisAngle about the model's X, Y, Z: the products with the axis's noughts are written out, since −0 is a number too
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

/// The trunk's share of MOVES, straight into the pose.
pub fn mv_trunk(g: usize, an: &Anim, w: f64, o: MvGait, aim_pass: bool, tune: &Knobs) -> u32 {
    let (og, ou, ost, phase, speed, rock, sho, head) = (o.g, o.u, o.st, an.phase, an.speed, an.mvs.rock, an.mvs.sho, an.mvs.head);
    with_pose(g, an.p_a.at as usize, |d, rig, n, _fk| {
        if !cast(n, &[SPINE1, SPINE2, SPINE3, NECK, HEAD]) {
            return 0;
        }
        if w < 0.003 {
            return RAN;
        }
        let (s1, s2, s3, nk, hd) = (role(SPINE1), role(SPINE2), role(SPINE3), role(NECK), role(HEAD));
        let (mut lean, mut chin, mut sway, mut roll);
        if aim_pass {
            lean = tune.mv.gait.aim_lean * ost + tune.mv.gait.lean * 0.5 * og - tune.mv.gait.back * ou;
            chin = tune.mv.gait.chin * ou;
            (sway, roll) = (0.0, 0.0);
        } else {
            lean = tune.mv.gait.lean * og;
            let ph = 2.0 * PI * phase;
            let mw = smoothstep(0.2, 0.8, abs(speed));
            sway = tune.mv.gait.slouch_sway * ou * mw * sin(ph);
            roll = tune.mv.gait.slouch_roll * ou * mw * cos(ph);
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
        if (tune.moves.on > 0.5) && (tune.moves.recoil > 0.5) && (rock > 1e-4 || sho > 1e-4) {
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

/// The long gun, trunk half: the run's pumping of the shoulder line taken back out down the spine (about the model's upright, or by the sprint's share
/// about the trunk's own line), the upper spine bladed toward the gun, the lean, the head set to the stock.
pub fn long_body(g: usize, an: &Anim, w: f64, run: f64, sprint: f64, ch: &[f64; LC_COUNT], tune: &Knobs) -> u32 {
    let (long_steady, c_blade, c_lean, c_hy, c_hp, c_hr) = (an.long_steady, ch[LC_BLADE], ch[LC_LEAN], ch[LC_HEAD_YAW], ch[LC_HEAD_PITCH], ch[LC_HEAD_ROLL]);
    with_pose(g, an.p_a.at as usize, |d, rig, n, fk| {
        if !cast(n, &[SPINE1, SPINE2, SPINE3, NECK, HEAD, SHOULDER_L, SHOULDER_R, UPPER_ARM_L, UPPER_ARM_R]) {
            return 0;
        }
        if w < 0.003 {
            return RAN;
        }
        let (s1, s2, s3, nk, hd) = (role(SPINE1), role(SPINE2), role(SPINE3), role(NECK), role(HEAD));
        let mut ran = RAN;
        let hold = run * clamp(long_steady, 0.0, 1.0) * LONG_STEADY.run_yaw * w;
        if hold > 0.003 {
            fk.part(rig, d, list(FK_ARMS));
            ran |= ran_list(FK_ARMS);
            let (a, b) = (fk.at(role(UPPER_ARM_L)), fk.at(role(UPPER_ARM_R)));
            // (by the SPRINT's share of the pose: a jog is held as it always was, to the bit)
            let lv = if (tune.moves.on > 0.5) && (tune.moves.steady > 0.5) && (tune.moves.steady_run > 0.5) { clamp(sprint, 0.0, 1.0) } else { 0.0 };
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

/// A row of MOVES, under its master switch
#[inline(always)]
fn mv_on(row: f64, tune: &Knobs) -> bool {
    tune.moves.on > 0.5 && row > 0.5
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

/// Before the stances and the arms: the recoil's sums bled off (each at its own rate, by training), the muzzle's dip past a friend's back, and the
/// pelvis's share of how a side carries itself: a trained man with a long gun sinks onto bent knees when he moves and most of the walk's bob is taken
/// out, the feet held where the cycle has them.
pub fn mv_pre(g: usize, an: &mut Anim, dt: f64, move_w: f64, stand_w: f64, tune: &Knobs) -> u32 {
    let (at, trained, long_gun, lg_w, aim) = (an.p_a.at as usize, an.trained, an.long_gun, an.lg.w.x, an.upper_w.aim);
    let (dip, m, sp, o) = (an.dip, &mut an.mvs, &mut an.mv2.dip, &mut an.mv_out);
    with_pose(g, at, |d, rig, n, fk| {
        if !cast(n, &LEG_ROLES) {
            return 0;
        }
        let t = clamp(trained, 0.0, 1.0);
        let kk = |p0: f64, p1: f64| lerp(p0, p1, t);
        let sum = m.back + m.climb + abs(m.yaw) + m.rock + m.sho + m.head;
        if sum > 0.0 {
            if sum < 1e-4 {
                (m.back, m.climb, m.yaw, m.rock, m.sho, m.head) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
            } else {
                let (eb, ec, er) = (exp(-dt / kk(tune.mv.recoil.back_tau[0], tune.mv.recoil.back_tau[1])), exp(-dt / kk(tune.mv.recoil.climb_tau[0], tune.mv.recoil.climb_tau[1])), exp(-dt / kk(tune.mv.recoil.rock_tau[0], tune.mv.recoil.rock_tau[1])));
                m.back *= eb;
                m.climb *= ec;
                m.yaw *= ec;
                m.rock *= er;
                m.sho *= er;
                m.head *= er;
            }
        }
        // MOVES.entry: the muzzle down past a friend's back, and up again the moment he is clear of it
        let want = if mv_on(tune.moves.entry, tune) && long_gun { clamp(dip, 0.0, 1.0) } else { 0.0 };
        if want > 0.0 || sp.x != 0.0 {
            let rate = if want > sp.x { MV2.dip.rate_in } else { MV2.dip.rate_out };
            spring_step(sp, want, rate, dt);
            if want == 0.0 && sp.x < 0.01 {
                sp.x = 0.0;
                sp.v = 0.0;
            }
        }
        an.mv2_shown.dip = clamp(sp.x, 0.0, 1.0);
        an.sights_lift = 0.0;
        o.g = 0.0;
        o.u = 0.0;
        o.st = 0.0;
        let mut ran = RAN;
        if mv_on(tune.moves.gait, tune) {
            let (lgw, tw) = (clamp(lg_w, 0.0, 1.0), smoothstep(0.5, 1.0, trained));
            spring_step(&mut m.glide, tw * move_w * lgw, 9.0, dt);
            o.g = clamp(m.glide.x, 0.0, 1.0);
            o.u = (1.0 - smoothstep(0.0, 0.5, trained)) * lgw * stand_w;
            o.st = tw * lgw * stand_w;
            let drop = tune.mv.gait.drop * o.g + tune.mv.gait.aim_drop * o.st * aim * (1.0 - o.g);
            if drop > 2e-4 || o.g > 0.003 {
                fk.part(rig, d, list(FK_LEGS));
                let hy = fk.at(role(HIPS))[1];
                m.hip_y = if finite(m.hip_y) { m.hip_y + (hy - m.hip_y) * (1.0 - exp(-dt / 0.35)) } else { hy };
                let flat = clamp((m.hip_y - hy) * tune.mv.gait.bob * o.g, -0.03, 0.03);
                mv_hips(d, rig, fk, &[0.0, flat - drop, 0.0]);
                ran |= RAN_ALL;
            } else {
                m.hip_y = f64::NAN;
            }
        }
        o.live = o.g > 0.003 || o.u > 0.003 || o.st > 0.003 || m.rock > 1e-4 || m.sho > 1e-4;
        ran
    })
    .unwrap_or(0)
}

/// The stance, body half: the pelvis bladed toward the wall and settled onto soft knees over feet put back exactly where the locomotion has them, the
/// upper spine countering, a lean spread hips-first with the head levelled back, the stance's own head yaw and chin.
pub fn stance_body(g: usize, an: &Anim, w: f64, speed: f64, stand: f64) -> u32 {
    let (s_blade, s_squat, s_lean, s_hp, s_hy, press) = (an.st.blade.x, an.st.squat.x, an.st.lean.x, an.st.head_pitch.x, an.st.head_yaw.x, an.stack_press_sm);
    with_pose(g, an.p_a.at as usize, |d, rig, n, fk| {
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
fn jog_shape(g: usize, at: usize, an: &Anim, ph: f64) -> u32 {
    let i0 = (floor(ph * 64.0) as i32 & 63) as usize;
    let (y0, y1, mean, run_k, mid) = (an.jog_hips_y[i0] as f64, an.jog_hips_y[(i0 + 1) & 63] as f64, an.jog_hips_mean, an.run_k, an.jog_mid);
    with_pose(g, at, |d, rig, n, fk| {
        if !cast(n, &LEG_ROLES) {
            return 0;
        }
        let fi = ph * 64.0;
        let fr = fi - floor(fi);
        let hy = y0 * (1.0 - fr) + y1 * fr;
        shift(d, rig, role(HIPS), &[0.0, -(hy - mean) * (1.0 - BOB.keep), 0.0]);
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
fn sprint_shape(g: usize, at: usize, an: &Anim) -> u32 {
    let (mid, y0) = (an.sprint_mid, an.sprint_floor_y);
    with_pose(g, at, |d, rig, n, fk| {
        if !cast(n, &LEG_ROLES) || !cast(n, &[TOE_L, TOE_R]) {
            return 0;
        }
        let kk = SPRINT_WARP.stride;
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
            if y + toe_off < TOE_FLOOR_Y {
                y = TOE_FLOOR_Y - toe_off;
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
fn clip_at(g: usize, at: usize, nodes: usize, clip: f64, t: f64, tune: &Knobs) -> bool {
    clip >= 0.0 && ask_sample(g, at, nodes, clip as usize, t, true, tune)
}
/// CharacterAnimator.sampleJog — the jog clip at phase `ph` (its time `t`), its SEAM closed: the clip's last key is not its first, and each joint that
/// jumps there is carried half the jump back at the cycle's end and half forward at its start (the rig's table JOG_SEAM, six numbers a joint: its
/// node · 0 an origin, 1 a rotation · the jump: an offset, or an axis and, last, its angle)
fn sample_jog(g: usize, at: usize, nodes: usize, clip: f64, t: f64, ph: f64, tune: &Knobs) -> bool {
    if !clip_at(g, at, nodes, clip, t, tune) {
        return false;
    }
    let kk = ph - floor(ph) - 0.5;
    with_pose(g, at, |d, _rig, n, _fk| {
        for s in nums(JOG_SEAM).chunks_exact(6) {
            // (a seam's node; which of its channels moves is s[1], an origin or a rotation)
            let i = s[0] as usize;
            if i >= n {
                continue;
            }
            if s[1] == 0.0 {
                let (x, y, z) = (ch(d, i, 0) + s[2] * kk, ch(d, i, 1) + s[3] * kk, ch(d, i, 2) + s[4] * kk);
                set_ch(d, i, 0, x);
                set_ch(d, i, 1, y);
                set_ch(d, i, 2, z);
            } else {
                let q = q_mul(&own_rot(d, i), &axis_angle(&[s[2], s[3], s[4]], s[5] * kk));
                set_own_rot(d, i, &q);
            }
        }
    })
    .is_some()
}
/// a clip by its number, and where in it
#[derive(Clone, Copy)]
pub struct At {
    pub clip: f64,
    pub t: f64,
}
/// how much of each gait the stand pose shows, and the clips of them
pub struct Cycle {
    pub ph: f64,
    pub w_idle: f64,
    pub w_walk: f64,
    pub w_jog: f64,
    pub w_sprint: f64,
    pub idle: At,
    pub jog: At,
    pub sprint: At,
    pub walk: At,
}
/// The standing locomotion: the idle, and over it the cycle (the walk, the jog laid on it, or the jog and the sprint laid together), each run shaped as
/// the game shows it before the blend. The cycle's own poses are left in the scratch poses for the layers that read them.
pub fn stand_pose(g: usize, an: &Anim, c: &Cycle, tune: &Knobs) -> u32 {
    let (at, at_b, at_c) = (an.p_a.at as usize, an.p_b.at as usize, an.p_c.at as usize);
    let (ph, w_idle, w_walk, w_jog, w_sprint) = (c.ph, c.w_idle, c.w_walk, c.w_jog, c.w_sprint);
    let Some(n) = with_pose(g, at, |_d, _rig, n, _fk| n) else {
        return 0;
    };
    let moving = w_walk + w_jog + w_sprint > 0.001;
    let mut code = RAN;
    // (the idle is not sampled when the cycle's pose is about to replace all of it)
    if !(moving && 1.0 - w_idle >= 1.0) && !clip_at(g, at, n, c.idle.clip, c.idle.t, tune) {
        return 0;
    }
    if moving {
        let mut shaped = |s: u32| {
            code |= s;
            s != 0
        };
        if w_sprint > 0.001 {
            if !sample_jog(g, at_b, n, c.jog.clip, c.jog.t, ph, tune) || !shaped(jog_shape(g, at_b, an, ph)) {
                return 0;
            }
            if !clip_at(g, at_c, n, c.sprint.clip, c.sprint.t, tune) || !shaped(sprint_shape(g, at_c, an)) || !ask_blend(g, at_b, at_c, w_sprint, false, at_b, n, tune) {
                return 0;
            }
        } else {
            if !clip_at(g, at_b, n, c.walk.clip, c.walk.t, tune) {
                return 0;
            }
            if w_jog > 0.001 && (!sample_jog(g, at_c, n, c.jog.clip, c.jog.t, ph, tune) || !shaped(jog_shape(g, at_c, an, ph)) || !ask_blend(g, at_b, at_c, w_jog, false, at_b, n, tune)) {
                return 0;
            }
        }
        if !ask_blend(g, at, at_b, 1.0 - w_idle, false, at, n, tune) {
            return 0;
        }
    }
    code
}
/// The crouch over the stand pose: its idle, the crouch walk's cycle laid on it by how fast he goes, and that laid over the pose by the crouch shown.
pub fn crouch_pose(g: usize, an: &Anim, w_move: f64, crouch: f64, idle: At, fwd: At, tune: &Knobs) -> u32 {
    let (at, at_b, at_c) = (an.p_a.at as usize, an.p_b.at as usize, an.p_c.at as usize);
    let Some(n) = with_pose(g, at, |_d, _rig, n, _fk| n) else {
        return 0;
    };
    if !(w_move >= 1.0) && !clip_at(g, at_b, n, idle.clip, idle.t, tune) {
        return 0;
    }
    if w_move > 0.001 && (!clip_at(g, at_c, n, fwd.clip, fwd.t, tune) || !ask_blend(g, at_b, at_c, w_move, false, at_b, n, tune)) {
        return 0;
    }
    if !ask_blend(g, at, at_b, crouch, false, at, n, tune) {
        return 0;
    }
    RAN
}

#[inline(always)]
fn dist3(a: &[f64; 3], b: &[f64; 3]) -> f64 {
    hypot3(a[0] - b[0], a[1] - b[1], a[2] - b[2])
}
/// SOFT_KNEE: each leg re-solved toward its own ankle with the hip→ankle reach eased below full extension.
pub fn soft_knees(g: usize, at: usize, w: f64) -> u32 {
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
            if r <= SOFT_KNEE.from {
                continue;
            }
            let span = SOFT_KNEE.to - SOFT_KNEE.from;
            let x = (r - SOFT_KNEE.from) / span;
            let r_eased = SOFT_KNEE.from + span * (1.0 - exp(-x));
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

/// A run carries its arms close: each hand read in the shoulder line's own frame (to his left along the two shoulder joints,
/// level; up; forward), its sideways travel drawn in by the run's share of the standing pose, its fore-aft travel lengthened and its front lifted by the
/// sprint's, and the arm solved to it with the elbow behind him. The hand follows its forearm.
pub fn run_arms(g: usize, at: usize, w: f64, ws: f64, tune: &Knobs) -> u32 {
    with_pose(g, at, |d, rig, n, fk| {
        if !cast(n, &ARM_ROLES) {
            return 0;
        }
        fk.part(rig, d, list(FK_ARMS));
        let (a, b) = (fk.at(role(UPPER_ARM_L)), fk.at(role(UPPER_ARM_R)));
        let (mut lx, mut lz) = (a[0] - b[0], a[2] - b[2]);
        let ll = hypot2(lx, lz);
        if ll < 1e-4 {
            return RAN | ran_list(FK_ARMS);
        }
        lx /= ll;
        lz /= ll;
        // (forward, level: his left × up)
        let (fx, fz, keep, pump) = (-lz, lx, lerp(1.0, tune.runarms.keep, w), lerp(1.0, tune.runarms.pump, ws));
        // (neither solve moves what the other reads: the second finds the skeleton current)
        for right in [false, true] {
            let sg = if right { -1.0 } else { 1.0 };
            let (ua, fa, ha) = if right { (role(UPPER_ARM_R), role(FOREARM_R), role(HAND_R)) } else { (role(UPPER_ARM_L), role(FOREARM_L), role(HAND_L)) };
            let (s, h) = (fk.at(ua), fk.at(ha));
            let (dx, dz) = (h[0] - s[0], h[2] - s[2]);
            let (out, fwd) = ((dx * lx + dz * lz) * sg, dx * fx + dz * fz);
            let (o, f) = (tune.runarms.at + (out - tune.runarms.at) * keep, tune.runarms.mid + (fwd - tune.runarms.mid) * pump);
            let y = h[1] - s[1] + tune.runarms.lift * ws * clamp((fwd - tune.runarms.mid) / tune.runarms.swing, 0.0, 1.0);
            let to = [s[0] + lx * sg * o + fx * f, s[1] + y, s[2] + lz * sg * o + fz * f];
            let pole = [lx * sg * tune.runarms.pole[0] + fx * tune.runarms.pole[2], tune.runarms.pole[1], lz * sg * tune.runarms.pole[0] + fz * tune.runarms.pole[2]];
            two_bone_ik(rig, d, fk.rot, fk.pos, ua, fa, ha, &to, Some(&pole), tune.runarms.pole_w * w, None, right);
        }
        RAN | RAN_ALL
    })
    .unwrap_or(0)
}

#[inline(always)]
fn bump(t: f64, a: f64, b: f64, c: f64, d: f64) -> f64 {
    smoothstep(a, b, t) * (1.0 - smoothstep(c, d, t))
}
/// The run's stance feet: through each foot's stance window of the sampled phase the toe is put on the floor and carried back at the ground's speed, the
/// offset it is left holding bled away up the swing.
pub fn run_plant(g: usize, an: &mut Anim, ph: f64, s: f64, dt: f64, w_run: f64, w_sprint: f64) -> u32 {
    let (jog, sprint, reverse) = (&an.run_gait.jog, &an.run_gait.sprint, an.reverse);
    let mids = [lerp(jog.c_l, sprint.c_l, w_sprint), lerp(jog.c_r, sprint.c_r, w_sprint)];
    let mut plant = [&mut an.plant_l, &mut an.plant_r];
    with_pose(g, an.p_a.at as usize, |d, rig, n, fk| {
        if !cast(n, &LEG_ROLES) || !cast(n, &[TOE_L, TOE_R]) {
            return 0;
        }
        let dz = if reverse { 1.0 } else { -1.0 } * s * dt;
        fk.part(rig, d, list(FK_LEGS));
        let legs = [(role(THIGH_L), role(SHIN_L), role(FOOT_L), role(TOE_L)), (role(THIGH_R), role(SHIN_R), role(FOOT_R), role(TOE_R))];
        let mut cur = false;
        for i in 0..2 {
            let (thigh, shin, foot, toe) = legs[i];
            let mut x = ph - mids[i];
            x -= round(x);
            let w = bump(x, RUN_LINGER.on[0], RUN_LINGER.on[1], RUN_LINGER.off[0], RUN_LINGER.off[1]) * w_run;
            let tp = fk.at(toe);
            let miss = if finite(plant[i].pz) { dz - (tp[2] - plant[i].pz) } else { 0.0 };
            plant[i].pz = tp[2];
            plant[i].dev = clamp((plant[i].dev + if abs(miss) < 0.2 { miss } else { 0.0 } * w) * exp(-(1.0 - w) * dt / max(RUN_LINGER.bleed, 1e-3)), -0.25, 0.25);
            if w < 1e-3 && abs(plant[i].dev) < 5e-4 {
                plant[i].dev = 0.0;
                continue;
            }
            let (ap, ar) = (fk.at(foot), fk.rot(foot));
            two_bone_ik(rig, d, fk.rot, fk.pos, thigh, shin, foot, &[ap[0], ap[1] - (tp[1] - TOE_FLOOR_Y) * w, ap[2] + plant[i].dev], Some(&knee_pole(&ar)), 0.0, Some(&ar), cur);
            cur = true;
        }
        RAN | if cur { RAN_ALL } else { ran_list(FK_LEGS) }
    })
    .unwrap_or(0)
}

/// Deliberate movement and the crouch's centre of gravity by pace, the legs and the trunk: the stride warped at the feet about the gait's middle, both
/// feet and the pelvis riding the surge, the pelvis lower and tipped, the torso further over with the head level back out of it.
pub fn sneak_body(g: usize, an: &Anim, dl: f64, pace: f64, soft: f64, surge: f64, dsh: f64, tune: &Knobs) -> u32 {
    let (aim, mid) = (an.upper_w.aim, an.gait.mid);
    with_pose(g, an.p_a.at as usize, |d, rig, n, fk| {
        if !cast(n, &LEG_ROLES) || !cast(n, &[SPINE1, SPINE2, SPINE3, NECK, HEAD]) {
            return 0;
        }
        let kk = (1.0 + SNEAK.stride * dl) * (1.0 + SNEAK.pace_stride * pace) * (1.0 + tune.dash.stride * dsh);
        let drop = SNEAK.drop * dl + SNEAK.pace_drop * pace + SNEAK.soft_drop * soft + tune.dash.drop * dsh;
        let tip = drop * 25.0 * DEGR;
        let lean = (SNEAK.lean * dl + SNEAK.pace_lean * pace + tune.dash.lean * dsh) * DEGR * (1.0 - aim);
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

/// MOVES.steady: this frame's swing of the crouch cycle (its own sample, the pose at `at_c`, against its own mean) taken out by MV.steady's shares: the
/// pelvis's roll at the hips with the chest handed the same turn back, a little of the hips' sideways travel, both ankles put back nearer his middle and
/// the knees swung in over them; at setting 2 the pelvis ridden up by as much as both legs can pay out.
pub fn steady_layer(g: usize, an: &Anim, w: f64, base: f64, sunk: f64, mean: [f64; 2], tune: &Knobs) -> u32 {
    let (mean_roll, mean_x, steady_up) = (mean[0], mean[1], an.steady_up);
    let roll_now = |fk: &Fk| -> f64 {
        let (a, b) = (fk.at(role(THIGH_L)), fk.at(role(THIGH_R)));
        atan2(a[1] - b[1], a[0] - b[0])
    };
    // this frame's swing: the cycle's sample off the cycle's mean
    let Some(Some((d_roll, d_x))) = with_pose(g, an.p_c.at as usize, |d, rig, n, fk| {
        if !cast(n, &LEG_ROLES) || !cast(n, &[SPINE3]) {
            return None;
        }
        fk.part(rig, d, list(FK_LEGS));
        Some((roll_now(fk) - mean_roll, fk.at(role(HIPS))[0] - mean_x))
    }) else {
        return 0;
    };
    with_pose(g, an.p_a.at as usize, |d, rig, _n, fk| {
        let (hips, foot_l, foot_r) = (role(HIPS), role(FOOT_L), role(FOOT_R));
        let legs = [(role(THIGH_L), role(SHIN_L), foot_l), (role(THIGH_R), role(SHIN_R), foot_r)];
        fk.part(rig, d, list(FK_LEGS));
        let (fl, fr, rl, rr) = (fk.at(foot_l), fk.at(foot_r), fk.rot(foot_l), fk.rot(foot_r));
        let (tl, tr, sl, sr) = (fk.at(legs[0].0), fk.at(legs[1].0), fk.at(legs[0].1), fk.at(legs[1].1));
        // each knee's set off its hip, and its height, as the cycle has them
        let (ol, or) = ([sl[0] - tl[0], sl[1] - tl[1], sl[2] - tl[2]], [sr[0] - tr[0], sr[1] - tr[1], sr[2] - tr[2]]);
        let (yl, yr) = (sl[1], sr[1]);
        let (a, sx) = (d_roll * tune.mv.steady.roll * w, d_x * tune.mv.steady.sway * w);
        if abs(a) > 1e-6 {
            turn(d, rig, hips, q_roll(-a));
            turn(d, rig, role(SPINE3), q_roll(a));
        }
        if abs(sx) > 1e-6 {
            shift(d, rig, hips, &[-sx, 0.0, 0.0]);
        }
        let kx = 1.0 - (1.0 - tune.mv.steady.feet) * w * base;
        fk.part(rig, d, list(FK_LEGS));
        let (al, ar) = ([fl[0] * kx, fl[1], fl[2]], [fr[0] * kx, fr[1], fr[2]]);
        if steady_up > 0.0 {
            let mut up = steady_up;
            for (i, at) in [al, ar].iter().enumerate() {
                let (hip, knee, foot) = legs[i];
                let h = fk.at(hip);
                let len = (dist3(&h, &fk.at(knee)) + dist3(&fk.at(knee), &fk.at(foot))) * tune.mv.steady.reach;
                let (hx, hy, hz) = (h[0] - at[0], h[1] - at[1], h[2] - at[2]);
                let q = len * len - hx * hx - hz * hz;
                up = min(up, if q > 0.0 { sqrt(q) - hy } else { 0.0 });
            }
            if up > 1e-5 {
                shift(d, rig, hips, &[0.0, up, 0.0]);
                fk.part(rig, d, list(FK_LEGS));
            }
        }
        let (kn, ko) = (tune.mv.steady.knee * w * base, tune.mv.steady.knee_out * w * sunk);
        let (hl, hr) = (smoothstep(0.10, 0.22, yl), smoothstep(0.10, 0.22, yr));
        let (kl, kr) = (1.0 - kn * lerp(tune.mv.steady.knee_low, 1.0, hl) + ko * (1.0 - hl), 1.0 - kn * lerp(tune.mv.steady.knee_low, 1.0, hr) + ko * (1.0 - hr));
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

/// The close-in mode's carriage, the body: the pelvis sunk and set back with the feet put back where the gait has them, the heft of his stride read off
/// the feet (his shoulders round against the step and over the foot he is on), the spine bowed forward hips first with the head levelled back out of it
/// and turned onto what he creeps at.
pub fn creep_body(g: usize, an: &mut Anim, e: f64, free: f64, cr: f64, gw: f64, tw_y: f64, tw_p: f64, dt: f64, p0: [f64; 3], out: &mut [f64; 2]) -> u32 {
    *out = [0.0; 2];
    let (at, lo, speed, wt_w, yin, ls, hsh, gaze_local) = (an.p_a.at as usize, an.low_s, an.speed, an.wt.w, an.body_yaw_in, an.low_shape, an.heft_shape, an.gaze_local);
    let (at_there, creep_at, hf) = (an.creep_at.is_some(), an.creep_at.unwrap_or([0.0; 3]), &mut an.hf);
    with_pose(g, at, |d, rig, n, fk| {
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
            let h = min(dt, CREEP.heft.dt);
            hf.t += dt;
            let step = |x: &mut f64, v: &mut f64, to: f64, w0: f64, w1: f64| {
                let nv = *v + (w0 * w0 * (to - *x) - 2.0 * w1 * w0 * *v) * h;
                *x += nv * h;
                *v = nv;
            };
            step(&mut hf.s, &mut hf.sv, clamp((fl[2] - fr[2]) / CREEP.heft.stride, -1.0, 1.0), CREEP.heft.trunk[0], CREEP.heft.trunk[1]);
            let to = hf.s;
            step(&mut hf.a, &mut hf.av, to, CREEP.heft.arm[0], CREEP.heft.arm[1]);
            step(&mut hf.w, &mut hf.wv, clamp((fr[1] - fl[1]) / CREEP.heft.lift, -1.0, 1.0), CREEP.heft.body[0], CREEP.heft.body[1]);
            step(&mut hf.v, &mut hf.vv, clamp(abs(speed) / CREEP.heft.run, 0.0, 1.0), CREEP.heft.go[0], CREEP.heft.go[1]);
            hs = hf.s * gw;
            ha = hf.a * gw;
            hw = hf.w * gw;
            hg = (clamp(abs(speed) / CREEP.heft.run, 0.0, 1.0) - hf.v) * gw * hsh[5];
        }
        shift(d, rig, hips, &[hw * hsh[2], -(lerp(CREEP.drop[0], CREEP.drop[1], cr) + ls[0] * lo) * e - abs(hs) * hsh[4], -(lerp(CREEP.back[0], CREEP.back[1], cr) + ls[1] * lo) * e * free]);
        fk.part(rig, d, list(FK_LEGS));
        two_bone_ik(rig, d, fk.rot, fk.pos, role(THIGH_L), role(SHIN_L), foot_l, &fl, Some(&knee_pole(&rl)), 0.0, Some(&rl), false);
        two_bone_ik(rig, d, fk.rot, fk.pos, role(THIGH_R), role(SHIN_R), foot_r, &fr, Some(&knee_pole(&rr)), 0.0, Some(&rr), true);
        let lean = (lerp(CREEP.lean[0], CREEP.lean[1], cr) + ls[2] * lo + hsh[3] * gw - hg * 9.0 + if gw > 0.0 { CREEP.heft.breath * gw * sin(hf.t * PI * 2.0 * CREEP.heft.breath_hz) } else { 0.0 }) * DEGR * e * free;
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
        let lw = CREEP.look * e * free * (1.0 - clamp(wt_w, 0.0, 1.0));
        if at_there && lw > 0.003 {
            fk_run(rig, d, fk.rot, fk.pos);
            // (toModelP: the world point in the model's frame)
            let (cy, sy) = (cos(yin), sin(yin));
            let (dx, dz) = (creep_at[0] - p0[0], creep_at[2] - p0[2]);
            let m = [dx * cy - dz * sy, creep_at[1] - p0[1], dx * sy + dz * cy];
            let target = untwist(fk, tw_y, tw_p, m);
            look_at(d, rig, fk, &gaze_local, target, lw, 0.0);
        }
        *out = [ha, hg];
        RAN | RAN_ALL
    })
    .unwrap_or(0)
}

// the life layer's columns (after its own numbers): the breath and the body's mass, then the foot lock, then a foot (two of them)
/// One hand-animated channel of a one-shot — Catmull-Rom through (time, value) keys, flat at the first and the last key, clamped outside
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
/// A cubic segment with its end slopes given
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

/// A crouched man's toes kept out of the floor: the foot joint alone turned about the ankle until the toe is back on the floor, laid away from the knee
/// along the shin's own heading unless the foot already points somewhere along the ground. The ankle is not moved.
pub fn toes_up(g: usize, an: &Anim, w: f64) -> u32 {
    with_pose(g, an.p_a.at as usize, |d, rig, n, fk| {
        if !cast(n, &[SHIN_L, FOOT_L, TOE_L, SHIN_R, FOOT_R, TOE_R]) {
            return 0;
        }
        fk_run(rig, d, fk.rot, fk.pos);
        let floor_y = TOE_FLOOR_Y;
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

/// The kick's body half: the pelvis's shift, dip, drive, lean back, counter-yaw and hike as overlapping curves, the spine and the head countering so the
/// eyes stay on the door, the swing foot flown along chamber, strike, recoil and plant by IK, the other foot held from the first frame. Built on the
/// scratch pose at `at_c` from the pose at `at`, and laid on that one by w. The left leg's kick is the same kick in a mirror.
pub fn kick_body(g: usize, an: &mut Anim, t: f64, w: f64, tune: &Knobs) -> u32 {
    const ROLES: [usize; 12] = [HIPS, SPINE1, SPINE2, SPINE3, NECK, HEAD, THIGH_L, SHIN_L, FOOT_L, THIGH_R, SHIN_R, FOOT_R];
    let (at, at_c, left, f) = (an.p_a.at as usize, an.p_c.at as usize, an.kick_leg == AnimKickLeg::Left, an.idle_feet);
    let from = &mut an.kick_from;
    let m = if left { -1.0 } else { 1.0 };
    // when the sole meets the door, and when the foot is back on the floor (seconds into the kick)
    let (kick_impact, kick_plant) = (KICK_IMPACT, KICK_PLANT);
    let (f_l, f_rl, f_r, f_rr) = (f.l, f.r_l, f.r, f.r_r);
    // where the kick found the feet: taken off the pose on its first tick
    let Some(Some((n, from))) = with_pose(g, at, |d, rig, n, fk| {
        if !cast(n, &ROLES) {
            return None;
        }
        fk_run(rig, d, fk.rot, fk.pos);
        let from = *from.get_or_insert_with(|| AnimKickFrom { l: fk.at(role(FOOT_L)), r_l: fk.rot(role(FOOT_L)), r: fk.at(role(FOOT_R)), r_r: fk.rot(role(FOOT_R)) });
        Some((n, from))
    }) else {
        return 0;
    };
    let (k_l, k_rl, k_r, k_rr) = (from.l, from.r_l, from.r, from.r_r);
    // the swing leg and the planted one; their idle spots and sole rotations; where the kick found them
    let (th_s, sh_s, ft_s, th_p, sh_p, ft_p) =
        if left { (role(THIGH_L), role(SHIN_L), role(FOOT_L), role(THIGH_R), role(SHIN_R), role(FOOT_R)) } else { (role(THIGH_R), role(SHIN_R), role(FOOT_R), role(THIGH_L), role(SHIN_L), role(FOOT_L)) };
    let (fs, fsr, fp, fpr) = if left { (f_l, f_rl, f_r, f_rr) } else { (f_r, f_rr, f_l, f_rl) };
    let (ks, kp, kpr) = if left { (k_l, k_r, k_rr) } else { (k_r, k_l, k_rl) };
    // pC.copy(pA)
    if !ask_blend(g, at, at, 1.0, false, at_c, n, tune) {
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
            let (sh, own) = (fk.rot(sh_s), own_rot(d, ft_s));
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
    if code == 0 || !ask_blend(g, at, at_c, w, false, at, n, tune) {
        return 0;
    }
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
    [ch(d, i, 3), ch(d, i, 4), ch(d, i, 5), ch(d, i, 6)]
}
#[inline(always)]
fn set_own_rot(d: &mut [S], i: usize, q: &[f64; 4]) {
    set_ch(d, i, 3, q[0]);
    set_ch(d, i, 4, q[1]);
    set_ch(d, i, 5, q[2]);
    set_ch(d, i, 6, q[3]);
}
/// a foot's place and its sole's rotation (model space)
type Sole = ([f64; 3], [f64; 4]);
/// …and of the hold's own step: where the knee points as well
type Tread = ([f64; 3], [f64; 4], [f64; 3]);

/// A CLIP A LAYER SAMPLES FOR ITSELF (`p.copy(rest); sampleClip(clip, t, looped, p)`): the rest pose copied into a scratch pose of the same character and the
/// clip sampled over it. `clip` is the module's number for it, handed with the layer's own numbers (skeleton.ts clipNo: the host asks a layer only when the
/// module holds every clip it samples); false = no such clip or pose
fn sample_on_rest(g: usize, at_rest: usize, at_out: usize, nodes: usize, clip: f64, t: f64, looped: bool, tune: &Knobs) -> bool {
    clip >= 0.0 && ask_blend(g, at_rest, at_rest, 1.0, false, at_out, nodes, tune) && ask_sample(g, at_out, nodes, clip as usize, t, looped, tune)
}

/// The wall hold's body: the spine blended toward the standing idle's and then measured and put (the chest square to the wall but for a counter-turn
/// toward the lead side, the spine's line leant onto the plaster, a crouch's pelvis tilt taken back at the pelvis, the peek's own numbers taking over as
/// it comes in), the pelvis carried back onto the wall and along it onto the lead foot, the head turned along the wall, and the feet planted by IK:
/// parked on the hold's own spots, on the move on the hold's own step or the turned cycle narrowed toward the wall, each sole kept where it stood in the
/// world through a take until its turn to step. `at_c`: the standing idle sampled over the rest pose (its spine, and its pelvis) · `at_b`: the crouched
/// idle sampled the same way, read only under the hold's own step (gw > 1e-4): the host samples both until the clip sampler is the module's.
pub fn wall_body(g: usize, an: &mut Anim, w: f64, p0: [f64; 3], idle: f64, crouch_idle: f64, tune: &Knobs) -> u32 {
    const ROLES: [usize; 16] = [HIPS, SPINE1, SPINE2, SPINE3, NECK, HEAD, SHOULDER_L, SHOULDER_R, UPPER_ARM_L, UPPER_ARM_R, THIGH_L, SHIN_L, FOOT_L, THIGH_R, SHIN_R, FOOT_R];
    let (at, at_b, at_rest) = poses(an);
    let (at_c, idle_t, ws) = (an.p_c.at as usize, an.idle_t, &an.wst);
    let (cr, s, side_v, pk, park) = (clamp(an.crouch, 0.0, 1.0), clamp(ws.side.x, -1.0, 1.0), ws.side.v, clamp(ws.peek.x, 0.0, 1.0), clamp(ws.park.x, 0.0, 1.0));
    let (aim_w, ny, phase, wall_hit, aim_yaw, lk) = (an.upper_w.aim, an.wall_yaw, an.phase, an.wall_hit, an.aim_yaw_offset, an.look_yaw_extra);
    let (chest_rest_inv, gaze_local, wall_take, on_wall, yin) = (an.chest_rest_inv, an.gaze_local, an.wall_take, an.stance == Stance::Wall, an.body_yaw_in);
    let (take_feet, f) = (an.wall_take_feet, &an.idle_feet);
    let (i_rl, i_ly, i_rr, i_ry) = (f.r_l, f.l[1], f.r_r, f.r[1]);
    let e = pk * pk * (3.0 - 2.0 * pk);
    // the wall's normal (into the room) and his right along the wall, model space
    let n = [sin(ny), 0.0, cos(ny)];
    let tr = [-n[2], 0.0, n[0]];
    let root_off = WALL_ROOT_OFF;
    // the sidle's own step: its share of the feet, and of the pelvis on the move
    let g_share = if tune.wallsnap.on && true { lerp(WALL_POSE.gait.share[0], WALL_POSE.gait.share[1], cr) } else { 0.0 };
    let gw = g_share * (1.0 - park) * w;
    // the idles underneath, sampled here: the standing one's spine and pelvis, the crouched one's pelvis
    let Some(nodes) = with_pose(g, at, |_d, _rig, nodes, _fk| nodes) else {
        return 0;
    };
    if !sample_on_rest(g, at_rest, at_c, nodes, idle, idle_t, true, tune) {
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
        if !sample_on_rest(g, at_rest, at_b, nodes, crouch_idle, idle_t, true, tune) {
            return 0;
        }
        let Some(q) = with_pose(g, at_b, |d, _rig, _nodes, _fk| own_rot(d, role(HIPS))) else {
            return 0;
        };
        q
    } else {
        [0.0; 4]
    };
    let (gait, held, wtk) = (&mut an.wall_gait, &mut an.wtk_hold, &mut an.wtk);
    with_pose(g, at, |d, rig, _nodes, fk| {
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
            *gait = WALL_GAIT_REST;
        }
        if gw > 1e-4 {
            let q = nlerp(&own_rot(d, hips), &nlerp(&under[7], &hips_low, cr), gw);
            set_own_rot(d, hips, &q);
        }
        fk_run(rig, d, fk.rot, fk.pos);
        let hp = fk.at(hips);
        let back_off = lerp(lerp(WALL_POSE.back[0], WALL_POSE.back[1], cr), WALL_POSE.back[2], 1.0 - park);
        let out = dot3(&hp, &n) - (-root_off + back_off);
        let drop = lerp(WALL_POSE.drop[0], WALL_POSE.drop[1], cr) + 0.006 * (1.0 - park) * (1.0 - g_share) * (1.0 - cos(phase * 4.0 * PI)) + tune.wallsnap.dip * wall_hit;
        shift(d, rig, hips, &add3(add3(scale3(n, -out * w), [0.0, -drop * w, 0.0]), scale3(tr, s * WALL_POSE.peek.shift * e * w)));
        if gw > 1e-4 {
            let lvl = lerp(WALL_POSE.gait.hips[0], WALL_POSE.gait.hips[1], cr) - lerp(WALL_POSE.gait.bob[0], WALL_POSE.gait.bob[1], cr) * 0.5 * (1.0 + cos((*gait - 0.05) * 4.0 * PI));
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
        let (turn_deg, lean_deg, gaze_deg) = (lerp(WALL_POSE.chest_turn, WALL_POSE.peek.turn, e), lerp(lerp(WALL_POSE.lean[0], WALL_POSE.lean[1], cr), WALL_POSE.peek.tilt, e), lerp(WALL_POSE.gaze, WALL_POSE.peek.gaze, e));
        for _pass in 0..2 {
            fk_run(rig, d, fk.rot, fk.pos);
            let dy = wrap_angle(ny - s * turn_deg * DEGR - (twist - aim_yaw * aim_w) - lerp(shoulder_yaw_of(fk), chest_yaw_of(fk), e)) * w;
            turn(d, rig, s1, q_yaw(dy * 0.3));
            turn(d, rig, s2, q_yaw(dy * 0.35));
            turn(d, rig, s3, q_yaw(dy * 0.35));
            fk_run(rig, d, fk.rot, fk.pos);
            let dv = sub3(fk.at(nk), fk.at(s1));
            let (tilt_n, tilt_t) = (atan2(dot3(&dv, &n), dv[1]), atan2(dot3(&dv, &tr), dv[1]));
            let (d_n, d_t) = ((lean_deg * DEGR - tilt_n) * w, (s * WALL_POSE.peek.lean * DEGR * e - tilt_t) * w);
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
            let dpitch = (el + (WALL_POSE.head_pitch - tune.wallsnap.nod * wall_hit) * DEGR) * hw;
            let gh = normalize([gz[0], 0.0, gz[2] + 1e-6]);
            let ax_p = [gh[2], 0.0, -gh[0]];
            let unroll = -s * WALL_POSE.peek.lean * DEGR * e * 0.5 * hw;
            let q = |share: f64| q_mul(&axis_angle(&n, unroll * share), &q_mul(&q_yaw(dyaw * share), &axis_angle(&ax_p, dpitch * share)));
            turn(d, rig, nk, q(0.4));
            turn(d, rig, hd, q(0.6));
        }
        // 5. the feet
        let u = (1.0 + s) / 2.0;
        // a parked spot in the wall's frame for one role of one foot, and its toes' yaw
        let spot = |lead: bool, right: bool| -> ([f64; 3], f64) {
            let (stand, low) = if lead {
                ([WALL_POSE.feet.lead[0], WALL_POSE.feet.lead[1], WALL_POSE.feet.lead[2]], [WALL_POSE.feet.lead_crouch[0], WALL_POSE.feet.lead_crouch[1], WALL_POSE.feet.lead_crouch[2]])
            } else {
                ([WALL_POSE.feet.trail[0], WALL_POSE.feet.trail[1], WALL_POSE.feet.trail[2]], [WALL_POSE.feet.trail_crouch[0], WALL_POSE.feet.trail_crouch[1], WALL_POSE.feet.trail_crouch[2]])
            };
            let av = lerp3(&stand, &low, cr);
            let sg = if right == lead { 1.0 } else { -1.0 };
            (add3(scale3(tr, sg * av[0]), scale3(n, av[1] - root_off)), -sg * av[2] * DEGR)
        };
        let mv = (1.0 - park) * w * (1.0 - g_share);
        let wall_gait = *gait;
        // the hold's own step: where this sole is on its track at the step clock's phase, its rotation, and where its knee points
        let gait_foot = |right: bool, r_idle: &[f64; 4], y_idle: f64| -> Tread {
            let (sl, duty) = (lerp(WALL_POSE.gait.step[0], WALL_POSE.gait.step[1], cr), WALL_POSE.gait.duty);
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
            let heel = lerp(WALL_POSE.gait.heel[0], WALL_POSE.gait.heel[1], cr) * DEGR * smoothstep(0.1, 1.0, -along * clamp(s * 2.0, -1.0, 1.0) / half);
            let lw = if right { u } else { 1.0 - u };
            let off = lerp(lerp(WALL_POSE.gait.far[0], WALL_POSE.gait.far[1], cr), lerp(WALL_POSE.gait.near[0], WALL_POSE.gait.near[1], cr), lw);
            let yaw_abs = ny - s * lerp(WALL_POSE.gait.toe[0], WALL_POSE.gait.toe[1], cr) * DEGR * (0.5 + 0.5 * lw);
            let h = [sin(yaw_abs), 0.0, cos(yaw_abs)];
            const BALL: f64 = 0.12;
            let al = along + s * lerp(WALL_POSE.gait.ahead[0], WALL_POSE.gait.ahead[1], cr) + (if right { 1.0 } else { -1.0 }) * lerp(WALL_POSE.gait.apart[0], WALL_POSE.gait.apart[1], cr);
            let p = [
                tr[0] * al + n[0] * (off - root_off) + h[0] * BALL * (1.0 - cos(heel)),
                y_idle + lerp(WALL_POSE.gait.lift[0], WALL_POSE.gait.lift[1], cr) * up + BALL * sin(heel),
                tr[2] * al + n[2] * (off - root_off) + h[2] * BALL * (1.0 - cos(heel)),
            ];
            let fwd = q_rotate(r_idle, &AY);
            let yaw0 = atan2(fwd[0], fwd[2]);
            let r = q_unit(q_mul(&axis_angle(&[h[2], 0.0, -h[0]], heel), &q_mul(&q_yaw(wrap_angle(yaw_abs - yaw0)), r_idle)));
            let kr = lerp(WALL_POSE.gait.knee_room[0], WALL_POSE.gait.knee_room[1], cr);
            let dn = normalize([n[0] * kr, 0.0, (1.0 - kr) + n[2] * kr]);
            (p, r, normalize([h[0] * 0.25 + dn[0] * 0.75, 0.5, h[2] * 0.25 + dn[2] * 0.75]))
        };
        let (g_l, g_r) = if g_share > 0.0 { (Some(gait_foot(false, &i_rl, i_ly)), Some(gait_foot(true, &i_rr, i_ry))) } else { (None, None) };
        // (a short take ends before the pose's weight is whole: the soles stay where they stepped to)
        let taking = wall_take >= 0.0 && wall_take < 1.0;
        if taking {
            *held = true;
        } else if *held && (w >= 0.98 || !on_wall) {
            *held = false;
        }
        let wtk_hold = *held;
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
            let on_toes = cr * (1.0 - lw) * WALL_POSE.feet.trail_heel;
            p[1] = lerp(y_idle, f_clip[1], on_toes) + 0.04 * (1.0 - s * s) * (if (side_v > 0.0) == right { 1.0 } else { 0.25 }) + 0.08 * park * (1.0 - park) * (0.4 + 0.6 * lw);
            let base = nlerp(r_idle, &r_clip, on_toes);
            let fwd = q_rotate(&base, &AY);
            let yaw0 = atan2(fwd[0], fwd[2]);
            let r = q_unit(q_mul(&q_yaw(wrap_angle(ny + yaw_rel - yaw0)), &base));
            let share = if full { 1.0 } else { park * (if wtk_hold { 1.0 } else { w }) };
            let mut q = lerp3(&f_clip, &p, share);
            if mv > 1e-4 {
                let off_c = dot3(&q, &n) + root_off;
                q = mad3(q, n, (WALL_POSE.slide.line + (off_c - root_off) * WALL_POSE.slide.narrow - off_c) * mv);
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
            if wtk.map_or(true, |k| tk < k.last) {
                let (l, r) = match take_feet {
                    Some(t) => (t.l, t.r),
                    None => (AnimFootSpot { p: to_world(&f_l), yaw: yin + foot_yaw(&r_l) }, AnimFootSpot { p: to_world(&f_r), yaw: yin + foot_yaw(&r_r) }),
                };
                let first = if dist_xz(&f_l, &s_l.0) >= dist_xz(&f_r, &s_r.0) { 0.0 } else { 1.0 };
                *wtk = Some(AnimWtk { l, r, first, last: tk });
            }
            let mut k = wtk.unwrap_or_default();
            k.last = tk;
            *wtk = Some(k);
            let step = |from: [f64; 3], from_yaw: f64, sp: &Sole, r_clip: &[f64; 4], t0: f64, t1: f64| -> Sole {
                let q = clamp((tk - t0) / (t1 - t0), 0.0, 1.0);
                let ease = q * q * (3.0 - 2.0 * q);
                let (wx, wz) = (from[0] - p0[0], from[2] - p0[2]);
                let mut m = [wx * cy - wz * sy, from[1] - p0[1], wx * sy + wz * cy];
                let (dx, dz) = (m[0] - sp.0[0], m[2] - sp.0[2]);
                let (dl, mx) = (hypot2(dx, dz), tune.wallsnap.step_trail);
                if t0 > 0.0 && dl > mx {
                    m[0] = sp.0[0] + dx * mx / dl;
                    m[2] = sp.0[2] + dz * mx / dl;
                }
                let mut p = lerp3(&m, &sp.0, ease);
                p[1] += tune.wallsnap.step_lift * sin(PI * q);
                (p, nlerp(&q_unit(q_mul(&q_yaw(wrap_angle(from_yaw - yin - foot_yaw(r_clip))), r_clip)), &sp.1, ease))
            };
            // (the further foot goes at once, the other waits, then closes as the back lands)
            let (early, late) = ((0.0, 0.58), (0.42, 1.0));
            let (tl, trr) = if k.first == 0.0 { (early, late) } else { (late, early) };
            t_l = step(k.l.p, k.l.yaw, &s_l, &r_l, tl.0, tl.1);
            t_r = step(k.r.p, k.r.yaw, &s_r, &r_r, trr.0, trr.1);
        } else {
            *wtk = None;
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
    .unwrap_or(0)
}


/// He gives a man room as he passes, the body's share: by how near the man is the pelvis comes lower and off his side over feet left where they were,
/// and the trunk tips away from him with the head brought level back out of it.
pub fn watch_slip(g: usize, an: &mut Anim, dt: f64, free: f64, there: Option<[f64; 3]>, tune: &Knobs) -> u32 {
    let (at, slip_k, crouch, slip_lean, wt) = (an.p_a.at as usize, an.slip_k, an.crouch, an.slip_lean, &mut an.wt);
    let (there, m) = (there.is_some(), there.unwrap_or([0.0; 3]));
    with_pose(g, at, |d, rig, n, fk| {
        if !cast(n, &LEG_ROLES) || !cast(n, &[SPINE1, SPINE2, SPINE3, NECK, HEAD]) {
            return 0;
        }
        let s_want = if there { clamp(slip_k, 0.0, 1.0) * free * clamp(crouch, 0.0, 1.0) } else { 0.0 };
        wt.sw = if abs(s_want - wt.sw) < 0.003 { s_want } else { damp(wt.sw, s_want, tune.watch.slip_rate, dt) };
        if there {
            let l = hypot2(m[0], m[2]);
            if l > 1e-4 {
                wt.sx = m[0] / l;
                wt.sz = m[2] / l;
            }
            if if wt.side == 0.0 { wt.sx < -0.2 } else { wt.sx > 0.2 } {
                wt.side = if wt.sx > 0.0 { 0.0 } else { 1.0 };
            }
            let lw = clamp(slip_lean, 0.0, 1.0);
            wt.sl = if abs(lw - wt.sl) < 0.003 { lw } else { damp(wt.sl, lw, tune.watch.slip_rate, dt) };
        }
        let mut ran = RAN;
        let (sw, sx, sl) = (wt.sw, wt.sx, wt.sl);
        if sw > 0.003 {
            let kk = sw * sw * (3.0 - 2.0 * sw);
            let (kl, kd) = (kk * sl, kk * (1.0 + tune.watch.squeeze * (1.0 - sl)));
            let (hips, foot_l, foot_r) = (role(HIPS), role(FOOT_L), role(FOOT_R));
            fk.part(rig, d, list(FK_LEGS));
            let (f_l, f_r, r_l, r_r) = (fk.at(foot_l), fk.at(foot_r), fk.rot(foot_l), fk.rot(foot_r));
            shift(d, rig, hips, &[-sx * tune.watch.away * kl, -tune.watch.drop * kd, 0.0]);
            fk.part(rig, d, list(FK_LEGS));
            two_bone_ik(rig, d, fk.rot, fk.pos, role(THIGH_L), role(SHIN_L), foot_l, &f_l, Some(&knee_pole(&r_l)), 0.0, Some(&r_l), false);
            two_bone_ik(rig, d, fk.rot, fk.pos, role(THIGH_R), role(SHIN_R), foot_r, &f_r, Some(&knee_pole(&r_r)), 0.0, Some(&r_r), true);
            ran |= RAN_ALL;
            let roll = sx * tune.watch.lean * DEGR * kl;
            turn(d, rig, role(SPINE1), q_roll(roll * 0.4));
            turn(d, rig, role(SPINE2), q_roll(roll * 0.35));
            turn(d, rig, role(SPINE3), q_roll(roll * 0.25));
            turn(d, rig, role(NECK), q_roll(-roll * 0.4));
            turn(d, rig, role(HEAD), q_roll(-roll * 0.6));
        }
        ran
    })
    .unwrap_or(0)
}

/// He watches a man: the head goes to the point (the chest a little, the neck, the head), within what a neck does of the chest's line; the angle follows
/// the point, and the whole comes and goes by its weight.
pub fn watch_look(g: usize, an: &mut Anim, dt: f64, free: f64, there: Option<[f64; 3]>, tune: &Knobs) -> u32 {
    let (at, watch_k, gaze_local, wt) = (an.p_a.at as usize, an.watch_k, an.gaze_local, &mut an.wt);
    let (there, m) = (there.is_some(), there.unwrap_or([0.0; 3]));
    with_pose(g, at, |d, rig, n, fk| {
        if !cast(n, &[SPINE3, NECK, HEAD]) {
            return 0;
        }
        let (s3, nk, hd) = (role(SPINE3), role(NECK), role(HEAD));
        let w_want = if there { clamp(watch_k, 0.0, 1.0) * free } else { 0.0 };
        wt.w = if abs(w_want - wt.w) < 0.003 { w_want } else { damp(wt.w, w_want, tune.watch.w_rate, dt) };
        let mut ran = RAN;
        if there && w_want > 0.0 {
            fk_run(rig, d, fk.rot, fk.pos);
            ran |= RAN_ALL;
            let hr = fk.rot(hd);
            let gaze = q_rotate(&hr, &gaze_local);
            let eyes = mad3(mad3(fk.at(hd), q_rotate(&hr, &AY), 0.1), gaze, 0.08);
            let want = normalize(sub3(m, eyes));
            let yaw_e = clamp(wrap_angle(atan2(want[0], want[2]) - atan2(gaze[0], gaze[2])), -tune.watch.yaw * DEGR, tune.watch.yaw * DEGR);
            let pitch_e = clamp(asin(clamp(gaze[1], -1.0, 1.0)) - asin(clamp(want[1], -1.0, 1.0)), -tune.watch.up * DEGR, tune.watch.down * DEGR);
            if wt.w < 0.01 {
                wt.yaw = yaw_e;
                wt.pitch = pitch_e;
            } else {
                wt.yaw = damp(wt.yaw, yaw_e, tune.watch.rate, dt);
                wt.pitch = damp(wt.pitch, pitch_e, tune.watch.rate, dt);
            }
        } else if wt.w <= 0.0 {
            wt.yaw = 0.0;
            wt.pitch = 0.0;
        }
        if wt.w > 0.003 {
            let (yw, pw) = (wt.yaw * wt.w, wt.pitch * wt.w);
            let (sh0, sh1, sh2) = (tune.watch.share[0], tune.watch.share[1], tune.watch.share[2]);
            let rest = sh1 + sh2;
            turn(d, rig, s3, q_yaw(yw * sh0));
            turn(d, rig, nk, q_mul(&q_yaw(yw * sh1), &q_pitch(pw * sh1 / rest)));
            turn(d, rig, hd, q_mul(&q_yaw(yw * sh2), &q_pitch(pw * sh2 / rest)));
        }
        ran
    })
    .unwrap_or(0)
}

/// After the stances and the arms, the body's share of the reactions: the flinch, the start, the hit, the blinded hunch, the clutch's fold, a hurt man's
/// hunch and heavy step, a shaken man's ducked head and the toss's wind-up, summed into one turn of the trunk, one of the head and one move of the
/// pelvis over feet held where they are. The arms that follow (the start's, the blinded hand, the held gun, the toss) are the host's.
pub fn mv_post_body(g: usize, an: &mut Anim, dt: f64, gate: f64, fold_on: bool, tune: &Knobs) -> u32 {
    let (at, trained, lg_w, blind, blind_k) = (an.p_a.at as usize, an.trained, an.lg.w.x, an.dazzle.is_some(), an.dazzle.map_or(0.0, |d| d.k));
    let (clutch, fold_too, hurt, hurt_side, aim, speed, phase, shaken) = (an.clutch, an.fold_too, an.hurt, an.hurt_side, an.upper_w.aim, an.speed, an.phase, an.shaken);
    let (m, po) = (&mut an.mvs, &mut an.mv_po);
    let (fl_k_to, fl_d, st_d, ht_d, ht_side, dz_d, ts, rel) = (m.fl_k_to, m.fl_d, m.st_d, m.ht_d, m.ht_side, m.dz_d, m.ts_t, m.ts_rel);
    with_pose(g, at, |d, rig, n, fk| {
        if !cast(n, &LEG_ROLES) || !cast(n, &[SPINE1, SPINE2, SPINE3, NECK, HEAD, HAND_R]) {
            return 0;
        }
        let t = clamp(trained, 0.0, 1.0);
        let kk = |p0: f64, p1: f64| lerp(p0, p1, t);
        let (mut pitch, mut yaw, mut roll, mut hp, mut hy, mut drop, mut sx, mut sz) = (0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64);
        if m.fl_t >= 0.0 {
            m.fl_k += (fl_k_to - m.fl_k) * min(1.0, dt * 12.0);
            let dur = kk(tune.mv.flinch.dur[0], tune.mv.flinch.dur[1]);
            let e = curve(m.fl_t / dur, &MV_FLINCH_ENV) * kk(tune.mv.flinch.amp[0], tune.mv.flinch.amp[1]) * m.fl_k * gate;
            let (sd, lat) = (if fl_d[0] >= 0.0 { 1.0 } else { -1.0 }, min(1.0, abs(fl_d[0]) * 1.6));
            pitch += tune.mv.flinch.pitch * e * (0.65 + 0.35 * max(0.0, fl_d[2]));
            yaw -= sd * tune.mv.flinch.away * e * lat;
            roll += sd * 3.0 * e * lat;
            hp += tune.mv.flinch.head * e;
            hy -= sd * tune.mv.flinch.away * e * lat;
            drop += tune.mv.flinch.drop * e;
            sx -= fl_d[0] * tune.mv.flinch.side * e;
            sz -= fl_d[2] * tune.mv.flinch.side * e;
            m.fl_t += dt;
            if m.fl_t > dur {
                m.fl_t = -1.0;
            }
        }
        // the start: reared back off a thing in front, turned toward the side it is on; a thing behind him he comes round to
        let (mut stw, mut stn) = (0.0f64, 1.0f64);
        if m.st_t >= 0.0 {
            let (dur, lw, dv) = (kk(MV_STARTLE.dur[0], MV_STARTLE.dur[1]), clamp(lg_w, 0.0, 1.0), st_d);
            let amp = kk(MV_STARTLE.amp[0], MV_STARTLE.amp[1]);
            let e = curve(m.st_t / dur, &MV_STARTLE.env) * lerp(amp, max(amp, MV_STARTLE.gun_amp), lw) * gate;
            let (bh, sd) = (clamp(-dv[2], 0.0, 1.0), if dv[0] >= 0.0 { 1.0 } else { -1.0 });
            let lat = if bh > 0.0 { 1.0 } else { min(1.0, abs(dv[0]) * 1.6) };
            pitch -= MV_STARTLE.rear * (1.0 + MV_STARTLE.gun_rear * lw) * e * dv[2] * (if dv[2] < 0.0 { MV_STARTLE.behind_pitch } else { 1.0 });
            yaw += sd * MV_STARTLE.turn * (1.0 + MV_STARTLE.behind_turn * bh) * e * lat;
            roll -= sd * MV_STARTLE.lean * e * lat;
            hp -= MV_STARTLE.head * e * max(0.3, dv[2]);
            hy += sd * MV_STARTLE.head_turn * (1.0 + MV_STARTLE.behind_head * bh) * e * lat;
            drop += MV_STARTLE.drop * e;
            sx -= dv[0] * MV_STARTLE.back * (1.0 + MV_STARTLE.gun_back * lw) * e;
            sz -= dv[2] * MV_STARTLE.back * (1.0 + MV_STARTLE.gun_back * lw) * e;
            stw = e;
            stn = sd;
            m.st_t += dt;
            if m.st_t > dur {
                m.st_t = -1.0;
            }
        }
        if m.ht_t >= 0.0 {
            let (dv, e) = (ht_d, curve(m.ht_t, &MV_HIT_ENV) * gate);
            let fz = if dv[2] <= 0.0 { 1.0 } else { -1.0 };
            let (tp, ty) = (dv[2] * tune.mv.hit.pitch * e, ht_side * fz * tune.mv.hit.yaw * e * (0.4 + 0.6 * abs(dv[2])));
            pitch += tp;
            yaw += ty;
            roll += (ht_side * fz * tune.mv.hit.roll * abs(dv[2]) - dv[0] * tune.mv.hit.roll * 2.0) * e;
            hp -= tp * 0.5;
            hy -= ty * 0.5;
            drop += tune.mv.hit.drop * abs(e);
            sx += dv[0] * tune.mv.hit.shove * e;
            sz += dv[2] * tune.mv.hit.shove * e;
            m.ht_t += dt;
            if m.ht_t > tune.mv.hit.dur {
                m.ht_t = -1.0;
            }
        }
        // blinded: the side he turns away to is chosen once, as it goes off
        let dz_t = if blind && mv_on(tune.moves.dazzle, tune) { clamp(blind_k, 0.0, 1.0) } else { 0.0 };
        let rate = if dz_t > m.dz.x { tune.mv.dazzle.rate_in } else { tune.mv.dazzle.rate_out };
        spring_step(&mut m.dz, dz_t, rate, dt);
        if dz_t == 0.0 && m.dz.x < 0.02 {
            m.dz.x = 0.0;
            m.dz.v = 0.0;
        }
        let dzw = clamp(m.dz.x, 0.0, 1.0) * kk(tune.mv.dazzle.amp[0], tune.mv.dazzle.amp[1]) * gate;
        if m.dz.x <= 0.0 {
            m.dz_sd = 0.0;
        } else if m.dz_sd == 0.0 {
            m.dz_sd = if dz_d[0] >= 0.0 { 1.0 } else { -1.0 };
        }
        if dzw > 0.003 {
            m.dz_t += dt;
            let sd = if falsy(m.dz_sd) { 1.0 } else { m.dz_sd };
            pitch += tune.mv.dazzle.hunch * dzw;
            yaw -= sd * tune.mv.dazzle.away * dzw;
            roll += sd * tune.mv.dazzle.roll * dzw;
            hp += tune.mv.dazzle.head * dzw;
            hy -= sd * tune.mv.dazzle.head_away * dzw;
            drop += tune.mv.dazzle.drop * dzw;
            sx += (-dz_d[0] * tune.mv.dazzle.back + tune.mv.dazzle.sway_x * sin(m.dz_t * 3.1)) * dzw;
            sz += (-dz_d[2] * tune.mv.dazzle.back + tune.mv.dazzle.sway_z * sin(m.dz_t * 2.3 + 1.0)) * dzw;
        } else {
            m.dz_t = 0.0;
        }
        // the clutch's fold: under the blinded hunch it gives way by the hunch's own weight
        let want = if clutch > 0.0 && fold_on { clamp(clutch, 0.0, 1.0) * (1.0 - clamp(dzw, 0.0, 1.0)) } else { 0.0 };
        if want > 0.0 || m.cf.x != 0.0 {
            spring_step(&mut m.cf, want, MV2.clutch.fold.rate, dt);
            if want == 0.0 && m.cf.x < 0.01 {
                m.cf.x = 0.0;
                m.cf.v = 0.0;
            }
        }
        let cfw = clamp(m.cf.x, 0.0, 1.0) * gate;
        if cfw > 0.003 {
            m.cf_t += dt;
            let s = sin(m.cf_t * 2.0 * PI * MV2.clutch.fold.hz);
            pitch += (if fold_too { MV2.clutch.fold.pitch_too } else { MV2.clutch.fold.pitch }) * cfw;
            roll += MV2.clutch.fold.sway * s * cfw;
            hp += MV2.clutch.fold.head * cfw;
            drop += MV2.clutch.fold.drop * cfw;
            sx += 0.012 * s * cfw;
        } else {
            m.cf_t = 0.0;
        }
        // a man's condition: hurt, then shaken; both give way under the fold and the blinded hunch
        let (want, under) = (clamp(hurt, 0.0, 1.0), (1.0 - clamp(dzw, 0.0, 1.0)) * (1.0 - cfw));
        if want > 0.0 || m.hu.x != 0.0 {
            spring_step(&mut m.hu, want, MV2.hurt.rate, dt);
            if want == 0.0 && m.hu.x < 0.01 {
                m.hu.x = 0.0;
                m.hu.v = 0.0;
            }
        }
        let hu = clamp(m.hu.x, 0.0, 1.0) * gate * under;
        if hu > 0.003 {
            let (sd, aim_w) = (hurt_side, clamp(aim, 0.0, 1.0));
            let (brace, mw) = (1.0 - MV2.hurt.brace * aim_w, smoothstep(0.15, 0.7, abs(speed)));
            let on_it = max(0.0, -sd * sin(2.0 * PI * phase));
            let step = on_it * on_it * mw;
            m.hu_t += dt;
            let br = sin(m.hu_t * 2.0 * PI * MV2.hurt.hz) * (1.0 - mw);
            pitch += (MV2.hurt.pitch + MV2.hurt.step_pitch * step) * brace * hu;
            roll -= sd * (MV2.hurt.lean + MV2.hurt.step_roll * step + MV2.hurt.sway * br) * hu;
            yaw += sd * MV2.hurt.step_yaw * step * brace * hu;
            hp += (MV2.hurt.head * (1.0 - MV2.hurt.head_brace * aim_w) + MV2.hurt.step_head * step) * hu;
            drop += (MV2.hurt.drop + MV2.hurt.step_drop * step) * hu;
            sx += sd * MV2.hurt.step_side * step * hu;
        } else {
            m.hu_t = 0.0;
        }
        let want = clamp(shaken, 0.0, 1.0);
        if want > 0.0 || m.sk.x != 0.0 {
            let rate = if want > m.sk.x { MV2.shaken.rate_in } else { MV2.shaken.rate_out };
            spring_step(&mut m.sk, want, rate, dt);
            if want == 0.0 && m.sk.x < 0.01 {
                m.sk.x = 0.0;
                m.sk.v = 0.0;
            }
        }
        let sk = clamp(m.sk.x, 0.0, 1.0) * gate * under;
        if sk > 0.003 {
            pitch += MV2.shaken.pitch * (1.0 - MV2.shaken.brace * clamp(aim, 0.0, 1.0)) * sk;
            hp += MV2.shaken.head * sk;
            drop += MV2.shaken.drop * sk;
        }
        // the toss's body English: the left shoulder back with the hand, then through with it; a dip of the knees under the swing
        if ts >= 0.0 {
            let td = rel + TOSS_AFTER;
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
        po.gun = false;
        if dzw > 0.003 && lg_w > 0.02 {
            fk_run(rig, d, fk.rot, fk.pos);
            ran |= RAN_ALL;
            let hand = role(HAND_R);
            po.gun = true;
            po.gun_p = fk.at(hand);
            po.gun_r = fk.rot(hand);
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
        po.stw = stw;
        po.stn = stn;
        po.dzw = dzw;
        ran
    })
    .unwrap_or(0)
}

/// A turn's speed toward what is left of it (`err`), never faster than it can stop in and never carried past the point. Answers how
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
/// TURN: bodyYaw and aimYaw as his AI left them are where he WANTS to face; he comes round to them as a mass (his feet the short way round, the gun about his
/// waist through his front), and the two are rewritten with where he has got to. A rule of the character's and not of his pose.
pub fn turn_weigh(ch: &mut Character, dt: f64, tune: &Knobs) {
    let (px, pz, body_yaw, aim_yaw) = (ch.pos[0], ch.pos[2], ch.body_yaw, ch.aim_yaw);
    let stands = ch.tn.filter(|t| !(hypot2(px - t.x, pz - t.z) > tune.turn.jump) && finite(body_yaw + aim_yaw));
    let Some(mut t) = stands else {
        ch.tn = Some(CharacterTn { body: body_yaw, aim: aim_yaw, wb: 0.0, wa: 0.0, ib: body_yaw, ia: aim_yaw, x: px, z: pz });
        return;
    };
    if body_yaw != t.body {
        t.ib = body_yaw;
    }
    if aim_yaw != t.aim {
        t.ia = aim_yaw;
    }
    let (h, kk) = (clamp(ch.turn.h, 0.2, 1.5), lerp(tune.turn.green, 1.0, clamp(ch.turn.k, 0.0, 1.0)));
    let hb = lerp(1.0, h, tune.turn.body_share);
    let sb = turn_step(t.wb, wrap_angle(t.ib - t.body), tune.turn.body * hb, tune.turn.body_acc * hb * kk, dt);
    t.body = wrap_angle(t.body + sb.0);
    t.wb = sb.1;
    let r0 = clamp(wrap_angle(t.aim - t.body), -1.9, 1.9);
    let r1 = clamp(wrap_angle(t.ia - t.ib) + wrap_angle(t.ib - t.body), -1.9, 1.9);
    let sa = turn_step(t.wa, r1 - r0, tune.turn.aim * h, tune.turn.aim_acc * h * kk, dt);
    let rr = clamp(r0 + sa.0, -1.9, 1.9);
    t.wa = if rr == r0 + sa.0 { sa.1 } else { 0.0 };
    t.aim = wrap_angle(t.body + rr);
    (t.x, t.z) = (px, pz);
    (ch.body_yaw, ch.aim_yaw) = (t.body, t.aim);
    let (ld, was) = (clamp(wrap_angle(t.ia - t.aim), -tune.turn.lead, tune.turn.lead), ch.anim.turn_lead);
    ch.anim.turn_lead = damp(was, ld, if abs(ld) < abs(was) { tune.turn.lead_rate * 2.5 } else { tune.turn.lead_rate }, dt);
    ch.tn = Some(t);
}

/// A lean round an edge: the trunk rolled about the figure's fore-aft line over the three spine joints, the arms and the gun with it, and the neck and
/// head brought part of the way back so the eyes stay near level.
pub fn lean_body(g: usize, an: &mut Anim, dt: f64) -> u32 {
    with_pose(g, an.p_a.at as usize, |d, rig, n, _fk| {
        if !cast(n, &[SPINE1, SPINE2, SPINE3, NECK, HEAD]) {
            return 0;
        }
        let want = clamp(an.lean, -1.0, 1.0);
        an.lean_sm += (want - an.lean_sm) * min(1.0, LEAN_POSE.rate * dt);
        if want == 0.0 && abs(an.lean_sm) < 0.01 {
            an.lean_sm = 0.0;
        }
        let r = an.lean_sm * LEAN_POSE.deg * DEGR;
        if r != 0.0 {
            turn(d, rig, role(SPINE1), q_roll(r * 0.3));
            turn(d, rig, role(SPINE2), q_roll(r * 0.35));
            turn(d, rig, role(SPINE3), q_roll(r * 0.35));
            turn(d, rig, role(NECK), q_roll(-r * LEAN_POSE.level * 0.5));
            turn(d, rig, role(HEAD), q_roll(-r * LEAN_POSE.level * 0.5));
        }
        RAN
    })
    .unwrap_or(0)
}

/// A one-shot action's clip laid over the finished pose (the animator keeps which clip, its clock and its weight), and a man asleep in his chair slumped
/// over it.
pub fn act_pose(g: usize, an: &mut Anim, clip: f64, w: f64, dt: f64, tune: &Knobs) -> u32 {
    let (at, at_b, at_rest, t, looped, slump) = (an.p_a.at as usize, an.p_b.at as usize, an.rest.at as usize, an.act_t, an.act_loop, an.slump);
    let Some(Some(n)) = with_pose(g, at, |_d, _rig, n, _fk| if cast(n, &[SPINE3, NECK, HEAD]) { Some(n) } else { None }) else {
        return 0;
    };
    if !sample_on_rest(g, at_rest, at_b, n, clip, t, looped, tune) || !ask_blend(g, at, at_b, w, false, at, n, tune) {
        return 0;
    }
    let mut code = RAN;
    if slump > 0.0 || an.slump_w > 0.0 {
        let ds = slump - an.slump_w;
        an.slump_w += if ds > 0.0 { min(ds, dt / 1.6) } else { max(ds, -dt / 0.35) };
        let s = an.slump_w * w;
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
    code
}

/// The gaze stabiliser: the head kept level and steady while the body works under it. Measured off the finished pose (the head's chain alone), it works
/// out the pitch the neck and head take toward the gait's own eye line and the yaw wobble they take out against a half-second reference of itself. It
/// turns nothing: the host pushes the two twists.
pub fn gaze_keep(g: usize, an: &mut Anim, dt: f64, want_w: f64, crouch: f64, sprint: f64, fix: &mut f64) -> u32 {
    let (nod, hu, sk, gaze_local, v) = (an.nv_a, an.mvs.hu.x, an.mvs.sk.x, an.gaze_local, &mut an.gz);
    *fix = 0.0;
    with_pose(g, an.p_a.at as usize, |d, rig, n, fk| {
        if !cast(n, &[NECK, HEAD]) {
            return 0;
        }
        v.w = damp(v.w, want_w, 10.0, dt);
        if v.w > 0.01 {
            fk.part(rig, d, list(FK_HEAD));
            let gz = q_rotate(&fk.rot(role(HEAD)), &gaze_local);
            let (el, gyaw) = (asin(clamp(gz[1], -1.0, 1.0)) + nod, atan2(gz[0], gz[2]));
            let bow = (MV2.hurt.pitch + MV2.hurt.head) * clamp(hu, 0.0, 1.0) + (MV2.shaken.pitch + MV2.shaken.head) * clamp(sk, 0.0, 1.0);
            let target_el = -(GAZE_EL.stand + (GAZE_EL.crouch - GAZE_EL.stand) * crouch + (GAZE_EL.sprint - GAZE_EL.stand) * sprint * (1.0 - crouch) + bow) * DEGR;
            let want_pitch = clamp((el - target_el) * GAZE.keep, -30.0 * DEGR, 30.0 * DEGR);
            v.pitch = damp(v.pitch, want_pitch, 25.0, dt);
            if !v.yaw_init {
                v.yaw_ref = gyaw;
                v.yaw_init = true;
            }
            v.yaw_ref += wrap_angle(gyaw - v.yaw_ref) * clamp(dt / 0.5, 0.0, 1.0);
            *fix = clamp(-wrap_angle(gyaw - v.yaw_ref) * GAZE.yaw_damp, -12.0 * DEGR, 12.0 * DEGR);
            RAN | ran_list(FK_HEAD)
        } else {
            v.yaw_init = false;
            v.pitch = damp(v.pitch, 0.0, 10.0, dt);
            RAN
        }
    })
    .unwrap_or(0)
}

// ================================================================ THE ARMS
// An arm is posed on a COPY of the pose (a second pose of the pool, the animator's pB) and laid back through a mask: the copy is made with `ask_blend` at
// weight 1 (singles copied, none worked out), the limbs are solved on it, the mask is written from the named joints and
// the copy is blended in. The run-forward bones are then the COPY's, as the TypeScript leaves them.

/// both arms from the collar bones down (CharacterAnimator.armsMask)
fn arms_mask() {
    ask_mask(&[(role(SHOULDER_L), 1.0, true), (role(SHOULDER_R), 1.0, true)]);
}
const ARM_ROLES: [usize; 9] = [SPINE3, SHOULDER_L, SHOULDER_R, UPPER_ARM_L, UPPER_ARM_R, FOREARM_L, FOREARM_R, HAND_L, HAND_R];

/// The sneak's arms: hanging hands drawn in toward the thighs, or the two-hand carry brought down and back toward the lap, the support hand rigid with
/// wherever the gun hand got to.
pub fn sneak_arms(g: usize, an: &Anim, arms_in: f64, tune: &Knobs) -> u32 {
    let (at, at_b, _) = poses(an);
    let (crouch, at_ease) = (an.crouch, an.upper_w.relaxed);
    let Some(Some(n)) = with_pose(g, at, |_d, _rig, n, _fk| if cast(n, &ARM_ROLES) { Some(n) } else { None }) else {
        return 0;
    };
    if !ask_blend(g, at, at, 1.0, false, at_b, n, tune) {
        return 0;
    }
    let code = with_pose(g, at_b, |d, rig, _n, fk| {
        fk.part(rig, d, list(FK_ARMS));
        let (cr, r) = (clamp(crouch, 0.0, 1.0), clamp(at_ease, 0.0, 1.0));
        let kk = lerp(0.5, 1.0, cr);
        let (hand_r, hand_l) = (role(HAND_R), role(HAND_L));
        let (pr0, pl0, rr0, rl0) = (fk.at(hand_r), fk.at(hand_l), fk.rot(hand_r), fk.rot(hand_l));
        let gun_in = lerp3(&[SNEAK.gun_in_stand[0], SNEAK.gun_in_stand[1], SNEAK.gun_in_stand[2]], &[SNEAK.gun_in[0], SNEAK.gun_in[1], SNEAK.gun_in[2]], cr);
        let gun_r = add3(pr0, gun_in);
        // (the right hand lives at −x: in = toward the middle, never past it)
        let free_r = [pr0[0] + min(SNEAK.hand_in, max(0.0, -pr0[0])) * kk, pr0[1] - 0.02 * kk, pr0[2] - 0.03 * kk];
        let free_l = [pl0[0] - min(SNEAK.hand_in, max(0.0, pl0[0])) * kk, pl0[1] - 0.02 * kk, pl0[2] - 0.03 * kk];
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
    if !ask_blend(g, at, at_b, clamp(arms_in, 0.0, 1.0), true, at, n, tune) {
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
/// Both arms onto the pistol: the gun hand at the chest + the stance's [side, up, fwd], the muzzle at its pitch and level in roll, the support hand
/// keeping the exact hold it has in the aim clip (both hands moved by one rigid transform of the clip's grip); elbows swivelled down and in by the tuck;
/// a little sway, and during a kick a pump of the gun toward the chest. Solved on a copy seeded with the aim clip's arms (`at_aim`).
pub fn carry_arms(g: usize, an: &Anim, w: f64, kick_t: f64, tune: &Knobs) -> u32 {
    let (at, at_b, _) = poses(an);
    let (at_aim, t, st, grip_up) = (an.p_aim.at as usize, an.idle_t, &an.st, an.grip_up);
    let (st_fwd, st_up, st_side, st_pitch, st_blade, tuck) = (st.fwd.x, st.up.x, st.side.x, st.pitch.x, st.blade.x, st.tuck.x);
    let Some(Some(n)) = with_pose(g, at, |_d, _rig, n, _fk| if cast(n, &ARM_ROLES) && cast(n, &[MIDDLE1_R]) { Some(n) } else { None }) else {
        return 0;
    };
    // the body as it is, the aim clip's arms
    arms_mask();
    if !ask_blend(g, at, at, 1.0, false, at_b, n, tune) || !ask_blend(g, at_b, at_aim, 1.0, true, at_b, n, tune) {
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
    if !ask_blend(g, at, at_b, w, true, at, n, tune) {
        return 0;
    }
    code
}

// ─────────────────────────────── THE HANDS ───────────────────────────────

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
/// what stands between a man's hand pass and a layer that shapes a hand: the digits as the locomotion left them, the notes so far, the straight hand
fn hands_of(g: usize, an: &Anim) -> Option<Hands> {
    if digits(0).len() != HAND_DIGITS || digits(1).len() != HAND_DIGITS {
        return None;
    }
    let mut h = Hands { base: [0.0; 120], note: [0.0; 10], rest: [[0.0; 4]; 30] };
    for side in 0..2 {
        for (to, from) in h.base[side * 60..side * 60 + 60].iter_mut().zip(&an.hn.base[side]) {
            *to = *from as f64;
        }
        for (to, from) in h.note[side * 5..side * 5 + 5].iter_mut().zip(&an.hn.note[side]) {
            *to = *from as f64;
        }
    }
    let rest = &mut h.rest;
    with_pose(g, an.rest.at as usize, |d, _rig, n, _fk| {
        for side in 0..2 {
            for (i, &node) in digits(side).iter().enumerate() {
                if (node as usize) < n {
                    rest[side * HAND_DIGITS + i] = own_rot(d, node as usize);
                }
            }
        }
    })?;
    Some(h)
}
/// …and the notes back
fn hands_to(h: &Hands, an: &mut Anim) {
    for side in 0..2 {
        for (to, from) in an.hn.note[side].iter_mut().zip(&h.note[side * 5..side * 5 + 5]) {
            *to = *from as f32;
        }
    }
}
/// the poses a layer works on: his, the scratch, the rest
fn poses(an: &Anim) -> (usize, usize, usize) {
    (an.p_a.at as usize, an.p_b.at as usize, an.rest.at as usize)
}
/// what a Float32Array keeps of a double
#[inline(always)]
fn stored(v: f64) -> f64 {
    let mut s = [S::ZERO];
    put(&mut s, 0, v);
    get(&s, 0)
}
/// The angle between a stored rotation and one of four doubles
#[inline(always)]
fn q_ang(d: &[S], i: usize, b: &[f64], k: usize) -> f64 {
    let dot = abs(ch(d, i, 3) * b[k] + ch(d, i, 4) * b[k + 1] + ch(d, i, 5) * b[k + 2] + ch(d, i, 6) * b[k + 3]);
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
fn hand(d: &mut [S], h: &mut Hands, side: usize, curl: &[f64; 4], thumb: f64, spread: f64, tune: &Knobs) {
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
    if tune.moves.on > 0.5 {
        let b = &h.base[side * 60..side * 60 + 60];
        for g in 0..5 {
            let k = g * 3;
            let far = (q_ang(d, ns[k] as usize, b, k * 4) + q_ang(d, ns[k + 1] as usize, b, k * 4 + 4) + q_ang(d, ns[k + 2] as usize, b, k * 4 + 8)) / 3.0;
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

/// The wall hold's arms: both palms laid on the face beside the hips, fingers down and out along it, the elbows where the pose says; mended (WALLSNAP),
/// the wrists hang beside the hips, a resting hand on the plaster with the thumb beside the index.
pub fn wall_arms(g: usize, an: &mut Anim, w: f64, tune: &Knobs) -> u32 {
    let (at, at_b, _) = poses(an);
    let (cr, side_x, peek, yaw, gait, park, fixes) = (an.crouch, an.wst.side.x, an.wst.peek.x, an.wall_yaw, an.wall_gait, an.wst.park.x, [an.hand_fix[0], an.hand_fix[1]]);
    let Some(Some(n)) = with_pose(g, at, |_d, _rig, n, _fk| if cast(n, &ARM_ROLES) && cast(n, &[HIPS]) { Some(n) } else { None }) else {
        return 0;
    };
    let Some(mut h) = hands_of(g, an) else {
        return 0;
    };
    if !ask_blend(g, at, at, 1.0, false, at_b, n, tune) {
        return 0;
    }
    let hh = &mut h;
    let code = with_pose(g, at_b, |d, rig, _n, fk| {
        let (w_cr, s, pk) = (clamp(cr, 0.0, 1.0), clamp(side_x, -1.0, 1.0), clamp(peek, 0.0, 1.0));
        let e = pk * pk * (3.0 - 2.0 * pk);
        // the wall's normal (into the room) and his right along the wall, model space
        let nn = [sin(yaw), 0.0, cos(yaw)];
        let tr = [-nn[2], 0.0, nn[0]];
        fk_run(rig, d, fk.rot, fk.pos);
        let mend = tune.wallsnap.on && (1.0 != 0.0);
        let hips = fk.at(role(HIPS));
        let (hx, hy, hz) = (hips[0], hips[1] + if mend { lerp(WALL_POSE.mend.up[0], WALL_POSE.mend.up[1], w_cr) } else { lerp(WALL_POSE.hands.up, WALL_POSE.hands.up_crouch, w_cr) }, hips[2]);
        let along0 = if mend {
            lerp(WALL_POSE.mend.along[0], WALL_POSE.mend.along[1], w_cr) - WALL_POSE.mend.sway * cos(gait * 2.0 * PI) * (1.0 - clamp(park, 0.0, 1.0)) * lerp(WALL_POSE.gait.share[0], WALL_POSE.gait.share[1], w_cr)
        } else {
            lerp(WALL_POSE.hands.along, WALL_POSE.hands.along_crouch, w_cr)
        };
        let (e_out, e_room, e_down) = if mend { (WALL_POSE.mend.elbow.out, WALL_POSE.mend.elbow.room, WALL_POSE.mend.elbow.down) } else { (WALL_POSE.elbow.out, WALL_POSE.elbow.room, WALL_POSE.elbow.down) };
        let splay = if mend { WALL_POSE.mend.splay } else { WALL_POSE.hands.out };
        for side in 0..2usize {
            // his right is +tr: the right hand goes that way, the left the other
            let sg = if side == 1 { 1.0 } else { -1.0 };
            let along = along0 - WALL_POSE.hands.peek_in * e * (if side == 1 { 1.0 + s } else { 1.0 - s }) / 2.0;
            let mut p = [hx + tr[0] * sg * along, hy, hz + tr[2] * sg * along];
            // carried square onto the plane, the wrist proud of it
            let onto = (-WALL_ROOT_OFF + WALL_POSE.hands.off) - dot3(&p, &nn);
            p[0] += nn[0] * onto;
            p[2] += nn[2] * onto;
            let fix = fixes[side];
            let r = hand_rot(&fix, [tr[0] * sg * splay, -1.0, tr[2] * sg * splay], nn);
            let pole = [tr[0] * sg * e_out + nn[0] * e_room, -e_down, tr[2] * sg * e_out + nn[2] * e_room];
            let (up, fore, hnd) = if side == 1 { (role(UPPER_ARM_R), role(FOREARM_R), role(HAND_R)) } else { (role(UPPER_ARM_L), role(FOREARM_L), role(HAND_L)) };
            two_bone_ik(rig, d, fk.rot, fk.pos, up, fore, hnd, &p, Some(&pole), 0.9, Some(&r), false);
            if mend {
                hand(d, hh, side, &[WALL_POSE.mend.curl[0], WALL_POSE.mend.curl[1], WALL_POSE.mend.curl[2], WALL_POSE.mend.curl[3]], WALL_POSE.mend.thumb, WALL_POSE.mend.fan, tune);
                // the thumb beside the index, out of the plaster
                turn(d, rig, digits(side)[0] as usize, axis_angle(&q_rotate(&fk.rot(hnd), &AY), sg * WALL_POSE.mend.thumb_in * DEGR));
            } else {
                hand(d, hh, side, &[0.05, 0.06, 0.08, 0.12], 0.1, 0.5, tune);
            }
        }
        RAN | RAN_ALL
    })
    .unwrap_or(0);
    if code == 0 {
        return 0;
    }
    arms_mask();
    if !ask_blend(g, at, at_b, w, true, at, n, tune) {
        return 0;
    }
    hands_to(&h, an);
    code
}

/// CharacterAnimator.trunkFrame: the trunk's own left, up and forward, off the run-forward bones
fn trunk_frame(fk: &Fk) -> ([f64; 3], [f64; 3], [f64; 3]) {
    let up = normalize(sub3(fk.at(role(SPINE3)), fk.at(role(HIPS))));
    let l = sub3(fk.at(role(UPPER_ARM_L)), fk.at(role(UPPER_ARM_R)));
    let left = normalize(mad3(l, up, -dot3(&l, &up)));
    (left, up, cross3(&left, &up))
}

/// LC of character.ts: the long gun's channels, in the order the animator keeps them
const LC_SIDE: usize = 0;
const LC_UP: usize = 1;
const LC_FWD: usize = 2;
const LC_PITCH: usize = 3;
const LC_AIM_K: usize = 4;
const LC_YAW: usize = 5;
const LC_BLADE: usize = 6;
const LC_LEAN: usize = 7;
const LC_HEAD_YAW: usize = 8;
const LC_HEAD_PITCH: usize = 9;
const LC_HEAD_ROLL: usize = 10;
const LC_ELBOW_R: usize = 11;
const LC_ELBOW_L: usize = 14;
const LC_POLE_W: usize = 17;
const LC_SWAY: usize = 18;
const LC_ANCHOR: usize = 19;
const LC_STEADY: usize = 20;
pub const LC_COUNT: usize = 21;
/// The long gun's arms: the trigger hand's wrist at the carriage's point off the chest joint (or, by its anchor, off the gun shoulder), swung about the
/// shoulders with the aim's share of the pitch, holding the grip as the aim clip holds the pistol's turned onto the wanted bore; the support hand solved
/// onto the gun's grip point from where the trigger hand got to. Should the support arm fall short the whole gun is brought back by the shortfall
/// (capped) and both re-solved, and that hold-back is kept and let out over half a second. Solved on a copy seeded with the aim clip's arms.
pub fn long_arms(g: usize, an: &mut Anim, w: f64, kick_t: f64, dt: f64, yield_door: bool, keep: bool, ch: &[f64; LC_COUNT], rl_at: [f64; 3], tune: &Knobs) -> u32 {
    let (at, at_b, _) = poses(an);
    let at_aim = an.p_aim.at as usize;
    let (t, aim, aim_pitch, shoot_t, long_steady, trained, dip_w, rl_k) = (an.idle_t, an.upper_w.aim, an.aim_pitch, an.shoot_t, an.long_steady, an.trained, an.mv2_shown.dip, an.rlc.k);
    let (mvs_yaw, mvs_climb, mvs_back, dl, dr) = (an.mvs.yaw, an.mvs.climb, an.mvs.back, an.dp.w_l.x, an.dp.w_r.x);
    let (grip_up, fix_l, g_wrist, g_fingers, g_back) = (an.grip_up, an.hand_fix[0], an.long_grip.wrist, an.long_grip.fingers, an.long_grip.back);
    let Some(Some(n)) = with_pose(g, at, |_d, _rig, n, _fk| if cast(n, &ARM_ROLES) && cast(n, &[MIDDLE1_R]) { Some(n) } else { None }) else {
        return 0;
    };
    let Some(mut h) = hands_of(g, an) else {
        return 0;
    };
    // the body as it is, the aim clip's clavicles, trigger grip and fingers to start from
    arms_mask();
    if !ask_blend(g, at, at, 1.0, false, at_b, n, tune) || !ask_blend(g, at_b, at_aim, 1.0, true, at_b, n, tune) {
        return 0;
    }
    let hh = &mut h;
    let code = with_pose(g, at_b, |d, rig, _n, fk| {
        fk.part(rig, d, list(FK_ARMS));
        let (hand_r, hand_l) = (role(HAND_R), role(HAND_L));
        let (chest, pr0, rr0) = (fk.at(role(SPINE3)), fk.at(hand_r), fk.rot(hand_r));
        // the clip's bore as posed on this spine
        let g0 = normalize(sub3(fk.at(role(MIDDLE1_R)), pr0));
        // the bore, model space
        let sway = ch[LC_SWAY];
        let rcl = mv_on(tune.moves.recoil, tune);
        let yaw = (ch[LC_YAW] + 0.6 * sway * sin(t * 1.1) + if rcl { mvs_yaw } else { 0.0 }) * DEGR;
        let pitch0 = clamp((ch[LC_PITCH] + 0.8 * sway * sin(t * 1.7 + 1.1) + if rcl { mvs_climb } else { 0.0 }) * DEGR + aim_pitch * (ch[LC_AIM_K] - 0.35 * aim), -1.35, 1.35);
        // (only ever down, and only from a gun that is about level)
        let dip_k = if dip_w > 0.0 { dip_w * (1.0 - smoothstep(0.3, 0.6, pitch0)) } else { 0.0 };
        let pitch = if dip_k > 0.0 { lerp(pitch0, min(pitch0, MV2.dip.pitch * DEGR), dip_k) } else { pitch0 };
        if dip_k > 0.0 && w > 0.99 && mv_on(tune.moves.entry_eyes, tune) {
            an.sights_lift = pitch0 - pitch;
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
        anchor[0] = max(anchor[0], chest[0] - LONG_ANCHOR_SIDE_MAX);
        // steadied against the gait: the anchor's side-to-side followed through a two-pole low-pass
        {
            let (mut l0, mut l1) = (an.lg.lat[0], an.lg.lat[1]);
            if !finite(l0) || !finite(l1) {
                l0 = anchor[0];
                l1 = anchor[0];
            } else {
                let kk = 1.0 - exp(-dt / max(1e-3, LONG_STEADY.tau));
                l0 += (anchor[0] - l0) * kk;
                l1 += (l0 - l1) * kk;
            }
            if keep {
                an.lg.lat[0] = l0;
                an.lg.lat[1] = l1;
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
            let (py, pz) = (lerp(LONG_PIVOT[1], anchor[1] - chest[1], ka), lerp(LONG_PIVOT[2], anchor[2] - chest[2], ka));
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
            pr[1] -= MV2.dip.down * dip_k;
            pr[2] -= MV2.dip.back * dip_k;
        }
        if rcl {
            if mvs_back > 1e-5 {
                pr = mad3(pr, dd, -mvs_back);
            }
        } else if kick > 0.0 {
            pr = mad3(pr, dd, -LONG_KICK_BACK * kick);
        }
        let mut pole_r = [ch[LC_ELBOW_R], ch[LC_ELBOW_R + 1], ch[LC_ELBOW_R + 2]];
        let mut pole_l = [ch[LC_ELBOW_L], ch[LC_ELBOW_L + 1], ch[LC_ELBOW_L + 2]];
        let pw = clamp(ch[LC_POLE_W], 0.0, 1.0);
        if mv_on(tune.moves.gait, tune) {
            // the elbows by training, shouldered
            let (tw, uw) = (smoothstep(0.5, 1.0, trained) * aim, (1.0 - smoothstep(0.0, 0.5, trained)) * aim);
            pole_r[0] += tune.mv.gait.elbow_in * tw;
            pole_l[0] -= tune.mv.gait.elbow_in * 0.8 * tw;
            pole_r[1] += tune.mv.gait.wing * uw;
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
        let mut pull = [an.lg.pull[0], an.lg.pull[1], an.lg.pull[2]];
        let m = solve(d, fk, add3(pr, pull));
        if len3(&m) > 5e-4 {
            // still short with what is held: take up the rest now (capped), and pose there
            pull = [pull[0] + m[0], pull[1] + m[1], pull[2] + m[2]];
            let l = len3(&pull);
            if l > LONG_PULL_MAX {
                let kk = LONG_PULL_MAX / l;
                pull = [pull[0] * kk, pull[1] * kk, pull[2] * kk];
            }
            solve(d, fk, add3(pr, pull));
        } else if keep {
            // reached: let a little out for next frame
            let kk = exp(-2.0 * dt);
            pull = [pull[0] * kk, pull[1] * kk, pull[2] * kk];
        }
        if keep {
            an.lg.pull[0] = pull[0];
            an.lg.pull[1] = pull[1];
            an.lg.pull[2] = pull[2];
        }
        // the support hand cupped round the handguard
        hand(d, hh, 0, &[0.5, 0.55, 0.6, 0.68], 0.35, 0.0, tune);
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
        if !ask_blend(g, at, at_b, wl, true, at, n, tune) {
            return 0;
        }
        ask_mask(&[(role(SHOULDER_R), 1.0, true)]);
        if !ask_blend(g, at, at_b, wr, true, at, n, tune) {
            return 0;
        }
    } else {
        arms_mask();
        if !ask_blend(g, at, at_b, w, true, at, n, tune) {
            return 0;
        }
    }
    hands_to(&h, an);
    code
}

/// The scalar curve's spline for a keyed vector, one search for the segment and the same Hermite weights for each component
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

/// The reactions' arms, after their body: the start's shoulders and elbows on the pose at `at`; then, each solved on the scratch pose at `at_b` and laid
/// back through its own mask, a blinded man's gun held on the line it had, his hand flat over his eyes, and the toss of the support hand along its keyed
/// path.
pub fn mv_post_arms(g: usize, an: &mut Anim, dt: f64, gate: f64, tune: &Knobs) -> u32 {
    let (at, at_b, _) = poses(an);
    let (lg_w, gaze, fix_l, m, po) = (an.lg.w.x, an.gaze_local, an.hand_fix[0], &an.mvs, &an.mv_po);
    let (rel, dz_t, ts, stw, stn, dzw, gun, gun_p, gun_r) = (m.ts_rel, m.dz_t, m.ts_t, po.stw, po.stn, po.dzw, po.gun, po.gun_p, po.gun_r);
    let (td, mut arm_l) = (rel + TOSS_AFTER, m.arm_l);
    let Some(Some(n)) = with_pose(g, at, |_d, _rig, n, _fk| if cast(n, &ARM_ROLES) && cast(n, &[HEAD]) { Some(n) } else { None }) else {
        return 0;
    };
    let Some(mut h) = hands_of(g, an) else {
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
                turn(d, rig, sh, q_roll(sg * MV_STARTLE.shrug * DEGR * stw));
                if aa > 0.003 {
                    turn(d, rig, up, q_roll(sg * (if far { MV_STARTLE.wide[1] } else { MV_STARTLE.wide[0] }) * DEGR * aa));
                    turn(d, rig, fo, q_pitch(-(if far { MV_STARTLE.raise[1] } else { MV_STARTLE.raise[0] }) * DEGR * aa));
                }
            }
        });
    }
    if gun {
        // a blinded man's gun stays on the line it had, by how much of a long gun he has up
        if !ask_blend(g, at, at, 1.0, false, at_b, n, tune) {
            return 0;
        }
        with_pose(g, at_b, |d, rig, _n, fk| {
            fk_run(rig, d, fk.rot, fk.pos);
            two_bone_ik(rig, d, fk.rot, fk.pos, role(UPPER_ARM_R), role(FOREARM_R), role(HAND_R), &gun_p, Some(&[-0.3, -1.0, -0.4]), 0.6, Some(&gun_r), false);
        });
        ask_mask(&[(role(UPPER_ARM_R), smoothstep(0.003, 0.15, dzw) * smoothstep(0.02, 0.6, lg_w), true)]);
        if !ask_blend(g, at, at_b, 1.0, true, at, n, tune) {
            return 0;
        }
        code |= RAN_ALL;
    }
    if dzw > 0.003 {
        // blinded: the support hand goes flat over the eyes, wrist by the near eye, fingers across to the far one
        if !ask_blend(g, at, at, 1.0, false, at_b, n, tune) {
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
            let to = mad3(mad3(mad3(eyes, gz, tune.mv.dazzle.hand_out), left, 0.04), up, 0.006 * sin(dz_t * 4.3) - tune.mv.dazzle.hand_drop);
            let pole = [tune.mv.dazzle.hand_pole[0], tune.mv.dazzle.hand_pole[1], tune.mv.dazzle.hand_pole[2]];
            let r = hand_rot(&fix_l, mad3(scale3(left, -1.0), up, tune.mv.dazzle.hand_tilt), mad3(gz, up, tune.mv.dazzle.hand_roll));
            two_bone_ik(rig, d, fk.rot, fk.pos, up_l, fore_l, hand_l, &to, Some(&pole), 0.85, Some(&r), false);
            hand(d, hh, 0, &[0.22, 0.28, 0.34, 0.4], 0.3, 0.3, tune);
        });
        let wz = smoothstep(0.08, 0.6, dzw);
        ask_mask(&[(sh_l, wz, true)]);
        if !ask_blend(g, at, at_b, 1.0, true, at, n, tune) {
            return 0;
        }
        arm_l = max(arm_l, wz);
        code |= RAN_ALL;
    }
    let mut ts_out = ts;
    if ts >= 0.0 {
        // the toss: the support hand off the gun, down beside the hip, through and up, back to the gun (a path off the shoulder; its ends are the live hand)
        if !ask_blend(g, at, at, 1.0, false, at_b, n, tune) {
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
            hand(d, hh, 0, &[c(0.6, 0.05, 0.5), c(0.65, 0.08, 0.55), c(0.7, 0.1, 0.6), c(0.78, 0.14, 0.68)], c(0.7, 0.1, 0.35), 0.25 * open * (1.0 - recup), tune);
            // the wrist's turn eases from and back to the grip it had on the gun at the ends (the path does the same for its place)
            wrist_ease(d, rig, fk, hand_l, &rot0, bump(ts, 0.0, 0.08, td - 0.22, td - 0.04));
        });
        let hold = bump(ts, 0.0, 0.05, td - 0.2, td);
        arm_l = max(arm_l, hold);
        ask_mask(&[(sh_l, hold * gate, true), (sh_l, 0.0, false)]);
        if !ask_blend(g, at, at_b, 1.0, true, at, n, tune) {
            return 0;
        }
        ts_out = ts + dt;
        if ts_out > td {
            ts_out = -1.0;
        }
        code |= RAN_ALL;
    }
    (an.mvs.ts_t, an.mvs.arm_l) = (ts_out, arm_l);
    hands_to(&h, an);
    code
}

/// The slip's arm: the one on the man's side goes out low toward him, the hand open. Solved on the scratch pose at `at_b` and laid back through that
/// shoulder's mask.
pub fn watch_arm(g: usize, an: &mut Anim, wl: f64, wr: f64, tune: &Knobs) -> u32 {
    let (at, at_b, _) = poses(an);
    let (t, f) = (an.wt, &an.hand_fix);
    let (sw, sl, left, sx, sz, fix) = (t.sw, t.sl, t.side == 0.0, t.sx, t.sz, [f[0], f[1]]);
    let Some(Some(n)) = with_pose(g, at, |_d, _rig, n, _fk| if cast(n, &ARM_ROLES) { Some(n) } else { None }) else {
        return 0;
    };
    if !(sw > 0.003) {
        return RAN;
    }
    let kl = sw * sw * (3.0 - 2.0 * sw) * sl;
    let sgn = if left { 1.0 } else { -1.0 };
    let w_arm = (if left { wl } else { wr }) * kl * tune.watch.arm_w * smoothstep(tune.watch.arm_side[0], tune.watch.arm_side[1], sgn * sx);
    if !(w_arm > 0.003) {
        return RAN;
    }
    let Some(mut h) = hands_of(g, an) else {
        return 0;
    };
    if !ask_blend(g, at, at, 1.0, false, at_b, n, tune) {
        return 0;
    }
    let side = if left { 0 } else { 1 };
    let fix = fix[side];
    let (up, fore, hand_n, sh_n) = if left { (role(UPPER_ARM_L), role(FOREARM_L), role(HAND_L), role(SHOULDER_L)) } else { (role(UPPER_ARM_R), role(FOREARM_R), role(HAND_R), role(SHOULDER_R)) };
    let hh = &mut h;
    with_pose(g, at_b, |d, rig, _n, fk| {
        fk_run(rig, d, fk.rot, fk.pos);
        let sh = fk.at(up);
        let to = [sh[0] + sx * tune.watch.arm_out, sh[1] - tune.watch.arm_down, sh[2] + sz * tune.watch.arm_out];
        let r = hand_rot(&fix, normalize([sx * 0.75, -0.55, sz * 0.75]), [-sx * 0.3, 1.0, -sz * 0.3]);
        two_bone_ik(rig, d, fk.rot, fk.pos, up, fore, hand_n, &to, Some(&[sgn * 0.5, -0.3, -0.8]), 0.6, Some(&r), false);
        hand(d, hh, side, &[0.12, 0.15, 0.18, 0.22], 0.15, 0.35, tune);
    });
    ask_mask(&[(sh_n, w_arm, true)]);
    if !ask_blend(g, at, at_b, 1.0, true, at, n, tune) {
        return 0;
    }
    hands_to(&h, an);
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
    let t = normalize(q_rotate(&q, &RIG.sheath.along));
    let b0 = q_rotate(&q, &RIG.sheath.out);
    let b = normalize(mad3(b0, t, -dot3(&b0, &t)));
    let f = cross3(&t, &b);
    let h = mad3(mad3(add3(fk.at(s3), q_rotate(&q, &RIG.sheath.at)), t, -pull), b, off);
    let to = mad3(mad3(mad3(h, f, -RIG.knife.grip.fwd), b, RIG.knife.grip.palm), t, -RIG.knife.grip.side);
    let pole = mad3(normalize(q_rotate(&q, &REACH.pole)), AY, lerp(REACH.pole_up[0], REACH.pole_up[1], clamp(crouch, 0.0, 1.0)));
    (to, knife_rot(fix_r, t, b), pole)
}

/// The close-in carriage's arms, after its body: the shoulders rounded and the hands brought up ahead of the hips in the shoulders' frame, let out wide
/// and low by `ws`, alive with a slow working of the fingers when the distance is dealt, and the knife hand taken up across the chest to hover off the
/// hilt as he closes. Solved on the scratch pose at `at_b`, laid back joint by joint down each arm.
pub fn creep_arms(g: usize, an: &mut Anim, w: f64, wl: f64, wr: f64, e: f64, cr: f64, ck: f64, cke: f64, ws: f64, ha: f64, hg: f64, rc_inv: [f64; 4], tune: &Knobs) -> u32 {
    let (at, at_b, _) = poses(an);
    let (cl_t, held, no_knife, crouch, swing, wsh) = (an.cl_t, an.grip[1], an.no_knife, an.crouch, an.heft_shape[1], an.wide_shape);
    let (wide_pole, fix_l, fix_r) = (an.wide_pole.unwrap_or(CREEP.wide.pole), an.hand_fix[0], an.hand_fix[1]);
    let Some(Some(n)) = with_pose(g, at, |_d, _rig, n, _fk| if cast(n, &ARM_ROLES) { Some(n) } else { None }) else {
        return 0;
    };
    let Some(mut h) = hands_of(g, an) else {
        return 0;
    };
    if !ask_blend(g, at, at, 1.0, false, at_b, n, tune) {
        return 0;
    }
    let (on_l, on_r) = (wl > 0.003, wr > 0.003);
    let (w_up, w_fore, w_hand) = (smoothstep(0.0, 0.75, w), smoothstep(0.08, 0.88, w), smoothstep(0.16, 1.0, w));
    let dealt = ck >= 0.0;
    let lw = wl * if dealt { lerp(tune.kit.close.left, 1.0, ws) } else { 1.0 };
    let hh = &mut h;
    with_pose(g, at_b, |d, rig, _n, fk| {
        let rnd = CREEP.round * DEGR * e;
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
        let lt = cl_t * PI * 2.0 * tune.kit.close.flex;
        let fl_l = if dealt { tune.kit.close.flex_amp * (0.6 * sin(lt) + 0.4 * sin(lt * 1.63 + 1.1)) } else { 0.0 };
        let fl_r = if dealt { tune.kit.close.flex_amp * (0.6 * sin(lt * 1.21 + 2.3) + 0.4 * sin(lt * 0.71 + 0.4)) } else { 0.0 };
        let drift = |ph: f64| -> [f64; 3] {
            if dealt {
                [tune.kit.close.drift * sin(lt * 0.83 + ph), tune.kit.close.drift * sin(lt * 1.17 + ph * 1.7), tune.kit.close.drift * 0.6 * sin(lt * 0.61 + ph * 0.6)]
            } else {
                [0.0, 0.0, 0.0]
            }
        };
        let (wd_fing, wd_back) = (CREEP.wide.fing, CREEP.wide.back);
        if on_l {
            let hd = &CREEP.l;
            let (mut pole, mut fing, mut back, mut cu, mut th, mut sp) = (hd.pole, hd.fing, hd.back, hd.curl, hd.thumb, hd.spread);
            // (the drift at its full size through the part of this arm the layer has)
            let mut to = add3(
                mad3(lerp3(&hd.at, &hd.at_c, cr), tune.kit.close.lead, cke),
                scale3(drift(0.0), if dealt { 1.0 / max(0.38, lerp(tune.kit.close.left, 1.0, ws)) } else { 1.0 }),
            );
            if ws > 0.0 {
                // out to its side, and still
                let s = -ha * swing;
                to = lerp3(&to, &[wsh[0] + s * CREEP.heft.out, -wsh[1] + abs(s) * CREEP.heft.up, wsh[2] + wsh[3] + s - hg * 0.1], ws);
                pole = lerp3(&pole, &wide_pole, ws);
                fing = lerp3(&fing, &wd_fing, ws);
                back = lerp3(&back, &wd_back, ws);
                cu = lerp(cu, CREEP.wide.curl, ws);
                th = lerp(th, CREEP.wide.thumb, ws);
                sp = lerp(sp, CREEP.wide.spread, ws);
            }
            let r = hand_rot(&fix_l, sq(fing), sq(back));
            two_bone_ik(rig, d, fk.rot, fk.pos, role(UPPER_ARM_L), role(FOREARM_L), role(HAND_L), &add3(sl, sq(to)), Some(&sq(pole)), 0.8, Some(&r), false);
            hand(d, hh, 0, &[cu + fl_l, cu + 0.05 + fl_l * 0.9, cu + 0.1 + fl_l * 0.8, cu + 0.15 + fl_l * 0.7], th + fl_l * 0.5, sp, tune);
        }
        if on_r {
            let hd = &CREEP.r;
            let mir = |v: [f64; 3]| -> [f64; 3] { [-v[0], v[1], v[2]] };
            let (mut a0, mut p0, mut fing, mut back) = (add3(lerp3(&hd.at, &hd.at_c, cr), drift(2.1)), hd.pole, hd.fing, hd.back);
            let (mut curl, mut thb, mut spr) = (hd.curl, hd.thumb, hd.spread * (1.0 - cke));
            if ws > 0.0 {
                // (the left's mirror)
                let s = ha * swing;
                a0 = lerp3(&a0, &[-wsh[0] - s * CREEP.heft.out, -wsh[1] + abs(s) * CREEP.heft.up, wsh[2] - wsh[3] + s - hg * 0.1], ws);
                p0 = lerp3(&p0, &mir(wide_pole), ws);
                fing = lerp3(&fing, &mir(wd_fing), ws);
                back = lerp3(&back, &mir(wd_back), ws);
                curl = lerp(curl, CREEP.wide.curl, ws);
                thb = lerp(thb, CREEP.wide.thumb, ws);
                spr = lerp(spr, CREEP.wide.spread, ws);
            }
            let (mut to, mut rot, mut pole) = (add3(sr, sq(a0)), hand_rot(&fix_r, sq(fing), sq(back)), sq(p0));
            if dealt && !held && !no_knife {
                // the knife hand by distance: from its guard up across the chest to hover off the hilt
                let (s_at, s_rot, s_pole) = sheath_fist(fk, &rc_inv, &fix_r, crouch, 0.0, tune.kit.close.hover);
                // (the travel half-eased: the hand must be seen to go with every step)
                let cp = cke + (ck - cke) * 0.5;
                to = add3(mad3(lerp3(&to, &s_at, cp), sq(tune.kit.close.bow), sin(PI * cke)), scale3(sq(drift(2.1)), cke));
                rot = slerp(&rot, &s_rot, cke);
                pole = lerp3(&pole, &s_pole, cke);
                curl = lerp(hd.curl, tune.kit.close.curl, cke);
            }
            two_bone_ik(rig, d, fk.rot, fk.pos, role(UPPER_ARM_R), role(FOREARM_R), role(HAND_R), &to, Some(&pole), 0.8, Some(&rot), false);
            if !held {
                hand(d, hh, 1, &[curl + fl_r, curl + 0.05 + fl_r * 0.9, curl + 0.1 + fl_r * 0.8, curl + 0.15 + fl_r * 0.7], thb + fl_r * 0.5, spr, tune);
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
    if !ask_blend(g, at, at_b, 1.0, true, at, n, tune) {
        return 0;
    }
    hands_to(&h, an);
    RAN | RAN_ALL
}

/// the support arm from the upper arm down (the collar bone stays the clip's), or both arms
fn arm_mask(both: bool) {
    ask_mask(&[(role(UPPER_ARM_L), 1.0, true), (if both { role(UPPER_ARM_R) } else { usize::MAX }, 1.0, true)]);
}

/// A man knocked about and still up: the support hand off the gun and pressed to his ribs. Solved on the scratch pose at `at_b`, laid on the pose by w
/// through the support arm's mask.
pub fn clutch_arm(g: usize, an: &mut Anim, w: f64, tune: &Knobs) -> u32 {
    let (at, at_b, _) = poses(an);
    let fix_l = an.hand_fix[0];
    let Some(Some(n)) = with_pose(g, at, |_d, _rig, n, _fk| if cast(n, &ARM_ROLES) && cast(n, &[HIPS, SPINE1]) { Some(n) } else { None }) else {
        return 0;
    };
    let Some(mut h) = hands_of(g, an) else {
        return 0;
    };
    if !ask_blend(g, at, at, 1.0, false, at_b, n, tune) {
        return 0;
    }
    let hh = &mut h;
    with_pose(g, at_b, |d, rig, _n, fk| {
        fk_run(rig, d, fk.rot, fk.pos);
        let (left, up, fwd) = trunk_frame(fk);
        let p = mad3(mad3(mad3(fk.at(role(SPINE1)), left, MV2.clutch.at[0]), up, MV2.clutch.at[1]), fwd, MV2.clutch.at[2]);
        let r = hand_rot(&fix_l, mad3(scale3(left, -1.0), up, -0.25), mad3(fwd, up, 0.2));
        two_bone_ik(rig, d, fk.rot, fk.pos, role(UPPER_ARM_L), role(FOREARM_L), role(HAND_L), &p, Some(&MV2.clutch.pole), 0.7, Some(&r), false);
        hand(d, hh, 0, &[0.3, 0.34, 0.38, 0.42], 0.3, 0.2, tune);
    });
    arm_mask(false);
    if !ask_blend(g, at, at_b, w, true, at, n, tune) {
        return 0;
    }
    hands_to(&h, an);
    RAN | RAN_ALL
}

/// Kept down: the wrists crossed in the small of the back, palms out (kind 1), or the fingers laced on the crown with the elbows wide, in the head's own
/// frame (kind 2). Both arms, solved on the scratch pose at `at_b` and laid on the pose by w.
pub fn bound_arms(g: usize, an: &mut Anim, w: f64, kind: f64, tune: &Knobs) -> u32 {
    let (at, at_b, _) = poses(an);
    let (back, gaze, fix_l, fix_r) = (kind == 1.0, an.gaze_local, an.hand_fix[0], an.hand_fix[1]);
    let Some(Some(n)) = with_pose(g, at, |_d, _rig, n, _fk| if cast(n, &ARM_ROLES) && cast(n, &[HIPS, HEAD]) { Some(n) } else { None }) else {
        return 0;
    };
    let Some(mut h) = hands_of(g, an) else {
        return 0;
    };
    if !ask_blend(g, at, at, 1.0, false, at_b, n, tune) {
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
            let (b_up, b_out, b_behind) = (MV2.bound.back.up, MV2.bound.back.out, MV2.bound.back.behind);
            let (to, r) = (tk(o, -b_out * 0.4, b_up, -b_behind), hand_rot(&fix_l, tv(-0.75, -0.6, -0.1), tv(0.0, 0.0, 1.0)));
            two_bone_ik(rig, d, fk.rot, fk.pos, up_l, fore_l, hand_l, &to, Some(&tv(0.8, 0.1, -0.6)), 0.8, Some(&r), false);
            let (to, r) = (tk(o, b_out * 0.4, b_up + 0.02, -b_behind - 0.02), hand_rot(&fix_r, tv(0.75, -0.6, -0.1), tv(0.0, 0.0, 1.0)));
            two_bone_ik(rig, d, fk.rot, fk.pos, up_r, fore_r, hand_r, &to, Some(&tv(-0.8, 0.1, -0.6)), 0.8, Some(&r), false);
            // a loose half fist each
            let curl = [MV2.bound.back.curl[0], MV2.bound.back.curl[1], MV2.bound.back.curl[2], MV2.bound.back.curl[3]];
            hand(d, hh, 0, &curl, MV2.bound.back.thumb, 0.0, tune);
            hand(d, hh, 1, &curl, MV2.bound.back.thumb, 0.0, tune);
        } else {
            // fingers laced on the crown, elbows wide: in the head's own frame, up its crown and forward along its gaze
            let hr = fk.rot(role(HEAD));
            let hu = q_rotate(&hr, &AY);
            let hf0 = q_rotate(&hr, &gaze);
            let hf = normalize(mad3(hf0, hu, -dot3(&hf0, &hu)));
            let hl = cross3(&hu, &hf);
            let o = mad3(mad3(fk.at(role(HEAD)), hu, MV2.bound.head.up), hf, -MV2.bound.head.behind);
            let hk = |x: f64, y: f64, z: f64| mad3(mad3(scale3(hl, x), hu, y), hf, z);
            let (out, elbow) = (MV2.bound.head.out, MV2.bound.head.elbow);
            let r = hand_rot(&fix_l, hk(-0.84, 0.5, 0.04), hk(0.2, 0.75, -0.6));
            two_bone_ik(rig, d, fk.rot, fk.pos, up_l, fore_l, hand_l, &mad3(o, hl, out), Some(&tv(1.0, -0.15, elbow)), 0.9, Some(&r), false);
            let r = hand_rot(&fix_r, hk(0.84, 0.5, 0.04), hk(-0.2, 0.75, -0.6));
            two_bone_ik(rig, d, fk.rot, fk.pos, up_r, fore_r, hand_r, &mad3(o, hl, -out), Some(&tv(-1.0, -0.15, elbow)), 0.9, Some(&r), false);
            // open hands, the fingers a little apart, and bent down at the knuckles onto the crown (an angle a side)
            let curl = [MV2.bound.head.curl[0], MV2.bound.head.curl[1], MV2.bound.head.curl[2], MV2.bound.head.curl[3]];
            hand(d, hh, 0, &curl, MV2.bound.head.thumb, 0.25, tune);
            hand(d, hh, 1, &curl, MV2.bound.head.thumb, 0.25, tune);
            for (sd, kn) in [(0, MV2.bound.head.knuckle[0]), (1, MV2.bound.head.knuckle[1])] {
                let ns = digits(sd);
                for fi in 0..4 {
                    let i = ns[3 + fi * 3] as usize;
                    set_unit(d, i, &q_mul(&own_rot(d, i), &axis_angle(&AX, kn * DEGR)));
                }
            }
        }
    });
    arm_mask(true);
    if !ask_blend(g, at, at_b, w, true, at, n, tune) {
        return 0;
    }
    hands_to(&h, an);
    RAN | RAN_ALL
}

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
/// CharacterAnimator.reloadArm (and its feedArm) — the long gun's magazine change, the support hand's: to the well, the empty stripped, a fresh one
/// fetched and seated, the handle run, back to the fore-end; an untrained man's lands on the lip, is backed off and found, and is slapped home. Or, for
/// a gun fed its own way, the hand through the stations it is handed. Solved on the scratch pose at `at_b` in the gun's frame as the trigger hand has
/// it, laid on the pose by w through the support arm's mask.
pub fn reload_arm(g: usize, an: &mut Anim, t: f64, w: f64, marks: &FeedMarks, keys: Option<&[FeedKey]>, tune: &Knobs) -> u32 {
    let (at, at_b, _) = poses(an);
    let (tr, hip_line, bt, fix_l) = (clamp(an.trained, 0.0, 1.0), an.long_gun, an.rl_b, an.hand_fix[0]);
    if keys.map_or(0, |k| k.len()) > KEYS_MAX {
        return 0;
    }
    let Some(Some(n)) = with_pose(g, at, |_d, _rig, n, _fk| if cast(n, &ARM_ROLES) && cast(n, &[MIDDLE1_R, HIPS, SPINE2, THIGH_L, THIGH_R]) { Some(n) } else { None }) else {
        return 0;
    };
    let Some(mut h) = hands_of(g, an) else {
        return 0;
    };
    if !ask_blend(g, at, at, 1.0, false, at_b, n, tune) {
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
        let pouch = if tr > 0.5 { tk(fk.at(role(SPINE2)), MV2.reload.pouch[1]) } else { hk(MV2.reload.pouch[0]) };
        let (r_pole, pole_w) = (MV2.reload.pole, MV2.reload.pole_w);
        let (up_l, fore_l) = (role(UPPER_ARM_L), role(FOREARM_L));
        let (o, i, r, dur) = (bt.out, bt.in_, bt.rack, bt.dur);
        // the hand: round a thing from its front (fingers across it, palm toward him) · fingers down into the pouch
        let (mf, mb) = (gun.dir(&[-1.0, 0.15, 0.1]), gun.dir(&[0.1, -0.35, 1.0]));
        let (pf, pb) = (mad3(mad3(scale3(up, -0.9), left, -0.3), fwd, 0.1), mad3(fwd, left, 0.3));
        if let Some(keys) = keys {
            // a gun fed its own way: the hand through its stations, and back to where it was
            if keys.len() < 2 {
                return false;
            }
            // …or flat on top of the gun (fingers across it, the back of the hand up)
            let (tf, tb) = (gun.dir(&[-1.0, 0.0, 0.15]), gun.dir(&[0.1, 1.0, 0.1]));
            let (mut p, mut f, mut b) = (Path::of(&[(0.0, cur)]), Path::new(), Path::new());
            for key in keys {
                p.push(
                    key.t,
                    match key.at {
                        FeedKeyAt::Pouch => pouch,
                        FeedKeyAt::List(v) => gun.at(&v),
                    },
                );
                let (ff, bb) = match key.how {
                    FeedKeyHow::Top => (tf, tb),
                    FeedKeyHow::Pouch => (pf, pb),
                    FeedKeyHow::Mag => (mf, mb),
                };
                f.push(key.t, ff);
                b.push(key.t, bb);
            }
            // (the hand rests where its last work was until the weapon is nearly done, then back to the gun)
            let end = keys[keys.len() - 1].t;
            if end + 0.1 < dur - 0.3 {
                let last = p.k[p.n - 1].1;
                p.push(dur - 0.3, last);
            }
            p.push(max(dur - 0.03, end + 0.02), cur);
            let rot = hand_rot(&fix_l, f.at(t), b.at(t));
            two_bone_ik(rig, d, fk.rot, fk.pos, up_l, fore_l, hand_l, &p.at(t), Some(&r_pole), pole_w, Some(&rot), false);
            let (first, last) = (&keys[0], &keys[keys.len() - 1]);
            let mut gr = if t <= first.t { first.grip * smoothstep(first.t - 0.12, first.t, t) } else { last.grip * (1.0 - smoothstep(end, end + 0.12, t)) };
            for j in 0..keys.len() - 1 {
                let (ka, kb) = (&keys[j], &keys[j + 1]);
                if t > ka.t && t <= kb.t {
                    gr = lerp(ka.grip, kb.grip, (t - ka.t) / (kb.t - ka.t));
                    break;
                }
            }
            let c = |kk: f64| lerp(0.3, kk, gr);
            hand(d, hh, 0, &[c(0.62), c(0.68), c(0.72), c(0.78)], lerp(0.3, 0.6, gr), 0.1 * (1.0 - gr), tune);
            wrist_ease(d, rig, fk, hand_l, &rot0, bump(t, 0.0, 0.12, dur - 0.26, dur - 0.05));
            return true;
        }
        let half = (o + i) * 0.5;
        let (mag, strip, hd, pl) = (gun.at(&marks.mag), gun.at(&marks.strip), gun.at(&marks.handle), gun.at(&marks.pull));
        // the empty comes straight down out of the well and is let go there; the fresh one comes up from under. An untrained man's lands on the lip beside
        // the well, is backed off, then found; seated, he slaps it home; and the handle let go, the hand flies on back.
        let wob = MV2.reload.fumble * (1.0 - tr);
        let fum = wob > 0.004;
        let hp = if fum { half - 0.1 } else { half };
        let slap = fum && bt.pull && r - i >= 0.4;
        let fl = if fum { (1.0 - tr) * max(bump(t, i - 0.36, i - 0.22, i - 0.09, i + 0.02), if slap { bump(t, i + 0.07, i + 0.15, i + 0.19, i + 0.27) } else { 0.0 }) } else { 0.0 };
        let hit = mad3(mad3(mad3(mag, gun.up, -0.035), gun.left, wob * 1.2), gun.bore, wob * 1.6);
        let off = if fum { tk(lerp3(&hit, &strip, 0.65), scale3(MV2.reload.flare.off, 1.0 - tr)) } else { hit };
        let tap_p = if slap { tk(mad3(mag, gun.up, -MV2.reload.tap * (1.0 - tr)), scale3(MV2.reload.flare.tap, 1.0 - tr)) } else { mag };
        let over = mad3(mad3(pl, gun.bore, -(MV2.reload.fling + 0.1 * (1.0 - tr))), gun.left, 0.04 + 0.05 * (1.0 - tr));
        // (a magazine with rounds in it goes to the dump pouch on the hip before the hand goes for a fresh one)
        let (kp, fr) = (bt.keep, bt.fresh);
        let dump = if kp >= 0.0 { hk(tune.kit.mag.dump) } else { pouch };
        let mut p = Path::of(&[(0.0, cur), (max(0.1, o - 0.2), mag), (o, mag), (o + 0.12, strip)]);
        if kp >= 0.0 {
            p.push((o + 0.05 + kp) * 0.5, hk(tune.kit.mag.via));
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
        let (hf_d, hb_d) = (gun.dir(&marks.hf), gun.dir(&marks.hb));
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
        let pole = if fl > 0.0 { lerp3(&r_pole, &MV2.reload.flare.pole, fl) } else { r_pole };
        two_bone_ik(rig, d, fk.rot, fk.pos, up_l, fore_l, hand_l, &p.at(t), Some(&pole), pole_w, Some(&rot), false);
        let grip = if kp >= 0.0 { max(bump(t, o - 0.12, o - 0.02, kp - 0.02, kp + 0.06), bump(t, fr, fr + 0.08, i + 0.04, i + 0.12)) } else { bump(t, o - 0.12, o - 0.02, i + 0.04, i + 0.12) };
        let hook = if bt.pull { bump(t, r - 0.14, r - 0.04, r + 0.14, r + 0.22) } else { 0.0 };
        let c = |kk: f64| lerp(0.3, kk, max(grip, hook));
        hand(d, hh, 0, &[c(0.62), c(0.68), c(0.72), c(0.78)], lerp(0.3, 0.6, grip), 0.1 * (1.0 - grip), tune);
        wrist_ease(d, rig, fk, hand_l, &rot0, bump(t, 0.0, 0.12, dur - 0.26, dur - 0.05));
        true
    });
    if posed == Some(true) {
        arm_mask(false);
        if !ask_blend(g, at, at_b, w, true, at, n, tune) {
            return 0;
        }
        hands_to(&h, an);
    }
    RAN | RAN_ALL
}

/// A pistol's magazine change, the support hand's, over the reload clip's pose: to the butt as the magazine drops free, to the belt for a fresh one (a
/// magazine kept goes to the hip first), seated with the heel of the palm, and over the top to run the slide. The trigger hand, the gun and the body
/// stay the clip's.
pub fn pistol_reload_arm(g: usize, an: &mut Anim, t: f64, w: f64, tune: &Knobs) -> u32 {
    let (at, at_b, _) = poses(an);
    let (bt, fix_l) = (an.rl_b, an.hand_fix[0]);
    let Some(Some(n)) = with_pose(g, at, |_d, _rig, n, _fk| if cast(n, &ARM_ROLES) && cast(n, &[MIDDLE1_R, HIPS]) { Some(n) } else { None }) else {
        return 0;
    };
    let Some(mut h) = hands_of(g, an) else {
        return 0;
    };
    if !ask_blend(g, at, at, 1.0, false, at_b, n, tune) {
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
        let (pouch, dump) = (tk(tune.kit.mag.pouch), tk(tune.kit.mag.dump));
        let (o, i, r, dur, kp, fr) = (bt.out, bt.in_, bt.rack, bt.dur, bt.keep, bt.fresh);
        let (well, strip) = (gun.at(&tune.kit.mag.well), gun.at(&tune.kit.mag.strip));
        let (sl, pl) = (gun.at(&tune.kit.mag.slide), gun.at(&tune.kit.mag.pull));
        // (seated: the heel of the palm drives it the last centimetre)
        let seat = mad3(well, gun.up, 0.012);
        let mut p = Path::of(&[(0.0, cur), (max(0.08, o - 0.14), well), (o, well), (o + 0.1, strip)]);
        if kp >= 0.0 {
            p.push((o + 0.04 + kp) * 0.5, tk(tune.kit.mag.via));
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
        let (wf, wb) = (gun.dir(&tune.kit.mag.wf), gun.dir(&tune.kit.mag.wb));
        let (sf, sb) = (gun.dir(&tune.kit.mag.sf), gun.dir(&tune.kit.mag.sb));
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
        two_bone_ik(rig, d, fk.rot, fk.pos, role(UPPER_ARM_L), role(FOREARM_L), hand_l, &p.at(t), Some(&tune.kit.mag.pole), tune.kit.mag.pole_w, Some(&rot), false);
        let grip = max(if kp >= 0.0 { bump(t, o - 0.08, o, kp - 0.02, kp + 0.06) } else { 0.0 }, bump(t, fr, fr + 0.08, i - 0.02, i + 0.06));
        let pinch = if bt.pull { bump(t, r - 0.1, r - 0.02, r + 0.08, r + 0.16) } else { 0.0 };
        let c = |kk: f64| lerp(0.25, kk, max(grip, pinch));
        hand(d, hh, 0, &[c(0.6), c(0.66), c(0.7), c(0.76)], lerp(0.3, 0.65, max(grip, pinch)), 0.1 * (1.0 - grip), tune);
        wrist_ease(d, rig, fk, hand_l, &rot0, bump(t, 0.0, 0.1, dur - 0.22, dur - 0.05));
    });
    arm_mask(false);
    if !ask_blend(g, at, at_b, w, true, at, n, tune) {
        return 0;
    }
    hands_to(&h, an);
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
/// Node i's stored rotation turned on by the share f of a further local rotation, in place
fn q_turn_by(d: &mut [S], i: usize, q: [f64; 4], f: f64) {
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
    let [qx, qy, qz, qw] = own_rot(d, i);
    set_own_rot(d, i, &[qw * x + qx * cw + qy * z - qz * y, qw * y - qx * z + qy * cw + qz * x, qw * z + qx * y - qy * x + qz * cw, qw * cw - qx * x - qy * y - qz * z]);
}
/// Node i's stored rotation turned the share f of the way to the local rotation at t[k‥k+3], in place
fn q_turn_to(d: &mut [S], i: usize, t: &[f64], k: usize, f: f64) {
    let (cx, cy, cz, cw) = (-ch(d, i, 3), -ch(d, i, 4), -ch(d, i, 5), ch(d, i, 6));
    // conj(q) · target
    let q = [
        cw * t[k] + cx * t[k + 3] + cy * t[k + 2] - cz * t[k + 1],
        cw * t[k + 1] - cx * t[k + 2] + cy * t[k + 3] + cz * t[k],
        cw * t[k + 2] + cx * t[k + 1] - cy * t[k] + cz * t[k + 3],
        cw * t[k + 3] - cx * t[k] - cy * t[k + 1] - cz * t[k + 2],
    ];
    q_turn_by(d, i, q, f);
}


/// The hand pass, after every other layer and on the digit joints alone: a hand the library keyed as a fist and no layer has shaped is opened to the
/// man's relaxed hand, by how much of it is the fist, how free the hand is and how little a layer made of it; the thumb of a hand that is not the fist
/// goes to the relaxed thumb; and a drilled man's trigger finger lies straight along the frame until he fires.
pub fn hands_pass(g: usize, an: &mut Anim, dt: f64, full: [f64; 2], gun_r: f64) -> u32 {
    let (at, held, trained) = (an.p_a.at as usize, an.grip, an.trained);
    let Some(tb) = &an.hn_flat else {
        return 0;
    };
    let hn = &mut an.hn;
    let (fist, open, own, thumb, trig) = (&tb[..120], &tb[120..240], &tb[240..250], &tb[250..274], &tb[274..]);
    if digits(0).len() != HAND_DIGITS || digits(1).len() != HAND_DIGITS {
        return 0;
    }
    let base: [[f64; 60]; 2] = core::array::from_fn(|sd| core::array::from_fn(|i| hn.base[sd][i] as f64));
    let far = HAND.far * DEGR;
    with_pose(g, at, |d, _rig, n, _fk| {
        if digits(0).iter().chain(digits(1)).any(|&i| i as usize >= n) {
            return 0;
        }
        for sd in 0..2 {
            hn.grip[sd] = damp(hn.grip[sd], if held[sd] { 1.0 } else { 0.0 }, 14.0, dt);
            let free = (1.0 - clamp(full[sd], 0.0, 1.0)) * (1.0 - hn.grip[sd]);
            if free <= 0.003 {
                continue;
            }
            let (ns, b, fs, op, nt, ow) = (digits(sd), &base[sd], &fist[sd * 60..sd * 60 + 60], &open[sd * 60..sd * 60 + 60], &hn.note[sd], &own[sd * 5..sd * 5 + 5]);
            for dg in 0..5 {
                let kk = dg * 12;
                let (o0, o1, o2) = (ns[dg * 3] as usize, ns[dg * 3 + 1] as usize, ns[dg * 3 + 2] as usize);
                // how much of the hand under the layers is the library's fist (none of it, crouched: that clip's hand is its own)
                let fist_k = 1.0 - clamp((q_ang_of(b, kk, fs, kk) + q_ang_of(b, kk + 4, fs, kk + 4) + q_ang_of(b, kk + 8, fs, kk + 8)) / 3.0 / (ow[dg] * 0.85), 0.0, 1.0);
                if fist_k <= 0.003 && dg > 0 {
                    continue;
                }
                // …less what a layer has made of it
                let moved = (q_ang(d, o0, b, kk) + q_ang(d, o1, b, kk + 4) + q_ang(d, o2, b, kk + 8)) / 3.0;
                let noted = nt[dg] < f32::INFINITY;
                let dd = if noted { nt[dg] as f64 } else { far };
                let lay = 1.0 - if dd < 0.03 { if noted { 1.0 } else { 0.0 } } else { clamp(moved / dd, 0.0, 1.0) };
                let f = fist_k * free * lay;
                if f > 0.003 {
                    q_turn_by(d, o0, [op[kk], op[kk + 1], op[kk + 2], op[kk + 3]], f);
                    q_turn_by(d, o1, [op[kk + 4], op[kk + 5], op[kk + 6], op[kk + 7]], f);
                    q_turn_by(d, o2, [op[kk + 8], op[kk + 9], op[kk + 10], op[kk + 11]], f);
                }
                if dg == 0 {
                    // the thumb of a hand that is not the fist: to the relaxed hand's thumb
                    let (t, th) = ((1.0 - fist_k) * free * lay * HAND.thumb_own, &thumb[sd * 12..sd * 12 + 12]);
                    if t > 0.003 {
                        q_turn_to(d, o0, th, 0, t);
                        q_turn_to(d, o1, th, 4, t);
                        q_turn_to(d, o2, th, 8, t);
                    }
                }
            }
        }
        // the trigger hand's index, a drilled man's: straight along the frame until he fires, on the trigger while he does and for a moment after
        hn.fired += dt;
        let off_t = if hn.fired < HAND.trig_hold { 0.0 } else { 1.0 };
        hn.off = damp(hn.off, off_t, if off_t < hn.off { 40.0 } else { 9.0 }, dt);
        // (eased: the caller's gate for a throw, a reload or a strike is a step)
        hn.gun = damp(hn.gun, clamp(gun_r, 0.0, 1.0), 18.0, dt);
        let tr = hn.gun * smoothstep(HAND.trig_trained, HAND.trig_trained + 0.1, trained) * hn.off;
        if tr > 0.003 {
            let ns = digits(1);
            for j in 0..3 {
                q_turn_to(d, ns[3 + j] as usize, trig, j * 4, tr);
            }
        }
        RAN
    })
    .unwrap_or(0)
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
fn head_hand(g: usize, an: &mut Anim, p: &HeadHand, tune: &Knobs) -> u32 {
    let (at, at_b, _) = poses(an);
    let (tw, hri, fix_l) = (an.nv_tw, an.head_rest_inv, an.hand_fix[0]);
    let Some(Some((n, hd))) = with_pose(g, at, |d, rig, n, fk| {
        if !(cast(n, &ARM_ROLES) && cast(n, &[NECK, HEAD])) {
            return None;
        }
        fk_run(rig, d, fk.rot, fk.pos);
        Some((n, HeadDrawn::of(fk, &tw, &hri)))
    }) else {
        return 0;
    };
    let Some(mut h) = hands_of(g, an) else {
        return 0;
    };
    if !ask_blend(g, at, at, 1.0, false, at_b, n, tune) {
        return 0;
    }
    let hh = &mut h;
    with_pose(g, at_b, |d, rig, _n, fk| {
        turn(d, rig, role(SHOULDER_L), q_roll(p.shrug * DEGR));
        fk_run(rig, d, fk.rot, fk.pos);
        let r = hand_rot(&fix_l, hd.dir(&p.fingers), hd.dir(&p.back));
        two_bone_ik(rig, d, fk.rot, fk.pos, role(UPPER_ARM_L), role(FOREARM_L), role(HAND_L), &add3(hd.at, hd.dir(&p.to)), Some(&p.pole), 0.85, Some(&r), false);
        hand(d, hh, 0, &p.curl, p.thumb, 0.0, tune);
    });
    left_arm_mask(p.e, p.e, p.e);
    if !ask_blend(g, at, at_b, 1.0, true, at, n, tune) {
        return 0;
    }
    hands_to(&h, an);
    RAN | RAN_ALL
}
/// A smoke's arm: the left wrist to a place in front of the mouth.
pub fn smoke_pose(g: usize, an: &mut Anim, tune: &Knobs) -> u32 {
    let x = an.smoke_arm;
    let p = HeadHand {
        e: x * x * (3.0 - 2.0 * x),
        shrug: SMOKEARM.shrug,
        to: SMOKEARM.at,
        pole: SMOKEARM.pole,
        fingers: SMOKEARM.fingers,
        back: SMOKEARM.back,
        curl: [SMOKEARM.curl[0], SMOKEARM.curl[1], SMOKEARM.curl[2], SMOKEARM.curl[3]],
        thumb: SMOKEARM.thumb,
    };
    head_hand(g, an, &p, tune)
}
/// A goggle flip's arm: the left hand at the goggles, wherever between up and down they are.
pub fn nv_pose(g: usize, an: &mut Anim, k: f64, tune: &Knobs) -> u32 {
    let p = HeadHand {
        e: an.nv_arm,
        shrug: tune.kit.nv.hand.shrug,
        to: lerp3(&tune.kit.nv.hand.up, &tune.kit.nv.hand.down, k),
        pole: tune.kit.nv.hand.pole,
        fingers: tune.kit.nv.hand.fingers,
        back: tune.kit.nv.hand.back,
        curl: [tune.kit.nv.hand.curl[0], tune.kit.nv.hand.curl[1], tune.kit.nv.hand.curl[2], tune.kit.nv.hand.curl[3]],
        thumb: tune.kit.nv.hand.thumb,
    };
    head_hand(g, an, &p, tune)
}

/// the support arm laid on shoulder first, then the elbow, then the hand, and off in the same order (the signals' and the tells' envelopes)
fn staggered(t: f64, tin: f64, tout: f64, e: f64) -> (f64, f64, f64) {
    let seg = |lag: f64| bump(t, lag, lag + tin, e - tout + lag * 0.6 - 0.06, e - 0.06 + lag * 0.6);
    (seg(0.0), seg(0.05), seg(0.09))
}
/// A left-hand signal: the arm solved against the body as it stands and laid on shoulder first.
pub fn signal_layer(g: usize, an: &mut Anim, t: f64, kind: SignalKind, dur: f64, tune: &Knobs) -> u32 {
    let (at, at_b, _) = poses(an);
    let (side, fix_l) = (an.sig_side, an.hand_fix[0]);
    let Some(Some(n)) = with_pose(g, at, |d, rig, n, _fk| {
        if !(cast(n, &ARM_ROLES) && cast(n, &[SPINE2, NECK, HEAD])) {
            return None;
        }
        if kind == SignalKind::Go {
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
    let Some(mut h) = hands_of(g, an) else {
        return 0;
    };
    if !ask_blend(g, at, at, 1.0, false, at_b, n, tune) {
        return 0;
    }
    let hh = &mut h;
    with_pose(g, at_b, |d, rig, _n, fk| {
        fk_run(rig, d, fk.rot, fk.pos);
        let sh = fk.at(role(UPPER_ARM_L));
        let (to, r, pole, clav, curl, thumb, spread): ([f64; 3], [f64; 4], [f64; 3], f64, [f64; 4], f64, f64);
        if kind == SignalKind::Hold {
            // the fist snaps up beside the head, palm forward
            let settle = 0.006 * sin(t * 5.5) * bump(t, 0.3, 0.4, 0.6, 0.7);
            to = [sh[0] + 0.14, sh[1] + 0.19 + settle, sh[2] + 0.16];
            r = hand_rot(&fix_l, [0.12, 1.0, 0.12], [0.0, 0.05, -1.0]);
            (pole, clav, curl, thumb, spread) = ([1.0, -0.4, 0.35], 8.0, [1.0, 1.0, 1.0, 1.0], 1.0, 0.0);
        } else if kind == SignalKind::Go {
            // the knife hand comes up by the ear, chops over and down to point the way, holds, returns
            let hx = curve(t, &[[0.0, 0.1], [0.28, 0.1], [0.46, 0.04], [0.95, 0.04]]);
            let hy = curve(t, &[[0.0, 0.2], [0.28, 0.22], [0.36, 0.2], [0.46, -0.02], [0.54, 0.015], [0.95, 0.01]]);
            let hz = curve(t, &[[0.0, 0.1], [0.28, 0.08], [0.38, 0.3], [0.46, 0.43], [0.95, 0.42]]);
            to = [sh[0] + hx, sh[1] + hy, sh[2] + hz];
            let chop = smoothstep(0.28, 0.46, t);
            r = hand_rot(&fix_l, lerp3(&[0.1, 1.0, 0.25], &[0.05, -0.18, 1.0], chop), lerp3(&[1.0, 0.0, -0.2], &[0.6, 0.8, 0.1], chop));
            (pole, clav, curl, thumb, spread) = ([1.0, -0.35, 0.1], 7.0 * (1.0 - 0.6 * chop), [0.05, 0.08, 0.12, 0.18], 0.3, 0.0);
        } else if kind == SignalKind::Squeeze {
            // the stack's "ready": the hand onto the shoulder of the man ahead, two squeezes, and back
            let sq = bump(t, 0.3, 0.36, 0.4, 0.47) + bump(t, 0.52, 0.58, 0.62, 0.69);
            to = [sh[0] - 0.1, sh[1] + 0.04 - 0.012 * sq, sh[2] + 0.5 + 0.008 * sq];
            r = hand_rot(&fix_l, [-0.15, -0.3, 1.0], [0.1, 1.0, 0.3]);
            (pole, clav, curl, thumb, spread) = ([0.8, -0.6, -0.1], 3.0, [0.3 + 0.4 * sq, 0.34 + 0.4 * sq, 0.38 + 0.4 * sq, 0.42 + 0.4 * sq], 0.15 + 0.5 * sq, 0.2);
        } else if kind == SignalKind::Aside {
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
        hand(d, hh, 0, &curl, thumb, spread, tune);
    });
    let (w_up, w_fore, w_hand) = staggered(t, 0.2, 0.26, dur);
    left_arm_mask(w_up, w_fore, w_hand);
    if !ask_blend(g, at, at_b, 1.0, true, at, n, tune) {
        return 0;
    }
    hands_to(&h, an);
    RAN | RAN_ALL
}
/// An idle tell: a look at the watch (the forearm level across the chest, the eyes down onto it), or a hand round to the back of the neck with the chest
/// lifted into the stretch.
pub fn tell_layer(g: usize, an: &mut Anim, t: f64, watch: bool, dur: f64, tune: &Knobs) -> u32 {
    let (at, at_b, _) = poses(an);
    let (gaze_local, fix_l) = (an.gaze_local, an.hand_fix[0]);
    let (tin, tout) = if watch { (0.2, 0.28) } else { (0.26, 0.34) };
    // the body's share (the head, the chest): one window over the whole tell
    let env = bump(t, 0.0, tin, dur - tout, dur);
    let Some(Some(n)) = with_pose(g, at, |_d, _rig, n, _fk| if cast(n, &ARM_ROLES) && cast(n, &[SPINE2, NECK, HEAD]) { Some(n) } else { None }) else {
        return 0;
    };
    if env <= 1e-4 {
        an.tell_shown = 0.0;
        return RAN;
    }
    let Some(mut h) = hands_of(g, an) else {
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
    if !ask_blend(g, at, at, 1.0, false, at_b, n, tune) {
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
            hand(d, hh, 0, &[0.35, 0.4, 0.45, 0.5], 0.4, 0.0, tune);
        } else {
            // the scapula comes up with the raised elbow; the hand works up and down the back of the neck
            turn(d, rig, role(SHOULDER_L), q_roll(12.0 * DEGR * env));
            fk_run(rig, d, fk.rot, fk.pos);
            let nk = fk.at(role(NECK));
            let rub = 0.018 * sin((t - tin) * 2.0 * PI * 2.5) * bump(t, tin, tin + 0.15, dur - tout - 0.1, dur - tout);
            let to = [nk[0] + 0.085, nk[1] + 0.035 + rub, nk[2] - 0.075];
            let r = hand_rot(&fix_l, [-0.55, 0.35, -0.75], [0.45, 0.55, 0.55]);
            two_bone_ik(rig, d, fk.rot, fk.pos, role(UPPER_ARM_L), role(FOREARM_L), role(HAND_L), &to, Some(&[0.75, 0.35, 0.8]), 0.9, Some(&r), false);
            hand(d, hh, 0, &[0.5, 0.55, 0.6, 0.65], 0.5, 0.0, tune);
        }
    });
    let (w_up, w_fore, w_hand) = staggered(t, tin, tout, dur);
    left_arm_mask(w_up, w_fore, w_hand);
    if !ask_blend(g, at, at_b, 1.0, true, at, n, tune) {
        return 0;
    }
    hands_to(&h, an);
    an.tell_shown = w_up;
    RAN | RAN_ALL
}

/// The flashlight brought up in the gun hand: the right wrist to a spot ahead of the shoulder, the elbow down and a little out, by e. Solved on the
/// scratch pose at `at_b`, laid on the right arm.
pub fn torch_up(g: usize, an: &Anim, e: f64, tune: &Knobs) -> u32 {
    let (at, at_b, _) = poses(an);
    let fix_r = an.hand_fix[1];
    let Some(Some(n)) = with_pose(g, at, |_d, _rig, n, _fk| if cast(n, &ARM_ROLES) { Some(n) } else { None }) else {
        return 0;
    };
    if !ask_blend(g, at, at, 1.0, false, at_b, n, tune) {
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
    if !ask_blend(g, at, at_b, smoothstep(0.0, 0.25, e), true, at, n, tune) {
        return 0;
    }
    RAN | RAN_ALL
}
/// A hand to the radio: the left hand up to the handset at his left collar, the head turned and tipped to it as far as his gun is down.
pub fn radio_pose(g: usize, an: &mut Anim, w: f64, hw: f64, tune: &Knobs) -> u32 {
    let (at, at_b, _) = poses(an);
    let fix_l = an.hand_fix[0];
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
    let Some(mut h) = hands_of(g, an) else {
        return 0;
    };
    if !ask_blend(g, at, at, 1.0, false, at_b, n, tune) {
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
        hand(d, hh, 0, &[0.4, 0.5, 0.6, 0.65], 0.2, 0.0, tune);
    });
    ask_mask(&[(role(SHOULDER_L), 1.0, true)]);
    if !ask_blend(g, at, at_b, smoothstep(0.0, 0.25, w), true, at, n, tune) {
        return 0;
    }
    hands_to(&h, an);
    RAN | RAN_ALL
}

/// At a lock: the crouch's rounded spine sat up toward it, the shoulders up, the head onto it, and both hands at the keyway (the pick raking, the wrench
/// hand under it giving as a pin sets).
pub fn lockpick_layer(g: usize, an: &mut Anim, w: f64, tune: &Knobs) -> u32 {
    let (at, at_b, _) = poses(an);
    let (t, lock, gaze_local, fix_l, fix_r) = (an.lp_t, an.lockpick_at, an.gaze_local, an.hand_fix[0], an.hand_fix[1]);
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
    let Some(mut h) = hands_of(g, an) else {
        return 0;
    };
    if !ask_blend(g, at, at, 1.0, false, at_b, n, tune) {
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
        hand(d, hh, 1, &[0.55, 0.7, 0.9, 0.95], 0.75, 0.0, tune);
        hand(d, hh, 0, &[0.75, 0.8, 0.85, 0.85], 0.6, 0.0, tune);
    });
    arms_mask();
    if !ask_blend(g, at, at_b, w, true, at, n, tune) {
        return 0;
    }
    hands_to(&h, an);
    RAN | RAN_ALL
}
/// Both hands on a body: the trunk bowed toward the grip and tipped off a load, the eyes a stride beyond the hands, and each wrist to its spot through
/// the inverse of the chest twist still to come.
pub fn haul_layer(g: usize, an: &mut Anim, h_: &HaulPose, w: f64, l_at: [f64; 3], r_at: [f64; 3], tw_y: f64, tw_p: f64, tune: &Knobs) -> u32 {
    let (at, at_b, _) = poses(an);
    let (rw, look, grip) = (clamp(h_.rw.unwrap_or(1.0), 0.0, 1.0), h_.look.unwrap_or(0.7), h_.grip);
    let (bow, tilt) = (h_.bow * w, h_.tilt.unwrap_or(0.0) * w);
    let (gaze_local, fix_l, fix_r) = (an.gaze_local, an.hand_fix[0], an.hand_fix[1]);
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
        let (tl, tr) = (untwist(fk, tw_y, tw_p, l_at), untwist(fk, tw_y, tw_p, r_at));
        if look > 0.01 {
            let mut to = lerp3(&tl, &tr, 0.5 * rw);
            to[2] += 0.45;
            look_at(d, rig, fk, &gaze_local, to, w * look, 0.0);
        }
        Some((n, tl, tr))
    }) else {
        return 0;
    };
    let Some(mut h) = hands_of(g, an) else {
        return 0;
    };
    if !ask_blend(g, at, at, 1.0, false, at_b, n, tune) {
        return 0;
    }
    let hh = &mut h;
    with_pose(g, at_b, |d, rig, _n, fk| {
        fk_run(rig, d, fk.rot, fk.pos);
        let c = clamp(grip, 0.0, 1.0);
        two_bone_ik(rig, d, fk.rot, fk.pos, role(UPPER_ARM_R), role(FOREARM_R), role(HAND_R), &tr, Some(&[-0.7, -0.25, -0.65]), 0.75, Some(&hand_rot(&fix_r, [0.12, -0.55, 0.83], [-0.45, 1.0, 0.5])), false);
        two_bone_ik(rig, d, fk.rot, fk.pos, role(UPPER_ARM_L), role(FOREARM_L), role(HAND_L), &tl, Some(&[0.7, -0.25, -0.65]), 0.75, Some(&hand_rot(&fix_l, [-0.12, -0.55, 0.83], [0.45, 1.0, 0.5])), true);
        hand(d, hh, 1, &[c * 0.85, c * 0.9, c * 0.95, c], c * 0.7, 0.0, tune);
        hand(d, hh, 0, &[c * 0.85, c * 0.9, c * 0.95, c], c * 0.7, 0.0, tune);
    });
    if rw >= 0.999 {
        arms_mask();
        if !ask_blend(g, at, at_b, w, true, at, n, tune) {
            return 0;
        }
    } else {
        ask_mask(&[(role(SHOULDER_L), 1.0, true)]);
        if !ask_blend(g, at, at_b, w, true, at, n, tune) {
            return 0;
        }
        if rw > 0.003 {
            ask_mask(&[(role(SHOULDER_R), 1.0, true)]);
            if !ask_blend(g, at, at_b, w * rw, true, at, n, tune) {
                return 0;
            }
        }
    }
    hands_to(&h, an);
    RAN | RAN_ALL
}

/// The gun hand's errand between the flashlight's ring at the front of the hip and the sidearm on the thigh: the wrist from wherever the pose has it
/// down onto the one, across to the other, and back. The pose as it stood is kept at `at_sw` (what was in the hand is drawn off it).
pub fn swap_pose(g: usize, an: &mut Anim, at_sw: usize, e: f64, across: f64, to_weapon: bool, tw_y: f64, tw_p: f64, tune: &Knobs) -> u32 {
    let (at, at_b, _) = poses(an);
    let fix_r = an.hand_fix[1];
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
    let Some(mut h) = hands_of(g, an) else {
        return 0;
    };
    if !ask_blend(g, at, at, 1.0, false, at_b, n, tune) {
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
        hand(d, hh, 1, &[0.72, 0.78, 0.82, 0.85], 0.5, 0.0, tune);
    });
    if !ask_blend(g, at, at, 1.0, false, at_sw, n, tune) {
        return 0;
    }
    ask_mask(&[(role(SHOULDER_R), 1.0, true)]);
    if !ask_blend(g, at, at_b, smoothstep(0.0, 0.25, e), true, at, n, tune) {
        return 0;
    }
    hands_to(&h, an);
    RAN | RAN_ALL
}

/// The crouch's own arms: a free man low has his hands down ahead of his knees, never a held frame (the weight goes foot to foot at a stand, the hands
/// drift), swinging with the crouch walk, the left leading its foot in the close-in mode, the elbows flaring as he sinks and the arms rising after him.
pub fn stealth_layer(g: usize, an: &mut Anim, dt: f64, busy: bool, crouch: f64, w_move: f64, tw_y: f64, tw_p: f64, tune: &Knobs) -> u32 {
    let (at, at_b, _) = poses(an);
    let (want, cs, ready_w, wide_s, hands_down) = (an.stealth.as_ref().map_or(false, |f| f.free) && !busy, an.creep_shown, an.ready_w, an.wide_s, an.hands_down);
    let (wm, fixes) = (clamp(w_move, 0.0, 1.0), [an.hand_fix[0], an.hand_fix[1]]);
    let Some(Some(n)) = with_pose(g, at, |_d, _rig, n, _fk| if cast(n, &ARM_ROLES) && cast(n, &LEG_ROLES) && cast(n, &[SPINE1, SPINE2]) { Some(n) } else { None }) else {
        return 0;
    };
    let Some(mut st) = an.sp else {
        return 0;
    };
    let (p_rise, p_idle, p_walk, p_creep) = (true, true, true, true);
    st.w_a = clamp(st.w_a + if want { dt / STEALTH.in_secs } else { -dt / STEALTH.out_secs }, 0.0, 1.0);
    let w_a = st.w_a * st.w_a * (3.0 - 2.0 * st.w_a);
    // the arms' own crouch: down with him at once (the elbows flaring for as long as he is still sinking), up after him
    let dc = crouch - st.cr;
    if !p_rise || dc >= 0.0 {
        st.fl += ((if p_rise && dt > 0.0 { clamp(dc / dt * 0.35, 0.0, 1.0) } else { 0.0 }) - st.fl) * min(1.0, dt * 12.0);
        st.cr = if p_rise { min(crouch, st.cr + dt * STEALTH.rise.down) } else { crouch };
    } else {
        st.cr = max(crouch, st.cr - dt / max(0.05, tune.stealthpose.rise_secs));
        st.fl += (0.0 - st.fl) * min(1.0, dt * 12.0);
    }
    st.t += dt;
    let up = st.cr * st.cr * (3.0 - 2.0 * st.cr);
    let mix = lerp(if p_idle { 1.0 } else { 0.0 }, if p_walk { 1.0 } else { 0.0 }, wm);
    let w = w_a * up * max(mix, if p_creep { cs } else { 0.0 });
    if st.w_a <= 0.003 || up <= 0.003 {
        st.lead = 0.0;
        an.sp = Some(st);
        return RAN;
    }
    let t = st.t;
    let code = with_pose(g, at, |d, rig, _n, fk| {
        fk.part(rig, d, list(FK_LEGS));
        // at a stand: never a held frame, the weight goes foot to foot with the feet where they were
        let sway = if p_idle { w_a * up * (1.0 - wm) * (1.0 - cs) * crouch } else { 0.0 };
        if !(sway > 0.003) {
            return RAN | ran_list(FK_LEGS);
        }
        let (foot_l, foot_r) = (role(FOOT_L), role(FOOT_R));
        let (fl, fr, rl, rr) = (fk.at(foot_l), fk.at(foot_r), fk.rot(foot_l), fk.rot(foot_r));
        let ph = t * 2.0 * PI / max(0.5, tune.stealthpose.sway_secs);
        shift(d, rig, role(HIPS), &[tune.stealthpose.sway_m * sway * sin(ph), 0.0, tune.stealthpose.sway_m * 0.35 * sway * sin(ph * 2.0 + 0.6)]);
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
        st.lead = 0.0;
        an.sp = Some(st);
        return code;
    }
    let Some(mut h) = hands_of(g, an) else {
        return 0;
    };
    if !ask_blend(g, at, at, 1.0, false, at_b, n, tune) {
        return 0;
    }
    let hh = &mut h;
    let mut arm_w = [0.0f64; 2];
    let aw = &mut arm_w;
    with_pose(g, at_b, |d, rig, _n, fk| {
        let rnd = STEALTH.round * DEGR * w;
        turn(d, rig, role(SHOULDER_L), q_yaw(-rnd));
        turn(d, rig, role(SHOULDER_R), q_yaw(rnd));
        fk_run(rig, d, fk.rot, fk.pos);
        let (kl, kr) = (fk.at(role(SHIN_L)), fk.at(role(SHIN_R)));
        let (k_hi, k_z) = (max(kl[1], kr[1]), max(kl[2], kr[2]));
        let (fz_l, fz_r) = (fk.at(role(FOOT_L))[2], fk.at(role(FOOT_R))[2]);
        // the walk: + = the right foot ahead, so the LEFT hand comes forward
        st.sw += ((if p_walk { clamp((fz_r - fz_l) / STEALTH.walk.stride, -1.0, 1.0) * wm } else { 0.0 }) - st.sw) * min(1.0, dt * STEALTH.walk.rate);
        let s = st.sw;
        // the close-in mode: the free hand leads its own foot
        let vz = if dt > 0.0 { (fz_l - st.fz) / dt } else { 0.0 };
        st.fz = fz_l;
        let tgt = if p_creep && cs > 0.003 { if vz > 0.15 { 1.0 } else { clamp((fz_l + 0.25) / 0.55, 0.0, 1.0) } } else { 0.0 };
        st.lead += (tgt - st.lead) * min(1.0, dt * STEALTH.creep.rate);
        let (lt, dr) = (t * 2.0 * PI / max(0.5, tune.stealthpose.sway_secs), tune.stealthpose.drift_m * (1.0 - wm));
        let (wf, wb) = (STEALTH.walk.fwd, STEALTH.walk.back);
        let (pole, fing, back) = (STEALTH.idle.pole, STEALTH.idle.fing, STEALTH.idle.back);
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
            let mut to = untwist(fk, tw_y, tw_p, [kn[0] + sg * STEALTH.idle.out + d1, max(STEALTH.idle.floor, k_hi - STEALTH.idle.below) + d2, max(kn[2] + STEALTH.idle.fwd, k_z + STEALTH.idle.lead) + d3]);
            if a_ > 0.0 {
                to = lerp3(&to, &untwist(fk, tw_y, tw_p, [kn[0] * 0.6 + sg * wf[0], max(STEALTH.idle.floor, k_hi + wf[1]), k_z + wf[2]]), a_);
            } else if a_ < 0.0 {
                to = lerp3(&to, &[s0[0] + sg * wb[0], s0[1] + wb[1], s0[2] + wb[2]], -a_);
            }
            if side == 0 && cs > 0.003 && p_creep {
                let behind = [s0[0] + wb[0], s0[1] + wb[1] + 0.06, s0[2] + wb[2] * STEALTH.creep.swing];
                let ahead = untwist(fk, tw_y, tw_p, [kl[0] * 0.5 + STEALTH.creep.at[0], max(STEALTH.creep.floor, k_hi + STEALTH.creep.at[1]), k_z + STEALTH.creep.at[2]]);
                to = lerp3(&to, &lerp3(&behind, &ahead, st.lead), cs);
            }
            // (beyond the arm: reached toward, never locked out)
            let (dx, dy, dz) = (to[0] - s0[0], to[1] - s0[1], to[2] - s0[2]);
            let dd = hypot3(dx, dy, dz);
            if dd > len {
                to = [s0[0] + dx * len / dd, s0[1] + dy * len / dd, s0[2] + dz * len / dd];
            }
            // (the back arm's elbow goes behind him, not out)
            let (flare, bk) = (st.fl * STEALTH.walk.flare_pole, max(0.0, -a_) * STEALTH.walk.tuck);
            let r = hand_rot(&fixes[side], [sg * fing[0], fing[1], fing[2]], [sg * back[0], back[1], back[2]]);
            two_bone_ik(rig, d, fk.rot, fk.pos, up_n, fo_n, ha_n, &to, Some(&[sg * (pole[0] + flare - bk), pole[1] + flare * 0.5 + bk * 0.4, pole[2] - bk]), 0.8, Some(&r), false);
            let c = STEALTH.idle.curl;
            hand(d, hh, side, &[c, c + 0.04, c + 0.08, c + 0.12], STEALTH.idle.thumb, STEALTH.idle.spread, tune);
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
    if !ask_blend(g, at, at_b, 1.0, true, at, n, tune) {
        return 0;
    }
    hands_to(&h, an);
    an.sp = Some(st);
    RAN | RAN_ALL
}

/// A goggle nod shown as a lift of the head, as big less a little whichever way the nod went, and smooth through nothing
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
/// The look under a door: on the pose at `at` (a copy of the finished pose, which is then MADE the pose) he shuffles in and sinks with his feet where
/// they were, the trunk folds, the head bows at the foot of the leaf, a goggle nod under way is posed on it, and the hands go low: the right wrist at
/// the gap, the left by its knee.
pub fn cable_pose(g: usize, an: &mut Anim, pic: usize, e: f64, eh: f64, tune: &Knobs) -> u32 {
    let (at, at_b, lp, lock_z, nod, gaze_local) = (pic, an.p_b.at as usize, clamp(an.lp_w.x, 0.0, 1.0), an.lockpick_at[2], an.nv_nod, an.gaze_local);
    let (tw, hri, fix_l, fix_r, nv_a) = (an.nv_tw, an.head_rest_inv, an.hand_fix[0], an.hand_fix[1], &mut an.nv_a);
    let Some(nod_up) = an.sp.as_mut().map(|s| &mut s.nod_up) else {
        return 0;
    };
    let Some(Some(n)) = with_pose(g, at, |d, rig, n, fk| {
        if !(cast(n, &ARM_ROLES) && cast(n, &LEG_ROLES) && cast(n, &[SPINE1, SPINE2, NECK, HEAD])) {
            return None;
        }
        let (hips, foot_l, foot_r, hd) = (role(HIPS), role(FOOT_L), role(FOOT_R), role(HEAD));
        // the shuffle in, all of him, then the sink over feet put back where they were
        shift(d, rig, hips, &[0.0, 0.0, tune.stealthpose.cable_step * e]);
        fk.part(rig, d, list(FK_LEGS));
        let (fl, fr, rl, rr) = (fk.at(foot_l), fk.at(foot_r), fk.rot(foot_l), fk.rot(foot_r));
        shift(d, rig, hips, &[0.0, -STEALTH.cable.sink * e, 0.0]);
        fk.part(rig, d, list(FK_LEGS));
        two_bone_ik(rig, d, fk.rot, fk.pos, role(THIGH_L), role(SHIN_L), foot_l, &fl, Some(&knee_pole(&rl)), 0.0, Some(&rl), false);
        two_bone_ik(rig, d, fk.rot, fk.pos, role(THIGH_R), role(SHIN_R), foot_r, &fr, Some(&knee_pole(&rr)), 0.0, Some(&rr), true);
        let bow = (STEALTH.cable.undo * lp + tune.stealthpose.cable_bow) * DEGR * e;
        turn(d, rig, role(SPINE1), q_pitch(bow * 0.36));
        turn(d, rig, role(SPINE2), q_pitch(bow * 0.39));
        turn(d, rig, role(SPINE3), q_pitch(bow * 0.25));
        // (on the way out the head is level again before the back is)
        turn(d, rig, role(NECK), q_pitch(-bow * 0.25 * (1.0 - eh)));
        turn(d, rig, hd, q_pitch(-bow * 0.3 * (1.0 - eh)));
        turn(d, rig, role(SHOULDER_L), q_yaw(-STEALTH.cable.round * DEGR * e));
        turn(d, rig, role(SHOULDER_R), q_yaw(STEALTH.cable.round * DEGR * e));
        fk_run(rig, d, fk.rot, fk.pos);
        let gap = [0.0, 0.04, max(lock_z + 0.05, fk.at(hd)[2] + STEALTH.cable.ahead)];
        look_at(d, rig, fk, &gaze_local, gap, eh, 0.0);
        if true && nod != 0.0 {
            // a nod that starts with the look's head mostly in is a lift of the head off the gap and back; one the look came in over stays the
            // chin-down nod it began as, kept short of straight down. Which is settled once a nod.
            if nod_up.is_none() {
                *nod_up = Some(eh > 0.3);
            }
            let lift = *nod_up == Some(true);
            let mut tip = if lift { nod_lift(nod) } else { nod };
            fk_run(rig, d, fk.rot, fk.pos);
            let head = q_mul(&twist_of(tw[2], tw[3]), &q_mul(&twist_of(tw[0], tw[1]), &fk.rot(hd)));
            if !lift && nod > 0.0 {
                tip = min(nod, max(0.0, STEALTH.cable.nod_max * DEGR - asin(clamp(-q_rotate(&q_mul(&head, &hri), &[0.0, 0.0, 1.0])[1], -1.0, 1.0))));
            }
            nv_tip(d, &head, &hri, tip);
            *nv_a = tip;
        }
        Some(n)
    }) else {
        return 0;
    };
    let Some(mut h) = hands_of(g, an) else {
        return 0;
    };
    if !ask_blend(g, at, at, 1.0, false, at_b, n, tune) {
        return 0;
    }
    let hh = &mut h;
    with_pose(g, at_b, |d, rig, _n, fk| {
        fk_run(rig, d, fk.rot, fk.pos);
        let kl = fk.at(role(SHIN_L));
        let (rr, rl) = (hand_rot(&fix_r, [0.05, -0.35, 0.94], [-0.3, 0.9, 0.3]), hand_rot(&fix_l, [-0.15, -0.85, 0.5], [0.45, 0.25, 0.85]));
        two_bone_ik(rig, d, fk.rot, fk.pos, role(UPPER_ARM_R), role(FOREARM_R), role(HAND_R), &[STEALTH.cable.gap[0], STEALTH.cable.gap[1], lock_z - STEALTH.cable.short], Some(&[-0.6, 0.1, -0.75]), 0.8, Some(&rr), false);
        two_bone_ik(rig, d, fk.rot, fk.pos, role(UPPER_ARM_L), role(FOREARM_L), role(HAND_L), &[kl[0] + STEALTH.cable.knee[0], kl[1] + STEALTH.cable.knee[1], kl[2] + STEALTH.cable.knee[2]], Some(&[0.8, 0.0, -0.6]), 0.8, Some(&rl), false);
        hand(d, hh, 1, &[0.5, 0.6, 0.7, 0.75], 0.5, 0.0, tune);
        hand(d, hh, 0, &[0.2, 0.24, 0.28, 0.32], 0.15, 0.3, tune);
    });
    arms_mask();
    if !ask_blend(g, at, at_b, e, true, at, n, tune) {
        return 0;
    }
    hands_to(&h, an);
    RAN | RAN_ALL
}

/// The man a takedown is done to, still on his feet: head, trunk and (hauled back) his arms, pitch only.
pub fn taken_layer(g: usize, an: &Anim, style: TakenStyle, off: f64, u: f64) -> u32 {
    with_pose(g, an.p_a.at as usize, |d, rig, n, _fk| {
        if !(cast(n, &ARM_ROLES) && cast(n, &[SPINE1, SPINE2, NECK, HEAD])) {
            return 0;
        }
        let out = (1.0 - smoothstep(0.6, TAKEN.secs, u)) * if off < 0.0 { 1.0 } else { 1.0 - smoothstep(0.0, 0.2, off) };
        if style == TakenStyle::Arch {
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
        } else if style == TakenStyle::Hunch {
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
/// A takedown's own move, the legs: by contact the rear foot is drawn in under him and the pelvis comes up and over the feet until the shorter leg is
/// near straight; let go after the move's end.
pub fn blow_legs(g: usize, an: &Anim, b: &BlowRun) -> u32 {
    let (t, hit, end, tail) = (b.t, b.hit, b.end, b.tail);
    let u = smoothstep(0.0, 0.85 * hit, t);
    let w = u * (1.0 - smoothstep(hit + 0.12, end + if tail > 0.0 { 0.05 } else { 0.0 }, t));
    with_pose(g, an.p_a.at as usize, |d, rig, n, fk| {
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
        let dz = front[2] - back[2] - BLOW.legs.gap;
        if dz > 0.0 {
            // (…and its heel comes down as it lands)
            back[2] += dz * w;
            back[1] += (front[1] - back[1]) * w * 0.8 + BLOW.legs.lift * 4.0 * u * (1.0 - u) * min(1.0, dz / 0.25);
        }
        if left_back {
            f_l = back;
        } else {
            f_r = back;
        }
        let hp = fk.at(role(HIPS));
        let hz = ((f_l[2] + f_r[2]) / 2.0 - hp[2]) * BLOW.legs.fwd * w;
        let mut up = BLOW.legs.up;
        for (th, sh, ft, f) in [(role(THIGH_L), role(SHIN_L), role(FOOT_L), f_l), (role(THIGH_R), role(SHIN_R), role(FOOT_R), f_r)] {
            let (a, b, c) = (fk.at(th), fk.at(sh), fk.at(ft));
            let len = (dist3(&a, &b) + dist3(&b, &c)) * BLOW.legs.straight;
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
/// A takedown's own move, the trunk's share (every one is delivered by the body, the arm its last link).
pub fn blow_trunk(g: usize, an: &Anim, b: &BlowRun, tl: f64) -> u32 {
    let (t, h, e, style) = (b.t, b.hit, b.end, b.style);
    let out = 1.0 - smoothstep(e - 0.2, e, t);
    let pt;
    let mut yw = 0.0;
    if style == BlowStyle::Chop {
        pt = -10.0 * bump(t, 0.05, 0.5 * h, 0.75 * h, h) + 40.0 * smoothstep(0.75 * h, h + 0.14, t) * max(out, 0.55 * tl);
        yw = -16.0 * bump(t, 0.0, 0.5 * h, 0.72 * h, h) + 18.0 * smoothstep(0.72 * h, h + 0.04, t) * out;
    } else if style == BlowStyle::Palm {
        pt = 14.0 * bump(t, 0.0, 0.45 * h, 0.7 * h, h) - 14.0 * bump(t, 0.8 * h, h + 0.08, e - 0.14, e) + 14.0 * tl;
        yw = -14.0 * bump(t, 0.0, 0.45 * h, 0.7 * h, h) + 24.0 * smoothstep(0.7 * h, h + 0.06, t) * out;
    } else if style == BlowStyle::Mouth {
        pt = 8.0 * bump(t, 0.0, 0.12, h - 0.06, h) - 8.0 * bump(t, h - 0.04, h + 0.08, h + 0.2, e) + 34.0 * tl;
    } else if style == BlowStyle::Slash {
        pt = -8.0 * smoothstep(h, h + 0.1, t) * out + 12.0 * tl;
        yw = 14.0 * bump(t, 0.0, 0.5 * h, 0.7 * h, h) - 26.0 * smoothstep(0.7 * h, h + 0.08, t) * out;
    } else {
        pt = 5.0 * bump(t, 0.0, 0.5 * h, h, h + 0.12) + 12.0 * tl;
    }
    with_pose(g, an.p_a.at as usize, |d, rig, n, _fk| {
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
/// A takedown's own move, the arms, by IK on a copy of the pose (the knife styles' right arm is the knife's own): the chop's fist cocked behind the ear
/// and hammered down, the palm heel driven up and through with the arm locked out; the left flat on his back (chop), over his mouth (the knife from
/// behind) or thrown back and down (the rest). The eyes stay on the mark.
pub fn blow_arms(g: usize, an: &mut Anim, b: &BlowRun, at1: [f64; 3], at2: Option<[f64; 3]>, tw_y: f64, tw_p: f64, shown: &mut f64, tune: &Knobs) -> u32 {
    let (at, at_b, _) = poses(an);
    let (t, h, e, tail, style, free, speed) = (b.t, b.hit, b.end, b.tail, b.style, 1.0 - clamp(an.upper_w.aim, 0.0, 1.0), abs(an.speed));
    let (gaze_local, fix_l, fix_r) = (an.gaze_local, an.hand_fix[0], an.hand_fix[1]);
    let (chop, palm, mouth) = (style == BlowStyle::Chop, style == BlowStyle::Palm, style == BlowStyle::Mouth);
    let x = e + tail;
    let w_all = smoothstep(0.0, 0.5 * h, t) * (1.0 - smoothstep(e - 0.16, e, t)) * free;
    let Some(Some(n)) = with_pose(g, at, |_d, _rig, n, _fk| if cast(n, &ARM_ROLES) && cast(n, &[SPINE1, SPINE2, NECK, HEAD]) { Some(n) } else { None }) else {
        return 0;
    };
    if w_all <= 0.0 && !(mouth && t < x) {
        *shown = 0.0;
        return RAN;
    }
    let Some(mut hs) = hands_of(g, an) else {
        return 0;
    };
    if !ask_blend(g, at, at, 1.0, false, at_b, n, tune) {
        return 0;
    }
    let hh = &mut hs;
    let Some((tgt, right, w_l)) = with_pose(g, at_b, |d, rig, _n, fk| {
        fk_run(rig, d, fk.rot, fk.pos);
        let tgt = untwist(fk, tw_y, tw_p, at1);
        let t2 = at2.map(|p| untwist(fk, tw_y, tw_p, p));
        let (sl, sr) = (fk.at(role(UPPER_ARM_L)), fk.at(role(UPPER_ARM_R)));
        let (er, el) = (fk.at(role(FOREARM_R)), fk.at(role(FOREARM_L)));
        let arm_r = dist3(&sr, &er) + dist3(&er, &fk.at(role(HAND_R)));
        let arm_l = dist3(&sl, &el) + dist3(&el, &fk.at(role(HAND_L)));
        let cap = |s: [f64; 3], p: [f64; 3], arm: f64| -> [f64; 3] {
            let o = sub3(p, s);
            let (l, c) = (len3(&o), BLOW.reach * arm);
            if l > c {
                mad3(s, o, c / l)
            } else {
                p
            }
        };
        // the arm thrown out straight along a line
        let lock = |s: [f64; 3], dv: [f64; 3], arm: f64| -> [f64; 3] { mad3(s, normalize(dv), BLOW.reach * arm) };
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
                hand(d, hh, 1, &[BLOW.fist[0], BLOW.fist[1], BLOW.fist[2], BLOW.fist[3]], 0.6, 0.0, tune);
            } else {
                hand(d, hh, 1, &[BLOW.open[0], BLOW.open[1], BLOW.open[2], BLOW.open[3]], 0.2, 0.0, tune);
            }
        }
        let mut w_l = 0.0;
        if let (true, Some(t2)) = (chop, t2) {
            w_l = bump(t, 0.04, 0.5 * h, h - 0.02, h + 0.15) * free;
            fk_run(rig, d, fk.rot, fk.pos);
            let rot = hand_rot(&fix_l, [0.0, 1.0, 0.2], [0.0, 0.2, -1.0]);
            two_bone_ik(rig, d, fk.rot, fk.pos, role(UPPER_ARM_L), role(FOREARM_L), role(HAND_L), &cap(sl, t2, arm_l), Some(&[0.6, -0.7, -0.2]), 0.8, Some(&rot), false);
            hand(d, hh, 0, &[BLOW.open[0], BLOW.open[1], BLOW.open[2], BLOW.open[3]], 0.2, 0.0, tune);
        } else if let (true, Some(t2)) = (mouth, t2) {
            let (c, dn) = if tail > 0.0 { (x - 0.5, x - 0.2) } else { (e - 0.18, e - 0.04) };
            w_l = bump(t, 0.03, 0.14, c, dn) * free * if tail > 0.0 && t > e { 1.0 - smoothstep(BLOW.walk[0], BLOW.walk[1], speed) } else { 1.0 };
            fk_run(rig, d, fk.rot, fk.pos);
            let rot = hand_rot(&fix_l, [-1.0, 0.0, 0.1], [0.1, 0.0, 1.0]);
            two_bone_ik(rig, d, fk.rot, fk.pos, role(UPPER_ARM_L), role(FOREARM_L), role(HAND_L), &cap(sl, t2, arm_l), Some(&[0.8, -0.5, 0.0]), 0.8, Some(&rot), false);
            hand(d, hh, 0, &[BLOW.cup[0], BLOW.cup[1], BLOW.cup[2], BLOW.cup[3]], 0.3, 0.0, tune);
        } else if !chop && !mouth {
            w_l = bump(t, 0.4 * h, h, e - 0.2, e) * free;
            fk_run(rig, d, fk.rot, fk.pos);
            let rot = hand_rot(&fix_l, [0.15, -0.7, -0.7], [1.0, 0.2, 0.1]);
            two_bone_ik(rig, d, fk.rot, fk.pos, role(UPPER_ARM_L), role(FOREARM_L), role(HAND_L), &lock(sl, [0.16, -0.72, -0.68], arm_l), Some(&[0.5, 0.2, -0.8]), 0.6, Some(&rot), false);
            hand(d, hh, 0, &[BLOW.loose[0], BLOW.loose[1], BLOW.loose[2], BLOW.loose[3]], 0.3, 0.0, tune);
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
    if !ask_blend(g, at, at_b, 1.0, true, at, n, tune) {
        return 0;
    }
    if w_all > 0.0 {
        with_pose(g, at, |d, rig, _n, fk| look_at(d, rig, fk, &gaze_local, tgt, 0.6 * w_all, 0.0));
    }
    hands_to(&hs, an);
    *shown = max(w_all, w_l);
    RAN | RAN_ALL
}
/// The knife's blow, the right arm over the punch clip's pose, by IK on a copy: to the hilt, the knife drawn down out of the sheath, through the kind's
/// keys (eased key to key, the wrist's spots off the right shoulder, turned onto the man), back to the sheath's mouth and home; then the arm goes back
/// to what the pose under it has. The clock is the caller's.
pub fn knife_pose(g: usize, an: &mut Anim, k: &KnifeSlash, t: f64, t0: f64, total: f64, tw_y: f64, tw_p: f64, there: Option<[f64; 3]>, rc_inv: [f64; 4], tune: &Knobs) -> u32 {
    let (at, at_b, _) = poses(an);
    let (kb_t, kb_r0, under, crouch, fix_r) = (an.kb_t, an.kb_r0, if an.rc_t > 0.0 { an.reach_shown } else { 0.0 }, an.crouch, an.hand_fix[1]);
    let (has_at, rd_at, end, keys, nk) = (there.is_some(), there.unwrap_or([0.0; 3]), k.end, k.keys, k.keys.len());
    if nk > KNIFE_KEYS {
        return 0;
    }
    let Some(Some(n)) = with_pose(g, at, |_d, _rig, n, _fk| if cast(n, &ARM_ROLES) { Some(n) } else { None }) else {
        return 0;
    };
    let Some(mut hs) = hands_of(g, an) else {
        return 0;
    };
    if !ask_blend(g, at, at, 1.0, false, at_b, n, tune) {
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
                az = clamp(atan2(gp[0] - mx, gp[2] - mz), -KNIFE_BLOW.az, KNIFE_BLOW.az);
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
                return sheath_fist(fk, &rc_inv, &fix_r, crouch, KNIFE_BLOW.clear[0], KNIFE_BLOW.clear[1]);
            }
            let q = &keys[i - 2];
            let mut o = fr(q.at);
            let (l, cap) = (len3(&o), KNIFE_BLOW.reach * arm);
            if l > cap {
                o = scale3(o, cap / l);
            }
            let bl = fr(q.blade);
            (add3(sr, o), knife_rot(&fix_r, bl, cross3(&fr(q.fing), &bl)), fr(q.pole))
        };
        let mut ts = [0.0f64; 8];
        ts[1] = KNIFE_BLOW.t_clear;
        for i in 0..nk {
            ts[2 + i] = keys[i].t;
        }
        ts[nk + 2] = end - KNIFE_BLOW.t_clear;
        ts[nk + 3] = end;
        let mut i = 0;
        while i < nk + 2 && t >= ts[i + 1] {
            i += 1;
        }
        let (sa_, sb_) = (stop(i), stop(if i + 1 < nk + 3 { i + 1 } else { nk + 3 }));
        let ez = if i >= nk + 3 || t <= 0.0 { 0.0 } else { smoothstep(ts[i], ts[i + 1], t) };
        two_bone_ik(rig, d, fk.rot, fk.pos, role(UPPER_ARM_R), role(FOREARM_R), role(HAND_R), &lerp3(&sa_.0, &sb_.0, ez), Some(&lerp3(&sa_.2, &sb_.2, ez)), 0.9, Some(&slerp(&sa_.1, &sb_.1, ez)), false);
        hand(d, hh, 1, &[REACH.curl[0], REACH.curl[1], REACH.curl[2], REACH.curl[3]], REACH.thumb, 0.0, tune);
    });
    let w = lerp(under, 1.0, max(kb_r0, smoothstep(0.0, if falsy(t0) { 1e-6 } else { t0 }, kb_t)) * (1.0 - smoothstep(total - KNIFE_BLOW.fade, total, kb_t)));
    ask_mask(&[(role(SHOULDER_R), w, false), (role(UPPER_ARM_R), w, false), (role(FOREARM_R), w, false), (role(HAND_R), w, true)]);
    if !ask_blend(g, at, at_b, 1.0, true, at, n, tune) {
        return 0;
    }
    hands_to(&hs, an);
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
/// The ready pose, a small layer: the trunk bowed toward the man (and bladed for the guard) straight into the pose by the weight, then the arms by IK on
/// a copy in the shoulders' frame as it now stands, blended in shoulder → elbow → hand: the left always, the right by the kind's share, or on the
/// knife's hilt by the reach's clock. Every number is eased (`Anim.rd`).
pub fn ready_layer(g: usize, an: &mut Anim, w: f64, kind: ReadyKind, shape: &ReadyShape, live: bool, dt: f64, tw_y: f64, tw_p: f64, there: Option<[f64; 3]>, rc_inv: [f64; 4], tune: &Knobs) -> u32 {
    let (at, at_b, _) = poses(an);
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
    let (grab, ready_up, has_wp, wp) = (kind == ReadyKind::Grab, an.ready_up, there.is_some(), there.unwrap_or([0.0; 3]));
    let bare = an.no_knife && grab && !an.grip[1];
    let (rd_a, u, rd_out_r, close_k, cl_t, knife_on, crouch) = (an.rd_a, an.rc_t, an.rd_out_r, an.close_k, an.cl_t, shape.knife && !an.no_knife, an.crouch);
    let (fix_l, fix_r) = (an.hand_fix[0], an.hand_fix[1]);
    let (l, r) = (&shape.l, &READY_BARE.r);
    let (l_pole, l_fing, l_back, l_curl, l_thumb, l_spread) = if bare {
        let b = &READY_BARE.l;
        (b.pole, b.fing, b.back, b.curl, b.thumb, b.spread)
    } else {
        (l.pole, l.fing, l.back, l.curl, l.thumb, l.spread)
    };
    let (reach, spot) = (l.reach, l.at.unwrap_or([0.0; 3]));
    let rd_live = an.rd_live;
    let kq = if !rd_live {
        1.0
    } else if live {
        1.0 - exp(-READY_RATE * dt)
    } else {
        0.0
    };
    let e = w * w * (3.0 - 2.0 * w);
    let (mut rd, mut cl_l, mut reach_shown) = (core::mem::take(&mut an.rd), an.cl_l, an.reach_shown);
    let c = &mut rd[..];
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
        ease(c, LEAN, shape.lean * DEGR, kq);
        ease(c, BLADE, shape.blade * DEGR, kq);
        ease(c, CLAV, shape.clav * DEGR, kq);
        if !rd_live {
            // (never snapped: it comes in from the reach as it was, at its own rate)
            c[UP] = 0.0;
        }
        if ready_up > 0.0 || c[UP] > 1e-4 {
            // (the grab alone: a pistol that is up keeps its aim and a seated man is under him)
            let to = if ready_up > 0.0 && has_wp && grab { ready_up * smoothstep(READY_UP.over[0], READY_UP.over[1], untwist(fk, tw_y, tw_p, wp)[1] - fk.at(role(UPPER_ARM_L))[1]) } else { 0.0 };
            ease(c, UP, to, if live { 1.0 - exp(-READY_UP.rate * dt) } else { 0.0 });
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
            let r = hi * READY_UP.rise * DEGR;
            turn(d, rig, role(SPINE1), axis_angle(&ax, -r * 0.3));
            turn(d, rig, role(SPINE2), axis_angle(&ax, -r * 0.35));
            turn(d, rig, role(SPINE3), axis_angle(&ax, -r * 0.35));
            turn(d, rig, role(NECK), axis_angle(&ax, r * READY_UP.look * 0.45));
            turn(d, rig, role(HEAD), axis_angle(&ax, r * READY_UP.look * 0.55));
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
    let Some(mut hs) = hands_of(g, an) else {
        return 0;
    };
    // ---- the arms, on a copy: the left clavicle forward with a reach, then the shoulders' frame as it stands
    if !ask_blend(g, at, at, 1.0, false, at_b, n, tune) {
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
                    az = clamp(atan2(pb, pa), -READY_AZ[0], READY_AZ[1]);
                }
            }
            let o_l = if let Some(q) = reach {
                let (frac, r0, r1, dy0, dy1, bias) = (q.frac, q.r[0], q.r[1], q.dy[0], q.dy[1], q.bias);
                if !has_wp {
                    h = q.h0;
                    up = q.y0;
                }
                let q = c[UP];
                let lift = if bare { smoothstep(READY_BARE.way.lift[0], READY_BARE.way.lift[1], rd_a) } else { 1.0 };
                let mut o = if bare {
                    // (out ahead of his own shoulder, never over his head)
                    let y = lerp(READY_BARE.way.low, clamp(READY_BARE.l.frac * up + bias, dy0, READY_BARE.l.dy), lift);
                    let z = lerp(READY_BARE.way.near, clamp(frac * h, READY_BARE.l.r[0], max(READY_BARE.l.r[0], min(READY_BARE.l.r[1], h - READY_BARE.l.short))), lift);
                    frame([0.0, y, z], az)
                } else {
                    frame([0.0, clamp(frac * up + bias, dy0, lerp(dy1, READY_UP.dy, q)), clamp(frac * h, lerp(r0, READY_UP.r[0], q), lerp(r1, READY_UP.r[1], q))], az)
                };
                let (l, cap) = (len3(&o), READY_REACH * arm);
                if l > cap {
                    o = scale3(o, cap / l);
                }
                o
            } else {
                // (a fixed spot is still turned to face him: the guard's lead hand is out toward the man)
                frame(spot, az)
            };
            ease3(c, R_L, o_l, kq);
            ease3(c, L_POLE, frame(l_pole, az), kq);
            let fing = if c[UP] > 1e-4 { [l_fing[0], l_fing[1] + READY_UP.fing * c[UP], l_fing[2]] } else { l_fing };
            ease3(c, L_FING, frame(fing, az), kq);
            ease3(c, L_BACK, frame(l_back, az), kq);
            ease(c, L_CURL, l_curl, kq);
            ease(c, L_THUMB, l_thumb, kq);
            ease(c, L_SPREAD, l_spread, kq);
            if bare {
                // (the right hand's numbers snap when it had no share: it comes in from the arm as it is, by the weight)
                let kr = if c[W_R] < 1e-3 { 1.0 } else { kq };
                ease3(c, R_R, frame(r.at, az), kr);
                ease3(c, R_POLE, frame(r.pole, az), kr);
                ease3(c, R_FING, frame(r.fing, az), kr);
                ease3(c, R_BACK, frame(r.back, az), kr);
                ease(c, R_CURL, r.curl, kr);
                ease(c, R_THUMB, r.thumb, kr);
            }
            ease(c, W_R, if bare { 1.0 } else { 0.0 }, kq);
        } else if !rd_live {
            c[W_R] = 0.0;
        }
        let rot = hand_rot(&fix_l, get3(c, L_FING), get3(c, L_BACK));
        two_bone_ik(rig, d, fk.rot, fk.pos, role(UPPER_ARM_L), role(FOREARM_L), role(HAND_L), &add3(sl, get3(c, R_L)), Some(&get3(c, L_POLE)), 0.85, Some(&rot), false);
        let q = c[L_CURL];
        hand(d, hh, 0, &[q, q + 0.04, q + 0.08, q + 0.12], c[L_THUMB], c[L_SPREAD], tune);
        // (the right hand comes off his ribs for the last stretch only; never under a hand that is on a hilt)
        let w_rb = if u > 0.0 { 0.0 } else { c[W_R] * rd_out_r * smoothstep(READY_BARE.way.right[0], READY_BARE.way.right[1], rd_a) };
        if w_rb > 1e-3 {
            let rot = hand_rot(&fix_r, get3(c, R_FING), get3(c, R_BACK));
            two_bone_ik(rig, d, fk.rot, fk.pos, role(UPPER_ARM_R), role(FOREARM_R), role(HAND_R), &add3(sr, get3(c, R_R)), Some(&get3(c, R_POLE)), 0.85, Some(&rot), false);
            let q = c[R_CURL];
            hand(d, hh, 1, &[q, q + 0.04, q + 0.08, q + 0.12], c[R_THUMB], 0.0, tune);
        }
        // ---- the right hand on the knife: the fist closes on the sheathed handle; the knife stays where it is
        if u > 0.0 {
            let (s_at, s_rot, s_pole) = sheath_fist(fk, &rc_inv, &fix_r, crouch, 0.0, 0.0);
            two_bone_ik(rig, d, fk.rot, fk.pos, role(UPPER_ARM_R), role(FOREARM_R), role(HAND_R), &s_at, Some(&s_pole), 0.9, Some(&s_rot), false);
            if close_k >= 0.0 {
                // (the fist on the hilt is alive: the fingers let go a little, little finger first, and close again; the thumb shifts with them)
                let ph = (cl_t % tune.kit.close.regrip) / tune.kit.close.regrip;
                let gg = |dv: f64| bump(ph, 0.55 + dv, 0.66 + dv, 0.72 + dv, 0.86 + dv);
                hand(d, hh, 1, &[REACH.curl[0] - 0.14 * gg(0.06), REACH.curl[1] - 0.2 * gg(0.04), REACH.curl[2] - 0.26 * gg(0.02), REACH.curl[3] - 0.3 * gg(0.0)], REACH.thumb - 0.25 * gg(0.03), 0.0, tune);
            } else {
                hand(d, hh, 1, &[REACH.curl[0], REACH.curl[1], REACH.curl[2], REACH.curl[3]], REACH.thumb, 0.0, tune);
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
            cl_l = 0.0;
        }
        cl_l = clamp(cl_l + if live && u >= tune.kit.close.left_at { dt / (tune.kit.close.left_in * (1.0 + READY_UP.slow * c[UP])) } else { -dt / tune.kit.close.left_out }, 0.0, 1.0);
        lg = cl_l * cl_l * (3.0 - 2.0 * cl_l);
    }
    reach_shown = smoothstep(0.16, 1.0, u);
    let right = u > 0.0 || w_rb > 1e-3;
    let (r_up, r_fore, r_hand) = if u > 0.0 { (smoothstep(0.0, 0.75, u), smoothstep(0.08, 0.88, u), reach_shown) } else { (w_up * w_rb, w_fore * w_rb, w_hand * w_rb) };
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
    if !ask_blend(g, at, at_b, 1.0, true, at, n, tune) {
        return 0;
    }
    hands_to(&hs, an);
    (an.rd, an.cl_l, an.reach_shown, an.rd_live) = (rd, cl_l, reach_shown, true);
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
/// A place and three axes
#[derive(Clone, Copy)]
struct Frame {
    o: [f64; 3],
    x: [f64; 3],
    y: [f64; 3],
    z: [f64; 3],
}
/// an Adj as a layer is handed it: is there one, its offset along the parent's axes, its turn in degrees about the parent's x, then y, then z
type Adj = (bool, [f64; 3], [f64; 3]);
fn frame_from(o: [f64; 3], along: [f64; 3], up_hint: [f64; 3]) -> Frame {
    let z = normalize(along);
    let mut y = sub3(up_hint, scale3(z, dot3(&up_hint, &z)));
    if len3(&y) < 1e-3 {
        y = sub3([1.0, 0.0, 0.0], scale3(z, z[0]));
    }
    let y = normalize(y);
    Frame { o, x: cross3(&y, &z), y, z }
}
/// The child frame = the parent's moved by the offset and turned about the parent's x, then y, then z
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
/// rigProps.ts applyAdj backwards — the parent frame whose child by `adj` is g
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
/// The thigh holster's group frame off three joints and the body's forward, in whatever one space they are given in;
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
/// The kit change's frame: the right hand's errand between what was in it and what will be, both arms by IK on a copy of the pose, whole from the upper
/// arm down. The path begins on the hand as the pose under it had it when the change began and ends on the hand of the pose under it now. The clock is
/// the caller's.
pub fn kit_layer(g: usize, an: &mut Anim, tw_y: f64, tw_p: f64, rc_inv: [f64; 4], sling: &SlingFrontHand, tune: &Knobs) -> u32 {
    let (at, at_b, _) = poses(an);
    let Some(k) = an.kit else {
        return 0;
    };
    let (t, t_a, t_r, dur, s0, roll0, re, front, from, to, cap, crouch, fix_r) = (k.t, k.t_a, k.t_r, k.d, k.s0, k.roll0, k.re, k.front, k.from, k.to, k.cap, an.crouch, an.hand_fix[1]);
    let (h, p) = (&RIG.holster, &RIG.pistol.adj);
    let adj = |a: &Option<RigAdj>| -> Adj { a.map_or((false, [0.0; 3], [0.0; 3]), |v| (true, v.off, v.rot)) };
    let (holster, h_adj, p_adj) = ([h.down, h.out, h.fwd], adj(&h.adj), adj(p));
    const PISTOL: Option<KitItem> = Some(KitItem::Pistol);
    const RIFLE: Option<KitItem> = Some(KitItem::Rifle);
    const CAN: Option<KitItem> = Some(KitItem::Can);
    let Some(Some(n)) = with_pose(g, at, |_d, _rig, n, _fk| if cast(n, &ARM_ROLES) && cast(n, &LEG_ROLES) && cast(n, &[SPINE2]) { Some(n) } else { None }) else {
        return 0;
    };
    let Some(mut hs) = hands_of(g, an) else {
        return 0;
    };
    if !ask_blend(g, at, at, 1.0, false, at_b, n, tune) {
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
        let st: Spot = (match cap.mr { Some(m) => un(m), None => add3(s_r, dir_t(cap.r)) }, match (cap.mf, cap.mb) { (Some(f), Some(b)) => hand_rot(&fix_r, und(f), und(b)), _ => q_mul(&q3, &cap.rq) }, None);
        // where the right wrist is, how the hand lies and where the elbow points with the thing in the fist at the place it lives
        let spot = |it: Option<KitItem>| -> Spot {
            if it == RIFLE && front {
                return (add3(fk.at(role(SPINE2)), dir_t(sling.at)), hand_rot(&fix_r, dir_t(sling.fing), dir_t(sling.back)), Some(dir_t(tune.kit.front.pole)));
            }
            if it == RIFLE {
                let (p, f, b) = (tune.kit.apex.at, tune.kit.apex.fing, tune.kit.apex.back);
                return (add3(fk.at(role(UPPER_ARM_R)), dir_t(p)), hand_rot(&fix_r, dir_t(f), dir_t(b)), Some(dir_t(tune.kit.apex.pole)));
            }
            if it == CAN {
                let (p, f, b) = (tune.kit.belt.at, tune.kit.belt.fing, tune.kit.belt.back);
                return (un(add3(fk.at(role(HIPS)), dir_t(p))), hand_rot(&fix_r, und(dir_t(f)), und(dir_t(b))), Some(dir_t(tune.kit.belt.pole)));
            }
            if it == PISTOL {
                // the hand that holds the gun as it stands in the holster
                let p = un_adj(&holster_frame_of(fk.at(role(THIGH_R)), fk.at(role(SHIN_R)), fk.at(role(HIPS)), [0.0, 0.0, 1.0], holster, &h_adj), &p_adj);
                return (un(p.o), hand_rot(&fix_r, und(p.z), und(scale3(p.x, -1.0))), Some(dir_t(tune.kit.holster_pole)));
            }
            let (p, r, pl) = sheath_fist(fk, &rc_inv, &fix_r, crouch, 0.0, 0.0);
            (p, r, Some(pl))
        };
        let arc_of = |it: Option<KitItem>| -> [f64; 3] {
            if it == PISTOL {
                *tune.kit.arc.get(game_types::character::KitItem::Pistol).unwrap()
            } else if it == RIFLE {
                *tune.kit.arc.get(game_types::character::KitItem::Rifle).unwrap()
            } else if it == CAN {
                *tune.kit.arc.get(game_types::character::KitItem::Can).unwrap()
            } else {
                *tune.kit.arc.get(game_types::character::KitItem::Knife).unwrap()
            }
        };
        // (the rifle's home: over the shoulder onto his back, or let down on its sling in front)
        let back = rifle && !front;
        let rarc = if front { tune.kit.front.arc } else { arc_of(RIFLE) };
        let (over, cross) = (tune.kit.over, tune.kit.cross);
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
        let top = if back { Some(hand_rot(&fix_r, dir_t(tune.kit.top.fing), dir_t(tune.kit.top.back))) } else { None };
        let ap = clamp(if front { tune.kit.front.hand_at } else { tune.kit.apex_at }, 0.05, 0.95);
        let sl = clamp(if front { tune.kit.front.sling_at } else { tune.kit.sling_at }, 0.05, ap);
        let (has_from, has_to) = (from.is_some(), to.is_some());
        let mut hold: Option<KitItem> = None;
        let (mut stow, mut roll, mut join, mut free) = (0.0, 0.0, 0.0, 0.0);
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
            roll = max(smoothstep(0.0, if it == RIFLE { tune.kit.roll_in } else { 0.35 }, hu), roll0);
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
                    tune.kit.up
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
            join = smoothstep(tune.kit.join, 1.0, hu);
            roll = if re { roll0 * (1.0 - smoothstep(0.4, 1.0, hu)) } else { 1.0 - smoothstep(if it == RIFLE { 1.0 - tune.kit.roll_in } else { 0.65 }, 1.0, hu) };
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
            let r_min = sqrt(max(0.0, la * la + lb * lb - 2.0 * la * lb * cos(tune.kit.elbow_min * PI / 180.0))) * own;
            let dv = sub3(tg.0, s_r);
            let dl = len3(&dv);
            if dl < r_min {
                tg.0 = mad3(s_r, if dl > 1e-4 { scale3(dv, 1.0 / dl) } else { dir_t([-0.4, 0.2, 0.9]) }, r_min);
            }
        }
        // the support hand, with a gun coming: off whatever it had to wait in front of him, onto the gun late
        let gun_next = to == PISTOL || to == RIFLE;
        let s_l0: Spot = (match cap.ml { Some(m) => un(m), None => add3(s_l, dir_t(cap.l)) }, q_mul(&q3, &cap.lq), None);
        let mut tl: Option<Spot> = None;
        if gun_next {
            let rest: Spot = (add3(s_l, dir_t(tune.kit.rest)), s_l0.1, None);
            tl = Some(mix(&mix(&s_l0, &rest, e3(t / 0.22), None, None), &u_l, join, None, None));
        } else if t < tune.kit.let_go {
            // …and no gun coming: it lets go and falls away, not snapped off
            join = e3(t / tune.kit.let_go);
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
                let (ang, kn) = (acos(clamp(dot3(&fd, &fg), -1.0, 1.0)), tune.kit.wrist_knee * PI / 180.0);
                let mx = max(kn + 0.01, tune.kit.wrist_max * PI / 180.0);
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
        let curl = [lerp(tune.kit.open[0], tune.kit.fist[0], grip), lerp(tune.kit.open[1], tune.kit.fist[1], grip), lerp(tune.kit.open[2], tune.kit.fist[2], grip), lerp(tune.kit.open[3], tune.kit.fist[3], grip)];
        hand(d, hh, 1, &curl, lerp(0.25, 0.6, grip), 0.1 * (1.0 - grip), tune);
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
    if !ask_blend(g, at, at_b, 1.0, true, at, n, tune) {
        return 0;
    }
    hands_to(&hs, an);
    (an.kit_hold, an.kit_stow, an.kit_roll, an.kit_lie) = (Some(hold), stow, roll, lie);
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
/// A hand on a door: the spine bowed into a long reach, then the arm that is up by IK on a copy, its target travelling with the eased weight from
/// wherever the pose beneath has the wrist to the spot on the leaf, the palm flat on the face.
pub fn door_layer(g: usize, an: &mut Anim, l: &DoorPush, n: [f64; 3], p0: [f64; 3], dt: f64, intent: f64, right_free: bool, tw_y: f64, tw_p: f64, tune: &Knobs) -> u32 {
    let (at, at_b, _) = poses(an);
    let (hand_no, reach, fix, dp) = (l.hand, l.reach.unwrap_or(false), [an.hand_fix[0], an.hand_fix[1]], &mut an.dp);
    let (live, w_was) = (intent > 0.003, clamp(max(dp.w_l.x, dp.w_r.x), 0.0, 1.0));
    if live && hand_no >= 0.0 {
        dp.right = hand_no == 1.0 && right_free;
    }
    let right = dp.right;
    let arm = if right { 1 } else { 0 };
    let n0 = normalize(n);
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
            let ok = if dp.reach_ok { dd < 0.585 + far && dd > 0.16 && ahead > 0.02 } else { dd < 0.56 + far && dd > 0.2 && ahead > 0.06 };
            dp.reach_ok = ok;
            if ok {
                if right {
                    tgt_r = intent;
                } else {
                    tgt_l = intent;
                }
            }
        } else {
            dp.reach_ok = false;
        }
        // ≈ 0.15 s in, 0.25 s out
        let rate = |t: f64, x: f64| if t > x { 40.0 } else { 26.0 };
        let (rl, rr) = (rate(tgt_l, dp.w_l.x), rate(tgt_r, dp.w_r.x));
        spring_step(&mut dp.w_l, tgt_l, rl, dt);
        spring_step(&mut dp.w_r, tgt_r, rr, dt);
        Some((n, tw, f, p_tgt, clamp(dp.w_l.x, 0.0, 1.0), clamp(dp.w_r.x, 0.0, 1.0)))
    }) else {
        return 0;
    };
    let Some(mut hs) = hands_of(g, an) else {
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
        if !ask_blend(g, at, at, 1.0, false, at_b, n, tune) {
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
            hand(d, hh, side, &[0.1, 0.1, 0.15, 0.2], 0.1, 0.35, tune);
            turn(d, rig, digits(side)[0] as usize, axis_angle(&q_rotate(&fk.rot(hd), &AY), sg * 40.0 * DEGR * e));
        });
        ask_mask(&[(arm_of(side).0, 1.0, true)]);
        if !ask_blend(g, at, at_b, smoothstep(0.0, 0.25, e), true, at, n, tune) {
            return 0;
        }
    }
    hands_to(&hs, an);
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
    kind: WorkKind,
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
        Shape { wrist: mad3(self.at(x, y, off), f, -WORK.tip_len), f, b, curl, thumb, spread: 0.0, body: false }
    }
    /// what a hand does, by the verb (0 keys, 1 charge, 2 lever, 3 take); `fk` as it stands
    fn shape(&self, fk: &Fk, side: usize) -> Shape {
        let (sg, lead, t, dur) = (if side == 1 { 1.0 } else { -1.0 }, side == self.main, self.t, self.dur);
        if self.kind == WorkKind::Keys {
            let (mut kx, mut ky, mut press) = (self.keys[0][0], self.keys[0][1], 0.0);
            for i in 0..4usize {
                // (when key i of a code is pressed, on a verb `dur` long)
                let tk = WORK.key_lead + (i as f64 + 0.5) * (dur - WORK.key_lead) / 4.0;
                let key = self.keys[i % self.nk];
                if i != 0 {
                    let gl = smoothstep(tk - 0.19, tk - 0.06, t);
                    kx = lerp(kx, key[0], gl);
                    ky = lerp(ky, key[1], gl);
                }
                press += bump(t, tk - 0.07, tk - 0.005, tk + 0.03, tk + 0.1);
            }
            return self.poke(side, kx, ky, WORK.key_off + WORK.hover * (1.0 - press), [0.03, 0.8, 0.88, 0.92], 0.8);
        }
        if self.kind == WorkKind::Charge {
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
        if self.kind == WorkKind::Lever {
            let thr = smoothstep(dur - 0.26, dur - 0.1, t);
            let jig = if dur > 1.2 { bump(t, 0.45, 0.55, 0.62, 0.8) + bump(t, 1.0, 1.1, 1.17, 1.35) } else { 0.0 };
            let gp = self.at(0.0, self.dir * (WORK.lever * (1.0 - 2.0 * thr) - 0.012 * jig), 0.05);
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
/// Hands at work on a thing: the body bowed to a low thing and leaned into a long reach, the eyes onto it, then each hand that is up by IK on a copy,
/// its target travelling with the eased weight.
pub fn work_layer(g: usize, an: &mut Anim, l: &WorkPose, n_at: [f64; 3], p0: [f64; 3], dt: f64, live: bool, r_free: bool, tw_y: f64, tw_p: f64, tune: &Knobs) -> u32 {
    let (at, at_b, _) = poses(an);
    let (t, dur, kind, dir, gaze_local, fix) = (l.t, l.dur, l.kind, l.dir.unwrap_or(1.0), an.gaze_local, [an.hand_fix[0], an.hand_fix[1]]);
    // (four presses: the first four keys are all a code's hand reads)
    let mut keys = [[0.0f64; 2]; 4];
    let mut nk = 1;
    if let Some(k) = l.keys.as_ref().filter(|k| !k.is_empty()) {
        nk = if k.len() < 4 { k.len() } else { 4 };
        keys[..nk].copy_from_slice(&k[..nk]);
    }
    let wk_ = &mut an.wk;
    let (w_was, done) = (clamp(max(wk_.w_l.x, wk_.w_r.x), 0.0, 1.0), t > dur);
    if live && !done {
        // (which hands: settled while the verb runs, kept while it comes away)
        wk_.main = if r_free { 1.0 } else { 0.0 };
        wk_.two = (kind == WorkKind::Charge || kind == WorkKind::Take) && r_free;
    }
    let (main, two) = (if wk_.main != 0.0 { 1usize } else { 0 }, wk_.two);
    let n0 = normalize(n_at);
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
        if !ask_blend(g, at, at, 1.0, false, at_b, n, tune) {
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
    let Some(mut hs) = hands_of(g, an) else {
        return 0;
    };
    let c = &mut an.wk;
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
                let ok = sp.body || if (if side == 1 { c.ok_r } else { c.ok_l }) { dd < 0.6 && dd > 0.14 && ahead > 0.0 } else { dd < 0.57 && dd > 0.18 && ahead > 0.04 };
                *(if side == 1 { &mut c.ok_r } else { &mut c.ok_l }) = ok;
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
        let (rl, rr) = (rate(tgt_l, c.w_l.x), rate(tgt_r, c.w_r.x));
        spring_step(&mut c.w_l, tgt_l, rl, dt);
        spring_step(&mut c.w_r, tgt_r, rr, dt);
        let (w_l, w_r) = (clamp(c.w_l.x, 0.0, 1.0), clamp(c.w_r.x, 0.0, 1.0));
        c.shown = if main == 1 { w_r } else { w_l };
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
        if !ask_blend(g, at, at, 1.0, false, at_b, n, tune) {
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
            hand(d, hh, side, &sp.curl, sp.thumb, sp.spread, tune);
        });
        ask_mask(&[(arm_of(side).0, 1.0, true)]);
        if !ask_blend(g, at, at_b, smoothstep(0.0, 0.25, e), true, at, n, tune) {
            return 0;
        }
    }
    hands_to(&hs, an);
    RAN | RAN_ALL
}
const WHIP_WIND: f64 = 0.36;
const WHIP_STRIKE: f64 = 0.42;
const WHIP_THRU: f64 = 0.14;
const KILL_RATE: f64 = 40.0;
const KILL_BACK: f64 = 9.0;
/// The disarm's clock on the way in, gentle off the mark and at speed as the hand lands at `take`
fn strip_in(t: f64, take: f64) -> f64 {
    if t >= take {
        return t;
    }
    let x = t / take;
    take * x * x * (2.0 - x)
}
/// character.ts killEase, over its spring
fn kill_ease(c: &mut Spring, on: bool, t: f64, dt: f64, tune: &Knobs) -> f64 {
    let target = if on { clamp((t + 2.0 / KILL_RATE) / tune.hold.secs.kill, 0.0, 1.0) } else { 0.0 };
    spring_step(c, target, if on { KILL_RATE } else { KILL_BACK }, dt);
    clamp(c.x, 0.0, 1.0)
}
/// Down with its clock, back up on a spring of its own
fn lower_ease(c: &mut Spring, target: f64, up: f64, dt: f64) -> f64 {
    if target >= c.x {
        c.x = target;
        c.v = 0.0;
        return target;
    }
    spring_step(c, target, up, dt);
    max(target, c.x)
}
/// The choke's own curve while it runs, let down on a spring when it is let go of
fn haul_ease(c: &mut Spring, on: bool, target: f64, down: f64, dt: f64) -> f64 {
    if on {
        c.x = target;
        c.v = 0.0;
        return target;
    }
    if c.x <= 1e-4 {
        c.x = 0.0;
        c.v = 0.0;
        return 0.0;
    }
    spring_step(c, 0.0, down, dt);
    max(0.0, c.x)
}
/// Sam holding a man from behind: the legs sat, sunk and stepped under him, the trunk over the man's back, then both arms by IK on a copy (the left
/// across his throat, the right the knife's fist or, on a scratch copy cross-faded in, the pistol at his head).
pub fn hold_layer(g: usize, an: &mut Anim, w: f64, dt: f64, h: &HoldPose, kill_on: bool, beats: bool, man: Option<[f64; 3]>, s_at: Option<[f64; 3]>, tune: &Knobs) -> u32 {
    let (at, at_b, _) = poses(an);
    let (at_c, at_aim, live, gun_v, no_knife) = (an.p_c.at as usize, an.p_aim.at as usize, an.hold_pose.is_some(), h.variant == Some(HoldVariant::Gun), an.no_knife);
    let (ph, t, low, haul, yank, step, pull) = (h.phase, h.t, h.low.unwrap_or(0.0), h.haul.unwrap_or(false), h.yank, h.step, h.pull);
    let (grab, held, choke, whip, kill) = (ph == HoldPhase::Grab, ph == HoldPhase::Held, ph == HoldPhase::Choke, ph == HoldPhase::Whip, ph == HoldPhase::Kill);
    let (idle_t, strip, tw_y, tw_p) = (an.idle_t, an.hold_pose.and_then(|h| h.strip).map(|s| s.t), an.aim_yaw_offset, -an.aim_pitch * an.upper_w.aim * 0.35);
    let (gun_out, gun_pitch, shoot_t, rd_hand) = (h.gun_out.unwrap_or(0.0), h.gun_pitch.unwrap_or(0.0), an.shoot_t, an.rd_hand_l);
    let (fix_l, fix_r, grip_up) = (an.hand_fix[0], an.hand_fix[1], an.grip_up);
    // (read before anything is posed: an answer of 0 leaves the pose as it was handed)
    let Some(mut hs) = hands_of(g, an) else {
        return 0;
    };
    // ---- the clocks and shares: nothing here reads the pose
    let gv = clamp(an.hd_gv.x, 0.0, 1.0);
    // the reach, 0 → 1
    let ug = if grab { clamp(t / tune.hold.secs.grab, 0.0, 1.0) } else { 1.0 };
    let cs_on = true && no_knife;
    let cr = cs_on && !gun_v && grab;
    // the elbow closing
    let ck = if choke && !gun_v { smoothstep(0.0, tune.hold.secs.choke * 0.75, t) } else { 0.0 };
    spring_step(&mut an.hd_kn, if choke { 1.0 } else { 0.0 }, if choke { 6.0 / tune.hold.knife.off_secs } else { 3.0 / tune.hold.knife.off_secs }, dt);
    let kn_e = clamp(an.hd_kn.x, 0.0, 1.0);
    let kn = if no_knife { 1.0 } else { kn_e };
    let fight = kn_e * if choke { 1.0 - smoothstep(tune.hold.struggle.fade_from, tune.hold.struggle.fade_to, t / tune.hold.secs.choke) } else { 1.0 };
    let wnd = if whip { smoothstep(0.0, WHIP_WIND, t) } else { 0.0 };
    let stk = if !whip || t < WHIP_STRIKE { 0.0 } else { hermite(t, WHIP_STRIKE, 0.0, 0.0, tune.hold.secs.whip, 1.0, 7.0) };
    let thru = if whip { smoothstep(tune.hold.secs.whip, tune.hold.secs.whip + WHIP_THRU, t) } else { 0.0 };
    let s_t = strip.unwrap_or(0.0);
    let s_arm = strip.is_some() && !gun_v && s_t < tune.hold.strip.home;
    let kk = kill_ease(&mut an.hd_kc, kill_on && kill && !gun_v, t, dt, tune);
    let k_on = kk > 1e-4;
    let k_w = if k_on { smoothstep(0.0, tune.hold.kill.wind, kk) } else { 0.0 };
    let k_s = if k_on { smoothstep(tune.hold.kill.hit - 0.07, tune.hold.kill.out, kk) } else { 0.0 };
    let k_c = if k_on { smoothstep(tune.hold.kill.hit + 0.05, tune.hold.kill.hit + 0.3, kk) } else { 0.0 };
    let k_a = if k_on { smoothstep(tune.hold.kill.go, 1.0, kk) } else { 0.0 };
    let e_body = smoothstep(0.15, 0.9, ug);
    spring_step(&mut an.hd_ck, if choke && !gun_v { clamp(t / tune.hold.secs.choke, 0.0, 1.0) } else { 0.0 }, if choke { 30.0 } else { 9.0 }, dt);
    let lck = clamp(an.hd_ck.x, 0.0, 1.0);
    let lo = lower_ease(&mut an.hd_lo, if low > 0.0 { clamp(low, 0.0, 1.0) * smoothstep(lower_k(beats, tune.hold.beats.lower.from, tune.hold.lower.from), lower_k(beats, tune.hold.beats.lower.by, tune.hold.lower.by), lck) } else { 0.0 }, lower_k(beats, tune.hold.beats.lower.up, tune.hold.lower.up), dt) * (1.0 - gv);
    let lw = if live { 1.0 } else { w };
    let hu = haul_ease(&mut an.hd_hu, haul && choke && !gun_v, smoothstep(tune.hold.haul.in_[0], tune.hold.haul.in_[1], lck) * (1.0 - smoothstep(tune.hold.haul.out[0], tune.hold.haul.out[1], lck)), tune.hold.haul.down, dt) * (1.0 - gv) * lw;
    let hauled = hu > 1e-4;
    let yk = match yank {
        Some(y) if live => bump(y, 0.0, tune.hold.yank.in_, tune.hold.yank.in_ + tune.hold.yank.hold, tune.hold.yank.in_ + tune.hold.yank.hold + tune.hold.yank.out),
        _ => 0.0,
    };
    let hb_s = match yank {
        Some(y) if true && cs_on => bump(y, tune.hold.hit.t[0], tune.hold.hit.t[1], tune.hold.hit.t[2], tune.hold.hit.t[3]) * (1.0 - gv) * w,
        _ => 0.0,
    };
    let dip_w = match step {
        Some(s) if live && grab => bump(s, 0.0, tune.hold.lunge.t[0], tune.hold.lunge.t[0] + tune.hold.lunge.t[1], tune.hold.lunge.t[0] + tune.hold.lunge.t[1] + tune.hold.lunge.t[2]),
        _ => 0.0,
    };
    let sw_live = no_knife && live && held;
    an.hd_sw = if sw_live { sin(t / tune.hold.sway.secs * 2.0 * PI) * smoothstep(0.0, tune.hold.sway.in_, t) } else { damp(an.hd_sw, 0.0, tune.hold.sway.out, dt) };
    let sw_s = an.hd_sw * (1.0 - gv) * w;
    let sw_on = tune.hold.sway.m != 0.0 && (1.0 - gv) * w > 1e-4 && if sw_live { smoothstep(0.0, tune.hold.sway.in_, t) > 1e-4 } else { abs(an.hd_sw) > 1e-4 };
    let seat_f = if cs_on { (if grab { smoothstep(0.55, 1.0, ug) } else { 1.0 }) * (1.0 - gv) * w } else { 0.0 };
    let seat = seat_f * (1.0 - clamp(lo, 0.0, 1.0)) * (1.0 - clamp(hu, 0.0, 1.0));
    let ps = match (pull, yank) {
        (Some(p), Some(_)) => p * w,
        _ => 0.0,
    };
    let lag_s = |y0: f64, y1: f64| if ps > 1e-4 { smoothstep(y0, y1, yank.unwrap_or(0.0)) } else { 1.0 };
    let (e_sl, e_sr) = (lag_s(tune.hold.catch.sam_feet[0][0], tune.hold.catch.sam_feet[0][1]), lag_s(tune.hold.catch.sam_feet[1][0], tune.hold.catch.sam_feet[1][1]));
    let lag_on = ps * (1.0 - min(e_sl, e_sr)) > 1e-4;
    let leg_w = if cs_on {
        lerp(0.6 * clamp(8.0 * max(max(max(max(max(max(max(seat_f, lo), k_a), yk), dip_w), hb_s), if sw_on { 1.0 } else { 0.0 }), 20.0 * ps * (1.0 - min(e_sl, e_sr))), 0.0, 1.0), 0.6, gv)
    } else {
        0.6
    };
    let legs = lo > 1e-4 || k_a > 1e-4 || yk > 1e-4 || ((tune.hold.lunge.dip != 0.0 || tune.hold.lunge.back != 0.0) && dip_w > 1e-4) || sw_on || seat_f > 1e-4 || lag_on || hb_s > 1e-4;
    let ck_e = if low > 0.0 { max(ck, smoothstep(0.0, 0.75, lck)) } else { ck };
    let lean0 = lerp(lerp(tune.hold.lean, tune.hold.gun.lean, gv), tune.hold.choke_lean, ck_e) * DEGR * w * e_body;
    let lean1 = if lo > 1e-4 { lean0 + lower_k(beats, tune.hold.beats.lower.sam_lean, tune.hold.lower.sam_lean) * lo * DEGR * lw } else { lean0 };
    let lean2 = if k_on { lean1 + (tune.hold.kill.lean[0] * k_w * (1.0 - k_a) + tune.hold.kill.lean[1] * k_a) * DEGR * w } else { lean1 };
    let lean = (if hauled { lean2 + tune.hold.haul.sam_lean * hu * DEGR } else { lean2 }) + tune.hold.catch.seat_lean * seat * DEGR - tune.hold.hit.heave * hb_s * DEGR;
    let brace = tune.hold.brace * sin(idle_t * 2.0 * PI * tune.hold.struggle.hz[2] + 0.7) * fight * DEGR * w;
    let side = (lerp(tune.hold.knife.side, tune.hold.gun.side, gv) - if no_knife { tune.hold.apart.side * (1.0 - gv) } else { 0.0 }) * DEGR * w * e_body + brace;
    let yaw_w = (tune.hold.gun.wind_yaw * wnd * (1.0 - stk) + tune.hold.gun.hit_yaw * (stk - thru)) * DEGR * w
        + if s_arm { tune.hold.strip.yaw * bump(s_t, tune.hold.strip.take - 0.05, tune.hold.strip.out - 0.02, tune.hold.strip.out + 0.06, tune.hold.strip.drop + 0.14) * DEGR * w } else { 0.0 };
    let yaw_b = if k_on { yaw_w + (tune.hold.kill.yaw[0] * k_w * (1.0 - k_s) + tune.hold.kill.yaw[1] * k_s * (1.0 - 0.5 * k_a)) * DEGR * w } else { yaw_w };
    let hy = lerp(tune.hold.knife.head_yaw, tune.hold.gun.head_yaw, gv) * DEGR * w * e_body - yaw_b;
    let hp = (lerp(tune.hold.head_pitch, tune.hold.gun.head_pitch, gv) * DEGR - lean) * w * e_body + if lo > 1e-4 { lower_k(beats, tune.hold.beats.lower.sam_look, tune.hold.lower.sam_look) * lower_k(beats, tune.hold.beats.lower.sam_lean, tune.hold.lower.sam_lean) * lo * DEGR * lw } else { 0.0 } + if k_on { tune.hold.kill.look * k_a * DEGR * w } else { 0.0 };
    let limp0 = if choke && !gun_v { smoothstep(0.62 * tune.hold.secs.choke, 0.95 * tune.hold.secs.choke, t) } else { 0.0 };
    let limp = if low > 0.0 { max(limp0, smoothstep(0.62, 0.95, lck)) } else { limp0 };
    let mut give = if lo > 1e-4 {
        [0.0, -(tune.hold.sag[1] - tune.hold.sag[0]) * limp - lower_k(beats, tune.hold.beats.lower.sam_arm[0], tune.hold.lower.sam_arm[0]) * lo, 0.025 * limp + lower_k(beats, tune.hold.beats.lower.sam_arm[1], tune.hold.lower.sam_arm[1]) * lo]
    } else {
        [0.0, -(tune.hold.sag[1] - tune.hold.sag[0]) * limp, 0.025 * limp]
    };
    if hauled {
        give[1] += tune.hold.haul.sam_arm[0] * hu;
        give[2] -= tune.hold.haul.sam_arm[1] * hu;
    }
    if seat > 1e-4 {
        give[1] -= tune.hold.catch.sink * seat / max(w, 1e-3);
    }
    if hb_s > 1e-4 {
        give[1] -= tune.hold.hit.dip * hb_s / max(w, 1e-3);
    }
    let (hold_l, hold_r) = (add3(lerp3(&tune.hold.throat, &tune.hold.tight, ck_e), give), add3(lerp3(&tune.hold.knife.hand, &tune.hold.knife.off_hand, kn), give));
    let (e_l, e_r) = (if grab { smoothstep(0.1, 0.7, ug) } else { 1.0 }, if grab { smoothstep(0.3, 0.95, ug) } else { 1.0 });
    let xg = smoothstep(0.3, 0.7, gv);
    let (back_k, back_g) = (if grab { 0.0 } else { smoothstep(0.0, 0.55, gv) }, if grab { 0.0 } else { 1.0 - smoothstep(0.45, 1.0, gv) });
    // ---- the legs under him, then body English straight into the pose
    let Some(Some(n)) = with_pose(g, at, |d, rig, n, fk| {
        if !(cast(n, &ARM_ROLES) && cast(n, &LEG_ROLES) && cast(n, &[SPINE1, SPINE2, NECK, HEAD, MIDDLE1_R])) {
            return None;
        }
        if legs {
            fk_run(rig, d, fk.rot, fk.pos);
            let (f_l, f_r, r_l, r_r) = (fk.at(role(FOOT_L)), fk.at(role(FOOT_R)), fk.rot(role(FOOT_L)), fk.rot(role(FOOT_R)));
            let off = if k_a > 1e-4 {
                [0.0, -lower_k(beats, tune.hold.beats.lower.sam_sink, tune.hold.lower.sam_sink) * lo * lw - tune.hold.kill.sam_sink * k_a * w, lower_k(beats, tune.hold.beats.lower.sam_fwd, tune.hold.lower.sam_fwd) * lo * lw]
            } else {
                [
                    tune.hold.sway.m * sw_s + tune.hold.catch.seat[2] * seat,
                    -lower_k(beats, tune.hold.beats.lower.sam_sink, tune.hold.lower.sam_sink) * lo * lw - tune.hold.yank.sink * yk * w * (if cs_on { tune.hold.catch.yank_sit } else { 1.0 }) - tune.hold.lunge.dip * dip_w - tune.hold.catch.seat[1] * seat - tune.hold.hit.sam_dip * hb_s,
                    lower_k(beats, tune.hold.beats.lower.sam_fwd, tune.hold.lower.sam_fwd) * lo * lw - tune.hold.lunge.back * dip_w - tune.hold.catch.seat[0] * seat + tune.hold.hit.sam_in * hb_s,
                ]
            };
            shift(d, rig, role(HIPS), &off);
            fk_run(rig, d, fk.rot, fk.pos);
            let pole = |r: &[f64; 4], sx: f64| {
                let kp = knee_pole(r);
                normalize([kp[0] + sx * lower_k(beats, tune.hold.beats.lower.sam_knees, tune.hold.lower.sam_knees) * lo, kp[1], kp[2]])
            };
            let step_s = |f: [f64; 3], e: f64| if lag_on { [f[0], f[1] + tune.hold.catch.sam_lift * sin(PI * e) * min(1.0, ps / 0.1), f[2] + ps * (1.0 - e)] } else { f };
            let (s_l, s_r) = (step_s(f_l, e_sl), step_s(f_r, e_sr));
            two_bone_ik(rig, d, fk.rot, fk.pos, role(THIGH_L), role(SHIN_L), role(FOOT_L), &s_l, Some(&pole(&r_l, 1.0)), leg_w, Some(&r_l), false);
            // (the rear foot back and out under the seat)
            let t_r = if seat_f > 1e-4 {
                [s_r[0] - tune.hold.catch.brace[1] * seat_f, s_r[1] + if grab { tune.hold.catch.brace_lift * sin(PI * smoothstep(0.55, 1.0, ug)) * seat_f } else { 0.0 }, s_r[2] - tune.hold.catch.brace[0] * seat_f]
            } else {
                s_r
            };
            two_bone_ik(rig, d, fk.rot, fk.pos, role(THIGH_R), role(SHIN_R), role(FOOT_R), &t_r, Some(&pole(&r_r, -1.0)), leg_w, Some(&r_r), true);
        }
        turn(d, rig, role(SPINE1), q_mul(&q_yaw(yaw_b * 0.3), &q_pitch(lean * 0.45)));
        turn(d, rig, role(SPINE2), q_mul(&q_yaw(yaw_b * 0.35), &q_mul(&q_roll(side * 0.4), &q_pitch(lean * 0.35))));
        turn(d, rig, role(SPINE3), q_mul(&q_yaw(yaw_b * 0.35), &q_mul(&q_roll(side * 0.6), &q_pitch(lean * 0.2))));
        turn(d, rig, role(NECK), q_mul(&q_roll(side * 0.8 - brace), &q_mul(&q_yaw(hy * 0.4), &q_pitch(hp * 0.4))));
        turn(d, rig, role(HEAD), q_mul(&q_roll(-side * 0.9), &q_mul(&q_yaw(hy * 0.6), &q_pitch(hp * 0.6))));
        turn(d, rig, role(SHOULDER_L), q_roll(5.0 * DEGR * w * e_body));
        turn(d, rig, role(SHOULDER_R), q_roll(-lerp(tune.hold.knife.clav * (1.0 - 0.6 * kn), tune.hold.gun.clav + 4.0 * wnd * (1.0 - stk), gv) * DEGR * w * e_body));
        Some(n)
    }) else {
        return 0;
    };
    // ---- arms on a copy, against the body as it now is
    if !ask_blend(g, at, at, 1.0, false, at_b, n, tune) {
        return 0;
    }
    let (hh, from, caught, fix) = (&mut hs, &mut an.hd_from, &mut an.hd_caught, &mut an.hd_fix);
    let Some((live_r, r_r0)) = with_pose(g, at_b, |d, rig, _n, fk| {
        fk_run(rig, d, fk.rot, fk.pos);
        let (hl, hr) = (role(HAND_L), role(HAND_R));
        let (live_l, live_r, r_l0, r_r0) = (fk.at(hl), fk.at(hr), fk.rot(hl), fk.rot(hr));
        let (mut tgt_l, mut tgt_r);
        if grab {
            // round the outside of him and on: the left leads, the right follows
            let (u_l, u_r) = (smoothstep(0.0, 0.75, ug), smoothstep(0.2, 1.0, ug));
            if cr {
                // over his left shoulder beside the neck and round the front of the throat: keys that ride the man, from where the reach had the hand
                let off = match man {
                    Some(m) => [clamp(m[0], -0.3, 0.3), clamp(m[2] - tune.hold.dist, -0.12, 0.45)],
                    None => [0.0, 0.0],
                };
                let on_him = |p: [f64; 3]| [p[0] + off[0], p[1], p[2] + off[1]];
                let from = *from.get_or_insert(rd_hand.unwrap_or(live_l));
                tgt_l = curve3(u_l, &[(0.0, from), (tune.hold.catch.at[0], on_him(tune.hold.catch.via)), (tune.hold.catch.at[1], on_him(tune.hold.catch.over)), (1.0, on_him(hold_l))]);
            } else {
                tgt_l = curve3(u_l, &[(0.0, live_l), (0.4, tune.hold.via_l), (0.72, tune.hold.over_l), (1.0, hold_l)]);
            }
            tgt_r = curve3(u_r, &[(0.0, live_r), (0.55, tune.hold.knife.via), (1.0, hold_r)]);
        } else {
            tgt_l = hold_l;
            tgt_r = hold_r;
        }
        if k_on {
            // the kill: the left hand up over his mouth, then down across his chest and on down with him; the knife fist set, drawn across and out wide
            tgt_l = lerp3(&lerp3(&tgt_l, &tune.hold.kill.mouth, k_w), &lerp3(&tune.hold.kill.chest, &tune.hold.kill.collar, smoothstep(0.3, 1.0, k_a)), k_c);
            let drawn = if k_s > 0.0 { curve3(k_s, &[(0.0, tune.hold.kill.set), (0.4, tune.hold.kill.across), (1.0, tune.hold.kill.wide)]) } else { lerp3(&tgt_r, &tune.hold.kill.set, k_w) };
            tgt_r = lerp3(&drawn, &tune.hold.kill.low, smoothstep(0.3, 1.0, k_a));
        }
        // the left's fingers run on across the throat and hook back round it, palm to the neck
        let rot_hold_l = hand_rot(&fix_l, [-1.0, 0.12, -0.4], [0.1, 0.35, 1.0]);
        let u_lc = if cr { smoothstep(0.0, 0.75, ug) } else { 1.0 };
        let rot_reach = if cr { hand_rot(&fix_l, [-0.3, 0.1, 1.0], [0.75, 0.6, -0.1]) } else { rot_hold_l };
        let lv = if live { 1.0 } else { w };
        let rot_l = if cr {
            nlerp(&nlerp(&r_l0, &rot_reach, smoothstep(0.0, tune.hold.catch.arm_in, ug)), &rot_hold_l, smoothstep(tune.hold.catch.at[0], 0.95, u_lc))
        } else {
            nlerp(&r_l0, &rot_hold_l, e_l)
        };
        let (ual, fal) = (role(UPPER_ARM_L), role(FOREARM_L));
        if cr {
            // (the elbow under the wrist and close to him until the hand is round the front of the throat, then up onto his collarbone)
            let pole = lerp3(&[0.3, -0.92, 0.15], &[0.55, 0.25, 0.85], smoothstep(0.62, 1.0, u_lc));
            let a_in = smoothstep(0.0, tune.hold.catch.arm_in, ug);
            two_bone_ik(rig, d, fk.rot, fk.pos, ual, fal, hl, &tgt_l, Some(&pole), 0.95 * a_in, Some(&rot_l), false);
            *caught = true;
            *fix = Some(AnimHdFix { tgt: tgt_l, pole, pole_w: 0.95 * a_in, rot: nlerp(&rot_reach, &rot_hold_l, smoothstep(tune.hold.catch.at[0], 0.95, u_lc)), w: smoothstep(0.0, 0.1, ug) * lv, rw: a_in * lv });
        } else {
            if *caught && cs_on && !gun_v && held && live && !k_on && t < tune.hold.catch.settle {
                let f = 1.0 - smoothstep(0.0, tune.hold.catch.settle, t);
                *fix = Some(AnimHdFix { tgt: tgt_l, pole: [0.55, 0.25, 0.85], pole_w: 0.95, rot: rot_hold_l, w: f, rw: f });
            } else if !grab {
                *caught = false;
            }
            // elbow forward and a little left, up on his left collarbone
            let pole = if k_on { lerp3(&lerp3(&[0.55, 0.25, 0.85], &[0.9, 0.35, 0.3], k_w), &[0.85, -0.45, 0.3], k_c) } else { [0.55, 0.25, 0.85] };
            two_bone_ik(rig, d, fk.rot, fk.pos, ual, fal, hl, &tgt_l, Some(&pole), 0.95 * e_l, Some(&rot_l), false);
        }
        // fingers close as the hands arrive
        let open = 1.0 - if grab { smoothstep(0.5, 1.0, ug) } else { 1.0 };
        let cl = |x: f64| lerp(x, 0.08, open);
        hand(d, hh, 0, &[cl(0.5), cl(0.58), cl(0.64), cl(0.68)], lerp(0.55, 0.1, open), open * 0.4, tune);
        if xg < 1.0 {
            // ---- the knife fist: the blade kept pointing ahead round the outside of him, turned across only as it comes in over the shoulder
            let (uar, far) = (role(UPPER_ARM_R), role(FOREARM_R));
            let e_rk = if grab { smoothstep(0.5, 1.0, ug) } else { 1.0 };
            let rot_k = nlerp(&knife_rot(&fix_r, tune.hold.knife.blade, tune.hold.knife.back), &knife_rot(&fix_r, tune.hold.knife.off_blade, tune.hold.knife.off_back), kn);
            let rot_r = nlerp(&r_r0, &if k_s > 0.0 { nlerp(&rot_k, &knife_rot(&fix_r, tune.hold.kill.wide_blade, tune.hold.kill.wide_back), k_s) } else { rot_k }, e_rk);
            let pole_k = lerp3(&[-0.9, 0.15, -0.4], &[-0.8, -0.55, -0.3], kn);
            let base_r = lerp3(&tgt_r, &tune.hold.knife.via, back_k);
            let pole_r = if k_s > 0.0 { lerp3(&pole_k, &[-0.85, 0.5, -0.5], k_s) } else { pole_k };
            if s_arm {
                // THE DISARM: one path for the wrist, keyed in seconds: onto the gun in his hand, torn out and down to Sam's hip, let go, then up to its station
                let (take, out, drop, home, follow) = (tune.hold.strip.take, tune.hold.strip.out, tune.hold.strip.drop, tune.hold.strip.home, tune.hold.strip.follow);
                let m = add3(
                    match s_at {
                        Some(sa) => untwist(fk, tw_y, tw_p, sa),
                        None => [tune.hold.strip.meet[0], tune.hold.strip.meet[1], tune.hold.strip.meet[2] + tune.hold.dist],
                    },
                    tune.hold.strip.over,
                );
                let back = lerp(drop, home, 0.45);
                let p = curve3(
                    strip_in(s_t, take),
                    &[
                        (0.0, live_r),
                        (take * 0.55, [lerp(live_r[0], m[0], 0.5) - 0.07, lerp(live_r[1], m[1], 0.6), lerp(live_r[2], m[2], 0.45)]),
                        (take, m),
                        (out, add3(m, tune.hold.strip.wrench)),
                        (drop, tune.hold.strip.low),
                        (drop + follow, tune.hold.strip.low),
                        (max(back, drop + follow + 0.06), tune.hold.knife.via),
                        (home, base_r),
                    ],
                );
                let (u1, u2, u3, u4) = (smoothstep(take * 0.3, take, s_t), smoothstep(take, out, s_t), smoothstep(out, drop, s_t), smoothstep(drop + 0.08, home - 0.06, s_t));
                // over the top of the gun, fingers round it · rolled out, the back of the hand to the right · hanging at the thigh
                let (r_grip, r_tear, r_low) = (hand_rot(&fix_r, [0.3, -0.5, 0.8], [-0.25, 0.9, 0.3]), hand_rot(&fix_r, [-0.2, -0.8, 0.55], [-0.9, 0.25, 0.3]), hand_rot(&fix_r, [0.05, -1.0, 0.15], [-1.0, 0.0, 0.1]));
                let grip = smoothstep(take - 0.05, take + 0.02, s_t) * (1.0 - smoothstep(drop, drop + 0.07, s_t));
                let fist = smoothstep(tune.hold.strip.knife - 0.12, tune.hold.strip.knife, s_t);
                let cu = |x: f64| lerp(lerp(0.14, 0.7, grip), x, fist);
                let rot = nlerp(&nlerp(&nlerp(&nlerp(&r_r0, &r_grip, u1), &r_tear, u2), &r_low, u3), &rot_r, u4);
                two_bone_ik(rig, d, fk.rot, fk.pos, uar, far, hr, &p, Some(&lerp3(&[-0.95, -0.3, -0.2], &pole_r, u4)), lerp(0.95 * smoothstep(0.0, 0.1, s_t), 0.9 * max(e_r, 0.3), u4), Some(&rot), false);
                hand(d, hh, 1, &[cu(0.62), cu(0.68), cu(0.72), cu(0.76)], lerp(lerp(0.15, 0.5, grip), 0.6, fist), 0.25 * (1.0 - grip) * (1.0 - fist), tune);
            } else {
                // elbow out to the right and up, over the man's shoulder, for the throat; down and back once the fist is on the shoulder
                two_bone_ik(rig, d, fk.rot, fk.pos, uar, far, hr, &base_r, Some(&pole_r), 0.9 * max(e_r, 0.3), Some(&rot_r), false);
                hand(d, hh, 1, &[0.62, 0.68, 0.72, 0.76], 0.6, 0.0, tune);
            }
        }
        (live_r, r_r0)
    }) else {
        return 0;
    };
    if xg > 0.0 {
        // ---- the gun hand: the right arm re-seeded from the aim clip, one rigid turn of that grip onto the wanted bore, the wrist to its station;
        //      on a scratch copy while the knife fist still has a share of the arm
        let p = if xg < 1.0 { at_c } else { at_b };
        if p != at_b && !ask_blend(g, at_b, at_b, 1.0, false, p, n, tune) {
            return 0;
        }
        // (the clavicle stays the body's)
        ask_mask(&[(role(SHOULDER_R), 1.0, true), (role(SHOULDER_R), 0.0, false)]);
        if !ask_blend(g, p, at_aim, 1.0, true, p, n, tune) {
            return 0;
        }
        with_pose(g, p, |d, rig, _n, fk| {
            fk_run(rig, d, fk.rot, fk.pos);
            let hr = role(HAND_R);
            let (p_r0, r_aim) = (fk.at(hr), fk.rot(hr));
            // the clip's bore as posed on this body, and where the carry underneath actually has it pointing
            let g0 = normalize(sub3(fk.at(role(MIDDLE1_R)), p_r0));
            let g_live = normalize(q_rotate(&q_mul(&r_r0, &conj(r_aim)), &g0));
            let (out, pitch) = (clamp(gun_out, 0.0, 1.0) * (1.0 - wnd), gun_pitch);
            // recoil: up in ~35 ms, back down over ~0.2 s
            let kick = if shoot_t >= 0.0 { bump(shoot_t, 0.0, 0.035, 0.06, 0.24) } else { 0.0 };
            let rest = lerp3(&lerp3(&tune.hold.gun.hand, &tune.hold.gun.out_hand, out), &tune.hold.gun.via_b, back_g);
            let bore_rest = normalize(lerp3(&lerp3(&tune.hold.gun.bore, &[0.0, sin(pitch), cos(pitch)], out), &tune.hold.gun.via_bore, back_g));
            let (mut hand_at, mut bore);
            if grab {
                let u_r = smoothstep(0.1, 1.0, ug);
                hand_at = match s_at {
                    // (the disarm, the pistol hold's: the frame comes down on the man's gun wrist on its way round and up to his head)
                    Some(sa) if gun_v => {
                        let (rap, gs, take) = (add3(untwist(fk, tw_y, tw_p, sa), tune.hold.strip.rap), tune.hold.secs.grab, tune.hold.strip.take);
                        curve3(strip_in(t, take), &[(0.0, live_r), (take, rap), (lerp(take, gs, 0.35), tune.hold.gun.via_a), (lerp(take, gs, 0.7), tune.hold.gun.via_b), (gs, rest)])
                    }
                    _ => curve3(u_r, &[(0.0, live_r), (0.38, tune.hold.gun.via_a), (0.7, tune.hold.gun.via_b), (1.0, rest)]),
                };
                bore = normalize(curve3(u_r, &[(0.0, g_live), (0.5, tune.hold.gun.via_bore), (1.0, bore_rest)]));
            } else if whip {
                hand_at = lerp3(&lerp3(&lerp3(&rest, &tune.hold.gun.wind_hand, wnd), &tune.hold.gun.hit_hand, stk), &tune.hold.gun.thru_hand, thru);
                bore = normalize(lerp3(&lerp3(&lerp3(&bore_rest, &tune.hold.gun.wind_bore, wnd), &tune.hold.gun.hit_bore, stk), &tune.hold.gun.bore, thru));
            } else {
                hand_at = rest;
                bore = bore_rest;
            }
            if kick > 0.0 {
                // the muzzle flips up about the wrist and the hand comes back along the bore
                let up_k = sin(tune.hold.gun.kick_pitch * DEGR * kick);
                bore = normalize([bore[0], bore[1] + up_k, bore[2]]);
                hand_at = mad3(add3(hand_at, [0.0, 0.01 * kick, 0.0]), bore, -tune.hold.gun.kick_back * kick);
            }
            let turned = grip_turn(&g0, &r_aim, &bore, &AY, &grip_up);
            // (the first frames of the reach: still mostly the carry's own hand)
            let e_g = if grab { smoothstep(0.05, 0.6, ug) } else { 1.0 };
            let rot_r = nlerp(&r_r0, &q_mul(&turned, &r_aim), e_g);
            // elbow out to the right and a little down; up and back as it cocks
            let pole = lerp3(&[-1.0, -0.35, -0.45], &[-0.7, 0.2, -0.7], wnd * (1.0 - stk));
            two_bone_ik(rig, d, fk.rot, fk.pos, role(UPPER_ARM_R), role(FOREARM_R), hr, &hand_at, Some(&pole), 0.9 * max(e_g, 0.3), Some(&rot_r), false);
        });
        if p != at_b {
            ask_mask(&[(role(SHOULDER_R), 1.0, true)]);
            if !ask_blend(g, at_b, p, xg, true, at_b, n, tune) {
                return 0;
            }
        }
    }
    arms_mask();
    if !ask_blend(g, at, at_b, w, true, at, n, tune) {
        return 0;
    }
    if cr && live && w < 1.0 {
        // (the left arm is the hold's own before the layer's weight has come in: its path starts where the reach had the hand)
        let w_l = max(w, smoothstep(0.0, 0.1, ug));
        if w_l > w {
            ask_mask(&[(role(SHOULDER_L), 1.0, true)]);
            if !ask_blend(g, at, at_b, (w_l - w) / (1.0 - w), true, at, n, tune) {
                return 0;
            }
        }
    }
    if strip.is_some() && s_t < tune.hold.strip.out && w < 1.0 {
        // (the disarm: the hand is on the gun at `take`, before the layer's own weight has come in)
        let w_s = max(w, smoothstep(0.0, 0.8 * tune.hold.strip.take, s_t));
        if w_s > w {
            ask_mask(&[(role(SHOULDER_R), 1.0, true)]);
            if !ask_blend(g, at, at_b, (w_s - w) / (1.0 - w), true, at, n, tune) {
                return 0;
            }
        }
    }
    hands_to(&hs, an);
    RAN | RAN_ALL
}
/// CharacterAnimator.catchFix's arm: the catch's left hand put back where the hold's own arm had it, after the layers that still move his trunk.
pub fn catch_arm(g: usize, an: &Anim, x: &AnimHdFix) -> u32 {
    with_pose(g, an.p_a.at as usize, |d, rig, n, fk| {
        if !cast(n, &ARM_ROLES) {
            return 0;
        }
        fk_run(rig, d, fk.rot, fk.pos);
        let hl = role(HAND_L);
        let rot = nlerp(&fk.rot(hl), &x.rot, x.rw);
        two_bone_ik(rig, d, fk.rot, fk.pos, role(UPPER_ARM_L), role(FOREARM_L), hl, &lerp3(&fk.at(hl), &x.tgt, x.w), Some(&x.pole), x.pole_w, Some(&rot), false);
        RAN | RAN_ALL
    })
    .unwrap_or(0)
}
/// what the held man's two arms do: each wrist's spot, turn and elbow pole, and each hand's fingers
#[derive(Clone, Copy)]
struct ArmSet {
    tgt: [[f64; 3]; 2],
    rot: [[f64; 4]; 2],
    pole: [[f64; 3]; 2],
    curl: [[f64; 4]; 2],
    thumb: [f64; 2],
    spread: [f64; 2],
}
/// The held man: the pelvis sunk, hauled, sat and lowered over feet put back where they stood (or stepped after him), the spine arched onto the arm with
/// his struggle, his breathing and the choke's fight on it, the head pulled back and lolling; then both arms by IK on a copy (the arm hold's set, the
/// gun hold's, or the two blended for the beat of a change).
pub fn held_layer(g: usize, an: &mut Anim, w: f64, dt: f64, h: &HeldPose, kill_on: bool, beats: bool, tune: &Knobs) -> u32 {
    let (at, at_b, _) = poses(an);
    let (gun, bare, live, ph, t) = (h.variant == Some(HoldVariant::Gun), h.bare.unwrap_or(false), an.held_pose.is_some(), h.phase, h.t);
    let (grab, held, choke, whip, kill) = (ph == HoldPhase::Grab, ph == HoldPhase::Held, ph == HoldPhase::Choke, ph == HoldPhase::Whip, ph == HoldPhase::Kill);
    let (ag, low, haul, yank, pull, shot_in, armed) = (clamp(h.agitation, 0.0, 1.0), h.low.unwrap_or(0.0), h.haul.unwrap_or(false), h.yank, h.pull.unwrap_or(0.0), h.shot.filter(|s| *s >= 0.0), h.armed.unwrap_or(false));
    let (tt, strip, fix_l, fix_r) = (an.idle_t, an.held_pose.and_then(|h| h.strip).map(|s| s.t), an.hand_fix[0], an.hand_fix[1]);
    // (read before anything is posed: an answer of 0 leaves the pose as it was handed)
    let Some(mut hs) = hands_of(g, an) else {
        return 0;
    };
    let tau = 2.0 * PI;
    // 0 = the arm at his throat ↔ 1 = the gun at his head
    let gv = clamp(an.he_gv.x, 0.0, 1.0);
    let ug = if grab { clamp(t / tune.hold.secs.grab, 0.0, 1.0) } else { 1.0 };
    spring_step(&mut an.he_ck, if choke { clamp(t / tune.hold.secs.choke, 0.0, 1.0) } else { 0.0 }, if choke { 30.0 } else { 9.0 }, dt);
    let ck = clamp(an.he_ck.x, 0.0, 1.0);
    // going: the arch and the struggle give way, the head lolls, the hands fall
    let limp = smoothstep(0.62, 0.95, ck);
    let e_in = smoothstep(0.05, 0.6, ug);
    let cs_on = true && bare;
    let cr = cs_on && !gun && grab;
    let jl = if cr { bump(t, tune.hold.catch.jolt.t[0], tune.hold.catch.jolt.t[1], tune.hold.catch.jolt.t[2], tune.hold.catch.jolt.t[3]) * w } else { 0.0 };
    let hb = match yank {
        Some(y) if true && cs_on => bump(y, tune.hold.hit.t[0], tune.hold.hit.t[1], tune.hold.hit.t[2], tune.hold.hit.t[3]) * (1.0 - gv) * w,
        _ => 0.0,
    };
    let shot = match shot_in {
        Some(s) => bump(s, 0.0, 0.05, 0.12, tune.hold.gun.duck.secs) * w,
        None => 0.0,
    };
    let knock = if whip { smoothstep(tune.hold.secs.whip - 0.1, tune.hold.secs.whip, t) * w } else { 0.0 };
    let kk = kill_ease(&mut an.he_kc, kill_on && kill && !gun, t, dt, tune);
    let k_on = kk > 1e-4;
    let hit = tune.hold.kill.hit;
    let k_w = if k_on { smoothstep(0.0, tune.hold.kill.wind, kk) } else { 0.0 };
    let k_j = if k_on { bump(kk, hit - 0.03, hit + 0.02, hit + 0.08, hit + 0.24) } else { 0.0 };
    let k_t = if k_on { smoothstep(hit, hit + 0.14, kk) } else { 0.0 };
    let k_f = if k_on { smoothstep(0.58, 1.0, kk) } else { 0.0 };
    let k_l = if k_on { smoothstep(hit + 0.12, 0.9, kk) } else { 0.0 };
    let k_g = if k_on {
        let q = clamp((kk - tune.hold.kill.go) / (1.0 - tune.hold.kill.go), 0.0, 1.0);
        q * q * w
    } else {
        0.0
    };
    // ---- restlessness, and now and then a tug at the arm
    let amp = (0.35 + 0.65 * ag) * (1.0 - limp) * (if grab { 1.4 } else { 1.0 }) * lerp(1.0, 0.55, gv) * w * (if k_on { 1.0 - k_l } else { 1.0 });
    let period = 3.0 + 3.0 * (0.5 + 0.5 * sin(floor(tt / 4.5) * 12.9898));
    let jerk = bump(tt % period, 0.0, 0.06, 0.14, 0.24) * (1.0 - limp) * w * (0.5 + 0.5 * ag) * (if k_on { 1.0 - max(k_w, k_l) } else { 1.0 });
    // ---- the choke's fight: an envelope on the eased choke amount, three sines off his own clock
    let fgt = smoothstep(0.0, tune.hold.struggle.rise, ck) * (1.0 - smoothstep(tune.hold.struggle.fade_from, tune.hold.struggle.fade_to, ck)) * (1.0 - gv) * w;
    let (s1, s2, s3) = (sin(tt * tau * tune.hold.struggle.hz[0] + 0.4), sin(tt * tau * tune.hold.struggle.hz[1] + 1.3), sin(tt * tau * tune.hold.struggle.hz[2] + 2.1));
    let roll = (4.0 * sin(tt * tau * 0.7) * amp + 5.0 * jerk + tune.hold.struggle.roll * s3 * fgt) * DEGR;
    let yaw_s = (3.0 * sin(tt * tau * 0.45 + 1.0) * amp - 7.0 * jerk + tune.hold.struggle.yaw * (0.65 * s1 + 0.35 * s2) * fgt) * DEGR;
    // ---- hard breathing while he is held
    let br = (0.5 - 0.5 * cos(tt * tau * tune.hold.breath.hz)) * (1.0 - limp) * (1.0 - 0.7 * fgt) * lerp(1.0, 0.6, gv) * w * e_in * (if k_on { 1.0 - k_l } else { 1.0 });
    let lv = if live { 1.0 } else { w };
    let hu = haul_ease(&mut an.he_hu, haul && choke, smoothstep(tune.hold.haul.in_[0], tune.hold.haul.in_[1], ck) * (1.0 - smoothstep(tune.hold.haul.out[0], tune.hold.haul.out[1], ck)), tune.hold.haul.down, dt) * (1.0 - gv) * lv;
    let hauled = hu > 1e-4;
    let drop = (lerp(tune.hold.sag[0], tune.hold.sag[1], ck) + 0.02 * shot + tune.hold.struggle.dip * (0.5 + 0.5 * sin(tt * tau * 1.3)) * fgt) * w * e_in + tune.hold.catch.jolt.dip * jl;
    an.he_sw = if bare && live && held { sin(t / tune.hold.sway.secs * 2.0 * PI) * smoothstep(0.0, tune.hold.sway.in_, t) } else { damp(an.he_sw, 0.0, tune.hold.sway.out, dt) };
    let sw = an.he_sw * (1.0 - gv) * (1.0 - limp) * w;
    spring_step(&mut an.he_lk, if choke { clamp(t / tune.hold.secs.choke, 0.0, 1.0) } else { 0.0 }, if choke { 30.0 } else { 9.0 }, dt);
    let lk = clamp(an.he_lk.x, 0.0, 1.0);
    let lo0 = lower_ease(&mut an.he_lo, if low > 0.0 { clamp(low, 0.0, 1.0) * smoothstep(lower_k(beats, tune.hold.beats.lower.from, tune.hold.lower.from), lower_k(beats, tune.hold.beats.lower.by, tune.hold.lower.by), lk) } else { 0.0 }, lower_k(beats, tune.hold.beats.lower.up, tune.hold.lower.up), dt);
    let lo = lo0 * (1.0 - gv) * lv;
    let b_off = lower_ease(&mut an.he_bo, if beats { smoothstep(tune.hold.beats.off[0], tune.hold.beats.off[1], lk) } else { 0.0 }, tune.hold.beats.back, dt);
    let b_in = lower_ease(&mut an.he_bc, if beats { smoothstep(tune.hold.beats.clamp[0], tune.hold.beats.clamp[1], lk) } else { 0.0 }, tune.hold.beats.back, dt);
    let tip_u = if beats { smoothstep(tune.hold.beats.tip_by[0], tune.hold.beats.tip_by[1], lk) } else { 0.0 };
    let b_tip = tip_u * (1.0 - limp) * (1.0 - gv);
    // his arms hang for as long as he is down
    let limp_a = max(max(max(limp, lo0), k_f), b_off);
    let sit_in = if grab { smoothstep(0.55, 1.0, ug) } else { 1.0 };
    let sit = if cs_on { sit_in * (1.0 - gv) * (1.0 - clamp(lo0, 0.0, 1.0)) * (1.0 - clamp(hu, 0.0, 1.0)) * lv } else { 0.0 };
    let off = sit * (1.0 - limp);
    let sit_f = if cs_on { sit_in * (1.0 - gv) * lv } else { 0.0 };
    let foot = |feet0: f64, feet1: f64, heels: f64| -> [f64; 3] {
        if !cs_on {
            return [0.0, 0.0, 0.0];
        }
        let p = pull * (1.0 - limp) * (1.0 - gv);
        let e = match yank {
            Some(y) if live => smoothstep(feet0, feet1, y),
            _ => 1.0,
        };
        let (lag, rest) = (p * (1.0 - e), heels * sit_f);
        [0.0, if p > 1e-4 { tune.hold.catch.lift * sin(PI * e) * min(1.0, p / 0.1) } else { 0.0 }, max(lag, rest)]
    };
    let (off_l, off_r) = (foot(tune.hold.catch.feet[0][0], tune.hold.catch.feet[0][1], tune.hold.catch.heels[0]), foot(tune.hold.catch.feet[1][0], tune.hold.catch.feet[1][1], tune.hold.catch.heels[1]));
    let off_on = off_l[2] > 1e-4 || off_r[2] > 1e-4;
    let arch0 = (if k_on { lerp(lerp(lerp(tune.hold.arch, 2.0, limp), tune.hold.kill.arch, k_w), 2.0, k_l) } else { lerp(tune.hold.arch, 2.0, limp) }) * DEGR * w * e_in + tune.hold.gun.duck.pitch * DEGR * 0.5 * shot;
    let yk_h = match yank {
        Some(y) if live => bump(y, 0.0, tune.hold.yank.in_, tune.hold.yank.in_ + tune.hold.yank.hold, tune.hold.yank.in_ + tune.hold.yank.hold + tune.hold.yank.out),
        _ => 0.0,
    };
    let arch_b = if bare { (tune.hold.bare_arch * (1.0 - limp) * (1.0 - gv) * e_in + tune.hold.yank.arch * yk_h) * DEGR * w } else { 0.0 };
    let arch = ((if hauled { arch0 + tune.hold.haul.arch * hu * DEGR } else { arch0 }) + arch_b) * (1.0 - jl) + tune.hold.catch.jolt.fwd * jl * DEGR;
    let heave = -tune.hold.breath.pitch * br * DEGR;
    let clav = (tune.hold.clav_up * e_in * (1.0 - limp) + tune.hold.gun.duck.hunch * shot + tune.hold.breath.clav * br) * DEGR * w + tune.hold.catch.jolt.clav * jl * DEGR;
    let (aside, chin_up) = (lerp(tune.hold.aside, tune.hold.gun.aside, gv), lerp(tune.hold.chin_up, tune.hold.gun.chin_up, gv));
    let chin0 = if k_on { lerp(lerp(lerp(chin_up, tune.hold.loll, limp), tune.hold.kill.chin, k_w), tune.hold.loll, k_l) + tune.hold.kill.jolt[1] * k_j } else { lerp(chin_up, tune.hold.loll, limp) };
    let chin1 = chin0 * DEGR * w * e_in - drop * 40.0 * DEGR + (tune.hold.gun.duck.pitch * shot + tune.hold.gun.knock.pitch * knock + tune.hold.struggle.head * 0.35 * s3 * fgt) * DEGR - heave;
    let hr = (tune.hold.loll_roll * limp - aside * (1.0 - limp) * e_in * (if k_on { 1.0 - k_w } else { 1.0 }) * (1.0 - tip_u)
        + if k_on { tune.hold.loll_roll * k_l } else { 0.0 }
        + if beats { tune.hold.beats.tip * b_tip } else { 0.0 }
        + 3.0 * sin(tt * 3.1) * amp
        + tune.hold.struggle.roll * 0.7 * s1 * fgt
        - tune.hold.gun.duck.roll * shot
        - tune.hold.gun.knock.roll * knock)
        * DEGR
        * w;
    let hyw = (6.0 * sin(tt * 1.7 + 2.0) * amp + 9.0 * jerk + tune.hold.struggle.head * (0.6 * s2 + 0.4 * s3) * fgt + tune.hold.gun.away_yaw * e_in * gv + tune.hold.gun.duck.yaw * shot + tune.hold.gun.knock.yaw * knock
        + if k_on { tune.hold.kill.jolt[0] * k_j * w } else { 0.0 })
        * DEGR;
    let hg_y = if k_g > 1e-4 { lower_k(beats, tune.hold.beats.lower.hang[0], tune.hold.lower.hang[0]) * lo + tune.hold.kill.hang[0] * k_g } else { lower_k(beats, tune.hold.beats.lower.hang[0], tune.hold.lower.hang[0]) * lo };
    let hg_z = if k_g > 1e-4 { lower_k(beats, tune.hold.beats.lower.hang[1], tune.hold.lower.hang[1]) * lo + tune.hold.kill.hang[1] * k_g } else { lower_k(beats, tune.hold.beats.lower.hang[1], tune.hold.lower.hang[1]) * lo };
    let dn = lo > 1e-4 || k_g > 1e-4;
    // (his hands hang from where his shoulders now are)
    let (h0, h1) = (tune.hold.hang[0], tune.hold.hang[1]);
    let (hang_l, hang_r) = if dn { ([h0[0], h0[1] - hg_y, h0[2] + hg_z], [h1[0], h1[1] - hg_y, h1[2] + hg_z]) } else { (h0, h1) };
    // ---- the arms' sets: the arm hold's (gv 0) and / or the gun hold's (gv 1)
    let arm_hold = || -> ArmSet {
        let c_in = max(if beats { b_in } else { smoothstep(0.04, 0.3, ck) }, k_w);
        let c_r = if bare { max(c_in, tune.hold.bare_grip) } else { c_in };
        let cw_l = if k_on { lerp3(&tune.hold.claw_l, &lerp3(&tune.hold.kill.reach[0], &tune.hold.kill.clutch[0], k_t), k_w) } else { tune.hold.claw_l };
        let cw_r = if k_on { lerp3(&tune.hold.claw_r, &lerp3(&tune.hold.kill.reach[1], &tune.hold.kill.clutch[1], k_t), k_w) } else { tune.hold.claw_r };
        // (the haul: his hands go with the forearm they are on)
        let (cl_l, cl_r) = if hauled || sit > 1e-4 {
            let hv = [0.0, tune.hold.haul.sam_arm[0] * hu - tune.hold.catch.sink * sit, -tune.hold.haul.sam_arm[1] * hu];
            (add3(cw_l, hv), add3(cw_r, hv))
        } else {
            (cw_l, cw_r)
        };
        let claw = |o: f64| [tune.hold.struggle.claw * 0.6 * sin(tt * tau * 1.7 + o) * fgt, tune.hold.struggle.claw * sin(tt * tau * 2.6 + o) * fgt, tune.hold.struggle.claw * 0.4 * sin(tt * tau * 2.6 + o + 1.0) * fgt];
        let snk = tune.hold.catch.sink * sit;
        let rest = [tune.hold.rest_l[0], tune.hold.rest_l[1] - 0.03 * jerk - snk, tune.hold.rest_l[2] - 0.02 * jerk];
        let up = [tune.hold.raised_r[0] + 0.01 * sin(tt * tau * 1.1) * amp, tune.hold.raised_r[1] + 0.012 * sin(tt * tau * 0.9 + 1.0) * amp + 0.03 * jerk - snk, tune.hold.raised_r[2] + 0.01 * cos(tt * tau * 0.6) * amp];
        // clawing: the fingers working
        let cl = lerp(0.62, 0.82, 0.5 + 0.5 * sin(tt * tau * 2.6)) * fgt + 0.65 * (1.0 - fgt);
        let pole_l = lerp3(&[1.0, -0.9, 0.3], &[1.0, -0.5, 0.55], c_in);
        let pole_r = lerp3(&[-0.8, -1.0, 0.15], &[-1.0, -0.6, 0.5], c_r);
        let cu_l = |x: f64| lerp(lerp(x, cl, c_in), 0.2, limp_a);
        let cu_r = |x: f64| lerp(lerp(x, cl, c_r), 0.25, limp_a);
        let fi = |x: f64| lerp(x, 0.35, limp_a);
        ArmSet {
            tgt: [lerp3(&lerp3(&rest, &add3(cl_l, claw(0.0)), c_in), &hang_l, limp_a), lerp3(&lerp3(&up, &add3(cl_r, claw(PI)), c_r), &hang_r, limp_a)],
            rot: [
                nlerp(&hand_rot(&fix_l, [-0.25, -0.8, 0.5], [0.0, 0.55, 0.8]), &hand_rot(&fix_l, [-0.5, 0.6, -0.6], [0.2, 0.5, 0.85]), c_in),
                nlerp(&hand_rot(&fix_r, [-0.2, 1.0, 0.1], [-0.15, 0.1, -1.0]), &hand_rot(&fix_r, [0.5, 0.55, -0.6], [-0.2, 0.5, 0.85]), c_r),
            ],
            pole: [if bare { lerp3(&pole_l, &[0.45, -1.0, 0.35], tune.hold.bare_elbow) } else { pole_l }, if bare { lerp3(&pole_r, &[-0.45, -1.0, 0.35], tune.hold.bare_elbow) } else { pole_r }],
            curl: [[cu_l(0.25), cu_l(0.3), cu_l(0.32), cu_l(0.35)], if armed { [fi(0.75), fi(0.8), fi(0.8), fi(0.8)] } else { [cu_r(0.08), cu_r(0.1), cu_r(0.12), cu_r(0.18)] }],
            thumb: [lerp(0.15, 0.3, c_in), if armed { 0.6 } else { lerp(0.05, 0.3, c_r) }],
            spread: [0.0, if armed { 0.0 } else { 0.7 * (1.0 - c_r) }],
        }
    };
    let gun_hold = || -> ArmSet {
        // both hands half up in front of the shoulders, palms forward, trembling, jerked in when a round goes past
        let trem = 0.012 * sin(tt * tau * 1.1) * amp;
        let (u0, u1) = (tune.hold.gun.hands_up[0], tune.hold.gun.hands_up[1]);
        let (open_l, fist) = (max(limp_a, 0.85), 1.0 - limp_a * 0.7);
        ArmSet {
            tgt: [
                lerp3(&[u0[0] - 0.03 * shot, u0[1] + trem + 0.05 * shot + 0.03 * jerk, u0[2] - 0.04 * shot], &h0, limp_a),
                lerp3(&[u1[0] + 0.03 * shot + 0.03 * sin(tt * 0.8) * amp, u1[1] - trem + 0.05 * shot + 0.04 * jerk, u1[2] - 0.04 * shot + 0.02 * cos(tt * 0.6) * amp], &h1, limp_a),
            ],
            rot: [hand_rot(&fix_l, [0.15, 1.0, 0.2], [0.1, 0.15, -1.0]), hand_rot(&fix_r, [-0.15, 1.0, 0.2], [-0.1, 0.15, -1.0])],
            pole: [[0.8, -1.0, 0.2], [-0.8, -1.0, 0.2]],
            curl: [[lerp(0.6, 0.15, open_l), lerp(0.7, 0.2, open_l), lerp(0.75, 0.25, open_l), lerp(0.8, 0.3, open_l)], [0.2 + 0.55 * fist, 0.25 + 0.55 * fist, 0.3 + 0.55 * fist, 0.3 + 0.55 * fist]],
            thumb: [lerp(0.3, 0.15, open_l), 0.2 + 0.4 * fist],
            spread: [0.35, 0.0],
        }
    };
    let z = if gv < 1.0 && gv > 0.0 {
        let (x, y) = (arm_hold(), gun_hold());
        let mut z = x;
        for s in 0..2 {
            z.tgt[s] = lerp3(&x.tgt[s], &y.tgt[s], gv);
            z.rot[s] = nlerp(&x.rot[s], &y.rot[s], gv);
            z.pole[s] = lerp3(&x.pole[s], &y.pole[s], gv);
            for i in 0..4 {
                z.curl[s][i] = lerp(x.curl[s][i], y.curl[s][i], gv);
            }
            z.thumb[s] = lerp(x.thumb[s], y.thumb[s], gv);
            z.spread[s] = lerp(x.spread[s], y.spread[s], gv);
        }
        z
    } else if gv < 1.0 {
        arm_hold()
    } else {
        gun_hold()
    };
    let s_t = strip.unwrap_or(0.0);
    let s_on = strip.is_some() && s_t < tune.hold.strip.home;
    let e_a = if grab { smoothstep(0.1, 0.75, ug) } else { 1.0 };
    let kept = lo0 > 1e-4 || k_f > 1e-4 || b_off > 1e-4;
    let (he_span, he_hr) = (&mut an.he_span, &mut an.he_hr);
    // ---- body: feet noted first (put back after the pelvis sinks), then arch / hunch / head
    let Some(Some(n)) = with_pose(g, at, |d, rig, n, fk| {
        if !(cast(n, &ARM_ROLES) && cast(n, &LEG_ROLES) && cast(n, &[SPINE1, SPINE2, NECK, HEAD])) {
            return None;
        }
        let (hips, sp1, sp2, sp3) = (role(HIPS), role(SPINE1), role(SPINE2), role(SPINE3));
        fk_run(rig, d, fk.rot, fk.pos);
        let (f_l, f_r, r_l, r_r) = (fk.at(role(FOOT_L)), fk.at(role(FOOT_R)), fk.rot(role(FOOT_L)), fk.rot(role(FOOT_R)));
        if drop > 1e-4 {
            shift(d, rig, hips, &[0.0, -drop, 0.0]);
            turn(d, rig, hips, q_pitch(drop * 40.0 * DEGR));
        }
        if hauled {
            shift(d, rig, hips, &[0.0, tune.hold.haul.lift * hu, -tune.hold.haul.back * hu]);
            turn(d, rig, hips, q_pitch(tune.hold.haul.recline * hu * DEGR));
        }
        if bare {
            // (his head tips away from Sam's)
            turn(d, rig, role(NECK), q_roll(tune.hold.apart.tip * DEGR * w * (1.0 - gv) * (1.0 - limp) * (1.0 - clamp(hu, 0.0, 1.0)) * (if grab { smoothstep(0.3, 1.0, t / tune.hold.secs.grab) } else { 1.0 })));
        }
        if abs(sw) > 1e-4 {
            shift(d, rig, hips, &[tune.hold.sway.m * tune.hold.sway.his * sw, 0.0, 0.0]);
            turn(d, rig, sp2, q_yaw(tune.hold.sway.twist * (0.6 + 0.8 * ag) * sw * DEGR));
        }
        // hauled off his feet's line: sat down into the arm, the hips ahead of where they stood with the trunk reclined about his own neck
        let mut hit_rec = 0.0;
        if sit > 1e-4 || hb > 1e-4 {
            if falsy(*he_span) {
                fk_run(rig, d, fk.rot, fk.pos);
                *he_span = max(0.2, fk.at(role(NECK))[1] - fk.at(hips)[1]);
            }
            let (span, ahead) = (*he_span, tune.hold.catch.hips * off + tune.hold.hit.hips * hb);
            let (rec, rec0) = (asin(clamp(ahead / span, 0.0, 0.6)), asin(clamp(tune.hold.catch.hips * off / span, 0.0, 0.6)));
            hit_rec = rec - rec0;
            shift(d, rig, hips, &[0.0, -tune.hold.catch.sink * sit - tune.hold.hit.dip * hb + span * (cos(rec0) - cos(rec)), ahead]);
            turn(d, rig, hips, q_pitch(-rec));
        }
        if lo > 1e-4 {
            shift(d, rig, hips, &[0.0, -lower_k(beats, tune.hold.beats.lower.sink, tune.hold.lower.sink) * lo, lower_k(beats, tune.hold.beats.lower.fwd, tune.hold.lower.fwd) * lo]);
            turn(d, rig, hips, q_pitch(lower_k(beats, tune.hold.beats.lower.hip, tune.hold.lower.hip) * lo * DEGR));
        }
        if k_g > 1e-4 {
            shift(d, rig, hips, &[0.0, -tune.hold.kill.sink * k_g, tune.hold.kill.fwd * k_g]);
            turn(d, rig, hips, q_pitch(tune.hold.kill.hip * k_g * DEGR));
        }
        turn(d, rig, sp1, q_mul(&q_yaw(yaw_s * 0.3), &q_pitch(arch * 0.3)));
        turn(d, rig, sp2, q_mul(&q_yaw(yaw_s * 0.4), &q_mul(&q_roll(roll * 0.4), &q_pitch(arch * 0.35 + heave * 0.4))));
        turn(d, rig, sp3, q_mul(&q_yaw(yaw_s * 0.3), &q_mul(&q_roll(roll * 0.6), &q_pitch(arch * 0.35 + heave * 0.6))));
        // (the shoulders working against the arm, out of step)
        turn(d, rig, role(SHOULDER_L), q_roll(clav + 3.0 * s1 * fgt * DEGR));
        turn(d, rig, role(SHOULDER_R), q_roll(-clav + 3.0 * s2 * fgt * DEGR));
        let chin = (if hauled { chin1 + tune.hold.haul.chin * hu * DEGR } else { chin1 }) * (1.0 - jl) + tune.hold.catch.jolt.head * jl * DEGR + tune.hold.hit.head * hit_rec;
        turn(d, rig, role(NECK), q_mul(&q_yaw(hyw * 0.4), &q_mul(&q_roll(hr * 0.4), &q_pitch(chin * 0.45))));
        turn(d, rig, role(HEAD), q_mul(&q_yaw(hyw * 0.6), &q_mul(&q_roll(hr * 0.6), &q_pitch(chin * 0.55))));
        if lo > 1e-4 {
            let f = lower_k(beats, tune.hold.beats.lower.fold, tune.hold.lower.fold) * lo * DEGR;
            turn(d, rig, sp1, q_pitch(f * 0.3));
            turn(d, rig, sp2, q_pitch(f * 0.35));
            turn(d, rig, sp3, q_pitch(f * 0.35));
        }
        if k_g > 1e-4 {
            let f = tune.hold.kill.fold * k_g * DEGR;
            turn(d, rig, sp1, q_pitch(f * 0.3));
            turn(d, rig, sp2, q_pitch(f * 0.35));
            turn(d, rig, sp3, q_pitch(f * 0.35));
        }
        let (thl, shl, ftl, thr, shr, ftr) = (role(THIGH_L), role(SHIN_L), role(FOOT_L), role(THIGH_R), role(SHIN_R), role(FOOT_R));
        if hauled {
            // off his feet: they come up and forward off where they stood, toes down, kicking with the fight
            let (toe, kf, ku) = (q_pitch(tune.hold.haul.toes * hu * DEGR), tune.hold.haul.kick[0] * fgt * hu, tune.hold.haul.kick[1] * fgt * hu);
            let t_l = [f_l[0], f_l[1] + tune.hold.haul.feet[0] * hu + ku * (0.5 + 0.5 * s2) + off_l[1], f_l[2] + tune.hold.haul.feet[1] * hu + kf * s1 + off_l[2]];
            let t_r = [f_r[0], f_r[1] + tune.hold.haul.feet[0] * hu + ku * (0.5 - 0.5 * s2) + off_r[1], f_r[2] + tune.hold.haul.feet[1] * hu - kf * s1 + off_r[2]];
            fk_run(rig, d, fk.rot, fk.pos);
            two_bone_ik(rig, d, fk.rot, fk.pos, thl, shl, ftl, &t_l, Some(&knee_pole(&r_l)), 0.6, Some(&q_mul(&toe, &r_l)), false);
            two_bone_ik(rig, d, fk.rot, fk.pos, thr, shr, ftr, &t_r, Some(&knee_pole(&r_r)), 0.6, Some(&q_mul(&toe, &r_r)), true);
        } else if off_on {
            fk_run(rig, d, fk.rot, fk.pos);
            two_bone_ik(rig, d, fk.rot, fk.pos, thl, shl, ftl, &add3(f_l, off_l), Some(&knee_pole(&r_l)), 0.6, Some(&r_l), false);
            two_bone_ik(rig, d, fk.rot, fk.pos, thr, shr, ftr, &add3(f_r, off_r), Some(&knee_pole(&r_r)), 0.6, Some(&r_r), true);
        } else if drop > 1e-4 || lo > 1e-4 || k_g > 1e-4 || hb > 1e-4 {
            fk_run(rig, d, fk.rot, fk.pos);
            two_bone_ik(rig, d, fk.rot, fk.pos, thl, shl, ftl, &f_l, Some(&knee_pole(&r_l)), 0.6, Some(&r_l), false);
            two_bone_ik(rig, d, fk.rot, fk.pos, thr, shr, ftr, &f_r, Some(&knee_pole(&r_r)), 0.6, Some(&r_r), true);
        }
        Some(n)
    }) else {
        return 0;
    };
    if !ask_blend(g, at, at, 1.0, false, at_b, n, tune) {
        return 0;
    }
    let hh = &mut hs;
    if with_pose(g, at_b, |d, rig, _n, fk| {
        fk_run(rig, d, fk.rot, fk.pos);
        let (hl, hr) = (role(HAND_L), role(HAND_R));
        let (live_l, live_r, r_l0, r_r0) = (fk.at(hl), fk.at(hr), fk.rot(hl), fk.rot(hr));
        let (mut tgt_l, mut tgt_r) = (z.tgt[0], z.tgt[1]);
        if cr {
            // (both hands fly out from his sides first, hang there a tenth of a second, and come onto the arm from there one after the other)
            let way = |live: [f64; 3], land: f64, to: [f64; 3], end: [f64; 3]| curve3(clamp(ug / land, 0.0, 1.0), &[(0.0, live), (tune.hold.catch.fling.t[0] / land, to), (tune.hold.catch.fling.t[1] / land, add3(to, tune.hold.catch.fling.drift)), (1.0, end)]);
            tgt_l = way(live_l, tune.hold.catch.fling.land[0], tune.hold.catch.fling.to[0], tgt_l);
            tgt_r = way(live_r, tune.hold.catch.fling.land[1], tune.hold.catch.fling.to[1], tgt_r);
        } else if grab {
            tgt_l = lerp3(&live_l, &tgt_l, smoothstep(0.15, 0.7, ug));
            tgt_r = curve3(smoothstep(0.05, 0.85, ug), &[(0.0, live_r), (0.5, tune.hold.via_up), (1.0, tgt_r)]);
        }
        // the disarm: his gun hand is thrown out beside his ribs as the arm lands, dragged after the wrench, let go of, and only then comes up
        if s_on {
            tgt_r = curve3(strip_in(s_t, tune.hold.strip.take), &[(0.0, live_r), (tune.hold.strip.take, tune.hold.strip.meet), (tune.hold.strip.take + 0.06, tune.hold.strip.torn), (tune.hold.strip.free, tune.hold.strip.torn), (tune.hold.strip.up, z.tgt[1])]);
        }
        // limp, both hang palm-in
        let (rot_hang_l, rot_hang_r) = (hand_rot(&fix_l, [0.1, -1.0, 0.15], [1.0, 0.0, 0.1]), hand_rot(&fix_r, [-0.1, -1.0, 0.15], [-1.0, 0.0, 0.1]));
        // (which way round the blend between the hold and the hang goes is the way it went the frame before)
        let to_hang = |q: [f64; 4], h: [f64; 4], prev: Option<[f64; 4]>| -> [f64; 4] {
            let dot = |x: &[f64; 4], y: &[f64; 4]| x[0] * y[0] + x[1] * y[1] + x[2] * y[2] + x[3] * y[3];
            let near = nlerp(&q, &h, limp_a);
            let Some(prev) = prev else {
                return near;
            };
            let s = if dot(&q, &h) < 0.0 { 1.0 } else { -1.0 };
            let far = q_unit([lerp(q[0], h[0] * s, limp_a), lerp(q[1], h[1] * s, limp_a), lerp(q[2], h[2] * s, limp_a), lerp(q[3], h[3] * s, limp_a)]);
            if abs(dot(&far, &prev)) > abs(dot(&near, &prev)) {
                far
            } else {
                near
            }
        };
        let was = if kept { *he_hr } else { [None, None] };
        let rot_l = to_hang(nlerp(&r_l0, &z.rot[0], e_a), rot_hang_l, was[0]);
        let rot_r = to_hang(nlerp(&r_r0, &z.rot[1], if s_on { smoothstep(tune.hold.strip.take, tune.hold.strip.up, s_t) } else { e_a }), rot_hang_r, was[1]);
        *he_hr = if kept { [Some(rot_l), Some(rot_r)] } else { [None, None] };
        two_bone_ik(rig, d, fk.rot, fk.pos, role(UPPER_ARM_L), role(FOREARM_L), hl, &tgt_l, Some(&z.pole[0]), 0.85 * e_a, Some(&rot_l), false);
        let w_r = if s_on { max(0.85 * e_a, 0.9 * smoothstep(0.0, 0.1, s_t) * (1.0 - smoothstep(tune.hold.strip.up, tune.hold.strip.home, s_t))) } else { 0.85 * e_a };
        two_bone_ik(rig, d, fk.rot, fk.pos, role(UPPER_ARM_R), role(FOREARM_R), hr, &tgt_r, Some(&z.pole[1]), w_r, Some(&rot_r), false);
        hand(d, hh, 0, &z.curl[0], z.thumb[0], z.spread[0], tune);
        if s_on {
            // (the fist round the gun opens as the gun leaves it, not in a frame)
            let op = smoothstep(tune.hold.strip.take, tune.hold.strip.take + 0.1, s_t);
            hand(d, hh, 1, &[lerp(0.75, z.curl[1][0], op), lerp(0.8, z.curl[1][1], op), lerp(0.8, z.curl[1][2], op), lerp(0.8, z.curl[1][3], op)], lerp(0.6, z.thumb[1], op), z.spread[1] * op, tune);
        } else {
            hand(d, hh, 1, &z.curl[1], z.thumb[1], z.spread[1], tune);
        }
    })
    .is_none()
    {
        return 0;
    }
    // (the first frames of the grab: mostly still his own arms; the catch's fling has them before the layer's weight is in)
    let w_a0 = w * e_a + w * (1.0 - e_a) * 0.35;
    let w_a = if cr { max(w_a0, smoothstep(tune.hold.catch.fling.arm_in[0], tune.hold.catch.fling.arm_in[1], ug) * lv) } else { w_a0 };
    arms_mask();
    if !ask_blend(g, at, at_b, w_a, true, at, n, tune) {
        return 0;
    }
    if s_on && w_a < 1.0 {
        // (…but his gun arm is where the disarm wants it by `take`)
        let w_s = max(w_a, smoothstep(0.0, 0.8 * tune.hold.strip.take, s_t) * (1.0 - smoothstep(tune.hold.strip.up, tune.hold.strip.home, s_t)));
        if w_s > w_a {
            ask_mask(&[(role(SHOULDER_R), 1.0, true)]);
            if !ask_blend(g, at, at_b, (w_s - w_a) / (1.0 - w_a), true, at, n, tune) {
                return 0;
            }
        }
    }
    hands_to(&hs, an);
    RAN | RAN_ALL
}
/// bigLayer's ease: a share clamped to 0‥1 and smoothed
fn e3(u: f64) -> f64 {
    let u = clamp(u, 0.0, 1.0);
    u * u * (3.0 - 2.0 * u)
}
/// a unit direction turned k of the way to another
fn dir_mix(a: [f64; 3], b: [f64; 3], kk: f64) -> [f64; 3] {
    if kk <= 0.0 {
        a
    } else if kk >= 1.0 {
        b
    } else {
        normalize(q_rotate(&slerp(&[0.0, 0.0, 0.0, 1.0], &from_to(&a, &b), kk), &a))
    }
}
/// where a hand goes and how it is turned; a fist laid on the hammer's line also keeps the line's two axes through it (fingers, back)
#[derive(Clone, Copy)]
struct BigSpot {
    at: [f64; 3],
    rot: [f64; 4],
    f: [f64; 3],
    b: [f64; 3],
}
fn spot_mix(a: BigSpot, b: BigSpot, e: f64, bow: Option<[f64; 3]>) -> BigSpot {
    if e <= 0.0 {
        a
    } else if e >= 1.0 {
        b
    } else {
        let at = match bow {
            Some(bw) => mad3(lerp3(&a.at, &b.at, e), bw, sin(PI * e)),
            None => lerp3(&a.at, &b.at, e),
        };
        BigSpot { at, rot: slerp(&a.rot, &b.rot, e), f: [0.0; 3], b: [0.0; 3] }
    }
}
/// the hammer as its own line: the right fist's grip and the handle's direction, butt to head
#[derive(Clone, Copy)]
struct BigLine {
    g: [f64; 3],
    d: [f64; 3],
}
fn line_mix(a: BigLine, b: BigLine, kk: f64, bow: Option<[f64; 3]>) -> BigLine {
    if kk <= 0.0 {
        a
    } else if kk >= 1.0 {
        b
    } else {
        let g = match bow {
            Some(bw) => mad3(lerp3(&a.g, &b.g, kk), bw, sin(PI * kk)),
            None => lerp3(&a.g, &b.g, kk),
        };
        BigLine { g, d: dir_mix(a.d, b.d, kk) }
    }
}
/// CharacterAnimator.twisted: where a point posed below the chest ENDS UP after the chest twist still to come, and where spine2 itself does
struct Twisted {
    e1: [f64; 4],
    e2: [f64; 4],
    e3: [f64; 4],
    p1: [f64; 3],
    p2t: [f64; 3],
    p3t: [f64; 3],
}
impl Twisted {
    fn of(fk: &Fk, tw_y: f64, tw_p: f64) -> Twisted {
        let (e1, e2) = (q_yaw(tw_y * 0.25), q_yaw(tw_y * 0.35));
        let mut e3 = q_yaw(tw_y * 0.4);
        if !falsy(tw_p) {
            let right = q_rotate(&e3, &AX);
            e3 = q_mul(&axis_angle(&right, tw_p), &e3);
        }
        let (p1, p2, p3) = (fk.at(role(SPINE1)), fk.at(role(SPINE2)), fk.at(role(SPINE3)));
        let p2t = add3(q_rotate(&e1, &sub3(p2, p1)), p1);
        let p3t = add3(q_rotate(&e2, &sub3(add3(q_rotate(&e1, &sub3(p3, p1)), p1), p2t)), p2t);
        Twisted { e1, e2, e3, p1, p2t, p3t }
    }
    fn at(&self, h: [f64; 3]) -> [f64; 3] {
        let x1 = add3(q_rotate(&self.e1, &sub3(h, self.p1)), self.p1);
        let x2 = add3(q_rotate(&self.e2, &sub3(x1, self.p2t)), self.p2t);
        add3(q_rotate(&self.e3, &sub3(x2, self.p3t)), self.p3t)
    }
}
/// rigProps.ts bigBackOf for the hammer (BIGRIG.hammer.back): the point and the line across his back, off six joints in one space, `clear` m further off
fn big_back_of(hips: [f64; 3], mid: [f64; 3], th_l: [f64; 3], th_r: [f64; 3], ua_l: [f64; 3], ua_r: [f64; 3], clear: f64) -> ([f64; 3], [f64; 3]) {
    let mut up = sub3(lerp3(&ua_l, &ua_r, 0.5), hips);
    up = if len3(&up) > 1e-3 { normalize(up) } else { [0.0, 1.0, 0.0] };
    let mut left = lerp3(&normalize(sub3(th_l, th_r)), &normalize(sub3(ua_l, ua_r)), max(0.0, min(1.0, BIGRIG.hammer.back.follow)));
    left = mad3(left, up, -dot3(&left, &up));
    left = if len3(&left) > 1e-3 { normalize(left) } else { normalize(sub3(ua_l, ua_r)) };
    let cz = cross3(&left, &up);
    let in_c = |dv: [f64; 3]| mad3(mad3(scale3(left, dv[0]), up, dv[1]), cz, dv[2]);
    let (off, bore) = (BIGRIG.hammer.back.off, BIGRIG.hammer.back.bore);
    (add3(mid, in_c([off[0], off[1], off[2] - clear])), normalize(in_c(bore)))
}
/// how many keys of a knife blow a call may hand over (character.ts KNIFE_KEYS_MAX: a longer blow is refused by name as the game loads)
const KNIFE_KEYS: usize = 4;
/// how many blows of a beat a call may hand over (the room on the bench: the layer is handed the two blows it reads, character.ts blowsNear)
const BIG_BLOWS_MAX: usize = 64;
/// The big slot's tool in his hands (the hammer or the breaching shotgun): the trunk and the feet for the blow or the shot straight into the pose, the
/// head back onto the point, then both arms by IK on a copy, whole from the upper arm down.
pub fn big_layer(g: usize, an: &mut Anim, b: &AnimBigTool, at_buf: usize, blows: &[f64], at0: [f64; 3], lg: &AnimLongGrip, tw_y: f64, tw_p: f64, tune: &Knobs) -> u32 {
    let (at, at_b, _) = poses(an);
    let (ham, t, equip, stow, end, snug, windup) = (b.kind == AnimBigToolKind::Hammer, b.t, b.equip, b.stow, b.end, b.snug.unwrap_or(false), b.windup);
    // (the shotgun is drawn as the carbine it is: its support grip, its group's offset and its stock's rear end)
    let (lg_wrist, lg_fing, lg_back, adj, stock_l0) = (lg.wrist, lg.fingers, lg.back, RIG.carbine.adj.map_or([0.0; 3], |a| a.off), RIG.carbine.stock_top.l0);
    let (speed, clear, fix_l, fix_r, gaze_local) = (an.speed, an.big_clear, an.hand_fix[0], an.hand_fix[1], an.gaze_local);
    let stowing = end >= 0.0 && t >= end;
    let us = if stowing { clamp((t - end) / max(1e-3, stow), 0.0, 1.0) } else { 0.0 };
    an.big_under = false;
    // ---- the beat as shares
    let (mut prev, mut next) = (f64::NEG_INFINITY, f64::INFINITY);
    for &b in blows {
        if b <= t {
            prev = max(prev, b);
        } else if !(end >= 0.0 && b > end) {
            next = min(next, b);
        }
    }
    let (mut wind, mut sw, mut hit, mut kick, mut plant) = (0.0, 0.0, 0.0, 0.0, 0.0);
    if ham {
        if next < f64::INFINITY {
            let ws = max(next - windup, prev);
            let dd = next - ws;
            if t >= ws && dd > 1e-3 {
                let s_w = min(clamp(BIGMOVE.hammer.swing * dd, BIGMOVE.hammer.swing_min, BIGMOVE.hammer.swing_max), dd * 0.6);
                wind = e3((t - ws) / max(1e-3, (dd - s_w) * BIGMOVE.hammer.raise));
                sw = clamp((t - (next - s_w)) / s_w, 0.0, 1.0);
            }
        }
        if prev > f64::NEG_INFINITY {
            hit = 1.0 - e3((t - prev) / BIGMOVE.hammer.rebound);
        }
        plant = max(wind, if prev > f64::NEG_INFINITY { if next < f64::INFINITY { 1.0 } else { 1.0 - e3((t - prev) / BIGMOVE.hammer.settle) } } else { 0.0 });
    } else if prev > f64::NEG_INFINITY {
        let r = t - prev;
        kick = if r < BIGMOVE.shotgun.kick_in { r / BIGMOVE.shotgun.kick_in } else { exp(-(r - BIGMOVE.shotgun.kick_in) / BIGMOVE.shotgun.kick_out) };
    }
    let (grab, put) = (BIGMOVE.grab, BIGMOVE.put);
    if stowing {
        let kq = 1.0 - e3(us / (put * 0.5));
        wind *= kq;
        sw *= kq;
        plant *= kq;
    }
    // (a blow asked for before the tool is out: it comes out with the wind-up)
    let ue = max(clamp(t / max(1e-3, equip), 0.0, 1.0), wind);
    let eb = e3((ue - grab) / (1.0 - grab));
    let in_hands = if stowing { min(eb, 1.0 - e3(us / put)) } else { eb };
    let es = pow(sw, 1.8);
    {
        let (t0, t1, put_at) = if snug { (BIGMOVE.snug.take[0], BIGMOVE.snug.take[1], BIGMOVE.snug.put_at) } else { (BIGMOVE.take[0], BIGMOVE.take[1], BIGMOVE.put_at) };
        an.big_k = smoothstep(t0, t1, ue) * (if stowing { 1.0 - smoothstep(put * put_at, put, us) } else { 1.0 });
    }
    const LEGS: [usize; 11] = [HIPS, SPINE1, SPINE2, NECK, HEAD, THIGH_L, SHIN_L, FOOT_L, THIGH_R, SHIN_R, FOOT_R];
    let Some(Some(n)) = with_pose(g, at, |_d, _rig, n, _fk| if cast(n, &ARM_ROLES) && cast(n, &LEGS) { Some(n) } else { None }) else {
        return 0;
    };
    let Some(mut hs) = hands_of(g, an) else {
        return 0;
    };
    if ham {
        if !ask_blend(g, at, at, 1.0, false, at_buf, n, tune) {
            return 0;
        }
        an.big_under = true;
    }
    // ---- where the point is from where he stands, the trunk and the pelvis under it, the feet put back, the head onto the point
    let Some((tdir, tleft)) = with_pose(g, at, |d, rig, _n, fk| {
        fk_run(rig, d, fk.rot, fk.pos);
        let hips_n = role(HIPS);
        let hp = fk.at(hips_n);
        let (mut tx, mut tz) = (at0[0] - hp[0], at0[2] - hp[2]);
        let reach = hypot2(tx, tz);
        if reach > 1e-3 {
            tx /= reach;
            tz /= reach;
        } else {
            tx = 0.0;
            tz = 1.0;
        }
        let (tdir, tleft, psi) = ([tx, 0.0, tz], [tz, 0.0, -tx], clamp(atan2(tx, tz), -1.0, 1.0));
        // what squares the chest AS THE POSE UNDER IT HAS IT onto the point
        let turn_to = {
            let f = trunk_frame(fk).2;
            clamp(wrap_angle(psi - atan2(f[0], f[2])), -1.2, 1.2)
        };
        let still = 1.0 - smoothstep(0.2, 1.0, abs(speed));
        let (chest_yaw, pel_yaw, pitch, sh_f, sh_s, drop);
        if ham {
            let (a_w, a_i, far) = (wind * (1.0 - es), max(hit, es), smoothstep(BIGMOVE.hammer.far[0], BIGMOVE.hammer.far[1], reach));
            chest_yaw = plant * turn_to * BIGMOVE.hammer.face + (a_w * BIGMOVE.hammer.wound_body.yaw + a_i * BIGMOVE.hammer.blow_body.yaw) * DEGR;
            pel_yaw = (plant * turn_to * BIGMOVE.hammer.face * 0.4 + (a_w * BIGMOVE.hammer.wound_body.pel_yaw + a_i * BIGMOVE.hammer.blow_body.pel_yaw) * DEGR) * still;
            pitch = (a_w * BIGMOVE.hammer.wound_body.pitch + a_i * (BIGMOVE.hammer.blow_body.pitch + far * BIGMOVE.hammer.blow_body.pitch_far)) * DEGR;
            sh_f = (a_i * (BIGMOVE.hammer.blow_body.fwd + far * BIGMOVE.hammer.blow_body.fwd_far) - a_w * BIGMOVE.hammer.wound_body.back) * still;
            sh_s = a_w * BIGMOVE.hammer.wound_body.side * still;
            drop = (plant * BIGMOVE.hammer.drop + a_i * BIGMOVE.hammer.blow_body.drop) * still;
        } else {
            chest_yaw = in_hands * (turn_to + BIGMOVE.shotgun.yaw * DEGR);
            pel_yaw = in_hands * (turn_to * 0.3 + BIGMOVE.shotgun.pel_yaw * DEGR) * still;
            pitch = (in_hands * BIGMOVE.shotgun.pitch + kick * BIGMOVE.shotgun.kick_pitch) * DEGR;
            sh_f = -BIGMOVE.shotgun.shove * kick * still;
            sh_s = 0.0;
            drop = 0.0;
        }
        if abs(chest_yaw) + abs(pel_yaw) + abs(pitch) + abs(sh_f) + abs(sh_s) + drop > 1e-5 {
            let (fl_n, fr_n) = (role(FOOT_L), role(FOOT_R));
            let (mut f_l, mut f_r, r_l, r_r) = (fk.at(fl_n), fk.at(fr_n), fk.rot(fl_n), fk.rot(fr_n));
            let ax_t = cross3(&AY, &tdir);
            // (the pelvis turn carries the whole spine; the spine joints add the rest)
            let seg = |kk: f64| q_mul(&q_yaw((chest_yaw - pel_yaw) * kk), &axis_angle(&ax_t, pitch * kk));
            shift(d, rig, hips_n, &[tdir[0] * sh_f + tleft[0] * sh_s, -drop, tdir[2] * sh_f + tleft[2] * sh_s]);
            turn(d, rig, hips_n, q_yaw(pel_yaw));
            turn(d, rig, role(SPINE1), seg(0.3));
            turn(d, rig, role(SPINE2), seg(0.35));
            turn(d, rig, role(SPINE3), seg(0.35));
            // the head: back out of most of it (the look below puts the eyes on the point)
            let hq = |kk: f64| q_mul(&q_yaw(-chest_yaw * 0.8 * kk), &axis_angle(&ax_t, -pitch * 0.8 * kk));
            turn(d, rig, role(NECK), hq(0.4));
            turn(d, rig, role(HEAD), hq(0.6));
            if ham && plant > 0.0 {
                // a wider stance for the blow, the left foot toward the point: stepped out to, not slid
                let (kq, up) = (plant * still, BIGMOVE.hammer.stance.lift * sin(PI * plant) * still);
                f_l[0] += (tleft[0] * BIGMOVE.hammer.stance.l[0] + tdir[0] * BIGMOVE.hammer.stance.l[1]) * kq;
                f_l[2] += (tleft[2] * BIGMOVE.hammer.stance.l[0] + tdir[2] * BIGMOVE.hammer.stance.l[1]) * kq;
                f_l[1] += up;
                f_r[0] += (tleft[0] * BIGMOVE.hammer.stance.r[0] + tdir[0] * BIGMOVE.hammer.stance.r[1]) * kq;
                f_r[2] += (tleft[2] * BIGMOVE.hammer.stance.r[0] + tdir[2] * BIGMOVE.hammer.stance.r[1]) * kq;
                f_r[1] += up;
            }
            // feet back where the locomotion has them, soles as they were
            fk_run(rig, d, fk.rot, fk.pos);
            two_bone_ik(rig, d, fk.rot, fk.pos, role(THIGH_L), role(SHIN_L), fl_n, &f_l, Some(&knee_pole(&r_l)), 0.6, Some(&r_l), false);
            two_bone_ik(rig, d, fk.rot, fk.pos, role(THIGH_R), role(SHIN_R), fr_n, &f_r, Some(&knee_pole(&r_r)), 0.6, Some(&r_r), false);
        }
        let look_w = if ham { plant } else { in_hands };
        if look_w > 0.003 {
            fk_run(rig, d, fk.rot, fk.pos);
            let tg = untwist(fk, tw_y, tw_p, at0);
            look_at(d, rig, fk, &gaze_local, tg, 0.8 * look_w, 0.0);
        }
        (tdir, tleft)
    }) else {
        return 0;
    };
    // ---- the arms, by IK on a copy against the body as it now stands
    if !ask_blend(g, at, at, 1.0, false, at_b, n, tune) {
        return 0;
    }
    let hh = &mut hs;
    let (at_back, grab_at) = (&mut an.big_at_back, &mut an.big_grab);
    let done = with_pose(g, at_b, |d, rig, _n, fk| {
        fk_run(rig, d, fk.rot, fk.pos);
        let (f_left, f_up, f_fwd) = trunk_frame(fk);
        let dir_t = |q: [f64; 3]| -> [f64; 3] {
            [f_left[0] * q[0] + f_up[0] * q[1] + f_fwd[0] * q[2], f_left[1] * q[0] + f_up[1] * q[1] + f_fwd[1] * q[2], f_left[2] * q[0] + f_up[2] * q[1] + f_fwd[2] * q[2]]
        };
        let (hr_n, hl_n) = (role(HAND_R), role(HAND_L));
        let (s_r, s_l) = (fk.at(role(UPPER_ARM_R)), fk.at(role(UPPER_ARM_L)));
        let hp_at = untwist(fk, tw_y, tw_p, at0);
        let arc = dir_t(BIGMOVE.arc);
        let (u_r, u_l) = (BigSpot { at: fk.at(hr_n), rot: fk.rot(hr_n), f: [0.0; 3], b: [0.0; 3] }, BigSpot { at: fk.at(hl_n), rot: fk.rot(hl_n), f: [0.0; 3], b: [0.0; 3] });
        // a fist on the line at g: the handle through it, thumb toward the head; the knuckles lead away from that shoulder, squared to the handle
        let fist = |side: usize, gp: [f64; 3], dv: [f64; 3], sh: [f64; 3]| -> BigSpot {
            let mut h = add3(sub3(gp, sh), dir_t([if side == 1 { -0.03 } else { 0.03 }, 0.0, 0.02]));
            h = mad3(h, dv, -dot3(&h, &dv));
            if len3(&h) < 1e-3 {
                h = cross3(&dv, &f_up);
            }
            let f = normalize(h);
            let b = if side == 1 { cross3(&f, &dv) } else { cross3(&dv, &f) };
            BigSpot { at: mad3(mad3(gp, f, -BIGRIG.hammer.grip.fwd), b, BIGRIG.hammer.grip.palm), rot: hand_rot(if side == 1 { &fix_r } else { &fix_l }, f, b), f, b }
        };
        let (back_r, held): (BigSpot, BigSpot);
        // (the left hand's place on the tool, by the kind: the hammer's line, or the gun's three axes)
        let mut ham_line = BigLine { g: [0.0; 3], d: [0.0; 3] };
        let mut gun = ([0.0f64; 3], [0.0f64; 3], [0.0f64; 3]);
        if ham {
            let off = |gp: [f64; 3], dv: [f64; 3]| BigLine { g: add3(s_r, dir_t(gp)), d: normalize(dir_t(dv)) };
            // on his back, as the drawer has it: the fist takes it just under the head (snug: the back's frame as it WILL be after the chest twist still
            // to come, and the place that ENDS UP there)
            let tw = snug && (!falsy(tw_y) || !falsy(tw_p));
            let (ua_l, ua_r) = (fk.at(role(UPPER_ARM_L)), fk.at(role(UPPER_ARM_R)));
            let (hips, th_l, th_r) = (fk.at(role(HIPS)), fk.at(role(THIGH_L)), fk.at(role(THIGH_R)));
            let (bo, bz) = if tw {
                let twd = Twisted::of(fk, tw_y, tw_p);
                let (o, z) = big_back_of(hips, twd.p2t, th_l, th_r, twd.at(ua_l), twd.at(ua_r), clear);
                (untwist(fk, tw_y, tw_p, o), untwist_dir(tw_y, tw_p, z))
            } else {
                big_back_of(hips, fk.at(role(SPINE2)), th_l, th_r, ua_l, ua_r, clear)
            };
            let head_at = BIGRIG.hammer.head_at;
            let back_l = BigLine { g: mad3(bo, bz, head_at - BIGMOVE.hammer.grab_at), d: bz };
            let (ready_l, wound_l, over_l) = (off(BIGMOVE.hammer.ready.g, BIGMOVE.hammer.ready.d), off(BIGMOVE.hammer.wound.g, BIGMOVE.hammer.wound.d), off(BIGMOVE.hammer.over.g, BIGMOVE.hammer.over.d));
            // the blow: the head ON the point, the butt end by the pelvis on the point's own side
            let butt = BIGMOVE.hammer.butt;
            let b0 = [hips[0] + tleft[0] * butt[0] + tdir[0] * butt[2], hips[1] + butt[1], hips[2] + tleft[2] * butt[0] + tdir[2] * butt[2]];
            let di = normalize(sub3(hp_at, b0));
            let blow_l = BigLine { g: mad3(hp_at, di, -head_at), d: di };
            let mut u = line_mix(back_l, ready_l, eb, Some(arc));
            if hit > 0.0 {
                u = line_mix(u, blow_l, hit, None);
            }
            if stowing {
                u = line_mix(u, back_l, e3(us / put), Some(arc));
            }
            let mut l = line_mix(u, wound_l, wind, None);
            if sw > 0.0 {
                // over the top and down into it
                l = line_mix(line_mix(l, over_l, es, None), line_mix(over_l, blow_l, es, None), es, None);
            }
            back_r = fist(1, back_l.g, back_l.d, s_r);
            held = fist(1, l.g, l.d, s_r);
            // (how much of the fist's place this frame is the tool's place ON HIS BACK, and how far up the handle it holds it)
            {
                let mut wb = (1.0 - eb) * (1.0 - hit);
                if stowing {
                    wb += (1.0 - wb) * e3(us / put);
                }
                wb *= (1.0 - wind) * (if sw > 0.0 { (1.0 - es) * (1.0 - es) } else { 1.0 });
                *at_back = if ue < grab { e3(ue / grab) } else if stowing && us > put { 1.0 - e3((us - put) / (1.0 - put)) } else { wb };
                *grab_at = head_at - BIGMOVE.hammer.grab_at;
            }
            ham_line = l;
        } else {
            // (the rifle's own place over the shoulder: the fist there, the muzzle down his back)
            back_r = BigSpot { at: add3(s_r, dir_t(tune.kit.apex.at)), rot: hand_rot(&fix_r, dir_t(tune.kit.apex.fing), dir_t(tune.kit.apex.back)), f: [0.0; 3], b: [0.0; 3] };
            let pk = add3(s_r, dir_t(BIGMOVE.shotgun.pocket));
            let mut z = normalize(sub3(hp_at, pk));
            let mut yu = mad3(AY, z, -z[1]);
            yu = if len3(&yu) > 1e-3 { normalize(yu) } else { f_fwd };
            if kick > 0.0 {
                let an = kick * BIGMOVE.shotgun.climb * DEGR;
                let (cs, sn) = (cos(an), sin(an));
                let z1 = add3(scale3(z, cs), scale3(yu, sn));
                yu = sub3(scale3(yu, cs), scale3(z, sn));
                z = z1;
            }
            let x = cross3(&yu, &z);
            gun = (x, yu, z);
            let in_g = |p: [f64; 3]| -> [f64; 3] { [x[0] * p[0] + yu[0] * p[1] + z[0] * p[2], x[1] * p[0] + yu[1] * p[1] + z[1] * p[2], x[2] * p[0] + yu[2] * p[1] + z[2] * p[2]] };
            // the trigger wrist that puts the butt (the stock's rear end, through the group's own offset) in the pocket
            let wr0 = add3(pk, in_g([-adj[0], -adj[1], -stock_l0 - adj[2] - kick * BIGMOVE.shotgun.kick]));
            let up0 = BigSpot { at: wr0, rot: hand_rot(&fix_r, z, in_g([-0.8, 0.6, 0.0])), f: [0.0; 3], b: [0.0; 3] };
            held = spot_mix(back_r, up0, in_hands, Some(arc));
        }
        let r = if ue < grab { spot_mix(u_r, back_r, e3(ue / grab), Some(arc)) } else if stowing && us > put { spot_mix(back_r, u_r, e3((us - put) / (1.0 - put)), Some(arc)) } else { held };
        // how much of the elbows is the layer's: none at either end (the pose's own under it)
        let own = e3(ue / 0.2) * (if stowing { 1.0 - e3((us - put) / (1.0 - put)) } else { 1.0 });
        // a hand up by the shoulder: the elbow out and up, not folded into the ribs
        let high = smoothstep(-0.18, 0.1, r.at[1] - s_r[1]);
        two_bone_ik(rig, d, fk.rot, fk.pos, role(UPPER_ARM_R), role(FOREARM_R), hr_n, &r.at, Some(&dir_t(lerp3(&[-0.5, -0.8, -0.3], &[-1.0, 0.3, -0.1], high))), 0.85 * own, Some(&r.rot), false);
        let j = smoothstep(BIGMOVE.join[0], BIGMOVE.join[1], ue) * (if stowing { 1.0 - smoothstep(0.0, BIGMOVE.unjoin, us) } else { 1.0 });
        // let go of his gun, waiting in front of him
        let mut l = spot_mix(u_l, BigSpot { at: add3(s_l, dir_t(BIGMOVE.wait)), rot: hand_rot(&fix_l, dir_t([-0.25, -0.35, 0.9]), dir_t([0.35, 0.9, 0.25])), f: [0.0; 3], b: [0.0; 3] }, e3(ue / 0.3), None);
        if j > 0.0 {
            let wr = fk.at(hr_n);
            let on_it = if ham {
                let gp = mad3(mad3(wr, held.f, BIGRIG.hammer.grip.fwd), held.b, -BIGRIG.hammer.grip.palm);
                fist(0, mad3(gp, ham_line.d, -BIGRIG.hammer.hands), ham_line.d, s_l)
            } else {
                let (x, yu, z) = gun;
                let in_g = |p: [f64; 3]| -> [f64; 3] { [x[0] * p[0] + yu[0] * p[1] + z[0] * p[2], x[1] * p[0] + yu[1] * p[1] + z[1] * p[2], x[2] * p[0] + yu[2] * p[1] + z[2] * p[2]] };
                BigSpot { at: add3(wr, in_g(lg_wrist)), rot: hand_rot(&fix_l, in_g(lg_fing), in_g(lg_back)), f: [0.0; 3], b: [0.0; 3] }
            };
            l = spot_mix(l, on_it, e3(j), None);
        }
        if stowing {
            l = spot_mix(l, u_l, e3((us - BIGMOVE.unjoin * 0.5) / (0.92 - BIGMOVE.unjoin * 0.5)), None);
        }
        two_bone_ik(rig, d, fk.rot, fk.pos, role(UPPER_ARM_L), role(FOREARM_L), hl_n, &l.at, Some(&dir_t([0.5, -0.85, -0.15])), 0.6 * own, Some(&l.rot), false);
        {
            let gr = smoothstep(grab - 0.15, grab, ue) * (if stowing { 1.0 - smoothstep(put, put + 0.12, us) } else { 1.0 });
            let (open, shut) = ([BIGMOVE.open[0], BIGMOVE.open[1], BIGMOVE.open[2], BIGMOVE.open[3]], [BIGMOVE.fist[0], BIGMOVE.fist[1], BIGMOVE.fist[2], BIGMOVE.fist[3]]);
            let cu = |kk: f64| [lerp(open[0], shut[0], kk), lerp(open[1], shut[1], kk), lerp(open[2], shut[2], kk), lerp(open[3], shut[3], kk)];
            hand(d, hh, 1, &cu(gr), lerp(0.25, 0.6, gr), 0.0, tune);
            hand(d, hh, 0, &cu(j), lerp(0.25, 0.6, j), 0.0, tune);
        }
    });
    if done.is_none() {
        return 0;
    }
    arm_mask(true);
    if !ask_blend(g, at, at_b, 1.0, true, at, n, tune) {
        return 0;
    }
    hands_to(&hs, an);
    RAN | RAN_ALL
}

/// An arm's IK pole from a swivel angle about the shoulder → hand line, in a basis off a reference the hand never points along
fn swivel_pole(h: [f64; 3], rf: [f64; 3], psi: f64) -> [f64; 3] {
    let hn = normalize(h);
    let b1 = normalize(mad3(rf, hn, -dot3(&rf, &hn)));
    let b2 = cross3(&hn, &b1);
    mad3(scale3(b1, cos(psi)), b2, sin(psi))
}
/// how many keys a table of the throw may have (character.ts THROW_KEYS_MAX: a longer one is refused by name as the game loads)
const THROW_KEYS: usize = 16;
/// (seventeen counts, eleven tables of two numbers a key and six of four: the whole of it fits the bench's own numbers)
/// the throw's tables as handed over, in the order character.ts throwLayer lists them (K1, K3) and throwLayerAsk writes them: eleven of (time, value) keys, then six of (time, vector) keys
const TH_CHEST_YAW: usize = 0;
const TH_PELVIS_YAW: usize = 1;
const TH_PITCH: usize = 2;
const TH_ROLL: usize = 3;
const TH_PEL_X: usize = 4;
const TH_PEL_Y: usize = 5;
const TH_PEL_Z: usize = 6;
const TH_CLAV_UP: usize = 7;
const TH_CLAV_FWD: usize = 8;
const TH_SWIV_R: usize = 9;
const TH_SWIV_L: usize = 10;
const TH_HAND_R: usize = 0;
const TH_FING_R: usize = 1;
const TH_BACK_R: usize = 2;
const TH_HAND_L: usize = 3;
const TH_FING_L: usize = 4;
const TH_BACK_L: usize = 5;
struct ThrowTables {
    n1: [usize; 11],
    k1: [[[f64; 2]; THROW_KEYS]; 11],
    n3: [usize; 6],
    k3: [[(f64, [f64; 3]); THROW_KEYS]; 6],
}
impl ThrowTables {
    fn of(t: &Tables) -> Option<ThrowTables> {
        let mut tb = ThrowTables { n1: [0; 11], k1: [[[0.0; 2]; THROW_KEYS]; 11], n3: [0; 6], k3: [[(0.0, [0.0; 3]); THROW_KEYS]; 6] };
        for (i, keys) in t.k1.iter().enumerate() {
            if keys.len() < 2 || keys.len() > THROW_KEYS {
                return None;
            }
            tb.n1[i] = keys.len();
            tb.k1[i][..keys.len()].copy_from_slice(keys);
        }
        for (i, keys) in t.k3.iter().enumerate() {
            if keys.len() < 2 || keys.len() > THROW_KEYS {
                return None;
            }
            tb.n3[i] = keys.len();
            for (to, (at, v)) in tb.k3[i].iter_mut().zip(keys) {
                *to = (*at, [v[0], v[1], v[2]]);
            }
        }
        Some(tb)
    }
    fn c1(&self, i: usize, t: f64) -> f64 {
        curve(t, &self.k1[i][..self.n1[i]])
    }
    fn c3(&self, i: usize, t: f64) -> [f64; 3] {
        curve3(t, &self.k3[i][..self.n3[i]])
    }
}
/// The throw (the overhand at its size, or the underhand toss; mirrored for the free hand's throw): body English straight into the pose with the feet
/// put back, then both arms by IK on a copy along paths that begin and end on the live hands, in through per-arm masks.
pub fn throw_layer(g: usize, an: &mut Anim, t: f64, tables: &Tables, tune: &Knobs) -> u32 {
    let (at, at_b, _) = poses(an);
    let Some(tb) = ThrowTables::of(tables) else {
        return 0;
    };
    let (thr_low, thr_far, off) = (an.thr_low, an.thr_far, an.thr_off);
    let (dur, rel) = (THROW_DUR, THROW_RELEASE);
    // which swing: the underhand toss, or the overhand at its size (`full` = as authored)
    let low = thr_low >= 0.0;
    let full = !low && thr_far >= 1.0;
    // (the free hand's throw: the same swing, mirrored)
    let sx = if off { -1.0 } else { 1.0 };
    let mir = |p: [f64; 3]| -> [f64; 3] { [-p[0], p[1], p[2]] };
    // a blast that folds or blinds him mid-swing takes the swing
    let (hit, h0) = (max(clamp(an.mvs.cf.x, 0.0, 1.0), clamp(an.mvs.dz.x, 0.0, 1.0)), an.thr_hit0);
    let give = if true && hit > h0 { 1.0 - clamp((hit - h0) / max(1.0 - h0, 0.25), 0.0, 1.0) } else { 1.0 };
    let k_s = if low { lerp(tune.throw_by.toss_min, 1.0, thr_low) } else if full { 1.0 } else { lerp(tune.throw_by.turn_min, 1.0, thr_far) };
    let k_t = k_s * give;
    let k_a = if low { k_s } else if full { 1.0 } else { lerp(tune.throw_by.arm_min, 1.0, thr_far) };
    let piv = if low { tune.throw_by.toss_pivot } else { tune.throw_by.pivot };
    // the legs' share: halved on bent knees, less again on the move
    let cr = clamp(an.crouch, 0.0, 1.0);
    let leg_k = (1.0 - 0.5 * cr) * (1.0 - 0.4 * smoothstep(0.3, 1.5, abs(an.speed)));
    let cin = an.thr_carry.as_ref().map(|k| (k.r, k.l));
    let (fix_l, fix_r) = (an.hand_fix[0], an.hand_fix[1]);
    const BODY: [usize; 11] = [HIPS, SPINE1, SPINE2, NECK, HEAD, THIGH_L, SHIN_L, FOOT_L, THIGH_R, SHIN_R, FOOT_R];
    let Some(Some(n)) = with_pose(g, at, |_d, _rig, n, _fk| if cast(n, &ARM_ROLES) && cast(n, &BODY) { Some(n) } else { None }) else {
        return 0;
    };
    let Some(mut hs) = hands_of(g, an) else {
        return 0;
    };
    // ---- what is underneath (feet to re-plant, each hand from its shoulder), then the body English
    let Some((u_r, u_l, gun)) = with_pose(g, at, |d, rig, _n, fk| {
        fk_run(rig, d, fk.rot, fk.pos);
        let (fl_n, fr_n, hr_n, hl_n) = (role(FOOT_L), role(FOOT_R), role(HAND_R), role(HAND_L));
        let (f_l, f_r, r_l, r_r) = (fk.at(fl_n), fk.at(fr_n), fk.rot(fl_n), fk.rot(fr_n));
        let (u_r, u_l) = (sub3(fk.at(hr_n), fk.at(role(UPPER_ARM_R))), sub3(fk.at(hl_n), fk.at(role(UPPER_ARM_L))));
        // (the free hand's throw: where the long hold has the trigger hand, before the trunk winds)
        let gun = if off { Some((fk.at(hr_n), fk.rot(hr_n))) } else { None };
        let (chest_yaw, pel_yaw) = (tb.c1(TH_CHEST_YAW, t) * DEGR * k_t * sx, tb.c1(TH_PELVIS_YAW, t) * DEGR * leg_k * k_t * sx);
        // the pelvis turn carries the whole spine; the spine joints add the rest
        let sp_yaw = chest_yaw - pel_yaw;
        let (pitch, roll) = (tb.c1(TH_PITCH, t) * DEGR * k_t, tb.c1(TH_ROLL, t) * DEGR * k_t * sx);
        // crouched: sit up out of the clip's hunch to get the arm over, chest first
        let ext = if low { 0.0 } else { 18.0 * DEGR * cr * bump(t, 0.0, 0.12, 0.5, dur) * (if full { 1.0 } else { lerp(tune.throw_by.sit, 1.0, thr_far) }) * give };
        let hips_n = role(HIPS);
        shift(d, rig, hips_n, &[tb.c1(TH_PEL_X, t) * leg_k * k_t * sx, tb.c1(TH_PEL_Y, t) * leg_k * k_t, tb.c1(TH_PEL_Z, t) * leg_k * k_t]);
        turn(d, rig, hips_n, q_yaw(pel_yaw));
        let seg = |kk: f64, kp: f64, ke: f64| q_mul(&q_yaw(sp_yaw * kk), &q_mul(&q_roll(roll * kk), &q_pitch(pitch * kp - ext * ke)));
        turn(d, rig, role(SPINE1), seg(0.3 + 0.12 * cr, 0.4, 0.25));
        turn(d, rig, role(SPINE2), seg(0.35 + 0.13 * cr, 0.35, 0.3));
        turn(d, rig, role(SPINE3), seg(0.35 - 0.25 * cr, 0.25, 0.45));
        // head: stays on the target, levelled out of the lean and the tilt, craned back down out of the sit-up
        let (hy, hr, hp) = (-chest_yaw * 0.85, -roll * 0.7, -pitch * 0.8 + ext);
        turn(d, rig, role(NECK), q_mul(&q_yaw(hy * 0.4), &q_mul(&q_roll(hr * 0.4), &q_pitch(hp * 0.45))));
        turn(d, rig, role(HEAD), q_mul(&q_yaw(hy * 0.6), &q_mul(&q_roll(hr * 0.6), &q_pitch(hp * 0.55))));
        turn(d, rig, if off { role(SHOULDER_L) } else { role(SHOULDER_R) }, q_mul(&q_yaw(tb.c1(TH_CLAV_FWD, t) * DEGR * k_t * sx), &q_roll(-tb.c1(TH_CLAV_UP, t) * DEGR * k_t * sx)));
        // feet back where the locomotion has them, soles as they were
        fk_run(rig, d, fk.rot, fk.pos);
        two_bone_ik(rig, d, fk.rot, fk.pos, role(THIGH_L), role(SHIN_L), fl_n, &f_l, Some(&knee_pole(&r_l)), 0.6, Some(&r_l), false);
        two_bone_ik(rig, d, fk.rot, fk.pos, role(THIGH_R), role(SHIN_R), fr_n, &f_r, Some(&knee_pole(&r_r)), 0.6, Some(&r_r), false);
        (u_r, u_l, gun)
    }) else {
        return 0;
    };
    // ---- arms, IK'd on a copy against the body as it now is
    if !ask_blend(g, at, at, 1.0, false, at_b, n, tune) {
        return 0;
    }
    let hh = &mut hs;
    let hang = &mut an.thr_hang;
    let Some((w_r, w_l)) = with_pose(g, at_b, |d, rig, _n, fk| {
        fk_run(rig, d, fk.rot, fk.pos);
        let (sh_r, sh_l) = (fk.at(role(UPPER_ARM_R)), fk.at(role(UPPER_ARM_L)));
        // crouched: the follow-through stays up off the front knee
        let low_k = 1.0 - 0.45 * cr;
        let lift = |p: [f64; 3]| -> [f64; 3] { [p[0], if p[1] < -0.25 { -0.25 + (p[1] + 0.25) * low_k } else { p[1] }, p[2]] };
        // a shorter throw: the hand's path drawn in toward its pivot, but it lets go where it lets go
        let near = |kt: f64, p: [f64; 3]| -> [f64; 3] {
            if full {
                return p;
            }
            let kk = if low { if kt > TOSS_LOW { 1.0 } else { k_a } } else { lerp(k_a, 1.0, 1.0 - smoothstep(0.0, tune.throw_by.through, abs(kt - rel))) };
            [piv[0] + (p[0] - piv[0]) * kk, piv[1] + (p[1] - piv[1]) * kk, piv[2] + (p[2] - piv[2]) * kk]
        };
        // the throwing hand under the layer, in the right arm's terms
        let u_t = if off { mir(u_l) } else { u_r };
        let r0 = if off { mir(match cin { Some((_, l)) => l, None => u_l }) } else { match cin { Some((r, _)) => r, None => u_r } };
        let l0 = match cin { Some((_, l)) => l, None => u_l };
        let n_r = tb.n3[TH_HAND_R];
        let mut k_r = [(0.0f64, [0.0f64; 3]); THROW_KEYS];
        for i in 0..n_r {
            let (kt, p) = tb.k3[TH_HAND_R][i];
            k_r[i] = (kt, lift(near(kt, p)));
        }
        if low {
            // a toss from a hand that is nowhere near his hip: the further the authored top is past what `tossIn` m/s reaches by its time, the more
            // the hand just drops, on one arc, to the swing's low point and scoops from there
            let (top, bot) = (k_r[0].1, k_r[1].1);
            let dd = hypot3(top[0] - r0[0], top[1] - r0[1], top[2] - r0[2]);
            let s = smoothstep(0.5, 1.0, tune.throw_by.toss_in * TOSS_TOP / max(dd, 1e-6));
            if s < 1.0 {
                k_r[0].1 = [lerp(lerp(r0[0], bot[0], 0.6), top[0], s), lerp(lerp(r0[1], bot[1], 0.6), top[1], s), lerp(lerp(r0[2], bot[2], 0.6) - 0.06, top[2], s)];
            }
        }
        // the balance arm: all of its reach only for the full throw; a toss lets it hang, read once, on the throw's first frame
        if low && !off && *hang < 0.0 {
            *hang = smoothstep(-0.38, -0.28, u_l[1]) * smoothstep(0.15, 0.28, u_l[2]);
        }
        let k_b = if full { 1.0 } else { lerp(tune.throw_by.off_min, 1.0, thr_far) };
        let off_g = if low { *hang } else { 0.0 };
        let hang = TOSS_HANG;
        let hang_y = if hang[1] < -0.25 { -0.25 + (hang[1] + 0.25) * low_k } else { hang[1] };
        let off_at = [lerp(u_l[0], hang[0], off_g), lerp(u_l[1], hang_y, off_g), lerp(u_l[2], hang[2], off_g)];
        let mut k_l = [(0.0f64, [0.0f64; 3]); THROW_KEYS];
        let n_l;
        if low {
            k_l[0] = (0.32, off_at);
            k_l[1] = (0.42, off_at);
            n_l = 2;
        } else {
            n_l = tb.n3[TH_HAND_L];
            for i in 0..n_l {
                let (kt, p) = tb.k3[TH_HAND_L][i];
                k_l[i] = (kt, if full { lift(p) } else { lift([u_l[0] + (p[0] - u_l[0]) * k_b, u_l[1] + (p[1] - u_l[1]) * k_b, u_l[2] + (p[2] - u_l[2]) * k_b]) });
            }
        }
        // the ends of each path hang off the live hand: part way toward the first / from the last authored point, bowed out round the body
        let via = |p: [f64; 3], q: [f64; 3], s: f64, bow: [f64; 3]| -> [f64; 3] { [lerp(p[0], q[0], s) + bow[0], lerp(p[1], q[1], s) + bow[1], lerp(p[2], q[2], s) + bow[2]] };
        let (top_r, end_r, reach_l, end_l) = (k_r[0].1, k_r[n_r - 1].1, k_l[0].1, k_l[n_l - 1].1);
        let mut pr = Path::new();
        pr.push(0.0, r0);
        if low {
            // (the toss: straight back past the hip and round again, no detour)
            pr.push(0.05, via(r0, top_r, 0.5, [-0.02, 0.0, -0.01]));
        } else {
            // (up: hanging below the lifting elbow, then out behind it at shoulder height before it cocks up over it)
            pr.push(0.06, via(r0, top_r, 0.35, [-0.075 * k_a, -0.015 * k_a, 0.02 * k_a]));
            pr.push(0.125, via(r0, top_r, 0.75, [-0.11 * k_a, 0.03 * k_a, -0.09 * k_a]));
        }
        for &(kt, p) in &k_r[..n_r] {
            pr.push(kt, p);
        }
        pr.push(0.62, via(end_r, u_t, 0.55, if low { [-0.03, 0.0, 0.04] } else { [-0.10, 0.02, -0.02] }));
        pr.push(dur, u_t);
        let off_r = pr.at(t);
        let mut pl = Path::new();
        pl.push(0.0, l0);
        pl.push(0.09, if low { via(l0, reach_l, 0.2, [0.0, 0.0, 0.0]) } else { via(l0, reach_l, 0.5, [0.04, 0.0, 0.02]) });
        for &(kt, p) in &k_l[..n_l] {
            pl.push(kt, p);
        }
        pl.push(0.63, via(end_l, u_l, 0.5, if low { [0.0, 0.0, 0.0] } else { [0.03, 0.0, 0.02] }));
        pl.push(dur, u_l);
        let off_l = pl.at(t);
        let (ua_r, fa_r, hr_n, ua_l, fa_l, hl_n) = (role(UPPER_ARM_R), role(FOREARM_R), role(HAND_R), role(UPPER_ARM_L), role(FOREARM_L), role(HAND_L));
        let (ref_r, ref_l) = (SWIV_REF_R, SWIV_REF_L);
        if off {
            let o = mir(off_r);
            two_bone_ik(rig, d, fk.rot, fk.pos, ua_l, fa_l, hl_n, &add3(sh_l, o), Some(&swivel_pole(o, ref_l, -tb.c1(TH_SWIV_R, t) * DEGR)), 1.0, Some(&hand_rot(&fix_l, mir(tb.c3(TH_FING_R, t)), mir(tb.c3(TH_BACK_R, t)))), false);
        } else {
            two_bone_ik(rig, d, fk.rot, fk.pos, ua_r, fa_r, hr_n, &add3(sh_r, off_r), Some(&swivel_pole(off_r, ref_r, tb.c1(TH_SWIV_R, t) * DEGR)), 1.0, Some(&hand_rot(&fix_r, tb.c3(TH_FING_R, t), tb.c3(TH_BACK_R, t))), false);
        }
        // (the free hand's throw) the gun does not swing with the trunk: the trigger hand stays where the long hold had it, turned as it was; lowered
        // through the swing to a hand's length under the shoulder
        if let Some((gp, gq)) = gun {
            let dn = max(0.0, gp[1] - (sh_r[1] - 0.12)) * bump(t, 0.0, 0.12, dur - 0.2, dur);
            two_bone_ik(rig, d, fk.rot, fk.pos, ua_r, fa_r, hr_n, &[gp[0], gp[1] - dn, gp[2]], None, 0.0, Some(&gq), false);
        }
        // (a toss's other arm: at weight only while it is let down, or while a throw re-triggered mid-swing hands it back to the pose underneath)
        let w_toss = if off { 0.0 } else if low { max(off_g, if cin.is_some() { 1.0 - smoothstep(0.0, 0.15, t) } else { 0.0 }) } else { 1.0 };
        if w_toss > 0.0 {
            let (f0, b0) = (tb.k3[TH_FING_L][0].1, tb.k3[TH_BACK_L][0].1);
            let (f_t, b_t) = (if low { f0 } else { tb.c3(TH_FING_L, t) }, if low { b0 } else { tb.c3(TH_BACK_L, t) });
            let s_t = if low { tb.k1[TH_SWIV_L][0][1] } else { tb.c1(TH_SWIV_L, t) };
            let part = |p: [f64; 3], q: [f64; 3]| -> [f64; 3] { [lerp(p[0], q[0], k_b), lerp(p[1], q[1], k_b), lerp(p[2], q[2], k_b)] };
            let rot = if low { None } else if full { Some(hand_rot(&fix_l, f_t, b_t)) } else { Some(hand_rot(&fix_l, part(f0, f_t), part(b0, b_t))) };
            let psi = (if full || low { s_t } else { lerp(tb.k1[TH_SWIV_L][0][1], s_t, k_b) }) * DEGR;
            two_bone_ik(rig, d, fk.rot, fk.pos, ua_l, fa_l, hl_n, &add3(sh_l, off_l), Some(&swivel_pole(off_l, ref_l, psi)), 1.0, rot.as_ref(), false);
        }
        // fingers: closed round the can, sprung open at the release, hanging loose after; the left hand firms up as it tucks in
        let (open, loose) = (smoothstep(rel - 0.025, rel + 0.03, t), smoothstep(0.34, 0.49, t));
        let c_r = |grip: f64, flat: f64, hng: f64| lerp(lerp(grip, flat, open), hng, loose);
        hand(d, hh, if off { 0 } else { 1 }, &[c_r(0.5, 0.05, 0.25), c_r(0.55, 0.08, 0.3), c_r(0.62, 0.12, 0.35), c_r(0.7, 0.15, 0.42)], c_r(0.7, 0.1, 0.3), 0.0, tune);
        let firm = bump(t, 0.22, 0.32, 0.5, 0.64);
        let c_l = |v: f64| lerp(v, v + 0.25, firm);
        if !low && !off {
            hand(d, hh, 0, &[c_l(0.3), c_l(0.35), c_l(0.4), c_l(0.45)], lerp(0.3, 0.6, firm), 0.0, tune);
        }
        // ---- in through per-arm masks: full weight but for a short hand-over of the elbow at each end; a carried-in restart is already at weight
        let w_r = (if cin.is_some() { 1.0 - smoothstep(0.62, dur, t) } else { bump(t, 0.0, 0.06, 0.62, dur) }) * give;
        let w_l = (if cin.is_some() { 1.0 - smoothstep(0.57, 0.72, t) } else { bump(t, 0.0, 0.09, 0.57, 0.72) }) * w_toss * give;
        (w_r, w_l)
    }) else {
        return 0;
    };
    // (the clavicles themselves are already posed in the pose; the copy carries the same turn)
    let (sl, sr) = (role(SHOULDER_L), role(SHOULDER_R));
    if off {
        ask_mask(&[(sl, w_r, true), (sr, 1.0, true), (sr, 0.0, false), (sl, 0.0, false)]);
    } else {
        ask_mask(&[(sr, w_r, true), (sl, w_l, true), (sr, 0.0, false), (sl, 0.0, false)]);
    }
    if !ask_blend(g, at, at_b, 1.0, true, at, n, tune) {
        return 0;
    }
    let left = &mut an.thr_last;
    let last = with_pose(g, at, |d, rig, _n, fk| {
        fk_run(rig, d, fk.rot, fk.pos);
        let (r, l) = (sub3(fk.at(role(HAND_R)), fk.at(role(UPPER_ARM_R))), sub3(fk.at(role(HAND_L)), fk.at(role(UPPER_ARM_L))));
        *left = Some(AnimThrLast { r, l });
    });
    if last.is_none() {
        return 0;
    }
    hands_to(&hs, an);
    RAN | RAN_ALL
}

// …and within a foot
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

/// Breathing, the body's mass on starts and stops, the weight shifting foot to foot at rest, and the standing foot lock: the feet planted in the world
/// while he stands, each stepping across in a lifted arc when the body has turned or shifted out from over it.
pub fn life_layer(g: usize, an: &mut Anim, dt: f64, s: f64, crouch_pose: f64, w: f64, w_f: f64, p0: [f64; 3], tune: &Knobs) -> u32 {
    let (at, yin, cr_t, lf) = (an.p_a.at as usize, an.body_yaw_in, an.cr_t, &mut an.life);
    let (f_w, f_next, mut foot) = (&mut an.feet.w, &mut an.feet.next, [an.feet.l, an.feet.r]);
    for f in foot.iter_mut() { f.lock_off = [0.0; 3]; f.lock_yaw = 0.0; }
    let code = with_pose(g, at, |d, rig, n, fk| {
        if !cast(n, &[HIPS, SPINE1, SPINE2, SPINE3, NECK, SHOULDER_L, SHOULDER_R, THIGH_L, SHIN_L, FOOT_L, THIGH_R, SHIN_R, FOOT_R]) {
            return 0;
        }
        // clocks
        if !finite(lf.yaw_prev) {
            lf.yaw_prev = yin;
        }
        let yr = if dt > 0.0 { wrap_angle(yin - lf.yaw_prev) / dt } else { 0.0 };
        lf.yaw_prev = yin;
        lf.yaw_rate = damp(lf.yaw_rate, clamp(yr, -8.0, 8.0), 8.0, dt);
        // the root's OWN planar motion: more than half a metre in a frame is a teleport — the feet re-lock where they land
        let mut teleported = false;
        if !finite(lf.prev_pos[0]) {
            lf.prev_pos = p0;
        } else {
            let dd = hypot2(p0[0] - lf.prev_pos[0], p0[2] - lf.prev_pos[2]);
            teleported = dd > 0.5;
            lf.root_v = if dt > 0.0 && !teleported {
                damp(lf.root_v, min(dd / dt, 8.0), 14.0, dt)
            } else if teleported {
                0.0
            } else {
                lf.root_v
            };
            lf.prev_pos = p0;
        }
        let hb = true;
        lf.exert = clamp(lf.exert + if s > 3.2 { dt / if hb { tune.hardbreath.chest_up } else { 4.0 } } else { -dt / if hb { tune.hardbreath.chest_down } else { 8.0 } }, 0.0, 1.0);
        let b_rate = lerp(LIFE.breath_rate, if hb { tune.hardbreath.chest_rate } else { LIFE.breath_rate_run }, lf.exert);
        let b_amp = lerp(LIFE.breath_amp, if hb { tune.hardbreath.chest_amp } else { LIFE.breath_amp_run }, lf.exert) * lerp(1.0, 0.35, clamp(s / 2.0, 0.0, 1.0)) * LIFE.on;
        lf.breath = (lf.breath + dt * b_rate * 2.0 * PI) % (2.0 * PI);
        let accel = if dt > 0.0 { (s - lf.prev_s) / dt } else { 0.0 };
        lf.prev_s = s;
        lf.accel = damp(lf.accel, clamp(accel, -12.0, 12.0), 10.0, dt);
        let t = clamp(-lf.accel * LIFE.mass_gain, -LIFE.mass_back, LIFE.mass_fwd) * LIFE.on;
        spring_step(&mut lf.mass, t, 9.0, dt);
        let t = clamp(-abs(lf.accel) * LIFE.mass_dip, -LIFE.mass_dip_max, 0.0) * LIFE.on;
        spring_step(&mut lf.mass_v, t, 10.0, dt);
        lf.still_t = if s < 0.12 && abs(lf.yaw_rate) < 0.5 { lf.still_t + dt } else { 0.0 };
        let resting = if lf.still_t > LIFE.rest_after && LIFE.on > 0.0 && !(cr_t > 0.5) { 1.0 } else { 0.0 };
        if resting != 0.0 {
            lf.shift_t -= dt;
            if lf.shift_t <= 0.0 {
                lf.shift_side = if lf.shift_side > 0.0 { -1.0 } else { 1.0 };
                lf.shift_n += 1.0;
                lf.shift_t = LIFE.shift_min + ((lf.shift_n * 0.618034) % 1.0) * LIFE.shift_span;
            }
        }
        let t = resting * lf.shift_side;
        spring_step(&mut lf.shift, t, if resting != 0.0 { LIFE.shift_in } else { LIFE.shift_out }, dt);
        if w_f < 0.003 {
            *f_w = 0.0;
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
            let (br, br2) = (sin(lf.breath), sin(lf.breath - 0.6));
            let sh = lf.shift.x * w * min(1.0, LIFE.on);
            let side = LIFE.shift_travel * sh * (1.0 - 0.5 * crouch_pose);
            let off = [side, lf.mass_v.x * w + 0.004 * b_amp * br * w - abs(sh) * 0.006, lf.mass.x * w * (1.0 - 0.4 * crouch_pose)];
            shift(d, rig, hips, &off);
            if abs(sh) > 1e-3 {
                turn(d, rig, hips, q_roll(-sh * LIFE.hip_drop * DEGR));
                turn(d, rig, s2, q_roll(sh * LIFE.spine_counter * DEGR));
                turn(d, rig, nk, q_roll(sh * LIFE.neck_counter * DEGR));
            }
            let tip = clamp(lf.accel * LIFE.tip_gain, -LIFE.tip_back, LIFE.tip_fwd) * DEGR * w * (1.0 - 0.5 * crouch_pose) * LIFE.on;
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
        let standing = if max(s, lf.root_v) < 0.22 && w_f > 0.5 && FEET.on > 0.5 && !(cr_t > 0.0 && cr_t < 1.0) { 1.0 } else { 0.0 };
        let w_was = *f_w;
        *f_w = damp(*f_w, standing, if standing != 0.0 { 12.0 } else { 16.0 + 40.0 * lf.root_v }, dt);
        if w < 0.003 && *f_w <= 0.01 {
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
        if *f_w > 0.01 {
            let want_p = [to_world(&fl), to_world(&fr)];
            let want_yaw = [yin + foot_yaw(&rl), yin + foot_yaw(&rr)];
            let relock = w_was <= 0.01 || teleported || dist_xz(&foot[0].p, &want_p[0]) > 0.45 || dist_xz(&foot[1].p, &want_p[1]) > 0.45;
            if relock {
                for i in 0..2 {
                    foot[i].p = want_p[i];
                    foot[i].yaw = want_yaw[i];
                    foot[i].swing = -1.0;
                }
            }
            let (swing, lift, dist, turn_at, pivot, lag_cap) = (FEET.swing, FEET.lift, FEET.dist, FEET.turn_deg * DEGR, FEET.pivot, FEET.lag_cap_deg * DEGR);
            let far_of = |foot: &[AnimFoot; 2], i: usize| -> bool { dist_xz(&foot[i].p, &want_p[i]) > dist || abs(wrap_angle(foot[i].yaw - want_yaw[i])) > turn_at };
            for i in 0..2 {
                if foot[i].swing < 0.0 {
                    let mut err_yaw = wrap_angle(want_yaw[i] - foot[i].yaw);
                    if abs(err_yaw) > lag_cap {
                        foot[i].yaw += sign(err_yaw) * (abs(err_yaw) - lag_cap);
                        err_yaw = sign(err_yaw) * lag_cap;
                    }
                    if abs(err_yaw) > turn_at {
                        foot[i].yaw += sign(err_yaw) * min(abs(err_yaw) - turn_at, pivot * dt);
                    }
                    let my_turn = *f_next == i as f64;
                    if far_of(&foot, i) && foot[1 - i].swing < 0.0 && (my_turn || !far_of(&foot, 1 - i)) {
                        foot[i].swing = 0.0;
                        foot[i].from = foot[i].p;
                        foot[i].from_yaw = foot[i].yaw;
                        *f_next = (1 - i) as f64;
                    }
                }
                if foot[i].swing >= 0.0 {
                    foot[i].swing += dt / swing;
                    let e = smoothstep(0.0, 1.0, min(1.0, foot[i].swing));
                    let from = foot[i].from;
                    let mut p = [lerp(from[0], want_p[i][0], e), lerp(from[1], want_p[i][1], e), lerp(from[2], want_p[i][2], e)];
                    p[1] = want_p[i][1] + sin(PI * min(1.0, foot[i].swing)) * lift;
                    foot[i].p = p;
                    foot[i].yaw = foot[i].from_yaw + wrap_angle(want_yaw[i] - foot[i].from_yaw) * e;
                    if foot[i].swing >= 1.0 {
                        foot[i].swing = -1.0;
                        foot[i].p = want_p[i];
                        foot[i].yaw = want_yaw[i];
                    }
                }
            }
            let mk = |f: &AnimFoot, rest_p: &[f64; 3], rest_r: &[f64; 4]| -> ([f64; 3], [f64; 4]) {
                let mp = to_model(&f.p);
                let fw = *f_w;
                let p = [lerp(rest_p[0], mp[0], fw), lerp(rest_p[1], mp[1], fw), lerp(rest_p[2], mp[2], fw)];
                let dyaw = wrap_angle((f.yaw - yin) - foot_yaw(rest_r)) * fw;
                (p, q_mul(&q_yaw(dyaw), rest_r))
            };
            (tl, ql) = mk(&foot[0], &fl, &rl);
            (tr, qr) = mk(&foot[1], &fr, &rr);
            // (feet_last reads the stack's feet without this)
            foot[0].lock_off = sub3(tl, fl);
            foot[1].lock_off = sub3(tr, fr);
            foot[0].lock_yaw = wrap_angle((foot[0].yaw - yin) - foot_yaw(&rl)) * *f_w;
            foot[1].lock_yaw = wrap_angle((foot[1].yaw - yin) - foot_yaw(&rr)) * *f_w;
        }
        // put the feet back: the knee steered over the toes only as far as the feet are LOCKED
        let pole_w = if cr_t > 0.0 { 0.6 * *f_w * (1.0 - smoothstep(0.0, 1.0, cr_t)) } else { 0.6 * *f_w };
        two_bone_ik(rig, d, fk.rot, fk.pos, role(THIGH_L), role(SHIN_L), foot_l, &tl, Some(&knee_pole(&ql)), pole_w, Some(&ql), false);
        two_bone_ik(rig, d, fk.rot, fk.pos, role(THIGH_R), role(SHIN_R), foot_r, &tr, Some(&knee_pole(&qr)), pole_w, Some(&qr), true);
        ran | RAN_ALL
    })
    .unwrap_or(0);
    (an.feet.l, an.feet.r) = (foot[0], foot[1]);
    code
}

/// THE RULE OF A FOOT, last of the stack: a foot that is down stays where it is; it goes somewhere else only through the air, and no faster than a foot goes.
/// It reads the feet the finished pose has, without the standing lock (life_layer's, which every layer above was posed over and goes on being),
/// and writes the thigh, the shin and the foot of each leg and nothing above the hips. `w_f`: the legs are his to stand on; `own`: how much of what the stack has in them is the stack's and not a whole-body clip's.
pub fn feet_last(g: usize, an: &mut Anim, dt: f64, s: f64, w_f: f64, own: f64, p0: [f64; 3]) -> u32 {
    let (at, yin, cr_t, f_w) = (an.p_a.at as usize, an.body_yaw_in, an.cr_t, an.feet.w);
    let (lock_off, lock_yaw) = ([an.feet.l.lock_off, an.feet.r.lock_off], [an.feet.l.lock_yaw, an.feet.r.lock_yaw]);
    let (was_known, was_at) = (an.feet.at_known, an.feet.at_was);
    let mut foot = [an.feet.step_l, an.feet.step_r];
    // his own motion over the ground: more than half a metre in a frame is a teleport, and the feet are set down where they land
    let p_was = if was_known { was_at } else { p0 };
    let dd = hypot2(p0[0] - p_was[0], p0[2] - p_was[2]);
    let teleported = dd > 0.5;
    let went = if dd > 1e-6 { [(p0[0] - p_was[0]) / dd, (p0[2] - p_was[2]) / dd] } else { [0.0, 0.0] };
    (an.feet.at_known, an.feet.at_was) = (true, p0);
    let code = with_pose(g, at, |d, rig, n, fk| {
        if !cast(n, &[HIPS, THIGH_L, SHIN_L, FOOT_L, TOE_L, THIGH_R, SHIN_R, FOOT_R, TOE_R]) {
            return 0;
        }
        if w_f < 0.003 {
            for f in foot.iter_mut() { f.down = false; f.swing = -1.0; f.off = [0.0; 3]; f.off_v = [0.0; 3]; f.off_yaw = 0.0; f.fresh = true; }
            return RAN;
        }
        let active = w_f > 0.5 && FEET.on > 0.5;
        if !active && foot.iter().all(|f| f.swing < 0.0 && f.off == [0.0; 3] && f.off_yaw == 0.0) {
            return RAN;
        }
        let (foot_l, foot_r) = (role(FOOT_L), role(FOOT_R));
        let legs = list(FK_LEGS);
        fk.part(rig, d, legs);
        let ran = RAN | ran_list(FK_LEGS);
        // the feet the stack has, the standing lock taken back out of them
        let k = if active { own } else { 0.0 };
        let without = |i: usize, a: [f64; 3], t: [f64; 3], r: [f64; 4]| -> ([f64; 3], [f64; 3], [f64; 4]) {
            let un = q_yaw(-lock_yaw[i] * k);
            let o = lock_off[i];
            let at = [a[0] - o[0] * k, a[1] - o[1] * k, a[2] - o[2] * k];
            (at, add3(at, q_rotate(&un, &sub3(t, a))), q_mul(&un, &r))
        };
        let (fl, toe_pose_l, rl) = without(0, fk.at(foot_l), fk.at(role(TOE_L)), fk.rot(foot_l));
        let (fr, toe_pose_r, rr) = without(1, fk.at(foot_r), fk.at(role(TOE_R)), fk.rot(foot_r));
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
        let (ankle, toe, rot) = ([fl, fr], [toe_pose_l, toe_pose_r], [rl, rr]);
        let want_p = [to_world(&fl), to_world(&fr)];
        let want_toe = [to_world(&toe_pose_l), to_world(&toe_pose_r)];
        let want_yaw = [yin + foot_yaw(&rl), yin + foot_yaw(&rr)];
        let (swing, lift, turn_at, pivot, lag_cap) = (FEET.swing, FEET.lift, FEET.turn_deg * DEGR, FEET.pivot, FEET.lag_cap_deg * DEGR);
        // how far the pose may want a foot from where it stands before it steps: a stand's small shuffle, a stride's reach
        let reach_now = lerp(FEET.dist, FEET.reach, min(s, 1.0));
        let far_of = |f: &AnimStep, i: usize| -> f64 {
            let d = hypot2(f.off[0], f.off[2]);
            if d > reach_now || abs(wrap_angle(f.yaw - want_yaw[i])) > turn_at { d + 1e-3 } else { 0.0 }
        };
        // from the ankle to the toe over the ground, turned as far as the foot is off the pose's way
        let lever = |i: usize, off_yaw: f64| -> [f64; 3] {
            let (dx, dz) = (want_toe[i][0] - want_p[i][0], want_toe[i][2] - want_p[i][2]);
            let (c, s_) = (cos(off_yaw), sin(off_yaw));
            [dx * c + dz * s_, 0.0, dz * c - dx * s_]
        };
        for i in 0..2 {
            // the cycle under the foot: at the floor or lifted, going forward over the ground or not
            let (ay_, ty_) = (ankle[i][1], toe[i][1]);
            let toe_down = ty_ < TOE_FLOOR_Y + FEET.floor_off;
            let at_floor = toe_down || ay_ < FEET.ankle_y + FEET.floor_off;
            let lifted = !at_floor && ty_ > TOE_FLOOR_Y + FEET.lifted;
            let v = if dt > 1e-5 { ((want_toe[i][0] - foot[i].toe_was[0]) * went[0] + (want_toe[i][2] - foot[i].toe_was[2]) * went[1]) / dt } else { 0.0 };
            if dt > 1e-5 { foot[i].toe_was = want_toe[i]; }
            let going = s > FEET.going_from && v > FEET.going_k * s;
            if teleported {
                foot[i].down = false;
                foot[i].swing = -1.0;
                foot[i].off = [0.0; 3];
                foot[i].off_v = [0.0; 3];
                foot[i].off_yaw = 0.0;
                foot[i].fresh = true;
            }
            // where its pivot stood in the last frame's picture: what it holds from now on
            let pivot_at = |f: &AnimStep, on_toe: bool| -> [f64; 3] {
                if f.fresh { return if on_toe { want_toe[i] } else { want_p[i] }; }
                if on_toe { add3(f.shown, lever(i, f.off_yaw)) } else { f.shown }
            };
            if dt > 1e-5 {
                if !active {
                    // a layer has the legs: whatever the foot was doing, it eases into the pose
                    if foot[i].swing >= 0.0 || foot[i].down {
                        foot[i].swing = -1.0;
                        foot[i].down = false;
                        foot[i].off_v = [0.0; 3];
                    }
                } else if foot[i].swing < 0.0 {
                    if foot[i].down && (lifted || going) {
                        // the cycle has it: it leaves with what it is off by, and that eases out
                        foot[i].down = false;
                        foot[i].off_v = [0.0; 3];
                    } else if !foot[i].down && at_floor && !going {
                        foot[i].down = true;
                        foot[i].toe_down = toe_down;
                        foot[i].p = pivot_at(&foot[i], toe_down);
                        foot[i].yaw = want_yaw[i] + foot[i].off_yaw;
                    }
                }
            }
            // over the ground: down it holds; let go what it is off by eases out, critically damped and at no more than a foot's speed. Up and down is always the pose's, eased.
            let (mut off, mut off_v) = (foot[i].off, foot[i].off_v);
            let holds = foot[i].swing < 0.0 && foot[i].down;
            for c in 0..3 {
                if holds && c != 1 { continue; }
                let mut sp = Spring { x: off[c], v: off_v[c] };
                spring_step(&mut sp, 0.0, FEET.ease, dt);
                off[c] = sp.x;
                off_v[c] = sp.v;
            }
            foot[i].off = off;
            foot[i].off_v = off_v;
            if foot[i].swing < 0.0 && foot[i].down {
                // it holds its place, on the toe, or on the heel while the toe is still coming down; its way within what the ankle takes
                if foot[i].toe_down != toe_down {
                    foot[i].p = pivot_at(&foot[i], toe_down);
                    foot[i].toe_down = toe_down;
                }
                let mut err_yaw = wrap_angle(want_yaw[i] - foot[i].yaw);
                if abs(err_yaw) > lag_cap {
                    foot[i].yaw += sign(err_yaw) * (abs(err_yaw) - lag_cap);
                    err_yaw = sign(err_yaw) * lag_cap;
                }
                if abs(err_yaw) > turn_at {
                    foot[i].yaw += sign(err_yaw) * min(abs(err_yaw) - turn_at, pivot * dt);
                }
                foot[i].off_yaw = wrap_angle(foot[i].yaw - want_yaw[i]);
                let under = if toe_down { add3(want_p[i], lever(i, foot[i].off_yaw)) } else { want_p[i] };
                foot[i].off[0] = foot[i].p[0] - under[0];
                foot[i].off[2] = foot[i].p[2] - under[2];
                foot[i].off_v[0] = 0.0;
                foot[i].off_v[2] = 0.0;
            } else if foot[i].swing < 0.0 {
                foot[i].off_yaw = damp(foot[i].off_yaw, 0.0, FEET.ease * 0.5, dt);
                if abs(foot[i].off_yaw) < 1e-4 { foot[i].off_yaw = 0.0; }
                if off.iter().all(|x| abs(*x) < 1e-4) && off_v.iter().all(|x| abs(*x) < 1e-3) { foot[i].off = [0.0; 3]; foot[i].off_v = [0.0; 3]; }
                foot[i].yaw = want_yaw[i] + foot[i].off_yaw;
            }
        }
        // a foot down that the pose wants far from it steps, the one further off first; the other may start as this one comes to its last third
        if active && dt > 1e-5 {
            for _ in 0..2 {
                let d = [if foot[0].swing < 0.0 && foot[0].down { far_of(&foot[0], 0) } else { 0.0 }, if foot[1].swing < 0.0 && foot[1].down { far_of(&foot[1], 1) } else { 0.0 }];
                let i = if d[0] >= d[1] { 0 } else { 1 };
                if d[i] <= 0.0 || (foot[1 - i].swing >= 0.0 && foot[1 - i].swing < 2.0 / 3.0) {
                    break;
                }
                let f = &mut foot[i];
                f.from = add3(want_p[i], f.off);
                f.swing = 0.0;
                f.from_yaw = f.yaw;
                f.down = false;
            }
        }
        let mut shown = [want_p[0], want_p[1]];
        for i in 0..2 {
            if foot[i].swing >= 0.0 {
                // through the air: the further off past what a step takes, the quicker the step, so a fast turn is quick steps and the foot gets there
                foot[i].swing += dt / swing * max(1.0, dist_xz(&foot[i].from, &want_p[i]) / FEET.reach);
                let e = smoothstep(0.0, 1.0, min(1.0, foot[i].swing));
                let from = foot[i].from;
                shown[i] = [lerp(from[0], want_p[i][0], e), want_p[i][1] + sin(PI * min(1.0, foot[i].swing)) * lift, lerp(from[2], want_p[i][2], e)];
                foot[i].off = [shown[i][0] - want_p[i][0], 0.0, shown[i][2] - want_p[i][2]];
                foot[i].off_v = [0.0; 3];
                foot[i].yaw = foot[i].from_yaw + wrap_angle(want_yaw[i] - foot[i].from_yaw) * e;
                foot[i].off_yaw = wrap_angle(foot[i].yaw - want_yaw[i]);
                if foot[i].swing >= 1.0 {
                    // set down where it has come to
                    foot[i].swing = -1.0;
                    foot[i].off = [0.0; 3];
                    foot[i].off_v = [0.0; 3];
                    foot[i].off_yaw = 0.0;
                    foot[i].yaw = want_yaw[i];
                }
            } else {
                shown[i] = add3(want_p[i], foot[i].off);
            }
            // a foot not down goes no faster than a foot goes, seen from where he stands: what the pose asks past that it comes to by the same easing. Down, it is the body that moves.
            let free = !(foot[i].swing < 0.0 && foot[i].down);
            if dt > 1e-5 && free && !foot[i].fresh && !teleported {
                let d = sub3(sub3(shown[i], p0), sub3(foot[i].shown, p_was));
                let (l, most) = (hypot3(d[0], d[1], d[2]), min(FEET.top_speed + FEET.top_speed_k * s, FEET.top_speed_most) * dt);
                if l > most {
                    let k = most / l;
                    shown[i] = [shown[i][0] - d[0] * (1.0 - k), shown[i][1] - d[1] * (1.0 - k), shown[i][2] - d[2] * (1.0 - k)];
                    if foot[i].swing < 0.0 { foot[i].off = sub3(shown[i], want_p[i]); foot[i].off_v = [0.0; 3]; }
                }
            }
            foot[i].shown = shown[i];
            foot[i].fresh = false;
        }
        // and it turns no faster than a foot turns, seen in the world: a pose that throws a foot over all at once is met over a few frames
        let (body, body_back) = (q_yaw(yin), q_yaw(-yin));
        let mut placed = [(fl, rl), (fr, rr)];
        for i in 0..2 {
            let dyaw = wrap_angle((foot[i].yaw - yin) - foot_yaw(&rot[i]));
            let mut q = q_mul(&body, &q_mul(&q_yaw(dyaw), &rot[i]));
            if dt > 1e-5 && !teleported && foot[i].q_was != [0.0; 4] {
                let a = 2.0 * acos(min(1.0, abs(q[0] * foot[i].q_was[0] + q[1] * foot[i].q_was[1] + q[2] * foot[i].q_was[2] + q[3] * foot[i].q_was[3])));
                let most = FEET.top_turn * dt;
                if a > most { q = slerp(&foot[i].q_was, &q, most / a); }
            }
            foot[i].q_was = q;
            placed[i] = (to_model(&shown[i]), q_mul(&body_back, &q));
        }
        let ((tl, ql), (tr, qr)) = (placed[0], placed[1]);
        // the knee over the toes only as far as he stands
        let pole_w = if cr_t > 0.0 { 0.6 * f_w * (1.0 - smoothstep(0.0, 1.0, cr_t)) } else { 0.6 * f_w };
        two_bone_ik(rig, d, fk.rot, fk.pos, role(THIGH_L), role(SHIN_L), foot_l, &tl, Some(&knee_pole(&ql)), pole_w, Some(&ql), false);
        two_bone_ik(rig, d, fk.rot, fk.pos, role(THIGH_R), role(SHIN_R), foot_r, &tr, Some(&knee_pole(&qr)), pole_w, Some(&qr), true);
        ran | RAN_ALL
    })
    .unwrap_or(0);
    (an.feet.step_l, an.feet.step_r) = (foot[0], foot[1]);
    code
}
