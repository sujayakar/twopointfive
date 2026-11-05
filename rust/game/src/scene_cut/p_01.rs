//! src/scene/cut.ts lines 1 to 247
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// Whoever emits cut pieces asks this. Settled: the page has the switch on.   (cut.ts cutBoxesOn)
pub fn cut_boxes_on(g: &mut crate::game::Game) -> bool {
    true
}

/// A plane from any normal (normalised here) and its offset along that normal's length: keeps n · p ≤ d.   (cut.ts cutPlane)
pub fn cut_plane(n: V3, d: f64) -> crate::scene_cut::CutPlane {
    let l = m::hyp3(n[0], n[1], n[2]);
    if !(l > 1e-12) {
        return CutPlane { n: [0.0, 0.0, 0.0], d: 1.0 };
    }
    CutPlane { n: [n[0] / l, n[1] / l, n[2] / l], d: d / l }
}

/// The plane through local point p whose normal n points at what is removed.   (cut.ts cutThrough)
pub fn cut_through(p: V3, n: V3) -> crate::scene_cut::CutPlane {
    let l = m::hyp3(n[0], n[1], n[2]);
    if !(l > 1e-12) {
        return CutPlane { n: [0.0, 0.0, 0.0], d: 1.0 };
    }
    let u: V3 = [n[0] / l, n[1] / l, n[2] / l];
    CutPlane { n: u, d: u[0] * p[0] + u[1] * p[1] + u[2] * p[2] }
}

/// The farthest the uncut box reaches along n.   (cut.ts cutSupport)
pub fn cut_support(h: V3, n: V3) -> f64 {
    m::abs(n[0]) * h[0] + m::abs(n[1]) * h[1] + m::abs(n[2]) * h[2]
}

/// The corners of the cut solid (local frame): every triple of its bounding planes that meets in a point inside all the others.   (cut.ts cutVerts)
pub fn cut_verts(h: V3, cut: Option<&[crate::scene_cut::CutPlane]>) -> Vec<V3> {
    let mut pl: Vec<CutPlane> = vec![
        CutPlane { n: [1.0, 0.0, 0.0], d: h[0] },
        CutPlane { n: [-1.0, 0.0, 0.0], d: h[0] },
        CutPlane { n: [0.0, 1.0, 0.0], d: h[1] },
        CutPlane { n: [0.0, -1.0, 0.0], d: h[1] },
        CutPlane { n: [0.0, 0.0, 1.0], d: h[2] },
        CutPlane { n: [0.0, 0.0, -1.0], d: h[2] },
    ];
    if let Some(cut) = cut {
        for c in cut {
            if c.n[0] != 0.0 || c.n[1] != 0.0 || c.n[2] != 0.0 {
                pl.push(*c);
            }
        }
    }
    let scale = m::max_n(&[h[0], h[1], h[2], 1e-6]);
    let eps = 1e-7 * scale + 1e-9;
    let mut out: Vec<V3> = Vec::new();
    for i in 0..pl.len() {
        for j in (i + 1)..pl.len() {
            for k in (j + 1)..pl.len() {
                let (a, b, c) = (pl[i].n, pl[j].n, pl[k].n);
                let bcx = b[1] * c[2] - b[2] * c[1];
                let bcy = b[2] * c[0] - b[0] * c[2];
                let bcz = b[0] * c[1] - b[1] * c[0];
                let det = a[0] * bcx + a[1] * bcy + a[2] * bcz;
                if m::abs(det) < 1e-9 {
                    continue;
                }
                let cax = c[1] * a[2] - c[2] * a[1];
                let cay = c[2] * a[0] - c[0] * a[2];
                let caz = c[0] * a[1] - c[1] * a[0];
                let abx = a[1] * b[2] - a[2] * b[1];
                let aby = a[2] * b[0] - a[0] * b[2];
                let abz = a[0] * b[1] - a[1] * b[0];
                let p: V3 = [
                    (pl[i].d * bcx + pl[j].d * cax + pl[k].d * abx) / det,
                    (pl[i].d * bcy + pl[j].d * cay + pl[k].d * aby) / det,
                    (pl[i].d * bcz + pl[j].d * caz + pl[k].d * abz) / det,
                ];
                let mut ok = true;
                let mut q = 0;
                while q < pl.len() && ok {
                    if pl[q].n[0] * p[0] + pl[q].n[1] * p[1] + pl[q].n[2] * p[2] > pl[q].d + eps {
                        ok = false;
                    }
                    q += 1;
                }
                if !ok {
                    continue;
                }
                let mut dup = false;
                for q in &out {
                    if m::abs(q[0] - p[0]) + m::abs(q[1] - p[1]) + m::abs(q[2] - p[2]) < 4.0 * eps {
                        dup = true;
                        break;
                    }
                }
                if !dup {
                    out.push(p);
                }
            }
        }
    }
    out
}

/// How far into the box the plane reaches.   (cut.ts cutRecord's depth)
fn depth(h: V3, c: &CutPlane) -> f64 {
    cut_support(h, c.n) - c.d
}

/// THE BUILDER: a valid record for a box of half-extents h from any list of planes.   (cut.ts cutRecord)
pub fn cut_record(h: V3, planes: &[crate::scene_cut::CutPlane]) -> crate::scene_cut::CutResult {
    let mut list: Vec<CutPlane> = Vec::new();
    for raw in planes {
        let c = cut_plane(raw.n, raw.d);
        if c.n[0] == 0.0 && c.n[1] == 0.0 && c.n[2] == 0.0 {
            continue;
        }
        let s = cut_support(h, c.n);
        let tol = 1e-6 * m::max_n(&[h[0], h[1], h[2]]) + 1e-8;
        if c.d >= s - tol {
            continue;
        }
        if c.d <= -s + tol {
            return CutResult { cut: None, empty: true, dropped: 0.0 };
        }
        let mut twin: i32 = -1;
        let mut i = 0;
        while i < list.len() && twin < 0 {
            let o = list[i].n;
            if o[0] * c.n[0] + o[1] * c.n[1] + o[2] * c.n[2] > 1.0 - 1e-9 {
                twin = i as i32;
            }
            i += 1;
        }
        if twin >= 0 {
            if c.d < list[twin as usize].d {
                list[twin as usize] = c;
            }
            continue;
        }
        list.push(c);
    }
    if list.is_empty() {
        return CutResult { cut: None, empty: false, dropped: 0.0 };
    }
    let verts = cut_verts(h, Some(&list));
    if verts.len() < 4 {
        return CutResult { cut: None, empty: true, dropped: 0.0 };
    }
    {
        let (mut x0, mut x1, mut y0, mut y1, mut z0, mut z1) = (m::INF, -m::INF, m::INF, -m::INF, m::INF, -m::INF);
        for v in &verts {
            x0 = m::min(x0, v[0]);
            x1 = m::max(x1, v[0]);
            y0 = m::min(y0, v[1]);
            y1 = m::max(y1, v[1]);
            z0 = m::min(z0, v[2]);
            z1 = m::max(z1, v[2]);
        }
        let tiny = 1e-5;
        if x1 - x0 < tiny || y1 - y0 < tiny || z1 - z0 < tiny {
            return CutResult { cut: None, empty: true, dropped: 0.0 };
        }
    }
    let scale = m::max_n(&[h[0], h[1], h[2]]);
    let eps = 1e-6 * scale + 1e-8;
    list.retain(|c| {
        let mut on = 0;
        for v in &verts {
            if m::abs(c.n[0] * v[0] + c.n[1] * v[1] + c.n[2] * v[2] - c.d) < eps {
                on += 1;
            }
        }
        on >= 3
    });
    let max_planes = MAX_CUT_PLANES as usize;
    let mut dropped = 0.0;
    if list.len() > max_planes {
        dropped = (list.len() - max_planes) as f64;
        let mut tagged: Vec<(CutPlane, usize)> = list.iter().enumerate().map(|(i, c)| (*c, i)).collect();
        js::sort_by(&mut tagged, |a, b| js::or(depth(h, &b.0) - depth(h, &a.0), a.1 as f64 - b.1 as f64));
        tagged.truncate(max_planes);
        js::sort_by(&mut tagged, |a, b| a.1 as f64 - b.1 as f64);
        list = tagged.iter().map(|e| e.0).collect();
    }
    CutResult { cut: if list.is_empty() { None } else { Some(list) }, empty: false, dropped }
}

/// The cut solid's own bounds in the box's local frame.   (cut.ts cutBounds)
pub fn cut_bounds(h: V3, cut: Option<&[crate::scene_cut::CutPlane]>) -> crate::scene_cut::CutBounds {
    let cut = match cut {
        Some(c) if !c.is_empty() => c,
        _ => return CutBounds { min: [-h[0], -h[1], -h[2]], max: [h[0], h[1], h[2]] },
    };
    let mut min: V3 = [m::INF, m::INF, m::INF];
    let mut max: V3 = [-m::INF, -m::INF, -m::INF];
    for v in cut_verts(h, Some(cut)) {
        for a in 0..3 {
            if v[a] < min[a] {
                min[a] = v[a];
            }
            if v[a] > max[a] {
                max[a] = v[a];
            }
        }
    }
    if !(min[0] <= max[0]) {
        return CutBounds { min: [0.0, 0.0, 0.0], max: [0.0, 0.0, 0.0] };
    }
    CutBounds { min, max }
}

/// The same solid in the tightest box of the same orientation.   (cut.ts cutFit)
pub fn cut_fit(h: V3, cut: Option<&[crate::scene_cut::CutPlane]>) -> crate::scene_cut::CutFit {
    let cut = match cut {
        Some(c) if !c.is_empty() => c,
        _ => return CutFit { off: [0.0, 0.0, 0.0], h: [h[0], h[1], h[2]], cut: None },
    };
    let b = cut_bounds(h, Some(cut));
    let off: V3 = [(b.min[0] + b.max[0]) / 2.0, (b.min[1] + b.max[1]) / 2.0, (b.min[2] + b.max[2]) / 2.0];
    let nh: V3 = [(b.max[0] - b.min[0]) / 2.0, (b.max[1] - b.min[1]) / 2.0, (b.max[2] - b.min[2]) / 2.0];
    let moved: Vec<CutPlane> = cut
        .iter()
        .map(|c| CutPlane { n: [c.n[0], c.n[1], c.n[2]], d: c.d - (c.n[0] * off[0] + c.n[1] * off[1] + c.n[2] * off[2]) })
        .collect();
    CutFit { off, h: nh, cut: cut_record(nh, &moved).cut }
}

/// Rotate a local vector into the world by a box's orientation (rot if it has one, else its yaw).   (cut.ts cutToWorld)
pub fn cut_to_world(v: V3, yaw: f64, rot: Option<Quat>) -> V3 {
    if let Some(rot) = rot {
        let (qx, qy, qz, qw) = (rot[0], rot[1], rot[2], rot[3]);
        let tx = 2.0 * (qy * v[2] - qz * v[1]);
        let ty = 2.0 * (qz * v[0] - qx * v[2]);
        let tz = 2.0 * (qx * v[1] - qy * v[0]);
        return [v[0] + qw * tx + (qy * tz - qz * ty), v[1] + qw * ty + (qz * tx - qx * tz), v[2] + qw * tz + (qx * ty - qy * tx)];
    }
    let cs = m::cos(yaw);
    let sn = m::sin(yaw);
    [cs * v[0] + sn * v[2], v[1], -sn * v[0] + cs * v[2]]
}

/// A prism from a convex outline, extruded ± half_t along `axis` (0, 1 or 2).   (cut.ts cutPrism)
pub fn cut_prism(axis: f64, half_t: f64, poly: &[[f64; 2]]) -> crate::scene_cut::CutPrism {
    let axis = axis as usize;
    let (ua, va) = ((axis + 1) % 3, (axis + 2) % 3);
    let (mut u0, mut u1, mut v0, mut v1, mut area2) = (m::INF, -m::INF, m::INF, -m::INF, 0.0);
    for i in 0..poly.len() {
        let p = poly[i];
        let q = poly[(i + 1) % poly.len()];
        u0 = m::min(u0, p[0]);
        u1 = m::max(u1, p[0]);
        v0 = m::min(v0, p[1]);
        v1 = m::max(v1, p[1]);
        area2 += p[0] * q[1] - q[0] * p[1];
    }
    let mut off: V3 = [0.0, 0.0, 0.0];
    let mut h: V3 = [0.0, 0.0, 0.0];
    if poly.len() < 3 || !(m::abs(area2) > 1e-12) || !(half_t > 0.0) {
        return CutPrism { off, h, cut: None, dropped: 0.0, empty: true };
    }
    let (cu, cv) = ((u0 + u1) / 2.0, (v0 + v1) / 2.0);
    off[ua] = cu;
    off[va] = cv;
    h[axis] = half_t;
    h[ua] = (u1 - u0) / 2.0;
    h[va] = (v1 - v0) / 2.0;
    let sgn = if area2 > 0.0 { 1.0 } else { -1.0 };
    let mut planes: Vec<CutPlane> = Vec::new();
    for i in 0..poly.len() {
        let p = poly[i];
        let q = poly[(i + 1) % poly.len()];
        let (eu, ev) = (q[0] - p[0], q[1] - p[1]);
        if m::abs(eu) + m::abs(ev) < 1e-12 {
            continue;
        }
        let mut n: V3 = [0.0, 0.0, 0.0];
        n[ua] = sgn * ev;
        n[va] = -sgn * eu;
        let mut at: V3 = [0.0, 0.0, 0.0];
        at[ua] = p[0] - cu;
        at[va] = p[1] - cv;
        planes.push(cut_through(at, n));
    }
    let r = cut_record(h, &planes);
    CutPrism { off, h, cut: r.cut, dropped: r.dropped, empty: r.empty }
}

/// THE CPU CLIP every ray / segment test shares: [t0, t1] of the local line o + t·d cut down by the record; the answer in `g.m.scene_cut.cut_clip`, false = nothing left (and it is not written).   (cut.ts cutClip)
pub fn cut_clip(g: &mut crate::game::Game, cut: &[crate::scene_cut::CutPlane], ox: f64, oy: f64, oz: f64, dx: f64, dy: f64, dz: f64, t0: f64, t1: f64) -> bool {
    let (mut t0, mut t1) = (t0, t1);
    let mut k = -1.0;
    let mut i = 0;
    while i < cut.len() && i < MAX_CUT_PLANES as usize {
        let n = cut[i].n;
        if n[0] == 0.0 && n[1] == 0.0 && n[2] == 0.0 {
            break;
        }
        let den = n[0] * dx + n[1] * dy + n[2] * dz;
        let num = cut[i].d - (n[0] * ox + n[1] * oy + n[2] * oz);
        if den < 0.0 {
            let t = num / den;
            if t > t0 {
                t0 = t;
                k = i as f64;
            }
        } else if den > 0.0 {
            let t = num / den;
            if t < t1 {
                t1 = t;
            }
        } else if num < 0.0 {
            return false;
        }
        if t0 > t1 {
            return false;
        }
        i += 1;
    }
    let out = &mut g.m.scene_cut.cut_clip;
    out.t0 = t0;
    out.t1 = t1;
    out.k = k;
    true
}

#[cfg(test)]
mod tests {
    //! The TypeScript's own bits (bun, EXACT on), by the same inputs.
    use super::*;
    const H: V3 = [1.5, 0.7, 2.1];

    const PLANE: &[u64] = &[0x4010000000000000, 0x3fd5555555555555, 0x3fe5555555555555, 0xbfe5555555555555, 0x3fd3333333333333, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x3ff0000000000000, 0x3fd513220631ab5d, 0xbfe18ff1afd40ecd, 0x3fe896525c8f47ec, 0x3fc3ab7516d90656, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x3ff0000000000000, 0x400228f5c28f5c29];
    const VERTS: &[u64] = &[0xbff8000000000000, 0x3fe6666666666666, 0xc000cccccccccccd, 0xbff8000000000000, 0x3fe6666666666666, 0xbfdfffffffffffff, 0xbff8000000000000, 0xbfe6666666666666, 0xc000cccccccccccd, 0xbff8000000000000, 0xbfe6666666666666, 0x3feccccccccccccc, 0xbfd3333333333332, 0x3fe6666666666666, 0xc000cccccccccccd, 0xbfd3333333333332, 0x3fe6666666666666, 0xbfe0000000000000, 0x3ff1999999999999, 0xbfe6666666666666, 0xc000cccccccccccd, 0x3ff199999999999a, 0xbfe6666666666666, 0x3feccccccccccccd];
    const RECORD_TWIN: &[u64] = &[0x3ff0000000000000, 0x3ff0000000000000, 0x0000000000000000, 0x0000000000000000, 0x3fc999999999999a, 0x0000000000000000, 0x0000000000000000];
    const RECORD_ALL_KEPT: &[u64] = &[0xbff0000000000000, 0x0000000000000000, 0x0000000000000000];
    const RECORD_NONE_LEFT: &[u64] = &[0xbff0000000000000, 0x3ff0000000000000, 0x0000000000000000];
    const RECORD_SLIVER: &[u64] = &[0xbff0000000000000, 0x3ff0000000000000, 0x0000000000000000];
    const RECORD_RING: &[u64] = &[0x4020000000000000, 0x3fe3907117678914, 0x3fc91a556151761d, 0x3fe8886327b738b2, 0x3fe9bafdea26b2ab, 0x3fcbedf7fb64ad4d, 0x3fc91a556151761d, 0x3fee9783d051553d, 0x3fea0b522e9150f2, 0xbfcbedf7fb64ad47, 0x3fc91a556151761b, 0x3fee9783d051553c, 0x3fea5ba672fbef37, 0xbfe3907117678912, 0x3fc91a556151761d, 0x3fe8886327b738b3, 0x3feaabfab7668d7f, 0xbfe3907117678914, 0x3fc91a556151761b, 0xbfe8886327b738b0, 0x3febed4bc9110698, 0xbfcbedf7fb64ad52, 0x3fc91a556151761d, 0xbfee9783d051553d, 0x3fec3da00d7ba4e0, 0x3fcbedf7fb64ad24, 0x3fc91a556151761b, 0xbfee9783d051553d, 0x3fec8df451e64325, 0x3fe3907117678911, 0x3fc91a556151761b, 0xbfe8886327b738b3, 0x3fecde489650e16c, 0x0000000000000000, 0x4018000000000000];
    const BOUNDS: &[u64] = &[0xbff8000000000000, 0xbfe6666666666666, 0xc000cccccccccccd, 0x3ff199999999999a, 0x3fe6666666666666, 0x3feccccccccccccd];
    const FIT: &[u64] = &[0xbfc9999999999998, 0x0000000000000000, 0xbfe3333333333334, 0x3ff4cccccccccccd, 0x3fe6666666666666, 0x3ff8000000000000, 0x4000000000000000, 0x3fe6a09e667f3bcc, 0x3fe6a09e667f3bcc, 0x0000000000000000, 0x3fdb27247aff148e, 0x0000000000000000, 0x3fe6a09e667f3bcc, 0x3fe6a09e667f3bcc, 0x3fe21a1851ff630a];
    const WORLD: &[u64] = &[0xbfcc5a1a5d37f154, 0x3fe0000000000000, 0xbfe75123f304c2d0, 0xbfd59c38b04ab607, 0x3fe3fd0d0678c006, 0xbfe241dd1a21ea35];
    const PRISM_0: &[u64] = &[0x0000000000000000, 0x3fd3333333333334, 0xbfc9999999999994, 0x3fa999999999999a, 0x3ff4cccccccccccd, 0x3febb67ae8584cab, 0x4010000000000000, 0x0000000000000000, 0x3fe997da62ee70a0, 0x3fe33587439aacdf, 0x3ff0a2b459e7c935, 0x0000000000000000, 0xbfe997da62ee709e, 0x3fe33587439aace0, 0x3ff0a2b459e7c934, 0x0000000000000000, 0xbfe997da62ee70a1, 0xbfe33587439aacde, 0x3ff0a2b459e7c936, 0x0000000000000000, 0x3fe997da62ee70a0, 0xbfe33587439aace0, 0x3ff0a2b459e7c935, 0x0000000000000000, 0x0000000000000000];
    const PRISM_1_CW: &[u64] = &[0xbfc999999999999c, 0x0000000000000000, 0x3fcb663db1ab1ec8, 0x3fee6f0e134454ff, 0x3fc999999999999a, 0x3fea0cc29f9dfae6, 0x4010000000000000, 0x3fe18344e2ec784e, 0x0000000000000000, 0xbfeac852a2c8cfd3, 0x3fe5cd71274cc277, 0x3fee193cb5e0cd69, 0x0000000000000000, 0x3fd5bb84bada0638, 0x3fea8989823c5e48, 0xbfee193cb5e0cd67, 0x0000000000000000, 0x3fd5bb84bada063d, 0x3fea8989823c5e47, 0xbfe18344e2ec784f, 0x0000000000000000, 0xbfeac852a2c8cfd3, 0x3fe5cd71274cc276, 0x0000000000000000, 0x0000000000000000];
    const PRISM_2_MANY: &[u64] = &[0x3fd3333333333334, 0xbfc999999999999c, 0x0000000000000000, 0x3ff0000000000000, 0x3fef329c0558e96b, 0x3fb999999999999a, 0x4020000000000000, 0x3fe904c37505de4c, 0x3fe3f3a0e28bedd0, 0x0000000000000000, 0x3fef329c0558e969, 0x3fdbc4c04d71abc0, 0x3fecd4bca9cb5c72, 0x0000000000000000, 0x3fef329c0558e96a, 0xbfdbc4c04d71abc4, 0x3fecd4bca9cb5c71, 0x0000000000000000, 0x3fef329c0558e96b, 0xbfe904c37505de4b, 0x3fe3f3a0e28bedd1, 0x0000000000000000, 0x3fef329c0558e969, 0xbfe904c37505de4c, 0xbfe3f3a0e28bedd1, 0x0000000000000000, 0x3fef329c0558e969, 0xbfdbc4c04d71abc8, 0xbfecd4bca9cb5c6f, 0x0000000000000000, 0x3fef329c0558e968, 0x3fdbc4c04d71abb1, 0xbfecd4bca9cb5c74, 0x0000000000000000, 0x3fef329c0558e968, 0x3fe904c37505de4f, 0xbfe3f3a0e28bedcd, 0x0000000000000000, 0x3fef329c0558e967, 0x4010000000000000, 0x0000000000000000];
    const PRISM_FLAT: &[u64] = &[0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0xbff0000000000000, 0x0000000000000000, 0x3ff0000000000000];
    const CLIP: &[u64] = &[0x3ff0000000000000, 0xbff0000000000000, 0xbfe5555555555555, 0xbff0000000000000, 0x3ff0000000000000, 0x4008000000000000, 0x4022000000000000, 0x3ff0000000000000, 0x0000000000000000, 0x4008000000000000, 0x4022000000000000, 0x3ff0000000000000, 0x3ff0000000000000, 0x0000000000000000, 0x4000000000000000, 0xbff0000000000000, 0x0000000000000000, 0x0000000000000000, 0x4000000000000000, 0xbff0000000000000];


    fn same(name: &str, got: Vec<f64>, want: &[u64]) {
        assert_eq!(got.len(), want.len(), "{name}: length");
        for (i, (a, b)) in got.iter().zip(want).enumerate() {
            assert_eq!(a.to_bits(), *b, "{name}: number {i}");
        }
    }
    fn pl(n: V3, d: f64) -> CutPlane {
        CutPlane { n, d }
    }
    fn flat_planes(c: &Option<Vec<CutPlane>>) -> Vec<f64> {
        match c {
            None => vec![-1.0],
            Some(c) => {
                let mut v = vec![c.len() as f64];
                for p in c {
                    v.extend_from_slice(&p.n);
                    v.push(p.d);
                }
                v
            }
        }
    }
    fn flat_record(r: &CutResult) -> Vec<f64> {
        let mut v = flat_planes(&r.cut);
        v.push(if r.empty { 1.0 } else { 0.0 });
        v.push(r.dropped);
        v
    }
    fn flat_prism(p: &CutPrism) -> Vec<f64> {
        let mut v = Vec::new();
        v.extend_from_slice(&p.off);
        v.extend_from_slice(&p.h);
        v.extend(flat_planes(&p.cut));
        v.push(p.dropped);
        v.push(if p.empty { 1.0 } else { 0.0 });
        v
    }
    fn wedge() -> Vec<CutPlane> {
        vec![cut_plane([1.0, 1.0, 0.0], 0.4), cut_plane([0.0, 1.0, 1.0], 0.2)]
    }
    fn ring(n: usize) -> Vec<CutPlane> {
        (0..n)
            .map(|i| {
                let t = i as f64 * 2.0 * m::PI / n as f64;
                pl([m::cos(t), 0.2, m::sin(t)], 0.8 + i as f64 * 0.01)
            })
            .collect()
    }
    fn gon(n: usize, sx: f64) -> Vec<[f64; 2]> {
        (0..n)
            .map(|i| {
                let t = i as f64 * 2.0 * m::PI / n as f64;
                [sx * m::cos(t) + 0.3, m::sin(t) - 0.2]
            })
            .collect()
    }

    #[test]
    fn planes_and_support() {
        let mut v = flat_planes(&Some(vec![
            cut_plane([1.0, 2.0, -2.0], 0.9),
            cut_plane([0.0, 0.0, 0.0], 1.0),
            cut_through([0.1, 0.2, 0.3], [0.3, -0.5, 0.7]),
            cut_through([1.0, 1.0, 1.0], [0.0, 0.0, 0.0]),
        ]));
        v.push(cut_support(H, [0.3, -0.5, 0.7]));
        same("PLANE", v, PLANE);
    }
    #[test]
    fn verts_bounds_fit() {
        let w = wedge();
        same("VERTS", cut_verts(H, Some(&w)).iter().flatten().copied().collect(), VERTS);
        let b = cut_bounds(H, Some(&w));
        same("BOUNDS", b.min.iter().chain(b.max.iter()).copied().collect(), BOUNDS);
        let f = cut_fit(H, Some(&w));
        let mut v = Vec::new();
        v.extend_from_slice(&f.off);
        v.extend_from_slice(&f.h);
        v.extend(flat_planes(&f.cut));
        same("FIT", v, FIT);
    }
    #[test]
    fn the_builder() {
        let twin = [pl([2.0, 0.0, 0.0], 1.0), pl([1.0, 0.0, 0.0], 0.2), pl([3.0, 0.0, 0.0], 2.7), pl([0.0, 0.0, 0.0], 1.0)];
        same("RECORD_TWIN", flat_record(&cut_record(H, &twin)), RECORD_TWIN);
        same("RECORD_ALL_KEPT", flat_record(&cut_record(H, &[pl([1.0, 0.0, 0.0], 1.5)])), RECORD_ALL_KEPT);
        same("RECORD_NONE_LEFT", flat_record(&cut_record(H, &[pl([1.0, 0.0, 0.0], -1.5)])), RECORD_NONE_LEFT);
        same("RECORD_SLIVER", flat_record(&cut_record(H, &[pl([1.0, 0.0, 0.0], 0.1), pl([-1.0, 0.0, 0.0], -0.1)])), RECORD_SLIVER);
        // 14 planes, 6 over the limit: the deepest eight, in the order given
        let r = cut_record(H, &ring(14));
        assert_eq!(r.dropped, 6.0);
        same("RECORD_RING", flat_record(&r), RECORD_RING);
    }
    #[test]
    fn to_world() {
        let mut v = Vec::new();
        v.extend_from_slice(&cut_to_world([0.3, 0.5, -0.7], 0.7, None));
        v.extend_from_slice(&cut_to_world([0.3, 0.5, -0.7], 0.7, Some([0.1, 0.2, 0.3, 0.927])));
        same("WORLD", v, WORLD);
    }
    #[test]
    fn prisms() {
        same("PRISM_0", flat_prism(&cut_prism(0.0, 0.05, &gon(6, 1.3))), PRISM_0);
        same("PRISM_1_CW", flat_prism(&cut_prism(1.0, 0.2, &gon(5, -0.9))), PRISM_1_CW);
        same("PRISM_2_MANY", flat_prism(&cut_prism(2.0, 0.1, &gon(14, 1.0))), PRISM_2_MANY);
        same("PRISM_FLAT", flat_prism(&cut_prism(2.0, 0.1, &[[0.0, 0.0], [1.0, 1.0]])), PRISM_FLAT);
    }
    #[test]
    fn the_clip_keeps_its_old_answer_on_false() {
        let mut g = Game::blank();
        let w = wedge();
        let mut v = Vec::new();
        let mut run = |g: &mut Game, r: bool| {
            let c = g.m.scene_cut.cut_clip;
            v.extend_from_slice(&[if r { 1.0 } else { 0.0 }, c.t0, c.t1, c.k]);
        };
        let r = cut_clip(&mut g, &w, -3.0, 0.1, 0.2, 1.0, 0.1, 0.05, -1.0, 5.0);
        run(&mut g, r);
        let r = cut_clip(&mut g, &w, 3.0, 3.0, 0.2, -1.0, -1.0, 0.0, 0.0, 9.0);
        run(&mut g, r);
        let r = cut_clip(&mut g, &w, 3.0, 3.0, 3.0, 0.0, 0.0, 0.0, 0.0, 9.0);
        run(&mut g, r);
        let stop = [pl([0.0, 0.0, 0.0], 1.0), w[0]];
        let r = cut_clip(&mut g, &stop, 5.0, 5.0, 5.0, 1.0, 0.0, 0.0, 0.0, 2.0);
        run(&mut g, r);
        let r = cut_clip(&mut g, &w, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 0.5, 0.6);
        run(&mut g, r);
        same("CLIP", v, CLIP);
    }
}
