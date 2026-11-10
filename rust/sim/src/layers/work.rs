//! A hand on a door, a lock, a thing, a body.
use super::*;

/// At a lock: the crouch's rounded spine sat up toward it, the shoulders up, the head onto it, and both hands at the keyway (the pick raking, the wrench
/// hand under it giving as a pin sets).
pub fn lockpick_layer(g: usize, an: &mut Anim, w: f64) -> u32 {
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
    hands_to(&h, an);
    RAN | RAN_ALL
}
/// Both hands on a body: the trunk bowed toward the grip and tipped off a load, the eyes a stride beyond the hands, and each wrist to its spot through
/// the inverse of the chest twist still to come.
pub fn haul_layer(g: usize, an: &mut Anim, h_: &HaulPose, w: f64, l_at: [f64; 3], r_at: [f64; 3], tw_y: f64, tw_p: f64) -> u32 {
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
    hands_to(&h, an);
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
pub fn door_layer(g: usize, an: &mut Anim, l: &DoorPush, n: [f64; 3], p0: [f64; 3], dt: f64, intent: f64, right_free: bool, tw_y: f64, tw_p: f64) -> u32 {
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
pub fn work_layer(g: usize, an: &mut Anim, l: &WorkPose, n_at: [f64; 3], p0: [f64; 3], dt: f64, live: bool, r_free: bool, tw_y: f64, tw_p: f64) -> u32 {
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
                let ok = sp.body || if if side == 1 { c.ok_r } else { c.ok_l } { dd < 0.6 && dd > 0.14 && ahead > 0.0 } else { dd < 0.57 && dd > 0.18 && ahead > 0.04 };
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
    hands_to(&hs, an);
    RAN | RAN_ALL
}
