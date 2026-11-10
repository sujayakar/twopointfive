pub use ::game_types::anim_skeleton::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{string::String, vec, vec::Vec};

// ---------------------------------------------------------------- the bench (rust/sim/src/skeleton.rs Bench: its tables are reached directly, no addresses)

/// The bench has room for a pose of `n` nodes.
fn bench_holds(n: f64) -> bool { n <= BENCH_NODES }

/// A size a layer of the module is handed at most: one past it is a mistake in the game's own tables, said by name.
pub(super) fn layer_fits(what: &str, n: f64, max: f64, min: Option<f64>) {
    let min = min.unwrap_or(0.0);
    if !(n >= min && n <= max) {
        if min > 0.0 {
            panic!("posing: {} has {} entries and a layer of the Rust module takes {} to {} (rust/sim/src/layers and knobs.rs hold the lengths)", what, n, min, max);
        }
        panic!("posing: {} has {} entries and a layer of the Rust module takes at most {} (rust/sim/src/layers and knobs.rs hold the lengths)", what, n, max);
    }
}

/// A table of names of rust/sim/src/knobs.rs, one a line.
fn bench_names(table: f64) -> Vec<String> {
    let table = table as u32;
    let (at, n) = (::sim::roles::addr(table), ::sim::roles::len(table));
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

/// The slot to ask a layer of man `c` in: his game's block, all his poses standing there, and his rig on the bench with its roles, lists and tables.
pub(super) fn layer_slot(g: &mut crate::game::Game, c: CharId) -> f64 {
    let (ch, set) = (g.ch[c].anim.rig, g.ch[c].anim.poses);
    let stray = g.tab.pose_set[set].stray;
    if stray != 0.0 {
        panic!("posing: a man has {} pose(s) the pose pool had no room for, and a layer is posed only where all of his stand in the module (game/posehome.ts holds the pool's size)", stray);
    }
    let slot = g.sim as f64;
    if !bench_rig(g, ch) {
        panic!("posing: a layer was asked of a rig the module's bench does not hold (anim/skeleton.ts benchRig: the rig's nodes or its order are more than the bench has room for)");
    }
    if g.m.anim_skeleton.layers.roles != Some(ch) {
        if g.m.anim_skeleton.layers.names.is_none() {
            let names = AnimSkeletonModLAYERSNames { roles: bench_names(3.0), lists: bench_names(5.0), nums: bench_names(8.0) };
            g.m.anim_skeleton.layers.names = Some(names);
        }
        let names = g.m.anim_skeleton.layers.names.clone().unwrap();
        let cast = g.ch[c].anim.cast.clone();
        let cast = &cast;
        let w_role = ::sim::roles::addr(2) as *mut u32;
        let w_list = ::sim::roles::addr(4) as *mut u32;
        let w_len = ::sim::roles::addr(6) as *mut u32;
        let max = ::sim::roles::len(4) / core::cmp::max(1, ::sim::roles::len(6));
        let nmax = ::sim::roles::len(7) / core::cmp::max(1, ::sim::roles::len(9));
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
        let m_num = ::sim::roles::addr(7) as *mut f64;
        let w_num = ::sim::roles::addr(9) as *mut u32;
        for i in 0..names.nums.len() {
            let t = cast_nums(cast, &names.nums[i]).unwrap();
            for k in 0..t.len() {
                unsafe { m_num.add(i * nmax + k).write(t[k]) };
            }
            unsafe { w_num.add(i).write(t.len() as u32) };
        }
        g.m.anim_skeleton.layers.roles = Some(ch);
    }
    slot
}

/// What a layer's export answered: 0 is a mistake and loud; else the run-forward bones it ran are brought back from the bench into the man's `fk`, and no others.
pub(super) fn layer_ran(g: &mut crate::game::Game, code: f64, c: CharId) {
    if code == 0.0 {
        panic!("posing: a layer of the Rust module could not run: it was handed a pose, a node of the rig, a clip or a table of keys the module does not hold: char row {:?}, unit {}, guard {:?}", c, g.guards.iter().any(|x| g.gd[*x].ch == c && g.gd[*x].unit.is_some()), g.guards.iter().find(|x| g.gd[**x].ch == c).map(|x| g.gd[*x].callsign.clone()));
    }
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
            let b = ::sim::skeleton::bench_read();
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

/// `ch`'s rig on the bench, written when another was there; false: a rig the bench cannot hold.
fn bench_rig(g: &mut crate::game::Game, ch: RigId) -> bool {
    let n = g.art.rig[ch].nodes.len();
    if !bench_holds(n as f64) { return false; }
    if g.m.anim_skeleton.pose_bench.rig != Some(ch) {
        g.m.anim_skeleton.pose_bench.rig = None;
        let rig = &g.art.rig[ch];
        if rig.order.len() > n {
            return false;
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
            return false;
        }
        g.m.anim_skeleton.pose_bench.rig = Some(ch);
    }
    true
}

/// A pose written to place `slot` of the bench.
fn bench_put(g: &crate::game::Game, p: crate::anim_skeleton::Pose, slot: f64) {
    let o = (slot * BENCH_SLOT) as usize;
    let d = cols_of_read(g, p);
    let b = ::sim::skeleton::bench();
    for k in 0..d.len() {
        ::sim::single::put(&mut b.pose, o + k, d[k] as f64);
    }
}

/// A pose read back from place `slot`: each column's rows of the pose's own nodes, the rows past the last node staying its zeros.
fn bench_get(g: &mut crate::game::Game, p: crate::anim_skeleton::Pose, slot: f64) {
    let o = (slot * BENCH_SLOT) as usize;
    let (n_, n) = (rows_of(p), p.nodes as usize);
    let d = cols_of(g, p);
    let b = ::sim::skeleton::bench_read();
    for c in 0..10 {
        for i in 0..n {
            d[c * n_ + i] = ::sim::single::get(&b.pose, o + c * n_ + i) as f32;
        }
    }
}

/// The run-forward bones onto the bench.
fn bench_put_fk(fk: &crate::anim_skeleton::PoseFK) {
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

/// The run-forward bones back off the bench.
fn bench_get_fk(fk: &mut crate::anim_skeleton::PoseFK) {
    let b = ::sim::skeleton::bench_read();
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

/// Rows a column of a pose of `n` nodes holds: `n` rounded up to whole vectors of four.
pub(super) fn pose_rows(n: f64) -> f64 {
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
/// The same pool, to read.
fn pool_at_read(slot: usize) -> &'static [f32] {
    let data = &::sim::state::state_read(slot).poses.data;
    unsafe { core::slice::from_raw_parts(data.as_ptr() as *const f32, data.len()) }
}

/// A pose's singles. Never hold two of these for one pose: a body that works on poses that may be one pose uses pool_at and spans.
fn cols_at(slot: usize, p: Pose) -> &'static mut [f32] {
    let (s, n) = span_of(p);
    &mut pool_at(slot)[s..s + n]
}
/// A pose's singles, to read.
fn cols_at_read(slot: usize, p: Pose) -> &'static [f32] {
    let (s, n) = span_of(p);
    &pool_at_read(slot)[s..s + n]
}

/// A pose's singles WHEREVER THEY ARE, as the TypeScript's `cols`: in the pool, or, for a pose of its own (a function's scratch, a ragdoll's: a plain
/// `new Pose`, which never moves into the pool), in its row of `g.loose_poses`. The row's floats are its own heap, which stands while the row does
/// wherever the list of rows moves; like a pool slice, the answer is for use now and is never kept.
fn cols_of(g: &mut crate::game::Game, p: Pose) -> &'static mut [f32] {
    if p.nodes != 0 && p.at >= Pose::LOOSE {
        let n = ::sim::skeleton::singles(p.nodes as usize);
        if let Some(r) = g.loose_poses.get_mut((p.at - Pose::LOOSE) as usize) {
            if r.nodes == p.nodes && r.floats.len() == n { return unsafe { core::slice::from_raw_parts_mut(r.floats.as_mut_ptr(), n) }; }
        }
    }
    cols_at(g.sim, p)
}

/// A pose's singles wherever they are, to read.
fn cols_of_read(g: &crate::game::Game, p: Pose) -> &'static [f32] {
    if p.nodes != 0 && p.at >= Pose::LOOSE {
        let n = ::sim::skeleton::singles(p.nodes as usize);
        if let Some(r) = g.loose_poses.get((p.at - Pose::LOOSE) as usize) {
            if r.nodes == p.nodes && r.floats.len() == n { return unsafe { core::slice::from_raw_parts(r.floats.as_ptr(), n) }; }
        }
    }
    cols_at_read(g.sim, p)
}

/// A pose of `num_nodes` nodes, all zeros, OF ITS OWN: a row of `g.loose_poses`, as the TypeScript's `new Pose` is an array of its own. Only a pose
/// made through a character's set lives in the game's pool (pose_set_make): so the pool holds what the TypeScript's holds, and no more. No nodes
/// makes a pose with no place and no floats.
pub(super) fn pose_new(g: &mut crate::game::Game, num_nodes: usize) -> crate::anim_skeleton::Pose {
    if num_nodes == 0 { return Pose { at: u32::MAX, nodes: 0 }; }
    let row = g.loose_poses.len() as u32;
    if row >= 0xfffe { panic!("posing: {} poses of their own stand outside the pool: somebody makes one a tick and never lets it go (pose_done)", row); }
    g.loose_poses.push(LoosePose { nodes: num_nodes as u32, floats: alloc::vec![0.0; ::sim::skeleton::singles(num_nodes)] });
    Pose { at: Pose::LOOSE + row, nodes: num_nodes as u32 }
}

/// A function's scratch pose let go (the TypeScript drops the object): its row is freed when it is the last, which a scratch made and dropped inside
/// one function is. A pose in the pool, or a row with later rows after it, stands.
pub(super) fn pose_done(g: &mut crate::game::Game, p: crate::anim_skeleton::Pose) {
    if p.nodes != 0 && p.at >= Pose::LOOSE && (p.at - Pose::LOOSE) as usize + 1 == g.loose_poses.len() { g.loose_poses.pop(); }
}

/// A pose of a set, in its game's pool, zeros. A pool with no room leaves it a pose of its own, as the TypeScript's stays its own array (the set
/// counts it a stray).
fn pose_pool_new(g: &mut crate::game::Game, num_nodes: usize) -> crate::anim_skeleton::Pose {
    let at = ::sim::poses::new_of(g.sim, num_nodes);
    if at >= Pose::LOOSE && num_nodes != 0 { return pose_new(g, num_nodes); }
    Pose { at, nodes: num_nodes as u32 }
}

/// A pose of its own takes a place in the pool and its floats (a man's set coming home; a pose the bridge carried from an array of its own). One in
/// the pool stands as it is, and so does one with no row. A pool with no room leaves it where it is.
pub(super) fn pose_take_home(g: &mut crate::game::Game, p: &mut crate::anim_skeleton::Pose) {
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
    // (the row is let go: emptied, and the list's empty tail dropped)
    g.loose_poses[row] = LoosePose { nodes: 0, floats: Vec::new() };
    while g.loose_poses.last().map_or(false, |r| r.nodes == 0) { g.loose_poses.pop(); }
    *p = home;
}

/// The floats, wherever they are: the row itself, to write through and let go before the next call that takes g.
pub(super) fn pose_cols<'g>(g: &'g mut crate::game::Game, this: crate::anim_skeleton::Pose) -> &'g mut [f32] {
    cols_of(g, this)
}

/// Between poses of one rig; another count of nodes is refused.
pub(super) fn pose_copy(g: &mut crate::game::Game, this: crate::anim_skeleton::Pose, src: crate::anim_skeleton::Pose) {
    if rows_of(src) != rows_of(this) {
        panic!("Pose.copy: a pose of {} nodes into one of {}", src.nodes, this.nodes);
    }
    if src == this {
        return;
    }
    let from = cols_of_read(g, src);
    cols_of(g, this).copy_from_slice(from);
}

/// Node i's rotation.
pub(super) fn pose_rot(g: &crate::game::Game, this: crate::anim_skeleton::Pose, i: f64) -> Quat {
    let n_ = rows_of(this);
    let o = ((3 * n_) as isize + i as isize) as usize;
    let d = cols_of_read(g, this);
    [d[o] as f64, d[o + n_] as f64, d[o + 2 * n_] as f64, d[o + 3 * n_] as f64]
}

/// Node i's rotation written; no such node writes nothing.
pub(super) fn pose_set_rot(g: &mut crate::game::Game, this: crate::anim_skeleton::Pose, i: f64, q: Quat) {
    if !(i >= 0.0 && i < this.nodes as f64) {
        return;
    }
    let n_ = rows_of(this);
    let o = 3 * n_ + i as usize;
    let d = cols_of(g, this);
    d[o] = q[0] as f32;
    d[o + n_] = q[1] as f32;
    d[o + 2 * n_] = q[2] as f32;
    d[o + 3 * n_] = q[3] as f32;
}

/// Node i's origin.
pub(super) fn pose_pos(g: &crate::game::Game, this: crate::anim_skeleton::Pose, i: f64) -> V3 {
    let n_ = rows_of(this);
    let i = i as isize;
    let d = cols_of_read(g, this);
    [d[i as usize] as f64, d[(n_ as isize + i) as usize] as f64, d[((2 * n_) as isize + i) as usize] as f64]
}

// ---------------------------------------------------------------- the poses one character keeps

/// A set that will make at most `most` poses.
pub(super) fn pose_set_new(g: &mut crate::game::Game, most: f64) -> PoseSetId {
    g.tab.pose_set.push(PoseSet { most, ..PoseSet::blank() })
}

/// One more pose of the set; past `most` is a mistake said by name.
pub(super) fn pose_set_make(g: &mut crate::game::Game, id: PoseSetId, n: f64) -> crate::anim_skeleton::Pose {
    let most = g.tab.pose_set[id].most;
    if g.tab.pose_set[id].list.len() as f64 >= most {
        panic!("posing: a set of {} poses at most is asked for one more (the owner says how many it keeps as it makes the set, and a level is held to the pose pool by that number: game/levelfit.ts fitRows)", most);
    }
    // (a pose made while the set is at home is made in the pool; one made before is an array of its own until the set goes home, as the TypeScript's)
    let p = if g.tab.pose_set[id].home.is_some() { pose_pool_new(g, n as usize) } else { pose_new(g, n as usize) };
    g.tab.pose_set[id].list.push(p);
    if g.tab.pose_set[id].home.is_some() && !pose_in(g, p) {
        g.tab.pose_set[id].stray += 1.0;
    }
    p
}

/// Singles a game's pool holds.
pub fn pose_pool_room() -> f64 {
    ::sim::poses::POOL as f64
}

/// Singles of the pool this set fills when it has made every pose it can (none made yet: none, it has no rig to count by).
pub fn pose_set_most(g: &crate::game::Game, s: PoseSetId) -> f64 {
    let set = &g.tab.pose_set[s];
    match set.list.first() {
        Some(p) => set.most * rows_of(*p) as f64 * 10.0,
        None => 0.0,
    }
}

/// Is this pose's floats in the block. A handle gets its place as it is made and a copy of it could not take a later one, so a pose with none stays so: false.
fn pose_in(g: &crate::game::Game, p: crate::anim_skeleton::Pose) -> bool {
    p.at < Pose::LOOSE
}

/// Every pose of a set at home in the game's block; false = the pool had no room for one of them, see `stray`.
pub(super) fn pose_set_in(g: &mut crate::game::Game, s: PoseSetId) -> bool {
    g.tab.pose_set[s].home = Some(crate::Opaque);
    let mut stray = 0.0;
    for k in 0..g.tab.pose_set[s].list.len() {
        let mut p = g.tab.pose_set[s].list[k];
        if p.nodes != 0 && !pose_in(g, p) {
            // (its floats into the pool. The list is the set's; whoever else holds a handle on the pose, a man's animator, is given the new
            //  place by the one who moves him: posehome's come_home, which takes his poses home before it calls this)
            pose_take_home(g, &mut p);
            g.tab.pose_set[s].list[k] = p;
            if !pose_in(g, p) { stray += 1.0; }
        }
    }
    g.tab.pose_set[s].stray = stray;
    stray == 0.0
}

/// …and out again.
pub(super) fn pose_set_out(g: &mut crate::game::Game, s: PoseSetId) {
    g.tab.pose_set[s].home = None;
    g.tab.pose_set[s].stray = 0.0;
}

/// Is the pose in a block (not −1, the bench). No g here, so only the sign says anything: the slot it is in is g.sim, which is what lend answers.
fn slot_of(p: crate::anim_skeleton::Pose) -> f64 {
    if p.at >= Pose::LOOSE {
        -1.0
    } else {
        0.0
    }
}

/// A pose made ready for a question: at home as it stands (its slot), or copied to place `place` of the bench (−1).
fn lend(g: &crate::game::Game, p: crate::anim_skeleton::Pose, place: f64) -> f64 {
    if slot_of(p) < 0.0 {
        bench_put(g, p, place);
        return -1.0;
    }
    g.sim as f64
}

/// lend, as rust/sim's questions want the answer: (the game or ON_BENCH, the pose's place in it).
fn lent_at(g: &crate::game::Game, p: crate::anim_skeleton::Pose, place: f64) -> (usize, usize) {
    let slot = lend(g, p, place);
    if slot < 0.0 {
        (::sim::skeleton::ON_BENCH, place as usize)
    } else {
        (slot as usize, p.at as usize)
    }
}

/// The module did not know the pose it was asked about: never the game's doing, so it is loud.
fn known(ok: f64) {
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

/// `out` set to the rig's rest pose.
pub(super) fn rest_pose(g: &mut crate::game::Game, ch: RigId, out: crate::anim_skeleton::Pose) {
    let d = cols_of(g, out);
    let rig = &g.art.rig[ch];
    if (out.nodes as usize) < rig.nodes.len() {
        panic!("restPose: a pose of {} nodes for a rig of {}", out.nodes, rig.nodes.len());
    }
    let n_ = rows_of(out);
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

/// What this file keeps beside a clip (its lines of times, its number in the module, the nodes it needs), worked out the first time.
fn side_of<'g>(g: &'g mut crate::game::Game, clip: ClipId) -> &'g mut crate::anim_skeleton::ClipSide {
    if !g.m.anim_skeleton.clip_side.has(clip) {
        let tr = &g.art.clip[clip].tracks;
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
        g.m.anim_skeleton.clip_side.set(clip, ClipSide { lines: l, word: 0, need });
    }
    // (the word goes by no file and no wire, being worked out from the clip again whenever it is wanted: a game read from a file comes back without it)
    if g.m.anim_skeleton.clip_side.get(clip).unwrap().word == 0 {
        let word = clip_word(&g.art.clip[clip]);
        if let Some(at) = ::sim::clips::find(word) {
            if !clip_is_as_held(&g.art.clip[clip], at) { panic!("posing: the clip \"{}\" has the word of another clip the store holds (rust/sim/src/clips.rs): the word wants more bits", g.art.clip[clip].name); }
        }
        g.m.anim_skeleton.clip_side.get_mut(clip).unwrap().word = word;
    }
    g.m.anim_skeleton.clip_side.get_mut(clip).unwrap()
}

/// clipIn's work, answering the two numbers its callers read: where the clip stands in the store, and the nodes it needs. Where it stands is asked of the store
/// each time: it is the process's, first come first served, so a game keeps none of it. −1: the store does not take it.
fn clip_place(g: &mut crate::game::Game, clip: ClipId) -> (f64, f64) {
    let c = side_of(g, clip);
    let (word, need) = (c.word, c.need);
    if let Some(at) = ::sim::clips::find(word) {
        return (at as f64, need);
    }
    if ::sim::clips::refused(word) { return (-1.0, need); }
    let at = clip_write(&g.art.clip[clip]).unwrap_or(-1.0);
    if at < 0.0 { ::sim::clips::refuse(word); }
    (at, need)
}

/// The word the store knows a clip by, off the game's own copy: the same fold over the same numbers, so nothing has to be written to find it.
fn clip_word(clip: &crate::anim_gltf::Clip) -> u32 {
    let mut f = ::sim::clips::Fold::new(clip.duration);
    for t in &clip.tracks {
        let (out, comps) = ::sim::clips::shape_of(t.node as usize, t.path as u32);
        f.shape(t.times.len() as u32, out, comps, t.step);
    }
    for t in &clip.tracks {
        for v in t.times.iter().chain(t.values.iter()) {
            f.single(*v as f64);
        }
    }
    f.word()
}

/// A word is only a hash: the clip held under it is compared whole, once for each game that comes to it, as `clips::end` compares a clip written again.
fn clip_is_as_held(clip: &crate::anim_gltf::Clip, at: usize) -> bool {
    let tb = ::sim::clips::table_read();
    let held = &tb.clips[at];
    if held.duration.to_bits() != clip.duration.to_bits() || held.tracks as usize != clip.tracks.len() {
        return false;
    }
    clip.tracks.iter().enumerate().all(|(k, t)| {
        let x = &tb.tracks[held.track0 as usize + k];
        let (out, comps) = ::sim::clips::shape_of(t.node as usize, t.path as u32);
        x.keys as usize == t.times.len() && x.out == out && x.comps == comps && x.step == t.step
            && t.times.iter().chain(t.values.iter()).enumerate().all(|(i, v)| ::sim::single::get(&tb.pool, x.at as usize + i).to_bits() == (*v as f64).to_bits())
    })
}

/// The tracks of a clip into rust/sim/src/clips.rs. None: not asked of the module at all, its number stays −1.
fn clip_write(clip: &crate::anim_gltf::Clip) -> Option<f64> {
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

/// The number a layer samples `clip` by; a clip the module does not hold, or one past the pose's nodes, is a mistake said by name.
pub(super) fn clip_no(g: &mut crate::game::Game, clip: ClipId, nodes: f64) -> f64 {
    let (at, need) = clip_place(g, clip);
    if at >= 0.0 && need <= nodes {
        return at;
    }
    let name = &g.art.clip[clip].name;
    if at < 0.0 {
        panic!("posing: a layer samples the clip \"{}\" and the Rust module does not hold it (rust/sim/src/clips.rs: no room in the store, or a track it cannot take)", name);
    }
    panic!("posing: a layer samples the clip \"{}\", which writes node {}, over a pose of {} nodes", name, need - 1.0, nodes);
}

/// A character file's clips against what the module holds, as an animator is made.
pub(super) fn layer_clips_fit(g: &mut crate::game::Game, clips: &[ClipId], nodes: f64) {
    if nodes > BENCH_NODES {
        panic!("posing: a rig of {} nodes is more than the Rust module's bench holds ({}: anim/skeleton.ts BENCH_NODES with rust/sim/src/skeleton.rs), and a layer is posed nowhere else", nodes, BENCH_NODES);
    }
    for c in clips {
        clip_no(g, *c, nodes);
    }
}

// ---------------------------------------------------------------- sampling and blending

/// Sample clip at time t (seconds, wrapped if loop) into pose (only nodes with tracks are written): a question of the module, or this file's own arithmetic for what it refuses.
pub(super) fn sample_clip(g: &mut crate::game::Game, clip: ClipId, t: f64, loop_: bool, out: crate::anim_skeleton::Pose) {
    let n = out.nodes as f64;
    if bench_holds(n) {
        let (at, need) = clip_place(g, clip);
        if at >= 0.0 && need <= n {
            let (slot, place) = lent_at(g, out, 0.0);
            known(answer(::sim::skeleton::ask_sample(slot, place, out.nodes as usize, at as usize, t, loop_)));
            if slot == ::sim::skeleton::ON_BENCH {
                bench_get(g, out, 0.0);
            }
            return;
        }
    }
    let d = cols_of(g, out);
    sample_own(&g.art.clip[clip], t, loop_, d, rows_of(out), n);
}

/// sampleClip's own arithmetic over a pose's singles (`n_` rows a column). Every track searches by itself: the same place as ClipSide.lines finds, so the same pose.
fn sample_own(clip: &crate::anim_gltf::Clip, t: f64, loop_: bool, d: &mut [f32], n_: usize, num_nodes: f64) {
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
    }
}

/// out = lerp(a, b, w * mask[node]); no mask means uniform weight. `a` None means out is a.
pub(super) fn blend_pose(g: &mut crate::game::Game, a: Option<crate::anim_skeleton::Pose>, b: crate::anim_skeleton::Pose, w: f64, mask: Option<&[f32]>, out: crate::anim_skeleton::Pose) {
    let a = a.unwrap_or(out);
    let n = a.nodes as usize;
    let alike = b.nodes == a.nodes && out.nodes == a.nodes && mask.map_or(true, |k| k.len() == n);
    if alike && bench_holds(n as f64) {
        if let Some(k) = mask {
            let bn = ::sim::skeleton::bench();
            for i in 0..k.len() {
                ::sim::single::put(&mut bn.mask, i, k[i] as f64);
            }
        }
        if a.at < Pose::LOOSE && b.at < Pose::LOOSE && out.at < Pose::LOOSE {
            known(answer(::sim::skeleton::ask_blend(g.sim, a.at as usize, b.at as usize, w, mask.is_some(), out.at as usize, n)));
            return;
        }
        let sb = if b == a { 0 } else { 1 };
        let so = if out == a { 0 } else if out == b { sb } else { 2 };
        bench_put(g, a, 0.0);
        if sb != 0 {
            bench_put(g, b, 1.0);
        }
        known(answer(::sim::skeleton::ask_blend(::sim::skeleton::ON_BENCH, 0, sb, w, mask.is_some(), so, n)));
        bench_get(g, out, so as f64);
        return;
    }
    blend_own(pool_at(g.sim), span_of(a), span_of(b), span_of(out), n, rows_of(a), w, mask);
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
fn blend_own(d: &mut [f32], a: (usize, usize), b: (usize, usize), out: (usize, usize), n: usize, n_: usize, w: f64, mask: Option<&[f32]>) {
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
}

// ---------------------------------------------------------------- a body's world of joints

/// Every node's global matrix and rotation at rest values, and room for the skinning matrices.
pub(super) fn skeleton_instance_new(g: &crate::game::Game, ch: RigId) -> crate::anim_skeleton::SkeletonInstance {
    let rig = &g.art.rig[ch];
    SkeletonInstance {
        rig: ch,
        globals: rig.nodes.iter().map(|_| v3::m4::create()).collect(),
        global_rot: rig.nodes.iter().map(|_| v3::quat::ident()).collect(),
        joint_data: vec![0.0f32; rig.skin.joints.len() * 16],
    }
}

/// A body posed: run forward, and skinned where it stands.
pub(super) fn skeleton_instance_update(g: &crate::game::Game, this: &mut crate::anim_skeleton::SkeletonInstance, pose: crate::anim_skeleton::Pose, world: &crate::v3::m4::Mat4, twists: Option<&[crate::anim_skeleton::TwistSpec]>, overrides: Option<&[Option<crate::anim_skeleton::NodeOverride>]>) {
    skeleton_instance_forward(g, this, pose, twists, overrides);
    skeleton_instance_skin(g, this, world);
}

/// Model-space matrices of all nodes from a local pose, with model-space twists and per-node overrides.
pub(super) fn skeleton_instance_forward(g: &crate::game::Game, this: &mut crate::anim_skeleton::SkeletonInstance, pose: crate::anim_skeleton::Pose, twists: Option<&[crate::anim_skeleton::TwistSpec]>, overrides: Option<&[Option<crate::anim_skeleton::NodeOverride>]>) {
    let d = cols_of_read(g, pose);
    let rig = &g.art.rig[this.rig];
    let n_ = rows_of(pose);
    // several layers may push a twist for the same node: they accumulate
    let twists = twists.unwrap_or(&[]);
    let turned = twists.iter().fold(0u128, |w, t| w | 1 << (t.node as u32 & 127));
    let twist_of = |i: usize| -> Option<TwistSpec> {
        if turned >> (i & 127) & 1 == 0 { return None; }
        twists.iter().filter(|t| t.node as usize == i).copied().reduce(|e, t| TwistSpec { node: t.node, yaw: e.yaw + t.yaw, pitch: e.pitch + t.pitch })
    };
    for &oi in &rig.order {
        let i = oi as usize;
        let parent = rig.nodes[i].parent;
        let (rx, ry, rz, rw) = (d[3 * n_ + i] as f64, d[4 * n_ + i] as f64, d[5 * n_ + i] as f64, d[6 * n_ + i] as f64);
        let (tx, ty, tz, sx, sy, sz) = (d[i] as f64, d[n_ + i] as f64, d[2 * n_ + i] as f64, d[7 * n_ + i] as f64, d[8 * n_ + i] as f64, d[9 * n_ + i] as f64);
        if let Some(Some(ov)) = overrides.and_then(|o| o.get(i)) {
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
        let tw = twist_of(i);
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
}

/// The skinning matrices of a body run forward (world · global · inverse bind, straight into the joint's slice of joint_data): what is drawn of it, and no part of the run.
pub(super) fn skeleton_instance_skin(g: &crate::game::Game, this: &mut crate::anim_skeleton::SkeletonInstance, world: &crate::v3::m4::Mat4) {
    let rig = &g.art.rig[this.rig];
    let world = v3::m4::wide(world);
    for ((&node, ibm), out) in rig.skin.joints.iter().zip(rig.skin.inverse_bind.chunks_exact(16)).zip(this.joint_data.chunks_exact_mut(16)) {
        let bound = v3::m4::mul(&this.globals[node as usize], ibm.first_chunk().unwrap());
        out.copy_from_slice(&v3::m4::mul_wide(&world, &bound));
    }
}

/// Model-space origin of a node.
pub(super) fn skeleton_instance_node_pos(this: &crate::anim_skeleton::SkeletonInstance, node: f64) -> V3 {
    let g = &this.globals[node as usize];
    [g[12] as f64, g[13] as f64, g[14] as f64]
}

/// Rewrite the rotations of `pose` so that plain FK reproduces the globals of the last update: parent⁻¹ · global per node.
pub(super) fn skeleton_instance_bake_local_rotations(g: &mut crate::game::Game, this: &mut crate::anim_skeleton::SkeletonInstance, pose: crate::anim_skeleton::Pose) {
    let d = cols_of(g, pose);
    let rig = &g.art.rig[this.rig];
    let n_ = rows_of(pose);
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

/// A per-node mask: a weight for listed node names and (optionally) all their descendants.
pub(super) fn build_mask(g: &crate::game::Game, ch: RigId, entries: &[crate::anim_skeleton::BuildMaskEntriesItem]) -> Vec<f32> {
    let rig = &g.art.rig[ch];
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

/// Every node's rotation the identity and its origin nought.
pub(super) fn pose_fk_new(g: &crate::game::Game, ch: RigId) -> crate::anim_skeleton::PoseFK {
    let rig = &g.art.rig[ch];
    PoseFK {
        rot: rig.nodes.iter().map(|_| v3::quat::ident()).collect(),
        pos: rig.nodes.iter().map(|_| [0.0, 0.0, 0.0]).collect(),
        rig: ch,
        ..PoseFK::blank()
    }
}

/// Model-space rotation and origin of every node from a local pose, written in place: a question of the module, or this file's own arithmetic for a rig the bench cannot hold.
pub(super) fn pose_fk_run(g: &mut crate::game::Game, this: &mut crate::anim_skeleton::PoseFK, pose: crate::anim_skeleton::Pose) {
    if bench_rig(g, this.rig) {
        bench_put_fk(this);
        let (slot, place) = lent_at(g, pose, 0.0);
        known(answer(::sim::skeleton::ask_run(slot, place, -1)));
        bench_get_fk(this);
        return;
    }
    let d = cols_of_read(g, pose);
    fk_own(this, d, rows_of(pose), &g.art.rig[this.rig]);
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

    fn folded(a: &[f32]) -> Vec<u32> {
        a.iter().map(|v| v.to_bits()).collect()
    }
    fn val(i: usize, s: usize) -> f64 {
        ((i * s + 11) % 101) as f64 / 101.0 - 0.5
    }
    /// the singles of the test's pose of five nodes (eight rows a column)
    fn cols(s: usize) -> Vec<f32> {
        (0..80).map(|i| if i % 8 < 5 { val(i, s) as f32 } else { 0.0 }).collect()
    }
    fn pose_of(g: &mut crate::game::Game, s: usize) -> Pose {
        let p = pose_pool_new(g, 5);
        pose_cols(g, p).copy_from_slice(&cols(s));
        p
    }
    const ROWS: usize = 8;
    /// a pose of five nodes standing at (x, i, 0) with its scale s and the turn `yaw` about the vertical
    fn stood(x: f32, yaw: f64, s: f32) -> Vec<f32> {
        let q = v3::quat::yaw(yaw);
        let mut d = vec![0.0f32; 10 * ROWS];
        for i in 0..5 {
            let col = [x, i as f32, 0.0, q[0] as f32, q[1] as f32, q[2] as f32, q[3] as f32, s, s, s];
            for c in 0..10 { d[c * ROWS + i] = col[c]; }
        }
        d
    }
    fn yaw_at(d: &[f32], i: usize) -> f64 {
        2.0 * m::atan2(d[4 * ROWS + i] as f64, d[6 * ROWS + i] as f64)
    }
    fn near(a: f64, b: f64, what: &str) { assert!(m::abs(a - b) < 1e-5, "{what}: {a} against {b}"); }
    /// the three poses of a blend laid end to end: a, b, what comes out
    fn store(a: &[f32], b: &[f32], o: &[f32]) -> Vec<f32> {
        let mut d = a.to_vec();
        d.extend_from_slice(b);
        d.extend_from_slice(o);
        d
    }
    const A: (usize, usize) = (0, 80);
    const B: (usize, usize) = (80, 80);
    const OUT: (usize, usize) = (160, 80);

    /// A move going into the next: the whole of one at nought, the whole of the other at one, and on the way a real pose.
    #[test]
    fn a_blend_goes_from_one_pose_to_the_other_by_its_weight() {
        let (a, b) = (stood(1.0, 0.2, 1.0), stood(3.0, 1.0, 2.0));
        for (w, want) in [(0.0, &a), (-1.0, &a), (1.0, &b), (2.0, &b)] {
            let mut d = store(&a, &b, &[0.0; 80]);
            blend_own(&mut d, A, B, OUT, 5, ROWS, w, None);
            assert_eq!(folded(&d[160..]), folded(want), "weight {w}");
        }
        let mut d = store(&a, &b, &[0.0; 80]);
        blend_own(&mut d, A, B, OUT, 5, ROWS, 0.25, None);
        let o = &d[160..];
        near(o[2] as f64, 1.5, "a quarter of the way between the places");
        near(o[7 * ROWS + 2] as f64, 1.25, "and the scales");
        assert!(m::abs(yaw_at(o, 2) - 0.4) < 0.01, "and about that of the turn: {}", yaw_at(o, 2));
        let l: f64 = (3..7).map(|c| { let x = o[c * ROWS + 2] as f64; x * x }).sum();
        near(l, 1.0, "a turn stays a turn");
    }

    /// A rotation and its negative are one rotation: a blend between them stands still, it does not go round the long way or through nothing.
    #[test]
    fn a_blend_takes_the_short_way_round() {
        let a = stood(0.0, 0.3, 1.0);
        let mut b = a.clone();
        for i in 0..5 { for c in 3..7 { b[c * ROWS + i] = -b[c * ROWS + i]; } }
        let mut d = store(&a, &b, &[0.0; 80]);
        blend_own(&mut d, A, B, OUT, 5, ROWS, 0.5, None);
        near(yaw_at(&d[160..], 1), 0.3, "half way between a turn and itself");
    }

    /// A layer on part of the body: each node takes the weight times his share, and a node with no share stays the first pose.
    #[test]
    fn a_mask_gives_each_node_his_share_of_a_blend() {
        let (a, b) = (stood(0.0, 0.0, 1.0), stood(2.0, 0.0, 1.0));
        let mask = [0.0f32, 0.25, 0.5, 1.0, 1.5];
        let mut d = store(&a, &b, &[0.0; 80]);
        blend_own(&mut d, A, B, OUT, 5, ROWS, 0.8, Some(&mask));
        let got: Vec<f64> = (0..5).map(|i| d[160 + i] as f64).collect();
        for (i, want) in [0.0, 0.4, 0.8, 1.6, 2.0].iter().enumerate() { near(got[i], *want, "a node's share"); }
        // a blend written over its own first pose comes out the same, and with no share leaves the node as it stood
        let mut own = store(&a, &b, &[]);
        blend_own(&mut own, A, B, A, 5, ROWS, 0.8, Some(&mask));
        assert_eq!(folded(&own[..80]), folded(&d[160..]));
    }

    /// The module and the rules each hold a blend: they answer alike, and it writes only the pose it was given.
    #[test]
    fn the_module_finds_a_pose_by_its_place_and_blends_as_the_rules_do() {
        let (_h, mut g) = hold();
        let (a, b, o) = (pose_of(&mut g, 37), pose_of(&mut g, 53), pose_pool_new(&mut g, 5));
        assert!(::sim::skeleton::ask_blend(g.sim, a.at as usize, b.at as usize, 0.37, false, o.at as usize, 5));
        let mut d = store(&cols(37), &cols(53), &[0.0; 80]);
        blend_own(&mut d, A, B, OUT, 5, ROWS, 0.37, None);
        assert_eq!(folded(pose_cols(&mut g, o)), folded(&d[160..]));
        // the poses either side are as they were
        assert_eq!(folded(pose_cols(&mut g, a)), folded(&cols(37)));
        assert_eq!(folded(pose_cols(&mut g, b)), folded(&cols(53)));
    }

    fn track(node: f64, path: f64, times: &[f32], values: &[f32], step: bool) -> crate::anim_gltf::Track {
        crate::anim_gltf::Track { node, path, times: times.to_vec(), values: values.to_vec(), step }
    }
    /// A clip of 1.25 s: a move and a half turn on two nodes, a stepped scale, a node with one key, and a track of a node the rig has not (the last, written apart).
    fn clip_of(past_the_rig: bool) -> crate::anim_gltf::Clip {
        let s = core::f32::consts::FRAC_1_SQRT_2;
        let mut tracks = vec![
            track(0.0, 0.0, &[0.0, 0.5, 1.0], &[0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 3.0, 2.0, 1.0], false),
            track(1.0, 1.0, &[0.0, 1.0], &[0.0, 0.0, 0.0, 1.0, 0.0, s, 0.0, s], false),
            track(2.0, 2.0, &[0.25, 0.75], &[1.0, 1.0, 1.0, 2.0, 2.0, 2.0], true),
            track(3.0, 1.0, &[0.2], &[0.0, 0.0, 0.0, 1.0], false),
        ];
        if past_the_rig { tracks.push(track(9.0, 0.0, &[0.0, 1.0], &[9.0, 9.0, 9.0, 8.0, 8.0, 8.0], false)); }
        crate::anim_gltf::Clip { name: String::from("c"), duration: 1.25, tracks }
    }

    #[test]
    fn a_sample_goes_between_a_clips_keys_and_holds_at_its_ends() {
        let clip = clip_of(true);
        let read = |t: f64, looped: bool| { let mut o = cols(37); sample_own(&clip, t, looped, &mut o, ROWS, 5.0); o };
        let o = read(0.25, false);
        near(o[0] as f64, 0.5, "half way between two keys: half way between their places");
        near(o[ROWS] as f64, 1.0, "in each column");
        near(read(0.5, false)[0] as f64, 1.0, "on a key: the key");
        near(yaw_at(&read(0.5, false), 1), m::PI / 4.0, "half of a quarter turn");
        near(read(1.2, false)[0] as f64, 3.0, "past the last key: the last");
        near(read(-1.0, false)[0] as f64, 0.0, "before the first key, and before the clip: the first");
        near(read(9.0, false)[0] as f64, 3.0, "past the clip's end: its end");
        // a stepped track holds its key until the next
        near(read(0.74, false)[7 * ROWS + 2] as f64, 1.0, "a step before its next key");
        near(read(0.76, false)[7 * ROWS + 2] as f64, 2.0, "a step past it");
        // a loop comes round
        near(read(0.25 + 1.25, true)[0] as f64, 0.5, "once round");
        near(read(0.25 - 1.25, true)[0] as f64, 0.5, "back round");
        // what no track names stands as it stood, and a node past the rig writes nothing
        let was = cols(37);
        let o = read(0.25, false);
        assert_eq!(o[4].to_bits(), was[4].to_bits(), "node 4 has no track");
        assert_eq!(o[ROWS + 4].to_bits(), was[ROWS + 4].to_bits());
    }

    /// The module and the rules each hold a sample: they answer alike. A clip is written once, however often it is asked for.
    #[test]
    fn a_clip_written_into_the_module_samples_as_the_rules_do() {
        let (_h, mut g) = hold();
        let clip = clip_of(false);
        let at = clip_write(&clip).unwrap() as usize;
        assert_eq!(clip_write(&clip).unwrap() as usize, at);
        let p = pose_pool_new(&mut g, 5);
        for (t, looped) in [(0.7, true), (-0.2, true), (5.1, false), (-1.0, false), (0.5, true), (1.1, false)] {
            let mut want = cols(37);
            sample_own(&clip, t, looped, &mut want, ROWS, 5.0);
            pose_cols(&mut g, p).copy_from_slice(&cols(37));
            assert!(::sim::skeleton::ask_sample(g.sim, p.at as usize, 5, at, t, looped));
            assert_eq!(folded(pose_cols(&mut g, p)), folded(&want), "t {t}");
        }
    }

    fn rig() -> crate::anim_gltf::GltfCharacter {
        let mut ch = crate::anim_gltf::GltfCharacter::default();
        for parent in [-1.0, 0.0, 1.0, 1.0, 3.0] {
            ch.nodes.push(crate::anim_gltf::GltfNode { parent, t: [0.0; 3], r: [0.0, 0.0, 0.0, 1.0], s: [1.0; 3], ..crate::anim_gltf::GltfNode::default() });
        }
        ch.order = vec![0.0, 1.0, 2.0, 3.0, 4.0];
        ch.skin.joints = vec![0.0, 1.0, 2.0, 3.0, 4.0];
        ch.skin.inverse_bind = (0..80).map(|i| if (i % 16) % 5 == 0 { 1.0 } else { 0.0 }).collect();
        ch
    }
    /// the rig's five nodes each a metre along +z from their parent, the given yaw on each, the root at (1, 2, 3)
    fn chain(yaws: [f64; 5]) -> Vec<f32> {
        let mut d = stood(0.0, 0.0, 1.0);
        for i in 0..5 {
            let q = v3::quat::yaw(yaws[i]);
            let t = if i == 0 { [1.0, 2.0, 3.0] } else { [0.0, 0.0, 1.0] };
            for c in 0..3 { d[c * ROWS + i] = t[c]; }
            for c in 0..4 { d[(3 + c) * ROWS + i] = q[c] as f32; }
        }
        d
    }
    fn at_near(a: V3, b: V3, what: &str) { assert!(v3::dist(a, b) < 1e-5, "{what}: {a:?} against {b:?}"); }

    /// Each joint hangs off its parent, by the parent's whole turn so far. The rig is 0 - 1 - (2, 3 - 4).
    #[test]
    fn a_pose_run_forward_hangs_each_joint_off_its_parent() {
        let ch = rig();
        let mut fk = PoseFK { rot: vec![v3::quat::ident(); 5], pos: vec![[0.0; 3]; 5], ..PoseFK::blank() };
        let quarter = m::PI / 2.0;
        fk_own(&mut fk, &chain([quarter, 0.0, 0.0, -quarter, 0.0]), ROWS, &ch);
        at_near(fk.pos[0], [1.0, 2.0, 3.0], "the root");
        at_near(fk.pos[1], [2.0, 2.0, 3.0], "a metre along the way the root faces");
        at_near(fk.pos[2], [3.0, 2.0, 3.0], "and another");
        at_near(fk.pos[3], [3.0, 2.0, 3.0], "a child of the same parent, the same place");
        at_near(fk.pos[4], [3.0, 2.0, 4.0], "and a joint that turned back takes his child with him");
    }

    /// What a body draws with: plain, a turn laid on a subtree, a joint driven from outside, and a pose made again from what was drawn.
    #[test]
    fn a_body_is_posed_from_its_joints_and_a_bake_keeps_what_it_drew() {
        let (_h, mut g) = hold();
        let id = alloc::sync::Arc::make_mut(&mut g.art).rig.push(rig());
        let mut si = SkeletonInstance { rig: id, globals: vec![v3::m4::create(); 5], global_rot: vec![v3::quat::ident(); 5], joint_data: vec![0.0; 80] };
        let w = v3::m4::from_trss(10.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0);
        let quarter = m::PI / 2.0;
        let p = pose_pool_new(&mut g, 5);
        pose_cols(&mut g, p).copy_from_slice(&chain([0.0; 5]));

        // plain: where a run forward puts them, the skinning matrices in the world
        skeleton_instance_update(&mut g, &mut si, p, &w, None, None);
        at_near(skeleton_instance_node_pos(&si, 2.0), [1.0, 2.0, 5.0], "two joints along");
        near(si.joint_data[2 * 16 + 12] as f64, 11.0, "the skin of that joint stands in the world");

        // a twist turns that joint and all under it, about the vertical, and two on one joint add
        let twists = [TwistSpec { node: 1.0, yaw: quarter * 0.5, pitch: 0.0 }, TwistSpec { node: 1.0, yaw: quarter * 0.5, pitch: 0.0 }];
        skeleton_instance_update(&mut g, &mut si, p, &w, Some(&twists), None);
        at_near(skeleton_instance_node_pos(&si, 1.0), [1.0, 2.0, 4.0], "the twisted joint stays where it is");
        at_near(skeleton_instance_node_pos(&si, 2.0), [2.0, 2.0, 4.0], "and what hangs off it swings round");
        at_near(skeleton_instance_node_pos(&si, 4.0), [3.0, 2.0, 4.0], "all the way down");
        at_near(skeleton_instance_node_pos(&si, 0.0), [1.0, 2.0, 3.0], "what is above it does not");

        // driven from outside: the joint's rotation and place as given, or where its parent puts it; what hangs off it follows
        let q = v3::quat::yaw(quarter);
        let mut ov = [None; 5];
        ov[3] = Some(NodeOverride { rot: [q[0], q[1], q[2], q[3]], pos: Some([7.0, 7.0, 7.0]) });
        skeleton_instance_update(&mut g, &mut si, p, &w, None, Some(&ov));
        at_near(skeleton_instance_node_pos(&si, 3.0), [7.0, 7.0, 7.0], "a place from outside");
        at_near(skeleton_instance_node_pos(&si, 4.0), [8.0, 7.0, 7.0], "his child follows");
        ov[3] = Some(NodeOverride { rot: [q[0], q[1], q[2], q[3]], pos: None });
        skeleton_instance_update(&mut g, &mut si, p, &w, None, Some(&ov));
        at_near(skeleton_instance_node_pos(&si, 3.0), [1.0, 2.0, 5.0], "no place from outside: where his parent has him");

        // a bake writes the pose so that a plain pose run forward draws what the twists and the drive drew
        skeleton_instance_update(&mut g, &mut si, p, &w, Some(&twists), Some(&ov));
        let drawn: Vec<V3> = (0..5).map(|i| skeleton_instance_node_pos(&si, i as f64)).collect();
        skeleton_instance_bake_local_rotations(&mut g, &mut si, p);
        skeleton_instance_update(&mut g, &mut si, p, &w, None, None);
        for i in 0..5 { at_near(skeleton_instance_node_pos(&si, i as f64), drawn[i], "the bake"); }
    }

    #[test]
    fn a_pose_goes_onto_the_bench_and_back_by_its_own_nodes() {
        let (_h, mut g) = hold();
        let p = pose_of(&mut g, 37);
        bench_put(&mut g, p, 1.0);
        let back = pose_pool_new(&mut g, 5);
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
        let (p, q) = (pose_pool_new(&mut g, 5), pose_pool_new(&mut g, 5));
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

    /// a man of the shift with a set of two poses, each filled with its own numbers, his animator's two stages on them
    fn posed_man(g: &mut crate::game::Game, s: usize) -> (GuardId, CharId) {
        let set = pose_set_new(g, 2.0);
        let (a, b) = (pose_set_make(g, set, 5.0), pose_set_make(g, set, 5.0));
        pose_cols(g, a).copy_from_slice(&cols(s));
        pose_cols(g, b).copy_from_slice(&cols(s + 1));
        let mut ch = crate::character::Character::blank();
        ch.anim.poses = set;
        ch.anim.p_a = a;
        ch.anim.p_b = b;
        let c = crate::character::man_new(g, ch);
        let mut gd = crate::game::Guard::blank();
        gd.ch = c;
        let id = g.gd.push(gd);
        g.guards.push(id);
        (id, c)
    }

    #[test]
    fn the_pool_holds_the_men_on_the_lists_and_one_who_left_keeps_his_floats() {
        let (_h, mut g) = hold();
        let (a, ca) = posed_man(&mut g, 10);
        let (b, cb) = posed_man(&mut g, 20);
        let (c, cc) = posed_man(&mut g, 30);
        let used = |g: &crate::game::Game| ::sim::poses::used(&::sim::state::state(g.sim).poses);
        // the first tick: everybody goes home (their poses are in the pool as made), and nothing moves
        crate::posehome::pose_home_tend(&mut g);
        assert_eq!(g.pose_sets.len(), 3);
        assert_eq!(used(&g), 6 * 80);
        let was = g.ch[cc].anim.p_a;
        // a ragdoll's pose is his own, outside the pool
        let rag = pose_new(&mut g, 5);
        pose_cols(&mut g, rag).copy_from_slice(&cols(77));
        assert!(rag.at >= Pose::LOOSE);
        assert_eq!(used(&g), 6 * 80);
        // the second man leaves the list: everybody out, the pool emptied, the two who are there back in, in the list's order
        g.guards.retain(|x| *x != b);
        crate::posehome::pose_home_tend(&mut g);
        assert_eq!(g.pose_sets.len(), 2);
        assert_eq!(used(&g), 4 * 80);
        let (pa, pc, pb) = (g.ch[ca].anim.p_a, g.ch[cc].anim.p_a, g.ch[cb].anim.p_a);
        assert!(pc.at < was.at, "the third man's poses moved down into the room the second left");
        assert_eq!(g.tab.pose_set[g.ch[cc].anim.poses].list[0], pc, "a set's list and the animator's handle name one pose");
        assert_eq!(folded(pose_cols(&mut g, pa)), folded(&cols(10)));
        assert_eq!(folded(pose_cols(&mut g, pc)), folded(&cols(30)));
        let pcb = g.ch[cc].anim.p_b;
        assert_eq!(folded(pose_cols(&mut g, pcb)), folded(&cols(31)));
        // the one who left: out of the pool, his floats kept
        assert!(pb.at >= Pose::LOOSE);
        assert!(g.tab.pose_set[g.ch[cb].anim.poses].home.is_none());
        assert_eq!(folded(pose_cols(&mut g, pb)), folded(&cols(20)));
        assert_eq!(folded(pose_cols(&mut g, rag)), folded(&cols(77)), "a pose of its own is not the pool's to move");
        // nothing more to do the tick after
        let before = used(&g);
        crate::posehome::pose_home_tend(&mut g);
        assert_eq!(used(&g), before);
        // he comes back: in again at the end, as one who arrived
        g.guards.push(b);
        crate::posehome::pose_home_tend(&mut g);
        assert_eq!(g.pose_sets.len(), 3);
        assert_eq!(used(&g), 6 * 80);
        let pb = g.ch[cb].anim.p_a;
        assert!(pb.at < Pose::LOOSE && pb.at > g.ch[cc].anim.p_b.at);
        assert_eq!(folded(pose_cols(&mut g, pb)), folded(&cols(20)));
        let pbb = g.ch[cb].anim.p_b;
        assert_eq!(folded(pose_cols(&mut g, pbb)), folded(&cols(21)));
        let _ = (a, c);
    }

    #[test]
    fn rest_pose_fills_the_rigs_nodes() {
        let (_h, mut g) = hold();
        let mut ch = rig();
        ch.nodes[1].t = [1.0, 2.0, 3.0];
        ch.nodes[1].r = [0.5, 0.5, 0.5, 0.5];
        ch.nodes[1].s = [4.0, 5.0, 6.0];
        let id = alloc::sync::Arc::make_mut(&mut g.art).rig.push(ch);
        let p = pose_pool_new(&mut g, 5);
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
        // the pool has room again: the set going home takes the stray in (the TypeScript's poseSetIn tries each pose again)
        assert!(pose_set_in(&mut g, s));
        assert!(g.tab.pose_set[s].list[1].at < Pose::LOOSE);
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
        let clip = alloc::sync::Arc::make_mut(&mut g.art).clip.push(crate::anim_gltf::Clip {
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
        assert_ne!(c.word, 0);
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
        let id = alloc::sync::Arc::make_mut(&mut g.art).rig.push(ch);
        let e = |name: &str, w: f64, subtree: bool| BuildMaskEntriesItem { name: String::from(name), w, subtree };
        let mask = build_mask(&mut g, id, &[e("arm", 0.5, true), e("root", 1.0, false), e("none", 1.0, true)]);
        assert_eq!(mask, vec![1.0, 0.0, 0.0, 0.5, 0.5]);
    }
}
