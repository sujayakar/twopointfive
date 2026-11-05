//! src/game/character.ts lines 3539 to 3981
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

// ---- the bench, and what is private to this part

/// where a layer's columns go on the bench's `args` (skeleton.ts LAYER_COLS_AT)
const COLS: usize = crate::anim_skeleton::LAYER_COLS_AT as usize;
/// where a layer's path of keys goes on the bench's `args` (skeleton.ts LAYER_KEYS_AT)
const KEYS: usize = crate::anim_skeleton::LAYER_KEYS_AT as usize;
/// RD_N (character.ts:1277): how many eased numbers the ready pose keeps
const RD: usize = crate::character::RD_N as usize;

/// the bench's `args`: a layer's own numbers from 0, its columns from COLS (`f64()` at `POSE_BENCH.args`). Never held across a call that may use the bench.
fn args() -> &'static mut [f64; sim::skeleton::ARGS] { &mut sim::skeleton::bench().args }
/// a boolean as the bench holds it
fn b01(x: bool) -> f64 { if x { 1.0 } else { 0.0 } }
/// the slot a layer is asked in: `layer_slot`'s -1 is the bench (`-1.0 as usize` would be game 0)
fn slot_no(g_2: f64) -> usize { if g_2 < 0.0 { sim::skeleton::ON_BENCH } else { g_2 as usize } }
/// BLOW_NO[style] (character.ts:1107)
fn blow_no_of(s: BlowStyle) -> f64 { *crate::character::blow_no().get(String::from(s.as_str())).unwrap() }
/// `x ? KIT_NO[x] : 0` (character.ts:1138, read at 3748)
fn kit_no_of(x: Option<KitItem>) -> f64 {
    match x { Some(k) => *crate::character::kit_no().get(String::from(k.as_str())).unwrap(), None => 0.0 }
}
/// KIT_OF[n] (character.ts:1138), the table's word as the enum
fn kit_item_of(n: f64) -> Option<KitItem> {
    let w = crate::character::kit_of()[n as usize].clone()?;
    [KitItem::Pistol, KitItem::Rifle, KitItem::Can, KitItem::Knife].into_iter().find(|k| k.as_str() == w)
}
/// `layerRan(code, this.fk)`: fk is lent out of the man for the call, which reads only the bench and its own lists
fn layer_ran_of(g: &mut Game, c: CharId, code: f64) {
    let mut fk = core::mem::replace(&mut g.ch[c].anim.fk, crate::anim_skeleton::PoseFK::blank());
    crate::anim_skeleton::layer_ran(g, code, &mut fk);
    g.ch[c].anim.fk = fk;
}
/// `layerSlot(this.ch, this.pA, this.cast)`: pose and cast are lent out of the man for the call (layer_slot reads only its arguments)
fn layer_slot_p_a(g: &mut Game, c: CharId) -> f64 {
    let p = core::mem::replace(&mut g.ch[c].anim.p_a, crate::anim_skeleton::Pose::blank());
    let slot = layer_slot_of(g, c, &p);
    g.ch[c].anim.p_a = p;
    slot
}
/// `layerSlot(this.ch, pose, this.cast)`
fn layer_slot_of(g: &mut Game, c: CharId, pose: &crate::anim_skeleton::Pose) -> f64 {
    let rig = g.ch[c].anim.rig;
    let cast = core::mem::replace(&mut g.ch[c].anim.cast, crate::anim_skeleton::LayerCast::blank());
    let slot = crate::anim_skeleton::layer_slot(g, rig, pose, &cast);
    g.ch[c].anim.cast = cast;
    slot
}
/// the places of pA, pB and rest, as a layer that works on a copy is handed them
fn places(g: &Game, c: CharId) -> (usize, usize, usize) {
    let a = &g.ch[c].anim;
    (seam_pose_at(&a.p_a), seam_pose_at(&a.p_b), seam_pose_at(&a.rest))
}

// ---- the seams

/// `pose.at`: where a pose stands in the game's block
fn seam_pose_at(p: &crate::anim_skeleton::Pose) -> usize { crate::anim_skeleton::b_01::stage_pose(p) } // SKELETON-GAP: Pose.at (skeleton.ts:145): the bring-up stages the pose, anim_skeleton/b_01.rs
/// `this.kitHold = v`: None = undefined, Some(None) = null. Readers tell them apart
fn seam_kit_hold_put(g: &mut Game, c: CharId, v: Option<Option<KitItem>>) { todo!("needs: Anim.kit_hold as Option<Option<KitItem>>: its readers tell undefined from null (TS 3742, 2517, kitanim.ts:191, :217, :264, player.ts:805, irlaser.ts:89)") }

// ---- the held gait

/// The held gait's share this frame; sets crV, the speed the root itself made since the last frame. (character.ts CharacterAnimator.creepGait)
pub fn anim_creep_gait(g: &mut crate::game::Game, c: CharId, dt: f64) -> f64 {
    if g.ch[c].anim.cr_t <= 0.0 && m::abs(g.ch[c].anim.speed) < 0.05 {
        let pl = anim_creep_plant(g, c);
        let a = &mut g.ch[c].anim;
        let p = pl[if a.crouch_target > 0.5 { 1 } else { 0 }];
        a.phase = js::rem(p + m::round((a.phase - p) * 2.0) / 2.0 + 2.0, 1.0);
    }
    let (gait_in, gait_out) = { let k = crate::character::creep(); (k.gait_in, k.gait_out) };
    let rate = if g.ch[c].anim.creep { dt / gait_in } else { -dt / gait_out };
    g.ch[c].anim.cr_t = v3::clamp(g.ch[c].anim.cr_t + rate, 0.0, 1.0);
    let pp = g.ch[c].anim.life.prev_pos;
    let p0 = g.ch[c].pos;
    let d = if pp[0].is_finite() { m::hyp2(p0[0] - pp[0], p0[2] - pp[2]) } else { 0.0 };
    let a = &mut g.ch[c].anim;
    a.cr_v = if dt > 0.0 && d <= 0.5 { m::min(d / dt, 8.0) } else { 0.0 };
    v3::smoothstep(0.0, 1.0, a.cr_t)
}

/// [walking, crouch-walking]: the phase, mod half a cycle, at which the clip stands widest with both feet down; found once. (character.ts CharacterAnimator.creepPlant)
pub fn anim_creep_plant(g: &mut crate::game::Game, c: CharId) -> [f64; 2] {
    if let Some(p) = g.ch[c].anim.cr_ph { return p; }
    let walk = *g.ch[c].anim.clips.get("Walk_Loop").unwrap();
    let crouch = *g.ch[c].anim.clips.get("Crouch_Fwd_Loop").unwrap();
    let a = creep_plant_find(g, c, walk, crate::character::WALK_PHASE);
    let b = creep_plant_find(g, c, crouch, 0.0);
    let ph = [a, b];
    g.ch[c].anim.cr_ph = Some(ph);
    ph
}

/// the closure `find` of creepPlant. pC and the legs' list are lent out of the man for the walk: fkPart reads them as arguments only
fn creep_plant_find(g: &mut Game, c: CharId, clip: ClipId, off: f64) -> f64 {
    let (toe_l, toe_r) = (g.ch[c].anim.nodes.toe_l, g.ch[c].anim.nodes.toe_r);
    let mut pc = core::mem::replace(&mut g.ch[c].anim.p_c, crate::anim_skeleton::Pose::blank());
    let legs = core::mem::take(&mut g.ch[c].anim.fk_legs);
    let mut best = 0.0;
    let mut score = -m::INF;
    for i in 0..96 {
        let dur = g.tab.clip[clip].duration;
        crate::anim_skeleton::sample_clip(&g.tab.clip[clip], (i as f64 / 96.0 + off) * dur, true, &mut pc);
        anim_fk_part(g, c, &pc, &legs);
        let pos = &g.ch[c].anim.fk.pos;
        let (a, b) = (pos[toe_l], pos[toe_r]);
        let sc = m::abs(a[2] - b[2]) - 4.0 * m::abs(a[1] - b[1]);
        if sc > score { score = sc; best = i as f64 / 96.0; }
    }
    g.ch[c].anim.p_c = pc;
    g.ch[c].anim.fk_legs = legs;
    js::rem(best, 0.5)
}

// ---- WATCH

/// WATCH: the slip's body and arm and the look, both points taken through the twist's inverse first. (character.ts CharacterAnimator.watchLayer)
pub fn anim_watch_layer(g: &mut crate::game::Game, c: CharId, dt: f64, free: f64, w_l: f64, w_r: f64, tw_y: f64, tw_p: f64) {
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

/// watchCols.get onto the bench (character.ts:3578)
fn watch_cols_get(g: &Game, c: CharId) {
    let t = g.ch[c].anim.wt;
    let v = &mut args()[COLS..];
    v[0] = t.sw; v[1] = t.sx; v[2] = t.sz; v[3] = t.side; v[4] = t.sl; v[5] = t.w; v[6] = t.yaw; v[7] = t.pitch;
}
/// watchCols.put off the bench (character.ts:3579)
fn watch_cols_put(g: &mut Game, c: CharId) {
    let v = &args()[COLS..];
    let t = &mut g.ch[c].anim.wt;
    t.sw = v[0]; t.sx = v[1]; t.sz = v[2]; t.side = if js::truthy(v[3]) { 1.0 } else { 0.0 }; t.sl = v[4]; t.w = v[5]; t.yaw = v[6]; t.pitch = v[7];
}

/// The slip's body: pelvis and trunk; `m` the man he gives room to, in the model. (character.ts CharacterAnimator.watchSlip)
pub fn anim_watch_slip(g: &mut crate::game::Game, c: CharId, dt: f64, free: f64, m: Option<V3>) {
    let slot = layer_slot_p_a(g, c);
    anim_watch_slip_ask(g, c, slot, dt, free, m);
}

/// (character.ts CharacterAnimator.watchSlipAsk)
pub fn anim_watch_slip_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, dt: f64, free: f64, m: Option<V3>) {
    {
        let an = &g.ch[c].anim;
        let a = args();
        let p = m.unwrap_or([0.0; 3]);
        a[0] = dt; a[1] = free; a[2] = b01(m.is_some()); a[3] = p[0]; a[4] = p[1]; a[5] = p[2]; a[6] = an.slip_k; a[7] = an.crouch; a[8] = an.slip_lean;
    }
    watch_cols_get(g, c);
    let code = ({ let ran__ = sim::layers::watch_slip(slot_no(g_2), seam_pose_at(&g.ch[c].anim.p_a)); crate::anim_skeleton::b_01::stage_back(); ran__ }) as f64;
    layer_ran_of(g, c, code);
    watch_cols_put(g, c);
}

/// The slip's arm, the one on the man's side out low toward him. (character.ts CharacterAnimator.watchArm)
pub fn anim_watch_arm(g: &mut crate::game::Game, c: CharId, w_l: f64, w_r: f64) {
    if !(g.ch[c].anim.wt.sw > 0.003) { return; }
    let slot = layer_slot_p_a(g, c);
    anim_watch_arm_ask(g, c, slot, w_l, w_r);
}

/// (character.ts CharacterAnimator.watchArmAsk)
pub fn anim_watch_arm_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, w_l: f64, w_r: f64) {
    {
        let an = &g.ch[c].anim;
        let (t, f0, f1) = (an.wt, an.hand_fix[0], an.hand_fix[1]);
        let a = args();
        a[0] = w_l; a[1] = w_r; a[2] = t.sw; a[3] = t.sl; a[4] = t.side; a[5] = t.sx; a[6] = t.sz;
        for i in 0..4 { a[7 + i] = f0[i]; a[11 + i] = f1[i]; }
    }
    let (slot, at, at_b, at_rest) = { let p = places(g, c); (slot_no(g_2), p.0, p.1, p.2) };
    anim_hands_ask(g, c, &mut |_g: &mut Game| ({ let ran__ = sim::layers::watch_arm(slot, at, at_b, at_rest); crate::anim_skeleton::b_01::stage_back(); ran__ }) as f64);
}

/// The look: the chest a little, the neck, the head. (character.ts CharacterAnimator.watchLook)
pub fn anim_watch_look(g: &mut crate::game::Game, c: CharId, dt: f64, free: f64, m: Option<V3>) {
    let slot = layer_slot_p_a(g, c);
    anim_watch_look_ask(g, c, slot, dt, free, m);
}

/// (character.ts CharacterAnimator.watchLookAsk)
pub fn anim_watch_look_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, dt: f64, free: f64, m: Option<V3>) {
    {
        let an = &g.ch[c].anim;
        let gl = an.gaze_local;
        let a = args();
        let p = m.unwrap_or([0.0; 3]);
        a[0] = dt; a[1] = free; a[2] = b01(m.is_some()); a[3] = p[0]; a[4] = p[1]; a[5] = p[2]; a[6] = an.watch_k; a[7] = gl[0]; a[8] = gl[1]; a[9] = gl[2];
    }
    watch_cols_get(g, c);
    let code = ({ let ran__ = sim::layers::watch_look(slot_no(g_2), seam_pose_at(&g.ch[c].anim.p_a)); crate::anim_skeleton::b_01::stage_back(); ran__ }) as f64;
    layer_ran_of(g, c, code);
    watch_cols_put(g, c);
}

// ---- the close-in carriage

/// The close-in mode's carriage, weight `w`: the body, then the arms. (character.ts CharacterAnimator.creepLayer)
pub fn anim_creep_layer(g: &mut crate::game::Game, c: CharId, w: f64, w_l: f64, w_r: f64, tw_y: f64, tw_p: f64, dt: f64) {
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

/// Its arms: the shoulders rounded and the hands brought up ahead of the hips by IK on a copy. (character.ts CharacterAnimator.creepArms)
pub fn anim_creep_arms(g: &mut crate::game::Game, c: CharId, w: f64, w_l: f64, w_r: f64, e: f64, cr: f64, ck: f64, cke: f64, ws: f64, ha: f64, hg: f64) {
    anim_sheath_rest_inv(g, c);
    let slot = layer_slot_p_a(g, c);
    anim_creep_arms_ask(g, c, slot, w, w_l, w_r, e, cr, ck, cke, ws, ha, hg);
}

/// (character.ts CharacterAnimator.creepArmsAsk)
pub fn anim_creep_arms_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, w: f64, w_l: f64, w_r: f64, e: f64, cr: f64, ck: f64, cke: f64, ws: f64, ha: f64, hg: f64) {
    let rc = anim_sheath_rest_inv(g, c);
    {
        let an = &g.ch[c].anim;
        let (w_s, w_p, f0, f1) = (an.wide_shape, an.wide_pole, an.hand_fix[0], an.hand_fix[1]);
        let a = args();
        a[0] = w; a[1] = w_l; a[2] = w_r; a[3] = e; a[4] = cr; a[5] = ck; a[6] = cke; a[7] = ws; a[8] = ha; a[9] = hg; a[10] = an.cl_t;
        a[11] = b01(an.grip[1]); a[12] = b01(an.no_knife); a[13] = an.crouch; a[14] = an.heft_shape[1]; a[19] = b01(w_p.is_some());
        for i in 0..3 { a[20 + i] = match w_p { Some(p) => p[i], None => 0.0 }; }
        for i in 0..4 { a[15 + i] = w_s[i]; a[23 + i] = f0[i]; a[27 + i] = f1[i]; a[31 + i] = rc[i]; }
    }
    let (slot, at, at_b, at_rest) = { let p = places(g, c); (slot_no(g_2), p.0, p.1, p.2) };
    anim_hands_ask(g, c, &mut |_g: &mut Game| ({ let ran__ = sim::layers::creep_arms(slot, at, at_b, at_rest); crate::anim_skeleton::b_01::stage_back(); ran__ }) as f64);
}

/// Its body: the pelvis and the feet, the heft, the bow and the look. Answers [the arms' share of his stride, his surge]. (character.ts CharacterAnimator.creepBody)
pub fn anim_creep_body(g: &mut crate::game::Game, c: CharId, e: f64, free: f64, cr: f64, gw: f64, tw_y: f64, tw_p: f64, dt: f64) -> Vec<f64> {
    let slot = layer_slot_p_a(g, c);
    anim_creep_body_ask(g, c, slot, e, free, cr, gw, tw_y, tw_p, dt)
}

/// (character.ts CharacterAnimator.creepBodyAsk)
pub fn anim_creep_body_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, e: f64, free: f64, cr: f64, gw: f64, tw_y: f64, tw_p: f64, dt: f64) -> Vec<f64> {
    let p0 = g.ch[c].pos;
    {
        let an = &g.ch[c].anim;
        let (ls, hs, at, gl) = (an.low_shape, an.heft_shape, an.creep_at, an.gaze_local);
        let a = args();
        a[0] = e; a[1] = free; a[2] = cr; a[3] = gw; a[4] = dt; a[5] = tw_y; a[6] = tw_p; a[7] = an.low_s; a[8] = ls[0]; a[9] = ls[1]; a[10] = ls[2];
        for k in 0..6 { a[11 + k] = hs[k]; }
        let t = at.unwrap_or([0.0; 3]);
        a[17] = an.speed; a[18] = an.wt.w; a[19] = b01(at.is_some()); a[20] = t[0]; a[21] = t[1]; a[22] = t[2];
        a[23] = an.body_yaw_in; a[24] = p0[0]; a[25] = p0[1]; a[26] = p0[2]; a[27] = gl[0]; a[28] = gl[1]; a[29] = gl[2];
    }
    creep_cols_get(g, c);
    let code = ({ let ran__ = sim::layers::creep_body(slot_no(g_2), seam_pose_at(&g.ch[c].anim.p_a)); crate::anim_skeleton::b_01::stage_back(); ran__ }) as f64;
    layer_ran_of(g, c, code);
    creep_cols_put(g, c);
    let b = args();
    vec![b[0], b[1]]
}

/// creepCols.get onto the bench (character.ts:3662)
fn creep_cols_get(g: &Game, c: CharId) {
    let h = g.ch[c].anim.hf;
    let v = &mut args()[COLS..];
    v[0] = h.t; v[1] = h.s; v[2] = h.sv; v[3] = h.a; v[4] = h.av; v[5] = h.w; v[6] = h.wv; v[7] = h.v; v[8] = h.vv;
}
/// creepCols.put off the bench (character.ts:3663)
fn creep_cols_put(g: &mut Game, c: CharId) {
    let v = &args()[COLS..];
    let h = &mut g.ch[c].anim.hf;
    h.t = v[0]; h.s = v[1]; h.sv = v[2]; h.a = v[3]; h.av = v[4]; h.w = v[5]; h.wv = v[6]; h.v = v[7]; h.vv = v[8];
}

// ---- the stealth poses

/// The arms of the hands-empty crouch. `busy`: something the animator knows has his arms or body. (character.ts CharacterAnimator.stealthLayer)
pub fn anim_stealth_layer(g: &mut crate::game::Game, c: CharId, dt: f64, busy: bool, crouch_pose: f64, w_move: f64, tw_y: f64, tw_p: f64) {
    if g.ch[c].anim.sp.is_none() {
        g.ch[c].anim.sp = Some(AnimSp { w_a: 0.0, t: 0.0, cr: crouch_pose, fl: 0.0, lead: 0.0, fz: 0.0, sw: 0.0, w_c: 0.0, w_h: 0.0, pic: None, pic_on: false, nod_up: None });
    }
    let slot = layer_slot_p_a(g, c);
    anim_stealth_layer_ask(g, c, slot, dt, busy, crouch_pose, w_move, tw_y, tw_p);
}

/// stealthCols.get onto the bench (character.ts:3677)
fn stealth_cols_get(g: &Game, c: CharId) {
    let s = g.ch[c].anim.sp.as_ref().unwrap();
    let v = &mut args()[COLS..];
    v[0] = s.w_a; v[1] = s.t; v[2] = s.cr; v[3] = s.fl; v[4] = s.lead; v[5] = s.fz; v[6] = s.sw;
}
/// stealthCols.put off the bench (character.ts:3678)
fn stealth_cols_put(g: &mut Game, c: CharId) {
    let v = &args()[COLS..];
    let s = g.ch[c].anim.sp.as_mut().unwrap();
    s.w_a = v[0]; s.t = v[1]; s.cr = v[2]; s.fl = v[3]; s.lead = v[4]; s.fz = v[5]; s.sw = v[6];
}

/// (character.ts CharacterAnimator.stealthLayerAsk)
pub fn anim_stealth_layer_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, dt: f64, busy: bool, crouch_pose: f64, w_move: f64, tw_y: f64, tw_p: f64) {
    {
        let an = &g.ch[c].anim;
        let free = an.stealth.as_ref().map_or(false, |f| f.free);
        let (f0, f1) = (an.hand_fix[0], an.hand_fix[1]);
        let a = args();
        a[0] = dt; a[1] = b01(free && !busy); a[2] = crouch_pose; a[3] = w_move; a[4] = tw_y; a[5] = tw_p; a[6] = an.creep_shown; a[7] = an.ready_w; a[8] = an.wide_s; a[9] = an.hands_down;
        for i in 0..4 { a[10 + i] = f0[i]; a[14 + i] = f1[i]; }
    }
    stealth_cols_get(g, c);
    let (slot, at, at_b, at_rest) = { let p = places(g, c); (slot_no(g_2), p.0, p.1, p.2) };
    anim_hands_ask(g, c, &mut |_g: &mut Game| ({ let ran__ = sim::layers::stealth_layer(slot, at, at_b, at_rest); crate::anim_skeleton::b_01::stage_back(); ran__ }) as f64);
    stealth_cols_put(g, c);
}

/// The look under a door with the wand, on a copy of the finished pose that is then made the pose. (character.ts CharacterAnimator.stealthPicture)
pub fn anim_stealth_picture(g: &mut crate::game::Game, c: CharId, dt: f64) {
    let cable_in = g.k.stealthpose.cable_in;
    let (cable_out, cable_head_out) = { let k = crate::character::stealth().cable; (k.out, k.head_out) };
    let an = &mut g.ch[c].anim;
    let want = an.stealth.as_ref().map_or(false, |f| f.cable) && an.lockpick;
    let nv_nod = an.nv_nod;
    let s = an.sp.as_mut().unwrap();
    s.w_c = v3::clamp(s.w_c + (if want { dt / m::max(0.05, cable_in) } else { -dt / cable_out }), 0.0, 1.0);
    s.w_h = v3::clamp(s.w_h + (if want { dt / m::max(0.05, cable_in) } else { -dt / cable_head_out }), 0.0, 1.0);
    if nv_nod == 0.0 { s.nod_up = None; }
    if s.w_c <= 0.003 { s.pic_on = false; return; }
    let (w_c, w_h) = (s.w_c, s.w_h);
    let kept = s.pic.take();
    // lent for the layer: the columns of the look read sp.nodUp, never sp.pic
    let mut pic = match kept {
        Some(p) => p,
        None => crate::anim_skeleton::pose_new(g.ch[c].anim.p_a.num_nodes as f64)
    };
    crate::anim_skeleton::pose_copy(&mut pic, &g.ch[c].anim.p_a);
    anim_cable_pose(g, c, &pic, w_c * w_c * (3.0 - 2.0 * w_c), w_h * w_h * (3.0 - 2.0 * w_h));
    let s = g.ch[c].anim.sp.as_mut().unwrap();
    s.pic = Some(pic);
    s.pic_on = true;
}

/// The look under a door itself, on `pic`, by its eased weight e and the head's eh. (character.ts CharacterAnimator.cablePose)
pub fn anim_cable_pose(g: &mut crate::game::Game, c: CharId, pic: &crate::anim_skeleton::Pose, e: f64, eh: f64) {
    let slot = layer_slot_of(g, c, pic);
    anim_cable_pose_ask(g, c, slot, pic, e, eh);
}

/// cableCols.get onto the bench (character.ts:3708)
fn cable_cols_get(g: &Game, c: CharId) {
    let an = &g.ch[c].anim;
    let u = an.sp.as_ref().unwrap().nod_up;
    let v = &mut args()[COLS..];
    v[0] = match u { None => -1.0, Some(true) => 1.0, Some(false) => 0.0 };
    v[1] = an.nv_a;
}
/// cableCols.put off the bench (character.ts:3709)
fn cable_cols_put(g: &mut Game, c: CharId) {
    let v = &args()[COLS..];
    let an = &mut g.ch[c].anim;
    an.sp.as_mut().unwrap().nod_up = if v[0] < 0.0 { None } else { Some(v[0] > 0.0) };
    an.nv_a = v[1];
}

/// (character.ts CharacterAnimator.cablePoseAsk)
pub fn anim_cable_pose_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, pic: &crate::anim_skeleton::Pose, e: f64, eh: f64) {
    {
        let an = &g.ch[c].anim;
        let gl = an.gaze_local;
        let a = args();
        a[0] = e; a[1] = eh; a[2] = v3::clamp(an.lp_w.x, 0.0, 1.0); a[3] = an.lockpick_at[2]; a[4] = an.nv_nod; a[5] = gl[0]; a[6] = gl[1]; a[7] = gl[2];
    }
    let o = g.m.anim_skeleton.pose_bench.args;
    anim_head_hand_to_bench(g, c, o + 8.0);
    {
        let f1 = g.ch[c].anim.hand_fix[1];
        let a = args();
        for i in 0..4 { a[20 + i] = f1[i]; }
    }
    cable_cols_get(g, c);
    let at = seam_pose_at(pic);
    let (at_b, at_rest) = (seam_pose_at(&g.ch[c].anim.p_b), seam_pose_at(&g.ch[c].anim.rest));
    let slot = slot_no(g_2);
    anim_hands_ask(g, c, &mut |_g: &mut Game| ({ let ran__ = sim::layers::cable_pose(slot, at, at_b, at_rest); crate::anim_skeleton::b_01::stage_back(); ran__ }) as f64);
    cable_cols_put(g, c);
}

// ---- the twist's inverse

/// The chest twist run backwards for a point; fk must be current on the pose asked about. (character.ts CharacterAnimator.untwist)
pub fn anim_untwist(g: &mut crate::game::Game, c: CharId, tw_y: f64, tw_p: f64) -> crate::character::Untwist {
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
pub fn anim_kit_layer(g: &mut crate::game::Game, c: CharId, dt: f64, tw_y: f64, tw_p: f64) {
    let slot = layer_slot_p_a(g, c);
    anim_kit_layer_ask(g, c, slot, tw_y, tw_p);
    let an = &mut g.ch[c].anim;
    let k = an.kit.as_mut().unwrap();
    k.t += dt;
    if k.t >= k.d { an.kit = None; }
}

/// kitCols.put off the bench (character.ts:3742)
fn kit_cols_put(g: &mut Game, c: CharId) {
    let v = &args()[COLS..];
    let hold = if v[0] < 0.0 { None } else { Some(kit_item_of(v[0])) };
    seam_kit_hold_put(g, c, hold);
    let an = &mut g.ch[c].anim;
    an.kit_stow = v[1]; an.kit_roll = v[2];
    an.kit_lie = if js::truthy(v[3]) { Some([v[4], v[5], v[6], v[7]]) } else { None };
}

/// (character.ts CharacterAnimator.kitLayerAsk)
pub fn anim_kit_layer_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, tw_y: f64, tw_p: f64) {
    let rc = anim_sheath_rest_inv(g, c);
    let kit_rest = g.ch[c].anim.kit_rest;
    let s = crate::rig_props::sling_front_hand(g, None, Some(kit_rest));
    let rig = crate::rig_props::rig();
    let (h, p) = (rig.holster, rig.pistol.adj);
    {
        let an = &g.ch[c].anim;
        let k = an.kit.unwrap();
        let cap = k.cap;
        let f = an.hand_fix[1];
        let a = args();
        a[0] = k.t; a[1] = k.t_a; a[2] = k.t_r; a[3] = k.d; a[4] = k.s0; a[5] = k.roll0; a[6] = b01(k.re); a[7] = b01(k.front); a[8] = kit_no_of(k.from); a[9] = kit_no_of(k.to);
        a[10] = tw_y; a[11] = tw_p; a[12] = an.crouch; a[21] = b01(cap.mr.is_some()); a[28] = b01(cap.mf.is_some() && cap.mb.is_some()); a[39] = b01(cap.ml.is_some());
        for i in 0..4 { a[13 + i] = f[i]; a[17 + i] = rc[i]; a[35 + i] = cap.rq[i]; a[46 + i] = cap.lq[i]; }
        for i in 0..3 {
            a[22 + i] = match cap.mr { Some(v) => v[i], None => 0.0 };
            a[25 + i] = cap.r[i];
            a[29 + i] = match cap.mf { Some(v) => v[i], None => 0.0 };
            a[32 + i] = match cap.mb { Some(v) => v[i], None => 0.0 };
            a[40 + i] = match cap.ml { Some(v) => v[i], None => 0.0 };
            a[43 + i] = cap.l[i];
        }
        let q = &mut a[KEYS..];
        for i in 0..3 {
            q[i] = s.at[i]; q[3 + i] = s.fing[i]; q[6 + i] = s.back[i];
            q[13 + i] = match h.adj { Some(v) => v.off[i], None => 0.0 };
            q[16 + i] = match h.adj { Some(v) => v.rot[i], None => 0.0 };
            q[20 + i] = match p { Some(v) => v.off[i], None => 0.0 };
            q[23 + i] = match p { Some(v) => v.rot[i], None => 0.0 };
        }
        q[9] = h.down; q[10] = h.out; q[11] = h.fwd; q[12] = b01(h.adj.is_some()); q[19] = b01(p.is_some());
    }
    let (slot, at, at_b, at_rest) = { let p = places(g, c); (slot_no(g_2), p.0, p.1, p.2) };
    anim_hands_ask(g, c, &mut |_g: &mut Game| ({ let ran__ = sim::layers::kit_layer(slot, at, at_b, at_rest); crate::anim_skeleton::b_01::stage_back(); ran__ }) as f64);
    kit_cols_put(g, c);
}

// ---- the hold's catch and the ready pose

/// The hold's arm's last word: the left hand put on the hold's target, after the layers that move his trunk. (character.ts CharacterAnimator.catchArm)
pub fn anim_catch_arm(g: &mut crate::game::Game, c: CharId, x: crate::character::AnimHdFix) {
    let slot = layer_slot_p_a(g, c);
    anim_catch_arm_ask(g, c, slot, x);
}

/// (character.ts CharacterAnimator.catchArmAsk)
pub fn anim_catch_arm_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, x: crate::character::AnimHdFix) {
    {
        let a = args();
        a[0] = x.w; a[7] = x.pole_w; a[12] = x.rw;
        for i in 0..3 { a[1 + i] = x.tgt[i]; a[4 + i] = x.pole[i]; }
        for i in 0..4 { a[8 + i] = x.rot[i]; }
    }
    let code = ({ let ran__ = sim::layers::catch_arm(slot_no(g_2), seam_pose_at(&g.ch[c].anim.p_a)); crate::anim_skeleton::b_01::stage_back(); ran__ }) as f64;
    layer_ran_of(g, c, code);
}

/// HOLD.catch, after the layers: the left hand onto the hold's target; with the reach up and no hold, where it has the hand is noted. (character.ts CharacterAnimator.catchFix)
pub fn anim_catch_fix(g: &mut crate::game::Game, c: CharId) {
    let x = g.ch[c].anim.hd_fix.take();
    if !g.ch[c].anim.no_knife { g.ch[c].anim.rd_hand_l = None; return; }
    if let Some(x) = x {
        anim_catch_arm(g, c, x);
    } else if g.ch[c].anim.hold_pose.is_none() {
        if g.ch[c].anim.ready_w > 0.003 && g.ch[c].anim.ready_k == ReadyKind::Grab {
            let rig = g.ch[c].anim.rig;
            let an = &mut g.ch[c].anim;
            crate::anim_skeleton::pose_fk_run(&mut an.fk, &an.p_a, &g.tab.rig[rig]);
            let an = &mut g.ch[c].anim;
            an.rd_hand_l = Some(v3::copy(an.fk.pos[an.nodes.hand_l]));
        } else {
            g.ch[c].anim.rd_hand_l = None;
        }
    }
}

/// The ready pose: the trunk, then the arms. Weight `w`; `live` = still asked for. (character.ts CharacterAnimator.readyLayer)
pub fn anim_ready_layer(g: &mut crate::game::Game, c: CharId, w: f64, kind: crate::character::ReadyKind, live: bool, dt: f64, tw_y: f64, tw_p: f64) {
    let slot = layer_slot_p_a(g, c);
    anim_ready_layer_ask(g, c, slot, w, kind, live, dt, tw_y, tw_p);
    g.ch[c].anim.ready_shown = w;
}

/// rdCols.get onto the bench (character.ts:3794)
fn rd_cols_get(g: &Game, c: CharId) {
    let an = &g.ch[c].anim;
    let v = &mut args()[COLS..];
    for i in 0..RD { v[i] = an.rd[i]; }
    v[RD] = an.cl_l; v[RD + 1] = an.reach_shown; v[RD + 2] = b01(an.rd_live);
}
/// rdCols.put off the bench (character.ts:3795)
fn rd_cols_put(g: &mut Game, c: CharId) {
    let v = &args()[COLS..];
    let an = &mut g.ch[c].anim;
    for i in 0..RD { an.rd[i] = v[i]; }
    an.cl_l = v[RD]; an.reach_shown = v[RD + 1]; an.rd_live = v[RD + 2] != 0.0;
}

/// (character.ts CharacterAnimator.readyLayerAsk)
pub fn anim_ready_layer_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, w: f64, kind: crate::character::ReadyKind, live: bool, dt: f64, tw_y: f64, tw_p: f64) {
    let k = crate::character::ready().get(String::from(kind.as_str())).unwrap().clone();
    let wp = match g.ch[c].anim.rd_at { Some(p) => Some(anim_to_model_p(g, c, p)), None => None };
    let rc = anim_sheath_rest_inv(g, c);
    {
        let an = &g.ch[c].anim;
        let (f0, f1) = (an.hand_fix[0], an.hand_fix[1]);
        let bare = an.no_knife && kind == ReadyKind::Grab && !an.grip[1];
        let table = crate::character::ready_bare();
        let (b_pole, b_fing, b_back, b_curl, b_thumb, b_spread) = if bare {
            let b = &table.l; (b.pole, b.fing, b.back, b.curl, b.thumb, b.spread)
        } else {
            let b = &k.l; (b.pole, b.fing, b.back, b.curl, b.thumb, b.spread)
        };
        let r2 = &table.r;
        let (r, at) = (k.l.reach, k.l.at);
        let a = args();
        a[0] = w; a[1] = b01(live); a[2] = dt; a[3] = tw_y; a[4] = tw_p; a[5] = b01(wp.is_some()); a[9] = b01(kind == ReadyKind::Grab); a[10] = an.ready_up; a[11] = b01(bare); a[12] = 1.0;
        a[13] = an.rd_a; a[14] = an.rc_t; a[15] = an.rd_out_r; a[16] = an.close_k; a[17] = an.cl_t; a[18] = b01(k.knife && !an.no_knife); a[19] = an.crouch;
        for i in 0..3 { a[6 + i] = match wp { Some(p) => p[i], None => 0.0 }; }
        for i in 0..4 { a[20 + i] = f0[i]; a[24 + i] = f1[i]; a[28 + i] = rc[i]; }
        let q = &mut a[KEYS..];
        q[0] = k.lean; q[1] = k.blade; q[2] = k.clav; q[3] = b01(r.is_some());
        if let Some(r) = r {
            q[4] = r.frac; q[5] = r.r[0]; q[6] = r.r[1]; q[7] = r.dy[0]; q[8] = r.dy[1]; q[9] = r.bias; q[10] = r.h0; q[11] = r.y0;
        } else {
            for i in 4..12 { q[i] = 0.0; }
        }
        for i in 0..3 {
            q[12 + i] = match at { Some(p) => p[i], None => 0.0 };
            q[15 + i] = b_pole[i]; q[18 + i] = b_fing[i]; q[21 + i] = b_back[i];
            q[27 + i] = r2.at[i]; q[30 + i] = r2.pole[i]; q[33 + i] = r2.fing[i]; q[36 + i] = r2.back[i];
        }
        q[24] = b_curl; q[25] = b_thumb; q[26] = b_spread; q[39] = r2.curl; q[40] = r2.thumb;
    }
    rd_cols_get(g, c);
    let (slot, at, at_b, at_rest) = { let p = places(g, c); (slot_no(g_2), p.0, p.1, p.2) };
    anim_hands_ask(g, c, &mut |_g: &mut Game| ({ let ran__ = sim::layers::ready_layer(slot, at, at_b, at_rest); crate::anim_skeleton::b_01::stage_back(); ran__ }) as f64);
    rd_cols_put(g, c);
}

/// The rest pose's chest turn, inverted: the sheath's frame; found once, off a rest pose of its own. (character.ts CharacterAnimator.sheathRestInv)
pub fn anim_sheath_rest_inv(g: &mut crate::game::Game, c: CharId) -> Quat {
    if let Some(q) = g.ch[c].anim.rc_inv { return q; }
    let rig = g.ch[c].anim.rig;
    let mut f = crate::anim_skeleton::pose_fk_new(g, rig);
    let mut fresh = crate::anim_skeleton::pose_new(g.tab.rig[rig].nodes.len() as f64);
    let rest = crate::anim_skeleton::rest_pose(&g.tab.rig[rig], &mut fresh);
    crate::anim_skeleton::pose_fk_run(&mut f, &rest, &g.tab.rig[rig]);
    let an = &mut g.ch[c].anim;
    let q = v3::quat::conj(f.rot[an.nodes.spine3]);
    an.rc_inv = Some(q);
    q
}

// ---- the takedown

/// A takedown's clocks, ahead of the pose: the move's and the reaction's seconds, the blow's points riding the man, and the crouch they leave. (character.ts CharacterAnimator.takePre)
pub fn anim_take_pre(g: &mut crate::game::Game, c: CharId, dt: f64, cp: f64) -> f64 {
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
            let table = crate::character::blow();
            let rise = if b.rise { *table.rise.get(String::from(b.style.as_str())).unwrap() * bump(b.t, 0.0, 0.6 * b.hit, b.hit + 0.1, b.end) } else { 0.0 };
            g.ch[c].anim.bw_rise = rise;
            cp *= 1.0 - rise;
            if b.tail > 0.0 { cp = m::max(cp, *table.sink.get(String::from(b.style.as_str())).unwrap() * anim_blow_tail(g, c, &b)); }
            g.ch[c].anim.bw = Some(b);
        }
    }
    if let Some(mut tk) = t {
        tk.t += dt;
        let u = tk.t - tk.delay;
        if tk.off >= 0.0 { tk.off += dt; }
        g.ch[c].anim.tk = Some(tk);
        let table = crate::character::taken();
        let secs = table.secs;
        if u >= secs || tk.off >= 0.2 {
            g.ch[c].anim.tk = None;
        } else if u > 0.0 {
            cp = m::max(cp, *table.sag.get(String::from(tk.style.as_str())).unwrap() * v3::smoothstep(0.0, 0.3, u) * (1.0 - v3::smoothstep(0.6, secs, u)) * (if tk.off < 0.0 { 1.0 } else { 1.0 - tk.off / 0.2 }));
        }
    }
    cp
}

/// How far into the move's tail he is, 0 to 1; let go of as he walks off. (character.ts CharacterAnimator.blowTail)
pub fn anim_blow_tail(g: &mut crate::game::Game, c: CharId, b: &crate::character::BlowRun) -> f64 {
    let x = b.end + b.tail;
    let w = crate::character::blow().walk;
    bump(b.t, b.hit + 0.08, b.end + 0.1, x - 0.45, x) * (1.0 - v3::smoothstep(w[0], w[1], m::abs(g.ch[c].anim.speed)))
}

/// The legs under the blow: he stands into the man. Nothing where he may not rise. (character.ts CharacterAnimator.blowLegs)
pub fn anim_blow_legs(g: &mut crate::game::Game, c: CharId, b: &crate::character::BlowRun) {
    if !b.rise { return; }
    let slot = layer_slot_p_a(g, c);
    anim_blow_legs_ask(g, c, slot, b);
}

/// (character.ts CharacterAnimator.blowLegsAsk)
pub fn anim_blow_legs_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, b: &crate::character::BlowRun) {
    {
        let a = args();
        a[0] = b.t; a[1] = b.hit; a[2] = b.end; a[3] = b.tail;
    }
    let code = ({ let ran__ = sim::layers::blow_legs(slot_no(g_2), seam_pose_at(&g.ch[c].anim.p_a)); crate::anim_skeleton::b_01::stage_back(); ran__ }) as f64;
    layer_ran_of(g, c, code);
}

/// The trunk's share of the move, on pA. (character.ts CharacterAnimator.blowTrunk)
pub fn anim_blow_trunk(g: &mut crate::game::Game, c: CharId, b: &crate::character::BlowRun) {
    let tl = if b.tail > 0.0 { anim_blow_tail(g, c, b) } else { 0.0 };
    let slot = layer_slot_p_a(g, c);
    anim_blow_trunk_ask(g, c, slot, b, tl);
}

/// (character.ts CharacterAnimator.blowTrunkAsk)
pub fn anim_blow_trunk_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, b: &crate::character::BlowRun, tl: f64) {
    {
        let a = args();
        a[0] = b.t; a[1] = b.hit; a[2] = b.end; a[3] = blow_no_of(b.style); a[4] = tl;
    }
    let code = ({ let ran__ = sim::layers::blow_trunk(slot_no(g_2), seam_pose_at(&g.ch[c].anim.p_a)); crate::anim_skeleton::b_01::stage_back(); ran__ }) as f64;
    layer_ran_of(g, c, code);
}

/// The arms of the move, by IK on a copy of pA. Returns the layer's weight. (character.ts CharacterAnimator.blowArms)
pub fn anim_blow_arms(g: &mut crate::game::Game, c: CharId, b: &crate::character::BlowRun, tw_y: f64, tw_p: f64) -> f64 {
    let slot = layer_slot_p_a(g, c);
    anim_blow_arms_ask(g, c, slot, b, tw_y, tw_p)
}

/// (character.ts CharacterAnimator.blowArmsAsk)
pub fn anim_blow_arms_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, b: &crate::character::BlowRun, tw_y: f64, tw_p: f64) -> f64 {
    let t = anim_to_model_p(g, c, b.at);
    let t2 = match b.at2 { Some(p) => Some(anim_to_model_p(g, c, p)), None => None };
    {
        let an = &g.ch[c].anim;
        let (gl, f0, f1) = (an.gaze_local, an.hand_fix[0], an.hand_fix[1]);
        let a = args();
        a[0] = b.t; a[1] = b.hit; a[2] = b.end; a[3] = b.tail; a[4] = blow_no_of(b.style); a[5] = 1.0 - v3::clamp(an.upper_w.aim, 0.0, 1.0); a[6] = tw_y; a[7] = tw_p;
        a[11] = b01(t2.is_some()); a[15] = m::abs(an.speed);
        for i in 0..3 { a[8 + i] = t[i]; a[12 + i] = match t2 { Some(p) => p[i], None => 0.0 }; a[16 + i] = gl[i]; }
        for i in 0..4 { a[19 + i] = f0[i]; a[23 + i] = f1[i]; }
    }
    let (slot, at, at_b, at_rest) = { let p = places(g, c); (slot_no(g_2), p.0, p.1, p.2) };
    anim_hands_ask(g, c, &mut |_g: &mut Game| ({ let ran__ = sim::layers::blow_arms(slot, at, at_b, at_rest); crate::anim_skeleton::b_01::stage_back(); ran__ }) as f64);
    args()[COLS]
}

/// The man a takedown is done to, still on his feet. (character.ts CharacterAnimator.takenLayer)
pub fn anim_taken_layer(g: &mut crate::game::Game, c: CharId, t: crate::character::AnimTakenLayerT) {
    let u = t.t - t.delay;
    if u <= 0.0 { return; }
    let slot = layer_slot_p_a(g, c);
    anim_taken_layer_ask(g, c, slot, AnimTakenLayerAskT { style: t.style, off: t.off }, u);
}

/// (character.ts CharacterAnimator.takenLayerAsk)
pub fn anim_taken_layer_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, t: crate::character::AnimTakenLayerAskT, u: f64) {
    {
        let a = args();
        a[0] = u; a[1] = t.off; a[2] = match t.style { TakenStyle::Arch => 0.0, TakenStyle::Hunch => 1.0, TakenStyle::Snap => 2.0 };
    }
    let code = ({ let ran__ = sim::layers::taken_layer(slot_no(g_2), seam_pose_at(&g.ch[c].anim.p_a)); crate::anim_skeleton::b_01::stage_back(); ran__ }) as f64;
    layer_ran_of(g, c, code);
}

/// The knife's blow: the right arm over the punch clip's pose, its clock and its pose. (character.ts CharacterAnimator.knifeBlow)
pub fn anim_knife_blow(g: &mut crate::game::Game, c: CharId, kind: crate::character::StrikeKind, dt: f64, total: f64, tw_y: f64, tw_p: f64) {
    let kb = crate::character::knife_blow();
    let k = if g.ch[c].anim.kb_slash {
        let t = crate::character::knife_slash(); AnimKnifePoseK { end: t.end, keys: t.keys }
    } else if g.ch[c].anim.kb_back {
        let t = crate::character::knife_back(); AnimKnifePoseK { end: t.end, keys: t.keys }
    } else {
        let t = match kind { StrikeKind::Cross => kb.cross, StrikeKind::Jab => kb.jab }; AnimKnifePoseK { end: t.end, keys: t.keys }
    };
    let t0 = kb.snap * (1.0 - g.ch[c].anim.kb_r0);
    let fit = m::min(1.0, m::max(0.2, total - kb.fade - t0 - 0.035) / k.end);
    let t = (g.ch[c].anim.kb_t - t0) / fit;
    g.ch[c].anim.kb_t += dt;
    anim_knife_pose(g, c, &k, t, t0, total, tw_y, tw_p);
    g.ch[c].anim.knife_shown = t > 0.0 && t < k.end;
}

/// The blow's pose, its clock being knifeBlow's: `t` the key time, `t0` the seconds to the hilt. (character.ts CharacterAnimator.knifePose)
pub fn anim_knife_pose(g: &mut crate::game::Game, c: CharId, k: &crate::character::AnimKnifePoseK, t: f64, t0: f64, total: f64, tw_y: f64, tw_p: f64) {
    let slot = layer_slot_p_a(g, c);
    anim_knife_pose_ask(g, c, slot, &AnimKnifePoseAskK { end: k.end, keys: k.keys.clone() }, t, t0, total, tw_y, tw_p);
}

/// (character.ts CharacterAnimator.knifePoseAsk)
pub fn anim_knife_pose_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, k: &crate::character::AnimKnifePoseAskK, t: f64, t0: f64, total: f64, tw_y: f64, tw_p: f64) {
    let at_p = match g.ch[c].anim.rd_at { Some(p) => Some(anim_to_model_p(g, c, p)), None => None };
    let rc = anim_sheath_rest_inv(g, c);
    {
        let an = &g.ch[c].anim;
        let f = an.hand_fix[1];
        let a = args();
        a[0] = t; a[1] = t0; a[2] = total; a[3] = an.kb_t; a[4] = an.kb_r0; a[5] = if an.rc_t > 0.0 { an.reach_shown } else { 0.0 }; a[6] = tw_y; a[7] = tw_p;
        a[8] = b01(at_p.is_some()); a[12] = an.crouch; a[21] = k.end; a[22] = k.keys.len() as f64;
        for i in 0..3 { a[9 + i] = match at_p { Some(p) => p[i], None => 0.0 }; }
        for i in 0..4 { a[13 + i] = f[i]; a[17 + i] = rc[i]; }
        for (i, key) in k.keys.iter().enumerate() {
            let q = KEYS + i * 13;
            a[q] = key.t;
            for j in 0..3 { a[q + 1 + j] = key.at[j]; a[q + 4 + j] = key.blade[j]; a[q + 7 + j] = key.fing[j]; a[q + 10 + j] = key.pole[j]; }
        }
    }
    let (slot, at, at_b, at_rest) = { let p = places(g, c); (slot_no(g_2), p.0, p.1, p.2) };
    anim_hands_ask(g, c, &mut |_g: &mut Game| ({ let ran__ = sim::layers::knife_pose(slot, at, at_b, at_rest); crate::anim_skeleton::b_01::stage_back(); ran__ }) as f64);
}

// ---- lock work, the haul

/// Lock work: the back un-rounded, the head on the lock, both hands at the keyway. (character.ts CharacterAnimator.lockpickLayer)
pub fn anim_lockpick_layer(g: &mut crate::game::Game, c: CharId, w: f64) {
    let slot = layer_slot_p_a(g, c);
    anim_lockpick_layer_ask(g, c, slot, w);
}

/// (character.ts CharacterAnimator.lockpickLayerAsk)
pub fn anim_lockpick_layer_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, w: f64) {
    {
        let an = &g.ch[c].anim;
        let (at, gl, f0, f1) = (an.lockpick_at, an.gaze_local, an.hand_fix[0], an.hand_fix[1]);
        let a = args();
        a[0] = w; a[1] = an.lp_t;
        for i in 0..3 { a[2 + i] = at[i]; a[5 + i] = gl[i]; }
        for i in 0..4 { a[8 + i] = f0[i]; a[12 + i] = f1[i]; }
    }
    let (slot, at, at_b, at_rest) = { let p = places(g, c); (slot_no(g_2), p.0, p.1, p.2) };
    anim_hands_ask(g, c, &mut |_g: &mut Game| ({ let ran__ = sim::layers::lockpick_layer(slot, at, at_b, at_rest); crate::anim_skeleton::b_01::stage_back(); ran__ }) as f64);
}

/// Both hands on a body: the trunk bows toward the grip, both arms by IK on a copy. (character.ts CharacterAnimator.haulLayer)
pub fn anim_haul_layer(g: &mut crate::game::Game, c: CharId, w: f64, tw_y: f64, tw_p: f64) {
    let h = match g.ch[c].anim.hl_last { Some(h) => h, None => return };
    let slot = layer_slot_p_a(g, c);
    anim_haul_layer_ask(g, c, slot, h, w, tw_y, tw_p);
}

/// (character.ts CharacterAnimator.haulLayerAsk)
pub fn anim_haul_layer_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, h: crate::character::HaulPose, w: f64, tw_y: f64, tw_p: f64) {
    let l = anim_to_model_p(g, c, h.l);
    let r = anim_to_model_p(g, c, h.r);
    {
        let an = &g.ch[c].anim;
        let (gl, f0, f1) = (an.gaze_local, an.hand_fix[0], an.hand_fix[1]);
        let a = args();
        a[0] = w; a[1] = tw_y; a[2] = tw_p; a[3] = h.bow; a[4] = v3::clamp(h.rw.unwrap_or(1.0), 0.0, 1.0); a[5] = h.look.unwrap_or(0.7); a[6] = h.tilt.unwrap_or(0.0); a[7] = h.grip;
        for i in 0..3 { a[8 + i] = l[i]; a[11 + i] = r[i]; a[14 + i] = gl[i]; }
        for i in 0..4 { a[17 + i] = f0[i]; a[21 + i] = f1[i]; }
    }
    let (slot, at, at_b, at_rest) = { let p = places(g, c); (slot_no(g_2), p.0, p.1, p.2) };
    anim_hands_ask(g, c, &mut |_g: &mut Game| ({ let ran__ = sim::layers::haul_layer(slot, at, at_b, at_rest); crate::anim_skeleton::b_01::stage_back(); ran__ }) as f64);
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
pub fn anim_to_model_p(g: &mut crate::game::Game, c: CharId, wp: V3) -> V3 {
    let p0 = g.ch[c].pos;
    to_model_p_of(g.ch[c].anim.body_yaw_in, p0, wp)
}

/// The same for a direction. (character.ts CharacterAnimator.toModelD)
pub fn anim_to_model_d(g: &mut crate::game::Game, c: CharId, wd: V3) -> V3 {
    to_model_d_of(g.ch[c].anim.body_yaw_in, wd)
}

// ---- the door hand

/// The door hand's eased weights per arm and which arm is the wanted one. (character.ts CharacterAnimator.doorHands)
pub fn anim_door_hands(g: &mut crate::game::Game, c: CharId) -> crate::character::AnimDoorHands {
    let dp = &g.ch[c].anim.dp;
    AnimDoorHands { l: v3::clamp(dp.w_l.x, 0.0, 1.0), r: v3::clamp(dp.w_r.x, 0.0, 1.0), right: dp.right }
}

/// A hand on the door leaf he is pushing open; the free arm only, the weight this layer's own springs. (character.ts CharacterAnimator.doorLayer)
pub fn anim_door_layer(g: &mut crate::game::Game, c: CharId, dt: f64, intent: f64, armed: bool, tw_y: f64, tw_p: f64) {
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
    let slot = layer_slot_p_a(g, c);
    anim_door_layer_ask(g, c, slot, l, dt, intent, armed, tw_y, tw_p);
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
        let (b, t, r) = (crate::character::blow(), crate::character::taken(), crate::character::ready());
        for s in [BlowStyle::Chop, BlowStyle::Palm, BlowStyle::Mouth, BlowStyle::Slash, BlowStyle::Sweep] {
            let w = String::from(s.as_str());
            assert!(b.rise.get(w.clone()).is_some() && b.sink.get(w.clone()).is_some());
            let _ = blow_no_of(s);
        }
        for s in [TakenStyle::Arch, TakenStyle::Hunch, TakenStyle::Snap] { assert!(t.sag.get(String::from(s.as_str())).is_some()); }
        for k in [ReadyKind::Grab, ReadyKind::Seat, ReadyKind::Strike, ReadyKind::GrabGun] { assert!(r.get(String::from(k.as_str())).is_some()); }
    }

    #[test]
    fn the_kit_numbers_round_trip() {
        for k in [KitItem::Pistol, KitItem::Rifle, KitItem::Can, KitItem::Knife] { assert_eq!(kit_item_of(kit_no_of(Some(k))), Some(k)); }
        assert_eq!(kit_item_of(0.0), None);
        assert_eq!(kit_no_of(None), 0.0);
    }
}
