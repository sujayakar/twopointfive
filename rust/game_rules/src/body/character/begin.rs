use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// the rig's named nodes as `Object.entries(N)` walks them: the key a layer asks for (the TypeScript's own name), the node's name
fn n_entries(nn: &crate::character::N) -> [(&'static str, &str); 24] {
    [
        ("hips", &nn.hips), ("spine1", &nn.spine1), ("spine2", &nn.spine2), ("spine3", &nn.spine3), ("neck", &nn.neck), ("head", &nn.head),
        ("upperArmL", &nn.upper_arm_l), ("forearmL", &nn.forearm_l), ("handL", &nn.hand_l), ("upperArmR", &nn.upper_arm_r), ("forearmR", &nn.forearm_r), ("handR", &nn.hand_r),
        ("thighL", &nn.thigh_l), ("shinL", &nn.shin_l), ("footL", &nn.foot_l), ("toeL", &nn.toe_l), ("thighR", &nn.thigh_r), ("shinR", &nn.shin_r), ("footR", &nn.foot_r), ("toeR", &nn.toe_r),
        ("shoulderL", &nn.shoulder_l), ("shoulderR", &nn.shoulder_r), ("middle1R", &nn.middle1_r), ("middle1L", &nn.middle1_l),
    ]
}

// ---- what the constructor does before the animator is on a body: anim_fk_list, anim_stance_of and anim_sample_jog take a CharId (notes, ## Skeleton)

/// the nodes fkPart must compute for `ends`: each and all its ancestors, parents first
fn fk_list(rig: &crate::anim_gltf::GltfCharacter, ends: &[f64]) -> Vec<i32> {
    let mut want = vec![false; rig.nodes.len()];
    for &e in ends {
        let mut i = e;
        while i >= 0.0 {
            want[i as usize] = true;
            i = rig.nodes[i as usize].parent;
        }
    }
    rig.order.iter().filter(|&&i| want[i as usize]).map(|&i| i as i32).collect()
}
/// a run clip's stances from its toes; None if either foot never qualifies   (character.ts stanceOf: it reads no state)
fn stance_of(ty: &[Vec<f64>], tz: &[Vec<f64>]) -> Option<crate::character::AnimStanceOf> {
    let mm = ty[0].len();
    let mut cs: Vec<f64> = Vec::with_capacity(ty.len());
    for k in 0..ty.len() {
        let ys = &ty[k];
        let lo = m::min_n(ys) + 0.02;
        let (mut sx, mut cx, mut n) = (0.0, 0.0, 0);
        for i in 0..mm {
            if ys[i] <= lo && tz[k][(i + 1) % mm] < tz[k][(i + mm - 1) % mm] {
                sx += m::sin(2.0 * m::PI * i as f64 / mm as f64);
                cx += m::cos(2.0 * m::PI * i as f64 / mm as f64);
                n += 1;
            }
        }
        cs.push(if n != 0 { js::rem(m::atan2(sx, cx) / (2.0 * m::PI) + 1.0, 1.0) } else { f64::NAN });
    }
    if !cs[0].is_finite() || !cs[1].is_finite() { return None; }
    let c = js::rem(m::atan2(m::sin(4.0 * m::PI * cs[0]) + m::sin(4.0 * m::PI * cs[1]), m::cos(4.0 * m::PI * cs[0]) + m::cos(4.0 * m::PI * cs[1])) / (4.0 * m::PI) + 1.0, 0.5);
    Some(crate::character::AnimStanceOf { c, c_l: cs[0], c_r: cs[1] })
}
/// Jog_Fwd_Loop at phase ph with its loop seam closed, on the animator being built
fn sample_jog(g: &mut crate::game::Game, a: &crate::character::Anim, ph: f64, out: crate::anim_skeleton::Pose) {
    let cj = kept(a, "Jog_Fwd_Loop");
    let t = ph * g.art.clip[cj].duration;
    crate::anim_skeleton::sample_clip(g, cj, t, true, out);
    let k = ph - m::floor(ph) - 0.5;
    let rows = crate::anim_skeleton::pose_rows(out.nodes as f64) as usize;
    jog_seam_close(crate::anim_skeleton::pose_cols(g, out), rows, &a.jog_seam, k);
}
/// every channel whose last key misses its first carries k × the gap: the sums in doubles, each stored as a single once
fn jog_seam_close(d: &mut [f32], rows: usize, seam: &[crate::character::AnimJogSeamItem], k: f64) {
    for s in seam {
        if let Some(v) = s.d {
            let o = s.node as usize;
            d[o] = (d[o] as f64 + v[0] * k) as f32;
            d[o + rows] = (d[o + rows] as f64 + v[1] * k) as f32;
            d[o + 2 * rows] = (d[o + 2 * rows] as f64 + v[2] * k) as f32;
        } else {
            let o = 3 * rows + s.node as usize;
            let q = v3::quat::mul([d[o] as f64, d[o + rows] as f64, d[o + 2 * rows] as f64, d[o + 3 * rows] as f64], v3::quat::axis_angle(s.axis.unwrap(), s.ang.unwrap() * k));
            d[o] = q[0] as f32;
            d[o + rows] = q[1] as f32;
            d[o + 2 * rows] = q[2] as f32;
            d[o + 3 * rows] = q[3] as f32;
        }
    }
}

// ---- helpers

/// `ch.nodeByName.get(name)`, loud when missing (the constructor's `node`)
fn node(g: &crate::game::Game, ch: RigId, name: &str) -> usize {
    match g.art.rig[ch].node_by_name.of(name) {
        Some(i) => *i as usize,
        None => panic!("missing node {}", name),
    }
}
/// one row of a mask's list
fn entry(name: &str, w: f64, subtree: bool) -> crate::anim_skeleton::BuildMaskEntriesItem {
    crate::anim_skeleton::BuildMaskEntriesItem { name: String::from(name), w, subtree }
}
/// a clip the constructor has kept by name
fn kept(a: &crate::character::Anim, k: &'static str) -> ClipId {
    *a.clips.get(k).unwrap()
}
/// the fist-taking `grab` of the constructor: each hand's 15 digit joints' rotations off a pose
fn grab(g: &crate::game::Game, hands: &[Vec<i32>], q: crate::anim_skeleton::Pose) -> Vec<Vec<f32>> {
    hands.iter().map(|ns| {
        let mut a = vec![0.0f32; 60];
        for (k, i) in ns.iter().enumerate() {
            let r = crate::anim_skeleton::pose_rot(g, q, *i as f64);
            for j in 0..4 { a[k * 4 + j] = r[j] as f32; }
        }
        a
    }).collect()
}
/// `h(k)` of desync: the figure's k-th hash, 0‥1
fn desync_hash(seed: f64, k: f64) -> f64 {
    let a = js::to_i32((seed + 1.0) * 2654435761.0) ^ js::to_i32(k * 40503.0);
    let mut x = m::imul(a, js::to_i32(2246822519.0)) as u32;
    x ^= x >> 15;
    x = m::imul(x as i32, js::to_i32(3266489917.0)) as u32;
    x ^= x >> 13;
    x as f64 / 4294967296.0
}
/// the figure's own round count, hashed (fire)
fn round_hash(n: f64) -> f64 {
    js::to_u32(n * 2654435761.0) as f64 / 4294967296.0
}
/// the arithmetic of rlBeat; `b.rack`: outer None = undefined, Some(None) = null, a round still chambered
fn rl_beat_keys(k: &mut crate::character::AnimRlB, b: &crate::character::ReloadBeat) {
    let d = m::max(0.3, b.dur);
    let rack = b.rack;
    let rack_n = rack.flatten();
    if b.exact == Some(true) && b.out.is_some() && b.in_.is_some() {
        k.out = b.out.unwrap();
        k.in_ = b.in_.unwrap();
        k.pull = rack_n.is_some();
        k.rack = rack_n.unwrap_or(d);
        k.dur = d;
        k.hand = false;
        k.keep = -1.0;
        k.fresh = (k.out + k.in_) * 0.5;
        return;
    }
    k.out = m::max(0.15, match b.out { Some(x) => x, None => d * 0.27 });
    k.in_ = m::max(k.out + 0.6, match b.in_ { Some(x) => x, None => d * 0.65 });
    k.pull = rack != Some(None);
    k.rack = m::max(k.in_ + 0.2, match rack_n { Some(x) => x, None => d * 0.85 });
    k.dur = m::max(d, if k.pull { k.rack + 0.25 } else { k.in_ + 0.5 });
    k.hand = b.hand == Some(true);
    k.keep = match b.keep { Some(x) => v3::clamp(x, k.out + 0.25, k.in_ - 0.4), None => -1.0 };
    k.fresh = if k.keep >= 0.0 { k.keep + (k.in_ - k.keep) * 0.3 } else if k.hand { k.out + (k.in_ - k.out) * 0.4 } else { (k.out + k.in_) * 0.5 };
}
/// `sec` of kitSwap: a rifle on a sling in front has its own times
fn kit_sec(front: bool, it: crate::character::KitItem, table: &JsMap<crate::character::KitItem, f64>, f: f64) -> f64 {
    if front && it == crate::character::KitItem::Rifle { f } else { *table.get(it).unwrap() }
}
/// `loc` of kitSwap: a point off its shoulder, in the trunk's frame
fn loc_in(f: &crate::character::AnimTrunkFrame, a: V3, o: V3) -> V3 {
    let d = v3::sub(a, o);
    [v3::dot(d, f.left), v3::dot(d, f.up), v3::dot(d, f.fwd)]
}
/// the throw's read of its distance (throwItem): (thrLow, thrFar)
fn throw_by_read(lo: f64, full: f64, toss_fast: f64, dist: f64, speed: Option<f64>) -> (f64, f64) {
    let hi = m::max(full, lo + 0.01);
    if dist < lo && !(speed.is_some() && speed.unwrap() >= toss_fast) {
        (v3::clamp(dist / lo, 0.0, 1.0), 1.0)
    } else {
        (-1.0, v3::smoothstep(lo, hi, dist))
    }
}

// ---- the part

/// the pose under every look-only arm layer that showed this frame; none when none showed
pub(super) fn anim_under_pose(g: &mut crate::game::Game, c: CharId) -> Option<crate::anim_skeleton::Pose> {
    let (a, h) = match (g.ch[c].anim.arms_pre, g.ch[c].anim.hand_pre) {
        (Some(a), Some(h)) => (a, h),
        (a, h) => return a.or(h),
    };
    let buf = match g.ch[c].anim.under_buf {
        Some(b) => b,
        None => {
            let id = g.ch[c].anim.poses;
            let b = crate::anim_skeleton::pose_set_make(g, id, a.nodes as f64);
            g.ch[c].anim.under_buf = Some(b);
            b
        }
    };
    // (lent out of the row for the call, which takes g: blend_pose reads the pool, not the animator)
    let mask = core::mem::take(&mut g.ch[c].anim.arm_r_only);
    crate::anim_skeleton::blend_pose(g, Some(a), h, 1.0, Some(mask.as_slice()), buf);
    g.ch[c].anim.arm_r_only = mask;
    Some(buf)
}

/// the animator of a rig: its poses, clips, masks, and everything measured off the clips (on p_b and p_c, which go back to rest: a pose's place in the pool is for
/// good); then his phase off the game's die
pub(super) fn anim_new(g: &mut crate::game::Game, ch: RigId) -> crate::character::Anim {
    let mut a = crate::character::Anim::blank();
    a.rig = ch;
    its_poses(g, ch, &mut a);
    its_clips(g, ch, &mut a);
    its_nodes(g, ch, &mut a);
    its_masks(g, ch, &mut a);
    its_grip(g, ch, &mut a);
    its_idle(g, ch, &mut a);
    crouch_gait(g, ch, &mut a);
    jog_seam(g, ch, &mut a);
    its_cast(g, ch, &mut a);
    jog_gait(g, ch, &mut a);
    sprint_gait(g, ch, &mut a);
    let (s1, s2, rest) = (a.p_b, a.p_c, a.rest);
    crate::anim_skeleton::pose_copy(g, s1, rest);
    crate::anim_skeleton::pose_copy(g, s2, rest);
    a.phase = crate::dice::draw_world(g, "character::anim_new#1");
    a
}

/// Its poses, all at rest.
fn its_poses(g: &mut Game, ch: RigId, a: &mut Anim) {
    a.poses = crate::anim_skeleton::pose_set_new(g, crate::character::ANIM_POSES);
    let n = g.art.rig[ch].nodes.len() as f64;
    let set = a.poses;
    a.pose = crate::anim_skeleton::pose_set_make(g, set, n);
    a.p_a = crate::anim_skeleton::pose_set_make(g, set, n);
    a.p_b = crate::anim_skeleton::pose_set_make(g, set, n);
    a.p_c = crate::anim_skeleton::pose_set_make(g, set, n);
    a.p_upper = crate::anim_skeleton::pose_set_make(g, set, n);
    a.p_aim = crate::anim_skeleton::pose_set_make(g, set, n);
    a.rest = crate::anim_skeleton::pose_set_make(g, set, n);
    crate::anim_skeleton::rest_pose(g, ch, a.rest);
    let rest = a.rest;
    for p in [a.pose, a.p_a, a.p_b, a.p_c, a.p_upper, a.p_aim] { crate::anim_skeleton::pose_copy(g, p, rest); }
}

/// The clips it cannot do without.
fn its_clips(g: &mut Game, ch: RigId, a: &mut Anim) {
    let need: [&'static str; 15] = ["Idle_Loop", "Walk_Loop", "Jog_Fwd_Loop", "Crouch_Idle_Loop", "Crouch_Fwd_Loop", "Pistol_Idle_Loop", "Pistol_Aim_Neutral", "Pistol_Aim_Up", "Pistol_Aim_Down", "Pistol_Shoot", "Idle_Torch_Loop", "Hit_Chest", "Sprint_Loop", "Pistol_Reload", "Punch_Cross"];
    for k in need {
        match g.art.rig[ch].clips.of(k) {
            Some(cl) => { a.clips.set(k, *cl); }
            None => panic!("missing clip {}", k),
        }
    }
    if let Some(j) = g.art.rig[ch].clips.of("Punch_Jab") { a.clips.set("Punch_Jab", *j); }
}

/// The rig's named nodes, and each digit's joints.
fn its_nodes(g: &mut Game, ch: RigId, a: &mut Anim) {
    let nn = crate::character::n();
    let names = n_entries(&nn);
    let at: Vec<usize> = names.iter().map(|e| node(g, ch, e.1)).collect();
    a.nodes = crate::character::AnimNodes {
        hips: at[0], spine1: at[1], spine2: at[2], spine3: at[3], neck: at[4], head: at[5],
        upper_arm_l: at[6], forearm_l: at[7], hand_l: at[8], upper_arm_r: at[9], forearm_r: at[10], hand_r: at[11],
        thigh_l: at[12], shin_l: at[13], foot_l: at[14], toe_l: at[15], thigh_r: at[16], shin_r: at[17], foot_r: at[18], toe_r: at[19],
        middle1_r: at[22], middle1_l: at[23],
    };
    for s in ["L", "R"] {
        let mut hand = Vec::new();
        for f in ["f_index", "f_middle", "f_ring", "f_pinky"] {
            let mut joints = Vec::new();
            for j in [1, 2, 3] { joints.push(node(g, ch, &alloc::format!("DEF-{}.0{}.{}", f, j, s))); }
            hand.push(joints);
        }
        a.fingers.push(hand);
    }
    for s in ["L", "R"] {
        let mut joints = Vec::new();
        for j in [1, 2, 3] { joints.push(node(g, ch, &alloc::format!("DEF-thumb.0{}.{}", j, s))); }
        a.thumbs.push(joints);
    }
    a.hn.nodes = [0usize, 1].iter().map(|&s| {
        let mut l: Vec<i32> = a.thumbs[s].iter().map(|&i| i as i32).collect();
        for f in &a.fingers[s] { for &i in f { l.push(i as i32); } }
        l
    }).collect();
}

/// What of a body each kind of layer has.
fn its_masks(g: &mut Game, ch: RigId, a: &mut Anim) {
    let nn = crate::character::n();
    a.upper_mask = crate::anim_skeleton::build_mask(g, ch, &[entry(&nn.spine1, 0.3, false), entry(&nn.spine2, 0.65, false), entry(&nn.spine3, 1.0, true)]);
    a.arm_r_mask = crate::anim_skeleton::build_mask(g, ch, &[entry(&nn.shoulder_r, 1.0, true), entry(&nn.spine3, 0.25, false)]);
    a.head_mask = crate::anim_skeleton::build_mask(g, ch, &[entry(&nn.neck, 1.0, true)]);
    a.arms_mask = crate::anim_skeleton::build_mask(g, ch, &[entry(&nn.shoulder_l, 1.0, true), entry(&nn.shoulder_r, 1.0, true)]);
    a.arm_r_only = crate::anim_skeleton::build_mask(g, ch, &[entry(&nn.shoulder_r, 1.0, true)]);
    a.torso_mask = a.upper_mask.iter().enumerate().map(|(i, w)| if a.arms_mask[i] > 0.0 { 0.0 } else { *w }).collect();
}

/// A chest, a head and a hand as they are at rest, and the hand's up as it aims.
fn its_grip(g: &mut Game, ch: RigId, a: &mut Anim) {
    let (rest, nd) = (a.rest, a.nodes);
    let mut fk = crate::anim_skeleton::pose_fk_new(g, ch);
    a.fk_legs = fk_list(&g.art.rig[ch], &[nd.hips as f64, nd.thigh_l as f64, nd.shin_l as f64, nd.foot_l as f64, nd.toe_l as f64, nd.thigh_r as f64, nd.shin_r as f64, nd.foot_r as f64, nd.toe_r as f64]);
    a.fk_arms = fk_list(&g.art.rig[ch], &[nd.spine3 as f64, nd.upper_arm_r as f64, nd.forearm_r as f64, nd.hand_r as f64, nd.middle1_r as f64, nd.upper_arm_l as f64, nd.forearm_l as f64, nd.hand_l as f64]);
    a.fk_head = fk_list(&g.art.rig[ch], &[nd.head as f64]);

    crate::anim_skeleton::sample_clip(g, kept(a, "Pistol_Aim_Neutral"), 0.0, false, a.p_aim);
    crate::anim_skeleton::pose_fk_run(g, &mut fk, rest);
    a.chest_rest_inv = v3::quat::conj(fk.rot[nd.spine3]);
    a.head_rest_inv = v3::quat::conj(fk.rot[nd.head]);
    a.hand_fix = [nd.hand_l, nd.hand_r].iter().map(|&h| {
        let r = fk.rot[h];
        let f = v3::normalize(v3::quat::rotate(r, ay()));
        let b = v3::normalize(v3::mad(ay(), f, -v3::dot(ay(), f)));
        v3::quat::mul(v3::quat::conj(v3::quat::from_basis(v3::cross(f, b), f, b)), r)
    }).collect();
    crate::anim_skeleton::pose_fk_run(g, &mut fk, a.p_aim);
    a.grip_up = v3::quat::rotate(v3::quat::conj(fk.rot[nd.hand_r]), ay());
}

/// The standing idle: the gaze, the feet, and the fist the hand pass measures from.
fn its_idle(g: &mut Game, ch: RigId, a: &mut Anim) {
    let mut fk = crate::anim_skeleton::pose_fk_new(g, ch);
    let (s1, s2, rest, nd) = (a.p_b, a.p_c, a.rest, a.nodes);
    let p = s1;
    crate::anim_skeleton::pose_copy(g, p, rest);
    crate::anim_skeleton::sample_clip(g, kept(a, "Idle_Loop"), 0.0, false, p);
    crate::anim_skeleton::pose_fk_run(g, &mut fk, p);
    a.gaze_local = v3::quat::rotate(v3::quat::conj(fk.rot[nd.head]), az());
    a.hn.fist = grab(g, &a.hn.nodes, p);
    let pc = s2;
    crate::anim_skeleton::pose_copy(g, pc, rest);
    crate::anim_skeleton::sample_clip(g, kept(a, "Crouch_Idle_Loop"), 0.0, false, pc);
    let cr = grab(g, &a.hn.nodes, pc);
    a.hn.own = [0usize, 1].iter().map(|&sd| {
        let (cd, fd) = (&cr[sd], &a.hn.fist[sd]);
        [0.0f64, 1.0, 2.0, 3.0, 4.0].iter().map(|&d| {
            m::max(10.0 * DEGR, (q_ang(cd, d * 12.0, 1.0, fd, d * 12.0) + q_ang(cd, d * 12.0 + 4.0, 1.0, fd, d * 12.0 + 4.0) + q_ang(cd, d * 12.0 + 8.0, 1.0, fd, d * 12.0 + 8.0)) / 3.0) as f32
        }).collect()
    }).collect();
    a.idle_feet = crate::character::AnimIdleFeet { l: fk.pos[nd.foot_l], r: fk.pos[nd.foot_r], r_l: fk.rot[nd.foot_l], r_r: fk.rot[nd.foot_r] };
}

/// The crouch walk's gait for the deliberate retime: the ankles' fore-aft middle, and where the toes come down.
fn crouch_gait(g: &mut Game, ch: RigId, a: &mut Anim) {
    let mut fk = crate::anim_skeleton::pose_fk_new(g, ch);
    let (s1, rest, nd) = (a.p_b, a.rest, a.nodes);
    let p = s1;
    crate::anim_skeleton::pose_copy(g, p, rest);
    let cf = kept(a, "Crouch_Fwd_Loop");
    let cf_dur = g.art.clip[cf].duration;
    const M: usize = 48;
    let (mut zmin, mut zmax) = (1e9f64, -1e9f64);
    let mut ty: [Vec<f64>; 2] = [Vec::new(), Vec::new()];
    for i in 0..M {
        crate::anim_skeleton::sample_clip(g, cf, cf_dur * i as f64 / M as f64, true, p);
        crate::anim_skeleton::pose_fk_run(g, &mut fk, p);
        for f in [nd.foot_l, nd.foot_r] { zmin = m::min(zmin, fk.pos[f][2]); zmax = m::max(zmax, fk.pos[f][2]); }
        ty[0].push(fk.pos[nd.toe_l][1]);
        ty[1].push(fk.pos[nd.toe_r][1]);
    }
    let mut plants: Vec<f64> = Vec::new();
    for (side, ys) in ty.iter().enumerate() {
        let lo = m::min_n(ys) + 0.01;
        for i in 0..M {
            if ys[(i + M - 1) % M] > lo && ys[i] <= lo { plants.push(i as f64 / M as f64 - 0.5 * side as f64); break; }
        }
    }
    if zmax > zmin { a.gait.mid = (zmin + zmax) / 2.0; }
    if !plants.is_empty() {
        let (mut sx, mut cy) = (0.0, 0.0);
        for &ph in &plants { sx += m::sin(4.0 * m::PI * ph); cy += m::cos(4.0 * m::PI * ph); }
        a.gait.linger = js::rem((m::atan2(sx, cy) / (4.0 * m::PI)) + 0.07 + 1.0, 0.5);
    }
}

/// The run clip's loop seam: each keyed channel's gap, first less last, spread round the cycle at sample time (sampleJog).
fn jog_seam(g: &mut Game, ch: RigId, a: &mut Anim) {
    let (s1, s2, rest) = (a.p_b, a.p_c, a.rest);
    let cj = kept(a, "Jog_Fwd_Loop");
    let cj_dur = g.art.clip[cj].duration;
    let (pa, pb) = (s1, s2);
    crate::anim_skeleton::pose_copy(g, pa, rest);
    crate::anim_skeleton::pose_copy(g, pb, rest);
    crate::anim_skeleton::sample_clip(g, cj, 0.0, false, pa);
    crate::anim_skeleton::sample_clip(g, cj, cj_dur, false, pb);
    let tracks: Vec<(f64, f64)> = g.art.clip[cj].tracks.iter().map(|tr| (tr.node, tr.path)).collect();
    for (node, path) in tracks {
        if path == 0.0 {
            let d = v3::sub(crate::anim_skeleton::pose_pos(g, pa, node), crate::anim_skeleton::pose_pos(g, pb, node));
            if v3::len(d) > 5e-4 { a.jog_seam.push(crate::character::AnimJogSeamItem { node, d: Some(d), axis: None, ang: None }); }
        } else if path == 1.0 {
            let mut q = v3::quat::mul(v3::quat::conj(crate::anim_skeleton::pose_rot(g, pb, node)), crate::anim_skeleton::pose_rot(g, pa, node));
            if q[3] < 0.0 { q = [-q[0], -q[1], -q[2], -q[3]]; }
            let s = m::hyp3(q[0], q[1], q[2]);
            let ang = 2.0 * m::atan2(s, q[3]);
            if ang > 0.3 * DEGR { a.jog_seam.push(crate::character::AnimJogSeamItem { node, d: None, axis: Some([q[0] / s, q[1] / s, q[2] / s]), ang: Some(ang) }); }
        }
    }
}

/// What a layer of the sim is handed with the rig.
fn its_cast(g: &mut Game, ch: RigId, a: &mut Anim) {
    let nn = crate::character::n();
    let names = n_entries(&nn);
    let at: Vec<usize> = names.iter().map(|e| node(g, ch, e.1)).collect();
    layer_tables_fit();
    let all: Vec<ClipId> = g.art.rig[ch].clips.values().copied().collect();
    crate::anim_skeleton::layer_clips_fit(g, &all, a.p_a.nodes as f64);

    let mut cast = crate::anim_skeleton::LayerCast::blank();
    for (i, e) in names.iter().enumerate() { cast.nodes.set(e.0, at[i]); }
    cast.lists.set("fkLegs", a.fk_legs.clone());
    cast.lists.set("fkArms", a.fk_arms.clone());
    cast.lists.set("fkHead", a.fk_head.clone());
    cast.lists.set("digitsL", a.hn.nodes[0].clone());
    cast.lists.set("digitsR", a.hn.nodes[1].clone());
    let mut seam_nums: Vec<f64> = Vec::new();
    for e in &a.jog_seam {
        if let Some(d) = e.d {
            seam_nums.extend_from_slice(&[e.node, 0.0, d[0], d[1], d[2], 0.0]);
        } else {
            let ax = e.axis.unwrap();
            seam_nums.extend_from_slice(&[e.node, 1.0, ax[0], ax[1], ax[2], e.ang.unwrap()]);
        }
    }
    cast.nums.set("jogSeam", seam_nums);
    a.cast = cast;
}

/// The run clip's pelvis height by phase and its mean, its ankles' middle, its stances.
fn jog_gait(g: &mut Game, ch: RigId, a: &mut Anim) {
    let mut fk = crate::anim_skeleton::pose_fk_new(g, ch);
    let (s1, rest, nd) = (a.p_b, a.rest, a.nodes);
    let p = s1;
    crate::anim_skeleton::pose_copy(g, p, rest);
    const M: usize = 64;
    let mut sum = 0.0;
    let (mut zmin, mut zmax) = (1e9f64, -1e9f64);
    let mut ty: [Vec<f64>; 2] = [Vec::new(), Vec::new()];
    let mut tz: [Vec<f64>; 2] = [Vec::new(), Vec::new()];
    for i in 0..M {
        sample_jog(g, a, i as f64 / M as f64, p);
        crate::anim_skeleton::pose_fk_run(g, &mut fk, p);
        a.jog_hips_y[i] = fk.pos[nd.hips][1] as f32;
        sum += a.jog_hips_y[i] as f64;   // (the table's single, read back)
        for f in [nd.foot_l, nd.foot_r] { zmin = m::min(zmin, fk.pos[f][2]); zmax = m::max(zmax, fk.pos[f][2]); }
        ty[0].push(fk.pos[nd.toe_l][1]);
        ty[1].push(fk.pos[nd.toe_r][1]);
        tz[0].push(fk.pos[nd.toe_l][2]);
        tz[1].push(fk.pos[nd.toe_r][2]);
    }
    a.jog_hips_mean = sum / M as f64;
    if zmax > zmin { a.jog_mid = (zmin + zmax) / 2.0; }
    if let Some(s) = stance_of(&ty, &tz) { a.run_gait.jog = crate::character::AnimRunGaitJog { c: s.c, c_l: s.c_l, c_r: s.c_r }; }
}

/// The sprint clip likewise: its ankles' middle, their lowest height, its stances.
fn sprint_gait(g: &mut Game, ch: RigId, a: &mut Anim) {
    let mut fk = crate::anim_skeleton::pose_fk_new(g, ch);
    let (s1, rest, nd) = (a.p_b, a.rest, a.nodes);
    let p = s1;
    crate::anim_skeleton::pose_copy(g, p, rest);
    let cs = kept(a, "Sprint_Loop");
    let cs_dur = g.art.clip[cs].duration;
    const M: usize = 64;
    let (mut zmin, mut zmax, mut ymin) = (1e9f64, -1e9f64, 1e9f64);
    let mut ty: [Vec<f64>; 2] = [Vec::new(), Vec::new()];
    let mut tz: [Vec<f64>; 2] = [Vec::new(), Vec::new()];
    for i in 0..M {
        crate::anim_skeleton::sample_clip(g, cs, cs_dur * i as f64 / M as f64, true, p);
        crate::anim_skeleton::pose_fk_run(g, &mut fk, p);
        for f in [nd.foot_l, nd.foot_r] { zmin = m::min(zmin, fk.pos[f][2]); zmax = m::max(zmax, fk.pos[f][2]); ymin = m::min(ymin, fk.pos[f][1]); }
        ty[0].push(fk.pos[nd.toe_l][1]);
        ty[1].push(fk.pos[nd.toe_r][1]);
        tz[0].push(fk.pos[nd.toe_l][2]);
        tz[1].push(fk.pos[nd.toe_r][2]);
    }
    if zmax > zmin { a.sprint_mid = (zmin + zmax) / 2.0; }
    if ymin < 1e8 { a.sprint_floor_y = ymin; }
    if let Some(s) = stance_of(&ty, &tz) { a.run_gait.sprint = crate::character::AnimRunGaitSprint { c: s.c, c_l: s.c_l, c_r: s.c_r }; }
}

/// the idle and life clocks set off per figure, deterministically, so a room does not breathe in step
pub(super) fn anim_desync(g: &mut crate::game::Game, c: CharId, seed: f64) {
    let a = &mut g.ch[c].anim;
    a.idle_t = desync_hash(seed, 1.0) * 20.0;
    a.life.breath = desync_hash(seed, 2.0) * 6.28;
    a.life.shift_t = 2.0 + desync_hash(seed, 3.0) * 5.0;
    a.life.shift_side = if desync_hash(seed, 4.0) < 0.5 { -1.0 } else { 1.0 };
    a.hn.seed = seed;
}

/// a round fired: the shot's clock, and a long gun's recoil sums
pub fn anim_fire(g: &mut crate::game::Game, c: CharId) {
    g.ch[c].anim.shoot_t = 0.0;
    g.ch[c].anim.hn.fired = 0.0;
    if g.ch[c].anim.long_gun {
        let t = v3::clamp(g.ch[c].anim.trained, 0.0, 1.0);
        let r = &g.k.mv.recoil;
        let k = |p: [f64; 2]| v3::lerp(p[0], p[1], t);
        let (back, climb, yaw, rock, shoulder, head) = (k(r.back), k(r.climb), k(r.yaw), k(r.rock), k(r.shoulder), k(r.head));
        let m_ = &mut g.ch[c].anim.mvs;
        let h = round_hash(m_.n);
        m_.n += 1.0;
        m_.back = m::min(m_.back + back, back * 1.35);
        m_.climb += climb * (0.8 + 0.4 * h);
        m_.yaw += yaw * (if h < 0.7 { -1.0 } else { 1.0 }) * (0.5 + h);
        m_.rock += rock;
        m_.sho += shoulder;
        m_.head += head;
    }
}

/// the Hit_Chest layer's clock
pub fn anim_hit(g: &mut crate::game::Game, c: CharId) {
    g.ch[c].anim.hit_t = 0.0;
}

/// startled, from his front
pub fn anim_flinch(g: &mut crate::game::Game, c: CharId) {
    anim_flinch_from(g, c, None, None, None);
}

/// rounds going past: a duck away from where they come from
pub fn anim_flinch_from(g: &mut crate::game::Game, c: CharId, toward: Option<V3>, k: Option<f64>, soft: Option<bool>) {
    let (k, soft) = (k.unwrap_or(1.0), soft.unwrap_or(false));
    let dur = g.k.mv.flinch.dur;
    let d = v3::lerp(dur[0], dur[1], v3::clamp(g.ch[c].anim.trained, 0.0, 1.0));
    let fl_t = g.ch[c].anim.mvs.fl_t;
    if fl_t >= 0.0 {
        let m_ = &mut g.ch[c].anim.mvs;
        m_.fl_k_to = m::max(m_.fl_k_to, k);
        if !soft { m_.fl_soft = false; }
    }
    if fl_t >= 0.0 && fl_t < 0.34 * d { return; }
    if fl_t >= 0.0 {
        g.ch[c].anim.mvs.fl_t = 0.14 * d * v3::clamp(curve(fl_t / d, &crate::character::MV_FLINCH_ENV), 0.0, 1.0);
        return;
    }
    let fl_d = match toward { Some(w) => anim_flat_model(g, c, w, [0.0, 0.0, 1.0]), None => [0.0, 0.0, 1.0] };
    let m_ = &mut g.ch[c].anim.mvs;
    m_.fl_d = fl_d;
    m_.fl_t = 0.0;
    m_.fl_k_to = k;
    m_.fl_k = k;
    m_.fl_soft = soft;
}

/// the start of a man who has just seen what he did not expect
pub fn anim_startle_from(g: &mut crate::game::Game, c: CharId, toward: Option<V3>) {
    if g.ch[c].anim.mvs.st_t >= 0.0 { return; }
    let st_d = match toward { Some(w) => anim_flat_model(g, c, w, [0.0, 0.0, 1.0]), None => [0.0, 0.0, 1.0] };
    g.ch[c].anim.mvs.st_d = st_d;
    g.ch[c].anim.mvs.st_t = 0.0;
}

/// a round that went home on a man who stays up
pub fn anim_hit_from(g: &mut crate::game::Game, c: CharId, along: Option<V3>) {
    let ht_t = g.ch[c].anim.mvs.ht_t;
    let e0 = if ht_t >= 0.0 { v3::clamp(curve(ht_t, &crate::character::MV_HIT_ENV), 0.0, 1.0) } else { 0.0 };
    g.ch[c].anim.mvs.ht_t = 0.07 * e0;
    if e0 > 0.15 { return; }
    let ht_d = match along { Some(w) => anim_flat_model(g, c, w, [0.0, 0.0, -1.0]), None => [0.0, 0.0, -1.0] };
    let m_ = &mut g.ch[c].anim.mvs;
    m_.ht_d = ht_d;
    m_.ht_side = if m::abs(ht_d[0]) > 0.3 {
        m::sign(ht_d[0])
    } else {
        let n = m_.n;
        m_.n = n + 1.0;
        if js::to_i32(n) & 1 != 0 { 1.0 } else { -1.0 }
    };
}

/// the support hand's underhand toss
pub fn anim_toss_item(g: &mut crate::game::Game, c: CharId, release: Option<f64>) {
    let release = release.unwrap_or(crate::character::TOSS_RELEASE);
    let m_ = &mut g.ch[c].anim.mvs;
    if m_.ts_t >= 0.0 { return; }
    m_.ts_t = 0.0;
    m_.ts_rel = v3::clamp(release, 0.08, 0.6);
}

/// a world direction, flattened, in the figure's un-twisted model frame
pub(super) fn anim_flat_model(g: &crate::game::Game, c: CharId, w: V3, fb: V3) -> V3 {
    let d = anim_to_model_d(g, c, [w[0], 0.0, w[2]]);
    let l = m::hyp2(d[0], d[2]);
    if l > 1e-4 { [d[0] / l, 0.0, d[2] / l] } else { fb }
}

/// the takedown strike's clock, and the knife's when it is lethal
pub fn anim_strike(g: &mut crate::game::Game, c: CharId, kind: Option<crate::character::StrikeKind>, lethal: Option<bool>) {
    let (kind, lethal) = (kind.unwrap_or(crate::character::StrikeKind::Cross), lethal.unwrap_or(false));
    let a = &mut g.ch[c].anim;
    a.strike_t = 0.0;
    a.strike_k = kind;
    a.kb_back = false;
    a.kb_slash = false;
    a.bw = None;
    a.bw_rise = 0.0;
    if lethal || a.strike_leth {
        a.strike_leth = lethal;
        a.kb_r0 = a.reach_shown;
        a.kb_t = 0.0;
    }
}

/// a takedown's own move, right after the strike it rides
pub fn anim_blow(g: &mut crate::game::Game, c: CharId, style: crate::character::BlowStyle, at: V3, at2: Option<V3>, o: Option<&crate::character::BlowOpts>) {
    let none = crate::character::BlowOpts::default();
    let o = o.unwrap_or(&none);
    let tail = if o.tail == Some(true) { crate::character::BLOW.tail } else { 0.0 };
    let live = o.live;
    let a = &mut g.ch[c].anim;
    a.bw = Some(crate::character::BlowRun { style, t: 0.0, hit: o.hit.unwrap_or(0.28), end: o.end.unwrap_or(0.62), tail, rise: o.rise.unwrap_or(true), at, at2, live });
    a.kb_slash = style == crate::character::BlowStyle::Slash;
    a.kb_back = style == crate::character::BlowStyle::Mouth;
}

/// the move called off: eased out from where it is
pub fn anim_blow_cancel(g: &mut crate::game::Game, c: CharId) {
    if let Some(b) = &mut g.ch[c].anim.bw {
        b.end = m::min(b.end, b.t + 0.18);
        b.tail = 0.0;
        b.live = None;
    }
}

/// the man it was being done to, let go of
pub fn anim_taken_cancel(g: &mut crate::game::Game, c: CharId) {
    let a = &mut g.ch[c].anim;
    let begun = match &mut a.tk {
        None => return,
        Some(t) => {
            if t.t - t.delay <= 0.0 {
                false
            } else {
                if t.off < 0.0 { t.off = 0.0; }
                true
            }
        }
    };
    if !begun { a.tk = None; }
}

/// the man it is done to, while he is still on his feet
pub fn anim_taken(g: &mut crate::game::Game, c: CharId, style: crate::character::TakenStyle, delay: Option<f64>) {
    g.ch[c].anim.tk = Some(crate::character::AnimTk { style, t: 0.0, delay: delay.unwrap_or(0.0), off: -1.0 });
}

/// the reload one-shot, retimed to `dur`, and the support hand's keys off the weapon's beat
pub fn anim_reload(g: &mut crate::game::Game, c: CharId, dur: Option<f64>, beat: Option<crate::character::ReloadBeat>) {
    g.ch[c].anim.reload_t = 0.0;
    let d = m::max(0.3, dur.unwrap_or(1.6));
    g.ch[c].anim.reload_dur = d;
    let b = match beat {
        Some(b) => b,
        None => crate::character::ReloadBeat { dur: d, out: None, in_: None, rack: None, keep: None, hand: None, exact: None },
    };
    anim_rl_beat(g, c, b);
    g.ch[c].anim.rl_t = 0.0;
}

/// the hand's keys off a weapon's beat
pub(super) fn anim_rl_beat(g: &mut crate::game::Game, c: CharId, b: crate::character::ReloadBeat) {
    rl_beat_keys(&mut g.ch[c].anim.rl_b, &b);
}

/// begin a change of what is in the right fist; the seconds until `to` is up
pub fn anim_kit_swap(g: &mut crate::game::Game, c: CharId, from: Option<crate::character::KitItem>, to: Option<crate::character::KitItem>, shown: Option<crate::character::AnimKitSwapShown>, same: Option<bool>) -> f64 {
    use crate::character::KitItem as Kit;
    let (mut from, same) = (from, same.unwrap_or(false));
    let (mut s0, mut roll0, mut re) = (0.0, 0.0, false);
    let mid_swap = g.ch[c].anim.kit.is_some();
    if mid_swap {
        from = g.ch[c].anim.kit_hold.flatten();   // (`?? null`: undefined and null alike)
        s0 = if from.is_some() { g.ch[c].anim.kit_stow } else { 0.0 };
        roll0 = if from.is_some() { g.ch[c].anim.kit_roll } else { 0.0 };
    }
    if from == to && !(same && !mid_swap && to.is_some()) {
        if !mid_swap { anim_kit_cancel(g, c); return 0.0; }
        re = to.is_some();
        from = None;
    }
    let front = g.ch[c].anim.kit_front;
    let (fr_stow, fr_reach, fr_draw) = (g.k.kit.front.stow, g.k.kit.front.reach, g.k.kit.front.draw);
    let both = from.is_some() && to.is_some();
    let q = if both { g.k.kit.swap_k } else { 1.0 };
    let far = if !front && both && (from == Some(Kit::Rifle)) != (to == Some(Kit::Rifle)) { g.k.kit.far } else { 0.0 };
    let t_a = match from { Some(it) => kit_sec(front, it, &g.k.kit.stow, fr_stow) * q, None => 0.0 };
    let t_r = match to {
        Some(it) if !re => t_a + (kit_sec(front, it, &g.k.kit.reach, fr_reach) + far) * q,
        _ => t_a,
    };
    let d = match to {
        Some(it) => t_r + kit_sec(front, it, &g.k.kit.draw, fr_draw) * q * (if re { 0.6 } else { 1.0 }),
        None => t_a + g.k.kit.away + (if from == Some(Kit::Rifle) && !front { g.k.kit.far } else { 0.0 }),
    };
    // where the hands are now: off the pose as last shown
    let pose = g.ch[c].anim.pose;
    let mut fk = crate::anim_skeleton::pose_fk_new(g, g.ch[c].anim.rig);
    crate::anim_skeleton::pose_fk_run(g, &mut fk, pose);
    let f = anim_trunk_frame(g, c, &fk);
    let (r, rq, l, lq) = {
        let nd = &g.ch[c].anim.nodes;
        let qi = v3::quat::conj(fk.rot[nd.spine3]);
        (loc_in(&f, fk.pos[nd.hand_r], fk.pos[nd.upper_arm_r]), v3::quat::mul(qi, fk.rot[nd.hand_r]),
         loc_in(&f, fk.pos[nd.hand_l], fk.pos[nd.upper_arm_l]), v3::quat::mul(qi, fk.rot[nd.hand_l]))
    };
    let (mr, ml, mf, mb) = match shown {
        Some(s) => (Some(anim_to_model_p(g, c, s.r)), Some(anim_to_model_p(g, c, s.l)), Some(anim_to_model_d(g, c, s.fingers)), Some(anim_to_model_d(g, c, s.back))),
        None => (None, None, None, None),
    };
    let cap = crate::character::AnimKitCap { r, rq, l, lq, mr, ml, mf, mb };
    g.ch[c].anim.kit = Some(crate::character::AnimKit { t: 0.0, from, to, t_a, t_r, d, s0, roll0, re, front, cap });
    g.ch[c].anim.kit_hold = Some(from.or(if re { to } else { None }));
    g.ch[c].anim.kit_stow = s0;
    g.ch[c].anim.kit_roll = roll0;
    d
}

/// the swap dropped where it stands: the kit is where the game says, at once
pub fn anim_kit_cancel(g: &mut crate::game::Game, c: CharId) {
    g.ch[c].anim.kit = None;
    g.ch[c].anim.kit_hold = None;   // (undefined: the kit is where the game says)
    g.ch[c].anim.kit_roll = 0.0;
    g.ch[c].anim.kit_stow = 0.0;
    g.ch[c].anim.kit_lie = None;
}

/// a swap is under way
pub fn anim_kit_busy(g: &crate::game::Game, c: CharId) -> bool {
    g.ch[c].anim.kit.is_some()
}

/// the goggles go down or up: by the left hand if `hand`, else a nod
pub fn anim_nv_flip(g: &mut crate::game::Game, c: CharId, down: bool, hand: bool) {
    let reach = g.k.kit.nv.hand.reach;
    let a = &mut g.ch[c].anim;
    let k0 = match a.nv_shown { Some(k) => k, None => if down { 0.0 } else { 1.0 } };
    let mut t = 0.0;
    if hand && a.nv.as_ref().map_or(false, |n| n.hand) && a.nv_arm > 0.02 {
        let (mut lo, mut hi) = (0.0, reach);
        for _ in 0..14 {
            let mid = (lo + hi) / 2.0;
            if v3::smoothstep(0.0, reach, mid) < a.nv_arm { lo = mid; } else { hi = mid; }
        }
        t = lo;
    }
    let c0 = if a.nv.is_some() { a.nv_nod } else { 0.0 };
    a.nv = Some(crate::character::AnimNv { t, hand, down, k0, c0 });
    a.nv_shown = Some(k0);
}

/// the flip let go of where it is
pub fn anim_nv_cancel(g: &mut crate::game::Game, c: CharId) {
    let a = &mut g.ch[c].anim;
    a.nv = None;
    a.nv_arm = 0.0;
    a.nv_nod = 0.0;
    a.nv_a = 0.0;
    a.nv_shown = None;
}

/// the errand under way has the rifle in it
pub(super) fn anim_kit_rifle(g: &crate::game::Game, c: CharId) -> bool {
    use crate::character::KitItem as Kit;
    match &g.ch[c].anim.kit {
        Some(k) => k.from == Some(Kit::Rifle) || k.to == Some(Kit::Rifle),
        None => false,
    }
}

/// a magazine change as the drawer needs it; none when none is shown
pub fn anim_reload_show(g: &crate::game::Game, c: CharId) -> Option<crate::character::AnimReloadShow> {
    let a = &g.ch[c].anim;
    if !(a.mv2_shown.reload > 0.5) { return None; }
    let (b, t) = (&a.rl_b, a.mv2.rl_at);
    Some(crate::character::AnimReloadShow {
        out: t >= b.out && t < b.in_,
        hand: (b.keep >= 0.0 && t >= b.out && t < b.keep) || (t >= b.fresh + 0.04 && t < b.in_),
    })
}

/// the change the support hand is showing, as its beat and where it is on it
pub fn anim_feed_clock(g: &crate::game::Game, c: CharId) -> Option<crate::character::AnimFeedClock> {
    let a = &g.ch[c].anim;
    if !(a.mv2_shown.reload > 0.5) { return None; }
    let b = &a.rl_b;
    Some(crate::character::AnimFeedClock { t: a.mv2.rl_at, dur: b.dur, out: b.out, in_: b.in_, rack: b.rack, pull: b.pull, keep: b.keep, fresh: b.fresh })
}

/// the hand's clock stops where it is and the arm eases back
pub fn anim_cancel_reload(g: &mut crate::game::Game, c: CharId) {
    g.ch[c].anim.reload_t = -1.0;
    g.ch[c].anim.rl_t = -1.0;
}

/// the overhand throw's clock, and which swing its distance reads as
pub fn anim_throw_item(g: &mut crate::game::Game, c: CharId, dist: Option<f64>, speed: Option<f64>) {
    let (lo, full, toss_fast) = (g.k.throw_by.toss, g.k.throw_by.full, g.k.throw_by.toss_fast);
    let a = &mut g.ch[c].anim;
    a.thr_carry = match &a.thr_last {
        Some(l) if a.throw_t >= 0.0 => Some(crate::character::AnimThrCarry { r: l.r, l: l.l }),
        _ => None,
    };
    a.throw_t = 0.0;
    a.thr_off = a.long_gun;
    a.thr_low = -1.0;
    a.thr_far = 1.0;
    a.thr_hang = -1.0;
    a.thr_hit0 = m::max(v3::clamp(a.mvs.cf.x, 0.0, 1.0), v3::clamp(a.mvs.dz.x, 0.0, 1.0));
    if let Some(d) = dist {
        if !d.is_nan() {
            let (low, far) = throw_by_read(lo, full, toss_fast, d, speed);
            a.thr_low = low;
            a.thr_far = far;
        }
    }
}

/// the reload one-shot is playing
pub fn anim_reloading(g: &crate::game::Game, c: CharId) -> bool {
    g.ch[c].anim.reload_t >= 0.0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::character::{AnimRlB, ReloadBeat};

    fn beat(dur: f64) -> ReloadBeat { ReloadBeat { dur, out: None, in_: None, rack: None, keep: None, hand: None, exact: None } }
    fn keys(b: ReloadBeat) -> AnimRlB {
        let mut k = AnimRlB::default();
        rl_beat_keys(&mut k, &b);
        k
    }
    const EPS: f64 = 1e-9;

    /// How he idles is his own: numbers from 0 to 1, the same each time he is made, and not the next man's.
    /// A FAILURE MEANS: a room of men breathe and shift their weight together.
    #[test]
    fn a_man_does_not_idle_in_step_with_the_next() {
        let mut sum = 0.0;
        for seed in 0..200 {
            let seed = seed as f64;
            let h = desync_hash(seed, 1.0);
            assert!((0.0..1.0).contains(&h), "seed {seed}: {h}");
            assert_eq!(h, desync_hash(seed, 1.0));
            assert_ne!(h, desync_hash(seed + 1.0, 1.0), "seed {seed}");
            assert_ne!(h, desync_hash(seed, 2.0), "seed {seed}: his clock and his breath are one number");
            sum += h;
        }
        assert!(m::abs(sum / 200.0 - 0.5) < 0.1, "the numbers lie across the range: their mean is {}", sum / 200.0);
    }

    /// A gun's kick differs from round to round.
    #[test]
    fn each_round_kicks_by_a_number_of_its_own() {
        for n in 1..500 {
            let h = round_hash(n as f64);
            assert!((0.0..1.0).contains(&h), "round {n}: {h}");
            assert!(m::abs(h - round_hash((n + 1) as f64)) > 1e-3, "rounds {n} and the next kick alike");
        }
    }

    /// A FAILURE MEANS: a short reload plays its magazine out, in and racked on top of each other.
    #[test]
    fn a_reloads_beats_come_in_order_with_room_for_each_however_short_it_is() {
        for dur in [0.1, 0.8, 1.6, 2.4, 6.0] {
            let k = keys(beat(dur));
            assert!(k.pull, "dur {dur}: a dry gun is racked");
            assert!(k.out >= 0.15 - EPS, "dur {dur}: mag out at {}", k.out);
            assert!(k.in_ >= k.out + 0.6 - EPS, "dur {dur}: mag in at {} after out at {}", k.in_, k.out);
            assert!(k.rack >= k.in_ + 0.2 - EPS, "dur {dur}: rack at {} after in at {}", k.rack, k.in_);
            assert!(k.dur >= k.rack + 0.25 - EPS && k.dur >= dur, "dur {dur}: it runs {} and racks at {}", k.dur, k.rack);
            assert!(k.fresh > k.out && k.fresh < k.in_, "dur {dur}: the new magazine shows at {}", k.fresh);
            assert_eq!(k.keep, -1.0, "dur {dur}: he keeps no old magazine unless asked to");
        }
        // a long one spreads out over its own length, and takes no longer than it was asked to
        let long = keys(beat(6.0));
        assert!(long.out > 1.0 && long.in_ > 3.0 && long.dur == 6.0, "{} {} {}", long.out, long.in_, long.dur);
    }

    #[test]
    fn a_round_still_chambered_is_no_rack() {
        let k = keys(ReloadBeat { rack: Some(None), ..beat(1.6) });
        assert!(!k.pull);
        assert!(k.dur >= k.in_ + 0.5 - EPS);
        assert!(keys(beat(1.6)).pull, "with no word on it he racks");
    }

    #[test]
    fn the_magazine_he_keeps_stays_in_his_hand_between_out_and_in() {
        for asked in [0.0, 1.0, 9.0] {
            let k = keys(ReloadBeat { out: Some(0.5), in_: Some(1.6), keep: Some(asked), ..beat(2.4) });
            assert!(k.keep >= k.out + 0.25 - EPS && k.keep <= k.in_ - 0.4 + EPS, "asked {asked}: stowed at {}", k.keep);
            assert!(k.fresh > k.keep && k.fresh < k.in_, "asked {asked}: the new one shows at {} after {}", k.fresh, k.keep);
        }
        let by_hand = keys(ReloadBeat { hand: Some(true), ..beat(2.0) });
        assert!(by_hand.hand && by_hand.fresh < keys(beat(2.0)).fresh, "a magazine from the hand shows sooner");
    }

    /// An exact beat is a clip's own times, floors or no floors.
    #[test]
    fn an_exact_beat_keeps_its_times() {
        let exact = ReloadBeat { out: Some(0.1), in_: Some(0.2), exact: Some(true), ..beat(1.1) };
        let k = keys(exact);
        assert_eq!((k.out, k.in_, k.dur, k.pull, k.rack), (0.1, 0.2, 1.1, false, 1.1));
        let k = keys(ReloadBeat { rack: Some(Some(0.9)), ..exact });
        assert_eq!((k.pull, k.rack), (true, 0.9));
    }

    /// A short throw is a toss from below, unless it is hard; anything else is the overhand, by how far.
    #[test]
    fn a_short_throw_is_a_toss_unless_it_is_hard() {
        let (lo, full, fast) = (2.3, 7.1, 8.0);
        let read = |d: f64, s: Option<f64>| throw_by_read(lo, full, fast, d, s);
        let (low, far) = read(1.15, None);
        assert_eq!((low, far), (0.5, 1.0), "half the toss's reach: half a toss");
        assert_eq!(read(1.15, Some(fast - 1.0)).0, 0.5, "a slow one is a toss");
        assert_eq!(read(1.15, Some(fast)).0, -1.0, "a hard one is no toss");
        assert_eq!(read(1.15, Some(fast)).1, 0.0, "and is the shortest overhand");
        assert_eq!(read(lo, None).0, -1.0, "at the toss's reach, the overhand");
        let (a, b, c) = (read(lo + 0.5, None).1, read(5.0, None).1, read(full, None).1);
        assert!(a > 0.0 && a < b && b < c && c == 1.0, "the overhand grows with the distance: {a} {b} {c}");
        assert_eq!(read(99.0, None).1, 1.0);
    }

    /// The pairs are by hand, and a swap would compile: a joint's word names the rig's joint on that side, and no two words name one joint.
    #[test]
    fn each_joints_word_names_its_own_side_of_the_rig() {
        let nn = crate::character::n();
        let e = n_entries(&nn);
        for (k, name) in e {
            if k.ends_with('L') { assert!(name.ends_with(".L"), "{k} names {name}"); }
            if k.ends_with('R') { assert!(name.ends_with(".R"), "{k} names {name}"); }
        }
        for (i, a) in e.iter().enumerate() {
            assert!(e[i + 1..].iter().all(|b| a.1 != b.1), "{} is named twice", a.1);
        }
    }

    /// each end and its ancestors, in the rig's order and no other
    #[test]
    fn fk_list_is_ancestors_in_order() {
        let mut rig = crate::anim_gltf::GltfCharacter::default();
        for parent in [-1.0, 0.0, 1.0, 1.0, 3.0, 0.0] {
            rig.nodes.push(crate::anim_gltf::GltfNode { parent, ..crate::anim_gltf::GltfNode::default() });
        }
        rig.order = vec![0.0, 5.0, 1.0, 3.0, 2.0, 4.0];
        assert_eq!(fk_list(&rig, &[4.0]), vec![0, 1, 3, 4]);
        assert_eq!(fk_list(&rig, &[2.0, 5.0]), vec![0, 5, 1, 2]);
        assert_eq!(fk_list(&rig, &[]), Vec::<i32>::new());
    }

    /// Two toes of a run clip of n frames: each lowest at its phase and travelling back there.
    fn toes(left: f64, right: f64, back: f64) -> (Vec<Vec<f64>>, Vec<Vec<f64>>) {
        let n = 37;
        let wave = |at: f64, f: &dyn Fn(f64) -> f64| -> Vec<f64> { (0..n).map(|i| f(2.0 * m::PI * (i as f64 / n as f64 - at))).collect() };
        let y = |at: f64| wave(at, &|a| 0.1 - 0.08 * m::cos(a));
        let z = |at: f64| wave(at, &|a| -back * 0.2 * m::sin(a));
        (vec![y(left), y(right)], vec![z(left), z(right)])
    }

    /// A stance is when a foot is at its lowest and going back under him: the run's clock lands his feet by it.
    /// A FAILURE MEANS: a running man's feet land out of time with the ground going by.
    #[test]
    fn a_stance_is_when_a_foot_is_low_and_going_back() {
        let (ty, tz) = toes(0.2, 0.7, 1.0);
        let s = stance_of(&ty, &tz).unwrap();
        assert!(m::abs(s.c_l - 0.2) < 0.02, "the left foot's stance is at {}", s.c_l);
        assert!(m::abs(s.c_r - 0.7) < 0.02, "the right foot's stance is at {}", s.c_r);
        assert!(m::abs(s.c - 0.2) < 0.02, "the pair's, within a half cycle, is at {}", s.c);
        // a foot that is low only as it comes forward has no stance, and nor has one that never goes back
        let (ty, tz) = toes(0.2, 0.7, -1.0);
        assert!(stance_of(&ty, &tz).is_none());
        let flat = vec![vec![1.0; 4], vec![1.0; 4]];
        assert!(stance_of(&flat, &flat).is_none());
    }

    /// A loop whose last key misses its first would jump once a cycle: half the gap comes off each end, so the two ends meet.
    #[test]
    fn the_jog_clips_seam_closes_and_touches_nothing_else() {
        use crate::character::AnimJogSeamItem as S;
        let rows = 3;
        let start = || -> Vec<f32> {
            let mut d = vec![0.0f32; 7 * rows];
            for i in 0..rows { d[6 * rows + i] = 1.0; }
            d
        };
        let (gap, ang) = ([0.2, -0.1, 0.4], 0.3);
        let seam = [S { node: 1.0, d: Some(gap), axis: None, ang: None }, S { node: 2.0, d: None, axis: Some([0.0, 1.0, 0.0]), ang: Some(ang) }];
        let at = |k: f64| { let mut d = start(); jog_seam_close(&mut d, rows, &seam, k); d };
        assert_eq!(at(0.0), start(), "the middle of the loop is the clip's own");
        let (first, last) = (at(-0.5), at(0.5));
        for c in 0..3 {
            let got = (last[c * rows + 1] - first[c * rows + 1]) as f64;
            assert!(m::abs(got - gap[c]) < 1e-6, "column {c}: the ends part by {got}, the gap is {}", gap[c]);
        }
        let turn = moves_angle(&first, &last, rows, 2);
        assert!(m::abs(turn - ang) < 1e-5, "the ends part by a turn of {turn}, the gap is {ang}");
        for k in 0..7 { assert_eq!(first[k * rows], start()[k * rows], "a node with no seam stays"); }
    }

    /// the turn between a node's rotation in two poses
    fn moves_angle(a: &[f32], b: &[f32], rows: usize, node: usize) -> f64 {
        let q = |d: &[f32]| -> [f64; 4] { let mut o = [0.0; 4]; for c in 0..4 { o[c] = d[(3 + c) * rows + node] as f64; } o };
        let (x, y) = (q(a), q(b));
        2.0 * m::acos(m::min(1.0, m::abs(x[0] * y[0] + x[1] * y[1] + x[2] * y[2] + x[3] * y[3])))
    }
}
