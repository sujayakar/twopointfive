//! The stealth poses, and the wand.
use super::*;

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
    if !ask_blend(g, at, at, 1.0, false, at_b, n) {
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
            hand(d, hh, side, &[c, c + 0.04, c + 0.08, c + 0.12], STEALTH.idle.thumb, STEALTH.idle.spread);
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
    if !ask_blend(g, at, at_b, 1.0, true, at, n) {
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
        if nod != 0.0 {
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
    if !ask_blend(g, at, at, 1.0, false, at_b, n) {
        return 0;
    }
    let hh = &mut h;
    with_pose(g, at_b, |d, rig, _n, fk| {
        fk_run(rig, d, fk.rot, fk.pos);
        let kl = fk.at(role(SHIN_L));
        let (rr, rl) = (hand_rot(&fix_r, [0.05, -0.35, 0.94], [-0.3, 0.9, 0.3]), hand_rot(&fix_l, [-0.15, -0.85, 0.5], [0.45, 0.25, 0.85]));
        two_bone_ik(rig, d, fk.rot, fk.pos, role(UPPER_ARM_R), role(FOREARM_R), role(HAND_R), &[STEALTH.cable.gap[0], STEALTH.cable.gap[1], lock_z - STEALTH.cable.short], Some(&[-0.6, 0.1, -0.75]), 0.8, Some(&rr), false);
        two_bone_ik(rig, d, fk.rot, fk.pos, role(UPPER_ARM_L), role(FOREARM_L), role(HAND_L), &[kl[0] + STEALTH.cable.knee[0], kl[1] + STEALTH.cable.knee[1], kl[2] + STEALTH.cable.knee[2]], Some(&[0.8, 0.0, -0.6]), 0.8, Some(&rl), false);
        hand(d, hh, 1, &[0.5, 0.6, 0.7, 0.75], 0.5, 0.0);
        hand(d, hh, 0, &[0.2, 0.24, 0.28, 0.32], 0.15, 0.3);
    });
    arms_mask();
    if !ask_blend(g, at, at_b, e, true, at, n) {
        return 0;
    }
    hands_to(&h, an);
    RAN | RAN_ALL
}
