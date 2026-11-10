//! The hammer and the breaching shotgun.
use super::*;

/// a unit direction turned k of the way to another
fn dir_mix(a: [f64; 3], b: [f64; 3], kk: f64) -> [f64; 3] {
    if kk <= 0.0 {
        a
    } else if kk >= 1.0 {
        b
    } else {
        normalize(q_rotate(&slerp(&[0.0, 0.0, 0.0, 1.0], &from_to(&a, &b), kk), &a))
    }
}
/// where a hand goes and how it is turned; a fist laid on the hammer's line also keeps the line's two axes through it (fingers, back)
#[derive(Clone, Copy)]
struct BigSpot {
    at: [f64; 3],
    rot: [f64; 4],
    f: [f64; 3],
    b: [f64; 3],
}
fn spot_mix(a: BigSpot, b: BigSpot, e: f64, bow: Option<[f64; 3]>) -> BigSpot {
    if e <= 0.0 {
        a
    } else if e >= 1.0 {
        b
    } else {
        let at = match bow {
            Some(bw) => mad3(lerp3(&a.at, &b.at, e), bw, sin(PI * e)),
            None => lerp3(&a.at, &b.at, e),
        };
        BigSpot { at, rot: slerp(&a.rot, &b.rot, e), f: [0.0; 3], b: [0.0; 3] }
    }
}
/// the hammer as its own line: the right fist's grip and the handle's direction, butt to head
#[derive(Clone, Copy)]
struct BigLine {
    g: [f64; 3],
    d: [f64; 3],
}
fn line_mix(a: BigLine, b: BigLine, kk: f64, bow: Option<[f64; 3]>) -> BigLine {
    if kk <= 0.0 {
        a
    } else if kk >= 1.0 {
        b
    } else {
        let g = match bow {
            Some(bw) => mad3(lerp3(&a.g, &b.g, kk), bw, sin(PI * kk)),
            None => lerp3(&a.g, &b.g, kk),
        };
        BigLine { g, d: dir_mix(a.d, b.d, kk) }
    }
}
/// CharacterAnimator.twisted: where a point posed below the chest ENDS UP after the chest twist still to come, and where spine2 itself does
struct Twisted {
    e1: [f64; 4],
    e2: [f64; 4],
    e3: [f64; 4],
    p1: [f64; 3],
    p2t: [f64; 3],
    p3t: [f64; 3],
}
impl Twisted {
    fn of(fk: &Fk, tw_y: f64, tw_p: f64) -> Twisted {
        let (e1, e2) = (q_yaw(tw_y * 0.25), q_yaw(tw_y * 0.35));
        let mut e3 = q_yaw(tw_y * 0.4);
        if !falsy(tw_p) {
            let right = q_rotate(&e3, &AX);
            e3 = q_mul(&axis_angle(&right, tw_p), &e3);
        }
        let (p1, p2, p3) = (fk.at(role(SPINE1)), fk.at(role(SPINE2)), fk.at(role(SPINE3)));
        let p2t = add3(q_rotate(&e1, &sub3(p2, p1)), p1);
        let p3t = add3(q_rotate(&e2, &sub3(add3(q_rotate(&e1, &sub3(p3, p1)), p1), p2t)), p2t);
        Twisted { e1, e2, e3, p1, p2t, p3t }
    }
    fn at(&self, h: [f64; 3]) -> [f64; 3] {
        let x1 = add3(q_rotate(&self.e1, &sub3(h, self.p1)), self.p1);
        let x2 = add3(q_rotate(&self.e2, &sub3(x1, self.p2t)), self.p2t);
        add3(q_rotate(&self.e3, &sub3(x2, self.p3t)), self.p3t)
    }
}
/// rigProps.ts bigBackOf for the hammer (BIGRIG.hammer.back): the point and the line across his back, off six joints in one space, `clear` m further off
fn big_back_of(hips: [f64; 3], mid: [f64; 3], th_l: [f64; 3], th_r: [f64; 3], ua_l: [f64; 3], ua_r: [f64; 3], clear: f64) -> ([f64; 3], [f64; 3]) {
    let mut up = sub3(lerp3(&ua_l, &ua_r, 0.5), hips);
    up = if len3(&up) > 1e-3 { normalize(up) } else { [0.0, 1.0, 0.0] };
    let mut left = lerp3(&normalize(sub3(th_l, th_r)), &normalize(sub3(ua_l, ua_r)), max(0.0, min(1.0, BIGRIG.hammer.back.follow)));
    left = mad3(left, up, -dot3(&left, &up));
    left = if len3(&left) > 1e-3 { normalize(left) } else { normalize(sub3(ua_l, ua_r)) };
    let cz = cross3(&left, &up);
    let in_c = |dv: [f64; 3]| mad3(mad3(scale3(left, dv[0]), up, dv[1]), cz, dv[2]);
    let (off, bore) = (BIGRIG.hammer.back.off, BIGRIG.hammer.back.bore);
    (add3(mid, in_c([off[0], off[1], off[2] - clear])), normalize(in_c(bore)))
}
/// The big slot's tool in his hands (the hammer or the breaching shotgun): the trunk and the feet for the blow or the shot straight into the pose, the
/// head back onto the point, then both arms by IK on a copy, whole from the upper arm down.
pub fn big_layer(g: usize, an: &mut Anim, b: &AnimBigTool, at_buf: usize, blows: &[f64], at0: [f64; 3], lg: &AnimLongGrip, tw_y: f64, tw_p: f64, tune: &Knobs) -> u32 {
    let (at, at_b, _) = poses(an);
    let (ham, t, equip, stow, end, snug, windup) = (b.kind == game_types::kit::BigTool::Hammer, b.t, b.equip, b.stow, b.end, b.snug.unwrap_or(false), b.windup);
    // (the shotgun is drawn as the carbine it is: its support grip, its group's offset and its stock's rear end)
    let (lg_wrist, lg_fing, lg_back, adj, stock_l0) = (lg.wrist, lg.fingers, lg.back, RIG.carbine.adj.map_or([0.0; 3], |a| a.off), RIG.carbine.stock_top.l0);
    let (speed, clear, fix_l, fix_r, gaze_local) = (an.speed, an.big_clear, an.hand_fix[0], an.hand_fix[1], an.gaze_local);
    let stowing = end >= 0.0 && t >= end;
    let us = if stowing { clamp((t - end) / max(1e-3, stow), 0.0, 1.0) } else { 0.0 };
    an.big_under = false;
    // ---- the beat as shares
    let (mut prev, mut next) = (f64::NEG_INFINITY, f64::INFINITY);
    for &b in blows {
        if b <= t {
            prev = max(prev, b);
        } else if !(end >= 0.0 && b > end) {
            next = min(next, b);
        }
    }
    let (mut wind, mut sw, mut hit, mut kick, mut plant) = (0.0, 0.0, 0.0, 0.0, 0.0);
    if ham {
        if next < f64::INFINITY {
            let ws = max(next - windup, prev);
            let dd = next - ws;
            if t >= ws && dd > 1e-3 {
                let s_w = min(clamp(BIGMOVE.hammer.swing * dd, BIGMOVE.hammer.swing_min, BIGMOVE.hammer.swing_max), dd * 0.6);
                wind = e3((t - ws) / max(1e-3, (dd - s_w) * BIGMOVE.hammer.raise));
                sw = clamp((t - (next - s_w)) / s_w, 0.0, 1.0);
            }
        }
        if prev > f64::NEG_INFINITY {
            hit = 1.0 - e3((t - prev) / BIGMOVE.hammer.rebound);
        }
        plant = max(wind, if prev > f64::NEG_INFINITY { if next < f64::INFINITY { 1.0 } else { 1.0 - e3((t - prev) / BIGMOVE.hammer.settle) } } else { 0.0 });
    } else if prev > f64::NEG_INFINITY {
        let r = t - prev;
        kick = if r < BIGMOVE.shotgun.kick_in { r / BIGMOVE.shotgun.kick_in } else { exp(-(r - BIGMOVE.shotgun.kick_in) / BIGMOVE.shotgun.kick_out) };
    }
    let (grab, put) = (BIGMOVE.grab, BIGMOVE.put);
    if stowing {
        let kq = 1.0 - e3(us / (put * 0.5));
        wind *= kq;
        sw *= kq;
        plant *= kq;
    }
    // (a blow asked for before the tool is out: it comes out with the wind-up)
    let ue = max(clamp(t / max(1e-3, equip), 0.0, 1.0), wind);
    let eb = e3((ue - grab) / (1.0 - grab));
    let in_hands = if stowing { min(eb, 1.0 - e3(us / put)) } else { eb };
    let es = pow(sw, 1.8);
    {
        let (t0, t1, put_at) = if snug { (BIGMOVE.snug.take[0], BIGMOVE.snug.take[1], BIGMOVE.snug.put_at) } else { (BIGMOVE.take[0], BIGMOVE.take[1], BIGMOVE.put_at) };
        an.big_k = smoothstep(t0, t1, ue) * (if stowing { 1.0 - smoothstep(put * put_at, put, us) } else { 1.0 });
    }
    const LEGS: [usize; 11] = [HIPS, SPINE1, SPINE2, NECK, HEAD, THIGH_L, SHIN_L, FOOT_L, THIGH_R, SHIN_R, FOOT_R];
    let Some(Some(n)) = with_pose(g, at, |_d, _rig, n, _fk| if cast(n, &ARM_ROLES) && cast(n, &LEGS) { Some(n) } else { None }) else {
        return 0;
    };
    let Some(mut hs) = hands_of(g, an) else {
        return 0;
    };
    if ham {
        if !ask_blend(g, at, at, 1.0, false, at_buf, n) {
            return 0;
        }
        an.big_under = true;
    }
    // ---- where the point is from where he stands, the trunk and the pelvis under it, the feet put back, the head onto the point
    let Some((tdir, tleft)) = with_pose(g, at, |d, rig, _n, fk| {
        fk_run(rig, d, fk.rot, fk.pos);
        let hips_n = role(HIPS);
        let hp = fk.at(hips_n);
        let (mut tx, mut tz) = (at0[0] - hp[0], at0[2] - hp[2]);
        let reach = hypot2(tx, tz);
        if reach > 1e-3 {
            tx /= reach;
            tz /= reach;
        } else {
            tx = 0.0;
            tz = 1.0;
        }
        let (tdir, tleft, psi) = ([tx, 0.0, tz], [tz, 0.0, -tx], clamp(atan2(tx, tz), -1.0, 1.0));
        // what squares the chest AS THE POSE UNDER IT HAS IT onto the point
        let turn_to = {
            let f = trunk_frame(fk).2;
            clamp(wrap_angle(psi - atan2(f[0], f[2])), -1.2, 1.2)
        };
        let still = 1.0 - smoothstep(0.2, 1.0, abs(speed));
        let (chest_yaw, pel_yaw, pitch, sh_f, sh_s, drop);
        if ham {
            let (a_w, a_i, far) = (wind * (1.0 - es), max(hit, es), smoothstep(BIGMOVE.hammer.far[0], BIGMOVE.hammer.far[1], reach));
            chest_yaw = plant * turn_to * BIGMOVE.hammer.face + (a_w * BIGMOVE.hammer.wound_body.yaw + a_i * BIGMOVE.hammer.blow_body.yaw) * DEGR;
            pel_yaw = (plant * turn_to * BIGMOVE.hammer.face * 0.4 + (a_w * BIGMOVE.hammer.wound_body.pel_yaw + a_i * BIGMOVE.hammer.blow_body.pel_yaw) * DEGR) * still;
            pitch = (a_w * BIGMOVE.hammer.wound_body.pitch + a_i * (BIGMOVE.hammer.blow_body.pitch + far * BIGMOVE.hammer.blow_body.pitch_far)) * DEGR;
            sh_f = (a_i * (BIGMOVE.hammer.blow_body.fwd + far * BIGMOVE.hammer.blow_body.fwd_far) - a_w * BIGMOVE.hammer.wound_body.back) * still;
            sh_s = a_w * BIGMOVE.hammer.wound_body.side * still;
            drop = (plant * BIGMOVE.hammer.drop + a_i * BIGMOVE.hammer.blow_body.drop) * still;
        } else {
            chest_yaw = in_hands * (turn_to + BIGMOVE.shotgun.yaw * DEGR);
            pel_yaw = in_hands * (turn_to * 0.3 + BIGMOVE.shotgun.pel_yaw * DEGR) * still;
            pitch = (in_hands * BIGMOVE.shotgun.pitch + kick * BIGMOVE.shotgun.kick_pitch) * DEGR;
            sh_f = -BIGMOVE.shotgun.shove * kick * still;
            sh_s = 0.0;
            drop = 0.0;
        }
        if abs(chest_yaw) + abs(pel_yaw) + abs(pitch) + abs(sh_f) + abs(sh_s) + drop > 1e-5 {
            let (fl_n, fr_n) = (role(FOOT_L), role(FOOT_R));
            let (mut f_l, mut f_r, r_l, r_r) = (fk.at(fl_n), fk.at(fr_n), fk.rot(fl_n), fk.rot(fr_n));
            let ax_t = cross3(&AY, &tdir);
            // (the pelvis turn carries the whole spine; the spine joints add the rest)
            let seg = |kk: f64| q_mul(&q_yaw((chest_yaw - pel_yaw) * kk), &axis_angle(&ax_t, pitch * kk));
            shift(d, rig, hips_n, &[tdir[0] * sh_f + tleft[0] * sh_s, -drop, tdir[2] * sh_f + tleft[2] * sh_s]);
            turn(d, rig, hips_n, q_yaw(pel_yaw));
            turn(d, rig, role(SPINE1), seg(0.3));
            turn(d, rig, role(SPINE2), seg(0.35));
            turn(d, rig, role(SPINE3), seg(0.35));
            // the head: back out of most of it (the look below puts the eyes on the point)
            let hq = |kk: f64| q_mul(&q_yaw(-chest_yaw * 0.8 * kk), &axis_angle(&ax_t, -pitch * 0.8 * kk));
            turn(d, rig, role(NECK), hq(0.4));
            turn(d, rig, role(HEAD), hq(0.6));
            if ham && plant > 0.0 {
                // a wider stance for the blow, the left foot toward the point: stepped out to, not slid
                let (kq, up) = (plant * still, BIGMOVE.hammer.stance.lift * sin(PI * plant) * still);
                f_l[0] += (tleft[0] * BIGMOVE.hammer.stance.l[0] + tdir[0] * BIGMOVE.hammer.stance.l[1]) * kq;
                f_l[2] += (tleft[2] * BIGMOVE.hammer.stance.l[0] + tdir[2] * BIGMOVE.hammer.stance.l[1]) * kq;
                f_l[1] += up;
                f_r[0] += (tleft[0] * BIGMOVE.hammer.stance.r[0] + tdir[0] * BIGMOVE.hammer.stance.r[1]) * kq;
                f_r[2] += (tleft[2] * BIGMOVE.hammer.stance.r[0] + tdir[2] * BIGMOVE.hammer.stance.r[1]) * kq;
                f_r[1] += up;
            }
            // feet back where the locomotion has them, soles as they were
            fk_run(rig, d, fk.rot, fk.pos);
            two_bone_ik(rig, d, fk.rot, fk.pos, role(THIGH_L), role(SHIN_L), fl_n, &f_l, Some(&knee_pole(&r_l)), 0.6, Some(&r_l), false);
            two_bone_ik(rig, d, fk.rot, fk.pos, role(THIGH_R), role(SHIN_R), fr_n, &f_r, Some(&knee_pole(&r_r)), 0.6, Some(&r_r), false);
        }
        let look_w = if ham { plant } else { in_hands };
        if look_w > 0.003 {
            fk_run(rig, d, fk.rot, fk.pos);
            let tg = untwist(fk, tw_y, tw_p, at0);
            look_at(d, rig, fk, &gaze_local, tg, 0.8 * look_w, 0.0);
        }
        (tdir, tleft)
    }) else {
        return 0;
    };
    // ---- the arms, by IK on a copy against the body as it now stands
    if !ask_blend(g, at, at, 1.0, false, at_b, n) {
        return 0;
    }
    let hh = &mut hs;
    let (at_back, grab_at) = (&mut an.big_at_back, &mut an.big_grab);
    let done = with_pose(g, at_b, |d, rig, _n, fk| {
        fk_run(rig, d, fk.rot, fk.pos);
        let (f_left, f_up, f_fwd) = trunk_frame(fk);
        let dir_t = |q: [f64; 3]| -> [f64; 3] {
            [f_left[0] * q[0] + f_up[0] * q[1] + f_fwd[0] * q[2], f_left[1] * q[0] + f_up[1] * q[1] + f_fwd[1] * q[2], f_left[2] * q[0] + f_up[2] * q[1] + f_fwd[2] * q[2]]
        };
        let (hr_n, hl_n) = (role(HAND_R), role(HAND_L));
        let (s_r, s_l) = (fk.at(role(UPPER_ARM_R)), fk.at(role(UPPER_ARM_L)));
        let hp_at = untwist(fk, tw_y, tw_p, at0);
        let arc = dir_t(BIGMOVE.arc);
        let (u_r, u_l) = (BigSpot { at: fk.at(hr_n), rot: fk.rot(hr_n), f: [0.0; 3], b: [0.0; 3] }, BigSpot { at: fk.at(hl_n), rot: fk.rot(hl_n), f: [0.0; 3], b: [0.0; 3] });
        // a fist on the line at g: the handle through it, thumb toward the head; the knuckles lead away from that shoulder, squared to the handle
        let fist = |side: usize, gp: [f64; 3], dv: [f64; 3], sh: [f64; 3]| -> BigSpot {
            let mut h = add3(sub3(gp, sh), dir_t([if side == 1 { -0.03 } else { 0.03 }, 0.0, 0.02]));
            h = mad3(h, dv, -dot3(&h, &dv));
            if len3(&h) < 1e-3 {
                h = cross3(&dv, &f_up);
            }
            let f = normalize(h);
            let b = if side == 1 { cross3(&f, &dv) } else { cross3(&dv, &f) };
            BigSpot { at: mad3(mad3(gp, f, -BIGRIG.hammer.grip.fwd), b, BIGRIG.hammer.grip.palm), rot: hand_rot(if side == 1 { &fix_r } else { &fix_l }, f, b), f, b }
        };
        let (back_r, held): (BigSpot, BigSpot);
        // (the left hand's place on the tool, by the kind: the hammer's line, or the gun's three axes)
        let mut ham_line = BigLine { g: [0.0; 3], d: [0.0; 3] };
        let mut gun = ([0.0f64; 3], [0.0f64; 3], [0.0f64; 3]);
        if ham {
            let off = |gp: [f64; 3], dv: [f64; 3]| BigLine { g: add3(s_r, dir_t(gp)), d: normalize(dir_t(dv)) };
            // on his back, as the drawer has it: the fist takes it just under the head (snug: the back's frame as it WILL be after the chest twist still
            // to come, and the place that ENDS UP there)
            let tw = snug && (!falsy(tw_y) || !falsy(tw_p));
            let (ua_l, ua_r) = (fk.at(role(UPPER_ARM_L)), fk.at(role(UPPER_ARM_R)));
            let (hips, th_l, th_r) = (fk.at(role(HIPS)), fk.at(role(THIGH_L)), fk.at(role(THIGH_R)));
            let (bo, bz) = if tw {
                let twd = Twisted::of(fk, tw_y, tw_p);
                let (o, z) = big_back_of(hips, twd.p2t, th_l, th_r, twd.at(ua_l), twd.at(ua_r), clear);
                (untwist(fk, tw_y, tw_p, o), untwist_dir(tw_y, tw_p, z))
            } else {
                big_back_of(hips, fk.at(role(SPINE2)), th_l, th_r, ua_l, ua_r, clear)
            };
            let head_at = BIGRIG.hammer.head_at;
            let back_l = BigLine { g: mad3(bo, bz, head_at - BIGMOVE.hammer.grab_at), d: bz };
            let (ready_l, wound_l, over_l) = (off(BIGMOVE.hammer.ready.g, BIGMOVE.hammer.ready.d), off(BIGMOVE.hammer.wound.g, BIGMOVE.hammer.wound.d), off(BIGMOVE.hammer.over.g, BIGMOVE.hammer.over.d));
            // the blow: the head ON the point, the butt end by the pelvis on the point's own side
            let butt = BIGMOVE.hammer.butt;
            let b0 = [hips[0] + tleft[0] * butt[0] + tdir[0] * butt[2], hips[1] + butt[1], hips[2] + tleft[2] * butt[0] + tdir[2] * butt[2]];
            let di = normalize(sub3(hp_at, b0));
            let blow_l = BigLine { g: mad3(hp_at, di, -head_at), d: di };
            let mut u = line_mix(back_l, ready_l, eb, Some(arc));
            if hit > 0.0 {
                u = line_mix(u, blow_l, hit, None);
            }
            if stowing {
                u = line_mix(u, back_l, e3(us / put), Some(arc));
            }
            let mut l = line_mix(u, wound_l, wind, None);
            if sw > 0.0 {
                // over the top and down into it
                l = line_mix(line_mix(l, over_l, es, None), line_mix(over_l, blow_l, es, None), es, None);
            }
            back_r = fist(1, back_l.g, back_l.d, s_r);
            held = fist(1, l.g, l.d, s_r);
            // (how much of the fist's place this frame is the tool's place ON HIS BACK, and how far up the handle it holds it)
            {
                let mut wb = (1.0 - eb) * (1.0 - hit);
                if stowing {
                    wb += (1.0 - wb) * e3(us / put);
                }
                wb *= (1.0 - wind) * (if sw > 0.0 { (1.0 - es) * (1.0 - es) } else { 1.0 });
                *at_back = if ue < grab { e3(ue / grab) } else if stowing && us > put { 1.0 - e3((us - put) / (1.0 - put)) } else { wb };
                *grab_at = head_at - BIGMOVE.hammer.grab_at;
            }
            ham_line = l;
        } else {
            // (the rifle's own place over the shoulder: the fist there, the muzzle down his back)
            back_r = BigSpot { at: add3(s_r, dir_t(tune.kit.apex.at)), rot: hand_rot(&fix_r, dir_t(tune.kit.apex.fing), dir_t(tune.kit.apex.back)), f: [0.0; 3], b: [0.0; 3] };
            let pk = add3(s_r, dir_t(BIGMOVE.shotgun.pocket));
            let mut z = normalize(sub3(hp_at, pk));
            let mut yu = mad3(AY, z, -z[1]);
            yu = if len3(&yu) > 1e-3 { normalize(yu) } else { f_fwd };
            if kick > 0.0 {
                let an = kick * BIGMOVE.shotgun.climb * DEGR;
                let (cs, sn) = (cos(an), sin(an));
                let z1 = add3(scale3(z, cs), scale3(yu, sn));
                yu = sub3(scale3(yu, cs), scale3(z, sn));
                z = z1;
            }
            let x = cross3(&yu, &z);
            gun = (x, yu, z);
            let in_g = |p: [f64; 3]| -> [f64; 3] { [x[0] * p[0] + yu[0] * p[1] + z[0] * p[2], x[1] * p[0] + yu[1] * p[1] + z[1] * p[2], x[2] * p[0] + yu[2] * p[1] + z[2] * p[2]] };
            // the trigger wrist that puts the butt (the stock's rear end, through the group's own offset) in the pocket
            let wr0 = add3(pk, in_g([-adj[0], -adj[1], -stock_l0 - adj[2] - kick * BIGMOVE.shotgun.kick]));
            let up0 = BigSpot { at: wr0, rot: hand_rot(&fix_r, z, in_g([-0.8, 0.6, 0.0])), f: [0.0; 3], b: [0.0; 3] };
            held = spot_mix(back_r, up0, in_hands, Some(arc));
        }
        let r = if ue < grab { spot_mix(u_r, back_r, e3(ue / grab), Some(arc)) } else if stowing && us > put { spot_mix(back_r, u_r, e3((us - put) / (1.0 - put)), Some(arc)) } else { held };
        // how much of the elbows is the layer's: none at either end (the pose's own under it)
        let own = e3(ue / 0.2) * (if stowing { 1.0 - e3((us - put) / (1.0 - put)) } else { 1.0 });
        // a hand up by the shoulder: the elbow out and up, not folded into the ribs
        let high = smoothstep(-0.18, 0.1, r.at[1] - s_r[1]);
        two_bone_ik(rig, d, fk.rot, fk.pos, role(UPPER_ARM_R), role(FOREARM_R), hr_n, &r.at, Some(&dir_t(lerp3(&[-0.5, -0.8, -0.3], &[-1.0, 0.3, -0.1], high))), 0.85 * own, Some(&r.rot), false);
        let j = smoothstep(BIGMOVE.join[0], BIGMOVE.join[1], ue) * (if stowing { 1.0 - smoothstep(0.0, BIGMOVE.unjoin, us) } else { 1.0 });
        // let go of his gun, waiting in front of him
        let mut l = spot_mix(u_l, BigSpot { at: add3(s_l, dir_t(BIGMOVE.wait)), rot: hand_rot(&fix_l, dir_t([-0.25, -0.35, 0.9]), dir_t([0.35, 0.9, 0.25])), f: [0.0; 3], b: [0.0; 3] }, e3(ue / 0.3), None);
        if j > 0.0 {
            let wr = fk.at(hr_n);
            let on_it = if ham {
                let gp = mad3(mad3(wr, held.f, BIGRIG.hammer.grip.fwd), held.b, -BIGRIG.hammer.grip.palm);
                fist(0, mad3(gp, ham_line.d, -BIGRIG.hammer.hands), ham_line.d, s_l)
            } else {
                let (x, yu, z) = gun;
                let in_g = |p: [f64; 3]| -> [f64; 3] { [x[0] * p[0] + yu[0] * p[1] + z[0] * p[2], x[1] * p[0] + yu[1] * p[1] + z[1] * p[2], x[2] * p[0] + yu[2] * p[1] + z[2] * p[2]] };
                BigSpot { at: add3(wr, in_g(lg_wrist)), rot: hand_rot(&fix_l, in_g(lg_fing), in_g(lg_back)), f: [0.0; 3], b: [0.0; 3] }
            };
            l = spot_mix(l, on_it, e3(j), None);
        }
        if stowing {
            l = spot_mix(l, u_l, e3((us - BIGMOVE.unjoin * 0.5) / (0.92 - BIGMOVE.unjoin * 0.5)), None);
        }
        two_bone_ik(rig, d, fk.rot, fk.pos, role(UPPER_ARM_L), role(FOREARM_L), hl_n, &l.at, Some(&dir_t([0.5, -0.85, -0.15])), 0.6 * own, Some(&l.rot), false);
        {
            let gr = smoothstep(grab - 0.15, grab, ue) * (if stowing { 1.0 - smoothstep(put, put + 0.12, us) } else { 1.0 });
            let (open, shut) = ([BIGMOVE.open[0], BIGMOVE.open[1], BIGMOVE.open[2], BIGMOVE.open[3]], [BIGMOVE.fist[0], BIGMOVE.fist[1], BIGMOVE.fist[2], BIGMOVE.fist[3]]);
            let cu = |kk: f64| [lerp(open[0], shut[0], kk), lerp(open[1], shut[1], kk), lerp(open[2], shut[2], kk), lerp(open[3], shut[3], kk)];
            hand(d, hh, 1, &cu(gr), lerp(0.25, 0.6, gr), 0.0);
            hand(d, hh, 0, &cu(j), lerp(0.25, 0.6, j), 0.0);
        }
    });
    if done.is_none() {
        return 0;
    }
    arm_mask(true);
    if !ask_blend(g, at, at_b, 1.0, true, at, n) {
        return 0;
    }
    hands_to(&hs, an);
    RAN | RAN_ALL
}
