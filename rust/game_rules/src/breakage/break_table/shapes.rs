#![allow(unused_variables)]
use super::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, vec, vec::Vec};

/// INVENTED, for BREAK[mat]: the row of a material. Rows are read only by key, nothing writes them, and they do not borrow from g, so a row may be held across a call that takes g.
pub(super) fn break_row(mat: crate::break_table::Mat) -> &'static crate::break_table::BreakRow { &crate::break_table::break_().iter().find(|e| e.0 == mat).unwrap().1 }

type P2 = [f64; 2];

/// The row a Surf reads.
pub(super) fn mat_of_surf(s: crate::scene_materials::Surf) -> crate::break_table::Mat {
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

/// The row a pool chunk reads: by what the lane knows the thing is, else by what its box does.
pub(super) fn mat_of(g: &crate::game::Game, kind: crate::debris::ChunkKind, box_: &crate::break_table::MatOfBox) -> crate::break_table::Mat {
    use crate::debris::ChunkKind;
    use crate::scene_materials::Surf;
    if kind == ChunkKind::Shard { return Mat::Glass; }
    if kind == ChunkKind::Tile { return Mat::Tile; }
    if kind == ChunkKind::Housing { return Mat::Screen; }
    if kind == ChunkKind::Binder { return Mat::Plastic; }
    let s = crate::scene_materials::does_of_word(g, box_.does.as_deref()).surf;
    if kind == ChunkKind::Clutter { return if s == Surf::Foliage { Mat::Plant } else { Mat::Plastic }; }
    mat_of_surf(s)
}

/// WHAT A THING IS INSIDE IS WHAT IT SAYS IT IS MADE OF. The break table's row gives a piece its shape; what the piece is made of comes from the thing it broke off.
/// A body says what is inside it: mud is mud all through and stands by itself, block is grey, timber is pale, cloth is paler than its dye. A coat (plaster, paint,
/// enamel: the thing's stuff is only its skin) and a thing that says nothing leave it to what the thing does, which is the row `mat`'s core: paint on a wooden
/// locker is timber inside, skim on a partition is board.
pub(super) fn inside_of(stuff: Option<crate::scene_stuff::BoxStuff>, mat: crate::break_table::Mat) -> crate::break_table::Inside {
    use crate::scene_stuff::BoxStuff as S;
    let body = |core: Option<V3>, pale: Option<f64>, framed: bool| Inside { core, pale, facing: None, framed };
    match stuff {
        Some(S::MudWall | S::Earth | S::Gravel | S::Tarmac | S::Grass) => body(None, None, false),
        Some(S::Block | S::Slab | S::GlossBlock | S::BoardConcrete | S::CastPanel | S::Paver) => body(break_row(Mat::Blockwork).core, None, true),
        Some(S::Wood | S::Veneer) => body(break_row(Mat::Wood).core, None, true),
        Some(S::Metal | S::Steel | S::Sheet | S::Brushed | S::Louvre | S::Rubber | S::Carpet) => body(None, None, true),
        Some(S::Plastic) => body(None, break_row(Mat::Plastic).pale, true),
        Some(S::Cloth | S::Fabric) => body(None, break_row(Mat::Fabric).pale, true),
        _ => { let r = break_row(mat); Inside { core: r.core, pale: r.pale, facing: r.facing, framed: true } }
    }
}

/// What a broken edge shows.
pub(in crate::breakage) fn core_of(inside: crate::break_table::Inside, paint: V3) -> V3 {
    match inside.core {
        Some(c) => c,
        None => v3::lerp3(paint, [1.0, 1.0, 1.0], inside.pale.unwrap_or(0.0)),
    }
}

/// The colour a piece's cut faces show; None = the box's own.
pub(super) fn face_core(inside: crate::break_table::Inside, paint: V3) -> Option<V3> {
    if let Some(c) = inside.core { return Some(v3::copy(c)); }
    match inside.pale {
        Some(p) if js::truthy(p) => Some(v3::lerp3(paint, [1.0, 1.0, 1.0], p)),
        _ => None,
    }
}

/// A point of a range.
pub(super) fn lerp(r: [f64; 2], t: f64) -> f64 { r[0] + (r[1] - r[0]) * t }

/// SLAB: the rectangle with `tears` of its corners broken off, one of them the long tear.
fn slab_outline(l: f64, s: f64, tears: f64, r: &mut dyn FnMut() -> f64) -> Vec<[f64; 2]> {
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

/// WEDGE: the rectangle cut by one line near a diagonal, one half kept; `flip` keeps the other half.
fn wedge_outline(l: f64, s: f64, r: &mut dyn FnMut() -> f64, flip: Option<bool>) -> Vec<[f64; 2]> {
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

/// One broken end of a snapped piece.
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

/// SNAPPED END: a long piece whose +l end is broken to a point (the −l end too, one time in three).
fn snapped_outline(l: f64, s: f64, r: &mut dyn FnMut() -> f64) -> Vec<[f64; 2]> {
    let mut out = snapped_end(1.0, l, s, r);
    let mut lo: Vec<P2> = if r() < 0.33 { snapped_end(-1.0, l, s, r) } else { vec![[-l, -s], [-l, s]] };
    lo.reverse();
    out.extend_from_slice(&lo);   // (+l end bottom → top, then the −l end top → bottom: one loop)
    out
}

/// STRIP: a long splinter, both long sides closing toward the +l end.
fn strip_outline(l: f64, s: f64, r: &mut dyn FnMut() -> f64) -> Vec<[f64; 2]> {
    let most = 2.0 * l * m::tan(8.0 * RAD);
    let t0 = m::min(s * (0.55 + 0.4 * r()), most);
    let t1 = m::min(s * (0.55 + 0.4 * r()), most);
    vec![[-l, -s], [l, -s + t0], [l, s - t1], [-l, s]]
}

/// The outline of `shape` in the piece's own (l, s).
fn outline_of(shape: crate::break_table::Shape, l: f64, s: f64, row: &crate::break_table::BreakRow, r: &mut dyn FnMut() -> f64, flip: bool) -> Vec<[f64; 2]> {
    if shape == Shape::Wedge { return wedge_outline(l, s, r, Some(flip)); }
    if shape == Shape::Snapped { return snapped_outline(l, s, r); }
    if shape == Shape::Strip { return strip_outline(l, s, r); }
    if shape == Shape::Panel { return vec![[-l, -s], [l, -s], [l, s], [-l, s]]; }
    let tears = if shape == Shape::Lump { 3.0 + m::floor(r() * 2.0) } else { m::round(lerp(row.tears, r())) };
    slab_outline(l, s, tears, r)
}

/// `shape` as a fitted cut box, extruded across `thin`; a lump has 1‥3 corners broken as well. None = nothing left.
pub(super) fn shape_box(g: &mut crate::game::Game, shape: crate::break_table::Shape, thin: f64, half_t: f64, l: f64, s: f64, long_first: bool, row: &crate::break_table::BreakRow, r: &mut dyn FnMut(&mut crate::game::Game) -> f64, flip: Option<bool>) -> Option<crate::break_table::Shaped> {
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

/// Lays one plane on an open face if the face and the piece still have room and it bites at all.
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

/// RAGGED END: the planes for a standing piece's open faces. The record is not fitted: feet and the nav grid keep the box.
pub(super) fn ragged_cut(g: &mut crate::game::Game, row: &crate::break_table::BreakRow, h: V3, faces: &[crate::break_table::OpenFace]) -> Option<Vec<crate::scene_cut::CutPlane>> {
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

/// The horizontal unit normal of a wall-like box on the side `toward` points to, with `keep` of `toward` mixed back in.
pub(super) fn wall_normal(g: &crate::game::Game, b: BoxId, toward: V3, keep: Option<f64>) -> V3 {
    let keep = keep.unwrap_or(0.0);
    let (yaw, h, c) = seam_box_numbers(g, b);
    wall_normal_at(yaw, h, c, toward, keep)
}

/// A box's yaw, half sizes and centre. Once `g.col.boxes` is the `Tab<BoxId, Box>` the index says, the body is: `let x = &g.col.boxes[b]; (x.yaw, x.h, x.c)`.
fn seam_box_numbers(g: &crate::game::Game, b: BoxId) -> (f64, V3, V3) { let x = &g.tab.boxes[b]; (x.yaw, x.h, x.c) }

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
    struct Fold(u64);
    impl Fold {
        fn new() -> Self { Fold(0xcbf29ce484222325) }
        fn f(&mut self, x: f64) { assert!(x == x); self.0 = (self.0 ^ x.to_bits()).wrapping_mul(0x100000001b3); }
        fn pts(&mut self, o: &[P2]) { self.f(o.len() as f64); for p in o { self.f(p[0]); self.f(p[1]); } }
    }
    /// `BREAK[mat]`, off the skeleton's rows (the test below holds them to the TypeScript's bits)
    fn break_row(mat: Mat) -> &'static BreakRow { &break_().iter().find(|(k, _)| *k == mat).unwrap().1 }
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
        for i in 0..120 { h.pts(&outline_of(sh[i % 6], ln(i), sn(i), row, &mut stream(seed(i)), i % 3 == 0)); }
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
        OpenFace { axis, sign, lat: lat.to_vec(), seed, lo }
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

}
