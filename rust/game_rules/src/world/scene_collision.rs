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
use sim::world::ASK_HOST;

// ---- small private helpers

/// A box's row (Box.* in the TypeScript).
fn seam_box(g: &mut Game, b: BoxId) -> &mut crate::scene_boxes::Box { &mut g.tab.boxes[b] }
/// which of the module's worlds is this game's: M.slot, which the mirror's `take` names as the copy first stands
#[inline]
fn slot(g: &Game) -> usize {
    g.col.mirror.slot as usize
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
/// a point as `a.join(', ')` writes it, in the language's own printing of a number
fn joined(a: &[f64]) -> String {
    let mut s = String::new();
    for (i, v) in a.iter().enumerate() {
        if i > 0 { s.push_str(", "); }
        s.push_str(&js::num_str(*v));
    }
    s
}
/// the same function lent on again, for one call
fn relend<'a>(p: &'a mut Option<&mut dyn FnMut(&mut Game, f64, f64) -> f64>) -> Option<&'a mut dyn FnMut(&mut Game, f64, f64) -> f64> {
    match p {
        Some(f) => Some(&mut **f),
        None => None,
    }
}

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
pub fn line_box(g: &mut crate::game::Game, a: V3, rd: V3, b: BoxId) -> Option<crate::scene_collision::LineBox> {
    let (c, h, yaw, rot, round, has_cut) = { let x = seam_box(g, b); (x.c, x.h, x.yaw, x.rot, x.round == Some(true), x.cut.is_some()) };
    let s = slab(a, rd, c, h, yaw, rot)?;
    if round {
        let s = slab_round(s, h)?;
        return Some(LineBox { t0: s.t0, t1: s.t1, depth: s.depth });
    }
    let (mut t0, mut t1, mut depth) = (s.t0, s.t1, s.depth);
    if has_cut && crate::scene_boxes::cut_met(g, b.into()) {
        let cut = seam_box(g, b).cut.clone().unwrap();
        let ([ox, oy, oz], [dx, dy, dz]) = (s.o, s.d);
        (t0, t1) = crate::scene_cut::cut_clip(&cut, ox, oy, oz, dx, dy, dz, t0, t1)?;
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
fn foot_band(g: &crate::game::Game, y0: f64) -> f64 {
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

/// the level's rows as the module is told them: 1 they are read, 2 their steady selves.
fn rows_how(r: Option<crate::scene_collision::LampRows>) -> f64 {
    match r {
        Some(r) => if r.steady { 3.0 } else { 1.0 },
        None => 0.0,
    }
}
/// a question's word of the rows as a call of the module takes it (lib.rs light_rows over `rowsHow`). `LampRows.st` is Opaque: the block that holds the rows is this game's, `g.sim`
fn rows_for(g: &Game, r: Option<LampRows>) -> Option<sim::light::Rows> {
    r.map(|r| sim::light::Rows { g: g.sim, leave: js::to_u32(r.leave), steady: js::to_u32(rows_how(Some(r))) & 2 != 0, exposure: crate::hours::exposure(g) })
}

/// a sense's question the module handed back, in words
fn handed_back_words(row: &str, why: &str, a: &[f64], b: &[f64]) -> String {
    alloc::format!("{}: the Rust module handed the question back ({}): [{}] → [{}]", row, why, joined(&a[..3]), joined(&b[..3]))
}
// ---- StaticCollision: the grid, as the boxes are filed in it

/// the box world stood up: the cells from `boxes`, the walk grid baked. `g.col = new(g, …)`: everything of g.col is new, and is built where the bake can ask it.
pub fn new(g: &mut crate::game::Game, boxes: &[BoxId], of: Option<&str>) -> crate::scene_collision::StaticCollision {
    g.col = StaticCollision::blank();
    g.col.boxes = boxes.to_vec();
    g.col.mirror.of = of.map(String::from);
    let wd = crate::scene_world::world();
    g.col.w = wd.grid_w;
    g.col.h = wd.grid_h;
    g.col.cs = wd.grid_cell;
    let (gw, gh, gc) = (g.col.w, g.col.h, g.col.cs);
    g.col.cells = vec![Vec::new(); (gw * gh) as usize];
    let skip = js::to_u32(crate::scene_boxes::box_flag().no_shadow) | js::to_u32(crate::scene_boxes::box_flag().dynamic);
    for (at, &b) in boxes.iter().enumerate() {
        let (flags, yaw, h, c, tilted) = { let r = seam_box(g, b); (r.flags, r.yaw, r.h, r.c, r.rot.is_some()) };
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
    g.col.nav = nav_grid_new(g);
    g.col.sound = sound_paths_new(g);
    core::mem::take(&mut g.col)
}

/// a static appended in play: `b` joins the list and is filed in the cells of `footprint`'s plan. → its index.
pub fn add_static(g: &mut crate::game::Game, b: BoxId, footprint: Option<BoxId>) -> f64 {
    let i = g.col.boxes.len() as f64;
    g.col.boxes.push(b);
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

/// the cells a static appended over `f` is filed in: x0, x1, z0, z1.
fn foot_cells(g: &mut crate::game::Game, f: BoxId) -> [f64; 4] {
    let (yaw, h, c) = { let r = seam_box(g, f); (r.yaw, r.h, r.c) };
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
pub fn foot_entries(g: &mut crate::game::Game, f: BoxId) -> f64 {
    let [x0, x1, z0, z1] = foot_cells(g, f);
    m::max(0.0, x1 - x0 + 1.0) * m::max(0.0, z1 - z0 + 1.0)
}

/// the level's list cut back to its first `n`: every later index leaves the cells.
pub fn truncate_statics(g: &mut crate::game::Game, n: f64) {
    if (g.col.boxes.len() as f64) <= n { return; }
    cut_cells(&mut g.col.cells, n);
    g.col.boxes.truncate(n as usize);
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
/// a question asked where no copy stands (a level's own standing, a writer that put it down) is proven for itself, and shut behind it
#[inline]
fn alone(g: &mut Game) -> bool {
    !standing(g) && lone(g, None)
}
/// M.open, with the rows it says the questions go to. (`cells` and `tiltedAt` are lent for the call: the copy's writer reads neither through g.)
fn open_copy(g: &mut Game, rows: bool) {
    let ids = g.col.boxes.clone();
    let (cells, tilted) = (core::mem::take(&mut g.col.cells), core::mem::take(&mut g.col.tilted_at));
    let (gw, gh, gc) = (g.col.w, g.col.h, g.col.cs);
    let leaves = g.col.dynamic_boxes.clone();
    let glass = glass_panes(g);
    let mut rule = walk_rule;
    let walk: Option<&mut dyn FnMut(&mut Game, f64) -> bool> = if g.col.glass_walkable.is_some() { Some(&mut rule) } else { None };
    crate::scene_box_mirror::box_mirror_open(g, &ids, &cells, &tilted, gw, gh, gc, &leaves, walk, glass.as_ref(), rows, rows, rows);
    g.col.cells = cells;
    g.col.tilted_at = tilted;
}

/// a tick opens: the rows are made the boxes as they stand.
pub fn tick_open(g: &mut crate::game::Game) {
    let kept = if cfg!(debug_assertions) && crate::scene_box_mirror::box_mirror_stands(g).is_ok() { Some(rows_now(g)) } else { None };
    stand(g);
    g.col.mirror.played = true;
    // (THE PROOF OF THE COPY THAT STANDS BETWEEN TICKS: a build with its checks on holds every row of it to its box as the next tick opens)
    if let Some(k) = kept {
        let now = rows_now(g);
        for (what, a, b) in [("static", &k.0, &now.0), ("leaf", &k.1, &now.1), ("cut plane's box", &k.2, &now.2)] {
            if let Some(i) = a.iter().zip(b).position(|(x, y)| x.to_bits() != y.to_bits()) { panic!("{} {} was written between two ticks and nobody said so: a question asked there of the game shared was answered with it as it was", what, i / sim::world::REC); }
        }
    }
}
/// the rows as the module holds them: the statics (each with its flags), the game's own leaves, the statics' cut planes
fn rows_now(g: &mut Game) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
    let d = g.col.dynamic_boxes.clone();
    crate::scene_box_mirror::box_mirror_rules(g, &d);
    let (o, n, l) = (sim::world::world_read(slot(g)), g.col.boxes.len(), d.len());
    let mut rec = o.rec[..n * sim::world::REC].to_vec();
    for i in 0..n { rec.push(o.flags[i] as f64); }
    (rec, o.leaf[..l * sim::world::REC].to_vec(), o.cut[..sim::world::MAX_CUTS * sim::world::CUT_PLANES * 4].to_vec())
}

/// the rows made the boxes as they stand, for whoever opens a stretch of questions. Every row of RUST is on: there is no other side.
fn stand(g: &mut crate::game::Game) {
    open_copy(g, true);
}

/// the world a rule of the module's steps over, with the game's own leaves `dyn_` in list 0 as they are.
pub fn rules_world(g: &mut crate::game::Game, dyn_: &[BoxId]) {
    let was_alone = alone(g);
    ready(g);
    if !crate::scene_box_mirror::box_mirror_rules(g, dyn_) {
        panic!("another list in the leaves' place, or leaves that do not fit a list");
    }
    if was_alone { crate::scene_box_mirror::box_mirror_shut(g); }
}

/// the world a rule asks of the statics alone. The signature has no null: outside a tick and a window the rows are proven and shut, and their tables hold.
pub fn rules_statics(g: &mut crate::game::Game) {
    ready(g);
    if alone(g) { crate::scene_box_mirror::box_mirror_shut(g); }
}

/// the copy as a rule in the module that asks its own lines takes it: standing, with the game's leaves in it; `rule_set` and `rule_panes` on the mirror say what to hand it.
pub fn rule_open(g: &mut crate::game::Game) {
    ready(g);
    if !standing(g) { open_copy(g, true); }
    let d = g.col.dynamic_boxes.clone();
    let set = crate::scene_box_mirror::box_mirror_leaves(g, &d);
    if set == -2.0 { panic!("a rule of the module was asked with more door leaves than its copy holds"); }
    g.col.mirror.rule_set = set;
    g.col.mirror.rule_panes = if g.col.pane_rule.is_some() && crate::scene_box_mirror::box_mirror_glass_in(g) { 1.0 } else { 0.0 };
}

/// the window a rule that takes `rule_open` stands in; where there is none at all the copy `rule_open` stood is shut behind it.
pub fn rule<T: Clone + core::fmt::Debug>(g: &mut crate::game::Game, fn_: &mut dyn FnMut(&mut crate::game::Game) -> T) -> T {
    let was = standing(g);
    let r = window(g, fn_);
    if !was && standing(g) && !g.col.windowed { crate::scene_box_mirror::box_mirror_shut(g); }
    r
}

/// the pane rule as a rule's step takes it.
pub fn rules_panes(g: &crate::game::Game) -> f64 {
    pane_ask(g)
}

/// why the copy is not answering now, for the count of questions the TypeScript took.
fn fell_why(g: &crate::game::Game) -> String {
    String::from(if standing(g) { "the row was not up when the tick opened" } else { "outside a tick" })
}

/// the foot of a span as the statics are asked it.
pub fn foot_of(g: &crate::game::Game, y0: f64) -> f64 {
    foot_band(g, y0)
}

/// A TICK SHUTS AND THE COPY STANDS: its debts are paid, so that whoever asks before the next tick asks it of the game shared (`ask::WorldAsk`). A writer that
/// cannot say what it writes (a tool's write by path, a state read in) puts it down first (`unstand`).
pub fn tick_shut(g: &mut crate::game::Game) {
    if standing(g) { stands(g); crate::scene_box_mirror::box_mirror_things(g); }
}
/// does a copy of the boxes stand that is this game's and of this moment (it may owe a debt, which the next question pays)
pub fn standing(g: &Game) -> bool { g.col.mirror.live && g.col.mirror.stood != 0 && g.col.mirror.stood == sim::world::world_read(g.sim).stood }
/// the copy stands and owes nothing, whatever it takes: proven from nothing, or its debts paid
pub fn stands(g: &mut crate::game::Game) {
    if !standing(g) { stand(g); } else { ready(g); }
    let d = g.col.dynamic_boxes.clone();
    if !crate::scene_box_mirror::box_mirror_rules(g, &d) {
        // (the list of the leaves itself is another than the copy opened with: the level's was made again)
        stand(g);
        if !crate::scene_box_mirror::box_mirror_rules(g, &d) { panic!("more leaves than a list of the box world holds ({}), or a copy that cannot stand", d.len()); }
    }
}
/// the copy is put down: the next question, or the next tick, proves it from nothing
pub fn unstand(g: &mut crate::game::Game) {
    if g.col.mirror.live { crate::scene_box_mirror::box_mirror_shut(g); }
}

/// `fn_`'s questions share one proof of the rows, made by the first of them.
pub fn window<T: Clone + core::fmt::Debug>(g: &mut crate::game::Game, fn_: &mut dyn FnMut(&mut crate::game::Game) -> T) -> T {
    if standing(g) || g.col.windowed { return fn_(g); }
    g.col.windowed = true;
    let r = fn_(g);
    g.col.windowed = false;
    if standing(g) { crate::scene_box_mirror::box_mirror_shut(g); }
    r
}

/// the rows ready for a question: a window's first question proves them, and a debt is paid.
fn ready(g: &mut crate::game::Game) {
    if !standing(g) && g.col.windowed { lone(g, Some(true)); }
    if g.col.mirror.owes {
        let ids = g.col.boxes.clone();
        let mut rule = walk_rule;
        let walk: Option<&mut dyn FnMut(&mut Game, f64) -> bool> = if g.col.glass_walkable.is_some() { Some(&mut rule) } else { None };
        crate::scene_box_mirror::box_mirror_again(g, &ids, walk);
    }
}

/// a question outside a tick: true = the rows stand for it alone, and the caller shuts them behind it; `must` = whatever ANYTIME says.
fn lone(g: &mut crate::game::Game, must: Option<bool>) -> bool {
    if !g.m.scene_collision.anytime.on && !must.unwrap_or(false) { return false; }
    stand(g);
    if !standing(g) { return false; }
    if g.col.windowed {
        g.m.scene_collision.anytime.windows += 1.0;
        return false;
    }
    g.m.scene_collision.anytime.proofs += 1.0;
    true
}

/// a static was written inside a tick: the next question compares them again.
pub fn wrote(g: &mut crate::game::Game) {
    crate::scene_box_mirror::box_mirror_wrote(g);
}

/// a question the picture asks: no sound's route kept. The query stamps it swaps in the TypeScript are the twins' scratch, and the module's here.
pub fn aside<T: Clone + core::fmt::Debug>(g: &mut crate::game::Game, own: &mut crate::scene_collision::AsideOwn, fn_: &mut dyn FnMut(&mut crate::game::Game) -> T) -> T {
    let asked = g.col.sound.asking;
    g.col.sound.asking = true;
    let r = fn_(g);
    g.col.sound.asking = asked;
    r
}

// ---- circles against the boxes (rust/sim collide.rs)

/// the statics filed under a rectangle that overlap a span, once each, into `out`.
pub fn gather(g: &mut crate::game::Game, x0: f64, z0: f64, x1: f64, z1: f64, y0: f64, y1: f64, out: &mut Vec<i32>) -> f64 {
    if alone(g) {
        let r = gather_ask(g, x0, z0, x1, z1, y0, y1, out);
        crate::scene_box_mirror::box_mirror_shut(g);
        return r;
    }
    gather_ask(g, x0, z0, x1, z1, y0, y1, out)
}

fn gather_ask(g: &mut crate::game::Game, x0: f64, z0: f64, x1: f64, z1: f64, y0: f64, y1: f64, out: &mut Vec<i32>) -> f64 {
    ready(g);
    gather_here(g, x0, z0, x1, z1, y0, y1, out)
}

/// TWIN: the module's gather, its list read into `out`.
pub(super) fn gather_here(g: &crate::game::Game, x0: f64, z0: f64, x1: f64, z1: f64, y0: f64, y1: f64, out: &mut Vec<i32>) -> f64 {
    let w = slot(g);
    let n = sim::collide::gather(w, x0, z0, x1, z1, y0, y1, out.len());
    let list = &sim::world::asking(w).list;
    for i in 0..n {
        out[i] = list[i] as i32;
    }
    n as f64
}

/// the statics a circle touches over a span: how many, −1 = `out` too short.
pub fn circle_touches(g: &mut crate::game::Game, x: f64, z: f64, r: f64, y0: f64, y1: f64, out: &mut Vec<i32>) -> f64 {
    if alone(g) {
        let k = circle_touches_ask(g, x, z, r, y0, y1, out);
        crate::scene_box_mirror::box_mirror_shut(g);
        return k;
    }
    circle_touches_ask(g, x, z, r, y0, y1, out)
}

fn circle_touches_ask(g: &mut crate::game::Game, x: f64, z: f64, r: f64, y0: f64, y1: f64, out: &mut Vec<i32>) -> f64 {
    ready(g);
    circle_touches_here(g, x, z, r, y0, y1, out)
}

/// TWIN: the module's touches; `out` is left as the two steps leave it (the touching ones, then what the gather put past them).
fn circle_touches_here(g: &mut crate::game::Game, x: f64, z: f64, r: f64, y0: f64, y1: f64, out: &mut Vec<i32>) -> f64 {
    let w = slot(g);
    let y0 = foot_band(g, y0);
    let cap = if out.len() > sim::world::MAX_LIST { sim::world::MAX_LIST } else { out.len() };
    let v = sim::collide::touches(w, x, z, r, y0, y1, cap);
    let full = v < 0;
    let (n, k) = if full { (cap, -1.0) } else { ((v as u32 & 0xffff) as usize, (v as u32 >> 16) as f64) };
    let list = &sim::world::asking(w).list;
    for i in 0..n {
        out[i] = list[i] as i32;
    }
    k
}

/// a circle pushed out of the statics over a span; true = it touched.
pub fn collide_circle(g: &mut crate::game::Game, p: &mut V3, r: f64, y0: f64, y1: f64, iterations: Option<f64>) -> bool {
    if alone(g) {
        let hit = collide_circle_ask(g, p, r, y0, y1, iterations);
        crate::scene_box_mirror::box_mirror_shut(g);
        return hit;
    }
    collide_circle_ask(g, p, r, y0, y1, iterations)
}

fn collide_circle_ask(g: &mut crate::game::Game, p: &mut V3, r: f64, y0: f64, y1: f64, iterations: Option<f64>) -> bool {
    let iterations = iterations.unwrap_or(3.0);
    ready(g);
    collide_circle_here(g, p, r, y0, y1, iterations)
}

/// TWIN: the module's push, and where it ended read back.
fn collide_circle_here(g: &mut crate::game::Game, p: &mut V3, r: f64, y0: f64, y1: f64, iterations: f64) -> bool {
    let w = slot(g);
    let y0 = foot_band(g, y0);
    let hit = sim::collide::circle(w, p[0], p[2], r, y0, y1, js::to_u32(iterations)) != 0;
    let pt = sim::world::asking(w).pt;
    p[0] = pt[0];
    p[2] = pt[1];
    hit
}

/// a circle kept on the world's floor rectangle; true = it had to move.
pub fn clamp_to_world(g: &crate::game::Game, p: &mut V3, r: f64) -> bool {
    let wd = crate::scene_world::world();
    let x = m::min(wd.max[0] - r, m::max(wd.min[0] + r, p[0]));
    let z = m::min(wd.max[2] - r, m::max(wd.min[2] + r, p[2]));
    let moved = x != p[0] || z != p[2];
    p[0] = x;
    p[2] = z;
    moved
}

// ---- the glass

/// the pane rule and what it answers by, hung together.
pub fn glass_rule(g: &mut crate::game::Game, rule: Option<crate::scene_collision::PaneRule>, rows: Option<crate::scene_collision::PaneRows>) {
    g.col.pane_rule = rule;
    g.col.pane_rows = if rule.is_none() { None } else { rows };
    // (mirror.glassIs takes it only under a standing copy. `glassPanes` is the object in the TypeScript and reads nothing; here it reads `sheets`, a cache whose first read must not come as GlassBreak's constructor hangs the rule)
    if !standing(g) { return; }
    let glass = glass_panes(g);
    crate::scene_box_mirror::box_mirror_glass_is(g, glass.as_ref());
}

/// the game's glass as the world's rows read it, or none.
fn glass_panes(g: &mut crate::game::Game) -> Option<crate::scene_box_mirror::PaneSource> {
    match g.col.pane_rows {
        Some(PaneRows::GlassBreak) => {
            let (view, home, sheets) = (crate::glassbreak::glass_break_view(g), crate::glassbreak::glass_break_home(g), crate::glassbreak::glass_break_sheets(g));
            Some(crate::scene_box_mirror::PaneSource {  })
        }
        None => None,
    }
}

/// the glass was written with no question of its own between.
pub fn glass_wrote(g: &mut crate::game::Game) {
    crate::scene_box_mirror::box_mirror_glass_wrote(g);
}

/// a burst's view of the glass was put on or taken off.
pub fn glass_viewed(g: &mut crate::game::Game) {
    crate::scene_box_mirror::box_mirror_viewed(g);
}

/// `g.col.glassOpen && g.col.glassOpen(bi, p)`: false with no rule hung, else the rule glassbreak.ts:229 hangs, its whole answer at a point.
pub fn pane_open(g: &mut crate::game::Game, bi: f64, p: V3) -> bool {
    match g.col.pane_rule {
        None => false,
        Some(PaneRule::GlassBreak) => crate::glassbreak::glass_break_rule_open(g, bi, p),
    }
}

/// the pane rule as a question takes it: 0 none, 1 it has one and rows behind it, −1 a rule with no rows. EXACT.on is settled for the module, so no −2.
fn pane_ask(g: &crate::game::Game) -> f64 {
    if g.col.pane_rule.is_none() {
        0.0
    } else if g.col.pane_rows.is_some() {
        1.0
    } else {
        -1.0
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
/// the world as a question takes it, in the module's words, for the question `f`
fn over_lent<R>(g: &mut Game, over: &Over, f: impl FnOnce(&mut Game, &sim::world::Over) -> R) -> R {
    if over.plain() { let set = leaves_set(g); return f(g, &sim::world::Over::with(set)); }
    let set = if over.leaves == Leaves::None { -1 } else { leaves_set(g) };
    let but: Vec<u32> = match &over.leaves { Leaves::But(l) if set >= 0 => (0..g.col.dynamic_boxes.len() as u32).filter(|&i| l.contains(&g.col.dynamic_boxes[i as usize])).collect(), _ => Vec::new() };
    let (extra, extra_cut) = if over.extra.is_empty() { (Vec::new(), Vec::new()) } else { crate::scene_box_mirror::rows_of(g, &over.extra) };
    f(g, &sim::world::Over { set, but: &but, open: &over.open, extra: &extra, extra_cut: &extra_cut })
}
/// a question past the proof: alone it is proven for and shut behind it
fn proven<R>(g: &mut Game, f: impl FnOnce(&mut Game) -> R) -> R {
    if !alone(g) { ready(g); return f(g); }
    ready(g);
    let r = f(g);
    crate::scene_box_mirror::box_mirror_shut(g);
    r
}
/// the pane rule for a question that will be answered; below nought there is nobody else to answer
fn ask_of(g: &Game) -> u32 {
    let ask = pane_ask(g);
    if ask < 0.0 { panic!("a pane's rule the module cannot judge by: no glass stands behind it"); }
    ask as u32
}

/// a leaf is across a→b; with `need_flags`, one carrying those.
pub fn segment_blocked_dynamic(g: &mut crate::game::Game, a: V3, b: V3, need_flags: Option<f64>) -> bool {
    if alone(g) {
        let r = segment_blocked_dynamic_ask(g, a, b, need_flags);
        crate::scene_box_mirror::box_mirror_shut(g);
        return r;
    }
    segment_blocked_dynamic_ask(g, a, b, need_flags)
}

fn segment_blocked_dynamic_ask(g: &mut crate::game::Game, a: V3, b: V3, need_flags: Option<f64>) -> bool {
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

/// the furniture's ray: the nearest part met; inner None = nothing met, outer None = the module cannot answer it now.
pub fn things_ray(g: &mut crate::game::Game, ro: V3, rd: V3, tmax: f64) -> Option<Option<crate::scene_collision::ThingsRay>> {
    let r = if alone(g) {
        let r = things_ask(g, ro, rd, tmax);
        crate::scene_box_mirror::box_mirror_shut(g);
        r
    } else {
        things_ask(g, ro, rd, tmax)
    };
    r.map(|o| o.map(|t| ThingsRay { row: t.row, t: t.t, n: t.n, box_: t.box_ }))
}

fn things_ask(g: &mut crate::game::Game, ro: V3, rd: V3, tmax: f64) -> Option<Option<crate::scene_collision::ThingsAsk>> {
    ready(g);
    if !standing(g) { return None; }
    if !crate::scene_box_mirror::box_mirror_things(g) { return None; }
    let mut out = [0.0; 5];
    let k = sim::lines::ray_things(slot(g), ro[0], ro[1], ro[2], rd[0], rd[1], rd[2], tmax, &mut out);
    if k < 0 { return Some(None); }
    let at = out[sim::lines::RAY_PART];
    let box_ = match crate::scene_box_mirror::box_mirror_thing_box(g, at) {
        Some(b) => b,
        None => panic!("the Rust module's ray into the furniture met part {} of its list, which is no part of the furniture as it stands ({} things)", js::num_str(at), g.props.props.len()),
    };
    Some(Some(ThingsAsk { row: k as f64, t: out[0], n: [out[1], out[2], out[3]], box_ }))
}

/// segClear over the row's numbers
fn seg_clear_at(c: V3, h: V3, tilted: bool, a0: f64, a1: f64, a2: f64, r0: f64, r1: f64, r2: f64, len: f64, pad: f64) -> bool {
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

/// rayBoxHits over the row's numbers
fn ray_box_hits_at(c: V3, h: V3, yaw: f64, a0: f64, a1: f64, a2: f64, r0: f64, r1: f64, r2: f64, tmax: f64) -> bool {
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

/// the module's line a→b, past the proof. Panes in the way are added to `panes` where it is given (static, metres along).
fn line(g: &mut Game, over: &Over, a: V3, b: V3, shrink: f64, through: f64, panes: Option<&mut Vec<crate::scene_collision::SegmentBlockedPanesItem>>) -> bool {
    let (a0, a1, a2) = (a[0], a[1], a[2]);
    let (d0, d1, d2) = (b[0] - a0, b[1] - a1, b[2] - a2);
    let len = m::hyp3(d0, d1, d2);
    if len < 1e-4 { return false; }
    if !(len < 1e6) { return false; }   // (TRAP: a length that is no length of this world's; the walk it went to answered no)
    let thr = js::to_u32(through);
    debug_assert!(thr & !js::to_u32(crate::scene_box_mirror::mirrored()) == 0, "a flag the box world does not keep");
    let ask = if thr & sim::world::F_GLASS != 0 { 0 } else { ask_of(g) };
    let w = slot(g);
    let word = ask | if panes.is_some() { sim::lines::ASK_PANES } else { 0 };
    let r = over_lent(g, over, |g, v| sim::lines::blocked_over(w, v, a0, a1, a2, b[0], b[1], b[2], len, shrink, thr, word, &g.k));
    if r == -2 { panic!("more glass on one line than the box world lists ({})", sim::world::MAX_PANES); }
    if r > 0 {
        if let Some(out) = panes {
            let o = sim::world::asking(w);
            for i in 0..r as usize {
                out.push(SegmentBlockedPanesItem { bi: o.pane_bi[i] as f64, t: o.pane_t[i] });
            }
        }
    }
    r == -1
}

/// is a→b blocked by a static or a leaf; `through` = the flags it passes (glass by default: the line of sight).
pub fn segment_blocked(g: &mut crate::game::Game, a: V3, b: V3, shrink: Option<f64>, through: Option<f64>, panes: Option<&mut Vec<crate::scene_collision::SegmentBlockedPanesItem>>) -> bool {
    if alone(g) {
        let r = segment_blocked_ask(g, a, b, shrink, through, panes);
        crate::scene_box_mirror::box_mirror_shut(g);
        return r;
    }
    segment_blocked_ask(g, a, b, shrink, through, panes)
}

fn segment_blocked_ask(g: &mut crate::game::Game, a: V3, b: V3, shrink: Option<f64>, through: Option<f64>, panes: Option<&mut Vec<crate::scene_collision::SegmentBlockedPanesItem>>) -> bool {
    let (shrink, through) = (shrink.unwrap_or(0.02), through.unwrap_or(crate::scene_boxes::box_flag().glass));
    ready(g);
    line(g, &Over::OWN, a, b, shrink, through, panes)
}
/// …of the world as the question takes it
pub fn segment_blocked_over(g: &mut crate::game::Game, over: &Over, a: V3, b: V3, shrink: Option<f64>, through: Option<f64>) -> bool {
    let (shrink, through) = (shrink.unwrap_or(0.02), through.unwrap_or(crate::scene_boxes::box_flag().glass));
    proven(g, |g| line(g, over, a, b, shrink, through, None))
}

/// segmentBlocked with nothing passed: the solid test.
pub fn solid_blocked(g: &mut crate::game::Game, a: V3, b: V3, shrink: Option<f64>) -> bool {
    segment_blocked(g, a, b, Some(shrink.unwrap_or(0.02)), Some(0.0), None)
}
/// …of the world as the question takes it
pub fn solid_blocked_over(g: &mut crate::game::Game, over: &Over, a: V3, b: V3, shrink: Option<f64>) -> bool {
    segment_blocked_over(g, over, a, b, shrink, Some(0.0))
}

/// the line of fire: anything a round would stop in.
pub fn shot_blocked(g: &mut crate::game::Game, a: V3, b: V3, shrink: Option<f64>, energy: Option<f64>) -> bool {
    shot_blocked_over(g, &Over::OWN, a, b, shrink, energy)
}
/// …of the world as the question takes it
pub fn shot_blocked_over(g: &mut crate::game::Game, over: &Over, a: V3, b: V3, shrink: Option<f64>, energy: Option<f64>) -> bool {
    let shrink = shrink.unwrap_or(0.02);
    let pass = match g.col.fire_pass {
        Some(FirePass::Bars) => crate::scene_boxes::box_flag().bar,
        None => 0.0,
    };
    let mut panes: Vec<SegmentBlockedPanesItem> = Vec::new();
    let blocked = if over.plain() {
        segment_blocked(g, a, b, Some(shrink), Some(pass), Some(&mut panes))
    } else {
        proven(g, |g| line(g, over, a, b, shrink, pass, Some(&mut panes)))
    };
    if blocked { return blocked; }
    if panes.is_empty() { return false; }
    js::sort_by(&mut panes, |x, y| x.t - y.t);
    let d = v3::sub(b, a);
    !crate::glasspen::shot_passes(g, a, v3::scale(d, 1.0 / v3::len(d)), &panes, energy)
}

// ---- rays

/// the closest static or leaf along a ray; a leaf's index is 1e6 + its place.
pub fn raycast(g: &mut crate::game::Game, ro: V3, rd: V3, tmax: f64) -> Option<crate::scene_collision::Raycast> {
    raycast_over(g, &Over::OWN, ro, rd, tmax)
}
/// …of the world as the question takes it. A leaf's place is its place among the game's own, whichever are let through; a box the question adds is 2e6 + its place.
pub fn raycast_over(g: &mut crate::game::Game, over: &Over, ro: V3, rd: V3, tmax: f64) -> Option<crate::scene_collision::Raycast> {
    if !(tmax > 0.0) { return None; }
    let r = proven(g, |g| raycast_here(g, over, ro, rd, tmax));
    g.m.scene_collision.rays.n += 1.0;
    r
}
/// the module's ray. A ray that is no number is the caller's mistake, by name.
fn raycast_here(g: &mut Game, over: &Over, ro: V3, rd: V3, tmax: f64) -> Option<crate::scene_collision::Raycast> {
    let ask = ask_of(g) != 0;
    let mut out = [0.0; 4];
    let w = slot(g);
    let k = over_lent(g, over, |g, v| sim::lines::ray_over(w, v, ro[0], ro[1], ro[2], rd[0], rd[1], rd[2], tmax, ask, &mut out, &g.k));
    if k == sim::lines::RAY_HOST {
        panic!("a ray that is no number: from {:?} along {:?} for {}: where it starts, the way it goes and how far are numbers, and how far is finite", ro, rd, tmax);
    }
    if k < 0 { return None; }
    Some(Raycast { t: out[0], index: k as f64, n: [out[1], out[2], out[3]] })
}

// ---- the senses, the module's alone (rust/sim fog.rs, sight.rs, light.rs, smoke.rs, bricks.rs): what it cannot take is the asker's mistake, by name

/// the copy, standing, for a sense's question → which of the module's lists holds the leaves (−1 none).
fn sense(g: &mut crate::game::Game, row: &str) -> f64 {
    ready(g);
    if !standing(g) {
        panic!("{}: asked with no copy of the box world standing ({}): a sense is the Rust module's alone, asked inside a tick or a window (StaticCollision.window)", row, fell_why(g));
    }
    let d = g.col.dynamic_boxes.clone();
    let set = crate::scene_box_mirror::box_mirror_leaves(g, &d);
    if set == -2.0 {
        panic!("{}: {} leaves, and a list of the module's holds {} (game/levelfit.ts holds a level to that as it loads)", row, d.len(), sim::world::MAX_LEAVES);
    }
    set
}

/// the pane rule as a sense's question takes it: 0 none, 1 it has one and the copy holds its rows.
fn sense_panes(g: &mut crate::game::Game, row: &str) -> f64 {
    if g.col.pane_rule.is_none() { return 0.0; }
    if !crate::scene_box_mirror::box_mirror_glass_in(g) {
        let why = if g.col.pane_rows.is_none() { "it was hung with no rows behind it: glassRule hands the rule and its rows together" } else { "more broken panes, or a view of more panes, than the copy holds" };
        panic!("{}: a pane's rule the module cannot judge by ({})", row, why);
    }
    1.0
}

/// an eye's line, and the fog's: it passes glass, and is asked by flags the copy keeps.
fn eye_line(g: &crate::game::Game, row: &str, through: f64) {
    if js::to_u32(through) & !js::to_u32(crate::scene_box_mirror::mirrored()) != 0 || js::to_u32(through) & sim::world::F_GLASS == 0 {
        panic!("{}: an eye's line passes glass and names only flags the copy keeps (asked through {}): one that stops at glass is no eye's, and is asked as a line (segmentBlocked)", row, js::num_str(through));
    }
}

/// a sight pass opens: its `n` eyes are written next. More than a question names is a mistake by name.
pub fn sight_open(g: &mut crate::game::Game, n: f64) {
    sense(g, "fog");
    if n > SIGHT_EYES {
        panic!("fog: a sight pass of {} eyes, and a question names {} (rust/sim/src/fog.rs; the fog's own most is render/fogofwar.ts FOG_EYES)", js::num_str(n), js::num_str(SIGHT_EYES));
    }
}

/// eye `k` of the pass.
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

/// which eyes have a line: how many × 2²⁸ + those eyes, four bits each.
pub fn eyes_see(g: &mut crate::game::Game, order: f64, n: f64, want: f64, kind: f64, through: f64, a: V3, b: V3) -> f64 {
    eye_line(g, "fog", through);
    let set = sense(g, "fog") as i32;
    let r = sim::fog::see(slot(g), set, js::to_u32(order), js::to_u32(n), js::to_u32(want), js::to_u32(kind), js::to_u32(through), a[0], a[1], a[2], b[0], b[1], b[2], &g.k);
    if r == ASK_HOST { panic!("{}", handed_back_words("fog", "a length that is no length of this world's, or more eyes than a question names", &a, &b)); }
    r as f64
}

/// an eye on a man's chest and head as one question.
pub fn eye_contact(g: &mut crate::game::Game, e: crate::scene_collision::EyeContactE, chest: V3, head: V3, unseen: bool, smoked: bool, through: f64) -> crate::scene_collision::EyeContact {
    eye_line(g, "sight", through);
    let q = EyeContactAskE { pos: e.pos, look: e.look, range: e.range, half_deg: e.half_deg, near_m: e.near_m, near_deg: e.near_deg, v_half_deg: e.v_half_deg };
    let alone_now = !standing(g) && lone(g, Some(true));
    let c = eye_contact_ask(g, q, chest, head, unseen, smoked, through);
    if alone_now { crate::scene_box_mirror::box_mirror_shut(g); }
    EyeContact { visible: c.visible, dist: c.dist, in_cone: c.in_cone }
}

fn eye_contact_ask(g: &mut crate::game::Game, e: crate::scene_collision::EyeContactAskE, chest: V3, head: V3, unseen: bool, smoked: bool, through: f64) -> crate::scene_collision::EyeContactAsk {
    let set = sense(g, "sight") as i32;
    let q = (if e.v_half_deg.is_some() { sim::sight::Q_LENS } else { 0 }) | (if unseen { sim::sight::Q_UNSEEN } else { 0 }) | (if smoked { sim::sight::Q_SMOKED } else { 0 });
    let mut dist = 0.0;
    let r = sim::sight::contact(slot(g), set, js::to_u32(through), q, e.pos, e.look, e.range, e.half_deg, e.near_m, e.near_deg, e.v_half_deg.unwrap_or(0.0), chest, head, &mut dist, &g.k);
    if r == ASK_HOST { panic!("{}", handed_back_words("sight", "a line too long to be one, or of no number", &e.pos, &chest)); }
    g.m.scene_collision.sensed.sight += 1.0;
    EyeContactAsk { visible: r & sim::sight::A_VISIBLE != 0, dist, in_cone: r & sim::sight::A_IN_CONE != 0 }
}

/// a question about the light begins; `shade` = the smoke dims each lamp on its way, `rows` = what it says of the level's own lamps, which the module reads where they stand.
pub fn lamps_begin(g: &mut crate::game::Game, p: V3, shade: bool, rows: Option<crate::scene_collision::LampRows>) {
    lamps_over(g);
    g.col.mirror.lamp_set = sense(g, "light");
    g.col.mirror.lamp_n = 0.0;
    g.col.lamp_q.stands = if shade { 1.0 } else { 0.0 };
    g.col.lamp_q.p = Some(p);
    g.col.lamp_q.rows = rows;
    g.col.lamp_q.on = true;
}

/// no question about the light stands.
fn lamps_over(g: &mut crate::game::Game) {
    let q = &mut g.col.lamp_q;
    q.on = false;
    q.p = None;
    q.stands = 0.0;
    q.parts = 0.0;
    q.rows = None;
}

/// the lamps written so far, asked as a part; `last`: no part follows, and the rows after the last written lamp are taken; the six faces so far into `out` (the module's scratch, in the TypeScript)
fn lamps_part_into(g: &mut Game, last: bool, out: &mut [f64; 6]) {
    let w = slot(g);
    let q = g.col.lamp_q;
    let p = q.p.unwrap_or([0.0; 3]);
    let more = q.parts > 0.0;
    let (set, n) = (g.col.mirror.lamp_set as i32, g.col.mirror.lamp_n as usize);
    let rows = rows_for(g, q.rows);
    let r = if q.stands != 0.0 {
        let shade = sim::light::Shade { sigma: g.k.sense.smoke.sigma, g: g.sim, stamp: sim::state::stamp(g.sim) };
        sim::light::asked(w, set, n, p, &shade, more, rows, last, out, &g.k)
    } else {
        sim::light::asked(w, set, n, p, &sim::light::NO_SHADE, more, rows, last, out, &g.k)
    };
    g.col.lamp_q.parts += 1.0;
    g.col.mirror.lamp_n = 0.0;
    if r == ASK_HOST {
        lamps_over(g);
        panic!("{}", handed_back_words("light", "a lamp's line too long to be one, or of no number", &p, &p));
    }
}

/// the lamps written so far, asked as a part of the question.
fn lamps_part(g: &mut crate::game::Game, last: Option<bool>) {
    let mut out = [0.0; 6];
    lamps_part_into(g, last.unwrap_or(false), &mut out);
}

/// one lamp of the question, or of the grid's sweep; past the table's room a question sends the table on as a part, and a sweep answers false. `rows`: how many of the level's rows stand before it in the engine's list.
pub fn lamp(g: &mut crate::game::Game, pos: V3, range: f64, radius: f64, kind: f64, dir: V3, outer_deg: f64, inner_deg: f64, color: V3, intensity: f64, rows: Option<f64>) -> bool {
    let rows = rows.unwrap_or(0.0);
    if g.col.mirror.lamp_n as usize >= sim::light::MAX_LAMPS {
        if !g.col.lamp_q.on { return false; }
        g.m.scene_collision.lamps.parts += 1.0;
        lamps_part(g, None);
    }
    let at = sim::light::lamps_addr();
    let o = g.col.mirror.lamp_n as usize * sim::light::LAMP;
    g.col.mirror.lamp_n += 1.0;
    let row = [pos[0], pos[1], pos[2], range, radius, kind, dir[0], dir[1], dir[2], outer_deg, inner_deg, color[0], color[1], color[2], intensity, rows];
    for (i, v) in row.iter().enumerate() {
        put_f64(at, o + i, *v);
    }
    true
}

/// the last part asked, and the six faces of the whole question read into `acc`.
pub fn lamps_on(g: &mut crate::game::Game, acc: &mut Vec<f64>) {
    if !g.col.lamp_q.on { panic!("light: lampsOn with no question begun (lampsBegin)"); }
    let mut out = [0.0; 6];
    lamps_part_into(g, true, &mut out);
    lamps_over(g);
    for k in 0..6 {
        if k < acc.len() { acc[k] = out[k]; } else { acc.push(out[k]); }
    }
    g.m.scene_collision.sensed.light += 1.0;
}

/// a sound's or a search's question: the leaves' list and the pane rule to ask with.
fn sense_for(g: &mut crate::game::Game, row: crate::scene_collision::SenseForRow) -> crate::scene_collision::SenseFor {
    let set = sense(g, row.as_str());
    SenseFor { set, ask: sense_panes(g, row.as_str()) }
}

/// the boxes and leaves a grid is asked over: is the copy of them stood for this alone (it is shut again after), and the leaves' list. None = there is no asking
fn grid_over(g: &mut crate::game::Game) -> Option<(bool, i32)> {
    ready(g);
    let mine = !standing(g);
    if mine {
        open_copy(g, false);   // (with no question of the game's sent to it)
        if !standing(g) { return None; }
    }
    let d = g.col.dynamic_boxes.clone();
    let set = crate::scene_box_mirror::box_mirror_leaves(g, &d);
    if set == -2.0 {
        if mine { crate::scene_box_mirror::box_mirror_shut(g); }
        return None;
    }
    Some((mine, set as i32))
}

/// the ways between the coarse grid's cells are asked of the boxes and leaves as they stand: a game put back knows none till its next step
pub fn grid_ways(g: &mut crate::game::Game) {
    let Some((mine, set)) = grid_over(g) else { return };
    let copy = crate::scene_box_mirror::box_mirror_grid_copy(g);
    sim::bricks::ways(g.sim, sim::state::stamp(g.sim), slot(g), copy, set, &g.k);
    if mine { crate::scene_box_mirror::box_mirror_shut(g); }
}

/// `n` steps of the coarse grid's smoke over the boxes and leaves as they stand. False = nothing was done.
pub fn grid_step(g: &mut crate::game::Game, n: f64, k: crate::scene_collision::GridStepK, sweep: f64, wind: Option<&crate::scene_collision::GridWind>) -> bool {
    let Some((mine, set)) = grid_over(g) else { return false };
    // the light that comes round: the only `lamps` hung is guards.ts steadyLamps, which writes the lamps of nobody's fixture and answers with the rows' word
    let mut lamps = -1i32;
    let mut said_rows: Option<LampRows> = None;
    g.col.mirror.lamp_n = 0.0;
    lamps_over(g);
    match crate::guards::steady_lamps(g) {
        crate::guards::SteadyLamps::Bool(false) => {}
        crate::guards::SteadyLamps::Bool(true) => { lamps = g.col.mirror.lamp_n as i32; }
        crate::guards::SteadyLamps::LampRows(r) => { lamps = g.col.mirror.lamp_n as i32; said_rows = Some(r); }
    }
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
    let stamp = sim::state::stamp(g.sim);
    let copy = crate::scene_box_mirror::box_mirror_grid_copy(g);
    let (w, block) = (slot(g), g.sim);
    let rows = rows_for(g, said_rows);
    let gk = if lamps >= 0 { Some(sim::glow::K { sweep, sky, lamps: lamps as usize, rows }) } else { None };
    let r = sim::bricks::step(block, stamp, w, copy, set, js::to_u32(n), &bk, gk.as_ref(), &g.k);
    if mine { crate::scene_box_mirror::box_mirror_shut(g); }
    r != ASK_HOST
}

/// What the light under a roof is worked out from is made as a game stands, behind the loading, and not in its first step: the statics' lines, no state.
pub fn light_ready(g: &mut crate::game::Game) {
    let to_sun = crate::hours::sky_now(g).to_sun;
    window(g, &mut |g: &mut Game| {
        ready(g);
        if standing(g) { sim::bricks::light_ready(slot(g), to_sun, &g.k); }
    });
}

// ---- how a sound reaches a listener

fn sound_paths_new(g: &crate::game::Game) -> crate::scene_collision::SoundPaths {
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

/// how a sound at `src` reaches `dst`; `keep` false = the answer is not kept.
pub fn sound_paths_propagate(g: &mut crate::game::Game, src: V3, dst: V3, keep: Option<bool>) -> crate::scene_collision::Propagation {
    let keep = keep.unwrap_or(true);
    if g.m.scene_collision.priced.waits {
        panic!("a sound was asked for from inside the price of a search: its way round is a search, and the module has one search's tables (rust/sim nav.rs find_priced)");
    }
    if !standing(g) {
        window(g, &mut |g: &mut Game| sound_paths_propagate_ask(g, src, dst, keep))
    } else {
        sound_paths_propagate_ask(g, src, dst, keep)
    }
}

/// how a sound at `src` reaches `dst`, from the module's two questions with this file's memory of routed answers between them.
fn sound_paths_propagate_ask(g: &mut crate::game::Game, src: V3, dst: V3, keep: bool) -> crate::scene_collision::Propagation {
    let s = sense_for(g, SenseForRow::Sound);
    let (set, ask) = (s.set as i32, s.ask as u32);
    let straight = match heard_straight(g, set, ask, src, dst) {
        Ok((r, asked)) => { if asked { g.m.scene_collision.sensed.sound += 1.0; } return r; }
        Err(straight) => straight,
    };
    let key = route_key(g, src, dst);
    if let Some(hit) = g.col.sound.cache.get(key.clone()) {
        if m::abs(g.col.sound.now - hit.t) < 0.5 { return hit.r; }
    }
    let Some(r) = heard_round(g, set, ask, src, dst, straight) else { panic!("{}", handed_back_words("sound", "a line of its way round too long to be one", &src, &dst)) };
    g.m.scene_collision.sensed.sound += 1.0;
    if keep && !g.col.sound.asking {
        let t = g.col.sound.now;
        g.col.sound.cache.set(key, SoundPathsCacheValue { t, r });
    }
    r
}

/// the first half of how a sound reaches: straight, over the top, or too far walled off for a way round (Ok: its answer, and whether it counts as asked) · Err: the
/// straight distance, walled off, and the way round is to be asked
fn heard_straight(g: &Game, set: i32, ask: u32, src: V3, dst: V3) -> Result<(Propagation, bool), f64> {
    let mut straight = 0.0;
    let c = sim::hear::direct(slot(g), set, ask, src, dst, &mut straight, &g.k);
    if c == ASK_HOST { panic!("{}", handed_back_words("sound", "a straight line too long to be one, or of no number", &src, &dst)); }
    if c < sim::hear::OVER {
        let mut graze = 0.0;
        if c & sim::hear::GRAZE_A != 0 { graze += 0.1; }
        if c & sim::hear::GRAZE_B != 0 { graze += 0.1; }
        return Ok((Propagation { direct: true, routed: false, path_len: straight, via: v3::copy(src), apparent: v3::copy(src), muffle: graze, carry: 1.0 }, true));
    }
    if c == sim::hear::OVER {
        return Ok((Propagation { direct: true, routed: false, path_len: straight, via: v3::copy(src), apparent: v3::copy(src), muffle: 0.15, carry: 0.9 }, true));
    }
    if c == sim::hear::FAR { return Ok((sealed(src, straight), true)); }
    Err(straight)
}
/// the second half: round by the floor, or sealed (None: no answer to give — a line of it is none of this world's, or a search waits for its prices)
fn heard_round(g: &Game, set: i32, ask: u32, src: V3, dst: V3, straight: f64) -> Option<Propagation> {
    nav_grid_there(g);
    let (nw, nh, cs) = (g.col.nav.w as usize, g.col.nav.h as usize, g.col.nav.cell);
    let seal = js::to_u32(crate::scene_boxes::box_flag().sound_seal);
    let mut out = [0.0; 4];
    let k = sim::hear::route(slot(g), set, ask, seal, nw, nh, cs, src, dst, straight, &mut out, &g.k);
    if k == ASK_HOST { return None; }
    if k == sim::hear::SEALED { return Some(sealed(src, straight)); }
    let (l, via) = (out[0], [out[1], out[2], out[3]]);
    let dir = v3::normalize([via[0] - dst[0], 0.0, via[2] - dst[2]]);
    let muffle = if k == sim::hear::ROUTED_DOOR { 0.85 } else { 0.35 };
    Some(Propagation { direct: false, routed: true, path_len: l, via, apparent: [dst[0] + dir[0] * l, src[1], dst[2] + dir[2] * l], muffle, carry: 1.0 - muffle * 0.8 })
}

// ---- the same questions of the game shared (ask::WorldAsk), the copy standing
/// the pane rule, as a question of the game shared takes it
fn pane_word(g: &Game) -> u32 { if g.col.pane_rule.is_some() { sim::lines::ASK_GLASS } else { 0 } }
/// which list of the leaves is the game's own, the copy standing
fn own_set(g: &Game) -> i32 { if g.col.dynamic_boxes.is_empty() { -1 } else { 0 } }
fn lent<R>(g: &Game, over: &Over, f: impl FnOnce(&sim::world::Over) -> R) -> R {
    let set = if over.leaves == Leaves::None { -1 } else { own_set(g) };
    if over.plain() { return f(&sim::world::Over::with(set)); }
    let but: Vec<u32> = match &over.leaves { Leaves::But(l) if set >= 0 => (0..g.col.dynamic_boxes.len() as u32).filter(|&i| l.contains(&g.col.dynamic_boxes[i as usize])).collect(), _ => Vec::new() };
    let (extra, extra_cut) = if over.extra.is_empty() { (Vec::new(), Vec::new()) } else { crate::scene_box_mirror::rows_of(g, &over.extra) };
    f(&sim::world::Over { set, but: &but, open: &over.open, extra: &extra, extra_cut: &extra_cut })
}
/// (ask::WorldAsk's line: `through` = the flags it passes)
pub(super) fn line_shared(g: &Game, over: &Over, a: V3, b: V3, shrink: f64, through: f64) -> bool {
    let len = m::hyp3(b[0] - a[0], b[1] - a[1], b[2] - a[2]);
    if !(len >= 1e-4) || !(len < 1e6) { return false; }
    let thr = js::to_u32(through);
    let word = if thr & sim::world::F_GLASS != 0 { 0 } else { pane_word(g) };
    lent(g, over, |v| sim::lines::blocked_over(slot(g), v, a[0], a[1], a[2], b[0], b[1], b[2], len, shrink, thr, word, &g.k)) == -1
}
/// (ask::WorldAsk's ray)
pub(super) fn ray_shared(g: &Game, over: &Over, ro: V3, rd: V3, tmax: f64) -> Option<crate::scene_collision::Raycast> {
    if !(tmax > 0.0) { return None; }
    let mut out = [0.0; 4];
    let k = lent(g, over, |v| sim::lines::ray_over(slot(g), v, ro[0], ro[1], ro[2], rd[0], rd[1], rd[2], tmax, pane_word(g) != 0, &mut out, &g.k));
    if k < 0 { return None; }   // (a ray that is no number meets nothing here: a host asks it of a camera or a cursor, and the game is not to fall for it)
    Some(Raycast { t: out[0], index: k as f64, n: [out[1], out[2], out[3]] })
}
/// (ask::WorldAsk's leaf: one across a→b; with `need`, one carrying those flags)
pub(super) fn leaf_shared(g: &Game, a: V3, b: V3, need: Option<f64>) -> bool {
    let len = m::hyp3(b[0] - a[0], b[1] - a[1], b[2] - a[2]);
    if !(len >= 1e-4) || !(len < 1e6) { return false; }
    let set = own_set(g);
    set >= 0 && sim::lines::leaves_blocked(slot(g), a[0], a[1], a[2], b[0], b[1], b[2], len, js::to_u32(need.unwrap_or(0.0)), set)
}
/// (ask::WorldAsk's furniture: the nearest part met, of a copy that has every write of the furniture; one that could not take them all, or owes one, meets nothing)
pub(super) fn things_shared(g: &Game, ro: V3, rd: V3, tmax: f64) -> Option<(f64, V3)> {
    if !g.col.mirror.things_ok || g.col.mirror.things_at != g.m.scene_box_mirror.things.n { return None; }
    let mut out = [0.0; 5];
    let k = sim::lines::ray_things(slot(g), ro[0], ro[1], ro[2], rd[0], rd[1], rd[2], tmax, &mut out);
    if k < 0 { None } else { Some((out[0], [out[1], out[2], out[3]])) }
}
/// (ask::WorldAsk's way: the module's plain search, on a grid written to it for the question. A place that is no number has no way, and the game is not to fall for it)
pub(super) fn way_shared(g: &Game, a: V3, b: V3, max_iter: u32) -> Option<Vec<V3>> {
    nav_grid_there(g);
    let n = sim::nav::find_path(slot(g), own_set(g), pane_word(g), g.col.nav.w as usize, g.col.nav.h as usize, g.col.nav.cell, a[0], a[2], b[0], b[2], max_iter, &g.k);
    if n == sim::nav::NO_WAY || n == sim::nav::ASK_HOST { return None; }
    let at = sim::nav::way_addr();
    Some((0..n as usize).map(|i| [get_f64(at, i * 2), 0.0, get_f64(at, i * 2 + 1)]).collect())
}
/// (ask::WorldAsk's path of sound: no route is remembered and none is looked up. A place that is no number, or no place of this world, is heard from nowhere: a host asks
/// it of a sound and a listener of its own, and the game is not to fall for it)
pub(super) fn heard_shared(g: &Game, src: V3, dst: V3) -> crate::scene_collision::Propagation { heard_shared_by(g, src, dst, |g, find| find(g)) }
/// …and who remembers the ways round, if anybody does (`way`: asks for one, or has it)
pub(super) fn heard_shared_by(g: &Game, src: V3, dst: V3, way: impl FnOnce(&Game, &dyn Fn(&Game) -> Propagation) -> Propagation) -> crate::scene_collision::Propagation {
    let len = m::hyp3(src[0] - dst[0], src[1] - dst[1], src[2] - dst[2]);
    if !(len < 1e5) { return Propagation { direct: false, routed: false, path_len: 1e5, via: dst, apparent: dst, muffle: 1.0, carry: 0.0 }; }
    let (set, ask) = (own_set(g), pane_word(g));
    match heard_straight(g, set, ask, src, dst) { Ok((r, _)) => r, Err(straight) => way(g, &|g| heard_round(g, set, ask, src, dst, straight).unwrap_or_else(|| sealed(src, straight))) }
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
fn nav_grid_new(g: &mut crate::game::Game) -> crate::scene_collision::NavGrid {
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
pub fn nav_grid_cell_blockers(g: &mut crate::game::Game, wx: f64, wz: f64, out: &mut Vec<i32>) -> f64 {
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

/// A* from a to b, smoothed; `penalty` a cost multiplier for stepping into a cell.
pub fn nav_grid_find_path(g: &mut crate::game::Game, a: V3, b: V3, max_iter: Option<f64>, penalty: Option<&mut dyn FnMut(&mut crate::game::Game, f64, f64) -> f64>) -> Option<Vec<V3>> {
    let max_iter = max_iter.unwrap_or(6000.0);
    if g.m.scene_collision.priced.waits {
        panic!("a way was asked for from inside the price of another search: the module has one search's tables (rust/sim nav.rs find_priced)");
    }
    let mut penalty = penalty;
    if !standing(g) {
        window(g, &mut |g: &mut Game| nav_grid_find_path_ask(g, a, b, max_iter, relend(&mut penalty)))
    } else {
        nav_grid_find_path_ask(g, a, b, max_iter, penalty)
    }
}

/// the grid as it stands, written to the module for one question of its search. A floor larger than its table is the build's mistake, by name.
fn nav_grid_there(g: &crate::game::Game) {
    let n = g.col.nav.blocked.len();
    if n > sim::nav::MAX_NAV {
        panic!("the floor's grid has {} cells and a search of the Rust module holds {} (rust/sim/src/nav.rs MAX_FLOOR): the floor is the build's (scene/world.ts WORLD), so the module's tables are to be made larger with it", n, sim::nav::MAX_NAV);
    }
    unsafe { core::ptr::copy_nonoverlapping(g.col.nav.blocked.as_ptr(), sim::nav::shut_addr() as *mut u8, n); }
}

/// the way from the module's search; a price is asked of the caller where the search comes to a cell it has none for.
fn nav_grid_find_path_ask(g: &mut crate::game::Game, a: V3, b: V3, max_iter: f64, penalty: Option<&mut dyn FnMut(&mut crate::game::Game, f64, f64) -> f64>) -> Option<Vec<V3>> {
    if js::to_u32(max_iter) as f64 != max_iter { panic!("paths: a cap on the search that is no whole number ({})", js::num_str(max_iter)); }
    let mut penalty = penalty;
    let s = sense_for(g, SenseForRow::Paths);
    let (set, ask) = (s.set as i32, s.ask as u32);
    nav_grid_there(g);
    let (w, nw, nh, cell) = (slot(g), g.col.nav.w as usize, g.col.nav.h as usize, g.col.nav.cell);
    let mi = js::to_u32(max_iter);
    let mut n;
    if penalty.is_none() {
        n = sim::nav::find_path(w, set, ask, nw, nh, cell, a[0], a[2], b[0], b[2], mi, &g.k);
    } else {
        n = sim::nav::find_priced(w, set, ask, nw, nh, cell, a[0], a[2], b[0], b[2], mi, &g.k);
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
    if n == sim::nav::ASK_HOST { panic!("{}", handed_back_words("paths", "a point that is no number, or a line too long to be one", &a, &b)); }
    g.m.scene_collision.sensed.paths += 1.0;
    if n == sim::nav::NO_WAY { return None; }
    let at = sim::nav::way_addr();
    let mut out: Vec<V3> = Vec::with_capacity(n as usize);
    for i in 0..n as usize {
        out.push([get_f64(at, i * 2), 0.0, get_f64(at, i * 2 + 1)]);
    }
    Some(out)
}

/// the prices the module's search has asked for, from the caller's function, in the order asked.
fn nav_grid_price_these(g: &mut crate::game::Game, k: f64, penalty: &mut dyn FnMut(&mut crate::game::Game, f64, f64) -> f64) {
    let nw = g.col.nav.w as u32;
    for i in 0..k as usize {
        let c = get_u32(sim::nav::need_addr(), i);
        let v = penalty(g, (c % nw) as f64, (c / nw) as f64);
        put_f64(sim::nav::price_addr(), c as usize, v);
    }
    g.m.scene_collision.priced.cells += k;
}

/// the free cell nearest by ring to a shut one; with the point that asked, on its own side of the wall.
fn nav_grid_nearest_free(g: &mut crate::game::Game, gx: f64, gz: f64, wx: Option<f64>, wz: Option<f64>) -> Option<[f64; 2]> {
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
pub fn nav_grid_nearest_free_point(g: &mut crate::game::Game, wx: f64, wz: f64) -> Option<V3> {
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
pub fn wall_between(g: &mut crate::game::Game, a: V3, b: V3) -> bool {
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
    fn the_twins_helpers_answer_as_the_typescripts_do() {
        // (the answers of segClear and rayBoxHits under bun with EXACT.on, over the same 256 made-up lines and boxes)
        let want_clear = "1010101111011110101111111011101100111001101000111011111111111011111001111101101111011111001111110011011011111010111011101110111100011111110011110111011011111111011100000111111000110000111011110000101101110111111101101001011111111111001010110110110111011111";
        let want_hits = "0000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000100000000100000000000000000000100000000000000000000000000000000000011000000010000001000000000101000000001000000000010000000000000000000000000000000000000000";
        let mut x = 12345.0;
        let mut r = |lo: f64, hi: f64| { x = js::rem(x * 16807.0, 2147483647.0); lo + (hi - lo) * (x / 2147483647.0) };
        let (mut clear, mut hits) = (String::new(), String::new());
        for i in 0..256 {
            let (a, e): (V3, V3) = ([r(0.0, 12.0), r(0.0, 3.0), r(0.0, 12.0)], [r(0.0, 12.0), r(0.0, 3.0), r(0.0, 12.0)]);
            let d = [e[0] - a[0], e[1] - a[1], e[2] - a[2]];
            let len = m::sqrt(d[0] * d[0] + d[1] * d[1] + d[2] * d[2]);
            let u = [d[0] / len, if i % 7 == 0 { 0.0 } else { d[1] / len }, d[2] / len];
            let (c, h, yaw) = ([r(0.0, 12.0), r(0.0, 3.0), r(0.0, 12.0)], [r(0.1, 2.0), r(0.1, 1.5), r(0.1, 2.0)], r(-3.0, 3.0));
            clear.push(if seg_clear_at(c, h, i % 5 == 0, a[0], a[1], a[2], u[0], u[1], u[2], len, 0.05) { '1' } else { '0' });
            hits.push(if ray_box_hits_at(c, h, yaw, a[0], a[1], a[2], u[0], u[1], u[2], len) { '1' } else { '0' });
        }
        assert_eq!(clear, want_clear);
        assert_eq!(hits, want_hits);
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
        let r = |steady| LampRows { leave: 4.0, steady };
        assert_eq!(rows_how(None), 0.0);
        assert_eq!(rows_how(Some(r(false))), 1.0);
        assert_eq!(rows_how(Some(r(true))), 3.0);
        let mut g = Box::new(Game::blank());
        g.sim = 2;
        let rows = rows_for(&g, Some(r(true))).unwrap();
        assert_eq!((rows.g, rows.leave, rows.steady), (2, 4, true));
        assert!(rows_for(&g, None).is_none());
    }

    #[test]
    fn a_lamp_is_sixteen_doubles_and_the_last_is_the_rows_before_it() {
        assert_eq!(sim::light::LAMP, 16);
        let mut g = Box::new(Game::blank());
        let g = &mut *g;
        assert!(lamp(g, [1.0, 2.0, 3.0], 4.0, 5.0, 6.0, [7.0, 8.0, 9.0], 10.0, 11.0, [12.0, 13.0, 14.0], 15.0, Some(16.0)));
        assert!(lamp(g, [1.0, 2.0, 3.0], 4.0, 5.0, 6.0, [7.0, 8.0, 9.0], 10.0, 11.0, [12.0, 13.0, 14.0], 15.0, None));
        let at = sim::light::lamps_addr();
        for i in 0..16 { assert_eq!(get_f64(at, i), (i + 1) as f64); }
        assert_eq!(get_f64(at, 16 + 15), 0.0);
        assert_eq!(g.col.mirror.lamp_n, 2.0);
    }

    #[test]
    fn small_ones() {
        assert_eq!(clampi(-1.0, 0.0, 3.0), 0.0);
        assert_eq!(clampi(4.0, 0.0, 3.0), 3.0);
        assert!(clampi(f64::NAN, 0.0, 3.0).is_nan());
    }
}
