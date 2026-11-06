//! THE BRING-UP'S OWN PART of edge_look: bodies for skeleton stubs the first tick reached (bringup-log.md says which is a boundary no-op and which is translated)
#![allow(unused_imports, unused_variables)]
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// Cells of a wall just opened: the look's one hook. Of it only the soot's marks tell the host. (src/game/edgeLook.ts:104 EdgeLook.opened)
pub fn edge_look_opened(g: &mut crate::game::Game, b: &crate::edge_look::BreakLike, fresh: &[f64], cause: &str, dir: Option<V3>, quiet: bool) {
    if !g.m.edge_look.edge.on { return; }
    edge_look_dress(g, b, fresh, cause, dir, quiet);
}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/edgeLook.ts:98 EdgeLook.fail)
pub fn edge_look_fail(g: &mut crate::game::Game, e: crate::Opaque) {}

/// The frame a wall goes: the soot's marks land on it. The TypeScript's `dirty`, `todo`, `held` and `shed` are the picture's own pieces: not kept here. (src/game/edgeLook.ts:111 EdgeLook.dress)
pub fn edge_look_dress(g: &mut crate::game::Game, b: &crate::edge_look::BreakLike, fresh: &[f64], cause: &str, dir: Option<V3>, quiet: bool) {
    let f = if matches!(b.kind, crate::wallbreak::BKind::Wall) { Some(edge_look_frame(g, b)) } else { None };
    let blast = cause == "blast";
    if let Some(f) = f { if blast && !quiet { edge_look_scorch(g, b, f, fresh, dir); } }
}

/// The wall's own frame: u along it, y up, v across it. (src/game/edgeLook.ts:178 EdgeLook.frame)
pub fn edge_look_frame(g: &mut crate::game::Game, b: &crate::edge_look::BreakLike) -> crate::edge_look::Frame {
    let (c, h, yaw) = { let r = &g.tab.boxes[b.box_]; (r.c, r.h, r.yaw) };
    let along_x = b.nx > 1.0 || h[0] >= h[2];
    let (cs, sn) = (m::cos(yaw), m::sin(yaw));
    let n = if along_x { b.nx } else { b.nz };
    let l = 2.0 * (if along_x { h[0] } else { h[2] });
    Frame {
        b: b.box_, along_x, cs, sn, t: if along_x { h[2] } else { h[0] }, l, n, w: l / n,
        y0: c[1] - h[1], y_top: c[1] + h[1], y_sill: c[1] + b.yb[1],
        along: if along_x { [cs, 0.0, -sn] } else { [sn, 0.0, cs] },
        nrm: if along_x { [sn, 0.0, cs] } else { [cs, 0.0, -sn] },
    }
}

/// A place in the wall's frame. (src/game/edgeLook.ts:184 EdgeLook.at)
pub fn edge_look_at(g: &mut crate::game::Game, f: crate::edge_look::Frame, u: f64, y: f64, v: f64) -> V3 {
    let c = g.tab.boxes[f.b].c;
    [c[0] + f.along[0] * u + f.nrm[0] * v, y, c[2] + f.along[2] * u + f.nrm[2] * v]
}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/edgeLook.ts:108 EdgeLook.cap)
pub fn edge_look_cap(g: &mut crate::game::Game) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/edgeLook.ts:114 EdgeLook.clear)
pub fn edge_look_clear(g: &mut crate::game::Game) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/edgeLook.ts:117 EdgeLook.update)
pub fn edge_look_update(g: &mut crate::game::Game, dt: f64) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/edgeLook.ts:144 EdgeLook.feet)
pub fn edge_look_feet(g: &mut crate::game::Game, men: &[crate::debris::FootMan], k: crate::debris::FootKnobs, dt: f64) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/edgeLook.ts:158 EdgeLook.flush)
pub fn edge_look_flush(g: &mut crate::game::Game) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/edgeLook.ts:160 EdgeLook.boxes)
pub fn edge_look_boxes(g: &mut crate::game::Game, out: &mut Vec<BoxId>) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/edgeLook.ts:184 EdgeLook.rebuild)
pub fn edge_look_rebuild(g: &mut crate::game::Game, b: &crate::edge_look::BreakLike) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/edgeLook.ts:323 EdgeLook.shed)
pub fn edge_look_shed(g: &mut crate::game::Game, b: &crate::edge_look::BreakLike, f: Option<crate::edge_look::Frame>, c_2: f64, dir: Option<V3>, quiet: bool, blast: bool) {}

/// One soot mark, tagged as the picture's. The answer is `None` here: the TypeScript's only use of it is a count for the checks, not kept. (edgeLook.ts:406-409,423)
fn soot(g: &mut crate::game::Game, pos: V3, normal: V3, size: f64, param: f64) {
    let opts = crate::render_decals::DecalOpts { param: Some(param), look: Some(true), ..Default::default() };
    let d = crate::render_decals::decals_add(g, crate::render_decals::DecalKind::Scorch, pos, normal, Some(size), Some(opts));
    crate::look_decals::look_decal(g, d.as_ref());
}

/// Soot round a blast's gap: both faces of the wall about the opening, the floor at its foot, the rim of what still stands. Only the marks tell the host; `blasted` (the picture's pieces read it) and EDGE_STATS are not kept. (src/game/edgeLook.ts:398 EdgeLook.scorch)
pub fn edge_look_scorch(g: &mut crate::game::Game, b: &crate::edge_look::BreakLike, f: crate::edge_look::Frame, fresh: &[f64], dir: Option<V3>) {
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

/// BOUNDARY: the look's books, all empty (the TypeScript's field initialisers; its constructor keeps only `g`, here reached by path); at the swap the picture's side fills them. (src/game/edgeLook.ts:99 EdgeLook.constructor)
pub fn edge_look_new(g: &mut crate::game::Game) -> crate::edge_look::EdgeLook { EdgeLook::blank() }
