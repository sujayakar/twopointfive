#![allow(unused_imports, unused_variables)]
//! WHAT IS PAINTED ON A WALL GOES WITH THE WALL, whole. A sheet (an authored static that is drawn and nothing else) is drawn where
//! something solid still stands behind it: over an opened cell it is cut square, and what stands of it goes onto the frame's list as boxes of its own
//! paint, keyed MKEY.skin. No flag of any box is written; the authored sheet is left out of the picture by the world's `undrawn` (the host reads this
//! lane's `out`). The pieces are on the frame's list, which the tick's own counts read (the fog's `rig_from` / `rig_to`):
//! so the lane runs here.
//!
//! WHAT TO KNOW:
//! - THE LANE'S BOOKS ARE MADE ON FIRST USE (`ensure`), from the level as authored: `ensure` reads every box's flags AS AUTHORED (what the two lanes that
//!   hide a static kept of it), so it gives the same lists whenever it is asked.
//! - A piece is a row its sheet keeps (`scene_boxes::box_kept`) and lets go when it is cut again or made whole.
//! - A fault here is a panic with its place; `failed` is never raised.

pub use ::game_types::wallskin::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

use crate::scene_cut::CutPlane;

type Bx = crate::scene_boxes::Box;
type Rect = [f64; 4];

mod cut;
use cut::wall_skin_recut;

fn into_b(b: &Bx, p: V3) -> V3 { let (cs, sn) = (m::cos(b.yaw), m::sin(b.yaw)); let (x, z) = (p[0] - b.c[0], p[2] - b.c[2]); [cs * x - sn * z, p[1] - b.c[1], sn * x + cs * z] }
fn out_of_b(b: &Bx, l: V3) -> V3 { let (cs, sn) = (m::cos(b.yaw), m::sin(b.yaw)); [b.c[0] + cs * l[0] + sn * l[2], b.c[1] + l[1], b.c[2] - sn * l[0] + cs * l[2]] }

fn inside(r: [f64; 4], u: f64, v: f64) -> bool { u > r[0] && u < r[1] && v > r[2] && v < r[3] }

/// WHAT STANDS of a face 2·hu × 2·hv: a point lies on the nearest thing behind it (and on any other within FLUSH of that); it is gone when it lay on
/// something and nothing it lay on stands there; the rest stands, in rectangles each as long along the face as it can be, rows of one outline joined.
fn standing(hu: f64, hv: f64, on: &[[f64; 4]], near: &[f64], open: &[Vec<[f64; 4]>], cols: Option<bool>, parts: Option<&[[f64; 4]]>) -> Vec<[f64; 4]> {
    let cols = cols.unwrap_or(false); let parts: &[Rect] = parts.unwrap_or(&[]);
    let edges = |h: f64, k: usize| -> Vec<f64> {
        let mut e = vec![-h, h];
        let mut add = |x: f64| { if x > -h + EPS && x < h - EPS && !e.iter().any(|y| m::abs(*y - x) <= EPS) { e.push(x); } };
        for r in on { add(r[k]); add(r[k + 1]); }
        for l in open { for r in l { add(r[k]); add(r[k + 1]); } }
        for r in parts { add(r[k]); add(r[k + 1]); }
        js::sort_by(&mut e, |p, q| *p - *q);
        e
    };
    let gone = |u: f64, v: f64| -> bool {
        let mut mn = 1e9;
        for i in 0..on.len() { if near[i] < mn && inside(on[i], u, v) { mn = near[i]; } }
        for i in 0..on.len() { if near[i] <= mn + FLUSH && inside(on[i], u, v) && !open[i].iter().any(|r| inside(*r, u, v)) { return false; } }
        mn < 1e9
    };
    let part = |u: f64, v: f64| -> i64 { parts.iter().position(|r| inside(*r, u, v)).map(|i| i as i64).unwrap_or(-1) };
    let (us, vs) = (edges(hu, 0), edges(hv, 2));
    let mut out: Vec<Rect> = Vec::new();
    // (`last` is the last row's rectangles, by their places in `out`; its key is the row's outline)
    let (mut last_from, mut last_n) = (0usize, 0usize); let mut last_key: Vec<(f64, f64, i64)> = Vec::new();
    let col: Option<Vec<bool>> = if cols { Some((0..us.len() - 1).map(|i| (0..vs.len() - 1).any(|j| gone((us[i] + us[i + 1]) / 2.0, (vs[j] + vs[j + 1]) / 2.0))).collect()) } else { None };
    for j in 0..vs.len() - 1 {
        let vm = (vs[j] + vs[j + 1]) / 2.0;
        let mut row: Vec<Rect> = Vec::new(); let mut tags: Vec<i64> = Vec::new(); let (mut from, mut tag) = (-1i64, -1i64);
        for i in 0..us.len() - 1 {
            let um = (us[i] + us[i + 1]) / 2.0;
            let gn = match &col { Some(c) => c[i], None => gone(um, vm) };
            let k = if gn || parts.is_empty() { -1 } else { part(um, vm) };
            if from >= 0 && (gn || k != tag) { row.push([us[from as usize], us[i], vs[j], vs[j + 1]]); tags.push(tag); from = -1; }
            if !gn && from < 0 { from = i as i64; tag = k; }
        }
        if from >= 0 { row.push([us[from as usize], us[us.len() - 1], vs[j], vs[j + 1]]); tags.push(tag); }
        let key: Vec<(f64, f64, i64)> = row.iter().zip(tags.iter()).map(|(r, t)| (r[0], r[1], *t)).collect();
        if j > 0 && key == last_key { for r in out[last_from..last_from + last_n].iter_mut() { r[3] = vs[j + 1]; } }
        else { last_from = out.len(); last_n = row.len(); for r in row { out.push(r); } last_key = key; }
    }
    out
}

// ---------------------------------------------------------------- what a static is to this rule

/// Static j has a road of its own (a fixture's box, one of the lane's destructibles) and is no sheet of this rule.
fn wall_skin_own(g: &crate::game::Game, j: f64) -> bool { g.destr.walls.fixture_boxes.has(crate::Num(j)) || g.destr.items.has(crate::Num(j)) }
/// the breakable whose own box static k is, if wallbreak opens it
fn brk_id(g: &Game, k: f64) -> Option<WallbreakBreakableId> { let b = g.destr.walls.owner.get(crate::Num(k)).copied()?; if g.tab.wallbreak_breakable[b].id == k { Some(b) } else { None } }
// ---------------------------------------------------------------- the lane's books

/// The authored statics that could be a sheet (drawn and nothing else, level, lightless) and those that are solid (met by rays and drawn, level, square).
fn wall_skin_new(g: &crate::game::Game, authored: f64) -> crate::wallskin::WallSkin {
    let f = crate::scene_boxes::box_flag();
    let lone = js::to_i32(f.no_shadow) | js::to_i32(f.no_primary) | js::to_i32(f.glass) | js::to_i32(f.dynamic);
    let mut w = WallSkin::blank();
    let n = authored as usize; w.solid = vec![0; n];
    for i in 0..n {
        let id = g.col.boxes[i];
        // (THE FLAGS AS AUTHORED: a box either lane has since hidden or holed is asked by what it was loaded with, as the constructor found it)
        let flags0 = g.destr.walls.hidden.get(crate::Num(i as f64)).copied().or_else(|| brk_id(g, i as f64).map(|b| g.tab.wallbreak_breakable[b].flags0)).or_else(|| g.destr.hidden.get(crate::Num(i as f64)).copied());
        let b = &g.tab.boxes[id];
        if b.rot.is_some() || b.round == Some(true) { continue; }
        let fl = js::to_i32(flags0.unwrap_or(b.flags)) & lone;
        if fl == 0 { w.solid[i] = 1; }
        else if fl == js::to_i32(f.no_shadow) && b.emissive[0] + b.emissive[1] + b.emissive[2] <= 0.0 { w.cand.push(i as f64); }   // (trim a box says is asked like any other)
    }
    w
}

/// the books made the first time anything of the lane is asked on a game that came without them
fn ensure(g: &mut Game) {
    let authored = g.destr.walls.authored;
    if g.destr.walls.skins.solid.len() as f64 == authored { return; }
    let made = wall_skin_new(g, authored);
    let s = &mut g.destr.walls.skins;
    s.cand = made.cand; s.solid = made.solid;
}

/// Cells of B have just opened: every sheet lying on it is cut again as this frame's boxes are handed over.
pub(super) fn wall_skin_opened(g: &mut crate::game::Game, b: &crate::edge_look::BreakLike) {
    if g.destr.walls.skins.failed { return; }
    for j in wall_skin_skins_of(g, b) { g.destr.walls.skins.dirty.add(crate::Num(j)); }
}

/// The level put back: every sheet whole again.
pub(super) fn wall_skin_clear(g: &mut crate::game::Game) {
    let gone: Vec<FrameBoxId> = g.destr.walls.skins.cut.values().flat_map(|l| l.iter().copied()).collect();
    for b in gone { crate::scene_boxes::box_let_go(g, b); }
    let s = &mut g.destr.walls.skins;
    s.cut.clear(); s.out.clear(); s.laid.clear(); s.seats.clear(); s.on.clear(); s.dirty.clear(); s.marked.clear();
     s.room = 1e9; s.was = false; s.failed = false;
}

/// Once a frame, before the frame's box list is made again: what the list can still take of this lane's.
pub(super) fn wall_skin_update(g: &mut crate::game::Game) {
    if g.destr.walls.skins.cut.is_empty() && g.destr.walls.skins.dirty.is_empty() { return; }
    g.destr.walls.skins.room = crate::edge_look::frame_room(g, g.k.wallskin.box_slack, None);
}

/// What stands of the cut sheets, into the frame's dynamic list, and the world told which authored boxes to leave out.
pub(super) fn wall_skin_boxes(g: &mut crate::game::Game, out: &mut Vec<AnyBoxId>) {
    let on = !g.destr.walls.skins.failed;
    if !on && (!g.destr.walls.skins.cut.is_empty() || !g.destr.walls.skins.dirty.is_empty()) { let f = g.destr.walls.skins.failed; wall_skin_clear(g); g.destr.walls.skins.failed = f; }   // (put down under a standing game: the bands as authored)
    if on && !g.destr.walls.skins.was {   // (raised under a standing game: every wall that has opened is asked again)
        for b in g.destr.walls.list.clone() { if g.tab.wallbreak_breakable[b].open.iter().any(|x| *x != 0) { let bl = crate::edge_look::break_like(g, b); wall_skin_opened(g, &bl); } }
    }
    g.destr.walls.skins.was = on;
    if !g.destr.walls.skins.dirty.is_empty() {
        let mut js_: Vec<f64> = g.destr.walls.skins.dirty.keys().into_iter().map(|k| k.0).collect();
        js::sort_by(&mut js_, |p, q| *p - *q);
        for j in js_ { wall_skin_recut(g, j); }
        g.destr.walls.skins.dirty.clear();
    }
    g.destr.walls.skins.out.clear();
    if !g.destr.walls.skins.cut.is_empty() {
        let slack = g.k.wallskin.box_slack;
        let now = crate::edge_look::frame_room(g, slack, Some(out.len() as f64));
        let room = m::min(g.destr.walls.skins.room, now);   // (… and no more than the list has left as it stands now)
        let mut n = 0.0;
        let col = g.col.boxes.clone();
        for j in g.destr.walls.skins.cut.keys() {
            let l = g.destr.walls.skins.cut.get(j).cloned().unwrap_or_default();
            // (a sheet with nothing left standing needs no room to stay out)
            if !crate::scene_boxes::box_drawn(g, &col, col[j.0 as usize]) || (!l.is_empty() && n + l.len() as f64 > room) { continue; }
            for &p in l.iter() { out.push(p.into()); }
            n += l.len() as f64; g.destr.walls.skins.out.add(j);
        }
    } else {  }
}

// ---------------------------------------------------------------- what a sheet lies on

fn ax(x: f64) -> usize { x as usize }

fn seat_of(off: f64, k: f64, w: &Bx, s: &Bx, t: f64) -> Option<Seat> {
    let r = s.yaw - w.yaw; if m::abs(m::sin(2.0 * r)) > 1e-4 { return None; }
    let n = if m::abs(m::sin(r)) > 0.5 { 2.0 - t } else { t }; let u = 2.0 - n; let a = 2.0 - t;
    let (hn, hu, hh) = (s.h[ax(t)], s.h[ax(a)], w.h[ax(n)]); let l = into_b(w, s.c);
    let (u0, u1) = (m::max(l[ax(u)] - hu, -w.h[ax(u)]), m::min(l[ax(u)] + hu, w.h[ax(u)]));
    let (y0, y1) = (m::max(l[1] - s.h[1], -w.h[1]), m::min(l[1] + s.h[1], w.h[1]));
    if u1 - u0 <= EPS || y1 - y0 <= EPS { return None; }
    let (face, near);
    if l[ax(n)] + hn - hh > EPS { face = 1.0; near = l[ax(n)] - hn - hh; } else if -hh - (l[ax(n)] - hn) > EPS { face = -1.0; near = -hh - (l[ax(n)] + hn); } else { return None; }
    if near > off + EPS { return None; }
    let (mut lo, mut hi, mut o): (V3, V3, V3) = ([0.0, y0, 0.0], [0.0, y1, 0.0], [0.0, 0.0, 0.0]);
    lo[ax(n)] = face * hh; hi[ax(n)] = face * hh; lo[ax(u)] = u0; hi[ax(u)] = u1; o[ax(n)] = face;
    let (p, q) = (into_b(s, out_of_b(w, lo)), into_b(s, out_of_b(w, hi)));
    let out = into_b(s, out_of_b(w, o))[ax(t)] - into_b(s, w.c)[ax(t)];   // (the face's outward normal along the sheet's thin axis: the solid is on the other side)
    Some(Seat { k, n, face, t, side: if out > 0.0 { -1.0 } else { 1.0 }, near, r: [m::min(p[ax(a)], q[ax(a)]), m::max(p[ax(a)], q[ax(a)]), m::min(p[1], q[1]), m::max(p[1], q[1])] })
}

/// What sheet j lies on: every authored solid it has a seat on, on its BACK (the side more of it lies against, then the nearer, then the earlier box).
fn wall_skin_backs(g: &mut crate::game::Game, j: f64) -> Vec<crate::wallskin::Seat> {
    ensure(g);
    if let Some(l) = g.destr.walls.skins.seats.get(crate::Num(j)) { return l.clone(); }
    let kk = g.k.wallskin;
    let s = g.tab.boxes[g.col.boxes[j as usize]].clone(); let r_ = s.h[0] + s.h[2] + kk.off;
    let mut l: Vec<Seat> = Vec::new();
    if !wall_skin_own(g, j) {
        for t in if s.h[0] <= s.h[2] { [0.0, 2.0] } else { [2.0, 0.0] } {
            if 2.0 * s.h[ax(t)] > kk.thick + EPS || s.h[ax(t)] > s.h[1] + EPS { continue; }   // (deeper than a sheet; or thinner up than across: the floor's)
            let mut got: Vec<Seat> = Vec::new(); let (mut sum, mut near, mut first) = ([0.0, 0.0], [1e9, 1e9], [1e9, 1e9]);
            for k in 0..g.destr.walls.skins.solid.len() {
                if g.destr.walls.skins.solid[k] == 0 { continue; }
                let w = &g.tab.boxes[g.col.boxes[k]]; let far = r_ + w.h[0] + w.h[2];
                if m::abs(s.c[0] - w.c[0]) > far || m::abs(s.c[2] - w.c[2]) > far || m::abs(s.c[1] - w.c[1]) > s.h[1] + w.h[1] { continue; }
                let Some(q) = seat_of(kk.off, k as f64, w, &s, t) else { continue; };
                let i = if q.side > 0.0 { 1 } else { 0 };
                got.push(q); sum[i] += (q.r[1] - q.r[0]) * (q.r[3] - q.r[2]); near[i] = m::min(near[i], q.near); first[i] = m::min(first[i], k as f64);
            }
            if got.is_empty() { continue; }
            let side = if m::abs(sum[1] - sum[0]) > EPS { if sum[1] > sum[0] { 1.0 } else { -1.0 } } else if m::abs(near[1] - near[0]) > EPS { if near[1] < near[0] { 1.0 } else { -1.0 } } else if first[1] < first[0] { 1.0 } else { -1.0 };
            l = got.into_iter().filter(|q| q.side == side).collect();
            break;
        }
    }
    g.destr.walls.skins.seats.set(crate::Num(j), l.clone());
    l
}

/// A breakable's sheets: of the sheets near it, those that lie on its own box.
fn wall_skin_skins_of(g: &mut crate::game::Game, b: &crate::edge_look::BreakLike) -> Vec<f64> {
    ensure(g);
    if let Some(l) = g.destr.walls.skins.on.get(crate::Num(b.id)) { return l.clone(); }
    let (wc, wh) = { let w = &g.tab.boxes[b.box_]; (w.c, w.h) }; let reach = wh[0] + wh[2] + g.k.wallskin.off;
    let mut l: Vec<f64> = Vec::new();
    for j in g.destr.walls.skins.cand.clone() {
        let (sc, sh) = { let s = &g.tab.boxes[g.col.boxes[j as usize]]; (s.c, s.h) }; let far = reach + sh[0] + sh[2];
        if m::abs(sc[0] - wc[0]) > far || m::abs(sc[2] - wc[2]) > far { continue; }
        if wall_skin_backs(g, j).iter().any(|q| q.k == b.id) { l.push(j); }
    }
    g.destr.walls.skins.on.set(crate::Num(b.id), l.clone());
    l
}
