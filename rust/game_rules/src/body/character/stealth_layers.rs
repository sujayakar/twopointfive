//! src/game/character.ts lines 3539 to 3981
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

// ---- the bench, and what is private to this part


// ---- the held gait

/// The held gait's share this frame; sets crV, the speed the root itself made since the last frame. (character.ts CharacterAnimator.creepGait)
pub(super) fn anim_creep_gait(g: &mut crate::game::Game, c: CharId, dt: f64) -> f64 {
    if g.ch[c].anim.cr_t <= 0.0 && m::abs(g.ch[c].anim.speed) < 0.05 {
        let pl = anim_creep_plant(g, c);
        let a = &mut g.ch[c].anim;
        let p = pl[if a.crouch_target > 0.5 { 1 } else { 0 }];
        a.phase = js::rem(p + m::round((a.phase - p) * 2.0) / 2.0 + 2.0, 1.0);
    }
    let (gait_in, gait_out) = { let k = &crate::character::CREEP; (k.gait_in, k.gait_out) };
    let rate = if g.ch[c].anim.creep { dt / gait_in } else { -dt / gait_out };
    g.ch[c].anim.cr_t = v3::clamp(g.ch[c].anim.cr_t + rate, 0.0, 1.0);
    let pp = g.ch[c].anim.life.prev_pos;
    let p0 = g.ch[c].pos;   // posIn is Character.pos itself (character.ts:4419 keeps the array, not a copy)
    let d = if pp[0].is_finite() { m::hyp2(p0[0] - pp[0], p0[2] - pp[2]) } else { 0.0 };
    let a = &mut g.ch[c].anim;
    a.cr_v = if dt > 0.0 && d <= 0.5 { m::min(d / dt, 8.0) } else { 0.0 };
    v3::smoothstep(0.0, 1.0, a.cr_t)
}

/// [walking, crouch-walking]: the phase, mod half a cycle, at which the clip stands widest with both feet down; found once. (character.ts CharacterAnimator.creepPlant)
fn anim_creep_plant(g: &mut crate::game::Game, c: CharId) -> [f64; 2] {
    if let Some(p) = g.ch[c].anim.cr_ph { return p; }
    let walk = *g.ch[c].anim.clips.get("Walk_Loop").unwrap();
    let crouch = *g.ch[c].anim.clips.get("Crouch_Fwd_Loop").unwrap();
    let a = creep_plant_find(g, c, walk, crate::character::WALK_PHASE);
    let b = creep_plant_find(g, c, crouch, 0.0);
    let ph = [a, b];
    g.ch[c].anim.cr_ph = Some(ph);
    ph
}

/// the closure `find` of creepPlant. The legs' list is copied out of the man for the walk (once a man; fkPart takes it as an argument)
fn creep_plant_find(g: &mut Game, c: CharId, clip: ClipId, off: f64) -> f64 {
    let (toe_l, toe_r, p_c) = (g.ch[c].anim.nodes.toe_l, g.ch[c].anim.nodes.toe_r, g.ch[c].anim.p_c);
    let legs = g.ch[c].anim.fk_legs.clone();
    let singles = g.k.posemath.singles;
    let mut best = 0.0;
    let mut score = -m::INF;
    for i in 0..96 {
        let dur = g.tab.clip[clip].duration;
        crate::anim_skeleton::sample_clip(g, clip, (i as f64 / 96.0 + off) * dur, true, p_c, singles);
        anim_fk_part(g, c, p_c, &legs);
        let pos = &g.ch[c].anim.fk.pos;
        let (a, b) = (pos[toe_l], pos[toe_r]);
        let sc = m::abs(a[2] - b[2]) - 4.0 * m::abs(a[1] - b[1]);
        if sc > score { score = sc; best = i as f64 / 96.0; }
    }
    js::rem(best, 0.5)
}

// ---- WATCH

/// WATCH: the slip's body and arm and the look, both points taken through the twist's inverse first. (character.ts CharacterAnimator.watchLayer)
pub(super) fn anim_watch_layer(g: &mut crate::game::Game, c: CharId, dt: f64, free: f64, w_l: f64, w_r: f64, tw_y: f64, tw_p: f64) {
    let un = anim_untwist(g, c, tw_y, tw_p);
    let m_s = match g.ch[c].anim.slip_at { Some(p) => Some(untwist_at(&un, anim_to_model_p(g, c, p))), None => None };
    let m_w = match g.ch[c].anim.watch_at { Some(p) => Some(untwist_at(&un, anim_to_model_p(g, c, p))), None => None };
    anim_watch_slip(g, c, dt, free, m_s);
    anim_watch_arm(g, c, w_l, w_r);
    anim_watch_look(g, c, dt, free, m_w);
    let a = &mut g.ch[c].anim;
    let t = a.wt;
    a.watch_shown.w = t.w; a.watch_shown.yaw = t.yaw * t.w; a.watch_shown.slip = t.sw; a.watch_shown.lean = t.sl;
}

/// The slip's body: pelvis and trunk; `m` the man he gives room to, in the model.
fn anim_watch_slip(g: &mut crate::game::Game, c: CharId, dt: f64, free: f64, m: Option<V3>) { anim_layer(g, c, |s, an, k| sim::layers::watch_slip(s, an, dt, free, m, k)); }

/// The slip's arm, the one on the man's side out low toward him.
fn anim_watch_arm(g: &mut crate::game::Game, c: CharId, w_l: f64, w_r: f64) {
    if g.ch[c].anim.wt.sw > 0.003 { anim_layer(g, c, |s, an, k| sim::layers::watch_arm(s, an, w_l, w_r, k)); }
}

/// The look: the chest a little, the neck, the head.
fn anim_watch_look(g: &mut crate::game::Game, c: CharId, dt: f64, free: f64, m: Option<V3>) { anim_layer(g, c, |s, an, k| sim::layers::watch_look(s, an, dt, free, m, k)); }

// ---- the close-in carriage

/// The close-in mode's carriage, weight `w`: the body, then the arms. (character.ts CharacterAnimator.creepLayer)
pub(super) fn anim_creep_layer(g: &mut crate::game::Game, c: CharId, w: f64, w_l: f64, w_r: f64, tw_y: f64, tw_p: f64, dt: f64) {
    let (mut w_l, mut w_r) = (w_l, w_r);
    let far = g.k.kit.close.far;
    let an = &g.ch[c].anim;
    let cr = v3::clamp(an.crouch, 0.0, 1.0);
    let ck = if an.close_k >= 0.0 { an.cl_k } else { -1.0 };
    let cke = if ck < 0.0 { 0.0 } else { ck * ck * (3.0 - 2.0 * ck) };
    let e = w * w * (3.0 - 2.0 * w) * (if ck < 0.0 { 1.0 } else { v3::lerp(far, 1.0, cke) });
    let free = 1.0 - v3::clamp(an.upper_w.aim, 0.0, 1.0);
    let mut ws = 0.0;
    if an.hands_down > 0.0 {
        let hd = v3::clamp(an.hands_down, 0.0, 1.0);
        let k = 1.0 - hd * (1.0 - an.wide_s);
        ws = if k > 1e-4 { v3::clamp(hd * an.wide_s / k, 0.0, 1.0) } else { 0.0 };
        w_l *= k; w_r *= k;
    }
    let gw = if an.heft > 0.0 { v3::clamp(an.heft, 0.0, 1.0) * an.wide_s * e } else { 0.0 };
    let o = anim_creep_body(g, c, e, free, cr, gw, tw_y, tw_p, dt);
    let (ha, hg) = (o[0], o[1]);
    if !(w_l <= 0.003 && w_r <= 0.003) { anim_creep_arms(g, c, w, w_l, w_r, e, cr, ck, cke, ws, ha, hg); }
    g.ch[c].anim.creep_shown = w;
}

/// Its arms: the shoulders rounded and the hands brought up ahead of the hips by IK on a copy.
fn anim_creep_arms(g: &mut crate::game::Game, c: CharId, w: f64, w_l: f64, w_r: f64, e: f64, cr: f64, ck: f64, cke: f64, ws: f64, ha: f64, hg: f64) {
    let rc = anim_sheath_rest_inv(g, c);
    anim_layer(g, c, |s, an, k| sim::layers::creep_arms(s, an, w, w_l, w_r, e, cr, ck, cke, ws, ha, hg, rc, k));
}

/// Its body: the pelvis and the feet, the heft, the bow and the look. Answers [the arms' share of his stride, his surge].
fn anim_creep_body(g: &mut crate::game::Game, c: CharId, e: f64, free: f64, cr: f64, gw: f64, tw_y: f64, tw_p: f64, dt: f64) -> [f64; 2] {
    let (p0, mut out) = (g.ch[c].pos, [0.0; 2]);
    anim_layer(g, c, |s, an, _| sim::layers::creep_body(s, an, e, free, cr, gw, tw_y, tw_p, dt, p0, &mut out));
    out
}

// ---- the stealth poses

/// The arms of the hands-empty crouch. `busy`: something the animator knows has his arms or body. (character.ts CharacterAnimator.stealthLayer)
pub(super) fn anim_stealth_layer(g: &mut crate::game::Game, c: CharId, dt: f64, busy: bool, crouch_pose: f64, w_move: f64, tw_y: f64, tw_p: f64) {
    if g.ch[c].anim.sp.is_none() {
        g.ch[c].anim.sp = Some(AnimSp { w_a: 0.0, t: 0.0, cr: crouch_pose, fl: 0.0, lead: 0.0, fz: 0.0, sw: 0.0, w_c: 0.0, w_h: 0.0, pic: None, pic_on: false, nod_up: None });
    }
    anim_layer(g, c, |s, an, k| sim::layers::stealth_layer(s, an, dt, busy, crouch_pose, w_move, tw_y, tw_p, k));
}

/// The look under a door with the wand, on a copy of the finished pose that is then made the pose. (character.ts CharacterAnimator.stealthPicture)
pub(super) fn anim_stealth_picture(g: &mut crate::game::Game, c: CharId, dt: f64) {
    let cable_in = g.k.stealthpose.cable_in;
    let (cable_out, cable_head_out) = { let k = crate::character::STEALTH.cable; (k.out, k.head_out) };
    let an = &mut g.ch[c].anim;
    let want = an.stealth.as_ref().map_or(false, |f| f.cable) && an.lockpick;
    let nv_nod = an.nv_nod;
    let s = an.sp.as_mut().unwrap();
    s.w_c = v3::clamp(s.w_c + (if want { dt / m::max(0.05, cable_in) } else { -dt / cable_out }), 0.0, 1.0);
    s.w_h = v3::clamp(s.w_h + (if want { dt / m::max(0.05, cable_in) } else { -dt / cable_head_out }), 0.0, 1.0);
    if nv_nod == 0.0 { s.nod_up = None; }
    if s.w_c <= 0.003 { s.pic_on = false; return; }
    let (w_c, w_h, kept) = (s.w_c, s.w_h, s.pic);
    let (poses, p_a) = (g.ch[c].anim.poses, g.ch[c].anim.p_a);
    let pic = match kept {
        Some(p) => p,
        None => {
            let p = crate::anim_skeleton::pose_set_make(g, poses, p_a.nodes as f64);
            g.ch[c].anim.sp.as_mut().unwrap().pic = Some(p);
            p
        }
    };
    crate::anim_skeleton::pose_copy(g, pic, p_a);
    anim_cable_pose(g, c, pic, w_c * w_c * (3.0 - 2.0 * w_c), w_h * w_h * (3.0 - 2.0 * w_h));
    g.ch[c].anim.sp.as_mut().unwrap().pic_on = true;
}

/// The look under a door itself, on `pic`, by its eased weight e and the head's eh.
fn anim_cable_pose(g: &mut crate::game::Game, c: CharId, pic: crate::anim_skeleton::Pose, e: f64, eh: f64) { anim_layer(g, c, |s, an, k| sim::layers::cable_pose(s, an, pic.at as usize, e, eh, k)); }

// ---- the twist's inverse

/// The chest twist run backwards for a point; fk must be current on the pose asked about. (character.ts CharacterAnimator.untwist)
fn anim_untwist(g: &crate::game::Game, c: CharId, tw_y: f64, tw_p: f64) -> crate::character::Untwist {
    if !js::truthy(tw_y) && !js::truthy(tw_p) { return Untwist::Same; }
    let an = &g.ch[c].anim;
    let (p1, p2, p3) = (an.fk.pos[an.nodes.spine1], an.fk.pos[an.nodes.spine2], an.fk.pos[an.nodes.spine3]);
    untwist_of(q_yaw(tw_y * 0.25), q_yaw(tw_y * 0.35), q_yaw(tw_y * 0.4), tw_p, p1, p2, p3)
}

/// untwist's arithmetic, off the three yaws (E3 before its pitch) and the three joints
fn untwist_of(e1: Quat, e2: Quat, e3: Quat, tw_p: f64, p1: V3, p2: V3, p3: V3) -> Untwist {
    let mut e3 = e3;
    if js::truthy(tw_p) {
        let right = v3::quat::rotate(e3, [1.0, 0.0, 0.0]);
        e3 = v3::quat::mul(v3::quat::axis_angle(right, tw_p), e3);
    }
    let (i_e1, i_e2, i_e3) = (v3::quat::conj(e1), v3::quat::conj(e2), v3::quat::conj(e3));
    let p2t = v3::add(v3::quat::rotate(e1, v3::sub(p2, p1)), p1);
    let p3t = v3::add(v3::quat::rotate(e2, v3::sub(v3::add(v3::quat::rotate(e1, v3::sub(p3, p1)), p1), p2t)), p2t);
    Untwist::Back { i_e1, i_e2, i_e3, p1, p2t, p3t }
}

/// the closure untwist hands back: where to pose something below spine3 for it to end up at `h`
fn untwist_at(un: &Untwist, h: V3) -> V3 {
    let (i_e1, i_e2, i_e3, p1, p2t, p3t) = match *un { Untwist::Same => return h, Untwist::Back { i_e1, i_e2, i_e3, p1, p2t, p3t } => (i_e1, i_e2, i_e3, p1, p2t, p3t) };
    let x1 = v3::add(v3::quat::rotate(i_e3, v3::sub(h, p3t)), p3t);
    let x2 = v3::add(v3::quat::rotate(i_e2, v3::sub(x1, p2t)), p2t);
    v3::add(v3::quat::rotate(i_e1, v3::sub(x2, p1)), p1)
}

// ---- the kit swap

/// The swap's frame: both arms by IK on a copy of pA, then the clock. (character.ts CharacterAnimator.kitLayer)
pub(super) fn anim_kit_layer(g: &mut crate::game::Game, c: CharId, dt: f64, tw_y: f64, tw_p: f64) {
    let rc = anim_sheath_rest_inv(g, c);
    let sling = crate::rig_props::sling_front_hand(g, None, Some(g.ch[c].anim.kit_rest));
    anim_layer(g, c, |s, an, k| sim::layers::kit_layer(s, an, tw_y, tw_p, rc, &sling, k));
    let an = &mut g.ch[c].anim;
    let k = an.kit.as_mut().unwrap();
    k.t += dt;
    if k.t >= k.d { an.kit = None; }
}

// ---- the hold's catch and the ready pose

/// The hold's arm's last word: the left hand put on the hold's target, after the layers that move his trunk.
fn anim_catch_arm(g: &mut crate::game::Game, c: CharId, x: crate::character::AnimHdFix) { anim_layer(g, c, |s, an, _| sim::layers::catch_arm(s, an, &x)); }

/// HOLD.catch, after the layers: the left hand onto the hold's target; with the reach up and no hold, where it has the hand is noted. (character.ts CharacterAnimator.catchFix)
pub(super) fn anim_catch_fix(g: &mut crate::game::Game, c: CharId) {
    let x = g.ch[c].anim.hd_fix.take();
    if !g.ch[c].anim.no_knife { g.ch[c].anim.rd_hand_l = None; return; }
    if let Some(x) = x {
        anim_catch_arm(g, c, x);
    } else if g.ch[c].anim.hold_pose.is_none() {
        if g.ch[c].anim.ready_w > 0.003 && g.ch[c].anim.ready_k == ReadyKind::Grab {
            let p_a = g.ch[c].anim.p_a;
            let mut fk = core::mem::take(&mut g.ch[c].anim.fk);
            crate::anim_skeleton::pose_fk_run(g, &mut fk, p_a);
            let an = &mut g.ch[c].anim;
            an.fk = fk;
            an.rd_hand_l = Some(v3::copy(an.fk.pos[an.nodes.hand_l]));
        } else {
            g.ch[c].anim.rd_hand_l = None;
        }
    }
}

/// The ready pose: the trunk, then the arms. Weight `w`; `live` = still asked for. (character.ts CharacterAnimator.readyLayer)
pub(super) fn anim_ready_layer(g: &mut crate::game::Game, c: CharId, w: f64, kind: crate::character::ReadyKind, live: bool, dt: f64, tw_y: f64, tw_p: f64) {
    let shape = crate::character::ready().get(String::from(kind.as_str())).unwrap().clone();
    let there = g.ch[c].anim.rd_at.map(|p| anim_to_model_p(g, c, p));
    let rc = anim_sheath_rest_inv(g, c);
    anim_layer(g, c, |s, an, k| sim::layers::ready_layer(s, an, w, kind, &shape, live, dt, tw_y, tw_p, there, rc, k));
    g.ch[c].anim.ready_shown = w;
}

/// The rest pose's chest turn, inverted: the sheath's frame; found once, off a rest pose of its own. (character.ts CharacterAnimator.sheathRestInv)
fn anim_sheath_rest_inv(g: &mut crate::game::Game, c: CharId) -> Quat {
    if let Some(q) = g.ch[c].anim.rc_inv { return q; }
    let rig = g.ch[c].anim.rig;
    let mut f = crate::anim_skeleton::pose_fk_new(g, rig);
    let n = g.tab.rig[rig].nodes.len();
    let fresh = crate::anim_skeleton::pose_new(g, n);
    crate::anim_skeleton::rest_pose(g, rig, fresh);
    crate::anim_skeleton::pose_fk_run(g, &mut f, fresh);
    crate::anim_skeleton::pose_done(g, fresh);   // (the scratch is dropped)
    let an = &mut g.ch[c].anim;
    let q = v3::quat::conj(f.rot[an.nodes.spine3]);
    an.rc_inv = Some(q);
    q
}

// ---- the takedown

/// A takedown's clocks, ahead of the pose: the move's and the reaction's seconds, the blow's points riding the man, and the crouch they leave. (character.ts CharacterAnimator.takePre)
pub(super) fn anim_take_pre(g: &mut crate::game::Game, c: CharId, dt: f64, cp: f64) -> f64 {
    let mut cp = cp;
    let t = g.ch[c].anim.tk;
    if g.ch[c].anim.bw.is_none() { g.ch[c].anim.bw_rise = 0.0; }
    if let Some(mut b) = g.ch[c].anim.bw.clone() {
        b.t += dt;
        if b.t >= b.end + b.tail {
            g.ch[c].anim.bw = None;
            g.ch[c].anim.bw_rise = 0.0;
        } else {
            g.ch[c].anim.bw = Some(b.clone());
            if let Some(l) = b.live {
                if b.t < b.hit || b.style == BlowStyle::Mouth {
                    let (a, a2) = crate::takedown::marks(g, l.by, l.vic, l.style);
                    if b.t < b.hit { b.at = a; }
                    if let Some(a2) = a2 { b.at2 = Some(a2); }
                }
            }
            let table = &crate::character::BLOW;
            let rise = if b.rise { table.rise.of(b.style) * bump(b.t, 0.0, 0.6 * b.hit, b.hit + 0.1, b.end) } else { 0.0 };
            g.ch[c].anim.bw_rise = rise;
            cp *= 1.0 - rise;
            if b.tail > 0.0 { cp = m::max(cp, table.sink.of(b.style) * anim_blow_tail(g, c, &b)); }
            g.ch[c].anim.bw = Some(b);
        }
    }
    if let Some(mut tk) = t {
        tk.t += dt;
        let u = tk.t - tk.delay;
        if tk.off >= 0.0 { tk.off += dt; }
        g.ch[c].anim.tk = Some(tk);
        let table = &crate::character::TAKEN;
        let secs = table.secs;
        if u >= secs || tk.off >= 0.2 {
            g.ch[c].anim.tk = None;
        } else if u > 0.0 {
            cp = m::max(cp, table.sag.of(tk.style) * v3::smoothstep(0.0, 0.3, u) * (1.0 - v3::smoothstep(0.6, secs, u)) * (if tk.off < 0.0 { 1.0 } else { 1.0 - tk.off / 0.2 }));
        }
    }
    cp
}

/// How far into the move's tail he is, 0 to 1; let go of as he walks off. (character.ts CharacterAnimator.blowTail)
fn anim_blow_tail(g: &crate::game::Game, c: CharId, b: &crate::character::BlowRun) -> f64 {
    let x = b.end + b.tail;
    let w = crate::character::BLOW.walk;
    bump(b.t, b.hit + 0.08, b.end + 0.1, x - 0.45, x) * (1.0 - v3::smoothstep(w[0], w[1], m::abs(g.ch[c].anim.speed)))
}

/// The legs under the blow: he stands into the man. Nothing where he may not rise.
pub(super) fn anim_blow_legs(g: &mut crate::game::Game, c: CharId, b: &crate::character::BlowRun) {
    if b.rise { anim_layer(g, c, |s, an, _| sim::layers::blow_legs(s, an, b)); }
}

/// The trunk's share of the move, on pA. (character.ts CharacterAnimator.blowTrunk)
pub(super) fn anim_blow_trunk(g: &mut crate::game::Game, c: CharId, b: &crate::character::BlowRun) {
    let tl = if b.tail > 0.0 { anim_blow_tail(g, c, b) } else { 0.0 };
    anim_layer(g, c, |s, an, _| sim::layers::blow_trunk(s, an, b, tl));
}

/// The arms of the move, by IK on a copy of pA. Returns the layer's weight.
pub(super) fn anim_blow_arms(g: &mut crate::game::Game, c: CharId, b: &crate::character::BlowRun, tw_y: f64, tw_p: f64) -> f64 {
    let (at, at2) = (anim_to_model_p(g, c, b.at), b.at2.map(|p| anim_to_model_p(g, c, p)));
    let mut shown = 0.0;
    anim_layer(g, c, |s, an, k| sim::layers::blow_arms(s, an, b, at, at2, tw_y, tw_p, &mut shown, k));
    shown
}

/// The man a takedown is done to, still on his feet. (character.ts CharacterAnimator.takenLayer)
pub(super) fn anim_taken_layer(g: &mut crate::game::Game, c: CharId, t: crate::character::AnimTakenLayerT) {
    let u = t.t - t.delay;
    if u <= 0.0 { return; }
    anim_layer(g, c, |s, an, _| sim::layers::taken_layer(s, an, t.style, t.off, u));
}

/// The knife's blow: the right arm over the punch clip's pose, its clock and its pose. (character.ts CharacterAnimator.knifeBlow)
pub(super) fn anim_knife_blow(g: &mut crate::game::Game, c: CharId, kind: crate::character::StrikeKind, dt: f64, total: f64, tw_y: f64, tw_p: f64) {
    let kb = &crate::character::KNIFE_BLOW;
    let k = if g.ch[c].anim.kb_slash { &crate::character::KNIFE_SLASH } else if g.ch[c].anim.kb_back { &crate::character::KNIFE_BACK } else { match kind { StrikeKind::Cross => &kb.cross, StrikeKind::Jab => &kb.jab } };
    let t0 = kb.snap * (1.0 - g.ch[c].anim.kb_r0);
    let fit = m::min(1.0, m::max(0.2, total - kb.fade - t0 - 0.035) / k.end);
    let t = (g.ch[c].anim.kb_t - t0) / fit;
    g.ch[c].anim.kb_t += dt;
    anim_knife_pose(g, c, &k, t, t0, total, tw_y, tw_p);
    g.ch[c].anim.knife_shown = t > 0.0 && t < k.end;
}

/// The blow's pose, its clock being knifeBlow's: `t` the key time, `t0` the seconds to the hilt.
fn anim_knife_pose(g: &mut crate::game::Game, c: CharId, k: &crate::character::KnifeSlash, t: f64, t0: f64, total: f64, tw_y: f64, tw_p: f64) {
    let there = g.ch[c].anim.rd_at.map(|p| anim_to_model_p(g, c, p));
    let rc = anim_sheath_rest_inv(g, c);
    anim_layer(g, c, |s, an, n| sim::layers::knife_pose(s, an, k, t, t0, total, tw_y, tw_p, there, rc, n));
}

// ---- lock work, the haul

/// Lock work: the back un-rounded, the head on the lock, both hands at the keyway.
pub(super) fn anim_lockpick_layer(g: &mut crate::game::Game, c: CharId, w: f64) { anim_layer(g, c, |s, an, k| sim::layers::lockpick_layer(s, an, w, k)); }

/// Both hands on a body: the trunk bows toward the grip, both arms by IK on a copy. (character.ts CharacterAnimator.haulLayer)
pub(super) fn anim_haul_layer(g: &mut crate::game::Game, c: CharId, w: f64, tw_y: f64, tw_p: f64) {
    let h = match g.ch[c].anim.hl_last { Some(h) => h, None => return };
    let (l, r) = (anim_to_model_p(g, c, h.l), anim_to_model_p(g, c, h.r));
    anim_layer(g, c, |s, an, k| sim::layers::haul_layer(s, an, &h, w, l, r, tw_y, tw_p, k));
}

// ---- world and model

/// a point of the world in the un-twisted model space
fn to_model_p_of(yaw: f64, p0: V3, wp: V3) -> V3 {
    let (cy, sy) = (m::cos(yaw), m::sin(yaw));
    let (dx, dz) = (wp[0] - p0[0], wp[2] - p0[2]);
    [dx * cy - dz * sy, wp[1] - p0[1], dx * sy + dz * cy]
}
/// a direction of the world in the un-twisted model space
fn to_model_d_of(yaw: f64, wd: V3) -> V3 {
    let (cy, sy) = (m::cos(yaw), m::sin(yaw));
    [wd[0] * cy - wd[2] * sy, wd[1], wd[0] * sy + wd[2] * cy]
}

/// The controller's world placement to the model space the layers pose in: a point. (character.ts CharacterAnimator.toModelP)
pub(super) fn anim_to_model_p(g: &crate::game::Game, c: CharId, wp: V3) -> V3 {
    let p0 = crate::character::anim_pos_in(g, c);   // (`this.posIn`: the array handed in at the last update, not the body's place now)
    to_model_p_of(g.ch[c].anim.body_yaw_in, p0, wp)
}

/// The same for a direction. (character.ts CharacterAnimator.toModelD)
pub(super) fn anim_to_model_d(g: &crate::game::Game, c: CharId, wd: V3) -> V3 {
    to_model_d_of(g.ch[c].anim.body_yaw_in, wd)
}

// ---- the door hand

/// The door hand's eased weights per arm and which arm is the wanted one. (character.ts CharacterAnimator.doorHands)
pub fn anim_door_hands(g: &crate::game::Game, c: CharId) -> crate::character::AnimDoorHands {
    let dp = &g.ch[c].anim.dp;
    AnimDoorHands { l: v3::clamp(dp.w_l.x, 0.0, 1.0), r: v3::clamp(dp.w_r.x, 0.0, 1.0) }
}

/// A hand on the door leaf he is pushing open; the free arm only, the weight this layer's own springs. (character.ts CharacterAnimator.doorLayer)
pub(super) fn anim_door_layer(g: &mut crate::game::Game, c: CharId, dt: f64, intent: f64, armed: bool, tw_y: f64, tw_p: f64) {
    let s = &mut g.ch[c].anim.dp;
    let l = match s.last {
        Some(l) => l,
        None => {
            s.w_l.x = 0.0; s.w_l.v = 0.0; s.w_r.x = 0.0; s.w_r.v = 0.0;
            s.reach_ok = false;
            return;
        }
    };
    if !(intent > 0.003) && v3::clamp(m::max(s.w_l.x, s.w_r.x), 0.0, 1.0) <= 0.003 { s.reach_ok = false; return; }
    let (n, p) = (anim_to_model_d(g, c, l.n), anim_to_model_p(g, c, l.p));
    let an = &g.ch[c].anim;
    let free = !armed && an.upper != UpperMode::Torch && an.throw_t < 0.0;
    anim_layer(g, c, |s, an, k| sim::layers::door_layer(s, an, &l, n, p, dt, intent, free, tw_y, tw_p, k));
}

#[cfg(test)]
mod tests {
    use super::*;

    // Expected bits printed by bun from the TypeScript's own untwist and toModelP / toModelD arithmetic, under EXACT.on.
    fn bits(v: V3) -> [u64; 3] { [v[0].to_bits(), v[1].to_bits(), v[2].to_bits()] }
    const P1: V3 = [0.01, 0.95, -0.02];
    const P2: V3 = [0.015, 1.1, -0.01];
    const P3: V3 = [0.02, 1.27, 0.005];
    const H: V3 = [0.31, 1.42, 0.48];

    #[test]
    fn no_twist_is_the_point_itself() {
        assert_eq!(bits(untwist_at(&Untwist::Same, H)), bits(H));
    }

    #[test]
    fn untwist_matches_the_typescript_bits() {
        let y = |a: f64| v3::quat::axis_angle([0.0, 1.0, 0.0], a);   // (qYaw is another part's)
        let un = untwist_of(y(0.7 * 0.25), y(0.7 * 0.35), y(0.7 * 0.4), -0.3, P1, P2, P3);
        assert_eq!(bits(untwist_at(&un, H)), [0xbfb4b567bc34da6d, 0x3ff410ad046c02dc, 0x3fe24fb80647771c]);
        let un = untwist_of(y(-1.1 * 0.25), y(-1.1 * 0.35), y(-1.1 * 0.4), 0.0, P1, P2, P3);
        assert_eq!(bits(untwist_at(&un, H)), [0x3fe28f9e04981869, 0x3ff6b851eb851eb8, 0xbfa9a923ab14f612]);
    }

    #[test]
    fn to_model_matches_the_typescript_bits() {
        let (yaw, p0, wp) = (0.83, [3.25, 0.1, -7.5], [4.1, 1.6, -6.2]);
        assert_eq!(bits(to_model_p_of(yaw, p0, wp)), [0xbfd8aec2112b716e, 0x3ff8000000000000, 0x3ff812c2a39a5f54]);
        assert_eq!(bits(to_model_d_of(yaw, wp)), [0x401d5e6088506071, 0x3ff999999999999a, 0xbff28a14a40e3ade]);
    }

    #[test]
    fn every_word_has_its_row() {
        let r = crate::character::ready();
        for k in [ReadyKind::Grab, ReadyKind::Seat, ReadyKind::Strike, ReadyKind::GrabGun] { assert!(r.get(String::from(k.as_str())).is_some()); }
    }
}
