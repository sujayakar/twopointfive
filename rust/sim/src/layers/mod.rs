//! THE ANIMATOR'S LAYERS: the steps of a man's update that pose him, each written once, here. A layer takes his animation as it is (`an`), the numbers the game is
//! tuned by (`tune`) where it asks one, and what only this call knows, reads and writes the pose WHERE IT LIVES (the game's pool, poses.rs) and the rig on the bench (skeleton.rs), and
//! asks the rig's named nodes of roles.rs. What it keeps from one tick to the next it keeps on him.
//!
//! A layer answers whether it ran: not = not a pose of this pool, or a role with no node (the caller stops, by name).
use game_types::character::{KICK_IMPACT, KICK_PLANT, LONG_ANCHOR_SIDE_MAX, LONG_PULL_MAX, READY_RATE, READY_REACH, THROW_DUR, THROW_RELEASE, TOE_FLOOR_Y, TOSS_AFTER, TOSS_LOW, TOSS_TOP, WALL_GAIT_REST, WALL_ROOT_OFF, BIGMOVE, BLOW, BOB, CREEP, FEET, GAZE, GAZE_EL, HAND, KNIFE_BLOW, LEAN_POSE, LIFE, LONG_PIVOT, LONG_STEADY, MV2, MV_FLINCH_ENV, MV_HIT_ENV, MV_STARTLE, REACH, READY_AZ, READY_BARE, READY_UP, RUN_LINGER, SMOKEARM, SNEAK, SOFT_KNEE, SPRINT_WARP, STEALTH, SWIV_REF_L, SWIV_REF_R, TAKEN, TOSS_HANG, WALL_POSE, WORK};
use game_types::rig_props::{Adj as RigAdj, SlingFrontHand, BIGRIG, RIG};
use game_types::character::{Anim, AnimBigTool, AnimFoot, AnimFootSpot, AnimStep, AnimHdFix, AnimKickFrom, AnimKickLeg, AnimLongGrip, AnimThrLast, AnimWtk, BlowRun, BlowStyle, Character, CharacterTn, DoorPush, HaulPose, HeldPose, HoldPhase, HoldPose, HoldVariant, KitItem, KnifeSlash, MvGait, ReadyKind, ReadyShape, SignalKind, Spring, Stance, TakenStyle, ThrowTables as Tables, WorkKind, WorkPose};
use game_types::gunreload::{FeedKey, FeedKeyAt, FeedKeyHow, FeedMarks};
use game_types::knobs::Knobs;
use game_types::m::{abs, max, min, round, sign, PI};
use game_types::v3::{self, clamp, damp, lerp, quat, smoothstep, wrap_angle};
use crate::roles::*;
use crate::maths::{acos, asin, atan2, cos, exp, hypot2, hypot3, pow, sin};
use prim::{floor, sqrt};
use crate::single::{get, put, S};
use crate::skeleton::{NODE, ask_blend, ask_mask, ask_sample, fk_run, parent_rot, rotate_local, rotate_model, set_model_rot, set_unit, two_bone_ik, with_pose, Fk, Rig};

mod big;
mod carry;
mod close;
mod fingers;
mod gait;
mod hold;
mod kit;
mod life;
mod react;
mod reload;
mod signals;
mod sneak;
mod stealth;
mod throw;
mod wall;
mod watch;
mod work;
pub use big::*;
pub use carry::*;
pub use close::*;
pub use fingers::*;
pub use gait::*;
pub use hold::*;
pub use kit::*;
pub use life::*;
pub use react::*;
pub use reload::*;
pub use signals::*;
pub use sneak::*;
pub use stealth::*;
pub use throw::*;
pub use wall::*;
pub use watch::*;
pub use work::*;

const DEGR: f64 = PI / 180.0;
#[inline(always)]
fn finite(x: f64) -> bool {
    abs(x) < f64::INFINITY
}
/// a spring's step on two doubles of a layer's state: the value, then its speed
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
const AX: [f64; 3] = [1.0, 0.0, 0.0];
const AY: [f64; 3] = [0.0, 1.0, 0.0];
/// the knee steered over the toes (the foot bone's Y runs along the foot)
fn knee_pole(foot_rot: &[f64; 4]) -> [f64; 3] {
    let f = quat::rotate(*foot_rot, AY);
    let h = v3::normalize([f[0], 0.0, f[2] + 1e-6]);
    v3::normalize([h[0] * 0.25, 0.5, h[2] * 0.25 + 0.75])
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
/// node i's origin moved by a model-space offset
fn shift(d: &mut [S], rig: &Rig, i: usize, off: &[f64; 3]) {
    let p = parent_rot(rig, d, i);
    let l = quat::rotate([-p[0], -p[1], -p[2], p[3]], *off);
    let (x, y, z) = (ch(d, i, 0) + l[0], ch(d, i, 1) + l[1], ch(d, i, 2) + l[2]);
    set_ch(d, i, 0, x);
    set_ch(d, i, 1, y);
    set_ch(d, i, 2, z);
}
/// a rotation about the model's X, Y, Z: the products with the axis's noughts are written out, since −0 is a number too
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
/// node i (and its subtree) about model axes, in place
#[inline(always)]
fn turn(d: &mut [S], rig: &Rig, i: usize, q: [f64; 4]) {
    let par = parent_rot(rig, d, i);
    rotate_model(d, i, &q, &par);
}
/// do these roles have nodes of a rig of `n` nodes
fn cast(n: usize, roles: &[usize]) -> bool {
    roles.iter().all(|&r| role(r) < n)
}
/// the pelvis moved by `off` (model space) with both feet held where the pose under it has them (the legs of `fk` current on entry
/// and on exit)
fn mv_hips(d: &mut [S], rig: &Rig, fk: &mut Fk, off: &[f64; 3]) {
    let (foot_l, foot_r) = (role(FOOT_L), role(FOOT_R));
    let (fl, fr, rl, rr) = (fk.at(foot_l), fk.at(foot_r), fk.rot(foot_l), fk.rot(foot_r));
    shift(d, rig, role(HIPS), off);
    fk.part(rig, d, list(FK_LEGS));
    two_bone_ik(rig, d, fk.rot, fk.pos, role(THIGH_L), role(SHIN_L), foot_l, &fl, Some(&knee_pole(&rl)), 0.0, Some(&rl), true);
    two_bone_ik(rig, d, fk.rot, fk.pos, role(THIGH_R), role(SHIN_R), foot_r, &fr, Some(&knee_pole(&rr)), 0.0, Some(&rr), true);
}
const LEG_ROLES: [usize; 7] = [HIPS, THIGH_L, SHIN_L, FOOT_L, THIGH_R, SHIN_R, FOOT_R];
#[inline(always)]
fn bump(t: f64, a: f64, b: f64, c: f64, d: f64) -> f64 {
    smoothstep(a, b, t) * (1.0 - smoothstep(c, d, t))
}
/// `!x` of a number: nothing or no number
#[inline(always)]
fn falsy(x: f64) -> bool {
    x == 0.0 || x != x
}
/// where a point must be posed below spine3 to END UP at `h` after the chest twist still to come (the run-forward bones current)
fn untwist(fk: &Fk, tw_y: f64, tw_p: f64, h: [f64; 3]) -> [f64; 3] {
    if falsy(tw_y) && falsy(tw_p) {
        return h;
    }
    let (e1, e2) = (q_yaw(tw_y * 0.25), q_yaw(tw_y * 0.35));
    let mut e3 = q_yaw(tw_y * 0.4);
    if !falsy(tw_p) {
        let right = quat::rotate(e3, AX);
        e3 = quat::mul(quat::axis_angle(right, tw_p), e3);
    }
    let (ie1, ie2, ie3) = (quat::conj(e1), quat::conj(e2), quat::conj(e3));
    let (p1, p2, p3) = (fk.at(role(SPINE1)), fk.at(role(SPINE2)), fk.at(role(SPINE3)));
    let p2t = v3::add(quat::rotate(e1, v3::sub(p2, p1)), p1);
    let p3t = v3::add(quat::rotate(e2, v3::sub(v3::add(quat::rotate(e1, v3::sub(p3, p1)), p1), p2t)), p2t);
    let x1 = v3::add(quat::rotate(ie3, v3::sub(h, p3t)), p3t);
    let x2 = v3::add(quat::rotate(ie2, v3::sub(x1, p2t)), p2t);
    v3::add(quat::rotate(ie1, v3::sub(x2, p1)), p1)
}
/// the neck and head turned by `w` of the way from where the eyes look to `target` (model space); runs the pose forward first
fn look_at(d: &mut [S], rig: &Rig, fk: &mut Fk, gaze_local: &[f64; 3], target: [f64; 3], w: f64, roll: f64) {
    fk_run(rig, d, fk.rot, fk.pos);
    let (nk, hd) = (role(NECK), role(HEAD));
    let hr = fk.rot(hd);
    let gaze = quat::rotate(hr, *gaze_local);
    let eyes = v3::mad(v3::mad(fk.at(hd), quat::rotate(hr, AY), 0.1), gaze, 0.08);
    let want = v3::normalize(v3::sub(target, eyes));
    let yaw_e = clamp(wrap_angle(atan2(want[0], want[2]) - atan2(gaze[0], gaze[2])), -0.7, 0.7);
    let pitch_e = clamp(asin(clamp(gaze[1], -1.0, 1.0)) - asin(clamp(want[1], -1.0, 1.0)), -0.6, 0.8);
    let look = |kk: f64| quat::mul(q_yaw(yaw_e * kk * w), q_pitch(pitch_e * kk * w));
    turn(d, rig, nk, look(0.4));
    turn(d, rig, hd, if !falsy(roll) { quat::mul(q_roll(roll), look(0.6)) } else { look(0.6) });
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
/// `x || 1` of a length: 1 where it is nothing or no number
#[inline(always)]
fn or_one(l: f64) -> f64 {
    if l == 0.0 || l != l {
        1.0
    } else {
        l
    }
}
/// a node's own rotation in a pose, read and put
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
/// A CLIP A LAYER SAMPLES FOR ITSELF: the rest pose copied into a scratch pose of the same character and the
/// clip sampled over it. `clip` is the module's number for it, handed with the layer's own numbers (game_rules anim_skeleton `clip_no`); false = no such clip or pose
fn sample_on_rest(g: usize, at_rest: usize, at_out: usize, nodes: usize, clip: f64, t: f64, looped: bool) -> bool {
    clip >= 0.0 && ask_blend(g, at_rest, at_rest, 1.0, false, at_out, nodes) && ask_sample(g, at_out, nodes, clip as usize, t, looped)
}
// ================================================================ THE ARMS
// An arm is posed on a COPY of the pose (a second pose of the pool) and laid back through a mask: the copy is made with `ask_blend` at
// weight 1 (singles copied, none worked out), the limbs are solved on it, the mask is written from the named joints and
// the copy is blended in. The run-forward bones are then the COPY's.

/// both arms from the collar bones down
fn arms_mask() {
    ask_mask(&[(role(SHOULDER_L), 1.0, true), (role(SHOULDER_R), 1.0, true)]);
}
const ARM_ROLES: [usize; 9] = [SPINE3, SHOULDER_L, SHOULDER_R, UPPER_ARM_L, UPPER_ARM_R, FOREARM_L, FOREARM_R, HAND_L, HAND_R];
/// the rotation that carries the aim clip's grip (bore `g0`, gun hand at model rotation `rr0`) onto the wanted bore `gd` — the
/// shortest arc, then unrolled about the bore so the pistol's top strap (`grip_up`, in the hand) faces `up_ref` as nearly as the bore allows
fn grip_turn(g0: &[f64; 3], rr0: &[f64; 4], gd: &[f64; 3], up_ref: &[f64; 3], grip_up: &[f64; 3]) -> [f64; 4] {
    let mut turn = quat::from_to(*g0, *gd);
    let up0 = quat::rotate(quat::mul(turn, *rr0), *grip_up);
    let (want, cur) = (v3::sub(*up_ref, v3::scale(*gd, v3::dot(*up_ref, *gd))), v3::sub(up0, v3::scale(*gd, v3::dot(up0, *gd))));
    if v3::len(want) > 1e-4 && v3::len(cur) > 1e-4 {
        let (wn, cn) = (v3::normalize(want), v3::normalize(cur));
        turn = quat::mul(quat::axis_angle(*gd, atan2(v3::dot(v3::cross(cn, wn), *gd), v3::dot(cn, wn))), turn);
    }
    turn
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
/// a double as a single keeps it
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
/// curl the four fingers (0 straight … 1 fist) and set the thumb (0 alongside … 1 folded over the fist) of one hand, from the rest
/// (straight) hand; then the hand pass's note: a layer has set this hand, and how far its shape stands from where the locomotion left each digit.
fn hand(d: &mut [S], h: &mut Hands, side: usize, curl: &[f64; 4], thumb: f64, spread: f64) {
    let (ns, at) = (digits(side), side * HAND_DIGITS);
    for fi in 0..4 {
        for j in 0..3 {
            let mut q = q_pitch(curl[fi] * FIST[j] * DEGR);
            if j == 0 && !falsy(spread) {
                // fan about the knuckle's palm-normal axis
                q = quat::mul(q_roll((fi as f64 - 1.2) * spread * (if side == 1 { 1.0 } else { -1.0 }) * 9.0 * DEGR), q);
            }
            let k = 3 + fi * 3 + j;
            set_unit(d, ns[k] as usize, &quat::mul(h.rest[at + k], q));
        }
    }
    for j in 0..3 {
        set_unit(d, ns[j] as usize, &quat::mul(h.rest[at + j], q_pitch(thumb * THUMB_FIST[j] * DEGR)));
    }
    let b = &h.base[side * 60..side * 60 + 60];
    for g in 0..5 {
        let k = g * 3;
        let far = (q_ang(d, ns[k] as usize, b, k * 4) + q_ang(d, ns[k + 1] as usize, b, k * 4 + 4) + q_ang(d, ns[k + 2] as usize, b, k * 4 + 8)) / 3.0;
        h.note[side * 5 + g] = stored(min(h.note[side * 5 + g], far));
    }
}
/// the rotation whose axes are x, y, z
fn from_basis(x: &[f64; 3], y: &[f64; 3], z: &[f64; 3]) -> [f64; 4] {
    let (m00, m10, m20, m01, m11, m21, m02, m12, m22) = (x[0], x[1], x[2], y[0], y[1], y[2], z[0], z[1], z[2]);
    let tr = m00 + m11 + m22;
    quat::normalize(if tr > 0.0 {
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
/// which way a foot points.
fn foot_yaw(r: &[f64; 4]) -> f64 {
    let f = quat::rotate(*r, AY);
    atan2(f[0], f[2])
}
/// where he stands and which way he faces: points between his own axes and the world
#[derive(Clone, Copy, Default)]
struct Stands {
    p0: [f64; 3],
    cy: f64,
    sy: f64,
}
impl Stands {
    fn at(p0: [f64; 3], yaw: f64) -> Stands {
        Stands { p0, cy: cos(yaw), sy: sin(yaw) }
    }
    /// a point of his in the world
    fn world(&self, m: &[f64; 3]) -> [f64; 3] {
        let Stands { p0, cy, sy } = *self;
        [p0[0] + m[0] * cy + m[2] * sy, p0[1] + m[1], p0[2] - m[0] * sy + m[2] * cy]
    }
    /// a point of the world in his own axes
    fn model(&self, wp: &[f64; 3]) -> [f64; 3] {
        let Stands { p0, cy, sy } = *self;
        let (dx, dz) = (wp[0] - p0[0], wp[2] - p0[2]);
        [dx * cy - dz * sy, wp[1] - p0[1], dx * sy + dz * cy]
    }
}
/// the model rotation of a hand whose fingers point along `f` with the back of the hand to `b` (made square to f); `fix` is that
/// side's calibration from the rest pose (handed in with the layer's numbers)
fn hand_rot(fix: &[f64; 4], f: [f64; 3], b: [f64; 3]) -> [f64; 4] {
    let ff = v3::normalize(f);
    let bb = v3::normalize(v3::mad(b, ff, -v3::dot(b, ff)));
    quat::mul(from_basis(&v3::cross(ff, bb), &ff, &bb), *fix)
}
/// the trunk's own left, up and forward, off the run-forward bones
fn trunk_frame(fk: &Fk) -> ([f64; 3], [f64; 3], [f64; 3]) {
    let up = v3::normalize(v3::sub(fk.at(role(SPINE3)), fk.at(role(HIPS))));
    let l = v3::sub(fk.at(role(UPPER_ARM_L)), fk.at(role(UPPER_ARM_R)));
    let left = v3::normalize(v3::mad(l, up, -v3::dot(l, up)));
    (left, up, v3::cross(left, up))
}
pub const LC_COUNT: usize = 21;
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
        let (q, par) = (quat::slerp(*rot0, fk.rot(hand_n), wr), parent_rot(rig, d, hand_n));
        set_model_rot(d, hand_n, &q, &par);
    }
}
/// the right hand's turn from where the blade is to point and which way the back of the hand faces (squared to the blade here)
fn knife_rot(fix_r: &[f64; 4], blade: [f64; 3], back: [f64; 3]) -> [f64; 4] {
    let t = v3::normalize(blade);
    let b = v3::normalize(v3::mad(back, t, -v3::dot(back, t)));
    hand_rot(fix_r, v3::cross(t, b), b)
}
/// where the right wrist goes, the fist's turn and the elbow's pole with the fist closed on the sheathed handle, in the chest
/// joint's frame off the rest pose's (`rc_inv`: the rest chest's turn, inverted). `pull` m back down the knife's line, `off` m further off the chest.
fn sheath_fist(fk: &Fk, rc_inv: &[f64; 4], fix_r: &[f64; 4], crouch: f64, pull: f64, off: f64) -> ([f64; 3], [f64; 4], [f64; 3]) {
    let s3 = role(SPINE3);
    let q = quat::mul(fk.rot(s3), *rc_inv);
    let t = v3::normalize(quat::rotate(q, RIG.sheath.along));
    let b0 = quat::rotate(q, RIG.sheath.out);
    let b = v3::normalize(v3::mad(b0, t, -v3::dot(b0, t)));
    let f = v3::cross(t, b);
    let h = v3::mad(v3::mad(v3::add(fk.at(s3), quat::rotate(q, RIG.sheath.at)), t, -pull), b, off);
    let to = v3::mad(v3::mad(v3::mad(h, f, -RIG.knife.grip.fwd), b, RIG.knife.grip.palm), t, -RIG.knife.grip.side);
    let pole = v3::mad(v3::normalize(quat::rotate(q, REACH.pole)), AY, lerp(REACH.pole_up[0], REACH.pole_up[1], clamp(crouch, 0.0, 1.0)));
    (to, knife_rot(fix_r, t, b), pole)
}
/// the support arm from the upper arm down (the collar bone stays the clip's), or both arms
fn arm_mask(both: bool) {
    ask_mask(&[(role(UPPER_ARM_L), 1.0, true), (if both { role(UPPER_ARM_R) } else { usize::MAX }, 1.0, true)]);
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
/// quat.yaw: about the model's up, the noughts plain
#[inline(always)]
fn yaw_of(a: f64) -> [f64; 4] {
    [0.0, sin(a / 2.0), 0.0, cos(a / 2.0)]
}
/// a twist the skeleton lays on after the layers: the yaw, then the pitch about the level axis the yaw has turned
fn twist_of(y: f64, p: f64) -> [f64; 4] {
    let mut q = yaw_of(y);
    if p != 0.0 {
        q = quat::mul(quat::axis_angle(quat::rotate(q, AX), p), q);
    }
    q
}
#[inline(always)]
fn ease(c: &mut [f64], i: usize, v: f64, q: f64) {
    c[i] += (v - c[i]) * q;
}
/// a direction taken back through the chest twist still to come
fn untwist_dir(tw_y: f64, tw_p: f64, dv: [f64; 3]) -> [f64; 3] {
    if falsy(tw_y) && falsy(tw_p) {
        return dv;
    }
    let (e1, e2) = (q_yaw(tw_y * 0.25), q_yaw(tw_y * 0.35));
    let mut e3 = q_yaw(tw_y * 0.4);
    if !falsy(tw_p) {
        let right = quat::rotate(e3, AX);
        e3 = quat::mul(quat::axis_angle(right, tw_p), e3);
    }
    quat::rotate(quat::mul(quat::conj(e1), quat::mul(quat::conj(e2), quat::conj(e3))), dv)
}
/// the big layer's ease: a share clamped to 0‥1 and smoothed
fn e3(u: f64) -> f64 {
    let u = clamp(u, 0.0, 1.0);
    u * u * (3.0 - 2.0 * u)
}
#[cfg(test)]
mod tests {
    use super::*;

    /// Every carriage and sway rides on these: a follower that passed its mark would show as a wobble, and one that went by the frame would change with the machine.
    #[test]
    fn a_follower_gets_there_without_passing_it_at_any_frame_rate() {
        let mut s = Spring { x: 0.3, v: 0.0 };
        let mut last = s.x;
        for _ in 0..120 {
            spring_step(&mut s, 1.0, 16.0, 1.0 / 60.0);
            assert!(s.x >= last - 1e-9 && s.x <= 1.0 + 1e-9, "it went from {last} to {}", s.x);
            last = s.x;
        }
        assert!((last - 1.0).abs() < 1e-3, "two seconds on it is at {last}");
        // one long frame or four short ones: the same place and the same speed
        let (mut one, mut four) = (Spring { x: 0.0, v: 2.0 }, Spring { x: 0.0, v: 2.0 });
        spring_step(&mut one, 1.0, 16.0, 1.0 / 15.0);
        for _ in 0..4 { spring_step(&mut four, 1.0, 16.0, 1.0 / 60.0); }
        assert!((one.x - four.x).abs() < 1e-9 && (one.v - four.v).abs() < 1e-9, "{} {} against {} {}", one.x, one.v, four.x, four.v);
    }
}
