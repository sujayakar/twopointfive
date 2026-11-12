//! Watching a man: the slip, the look, the arm.
use super::*;

/// He gives a man room as he passes, the body's share: by how near the man is the pelvis comes lower and off his side over feet left where they were,
/// and the trunk tips away from him with the head brought level back out of it.
pub fn watch_slip(body: &mut Body, an: &mut Anim, dt: f64, free: f64, there: Option<[f64; 3]>, tune: &Knobs) {
    let (at, slip_k, crouch, slip_lean, wt) = (an.p_a.at as usize, an.slip_k, an.crouch, an.slip_lean, &mut an.wt);
    let (there, m) = (there.is_some(), there.unwrap_or([0.0; 3]));
    body.pose(at, |d, rig, fk| {
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
        let (sw, sx, sl) = (wt.sw, wt.sx, wt.sl);
        if sw > 0.003 {
            let kk = sw * sw * (3.0 - 2.0 * sw);
            let (kl, kd) = (kk * sl, kk * (1.0 + tune.watch.squeeze * (1.0 - sl)));
            let (hips, foot_l, foot_r) = (rig.role(HIPS), rig.role(FOOT_L), rig.role(FOOT_R));
            fk.part(rig, d, rig.list(FK_LEGS));
            let (f_l, f_r, r_l, r_r) = (fk.at(foot_l), fk.at(foot_r), fk.rot(foot_l), fk.rot(foot_r));
            shift(d, rig, hips, &[-sx * tune.watch.away * kl, -tune.watch.drop * kd, 0.0]);
            fk.part(rig, d, rig.list(FK_LEGS));
            two_bone_ik(rig, d, fk.rot, fk.pos, rig.role(THIGH_L), rig.role(SHIN_L), foot_l, &f_l, Some(&knee_pole(&r_l)), 0.0, Some(&r_l), false);
            two_bone_ik(rig, d, fk.rot, fk.pos, rig.role(THIGH_R), rig.role(SHIN_R), foot_r, &f_r, Some(&knee_pole(&r_r)), 0.0, Some(&r_r), true);
            let roll = sx * tune.watch.lean * DEGR * kl;
            turn(d, rig, rig.role(SPINE1), q_roll(roll * 0.4));
            turn(d, rig, rig.role(SPINE2), q_roll(roll * 0.35));
            turn(d, rig, rig.role(SPINE3), q_roll(roll * 0.25));
            turn(d, rig, rig.role(NECK), q_roll(-roll * 0.4));
            turn(d, rig, rig.role(HEAD), q_roll(-roll * 0.6));
        }
    })
}
/// He watches a man: the head goes to the point (the chest a little, the neck, the head), within what a neck does of the chest's line; the angle follows
/// the point, and the whole comes and goes by its weight.
pub fn watch_look(body: &mut Body, an: &mut Anim, dt: f64, free: f64, there: Option<[f64; 3]>, tune: &Knobs) {
    let (at, watch_k, gaze_local, wt) = (an.p_a.at as usize, an.watch_k, an.gaze_local, &mut an.wt);
    let (there, m) = (there.is_some(), there.unwrap_or([0.0; 3]));
    body.pose(at, |d, rig, fk| {
        let (s3, nk, hd) = (rig.role(SPINE3), rig.role(NECK), rig.role(HEAD));
        let w_want = if there { clamp(watch_k, 0.0, 1.0) * free } else { 0.0 };
        wt.w = if abs(w_want - wt.w) < 0.003 { w_want } else { damp(wt.w, w_want, tune.watch.w_rate, dt) };
        if there && w_want > 0.0 {
            fk_run(rig, d, fk.rot, fk.pos);
            let hr = fk.rot(hd);
            let gaze = quat::rotate(hr, gaze_local);
            let eyes = v3::mad(v3::mad(fk.at(hd), quat::rotate(hr, AY), 0.1), gaze, 0.08);
            let want = v3::normalize(v3::sub(m, eyes));
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
            turn(d, rig, nk, quat::mul(q_yaw(yw * sh1), q_pitch(pw * sh1 / rest)));
            turn(d, rig, hd, quat::mul(q_yaw(yw * sh2), q_pitch(pw * sh2 / rest)));
        }
    })
}
/// The slip's arm: the one on the man's side goes out low toward him, the hand open. Solved on the scratch pose at `at_b` and laid back through that
/// shoulder's mask.
pub fn watch_arm(body: &mut Body, an: &mut Anim, wl: f64, wr: f64, tune: &Knobs) {
    let (at, at_b, _) = poses(an);
    let (t, f) = (an.wt, &an.hand_fix);
    let (sw, sl, left, sx, sz, fix) = (t.sw, t.sl, t.side == 0.0, t.sx, t.sz, [f[0], f[1]]);
    if !(sw > 0.003) {
        return;
    }
    let kl = sw * sw * (3.0 - 2.0 * sw) * sl;
    let sgn = if left { 1.0 } else { -1.0 };
    let w_arm = (if left { wl } else { wr }) * kl * tune.watch.arm_w * smoothstep(tune.watch.arm_side[0], tune.watch.arm_side[1], sgn * sx);
    if !(w_arm > 0.003) {
        return;
    }
    let mut h = hands_of(body, an);
    body.copy(at, at_b);
    let side = if left { 0 } else { 1 };
    let fix = fix[side];
    let (up, fore, hand_n, sh_n) = if left { (body.rig.role(UPPER_ARM_L), body.rig.role(FOREARM_L), body.rig.role(HAND_L), body.rig.role(SHOULDER_L)) } else { (body.rig.role(UPPER_ARM_R), body.rig.role(FOREARM_R), body.rig.role(HAND_R), body.rig.role(SHOULDER_R)) };
    let hh = &mut h;
    body.pose(at_b, |d, rig, fk| {
        fk_run(rig, d, fk.rot, fk.pos);
        let sh = fk.at(up);
        let to = [sh[0] + sx * tune.watch.arm_out, sh[1] - tune.watch.arm_down, sh[2] + sz * tune.watch.arm_out];
        let r = hand_rot(&fix, v3::normalize([sx * 0.75, -0.55, sz * 0.75]), [-sx * 0.3, 1.0, -sz * 0.3]);
        two_bone_ik(rig, d, fk.rot, fk.pos, up, fore, hand_n, &to, Some(&[sgn * 0.5, -0.3, -0.8]), 0.6, Some(&r), false);
        hand(rig, d, hh, side, &[0.12, 0.15, 0.18, 0.22], 0.15, 0.35);
    });
    body.mask(&[(sh_n, w_arm, true)]);
    body.blend(at, at_b, 1.0, true, at);
    hands_to(&h, an);
}
