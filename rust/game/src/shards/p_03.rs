//! src/game/shards.ts lines 670 to 764
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// x held between lo and hi; lo wins a crossed pair.   (shards.ts clampI)
pub fn clamp_i(x: f64, lo: f64, hi: f64) -> f64 {
    m::max(lo, m::min(hi, x))
}

/// a plate's cut edge: the pane's tint taken k of the way to the edge green.   (shards.ts edgeColour)
pub fn edge_colour(tint: V3, k: f64) -> V3 {
    let q = m::min(1.0, k);
    let e = crate::shards::edge_green();
    [tint[0] + (e[0] - tint[0]) * q, tint[1] + (e[1] - tint[1]) * q, tint[2] + (e[2] - tint[2]) * q]
}

/// a CUT plate's face colour: dark body in the air, lifted toward the edge at rest, the floor through the tint when opaque at rest.   (shards.ts cutFaceAlbedo)
pub fn cut_face_albedo(g: &mut crate::game::Game, tint: V3, under: V3, type_: crate::glassbreak::GlassType, resting: bool) -> V3 {
    let k = g.k.shards.cut_body;
    if !(k > 0.0) {
        return plate_albedo(tint, under, type_);
    }
    if resting && !(g.k.shards.cut_see > 0.0) {
        return [under[0] * tint[0], under[1] * tint[1], under[2] * tint[2]];
    }
    let q = if resting { m::min(1.0, m::max(0.0, g.k.shards.cut_rest_body)) } else { 0.0 };
    let e = crate::shards::edge_green();
    [tint[0] * k + (e[0] - tint[0] * k) * q, tint[1] * k + (e[1] - tint[1] * k) * q, tint[2] * k + (e[2] - tint[2] * k) * q]
}

/// a plate seen through: the surface under it × the tint, a little of the glass's grey-green; laminated is milky.   (shards.ts plateAlbedo)
pub fn plate_albedo(tint: V3, under: V3, type_: crate::glassbreak::GlassType) -> V3 {
    let k = if type_ == crate::glassbreak::GlassType::Laminated { 0.45 } else { 0.82 };
    [
        under[0] * tint[0] * k + tint[0] * (1.0 - k) * 0.6,
        under[1] * tint[1] * k + tint[1] * (1.0 - k) * 0.6,
        under[2] * tint[2] * k + tint[2] * (1.0 - k) * 0.6,
    ]
}

/// the principal in-plane frame of a polygon on a plane of normal n: u along its long side, v across, extents and the middle of its bounds.   (shards.ts frameOf)
pub fn frame_of(outline: &[V3], centre: V3, n: V3) -> crate::shards::GlassShardsCutPlatesFr {
    let t0 = v3::normalize(v3::cross(if m::abs(n[1]) > 0.9 { [1.0, 0.0, 0.0] } else { [0.0, 1.0, 0.0] }, n));
    let b0 = v3::cross(n, t0);
    let (mut cxx, mut cyy, mut cxy) = (0.0, 0.0, 0.0);
    let mut pts: Vec<[f64; 2]> = Vec::new();
    for &p in outline {
        let d = v3::sub(p, centre);
        let (a, b) = (v3::dot(d, t0), v3::dot(d, b0));
        pts.push([a, b]);
        cxx += a * a;
        cyy += b * b;
        cxy += a * b;
    }
    let th = 0.5 * m::atan2(2.0 * cxy, cxx - cyy);
    let (ca, sa) = (m::cos(th), m::sin(th));
    let u: V3 = [t0[0] * ca + b0[0] * sa, t0[1] * ca + b0[1] * sa, t0[2] * ca + b0[2] * sa];
    let v = v3::cross(n, u);
    let (mut u0, mut u1, mut v0, mut v1) = (1e9, -1e9, 1e9, -1e9);
    for &[a, b] in &pts {
        let (pu, pv) = (a * ca + b * sa, -a * sa + b * ca);
        u0 = m::min(u0, pu);
        u1 = m::max(u1, pu);
        v0 = m::min(v0, pv);
        v1 = m::max(v1, pv);
    }
    if !(u1 > u0) || !(v1 > v0) {
        return crate::shards::GlassShardsCutPlatesFr { u, v, lu: 0.05, lv: 0.05, mid: v3::copy(centre) };
    }
    crate::shards::GlassShardsCutPlatesFr { u, v, lu: u1 - u0, lv: v1 - v0, mid: v3::mad(v3::mad(centre, u, (u0 + u1) / 2.0), v, (v0 + v1) / 2.0) }
}

/// twice-signed-area ÷ 2, positive.   (shards.ts polyArea)
pub fn poly_area(p: &[[f64; 2]]) -> f64 {
    let mut a = 0.0;
    for i in 0..p.len() {
        let q = p[(i + 1) % p.len()];
        a += p[i][0] * q[1] - q[0] * p[i][1];
    }
    m::abs(a) / 2.0
}

/// P without repeated points and without points on the straight line between their neighbours.   (shards.ts tidyPoly)
pub fn tidy_poly(p: &[[f64; 2]]) -> Vec<[f64; 2]> {
    let mut q: Vec<[f64; 2]> = Vec::new();
    for &pt in p {
        let keep = match q.last() {
            None => true,
            Some(l) => m::hyp2(pt[0] - l[0], pt[1] - l[1]) > 1e-5,
        };
        if keep {
            q.push(pt);
        }
    }
    while q.len() > 1 && m::hyp2(q[0][0] - q[q.len() - 1][0], q[0][1] - q[q.len() - 1][1]) <= 1e-5 {
        q.pop();
    }
    let mut again = true;
    while again && q.len() > 3 {
        again = false;
        let n = q.len();
        let mut r: Vec<[f64; 2]> = Vec::new();
        for i in 0..n {
            let pt = q[i];
            let (a, b) = (q[(i + n - 1) % n], q[(i + 1) % n]);
            let (ax, ay, bx, by) = (pt[0] - a[0], pt[1] - a[1], b[0] - pt[0], b[1] - pt[1]);
            if m::abs(ax * by - ay * bx) > 1e-4 * m::hyp2(ax, ay) * m::hyp2(bx, by) || ax * bx + ay * by < 0.0 {
                r.push(pt);
            }
        }
        if r.len() < n && r.len() >= 3 {
            q = r;
            again = true;
        }
    }
    q
}

/// a polygon's centre of area; the mean of its points when it has none.   (shards.ts polyCentroid)
pub fn poly_centroid(p: &[[f64; 2]]) -> [f64; 2] {
    let (mut a, mut cx, mut cy) = (0.0, 0.0, 0.0);
    for i in 0..p.len() {
        let (pt, q) = (p[i], p[(i + 1) % p.len()]);
        let w = pt[0] * q[1] - q[0] * pt[1];
        a += w;
        cx += (pt[0] + q[0]) * w;
        cy += (pt[1] + q[1]) * w;
    }
    if m::abs(a) < 1e-12 {
        let (mut mx, mut my) = (0.0, 0.0);
        for pt in p {
            mx += pt[0];
            my += pt[1];
        }
        return if p.is_empty() { [0.0, 0.0] } else { [mx / p.len() as f64, my / p.len() as f64] };
    }
    [cx / (3.0 * a), cy / (3.0 * a)]
}

/// the part of convex polygon P on the side n · p ≤ d; fewer than 3 points back = nothing.   (shards.ts clipHalf)
pub fn clip_half(p: &[[f64; 2]], nx: f64, ny: f64, d: f64) -> Vec<[f64; 2]> {
    let mut out: Vec<[f64; 2]> = Vec::new();
    for i in 0..p.len() {
        let (a, b) = (p[i], p[(i + 1) % p.len()]);
        let (sa, sb) = (nx * a[0] + ny * a[1] - d, nx * b[0] + ny * b[1] - d);
        if sa <= 0.0 {
            out.push([a[0], a[1]]);
        }
        if (sa < 0.0 && sb > 0.0) || (sa > 0.0 && sb < 0.0) {
            let t = sa / (sa - sb);
            out.push([a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t]);
        }
    }
    if out.len() >= 3 { out } else { Vec::new() }
}

/// a cell as the outlines its plates take: itself when a cut record holds its edges, else its two halves across its long side.   (shards.ts cellParts)
pub fn cell_parts(poly: &[[f64; 2]], half_t: f64, seed: f64) -> Vec<Vec<[f64; 2]>> {
    if crate::scene_cut::cut_prism(2.0, half_t, poly).dropped == 0.0 {
        return vec![poly.to_vec()];
    }
    let (mut x0, mut x1, mut y0, mut y1) = (m::INF, -m::INF, m::INF, -m::INF);
    for pt in poly {
        x0 = m::min(x0, pt[0]);
        x1 = m::max(x1, pt[0]);
        y0 = m::min(y0, pt[1]);
        y1 = m::max(y1, pt[1]);
    }
    let ang = (crate::debris::hash01(super::u32((js::to_i32(seed) ^ 0x5B17) as f64)) - 0.5) * 0.5;
    let (ca, sa) = (m::cos(ang), m::sin(ang));
    let along_x = x1 - x0 >= y1 - y0;
    let (nx, ny) = (if along_x { ca } else { -sa }, if along_x { sa } else { ca });
    let d = nx * (x0 + x1) / 2.0 + ny * (y0 + y1) / 2.0;
    let parts: Vec<Vec<[f64; 2]>> = vec![clip_half(poly, nx, ny, d), clip_half(poly, -nx, -ny, -d)].into_iter().filter(|p| p.len() >= 3).collect();
    if parts.is_empty() { vec![poly.to_vec()] } else { parts }
}

/// the box of one cut plate: `poly` in the frame `rot` at `origin` extruded ± halfT; None = a degenerate outline.   (shards.ts cutPlateBox)
pub fn cut_plate_box(g: &mut crate::game::Game, origin: V3, rot: Quat, poly: &[[f64; 2]], half_t: f64, albedo: V3) -> Option<crate::shards::CutPlateBox> {
    let pr = crate::scene_cut::cut_prism(2.0, half_t, poly);
    if pr.empty || !(pr.h[0] > 1e-4) || !(pr.h[1] > 1e-4) {
        return None;
    }
    let c = v3::add(origin, v3::quat::rotate(rot, pr.off));
    let box_ = crate::scene_boxes::make_box(g, &crate::scene_boxes::MakeBoxP {
        c,
        h: pr.h,
        yaw: Some(0.0),
        rot: Some(rot),
        albedo: Some(albedo),
        flags: Some(crate::scene_boxes::box_flag().dynamic),
        ..Default::default()
    });
    let anti = signed_area(poly) >= 0.0;
    let off = pr.off;
    let at = |pt: &[f64; 2]| -> [f64; 2] { [pt[0] - off[0], pt[1] - off[1]] };
    let local: Vec<[f64; 2]> = if anti { poly.iter().map(at).collect() } else { poly.iter().rev().map(at).collect() };
    Some(crate::shards::CutPlateBox { box_, cut: pr.cut, dropped: pr.dropped, poly: local })
}

/// a polygon's area with its winding's sign.   (shards.ts signedArea)
pub fn signed_area(p: &[[f64; 2]]) -> f64 {
    let mut a = 0.0;
    for i in 0..p.len() {
        let q = p[(i + 1) % p.len()];
        a += p[i][0] * q[1] - q[0] * p[i][1];
    }
    a / 2.0
}

/// what fragment_of_box reads of a box: (c, h, yaw, rot, albedo). The box is a dynamic shard's, and a BoxId's table is still undecided.
fn seam_box_read(g: &mut Game, b: BoxId) -> (V3, V3, f64, Option<Quat>, V3) {
    let x = crate::scene_boxes::b_01::any_box(g, b); (x.c, x.h, x.yaw, x.rot, x.albedo) // TEMP-BRINGUP: the bring-up's table of made boxes (scene_boxes/b_01.rs)
}

/// the base's shard BOX read back as the fragment it stands for: its long sides the outline, its thin one the pane's normal, signed as thrown.   (shards.ts fragmentOfBox)
pub fn fragment_of_box(g: &mut crate::game::Game, box_: BoxId, v: V3, w: V3, who: f64, seed: f64) -> crate::shards::GlassFragmentLike {
    let (c, h, yaw, rot, albedo) = seam_box_read(g, box_);
    let rot = match rot {
        Some(r) => r,
        None => v3::quat::yaw(yaw),
    };
    let mut ax: [usize; 3] = [0, 1, 2];
    js::sort_by(&mut ax, |&p, &q| h[p] - h[q]);
    let e: [V3; 3] = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
    let mut n = v3::quat::rotate(rot, e[ax[0]]);
    if v3::dot(n, v) < 0.0 {
        n = v3::scale(n, -1.0);
    }
    let (a, b) = (v3::quat::rotate(rot, e[ax[1]]), v3::quat::rotate(rot, e[ax[2]]));
    let (ha, hb) = (h[ax[1]], h[ax[2]]);
    let corner = |sa: f64, sb: f64| -> V3 {
        [c[0] + a[0] * ha * sa + b[0] * hb * sb, c[1] + a[1] * ha * sa + b[1] * hb * sb, c[2] + a[2] * ha * sa + b[2] * hb * sb]
    };
    let tinted = m::abs(albedo[0] - albedo[2]) > 0.02 || albedo[0] < 0.9;
    crate::shards::GlassFragmentLike {
        pane: -1.0,
        type_: crate::glassbreak::GlassType::Annealed,
        kind: if hb > 2.2 * ha { crate::shards::GlassFragmentLikeKind::Dagger } else { crate::shards::GlassFragmentLikeKind::Piece },
        outline: vec![corner(-1.0, -1.0), corner(1.0, -1.0), corner(1.0, 1.0), corner(-1.0, 1.0)],
        centre: v3::copy(c),
        normal: n,
        thickness: 2.0 * h[ax[0]],
        area: 4.0 * ha * hb,
        velocity: v3::copy(v),
        spin: v3::copy(w),
        tint: if tinted { v3::copy(albedo) } else { [0.72, 0.8, 0.82] },
        grain: 0.0,
        cause: crate::shards::GlassFragmentLikeCause::Hit,
        who,
        seed,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::glassbreak::GlassType;

    // Bits printed by the TypeScript's own functions (EXACT on), under bun.
    fn same(what: &str, got: &[f64], want: &[u64]) {
        let g: Vec<u64> = got.iter().map(|x| x.to_bits()).collect();
        assert_eq!(g, want, "{}", what);
    }
    fn flat(p: &[[f64; 2]]) -> Vec<f64> {
        p.iter().flat_map(|q| [q[0], q[1]]).collect()
    }
    fn fr(f: crate::shards::GlassShardsCutPlatesFr) -> Vec<f64> {
        let mut o = Vec::new();
        o.extend_from_slice(&f.u);
        o.extend_from_slice(&f.v);
        o.push(f.lu);
        o.push(f.lv);
        o.extend_from_slice(&f.mid);
        o
    }
    const TINT: V3 = [0.7213, 0.8011, 0.8197];
    const UNDER: V3 = [0.413, 0.377, 0.291];
    const P: [[f64; 2]; 8] = [[0.1, 0.2], [1.37, 0.21], [1.37, 0.21000001], [2.3, 1.1], [1.9, 2.7], [1.15, 1.95], [0.4, 1.2], [0.1, 0.2]];

    #[test]
    fn colours_match_the_typescripts_bits() {
        same("clamp_a", &[clamp_i(1.5, 0.0, 1.0)], &[0x3ff0000000000000]);
        same("clamp_b", &[clamp_i(-0.0, 0.0, 1.0)], &[0x0000000000000000]);
        same("edge", &edge_colour(TINT, 0.37), &[0x3fe74d91eeaa6d27, 0x3feb479f666234a8, 0x3feab52220bc382a]);
        same("edge2", &edge_colour(TINT, 1.25), &[0x3fe7ae147ae147ae, 0x3fee147ae147ae14, 0x3feb851eb851eb85]);
        same("plate", &plate_albedo(TINT, UNDER, GlassType::Annealed), &[0x3fd49e877d6aed68, 0x3fd5630e264d3582, 0x3fd22f182a8802b2]);
        same("plate_lam", &plate_albedo(TINT, UNDER, GlassType::Laminated), &[0x3fd7d0338ef8abce, 0x3fd99e0472807b10, 0x3fd82e8ad2816ca7]);
    }

    #[test]
    fn cut_faces_match_the_typescripts_bits() {
        let mut g = Game::blank();
        let mut run = |body: f64, see: f64, rest: f64, ty: GlassType, resting: bool| {
            g.k.shards.cut_body = body;
            g.k.shards.cut_see = see;
            g.k.shards.cut_rest_body = rest;
            cut_face_albedo(&mut g, TINT, UNDER, ty, resting)
        };
        same("no_body", &run(0.0, 1.0, 0.3, GlassType::Laminated, true), &[0x3fd7d0338ef8abce, 0x3fd99e0472807b10, 0x3fd82e8ad2816ca7]);
        same("opaque_rest", &run(0.37, 0.0, 0.3, GlassType::Annealed, true), &[0x3fd310be28c51b28, 0x3fd3543576da51f1, 0x3fce883d50c366f2]);
        same("see_rest", &run(0.37, 1.0, 0.3, GlassType::Annealed, true), &[0x3fda2a0d8523c0d0, 0x3fdf53b87958b172, 0x3fde196ed16ef2fc]);
        same("see_rest_clamped", &run(0.37, 1.0, 1.7, GlassType::Annealed, true), &[0x3fe7ae147ae147ae, 0x3fee147ae147ae14, 0x3feb851eb851eb84]);
        same("in_air", &run(0.37, 1.0, 0.3, GlassType::Annealed, false), &[0x3fd114940bbb1f26, 0x3fd2f85510d38cdb, 0x3fd36916440f2389]);
    }

    #[test]
    fn frames_match_the_typescripts_bits() {
        let l: [V3; 4] = [[1.1, 0.3, 0.2], [1.7, 0.31, 0.9], [1.5, 1.4, 1.2], [0.9, 1.2, 0.5]];
        let n = v3::normalize([0.6, 0.05, -0.5]);
        same("frame", &fr(frame_of(&l, [1.3, 0.8, 0.7], n)), &[0xbfd33c5b2b0d47f6, 0xbfeafe8bc6a19aa9, 0xbfdc7b5628304214, 0xbfe227e62af5811b, 0x3fe10ffc7db3f3de, 0xbfe414ae26fb0289, 0x3ff7e411eeccacc6, 0x3ff2df0cb2329006, 0x3ff5387c9477c607, 0x3fea544586790b03, 0x3fe77b83dd4a47e1]);
        same("frame_up", &fr(frame_of(&l, [1.3, 0.8, 0.7], [0.0, 1.0, 0.0])), &[0x3fe3b174f21e3086, 0x0000000000000000, 0x3fe938fa9c59df99, 0x3fe938fa9c59df99, 0x0000000000000000, 0xbfe3b174f21e3086, 0x3ff08cc7e4ffc64e, 0x3fd89a003877a555, 0x3ff4cccccccccccd, 0x3fe999999999999a, 0x3fe6666666666666]);
        same("frame_flat", &fr(frame_of(&[[1.0, 1.0, 1.0]], [1.0, 1.0, 1.0], [0.0, 0.0, 1.0])), &[0x3ff0000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x3ff0000000000000, 0x0000000000000000, 0x3fa999999999999a, 0x3fa999999999999a, 0x3ff0000000000000, 0x3ff0000000000000, 0x3ff0000000000000]);
    }

    #[test]
    fn polygons_match_the_typescripts_bits() {
        same("area", &[poly_area(&P)], &[0x4008343957709c78]);
        let mut r = P.to_vec();
        r.reverse();
        same("signed", &[signed_area(&r)], &[0xc008343957709c79]);
        let t = tidy_poly(&P);
        same("tidy", &flat(&t), &[0x3fb999999999999a, 0x3fc999999999999a, 0x3ff5eb851eb851ec, 0x3fcae147ae147ae1, 0x4002666666666666, 0x3ff199999999999a, 0x3ffe666666666666, 0x400599999999999a, 0x3fd999999999999a, 0x3ff3333333333333]);
        same("centroid", &poly_centroid(&t), &[0x3ff4a6ba68cebbd3, 0x3ff2f7348894a9e2]);
        same("centroid0", &poly_centroid(&[[1.0, 1.0], [2.0, 2.0], [3.0, 3.0]]), &[0x4000000000000000, 0x4000000000000000]);
        same("centroid_none", &poly_centroid(&[]), &[0x0000000000000000, 0x0000000000000000]);
        same("clip", &flat(&clip_half(&t, 0.8, 0.6, 1.3)), &[0x3fb999999999999a, 0x3fc999999999999a, 0x3ff5eb851eb851ec, 0x3fcae147ae147ae1, 0x3ff6d402e26f0db4, 0x3fd0ea9b485a6198, 0x3fe2be2be2be2be2, 0x3ff62be2be2be2be, 0x3fd999999999999a, 0x3ff3333333333333]);
        same("clip2", &flat(&clip_half(&t, -0.8, -0.6, -1.3)), &[0x3ff6d402e26f0db4, 0x3fd0ea9b485a6198, 0x4002666666666666, 0x3ff199999999999a, 0x3ffe666666666666, 0x400599999999999a, 0x3fe2be2be2be2be2, 0x3ff62be2be2be2be]);
        assert!(clip_half(&t, 1.0, 0.0, -5.0).is_empty());
    }
}
