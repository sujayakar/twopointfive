//! posehome's bodies: who is posed in a game, and the pose pool tended as a tick opens.
#![allow(unused_imports, unused_variables)]
use super::*;
use crate::anim_skeleton::{LoosePose, Pose};
use crate::game::Game;
use crate::ids::*;
use alloc::{collections::BTreeMap, vec::Vec};

/// every character the game poses, in the order the pool is filled: the players' men, the shift, the civilians
fn posed_chars(g: &mut Game) -> Vec<CharId> {
    let mut out = Vec::new();
    for pl in crate::seats::men(g) { out.push(g.pl[pl].ch); }
    for i in 0..g.guards.len() { out.push(g.gd[g.guards[i]].ch); }
    for i in 0..g.civilians.len() { out.push(g.gd[g.civilians[i]].ch); }
    out
}

/// Every handle on a pose one character keeps besides his set's own list: his animator's stages and buffers, and his ragdoll's pose.
/// A handle is a COPY of a place in the pool (the TypeScript's is the pose object, whose place is written in it), so whatever moves a pose rewrites these.
fn handles(ch: &mut crate::character::Character, f: &mut dyn FnMut(&mut Pose)) {
    let a = &mut ch.anim;
    for p in [&mut a.pose, &mut a.p_a, &mut a.p_b, &mut a.p_c, &mut a.p_upper, &mut a.rest, &mut a.p_aim] { f(p); }
    for p in [&mut a.arms_pre, &mut a.arms_buf, &mut a.hand_pre, &mut a.sw_buf, &mut a.hand_buf, &mut a.under_buf, &mut a.big_buf] { if let Some(p) = p.as_mut() { f(p); } }
    if let Some(sp) = a.sp.as_mut() { if let Some(p) = sp.pic.as_mut() { f(p); } }
    if let Some(rd) = ch.rd.as_mut() { f(&mut rd.pose); }
}

/// a character's poses given their new places: his set's list and every handle of his
fn replace(g: &mut Game, c: CharId, map: &BTreeMap<u32, Pose>) {
    let s = g.ch[c].anim.poses;
    for p in g.tab.pose_set[s].list.iter_mut() { if let Some(n) = map.get(&p.at) { *p = *n; } }
    handles(&mut g.ch[c], &mut |p| { if let Some(n) = map.get(&p.at) { *p = *n; } });
}

/// the poses a character keeps from tick to tick, each once, in the order they go into the pool: his set's list, then his ragdoll's
fn kept(g: &mut Game, c: CharId, ragdoll: bool) -> Vec<Pose> {
    if ragdoll { return g.ch[c].rd.as_ref().map(|rd| alloc::vec![rd.pose]).unwrap_or_default(); }
    let s = g.ch[c].anim.poses;
    g.tab.pose_set[s].list.clone()
}

/// A man who is on a list again, or was made since the last tick: whatever of his stands outside the pool (he left while others stayed, and his floats
/// were kept for him) takes a place at the pool's end, as the TypeScript's go in from arrays of their own.
fn come_home(g: &mut Game, c: CharId) {
    let mut map: BTreeMap<u32, Pose> = BTreeMap::new();
    for rag in [false, true] {
        for p in kept(g, c, rag) {
            if p.at < Pose::LOOSE || map.contains_key(&p.at) { continue; }
            let mut q = p;
            crate::anim_skeleton::pose_take_home(g, &mut q);
            if q.at < Pose::LOOSE { map.insert(p.at, q); }
        }
    }
    if !map.is_empty() { replace(g, c, &map); }
}

/// EVERYBODY OUT, THE POOL EMPTIED, THOSE WHO ARE THERE BACK IN (posehome.ts moveOut, then moveIn's second pass): the sets of the men on the lists in
/// the lists' order, each set's poses in the order it made them. A man who is on no list keeps his floats outside the pool (rows of `loose_poses`,
/// as the TypeScript's go out into arrays of their own) until he is on one again. A ragdoll's pose is a pose of its own on both sides and is not
/// the pool's to move (a carried game may still hold one in the pool: it goes back in after the sets).
fn everybody_out_and_in(g: &mut Game, chars: &[CharId]) {
    let slot = g.sim;
    let stay: Vec<PoseSetId> = chars.iter().map(|&c| g.ch[c].anim.poses).collect();
    let held = g.pose_sets.clone();
    let leavers: Vec<CharId> = g.ch.ids().filter(|&c| { let s = g.ch[c].anim.poses; held.contains(&s) && !stay.contains(&s) }).collect();
    let mut map: BTreeMap<u32, Pose> = BTreeMap::new();
    // out: the floats of every pose that goes back in, and of every leaver's
    let mut back: Vec<(Pose, Vec<f32>)> = Vec::new();
    for rag in [false, true] {
        for &c in chars {
            for p in kept(g, c, rag) {
                // (a pose of a set's list that stands OUTSIDE the pool goes in with the rest, in the list's place: a man made since the last tick,
                //  whose set was never at home (a restart made the players and the guards anew while the old sets still held the pool), is on the
                //  lists like any other, and the TypeScript's second pass puts every pose of every listed set in, in order (posehome.ts moveIn →
                //  poseSetIn). Left out here, his set was told it was at home while his animator's handles still named rows outside the pool, and
                //  the next layer was handed a pose the module does not hold. A ragdoll's own pose outside the pool stays outside.)
                if p.nodes == 0 || (rag && p.at >= Pose::LOOSE) || back.iter().any(|b| b.0.at == p.at) { continue; }
                let f = crate::anim_skeleton::pose_cols(g, p).to_vec();
                back.push((p, f));
            }
        }
    }
    for &c in &leavers {
        for rag in [false, true] {
            for p in kept(g, c, rag) {
                if p.nodes == 0 || p.at >= Pose::LOOSE || map.contains_key(&p.at) { continue; }
                let floats = crate::anim_skeleton::pose_cols(g, p).to_vec();
                let row = g.loose_poses.len() as u32;
                g.loose_poses.push(LoosePose { nodes: p.nodes, floats });
                map.insert(p.at, Pose { at: Pose::LOOSE + row, nodes: p.nodes });
            }
        }
    }
    for &s in &held { crate::anim_skeleton::pose_set_out(g, s); }
    g.pose_sets.clear();
    ::sim::poses::clear_of(slot);
    // in
    for (p, floats) in back {
        let at = ::sim::poses::new_of(slot, p.nodes as usize);
        let q = if at < Pose::LOOSE {
            let q = Pose { at, nodes: p.nodes };
            crate::anim_skeleton::pose_cols(g, q).copy_from_slice(&floats);
            q
        } else {
            // (the pool has no room: the pose stays outside it, and its set counts it as a stray)
            let row = g.loose_poses.len() as u32;
            g.loose_poses.push(LoosePose { nodes: p.nodes, floats });
            Pose { at: Pose::LOOSE + row, nodes: p.nodes }
        };
        map.insert(p.at, q);
    }
    for &c in chars.iter().chain(leavers.iter()) { replace(g, c, &map); }
    for &s in &stay { if !g.pose_sets.contains(&s) { g.pose_sets.push(s); crate::anim_skeleton::pose_set_in(g, s); } }
}

/// THE POOL HOLDS THE POSES OF THE CHARACTERS WHO ARE IN THE GAME, and no others. As a tick opens: is every character of the game's lists at home, and
/// is every set that is at home one of theirs? Then nothing is done. A set at home that is no longer on a list (a man was taken off the floor, the
/// shift was benched) means everybody comes out, the pool is emptied, and those who are there go back in; one who arrived, or came back, goes in at
/// the end.   (posehome.ts moveIn, the tenant's `tend`; game.ts stepSeats `this.sim.settle(this)`)
pub fn pose_home_tend(g: &mut crate::game::Game) {
    let chars = posed_chars(g);
    let mut there = 0usize;
    let mut late = false;
    for &c in &chars {
        let s = &g.tab.pose_set[g.ch[c].anim.poses];
        if s.home.is_none() { late = true; } else { there += 1; if s.stray != 0.0 { late = true; } }
    }
    if there != g.pose_sets.len() { everybody_out_and_in(g, &chars); return; }
    if !late { return; }
    for &c in &chars {
        let s = g.ch[c].anim.poses;
        let home = g.tab.pose_set[s].home.is_some();
        if !home { g.pose_sets.push(s); }
        if !home || g.tab.pose_set[s].stray != 0.0 { come_home(g, c); crate::anim_skeleton::pose_set_in(g, s); }
    }
}

// ---- THE STAGE (anim/skeleton.ts stageOpen / stageShut)
//
// An animator's update runs over poses that stand in a pool (its layers are handed their poses by their places). A man of a ticking game has his in
// the game's pool. A man made INSIDE a tick is not at home until the next tick opens: the TypeScript lends his update "the stage", a block of its
// own, and takes his poses back into arrays of his own as the update shuts. Here the stage is the END OF THE GAME'S OWN POOL for the length of the
// update: his poses go in past what is in use, and as it shuts they come out into the rows they came from and the pool is as it was, to the byte.

/// The update of a man whose set is not at home opens: his poses into the pool's end, his handles on them, and the set told it stands (a pose it
/// makes during the update is made there). None: the set is at home and nothing is lent.   (skeleton.ts stageOpen)
pub fn stage_open(g: &mut Game, c: CharId) -> Option<Stage> {
    let set = g.ch[c].anim.poses;
    if g.tab.pose_set[set].home.is_some() || g.tab.pose_set[set].list.is_empty() { return None; }
    let live = ::sim::state::state_read(g.sim).poses.live;
    let mut map: BTreeMap<u32, Pose> = BTreeMap::new();
    let mut back = Vec::new();
    for p in g.tab.pose_set[set].list.clone() {
        if p.nodes == 0 || p.at < Pose::LOOSE || map.contains_key(&p.at) { continue; }
        let at = ::sim::poses::new_of(g.sim, p.nodes as usize);
        if at >= Pose::LOOSE { panic!("posing: one animator's poses do not fit the stage (the pool's end: rust/sim/src/poses.rs)"); }
        let home = Pose { at, nodes: p.nodes };
        let floats = crate::anim_skeleton::pose_cols(g, p).to_vec();
        crate::anim_skeleton::pose_cols(g, home).copy_from_slice(&floats);
        map.insert(p.at, home);
        back.push((home, p));
    }
    replace(g, c, &map);
    g.tab.pose_set[set].home = Some(crate::Opaque);
    Some(Stage { set, live, back })
}

/// …and shuts: every pose of his out again (those lent, into the rows they came from; one made during the update, into a row of its own), and the
/// pool's end zeros again.   (skeleton.ts stageShut)
pub fn stage_shut(g: &mut Game, c: CharId, st: Stage) {
    let mut map: BTreeMap<u32, Pose> = BTreeMap::new();
    for (home, own) in st.back.iter().copied() {
        let floats = crate::anim_skeleton::pose_cols(g, home).to_vec();
        crate::anim_skeleton::pose_cols(g, own).copy_from_slice(&floats);
        map.insert(home.at, own);
    }
    g.tab.pose_set[st.set].home = None;
    g.tab.pose_set[st.set].stray = 0.0;
    for p in g.tab.pose_set[st.set].list.clone() {
        if p.nodes == 0 || p.at >= Pose::LOOSE || map.contains_key(&p.at) { continue; }
        let floats = crate::anim_skeleton::pose_cols(g, p).to_vec();
        let own = crate::anim_skeleton::pose_new(g, p.nodes as usize);
        crate::anim_skeleton::pose_cols(g, own).copy_from_slice(&floats);
        map.insert(p.at, own);
    }
    replace(g, c, &map);
    // the pool as it was: what the stage used is zeros again, and not in use
    let ps = &mut ::sim::state::state(g.sim).poses;
    let from = ::sim::poses::used(ps);
    ps.live = st.live;
    let keep = ::sim::poses::used(ps);
    for i in keep..from { ps.data[i] = ::sim::single::S::ZERO; }
}
