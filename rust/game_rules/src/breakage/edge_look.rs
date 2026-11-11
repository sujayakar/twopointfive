#![allow(unused_imports, unused_variables)]
//! A BROKEN WALL'S LOOK, whole: the standing pieces round a cut, the rubble at its foot, the soot. Every piece is a box keyed
//! MKEY.look on the frame's list, which no ray of the game meets; but the fog of war keeps a record of every keyed loose thing of that list and caps its
//! records, the boots scuff the rubble at rest, and the frame's room is budgeted with these boxes in it: so the tick does read what this lane writes, and
//! the lane runs here.
//!
//! WHAT TO KNOW:
//! - A BREAKABLE IS HELD BY VALUE. The lane's `dirty` map and `todo` list hold `BreakLike` copies of the wallbreak's Breakable, and a wall is dressed a frame
//!   later from its cells as they then stand, so every copy this lane holds of a breakable is written again
//!   whenever cells of that breakable open (`opened`, the lane's one hook, is every way a cell opens).
//! - A PIECE'S BOX is a row its holder keeps (`scene_boxes::box_kept`) and lets go when the piece is dropped (a wall dressed again, a cap, a reset).
//! - A fault here is a panic with its place, not a look switched off.

pub use ::game_types::edge_look::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

use crate::v3::quat;

mod heap;
mod standing;
use heap::edge_look_shed;
use standing::edge_look_rebuild;

type Bx = crate::scene_boxes::Box;

/// what this lane reads of a breakable, as it stands now
pub(super) fn break_like(g: &Game, b: WallbreakBreakableId) -> BreakLike {
    let r = &g.tab.wallbreak_breakable[b];
    BreakLike { id: r.id, kind: r.kind, surf: String::from(r.surf.as_str()), box_: r.box_, nx: r.nx, ny: r.ny, nz: r.nz, yb: r.yb.clone(), open: r.open.clone() }
}

/// What the frame's box list can take of a picture-only lane's boxes: the table, less the level's boxes, less what the frame keeps for its own (`FRAME_BOXES`, or
/// `now`: what a lane has seen on the list as it stands), less `slack`.
pub(super) fn frame_room(g: &crate::game::Game, slack: f64, now: Option<f64>) -> f64 {
    let w = &g.engine.world;
    w.max_boxes - w.statics.len() as f64 - now.unwrap_or(crate::scene_boxes::FRAME_BOXES) - slack
}

/// Cells of a wall just opened: the look's one hook.
pub(super) fn edge_look_opened(g: &mut crate::game::Game, b: &crate::edge_look::BreakLike, fresh: &[f64], cause: &str, dir: Option<V3>, quiet: bool) {
    // (the copies this lane already holds of this breakable read its cells as they now stand)
    { let e = &mut g.destr.edge_look;
      if let Some(d) = e.dirty.get_mut(crate::Num(b.id)) { d.open = b.open.clone(); }
      for q in e.todo.iter_mut() { if q.b.id == b.id && q.b.kind == b.kind { q.b.open = b.open.clone(); } } }
    if !g.m.edge_look.edge.on { return; }
    edge_look_dress(g, b, fresh, cause, dir, quiet);
}

/// The frame a wall goes: only the soot's record and its marks land on it; the wall is dressed on the next frame and its rubble shed a few cells a frame
/// after that. A checkpoint being applied does it all at once.
fn edge_look_dress(g: &mut crate::game::Game, b: &crate::edge_look::BreakLike, fresh: &[f64], cause: &str, dir: Option<V3>, quiet: bool) {
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
fn edge_look_frame(g: &crate::game::Game, b: &crate::edge_look::BreakLike) -> crate::edge_look::Frame {
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

/// What a breakable's broken faces are painted from: its own paint, its core's and what it is made of.
struct Painted {
    paint: V3,
    core: V3,
    inside: crate::break_table::Inside,
    stuff: Option<crate::scene_stuff::BoxStuff>,
    does: Option<crate::scene_materials::DoesWord>,
}

fn painted(g: &Game, b: &BreakLike) -> Painted {
    let inside = crate::break_table::breaks_inside(g, &b.surf, b.box_);
    let x = &g.tab.boxes[b.box_];
    Painted { paint: x.albedo, core: v3::lerp3(x.albedo, crate::break_table::core_of(inside, x.albedo), 0.82), inside, stuff: x.stuff, does: x.does }
}

/// 0‥1 off four whole numbers (a hash, not a die).
fn look_hash(w: [f64; 4]) -> f64 {
    let word = |i: usize, by: i32| m::imul(js::to_i32(w[i]), by);
    crate::debris::hash01((word(0, 73856093) ^ word(1, 19349663) ^ word(2, 83492791) ^ word(3, 2654435761u32 as i32)) as f64)
}

/// A place in the wall's frame.
fn edge_look_at(g: &crate::game::Game, f: crate::edge_look::Frame, u: f64, y: f64, v: f64) -> V3 {
    let c = g.tab.boxes[f.b].c;
    at_c(c, &f, u, y, v)
}
fn at_c(c: V3, f: &Frame, u: f64, y: f64, v: f64) -> V3 { [c[0] + f.along[0] * u + f.nrm[0] * v, y, c[2] + f.along[2] * u + f.nrm[2] * v] }

/// The oldest litter goes first past the cap.
fn edge_look_cap(g: &mut crate::game::Game) {
    let most = g.m.edge_look.edge.max_rubble;
    while g.destr.edge_look.rubble.len() as f64 > most {
        let r = g.destr.edge_look.rubble.remove(0);
        if r.t < r.t_2 { g.destr.edge_look.flying -= 1.0; }
        crate::scene_boxes::box_let_go(g, r.box_);
    }
}

/// The level put back.
pub(super) fn edge_look_clear(g: &mut crate::game::Game) {
    let e = core::mem::replace(&mut g.destr.edge_look, EdgeLook::blank());
    for l in e.decor.values() { for &b in l { crate::scene_boxes::box_let_go(g, b); } }
    for r in e.rubble.iter() { crate::scene_boxes::box_let_go(g, r.box_); }
    // (`room` is not put back by the clear: kept)
    g.destr.edge_look.room = e.room;
}

/// Once a frame: the frame's room, a wall dressed or a few cells shed, the fragments in flight moved.
pub(super) fn edge_look_update(g: &mut crate::game::Game, dt: f64) {
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
    g.destr.edge_look.flying = fl; 
}

/// Kicked rubble: the fragments at rest are shoved by the men's feet as the pool's pieces are, over records of this lane's own.
pub(super) fn edge_look_feet(g: &mut crate::game::Game, men: &[crate::debris::FootMan], k: crate::debris::FootKnobs, dt: f64) {
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
        if slid && flags & no_shadow == 0 && edge_look_in_beam(g, bc, m::max(bh[0], bh[2])) { g.tab.edit(bx).flags = (flags | no_shadow) as f64; }
    }
}

/// Every wall that opened since the last flush is dressed again.
fn edge_look_flush(g: &mut crate::game::Game) {
    if g.destr.edge_look.dirty.is_empty() { return; }
    let all: Vec<BreakLike> = g.destr.edge_look.dirty.values().cloned().collect();
    for b in all.iter() { edge_look_rebuild(g, b); }
    g.destr.edge_look.dirty.clear();
}

/// This lane's pieces into the frame's dynamic list (after the lane's debris).
pub(super) fn edge_look_boxes(g: &mut crate::game::Game, out: &mut Vec<AnyBoxId>) {
    if g.destr.edge_look.held { g.destr.edge_look.held = false; } else { edge_look_flush(g); }   // (a wall that opened this frame is dressed on the next: `dress`)
    let e = &g.destr.edge_look;
    let (room, mut n) = (e.room, 0.0);
    for l in e.decor.values() { for &b in l { if n >= room { break; } out.push(b.into()); n += 1.0; } }
    let mut i = e.rubble.len();
    while i > 0 && n < room { i -= 1; out.push(e.rubble[i].box_.into()); n += 1.0; }   // (newest first: what a short frame drops is the oldest litter)
}

// ---------------------------------------------------------------- rubble

/// Does a fragment at `p` (reach `r`) lie in a tripwire's beam? A look-only slab in a beam's line casts no shadow.
fn edge_look_in_beam(g: &crate::game::Game, p: V3, r: f64) -> bool {
    for &l in g.devices.lasers.iter() {
        let lz = &g.tab.devices_laser_rt[l]; let (f, d) = (lz.def.from, lz.dir);
        let t = m::max(0.0, m::min(lz.len, (p[0] - f[0]) * d[0] + (p[1] - f[1]) * d[1] + (p[2] - f[2]) * d[2]));
        if m::hyp3(p[0] - f[0] - d[0] * t, p[1] - f[1] - d[1] * t, p[2] - f[2] - d[2] * t) < r + 0.12 { return true; }
    }
    false
}

/// One soot mark, the picture's alone.
fn soot(g: &mut crate::game::Game, pos: V3, normal: V3, size: f64, param: f64) {
    let opts = crate::render_decals::DecalOpts { param: Some(param), look: Some(true), ..Default::default() };
    crate::render_decals::decals_add(g, crate::render_decals::DecalKind::Scorch, pos, normal, Some(size), Some(opts));
}

/// Soot round a blast's gap: where along the wall the charge opened it (the edge pieces there are sooted), then both faces of the wall about the opening, the floor at its foot, the rim of what still stands.
fn edge_look_scorch(g: &mut crate::game::Game, b: &crate::edge_look::BreakLike, f: crate::edge_look::Frame, fresh: &[f64], dir: Option<V3>) {
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
    let dir = match dir { Some(d) if g.m.edge_look.edge.scorch_decals => d, _ => return };
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
    // the rim: marks centred ON the cut, the uppers' run and the sills' measured apart
    if s.rim <= 0.0 { return; }
    let cells = standing::Cells::of(f, b);
    let (k0, k1) = (k0 as i64, k1 as i64);
    let run = |iy: i64| { let (mut a, mut e) = (k0, k1); while cells.open(a - 1, iy) { a -= 1; } while cells.open(e + 1, iy) { e += 1; } (a, e) };
    let mut spots: Vec<(f64, f64)> = Vec::new();
    let (sill, up_h) = (f.y_sill - floor, f.y_top - f.y_sill);
    let mut ends = |(a, e): (i64, i64), ys: &[f64]| {
        if a > 0 { for &y in ys { spots.push((cells.edge_u(a), y)); } }
        if e < cells.n - 1 { for &y in ys { spots.push((cells.edge_u(e + 1), y)); } }
    };
    ends(run(0), &[floor + sill * 0.6]);
    if b.ny > 1.0 && cells.open(k0, 1) {
        if up_h >= 1.8 { ends(run(1), &[f.y_sill + up_h * 0.3, f.y_sill + up_h * 0.8]); } else { ends(run(1), &[f.y_sill + up_h * 0.5]); }
    }
    for (i, &(u, y)) in spots.iter().enumerate() {
        for side in [-far, far] {
            let h = look_hash([b.id + 1.0, i as f64 * 2.0 + (if side > 0.0 { 1.0 } else { 0.0 }) + 31.0, k0 as f64 + 3.0, 0.0]);
            let p = edge_look_at(g, f, u, y, side * f.t);
            soot(g, p, v3::scale(f.nrm, side), s.rim * (0.85 + 0.3 * h), (if side == far { s.soot_far } else { s.soot_near }) * s.soot_rim);
        }
    }
}

/// The look's books, all empty.
pub(super) fn edge_look_new() -> crate::edge_look::EdgeLook { EdgeLook::blank() }
