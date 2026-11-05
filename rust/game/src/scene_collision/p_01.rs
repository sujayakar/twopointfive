//! src/scene/collision.ts lines 1 to 1192
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

use crate::v3::quat;
use sim::world::ASK_HOST;

// ---- small private helpers

/// which of the module's worlds is this game's: M.slot, which the mirror's `take` names as the copy first stands
#[inline]
fn slot(g: &Game) -> usize {
    g.col.mirror.slot as usize
}
/// a box's flags as a word
#[inline]
fn flags_of(g: &Game, b: BoxId) -> u32 {
    js::to_u32(g.col.boxes[b].flags)
}
/// every row of the box table, in order: a box's row in the world is its ID's row
fn all_rows(g: &Game) -> Vec<BoxId> {
    g.col.boxes.ids().collect()
}
/// a double of one of the module's tables, by its byte address and its place
#[inline]
fn put_f64(addr: usize, i: usize, v: f64) {
    unsafe { *(addr as *mut f64).add(i) = v }
}
#[inline]
fn get_f64(addr: usize, i: usize) -> f64 {
    unsafe { *(addr as *const f64).add(i) }
}
#[inline]
fn get_u32(addr: usize, i: usize) -> u32 {
    unsafe { *(addr as *const u32).add(i) }
}
/// `Object.is`: a −0 is not a 0, and a NaN is any NaN
#[inline]
fn is(x: f64, y: f64) -> bool {
    x.to_bits() == y.to_bits() || (x.is_nan() && y.is_nan())
}
/// a point as `a.join(', ')` writes it, for a check's line (a check's words, so Rust's printing of a double will do)
fn joined(a: V3) -> String {
    alloc::format!("{}, {}, {}", a[0], a[1], a[2])
}
/// the same function lent on again, for one call
fn relend<'a>(p: &'a mut Option<&mut dyn FnMut(&mut Game, f64, f64) -> f64>) -> Option<&'a mut dyn FnMut(&mut Game, f64, f64) -> f64> {
    match p {
        Some(f) => Some(&mut **f),
        None => None,
    }
}
/// the block's stamp (SimState.stamp): the tables made from the game's state are kept by it. The skeleton has no SimState: `st`, `Q.sim` are Opaque.
fn seam_sim_stamp(g: &mut Game) -> f64 {
    todo!("needs: SimState.stamp of the game's block (collision.ts:594, :659; skeleton has st: Opaque)")
}

// ---- the line through a box

/// the interval of the line a + t·rd inside box b, and how deep the origin sits. (collision.ts lineBox)
pub fn line_box(g: &mut crate::game::Game, a: V3, rd: V3, b: BoxId) -> Option<crate::scene_collision::LineBox> {
    let (ox, oy, oz, dx, dy, dz);
    let bc = g.col.boxes[b].c;
    if let Some(rot) = g.col.boxes[b].rot {
        let qi = quat::conj(rot);
        let o = quat::rotate(qi, v3::sub(a, bc));
        let d = quat::rotate(qi, rd);
        ox = o[0]; oy = o[1]; oz = o[2]; dx = d[0]; dy = d[1]; dz = d[2];
    } else {
        let yaw = g.col.boxes[b].yaw;
        let (cs, sn) = (m::cos(yaw), m::sin(yaw));
        let (px, pz) = (a[0] - bc[0], a[2] - bc[2]);
        ox = cs * px - sn * pz; oy = a[1] - bc[1]; oz = sn * px + cs * pz;
        dx = cs * rd[0] - sn * rd[2]; dy = rd[1]; dz = sn * rd[0] + cs * rd[2];
    }
    let h = g.col.boxes[b].h;
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
    let mut depth = m::min_n(&[h[0] - m::abs(ox), h[1] - m::abs(oy), h[2] - m::abs(oz)]);
    if g.col.boxes[b].round == Some(true) {
        let (qa, qb, qc) = (dx * dx + dz * dz, ox * dx + oz * dz, ox * ox + oz * oz - h[0] * h[0]);
        if qa < 1e-18 {
            if qc > 0.0 { return None; }
        } else {
            let disc = qb * qb - qa * qc;
            if disc < 0.0 { return None; }
            let s = m::sqrt(disc);
            let (ta, tb) = ((-qb - s) / qa, (-qb + s) / qa);
            if ta > t0 { t0 = ta; }
            if tb < t1 { t1 = tb; }
            if t0 > t1 { return None; }
        }
        depth = m::min(h[0] - m::sqrt(ox * ox + oz * oz), h[1] - m::abs(oy));
    } else if g.col.boxes[b].cut.is_some() && crate::scene_boxes::cut_met(g, b) {
        let cut = g.col.boxes[b].cut.clone().unwrap();
        if !crate::scene_cut::cut_clip(g, &cut, ox, oy, oz, dx, dy, dz, t0, t1) { return None; }
        t0 = g.m.scene_cut.cut_clip.t0;
        t1 = g.m.scene_cut.cut_clip.t1;
        let mut k = 0;
        while k < cut.len() && (k as f64) < crate::scene_cut::MAX_CUT_PLANES {
            let c = &cut[k];
            if c.n[0] == 0.0 && c.n[1] == 0.0 && c.n[2] == 0.0 { break; }
            depth = m::min(depth, c.d - (c.n[0] * ox + c.n[1] * oy + c.n[2] * oz));
            k += 1;
        }
    }
    Some(crate::scene_collision::LineBox { t0, t1, depth })
}

/// where a span's foot is asked from: a test that begins at the feet begins at the bake's height. UNDERFOOT.on is settled on. (collision.ts footBand)
pub fn foot_band(g: &mut crate::game::Game, y0: f64) -> f64 {
    if y0 >= FEET_Y && y0 < BAKE_Y { BAKE_Y } else { y0 }
}

/// a ray that is no number, as a value. The skeleton's Opaque holds no words: the raycast says them itself. (collision.ts noNumber)
pub fn no_number(ro: Option<V3>, rd: Option<V3>, tmax: Option<f64>) -> crate::Opaque {
    crate::Opaque
}

/// a sense's row by its bit. (collision.ts senseName)
pub fn sense_name(bit: f64) -> String {
    let s = crate::scene_box_mirror::senses();
    let rows: [(&str, f64); 5] = [("sight", s.sight), ("light", s.light), ("smoke", s.smoke), ("sound", s.sound), ("paths", s.paths)];
    for (k, v) in rows.iter() {
        if *v == bit { return String::from(*k); }
    }
    String::from("a sense")
}

// ---- StaticCollision: the grid, as the boxes are filed in it

/// the box world stood up: the cells from `boxes`, the rows written, the walk grid baked. `g.col = new(g, …)`: the boxes are already rows of g.col.boxes,
/// and everything else of g.col is new. (collision.ts StaticCollision.constructor)
pub fn new(g: &mut crate::game::Game, boxes: &[BoxId], of: Option<&str>) -> crate::scene_collision::StaticCollision {
    let table = core::mem::take(&mut g.col.boxes);
    g.col = StaticCollision::blank();
    g.col.boxes = table;
    g.col.mirror.of = of.map(String::from);
    let wd = crate::scene_world::world();
    g.col.w = wd.grid_w;
    g.col.h = wd.grid_h;
    g.col.cs = wd.grid_cell;
    let (gw, gh, gc) = (g.col.w, g.col.h, g.col.cs);
    g.col.cells = vec![Vec::new(); (gw * gh) as usize];
    let skip = js::to_u32(crate::scene_boxes::box_flag().no_shadow) | js::to_u32(crate::scene_boxes::box_flag().dynamic);
    for &b in boxes.iter() {
        if flags_of(g, b) & skip != 0 { continue; }
        // (a box's index is its ID: TRAP, see the notes)
        let i = b.ix() as f64;
        let (yaw, h, c, tilted) = { let r = &g.col.boxes[b]; (r.yaw, r.h, r.c, r.rot.is_some()) };
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
    g.col.nav = nav_grid_new(g);
    g.col.sound = sound_paths_new(g);
    core::mem::take(&mut g.col)
}

/// a static appended in play: `b` (a row already) filed in the cells of `footprint`'s plan. Its index is its row. (collision.ts StaticCollision.addStatic)
pub fn add_static(g: &mut crate::game::Game, b: BoxId, footprint: Option<BoxId>) -> f64 {
    let i = b.ix() as f64;
    crate::scene_box_mirror::box_mirror_wrote_grid(g);
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
    i
}

/// the cells a static appended over `f` is filed in: x0, x1, z0, z1. (collision.ts StaticCollision.footCells)
pub fn foot_cells(g: &mut crate::game::Game, f: BoxId) -> [f64; 4] {
    let (yaw, h, c) = { let r = &g.col.boxes[f]; (r.yaw, r.h, r.c) };
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

/// how many entries of the cells that is. (collision.ts StaticCollision.footEntries)
pub fn foot_entries(g: &mut crate::game::Game, f: BoxId) -> f64 {
    let [x0, x1, z0, z1] = foot_cells(g, f);
    m::max(0.0, x1 - x0 + 1.0) * m::max(0.0, z1 - z0 + 1.0)
}

/// every static past the first `n` leaves the cells. The rows stay: a Tab never shrinks, and the table holds boxes that are no statics. (collision.ts StaticCollision.truncateStatics)
pub fn truncate_statics(g: &mut crate::game::Game, n: f64) {
    if (g.col.boxes.len() as f64) <= n { return; }
    cut_cells(&mut g.col.cells, n);
    crate::scene_box_mirror::box_mirror_wrote_grid(g);
}
/// every index from `n` on leaves the cells, the rest keeping their order
fn cut_cells(cells: &mut [Vec<f64>], n: f64) {
    for cell in cells.iter_mut() {
        cell.retain(|&i| i < n);
    }
}

// ---- the world's rows are the boxes

/// the walk's rule as the mirror is handed it: the only one hung is glassbreak's (GlassWalkable::GlassBreak)
fn walk_rule(g: &mut Game, bi: f64) -> bool {
    crate::glassbreak::glass_break_walkable(g, bi)
}
/// the rows good for a question, whoever asks: in a tick or a window they stand, and outside both they are proven for this one. (Rust's: every question here answers)
fn prove(g: &mut Game) {
    ready(g);
    if !g.col.mirror.live && lone(g) { crate::scene_box_mirror::box_mirror_shut(g); }
}
/// a question outside every tick and window is proven for itself, and shut behind it
#[inline]
fn alone(g: &mut Game) -> bool {
    !g.col.mirror.live && lone(g)
}

/// a tick opens: the rows are made the boxes as they stand. (collision.ts StaticCollision.tickOpen)
pub fn tick_open(g: &mut crate::game::Game) {
    stand(g);
    g.col.mirror.played = true;
}

/// the rows made the boxes as they stand, for whoever opens a stretch of questions. Every row of RUST is on: there is no other side. (collision.ts StaticCollision.stand)
pub fn stand(g: &mut crate::game::Game) {
    let ids = all_rows(g);
    let (cells, tilted) = (core::mem::take(&mut g.col.cells), core::mem::take(&mut g.col.tilted_at));
    let (gw, gh, gc) = (g.col.w, g.col.h, g.col.cs);
    let leaves = g.col.dynamic_boxes.clone();
    let glass = glass_panes(g);
    let s = crate::scene_box_mirror::senses();
    let senses = s.sight + s.light + s.smoke + s.sound + s.paths;
    let mut rule = walk_rule;
    let walk: Option<&mut dyn FnMut(&mut Game, f64) -> bool> = if g.col.glass_walkable.is_some() { Some(&mut rule) } else { None };
    crate::scene_box_mirror::box_mirror_open(g, &ids, &cells, &tilted, gw, gh, gc, &leaves, walk, glass.as_ref(), true, true, true, Some(senses), Some(true));
    g.col.cells = cells;
    g.col.tilted_at = tilted;
}

/// the world a rule of the module's steps over, with the game's own leaves `dyn_` in list 0 as they are. (collision.ts StaticCollision.rulesWorld, boxMirror.ts rules)
pub fn rules_world(g: &mut crate::game::Game, dyn_: &[BoxId]) {
    let was_alone = alone(g);
    ready(g);
    if !crate::scene_box_mirror::box_mirror_rules(g, dyn_) {
        panic!("another list in the leaves' place, or leaves that do not fit a list");
    }
    if was_alone { crate::scene_box_mirror::box_mirror_shut(g); }
}

/// the world a rule asks of the statics alone. (collision.ts StaticCollision.rulesStatics)
pub fn rules_statics(g: &mut crate::game::Game) {
    prove(g);
}

/// the pane rule as a rule's step takes it. (collision.ts StaticCollision.rulesPanes)
pub fn rules_panes(g: &mut crate::game::Game) -> f64 {
    pane_ask(g)
}

/// why the copy is not answering now, for the count of questions the TypeScript took. (collision.ts StaticCollision.fellWhy)
pub fn fell_why(g: &mut crate::game::Game) -> String {
    String::from(if g.col.mirror.live { "the row was not up when the tick opened" } else { "outside a tick" })
}

/// the foot of a span as the statics are asked it. (collision.ts StaticCollision.footOf)
pub fn foot_of(g: &mut crate::game::Game, y0: f64) -> f64 {
    foot_band(g, y0)
}

/// a tick shuts. (collision.ts StaticCollision.tickShut)
pub fn tick_shut(g: &mut crate::game::Game) {
    if g.col.mirror.live { crate::scene_box_mirror::box_mirror_shut(g); }
}

/// `fn_`'s questions share one proof of the rows, made by the first of them. (collision.ts StaticCollision.window)
pub fn window<T: Clone + core::fmt::Debug>(g: &mut crate::game::Game, fn_: &mut dyn FnMut(&mut crate::game::Game) -> T) -> T {
    if g.col.mirror.live || g.col.windowed { return fn_(g); }
    g.col.windowed = true;
    let r = fn_(g);
    g.col.windowed = false;
    if g.col.mirror.live { crate::scene_box_mirror::box_mirror_shut(g); }
    r
}

/// the rows ready for a question: a window's first question proves them, and a debt is paid. (collision.ts StaticCollision.ready)
pub fn ready(g: &mut crate::game::Game) {
    if !g.col.mirror.live && g.col.windowed { lone(g); }
    if g.col.mirror.owes {
        let ids = all_rows(g);
        let mut rule = walk_rule;
        let walk: Option<&mut dyn FnMut(&mut Game, f64) -> bool> = if g.col.glass_walkable.is_some() { Some(&mut rule) } else { None };
        crate::scene_box_mirror::box_mirror_again(g, &ids, walk);
    }
}

/// a question outside a tick: true = the rows stand for it alone, and the caller shuts them behind it. (collision.ts StaticCollision.lone)
pub fn lone(g: &mut crate::game::Game) -> bool {
    stand(g);
    if g.col.windowed {
        g.m.scene_collision.anytime.windows += 1.0;
        return false;
    }
    g.m.scene_collision.anytime.proofs += 1.0;
    true
}

/// a static was written inside a tick: the next question compares them again. (collision.ts StaticCollision.wrote, boxMirror.ts wrote)
pub fn wrote(g: &mut crate::game::Game) {
    crate::scene_box_mirror::box_mirror_wrote(g);
}

/// a question the picture asks: searches on the asker's tables, no sound's route kept. The stamps are the module's now. (collision.ts StaticCollision.aside)
pub fn aside<T: Clone + core::fmt::Debug>(g: &mut crate::game::Game, own: &mut crate::scene_collision::AsideOwn, fn_: &mut dyn FnMut(&mut crate::game::Game) -> T) -> T {
    let asked = g.col.sound.asking;
    g.col.sound.asking = true;
    let mut nav_own = NavGridAsideOwn { s: own.s.take() };
    let r = nav_grid_aside(g, &mut nav_own, fn_);
    own.s = nav_own.s;
    g.col.sound.asking = asked;
    r
}

// ---- circles against the boxes (rust/sim collide.rs)

/// the statics filed under a rectangle that overlap a span, once each, into `out`. (collision.ts StaticCollision.gather)
pub fn gather(g: &mut crate::game::Game, x0: f64, z0: f64, x1: f64, z1: f64, y0: f64, y1: f64, out: &mut Vec<i32>) -> f64 {
    if alone(g) {
        let r = gather_ask(g, x0, z0, x1, z1, y0, y1, out);
        crate::scene_box_mirror::box_mirror_shut(g);
        return r;
    }
    gather_ask(g, x0, z0, x1, z1, y0, y1, out)
}

/// (collision.ts StaticCollision.gatherAsk)
pub fn gather_ask(g: &mut crate::game::Game, x0: f64, z0: f64, x1: f64, z1: f64, y0: f64, y1: f64, out: &mut Vec<i32>) -> f64 {
    ready(g);
    gather_here(g, x0, z0, x1, z1, y0, y1, out)
}

/// TWIN: the module's gather, its list read into `out`. (collision.ts StaticCollision.gatherHere)
pub fn gather_here(g: &mut crate::game::Game, x0: f64, z0: f64, x1: f64, z1: f64, y0: f64, y1: f64, out: &mut Vec<i32>) -> f64 {
    let w = slot(g);
    let n = sim::collide::gather(w, x0, z0, x1, z1, y0, y1, out.len());
    let list = &sim::world::world(w).list;
    for i in 0..n {
        out[i] = list[i] as i32;
    }
    n as f64
}

/// the statics a circle touches over a span: how many, −1 = `out` too short. (collision.ts StaticCollision.circleTouches)
pub fn circle_touches(g: &mut crate::game::Game, x: f64, z: f64, r: f64, y0: f64, y1: f64, out: &mut Vec<i32>) -> f64 {
    if alone(g) {
        let k = circle_touches_ask(g, x, z, r, y0, y1, out);
        crate::scene_box_mirror::box_mirror_shut(g);
        return k;
    }
    circle_touches_ask(g, x, z, r, y0, y1, out)
}

/// (collision.ts StaticCollision.circleTouchesAsk)
pub fn circle_touches_ask(g: &mut crate::game::Game, x: f64, z: f64, r: f64, y0: f64, y1: f64, out: &mut Vec<i32>) -> f64 {
    ready(g);
    circle_touches_here(g, x, z, r, y0, y1, out)
}

/// TWIN: the module's touches; `out` is left as the two steps leave it (the touching ones, then what the gather put past them). (collision.ts StaticCollision.circleTouchesHere)
pub fn circle_touches_here(g: &mut crate::game::Game, x: f64, z: f64, r: f64, y0: f64, y1: f64, out: &mut Vec<i32>) -> f64 {
    let w = slot(g);
    let y0 = foot_band(g, y0);
    let cap = if out.len() > sim::world::MAX_LIST { sim::world::MAX_LIST } else { out.len() };
    let v = sim::collide::touches(w, x, z, r, y0, y1, cap);
    let full = v < 0;
    let (n, k) = if full { (cap, -1.0) } else { ((v as u32 & 0xffff) as usize, (v as u32 >> 16) as f64) };
    let list = &sim::world::world(w).list;
    for i in 0..n {
        out[i] = list[i] as i32;
    }
    k
}

/// a circle pushed out of the statics over a span; true = it touched. (collision.ts StaticCollision.collideCircle)
pub fn collide_circle(g: &mut crate::game::Game, p: &mut V3, r: f64, y0: f64, y1: f64, iterations: Option<f64>) -> bool {
    if alone(g) {
        let hit = collide_circle_ask(g, p, r, y0, y1, iterations);
        crate::scene_box_mirror::box_mirror_shut(g);
        return hit;
    }
    collide_circle_ask(g, p, r, y0, y1, iterations)
}

/// (collision.ts StaticCollision.collideCircleAsk)
pub fn collide_circle_ask(g: &mut crate::game::Game, p: &mut V3, r: f64, y0: f64, y1: f64, iterations: Option<f64>) -> bool {
    let iterations = iterations.unwrap_or(3.0);
    ready(g);
    collide_circle_here(g, p, r, y0, y1, iterations)
}

/// TWIN: the module's push, and where it ended read back. (collision.ts StaticCollision.collideCircleHere)
pub fn collide_circle_here(g: &mut crate::game::Game, p: &mut V3, r: f64, y0: f64, y1: f64, iterations: f64) -> bool {
    let w = slot(g);
    let y0 = foot_band(g, y0);
    let hit = sim::collide::circle(w, p[0], p[2], r, y0, y1, js::to_u32(iterations)) != 0;
    let pt = sim::world::world(w).pt;
    p[0] = pt[0];
    p[2] = pt[1];
    hit
}

/// a circle kept on the world's floor rectangle; true = it had to move. (collision.ts StaticCollision.clampToWorld)
pub fn clamp_to_world(g: &mut crate::game::Game, p: &mut V3, r: f64) -> bool {
    let wd = crate::scene_world::world();
    let x = m::min(wd.max[0] - r, m::max(wd.min[0] + r, p[0]));
    let z = m::min(wd.max[2] - r, m::max(wd.min[2] + r, p[2]));
    let moved = x != p[0] || z != p[2];
    p[0] = x;
    p[2] = z;
    moved
}

// ---- the glass

/// has the game a pane rule. Its question is the module's now, and glassbreak's for a rule of its own. (collision.ts StaticCollision.glassOpen)
pub fn glass_open(g: &mut crate::game::Game) -> Option<crate::FnField> {
    g.col.pane_rule.as_ref().map(|_| crate::FnField)
}

/// the pane rule and what it answers by, hung together. The only rule is glassbreak's: what is kept is the enum, and the closures say only whether there is one. (collision.ts StaticCollision.glassRule)
pub fn glass_rule(g: &mut crate::game::Game, rule: Option<&mut dyn FnMut(&mut crate::game::Game, f64, V3) -> bool>, rows: Option<&mut dyn FnMut(&mut crate::game::Game) -> crate::scene_box_mirror::PaneSource>) {
    let hung = rule.is_some();
    g.col.pane_rule = if hung { Some(PaneRule::GlassBreak) } else { None };
    g.col.pane_rows = if hung && rows.is_some() { Some(PaneRows::GlassBreak) } else { None };
    let glass = glass_panes(g);
    crate::scene_box_mirror::box_mirror_glass_is(g, glass.as_ref());
}

/// the game's glass as the world's rows read it, or none. (collision.ts StaticCollision.glassPanes)
pub fn glass_panes(g: &mut crate::game::Game) -> Option<crate::scene_box_mirror::PaneSource> {
    match g.col.pane_rows {
        Some(PaneRows::GlassBreak) => Some(crate::scene_box_mirror::PaneSource { view: crate::glassbreak::glass_break_view(g) }),
        None => None,
    }
}

/// the glass is being worked on: every question owes the panes' rows, until the first after `glass_end`. (collision.ts StaticCollision.glassBegin)
pub fn glass_begin(g: &mut crate::game::Game) {
    crate::scene_box_mirror::box_mirror_glass_begin(g);
}

/// (collision.ts StaticCollision.glassEnd)
pub fn glass_end(g: &mut crate::game::Game) {
    crate::scene_box_mirror::box_mirror_glass_end(g);
}

/// the glass was written with no question of its own between. (collision.ts StaticCollision.glassWrote)
pub fn glass_wrote(g: &mut crate::game::Game) {
    crate::scene_box_mirror::box_mirror_glass_wrote(g);
}

/// a burst's view of the glass was put on or taken off. (collision.ts StaticCollision.glassViewed)
pub fn glass_viewed(g: &mut crate::game::Game) {
    crate::scene_box_mirror::box_mirror_viewed(g);
}

/// the pane rule as a question takes it: 0 none, 1 it has one and rows behind it, −1 a rule with no rows. EXACT.on is settled for the module, so no −2. (collision.ts StaticCollision.paneAsk)
pub fn pane_ask(g: &mut crate::game::Game) -> f64 {
    if g.col.pane_rule.is_none() {
        0.0
    } else if g.col.pane_rows.is_some() {
        1.0
    } else {
        -1.0
    }
}

/// a question the TypeScript took for it, counted where the module should have had it. (collision.ts StaticCollision.paneFell)
pub fn pane_fell(g: &mut crate::game::Game, row: &str, ask: f64) {
    if g.m.sim_sim.fell.on && ask == -1.0 {
        let why = if g.col.pane_rows.is_none() { "no glass stands behind glassOpen" } else { "more broken panes, or a view of more panes, than the copy holds" };
        crate::sim_sim::fell(row, &alloc::format!("a pane's rule the module cannot judge by: {}", why));
    }
}

// ---- lines

/// which of the world's lists holds the leaves to ask with (−1 none). No answer but the module's, so more leaves than a list is a mistake by name.
fn leaves_set(g: &mut Game) -> i32 {
    if g.col.dynamic_boxes.is_empty() { return -1; }
    let d = g.col.dynamic_boxes.clone();
    let set = crate::scene_box_mirror::box_mirror_leaves(g, &d);
    if set == -2.0 { panic!("{} leaves, and a list of the box world holds {}", d.len(), sim::world::MAX_LEAVES); }
    set as i32
}
/// the pane rule for a question that will be answered; below nought there is nobody else to answer
fn ask_of(g: &mut Game) -> u32 {
    let ask = pane_ask(g);
    if ask < 0.0 { panic!("a pane's rule the module cannot judge by: no glass stands behind it"); }
    ask as u32
}

/// a leaf is across a→b; with `need_flags`, one carrying those. (collision.ts StaticCollision.segmentBlockedDynamic)
pub fn segment_blocked_dynamic(g: &mut crate::game::Game, a: V3, b: V3, need_flags: Option<f64>) -> bool {
    if alone(g) {
        let r = segment_blocked_dynamic_ask(g, a, b, need_flags);
        crate::scene_box_mirror::box_mirror_shut(g);
        return r;
    }
    segment_blocked_dynamic_ask(g, a, b, need_flags)
}

/// (collision.ts StaticCollision.segmentBlockedDynamicAsk)
pub fn segment_blocked_dynamic_ask(g: &mut crate::game::Game, a: V3, b: V3, need_flags: Option<f64>) -> bool {
    let need = need_flags.unwrap_or(0.0);
    let (d0, d1, d2) = (b[0] - a[0], b[1] - a[1], b[2] - a[2]);
    let len = m::hyp3(d0, d1, d2);
    if len < 1e-4 { return false; }
    ready(g);
    if !(len < 1e6) { return false; }   // (TRAP: a length that is no length of this world's; the walk it went to answered no)
    debug_assert!(js::to_u32(need) & !js::to_u32(crate::scene_box_mirror::mirrored()) == 0, "a flag the box world does not keep");
    let set = leaves_set(g);
    set >= 0 && sim::lines::leaves_blocked(slot(g), a[0], a[1], a[2], b[0], b[1], b[2], len, js::to_u32(need), set)
}

/// the furniture's ray: the nearest part met. None = nothing met (the TypeScript's undefined, the caller's own walk, is gone). (collision.ts StaticCollision.thingsRay)
pub fn things_ray(g: &mut crate::game::Game, p: &crate::scene_box_mirror::Furniture, ro: V3, rd: V3, tmax: f64) -> Option<crate::scene_collision::ThingsRay> {
    let r = if alone(g) {
        let r = things_ask(g, p, ro, rd, tmax);
        crate::scene_box_mirror::box_mirror_shut(g);
        r
    } else {
        things_ask(g, p, ro, rd, tmax)
    };
    r.map(|t| ThingsRay { row: t.row, t: t.t, n: t.n, box_: t.box_ })
}

/// (collision.ts StaticCollision.thingsAsk)
pub fn things_ask(g: &mut crate::game::Game, p: &crate::scene_box_mirror::Furniture, ro: V3, rd: V3, tmax: f64) -> Option<crate::scene_collision::ThingsAsk> {
    ready(g);
    if !crate::scene_box_mirror::box_mirror_things(g, p) {
        panic!("more furniture than the box world holds: {} things", p.all.len());
    }
    let mut out = [0.0; 5];
    let k = sim::lines::ray_things(slot(g), ro[0], ro[1], ro[2], rd[0], rd[1], rd[2], tmax, &mut out);
    if k < 0 { return None; }
    let at = out[sim::lines::RAY_PART];
    let box_ = match crate::scene_box_mirror::box_mirror_thing_box(g, p, at) {
        Some(b) => b,
        None => panic!("the module's ray into the furniture met part {} of its list, which is no part of the furniture as it stands ({} things)", at, p.props.len()),
    };
    Some(ThingsAsk { row: k as f64, t: out[0], n: [out[1], out[2], out[3]], box_ })
}

/// the furniture's ray answered both ways: the module's against what the caller's own walk answered, and the copy against the furniture. A check's. (collision.ts StaticCollision.thingsBoth)
pub fn things_both(g: &mut crate::game::Game, p: &crate::scene_box_mirror::Furniture, ro: V3, rd: V3, tmax: f64, t: Option<crate::scene_collision::ThingsBothT>) {
    if !g.col.mirror.live || crate::scene_box_mirror::box_mirror_owes_boxes(g) || !crate::scene_box_mirror::box_mirror_things(g, p) { return; }
    let mut out = [0.0; 5];
    let k = sim::lines::ray_things(slot(g), ro[0], ro[1], ro[2], rd[0], rd[1], rd[2], tmax, &mut out);
    let ok = if k < 0 {
        t.is_none()
    } else {
        // (`P.props[k] === t.prop`: `prop` holds nothing here, and a box is one thing's part, so the box's compare answers that too)
        match &t {
            None => false,
            Some(h) => is(out[0], h.t) && is(out[1], h.n[0]) && is(out[2], h.n[1]) && is(out[3], h.n[2]) && crate::scene_box_mirror::box_mirror_thing_box(g, p, out[sim::lines::RAY_PART]) == Some(h.box_),
        }
    };
    let st = crate::scene_box_mirror::box_mirror_stale_things(g, p);
    let both = &mut g.m.scene_collision.both;
    both.asked += 1.0;
    both.things += 1.0;
    both.stale += st;
    if !ok { both.differ += 1.0; }
    if (js::truthy(st) || !ok) && both.first.is_empty() {
        both.first = alloc::format!("the furniture's ray [{}] → [{}] ({}, {}): {}{} row(s) of the copy not the furniture", joined(ro), joined(rd), tmax, k, if ok { "" } else { "the two answers differ; " }, st);
    }
}

/// the check's count of one question answered both ways, and of the copy against its boxes and panes. (collision.ts StaticCollision.compared)
pub fn compared(g: &mut crate::game::Game, set: f64, ok: bool, what: &str, a: V3, b: V3, p: f64, q: f64) {
    let (ids, leaves) = (all_rows(g), g.col.dynamic_boxes.clone());
    let st = crate::scene_box_mirror::box_mirror_stale(g, &ids, &leaves, set) + crate::scene_box_mirror::box_mirror_stale_panes(g);
    let both = &mut g.m.scene_collision.both;
    both.asked += 1.0;
    both.stale += st;
    if !ok { both.differ += 1.0; }
    if (js::truthy(st) || !ok) && both.first.is_empty() {
        both.first = alloc::format!("{} [{}] → [{}] ({}, {}): {}{} row(s) of the copy not their box or their pane", what, joined(a), joined(b), p, q, if ok { "" } else { "the two answers differ; " }, st);
    }
}

/// the check's line for a sense's question answered both ways. (collision.ts StaticCollision.senseCompared)
pub fn sense_compared(g: &mut crate::game::Game, set: f64, ok: bool, what: &str, a: V3, b: V3) {
    compared(g, set, ok, what, a, b, 0.0, 0.0);
}

/// can the segment from a along unit r for `len` be said, without the slab test, to come nowhere near box b (grown by `pad`). (collision.ts segClear)
pub fn seg_clear(g: &mut crate::game::Game, a0: f64, a1: f64, a2: f64, r0: f64, r1: f64, r2: f64, len: f64, b: BoxId, pad: f64) -> bool {
    let (c, h, tilted) = { let x = &g.col.boxes[b]; (x.c, x.h, x.rot.is_some()) };
    let hy = if h[1] < 0.0 { -h[1] } else { h[1] };
    let m1 = c[1] - a1;
    if !tilted {
        let e1 = r1 * len;
        let off = if e1 > 0.0 { m1 - hy > e1 + pad || m1 + hy < -pad } else { m1 - hy > pad || m1 + hy < e1 - pad };
        if off { return true; }
    }
    let rr = (if h[0] < 0.0 { -h[0] } else { h[0] }) + hy + (if h[2] < 0.0 { -h[2] } else { h[2] }) + pad;
    let (m0, m2) = (c[0] - a0, c[2] - a2);
    let s = m0 * r0 + m1 * r1 + m2 * r2;
    if s < -rr || s > len + rr { return true; }
    m0 * m0 + m1 * m1 + m2 * m2 - s * s > rr * rr
}

/// does the ray enter this yawed box within tmax: rayBox's own statements, stopping where it would build the normal. (collision.ts rayBoxHits)
pub fn ray_box_hits(g: &mut crate::game::Game, a0: f64, a1: f64, a2: f64, r0: f64, r1: f64, r2: f64, b: BoxId, tmax: f64) -> bool {
    let (c, h, yaw) = { let x = &g.col.boxes[b]; (x.c, x.h, x.yaw) };
    let (cs, sn) = (m::cos(yaw), m::sin(yaw));
    let (px, py, pz) = (a0 - c[0], a1 - c[1], a2 - c[2]);
    let (lx, lz, ly) = (cs * px - sn * pz, sn * px + cs * pz, py);
    let (dx, dz, dy) = (cs * r0 - sn * r2, sn * r0 + cs * r2, r1);
    let (ix, iy, iz) = (1.0 / js::or(dx, 1e-20), 1.0 / js::or(dy, 1e-20), 1.0 / js::or(dz, 1e-20));
    let (mut t0, mut t1) = (0.0, tmax);
    let mut entered = false;
    for (ax, l, inv) in [(0, lx, ix), (1, ly, iy), (2, lz, iz)] {
        let (mut ta, mut tb) = ((-h[ax] - l) * inv, (h[ax] - l) * inv);
        if ta > tb { core::mem::swap(&mut ta, &mut tb); }
        if ta > t0 { t0 = ta; entered = true; }
        if tb < t1 { t1 = tb; }
        if t0 > t1 { return false; }
    }
    entered
}

/// the leaves' own loop for a segment already reduced to origin, unit direction and length. (collision.ts StaticCollision.dynamicHit)
pub fn dynamic_hit(g: &mut crate::game::Game, a: V3, r0: f64, r1: f64, r2: f64, len: f64, need_flags: f64) -> bool {
    let (a0, a1, a2) = (a[0], a[1], a[2]);
    let uu = r0 * r0 + r2 * r2;
    let (su, lu) = (m::sqrt(uu), len * uu);
    let need = js::to_i32(need_flags);
    let mut i = 0;
    while i < g.col.dynamic_boxes.len() {
        let db = g.col.dynamic_boxes[i];
        i += 1;
        let (flags, c, h, tilted, round, cut) = { let x = &g.col.boxes[db]; (x.flags, x.c, x.h, x.rot.is_some(), x.round == Some(true), x.cut.is_some()) };
        if ((js::to_i32(flags) & need) as f64) != need_flags { continue; }
        if tilted || round || (cut && crate::scene_boxes::cut_met(g, db)) {
            if seg_clear(g, a0, a1, a2, r0, r1, r2, len, db, SEG_PAD) { continue; }
            if let Some(hit) = crate::scene_boxes::ray_box(g, a, [r0, r1, r2], db, len) {
                if hit.t >= 0.0 { return true; }
            }
            continue;
        }
        let rr = (if h[0] < 0.0 { -h[0] } else { h[0] }) + (if h[2] < 0.0 { -h[2] } else { h[2] }) + SEG_PAD;
        let (m0, m2) = (c[0] - a0, c[2] - a2);
        let cr = m0 * r2 - m2 * r0;
        if cr * cr > rr * rr * uu { continue; }
        let (s, e) = (m0 * r0 + m2 * r2, rr * su);
        if s < -e || s > lu + e { continue; }
        if ray_box_hits(g, a0, a1, a2, r0, r1, r2, db, len) { return true; }
    }
    false
}

/// the module's line a→b, past the proof. Panes in the way are added to `panes` where it is given (static, metres along). (collision.ts segmentBlockedAsk's call of lines_blocked)
fn line(g: &mut Game, a: V3, b: V3, shrink: f64, through: f64, panes: Option<&mut Vec<(f64, f64)>>) -> bool {
    let (a0, a1, a2) = (a[0], a[1], a[2]);
    let (d0, d1, d2) = (b[0] - a0, b[1] - a1, b[2] - a2);
    let len = m::hyp3(d0, d1, d2);
    if len < 1e-4 { return false; }
    if !(len < 1e6) { return false; }   // (TRAP: a length that is no length of this world's; the walk it went to answered no)
    let thr = js::to_u32(through);
    debug_assert!(thr & !js::to_u32(crate::scene_box_mirror::mirrored()) == 0, "a flag the box world does not keep");
    let set = leaves_set(g);
    let ask = if thr & sim::world::F_GLASS != 0 { 0 } else { ask_of(g) };
    let w = slot(g);
    let word = ask | if panes.is_some() { sim::lines::ASK_PANES } else { 0 };
    let r = sim::lines::blocked(w, a0, a1, a2, b[0], b[1], b[2], len, shrink, thr, set, word);
    if r == -2 { panic!("more glass on one line than the box world lists ({})", sim::world::MAX_PANES); }
    if r > 0 {
        if let Some(out) = panes {
            let o = sim::world::world(w);
            for i in 0..r as usize {
                out.push((o.pane_bi[i] as f64, o.pane_t[i]));
            }
        }
    }
    r == -1
}

/// is a→b blocked by a static or a leaf; `through` = the flags it passes (glass by default: the line of sight). (collision.ts StaticCollision.segmentBlocked)
pub fn segment_blocked(g: &mut crate::game::Game, a: V3, b: V3, shrink: Option<f64>, through: Option<f64>, panes: Option<&mut Vec<crate::scene_collision::SegmentBlockedPanesItem>>) -> bool {
    let (shrink, through) = (shrink.unwrap_or(0.02), through.unwrap_or(crate::scene_boxes::box_flag().glass));
    let was_alone = alone(g);
    ready(g);
    let r = match panes {
        Some(out) => {
            let mut got = Vec::new();
            let r = line(g, a, b, shrink, through, Some(&mut got));
            for (bi, t) in got { out.push(SegmentBlockedPanesItem { bi, t }); }
            r
        }
        None => line(g, a, b, shrink, through, None),
    };
    if was_alone { crate::scene_box_mirror::box_mirror_shut(g); }
    r
}

/// (collision.ts StaticCollision.segmentBlockedAsk)
pub fn segment_blocked_ask(g: &mut crate::game::Game, a: V3, b: V3, shrink: Option<f64>, through: Option<f64>, panes: Option<&mut Vec<crate::scene_collision::SegmentBlockedAskPanesItem>>) -> bool {
    let (shrink, through) = (shrink.unwrap_or(0.02), through.unwrap_or(crate::scene_boxes::box_flag().glass));
    ready(g);
    match panes {
        Some(out) => {
            let mut got = Vec::new();
            let r = line(g, a, b, shrink, through, Some(&mut got));
            for (bi, t) in got { out.push(SegmentBlockedAskPanesItem { bi, t }); }
            r
        }
        None => line(g, a, b, shrink, through, None),
    }
}

/// is the line test this class's own: a check's override has no Rust. (collision.ts StaticCollision.ownLines)
pub fn own_lines(g: &mut crate::game::Game) -> bool {
    true
}

/// TWIN: the module's line. (collision.ts StaticCollision.segmentBlockedHere)
pub fn segment_blocked_here(g: &mut crate::game::Game, a: V3, b: V3, shrink: f64, through: f64, panes: Option<&mut Vec<crate::scene_collision::SegmentBlockedHerePanesItem>>) -> bool {
    match panes {
        Some(out) => {
            let mut got = Vec::new();
            let r = line(g, a, b, shrink, through, Some(&mut got));
            for (bi, t) in got { out.push(SegmentBlockedHerePanesItem { bi, t }); }
            r
        }
        None => line(g, a, b, shrink, through, None),
    }
}

/// segmentBlocked with nothing passed: the solid test. (collision.ts StaticCollision.solidBlocked)
pub fn solid_blocked(g: &mut crate::game::Game, a: V3, b: V3, shrink: Option<f64>) -> bool {
    segment_blocked(g, a, b, Some(shrink.unwrap_or(0.02)), Some(0.0), None)
}

/// the line of fire: anything a round would stop in. (collision.ts StaticCollision.shotBlocked)
pub fn shot_blocked(g: &mut crate::game::Game, a: V3, b: V3, shrink: Option<f64>, energy: Option<f64>) -> bool {
    let shrink = shrink.unwrap_or(0.02);
    let pass = match g.col.fire_pass {
        Some(FirePass::Bars) => if crate::winbars::bars_live(g) { crate::scene_boxes::box_flag().bar } else { 0.0 },
        None => 0.0,
    };
    if !crate::glasspen::glass_pen_live(g) {
        return segment_blocked(g, a, b, Some(shrink), Some(pass), None);
    }
    let mut panes: Vec<SegmentBlockedPanesItem> = Vec::new();
    if segment_blocked(g, a, b, Some(shrink), Some(pass), Some(&mut panes)) { return true; }
    if panes.is_empty() { return false; }
    js::sort_by(&mut panes, |x, y| x.t - y.t);
    let list: Vec<crate::glasspen::ShotPassesPanesItem> = panes.iter().map(|p| crate::glasspen::ShotPassesPanesItem { bi: p.bi, t: p.t }).collect();
    let d = v3::sub(b, a);
    !crate::glasspen::shot_passes(g, a, v3::scale(d, 1.0 / v3::len(d)), &list, energy)
}

// ---- rays

/// the closest static or leaf along a ray; a leaf's index is 1e6 + its place. (collision.ts StaticCollision.raycast)
pub fn raycast(g: &mut crate::game::Game, ro: V3, rd: V3, tmax: f64) -> Option<crate::scene_collision::Raycast> {
    if !(tmax > 0.0) { return None; }
    let r = if alone(g) {
        let r = raycast_ask(g, ro, rd, tmax);
        crate::scene_box_mirror::box_mirror_shut(g);
        r
    } else {
        raycast_ask(g, ro, rd, tmax)
    };
    r.map(|h| Raycast { t: h.t, index: h.index, n: h.n })
}

/// (collision.ts StaticCollision.raycastAsk)
pub fn raycast_ask(g: &mut crate::game::Game, ro: V3, rd: V3, tmax: f64) -> Option<crate::scene_collision::RaycastAsk> {
    ready(g);
    let r = raycast_here(g, ro, rd, tmax);
    g.m.scene_collision.rays.n += 1.0;
    r.map(|h| RaycastAsk { t: h.t, index: h.index, n: h.n })
}

/// TWIN: the module's ray. A ray that is no number is the caller's mistake, by name (noNumber). (collision.ts StaticCollision.raycastHere)
pub fn raycast_here(g: &mut crate::game::Game, ro: V3, rd: V3, tmax: f64) -> Option<crate::scene_collision::RaycastHere> {
    let ask = ask_of(g) != 0;
    let set = leaves_set(g);
    let mut out = [0.0; 4];
    let k = sim::lines::ray(slot(g), ro[0], ro[1], ro[2], rd[0], rd[1], rd[2], tmax, set, ask, &mut out);
    if k == sim::lines::RAY_HOST {
        panic!("a ray that is no number: from {:?} along {:?} for {}: where it starts, the way it goes and how far are numbers, and how far is finite", ro, rd, tmax);
    }
    if k < 0 { return None; }
    Some(RaycastHere { t: out[0], index: k as f64, n: [out[1], out[2], out[3]] })
}

// ---- the senses in the module (rust/sim fog.rs, sight.rs, light.rs, smoke.rs, bricks.rs)

/// a sight pass opens: may its questions go to the module (no more eyes than a question names). (collision.ts StaticCollision.sightOpen)
pub fn sight_open(g: &mut crate::game::Game, n: f64) -> bool {
    prove(g);
    n <= SIGHT_EYES
}

/// eye `k` of the pass. (collision.ts StaticCollision.sightEye)
pub fn sight_eye(g: &mut crate::game::Game, k: f64, pos: V3, look: V3, look_len: f64, cos_half: f64, near: f64, full: bool) {
    let at = sim::fog::eyes_addr(slot(g));
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

/// which eyes have a line: how many × 2²⁸ + those eyes, four bits each; −1 = the caller walks its own lines. (collision.ts StaticCollision.eyesSee)
pub fn eyes_see(g: &mut crate::game::Game, order: f64, n: f64, want: f64, kind: f64, through: f64, a: V3, b: V3) -> f64 {
    prove(g);
    let thr = js::to_u32(through);
    if thr & !js::to_u32(crate::scene_box_mirror::mirrored()) != 0 || thr & sim::world::F_GLASS == 0 { return -1.0; }
    let set = leaves_set(g);
    let r = sim::fog::see(slot(g), set, js::to_u32(order), js::to_u32(n), js::to_u32(want), js::to_u32(kind), thr, a[0], a[1], a[2], b[0], b[1], b[2]);
    if r == ASK_HOST { return -1.0; }
    r as f64
}

/// may a sense be asked of the module now: always, and which list holds the leaves (−1 none). (collision.ts StaticCollision.senseSet)
pub fn sense_set(g: &mut crate::game::Game, bit: f64) -> f64 {
    prove(g);
    leaves_set(g) as f64
}

/// an eye on a man's chest and head as one question. None = the caller's own statements (a line that stops at glass). (collision.ts StaticCollision.eyeContact)
pub fn eye_contact(g: &mut crate::game::Game, e: crate::scene_collision::EyeContactE, chest: V3, head: V3, unseen: bool, smoked: bool, through: f64) -> Option<crate::scene_collision::EyeContact> {
    let thr = js::to_u32(through);
    if thr & !js::to_u32(crate::scene_box_mirror::mirrored()) != 0 || thr & sim::world::F_GLASS == 0 { return None; }
    let q = EyeContactAskE { pos: e.pos, look: e.look, range: e.range, half_deg: e.half_deg, near_m: e.near_m, near_deg: e.near_deg, v_half_deg: e.v_half_deg };
    let r = if alone(g) {
        let r = eye_contact_ask(g, q, chest, head, unseen, smoked, through);
        crate::scene_box_mirror::box_mirror_shut(g);
        r
    } else {
        eye_contact_ask(g, q, chest, head, unseen, smoked, through)
    };
    r.map(|c| EyeContact { visible: c.visible, dist: c.dist, in_cone: c.in_cone })
}

/// (collision.ts StaticCollision.eyeContactAsk)
pub fn eye_contact_ask(g: &mut crate::game::Game, e: crate::scene_collision::EyeContactAskE, chest: V3, head: V3, unseen: bool, smoked: bool, through: f64) -> Option<crate::scene_collision::EyeContactAsk> {
    let bit = crate::scene_box_mirror::senses().sight;
    let set = sense_set(g, bit) as i32;
    let q = (if e.v_half_deg.is_some() { sim::sight::Q_LENS } else { 0 }) | (if unseen { sim::sight::Q_UNSEEN } else { 0 }) | (if smoked { sim::sight::Q_SMOKED } else { 0 });
    let mut dist = 0.0;
    let r = sim::sight::contact(slot(g), set, js::to_u32(through), q, e.pos, e.look, e.range, e.half_deg, e.near_m, e.near_deg, e.v_half_deg.unwrap_or(0.0), chest, head, &mut dist);
    if r == ASK_HOST { return None; }
    g.m.scene_collision.sensed.sight += 1.0;
    Some(EyeContactAsk { visible: r & sim::sight::A_VISIBLE != 0, dist, in_cone: r & sim::sight::A_IN_CONE != 0 })
}

/// a question about the light begins; `shade` = the smoke's shadow. (collision.ts StaticCollision.lampsBegin)
pub fn lamps_begin(g: &mut crate::game::Game, p: V3, shade: Option<&crate::scene_collision::LampsBeginShade>, k: Option<crate::scene_collision::SmokeNumbers>) -> bool {
    lamps_over(g);
    let bit = crate::scene_box_mirror::senses().light;
    g.col.mirror.lamp_set = sense_set(g, bit);
    g.col.mirror.lamp_n = 0.0;
    if let (Some(shade), Some(k)) = (shade, k) {
        blobs_fit(g, shade.blobs.len() as f64);
        g.col.mirror.blob_n = 0.0;
        for b in shade.blobs.iter() {
            blob(g, b.x, b.y0, b.z, b.r2, b.age, b.mass);
        }
        let q = &mut g.col.lamp_q;
        q.sim = Some(shade.grid.sim);
        q.stands = if shade.grid.stands { 1.0 } else { 0.0 };
        q.k = Some(k);
        q.nb = g.col.mirror.blob_n;
    }
    g.col.lamp_q.p = Some(p);
    g.col.lamp_q.on = true;
    true
}

/// no question about the light stands. (collision.ts StaticCollision.lampsOver)
pub fn lamps_over(g: &mut crate::game::Game) {
    let q = &mut g.col.lamp_q;
    q.on = false;
    q.p = None;
    q.sim = None;
    q.k = None;
    q.stands = 0.0;
    q.nb = 0.0;
    q.parts = 0.0;
}

/// the lamps written so far, asked as a part; the six faces so far into `out`. False = the module handed it back.
fn lamps_part_into(g: &mut Game, out: &mut [f64; 6]) -> bool {
    let w = slot(g);
    let q = g.col.lamp_q;
    let p = q.p.unwrap_or([0.0; 3]);
    let more = q.parts > 0.0;
    let (set, n) = (g.col.mirror.lamp_set as i32, g.col.mirror.lamp_n as usize);
    let r = match (q.sim, q.k) {
        (Some(_), Some(k)) => {
            let stamp = seam_sim_stamp(g);
            sim::light::asked(w, set, n, p, k.sigma, g.sim, stamp, q.stands != 0.0, more, q.nb as usize, [k.spread, k.widest, k.rise, k.rise_rate], out)
        }
        _ => sim::light::asked(w, set, n, p, 0.0, 0, 0.0, false, more, 0, [0.0; 4], out),
    };
    g.col.lamp_q.parts += 1.0;
    g.col.mirror.lamp_n = 0.0;
    if r == ASK_HOST {
        lamps_over(g);
        return false;
    }
    true
}

/// the lamps written so far, asked as a part of the question. (collision.ts StaticCollision.lampsPart)
pub fn lamps_part(g: &mut crate::game::Game) -> bool {
    let mut out = [0.0; 6];
    lamps_part_into(g, &mut out)
}

/// one lamp of the question; past the table's room the table is sent on as a part. False = no room and no question, or handed back. (collision.ts StaticCollision.lamp)
pub fn lamp(g: &mut crate::game::Game, pos: V3, range: f64, radius: f64, kind: f64, dir: V3, outer_deg: f64, inner_deg: f64, color: V3, intensity: f64) -> bool {
    if g.col.mirror.lamp_n as usize >= sim::light::MAX_LAMPS {
        if !g.col.lamp_q.on { return false; }
        g.m.scene_collision.lamps.parts += 1.0;
        if !lamps_part(g) { return false; }
    }
    let at = sim::light::lamps_addr();
    let o = g.col.mirror.lamp_n as usize * sim::light::LAMP;
    g.col.mirror.lamp_n += 1.0;
    let row = [pos[0], pos[1], pos[2], range, radius, kind, dir[0], dir[1], dir[2], outer_deg, inner_deg, color[0], color[1], color[2], intensity];
    for (i, v) in row.iter().enumerate() {
        put_f64(at, o + i, *v);
    }
    true
}

/// the last part asked, and the six faces of the whole question read into `acc`. (collision.ts StaticCollision.lampsOn)
pub fn lamps_on(g: &mut crate::game::Game, acc: &mut Vec<f64>) -> bool {
    if !g.col.lamp_q.on { return false; }
    let mut out = [0.0; 6];
    if !lamps_part_into(g, &mut out) { return false; }
    lamps_over(g);
    for k in 0..6 {
        if k < acc.len() { acc[k] = out[k]; } else { acc.push(out[k]); }
    }
    g.m.scene_collision.sensed.light += 1.0;
    true
}

/// a question of the smoke's blobs begins. (collision.ts StaticCollision.blobsBegin)
pub fn blobs_begin(g: &mut crate::game::Game, n: f64) -> bool {
    let bit = crate::scene_box_mirror::senses().smoke;
    g.col.mirror.blob_set = sense_set(g, bit);
    g.col.mirror.blob_n = 0.0;
    blobs_fit(g, n);
    true
}

/// a game keeps no more blobs than the module's table: more is a mistake, by name. (collision.ts StaticCollision.blobsFit)
pub fn blobs_fit(g: &mut crate::game::Game, n: f64) {
    if n > sim::smoke::MAX_BLOBS as f64 {
        panic!("{} blobs of smoke and the module's table holds {}: a game keeps no more than that (sense.ts CpuSmoke.emit), so somebody else filled the list", n, sim::smoke::MAX_BLOBS);
    }
}

/// one blob of the question. (collision.ts StaticCollision.blob)
pub fn blob(g: &mut crate::game::Game, x: f64, y0: f64, z: f64, r2: f64, age: f64, mass: f64) {
    let at = sim::smoke::blobs_addr();
    let o = g.col.mirror.blob_n as usize * sim::smoke::BLOB;
    g.col.mirror.blob_n += 1.0;
    let row = [x, y0, z, r2, age, mass];
    for (i, v) in row.iter().enumerate() {
        put_f64(at, o + i, *v);
    }
}

/// the transmittance along a→b; −1 = the caller's own loop. (collision.ts StaticCollision.blobsAlong)
pub fn blobs_along(g: &mut crate::game::Game, a: V3, b: V3, sigma: f64, spread: f64, widest: f64, rise: f64, rise_rate: f64) -> f64 {
    let mut out = 0.0;
    let (set, n) = (g.col.mirror.blob_set as i32, g.col.mirror.blob_n as usize);
    if sim::smoke::trans(slot(g), set, n, a, b, sigma, spread, widest, rise, rise_rate, &mut out) == ASK_HOST { return -1.0; }
    g.m.scene_collision.sensed.smoke += 1.0;
    out
}

/// `n` steps of the coarse grid's smoke over the boxes and leaves as they stand. False = nothing was done. (collision.ts StaticCollision.gridStep)
pub fn grid_step(g: &mut crate::game::Game, st: &mut crate::Opaque, pending: &[f64], n: f64, k: crate::scene_collision::GridStepK, glow: Option<crate::scene_collision::GridStepGlow>, wind: Option<&crate::scene_collision::GridWind>) -> bool {
    ready(g);
    let mine = !g.col.mirror.live;
    if mine { stand(g); }
    let d = g.col.dynamic_boxes.clone();
    let set = crate::scene_box_mirror::box_mirror_leaves(g, &d);
    if set == -2.0 {
        if mine { crate::scene_box_mirror::box_mirror_shut(g); }
        return false;
    }
    let set = set as i32;
    // the light that comes round: the only `lamps` hung is guards.ts steadyLamps (sense.ts step, called from game.ts)
    let mut lamps = -1i32;
    if glow.is_some() {
        g.col.mirror.lamp_n = 0.0;
        lamps_over(g);
        if crate::guards::steady_lamps(g) { lamps = g.col.mirror.lamp_n as i32; }
    }
    let all = pending.len() - pending.len() % 6;
    let cap = sim::SCRATCH_LEN - sim::SCRATCH_LEN % 6;
    let e = if all < cap { all } else { cap };
    let emits = &pending[all - e..all];
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
    // (lib.rs grid_step: a wind that is no number, or whose rise with height is none, is still air)
    let real = |v: f64| v - v == 0.0;
    let windy = real(wx) && real(wz) && real(low) && real(gain) && real(full_at) && real(k.in_wind) && low >= 0.0 && gain >= 0.0 && full_at > 0.0;
    let (wx, wz) = if windy { (wx, wz) } else { (0.0, 0.0) };
    let bk = sim::bricks::K { decay: k.decay, spread: k.spread, rise: k.rise, rise_rate: k.rise_rate, gone: k.gone, wind_x: wx, wind_z: wz, wind_low: low, wind_gain: gain, wind_full_at: full_at, wind_keep: k.in_wind };
    let stamp = seam_sim_stamp(g);
    let copy = crate::scene_box_mirror::box_mirror_grid_copy(g);
    let (w, block) = (slot(g), g.sim);
    let r = match (&glow, lamps >= 0) {
        (Some(gl), true) => {
            let gk = sim::glow::K { albedo: gl.k.albedo, share: gl.k.share, passes: js::to_u32(gl.k.passes), sweep: gl.k.sweep, lamps: lamps as usize };
            sim::bricks::step(block, stamp, w, copy, set, js::to_u32(n), emits, &bk, Some(&gk))
        }
        _ => sim::bricks::step(block, stamp, w, copy, set, js::to_u32(n), emits, &bk, None),
    };
    if mine { crate::scene_box_mirror::box_mirror_shut(g); }
    r != ASK_HOST
}

// ---- how a sound reaches a listener

/// (collision.ts SoundPaths.constructor)
pub fn sound_paths_new(g: &mut crate::game::Game) -> crate::scene_collision::SoundPaths {
    SoundPaths::blank()
}

/// once a tick: the clock of the routes remembered, and a memory over 512 cleared whole. (collision.ts SoundPaths.tick)
pub fn sound_paths_tick(g: &mut crate::game::Game, time: f64) {
    g.col.sound.now = time;
    if g.col.sound.cache.len() > 512 { g.col.sound.cache.clear(); }
}

/// the level changed under the routes remembered. (collision.ts SoundPaths.invalidate)
pub fn sound_paths_invalidate(g: &mut crate::game::Game) {
    g.col.sound.cache.clear();
}

/// how a sound at `src` reaches `dst`; `keep` false = the answer is not kept. (collision.ts SoundPaths.propagate)
pub fn sound_paths_propagate(g: &mut crate::game::Game, src: V3, dst: V3, keep: Option<bool>) -> crate::scene_collision::Propagation {
    let keep = keep.unwrap_or(true);
    if g.m.scene_collision.priced.waits {
        panic!("a sound was asked for from inside the price of a search: its way round is a search, and the module has one search's tables (rust/sim nav.rs find_priced)");
    }
    if !g.col.mirror.live {
        window(g, &mut |g: &mut Game| sound_paths_propagate_ask(g, src, dst, keep))
    } else {
        sound_paths_propagate_ask(g, src, dst, keep)
    }
}

/// (collision.ts SoundPaths.propagateAsk)
pub fn sound_paths_propagate_ask(g: &mut crate::game::Game, src: V3, dst: V3, keep: bool) -> crate::scene_collision::Propagation {
    match sound_paths_propagate_there(g, src, dst, keep, true) {
        Some(r) => r,
        None => sound_paths_propagate_here(g, src, dst, Some(keep)),
    }
}

/// the remembered routes' key: the two nav cells. (A −0 reads as 0, as a JavaScript string does.)
fn route_key(g: &Game, src: V3, dst: V3) -> String {
    let cs = g.col.nav.cell;
    alloc::format!("{},{}>{},{}", m::floor(src[0] / cs) + 0.0, m::floor(src[2] / cs) + 0.0, m::floor(dst[0] / cs) + 0.0, m::floor(dst[2] / cs) + 0.0)
}
fn sealed(src: V3, straight: f64) -> Propagation {
    Propagation { direct: false, routed: false, path_len: straight * 1.5, via: v3::copy(src), apparent: v3::copy(src), muffle: 1.0, carry: 0.25 }
}

/// the answer from the module, with this file's memory of routed answers between its two questions. None = propagateHere's. (collision.ts SoundPaths.propagateThere)
pub fn sound_paths_propagate_there(g: &mut crate::game::Game, src: V3, dst: V3, keep: bool, write: bool) -> Option<crate::scene_collision::Propagation> {
    let bit = crate::scene_box_mirror::senses().sound;
    let set = sense_set(g, bit) as i32;
    let ask = pane_ask(g);
    if ask < 0.0 { return None; }
    let ask = ask as u32;
    let w = slot(g);
    let mut straight = 0.0;
    let c = sim::hear::direct(w, set, ask, src, dst, &mut straight);
    if c == ASK_HOST { return None; }
    if c != sim::hear::WALLED { g.m.scene_collision.sensed.sound += 1.0; }
    if c < sim::hear::OVER {
        let mut graze = 0.0;
        if c & sim::hear::GRAZE_A != 0 { graze += 0.1; }
        if c & sim::hear::GRAZE_B != 0 { graze += 0.1; }
        return Some(Propagation { direct: true, routed: false, path_len: straight, via: v3::copy(src), apparent: v3::copy(src), muffle: graze, carry: 1.0 });
    }
    if c == sim::hear::OVER {
        return Some(Propagation { direct: true, routed: false, path_len: straight, via: v3::copy(src), apparent: v3::copy(src), muffle: 0.15, carry: 0.9 });
    }
    if c == sim::hear::FAR { return Some(sealed(src, straight)); }
    let key = route_key(g, src, dst);
    if let Some(hit) = g.col.sound.cache.get(key.clone()) {
        if m::abs(g.col.sound.now - hit.t) < 0.5 { return Some(hit.r); }
    }
    if !nav_grid_there(g) { return None; }
    let (nw, nh, cs) = (g.col.nav.w as usize, g.col.nav.h as usize, g.col.nav.cell);
    let seal = js::to_u32(crate::scene_boxes::box_flag().sound_seal);
    let mut out = [0.0; 4];
    let k = sim::hear::route(w, set, ask, seal, nw, nh, cs, src, dst, straight, &mut out);
    if k == ASK_HOST { return None; }
    g.m.scene_collision.sensed.sound += 1.0;
    let r = if k == sim::hear::SEALED {
        sealed(src, straight)
    } else {
        let (l, via) = (out[0], [out[1], out[2], out[3]]);
        let dir = v3::normalize([via[0] - dst[0], 0.0, via[2] - dst[2]]);
        let muffle = if k == sim::hear::ROUTED_DOOR { 0.85 } else { 0.35 };
        Propagation { direct: false, routed: true, path_len: l, via, apparent: [dst[0] + dir[0] * l, src[1], dst[2] + dir[2] * l], muffle, carry: 1.0 - muffle * 0.8 }
    };
    if write && keep && !g.col.sound.asking {
        let t = g.col.sound.now;
        g.col.sound.cache.set(key, SoundPathsCacheValue { t, r });
    }
    Some(r)
}

/// propagate as this file answers it, wherever the module hands it back. (collision.ts SoundPaths.propagateHere)
pub fn sound_paths_propagate_here(g: &mut crate::game::Game, src: V3, dst: V3, keep: Option<bool>) -> crate::scene_collision::Propagation {
    let keep = keep.unwrap_or(true);
    let a: V3 = [src[0], m::max(src[1], 0.9), src[2]];
    let b: V3 = [dst[0], m::max(dst[1], 0.9), dst[2]];
    let straight = v3::dist(a, b);
    if !solid_blocked(g, a, b, None) {
        let mut graze = 0.0;
        if straight > 1.0 {
            let d = v3::sub(b, a);
            let side = v3::normalize([-d[2], 0.0, d[0]]);
            if solid_blocked(g, v3::mad(a, side, 0.5), b, None) { graze += 0.1; }
            if solid_blocked(g, v3::mad(a, side, -0.5), b, None) { graze += 0.1; }
        }
        return Propagation { direct: true, routed: false, path_len: straight, via: v3::copy(src), apparent: v3::copy(src), muffle: graze, carry: 1.0 };
    }
    let (a2, b2): (V3, V3) = ([src[0], 2.0, src[2]], [dst[0], 2.0, dst[2]]);
    if !solid_blocked(g, a2, b2, None) {
        return Propagation { direct: true, routed: false, path_len: straight, via: v3::copy(src), apparent: v3::copy(src), muffle: 0.15, carry: 0.9 };
    }
    if straight > 28.0 { return sealed(src, straight); }
    let key = route_key(g, src, dst);
    if let Some(hit) = g.col.sound.cache.get(key.clone()) {
        if m::abs(g.col.sound.now - hit.t) < 0.5 { return hit.r; }
    }
    let path = nav_grid_find_path(g, dst, src, Some(6000.0), None);
    let r = match path {
        Some(path) if !path.is_empty() => {
            let mut l = 0.0;
            let mut prev: V3 = [dst[0], 0.0, dst[2]];
            let mut via: V3 = [path[0][0], 1.0, path[0][2]];
            let mut door_in_way = false;
            let mut pa: V3 = [dst[0], 1.0, dst[2]];
            let mut sight_lost = false;
            let seal = crate::scene_boxes::box_flag().sound_seal;
            for q in path.iter() {
                l += v3::dist_xz(*q, prev);
                prev = *q;
                let pb: V3 = [q[0], 1.0, q[2]];
                if !sight_lost {
                    if !solid_blocked(g, b, pb, None) { via = pb; } else { sight_lost = true; }
                }
                if !door_in_way && segment_blocked_dynamic(g, pa, pb, Some(seal)) { door_in_way = true; }
                pa = pb;
            }
            let last_leg = v3::dist_xz(src, prev);
            l += last_leg;
            l = m::max(l, straight);
            let s1: V3 = [src[0], 1.0, src[2]];
            let end: V3 = if last_leg > 0.3 { v3::mad(s1, v3::normalize(v3::sub(pa, s1)), 0.25) } else { pa };
            if !door_in_way && segment_blocked_dynamic(g, pa, end, Some(seal)) { door_in_way = true; }
            let dir = v3::normalize([via[0] - dst[0], 0.0, via[2] - dst[2]]);
            let muffle = if door_in_way { 0.85 } else { 0.35 };
            Propagation { direct: false, routed: true, path_len: l, via, apparent: [dst[0] + dir[0] * l, src[1], dst[2] + dir[2] * l], muffle, carry: 1.0 - muffle * 0.8 }
        }
        _ => sealed(src, straight),
    };
    if keep && !g.col.sound.asking {
        let t = g.col.sound.now;
        g.col.sound.cache.set(key, SoundPathsCacheValue { t, r });
    }
    r
}

// ---- the walk grid

/// the floor's grid baked: a cell is shut where a guard-sized circle at its centre is pushed. (collision.ts NavGrid.constructor)
pub fn nav_grid_new(g: &mut crate::game::Game) -> crate::scene_collision::NavGrid {
    let mut nav = NavGrid::blank();
    let wd = crate::scene_world::world();
    let cell = nav.cell;
    nav.w = m::round(wd.grid_w * wd.grid_cell / cell);
    nav.h = m::round(wd.grid_h * wd.grid_cell / cell);
    let (nw, nh) = (nav.w, nav.h);
    let mut blocked = vec![0u8; (nw * nh) as usize];
    window(g, &mut |g: &mut Game| {
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
    });
    nav.blocked = blocked;
    nav
}

/// the level changed inside a rectangle: the bake's question asked again of the cells near it. → the cells that changed, index and now-shut pairs. (collision.ts NavGrid.rebake)
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

/// (collision.ts NavGrid.idx)
pub fn nav_grid_idx(g: &mut crate::game::Game, x: f64, z: f64) -> f64 {
    z * g.col.nav.w + x
}

/// the statics the bake's own question meets at a cell: count into `out`, −1 = too short. (collision.ts NavGrid.cellBlockers)
pub fn nav_grid_cell_blockers(g: &mut crate::game::Game, wx: f64, wz: f64, out: &mut Vec<i32>) -> f64 {
    let (cell, nw, nh) = (g.col.nav.cell, g.col.nav.w, g.col.nav.h);
    let (x, z) = (m::floor(wx / cell), m::floor(wz / cell));
    if x < 0.0 || z < 0.0 || x >= nw || z >= nh { return 0.0; }
    circle_touches(g, (m::floor(wx / cell) + 0.5) * cell, (m::floor(wz / cell) + 0.5) * cell, 0.34, 0.25, 1.5, out)
}

/// (collision.ts NavGrid.isBlocked)
pub fn nav_grid_is_blocked(g: &mut crate::game::Game, wx: f64, wz: f64) -> bool {
    let (cell, nw, nh) = (g.col.nav.cell, g.col.nav.w, g.col.nav.h);
    let (x, z) = (m::floor(wx / cell), m::floor(wz / cell));
    if x < 0.0 || z < 0.0 || x >= nw || z >= nh { return true; }
    // (a point that is no number: every comparison above is false, and the TypeScript's table reads undefined, which is not 1)
    if x != x || z != z { return false; }
    g.col.nav.blocked[(z * nw + x) as usize] == 1
}

/// findPath for a picture's question: on the asker's tables. (collision.ts NavGrid.findPathAside)
pub fn nav_grid_find_path_aside(g: &mut crate::game::Game, own: &mut crate::scene_collision::NavGridFindPathAsideOwn, a: V3, b: V3, max_iter: Option<f64>) -> Option<Vec<V3>> {
    let max_iter = max_iter.unwrap_or(6000.0);
    let mut o = NavGridAsideOwn { s: own.s.take() };
    let r = nav_grid_aside(g, &mut o, &mut |g: &mut Game| nav_grid_find_path(g, a, b, Some(max_iter), None));
    own.s = o.s;
    r
}

/// `fn_` with every search of it on the asker's own tables. (collision.ts NavGrid.aside)
pub fn nav_grid_aside<T: Clone + core::fmt::Debug>(g: &mut crate::game::Game, own: &mut crate::scene_collision::NavGridAsideOwn, fn_: &mut dyn FnMut(&mut crate::game::Game) -> T) -> T {
    let mine = g.col.nav.scratch.take();
    g.col.nav.scratch = own.s.take();
    let r = fn_(g);
    own.s = g.col.nav.scratch.take();
    g.col.nav.scratch = mine;
    r
}

/// A* from a to b, smoothed; `penalty` a cost multiplier for stepping into a cell. (collision.ts NavGrid.findPath)
pub fn nav_grid_find_path(g: &mut crate::game::Game, a: V3, b: V3, max_iter: Option<f64>, penalty: Option<&mut dyn FnMut(&mut crate::game::Game, f64, f64) -> f64>) -> Option<Vec<V3>> {
    let max_iter = max_iter.unwrap_or(6000.0);
    if g.m.scene_collision.priced.waits {
        panic!("a way was asked for from inside the price of another search: the module has one search's tables (rust/sim nav.rs find_priced)");
    }
    let mut penalty = penalty;
    if !g.col.mirror.live {
        window(g, &mut |g: &mut Game| nav_grid_find_path_ask(g, a, b, max_iter, relend(&mut penalty)))
    } else {
        nav_grid_find_path_ask(g, a, b, max_iter, penalty)
    }
}

/// (collision.ts NavGrid.findPathAsk)
pub fn nav_grid_find_path_ask(g: &mut crate::game::Game, a: V3, b: V3, max_iter: f64, penalty: Option<&mut dyn FnMut(&mut crate::game::Game, f64, f64) -> f64>) -> Option<Vec<V3>> {
    nav_grid_find_path_there(g, a, b, max_iter, penalty)
}

/// the grid as it stands, written to the module for one question of its search. False = larger than its table. (collision.ts NavGrid.there)
pub fn nav_grid_there(g: &mut crate::game::Game) -> bool {
    let n = g.col.nav.blocked.len();
    if n > sim::nav::MAX_NAV { return false; }
    unsafe { core::ptr::copy_nonoverlapping(g.col.nav.blocked.as_ptr(), sim::nav::shut_addr() as *mut u8, n); }
    true
}

/// findPath from the module; where the module hands a search back, this file's own (the signature has no third answer). (collision.ts NavGrid.findPathThere)
pub fn nav_grid_find_path_there(g: &mut crate::game::Game, a: V3, b: V3, max_iter: f64, penalty: Option<&mut dyn FnMut(&mut crate::game::Game, f64, f64) -> f64>) -> Option<Vec<V3>> {
    let mut penalty = penalty;
    let bit = crate::scene_box_mirror::senses().paths;
    let set = sense_set(g, bit) as i32;
    if js::to_u32(max_iter) as f64 != max_iter || !nav_grid_there(g) {
        return nav_grid_find_path_here(g, a, b, Some(max_iter), penalty);
    }
    let ask = pane_ask(g);
    if ask < 0.0 { return nav_grid_find_path_here(g, a, b, Some(max_iter), penalty); }
    let ask = ask as u32;
    let (w, nw, nh, cell) = (slot(g), g.col.nav.w as usize, g.col.nav.h as usize, g.col.nav.cell);
    let mi = js::to_u32(max_iter);
    let mut n;
    if penalty.is_none() {
        n = sim::nav::find_path(w, set, ask, nw, nh, cell, a[0], a[2], b[0], b[2], mi);
    } else {
        n = sim::nav::find_priced(w, set, ask, nw, nh, cell, a[0], a[2], b[0], b[2], mi);
        if n == sim::nav::NEED {
            g.m.scene_collision.priced.waits = true;
            while n == sim::nav::NEED {
                g.m.scene_collision.priced.needs += 1.0;
                let k = sim::nav::need_len() as f64;
                nav_grid_price_these(g, k, &mut **penalty.as_mut().unwrap());
                n = sim::nav::resume();
            }
            g.m.scene_collision.priced.waits = false;
        }
        if n != sim::nav::ASK_HOST { g.m.scene_collision.priced.n += 1.0; }
    }
    if n == sim::nav::ASK_HOST { return nav_grid_find_path_here(g, a, b, Some(max_iter), penalty); }
    g.m.scene_collision.sensed.paths += 1.0;
    if n == sim::nav::NO_WAY { return None; }
    let at = sim::nav::way_addr();
    let mut out: Vec<V3> = Vec::with_capacity(n as usize);
    for i in 0..n as usize {
        out.push([get_f64(at, i * 2), 0.0, get_f64(at, i * 2 + 1)]);
    }
    Some(out)
}

/// the prices the module's search has asked for, from the caller's function, in the order asked. (collision.ts NavGrid.priceThese)
pub fn nav_grid_price_these(g: &mut crate::game::Game, k: f64, penalty: &mut dyn FnMut(&mut crate::game::Game, f64, f64) -> f64) {
    let nw = g.col.nav.w as u32;
    for i in 0..k as usize {
        let c = get_u32(sim::nav::need_addr(), i);
        let v = penalty(g, (c % nw) as f64, (c / nw) as f64);
        put_f64(sim::nav::price_addr(), c as usize, v);
    }
    g.m.scene_collision.priced.cells += k;
}

/// findPath as this file answers it, wherever the module hands a search back. (collision.ts NavGrid.findPathHere)
pub fn nav_grid_find_path_here(g: &mut crate::game::Game, a: V3, b: V3, max_iter: Option<f64>, penalty: Option<&mut dyn FnMut(&mut crate::game::Game, f64, f64) -> f64>) -> Option<Vec<V3>> {
    let max_iter = max_iter.unwrap_or(6000.0);
    let mut penalty = penalty;
    if a[0].is_nan() || a[2].is_nan() || b[0].is_nan() || b[2].is_nan() { return None; }   // (TRAP: the TypeScript's walk over cells that are no cells ends with no way)
    let (cell, w_f, h_f) = (g.col.nav.cell, g.col.nav.w, g.col.nav.h);
    let (w, h) = (w_f as i32, h_f as i32);
    let sx = clampi(m::floor(a[0] / cell), 0.0, w_f - 1.0) as i32;
    let sz = clampi(m::floor(a[2] / cell), 0.0, h_f - 1.0) as i32;
    let mut gx = clampi(m::floor(b[0] / cell), 0.0, w_f - 1.0) as i32;
    let mut gz = clampi(m::floor(b[2] / cell), 0.0, h_f - 1.0) as i32;
    if g.col.nav.blocked[(gz * w + gx) as usize] != 0 {
        let alt = nav_grid_nearest_free(g, gx as f64, gz as f64, Some(b[0]), Some(b[2]))?;
        gx = alt[0] as i32;
        gz = alt[1] as i32;
    }
    let (start, goal) = (sz * w + sx, gz * w + gx);
    if start == goal {
        return Some(vec![[(gx as f64 + 0.5) * cell, 0.0, (gz as f64 + 0.5) * cell]]);
    }
    // (the tables are lent out of the grid for the search: a search inside a penalty finds none and takes fresh ones, as it finds them busy in the TypeScript)
    let mut s = match g.col.nav.scratch.take() {
        Some(s) if s.g.len() == (w * h) as usize => s,
        _ => nav_scratch((w * h) as f64),
    };
    let (mut n_open, mut n_seen) = (0usize, 0usize);
    {
        let (x, z) = (start % w, start / w);
        let (dx, dz) = ((x - gx).abs(), (z - gz).abs());
        s.g[start as usize] = 0.0;
        s.f[start as usize] = (((dx + dz) as f64) + (core::f64::consts::SQRT_2 - 2.0) * (dx.min(dz) as f64)) as f32;
    }
    s.open[n_open] = start;
    n_open += 1;
    s.seen[n_seen] = start;
    n_seen += 1;
    let mut iter = 0.0;
    loop {
        if n_open == 0 { break; }
        let go = iter < max_iter;
        iter += 1.0;
        if !go { break; }
        let (mut mi, mut mf) = (0usize, s.f[s.open[0] as usize] as f64);
        for i in 1..n_open {
            let fi = s.f[s.open[i] as usize] as f64;
            if fi < mf { mi = i; mf = fi; }
        }
        let cur = s.open[mi];
        n_open -= 1;
        s.open[mi] = s.open[n_open];
        if cur == goal { break; }
        if s.closed[cur as usize] != 0 { continue; }
        s.closed[cur as usize] = 1;
        let (cx, cz) = (cur % w, cur / w);
        if n_open + 8 > s.open.len() { let n = s.open.len() * 2; s.open.resize(n, 0); }
        if n_seen + 8 > s.seen.len() { let n = s.seen.len() * 2; s.seen.resize(n, 0); }
        for dz in -1..=1i32 {
            for dx in -1..=1i32 {
                if dx == 0 && dz == 0 { continue; }
                let (nx, nz) = (cx + dx, cz + dz);
                if nx < 0 || nz < 0 || nx >= w || nz >= h { continue; }
                let ni = (nz * w + nx) as usize;
                if g.col.nav.blocked[ni] != 0 || s.closed[ni] != 0 { continue; }
                if dx != 0 && dz != 0 && (g.col.nav.blocked[(cz * w + cx + dx) as usize] != 0 || g.col.nav.blocked[((cz + dz) * w + cx) as usize] != 0) { continue; }
                let step = if dx != 0 && dz != 0 { core::f64::consts::SQRT_2 } else { 1.0 };
                let price = match penalty.as_mut() {
                    Some(p) => step * m::max(1.0, p(g, nx as f64, nz as f64)),
                    None => step,
                };
                let ng = s.g[cur as usize] as f64 + price;
                if ng < s.g[ni] as f64 {
                    let (hx, hz) = ((nx - gx).abs(), (nz - gz).abs());
                    s.g[ni] = ng as f32;
                    s.came[ni] = cur;
                    s.f[ni] = (ng + (((hx + hz) as f64) + (core::f64::consts::SQRT_2 - 2.0) * (hx.min(hz) as f64))) as f32;
                    s.open[n_open] = ni as i32;
                    n_open += 1;
                    s.seen[n_seen] = ni as i32;
                    n_seen += 1;
                }
            }
        }
    }
    let mut cells: Vec<i32> = Vec::new();
    let found = s.came[goal as usize] >= 0;
    if found {
        let mut c = goal;
        while c >= 0 && c != start {
            cells.push(c);
            c = s.came[c as usize];
        }
    }
    for i in 0..n_seen {
        let c = s.seen[i] as usize;
        s.g[c] = f32::INFINITY;
        s.f[c] = f32::INFINITY;
        s.came[c] = -1;
        s.closed[c] = 0;
    }
    g.col.nav.scratch = Some(s);
    if !found { return None; }
    cells.reverse();
    let n = cells.len();
    let dear: Option<Vec<bool>> = match penalty.as_mut() {
        Some(p) => {
            let mut d = Vec::with_capacity(n);
            for &c in cells.iter() {
                d.push(p(g, (c % w) as f64, (c / w) as f64) > 1.0);
            }
            Some(d)
        }
        None => None,
    };
    let mut out: Vec<V3> = Vec::new();
    let mut anchor: V3 = [a[0], 0.0, a[2]];
    let mut i = 0usize;
    while i < n {
        let mut j = n - 1;
        while j > i {
            let c = cells[j];
            let q: V3 = [((c % w) as f64 + 0.5) * cell, 0.0, ((c / w) as f64 + 0.5) * cell];
            if nav_grid_walkable(g, anchor, q) {
                match (&dear, penalty.as_mut()) {
                    (Some(d), Some(p)) => {
                        if nav_grid_dear_along(g, anchor, q, &mut **p) <= count_true(d, i as f64, j as f64) { break; }
                    }
                    _ => break,
                }
            }
            j -= 1;
        }
        let c = cells[j];
        let p: V3 = [((c % w) as f64 + 0.5) * cell, 0.0, ((c / w) as f64 + 0.5) * cell];
        out.push(p);
        anchor = p;
        i = j + 1;
    }
    Some(out)
}

/// distinct cells along a straight leg whose penalty is above plain floor. (collision.ts NavGrid.dearAlong)
pub fn nav_grid_dear_along(g: &mut crate::game::Game, a: V3, b: V3, penalty: &mut dyn FnMut(&mut crate::game::Game, f64, f64) -> f64) -> f64 {
    let (cell, nw, nh) = (g.col.nav.cell, g.col.nav.w, g.col.nav.h);
    let (dx, dz) = (b[0] - a[0], b[2] - a[2]);
    let n = m::ceil(m::hyp2(dx, dz) / (cell * 0.5)) + 1.0;
    let (mut last, mut count) = (-1.0, 0.0);
    let mut k = 0.0;
    while k <= n {
        let t = k / n;
        let (ix, iz) = (m::floor((a[0] + dx * t) / cell), m::floor((a[2] + dz * t) / cell));
        let i = iz * nw + ix;
        k += 1.0;
        if i == last { continue; }
        last = i;
        if ix >= 0.0 && iz >= 0.0 && ix < nw && iz < nh && penalty(g, ix, iz) > 1.0 { count += 1.0; }
    }
    count
}

/// the free cell nearest by ring to a shut one; with the point that asked, on its own side of the wall. (collision.ts NavGrid.nearestFree)
pub fn nav_grid_nearest_free(g: &mut crate::game::Game, gx: f64, gz: f64, wx: Option<f64>, wz: Option<f64>) -> Option<[f64; 2]> {
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

/// the centre of the walkable cell nearest a world point, or none within reach. (collision.ts NavGrid.nearestFreePoint)
pub fn nav_grid_nearest_free_point(g: &mut crate::game::Game, wx: f64, wz: f64) -> Option<V3> {
    let (cell, nw, nh) = (g.col.nav.cell, g.col.nav.w, g.col.nav.h);
    let x = clampi(m::floor(wx / cell), 0.0, nw - 1.0);
    let z = clampi(m::floor(wz / cell), 0.0, nh - 1.0);
    let c = if g.col.nav.blocked[(z * nw + x) as usize] != 0 { nav_grid_nearest_free(g, x, z, Some(wx), Some(wz)) } else { Some([x, z]) };
    c.map(|c| [(c[0] + 0.5) * cell, 0.0, (c[1] + 0.5) * cell])
}

/// conservative straight-line walkability. (collision.ts NavGrid.walkable)
pub fn nav_grid_walkable(g: &mut crate::game::Game, a: V3, b: V3) -> bool {
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

/// (collision.ts clampi)
pub fn clampi(x: f64, a: f64, b: f64) -> f64 {
    if x < a { a } else if x > b { b } else { x }
}

/// two points, the same to the bit. (collision.ts same3)
pub fn same3(a: V3, b: V3) -> bool {
    is(a[0], b[0]) && is(a[1], b[1]) && is(a[2], b[2])
}

/// a search's tables, blank. (collision.ts navScratch)
pub fn nav_scratch(n: f64) -> crate::scene_collision::NavScratch {
    let n = n as usize;
    NavScratch { g: vec![f32::INFINITY; n], f: vec![f32::INFINITY; n], came: vec![-1; n], closed: vec![0; n], open: vec![0; 1024], seen: vec![0; 1024] }
}

/// how many of flags[i‥j] are set. (collision.ts countTrue)
pub fn count_true(flags: &[bool], i: f64, j: f64) -> f64 {
    let mut n = 0.0;
    let mut k = i as usize;
    while k <= j as usize {
        if flags[k] { n += 1.0; }
        k += 1;
    }
    n
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bits(a: &[f64]) -> Vec<u64> {
        a.iter().map(|x| x.to_bits()).collect()
    }
    fn a_box(g: &mut Game, c: V3, h: V3, yaw: f64) -> BoxId {
        let mut b = crate::scene_boxes::Box::zeroed();
        b.c = c;
        b.h = h;
        b.yaw = yaw;
        g.col.boxes.push(b)
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
        let mut g = Box::new(Game::blank());
        let g = &mut *g;
        let (a, rd): (V3, V3) = ([1.25, 0.7, -2.5], [0.6, 0.0, 0.8]);
        let yaw = a_box(g, [3.0, 1.0, 1.0], [1.5, 1.0, 0.75], 0.7);
        let r = line_box(g, a, rd, yaw).unwrap();
        assert_eq!(bits(&[r.t0, r.t1, r.depth]), [0x400879431796fbb5, 0x40123f15faa99759, 0xc0086f43d65a5520]);
        let rot = a_box(g, [3.0, 1.0, 1.0], [1.5, 1.0, 0.75], 0.0);
        g.col.boxes[rot].rot = Some([0.1, 0.2, 0.3, m::sqrt(1.0 - 0.14)]);
        let r = line_box(g, a, rd, rot).unwrap();
        assert_eq!(bits(&[r.t0, r.t1, r.depth]), [0x4009a0153febbaba, 0x4012f1af13a82510, 0xc009137acf27ccdb]);
        let round = a_box(g, [3.0, 1.0, 1.0], [1.5, 1.0, 1.5], 0.7);
        g.col.boxes[round].round = Some(true);
        let r = line_box(g, a, rd, round).unwrap();
        assert_eq!(bits(&[r.t0, r.t1, r.depth]), [0x40042fd22b3439eb, 0x4014b4e3b732afd6, 0xc0034e115049ec25]);
        let miss = a_box(g, [30.0, 1.0, 1.0], [1.0, 1.0, 1.0], 0.2);
        assert!(line_box(g, a, rd, miss).is_none());
    }

    #[test]
    fn the_hosts_search_finds_the_typescripts_way() {
        let mut g = Box::new(Game::blank());
        let g = &mut *g;
        grid(g);
        let (a, b): (V3, V3) = ([5.1, 0.0, 5.2], [25.3, 0.0, 3.7]);
        let want = [0x402d800000000000u64, 0, 0x4024800000000000, 0x402f800000000000, 0, 0x4024800000000000, 0x4039400000000000, 0, 0x400e000000000000];
        let flat = |p: Vec<V3>| -> Vec<u64> { p.iter().flat_map(|q| q.iter().map(|x| x.to_bits())).collect() };
        assert_eq!(flat(nav_grid_find_path_here(g, a, b, None, None).unwrap()), want);
        let mut pen = |_: &mut Game, ix: f64, iz: f64| if iz >= 5.0 && iz <= 14.0 && ix >= 20.0 && ix <= 25.0 { 6.0 } else { 1.0 };
        assert_eq!(flat(nav_grid_find_path_here(g, a, b, None, Some(&mut pen)).unwrap()), want);
        // (its tables are blank again, and kept)
        let s = g.col.nav.scratch.as_ref().unwrap();
        assert!(s.came.iter().all(|&c| c == -1) && s.closed.iter().all(|&c| c == 0));
        assert!(!nav_grid_walkable(g, [5.0, 0.0, 5.0], [20.0, 0.0, 5.0]));
        assert!(nav_grid_walkable(g, [5.0, 0.0, 5.0], [5.0, 0.0, 19.9]));
        assert!(nav_grid_find_path_here(g, [f64::NAN, 0.0, 5.0], b, None, None).is_none());
    }

    #[test]
    fn cells_keep_what_a_cut_leaves() {
        let mut cells = vec![vec![0.0, 5.0, 2.0, 7.0], vec![6.0]];
        cut_cells(&mut cells, 5.0);
        assert_eq!(cells, vec![vec![0.0, 2.0], vec![]]);
    }

    #[test]
    fn the_twins_helpers_answer_as_the_typescripts_do() {
        // (the answers of segClear and rayBoxHits under bun with EXACT.on, over the same 256 made-up lines and boxes)
        let want_clear = "1010101111011110101111111011101100111001101000111011111111111011111001111101101111011111001111110011011011111010111011101110111100011111110011110111011011111111011100000111111000110000111011110000101101110111111101101001011111111111001010110110110111011111";
        let want_hits = "0000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000100000000100000000000000000000100000000000000000000000000000000000011000000010000001000000000101000000001000000000010000000000000000000000000000000000000000";
        let mut g = Box::new(Game::blank());
        let g = &mut *g;
        let mut x = 12345.0;
        let mut r = |lo: f64, hi: f64| { x = js::rem(x * 16807.0, 2147483647.0); lo + (hi - lo) * (x / 2147483647.0) };
        let (mut clear, mut hits) = (String::new(), String::new());
        for i in 0..256 {
            let (a, e): (V3, V3) = ([r(0.0, 12.0), r(0.0, 3.0), r(0.0, 12.0)], [r(0.0, 12.0), r(0.0, 3.0), r(0.0, 12.0)]);
            let d = [e[0] - a[0], e[1] - a[1], e[2] - a[2]];
            let len = m::sqrt(d[0] * d[0] + d[1] * d[1] + d[2] * d[2]);
            let u = [d[0] / len, if i % 7 == 0 { 0.0 } else { d[1] / len }, d[2] / len];
            let (c, h, yaw) = ([r(0.0, 12.0), r(0.0, 3.0), r(0.0, 12.0)], [r(0.1, 2.0), r(0.1, 1.5), r(0.1, 2.0)], r(-3.0, 3.0));
            let b = a_box(g, c, h, yaw);
            if i % 5 == 0 { g.col.boxes[b].rot = Some([0.0, 0.0, 0.0, 1.0]); }
            clear.push(if seg_clear(g, a[0], a[1], a[2], u[0], u[1], u[2], len, b, 0.05) { '1' } else { '0' });
            hits.push(if ray_box_hits(g, a[0], a[1], a[2], u[0], u[1], u[2], b, len) { '1' } else { '0' });
        }
        assert_eq!(clear, want_clear);
        assert_eq!(hits, want_hits);
    }

    #[test]
    fn a_leaf_across_the_line_stops_it_if_it_carries_the_flags() {
        let mut g = Box::new(Game::blank());
        let g = &mut *g;
        let leaf = a_box(g, [5.0, 1.0, 0.0], [0.05, 1.0, 1.0], 0.0);
        g.col.boxes[leaf].flags = 4.0;
        g.col.dynamic_boxes = vec![leaf];
        let a: V3 = [0.0, 1.0, 0.0];
        assert!(dynamic_hit(g, a, 1.0, 0.0, 0.0, 10.0, 0.0));
        assert!(dynamic_hit(g, a, 1.0, 0.0, 0.0, 10.0, 4.0));
        assert!(!dynamic_hit(g, a, 1.0, 0.0, 0.0, 10.0, 8.0));
        assert!(!dynamic_hit(g, a, 1.0, 0.0, 0.0, 4.0, 0.0));
        assert!(!dynamic_hit(g, a, 0.0, 0.0, 1.0, 10.0, 0.0));
    }

    #[test]
    fn small_ones() {
        assert_eq!(clampi(-1.0, 0.0, 3.0), 0.0);
        assert_eq!(clampi(4.0, 0.0, 3.0), 3.0);
        assert!(clampi(f64::NAN, 0.0, 3.0).is_nan());
        assert!(same3([f64::NAN, 0.0, 1.0], [-f64::NAN, 0.0, 1.0]));
        assert!(!same3([0.0, 0.0, 1.0], [-0.0, 0.0, 1.0]));
        assert_eq!(count_true(&[true, false, true, true], 1.0, 3.0), 2.0);
        assert_eq!(sense_name(8.0), "sound");
        assert_eq!(sense_name(3.0), "a sense");
        let s = nav_scratch(4.0);
        assert!(s.g[0].is_infinite() && s.came[3] == -1 && s.open.len() == 1024);
    }
}
