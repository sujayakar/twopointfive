//! src/game/breakTable.ts lines 1 to 333
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

type P2 = [f64; 2];

fn row_of(core: Option<V3>, pale: Option<f64>, cell: &[Shape], thick: f64, span: P2, slim: P2, tears: P2, ragged: Option<BreakRowRagged>, again: Option<BreakRowAgain>, small: (f64, P2, P2), fines: [f64; 3], rest: f64, level: f64, mark: f64) -> BreakRow {
    BreakRow { core, pale, cell: cell.to_vec(), thick, span, slim, tears, ragged, again, small: BreakRowSmall { n: small.0, size: small.1, aspect: small.2 }, dust: fines[0], sparks: fines[1], puff: fines[2], rest, level, mark }
}
fn ragged_of(deg: P2, bite: f64, grain: Option<bool>) -> Option<BreakRowRagged> { Some(BreakRowRagged { deg, bite, grain }) }
fn again_of(land: f64, kick: f64, how: Split, chips: f64) -> Option<BreakRowAgain> { Some(BreakRowAgain { land, kick, how, chips, chance: None }) }

/// THE TABLE, one row a material: `BREAK[mat]`. Private until the skeleton gives `BREAK` its rows (notes, Skeleton).   (breakTable.ts BREAK)
fn break_row(mat: Mat) -> BreakRow {
    use Shape::{Lump, Panel, Slab, Snapped, Strip, Wedge};
    match mat {
        Mat::Plasterboard => row_of(Some([0.92, 0.91, 0.88]), None, &[Slab, Wedge], 0.015, [0.15, 0.7], [0.35, 0.8], [2.0, 3.0], ragged_of([8.0, 25.0], 0.12, None),
            again_of(6.5, 0.7, Split::Across, 4.0), (18.0, [0.01, 0.035], [1.0, 1.6]), [6.0, 0.0, 1.4], 0.1, 0.0, 0.03),
        Mat::Masonry => row_of(Some([0.6, 0.59, 0.57]), None, &[], 0.03, [0.1, 0.2], [0.6, 1.0], [2.0, 3.0], None,
            None, (16.0, [0.008, 0.02], [1.0, 1.0]), [5.0, 5.0, 1.0], 0.1, 0.0, 0.02),
        Mat::Blockwork => row_of(Some([0.6, 0.59, 0.57]), None, &[Lump, Slab], 0.05, [0.16, 0.4], [0.55, 0.95], [2.0, 3.0], None,
            None, (20.0, [0.01, 0.035], [1.0, 1.4]), [8.0, 3.0, 1.8], 0.08, 0.0, 0.03),
        Mat::Wood => row_of(Some([0.78, 0.63, 0.42]), None, &[Snapped, Strip], 0.011, [0.3, 0.6], [0.16, 0.33], [1.0, 2.0], ragged_of([8.0, 25.0], 0.12, Some(true)),
            again_of(NEVER, 0.85, Split::Along, 4.0), (18.0, [0.006, 0.014], [3.0, 6.0]), [3.0, 0.0, 0.0], 0.25, 0.0, 0.014),
        Mat::Metal => row_of(None, None, &[Panel], 0.004, [0.3, 0.48], [0.6, 1.0], [0.0, 0.0], None,
            None, (2.0, [0.006, 0.012], [1.0, 2.0]), [0.0, 16.0, 0.0], 0.35, 0.5, 0.0),
        Mat::Glass => row_of(None, None, &[], 0.003, [0.1, 0.3], [0.4, 1.0], [0.0, 0.0], None, None, (0.0, [0.01, 0.04], [1.0, 2.0]), [0.0, 0.0, 0.0], 0.22, 0.0, 0.0),
        Mat::GlassTough => row_of(None, None, &[], 0.003, [0.1, 0.3], [0.4, 1.0], [0.0, 0.0], None, None, (0.0, [0.005, 0.01], [1.0, 1.0]), [0.0, 0.0, 0.0], 0.22, 0.0, 0.0),
        Mat::GlassLam => row_of(None, None, &[], 0.003, [0.1, 0.3], [0.4, 1.0], [0.0, 0.0], None, None, (0.0, [0.005, 0.01], [1.0, 1.0]), [0.0, 0.0, 0.0], 0.22, 0.0, 0.0),
        Mat::Screen => row_of(Some([0.09, 0.09, 0.1]), None, &[Wedge, Wedge], 0.02, [0.15, 0.3], [0.5, 0.9], [1.0, 2.0], None,
            again_of(5.0, NEVER, Split::Diag, 5.0), (14.0, [0.006, 0.014], [1.0, 2.0]), [0.0, 6.0, 0.0], 0.22, 0.0, 0.0),
        Mat::Fabric => row_of(None, Some(0.35), &[Wedge, Wedge], 0.012, [0.3, 0.5], [0.6, 1.0], [1.0, 1.0], ragged_of([8.0, 15.0], 0.08, None),
            None, (5.0, [0.01, 0.02], [1.0, 1.0]), [3.0, 0.0, 0.0], 0.1, 0.0, 0.0),
        Mat::Paper => row_of(None, None, &[], 0.001, [0.2, 0.3], [0.7, 0.7], [0.0, 0.0], None, None, (0.0, [0.01, 0.02], [1.0, 1.0]), [0.0, 0.0, 0.0], 0.1, 0.0, 0.0),
        Mat::Cardboard => row_of(Some([0.66, 0.54, 0.38]), None, &[Slab, Slab], 0.015, [0.2, 0.4], [0.55, 1.0], [1.0, 1.0], ragged_of([8.0, 15.0], 0.12, None),
            None, (8.0, [0.008, 0.02], [1.0, 2.0]), [3.0, 0.0, 0.0], 0.1, 0.0, 0.0),
        Mat::Tile => row_of(Some([0.9, 0.89, 0.86]), None, &[], 0.0075, [0.3, 0.58], [0.6, 1.0], [1.0, 2.0], None,
            again_of(5.0, NEVER, Split::Across, 6.0), (6.0, [0.009, 0.022], [1.0, 1.6]), [4.0, 0.0, 1.0], 0.1, 0.0, 0.03),
        Mat::Plastic => row_of(None, Some(0.25), &[Wedge, Wedge], 0.01, [0.08, 0.2], [0.5, 1.0], [1.0, 2.0], None,
            again_of(5.0, NEVER, Split::Diag, 8.0), (8.0, [0.006, 0.014], [1.0, 2.0]), [0.0, 0.0, 0.0], 0.22, 0.0, 0.0),
        Mat::Carpet => row_of(None, None, &[], 0.005, [0.1, 0.2], [0.6, 1.0], [0.0, 0.0], None, None, (4.0, [0.008, 0.016], [1.0, 3.0]), [4.0, 0.0, 0.0], 0.1, 0.0, 0.0),
        Mat::Plant => row_of(Some([0.62, 0.36, 0.24]), None, &[Wedge, Wedge], 0.02, [0.1, 0.25], [0.6, 1.0], [1.0, 2.0], None,
            again_of(NEVER, 0.85, Split::Diag, 6.0), (16.0, [0.008, 0.018], [1.0, 2.5]), [2.0, 0.0, 0.0], 0.1, 0.0, 0.02),
    }
}

/// The row a Surf reads.   (breakTable.ts matOfSurf, SURF_MAT)
pub fn mat_of_surf(s: crate::scene_materials::Surf) -> crate::break_table::Mat {
    use crate::scene_materials::Surf;
    match s {
        Surf::Metal => Mat::Metal,
        Surf::Plastic => Mat::Plastic,
        Surf::Glass => Mat::Screen,
        Surf::Wood => Mat::Wood,
        Surf::Cardboard => Mat::Cardboard,
        Surf::Drywall => Mat::Plasterboard,
        Surf::Concrete => Mat::Masonry,
        Surf::Carpet => Mat::Carpet,
        Surf::Fabric => Mat::Fabric,
        Surf::Foliage => Mat::Plant,
    }
}

/// The row a pool chunk reads: by what the lane knows the thing is, else by what its box does.   (breakTable.ts matOf)
pub fn mat_of(g: &mut crate::game::Game, kind: crate::debris::ChunkKind, box_: &crate::break_table::MatOfBox) -> crate::break_table::Mat {
    use crate::debris::ChunkKind;
    use crate::scene_materials::Surf;
    if kind == ChunkKind::Shard { return Mat::Glass; }
    if kind == ChunkKind::Tile { return Mat::Tile; }
    if kind == ChunkKind::Housing { return Mat::Screen; }
    if kind == ChunkKind::Binder { return Mat::Plastic; }
    let s = crate::scene_materials::does_of(g, Some(&crate::scene_materials::DoesOfB { does: box_.does.clone() })).surf;
    if kind == ChunkKind::Clutter { return if s == Surf::Foliage { Mat::Plant } else { Mat::Plastic }; }
    mat_of_surf(s)
}

/// What a broken edge shows.   (breakTable.ts coreOf)
pub fn core_of(row: &crate::break_table::BreakRow, paint: V3) -> V3 {
    match row.core {
        Some(c) => c,
        None => v3::lerp3(paint, [1.0, 1.0, 1.0], row.pale.unwrap_or(0.0)),
    }
}

/// The colour a piece's cut faces show; None = the box's own.   (breakTable.ts faceCore)
pub fn face_core(row: &crate::break_table::BreakRow, paint: V3) -> Option<V3> {
    if let Some(c) = row.core { return Some(v3::copy(c)); }
    match row.pale {
        Some(p) if js::truthy(p) => Some(v3::lerp3(paint, [1.0, 1.0, 1.0], p)),
        _ => None,
    }
}

/// A point of a range.   (breakTable.ts lerp)
pub fn lerp(r: [f64; 2], t: f64) -> f64 { r[0] + (r[1] - r[0]) * t }

/// SLAB: the rectangle with `tears` of its corners broken off, one of them the long tear.   (breakTable.ts slabOutline)
pub fn slab_outline(l: f64, s: f64, tears: f64, r: &mut dyn FnMut() -> f64) -> Vec<[f64; 2]> {
    let corner: [P2; 4] = [[-l, -s], [l, -s], [l, s], [-l, s]];
    let mut pick = [0usize, 1, 2, 3];
    let mut i = 3usize;
    while i > 0 {
        let j = m::floor(r() * (i as f64 + 1.0)) as usize;
        let t = pick[i]; pick[i] = pick[j]; pick[j] = t;
        i -= 1;
    }
    let n_t = m::max(0.0, m::min(4.0, tears));
    let mut off = [false; 4];
    for &k in &pick[..n_t as usize] { off[k] = true; }
    let big = if js::truthy(n_t) { Some(pick[0]) } else { None };
    let mut a_ = [0.0f64; 4];
    let mut bv = [0.0f64; 4];
    for k in 0..4 {
        if !off[k] { continue; }
        let ang = (15.0 + 45.0 * r()) * RAD;
        let long = big == Some(k);
        let mut a = (if long { 0.5 + 0.35 * r() } else { 0.18 + 0.27 * r() }) * 2.0 * l;
        let mut b = a * m::tan(ang);
        let cap = (if long { 1.6 } else { 0.9 }) * s;
        if b > cap {
            b = cap;
            if !long { a = m::max(0.12 * 2.0 * l, m::min(0.9 * l, b / m::tan(ang))); }
        }
        a_[k] = a; bv[k] = b;
    }
    if let Some(big) = big {
        let (kl, ks) = (big ^ 1, 3 - big);   // the corner at the other end of its long edge, and of its short edge
        if off[kl] {
            let most = (2.0 * l - a_[big]) * 0.8;
            if most < 0.06 * 2.0 * l { off[kl] = false; } else if a_[kl] > most { bv[kl] *= most / a_[kl]; a_[kl] = most; }
        }
        if off[ks] {
            let most = (2.0 * s - bv[big]) * 0.8;
            if most < 0.06 * 2.0 * s { off[ks] = false; } else if bv[ks] > most { a_[ks] *= most / bv[ks]; bv[ks] = most; }
        }
    }
    let mut out: Vec<P2> = Vec::new();
    for k in 0..4 {
        let c = corner[k];
        if !off[k] { out.push(c); continue; }
        let pl: P2 = [c[0] - m::sign(c[0]) * a_[k], c[1]];
        let ps: P2 = [c[0], c[1] - m::sign(c[1]) * bv[k]];
        // keep the winding: at corners 0 and 2 the walk arrives along s and leaves along l
        if k == 0 || k == 2 { out.push(ps); out.push(pl); } else { out.push(pl); out.push(ps); }
    }
    out
}

/// WEDGE: the rectangle cut by one line near a diagonal, one half kept; `flip` keeps the other half.   (breakTable.ts wedgeOutline)
pub fn wedge_outline(l: f64, s: f64, r: &mut dyn FnMut() -> f64, flip: Option<bool>) -> Vec<[f64; 2]> {
    let flip = flip.unwrap_or(false);
    let which = if r() < 0.5 { 1.0 } else { -1.0 };
    let turn = (r() * 2.0 - 1.0) * 15.0 * RAD;
    let o = (r() - 0.5) * 0.3;
    let a = m::atan2(which * s, l) + turn;
    let (nx, ny) = (-m::sin(a), m::cos(a));   // the line runs along (cos a, sin a); (nx, ny) is its normal
    let d = (nx * l * o + ny * s * o) * (if flip { -1.0 } else { 1.0 });
    let (sx, sy) = (if flip { -nx } else { nx }, if flip { -ny } else { ny });
    let rect: [P2; 4] = [[-l, -s], [l, -s], [l, s], [-l, s]];
    let mut out: Vec<P2> = Vec::new();
    for k in 0..4 {   // the rectangle clipped to sx·u + sy·v ≤ d
        let (p, q) = (rect[k], rect[(k + 1) % 4]);
        let (fp, fq) = (sx * p[0] + sy * p[1] - d, sx * q[0] + sy * q[1] - d);
        if fp <= 0.0 { out.push(p); }
        if (fp < 0.0 && fq > 0.0) || (fp > 0.0 && fq < 0.0) {
            let t = fp / (fp - fq);
            out.push([p[0] + (q[0] - p[0]) * t, p[1] + (q[1] - p[1]) * t]);
        }
    }
    out
}

/// One broken end of a snapped piece.   (breakTable.ts snappedOutline's `end`)
fn snapped_end(sign: f64, l: f64, s: f64, r: &mut dyn FnMut() -> f64) -> Vec<P2> {
    let a1 = (20.0 + 20.0 * r()) * RAD;
    let two = r() < 0.55;
    if !two {
        let run = m::min(2.0 * s * m::tan(a1), 0.8 * l);
        let up = r() < 0.5;
        return if up { vec![[sign * (l - run), -s], [sign * l, s]] } else { vec![[sign * l, -s], [sign * (l - run), s]] };
    }
    let s0 = (r() - 0.5) * s;
    let a2 = (20.0 + 20.0 * r()) * RAD;
    vec![[sign * (l - m::min((s0 + s) * m::tan(a1), 0.8 * l)), -s], [sign * l, s0], [sign * (l - m::min((s - s0) * m::tan(a2), 0.8 * l)), s]]
}

/// SNAPPED END: a long piece whose +l end is broken to a point (the −l end too, one time in three).   (breakTable.ts snappedOutline)
pub fn snapped_outline(l: f64, s: f64, r: &mut dyn FnMut() -> f64) -> Vec<[f64; 2]> {
    let mut out = snapped_end(1.0, l, s, r);
    let mut lo: Vec<P2> = if r() < 0.33 { snapped_end(-1.0, l, s, r) } else { vec![[-l, -s], [-l, s]] };
    lo.reverse();
    out.extend_from_slice(&lo);   // (+l end bottom → top, then the −l end top → bottom: one loop)
    out
}

/// STRIP: a long splinter, both long sides closing toward the +l end.   (breakTable.ts stripOutline)
pub fn strip_outline(l: f64, s: f64, r: &mut dyn FnMut() -> f64) -> Vec<[f64; 2]> {
    let most = 2.0 * l * m::tan(8.0 * RAD);
    let t0 = m::min(s * (0.55 + 0.4 * r()), most);
    let t1 = m::min(s * (0.55 + 0.4 * r()), most);
    vec![[-l, -s], [l, -s + t0], [l, s - t1], [-l, s]]
}

/// The outline of `shape` in the piece's own (l, s).   (breakTable.ts outlineOf)
pub fn outline_of(shape: crate::break_table::Shape, l: f64, s: f64, row: &crate::break_table::BreakRow, r: &mut dyn FnMut() -> f64, flip: bool) -> Vec<[f64; 2]> {
    if shape == Shape::Wedge { return wedge_outline(l, s, r, Some(flip)); }
    if shape == Shape::Snapped { return snapped_outline(l, s, r); }
    if shape == Shape::Strip { return strip_outline(l, s, r); }
    if shape == Shape::Panel { return vec![[-l, -s], [l, -s], [l, s], [-l, s]]; }
    let tears = if shape == Shape::Lump { 3.0 + m::floor(r() * 2.0) } else { m::round(lerp(row.tears, r())) };
    slab_outline(l, s, tears, r)
}

/// `shape` as a fitted cut box, extruded across `thin`; a lump has 1‥3 corners broken as well. None = nothing left.   (breakTable.ts shapeBox)
pub fn shape_box(g: &mut crate::game::Game, shape: crate::break_table::Shape, thin: f64, half_t: f64, l: f64, s: f64, long_first: bool, row: &crate::break_table::BreakRow, r: &mut dyn FnMut(&mut crate::game::Game) -> f64, flip: Option<bool>) -> Option<crate::break_table::Shaped> {
    let flip = flip.unwrap_or(false);
    let o = {
        let mut r0 = || r(&mut *g);
        outline_of(shape, l, s, row, &mut r0, flip)
    };
    let poly: Vec<P2> = o.iter().map(|p| if long_first { [p[0], p[1]] } else { [p[1], p[0]] }).collect();
    let pr = crate::scene_cut::cut_prism(thin, half_t, &poly);
    if pr.empty { return None; }
    g.stats.break_table.dropped += pr.dropped as i32;
    if shape != Shape::Lump { return Some(Shaped { off: pr.off, h: pr.h, cut: pr.cut }); }
    let thin = thin as usize;
    let (ua, va) = ((thin + 1) % 3, (thin + 2) % 3);
    let mut extra: Vec<crate::scene_cut::CutPlane> = Vec::new();
    // The first plane of every corner goes in before any second: the plane limit takes facets, never a whole corner.
    let mut more: Vec<crate::scene_cut::CutPlane> = Vec::new();
    let n = 1 + m::floor(r(g) * 3.0) as usize;
    for _k in 0..n {
        let su = if r(g) < 0.5 { -1.0 } else { 1.0 };
        let sv = if r(g) < 0.5 { -1.0 } else { 1.0 };
        let st = if r(g) < 0.5 { -1.0 } else { 1.0 };
        let deep = 0.15 + 0.2 * r(g);
        let mut n3: V3 = [0.0, 0.0, 0.0];
        n3[ua] = su / pr.h[ua]; n3[va] = sv / pr.h[va]; n3[thin] = st / pr.h[thin];
        let mut at: V3 = [0.0, 0.0, 0.0];
        at[ua] = su * pr.h[ua] * (1.0 - deep); at[va] = sv * pr.h[va] * (1.0 - deep); at[thin] = st * pr.h[thin] * (1.0 - deep);
        extra.push(crate::scene_cut::cut_through(at, n3));
        let facets = 1 + m::floor(r(g) * 2.0) as usize;
        let first = m::floor(r(g) * 3.0) as usize;
        for j in 0..facets {
            let lean = [ua, va, thin][(first + j) % 3];
            let mut m3: V3 = [n3[0] * 0.45, n3[1] * 0.45, n3[2] * 0.45];
            m3[lean] = n3[lean] * (1.5 + 0.9 * r(g));
            let back = 0.04 + 0.1 * r(g);
            let mut p3: V3 = [at[0], at[1], at[2]];
            p3[lean] += m::sign(n3[lean]) * pr.h[lean] * deep * back * 2.0;
            more.push(crate::scene_cut::cut_through(p3, m3));
        }
    }
    let had = pr.cut.as_ref().map_or(0, |c| c.len());
    let room = m::max(0.0, crate::scene_cut::MAX_CUT_PLANES - had as f64 - extra.len() as f64) as usize;
    extra.extend_from_slice(&more[..room.min(more.len())]);
    let mut all: Vec<crate::scene_cut::CutPlane> = pr.cut.clone().unwrap_or_default();
    all.extend_from_slice(&extra);
    let rr = crate::scene_cut::cut_record(pr.h, &all);
    if rr.empty { return Some(Shaped { off: pr.off, h: pr.h, cut: pr.cut }); }
    g.stats.break_table.dropped += rr.dropped as i32;
    let f = crate::scene_cut::cut_fit(pr.h, rr.cut.as_deref());
    Some(Shaped { off: [pr.off[0] + f.off[0], pr.off[1] + f.off[1], pr.off[2] + f.off[2]], h: f.h, cut: f.cut })
}

/// Lays one plane on an open face if the face and the piece still have room and it bites at all.   (breakTable.ts raggedCut's `push`)
fn ragged_push(planes: &mut Vec<crate::scene_cut::CutPlane>, mine: &mut f64, per_face: f64, most: f64, f: &OpenFace, h: V3, l: usize, side: f64, th: f64, bite: f64) {
    if *mine >= per_face || planes.len() as f64 >= most || !(bite > 0.004) { return; }
    let axis = f.axis as usize;
    let mut n: V3 = [0.0, 0.0, 0.0];
    n[axis] = f.sign * m::cos(th);
    n[l] = side * m::sin(th);
    planes.push(crate::scene_cut::CutPlane { n, d: m::cos(th) * (h[axis] - bite) + m::sin(th) * h[l] });
    *mine += 1.0;
}

/// The planes of `raggedCut` before the record is made; `most` is BREAKS.maxStandingPlanes, `across` BREAKS.acrossDeg, `hash` the module's hash01.
fn ragged_planes(rag: &BreakRowRagged, h: V3, faces: &[OpenFace], most: f64, across: [f64; 2], hash: &dyn Fn(f64) -> f64) -> Vec<crate::scene_cut::CutPlane> {
    let mut planes: Vec<crate::scene_cut::CutPlane> = Vec::new();
    let per_face = m::max(1.0, m::floor(most / faces.len() as f64));
    for f in faces {
        if planes.len() as f64 >= most { break; }
        let s = |j: f64| hash(f.seed * 31.0 + j);
        let mut mine = 0.0;
        let full = rag.bite * (0.55 + 0.45 * s(3.0));   // the deepest this end is bitten, at its far corner
        if rag.grain == Some(true) {   // along the grain: one plane with the piece's long side, as a board splits
            let l = match f.lo {
                Some(_) => 1usize,
                None => {
                    let mut lat = f.lat.clone();
                    js::sort_by(&mut lat, |a, b| h[*b as usize] - h[*a as usize]);
                    lat[0] as usize
                }
            };
            let th = lerp(rag.deg, s(1.0)) * RAD;
            let room = match f.lo { Some(lo) => h[l] - lo, None => 2.0 * h[l] };
            let side = if f.lo.is_some() { 1.0 } else if s(2.0) < 0.5 { -1.0 } else { 1.0 };
            ragged_push(&mut planes, &mut mine, per_face, most, f, h, l, side, th, m::min(full, room * m::tan(th) * 0.9));
            continue;
        }
        // (1) the lean of the whole open end; (2) a steeper break at that corner, or the opposite corner knocked off; (3) the end turned across the piece, dealt second
        let l = f.lat[0] as usize;
        let room = match f.lo { Some(lo) => h[l] - lo, None => 2.0 * h[l] };
        let side = if f.lo.is_some() { 1.0 } else if s(2.0) < (if l == 1 { 0.7 } else { 0.5 }) { 1.0 } else { -1.0 };
        let cover = 0.75 + 0.2 * s(5.0);
        let b1 = full * (0.45 + 0.3 * s(4.0));
        let th1 = m::atan(b1 / m::max(0.05, room * cover));
        ragged_push(&mut planes, &mut mine, per_face, most, f, h, l, side, th1, b1);
        let o = f.lat.iter().map(|a| *a as usize).find(|a| *a != l);
        if let (Some(o), None) = (o, f.lo) {
            let th3 = lerp(across, s(7.0)) * RAD;
            let side3 = if s(8.0) < 0.5 { -1.0 } else { 1.0 };
            ragged_push(&mut planes, &mut mine, per_face, most, f, h, o, side3, th3, m::min(full, 2.0 * h[o] * m::tan(th3) * 0.9));
        }
        let th2 = lerp(rag.deg, s(1.0)) * RAD;   // (the opposite corner stops short of the lean)
        if f.lo.is_none() && s(6.0) < 0.5 {
            ragged_push(&mut planes, &mut mine, per_face, most, f, h, l, -side, th2, m::min(full * 0.8, room * (1.0 - cover) * 0.8 * m::tan(th2)));
        } else {
            ragged_push(&mut planes, &mut mine, per_face, most, f, h, l, side, th2, m::min(full, room * m::tan(th2) * 0.9));
        }
    }
    planes
}

/// RAGGED END: the planes for a standing piece's open faces. The record is not fitted: feet and the nav grid keep the box.   (breakTable.ts raggedCut)
pub fn ragged_cut(g: &mut crate::game::Game, row: &crate::break_table::BreakRow, h: V3, faces: &[crate::break_table::OpenFace]) -> Option<Vec<crate::scene_cut::CutPlane>> {
    let rag = row.ragged?;
    if faces.is_empty() { return None; }
    let (most, across) = (g.k.breaks.max_standing_planes, g.k.breaks.across_deg);
    let planes = ragged_planes(&rag, h, faces, most, across, &crate::debris::hash01);
    if planes.is_empty() { return None; }
    let rr = crate::scene_cut::cut_record(h, &planes);
    if rr.empty { return None; }
    rr.cut
}

/// `wallNormal` on the box's numbers.
fn wall_normal_at(yaw: f64, h: V3, c: V3, toward: V3, keep: f64) -> V3 {
    let (cs, sn) = (m::cos(yaw), m::sin(yaw));
    let n: V3 = if h[0] <= h[2] { [cs, 0.0, -sn] } else { [sn, 0.0, cs] };
    let raw = n[0] * toward[0] + n[2] * toward[2];   // (the side by the raw distance across the wall)
    let len = m::hyp2(toward[0], toward[2]);
    if len < 1e-6 { return [n[0], 0.0, n[2]]; }
    let t: V3 = [toward[0] / len, 0.0, toward[2] / len];
    // (in the wall's own plane no side is away: the cells take sides by where they are)
    let s = if m::abs(raw) < 0.01 && m::abs(raw) < 0.08 * len {
        if js::to_i32(m::floor((c[0] + c[2]) * 4.0 + c[1] * 2.0)) & 1 != 0 { 1.0 } else { -1.0 }
    } else if raw < 0.0 { -1.0 } else { 1.0 };
    v3::normalize([n[0] * s + t[0] * keep, 0.0, n[2] * s + t[2] * keep])
}

/// The horizontal unit normal of a wall-like box on the side `toward` points to, with `keep` of `toward` mixed back in.   (breakTable.ts wallNormal)
pub fn wall_normal(g: &mut crate::game::Game, b: BoxId, toward: V3, keep: Option<f64>) -> V3 {
    let keep = keep.unwrap_or(0.0);
    let bx = &g.col.boxes[b];
    wall_normal_at(bx.yaw, bx.h, bx.c, toward, keep)
}

#[cfg(test)]
mod tests {
    use super::*;

    // FNV of the doubles' bits, from the TypeScript under EXACT (the outlines and wallNormal as exported or pasted; raggedCut's body pasted, up to its cutRecord; BREAK itself).
    const SLAB: u64 = 0xba975fe2df4b41d3;
    const WEDGE: u64 = 0x6dcd936e502ad9f8;
    const SNAPPED: u64 = 0xbcb4561af1ee2bea;
    const STRIP: u64 = 0xc704568e5d7fcc04;
    const OUTLINE: u64 = 0x0271d81e5c36523c;
    const WALL: u64 = 0x49add9f7ce54aa50;
    const RAGGED: u64 = 0xd985aa2a8a03a242;
    const TABLE: u64 = 0xeb464685076bdf0d;

    struct Fold(u64);
    impl Fold {
        fn new() -> Self { Fold(0xcbf29ce484222325) }
        fn f(&mut self, x: f64) { assert!(x == x); self.0 = (self.0 ^ x.to_bits()).wrapping_mul(0x100000001b3); }
        fn w(&mut self, s: &str) { self.f(s.len() as f64); for b in s.bytes() { self.f(b as f64); } }
        fn pts(&mut self, o: &[P2]) { self.f(o.len() as f64); for p in o { self.f(p[0]); self.f(p[1]); } }
    }
    fn stream(seed: f64) -> impl FnMut() -> f64 { let mut x = seed; move || { x = js::rem(x + 0.6180339887498949, 1.0); x } }
    fn ln(i: usize) -> f64 { 0.1 + 0.03 * i as f64 }
    fn sn(i: usize) -> f64 { 0.05 + 0.011 * i as f64 }
    fn seed(i: usize) -> f64 { i as f64 * 0.037 }
    /// debris.ts hash01 (the skeleton's is a stub here)
    fn hash(x: f64) -> f64 {
        let mut v = (js::to_i32(x) ^ (0x9E3779B9u32 as i32)) as u32;
        v = v.wrapping_mul(0x85EBCA6B);
        v ^= v >> 13;
        v = v.wrapping_mul(0xC2B2AE35);
        v ^= v >> 16;
        (v >> 8) as f64 / 16777216.0
    }

    #[test]
    fn outlines_match_the_typescript_bits() {
        let mut h = Fold::new();
        for i in 0..120 { h.pts(&slab_outline(ln(i), sn(i), (i % 6) as f64, &mut stream(seed(i)))); }
        assert_eq!(h.0, SLAB);
        let mut h = Fold::new();
        for i in 0..120 { h.pts(&wedge_outline(ln(i), sn(i), &mut stream(seed(i)), Some(i % 2 == 1))); }
        assert_eq!(h.0, WEDGE);
        let mut h = Fold::new();
        for i in 0..120 { h.pts(&snapped_outline(ln(i), sn(i), &mut stream(seed(i)))); }
        assert_eq!(h.0, SNAPPED);
        let mut h = Fold::new();
        for i in 0..120 { h.pts(&strip_outline(ln(i), sn(i), &mut stream(seed(i)))); }
        assert_eq!(h.0, STRIP);
        let sh = [Shape::Slab, Shape::Lump, Shape::Panel, Shape::Wedge, Shape::Snapped, Shape::Strip];
        let row = break_row(Mat::Plasterboard);
        let mut h = Fold::new();
        for i in 0..120 { h.pts(&outline_of(sh[i % 6], ln(i), sn(i), &row, &mut stream(seed(i)), i % 3 == 0)); }
        assert_eq!(h.0, OUTLINE);
    }

    #[test]
    fn wall_normal_matches_the_typescript_bits() {
        let mut f = Fold::new();
        for i in 0..80usize {
            let x = i as f64;
            let yaw = x * 0.31 - 6.0;
            let h: V3 = if i % 2 == 1 { [0.1, 1.0, 2.0] } else { [2.0, 1.0, 0.1] };
            let c: V3 = [x * 0.13, 0.7 + x * 0.01, -x * 0.21];
            let (cs, sn) = (m::cos(yaw), m::sin(yaw));
            let toward: V3 = if i % 5 == 0 { [0.0, 0.0, 0.0] }
                else if i % 5 == 1 { if h[0] <= h[2] { [2.0 * sn, 0.0, 2.0 * cs] } else { [2.0 * cs, 0.0, -2.0 * sn] } }
                else { [(i % 7) as f64 - 3.0, 0.0, (i % 11) as f64 - 5.0] };
            let n = wall_normal_at(yaw, h, c, toward, if i % 3 == 0 { 0.3 } else { 0.0 });
            f.f(n[0]); f.f(n[1]); f.f(n[2]);
        }
        assert_eq!(f.0, WALL);
    }

    fn face(axis: f64, sign: f64, lat: &[f64], seed: f64, lo: Option<f64>) -> OpenFace {
        OpenFace { axis, sign, lat: lat.to_vec(), seed, lo, head: None }
    }

    #[test]
    fn ragged_planes_match_the_typescript_bits() {
        let mut f = Fold::new();
        for mat in [Mat::Plasterboard, Mat::Wood, Mat::Fabric, Mat::Cardboard] {
            let rag = break_row(mat).ragged.unwrap();
            for i in 0..100usize {
                let x = i as f64;
                let h: V3 = [0.1 + x * 0.01, 1.2, 0.5 + x * 0.02];
                let sd = x * 7349.0 + 131.0;
                let faces = match i % 4 {
                    0 => vec![face(0.0, 1.0, &[1.0, 2.0], sd, None)],
                    1 => vec![face(0.0, -1.0, &[1.0, 2.0], sd, None), face(2.0, 1.0, &[1.0, 0.0], sd + 17.0, None)],
                    2 => vec![face(0.0, 1.0, &[1.0], sd, Some(-0.3 + x * 0.005))],
                    _ => vec![face(1.0, 1.0, &[2.0, 0.0], sd, None), face(0.0, 1.0, &[1.0, 2.0], sd + 5.0, None), face(2.0, -1.0, &[1.0, 0.0], sd + 9.0, None)],
                };
                let p = ragged_planes(&rag, h, &faces, 4.0, [35.0, 55.0], &hash);
                f.f(p.len() as f64);
                for q in &p { f.f(q.n[0]); f.f(q.n[1]); f.f(q.n[2]); f.f(q.d); }
            }
        }
        assert_eq!(f.0, RAGGED);
    }

    #[test]
    fn the_table_matches_the_typescript_bits() {
        let mut f = Fold::new();
        for mat in [Mat::Plasterboard, Mat::Masonry, Mat::Blockwork, Mat::Wood, Mat::Metal, Mat::Glass, Mat::GlassTough, Mat::GlassLam, Mat::Screen, Mat::Fabric, Mat::Paper, Mat::Cardboard, Mat::Tile, Mat::Plastic, Mat::Carpet, Mat::Plant] {
            let r = break_row(mat);
            f.w(mat.as_str());
            match r.core { Some(c) => { f.f(c[0]); f.f(c[1]); f.f(c[2]); } None => f.f(-1.0) }
            f.f(r.pale.unwrap_or(-1.0));
            f.f(r.cell.len() as f64);
            for s in &r.cell { f.w(s.as_str()); }
            for x in [r.thick, r.span[0], r.span[1], r.slim[0], r.slim[1], r.tears[0], r.tears[1]] { f.f(x); }
            match r.ragged { Some(g) => { f.f(g.deg[0]); f.f(g.deg[1]); f.f(g.bite); f.f(if g.grain == Some(true) { 1.0 } else { 0.0 }); } None => f.f(-1.0) }
            match r.again { Some(a) => { f.f(a.land); f.f(a.kick); f.w(a.how.as_str()); f.f(a.chips); f.f(a.chance.unwrap_or(-1.0)); } None => f.f(-1.0) }
            for x in [r.small.n, r.small.size[0], r.small.size[1], r.small.aspect[0], r.small.aspect[1], r.dust, r.sparks, r.puff, r.rest, r.level, r.mark] { f.f(x); }
        }
        assert_eq!(f.0, TABLE);
    }
}
