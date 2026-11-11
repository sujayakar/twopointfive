use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};


// ---- what stands between a body and the skeleton's calls

/// `pose.rot(i)`, a node by the list's usize
fn rot(g: &Game, pose: crate::anim_skeleton::Pose, i: usize) -> Quat { crate::anim_skeleton::pose_rot(g, pose, i as f64) }
/// `pose.setRot(i, q)`, a node by the list's usize
fn set_rot(g: &mut Game, pose: crate::anim_skeleton::Pose, i: usize, q: Quat) { crate::anim_skeleton::pose_set_rot(g, pose, i as f64, q) }

// ---- the layers' columns: get / put, in the order the module's layer is handed them

// ---- the action clip

/// The whole-body action clip over the finished pA: clocks and weight here, the blend in the layer. The weight used (0 = untouched).
pub(super) fn anim_act_layer(g: &mut crate::game::Game, c: CharId, dt: f64) -> f64 {
    let a = g.ch[c].anim.act.clone();
    if let Some(a) = &a {
        if a.clip != g.ch[c].anim.act_name {
            let rig = g.ch[c].anim.rig;
            let clip = g.art.rig[rig].clips.get(a.clip.clone()).copied();
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

/// The action's clip over the pose at weight `w`, and the slump.
fn anim_act_pose(g: &mut crate::game::Game, c: CharId, clip: ClipId, w: f64, dt: f64) {
    let clip = crate::anim_skeleton::clip_no(g, clip, g.ch[c].anim.p_a.nodes as f64);
    anim_layer(g, c, |s, an, _| ::sim::layers::act_pose(s, an, clip, w, dt));
}

// ---- MOVES

/// Before the stances and the arms: the recoil's sums bled off and the pelvis's share of how a side carries itself. What the trunk layer needs.
pub(super) fn anim_mv_pre(g: &mut crate::game::Game, c: CharId, dt: f64, move_w: f64, stand_w: f64) -> crate::character::MvGait {
    anim_layer(g, c, |s, an, k| ::sim::layers::mv_pre(s, an, dt, move_w, stand_w, k));
    g.ch[c].anim.mv_out
}

/// A lean round an edge, eased.
pub(super) fn anim_lean_body(g: &mut crate::game::Game, c: CharId, dt: f64) { anim_layer(g, c, |s, an, _| ::sim::layers::lean_body(s, an, dt)); }

/// The gaze stabiliser's working out: the yaw the neck and head take out this frame.
pub(super) fn anim_gaze_keep(g: &mut crate::game::Game, c: CharId, dt: f64, want_w: f64, crouch_pose: f64, w_sprint: f64) -> f64 {
    let mut fix = 0.0;
    anim_layer(g, c, |s, an, _| ::sim::layers::gaze_keep(s, an, dt, want_w, crouch_pose, w_sprint, &mut fix));
    fix
}

/// The trunk's share, into pA right before the long gun's own trunk set.
pub(super) fn anim_mv_trunk(g: &mut crate::game::Game, c: CharId, w: f64, o: crate::character::MvGait, aim_pass: bool) {
    if w >= 0.003 { anim_layer(g, c, |s, an, k| ::sim::layers::mv_trunk(s, an, w, o, aim_pass, k)); }
}

/// MOVES, the second pass: the arms the fight does not read, after every other layer: the magazine change, the hand to his ribs, a figure kept down.
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
            && (pist || (long_gun && (reload_mag || has_feed)))
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
        let want = if reload_shown == 0.0 && !busy { v3::clamp(m::max(clutch_in, rest), 0.0, 1.0) } else { 0.0 };
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
        let an = &mut g.ch[c].anim;
        let bound = an.bound;
        let on = bound != 0.0 && !an.long_gun;
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

/// The trunk's own frame off the joints of `fk`: across the shoulders to his left, up the spine, out of the chest.
pub(super) fn anim_trunk_frame(g: &crate::game::Game, c: CharId, fk: &crate::anim_skeleton::PoseFK) -> crate::character::AnimTrunkFrame {
    let nd = &g.ch[c].anim.nodes;
    let up = v3::normalize(v3::sub(fk.pos[nd.spine3], fk.pos[nd.hips]));
    let mut left = v3::sub(fk.pos[nd.upper_arm_l], fk.pos[nd.upper_arm_r]);
    left = v3::normalize(v3::mad(left, up, -v3::dot(left, up)));
    AnimTrunkFrame { left, up, fwd: v3::cross(left, up) }
}

/// The long gun's magazine change on the support arm: the gun's own way of being fed, where it has one.
fn anim_reload_arm(g: &mut crate::game::Game, c: CharId, t: f64, w: f64) {
    use crate::gunreload::{FeedBeat, FeedMarks, FeedPath};
    let an = &g.ch[c].anim;
    let (feed, b, n, port, back) = (an.feed, an.rl_b, an.feed_n, an.feed_port, an.reload_back);
    let own = feed.and_then(|model| crate::gunreload::feed_path(model, FeedBeat { dur: b.dur, out: b.out, in_: b.in_, rack: b.rack, pull: b.pull }, n, port));
    let marks = |r: &Mv2ReloadBox| FeedMarks { mag: r.mag, strip: r.strip, handle: r.handle, pull: r.pull, hf: r.hf, hb: r.hb };
    let (marks, keys) = match own {
        Some(FeedPath::Arm1(p)) => (p.marks.unwrap_or(marks(&MV2.reload.box_)), None),
        Some(FeedPath::Arm0(p)) => (marks(if back { &MV2.reload.back } else { &MV2.reload.box_ }), Some(p.keys)),
        None => (marks(if back { &MV2.reload.back } else { &MV2.reload.box_ }), None),
    };
    if let Some(k) = &keys {
        crate::anim_skeleton::layer_fits("a reload's list of the hand's stations", k.len() as f64, crate::anim_skeleton::LAYER_KEYS_MAX, None);
    }
    anim_layer(g, c, |s, an, k| ::sim::layers::reload_arm(s, an, t, w, &marks, keys.as_deref(), k));
}

/// A pistol's magazine change, the support hand's.
fn anim_pistol_reload_arm(g: &mut crate::game::Game, c: CharId, t: f64, w: f64) { anim_layer(g, c, |s, an, k| ::sim::layers::pistol_reload_arm(s, an, t, w, k)); }

/// The support hand pressed to his ribs.
fn anim_clutch_arm(g: &mut crate::game::Game, c: CharId, w: f64) { anim_layer(g, c, |s, an, _| ::sim::layers::clutch_arm(s, an, w)); }

/// A figure kept down: wrists behind the back, or hands on the head.
fn anim_bound_arms(g: &mut crate::game::Game, c: CharId, w: f64, kind: f64) { anim_layer(g, c, |s, an, _| ::sim::layers::bound_arms(s, an, w, kind)); }

/// The reactions: where the dazzle is from in his own frame, then their body, then their arms.
pub(super) fn anim_mv_post(g: &mut crate::game::Game, c: CharId, dt: f64, gate: f64) {
    if let Some(dz) = g.ch[c].anim.dazzle {
        let (pos, was) = (g.ch[c].pos, g.ch[c].anim.mvs.dz_d);
        let d = anim_flat_model(g, c, v3::sub(dz.from, pos), was);
        g.ch[c].anim.mvs.dz_d = d;
    }
    anim_mv_post_body(g, c, dt, gate);
    anim_mv_post_arms(g, c, dt, gate);
}

/// The reactions' body, summed into one turn of the trunk, one of the head and one move of the pelvis.
fn anim_mv_post_body(g: &mut crate::game::Game, c: CharId, dt: f64, gate: f64) {
    anim_layer(g, c, |s, an, k| ::sim::layers::mv_post_body(s, an, dt, gate, k));
}

/// The reactions' arms, after their body: the start's, a blinded man's gun and hand, the toss.
fn anim_mv_post_arms(g: &mut crate::game::Game, c: CharId, dt: f64, gate: f64) {
    let an = &g.ch[c].anim;
    let (m, p) = (&an.mvs, &an.mv_po);
    if !(p.stw > 0.003) && !p.gun && !(p.dzw > 0.003) && !(m.ts_t >= 0.0) { return; }
    anim_layer(g, c, |s, an, k| ::sim::layers::mv_post_arms(s, an, dt, gate, k));
}

// ---- procedural layers

/// Curl the four fingers and set the thumb of one hand, from the rest hand; and the hand pass's note of how far this shape stands from the locomotion's.
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

/// MOVES, crouched: the kneeling foot's toe brought back onto the floor.
pub(super) fn anim_toes_up(g: &mut crate::game::Game, c: CharId, w: f64) { anim_layer(g, c, |s, an, _| ::sim::layers::toes_up(s, an, w)); }

/// The man's own small hash of `k` to −1‥1 (handsBuild's `h`): the die of how his hands hang, drawing nothing from the game's.
fn hand_hash(seed: f64, k: f64) -> f64 {
    let mut x = m::imul(js::to_i32((seed + 1.0) * 2654435761.0) ^ js::to_i32(k * 40503.0), js::to_i32(2246822519.0)) as u32;
    x ^= x >> 15;
    x = m::imul(x as i32, js::to_i32(3266489917.0)) as u32;
    x ^= x >> 13;
    x as f64 / 4294967296.0 * 2.0 - 1.0
}

/// This man's relaxed hands, as the turn from the library's fist on each digit joint, and the index laid straight for the trigger hand.
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

/// The hand pass: digit joints of pA only.
pub(super) fn anim_hands_pass(g: &mut crate::game::Game, c: CharId, dt: f64, full_l: f64, full_r: f64, gun_r: f64) {
    if g.ch[c].anim.hn.built != g.ch[c].anim.hn.seed { anim_hands_build(g, c); }
    hand_tables_now(&mut g.ch[c].anim);
    anim_layer(g, c, |s, an, _| ::sim::layers::hands_pass(s, an, dt, [full_l, full_r], gun_r));
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

/// Stance, body half.
pub(super) fn anim_stance_body(g: &mut crate::game::Game, c: CharId, w: f64, speed: f64, stand: f64) { anim_layer(g, c, |s, an, _| ::sim::layers::stance_body(s, an, w, speed, stand)); }

/// The stand pose: the idle and over it the cycle.
pub(super) fn anim_stand_pose(g: &mut crate::game::Game, c: CharId, ph: f64, w_idle: f64, w_walk: f64, w_jog: f64, w_sprint: f64) {
    let (idle, idle_t) = (clip_of(g, c, "Idle_Loop"), g.ch[c].anim.idle_t);
    let idle = clip_at(g, c, idle, idle_t);
    let mut at = |name: &'static str, of: f64| { let clip = clip_of(g, c, name); let t = of * g.art.clip[clip].duration; clip_at(g, c, clip, t) };
    let (jog, sprint, walk) = (at("Jog_Fwd_Loop", ph), at("Sprint_Loop", ph), at("Walk_Loop", ph + WALK_PHASE));
    let cycle = ::sim::layers::Cycle { ph, w_idle, w_walk, w_jog, w_sprint, idle, jog, sprint, walk };
    anim_layer(g, c, |s, an, _| ::sim::layers::stand_pose(s, an, &cycle));
}

/// The crouch laid over the stand pose.
pub(super) fn anim_crouch_laid(g: &mut crate::game::Game, c: CharId, ph: f64, w_move: f64, crouch_pose: f64) { anim_crouch_laid_ask(g, c, ph, w_move, crouch_pose); }

#[cfg(test)]
mod tests {
    use super::*;

    /// How a man's hands hang is his own: a number of his from -1 to 1, the same each time he is made, and not the next man's.
    #[test]
    fn a_mans_hands_are_his_own() {
        let (mut low, mut high) = (0, 0);
        for seed in 0..200 {
            let (seed, k) = (seed as f64, 40.0);
            let h = hand_hash(seed, k);
            assert!((-1.0..1.0).contains(&h), "seed {seed}: {h}");
            assert_eq!(h, hand_hash(seed, k));
            assert_ne!(h, hand_hash(seed + 1.0, k), "seed {seed}: the next man has the same hands");
            assert_ne!(h, hand_hash(seed, k + 1.0), "seed {seed}: two fingers of a man move as one");
            if h < 0.0 { low += 1; } else { high += 1; }
        }
        assert!(low > 60 && high > 60, "men's hands fall to both sides: {low} and {high}");
    }
}
