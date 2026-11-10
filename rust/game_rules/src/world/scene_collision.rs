pub use ::game_types::scene_collision::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

use crate::ask::{Leaves, Over};
use crate::v3::quat;
use sim::world::{World, ASK_HOST};

// ---- small private helpers

/// the boxes' rows, as a question reads them
#[inline(always)]
pub fn world(g: &Game) -> &World { g.tab.boxes.world() }
/// a point as a list of its numbers is printed
fn joined(a: &[f64]) -> String {
    let mut s = String::new();
    for (i, v) in a.iter().enumerate() {
        if i > 0 { s.push_str(", "); }
        s.push_str(&js::num_str(*v));
    }
    s
}
#[inline]
fn put_f64(addr: usize, i: usize, v: f64) { unsafe { *(addr as *mut f64).add(i) = v } }
#[inline]
fn get_f64(addr: usize, i: usize) -> f64 { unsafe { *(addr as *const f64).add(i) } }
#[inline]
fn get_u32(addr: usize, i: usize) -> u32 { unsafe { *(addr as *const u32).add(i) } }

// ---- the line through a box

/// where a line stands in a box's own frame, and its interval through the slabs
#[derive(Clone, Copy)]
struct Slab { t0: f64, t1: f64, depth: f64, o: V3, d: V3 }

/// lineBox's first half: the box's frame and the slabs, over the row's numbers
fn slab(a: V3, rd: V3, c: V3, h: V3, yaw: f64, rot: Option<Quat>) -> Option<Slab> {
    let (ox, oy, oz, dx, dy, dz);
    if let Some(rot) = rot {
        let qi = quat::conj(rot);
        let o = quat::rotate(qi, v3::sub(a, c));
        let d = quat::rotate(qi, rd);
        ox = o[0]; oy = o[1]; oz = o[2]; dx = d[0]; dy = d[1]; dz = d[2];
    } else {
        let (cs, sn) = (m::cos(yaw), m::sin(yaw));
        let (px, pz) = (a[0] - c[0], a[2] - c[2]);
        ox = cs * px - sn * pz; oy = a[1] - c[1]; oz = sn * px + cs * pz;
        dx = cs * rd[0] - sn * rd[2]; dy = rd[1]; dz = sn * rd[0] + cs * rd[2];
    }
    let (mut t0, mut t1) = (-m::INF, m::INF);
    let (o, d) = ([ox, oy, oz], [dx, dy, dz]);
    for ax in 0..3 {
        if m::abs(d[ax]) < 1e-12 {
            if m::abs(o[ax]) > h[ax] { return None; }
            continue;
        }
        let (mut ta, mut tb) = ((-h[ax] - o[ax]) / d[ax], (h[ax] - o[ax]) / d[ax]);
        if ta > tb { core::mem::swap(&mut ta, &mut tb); }
        if ta > t0 { t0 = ta; }
        if tb < t1 { t1 = tb; }
        if t0 > t1 { return None; }
    }
    let depth = m::min_n(&[h[0] - m::abs(ox), h[1] - m::abs(oy), h[2] - m::abs(oz)]);
    Some(Slab { t0, t1, depth, o, d })
}
/// lineBox's round things: the interval inside the cylinder about the box's own y, and the origin's depth
fn slab_round(s: Slab, h: V3) -> Option<Slab> {
    let ([ox, oy, oz], [dx, _, dz]) = (s.o, s.d);
    let (mut t0, mut t1) = (s.t0, s.t1);
    let (qa, qb, qc) = (dx * dx + dz * dz, ox * dx + oz * dz, ox * ox + oz * oz - h[0] * h[0]);
    if qa < 1e-18 {
        if qc > 0.0 { return None; }
    } else {
        let disc = qb * qb - qa * qc;
        if disc < 0.0 { return None; }
        let r = m::sqrt(disc);
        let (ta, tb) = ((-qb - r) / qa, (-qb + r) / qa);
        if ta > t0 { t0 = ta; }
        if tb < t1 { t1 = tb; }
        if t0 > t1 { return None; }
    }
    let depth = m::min(h[0] - m::sqrt(ox * ox + oz * oz), h[1] - m::abs(oy));
    Some(Slab { t0, t1, depth, ..s })
}

/// the interval of the line a + t·rd inside box b, and how deep the origin sits.
pub fn line_box(g: &Game, a: V3, rd: V3, b: BoxId) -> Option<crate::scene_collision::LineBox> {
    let x = &g.tab.boxes[b];
    let h = x.h;
    let s = slab(a, rd, x.c, h, x.yaw, x.rot)?;
    if x.round == Some(true) {
        let s = slab_round(s, h)?;
        return Some(LineBox { t0: s.t0, t1: s.t1, depth: s.depth });
    }
    let (mut t0, mut t1, mut depth) = (s.t0, s.t1, s.depth);
    if let Some(cut) = x.cut.as_deref().filter(|c| !c.is_empty()) {
        let ([ox, oy, oz], [dx, dy, dz]) = (s.o, s.d);
        (t0, t1) = crate::scene_cut::cut_clip(cut, ox, oy, oz, dx, dy, dz, t0, t1)?;
        let mut k = 0;
        while k < cut.len() && (k as f64) < crate::scene_cut::MAX_CUT_PLANES {
            let c = &cut[k];
            if c.n[0] == 0.0 && c.n[1] == 0.0 && c.n[2] == 0.0 { break; }
            depth = m::min(depth, c.d - (c.n[0] * ox + c.n[1] * oy + c.n[2] * oz));
            k += 1;
        }
    }
    Some(LineBox { t0, t1, depth })
}

/// where a span's foot is asked from: a test that begins at the feet begins at the bake's height. UNDERFOOT.on is settled on.
fn foot_band(y0: f64) -> f64 {
    if y0 >= FEET_Y && y0 < BAKE_Y { BAKE_Y } else { y0 }
}

/// a ray that is no number, in words.
pub fn no_number(ro: Option<V3>, rd: Option<V3>, tmax: Option<f64>) -> String {
    let at = match (ro, rd) {
        (Some(ro), Some(rd)) => alloc::format!(": from [{}] along [{}] for {}", joined(&ro), joined(&rd), match tmax { Some(t) => js::num_str(t), None => String::from("undefined") }),
        _ => String::new(),
    };
    alloc::format!("a ray that is no number{} — where it starts, the way it goes and how far are numbers, and how far is finite (a way worked out of two points that are one point is none)", at)
}

/// a question's word of the rows: the block that holds them is this game's
fn rows_for(g: &Game, r: Option<LampRows>) -> Option<sim::light::Rows> {
    r.map(|r| sim::light::Rows { g: g.sim, leave: js::to_u32(r.leave), steady: r.steady, exposure: crate::hours::exposure(g) })
}

/// a sense's question the module handed back, in words
fn handed_back_words(row: &str, why: &str, a: &[f64], b: &[f64]) -> String {
    alloc::format!("{}: the Rust module handed the question back ({}): [{}] → [{}]", row, why, joined(&a[..3]), joined(&b[..3]))
}
// ---- StaticCollision: the grid, as the boxes are filed in it

/// the box world stood up: the cells from `boxes`, the walk grid baked. `g.col = new(g, …)`: everything of g.col is new, and is built where the bake can ask it.
pub fn new(g: &mut crate::game::Game, boxes: &[BoxId]) -> crate::scene_collision::StaticCollision {
    g.col = StaticCollision::blank();
    g.col.boxes = boxes.to_vec();
    g.tab.boxes.statics(boxes);
    let wd = crate::scene_world::world();
    g.col.w = wd.grid_w;
    g.col.h = wd.grid_h;
    g.col.cs = wd.grid_cell;
    let (gw, gh, gc) = (g.col.w, g.col.h, g.col.cs);
    g.col.cells = vec![Vec::new(); (gw * gh) as usize];
    let skip = js::to_u32(crate::scene_boxes::box_flag().no_shadow) | js::to_u32(crate::scene_boxes::box_flag().dynamic);
    for (at, &b) in boxes.iter().enumerate() {
        let (flags, yaw, h, c, tilted) = { let r = &g.tab.boxes[b]; (r.flags, r.yaw, r.h, r.c, r.rot.is_some()) };
        if js::to_u32(flags) & skip != 0 { continue; }
        let i = at as f64;
        let (cs, sn) = (m::abs(m::cos(yaw)), m::abs(m::sin(yaw)));
        let (ex, ez) = (cs * h[0] + sn * h[2], sn * h[0] + cs * h[2]);
        let (x0, x1) = (m::max(0.0, m::floor((c[0] - ex) / gc)), m::min(gw - 1.0, m::floor((c[0] + ex) / gc)));
        let (z0, z1) = (m::max(0.0, m::floor((c[2] - ez) / gc)), m::min(gh - 1.0, m::floor((c[2] + ez) / gc)));
        let mut z = z0;
        while z <= z1 {
            let mut x = x0;
            while x <= x1 {
                g.col.cells[(z * gw + x) as usize].push(i);
                x += 1.0;
            }
            z += 1.0;
        }
        if tilted && x0 <= x1 && z0 <= z1 {
            g.col.tilted_at.extend_from_slice(&[i, x0, x1, z0, z1]);
        }
    }
    filed(g);
    g.col.nav = nav_grid_new(g);
    g.col.sound = sound_paths_new();
    core::mem::take(&mut g.col)
}

/// THE CHECK (a build with debug assertions makes it as a tick ends): where the box world is not the boxes, the lists and the cells as they stand
pub fn astray(g: &Game) -> Option<(&'static str, usize)> {
    let t = &g.tab.props_prop;
    let things = g.props.props.iter().map(|&p| (t[p].id as u32, &t[p].boxes[..], [t[p].x, t[p].z, t[p].yaw, t[p].radius]));
    g.tab.boxes.astray(&g.col.boxes, &g.col.dynamic_boxes, things, &g.col.cells)
        .or_else(|| world(g).sheets.iter().find(|s| world(g).walked(s.bi as usize) != crate::glassbreak::glass_break_walkable(g, s.bi as f64)).map(|s| ("pane a walker goes through", s.bi as usize)))
}

/// the grid the rows are asked by is the cells as they stand
fn filed(g: &mut Game) {
    let c = &g.col;
    g.tab.boxes.grid(&c.cells, &c.tilted_at, c.w, c.h, c.cs);
}

/// a static appended in play: `b` joins the list and is filed in the cells of `footprint`'s plan. → its index.
pub fn add_static(g: &mut crate::game::Game, b: BoxId, footprint: Option<BoxId>) -> f64 {
    let i = g.col.boxes.len() as f64;
    g.col.boxes.push(b);
    g.tab.boxes.add_static(b);
    let [x0, x1, z0, z1] = foot_cells(g, footprint.unwrap_or(b));
    let gw = g.col.w;
    let mut z = z0;
    while z <= z1 {
        let mut x = x0;
        while x <= x1 {
            g.col.cells[(z * gw + x) as usize].push(i);
            x += 1.0;
        }
        z += 1.0;
    }
    filed(g);
    i
}

/// the cells a static appended over `f` is filed in: x0, x1, z0, z1.
fn foot_cells(g: &Game, f: BoxId) -> [f64; 4] {
    let (yaw, h, c) = { let r = &g.tab.boxes[f]; (r.yaw, r.h, r.c) };
    let (gw, gh, gc) = (g.col.w, g.col.h, g.col.cs);
    let (cs, sn) = (m::abs(m::cos(yaw)), m::abs(m::sin(yaw)));
    let (ex, ez) = (cs * h[0] + sn * h[2], sn * h[0] + cs * h[2]);
    [
        m::max(0.0, m::floor((c[0] - ex) / gc)),
        m::min(gw - 1.0, m::floor((c[0] + ex) / gc)),
        m::max(0.0, m::floor((c[2] - ez) / gc)),
        m::min(gh - 1.0, m::floor((c[2] + ez) / gc)),
    ]
}

/// how many entries of the cells that is.
pub fn foot_entries(g: &Game, f: BoxId) -> f64 {
    let [x0, x1, z0, z1] = foot_cells(g, f);
    m::max(0.0, x1 - x0 + 1.0) * m::max(0.0, z1 - z0 + 1.0)
}

/// the level's list cut back to its first `n`: every later index leaves the cells.
pub fn truncate_statics(g: &mut crate::game::Game, n: f64) {
    if (g.col.boxes.len() as f64) <= n { return; }
    cut_cells(&mut g.col.cells, n);
    g.tab.boxes.cut_statics(&g.col.boxes, n as usize);
    g.col.boxes.truncate(n as usize);
    filed(g);
}
/// every index from `n` on leaves the cells, the rest keeping their order
fn cut_cells(cells: &mut [Vec<f64>], n: f64) {
    for cell in cells.iter_mut() {
        cell.retain(|&i| i < n);
    }
}

/// the foot of a span as the statics are asked it.
pub fn foot_of(y0: f64) -> f64 { foot_band(y0) }

// ---- circles against the boxes (rust/sim collide.rs)

/// the statics filed under a rectangle that overlap a span, once each, into `out`.
pub fn gather(g: &Game, x0: f64, z0: f64, x1: f64, z1: f64, y0: f64, y1: f64, out: &mut Vec<i32>) -> f64 {
    let n = sim::collide::gather(world(g), x0, z0, x1, z1, y0, y1, out.len());
    let list = &sim::world::asking().list;
    for i in 0..n {
        out[i] = list[i] as i32;
    }
    n as f64
}

/// the statics a circle touches over a span: how many, −1 = `out` too short. `out` is left as the two steps leave it (the touching ones, then what the gather put past them).
pub fn circle_touches(g: &Game, x: f64, z: f64, r: f64, y0: f64, y1: f64, out: &mut Vec<i32>) -> f64 {
    let y0 = foot_band(y0);
    let cap = if out.len() > sim::world::MAX_LIST { sim::world::MAX_LIST } else { out.len() };
    let v = sim::collide::touches(world(g), x, z, r, y0, y1, cap);
    let full = v < 0;
    let (n, k) = if full { (cap, -1.0) } else { ((v as u32 & 0xffff) as usize, (v as u32 >> 16) as f64) };
    let list = &sim::world::asking().list;
    for i in 0..n {
        out[i] = list[i] as i32;
    }
    k
}

/// a circle pushed out of the statics over a span; true = it touched.
pub fn collide_circle(g: &Game, p: &mut V3, r: f64, y0: f64, y1: f64, iterations: Option<f64>) -> bool {
    let y0 = foot_band(y0);
    let hit = sim::collide::circle(world(g), p[0], p[2], r, y0, y1, js::to_u32(iterations.unwrap_or(3.0))) != 0;
    let pt = sim::world::asking().pt;
    p[0] = pt[0];
    p[2] = pt[1];
    hit
}

/// a circle kept on the world's floor rectangle; true = it had to move.
pub fn clamp_to_world(p: &mut V3, r: f64) -> bool {
    let wd = crate::scene_world::world();
    let x = m::min(wd.max[0] - r, m::max(wd.min[0] + r, p[0]));
    let z = m::min(wd.max[2] - r, m::max(wd.min[2] + r, p[2]));
    let moved = x != p[0] || z != p[2];
    p[0] = x;
    p[2] = z;
    moved
}

// ---- lines

/// the world as a question takes it, in the loops' own words, for the question `f`
fn over_lent<R>(g: &Game, over: &Over, f: impl FnOnce(&sim::world::Over) -> R) -> R {
    let leaves = over.leaves != Leaves::None;
    if over.plain() { return f(&sim::world::Over::with(true)); }
    let but: Vec<u32> = match &over.leaves { Leaves::But(l) => (0..g.col.dynamic_boxes.len() as u32).filter(|&i| l.contains(&g.col.dynamic_boxes[i as usize])).collect(), _ => Vec::new() };
    let (extra, extra_cut) = if over.extra.is_empty() { (Vec::new(), Vec::new()) } else { sim::world::rows_of(over.extra.iter().map(|&b| &g.tab.boxes[b])) };
    f(&sim::world::Over { leaves, but: &but, open: &over.open, extra: &extra, extra_cut: &extra_cut })
}

/// a leaf is across a→b; with `need_flags`, one carrying those.
pub fn segment_blocked_dynamic(g: &Game, a: V3, b: V3, need_flags: Option<f64>) -> bool {
    let need = js::to_u32(need_flags.unwrap_or(0.0));
    let len = m::hyp3(b[0] - a[0], b[1] - a[1], b[2] - a[2]);
    if !(len >= 1e-4) || !(len < 1e6) { return false; }   // (a length that is no length of this world's is across nothing)
    debug_assert!(need & !sim::world::F_KEPT == 0, "a flag the box world does not keep");
    sim::lines::leaves_blocked(world(g), a[0], a[1], a[2], b[0], b[1], b[2], len, need)
}

/// the furniture's ray: the nearest part met.
pub fn things_ray(g: &Game, ro: V3, rd: V3, tmax: f64) -> Option<crate::scene_collision::ThingsRay> {
    let mut out = [0.0; sim::lines::RAY_PART + 1];
    let k = sim::lines::ray_things(world(g), ro[0], ro[1], ro[2], rd[0], rd[1], rd[2], tmax, &mut out);
    if k < 0 { return None; }
    let (w, at) = (world(g), out[sim::lines::RAY_PART] as u32);
    let part = at - w.things[k as usize].at;
    Some(ThingsRay { row: k as f64, t: out[0], n: [out[1], out[2], out[3]], box_: g.tab.props_prop[g.props.props[k as usize]].boxes[part as usize] })
}

/// the line a→b. Panes in the way are added to `panes` where it is given (static, metres along).
fn line(g: &Game, over: &Over, a: V3, b: V3, shrink: f64, through: f64, panes: Option<&mut Vec<crate::scene_collision::SegmentBlockedPanesItem>>) -> bool {
    let len = m::hyp3(b[0] - a[0], b[1] - a[1], b[2] - a[2]);
    if !(len >= 1e-4) || !(len < 1e6) { return false; }   // (a length that is no length of this world's is stopped by nothing)
    let thr = js::to_u32(through);
    debug_assert!(thr & !sim::world::F_KEPT == 0, "a flag the box world does not keep");
    let word = if thr & sim::world::F_GLASS != 0 { 0 } else { sim::lines::ASK_GLASS } | if panes.is_some() { sim::lines::ASK_PANES } else { 0 };
    let r = over_lent(g, over, |v| sim::lines::blocked_over(world(g), v, a[0], a[1], a[2], b[0], b[1], b[2], len, shrink, thr, word, &g.k));
    if r == -2 { panic!("more glass on one line than the box world lists ({})", sim::world::MAX_PANES); }
    if r > 0 {
        if let Some(out) = panes {
            let o = sim::world::asking();
            for i in 0..r as usize {
                out.push(SegmentBlockedPanesItem { bi: o.pane_bi[i] as f64, t: o.pane_t[i] });
            }
        }
    }
    r == -1
}

/// is a→b blocked by a static or a leaf; `through` = the flags it passes (glass by default: the line of sight).
pub fn segment_blocked(g: &Game, a: V3, b: V3, shrink: Option<f64>, through: Option<f64>, panes: Option<&mut Vec<crate::scene_collision::SegmentBlockedPanesItem>>) -> bool {
    line(g, &Over::OWN, a, b, shrink.unwrap_or(0.02), through.unwrap_or(crate::scene_boxes::box_flag().glass), panes)
}
/// …of the world as the question takes it
pub fn segment_blocked_over(g: &Game, over: &Over, a: V3, b: V3, shrink: Option<f64>, through: Option<f64>) -> bool {
    line(g, over, a, b, shrink.unwrap_or(0.02), through.unwrap_or(crate::scene_boxes::box_flag().glass), None)
}

/// segmentBlocked with nothing passed: the solid test.
pub fn solid_blocked(g: &Game, a: V3, b: V3, shrink: Option<f64>) -> bool { solid_blocked_over(g, &Over::OWN, a, b, shrink) }
/// …of the world as the question takes it
pub fn solid_blocked_over(g: &Game, over: &Over, a: V3, b: V3, shrink: Option<f64>) -> bool { segment_blocked_over(g, over, a, b, shrink, Some(0.0)) }

/// the line of fire: anything a round would stop in.
pub fn shot_blocked(g: &Game, a: V3, b: V3, shrink: Option<f64>, energy: Option<f64>) -> bool { shot_blocked_over(g, &Over::OWN, a, b, shrink, energy) }
/// …of the world as the question takes it
pub fn shot_blocked_over(g: &Game, over: &Over, a: V3, b: V3, shrink: Option<f64>, energy: Option<f64>) -> bool {
    let pass = match g.col.fire_pass {
        Some(FirePass::Bars) => crate::scene_boxes::box_flag().bar,
        None => 0.0,
    };
    let mut panes: Vec<SegmentBlockedPanesItem> = Vec::new();
    if line(g, over, a, b, shrink.unwrap_or(0.02), pass, Some(&mut panes)) { return true; }
    if panes.is_empty() { return false; }
    js::sort_by(&mut panes, |x, y| x.t - y.t);
    let d = v3::sub(b, a);
    !crate::glasspen::shot_passes(g, a, v3::scale(d, 1.0 / v3::len(d)), &panes, energy)
}

// ---- rays

/// the closest static or leaf along a ray; a leaf's index is 1e6 + its place.
pub fn raycast(g: &Game, ro: V3, rd: V3, tmax: f64) -> Option<crate::scene_collision::Raycast> { raycast_over(g, &Over::OWN, ro, rd, tmax) }
/// …of the world as the question takes it. A leaf's place is its place among the game's own, whichever are let through; a box the question adds is 2e6 + its place. A ray
/// that is no number is the caller's mistake, by name.
pub fn raycast_over(g: &Game, over: &Over, ro: V3, rd: V3, tmax: f64) -> Option<crate::scene_collision::Raycast> {
    match ray_met(g, over, ro, rd, tmax) {
        Ok(h) => h,
        Err(()) => panic!("{}", no_number(Some(ro), Some(rd), Some(tmax))),
    }
}
/// …for a host, that asks it of a camera or a cursor: a ray that is no number meets nothing, and the game is not to fall for it
pub fn raycast_host(g: &Game, over: &Over, ro: V3, rd: V3, tmax: f64) -> Option<crate::scene_collision::Raycast> { ray_met(g, over, ro, rd, tmax).unwrap_or(None) }
fn ray_met(g: &Game, over: &Over, ro: V3, rd: V3, tmax: f64) -> Result<Option<crate::scene_collision::Raycast>, ()> {
    if !(tmax > 0.0) { return Ok(None); }
    let mut out = [0.0; 4];
    let k = over_lent(g, over, |v| sim::lines::ray_over(world(g), v, ro[0], ro[1], ro[2], rd[0], rd[1], rd[2], tmax, true, &mut out, &g.k));
    if k == sim::lines::RAY_HOST { return Err(()); }
    Ok(if k < 0 { None } else { Some(Raycast { t: out[0], index: k as f64, n: [out[1], out[2], out[3]] }) })
}

// ---- the senses, the module's alone (rust/sim fog.rs, sight.rs, light.rs, smoke.rs, bricks.rs): what it cannot take is the asker's mistake, by name

/// an eye's line, and the fog's: it passes glass, and is asked by flags the rows keep.
fn eye_line(row: &str, through: f64) {
    if js::to_u32(through) & !sim::world::F_KEPT != 0 || js::to_u32(through) & sim::world::F_GLASS == 0 {
        panic!("{}: an eye's line passes glass and names only flags the box world keeps (asked through {}): one that stops at glass is no eye's, and is asked as a line (segmentBlocked)", row, js::num_str(through));
    }
}

/// a sight pass opens: its `n` eyes are written next. More than a question names is a mistake by name.
pub fn sight_open(n: f64) {
    if n > SIGHT_EYES {
        panic!("fog: a sight pass of {} eyes, and a question names {} (rust/sim/src/fog.rs; the fog's own most is render/fogofwar.ts FOG_EYES)", js::num_str(n), js::num_str(SIGHT_EYES));
    }
}

/// eye `k` of the pass.
pub fn sight_eye(k: f64, pos: V3, look: V3, look_len: f64, cos_half: f64, near: f64, full: bool) {
    let at = sim::fog::eyes_addr();
    let o = k as usize * sim::fog::EYE;
    put_f64(at, o, pos[0]);
    put_f64(at, o + 1, pos[1]);
    put_f64(at, o + 2, pos[2]);
    put_f64(at, o + 3, look[0]);
    put_f64(at, o + 4, look[2]);
    put_f64(at, o + 5, look_len);
    put_f64(at, o + 6, cos_half);
    put_f64(at, o + 7, near);
    put_f64(at, o + 8, if full { 1.0 } else { 0.0 });
}

/// which eyes have a line: how many × 2²⁸ + those eyes, four bits each.
pub fn eyes_see(g: &Game, order: f64, n: f64, want: f64, kind: f64, through: f64, a: V3, b: V3) -> f64 {
    eye_line("fog", through);
    let r = sim::fog::see(world(g), js::to_u32(order), js::to_u32(n), js::to_u32(want), js::to_u32(kind), js::to_u32(through), a[0], a[1], a[2], b[0], b[1], b[2], &g.k);
    if r == ASK_HOST { panic!("{}", handed_back_words("fog", "a length that is no length of this world's, or more eyes than a question names", &a, &b)); }
    r as f64
}

/// an eye on a man's chest and head as one question.
pub fn eye_contact(g: &Game, e: crate::scene_collision::EyeContactE, chest: V3, head: V3, unseen: bool, smoked: bool, through: f64) -> crate::scene_collision::EyeContact {
    eye_line("sight", through);
    let q = (if e.v_half_deg.is_some() { sim::sight::Q_LENS } else { 0 }) | (if unseen { sim::sight::Q_UNSEEN } else { 0 }) | (if smoked { sim::sight::Q_SMOKED } else { 0 });
    let mut dist = 0.0;
    let r = sim::sight::contact(world(g), js::to_u32(through), q, e.pos, e.look, e.range, e.half_deg, e.near_m, e.near_deg, e.v_half_deg.unwrap_or(0.0), chest, head, &mut dist, &g.k);
    if r == ASK_HOST { panic!("{}", handed_back_words("sight", "a line too long to be one, or of no number", &e.pos, &chest)); }
    EyeContact { visible: r & sim::sight::A_VISIBLE != 0, dist, in_cone: r & sim::sight::A_IN_CONE != 0 }
}

/// the light of these lamps on a point, onto the six faces: the question's own (`lamps`) and the level's, which the loop reads where they stand (`rows`); `shade` = the smoke dims
/// each on its way.
pub fn lamps_on(g: &Game, p: V3, lamps: &[[f64; sim::light::LAMP]], shade: bool, rows: crate::scene_collision::LampRows) -> [f64; 6] {
    let shade = if shade { sim::light::Shade { sigma: g.k.sense.smoke.sigma, g: g.sim, stamp: sim::state::stamp(g.sim) } } else { sim::light::NO_SHADE };
    let mut out = [0.0; 6];
    let r = sim::light::estimate(world(g), lamps, p, &shade, rows_for(g, Some(rows)), &mut out, &g.k);
    if r == ASK_HOST { panic!("{}", handed_back_words("light", "a lamp's line too long to be one, or of no number", &p, &p)); }
    out
}

/// the ways between the coarse grid's cells are asked of the boxes and leaves as they stand: a game put back knows none till its next step
pub fn grid_ways(g: &Game) { sim::bricks::ways(g.sim, sim::state::stamp(g.sim), world(g), &g.k); }

/// `n` steps of the coarse grid's smoke over the boxes and leaves as they stand. False = nothing was done.
pub fn grid_step(g: &mut crate::game::Game, n: f64, k: crate::scene_collision::GridStepK, sweep: f64, wind: Option<&crate::scene_collision::GridWind>) -> bool {
    // (the light that comes round: the lamps of nobody's fixture, and the level's by the rows' word)
    let (lamps, said) = crate::guards::steady_lamps(g);
    let sky = crate::hours::sky_now(g);
    let (mut wx, mut wz, mut low, mut gain, mut full_at) = (0.0, 0.0, 1.0, 0.0, 1.0);
    if let Some(wd) = wind {
        if wd.x != 0.0 || wd.z != 0.0 {
            let (gw, gh, gc) = (g.col.w, g.col.h, g.col.cs);
            let (nx, nz) = (2.0 * m::ceil(gw * gc / 2.0), 2.0 * m::ceil(gh * gc / 2.0));
            if nx * nz <= sim::bricks::MAX_GLOW as f64 {
                let at = sim::bricks::wind_addr();
                let GridWindShare::Under { roofs } = &wd.share;
                let mut o = 0usize;
                let mut z = 0.0;
                while z < nz {
                    let mut i = 0.0;
                    while i < nx {
                        let v = crate::wind::wind_share_under(g, i + 0.5, z + 0.5, roofs.as_deref());
                        unsafe { *at.add(o) = v; }
                        o += 1;
                        i += 1.0;
                    }
                    z += 1.0;
                }
                wx = wd.x;
                wz = wd.z;
                low = wd.low;
                gain = wd.gain;
                full_at = wd.full_at;
            }
        }
    }
    // (a wind that is no number, or whose rise with height is none, is still air: the block is the game's state, and what is no number spreads)
    let real = |v: f64| v - v == 0.0;
    let windy = real(wx) && real(wz) && real(low) && real(gain) && real(full_at) && real(k.in_wind) && low >= 0.0 && gain >= 0.0 && full_at > 0.0;
    let (wx, wz) = if windy { (wx, wz) } else { (0.0, 0.0) };
    let bk = sim::bricks::K { decay: k.decay, spread: k.spread, rise: k.rise, rise_rate: k.rise_rate, gone: k.gone, wind_x: wx, wind_z: wz, wind_low: low, wind_gain: gain, wind_full_at: full_at, wind_keep: k.in_wind };
    let gk = sim::glow::K { sweep, sky, lamps: &lamps, rows: rows_for(g, Some(said)) };
    sim::bricks::step(g.sim, sim::state::stamp(g.sim), world(g), js::to_u32(n), &bk, Some(&gk), &g.k) != ASK_HOST
}

/// What the light under a roof is worked out from is made as a game stands, behind the loading, and not in its first step: the statics' lines, no state.
pub fn light_ready(g: &Game) { sim::bricks::light_ready(g.sim, world(g), crate::hours::sky_now(g).to_sun, &g.k); }

// ---- how a sound reaches a listener

fn sound_paths_new() -> crate::scene_collision::SoundPaths {
    SoundPaths::blank()
}

/// once a tick: the clock of the routes remembered, and a memory over 512 cleared whole.
pub fn sound_paths_tick(g: &mut crate::game::Game, time: f64) {
    g.col.sound.now = time;
    if g.col.sound.cache.len() > 512 { g.col.sound.cache.clear(); }
}

/// the level changed under the routes remembered.
pub fn sound_paths_invalidate(g: &mut crate::game::Game) {
    g.col.sound.cache.clear();
}

/// how a sound at `src` reaches `dst`; `keep` false = the answer is not kept. (The game's own ears remember a way round for half a second, so this one question writes; whoever may
/// not write asks `heard`.)
pub fn sound_paths_propagate(g: &mut crate::game::Game, src: V3, dst: V3, keep: Option<bool>) -> crate::scene_collision::Propagation {
    let straight = match heard_straight(g, src, dst) { Ok(r) => return r, Err(straight) => straight };
    let key = route_key(g, src, dst);
    if let Some(hit) = g.col.sound.cache.get(key.clone()) {
        if m::abs(g.col.sound.now - hit.t) < 0.5 { return hit.r; }
    }
    let Some(r) = heard_round(g, src, dst, straight) else { panic!("{}", handed_back_words("sound", "a line of its way round too long to be one, or a sound asked for from inside the price of a search (its way round is a search, and there is one search's tables)", &src, &dst)) };
    if keep.unwrap_or(true) && !g.col.sound.asking {
        let t = g.col.sound.now;
        g.col.sound.cache.set(key, SoundPathsCacheValue { t, r });
    }
    r
}

/// the first half of how a sound reaches: straight, over the top, or too far walled off for a way round (Ok: its answer) · Err: the
/// straight distance, walled off, and the way round is to be asked
fn heard_straight(g: &Game, src: V3, dst: V3) -> Result<Propagation, f64> {
    let mut straight = 0.0;
    let c = sim::hear::direct(world(g), src, dst, &mut straight, &g.k);
    if c == ASK_HOST { panic!("{}", handed_back_words("sound", "a straight line too long to be one, or of no number", &src, &dst)); }
    if c < sim::hear::OVER {
        let mut graze = 0.0;
        if c & sim::hear::GRAZE_A != 0 { graze += 0.1; }
        if c & sim::hear::GRAZE_B != 0 { graze += 0.1; }
        return Ok(Propagation { direct: true, routed: false, path_len: straight, via: v3::copy(src), apparent: v3::copy(src), muffle: graze, carry: 1.0 });
    }
    if c == sim::hear::OVER {
        return Ok(Propagation { direct: true, routed: false, path_len: straight, via: v3::copy(src), apparent: v3::copy(src), muffle: 0.15, carry: 0.9 });
    }
    if c == sim::hear::FAR { return Ok(sealed(src, straight)); }
    Err(straight)
}
/// the second half: round by the floor, or sealed (None: no answer to give — a line of it is none of this world's, or a search waits for its prices)
fn heard_round(g: &Game, src: V3, dst: V3, straight: f64) -> Option<Propagation> {
    nav_grid_there(g);
    let (nw, nh, cs) = (g.col.nav.w as usize, g.col.nav.h as usize, g.col.nav.cell);
    let seal = js::to_u32(crate::scene_boxes::box_flag().sound_seal);
    let mut out = [0.0; 4];
    let k = sim::hear::route(world(g), seal, nw, nh, cs, src, dst, straight, &mut out, &g.k);
    if k == ASK_HOST { return None; }
    if k == sim::hear::SEALED { return Some(sealed(src, straight)); }
    let (l, via) = (out[0], [out[1], out[2], out[3]]);
    let dir = v3::normalize([via[0] - dst[0], 0.0, via[2] - dst[2]]);
    let muffle = if k == sim::hear::ROUTED_DOOR { 0.85 } else { 0.35 };
    Some(Propagation { direct: false, routed: true, path_len: l, via, apparent: [dst[0] + dir[0] * l, src[1], dst[2] + dir[2] * l], muffle, carry: 1.0 - muffle * 0.8 })
}

/// …for whoever may not write the game: no route is remembered and none is looked up (`way`: who remembers the ways round, if anybody does). A place that is no number, or no
/// place of this world, is heard from nowhere: a host asks it of a sound and a listener of its own, and the game is not to fall for it
pub fn heard_by(g: &Game, src: V3, dst: V3, way: impl FnOnce(&Game, &dyn Fn(&Game) -> Propagation) -> Propagation) -> crate::scene_collision::Propagation {
    let len = m::hyp3(src[0] - dst[0], src[1] - dst[1], src[2] - dst[2]);
    if !(len < 1e5) { return Propagation { direct: false, routed: false, path_len: 1e5, via: dst, apparent: dst, muffle: 1.0, carry: 0.0 }; }
    match heard_straight(g, src, dst) { Ok(r) => r, Err(straight) => way(g, &|g| heard_round(g, src, dst, straight).unwrap_or_else(|| sealed(src, straight))) }
}

/// the remembered routes' key: the two nav cells. (A −0 reads as 0, as a JavaScript string does.)
fn route_key(g: &Game, src: V3, dst: V3) -> String {
    let cs = g.col.nav.cell;
    alloc::format!("{},{}>{},{}", m::floor(src[0] / cs) + 0.0, m::floor(src[2] / cs) + 0.0, m::floor(dst[0] / cs) + 0.0, m::floor(dst[2] / cs) + 0.0)
}
fn sealed(src: V3, straight: f64) -> Propagation {
    Propagation { direct: false, routed: false, path_len: straight * 1.5, via: v3::copy(src), apparent: v3::copy(src), muffle: 1.0, carry: 0.25 }
}

// ---- the walk grid

/// the floor's grid baked: a cell is shut where a guard-sized circle at its centre is pushed.
fn nav_grid_new(g: &Game) -> crate::scene_collision::NavGrid {
    let mut nav = NavGrid::blank();
    let wd = crate::scene_world::world();
    let cell = nav.cell;
    nav.w = m::round(wd.grid_w * wd.grid_cell / cell);
    nav.h = m::round(wd.grid_h * wd.grid_cell / cell);
    let (nw, nh) = (nav.w, nav.h);
    let mut blocked = vec![0u8; (nw * nh) as usize];
    let mut z = 0.0;
    while z < nh {
        let mut x = 0.0;
        while x < nw {
            let p: V3 = [(x + 0.5) * cell, 0.0, (z + 0.5) * cell];
            let mut q: V3 = [p[0], 0.0, p[2]];
            if collide_circle(g, &mut q, 0.34, 0.25, 1.5, Some(1.0)) {
                let moved = v3::dist_xz(q, p);
                if moved > 0.005 { blocked[(z * nw + x) as usize] = 1; }
            }
            x += 1.0;
        }
        z += 1.0;
    }
    nav.blocked = blocked;
    nav
}

/// the level changed inside a rectangle: the bake's question asked again of the cells near it. → the cells that changed, index and now-shut pairs.
pub fn nav_grid_rebake(g: &mut crate::game::Game, x0: f64, z0: f64, x1: f64, z1: f64) -> Vec<f64> {
    let (cell, nw, nh) = (g.col.nav.cell, g.col.nav.w, g.col.nav.h);
    let pad = 0.34 + cell;
    let mut changed = Vec::new();
    let cx0 = clampi(m::floor((x0 - pad) / cell), 0.0, nw - 1.0);
    let cx1 = clampi(m::floor((x1 + pad) / cell), 0.0, nw - 1.0);
    let cz0 = clampi(m::floor((z0 - pad) / cell), 0.0, nh - 1.0);
    let cz1 = clampi(m::floor((z1 + pad) / cell), 0.0, nh - 1.0);
    let mut z = cz0;
    while z <= cz1 {
        let mut x = cx0;
        while x <= cx1 {
            let p: V3 = [(x + 0.5) * cell, 0.0, (z + 0.5) * cell];
            let mut q: V3 = [p[0], 0.0, p[2]];
            let b: u8 = if collide_circle(g, &mut q, 0.34, 0.25, 1.5, Some(1.0)) && v3::dist_xz(q, p) > 0.005 { 1 } else { 0 };
            let i = z * nw + x;
            if g.col.nav.blocked[i as usize] != b {
                g.col.nav.blocked[i as usize] = b;
                changed.push(i);
                changed.push(b as f64);
            }
            x += 1.0;
        }
        z += 1.0;
    }
    changed
}

pub fn nav_grid_idx(g: &crate::game::Game, x: f64, z: f64) -> f64 {
    z * g.col.nav.w + x
}

/// the statics the bake's own question meets at a cell: count into `out`, −1 = too short.
pub fn nav_grid_cell_blockers(g: &Game, wx: f64, wz: f64, out: &mut Vec<i32>) -> f64 {
    let (cell, nw, nh) = (g.col.nav.cell, g.col.nav.w, g.col.nav.h);
    let (x, z) = (m::floor(wx / cell), m::floor(wz / cell));
    if x < 0.0 || z < 0.0 || x >= nw || z >= nh { return 0.0; }
    circle_touches(g, (m::floor(wx / cell) + 0.5) * cell, (m::floor(wz / cell) + 0.5) * cell, 0.34, 0.25, 1.5, out)
}

pub fn nav_grid_is_blocked(g: &crate::game::Game, wx: f64, wz: f64) -> bool {
    let (cell, nw, nh) = (g.col.nav.cell, g.col.nav.w, g.col.nav.h);
    let (x, z) = (m::floor(wx / cell), m::floor(wz / cell));
    if x < 0.0 || z < 0.0 || x >= nw || z >= nh { return true; }
    // (a point that is no number: every comparison above is false, and the TypeScript's table reads undefined, which is not 1)
    if x != x || z != z { return false; }
    g.col.nav.blocked[(z * nw + x) as usize] == 1
}

/// the grid as it stands, written to the module for one question of its search. A floor larger than its table is the build's mistake, by name.
fn nav_grid_there(g: &crate::game::Game) {
    let n = g.col.nav.blocked.len();
    if n > sim::nav::MAX_NAV {
        panic!("the floor's grid has {} cells and a search of the Rust module holds {} (rust/sim/src/nav.rs MAX_FLOOR): the floor is the build's (scene/world.ts WORLD), so the module's tables are to be made larger with it", n, sim::nav::MAX_NAV);
    }
    unsafe { core::ptr::copy_nonoverlapping(g.col.nav.blocked.as_ptr(), sim::nav::shut_addr() as *mut u8, n); }
}

/// A* from a to b, smoothed; `penalty` a cost multiplier for stepping into a cell, asked where the search comes to a cell it has none for.
pub fn nav_grid_find_path(g: &Game, a: V3, b: V3, max_iter: Option<f64>, penalty: Option<&mut dyn FnMut(&Game, f64, f64) -> f64>) -> Option<Vec<V3>> {
    let max_iter = max_iter.unwrap_or(6000.0);
    if js::to_u32(max_iter) as f64 != max_iter { panic!("paths: a cap on the search that is no whole number ({})", js::num_str(max_iter)); }
    match penalty {
        Some(p) => way_priced(g, a, b, max_iter, &mut |g: &mut &Game, x, z| p(g, x, z)),
        None => way_of(search(g, a, b, max_iter), a, b),
    }
}
/// …where the price remembers: the game it's asked of is the caller's own, to write
pub fn way_priced<G: core::ops::Deref<Target = Game>>(g: G, a: V3, b: V3, max_iter: f64, price: &mut dyn FnMut(&mut G, f64, f64) -> f64) -> Option<Vec<V3>> {
    let mut g = g;
    nav_grid_there(&g);
    let (nw, nh, cell) = (g.col.nav.w as usize, g.col.nav.h as usize, g.col.nav.cell);
    let mut n = sim::nav::find_priced(world(&g), nw, nh, cell, a[0], a[2], b[0], b[2], js::to_u32(max_iter), &g.k);
    while n == sim::nav::NEED {
        for i in 0..sim::nav::need_len() {
            let c = get_u32(sim::nav::need_addr(), i);
            let v = price(&mut g, (c % nw as u32) as f64, (c / nw as u32) as f64);
            put_f64(sim::nav::price_addr(), c as usize, v);
        }
        n = sim::nav::resume();
    }
    way_of(n, a, b)
}
/// …for a host, that asks it of places of its own: one that is no number has no way
pub fn way_host(g: &Game, a: V3, b: V3, max_iter: u32) -> Option<Vec<V3>> {
    let n = search(g, a, b, max_iter as f64);
    if n == sim::nav::ASK_HOST { None } else { walked(n) }
}
fn search(g: &Game, a: V3, b: V3, max_iter: f64) -> i32 {
    nav_grid_there(g);
    sim::nav::find_path(world(g), g.col.nav.w as usize, g.col.nav.h as usize, g.col.nav.cell, a[0], a[2], b[0], b[2], js::to_u32(max_iter), &g.k)
}
/// …or the question was no question
fn way_of(n: i32, a: V3, b: V3) -> Option<Vec<V3>> {
    if n == sim::nav::ASK_HOST { panic!("{}", handed_back_words("paths", "a point that is no number, a line too long to be one, or a way asked for from inside the price of another search (there is one search's tables)", &a, &b)); }
    walked(n)
}
/// the way a search left
fn walked(n: i32) -> Option<Vec<V3>> {
    if n == sim::nav::NO_WAY { return None; }
    let at = sim::nav::way_addr();
    Some((0..n as usize).map(|i| [get_f64(at, i * 2), 0.0, get_f64(at, i * 2 + 1)]).collect())
}

/// the free cell nearest by ring to a shut one; with the point that asked, on its own side of the wall.
fn nav_grid_nearest_free(g: &Game, gx: f64, gz: f64, wx: Option<f64>, wz: Option<f64>) -> Option<[f64; 2]> {
    let (cell, nw, nh) = (g.col.nav.cell, g.col.nav.w, g.col.nav.h);
    let mut fallback: Option<[f64; 2]> = None;
    let mut extra = 1;
    let from: Option<V3> = match (wx, wz) {
        (Some(x), Some(z)) => Some([x, 1.0, z]),
        _ => None,
    };
    for r in 1..8i32 {
        for dz in -r..=r {
            for dx in -r..=r {
                if dx.abs().max(dz.abs()) != r { continue; }
                let (x, z) = (gx + dx as f64, gz + dz as f64);
                if x < 0.0 || z < 0.0 || x >= nw || z >= nh || g.col.nav.blocked[(z * nw + x) as usize] != 0 { continue; }
                let Some(from) = from else { return Some([x, z]); };
                if fallback.is_none() { fallback = Some([x, z]); }
                if !solid_blocked(g, from, [(x + 0.5) * cell, 1.0, (z + 0.5) * cell], None) { return Some([x, z]); }
            }
        }
        if fallback.is_some() {
            let spent = extra <= 0;
            extra -= 1;
            if spent { return fallback; }
        }
    }
    fallback
}

/// the centre of the walkable cell nearest a world point, or none within reach.
pub fn nav_grid_nearest_free_point(g: &Game, wx: f64, wz: f64) -> Option<V3> {
    let (cell, nw, nh) = (g.col.nav.cell, g.col.nav.w, g.col.nav.h);
    let x = clampi(m::floor(wx / cell), 0.0, nw - 1.0);
    let z = clampi(m::floor(wz / cell), 0.0, nh - 1.0);
    let c = if g.col.nav.blocked[(z * nw + x) as usize] != 0 { nav_grid_nearest_free(g, x, z, Some(wx), Some(wz)) } else { Some([x, z]) };
    c.map(|c| [(c[0] + 0.5) * cell, 0.0, (c[1] + 0.5) * cell])
}

/// conservative straight-line walkability.
pub fn nav_grid_walkable(g: &crate::game::Game, a: V3, b: V3) -> bool {
    let cell = g.col.nav.cell;
    let (dx, dz) = (b[0] - a[0], b[2] - a[2]);
    let len = m::hyp2(dx, dz);
    let n = m::ceil(len / (cell * 0.5)) + 1.0;
    let (px, pz) = (-dz / js::or(len, 1.0) * 0.22, dx / js::or(len, 1.0) * 0.22);
    let mut i = 0.0;
    while i <= n {
        let t = i / n;
        let (x, z) = (a[0] + dx * t, a[2] + dz * t);
        if nav_grid_is_blocked(g, x, z) || nav_grid_is_blocked(g, x + px, z + pz) || nav_grid_is_blocked(g, x - px, z - pz) { return false; }
        i += 1.0;
    }
    true
}

fn clampi(x: f64, a: f64, b: f64) -> f64 {
    if x < a { a } else if x > b { b } else { x }
}

/// the sight line a→b against the level's STATICS alone
pub fn wall_between(g: &Game, a: V3, b: V3) -> bool {
    crate::scene_collision::segment_blocked_over(g, &crate::ask::Over::statics(), a, b, None, None)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bits(a: &[f64]) -> Vec<u64> {
        a.iter().map(|x| x.to_bits()).collect()
    }
    fn grid(g: &mut Game) {
        g.col.nav.w = 80.0;
        g.col.nav.h = 56.0;
        g.col.nav.blocked = vec![0; 80 * 56];
        for z in 0..40 {
            if z != 20 { g.col.nav.blocked[z * 80 + 30] = 1; }
        }
    }

    #[test]
    fn line_box_is_the_typescripts_to_the_bit() {
        let (a, rd): (V3, V3) = ([1.25, 0.7, -2.5], [0.6, 0.0, 0.8]);
        let (c, h): (V3, V3) = ([3.0, 1.0, 1.0], [1.5, 1.0, 0.75]);
        let r = slab(a, rd, c, h, 0.7, None).unwrap();
        assert_eq!(bits(&[r.t0, r.t1, r.depth]), [0x400879431796fbb5, 0x40123f15faa99759, 0xc0086f43d65a5520]);
        let r = slab(a, rd, c, h, 0.0, Some([0.1, 0.2, 0.3, m::sqrt(1.0 - 0.14)])).unwrap();
        assert_eq!(bits(&[r.t0, r.t1, r.depth]), [0x4009a0153febbaba, 0x4012f1af13a82510, 0xc009137acf27ccdb]);
        let h2: V3 = [1.5, 1.0, 1.5];
        let r = slab_round(slab(a, rd, c, h2, 0.7, None).unwrap(), h2).unwrap();
        assert_eq!(bits(&[r.t0, r.t1, r.depth]), [0x40042fd22b3439eb, 0x4014b4e3b732afd6, 0xc0034e115049ec25]);
        assert!(slab(a, rd, [30.0, 1.0, 1.0], [1.0, 1.0, 1.0], 0.2, None).is_none());
    }

    #[test]
    fn the_walk_grid_answers_as_it_stands() {
        let mut g = Box::new(Game::blank());
        let g = &mut *g;
        grid(g);
        assert!(!nav_grid_walkable(g, [5.0, 0.0, 5.0], [20.0, 0.0, 5.0]));
        assert!(nav_grid_walkable(g, [5.0, 0.0, 5.0], [5.0, 0.0, 19.9]));
        assert!(nav_grid_is_blocked(g, -1.0, 5.0));
        assert!(!nav_grid_is_blocked(g, f64::NAN, 5.0));
        assert_eq!(nav_grid_idx(g, 3.0, 2.0), 163.0);
        // (no point asked for: the ring's first free cell, and no line is asked)
        assert_eq!(nav_grid_nearest_free(g, 30.0, 5.0, None, None), Some([29.0, 4.0]));
    }

    #[test]
    fn cells_keep_what_a_cut_leaves() {
        let mut cells = vec![vec![0.0, 5.0, 2.0, 7.0], vec![6.0]];
        cut_cells(&mut cells, 5.0);
        assert_eq!(cells, vec![vec![0.0, 2.0], vec![]]);
    }

    #[test]
    fn a_mistake_is_said_in_the_typescripts_words() {
        let tail = " — where it starts, the way it goes and how far are numbers, and how far is finite (a way worked out of two points that are one point is none)";
        assert_eq!(no_number(None, None, None), alloc::format!("a ray that is no number{}", tail));
        assert_eq!(no_number(Some([1.5, 0.0, -0.0]), Some([f64::NAN, 1.0, 2.0]), Some(3.0)), alloc::format!("a ray that is no number: from [1.5, 0, 0] along [NaN, 1, 2] for 3{}", tail));
        assert_eq!(handed_back_words("sound", "why", &[1.0, 2.5, 3.0], &[4.0, 5.0, 6.0]), "sound: the Rust module handed the question back (why): [1, 2.5, 3] → [4, 5, 6]");
    }

    #[test]
    fn a_question_says_how_the_rows_are_read() {
        let mut g = Box::new(Game::blank());
        g.sim = 2;
        let rows = rows_for(&g, Some(LampRows { leave: 4.0, steady: true })).unwrap();
        assert_eq!((rows.g, rows.leave, rows.steady), (2, 4, true));
        assert!(rows_for(&g, None).is_none());
    }

    #[test]
    fn small_ones() {
        assert_eq!(clampi(-1.0, 0.0, 3.0), 0.0);
        assert_eq!(clampi(4.0, 0.0, 3.0), 3.0);
        assert!(clampi(f64::NAN, 0.0, 3.0).is_nan());
    }
}
