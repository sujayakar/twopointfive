use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

// ---- small helpers

/// pose.at: where a pose stands in its game's pool, as a layer of rust/sim takes it
fn at(pose: crate::anim_skeleton::Pose) -> usize { pose.at as usize }
/// SIGNAL_DUR[kind]
fn signal_dur_of(kind: SignalKind) -> f64 { *crate::character::signal_dur().get(String::from(kind.as_str())).unwrap() }
/// TELL_DUR[kind]
fn tell_dur_of(kind: TellKind) -> f64 { *crate::character::tell_dur().get(String::from(kind.as_str())).unwrap() }

// ---- the layers of rust/sim (args: their numbers already on the bench)

// ---- the layers' columns (the static LayerCols tables: get and put, by the order rust/sim/src/layers.rs gives)

// ---- the crouch's lay

/// the crouch over the stand pose: its numbers onto the bench and the layer
pub(super) fn anim_crouch_laid_ask(g: &mut crate::game::Game, c: CharId, ph: f64, w_move: f64, crouch_pose: f64) {
    let (idle, fwd) = (clip_of(g, c, "Crouch_Idle_Loop"), clip_of(g, c, "Crouch_Fwd_Loop"));
    let (idle_t, fwd_t) = (g.ch[c].anim.idle_t, ph * g.tab.clip[fwd].duration);
    let (idle, fwd) = (clip_at(g, c, idle, idle_t), clip_at(g, c, fwd, fwd_t));
    anim_layer(g, c, |s, an, k| ::sim::layers::crouch_pose(s, an, w_move, crouch_pose, idle, fwd, k));
}
/// a clip of his at `t` seconds, as a layer is handed one
pub(super) fn clip_at(g: &mut Game, c: CharId, clip: ClipId, t: f64) -> ::sim::layers::At {
    ::sim::layers::At { clip: crate::anim_skeleton::clip_no(g, clip, g.ch[c].anim.p_a.nodes as f64), t }
}

/// RUNARMS: the run's own arms carried close
pub(super) fn anim_run_arms(g: &mut crate::game::Game, c: CharId, pose: crate::anim_skeleton::Pose, w: f64, ws: f64) { anim_layer(g, c, |s, _, k| ::sim::layers::run_arms(s, at(pose), w, ws, k)); }

/// SOFT_KNEE: each leg re-solved with its reach eased below full extension
pub(super) fn anim_soft_knees(g: &mut crate::game::Game, c: CharId, pose: crate::anim_skeleton::Pose, w: f64) { anim_layer(g, c, |s, _, _| ::sim::layers::soft_knees(s, at(pose), w)); }

/// the run's stance feet (RUN_LINGER)
pub(super) fn anim_run_plant(g: &mut crate::game::Game, c: CharId, ph: f64, s: f64, dt: f64, w_run: f64, w_sprint: f64) {
    anim_layer(g, c, |slot, an, _| ::sim::layers::run_plant(slot, an, ph, s, dt, w_run, w_sprint));
}

/// idle life and weight: breath, the body's mass, the weight shift, the foot lock
pub(super) fn anim_life_layer(g: &mut crate::game::Game, c: CharId, dt: f64, s: f64, crouch_pose: f64, w: f64, w_f: f64) {
    let p0 = g.ch[c].pos;
    anim_layer(g, c, |slot, an, k| ::sim::layers::life_layer(slot, an, dt, s, crouch_pose, w, w_f, p0, k));
}

/// the rule of a foot, over the finished pose
pub(super) fn anim_feet_last(g: &mut crate::game::Game, c: CharId, dt: f64, s: f64, w_f: f64, own: f64) {
    let p0 = g.ch[c].pos;
    anim_layer(g, c, |slot, an, _| ::sim::layers::feet_last(slot, an, dt, s, w_f, own, p0));
}

/// deliberate movement's body and, if they are in, its arms
pub(super) fn anim_sneak_layer(g: &mut crate::game::Game, c: CharId, dl: f64, pace: f64, soft: f64, arms_in: f64, surge: f64, dsh: Option<f64>) {
    let dsh = dsh.unwrap_or(0.0);
    anim_sneak_body(g, c, dl, pace, soft, surge, dsh);
    if arms_in > 0.003 { anim_sneak_arms(g, c, arms_in); }
}
/// …its legs and trunk
fn anim_sneak_body(g: &mut crate::game::Game, c: CharId, dl: f64, pace: f64, soft: f64, surge: f64, dsh: f64) {
    anim_layer(g, c, |s, an, k| ::sim::layers::sneak_body(s, an, dl, pace, soft, surge, dsh, k));
}
/// …and its arms
fn anim_sneak_arms(g: &mut crate::game::Game, c: CharId, arms_in: f64) { anim_layer(g, c, |s, an, k| ::sim::layers::sneak_arms(s, an, arms_in, k)); }

/// MOVES.steady: the crouch walk's waddle taken out; the cycle's own mean is measured once off the clip
pub(super) fn anim_steady_layer(g: &mut crate::game::Game, c: CharId, w: f64, base: Option<f64>, sunk: Option<f64>) {
    let (base, sunk) = (base.unwrap_or(1.0), sunk.unwrap_or(0.0));
    if g.ch[c].anim.steady_mean.is_none() {
        let (thigh_l, thigh_r, hips) = { let nd = &g.ch[c].anim.nodes; (nd.thigh_l, nd.thigh_r, nd.hips) };
        let clip = clip_of(g, c, "Crouch_Fwd_Loop");
        let dur = g.tab.clip[clip].duration;
        let nn = 24.0;
        let (mut roll, mut x) = (0.0, 0.0);
        let mut i = 0.0;
        while i < nn {
            let (p, list, singles) = (g.ch[c].anim.p_upper, g.ch[c].anim.fk_legs.clone(), g.k.posemath.singles);
            crate::anim_skeleton::sample_clip(g, clip, i / nn * dur, true, p, singles);
            anim_fk_part(g, c, p, &list);
            let fk = &g.ch[c].anim.fk;
            let (a, b) = (fk.pos[thigh_l], fk.pos[thigh_r]);
            roll += m::atan2(a[1] - b[1], a[0] - b[0]);
            x += fk.pos[hips][0];
            i += 1.0;
        }
        g.ch[c].anim.steady_mean = Some([roll / nn, x / nn]);
    }
    let mean = g.ch[c].anim.steady_mean.unwrap();
    anim_layer(g, c, |s, an, k| ::sim::layers::steady_layer(s, an, w, base, sunk, mean, k));
}

/// both arms onto the pistol
pub(super) fn anim_carry_arms(g: &mut crate::game::Game, c: CharId, w: f64, kick_t: f64) { anim_layer(g, c, |s, an, k| ::sim::layers::carry_arms(s, an, w, kick_t, k)); }

// ---- the long gun

/// which row of LONG_RELOAD this gun's carry takes
fn anim_rl_row(g: &crate::game::Game, c: CharId) -> crate::character::LONGRELOADBox {
    let (back, has_feed) = (g.ch[c].anim.reload_back, g.ch[c].anim.feed.is_some());
    let r = crate::character::long_reload();
    if back && !has_feed {
        let b = r.back;
        crate::character::LONGRELOADBox { at: b.at, pitch: b.pitch, aim_k: b.aim_k, yaw: b.yaw }
    } else {
        r.box_
    }
}
/// the carriage's channels as they are posed: the springs', or those with a reload carry's share taken
fn anim_lg_channels(g: &crate::game::Game, c: CharId) -> [f64; ::sim::layers::LC_COUNT] {
    let a = &g.ch[c].anim;
    let cc = &a.rlc;
    match &cc.ch {
        Some(ch) if cc.k > 0.0 || cc.h > 0.0 => core::array::from_fn(|i| ch[i].x),
        _ => core::array::from_fn(|i| a.lg.ch[i].x),
    }
}
/// GUN HAND (`longReload`), once a frame after the carriage's springs: how much of the reload carry the long gun shows
pub(super) fn anim_reload_carry(g: &mut crate::game::Game, c: CharId, dt: f64, want: crate::character::LongStance) {
    let r = crate::character::long_reload();
    let a = &mut g.ch[c].anim;
    let (mut t, mut d) = (-1.0, 0.0);
    let (out, in_) = (a.rl_b.out, a.rl_b.in_);
    if a.long_gun {
        // (the hand's own clock, as mvArms picks it)
        if a.reload_at >= 0.0 && a.reload_row.is_some() { t = a.reload_at; d = m::max(0.3, a.reload_row.as_ref().unwrap().dur); }
        else if a.rl_t >= 0.0 { t = a.rl_t; d = a.rl_b.dur; }
    }
    a.rlc.still = if t >= 0.0 && t == a.rlc.at { a.rlc.still + dt } else { 0.0 };
    a.rlc.at = t;
    if a.rlc.still > 0.25 || a.sig_t >= 0.0 || a.tell_t >= 0.0 || a.mvs.ts_t >= 0.0 || a.throw_t >= 0.0 || a.strike_t >= 0.0 || a.big_tool.is_some() { t = -1.0; }
    let tuck = if want == LongStance::LowLong || want == LongStance::PatrolLong { v3::clamp(a.gun_wall, 0.0, 1.0) } else { 0.0 };
    let k_t = if t >= 0.0 { bump(t, 0.0, r.in_, d - r.end - r.out, d - r.end) * (1.0 - tuck) } else { 0.0 };
    let h_t = if t >= 0.0 { bump(t, m::max(0.0, out - r.head_in), m::max(0.05, out - 0.05), m::min(in_ + 0.1, d - r.end - r.head_out), m::min(in_ + 0.1 + r.head_out, d - r.end)) } else { 0.0 };
    let step = m::max(0.0, r.rate * dt);
    a.rlc.k += v3::clamp(k_t - a.rlc.k, -step, step);
    a.rlc.h += v3::clamp(h_t - a.rlc.h, -step, step);
    a.rl_shown.carry = a.rlc.k;
    a.rl_shown.head = a.rlc.h;
    if a.rlc.k > 0.0 || a.rlc.h > 0.0 {
        let l = crate::character::lc();
        let count = l.count as usize;
        let (lp, la, ly, lhy, lhp, lhr) = (l.pitch as usize, l.aim_k as usize, l.yaw as usize, l.head_yaw as usize, l.head_pitch as usize, l.head_roll as usize);
        if g.ch[c].anim.rlc.ch.is_none() {
            g.ch[c].anim.rlc.ch = Some((0..count).map(|_| AnimRlcChItem { x: 0.0 }).collect());
        }
        let row = anim_rl_row(g, c);
        let a = &mut g.ch[c].anim;
        let (ck, ch) = (a.rlc.k, a.rlc.h);
        let lg_ch = &a.lg.ch;
        let x = a.rlc.ch.as_mut().unwrap();
        for i in 0..count { x[i].x = lg_ch[i].x; }
        x[lp].x += (row.pitch - x[lp].x) * ck;
        x[la].x += (row.aim_k - x[la].x) * ck;
        x[ly].x += (row.yaw - x[ly].x) * ck;
        x[lhy].x += (r.head.yaw - x[lhy].x) * ch;
        x[lhp].x += (r.head.pitch - x[lhp].x) * ch;
        x[lhr].x += (r.head.roll - x[lhr].x) * ch;
    }
}
/// the long gun, trunk half
pub(super) fn anim_long_body(g: &mut crate::game::Game, c: CharId, w: f64, run: Option<f64>, sprint: Option<f64>) {
    let (run, sprint) = (run.unwrap_or(0.0), sprint.unwrap_or(0.0));
    if w < 0.003 { return; }
    let ch = anim_lg_channels(g, c);
    anim_layer(g, c, |s, an, k| ::sim::layers::long_body(s, an, w, run, sprint, &ch, k));
}
/// the long gun, arms
pub(super) fn anim_long_arms(g: &mut crate::game::Game, c: CharId, w: f64, kick_t: f64, dt: f64, yield_door: bool, keep: bool) {
    if w < 0.003 { return; }
    let (ch, at) = (anim_lg_channels(g, c), anim_rl_row(g, c).at);
    anim_layer(g, c, |s, an, k| ::sim::layers::long_arms(s, an, w, kick_t, dt, yield_door, keep, &ch, at, k));
}

// ---- the wall hold

/// the wall hold, body half
pub(super) fn anim_wall_body(g: &mut crate::game::Game, c: CharId, w: f64) {
    let (p0, idle, crouch_idle) = (g.ch[c].pos, clip_of(g, c, "Idle_Loop"), clip_of(g, c, "Crouch_Idle_Loop"));
    let (idle, crouch_idle) = (clip_at(g, c, idle, 0.0).clip, clip_at(g, c, crouch_idle, 0.0).clip);
    anim_layer(g, c, |s, an, k| ::sim::layers::wall_body(s, an, w, p0, idle, crouch_idle, k));
}

/// the wall hold, arms
pub(super) fn anim_wall_arms(g: &mut crate::game::Game, c: CharId, w: f64) { anim_layer(g, c, |s, an, k| ::sim::layers::wall_arms(s, an, w, k)); }

/// A layer over his animation: his rig on the bench, the layer, what it ran forward read back.
pub(super) fn anim_layer(g: &mut crate::game::Game, c: CharId, layer: impl FnOnce(usize, &mut Anim, &crate::knobs::Knobs) -> u32) {
    let slot = crate::anim_skeleton::layer_slot(g, c) as usize;
    let code = layer(slot, &mut g.ch[c].anim, &g.k);
    crate::anim_skeleton::layer_ran(g, code as f64, c);
}

// ---- the kick, signals, a smoke, the goggles, a tell

/// the kick, body half
pub(super) fn anim_kick_body(g: &mut crate::game::Game, c: CharId, t: f64, w: f64) { anim_layer(g, c, |s, an, k| ::sim::layers::kick_body(s, an, t, w, k)); }

/// left-hand signals
pub(super) fn anim_signal_layer(g: &mut crate::game::Game, c: CharId, t: f64) {
    let kind = g.ch[c].anim.sig_kind;
    let dur = signal_dur_of(kind);
    anim_layer(g, c, |s, an, k| ::sim::layers::signal_layer(s, an, t, kind, dur, k));
}

/// a smoke's arm, its clock
pub(super) fn anim_smoke_layer(g: &mut crate::game::Game, c: CharId, dt: f64, free: bool) {
    let k = crate::character::SMOKEARM;
    let a = &mut g.ch[c].anim;
    let want = match a.smoke { Some(s) if free => v3::clamp(s, 0.0, 1.0), _ => 0.0 };
    let step = (if free { k.rate } else { k.away }) * dt;
    a.smoke_arm += v3::clamp(want - a.smoke_arm, -step, step);
    if a.kit.is_some() { a.smoke_arm = 0.0; }
    if a.smoke_arm <= 0.004 { if want <= 0.0 { a.smoke_arm = 0.0; } return; }
    anim_smoke_pose(g, c);
}
/// the smoke's arm itself
fn anim_smoke_pose(g: &mut crate::game::Game, c: CharId) { anim_layer(g, c, |s, an, k| ::sim::layers::smoke_pose(s, an, k)); }

/// KIT.nv: one frame of a goggle flip
pub(super) fn anim_nv_layer(g: &mut crate::game::Game, c: CharId, dt: f64, free: bool) {
    let (h, o) = (g.k.kit.nv.hand, g.k.kit.nv.nod);
    let a = &mut g.ch[c].anim;
    let nv_nod = a.nv_nod;
    {
        let n = a.nv.as_mut().unwrap();
        if n.hand && !free && n.t < h.flip[0] { n.hand = false; n.t = 0.0; n.c0 = nv_nod; }
        n.t += dt;
    }
    let n = a.nv.unwrap();
    let dd = if n.hand { h.dur } else { o.dur };
    let fw = if n.hand { h.flip } else { o.flip };
    let shown = n.k0 + ((if n.down { 1.0 } else { 0.0 }) - n.k0) * v3::smoothstep(fw[0], fw[1], n.t);
    a.nv_shown = Some(shown);
    let want = if n.hand && free { bump(n.t, 0.0, h.reach, dd - h.out, dd) } else { 0.0 };
    let step = (if n.hand && free { h.rate } else if a.reload_t >= 0.0 || a.rl_t >= 0.0 || a.dp.w_l.x >= 0.05 { h.clear } else { h.away }) * dt;
    a.nv_arm += v3::clamp(want - a.nv_arm, -step, step);
    if a.kit.is_some() { a.nv_arm = 0.0; }
    if n.t >= dd && a.nv_arm < 0.004 { anim_nv_cancel(g, c); return; }
    if a.nv_arm > 0.004 { anim_nv_pose(g, c, shown); }
    let aa = if n.down { o.on } else { o.off };
    let nod = (if n.hand { 0.0 } else { (aa[0] * bump(n.t, 0.0, o.pre, o.pre, o.pre * 2.0) + aa[1] * bump(n.t, o.pre, o.peak, o.peak, dd)) * crate::character::DEGR }) + n.c0 * (1.0 - v3::smoothstep(0.0, o.carry, n.t));
    g.ch[c].anim.nv_nod = nod;
}
/// the goggle flip's arm itself
fn anim_nv_pose(g: &mut crate::game::Game, c: CharId, shown: f64) { anim_layer(g, c, |s, an, k| ::sim::layers::nv_pose(s, an, shown, k)); }
/// a goggle nod's tip of the HEAD joint on `pose` (+ = chin down)
pub(super) fn anim_nv_tip(g: &mut crate::game::Game, c: CharId, pose: crate::anim_skeleton::Pose, nod: f64, g_2: Option<Quat>) {
    let gq = match g_2 { Some(q) => q, None => anim_nv_head(g, c, pose) };
    let (hd, rest_inv) = (g.ch[c].anim.nodes.head, g.ch[c].anim.head_rest_inv);
    let z = v3::quat::rotate(v3::quat::mul(gq, rest_inv), [0.0, 0.0, 1.0]);
    let l = m::hyp2(z[0], z[2]);
    let ax: V3 = if l > 1e-4 { [z[2] / l, 0.0, -z[0] / l] } else { v3::quat::rotate(v3::quat::mul(gq, rest_inv), [1.0, 0.0, 0.0]) };
    let turned = v3::quat::normalize(v3::quat::mul(crate::anim_skeleton::pose_rot(g, pose, hd as f64), v3::quat::axis_angle(v3::quat::rotate(v3::quat::conj(gq), ax), nod)));
    crate::anim_skeleton::pose_set_rot(g, pose, hd as f64, turned);
}
/// a yaw and a pitch of the twists as one turn
fn twq(y: f64, p: f64) -> Quat {
    let mut q = v3::quat::yaw(y);
    if p != 0.0 { q = v3::quat::mul(v3::quat::axis_angle(v3::quat::rotate(q, [1.0, 0.0, 0.0]), p), q); }
    q
}
/// the head joint's rotation on `pose` as it will be DRAWN
fn anim_nv_head(g: &mut crate::game::Game, c: CharId, pose: crate::anim_skeleton::Pose) -> Quat {
    // (the fk stands out of g for the call: pose_fk_run takes both)
    let mut fk = core::mem::replace(&mut g.ch[c].anim.fk, crate::anim_skeleton::PoseFK::blank());
    crate::anim_skeleton::pose_fk_run(g, &mut fk, pose);
    g.ch[c].anim.fk = fk;
    let a = &g.ch[c].anim;
    let tw = a.nv_tw;
    v3::quat::mul(twq(tw[2], tw[3]), v3::quat::mul(twq(tw[0], tw[1]), a.fk.rot[a.nodes.head]))
}

/// the idle tells
pub(super) fn anim_tell_layer(g: &mut crate::game::Game, c: CharId, t: f64) {
    let kind = g.ch[c].anim.tell_k;
    let (watch, dur) = (kind == TellKind::Watch, tell_dur_of(kind));
    anim_layer(g, c, |s, an, k| ::sim::layers::tell_layer(s, an, t, watch, dur, k));
}
