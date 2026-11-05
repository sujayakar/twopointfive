//! THE ONE FILE OF THE SIMULATION THAT NAMES A SINGLE. The game's arithmetic is in doubles; a local pose is STORED in singles, because the TypeScript keeps it
//! in a Float32Array (src/anim/skeleton.ts Pose). Reading one gives the double it stands for and writing one rounds the double to the nearest single, ties to
//! even: exactly what a Float32Array does. The build's rule against the word stands everywhere else (tools/sim/build.ts RULES names this file as the
//! exception).
//!
//! ONE THING IS WORKED OUT IN SINGLES, AND ONLY UNDER ITS SWITCH (src/anim/skeleton.ts POSEMATH.singles; the maintainer, 2026-11-03: "Singles, plain root for
//! lengths"): the two bulk kernels of a pose, the blend of two poses and the step between two keys of a clip. Every operation there is ONE operation on
//! singles rounded once (an add, a subtract, a multiply, a divide, a root: each exact by the standard, so a lane of four gives what one at a time gives and
//! the TypeScript twin, which rounds each step with Math.fround, gives the same bits), in the order written here, and a rotation's length is the plain root
//! of the sum of squares. With the switch off nothing here runs and a pose is worked in doubles as it always was. The kernels are at the foot of the file.

/// a stored pose value: nothing outside this file can read one, write one or do arithmetic on one but through `get` and `put`
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct S(f32);
impl S {
    pub const ZERO: S = S(0.0);
    /// what a Float32Array holds after `fill(Infinity)`
    pub const INF: S = S(f32::INFINITY);
}
/// the double a stored value stands for
#[inline(always)]
pub fn get(s: &[S], i: usize) -> f64 {
    s[i].0 as f64
}
/// store a double as a Float32Array does
#[inline(always)]
pub fn put(s: &mut [S], i: usize, v: f64) {
    s[i] = S(v as f32);
}

// ---------------------------------------------------------------- the pose kernels in singles (POSEMATH.singles)
use prim::sqrt_single;

/// a weight or a fraction as a kernel in singles takes it: the double rounded once
#[derive(Clone, Copy)]
pub struct W(f32);
#[inline(always)]
pub fn weight(v: f64) -> W {
    W(v as f32)
}
/// `l || 1`: a length that is nothing or no number counts as one
#[inline(always)]
fn or_one(l: f32) -> f32 {
    if l == 0.0 || l != l {
        1.0
    } else {
        l
    }
}
/// A ROTATION BETWEEN TWO, one at a time — the rule's order of operations in the plainest words (the lanes below are this, four at once): the shorter way
/// round, each number a + (b − a)·t, made a unit again by the plain root of ((x² + y²) + z²) + w²
#[inline(always)]
fn unit_step(a: [f32; 4], b: [f32; 4], t: f32) -> [f32; 4] {
    let [ax, ay, az, aw] = a;
    let [mut bx, mut by, mut bz, mut bw] = b;
    if ((ax * bx + ay * by) + az * bz) + aw * bw < 0.0 {
        bx = -bx;
        by = -by;
        bz = -bz;
        bw = -bw;
    }
    let (x, y, z, w) = (ax + (bx - ax) * t, ay + (by - ay) * t, az + (bz - az) * t, aw + (bw - aw) * t);
    let l = or_one(sqrt_single(((x * x + y * y) + z * z) + w * w));
    [x / l, y / l, z / l, w / l]
}
/// between two keys of three numbers (a clip's origin or scale track): `pool[a..]` toward `pool[b..]` by `f`, into `d[o]`, `d[o + stride]`, `d[o + 2 stride]`
#[inline(always)]
pub fn step3(d: &mut [S], o: usize, stride: usize, pool: &[S], a: usize, b: usize, f: W) {
    for c in 0..3 {
        let (av, bv) = (pool[a + c].0, pool[b + c].0);
        d[o + c * stride] = S(av + (bv - av) * f.0);
    }
}
/// between two keys of a rotation
#[inline(always)]
pub fn step4(d: &mut [S], o: usize, stride: usize, pool: &[S], a: usize, b: usize, f: W) {
    let q = unit_step([pool[a].0, pool[a + 1].0, pool[a + 2].0, pool[a + 3].0], [pool[b].0, pool[b + 1].0, pool[b + 2].0, pool[b + 3].0], f.0);
    for c in 0..4 {
        d[o + c * stride] = S(q[c]);
    }
}
/// node `i`'s ten numbers of one pose handed on to another (the rows past the last node are nobody's: never read here, never written)
#[inline(always)]
fn hand_on(store: &mut [S], from: usize, to: usize, n: usize, rows: usize) {
    if from == to {
        return;
    }
    for c in 0..10 {
        for i in 0..n {
            store[to + c * rows + i] = store[from + c * rows + i];
        }
    }
}
/// skeleton.ts blendPose in singles: out = a toward b by `w` (times the mask's weight of the node, where there is a mask). `a`, `b` and `out` are places on
/// one store of poses of `rows` rows (out may be a's or b's), `n` the nodes. A weight of nought or less hands `a`'s numbers on and one of one or more `b`'s,
/// with nothing worked out; a weight that is no number takes the blend, as the comparisons say. ONLY THE NODES' ROWS ARE WRITTEN: the rows past the last node
/// stay as they stand, as in the doubles' blend and the TypeScript's.
pub fn blend4(store: &mut [S], a: usize, b: usize, w: W, mask: Option<&[S]>, out: usize, n: usize, rows: usize) {
    let end = 10 * rows;
    assert!(rows % 4 == 0 && n <= rows && a + end <= store.len() && b + end <= store.len() && out + end <= store.len());
    let Some(m) = mask else {
        // (one weight for every node: the two ends are a copy, and a layer's `blend by one` is how it copies a pose)
        if w.0 <= 0.0 {
            return hand_on(store, a, out, n, rows);
        }
        if w.0 >= 1.0 {
            return hand_on(store, b, out, n, rows);
        }
        return blend_rows(store, a, b, w.0, None, out, n, rows);
    };
    assert!(rows <= m.len());
    blend_rows(store, a, b, w.0, Some(m), out, n, rows)
}
/// …the rows themselves: FOUR NODES A STEP where the module is WebAssembly (fixed-width SIMD: every lane is the one-at-a-time operation, rounded once, so the
/// bits are `unit_step`'s), one at a time anywhere else the crate is compiled (a check on the building machine)
#[cfg(target_arch = "wasm32")]
fn blend_rows(store: &mut [S], a: usize, b: usize, w: f32, mask: Option<&[S]>, out: usize, n: usize, rows: usize) {
    // SAFETY: the three places lie whole inside `store` and the mask holds a number a row (asserted by blend4); a step loads its four rows of both poses
    // before it stores any, so `out` on `a` or on `b` reads what was there.
    unsafe { lanes::blend(store.as_mut_ptr() as *mut f32, a, b, w, mask.map(|m| m.as_ptr() as *const f32), out, n, rows) }
}
#[cfg(not(target_arch = "wasm32"))]
fn blend_rows(store: &mut [S], a: usize, b: usize, w: f32, mask: Option<&[S]>, out: usize, n: usize, rows: usize) {
    for i in 0..n {
        let t = match mask {
            Some(m) => w * m[i].0,
            None => w,
        };
        let at = |p: usize, c: usize| p + c * rows + i;
        if t <= 0.0 {
            for c in 0..10 {
                store[at(out, c)] = store[at(a, c)];
            }
            continue;
        }
        if t >= 1.0 {
            for c in 0..10 {
                store[at(out, c)] = store[at(b, c)];
            }
            continue;
        }
        for c in [0usize, 1, 2, 7, 8, 9] {
            let (av, bv) = (store[at(a, c)].0, store[at(b, c)].0);
            store[at(out, c)] = S(av + (bv - av) * t);
        }
        let q = unit_step(
            [store[at(a, 3)].0, store[at(a, 4)].0, store[at(a, 5)].0, store[at(a, 6)].0],
            [store[at(b, 3)].0, store[at(b, 4)].0, store[at(b, 5)].0, store[at(b, 6)].0],
            t,
        );
        for c in 0..4 {
            store[at(out, 3 + c)] = S(q[c]);
        }
    }
}
#[cfg(target_arch = "wasm32")]
mod lanes {
    use core::arch::wasm32::*;
    #[target_feature(enable = "simd128")]
    pub unsafe fn blend(p: *mut f32, a: usize, b: usize, w: f32, mask: Option<*const f32>, out: usize, n: usize, rows: usize) {
        let (pa, pb, po) = (p.add(a) as *const f32, p.add(b) as *const f32, p.add(out));
        let ld = |q: *const f32, c: usize, i: usize| v128_load(q.add(c * rows + i) as *const v128);
        let (zero, one, wv, nv) = (f32x4_splat(0.0), f32x4_splat(1.0), f32x4_splat(w), i32x4_splat(n as i32));
        let mut i = 0;
        while i < n {
            // the lanes that are nodes: a lane past the last node is given weight nought (whatever stands in the mask there) and is not written
            let live = i32x4_lt(i32x4(i as i32, i as i32 + 1, i as i32 + 2, i as i32 + 3), nv);
            let whole = i + 4 <= n;
            let t = match mask {
                Some(m) => f32x4_mul(wv, v128_load(m.add(i) as *const v128)),
                None => wv,
            };
            let t = v128_and(t, live);
            // t <= 0: a's own numbers · t >= 1: b's · else the blend
            let (lo, hi) = (f32x4_le(t, zero), f32x4_ge(t, one));
            let pick = |blend: v128, av: v128, bv: v128| v128_bitselect(av, v128_bitselect(bv, blend, hi), lo);
            let put = |c: usize, v: v128| {
                let at = po.add(c * rows + i) as *mut v128;
                v128_store(at, if whole { v } else { v128_bitselect(v, v128_load(at as *const v128), live) });
            };
            let mut c = 0;
            while c < 10 {
                if c == 3 {
                    c = 7;
                }
                let (av, bv) = (ld(pa, c, i), ld(pb, c, i));
                put(c, pick(f32x4_add(av, f32x4_mul(f32x4_sub(bv, av), t)), av, bv));
                c += 1;
            }
            let (ax, ay, az, aw) = (ld(pa, 3, i), ld(pa, 4, i), ld(pa, 5, i), ld(pa, 6, i));
            let (bx0, by0, bz0, bw0) = (ld(pb, 3, i), ld(pb, 4, i), ld(pb, 5, i), ld(pb, 6, i));
            let dot = f32x4_add(f32x4_add(f32x4_add(f32x4_mul(ax, bx0), f32x4_mul(ay, by0)), f32x4_mul(az, bz0)), f32x4_mul(aw, bw0));
            let neg = f32x4_lt(dot, zero);
            let (bx, by, bz, bw) = (v128_bitselect(f32x4_neg(bx0), bx0, neg), v128_bitselect(f32x4_neg(by0), by0, neg), v128_bitselect(f32x4_neg(bz0), bz0, neg), v128_bitselect(f32x4_neg(bw0), bw0, neg));
            let x = f32x4_add(ax, f32x4_mul(f32x4_sub(bx, ax), t));
            let y = f32x4_add(ay, f32x4_mul(f32x4_sub(by, ay), t));
            let z = f32x4_add(az, f32x4_mul(f32x4_sub(bz, az), t));
            let ww = f32x4_add(aw, f32x4_mul(f32x4_sub(bw, aw), t));
            let l = f32x4_sqrt(f32x4_add(f32x4_add(f32x4_add(f32x4_mul(x, x), f32x4_mul(y, y)), f32x4_mul(z, z)), f32x4_mul(ww, ww)));
            let l = v128_bitselect(one, l, v128_or(f32x4_eq(l, zero), f32x4_ne(l, l)));
            put(3, pick(f32x4_div(x, l), ax, bx0));
            put(4, pick(f32x4_div(y, l), ay, by0));
            put(5, pick(f32x4_div(z, l), az, bz0));
            put(6, pick(f32x4_div(ww, l), aw, bw0));
            i += 4;
        }
    }
}
