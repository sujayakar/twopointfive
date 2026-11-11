#![allow(unused_imports, unused_variables)]
pub use ::game_types::scene_boxes::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{string::String, sync::Arc, vec, vec::Vec};

// THE BOX TABLES' MAKERS. A box of the world is a row of `g.tab.boxes` and is never
// dealt again, so its number is the same on every host. A box of the frame, or one its maker keeps a while, is a row of `g.tab.frame_boxes`: a row is freed by
// the tick's own listing, not by a host's drawing, but a freed row is dealt again: it names a box only while its holder holds it.

/// WHAT THE PICTURE'S TABLE (`WORLD.max_boxes`) KEEPS FOR THE FRAME'S OWN BOXES: the men, the props, what is in flight. A level's walls and what is shed of them
/// stay out of it. It is a number of the game and not a count of this frame's list, so that what the game makes of a broken wall is the same on every host.
pub const FRAME_BOXES: f64 = 768.0;

/// a box made to stay: a row of g.tab.boxes for good
pub fn box_new(g: &mut Game, b: crate::scene_boxes::Box) -> BoxId { g.tab.boxes.push(b) }
/// a box made for this frame's list: a row freed by boxes_frame_reset is used again, else a new row
pub fn box_frame(g: &mut Game, b: crate::scene_boxes::Box) -> FrameBoxId {
    let id = box_kept(g, b);
    g.box_frame_rows.push(id);
    id
}
/// THE BOX WORLD LISTED AS A TICK OPENS, AND THE ROWS OF THE FRAMES GONE BY FREE AGAIN. Two lists hold a frame box's ID: the frame's own (`g.dyn_boxes`, made
/// again every tick) and the engine's (`g.engine.world`, made again here). A row is freed when it is not one of the frame that was just listed.
pub fn box_world_listed(g: &mut Game) {
    crate::scene_boxes::box_world_list(g);
    boxes_frame_reset(g);
}
fn boxes_frame_reset(g: &mut Game) {
    let rows = core::mem::take(&mut g.box_frame_rows);
    let mut live = vec![false; g.tab.frame_boxes.len()];
    for b in g.dyn_boxes.iter().chain(g.engine.world.dynamics.iter()) { if let AnyBoxId::Frame(f) = *b { live[f.ix()] = true; } }
    for id in rows {
        if live[id.ix()] { g.box_frame_rows.push(id); continue; }
        // (what a maker noted beside the box that stood in this row, a thin line's width or an infrared mark, goes with the box)
        crate::thinline::thin_forget(g, id.into());
        crate::irlaser::ir_forget(g, id.into());
        g.box_frame_free.push(id);
    }
}

/// A BOX ITS MAKER KEEPS FOR A WHILE AND THEN LETS GO (a remembered thing's copy in the fog of war, an fx row's box): a freed row used again, else a new
/// row. Not one of the frame's rows: it stands until its holder hands it to `box_let_go`.
pub fn box_kept(g: &mut Game, b: crate::scene_boxes::Box) -> FrameBoxId {
    match g.box_frame_free.pop() { Some(id) => { g.tab.frame_boxes[id] = b; id } None => g.tab.frame_boxes.push(b) }
}
/// The holder of a kept box has dropped it. The frame's list or the engine's may still name it, so it is not free at
/// once: it joins the frame's rows, and the next listing frees it once it is on no list (`box_world_listed`).
pub fn box_let_go(g: &mut Game, id: FrameBoxId) { g.box_frame_rows.push(id); }

/// makeBox for a box pushed straight onto the frame's list: a row for this frame, of which its caller keeps nothing
pub fn make_box_frame(g: &mut Game, p: &crate::scene_boxes::MakeBoxP) -> FrameBoxId { let b = crate::scene_boxes::make_box_value(p); box_frame(g, b) }
/// makeBox for a box its maker keeps until it hands it to `box_let_go` (one that stands by a short timer on `g.fx`)
pub fn make_box_kept(g: &mut Game, p: &crate::scene_boxes::MakeBoxP) -> FrameBoxId { let b = crate::scene_boxes::make_box_value(p); box_kept(g, b) }
/// boxOf for a box pushed straight onto the frame's list: a row for this frame
pub fn box_of_frame(g: &mut Game, c: V3, h: V3, yaw: f64, albedo: V3, emissive: V3, flags: f64, owner: f64, rot: Option<Quat>) -> FrameBoxId {
    let b = crate::scene_boxes::make_box_value(&crate::scene_boxes::MakeBoxP { c, h, yaw: Some(yaw), albedo: Some(albedo), emissive: Some(emissive), flags: Some(flags), owner: Some(owner), rot, ..Default::default() });
    box_frame(g, b)
}

/// the game's Box (the header's `Box` is alloc's)
type Bx = crate::scene_boxes::Box;

// the browser's GPUBufferUsage words: SKELETON-GAP, no entry in the index

/// A piece of the picture, which no game ray meets.
fn look_keyed(g: &crate::game::Game, b: AnyBoxId) -> bool {
    let (dress, mkey) = { let x = &g.tab[b]; (x.dress, x.mkey) };
    dress == Some(true) || (match mkey { Some(k) => js::to_u32(k) >> 24 == js::to_u32(crate::scene_boxes::mkey().look) >> 24, None => false })
}

/// boxOf's value, before it has a place in the table
pub fn box_value(c: V3, h: V3, yaw: f64, albedo: V3, emissive: V3, flags: f64, owner: f64, rot: Option<Quat>) -> Bx {
    Bx { c, h, yaw, albedo, emissive, flags, owner, cut_height: 1e6, rot, ..Bx::default() }
}

/// Is this piece of trim drawn: while the box that says it is.
fn trim_drawn(g: &crate::game::Game, all: &[BoxId], t: BoxId) -> bool {
    match g.tab.boxes[t].of {
        None => true,
        Some(of) => js::to_i32(g.tab.boxes[all[of as usize]].flags) & js::to_i32(box_flag().no_primary) == 0,
    }
}

/// Is a static drawn: not hidden, not a pane, and trim only while its box is.
pub fn box_drawn(g: &crate::game::Game, all: &[BoxId], b: BoxId) -> bool {
    let f = box_flag();
    let (flags, piece) = { let x = &g.tab.boxes[b]; (x.flags, x.piece) };
    js::to_i32(flags) & (js::to_i32(f.no_primary) | js::to_i32(f.glass)) == 0 && (piece != Some(true) || trim_drawn(g, all, b))
}

/// makeBox's object as a value, in no table: what the caller gives; absent = boxOf's.
pub fn make_box_value(p: &crate::scene_boxes::MakeBoxP) -> crate::scene_boxes::Box {
    let mut b = box_value(p.c, p.h, 0.0, [0.5, 0.5, 0.5], [0.0, 0.0, 0.0], 0.0, 0.0, None);
    if let Some(x) = p.yaw { b.yaw = x; }
    if let Some(x) = p.albedo { b.albedo = x; }
    if let Some(x) = p.emissive { b.emissive = x; }
    if let Some(x) = p.flags { b.flags = x; }
    if let Some(x) = p.owner { b.owner = x; }
    if let Some(x) = p.cut_height { b.cut_height = x; }
    b.wet_k = p.wet_k;
    b.rot = p.rot;
    b.dc = p.dc;
    b.dh = p.dh;
    b.rough = p.rough;
    b.metal = p.metal;
    b.stuff = p.stuff;
    b.does = p.does;
    b.aniso = p.aniso;
    b.marks = p.marks.clone();
    b.mark_gain = p.mark_gain;
    b.fog_emit = p.fog_emit;
    b.fog_mark = p.fog_mark;
    b.mkey = p.mkey;
    b.cut = p.cut.clone();
    b.cut_core = p.cut_core;
    b.cut_glass = p.cut_glass;
    b.cut_round = p.cut_round;
    b.cut_arc = p.cut_arc;
    b.cut_old = p.cut_old;
    b.cut_core_stuff = p.cut_core_stuff;
    b.round = p.round;
    b.shape = p.shape.clone();
    b.shaped = p.shaped;
    b.piece = p.piece;
    b.of = p.of;
    b.dress = p.dress;
    b
}

/// A box from what the caller gives, in the table.
pub fn make_box(g: &mut crate::game::Game, p: &crate::scene_boxes::MakeBoxP) -> BoxId {
    let b = make_box_value(p);
    crate::scene_boxes::box_new(g, b)
}

/// A copy of a box to change.
pub fn copy_box(g: &mut crate::game::Game, b: BoxId) -> BoxId {
    let copy = g.tab.boxes[b].clone();
    crate::scene_boxes::box_new(g, copy)
}

/// A box's two words, to go on the box made from its paint.
pub fn word_of(b: crate::scene_boxes::WordOfB) -> crate::scene_boxes::WordOf {
    crate::scene_boxes::WordOf { stuff: b.stuff, does: b.does }
}

/// The world's buffers and tables.
pub fn box_world_new(g: &crate::game::Game, max_boxes: Option<f64>) -> crate::scene_boxes::BoxWorld {
    let wd = crate::scene_world::world();
    let max_boxes = max_boxes.unwrap_or(wd.max_boxes);
    let mut w = crate::scene_boxes::BoxWorld::blank();
    w.max_boxes = max_boxes;
    if max_boxes != wd.max_boxes { panic!("BoxWorld: maxBoxes must be WORLD.maxBoxes (the shaders place the marking table after that many mat rows)"); }
    if wd.grid_w + wd.grid_h > 96.0 { panic!("trace grid: the DDA in common.wgsl caps a ray at 96 cell visits; W + H must stay <= 96 for that cap to be unreachable"); }
    if max_boxes > 65536.0 || max_boxes * wd.grid_w * wd.grid_h >= 16777216.0 { panic!("trace grid: packed items keep the box index in 16 bits and cell starts in 24"); }
    w
}

pub fn box_world_add_static(g: &mut crate::game::Game, b: BoxId) -> f64 {
    let w = &mut g.engine.world;
    w.statics.push(b);
    (w.statics.len() - 1) as f64
}

/// The first n statics stand and the rest go, the one place the statics are cut short. The grid's numbers then name other boxes, and a static added later in the tick would pass the ray's guard, so
/// the grid goes with them: the rays ask one by one until the next tick opens.
pub fn box_world_keep_statics(g: &mut crate::game::Game, n: usize) {
    g.engine.world.statics.truncate(n);
    g.ray_grid = Default::default();
}

/// Static `i` goes and those after it close up, the one place one static is taken out. The grid's numbers then name other boxes, so the grid goes too: the rays ask one by one until the next tick opens.
fn box_world_drop_static(g: &mut crate::game::Game, i: usize) {
    g.engine.world.statics.remove(i);
    g.ray_grid = Default::default();
}

/// Half extents in world XZ of a yawed or rotated box.
fn box_world_footprint(g: &mut crate::game::Game, b: BoxId) -> [f64; 2] {
    footprint_of(&g.tab.boxes[b])
}

fn footprint_of(b: &Bx) -> [f64; 2] {
    if let Some(rot) = b.rot {
        let ax = v3::quat::rotate(rot, [1.0, 0.0, 0.0]);
        let ay = v3::quat::rotate(rot, [0.0, 1.0, 0.0]);
        let az = v3::quat::rotate(rot, [0.0, 0.0, 1.0]);
        return [m::abs(ax[0]) * b.h[0] + m::abs(ay[0]) * b.h[1] + m::abs(az[0]) * b.h[2], m::abs(ax[2]) * b.h[0] + m::abs(ay[2]) * b.h[1] + m::abs(az[2]) * b.h[2]];
    }
    let cs = m::abs(m::cos(b.yaw));
    let sn = m::abs(m::sin(b.yaw));
    [cs * b.h[0] + sn * b.h[2], sn * b.h[0] + cs * b.h[2]]
}

/// THE RAY GRID: the statics by the cells of the floor, which is all it is. It is made from them, so it is no part of the record: it stands in a slot of the game (`Game.ray_grid`),
/// brought up as a tick opens, and a game's copies share it. A game with none, or a grid of fewer statics than stand, asks the rest one by one and has the same answers.
#[derive(Debug, PartialEq)]
pub struct RayGrid { start: Vec<u32>, items: Vec<u32>, globals: Vec<u32>, n: usize, stood: u64 }
/// (made from the statics: in no bytes)
impl crate::bytes::ToBytes for RayGrid { fn w(&self, _: &mut crate::bytes::W) {} }
type Grid = Option<Arc<RayGrid>>;
/// the grid the rays are asked by, where one stands
pub fn ray_grid(g: &Game) -> Option<&Arc<RayGrid>> { g.ray_grid.get::<Grid>().and_then(Option::as_ref) }

/// Box `i` of the world, the number a ray's `index` is: the statics first, then the frame's boxes.
pub fn box_world_at(g: &crate::game::Game, i: usize) -> Option<AnyBoxId> {
    let w = &g.engine.world;
    let n = w.statics.len();
    if i < n { Some(AnyBoxId::World(w.statics[i])) } else { w.dynamics.get(i - n).copied() }
}

/// The ray grid brought up to the statics, where one of them stands another way. Returns the statics' count. (A static that has no row in the box world, the kit picture's lamp, tells nobody
/// when it is moved, and nobody moves it: a build with debug assertions looks.)
fn box_world_list(g: &mut crate::game::Game) -> f64 {
    let w = &g.engine.world;
    let (n_stat, stood) = (w.statics.len(), g.tab.boxes.world().stood);
    if (n_stat + w.dynamics.len()) as f64 > w.max_boxes { panic!("too many boxes: {}", n_stat + w.dynamics.len()); }
    match ray_grid(g).filter(|r| r.n == n_stat && r.stood == stood) {
        Some(r) => debug_assert!(**r == build_ray_grid(g), "a static stands another way and the box world was not told"),
        None => *g.ray_grid.at::<Grid>() = Some(Arc::new(build_ray_grid(g))),
    }
    n_stat as f64
}

/// words a ray's bits stand in: a bit a static
pub const MET: usize = 128;
/// the nearest a ray has met so far: how far, which box of the world (−1 none), the face
struct Nearest { best: f64, bi: f64, bn: V3 }

/// Closest box along the ray, of every box of the frame as it is drawn (the statics by their grid). A read: the game keeps nothing of a ray.
pub fn box_world_raycast(g: &crate::game::Game, ro: V3, rd: V3, tmax: Option<f64>, skip_flags: Option<f64>, skip_owner: Option<f64>, with_look: Option<bool>) -> Option<crate::scene_boxes::BoxWorldRaycast> {
    let tmax = tmax.unwrap_or(1e9);
    let skip_flags = skip_flags.unwrap_or_else(|| { let f = box_flag(); (js::to_i32(f.no_shadow) | js::to_i32(f.no_primary)) as f64 });
    let skip_owner = skip_owner.unwrap_or(0.0);
    let with_look = with_look.unwrap_or(false);
    let w = &g.engine.world;
    let mut near = Nearest { best: tmax, bi: -1.0, bn: [0.0, 1.0, 0.0] };
    let n_stat = w.statics.len();
    // (a grid of more statics than stand was made before some went: its numbers name other boxes now, so it is not asked)
    let grid = ray_grid(g).filter(|r| r.n <= n_stat);
    // (a look's box is passed over among the frame's boxes only: the grid holds every static, so a static asked one by one is asked whatever its look, and a game with no grid has the same answers)
    for i in grid.map_or(0, |r| r.n)..n_stat + w.dynamics.len() {
        if i < n_stat || with_look || !look_keyed(g, w.dynamics[i - n_stat]) { ray_test(g, &mut near, i, ro, rd, skip_flags, skip_owner); }
    }
    if let Some(r) = grid {
        for &gi in &r.globals { ray_test(g, &mut near, gi as usize, ro, rd, skip_flags, skip_owner); }
        ray_walk(g, r, &mut near, ro, rd, skip_flags, skip_owner);
    }
    if near.bi >= 0.0 { Some(crate::scene_boxes::BoxWorldRaycast { t: near.best, index: near.bi, n: near.bn }) } else { None }
}

/// One candidate of the raycast, with the tie rule the index order gave it.
fn ray_test(g: &Game, near: &mut Nearest, i: usize, ro: V3, rd: V3, skip_flags: f64, skip_owner: f64) {
    let id = box_world_at(g, i).expect("a ray asked a box the world has not got");
    let (flags, owner) = { let b = &g.tab[id]; (b.flags, b.owner) };
    if js::to_i32(flags) & js::to_i32(skip_flags) != 0 { return; }
    if js::truthy(skip_owner) && owner == skip_owner { return; }
    let Some(r) = ray_box(g, ro, rd, id, near.best) else { return };
    if r.t < near.best || (r.t == near.best && (i as f64) < near.bi) {
        // (a pane that hangs is asked whether it is open where the ray meets it)
        if g.tab.boxes.world().panes.is_some() && js::to_i32(flags) & js::to_i32(box_flag().glass) != 0 && i < g.engine.world.statics.len()
            && crate::glassbreak::glass_break_live(g) && crate::glassbreak::glass_break_open_at(g, i as f64, [ro[0] + rd[0] * r.t, ro[1] + rd[1] * r.t, ro[2] + rd[2] * r.t]) { return; }
        *near = Nearest { best: r.t, bi: i as f64, bn: r.n };
    }
}

/// The statics along the ray: a 2D DDA over the XZ cells, each cell's statics tested once.
fn ray_walk(g: &Game, r: &RayGrid, near: &mut Nearest, ro: V3, rd: V3, skip_flags: f64, skip_owner: f64) {
    let wd = crate::scene_world::world();
    let (w_, h_, cs) = (wd.grid_w, wd.grid_h, wd.grid_cell);
    let (gx, gz) = (w_ * cs, h_ * cs);
    let (ox, oz, dx, dz) = (ro[0], ro[2], rd[0], rd[2]);
    let (mut t0, mut t1) = (0.0, f64::INFINITY);
    if dx != 0.0 {
        let (ta, tb) = ((0.0 - ox) / dx, (gx - ox) / dx);
        if ta < tb { if ta > t0 { t0 = ta; } if tb < t1 { t1 = tb; } } else { if tb > t0 { t0 = tb; } if ta < t1 { t1 = ta; } }
    } else if ox < 0.0 || ox > gx { return; }
    if dz != 0.0 {
        let (ta, tb) = ((0.0 - oz) / dz, (gz - oz) / dz);
        if ta < tb { if ta > t0 { t0 = ta; } if tb < t1 { t1 = tb; } } else { if tb > t0 { t0 = tb; } if ta < t1 { t1 = ta; } }
    } else if oz < 0.0 || oz > gz { return; }
    if !(t0 <= t1) { return; }
    let mut cx = m::floor((ox + dx * t0) / cs);
    let mut cz = m::floor((oz + dz * t0) / cs);
    if cx < 0.0 { cx = 0.0; } else if cx > w_ - 1.0 { cx = w_ - 1.0; }
    if cz < 0.0 { cz = 0.0; } else if cz > h_ - 1.0 { cz = h_ - 1.0; }
    let step_x = if dx > 0.0 { 1.0 } else if dx < 0.0 { -1.0 } else { 0.0 };
    let step_z = if dz > 0.0 { 1.0 } else if dz < 0.0 { -1.0 } else { 0.0 };
    let dtx = if step_x != 0.0 { cs / m::abs(dx) } else { f64::INFINITY };
    let dtz = if step_z != 0.0 { cs / m::abs(dz) } else { f64::INFINITY };
    let mut tmx = if dx > 0.0 { ((cx + 1.0) * cs - ox) / dx } else if dx < 0.0 { (cx * cs - ox) / dx } else { f64::INFINITY };
    let mut tmz = if dz > 0.0 { ((cz + 1.0) * cs - oz) / dz } else if dz < 0.0 { (cz * cs - oz) / dz } else { f64::INFINITY };
    // (a static filed in several cells is tested once: a bit a static, which this ray alone has)
    let mut met = [0u64; MET];
    let mut t_enter = t0;
    loop {
        if t_enter > near.best { return; }
        let c = cz * w_ + cx;
        // (a NaN origin leaves the cell NaN: no candidate)
        if c == c {
            let c = c as usize;
            let (s, e) = (r.start[c] as usize, r.start[c + 1] as usize);
            for k in s..e {
                let i = r.items[k] as usize;
                if met[i >> 6] >> (i & 63) & 1 != 0 { continue; }
                met[i >> 6] |= 1 << (i & 63);
                ray_test(g, near, i, ro, rd, skip_flags, skip_owner);
            }
        }
        if tmx < tmz {
            t_enter = tmx;
            cx += step_x;
            if cx < 0.0 || cx >= w_ { return; }
            tmx += dtx;
        } else if tmz < f64::INFINITY {
            t_enter = tmz;
            cz += step_z;
            if cz < 0.0 || cz >= h_ { return; }
            tmz += dtz;
        } else {
            return;
        }
    }
}

/// The statics into a ray grid: padded footprints into cells, the oversize and outside ones as globals.
fn build_ray_grid(g: &Game) -> RayGrid {
    let wd = crate::scene_world::world();
    let (w, h, cs) = (wd.grid_w, wd.grid_h, wd.grid_cell);
    let n = g.engine.world.statics.len();
    assert!(n <= MET * 64, "more statics than a ray keeps a bit for");
    let mut globals = Vec::new();
    let mut list: Vec<(u32, [i32; 4])> = Vec::with_capacity(n);
    for (i, &b) in (0..).zip(&g.engine.world.statics) {
        let b = &g.tab.boxes[b];
        let [ex, ez] = footprint_of(b);
        let at = |v: f64| m::floor(v / cs);
        let (x0, x1) = (at(b.c[0] - ex - BOX_WORLD_RAY_PAD), at(b.c[0] + ex + BOX_WORLD_RAY_PAD));
        let (z0, z1) = (at(b.c[2] - ez - BOX_WORLD_RAY_PAD), at(b.c[2] + ez + BOX_WORLD_RAY_PAD));
        if !(x0 >= 0.0 && z0 >= 0.0 && x1 < w && z1 < h) || (x1 - x0 + 1.0) * (z1 - z0 + 1.0) > 96.0 { globals.push(i); } else { list.push((i, [x0, x1, z0, z1].map(|v| v as i32))); }
    }
    let (start, items) = game_types::boxworld::filed(w as usize, (w * h) as usize, &list);
    RayGrid { start, items, globals, n, stood: g.tab.boxes.world().stood }
}

/// Is this box's record a shape's and no break's?
pub fn shape_cut(g: &crate::game::Game, b: AnyBoxId) -> bool {
    let x = &g.tab[b];
    x.shaped == Some(true) && x.cut.is_some()
}

/// Is this box's cut record met by the game's rays? Ask it only of a box that has one.
pub(super) fn cut_met(g: &crate::game::Game, b: AnyBoxId) -> bool {
    g.tab[b].cut.as_ref().unwrap().len() > 0
}

/// One slab of the test: false = the ray has left the box
#[inline]
fn slab(h: f64, o: f64, inv: f64, ax: i32, t0: &mut f64, t1: &mut f64, axis: &mut i32, sgn: &mut f64) -> bool {
    let (mut ta, mut tb) = ((-h - o) * inv, (h - o) * inv);
    let mut s = -1.0;
    if ta > tb {
        core::mem::swap(&mut ta, &mut tb);
        s = 1.0;
    }
    if ta > *t0 {
        *t0 = ta;
        *axis = ax;
        *sgn = s;
    }
    if tb < *t1 { *t1 = tb; }
    !(*t0 > *t1)
}

/// `quat.rotate(quat.conj(rot), v)` over scalars
#[inline]
fn unturn(rot: Quat, v: V3) -> V3 {
    let (qx, qy, qz, qw) = (-rot[0], -rot[1], -rot[2], rot[3]);
    let (tx, ty, tz) = (2.0 * (qy * v[2] - qz * v[1]), 2.0 * (qz * v[0] - qx * v[2]), 2.0 * (qx * v[1] - qy * v[0]));
    [v[0] + qw * tx + (qy * tz - qz * ty), v[1] + qw * ty + (qz * tx - qx * tz), v[2] + qw * tz + (qx * ty - qy * tx)]
}

/// rayBox's own lines: a box with neither record nor rotation
fn ray_plain(ro: V3, rd: V3, b: &Bx, tmax: f64) -> Option<(f64, V3)> {
    let cs = m::cos(b.yaw);
    let sn = m::sin(b.yaw);
    let (px, py, pz) = (ro[0] - b.c[0], ro[1] - b.c[1], ro[2] - b.c[2]);
    let (lx, lz, ly) = (cs * px - sn * pz, sn * px + cs * pz, py);
    let (dx, dz, dy) = (cs * rd[0] - sn * rd[2], sn * rd[0] + cs * rd[2], rd[1]);
    let (ix, iy, iz) = (1.0 / js::or(dx, 1e-20), 1.0 / js::or(dy, 1e-20), 1.0 / js::or(dz, 1e-20));
    let h = b.h;
    let (mut t0, mut t1, mut axis, mut sgn) = (0.0, tmax, -1, 0.0);
    if !slab(h[0], lx, ix, 0, &mut t0, &mut t1, &mut axis, &mut sgn) { return None; }
    if !slab(h[1], ly, iy, 1, &mut t0, &mut t1, &mut axis, &mut sgn) { return None; }
    if !slab(h[2], lz, iz, 2, &mut t0, &mut t1, &mut axis, &mut sgn) { return None; }
    if axis < 0 { return None; }
    let (ln0, ln1, ln2) = (if axis == 0 { sgn } else { 0.0 }, if axis == 1 { sgn } else { 0.0 }, if axis == 2 { sgn } else { 0.0 });
    Some((t0, [cs * ln0 + sn * ln2, ln1, -sn * ln0 + cs * ln2]))
}

/// The slab test in the frame of a fully rotated box
fn ray_rot(ro: V3, rd: V3, b: &Bx, rot: Quat, tmax: f64) -> Option<(f64, V3)> {
    let o = unturn(rot, [ro[0] - b.c[0], ro[1] - b.c[1], ro[2] - b.c[2]]);
    let d = unturn(rot, [rd[0], rd[1], rd[2]]);
    let h = b.h;
    let (mut t0, mut t1, mut axis, mut sgn) = (0.0, tmax, -1, 0.0);
    if !slab(h[0], o[0], 1.0 / js::or(d[0], 1e-20), 0, &mut t0, &mut t1, &mut axis, &mut sgn) { return None; }
    if !slab(h[1], o[1], 1.0 / js::or(d[1], 1e-20), 1, &mut t0, &mut t1, &mut axis, &mut sgn) { return None; }
    if !slab(h[2], o[2], 1.0 / js::or(d[2], 1e-20), 2, &mut t0, &mut t1, &mut axis, &mut sgn) { return None; }
    if axis < 0 { return None; }
    let mut ln = [0.0, 0.0, 0.0];
    ln[axis as usize] = sgn;
    Some((t0, v3::quat::rotate(rot, ln)))
}

/// The cylinder about the box's own y, radius h[0], between its end faces
fn ray_round(ro: V3, rd: V3, b: &Bx, tmax: f64) -> Option<(f64, V3)> {
    let (mut cs, mut sn) = (1.0, 0.0);
    let (px, py, pz) = (ro[0] - b.c[0], ro[1] - b.c[1], ro[2] - b.c[2]);
    let (ox, oy, oz, dx, dy, dz);
    if let Some(rot) = b.rot {
        let o = unturn(rot, [px, py, pz]);
        let d = unturn(rot, [rd[0], rd[1], rd[2]]);
        (ox, oy, oz, dx, dy, dz) = (o[0], o[1], o[2], d[0], d[1], d[2]);
    } else {
        cs = m::cos(b.yaw);
        sn = m::sin(b.yaw);
        (ox, oy, oz, dx, dy, dz) = (cs * px - sn * pz, py, sn * px + cs * pz, cs * rd[0] - sn * rd[2], rd[1], sn * rd[0] + cs * rd[2]);
    }
    let (r, hy) = (b.h[0], b.h[1]);
    let (mut t0, mut t1, mut face, mut sgn) = (0.0, tmax, 0, 0.0);
    {
        let inv = 1.0 / js::or(dy, 1e-20);
        let (mut ta, mut tb) = ((-hy - oy) * inv, (hy - oy) * inv);
        let mut s = -1.0;
        if ta > tb {
            core::mem::swap(&mut ta, &mut tb);
            s = 1.0;
        }
        if ta > t0 {
            t0 = ta;
            face = 1;
            sgn = s;
        }
        if tb < t1 { t1 = tb; }
        if t0 > t1 { return None; }
    }
    let (a, hb, c) = (dx * dx + dz * dz, ox * dx + oz * dz, ox * ox + oz * oz - r * r);
    if a < 1e-18 {
        if c > 0.0 { return None; }
    } else {
        let disc = hb * hb - a * c;
        if disc < 0.0 { return None; }
        let s = m::sqrt(disc);
        let (ta, tb) = ((-hb - s) / a, (-hb + s) / a);
        if ta > t0 {
            t0 = ta;
            face = 2;
        }
        if tb < t1 { t1 = tb; }
        if t0 > t1 { return None; }
    }
    if face == 0 { return None; }
    let (mut l0, mut l1, mut l2) = (0.0, sgn, 0.0);
    if face == 2 {
        let (hx, hz) = (ox + dx * t0, oz + dz * t0);
        let il = 1.0 / js::or(m::sqrt(hx * hx + hz * hz), 1.0);
        l0 = hx * il;
        l1 = 0.0;
        l2 = hz * il;
    }
    if let Some(rot) = b.rot { return Some((t0, v3::quat::rotate(rot, [l0, l1, l2]))); }
    Some((t0, [cs * l0 + sn * l2, l1, -sn * l0 + cs * l2]))
}

/// A ray that keeps further from a box's middle than its corners stand (and a little over, for a turn that is not quite of length one) meets nothing
/// of it, however it is turned: most boxes a ray is asked of are so, and are asked for no sine.
#[inline]
fn passes_by(ro: V3, rd: V3, x: &Bx) -> bool {
    let p = v3::sub(x.c, ro);
    let (pp, dd, pd) = (v3::dot(p, p), v3::dot(rd, rd), v3::dot(p, rd));
    let rr = v3::dot(x.h, x.h) * 1.04 + 0.02;
    pp * dd - pd * pd > rr * dd || (pd < 0.0 && pp > rr)
}

/// The closest hit of a ray on one box; origin inside = no hit.
pub fn ray_box(g: &crate::game::Game, ro: V3, rd: V3, b: AnyBoxId, tmax: f64) -> Option<crate::scene_boxes::RayBox> {
    let (round, has_cut) = { let x = &g.tab[b]; if passes_by(ro, rd, x) { return None; } (x.round == Some(true), x.cut.is_some()) };
    if round { return ray_box_round(g, ro, rd, b, tmax).map(|r| crate::scene_boxes::RayBox { t: r.t, n: r.n }); }
    if has_cut && cut_met(g, b) { return ray_box_cut(g, ro, rd, b, tmax).map(|r| crate::scene_boxes::RayBox { t: r.t, n: r.n }); }
    let x = &g.tab[b];
    let hit = match x.rot {
        Some(rot) => ray_rot(ro, rd, x, rot, tmax),
        None => ray_plain(ro, rd, x, tmax),
    };
    hit.map(|(t, n)| crate::scene_boxes::RayBox { t, n })
}

/// The test against a box with a cut record: the slab interval, cut down by the record's planes.
fn ray_box_cut(g: &crate::game::Game, ro: V3, rd: V3, b: AnyBoxId, tmax: f64) -> Option<crate::scene_boxes::RayBoxCut> {
    let (c, h, yaw, rot) = { let x = &g.tab[b]; (x.c, x.h, x.yaw, x.rot) };
    let (mut cs, mut sn) = (1.0, 0.0);
    let (px, py, pz) = (ro[0] - c[0], ro[1] - c[1], ro[2] - c[2]);
    let (ox, oy, oz, dx, dy, dz);
    if let Some(r) = rot {
        let o = unturn(r, [px, py, pz]);
        let d = unturn(r, [rd[0], rd[1], rd[2]]);
        (ox, oy, oz, dx, dy, dz) = (o[0], o[1], o[2], d[0], d[1], d[2]);
    } else {
        cs = m::cos(yaw);
        sn = m::sin(yaw);
        (ox, oy, oz, dx, dy, dz) = (cs * px - sn * pz, py, sn * px + cs * pz, cs * rd[0] - sn * rd[2], rd[1], sn * rd[0] + cs * rd[2]);
    }
    let (mut t0, mut t1, mut axis, mut sgn) = (0.0, tmax, -1, 0.0);
    if !slab(h[0], ox, 1.0 / js::or(dx, 1e-20), 0, &mut t0, &mut t1, &mut axis, &mut sgn) { return None; }
    if !slab(h[1], oy, 1.0 / js::or(dy, 1e-20), 1, &mut t0, &mut t1, &mut axis, &mut sgn) { return None; }
    if !slab(h[2], oz, 1.0 / js::or(dz, 1e-20), 2, &mut t0, &mut t1, &mut axis, &mut sgn) { return None; }
    let cut = g.tab[b].cut.as_ref().unwrap();
    let (clip_t0, _, k) = crate::scene_cut::cut_clip_of(cut, ox, oy, oz, dx, dy, dz, t0, t1)?;
    let plane = if k >= 0.0 { Some(cut[k as usize].n) } else { None };
    if k < 0.0 && axis < 0 { return None; }
    let ln: V3 = match plane {
        Some(n) => [n[0], n[1], n[2]],
        None => { let mut l = [0.0, 0.0, 0.0]; l[axis as usize] = sgn; l }
    };
    if let Some(r) = rot { return Some(crate::scene_boxes::RayBoxCut { t: clip_t0, n: v3::quat::rotate(r, ln) }); }
    Some(crate::scene_boxes::RayBoxCut { t: clip_t0, n: [cs * ln[0] + sn * ln[2], ln[1], -sn * ln[0] + cs * ln[2]] })
}

fn ray_box_round(g: &crate::game::Game, ro: V3, rd: V3, b: AnyBoxId, tmax: f64) -> Option<crate::scene_boxes::RayBoxRound> {
    ray_round(ro, rd, &g.tab[b], tmax).map(|(t, n)| crate::scene_boxes::RayBoxRound { t, n })
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Lcg(u32);
    impl Lcg {
        fn r(&mut self) -> f64 {
            self.0 = self.0.wrapping_mul(1664525).wrapping_add(1013904223);
            self.0 as f64 / 4294967296.0
        }
    }
    fn fnv(h: u32, w: u32) -> u32 { (h ^ w).wrapping_mul(16777619) }
    fn fnv_f(h: u32, x: f64) -> u32 {
        let b = x.to_bits();
        fnv(fnv(h, b as u32), (b >> 32) as u32)
    }

    /// the world the pinned bits are of, draw for draw
    fn world(g: &mut Game, x: &mut Lcg) {
        g.engine.world = crate::scene_boxes::BoxWorld::blank();
        g.engine.world.max_boxes = 4096.0;
        for i in 0..220 {
            let c = [x.r() * 40.0, x.r() * 3.0, x.r() * 28.0];
            let mut h = [0.1 + x.r() * 3.0, 0.1 + x.r() * 1.5, 0.1 + x.r() * 3.0];
            let kind = m::floor(x.r() * 5.0) as i32;
            let (a, b_, c_, d) = (x.r() - 0.5, x.r() - 0.5, x.r() - 0.5, x.r() - 0.5);
            let yaw_draw = x.r() * 6.28;
            let (f0, f1, f2, f3) = (x.r(), x.r(), x.r(), x.r());
            let mut b = box_value(c, h, 0.0, [0.5, 0.5, 0.5], [0.0, 0.0, 0.0], 0.0, 0.0, None);
            if kind == 1 { b.yaw = yaw_draw; }
            if kind == 2 {
                let l = m::sqrt(a * a + b_ * b_ + c_ * c_ + d * d);
                b.rot = Some([a / l, b_ / l, c_ / l, d / l]);
            }
            if kind == 3 { h[2] = h[0]; b.h = h; b.round = Some(true); b.yaw = yaw_draw; }
            if kind == 4 { b.h = [60.0, 0.1, 60.0]; }
            let mut flags = 0;
            if f0 < 0.1 { flags |= 2; }
            if f1 < 0.1 { flags |= 1; }
            if f2 < 0.1 { flags |= 131072; }
            b.flags = flags as f64;
            let dynamic = i >= 200;
            if dynamic {
                b.owner = 1.0 + m::floor(f3 * 3.0);
                if f3 < 0.2 { b.mkey = Some(crate::scene_boxes::mkey().look + 5.0); }
                if f3 > 0.9 { b.dress = Some(true); }
            }
            let id = crate::scene_boxes::box_new(g, b);
            if dynamic { g.engine.world.dynamics.push(id.into()); } else { box_world_add_static(g, id); }
        }
        let list = g.engine.world.statics.clone();
        g.tab.boxes.statics(&list);
    }

    fn rays(g: &Game, x: &mut Lcg) -> u32 {
        let mut h = 2166136261u32;
        for j in 0..4000 {
            let ro = [x.r() * 50.0 - 5.0, x.r() * 3.0, x.r() * 38.0 - 5.0];
            let mut rd = [x.r() * 2.0 - 1.0, x.r() * 2.0 - 1.0, x.r() * 2.0 - 1.0];
            if j % 7 == 0 { rd[1] = 0.0; }
            if j % 11 == 0 { rd[0] = 0.0; }
            if j % 13 == 0 { rd = [0.0, -1.0, 0.0]; }
            let tmax = if j % 5 == 0 { Some(10.0) } else { None };
            let skip = if j % 3 == 0 { Some(2.0) } else { None };
            let owner = if j % 4 == 0 { Some(1.0) } else { None };
            let look = if j % 6 == 0 { Some(true) } else { None };
            match box_world_raycast(g, ro, rd, tmax, skip, owner, look) {
                None => h = fnv(h, 0xdead),
                Some(r) => { h = fnv_f(h, r.index); h = fnv_f(h, r.t); h = fnv_f(h, r.n[0]); h = fnv_f(h, r.n[1]); h = fnv_f(h, r.n[2]); }
            }
        }
        h
    }

    /// raycast through the grid, the brute force and the tests of every shape, against pinned bits
    #[test]
    fn raycast_matches_the_typescripts_bits() {
        let mut g = Game::blank();
        let mut x = Lcg(12345);
        world(&mut g, &mut x);
        assert_eq!(box_world_list(&mut g), 200.0);
        assert!(box_world_at(&g, 219).is_some() && box_world_at(&g, 220).is_none());
        let mut seen = 2166136261u32;
        { let r = ray_grid(&g).unwrap(); seen = fnv_f(seen, r.globals.len() as f64); seen = fnv_f(seen, r.start[r.start.len() - 1] as f64); }
        for i in 0..200 { let id = g.engine.world.statics[i]; let f = box_world_footprint(&mut g, id); seen = fnv_f(seen, f[0]); seen = fnv_f(seen, f[1]); }
        assert_eq!(seen, TS_GRID, "grid and footprints");
        let hits = rays(&mut g, &mut x);
        assert_eq!(hits, TS_RAYS, "the rays through the grid");
        // the same rays by the brute force: the grid changes no answer
        let mut x2 = Lcg(777);
        g.ray_grid = Default::default();
        let brute = rays(&mut g, &mut x2);
        box_world_list(&mut g);
        let mut x3 = Lcg(777);
        assert_eq!(rays(&mut g, &mut x3), brute, "grid against brute force");
        // the statics cut short and one added elsewhere in the same tick: no grid stands to name other boxes, and the rays are those of a grid made afresh
        box_world_keep_statics(&mut g, 150);
        assert!(ray_grid(&g).is_none());
        let far = g.engine.world.statics[3];
        let mut added = g.tab.boxes[far].clone();
        added.c[0] += 3.0;
        let added = crate::scene_boxes::box_new(&mut g, added);
        box_world_add_static(&mut g, added);
        let mut x4 = Lcg(777);
        let brute = rays(&mut g, &mut x4);
        box_world_list(&mut g);
        let mut x5 = Lcg(777);
        assert_eq!(rays(&mut g, &mut x5), brute, "statics cut short: brute force against a grid made afresh");
        // one static taken out from the middle: the same
        box_world_drop_static(&mut g, 40);
        assert!(ray_grid(&g).is_none());
        let mut x6 = Lcg(777);
        let brute = rays(&mut g, &mut x6);
        box_world_list(&mut g);
        let mut x7 = Lcg(777);
        assert_eq!(rays(&mut g, &mut x7), brute, "a static taken out: brute force against a grid made afresh");
    }

    /// the ray as it is said: the statics of the cells round each half metre of it, in order, each once; the nearest entered and of two as near the first come on
    fn ray_by_the_half_metre(g: &Game, list: &[BoxId], ro: V3, rd: V3, tmax: f64) -> (f64, i32) {
        let w = g.tab.boxes.world();
        let (mut met, mut asked, steps) = ((tmax, -1), vec![false; list.len()], m::ceil(tmax / 0.5) + 1.0);
        for i in 0..=steps as usize {
            let t = (i as f64 / steps) * tmax;
            if t > met.0 + 1.5 { break; }
            let (x, z) = (m::floor((ro[0] + rd[0] * t) / w.cs), m::floor((ro[2] + rd[2] * t) / w.cs));
            for (cx, cz) in [-1.0, 0.0, 1.0].into_iter().flat_map(|dz| [-1.0, 0.0, 1.0].map(|dx| (x + dx, z + dz))) {
                if !(cx >= 0.0 && cz >= 0.0 && cx < w.w as f64 && cz < w.h as f64) { continue; }
                let c = cz as usize * w.w as usize + cx as usize;
                for &b in &w.items[w.start[c] as usize..w.start[c + 1] as usize] {
                    if core::mem::replace(&mut asked[b as usize], true) || js::to_i32(g.tab.boxes[list[b as usize]].flags) & js::to_i32(box_flag().no_shadow) != 0 { continue; }
                    if let Some(r) = ray_box(g, ro, rd, list[b as usize].into(), met.0) { if r.t < met.0 { met = (r.t, b as i32); } }
                }
            }
        }
        met
    }

    /// THE RAY THE RULES ASK (sim lines.rs) passes over cells and boxes it need not ask: its answer is the nearest, and of two as near the same one, however it went about it. Among them boxes with
    /// a face in one plane, tilted ones, some that stand over the level's edge, and rays from outside it.
    #[test]
    fn a_ray_among_the_rows_meets_what_the_half_metres_meet() {
        let mut g = Game::blank();
        let mut x = Lcg(4242);
        world(&mut g, &mut x);
        let mut list = g.engine.world.statics.clone();
        for i in 0..80 {
            let mut b = g.tab.boxes[list[i]].clone();
            let up = i % 3;
            b.c[up] += b.h[up] * (x.r() - 0.5);
            list.push(crate::scene_boxes::box_new(&mut g, b));
        }
        g.col = crate::scene_collision::new(&mut g, &list);
        let (mut met, mut tied) = (0, 0);
        for j in 0..8000 {
            let ro = if j % 8 == 0 { [x.r() * 50.0 - 5.0, x.r() * 3.0, x.r() * 38.0 - 5.0] } else { [x.r() * 40.0, x.r() * 3.0, x.r() * 28.0] };
            let mut rd = [x.r() * 2.0 - 1.0, x.r() * 2.0 - 1.0, x.r() * 2.0 - 1.0];
            if j % 7 == 0 { rd[1] = 0.0; }
            if j % 11 == 0 { rd[0] = 0.0; }
            if j % 13 == 0 { rd = [0.0, if j % 2 == 0 { 1.0 } else { -1.0 }, 0.0]; }
            if j % 17 == 0 { rd = [1e-200, -1.0, 0.0]; }
            let len = m::hyp3(rd[0], rd[1], rd[2]);
            for v in &mut rd { *v /= len; }
            let tmax = if j % 5 == 0 { 3.0 } else { 60.0 };
            let mut out = [0.0; 4];
            let got = sim::lines::ray(g.tab.boxes.world(), ro[0], ro[1], ro[2], rd[0], rd[1], rd[2], tmax, false, false, &mut out, &g.k);
            let want = ray_by_the_half_metre(&g, &list, ro, rd, tmax);
            assert_eq!(got.max(-1), want.1, "ray {j} from {ro:?} along {rd:?} for {tmax}");
            if got < 0 { continue; }
            assert_eq!(out[0].to_bits(), want.0.to_bits(), "ray {j} from {ro:?} along {rd:?} for {tmax}: {got}");
            met += 1;
            for (i, &b) in list.iter().enumerate() {
                let Some(r) = ray_box(&g, ro, rd, b.into(), tmax) else { continue };
                tied += (i as i32 != got && r.t == want.0) as u32;
                // (a tilted one is met where the walk comes by the cells it is filed in, and what stands past the edge is in no cell)
                let plain = g.tab.boxes[b].rot.is_none() && js::to_i32(g.tab.boxes[b].flags) & js::to_i32(box_flag().no_shadow) == 0;
                let at = [ro[0] + rd[0] * r.t, ro[2] + rd[2] * r.t];
                assert!(!plain || r.t >= want.0 || !(at[0] >= 0.0 && at[0] < 40.0 && at[1] >= 0.0 && at[1] < 28.0), "ray {j} from {ro:?} along {rd:?}: {i} is nearer than {got}");
            }
        }
        assert!(met > 3000 && tied > 100, "{met} rays met anything, {tied} met two at one place");
    }

    #[test]
    fn a_moved_static_rebuilds_the_grid_a_repaint_does_not_and_copies_share_it() {
        let mut g = Game::blank();
        let mut x = Lcg(99);
        world(&mut g, &mut x);
        box_world_list(&mut g);
        let was = ray_grid(&g).unwrap().clone();
        box_world_list(&mut g);
        let mut copy = g.clone();
        let id = g.engine.world.statics[3];
        (g.tab.edit(id).albedo, g.tab.edit(id).flags) = ([1.0, 0.0, 0.0], 1.0);
        box_world_list(&mut g);
        assert!([&g, &copy].iter().all(|g| Arc::ptr_eq(ray_grid(g).unwrap(), &was)), "colour and flags leave it standing, and it is the copy's too");
        g.tab.edit(id).c[0] += 1.0;
        box_world_list(&mut g);
        box_world_list(&mut copy);
        assert!(!Arc::ptr_eq(ray_grid(&g).unwrap(), &was), "a move rebuilds it");
        assert!(Arc::ptr_eq(ray_grid(&copy).unwrap(), &was), "and not the copy's");
    }

    #[test]
    fn trim_is_drawn_while_its_box_is() {
        let mut g = Game::blank();
        let f = box_flag();
        let plain = || box_value([0.0; 3], [1.0; 3], 0.0, [0.5; 3], [0.0; 3], 0.0, 0.0, None);
        let wall = crate::scene_boxes::box_new(&mut g, plain());
        let mut piece = plain();
        piece.piece = Some(true);
        piece.of = Some(0.0);
        let trim = crate::scene_boxes::box_new(&mut g, piece);
        let all = [wall, trim];
        assert!(trim_drawn(&mut g, &all, wall), "no `of`: its own");
        assert!(trim_drawn(&mut g, &all, trim) && box_drawn(&mut g, &all, trim));
        g.tab.edit(wall).flags = f.no_primary;
        assert!(!trim_drawn(&mut g, &all, trim) && !box_drawn(&mut g, &all, trim), "the wall hidden: its trim with it");
        assert!(!box_drawn(&mut g, &all, wall));
        g.tab.edit(wall).flags = f.glass;
        assert!(!box_drawn(&mut g, &all, wall), "a pane is not drawn");
        assert!(box_drawn(&mut g, &all, trim), "trim asks only whether its box is hidden");
    }

    const TS_GRID: u32 = 0x92b81bdb;
    const TS_RAYS: u32 = 0x72cbf515;
}
