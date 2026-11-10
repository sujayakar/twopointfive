//! Breath, weight and balance, and the feet last of all.
use super::*;

#[inline(always)]
fn sign(x: f64) -> f64 {
    if x > 0.0 {
        1.0
    } else if x < 0.0 {
        -1.0
    } else {
        x
    }
}
/// Breathing, the body's mass on starts and stops, the weight shifting foot to foot at rest, and the standing foot lock: the feet planted in the world
/// while he stands, each stepping across in a lifted arc when the body has turned or shifted out from over it.
pub fn life_layer(g: usize, an: &mut Anim, dt: f64, s: f64, crouch_pose: f64, w: f64, w_f: f64, p0: [f64; 3], tune: &Knobs) -> u32 {
    let (at, yin, cr_t, lf) = (an.p_a.at as usize, an.body_yaw_in, an.cr_t, &mut an.life);
    let (f_w, f_next, mut foot) = (&mut an.feet.w, &mut an.feet.next, [an.feet.l, an.feet.r]);
    for f in foot.iter_mut() { f.lock_off = [0.0; 3]; f.lock_yaw = 0.0; }
    let code = with_pose(g, at, |d, rig, n, fk| {
        if !cast(n, &[HIPS, SPINE1, SPINE2, SPINE3, NECK, SHOULDER_L, SHOULDER_R, THIGH_L, SHIN_L, FOOT_L, THIGH_R, SHIN_R, FOOT_R]) {
            return 0;
        }
        // clocks
        if !finite(lf.yaw_prev) {
            lf.yaw_prev = yin;
        }
        let yr = if dt > 0.0 { wrap_angle(yin - lf.yaw_prev) / dt } else { 0.0 };
        lf.yaw_prev = yin;
        lf.yaw_rate = damp(lf.yaw_rate, clamp(yr, -8.0, 8.0), 8.0, dt);
        // the root's OWN planar motion: more than half a metre in a frame is a teleport — the feet re-lock where they land
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
        let hb = true;
        lf.exert = clamp(lf.exert + if s > 3.2 { dt / if hb { tune.hardbreath.chest_up } else { 4.0 } } else { -dt / if hb { tune.hardbreath.chest_down } else { 8.0 } }, 0.0, 1.0);
        let b_rate = lerp(LIFE.breath_rate, if hb { tune.hardbreath.chest_rate } else { LIFE.breath_rate_run }, lf.exert);
        let b_amp = lerp(LIFE.breath_amp, if hb { tune.hardbreath.chest_amp } else { LIFE.breath_amp_run }, lf.exert) * lerp(1.0, 0.35, clamp(s / 2.0, 0.0, 1.0)) * LIFE.on;
        lf.breath = (lf.breath + dt * b_rate * 2.0 * PI) % (2.0 * PI);
        let accel = if dt > 0.0 { (s - lf.prev_s) / dt } else { 0.0 };
        lf.prev_s = s;
        lf.accel = damp(lf.accel, clamp(accel, -12.0, 12.0), 10.0, dt);
        let t = clamp(-lf.accel * LIFE.mass_gain, -LIFE.mass_back, LIFE.mass_fwd) * LIFE.on;
        spring_step(&mut lf.mass, t, 9.0, dt);
        let t = clamp(-abs(lf.accel) * LIFE.mass_dip, -LIFE.mass_dip_max, 0.0) * LIFE.on;
        spring_step(&mut lf.mass_v, t, 10.0, dt);
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
        if w_f < 0.003 {
            *f_w = 0.0;
        }
        if w < 0.003 && w_f < 0.003 {
            return RAN;
        }
        // remember the feet, pose the body, put the feet back
        let (hips, s1, s2, s3, nk) = (role(HIPS), role(SPINE1), role(SPINE2), role(SPINE3), role(NECK));
        let (foot_l, foot_r) = (role(FOOT_L), role(FOOT_R));
        let legs = list(FK_LEGS);
        fk.part(rig, d, legs);
        let ran = RAN | ran_list(FK_LEGS);
        let (fl, fr, rl, rr) = (fk.at(foot_l), fk.at(foot_r), fk.rot(foot_l), fk.rot(foot_r));
        if w >= 0.003 {
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
            fk.part(rig, d, legs);
        }
        // ---- feet: planted in the WORLD while he stands, released into the cycle's feet as he moves off
        let standing = if max(s, lf.root_v) < 0.22 && w_f > 0.5 && FEET.on > 0.5 && !(cr_t > 0.0 && cr_t < 1.0) { 1.0 } else { 0.0 };
        let w_was = *f_w;
        *f_w = damp(*f_w, standing, if standing != 0.0 { 12.0 } else { 16.0 + 40.0 * lf.root_v }, dt);
        if w < 0.003 && *f_w <= 0.01 {
            return ran;
        }
        let (cy, sy) = (cos(yin), sin(yin));
        let to_world = |m: &[f64; 3]| -> [f64; 3] { [p0[0] + m[0] * cy + m[2] * sy, p0[1] + m[1], p0[2] - m[0] * sy + m[2] * cy] };
        let to_model = |wp: &[f64; 3]| -> [f64; 3] {
            let (dx, dz) = (wp[0] - p0[0], wp[2] - p0[2]);
            [dx * cy - dz * sy, wp[1] - p0[1], dx * sy + dz * cy]
        };
        let foot_yaw = |r: &[f64; 4]| -> f64 {
            let f = q_rotate(r, &AY);
            atan2(f[0], f[2])
        };
        let (mut tl, mut tr, mut ql, mut qr) = (fl, fr, rl, rr);
        if *f_w > 0.01 {
            let want_p = [to_world(&fl), to_world(&fr)];
            let want_yaw = [yin + foot_yaw(&rl), yin + foot_yaw(&rr)];
            let relock = w_was <= 0.01 || teleported || dist_xz(&foot[0].p, &want_p[0]) > 0.45 || dist_xz(&foot[1].p, &want_p[1]) > 0.45;
            if relock {
                for i in 0..2 {
                    foot[i].p = want_p[i];
                    foot[i].yaw = want_yaw[i];
                    foot[i].swing = -1.0;
                }
            }
            let (swing, lift, dist, turn_at, pivot, lag_cap) = (FEET.swing, FEET.lift, FEET.dist, FEET.turn_deg * DEGR, FEET.pivot, FEET.lag_cap_deg * DEGR);
            let far_of = |foot: &[AnimFoot; 2], i: usize| -> bool { dist_xz(&foot[i].p, &want_p[i]) > dist || abs(wrap_angle(foot[i].yaw - want_yaw[i])) > turn_at };
            for i in 0..2 {
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
                    if far_of(&foot, i) && foot[1 - i].swing < 0.0 && (my_turn || !far_of(&foot, 1 - i)) {
                        foot[i].swing = 0.0;
                        foot[i].from = foot[i].p;
                        foot[i].from_yaw = foot[i].yaw;
                        *f_next = (1 - i) as f64;
                    }
                }
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
            let mk = |f: &AnimFoot, rest_p: &[f64; 3], rest_r: &[f64; 4]| -> ([f64; 3], [f64; 4]) {
                let mp = to_model(&f.p);
                let fw = *f_w;
                let p = [lerp(rest_p[0], mp[0], fw), lerp(rest_p[1], mp[1], fw), lerp(rest_p[2], mp[2], fw)];
                let dyaw = wrap_angle((f.yaw - yin) - foot_yaw(rest_r)) * fw;
                (p, q_mul(&q_yaw(dyaw), rest_r))
            };
            (tl, ql) = mk(&foot[0], &fl, &rl);
            (tr, qr) = mk(&foot[1], &fr, &rr);
            // (feet_last reads the stack's feet without this)
            foot[0].lock_off = sub3(tl, fl);
            foot[1].lock_off = sub3(tr, fr);
            foot[0].lock_yaw = wrap_angle((foot[0].yaw - yin) - foot_yaw(&rl)) * *f_w;
            foot[1].lock_yaw = wrap_angle((foot[1].yaw - yin) - foot_yaw(&rr)) * *f_w;
        }
        // put the feet back: the knee steered over the toes only as far as the feet are LOCKED
        let pole_w = if cr_t > 0.0 { 0.6 * *f_w * (1.0 - smoothstep(0.0, 1.0, cr_t)) } else { 0.6 * *f_w };
        two_bone_ik(rig, d, fk.rot, fk.pos, role(THIGH_L), role(SHIN_L), foot_l, &tl, Some(&knee_pole(&ql)), pole_w, Some(&ql), false);
        two_bone_ik(rig, d, fk.rot, fk.pos, role(THIGH_R), role(SHIN_R), foot_r, &tr, Some(&knee_pole(&qr)), pole_w, Some(&qr), true);
        ran | RAN_ALL
    })
    .unwrap_or(0);
    (an.feet.l, an.feet.r) = (foot[0], foot[1]);
    code
}
/// THE RULE OF A FOOT, last of the stack: a foot that is down stays where it is; it goes somewhere else only through the air, and no faster than a foot goes.
/// It reads the feet the finished pose has, without the standing lock (life_layer's, which every layer above was posed over and goes on being),
/// and writes the thigh, the shin and the foot of each leg and nothing above the hips. `w_f`: the legs are his to stand on; `own`: how much of what the stack has in them is the stack's and not a whole-body clip's.
pub fn feet_last(g: usize, an: &mut Anim, dt: f64, s: f64, w_f: f64, own: f64, p0: [f64; 3]) -> u32 {
    let (at, yin, cr_t, f_w) = (an.p_a.at as usize, an.body_yaw_in, an.cr_t, an.feet.w);
    let (lock_off, lock_yaw) = ([an.feet.l.lock_off, an.feet.r.lock_off], [an.feet.l.lock_yaw, an.feet.r.lock_yaw]);
    let (was_known, was_at) = (an.feet.at_known, an.feet.at_was);
    let mut foot = [an.feet.step_l, an.feet.step_r];
    // his own motion over the ground: more than half a metre in a frame is a teleport, and the feet are set down where they land
    let p_was = if was_known { was_at } else { p0 };
    let dd = hypot2(p0[0] - p_was[0], p0[2] - p_was[2]);
    let teleported = dd > 0.5;
    let went = if dd > 1e-6 { [(p0[0] - p_was[0]) / dd, (p0[2] - p_was[2]) / dd] } else { [0.0, 0.0] };
    (an.feet.at_known, an.feet.at_was) = (true, p0);
    let code = with_pose(g, at, |d, rig, n, fk| {
        if !cast(n, &[HIPS, THIGH_L, SHIN_L, FOOT_L, TOE_L, THIGH_R, SHIN_R, FOOT_R, TOE_R]) {
            return 0;
        }
        if w_f < 0.003 {
            for f in foot.iter_mut() { f.down = false; f.swing = -1.0; f.off = [0.0; 3]; f.off_v = [0.0; 3]; f.off_yaw = 0.0; f.clear = 0.0; f.fresh = true; }
            return RAN;
        }
        let active = w_f > 0.5 && FEET.on > 0.5;
        if !active && foot.iter().all(|f| f.swing < 0.0 && f.off == [0.0; 3] && f.off_yaw == 0.0 && f.clear == 0.0) {
            return RAN;
        }
        let (foot_l, foot_r) = (role(FOOT_L), role(FOOT_R));
        let legs = list(FK_LEGS);
        fk.part(rig, d, legs);
        let ran = RAN | ran_list(FK_LEGS);
        // the feet the stack has, the standing lock taken back out of them
        let k = if active { own } else { 0.0 };
        let without = |i: usize, a: [f64; 3], t: [f64; 3], r: [f64; 4]| -> ([f64; 3], [f64; 3], [f64; 4]) {
            let un = q_yaw(-lock_yaw[i] * k);
            let o = lock_off[i];
            let at = [a[0] - o[0] * k, a[1] - o[1] * k, a[2] - o[2] * k];
            (at, add3(at, q_rotate(&un, &sub3(t, a))), q_mul(&un, &r))
        };
        let (fl, toe_pose_l, rl) = without(0, fk.at(foot_l), fk.at(role(TOE_L)), fk.rot(foot_l));
        let (fr, toe_pose_r, rr) = without(1, fk.at(foot_r), fk.at(role(TOE_R)), fk.rot(foot_r));
        let (cy, sy) = (cos(yin), sin(yin));
        let to_world = |m: &[f64; 3]| -> [f64; 3] { [p0[0] + m[0] * cy + m[2] * sy, p0[1] + m[1], p0[2] - m[0] * sy + m[2] * cy] };
        let to_model = |wp: &[f64; 3]| -> [f64; 3] {
            let (dx, dz) = (wp[0] - p0[0], wp[2] - p0[2]);
            [dx * cy - dz * sy, wp[1] - p0[1], dx * sy + dz * cy]
        };
        let foot_yaw = |r: &[f64; 4]| -> f64 {
            let f = q_rotate(r, &AY);
            atan2(f[0], f[2])
        };
        let (ankle, toe, rot) = ([fl, fr], [toe_pose_l, toe_pose_r], [rl, rr]);
        let want_p = [to_world(&fl), to_world(&fr)];
        let want_toe = [to_world(&toe_pose_l), to_world(&toe_pose_r)];
        let want_yaw = [yin + foot_yaw(&rl), yin + foot_yaw(&rr)];
        let (swing, lift, turn_at, pivot, lag_cap) = (FEET.swing, FEET.lift, FEET.turn_deg * DEGR, FEET.pivot, FEET.lag_cap_deg * DEGR);
        // how far the pose may want a foot from where it stands before it steps: a stand's small shuffle, a stride's reach
        let reach_now = lerp(FEET.dist, FEET.reach, min(s, 1.0));
        let far_of = |f: &AnimStep, i: usize| -> f64 {
            let d = hypot2(f.off[0], f.off[2]);
            if d > reach_now || abs(wrap_angle(f.yaw - want_yaw[i])) > turn_at { d + 1e-3 } else { 0.0 }
        };
        // from the ankle to the toe over the ground, turned as far as the foot is off the pose's way
        let lever = |i: usize, off_yaw: f64| -> [f64; 3] {
            let (dx, dz) = (want_toe[i][0] - want_p[i][0], want_toe[i][2] - want_p[i][2]);
            let (c, s_) = (cos(off_yaw), sin(off_yaw));
            [dx * c + dz * s_, 0.0, dz * c - dx * s_]
        };
        for i in 0..2 {
            // the cycle under the foot: at the floor or lifted, going forward over the ground or not
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
            // where its pivot stood in the last frame's picture: what it holds from now on
            let pivot_at = |f: &AnimStep, on_toe: bool| -> [f64; 3] {
                if f.fresh { return if on_toe { want_toe[i] } else { want_p[i] }; }
                if on_toe { add3(f.shown, lever(i, f.off_yaw)) } else { f.shown }
            };
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
                        foot[i].p = pivot_at(&foot[i], toe_down);
                        foot[i].yaw = want_yaw[i] + foot[i].off_yaw;
                    }
                }
            }
            // over the ground: down it holds; let go what it is off by eases out, critically damped and at no more than a foot's speed. Up and down is always the pose's, eased.
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
            if foot[i].swing < 0.0 && foot[i].down {
                // it holds its place, on the toe, or on the heel while the toe is still coming down; its way within what the ankle takes
                if foot[i].toe_down != toe_down {
                    foot[i].p = pivot_at(&foot[i], toe_down);
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
                let under = if toe_down { add3(want_p[i], lever(i, foot[i].off_yaw)) } else { want_p[i] };
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
        // a foot down that the pose wants far from it steps, the one further off first; the other may start as this one comes to its last third
        if active && dt > 1e-5 {
            for _ in 0..2 {
                let d = [if foot[0].swing < 0.0 && foot[0].down { far_of(&foot[0], 0) } else { 0.0 }, if foot[1].swing < 0.0 && foot[1].down { far_of(&foot[1], 1) } else { 0.0 }];
                let i = if d[0] >= d[1] { 0 } else { 1 };
                if d[i] <= 0.0 || (foot[1 - i].swing >= 0.0 && foot[1 - i].swing < 2.0 / 3.0) {
                    break;
                }
                let f = &mut foot[i];
                f.from = add3(want_p[i], f.off);
                f.swing = 0.0;
                f.from_yaw = f.yaw;
                f.down = false;
            }
        }
        let mut shown = [want_p[0], want_p[1]];
        for i in 0..2 {
            if foot[i].swing >= 0.0 {
                // through the air: the further off past what a step takes, the quicker the step, so a fast turn is quick steps and the foot gets there
                foot[i].swing += dt / swing * max(1.0, dist_xz(&foot[i].from, &want_p[i]) / FEET.reach);
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
                shown[i] = add3(want_p[i], foot[i].off);
            }
            shown[i][1] += foot[i].clear;
            // a foot not down goes no faster than a foot goes, seen from where he stands: what the pose asks past that it comes to by the same easing. Down, it is the body that moves.
            let free = !(foot[i].swing < 0.0 && foot[i].down);
            if dt > 1e-5 && free && !foot[i].fresh && !teleported {
                let d = sub3(sub3(shown[i], p0), sub3(foot[i].shown, p_was));
                let (l, most) = (hypot3(d[0], d[1], d[2]), min(FEET.top_speed + FEET.top_speed_k * s, FEET.top_speed_most) * dt);
                if l > most {
                    let k = most / l;
                    shown[i] = [shown[i][0] - d[0] * (1.0 - k), shown[i][1] - d[1] * (1.0 - k), shown[i][2] - d[2] * (1.0 - k)];
                    if foot[i].swing < 0.0 { foot[i].off = sub3(shown[i], want_p[i]); foot[i].off_v = [0.0; 3]; }
                }
            }
            foot[i].shown = shown[i];
            foot[i].fresh = false;
        }
        // and it turns no faster than a foot turns, seen in the world: a pose that throws a foot over all at once is met over a few frames
        let (body, body_back) = (q_yaw(yin), q_yaw(-yin));
        let mut placed = [(fl, rl), (fr, rr)];
        for i in 0..2 {
            let dyaw = wrap_angle((foot[i].yaw - yin) - foot_yaw(&rot[i]));
            let mut q = q_mul(&body, &q_mul(&q_yaw(dyaw), &rot[i]));
            if dt > 1e-5 && !teleported && foot[i].q_was != [0.0; 4] {
                let a = 2.0 * acos(min(1.0, abs(q[0] * foot[i].q_was[0] + q[1] * foot[i].q_was[1] + q[2] * foot[i].q_was[2] + q[3] * foot[i].q_was[3])));
                let most = FEET.top_turn * dt;
                if a > most { q = slerp(&foot[i].q_was, &q, most / a); }
            }
            foot[i].q_was = q;
            // and a toe that would be under the floor comes up onto it: in a step, onto it and then the step's lift over that
            let toe_off = q_rotate(&q, &q_rotate(&[-rot[i][0], -rot[i][1], -rot[i][2], rot[i][3]], &sub3(toe[i], ankle[i])));
            let up = if foot[i].swing >= 0.0 { sin(PI * min(1.0, foot[i].swing)) * lift } else { 0.0 };
            shown[i][1] = max(shown[i][1] - up, p0[1] + TOE_FLOOR_Y - toe_off[1]) + up;
            placed[i] = (to_model(&shown[i]), q_mul(&body_back, &q));
        }
        let ((tl, ql), (tr, qr)) = (placed[0], placed[1]);
        // the knee over the toes only as far as he stands
        let pole_w = if cr_t > 0.0 { 0.6 * f_w * (1.0 - smoothstep(0.0, 1.0, cr_t)) } else { 0.6 * f_w };
        two_bone_ik(rig, d, fk.rot, fk.pos, role(THIGH_L), role(SHIN_L), foot_l, &tl, Some(&knee_pole(&ql)), pole_w, Some(&ql), false);
        two_bone_ik(rig, d, fk.rot, fk.pos, role(THIGH_R), role(SHIN_R), foot_r, &tr, Some(&knee_pole(&qr)), pole_w, Some(&qr), true);
        ran | RAN_ALL
    })
    .unwrap_or(0);
    (an.feet.step_l, an.feet.step_r) = (foot[0], foot[1]);
    code
}
