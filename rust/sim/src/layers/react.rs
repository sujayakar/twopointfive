//! What is done to him: a hit, a flinch, a startle, a blow, hands bound, a wound held.
use super::*;

/// How far what has happened to him leans him, piece upon piece: degrees of the trunk and the head, metres of the hips.
#[derive(Clone, Copy, Default)]
struct Lean {
    dt: f64,
    gate: f64,
    // as_he_is: how he is, as the rules have said it
    at: usize,
    trained: f64,
    lg_w: f64,
    blind: bool,
    blind_k: f64,
    clutch: f64,
    fold_too: bool,
    hurt: f64,
    hurt_side: f64,
    aim: f64,
    speed: f64,
    phase: f64,
    shaken: f64,
    // as_it_struck: the way each thing came from, as it struck
    fl_k_to: f64,
    fl_d: [f64; 3],
    st_d: [f64; 3],
    ht_d: [f64; 3],
    ht_side: f64,
    dz_d: [f64; 3],
    ts: f64,
    rel: f64,
    // none_yet: how trained he is, and nothing leant yet
    t: f64,
    pitch: f64,
    yaw: f64,
    roll: f64,
    hp: f64,
    hy: f64,
    drop: f64,
    sx: f64,
    sz: f64,
    stw: f64,
    stn: f64,
    // blinded: the side a blinded man turns away to is chosen once, as it goes off
    dzw: f64,
    // fold: the clutch's fold: under the blinded hunch it gives way by the hunch's own weight
    cfw: f64,
    // hurt, he hunches over it and steps heavy on that side; it gives way under the fold and the blinded hunch
    under: f64,
}

/// After the stances and the arms, the body's share of the reactions: the flinch, the start, the hit, the blinded hunch, the clutch's fold, a hurt man's
/// hunch and heavy step, a shaken man's ducked head and the toss's wind-up, summed into one turn of the trunk, one of the head and one move of the
/// pelvis over feet held where they are. The arms that follow (the start's, the blinded hand, the held gun, the toss) are the host's.
pub fn mv_post_body(g: usize, an: &mut Anim, dt: f64, gate: f64, tune: &Knobs) -> u32 {
    let mut o = Lean { dt, gate, ..Lean::default() };
    as_he_is(an, &mut o);
    let (m, po) = (&mut an.mvs, &mut an.mv_po);
    as_it_struck(m, &mut o);
    with_pose(g, o.at, |d, rig, n, fk| {
        if !cast(n, &LEG_ROLES) || !cast(n, &[SPINE1, SPINE2, SPINE3, NECK, HEAD, HAND_R]) {
            return 0;
        }
        none_yet(&mut o);
        flinch(tune, m, &mut o);
        startle(m, &mut o);
        hit(tune, m, &mut o);
        blinded(tune, m, &mut o);
        fold(m, &mut o);
        // a man's condition: hurt, then shaken; both give way under the fold and the blinded hunch
        hurt(m, &mut o);
        shaken(m, &mut o);
        toss_english(&mut o);
        let mut ran = RAN;
        if o.drop > 2e-4 || abs(o.sx) + abs(o.sz) > 2e-4 {
            fk_run(rig, d, fk.rot, fk.pos);
            mv_hips(d, rig, fk, &[o.sx, -o.drop, o.sz]);
            ran |= RAN_ALL;
        }
        // a blinded man's gun stays on the line it had: where the trigger hand is before the trunk turns away
        po.gun = false;
        if o.dzw > 0.003 && o.lg_w > 0.02 {
            fk_run(rig, d, fk.rot, fk.pos);
            ran |= RAN_ALL;
            let hand = role(HAND_R);
            po.gun = true;
            po.gun_p = fk.at(hand);
            po.gun_r = fk.rot(hand);
        }
        trunk_turn(d, rig, &o);
        head_turn(d, rig, &o);
        po.stw = o.stw;
        po.stn = o.stn;
        po.dzw = o.dzw;
        ran
    })
    .unwrap_or(0)
}

/// How he is, as the rules have said it.
fn as_he_is(an: &Anim, o: &mut Lean) {
    let (at, trained, lg_w, blind, blind_k) = (an.p_a.at as usize, an.trained, an.lg.w.x, an.dazzle.is_some(), an.dazzle.map_or(0.0, |z| z.k));
    let (clutch, fold_too, hurt, hurt_side, aim, speed, phase, shaken) = (an.clutch, an.fold_too, an.hurt, an.hurt_side, an.upper_w.aim, an.speed, an.phase, an.shaken);
    *o = Lean { at, trained, lg_w, blind, blind_k, clutch, fold_too, hurt, hurt_side, aim, speed, phase, shaken, ..*o };
}

/// The way each thing came from, as it struck.
fn as_it_struck(m: &mut game_types::character::AnimMvs, o: &mut Lean) {
    let (fl_k_to, fl_d, st_d, ht_d, ht_side, dz_d, ts, rel) = (m.fl_k_to, m.fl_d, m.st_d, m.ht_d, m.ht_side, m.dz_d, m.ts_t, m.ts_rel);
    *o = Lean { fl_k_to, fl_d, st_d, ht_d, ht_side, dz_d, ts, rel, ..*o };
}

/// How trained he is, and nothing leant yet.
fn none_yet(o: &mut Lean) {
    let Lean { trained, .. } = *o;
    let t = clamp(trained, 0.0, 1.0);
    let (pitch, yaw, roll, hp, hy, drop, sx, sz) = (0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64);
    let (stw, stn) = (0.0f64, 1.0f64);
    *o = Lean { t, pitch, yaw, roll, hp, hy, drop, sx, sz, stw, stn, ..*o };
}

/// Between what an untrained man does and what a trained one does.
fn kk(o: &Lean, p0: f64, p1: f64) -> f64 {
    let Lean { t, .. } = *o;
    lerp(p0, p1, t)
}

/// A round close by.
fn flinch(tune: &Knobs, m: &mut game_types::character::AnimMvs, o: &mut Lean) {
    let Lean { dt, gate, fl_k_to, fl_d, mut pitch, mut yaw, mut roll, mut hp, mut hy, mut drop, mut sx, mut sz, .. } = *o;
    if m.fl_t >= 0.0 {
        m.fl_k += (fl_k_to - m.fl_k) * min(1.0, dt * 12.0);
        let dur = kk(o, tune.mv.flinch.dur[0], tune.mv.flinch.dur[1]);
        let e = curve(m.fl_t / dur, &MV_FLINCH_ENV) * kk(o, tune.mv.flinch.amp[0], tune.mv.flinch.amp[1]) * m.fl_k * gate;
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
    *o = Lean { pitch, yaw, roll, hp, hy, drop, sx, sz, ..*o };
}

/// The start: reared back off a thing in front, turned toward the side it is on; a thing behind him he comes round to.
fn startle(m: &mut game_types::character::AnimMvs, o: &mut Lean) {
    let Lean { dt, gate, lg_w, st_d, mut pitch, mut yaw, mut roll, mut hp, mut hy, mut drop, mut sx, mut sz, mut stw, mut stn, .. } = *o;
    if m.st_t >= 0.0 {
        let (dur, lw, dv) = (kk(o, MV_STARTLE.dur[0], MV_STARTLE.dur[1]), clamp(lg_w, 0.0, 1.0), st_d);
        let amp = kk(o, MV_STARTLE.amp[0], MV_STARTLE.amp[1]);
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
    *o = Lean { pitch, yaw, roll, hp, hy, drop, sx, sz, stw, stn, ..*o };
}

/// A hit: shoved and turned the way the round went.
fn hit(tune: &Knobs, m: &mut game_types::character::AnimMvs, o: &mut Lean) {
    let Lean { dt, gate, ht_d, ht_side, mut pitch, mut yaw, mut roll, mut hp, mut hy, mut drop, mut sx, mut sz, .. } = *o;
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
    *o = Lean { pitch, yaw, roll, hp, hy, drop, sx, sz, ..*o };
}

/// The side a blinded man turns away to is chosen once, as it goes off.
fn blinded(tune: &Knobs, m: &mut game_types::character::AnimMvs, o: &mut Lean) {
    let Lean { dt, gate, blind, blind_k, dz_d, mut pitch, mut yaw, mut roll, mut hp, mut hy, mut drop, mut sx, mut sz, .. } = *o;
    let dz_t = if blind { clamp(blind_k, 0.0, 1.0) } else { 0.0 };
    let rate = if dz_t > m.dz.x { tune.mv.dazzle.rate_in } else { tune.mv.dazzle.rate_out };
    spring_step(&mut m.dz, dz_t, rate, dt);
    if dz_t == 0.0 && m.dz.x < 0.02 {
        m.dz.x = 0.0;
        m.dz.v = 0.0;
    }
    let dzw = clamp(m.dz.x, 0.0, 1.0) * kk(o, tune.mv.dazzle.amp[0], tune.mv.dazzle.amp[1]) * gate;
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
    *o = Lean { pitch, yaw, roll, hp, hy, drop, sx, sz, dzw, ..*o };
}

/// The clutch's fold: under the blinded hunch it gives way by the hunch's own weight.
fn fold(m: &mut game_types::character::AnimMvs, o: &mut Lean) {
    let Lean { dt, gate, clutch, fold_too, mut pitch, mut roll, mut hp, mut drop, mut sx, dzw, .. } = *o;
    let want = if clutch > 0.0 && fold_too { clamp(clutch, 0.0, 1.0) * (1.0 - clamp(dzw, 0.0, 1.0)) } else { 0.0 };
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
    *o = Lean { pitch, roll, hp, drop, sx, cfw, ..*o };
}

/// Hurt, he hunches over it and steps heavy on that side; it gives way under the fold and the blinded hunch.
fn hurt(m: &mut game_types::character::AnimMvs, o: &mut Lean) {
    let Lean { dt, gate, hurt, hurt_side, aim, speed, phase, mut pitch, mut yaw, mut roll, mut hp, mut drop, mut sx, dzw, cfw, .. } = *o;
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
    *o = Lean { pitch, yaw, roll, hp, drop, sx, under, ..*o };
}

/// Shaken, he ducks his head; it gives way the same.
fn shaken(m: &mut game_types::character::AnimMvs, o: &mut Lean) {
    let Lean { dt, gate, aim, shaken, mut pitch, mut hp, mut drop, under, .. } = *o;
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
    *o = Lean { pitch, hp, drop, ..*o };
}

/// The toss's body English: the left shoulder back with the hand, then through with it; a dip of the knees under the swing.
fn toss_english(o: &mut Lean) {
    let Lean { gate, ts, rel, mut pitch, mut yaw, mut drop, .. } = *o;
    if ts >= 0.0 {
        let td = rel + TOSS_AFTER;
        let wind = curve(ts, &[[0.0, 0.0], [rel * 0.8, 1.0], [rel + 0.03, 0.2], [rel + 0.2, -1.0], [rel + 0.4, -0.6], [td, 0.0]]) * gate;
        yaw += 9.0 * wind;
        pitch += 5.0 * max(0.0, -wind) + 2.0 * max(0.0, wind);
        drop += 0.02 * abs(wind);
    }
    *o = Lean { pitch, yaw, drop, ..*o };
}

/// The trunk turned.
fn trunk_turn(d: &mut [S], rig: &Rig, o: &Lean) {
    let Lean { pitch, yaw, roll, .. } = *o;
    if abs(pitch) + abs(yaw) + abs(roll) > 1e-3 {
        let q = |ky: f64, kp: f64| q_mul(&q_yaw(yaw * DEGR * ky), &q_mul(&q_roll(roll * DEGR * ky), &q_pitch(pitch * DEGR * kp)));
        turn(d, rig, role(SPINE1), q(0.3, 0.4));
        turn(d, rig, role(SPINE2), q(0.35, 0.35));
        turn(d, rig, role(SPINE3), q(0.35, 0.25));
    }
}

/// The head turned.
fn head_turn(d: &mut [S], rig: &Rig, o: &Lean) {
    let Lean { hp, hy, .. } = *o;
    if abs(hp) + abs(hy) > 1e-3 {
        let q = |kq: f64| q_mul(&q_yaw(hy * DEGR * kq), &q_pitch(hp * DEGR * kq));
        turn(d, rig, role(NECK), q(0.4));
        turn(d, rig, role(HEAD), q(0.6));
    }
}
/// What the arms of it are made of.
#[derive(Clone, Copy, Default)]
struct Arms {
    dt: f64,
    gate: f64,
    // as_moved: how he moved, as the body's share left it
    at: usize,
    at_b: usize,
    lg_w: f64,
    gaze: [f64; 3],
    fix_l: [f64; 4],
    rel: f64,
    dz_t: f64,
    ts: f64,
    stw: f64,
    stn: f64,
    dzw: f64,
    gun: bool,
    gun_p: [f64; 3],
    gun_r: [f64; 4],
    td: f64,
    arm_l: f64,
    ts_out: f64,
    code: u32,
    up_l: usize,
    fore_l: usize,
    hand_l: usize,
    sh_l: usize,
}

/// The reactions' arms, after their body: the start's shoulders and elbows on the pose at `at`; then, each solved on the scratch pose at `at_b` and laid
/// back through its own mask, a blinded man's gun held on the line it had, his hand flat over his eyes, and the toss of the support hand along its keyed
/// path.
pub fn mv_post_arms(g: usize, an: &mut Anim, dt: f64, gate: f64, tune: &Knobs) -> u32 {
    let mut o = Arms { dt, gate, ..Arms::default() };
    as_moved(an, &mut o);
    let Some(Some(n)) = with_pose(g, o.at, |_d, _rig, n, _fk| if cast(n, &ARM_ROLES) && cast(n, &[HEAD]) { Some(n) } else { None }) else {
        return 0;
    };
    let Some(mut h) = hands_of(g, an) else {
        return 0;
    };
    start_arms(g, &o);
    if !held_gun(g, n, &mut o) {
        return 0;
    }
    if !blinded_hand(g, n, tune, &mut h, &mut o) {
        return 0;
    }
    if !toss_hand(g, n, &mut h, &mut o) {
        return 0;
    }
    (an.mvs.ts_t, an.mvs.arm_l) = (o.ts_out, o.arm_l);
    hands_to(&h, an);
    o.code
}

/// How he moved, as the body's share left it.
fn as_moved(an: &Anim, o: &mut Arms) {
    let (at, at_b, _) = poses(an);
    let (lg_w, gaze, fix_l, m, po) = (an.lg.w.x, an.gaze_local, an.hand_fix[0], &an.mvs, &an.mv_po);
    let (rel, dz_t, ts, stw, stn, dzw, gun, gun_p, gun_r) = (m.ts_rel, m.dz_t, m.ts_t, po.stw, po.stn, po.dzw, po.gun, po.gun_p, po.gun_r);
    let (td, arm_l, ts_out, code) = (rel + TOSS_AFTER, m.arm_l, ts, RAN);
    let (up_l, fore_l, hand_l, sh_l) = (role(UPPER_ARM_L), role(FOREARM_L), role(HAND_L), role(SHOULDER_L));
    *o = Arms { at, at_b, lg_w, gaze, fix_l, rel, dz_t, ts, stw, stn, dzw, gun, gun_p, gun_r, td, arm_l, ts_out, code, up_l, fore_l, hand_l, sh_l, ..*o };
}

/// The start's arms: shoulders up, elbows wide and forearms half raised by what a long gun leaves of them.
fn start_arms(g: usize, o: &Arms) {
    let Arms { at, lg_w, stw, stn, sh_l, .. } = *o;
    if stw > 0.003 {
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
}

/// A blinded man's gun stays on the line it had, by how much of a long gun he has up.
fn held_gun(g: usize, n: usize, o: &mut Arms) -> bool {
    let Arms { at, at_b, lg_w, dzw, gun, gun_p, gun_r, mut code, .. } = *o;
    if gun {
        if !ask_blend(g, at, at, 1.0, false, at_b, n) {
            return false;
        }
        with_pose(g, at_b, |d, rig, _n, fk| {
            fk_run(rig, d, fk.rot, fk.pos);
            two_bone_ik(rig, d, fk.rot, fk.pos, role(UPPER_ARM_R), role(FOREARM_R), role(HAND_R), &gun_p, Some(&[-0.3, -1.0, -0.4]), 0.6, Some(&gun_r), false);
        });
        ask_mask(&[(role(UPPER_ARM_R), smoothstep(0.003, 0.15, dzw) * smoothstep(0.02, 0.6, lg_w), true)]);
        if !ask_blend(g, at, at_b, 1.0, true, at, n) {
            return false;
        }
        code |= RAN_ALL;
    }
    *o = Arms { code, ..*o };
    true
}

/// Blinded: the support hand goes flat over the eyes, wrist by the near eye, fingers across to the far one.
fn blinded_hand(g: usize, n: usize, tune: &Knobs, h: &mut Hands, o: &mut Arms) -> bool {
    let Arms { at, at_b, gaze, fix_l, dz_t, dzw, mut arm_l, mut code, up_l, fore_l, hand_l, sh_l, .. } = *o;
    if dzw > 0.003 {
        if !ask_blend(g, at, at, 1.0, false, at_b, n) {
            return false;
        }
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
            hand(d, h, 0, &[0.22, 0.28, 0.34, 0.4], 0.3, 0.3);
        });
        let wz = smoothstep(0.08, 0.6, dzw);
        ask_mask(&[(sh_l, wz, true)]);
        if !ask_blend(g, at, at_b, 1.0, true, at, n) {
            return false;
        }
        arm_l = max(arm_l, wz);
        code |= RAN_ALL;
    }
    *o = Arms { arm_l, code, ..*o };
    true
}

/// The toss: the support hand off the gun, down beside the hip, through and up, back to the gun (a path off the shoulder; its ends are the live hand).
fn toss_hand(g: usize, n: usize, h: &mut Hands, o: &mut Arms) -> bool {
    let Arms { dt, gate, at, at_b, fix_l, rel, ts, td, mut arm_l, mut ts_out, mut code, up_l, fore_l, hand_l, sh_l, .. } = *o;
    if ts >= 0.0 {
        if !ask_blend(g, at, at, 1.0, false, at_b, n) {
            return false;
        }
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
            hand(d, h, 0, &[c(0.6, 0.05, 0.5), c(0.65, 0.08, 0.55), c(0.7, 0.1, 0.6), c(0.78, 0.14, 0.68)], c(0.7, 0.1, 0.35), 0.25 * open * (1.0 - recup));
            // the wrist's turn eases from and back to the grip it had on the gun at the ends (the path does the same for its place)
            wrist_ease(d, rig, fk, hand_l, &rot0, bump(ts, 0.0, 0.08, td - 0.22, td - 0.04));
        });
        let hold = bump(ts, 0.0, 0.05, td - 0.2, td);
        arm_l = max(arm_l, hold);
        ask_mask(&[(sh_l, hold * gate, true), (sh_l, 0.0, false)]);
        if !ask_blend(g, at, at_b, 1.0, true, at, n) {
            return false;
        }
        ts_out = ts + dt;
        if ts_out > td {
            ts_out = -1.0;
        }
        code |= RAN_ALL;
    }
    *o = Arms { arm_l, ts_out, code, ..*o };
    true
}
/// A man knocked about and still up: the support hand off the gun and pressed to his ribs. Solved on the scratch pose at `at_b`, laid on the pose by w
/// through the support arm's mask.
pub fn clutch_arm(g: usize, an: &mut Anim, w: f64) -> u32 {
    let (at, at_b, _) = poses(an);
    let fix_l = an.hand_fix[0];
    let Some(Some(n)) = with_pose(g, at, |_d, _rig, n, _fk| if cast(n, &ARM_ROLES) && cast(n, &[HIPS, SPINE1]) { Some(n) } else { None }) else {
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
        let (left, up, fwd) = trunk_frame(fk);
        let p = mad3(mad3(mad3(fk.at(role(SPINE1)), left, MV2.clutch.at[0]), up, MV2.clutch.at[1]), fwd, MV2.clutch.at[2]);
        let r = hand_rot(&fix_l, mad3(scale3(left, -1.0), up, -0.25), mad3(fwd, up, 0.2));
        two_bone_ik(rig, d, fk.rot, fk.pos, role(UPPER_ARM_L), role(FOREARM_L), role(HAND_L), &p, Some(&MV2.clutch.pole), 0.7, Some(&r), false);
        hand(d, hh, 0, &[0.3, 0.34, 0.38, 0.42], 0.3, 0.2);
    });
    arm_mask(false);
    if !ask_blend(g, at, at_b, w, true, at, n) {
        return 0;
    }
    hands_to(&h, an);
    RAN | RAN_ALL
}
/// Kept down: the wrists crossed in the small of the back, palms out (kind 1), or the fingers laced on the crown with the elbows wide, in the head's own
/// frame (kind 2). Both arms, solved on the scratch pose at `at_b` and laid on the pose by w.
pub fn bound_arms(g: usize, an: &mut Anim, w: f64, kind: f64) -> u32 {
    let (at, at_b, _) = poses(an);
    let (back, gaze, fix_l, fix_r) = (kind == 1.0, an.gaze_local, an.hand_fix[0], an.hand_fix[1]);
    let Some(Some(n)) = with_pose(g, at, |_d, _rig, n, _fk| if cast(n, &ARM_ROLES) && cast(n, &[HIPS, HEAD]) { Some(n) } else { None }) else {
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
            hand(d, hh, 0, &curl, MV2.bound.back.thumb, 0.0);
            hand(d, hh, 1, &curl, MV2.bound.back.thumb, 0.0);
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
            hand(d, hh, 0, &curl, MV2.bound.head.thumb, 0.25);
            hand(d, hh, 1, &curl, MV2.bound.head.thumb, 0.25);
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
    if !ask_blend(g, at, at_b, w, true, at, n) {
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
