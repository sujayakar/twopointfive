use super::*;                      // your own module: its types, its other parts' functions
use crate::ids::*;
use crate::js;
use crate::m;
use crate::v3::{self, V3};
use alloc::{vec, vec::Vec};

/// a boolean as a number
fn b01(x: bool) -> f64 { if x { 1.0 } else { 0.0 } }

// ---- the door hand

// ---- handwork

/// What the work hands are doing: the pose last fed, the shown weight, which hand, both hands?
pub fn anim_work_info(g: &crate::game::Game, c: CharId) -> crate::character::AnimWorkInfo {
    let k = &g.ch[c].anim.wk;
    crate::character::AnimWorkInfo { pose: k.last.clone(), w: k.shown, hand: k.main, two: k.two }
}

/// The verb is done: the pose carries on by itself for `secs` past its end.
pub fn anim_work_after(g: &mut crate::game::Game, c: CharId, secs: f64) {
    if g.ch[c].anim.wk.last.is_some() { g.ch[c].anim.wk.after = secs; }
}

/// A line over the radio: the left hand goes up for about `secs`.
pub fn anim_radio(g: &mut crate::game::Game, c: CharId, secs: f64) {
    let r = &mut g.ch[c].anim.rx;
    r.left = m::max(r.left, v3::clamp(secs, 0.8, 3.0));
}

/// What the gun hand visibly holds: the rule's, unless a swap is running.
pub fn anim_swap_held(g: &crate::game::Game, c: CharId, rule: Option<crate::guards::InHand>) -> Option<crate::guards::InHand> {
    let s = g.ch[c].anim.sw;
    if s.t < 0.0 {
        if let (Some(r), Some(Some(cur))) = (rule, s.cur) {
            if cur != r { return Some(cur); }
        }
        return rule;
    }
    let a = s.t / s.dur;
    let k = swap();
    if a < k.away { s.from } else if a < k.take { None } else { s.to }
}

/// How far the swap's hand is at the torch on his belt this frame.
pub(super) fn anim_swap_belt(g: &crate::game::Game, c: CharId) -> f64 {
    g.ch[c].anim.sw.belt
}

/// The torch and sidearm swap's clock, and its arm when it shows.
pub(super) fn anim_swap_layer(g: &mut crate::game::Game, c: CharId, dt: f64, taken: bool, tw_y: f64, tw_p: f64) {
    if taken { g.ch[c].anim.sw.t = -1.0; return; }
    let weapon = g.ch[c].anim.sw.to == Some(crate::guards::InHand::Weapon);
    if g.ch[c].anim.shoot_t >= 0.0 && weapon {
        let s = &mut g.ch[c].anim.sw;
        s.t = m::max(s.t, s.dur * 0.85);
    }
    let s = &mut g.ch[c].anim.sw;
    s.t += dt;
    if s.t >= s.dur { s.t = -1.0; return; }
    let a = s.t / s.dur;
    let k_sw = swap();
    let e = bump(a, 0.0, k_sw.away * 0.85, k_sw.take + 0.08, 0.98);
    if e <= 0.003 { return; }
    let k = v3::smoothstep(k_sw.away - 0.04, k_sw.take, a);
    s.belt = e * (if weapon { 1.0 - k } else { k });
    if g.ch[c].anim.sw_buf.is_none() {
        let p = g.ch[c].poses.make();
        g.ch[c].anim.sw_buf = Some(p);
    }
    let sw = g.ch[c].anim.sw_buf.unwrap();
    anim_swap_pose(g, c, e, k, weapon, tw_y, tw_p, sw);
    g.ch[c].anim.sw.shown = true;
}

/// The swap's arm, by its envelope e and how far across k.
fn anim_swap_pose(g: &mut crate::game::Game, c: CharId, e: f64, k: f64, weapon: bool, tw_y: f64, tw_p: f64, sw: crate::anim_skeleton::Pose) {
    crate::anim_skeleton::anim_layer(g, c, |s, an, _| ::sim::layers::swap_pose(s, an, sw.at as usize, e, k, weapon, tw_y, tw_p));
}

/// Hands at work on a thing: the gate, and the layer when it runs.
pub(super) fn anim_work_layer(g: &mut crate::game::Game, c: CharId, dt: f64, free: bool, armed: bool, tw_y: f64, tw_p: f64) {
    let k = &mut g.ch[c].anim.wk;
    if k.last.is_none() { k.shown = 0.0; return; }
    let live = k.fed && free;
    if !live && v3::clamp(m::max(k.w_l.x, k.w_r.x), 0.0, 1.0) <= 0.003 {
        k.w_l.x = 0.0; k.w_l.v = 0.0; k.w_r.x = 0.0; k.w_r.v = 0.0;
        k.ok_l = false; k.ok_r = false; k.shown = 0.0;
        if !k.fed && k.tw.x < 0.003 { k.last = None; }
        return;
    }
    let l = k.last.clone().unwrap();
    let (p, n) = (anim_to_model_p(g, c, l.p), anim_to_model_d(g, c, l.n));
    let an = &g.ch[c].anim;
    let r_free = l.hand != Some(0.0) && !armed && an.upper != UpperMode::Torch && (!an.long_gun || an.stowed) && (!an.grip[1] || an.stowed) && an.throw_t < 0.0 && an.sw.t < 0.0;
    crate::anim_skeleton::anim_layer(g, c, |s, an, _| ::sim::layers::work_layer(s, an, &l, n, p, dt, live, r_free, tw_y, tw_p));
}

/// The torch up in the hand that holds it, weight e.
pub(super) fn anim_torch_up_layer(g: &mut crate::game::Game, c: CharId, e: f64) { crate::anim_skeleton::anim_layer(g, c, |s, an, _| ::sim::layers::torch_up(s, an, e)); }

/// A hand to the radio: its clock and spring, and the pose when it shows.
pub(super) fn anim_radio_layer(g: &mut crate::game::Game, c: CharId, dt: f64, free: bool) {
    let r = &mut g.ch[c].anim.rx;
    r.left = m::max(0.0, r.left - dt);
    let want = r.left > 0.0 && free;
    let w = v3::clamp(spring_step(&mut r.w, b01(want), if want { 20.0 } else { 15.0 }, dt), 0.0, 1.0);
    if w <= 0.003 {
        if !want { r.w.x = 0.0; r.w.v = 0.0; }
        return;
    }
    let hw = w * (1.0 - g.ch[c].anim.upper_w.aim);   // (the head tips to the handset only as far as his gun is down)
    anim_radio_pose(g, c, w, hw);
}

/// The hand at the radio by w, and the head's share hw.
fn anim_radio_pose(g: &mut crate::game::Game, c: CharId, w: f64, hw: f64) { crate::anim_skeleton::anim_layer(g, c, |s, an, _| ::sim::layers::radio_pose(s, an, w, hw)); }

// ---- holding a man, and being held

/// One man holding another from behind, weight w.
pub(super) fn anim_hold_layer(g: &mut crate::game::Game, c: CharId, w: f64, dt: f64) {
    let (show, an) = (g.m.character.hold_show, &g.ch[c].anim);
    let h = an.hold_pose.unwrap_or(an.hd_last);
    let strip_at = an.hold_pose.and_then(|h| h.strip).and_then(|s| s.at);
    let (man, strip_at) = (h.man.map(|p| anim_to_model_p(g, c, p)), strip_at.map(|p| anim_to_model_p(g, c, p)));
    let beats = h.beats.unwrap_or(show.beats);
    crate::anim_skeleton::anim_layer(g, c, |s, an, k| ::sim::layers::hold_layer(s, an, w, dt, &h, beats, man, strip_at, k));
}

/// The held man, weight w.
pub(super) fn anim_held_layer(g: &mut crate::game::Game, c: CharId, w: f64, dt: f64) {
    let (show, an) = (g.m.character.hold_show, &g.ch[c].anim);
    let h = an.held_pose.unwrap_or(an.he_last);
    let beats = h.beats.unwrap_or(show.beats);
    crate::anim_skeleton::anim_layer(g, c, |s, an, k| ::sim::layers::held_layer(s, an, w, dt, &h, beats, k));
}

/// Out of a hold, with the layer still on its way out: has he been told to do anything since it let go? Latched until the next hold.
pub(super) fn anim_hold_busy(g: &mut crate::game::Game, c: CharId) -> bool {
    let an = &mut g.ch[c].anim;
    // (undefined, null or a word: all three are told apart)
    let carry = an.carry;
    if an.hold_pose.is_some() {
        an.hd_idle.upper = an.upper; an.hd_idle.long = an.long_gun; an.hd_idle.crouch = an.crouch_target;
        an.hd_idle.carry = carry;
        an.hd_busy = false;
        return false;
    }
    if an.hd_busy || an.hd_w.x <= 0.003 { return an.hd_busy; }
    let i = &an.hd_idle;
    an.hd_busy = an.upper != i.upper || an.long_gun != i.long || an.crouch_target != i.crouch || carry != i.carry
        || an.shoot_t >= 0.0 || an.reload_t >= 0.0 || an.throw_t >= 0.0 || an.strike_t >= 0.0 || an.hit_t >= 0.0 || an.kick_t >= 0.0 || an.sig_t >= 0.0 || an.bw.is_some() || an.kit.is_some()
        || an.lockpick || an.haul.is_some() || an.work.is_some() || an.door_push.is_some() || an.stance != Stance::None || an.ready.is_some() || an.creep || an.act.is_some();
    an.hd_busy
}

// ---- the big tool

/// The big slot's tool in his hands: the way out that poses nothing, then the layer.
pub(super) fn anim_big_layer(g: &mut crate::game::Game, c: CharId, b: &crate::character::AnimBigTool, tw_y: f64, tw_p: f64) {
    let stowing = b.end >= 0.0 && b.t >= b.end;
    let us = if stowing { v3::clamp((b.t - b.end) / m::max(1e-3, b.stow), 0.0, 1.0) } else { 0.0 };
    let ham = b.kind == crate::kit::BigTool::Hammer;
    if !(b.t >= 0.0) || us >= 1.0 { g.ch[c].anim.big_under = false; g.ch[c].anim.big_k = 0.0; return; }
    if ham && g.ch[c].anim.big_buf.is_none() {
        let p = g.ch[c].poses.make();
        g.ch[c].anim.big_buf = Some(p);
    }
    let near = blows_near(&crate::character::BlowsNearB { t: b.t, end: b.end, blows: b.blows.clone() });
    let at = if (b.at[0] + b.at[1] + b.at[2]).is_finite() { anim_to_model_p(g, c, b.at) } else { [0.0, 1.0, 0.8] };
    let at_buf = if ham { g.ch[c].anim.big_buf.unwrap().at as usize } else { 0 };
    let grip = crate::rig_props::carbine_grip();
    crate::anim_skeleton::anim_layer(g, c, |s, an, k| ::sim::layers::big_layer(s, an, b, at_buf, &near, at, &grip, tw_y, tw_p, k));
}

// ---- the throw

/// The overhand throw at t.
pub(super) fn anim_throw_layer(g: &mut crate::game::Game, c: CharId, t: f64) {
    let tables = throw_tables(g.ch[c].anim.thr_low >= 0.0);   // (the swing's tables, in the layer's order)
    crate::anim_skeleton::anim_layer(g, c, |s, an, k| ::sim::layers::throw_layer(s, an, t, &tables, k));
}

// ---- the capsules' records

/// One capsule record [a, r, b] of Character.capsules: the segment a to c, its far end carried `extend_b` further along it.
pub(super) fn cap_rec(a: V3, c_2: V3, r: f64, extend_b: Option<f64>) -> Vec<f64> {
    let extend_b = extend_b.unwrap_or(0.0);
    let (mut bx, mut by, mut bz) = (c_2[0], c_2[1], c_2[2]);
    if extend_b > 0.0 {
        let (dx, dy, dz) = (c_2[0] - a[0], c_2[1] - a[1], c_2[2] - a[2]);
        let l = js::or(m::hyp3(dx, dy, dz), 1.0);
        bx += dx / l * extend_b; by += dy / l * extend_b; bz += dz / l * extend_b;
    }
    vec![a[0], a[1], a[2], r, bx, by, bz]
}

/// The shoe's capsule: heel to past the toe along the sole.
pub(super) fn cap_sole(foot: V3, toe: V3) -> Vec<f64> {
    let up = TOE_FLOOR_Y;
    let heel: V3 = [foot[0] - (toe[0] - foot[0]) * 0.35, m::min(foot[1], toe[1] - up + 0.033 + (foot[1] - toe[1]) * 0.15), foot[2] - (toe[2] - foot[2]) * 0.35];
    let tip: V3 = [toe[0], toe[1] - up + 0.033, toe[2]];
    cap_rec(heel, tip, 0.033, Some(0.06))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_capsule_runs_between_its_ends_and_its_far_end_can_be_carried_on() {
        let (a, c): (V3, V3) = ([0.0, 1.0, 0.0], [0.3, 1.4, 0.0]);
        assert_eq!(cap_rec(a, c, 0.07, None), vec![0.0, 1.0, 0.0, 0.07, 0.3, 1.4, 0.0]);
        let far = cap_rec(a, c, 0.07, Some(0.5));
        assert_eq!(&far[..4], &[0.0, 1.0, 0.0, 0.07], "the near end and the radius stay");
        let b: V3 = [far[4], far[5], far[6]];
        assert!(v3::dist(b, [0.6, 1.8, 0.0]) < 1e-9, "half a metre on along the same line: {b:?}");
        // a capsule of no length stands where it is: there is no way to carry it, and no NaN
        assert_eq!(cap_rec(c, c, 0.05, Some(0.2)), vec![0.3, 1.4, 0.0, 0.05, 0.3, 1.4, 0.0]);
    }

    /// A round at a boot hits the boot: the capsule reaches behind the heel and past the toe, and lies on the floor when the toe does.
    #[test]
    fn a_shoes_capsule_lies_on_the_floor_from_behind_his_heel_to_past_his_toe() {
        let foot: V3 = [0.0, 0.12, 0.0];
        let toe: V3 = [0.0, TOE_FLOOR_Y, 0.15];
        let r = cap_sole(foot, toe);
        assert_eq!(r[3], 0.033);
        assert!(r[2] < foot[2], "it starts behind the ankle: {}", r[2]);
        assert!(r[6] > toe[2], "it ends past the toe: {}", r[6]);
        assert!(m::abs(r[5] - 0.033) < 0.01, "the toe end's underside is on the floor: {}", r[5]);
        assert!(r[1] >= 0.033 - 1e-9 && r[1] <= foot[1], "the heel end is under the ankle and not under the floor: {}", r[1]);
    }
}
