//! Breath, weight and balance, and the feet last of all.
use super::*;

/// What the steps of a man's own life hand on to the ones after them.
#[derive(Clone, Copy, Default)]
struct Life {
    dt: f64,
    s: f64,
    crouch_pose: f64,
    w: f64,
    w_f: f64,
    p0: [f64; 3],
    // as_stands: which way he faces and how low he is
    yin: f64,
    cr_t: f64,
    // own_motion: the root's own motion over the ground: more than half a metre in a frame is a teleport, and the feet lock again where they land
    teleported: bool,
    // breathe: the breath, harder after a run
    b_amp: f64,
    // feet_of_pose: the feet as the pose has them
    fl: [f64; 3],
    fr: [f64; 3],
    rl: [f64; 4],
    rr: [f64; 4],
    // feet_stand: planted in the world while he stands, released into the cycle's feet as he moves off
    w_was: f64,
    // spots_wanted: where the pose wants each foot, in the world
    stands: Stands,
    want_p: [[f64; 3]; 2],
    want_yaw: [f64; 2],
    swing: f64,
    lift: f64,
    dist: f64,
    turn_at: f64,
    pivot: f64,
    lag_cap: f64,
    // lock_offs: both feet locked, and by how much: feet_last reads the stack's feet without this
    tl: [f64; 3],
    ql: [f64; 4],
    tr: [f64; 3],
    qr: [f64; 4],
}

/// Breathing, the body's mass on starts and stops, the weight shifting foot to foot at rest, and the standing foot lock: the feet planted in the world
/// while he stands, each stepping across in a lifted arc when the body has turned or shifted out from over it.
pub fn life_layer(g: usize, an: &mut Anim, dt: f64, s: f64, crouch_pose: f64, w: f64, w_f: f64, p0: [f64; 3], tune: &Knobs) -> u32 {
    let mut o = Life { dt, s, crouch_pose, w, w_f, p0, ..Life::default() };
    as_stands(an, &mut o);
    let (at, lf) = (an.p_a.at as usize, &mut an.life);
    let (f_w, f_next, mut foot) = (&mut an.feet.w, &mut an.feet.next, [an.feet.l, an.feet.r]);
    for f in foot.iter_mut() { f.lock_off = [0.0; 3]; f.lock_yaw = 0.0; }
    let code = with_pose(g, at, |d, rig, n, fk| {
        if !cast(n, &[HIPS, SPINE1, SPINE2, SPINE3, NECK, SHOULDER_L, SHOULDER_R, THIGH_L, SHIN_L, FOOT_L, THIGH_R, SHIN_R, FOOT_R]) {
            return 0;
        }
        // clocks
        turn_rate(lf, &o);
        own_motion(lf, &mut o);
        breathe(lf, tune, &mut o);
        weight_swings(lf, &o);
        weight_shifts(lf, &o);
        if w_f < 0.003 {
            *f_w = 0.0;
        }
        if w < 0.003 && w_f < 0.003 {
            return RAN;
        }
        // remember the feet, pose the body, put the feet back
        let (foot_l, foot_r) = (role(FOOT_L), role(FOOT_R));
        let legs = list(FK_LEGS);
        fk.part(rig, d, legs);
        let ran = RAN | ran_list(FK_LEGS);
        feet_of_pose(foot_l, foot_r, fk, &mut o);
        if w >= 0.003 {
            body_lives(lf, d, rig, &o);
            fk.part(rig, d, legs);
        }
        feet_stand(lf, f_w, &mut o);
        if w < 0.003 && *f_w <= 0.01 {
            return ran;
        }
        as_posed(&mut o);
        if *f_w > 0.01 {
            spots_wanted(&mut o);
            lock_again(&mut foot, &o);
            for i in 0..2 {
                foot_holds(f_next, &mut foot, i, &o);
                foot_swings(&mut foot, i, &o);
            }
            lock_offs(f_w, &mut foot, &mut o);
        }
        // put the feet back: the knee steered over the toes only as far as the feet are LOCKED
        let pole_w = if o.cr_t > 0.0 { 0.6 * *f_w * (1.0 - smoothstep(0.0, 1.0, o.cr_t)) } else { 0.6 * *f_w };
        two_bone_ik(rig, d, fk.rot, fk.pos, role(THIGH_L), role(SHIN_L), foot_l, &o.tl, Some(&knee_pole(&o.ql)), pole_w, Some(&o.ql), false);
        two_bone_ik(rig, d, fk.rot, fk.pos, role(THIGH_R), role(SHIN_R), foot_r, &o.tr, Some(&knee_pole(&o.qr)), pole_w, Some(&o.qr), true);
        ran | RAN_ALL
    })
    .unwrap_or(0);
    (an.feet.l, an.feet.r) = (foot[0], foot[1]);
    code
}

/// Which way he faces and how low he is.
fn as_stands(an: &Anim, o: &mut Life) {
    let (yin, cr_t) = (an.body_yaw_in, an.cr_t);
    *o = Life { yin, cr_t, ..*o };
}

/// How fast he turns.
fn turn_rate(lf: &mut game_types::character::AnimLife, o: &Life) {
    let Life { dt, yin, .. } = *o;
    if !finite(lf.yaw_prev) {
        lf.yaw_prev = yin;
    }
    let yr = if dt > 0.0 { wrap_angle(yin - lf.yaw_prev) / dt } else { 0.0 };
    lf.yaw_prev = yin;
    lf.yaw_rate = damp(lf.yaw_rate, clamp(yr, -8.0, 8.0), 8.0, dt);
}

/// The root's own motion over the ground: more than half a metre in a frame is a teleport, and the feet lock again where they land.
fn own_motion(lf: &mut game_types::character::AnimLife, o: &mut Life) {
    let Life { dt, p0, .. } = *o;
    let mut teleported = false;
    if !finite(lf.prev_pos[0]) {
        lf.prev_pos = p0;
    } else {
        let dd = hypot2(p0[0] - lf.prev_pos[0], p0[2] - lf.prev_pos[2]);
        teleported = dd > 0.5;
        lf.root_v = if dt > 0.0 && !teleported {
            damp(lf.root_v, min(dd / dt, 8.0), 14.0, dt)
        } else if teleported {
            0.0
        } else {
            lf.root_v
        };
        lf.prev_pos = p0;
    }
    *o = Life { teleported, ..*o };
}

/// The breath, harder after a run.
fn breathe(lf: &mut game_types::character::AnimLife, tune: &Knobs, o: &mut Life) {
    let Life { dt, s, .. } = *o;
    lf.exert = clamp(lf.exert + if s > 3.2 { dt / tune.hardbreath.chest_up } else { -dt / tune.hardbreath.chest_down }, 0.0, 1.0);
    let b_rate = lerp(LIFE.breath_rate, tune.hardbreath.chest_rate, lf.exert);
    let b_amp = lerp(LIFE.breath_amp, tune.hardbreath.chest_amp, lf.exert) * lerp(1.0, 0.35, clamp(s / 2.0, 0.0, 1.0)) * LIFE.on;
    lf.breath = (lf.breath + dt * b_rate * 2.0 * PI) % (2.0 * PI);
    *o = Life { b_amp, ..*o };
}

/// His weight swings as he starts and stops.
fn weight_swings(lf: &mut game_types::character::AnimLife, o: &Life) {
    let Life { dt, s, .. } = *o;
    let accel = if dt > 0.0 { (s - lf.prev_s) / dt } else { 0.0 };
    lf.prev_s = s;
    lf.accel = damp(lf.accel, clamp(accel, -12.0, 12.0), 10.0, dt);
    let t = clamp(-lf.accel * LIFE.mass_gain, -LIFE.mass_back, LIFE.mass_fwd) * LIFE.on;
    spring_step(&mut lf.mass, t, 9.0, dt);
    let t = clamp(-abs(lf.accel) * LIFE.mass_dip, -LIFE.mass_dip_max, 0.0) * LIFE.on;
    spring_step(&mut lf.mass_v, t, 10.0, dt);
}

/// Standing long enough, he shifts his weight from one leg to the other.
fn weight_shifts(lf: &mut game_types::character::AnimLife, o: &Life) {
    let Life { dt, s, cr_t, .. } = *o;
    lf.still_t = if s < 0.12 && abs(lf.yaw_rate) < 0.5 { lf.still_t + dt } else { 0.0 };
    let resting = if lf.still_t > LIFE.rest_after && LIFE.on > 0.0 && !(cr_t > 0.5) { 1.0 } else { 0.0 };
    if resting != 0.0 {
        lf.shift_t -= dt;
        if lf.shift_t <= 0.0 {
            lf.shift_side = if lf.shift_side > 0.0 { -1.0 } else { 1.0 };
            lf.shift_n += 1.0;
            lf.shift_t = LIFE.shift_min + ((lf.shift_n * 0.618034) % 1.0) * LIFE.shift_span;
        }
    }
    let t = resting * lf.shift_side;
    spring_step(&mut lf.shift, t, if resting != 0.0 { LIFE.shift_in } else { LIFE.shift_out }, dt);
}

/// The feet as the pose has them.
fn feet_of_pose(foot_l: usize, foot_r: usize, fk: &mut Fk, o: &mut Life) {
    let (fl, fr, rl, rr) = (fk.at(foot_l), fk.at(foot_r), fk.rot(foot_l), fk.rot(foot_r));
    *o = Life { fl, fr, rl, rr, ..*o };
}

/// The body posed: the weight, the hip dropped on the resting side, the breath in the chest.
fn body_lives(lf: &mut game_types::character::AnimLife, d: &mut [S], rig: &Rig, o: &Life) {
    let Life { crouch_pose, w, b_amp, .. } = *o;
    let (hips, s1, s2, s3, nk) = (role(HIPS), role(SPINE1), role(SPINE2), role(SPINE3), role(NECK));
    let (br, br2) = (sin(lf.breath), sin(lf.breath - 0.6));
    let sh = lf.shift.x * w * min(1.0, LIFE.on);
    let side = LIFE.shift_travel * sh * (1.0 - 0.5 * crouch_pose);
    let off = [side, lf.mass_v.x * w + 0.004 * b_amp * br * w - abs(sh) * 0.006, lf.mass.x * w * (1.0 - 0.4 * crouch_pose)];
    shift(d, rig, hips, &off);
    if abs(sh) > 1e-3 {
        turn(d, rig, hips, q_roll(-sh * LIFE.hip_drop * DEGR));
        turn(d, rig, s2, q_roll(sh * LIFE.spine_counter * DEGR));
        turn(d, rig, nk, q_roll(sh * LIFE.neck_counter * DEGR));
    }
    let tip = clamp(lf.accel * LIFE.tip_gain, -LIFE.tip_back, LIFE.tip_fwd) * DEGR * w * (1.0 - 0.5 * crouch_pose) * LIFE.on;
    if abs(tip) > 1e-4 {
        turn(d, rig, s1, q_pitch(tip * 0.5));
        turn(d, rig, s2, q_pitch(tip * 0.3));
        turn(d, rig, nk, q_pitch(-tip * 0.5));
    }
    let bp = b_amp * w * DEGR;
    turn(d, rig, s2, q_pitch(-0.5 * bp * br));
    turn(d, rig, s3, q_pitch(-0.4 * bp * br2));
    turn(d, rig, nk, q_pitch(0.5 * bp * br2));
    turn(d, rig, role(SHOULDER_L), q_roll(1.0 * 0.9 * bp * br2));
    turn(d, rig, role(SHOULDER_R), q_roll(-1.0 * 0.9 * bp * br2));
}

/// Planted in the world while he stands, released into the cycle's feet as he moves off.
fn feet_stand(lf: &mut game_types::character::AnimLife, f_w: &mut f64, o: &mut Life) {
    let Life { dt, s, w_f, cr_t, .. } = *o;
    let standing = if max(s, lf.root_v) < 0.22 && w_f > 0.5 && FEET.on > 0.5 && !(cr_t > 0.0 && cr_t < 1.0) { 1.0 } else { 0.0 };
    let w_was = *f_w;
    *f_w = damp(*f_w, standing, if standing != 0.0 { 12.0 } else { 16.0 + 40.0 * lf.root_v }, dt);
    *o = Life { w_was, ..*o };
}

/// The feet put back where they were.
fn as_posed(o: &mut Life) {
    let Life { fl, fr, rl, rr, .. } = *o;
    let (tl, tr, ql, qr) = (fl, fr, rl, rr);
    *o = Life { tl, ql, tr, qr, ..*o };
}

/// Where the pose wants each foot, in the world.
fn spots_wanted(o: &mut Life) {
    let Life { p0, yin, fl, fr, rl, rr, .. } = *o;
    let stands = Stands::at(p0, yin);
    let want_p = [stands.world(&fl), stands.world(&fr)];
    let want_yaw = [yin + foot_yaw(&rl), yin + foot_yaw(&rr)];
    let (swing, lift, dist, turn_at, pivot, lag_cap) = (FEET.swing, FEET.lift, FEET.dist, FEET.turn_deg * DEGR, FEET.pivot, FEET.lag_cap_deg * DEGR);
    *o = Life { stands, want_p, want_yaw, swing, lift, dist, turn_at, pivot, lag_cap, ..*o };
}

/// Newly planted, set down by a teleport or far off, they lock where the pose has them.
fn lock_again(foot: &mut [AnimFoot; 2], o: &Life) {
    let Life { teleported, w_was, want_p, want_yaw, .. } = *o;
    let relock = w_was <= 0.01 || teleported || v3::dist_xz(foot[0].p, want_p[0]) > 0.45 || v3::dist_xz(foot[1].p, want_p[1]) > 0.45;
    if relock {
        for side in 0..2 {
            foot[side].p = want_p[side];
            foot[side].yaw = want_yaw[side];
            foot[side].swing = -1.0;
        }
    }
}

/// Whether a foot is far from where the pose wants it.
fn astray(o: &Life, foot: &[AnimFoot; 2], i: usize) -> bool {
    let Life { want_p, want_yaw, dist, turn_at, .. } = *o;
    v3::dist_xz(foot[i].p, want_p[i]) > dist || abs(wrap_angle(foot[i].yaw - want_yaw[i])) > turn_at
}

/// A planted foot holds its place, turns within what the ankle takes, and starts a step when the pose is far from it and it is its turn.
fn foot_holds(f_next: &mut f64, foot: &mut [AnimFoot; 2], i: usize, o: &Life) {
    let Life { dt, want_yaw, turn_at, pivot, lag_cap, .. } = *o;
    if foot[i].swing < 0.0 {
        let mut err_yaw = wrap_angle(want_yaw[i] - foot[i].yaw);
        if abs(err_yaw) > lag_cap {
            foot[i].yaw += sign(err_yaw) * (abs(err_yaw) - lag_cap);
            err_yaw = sign(err_yaw) * lag_cap;
        }
        if abs(err_yaw) > turn_at {
            foot[i].yaw += sign(err_yaw) * min(abs(err_yaw) - turn_at, pivot * dt);
        }
        let my_turn = *f_next == i as f64;
        if astray(o, foot, i) && foot[1 - i].swing < 0.0 && (my_turn || !astray(o, foot, 1 - i)) {
            foot[i].swing = 0.0;
            foot[i].from = foot[i].p;
            foot[i].from_yaw = foot[i].yaw;
            *f_next = (1 - i) as f64;
        }
    }
}

/// A step: through the air and down where the pose wants it.
fn foot_swings(foot: &mut [AnimFoot; 2], i: usize, o: &Life) {
    let Life { dt, want_p, want_yaw, swing, lift, .. } = *o;
    if foot[i].swing >= 0.0 {
        foot[i].swing += dt / swing;
        let e = smoothstep(0.0, 1.0, min(1.0, foot[i].swing));
        let from = foot[i].from;
        let mut p = [lerp(from[0], want_p[i][0], e), lerp(from[1], want_p[i][1], e), lerp(from[2], want_p[i][2], e)];
        p[1] = want_p[i][1] + sin(PI * min(1.0, foot[i].swing)) * lift;
        foot[i].p = p;
        foot[i].yaw = foot[i].from_yaw + wrap_angle(want_yaw[i] - foot[i].from_yaw) * e;
        if foot[i].swing >= 1.0 {
            foot[i].swing = -1.0;
            foot[i].p = want_p[i];
            foot[i].yaw = want_yaw[i];
        }
    }
}

/// A foot in his own axes, as far as it is locked.
fn locked(f_w: &mut f64, o: &Life, f: &AnimFoot, rest_p: &[f64; 3], rest_r: &[f64; 4]) -> ([f64; 3], [f64; 4]) {
    let Life { yin, stands, .. } = *o;
    let mp = stands.model(&f.p);
    let fw = *f_w;
    let p = [lerp(rest_p[0], mp[0], fw), lerp(rest_p[1], mp[1], fw), lerp(rest_p[2], mp[2], fw)];
    let dyaw = wrap_angle((f.yaw - yin) - foot_yaw(rest_r)) * fw;
    (p, quat::mul(q_yaw(dyaw), *rest_r))
}

/// Both feet locked, and by how much: feet_last reads the stack's feet without this.
fn lock_offs(f_w: &mut f64, foot: &mut [AnimFoot; 2], o: &mut Life) {
    let Life { yin, fl, fr, rl, rr, .. } = *o;
    let (tl, ql) = locked(f_w, o, &foot[0], &fl, &rl);
    let (tr, qr) = locked(f_w, o, &foot[1], &fr, &rr);
    foot[0].lock_off = v3::sub(tl, fl);
    foot[1].lock_off = v3::sub(tr, fr);
    foot[0].lock_yaw = wrap_angle((foot[0].yaw - yin) - foot_yaw(&rl)) * *f_w;
    foot[1].lock_yaw = wrap_angle((foot[1].yaw - yin) - foot_yaw(&rr)) * *f_w;
    *o = Life { tl, ql, tr, qr, ..*o };
}
/// What the steps of the feet hand on to the ones after them. Pairs are the left and the right.
#[derive(Clone, Copy, Default)]
struct Feet {
    dt: f64,
    s: f64,
    w_f: f64,
    own: f64,
    p0: [f64; 3],
    // as_on_feet: how he stands, and his own motion over the ground: more than half a metre in a frame is a teleport, and the feet are set down where they land
    at: usize,
    yin: f64,
    cr_t: f64,
    f_w: f64,
    lock_off: [[f64; 3]; 2],
    lock_yaw: [f64; 2],
    p_was: [f64; 3],
    teleported: bool,
    went: [f64; 2],
    // legs_free: whether a layer has the legs
    active: bool,
    k: f64,
    // feet_as_posed: both feet as posed, and which way he faces
    fl: [f64; 3],
    toe_pose_l: [f64; 3],
    rl: [f64; 4],
    fr: [f64; 3],
    toe_pose_r: [f64; 3],
    rr: [f64; 4],
    stands: Stands,
    // feet_wanted: where the pose wants each foot, in the world; and how far it may want one from where it stands before it steps: a stand's small shuffle, a stride's reach
    ankle: [[f64; 3]; 2],
    toe: [[f64; 3]; 2],
    rot: [[f64; 4]; 2],
    want_p: [[f64; 3]; 2],
    want_toe: [[f64; 3]; 2],
    want_yaw: [f64; 2],
    swing: f64,
    lift: f64,
    turn_at: f64,
    pivot: f64,
    lag_cap: f64,
    reach_now: f64,
    // cycle_under: the cycle under the foot: at the floor or lifted, going forward over the ground or not
    toe_down: bool,
    at_floor: bool,
    lifted: bool,
    going: bool,
    // ease_off: over the ground: down it holds; let go, what it is off by eases out, critically damped and at no more than a foot's speed. Up and down is always the pose's, eased
    off: [f64; 3],
    off_v: [f64; 3],
    // shown_at_first: nothing off the pose yet
    shown: [[f64; 3]; 2],
    // body_turn: which way he faces, and the feet as posed
    body: [f64; 4],
    body_back: [f64; 4],
    placed: [([f64; 3], [f64; 4]); 2],
}

/// THE RULE OF A FOOT, last of the stack: a foot that is down stays where it is; it goes somewhere else only through the air, and no faster than a foot goes.
/// It reads the feet the finished pose has, without the standing lock (life_layer's, which every layer above was posed over and goes on being),
/// and writes the thigh, the shin and the foot of each leg and nothing above the hips. `w_f`: the legs are his to stand on; `own`: how much of what the stack has in them is the stack's and not a whole-body clip's.
pub fn feet_last(g: usize, an: &mut Anim, dt: f64, s: f64, w_f: f64, own: f64, p0: [f64; 3]) -> u32 {
    let mut o = Feet { dt, s, w_f, own, p0, ..Feet::default() };
    let mut foot = [an.feet.step_l, an.feet.step_r];
    as_on_feet(an, &mut o);
    (an.feet.at_known, an.feet.at_was) = (true, p0);
    let code = with_pose(g, o.at, |d, rig, n, fk| {
        if !cast(n, &[HIPS, THIGH_L, SHIN_L, FOOT_L, TOE_L, THIGH_R, SHIN_R, FOOT_R, TOE_R]) {
            return 0;
        }
        if w_f < 0.003 {
            for f in foot.iter_mut() { f.down = false; f.swing = -1.0; f.off = [0.0; 3]; f.off_v = [0.0; 3]; f.off_yaw = 0.0; f.clear = 0.0; f.fresh = true; }
            return RAN;
        }
        legs_free(&mut o);
        if !o.active && foot.iter().all(|f| f.swing < 0.0 && f.off == [0.0; 3] && f.off_yaw == 0.0 && f.clear == 0.0) {
            return RAN;
        }
        let (foot_l, foot_r) = (role(FOOT_L), role(FOOT_R));
        let legs = list(FK_LEGS);
        fk.part(rig, d, legs);
        let ran = RAN | ran_list(FK_LEGS);
        feet_as_posed(fk, foot_l, foot_r, &mut o);
        feet_wanted(&mut o);
        for i in 0..2 {
            cycle_under(&mut foot, i, &mut o);
            set_down(&mut foot, i, &o);
            ease_off(&mut foot, i, &mut o);
            hold_place(&mut foot, i, &o);
        }
        start_steps(&mut foot, &o);
        shown_at_first(&mut o);
        for i in 0..2 {
            swing_through(&mut foot, i, &mut o);
            o.shown[i][1] += foot[i].clear;
            foot_speed(&mut foot, i, &mut o);
            foot[i].shown = o.shown[i];
            foot[i].fresh = false;
        }
        // and it turns no faster than a foot turns, seen in the world: a pose that throws a foot over all at once is met over a few frames
        body_turn(&mut o);
        for i in 0..2 {
            foot_turn(&mut foot, i, &mut o);
        }
        let ((tl, ql), (tr, qr)) = (o.placed[0], o.placed[1]);
        // the knee over the toes only as far as he stands
        let pole_w = if o.cr_t > 0.0 { 0.6 * o.f_w * (1.0 - smoothstep(0.0, 1.0, o.cr_t)) } else { 0.6 * o.f_w };
        two_bone_ik(rig, d, fk.rot, fk.pos, role(THIGH_L), role(SHIN_L), foot_l, &tl, Some(&knee_pole(&ql)), pole_w, Some(&ql), false);
        two_bone_ik(rig, d, fk.rot, fk.pos, role(THIGH_R), role(SHIN_R), foot_r, &tr, Some(&knee_pole(&qr)), pole_w, Some(&qr), true);
        ran | RAN_ALL
    })
    .unwrap_or(0);
    (an.feet.step_l, an.feet.step_r) = (foot[0], foot[1]);
    code
}

/// How he stands, and his own motion over the ground: more than half a metre in a frame is a teleport, and the feet are set down where they land.
fn as_on_feet(an: &mut Anim, o: &mut Feet) {
    let Feet { p0, .. } = *o;
    let (at, yin, cr_t, f_w) = (an.p_a.at as usize, an.body_yaw_in, an.cr_t, an.feet.w);
    let (lock_off, lock_yaw) = ([an.feet.l.lock_off, an.feet.r.lock_off], [an.feet.l.lock_yaw, an.feet.r.lock_yaw]);
    let (was_known, was_at) = (an.feet.at_known, an.feet.at_was);
    // his own motion over the ground: more than half a metre in a frame is a teleport, and the feet are set down where they land
    let p_was = if was_known { was_at } else { p0 };
    let dd = hypot2(p0[0] - p_was[0], p0[2] - p_was[2]);
    let teleported = dd > 0.5;
    let went = if dd > 1e-6 { [(p0[0] - p_was[0]) / dd, (p0[2] - p_was[2]) / dd] } else { [0.0, 0.0] };
    *o = Feet { at, yin, cr_t, f_w, lock_off, lock_yaw, p_was, teleported, went, ..*o };
}

/// Whether a layer has the legs.
fn legs_free(o: &mut Feet) {
    let Feet { w_f, own, .. } = *o;
    let active = w_f > 0.5 && FEET.on > 0.5;
    let k = if active { own } else { 0.0 };
    *o = Feet { active, k, ..*o };
}

/// A foot the stack has, the standing lock taken back out of it.
fn without(o: &Feet, i: usize, a: [f64; 3], t: [f64; 3], r: [f64; 4]) -> ([f64; 3], [f64; 3], [f64; 4]) {
    let Feet { lock_off, lock_yaw, k, .. } = *o;
    let un = q_yaw(-lock_yaw[i] * k);
    let o = lock_off[i];
    let at = [a[0] - o[0] * k, a[1] - o[1] * k, a[2] - o[2] * k];
    (at, v3::add(at, quat::rotate(un, v3::sub(t, a))), quat::mul(un, r))
}

/// Both feet as posed, and which way he faces.
fn feet_as_posed(fk: &Fk, foot_l: usize, foot_r: usize, o: &mut Feet) {
    let Feet { p0, yin, .. } = *o;
    let (fl, toe_pose_l, rl) = without(o, 0, fk.at(foot_l), fk.at(role(TOE_L)), fk.rot(foot_l));
    let (fr, toe_pose_r, rr) = without(o, 1, fk.at(foot_r), fk.at(role(TOE_R)), fk.rot(foot_r));
    let stands = Stands::at(p0, yin);
    *o = Feet { fl, toe_pose_l, rl, fr, toe_pose_r, rr, stands, ..*o };
}

/// Where the pose wants each foot, in the world; and how far it may want one from where it stands before it steps: a stand's small shuffle, a stride's reach.
fn feet_wanted(o: &mut Feet) {
    let Feet { s, yin, fl, toe_pose_l, rl, fr, toe_pose_r, rr, .. } = *o;
    let (ankle, toe, rot) = ([fl, fr], [toe_pose_l, toe_pose_r], [rl, rr]);
    let want_p = [o.stands.world(&fl), o.stands.world(&fr)];
    let want_toe = [o.stands.world(&toe_pose_l), o.stands.world(&toe_pose_r)];
    let want_yaw = [yin + foot_yaw(&rl), yin + foot_yaw(&rr)];
    let (swing, lift, turn_at, pivot, lag_cap) = (FEET.swing, FEET.lift, FEET.turn_deg * DEGR, FEET.pivot, FEET.lag_cap_deg * DEGR);
    // how far the pose may want a foot from where it stands before it steps: a stand's small shuffle, a stride's reach
    let reach_now = lerp(FEET.dist, FEET.reach, min(s, 1.0));
    *o = Feet { ankle, toe, rot, want_p, want_toe, want_yaw, swing, lift, turn_at, pivot, lag_cap, reach_now, ..*o };
}

/// How far past that a foot is.
fn far_of(o: &Feet, f: &AnimStep, i: usize) -> f64 {
    let Feet { want_yaw, turn_at, reach_now, .. } = *o;
    let apart = hypot2(f.off[0], f.off[2]);
    if apart > reach_now || abs(wrap_angle(f.yaw - want_yaw[i])) > turn_at { apart + 1e-3 } else { 0.0 }
}

/// From the ankle to the toe over the ground, turned as far as the foot is off the pose's way.
fn lever(o: &Feet, i: usize, off_yaw: f64) -> [f64; 3] {
    let Feet { want_p, want_toe, .. } = *o;
    let (dx, dz) = (want_toe[i][0] - want_p[i][0], want_toe[i][2] - want_p[i][2]);
    let (c, s_) = (cos(off_yaw), sin(off_yaw));
    [dx * c + dz * s_, 0.0, dz * c - dx * s_]
}

/// The cycle under the foot: at the floor or lifted, going forward over the ground or not.
fn cycle_under(foot: &mut [AnimStep; 2], i: usize, o: &mut Feet) {
    let Feet { dt, s, teleported, went, ankle, toe, want_toe, .. } = *o;
    let (ay_, ty_) = (ankle[i][1], toe[i][1]);
    let toe_down = ty_ < TOE_FLOOR_Y + FEET.floor_off;
    let at_floor = toe_down || ay_ < FEET.ankle_y + FEET.floor_off;
    let lifted = !at_floor && ty_ > TOE_FLOOR_Y + FEET.lifted;
    let v = if dt > 1e-5 { ((want_toe[i][0] - foot[i].toe_was[0]) * went[0] + (want_toe[i][2] - foot[i].toe_was[2]) * went[1]) / dt } else { 0.0 };
    if dt > 1e-5 { foot[i].toe_was = want_toe[i]; }
    let going = s > FEET.going_from && v > FEET.going_k * s;
    // a foot going forward clears the floor: what the cycle does not lift it by (a crouch's stride hardly does), it is lifted by
    foot[i].clear = damp(foot[i].clear, if going { max(0.0, FEET.clear - (ty_ - TOE_FLOOR_Y)) } else { 0.0 }, FEET.clear_k, dt);
    if teleported {
        foot[i].down = false;
        foot[i].swing = -1.0;
        foot[i].off = [0.0; 3];
        foot[i].off_v = [0.0; 3];
        foot[i].off_yaw = 0.0;
        foot[i].fresh = true;
    }
    *o = Feet { toe_down, at_floor, lifted, going, ..*o };
}

/// Where its pivot stood in the last frame's picture: what it holds from now on.
fn pivot_at(i: usize, o: &Feet, f: &AnimStep, on_toe: bool) -> [f64; 3] {
    let Feet { want_p, want_toe, .. } = *o;
    if f.fresh { return if on_toe { want_toe[i] } else { want_p[i] }; }
    if on_toe { v3::add(f.shown, lever(o, i, f.off_yaw)) } else { f.shown }
}

/// The foot set down, let go of by the cycle, or eased into the pose where a layer has the legs.
fn set_down(foot: &mut [AnimStep; 2], i: usize, o: &Feet) {
    let Feet { dt, active, want_yaw, toe_down, at_floor, lifted, going, .. } = *o;
    if dt > 1e-5 {
        if !active {
            // a layer has the legs: whatever the foot was doing, it eases into the pose
            if foot[i].swing >= 0.0 || foot[i].down {
                foot[i].swing = -1.0;
                foot[i].down = false;
                foot[i].off_v = [0.0; 3];
            }
        } else if foot[i].swing < 0.0 {
            if foot[i].down && (lifted || going) {
                // the cycle has it: it leaves with what it is off by, and that eases out
                foot[i].down = false;
                foot[i].off_v = [0.0; 3];
            } else if !foot[i].down && at_floor && !going {
                foot[i].down = true;
                foot[i].toe_down = toe_down;
                foot[i].p = pivot_at(i, o, &foot[i], toe_down);
                foot[i].yaw = want_yaw[i] + foot[i].off_yaw;
            }
        }
    }
}

/// Over the ground: down it holds; let go, what it is off by eases out, critically damped and at no more than a foot's speed. Up and down is always the pose's, eased.
fn ease_off(foot: &mut [AnimStep; 2], i: usize, o: &mut Feet) {
    let Feet { dt, .. } = *o;
    let (mut off, mut off_v) = (foot[i].off, foot[i].off_v);
    let holds = foot[i].swing < 0.0 && foot[i].down;
    for c in 0..3 {
        if holds && c != 1 { continue; }
        let mut sp = Spring { x: off[c], v: off_v[c] };
        spring_step(&mut sp, 0.0, FEET.ease, dt);
        off[c] = sp.x;
        off_v[c] = sp.v;
    }
    foot[i].off = off;
    foot[i].off_v = off_v;
    *o = Feet { off, off_v, ..*o };
}

/// Down it holds its place, on the toe, or on the heel while the toe is still coming down, its way within what the ankle takes.
fn hold_place(foot: &mut [AnimStep; 2], i: usize, o: &Feet) {
    let Feet { dt, want_p, want_yaw, turn_at, pivot, lag_cap, toe_down, off, off_v, .. } = *o;
    if foot[i].swing < 0.0 && foot[i].down {
        // it holds its place, on the toe, or on the heel while the toe is still coming down; its way within what the ankle takes
        if foot[i].toe_down != toe_down {
            foot[i].p = pivot_at(i, o, &foot[i], toe_down);
            foot[i].toe_down = toe_down;
        }
        let mut err_yaw = wrap_angle(want_yaw[i] - foot[i].yaw);
        if abs(err_yaw) > lag_cap {
            foot[i].yaw += sign(err_yaw) * (abs(err_yaw) - lag_cap);
            err_yaw = sign(err_yaw) * lag_cap;
        }
        if abs(err_yaw) > turn_at {
            foot[i].yaw += sign(err_yaw) * min(abs(err_yaw) - turn_at, pivot * dt);
        }
        foot[i].off_yaw = wrap_angle(foot[i].yaw - want_yaw[i]);
        let under = if toe_down { v3::add(want_p[i], lever(o, i, foot[i].off_yaw)) } else { want_p[i] };
        foot[i].off[0] = foot[i].p[0] - under[0];
        foot[i].off[2] = foot[i].p[2] - under[2];
        foot[i].off_v[0] = 0.0;
        foot[i].off_v[2] = 0.0;
    } else if foot[i].swing < 0.0 {
        foot[i].off_yaw = damp(foot[i].off_yaw, 0.0, FEET.ease * 0.5, dt);
        if abs(foot[i].off_yaw) < 1e-4 { foot[i].off_yaw = 0.0; }
        if off.iter().all(|x| abs(*x) < 1e-4) && off_v.iter().all(|x| abs(*x) < 1e-3) { foot[i].off = [0.0; 3]; foot[i].off_v = [0.0; 3]; }
        foot[i].yaw = want_yaw[i] + foot[i].off_yaw;
    }
}

/// A foot down that the pose wants far from it steps, the one further off first; the other may start as this one comes to its last third.
fn start_steps(foot: &mut [AnimStep; 2], o: &Feet) {
    let Feet { dt, active, want_p, .. } = *o;
    if active && dt > 1e-5 {
        for _ in 0..2 {
            let far = [if foot[0].swing < 0.0 && foot[0].down { far_of(o, &foot[0], 0) } else { 0.0 }, if foot[1].swing < 0.0 && foot[1].down { far_of(o, &foot[1], 1) } else { 0.0 }];
            let one = if far[0] >= far[1] { 0 } else { 1 };
            if far[one] <= 0.0 || (foot[1 - one].swing >= 0.0 && foot[1 - one].swing < 2.0 / 3.0) {
                break;
            }
            let f = &mut foot[one];
            f.from = v3::add(want_p[one], f.off);
            f.swing = 0.0;
            f.from_yaw = f.yaw;
            f.down = false;
        }
    }
}

/// Nothing off the pose yet.
fn shown_at_first(o: &mut Feet) {
    let Feet { want_p, .. } = *o;
    let shown = [want_p[0], want_p[1]];
    *o = Feet { shown, ..*o };
}

/// A foot in the air, or off the pose by what it is off by.
fn swing_through(foot: &mut [AnimStep; 2], i: usize, o: &mut Feet) {
    let Feet { dt, want_p, want_yaw, swing, lift, mut shown, .. } = *o;
    if foot[i].swing >= 0.0 {
        // through the air: the further off past what a step takes, the quicker the step, so a fast turn is quick steps and the foot gets there
        foot[i].swing += dt / swing * max(1.0, v3::dist_xz(foot[i].from, want_p[i]) / FEET.reach);
        let e = smoothstep(0.0, 1.0, min(1.0, foot[i].swing));
        let from = foot[i].from;
        shown[i] = [lerp(from[0], want_p[i][0], e), want_p[i][1] + sin(PI * min(1.0, foot[i].swing)) * lift, lerp(from[2], want_p[i][2], e)];
        foot[i].off = [shown[i][0] - want_p[i][0], 0.0, shown[i][2] - want_p[i][2]];
        foot[i].off_v = [0.0; 3];
        foot[i].yaw = foot[i].from_yaw + wrap_angle(want_yaw[i] - foot[i].from_yaw) * e;
        foot[i].off_yaw = wrap_angle(foot[i].yaw - want_yaw[i]);
        if foot[i].swing >= 1.0 {
            // set down where it has come to
            foot[i].swing = -1.0;
            foot[i].off = [0.0; 3];
            foot[i].off_v = [0.0; 3];
            foot[i].off_yaw = 0.0;
            foot[i].yaw = want_yaw[i];
        }
    } else {
        shown[i] = v3::add(want_p[i], foot[i].off);
    }
    *o = Feet { shown, ..*o };
}

/// A foot not down goes no faster than a foot goes, seen from where he stands: what the pose asks past that it comes to by the same easing.
fn foot_speed(foot: &mut [AnimStep; 2], i: usize, o: &mut Feet) {
    let Feet { dt, s, p0, p_was, teleported, want_p, mut shown, .. } = *o;
    let free = !(foot[i].swing < 0.0 && foot[i].down);
    if dt > 1e-5 && free && !foot[i].fresh && !teleported {
        let gone = v3::sub(v3::sub(shown[i], p0), v3::sub(foot[i].shown, p_was));
        let (l, most) = (hypot3(gone[0], gone[1], gone[2]), min(FEET.top_speed + FEET.top_speed_k * s, FEET.top_speed_most) * dt);
        if l > most {
            let k = most / l;
            shown[i] = [shown[i][0] - gone[0] * (1.0 - k), shown[i][1] - gone[1] * (1.0 - k), shown[i][2] - gone[2] * (1.0 - k)];
            if foot[i].swing < 0.0 { foot[i].off = v3::sub(shown[i], want_p[i]); foot[i].off_v = [0.0; 3]; }
        }
    }
    *o = Feet { shown, ..*o };
}

/// Which way he faces, and the feet as posed.
fn body_turn(o: &mut Feet) {
    let Feet { yin, fl, rl, fr, rr, .. } = *o;
    let (body, body_back) = (q_yaw(yin), q_yaw(-yin));
    let placed = [(fl, rl), (fr, rr)];
    *o = Feet { body, body_back, placed, ..*o };
}

/// It turns no faster than a foot turns, seen in the world; and a toe that would be under the floor comes up onto it.
fn foot_turn(foot: &mut [AnimStep; 2], i: usize, o: &mut Feet) {
    let Feet { dt, p0, yin, teleported, ankle, toe, rot, lift, mut shown, body, body_back, mut placed, .. } = *o;
    let dyaw = wrap_angle((foot[i].yaw - yin) - foot_yaw(&rot[i]));
    let mut q = quat::mul(body, quat::mul(q_yaw(dyaw), rot[i]));
    if dt > 1e-5 && !teleported && foot[i].q_was != [0.0; 4] {
        let a = 2.0 * acos(min(1.0, abs(q[0] * foot[i].q_was[0] + q[1] * foot[i].q_was[1] + q[2] * foot[i].q_was[2] + q[3] * foot[i].q_was[3])));
        let most = FEET.top_turn * dt;
        if a > most { q = quat::slerp(foot[i].q_was, q, most / a); }
    }
    foot[i].q_was = q;
    // and a toe that would be under the floor comes up onto it: in a step, onto it and then the step's lift over that
    let toe_off = quat::rotate(q, quat::rotate([-rot[i][0], -rot[i][1], -rot[i][2], rot[i][3]], v3::sub(toe[i], ankle[i])));
    let up = if foot[i].swing >= 0.0 { sin(PI * min(1.0, foot[i].swing)) * lift } else { 0.0 };
    shown[i][1] = max(shown[i][1] - up, p0[1] + TOE_FLOOR_Y - toe_off[1]) + up;
    placed[i] = (o.stands.model(&shown[i]), quat::mul(body_back, q));
    *o = Feet { shown, placed, ..*o };
}
