//! src/game/character.ts lines 4293 to 4876
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};


// ---- the seam: what skeleton 4 still cannot do (notes.md, ## Skeleton) ----

/// standMan(this, men ?? loneMen()): Character holds no ManCols and ManCols is empty, so there is nothing to write his row into (ts:4356)
fn seam_stand_man(g: &mut Game, id: CharId, men: Option<crate::Opaque>) { todo!("needs: Character to hold his ManCols (the owner part), for stand_man to write his row") }

// ---- small private helpers ----

/// a joint of `bones` that the TypeScript reads as there (undefined would throw at its first use)
fn bone(g: &Game, id: CharId, k: Bone) -> V3 { *g.ch[id].bones.get(k).unwrap() }
/// the Bone a string key names
fn bone_of(name: &str) -> Option<Bone> { BONE_ORDER.iter().copied().find(|b| b.as_str() == name) }
/// `ov[i] = o`: an array grows to take it, as JavaScript's does
fn ov_set(ov: &mut Vec<Option<crate::anim_skeleton::NodeOverride>>, i: usize, o: crate::anim_skeleton::NodeOverride) {
    if ov.len() <= i { ov.resize(i + 1, None); }
    ov[i] = Some(o);
}
/// skel.nodePos(node)
fn node_pos(g: &mut Game, id: CharId, node: usize) -> V3 { crate::anim_skeleton::skeleton_instance_node_pos(&mut g.ch[id].skel, node as f64) }
/// skel.update(pose, world, anim.twists), for a pose that is not a field of his: the pose is the caller's copy
fn skel_pose_twisted(g: &mut Game, id: CharId, pose: &crate::anim_skeleton::Pose) {
    let c = &mut g.ch[id];
    crate::anim_skeleton::skeleton_instance_update(&mut c.skel, pose, &c.world, Some(&c.anim.twists), None, &g.tab.rig[c.rig]);
}
/// skel.update(anim.pose, world, anim.twists, ov)
fn skel_anim_pose(g: &mut Game, id: CharId, ov: Option<&[Option<crate::anim_skeleton::NodeOverride>]>) {
    let c = &mut g.ch[id];
    crate::anim_skeleton::skeleton_instance_update(&mut c.skel, &c.anim.pose, &c.world, Some(&c.anim.twists), ov, &g.tab.rig[c.rig]);
}
/// skel.update(rd.pose, world, null, rd.ov)
fn skel_rag_pose(g: &mut Game, id: CharId) {
    let c = &mut g.ch[id];
    let rd = c.rd.as_ref().unwrap();
    crate::anim_skeleton::skeleton_instance_update(&mut c.skel, &rd.pose, &c.world, None, Some(&rd.ov), &g.tab.rig[c.rig]);
}
/// the same word in the field's type and the parameter's (two enums of one shape in the skeleton)
fn word_cur(w: crate::inhand::InHandWord) -> CharacterInHandWordCur {
    match w {
        crate::inhand::InHandWord::Pistol => CharacterInHandWordCur::Pistol,
        crate::inhand::InHandWord::Torch(t) => CharacterInHandWordCur::Torch(t),
        crate::inhand::InHandWord::Long(l) => CharacterInHandWordCur::Long(l),
    }
}
fn same_kind(a: &crate::inhand::InHandWord, b: &crate::inhand::InHandWord) -> bool { core::mem::discriminant(a) == core::mem::discriminant(b) }

/// RAG_LIMBS with each row's name read as its node (`n[k]`): [node, parent row (−1 = the rigid trunk), from-particle, to-particle] (ts:4262)
fn limb_rows(n: &AnimNodes) -> Vec<(usize, f64, usize, usize)> {
    rag_limbs().iter().map(|(k, parent, a, c)| {
        let node = match k.as_str() {
            "thighL" => n.thigh_l, "shinL" => n.shin_l, "thighR" => n.thigh_r, "shinR" => n.shin_r,
            "upperArmL" => n.upper_arm_l, "forearmL" => n.forearm_l, "upperArmR" => n.upper_arm_r, "forearmR" => n.forearm_r,
            "neck" => n.neck,
            _ => unreachable!("RAG_LIMBS names only these nine"),
        };
        (node, *parent, *a as usize, *c as usize)
    }).collect()
}
/// RAG_BONES: the Character.bones key each particle is seeded from, in particle order (ts:4268 to 4269; the skeleton's rag_bones() is empty, notes.md)
fn rag_bone_order(r: &crate::ragdoll::RAG) -> [Bone; 16] {
    let mut o = [Bone::Hips; 16];
    for (i, b) in [
        (r.pelvis, Bone::Hips), (r.spine, Bone::Spine2), (r.neck, Bone::Neck), (r.head, Bone::Head),
        (r.sho_l, Bone::UpperArmL), (r.sho_r, Bone::UpperArmR), (r.elb_l, Bone::ForearmL), (r.elb_r, Bone::ForearmR),
        (r.hand_l, Bone::HandL), (r.hand_r, Bone::HandR), (r.hip_l, Bone::ThighL), (r.hip_r, Bone::ThighR),
        (r.knee_l, Bone::ShinL), (r.knee_r, Bone::ShinR), (r.ank_l, Bone::FootL), (r.ank_r, Bone::FootR),
    ] { o[i as usize] = b; }
    o
}

// ---- construction and the plain frames ----

/// A body: his columns, his animator, his skeleton, and the rest pose's frames. `who` stays blank: the owner writes it. (Character.constructor)
pub fn character_new(g: &mut crate::game::Game, ch: RigId, d: crate::character::CharacterDesc) -> CharId {
    let id = g.ch.push(Character::blank());
    g.ch[id].rig = ch;
    seam_stand_man(g, id, d.men);
    g.ch[id].id = d.id; g.ch[id].tint = d.tint; g.ch[id].tint2 = d.tint2; g.ch[id].pos = v3::copy(d.pos); g.ch[id].body_yaw = d.yaw; g.ch[id].aim_yaw = d.yaw;
    let anim = anim_new(g, ch);
    g.ch[id].anim = anim;
    anim_desync(g, id, d.id);
    let sk = crate::anim_skeleton::skeleton_instance_new(g, ch);
    g.ch[id].skel = sk;
    // rest-pose chest rotation for "facing" extraction
    let mut blank = crate::anim_skeleton::pose_new(g.tab.rig[ch].nodes.len() as f64);
    let rp = crate::anim_skeleton::rest_pose(&g.tab.rig[ch], &mut blank);
    crate::anim_skeleton::skeleton_instance_update(&mut g.ch[id].skel, &rp, &v3::m4::create(), None, None, &g.tab.rig[ch]);
    let n = g.ch[id].anim.nodes;
    g.ch[id].rest_chest_inv = v3::quat::conj(g.ch[id].skel.global_rot[n.spine3]);
    g.ch[id].rest_head_inv = v3::quat::conj(g.ch[id].skel.global_rot[n.head]);
    {
        let hr = g.ch[id].skel.global_rot[n.hand_r]; let hi = v3::quat::conj(hr);
        let f = v3::normalize(v3::quat::rotate(hr, [0.0, 1.0, 0.0])); let b = v3::normalize(v3::mad([0.0, 1.0, 0.0], f, -f[1]));
        g.ch[id].hand_r_back = v3::quat::rotate(hi, b); g.ch[id].hand_r_thumb = v3::quat::rotate(hi, v3::cross(b, f));
        let (kn, hn) = (node_pos(g, id, n.middle1_r), node_pos(g, id, n.hand_r));
        let k = v3::sub(kn, hn);
        if v3::len(k) > 1e-4 {
            let bore = v3::normalize(v3::quat::rotate(hi, k));
            g.ch[id].hand_r_bore = bore;
            let th = g.ch[id].hand_r_thumb;
            let t = v3::mad(th, bore, -v3::dot(th, bore));
            if v3::len(t) > 1e-3 { g.ch[id].hand_r_top = v3::normalize(t); }
        }
    }
    {
        let hr = g.ch[id].skel.global_rot[n.hand_l]; let hi = v3::quat::conj(hr);
        let f = v3::normalize(v3::quat::rotate(hr, [0.0, 1.0, 0.0])); let b = v3::normalize(v3::mad([0.0, 1.0, 0.0], f, -f[1]));
        g.ch[id].hand_l_back = v3::quat::rotate(hi, b); g.ch[id].hand_l_thumb = v3::quat::rotate(hi, v3::cross(f, b));
    }
    id
}

/// (Character.forward)
pub fn character_forward(g: &mut crate::game::Game, id: CharId) -> V3 {
    let y = g.ch[id].body_yaw;
    [m::sin(y), 0.0, m::cos(y)]
}
/// (Character.aimDir)
pub fn character_aim_dir(g: &mut crate::game::Game, id: CharId) -> V3 {
    let (p, y) = (g.ch[id].aim_pitch, g.ch[id].aim_yaw);
    let cp = m::cos(p);
    [m::sin(y) * cp, m::sin(p), m::cos(y) * cp]
}
/// The right hand's frame in the world; `sim`: as the animator posed it, a game rule's. (Character.handFrameR)
pub fn character_hand_frame_r(g: &mut crate::game::Game, id: CharId, sim: Option<bool>) -> crate::character::CharacterHandFrameR {
    let sim = sim.unwrap_or(false);
    let c = &g.ch[id];
    let n = c.anim.nodes.hand_r;
    let h = match c.sim_hand_r { Some(s) if sim => s, _ => c.skel.global_rot[n] };
    let r = v3::quat::mul(v3::quat::yaw(c.body_yaw), h);
    CharacterHandFrameR { fingers: v3::quat::rotate(r, [0.0, 1.0, 0.0]), back: v3::quat::rotate(r, c.hand_r_back), thumb: v3::quat::rotate(r, c.hand_r_thumb) }
}
/// The frame a gun in the right hand is rigid in, as the hand is drawn. (Character.gripFrameR)
pub fn character_grip_frame_r(g: &mut crate::game::Game, id: CharId) -> crate::character::CharacterGripFrameR {
    let c = &g.ch[id];
    let r = v3::quat::mul(v3::quat::yaw(c.body_yaw), c.skel.global_rot[c.anim.nodes.hand_r]);
    CharacterGripFrameR { bore: v3::quat::rotate(r, c.hand_r_bore), top: v3::quat::rotate(r, c.hand_r_top) }
}
/// How a gun on a kit errand lies off the right hand, in the world. (Character.kitLieWorld)
pub fn character_kit_lie_world(g: &mut crate::game::Game, id: CharId) -> Option<Quat> {
    let c = &g.ch[id];
    let d = c.anim.kit_lie?;
    let r = v3::quat::mul(v3::quat::yaw(c.body_yaw), c.skel.global_rot[c.anim.nodes.hand_r]);
    Some(v3::quat::mul(v3::quat::mul(r, d), v3::quat::conj(r)))
}
/// (Character.handFrameL)
pub fn character_hand_frame_l(g: &mut crate::game::Game, id: CharId) -> crate::character::CharacterHandFrameL {
    let c = &g.ch[id];
    let r = v3::quat::mul(v3::quat::yaw(c.body_yaw), c.skel.global_rot[c.anim.nodes.hand_l]);
    CharacterHandFrameL { fingers: v3::quat::rotate(r, [0.0, 1.0, 0.0]), back: v3::quat::rotate(r, c.hand_l_back), thumb: v3::quat::rotate(r, c.hand_l_thumb) }
}
/// The chest's frame in the world; None before his first update. (Character.chestFrame)
pub fn character_chest_frame(g: &mut crate::game::Game, id: CharId) -> Option<crate::character::CharacterChestFrame> {
    let c = &g.ch[id];
    let o = *c.bones.get(Bone::Chest)?;
    let r = v3::quat::mul(v3::quat::yaw(c.body_yaw), v3::quat::mul(c.skel.global_rot[c.anim.nodes.spine3], c.rest_chest_inv));
    Some(CharacterChestFrame { o, left: v3::quat::rotate(r, [1.0, 0.0, 0.0]), up: v3::quat::rotate(r, [0.0, 1.0, 0.0]), fwd: v3::quat::rotate(r, [0.0, 0.0, 1.0]) })
}

// ---- the frame ----

/// His frame, the stage his for the whole of it. (Character.update)
pub fn character_update(g: &mut crate::game::Game, id: CharId, dt: f64) {
    let lent = crate::anim_skeleton::stage_open(g, crate::Opaque);
    character_update_standing(g, id, dt);
    if lent { crate::anim_skeleton::stage_shut(g, crate::Opaque); }
}
/// The turn, the animator, the skeleton and what is read off it. (Character.updateStanding)
pub fn character_update_standing(g: &mut crate::game::Game, id: CharId, dt: f64) {
    if g.ch[id].ragdoll.is_some() { character_update_ragdoll(g, id, dt); return; }
    if g.ch[id].turn.on && dt > 0.0 && g.ch[id].anim.wound.is_none() { character_turn_weigh(g, id, dt); }
    else {
        let (jump, lead_rate) = (g.k.turn.jump, g.k.turn.lead_rate);
        let c = &mut g.ch[id];
        match c.tn {
            Some(t) if dt <= 0.0 && c.turn.on && (c.body_yaw == t.body || c.body_yaw == t.ib) && (c.aim_yaw == t.aim || c.aim_yaw == t.ia) && m::hyp2(c.pos[0] - t.x, c.pos[2] - t.z) <= jump => { c.body_yaw = t.body; c.aim_yaw = t.aim; }
            Some(_) => c.tn = None,
            None => {}
        }
        if c.anim.turn_lead != 0.0 && c.tn.is_none() {
            let l = if dt > 0.0 { v3::damp(c.anim.turn_lead, 0.0, lead_rate * 2.5, dt) } else { 0.0 };
            c.anim.turn_lead = if m::abs(l) < 1e-4 { 0.0 } else { l };
        }
    }
    {
        let c = &mut g.ch[id];
        c.anim.aim_yaw_offset = v3::clamp(v3::wrap_angle(c.aim_yaw - c.body_yaw), -1.9, 1.9);
        c.anim.aim_pitch = c.aim_pitch;
        c.anim.body_yaw_in = c.body_yaw;
    }
    anim_update(g, id, dt);   // (anim.posIn = this.pos is gone with the alias: the animator reads g.ch[c].pos)
    let (pos, by) = (g.ch[id].pos, g.ch[id].body_yaw);
    g.ch[id].world = v3::m4::from_trs(pos, v3::quat::yaw(by), [1.0, 1.0, 1.0]);
    let n = g.ch[id].anim.nodes;
    // the gun hand under the torch ↔ sidearm swap: what the bore and the muzzle are read off
    match g.ch[id].anim.hand_pre.clone() {
        Some(hp) => {
            let world = g.ch[id].world;
            skel_pose_twisted(g, id, &hp);
            let (a, b) = (node_pos(g, id, n.hand_r), node_pos(g, id, n.middle1_r));
            g.ch[id].gun_hand = Some([v3::m4::transform_point(&world, a), v3::m4::transform_point(&world, b)]);
        }
        None => g.ch[id].gun_hand = None,
    }
    // the joints under the moves' look-only arms, while they show: the ragdoll's seed
    let pre = anim_under_pose(g, id);
    if dt > 1e-5 { g.ch[id].prev_sim = g.ch[id].sim_bones.clone(); }
    match pre {
        Some(pre) => {
            skel_pose_twisted(g, id, &pre);
            let mut m: JsMap<Bone, V3> = JsMap::new();
            joints_into(g, id, &mut m);
            g.ch[id].sim_bones = Some(m);
        }
        None => g.ch[id].sim_bones = None,
    }
    skel_anim_pose(g, id, None);
    character_read_bones(g, id, dt);
    character_in_hand_pass(g, id, dt);
    if g.ch[id].anim.big_side.is_some() && g.ch[id].anim.big_at_back > 0.0 { character_pack_side_pass(g, id); }
}

/// The fist that goes for the hammer goes to it at the side of his pack: the drawn skeleton only. (Character.packSidePass)
pub fn character_pack_side_pass(g: &mut crate::game::Game, id: CharId) {
    let (n, sd, w, big_clear, big_grab) = { let an = &g.ch[id].anim; (an.nodes, an.big_side.unwrap(), an.big_at_back, an.big_clear, an.big_grab) };
    {
        let b = &g.ch[id].bones;
        if !(b.has(Bone::Hips) && b.has(Bone::Spine2) && b.has(Bone::ThighL) && b.has(Bone::ThighR) && b.has(Bone::UpperArmL) && b.has(Bone::UpperArmR)) { return; }
    }
    let qi = v3::quat::conj(v3::quat::yaw(g.ch[id].body_yaw));
    let old = crate::rig_props::big_back_of(bone(g, id, Bone::Hips), bone(g, id, Bone::Spine2), bone(g, id, Bone::ThighL), bone(g, id, Bone::ThighR), bone(g, id, Bone::UpperArmL), bone(g, id, Bone::UpperArmR), crate::rig_props::bigrig().hammer.back, Some(big_clear));
    let g0 = v3::quat::rotate(qi, v3::sub(v3::mad(old.o, old.z, big_grab), g.ch[id].pos)); let d0 = v3::quat::rotate(qi, old.z);
    let r3 = g.ch[id].skel.global_rot[n.spine3]; let d1 = v3::quat::rotate(r3, sd.z);
    let s3 = node_pos(g, id, n.spine3);
    let g1 = v3::mad(v3::add(s3, v3::quat::rotate(r3, sd.o)), d1, big_grab);
    let mut q = v3::quat::from_to(v3::normalize(d0), v3::normalize(d1));
    if w < 1.0 { q = v3::quat::slerp([0.0, 0.0, 0.0, 1.0], q, w); }
    let r = node_pos(g, id, n.hand_r);
    let gr = g.ch[id].skel.global_rot[n.hand_r];
    let mut ov = g.ch[id].in_hand_ov.take().unwrap_or_default();
    if g.ch[id].sim_hand_r.is_none() { g.ch[id].sim_hand_r = Some(gr); }
    character_reach_ov(g, id, &mut ov, true, v3::add(v3::lerp3(g0, g1, w), v3::quat::rotate(q, v3::sub(r, g0))), v3::quat::normalize(v3::quat::mul(q, gr)));
    skel_anim_pose(g, id, Some(&ov));
    g.ch[id].in_hand_ov = Some(ov);
}
/// An arm re-solved so its wrist is at `t`, the hand's rotation `rot`, into `ov` and not the pose. (Character.reachOv)
pub fn character_reach_ov(g: &mut crate::game::Game, id: CharId, ov: &mut Vec<Option<crate::anim_skeleton::NodeOverride>>, right: bool, t: V3, rot: Quat) {
    let n = g.ch[id].anim.nodes;
    let (i_s, i_e, i_h) = if right { (n.upper_arm_r, n.forearm_r, n.hand_r) } else { (n.upper_arm_l, n.forearm_l, n.hand_l) };
    let (s, e, h) = (node_pos(g, id, i_s), node_pos(g, id, i_e), node_pos(g, id, i_h));
    let (l1, l2, st) = (v3::len(v3::sub(e, s)), v3::len(v3::sub(h, e)), v3::sub(t, s));
    if l1 < 1e-3 || l2 < 1e-3 || v3::len(st) < 1e-3 { return; }
    let dist = v3::clamp(v3::len(st), m::abs(l1 - l2) + 0.01, (l1 + l2) * 0.998);
    let dir = v3::normalize(st);
    let a = (l1 * l1 - l2 * l2 + dist * dist) / (2.0 * dist);
    let hh = m::sqrt(m::max(0.0, l1 * l1 - a * a));
    let mut pole = v3::sub(e, s); pole = v3::mad(pole, dir, -v3::dot(pole, dir));
    if v3::len(pole) < 1e-4 {
        pole = v3::mad([0.0, -1.0, 0.0], dir, dir[1]);
        if v3::len(pole) < 1e-4 { pole = v3::mad([0.0, 0.0, -1.0], dir, dir[2]); }
    }
    pole = v3::normalize(pole);
    let e2 = v3::mad(v3::mad(s, dir, a), pole, hh); let t2 = v3::mad(s, dir, dist);
    let (rs, re) = (g.ch[id].skel.global_rot[i_s], g.ch[id].skel.global_rot[i_e]);
    ov_set(ov, i_s, crate::anim_skeleton::NodeOverride { rot: v3::quat::normalize(v3::quat::mul(v3::quat::from_to(v3::normalize(v3::sub(e, s)), v3::normalize(v3::sub(e2, s))), rs)), pos: None });
    ov_set(ov, i_e, crate::anim_skeleton::NodeOverride { rot: v3::quat::normalize(v3::quat::mul(v3::quat::from_to(v3::normalize(v3::sub(h, e)), v3::normalize(v3::sub(t2, e2))), re)), pos: Some(e2) });
    ov_set(ov, i_h, crate::anim_skeleton::NodeOverride { rot, pos: Some(t2) });
}

// ---- in hand ----

/// The hand that holds a thing put where that thing is, the skeleton posed again over it: bones, gunDir and muzzle were read before. (Character.inHandPass)
pub fn character_in_hand_pass(g: &mut crate::game::Game, id: CharId, dt: f64) {
    let n = g.ch[id].anim.nodes;
    let w = g.ch[id].in_hand;
    let mut cur = g.ch[id].in_hand_cur;
    if let Some(wd) = w {
        let take = match &cur { None => true, Some(c) => g.ch[id].in_hand_k <= 0.0 || same_kind(c, &wd) };
        if take { cur = Some(wd); g.ch[id].in_hand_cur = Some(wd); }
    }
    let belt = anim_swap_belt(g, id);
    g.ch[id].sim_hand_r = None;
    let hr = g.ch[id].skel.global_rot[n.hand_r];
    let (r, l) = (node_pos(g, id, n.hand_r), node_pos(g, id, n.hand_l));
    let mut ov = g.ch[id].in_hand_ov.take().unwrap_or_default();
    ov.clear();
    // `reach` closes over `ov`, which inHandWord also writes: it is asked here and answered after (notes, Traps)
    let mut asked: Vec<(bool, V3, Quat)> = Vec::new();
    {
        let mut reach = |_g: &mut Game, right: bool, t: V3, rot: Quat| asked.push((right, t, rot));
        character_in_hand_word(g, id, dt, cur.map(word_cur), hr, r, l, &mut ov, &mut reach);
    }
    for (right, t, rot) in asked { character_reach_ov(g, id, &mut ov, right, t, rot); }
    // the swap's hand at his belt: the drawn hand goes the rest of the way out to the torch
    if belt > 0.0 {
        let h = ov.get(n.hand_r).copied().flatten();
        let bt = crate::inhand::belt_torch();
        let from = h.and_then(|o| o.pos).unwrap_or(r);
        let rot = h.map(|o| o.rot).unwrap_or(hr);
        character_reach_ov(g, id, &mut ov, true, v3::mad(from, [bt.was[0] - bt.at[0], 0.0, bt.at[1] - bt.was[1]], belt), rot);
    }
    if !ov.is_empty() {
        g.ch[id].sim_hand_r = Some(hr);
        skel_anim_pose(g, id, Some(&ov));
    }
    g.ch[id].in_hand_ov = Some(ov);
}

/// What the hand does for the thing it holds: this frame's overrides into `ov`, none when the pass has nothing to say. (Character.inHandWord)
pub fn character_in_hand_word(g: &mut crate::game::Game, id: CharId, dt: f64, cur: Option<crate::character::CharacterInHandWordCur>, hr: Quat, r: V3, l: V3, ov: &mut Vec<Option<crate::anim_skeleton::NodeOverride>>, reach: &mut dyn FnMut(&mut crate::game::Game, bool, V3, Quat)) {
    let n = g.ch[id].anim.nodes;
    let w = g.ch[id].in_hand;
    let cur = match cur { Some(c) => c, None => { g.ch[id].in_hand_k = 0.0; return; } };
    let is_long = matches!(cur, CharacterInHandWordCur::Long(_));
    let is_pistol = matches!(cur, CharacterInHandWordCur::Pistol);
    let is_torch = matches!(cur, CharacterInHandWordCur::Torch(_));
    let live = match w {
        None => false,
        Some(crate::inhand::InHandWord::Pistol) => is_pistol,
        Some(crate::inhand::InHandWord::Torch(_)) => is_torch,
        Some(crate::inhand::InHandWord::Long(_)) => is_long,
    };
    let shown = (!anim_in_throw(g, id) || (anim_throw_off(g, id) && is_long)) && !g.ch[id].anim.hide_held_item && !anim_kit_busy(g, id);
    let fire = live && shown && is_pistol && anim_shooting(g, id);
    let want = if !live || !shown { 0.0 }
        else if is_long { if g.ch[id].anim.long_shown.is_some() { 1.0 } else { 0.0 } }
        else if fire { 1.0 }
        else if !anim_gun_arm_own(g, id) { 0.0 }
        else if is_torch { 1.0 }
        else { let a = anim_aim_shown(g, id); v3::smoothstep(g.k.inhand.from, g.k.inhand.to, a) };
    let step = if dt > 0.0 { g.k.inhand.rate * dt } else { 1.0 };
    let k = if fire { 1.0 } else { let was = g.ch[id].in_hand_k; was + v3::clamp(want - was, -step, step) };
    g.ch[id].in_hand_k = k;
    if k <= 0.0 { if !live { g.ch[id].in_hand_cur = None; } return; }
    let qi = v3::quat::conj(v3::quat::yaw(g.ch[id].body_yaw));
    // the line the thing lies along, model space, and the frame it is laid out in
    let line = match cur {
        CharacterInHandWordCur::Torch(t) => t.beam,
        _ => if is_long || g.ch[id].anim.upper == UpperMode::Aim || anim_shooting(g, id) { g.ch[id].gun_dir } else { character_aim_dir(g, id) },
    };
    let z = v3::normalize(v3::quat::rotate(qi, line));
    let mut y: V3 = [-z[0] * z[1], 1.0 - z[1] * z[1], -z[2] * z[1]];
    if v3::len(y) < 1e-3 { return; }
    y = v3::normalize(y);
    let x = v3::cross(y, z);
    let in_f = |d: V3| -> V3 { [x[0] * d[0] + y[0] * d[1] + z[0] * d[2], x[1] * d[0] + y[1] * d[1] + z[1] * d[2], x[2] * d[0] + y[2] * d[1] + z[2] * d[2]] };
    if let CharacterInHandWordCur::Long(c) = cur { reach(g, true, v3::mad(r, in_f(c.by), k), hr); return; }
    let mut up = 1.0;
    let (bore, top, thumb, back) = { let c = &g.ch[id]; (c.hand_r_bore, c.hand_r_top, c.hand_r_thumb, c.hand_r_back) };
    let mut d: Quat = match cur {
        CharacterInHandWordCur::Torch(c) => {
            let tu = anim_torch_up(g, id);
            if tu > 0.0 {
                let e = v3::quat::from_to(v3::normalize(v3::quat::rotate(hr, thumb)), v3::scale(v3::normalize(in_f(c.axis)), if tu > 0.5 { -1.0 } else { 1.0 }));
                up = m::abs(2.0 * tu - 1.0);
                e
            } else { v3::quat::from_to(v3::normalize(v3::quat::rotate(hr, thumb)), v3::normalize(in_f(c.axis))) }
        }
        _ => {
            let (kc, tc) = (v3::quat::rotate(hr, bore), v3::quat::rotate(hr, top));
            v3::quat::mul(v3::quat::from_basis(x, y, z), v3::quat::conj(v3::quat::from_basis(v3::cross(tc, kc), tc, kc)))
        }
    };
    if d[3] < 0.0 { d = [-d[0], -d[1], -d[2], -d[3]]; }
    let ang = 2.0 * m::acos(m::min(1.0, d[3]));
    d = if ang < 1e-4 { [0.0, 0.0, 0.0, 1.0] } else {
        let s = if fire { 1.0 } else { k * m::min(1.0, g.k.inhand.wrist * m::PI / 180.0 / ang) * up };
        v3::quat::slerp([0.0, 0.0, 0.0, 1.0], d, s)
    };
    let rot = v3::quat::normalize(v3::quat::mul(d, hr));
    match cur {
        CharacterInHandWordCur::Torch(c) => {
            // the hollow of the turned fist onto the point of its body the simulation's wrist and beam put there
            let hol = v3::quat::rotate(rot, v3::mad([0.0, c.fwd, 0.0], back, -c.palm));
            reach(g, true, v3::mad(r, v3::sub(v3::sub(v3::add(r, in_f(c.at)), hol), r), k), rot);
        }
        _ => {
            ov_set(ov, n.hand_r, crate::anim_skeleton::NodeOverride { rot, pos: None });
            // the support hand, when it is on the gun: carried rigidly round the gun hand's wrist with the same turn
            let both = g.k.inhand.both;
            let near = 1.0 - v3::smoothstep(both * 0.5, both, v3::len(v3::sub(l, r)));
            if near > 0.0 {
                let dl = if near < 1.0 { v3::quat::slerp([0.0, 0.0, 0.0, 1.0], d, near) } else { d };
                let rl = g.ch[id].skel.global_rot[n.hand_l];
                reach(g, false, v3::add(r, v3::quat::rotate(dl, v3::sub(l, r))), v3::quat::normalize(v3::quat::mul(dl, rl)));
            }
        }
    }
}

// ---- the turn ----

/// His yaws come round to the AI's wish as a mass. (Character.turnWeigh)
pub fn character_turn_weigh(g: &mut crate::game::Game, id: CharId, dt: f64) {
    anim_rule_slot(g, id);
    character_turn_weigh_ask(g, id, dt);
}
/// where a layer's columns start on the bench's `args`
const TURN_COLS_AT: usize = crate::anim_skeleton::LAYER_COLS_AT as usize;
/// The bench's arguments, the rule, and his columns back; the column order is Character.turnCols. (Character.turnWeighAsk)
pub fn character_turn_weigh_ask(g: &mut crate::game::Game, id: CharId, dt: f64) {
    // LAYERS.knobs?.() is rustknobs' knobsNow: the list again, unless a tick has already handed it
    if !g.m.rustknobs.rust_knobs.tick { crate::rustknobs::knobs_to_rust(g); }
    {
        let ch = &g.ch[id];
        let a = &mut ::sim::skeleton::bench().args;
        a[0] = dt; a[1] = ch.pos[0]; a[2] = ch.pos[2]; a[3] = ch.turn.h; a[4] = ch.turn.k;
        // from LAYER_COLS_AT, Character.turnCols.get: rust/sim/src/layers.rs turn_weigh gives the order
        let o = TURN_COLS_AT;
        for i in 0..9 { a[o + i] = 0.0; }
        if let Some(t) = ch.tn { a[o] = 1.0; a[o + 1] = t.body; a[o + 2] = t.aim; a[o + 3] = t.wb; a[o + 4] = t.wa; a[o + 5] = t.ib; a[o + 6] = t.ia; a[o + 7] = t.x; a[o + 8] = t.z; }
        a[o + 9] = ch.anim.turn_lead; a[o + 10] = ch.body_yaw; a[o + 11] = ch.aim_yaw;
    }
    let code = ::sim::layers::turn_weigh();
    anim_rule_ran(g, id, code as f64);
    // Character.turnCols.put
    let mut c = [0.0f64; 12];
    c.copy_from_slice(&::sim::skeleton::bench().args[TURN_COLS_AT..TURN_COLS_AT + 12]);
    let ch = &mut g.ch[id];
    ch.tn = if js::truthy(c[0]) { Some(CharacterTn { body: c[1], aim: c[2], wb: c[3], wa: c[4], ib: c[5], ia: c[6], x: c[7], z: c[8] }) } else { None };
    ch.anim.turn_lead = c[9]; ch.body_yaw = c[10]; ch.aim_yaw = c[11];
}
/// The turn he is in the middle of, for a checkpoint: where his weight let him get to, its speeds, the wish, where he stood, the neck's lead. (Character.turnSnap)
pub fn character_turn_snap(g: &mut crate::game::Game, id: CharId) -> Option<Vec<f64>> {
    let c = &g.ch[id];
    match c.tn {
        Some(t) if c.ragdoll.is_none() => Some(vec![t.body, t.aim, t.wb, t.wa, t.ib, t.ia, t.x, t.z, c.anim.turn_lead]),
        _ => None,
    }
}
/// …and put back from a file, read as untrusted: nine finite numbers or nothing, the speeds held to what a turn can carry. (Character.turnPut)
pub fn character_turn_put(g: &mut crate::game::Game, id: CharId, s: Option<&[f64]>) {
    let s = match s { Some(s) if s.len() == 9 && s.iter().all(|v| v.is_finite()) => s, _ => return };
    if g.ch[id].ragdoll.is_some() { return; }
    let (wb, wa, lead) = (g.k.turn.body * 1.5, g.k.turn.aim * 1.5, g.k.turn.lead);
    let c = &mut g.ch[id];
    c.tn = Some(CharacterTn { body: s[0], aim: s[1], wb: v3::clamp(s[2], -wb, wb), wa: v3::clamp(s[3], -wa, wa), ib: s[4], ia: s[5], x: s[6], z: s[7] });
    c.anim.turn_lead = v3::clamp(s[8], -lead, lead);
    c.turn.on = true;
}
/// The loader's bake of a living man: posed where his weight had got him, his yaws then handed back as filed. (Character.bakeLoaded)
pub fn character_bake_loaded(g: &mut crate::game::Game, id: CharId) {
    let t = match g.ch[id].tn { Some(t) => t, None => { character_update(g, id, 0.0); return; } };
    let (b, a) = (g.ch[id].body_yaw, g.ch[id].aim_yaw);
    g.ch[id].body_yaw = t.body; g.ch[id].aim_yaw = t.aim;
    character_update(g, id, 0.0);
    g.ch[id].body_yaw = b; g.ch[id].aim_yaw = a;
    if g.ch[id].tn.is_none() { character_update(g, id, 0.0); }
}
/// His yaws become the ones that were drawn, and no turn is carried on. (Character.turnDrop)
pub fn character_turn_drop(g: &mut crate::game::Game, id: CharId) {
    let c = &mut g.ch[id];
    if let Some(t) = c.tn { c.body_yaw = t.body; c.aim_yaw = t.aim; c.tn = None; }
}
/// His AI is about to ease his yaws: they read back as it left them; one written since is the new wish. (Character.turnBegin)
pub fn character_turn_begin(g: &mut crate::game::Game, id: CharId) {
    let c = &mut g.ch[id];
    let t = match c.tn { Some(t) => t, None => return };
    if c.ragdoll.is_some() || !c.turn.on { return; }
    if c.body_yaw == t.body { c.body_yaw = t.ib; }
    if c.aim_yaw == t.aim { c.aim_yaw = t.ia; }
}
/// Where his gun is in yaw, for a fire gate inside his own AI update. (Character.gunYaw)
pub fn character_gun_yaw(g: &mut crate::game::Game, id: CharId) -> f64 {
    let c = &g.ch[id];
    match c.tn { Some(t) if c.ragdoll.is_none() => t.aim, _ => c.aim_yaw }
}
/// Where his AI wants his body, for an easing outside his own update. (Character.wantBody)
pub fn character_want_body(g: &mut crate::game::Game, id: CharId) -> f64 {
    let c = &g.ch[id];
    match c.tn { Some(t) if c.body_yaw == t.body => t.ib, _ => c.body_yaw }
}

// ---- joints ----

/// A joint as the game reads it: under the moves' look-only arms while one shows. (Character.jointUnder)
pub fn character_joint_under(g: &mut crate::game::Game, id: CharId, name: &str) -> Option<V3> {
    let k = bone_of(name)?;
    let c = &g.ch[id];
    if let Some(v) = c.sim_bones.as_ref().and_then(|m| m.get(k)) { return Some(*v); }
    c.bones.get(k).copied()
}
/// `joints` over the map the fields really are (Bone keys): the one body of Character.joints
fn joints_into(g: &mut Game, id: CharId, b: &mut JsMap<Bone, V3>) {
    let n = g.ch[id].anim.nodes;
    let world = g.ch[id].world;
    let rows: [(Bone, usize); 21] = [
        (Bone::Hips, n.hips), (Bone::Spine2, n.spine2), (Bone::Chest, n.spine3), (Bone::Neck, n.neck), (Bone::Head, n.head),
        (Bone::UpperArmL, n.upper_arm_l), (Bone::ForearmL, n.forearm_l), (Bone::HandL, n.hand_l), (Bone::UpperArmR, n.upper_arm_r), (Bone::ForearmR, n.forearm_r), (Bone::HandR, n.hand_r),
        (Bone::ThighL, n.thigh_l), (Bone::ShinL, n.shin_l), (Bone::FootL, n.foot_l), (Bone::ToeL, n.toe_l), (Bone::ThighR, n.thigh_r), (Bone::ShinR, n.shin_r), (Bone::FootR, n.foot_r), (Bone::ToeR, n.toe_r),
        (Bone::FingerR, n.middle1_r), (Bone::FingerL, n.middle1_l),
    ];
    for (k, node) in rows {
        let p = node_pos(g, id, node);
        b.set(k, v3::m4::transform_point(&world, p));
    }
    let head = *b.get(Bone::Head).unwrap();
    b.set(Bone::HeadTop, v3::add(head, [0.0, 0.2, 0.0]));
}
/// The world-space joints off the skeleton's current globals, into `b`. (Character.joints)
pub fn character_joints(g: &mut crate::game::Game, id: CharId, b: &mut JsMap<String, V3>) -> JsMap<String, V3> {
    let mut m: JsMap<Bone, V3> = JsMap::new();
    joints_into(g, id, &mut m);
    for k in BONE_ORDER { if let Some(v) = m.get(k) { b.set(String::from(k.as_str()), *v); } }
    b.clone()
}
/// The joints and what is derived from them: chest facing, gun axis, muzzle. (Character.readBones)
pub fn character_read_bones(g: &mut crate::game::Game, id: CharId, dt: f64) {
    let n = g.ch[id].anim.nodes;
    if dt > 1e-5 && g.ch[id].bones.has(Bone::Hips) {
        // (a dt-0 re-bake keeps the older sample: it is still one real frame back)
        let c = &mut g.ch[id];
        for k in c.bones.keys() { let v = *c.bones.get(k).unwrap(); c.prev_bones.set(k, v); }
        c.prev_dt = dt;
    }
    let mut b = core::mem::take(&mut g.ch[id].bones);
    joints_into(g, id, &mut b);
    g.ch[id].bones = b;
    let pos = g.ch[id].pos;
    g.ch[id].bones_at[0] = pos[0]; g.ch[id].bones_at[1] = pos[2];
    // chest facing (rest → current delta applied to model +Z), in world
    let q_yaw = v3::quat::yaw(g.ch[id].body_yaw);
    let chest_delta = v3::quat::mul(g.ch[id].skel.global_rot[n.spine3], g.ch[id].rest_chest_inv);
    g.ch[id].chest_dir = v3::normalize(v3::quat::rotate(v3::quat::mul(q_yaw, chest_delta), [0.0, 0.0, 1.0]));
    // gun direction: wrist → middle finger base, muzzle ahead of the hand
    let gh = if g.ch[id].ragdoll.is_some() { None } else { g.ch[id].gun_hand };
    let h_r = match gh { Some(h) => h[0], None => bone(g, id, Bone::HandR) };
    let far = match gh { Some(h) => h[1], None => bone(g, id, Bone::FingerR) };
    g.ch[id].gun_dir = v3::normalize(v3::sub(far, h_r));
    if g.ch[id].ragdoll.is_none() && (g.ch[id].anim.upper == UpperMode::Aim || anim_shooting(g, id)) && !anim_in_throw(g, id) && g.ch[id].anim.long_shown.is_none() {
        // when aiming the pistol, trust the analytic aim direction; a long gun's bore is the hands' own line
        let free = g.ch[id].anim.kit_gun_free;
        let ad = character_aim_dir(g, id);
        let gd = g.ch[id].gun_dir;
        g.ch[id].gun_dir = if free > 0.0 { v3::normalize(v3::lerp3(ad, gd, free)) } else { ad };
    }
    let (gd, ahead) = (g.ch[id].gun_dir, g.ch[id].muzzle_ahead);
    g.ch[id].muzzle = v3::mad(h_r, gd, ahead);
}

// ---- the ragdoll ----

/// Die: hand the skeleton to a ragdoll seeded from the pose under the look-only arms and its momentum, then shove it. (Character.ragdollize)
pub fn character_ragdollize(g: &mut crate::game::Game, id: CharId, world: Option<&crate::ragdoll::RagdollWorld>, dir: V3, quiet: bool) {
    if let Some(t) = g.ch[id].tn { g.ch[id].body_yaw = t.body; g.ch[id].aim_yaw = t.aim; g.ch[id].tn = None; }
    if g.ch[id].ragdoll.is_some() { clear_fall(g, id); return; }
    if !g.ch[id].bones.has(Bone::Hips) { character_update(g, id, 0.0); }
    if let Some(under) = anim_under_pose(g, id) {
        crate::anim_skeleton::pose_copy(&mut g.ch[id].anim.pose, &under);
        skel_anim_pose(g, id, None);
        character_read_bones(g, id, 0.0);
    }
    if g.ch[id].in_hand_k > 0.0 || g.ch[id].in_hand_ov.as_ref().map_or(false, |o| !o.is_empty()) {
        skel_anim_pose(g, id, None);
        let c = &mut g.ch[id];
        c.in_hand_k = 0.0; c.sim_hand_r = None; c.in_hand_cur = None;
        if let Some(o) = c.in_hand_ov.as_mut() { o.clear(); }
    }
    anim_nv_cancel(g, id);
    g.ch[id].alive = false; g.ch[id].vel = [0.0, 0.0, 0.0];
    let n = g.ch[id].anim.nodes;
    let rg = crate::ragdoll::rag();
    let pdt = g.ch[id].prev_dt;
    let y_q = v3::quat::yaw(g.ch[id].body_yaw);
    let head_up = v3::quat::rotate(v3::quat::mul(y_q, g.ch[id].skel.global_rot[n.head]), [0.0, 1.0, 0.0]);
    let rag_bones = rag_bone_order(&rg);
    let mut pts: Vec<V3> = rag_bones.iter().map(|k| v3::copy(bone(g, id, *k))).collect();
    pts[rg.head as usize] = v3::mad(bone(g, id, Bone::Head), head_up, 0.12);
    pts[rg.hand_l as usize] = v3::lerp3(bone(g, id, Bone::HandL), bone(g, id, Bone::FingerL), 0.5);
    pts[rg.hand_r as usize] = v3::lerp3(bone(g, id, Bone::HandR), bone(g, id, Bone::FingerR), 0.5);
    let vel: Option<Vec<V3>> = {
        let c = &g.ch[id];
        let pb = c.prev_sim.as_ref().unwrap_or(&c.prev_bones);
        if pdt > 1e-5 && pb.has(Bone::Hips) {
            Some(rag_bones.iter().map(|k| {
                let v = v3::scale(v3::sub(*c.bones.get(*k).unwrap(), *pb.get(*k).unwrap()), 1.0 / pdt);
                let l = v3::len(v);
                if l > 3.0 { v3::scale(v, 3.0 / l) } else { v }   // (capped: an animation pop is not momentum)
            }).collect())
        } else { None }
    };
    let rag = crate::ragdoll::ragdoll_new(g, &pts, vel.as_deref(), world);
    // the shove: carried along `dir` and pitched top-first the same way, then the knees kicked out
    let f = character_forward(g, id);
    let j = 0.85 + crate::math_dice::sim_random(g) * 0.3;
    let twist = (crate::math_dice::sim_random(g) - 0.5) * (if quiet { 0.8 } else { 1.6 });
    let h = v3::normalize([dir[0], 0.0, dir[2]]);
    let pitch_axis: V3 = [h[2], 0.0, -h[0]];
    let fall = if !quiet && mv_on(g, MvOnK::Fall) { crate::math_dice::sim_random(g) } else { 0.6 };   // (the one extra draw only with the switch on)
    use_shove(g, id, rag, &rg, quiet, fall, f, h, j, twist, pitch_axis);
    clear_fall(g, id);
    // what the mesh drive needs from the moment of death
    let rig = g.ch[id].rig;
    let mut fresh = crate::anim_skeleton::pose_new(g.tab.rig[rig].nodes.len() as f64);
    crate::anim_skeleton::pose_copy(&mut fresh, &g.ch[id].anim.pose);
    let pose = crate::anim_skeleton::skeleton_instance_bake_local_rotations(&mut g.ch[id].skel, &mut fresh, &g.tab.rig[rig]);
    let limbs = limb_rows(&n);
    let mut rot0: Vec<Quat> = Vec::with_capacity(limbs.len());
    let mut dir0: Vec<V3> = Vec::with_capacity(limbs.len());
    for &(node, _, a, c) in limbs.iter() {
        rot0.push(v3::quat::mul(y_q, g.ch[id].skel.global_rot[node]));
        dir0.push(v3::normalize(v3::sub(pts[c], pts[a])));
    }
    let nodes = g.tab.rig[rig].nodes.len();
    let mut ov: Vec<Option<crate::anim_skeleton::NodeOverride>> = vec![None; nodes];
    ov[n.hips] = Some(crate::anim_skeleton::NodeOverride { rot: [0.0, 0.0, 0.0, 1.0], pos: Some([0.0, 0.0, 0.0]) });
    let limb_ov: Vec<usize> = limbs.iter().map(|(node, _, _, _)| { ov[*node] = Some(crate::anim_skeleton::NodeOverride { rot: [0.0, 0.0, 0.0, 1.0], pos: None }); *node }).collect();
    let hips_rot0 = v3::quat::mul(y_q, g.ch[id].skel.global_rot[n.hips]);
    let delta: Vec<Quat> = limbs.iter().map(|_| v3::quat::ident()).collect();
    g.ch[id].ragdoll = Some(rag);
    g.ch[id].rd = Some(RagDrive { pose, ov, hips: n.hips, hips_rot0, rot0, dir0, delta, limb_ov, synced: false });
}
/// fallAs, fallFrom, fallTight and fallAhead are read once and cleared
fn clear_fall(g: &mut Game, id: CharId) {
    let c = &mut g.ch[id];
    c.fall_as = None; c.fall_from = None; c.fall_tight = false; c.fall_ahead = 0.0;
}
/// the branches of the shove (ts:4645 to 4649)
fn use_shove(g: &mut Game, id: CharId, rag: RagdollId, rg: &crate::ragdoll::RAG, quiet: bool, fall: f64, f: V3, h: V3, j: f64, twist: f64, pa: V3) {
    if !quiet && fall < 0.5 {
        crate::ragdoll::ragdoll_add_rigid_velocity(g, rag, v3::mad(v3::scale(h, 0.8 * j), [0.0, -1.0, 0.0], 0.9), [pa[0] * 0.9 * j, twist * 0.6, pa[2] * 0.9 * j]);
        crate::ragdoll::ragdoll_add_velocity(g, rag, rg.knee_l, f, Some(2.6)); crate::ragdoll::ragdoll_add_velocity(g, rag, rg.knee_r, f, Some(2.2)); crate::ragdoll::ragdoll_add_velocity(g, rag, rg.pelvis, [0.0, -1.0, 0.0], Some(1.2));
    } else if !quiet && fall >= 0.8 {
        let sp = (if twist >= 0.0 { 1.0 } else { -1.0 }) * (2.6 + 1.2 * j);
        crate::ragdoll::ragdoll_add_rigid_velocity(g, rag, v3::scale(h, 1.5 * j), [pa[0] * 1.5 * j, sp, pa[2] * 1.5 * j]);
        crate::ragdoll::ragdoll_add_velocity(g, rag, rg.knee_l, f, Some(1.2)); crate::ragdoll::ragdoll_add_velocity(g, rag, rg.knee_r, f, Some(1.2));
    } else if !quiet {
        crate::ragdoll::ragdoll_add_rigid_velocity(g, rag, v3::scale(h, 1.9 * j), [pa[0] * 2.2 * j, twist, pa[2] * 2.2 * j]);
        crate::ragdoll::ragdoll_add_velocity(g, rag, rg.knee_l, f, Some(1.2)); crate::ragdoll::ragdoll_add_velocity(g, rag, rg.knee_r, f, Some(1.2));
    } else if let Some(kind) = g.ch[id].fall_as {
        let side = if twist >= 0.0 { 1.0 } else { -1.0 };
        let guide = character_fall_guide(g, id, rag, kind, f, h, j, side);
        g.ch[id].guide = Some(guide);
    } else {
        crate::ragdoll::ragdoll_add_rigid_velocity(g, rag, v3::scale(h, 0.5 * j), [pa[0] * 3.2 * j, twist, pa[2] * 3.2 * j]);
        crate::ragdoll::ragdoll_add_velocity(g, rag, rg.knee_l, h, Some(1.0)); crate::ragdoll::ragdoll_add_velocity(g, rag, rg.knee_r, h, Some(1.0));
    }
}

/// Dead: step the particles, then pose the skeleton off them. A sleeping body costs nothing. (Character.updateRagdoll)
pub fn character_update_ragdoll(g: &mut crate::game::Game, id: CharId, dt: f64) {
    let rag_id = g.ch[id].ragdoll.unwrap();
    if g.ch[id].guide.is_some() { character_guide_step(g, id, rag_id, dt); }
    crate::ragdoll::ragdoll_step(g, rag_id, dt);
    let sleeping = g.tab.ragdoll[rag_id].sleeping;
    if sleeping && g.ch[id].rd.as_ref().unwrap().synced {
        if let Some(b) = g.ch[id].breathe {
            character_breath_frame(g, id, dt, CharacterBreathFrameK { breath_hz: b.breath_hz, breath_rise: b.breath_rise, breath_in: b.breath_in });
        } else if g.ch[id].br.is_some() { character_breath_end(g, id, true); }
        return;
    }
    if g.ch[id].br.is_some() { character_breath_end(g, id, false); }
    let n = g.ch[id].anim.nodes;
    let limbs = limb_rows(&n);
    {
        let c = &mut g.ch[id];
        let rag = &g.tab.ragdoll[rag_id];
        let rd = c.rd.as_mut().unwrap();
        rd.synced = sleeping;
        let x = &rag.x;
        c.pos = [x[0], 0.0, x[2]];
        let y_q = v3::quat::yaw(c.body_yaw); let yi = v3::quat::conj(y_q);
        c.world = v3::m4::from_trs(c.pos, y_q, [1.0, 1.0, 1.0]);
        let d = rag.rot;
        let hips_rot = v3::quat::mul(yi, v3::quat::mul(d, rd.hips_rot0));
        let hips = rd.ov[rd.hips].as_mut().unwrap();
        hips.rot = hips_rot; hips.pos = Some([0.0, x[1], 0.0]);   // (pelvis − pos is straight up: yaw leaves it alone)
        for r in 0..limbs.len() {
            let (_, parent, a, cc) = limbs[r];
            let dp = if parent < 0.0 { d } else { rd.delta[parent as usize] };
            let d1 = v3::normalize([x[cc * 3] - x[a * 3], x[cc * 3 + 1] - x[a * 3 + 1], x[cc * 3 + 2] - x[a * 3 + 2]]);
            let mm = v3::quat::from_to(rd.dir0[r], v3::quat::rotate(v3::quat::conj(dp), d1));   // swing only, in the parent's frame
            let delta = v3::quat::normalize(v3::quat::mul(dp, mm)); rd.delta[r] = delta;
            let rot = v3::quat::mul(yi, v3::quat::mul(delta, rd.rot0[r]));
            let node = rd.limb_ov[r];
            rd.ov[node].as_mut().unwrap().rot = rot;
        }
    }
    skel_rag_pose(g, id);
    character_read_bones(g, id, dt);
}

/// a particle's place (fallGuide's P)
fn part_at(rag: &crate::ragdoll::Ragdoll, i: f64) -> V3 {
    let k = i as usize;
    [rag.x[k * 3], rag.x[k * 3 + 1], rag.x[k * 3 + 2]]
}
/// `led`: the pelvis over [p0, p1], the turn over [n0, n1]
fn led_at(l: &Led, t: f64) -> (V3, Quat) {
    (v3::lerp3(l.pv, l.p_t, v3::smoothstep(l.p0, l.p1, t)), v3::quat::from_to(l.v0n, v3::normalize(v3::lerp3(l.v0n, l.v_t, v3::smoothstep(l.n0, l.n1, t)))))
}
/// `frame(t)` of a guided fall
fn frame_at(fr: &FallFrame, t: f64) -> (V3, Quat) {
    match fr {
        FallFrame::Fold { kneel, t_a, lay, k62, flat, roll, sd, f, room, l_a, v0n } => {
            if t <= *t_a { return led_at(kneel, t); }
            let u = v3::smoothstep(*t_a, *t_a + *lay, t);
            let (p, _) = led_at(kneel, *t_a);
            let d = v3::normalize(v3::lerp3(*k62, *flat, u));
            ([p[0] + sd[0] * 0.1 * u * *room + f[0] * *l_a * u, v3::lerp(p[1], 0.135, u), p[2] + sd[2] * 0.1 * u * *room + f[2] * *l_a * u], v3::quat::mul(v3::quat::axis_angle(d, *roll * u), v3::quat::from_to(*v0n, d)))
        }
        FallFrame::Cut { sit, beat } => {
            let (p, q) = led_at(sit, t);
            let k = bump(t, 0.0, *beat * 0.8, *beat, *beat + 0.2);
            ([p[0], p[1] - 0.04 * k, p[2]], q)
        }
        FallFrame::Topple { a, r0, k, top, dur, ty, tw } => {
            let u = v3::clamp(t / *dur, 0.0, 1.0);
            let q = v3::quat::mul(v3::quat::axis_angle(*k, *top * u * u), v3::quat::axis_angle([0.0, *ty, 0.0], *tw * v3::smoothstep(0.0, 1.0, u)));
            (v3::add(*a, v3::quat::rotate(q, *r0)), q)
        }
        FallFrame::Plain(l) => led_at(l, t),
    }
}
/// `nod(t)` of a guided fall
fn nod_at(nd: &FallNod, t: f64) -> f64 {
    match nd {
        FallNod::Fold { slow, t_a, lay } => (if *slow { 22.0 } else { 34.0 }) * DEGR * v3::smoothstep(0.0, if *slow { 0.3 } else { 0.07 }, t) * (1.0 - 0.7 * v3::smoothstep(*t_a, *t_a + *lay, t)),
        FallNod::Cut { beat, dur, end } => 24.0 * DEGR * bump(t, 0.02, *beat, *beat + 0.3 * *dur, *end),
        FallNod::Topple { sign } => -16.0 * DEGR * v3::smoothstep(0.0, 0.06, t) * *sign,
        FallNod::Plain => 24.0 * DEGR * v3::smoothstep(0.0, 0.2, t),
    }
}
/// `q.at(t)` of a pin. A Lead keeps the place it first read (`a ??= P(pi)`)
fn pin_at(rag: &crate::ragdoll::Ragdoll, fr: &FallFrame, nd: &FallNod, at: &mut GuidePinAt, t: f64) -> V3 {
    match at {
        GuidePinAt::Rigid { r } => { let (p, q) = frame_at(fr, t); v3::add(p, v3::quat::rotate(q, *r)) }
        GuidePinAt::Head { v0, nod_ax, rh } => {
            let (p, q) = frame_at(fr, t);
            v3::add(v3::add(p, v3::quat::rotate(q, *v0)), v3::quat::rotate(q, v3::quat::rotate(v3::quat::axis_angle(*nod_ax, nod_at(nd, t)), *rh)))
        }
        GuidePinAt::Lead { pi, a, e, t_a, lay } => {
            let from = match *a { Some(v) => v, None => { let v = part_at(rag, *pi as f64); *a = Some(v); v } };
            v3::lerp3(from, *e, v3::smoothstep(*t_a, *t_a + *lay, t))
        }
        GuidePinAt::Throat { v0, f, lf } => {
            let (p, q) = frame_at(fr, t);
            let n = v3::add(p, v3::quat::rotate(q, *v0));
            [n[0] + f[0] * 0.1 + lf[0] * 0.05, n[1] - 0.02, n[2] + f[2] * 0.1 + lf[2] * 0.05]
        }
        GuidePinAt::Heel { a, r, ty, tw, dur } => v3::add(*a, v3::quat::rotate(v3::quat::axis_angle([0.0, *ty, 0.0], *tw * v3::smoothstep(0.0, 1.0, v3::clamp(t / *dur, 0.0, 1.0))), *r)),
    }
}
/// `line`: the trunk's line, leant `deg` from upright toward d
fn lean_line(d: V3, deg: f64) -> V3 { [d[0] * m::sin(deg * DEGR), m::cos(deg * DEGR), d[2] * m::sin(deg * DEGR)] }

/// A takedown's fall as a path for the trunk: a frame, and each particle its place in it. (Character.fallGuide)
pub fn character_fall_guide(g: &mut crate::game::Game, id: CharId, rag: RagdollId, kind: crate::character::FallStyle, f: V3, h: V3, j: f64, side: f64) -> crate::character::FallGuide {
    let rg = crate::ragdoll::rag();
    let from = g.ch[id].fall_from;
    let room = if g.ch[id].fall_tight { 0.2 } else { 1.0 };
    let ahead = if kind == FallStyle::Sink { g.ch[id].fall_ahead } else { 0.0 };
    let mut gd = fall_guide_of(&g.tab.ragdoll[rag], &rg, kind, f, h, j, side, from, room, ahead);
    let (frame, nod) = (gd.frame, gd.nod);
    for q in gd.pins.iter_mut() {
        if q.t0 <= 0.0 {
            let p = pin_at(&g.tab.ragdoll[rag], &frame, &nod, &mut q.at, 0.0);
            crate::ragdoll::ragdoll_set_pin(g, rag, q.i, p);
        }
    }
    gd
}
/// the fall's path and pins, from the numbers its body read off him (fallGuide's body up to the pins' first places)
fn fall_guide_of(rag: &crate::ragdoll::Ragdoll, rg: &crate::ragdoll::RAG, kind: crate::character::FallStyle, f: V3, h: V3, j: f64, side: f64, from: Option<V3>, room: f64, ahead: f64) -> crate::character::FallGuide {
    let pv = part_at(rag, rg.pelvis); let nk = part_at(rag, rg.neck);
    let v0 = v3::sub(nk, pv); let v0n = v3::normalize(v0);
    let away: V3 = match from {
        Some(fr) if m::hyp2(pv[0] - fr[0], pv[2] - fr[2]) > 0.05 => v3::normalize([pv[0] - fr[0], 0.0, pv[2] - fr[2]]),
        _ => h,
    };
    let sd: V3 = [f[2] * side, 0.0, -f[0] * side]; let nod_ax: V3 = [f[2], 0.0, -f[0]];   // (up × f: a turn about it takes the top of a thing toward his front)
    let led = |p_t: V3, v_t: V3, p0: f64, p1: f64, n0: f64, n1: f64| Led { pv, p_t, v_t, p0, p1, n0, n1, v0n };
    let mut extra: Vec<GuidePin> = Vec::new();
    let end: f64; let frame: FallFrame; let nod: FallNod; let release: Option<FallRelease>;
    match kind {
        FallStyle::Fold | FallStyle::Sink => {
            // onto his knees with the trunk coming forward over them, then down on his front, turned a little onto one shoulder
            let slow = kind == FallStyle::Sink;
            let hold = if slow { 0.05 } else { 0.0 };
            let dur = (if slow { 0.56 } else { 0.5 }) * j;
            let lay = if slow { 0.45 } else { 0.4 };
            let t_a = hold + dur;
            end = t_a + lay + 0.05;
            let k_a = ahead * 0.4; let l_a = ahead - k_a;   // (HOLD.layAhead: two fifths of it as he goes to his knees, the rest as he is laid flat)
            let kneel = led([pv[0] + f[0] * (0.05 * room + k_a), 0.36, pv[2] + f[2] * (0.05 * room + k_a)], lean_line(f, 62.0), hold, hold + dur * (if slow { 0.85 } else { 0.8 }), hold + dur * (if slow { 0.2 } else { 0.1 }), t_a);
            let roll = 22.0 * DEGR * side; let flat = lean_line(f, 88.0); let k62 = lean_line(f, 62.0);
            frame = FallFrame::Fold { kneel, t_a, lay, k62, flat, roll, sd, f, room, l_a, v0n };
            nod = FallNod::Fold { slow, t_a, lay };
            release = None;
            if room == 1.0 {
                // his limbs laid with him: the legs trailing together and straight, the hands up by his head
                let (p_e, q_e) = frame_at(&frame, t_a + lay);
                let at = |rag: &crate::ragdoll::Ragdoll, i: f64| -> V3 { v3::add(p_e, v3::quat::rotate(q_e, v3::sub(part_at(rag, i), pv))) };
                let mut lead = |pi: f64, e: V3| extra.push(GuidePin { i: pi, t0: t_a, off: t_a + lay + 0.05, at: GuidePinAt::Lead { pi: pi as usize, a: None, e, t_a, lay } });
                for (hip_i, knee_i, ank_i, sho_i, hand_i) in [(rg.hip_l, rg.knee_l, rg.ank_l, rg.sho_l, rg.hand_l), (rg.hip_r, rg.knee_r, rg.ank_r, rg.sho_r, rg.hand_r)] {
                    let hip = at(rag, hip_i); let sho = at(rag, sho_i);
                    let th = v3::dist(part_at(rag, hip_i), part_at(rag, knee_i)); let sh = v3::dist(part_at(rag, knee_i), part_at(rag, ank_i));
                    let o: V3 = v3::normalize([hip[0] - p_e[0], 0.0, hip[2] - p_e[2]]);
                    let r = m::sqrt(m::max(0.01, th * th - m::pow(hip[1] - 0.07, 2.0)));
                    lead(knee_i, [hip[0] - f[0] * r + o[0] * 0.02, 0.07, hip[2] - f[2] * r + o[2] * 0.02]);
                    lead(ank_i, [hip[0] - f[0] * (r + sh) + o[0] * 0.04, 0.06, hip[2] - f[2] * (r + sh) + o[2] * 0.04]);
                    lead(hand_i, [sho[0] + f[0] * 0.2 + o[0] * 0.12, 0.05, sho[2] + f[2] * 0.2 + o[2] * 0.12]);
                }
            }
        }
        FallStyle::Cut => {
            let beat = 0.28; let dur = 0.6 * j;
            end = beat + dur;
            let sit = led([pv[0] + away[0] * 0.24 * room, 0.27, pv[2] + away[2] * 0.24 * room], lean_line(away, 24.0), beat, end - 0.05, beat + 0.1 * dur, end);
            frame = FallFrame::Cut { sit, beat };
            nod = FallNod::Cut { beat, dur, end };
            for (i, sg) in [(rg.hand_l, 1.0), (rg.hand_r, -1.0)] {
                // both hands to the throat for the beat he stands, and on down with him
                let lf: V3 = [f[2] * sg, 0.0, -f[0] * sg];
                extra.push(GuidePin { i, t0: 0.03, off: beat + 0.55 * dur, at: GuidePinAt::Throat { v0, f, lf } });
            }
            release = Some(FallRelease::Cut { away, room });
        }
        FallStyle::Topple => {
            let a_l = part_at(rag, rg.ank_l); let a_r = part_at(rag, rg.ank_r);
            let a: V3 = [(a_l[0] + a_r[0]) / 2.0, 0.0, (a_l[2] + a_r[2]) / 2.0];
            let k: V3 = [away[2], 0.0, -away[0]];
            let dur = 0.34 * j; let top = 42.0 * DEGR; let r0 = v3::sub(pv, a);
            end = dur;
            let nf: V3 = [-f[0], 0.0, -f[2]];
            let tc = v3::clamp(v3::dot(nf, away), -1.0, 1.0);
            let tw = m::min(m::acos(tc), 100.0 * DEGR);
            let ty = if nf[2] * away[0] - nf[0] * away[2] >= 0.0 { 1.0 } else { -1.0 };
            frame = FallFrame::Topple { a, r0, k, top, dur, ty, tw };
            if tw > 0.2 {
                for (i, a0) in [(rg.ank_l, a_l), (rg.ank_r, a_r)] {
                    // his heels turning under him with it, or the legs end up crossed
                    let r = v3::sub(a0, a);
                    extra.push(GuidePin { i, t0: 0.0, off: dur, at: GuidePinAt::Heel { a, r, ty, tw, dur } });
                }
            }
            nod = FallNod::Topple { sign: if v3::dot(away, f) < 0.0 { 1.0 } else { -1.0 } };
            let w = 2.0 * top / dur * 0.8;
            release = Some(FallRelease::Topple { away, room, j, k, w });
        }
        FallStyle::Slump => {
            let dur = 0.6 * j;
            end = dur;
            frame = FallFrame::Plain(led(pv, lean_line(f, 58.0 * (0.5 + 0.5 * room)), 0.0, 1.0, 0.0, dur));
            nod = FallNod::Plain;
            release = Some(FallRelease::Plain { sd });
        }
    }
    let mut pins: Vec<GuidePin> = [rg.pelvis, rg.spine, rg.neck, rg.sho_l, rg.sho_r, rg.hip_l, rg.hip_r].iter().map(|&i| {
        let r = v3::sub(part_at(rag, i), pv);
        GuidePin { i, t0: 0.0, off: end, at: GuidePinAt::Rigid { r } }
    }).collect();
    let rh = v3::sub(part_at(rag, rg.head), nk);
    pins.push(GuidePin { i: rg.head, t0: 0.0, off: end, at: GuidePinAt::Head { v0, nod_ax, rh } });
    for e in extra { pins.push(e); }
    FallGuide { t: 0.0, end, pins, release, frame, nod }
}
/// A led fall's frame: its pins moved along their paths; let go at the end with the fall's own shove, or at once with none when anything else takes hold of the body. (Character.guideStep)
pub fn character_guide_step(g: &mut crate::game::Game, id: CharId, rag: RagdollId, dt: f64) {
    let mut gd = g.ch[id].guide.take().unwrap();
    let t0 = gd.t; gd.t += dt;
    let mut taken = false;
    let mut i = 0usize;
    while i < 16 && !taken {
        if crate::ragdoll::ragdoll_is_pinned(g, rag, i as f64) {
            let mut mine = false;
            for q in gd.pins.iter() { if q.i == i as f64 && t0 >= q.t0 && t0 < q.off { mine = true; break; } }
            taken = !mine;
        }
        i += 1;
    }
    let done = gd.t >= gd.end;
    let t = gd.t;
    let (frame, nod) = (gd.frame, gd.nod);
    for q in gd.pins.iter_mut() {
        if taken || done || t >= q.off {
            if t0 >= q.t0 && t0 < q.off { crate::ragdoll::ragdoll_unpin(g, rag, q.i); }
        } else if t >= q.t0 {
            let p = pin_at(&g.tab.ragdoll[rag], &frame, &nod, &mut q.at, t);
            crate::ragdoll::ragdoll_set_pin(g, rag, q.i, p);
        }
    }
    if taken || done {
        if done && !taken { if let Some(r) = gd.release { fall_release(g, rag, r); } }
    } else { g.ch[id].guide = Some(gd); }
}
/// `G.release(rag)`: the fall's own shove as it is let go
fn fall_release(g: &mut Game, r: RagdollId, rel: FallRelease) {
    let rg = crate::ragdoll::rag();
    match rel {
        FallRelease::Cut { away, room } => {
            crate::ragdoll::ragdoll_add_rigid_velocity(g, r, v3::scale(away, 0.5 * room), [away[2] * 2.2, 0.0, -away[0] * 2.2]);
            crate::ragdoll::ragdoll_add_velocity(g, r, rg.head, away, Some(0.4));
        }
        FallRelease::Topple { away, room, j, k, w } => {
            crate::ragdoll::ragdoll_add_rigid_velocity(g, r, v3::mad(v3::scale(away, (0.5 + 0.4 * room) * j), [0.0, -1.0, 0.0], 1.0), [k[0] * w, 0.0, k[2] * w]);
            crate::ragdoll::ragdoll_add_velocity(g, r, rg.head, away, Some(0.4));
        }
        FallRelease::Plain { sd } => crate::ragdoll::ragdoll_add_velocity(g, r, rg.head, sd, Some(0.3)),
    }
}

/// A man out cold breathes: the skeleton alone posed again, fifteen times a second. (Character.breathFrame)
pub fn character_breath_frame(g: &mut crate::game::Game, id: CharId, dt: f64, k: crate::character::CharacterBreathFrameK) {
    let nd = g.ch[id].anim.nodes;
    if g.ch[id].br.is_none() {
        let c = &g.ch[id];
        let sk = &c.skel;
        let rd = c.rd.as_ref().unwrap();
        let hold = [nd.upper_arm_l, nd.upper_arm_r, nd.neck];
        let hold_ov: Vec<crate::anim_skeleton::NodeOverride> = hold.iter().map(|&i| crate::anim_skeleton::NodeOverride { rot: sk.global_rot[i], pos: Some(v3::m4::get_translation(&sk.globals[i])) }).collect();
        let mut saved: Vec<Option<crate::anim_skeleton::NodeOverride>> = vec![rd.ov[nd.spine2]];
        for &i in hold.iter() { saved.push(rd.ov[i]); }
        let b = Breath { t: 0.0, due: 0.0, chest: nd.spine2 as f64, at: v3::m4::get_translation(&sk.globals[nd.spine2]), rot: sk.global_rot[nd.spine2], hold: hold.iter().map(|&i| i as f64).collect(), hold_ov, saved };
        g.ch[id].br = Some(b);
    }
    let me = g.ch[id].id;
    {
        let c = &mut g.ch[id];
        let b = c.br.as_mut().unwrap();
        b.t += dt; b.due -= dt;
        if b.due > 0.0 { return; }
        b.due = 1.0 / 15.0;
        let a = k.breath_rise * v3::smoothstep(0.0, m::max(k.breath_in, 1e-3), b.t) * (0.5 - 0.5 * m::cos(m::PI * 2.0 * k.breath_hz * b.t + me * 1.7));
        let rd = c.rd.as_mut().unwrap();
        rd.ov[b.chest as usize] = Some(crate::anim_skeleton::NodeOverride { rot: b.rot, pos: Some([b.at[0], b.at[1] + a, b.at[2]]) });
        for i in 0..b.hold.len() { rd.ov[b.hold[i] as usize] = Some(b.hold_ov[i]); }
    }
    skel_rag_pose(g, id);
}
/// …and stops: the drive's own overrides back; `pose` = the skeleton put back as it slept. (Character.breathEnd)
pub fn character_breath_end(g: &mut crate::game::Game, id: CharId, pose: bool) {
    {
        let c = &mut g.ch[id];
        let b = c.br.take().unwrap();
        let rd = c.rd.as_mut().unwrap();
        rd.ov[b.chest as usize] = b.saved[0];
        for i in 0..b.hold.len() { rd.ov[b.hold[i] as usize] = b.saved[i + 1]; }
    }
    if pose { skel_rag_pose(g, id); }
}

// ---- what the rules ask of the body ----

/// Unit vector 'down' out of the underside of the gun. (Character.gunUnder)
pub fn character_gun_under(g: &mut crate::game::Game, id: CharId) -> V3 {
    let v = g.ch[id].gun_dir; let gu = v[1];   // g × (g × up) = (g·up) g − up
    v3::normalize([v[0] * gu, v[1] * gu - 1.0, v[2] * gu])
}
/// Head rotation (world), from rest-pose model vectors. (Character.headRot)
pub fn character_head_rot(g: &mut crate::game::Game, id: CharId) -> Quat {
    let c = &g.ch[id];
    v3::quat::mul(v3::quat::yaw(c.body_yaw), v3::quat::mul(c.skel.global_rot[c.anim.nodes.head], c.rest_head_inv))
}
/// Head look direction (world). (Character.headDir)
pub fn character_head_dir(g: &mut crate::game::Game, id: CharId) -> V3 {
    let r = character_head_rot(g, id);
    v3::normalize(v3::quat::rotate(r, [0.0, 0.0, 1.0]))
}
/// Capsule proxies in world space and a bounding sphere; devices test laser trips on these. (Character.capsules)
pub fn character_capsules(g: &mut crate::game::Game, id: CharId) -> crate::character::CharacterCapsules {
    let hr = character_head_rot(g, id);
    let c = &g.ch[id];
    let b = |k: Bone| -> V3 { *c.bones.get(k).unwrap() };
    let cap = |a: V3, z: V3, r: f64| cap_rec(a, z, r, None);
    // shoulders: widen the upper torso capsule sideways by using the shoulder line
    let shoulder_mid: V3 = v3::lerp3(b(Bone::UpperArmL), b(Bone::UpperArmR), 0.5);
    let caps: Vec<Vec<f64>> = vec![
        cap(b(Bone::Hips), b(Bone::Spine2), 0.15),
        cap(b(Bone::Spine2), shoulder_mid, 0.16),
        cap(b(Bone::UpperArmL), b(Bone::UpperArmR), 0.07),
        cap(b(Bone::ThighL), b(Bone::ThighR), 0.11),
        cap(b(Bone::Neck), v3::mad(b(Bone::Head), v3::quat::rotate(hr, [0.0, 1.0, 0.0]), 0.13), 0.105),
        cap(b(Bone::UpperArmL), b(Bone::ForearmL), 0.052), cap_rec(b(Bone::ForearmL), b(Bone::HandL), 0.042, Some(0.09)),
        cap(b(Bone::UpperArmR), b(Bone::ForearmR), 0.052), cap_rec(b(Bone::ForearmR), b(Bone::HandR), 0.042, Some(0.09)),
        cap(b(Bone::ThighL), b(Bone::ShinL), 0.075), cap(b(Bone::ShinL), b(Bone::FootL), 0.058), cap_sole(b(Bone::FootL), b(Bone::ToeL)),
        cap(b(Bone::ThighR), b(Bone::ShinR), 0.075), cap(b(Bone::ShinR), b(Bone::FootR), 0.058), cap_sole(b(Bone::FootR), b(Bone::ToeR)),
    ];
    let o = v3::lerp3(b(Bone::Hips), b(Bone::Spine2), 0.5);
    CharacterCapsules { caps, bound: [o[0], o[1], o[2], 1.5] }
}
/// A point inside the body: the hips' column soles to crown, or the chest's when the trunk leans off it. (Character.insideBody)
pub fn character_inside_body(g: &mut crate::game::Game, id: CharId, p: V3, crown_pad: Option<f64>) -> bool {
    let crown_pad = crown_pad.unwrap_or(0.0);
    let c = &g.ch[id];
    if !c.alive { return false; }
    let top = (match c.bones.get(Bone::HeadTop) { Some(v) => v[1], None => c.pos[1] + 1.75 }) + crown_pad;
    let bot = c.pos[1] + 0.05;
    let hips = c.bones.get(Bone::Hips).copied(); let chest = c.bones.get(Bone::Spine2).copied();
    let cx = match hips { Some(v) => v[0], None => c.pos[0] }; let cz = match hips { Some(v) => v[2], None => c.pos[2] };
    if p[1] > bot && p[1] < top && m::hyp2(p[0] - cx, p[2] - cz) < 0.28 { return true; }
    if let (Some(chest), Some(hips)) = (chest, hips) {
        if m::hyp2(chest[0] - hips[0], chest[2] - hips[2]) > 0.12 {
            let cb = m::min(hips[1], chest[1]) - 0.12;
            if p[1] > cb && p[1] < top && m::hyp2(p[0] - chest[0], p[2] - chest[2]) < 0.25 { return true; }
        }
    }
    false
}
/// Ray test against the body's columns: t or -1. (Character.rayHit)
pub fn character_ray_hit(g: &mut crate::game::Game, id: CharId, ro: V3, rd: V3, tmax: f64) -> f64 {
    let c = &g.ch[id];
    if !c.alive { return -1.0; }
    let top = match c.bones.get(Bone::HeadTop) { Some(v) => v[1], None => c.pos[1] + 1.75 };
    let bot = c.pos[1] + 0.05;
    let hips = c.bones.get(Bone::Hips).copied(); let chest = c.bones.get(Bone::Spine2).copied();
    let cx = match hips { Some(v) => v[0], None => c.pos[0] }; let cz = match hips { Some(v) => v[2], None => c.pos[2] };
    let mut t = column_entry(ro, rd, tmax, cx, cz, 0.28, bot, top);
    if let (Some(chest), Some(hips)) = (chest, hips) {
        if m::hyp2(chest[0] - hips[0], chest[2] - hips[2]) > 0.12 {
            let tc = column_entry(ro, rd, tmax, chest[0], chest[2], 0.25, m::min(hips[1], chest[1]) - 0.12, top);
            if tc >= 0.0 && (t < 0.0 || tc < t) { t = tc; }
        }
    }
    t
}

#[cfg(test)]
mod tests {
    use super::*;
    // the bits of the TypeScript's own Character.prototype.fallGuide, on a still body: x[i] = (0.1 + i·0.03, 0.2 + i·0.1, −0.3 + i·0.017), run under bun with EXACT.on
    fn guide(kind: FallStyle, ahead: f64) -> (crate::ragdoll::Ragdoll, FallGuide) {
        let mut rag = crate::ragdoll::Ragdoll::blank();
        rag.x = vec![0.0; 48];
        for i in 0..16 { let n = i as f64; rag.x[i * 3] = 0.1 + n * 0.03; rag.x[i * 3 + 1] = 0.2 + n * 0.1; rag.x[i * 3 + 2] = -0.3 + n * 0.017; }
        let rg = crate::ragdoll::RAG { pelvis: 0.0, spine: 1.0, neck: 2.0, head: 3.0, sho_l: 4.0, sho_r: 5.0, elb_l: 6.0, elb_r: 7.0, hand_l: 8.0, hand_r: 9.0, hip_l: 10.0, hip_r: 11.0, knee_l: 12.0, knee_r: 13.0, ank_l: 14.0, ank_r: 15.0 };
        let gd = fall_guide_of(&rag, &rg, kind, [0.6, 0.0, 0.8], [-0.8, 0.0, 0.6], 0.97, -1.0, None, 1.0, ahead);
        (rag, gd)
    }
    fn check(kind: FallStyle, ahead: f64, end: u64, n: usize, rows: &[(f64, usize, f64, [u64; 3])]) {
        let (rag, mut gd) = guide(kind, ahead);
        assert_eq!(gd.end.to_bits(), end);
        assert_eq!(gd.pins.len(), n);
        let (fr, nd) = (gd.frame, gd.nod);
        for (t, k, i, want) in rows {
            let q = &mut gd.pins[*k];
            assert_eq!(q.i, *i);
            let p = pin_at(&rag, &fr, &nd, &mut q.at, *t);
            assert_eq!([p[0].to_bits(), p[1].to_bits(), p[2].to_bits()], *want, "t {} pin {}", t, k);
        }
    }
    #[test]
    fn topple_as_the_typescript() {
        check(FallStyle::Topple, 0.0, 0x3fd51b71758e2197, 10, &[
            (0.1, 0, 0.0, [0x3fa069fc88a1a470, 0x3fc694ae6d7f97f9, 0xbfc0b16e27990d6a]),
            (0.1, 7, 3.0, [0x3fc1f86ed349ba17, 0x3fddfcf0946fa087, 0xbfb6342864caa890]),
            (0.1, 9, 15.0, [0x3fe1a9ecbccef1bb, 0x3ffb333333333333, 0xbfa9e64ef3aae021]),
            (0.25, 0, 0.0, [0x3fc50752f6f23788, 0xbf930ad212da6ae4, 0x3fd58b948ff6735a]),
            (0.25, 7, 3.0, [0x3fc3f092e6db6b0e, 0x3fd2ee3942ee693a, 0x3fd4f3cbb402f3e2]),
            (0.9, 9, 15.0, [0x3fe1645a1cac0830, 0x3ffb333333333333, 0xbfb189374bc6a7ef]),
        ]);
    }
    #[test]
    fn sink_as_the_typescript() {
        check(FallStyle::Sink, 0.5, 0x3ff17dbf487fcb93, 14, &[
            (0.1, 0, 0.0, [0x3fbada78edfa4c3b, 0x3fca44bb60339278, 0xbfd2c83e1712f7a8]),
            (0.25, 7, 3.0, [0x3fd1f4b75576894c, 0x3fe12476d049d44e, 0xbfbf2c7f360fb80e]),
            (0.5, 13, 9.0, [0x3fd7ae147ae147ae, 0x3ff199999999999a, 0xbfc2d0e560418936]),
            (0.9, 0, 0.0, [0x3fd4de4142e8de56, 0x3fc82c55342fc593, 0x3fc068bac4a86935]),
            (0.9, 7, 3.0, [0x3fe04fb157ea554b, 0x3fcbac743e3fa3c8, 0x3fd895f79331efd1]),
            (0.9, 13, 9.0, [0x3fe7bf02e25cfd74, 0x3fd348b927d947e4, 0x3fe444284589586c]),
        ]);
    }
    #[test]
    fn slump_as_the_typescript() {
        check(FallStyle::Slump, 0.0, 0x3fe29fbe76c8b439, 8, &[
            (0.1, 0, 0.0, [0x3fb999999999999a, 0x3fc999999999999a, 0xbfd3333333333333]),
            (0.1, 7, 3.0, [0x3fcac1490664cc02, 0x3fdf0d003fb8e444, 0xbfcc0dfc19d04358]),
            (0.5, 7, 3.0, [0x3fd132e2b382f6be, 0x3fd5937e51984e14, 0xbfb3c5c791d4cdc5]),
            (0.9, 7, 3.0, [0x3fd143b7034bc990, 0x3fd4f12061d607ed, 0xbfb27b50b645cee7]),
        ]);
    }
}
