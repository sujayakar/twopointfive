use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};


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
fn node_pos(g: &Game, id: CharId, node: usize) -> V3 { crate::anim_skeleton::skeleton_instance_node_pos(&g.ch[id].skel, node as f64) }
/// skel.update(pose, world, twists?, ov), the skeleton and the twists out of `g` while `g` is lent: the update reads neither through g
fn skel_update(g: &mut Game, id: CharId, pose: crate::anim_skeleton::Pose, world: &crate::v3::m4::Mat4, twisted: bool, ov: Option<&[Option<crate::anim_skeleton::NodeOverride>]>) {
    let mut sk = core::mem::replace(&mut g.ch[id].skel, crate::anim_skeleton::SkeletonInstance::blank());
    if twisted {
        let tw = core::mem::take(&mut g.ch[id].anim.twists);
        crate::anim_skeleton::skeleton_instance_update(g, &mut sk, pose, world, Some(&tw), ov);
        g.ch[id].anim.twists = tw;
    } else {
        crate::anim_skeleton::skeleton_instance_update(g, &mut sk, pose, world, None, ov);
    }
    g.ch[id].skel = sk;
}
/// skel.update(pose, world, anim.twists)
fn skel_pose_twisted(g: &mut Game, id: CharId, pose: crate::anim_skeleton::Pose) {
    let world = g.ch[id].world;
    skel_update(g, id, pose, &world, true, None);
}
/// skel.update(anim.pose, world, anim.twists, ov)
fn skel_anim_pose(g: &mut Game, id: CharId, ov: Option<&[Option<crate::anim_skeleton::NodeOverride>]>) {
    let (pose, world) = (g.ch[id].anim.pose, g.ch[id].world);
    skel_update(g, id, pose, &world, true, ov);
}
/// skel.update(rd.pose, world, null, rd.ov)
fn skel_rag_pose(g: &mut Game, id: CharId) {
    let world = g.ch[id].world;
    let rd = g.ch[id].rd.take().unwrap();
    skel_update(g, id, rd.pose, &world, false, Some(&rd.ov));
    g.ch[id].rd = Some(rd);
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
// ---- construction and the plain frames ----

/// The man of the floor behind this body; a player's is none.
pub fn guard_of(g: &crate::game::Game, c: CharId) -> Option<GuardId> {
    match g.ch[c].who { crate::character::Who::Guard(gd) => Some(gd), crate::character::Who::Player(_) => None }
}

/// Is a player's hand on this man.
pub fn is_player(g: &crate::game::Game, c: CharId) -> bool { matches!(g.ch[c].who, crate::character::Who::Player(_)) }

/// A man stands: his body, and a row of each part a man has.
pub fn man_new(g: &mut crate::game::Game, body: crate::character::Character) -> CharId {
    let c = g.ch.push(body);
    g.cond.push(crate::condition::Cond::blank());
    g.load.push(None);
    g.shown.push(crate::guards::Shown::default());
    g.arms.push(crate::aim::Arms::blank());
    c
}

/// A body: his columns, his animator, his skeleton, and the rest pose's frames. `who` stays blank: the owner writes it. (Character.constructor)
pub fn character_new(g: &mut crate::game::Game, ch: RigId, d: crate::character::CharacterDesc) -> CharId {
    let id = crate::character::man_new(g, Character::blank());
    if d.men.is_some() { crate::sim_men_store::stand_man(g, id); } // standMan(this, d.men ?? loneMen()): a man of no game has no row of the block (ts:4356)
    g.ch[id].rig = ch;
    g.ch[id].id = d.id; g.ch[id].tint = d.tint; g.ch[id].tint2 = d.tint2; g.ch[id].pos = v3::copy(d.pos); g.ch[id].body_yaw = d.yaw; g.ch[id].aim_yaw = d.yaw;
    let anim = anim_new(g, ch);
    g.ch[id].anim = anim;
    anim_desync(g, id, d.id);
    let sk = crate::anim_skeleton::skeleton_instance_new(g, ch);
    g.ch[id].skel = sk;
    // rest-pose chest rotation for "facing" extraction
    let nodes = g.tab.rig[ch].nodes.len();
    let rp = crate::anim_skeleton::pose_new(g, nodes);
    crate::anim_skeleton::rest_pose(g, ch, rp);
    let origin = v3::m4::create();
    skel_update(g, id, rp, &origin, false, None);
    crate::anim_skeleton::pose_done(g, rp);   // (the scratch is dropped)
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
pub fn character_forward(g: &crate::game::Game, id: CharId) -> V3 {
    let y = g.ch[id].body_yaw;
    [m::sin(y), 0.0, m::cos(y)]
}
/// (Character.aimDir)
fn character_aim_dir(g: &crate::game::Game, id: CharId) -> V3 {
    let (p, y) = (g.ch[id].aim_pitch, g.ch[id].aim_yaw);
    let cp = m::cos(p);
    [m::sin(y) * cp, m::sin(p), m::cos(y) * cp]
}
/// The right hand's frame in the world; `sim`: as the animator posed it, a game rule's. (Character.handFrameR)
pub fn character_hand_frame_r(g: &crate::game::Game, id: CharId, sim: Option<bool>) -> crate::character::CharacterHandFrameR {
    let sim = sim.unwrap_or(false);
    let c = &g.ch[id];
    let n = c.anim.nodes.hand_r;
    let h = match c.sim_hand_r { Some(s) if sim => s, _ => c.skel.global_rot[n] };
    let r = v3::quat::mul(v3::quat::yaw(c.body_yaw), h);
    CharacterHandFrameR { fingers: v3::quat::rotate(r, [0.0, 1.0, 0.0]), back: v3::quat::rotate(r, c.hand_r_back), thumb: v3::quat::rotate(r, c.hand_r_thumb) }
}
/// The frame a gun in the right hand is rigid in, as the hand is drawn. (Character.gripFrameR)
pub fn character_grip_frame_r(g: &crate::game::Game, id: CharId) -> crate::character::CharacterGripFrameR {
    let c = &g.ch[id];
    let r = v3::quat::mul(v3::quat::yaw(c.body_yaw), c.skel.global_rot[c.anim.nodes.hand_r]);
    CharacterGripFrameR { bore: v3::quat::rotate(r, c.hand_r_bore), top: v3::quat::rotate(r, c.hand_r_top) }
}
/// How a gun on a kit errand lies off the right hand, in the world. (Character.kitLieWorld)
pub fn character_kit_lie_world(g: &crate::game::Game, id: CharId) -> Option<Quat> {
    let c = &g.ch[id];
    let d = c.anim.kit_lie?;
    let r = v3::quat::mul(v3::quat::yaw(c.body_yaw), c.skel.global_rot[c.anim.nodes.hand_r]);
    Some(v3::quat::mul(v3::quat::mul(r, d), v3::quat::conj(r)))
}
/// (Character.handFrameL)
pub fn character_hand_frame_l(g: &crate::game::Game, id: CharId) -> crate::character::CharacterHandFrameL {
    let c = &g.ch[id];
    let r = v3::quat::mul(v3::quat::yaw(c.body_yaw), c.skel.global_rot[c.anim.nodes.hand_l]);
    CharacterHandFrameL { fingers: v3::quat::rotate(r, [0.0, 1.0, 0.0]), back: v3::quat::rotate(r, c.hand_l_back), thumb: v3::quat::rotate(r, c.hand_l_thumb) }
}
/// The chest's frame in the world; None before his first update. (Character.chestFrame)
pub fn character_chest_frame(g: &crate::game::Game, id: CharId) -> Option<crate::character::CharacterChestFrame> {
    let c = &g.ch[id];
    let o = *c.bones.get(Bone::Chest)?;
    let r = v3::quat::mul(v3::quat::yaw(c.body_yaw), v3::quat::mul(c.skel.global_rot[c.anim.nodes.spine3], c.rest_chest_inv));
    Some(CharacterChestFrame { o, left: v3::quat::rotate(r, [1.0, 0.0, 0.0]), up: v3::quat::rotate(r, [0.0, 1.0, 0.0]), fwd: v3::quat::rotate(r, [0.0, 0.0, 1.0]) })
}

// ---- the frame ----

/// His frame. A man whose poses are not yet at home (made inside this tick) is lent the stage for it. (Character.update)
pub fn character_update(g: &mut crate::game::Game, id: CharId, dt: f64) {
    let lent = crate::posehome::stage_open(g, id);
    character_update_standing(g, id, dt);
    if let Some(st) = lent { crate::posehome::stage_shut(g, id, st); }
}
/// The turn, the animator, the skeleton and what is read off it. (Character.updateStanding)
fn character_update_standing(g: &mut crate::game::Game, id: CharId, dt: f64) {
    if g.ch[id].ragdoll.is_some() { character_update_ragdoll(g, id, dt); return; }
    if g.ch[id].turn.on && dt > 0.0 { character_turn_weigh(g, id, dt); }
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
        c.anim.pos_in = POS_IN_IS_POS;   // (`this.anim.posIn = this.pos`: the animator holds the body's own array again)
    }
    anim_update(g, id, dt);
    let (pos, by) = (g.ch[id].pos, g.ch[id].body_yaw);
    g.ch[id].world = v3::m4::from_trs(pos, v3::quat::yaw(by), [1.0, 1.0, 1.0]);
    let n = g.ch[id].anim.nodes;
    // the gun hand under the torch ↔ sidearm swap: what the bore and the muzzle are read off
    match g.ch[id].anim.hand_pre {
        Some(hp) => {
            let world = g.ch[id].world;
            skel_pose_twisted(g, id, hp);
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
            skel_pose_twisted(g, id, pre);
            let mut m: JsMap<Bone, V3> = JsMap::new();
            character_joints(g, id, &mut m);
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
fn character_pack_side_pass(g: &mut crate::game::Game, id: CharId) {
    let (n, sd, w, big_clear, big_grab) = { let an = &g.ch[id].anim; (an.nodes, an.big_side.unwrap(), an.big_at_back, an.big_clear, an.big_grab) };
    {
        let b = &g.ch[id].bones;
        if !(b.has(Bone::Hips) && b.has(Bone::Spine2) && b.has(Bone::ThighL) && b.has(Bone::ThighR) && b.has(Bone::UpperArmL) && b.has(Bone::UpperArmR)) { return; }
    }
    let qi = v3::quat::conj(v3::quat::yaw(g.ch[id].body_yaw));
    let old = crate::rig_props::big_back_of(bone(g, id, Bone::Hips), bone(g, id, Bone::Spine2), bone(g, id, Bone::ThighL), bone(g, id, Bone::ThighR), bone(g, id, Bone::UpperArmL), bone(g, id, Bone::UpperArmR), crate::rig_props::BIGRIG.hammer.back, Some(big_clear));
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
fn character_reach_ov(g: &crate::game::Game, id: CharId, ov: &mut Vec<Option<crate::anim_skeleton::NodeOverride>>, right: bool, t: V3, rot: Quat) {
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
fn character_in_hand_pass(g: &mut crate::game::Game, id: CharId, dt: f64) {
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
    character_in_hand_word(g, id, dt, cur, hr, r, l, &mut ov);
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
fn character_in_hand_word(g: &mut crate::game::Game, id: CharId, dt: f64, cur: Option<crate::inhand::InHandWord>, hr: Quat, r: V3, l: V3, ov: &mut Vec<Option<crate::anim_skeleton::NodeOverride>>) {
    let n = g.ch[id].anim.nodes;
    let w = g.ch[id].in_hand;
    let cur = match cur { Some(c) => c, None => { g.ch[id].in_hand_k = 0.0; return; } };
    let is_long = matches!(cur, crate::inhand::InHandWord::Long(_));
    let is_pistol = matches!(cur, crate::inhand::InHandWord::Pistol);
    let is_torch = matches!(cur, crate::inhand::InHandWord::Torch(_));
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
        crate::inhand::InHandWord::Torch(t) => t.beam,
        _ => if is_long || g.ch[id].anim.upper == UpperMode::Aim || anim_shooting(g, id) { g.ch[id].gun_dir } else { character_aim_dir(g, id) },
    };
    let z = v3::normalize(v3::quat::rotate(qi, line));
    let mut y: V3 = [-z[0] * z[1], 1.0 - z[1] * z[1], -z[2] * z[1]];
    if v3::len(y) < 1e-3 { return; }
    y = v3::normalize(y);
    let x = v3::cross(y, z);
    let in_f = |d: V3| -> V3 { [x[0] * d[0] + y[0] * d[1] + z[0] * d[2], x[1] * d[0] + y[1] * d[1] + z[1] * d[2], x[2] * d[0] + y[2] * d[1] + z[2] * d[2]] };
    if let crate::inhand::InHandWord::Long(c) = cur { character_reach_ov(g, id, ov, true, v3::mad(r, in_f(c.by), k), hr); return; }
    let mut up = 1.0;
    let (bore, top, thumb, back) = { let c = &g.ch[id]; (c.hand_r_bore, c.hand_r_top, c.hand_r_thumb, c.hand_r_back) };
    let mut d: Quat = match cur {
        crate::inhand::InHandWord::Torch(c) => {
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
        crate::inhand::InHandWord::Torch(c) => {
            // the hollow of the turned fist onto the point of its body the simulation's wrist and beam put there
            let hol = v3::quat::rotate(rot, v3::mad([0.0, c.fwd, 0.0], back, -c.palm));
            character_reach_ov(g, id, ov, true, v3::mad(r, v3::sub(v3::sub(v3::add(r, in_f(c.at)), hol), r), k), rot);
        }
        _ => {
            ov_set(ov, n.hand_r, crate::anim_skeleton::NodeOverride { rot, pos: None });
            // the support hand, when it is on the gun: carried rigidly round the gun hand's wrist with the same turn
            let both = g.k.inhand.both;
            let near = 1.0 - v3::smoothstep(both * 0.5, both, v3::len(v3::sub(l, r)));
            if near > 0.0 {
                let dl = if near < 1.0 { v3::quat::slerp([0.0, 0.0, 0.0, 1.0], d, near) } else { d };
                let rl = g.ch[id].skel.global_rot[n.hand_l];
                character_reach_ov(g, id, ov, false, v3::add(r, v3::quat::rotate(dl, v3::sub(l, r))), v3::quat::normalize(v3::quat::mul(dl, rl)));
            }
        }
    }
}

// ---- the turn ----

/// His yaws come round to the AI's wish as a mass. (Character.turnWeigh)
fn character_turn_weigh(g: &mut crate::game::Game, id: CharId, dt: f64) { ::sim::layers::turn_weigh(&mut g.ch[id], dt, &g.k); }
/// His AI is about to ease his yaws: they read back as it left them; one written since is the new wish. (Character.turnBegin)
pub fn character_turn_begin(g: &mut crate::game::Game, id: CharId) {
    let c = &mut g.ch[id];
    let t = match c.tn { Some(t) => t, None => return };
    if c.ragdoll.is_some() || !c.turn.on { return; }
    if c.body_yaw == t.body { c.body_yaw = t.ib; }
    if c.aim_yaw == t.aim { c.aim_yaw = t.ia; }
}
/// Where his gun is in yaw, for a fire gate inside his own AI update. (Character.gunYaw)
pub fn character_gun_yaw(g: &crate::game::Game, id: CharId) -> f64 {
    let c = &g.ch[id];
    match c.tn { Some(t) if c.ragdoll.is_none() => t.aim, _ => c.aim_yaw }
}
/// Where his AI wants his body, for an easing outside his own update. (Character.wantBody)
pub fn character_want_body(g: &crate::game::Game, id: CharId) -> f64 {
    let c = &g.ch[id];
    match c.tn { Some(t) if c.body_yaw == t.body => t.ib, _ => c.body_yaw }
}

// ---- joints ----

/// A joint as the game reads it: under the moves' look-only arms while one shows. (Character.jointUnder)
pub fn character_joint_under(g: &crate::game::Game, id: CharId, name: &str) -> Option<V3> {
    let k = bone_of(name)?;
    let c = &g.ch[id];
    if let Some(v) = c.sim_bones.as_ref().and_then(|m| m.get(k)) { return Some(*v); }
    c.bones.get(k).copied()
}
/// The world-space joints off the skeleton's current globals, into `b`. (Character.joints)
fn character_joints(g: &crate::game::Game, id: CharId, b: &mut JsMap<crate::character::Bone, V3>) {
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
/// The joints and what is derived from them: chest facing, gun axis, muzzle. (Character.readBones)
fn character_read_bones(g: &mut crate::game::Game, id: CharId, dt: f64) {
    let n = g.ch[id].anim.nodes;
    if dt > 1e-5 && g.ch[id].bones.has(Bone::Hips) {
        // (a dt-0 re-bake keeps the older sample: it is still one real frame back)
        let c = &mut g.ch[id];
        for k in c.bones.keys() { let v = *c.bones.get(k).unwrap(); c.prev_bones.set(k, v); }
        c.prev_dt = dt;
    }
    let mut b = core::mem::take(&mut g.ch[id].bones);
    character_joints(g, id, &mut b);
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
pub fn character_ragdollize(g: &mut crate::game::Game, id: CharId, world: bool, dir: V3, quiet: bool) {
    if let Some(t) = g.ch[id].tn { g.ch[id].body_yaw = t.body; g.ch[id].aim_yaw = t.aim; g.ch[id].tn = None; }
    if g.ch[id].ragdoll.is_some() { clear_fall(g, id); return; }
    if !g.ch[id].bones.has(Bone::Hips) { character_update(g, id, 0.0); }
    if let Some(under) = anim_under_pose(g, id) {
        let pose = g.ch[id].anim.pose;
        crate::anim_skeleton::pose_copy(g, pose, under);
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
    let rag_bones = rag_bones();
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
    // (ragdoll_new asks only whether he falls in a world; which one is read off g)
    let in_world = crate::ragdoll::RagdollWorld::default();
    let rag = crate::ragdoll::ragdoll_new(g, &pts, vel.as_deref(), if world { Some(&in_world) } else { None });
    // the shove: carried along `dir` and pitched top-first the same way, then the knees kicked out
    let f = character_forward(g, id);
    let j = 0.85 + crate::dice::draw_body(g, "character::character_ragdollize#1", id) * 0.3;
    let twist = (crate::dice::draw_body(g, "character::character_ragdollize#2", id) - 0.5) * (if quiet { 0.8 } else { 1.6 });
    let h = v3::normalize([dir[0], 0.0, dir[2]]);
    let pitch_axis: V3 = [h[2], 0.0, -h[0]];
    let fall = if !quiet && mv_on(g, MvOnK::Fall) { crate::dice::draw_body(g, "character::character_ragdollize#3", id) } else { 0.6 };   // (the one extra draw only with the switch on)
    use_shove(g, id, rag, &rg, quiet, fall, f, h, j, twist, pitch_axis);
    clear_fall(g, id);
    // what the mesh drive needs from the moment of death
    let rig = g.ch[id].rig;
    let nodes = g.tab.rig[rig].nodes.len();
    let pose = crate::anim_skeleton::pose_new(g, nodes);
    let drawn = g.ch[id].anim.pose;
    crate::anim_skeleton::pose_copy(g, pose, drawn);
    let mut sk = core::mem::replace(&mut g.ch[id].skel, crate::anim_skeleton::SkeletonInstance::blank());
    crate::anim_skeleton::skeleton_instance_bake_local_rotations(g, &mut sk, pose);
    g.ch[id].skel = sk;
    let limbs = limb_rows(&n);
    let mut rot0: Vec<Quat> = Vec::with_capacity(limbs.len());
    let mut dir0: Vec<V3> = Vec::with_capacity(limbs.len());
    for &(node, _, a, c) in limbs.iter() {
        rot0.push(v3::quat::mul(y_q, g.ch[id].skel.global_rot[node]));
        dir0.push(v3::normalize(v3::sub(pts[c], pts[a])));
    }
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
fn character_update_ragdoll(g: &mut crate::game::Game, id: CharId, dt: f64) {
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
        // (a new array: an animator handed the old one keeps it. pos_replace's rule, written out for the row in hand)
        if c.anim.pos_in[0].is_nan() { c.anim.pos_in = c.pos; }
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
fn character_fall_guide(g: &mut crate::game::Game, id: CharId, rag: RagdollId, kind: crate::character::FallStyle, f: V3, h: V3, j: f64, side: f64) -> crate::character::FallGuide {
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
fn character_guide_step(g: &mut crate::game::Game, id: CharId, rag: RagdollId, dt: f64) {
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
fn character_breath_frame(g: &mut crate::game::Game, id: CharId, dt: f64, k: crate::character::CharacterBreathFrameK) {
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
fn character_breath_end(g: &mut crate::game::Game, id: CharId, pose: bool) {
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
pub fn character_gun_under(g: &crate::game::Game, id: CharId) -> V3 {
    let v = g.ch[id].gun_dir; let gu = v[1];   // g × (g × up) = (g·up) g − up
    v3::normalize([v[0] * gu, v[1] * gu - 1.0, v[2] * gu])
}
/// Head rotation (world), from rest-pose model vectors. (Character.headRot)
pub fn character_head_rot(g: &crate::game::Game, id: CharId) -> Quat {
    let c = &g.ch[id];
    v3::quat::mul(v3::quat::yaw(c.body_yaw), v3::quat::mul(c.skel.global_rot[c.anim.nodes.head], c.rest_head_inv))
}
/// Head look direction (world). (Character.headDir)
pub fn character_head_dir(g: &crate::game::Game, id: CharId) -> V3 {
    let r = character_head_rot(g, id);
    v3::normalize(v3::quat::rotate(r, [0.0, 0.0, 1.0]))
}
/// Capsule proxies in world space and a bounding sphere; devices test laser trips on these. (Character.capsules)
pub fn character_capsules(g: &crate::game::Game, id: CharId) -> crate::character::CharacterCapsules {
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
pub fn character_inside_body(g: &crate::game::Game, id: CharId, p: V3, crown_pad: Option<f64>) -> bool {
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
pub fn character_ray_hit(g: &crate::game::Game, id: CharId, ro: V3, rd: V3, tmax: f64) -> f64 {
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
    const DT: f64 = 1.0 / 60.0;
    const F: V3 = [0.6, 0.0, 0.8];     // the way he faces
    const H: V3 = [-0.8, 0.0, 0.6];    // the way the blow pushes him
    const R: crate::ragdoll::RAG = crate::ragdoll::RAG { pelvis: 0.0, spine: 1.0, neck: 2.0, head: 3.0, sho_l: 4.0, sho_r: 5.0, hand_l: 8.0, hand_r: 9.0, hip_l: 10.0, hip_r: 11.0, knee_l: 12.0, knee_r: 13.0, ank_l: 14.0, ank_r: 15.0 };

    /// A man standing still at (2, 0, 3), facing F: every particle where it would be.
    fn stood() -> crate::ragdoll::Ragdoll {
        let side: V3 = [F[2], 0.0, -F[0]];
        let at = |s: f64, y: f64, f: f64| -> [f64; 3] { [2.0 + side[0] * s + F[0] * f, y, 3.0 + side[2] * s + F[2] * f] };
        let parts: [(f64, [f64; 3]); 16] = [
            (R.pelvis, at(0.0, 0.95, 0.0)), (R.spine, at(0.0, 1.2, 0.0)), (R.neck, at(0.0, 1.5, 0.0)), (R.head, at(0.0, 1.66, 0.02)), (R.sho_l, at(-0.2, 1.42, 0.0)), (R.sho_r, at(0.2, 1.42, 0.0)),
            (6.0, at(-0.25, 1.15, 0.0)), (7.0, at(0.25, 1.15, 0.0)), (R.hand_l, at(-0.25, 0.9, 0.05)), (R.hand_r, at(0.25, 0.9, 0.05)), (R.hip_l, at(-0.1, 0.9, 0.0)), (R.hip_r, at(0.1, 0.9, 0.0)),
            (R.knee_l, at(-0.11, 0.5, 0.03)), (R.knee_r, at(0.11, 0.5, 0.03)), (R.ank_l, at(-0.12, 0.08, 0.0)), (R.ank_r, at(0.12, 0.08, 0.0)),
        ];
        let mut rag = crate::ragdoll::Ragdoll::blank();
        rag.x = vec![0.0; 48];
        for (i, p) in parts { for c in 0..3 { rag.x[i as usize * 3 + c] = p[c]; } }
        rag
    }
    fn guide(rag: &crate::ragdoll::Ragdoll, kind: FallStyle, room: f64, ahead: f64) -> FallGuide { fall_guide_of(rag, &R, kind, F, H, 0.97, -1.0, None, room, ahead) }
    /// a particle's pin at `t`: the last of its pins that holds at that time
    fn pin(rag: &crate::ragdoll::Ragdoll, gd: &mut FallGuide, i: f64, t: f64) -> V3 {
        let (fr, nd) = (gd.frame, gd.nod);
        let q = gd.pins.iter_mut().rev().find(|q| q.i == i).unwrap_or_else(|| panic!("no pin on {i}"));
        pin_at(rag, &fr, &nd, &mut q.at, t)
    }
    const KINDS: [FallStyle; 5] = [FallStyle::Fold, FallStyle::Sink, FallStyle::Cut, FallStyle::Topple, FallStyle::Slump];

    /// A fall is led from where he stands, and the pins do not jump on the way down.
    /// A FAILURE MEANS: a man hit snaps to another pose before he falls.
    #[test]
    fn a_fall_starts_where_he_stands_and_nothing_jumps_on_the_way_down() {
        let rag = stood();
        for kind in KINDS {
            let mut gd = guide(&rag, kind, 1.0, 0.0);
            let (fr, nd, end) = (gd.frame, gd.nod, gd.end);
            assert!(end > 0.2 && end < 2.0, "{kind:?} takes {end} s");
            for q in gd.pins.iter_mut() {
                if q.t0 > 0.0 { continue; }
                let p = pin_at(&rag, &fr, &nd, &mut q.at, 0.0);
                assert!(v3::dist(p, part_at(&rag, q.i)) < 1e-6, "{kind:?}: pin {} starts {} m off him", q.i, v3::dist(p, part_at(&rag, q.i)));
            }
            for q in gd.pins.iter_mut() {
                let mut t = q.t0;
                let mut last = pin_at(&rag, &fr, &nd, &mut q.at, t);
                while t + DT < q.off {
                    t += DT;
                    let p = pin_at(&rag, &fr, &nd, &mut q.at, t);
                    assert!(v3::dist(p, last) < 0.2, "{kind:?}: pin {} jumps {} m at {t}", q.i, v3::dist(p, last));
                    last = p;
                }
            }
        }
    }

    /// Onto his knees and down on his front, carried on by what was pushing him when he was held.
    #[test]
    fn a_man_who_sinks_ends_flat_on_his_front_with_his_limbs_laid_by_him() {
        let rag = stood();
        let mut gd = guide(&rag, FallStyle::Sink, 1.0, 0.0);
        let end = gd.end;
        let (pv0, pv) = (part_at(&rag, R.pelvis), pin(&rag, &mut gd, R.pelvis, end));
        assert!(m::abs(pv[1] - 0.135) < 1e-6, "his hips lie on the floor: {}", pv[1]);
        assert!(pin(&rag, &mut gd, R.head, end)[1] < 0.5, "and his head: {}", pin(&rag, &mut gd, R.head, end)[1]);
        let forward = v3::dot(v3::sub(pv, pv0), F);
        assert!(forward > 0.0 && forward < 0.3, "he goes down where he was, a little forward: {forward}");
        let neck = pin(&rag, &mut gd, R.neck, end);
        assert!(v3::dot(v3::sub(neck, pv), F) > 0.3, "with his head the way he faced");
        let mid = pin(&rag, &mut gd, R.pelvis, 0.05 + 0.56 * 0.97);
        assert!(mid[1] < 0.5, "he is on his knees before he lies down: {}", mid[1]);
        for (i, y) in [(R.knee_l, 0.07), (R.ank_r, 0.06), (R.hand_l, 0.05)] { assert!(m::abs(pin(&rag, &mut gd, i, end + 1.0)[1] - y) < 1e-6, "limb {i} lies on the floor"); }
        // carried on, by the whole distance; and a tight spot lays no limbs out
        let mut on = guide(&rag, FallStyle::Sink, 1.0, 0.5);
        let on_end = on.end;
        let further = v3::dot(v3::sub(pin(&rag, &mut on, R.pelvis, on_end), pv), F);
        assert!(m::abs(further - 0.5) < 1e-6, "{further}");
        let tight = guide(&rag, FallStyle::Sink, 0.2, 0.0);
        assert!(tight.pins.len() < gd.pins.len(), "a tight spot lays no limbs out");
    }

    /// Thrown over from his heels: he goes the way he was pushed, and his feet stay where they stood.
    #[test]
    fn a_man_who_topples_goes_over_his_heels_the_way_he_was_pushed() {
        let rag = stood();
        let mut gd = guide(&rag, FallStyle::Topple, 1.0, 0.0);
        let end = gd.end;
        let (pv0, pv) = (part_at(&rag, R.pelvis), pin(&rag, &mut gd, R.pelvis, end));
        let along = v3::dot(v3::sub(pv, pv0), H);
        assert!(along > 0.3, "his hips go with the push: {along}");
        assert!(pv[1] < pv0[1] - 0.1, "and down: {}", pv[1]);
        let heels = |p: V3, q: V3| -> V3 { [(p[0] + q[0]) / 2.0, 0.0, (p[2] + q[2]) / 2.0] };
        let stood_at = heels(part_at(&rag, R.ank_l), part_at(&rag, R.ank_r));
        let (l, r) = (pin(&rag, &mut gd, R.ank_l, end), pin(&rag, &mut gd, R.ank_r, end));
        assert!(v3::dist(heels(l, r), stood_at) < 1e-6, "his feet turn under him and stay where they stood");
        assert!(m::abs(l[1] - part_at(&rag, R.ank_l)[1]) < 1e-6, "on the floor");
    }

    /// Where he stands, the trunk folding forward over his hips: what happens next is the ragdoll's.
    #[test]
    fn a_man_who_slumps_folds_forward_where_he_stands() {
        let rag = stood();
        let mut gd = guide(&rag, FallStyle::Slump, 1.0, 0.0);
        let end = gd.end;
        assert_eq!(pin(&rag, &mut gd, R.pelvis, end), part_at(&rag, R.pelvis));
        let neck = pin(&rag, &mut gd, R.neck, end);
        assert!(v3::dot(v3::sub(neck, part_at(&rag, R.pelvis)), F) > 0.3, "his neck comes forward: {neck:?}");
        assert!(neck[1] < part_at(&rag, R.neck)[1] - 0.1, "and down: {neck:?}");
    }

    /// Both hands up to the throat in the beat he stands, then the fall away from what cut him.
    #[test]
    fn a_man_cut_stands_a_beat_with_his_hands_at_his_throat() {
        let rag = stood();
        let mut gd = guide(&rag, FallStyle::Cut, 1.0, 0.0);
        let pv0 = part_at(&rag, R.pelvis);
        let beat = 0.2;
        assert!(v3::dist(pin(&rag, &mut gd, R.pelvis, beat), pv0) < 0.05, "he is still up: {:?}", pin(&rag, &mut gd, R.pelvis, beat));
        let neck = pin(&rag, &mut gd, R.neck, beat);
        for i in [R.hand_l, R.hand_r] { assert!(v3::dist(pin(&rag, &mut gd, i, beat), neck) < 0.15, "hand {i}"); }
        let end = gd.end;
        assert!(pin(&rag, &mut gd, R.pelvis, end)[1] < 0.3, "and then he is sitting on the floor");
    }
}


// ---- the animator's place: `anim.posIn = this.pos` is a REFERENCE to the body's position array
//
// The TypeScript hands the animator the body's own `pos` array at every Character.update. Until the next update the animator reads THAT ARRAY: it
// sees a write made in place (the collision's push, a door's shove), and it does NOT see a new array put in `pos`'s place (`c.pos = v3.mad(…)`, the
// step of every man who walks; a snap, a lerp onto a spot): from the first such replacement of a tick to that tick's Character.update, `posIn` is
// where the body stood before it. kitanim.ts asks in that window (kitSwap's `toModelP(shown.r)`: where the hands are, in the model's space), so a
// man who changes guns while he walks starts the errand a step back.
//
// STAND-IN for a serial on the position (the types are generated): `Anim.pos_in` holds POS_IN_IS_POS while the animator's array IS the body's, and
// the body's place as it was from the first replacement on. It belongs in a `pos_serial` on Character with the serial the animator was handed
// (the device asked for a goal: a value with the serial of the object it is).

/// `anim.pos_in` while the animator's array is the body's own
pub const POS_IN_IS_POS: V3 = [f64::NAN, f64::NAN, f64::NAN];

/// A NEW ARRAY in `pos`'s place (the TypeScript's `c.pos = <a fresh array>`): the animator keeps the one it was handed.
pub fn pos_replace(g: &mut crate::game::Game, c: CharId, to: V3) {
    let ch = &mut g.ch[c];
    if ch.anim.pos_in[0].is_nan() { ch.anim.pos_in = ch.pos; }
    ch.pos = to;
}

/// The same, said before a replacement made on a lent copy of the body (the player's legs): the animator keeps the place as it stands now.
pub fn pos_replaced(g: &mut crate::game::Game, c: CharId) {
    let ch = &mut g.ch[c];
    if ch.anim.pos_in[0].is_nan() { ch.anim.pos_in = ch.pos; }
}

/// `this.posIn`: the array the animator was handed at the last Character.update, as it reads now.
pub(super) fn anim_pos_in(g: &crate::game::Game, c: CharId) -> V3 {
    let ch = &g.ch[c];
    if ch.anim.pos_in[0].is_nan() { ch.pos } else { ch.anim.pos_in }
}
