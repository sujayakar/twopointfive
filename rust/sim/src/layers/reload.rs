//! A reload, by the arms.
use super::*;

/// the gun's frame where the trigger hand has it now, off the run-forward bones: the wrist, the bore (wrist to the middle knuckle), up out of the receiver
/// and across to his left. The support hand works on the weapon as drawn.
#[derive(Clone, Copy, Default)]
struct GunFrame {
    wrist: [f64; 3],
    bore: [f64; 3],
    up: [f64; 3],
    left: [f64; 3],
}
impl GunFrame {
    fn of(rig: &Rig, fk: &Fk) -> GunFrame {
        let wrist = fk.at(rig.role(HAND_R));
        let bore = v3::normalize(v3::sub(fk.at(rig.role(MIDDLE1_R)), wrist));
        let y = v3::mad(AY, bore, -bore[1]);
        let up = if v3::len(y) > 1e-4 { v3::normalize(y) } else { [0.0, 0.0, -1.0] };
        GunFrame { wrist, bore, up, left: v3::cross(up, bore) }
    }
    fn dir(&self, p: &[f64; 3]) -> [f64; 3] {
        let (x, y, d) = (&self.left, &self.up, &self.bore);
        [x[0] * p[0] + y[0] * p[1] + d[0] * p[2], x[1] * p[0] + y[1] * p[1] + d[1] * p[2], x[2] * p[0] + y[2] * p[1] + d[2] * p[2]]
    }
    fn at(&self, p: &[f64; 3]) -> [f64; 3] {
        v3::add(self.wrist, self.dir(p))
    }
}
/// What the steps of a reload hand on to the ones after them.
#[derive(Clone, Copy, Default)]
struct Feed {
    t: f64,
    // as_he_is: how he is trained, where his pouches are, and the beats of it
    tr: f64,
    hip_line: bool,
    bt: game_types::character::AnimRlB,
    fix_l: [f64; 4],
    // as_it_lies: the gun where the trigger hand has it, the hand where it was, and the trunk's own frame
    gun: GunFrame,
    cur: [f64; 3],
    rot0: [f64; 4],
    left: [f64; 3],
    up: [f64; 3],
    fwd: [f64; 3],
    // hip_line: the hip's pouches: off the pelvis in the trunk's frame, or by the hips' own line where a bladed chest would carry them round with it
    hips: [f64; 3],
    hl: [f64; 3],
    hf: [f64; 3],
    // pouch_and_beats: which pouch, the beats, and the hand: round a thing from its front (fingers across it, palm toward him), or fingers down into the pouch
    pouch: [f64; 3],
    r_pole: [f64; 3],
    pole_w: f64,
    o: f64,
    i: f64,
    r: f64,
    dur: f64,
    mf: [f64; 3],
    mb: [f64; 3],
    pf: [f64; 3],
    pb: [f64; 3],
    // keyed_hand: a gun fed its own way: the hand through its stations, and back to where it was
    end: f64,
    // mag_points: the empty comes straight down out of the well and is let go there; the fresh one comes up from under. An untrained man's lands on the lip beside the well, is backed off, then found; seated, he slaps it home; and the handle let go, the hand flies on back
    mag: [f64; 3],
    strip: [f64; 3],
    hd: [f64; 3],
    pl: [f64; 3],
    fum: bool,
    hp: f64,
    fl: f64,
    hit: [f64; 3],
    off: [f64; 3],
    tap_p: [f64; 3],
    over: [f64; 3],
    // mag_path: where the hand is on its way: a magazine with rounds in it goes to the dump pouch on the hip before the hand goes for a fresh one
    kp: f64,
    fr: f64,
    to: [f64; 3],
    // mag_turns: the hand: round the magazine from its front, fingers down into the pouch, hooked on the handle; down at the pouches from the first of them to the last
    hf_d: [f64; 3],
    hb_d: [f64; 3],
    hq: f64,
    hr: f64,
}

/// the long gun's magazine change, the support hand's: to the well, the empty stripped, a fresh one
/// fetched and seated, the handle run, back to the fore-end; an untrained man's lands on the lip, is backed off and found, and is slapped home. Or, for
/// a gun fed its own way, the hand through the stations it is handed. Solved on the scratch pose at `at_b` in the gun's frame as the trigger hand has
/// it, laid on the pose by w through the support arm's mask.
pub fn reload_arm(body: &mut Body, an: &mut Anim, t: f64, w: f64, marks: &FeedMarks, keys: Option<&[FeedKey]>, tune: &Knobs) {
    let mut q = Feed { t, ..Feed::default() };
    let (at, at_b, _) = poses(an);
    as_he_is(an, &mut q);
    let mut h = hands_of(body, an);
    body.copy(at, at_b);
    // (a feed with nothing to feed: the hand stays home)
    if keys.is_some_and(|k| k.len() < 2) {
        return;
    }
    let hh = &mut h;
    body.pose(at_b, |d, rig, fk| {
        fk_run(rig, d, fk.rot, fk.pos);
        let (up_l, fore_l, hand_l) = (rig.role(UPPER_ARM_L), rig.role(FOREARM_L), rig.role(HAND_L));
        as_it_lies(rig, fk, hand_l, &mut q);
        hip_line(rig, fk, &mut q);
        pouch_and_beats(rig, fk, &mut q);
        if let Some(keys) = keys {
            keyed_hand(keys, d, rig, fk, hand_l, up_l, fore_l, &mut q);
            keyed_fingers(keys, d, rig, fk, hh, hand_l, &q);
            return;
        }
        mag_points(marks, &mut q);
        mag_path(tune, &mut q);
        mag_turns(marks, &mut q);
        mag_hand(d, rig, fk, hand_l, up_l, fore_l, &q);
        mag_fingers(d, rig, fk, hh, hand_l, &q);
    });
    arm_mask(body, false);
    body.blend(at, at_b, w, true, at);
    hands_to(&h, an);
}

/// How he is trained, where his pouches are, and the beats of it.
fn as_he_is(an: &Anim, q: &mut Feed) {
    let (tr, hip_line, bt, fix_l) = (clamp(an.trained, 0.0, 1.0), an.long_gun, an.rl_b, an.hand_fix[0]);
    (q.tr, q.hip_line, q.bt, q.fix_l) = (tr, hip_line, bt, fix_l);
}

/// The gun where the trigger hand has it, the hand where it was, and the trunk's own frame.
fn as_it_lies(rig: &Rig, fk: &mut Fk, hand_l: usize, q: &mut Feed) {
    let gun = GunFrame::of(rig, fk);
    let (cur, rot0) = (fk.at(hand_l), fk.rot(hand_l));
    let (left, up, fwd) = trunk_frame(rig, fk);
    (q.gun, q.cur, q.rot0, q.left, q.up, q.fwd) = (gun, cur, rot0, left, up, fwd);
}

/// Off a point in the trunk's frame.
fn tk(q: &Feed, from: [f64; 3], by: [f64; 3]) -> [f64; 3] {
    let Feed { left, up, fwd, .. } = *q;
    v3::mad(v3::mad(v3::mad(from, left, by[0]), up, by[1]), fwd, by[2])
}

/// The hip's pouches: off the pelvis in the trunk's frame, or by the hips' own line where a bladed chest would carry them round with it.
fn hip_line(rig: &Rig, fk: &mut Fk, q: &mut Feed) {
    let Feed { hip_line, left, up, fwd, .. } = *q;
    let hips = fk.at(rig.role(HIPS));
    let (hl, hf) = if hip_line {
        let across = v3::sub(fk.at(rig.role(THIGH_L)), fk.at(rig.role(THIGH_R)));
        let level = v3::normalize(v3::mad(across, up, -v3::dot(across, up)));
        (level, v3::cross(level, up))
    } else {
        (left, fwd)
    };
    (q.hips, q.hl, q.hf) = (hips, hl, hf);
}

/// Off the hips by the hips' line.
fn hk(q: &Feed, by: [f64; 3]) -> [f64; 3] {
    let Feed { up, hips, hl, hf, .. } = *q;
    v3::mad(v3::mad(v3::mad(hips, hl, by[0]), up, by[1]), hf, by[2])
}

/// Which pouch, the beats, and the hand: round a thing from its front (fingers across it, palm toward him), or fingers down into the pouch.
fn pouch_and_beats(rig: &Rig, fk: &mut Fk, q: &mut Feed) {
    let Feed { tr, bt, gun, left, up, fwd, .. } = *q;
    let pouch = if tr > 0.5 { tk(q, fk.at(rig.role(SPINE2)), MV2.reload.pouch[1]) } else { hk(q, MV2.reload.pouch[0]) };
    let (r_pole, pole_w) = (MV2.reload.pole, MV2.reload.pole_w);
    let (o, i, r, dur) = (bt.out, bt.in_, bt.rack, bt.dur);
    // the hand: round a thing from its front (fingers across it, palm toward him) · fingers down into the pouch
    let (mf, mb) = (gun.dir(&[-1.0, 0.15, 0.1]), gun.dir(&[0.1, -0.35, 1.0]));
    let (pf, pb) = (v3::mad(v3::mad(v3::scale(up, -0.9), left, -0.3), fwd, 0.1), v3::mad(fwd, left, 0.3));
    (q.pouch, q.r_pole, q.pole_w, q.o, q.i, q.r, q.dur, q.mf, q.mb, q.pf, q.pb) = (pouch, r_pole, pole_w, o, i, r, dur, mf, mb, pf, pb);
}

/// A gun fed its own way: the hand through its stations (flat on top of the gun, at the pouch or round the magazine), resting where its last work was
/// until the weapon is nearly done, and back to where it was.
fn keyed_hand(keys: &[FeedKey], d: &mut [S], rig: &Rig, fk: &mut Fk, hand_l: usize, up_l: usize, fore_l: usize, q: &mut Feed) {
    let Feed { t, fix_l, gun, cur, pouch, r_pole, pole_w, dur, mf, mb, pf, pb, .. } = *q;
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
    q.end = end;
}

/// The fingers, closed as each station has them.
fn keyed_fingers(keys: &[FeedKey], d: &mut [S], rig: &Rig, fk: &mut Fk, hh: &mut Hands, hand_l: usize, q: &Feed) {
    let Feed { t, rot0, dur, end, .. } = *q;
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
    hand(rig, d, hh, 0, &[c(0.62), c(0.68), c(0.72), c(0.78)], lerp(0.3, 0.6, gr), 0.1 * (1.0 - gr));
    wrist_ease(d, rig, fk, hand_l, &rot0, bump(t, 0.0, 0.12, dur - 0.26, dur - 0.05));
}

/// The empty comes straight down out of the well and is let go there; the fresh one comes up from under. An untrained man's lands on the lip beside the well, is backed off, then found; seated, he slaps it home; and the handle let go, the hand flies on back.
fn mag_points(marks: &FeedMarks, q: &mut Feed) {
    let Feed { t, tr, bt, gun, o, i, r, .. } = *q;
    let half = (o + i) * 0.5;
    let (mag, strip, hd, pl) = (gun.at(&marks.mag), gun.at(&marks.strip), gun.at(&marks.handle), gun.at(&marks.pull));
    // the empty comes straight down out of the well and is let go there; the fresh one comes up from under. An untrained man's lands on the lip beside
    // the well, is backed off, then found; seated, he slaps it home; and the handle let go, the hand flies on back.
    let wob = MV2.reload.fumble * (1.0 - tr);
    let fum = wob > 0.004;
    let hp = if fum { half - 0.1 } else { half };
    let slap = fum && bt.pull && r - i >= 0.4;
    let fl = if fum { (1.0 - tr) * max(bump(t, i - 0.36, i - 0.22, i - 0.09, i + 0.02), if slap { bump(t, i + 0.07, i + 0.15, i + 0.19, i + 0.27) } else { 0.0 }) } else { 0.0 };
    let hit = v3::mad(v3::mad(v3::mad(mag, gun.up, -0.035), gun.left, wob * 1.2), gun.bore, wob * 1.6);
    let off = if fum { tk(q, v3::lerp3(hit, strip, 0.65), v3::scale(MV2.reload.flare.off, 1.0 - tr)) } else { hit };
    let tap_p = if slap { tk(q, v3::mad(mag, gun.up, -MV2.reload.tap * (1.0 - tr)), v3::scale(MV2.reload.flare.tap, 1.0 - tr)) } else { mag };
    let over = v3::mad(v3::mad(pl, gun.bore, -(MV2.reload.fling + 0.1 * (1.0 - tr))), gun.left, 0.04 + 0.05 * (1.0 - tr));
    (q.mag, q.strip, q.hd, q.pl, q.fum, q.hp, q.fl, q.hit, q.off, q.tap_p, q.over) = (mag, strip, hd, pl, fum, hp, fl, hit, off, tap_p, over);
}

/// Where the hand is on its way: a magazine with rounds in it goes to the dump pouch on the hip before the hand goes for a fresh one.
fn mag_path(tune: &Knobs, q: &mut Feed) {
    let Feed { t, bt, cur, pouch, o, i, r, dur, mag, strip, hd, pl, fum, hp, hit, off, tap_p, over, .. } = *q;
    let (kp, fr) = (bt.keep, bt.fresh);
    let dump = if kp >= 0.0 { hk(q, tune.kit.mag.dump) } else { pouch };
    let mut p = Path::of(&[(0.0, cur), (max(0.1, o - 0.2), mag), (o, mag), (o + 0.12, strip)]);
    if kp >= 0.0 {
        p.push((o + 0.05 + kp) * 0.5, hk(q, tune.kit.mag.via));
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
    let to = p.at(t);
    (q.kp, q.fr, q.to) = (kp, fr, to);
}

/// The hand: round the magazine from its front, fingers down into the pouch, hooked on the handle; down at the pouches from the first of them to the last.
fn mag_turns(marks: &FeedMarks, q: &mut Feed) {
    let Feed { gun, hp, kp, fr, .. } = *q;
    let (hf_d, hb_d) = (gun.dir(&marks.hf), gun.dir(&marks.hb));
    // (the hand is down at the pouches from the first of them to the last)
    let (hq, hr) = if kp >= 0.0 { (kp - 0.07, fr + 0.1) } else { (hp, hp + 0.12) };
    (q.hf_d, q.hb_d, q.hq, q.hr) = (hf_d, hb_d, hq, hr);
}

/// A line of the hand through it.
fn lines(q: &Feed, a0: [f64; 3], a1: [f64; 3], a2: [f64; 3]) -> [f64; 3] {
    let Feed { t, bt, o, i, r, hq, hr, .. } = *q;
    let mut line = Path::of(&[(max(0.1, o - 0.2), a0), (o + 0.12, a0), (hq, a1), (hr, a1), (i - 0.24, a0), (i + 0.08, a0)]);
    if bt.pull {
        line.push(r - 0.1, a2);
        line.push(r + 0.16, a2);
    }
    line.at(t)
}

/// The arm.
fn mag_hand(d: &mut [S], rig: &Rig, fk: &mut Fk, hand_l: usize, up_l: usize, fore_l: usize, q: &Feed) {
    let Feed { fix_l, r_pole, pole_w, mf, mb, pf, pb, fl, to, hf_d, hb_d, .. } = *q;
    let rot = hand_rot(&fix_l, lines(q, mf, pf, hf_d), lines(q, mb, pb, hb_d));
    let pole = if fl > 0.0 { v3::lerp3(r_pole, MV2.reload.flare.pole, fl) } else { r_pole };
    two_bone_ik(rig, d, fk.rot, fk.pos, up_l, fore_l, hand_l, &to, Some(&pole), pole_w, Some(&rot), false);
}

/// The fingers.
fn mag_fingers(d: &mut [S], rig: &Rig, fk: &mut Fk, hh: &mut Hands, hand_l: usize, q: &Feed) {
    let Feed { t, bt, rot0, o, i, r, dur, kp, fr, .. } = *q;
    let grip = if kp >= 0.0 { max(bump(t, o - 0.12, o - 0.02, kp - 0.02, kp + 0.06), bump(t, fr, fr + 0.08, i + 0.04, i + 0.12)) } else { bump(t, o - 0.12, o - 0.02, i + 0.04, i + 0.12) };
    let hook = if bt.pull { bump(t, r - 0.14, r - 0.04, r + 0.14, r + 0.22) } else { 0.0 };
    let c = |kk: f64| lerp(0.3, kk, max(grip, hook));
    hand(rig, d, hh, 0, &[c(0.62), c(0.68), c(0.72), c(0.78)], lerp(0.3, 0.6, grip), 0.1 * (1.0 - grip));
    wrist_ease(d, rig, fk, hand_l, &rot0, bump(t, 0.0, 0.12, dur - 0.26, dur - 0.05));
}
/// A pistol's magazine change, the support hand's, over the reload clip's pose: to the butt as the magazine drops free, to the belt for a fresh one (a
/// magazine kept goes to the hip first), seated with the heel of the palm, and over the top to run the slide. The trigger hand, the gun and the body
/// stay the clip's.
pub fn pistol_reload_arm(body: &mut Body, an: &mut Anim, t: f64, w: f64, tune: &Knobs) {
    let (at, at_b, _) = poses(an);
    let (bt, fix_l) = (an.rl_b, an.hand_fix[0]);
    let mut h = hands_of(body, an);
    body.copy(at, at_b);
    let hh = &mut h;
    body.pose(at_b, |d, rig, fk| {
        fk_run(rig, d, fk.rot, fk.pos);
        let gun = GunFrame::of(rig, fk);
        let hand_l = rig.role(HAND_L);
        let (cur, rot0) = (fk.at(hand_l), fk.rot(hand_l));
        let (left, up, fwd) = trunk_frame(rig, fk);
        let hips = fk.at(rig.role(HIPS));
        let tk = |p: [f64; 3]| v3::mad(v3::mad(v3::mad(hips, left, p[0]), up, p[1]), fwd, p[2]);
        let (pouch, dump) = (tk(tune.kit.mag.pouch), tk(tune.kit.mag.dump));
        let (o, i, r, dur, kp, fr) = (bt.out, bt.in_, bt.rack, bt.dur, bt.keep, bt.fresh);
        let (well, strip) = (gun.at(&tune.kit.mag.well), gun.at(&tune.kit.mag.strip));
        let (sl, pl) = (gun.at(&tune.kit.mag.slide), gun.at(&tune.kit.mag.pull));
        // (seated: the heel of the palm drives it the last centimetre)
        let seat = v3::mad(well, gun.up, 0.012);
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
            p.push(r + 0.15, v3::mad(pl, gun.up, 0.03));
        } else {
            p.push(i + 0.3, cur);
        }
        p.push(dur - 0.03, cur);
        let (wf, wb) = (gun.dir(&tune.kit.mag.wf), gun.dir(&tune.kit.mag.wb));
        let (sf, sb) = (gun.dir(&tune.kit.mag.sf), gun.dir(&tune.kit.mag.sb));
        let (pf, pb) = (v3::mad(v3::mad(v3::scale(up, -0.9), left, -0.25), fwd, 0.15), v3::mad(fwd, left, 0.4));
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
        two_bone_ik(rig, d, fk.rot, fk.pos, rig.role(UPPER_ARM_L), rig.role(FOREARM_L), hand_l, &p.at(t), Some(&tune.kit.mag.pole), tune.kit.mag.pole_w, Some(&rot), false);
        let grip = max(if kp >= 0.0 { bump(t, o - 0.08, o, kp - 0.02, kp + 0.06) } else { 0.0 }, bump(t, fr, fr + 0.08, i - 0.02, i + 0.06));
        let pinch = if bt.pull { bump(t, r - 0.1, r - 0.02, r + 0.08, r + 0.16) } else { 0.0 };
        let c = |kk: f64| lerp(0.25, kk, max(grip, pinch));
        hand(rig, d, hh, 0, &[c(0.6), c(0.66), c(0.7), c(0.76)], lerp(0.3, 0.65, max(grip, pinch)), 0.1 * (1.0 - grip));
        wrist_ease(d, rig, fk, hand_l, &rot0, bump(t, 0.0, 0.1, dur - 0.22, dur - 0.05));
    });
    arm_mask(body, false);
    body.blend(at, at_b, w, true, at);
    hands_to(&h, an);
}
