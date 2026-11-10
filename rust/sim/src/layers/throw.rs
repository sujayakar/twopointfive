//! A throw.
use super::*;

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
    let give = if hit > h0 { 1.0 - clamp((hit - h0) / max(1.0 - h0, 0.25), 0.0, 1.0) } else { 1.0 };
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
    if !ask_blend(g, at, at, 1.0, false, at_b, n) {
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
        hand(d, hh, if off { 0 } else { 1 }, &[c_r(0.5, 0.05, 0.25), c_r(0.55, 0.08, 0.3), c_r(0.62, 0.12, 0.35), c_r(0.7, 0.15, 0.42)], c_r(0.7, 0.1, 0.3), 0.0);
        let firm = bump(t, 0.22, 0.32, 0.5, 0.64);
        let c_l = |v: f64| lerp(v, v + 0.25, firm);
        if !low && !off {
            hand(d, hh, 0, &[c_l(0.3), c_l(0.35), c_l(0.4), c_l(0.45)], lerp(0.3, 0.6, firm), 0.0);
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
    if !ask_blend(g, at, at_b, 1.0, true, at, n) {
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
