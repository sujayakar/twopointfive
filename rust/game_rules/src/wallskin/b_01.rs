//! WHAT IS PAINTED ON A WALL GOES WITH THE WALL (src/game/wallskin.ts), whole. A sheet (an authored static that is drawn and nothing else) is drawn where
//! something solid still stands behind it: over an opened cell it is cut square, and what stands of it goes onto the frame's list as boxes of its own
//! paint, keyed MKEY.skin. No flag of any box is written; the authored sheet is left out of the picture by the world's `undrawn` (the host reads this
//! lane's `out`). The pieces are on the frame's list, which the tick's own counts read (`g.dyn_raw`, the fog's `rig_from` / `rig_to`, the frame's room):
//! so the lane runs here as it runs in the TypeScript.
//!
//! WHERE THE RUST IS NOT A PLAIN COPY:
//! - STAND-IN: THE LANE'S BOOKS ARE MADE ON FIRST USE (`ensure`). The TypeScript makes them in WallBreak's constructor, from the level as authored; the
//!   bridge does not carry `WallBreak.skins`, so an imported game holds a blank one. `ensure` reads every box's flags AS AUTHORED (what the two lanes that
//!   hide a static kept of it), so it answers the constructor's lists whenever it is asked. Belongs: the bridge (carry `cand` and `solid`), or the
//!   import's own deal.
//! - A piece is a row its sheet keeps (`scene_boxes::box_kept`) and lets go when it is cut again or made whole.
//! - The three closures handed to the constructor (`own`, `brk`, `all`) are asks of the game by path.
//! - `try … catch → fail`: a fault here is a panic with its place; `failed` is never raised.
#![allow(unused_imports, unused_variables)]
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::scene_cut::CutPlane;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

type Bx = crate::scene_boxes::Box;
type Rect = [f64; 4];

fn into_b(b: &Bx, p: V3) -> V3 { let (cs, sn) = (m::cos(b.yaw), m::sin(b.yaw)); let (x, z) = (p[0] - b.c[0], p[2] - b.c[2]); [cs * x - sn * z, p[1] - b.c[1], sn * x + cs * z] }
fn out_of_b(b: &Bx, l: V3) -> V3 { let (cs, sn) = (m::cos(b.yaw), m::sin(b.yaw)); [b.c[0] + cs * l[0] + sn * l[2], b.c[1] + l[1], b.c[2] - sn * l[0] + cs * l[2]] }

/// A point in a box's own frame (its centre the origin, its yaw undone). (src/game/wallskin.ts:82 into)
pub fn into(g: &mut crate::game::Game, b: BoxId, p: [f64; 3]) -> [f64; 3] { into_b(&g.tab.boxes[b], p) }
/// … and back. (src/game/wallskin.ts:83 outOf)
pub fn out_of(g: &crate::game::Game, b: BoxId, l: [f64; 3]) -> [f64; 3] { out_of_b(&g.tab.boxes[b], l) }
/// (src/game/wallskin.ts:84 inside)
pub fn inside(r: [f64; 4], u: f64, v: f64) -> bool { u > r[0] && u < r[1] && v > r[2] && v < r[3] }

/// WHAT STANDS of a face 2·hu × 2·hv: a point lies on the nearest thing behind it (and on any other within FLUSH of that); it is gone when it lay on
/// something and nothing it lay on stands there; the rest stands, in rectangles each as long along the face as it can be, rows of one outline joined.
/// (src/game/wallskin.ts:92 standing)
pub fn standing(hu: f64, hv: f64, on: &[[f64; 4]], near: &[f64], open: &[Vec<[f64; 4]>], cols: Option<bool>, parts: Option<&[[f64; 4]]>) -> Vec<[f64; 4]> {
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
    // (the TypeScript's `last` is the row's rectangles themselves, already in `out`: here their places in it; its key is the row's outline as a string)
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

// ---------------------------------------------------------------- the three asks the TypeScript hands the constructor as closures

/// Static j has a road of its own (a fixture's box, one of the lane's destructibles) and is no sheet of this rule. (wallbreak.ts:198, the first closure)
pub fn wall_skin_own(g: &crate::game::Game, j: f64) -> bool { g.destr.walls.fixture_boxes.has(crate::Num(j)) || g.destr.items.has(crate::Num(j)) }
/// the breakable whose own box static k is, if wallbreak opens it
fn brk_id(g: &Game, k: f64) -> Option<WallbreakBreakableId> { let b = g.destr.walls.owner.get(crate::Num(k)).copied()?; if g.tab.wallbreak_breakable[b].id == k { Some(b) } else { None } }
// ---------------------------------------------------------------- the lane's books

/// The authored statics that could be a sheet (drawn and nothing else, level, lightless) and those that are solid (met by rays and drawn, level, square).
/// (src/game/wallskin.ts:139 WallSkin.constructor)
pub fn wall_skin_new(g: &crate::game::Game, authored: f64) -> crate::wallskin::WallSkin {
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

/// STAND-IN (the head of this file): the books made the first time anything of the lane is asked on a game that came without them
fn ensure(g: &mut Game) {
    let authored = g.destr.walls.authored;
    if g.destr.walls.skins.solid.len() as f64 == authored { return; }
    let made = wall_skin_new(g, authored);
    let s = &mut g.destr.walls.skins;
    s.cand = made.cand; s.solid = made.solid;
}

/// Cells of B have just opened: every sheet lying on it is cut again as this frame's boxes are handed over. (src/game/wallskin.ts:150 WallSkin.opened)
pub fn wall_skin_opened(g: &mut crate::game::Game, b: &crate::edge_look::BreakLike) {
    if !g.k.wallskin.on || g.destr.walls.skins.failed { return; }
    for j in wall_skin_skins_of(g, b) { g.destr.walls.skins.dirty.add(crate::Num(j)); }
}

/// The level put back: every sheet whole again. (src/game/wallskin.ts:155 WallSkin.clear)
pub fn wall_skin_clear(g: &mut crate::game::Game) {
    let gone: Vec<BoxId> = g.destr.walls.skins.cut.values().flat_map(|l| l.iter().copied()).collect();
    for b in gone { crate::scene_boxes::box_let_go(g, b); }
    let s = &mut g.destr.walls.skins;
    s.cut.clear(); s.out.clear(); s.laid.clear(); s.seats.clear(); s.on.clear(); s.dirty.clear(); s.marked.clear();
    s.last = 0.0; s.room = 1e9; s.was = false; s.failed = false;
    s.stats.capped = 0.0; s.stats.coarse = 0.0; s.stats.uncut = 0.0;
    wall_skin_count(g);
}

/// (src/game/wallskin.ts:157 WallSkin.count)
pub fn wall_skin_count(g: &mut crate::game::Game) {
    let s = &mut g.destr.walls.skins;
    let mut whole = 0.0; for l in s.cut.values() { if l.is_empty() { whole += 1.0; } }
    s.stats.cut = s.cut.len() as f64; s.stats.whole = whole; s.stats.pieces = s.last;
}

/// Once a frame, before the frame's box list is made again: what the list can still take of this lane's. (src/game/wallskin.ts:160 WallSkin.update)
pub fn wall_skin_update(g: &mut crate::game::Game) {
    if g.destr.walls.skins.cut.is_empty() && g.destr.walls.skins.dirty.is_empty() { return; }
    let (last, slack) = (g.destr.walls.skins.last, g.k.wallskin.box_slack);
    if let Some(r) = crate::edge_look::frame_room(g, last, slack, None) { g.destr.walls.skins.room = r; }
}

/// What stands of the cut sheets, into the frame's dynamic list, and the world told which authored boxes to leave out. (src/game/wallskin.ts:166 WallSkin.boxes)
pub fn wall_skin_boxes(g: &mut crate::game::Game, out: &mut Vec<BoxId>) {
    let on = g.k.wallskin.on && !g.destr.walls.skins.failed;
    if !on && (!g.destr.walls.skins.cut.is_empty() || !g.destr.walls.skins.dirty.is_empty()) { let f = g.destr.walls.skins.failed; wall_skin_clear(g); g.destr.walls.skins.failed = f; }   // (put down under a standing game: the bands as authored)
    if on && !g.destr.walls.skins.was {   // (raised under a standing game: every wall that has opened is asked again)
        for b in g.destr.walls.list.clone() { if g.tab.wallbreak_breakable[b].open.iter().any(|x| *x != 0) { let bl = crate::edge_look::break_like(g, b); wall_skin_opened(g, &bl); } }
    }
    g.destr.walls.skins.was = on;
    if !g.destr.walls.skins.dirty.is_empty() {
        let mut js_: Vec<f64> = g.destr.walls.skins.dirty.keys().into_iter().map(|k| k.0).collect();
        js::sort_by(&mut js_, |p, q| *p - *q);
        for j in js_ { wall_skin_recut(g, j); }
        g.destr.walls.skins.dirty.clear(); wall_skin_count(g);
    }
    g.destr.walls.skins.out.clear();
    if !g.destr.walls.skins.cut.is_empty() {
        let slack = g.k.wallskin.box_slack;
        let now = crate::edge_look::frame_room(g, 0.0, slack, Some(out.len() as f64));
        let room0 = g.destr.walls.skins.room;
        let room = m::min(room0, now.unwrap_or(room0));   // (… and no more than the list has left as it stands now)
        let mut n = 0.0;
        let col = g.col.boxes.clone();
        for j in g.destr.walls.skins.cut.keys() {
            let l = g.destr.walls.skins.cut.get(j).cloned().unwrap_or_default();
            // (a sheet with nothing left standing needs no room to stay out)
            if !crate::scene_boxes::box_drawn(g, &col, col[j.0 as usize]) || (!l.is_empty() && n + l.len() as f64 > room) { continue; }
            for &p in l.iter() { out.push(p); }
            n += l.len() as f64; g.destr.walls.skins.out.add(j);
        }
        g.destr.walls.skins.last = n;
    } else { g.destr.walls.skins.last = 0.0; }
    g.destr.walls.skins.stats.pieces = g.destr.walls.skins.last;
    // (the world is the host's: it reads this lane's `out`; the word here says only whether there is one)
    g.engine.world.undrawn = if g.destr.walls.skins.out.is_empty() { None } else { Some(crate::scene_boxes::BoxWorldUndrawn {}) };
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
/// (src/game/wallskin.ts:200 WallSkin.backs)
pub fn wall_skin_backs(g: &mut crate::game::Game, j: f64) -> Vec<crate::wallskin::Seat> {
    ensure(g);
    if let Some(l) = g.destr.walls.skins.seats.get(crate::Num(j)) { return l.clone(); }
    let kk = g.k.wallskin;
    let s = g.tab.boxes[g.col.boxes[j as usize]].clone(); let r_ = s.h[0] + s.h[2] + kk.off;
    let mut l: Vec<Seat> = Vec::new();
    if !wall_skin_own(g, j) {
        for t in (if s.h[0] <= s.h[2] { [0.0, 2.0] } else { [2.0, 0.0] }) {
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

/// A breakable's sheets: of the sheets near it, those that lie on its own box. (src/game/wallskin.ts:220 WallSkin.skinsOf)
pub fn wall_skin_skins_of(g: &mut crate::game::Game, b: &crate::edge_look::BreakLike) -> Vec<f64> {
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

// ---------------------------------------------------------------- cutting one

#[allow(clippy::too_many_arguments)]
fn cells_of(s: &Bx, q: &Seat, w: &Bx, wnx: f64, wny: f64, wnz: f64, yb: &[f64], open: &[u8], a: f64) -> Vec<Rect> {
    let mut out: Vec<Rect> = Vec::new();
    let (n, face) = (q.n, q.face); let u = 2.0 - n; let nn = if n == 0.0 { wnz } else { wnx }; let at = if face > 0.0 { (if n == 0.0 { wnx } else { wnz }) - 1.0 } else { 0.0 };
    let mut k = 0.0;
    while k < nn {
        let mut iy = 0.0;
        while iy < wny {
            let (ix, iz) = (if n == 0.0 { at } else { k }, if n == 0.0 { k } else { at });
            if open.get(((iy * wnz + iz) * wnx + ix) as usize).copied().unwrap_or(0) != 0 {
                let (mut lo, mut hi): (V3, V3) = ([0.0, yb[iy as usize], 0.0], [0.0, yb[iy as usize + 1], 0.0]);
                lo[ax(n)] = face * w.h[ax(n)]; hi[ax(n)] = face * w.h[ax(n)];
                lo[ax(u)] = -w.h[ax(u)] + 2.0 * w.h[ax(u)] * k / nn; hi[ax(u)] = -w.h[ax(u)] + 2.0 * w.h[ax(u)] * (k + 1.0) / nn;
                let (p, e) = (into_b(s, out_of_b(w, lo)), into_b(s, out_of_b(w, hi)));
                let r: Rect = [m::min(p[ax(a)], e[ax(a)]), m::max(p[ax(a)], e[ax(a)]), m::min(p[1], e[1]), m::max(p[1], e[1])];
                if m::min(r[1], s.h[ax(a)]) - m::max(r[0], -s.h[ax(a)]) > EPS && m::min(r[3], s.h[1]) - m::max(r[2], -s.h[1]) > EPS { out.push(r); }
            }
            iy += 1.0;
        }
        k += 1.0;
    }
    out
}

fn uncut(g: &mut Game, j: f64) {
    if let Some(old) = g.destr.walls.skins.cut.delete(crate::Num(j)) { for b in old { crate::scene_boxes::box_let_go(g, b); } }
    g.destr.walls.skins.laid.delete(crate::Num(j));
}

/// One sheet cut again from what stands behind it. (src/game/wallskin.ts:245 WallSkin.recut)
pub fn wall_skin_recut(g: &mut crate::game::Game, j: f64) {
    let seats = wall_skin_backs(g, j); if seats.is_empty() { return; }
    let kk = g.k.wallskin; let fl = crate::scene_boxes::box_flag(); let col = g.col.boxes.clone();
    let s = g.tab.boxes[col[j as usize]].clone();
    let t = seats[0].t; let a = 2.0 - t; let (hu, hv) = (s.h[ax(a)], s.h[1]);
    // what has gone of each thing it lay on: a breakable's open cells at that face; any other solid whole, once something has hidden it
    let on: Vec<Rect> = seats.iter().map(|q| q.r).collect(); let near: Vec<f64> = seats.iter().map(|q| q.near).collect();
    let open: Vec<Vec<Rect>> = seats.iter().map(|q| match brk_id(g, q.k) {
        Some(b) => { let w = &g.tab.wallbreak_breakable[b]; cells_of(&s, q, &g.tab.boxes[w.box_], w.nx, w.ny, w.nz, &w.yb, &w.open, a) }
        None => if js::to_i32(g.tab.boxes[col[q.k as usize]].flags) & js::to_i32(fl.no_primary) != 0 { vec![all()] } else { Vec::new() },
    }).collect();
    let whole = |r: &[Rect]| r.len() == 1 && r[0][0] <= -hu + EPS && r[0][1] >= hu - EPS && r[0][2] <= -hv + EPS && r[0][3] >= hv - EPS;
    // what stands behind may END RAGGED (a breakable's standing piece under a cut record, with the cut switch up): the part of the sheet on such a piece
    // takes the outline of that piece's face
    let mut parts: Vec<Rect> = Vec::new(); let mut rag: Vec<(Bx, Seat)> = Vec::new();
    for q in seats.iter() {
        let Some(b) = brk_id(g, q.k) else { continue; };
        let (live, slots) = { let w = &g.tab.wallbreak_breakable[b]; (w.live, w.slots.clone()) };
        let mut i = 0.0;
        while i < live {
            let pi = slots.get(i as usize).copied(); i += 1.0;
            let Some(pid) = pi.and_then(|x| col.get(x as usize).copied()) else { continue; };
            let p = &g.tab.boxes[pid];
            if !p.cut.as_ref().map(|c| !c.is_empty()).unwrap_or(false) { continue; }
            let (lo, hi) = (into_b(&s, out_of_b(p, [-p.h[0], -p.h[1], -p.h[2]])), into_b(&s, out_of_b(p, [p.h[0], p.h[1], p.h[2]])));
            let r: Rect = [m::min(lo[ax(a)], hi[ax(a)]), m::max(lo[ax(a)], hi[ax(a)]), m::min(lo[1], hi[1]), m::max(lo[1], hi[1])];
            if m::min(r[1], hu) - m::max(r[0], -hu) > EPS && m::min(r[3], hv) - m::max(r[2], -hv) > EPS { parts.push(r); rag.push((p.clone(), *q)); }
        }
    } 
    let mut rects: Vec<Rect> = if open.iter().any(|l| !l.is_empty()) || !parts.is_empty() { standing(hu, hv, &on, &near, &open, Some(false), Some(&parts)) } else { vec![[-hu, hu, -hv, hv]] };
    if whole(&rects) && parts.is_empty() { uncut(g, j); return; }   // (something solid stands behind every part of it that lay on anything: the authored box, whole)
    let marked = s.marks.as_ref().map(|x| !x.is_empty()).unwrap_or(false); let base: Vec<CutPlane> = s.cut.clone().unwrap_or_default();
    if marked && base.len() as f64 + 4.0 > crate::scene_cut::MAX_CUT_PLANES {   // (its marks are laid out on the whole box and no record can be drawn, or has room: left as it was)
        if !g.destr.walls.skins.marked.has(crate::Num(j)) { g.destr.walls.skins.marked.add(crate::Num(j)); g.destr.walls.skins.stats.uncut += 1.0; }
        return;
    }
    // (what the other sheets hand over: one the hangers have since thrown spends none of the cap)
    let mut others = 0.0;
    for i in g.destr.walls.skins.cut.keys() { if i.0 != j && crate::scene_boxes::box_drawn(g, &col, col[i.0 as usize]) { others += g.destr.walls.skins.cut.get(i).map(|l| l.len()).unwrap_or(0) as f64; } }
    let fits = |r: &[Rect]| r.len() as f64 <= m::min(kk.per_skin, SLOTS) && others + r.len() as f64 <= kk.max_pieces;
    if !fits(&rects) && !parts.is_empty() { parts.clear(); rag.clear(); rects = standing(hu, hv, &on, &near, &open, None, None); if whole(&rects) { uncut(g, j); return; } }   // (past a cap: square ends first)
    if !fits(&rects) { rects = standing(hu, hv, &on, &near, &open, Some(true), None); if !fits(&rects) { g.destr.walls.skins.stats.capped += 1.0; return; } g.destr.walls.skins.stats.coarse += 1.0; }   // (… then by whole columns, else as it last stood)
    // the planes of the piece behind rectangle `r`, about a box of the sheet's frame centred at `pc`: each taken where it meets the face the sheet lies
    // on, and stood square to that face
    let outline = |r: &Rect, pc: V3| -> Vec<CutPlane> {
        let Some(k) = parts.iter().position(|x| inside(*x, (r[0] + r[1]) / 2.0, (r[2] + r[3]) / 2.0)) else { return Vec::new(); };
        let (p, q) = (&rag[k].0, &rag[k].1);
        let o = into_b(p, pc); let tf = q.side * (s.h[ax(t)] + m::max(q.near, 0.0)); let zero = into_b(&s, p.c);
        let mut out: Vec<CutPlane> = Vec::new();
        for c in p.cut.as_ref().unwrap().iter() {
            let mut n = into_b(&s, out_of_b(p, c.n)); n[0] -= zero[0]; n[1] -= zero[1]; n[2] -= zero[2];
            let d = c.d - (c.n[0] * o[0] + c.n[1] * o[1] + c.n[2] * o[2]) - n[ax(t)] * tf; n[ax(t)] = 0.0;
            let l = m::hyp3(n[0], n[1], n[2]);
            if l > 1e-6 { out.push(CutPlane { n: [n[0] / l, n[1] / l, n[2] / l], d: d / l }); }
        }
        out
    };
    // a piece's key is its sheet and one of 16 places: a piece a later cut leaves alone keeps its place, and a new one takes a place no piece of the
    // last cut had while there is one
    let prev: Vec<WallSkinLaidValueItem> = g.destr.walls.skins.laid.get(crate::Num(j)).cloned().unwrap_or_default();
    let old: Vec<f64> = prev.iter().map(|p| p.slot).collect(); let mut now: Vec<f64> = Vec::new();
    let mut lay: Vec<WallSkinLaidValueItem> = rects.iter().map(|r| {
        let same = prev.iter().find(|p| (0..4).all(|i| m::abs(p.r[i] - r[i]) <= EPS));
        if let Some(sm) = same { if !now.contains(&sm.slot) { now.push(sm.slot); } }
        WallSkinLaidValueItem { r: *r, slot: same.map(|x| x.slot).unwrap_or(-1.0) }
    }).collect();
    for x in lay.iter_mut() {
        if x.slot >= 0.0 { continue; }
        let mut k = 0.0; while k < SLOTS && (now.contains(&k) || old.contains(&k)) { k += 1.0; }
        if k == SLOTS { k = 0.0; while now.contains(&k) { k += 1.0; } }
        x.slot = k; now.push(k);
    }
    let mut pieces: Vec<Bx> = Vec::new(); let mut kept: Vec<WallSkinLaidValueItem> = Vec::new(); let mut bit = false;
    let mut area = 0.0; for r in rects.iter() { area += (r[1] - r[0]) * (r[3] - r[2]); }
    for x in lay.iter() {
        let (r, slot) = (x.r, x.slot);
        let (mut lc, mut h): (V3, V3) = ([0.0, (r[2] + r[3]) / 2.0, 0.0], [0.0, (r[3] - r[2]) / 2.0, 0.0]);
        lc[ax(a)] = (r[0] + r[1]) / 2.0; h[ax(a)] = (r[1] - r[0]) / 2.0; h[ax(t)] = s.h[ax(t)];
        let mut p = s.clone();
        p.flags = ((js::to_i32(s.flags) & !js::to_i32(fl.no_primary)) | js::to_i32(fl.dynamic)) as f64; p.name = Some(String::from("skin"));
        p.mkey = Some(crate::scene_boxes::mkey().skin + j * SLOTS + slot);
        if marked {   // the whole box again, under the planes of the rectangle that stands (a side that is the box's own takes none): every mark where it was
            let mut planes: Vec<CutPlane> = base.clone();
            let mut side = |axis: usize, sign: f64, at: f64| { let mut n: V3 = [0.0, 0.0, 0.0]; n[axis] = sign; planes.push(CutPlane { n, d: sign * at }); };
            if r[0] > -hu + EPS { side(ax(a), -1.0, r[0]); } if r[1] < hu - EPS { side(ax(a), 1.0, r[1]); } if r[2] > -hv + EPS { side(1, -1.0, r[2]); } if r[3] < hv - EPS { side(1, 1.0, r[3]); }
            p.c = s.c; p.h = s.h; p.cut = Some(planes);
        } else {
            p.c = out_of_b(&s, lc); p.h = h;
            // (a shape's planes are in the box's own frame: the same planes about the piece's middle)
            p.cut = if !base.is_empty() { Some(base.iter().map(|c| CutPlane { n: c.n, d: c.d - (c.n[0] * lc[0] + c.n[1] * lc[1] + c.n[2] * lc[2]) }).collect()) } else { None };
        }
        if !parts.is_empty() {
            let more = outline(&r, p.c);
            if !more.is_empty() {
                let mine = p.cut.clone().unwrap_or_default();
                let own = crate::scene_cut::cut_record(p.h, &mine).cut.map(|c| c.len()).unwrap_or(0);
                let mut both = mine; both.extend(more);
                let rr = crate::scene_cut::cut_record(p.h, &both);
                if rr.empty || rr.cut.as_ref().map(|c| c.len()).unwrap_or(0) > own { bit = true; }
                if rr.empty { continue; }
                p.cut = rr.cut;
            }
        }
        // (a plain box of the sheet's paint: what the level's stand-up keeps on the authored one is not its)
        p.was = None; p.shape = None; p.shaped = None; p.piece = None; p.of = None;
        pieces.push(p); kept.push(*x);
    }
    if !bit && area >= 4.0 * hu * hv - EPS { uncut(g, j); return; }   // (split at a ragged piece's bounds, and no plane of it bites the sheet: the authored box, whole)
    g.destr.walls.skins.laid.set(crate::Num(j), kept);
    let rows: Vec<BoxId> = pieces.into_iter().map(|p| crate::scene_boxes::box_kept(g, p)).collect();
    if let Some(old) = g.destr.walls.skins.cut.set(crate::Num(j), rows) { for b in old { crate::scene_boxes::box_let_go(g, b); } }
}
