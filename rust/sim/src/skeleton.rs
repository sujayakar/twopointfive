//! POSING — src/anim/skeleton.ts: a local pose blended into another, a pose run forward into every node's model-space rotation and origin, a node turned in
//! its own frame or about the model's axes, and the two-bone solve the animator's layers are built from. Each is the TypeScript's, statement for statement:
//! the same operands in the same order, so the same doubles, and every pose value stored as the single the TypeScript's Float32Array stores (single.rs).
//!
//! What a function works on is handed to it as slices. The animator's layers are layers.rs and ask here; under the row `poses` (src/sim/sim.ts) a character's
//! poses live in its game's block (poses.rs) and a question is asked of them THERE, with no copy; a pose that is an array of the host's (a function's
//! scratch) is written onto the BENCH below, asked about, and read back. A call a question still costs more than it saves, so the row is a proof and
//! not a speed-up: the layers that move here next call these with no crossing at all.
//!
//! The lengths and the angles are the game's own (maths.rs): the host asks only where those stand in its `Math` (math/exact.ts EXACT.on).

use crate::maths::{acos, atan2, cos, hypot3, hypot4, sin};
use crate::single::{get, put, S};

/// nodes a rig on the bench can have (the game's has 55)
pub const MAX_NODES: usize = 128;
/// poses the bench holds
pub const POSES: usize = 4;
/// singles a node of a local pose: tx ty tz | qx qy qz qw | sx sy sz
pub const NODE: usize = 10;
/// A POSE IS TEN COLUMNS, A NODE A ROW (src/anim/skeleton.ts Pose): channel `c` of node `i` is single `c * n + i` of it, `n` = its rows — the nodes
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

#[repr(C, align(16))]
pub struct Bench {
    /// a pose run forward: every node's model-space rotation (x y z w), then every node's origin (x y z)
    pub rot: [f64; MAX_NODES * 4],
    pub pos: [f64; MAX_NODES * 3],
    /// a question's own numbers, and its answer where that is a few doubles
    pub args: [f64; ARGS],
    pub pose: [S; POSES * MAX_NODES * NODE],
    pub mask: [S; MAX_NODES],
    /// the rig: every node's parent (−1 = none), and the nodes parents first
    pub parent: [i32; MAX_NODES],
    pub order: [u32; MAX_NODES],
    pub nodes: u32,
}
static mut BENCH: Bench = Bench {
    rot: [0.0; MAX_NODES * 4],
    pos: [0.0; MAX_NODES * 3],
    args: [0.0; ARGS],
    pose: [S::ZERO; POSES * MAX_NODES * NODE],
    mask: [S::ZERO; MAX_NODES],
    parent: [0; MAX_NODES],
    order: [0; MAX_NODES],
    nodes: 0,
};
// (a pose on the bench starts on a 16-byte boundary too, like one in a game's block: a kernel works either)
const _: () = assert!(core::mem::offset_of!(Bench, pose) % 16 == 0 && (MAX_NODES * NODE * 4) % 16 == 0);
/// the bench (one thread; nothing on it outlives the question it was written for)
#[inline(always)]
pub fn bench() -> &'static mut Bench {
    unsafe { &mut *core::ptr::addr_of_mut!(BENCH) }
}
/// where a table of the bench is (a byte address): 0 rot · 1 pos · 2 args · 3 pose · 4 mask · 5 parent · 6 order
pub fn addr(table: u32) -> usize {
    let b = bench();
    match table {
        0 => b.rot.as_ptr() as usize,
        1 => b.pos.as_ptr() as usize,
        2 => b.args.as_ptr() as usize,
        3 => b.pose.as_ptr() as usize,
        4 => b.mask.as_ptr() as usize,
        5 => b.parent.as_ptr() as usize,
        6 => b.order.as_ptr() as usize,
        _ => 0,
    }
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
/// vec.ts clamp
#[inline(always)]
fn clamp(x: f64, a: f64, b: f64) -> f64 {
    if x < a {
        a
    } else if x > b {
        b
    } else {
        x
    }
}
/// Math.abs
#[inline(always)]
fn abs(x: f64) -> f64 {
    f64::from_bits(x.to_bits() & 0x7fff_ffff_ffff_ffff)
}

/// how many doubles a question's own numbers may be (the IK's eleven; a layer's numbers, the state it keeps between ticks, the hand pass's columns and a
/// path of keys handed to it: layers.rs)
pub const ARGS: usize = 1024;

/// A RIG AS A FUNCTION SEES IT
pub struct Rig<'a> {
    pub parent: &'a [i32],
    pub order: &'a [u32],
}

/// skeleton.ts blendPose: out = lerp(a, b, w · mask[node]) — `a`, `b` and `out` are places on one store of poses (out may be a's)
/// `in_singles` = the rule POSEMATH.singles (single.rs): the same blend worked in singles, four nodes a step, a rotation's length a plain root.
pub fn blend(store: &mut [S], a: usize, b: usize, w: f64, mask: Option<&[S]>, out: usize, n: usize, in_singles: bool) {
    let r = rows(n);
    if in_singles {
        return crate::single::blend4(store, a, b, crate::single::weight(w), mask, out, n, r);
    }
    for i in 0..n {
        let t = match mask {
            Some(m) => w * get(m, i),
            None => w,
        };
        if t <= 0.0 {
            if out != a {
                for c in 0..NODE {
                    store[out + c * r + i] = store[a + c * r + i];
                }
            }
            continue;
        }
        if t >= 1.0 {
            for c in 0..NODE {
                store[out + c * r + i] = store[b + c * r + i];
            }
            continue;
        }
        for c in 0..3 {
            let (av, bv) = (get(store, a + c * r + i), get(store, b + c * r + i));
            put(store, out + c * r + i, av + (bv - av) * t);
        }
        for c in 7..10 {
            let (av, bv) = (get(store, a + c * r + i), get(store, b + c * r + i));
            put(store, out + c * r + i, av + (bv - av) * t);
        }
        let (qa, qb, qo) = (a + 3 * r + i, b + 3 * r + i, out + 3 * r + i);
        let (mut bx, mut by, mut bz, mut bw) = (get(store, qb), get(store, qb + r), get(store, qb + 2 * r), get(store, qb + 3 * r));
        let (ax, ay, az, aw) = (get(store, qa), get(store, qa + r), get(store, qa + 2 * r), get(store, qa + 3 * r));
        if ax * bx + ay * by + az * bz + aw * bw < 0.0 {
            bx = -bx;
            by = -by;
            bz = -bz;
            bw = -bw;
        }
        let (x, y, z, ww) = (ax + (bx - ax) * t, ay + (by - ay) * t, az + (bz - az) * t, aw + (bw - aw) * t);
        let l = or_one(hypot4(x, y, z, ww));
        put(store, qo, x / l);
        put(store, qo + r, y / l);
        put(store, qo + 2 * r, z / l);
        put(store, qo + 3 * r, ww / l);
    }
}

/// one node of PoseFK.run: its rotation and origin from its parent's and its own local row
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
/// PoseFK.run: every node, parents first
pub fn fk_run(rig: &Rig, d: &[S], rot: &mut [f64], pos: &mut [f64]) {
    let n = d.len() / NODE;
    for &i in rig.order {
        let i = i as usize;
        fk_node(d, n, rot, pos, i, rig.parent[i]);
    }
}
/// PoseFK.runFrom: `root` and what hangs under it, in the rig's order (the others are current and are left)
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
/// PoseFK.parentRot: the model-space rotation of node i's parent, walked up the pose
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
/// Pose.setRot of a rotation made unit (quat.normalize): node i's local rotation
pub fn set_unit(d: &mut [S], i: usize, q: &[f64; 4]) {
    let n = d.len() / NODE;
    store_unit(d, 3 * n + i, n, q[0], q[1], q[2], q[3]);
}
/// skeleton.ts rotateLocal: local ← local · q
pub fn rotate_local(d: &mut [S], i: usize, q: &[f64; 4]) {
    let n = d.len() / NODE;
    let o = 3 * n + i;
    let (ax, ay, az, aw) = (get(d, o), get(d, o + n), get(d, o + 2 * n), get(d, o + 3 * n));
    let (bx, by, bz, bw) = (q[0], q[1], q[2], q[3]);
    let (x, y, z, w) = (aw * bx + ax * bw + ay * bz - az * by, aw * by - ax * bz + ay * bw + az * bx, aw * bz + ax * by - ay * bx + az * bw, aw * bw - ax * bx - ay * by - az * bz);
    store_unit(d, o, n, x, y, z, w);
}
/// skeleton.ts rotateModel: local ← P⁻¹ · q · P · local, P = the parent's model rotation
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
/// skeleton.ts setModelRot: node i gets the model-space rotation `rot` outright
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

/// skeleton.ts twoBoneIK. `rot` / `pos` are the pose run forward: current on entry (all of it if `fresh`), current again on exit.
#[allow(clippy::too_many_arguments)]
pub fn two_bone_ik(
    rig: &Rig, d: &mut [S], rot: &mut [f64], pos: &mut [f64], a: usize, b: usize, c: usize, target: &[f64; 3], pole: Option<&[f64; 3]>, pole_w: f64, end_rot: Option<&[f64; 4]>, fresh: bool,
) {
    let (pa, pb, pc) = ([pos[a * 3], pos[a * 3 + 1], pos[a * 3 + 2]], [pos[b * 3], pos[b * 3 + 1], pos[b * 3 + 2]], [pos[c * 3], pos[c * 3 + 1], pos[c * 3 + 2]]);
    let (la, lb) = (hypot3(pa[0] - pb[0], pa[1] - pb[1], pa[2] - pb[2]), hypot3(pb[0] - pc[0], pb[1] - pc[1], pb[2] - pc[2]));
    let (tt0, tt1, tt2) = (target[0] - pa[0], target[1] - pa[1], target[2] - pa[2]);
    let ttl = hypot3(tt0, tt1, tt2);
    let dist = clamp(ttl, abs(la - lb) + 0.01, (la + lb) * 0.998);
    // 1. bend at b for the reach
    let (mut x, mut y, mut z) = (pb[0] - pa[0], pb[1] - pa[1], pb[2] - pa[2]);
    let mut l = or_one(hypot3(x, y, z));
    let (u0, u1, u2) = (x / l, y / l, z / l);
    x = pc[0] - pb[0];
    y = pc[1] - pb[1];
    z = pc[2] - pb[2];
    l = or_one(hypot3(x, y, z));
    let (w0, w1, w2) = (x / l, y / l, z / l);
    let bend_now = acos(clamp(u0 * w0 + u1 * w1 + u2 * w2, -1.0, 1.0));
    let bend_want = core::f64::consts::PI - acos(clamp((la * la + lb * lb - dist * dist) / (2.0 * la * lb), -1.0, 1.0));
    let (mut ax0, mut ax1, mut ax2) = (u1 * w2 - u2 * w1, u2 * w0 - u0 * w2, u0 * w1 - u1 * w0);
    let al = hypot3(ax0, ax1, ax2);
    let (pl0, pl1, pl2) = match pole {
        Some(p) => (p[0], p[1], p[2]),
        None => (0.0, 0.0, 1.0),
    };
    let (mut pa0, mut pa1, mut pa2) = (pl1 * u2 - pl2 * u1, pl2 * u0 - pl0 * u2, pl0 * u1 - pl1 * u0);
    if hypot3(pa0, pa1, pa2) < 1e-6 {
        pa0 = 1.0;
        pa1 = 0.0;
        pa2 = 0.0;
    }
    l = or_one(hypot3(pa0, pa1, pa2));
    pa0 = pa0 / l;
    pa1 = pa1 / l;
    pa2 = pa2 / l;
    if al < 1e-5 {
        ax0 = pa0;
        ax1 = pa1;
        ax2 = pa2;
    } else {
        let ia = 1.0 / al;
        ax0 = ax0 * ia;
        ax1 = ax1 * ia;
        ax2 = ax2 * ia;
        if ax0 * pa0 + ax1 * pa1 + ax2 * pa2 < 0.0 && bend_now < 0.26 {
            ax0 = ax0 * -1.0;
            ax1 = ax1 * -1.0;
            ax2 = ax2 * -1.0;
        }
        let k = clamp((bend_now - 0.09) / 0.17, 0.0, 1.0);
        x = pa0 + (ax0 - pa0) * k;
        y = pa1 + (ax1 - pa1) * k;
        z = pa2 + (ax2 - pa2) * k;
        l = or_one(hypot3(x, y, z));
        ax0 = x / l;
        ax1 = y / l;
        ax2 = z / l;
    }
    let mut q = [0.0f64; 4];
    {
        let sn = sin((bend_want - bend_now) / 2.0);
        q[0] = ax0 * sn;
        q[1] = ax1 * sn;
        q[2] = ax2 * sn;
        q[3] = cos((bend_want - bend_now) / 2.0);
    }
    rotate_model(d, b, &q, &rot_of(rot, a));
    if fresh {
        fk_run_from(rig, d, rot, pos, b);
    } else {
        fk_run(rig, d, rot, pos);
    }
    // 2. swing a so the end lands on the target line
    let par_a = rig.parent[a];
    let p_rot_a = if par_a >= 0 { rot_of(rot, par_a as usize) } else { [0.0, 0.0, 0.0, 1.0] };
    let tl1 = or_one(ttl);
    let (s0, s1, s2) = (tt0 / tl1, tt1 / tl1, tt2 / tl1);
    {
        x = pos[c * 3] - pos[a * 3];
        y = pos[c * 3 + 1] - pos[a * 3 + 1];
        z = pos[c * 3 + 2] - pos[a * 3 + 2];
        l = or_one(hypot3(x, y, z));
        let (f0, f1, f2) = (x / l, y / l, z / l);
        let dd = f0 * s0 + f1 * s1 + f2 * s2;
        if dd < -0.9999 {
            // quat.fromTo's own turn-about for two opposite directions
            let (e0, e1, e2) = if abs(f0) < 0.9 { (1.0, 0.0, 0.0) } else { (0.0, 1.0, 0.0) };
            let (c0, c1, c2) = (f1 * e2 - f2 * e1, f2 * e0 - f0 * e2, f0 * e1 - f1 * e0);
            let cl = or_one(hypot3(c0, c1, c2));
            q[0] = c0 / cl;
            q[1] = c1 / cl;
            q[2] = c2 / cl;
            q[3] = 0.0;
        } else {
            let (c0, c1, c2, c3) = (f1 * s2 - f2 * s1, f2 * s0 - f0 * s2, f0 * s1 - f1 * s0, 1.0 + dd);
            let ql = or_one(hypot4(c0, c1, c2, c3));
            q[0] = c0 / ql;
            q[1] = c1 / ql;
            q[2] = c2 / ql;
            q[3] = c3 / ql;
        }
        rotate_model(d, a, &q, &p_rot_a);
    }
    fk_run_from(rig, d, rot, pos, a);
    // 3. swivel the bend plane toward the pole about the a→target axis
    if let Some(pole) = pole {
        if pole_w > 0.0 {
            let (e0, e1, e2) = (pos[b * 3] - pos[a * 3], pos[b * 3 + 1] - pos[a * 3 + 1], pos[b * 3 + 2] - pos[a * 3 + 2]);
            let (es, ps) = (-(e0 * s0 + e1 * s1 + e2 * s2), -(pole[0] * s0 + pole[1] * s1 + pole[2] * s2));
            let (ep0, ep1, ep2) = (e0 + s0 * es, e1 + s1 * es, e2 + s2 * es);
            let (pp0, pp1, pp2) = (pole[0] + s0 * ps, pole[1] + s1 * ps, pole[2] + s2 * ps);
            let (el, ppl) = (hypot3(ep0, ep1, ep2), hypot3(pp0, pp1, pp2));
            if el > 1e-4 && ppl > 1e-4 {
                let (en0, en1, en2) = (ep0 / el, ep1 / el, ep2 / el);
                let (pn0, pn1, pn2) = (pp0 / ppl, pp1 / ppl, pp2 / ppl);
                let ang = atan2((en1 * pn2 - en2 * pn1) * s0 + (en2 * pn0 - en0 * pn2) * s1 + (en0 * pn1 - en1 * pn0) * s2, en0 * pn0 + en1 * pn1 + en2 * pn2);
                let sn = sin(ang * pole_w / 2.0);
                q[0] = s0 * sn;
                q[1] = s1 * sn;
                q[2] = s2 * sn;
                q[3] = cos(ang * pole_w / 2.0);
                rotate_model(d, a, &q, &p_rot_a);
                fk_run_from(rig, d, rot, pos, a);
            }
        }
    }
    // 4. the end's own rotation
    if let Some(er) = end_rot {
        set_model_rot(d, c, er, &rot_of(rot, b));
        fk_run_from(rig, d, rot, pos, c);
    }
}

// ---- THE QUESTIONS (the module's exports: lib.rs)
const SLOT: usize = MAX_NODES * NODE;
/// the rig on the bench has `nodes` nodes, `order` of them listed parents first
pub fn bench_rig(nodes: u32, order: u32) {
    let b = bench();
    b.nodes = if (nodes as usize) <= MAX_NODES && order <= nodes { nodes | (order << 16) } else { 0 };
}
/// the rig on the bench, borrowed table by table (so that a walk may write the bench's other tables)
macro_rules! rig_of {
    ($b:ident) => {{
        let (n, m) = (($b.nodes & 0xffff) as usize, ($b.nodes >> 16) as usize);
        (n, Rig { parent: &$b.parent[..n], order: &$b.order[..m] })
    }};
}
/// IS THIS A RIG THE WALKS ABOVE CAN WALK (asked once a rig): the order lists every node once, and a node's parent (if it has one) is listed before it. So
/// every index is a node, a run forward meets a parent before its children, and a walk up the parents ends.
pub fn bench_rig_ok() -> bool {
    let b = bench();
    let (n, rig) = rig_of!(b);
    if n == 0 || rig.order.len() != n {
        return false;
    }
    let mut seen = [false; MAX_NODES];
    for &i in rig.order {
        let i = i as usize;
        if i >= n || seen[i] {
            return false;
        }
        let p = rig.parent[i];
        if p >= 0 && (p as usize >= n || !seen[p as usize]) {
            return false;
        }
        seen[i] = true;
    }
    true
}
// WHERE A POSE IS, for a question: `g == ON_BENCH` → place `at` of the bench (the host wrote it there and reads the answer back) · else in game `g`'s pool
// (poses.rs), starting at single `at` of that part, and it is read and written THERE. One body a question, whichever it is. The rig, the mask, a
// question's own numbers and a pose run forward are the bench's either way. A question answers `true` when it did the work; `false` = that is not a pose
// (not a place of the bench, not a pose the pool handed out, not a node), and nothing was written.
use crate::poses::{holds, Poses, HEAD};
use crate::state::state;
pub const ON_BENCH: usize = u32::MAX as usize;
#[inline(always)]
fn pool(g: usize) -> &'static mut Poses {
    &mut state(g).poses
}
/// where a pose of `nodes` nodes starts in its array (the bench's `pose`, or the pool's `data`)
#[inline(always)]
fn start(g: usize, at: usize, nodes: usize) -> Option<usize> {
    if nodes > MAX_NODES {
        None
    } else if g == ON_BENCH {
        if at < POSES {
            Some(at * SLOT)
        } else {
            None
        }
    } else {
        let s = at.wrapping_sub(HEAD / 4);
        if holds(pool(g), s, nodes) {
            Some(s)
        } else {
            None
        }
    }
}
/// …and the array itself
macro_rules! poses_of {
    ($b:ident, $g:expr) => {
        if $g == ON_BENCH {
            &mut $b.pose[..]
        } else {
            &mut pool($g).data[..]
        }
    };
}
/// A LAYER'S WAY IN (layers.rs): the singles of the pose at `at` (of game `g`'s pool, or of the bench), the rig on the bench and its count of nodes, handed to
/// `f` — or None where a question would answer false
pub fn with_pose<R>(g: usize, at: usize, f: impl FnOnce(&mut [S], &Rig, usize, &mut Fk) -> R) -> Option<R> {
    let b = bench();
    let (n, rig) = rig_of!(b);
    let s = start(g, at, n)?;
    let mut fk = Fk { rot: &mut b.rot[..], pos: &mut b.pos[..] };
    Some(f(&mut poses_of!(b, g)[s..s + singles(n)], &rig, n, &mut fk))
}
/// THE RUN-FORWARD BONES on the bench (PoseFK.rot / .pos): what a layer runs forward it finds here, and the host reads back the nodes a layer says it ran
pub struct Fk<'a> {
    pub rot: &'a mut [f64],
    pub pos: &'a mut [f64],
}
impl Fk<'_> {
    /// CharacterAnimator.fkPart: PoseFK.run for the nodes of `list` alone (a list holds every ancestor of a node it holds, parents first; a node the rig has
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
/// blendPose: poses `a`, `b` → `out`, all three in the one array
pub fn ask_blend(g: usize, a: usize, b: usize, w: f64, masked: bool, out: usize, n: usize, in_singles: bool) -> bool {
    let (Some(a), Some(b), Some(out)) = (start(g, a, n), start(g, b, n), start(g, out, n)) else {
        return false;
    };
    let bn = bench();
    let d = poses_of!(bn, g);
    blend(d, a, b, w, if masked { Some(&bn.mask[..]) } else { None }, out, n, in_singles);
    true
}
/// skeleton.ts buildMask, onto the bench's mask (what a masked `ask_blend` then reads): an entry is a node, its weight, and whether its whole subtree takes
/// that weight. A later entry lies over an earlier one; a node the rig has not is skipped; every other node is nought.
pub fn ask_mask(entries: &[(usize, f64, bool)]) {
    let b = bench();
    let (n, rig) = rig_of!(b);
    for m in b.mask[..n].iter_mut() {
        *m = S::ZERO;
    }
    for &(root, w, sub) in entries {
        if root >= n {
            continue;
        }
        put(&mut b.mask, root, w);
        if sub {
            // (parents first: a node is in the subtree when its parent is)
            let mut inside = [false; MAX_NODES];
            inside[root] = true;
            for &i in rig.order {
                let i = i as usize;
                let par = rig.parent[i];
                if par >= 0 && inside[par as usize] {
                    inside[i] = true;
                    put(&mut b.mask, i, w);
                }
            }
        }
    }
}
/// sampleClip: clip `c` (clips.rs) at time `t` into the pose — only the nodes the clip has tracks for are written. `nodes` = the pose's own count
/// (a sample needs no rig); false also for a clip the table does not hold, or one that writes a node the pose has not
pub fn ask_sample(g: usize, at: usize, nodes: usize, c: usize, t: f64, looped: bool, in_singles: bool) -> bool {
    let need = crate::clips::nodes_of(c);
    if need == 0 || need > nodes {
        return false;
    }
    let Some(s) = start(g, at, nodes) else {
        return false;
    };
    let b = bench();
    crate::clips::sample(&mut poses_of!(b, g)[s..s + singles(nodes)], c, t, looped, in_singles);
    true
}
/// PoseFK.run (root < 0) / runFrom
pub fn ask_run(g: usize, at: usize, root: i32) -> bool {
    let b = bench();
    let (n, rig) = rig_of!(b);
    let Some(s) = start(g, at, n) else {
        return false;
    };
    if root >= n as i32 {
        return false;
    }
    let d = &poses_of!(b, g)[s..s + singles(n)];
    if root < 0 {
        fk_run(&rig, d, &mut b.rot, &mut b.pos);
    } else {
        fk_run_from(&rig, d, &mut b.rot, &mut b.pos, root as usize);
    }
    true
}
/// PoseFK.parentRot → args 0‥3
pub fn ask_parent_rot(g: usize, at: usize, i: usize) -> bool {
    let b = bench();
    let (n, rig) = rig_of!(b);
    let Some(s) = start(g, at, n) else {
        return false;
    };
    if i >= n {
        return false;
    }
    let q = parent_rot(&rig, &poses_of!(b, g)[s..s + singles(n)], i);
    b.args[..4].copy_from_slice(&q);
    true
}
/// 0 rotateLocal (q in args 0‥3) · 1 rotateModel (q, then the parent's rotation in 4‥7) · 2 setModelRot (the rotation, then the parent's). `nodes` = the
/// pose's own count of nodes (a turn needs no rig)
pub fn ask_turn(g: usize, at: usize, nodes: usize, i: usize, how: u32) -> bool {
    let b = bench();
    let Some(s) = start(g, at, nodes) else {
        return false;
    };
    if i >= nodes {
        return true; // (no such node: the TypeScript writes nothing there, and so does this)
    }
    let (q, par) = ([b.args[0], b.args[1], b.args[2], b.args[3]], [b.args[4], b.args[5], b.args[6], b.args[7]]);
    let d = &mut poses_of!(b, g)[s..s + singles(nodes)];
    match how {
        0 => rotate_local(d, i, &q),
        1 => rotate_model(d, i, &q, &par),
        _ => set_model_rot(d, i, &q, &par),
    }
    true
}
/// twoBoneIK: the target in args 0‥2, the pole in 3‥5 (flag 1), its weight in 6, the end's rotation in 7‥10 (flag 2); flag 4 = `fresh`
pub fn ask_ik(g: usize, at: usize, a: usize, bb: usize, c: usize, flags: u32) -> bool {
    let b = bench();
    let (n, rig) = rig_of!(b);
    let Some(s) = start(g, at, n) else {
        return false;
    };
    if a >= n || bb >= n || c >= n {
        return false;
    }
    let (target, pole, end) = ([b.args[0], b.args[1], b.args[2]], [b.args[3], b.args[4], b.args[5]], [b.args[7], b.args[8], b.args[9], b.args[10]]);
    let pole_w = b.args[6];
    let d = &mut poses_of!(b, g)[s..s + singles(n)];
    two_bone_ik(&rig, d, &mut b.rot, &mut b.pos, a, bb, c, &target, if flags & 1 != 0 { Some(&pole) } else { None }, pole_w, if flags & 2 != 0 { Some(&end) } else { None }, flags & 4 != 0);
    true
}
