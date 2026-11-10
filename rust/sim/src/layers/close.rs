//! A kick, a blow, a knife, and the pose that says one can be thrown.
use super::*;
use rd_row::*;

/// a key of a blow: when, where the wrist is, the line wrist to knuckles, the back of the hand, the elbow's pole
type BlowKey = (f64, [f64; 3], [f64; 3], [f64; 3], [f64; 3]);

/// where each of the ready pose's eased numbers stands in its row
mod rd_row {
    pub const R_L: usize = 0;
    pub const R_R: usize = 3;
    pub const L_POLE: usize = 6;
    pub const R_POLE: usize = 9;
    pub const L_FING: usize = 12;
    pub const L_BACK: usize = 15;
    pub const R_FING: usize = 18;
    pub const R_BACK: usize = 21;
    pub const LEAN: usize = 24;
    pub const BLADE: usize = 25;
    pub const W_R: usize = 26;
    pub const L_CURL: usize = 27;
    pub const L_THUMB: usize = 28;
    pub const L_SPREAD: usize = 29;
    pub const R_CURL: usize = 30;
    pub const R_THUMB: usize = 31;
    pub const CLAV: usize = 32;
    pub const INTO: usize = 33;
    pub const UP: usize = 35;
}

/// The kick's body half: the pelvis's shift, dip, drive, lean back, counter-yaw and hike as overlapping curves, the spine and the head countering so the
/// eyes stay on the door, the swing foot flown along chamber, strike, recoil and plant by IK, the other foot held from the first frame. Built on the
/// scratch pose at `at_c` from the pose at `at`, and laid on that one by w. The left leg's kick is the same kick in a mirror.
pub fn kick_body(g: usize, an: &mut Anim, t: f64, w: f64) -> u32 {
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
        turn(d, rig, hips, quat::mul(q_yaw(hyaw), quat::mul(q_roll(roll), q_pitch(tilt))));
        // the spine and the head
        let lay = curve(t, &[[0.0, 0.0], [0.2, 3.0], [0.39, -8.0], [0.47, -3.5], [0.6, -2.5], [0.85, 0.0]]) * DEGR;
        turn(d, rig, s1, quat::mul(q_yaw(-hyaw * 0.3), quat::mul(q_roll(-roll * 0.4), q_pitch(lay))));
        turn(d, rig, s2, quat::mul(q_yaw(-hyaw * 0.3), quat::mul(q_roll(-roll * 0.4), q_pitch(lay * 0.7))));
        turn(d, rig, s3, q_yaw(-hyaw * 0.2));
        turn(d, rig, nk, q_pitch(-(tilt + lay * 1.7) * 0.45));
        turn(d, rig, hd, q_pitch(-(tilt + lay * 1.7) * 0.55));
        fk_run(rig, d, fk.rot, fk.pos);
        // the swing foot: a moving origin (its own spot to the idle's) and offsets off it
        let o = v3::lerp3(ks, fs, smoothstep(0.0, 0.3, t));
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
        rotate_local(d, ft_s, &quat::axis_angle(AX, curve(t, &[[0.0, 0.0], [0.2, 14.0], [0.34, -16.0], [0.42, -18.0], [0.58, 4.0], [0.68, 0.0]]) * DEGR));
        let flat = smoothstep(0.6, kick_plant, t);
        if flat > 0.0 {
            let (sh, own) = (fk.rot(sh_s), own_rot(d, ft_s));
            set_model_rot(d, ft_s, &quat::nlerp(quat::mul(sh, own), fsr, flat), &sh);
        }
        // the planted foot: held, and set down on the idle's spot in one short low step if the kick found it off it
        let e = smoothstep(0.0, 0.24, t);
        let far = clamp(hypot2(kp[0] - fp[0], kp[2] - fp[2]) / 0.2, 0.0, 1.0);
        let mut pin = v3::lerp3(kp, fp, e);
        pin[1] += 0.05 * sin(PI * e) * far;
        fk_run(rig, d, fk.rot, fk.pos);
        two_bone_ik(rig, d, fk.rot, fk.pos, th_p, sh_p, ft_p, &pin, Some(&knee_pole(&fpr)), 0.6 * smoothstep(0.0, 0.15, t), Some(&quat::nlerp(kpr, fpr, e)), false);
        RAN | RAN_ALL
    })
    .unwrap_or(0);
    if code == 0 || !ask_blend(g, at, at_c, w, false, at, n) {
        return 0;
    }
    code
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
            let len = (v3::dist(a, b) + v3::dist(b, c)) * BLOW.legs.straight;
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
        let q = quat::mul(q_yaw(yw * 0.5 * DEGR), q_pitch(pt * 0.5 * DEGR));
        turn(d, rig, role(SPINE2), q);
        turn(d, rig, role(SPINE3), q);
        turn(d, rig, role(NECK), q_pitch(-pt * 0.5 * DEGR));
        RAN
    })
    .unwrap_or(0)
}
/// What the steps of a blow hand on to the ones after them.
#[derive(Clone, Copy, Default)]
struct Blow {
    at1: [f64; 3],
    at2: Option<[f64; 3]>,
    tw_y: f64,
    tw_p: f64,
    // as_thrown: the blow, its beats, and how much of him it has
    at: usize,
    at_b: usize,
    t: f64,
    h: f64,
    e: f64,
    tail: f64,
    free: f64,
    speed: f64,
    gaze_local: [f64; 3],
    fix_l: [f64; 4],
    fix_r: [f64; 4],
    chop: bool,
    palm: bool,
    mouth: bool,
    x: f64,
    w_all: f64,
    // blow_spots: where it goes, the shoulders, and how long each arm is
    tgt: [f64; 3],
    t2: Option<[f64; 3]>,
    sl: [f64; 3],
    sr: [f64; 3],
    arm_r: f64,
    arm_l: f64,
    // right_on: whether the right hand has a part in it
    right: bool,
    // right_keys: the keys the right hand goes by
    keys: [BlowKey; 5],
    nk: usize,
    // right_between: between which two keys he is: into the hit it only gathers speed
    ka: BlowKey,
    kb: BlowKey,
    ez: f64,
    // left_hand: the left hand: held up before a chop, over the mouth, or thrown back behind a palm
    w_l: f64,
}

/// A takedown's own move, the arms, by IK on a copy of the pose (the knife styles' right arm is the knife's own): the chop's fist cocked behind the ear
/// and hammered down, the palm heel driven up and through with the arm locked out; the left flat on his back (chop), over his mouth (the knife from
/// behind) or thrown back and down (the rest). The eyes stay on the mark.
pub fn blow_arms(g: usize, an: &mut Anim, b: &BlowRun, at1: [f64; 3], at2: Option<[f64; 3]>, tw_y: f64, tw_p: f64, shown: &mut f64) -> u32 {
    let mut o = Blow { at1, at2, tw_y, tw_p, ..Blow::default() };
    as_thrown(an, b, &mut o);
    let Some(Some(n)) = with_pose(g, o.at, |_d, _rig, n, _fk| if cast(n, &ARM_ROLES) && cast(n, &[SPINE1, SPINE2, NECK, HEAD]) { Some(n) } else { None }) else {
        return 0;
    };
    if o.w_all <= 0.0 && !(o.mouth && o.t < o.x) {
        *shown = 0.0;
        return RAN;
    }
    let Some(mut hs) = hands_of(g, an) else {
        return 0;
    };
    if !ask_blend(g, o.at, o.at, 1.0, false, o.at_b, n) {
        return 0;
    }
    let hh = &mut hs;
    let Some(()) = with_pose(g, o.at_b, |d, rig, _n, fk| {
        fk_run(rig, d, fk.rot, fk.pos);
        blow_spots(fk, &mut o);
        right_on(&mut o);
        if o.right {
            right_keys(&mut o);
            right_between(&mut o);
            right_hand(hh, d, rig, fk, &o);
        }
        left_hand(hh, d, rig, fk, &mut o);
    }) else {
        return 0;
    };
    let side = |on: bool, r: usize| if on { role(r) } else { usize::MAX };
    let left = o.w_l > 0.0;
    ask_mask(&[
        (side(o.right, SHOULDER_R), o.w_all, false),
        (side(o.right, UPPER_ARM_R), o.w_all, false),
        (side(o.right, FOREARM_R), o.w_all, false),
        (side(o.right, HAND_R), o.w_all, true),
        (side(left, SHOULDER_L), o.w_l, false),
        (side(left, UPPER_ARM_L), o.w_l, false),
        (side(left, FOREARM_L), o.w_l, false),
        (side(left, HAND_L), o.w_l, true),
    ]);
    if !ask_blend(g, o.at, o.at_b, 1.0, true, o.at, n) {
        return 0;
    }
    if o.w_all > 0.0 {
        with_pose(g, o.at, |d, rig, _n, fk| look_at(d, rig, fk, &o.gaze_local, o.tgt, 0.6 * o.w_all, 0.0));
    }
    hands_to(&hs, an);
    *shown = max(o.w_all, o.w_l);
    RAN | RAN_ALL
}

/// The blow, its beats, and how much of him it has.
fn as_thrown(an: &Anim, b: &BlowRun, o: &mut Blow) {
    let (at, at_b, _) = poses(an);
    let (t, h, e, tail, style, free, speed) = (b.t, b.hit, b.end, b.tail, b.style, 1.0 - clamp(an.upper_w.aim, 0.0, 1.0), abs(an.speed));
    let (gaze_local, fix_l, fix_r) = (an.gaze_local, an.hand_fix[0], an.hand_fix[1]);
    let (chop, palm, mouth) = (style == BlowStyle::Chop, style == BlowStyle::Palm, style == BlowStyle::Mouth);
    let x = e + tail;
    let w_all = smoothstep(0.0, 0.5 * h, t) * (1.0 - smoothstep(e - 0.16, e, t)) * free;
    o.at = at;
    o.at_b = at_b;
    o.t = t;
    o.h = h;
    o.e = e;
    o.tail = tail;
    o.free = free;
    o.speed = speed;
    o.gaze_local = gaze_local;
    o.fix_l = fix_l;
    o.fix_r = fix_r;
    o.chop = chop;
    o.palm = palm;
    o.mouth = mouth;
    o.x = x;
    o.w_all = w_all;
}

/// Where it goes, the shoulders, and how long each arm is.
fn blow_spots(fk: &mut Fk, o: &mut Blow) {
    let Blow { at1, at2, tw_y, tw_p, .. } = *o;
    let tgt = untwist(fk, tw_y, tw_p, at1);
    let t2 = at2.map(|p| untwist(fk, tw_y, tw_p, p));
    let (sl, sr) = (fk.at(role(UPPER_ARM_L)), fk.at(role(UPPER_ARM_R)));
    let (er, el) = (fk.at(role(FOREARM_R)), fk.at(role(FOREARM_L)));
    let arm_r = v3::dist(sr, er) + v3::dist(er, fk.at(role(HAND_R)));
    let arm_l = v3::dist(sl, el) + v3::dist(el, fk.at(role(HAND_L)));
    (o.tgt, o.t2, o.sl, o.sr, o.arm_r, o.arm_l) = (tgt, t2, sl, sr, arm_r, arm_l);
}

/// A point no further than the arm reaches.
fn cap(s: [f64; 3], p: [f64; 3], arm: f64) -> [f64; 3] {
    let off = v3::sub(p, s);
    let (l, c) = (v3::len(off), BLOW.reach * arm);
    if l > c {
        v3::mad(s, off, c / l)
    } else {
        p
    }
}

/// The arm thrown out straight along a line.
fn lock(s: [f64; 3], dv: [f64; 3], arm: f64) -> [f64; 3] {
    v3::mad(s, v3::normalize(dv), BLOW.reach * arm)
}

/// Whether the right hand has a part in it.
fn right_on(o: &mut Blow) {
    let Blow { chop, palm, w_all, .. } = *o;
    let right = (chop || palm) && w_all > 0.0;
    o.right = right;
}

/// The keys the right hand goes by.
fn right_keys(o: &mut Blow) {
    let Blow { h, e, chop, tgt, sr, arm_r, .. } = *o;
    let thru = v3::sub(tgt, sr);
    let last: BlowKey = (e - 0.14, lock(sr, [thru[0] * 0.5, max(thru[1], 0.0) + 0.4, max(thru[2], 0.1) * 0.8], arm_r), [0.0, 0.8, -0.5], [0.0, -0.5, -0.8], [-0.5, -0.8, -0.2]);
    let (keys, nk) = if chop {
        (
            [
                (0.5 * h, v3::add(sr, [0.02, 0.12, -0.14]), [0.2, 0.3, -0.9], [-1.0, 0.0, 0.0], [-0.7, 1.0, -0.4]),
                (0.72 * h, v3::add(sr, [0.03, 0.13, -0.17]), [0.2, 0.3, -0.9], [-1.0, 0.0, 0.0], [-0.7, 1.0, -0.4]),
                (h, cap(sr, tgt, arm_r), [0.1, -0.45, 0.9], [-1.0, 0.2, 0.0], [-0.8, 0.1, -0.3]),
                (h + 0.1, cap(sr, v3::add(tgt, [0.0, -0.32, 0.06]), arm_r), [0.1, -0.8, 0.6], [-1.0, 0.2, 0.0], [-0.8, -0.2, -0.4]),
                (h + 0.22, cap(sr, v3::add(tgt, [0.0, -0.5, 0.1]), arm_r), [0.1, -0.9, 0.4], [-1.0, 0.2, 0.0], [-0.8, -0.2, -0.5]),
            ],
            5,
        )
    } else {
        (
            [
                (0.5 * h, v3::add(sr, [0.16, -0.34, 0.2]), [0.0, 0.9, 0.3], [0.0, 0.3, -0.9], [-0.4, -0.7, -0.5]),
                (h, cap(sr, tgt, arm_r), [0.0, 0.9, -0.3], [0.0, -0.3, -0.9], [-0.5, -0.8, -0.2]),
                (h + 0.09, lock(sr, [thru[0] * 0.6, max(thru[1], 0.0) + 0.34, max(thru[2], 0.1) * 0.9], arm_r), [0.0, 0.8, -0.5], [0.0, -0.5, -0.8], [-0.5, -0.8, -0.2]),
                last,
                last,
            ],
            4,
        )
    };
    (o.keys, o.nk) = (keys, nk);
}

/// Between which two keys he is: into the hit it only gathers speed.
fn right_between(o: &mut Blow) {
    let Blow { t, h, keys, nk, .. } = *o;
    let mut i = 0;
    while i < nk - 1 && t >= keys[i + 1].0 {
        i += 1;
    }
    let j = if i + 1 < nk { i + 1 } else { nk - 1 };
    let (ka, kb) = (keys[i], keys[j]);
    let u = if j == i || t <= ka.0 { 0.0 } else { clamp((t - ka.0) / (kb.0 - ka.0), 0.0, 1.0) };
    // (into the hit it only gathers speed)
    let ez = if kb.0 == h { u * u } else { u * u * (3.0 - 2.0 * u) };
    (o.ka, o.kb, o.ez) = (ka, kb, ez);
}

/// The right arm there, a fist or an open hand.
fn right_hand(hh: &mut Hands, d: &mut [S], rig: &Rig, fk: &mut Fk, o: &Blow) {
    let Blow { fix_r, chop, ka, kb, ez, .. } = *o;
    let rot = quat::slerp(hand_rot(&fix_r, ka.2, ka.3), hand_rot(&fix_r, kb.2, kb.3), ez);
    two_bone_ik(rig, d, fk.rot, fk.pos, role(UPPER_ARM_R), role(FOREARM_R), role(HAND_R), &v3::lerp3(ka.1, kb.1, ez), Some(&v3::lerp3(ka.4, kb.4, ez)), 0.85, Some(&rot), false);
    if chop {
        hand(d, hh, 1, &[BLOW.fist[0], BLOW.fist[1], BLOW.fist[2], BLOW.fist[3]], 0.6, 0.0);
    } else {
        hand(d, hh, 1, &[BLOW.open[0], BLOW.open[1], BLOW.open[2], BLOW.open[3]], 0.2, 0.0);
    }
}

/// The left hand: held up before a chop, over the mouth, or thrown back behind a palm.
fn left_hand(hh: &mut Hands, d: &mut [S], rig: &Rig, fk: &mut Fk, o: &mut Blow) {
    let Blow { t, h, e, tail, free, speed, fix_l, chop, mouth, x, t2, sl, arm_l, .. } = *o;
    let mut w_l = 0.0;
    if let (true, Some(to)) = (chop, t2) {
        w_l = bump(t, 0.04, 0.5 * h, h - 0.02, h + 0.15) * free;
        fk_run(rig, d, fk.rot, fk.pos);
        let rot = hand_rot(&fix_l, [0.0, 1.0, 0.2], [0.0, 0.2, -1.0]);
        two_bone_ik(rig, d, fk.rot, fk.pos, role(UPPER_ARM_L), role(FOREARM_L), role(HAND_L), &cap(sl, to, arm_l), Some(&[0.6, -0.7, -0.2]), 0.8, Some(&rot), false);
        hand(d, hh, 0, &[BLOW.open[0], BLOW.open[1], BLOW.open[2], BLOW.open[3]], 0.2, 0.0);
    } else if let (true, Some(to)) = (mouth, t2) {
        let (from, gone) = if tail > 0.0 { (x - 0.5, x - 0.2) } else { (e - 0.18, e - 0.04) };
        w_l = bump(t, 0.03, 0.14, from, gone) * free * if tail > 0.0 && t > e { 1.0 - smoothstep(BLOW.walk[0], BLOW.walk[1], speed) } else { 1.0 };
        fk_run(rig, d, fk.rot, fk.pos);
        let rot = hand_rot(&fix_l, [-1.0, 0.0, 0.1], [0.1, 0.0, 1.0]);
        two_bone_ik(rig, d, fk.rot, fk.pos, role(UPPER_ARM_L), role(FOREARM_L), role(HAND_L), &cap(sl, to, arm_l), Some(&[0.8, -0.5, 0.0]), 0.8, Some(&rot), false);
        hand(d, hh, 0, &[BLOW.cup[0], BLOW.cup[1], BLOW.cup[2], BLOW.cup[3]], 0.3, 0.0);
    } else if !chop && !mouth {
        w_l = bump(t, 0.4 * h, h, e - 0.2, e) * free;
        fk_run(rig, d, fk.rot, fk.pos);
        let rot = hand_rot(&fix_l, [0.15, -0.7, -0.7], [1.0, 0.2, 0.1]);
        two_bone_ik(rig, d, fk.rot, fk.pos, role(UPPER_ARM_L), role(FOREARM_L), role(HAND_L), &lock(sl, [0.16, -0.72, -0.68], arm_l), Some(&[0.5, 0.2, -0.8]), 0.6, Some(&rot), false);
        hand(d, hh, 0, &[BLOW.loose[0], BLOW.loose[1], BLOW.loose[2], BLOW.loose[3]], 0.3, 0.0);
    }
    o.w_l = w_l;
}
/// The knife's blow, the right arm over the punch clip's pose, by IK on a copy: to the hilt, the knife drawn down out of the sheath, through the kind's
/// keys (eased key to key, the wrist's spots off the right shoulder, turned onto the man), back to the sheath's mouth and home; then the arm goes back
/// to what the pose under it has. The clock is the caller's.
pub fn knife_pose(g: usize, an: &mut Anim, k: &KnifeSlash, t: f64, t0: f64, total: f64, tw_y: f64, tw_p: f64, there: Option<[f64; 3]>, rc_inv: [f64; 4]) -> u32 {
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
    if !ask_blend(g, at, at, 1.0, false, at_b, n) {
        return 0;
    }
    let hh = &mut hs;
    with_pose(g, at_b, |d, rig, _n, fk| {
        fk_run(rig, d, fk.rot, fk.pos);
        let (sl, sr, er) = (fk.at(role(UPPER_ARM_L)), fk.at(role(UPPER_ARM_R)), fk.at(role(FOREARM_R)));
        let arm = v3::dist(sr, er) + v3::dist(er, fk.at(role(HAND_R)));
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
            let (l, cap) = (v3::len(o), KNIFE_BLOW.reach * arm);
            if l > cap {
                o = v3::scale(o, cap / l);
            }
            let bl = fr(q.blade);
            (v3::add(sr, o), knife_rot(&fix_r, bl, v3::cross(fr(q.fing), bl)), fr(q.pole))
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
        two_bone_ik(rig, d, fk.rot, fk.pos, role(UPPER_ARM_R), role(FOREARM_R), role(HAND_R), &v3::lerp3(sa_.0, sb_.0, ez), Some(&v3::lerp3(sa_.2, sb_.2, ez)), 0.9, Some(&quat::slerp(sa_.1, sb_.1, ez)), false);
        hand(d, hh, 1, &[REACH.curl[0], REACH.curl[1], REACH.curl[2], REACH.curl[3]], REACH.thumb, 0.0);
    });
    let w = lerp(under, 1.0, max(kb_r0, smoothstep(0.0, if falsy(t0) { 1e-6 } else { t0 }, kb_t)) * (1.0 - smoothstep(total - KNIFE_BLOW.fade, total, kb_t)));
    ask_mask(&[(role(SHOULDER_R), w, false), (role(UPPER_ARM_R), w, false), (role(FOREARM_R), w, false), (role(HAND_R), w, true)]);
    if !ask_blend(g, at, at_b, 1.0, true, at, n) {
        return 0;
    }
    hands_to(&hs, an);
    RAN | RAN_ALL
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
/// What the steps of the ready pose hand on to the ones after them.
#[derive(Clone, Copy, Default)]
struct Ready {
    w: f64,
    live: bool,
    dt: f64,
    tw_y: f64,
    tw_p: f64,
    rc_inv: [f64; 4],
    // as_ready: the pose's kind and shape, and how fast its numbers come
    grab: bool,
    ready_up: f64,
    has_wp: bool,
    wp: [f64; 3],
    bare: bool,
    rd_a: f64,
    u: f64,
    rd_out_r: f64,
    close_k: f64,
    cl_t: f64,
    knife_on: bool,
    crouch: f64,
    fix_l: [f64; 4],
    fix_r: [f64; 4],
    l_pole: [f64; 3],
    l_fing: [f64; 3],
    l_back: [f64; 3],
    l_curl: f64,
    l_thumb: f64,
    l_spread: f64,
    reach: Option<game_types::character::ReadyShapeLReach>,
    spot: [f64; 3],
    rd_live: bool,
    kq: f64,
    e: f64,
    // shoulders: the shoulders, and forward square to their line
    sl: [f64; 3],
    sr: [f64; 3],
    fx: f64,
    fz: f64,
    // arm_length: how long the arm is, and no bearing yet
    arm: f64,
    az: f64,
    h: f64,
    up: f64,
    // reach_spot: where the lead hand goes
    o_l: [f64; 3],
    // off_hand: the right hand, off his ribs for the last stretch only and never under a hand that is on a hilt
    w_rb: f64,
}

/// The ready pose, a small layer: the trunk bowed toward the man (and bladed for the guard) straight into the pose by the weight, then the arms by IK on
/// a copy in the shoulders' frame as it now stands, blended in shoulder → elbow → hand: the left always, the right by the kind's share, or on the
/// knife's hilt by the reach's clock. Every number is eased (`Anim.rd`).
pub fn ready_layer(g: usize, an: &mut Anim, w: f64, kind: ReadyKind, shape: &ReadyShape, live: bool, dt: f64, tw_y: f64, tw_p: f64, there: Option<[f64; 3]>, rc_inv: [f64; 4], tune: &Knobs) -> u32 {
    let mut o = Ready { w, live, dt, tw_y, tw_p, rc_inv, ..Ready::default() };
    let (at, at_b, _) = poses(an);
    as_ready(an, shape, kind, there, &mut o);
    let (mut rd, mut cl_l) = (core::mem::take(&mut an.rd), an.cl_l);
    let c = &mut rd[..];
    // ---- the trunk: which way the man is from the chest (flat, within a radian of dead ahead), the bow toward him, the guard's blade
    let Some(Some(n)) = with_pose(g, at, |d, rig, n, fk| {
        if !(cast(n, &ARM_ROLES) && cast(n, &[SPINE1, SPINE2, NECK, HEAD])) {
            return None;
        }
        fk_run(rig, d, fk.rot, fk.pos);
        ready_eases(fk, c, shape, &o);
        ready_trunk(d, rig, c, &o);
        Some(n)
    }) else {
        return 0;
    };
    let Some(mut hs) = hands_of(g, an) else {
        return 0;
    };
    // ---- the arms, on a copy: the left clavicle forward with a reach, then the shoulders' frame as it stands
    if !ask_blend(g, at, at, 1.0, false, at_b, n) {
        return 0;
    }
    let hh = &mut hs;
    let Some(()) = with_pose(g, at_b, |d, rig, _n, fk| {
        fk_run(rig, d, fk.rot, fk.pos);
        if c[CLAV] * o.e > 1e-4 {
            turn(d, rig, role(SHOULDER_L), q_yaw(-c[CLAV] * o.e));
            fk_run(rig, d, fk.rot, fk.pos);
        }
        shoulders(fk, &mut o);
        if live {
            arm_length(fk, &mut o);
            // (left, up, forward) in the shoulders' frame turned t toward his left → model axes
            bearing(fk, &mut o);
            reach_spot(c, &mut o);
            ease_lead(c, &o);
            ease_bare(c, &o);
            ease(c, W_R, if o.bare { 1.0 } else { 0.0 }, o.kq);
        } else if !o.rd_live {
            c[W_R] = 0.0;
        }
        lead_arm(d, rig, fk, hh, c, &o);
        // (the right hand comes off his ribs for the last stretch only; never under a hand that is on a hilt)
        off_hand(d, rig, fk, hh, c, &mut o);
        // ---- the right hand on the knife: the fist closes on the sheathed handle; the knife stays where it is
        on_the_hilt(tune, d, rig, fk, hh, &o);
    }) else {
        return 0;
    };
    // staggered, as the signals: the clavicle / upper arm lead, the forearm and the hand a little behind — in, and out in the same order
    let (w_up, w_fore, w_hand) = (smoothstep(0.0, 0.75, w), smoothstep(0.08, 0.88, w), smoothstep(0.16, 1.0, w));
    // the left arm's move for the man waits for the right hand to be on the hilt
    let mut lg = 1.0;
    if o.close_k >= 0.0 && o.knife_on {
        if !o.rd_live {
            cl_l = 0.0;
        }
        cl_l = clamp(cl_l + if live && o.u >= tune.kit.close.left_at { dt / (tune.kit.close.left_in * (1.0 + READY_UP.slow * c[UP])) } else { -dt / tune.kit.close.left_out }, 0.0, 1.0);
        lg = cl_l * cl_l * (3.0 - 2.0 * cl_l);
    }
    let reach_shown = smoothstep(0.16, 1.0, o.u);
    let right = o.u > 0.0 || o.w_rb > 1e-3;
    let (r_up, r_fore, r_hand) = if o.u > 0.0 { (smoothstep(0.0, 0.75, o.u), smoothstep(0.08, 0.88, o.u), reach_shown) } else { (w_up * o.w_rb, w_fore * o.w_rb, w_hand * o.w_rb) };
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
    hands_to(&hs, an);
    (an.rd, an.cl_l, an.reach_shown, an.rd_live) = (rd, cl_l, reach_shown, true);
    RAN | RAN_ALL
}

/// The pose's kind and shape, and how fast its numbers come.
fn as_ready(an: &mut Anim, shape: &ReadyShape, kind: ReadyKind, there: Option<[f64; 3]>, o: &mut Ready) {
    let Ready { w, live, dt, .. } = *o;
    let (grab, ready_up, has_wp, wp) = (kind == ReadyKind::Grab, an.ready_up, there.is_some(), there.unwrap_or([0.0; 3]));
    let bare = an.no_knife && grab && !an.grip[1];
    let (rd_a, u, rd_out_r, close_k, cl_t, knife_on, crouch) = (an.rd_a, an.rc_t, an.rd_out_r, an.close_k, an.cl_t, shape.knife && !an.no_knife, an.crouch);
    let (fix_l, fix_r) = (an.hand_fix[0], an.hand_fix[1]);
    let l = &shape.l;
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
    o.grab = grab;
    o.ready_up = ready_up;
    o.has_wp = has_wp;
    o.wp = wp;
    o.bare = bare;
    o.rd_a = rd_a;
    o.u = u;
    o.rd_out_r = rd_out_r;
    o.close_k = close_k;
    o.cl_t = cl_t;
    o.knife_on = knife_on;
    o.crouch = crouch;
    o.fix_l = fix_l;
    o.fix_r = fix_r;
    o.l_pole = l_pole;
    o.l_fing = l_fing;
    o.l_back = l_back;
    o.l_curl = l_curl;
    o.l_thumb = l_thumb;
    o.l_spread = l_spread;
    o.reach = reach;
    o.spot = spot;
    o.rd_live = rd_live;
    o.kq = kq;
    o.e = e;
}

/// The trunk's numbers eased: which way the man is from the chest (flat, within a radian of dead ahead), the bow, the guard's blade, the reach up.
fn ready_eases(fk: &mut Fk, c: &mut [f64], shape: &ReadyShape, o: &Ready) {
    let Ready { live, dt, tw_y, tw_p, grab, ready_up, has_wp, wp, rd_live, kq, .. } = *o;
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
}

/// The bow toward him, the rise out of a crouch, the guard's blade.
fn ready_trunk(d: &mut [S], rig: &Rig, c: &mut [f64], o: &Ready) {
    let Ready { e, .. } = *o;
    let (lean, blade, hi) = (c[LEAN] * e, c[BLADE] * e, c[UP] * e);
    // (bowing toward +Z is + about +X = up × Z)
    let ax = v3::normalize(v3::cross(AY, [c[INTO], 0.0, c[INTO + 1] + 1e-6]));
    if lean > 1e-4 {
        turn(d, rig, role(SPINE2), quat::axis_angle(ax, lean * 0.45));
        turn(d, rig, role(SPINE3), quat::axis_angle(ax, lean * 0.55));
        turn(d, rig, role(NECK), quat::axis_angle(ax, -lean * 0.45));
        turn(d, rig, role(HEAD), quat::axis_angle(ax, -lean * 0.55));
    }
    if hi > 1e-4 {
        // (up out of the crouch's bow toward the man who stands over him)
        let r = hi * READY_UP.rise * DEGR;
        turn(d, rig, role(SPINE1), quat::axis_angle(ax, -r * 0.3));
        turn(d, rig, role(SPINE2), quat::axis_angle(ax, -r * 0.35));
        turn(d, rig, role(SPINE3), quat::axis_angle(ax, -r * 0.35));
        turn(d, rig, role(NECK), quat::axis_angle(ax, r * READY_UP.look * 0.45));
        turn(d, rig, role(HEAD), quat::axis_angle(ax, r * READY_UP.look * 0.55));
    }
    if blade > 1e-4 {
        // (− yaw swings his left shoulder toward +Z)
        turn(d, rig, role(SPINE1), q_yaw(-blade * 0.25));
        turn(d, rig, role(SPINE2), q_yaw(-blade * 0.35));
        turn(d, rig, role(SPINE3), q_yaw(-blade * 0.4));
        turn(d, rig, role(NECK), q_yaw(blade * 0.4));
        turn(d, rig, role(HEAD), q_yaw(blade * 0.6));
    }
}

/// The shoulders, and forward square to their line.
fn shoulders(fk: &mut Fk, o: &mut Ready) {
    let (sl, sr) = (fk.at(role(UPPER_ARM_L)), fk.at(role(UPPER_ARM_R)));
    // forward, square to the shoulder line (his left = (fz, 0, −fx))
    let (mut fx, mut fz) = (-(sl[2] - sr[2]), sl[0] - sr[0]);
    let l = hypot2(fx, fz);
    let l = if falsy(l) { 1.0 } else { l };
    fx /= l;
    fz /= l;
    (o.sl, o.sr, o.fx, o.fz) = (sl, sr, fx, fz);
}

/// How long the arm is, and no bearing yet.
fn arm_length(fk: &mut Fk, o: &mut Ready) {
    let Ready { sl, .. } = *o;
    let el = fk.at(role(FOREARM_L));
    let arm = v3::dist(sl, el) + v3::dist(el, fk.at(role(HAND_L)));
    let (az, h, up) = (0.0, 0.0, 0.0);
    (o.arm, o.az, o.h, o.up) = (arm, az, h, up);
}

/// (left, up, forward) in the shoulders' frame turned t toward his left, in the model's axes.
fn frame(o: &Ready, v: [f64; 3], t: f64) -> [f64; 3] {
    let Ready { fx, fz, .. } = *o;
    let (ct, st) = (cos(t), sin(t));
    let (l, f) = (v[0] * ct + v[2] * st, v[2] * ct - v[0] * st);
    [fz * l + fx * f, v[1], fz * f - fx * l]
}

/// Where the man is off the left shoulder: level run, height, bearing off square.
fn bearing(fk: &mut Fk, o: &mut Ready) {
    let Ready { tw_y, tw_p, has_wp, wp, sl, fx, fz, mut az, mut h, mut up, .. } = *o;
    if has_wp {
        // where the man is off the left shoulder: level run, height, bearing off square (+ = his left)
        let dd = v3::sub(untwist(fk, tw_y, tw_p, wp), sl);
        let (pa, pb) = (dd[0] * fx + dd[2] * fz, dd[0] * fz - dd[2] * fx);
        h = hypot2(pa, pb);
        up = dd[1];
        if h > 0.05 {
            az = clamp(atan2(pb, pa), -READY_AZ[0], READY_AZ[1]);
        }
    }
    (o.az, o.h, o.up) = (az, h, up);
}

/// Where the lead hand goes.
fn reach_spot(c: &mut [f64], o: &mut Ready) {
    let Ready { has_wp, bare, rd_a, reach, spot, arm, az, mut h, mut up, .. } = *o;
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
            frame(o, [0.0, y, z], az)
        } else {
            frame(o, [0.0, clamp(frac * up + bias, dy0, lerp(dy1, READY_UP.dy, q)), clamp(frac * h, lerp(r0, READY_UP.r[0], q), lerp(r1, READY_UP.r[1], q))], az)
        };
        let (l, cap) = (v3::len(o), READY_REACH * arm);
        if l > cap {
            o = v3::scale(o, cap / l);
        }
        o
    } else {
        // (a fixed spot is still turned to face him: the guard's lead hand is out toward the man)
        frame(o, spot, az)
    };
    (o.h, o.up, o.o_l) = (h, up, o_l);
}

/// The lead hand's numbers eased.
fn ease_lead(c: &mut [f64], o: &Ready) {
    let Ready { l_pole, l_fing, l_back, l_curl, l_thumb, l_spread, kq, az, o_l, .. } = *o;
    ease3(c, R_L, o_l, kq);
    ease3(c, L_POLE, frame(o, l_pole, az), kq);
    let fing = if c[UP] > 1e-4 { [l_fing[0], l_fing[1] + READY_UP.fing * c[UP], l_fing[2]] } else { l_fing };
    ease3(c, L_FING, frame(o, fing, az), kq);
    ease3(c, L_BACK, frame(o, l_back, az), kq);
    ease(c, L_CURL, l_curl, kq);
    ease(c, L_THUMB, l_thumb, kq);
    ease(c, L_SPREAD, l_spread, kq);
}

/// The other hand's too, with no knife (they snap when it had no share: it comes in from the arm as it is, by the weight).
fn ease_bare(c: &mut [f64], o: &Ready) {
    let Ready { bare, kq, az, .. } = *o;
    if bare {
        // (the right hand's numbers snap when it had no share: it comes in from the arm as it is, by the weight)
        let kr = if c[W_R] < 1e-3 { 1.0 } else { kq };
        ease3(c, R_R, frame(o, READY_BARE.r.at, az), kr);
        ease3(c, R_POLE, frame(o, READY_BARE.r.pole, az), kr);
        ease3(c, R_FING, frame(o, READY_BARE.r.fing, az), kr);
        ease3(c, R_BACK, frame(o, READY_BARE.r.back, az), kr);
        ease(c, R_CURL, READY_BARE.r.curl, kr);
        ease(c, R_THUMB, READY_BARE.r.thumb, kr);
    }
}

/// The lead arm and its fingers.
fn lead_arm(d: &mut [S], rig: &Rig, fk: &mut Fk, hh: &mut Hands, c: &mut [f64], o: &Ready) {
    let Ready { fix_l, sl, .. } = *o;
    let rot = hand_rot(&fix_l, get3(c, L_FING), get3(c, L_BACK));
    two_bone_ik(rig, d, fk.rot, fk.pos, role(UPPER_ARM_L), role(FOREARM_L), role(HAND_L), &v3::add(sl, get3(c, R_L)), Some(&get3(c, L_POLE)), 0.85, Some(&rot), false);
    let q = c[L_CURL];
    hand(d, hh, 0, &[q, q + 0.04, q + 0.08, q + 0.12], c[L_THUMB], c[L_SPREAD]);
}

/// The right hand, off his ribs for the last stretch only and never under a hand that is on a hilt.
fn off_hand(d: &mut [S], rig: &Rig, fk: &mut Fk, hh: &mut Hands, c: &mut [f64], o: &mut Ready) {
    let Ready { rd_a, u, rd_out_r, fix_r, sr, .. } = *o;
    let w_rb = if u > 0.0 { 0.0 } else { c[W_R] * rd_out_r * smoothstep(READY_BARE.way.right[0], READY_BARE.way.right[1], rd_a) };
    if w_rb > 1e-3 {
        let rot = hand_rot(&fix_r, get3(c, R_FING), get3(c, R_BACK));
        two_bone_ik(rig, d, fk.rot, fk.pos, role(UPPER_ARM_R), role(FOREARM_R), role(HAND_R), &v3::add(sr, get3(c, R_R)), Some(&get3(c, R_POLE)), 0.85, Some(&rot), false);
        let q = c[R_CURL];
        hand(d, hh, 1, &[q, q + 0.04, q + 0.08, q + 0.12], c[R_THUMB], 0.0);
    }
    o.w_rb = w_rb;
}

/// The right hand on the knife: the fist closes on the sheathed handle; the knife stays where it is.
fn on_the_hilt(tune: &Knobs, d: &mut [S], rig: &Rig, fk: &mut Fk, hh: &mut Hands, o: &Ready) {
    let Ready { rc_inv, u, close_k, cl_t, crouch, fix_r, .. } = *o;
    if u > 0.0 {
        let (s_at, s_rot, s_pole) = sheath_fist(fk, &rc_inv, &fix_r, crouch, 0.0, 0.0);
        two_bone_ik(rig, d, fk.rot, fk.pos, role(UPPER_ARM_R), role(FOREARM_R), role(HAND_R), &s_at, Some(&s_pole), 0.9, Some(&s_rot), false);
        if close_k >= 0.0 {
            // (the fist on the hilt is alive: the fingers let go a little, little finger first, and close again; the thumb shifts with them)
            let ph = (cl_t % tune.kit.close.regrip) / tune.kit.close.regrip;
            let gg = |dv: f64| bump(ph, 0.55 + dv, 0.66 + dv, 0.72 + dv, 0.86 + dv);
            hand(d, hh, 1, &[REACH.curl[0] - 0.14 * gg(0.06), REACH.curl[1] - 0.2 * gg(0.04), REACH.curl[2] - 0.26 * gg(0.02), REACH.curl[3] - 0.3 * gg(0.0)], REACH.thumb - 0.25 * gg(0.03), 0.0);
        } else {
            hand(d, hh, 1, &[REACH.curl[0], REACH.curl[1], REACH.curl[2], REACH.curl[3]], REACH.thumb, 0.0);
        }
    }
}
/// how many keys of a knife blow a call may hand over (character.ts KNIFE_KEYS_MAX: a longer blow is refused by name as the game loads)
const KNIFE_KEYS: usize = 4;
