//! A reload, by the arms.
use super::*;

/// the gun's frame where the trigger hand has it now, off the run-forward bones: the wrist, the bore (wrist to the middle knuckle), up out of the receiver
/// and across to his left. The support hand works on the weapon as drawn.
struct GunFrame {
    wrist: [f64; 3],
    bore: [f64; 3],
    up: [f64; 3],
    left: [f64; 3],
}
impl GunFrame {
    fn of(fk: &Fk) -> GunFrame {
        let wrist = fk.at(role(HAND_R));
        let bore = normalize(sub3(fk.at(role(MIDDLE1_R)), wrist));
        let y = mad3(AY, bore, -bore[1]);
        let up = if len3(&y) > 1e-4 { normalize(y) } else { [0.0, 0.0, -1.0] };
        GunFrame { wrist, bore, up, left: cross3(&up, &bore) }
    }
    fn dir(&self, p: &[f64; 3]) -> [f64; 3] {
        let (x, y, d) = (&self.left, &self.up, &self.bore);
        [x[0] * p[0] + y[0] * p[1] + d[0] * p[2], x[1] * p[0] + y[1] * p[1] + d[1] * p[2], x[2] * p[0] + y[2] * p[1] + d[2] * p[2]]
    }
    fn at(&self, p: &[f64; 3]) -> [f64; 3] {
        add3(self.wrist, self.dir(p))
    }
}
/// CharacterAnimator.reloadArm (and its feedArm) — the long gun's magazine change, the support hand's: to the well, the empty stripped, a fresh one
/// fetched and seated, the handle run, back to the fore-end; an untrained man's lands on the lip, is backed off and found, and is slapped home. Or, for
/// a gun fed its own way, the hand through the stations it is handed. Solved on the scratch pose at `at_b` in the gun's frame as the trigger hand has
/// it, laid on the pose by w through the support arm's mask.
pub fn reload_arm(g: usize, an: &mut Anim, t: f64, w: f64, marks: &FeedMarks, keys: Option<&[FeedKey]>, tune: &Knobs) -> u32 {
    let (at, at_b, _) = poses(an);
    let (tr, hip_line, bt, fix_l) = (clamp(an.trained, 0.0, 1.0), an.long_gun, an.rl_b, an.hand_fix[0]);
    if keys.map_or(0, |k| k.len()) > KEYS_MAX {
        return 0;
    }
    let Some(Some(n)) = with_pose(g, at, |_d, _rig, n, _fk| if cast(n, &ARM_ROLES) && cast(n, &[MIDDLE1_R, HIPS, SPINE2, THIGH_L, THIGH_R]) { Some(n) } else { None }) else {
        return 0;
    };
    let Some(mut h) = hands_of(g, an) else {
        return 0;
    };
    if !ask_blend(g, at, at, 1.0, false, at_b, n) {
        return 0;
    }
    let hh = &mut h;
    let posed = with_pose(g, at_b, |d, rig, _n, fk| {
        fk_run(rig, d, fk.rot, fk.pos);
        let gun = GunFrame::of(fk);
        let hand_l = role(HAND_L);
        let (cur, rot0) = (fk.at(hand_l), fk.rot(hand_l));
        let (left, up, fwd) = trunk_frame(fk);
        let tk = |o: [f64; 3], p: [f64; 3]| mad3(mad3(mad3(o, left, p[0]), up, p[1]), fwd, p[2]);
        // the hip's pouches: off the pelvis in the trunk's frame, or by the hips' own line where a bladed chest would carry them round with it
        let hips = fk.at(role(HIPS));
        let (hl, hf) = if hip_line {
            let l = sub3(fk.at(role(THIGH_L)), fk.at(role(THIGH_R)));
            let hl = normalize(mad3(l, up, -dot3(&l, &up)));
            (hl, cross3(&hl, &up))
        } else {
            (left, fwd)
        };
        let hk = |p: [f64; 3]| mad3(mad3(mad3(hips, hl, p[0]), up, p[1]), hf, p[2]);
        let pouch = if tr > 0.5 { tk(fk.at(role(SPINE2)), MV2.reload.pouch[1]) } else { hk(MV2.reload.pouch[0]) };
        let (r_pole, pole_w) = (MV2.reload.pole, MV2.reload.pole_w);
        let (up_l, fore_l) = (role(UPPER_ARM_L), role(FOREARM_L));
        let (o, i, r, dur) = (bt.out, bt.in_, bt.rack, bt.dur);
        // the hand: round a thing from its front (fingers across it, palm toward him) · fingers down into the pouch
        let (mf, mb) = (gun.dir(&[-1.0, 0.15, 0.1]), gun.dir(&[0.1, -0.35, 1.0]));
        let (pf, pb) = (mad3(mad3(scale3(up, -0.9), left, -0.3), fwd, 0.1), mad3(fwd, left, 0.3));
        if let Some(keys) = keys {
            // a gun fed its own way: the hand through its stations, and back to where it was
            if keys.len() < 2 {
                return false;
            }
            // …or flat on top of the gun (fingers across it, the back of the hand up)
            let (tf, tb) = (gun.dir(&[-1.0, 0.0, 0.15]), gun.dir(&[0.1, 1.0, 0.1]));
            let (mut p, mut f, mut b) = (Path::of(&[(0.0, cur)]), Path::new(), Path::new());
            for key in keys {
                p.push(
                    key.t,
                    match key.at {
                        FeedKeyAt::Pouch => pouch,
                        FeedKeyAt::List(v) => gun.at(&v),
                    },
                );
                let (ff, bb) = match key.how {
                    FeedKeyHow::Top => (tf, tb),
                    FeedKeyHow::Pouch => (pf, pb),
                    FeedKeyHow::Mag => (mf, mb),
                };
                f.push(key.t, ff);
                b.push(key.t, bb);
            }
            // (the hand rests where its last work was until the weapon is nearly done, then back to the gun)
            let end = keys[keys.len() - 1].t;
            if end + 0.1 < dur - 0.3 {
                let last = p.k[p.n - 1].1;
                p.push(dur - 0.3, last);
            }
            p.push(max(dur - 0.03, end + 0.02), cur);
            let rot = hand_rot(&fix_l, f.at(t), b.at(t));
            two_bone_ik(rig, d, fk.rot, fk.pos, up_l, fore_l, hand_l, &p.at(t), Some(&r_pole), pole_w, Some(&rot), false);
            let (first, last) = (&keys[0], &keys[keys.len() - 1]);
            let mut gr = if t <= first.t { first.grip * smoothstep(first.t - 0.12, first.t, t) } else { last.grip * (1.0 - smoothstep(end, end + 0.12, t)) };
            for j in 0..keys.len() - 1 {
                let (ka, kb) = (&keys[j], &keys[j + 1]);
                if t > ka.t && t <= kb.t {
                    gr = lerp(ka.grip, kb.grip, (t - ka.t) / (kb.t - ka.t));
                    break;
                }
            }
            let c = |kk: f64| lerp(0.3, kk, gr);
            hand(d, hh, 0, &[c(0.62), c(0.68), c(0.72), c(0.78)], lerp(0.3, 0.6, gr), 0.1 * (1.0 - gr));
            wrist_ease(d, rig, fk, hand_l, &rot0, bump(t, 0.0, 0.12, dur - 0.26, dur - 0.05));
            return true;
        }
        let half = (o + i) * 0.5;
        let (mag, strip, hd, pl) = (gun.at(&marks.mag), gun.at(&marks.strip), gun.at(&marks.handle), gun.at(&marks.pull));
        // the empty comes straight down out of the well and is let go there; the fresh one comes up from under. An untrained man's lands on the lip beside
        // the well, is backed off, then found; seated, he slaps it home; and the handle let go, the hand flies on back.
        let wob = MV2.reload.fumble * (1.0 - tr);
        let fum = wob > 0.004;
        let hp = if fum { half - 0.1 } else { half };
        let slap = fum && bt.pull && r - i >= 0.4;
        let fl = if fum { (1.0 - tr) * max(bump(t, i - 0.36, i - 0.22, i - 0.09, i + 0.02), if slap { bump(t, i + 0.07, i + 0.15, i + 0.19, i + 0.27) } else { 0.0 }) } else { 0.0 };
        let hit = mad3(mad3(mad3(mag, gun.up, -0.035), gun.left, wob * 1.2), gun.bore, wob * 1.6);
        let off = if fum { tk(lerp3(&hit, &strip, 0.65), scale3(MV2.reload.flare.off, 1.0 - tr)) } else { hit };
        let tap_p = if slap { tk(mad3(mag, gun.up, -MV2.reload.tap * (1.0 - tr)), scale3(MV2.reload.flare.tap, 1.0 - tr)) } else { mag };
        let over = mad3(mad3(pl, gun.bore, -(MV2.reload.fling + 0.1 * (1.0 - tr))), gun.left, 0.04 + 0.05 * (1.0 - tr));
        // (a magazine with rounds in it goes to the dump pouch on the hip before the hand goes for a fresh one)
        let (kp, fr) = (bt.keep, bt.fresh);
        let dump = if kp >= 0.0 { hk(tune.kit.mag.dump) } else { pouch };
        let mut p = Path::of(&[(0.0, cur), (max(0.1, o - 0.2), mag), (o, mag), (o + 0.12, strip)]);
        if kp >= 0.0 {
            p.push((o + 0.05 + kp) * 0.5, hk(tune.kit.mag.via));
            p.push(kp - 0.07, dump);
            p.push(kp + 0.05, dump);
            p.push(fr, pouch);
            p.push(fr + 0.1, pouch);
        } else {
            p.push(hp, pouch);
            p.push(hp + 0.12, pouch);
        }
        if fum {
            p.push(max(hp + 0.13, i - 0.3), hit);
            p.push(i - 0.14, off);
        } else {
            p.push(i - 0.16, strip);
        }
        p.push(i, mag);
        p.push(i + 0.08, mag);
        if bt.pull {
            if fum && r - i >= 0.4 {
                p.push(i + 0.17, tap_p);
                p.push(i + 0.25, mag);
            }
            p.push(r - 0.1, hd);
            p.push(r, hd);
            p.push(r + 0.08, pl);
            p.push(r + 0.17, over);
        } else {
            p.push(i + 0.42, cur);
        }
        p.push(dur - 0.03, cur);
        // the hand: round the magazine from its front, fingers down into the pouch, hooked on the handle
        let (hf_d, hb_d) = (gun.dir(&marks.hf), gun.dir(&marks.hb));
        // (the hand is down at the pouches from the first of them to the last)
        let (hq, hr) = if kp >= 0.0 { (kp - 0.07, fr + 0.1) } else { (hp, hp + 0.12) };
        let lines = |a0: [f64; 3], a1: [f64; 3], a2: [f64; 3]| {
            let mut q = Path::of(&[(max(0.1, o - 0.2), a0), (o + 0.12, a0), (hq, a1), (hr, a1), (i - 0.24, a0), (i + 0.08, a0)]);
            if bt.pull {
                q.push(r - 0.1, a2);
                q.push(r + 0.16, a2);
            }
            q.at(t)
        };
        let rot = hand_rot(&fix_l, lines(mf, pf, hf_d), lines(mb, pb, hb_d));
        let pole = if fl > 0.0 { lerp3(&r_pole, &MV2.reload.flare.pole, fl) } else { r_pole };
        two_bone_ik(rig, d, fk.rot, fk.pos, up_l, fore_l, hand_l, &p.at(t), Some(&pole), pole_w, Some(&rot), false);
        let grip = if kp >= 0.0 { max(bump(t, o - 0.12, o - 0.02, kp - 0.02, kp + 0.06), bump(t, fr, fr + 0.08, i + 0.04, i + 0.12)) } else { bump(t, o - 0.12, o - 0.02, i + 0.04, i + 0.12) };
        let hook = if bt.pull { bump(t, r - 0.14, r - 0.04, r + 0.14, r + 0.22) } else { 0.0 };
        let c = |kk: f64| lerp(0.3, kk, max(grip, hook));
        hand(d, hh, 0, &[c(0.62), c(0.68), c(0.72), c(0.78)], lerp(0.3, 0.6, grip), 0.1 * (1.0 - grip));
        wrist_ease(d, rig, fk, hand_l, &rot0, bump(t, 0.0, 0.12, dur - 0.26, dur - 0.05));
        true
    });
    if posed == Some(true) {
        arm_mask(false);
        if !ask_blend(g, at, at_b, w, true, at, n) {
            return 0;
        }
        hands_to(&h, an);
    }
    RAN | RAN_ALL
}
/// A pistol's magazine change, the support hand's, over the reload clip's pose: to the butt as the magazine drops free, to the belt for a fresh one (a
/// magazine kept goes to the hip first), seated with the heel of the palm, and over the top to run the slide. The trigger hand, the gun and the body
/// stay the clip's.
pub fn pistol_reload_arm(g: usize, an: &mut Anim, t: f64, w: f64, tune: &Knobs) -> u32 {
    let (at, at_b, _) = poses(an);
    let (bt, fix_l) = (an.rl_b, an.hand_fix[0]);
    let Some(Some(n)) = with_pose(g, at, |_d, _rig, n, _fk| if cast(n, &ARM_ROLES) && cast(n, &[MIDDLE1_R, HIPS]) { Some(n) } else { None }) else {
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
        let gun = GunFrame::of(fk);
        let hand_l = role(HAND_L);
        let (cur, rot0) = (fk.at(hand_l), fk.rot(hand_l));
        let (left, up, fwd) = trunk_frame(fk);
        let hips = fk.at(role(HIPS));
        let tk = |p: [f64; 3]| mad3(mad3(mad3(hips, left, p[0]), up, p[1]), fwd, p[2]);
        let (pouch, dump) = (tk(tune.kit.mag.pouch), tk(tune.kit.mag.dump));
        let (o, i, r, dur, kp, fr) = (bt.out, bt.in_, bt.rack, bt.dur, bt.keep, bt.fresh);
        let (well, strip) = (gun.at(&tune.kit.mag.well), gun.at(&tune.kit.mag.strip));
        let (sl, pl) = (gun.at(&tune.kit.mag.slide), gun.at(&tune.kit.mag.pull));
        // (seated: the heel of the palm drives it the last centimetre)
        let seat = mad3(well, gun.up, 0.012);
        let mut p = Path::of(&[(0.0, cur), (max(0.08, o - 0.14), well), (o, well), (o + 0.1, strip)]);
        if kp >= 0.0 {
            p.push((o + 0.04 + kp) * 0.5, tk(tune.kit.mag.via));
            p.push(kp - 0.06, dump);
            p.push(kp + 0.05, dump);
        }
        p.push(fr, pouch);
        p.push(fr + 0.09, pouch);
        p.push(i - 0.13, strip);
        p.push(i, seat);
        p.push(i + 0.07, well);
        if bt.pull {
            p.push(r - 0.09, sl);
            p.push(r, sl);
            p.push(r + 0.07, pl);
            p.push(r + 0.15, mad3(pl, gun.up, 0.03));
        } else {
            p.push(i + 0.3, cur);
        }
        p.push(dur - 0.03, cur);
        let (wf, wb) = (gun.dir(&tune.kit.mag.wf), gun.dir(&tune.kit.mag.wb));
        let (sf, sb) = (gun.dir(&tune.kit.mag.sf), gun.dir(&tune.kit.mag.sb));
        let (pf, pb) = (mad3(mad3(scale3(up, -0.9), left, -0.25), fwd, 0.15), mad3(fwd, left, 0.4));
        let (hq, hr) = (if kp >= 0.0 { kp - 0.06 } else { fr }, fr + 0.09);
        let lines = |a0: [f64; 3], a1: [f64; 3], a2: [f64; 3]| {
            let mut q = Path::of(&[(max(0.08, o - 0.14), a0), (o + 0.1, a0), (hq, a1), (hr, a1), (i - 0.2, a0), (i + 0.07, a0)]);
            if bt.pull {
                q.push(r - 0.09, a2);
                q.push(r + 0.14, a2);
            }
            q.at(t)
        };
        let rot = hand_rot(&fix_l, lines(wf, pf, sf), lines(wb, pb, sb));
        two_bone_ik(rig, d, fk.rot, fk.pos, role(UPPER_ARM_L), role(FOREARM_L), hand_l, &p.at(t), Some(&tune.kit.mag.pole), tune.kit.mag.pole_w, Some(&rot), false);
        let grip = max(if kp >= 0.0 { bump(t, o - 0.08, o, kp - 0.02, kp + 0.06) } else { 0.0 }, bump(t, fr, fr + 0.08, i - 0.02, i + 0.06));
        let pinch = if bt.pull { bump(t, r - 0.1, r - 0.02, r + 0.08, r + 0.16) } else { 0.0 };
        let c = |kk: f64| lerp(0.25, kk, max(grip, pinch));
        hand(d, hh, 0, &[c(0.6), c(0.66), c(0.7), c(0.76)], lerp(0.3, 0.65, max(grip, pinch)), 0.1 * (1.0 - grip));
        wrist_ease(d, rig, fk, hand_l, &rot0, bump(t, 0.0, 0.1, dur - 0.22, dur - 0.05));
    });
    arm_mask(false);
    if !ask_blend(g, at, at_b, w, true, at, n) {
        return 0;
    }
    hands_to(&h, an);
    RAN | RAN_ALL
}
