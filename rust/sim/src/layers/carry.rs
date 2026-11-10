//! What the arms do with a gun or a torch in them.
use super::*;

/// Both arms onto the pistol: the gun hand at the chest + the stance's [side, up, fwd], the muzzle at its pitch and level in roll, the support hand
/// keeping the exact hold it has in the aim clip (both hands moved by one rigid transform of the clip's grip); elbows swivelled down and in by the tuck;
/// a little sway, and during a kick a pump of the gun toward the chest. Solved on a copy seeded with the aim clip's arms (`at_aim`).
pub fn carry_arms(g: usize, an: &Anim, w: f64, kick_t: f64) -> u32 {
    let (at, at_b, _) = poses(an);
    let (at_aim, t, st, grip_up) = (an.p_aim.at as usize, an.idle_t, &an.st, an.grip_up);
    let (st_fwd, st_up, st_side, st_pitch, st_blade, tuck) = (st.fwd.x, st.up.x, st.side.x, st.pitch.x, st.blade.x, st.tuck.x);
    let Some(Some(n)) = with_pose(g, at, |_d, _rig, n, _fk| if cast(n, &ARM_ROLES) && cast(n, &[MIDDLE1_R]) { Some(n) } else { None }) else {
        return 0;
    };
    // the body as it is, the aim clip's arms
    arms_mask();
    if !ask_blend(g, at, at, 1.0, false, at_b, n) || !ask_blend(g, at_b, at_aim, 1.0, true, at_b, n) {
        return 0;
    }
    let code = with_pose(g, at_b, |d, rig, _n, fk| {
        fk.part(rig, d, list(FK_ARMS));
        let (hand_r, hand_l) = (role(HAND_R), role(HAND_L));
        let (chest, pr0, pl0, rr0) = (fk.at(role(SPINE3)), fk.at(hand_r), fk.at(hand_l), fk.rot(hand_r));
        // the clip's bore axis as posed on this spine
        let g0 = normalize(sub3(fk.at(role(MIDDLE1_R)), pr0));
        // where the gun goes: sway (a slow figure of eight, millimetres) and the kick's pump
        let pump = if kick_t >= 0.0 { 0.05 * bump(kick_t, 0.15, 0.36, 0.5, 0.85) } else { 0.0 };
        let (fwd, up, side) = (st_fwd - pump, st_up + 0.004 * sin(t * 2.1) + 0.02 * pump, st_side + 0.005 * sin(t * 1.3 + 0.6));
        let (pitch, yaw) = ((st_pitch + 0.8 * sin(t * 1.7 + 1.1)) * DEGR + pump * 4.0, 0.6 * sin(t * 1.1) * DEGR + st_blade * DEGR * 0.2);
        let gd = [sin(yaw) * cos(pitch), sin(pitch), cos(yaw) * cos(pitch)];
        let turn = grip_turn(&g0, &rr0, &gd, &AY, &grip_up);
        let pr = [chest[0] + side, chest[1] + up, chest[2] + fwd];
        // the support wrist relative to the gun wrist, turned with the gun
        let grip = q_rotate(&turn, &sub3(pl0, pr0));
        two_bone_ik(rig, d, fk.rot, fk.pos, role(UPPER_ARM_R), role(FOREARM_R), hand_r, &pr, Some(&[-0.12 - 0.6 * (1.0 - tuck), -1.0, -0.5]), 0.4 + 0.55 * tuck, Some(&q_mul(&turn, &rr0)), false);
        // (from where the gun hand actually got to: the hands must not part; the support wrist's rotation read as the solve left the bones)
        let (pl, rl0) = (add3(fk.at(hand_r), grip), fk.rot(hand_l));
        two_bone_ik(rig, d, fk.rot, fk.pos, role(UPPER_ARM_L), role(FOREARM_L), hand_l, &pl, Some(&[0.2 + 0.6 * (1.0 - tuck), -1.0, -0.35]), 0.4 + 0.5 * tuck, Some(&q_mul(&turn, &rl0)), true);
        RAN | RAN_ALL
    })
    .unwrap_or(0);
    if code == 0 {
        return 0;
    }
    arms_mask();
    if !ask_blend(g, at, at_b, w, true, at, n) {
        return 0;
    }
    code
}
/// LC of character.ts: the long gun's channels, in the order the animator keeps them
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
/// The long gun's arms: the trigger hand's wrist at the carriage's point off the chest joint (or, by its anchor, off the gun shoulder), swung about the
/// shoulders with the aim's share of the pitch, holding the grip as the aim clip holds the pistol's turned onto the wanted bore; the support hand solved
/// onto the gun's grip point from where the trigger hand got to. Should the support arm fall short the whole gun is brought back by the shortfall
/// (capped) and both re-solved, and that hold-back is kept and let out over half a second. Solved on a copy seeded with the aim clip's arms.
pub fn long_arms(g: usize, an: &mut Anim, w: f64, kick_t: f64, dt: f64, yield_door: bool, keep: bool, ch: &[f64; LC_COUNT], rl_at: [f64; 3], tune: &Knobs) -> u32 {
    let (at, at_b, _) = poses(an);
    let at_aim = an.p_aim.at as usize;
    let (t, aim, aim_pitch, long_steady, trained, rl_k) = (an.idle_t, an.upper_w.aim, an.aim_pitch, an.long_steady, an.trained, an.rlc.k);
    let (mvs_yaw, mvs_climb, mvs_back, dl, dr) = (an.mvs.yaw, an.mvs.climb, an.mvs.back, an.dp.w_l.x, an.dp.w_r.x);
    let (grip_up, fix_l, g_wrist, g_fingers, g_back) = (an.grip_up, an.hand_fix[0], an.long_grip.wrist, an.long_grip.fingers, an.long_grip.back);
    let Some(Some(n)) = with_pose(g, at, |_d, _rig, n, _fk| if cast(n, &ARM_ROLES) && cast(n, &[MIDDLE1_R]) { Some(n) } else { None }) else {
        return 0;
    };
    let Some(mut h) = hands_of(g, an) else {
        return 0;
    };
    // the body as it is, the aim clip's clavicles, trigger grip and fingers to start from
    arms_mask();
    if !ask_blend(g, at, at, 1.0, false, at_b, n) || !ask_blend(g, at_b, at_aim, 1.0, true, at_b, n) {
        return 0;
    }
    let hh = &mut h;
    let code = with_pose(g, at_b, |d, rig, _n, fk| {
        fk.part(rig, d, list(FK_ARMS));
        let (hand_r, hand_l) = (role(HAND_R), role(HAND_L));
        let (chest, pr0, rr0) = (fk.at(role(SPINE3)), fk.at(hand_r), fk.rot(hand_r));
        // the clip's bore as posed on this spine
        let g0 = normalize(sub3(fk.at(role(MIDDLE1_R)), pr0));
        // the bore, model space
        let sway = ch[LC_SWAY];
        let yaw = (ch[LC_YAW] + 0.6 * sway * sin(t * 1.1) + mvs_yaw) * DEGR;
        let pitch = clamp((ch[LC_PITCH] + 0.8 * sway * sin(t * 1.7 + 1.1) + mvs_climb) * DEGR + aim_pitch * (ch[LC_AIM_K] - 0.35 * aim), -1.35, 1.35);
        let cp = cos(pitch);
        let dd = [sin(yaw) * cp, sin(pitch), cos(yaw) * cp];
        let rr = q_mul(&grip_turn(&g0, &rr0, &dd, &AY, &grip_up), &rr0);
        // the gun's own frame about that bore: y up out of the receiver, x to the figure's left
        let mut yw = mad3(AY, dd, -dd[1]);
        yw = if len3(&yw) > 1e-4 { normalize(yw) } else { [cos(yaw), 0.0, -sin(yaw)] };
        let xw = cross3(&yw, &dd);
        let in_gun = |p: &[f64; 3]| -> [f64; 3] { [xw[0] * p[0] + yw[0] * p[1] + dd[0] * p[2], xw[1] * p[0] + yw[1] * p[1] + dd[1] * p[2], xw[2] * p[0] + yw[2] * p[1] + dd[2] * p[2]] };
        // the trigger hand: the carriage's point off the chest joint, pumped toward the chest by a kick
        let pump = if kick_t >= 0.0 { 0.04 * bump(kick_t, 0.15, 0.36, 0.5, 0.85) } else { 0.0 };
        let ka = clamp(ch[LC_ANCHOR], 0.0, 1.0);
        let sh = fk.at(role(UPPER_ARM_R));
        let mut anchor = [lerp(chest[0], sh[0], ka), lerp(chest[1], sh[1], ka), lerp(chest[2], sh[2], ka)];
        anchor[0] = max(anchor[0], chest[0] - LONG_ANCHOR_SIDE_MAX);
        // steadied against the gait: the anchor's side-to-side followed through a two-pole low-pass
        {
            let (mut l0, mut l1) = (an.lg.lat[0], an.lg.lat[1]);
            if !finite(l0) || !finite(l1) {
                l0 = anchor[0];
                l1 = anchor[0];
            } else {
                let kk = 1.0 - exp(-dt / max(1e-3, LONG_STEADY.tau));
                l0 += (anchor[0] - l0) * kk;
                l1 += (l0 - l1) * kk;
            }
            if keep {
                an.lg.lat[0] = l0;
                an.lg.lat[1] = l1;
            }
            let iso = clamp(ch[LC_STEADY], 0.0, 1.0) * clamp(long_steady, 0.0, 1.0);
            if iso > 1e-3 {
                anchor[0] = lerp(anchor[0], l1, iso);
            }
        }
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
        let mut pr = add3(chest, off);
        if mvs_back > 1e-5 {
            pr = mad3(pr, dd, -mvs_back);
        }
        let mut pole_r = [ch[LC_ELBOW_R], ch[LC_ELBOW_R + 1], ch[LC_ELBOW_R + 2]];
        let mut pole_l = [ch[LC_ELBOW_L], ch[LC_ELBOW_L + 1], ch[LC_ELBOW_L + 2]];
        let pw = clamp(ch[LC_POLE_W], 0.0, 1.0);
        // the elbows by training, shouldered
        let (tw, uw) = (smoothstep(0.5, 1.0, trained) * aim, (1.0 - smoothstep(0.0, 0.5, trained)) * aim);
        pole_r[0] += tune.mv.gait.elbow_in * tw;
        pole_l[0] -= tune.mv.gait.elbow_in * 0.8 * tw;
        pole_r[1] += tune.mv.gait.wing * uw;
        pole_r[0] -= 0.25 * uw;
        let rl = hand_rot(&fix_l, in_gun(&g_fingers), in_gun(&g_back));
        let (up_r, fore_r, up_l, fore_l) = (role(UPPER_ARM_R), role(FOREARM_R), role(UPPER_ARM_L), role(FOREARM_L));
        // → how far the support hand fell short of its point (nought when it reached)
        let solve = |d: &mut [S], fk: &mut Fk, to: [f64; 3]| -> [f64; 3] {
            two_bone_ik(rig, d, fk.rot, fk.pos, up_r, fore_r, hand_r, &to, Some(&pole_r), pw, Some(&rr), false);
            // from where the trigger hand actually got to
            let pl = add3(fk.at(hand_r), in_gun(&g_wrist));
            two_bone_ik(rig, d, fk.rot, fk.pos, up_l, fore_l, hand_l, &pl, Some(&pole_l), pw, Some(&rl), true);
            sub3(fk.at(hand_l), pl)
        };
        // out of the support arm's reach: the gun comes back to where it can be held, at once, and is let out again over half a second
        let mut pull = [an.lg.pull[0], an.lg.pull[1], an.lg.pull[2]];
        let m = solve(d, fk, add3(pr, pull));
        if len3(&m) > 5e-4 {
            // still short with what is held: take up the rest now (capped), and pose there
            pull = [pull[0] + m[0], pull[1] + m[1], pull[2] + m[2]];
            let l = len3(&pull);
            if l > LONG_PULL_MAX {
                let kk = LONG_PULL_MAX / l;
                pull = [pull[0] * kk, pull[1] * kk, pull[2] * kk];
            }
            solve(d, fk, add3(pr, pull));
        } else if keep {
            // reached: let a little out for next frame
            let kk = exp(-2.0 * dt);
            pull = [pull[0] * kk, pull[1] * kk, pull[2] * kk];
        }
        if keep {
            an.lg.pull[0] = pull[0];
            an.lg.pull[1] = pull[1];
            an.lg.pull[2] = pull[2];
        }
        // the support hand cupped round the handguard
        hand(d, hh, 0, &[0.5, 0.55, 0.6, 0.68], 0.35, 0.0);
        RAN | RAN_ALL
    })
    .unwrap_or(0);
    if code == 0 {
        return 0;
    }
    if yield_door {
        // arm by arm, short of whatever a hand on a door still holds
        let (wl, wr) = (w * (1.0 - clamp(dl, 0.0, 1.0)), w * (1.0 - clamp(dr, 0.0, 1.0)));
        ask_mask(&[(role(SHOULDER_L), 1.0, true)]);
        if !ask_blend(g, at, at_b, wl, true, at, n) {
            return 0;
        }
        ask_mask(&[(role(SHOULDER_R), 1.0, true)]);
        if !ask_blend(g, at, at_b, wr, true, at, n) {
            return 0;
        }
    } else {
        arms_mask();
        if !ask_blend(g, at, at_b, w, true, at, n) {
            return 0;
        }
    }
    hands_to(&h, an);
    code
}
/// The flashlight brought up in the gun hand: the right wrist to a spot ahead of the shoulder, the elbow down and a little out, by e. Solved on the
/// scratch pose at `at_b`, laid on the right arm.
pub fn torch_up(g: usize, an: &Anim, e: f64) -> u32 {
    let (at, at_b, _) = poses(an);
    let fix_r = an.hand_fix[1];
    let Some(Some(n)) = with_pose(g, at, |_d, _rig, n, _fk| if cast(n, &ARM_ROLES) { Some(n) } else { None }) else {
        return 0;
    };
    if !ask_blend(g, at, at, 1.0, false, at_b, n) {
        return 0;
    }
    with_pose(g, at_b, |d, rig, _n, fk| {
        fk_run(rig, d, fk.rot, fk.pos);
        let (sh, live0, rot0) = (fk.at(role(UPPER_ARM_R)), fk.at(role(HAND_R)), fk.rot(role(HAND_R)));
        let wrist = [sh[0] - 0.07, sh[1] + 0.04, sh[2] + 0.22];
        let r = nlerp(&rot0, &hand_rot(&fix_r, [0.0, 0.92, 0.38], [-1.0, 0.1, 0.0]), e);
        two_bone_ik(rig, d, fk.rot, fk.pos, role(UPPER_ARM_R), role(FOREARM_R), role(HAND_R), &lerp3(&live0, &wrist, e), Some(&[-0.3, -1.0, 0.0]), e, Some(&r), false);
    });
    ask_mask(&[(role(SHOULDER_R), 1.0, true)]);
    if !ask_blend(g, at, at_b, smoothstep(0.0, 0.25, e), true, at, n) {
        return 0;
    }
    RAN | RAN_ALL
}
