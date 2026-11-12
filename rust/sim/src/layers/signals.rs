//! A hand signal, an idle tell, a radio, a smoke, the goggles.
use super::*;

/// the head as it will be DRAWN (the neck's and the head's twists of last frame over the run-forward bones): where the joint is and its three axes
struct HeadDrawn {
    at: [f64; 3],
    x: [f64; 3],
    y: [f64; 3],
    z: [f64; 3],
}
impl HeadDrawn {
    fn of(rig: &Rig, fk: &Fk, tw: &[f64; 4], rest_inv: &[f64; 4]) -> HeadDrawn {
        let tn = twist_of(tw[0], tw[1]);
        let hr = quat::mul(twist_of(tw[2], tw[3]), quat::mul(tn, quat::mul(fk.rot(rig.role(HEAD)), *rest_inv)));
        let nk = fk.at(rig.role(NECK));
        HeadDrawn { at: v3::add(nk, quat::rotate(tn, v3::sub(fk.at(rig.role(HEAD)), nk))), x: quat::rotate(hr, [1.0, 0.0, 0.0]), y: quat::rotate(hr, [0.0, 1.0, 0.0]), z: quat::rotate(hr, [0.0, 0.0, 1.0]) }
    }
    /// a direction of the head's frame, in the model's
    fn dir(&self, p: &[f64; 3]) -> [f64; 3] {
        let (x, y, z) = (&self.x, &self.y, &self.z);
        [x[0] * p[0] + y[0] * p[1] + z[0] * p[2], x[1] * p[0] + y[1] * p[1] + z[1] * p[2], x[2] * p[0] + y[2] * p[1] + z[2] * p[2]]
    }
}
/// the support arm's mask, joint by joint: the shoulder and the upper arm, the forearm, the hand and all its digits
fn left_arm_mask(body: &mut Body, w_up: f64, w_fore: f64, w_hand: f64) {
    body.mask(&[(body.rig.role(SHOULDER_L), w_up, false), (body.rig.role(UPPER_ARM_L), w_up, false), (body.rig.role(FOREARM_L), w_fore, false), (body.rig.role(HAND_L), w_hand, true)]);
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
fn head_hand(body: &mut Body, an: &mut Anim, p: &HeadHand) {
    let (at, at_b, _) = poses(an);
    let (tw, hri, fix_l) = (an.nv_tw, an.head_rest_inv, an.hand_fix[0]);
    let hd = body.pose(at, |d, rig, fk| {
        fk_run(rig, d, fk.rot, fk.pos);
        HeadDrawn::of(rig, fk, &tw, &hri)
    });
    let mut h = hands_of(body, an);
    body.copy(at, at_b);
    let hh = &mut h;
    body.pose(at_b, |d, rig, fk| {
        turn(d, rig, rig.role(SHOULDER_L), q_roll(p.shrug * DEGR));
        fk_run(rig, d, fk.rot, fk.pos);
        let r = hand_rot(&fix_l, hd.dir(&p.fingers), hd.dir(&p.back));
        two_bone_ik(rig, d, fk.rot, fk.pos, rig.role(UPPER_ARM_L), rig.role(FOREARM_L), rig.role(HAND_L), &v3::add(hd.at, hd.dir(&p.to)), Some(&p.pole), 0.85, Some(&r), false);
        hand(rig, d, hh, 0, &p.curl, p.thumb, 0.0);
    });
    left_arm_mask(body, p.e, p.e, p.e);
    body.blend(at, at_b, 1.0, true, at);
    hands_to(&h, an);
}
/// A smoke's arm: the left wrist to a place in front of the mouth.
pub fn smoke_pose(body: &mut Body, an: &mut Anim) {
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
    head_hand(body, an, &p)
}
/// A goggle flip's arm: the left hand at the goggles, wherever between up and down they are.
pub fn nv_pose(body: &mut Body, an: &mut Anim, k: f64, tune: &Knobs) {
    let p = HeadHand {
        e: an.nv_arm,
        shrug: tune.kit.nv.hand.shrug,
        to: v3::lerp3(tune.kit.nv.hand.up, tune.kit.nv.hand.down, k),
        pole: tune.kit.nv.hand.pole,
        fingers: tune.kit.nv.hand.fingers,
        back: tune.kit.nv.hand.back,
        curl: [tune.kit.nv.hand.curl[0], tune.kit.nv.hand.curl[1], tune.kit.nv.hand.curl[2], tune.kit.nv.hand.curl[3]],
        thumb: tune.kit.nv.hand.thumb,
    };
    head_hand(body, an, &p)
}
/// the support arm laid on shoulder first, then the elbow, then the hand, and off in the same order (the signals' and the tells' envelopes)
fn staggered(t: f64, tin: f64, tout: f64, e: f64) -> (f64, f64, f64) {
    let seg = |lag: f64| bump(t, lag, lag + tin, e - tout + lag * 0.6 - 0.06, e - 0.06 + lag * 0.6);
    (seg(0.0), seg(0.05), seg(0.09))
}
/// A left-hand signal: the arm solved against the body as it stands and laid on shoulder first.
pub fn signal_layer(body: &mut Body, an: &mut Anim, t: f64, kind: SignalKind, dur: f64) {
    let (at, at_b, _) = poses(an);
    let (side, fix_l) = (an.sig_side, an.hand_fix[0]);
    body.pose(at, |d, rig, _fk| {
        if kind == SignalKind::Go {
            // body English on the chop: the upper spine goes a few degrees with the hand, the eyes stay on where it sends people
            let ft = curve(t, &[[0.22, 0.0], [0.46, 1.0], [0.62, 0.5], [0.9, 0.0]]);
            if ft > 1e-3 {
                let q = |kk: f64| quat::mul(q_yaw(-3.5 * DEGR * ft * kk), quat::mul(q_roll(-2.0 * DEGR * ft * kk), q_pitch(2.5 * DEGR * ft * kk)));
                turn(d, rig, rig.role(SPINE2), q(0.4));
                turn(d, rig, rig.role(SPINE3), q(0.6));
                turn(d, rig, rig.role(NECK), q(-0.3));
                turn(d, rig, rig.role(HEAD), q(-0.4));
            }
        }
    });
    let mut h = hands_of(body, an);
    body.copy(at, at_b);
    let hh = &mut h;
    body.pose(at_b, |d, rig, fk| {
        fk_run(rig, d, fk.rot, fk.pos);
        let sh = fk.at(rig.role(UPPER_ARM_L));
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
            r = hand_rot(&fix_l, v3::lerp3([0.1, 1.0, 0.25], [0.05, -0.18, 1.0], chop), v3::lerp3([1.0, 0.0, -0.2], [0.6, 0.8, 0.1], chop));
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
            turn(d, rig, rig.role(SHOULDER_L), q_roll(clav * DEGR));
        }
        fk_run(rig, d, fk.rot, fk.pos);
        two_bone_ik(rig, d, fk.rot, fk.pos, rig.role(UPPER_ARM_L), rig.role(FOREARM_L), rig.role(HAND_L), &to, Some(&pole), 0.85, Some(&r), false);
        hand(rig, d, hh, 0, &curl, thumb, spread);
    });
    let (w_up, w_fore, w_hand) = staggered(t, 0.2, 0.26, dur);
    left_arm_mask(body, w_up, w_fore, w_hand);
    body.blend(at, at_b, 1.0, true, at);
    hands_to(&h, an);
}
/// An idle tell: a look at the watch (the forearm level across the chest, the eyes down onto it), or a hand round to the back of the neck with the chest
/// lifted into the stretch.
pub fn tell_layer(body: &mut Body, an: &mut Anim, t: f64, watch: bool, dur: f64) {
    let (at, at_b, _) = poses(an);
    let (gaze_local, fix_l) = (an.gaze_local, an.hand_fix[0]);
    let (tin, tout) = if watch { (0.2, 0.28) } else { (0.26, 0.34) };
    // the body's share (the head, the chest): one window over the whole tell
    let env = bump(t, 0.0, tin, dur - tout, dur);
    if env <= 1e-4 {
        return;
    }
    let mut h = hands_of(body, an);
    let mut wrist = [0.0f64; 3];
    body.pose(at, |d, rig, fk| {
        fk_run(rig, d, fk.rot, fk.pos);
        if watch {
            // the wrist's spot first, off the shoulder as the body has it now, then the eyes onto the watch on the back of it
            let sh = fk.at(rig.role(UPPER_ARM_L));
            wrist = [sh[0] - 0.21, sh[1] - 0.21, sh[2] + 0.21];
            look_at(d, rig, fk, &gaze_local, [wrist[0] + 0.03, wrist[1] + 0.02, wrist[2] + 0.01], 0.9 * env, 0.0);
        } else {
            // the stretch: the chest up a few degrees, the head forward and tipped away from the working hand
            turn(d, rig, rig.role(SPINE2), q_pitch(-1.6 * DEGR * env));
            turn(d, rig, rig.role(SPINE3), q_pitch(-1.8 * DEGR * env));
            turn(d, rig, rig.role(NECK), quat::mul(q_pitch(5.0 * DEGR * env), q_roll(-3.0 * DEGR * env)));
            turn(d, rig, rig.role(HEAD), quat::mul(q_pitch(9.0 * DEGR * env), quat::mul(q_roll(-6.0 * DEGR * env), q_yaw(-4.0 * DEGR * env))));
        }
    });
    body.copy(at, at_b);
    let hh = &mut h;
    body.pose(at_b, |d, rig, fk| {
        fk_run(rig, d, fk.rot, fk.pos);
        if watch {
            turn(d, rig, rig.role(SHOULDER_L), q_roll(3.0 * DEGR * env));
            fk_run(rig, d, fk.rot, fk.pos);
            let r = hand_rot(&fix_l, [-1.0, 0.14, 0.12], [0.05, 0.9, -0.42]);
            two_bone_ik(rig, d, fk.rot, fk.pos, rig.role(UPPER_ARM_L), rig.role(FOREARM_L), rig.role(HAND_L), &wrist, Some(&[1.0, -0.55, 0.18]), 0.85, Some(&r), false);
            hand(rig, d, hh, 0, &[0.35, 0.4, 0.45, 0.5], 0.4, 0.0);
        } else {
            // the scapula comes up with the raised elbow; the hand works up and down the back of the neck
            turn(d, rig, rig.role(SHOULDER_L), q_roll(12.0 * DEGR * env));
            fk_run(rig, d, fk.rot, fk.pos);
            let nk = fk.at(rig.role(NECK));
            let rub = 0.018 * sin((t - tin) * 2.0 * PI * 2.5) * bump(t, tin, tin + 0.15, dur - tout - 0.1, dur - tout);
            let to = [nk[0] + 0.085, nk[1] + 0.035 + rub, nk[2] - 0.075];
            let r = hand_rot(&fix_l, [-0.55, 0.35, -0.75], [0.45, 0.55, 0.55]);
            two_bone_ik(rig, d, fk.rot, fk.pos, rig.role(UPPER_ARM_L), rig.role(FOREARM_L), rig.role(HAND_L), &to, Some(&[0.75, 0.35, 0.8]), 0.9, Some(&r), false);
            hand(rig, d, hh, 0, &[0.5, 0.55, 0.6, 0.65], 0.5, 0.0);
        }
    });
    let (w_up, w_fore, w_hand) = staggered(t, tin, tout, dur);
    left_arm_mask(body, w_up, w_fore, w_hand);
    body.blend(at, at_b, 1.0, true, at);
    hands_to(&h, an);
}
/// A hand to the radio: the left hand up to the handset at his left collar, the head turned and tipped to it as far as his gun is down.
pub fn radio_pose(body: &mut Body, an: &mut Anim, w: f64, hw: f64) {
    let (at, at_b, _) = poses(an);
    let fix_l = an.hand_fix[0];
    body.pose(at, |d, rig, _fk| {
        if hw > 0.003 {
            turn(d, rig, rig.role(NECK), quat::mul(q_yaw(9.0 * DEGR * hw), q_pitch(5.0 * DEGR * hw)));
            turn(d, rig, rig.role(HEAD), quat::mul(q_roll(-5.0 * DEGR * hw), quat::mul(q_yaw(13.0 * DEGR * hw), q_pitch(8.0 * DEGR * hw))));
        }
    });
    let mut h = hands_of(body, an);
    body.copy(at, at_b);
    let hh = &mut h;
    body.pose(at_b, |d, rig, fk| {
        turn(d, rig, rig.role(SHOULDER_L), q_roll(7.0 * DEGR * w));
        fk_run(rig, d, fk.rot, fk.pos);
        let (sh, live0, rot0) = (fk.at(rig.role(UPPER_ARM_L)), fk.at(rig.role(HAND_L)), fk.rot(rig.role(HAND_L)));
        // under and ahead of the collar on that side: the fingers come up onto the handset
        let wrist = [sh[0] - 0.035, sh[1] - 0.115, sh[2] + 0.15];
        let r = quat::nlerp(rot0, hand_rot(&fix_l, [-0.4, 0.82, -0.4], [0.3, 0.3, 0.9]), w);
        two_bone_ik(rig, d, fk.rot, fk.pos, rig.role(UPPER_ARM_L), rig.role(FOREARM_L), rig.role(HAND_L), &v3::lerp3(live0, wrist, w), Some(&[0.75, -1.0, 0.1]), w, Some(&r), false);
        hand(rig, d, hh, 0, &[0.4, 0.5, 0.6, 0.65], 0.2, 0.0);
    });
    body.mask(&[(body.rig.role(SHOULDER_L), 1.0, true)]);
    body.blend(at, at_b, smoothstep(0.0, 0.25, w), true, at);
    hands_to(&h, an);
}
