//! src/anim/skeleton.ts lines 1 to 656
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};


// ---------------------------------------------------------------- the bench (rust/sim/src/skeleton.rs Bench: its tables are reached directly, no addresses)

/// The module, if it answers a question about a pose of `n` nodes now. The module is linked and its maths are the game's, so only the size can refuse.   (skeleton.ts benchFor)
pub fn bench_for(n: f64, owed: Option<bool>) -> Option<crate::sim_sim::SimExports> {
    let _owed = owed.unwrap_or(false);
    if n > BENCH_NODES {
        return None;
    }
    Some(crate::sim_sim::sim())
}

/// A size a layer of the module is handed at most: one past it is a mistake in the game's own tables, said by name.   (skeleton.ts layerFits)
pub fn layer_fits(what: &str, n: f64, max: f64, min: Option<f64>) {
    let min = min.unwrap_or(0.0);
    if !(n >= min && n <= max) {
        if min > 0.0 {
            panic!("posing: {} has {} entries and a layer of the Rust module takes {} to {} (rust/sim/src/layers.rs and knobs.rs hold the lengths)", what, n, min, max);
        }
        panic!("posing: {} has {} entries and a layer of the Rust module takes at most {} (rust/sim/src/layers.rs and knobs.rs hold the lengths)", what, n, max);
    }
}

/// A table of names of rust/sim/src/knobs.rs, one a line.   (skeleton.ts benchNames)
pub fn bench_names(x: crate::sim_sim::SimExports, table: f64) -> Vec<String> {
    let table = table as u32;
    let (at, n) = (::sim::knobs::addr(table), ::sim::knobs::len(table));
    // (knobs.rs hands its tables out by address alone, as the page reads them; these are its &'static str names)
    let b: &[u8] = unsafe { core::slice::from_raw_parts(at as *const u8, n) };
    let mut s = String::new();
    for i in 0..n {
        s.push(b[i] as char);
    }
    s.split('\n').filter(|l| !l.is_empty()).map(String::from).collect()
}

fn cast_node(cast: &LayerCast, name: &str) -> Option<usize> {
    cast.nodes.iter().find(|(k, _)| *k == name).map(|(_, v)| *v)
}
fn cast_list<'a>(cast: &'a LayerCast, name: &str) -> Option<&'a Vec<i32>> {
    cast.lists.iter().find(|(k, _)| *k == name).map(|(_, v)| v)
}
fn cast_nums<'a>(cast: &'a LayerCast, name: &str) -> Option<&'a Vec<f64>> {
    cast.nums.iter().find(|(k, _)| *k == name).map(|(_, v)| v)
}

/// The slot to ask a layer of man `c` in: his game's block, all his poses standing there, and his rig on the bench with its roles, lists and tables.   (skeleton.ts layerSlot)
pub fn layer_slot(g: &mut crate::game::Game, c: CharId) -> f64 {
    let (ch, set) = (g.ch[c].anim.rig, g.ch[c].anim.poses);
    let stray = g.tab.pose_set[set].stray;
    if stray != 0.0 {
        panic!("posing: a man has {} pose(s) the pose pool had no room for, and a layer is posed only where all of his stand in the module (game/posehome.ts holds the pool's size)", stray);
    }
    let slot = g.sim as f64;
    if bench_rig(g, ch, Some(true)).is_none() {
        panic!("posing: a layer was asked of a rig the module's bench does not hold (anim/skeleton.ts benchRig: the rig's nodes or its order are more than the bench has room for)");
    }
    if g.m.anim_skeleton.layers.roles != Some(ch) {
        if g.m.anim_skeleton.layers.names.is_none() {
            let x = crate::sim_sim::sim();
            let names = AnimSkeletonModLAYERSNames { roles: bench_names(x, 3.0), lists: bench_names(x, 5.0), nums: bench_names(x, 8.0) };
            g.m.anim_skeleton.layers.names = Some(names);
        }
        let names = g.m.anim_skeleton.layers.names.clone().unwrap();
        let cast = g.ch[c].anim.cast.clone();
        let cast = &cast;
        let w_role = ::sim::knobs::addr(2) as *mut u32;
        let w_list = ::sim::knobs::addr(4) as *mut u32;
        let w_len = ::sim::knobs::addr(6) as *mut u32;
        let max = ::sim::knobs::len(4) / core::cmp::max(1, ::sim::knobs::len(6));
        let nmax = ::sim::knobs::len(7) / core::cmp::max(1, ::sim::knobs::len(9));
        // a name the animator does not have, and a list or a table longer than the module holds, are found before a word is written
        for r in &names.roles {
            if cast_node(cast, r).is_none() {
                panic!("posing: the Rust module asks for the node \"{}\" (rust/sim/src/knobs.rs) and the animator has none of that name", r);
            }
        }
        for l in &names.lists {
            if cast_list(cast, l).is_none() {
                panic!("posing: the Rust module asks for the list \"{}\" (rust/sim/src/knobs.rs) and the animator has none of that name", l);
            }
        }
        for t in &names.nums {
            if cast_nums(cast, t).is_none() {
                panic!("posing: the Rust module asks for the table \"{}\" (rust/sim/src/knobs.rs) and the animator has none of that name", t);
            }
        }
        for l in &names.lists {
            layer_fits(&alloc::format!("the rig's list of nodes \"{}\"", l), cast_list(cast, l).unwrap().len() as f64, max as f64, None);
        }
        for t in &names.nums {
            layer_fits(&alloc::format!("the rig's table of numbers \"{}\"", t), cast_nums(cast, t).unwrap().len() as f64, nmax as f64, None);
        }
        // (knobs.rs's tables are private statics it hands out by address: written as the page writes them)
        for i in 0..names.roles.len() {
            let node = cast_node(cast, &names.roles[i]).unwrap();
            unsafe { w_role.add(i).write(node as u32) };
        }
        for i in 0..names.lists.len() {
            let l = cast_list(cast, &names.lists[i]).unwrap();
            for k in 0..l.len() {
                unsafe { w_list.add(i * max + k).write(l[k] as u32) };
            }
            unsafe { w_len.add(i).write(l.len() as u32) };
            let lists = &mut g.m.anim_skeleton.layers.lists;
            if lists.len() <= i {
                lists.resize(i + 1, Vec::new());
            }
            lists[i] = l.clone();
        }
        let m_num = ::sim::knobs::addr(7) as *mut f64;
        let w_num = ::sim::knobs::addr(9) as *mut u32;
        for i in 0..names.nums.len() {
            let t = cast_nums(cast, &names.nums[i]).unwrap();
            for k in 0..t.len() {
                unsafe { m_num.add(i * nmax + k).write(t[k]) };
            }
            unsafe { w_num.add(i).write(t.len() as u32) };
        }
        g.m.anim_skeleton.layers.roles = Some(ch);
    }
    g.m.anim_skeleton.pose_bench.in_place += 1.0;
    g.m.anim_skeleton.pose_bench.owed += 1.0;
    slot
}

/// What a layer's export answered: 0 is a mistake and loud; else the run-forward bones it ran are brought back from the bench into the man's `fk`, and no others.   (skeleton.ts layerRan)
pub fn layer_ran(g: &mut crate::game::Game, code: f64, c: CharId) {
    if code == 0.0 {
        panic!("posing: a layer of the Rust module could not run: it was handed a pose, a node of the rig, a clip or a table of keys the module does not hold");
    }
    g.m.anim_skeleton.layers.ran += 1.0;
    if js::to_i32(code) & 2 != 0 {
        bench_get_fk(&mut g.ch[c].anim.fk);
        return;
    }
    let mut bits = js::to_u32(code) >> 2;
    let mut i = 0usize;
    while bits != 0 {
        if bits & 1 != 0 {
            let l = &g.m.anim_skeleton.layers.lists[i];
            let fk = &mut g.ch[c].anim.fk;
            let b = ::sim::skeleton::bench();
            for k in 0..l.len() {
                let n = l[k] as usize;
                let (r, p) = (n * 4, n * 3);
                let (q, v) = (&mut fk.rot[n], &mut fk.pos[n]);
                q[0] = b.rot[r];
                q[1] = b.rot[r + 1];
                q[2] = b.rot[r + 2];
                q[3] = b.rot[r + 3];
                v[0] = b.pos[p];
                v[1] = b.pos[p + 1];
                v[2] = b.pos[p + 2];
            }
        }
        bits >>= 1;
        i += 1;
    }
}

/// skeleton.ts:108 `c.get.call(self, ...)`, that is cols_table_get(&c, &g.ch[self_], out, o): the skeleton gives cols_to_bench no g to find the man by.
fn seam_cols_get(c: ColsTable, self_: CharId, out: &mut [f64], o: usize) {
    todo!("needs: g in cols_to_bench's signature, to reach g.ch[self_] for cols_table_get")
}
/// skeleton.ts:109 `c.put.call(self, ...)`, that is cols_table_put(&c, &mut g.ch[self_], from, o): as seam_cols_get.
fn seam_cols_put(c: ColsTable, self_: CharId, from: &[f64], o: usize) {
    todo!("needs: g in cols_from_bench's signature, to reach g.ch[self_] for cols_table_put")
}

/// Hand a layer's kept columns over to the bench's args, before the ask.   (skeleton.ts colsToBench)
pub fn cols_to_bench(c_2: crate::anim_skeleton::ColsTable, self_: CharId, at: Option<f64>) {
    let at = at.unwrap_or(LAYER_COLS_AT);
    seam_cols_get(c_2, self_, &mut ::sim::skeleton::bench().args, at as usize);
}

/// Take a layer's kept columns back off the bench's args, after the ask.   (skeleton.ts colsFromBench)
pub fn cols_from_bench(c_2: crate::anim_skeleton::ColsTable, self_: CharId, at: Option<f64>) {
    let at = at.unwrap_or(LAYER_COLS_AT);
    seam_cols_put(c_2, self_, &::sim::skeleton::bench().args, at as usize);
}

/// The module with `ch`'s rig on the bench, written when another was there; None for a rig the bench cannot hold.   (skeleton.ts benchRig)
pub fn bench_rig(g: &mut crate::game::Game, ch: RigId, owed: Option<bool>) -> Option<crate::sim_sim::SimExports> {
    let n = g.tab.rig[ch].nodes.len();
    let x = bench_for(n as f64, owed)?;
    if g.m.anim_skeleton.pose_bench.rig != Some(ch) {
        g.m.anim_skeleton.pose_bench.rig = None;
        let rig = &g.tab.rig[ch];
        if rig.order.len() > n {
            return None;
        }
        let b = ::sim::skeleton::bench();
        for i in 0..n {
            b.parent[i] = rig.nodes[i].parent as i32;
        }
        for i in 0..rig.order.len() {
            b.order[i] = rig.order[i] as u32;
        }
        ::sim::skeleton::bench_rig(n as u32, rig.order.len() as u32);
        if !::sim::skeleton::bench_rig_ok() {
            return None;
        }
        g.m.anim_skeleton.pose_bench.rig = Some(ch);
    }
    Some(x)
}

/// A pose written to place `slot` of the bench.   (skeleton.ts benchPut)
pub fn bench_put(g: &mut crate::game::Game, p: crate::anim_skeleton::Pose, slot: f64) {
    let o = (slot * BENCH_SLOT) as usize;
    let d = cols_at(g.sim, p);
    let b = ::sim::skeleton::bench();
    for k in 0..d.len() {
        ::sim::single::put(&mut b.pose, o + k, d[k] as f64);
    }
}

/// A pose read back from place `slot`: each column's rows of the pose's own nodes, the rows past the last node staying its zeros.   (skeleton.ts benchGet)
pub fn bench_get(g: &mut crate::game::Game, p: crate::anim_skeleton::Pose, slot: f64) {
    let o = (slot * BENCH_SLOT) as usize;
    let (n_, n) = (rows_of(p), p.nodes as usize);
    let d = cols_at(g.sim, p);
    let b = ::sim::skeleton::bench();
    for c in 0..10 {
        for i in 0..n {
            d[c * n_ + i] = ::sim::single::get(&b.pose, o + c * n_ + i) as f32;
        }
    }
}

/// The run-forward bones onto the bench.   (skeleton.ts benchPutFK)
pub fn bench_put_fk(fk: &crate::anim_skeleton::PoseFK) {
    let b = ::sim::skeleton::bench();
    let (mut r, mut p) = (0usize, 0usize);
    for i in 0..fk.rot.len() {
        let (q, v) = (fk.rot[i], fk.pos[i]);
        b.rot[r] = q[0];
        b.rot[r + 1] = q[1];
        b.rot[r + 2] = q[2];
        b.rot[r + 3] = q[3];
        b.pos[p] = v[0];
        b.pos[p + 1] = v[1];
        b.pos[p + 2] = v[2];
        r += 4;
        p += 3;
    }
}

/// The run-forward bones back off the bench.   (skeleton.ts benchGetFK)
pub fn bench_get_fk(fk: &mut crate::anim_skeleton::PoseFK) {
    let b = ::sim::skeleton::bench();
    let (mut r, mut p) = (0usize, 0usize);
    for i in 0..fk.rot.len() {
        let (q, v) = (&mut fk.rot[i], &mut fk.pos[i]);
        q[0] = b.rot[r];
        q[1] = b.rot[r + 1];
        q[2] = b.rot[r + 2];
        q[3] = b.rot[r + 3];
        v[0] = b.pos[p];
        v[1] = b.pos[p + 1];
        v[2] = b.pos[p + 2];
        r += 4;
        p += 3;
    }
}

// ---------------------------------------------------------------- a pose

/// Rows a column of a pose of `n` nodes holds: `n` rounded up to whole vectors of four.   (skeleton.ts poseRows)
pub fn pose_rows(n: f64) -> f64 {
    (js::to_i32(n + 3.0) & !3) as f64
}

/// Rows a column of this pose holds.
fn rows_of(p: Pose) -> usize {
    ::sim::skeleton::rows(p.nodes as usize)
}

/// Where a pose's singles are in its block's pool: (first, how many). A pose with no place has no floats anywhere, which is loud.
fn span_of(p: Pose) -> (usize, usize) {
    if p.nodes == 0 {
        return (0, 0);
    }
    if p.at >= Pose::LOOSE {
        panic!("posing: a pose of {} nodes has no place in the block: the pose pool had no room as it was made, or the state bridge carried a pose that was an array of its own and no body took it home with pose_take_home (game/levelfit.ts holds the pool's size)", p.nodes);
    }
    (p.at as usize - ::sim::poses::HEAD / 4, ::sim::skeleton::singles(p.nodes as usize))
}

/// All of a block's pool, as the Float32Array the TypeScript's `cols` is a view of. It is rust/sim's static, not g's, so a body may hold it beside g's tables.
fn pool_at(slot: usize) -> &'static mut [f32] {
    let data = &mut ::sim::state::state(slot).poses.data;
    // (single.rs S is repr(transparent) over the single it stores)
    unsafe { core::slice::from_raw_parts_mut(data.as_mut_ptr() as *mut f32, data.len()) }
}

/// A pose's singles. Never hold two of these for one pose: a body that works on poses that may be one pose uses pool_at and spans.
fn cols_at(slot: usize, p: Pose) -> &'static mut [f32] {
    let (s, n) = span_of(p);
    &mut pool_at(slot)[s..s + n]
}

/// A pose of `num_nodes` nodes, all zeros, in its game's pool. No room (or no nodes) makes a pose with no place, as the TypeScript's stayed its own array: new_of's u32::MAX, past Pose::LOOSE and naming no row of g.loose_poses. See PoseSet.stray and span_of.   (skeleton.ts Pose.constructor)
pub fn pose_new(g: &mut crate::game::Game, num_nodes: usize) -> crate::anim_skeleton::Pose {
    Pose { at: ::sim::poses::new_of(g.sim, num_nodes), nodes: num_nodes as u32 }
}

/// A pose the bridge carried with no place (Pose::LOOSE + its row of g.loose_poses) takes one and its floats. Any other stands as it is: one in the pool, and one made with no place, which has no row. A pool with no room leaves it loose, and span_of says so at its first read.
pub fn pose_take_home(g: &mut crate::game::Game, p: &mut crate::anim_skeleton::Pose) {
    if p.at < Pose::LOOSE {
        return;
    }
    let row = (p.at - Pose::LOOSE) as usize;
    if row >= g.loose_poses.len() {
        return;
    }
    let nodes = g.loose_poses[row].nodes;
    let at = ::sim::poses::new_of(g.sim, nodes as usize);
    if at >= Pose::LOOSE {
        return;
    }
    let home = Pose { at, nodes };
    cols_at(g.sim, home).copy_from_slice(&g.loose_poses[row].floats);
    *p = home;
}

/// The floats, wherever they are: the row itself, to write through and let go before the next call that takes g.   (skeleton.ts Pose.cols)
pub fn pose_cols<'g>(g: &'g mut crate::game::Game, this: crate::anim_skeleton::Pose) -> &'g mut [f32] {
    cols_at(g.sim, this)
}

/// Between poses of one rig; another count of nodes is refused.   (skeleton.ts Pose.copy)
pub fn pose_copy(g: &mut crate::game::Game, this: crate::anim_skeleton::Pose, src: crate::anim_skeleton::Pose) {
    if rows_of(src) != rows_of(this) {
        panic!("Pose.copy: a pose of {} nodes into one of {}", src.nodes, this.nodes);
    }
    if src == this {
        return;
    }
    let from = cols_at(g.sim, src);
    cols_at(g.sim, this).copy_from_slice(from);
}

/// Node i's rotation.   (skeleton.ts Pose.rot)
pub fn pose_rot(g: &mut crate::game::Game, this: crate::anim_skeleton::Pose, i: f64) -> Quat {
    let n_ = rows_of(this);
    let o = ((3 * n_) as isize + i as isize) as usize;
    let d = cols_at(g.sim, this);
    [d[o] as f64, d[o + n_] as f64, d[o + 2 * n_] as f64, d[o + 3 * n_] as f64]
}

/// Node i's rotation written; no such node writes nothing.   (skeleton.ts Pose.setRot)
pub fn pose_set_rot(g: &mut crate::game::Game, this: crate::anim_skeleton::Pose, i: f64, q: Quat) {
    if !(i >= 0.0 && i < this.nodes as f64) {
        return;
    }
    let n_ = rows_of(this);
    let o = 3 * n_ + i as usize;
    let d = cols_at(g.sim, this);
    d[o] = q[0] as f32;
    d[o + n_] = q[1] as f32;
    d[o + 2 * n_] = q[2] as f32;
    d[o + 3 * n_] = q[3] as f32;
}

/// Node i's origin.   (skeleton.ts Pose.pos)
pub fn pose_pos(g: &mut crate::game::Game, this: crate::anim_skeleton::Pose, i: f64) -> V3 {
    let n_ = rows_of(this);
    let i = i as isize;
    let d = cols_at(g.sim, this);
    [d[i as usize] as f64, d[(n_ as isize + i) as usize] as f64, d[((2 * n_) as isize + i) as usize] as f64]
}

// ---------------------------------------------------------------- the poses one character keeps

/// A set that will make at most `most` poses.   (skeleton.ts PoseSet.constructor)
pub fn pose_set_new(g: &mut crate::game::Game, most: f64) -> PoseSetId {
    g.tab.pose_set.push(PoseSet { most, ..PoseSet::blank() })
}

/// One more pose of the set; past `most` is a mistake said by name.   (skeleton.ts PoseSet.make)
pub fn pose_set_make(g: &mut crate::game::Game, id: PoseSetId, n: f64) -> crate::anim_skeleton::Pose {
    let most = g.tab.pose_set[id].most;
    if g.tab.pose_set[id].list.len() as f64 >= most {
        panic!("posing: a set of {} poses at most is asked for one more (the owner says how many it keeps as it makes the set, and a level is held to the pose pool by that number: game/levelfit.ts fitRows)", most);
    }
    let p = pose_new(g, n as usize);
    g.tab.pose_set[id].list.push(p);
    if g.tab.pose_set[id].home.is_some() && !pose_in(g, p) {
        g.tab.pose_set[id].stray += 1.0;
    }
    p
}

/// Singles a game's pool holds.   (skeleton.ts posePoolRoom)
pub fn pose_pool_room() -> f64 {
    ::sim::poses::POOL as f64
}

/// Singles of the pool this set fills when it has made every pose it can (none made yet: none, it has no rig to count by).   (skeleton.ts poseSetMost)
pub fn pose_set_most(g: &mut crate::game::Game, s: PoseSetId) -> f64 {
    let set = &g.tab.pose_set[s];
    match set.list.first() {
        Some(p) => set.most * rows_of(*p) as f64 * 10.0,
        None => 0.0,
    }
}

/// Is this pose's floats in the block. A handle gets its place as it is made and a copy of it could not take a later one, so a pose with none stays so: false.   (skeleton.ts poseIn)
pub fn pose_in(g: &mut crate::game::Game, p: crate::anim_skeleton::Pose) -> bool {
    p.at < Pose::LOOSE
}

/// A pose out of the block: it keeps its place and its floats, as there is nowhere else for them.   (skeleton.ts poseOut)
pub fn pose_out(g: &mut crate::game::Game, p: crate::anim_skeleton::Pose) {}

/// Every pose of a set at home in the game's block; false = the pool had no room for one of them, see `stray`.   (skeleton.ts poseSetIn)
pub fn pose_set_in(g: &mut crate::game::Game, s: PoseSetId) -> bool {
    g.tab.pose_set[s].home = Some(crate::Opaque);
    let mut stray = 0.0;
    for k in 0..g.tab.pose_set[s].list.len() {
        let p = g.tab.pose_set[s].list[k];
        if !pose_in(g, p) {
            stray += 1.0;
        }
    }
    g.tab.pose_set[s].stray = stray;
    stray == 0.0
}

/// …and out again.   (skeleton.ts poseSetOut)
pub fn pose_set_out(g: &mut crate::game::Game, s: PoseSetId) {
    g.tab.pose_set[s].home = None;
    g.tab.pose_set[s].stray = 0.0;
    for k in 0..g.tab.pose_set[s].list.len() {
        let p = g.tab.pose_set[s].list[k];
        pose_out(g, p);
    }
}

/// Is the pose in a block (not −1, the bench). No g here, so only the sign says anything: the slot it is in is g.sim, which is what lend answers.   (skeleton.ts slotOf)
pub fn slot_of(p: crate::anim_skeleton::Pose) -> f64 {
    if p.at >= Pose::LOOSE {
        -1.0
    } else {
        0.0
    }
}

/// A pose made ready for a question: at home as it stands (its slot), or copied to place `place` of the bench (−1).   (skeleton.ts lend)
pub fn lend(g: &mut crate::game::Game, p: crate::anim_skeleton::Pose, place: f64) -> f64 {
    if slot_of(p) < 0.0 {
        bench_put(g, p, place);
        return -1.0;
    }
    g.m.anim_skeleton.pose_bench.in_place += 1.0;
    g.sim as f64
}

/// lend, as rust/sim's questions want the answer: (the game or ON_BENCH, the pose's place in it).
fn lent_at(g: &mut crate::game::Game, p: crate::anim_skeleton::Pose, place: f64) -> (usize, usize) {
    let slot = lend(g, p, place);
    if slot < 0.0 {
        (::sim::skeleton::ON_BENCH, place as usize)
    } else {
        (slot as usize, p.at as usize)
    }
}

/// The module did not know the pose it was asked about: never the game's doing, so it is loud.   (skeleton.ts known)
pub fn known(ok: f64) {
    if !js::truthy(ok) {
        panic!("posing: the Rust module was asked about a pose it does not hold");
    }
}

/// A question's answer, as known takes it.
fn answer(ok: bool) -> f64 {
    if ok {
        1.0
    } else {
        0.0
    }
}

/// `out` set to the rig's rest pose.   (skeleton.ts restPose)
pub fn rest_pose(g: &mut crate::game::Game, ch: RigId, out: crate::anim_skeleton::Pose) {
    let rig = &g.tab.rig[ch];
    if (out.nodes as usize) < rig.nodes.len() {
        panic!("restPose: a pose of {} nodes for a rig of {}", out.nodes, rig.nodes.len());
    }
    let n_ = rows_of(out);
    let d = cols_at(g.sim, out);
    for (i, nd) in rig.nodes.iter().enumerate() {
        for c in 0..3 {
            d[c * n_ + i] = nd.t[c] as f32;
            d[(7 + c) * n_ + i] = nd.s[c] as f32;
        }
        for c in 0..4 {
            d[(3 + c) * n_ + i] = nd.r[c] as f32;
        }
    }
}

// ---------------------------------------------------------------- beside a clip

/// JavaScript's Object.is of two singles as the doubles they stand for.
fn object_is(a: f32, b: f32) -> bool {
    (a.is_nan() && b.is_nan()) || a.to_bits() == b.to_bits()
}

/// What this file keeps beside a clip (its lines of times, its number in the module, the nodes it needs), worked out the first time.   (skeleton.ts sideOf)
pub fn side_of<'g>(g: &'g mut crate::game::Game, clip: ClipId) -> &'g mut crate::anim_skeleton::ClipSide {
    if !g.m.anim_skeleton.clip_side.has(clip) {
        let tr = &g.tab.clip[clip].tracks;
        let mut l = vec![0u8; tr.len()];
        let mut first: Vec<&[f32]> = Vec::new();
        for k in 0..tr.len() {
            let a = &tr[k].times[..];
            let mut line = first.iter().position(|b| b.len() == a.len() && a.iter().enumerate().all(|(i, v)| object_is(*v, b[i]))).map_or(-1i32, |p| p as i32);
            if line < 0 {
                if first.len() < LINES as usize {
                    first.push(a);
                    line = first.len() as i32 - 1;
                } else {
                    line = LINES as i32;
                }
            }
            l[k] = line as u8;
        }
        let mut need = 0.0;
        for t in tr {
            need = m::max(need, t.node + 1.0);
        }
        g.m.anim_skeleton.clip_side.set(clip, ClipSide { lines: l, at: -2.0, need });
    }
    g.m.anim_skeleton.clip_side.get_mut(clip).unwrap()
}

/// The clip copied into the module the first time it is asked for, and its number there kept (−1: the module does not take it).   (skeleton.ts clipIn)
pub fn clip_in(g: &mut crate::game::Game, x: crate::sim_sim::SimExports, clip: ClipId) -> crate::anim_skeleton::ClipSide {
    clip_place(g, clip);
    g.m.anim_skeleton.clip_side.get(clip).unwrap().clone()
}

/// clipIn's work, answering the two numbers its callers read and not a copy of the side's lines: (at, need).
fn clip_place(g: &mut crate::game::Game, clip: ClipId) -> (f64, f64) {
    {
        let c = side_of(g, clip);
        if c.at != -2.0 {
            return (c.at, c.need);
        }
        c.at = -1.0;
    }
    let at = clip_write(&g.tab.clip[clip]);
    let c = g.m.anim_skeleton.clip_side.get_mut(clip).unwrap();
    if let Some(at) = at {
        c.at = at;
    }
    (c.at, c.need)
}

/// The tracks of a clip into rust/sim/src/clips.rs. None: not asked of the module at all, its number stays −1.
pub fn clip_write(clip: &crate::anim_gltf::Clip) -> Option<f64> {
    if clip.tracks.is_empty() || !clip.tracks.iter().all(|t| !t.times.is_empty() && t.values.len() == t.times.len() * (if t.path == 1.0 { 4 } else { 3 })) {
        return None;
    }
    if !::sim::clips::begin(clip.duration, clip.tracks.len()) {
        return None;
    }
    for t in &clip.tracks {
        // (track answers an address as the page wants it, of no use off wasm; `bad` and `used` say the same)
        ::sim::clips::track(t.node as usize, t.path as u32, t.step, t.times.len());
        let tb = ::sim::clips::table();
        if tb.bad {
            break;
        }
        let at = tb.used - t.times.len() - t.values.len();
        for (i, v) in t.times.iter().enumerate() {
            ::sim::single::put(&mut tb.pool, at + i, *v as f64);
        }
        for (i, v) in t.values.iter().enumerate() {
            ::sim::single::put(&mut tb.pool, at + t.times.len() + i, *v as f64);
        }
    }
    Some(::sim::clips::end() as f64)
}

/// The number a layer samples `clip` by; a clip the module does not hold, or one past the pose's nodes, is a mistake said by name.   (skeleton.ts clipNo)
pub fn clip_no(g: &mut crate::game::Game, clip: ClipId, nodes: f64) -> f64 {
    let (at, need) = clip_place(g, clip);
    if at >= 0.0 && need <= nodes {
        return at;
    }
    let name = &g.tab.clip[clip].name;
    if at < 0.0 {
        panic!("posing: a layer samples the clip \"{}\" and the Rust module does not hold it (rust/sim/src/clips.rs: no room in the store, or a track it cannot take)", name);
    }
    panic!("posing: a layer samples the clip \"{}\", which writes node {}, over a pose of {} nodes", name, need - 1.0, nodes);
}

/// A character file's clips against what the module holds, as an animator is made.   (skeleton.ts layerClipsFit)
pub fn layer_clips_fit(g: &mut crate::game::Game, clips: &[ClipId], nodes: f64) {
    if nodes > BENCH_NODES {
        panic!("posing: a rig of {} nodes is more than the Rust module's bench holds ({}: anim/skeleton.ts BENCH_NODES with rust/sim/src/skeleton.rs), and a layer is posed nowhere else", nodes, BENCH_NODES);
    }
    for c in clips {
        clip_no(g, *c, nodes);
    }
}

// ---------------------------------------------------------------- sampling and blending

/// Sample clip at time t (seconds, wrapped if loop) into pose (only nodes with tracks are written): a question of the module, or this file's own arithmetic for what it refuses.   (skeleton.ts sampleClip)
pub fn sample_clip(g: &mut crate::game::Game, clip: ClipId, t: f64, loop_: bool, out: crate::anim_skeleton::Pose, singles: bool) {
    let n = out.nodes as f64;
    if bench_for(n, None).is_some() {
        let (at, need) = clip_place(g, clip);
        if at >= 0.0 && need <= n {
            let (slot, place) = lent_at(g, out, 0.0);
            known(answer(::sim::skeleton::ask_sample(slot, place, out.nodes as usize, at as usize, t, loop_, singles)));
            if slot == ::sim::skeleton::ON_BENCH {
                bench_get(g, out, 0.0);
            }
            return;
        }
    }
    sample_own(&g.tab.clip[clip], t, loop_, cols_at(g.sim, out), rows_of(out), n, singles);
}

/// sampleClip's own arithmetic over a pose's singles (`n_` rows a column). Every track searches by itself: the same place as ClipSide.lines finds, so the same pose.
fn sample_own(clip: &crate::anim_gltf::Clip, t: f64, loop_: bool, d: &mut [f32], n_: usize, num_nodes: f64, singles: bool) {
    let fr = m::fround;
    let dur = clip.duration;
    let mut t = t;
    if loop_ {
        t = js::rem(t, dur);
        if t < 0.0 {
            t += dur;
        }
    } else {
        t = m::min(m::max(t, 0.0), dur);
    }
    for tr in &clip.tracks {
        let times = &tr.times;
        let vals = &tr.values;
        let n = times.len();
        if tr.node >= num_nodes {
            continue;
        }
        let comps = if tr.path == 1.0 { 4 } else { 3 };
        let o = (if tr.path == 0.0 { 0 } else if tr.path == 1.0 { 3 } else { 7 }) * n_ + tr.node as usize;
        // where t falls among this track's keys: 1 before the first · 2 at or past the last · 3 between key `lo` and the next, `f` of the way
        let (how, lo, f);
        if n == 1 || t <= times[0] as f64 {
            how = 1;
            lo = 0;
            f = 0.0;
        } else if t >= times[n - 1] as f64 {
            how = 2;
            lo = n - 1;
            f = 0.0;
        } else {
            let (mut l, mut hi) = (0usize, n - 1);
            while hi - l > 1 {
                let mid = (l + hi) >> 1;
                if times[mid] as f64 <= t {
                    l = mid;
                } else {
                    hi = mid;
                }
            }
            how = 3;
            lo = l;
            f = (t - times[l] as f64) / m::max(1e-6, times[hi] as f64 - times[l] as f64);
        }
        let a = lo * comps;
        let b = a + comps;
        // a key's own values: the first, the last, or (a step track) the one before
        if how != 3 || tr.step {
            for c in 0..comps {
                d[o + c * n_] = vals[a + c];
            }
            continue;
        }
        let v = |i: usize| vals[i] as f64;
        if singles {
            let fs = fr(f);
            if comps == 3 {
                for c in 0..3 {
                    d[o + c * n_] = (v(a + c) + fr(fr(v(b + c) - v(a + c)) * fs)) as f32;
                }
                continue;
            }
            let (mut bx, mut by, mut bz, mut bw) = (v(b), v(b + 1), v(b + 2), v(b + 3));
            let (ax, ay, az, aw) = (v(a), v(a + 1), v(a + 2), v(a + 3));
            if fr(fr(fr(fr(ax * bx) + fr(ay * by)) + fr(az * bz)) + fr(aw * bw)) < 0.0 {
                bx = -bx;
                by = -by;
                bz = -bz;
                bw = -bw;
            }
            let (x, y, z, w) = (fr(ax + fr(fr(bx - ax) * fs)), fr(ay + fr(fr(by - ay) * fs)), fr(az + fr(fr(bz - az) * fs)), fr(aw + fr(fr(bw - aw) * fs)));
            let l = js::or(fr(m::sqrt(fr(fr(fr(fr(x * x) + fr(y * y)) + fr(z * z)) + fr(w * w)))), 1.0);
            d[o] = (x / l) as f32;
            d[o + n_] = (y / l) as f32;
            d[o + 2 * n_] = (z / l) as f32;
            d[o + 3 * n_] = (w / l) as f32;
            continue;
        }
        if comps == 3 {
            for c in 0..3 {
                d[o + c * n_] = (v(a + c) + (v(b + c) - v(a + c)) * f) as f32;
            }
        } else {
            // nlerp, the shorter way round
            let (mut bx, mut by, mut bz, mut bw) = (v(b), v(b + 1), v(b + 2), v(b + 3));
            let (ax, ay, az, aw) = (v(a), v(a + 1), v(a + 2), v(a + 3));
            if ax * bx + ay * by + az * bz + aw * bw < 0.0 {
                bx = -bx;
                by = -by;
                bz = -bz;
                bw = -bw;
            }
            let (x, y, z, w) = (ax + (bx - ax) * f, ay + (by - ay) * f, az + (bz - az) * f, aw + (bw - aw) * f);
            let l = js::or(m::hyp4(x, y, z, w), 1.0);
            d[o] = (x / l) as f32;
            d[o + n_] = (y / l) as f32;
            d[o + 2 * n_] = (z / l) as f32;
            d[o + 3 * n_] = (w / l) as f32;
        }
    }
}

/// out = lerp(a, b, w * mask[node]); no mask means uniform weight. `a` None means out is a.   (skeleton.ts blendPose)
pub fn blend_pose(g: &mut crate::game::Game, a: Option<crate::anim_skeleton::Pose>, b: crate::anim_skeleton::Pose, w: f64, mask: Option<&[f32]>, out: crate::anim_skeleton::Pose, singles: bool) {
    let a = a.unwrap_or(out);
    let n = a.nodes as usize;
    let alike = b.nodes == a.nodes && out.nodes == a.nodes && mask.map_or(true, |k| k.len() == n);
    if alike && bench_for(n as f64, None).is_some() {
        if let Some(k) = mask {
            let bn = ::sim::skeleton::bench();
            for i in 0..k.len() {
                ::sim::single::put(&mut bn.mask, i, k[i] as f64);
            }
        }
        if a.at < Pose::LOOSE && b.at < Pose::LOOSE && out.at < Pose::LOOSE {
            g.m.anim_skeleton.pose_bench.in_place += 1.0;
            known(answer(::sim::skeleton::ask_blend(g.sim, a.at as usize, b.at as usize, w, mask.is_some(), out.at as usize, n, singles)));
            return;
        }
        let sb = if b == a { 0 } else { 1 };
        let so = if out == a { 0 } else if out == b { sb } else { 2 };
        bench_put(g, a, 0.0);
        if sb != 0 {
            bench_put(g, b, 1.0);
        }
        known(answer(::sim::skeleton::ask_blend(::sim::skeleton::ON_BENCH, 0, sb, w, mask.is_some(), so, n, singles)));
        bench_get(g, out, so as f64);
        return;
    }
    blend_own(pool_at(g.sim), span_of(a), span_of(b), span_of(out), n, rows_of(a), w, mask, singles);
}

/// A read of a pose (first single, how many) as a typed array reads: past its end is no number.
#[inline(always)]
fn read_of(d: &[f32], p: (usize, usize), o: usize) -> f64 {
    if o < p.1 {
        d[p.0 + o] as f64
    } else {
        f64::NAN
    }
}
/// A write to a pose as a typed array writes: past its end is nothing.
#[inline(always)]
fn write_of(d: &mut [f32], p: (usize, usize), o: usize, v: f64) {
    if o < p.1 {
        d[p.0 + o] = v as f32;
    }
}

/// blendPose's own arithmetic, for what the module is not asked (poses of different sizes, a mask of another length, more nodes than the bench): the three poses are spans of one store, and out may be a's.
fn blend_own(d: &mut [f32], a: (usize, usize), b: (usize, usize), out: (usize, usize), n: usize, n_: usize, w: f64, mask: Option<&[f32]>, singles: bool) {
    let fr = m::fround;
    let same = out == a;
    macro_rules! aa {
        ($o:expr) => {
            read_of(d, a, $o)
        };
    }
    macro_rules! bb {
        ($o:expr) => {
            read_of(d, b, $o)
        };
    }
    let at = |k: &[f32], i: usize| k.get(i).map_or(f64::NAN, |v| *v as f64);
    if singles {
        let tw = fr(w);
        for i in 0..n {
            let t = match mask {
                Some(k) => fr(tw * at(k, i)),
                None => tw,
            };
            if t <= 0.0 {
                if !same {
                    let mut o = i;
                    for _ in 0..10 {
                        let v = aa!(o);
                        write_of(d, out, o, v);
                        o += n_;
                    }
                }
                continue;
            }
            if t >= 1.0 {
                let mut o = i;
                for _ in 0..10 {
                    let v = bb!(o);
                    write_of(d, out, o, v);
                    o += n_;
                }
                continue;
            }
            let mut o = i;
            for _ in 0..3 {
                let v = aa!(o) + fr(fr(bb!(o) - aa!(o)) * t);
                write_of(d, out, o, v);
                o += n_;
            }
            let mut o = 7 * n_ + i;
            for _ in 7..10 {
                let v = aa!(o) + fr(fr(bb!(o) - aa!(o)) * t);
                write_of(d, out, o, v);
                o += n_;
            }
            let (mut bx, mut by, mut bz, mut bw) = (bb!(3 * n_ + i), bb!(4 * n_ + i), bb!(5 * n_ + i), bb!(6 * n_ + i));
            let (ax, ay, az, aw) = (aa!(3 * n_ + i), aa!(4 * n_ + i), aa!(5 * n_ + i), aa!(6 * n_ + i));
            if fr(fr(fr(fr(ax * bx) + fr(ay * by)) + fr(az * bz)) + fr(aw * bw)) < 0.0 {
                bx = -bx;
                by = -by;
                bz = -bz;
                bw = -bw;
            }
            let (x, y, z, ww) = (fr(ax + fr(fr(bx - ax) * t)), fr(ay + fr(fr(by - ay) * t)), fr(az + fr(fr(bz - az) * t)), fr(aw + fr(fr(bw - aw) * t)));
            let l = js::or(fr(m::sqrt(fr(fr(fr(fr(x * x) + fr(y * y)) + fr(z * z)) + fr(ww * ww)))), 1.0);
            write_of(d, out, 3 * n_ + i, x / l);
            write_of(d, out, 4 * n_ + i, y / l);
            write_of(d, out, 5 * n_ + i, z / l);
            write_of(d, out, 6 * n_ + i, ww / l);
        }
        return;
    }
    for i in 0..n {
        let t = match mask {
            Some(k) => w * at(k, i),
            None => w,
        };
        if t <= 0.0 {
            if !same {
                let mut o = i;
                for _ in 0..10 {
                    let v = aa!(o);
                    write_of(d, out, o, v);
                    o += n_;
                }
            }
            continue;
        }
        if t >= 1.0 {
            let mut o = i;
            for _ in 0..10 {
                let v = bb!(o);
                write_of(d, out, o, v);
                o += n_;
            }
            continue;
        }
        let mut o = i;
        for _ in 0..3 {
            let v = aa!(o) + (bb!(o) - aa!(o)) * t;
            write_of(d, out, o, v);
            o += n_;
        }
        let mut o = 7 * n_ + i;
        for _ in 7..10 {
            let v = aa!(o) + (bb!(o) - aa!(o)) * t;
            write_of(d, out, o, v);
            o += n_;
        }
        let (mut bx, mut by, mut bz, mut bw) = (bb!(3 * n_ + i), bb!(4 * n_ + i), bb!(5 * n_ + i), bb!(6 * n_ + i));
        let (ax, ay, az, aw) = (aa!(3 * n_ + i), aa!(4 * n_ + i), aa!(5 * n_ + i), aa!(6 * n_ + i));
        if ax * bx + ay * by + az * bz + aw * bw < 0.0 {
            bx = -bx;
            by = -by;
            bz = -bz;
            bw = -bw;
        }
        let (x, y, z, ww) = (ax + (bx - ax) * t, ay + (by - ay) * t, az + (bz - az) * t, aw + (bw - aw) * t);
        let l = js::or(m::hyp4(x, y, z, ww), 1.0);
        write_of(d, out, 3 * n_ + i, x / l);
        write_of(d, out, 4 * n_ + i, y / l);
        write_of(d, out, 5 * n_ + i, z / l);
        write_of(d, out, 6 * n_ + i, ww / l);
    }
}

// ---------------------------------------------------------------- a body's world of joints

/// Every node's global matrix and rotation at rest values, and room for the skinning matrices.   (skeleton.ts SkeletonInstance.constructor)
pub fn skeleton_instance_new(g: &mut crate::game::Game, ch: RigId) -> crate::anim_skeleton::SkeletonInstance {
    let rig = &g.tab.rig[ch];
    SkeletonInstance {
        rig: ch,
        globals: rig.nodes.iter().map(|_| v3::m4::create()).collect(),
        global_rot: rig.nodes.iter().map(|_| v3::quat::ident()).collect(),
        joint_data: vec![0.0f32; rig.skin.joints.len() * 16],
    }
}

/// Model-space matrices of all nodes from a local pose, with model-space twists and per-node overrides, then the skinning matrices.   (skeleton.ts SkeletonInstance.update)
pub fn skeleton_instance_update(g: &mut crate::game::Game, this: &mut crate::anim_skeleton::SkeletonInstance, pose: crate::anim_skeleton::Pose, world: &crate::v3::m4::Mat4, twists: Option<&[crate::anim_skeleton::TwistSpec]>, overrides: Option<&[Option<crate::anim_skeleton::NodeOverride>]>) {
    let rig = &g.tab.rig[this.rig];
    let d = cols_at(g.sim, pose);
    let n_ = rows_of(pose);
    // several layers may push a twist for the same node: they accumulate
    let mut twist_map: Option<JsMap<usize, TwistSpec>> = None;
    if let Some(ts) = twists {
        if !ts.is_empty() {
            let mut map: JsMap<usize, TwistSpec> = JsMap::new();
            for t in ts {
                let k = t.node as usize;
                let e = map.get(k).copied();
                match e {
                    Some(e) => {
                        map.set(k, TwistSpec { node: t.node, yaw: e.yaw + t.yaw, pitch: e.pitch + t.pitch });
                    }
                    None => {
                        map.set(k, *t);
                    }
                }
            }
            twist_map = Some(map);
        }
    }
    for &oi in &rig.order {
        let i = oi as usize;
        let parent = rig.nodes[i].parent;
        let (rx, ry, rz, rw) = (d[3 * n_ + i] as f64, d[4 * n_ + i] as f64, d[5 * n_ + i] as f64, d[6 * n_ + i] as f64);
        let (tx, ty, tz, sx, sy, sz) = (d[i] as f64, d[n_ + i] as f64, d[2 * n_ + i] as f64, d[7 * n_ + i] as f64, d[8 * n_ + i] as f64, d[9 * n_ + i] as f64);
        let ov = overrides.and_then(|o| o.get(i).copied().flatten());
        if let Some(ov) = ov {
            // driven from outside: global rotation as given, origin as given or where the parent puts it
            let t: V3 = [tx, ty, tz];
            let p = match ov.pos {
                Some(p) => p,
                None => {
                    if parent >= 0.0 {
                        v3::m4::transform_point(&this.globals[parent as usize], t)
                    } else {
                        t
                    }
                }
            };
            this.global_rot[i] = ov.rot;
            this.globals[i] = v3::m4::from_trss(p[0], p[1], p[2], ov.rot[0], ov.rot[1], ov.rot[2], ov.rot[3], sx, sy, sz);
            continue;
        }
        let parent_rot = if parent >= 0.0 { this.global_rot[parent as usize] } else { ident() };
        let (ax, ay, az, aw) = (parent_rot[0], parent_rot[1], parent_rot[2], parent_rot[3]);
        let tw = twist_map.as_ref().and_then(|m| m.get(i).copied());
        let mut gr: Quat = [aw * rx + ax * rw + ay * rz - az * ry, aw * ry - ax * rz + ay * rw + az * rx, aw * rz + ax * ry - ay * rx + az * rw, aw * rw - ax * rx - ay * ry - az * rz];
        let mut r: Option<Quat> = None;
        if let Some(tw) = tw {
            // model-space yaw about +Y then pitch about the yawed model X axis, applied to this subtree
            let mut extra = v3::quat::yaw(tw.yaw);
            if tw.pitch != 0.0 {
                let right = v3::quat::rotate(extra, [1.0, 0.0, 0.0]);
                extra = v3::quat::mul(v3::quat::axis_angle(right, tw.pitch), extra);
            }
            gr = v3::quat::normalize(v3::quat::mul(extra, gr));
            // back to a local rotation consistent with the new global: r = parentRot^-1 * gr
            r = Some(v3::quat::mul(v3::quat::conj(parent_rot), gr));
        }
        this.global_rot[i] = gr;
        let local = match r {
            Some(r) => v3::m4::from_trss(tx, ty, tz, r[0], r[1], r[2], r[3], sx, sy, sz),
            None => v3::m4::from_trss(tx, ty, tz, rx, ry, rz, rw, sx, sy, sz),
        };
        this.globals[i] = if parent >= 0.0 { v3::m4::mul(&this.globals[parent as usize], &local) } else { v3::m4::copy(&local) };
    }
    // skinning matrices (world · global · inverse bind, written straight into the joint's slice of joint_data)
    let joints = &rig.skin.joints;
    for j in 0..joints.len() {
        let mut ibm: v3::m4::Mat4 = [0.0; 16];
        ibm.copy_from_slice(&rig.skin.inverse_bind[j * 16..j * 16 + 16]);
        let tmp = v3::m4::mul(&this.globals[joints[j] as usize], &ibm);
        let out = v3::m4::mul(world, &tmp);
        this.joint_data[j * 16..j * 16 + 16].copy_from_slice(&out);
    }
}

/// Model-space origin of a node.   (skeleton.ts SkeletonInstance.nodePos)
pub fn skeleton_instance_node_pos(this: &mut crate::anim_skeleton::SkeletonInstance, node: f64) -> V3 {
    let g = &this.globals[node as usize];
    [g[12] as f64, g[13] as f64, g[14] as f64]
}

/// Rewrite the rotations of `pose` so that plain FK reproduces the globals of the last update: parent⁻¹ · global per node.   (skeleton.ts SkeletonInstance.bakeLocalRotations)
pub fn skeleton_instance_bake_local_rotations(g: &mut crate::game::Game, this: &mut crate::anim_skeleton::SkeletonInstance, pose: crate::anim_skeleton::Pose) {
    let rig = &g.tab.rig[this.rig];
    let n_ = rows_of(pose);
    let d = cols_at(g.sim, pose);
    for &oi in &rig.order {
        let i = oi as usize;
        let parent = rig.nodes[i].parent;
        let r = if parent >= 0.0 { v3::quat::mul(v3::quat::conj(this.global_rot[parent as usize]), this.global_rot[i]) } else { this.global_rot[i] };
        d[3 * n_ + i] = r[0] as f32;
        d[4 * n_ + i] = r[1] as f32;
        d[5 * n_ + i] = r[2] as f32;
        d[6 * n_ + i] = r[3] as f32;
    }
}

/// buildMask's setSub: the weight on a node and all under it.
fn set_sub(nodes: &[crate::anim_gltf::GltfNode], mask: &mut [f32], i: usize, w: f64) {
    mask[i] = w as f32;
    for &c in &nodes[i].children {
        set_sub(nodes, mask, c as usize, w);
    }
}

/// A per-node mask: a weight for listed node names and (optionally) all their descendants.   (skeleton.ts buildMask)
pub fn build_mask(g: &mut crate::game::Game, ch: RigId, entries: &[crate::anim_skeleton::BuildMaskEntriesItem]) -> Vec<f32> {
    let rig = &g.tab.rig[ch];
    let mut mask = vec![0.0f32; rig.nodes.len()];
    for e in entries {
        let Some(&i) = rig.node_by_name.get(e.name.clone()) else {
            continue;
        };
        if e.subtree {
            set_sub(&rig.nodes, &mut mask, i as usize, e.w);
        } else {
            mask[i as usize] = e.w as f32;
        }
    }
    mask
}

// ---------------------------------------------------------------- forward kinematics of a local pose

/// Every node's rotation the identity and its origin nought.   (skeleton.ts PoseFK.constructor)
pub fn pose_fk_new(g: &mut crate::game::Game, ch: RigId) -> crate::anim_skeleton::PoseFK {
    let rig = &g.tab.rig[ch];
    PoseFK {
        rot: rig.nodes.iter().map(|_| v3::quat::ident()).collect(),
        pos: rig.nodes.iter().map(|_| [0.0, 0.0, 0.0]).collect(),
        rig: ch,
        ..PoseFK::blank()
    }
}

/// Model-space rotation and origin of every node from a local pose, written in place: a question of the module, or this file's own arithmetic for a rig the bench cannot hold.   (skeleton.ts PoseFK.run)
pub fn pose_fk_run(g: &mut crate::game::Game, this: &mut crate::anim_skeleton::PoseFK, pose: crate::anim_skeleton::Pose) {
    if bench_rig(g, this.rig, None).is_some() {
        bench_put_fk(this);
        let (slot, place) = lent_at(g, pose, 0.0);
        known(answer(::sim::skeleton::ask_run(slot, place, -1)));
        bench_get_fk(this);
        return;
    }
    fk_own(this, cols_at(g.sim, pose), rows_of(pose), &g.tab.rig[this.rig]);
}

/// PoseFK.run's own arithmetic over a pose's singles (`n_` rows a column).
fn fk_own(this: &mut crate::anim_skeleton::PoseFK, d: &[f32], n_: usize, rig: &crate::anim_gltf::GltfCharacter) {
    for &oi in &rig.order {
        let i = oi as usize;
        let parent = rig.nodes[i].parent;
        let (rx, ry, rz, rw) = (d[3 * n_ + i] as f64, d[4 * n_ + i] as f64, d[5 * n_ + i] as f64, d[6 * n_ + i] as f64);
        if parent < 0.0 {
            this.rot[i] = [rx, ry, rz, rw];
            this.pos[i] = [d[i] as f64, d[n_ + i] as f64, d[2 * n_ + i] as f64];
            continue;
        }
        let (pr, pp) = (this.rot[parent as usize], this.pos[parent as usize]);
        let (qx, qy, qz, qw) = (pr[0], pr[1], pr[2], pr[3]);
        let (v0, v1, v2) = (d[i] as f64, d[n_ + i] as f64, d[2 * n_ + i] as f64);
        let (tx, ty, tz) = (2.0 * (qy * v2 - qz * v1), 2.0 * (qz * v0 - qx * v2), 2.0 * (qx * v1 - qy * v0)); // t = quat.rotate(pr, t_local)
        let (t0, t1, t2) = (v0 + qw * tx + (qy * tz - qz * ty), v1 + qw * ty + (qz * tx - qx * tz), v2 + qw * tz + (qx * ty - qy * tx));
        this.rot[i] = [qw * rx + qx * rw + qy * rz - qz * ry, qw * ry - qx * rz + qy * rw + qz * rx, qw * rz + qx * ry - qy * rx + qz * rw, qw * rw - qx * rx - qy * ry - qz * rz]; // rot = quat.mul(pr, r)
        this.pos[i] = [pp[0] + t0, pp[1] + t1, pp[2] + t2];
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::sync::atomic::{AtomicBool, Ordering};

    // rust/sim's pool and bench are statics: one test at a time holds them.
    static HELD: AtomicBool = AtomicBool::new(false);
    struct Held;
    impl Drop for Held {
        fn drop(&mut self) {
            HELD.store(false, Ordering::Release);
        }
    }
    fn hold() -> (Held, crate::game::Game) {
        while HELD.swap(true, Ordering::Acquire) {
            core::hint::spin_loop();
        }
        let held = Held;
        let g = crate::game::Game::blank();
        ::sim::poses::clear_of(g.sim);
        (held, g)
    }

    // The expected words are the TypeScript's own bits (src/anim/skeleton.ts under bun, EXACT.on, RUST.poses off so its own arithmetic ran): FNV-1a over each number's bits.
    struct H(u32);
    impl H {
        fn new() -> H {
            H(0x811c9dc5)
        }
        fn word(&mut self, w: u32) {
            self.0 = (self.0 ^ w).wrapping_mul(0x01000193);
        }
        fn fs(&mut self, a: &[f32]) {
            for v in a {
                self.word(v.to_bits());
            }
        }
        fn fd(&mut self, a: &[f64]) {
            for v in a {
                let b = v.to_bits();
                self.word(b as u32);
                self.word((b >> 32) as u32);
            }
        }
    }
    fn folded(a: &[f32]) -> u32 {
        let mut h = H::new();
        h.fs(a);
        h.0
    }
    fn val(i: usize, s: usize) -> f64 {
        ((i * s + 11) % 101) as f64 / 101.0 - 0.5
    }
    /// the singles of the test's pose of five nodes (eight rows a column)
    fn cols(s: usize) -> Vec<f32> {
        (0..80).map(|i| if i % 8 < 5 { val(i, s) as f32 } else { 0.0 }).collect()
    }
    fn pose_of(g: &mut crate::game::Game, s: usize) -> Pose {
        let p = pose_new(g, 5);
        pose_cols(g, p).copy_from_slice(&cols(s));
        p
    }
    const MASK: [f32; 5] = [0.0, 0.25, 0.5, 1.0, 1.5];
    const WORDS: [(bool, u32, u32); 2] = [(false, 0x0700ea78, 0xeb7598f8), (true, 0x34134ae8, 0x6f845fa1)];

    #[test]
    fn blend_is_the_typescripts_bits() {
        let (a, b, o) = ((0, 80), (80, 80), (160, 80));
        let store = |x: &[f32]| {
            let mut d = cols(37);
            d.extend(cols(53));
            d.extend_from_slice(x);
            d
        };
        for (singles, plain, masked) in WORDS {
            let mut d = store(&[0.0; 80]);
            blend_own(&mut d, a, b, o, 5, 8, 0.37, None, singles);
            assert_eq!(folded(&d[160..]), plain);
            let mut d = store(&cols(71));
            blend_own(&mut d, a, b, o, 5, 8, 0.8, Some(&MASK), singles);
            assert_eq!(folded(&d[160..]), masked);
            let mut d = store(&[]);
            blend_own(&mut d, a, b, a, 5, 8, 0.8, Some(&MASK), singles);
            assert_eq!(folded(&d[..80]), masked);
        }
    }

    #[test]
    fn the_module_finds_a_pose_by_its_place_and_blends_the_same_bits() {
        let (_h, mut g) = hold();
        for (singles, plain, _) in WORDS {
            let (a, b, o) = (pose_of(&mut g, 37), pose_of(&mut g, 53), pose_new(&mut g, 5));
            assert!(::sim::skeleton::ask_blend(g.sim, a.at as usize, b.at as usize, 0.37, false, o.at as usize, 5, singles));
            assert_eq!(folded(pose_cols(&mut g, o)), plain);
            // the poses either side are as they were
            assert_eq!(folded(pose_cols(&mut g, a)), folded(&cols(37)));
            assert_eq!(folded(pose_cols(&mut g, b)), folded(&cols(53)));
        }
    }

    fn track(node: f64, path: f64, times: &[f32], values: &[f32], step: bool) -> crate::anim_gltf::Track {
        crate::anim_gltf::Track { node, path, times: times.to_vec(), values: values.to_vec(), step }
    }

    #[test]
    fn sample_is_the_typescripts_bits() {
        let clip = crate::anim_gltf::Clip {
            name: String::from("c"),
            duration: 1.25,
            tracks: vec![
                track(0.0, 0.0, &[0.0, 0.5, 1.0], &[0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9], false),
                track(1.0, 1.0, &[0.0, 0.5, 1.0], &[0.5, 0.5, 0.5, 0.5, -0.6, 0.2, -0.3, -0.7, 0.1, 0.9, 0.3, 0.2], false),
                track(2.0, 2.0, &[0.25, 0.75], &[1.0, 1.0, 1.0, 2.0, 2.0, 2.0], true),
                track(3.0, 1.0, &[0.2], &[0.0, 0.0, 0.0, 1.0], false),
                track(9.0, 0.0, &[0.0, 1.0], &[9.0, 9.0, 9.0, 8.0, 8.0, 8.0], false),
                track(4.0, 2.0, &[0.25, 0.75], &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0], false),
            ],
        };
        let at = [(0.7, true), (-0.2, true), (5.1, false), (-1.0, false), (0.5, true), (1.1, false)];
        let doubles = [0xe915e2dfu32, 0xdf585637, 0xdf585637, 0x6d0a0095, 0xe990b27b, 0xdf585637];
        let in_singles = [0xc428c977u32, 0xdf585637, 0xdf585637, 0x6d0a0095, 0xe990b27b, 0xdf585637];
        for k in 0..at.len() {
            let mut o = cols(37);
            sample_own(&clip, at[k].0, at[k].1, &mut o, 8, 5.0, false);
            assert_eq!(folded(&o), doubles[k], "doubles {}", k);
            let mut o = cols(37);
            sample_own(&clip, at[k].0, at[k].1, &mut o, 8, 5.0, true);
            assert_eq!(folded(&o), in_singles[k], "singles {}", k);
        }
    }

    #[test]
    fn a_clip_written_into_the_module_samples_the_same_bits() {
        let (_h, mut g) = hold();
        // (sample_is_the_typescripts_bits' clip less its track of node 9, which writes nothing over a pose of five nodes and which the module would refuse)
        let clip = crate::anim_gltf::Clip {
            name: String::from("c"),
            duration: 1.25,
            tracks: vec![
                track(0.0, 0.0, &[0.0, 0.5, 1.0], &[0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9], false),
                track(1.0, 1.0, &[0.0, 0.5, 1.0], &[0.5, 0.5, 0.5, 0.5, -0.6, 0.2, -0.3, -0.7, 0.1, 0.9, 0.3, 0.2], false),
                track(2.0, 2.0, &[0.25, 0.75], &[1.0, 1.0, 1.0, 2.0, 2.0, 2.0], true),
                track(3.0, 1.0, &[0.2], &[0.0, 0.0, 0.0, 1.0], false),
                track(4.0, 2.0, &[0.25, 0.75], &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0], false),
            ],
        };
        let at = clip_write(&clip).unwrap() as usize;
        assert_eq!(clip_write(&clip).unwrap() as usize, at);
        let p = pose_new(&mut g, 5);
        let times = [(0.7, true), (-0.2, true), (5.1, false), (-1.0, false), (0.5, true), (1.1, false)];
        let doubles = [0xe915e2dfu32, 0xdf585637, 0xdf585637, 0x6d0a0095, 0xe990b27b, 0xdf585637];
        let in_singles = [0xc428c977u32, 0xdf585637, 0xdf585637, 0x6d0a0095, 0xe990b27b, 0xdf585637];
        for k in 0..times.len() {
            for (singles, word) in [(false, doubles[k]), (true, in_singles[k])] {
                pose_cols(&mut g, p).copy_from_slice(&cols(37));
                assert!(::sim::skeleton::ask_sample(g.sim, p.at as usize, 5, at, times[k].0, times[k].1, singles));
                assert_eq!(folded(pose_cols(&mut g, p)), word, "{} singles {}", k, singles);
            }
        }
    }

    fn rig() -> crate::anim_gltf::GltfCharacter {
        let mut ch = crate::anim_gltf::GltfCharacter::zeroed();
        for parent in [-1.0, 0.0, 1.0, 1.0, 3.0] {
            ch.nodes.push(crate::anim_gltf::GltfNode { parent, t: [0.0; 3], r: [0.0, 0.0, 0.0, 1.0], s: [1.0; 3], ..crate::anim_gltf::GltfNode::zeroed() });
        }
        ch.order = vec![0.0, 1.0, 2.0, 3.0, 4.0];
        ch.skin.joints = vec![0.0, 1.0, 2.0, 3.0, 4.0];
        ch.skin.inverse_bind = (0..80).map(|i| val(i, 19) as f32).collect();
        ch
    }

    #[test]
    fn fk_is_the_typescripts_bits() {
        let ch = rig();
        let mut fk = PoseFK { rot: vec![v3::quat::ident(); 5], pos: vec![[0.0; 3]; 5], ..PoseFK::blank() };
        fk_own(&mut fk, &cols(37), 8, &ch);
        let mut h = H::new();
        for i in 0..5 {
            h.fd(&fk.rot[i]);
            h.fd(&fk.pos[i]);
        }
        assert_eq!(h.0, 0x297fc6f0);
    }

    #[test]
    fn update_and_bake_are_the_typescripts_bits() {
        let (_h, mut g) = hold();
        let id = g.tab.rig.push(rig());
        let mut si = SkeletonInstance { rig: id, globals: vec![v3::m4::create(); 5], global_rot: vec![v3::quat::ident(); 5], joint_data: vec![0.0; 80] };
        let w = v3::m4::from_trss(1.0, 2.0, 3.0, 0.0, 0.6, 0.0, 0.8, 1.0, 1.0, 1.0);
        let twists = [TwistSpec { node: 1.0, yaw: 0.3, pitch: 0.2 }, TwistSpec { node: 2.0, yaw: -0.4, pitch: 0.0 }, TwistSpec { node: 1.0, yaw: 0.1, pitch: 0.05 }];
        let overrides = [None, None, None, Some(NodeOverride { rot: [0.0, 0.0, 0.6, 0.8], pos: None }), Some(NodeOverride { rot: [0.6, 0.0, 0.0, 0.8], pos: Some([1.0, 2.0, 3.0]) })];
        let p37 = pose_of(&mut g, 37);
        skeleton_instance_update(&mut g, &mut si, p37, &w, Some(&twists), Some(&overrides));
        let mut h = H::new();
        h.fs(&si.joint_data);
        for i in 0..5 {
            h.fd(&si.global_rot[i]);
            h.fs(&si.globals[i]);
        }
        assert_eq!(h.0, 0xbf83741e);
        skeleton_instance_bake_local_rotations(&mut g, &mut si, p37);
        assert_eq!(folded(pose_cols(&mut g, p37)), 0x966af111);
        let p53 = pose_of(&mut g, 53);
        skeleton_instance_update(&mut g, &mut si, p53, &w, None, None);
        assert_eq!(folded(&si.joint_data), 0x29e003ec);
    }

    #[test]
    fn a_pose_goes_onto_the_bench_and_back_by_its_own_nodes() {
        let (_h, mut g) = hold();
        let p = pose_of(&mut g, 37);
        bench_put(&mut g, p, 1.0);
        let back = pose_new(&mut g, 5);
        bench_get(&mut g, back, 1.0);
        let (from, to) = (cols(37), pose_cols(&mut g, back).to_vec());
        for c in 0..10 {
            for i in 0..8 {
                let at = c * 8 + i;
                assert_eq!(to[at].to_bits(), if i < 5 { from[at].to_bits() } else { 0 });
            }
        }
    }

    #[test]
    fn a_pose_is_ten_columns_at_its_place() {
        let (_h, mut g) = hold();
        let (p, q) = (pose_new(&mut g, 5), pose_new(&mut g, 5));
        assert_eq!(q.at - p.at, 80);
        assert_eq!(pose_cols(&mut g, p).len(), 80);
        pose_set_rot(&mut g, p, 2.0, [0.5, 0.25, -0.5, 1.0]);
        pose_set_rot(&mut g, p, 5.0, [9.0; 4]); // no such node
        pose_cols(&mut g, p)[8 + 2] = 3.0;
        assert_eq!(pose_rot(&mut g, p, 2.0), [0.5, 0.25, -0.5, 1.0]);
        assert_eq!(pose_pos(&mut g, p, 2.0), [0.0, 3.0, 0.0]);
        assert_eq!(pose_cols(&mut g, p).iter().filter(|v| **v != 0.0).count(), 5);
        pose_copy(&mut g, q, p);
        assert_eq!(pose_rot(&mut g, q, 2.0), [0.5, 0.25, -0.5, 1.0]);
        pose_copy(&mut g, q, q);
        assert_eq!(pose_rot(&mut g, q, 2.0), [0.5, 0.25, -0.5, 1.0]);
        // the empty pose and the blank handle ask for no floats
        let none = pose_new(&mut g, 0);
        assert!(none.at >= Pose::LOOSE);
        assert_eq!(pose_cols(&mut g, none).len(), 0);
        assert_eq!(pose_cols(&mut g, Pose::blank()).len(), 0);
    }

    #[test]
    #[should_panic(expected = "no place")]
    fn a_pose_with_no_place_is_said_by_name() {
        let (_h, mut g) = hold();
        pose_cols(&mut g, Pose { at: Pose::LOOSE, nodes: 5 });
    }

    #[test]
    fn a_carried_pose_takes_a_place_and_its_floats() {
        let (_h, mut g) = hold();
        g.loose_poses.push(LoosePose { nodes: 5, floats: Vec::new() });
        g.loose_poses.push(LoosePose { nodes: 5, floats: cols(37) });
        let mut p = Pose { at: Pose::LOOSE + 1, nodes: 5 };
        assert!(!pose_in(&mut g, p));
        pose_take_home(&mut g, &mut p);
        assert!(pose_in(&mut g, p));
        assert_eq!(p.nodes, 5);
        assert_eq!(folded(pose_cols(&mut g, p)), folded(&cols(37)));
        // one at home stands
        let was = p;
        pose_take_home(&mut g, &mut p);
        assert_eq!(p, was);
        // one made with no place has no row, and stands
        let mut none = pose_new(&mut g, 0);
        let was = none;
        pose_take_home(&mut g, &mut none);
        assert_eq!(none, was);
        // a pool with no room leaves it loose
        let mut full = Pose { at: Pose::LOOSE + 1, nodes: 5 };
        ::sim::state::state(g.sim).poses.live = (::sim::poses::HEAD + ::sim::poses::POOL * 4) as u32;
        pose_take_home(&mut g, &mut full);
        ::sim::state::state(g.sim).poses.live = 0;
        assert_eq!(full.at, Pose::LOOSE + 1);
    }

    #[test]
    fn rest_pose_fills_the_rigs_nodes() {
        let (_h, mut g) = hold();
        let mut ch = rig();
        ch.nodes[1].t = [1.0, 2.0, 3.0];
        ch.nodes[1].r = [0.5, 0.5, 0.5, 0.5];
        ch.nodes[1].s = [4.0, 5.0, 6.0];
        let id = g.tab.rig.push(ch);
        let p = pose_new(&mut g, 5);
        rest_pose(&mut g, id, p);
        assert_eq!(pose_pos(&mut g, p, 1.0), [1.0, 2.0, 3.0]);
        assert_eq!(pose_rot(&mut g, p, 1.0), [0.5, 0.5, 0.5, 0.5]);
        assert_eq!(pose_cols(&mut g, p)[8 * 9 + 1], 6.0);
    }

    #[test]
    fn a_set_holds_its_poses_and_counts_what_has_no_place() {
        let (_h, mut g) = hold();
        let s = pose_set_new(&mut g, 2.0);
        assert_eq!(pose_set_most(&mut g, s), 0.0);
        let a = pose_set_make(&mut g, s, 5.0);
        assert_eq!(g.tab.pose_set[s].list, vec![a]);
        assert_eq!(pose_set_most(&mut g, s), 160.0);
        assert!(pose_set_in(&mut g, s));
        assert!(g.tab.pose_set[s].home.is_some());
        // a pool with no room: the pose has no place, and the set says so
        ::sim::state::state(g.sim).poses.live = (::sim::poses::HEAD + ::sim::poses::POOL * 4) as u32;
        let b = pose_set_make(&mut g, s, 5.0);
        ::sim::state::state(g.sim).poses.live = 0;
        assert!(b.at >= Pose::LOOSE);
        assert_eq!(g.tab.pose_set[s].stray, 1.0);
        assert!(!pose_set_in(&mut g, s));
        pose_set_out(&mut g, s);
        assert!(g.tab.pose_set[s].home.is_none());
        assert_eq!(g.tab.pose_set[s].stray, 0.0);
    }

    #[test]
    #[should_panic(expected = "asked for one more")]
    fn a_set_past_its_most_is_said_by_name() {
        let (_h, mut g) = hold();
        let s = pose_set_new(&mut g, 1.0);
        pose_set_make(&mut g, s, 5.0);
        pose_set_make(&mut g, s, 5.0);
    }

    #[test]
    fn a_clips_lines_are_its_sets_of_times() {
        let mut g = crate::game::Game::blank();
        let clip = g.tab.clip.push(crate::anim_gltf::Clip {
            name: String::from("c"),
            duration: 1.0,
            tracks: vec![
                track(4.0, 0.0, &[0.0, 1.0], &[0.0; 6], false),
                track(1.0, 1.0, &[0.0, 0.5], &[0.0; 8], false),
                track(2.0, 2.0, &[0.0, 1.0], &[0.0; 6], false),
                track(0.0, 0.0, &[-0.0, 1.0], &[0.0; 6], false),
            ],
        });
        let c = side_of(&mut g, clip);
        assert_eq!(c.lines, vec![0, 1, 0, 2]);
        assert_eq!(c.at, -2.0);
        assert_eq!(c.need, 5.0);
    }

    #[test]
    fn a_mask_reaches_a_subtree() {
        let mut g = crate::game::Game::blank();
        let mut ch = rig();
        ch.nodes[1].children = vec![2.0, 3.0];
        ch.nodes[3].children = vec![4.0];
        ch.node_by_name.set(String::from("arm"), 3.0);
        ch.node_by_name.set(String::from("root"), 0.0);
        let id = g.tab.rig.push(ch);
        let e = |name: &str, w: f64, subtree: bool| BuildMaskEntriesItem { name: String::from(name), w, subtree };
        let mask = build_mask(&mut g, id, &[e("arm", 0.5, true), e("root", 1.0, false), e("none", 1.0, true)]);
        assert_eq!(mask, vec![1.0, 0.0, 0.0, 0.5, 0.5]);
    }
}
