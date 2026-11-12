use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// x held between lo and hi; lo wins a crossed pair.
pub(super) fn clamp_i(x: f64, lo: f64, hi: f64) -> f64 {
    m::max(lo, m::min(hi, x))
}

/// a plate's cut edge: the pane's tint taken k of the way to the edge green.
pub(super) fn edge_colour(tint: V3, k: f64) -> V3 {
    let q = m::min(1.0, k);
    let e = crate::shards::edge_green();
    [tint[0] + (e[0] - tint[0]) * q, tint[1] + (e[1] - tint[1]) * q, tint[2] + (e[2] - tint[2]) * q]
}

/// a CUT plate's face colour: dark body in the air, lifted toward the edge at rest, the floor through the tint when opaque at rest.
pub(super) fn cut_face_albedo(g: &crate::game::Game, tint: V3, under: V3, type_: crate::glasspen::PaneType, resting: bool) -> V3 {
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

/// a plate seen through: the surface under it × the tint, a little of the glass's grey-green; laminated is milky.
pub(super) fn plate_albedo(tint: V3, under: V3, type_: crate::glasspen::PaneType) -> V3 {
    let k = if type_ == crate::glasspen::PaneType::Laminated { 0.45 } else { 0.82 };
    [
        under[0] * tint[0] * k + tint[0] * (1.0 - k) * 0.6,
        under[1] * tint[1] * k + tint[1] * (1.0 - k) * 0.6,
        under[2] * tint[2] * k + tint[2] * (1.0 - k) * 0.6,
    ]
}

/// the principal in-plane frame of a polygon on a plane of normal n: u along its long side, v across, extents and the middle of its bounds.
pub(super) fn frame_of(outline: &[V3], centre: V3, n: V3) -> crate::shards::GlassShardsCutPlatesFr {
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

/// twice-signed-area ÷ 2, positive.
pub(super) fn poly_area(p: &[[f64; 2]]) -> f64 {
    let mut a = 0.0;
    for i in 0..p.len() {
        let q = p[(i + 1) % p.len()];
        a += p[i][0] * q[1] - q[0] * p[i][1];
    }
    m::abs(a) / 2.0
}

/// P without repeated points and without points on the straight line between their neighbours.
pub(super) fn tidy_poly(p: &[[f64; 2]]) -> Vec<[f64; 2]> {
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

/// a polygon's centre of area; the mean of its points when it has none.
pub(super) fn poly_centroid(p: &[[f64; 2]]) -> [f64; 2] {
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

/// the part of convex polygon P on the side n · p ≤ d; fewer than 3 points back = nothing.
pub(super) fn clip_half(p: &[[f64; 2]], nx: f64, ny: f64, d: f64) -> Vec<[f64; 2]> {
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

/// a cell as the outlines its plates take: itself when a cut record holds its edges, else its two halves across its long side.
pub(super) fn cell_parts(poly: &[[f64; 2]], half_t: f64, seed: f64) -> Vec<Vec<[f64; 2]>> {
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

/// the box of one cut plate: `poly` in the frame `rot` at `origin` extruded ± halfT; None = a degenerate outline.
pub(super) fn cut_plate_box(g: &mut crate::game::Game, origin: V3, rot: Quat, poly: &[[f64; 2]], half_t: f64, albedo: V3) -> Option<crate::shards::CutPlateBox> {
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
        flags: Some(crate::scene_boxes::box_flag::DYNAMIC as f64),
        ..Default::default()
    });
    let anti = signed_area(poly) >= 0.0;
    let off = pr.off;
    let at = |pt: &[f64; 2]| -> [f64; 2] { [pt[0] - off[0], pt[1] - off[1]] };
    let local: Vec<[f64; 2]> = if anti { poly.iter().map(at).collect() } else { poly.iter().rev().map(at).collect() };
    Some(crate::shards::CutPlateBox { box_, cut: pr.cut, poly: local })
}

/// a polygon's area with its winding's sign.
fn signed_area(p: &[[f64; 2]]) -> f64 {
    let mut a = 0.0;
    for i in 0..p.len() {
        let q = p[(i + 1) % p.len()];
        a += p[i][0] * q[1] - q[0] * p[i][1];
    }
    a / 2.0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::glasspen::PaneType;

    const TINT: V3 = [0.7213, 0.8011, 0.8197];
    const UNDER: V3 = [0.413, 0.377, 0.291];
    const BLACK: V3 = [0.0, 0.0, 0.0];
    fn near(a: V3, b: V3) { assert!((0..3).all(|i| (a[i] - b[i]).abs() < 1e-12), "{a:?} {b:?}"); }
    fn times(a: V3, b: V3) -> V3 { [a[0] * b[0], a[1] * b[1], a[2] * b[2]] }

    #[test]
    fn a_cut_edge_goes_from_the_tint_to_the_edge_green_and_no_further() {
        let green = crate::shards::edge_green();
        near(edge_colour(TINT, 0.0), TINT);
        near(edge_colour(TINT, 1.0), green);
        near(edge_colour(TINT, 1.25), green);
        assert_eq!(clamp_i(1.5, 0.0, 1.0), 1.0);
        assert_eq!(clamp_i(5.0, 2.0, 1.0), 2.0);    // lo wins a crossed pair
    }

    /// A failure means laminated glass goes as clear as plain, or a plate on the floor does not take the floor's colour.
    #[test]
    fn a_plate_shows_the_floor_through_its_tint_and_laminate_is_milkier() {
        let through = |ty: PaneType| { let (a, b) = (plate_albedo(TINT, UNDER, ty), plate_albedo(TINT, BLACK, ty)); [a[0] - b[0], a[1] - b[1], a[2] - b[2]] };
        near(through(PaneType::Annealed), v3::scale(times(UNDER, TINT), 0.82));
        near(through(PaneType::Laminated), v3::scale(times(UNDER, TINT), 0.45));
        near(plate_albedo(TINT, BLACK, PaneType::Annealed), v3::scale(TINT, 0.18 * 0.6));
    }

    #[test]
    fn a_cut_plate_is_dark_in_the_air_and_lifts_at_rest() {
        let mut g = Game::blank();
        let mut run = |body: f64, see: f64, rest: f64, ty: PaneType, resting: bool| {
            g.k.shards.cut_body = body;
            g.k.shards.cut_see = see;
            g.k.shards.cut_rest_body = rest;
            cut_face_albedo(&mut g, TINT, UNDER, ty, resting)
        };
        near(run(0.0, 1.0, 0.3, PaneType::Laminated, true), plate_albedo(TINT, UNDER, PaneType::Laminated));    // no body: a plain plate
        near(run(0.37, 1.0, 0.3, PaneType::Annealed, false), v3::scale(TINT, 0.37));                            // in the air: the dark body
        near(run(0.37, 0.0, 0.3, PaneType::Annealed, true), times(UNDER, TINT));                                // at rest, opaque: the floor through the tint
        near(run(0.37, 1.0, 1.7, PaneType::Annealed, true), crate::shards::edge_green());                       // at rest, seen into: lifted, no further than the edge
        let some = run(0.37, 1.0, 0.3, PaneType::Annealed, true);
        assert!(some[1] > 0.37 * TINT[1] && some[1] < crate::shards::edge_green()[1], "{some:?}");
    }

    /// A plate is laid along what it is long. A failure means a long dagger of glass gets a square box.
    #[test]
    fn a_frame_lies_along_the_long_side_of_its_outline() {
        let n: V3 = [0.0, 0.0, 1.0];
        let (cs, sn) = (m::cos(0.5), m::sin(0.5));
        let along = |a: f64, b: f64| -> V3 { [1.0 + cs * a - sn * b, 2.0 + sn * a + cs * b, 0.0] };
        let l = [along(-2.0, -0.5), along(2.0, -0.5), along(2.0, 0.5), along(-2.0, 0.5)];
        let f = frame_of(&l, [1.0, 2.0, 0.0], n);
        assert!((v3::dot(f.u, [cs, sn, 0.0]).abs() - 1.0).abs() < 1e-9, "{:?}", f.u);
        assert!((f.lu - 4.0).abs() < 1e-9 && (f.lv - 1.0).abs() < 1e-9, "{} {}", f.lu, f.lv);
        near(f.v, v3::cross(n, f.u));
        assert!((f.mid[0] - 1.0).abs() < 1e-9 && (f.mid[1] - 2.0).abs() < 1e-9, "{:?}", f.mid);
        // one point has no long side: a small square where it is
        let f = frame_of(&[[1.0, 1.0, 1.0]], [1.0, 1.0, 1.0], n);
        assert_eq!((f.lu, f.lv, f.mid), (0.05, 0.05, [1.0, 1.0, 1.0]));
    }

    const SQUARE: [[f64; 2]; 4] = [[0.0, 0.0], [2.0, 0.0], [2.0, 2.0], [0.0, 2.0]];

    #[test]
    fn an_outline_s_area_and_its_winding() {
        let mut back = SQUARE;
        back.reverse();
        assert_eq!((poly_area(&SQUARE), signed_area(&SQUARE)), (4.0, 4.0));
        assert_eq!((poly_area(&back), signed_area(&back)), (4.0, -4.0));
    }

    /// A cut record has only so many planes: a corner that is no corner must not take one.
    #[test]
    fn a_tidy_outline_has_no_repeated_points_and_no_points_on_a_straight_edge() {
        let messy = [[0.0, 0.0], [1.0, 0.0], [2.0, 0.0], [2.0, 0.0], [2.0, 2.0], [0.0, 2.0], [0.0, 0.0]];
        assert_eq!(tidy_poly(&messy), SQUARE.to_vec());
    }

    #[test]
    fn an_outline_s_centre_is_its_centre_of_area() {
        assert_eq!(poly_centroid(&SQUARE), [1.0, 1.0]);
        assert_eq!(poly_centroid(&[[1.0, 1.0], [2.0, 2.0], [3.0, 3.0]]), [2.0, 2.0]);    // a line has none: the mean of its points
        assert_eq!(poly_centroid(&[]), [0.0, 0.0]);
    }

    /// Both sides of a cut are what a broken plate's pieces are: nothing lost, nothing made.
    #[test]
    fn a_clip_keeps_one_side_of_the_line() {
        let (a, b) = (clip_half(&SQUARE, 0.8, 0.6, 1.3), clip_half(&SQUARE, -0.8, -0.6, -1.3));
        assert!(a.iter().all(|p| 0.8 * p[0] + 0.6 * p[1] <= 1.3 + 1e-12) && b.iter().all(|p| 0.8 * p[0] + 0.6 * p[1] >= 1.3 - 1e-12));
        assert!((poly_area(&a) + poly_area(&b) - 4.0).abs() < 1e-12);
        assert_eq!(clip_half(&SQUARE, 1.0, 0.0, 5.0), SQUARE.to_vec());
        assert!(clip_half(&SQUARE, 1.0, 0.0, -5.0).is_empty());
    }
}
