//! WHAT A STAND-UP MARKS ON A LEVEL'S BOXES before they go to the world (`src/host/standup.ts populateHost`): the thin things that stand free
//! (`src/scene/thin.ts`) and a window's bars (`src/scene/bars.ts`), each by the box alone. Line for line from the TypeScript.
//! NOT HERE YET: the kit's octagons as one round box each (`scene/round.ts roundLevel`) and a box's shape (`scene/shapes.ts shapeLevel`).
use alloc::vec::Vec;
use game::game::Game;
use game::ids::*;
use game::m;
use game::scene_boxes::{box_flag, Box};
use game::scene_level::LevelDoorDef;

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
