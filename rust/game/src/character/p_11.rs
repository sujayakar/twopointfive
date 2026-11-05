//! src/game/character.ts lines 3981 to 4293
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

// ---- the bench and the seams of this part

/// where a layer's columns go on the bench's `args` (skeleton.ts LAYER_COLS_AT)
const COLS: usize = crate::anim_skeleton::LAYER_COLS_AT as usize;
/// where a layer's keys go on the bench's `args` (skeleton.ts LAYER_KEYS_AT)
const KEYS: usize = crate::anim_skeleton::LAYER_KEYS_AT as usize;

/// the bench's `args`: a layer's own numbers from 0, its columns from COLS (`f64()` at `POSE_BENCH.args`). Never hold it across a call that may use the bench.
fn args() -> &'static mut [f64; ::sim::skeleton::ARGS] { &mut ::sim::skeleton::bench().args }
/// a boolean as the bench holds it
fn b01(x: bool) -> f64 { if x { 1.0 } else { 0.0 } }

/// layerSlot(this.ch, this.pA, this.cast): the pose and the cast are lent out of the animator for the call (it reads neither through `g`) and put back
fn man_slot(g: &mut Game, c: CharId) -> f64 {
    let rig = g.ch[c].anim.rig;
    let pose = core::mem::replace(&mut g.ch[c].anim.p_a, crate::anim_skeleton::Pose::blank());
    let cast = core::mem::replace(&mut g.ch[c].anim.cast, crate::anim_skeleton::LayerCast::blank());
    let slot = crate::anim_skeleton::layer_slot(g, rig, &pose, &cast);
    g.ch[c].anim.p_a = pose;
    g.ch[c].anim.cast = cast;
    slot
}
/// layerRan(code, this.fk): the bones are lent out of the animator for the call and put back, written
fn man_ran(g: &mut Game, c: CharId, code: f64) {
    let mut fk = core::mem::replace(&mut g.ch[c].anim.fk, crate::anim_skeleton::PoseFK::blank());
    crate::anim_skeleton::layer_ran(g, code, &mut fk);
    g.ch[c].anim.fk = fk;
}
/// the slot as a layer takes it: below zero (a pose at home in no block) is the bench, never game slot 0
fn slot_no(g_2: f64) -> usize {
    if g_2 < 0.0 { ::sim::skeleton::ON_BENCH } else { slot_no(g_2) }
}
/// `pose.at`: where a pose stands in the game's block. A Pose is by value and has no place, and a layer finds its poses only by place (character.ts:3990)
fn seam_pose_at(p: &crate::anim_skeleton::Pose) -> usize { todo!("needs: a ruling on by-value poses against sim::layers (Pose.at, or copies into the pool or onto the bench), character.ts:3990") }

/// HOLD_PH[phase] (character.ts:1109), the phase's number on the bench
fn hold_no(p: HoldPhase) -> f64 { *hold_ph().get(String::from(p.as_str())).unwrap() }
/// WORK_NO[kind] (character.ts:1136), the kind's number on the bench
fn work_kind_no(k: WorkKind) -> f64 { *work_no().get(String::from(k.as_str())).unwrap() }

// ---- the door hand

/// doorCols.get onto the bench (character.ts:3984)
fn door_cols_get(g: &mut Game, c: CharId) {
    let s = &g.ch[c].anim.dp;
    let (a, o) = (args(), COLS);
    a[o] = s.w_l.x; a[o + 1] = s.w_l.v; a[o + 2] = s.w_r.x; a[o + 3] = s.w_r.v; a[o + 4] = b01(s.reach_ok); a[o + 5] = b01(s.right);
}
/// doorCols.put off the bench (character.ts:3985)
fn door_cols_put(g: &mut Game, c: CharId) {
    let s = &mut g.ch[c].anim.dp;
    let (a, o) = (args(), COLS);
    s.w_l.x = a[o]; s.w_l.v = a[o + 1]; s.w_r.x = a[o + 2]; s.w_r.v = a[o + 3]; s.reach_ok = a[o + 4] != 0.0; s.right = a[o + 5] != 0.0;
}

/// The door hand's numbers onto the bench, the layer, its columns back. (character.ts doorLayerAsk)
pub fn anim_door_layer_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, l: crate::character::DoorPush, dt: f64, intent: f64, armed: bool, tw_y: f64, tw_p: f64) {
    let n = anim_to_model_d(g, c, l.n);
    let p = anim_to_model_p(g, c, l.p);
    {
        let an = &g.ch[c].anim;
        let a = args();
        a[0] = dt; a[1] = intent; a[2] = b01(!armed && an.upper != UpperMode::Torch && an.throw_t < 0.0); a[3] = tw_y; a[4] = tw_p; a[5] = l.hand; a[6] = b01(l.reach.unwrap_or(false));
        for i in 0..3 { a[7 + i] = n[i]; a[10 + i] = p[i]; }
        for i in 0..4 { a[13 + i] = an.hand_fix[0][i]; a[17 + i] = an.hand_fix[1][i]; }
    }
    door_cols_get(g, c);
    let (slot, at, at_b, at_rest) = (slot_no(g_2), seam_pose_at(&g.ch[c].anim.p_a), seam_pose_at(&g.ch[c].anim.p_b), seam_pose_at(&g.ch[c].anim.rest));
    anim_hands_ask(g, c, &mut |_g| ::sim::layers::door_layer(slot, at, at_b, at_rest) as f64);
    door_cols_put(g, c);
}

// ---- handwork

/// Everything the three kinds of handwork keep, back to nothing. (character.ts hwReset)
pub fn anim_hw_reset(g: &mut crate::game::Game, c: CharId) {
    let an = &mut g.ch[c].anim;
    let k = &mut an.wk;
    k.last = None; k.fed = false; k.after = 0.0;
    k.w_l.x = 0.0; k.w_l.v = 0.0; k.w_r.x = 0.0; k.w_r.v = 0.0; k.tw.x = 0.0; k.tw.v = 0.0;
    k.ok_l = false; k.ok_r = false; k.shown = 0.0;
    let s = &mut an.sw;
    s.cur = None; s.t = -1.0; s.wait = -1.0;
    let r = &mut an.rx;
    r.left = 0.0; r.w.x = 0.0; r.w.v = 0.0;
    an.work = None; an.carry = None; an.sw.shown = false; an.hand_pre = None;
}

/// What the work hands are doing: the pose last fed, the shown weight, which hand, both hands? (character.ts workInfo)
pub fn anim_work_info(g: &mut crate::game::Game, c: CharId) -> crate::character::AnimWorkInfo {
    let k = &g.ch[c].anim.wk;
    crate::character::AnimWorkInfo { pose: k.last.clone(), w: k.shown, hand: k.main, two: k.two }
}

/// The verb is done: the pose carries on by itself for `secs` past its end. (character.ts workAfter)
pub fn anim_work_after(g: &mut crate::game::Game, c: CharId, secs: f64) {
    if g.ch[c].anim.wk.last.is_some() { g.ch[c].anim.wk.after = secs; }
}

/// A line over the radio: the left hand goes up for about `secs`. (character.ts radio)
pub fn anim_radio(g: &mut crate::game::Game, c: CharId, secs: f64) {
    let r = &mut g.ch[c].anim.rx;
    r.left = m::max(r.left, v3::clamp(secs, 0.8, 3.0));
}

/// 0..1: how far the radio hand is up. (character.ts radioShown)
pub fn anim_radio_shown(g: &mut crate::game::Game, c: CharId) -> f64 {
    v3::clamp(g.ch[c].anim.rx.w.x, 0.0, 1.0)
}

/// What the gun hand visibly holds: the rule's, unless a swap is running. (character.ts swapHeld)
pub fn anim_swap_held(g: &mut crate::game::Game, c: CharId, rule: Option<crate::character::AnimSwapHeldRule>) -> Option<crate::character::AnimSwapHeld> {
    let s = g.ch[c].anim.sw;
    if s.t < 0.0 {
        if let (Some(r), Some(cur)) = (rule, s.cur) {
            if cur.as_str() != r.as_str() { return Some(match cur { AnimSwCur::Torch => AnimSwapHeld::Torch, AnimSwCur::Weapon => AnimSwapHeld::Weapon }); }
        }
        return rule.map(|r| match r { AnimSwapHeldRule::Torch => AnimSwapHeld::Torch, AnimSwapHeldRule::Weapon => AnimSwapHeld::Weapon });
    }
    let a = s.t / s.dur;
    let k = swap();
    if a < k.away { s.from.map(|f| match f { AnimSwFrom::Torch => AnimSwapHeld::Torch, AnimSwFrom::Weapon => AnimSwapHeld::Weapon }) }
    else if a < k.take { None }
    else { s.to.map(|t| match t { AnimSwTo::Torch => AnimSwapHeld::Torch, AnimSwTo::Weapon => AnimSwapHeld::Weapon }) }
}

/// How far the swap's hand is at the torch on his belt this frame. (character.ts swapBelt)
pub fn anim_swap_belt(g: &mut crate::game::Game, c: CharId) -> f64 {
    g.ch[c].anim.sw.belt
}

/// The swap's progress (-1: none), what it goes to, and whether one waits. (character.ts swapInfo)
pub fn anim_swap_info(g: &mut crate::game::Game, c: CharId) -> crate::character::AnimSwapInfo {
    let s = &g.ch[c].anim.sw;
    crate::character::AnimSwapInfo {
        a: if s.t < 0.0 { -1.0 } else { s.t / s.dur },
        to: s.to.map(|t| match t { AnimSwTo::Torch => AnimSwapInfoTo::Torch, AnimSwTo::Weapon => AnimSwapInfoTo::Weapon }),
        pending: s.wait >= 0.0,
    }
}

/// The torch and sidearm swap's clock, and its arm when it shows. (character.ts swapLayer)
pub fn anim_swap_layer(g: &mut crate::game::Game, c: CharId, dt: f64, taken: bool, tw_y: f64, tw_p: f64) {
    if taken { g.ch[c].anim.sw.t = -1.0; return; }
    let weapon = g.ch[c].anim.sw.to == Some(AnimSwTo::Weapon);
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
        let n = g.ch[c].anim.p_a.num_nodes as f64;
        let p = crate::anim_skeleton::pose_new(n);
        g.ch[c].anim.sw_buf = Some(p);
    }
    // lent for the call: only its place is read, and nothing under the call reads swBuf
    let sw = g.ch[c].anim.sw_buf.take().unwrap();
    anim_swap_pose(g, c, e, k, weapon, tw_y, tw_p, &sw);
    g.ch[c].anim.sw_buf = Some(sw);
    g.ch[c].anim.sw.shown = true;
}

/// The swap's arm, by its envelope e and how far across k. (character.ts swapPose)
pub fn anim_swap_pose(g: &mut crate::game::Game, c: CharId, e: f64, k: f64, weapon: bool, tw_y: f64, tw_p: f64, sw: &crate::anim_skeleton::Pose) {
    let slot = man_slot(g, c);
    anim_swap_pose_ask(g, c, slot, e, k, weapon, tw_y, tw_p, sw);
}

/// The swap's numbers onto the bench, and the layer. (character.ts swapPoseAsk)
pub fn anim_swap_pose_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, e: f64, k: f64, weapon: bool, tw_y: f64, tw_p: f64, sw: &crate::anim_skeleton::Pose) {
    {
        let f = g.ch[c].anim.hand_fix[1];
        let a = args();
        a[0] = e; a[1] = k; a[2] = b01(weapon); a[3] = tw_y; a[4] = tw_p;
        for i in 0..4 { a[5 + i] = f[i]; }
    }
    let (slot, at, at_b, at_rest, at_sw) = (slot_no(g_2), seam_pose_at(&g.ch[c].anim.p_a), seam_pose_at(&g.ch[c].anim.p_b), seam_pose_at(&g.ch[c].anim.rest), seam_pose_at(sw));
    anim_hands_ask(g, c, &mut |_g| ::sim::layers::swap_pose(slot, at, at_b, at_rest, at_sw) as f64);
}

/// Hands at work on a thing: the gate, and the layer when it runs. (character.ts workLayer)
pub fn anim_work_layer(g: &mut crate::game::Game, c: CharId, dt: f64, free: bool, armed: bool, tw_y: f64, tw_p: f64) {
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
    let slot = man_slot(g, c);
    anim_work_layer_ask(g, c, slot, &l, dt, live, armed, tw_y, tw_p);
}

/// workCols.get onto the bench (character.ts:4055)
fn work_cols_get(g: &mut Game, c: CharId) {
    let k = &g.ch[c].anim.wk;
    let (a, o) = (args(), COLS);
    a[o] = k.w_l.x; a[o + 1] = k.w_l.v; a[o + 2] = k.w_r.x; a[o + 3] = k.w_r.v; a[o + 4] = b01(k.ok_l); a[o + 5] = b01(k.ok_r); a[o + 6] = k.main; a[o + 7] = b01(k.two); a[o + 8] = k.shown;
}
/// workCols.put off the bench (character.ts:4056)
fn work_cols_put(g: &mut Game, c: CharId) {
    let k = &mut g.ch[c].anim.wk;
    let (a, o) = (args(), COLS);
    k.w_l.x = a[o]; k.w_l.v = a[o + 1]; k.w_r.x = a[o + 2]; k.w_r.v = a[o + 3]; k.ok_l = a[o + 4] != 0.0; k.ok_r = a[o + 5] != 0.0; k.main = if js::truthy(a[o + 6]) { 1.0 } else { 0.0 }; k.two = a[o + 7] != 0.0; k.shown = a[o + 8];
}

/// The work hands' numbers onto the bench, the layer, its columns back. (character.ts workLayerAsk)
pub fn anim_work_layer_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, l: &crate::character::WorkPose, dt: f64, live: bool, armed: bool, tw_y: f64, tw_p: f64) {
    let p = anim_to_model_p(g, c, l.p);
    let n = anim_to_model_d(g, c, l.n);
    {
        let an = &g.ch[c].anim;
        let none = no_keys();
        let keys: &[[f64; 2]] = match &l.keys { Some(k) if !k.is_empty() => k.as_slice(), _ => none.as_slice() };
        let nk = if keys.len() < 4 { keys.len() } else { 4 };   // (four presses: the first four keys are all a code's hand reads)
        let r_free = l.hand != Some(0.0) && !armed && an.upper != UpperMode::Torch && (!an.long_gun || an.stowed) && (!an.grip[1] || an.stowed) && an.throw_t < 0.0 && an.sw.t < 0.0;
        let a = args();
        a[0] = dt; a[1] = b01(live); a[2] = b01(r_free); a[3] = tw_y; a[4] = tw_p; a[5] = l.t; a[6] = l.dur; a[7] = work_kind_no(l.kind); a[8] = l.dir.unwrap_or(1.0); a[26] = nk as f64;
        for i in 0..3 { a[9 + i] = p[i]; a[12 + i] = n[i]; a[15 + i] = an.gaze_local[i]; }
        for i in 0..4 { a[18 + i] = an.hand_fix[0][i]; a[22 + i] = an.hand_fix[1][i]; }
        for i in 0..nk { a[KEYS + i * 2] = keys[i][0]; a[KEYS + i * 2 + 1] = keys[i][1]; }
    }
    work_cols_get(g, c);
    let (slot, at, at_b, at_rest) = (slot_no(g_2), seam_pose_at(&g.ch[c].anim.p_a), seam_pose_at(&g.ch[c].anim.p_b), seam_pose_at(&g.ch[c].anim.rest));
    anim_hands_ask(g, c, &mut |_g| ::sim::layers::work_layer(slot, at, at_b, at_rest) as f64);
    work_cols_put(g, c);
}

/// The torch up in the hand that holds it, weight e. (character.ts torchUpLayer)
pub fn anim_torch_up_layer(g: &mut crate::game::Game, c: CharId, e: f64) {
    let slot = man_slot(g, c);
    anim_torch_up_layer_ask(g, c, slot, e);
}

/// The torch's numbers onto the bench, and the layer. (character.ts torchUpLayerAsk)
pub fn anim_torch_up_layer_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, e: f64) {
    {
        let f = g.ch[c].anim.hand_fix[1];
        let a = args();
        a[0] = e;
        for i in 0..4 { a[1 + i] = f[i]; }
    }
    let code = ::sim::layers::torch_up(slot_no(g_2), seam_pose_at(&g.ch[c].anim.p_a), seam_pose_at(&g.ch[c].anim.p_b)) as f64;
    man_ran(g, c, code);
}

/// A hand to the radio: its clock and spring, and the pose when it shows. (character.ts radioLayer)
pub fn anim_radio_layer(g: &mut crate::game::Game, c: CharId, dt: f64, free: bool) {
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

/// The hand at the radio by w, and the head's share hw. (character.ts radioPose)
pub fn anim_radio_pose(g: &mut crate::game::Game, c: CharId, w: f64, hw: f64) {
    let slot = man_slot(g, c);
    anim_radio_pose_ask(g, c, slot, w, hw);
}

/// The radio's numbers onto the bench, and the layer. (character.ts radioPoseAsk)
pub fn anim_radio_pose_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, w: f64, hw: f64) {
    {
        let f = g.ch[c].anim.hand_fix[0];
        let a = args();
        a[0] = w; a[1] = hw;
        for i in 0..4 { a[2 + i] = f[i]; }
    }
    let (slot, at, at_b, at_rest) = (slot_no(g_2), seam_pose_at(&g.ch[c].anim.p_a), seam_pose_at(&g.ch[c].anim.p_b), seam_pose_at(&g.ch[c].anim.rest));
    anim_hands_ask(g, c, &mut |_g| ::sim::layers::radio_pose(slot, at, at_b, at_rest) as f64);
}

// ---- holding a man, and being held

/// One man holding another from behind, weight w. (character.ts holdLayer)
pub fn anim_hold_layer(g: &mut crate::game::Game, c: CharId, w: f64, dt: f64) {
    let slot = man_slot(g, c);
    anim_hold_layer_ask(g, c, slot, w, dt);
}

/// holdCols.get onto the bench (character.ts:4125)
fn hold_cols_get(g: &mut Game, c: CharId) {
    let an = &g.ch[c].anim;
    let (a, o) = (args(), COLS);
    let s = [&an.hd_kn, &an.hd_kc, &an.hd_ck, &an.hd_lo, &an.hd_hu];
    for i in 0..5 { a[o + i * 2] = s[i].x; a[o + i * 2 + 1] = s[i].v; }
    a[o + 10] = an.hd_sw; a[o + 11] = b01(an.hd_from.is_some());
    for i in 0..3 { a[o + 12 + i] = match an.hd_from { Some(f) => f[i], None => 0.0 }; }
    a[o + 15] = b01(an.hd_caught);
    a[o + 16] = b01(an.hd_fix.is_some());
    for i in 0..3 { a[o + 17 + i] = match &an.hd_fix { Some(x) => x.tgt[i], None => 0.0 }; a[o + 20 + i] = match &an.hd_fix { Some(x) => x.pole[i], None => 0.0 }; }
    a[o + 23] = match &an.hd_fix { Some(x) => x.pole_w, None => 0.0 };
    for i in 0..4 { a[o + 24 + i] = match &an.hd_fix { Some(x) => x.rot[i], None => 0.0 }; }
    a[o + 28] = match &an.hd_fix { Some(x) => x.w, None => 0.0 };
    a[o + 29] = match &an.hd_fix { Some(x) => x.rw, None => 0.0 };
}
/// holdCols.put off the bench (character.ts:4128)
fn hold_cols_put(g: &mut Game, c: CharId) {
    let an = &mut g.ch[c].anim;
    let (a, o) = (args(), COLS);
    {
        let s = [&mut an.hd_kn, &mut an.hd_kc, &mut an.hd_ck, &mut an.hd_lo, &mut an.hd_hu];
        for (i, sp) in s.into_iter().enumerate() { sp.x = a[o + i * 2]; sp.v = a[o + i * 2 + 1]; }
    }
    an.hd_sw = a[o + 10];
    an.hd_from = if js::truthy(a[o + 11]) { Some([a[o + 12], a[o + 13], a[o + 14]]) } else { None };
    an.hd_caught = a[o + 15] != 0.0;
    an.hd_fix = if js::truthy(a[o + 16]) {
        Some(crate::character::AnimHdFix { tgt: [a[o + 17], a[o + 18], a[o + 19]], pole: [a[o + 20], a[o + 21], a[o + 22]], pole_w: a[o + 23], rot: [a[o + 24], a[o + 25], a[o + 26], a[o + 27]], w: a[o + 28], rw: a[o + 29] })
    } else { None };
}

/// The hold's numbers onto the bench, the layer, its columns back. (character.ts holdLayerAsk)
pub fn anim_hold_layer_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, w: f64, dt: f64) {
    let (hold_pose, h) = { let an = &g.ch[c].anim; (an.hold_pose, an.hold_pose.unwrap_or(an.hd_last)) };
    let sp = match &hold_pose { Some(hp) => hp.strip, None => None };
    let man = match h.man { Some(at) => Some(anim_to_model_p(g, c, at)), None => None };
    let sa = match sp { Some(s) => match s.at { Some(at) => Some(anim_to_model_p(g, c, at)), None => None }, None => None };
    {
        let show = g.m.character.hold_show;
        let an = &g.ch[c].anim;
        let (rd, u) = (an.rd_hand_l, an.grip_up);
        let a = args();
        a[0] = w; a[1] = dt; a[2] = hold_no(h.phase); a[3] = h.t; a[4] = b01(h.variant == Some(HoldVariant::Gun)); a[5] = an.hd_gv.x; a[6] = b01(an.no_knife); a[7] = b01(hold_pose.is_some());
        a[8] = b01(h.kill.unwrap_or(show.kill)); a[9] = b01(h.beats.unwrap_or(show.beats)); a[10] = h.low.unwrap_or(0.0); a[11] = b01(h.haul.unwrap_or(false));
        a[12] = b01(h.yank.is_some()); a[13] = h.yank.unwrap_or(0.0); a[14] = b01(h.step.is_some()); a[15] = h.step.unwrap_or(0.0); a[16] = b01(man.is_some()); a[20] = b01(h.pull.is_some()); a[21] = h.pull.unwrap_or(0.0); a[22] = an.idle_t;
        a[23] = b01(sp.is_some()); a[24] = match sp { Some(s) => s.t, None => 0.0 }; a[25] = b01(sa.is_some()); a[29] = an.aim_yaw_offset; a[30] = -an.aim_pitch * an.upper_w.aim * 0.35;
        a[31] = h.gun_out.unwrap_or(0.0); a[32] = h.gun_pitch.unwrap_or(0.0); a[33] = an.shoot_t; a[34] = b01(rd.is_some());
        for i in 0..3 {
            a[17 + i] = match man { Some(v) => v[i], None => 0.0 };
            a[26 + i] = match sa { Some(v) => v[i], None => 0.0 };
            a[35 + i] = match rd { Some(v) => v[i], None => 0.0 };
            a[46 + i] = u[i];
        }
        for i in 0..4 { a[38 + i] = an.hand_fix[0][i]; a[42 + i] = an.hand_fix[1][i]; }
    }
    hold_cols_get(g, c);
    let an = &g.ch[c].anim;
    let (slot, at, at_b, at_c, at_aim, at_rest) = (slot_no(g_2), seam_pose_at(&an.p_a), seam_pose_at(&an.p_b), seam_pose_at(&an.p_c), seam_pose_at(&an.p_aim), seam_pose_at(&an.rest));
    anim_hands_ask(g, c, &mut |_g| ::sim::layers::hold_layer(slot, at, at_b, at_c, at_aim, at_rest) as f64);
    hold_cols_put(g, c);
}

/// The held man, weight w. (character.ts heldLayer)
pub fn anim_held_layer(g: &mut crate::game::Game, c: CharId, w: f64, dt: f64) {
    let slot = man_slot(g, c);
    anim_held_layer_ask(g, c, slot, w, dt);
}

/// heldCols.get onto the bench (character.ts:4159)
fn held_cols_get(g: &mut Game, c: CharId) {
    let an = &g.ch[c].anim;
    let (a, o) = (args(), COLS);
    let s = [&an.he_ck, &an.he_kc, &an.he_hu, &an.he_lk, &an.he_lo, &an.he_bo, &an.he_bc];
    for i in 0..7 { a[o + i * 2] = s[i].x; a[o + i * 2 + 1] = s[i].v; }
    a[o + 14] = an.he_sw; a[o + 15] = an.he_span;
    for sd in 0..2 {
        let q = an.he_hr[sd];
        a[o + 16 + sd * 5] = b01(q.is_some());
        for i in 0..4 { a[o + 17 + sd * 5 + i] = match q { Some(v) => v[i], None => 0.0 }; }
    }
}
/// heldCols.put off the bench (character.ts:4161)
fn held_cols_put(g: &mut Game, c: CharId) {
    let an = &mut g.ch[c].anim;
    let (a, o) = (args(), COLS);
    {
        let s = [&mut an.he_ck, &mut an.he_kc, &mut an.he_hu, &mut an.he_lk, &mut an.he_lo, &mut an.he_bo, &mut an.he_bc];
        for (i, sp) in s.into_iter().enumerate() { sp.x = a[o + i * 2]; sp.v = a[o + i * 2 + 1]; }
    }
    an.he_sw = a[o + 14]; an.he_span = a[o + 15];
    for sd in 0..2 {
        let k = o + 16 + sd * 5;
        an.he_hr[sd] = if js::truthy(a[k]) { Some([a[k + 1], a[k + 2], a[k + 3], a[k + 4]]) } else { None };
    }
}

/// The held man's numbers onto the bench, the layer, its columns back. (character.ts heldLayerAsk)
pub fn anim_held_layer_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, w: f64, dt: f64) {
    {
        let show = g.m.character.hold_show;
        let an = &g.ch[c].anim;
        let h = an.held_pose.unwrap_or(an.he_last);
        let sp = match &an.held_pose { Some(hp) => hp.strip, None => None };
        let a = args();
        a[0] = w; a[1] = dt; a[2] = hold_no(h.phase); a[3] = h.t; a[4] = h.agitation; a[5] = b01(h.variant == Some(HoldVariant::Gun)); a[6] = an.he_gv.x; a[7] = b01(h.bare.unwrap_or(false)); a[8] = b01(an.held_pose.is_some());
        a[9] = b01(h.kill.unwrap_or(show.kill)); a[10] = b01(h.beats.unwrap_or(show.beats)); a[11] = h.low.unwrap_or(0.0); a[12] = b01(h.haul.unwrap_or(false)); a[13] = b01(h.yank.is_some()); a[14] = h.yank.unwrap_or(0.0); a[15] = h.pull.unwrap_or(0.0);
        a[16] = b01(match h.shot { Some(s) => s >= 0.0, None => false }); a[17] = h.shot.unwrap_or(0.0); a[18] = b01(h.armed.unwrap_or(false)); a[19] = an.idle_t; a[20] = b01(sp.is_some()); a[21] = match sp { Some(s) => s.t, None => 0.0 };
        for i in 0..4 { a[22 + i] = an.hand_fix[0][i]; a[26 + i] = an.hand_fix[1][i]; }
    }
    held_cols_get(g, c);
    let an = &g.ch[c].anim;
    let (slot, at, at_b, at_rest) = (slot_no(g_2), seam_pose_at(&an.p_a), seam_pose_at(&an.p_b), seam_pose_at(&an.rest));
    anim_hands_ask(g, c, &mut |_g| ::sim::layers::held_layer(slot, at, at_b, at_rest) as f64);
    held_cols_put(g, c);
}

/// Out of a hold, with the layer still on its way out: has he been told to do anything since it let go? Latched until the next hold. (character.ts holdBusy)
pub fn anim_hold_busy(g: &mut crate::game::Game, c: CharId) -> bool {
    let an = &mut g.ch[c].anim;
    if an.hold_pose.is_some() {
        an.hd_idle.upper = an.upper; an.hd_idle.long = an.long_gun; an.hd_idle.crouch = an.crouch_target;
        an.hd_idle.carry = an.carry.map(|k| match k { AnimCarry::Torch => AnimHdIdleCarry::Torch, AnimCarry::Weapon => AnimHdIdleCarry::Weapon });
        an.hd_busy = false;
        return false;
    }
    if an.hd_busy || an.hd_w.x <= 0.003 { return an.hd_busy; }
    let i = &an.hd_idle;
    an.hd_busy = an.upper != i.upper || an.long_gun != i.long || an.crouch_target != i.crouch || an.carry.map(|k| k.as_str()) != i.carry.map(|k| k.as_str())
        || an.shoot_t >= 0.0 || an.reload_t >= 0.0 || an.throw_t >= 0.0 || an.strike_t >= 0.0 || an.hit_t >= 0.0 || an.kick_t >= 0.0 || an.sig_t >= 0.0 || an.bw.is_some() || an.kit.is_some()
        || an.lockpick || an.haul.is_some() || an.work.is_some() || an.door_push.is_some() || an.stance != Stance::None || an.ready.is_some() || an.creep || an.wound.is_some() || an.act.is_some();
    an.hd_busy
}

// ---- the big tool

/// The big slot's tool in his hands: the way out that poses nothing, then the layer. (character.ts bigLayer)
pub fn anim_big_layer(g: &mut crate::game::Game, c: CharId, b: &crate::character::AnimBigTool, tw_y: f64, tw_p: f64) {
    let stowing = b.end >= 0.0 && b.t >= b.end;
    let us = if stowing { v3::clamp((b.t - b.end) / m::max(1e-3, b.stow), 0.0, 1.0) } else { 0.0 };
    let ham = b.kind == AnimBigToolKind::Hammer;
    if !(b.t >= 0.0) || us >= 1.0 { g.ch[c].anim.big_under = false; g.ch[c].anim.big_k = 0.0; return; }
    if ham && g.ch[c].anim.big_buf.is_none() {
        let n = g.ch[c].anim.p_a.num_nodes as f64;
        let p = crate::anim_skeleton::pose_new(n);
        g.ch[c].anim.big_buf = Some(p);
    }
    let slot = man_slot(g, c);
    let near = blows_near(&crate::character::BlowsNearB { t: b.t, end: b.end, blows: b.blows.clone() });
    anim_big_layer_ask(g, c, slot, b, &near, tw_y, tw_p);
}

/// bigCols.get onto the bench (character.ts:4210)
fn big_cols_get(g: &mut Game, c: CharId) {
    let an = &g.ch[c].anim;
    let (a, o) = (args(), COLS);
    a[o] = an.big_k; a[o + 1] = b01(an.big_under); a[o + 2] = an.big_at_back; a[o + 3] = an.big_grab;
}
/// bigCols.put off the bench (character.ts:4211)
fn big_cols_put(g: &mut Game, c: CharId) {
    let an = &mut g.ch[c].anim;
    let (a, o) = (args(), COLS);
    an.big_k = a[o]; an.big_under = a[o + 1] != 0.0; an.big_at_back = a[o + 2]; an.big_grab = a[o + 3];
}

/// The big tool's numbers onto the bench, the layer, its columns back. `blows`: the blows the layer reads. (character.ts bigLayerAsk)
pub fn anim_big_layer_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, b: &crate::character::AnimBigTool, blows: &[f64], tw_y: f64, tw_p: f64) {
    let ham = b.kind == AnimBigToolKind::Hammer;
    let p = if (b.at[0] + b.at[1] + b.at[2]).is_finite() { anim_to_model_p(g, c, b.at) } else { [0.0, 1.0, 0.8] };
    {
        let an = &g.ch[c].anim;
        let a = args();
        a[0] = b01(ham); a[1] = b.t; a[2] = b.equip; a[3] = b.stow; a[4] = b.end; a[5] = b01(b.snug.unwrap_or(false)); a[6] = b.windup; a[7] = blows.len() as f64;
        a[11] = tw_y; a[12] = tw_p; a[13] = an.speed; a[14] = an.big_clear;
        for i in 0..3 { a[8 + i] = p[i]; a[36 + i] = an.gaze_local[i]; }
        for i in 0..4 { a[15 + i] = an.hand_fix[0][i]; a[19 + i] = an.hand_fix[1][i]; }
        // (the shotgun is drawn as the carbine it is: its support grip, its group's offset and its stock's rear end, as rigProps has them now)
        if ham { for i in 23..36 { a[i] = 0.0; } }
        else {
            let k = crate::rig_props::rig().carbine;
            let adj = match &k.adj { Some(x) => x.off, None => [0.0, 0.0, 0.0] };
            let lg = crate::rig_props::carbine_grip(None);
            for i in 0..3 { a[23 + i] = lg.wrist[i]; a[26 + i] = lg.fingers[i]; a[29 + i] = lg.back[i]; a[32 + i] = adj[i]; }
            a[35] = k.stock_top.l0;
        }
        for i in 0..blows.len() { a[KEYS + i] = blows[i]; }
    }
    big_cols_get(g, c);
    let an = &g.ch[c].anim;
    let at_buf = if ham { seam_pose_at(an.big_buf.as_ref().unwrap()) } else { 0 };
    let (slot, at, at_b, at_rest) = (slot_no(g_2), seam_pose_at(&an.p_a), seam_pose_at(&an.p_b), seam_pose_at(&an.rest));
    anim_hands_ask(g, c, &mut |_g| ::sim::layers::big_layer(slot, at, at_b, at_buf, at_rest) as f64);
    big_cols_put(g, c);
}

// ---- the throw

/// The overhand throw at t. (character.ts throwLayer)
pub fn anim_throw_layer(g: &mut crate::game::Game, c: CharId, t: f64) {
    let slot = man_slot(g, c);
    let tables = throw_tables(g.ch[c].anim.thr_low >= 0.0);   // (the swing's tables in the module's order)
    anim_throw_layer_ask(g, c, slot, t, &tables.k1, &tables.k3);
}

/// throwCols.get onto the bench (character.ts:4245)
fn throw_cols_get(g: &mut Game, c: CharId) {
    let an = &g.ch[c].anim;
    let (a, o) = (args(), COLS);
    a[o] = an.thr_hang; a[o + 1] = b01(an.thr_last.is_some());
    for i in 0..3 {
        a[o + 2 + i] = match &an.thr_last { Some(l) => l.r[i], None => 0.0 };
        a[o + 5 + i] = match &an.thr_last { Some(l) => l.l[i], None => 0.0 };
    }
}
/// throwCols.put off the bench (character.ts:4246)
fn throw_cols_put(g: &mut Game, c: CharId) {
    let an = &mut g.ch[c].anim;
    let (a, o) = (args(), COLS);
    an.thr_hang = a[o];
    an.thr_last = if js::truthy(a[o + 1]) { Some(crate::character::AnimThrLast { r: [a[o + 2], a[o + 3], a[o + 4]], l: [a[o + 5], a[o + 6], a[o + 7]] }) } else { None };
}

/// The swing's tables, the throw's numbers onto the bench, the layer, its columns back: seventeen counts, then eleven tables of (time, value) keys and six of (time, vector). (character.ts throwLayerAsk)
pub fn anim_throw_layer_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, t: f64, k1: &[Vec<[f64; 2]>], k3: &[Vec<(f64, Vec<f64>)>]) {
    {
        let an = &g.ch[c].anim;
        let a = args();
        let mut q = KEYS + 17;
        for i in 0..11 {
            a[KEYS + i] = k1[i].len() as f64;
            for k in k1[i].iter() { a[q] = k[0]; a[q + 1] = k[1]; q += 2; }
        }
        for i in 0..6 {
            a[KEYS + 11 + i] = k3[i].len() as f64;
            for k in k3[i].iter() { a[q] = k.0; a[q + 1] = k.1[0]; a[q + 2] = k.1[1]; a[q + 3] = k.1[2]; q += 4; }
        }
        let cin = &an.thr_carry;
        a[0] = t; a[1] = an.thr_low; a[2] = an.thr_far; a[3] = b01(an.thr_off); a[4] = an.mvs.cf.x; a[5] = an.mvs.dz.x; a[6] = an.thr_hit0; a[7] = an.crouch; a[8] = an.speed; a[9] = b01(cin.is_some());
        for i in 0..3 {
            a[10 + i] = match cin { Some(k) => k.r[i], None => 0.0 };
            a[13 + i] = match cin { Some(k) => k.l[i], None => 0.0 };
        }
        for i in 0..4 { a[16 + i] = an.hand_fix[0][i]; a[20 + i] = an.hand_fix[1][i]; }
    }
    throw_cols_get(g, c);
    let an = &g.ch[c].anim;
    let (slot, at, at_b, at_rest) = (slot_no(g_2), seam_pose_at(&an.p_a), seam_pose_at(&an.p_b), seam_pose_at(&an.rest));
    anim_hands_ask(g, c, &mut |_g| ::sim::layers::throw_layer(slot, at, at_b, at_rest) as f64);
    throw_cols_put(g, c);
}

// ---- the capsules' records

/// One capsule record [a, r, b] of Character.capsules: the segment a to c, its far end carried `extend_b` further along it. (character.ts capRec)
pub fn cap_rec(a: V3, c_2: V3, r: f64, extend_b: Option<f64>) -> Vec<f64> {
    let extend_b = extend_b.unwrap_or(0.0);
    let (mut bx, mut by, mut bz) = (c_2[0], c_2[1], c_2[2]);
    if extend_b > 0.0 {
        let (dx, dy, dz) = (c_2[0] - a[0], c_2[1] - a[1], c_2[2] - a[2]);
        let l = js::or(m::hyp3(dx, dy, dz), 1.0);
        bx += dx / l * extend_b; by += dy / l * extend_b; bz += dz / l * extend_b;
    }
    vec![a[0], a[1], a[2], r, bx, by, bz]
}

/// The shoe's capsule: heel to past the toe along the sole. (character.ts capSole)
pub fn cap_sole(foot: V3, toe: V3) -> Vec<f64> {
    let up = TOE_FLOOR_Y;
    let heel: V3 = [foot[0] - (toe[0] - foot[0]) * 0.35, m::min(foot[1], toe[1] - up + 0.033 + (foot[1] - toe[1]) * 0.15), foot[2] - (toe[2] - foot[2]) * 0.35];
    let tip: V3 = [toe[0], toe[1] - up + 0.033, toe[2]];
    cap_rec(heel, tip, 0.033, Some(0.06))
}

#[cfg(test)]
mod tests {
    use super::*;
    fn bits(got: Vec<f64>, want: [u64; 7]) {
        for i in 0..7 { assert_eq!(got[i].to_bits(), want[i], "word {}", i); }
    }
    /// the TypeScript's own bits, run under bun with EXACT.on
    #[test]
    fn cap_records_match_the_typescript() {
        bits(cap_rec([0.1, 1.3, -2.7], [0.4, 0.9, -2.2], 0.07, None), [0x3fb999999999999a, 0x3ff4cccccccccccd, 0xc00599999999999a, 0x3fb1eb851eb851ec, 0x3fd999999999999a, 0x3feccccccccccccd, 0xc00199999999999a]);
        bits(cap_rec([0.1, 1.3, -2.7], [0.4, 0.9, -2.2], 0.07, Some(0.11)), [0x3fb999999999999a, 0x3ff4cccccccccccd, 0xc00599999999999a, 0x3fb1eb851eb851ec, 0x3fdc9639c5d97fb4, 0x3feacf0caf4cde11, 0xc000fa4d90619eff]);
        bits(cap_rec([1.0, 2.0, 3.0], [1.0, 2.0, 3.0], 0.05, Some(0.2)), [0x3ff0000000000000, 0x4000000000000000, 0x4008000000000000, 0x3fa999999999999a, 0x3ff0000000000000, 0x4000000000000000, 0x4008000000000000]);
        bits(cap_sole([3.17, 0.1013, -4.41], [3.29, 0.0151, -4.26]), [0x40090624dd2f1aa0, 0x3fa7b890d5a5b962, 0xc011d9999999999a, 0x3fa0e5604189374c, 0x400a9e96588d9d12, 0x3f9f245a96b797fa, 0xc010da52ac7e8812]);
        bits(cap_sole([-0.33, 0.4, 7.7], [-0.21, 0.52, 7.81]), [0xbfd7ced916872b02, 0x3fd999999999999a, 0x401ea5604189374c, 0x3fa0e5604189374c, 0xbfc616a7c0701766, 0x3fe23f8c0fd57294, 0x401f6093e3506a6c]);
    }
}
