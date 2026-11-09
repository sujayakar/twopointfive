//! WHAT A STAND-UP DOES TO A LEVEL'S BOXES before they go to the world (`src/host/standup.ts populateHost`), in its order: the thin things that
//! stand free marked (`src/scene/thin.ts`), a window's bars marked
//! (`src/scene/bars.ts`), and each box that says a shape given its record and its pieces of trim (`src/scene/shapes.ts`). Line for line from the
//! TypeScript. A level is stood ONCE here, on boxes fresh from its file, so the passes' "put back what an earlier stand-up gave" halves
//! (plainLevel, thinClear, barClear) have nothing to do and are not written.
use alloc::vec::Vec;
use game::game::Game;
use game::ids::*;
use game::m;
use game::scene_boxes::{box_flag, Box};
use game::scene_cut::{cut_plane, cut_record, cut_to_world, CutPlane};
use game::scene_level::LevelDoorDef;
use game::scene_shapes::ShapeFormArc;
use game::V3;

fn has(b: &Box, bits: f64) -> bool { (b.flags as u32) & (bits as u32) != 0 }
fn set(b: &mut Box, bit: f64, on: bool) { let f = b.flags as u32; b.flags = (if on { f | bit as u32 } else { f & !(bit as u32) }) as f64; }
fn cut(b: &Box) -> bool { b.cut.as_ref().map_or(false, |c| !c.is_empty()) }

// ---- thin.ts
pub const THIN_WIDTH: f64 = 0.25;
const BESIDE: f64 = 0.03;
const SHARE: f64 = 0.25;
pub const DOOR_NEAR: f64 = 0.3;

fn at_door(b: &Box, doors: &[LevelDoorDef]) -> bool {
    if b.c[1] - b.h[1] > 2.2 { return false; }   // (over the head of every leaf)
    for d in doors {
        let (ux, uz, px, pz) = (m::cos(d.closed_dir), m::sin(d.closed_dir), b.c[0] - d.hinge[0], b.c[2] - d.hinge[1]);
        let t = m::max(-0.1, m::min(d.width + 0.1, px * ux + pz * uz));
        if m::hyp2(px - ux * t, pz - uz * t) < DOOR_NEAR { return true; }
    }
    false
}
fn solid(b: &Box) -> bool { let f = box_flag(); !has(b, f.no_shadow + f.glass) }
fn narrow(b: &Box, width: f64) -> bool { b.rot.is_none() && !cut(b) && 2.0 * m::max(b.h[0], b.h[2]) < width }
/// a's footprint grown by `pad`, against b's: the two yawed rectangles overlap in plan
fn plans_meet(a: &Box, b: &Box, pad: f64) -> bool {
    let (dx, dz) = (b.c[0] - a.c[0], b.c[2] - a.c[2]);
    let (ca, sa, cb, sb) = (m::cos(a.yaw), m::sin(a.yaw), m::cos(b.yaw), m::sin(b.yaw));
    let (ahx, ahz, bhx, bhz) = (a.h[0] + pad, a.h[2] + pad, b.h[0], b.h[2]);
    let (ac, as_) = (m::abs(ca * cb + sa * sb), m::abs(sa * cb - ca * sb));
    if m::abs(dx * ca - dz * sa) > ahx + bhx * ac + bhz * as_ { return false; }
    if m::abs(dx * sa + dz * ca) > ahz + bhx * as_ + bhz * ac { return false; }
    if m::abs(dx * cb - dz * sb) > bhx + ahx * ac + ahz * as_ { return false; }
    if m::abs(dx * sb + dz * cb) > bhz + ahx * as_ + ahz * ac { return false; }
    true
}
/// b's centre is inside a's footprint (grown by `pad`): two slabs of one round thing, not two things side by side
fn centre_in(a: &Box, b: &Box, pad: f64) -> bool {
    let (dx, dz, c, s) = (b.c[0] - a.c[0], b.c[2] - a.c[2], m::cos(a.yaw), m::sin(a.yaw));
    m::abs(c * dx - s * dz) <= a.h[0] + pad && m::abs(s * dx + c * dz) <= a.h[2] + pad
}
fn stands_free(ai: usize, boxes: &[Box], width: f64) -> bool {
    let a = &boxes[ai];
    if !solid(a) || !narrow(a, width) { return false; }
    let (a0, a1) = (a.c[1] - a.h[1], a.c[1] + a.h[1]);
    let (need, ra) = (m::max(0.02, SHARE * (a1 - a0)), m::hyp2(a.h[0], a.h[2]) + BESIDE);
    let mut beside = 0.0;
    for (bi, b) in boxes.iter().enumerate() {
        if bi == ai || !solid(b) { continue; }
        let over = m::min(a1, b.c[1] + b.h[1]) - m::max(a0, b.c[1] - b.h[1]);
        if over <= 0.0 { continue; }   // (above it or under it)
        let (dx, dz) = (b.c[0] - a.c[0], b.c[2] - a.c[2]);
        let reach = ra + if b.rot.is_some() { m::hyp3(b.h[0], b.h[1], b.h[2]) } else { m::hyp2(b.h[0], b.h[2]) };
        if dx * dx + dz * dz > reach * reach { continue; }
        if b.rot.is_none() {
            if !plans_meet(a, b, BESIDE) { continue; }
            if narrow(b, width) && centre_in(a, b, 0.0) { continue; }
        }
        beside += over;
        if beside > need { return false; }
    }
    true
}
/// A LEVEL'S THIN THINGS (thin.ts thinLevel): every static's mark off, then on the ones that stand free. Answers how many.
pub fn thin_level(g: &mut Game, ids: &[BoxId], doors: &[LevelDoorDef]) -> usize {
    let f = box_flag();
    for &b in ids { set(&mut g.tab.boxes[b], f.thin, false); }
    let boxes: Vec<Box> = ids.iter().map(|&b| g.tab.boxes[b].clone()).collect();
    let free: Vec<usize> = (0..boxes.len()).filter(|&i| solid(&boxes[i]) && narrow(&boxes[i], THIN_WIDTH) && !at_door(&boxes[i], doors) && stands_free(i, &boxes, THIN_WIDTH)).collect();
    for &i in free.iter() { set(&mut g.tab.boxes[ids[i]], f.thin, true); }
    free.len()
}

// ---- bars.ts
const BAR_ACROSS: f64 = 0.1;
const BAR_DEEP: f64 = 0.25;
const BAR_TALL: f64 = 0.4;
const SILL_MIN: f64 = 0.2;
const KIT_DEEP: [f64; 2] = [0.72, 0.88];
const TOUCH: f64 = 0.02;

fn plain(b: &Box) -> bool { let f = box_flag(); !has(b, f.glass + f.no_shadow + f.no_primary + f.dynamic) && b.rot.is_none() && b.round != Some(true) && !cut(b) }
fn slim(b: &Box) -> bool {
    plain(b) && b.emissive[0] + b.emissive[1] + b.emissive[2] <= 0.0 && 2.0 * m::min(b.h[0], b.h[2]) <= BAR_ACROSS && 2.0 * m::max(b.h[0], b.h[2]) <= BAR_DEEP && 2.0 * b.h[1] >= BAR_TALL && b.h[1] > m::max(b.h[0], b.h[2])
}
fn local(b: &Box, x: f64, z: f64) -> [f64; 2] { let (dx, dz, c, s) = (x - b.c[0], z - b.c[2], m::cos(b.yaw), m::sin(b.yaw)); [c * dx - s * dz, s * dx + c * dz] }
fn wall_deep(bar: &Box, w: &Box, deep_x: bool) -> f64 {
    let [lx, lz] = local(w, bar.c[0], bar.c[2]);
    if m::abs(lx) > w.h[0] + TOUCH || m::abs(lz) > w.h[2] + TOUCH { return 0.0; }
    let turn = m::abs(m::cos(w.yaw - bar.yaw));
    let (along, cross) = (turn > 0.99, turn < 0.01);
    if !along && !cross { return 0.0; }
    2.0 * if deep_x == along { w.h[0] } else { w.h[2] }
}
fn in_opening(ai: usize, boxes: &[Box]) -> bool {
    let a = &boxes[ai];
    let (y0, y1, deep) = (a.c[1] - a.h[1], a.c[1] + a.h[1], 2.0 * m::max(a.h[0], a.h[2]));
    if y0 < SILL_MIN { return false; }
    let ways: &[bool] = if m::abs(a.h[0] - a.h[2]) < 1e-6 { &[true, false] } else if a.h[0] > a.h[2] { &[true] } else { &[false] };
    for &deep_x in ways {
        let (mut sill, mut head) = (false, false);
        for (bi, b) in boxes.iter().enumerate() {
            if bi == ai || !plain(b) { continue; }
            let (foot, top) = (m::abs(b.c[1] + b.h[1] - y0) <= TOUCH, m::abs(b.c[1] - b.h[1] - y1) <= TOUCH);
            if !foot && !top { continue; }
            let t = wall_deep(a, b, deep_x);
            if t <= 0.0 || deep < KIT_DEEP[0] * t || deep > KIT_DEEP[1] * t { continue; }
            if foot { sill = true; } else { head = true; }
            if sill && head { return true; }
        }
    }
    false
}
/// A LEVEL'S WINDOW BARS (bars.ts barLevel): every static's mark off, then on the ones the rule names. Answers how many.
pub fn bar_level(g: &mut Game, ids: &[BoxId]) -> usize {
    let f = box_flag();
    for &b in ids { set(&mut g.tab.boxes[b], f.bar, false); }
    let boxes: Vec<Box> = ids.iter().map(|&b| g.tab.boxes[b].clone()).collect();
    let bars: Vec<usize> = (0..boxes.len()).filter(|&i| slim(&boxes[i]) && in_opening(i, &boxes)).collect();
    for &i in bars.iter() { set(&mut g.tab.boxes[ids[i]], f.bar, true); }
    bars.len()
}

// ---- shapes.ts
pub const ARC_PLANES: usize = 5;
fn arc_facets(n_cut: usize) -> f64 { if n_cut <= 1 { 6.0 } else if n_cut <= 3 { 4.0 } else { 2.0 } }
/// the eight planes that, with the box's own faces, make a box a twelve-sided prism of radius r about its own x (1) or z (3)
fn ring_planes(r: f64, axis: f64) -> Vec<CutPlane> {
    let d = m::PI / 180.0;
    [30.0, 60.0, 120.0, 150.0, 210.0, 240.0, 300.0, 330.0].iter().map(|&a| cut_plane(if axis == 1.0 { [0.0, m::sin(a * d), m::cos(a * d)] } else { [m::cos(a * d), m::sin(a * d), 0.0] }, r)).collect()
}
/// an arc's `n` facets over a box of half size h
fn arc_planes(h: V3, axis: f64, rise: f64, n: f64) -> Vec<CutPlane> {
    let w = if axis == 1.0 { h[2] } else { h[0] };
    let r = (w * w + rise * rise) / (2.0 * rise);
    let (k, half) = (r - h[1], n / 2.0);
    let step = m::asin(m::min(1.0, w / r)) / (half + 0.5);
    let mut out = Vec::new();
    let mut i = -half;
    while i <= half {
        if i != 0.0 { let a = i * step; out.push(cut_plane(if axis == 1.0 { [0.0, m::cos(a), m::sin(a)] } else { [m::sin(a), m::cos(a), 0.0] }, r - k * m::cos(a))); }
        i += 1.0;
    }
    out
}
/// a shape's form, as a box's shape and a piece of trim both carry it
pub struct Form<'a> { pub round: Option<f64>, pub arc: Option<ShapeFormArc>, pub cut: Option<&'a [[f64; 4]]>, pub old: Option<bool> }
/// a form's planes over a box of half size h (shapes.ts planesOf)
pub fn planes_of(h: V3, f: &Form) -> Vec<CutPlane> {
    let more: Vec<CutPlane> = f.cut.unwrap_or(&[]).iter().map(|p| cut_plane([p[0], p[1], p[2]], p[3])).collect();
    if let Some(round) = f.round { if round != 0.0 { return ring_planes(m::min(h[1], if round == 1.0 { h[2] } else { h[0] }), round); } }
    if let Some(arc) = f.arc {
        let own = &more[..more.len().min(ARC_PLANES)];
        let mut out = arc_planes(h, arc.axis, arc.rise, arc_facets(own.len()));
        out.extend_from_slice(own);
        return out;
    }
    more
}
/// give `b` a shape's form: its record, and the picture's words of it (shapes.ts formBox)
fn form_box(b: &mut Box, f: &Form) {
    let planes = planes_of(b.h, f);
    if !planes.is_empty() { b.cut = cut_record(b.h, &planes).cut; }
    if b.cut.is_none() { return; }
    let round = f.round.filter(|&r| r != 0.0);
    if let Some(round) = round { b.cut_round = Some(round); }
    else if let Some(arc) = f.arc { let w = if arc.axis == 1.0 { b.h[2] } else { b.h[0] }; let r = (w * w + arc.rise * arc.rise) / (2.0 * arc.rise); b.cut_round = Some(arc.axis); b.cut_arc = Some(r - b.h[1]); }
    if f.old == Some(true) { b.cut_old = Some(true); }
}
/// A LEVEL'S SHAPES (shapes.ts shapeLevel with the switch up): each box that says a shape is given its record, and its pieces of trim follow the
/// level's last box, so no box changes its number. Never a tilted box, a box cut already or a window's bar. Answers how many
/// boxes are shaped; `ids` grows by the pieces.
pub fn shape_level(g: &mut Game, ids: &mut Vec<BoxId>) -> usize {
    let f = box_flag();
    let (mut made, mut tail) = (0, Vec::new());
    for (i, &id) in ids.iter().enumerate() {
        let b = &g.tab.boxes[id];
        let Some(s) = b.shape.clone() else { continue; };
        if b.rot.is_some() || b.cut.is_some() || has(b, f.bar) { continue; }
        let b = &mut g.tab.boxes[id];
        form_box(b, &Form { round: s.round, arc: s.arc, cut: s.cut.as_deref(), old: s.old });
        b.shaped = Some(true);
        let b = b.clone();
        for w in s.with.iter().flatten() {
            let o = cut_to_world(w.off, b.yaw, None);   // (the box's own frame)
            let mut p = game::scene_boxes::make_box_value(&game::scene_boxes::MakeBoxP {
                c: [b.c[0] + o[0], b.c[1] + o[1], b.c[2] + o[2]], h: w.h, yaw: Some(b.yaw + w.yaw.unwrap_or(0.0)),
                flags: Some(f.no_shadow + if has(&b, f.cutaway) { f.cutaway } else { 0.0 }), cut_height: Some(b.cut_height),   // (cut away with its box, where the box is)
                piece: Some(true), of: Some(i as f64), ..crate::level::blank_box()
            });
            // its own paint, finish and word, or its box's as the box stands
            match w.albedo {
                Some(albedo) => { p.albedo = albedo; p.rough = w.rough; p.metal = w.metal.filter(|&x| x); p.stuff = w.stuff; p.does = w.does; }
                None => { p.albedo = b.albedo; p.rough = b.rough; p.metal = b.metal.filter(|&x| x); p.stuff = b.stuff; p.does = b.does; }
            }
            form_box(&mut p, &Form { round: w.round, arc: w.arc, cut: w.cut.as_deref(), old: w.old });
            tail.push(p);
        }
        made += 1;
    }
    for p in tail { ids.push(g.tab.boxes.push(p)); }
    made
}
