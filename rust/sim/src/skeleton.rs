//! POSING: a local pose blended into another, a pose run forward into every node's model-space rotation and origin, a node turned in
//! its own frame or about the model's axes, and the two-bone solve the animator's layers are built from. The operands stand in a written order, so the doubles are the
//! same on every machine, and every pose value is stored as a single (single.rs).
//!
//! WHAT IT IS HANDED. A function works on slices, whoever's the poses are; the animator's layers are layers/, over a man (`Body`).
//!
//! The lengths and the angles are the game's own (rust/maths).

use crate::maths::{acos, atan2, hypot4};
use crate::single::{get, put, S};
use game_types::anim_skeleton::roles::{Roles, DIGITS_L, DIGITS_R};
use game_types::m;
use game_types::v3::{self, quat, Quat, V3};

/// nodes a rig can have (the game's has 55)
pub const MAX_NODES: usize = 128;
/// singles a node of a local pose: tx ty tz | qx qy qz qw | sx sy sz
pub const NODE: usize = 10;
/// A POSE IS TEN COLUMNS, A NODE A ROW: channel `c` of node `i` is single `c * n + i` of it, `n` = its rows — the nodes
/// rounded up to whole vectors of four (the rows past the last node are zeros, and stay zeros: nothing here writes them). A column a channel is what
/// lets a kernel work one channel of four nodes in a step. A pose is handed to a function as EXACTLY its singles, so `n` is their count over NODE.
#[inline(always)]
pub const fn rows(nodes: usize) -> usize {
    (nodes + 3) & !3
}
/// singles a pose of `nodes` nodes takes
#[inline(always)]
pub const fn singles(nodes: usize) -> usize {
    rows(nodes) * NODE
}

/// `x || 1` of a length: 1 where it is nothing or no number
#[inline(always)]
fn or_one(l: f64) -> f64 {
    if l == 0.0 || l != l {
        1.0
    } else {
        l
    }
}


/// A RIG AS A FUNCTION SEES IT: its nodes' parents, its nodes parents first, and which play which part
#[derive(Clone, Copy)]
pub struct Rig<'a> {
    pub parent: &'a [i32],
    pub order: &'a [u32],
    pub roles: &'a Roles,
}
impl<'a> Rig<'a> {
    /// the node that plays role `i`
    #[inline(always)]
    pub fn role(&self, i: usize) -> usize {
        self.roles.nodes[i] as usize
    }
    /// list `i` of its nodes, parents first
    #[inline(always)]
    pub fn list(&self, i: usize) -> &'a [u32] {
        &self.roles.lists[i]
    }
    /// the finger joints of a hand (0 the left)
    #[inline(always)]
    pub fn digits(&self, side: usize) -> &'a [u32] {
        self.list(if side == 1 { DIGITS_R } else { DIGITS_L })
    }
}

/// Out = lerp(a, b, w · mask[node]) — `a`, `b` and `out` are places on one store of poses (out may be a's)
/// Worked in singles (single.rs), four nodes a step, a rotation's length a plain root.
pub fn blend(store: &mut [S], a: usize, b: usize, w: f64, mask: Option<&[S]>, out: usize, n: usize) {
    crate::single::blend4(store, a, b, crate::single::weight(w), mask, out, n, rows(n));
}

/// one node run forward: its rotation and origin from its parent's and its own local row
#[inline(always)]
fn fk_node(d: &[S], n: usize, rot: &mut [f64], pos: &mut [f64], i: usize, par: i32) {
    let (rx, ry, rz, rw) = (get(d, 3 * n + i), get(d, 4 * n + i), get(d, 5 * n + i), get(d, 6 * n + i));
    if par < 0 {
        rot[i * 4] = rx;
        rot[i * 4 + 1] = ry;
        rot[i * 4 + 2] = rz;
        rot[i * 4 + 3] = rw;
        pos[i * 3] = get(d, i);
        pos[i * 3 + 1] = get(d, n + i);
        pos[i * 3 + 2] = get(d, 2 * n + i);
        return;
    }
    let p = par as usize;
    let (qx, qy, qz, qw) = (rot[p * 4], rot[p * 4 + 1], rot[p * 4 + 2], rot[p * 4 + 3]);
    let (v0, v1, v2) = (get(d, i), get(d, n + i), get(d, 2 * n + i));
    let (tx, ty, tz) = (2.0 * (qy * v2 - qz * v1), 2.0 * (qz * v0 - qx * v2), 2.0 * (qx * v1 - qy * v0));
    let (t0, t1, t2) = (v0 + qw * tx + (qy * tz - qz * ty), v1 + qw * ty + (qz * tx - qx * tz), v2 + qw * tz + (qx * ty - qy * tx));
    rot[i * 4] = qw * rx + qx * rw + qy * rz - qz * ry;
    rot[i * 4 + 1] = qw * ry - qx * rz + qy * rw + qz * rx;
    rot[i * 4 + 2] = qw * rz + qx * ry - qy * rx + qz * rw;
    rot[i * 4 + 3] = qw * rw - qx * rx - qy * ry - qz * rz;
    pos[i * 3] = pos[p * 3] + t0;
    pos[i * 3 + 1] = pos[p * 3 + 1] + t1;
    pos[i * 3 + 2] = pos[p * 3 + 2] + t2;
}
/// every node run forward, parents first
pub fn fk_run(rig: &Rig, d: &[S], rot: &mut [f64], pos: &mut [f64]) {
    fk_all(rig.parent, rig.order, d, rot, pos);
}
/// …of nodes nobody has cast yet
pub fn fk_all(parent: &[i32], order: &[u32], d: &[S], rot: &mut [f64], pos: &mut [f64]) {
    let n = d.len() / NODE;
    for &i in order {
        let i = i as usize;
        fk_node(d, n, rot, pos, i, parent[i]);
    }
}
/// `root` and what hangs under it, in the rig's order (the others are current and are left)
pub fn fk_run_from(rig: &Rig, d: &[S], rot: &mut [f64], pos: &mut [f64], root: usize) {
    let mut under = [false; MAX_NODES];
    let n = d.len() / NODE;
    for &i in rig.order {
        let i = i as usize;
        let par = rig.parent[i];
        if i != root && !(par >= 0 && under[par as usize]) {
            continue;
        }
        under[i] = true;
        fk_node(d, n, rot, pos, i, par);
    }
}
/// the model-space rotation of node i's parent, walked up the pose
pub fn parent_rot(rig: &Rig, d: &[S], i: usize) -> [f64; 4] {
    let (mut x, mut y, mut z, mut w) = (0.0, 0.0, 0.0, 1.0);
    let mut p = rig.parent[i];
    let n = d.len() / NODE;
    while p >= 0 {
        let o = 3 * n + p as usize;
        let (ax, ay, az, aw) = (get(d, o), get(d, o + n), get(d, o + 2 * n), get(d, o + 3 * n));
        let (bx, by, bz, bw) = (x, y, z, w);
        x = aw * bx + ax * bw + ay * bz - az * by;
        y = aw * by - ax * bz + ay * bw + az * bx;
        z = aw * bz + ax * by - ay * bx + az * bw;
        w = aw * bw - ax * bx - ay * by - az * bz;
        p = rig.parent[p as usize];
    }
    [x, y, z, w]
}

/// a rotation stored as a unit (`/ (hypot || 1)`): what a turn and a sample (clips.rs) end in. `o` = where its x is, `n` = the pose's rows (its y, z and w
/// stand a column on each)
#[inline(always)]
pub(crate) fn store_unit(d: &mut [S], o: usize, n: usize, x: f64, y: f64, z: f64, w: f64) {
    let l = or_one(hypot4(x, y, z, w));
    put(d, o, x / l);
    put(d, o + n, y / l);
    put(d, o + 2 * n, z / l);
    put(d, o + 3 * n, w / l);
}
/// node i's local rotation, of a rotation made unit
pub fn set_unit(d: &mut [S], i: usize, q: &[f64; 4]) {
    let n = d.len() / NODE;
    store_unit(d, 3 * n + i, n, q[0], q[1], q[2], q[3]);
}
/// Local ← local · q
pub fn rotate_local(d: &mut [S], i: usize, q: &[f64; 4]) {
    let n = d.len() / NODE;
    let o = 3 * n + i;
    let (ax, ay, az, aw) = (get(d, o), get(d, o + n), get(d, o + 2 * n), get(d, o + 3 * n));
    let (bx, by, bz, bw) = (q[0], q[1], q[2], q[3]);
    let (x, y, z, w) = (aw * bx + ax * bw + ay * bz - az * by, aw * by - ax * bz + ay * bw + az * bx, aw * bz + ax * by - ay * bx + az * bw, aw * bw - ax * bx - ay * by - az * bz);
    store_unit(d, o, n, x, y, z, w);
}
/// Local ← P⁻¹ · q · P · local, P = the parent's model rotation
pub fn rotate_model(d: &mut [S], i: usize, q: &[f64; 4], parent: &[f64; 4]) {
    let n = d.len() / NODE;
    let o = 3 * n + i;
    let (px, py, pz, pw) = (parent[0], parent[1], parent[2], parent[3]);
    let (rx, ry, rz, rw) = (get(d, o), get(d, o + n), get(d, o + 2 * n), get(d, o + 3 * n));
    let (m1x, m1y, m1z, m1w) = (pw * rx + px * rw + py * rz - pz * ry, pw * ry - px * rz + py * rw + pz * rx, pw * rz + px * ry - py * rx + pz * rw, pw * rw - px * rx - py * ry - pz * rz);
    let (qx, qy, qz, qw) = (q[0], q[1], q[2], q[3]);
    let (m2x, m2y, m2z, m2w) =
        (qw * m1x + qx * m1w + qy * m1z - qz * m1y, qw * m1y - qx * m1z + qy * m1w + qz * m1x, qw * m1z + qx * m1y - qy * m1x + qz * m1w, qw * m1w - qx * m1x - qy * m1y - qz * m1z);
    let (cx, cy, cz) = (-px, -py, -pz);
    let (x, y, z, w) = (pw * m2x + cx * m2w + cy * m2z - cz * m2y, pw * m2y - cx * m2z + cy * m2w + cz * m2x, pw * m2z + cx * m2y - cy * m2x + cz * m2w, pw * m2w - cx * m2x - cy * m2y - cz * m2z);
    store_unit(d, o, n, x, y, z, w);
}
/// Node i gets the model-space rotation `rot` outright
pub fn set_model_rot(d: &mut [S], i: usize, rot: &[f64; 4], parent: &[f64; 4]) {
    let n = d.len() / NODE;
    let o = 3 * n + i;
    let (ax, ay, az, aw) = (-parent[0], -parent[1], -parent[2], parent[3]);
    let (bx, by, bz, bw) = (rot[0], rot[1], rot[2], rot[3]);
    let (x, y, z, w) = (aw * bx + ax * bw + ay * bz - az * by, aw * by - ax * bz + ay * bw + az * bx, aw * bz + ax * by - ay * bx + az * bw, aw * bw - ax * bx - ay * by - az * bz);
    store_unit(d, o, n, x, y, z, w);
}

#[inline(always)]
fn rot_of(rot: &[f64], i: usize) -> [f64; 4] {
    [rot[i * 4], rot[i * 4 + 1], rot[i * 4 + 2], rot[i * 4 + 3]]
}

/// THE TWO-BONE SOLVE. `rot` / `pos` are the pose run forward: the limb and what it hangs from current on entry, the limb and what hangs off it on exit;
/// and, not `fresh`, the whole pose on exit, for a layer that has turned something else since it ran it and reads it after.
#[allow(clippy::too_many_arguments)]
pub fn two_bone_ik(
    rig: &Rig, d: &mut [S], rot: &mut [f64], pos: &mut [f64], a: usize, b: usize, c: usize, target: &[f64; 3], pole: Option<&[f64; 3]>, pole_w: f64, end_rot: Option<&[f64; 4]>, fresh: bool,
) {
    let (pa, pb, pc) = (at3(pos, a), at3(pos, b), at3(pos, c));
    let (la, lb) = (v3::dist(pa, pb), v3::dist(pb, pc));
    let to = v3::sub(*target, pa);
    let far = v3::len(to);
    let reach = v3::clamp(far, m::abs(la - lb) + 0.01, (la + lb) * 0.998);
    rotate_model(d, b, &bend(pa, pb, pc, la, lb, reach, pole), &rot_of(rot, a));
    // (between its turns only the limb's own bones are read: what hangs off them is run forward once, at the end)
    if fresh {
        fk_run_limb(rig, d, rot, pos, b, c);
    } else {
        fk_run(rig, d, rot, pos);
    }
    // swing a so the end lands on the target line
    let par_a = rig.parent[a];
    let p_rot_a = if par_a >= 0 { rot_of(rot, par_a as usize) } else { [0.0, 0.0, 0.0, 1.0] };
    let line = v3::normalize(to);
    let end = v3::normalize(v3::sub(at3(pos, c), at3(pos, a)));
    rotate_model(d, a, &quat::from_to(end, line), &p_rot_a);
    fk_run_limb(rig, d, rot, pos, a, c);
    // swivel the bend plane toward the pole about the a→target axis
    if let Some(q) = pole.filter(|_| pole_w > 0.0).and_then(|pole| swivel(v3::sub(at3(pos, b), at3(pos, a)), *pole, line, pole_w)) {
        rotate_model(d, a, &q, &p_rot_a);
        fk_run_limb(rig, d, rot, pos, a, c);
    }
    // the end's own rotation
    if let Some(er) = end_rot {
        set_model_rot(d, c, er, &rot_of(rot, b));
    }
    fk_run_from(rig, d, rot, pos, a);
}

/// The nodes from `root` down to `end` alone, run forward (`root` first, whatever stands between).
fn fk_run_limb(rig: &Rig, d: &[S], rot: &mut [f64], pos: &mut [f64], root: usize, end: usize) {
    let n = d.len() / NODE;
    let mut way = [0u32; LIMB];
    let (mut k, mut i) = (0, end);
    while i != root && k < LIMB {
        way[k] = i as u32;
        k += 1;
        match rig.parent[i] {
            p if p >= 0 => i = p as usize,
            _ => break,
        }
    }
    if i != root {
        return fk_run_from(rig, d, rot, pos, root);   // (a limb that is no limb: its end does not hang under its root, or is too far under it)
    }
    fk_node(d, n, rot, pos, root, rig.parent[root]);
    for &i in way[..k].iter().rev() {
        fk_node(d, n, rot, pos, i as usize, rig.parent[i as usize]);
    }
}
/// nodes a limb can have between its root and its end
const LIMB: usize = 12;

/// a bone's place in a pose run forward
#[inline(always)]
fn at3(pos: &[f64], i: usize) -> V3 {
    [pos[i * 3], pos[i * 3 + 1], pos[i * 3 + 2]]
}

/// The turn at the middle bone that brings the end to `reach` of the root: about the axis it is bent round now, or the pole's where it is all but straight
/// (a straight limb's own axis is noise: eased from the pole's to its own between 0.09 and 0.26 rad of bend, and never bent back to front).
fn bend(pa: V3, pb: V3, pc: V3, la: f64, lb: f64, reach: f64, pole: Option<&V3>) -> Quat {
    let (u, w) = (v3::normalize(v3::sub(pb, pa)), v3::normalize(v3::sub(pc, pb)));
    let bend_now = acos(v3::clamp(v3::dot(u, w), -1.0, 1.0));
    let bend_want = core::f64::consts::PI - acos(v3::clamp((la * la + lb * lb - reach * reach) / (2.0 * la * lb), -1.0, 1.0));
    let own = v3::cross(u, w);
    let al = v3::len(own);
    let across = v3::cross(pole.copied().unwrap_or([0.0, 0.0, 1.0]), u);
    let poles = v3::normalize(if v3::len(across) < 1e-6 { [1.0, 0.0, 0.0] } else { across });
    let axis = if al < 1e-5 {
        poles
    } else {
        let mut own = v3::scale(own, 1.0 / al);
        if v3::dot(own, poles) < 0.0 && bend_now < 0.26 {
            own = v3::scale(own, -1.0);
        }
        v3::normalize(v3::lerp3(poles, own, v3::clamp((bend_now - 0.09) / 0.17, 0.0, 1.0)))
    };
    quat::axis_angle(axis, bend_want - bend_now)
}

/// The turn about the root's line to the target that brings the bend (`e`: root to middle) `pole_w` of the way round toward the pole.
fn swivel(e: V3, pole: V3, line: V3, pole_w: f64) -> Option<Quat> {
    let ep = v3::mad(e, line, -v3::dot(e, line));
    let pp = v3::mad(pole, line, -v3::dot(pole, line));
    let (el, pl) = (v3::len(ep), v3::len(pp));
    if !(el > 1e-4 && pl > 1e-4) {
        return None;
    }
    let (en, pn) = ([ep[0] / el, ep[1] / el, ep[2] / el], [pp[0] / pl, pp[1] / pl, pp[2] / pl]);
    let ang = atan2(v3::dot(v3::cross(en, pn), line), v3::dot(en, pn));
    Some(quat::axis_angle(line, ang * pole_w))
}

/// A MAN AS A LAYER POSES HIM: his poses, his bones as last run forward, his rig, and the clips he is sampled from.
pub struct Body<'a> {
    pub poses: &'a mut [S],
    pub fk: Fk<'a>,
    pub rig: Rig<'a>,
    pub clips: &'a game_types::ids::Tab<game_types::ids::ClipId, game_types::anim_gltf::Clip>,
    /// the nodes' weights a masked blend goes by
    mask: [S; MAX_NODES],
}
impl<'a> Body<'a> {
    pub fn new(poses: &'a mut [S], fk: Fk<'a>, rig: Rig<'a>, clips: &'a game_types::ids::Tab<game_types::ids::ClipId, game_types::anim_gltf::Clip>) -> Body<'a> {
        Body { poses, fk, rig, clips, mask: [S::ZERO; MAX_NODES] }
    }
    /// how many nodes his rig has
    #[inline(always)]
    pub fn nodes(&self) -> usize {
        self.rig.parent.len()
    }
    /// A LAYER'S WAY IN (layers/): the singles of pose `at`, the rig, and the bones
    #[inline]
    pub fn pose<R>(&mut self, at: usize, f: impl FnOnce(&mut [S], &Rig, &mut Fk) -> R) -> R {
        let one = singles(self.nodes());
        f(&mut self.poses[at * one..(at + 1) * one], &self.rig, &mut self.fk)
    }
    /// poses `a` and `b` into `out`, `w` of `b`; `masked`: by the mask's weight of each node besides
    pub fn blend(&mut self, a: usize, b: usize, w: f64, masked: bool, out: usize) {
        let (n, one) = (self.nodes(), singles(self.nodes()));
        blend(self.poses, a * one, b * one, w, if masked { Some(&self.mask[..]) } else { None }, out * one, n);
    }
    /// clip `c` at time `t` into pose `at`
    pub fn sample(&mut self, at: usize, c: game_types::ids::ClipId, t: f64, looped: bool) {
        let one = singles(self.nodes());
        crate::clips::sample(&self.clips[c], t, looped, &mut self.poses[at * one..(at + 1) * one]);
    }
    /// The mask of the next blend that goes by one: an entry is a node, its weight, and whether its whole subtree takes that weight. A later entry lies over an earlier one; a node the
    /// rig has not is skipped; every other node is nought.
    pub fn mask(&mut self, entries: &[(usize, f64, bool)]) {
        let (n, rig, mask) = (self.rig.parent.len(), &self.rig, &mut self.mask);
        for m in mask[..n].iter_mut() {
            *m = S::ZERO;
        }
        for &(root, w, sub) in entries {
            if root >= n {
                continue;
            }
            put(mask, root, w);
            if sub {
                // (parents first: a node is in the subtree when its parent is)
                let mut inside = [false; MAX_NODES];
                inside[root] = true;
                for &i in rig.order {
                    let i = i as usize;
                    let par = rig.parent[i];
                    if par >= 0 && inside[par as usize] {
                        inside[i] = true;
                        put(mask, i, w);
                    }
                }
            }
        }
    }
    /// pose `from` into pose `to`
    pub fn copy(&mut self, from: usize, to: usize) {
        let one = singles(self.nodes());
        self.poses.copy_within(from * one..(from + 1) * one, to * one);
    }
}
/// THE RUN-FORWARD BONES: every node's model-space rotation (x y z w), then every node's origin
pub struct Fk<'a> {
    pub rot: &'a mut [f64],
    pub pos: &'a mut [f64],
}
impl Fk<'_> {
    /// the run forward for the nodes of `list` alone (a list holds every ancestor of a node it holds, parents first; a node the rig has
    /// not is skipped)
    pub fn part(&mut self, rig: &Rig, d: &[S], list: &[u32]) {
        for &i in list {
            let i = i as usize;
            if i < rig.parent.len() {
                fk_node(d, d.len() / NODE, self.rot, self.pos, i, rig.parent[i]);
            }
        }
    }
    /// a node's origin as last run
    #[inline(always)]
    pub fn at(&self, i: usize) -> [f64; 3] {
        [self.pos[i * 3], self.pos[i * 3 + 1], self.pos[i * 3 + 2]]
    }
    /// a node's model rotation as last run
    #[inline(always)]
    pub fn rot(&self, i: usize) -> [f64; 4] {
        [self.rot[i * 4], self.rot[i * 4 + 1], self.rot[i * 4 + 2], self.rot[i * 4 + 3]]
    }
}
