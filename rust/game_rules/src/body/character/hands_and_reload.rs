//! src/game/character.ts lines 2620 to 3085
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

// where a layer's own numbers, its columns and a path of keys go on the bench (args is index 0 of `POSE_BENCH.args`)
const COLS: usize = crate::anim_skeleton::LAYER_COLS_AT as usize;
const HAND_COLS: usize = crate::anim_skeleton::HAND_COLS_AT as usize;
const KEYS: usize = crate::anim_skeleton::LAYER_KEYS_AT as usize;
const KEY: usize = crate::anim_skeleton::LAYER_KEY as usize;

/// the bench's doubles: a layer's numbers go here, in the order its Rust comment gives (skeleton.ts `f64()` at `POSE_BENCH.args`)
#[inline(always)]
fn args() -> &'static mut [f64] {
    &mut ::sim::skeleton::bench().args
}
/// the same doubles, to read
#[inline(always)]
fn args_read() -> &'static [f64] {
    &::sim::skeleton::bench_read().args
}

// ---- what stands between a body and the skeleton's calls

/// `pose.rot(i)`, a node by the list's usize
fn rot(g: &mut Game, pose: crate::anim_skeleton::Pose, i: usize) -> Quat { crate::anim_skeleton::pose_rot(g, pose, i as f64) }
/// `pose.setRot(i, q)`, a node by the list's usize
fn set_rot(g: &mut Game, pose: crate::anim_skeleton::Pose, i: usize, q: Quat) { crate::anim_skeleton::pose_set_rot(g, pose, i as f64, q) }

/// the three poses a layer that lays a clip or shapes an arm is handed: pA, pB, rest
fn ats_abr(g: &Game, c: CharId) -> (usize, usize, usize) {
    let an = &g.ch[c].anim;
    (an.p_a.at as usize, an.p_b.at as usize, an.rest.at as usize)
}
/// the three poses a cycle is laid with: pA, pB, pC
fn ats_abc(g: &Game, c: CharId) -> (usize, usize, usize) {
    let an = &g.ch[c].anim;
    (an.p_a.at as usize, an.p_b.at as usize, an.p_c.at as usize)
}

// ---- the layers' columns: get / put, in the order the module's layer is handed them

fn act_cols_get(an: &Anim, c: &mut [f64], o: usize) { c[o] = an.slump_w; }
fn act_cols_put(an: &mut Anim, c: &[f64], o: usize) { an.slump_w = c[o]; }
fn lean_cols_get(an: &Anim, c: &mut [f64], o: usize) { c[o] = an.lean_sm; }
fn lean_cols_put(an: &mut Anim, c: &[f64], o: usize) { an.lean_sm = c[o]; }
fn gaze_cols_get(an: &Anim, c: &mut [f64], o: usize) {
    let gz = &an.gz;
    c[o] = gz.w; c[o + 1] = gz.pitch; c[o + 2] = gz.yaw_ref; c[o + 3] = if gz.yaw_init { 1.0 } else { 0.0 };
}
fn gaze_cols_put(an: &mut Anim, c: &[f64], o: usize) {
    let gz = &mut an.gz;
    gz.w = c[o]; gz.pitch = c[o + 1]; gz.yaw_ref = c[o + 2]; gz.yaw_init = c[o + 3] != 0.0;
}
fn mv_pre_cols_get(an: &Anim, c: &mut [f64], o: usize) {
    let (m, dp, gt) = (&an.mvs, &an.mv2.dip, &an.mv_out);
    c[o] = m.back; c[o + 1] = m.climb; c[o + 2] = m.yaw; c[o + 3] = m.rock; c[o + 4] = m.sho; c[o + 5] = m.head; c[o + 6] = m.glide.x; c[o + 7] = m.glide.v; c[o + 8] = m.hip_y; c[o + 9] = dp.x; c[o + 10] = dp.v;
    c[o + 11] = gt.g; c[o + 12] = gt.u; c[o + 13] = gt.st; c[o + 14] = if gt.live { 1.0 } else { 0.0 }; c[o + 15] = an.mv2_shown.dip; c[o + 16] = an.sights_lift;
}
fn mv_pre_cols_put(an: &mut Anim, c: &[f64], o: usize) {
    {
        let m = &mut an.mvs;
        m.back = c[o]; m.climb = c[o + 1]; m.yaw = c[o + 2]; m.rock = c[o + 3]; m.sho = c[o + 4]; m.head = c[o + 5]; m.glide.x = c[o + 6]; m.glide.v = c[o + 7]; m.hip_y = c[o + 8];
    }
    an.mv2.dip.x = c[o + 9]; an.mv2.dip.v = c[o + 10];
    let gt = &mut an.mv_out;
    gt.g = c[o + 11]; gt.u = c[o + 12]; gt.st = c[o + 13]; gt.live = c[o + 14] != 0.0;
    an.mv2_shown.dip = c[o + 15]; an.sights_lift = c[o + 16];
}
fn mv_post_cols_get(an: &Anim, c: &mut [f64], o: usize) {
    let (m, p) = (&an.mvs, &an.mv_po);
    c[o] = m.fl_t; c[o + 1] = m.fl_k; c[o + 2] = m.st_t; c[o + 3] = m.ht_t; c[o + 4] = m.dz.x; c[o + 5] = m.dz.v; c[o + 6] = m.dz_sd; c[o + 7] = m.dz_t; c[o + 8] = m.cf.x; c[o + 9] = m.cf.v; c[o + 10] = m.cf_t;
    c[o + 11] = m.hu.x; c[o + 12] = m.hu.v; c[o + 13] = m.hu_t; c[o + 14] = m.sk.x; c[o + 15] = m.sk.v; c[o + 16] = p.stw; c[o + 17] = p.stn; c[o + 18] = p.dzw; c[o + 19] = if p.gun { 1.0 } else { 0.0 };
    for i in 0..3 { c[o + 20 + i] = p.gun_p[i]; }
    for i in 0..4 { c[o + 23 + i] = p.gun_r[i]; }
}
fn mv_post_cols_put(an: &mut Anim, c: &[f64], o: usize) {
    {
        let m = &mut an.mvs;
        m.fl_t = c[o]; m.fl_k = c[o + 1]; m.st_t = c[o + 2]; m.ht_t = c[o + 3]; m.dz.x = c[o + 4]; m.dz.v = c[o + 5]; m.dz_sd = c[o + 6]; m.dz_t = c[o + 7]; m.cf.x = c[o + 8]; m.cf.v = c[o + 9]; m.cf_t = c[o + 10];
        m.hu.x = c[o + 11]; m.hu.v = c[o + 12]; m.hu_t = c[o + 13]; m.sk.x = c[o + 14]; m.sk.v = c[o + 15];
    }
    let p = &mut an.mv_po;
    p.stw = c[o + 16]; p.stn = c[o + 17]; p.dzw = c[o + 18]; p.gun = c[o + 19] != 0.0;
    p.gun_p = [c[o + 20], c[o + 21], c[o + 22]];
    p.gun_r = [c[o + 23], c[o + 24], c[o + 25], c[o + 26]];
}
fn mv_post_arm_cols_get(an: &Anim, c: &mut [f64], o: usize) { c[o] = an.mvs.ts_t; c[o + 1] = an.mvs.arm_l; }
fn mv_post_arm_cols_put(an: &mut Anim, c: &[f64], o: usize) { an.mvs.ts_t = c[o]; an.mvs.arm_l = c[o + 1]; }
fn hands_pass_cols_get(an: &Anim, c: &mut [f64], o: usize) {
    let h = &an.hn;
    c[o] = h.grip[0]; c[o + 1] = h.grip[1]; c[o + 2] = h.fired; c[o + 3] = h.off; c[o + 4] = h.gun;
}
fn hands_pass_cols_put(an: &mut Anim, c: &[f64], o: usize) {
    let h = &mut an.hn;
    h.grip[0] = c[o]; h.grip[1] = c[o + 1]; h.fired = c[o + 2]; h.off = c[o + 3]; h.gun = c[o + 4];
}
/// handCols' get (character.ts:3403, another part's table, read here by handsPassAsk): the hand pass's 130 columns over; nothing comes back in that ask
fn hand_cols_get(an: &Anim, c: &mut [f64], o: usize) {
    let h = &an.hn;
    for sd in 0..2 {
        let (b, nt) = (&h.base[sd], &h.note[sd]);
        for i in 0..60 { c[o + sd * 60 + i] = b[i] as f64; }
        for i in 0..5 { c[o + 120 + sd * 5 + i] = nt[i] as f64; }
    }
}

// ---- the action clip

/// The whole-body action clip over the finished pA: clocks and weight here, the blend in the layer. The weight used (0 = untouched). (character.ts actLayer)
pub(super) fn anim_act_layer(g: &mut crate::game::Game, c: CharId, dt: f64) -> f64 {
    let a = g.ch[c].anim.act.clone();
    if let Some(a) = &a {
        if a.clip != g.ch[c].anim.act_name {
            let rig = g.ch[c].anim.rig;
            let clip = g.tab.rig[rig].clips.get(a.clip.clone()).copied();
            let an = &mut g.ch[c].anim;
            an.act_name = a.clip.clone();
            an.act_clip = clip;
            an.act_t = 0.0;
        }
        let an = &mut g.ch[c].anim;
        an.act_out = a.out.unwrap_or(0.35);
        an.act_loop = a.loop_ != Some(false);
    }
    let an = &mut g.ch[c].anim;
    let target = a.is_some() && an.act_clip.is_some();
    let secs = if target { m::max(0.02, a.as_ref().unwrap().in_.unwrap_or(0.3)) } else { m::max(0.02, an.act_out) };
    an.act_w = if target { m::min(1.0, an.act_w + dt / secs) } else { m::max(0.0, an.act_w - dt / secs) };
    if an.act_w <= 0.0 {
        if a.is_none() {
            an.act_name = String::new();
            an.act_clip = None;
            an.act_t = 0.0;
        }
        return 0.0;
    }
    let Some(clip) = an.act_clip else { return 0.0 };
    an.act_t += dt * a.as_ref().and_then(|a| a.rate).unwrap_or(1.0);
    let x = an.act_w;
    let w = x * x * (3.0 - 2.0 * x);
    anim_act_pose(g, c, clip, w, dt);
    w
}

/// The action's clip over the pose at weight `w`, and the slump. (character.ts actPose)
fn anim_act_pose(g: &mut crate::game::Game, c: CharId, clip: ClipId, w: f64, dt: f64) {
    let slot = crate::anim_skeleton::layer_slot(g, c);
    anim_act_pose_ask(g, c, slot, clip, w, dt);
}

/// rust/sim layers act_pose, asked. (character.ts actPoseAsk)
fn anim_act_pose_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, clip: ClipId, w: f64, dt: f64) {
    let m = args();
    m[0] = w;
    let n = g.ch[c].anim.p_a.nodes as f64;
    m[1] = crate::anim_skeleton::clip_no(g, clip, n);
    let an = &g.ch[c].anim;
    m[2] = an.act_t; m[3] = if an.act_loop { 1.0 } else { 0.0 }; m[4] = dt; m[5] = an.slump;
    act_cols_get(an, args(), COLS);
    let (at, at_b, at_rest) = ats_abr(g, c);
    let code = ::sim::layers::act_pose(g_2 as usize, at, at_b, at_rest) as f64;
    crate::anim_skeleton::layer_ran(g, code, c);
    act_cols_put(&mut g.ch[c].anim, args(), COLS);
}

// ---- MOVES

/// Before the stances and the arms: the recoil's sums bled off and the pelvis's share of how a side carries itself. What the trunk layer needs. (character.ts mvPre)
pub(super) fn anim_mv_pre(g: &mut crate::game::Game, c: CharId, dt: f64, move_w: f64, stand_w: f64) -> crate::character::MvGait {
    let slot = crate::anim_skeleton::layer_slot(g, c);
    anim_mv_pre_ask(g, c, slot, dt, move_w, stand_w);
    g.ch[c].anim.mv_out
}

/// A lean round an edge, eased. (character.ts leanBody)
pub(super) fn anim_lean_body(g: &mut crate::game::Game, c: CharId, dt: f64) {
    let slot = crate::anim_skeleton::layer_slot(g, c);
    anim_lean_body_ask(g, c, slot, dt);
}

/// rust/sim layers lean_body, asked. (character.ts leanBodyAsk)
fn anim_lean_body_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, dt: f64) {
    let a = args();
    a[0] = dt; a[1] = g.ch[c].anim.lean;
    lean_cols_get(&g.ch[c].anim, args(), COLS);
    let at = g.ch[c].anim.p_a.at as usize;
    let code = ::sim::layers::lean_body(g_2 as usize, at) as f64;
    crate::anim_skeleton::layer_ran(g, code, c);
    lean_cols_put(&mut g.ch[c].anim, args(), COLS);
}

/// The gaze stabiliser's working out: the yaw the neck and head take out this frame. (character.ts gazeKeep)
pub(super) fn anim_gaze_keep(g: &mut crate::game::Game, c: CharId, dt: f64, want_w: f64, crouch_pose: f64, w_sprint: f64) -> f64 {
    let slot = crate::anim_skeleton::layer_slot(g, c);
    anim_gaze_keep_ask(g, c, slot, dt, want_w, crouch_pose, w_sprint)
}

/// rust/sim layers gaze_keep, asked. (character.ts gazeKeepAsk)
fn anim_gaze_keep_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, dt: f64, want_w: f64, crouch_pose: f64, w_sprint: f64) -> f64 {
    let a = args();
    let an = &g.ch[c].anim;
    let gl = an.gaze_local;
    a[0] = dt; a[1] = want_w; a[2] = crouch_pose; a[3] = w_sprint; a[4] = an.nv_a; a[5] = an.mvs.hu.x; a[6] = an.mvs.sk.x; a[7] = gl[0]; a[8] = gl[1]; a[9] = gl[2];
    gaze_cols_get(an, args(), COLS);
    let at = an.p_a.at as usize;
    let code = ::sim::layers::gaze_keep(g_2 as usize, at) as f64;
    crate::anim_skeleton::layer_ran(g, code, c);
    gaze_cols_put(&mut g.ch[c].anim, args(), COLS);
    args_read()[COLS + 4]
}

/// The slot a rule of the character's that keeps no pose is asked in. (character.ts ruleSlot)
pub(super) fn anim_rule_slot(g: &mut crate::game::Game, c: CharId) -> f64 {
    crate::anim_skeleton::layer_slot(g, c)
}

/// What such a rule's export answered. (character.ts ruleRan)
pub(super) fn anim_rule_ran(g: &mut crate::game::Game, c: CharId, code: f64) {
    crate::anim_skeleton::layer_ran(g, code, c);
}

/// rust/sim layers mv_pre, asked. (character.ts mvPreAsk)
fn anim_mv_pre_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, dt: f64, move_w: f64, stand_w: f64) {
    let a = args();
    let an = &g.ch[c].anim;
    a[0] = dt; a[1] = move_w; a[2] = stand_w; a[3] = an.trained; a[4] = if an.long_gun { 1.0 } else { 0.0 }; a[5] = an.dip; a[6] = an.lg.w.x; a[7] = an.upper_w.aim;
    mv_pre_cols_get(an, args(), COLS);
    let at = an.p_a.at as usize;
    let code = ::sim::layers::mv_pre(g_2 as usize, at) as f64;
    crate::anim_skeleton::layer_ran(g, code, c);
    mv_pre_cols_put(&mut g.ch[c].anim, args(), COLS);
}

/// The trunk's share, into pA right before the long gun's own trunk set. (character.ts mvTrunk)
pub(super) fn anim_mv_trunk(g: &mut crate::game::Game, c: CharId, w: f64, o: crate::character::MvGait, aim_pass: bool) {
    if w < 0.003 { return; }
    let slot = crate::anim_skeleton::layer_slot(g, c);
    anim_mv_trunk_ask(g, c, slot, w, o, aim_pass);
}

/// rust/sim layers mv_trunk, asked. (character.ts mvTrunkAsk)
fn anim_mv_trunk_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, w: f64, o: crate::character::MvGait, aim_pass: bool) {
    let a = args();
    let an = &g.ch[c].anim;
    let m = &an.mvs;
    a[0] = w; a[1] = o.g; a[2] = o.u; a[3] = o.st; a[4] = if aim_pass { 1.0 } else { 0.0 }; a[5] = an.phase; a[6] = an.speed; a[7] = m.rock; a[8] = m.sho; a[9] = m.head;
    let at = an.p_a.at as usize;
    let code = ::sim::layers::mv_trunk(g_2 as usize, at) as f64;
    crate::anim_skeleton::layer_ran(g, code, c);
}

/// MOVES, the second pass: the arms the fight does not read, after every other layer: the magazine change, the hand to his ribs, a figure kept down. (character.ts mvArms)
pub(super) fn anim_mv_arms(g: &mut crate::game::Game, c: CharId, dt: f64, gate: f64) {
    let an = &g.ch[c].anim;
    let (sig_t, tell_t, ts_t) = (an.sig_t, an.tell_t, an.mvs.ts_t);
    let free = gate * (1.0 - an.mvs.arm_l) * (if sig_t >= 0.0 { 1.0 - v3::smoothstep(0.0, 0.08, sig_t) } else { 1.0 });
    let busy = sig_t >= 0.0 || tell_t >= 0.0 || ts_t >= 0.0;
    // ---- the long gun's magazine change
    {
        let mut live = false;
        let (reload_at, row) = (g.ch[c].anim.reload_at, g.ch[c].anim.reload_row);
        if let (true, Some(row)) = (reload_at >= 0.0, row) {
            anim_rl_beat(g, c, row);   // (`rlSrc` only saved re-running rlBeat, a pure function of the row: rl_b comes out the same each frame)
            let an = &mut g.ch[c].anim;
            an.mv2.rl_at = an.reload_at;
            an.rl_t = -1.0;
            live = true;
        } else if g.ch[c].anim.rl_t >= 0.0 {
            let an = &mut g.ch[c].anim;
            an.mv2.rl_at = an.rl_t;
            live = true;
            an.rl_t += dt;
            if an.rl_t > an.rl_b.dur { an.rl_t = -1.0; }
        }
        let an = &g.ch[c].anim;
        let (long_gun, reload_mag, has_feed) = (an.long_gun, an.reload_mag, an.feed.is_some());
        let pist = an.rl_b.hand && !long_gun && an.reload_t >= 0.0;
        let (throw_t, strike_t, no_tool) = (an.throw_t, an.strike_t, an.big_tool.is_none());
        let want = if live
            && (pist || (mv_on(g, MvOnK::Reload) && long_gun && (reload_mag || has_feed)))
            && !busy && throw_t < 0.0 && strike_t < 0.0 && no_tool { 1.0 } else { 0.0 };
        if want > 0.0 || g.ch[c].anim.mv2.rw.x != 0.0 {
            let rate = MV2.reload.rate;
            let rw = &mut g.ch[c].anim.mv2.rw;
            spring_step(rw, want, rate, dt);
            if want == 0.0 && rw.x < 0.01 { rw.x = 0.0; rw.v = 0.0; }
        }
        let an = &mut g.ch[c].anim;
        let d = an.rl_b.dur;
        let rl_at = an.mv2.rl_at;
        let w = bump(rl_at, 0.0, 0.1, d - 0.24, d - 0.02) * v3::clamp(an.mv2.rw.x, 0.0, 1.0) * free * (if pist { 1.0 } else { v3::smoothstep(0.3, 0.9, v3::clamp(an.lg.w.x, 0.0, 1.0)) });
        an.mv2_shown.reload = if w > 0.003 { w } else { 0.0 };
        if an.mv2_shown.reload > 0.0 {
            if pist { anim_pistol_reload_arm(g, c, rl_at, w); } else { anim_reload_arm(g, c, rl_at, w); }
        }
    }
    // ---- a man knocked about and still up: the support hand off the gun and pressed to his ribs
    {
        let an = &g.ch[c].anim;
        let (hurt, aim, clutch_in, off_hand) = (an.hurt, an.upper_w.aim, an.clutch, an.throw_t < 0.0 && an.strike_t < 0.0 && an.big_tool.is_none());
        let reload_shown = an.mv2_shown.reload;
        let rest = if hurt > 0.0 { v3::clamp(hurt, 0.0, 1.0) * MV2.hurt.arm * (1.0 - v3::smoothstep(0.1, 0.5, aim)) * (if off_hand { 1.0 } else { 0.0 }) } else { 0.0 };
        let want = if mv_on(g, MvOnK::Clutch) && reload_shown == 0.0 && !busy { v3::clamp(m::max(clutch_in, rest), 0.0, 1.0) } else { 0.0 };
        if want > 0.0 || g.ch[c].anim.mv2.cl.x != 0.0 {
            let rate = MV2.clutch.rate;
            let cl = &mut g.ch[c].anim.mv2.cl;
            spring_step(cl, want, rate, dt);
            if want == 0.0 && cl.x < 0.01 { cl.x = 0.0; cl.v = 0.0; }
        }
        let an = &mut g.ch[c].anim;
        an.mv2_shown.clutch = v3::clamp(an.mv2.cl.x, 0.0, 1.0) * free;
        if an.mv2_shown.clutch > 0.003 {
            let shown = an.mv2_shown.clutch;
            anim_clutch_arm(g, c, shown);
        } else {
            an.mv2_shown.clutch = 0.0;
        }
    }
    // ---- kept down: a hostage's wrists behind her back, hands on the head for somebody told to stay where he is
    {
        let mv_bound = mv_on(g, MvOnK::Bound);
        let an = &mut g.ch[c].anim;
        let bound = an.bound;
        let on = mv_bound && bound != 0.0 && !an.long_gun;
        let want = if on && (an.mv2.bd.x < 0.02 || an.mv2.bd_k == bound) { 1.0 } else { 0.0 };
        if on && an.mv2.bd.x < 0.02 { an.mv2.bd_k = bound; }
        if want > 0.0 || an.mv2.bd.x != 0.0 {
            let rate = MV2.bound.rate;
            let bd = &mut g.ch[c].anim.mv2.bd;
            spring_step(bd, want, rate, dt);
            if want == 0.0 && !on && bd.x < 0.01 { bd.x = 0.0; bd.v = 0.0; }
        }
        let an = &mut g.ch[c].anim;
        an.mv2_shown.bound = v3::clamp(an.mv2.bd.x, 0.0, 1.0) * free;
        if an.mv2_shown.bound > 0.003 {
            let (shown, kind) = (an.mv2_shown.bound, an.mv2.bd_k);
            anim_bound_arms(g, c, shown, kind);
        } else {
            an.mv2_shown.bound = 0.0;
        }
    }
}

/// The trunk's own frame off the joints of `fk`: across the shoulders to his left, up the spine, out of the chest. (character.ts trunkFrame)
pub(super) fn anim_trunk_frame(g: &crate::game::Game, c: CharId) -> crate::character::AnimTrunkFrame {
    let an = &g.ch[c].anim;
    let (nd, fk) = (&an.nodes, &an.fk);
    let up = v3::normalize(v3::sub(fk.pos[nd.spine3], fk.pos[nd.hips]));
    let mut left = v3::sub(fk.pos[nd.upper_arm_l], fk.pos[nd.upper_arm_r]);
    left = v3::normalize(v3::mad(left, up, -v3::dot(left, up)));
    AnimTrunkFrame { left, up, fwd: v3::cross(left, up) }
}

/// The long gun's magazine change on the support arm: the gun's own way of being fed, where it has one. (character.ts reloadArm)
fn anim_reload_arm(g: &mut crate::game::Game, c: CharId, t: f64, w: f64) {
    let an = &g.ch[c].anim;
    let (feed, b, n, port) = (an.feed, an.rl_b, an.feed_n, an.feed_port);
    let own = match feed {
        Some(model) => {
            let beat = crate::gunreload::FeedBeat { dur: b.dur, out: b.out, in_: b.in_, rack: b.rack, pull: b.pull };
            match crate::gunreload::feed_path(g, model, beat, n, port) {
                Some(crate::gunreload::FeedPath::Arm0(p)) => Some(AnimReloadArmAskOwn::Arm0(p)),
                Some(crate::gunreload::FeedPath::Arm1(p)) => Some(AnimReloadArmAskOwn::Arm1(p)),
                None => None,
            }
        }
        None => None,
    };
    let slot = crate::anim_skeleton::layer_slot(g, c);
    if let Some(AnimReloadArmAskOwn::Arm0(p)) = &own {
        crate::anim_skeleton::layer_fits("a reload's list of the hand's stations (gunreload.ts feedPath)", p.keys.len() as f64, crate::anim_skeleton::LAYER_KEYS_MAX, None);
    }
    anim_reload_arm_ask(g, c, slot, t, w, own.as_ref());
}

/// A reload's beat and the left hand's fix as the module's layers are handed them: args 3‥9 and 11‥14. (character.ts beatToBench)
fn anim_beat_to_bench(g: &mut crate::game::Game, c: CharId) {
    let a = args();
    let an = &g.ch[c].anim;
    let (b, f) = (&an.rl_b, &an.hand_fix[0]);
    a[3] = b.dur; a[4] = b.out; a[5] = b.in_; a[6] = b.rack; a[7] = if b.pull { 1.0 } else { 0.0 }; a[8] = b.keep; a[9] = b.fresh;
    for i in 0..4 { a[11 + i] = f[i]; }
}

/// rust/sim layers reload_arm, asked through the hand pass's columns. (character.ts reloadArmAsk)
fn anim_reload_arm_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, t: f64, w: f64, own: Option<&crate::character::AnimReloadArmAskOwn>) {
    let a = args();
    let keys = match own { Some(AnimReloadArmAskOwn::Arm0(p)) => Some(&p.keys), _ => None };
    let an = &g.ch[c].anim;
    let (trained, reload_back, long_gun) = (an.trained, an.reload_back, an.long_gun);
    // K: [mag, strip, handle, pull, hf, hb]
    let r = MV2.reload;
    let bx = [r.box_.mag, r.box_.strip, r.box_.handle, r.box_.pull, r.box_.hf, r.box_.hb];
    let k = match own {
        Some(AnimReloadArmAskOwn::Arm1(p)) => match p.marks {
            Some(f) => [f.mag, f.strip, f.handle, f.pull, f.hf, f.hb],
            None => bx,
        },
        _ if reload_back => [r.back.mag, r.back.strip, r.back.handle, r.back.pull, r.back.hf, r.back.hb],
        _ => bx,
    };
    a[0] = t; a[1] = w; a[2] = v3::clamp(trained, 0.0, 1.0);
    anim_beat_to_bench(g, c);
    a[10] = if long_gun { 1.0 } else { 0.0 };
    a[15] = match keys { Some(ks) => ks.len() as f64, None => -1.0 };
    for i in 0..3 {
        a[16 + i] = k[0][i]; a[19 + i] = k[1][i]; a[22 + i] = k[2][i]; a[25 + i] = k[3][i]; a[28 + i] = k[4][i]; a[31 + i] = k[5][i];
    }
    if let Some(ks) = keys {
        for (i, kk) in ks.iter().enumerate() {
            let q = KEYS + i * KEY;
            let at = match kk.at { crate::gunreload::FeedKeyAt::Pouch => None, crate::gunreload::FeedKeyAt::List(p) => Some(p) };
            a[q] = kk.t;
            a[q + 1] = if at.is_some() { 0.0 } else { 1.0 };
            a[q + 2] = match at { Some(p) => p[0], None => 0.0 };
            a[q + 3] = match at { Some(p) => p[1], None => 0.0 };
            a[q + 4] = match at { Some(p) => p[2], None => 0.0 };
            a[q + 5] = match kk.how { crate::gunreload::FeedKeyHow::Top => 1.0, crate::gunreload::FeedKeyHow::Pouch => 2.0, crate::gunreload::FeedKeyHow::Mag => 0.0 };
            a[q + 6] = kk.grip;
        }
    }
    let (slot, (at, at_b, at_rest)) = (g_2 as usize, ats_abr(g, c));
    anim_hands_ask(g, c, &mut |_g| ::sim::layers::reload_arm(slot, at, at_b, at_rest) as f64);
}

/// A pistol's magazine change, the support hand's. (character.ts pistolReloadArm)
fn anim_pistol_reload_arm(g: &mut crate::game::Game, c: CharId, t: f64, w: f64) {
    let slot = crate::anim_skeleton::layer_slot(g, c);
    anim_pistol_reload_arm_ask(g, c, slot, t, w);
}

/// rust/sim layers pistol_reload_arm, asked through the hand pass's columns. (character.ts pistolReloadArmAsk)
fn anim_pistol_reload_arm_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, t: f64, w: f64) {
    let a = args();
    a[0] = t; a[1] = w;
    anim_beat_to_bench(g, c);
    let (slot, (at, at_b, at_rest)) = (g_2 as usize, ats_abr(g, c));
    anim_hands_ask(g, c, &mut |_g| ::sim::layers::pistol_reload_arm(slot, at, at_b, at_rest) as f64);
}

/// The support hand pressed to his ribs. (character.ts clutchArm)
fn anim_clutch_arm(g: &mut crate::game::Game, c: CharId, w: f64) {
    let slot = crate::anim_skeleton::layer_slot(g, c);
    anim_clutch_arm_ask(g, c, slot, w);
}

/// rust/sim layers clutch_arm, asked through the hand pass's columns. (character.ts clutchArmAsk)
fn anim_clutch_arm_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, w: f64) {
    let a = args();
    let f = &g.ch[c].anim.hand_fix[0];
    a[0] = w;
    for i in 0..4 { a[1 + i] = f[i]; }
    let (slot, (at, at_b, at_rest)) = (g_2 as usize, ats_abr(g, c));
    anim_hands_ask(g, c, &mut |_g| ::sim::layers::clutch_arm(slot, at, at_b, at_rest) as f64);
}

/// A figure kept down: wrists behind the back, or hands on the head. (character.ts boundArms)
fn anim_bound_arms(g: &mut crate::game::Game, c: CharId, w: f64, kind: f64) {
    let slot = crate::anim_skeleton::layer_slot(g, c);
    anim_bound_arms_ask(g, c, slot, w, kind);
}

/// rust/sim layers bound_arms, asked through the hand pass's columns. (character.ts boundArmsAsk)
fn anim_bound_arms_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, w: f64, kind: f64) {
    let a = args();
    let an = &g.ch[c].anim;
    let (f, gl) = (&an.hand_fix, an.gaze_local);
    a[0] = w; a[1] = kind;
    for i in 0..3 { a[2 + i] = gl[i]; }
    for i in 0..4 { a[5 + i] = f[0][i]; a[9 + i] = f[1][i]; }
    let (slot, (at, at_b, at_rest)) = (g_2 as usize, ats_abr(g, c));
    anim_hands_ask(g, c, &mut |_g| ::sim::layers::bound_arms(slot, at, at_b, at_rest) as f64);
}

/// The reactions: where the dazzle is from in his own frame, then their body, then their arms. (character.ts mvPost)
pub(super) fn anim_mv_post(g: &mut crate::game::Game, c: CharId, dt: f64, gate: f64) {
    if let Some(dz) = g.ch[c].anim.dazzle {
        let (pos, was) = (g.ch[c].pos, g.ch[c].anim.mvs.dz_d);
        let d = anim_flat_model(g, c, v3::sub(dz.from, pos), was);
        g.ch[c].anim.mvs.dz_d = d;
    }
    anim_mv_post_body(g, c, dt, gate);
    anim_mv_post_arms(g, c, dt, gate);
}

/// The reactions' body, summed into one turn of the trunk, one of the head and one move of the pelvis. (character.ts mvPostBody)
fn anim_mv_post_body(g: &mut crate::game::Game, c: CharId, dt: f64, gate: f64) {
    let slot = crate::anim_skeleton::layer_slot(g, c);
    anim_mv_post_body_ask(g, c, slot, dt, gate);
}

/// rust/sim layers mv_post_body, asked. (character.ts mvPostBodyAsk)
fn anim_mv_post_body_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, dt: f64, gate: f64) {
    let a = args();
    let an = &g.ch[c].anim;
    let (m, dz) = (&an.mvs, &an.dazzle);
    a[0] = dt; a[1] = gate; a[2] = an.trained; a[3] = an.lg.w.x; a[4] = m.fl_k_to;
    for j in 0..3 { a[5 + j] = m.fl_d[j]; a[8 + j] = m.st_d[j]; a[11 + j] = m.ht_d[j]; a[17 + j] = m.dz_d[j]; }
    a[14] = m.ht_side;
    a[15] = if dz.is_some() { 1.0 } else { 0.0 };
    a[16] = match dz { Some(d) => d.k, None => 0.0 };
    a[20] = an.clutch;
    let fold_too = an.fold_too;
    a[21] = if fold_on(g, Some(fold_too)) { 1.0 } else { 0.0 };
    a[22] = if fold_too { 1.0 } else { 0.0 };
    let an = &g.ch[c].anim;
    a[23] = an.hurt; a[24] = an.hurt_side; a[25] = an.upper_w.aim; a[26] = an.speed; a[27] = an.phase; a[28] = an.shaken; a[29] = an.mvs.ts_t; a[30] = an.mvs.ts_rel;
    mv_post_cols_get(an, args(), COLS);
    let at = an.p_a.at as usize;
    let code = ::sim::layers::mv_post_body(g_2 as usize, at) as f64;
    crate::anim_skeleton::layer_ran(g, code, c);
    mv_post_cols_put(&mut g.ch[c].anim, args(), COLS);
}

/// The reactions' arms, after their body: the start's, a blinded man's gun and hand, the toss. (character.ts mvPostArms)
fn anim_mv_post_arms(g: &mut crate::game::Game, c: CharId, dt: f64, gate: f64) {
    let an = &g.ch[c].anim;
    let (m, p) = (&an.mvs, &an.mv_po);
    if !(p.stw > 0.003) && !p.gun && !(p.dzw > 0.003) && !(m.ts_t >= 0.0) { return; }
    let slot = crate::anim_skeleton::layer_slot(g, c);
    anim_mv_post_arms_ask(g, c, slot, dt, gate);
}

/// rust/sim layers mv_post_arms, asked through its own columns and the hand pass's. (character.ts mvPostArmsAsk)
fn anim_mv_post_arms_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, dt: f64, gate: f64) {
    let a = args();
    let an = &g.ch[c].anim;
    let (m, p, gl, f) = (&an.mvs, &an.mv_po, &an.gaze_local, &an.hand_fix[0]);
    a[0] = dt; a[1] = gate; a[2] = an.lg.w.x; a[3] = m.ts_rel; a[4] = m.dz_t; a[12] = p.stw; a[13] = p.stn; a[14] = p.dzw; a[15] = if p.gun { 1.0 } else { 0.0 };
    for i in 0..3 { a[5 + i] = gl[i]; a[16 + i] = p.gun_p[i]; }
    for i in 0..4 { a[8 + i] = f[i]; a[19 + i] = p.gun_r[i]; }
    mv_post_arm_cols_get(an, args(), COLS);
    let (slot, (at, at_b, at_rest)) = (g_2 as usize, ats_abr(g, c));
    anim_hands_ask(g, c, &mut |_g| ::sim::layers::mv_post_arms(slot, at, at_b, at_rest) as f64);
    mv_post_arm_cols_put(&mut g.ch[c].anim, args(), COLS);
}

// ---- procedural layers

/// One node of PoseFK.run's arithmetic, operand for operand, under its parent's rotation and origin. (the body of character.ts fkPart)
#[inline]
fn fk_node(d: &[f32], n: usize, i: usize, pr: Quat, pp: V3) -> (Quat, V3) {
    let (rx, ry, rz, rw) = (d[3 * n + i] as f64, d[4 * n + i] as f64, d[5 * n + i] as f64, d[6 * n + i] as f64);
    let (qx, qy, qz, qw) = (pr[0], pr[1], pr[2], pr[3]);
    let (v0, v1, v2) = (d[i] as f64, d[n + i] as f64, d[2 * n + i] as f64);
    let (tx, ty, tz) = (2.0 * (qy * v2 - qz * v1), 2.0 * (qz * v0 - qx * v2), 2.0 * (qx * v1 - qy * v0));
    let (t0, t1, t2) = (v0 + qw * tx + (qy * tz - qz * ty), v1 + qw * ty + (qz * tx - qx * tz), v2 + qw * tz + (qx * ty - qy * tx));
    (
        [
            qw * rx + qx * rw + qy * rz - qz * ry,
            qw * ry - qx * rz + qy * rw + qz * rx,
            qw * rz + qx * ry - qy * rx + qz * rw,
            qw * rw - qx * rx - qy * ry - qz * rz,
        ],
        [pp[0] + t0, pp[1] + t1, pp[2] + t2],
    )
}

/// fk.run() for a few nodes: those of `list` hold exactly what a full run would give them, the rest of fk stands as the last run left it. (character.ts fkPart)
pub(super) fn anim_fk_part(g: &mut crate::game::Game, c: CharId, pose: crate::anim_skeleton::Pose, list: &[i32]) {
    let n = crate::anim_skeleton::pose_rows(pose.nodes as f64) as usize;
    let rig = g.ch[c].anim.rig;
    for k in 0..list.len() {
        let i = list[k] as usize;
        let par = g.tab.rig[rig].nodes[i].parent;
        if par < 0.0 {
            let d = crate::anim_skeleton::pose_cols(g, pose);
            let (r, t) = ([d[3 * n + i] as f64, d[4 * n + i] as f64, d[5 * n + i] as f64, d[6 * n + i] as f64], [d[i] as f64, d[n + i] as f64, d[2 * n + i] as f64]);
            let fk = &mut g.ch[c].anim.fk;
            fk.rot[i] = r;
            fk.pos[i] = t;
            continue;
        }
        let fk = &g.ch[c].anim.fk;
        let (pr, pp) = (fk.rot[par as usize], fk.pos[par as usize]);
        let (q, p) = fk_node(crate::anim_skeleton::pose_cols(g, pose), n, i, pr, pp);
        let fk = &mut g.ch[c].anim.fk;
        fk.rot[i] = q;
        fk.pos[i] = p;
    }
}

/// Curl the four fingers and set the thumb of one hand, from the rest hand; and the hand pass's note of how far this shape stands from the locomotion's. (character.ts hand)
pub(super) fn anim_hand(g: &mut crate::game::Game, c: CharId, pose: crate::anim_skeleton::Pose, side: f64, curl: &[f64], thumb: f64, spread: Option<f64>) {
    const J: [f64; 3] = [72.0, 92.0, 62.0];
    const TJ: [f64; 3] = [30.0, 25.0, 45.0];
    let spread = spread.unwrap_or(0.0);
    let sd = side as usize;
    let sign = if js::truthy(side) { 1.0 } else { -1.0 };
    let rest = g.ch[c].anim.rest;
    for fi in 0..g.ch[c].anim.fingers[sd].len() {
        for j in 0..g.ch[c].anim.fingers[sd][fi].len() {
            let node = g.ch[c].anim.fingers[sd][fi][j];
            let mut q = v3::quat::axis_angle(ax(), curl[fi] * J[j] * DEGR);
            if j == 0 && js::truthy(spread) {
                q = v3::quat::mul(v3::quat::axis_angle(az(), (fi as f64 - 1.2) * spread * sign * 9.0 * DEGR), q);
            }
            let r = v3::quat::normalize(v3::quat::mul(rot(g, rest, node), q));
            set_rot(g, pose, node, r);
        }
    }
    for j in 0..g.ch[c].anim.thumbs[sd].len() {
        let node = g.ch[c].anim.thumbs[sd][j];
        let r = v3::quat::normalize(v3::quat::mul(rot(g, rest, node), v3::quat::axis_angle(ax(), thumb * TJ[j] * DEGR)));
        set_rot(g, pose, node, r);
    }
    if !g.ch[c].anim.hn.quiet {
        let n = crate::anim_skeleton::pose_rows(pose.nodes as f64);
        let b = core::mem::take(&mut g.ch[c].anim.hn.base[sd]);   // (lent out for the sums, as the pose's columns are asked of g beside it; nothing between here and its return reads it)
        for gi in 0..5 {
            let k = gi * 3;
            let ns = &g.ch[c].anim.hn.nodes[sd];
            let (n0, n1, n2) = (ns[k] as f64, ns[k + 1] as f64, ns[k + 2] as f64);
            let d = crate::anim_skeleton::pose_cols(g, pose);
            let sum = (q_ang(d, 3.0 * n + n0, n, &b, (k * 4) as f64) + q_ang(d, 3.0 * n + n1, n, &b, (k * 4 + 4) as f64) + q_ang(d, 3.0 * n + n2, n, &b, (k * 4 + 8) as f64)) / 3.0;
            let nt = &mut g.ch[c].anim.hn.note[sd];
            nt[gi] = m::min(nt[gi] as f64, sum) as f32;
        }
        g.ch[c].anim.hn.base[sd] = b;
    }
}

/// MOVES, crouched: the kneeling foot's toe brought back onto the floor. (character.ts toesUp)
pub(super) fn anim_toes_up(g: &mut crate::game::Game, c: CharId, w: f64) {
    let slot = crate::anim_skeleton::layer_slot(g, c);
    anim_toes_up_ask(g, c, slot, w);
}

/// rust/sim layers toes_up, asked. (character.ts toesUpAsk)
fn anim_toes_up_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, w: f64) {
    args()[0] = w;
    let at = g.ch[c].anim.p_a.at as usize;
    let code = ::sim::layers::toes_up(g_2 as usize, at) as f64;
    crate::anim_skeleton::layer_ran(g, code, c);
}

/// The man's own small hash of `k` to −1‥1 (handsBuild's `h`): the die of how his hands hang, drawing nothing from the game's. (character.ts:2984)
fn hand_hash(seed: f64, k: f64) -> f64 {
    let mut x = m::imul(js::to_i32((seed + 1.0) * 2654435761.0) ^ js::to_i32(k * 40503.0), js::to_i32(2246822519.0)) as u32;
    x ^= x >> 15;
    x = m::imul(x as i32, js::to_i32(3266489917.0)) as u32;
    x ^= x >> 13;
    x as f64 / 4294967296.0 * 2.0 - 1.0
}

/// This man's relaxed hands, as the turn from the library's fist on each digit joint, and the index laid straight for the trigger hand. (character.ts handsBuild)
fn anim_hands_build(g: &mut crate::game::Game, c: CharId) {
    use v3::quat;
    let hd = &HAND;
    let (seed, nodes, rest) = (g.ch[c].anim.hn.seed, g.ch[c].anim.p_a.nodes, g.ch[c].anim.rest);
    let p = crate::anim_skeleton::pose_new(g, nodes as usize);   // (its own scratch, once a man)
    crate::anim_skeleton::pose_copy(g, p, rest);
    let h = |k: usize| hand_hash(seed, k as f64);
    g.ch[c].anim.hn.quiet = true;
    g.ch[c].anim.hn.open = Vec::new();
    for sd in 0..2usize {
        let sign = if sd != 0 { 1.0 } else { -1.0 };
        let man = hd.vary * h(40) + hd.side_vary * h(41) * sign;
        let curl: Vec<f64> = hd.curl.iter().enumerate().map(|(fi, &cl)| v3::clamp(cl + man + hd.finger_vary * h(42 + sd * 4 + fi), 0.05, 0.9)).collect();
        anim_hand(g, c, p, sd as f64, &curl, v3::clamp(hd.thumb + 0.1 * h(50 + sd), 0.0, 1.0), Some(hd.spread + 0.06 * h(52 + sd)));
        let th = g.ch[c].anim.thumbs[sd].clone();
        let t0 = th[0];
        // the thumb up toward the palm's plane, about the fingers' axis
        let r = quat::normalize(quat::mul(quat::axis_angle(ay(), sign * hd.thumb_in * DEGR), rot(g, p, t0)));
        set_rot(g, p, t0, r);
        // …brought in toward the fingers' line, and its two outer joints bent
        let along = quat::rotate(rot(g, p, t0), ay());
        let ax = v3::cross(along, ay());
        let al = v3::len(ax);
        if al > 1e-4 && js::truthy(hd.thumb_close) {
            let r = quat::normalize(quat::mul(quat::axis_angle(v3::scale(ax, 1.0 / al), hd.thumb_close * DEGR), rot(g, p, t0)));
            set_rot(g, p, t0, r);
        }
        let mut up = rot(g, p, t0);
        if js::truthy(hd.thumb_bend) {
            for j in [1usize, 2] {
                let dd = quat::rotate(quat::mul(up, rot(g, p, th[j])), ay());
                let bx = v3::cross(dd, ay());
                let bl = v3::len(bx);
                if bl > 1e-4 {
                    let r = quat::normalize(quat::mul(quat::axis_angle(quat::rotate(quat::conj(up), v3::scale(bx, 1.0 / bl)), hd.thumb_bend * DEGR), rot(g, p, th[j])));
                    set_rot(g, p, th[j], r);
                }
                up = quat::mul(up, rot(g, p, th[j]));
            }
        }
        let mut o = vec![0.0f32; 60];
        for k in 0..15 {
            let node = g.ch[c].anim.hn.nodes[sd][k] as usize;
            let f = &g.ch[c].anim.hn.fist[sd];
            let fq = [f[k * 4] as f64, f[k * 4 + 1] as f64, f[k * 4 + 2] as f64, f[k * 4 + 3] as f64];
            let q = quat::mul(quat::conj(fq), rot(g, p, node));
            for i in 0..4 { o[k * 4 + i] = q[i] as f32; }
        }
        g.ch[c].anim.hn.open.push(o);
        for k in 0..3 {
            let node = g.ch[c].anim.hn.nodes[sd][k] as usize;
            let q = rot(g, p, node);
            for i in 0..4 { g.ch[c].anim.hn.thumb[sd][k * 4 + i] = q[i] as f32; }
        }
    }
    for j in 0..g.ch[c].anim.fingers[1][0].len() {
        let node = g.ch[c].anim.fingers[1][0][j];
        let mut q = quat::axis_angle(ax(), hd.trig[j] * DEGR);
        if j == 0 { q = quat::mul(quat::axis_angle(az(), -hd.trig_spread * DEGR), q); }
        let r = quat::normalize(quat::mul(rot(g, rest, node), q));
        for i in 0..4 { g.ch[c].anim.hn.trig[j * 4 + i] = r[i] as f32; }
    }
    g.ch[c].anim.hn.quiet = false;
    g.ch[c].anim.hn.built = seed;
    crate::anim_skeleton::pose_done(g, p);   // (the scratch is dropped)
}

/// The hand pass: digit joints of pA only. (character.ts handsPass)
pub(super) fn anim_hands_pass(g: &mut crate::game::Game, c: CharId, dt: f64, full_l: f64, full_r: f64, gun_r: f64) {
    if g.ch[c].anim.hn.built != g.ch[c].anim.hn.seed { anim_hands_build(g, c); }
    let slot = crate::anim_skeleton::layer_slot(g, c);
    anim_hands_pass_ask(g, c, slot, dt, full_l, full_r, gun_r);
}

/// The kept run of doubles current for the man's hand tables (made again when the tables are)
fn hand_tables_now(an: &mut Anim) {
    let built = an.hn.built;
    if an.hn_flat.is_some() && an.hn_flat_of == built { return; }
    let h = &an.hn;
    let t = an.hn_flat.get_or_insert_with(|| vec![0.0; 286]);
    for sd in 0..2 {
        for (i, x) in h.fist[sd].iter().enumerate() { t[sd * 60 + i] = *x as f64; }
        for (i, x) in h.open[sd].iter().enumerate() { t[120 + sd * 60 + i] = *x as f64; }
        for (i, x) in h.own[sd].iter().enumerate() { t[240 + sd * 5 + i] = *x as f64; }
        for (i, x) in h.thumb[sd].iter().enumerate() { t[250 + sd * 12 + i] = *x as f64; }
    }
    for (i, x) in h.trig.iter().enumerate() { t[274 + i] = *x as f64; }
    an.hn_flat_of = built;
}

/// rust/sim layers hands_pass, asked. (character.ts handsPassAsk)
fn anim_hands_pass_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, dt: f64, full_l: f64, full_r: f64, gun_r: f64) {
    let a = args();
    let an = &mut g.ch[c].anim;
    a[0] = dt; a[1] = full_l; a[2] = full_r; a[3] = gun_r; a[4] = if an.grip[0] { 1.0 } else { 0.0 }; a[5] = if an.grip[1] { 1.0 } else { 0.0 }; a[6] = an.trained;
    hands_pass_cols_get(an, args(), COLS);
    hand_cols_get(an, args(), HAND_COLS);
    hand_tables_now(an);
    let t = an.hn_flat.as_ref().unwrap();
    a[KEYS..KEYS + t.len()].copy_from_slice(t);
    let at = an.p_a.at as usize;
    let code = ::sim::layers::hands_pass(g_2 as usize, at) as f64;
    crate::anim_skeleton::layer_ran(g, code, c);
    hands_pass_cols_put(&mut g.ch[c].anim, args(), COLS);
}

/// Stance, body half. (character.ts stanceBody)
pub(super) fn anim_stance_body(g: &mut crate::game::Game, c: CharId, w: f64, speed: f64, stand: f64) {
    let slot = crate::anim_skeleton::layer_slot(g, c);
    anim_stance_body_ask(g, c, slot, w, speed, stand);
}

/// rust/sim layers stance_body, asked. (character.ts stanceBodyAsk)
fn anim_stance_body_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, w: f64, speed: f64, stand: f64) {
    let a = args();
    let an = &g.ch[c].anim;
    let st = &an.st;
    a[0] = w; a[1] = speed; a[2] = stand; a[3] = st.blade.x; a[4] = st.squat.x; a[5] = st.lean.x; a[6] = st.head_pitch.x; a[7] = st.head_yaw.x; a[8] = an.stack_press_sm;
    let at = an.p_a.at as usize;
    let code = ::sim::layers::stance_body(g_2 as usize, at) as f64;
    crate::anim_skeleton::layer_ran(g, code, c);
}

/// The stand pose: the idle and over it the cycle. (character.ts standPose)
pub(super) fn anim_stand_pose(g: &mut crate::game::Game, c: CharId, ph: f64, w_idle: f64, w_walk: f64, w_jog: f64, w_sprint: f64) {
    let slot = crate::anim_skeleton::layer_slot(g, c);
    anim_stand_pose_ask(g, c, slot, ph, w_idle, w_walk, w_jog, w_sprint);
}

/// rust/sim layers stand_pose, asked. (character.ts standPoseAsk)
fn anim_stand_pose_ask(g: &mut crate::game::Game, c: CharId, g_2: f64, ph: f64, w_idle: f64, w_walk: f64, w_jog: f64, w_sprint: f64) {
    let a = args();
    let an = &g.ch[c].anim;
    let n = an.p_a.nodes as f64;
    let i0 = (js::to_i32(m::floor(ph * 64.0)) & 63) as usize;
    let idle = *an.clips.get("Idle_Loop").unwrap();
    let jog = *an.clips.get("Jog_Fwd_Loop").unwrap();
    let sprint = *an.clips.get("Sprint_Loop").unwrap();
    let walk = *an.clips.get("Walk_Loop").unwrap();
    a[0] = ph; a[1] = w_idle; a[2] = w_walk; a[3] = w_jog; a[4] = w_sprint; a[5] = an.idle_t;
    a[6] = crate::anim_skeleton::clip_no(g, idle, n);
    a[7] = crate::anim_skeleton::clip_no(g, jog, n);
    a[8] = ph * g.tab.clip[jog].duration;
    a[9] = crate::anim_skeleton::clip_no(g, sprint, n);
    a[10] = ph * g.tab.clip[sprint].duration;
    a[11] = crate::anim_skeleton::clip_no(g, walk, n);
    a[12] = (ph + WALK_PHASE) * g.tab.clip[walk].duration;
    let an = &g.ch[c].anim;
    a[13] = ph; a[14] = an.jog_hips_y[i0] as f64; a[15] = an.jog_hips_y[(i0 + 1) & 63] as f64; a[16] = an.jog_hips_mean; a[17] = an.run_k; a[18] = an.jog_mid;
    a[19] = an.sprint_mid; a[20] = an.sprint_floor_y;
    let (at, at_b, at_c) = ats_abc(g, c);
    let code = ::sim::layers::stand_pose(g_2 as usize, at, at_b, at_c) as f64;
    crate::anim_skeleton::layer_ran(g, code, c);
}

/// The crouch laid over the stand pose. (character.ts crouchLaid)
pub(super) fn anim_crouch_laid(g: &mut crate::game::Game, c: CharId, ph: f64, w_move: f64, crouch_pose: f64) {
    let slot = crate::anim_skeleton::layer_slot(g, c);
    anim_crouch_laid_ask(g, c, slot, ph, w_move, crouch_pose);
}

#[cfg(test)]
mod tests {
    use super::*;

    /// the hash's bits, from the TypeScript's own expression under bun (tools: see notes)
    #[test]
    fn hand_hash_bits() {
        for &(seed, k, want) in HASH_BITS {
            assert_eq!(hand_hash(seed, k).to_bits(), want, "seed {} k {}", seed, k);
        }
    }

    /// fkPart's arithmetic for one node with a parent, against the TypeScript's own bits
    #[test]
    fn fk_node_bits() {
        let n = 2usize;
        let mut d = [0f32; 20];
        for (i, x) in FK_D.iter().enumerate() { d[i] = f32::from_bits(*x); }
        let pr: Quat = [f64::from_bits(FK_PR[0]), f64::from_bits(FK_PR[1]), f64::from_bits(FK_PR[2]), f64::from_bits(FK_PR[3])];
        let pp: V3 = [f64::from_bits(FK_PP[0]), f64::from_bits(FK_PP[1]), f64::from_bits(FK_PP[2])];
        let (q, p) = fk_node(&d, n, 1, pr, pp);
        for i in 0..4 { assert_eq!(q[i].to_bits(), FK_Q[i], "q{}", i); }
        for i in 0..3 { assert_eq!(p[i].to_bits(), FK_P[i], "p{}", i); }
    }
    const HASH_BITS: &[(f64, f64, u64)] = &[
        (0.0, 40.0, 0xbfe15ae6d0800000),
        (0.0, 41.0, 0xbfebd78d78800000),
        (0.0, 42.0, 0x3fe4b3ef42400000),
        (0.0, 46.0, 0x3fe569d1f2c00000),
        (0.0, 49.0, 0x3fd27b4173800000),
        (0.0, 50.0, 0xbfd6f2be09000000),
        (0.0, 53.0, 0x3feb139699800000),
        (1.0, 40.0, 0xbfec3bc7e1400000),
        (1.0, 41.0, 0xbf63397b40000000),
        (1.0, 42.0, 0x3fe0527f6e800000),
        (1.0, 46.0, 0xbfe1f1e867c00000),
        (1.0, 49.0, 0xbfe5e8b74c800000),
        (1.0, 50.0, 0xbfe2d92ec3000000),
        (1.0, 53.0, 0x3fc0e1d51b000000),
        (7.0, 40.0, 0xbfe9b7ef4c000000),
        (7.0, 41.0, 0x3fde9ebe61800000),
        (7.0, 42.0, 0x3fe5be5934000000),
        (7.0, 46.0, 0xbfe84dbc77400000),
        (7.0, 49.0, 0x3fe1bdf90a000000),
        (7.0, 50.0, 0x3fdd253894000000),
        (7.0, 53.0, 0x3fb1163222000000),
        (123456.0, 40.0, 0x3fd1ced949800000),
        (123456.0, 41.0, 0x3fe1e9e6ca000000),
        (123456.0, 42.0, 0xbf79b7c920000000),
        (123456.0, 46.0, 0x3feac96875000000),
        (123456.0, 49.0, 0x3fe7f49c82400000),
        (123456.0, 50.0, 0xbface6c0fc000000),
        (123456.0, 53.0, 0x3fcf951fb7000000),
        (3.5, 40.0, 0x3fe45d19ff800000),
        (3.5, 41.0, 0xbfc2fff1bd000000),
        (3.5, 42.0, 0x3fc1dc33d8000000),
        (3.5, 46.0, 0xbfd5157670800000),
        (3.5, 49.0, 0xbf6949fbc0000000),
        (3.5, 50.0, 0xbfe1c88e9ec00000),
        (3.5, 53.0, 0xbfe2de9fc1800000),
        (4294967295.0, 40.0, 0xbfe8e8cafe800000),
        (4294967295.0, 41.0, 0xbfd91b5768800000),
        (4294967295.0, 42.0, 0xbfe2f2e8e9800000),
        (4294967295.0, 46.0, 0x3fb378eaba000000),
        (4294967295.0, 49.0, 0x3fe014d5cc400000),
        (4294967295.0, 50.0, 0xbfe9ddf5f6400000),
        (4294967295.0, 53.0, 0x3fd80f19a5000000),
        (-1.0, 40.0, 0xbfe8e8cafe800000),
        (-1.0, 41.0, 0xbfd91b5768800000),
        (-1.0, 42.0, 0xbfe2f2e8e9800000),
        (-1.0, 46.0, 0x3fb378eaba000000),
        (-1.0, 49.0, 0x3fe014d5cc400000),
        (-1.0, 50.0, 0xbfe9ddf5f6400000),
        (-1.0, 53.0, 0x3fd80f19a5000000),
    ];
    const FK_D: [u32; 20] = [0x3dbb9eb0, 0x3e9052dd, 0xbe2830f1, 0xbe754e6e, 0x3e676768, 0x3e39ad16, 0xbf2d67d7, 0xbe8d3a73, 0x3f3f9a24, 0x3da9ec57, 0xbf451356, 0x3dec59b1, 0x3f3d7661, 0xbe9cbb7b, 0xbf2944b1, 0x3ef3f832, 0x3f09d58e, 0xbf1d80cc, 0xbec27ea8, 0x3f36900d];
    const FK_PR: [u64; 4] = [0x3fbf9add3746f4c6, 0xbfd55555555552fd, 0x3fe22c2a3d04eba6, 0x3fe796e924193c52];
    const FK_PP: [u64; 3] = [0x3ff0000000001198, 0xc001c71c71c71c40, 0x3fb999999999999a];
    const FK_Q: [u64; 4] = [0xbfd4e8f1ea4e0d35, 0xbf7f7fa595a48b00, 0xbfc5d1f86ea7838c, 0xbfcd5fe11d5182da];
    const FK_P: [u64; 3] = [0x3ff31ac0460ea885, 0xc00182abe9527f47, 0x3fdd7b49ac10adea];
}
