//! src/game/character.ts lines 1733 to 2111
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

const DEGR: f64 = m::PI / 180.0;
const AY: V3 = [0.0, 1.0, 0.0];
const AZ: V3 = [0.0, 0.0, 1.0];

/// the rig's named nodes, as the animator and a layer call them (character.ts N), in the written order
static N: [(&str, &str); 24] = [
    ("hips", "DEF-hips"), ("spine1", "DEF-spine.001"), ("spine2", "DEF-spine.002"), ("spine3", "DEF-spine.003"), ("neck", "DEF-neck"), ("head", "DEF-head"),
    ("upperArmL", "DEF-upper_arm.L"), ("forearmL", "DEF-forearm.L"), ("handL", "DEF-hand.L"), ("upperArmR", "DEF-upper_arm.R"), ("forearmR", "DEF-forearm.R"), ("handR", "DEF-hand.R"),
    ("thighL", "DEF-thigh.L"), ("shinL", "DEF-shin.L"), ("footL", "DEF-foot.L"), ("toeL", "DEF-toe.L"), ("thighR", "DEF-thigh.R"), ("shinR", "DEF-shin.R"), ("footR", "DEF-foot.R"), ("toeR", "DEF-toe.R"),
    ("shoulderL", "DEF-shoulder.L"), ("shoulderR", "DEF-shoulder.R"), ("middle1R", "DEF-f_middle.01.R"), ("middle1L", "DEF-f_middle.01.L"),
];

// ---- seams: what skeleton 3 still lacks (each has its line in notes.md)

/// `this.kitHold = v`, outer None = undefined, Some(None) = null (character.ts:2047, 2051)
fn seam_set_kit_hold(g: &mut crate::game::Game, c: CharId, v: Option<Option<crate::character::KitItem>>) { todo!("needs: Anim.kit_hold as Option<Option<KitItem>>: readers tell undefined from null") }
/// `b.rack`, outer None = undefined, Some(None) = null: a round still chambered (character.ts:2023, 2024)
fn seam_rack(b: &crate::character::ReloadBeat) -> Option<Option<f64>> { todo!("needs: ReloadBeat.rack as Option<Option<f64>>: rl_beat tells undefined from null") }
/// this.fkList in the constructor (character.ts:1855): the stub takes a CharId and the animator is not on a body yet
fn seam_fk_list(g: &mut crate::game::Game, ch: RigId, ends: &[f64]) -> Vec<i32> { todo!("needs: fk_list by rig, callable before the animator is on a body") }
/// this.sampleJog in the constructor (character.ts:1904): the stub takes a CharId and the animator is not on a body yet
fn seam_sample_jog(g: &mut crate::game::Game, a: &crate::character::Anim, ph: f64, out: &mut crate::anim_skeleton::Pose) { todo!("needs: sample_jog on an animator not yet on a body") }
/// this.stanceOf in the constructor (character.ts:1909, 1921): the stub takes a CharId and the animator is not on a body yet
fn seam_stance_of(ty: &[Vec<f64>], tz: &[Vec<f64>]) -> Option<crate::character::AnimStanceOf> { todo!("needs: stance_of with no CharId (it reads no state)") }

// ---- helpers

/// `ch.nodeByName.get(name)`, loud when missing (the constructor's `node`)
fn node(g: &mut crate::game::Game, ch: RigId, name: &str) -> usize {
    match g.tab.rig[ch].node_by_name.get(String::from(name)) {
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
fn grab(hands: &[Vec<i32>], q: &mut crate::anim_skeleton::Pose) -> Vec<Vec<f32>> {
    hands.iter().map(|ns| {
        let mut a = vec![0.0f32; 60];
        for (k, i) in ns.iter().enumerate() {
            let r = crate::anim_skeleton::pose_rot(q, *i as f64);
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
/// the arithmetic of rlBeat, with `b.rack` as read (outer None = undefined, Some(None) = null)
fn rl_beat_keys(k: &mut crate::character::AnimRlB, b: &crate::character::ReloadBeat, rack: Option<Option<f64>>) {
    let d = m::max(0.3, b.dur);
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

/// the pose under every look-only arm layer that showed this frame; none when none showed. A copy, read at once by its caller   (character.ts underPose)
pub fn anim_under_pose(g: &mut crate::game::Game, c: CharId) -> Option<crate::anim_skeleton::Pose> {
    let singles = g.k.posemath.singles;
    let crate::character::Anim { arms_pre, hand_pre, under_buf, arm_r_only, .. } = &mut g.ch[c].anim;
    let (a, h) = match (arms_pre.as_ref(), hand_pre.as_ref()) {
        (Some(a), Some(h)) => (a, h),
        (a, h) => return a.or(h).cloned(),
    };
    let buf = under_buf.get_or_insert_with(|| crate::anim_skeleton::pose_new(a.num_nodes as f64));
    crate::anim_skeleton::blend_pose(Some(a), h, 1.0, Some(arm_r_only.as_slice()), buf, singles);
    Some(buf.clone())
}

/// the animator of a rig: its poses, clips, masks, and everything measured off the clips; then his phase off the game's die   (character.ts CharacterAnimator)
pub fn anim_new(g: &mut crate::game::Game, ch: RigId) -> crate::character::Anim {
    let mut a = crate::character::Anim::blank();
    a.rig = ch;
    let n = g.tab.rig[ch].nodes.len() as f64;
    a.pose = crate::anim_skeleton::pose_new(n);
    a.p_a = crate::anim_skeleton::pose_new(n);
    a.p_b = crate::anim_skeleton::pose_new(n);
    a.p_c = crate::anim_skeleton::pose_new(n);
    a.p_upper = crate::anim_skeleton::pose_new(n);
    a.p_aim = crate::anim_skeleton::pose_new(n);
    let mut made = crate::anim_skeleton::pose_new(n);
    a.rest = crate::anim_skeleton::rest_pose(&g.tab.rig[ch], &mut made);
    crate::anim_skeleton::pose_copy(&mut a.pose, &a.rest);
    crate::anim_skeleton::pose_copy(&mut a.p_a, &a.rest);
    crate::anim_skeleton::pose_copy(&mut a.p_b, &a.rest);
    crate::anim_skeleton::pose_copy(&mut a.p_c, &a.rest);
    crate::anim_skeleton::pose_copy(&mut a.p_upper, &a.rest);
    crate::anim_skeleton::pose_copy(&mut a.p_aim, &a.rest);

    // what is measured off a clip to keep is MEASURED IN DOUBLES whatever the pose rule says (POSEMATH)
    let pose_rule = g.k.posemath.singles;
    g.k.posemath.singles = false;

    let need: [&'static str; 15] = ["Idle_Loop", "Walk_Loop", "Jog_Fwd_Loop", "Crouch_Idle_Loop", "Crouch_Fwd_Loop", "Pistol_Idle_Loop", "Pistol_Aim_Neutral", "Pistol_Aim_Up", "Pistol_Aim_Down", "Pistol_Shoot", "Idle_Torch_Loop", "Hit_Chest", "Sprint_Loop", "Pistol_Reload", "Punch_Cross"];
    for k in need {
        match g.tab.rig[ch].clips.get(String::from(k)) {
            Some(cl) => { a.clips.set(k, *cl); }
            None => panic!("missing clip {}", k),
        }
    }
    if let Some(j) = g.tab.rig[ch].clips.get(String::from("Punch_Jab")) { a.clips.set("Punch_Jab", *j); }

    let at: Vec<usize> = N.iter().map(|e| node(g, ch, e.1)).collect();
    a.nodes = crate::character::AnimNodes {
        hips: at[0], spine1: at[1], spine2: at[2], spine3: at[3], neck: at[4], head: at[5],
        upper_arm_l: at[6], forearm_l: at[7], hand_l: at[8], upper_arm_r: at[9], forearm_r: at[10], hand_r: at[11],
        thigh_l: at[12], shin_l: at[13], foot_l: at[14], toe_l: at[15], thigh_r: at[16], shin_r: at[17], foot_r: at[18], toe_r: at[19],
        shoulder_l: at[20], shoulder_r: at[21], middle1_r: at[22], middle1_l: at[23],
    };
    let nd = a.nodes;

    for s in ["L", "R"] {
        let mut hand = Vec::new();
        for f in ["f_index", "f_middle", "f_ring", "f_pinky"] {
            let mut joints = Vec::new();
            for j in [1, 2, 3] { joints.push(node(g, ch, &format!("DEF-{}.0{}.{}", f, j, s))); }
            hand.push(joints);
        }
        a.fingers.push(hand);
    }
    for s in ["L", "R"] {
        let mut joints = Vec::new();
        for j in [1, 2, 3] { joints.push(node(g, ch, &format!("DEF-thumb.0{}.{}", j, s))); }
        a.thumbs.push(joints);
    }
    a.hn.nodes = [0usize, 1].iter().map(|&s| {
        let mut l: Vec<i32> = a.thumbs[s].iter().map(|&i| i as i32).collect();
        for f in &a.fingers[s] { for &i in f { l.push(i as i32); } }
        l
    }).collect();

    a.upper_mask = crate::anim_skeleton::build_mask(g, ch, &[entry("DEF-spine.001", 0.3, false), entry("DEF-spine.002", 0.65, false), entry("DEF-spine.003", 1.0, true)]);
    a.arm_r_mask = crate::anim_skeleton::build_mask(g, ch, &[entry("DEF-shoulder.R", 1.0, true), entry("DEF-spine.003", 0.25, false)]);
    a.head_mask = crate::anim_skeleton::build_mask(g, ch, &[entry("DEF-neck", 1.0, true)]);
    a.arms_mask = crate::anim_skeleton::build_mask(g, ch, &[entry("DEF-shoulder.L", 1.0, true), entry("DEF-shoulder.R", 1.0, true)]);
    a.arm_r_only = crate::anim_skeleton::build_mask(g, ch, &[entry("DEF-shoulder.R", 1.0, true)]);
    a.torso_mask = a.upper_mask.iter().enumerate().map(|(i, w)| if a.arms_mask[i] > 0.0 { 0.0 } else { *w }).collect();

    a.fk = crate::anim_skeleton::pose_fk_new(g, ch);
    a.fk_legs = seam_fk_list(g, ch, &[nd.hips as f64, nd.thigh_l as f64, nd.shin_l as f64, nd.foot_l as f64, nd.toe_l as f64, nd.thigh_r as f64, nd.shin_r as f64, nd.foot_r as f64, nd.toe_r as f64]);
    a.fk_arms = seam_fk_list(g, ch, &[nd.spine3 as f64, nd.upper_arm_r as f64, nd.forearm_r as f64, nd.hand_r as f64, nd.middle1_r as f64, nd.upper_arm_l as f64, nd.forearm_l as f64, nd.hand_l as f64]);
    a.fk_head = seam_fk_list(g, ch, &[nd.head as f64]);

    crate::anim_skeleton::sample_clip(&g.tab.clip[kept(&a, "Pistol_Aim_Neutral")], 0.0, false, &mut a.p_aim);
    crate::anim_skeleton::pose_fk_run(&mut a.fk, &a.rest, &g.tab.rig[ch]);
    a.chest_rest_inv = v3::quat::conj(a.fk.rot[nd.spine3]);
    a.head_rest_inv = v3::quat::conj(a.fk.rot[nd.head]);
    a.hand_fix = [nd.hand_l, nd.hand_r].iter().map(|&h| {
        let r = a.fk.rot[h];
        let f = v3::normalize(v3::quat::rotate(r, AY));
        let b = v3::normalize(v3::mad(AY, f, -v3::dot(AY, f)));
        v3::quat::mul(v3::quat::conj(v3::quat::from_basis(v3::cross(f, b), f, b)), r)
    }).collect();
    crate::anim_skeleton::pose_fk_run(&mut a.fk, &a.p_aim, &g.tab.rig[ch]);
    a.grip_up = v3::quat::rotate(v3::quat::conj(a.fk.rot[nd.hand_r]), AY);

    {   // the standing idle: the gaze, the feet, and the fist the hand pass measures from
        let mut p = a.rest.clone();
        crate::anim_skeleton::sample_clip(&g.tab.clip[kept(&a, "Idle_Loop")], 0.0, false, &mut p);
        crate::anim_skeleton::pose_fk_run(&mut a.fk, &p, &g.tab.rig[ch]);
        a.gaze_local = v3::quat::rotate(v3::quat::conj(a.fk.rot[nd.head]), AZ);
        a.hn.fist = grab(&a.hn.nodes, &mut p);
        let mut pc = a.rest.clone();
        crate::anim_skeleton::sample_clip(&g.tab.clip[kept(&a, "Crouch_Idle_Loop")], 0.0, false, &mut pc);
        let cr = grab(&a.hn.nodes, &mut pc);
        a.hn.own = [0usize, 1].iter().map(|&sd| {
            let crd: Vec<f64> = cr[sd].iter().map(|x| *x as f64).collect();
            let fd: Vec<f64> = a.hn.fist[sd].iter().map(|x| *x as f64).collect();
            [0.0f64, 1.0, 2.0, 3.0, 4.0].iter().map(|&d| {
                m::max(10.0 * DEGR, (q_ang(&crd, d * 12.0, 1.0, &fd, d * 12.0) + q_ang(&crd, d * 12.0 + 4.0, 1.0, &fd, d * 12.0 + 4.0) + q_ang(&crd, d * 12.0 + 8.0, 1.0, &fd, d * 12.0 + 8.0)) / 3.0) as f32
            }).collect()
        }).collect();
        a.idle_feet = crate::character::AnimIdleFeet { l: a.fk.pos[nd.foot_l], r: a.fk.pos[nd.foot_r], r_l: a.fk.rot[nd.foot_l], r_r: a.fk.rot[nd.foot_r] };
    }
    {   // the crouch walk's gait for the deliberate retime: the ankles' fore-aft middle, and where the toes come down
        let mut p = a.rest.clone();
        let cf = kept(&a, "Crouch_Fwd_Loop");
        let cf_dur = g.tab.clip[cf].duration;
        const M: usize = 48;
        let (mut zmin, mut zmax) = (1e9f64, -1e9f64);
        let mut ty: [Vec<f64>; 2] = [Vec::new(), Vec::new()];
        for i in 0..M {
            crate::anim_skeleton::sample_clip(&g.tab.clip[cf], cf_dur * i as f64 / M as f64, true, &mut p);
            crate::anim_skeleton::pose_fk_run(&mut a.fk, &p, &g.tab.rig[ch]);
            for f in [nd.foot_l, nd.foot_r] { zmin = m::min(zmin, a.fk.pos[f][2]); zmax = m::max(zmax, a.fk.pos[f][2]); }
            ty[0].push(a.fk.pos[nd.toe_l][1]);
            ty[1].push(a.fk.pos[nd.toe_r][1]);
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
    {   // the run clip's loop seam: each keyed channel's gap, first − last, spread round the cycle at sample time (sampleJog)
        let cj = kept(&a, "Jog_Fwd_Loop");
        let cj_dur = g.tab.clip[cj].duration;
        let (mut pa, mut pb) = (a.rest.clone(), a.rest.clone());
        crate::anim_skeleton::sample_clip(&g.tab.clip[cj], 0.0, false, &mut pa);
        crate::anim_skeleton::sample_clip(&g.tab.clip[cj], cj_dur, false, &mut pb);
        for tr in &g.tab.clip[cj].tracks {
            if tr.path == 0.0 {
                let d = v3::sub(crate::anim_skeleton::pose_pos(&mut pa, tr.node), crate::anim_skeleton::pose_pos(&mut pb, tr.node));
                if v3::len(d) > 5e-4 { a.jog_seam.push(crate::character::AnimJogSeamItem { node: tr.node, d: Some(d), axis: None, ang: None }); }
            } else if tr.path == 1.0 {
                let mut q = v3::quat::mul(v3::quat::conj(crate::anim_skeleton::pose_rot(&mut pb, tr.node)), crate::anim_skeleton::pose_rot(&mut pa, tr.node));
                if q[3] < 0.0 { q = [-q[0], -q[1], -q[2], -q[3]]; }
                let s = m::hyp3(q[0], q[1], q[2]);
                let ang = 2.0 * m::atan2(s, q[3]);
                if ang > 0.3 * DEGR { a.jog_seam.push(crate::character::AnimJogSeamItem { node: tr.node, d: None, axis: Some([q[0] / s, q[1] / s, q[2] / s]), ang: Some(ang) }); }
            }
        }
    }
    layer_tables_fit();
    let all: Vec<ClipId> = g.tab.rig[ch].clips.values().copied().collect();
    crate::anim_skeleton::layer_clips_fit(g, &all, a.p_a.num_nodes as f64);

    // what a layer of the Rust module is handed with the rig (skeleton.ts layerSlot)
    let mut cast = crate::anim_skeleton::LayerCast::blank();
    for (i, e) in N.iter().enumerate() { cast.nodes.set(e.0, at[i]); }
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

    {   // the run clip's pelvis height by phase and its mean, its ankles' middle, its stances
        let mut p = a.rest.clone();
        const M: usize = 64;
        let mut sum = 0.0;
        let (mut zmin, mut zmax) = (1e9f64, -1e9f64);
        let mut ty: [Vec<f64>; 2] = [Vec::new(), Vec::new()];
        let mut tz: [Vec<f64>; 2] = [Vec::new(), Vec::new()];
        for i in 0..M {
            seam_sample_jog(g, &a, i as f64 / M as f64, &mut p);
            crate::anim_skeleton::pose_fk_run(&mut a.fk, &p, &g.tab.rig[ch]);
            a.jog_hips_y[i] = a.fk.pos[nd.hips][1] as f32;
            sum += a.jog_hips_y[i] as f64;   // (the table's single, read back)
            for f in [nd.foot_l, nd.foot_r] { zmin = m::min(zmin, a.fk.pos[f][2]); zmax = m::max(zmax, a.fk.pos[f][2]); }
            ty[0].push(a.fk.pos[nd.toe_l][1]);
            ty[1].push(a.fk.pos[nd.toe_r][1]);
            tz[0].push(a.fk.pos[nd.toe_l][2]);
            tz[1].push(a.fk.pos[nd.toe_r][2]);
        }
        a.jog_hips_mean = sum / M as f64;
        if zmax > zmin { a.jog_mid = (zmin + zmax) / 2.0; }
        if let Some(s) = seam_stance_of(&ty, &tz) { a.run_gait.jog = crate::character::AnimRunGaitJog { c: s.c, c_l: s.c_l, c_r: s.c_r }; }
    }
    {   // the sprint clip likewise: its ankles' middle, their lowest height, its stances
        let mut p = a.rest.clone();
        let cs = kept(&a, "Sprint_Loop");
        let cs_dur = g.tab.clip[cs].duration;
        const M: usize = 64;
        let (mut zmin, mut zmax, mut ymin) = (1e9f64, -1e9f64, 1e9f64);
        let mut ty: [Vec<f64>; 2] = [Vec::new(), Vec::new()];
        let mut tz: [Vec<f64>; 2] = [Vec::new(), Vec::new()];
        for i in 0..M {
            crate::anim_skeleton::sample_clip(&g.tab.clip[cs], cs_dur * i as f64 / M as f64, true, &mut p);
            crate::anim_skeleton::pose_fk_run(&mut a.fk, &p, &g.tab.rig[ch]);
            for f in [nd.foot_l, nd.foot_r] { zmin = m::min(zmin, a.fk.pos[f][2]); zmax = m::max(zmax, a.fk.pos[f][2]); ymin = m::min(ymin, a.fk.pos[f][1]); }
            ty[0].push(a.fk.pos[nd.toe_l][1]);
            ty[1].push(a.fk.pos[nd.toe_r][1]);
            tz[0].push(a.fk.pos[nd.toe_l][2]);
            tz[1].push(a.fk.pos[nd.toe_r][2]);
        }
        if zmax > zmin { a.sprint_mid = (zmin + zmax) / 2.0; }
        if ymin < 1e8 { a.sprint_floor_y = ymin; }
        if let Some(s) = seam_stance_of(&ty, &tz) { a.run_gait.sprint = crate::character::AnimRunGaitSprint { c: s.c, c_l: s.c_l, c_r: s.c_r }; }
    }
    g.k.posemath.singles = pose_rule;
    a.phase = crate::math_dice::sim_random(g);
    a
}

/// the idle and life clocks set off per figure, deterministically, so a room does not breathe in step   (character.ts desync)
pub fn anim_desync(g: &mut crate::game::Game, c: CharId, seed: f64) {
    let a = &mut g.ch[c].anim;
    a.idle_t = desync_hash(seed, 1.0) * 20.0;
    a.life.breath = desync_hash(seed, 2.0) * 6.28;
    a.life.shift_t = 2.0 + desync_hash(seed, 3.0) * 5.0;
    a.life.shift_side = if desync_hash(seed, 4.0) < 0.5 { -1.0 } else { 1.0 };
    a.hn.seed = seed;
}

/// a round fired: the shot's clock, and a long gun's recoil sums   (character.ts fire)
pub fn anim_fire(g: &mut crate::game::Game, c: CharId) {
    g.ch[c].anim.shoot_t = 0.0;
    g.ch[c].anim.hn.fired = 0.0;
    if g.ch[c].anim.long_gun && mv_on(g, crate::character::MvOnK::Recoil) {
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

/// the Hit_Chest layer's clock   (character.ts hit)
pub fn anim_hit(g: &mut crate::game::Game, c: CharId) {
    g.ch[c].anim.hit_t = 0.0;
}

/// startled, from his front   (character.ts flinch)
pub fn anim_flinch(g: &mut crate::game::Game, c: CharId) {
    if mv_on(g, crate::character::MvOnK::Flinch) { anim_flinch_from(g, c, None, None, None); } else { anim_hit(g, c); }
}

/// rounds going past: a duck away from where they come from   (character.ts flinchFrom)
pub fn anim_flinch_from(g: &mut crate::game::Game, c: CharId, toward: Option<V3>, k: Option<f64>, soft: Option<bool>) {
    let (k, soft) = (k.unwrap_or(1.0), soft.unwrap_or(false));
    if !mv_on(g, crate::character::MvOnK::Flinch) {
        if !soft { g.ch[c].anim.hit_t = 0.0; }
        return;
    }
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
        g.ch[c].anim.mvs.fl_t = 0.14 * d * v3::clamp(curve(fl_t / d, &crate::character::mv_flinch_env()), 0.0, 1.0);
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

/// how much of the whole untrained duck is showing   (character.ts flinchShown)
pub fn anim_flinch_shown(g: &mut crate::game::Game, c: CharId) -> f64 {
    let t = v3::clamp(g.ch[c].anim.trained, 0.0, 1.0);
    let (fl_t, fl_k) = (g.ch[c].anim.mvs.fl_t, g.ch[c].anim.mvs.fl_k);
    if fl_t < 0.0 { return 0.0; }
    let (dur, amp) = (g.k.mv.flinch.dur, g.k.mv.flinch.amp);
    v3::clamp(curve(fl_t / v3::lerp(dur[0], dur[1], t), &crate::character::mv_flinch_env()), 0.0, 1.0) * v3::lerp(amp[0], amp[1], t) * fl_k
}

/// the start of a man who has just seen what he did not expect   (character.ts startleFrom)
pub fn anim_startle_from(g: &mut crate::game::Game, c: CharId, toward: Option<V3>) {
    if !mv_on(g, crate::character::MvOnK::Flinch) { g.ch[c].anim.hit_t = 0.0; return; }
    if g.ch[c].anim.mvs.st_t >= 0.0 { return; }
    let st_d = match toward { Some(w) => anim_flat_model(g, c, w, [0.0, 0.0, 1.0]), None => [0.0, 0.0, 1.0] };
    g.ch[c].anim.mvs.st_d = st_d;
    g.ch[c].anim.mvs.st_t = 0.0;
}

/// how much of the start is showing   (character.ts startleShown)
pub fn anim_startle_shown(g: &mut crate::game::Game, c: CharId) -> f64 {
    let st_t = g.ch[c].anim.mvs.st_t;
    if st_t < 0.0 { return 0.0; }
    let s = crate::character::mv_startle();
    v3::clamp(curve(st_t / v3::lerp(s.dur[0], s.dur[1], v3::clamp(g.ch[c].anim.trained, 0.0, 1.0)), &s.env), 0.0, 1.0)
}

/// a round that went home on a man who stays up   (character.ts hitFrom)
pub fn anim_hit_from(g: &mut crate::game::Game, c: CharId, along: Option<V3>) {
    if !mv_on(g, crate::character::MvOnK::Flinch) { g.ch[c].anim.hit_t = 0.0; return; }
    let ht_t = g.ch[c].anim.mvs.ht_t;
    let e0 = if ht_t >= 0.0 { v3::clamp(curve(ht_t, &crate::character::mv_hit_env()), 0.0, 1.0) } else { 0.0 };
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

/// the support hand's underhand toss; off, the chop the teams used to make   (character.ts tossItem)
pub fn anim_toss_item(g: &mut crate::game::Game, c: CharId, release: Option<f64>) {
    let release = release.unwrap_or(crate::character::TOSS_RELEASE);
    if !mv_on(g, crate::character::MvOnK::Toss) { anim_signal(g, c, crate::character::SignalKind::Go, None); return; }
    let m_ = &mut g.ch[c].anim.mvs;
    if m_.ts_t >= 0.0 { return; }
    m_.ts_t = 0.0;
    m_.ts_rel = v3::clamp(release, 0.08, 0.6);
}

/// the toss is under way   (character.ts tossing)
pub fn anim_tossing(g: &mut crate::game::Game, c: CharId) -> bool {
    g.ch[c].anim.mvs.ts_t >= 0.0
}

/// a world direction, flattened, in the figure's un-twisted model frame   (character.ts flatModel)
pub fn anim_flat_model(g: &mut crate::game::Game, c: CharId, w: V3, fb: V3) -> V3 {
    let d = anim_to_model_d(g, c, [w[0], 0.0, w[2]]);
    let l = m::hyp2(d[0], d[2]);
    if l > 1e-4 { [d[0] / l, 0.0, d[2] / l] } else { fb }
}

/// the takedown strike's clock, and the knife's when it is lethal   (character.ts strike)
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
        a.knife_shown = false;
    }
}

/// the blow under way is the knife's   (character.ts strikeLethal)
pub fn anim_strike_lethal(g: &mut crate::game::Game, c: CharId) -> bool {
    g.ch[c].anim.strike_t >= 0.0 && g.ch[c].anim.strike_leth
}

/// the strike is under way   (character.ts striking)
pub fn anim_striking(g: &mut crate::game::Game, c: CharId) -> bool {
    g.ch[c].anim.strike_t >= 0.0
}

/// a takedown's own move, right after the strike it rides   (character.ts blow)
pub fn anim_blow(g: &mut crate::game::Game, c: CharId, style: crate::character::BlowStyle, at: V3, at2: Option<V3>, o: Option<&crate::character::BlowOpts>) {
    let none = crate::character::BlowOpts::default();
    let o = o.unwrap_or(&none);
    let tail = if o.tail == Some(true) { crate::character::blow().tail } else { 0.0 };
    let live = o.live;
    let a = &mut g.ch[c].anim;
    a.bw = Some(crate::character::BlowRun { style, t: 0.0, hit: o.hit.unwrap_or(0.28), end: o.end.unwrap_or(0.62), tail, rise: o.rise.unwrap_or(true), at, at2, live });
    a.kb_slash = style == crate::character::BlowStyle::Slash;
    a.kb_back = style == crate::character::BlowStyle::Mouth;
}

/// the move called off: eased out from where it is   (character.ts blowCancel)
pub fn anim_blow_cancel(g: &mut crate::game::Game, c: CharId) {
    if let Some(b) = &mut g.ch[c].anim.bw {
        b.end = m::min(b.end, b.t + 0.18);
        b.tail = 0.0;
        b.live = None;
    }
}

/// the man it was being done to, let go of   (character.ts takenCancel)
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

/// the man it is done to, while he is still on his feet   (character.ts taken)
pub fn anim_taken(g: &mut crate::game::Game, c: CharId, style: crate::character::TakenStyle, delay: Option<f64>) {
    g.ch[c].anim.tk = Some(crate::character::AnimTk { style, t: 0.0, delay: delay.unwrap_or(0.0), off: -1.0 });
}

/// a takedown's move is under way   (character.ts blowing)
pub fn anim_blowing(g: &mut crate::game::Game, c: CharId) -> bool {
    g.ch[c].anim.bw.is_some()
}

/// the reload one-shot, retimed to `dur`, and the support hand's keys off the weapon's beat   (character.ts reload)
pub fn anim_reload(g: &mut crate::game::Game, c: CharId, dur: Option<f64>, beat: Option<crate::character::ReloadBeat>) {
    g.ch[c].anim.reload_t = 0.0;
    let d = m::max(0.3, dur.unwrap_or(1.6));
    g.ch[c].anim.reload_dur = d;
    let b = match beat {
        Some(b) => b,
        None => crate::character::ReloadBeat { dur: d, out: None, in_: None, rack: None, keep: None, hand: None, exact: None },
    };
    anim_rl_beat(g, c, b);
    g.ch[c].anim.rl_src = None;
    g.ch[c].anim.rl_t = 0.0;
}

/// the hand's keys off a weapon's beat   (character.ts rlBeat)
pub fn anim_rl_beat(g: &mut crate::game::Game, c: CharId, b: crate::character::ReloadBeat) {
    let rack = seam_rack(&b);
    rl_beat_keys(&mut g.ch[c].anim.rl_b, &b, rack);
}

/// begin a change of what is in the right fist; the seconds until `to` is up   (character.ts kitSwap)
pub fn anim_kit_swap(g: &mut crate::game::Game, c: CharId, from: Option<crate::character::KitItem>, to: Option<crate::character::KitItem>, shown: Option<crate::character::AnimKitSwapShown>, same: Option<bool>) -> f64 {
    use crate::character::KitItem as Kit;
    let (mut from, same) = (from, same.unwrap_or(false));
    let (mut s0, mut roll0, mut re) = (0.0, 0.0, false);
    let mid_swap = g.ch[c].anim.kit.is_some();
    if mid_swap {
        from = g.ch[c].anim.kit_hold;   // (`?? null`: undefined and null alike)
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
    {
        let a = &mut g.ch[c].anim;
        crate::anim_skeleton::pose_fk_run(&mut a.fk, &a.pose, &g.tab.rig[a.rig]);
    }
    let f = anim_trunk_frame(g, c);
    let (r, rq, l, lq) = {
        let a = &g.ch[c].anim;
        let (fk, nd) = (&a.fk, &a.nodes);
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
    seam_set_kit_hold(g, c, Some(from.or(if re { to } else { None })));
    g.ch[c].anim.kit_stow = s0;
    g.ch[c].anim.kit_roll = roll0;
    d
}

/// the swap dropped where it stands: the kit is where the game says, at once   (character.ts kitCancel)
pub fn anim_kit_cancel(g: &mut crate::game::Game, c: CharId) {
    g.ch[c].anim.kit = None;
    seam_set_kit_hold(g, c, None);
    g.ch[c].anim.kit_roll = 0.0;
    g.ch[c].anim.kit_stow = 0.0;
    g.ch[c].anim.kit_lie = None;
}

/// a swap is under way   (character.ts kitBusy)
pub fn anim_kit_busy(g: &mut crate::game::Game, c: CharId) -> bool {
    g.ch[c].anim.kit.is_some()
}

/// a goggle flip is under way   (character.ts nvBusy)
pub fn anim_nv_busy(g: &mut crate::game::Game, c: CharId) -> bool {
    g.ch[c].anim.nv.is_some()
}

/// the goggles go down or up: by the left hand if `hand`, else a nod   (character.ts nvFlip)
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

/// the flip let go of where it is   (character.ts nvCancel)
pub fn anim_nv_cancel(g: &mut crate::game::Game, c: CharId) {
    let a = &mut g.ch[c].anim;
    a.nv = None;
    a.nv_arm = 0.0;
    a.nv_nod = 0.0;
    a.nv_a = 0.0;
    a.nv_shown = None;
}

/// the errand under way has the rifle in it   (character.ts kitRifle)
pub fn anim_kit_rifle(g: &mut crate::game::Game, c: CharId) -> bool {
    use crate::character::KitItem as Kit;
    match &g.ch[c].anim.kit {
        Some(k) => k.from == Some(Kit::Rifle) || k.to == Some(Kit::Rifle),
        None => false,
    }
}

/// the support hand's magazine work can be shown at all   (character.ts kitHandsOn)
pub fn anim_kit_hands_on(g: &mut crate::game::Game, c: CharId) -> bool {
    g.k.moves.reload > 0.5
}

/// a magazine change as the drawer needs it; none when none is shown   (character.ts reloadShow)
pub fn anim_reload_show(g: &mut crate::game::Game, c: CharId) -> Option<crate::character::AnimReloadShow> {
    let a = &g.ch[c].anim;
    if !(a.mv2_shown.reload > 0.5) { return None; }
    let (b, t) = (&a.rl_b, a.mv2.rl_at);
    Some(crate::character::AnimReloadShow {
        out: t >= b.out && t < b.in_,
        hand: (b.keep >= 0.0 && t >= b.out && t < b.keep) || (t >= b.fresh + 0.04 && t < b.in_),
    })
}

/// the change the support hand is showing, as its beat and where it is on it   (character.ts feedClock)
pub fn anim_feed_clock(g: &mut crate::game::Game, c: CharId) -> Option<crate::character::AnimFeedClock> {
    let a = &g.ch[c].anim;
    if !(a.mv2_shown.reload > 0.5) { return None; }
    let b = &a.rl_b;
    Some(crate::character::AnimFeedClock { t: a.mv2.rl_at, dur: b.dur, out: b.out, in_: b.in_, rack: b.rack, pull: b.pull, keep: b.keep, fresh: b.fresh })
}

/// the hand's clock stops where it is and the arm eases back   (character.ts cancelReload)
pub fn anim_cancel_reload(g: &mut crate::game::Game, c: CharId) {
    g.ch[c].anim.reload_t = -1.0;
    g.ch[c].anim.rl_t = -1.0;
}

/// the overhand throw's clock, and which swing its distance reads as   (character.ts throwItem)
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

/// the reload one-shot is playing   (character.ts reloading)
pub fn anim_reloading(g: &mut crate::game::Game, c: CharId) -> bool {
    g.ch[c].anim.reload_t >= 0.0
}

/// which swing the throw under way is   (character.ts throwRead)
pub fn anim_throw_read(g: &mut crate::game::Game, c: CharId) -> Option<crate::character::AnimThrowRead> {
    let a = &g.ch[c].anim;
    if a.throw_t < 0.0 { None }
    else if a.thr_low >= 0.0 { Some(crate::character::AnimThrowRead::Toss) }
    else if a.thr_far < 1.0 { Some(crate::character::AnimThrowRead::Short) }
    else { Some(crate::character::AnimThrowRead::Full) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::character::{AnimRlB, ReloadBeat};

    fn beat(dur: f64) -> ReloadBeat { ReloadBeat { dur, out: None, in_: None, rack: None, keep: None, hand: None, exact: None } }
    fn keys(b: ReloadBeat, rack: Option<Option<f64>>) -> ([u64; 6], bool, bool) {
        let mut k = AnimRlB::zeroed();
        rl_beat_keys(&mut k, &b, rack);
        ([k.out.to_bits(), k.in_.to_bits(), k.rack.to_bits(), k.dur.to_bits(), k.keep.to_bits(), k.fresh.to_bits()], k.pull, k.hand)
    }

    /// the TypeScript's own bits for desync's `h` (bun, EXACT on)
    #[test]
    fn desync_hash_bits() {
        let want: [(f64, [u64; 4]); 5] = [
            (0.0, [0x3fbfd1d9b7000000, 0x3fdbe3b05c800000, 0x3fbbb788bd000000, 0x3fdb5e6720400000]),
            (1.0, [0x3fea48ad64a00000, 0x3fe4bbfa4dc00000, 0x3fceb8f5ae800000, 0x3fe43964c1400000]),
            (7.0, [0x3fe661a595e00000, 0x3fd9106c31400000, 0x3fe88667e3a00000, 0x3feb06a5c0000000]),
            (1000000000.0, [0x3fda673d08c00000, 0x3fd5439934000000, 0x3fec4335c1a00000, 0x3fe7a5bcf3e00000]),
            (123456789.0, [0x3feffda55e200000, 0x3fdd121a9ec00000, 0x3fe1802da5400000, 0x3feedf2008c00000]),
        ];
        for (seed, bits) in want {
            for (i, b) in bits.iter().enumerate() { assert_eq!(desync_hash(seed, (i + 1) as f64).to_bits(), *b); }
        }
    }

    /// the TypeScript's own bits for fire's round hash
    #[test]
    fn round_hash_bits() {
        for (n, b) in [(0.0, 0u64), (1.0, 0x3fe3c6ef36200000), (3.0, 0x3feb54cda2600000), (1000.0, 0x3fa166b6d0000000)] {
            assert_eq!(round_hash(n).to_bits(), b);
        }
    }

    /// the TypeScript's own bits for rlBeat: undefined against null in `rack`, the floors, the exact beat
    #[test]
    fn rl_beat_bits() {
        assert_eq!(keys(beat(1.6), None), ([0x3fdba5e353f7ceda, 0x3ff0a3d70a3d70a4, 0x3ff5c28f5c28f5c3, 0x3ff9c28f5c28f5c3, 0xbff0000000000000, 0x3fe78d4fdf3b645a], true, false));
        assert_eq!(keys(beat(2.4), Some(None)), ([0x3fe4bc6a7ef9db23, 0x3ff8f5c28f5c28f6, 0x400051eb851eb852, 0x4003333333333333, 0xbff0000000000000, 0x3ff1a9fbe76c8b44], false, false));
        let kept = ReloadBeat { out: Some(0.5), in_: Some(1.6), keep: Some(1.0), hand: Some(true), ..beat(2.4) };
        assert_eq!(keys(kept, Some(Some(2.0))), ([0x3fe0000000000000, 0x3ff999999999999a, 0x4000000000000000, 0x4003333333333333, 0x3ff0000000000000, 0x3ff2e147ae147ae1], true, true));
        assert_eq!(keys(beat(0.1), None), ([0x3fc3333333333333, 0x3fe8000000000000, 0x3fee666666666666, 0x3ff3333333333333, 0xbff0000000000000, 0x3fdccccccccccccd], true, false));
        let exact = ReloadBeat { out: Some(0.2), in_: Some(0.56), exact: Some(true), ..beat(1.1) };
        assert_eq!(keys(exact, None), ([0x3fc999999999999a, 0x3fe1eb851eb851ec, 0x3ff199999999999a, 0x3ff199999999999a, 0xbff0000000000000, 0x3fd851eb851eb852], false, false));
        assert_eq!(keys(exact, Some(Some(0.9))), ([0x3fc999999999999a, 0x3fe1eb851eb851ec, 0x3feccccccccccccd, 0x3ff199999999999a, 0xbff0000000000000, 0x3fd851eb851eb852], true, false));
        assert_eq!(keys(ReloadBeat { hand: Some(true), ..beat(2.0) }, None), ([0x3fe147ae147ae148, 0x3ff4cccccccccccd, 0x3ffb333333333333, 0x4000000000000000, 0xbff0000000000000, 0x3feb020c49ba5e36], true, true));
    }

    /// the TypeScript's own bits for the throw's read of its distance: a toss, a hard throw kept from being one, the short overhand, all of it
    #[test]
    fn throw_by_bits() {
        let read = |d: f64, s: Option<f64>| { let (a, b) = throw_by_read(2.3, 7.1, 8.0, d, s); (a.to_bits(), b.to_bits()) };
        assert_eq!(read(1.1, None), (0x3fde9bd37a6f4deb, 0x3ff0000000000000));
        assert_eq!(read(1.1, Some(9.0)), (0xbff0000000000000, 0));
        assert_eq!(read(5.3, None), (0xbff0000000000000, 0x3fe5e00000000000));
        assert_eq!(read(12.0, None), (0xbff0000000000000, 0x3ff0000000000000));
    }
}
