//! The careful step and the close-in mode: the body, the arms and the gaze.
use super::*;

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
/// The sneak's arms: hanging hands drawn in toward the thighs, or the two-hand carry brought down and back toward the lap, the support hand rigid with
/// wherever the gun hand got to.
pub fn sneak_arms(g: usize, an: &Anim, arms_in: f64) -> u32 {
    let (at, at_b, _) = poses(an);
    let (crouch, at_ease) = (an.crouch, an.upper_w.relaxed);
    let Some(Some(n)) = with_pose(g, at, |_d, _rig, n, _fk| if cast(n, &ARM_ROLES) { Some(n) } else { None }) else {
        return 0;
    };
    if !ask_blend(g, at, at, 1.0, false, at_b, n) {
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
    if !ask_blend(g, at, at_b, clamp(arms_in, 0.0, 1.0), true, at, n) {
        return 0;
    }
    code
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
    if !ask_blend(g, at, at, 1.0, false, at_b, n) {
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
            hand(d, hh, 0, &[cu + fl_l, cu + 0.05 + fl_l * 0.9, cu + 0.1 + fl_l * 0.8, cu + 0.15 + fl_l * 0.7], th + fl_l * 0.5, sp);
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
                hand(d, hh, 1, &[curl + fl_r, curl + 0.05 + fl_r * 0.9, curl + 0.1 + fl_r * 0.8, curl + 0.15 + fl_r * 0.7], thb + fl_r * 0.5, spr);
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
    if !ask_blend(g, at, at_b, 1.0, true, at, n) {
        return 0;
    }
    hands_to(&h, an);
    RAN | RAN_ALL
}
