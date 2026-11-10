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
            turn(d, rig, s1, quat::mul(q_yaw(tw * 0.25), q_roll(rl * 0.3)));
            turn(d, rig, s2, quat::mul(q_yaw(tw * 0.35), q_roll(rl * 0.35)));
            turn(d, rig, s3, quat::mul(q_yaw(tw * 0.4), q_roll(rl * 0.35)));
            turn(d, rig, nk, quat::mul(q_yaw(-tw * 0.5), q_roll(-rl * 0.5)));
            turn(d, rig, hd, quat::mul(q_yaw(-tw * 0.5), q_roll(-rl * 0.5)));
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
            let target = untwist(fk, tw_y, tw_p, Stands::at(p0, yin).model(&creep_at));
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
            let gz = quat::rotate(fk.rot(role(HEAD)), gaze_local);
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
        let gun_in = v3::lerp3([SNEAK.gun_in_stand[0], SNEAK.gun_in_stand[1], SNEAK.gun_in_stand[2]], [SNEAK.gun_in[0], SNEAK.gun_in[1], SNEAK.gun_in[2]], cr);
        let gun_r = v3::add(pr0, gun_in);
        // (the right hand lives at −x: in = toward the middle, never past it)
        let free_r = [pr0[0] + min(SNEAK.hand_in, max(0.0, -pr0[0])) * kk, pr0[1] - 0.02 * kk, pr0[2] - 0.03 * kk];
        let free_l = [pl0[0] - min(SNEAK.hand_in, max(0.0, pl0[0])) * kk, pl0[1] - 0.02 * kk, pl0[2] - 0.03 * kk];
        let pole_w = lerp(0.35, 0.7, r);
        two_bone_ik(rig, d, fk.rot, fk.pos, role(UPPER_ARM_R), role(FOREARM_R), hand_r, &v3::lerp3(free_r, gun_r, r), Some(&v3::lerp3([-0.05, 0.4, -1.0], [-0.12, -1.0, -0.5], r)), pole_w, Some(&rr0), false);
        let gun_l = v3::add(fk.at(hand_r), v3::sub(pl0, pr0));
        two_bone_ik(rig, d, fk.rot, fk.pos, role(UPPER_ARM_L), role(FOREARM_L), hand_l, &v3::lerp3(free_l, gun_l, r), Some(&v3::lerp3([0.05, 0.4, -1.0], [0.2, -1.0, -0.35], r)), pole_w, Some(&rl0), true);
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
/// What the steps of a creeping man's arms hand on to the ones after them.
#[derive(Clone, Copy, Default)]
struct CreepArms {
    w: f64,
    wl: f64,
    wr: f64,
    e: f64,
    cr: f64,
    ck: f64,
    cke: f64,
    ws: f64,
    ha: f64,
    hg: f64,
    rc_inv: [f64; 4],
    // as_creeps: how he creeps, what he holds, and how wide his arms go
    at: usize,
    at_b: usize,
    cl_t: f64,
    held: bool,
    no_knife: bool,
    crouch: f64,
    swing: f64,
    wsh: [f64; 4],
    wide_pole: [f64; 3],
    fix_l: [f64; 4],
    fix_r: [f64; 4],
    // weights: how much of each arm is the layer's
    on_l: bool,
    on_r: bool,
    w_up: f64,
    w_fore: f64,
    w_hand: f64,
    dealt: bool,
    lw: f64,
    // creep_shoulders: the shoulders rounded, and which way they face
    sl: [f64; 3],
    sr: [f64; 3],
    fx: f64,
    fz: f64,
    // alive: the hands alive: a slow working of the fingers and a drift of the wrists on the layer's own clock (two rates a hand, never in step)
    lt: f64,
    fl_l: f64,
    fl_r: f64,
    // left_shape: the left hand out in front, its drift at its full size through the part of this arm the layer has
    pole: [f64; 3],
    fing: [f64; 3],
    back: [f64; 3],
    cu: f64,
    th: f64,
    sp: f64,
    to: [f64; 3],
    // right_shape: the right hand by its guard
    a0: [f64; 3],
    p0: [f64; 3],
    curl: f64,
    thb: f64,
    spr: f64,
    // right_point: the right arm's point
    rot: [f64; 4],
}

/// The close-in carriage's arms, after its body: the shoulders rounded and the hands brought up ahead of the hips in the shoulders' frame, let out wide
/// and low by `ws`, alive with a slow working of the fingers when the distance is dealt, and the knife hand taken up across the chest to hover off the
/// hilt as he closes. Solved on the scratch pose at `at_b`, laid back joint by joint down each arm.
pub fn creep_arms(g: usize, an: &mut Anim, w: f64, wl: f64, wr: f64, e: f64, cr: f64, ck: f64, cke: f64, ws: f64, ha: f64, hg: f64, rc_inv: [f64; 4], tune: &Knobs) -> u32 {
    let mut o = CreepArms { w, wl, wr, e, cr, ck, cke, ws, ha, hg, rc_inv, ..CreepArms::default() };
    as_creeps(an, &mut o);
    let Some(Some(n)) = with_pose(g, o.at, |_d, _rig, n, _fk| if cast(n, &ARM_ROLES) { Some(n) } else { None }) else {
        return 0;
    };
    let Some(mut h) = hands_of(g, an) else {
        return 0;
    };
    if !ask_blend(g, o.at, o.at, 1.0, false, o.at_b, n) {
        return 0;
    }
    weights(tune, &mut o);
    let hh = &mut h;
    with_pose(g, o.at_b, |d, rig, _n, fk| {
        creep_shoulders(d, rig, fk, &mut o);
        alive(tune, &mut o);
        if o.on_l {
            left_shape(tune, &mut o);
            left_wide(&mut o);
            left_arm(hh, d, rig, fk, &o);
        }
        if o.on_r {
            right_shape(tune, &mut o);
            right_wide(&mut o);
            right_point(&mut o);
            by_distance(tune, fk, &mut o);
            right_arm(hh, d, rig, fk, &o);
        }
    });
    // (an arm the layer has none of is left out of the mask: a node no rig has is skipped)
    let (l, r) = (|i: usize| if o.on_l { role(i) } else { usize::MAX }, |i: usize| if o.on_r { role(i) } else { usize::MAX });
    ask_mask(&[
        (l(SHOULDER_L), o.w_up * o.lw, false),
        (l(UPPER_ARM_L), o.w_up * o.lw, false),
        (l(FOREARM_L), o.w_fore * o.lw, false),
        (l(HAND_L), o.w_hand * o.lw, true),
        (r(SHOULDER_R), o.w_up * wr, false),
        (r(UPPER_ARM_R), o.w_up * wr, false),
        (r(FOREARM_R), o.w_fore * wr, false),
        (r(HAND_R), o.w_hand * wr, true),
    ]);
    if !ask_blend(g, o.at, o.at_b, 1.0, true, o.at, n) {
        return 0;
    }
    hands_to(&h, an);
    RAN | RAN_ALL
}

/// How he creeps, what he holds, and how wide his arms go.
fn as_creeps(an: &Anim, o: &mut CreepArms) {
    let (at, at_b, _) = poses(an);
    let (cl_t, held, no_knife, crouch, swing, wsh) = (an.cl_t, an.grip[1], an.no_knife, an.crouch, an.heft_shape[1], an.wide_shape);
    let (wide_pole, fix_l, fix_r) = (an.wide_pole.unwrap_or(CREEP.wide.pole), an.hand_fix[0], an.hand_fix[1]);
    (o.at, o.at_b, o.cl_t, o.held, o.no_knife, o.crouch, o.swing, o.wsh, o.wide_pole, o.fix_l, o.fix_r) = (at, at_b, cl_t, held, no_knife, crouch, swing, wsh, wide_pole, fix_l, fix_r);
}

/// How much of each arm is the layer's.
fn weights(tune: &Knobs, o: &mut CreepArms) {
    let CreepArms { w, wl, wr, ck, ws, .. } = *o;
    let (on_l, on_r) = (wl > 0.003, wr > 0.003);
    let (w_up, w_fore, w_hand) = (smoothstep(0.0, 0.75, w), smoothstep(0.08, 0.88, w), smoothstep(0.16, 1.0, w));
    let dealt = ck >= 0.0;
    let lw = wl * if dealt { lerp(tune.kit.close.left, 1.0, ws) } else { 1.0 };
    (o.on_l, o.on_r, o.w_up, o.w_fore, o.w_hand, o.dealt, o.lw) = (on_l, on_r, w_up, w_fore, w_hand, dealt, lw);
}

/// The shoulders rounded, and which way they face.
fn creep_shoulders(d: &mut [S], rig: &Rig, fk: &mut Fk, o: &mut CreepArms) {
    let CreepArms { e, on_l, on_r, .. } = *o;
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
    let l = or_one(hypot2(fx, fz));
    fx /= l;
    fz /= l;
    (o.sl, o.sr, o.fx, o.fz) = (sl, sr, fx, fz);
}

/// Left, up and forward off the shoulder line, as model axes.
fn sq(o: &CreepArms, v: [f64; 3]) -> [f64; 3] {
    let CreepArms { fx, fz, .. } = *o;
    [fz * v[0] + fx * v[2], v[1], fz * v[2] - fx * v[0]]
}

/// The hands alive: a slow working of the fingers and a drift of the wrists on the layer's own clock (two rates a hand, never in step).
fn alive(tune: &Knobs, o: &mut CreepArms) {
    let CreepArms { cl_t, dealt, .. } = *o;
    let lt = cl_t * PI * 2.0 * tune.kit.close.flex;
    let fl_l = if dealt { tune.kit.close.flex_amp * (0.6 * sin(lt) + 0.4 * sin(lt * 1.63 + 1.1)) } else { 0.0 };
    let fl_r = if dealt { tune.kit.close.flex_amp * (0.6 * sin(lt * 1.21 + 2.3) + 0.4 * sin(lt * 0.71 + 0.4)) } else { 0.0 };
    (o.lt, o.fl_l, o.fl_r) = (lt, fl_l, fl_r);
}

/// A wrist's drift.
fn drift(tune: &Knobs, o: &CreepArms, ph: f64) -> [f64; 3] {
    let CreepArms { dealt, lt, .. } = *o;
    if dealt {
        [tune.kit.close.drift * sin(lt * 0.83 + ph), tune.kit.close.drift * sin(lt * 1.17 + ph * 1.7), tune.kit.close.drift * 0.6 * sin(lt * 0.61 + ph * 0.6)]
    } else {
        [0.0, 0.0, 0.0]
    }
}

/// The left hand out in front, its drift at its full size through the part of this arm the layer has.
fn left_shape(tune: &Knobs, o: &mut CreepArms) {
    let CreepArms { cr, cke, ws, dealt, .. } = *o;
    let hd = &CREEP.l;
    let (pole, fing, back, cu, th, sp) = (hd.pole, hd.fing, hd.back, hd.curl, hd.thumb, hd.spread);
    let to = v3::add(
        v3::mad(v3::lerp3(hd.at, hd.at_c, cr), tune.kit.close.lead, cke),
        v3::scale(drift(tune, o, 0.0), if dealt { 1.0 / max(0.38, lerp(tune.kit.close.left, 1.0, ws)) } else { 1.0 }),
    );
    (o.pole, o.fing, o.back, o.cu, o.th, o.sp, o.to) = (pole, fing, back, cu, th, sp, to);
}

/// Out to its side, and still.
fn left_wide(o: &mut CreepArms) {
    let CreepArms { ws, ha, hg, swing, wsh, wide_pole, mut pole, mut fing, mut back, mut cu, mut th, mut sp, mut to, .. } = *o;
    if ws > 0.0 {
        let s = -ha * swing;
        to = v3::lerp3(to, [wsh[0] + s * CREEP.heft.out, -wsh[1] + abs(s) * CREEP.heft.up, wsh[2] + wsh[3] + s - hg * 0.1], ws);
        pole = v3::lerp3(pole, wide_pole, ws);
        fing = v3::lerp3(fing, CREEP.wide.fing, ws);
        back = v3::lerp3(back, CREEP.wide.back, ws);
        cu = lerp(cu, CREEP.wide.curl, ws);
        th = lerp(th, CREEP.wide.thumb, ws);
        sp = lerp(sp, CREEP.wide.spread, ws);
    }
    (o.pole, o.fing, o.back, o.cu, o.th, o.sp, o.to) = (pole, fing, back, cu, th, sp, to);
}

/// The left arm there.
fn left_arm(hh: &mut Hands, d: &mut [S], rig: &Rig, fk: &mut Fk, o: &CreepArms) {
    let CreepArms { fix_l, sl, fl_l, pole, fing, back, cu, th, sp, to, .. } = *o;
    let r = hand_rot(&fix_l, sq(o, fing), sq(o, back));
    two_bone_ik(rig, d, fk.rot, fk.pos, role(UPPER_ARM_L), role(FOREARM_L), role(HAND_L), &v3::add(sl, sq(o, to)), Some(&sq(o, pole)), 0.8, Some(&r), false);
    hand(d, hh, 0, &[cu + fl_l, cu + 0.05 + fl_l * 0.9, cu + 0.1 + fl_l * 0.8, cu + 0.15 + fl_l * 0.7], th + fl_l * 0.5, sp);
}

/// The left's mirror.
fn mir(v: [f64; 3]) -> [f64; 3] {
    [-v[0], v[1], v[2]]
}

/// The right hand by its guard.
fn right_shape(tune: &Knobs, o: &mut CreepArms) {
    let CreepArms { cr, cke, .. } = *o;
    let hd = &CREEP.r;
    let (a0, p0, fing, back) = (v3::add(v3::lerp3(hd.at, hd.at_c, cr), drift(tune, o, 2.1)), hd.pole, hd.fing, hd.back);
    let (curl, thb, spr) = (hd.curl, hd.thumb, hd.spread * (1.0 - cke));
    (o.fing, o.back, o.a0, o.p0, o.curl, o.thb, o.spr) = (fing, back, a0, p0, curl, thb, spr);
}

/// The left's mirror.
fn right_wide(o: &mut CreepArms) {
    let CreepArms { ws, ha, hg, swing, wsh, wide_pole, mut fing, mut back, mut a0, mut p0, mut curl, mut thb, mut spr, .. } = *o;
    if ws > 0.0 {
        let s = ha * swing;
        a0 = v3::lerp3(a0, [-wsh[0] - s * CREEP.heft.out, -wsh[1] + abs(s) * CREEP.heft.up, wsh[2] - wsh[3] + s - hg * 0.1], ws);
        p0 = v3::lerp3(p0, mir(wide_pole), ws);
        fing = v3::lerp3(fing, mir(CREEP.wide.fing), ws);
        back = v3::lerp3(back, mir(CREEP.wide.back), ws);
        curl = lerp(curl, CREEP.wide.curl, ws);
        thb = lerp(thb, CREEP.wide.thumb, ws);
        spr = lerp(spr, CREEP.wide.spread, ws);
    }
    (o.fing, o.back, o.a0, o.p0, o.curl, o.thb, o.spr) = (fing, back, a0, p0, curl, thb, spr);
}

/// The right arm's point.
fn right_point(o: &mut CreepArms) {
    let CreepArms { fix_r, sr, fing, back, a0, p0, .. } = *o;
    let (to, rot, pole) = (v3::add(sr, sq(o, a0)), hand_rot(&fix_r, sq(o, fing), sq(o, back)), sq(o, p0));
    (o.pole, o.to, o.rot) = (pole, to, rot);
}

/// The knife hand by distance: from its guard up across the chest to hover off the hilt.
fn by_distance(tune: &Knobs, fk: &mut Fk, o: &mut CreepArms) {
    let CreepArms { ck, cke, rc_inv, held, no_knife, crouch, fix_r, dealt, mut pole, mut to, mut curl, mut rot, .. } = *o;
    if dealt && !held && !no_knife {
        let (s_at, s_rot, s_pole) = sheath_fist(fk, &rc_inv, &fix_r, crouch, 0.0, tune.kit.close.hover);
        // (the travel half-eased: the hand must be seen to go with every step)
        let cp = cke + (ck - cke) * 0.5;
        to = v3::add(v3::mad(v3::lerp3(to, s_at, cp), sq(o, tune.kit.close.bow), sin(PI * cke)), v3::scale(sq(o, drift(tune, o, 2.1)), cke));
        rot = quat::slerp(rot, s_rot, cke);
        pole = v3::lerp3(pole, s_pole, cke);
        curl = lerp(CREEP.r.curl, tune.kit.close.curl, cke);
    }
    (o.pole, o.to, o.curl, o.rot) = (pole, to, curl, rot);
}

/// The right arm there.
fn right_arm(hh: &mut Hands, d: &mut [S], rig: &Rig, fk: &mut Fk, o: &CreepArms) {
    let CreepArms { held, fl_r, pole, to, curl, thb, spr, rot, .. } = *o;
    two_bone_ik(rig, d, fk.rot, fk.pos, role(UPPER_ARM_R), role(FOREARM_R), role(HAND_R), &to, Some(&pole), 0.8, Some(&rot), false);
    if !held {
        hand(d, hh, 1, &[curl + fl_r, curl + 0.05 + fl_r * 0.9, curl + 0.1 + fl_r * 0.8, curl + 0.15 + fl_r * 0.7], thb + fl_r * 0.5, spr);
    }
}
