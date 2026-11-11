//! What the arms do with a gun or a torch in them.
use super::*;

/// Both arms onto the pistol: the gun hand at the chest + the stance's [side, up, fwd], the muzzle at its pitch and level in roll, the support hand
/// keeping the exact hold it has in the aim clip (both hands moved by one rigid transform of the clip's grip); elbows swivelled down and in by the tuck;
/// a little sway, and during a kick a pump of the gun toward the chest. Solved on a copy seeded with the aim clip's arms (`at_aim`).
pub fn carry_arms(g: usize, an: &Anim, w: f64, kick_t: f64) -> bool {
    let (at, at_b, _) = poses(an);
    let (at_aim, t, st, grip_up) = (an.p_aim.at as usize, an.idle_t, &an.st, an.grip_up);
    let (st_fwd, st_up, st_side, st_pitch, st_blade, tuck) = (st.fwd.x, st.up.x, st.side.x, st.pitch.x, st.blade.x, st.tuck.x);
    let Some(Some(n)) = with_pose(g, at, |_d, _rig, n, _fk| if cast(n, &ARM_ROLES) && cast(n, &[MIDDLE1_R]) { Some(n) } else { None }) else {
        return false;
    };
    // the body as it is, the aim clip's arms
    arms_mask();
    if !ask_blend(g, at, at, 1.0, false, at_b, n) || !ask_blend(g, at_b, at_aim, 1.0, true, at_b, n) {
        return false;
    }
    let code = with_pose(g, at_b, |d, rig, _n, fk| {
        fk.part(rig, d, list(FK_ARMS));
        let (hand_r, hand_l) = (role(HAND_R), role(HAND_L));
        let (chest, pr0, pl0, rr0) = (fk.at(role(SPINE3)), fk.at(hand_r), fk.at(hand_l), fk.rot(hand_r));
        // the clip's bore axis as posed on this spine
        let g0 = v3::normalize(v3::sub(fk.at(role(MIDDLE1_R)), pr0));
        // where the gun goes: sway (a slow figure of eight, millimetres) and the kick's pump
        let pump = if kick_t >= 0.0 { 0.05 * bump(kick_t, 0.15, 0.36, 0.5, 0.85) } else { 0.0 };
        let (fwd, up, side) = (st_fwd - pump, st_up + 0.004 * sin(t * 2.1) + 0.02 * pump, st_side + 0.005 * sin(t * 1.3 + 0.6));
        let (pitch, yaw) = ((st_pitch + 0.8 * sin(t * 1.7 + 1.1)) * DEGR + pump * 4.0, 0.6 * sin(t * 1.1) * DEGR + st_blade * DEGR * 0.2);
        let gd = [sin(yaw) * cos(pitch), sin(pitch), cos(yaw) * cos(pitch)];
        let turn = grip_turn(&g0, &rr0, &gd, &AY, &grip_up);
        let pr = [chest[0] + side, chest[1] + up, chest[2] + fwd];
        // the support wrist relative to the gun wrist, turned with the gun
        let grip = quat::rotate(turn, v3::sub(pl0, pr0));
        two_bone_ik(rig, d, fk.rot, fk.pos, role(UPPER_ARM_R), role(FOREARM_R), hand_r, &pr, Some(&[-0.12 - 0.6 * (1.0 - tuck), -1.0, -0.5]), 0.4 + 0.55 * tuck, Some(&quat::mul(turn, rr0)), true);
        // (from where the gun hand actually got to: the hands must not part; the support wrist's rotation read as the solve left the bones)
        let (pl, rl0) = (v3::add(fk.at(hand_r), grip), fk.rot(hand_l));
        two_bone_ik(rig, d, fk.rot, fk.pos, role(UPPER_ARM_L), role(FOREARM_L), hand_l, &pl, Some(&[0.2 + 0.6 * (1.0 - tuck), -1.0, -0.35]), 0.4 + 0.5 * tuck, Some(&quat::mul(turn, rl0)), true);
        true
    })
    .unwrap_or(false);
    if !code {
        return false;
    }
    arms_mask();
    if !ask_blend(g, at, at_b, w, true, at, n) {
        return false;
    }
    code
}
/// the long gun's channels, in the order the animator keeps them
const LC_SIDE: usize = 0;
const LC_UP: usize = 1;
const LC_FWD: usize = 2;
const LC_PITCH: usize = 3;
const LC_AIM_K: usize = 4;
const LC_YAW: usize = 5;
const LC_ELBOW_R: usize = 11;
const LC_ELBOW_L: usize = 14;
const LC_POLE_W: usize = 17;
const LC_SWAY: usize = 18;
const LC_ANCHOR: usize = 19;
const LC_STEADY: usize = 20;
/// What the steps of a long gun in his hands hand on to the ones after them.
#[derive(Clone, Copy, Default)]
struct LongArms {
    kick_t: f64,
    dt: f64,
    keep: bool,
    rl_at: [f64; 3],
    // as_carried: how he carries it and how he was trained
    t: f64,
    aim: f64,
    aim_pitch: f64,
    long_steady: f64,
    trained: f64,
    rl_k: f64,
    mvs_yaw: f64,
    mvs_climb: f64,
    mvs_back: f64,
    dl: f64,
    dr: f64,
    grip_up: [f64; 3],
    fix_l: [f64; 4],
    g_wrist: [f64; 3],
    g_fingers: [f64; 3],
    g_back: [f64; 3],
    // clip_bore: the chest, the trigger hand and the clip's bore as posed on this spine
    chest: [f64; 3],
    rr0: [f64; 4],
    g0: [f64; 3],
    // bore: the bore in the model, and the gun's own frame about it: y up out of the receiver, x to the figure's left
    sway: f64,
    dd: [f64; 3],
    rr: [f64; 4],
    yw: [f64; 3],
    xw: [f64; 3],
    // anchor_at: the trigger hand: the carriage's point off the chest joint, pumped toward the chest by a kick
    pump: f64,
    ka: f64,
    anchor: [f64; 3],
    // off_chest: where the hand is off the chest, the reload carry's own point in it
    off: [f64; 3],
    // hand_point: the hand's point, back by the kick
    pr: [f64; 3],
    // elbows: the elbows, by training, shouldered; and the support hand's turn
    pole_r: [f64; 3],
    pole_l: [f64; 3],
    pw: f64,
    rl: [f64; 4],
}

/// The long gun's arms: the trigger hand's wrist at the carriage's point off the chest joint (or, by its anchor, off the gun shoulder), swung about the
/// shoulders with the aim's share of the pitch, holding the grip as the aim clip holds the pistol's turned onto the wanted bore; the support hand solved
/// onto the gun's grip point from where the trigger hand got to. Should the support arm fall short the whole gun is brought back by the shortfall
/// (capped) and both re-solved, and that hold-back is kept and let out over half a second. Solved on a copy seeded with the aim clip's arms.
pub fn long_arms(g: usize, an: &mut Anim, w: f64, kick_t: f64, dt: f64, yield_door: bool, keep: bool, ch: &[f64; LC_COUNT], rl_at: [f64; 3], tune: &Knobs) -> bool {
    let mut o = LongArms { kick_t, dt, keep, rl_at, ..LongArms::default() };
    let (at, at_b, _) = poses(an);
    let at_aim = an.p_aim.at as usize;
    as_carried(an, &mut o);
    let Some(Some(n)) = with_pose(g, at, |_d, _rig, n, _fk| if cast(n, &ARM_ROLES) && cast(n, &[MIDDLE1_R]) { Some(n) } else { None }) else {
        return false;
    };
    let Some(mut h) = hands_of(g, an) else {
        return false;
    };
    // the body as it is, the aim clip's clavicles, trigger grip and fingers to start from
    arms_mask();
    if !ask_blend(g, at, at, 1.0, false, at_b, n) || !ask_blend(g, at_b, at_aim, 1.0, true, at_b, n) {
        return false;
    }
    let (hh, lg) = (&mut h, &mut an.lg);
    let code = with_pose(g, at_b, |d, rig, _n, fk| {
        fk.part(rig, d, list(FK_ARMS));
        let (hand_r, hand_l) = (role(HAND_R), role(HAND_L));
        clip_bore(fk, hand_r, &mut o);
        bore(ch, &mut o);
        anchor_at(ch, fk, &mut o);
        steady(lg, ch, &mut o);
        off_chest(ch, &mut o);
        swing(ch, &mut o);
        hand_point(&mut o);
        elbows(tune, ch, &mut o);
        hold_within_reach(lg, d, fk, rig, hand_r, hand_l, &o);
        // the support hand cupped round the handguard
        hand(d, hh, 0, &[0.5, 0.55, 0.6, 0.68], 0.35, 0.0);
        true
    })
    .unwrap_or(false);
    if !code {
        return false;
    }
    if yield_door {
        // arm by arm, short of whatever a hand on a door still holds
        let (wl, wr) = (w * (1.0 - clamp(o.dl, 0.0, 1.0)), w * (1.0 - clamp(o.dr, 0.0, 1.0)));
        ask_mask(&[(role(SHOULDER_L), 1.0, true)]);
        if !ask_blend(g, at, at_b, wl, true, at, n) {
            return false;
        }
        ask_mask(&[(role(SHOULDER_R), 1.0, true)]);
        if !ask_blend(g, at, at_b, wr, true, at, n) {
            return false;
        }
    } else {
        arms_mask();
        if !ask_blend(g, at, at_b, w, true, at, n) {
            return false;
        }
    }
    hands_to(&h, an);
    code
}

/// How he carries it and how he was trained.
fn as_carried(an: &Anim, o: &mut LongArms) {
    let (t, aim, aim_pitch, long_steady, trained, rl_k) = (an.idle_t, an.upper_w.aim, an.aim_pitch, an.long_steady, an.trained, an.rlc.k);
    let (mvs_yaw, mvs_climb, mvs_back, dl, dr) = (an.mvs.yaw, an.mvs.climb, an.mvs.back, an.dp.w_l.x, an.dp.w_r.x);
    let (grip_up, fix_l, g_wrist, g_fingers, g_back) = (an.grip_up, an.hand_fix[0], an.long_grip.wrist, an.long_grip.fingers, an.long_grip.back);
    o.t = t;
    o.aim = aim;
    o.aim_pitch = aim_pitch;
    o.long_steady = long_steady;
    o.trained = trained;
    o.rl_k = rl_k;
    o.mvs_yaw = mvs_yaw;
    o.mvs_climb = mvs_climb;
    o.mvs_back = mvs_back;
    o.dl = dl;
    o.dr = dr;
    o.grip_up = grip_up;
    o.fix_l = fix_l;
    o.g_wrist = g_wrist;
    o.g_fingers = g_fingers;
    o.g_back = g_back;
}

/// The chest, the trigger hand and the clip's bore as posed on this spine.
fn clip_bore(fk: &mut Fk, hand_r: usize, o: &mut LongArms) {
    let (chest, pr0, rr0) = (fk.at(role(SPINE3)), fk.at(hand_r), fk.rot(hand_r));
    // the clip's bore as posed on this spine
    let g0 = v3::normalize(v3::sub(fk.at(role(MIDDLE1_R)), pr0));
    (o.chest, o.rr0, o.g0) = (chest, rr0, g0);
}

/// The bore in the model, and the gun's own frame about it: y up out of the receiver, x to the figure's left.
fn bore(ch: &[f64; LC_COUNT], o: &mut LongArms) {
    let LongArms { t, aim, aim_pitch, mvs_yaw, mvs_climb, grip_up, rr0, g0, .. } = *o;
    let sway = ch[LC_SWAY];
    let yaw = (ch[LC_YAW] + 0.6 * sway * sin(t * 1.1) + mvs_yaw) * DEGR;
    let pitch = clamp((ch[LC_PITCH] + 0.8 * sway * sin(t * 1.7 + 1.1) + mvs_climb) * DEGR + aim_pitch * (ch[LC_AIM_K] - 0.35 * aim), -1.35, 1.35);
    let cp = cos(pitch);
    let dd = [sin(yaw) * cp, sin(pitch), cos(yaw) * cp];
    let rr = quat::mul(grip_turn(&g0, &rr0, &dd, &AY, &grip_up), rr0);
    // the gun's own frame about that bore: y up out of the receiver, x to the figure's left
    let flat = v3::mad(AY, dd, -dd[1]);
    let yw = if v3::len(flat) > 1e-4 { v3::normalize(flat) } else { [cos(yaw), 0.0, -sin(yaw)] };
    let xw = v3::cross(yw, dd);
    (o.sway, o.dd, o.rr, o.yw, o.xw) = (sway, dd, rr, yw, xw);
}

/// A point off the gun in its own frame.
fn in_gun(o: &LongArms, p: &[f64; 3]) -> [f64; 3] {
    let LongArms { dd, yw, xw, .. } = *o;
    [xw[0] * p[0] + yw[0] * p[1] + dd[0] * p[2], xw[1] * p[0] + yw[1] * p[1] + dd[1] * p[2], xw[2] * p[0] + yw[2] * p[1] + dd[2] * p[2]]
}

/// The trigger hand: the carriage's point off the chest joint, pumped toward the chest by a kick.
fn anchor_at(ch: &[f64; LC_COUNT], fk: &mut Fk, o: &mut LongArms) {
    let LongArms { kick_t, chest, .. } = *o;
    let pump = if kick_t >= 0.0 { 0.04 * bump(kick_t, 0.15, 0.36, 0.5, 0.85) } else { 0.0 };
    let ka = clamp(ch[LC_ANCHOR], 0.0, 1.0);
    let sh = fk.at(role(UPPER_ARM_R));
    let mut anchor = [lerp(chest[0], sh[0], ka), lerp(chest[1], sh[1], ka), lerp(chest[2], sh[2], ka)];
    anchor[0] = max(anchor[0], chest[0] - LONG_ANCHOR_SIDE_MAX);
    (o.pump, o.ka, o.anchor) = (pump, ka, anchor);
}

/// Steadied against the gait: the anchor's side-to-side followed through a two-pole low-pass.
fn steady(lg: &mut game_types::character::AnimLg, ch: &[f64; LC_COUNT], o: &mut LongArms) {
    let LongArms { dt, keep, long_steady, mut anchor, .. } = *o;
    {
        let (mut l0, mut l1) = (lg.lat[0], lg.lat[1]);
        if !finite(l0) || !finite(l1) {
            l0 = anchor[0];
            l1 = anchor[0];
        } else {
            let kk = 1.0 - exp(-dt / max(1e-3, LONG_STEADY.tau));
            l0 += (anchor[0] - l0) * kk;
            l1 += (l0 - l1) * kk;
        }
        if keep {
            lg.lat[0] = l0;
            lg.lat[1] = l1;
        }
        let iso = clamp(ch[LC_STEADY], 0.0, 1.0) * clamp(long_steady, 0.0, 1.0);
        if iso > 1e-3 {
            anchor[0] = lerp(anchor[0], l1, iso);
        }
    }
    o.anchor = anchor;
}

/// Where the hand is off the chest, the reload carry's own point in it.
fn off_chest(ch: &[f64; LC_COUNT], o: &mut LongArms) {
    let LongArms { rl_at, t, rl_k, chest, sway, pump, anchor, .. } = *o;
    let mut off = [
        anchor[0] - chest[0] + ch[LC_SIDE] + 0.004 * sway * sin(t * 1.3 + 0.6),
        anchor[1] - chest[1] + ch[LC_UP] + 0.003 * sway * sin(t * 2.1),
        anchor[2] - chest[2] + ch[LC_FWD] - pump,
    ];
    if rl_k > 0.0 {
        // (the reload carry's own point)
        off[0] += (rl_at[0] - off[0]) * rl_k;
        off[1] += (rl_at[1] - off[1]) * rl_k;
        off[2] += (rl_at[2] - off[2]) * rl_k;
    }
    o.off = off;
}

/// The swing's pivot, between the shoulders or the gun shoulder itself, less the twist's tilt of the whole trunk about the chest joint.
fn swing(ch: &[f64; LC_COUNT], o: &mut LongArms) {
    let LongArms { aim, aim_pitch, chest, ka, anchor, mut off, .. } = *o;
    {
        // the swing's pivot: between the shoulders … the gun shoulder itself
        let (py, pz) = (lerp(LONG_PIVOT[1], anchor[1] - chest[1], ka), lerp(LONG_PIVOT[2], anchor[2] - chest[2], ka));
        let (th, ry, rz) = (aim_pitch * ch[LC_AIM_K], off[1] - py, off[2] - pz);
        off[1] = py + ry * cos(th) + rz * sin(th);
        off[2] = pz - ry * sin(th) + rz * cos(th);
        // less the twist's tilt of the whole trunk about the chest joint
        let (ph, oy, oz) = (-0.35 * aim_pitch * aim, off[1], off[2]);
        off[1] = oy * cos(ph) + oz * sin(ph);
        off[2] = -oy * sin(ph) + oz * cos(ph);
    }
    o.off = off;
}

/// The hand's point, back by the kick.
fn hand_point(o: &mut LongArms) {
    let LongArms { mvs_back, chest, dd, off, .. } = *o;
    let mut pr = v3::add(chest, off);
    if mvs_back > 1e-5 {
        pr = v3::mad(pr, dd, -mvs_back);
    }
    o.pr = pr;
}

/// The elbows, by training, shouldered; and the support hand's turn.
fn elbows(tune: &Knobs, ch: &[f64; LC_COUNT], o: &mut LongArms) {
    let LongArms { aim, trained, fix_l, g_fingers, g_back, .. } = *o;
    let mut pole_r = [ch[LC_ELBOW_R], ch[LC_ELBOW_R + 1], ch[LC_ELBOW_R + 2]];
    let mut pole_l = [ch[LC_ELBOW_L], ch[LC_ELBOW_L + 1], ch[LC_ELBOW_L + 2]];
    let pw = clamp(ch[LC_POLE_W], 0.0, 1.0);
    // the elbows by training, shouldered
    let (tw, uw) = (smoothstep(0.5, 1.0, trained) * aim, (1.0 - smoothstep(0.0, 0.5, trained)) * aim);
    pole_r[0] += tune.mv.gait.elbow_in * tw;
    pole_l[0] -= tune.mv.gait.elbow_in * 0.8 * tw;
    pole_r[1] += tune.mv.gait.wing * uw;
    pole_r[0] -= 0.25 * uw;
    let rl = hand_rot(&fix_l, in_gun(o, &g_fingers), in_gun(o, &g_back));
    (o.pole_r, o.pole_l, o.pw, o.rl) = (pole_r, pole_l, pw, rl);
}

/// The arms at a point; how far the support hand fell short of its own (nought when it reached).
fn solve(rig: &Rig, hand_r: usize, hand_l: usize, o: &LongArms, d: &mut [S], fk: &mut Fk, to: [f64; 3]) -> [f64; 3] {
    let LongArms { g_wrist, rr, pole_r, pole_l, pw, rl, .. } = *o;
    let (up_r, fore_r, up_l, fore_l) = (role(UPPER_ARM_R), role(FOREARM_R), role(UPPER_ARM_L), role(FOREARM_L));
    two_bone_ik(rig, d, fk.rot, fk.pos, up_r, fore_r, hand_r, &to, Some(&pole_r), pw, Some(&rr), true);
    // from where the trigger hand actually got to
    let pl = v3::add(fk.at(hand_r), in_gun(o, &g_wrist));
    two_bone_ik(rig, d, fk.rot, fk.pos, up_l, fore_l, hand_l, &pl, Some(&pole_l), pw, Some(&rl), true);
    v3::sub(fk.at(hand_l), pl)
}

/// Out of the support arm's reach the gun comes back to where it can be held, at once, and is let out again over half a second.
fn hold_within_reach(lg: &mut game_types::character::AnimLg, d: &mut [S], fk: &mut Fk, rig: &Rig, hand_r: usize, hand_l: usize, o: &LongArms) {
    let LongArms { dt, keep, pr, .. } = *o;
    let mut pull = [lg.pull[0], lg.pull[1], lg.pull[2]];
    let m = solve(rig, hand_r, hand_l, o, d, fk, v3::add(pr, pull));
    if v3::len(m) > 5e-4 {
        // still short with what is held: take up the rest now (capped), and pose there
        pull = [pull[0] + m[0], pull[1] + m[1], pull[2] + m[2]];
        let l = v3::len(pull);
        if l > LONG_PULL_MAX {
            let kk = LONG_PULL_MAX / l;
            pull = [pull[0] * kk, pull[1] * kk, pull[2] * kk];
        }
        solve(rig, hand_r, hand_l, o, d, fk, v3::add(pr, pull));
    } else if keep {
        // reached: let a little out for next frame
        let kk = exp(-2.0 * dt);
        pull = [pull[0] * kk, pull[1] * kk, pull[2] * kk];
    }
    if keep {
        lg.pull[0] = pull[0];
        lg.pull[1] = pull[1];
        lg.pull[2] = pull[2];
    }
}
/// The flashlight brought up in the gun hand: the right wrist to a spot ahead of the shoulder, the elbow down and a little out, by e. Solved on the
/// scratch pose at `at_b`, laid on the right arm.
pub fn torch_up(g: usize, an: &Anim, e: f64) -> bool {
    let (at, at_b, _) = poses(an);
    let fix_r = an.hand_fix[1];
    let Some(Some(n)) = with_pose(g, at, |_d, _rig, n, _fk| if cast(n, &ARM_ROLES) { Some(n) } else { None }) else {
        return false;
    };
    if !ask_blend(g, at, at, 1.0, false, at_b, n) {
        return false;
    }
    with_pose(g, at_b, |d, rig, _n, fk| {
        fk_run(rig, d, fk.rot, fk.pos);
        let (sh, live0, rot0) = (fk.at(role(UPPER_ARM_R)), fk.at(role(HAND_R)), fk.rot(role(HAND_R)));
        let wrist = [sh[0] - 0.07, sh[1] + 0.04, sh[2] + 0.22];
        let r = quat::nlerp(rot0, hand_rot(&fix_r, [0.0, 0.92, 0.38], [-1.0, 0.1, 0.0]), e);
        two_bone_ik(rig, d, fk.rot, fk.pos, role(UPPER_ARM_R), role(FOREARM_R), role(HAND_R), &v3::lerp3(live0, wrist, e), Some(&[-0.3, -1.0, 0.0]), e, Some(&r), false);
    });
    ask_mask(&[(role(SHOULDER_R), 1.0, true)]);
    if !ask_blend(g, at, at_b, smoothstep(0.0, 0.25, e), true, at, n) {
        return false;
    }
    true
}
