#![allow(unused_imports, unused_variables)]
//! A BROKEN WALL'S LOOK, whole: the standing pieces round a cut, the rubble at its foot, the soot. Every piece is a box keyed
//! MKEY.look on the frame's list, which no ray of the game meets; but the fog of war keeps a record of every keyed loose thing of that list and caps its
//! records, the boots scuff the rubble at rest, and the frame's room is budgeted with these boxes in it: so the tick does read what this lane writes, and
//! the lane runs here as it runs in the TypeScript.
//!
//! WHERE THE RUST IS NOT A PLAIN COPY:
//! - A BREAKABLE IS HELD BY VALUE. The TypeScript's `dirty` map and `todo` list hold the wallbreak's own Breakable (an object: its cells are read as they
//!   stand when the wall is dressed a frame later). The generated `BreakLike` is a copy, so every copy this lane holds of a breakable is written again
//!   whenever cells of that breakable open (`opened`, the lane's one hook, is every way a cell opens).
//! - A PIECE'S BOX is a row its holder keeps (`scene_boxes::box_kept`) and lets go when the piece is dropped (a wall dressed again, a cap, a reset).
//! - `try … catch → fail` has no twin: a fault here is a panic with its place, not a look switched off.

pub use ::game_types::edge_look::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

use crate::v3::quat;

type Bx = crate::scene_boxes::Box;

/// what this lane reads of a breakable, as it stands now
pub(in crate::breakage) fn break_like(g: &Game, b: WallbreakBreakableId) -> BreakLike {
    let r = &g.tab.wallbreak_breakable[b];
    BreakLike { id: r.id, kind: r.kind, surf: String::from(r.surf.as_str()), box_: r.box_, nx: r.nx, ny: r.ny, nz: r.nz, yb: r.yb.clone(), open: r.open.clone() }
}

/// What the frame's box list can take of a picture-only lane's boxes: the table, less the level's boxes, less what the frame keeps for its own (`FRAME_BOXES`, or
/// `now`: what a lane has seen on the list as it stands), less `slack`.
pub(in crate::breakage) fn frame_room(g: &crate::game::Game, slack: f64, now: Option<f64>) -> f64 {
    let w = &g.engine.world;
    w.max_boxes - w.statics.len() as f64 - now.unwrap_or(crate::scene_boxes::FRAME_BOXES) - slack
}

/// Cells of a wall just opened: the look's one hook.
pub(in crate::breakage) fn edge_look_opened(g: &mut crate::game::Game, b: &crate::edge_look::BreakLike, fresh: &[f64], cause: &str, dir: Option<V3>, quiet: bool) {
    // (the copies this lane already holds of this breakable read its cells as they now stand: the TypeScript holds the object)
    { let e = &mut g.destr.edge_look;
      if let Some(d) = e.dirty.get_mut(crate::Num(b.id)) { d.open = b.open.clone(); }
      for q in e.todo.iter_mut() { if q.b.id == b.id && q.b.kind == b.kind { q.b.open = b.open.clone(); } } }
    if !g.m.edge_look.edge.on { return; }
    edge_look_dress(g, b, fresh, cause, dir, quiet);
}

/// The frame a wall goes: only the soot's record and its marks land on it; the wall is dressed on the next frame and its rubble shed a few cells a frame
/// after that. A checkpoint being applied does it all at once.
pub(in crate::breakage) fn edge_look_dress(g: &mut crate::game::Game, b: &crate::edge_look::BreakLike, fresh: &[f64], cause: &str, dir: Option<V3>, quiet: bool) {
    let wall = matches!(b.kind, crate::wallbreak::BKind::Wall);
    if wall { g.destr.edge_look.dirty.set(crate::Num(b.id), b.clone()); }
    let f = if wall { Some(edge_look_frame(g, b)) } else { None };
    let blast = cause == "blast";
    if quiet || g.m.edge_look.edge.spread <= 0.0 {
        for &c in fresh { edge_look_shed(g, b, f, c, dir, quiet, blast); }
        edge_look_cap(g);
    } else {
        for &c in fresh { g.destr.edge_look.todo.push(EdgeLookTodoItem { b: b.clone(), f, c, dir, blast }); }
        g.destr.edge_look.held = true;
    }
    if let Some(f) = f { if blast && !quiet { edge_look_scorch(g, b, f, fresh, dir); } }
}

/// The wall's own frame: u along it, y up, v across it.
pub(in crate::breakage) fn edge_look_frame(g: &crate::game::Game, b: &crate::edge_look::BreakLike) -> crate::edge_look::Frame {
    let (c, h, yaw) = { let r = &g.tab.boxes[b.box_]; (r.c, r.h, r.yaw) };
    let along_x = b.nx > 1.0 || h[0] >= h[2];
    let (cs, sn) = (m::cos(yaw), m::sin(yaw));
    let n = if along_x { b.nx } else { b.nz };
    let l = 2.0 * (if along_x { h[0] } else { h[2] });
    Frame {
        b: b.box_, along_x, t: if along_x { h[2] } else { h[0] }, l, n, w: l / n,
        y0: c[1] - h[1], y_top: c[1] + h[1], y_sill: c[1] + b.yb[1],
        along: if along_x { [cs, 0.0, -sn] } else { [sn, 0.0, cs] },
        nrm: if along_x { [sn, 0.0, cs] } else { [cs, 0.0, -sn] },
    }
}

/// A place in the wall's frame.
pub(in crate::breakage) fn edge_look_at(g: &crate::game::Game, f: crate::edge_look::Frame, u: f64, y: f64, v: f64) -> V3 {
    let c = g.tab.boxes[f.b].c;
    at_c(c, &f, u, y, v)
}
fn at_c(c: V3, f: &Frame, u: f64, y: f64, v: f64) -> V3 { [c[0] + f.along[0] * u + f.nrm[0] * v, y, c[2] + f.along[2] * u + f.nrm[2] * v] }

/// The oldest litter goes first past the cap.
pub(in crate::breakage) fn edge_look_cap(g: &mut crate::game::Game) {
    let most = g.m.edge_look.edge.max_rubble;
    while g.destr.edge_look.rubble.len() as f64 > most {
        let r = g.destr.edge_look.rubble.remove(0);
        if r.t < r.t_2 { g.destr.edge_look.flying -= 1.0; }
        g.stats.edge_look.capped += 1;
        crate::scene_boxes::box_let_go(g, r.box_);
    }
    g.stats.edge_look.rubble = g.destr.edge_look.rubble.len() as i32;
}

/// The level put back.
pub(in crate::breakage) fn edge_look_clear(g: &mut crate::game::Game) {
    let e = core::mem::replace(&mut g.destr.edge_look, EdgeLook::blank());
    for l in e.decor.values() { for &b in l { crate::scene_boxes::box_let_go(g, b); } }
    for r in e.rubble.iter() { crate::scene_boxes::box_let_go(g, r.box_); }
    // (`room` is not put back by the TypeScript's clear: kept)
    g.destr.edge_look.room = e.room;
    let s = &mut g.stats.edge_look; s.decor = 0; s.rubble = 0; s.flying = 0;
}

/// Once a frame: the frame's room, a wall dressed or a few cells shed, the fragments in flight moved.
pub(in crate::breakage) fn edge_look_update(g: &mut crate::game::Game, dt: f64) {
    g.destr.edge_look.room = frame_room(g, g.m.edge_look.edge.box_slack, None);
    if !g.destr.edge_look.held {
        if !g.destr.edge_look.dirty.is_empty() { edge_look_flush(g); }
        else if !g.destr.edge_look.todo.is_empty() {
            let n = core::cmp::max(1, js::to_i32(g.m.edge_look.edge.spread)) as usize;
            let n = n.min(g.destr.edge_look.todo.len());
            let take: Vec<EdgeLookTodoItem> = g.destr.edge_look.todo.drain(0..n).collect();
            for q in take.iter() { edge_look_shed(g, &q.b, q.f, q.c, q.dir, false, q.blast); }
            edge_look_cap(g);
        }
    }
    if g.destr.edge_look.flying == 0.0 { return; }
    let mut fl = 0.0;
    for i in 0..g.destr.edge_look.rubble.len() {
        let mut r = g.destr.edge_look.rubble[i];
        if r.t >= r.t_2 { continue; }
        r.t = m::min(r.t_2, r.t + dt);
        let s = r.t / r.t_2;
        let bx = &mut g.tab.frame_boxes[r.box_];
        bx.c[0] = r.p0[0] + (r.p1[0] - r.p0[0]) * s; bx.c[2] = r.p0[2] + (r.p1[2] - r.p0[2]) * s;
        bx.c[1] = if s >= 1.0 { r.p1[1] } else { m::max(r.p1[1], r.p0[1] + r.vy * r.t - 4.9 * r.t * r.t) };
        bx.rot = Some(if s >= 1.0 { r.q1 } else { quat::slerp(r.q0, r.q1, s * s) });
        if s < 1.0 { fl += 1.0; } else { let y = crate::debris::debris_rubble_height(g, r.box_.into(), r.p1[1]); g.tab.frame_boxes[r.box_].c[1] = y; }
        g.destr.edge_look.rubble[i].t = r.t;
    }
    g.destr.edge_look.flying = fl; g.stats.edge_look.flying = fl as i32;
}

/// Kicked rubble: the fragments at rest are shoved by the men's feet as the pool's pieces are, over records of this lane's own.
pub(in crate::breakage) fn edge_look_feet(g: &mut crate::game::Game, men: &[crate::debris::FootMan], k: crate::debris::FootKnobs, dt: f64) {
    if g.destr.edge_look.rubble.is_empty() { return; }
    let mut list: Vec<DebrisChunkId> = Vec::new();
    for i in 0..g.destr.edge_look.rubble.len() {
        let r = g.destr.edge_look.rubble[i];
        if r.t < r.t_2 { continue; }
        let foot = match r.foot {
            Some(f) => f,
            None => {
                let (c, mkey) = { let b = &g.tab.frame_boxes[r.box_]; (b.c, b.mkey) };
                let mut near = false;
                for mn in men { if m::abs(mn.pos[0] - c[0]) < 1.5 && m::abs(mn.pos[2] - c[2]) < 1.5 { near = true; break; } }
                if !near { continue; }
                // (its own height once a boot lays it flat: as shed the heap's flakes are staggered, so no two overlapping tops lie in one plane)
                let lift = 0.0007 * ((js::to_i32(mkey.unwrap_or(0.0)) & 0x7fffff) % 11) as f64;
                let f = crate::debris::debris_loose(g, r.box_.into(), Some(lift));
                g.destr.edge_look.rubble[i].foot = Some(f);
                f
            }
        };
        list.push(foot);
    }
    if list.is_empty() { return; }
    let mut book = g.destr.edge_look.foot_book;
    crate::debris::debris_feet(g, men, k, Some(&list), Some(&mut book));
    g.destr.edge_look.foot_book = book;
    let no_shadow = js::to_i32(crate::scene_boxes::box_flag().no_shadow);
    let frame = js::to_i32(m::floor(g.time * 60.0));
    for (i, &c) in list.iter().enumerate() {
        let slid = !g.tab.debris_chunk[c].rest;
        crate::debris::debris_foot_step(g, c, dt);
        if !slid && (frame + i as i32) & 31 == 0 { crate::debris::debris_resettle(g, c); }
        // a slab that casts a shadow and was slid to a tripwire's beam casts none from here on (shed()'s rule, kept true under the boots)
        let bx = g.tab.debris_chunk[c].box_;
        let (flags, bc, bh) = { let b = &g.tab[bx]; (js::to_i32(b.flags), b.c, b.h) };
        if slid && flags & no_shadow == 0 && edge_look_in_beam(g, bc, m::max(bh[0], bh[2])) { g.tab[bx].flags = (flags | no_shadow) as f64; }
    }
}

/// Every wall that opened since the last flush is dressed again.
pub(in crate::breakage) fn edge_look_flush(g: &mut crate::game::Game) {
    if g.destr.edge_look.dirty.is_empty() { return; }
    let all: Vec<BreakLike> = g.destr.edge_look.dirty.values().cloned().collect();
    for b in all.iter() { edge_look_rebuild(g, b); }
    g.destr.edge_look.dirty.clear();
    let mut n = 0; for l in g.destr.edge_look.decor.values() { n += l.len(); }
    g.stats.edge_look.decor = n as i32;
}

/// This lane's pieces into the frame's dynamic list (after the lane's debris).
pub(in crate::breakage) fn edge_look_boxes(g: &mut crate::game::Game, out: &mut Vec<AnyBoxId>) {
    if g.destr.edge_look.held { g.destr.edge_look.held = false; } else { edge_look_flush(g); }   // (a wall that opened this frame is dressed on the next: `dress`)
    let e = &g.destr.edge_look;
    let (room, mut n) = (e.room, 0.0);
    for l in e.decor.values() { for &b in l { if n >= room { break; } out.push(b.into()); n += 1.0; } }
    let mut i = e.rubble.len();
    while i > 0 && n < room { i -= 1; out.push(e.rubble[i].box_.into()); n += 1.0; }   // (newest first: what a short frame drops is the oldest litter)
}

// ---------------------------------------------------------------- the standing pieces of one wall

/// how far a point of a ragged edge may stand out, by its place along the edge (the TypeScript's `reach` closures)
#[derive(Clone, Copy)]
enum Reach { Flat(f64), Arch { lo: f64, hi: f64 } }
impl Reach { fn at(self, t: f64) -> f64 { match self { Reach::Flat(r) => r, Reach::Arch { lo, hi } => lo + (hi - lo) * m::pow(m::max(0.0, t - 0.3) / 0.7, 1.5) } } }

/// what `rebuild` holds while it lays one wall's pieces: the TypeScript's closures over these (H, push, tone, plate, ragged)
struct Lay {
    f: Frame, id: f64, fc: V3, f_yaw: f64, f_rough: Option<f64>,
    out: Vec<Bx>, cap: f64, capped: i32, key: i32,
    paint: V3, core: V3, bl: Option<Vec<[f64; 2]>>, rim: f64, real: bool,
    /// what the wall says it is, and whether a frame shows in a hole in it
    stuff: Option<crate::scene_stuff::BoxStuff>, does: Option<crate::scene_materials::DoesWord>, framed: bool,
}
impl Lay {
    fn h(&self, a: f64, b2: f64, c: f64) -> f64 {
        crate::debris::hash01((m::imul(js::to_i32(self.id + 1.0), 73856093) ^ m::imul(js::to_i32(a + 7.0), 19349663) ^ m::imul(js::to_i32(b2 + 3.0), 83492791) ^ m::imul(js::to_i32(c + 11.0), 2654435761u32 as i32)) as f64)
    }
    fn push(&mut self, b: Bx) { if (self.out.len() as f64) < self.cap { self.out.push(b); } else { self.capped += 1; } }
    /// a charge's gap: what stands round it is sooted, heaviest at the gap
    fn tone(&self, c: V3, u: f64) -> V3 {
        let Some(bl) = &self.bl else { return c };
        let mut x = 1.0;
        for &[uc, half] in bl.iter() { x = m::min(x, m::max(0.0, (m::abs(u - uc) - half * 0.5) / (half * 0.5 + (if js::truthy(self.rim) { self.rim } else { 0.9 })))); }
        if x >= 1.0 { c } else { v3::scale(c, 0.4 + 0.6 * x * x * (3.0 - 2.0 * x)) }
    }
    fn next_key(&mut self) -> i32 { let k = self.key; self.key = self.key.wrapping_add(1); k }
    /// a plate in the wall's plane: half extents a × hb × ht, turned `th` in the plane, centred (u, y), v across
    fn plate(&self, u: f64, y: f64, v: f64, a: f64, hb: f64, ht: f64, th: f64, albedo: V3, key: i32, solid: bool) -> Bx {
        let f = &self.f;
        let (c, s) = (m::cos(th), m::sin(th));
        let e1: V3 = [f.along[0] * c, s, f.along[2] * c]; let e2: V3 = [-f.along[0] * s, c, -f.along[2] * s]; let e3 = v3::cross(e1, e2);
        let fl = crate::scene_boxes::box_flag();
        crate::scene_boxes::make_box_value(&crate::scene_boxes::MakeBoxP {
            c: at_c(self.fc, f, u, y, v), h: [a, hb, ht], yaw: Some(self.f_yaw), rot: Some(quat::from_basis(e1, e2, e3)), albedo: Some(albedo),
            flags: Some((js::to_i32(fl.dynamic) | (if solid { 0 } else { js::to_i32(fl.no_shadow) })) as f64), name: Some(String::from("edge")),
            mkey: Some(MKEY_LOOK + (key & 0x7fffff) as f64), rough: self.f_rough, stuff: self.stuff, does: self.does, ..Default::default() })
    }
    /// A plate of what the wall is built on, which is not the wall: a wall that stands by itself shows none.
    fn frame(&mut self, u: f64, y: f64, v: f64, a: f64, hb: f64, ht: f64, th: f64, albedo: V3, key: i32) {
        if !self.framed { return; }
        let mut p = self.plate(u, y, v, a, hb, ht, th, albedo, key, false);
        p.stuff = None; p.does = None;
        self.push(p);
    }
    /// A RAGGED EDGE as one broken line: control points standing d out of the straight cut, each stretch between two of them one plate
    #[allow(clippy::too_many_arguments)]
    fn ragged(&mut self, ax: f64, ay: f64, bx: f64, by: f64, nx: f64, ny: f64, reach: Reach, bury: f64, past_a: f64, past_b: f64, tag: f64) {
        let len = m::hyp2(bx - ax, by - ay); if len < 0.12 { return; }
        let (ex, ey) = ((bx - ax) / len, (by - ay) / len);
        let t_wall = self.f.t;
        for coat in 0..2 {
            let cf = coat as f64; let two = coat == 1;
            let mut ts: Vec<f64> = vec![0.0]; let mut ds: Vec<f64> = Vec::new();
            let (st0, st1) = (if two { 0.2 } else { 0.24 }, if two { 0.32 } else { 0.34 });
            let mut t = 0.0;
            for i in 0..14 { let st = st0 + st1 * self.h(tag, i as f64, 30.0 + cf); if len - t - st < st0 * 0.7 { break; } t += st; ts.push(t); }
            ts.push(len);
            let par = if self.h(tag, 99.0, cf) < 0.5 { 0 } else { 1 };
            for j in 0..ts.len() {
                let r = m::min(reach.at(ts[j] / len), bury * 0.8) * (if two { 1.12 } else { 1.0 }) + (if two { 0.012 } else { 0.0 });
                let k = if (j + par) % 2 == 0 { 0.55 + 0.5 * self.h(tag, j as f64, 40.0 + cf) } else { 0.04 + 0.3 * self.h(tag, j as f64, 50.0 + cf) };
                ds.push(m::max(0.012, r * k));
            }
            // a plate's body leans past the end of its stretch that stands further out: by a hard end that lean has to fit in what is left of the edge
            for _pass in 0..2 { for j in 0..ts.len() - 1 {
                let dt = ts[j + 1] - ts[j]; let up = ds[j + 1] > ds[j]; let room = if up { len - ts[j + 1] + past_b } else { ts[j] + past_a };
                let (dmax, dmin) = (m::max(ds[j], ds[j + 1]), m::min(ds[j], ds[j + 1])); let most = room * dt / (dmax + 0.015);
                if dmax - dmin > most { if up { ds[j + 1] = dmin + most; } else { ds[j] = dmin + most; } }
            } }
            for j in 0..ts.len() - 1 {
                let dt = ts[j + 1] - ts[j]; let dd = ds[j + 1] - ds[j]; let ls = m::hyp2(dt, dd); let dmax = m::max(ds[j], ds[j + 1]);
                let (gx, gy, mx, my) = ((ex * dt + nx * dd) / ls, (ey * dt + ny * dd) / ls, -(nx * dt - ex * dd) / ls, -(ny * dt - ey * dd) / ls);   // along the stretch; into the wall
                let d = (dmax + 0.015) * ls / dt;
                let e0 = if j == 0 && past_a <= 0.0 { 0.0 } else { 0.012 }; let e1 = if j + 2 == ts.len() && past_b <= 0.0 { 0.0 } else { 0.012 };
                let cu = ax + ex * (ts[j] + dt / 2.0) + nx * (ds[j] + dd / 2.0) + gx * (e1 - e0) / 2.0 + mx * d / 2.0;
                let cy = ay + ey * (ts[j] + dt / 2.0) + ny * (ds[j] + dd / 2.0) + gy * (e1 - e0) / 2.0 + my * d / 2.0;
                let col = self.tone(if two { self.core } else { self.paint }, cu);
                let key = self.next_key();
                let b = self.plate(cu, cy, 0.0, (ls + e0 + e1) / 2.0, d / 2.0, (if two { t_wall * 0.5 } else { t_wall - 0.003 }) - 0.0007 * (j % 3) as f64, m::atan2(gy, gx), col, key, self.real && !two);
                self.push(b);
            }
        }
    }
}

/// The standing pieces of one wall, made again from its cells.
pub(in crate::breakage) fn edge_look_rebuild(g: &mut crate::game::Game, b: &crate::edge_look::BreakLike) {
    g.stats.edge_look.rebuilds += 1;
    let f = edge_look_frame(g, b);
    let (n, w, t) = (f.n, f.w, f.t);
    let e = g.m.edge_look.edge;
    let ni = n as i64;
    let open = |k: i64, iy: i64| -> bool { k >= 0 && k < ni && b.open.get(((iy as f64 * b.nz + (if f.along_x { 0.0 } else { k as f64 })) * b.nx + (if f.along_x { k as f64 } else { 0.0 })) as usize) == Some(&1) };
    let gone = |k: i64, iy: i64| -> bool { k < 0 || k >= ni || open(k, iy) };   // (nothing of THIS wall to bury a plate in: open, or past the wall's end)
    let edge_u = |k: i64| -> f64 { -f.l / 2.0 + w * k as f64 };   // the boundary on the low side of column k
    let (paint, fc, f_yaw, f_rough) = { let x = &g.tab.boxes[b.box_]; (x.albedo, x.c, x.yaw, x.rough) };
    let inside = crate::break_table::breaks_inside(g, &b.surf, b.box_);
    let core_c = v3::lerp3(paint, crate::break_table::core_of(inside, paint), 0.82);
    let (stuff, does) = { let x = &g.tab.boxes[b.box_]; (x.stuff, x.does) };
    let frame_c: V3 = if b.surf == "wood" { v3::scale(paint, 0.8) } else { steel() };   // what the wall is framed with
    let mut l = Lay { f, id: b.id, fc, f_yaw, f_rough, out: Vec::new(), cap: e.per_wall, capped: 0, key: (js::to_i32(b.id) & 0x3fff) << 9,
        paint, core: core_c, bl: g.destr.edge_look.blasted.get(crate::Num(b.id)).cloned(), rim: e.scorch.rim, real: true, stuff, does, framed: inside.framed };
    // standing wall of this level past column j going `s` (m, a little short of it; 0 = open there, or the wall's end)
    let past = |mut j: i64, s: i64, iy: i64| -> f64 { let mut c = 0.0; while !gone(j, iy) && c < 2.0 { c += 1.0; j += s; } c * w * 0.88 };
    // standing wall behind an edge at level iy, from column ke away from the opening (m, a little short of it)
    let behind = |ke: i64, s: i64, iy: i64| -> f64 { let mut c = 0.0; let mut j = ke; while j >= 0 && j < ni && !open(j, iy) && c < 2.0 { c += 1.0; j -= s; } m::min(0.85, c * w * 0.88) };
    let top = f.y_top - 0.004; let span = f.y_top - f.y_sill;
    // runs of open SILL cells: the lower sides of a hole, and a bent stub of the frame by each
    for k in 0..ni {
        if !open(k, 0) || open(k - 1, 0) { continue; }
        let mut k1 = k; while open(k1 + 1, 0) { k1 += 1; }
        let (ua, ub) = (edge_u(k), edge_u(k1 + 1)); let run_w = ub - ua;
        for (ke, ue, s) in [(k - 1, ua, 1i64), (k1 + 1, ub, -1i64)] {
            if ke < 0 || ke >= ni { continue; }
            let (kf, sf) = (ke as f64, s as f64);
            let r = m::min(e.hole_out * 1.5, run_w * e.run_k * 0.6);
            l.ragged(ue, f.y0, ue, f.y_sill, sf, 0.0, Reach::Flat(r), behind(ke, s, 0), 9.0, if open(ke, 1) { 0.0 } else { 0.9 }, kf * 8.0 + (if s > 0 { 3.0 } else { 4.0 }));
            if l.h(kf, 160.0, sf + 2.0) < 0.7 {
                let hgt = 0.16 + 0.3 * l.h(kf, 161.0, sf + 2.0); let th = sf * (0.25 + 0.5 * l.h(kf, 162.0, sf + 2.0)); let ub2 = ue + sf * (0.02 + 0.06 * l.h(kf, 163.0, sf + 2.0));
                let v = (l.h(kf, 164.0, 0.0) - 0.5) * t; let key = l.next_key();
                l.frame(ub2 + m::sin(th) * hgt / 2.0, m::max(f.y0, 0.0) + m::cos(th) * hgt / 2.0, v, 0.018, hgt / 2.0, 0.018, -th, frame_c, key);
            }
        }
    }
    // runs of open UPPER cells (a port, or the upper half of a hole)
    for k in 0..ni {
        if !open(k, 1) || open(k - 1, 1) { continue; }
        let mut k1 = k; while open(k1 + 1, 1) { k1 += 1; }
        let (ua, ub) = (edge_u(k), edge_u(k1 + 1)); let run_w = ub - ua;
        let mut holes = 0; for j in k..=k1 { if open(j, 0) { holes += 1; } }
        let is_hole = holes > 0; let wide = run_w * e.run_k; let kf = k as f64;
        // THE LINTEL: the top of the wall still spans the opening (a band of its sheet, torn along its underside). Under the ceiling only a deep lintel
        // reads from above: a bare rail up there is a dark bar
        let flush = span >= 1.3 && top >= crate::scene_world::world().ceiling_y - 0.03;
        let mut lin = if span >= 1.1 && (is_hole || flush || l.h(kf, 170.0, 0.0) < e.lintel_odds) { m::min(span * 0.3, (if is_hole { e.lintel_hole } else { e.lintel_port }) * (0.75 + 0.5 * l.h(kf, 171.0, 0.0))) } else { 0.0 };
        if flush { lin = m::max(lin, e.lintel_cut + 0.07 * l.h(kf, 172.0, 0.0)); }
        let y_l = f.y_top - lin; let has_lin = js::truthy(lin);
        if has_lin {
            for j in k..=k1 {
                let xa = if j == k && !gone(j - 1, 1) { 0.02 } else { 0.0 }; let xb = if j == k1 && !gone(j + 1, 1) { 0.02 } else { 0.0 }; let uc = edge_u(j) + w / 2.0;
                let col = l.tone(paint, uc); let key = l.next_key();
                let p = l.plate(uc + (xb - xa) / 2.0, (y_l + top) / 2.0, 0.0, (w + xa + xb) / 2.0, (top - y_l) / 2.0, t - 0.0035, 0.0, col, key, true);
                l.push(p);
            }
            let r = m::min(if is_hole { e.sill_hole } else { e.sill_port }, lin * 0.7);
            l.ragged(ua, y_l, ub, y_l, 0.0, -1.0, Reach::Flat(r), lin, past(k - 1, -1, 1), past(k1 + 1, 1, 1), kf * 8.0 + 6.0);
        }
        // the two sides (only against a standing column of this wall). A hole's upper corners are left hanging in: a breach with a ragged arch
        for (ke, ue, s) in [(k - 1, ua, 1i64), (k1 + 1, ub, -1i64)] {
            if ke < 0 || ke >= ni { continue; }
            let lo = m::min(if is_hole { e.hole_out } else { e.port_out * 0.7 }, wide); let hi = m::min(if is_hole { e.hole_top } else { e.port_out * 1.3 }, wide);
            l.ragged(ue, f.y_sill, ue, if has_lin { y_l + lin * 0.5 } else { top }, s as f64, 0.0, Reach::Arch { lo, hi }, behind(ke, s, 1), if open(ke, 0) { 0.0 } else { 0.6 }, 0.0, ke as f64 * 8.0 + (if s > 0 { 1.0 } else { 2.0 }));
        }
        // the torn tops of the sills left under it, run by run
        for j in k..=k1 {
            if open(j, 0) || (j > k && !open(j - 1, 0)) { continue; }
            let mut j1 = j; while j1 < k1 && !open(j1 + 1, 0) { j1 += 1; }
            let r = m::min(if is_hole { e.sill_hole } else { e.sill_port }, span * 0.3);
            l.ragged(edge_u(j), f.y_sill, edge_u(j1 + 1), f.y_sill, 0.0, 1.0, Reach::Flat(r), 0.6, past(j - 1, -1, 0), past(j1 + 1, 1, 0), j as f64 * 8.0 + 5.0);
        }
        // with no lintel, the top track: straight across a port, snapped and hanging from either side over a hole
        let ty = f.y_top - 0.022; let tt = m::min(t * 0.6, 0.022);
        if has_lin { /* (inside the lintel) */ }
        else if !is_hole {
            let xa = if k > 0 { 0.03 } else { 0.0 }; let xb = if k1 < ni - 1 { 0.03 } else { 0.0 };
            let col = l.tone(frame_c, (ua + ub) / 2.0); let key = l.next_key();
            l.frame((ua + ub + xb - xa) / 2.0, ty, 0.0, (run_w + xa + xb) / 2.0, 0.018, tt, 0.0, col, key);
        } else {
            for (ke, ue, s) in [(k - 1, ua, 1i64), (k1 + 1, ub, -1i64)] {
                if ke < 0 || ke >= ni { continue; }
                let (kef, sf) = (ke as f64, s as f64);
                let len = m::min(0.7, run_w * (0.2 + 0.2 * l.h(kef, 31.0, sf + 2.0))); let phi = 0.3 + 0.7 * l.h(kef, 32.0, sf + 2.0);
                let v = (l.h(kef, 33.0, 0.0) - 0.5) * t * 0.6; let col = l.tone(frame_c, ue); let key = l.next_key();
                l.frame(ue + sf * (len / 2.0) * m::cos(phi), ty - (len / 2.0) * m::sin(phi), v, len / 2.0, 0.016, tt, -sf * phi, col, key);
            }
        }
        // per column of a port: what still hangs from the top, a stud fire left standing
        for j in k..=k1 {
            if open(j, 0) { continue; }
            let u0 = edge_u(j); let jf = j as f64;
            if !is_hole && !has_lin {
                // (uneven lengths of board, square to the top so nothing shows above it)
                let mut u = u0 + 0.02 + 0.12 * l.h(jf, 39.0, 0.0); let mut i = 0;
                while u < u0 + w - 0.06 && i < 3 {
                    let fi = i as f64;
                    let ww = m::min(u0 + w - u, 0.09 + 0.17 * l.h(jf, 40.0 + fi, 0.0)); let d = 0.04 + 0.5 * m::pow(l.h(jf, 50.0 + fi, 0.0), 2.2);
                    let whole = l.h(jf, 70.0 + fi, 0.0) < 0.65; let key = l.next_key();
                    let p = l.plate(u + ww / 2.0, top - d / 2.0, 0.0, ww / 2.0, d / 2.0, (if whole { t - 0.004 } else { t * 0.5 }) - 0.0007 * fi, 0.0, if whole { paint } else { core_c }, key, false);
                    l.push(p);
                    u += ww * (0.9 + 1.3 * l.h(jf, 80.0 + fi, 0.0));
                    i += 1;
                }
            }
            if l.h(jf, 150.0, 0.0) < (if is_hole { 0.3 } else { 0.55 }) {   // a stud: whole, or snapped with its top hanging
                let us = u0 + w * (0.25 + 0.5 * l.h(jf, 151.0, 0.0)); let snap = is_hole || l.h(jf, 152.0, 0.0) < 0.5; let hs = 0.018;
                if !snap {
                    let th = (l.h(jf, 153.0, 0.0) - 0.5) * 0.06; let key = l.next_key();
                    l.frame(us, (f.y_sill + f.y_top) / 2.0, 0.0, hs, span / 2.0, hs, th, frame_c, key);
                } else {
                    let lo = span * (0.2 + 0.25 * l.h(jf, 154.0, 0.0)); let hi = span * (0.1 + 0.16 * l.h(jf, 155.0, 0.0));
                    let t1 = (l.h(jf, 156.0, 0.0) - 0.5) * 0.5; let t2 = (l.h(jf, 157.0, 0.0) - 0.5) * 0.6;
                    let key = l.next_key();
                    l.frame(us - m::sin(t1) * lo / 2.0, f.y_sill + m::cos(t1) * lo / 2.0 - 0.01, 0.0, hs, lo / 2.0, hs, t1, frame_c, key);
                    if !is_hole {
                        let key = l.next_key();
                        l.frame(us + m::sin(t2) * hi / 2.0, y_l - m::cos(t2) * hi / 2.0 + (if has_lin { 0.02 } else { -0.03 }), 0.0, hs, hi / 2.0, hs, t2, frame_c, key);
                    }
                }
            }
        }
    }
    g.stats.edge_look.capped += l.capped;
    // (re-set = the newest: over the level's cap it is the wall dressed longest ago that goes bare)
    if let Some(old) = g.destr.edge_look.decor.delete(crate::Num(b.id)) { for x in old { crate::scene_boxes::box_let_go(g, x); } }
    if !l.out.is_empty() {
        let rows: Vec<FrameBoxId> = l.out.into_iter().map(|x| crate::scene_boxes::box_kept(g, x)).collect();
        g.destr.edge_look.decor.set(crate::Num(b.id), rows);
    }
    let mut total = 0.0; for x in g.destr.edge_look.decor.values() { total += x.len() as f64; }
    let mut at = 0;
    while let Some(id) = g.destr.edge_look.decor.next_key(&mut at) {
        if total <= e.max_decor { break; }
        if id.0 == b.id { continue; }
        let gone_l = g.destr.edge_look.decor.delete(id).unwrap_or_default();
        total -= gone_l.len() as f64; g.stats.edge_look.capped += gone_l.len() as i32;
        for x in gone_l { crate::scene_boxes::box_let_go(g, x); }
    }
}

// ---------------------------------------------------------------- rubble

/// Does a fragment at `p` (reach `r`) lie in a tripwire's beam? A look-only slab in a beam's line casts no shadow.
pub(in crate::breakage) fn edge_look_in_beam(g: &crate::game::Game, p: V3, r: f64) -> bool {
    for &l in g.devices.lasers.iter() {
        let lz = &g.tab.devices_laser_rt[l]; let (f, d) = (lz.def.from, lz.dir);
        let t = m::max(0.0, m::min(lz.len, (p[0] - f[0]) * d[0] + (p[1] - f[1]) * d[1] + (p[2] - f[2]) * d[2]));
        if m::hyp3(p[0] - f[0] - d[0] * t, p[1] - f[1] - d[1] * t, p[2] - f[2] - d[2] * t) < r + 0.12 { return true; }
    }
    false
}

/// What one opened cell leaves at the foot: fragments at rest positions hashed off the cell, flying there from the cell unless quiet.
pub(in crate::breakage) fn edge_look_shed(g: &mut crate::game::Game, b: &crate::edge_look::BreakLike, f: Option<crate::edge_look::Frame>, c_2: f64, dir: Option<V3>, quiet: bool, blast: bool) {
    let c = c_2;
    let (bc, bh, byaw, paint, stuff, does) = { let x = &g.tab.boxes[b.box_]; (x.c, x.h, x.yaw, x.albedo, x.stuff, x.does) };
    let inside = crate::break_table::breaks_inside(g, &b.surf, b.box_);
    let core_c = v3::lerp3(paint, crate::break_table::core_of(inside, paint), 0.82);
    let facing = inside.facing.unwrap_or(core_c);
    let (ix, iz, iy) = (js::rem(c, b.nx), js::rem(m::floor(c / b.nx), b.nz), m::floor(c / (b.nx * b.nz)));
    let id = b.id;
    let hh = |a: f64, k: f64| -> f64 {
        crate::debris::hash01((m::imul(js::to_i32(id + 1.0), 73856093) ^ m::imul(js::to_i32(c + 5.0), 19349663) ^ m::imul(js::to_i32(a + 3.0), 83492791) ^ m::imul(js::to_i32(k + 17.0), 2654435761u32 as i32)) as f64)
    };
    let e = g.m.edge_look.edge;
    let floor = m::max(0.0, bc[1] - bh[1]);
    let (n, centre, half, nrm, along, y_lo, y_hi): (f64, V3, f64, V3, V3, f64, f64);
    if let Some(f) = f {
        let k = if f.along_x { ix } else { iz }; let u = -f.l / 2.0 + f.w * (k + 0.5);
        n = m::round((if iy != 0.0 { e.per_upper } else { e.per_sill }) * (if blast { e.blast.more } else { 1.0 }));
        centre = edge_look_at(g, f, u, 0.0, 0.0); half = f.w / 2.0; nrm = f.nrm; along = f.along;
        y_lo = if iy != 0.0 { f.y_sill } else { f.y0 }; y_hi = if iy != 0.0 { f.y_top } else { f.y_sill };
    } else {   // cover: a plan cell, top to bottom; thrown the way the round went
        let (cs, sn) = (m::cos(byaw), m::sin(byaw));
        let lx = -bh[0] + 2.0 * bh[0] * (ix + 0.5) / b.nx; let lz = -bh[2] + 2.0 * bh[2] * (iz + 0.5) / b.nz;
        n = e.per_cover; centre = [bc[0] + cs * lx + sn * lz, 0.0, bc[2] - sn * lx + cs * lz]; half = m::min(bh[0] / b.nx, bh[2] / b.nz);
        let d: V3 = match dir { Some(d) if m::hyp2(d[0], d[2]) > 0.05 => v3::normalize([d[0], 0.0, d[2]]), _ => [m::cos(hh(1.0, 0.0) * 6.283), 0.0, m::sin(hh(1.0, 0.0) * 6.283)] };
        nrm = d; along = [-d[2], 0.0, d[0]]; y_lo = bc[1] - bh[1]; y_hi = bc[1] + bh[1];
    }
    let far = match dir { Some(d) => if v3::dot(d, nrm) >= 0.0 { 1.0 } else { -1.0 }, None => if hh(2.0, 0.0) < 0.5 { 1.0 } else { -1.0 } };
    let wood = b.surf == "wood"; let soft = b.surf == "cardboard" || b.surf == "fabric";
    let reach = if blast { e.throw_blast } else { e.throw_fire };
    let sill_up = f.is_some() && iy == 1.0 && b.open.get((c - b.nx * b.nz) as usize) == Some(&0);
    let fl = crate::scene_boxes::box_flag();
    let no_shadow = js::to_i32(fl.no_shadow);
    let ch = e.cut_heap;
    let mut i = 0.0;
    while i < n {
        let side = match f { Some(_) => if hh(10.0, i) < (if blast { 0.6 } else { 0.78 }) { far } else { -far }, None => 1.0 };
        let flung = blast && hh(28.0, i) < e.blast.flung;   // (a few go well into the room; the rest heap at the wall's foot)
        let mut d = (match f { Some(f) => f.t, None => half }) + 0.04
            + (if flung { reach * (1.2 + 1.6 * hh(11.0, i)) } else { reach * m::pow(hh(11.0, i), if blast { 2.6 } else { 2.0 }) }) * (if side == far { 1.0 } else if blast { e.blast.near_k } else { 0.45 });
        let u = (hh(12.0, i) - 0.5) * (2.0 * half + (if blast { 0.5 } else { 0.25 }));
        let rd = v3::scale(nrm, side);
        let off = if sill_up { f.unwrap().t + 0.02 } else { 0.0 };   // (over a sill still standing the bits lie at its foot, either side: not inside it)
        let from: V3 = [centre[0] + along[0] * u + rd[0] * off, floor + 0.12, centre[2] + along[2] * u + rd[2] * off];
        d = m::max(0.03, d - off);
        // (a READ: rubble stops short of whatever stands there)
        if let Some(hit) = crate::scene_collision::raycast(g, from, rd, d + 0.14) { d = m::max(0.03, hit.t - 0.14) * (0.5 + 0.5 * hh(13.0, i)); }
        // the fragment: a board flake lying nearly flat (a splinter off wood), now and then a lump
        let lump = hh(14.0, i) < (if blast { e.blast.lumps } else { 0.34 });
        let mut hx = 0.022 + 0.085 * hh(15.0, i) * hh(15.0, i); let mut hz = 0.018 + 0.055 * hh(16.0, i);
        let mut hy = match f { Some(f) => m::min(0.011, m::max(0.005, f.t * 0.18)), None => 0.008 };
        if wood { hx = 0.06 + 0.12 * hh(15.0, i); hz = 0.008 + 0.012 * hh(16.0, i); hy = 0.006; }
        else if lump {
            hx = (0.018 + 0.03 * hh(15.0, i)) * (if blast { 2.5 * (if hh(29.0, i) < e.blast.slabs { e.blast.slab_k } else { 1.0 }) } else { 1.0 });
            hz = hx * (0.6 + 0.5 * hh(16.0, i)); hy = hx * (0.5 + 0.4 * hh(17.0, i));
            if let (true, Some(f)) = (blast, f) { hy = m::min(hy, m::max(f.t, 0.04)); }   // (a blast's chunk is a piece of the wall: never thicker than it)
        }
        else if blast { hy *= e.blast.flake_k; }
        // THE CUT SWITCH, a blast's heap off a wall: a few big SLABS of the wall's own sheet lying flat, and of the chunks a good share much smaller
        let mut slab = false;
        if let (true, Some(f), false, false) = (blast, f, wood, soft) {
            let cls = hh(33.0, i);
            if cls < ch.slabs { slab = true; hx = ch.slab[0] + (ch.slab[1] - ch.slab[0]) * hh(15.0, i); hz = hx * (0.45 + 0.5 * hh(16.0, i)); hy = m::min(ch.slab_t, m::max(0.012, f.t * 0.45)); }
            else if lump && cls > 1.0 - ch.smalls { let k = ch.small[0] + (ch.small[1] - ch.small[0]) * hh(34.0, i); hx *= k; hz *= k; hy *= k; }
        }
        if blast && !lump && hh(17.0, i) < 0.2 { hx *= 1.5; hz *= 1.4; }
        let tilt = if slab { (hh(18.0, i) - 0.5) * 0.16 } else if lump || d > 0.5 { (hh(18.0, i) - 0.5) * 0.3 } else { (hh(18.0, i) - 0.5) * 0.9 * (1.0 - d / 0.5) };
        let yaw_q = quat::yaw(hh(19.0, i) * 6.283);
        let q1 = quat::normalize(quat::mul(yaw_q, quat::axis_angle([1.0, 0.0, 0.0], tilt)));
        let (px, pz) = (from[0] + rd[0] * d, from[2] + rd[2] * d);
        // (a READ: the floor it lands on: cover on legs sheds onto the ground beside it)
        let ground = match crate::scene_collision::raycast(g, [px, floor + 0.3, pz], [0.0, -1.0, 0.0], floor + 0.35) { Some(dn) => floor + 0.3 - dn.t, None => 0.0 };
        let serial = g.destr.edge_look.serial;
        let y1 = ground + hy + m::abs(m::sin(tilt)) * hz + 0.002 + 0.0012 * js::rem(serial + i, 7.0);
        let mut p1: V3 = [px, y1, pz];
        let tone = hh(20.0, i);
        let mut col: V3 = match f {
            Some(_) => if tone < 0.45 { paint } else if tone < 0.8 { core_c } else if tone < 0.92 { facing } else { v3::scale(paint, 0.55) },
            None => v3::scale(paint, if soft { 0.75 + 0.3 * tone } else { 0.7 + 0.45 * tone }),
        };
        let soot = if blast { if d < 0.7 { 0.4 + 0.5 * hh(21.0, i) } else { 0.75 + 0.25 * hh(21.0, i) } } else { 1.0 };
        if blast { col = v3::scale(col, soot); }
        if blast && f.is_some() { col = v3::scale(col, 1.0 - ch.tone * hh(35.0, i)); }   // (each piece its own tone inside the material)
        g.destr.edge_look.serial = serial + 1.0;
        let bv = crate::scene_boxes::make_box_value(&crate::scene_boxes::MakeBoxP { c: p1, h: [hx, hy, hz], rot: Some(q1), albedo: Some(col), stuff, does,
            flags: Some((js::to_i32(fl.dynamic) | no_shadow) as f64), name: Some(String::from("rubble")),
            mkey: Some(MKEY_LOOK + 8388608.0 + (js::to_i32(serial) & 0x7fffff) as f64), ..Default::default() });
        let bx = crate::scene_boxes::box_kept(g, bv);
        // the cut switch: its outline by its material's row; a big piece casts its shadow
        if crate::break_table::breaks_shape_loose(g, bx.into(), &b.surf, (id * 4096.0 + c) * 32.0 + i) {
            p1 = g.tab.frame_boxes[bx].c;
            if m::max(hx, hz) >= 0.15 && !edge_look_in_beam(g, p1, m::max(hx, hz)) { let x = &mut g.tab.frame_boxes[bx]; x.flags = (js::to_i32(x.flags) & !no_shadow) as f64; }
            // its broken faces: the material's core, sooted with the piece, a little of the paper's brown in some, no two the same white
            if let (true, true, Some(cc)) = (blast, f.is_some(), g.tab.frame_boxes[bx].cut_core) {
                g.tab.frame_boxes[bx].cut_core = Some(v3::scale(v3::lerp3(cc, inside.facing.unwrap_or(cc), ch.core_paper * hh(36.0, i)), soot * (1.0 - ch.core_tone * hh(37.0, i))));
            }
        }
        let mut r = Rubble { box_: bx, t: 1.0, t_2: 1.0, p0: p1, p1, vy: 0.0, q0: q1, q1, foot: None };
        if !quiet {   // from the cell that went, down a short arc: more of a shove from a blast
            let y_s = y_lo + (y_hi - y_lo) * (0.15 + 0.75 * hh(22.0, i)); let u_s = u * 0.6;
            r.p0 = [centre[0] + along[0] * u_s, y_s, centre[2] + along[2] * u_s];
            let up = (if blast { 1.6 } else { 0.5 }) * hh(23.0, i);
            r.t_2 = (up + m::sqrt(up * up + 19.6 * m::max(0.02, y_s - y1))) / 9.8 + 0.03 * i * (if blast { 0.3 } else { 1.0 });
            r.vy = (y1 - y_s + 4.9 * r.t_2 * r.t_2) / r.t_2; r.t = 0.0;
            r.q0 = quat::normalize(quat::mul(quat::axis_angle(v3::normalize([hh(24.0, i) - 0.5, hh(25.0, i) - 0.5, hh(26.0, i) - 0.5]), 2.0 + 5.0 * hh(27.0, i)), q1));
            let x = &mut g.tab.frame_boxes[bx]; x.c = r.p0; x.rot = Some(r.q0);
            g.destr.edge_look.flying += 1.0;
        }
        g.destr.edge_look.rubble.push(r); g.stats.edge_look.shed += 1;
        i += 1.0;
    }
}

/// One soot mark, the picture's alone.
fn soot(g: &mut crate::game::Game, pos: V3, normal: V3, size: f64, param: f64) {
    let opts = crate::render_decals::DecalOpts { param: Some(param), look: Some(true), ..Default::default() };
    crate::render_decals::decals_add(g, crate::render_decals::DecalKind::Scorch, pos, normal, Some(size), Some(opts));
}

/// Soot round a blast's gap: where along the wall the charge opened it (the edge pieces there are sooted), then both faces of the wall about the opening, the floor at its foot, the rim of what still stands.
pub(in crate::breakage) fn edge_look_scorch(g: &mut crate::game::Game, b: &crate::edge_look::BreakLike, f: crate::edge_look::Frame, fresh: &[f64], dir: Option<V3>) {
    let (mut k0, mut k1) = (1e9_f64, -1.0_f64);
    let per = b.nx * b.nz;
    for &c in fresh {
        if c >= per { continue; }
        let k = if f.along_x { js::rem(c, b.nx) } else { js::rem(m::floor(c / b.nx), b.nz) };
        k0 = m::min(k0, k); k1 = m::max(k1, k);
    }
    if k1 < 0.0 { return; }
    let (ua, ub) = (-f.l / 2.0 + f.w * k0, -f.l / 2.0 + f.w * (k1 + 1.0));
    let (uc, half) = ((ua + ub) / 2.0, (ub - ua) / 2.0);
    let s = g.m.edge_look.edge.scorch;
    match g.destr.edge_look.blasted.get_mut(crate::Num(b.id)) { Some(bl) => bl.push([uc, half]), None => { g.destr.edge_look.blasted.set(crate::Num(b.id), vec![[uc, half]]); } }
    let dir = match dir { Some(d) if g.m.edge_look.edge.scorch_decals && g.decals.enabled => d, _ => return };
    let far = if v3::dot(dir, f.nrm) >= 0.0 { 1.0 } else { -1.0 };
    let floor = m::max(0.0, f.y0);
    let yc = floor + m::min(1.05, (f.y_top - floor) * 0.45);
    let p = edge_look_at(g, f, uc, yc, -far * f.t);
    soot(g, p, v3::scale(f.nrm, -far), half + s.near, s.soot_near);
    let p = edge_look_at(g, f, uc, yc, far * f.t);
    soot(g, p, v3::scale(f.nrm, far), half + s.far, s.soot_far);
    let p = edge_look_at(g, f, uc, floor, -far * (f.t + 0.4));
    soot(g, p, up(), half * 0.5 + s.floor, s.soot_near * s.soot_floor);
    let p = edge_look_at(g, f, uc, floor, far * (f.t + 0.35));
    soot(g, p, up(), half * 0.4 + s.floor * 0.7, s.soot_far * s.soot_floor);
    g.stats.edge_look.scorch += 4;
    // the rim: marks centred ON the cut, the uppers' run and the sills' measured apart
    if s.rim <= 0.0 { return; }
    let gone = |k: f64, iy: f64| k >= 0.0 && k < f.n
        && b.open.get(((iy * b.nz + (if f.along_x { 0.0 } else { k })) * b.nx + (if f.along_x { k } else { 0.0 })) as usize) == Some(&1);
    let run = |iy: f64| { let (mut a, mut e) = (k0, k1); while gone(a - 1.0, iy) { a -= 1.0; } while gone(e + 1.0, iy) { e += 1.0; } (a, e) };
    let cut = |k: f64| -f.l / 2.0 + f.w * k;
    let mut spots: Vec<(f64, f64)> = Vec::new();
    let (sill, up_h) = (f.y_sill - floor, f.y_top - f.y_sill);
    let mut ends = |(a, e): (f64, f64), ys: &[f64]| {
        if a > 0.0 { for &y in ys { spots.push((cut(a), y)); } }
        if e < f.n - 1.0 { for &y in ys { spots.push((cut(e + 1.0), y)); } }
    };
    ends(run(0.0), &[floor + sill * 0.6]);
    if b.ny > 1.0 && gone(k0, 1.0) {
        if up_h >= 1.8 { ends(run(1.0), &[f.y_sill + up_h * 0.3, f.y_sill + up_h * 0.8]); } else { ends(run(1.0), &[f.y_sill + up_h * 0.5]); }
    }
    for (i, &(u, y)) in spots.iter().enumerate() {
        for side in [-far, far] {
            let h = crate::debris::hash01((m::imul(js::to_i32(b.id + 1.0), 73856093)
                ^ m::imul(js::to_i32(i as f64 * 2.0 + (if side > 0.0 { 1.0 } else { 0.0 }) + 31.0), 19349663)
                ^ m::imul(js::to_i32(k0 + 3.0), 83492791)) as f64);
            let p = edge_look_at(g, f, u, y, side * f.t);
            soot(g, p, v3::scale(f.nrm, side), s.rim * (0.85 + 0.3 * h), (if side == far { s.soot_far } else { s.soot_near }) * s.soot_rim);
        }
    }
}

/// The look's books, all empty (the TypeScript's field initialisers; its constructor keeps only `g`, here reached by path).
pub(in crate::breakage) fn edge_look_new(g: &crate::game::Game) -> crate::edge_look::EdgeLook { EdgeLook::blank() }
