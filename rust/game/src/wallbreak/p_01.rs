//! src/game/wallbreak.ts lines 1 to 478
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

// ------------------------------------------------------------------ private helpers
fn flag() -> crate::scene_boxes::BoxFlag { crate::scene_boxes::box_flag() }
/// a static by its index: a box's index is its row
fn at(i: f64) -> BoxId { BoxId(i as u32) }
fn hide_box(g: &mut Game, b: BoxId) { let x = &mut g.col.boxes[b]; x.flags = (js::to_i32(x.flags) | js::to_i32(HIDE)) as f64; }
fn is_hard(b: &Breakable) -> bool { b.hard == Some(true) }
fn cell_ix(b: &Breakable, ix: f64, iy: f64, iz: f64) -> usize { ((iy * b.nz + iz) * b.nx + ix) as usize }
fn aabb_from(c: V3, h: V3, yaw: f64) -> AABB {
    let cs = m::abs(m::cos(yaw)); let sn = m::abs(m::sin(yaw));
    let ex = cs * h[0] + sn * h[2]; let ez = sn * h[0] + cs * h[2];
    AABB { x0: c[0] - ex, x1: c[0] + ex, y0: c[1] - h[1], y1: c[1] + h[1], z0: c[2] - ez, z1: c[2] + ez }
}
fn zone_of(r: [f64; 4]) -> AABB { AABB { x0: r[0], z0: r[1], x1: r[2], z1: r[3], y0: 0.0, y1: 0.0 } }
/// the row of `list` a static index belongs to: `owner` says whose it is, the list holds the truth (TRAP: the TypeScript's two lists share one object)
fn row_of(g: &Game, index: f64) -> Option<usize> { let id = g.destr.walls.owner.get(crate::Num(index))?.id; row_of_id(g, id) }
fn row_of_id(g: &Game, id: f64) -> Option<usize> { g.destr.walls.list.iter().position(|x| x.id == id) }
/// writes B to every place the TypeScript shares it from: its row of `list`, and `owner` under its own index and each of its pieces'
fn put_row(g: &mut Game, b: &Breakable) {
    let w = &mut *g.destr.walls;
    if let Some(r) = w.list.iter().position(|x| x.id == b.id) { w.list[r] = b.clone(); }
    w.owner.set(crate::Num(b.id), b.clone());
    for &s in &b.slots { w.owner.set(crate::Num(s), b.clone()); }
}
fn grid_of(b: &Breakable) -> crate::break_table::CellGrid {
    crate::break_table::CellGrid { id: b.id, surf: b.surf, nx: b.nx, ny: b.ny, nz: b.nz, yb: b.yb.clone(), open: b.open.clone(), n_open: Some(b.n_open), box_: b.box_ }
}
fn word(g: &Game, b: BoxId) -> crate::scene_boxes::WordOf {
    let x = &g.col.boxes[b];
    crate::scene_boxes::word_of(crate::scene_boxes::WordOfB { stuff: x.stuff, does: x.does })
}
/// (materials.ts doesOf of a box)
fn does_of_box(g: &mut Game, b: BoxId) -> crate::scene_materials::Does {
    let does = g.col.boxes[b].does.map(|w| String::from(w.as_str()));
    crate::scene_materials::does_of(g, Some(&crate::scene_materials::DoesOfB { does }))
}
/// the pieces standing: every one is a slot of what it stands in for. (wallbreak.ts:199,254 `col.boxes.length - authored`, which holds other boxes in Rust: notes.md, Decisions)
fn pieces_up(g: &Game) -> f64 { g.destr.walls.list.iter().map(|x| x.slots.len()).sum::<usize>() as f64 }
fn budget_of(chunks: f64) -> crate::break_table::ShedBudget { crate::break_table::ShedBudget { chunks, heap: None, from: None, per: None } }
fn note_cause(c: WallBreakOpenCellsCause) -> WallBreakNoteCCause {
    match c {
        WallBreakOpenCellsCause::Blast => WallBreakNoteCCause::Blast,
        WallBreakOpenCellsCause::Fire => WallBreakNoteCCause::Fire,
        WallBreakOpenCellsCause::Load => WallBreakNoteCCause::Load,
        WallBreakOpenCellsCause::Reset => WallBreakNoteCCause::Reset,
        WallBreakOpenCellsCause::Collapse => WallBreakNoteCCause::Collapse,
    }
}
fn row_cause(c: WallBreakNoteCCause) -> LevelChangeCause {
    match c {
        WallBreakNoteCCause::Blast => LevelChangeCause::Blast,
        WallBreakNoteCCause::Fire => LevelChangeCause::Fire,
        WallBreakNoteCCause::Load => LevelChangeCause::Load,
        WallBreakNoteCCause::Reset => LevelChangeCause::Reset,
        WallBreakNoteCCause::Collapse => LevelChangeCause::Collapse,
    }
}

// ------------------------------------------------------------------ the switch and the plain boxes
/// Is the engine on: tune.wallBreak not put off, outside the showcases. DESTRUCTION.on is settled on. (wallbreak.ts wallBreakOn)
pub fn wall_break_on(g: &mut crate::game::Game) -> bool { g.tune.wall_break && g.destr.show_chaos.is_none() }

/// A box's world bounds. (wallbreak.ts aabbOf)
pub fn aabb_of(g: &mut crate::game::Game, b: BoxId) -> crate::wallbreak::AABB { let x = &g.col.boxes[b]; aabb_from(x.c, x.h, x.yaw) }

/// Do two bounds overlap on the plan by more than `eps`. (wallbreak.ts overlapXZ)
pub fn overlap_xz(a: crate::wallbreak::AABB, b: crate::wallbreak::AABB, eps: Option<f64>) -> bool {
    let eps = eps.unwrap_or(0.01);
    m::min(a.x1, b.x1) - m::max(a.x0, b.x0) > eps && m::min(a.z1, b.z1) - m::max(a.z0, b.z0) > eps
}

// ------------------------------------------------------------------ standing up
/// The engine as a level stands: `g.destr` must already hold the Destruction with its items indexed, `index` reads them. (WallBreak.constructor)
pub fn wall_break_new(g: &mut crate::game::Game) -> crate::wallbreak::WallBreak {
    let mut w = WallBreak::blank();
    w.authored = g.col.boxes.len() as f64;
    // index() files into g.destr.walls, so the new one stands there for the call, and whatever stood there goes back
    let old = core::mem::replace(&mut *g.destr.walls, w);
    wall_break_index(g);
    let col = crate::levelfit::StoodCol { boxes: g.col.boxes.ids().collect(), dynamic_boxes: g.col.dynamic_boxes.clone() };
    let tables = crate::scene_box_mirror::box_mirror_tables(g);
    g.destr.walls.room = crate::levelfit::break_room(g, &col, &tables);
    core::mem::replace(&mut *g.destr.walls, old)
}

/// Would `need` more pieces of B be more than this level may stand up. (WallBreak.over)
pub fn wall_break_over(g: &mut crate::game::Game, b: &crate::wallbreak::Breakable, need: f64) -> bool {
    let n = pieces_up(g) + need;
    let r = g.destr.walls.room;
    if n > g.k.walls.max_pieces || n > r.boxes || n > r.cuts { return true; }
    if r.items == f64::INFINITY { return false; }
    let mut filed = need * crate::scene_collision::foot_entries(g, b.box_);
    for i in 0..g.destr.walls.list.len() {
        let (slots, bx) = (g.destr.walls.list[i].slots.len(), g.destr.walls.list[i].box_);
        if slots != 0 { filed += slots as f64 * crate::scene_collision::foot_entries(g, bx); }
    }
    filed > r.items
}

/// How many pieces the room surely holds, whichever things are broken. (WallBreak.piecesSure)
pub fn wall_break_pieces_sure(g: &mut crate::game::Game) -> f64 {
    let r = g.destr.walls.room;
    if r.items == f64::INFINITY { return f64::INFINITY; }
    let mut foot = 1.0;
    for i in 0..g.destr.walls.list.len() {
        let bx = g.destr.walls.list[i].box_;
        foot = m::max(foot, crate::scene_collision::foot_entries(g, bx));
    }
    let authored = g.destr.walls.authored as usize;
    for i in 0..authored {
        if wall_break_adoptable(g, i as f64) {
            let bx = BoxId(i as u32);
            foot = m::max(foot, crate::scene_collision::foot_entries(g, bx));
        }
    }
    m::max(0.0, m::min(m::min(r.boxes, r.cuts), m::floor(r.items / foot)))
}

/// May a man stand under a roof at this plan point (true with no interior map at all). (index's `roofed`)
fn roofed(interior: Option<&[u8]>, min: [f64; 3], w: f64, h: f64, x: f64, z: f64) -> bool {
    let Some(interior) = interior else { return true };
    let cx = m::floor(x - min[0]); let cz = m::floor(z - min[2]);
    cx >= 0.0 && cz >= 0.0 && cx < w && cz < h && interior[(cz * w + cx) as usize] == 1
}
/// a length's whole divisions into cells (index's `div`)
fn div(len: f64, col: f64) -> f64 { m::max(1.0, m::round(len / col)) }

/// Finds what can break, by shape and by the box's word, as the level stands. (WallBreak.index)
pub fn wall_break_index(g: &mut crate::game::Game) {
    for t in g.targets.clone() {
        for bi in g.tab.sim_lightshome_fixture[t].boxes.clone() { g.destr.walls.fixture_boxes.add(crate::Num(bi)); }
    }
    for k in 0..g.level.lights.len() {
        if let Some(fb) = g.level.lights[k].fixture_box { if fb >= 0.0 { g.destr.walls.fixture_boxes.add(crate::Num(fb)); } }
    }
    let world = crate::scene_world::world();
    let (w_, h_) = (world.grid_w, world.grid_h);
    let f = flag();
    let (shadow, primary, glass, dynamic) = (js::to_i32(f.no_shadow), js::to_i32(f.no_primary), js::to_i32(f.glass), js::to_i32(f.dynamic));
    let any = shadow | primary | glass | dynamic;
    for i in 0..g.col.boxes.len() {
        let b = BoxId(i as u32); let fi = i as f64;
        let (c, h, yaw, flags, em, rot, piece, round) = { let x = &g.col.boxes[b]; (x.c, x.h, x.yaw, js::to_i32(x.flags), x.emissive, x.rot.is_some(), x.piece == Some(true), x.round == Some(true)) };
        if (flags & any) == shadow && !piece && c[1] - h[1] > 0.06 && c[0] > world.min[0] && c[0] < world.max[0] && c[2] > world.min[2] && c[2] < world.max[2] { g.destr.walls.ghosts.push(fi); }
        if (flags & any) != 0 { continue; }
        if rot || em[0] + em[1] + em[2] > 0.0 || g.destr.walls.fixture_boxes.has(crate::Num(fi)) || crate::destruction::destruction_item_of(g, fi).is_some() { continue; }
        if c[0] < world.min[0] || c[0] > world.max[0] || c[2] < world.min[2] || c[2] > world.max[2] { continue; }
        let does = does_of_box(g, b);
        if does.wall.is_none() && does.cover.is_none() { continue; }
        let y0 = c[1] - h[1]; let y1 = c[1] + h[1]; let sx = h[0] * 2.0; let sz = h[2] * 2.0; let thin = m::min(sx, sz); let long = m::max(sx, sz);
        let part = does.wall == Some(crate::scene_materials::DoesWall::Screen);
        let mut kind: Option<BKind> = None; let mut surf = crate::scene_materials::Surf::Drywall;
        if y0 <= 0.1 && y1 >= g.k.walls.wall_min_top && does.wall.is_some() {
            if thin > g.k.walls.wall_max_thick || long < g.k.walls.wall_min_len || m::abs(m::sin(yaw)) > 1e-6 || crate::scene_boxes::shape_cut(g, b) { continue; }
            let along_x = sx >= sz; let mut in2 = 0; let mut n = 0;
            for u in [-0.6, 0.0, 0.6] {
                let px = c[0] + (if along_x { u * h[0] } else { 0.0 }); let pz = c[2] + (if along_x { 0.0 } else { u * h[2] });
                n += 1;
                let interior = g.reach_interior.as_deref();
                if roofed(interior, world.min, w_, h_, px + (if along_x { 0.0 } else { 0.6 }), pz + (if along_x { 0.6 } else { 0.0 }))
                    && roofed(interior, world.min, w_, h_, px - (if along_x { 0.0 } else { 0.6 }), pz - (if along_x { 0.6 } else { 0.0 })) { in2 += 1; }
            }
            if in2 * 2 <= n { continue; }
            kind = Some(BKind::Wall);
        } else if y0 <= 0.25 && y1 >= g.k.walls.cover_min_top && long >= g.k.walls.cover_min_len && thin >= 0.03 {
            if let Some(cover) = does.cover {
                surf = match cover { crate::scene_materials::DoesCover::Wood => crate::scene_materials::Surf::Wood, crate::scene_materials::DoesCover::Cardboard => crate::scene_materials::Surf::Cardboard };
            } else if part && thin <= 0.13 { surf = crate::scene_materials::Surf::Fabric; }
            else if does.wall == Some(crate::scene_materials::DoesWall::Board) && y1 < g.k.walls.wall_min_top { surf = crate::scene_materials::Surf::Drywall; }
            else { continue; }
            kind = Some(BKind::Cover);
        }
        let Some(kind) = kind else { continue };
        let (mut nx, mut ny, mut nz) = (1.0, 1.0, 1.0); let mut yb: Vec<f64> = vec![-h[1], h[1]];
        let col = g.k.walls.col;
        if kind == BKind::Wall {
            if sx >= sz { nx = m::min(32.0, div(sx, col)); } else { nz = m::min(32.0, div(sz, col)); }
            ny = 2.0;
            yb.insert(1, -h[1] + (g.k.walls.sill - y0));
        } else if !round && !crate::scene_boxes::shape_cut(g, b) {
            nx = div(sx, col); nz = div(sz, col);
            while nx * nz > g.k.walls.cover_max_cells { if nx >= nz { nx -= 1.0; } else { nz -= 1.0; } }
        }
        let n = (nx * ny * nz) as usize;
        let row = Breakable { id: fi, kind, surf, box_: b, flags0: flags as f64, nx, ny, nz, yb, open: vec![0u8; n], dmg: vec![0f32; n], slots: Vec::new(), live: 0.0, n_open: 0.0, hard: None };
        g.destr.walls.list.push(row.clone());
        g.destr.walls.owner.set(crate::Num(fi), row);
    }
}

/// Counts for the panel and the probes. (WallBreak.inventory)
pub fn wall_break_inventory(g: &mut crate::game::Game) -> crate::wallbreak::WallBreakInventory {
    let (mut walls, mut cover, mut wall_m) = (0.0, 0.0, 0.0);
    for i in 0..g.destr.walls.list.len() {
        if g.destr.walls.list[i].kind == BKind::Wall {
            walls += 1.0;
            let h = g.col.boxes[g.destr.walls.list[i].box_].h;
            wall_m += m::max(h[0], h[2]) * 2.0;
        } else { cover += 1.0; }
    }
    let pieces = pieces_up(g);
    let w = &*g.destr.walls;
    WallBreakInventory { walls, cover, wall_m, pieces, hidden: w.hidden.len() as f64, touched: w.list.iter().filter(|b| b.n_open > 0.0).count() as f64 }
}

/// The breakable a static index belongs to, an authored box or one of its pieces: what a round met. (WallBreak.breakableAt)
pub fn wall_break_breakable_at(g: &mut crate::game::Game, index: f64) -> Option<crate::wallbreak::WallBreakBreakableAt> {
    let b = &g.destr.walls.list[row_of(g, index)?];
    Some(WallBreakBreakableAt { id: b.id, kind: b.kind, surf: b.surf, open: b.n_open, cells: b.open.len() as f64, hard: if is_hard(b) { Some(true) } else { None } })
}

/// Could authored static `index` be taken on as a hard wall: a breaching charge's wall, by shape. (WallBreak.adoptable)
pub fn wall_break_adoptable(g: &mut crate::game::Game, index: f64) -> bool {
    if index < 0.0 || index >= g.col.boxes.len() as f64 || index >= g.destr.walls.authored || g.destr.walls.owner.has(crate::Num(index)) || g.destr.walls.hidden.has(crate::Num(index)) { return false; }
    let b = at(index);
    let (c, h, yaw, flags, em, rot) = { let x = &g.col.boxes[b]; (x.c, x.h, x.yaw, js::to_i32(x.flags), x.emissive, x.rot.is_some()) };
    let f = flag();
    if (flags & (js::to_i32(f.glass) | js::to_i32(f.no_primary) | js::to_i32(f.no_shadow) | js::to_i32(f.dynamic))) != 0 { return false; }
    if rot || m::abs(m::sin(yaw)) > 1e-6 || em[0] + em[1] + em[2] > 0.0 || g.destr.walls.fixture_boxes.has(crate::Num(index)) || crate::destruction::destruction_item_of(g, index).is_some() || crate::scene_boxes::shape_cut(g, b) { return false; }
    let world = crate::scene_world::world();
    if c[0] < world.min[0] || c[0] > world.max[0] || c[2] < world.min[2] || c[2] > world.max[2] { return false; }
    c[1] - h[1] <= 0.1 && c[1] + h[1] >= g.k.walls.wall_min_top && m::max(h[0], h[2]) * 2.0 >= g.k.walls.wall_min_len
}

/// The columns a hard wall is cut into along its length. (WallBreak.hardCols)
pub fn wall_break_hard_cols(g: &mut crate::game::Game, b: BoxId) -> f64 {
    let h = g.col.boxes[b].h;
    m::min(32.0, m::max(1.0, m::floor(m::max(h[0], h[2]) * 2.0 / g.k.walls.col + 1e-6)))
}

/// Takes static `index` on as a hard wall, on the frame its charge goes off. True = it is one, now or since an earlier charge. (WallBreak.adopt)
pub fn wall_break_adopt(g: &mut crate::game::Game, index: f64) -> bool {
    if let Some(r) = row_of(g, index) {
        let had = &g.destr.walls.list[r];
        return had.id == index && had.kind == BKind::Wall && is_hard(had);
    }
    if !wall_break_adoptable(g, index) { return false; }
    let b = at(index);
    let (c, h, flags0) = { let x = &g.col.boxes[b]; (x.c, x.h, x.flags) };
    let sx = h[0] * 2.0; let sz = h[2] * 2.0; let cols = wall_break_hard_cols(g, b);
    let nx = if sx >= sz { cols } else { 1.0 }; let nz = if sx >= sz { 1.0 } else { cols }; let n = (nx * 2.0 * nz) as usize;
    let surf = does_of_box(g, b).surf;
    let row = Breakable { id: index, kind: BKind::Wall, surf, box_: b, flags0, nx, ny: 2.0, nz, yb: vec![-h[1], -h[1] + (g.k.walls.sill - (c[1] - h[1])), h[1]], open: vec![0u8; n], dmg: vec![0f32; n], slots: Vec::new(), live: 0.0, n_open: 0.0, hard: Some(true) };
    g.destr.walls.list.push(row.clone());
    g.destr.walls.owner.set(crate::Num(index), row);
    true
}

/// workLeft's `cell`: the cell a round would count on (the sill under a standing upper counts on the upper), and what it still wants; None = gone.
fn work_cell(g: &Game, b: &Breakable, ix: f64, iy: f64, iz: f64) -> (usize, Option<f64>) {
    let mut iy = iy;
    if b.kind == BKind::Wall && iy == 0.0 && b.open[cell_ix(b, ix, 1.0, iz)] == 0 { iy = 1.0; }
    let c = cell_ix(b, ix, iy, iz);
    if b.open[c] != 0 { return (c, None); }
    let tough = g.k.walls.tough.get(String::from(b.surf.as_str())).copied().unwrap_or(8.0);
    (c, Some(m::max(0.0, tough * (if b.kind == BKind::Wall && iy == 0.0 { g.k.walls.sill_k } else { 1.0 }) - b.dmg[c] as f64)))
}

/// What the cell a round stopping in `index` would count on still wants, in round()'s weighted units; reads only. (WallBreak.workLeft)
pub fn wall_break_work_left(g: &mut crate::game::Game, index: f64, hit_p: V3, dir: V3, through: Option<f64>) -> Option<f64> {
    let through = through.unwrap_or(0.0);
    let r = if index < 1e6 { row_of(g, index) } else { None }?;
    let b = g.destr.walls.list[r].clone();
    if is_hard(&b) { return None; }
    let inside = at(index);
    let [ix, iy, iz] = wall_break_cell_at(g, &b, v3::mad(hit_p, dir, 0.02), Some(inside));
    let (c0, first) = work_cell(g, &b, ix, iy, iz);
    let first = first?;
    if through <= 0.0 { return Some(first); }
    let mut sum = first; let mut seen: Vec<usize> = vec![c0]; let h = g.col.boxes[b.box_].h;
    let mut s = 0.1;
    while s < through {
        let p = v3::mad(hit_p, dir, s); let l = wall_break_to_local(g, &b, p);
        if m::abs(l[0]) > h[0] || m::abs(l[1]) > h[1] || m::abs(l[2]) > h[2] { break; }
        let [ix, iy, iz] = wall_break_cell_at(g, &b, p, None);
        let (c, left) = work_cell(g, &b, ix, iy, iz);
        if !seen.contains(&c) { seen.push(c); if let Some(left) = left { sum += left; } }
        s += 0.1;
    }
    Some(sum)
}

/// Every breakable: the panel, the probes, the AI part. (WallBreak.all)
pub fn wall_break_all(g: &mut crate::game::Game) -> Vec<crate::wallbreak::WallBreakAllItem> {
    g.destr.walls.list.iter().map(|b| WallBreakAllItem { id: b.id, kind: b.kind, surf: b.surf, box_: b.box_, open: b.n_open, cells: b.open.len() as f64 }).collect()
}

// ------------------------------------------------------------------ geometry
/// The module's own deterministic die. (WallBreak.hash)
pub fn wall_break_hash(g: &mut crate::game::Game) -> f64 {
    let c = g.destr.walls.counter; g.destr.walls.counter = c + 1.0;
    crate::debris::hash01(c * 2654435761.0)
}

/// (WallBreak.toLocal)
pub fn wall_break_to_local(g: &mut crate::game::Game, b: &crate::wallbreak::Breakable, p: V3) -> V3 {
    let (c, yaw) = { let x = &g.col.boxes[b.box_]; (x.c, x.yaw) };
    let cs = m::cos(yaw); let sn = m::sin(yaw); let px = p[0] - c[0]; let pz = p[2] - c[2];
    [cs * px - sn * pz, p[1] - c[1], sn * px + cs * pz]
}

/// (WallBreak.toWorld)
pub fn wall_break_to_world(g: &mut crate::game::Game, b: &crate::wallbreak::Breakable, l: V3) -> V3 {
    let (c, yaw) = { let x = &g.col.boxes[b.box_]; (x.c, x.yaw) };
    let cs = m::cos(yaw); let sn = m::sin(yaw);
    [c[0] + cs * l[0] + sn * l[2], c[1] + l[1], c[2] - sn * l[0] + cs * l[2]]
}

/// A cell's index. (WallBreak.ci)
pub fn wall_break_ci(g: &mut crate::game::Game, b: &crate::wallbreak::Breakable, ix: f64, iy: f64, iz: f64) -> f64 { (iy * b.nz + iz) * b.nx + ix }

/// (WallBreak.xb)
pub fn wall_break_xb(g: &mut crate::game::Game, b: &crate::wallbreak::Breakable, ix: f64) -> f64 { let h = g.col.boxes[b.box_].h; -h[0] + 2.0 * h[0] * ix / b.nx }

/// (WallBreak.zb)
pub fn wall_break_zb(g: &mut crate::game::Game, b: &crate::wallbreak::Breakable, iz: f64) -> f64 { let h = g.col.boxes[b.box_].h; -h[2] + 2.0 * h[2] * iz / b.nz }

/// The cell holding world point p, clamped into the static `inside` when given. (WallBreak.cellAt)
pub fn wall_break_cell_at(g: &mut crate::game::Game, b: &crate::wallbreak::Breakable, p: V3, inside: Option<BoxId>) -> [f64; 3] {
    let mut l = wall_break_to_local(g, b, p);
    let bh = g.col.boxes[b.box_].h;
    let cl = |v: f64, n: f64| m::max(0.0, m::min(n - 1.0, v));
    if let Some(inside) = inside {
        if inside != b.box_ {
            let (ic, ih) = { let x = &g.col.boxes[inside]; (x.c, x.h) }; let c = wall_break_to_local(g, b, ic);
            for k in 0..3 {
                let mm = m::min(0.01, ih[k] * 0.5);
                l[k] = m::max(c[k] - ih[k] + mm, m::min(c[k] + ih[k] - mm, l[k]));
            }
        }
    }
    let ix = cl(m::floor((l[0] + bh[0]) / (2.0 * bh[0]) * b.nx), b.nx); let iz = cl(m::floor((l[2] + bh[2]) / (2.0 * bh[2]) * b.nz), b.nz);
    let mut iy = 0.0;
    for k in 1..b.ny as usize { if l[1] >= b.yb[k] { iy = k as f64; } }
    [ix, iy, iz]
}

/// World centre of a cell. (WallBreak.cellCentre)
pub fn wall_break_cell_centre(g: &mut crate::game::Game, b: &crate::wallbreak::Breakable, ix: f64, iy: f64, iz: f64) -> V3 {
    let l = [
        (wall_break_xb(g, b, ix) + wall_break_xb(g, b, ix + 1.0)) / 2.0,
        (b.yb[iy as usize] + b.yb[iy as usize + 1]) / 2.0,
        (wall_break_zb(g, b, iz) + wall_break_zb(g, b, iz + 1.0)) / 2.0,
    ];
    wall_break_to_world(g, b, l)
}

/// A cell's centre, half-extents and yaw: all that `aabbOf(cellBox(…))` and `cellBox(…).c` read, with no box made for it.
fn cell_geom(g: &mut Game, b: &Breakable, ix: f64, iy: f64, iz: f64) -> (V3, V3, f64) {
    let hx = (wall_break_xb(g, b, ix + 1.0) - wall_break_xb(g, b, ix)) / 2.0;
    let hy = (b.yb[iy as usize + 1] - b.yb[iy as usize]) / 2.0;
    let hz = (wall_break_zb(g, b, iz + 1.0) - wall_break_zb(g, b, iz)) / 2.0;
    let c = wall_break_cell_centre(g, b, ix, iy, iz);
    (c, [hx, hy, hz], g.col.boxes[b.box_].yaw)
}

/// The cell as a world box, for what it sheds. (WallBreak.cellBox)
pub fn wall_break_cell_box(g: &mut crate::game::Game, b: &crate::wallbreak::Breakable, ix: f64, iy: f64, iz: f64) -> BoxId {
    let (c, h, yaw) = cell_geom(g, b, ix, iy, iz);
    let (albedo, w) = (g.col.boxes[b.box_].albedo, word(g, b.box_));
    let p = crate::scene_boxes::MakeBoxP { c, h, yaw: Some(yaw), albedo: Some(v3::copy(albedo)), stuff: w.stuff, does: w.does, ..Default::default() };
    crate::scene_boxes::make_box(g, &p)
}

/// World centre and half-extents of one rect of `pieces`: what `rebuild` stands up and what `wouldOpen` forecasts. (WallBreak.rectBox)
pub fn wall_break_rect_box(g: &mut crate::game::Game, b: &crate::wallbreak::Breakable, arg1: [f64; 6]) -> crate::wallbreak::WallBreakRectBox {
    let [x0, x1, y0, y1, z0, z1] = arg1;
    let lx0 = wall_break_xb(g, b, x0); let lx1 = wall_break_xb(g, b, x1 + 1.0);
    let ly0 = b.yb[y0 as usize]; let ly1 = b.yb[y1 as usize + 1];
    let lz0 = wall_break_zb(g, b, z0); let lz1 = wall_break_zb(g, b, z1 + 1.0);
    let c = wall_break_to_world(g, b, [(lx0 + lx1) / 2.0, (ly0 + ly1) / 2.0, (lz0 + lz1) / 2.0]);
    WallBreakRectBox { c, h: [(lx1 - lx0) / 2.0, (ly1 - ly0) / 2.0, (lz1 - lz0) / 2.0] }
}

/// `pieces` needs no game: greedy boxes over the closed cells of `open`, y first.
fn pieces_of(nx: usize, ny: usize, nz: usize, open: &[u8]) -> Vec<[f64; 6]> {
    let ci = |ix: usize, iy: usize, iz: usize| (iy * nz + iz) * nx + ix;
    let mut used = vec![0u8; nx * ny * nz]; let mut out: Vec<[f64; 6]> = Vec::new();
    for iz in 0..nz { for ix in 0..nx { for iy in 0..ny {
        let free = |used: &[u8], ix: usize, iy: usize, iz: usize| { let c = ci(ix, iy, iz); open[c] == 0 && used[c] == 0 };
        if !free(&used, ix, iy, iz) { continue; }
        let mut y1 = iy; while y1 + 1 < ny && free(&used, ix, y1 + 1, iz) { y1 += 1; }
        let mut x1 = ix;
        'grow: while x1 + 1 < nx {
            for y in iy..=y1 { if !free(&used, x1 + 1, y, iz) { break 'grow; } }
            if iy > 0 && free(&used, x1 + 1, iy - 1, iz) { break; }
            if y1 + 1 < ny && free(&used, x1 + 1, y1 + 1, iz) { break; }
            x1 += 1;
        }
        let mut z1 = iz;
        'growz: while z1 + 1 < nz {
            for x in ix..=x1 { for y in iy..=y1 { if !free(&used, x, y, z1 + 1) { break 'growz; } } }
            for x in ix..=x1 {
                if iy > 0 && free(&used, x, iy - 1, z1 + 1) { break 'growz; }
                if y1 + 1 < ny && free(&used, x, y1 + 1, z1 + 1) { break 'growz; }
            }
            z1 += 1;
        }
        for z in iz..=z1 { for x in ix..=x1 { for y in iy..=y1 { used[ci(x, y, z)] = 1; } } }
        out.push([ix as f64, x1 as f64, iy as f64, y1 as f64, iz as f64, z1 as f64]);
    } } }
    out
}

/// Greedy boxes over the closed cells: an untouched wall column stays one floor-to-top piece. (WallBreak.pieces)
pub fn wall_break_pieces(g: &mut crate::game::Game, b: &crate::wallbreak::Breakable, open: Option<&[u8]>) -> Vec<[f64; 6]> {
    pieces_of(b.nx as usize, b.ny as usize, b.nz as usize, open.unwrap_or(&b.open))
}

/// Rebuilds B's standing pieces from its cells, and writes B back to its row. False = over the cap or the room, and nothing changed. (WallBreak.rebuild)
pub fn wall_break_rebuild(g: &mut crate::game::Game, b: &mut crate::wallbreak::Breakable) -> bool {
    let bx = b.box_;
    if b.n_open == 0.0 {
        g.col.boxes[bx].flags = b.flags0;
        for k in 0..b.live as usize { hide_box(g, at(b.slots[k])); }
        b.live = 0.0;
        wall_break_bump(g);
        put_row(g, b);
        return true;
    }
    let rects = if b.n_open == b.open.len() as f64 { Vec::new() } else { wall_break_pieces(g, b, None) };
    let studs = crate::break_table::breaks_studs(g, &grid_of(b));
    let need = rects.len() as f64 + studs.len() as f64 - b.slots.len() as f64;
    if need > 0.0 && (wall_break_over(g, b, need) || g.engine.world.statics.len() as f64 + need > g.engine.world.max_boxes - 768.0) { return false; }
    for k in 0..rects.len() {
        let WallBreakRectBox { c, h } = wall_break_rect_box(g, b, rects[k]);
        let cut = crate::break_table::breaks_ragged(g, &grid_of(b), &rects[k][..], h);
        let (albedo, rough, metal, aniso, stuff, does) = { let x = &g.col.boxes[bx]; (x.albedo, x.rough, x.metal, x.aniso, x.stuff, x.does) };
        if k < b.slots.len() {
            let p = at(b.slots[k]);
            { let x = &mut g.col.boxes[p]; x.c = c; x.h = h; x.flags = b.flags0; }
            if cut.is_some() || g.col.boxes[p].cut.is_some() {
                let core = if cut.is_some() { crate::break_table::breaks_end_core(g, b.surf.as_str(), albedo) } else { None };
                let x = &mut g.col.boxes[p]; x.cut = cut; x.cut_core = core;
            }
            let x = &mut g.col.boxes[p];
            if x.name.as_deref() == Some("stud") {
                x.name = Some(String::from("piece"));
                x.albedo = v3::copy(albedo); x.rough = rough; x.metal = metal; x.stuff = stuff; x.does = does;
            }
        } else {
            let (yaw, w) = (g.col.boxes[bx].yaw, word(g, bx));
            let mp = crate::scene_boxes::MakeBoxP { c, h, yaw: Some(yaw), albedo: Some(v3::copy(albedo)), flags: Some(b.flags0), name: Some(String::from("piece")), rough, metal, aniso, stuff: w.stuff, does: w.does, ..Default::default() };
            let p = crate::scene_boxes::make_box(g, &mp);
            if cut.is_some() {
                let core = crate::break_table::breaks_end_core(g, b.surf.as_str(), albedo);
                let x = &mut g.col.boxes[p]; x.cut = cut; x.cut_core = core;
            }
            let i = crate::scene_collision::add_static(g, p, Some(bx));
            crate::scene_boxes::box_world_add_static(g, p);
            b.slots.push(i);
        }
    }
    for j in 0..studs.len() {
        // each stud is one more stand-in after the pieces: a slot is a piece one rebuild and a stud the next, and its paint goes with its job
        let k = rects.len() + j;
        let c = wall_break_to_world(g, b, studs[j].c); let h = studs[j].h;
        let p;
        if k < b.slots.len() {
            p = at(b.slots[k]);
            let x = &mut g.col.boxes[p];
            x.c = c; x.h = h; x.flags = b.flags0;
            if x.cut.is_some() { x.cut = None; x.cut_core = None; }
        } else {
            let yaw = g.col.boxes[bx].yaw;
            let mp = crate::scene_boxes::MakeBoxP { c, h, yaw: Some(yaw), flags: Some(b.flags0), ..Default::default() };
            p = crate::scene_boxes::make_box(g, &mp);
            let i = crate::scene_collision::add_static(g, p, Some(bx));
            crate::scene_boxes::box_world_add_static(g, p);
            b.slots.push(i);
        }
        let x = &mut g.col.boxes[p];
        x.name = Some(String::from("stud"));
        x.flags = (js::to_i32(x.flags) | js::to_i32(flag().thin)) as f64;
        x.albedo = v3::copy(studs[j].albedo); x.rough = Some(studs[j].rough); x.metal = Some(studs[j].metal); x.does = Some(studs[j].does);
    }
    for k in rects.len() + studs.len()..b.slots.len() { hide_box(g, at(b.slots[k])); }
    b.live = (rects.len() + studs.len()) as f64;
    g.col.boxes[bx].flags = (js::to_i32(b.flags0) | js::to_i32(HIDE)) as f64;
    wall_break_bump(g);
    put_row(g, b);
    true
}

/// The picture's static tables are stale, and the boxes were written in place: say so. (WallBreak.bump)
pub fn wall_break_bump(g: &mut crate::game::Game) { g.engine.world.geom_serial += 1.0; crate::scene_collision::wrote(g); }

// ------------------------------------------------------------------ opening cells
/// Opens `cells` of B, and writes B back to its row; the row of the feed, or None when nothing opened or the piece cap refused it. (WallBreak.openCells)
pub fn wall_break_open_cells(g: &mut crate::game::Game, b: &mut crate::wallbreak::Breakable, cells: &[f64], cause: crate::wallbreak::WallBreakOpenCellsCause, who: f64, dir: Option<V3>, quiet: Option<bool>, budget: Option<&mut crate::break_table::ShedBudget>) -> Option<crate::wallbreak::LevelChange> {
    let quiet = quiet.unwrap_or(false);
    let mut own = budget_of(g.k.walls.chunks_per_cell);
    let budget: &mut crate::break_table::ShedBudget = match budget { Some(x) => x, None => &mut own };
    let mut fresh: Vec<f64> = Vec::new();
    for &c in cells { if b.open[c as usize] == 0 && !fresh.contains(&c) { fresh.push(c); } }
    if fresh.is_empty() { return None; }
    for &c in &fresh { b.open[c as usize] = 1; }
    b.n_open += fresh.len() as f64;
    if !wall_break_rebuild(g, b) {
        for &c in &fresh { b.open[c as usize] = 0; }
        b.n_open -= fresh.len() as f64;
        g.stats.wallbreak.refused += 1;
        return None;
    }
    g.destr.walls.dirty = true;
    // what opened: bounds, kind
    let (mut x0, mut z0, mut x1, mut z1) = (1e9, 1e9, -1e9, -1e9); let mut pos: V3 = [0.0, 0.0, 0.0]; let mut sill = false;
    for &c in &fresh {
        let ix = js::rem(c, b.nx); let iz = js::rem(m::floor(c / b.nx), b.nz); let iy = m::floor(c / (b.nx * b.nz));
        let (cc, ch, yaw) = cell_geom(g, b, ix, iy, iz); let a = aabb_from(cc, ch, yaw);
        x0 = m::min(x0, a.x0); z0 = m::min(z0, a.z0); x1 = m::max(x1, a.x1); z1 = m::max(z1, a.z1);
        pos[0] += cc[0]; pos[1] += cc[1]; pos[2] += cc[2];
        if iy == 0.0 { sill = true; }
        if !quiet { let cb = wall_break_cell_box(g, b, ix, iy, iz); wall_break_shed(g, b, cb, dir, who, budget); }
    }
    let n_fresh = fresh.len() as f64;
    pos[0] /= n_fresh; pos[1] /= n_fresh; pos[2] /= n_fresh;
    let dead = b.n_open == b.open.len() as f64;
    let kind = if b.kind == BKind::Cover { LevelChangeKind::Cover } else if sill { LevelChangeKind::Hole } else { LevelChangeKind::Port };
    if b.kind == BKind::Cover {
        g.stats.wallbreak.cover_cells += fresh.len() as i32;
        if dead { g.stats.wallbreak.cover_dead += 1; }
    } else if sill { g.stats.wallbreak.holes += 1; } else { g.stats.wallbreak.ports += 1; }
    // what stood on it, what hung from it
    let mut rects: Vec<[f64; 4]> = vec![[x0, z0, x1, z1]];
    wall_break_settle(g, b, AABB { x0, x1, z0, z1, y0: 0.0, y1: 0.0 }, dir, who, quiet, &mut rects);
    // what fell may have opened B again: B's row holds the truth
    if let Some(r) = row_of_id(g, b.id) { *b = g.destr.walls.list[r].clone(); }
    // one model: the routes sound took, the smoke's obstacles, the nav cells, the lights' reach
    crate::scene_collision::sound_paths_invalidate(g);
    crate::smoke_types::smoke_system_like_obstacles_changed(g, pos, m::hyp2(x1 - x0, z1 - z0) / 2.0 + 1.5);
    let mut opened: Vec<f64> = Vec::new();
    for r in &rects {
        let ch = crate::scene_collision::nav_grid_rebake(g, r[0], r[1], r[2], r[3]);
        let mut k = 0;
        while k < ch.len() { if ch[k + 1] == 0.0 { opened.push(ch[k]); } k += 2; }
    }
    if b.kind == BKind::Wall { crate::game::game_reach_changed(g, Some([x0, z0, x1, z1])); }
    if is_hard(b) { for &c in &opened { g.destr.walls.shell_gaps.add(crate::Num(c)); } }
    g.stats.wallbreak.nav_opened += opened.len() as i32;
    if !quiet && (cause == WallBreakOpenCellsCause::Fire || cause == WallBreakOpenCellsCause::Collapse) { crate::floormess::mess_opened(g, [x0, z0, x1, z1]); }
    let walkable = !opened.is_empty(); let n_opened = opened.len();
    let c = WallBreakNoteC { kind, cause: note_cause(cause), id: b.id, name: None, pos, rect: [x0, z0, x1, z1], opened, walkable, sight: true, dead, who };
    let row = wall_break_note(g, &c);
    let line = format!("{:.2} {} {} #{} cells {} ({}/{}) nav+{}{}", g.time, kind.as_str(), cause.as_str(), b.id, fresh.len(), b.n_open, b.open.len(), n_opened, if dead { " DEAD" } else { "" });
    g.m.wallbreak.wall_log.push(line);
    if g.m.wallbreak.wall_log.len() > 200 { g.m.wallbreak.wall_log.drain(0..100); }
    Some(row)
}

/// The flags authored static `i` was loaded with, if this module has changed them. (WallBreak.flags0)
pub fn wall_break_flags0(g: &mut crate::game::Game, i: f64) -> Option<f64> {
    if let Some(f) = g.destr.walls.hidden.get(crate::Num(i)) { return Some(*f); }
    let b = &g.destr.walls.list[row_of(g, i)?];
    if b.id == i { Some(b.flags0) } else { None }
}

/// Furniture under a blast: the cover piece `id` goes whole. (WallBreak.raze)
pub fn wall_break_raze(g: &mut crate::game::Game, id_2: f64, who: f64, dir: Option<V3>, budget: Option<&mut crate::wallbreak::WallBreakRazeBudget>) -> Option<crate::wallbreak::LevelChange> {
    let Some(budget) = budget else { return wall_break_raze_in(g, id_2, None, who, dir, None) };
    // one count, in two types: what this raze throws comes off the caller's
    let mut lent = WallBreakRazeInBudget { chunks: budget.chunks };
    let row = wall_break_raze_in(g, id_2, None, who, dir, Some(&mut lent));
    budget.chunks = lent.chunks;
    row
}

/// What stands in the mouth of a blown hole: only the cells of it over the plan rectangle `r`. (WallBreak.razeIn)
pub fn wall_break_raze_in(g: &mut crate::game::Game, id_2: f64, r: Option<[f64; 4]>, who: f64, dir: Option<V3>, budget: Option<&mut crate::wallbreak::WallBreakRazeInBudget>) -> Option<crate::wallbreak::LevelChange> {
    let row = row_of(g, id_2)?;
    let mut b = g.destr.walls.list[row].clone();
    if b.kind != BKind::Cover { return None; }
    let mut cells: Vec<f64> = Vec::new();
    let zone = r.map(zone_of);
    for i in 0..b.open.len() {
        if b.open[i] != 0 { continue; }
        let c = i as f64;
        if let Some(zone) = zone {
            let ix = js::rem(c, b.nx); let iz = js::rem(m::floor(c / b.nx), b.nz); let iy = m::floor(c / (b.nx * b.nz));
            let (cc, ch, yaw) = cell_geom(g, &b, ix, iy, iz);
            if !overlap_xz(aabb_from(cc, ch, yaw), zone, None) { continue; }
        }
        cells.push(c);
    }
    if cells.is_empty() { return None; }
    let Some(budget) = budget else { return wall_break_open_cells(g, &mut b, &cells, WallBreakOpenCellsCause::Blast, who, dir, Some(false), None) };
    // one count, in two types: what this throws comes off the caller's
    let mut lent = budget_of(budget.chunks);
    let row = wall_break_open_cells(g, &mut b, &cells, WallBreakOpenCellsCause::Blast, who, dir, Some(false), Some(&mut lent));
    budget.chunks = lent.chunks;
    row
}

/// The stretch of the wall `id` that stands open from the floor up inside the plan rectangle `r`: where a body can go through. (WallBreak.sillOpenIn)
pub fn wall_break_sill_open_in(g: &mut crate::game::Game, id_2: f64, r: [f64; 4]) -> Option<[f64; 4]> {
    let row = row_of(g, id_2)?;
    let b = g.destr.walls.list[row].clone();
    if b.kind != BKind::Wall { return None; }
    let zone = zone_of(r); let mut out: Option<[f64; 4]> = None;
    let (nx, ny, nz) = (b.nx as usize, b.ny as usize, b.nz as usize);
    for iz in 0..nz { for ix in 0..nx {
        let mut open = true;
        let mut iy = 0;
        while iy < ny && open { open = b.open[cell_ix(&b, ix as f64, iy as f64, iz as f64)] == 1; iy += 1; }
        if !open { continue; }
        let (cc, ch, yaw) = cell_geom(g, &b, ix as f64, 0.0, iz as f64); let a = aabb_from(cc, ch, yaw);
        if !overlap_xz(a, zone, None) { continue; }
        out = Some(match out {
            Some(o) => [m::min(o[0], a.x0), m::min(o[1], a.z0), m::max(o[2], a.x1), m::max(o[3], a.z1)],
            None => [a.x0, a.z0, a.x1, a.z1],
        });
    } }
    out
}

/// What each subscriber does with a row. WallReact: wallreact.ts:167, WALLREACT.on settled on.
fn tell(g: &mut Game, who: ChangeListener, row: &LevelChange) {
    match who {
        ChangeListener::WallReact => {
            if row.kind == LevelChangeKind::Reset { g.m.wallreact.wall_react.stale = true; }
            g.m.wallreact.wall_react.queue.push(row.clone());
        }
    }
}

/// Files a row in the feed: serial and time stamped here, the subscriber told. (WallBreak.note)
pub fn wall_break_note(g: &mut crate::game::Game, c_2: &crate::wallbreak::WallBreakNoteC) -> crate::wallbreak::LevelChange {
    g.destr.walls.serial += 1.0;
    let row = LevelChange { serial: g.destr.walls.serial, t: g.time, kind: c_2.kind, cause: row_cause(c_2.cause), id: c_2.id, name: c_2.name.clone(), pos: c_2.pos, rect: c_2.rect, opened: c_2.opened.clone(), walkable: c_2.walkable, sight: c_2.sight, dead: c_2.dead, who: c_2.who };
    let max = g.k.walls.max_changes;
    let w = &mut *g.destr.walls;
    w.changes.push(row.clone());
    if w.changes.len() as f64 > max { let cut = w.changes.len() - max as usize; w.changes.drain(0..cut); }
    let mut i = 0;
    while i < g.destr.walls.listeners.len() { let who = g.destr.walls.listeners[i].clone(); tell(g, who, &row); i += 1; }
    row
}

/// Calls back on every change, on the frame it happens. Nothing ever unsubscribes (wallreact.ts:167 keeps the function only to test that it has one). (WallBreak.onChange)
pub fn wall_break_on_change(g: &mut crate::game::Game, fn_: &crate::wallbreak::ChangeListener) { g.destr.walls.listeners.push(fn_.clone()); }

/// The rows after `serial`; None = the ring has already dropped some of them: re-read the level. (WallBreak.since)
pub fn wall_break_since(g: &mut crate::game::Game, serial: f64) -> Option<Vec<crate::wallbreak::LevelChange>> {
    let w = &*g.destr.walls;
    if serial >= w.serial { return Some(Vec::new()); }
    let first = if !w.changes.is_empty() { w.changes[0].serial } else { w.serial + 1.0 };
    if serial + 1.0 < first { return None; }
    Some(w.changes.iter().filter(|c| c.serial > serial).cloned().collect())
}

/// What a broken cell throws, by its material's row: CUT.on is settled on. (WallBreak.shed)
pub fn wall_break_shed(g: &mut crate::game::Game, b: &crate::wallbreak::Breakable, cell: BoxId, dir: Option<V3>, who: f64, budget: &mut crate::break_table::ShedBudget) {
    let as_ = if is_hard(b) { Some(crate::break_table::Mat::Blockwork) } else { None };
    let r = crate::break_table::breaks_shed_cell(g, b.surf, cell, dir, who, budget, as_);
    g.stats.wallbreak.chunks += r.thrown as i32;
    g.stats.wallbreak.chunks_demoted += r.refused as i32;
}

#[cfg(test)]
mod tests {
    extern crate std;
    use super::*;

    fn game_with(boxes: &[(V3, V3, f64)], b: &Breakable) -> alloc::boxed::Box<Game> {
        let mut g = alloc::boxed::Box::new(Game::blank());
        for &(c, h, yaw) in boxes { g.col.boxes.push(crate::scene_boxes::Box { c, h, yaw, ..Default::default() }); }
        g.destr.walls.list.push(b.clone());
        for i in 0..boxes.len() { g.destr.walls.owner.set(crate::Num(i as f64), b.clone()); }
        g
    }
    fn wall(open: &[usize], dmg: &[(usize, f32)]) -> Breakable {
        let mut b = Breakable { id: 0.0, kind: BKind::Wall, surf: crate::scene_materials::Surf::Drywall, box_: BoxId(0), nx: 8.0, ny: 2.0, nz: 1.0, yb: vec![-1.4, -0.7, 1.4], open: vec![0; 16], dmg: vec![0.0; 16], slots: vec![1.0], live: 1.0, n_open: open.len() as f64, ..Default::default() };
        for &c in open { b.open[c] = 1; }
        for &(c, d) in dmg { b.dmg[c] = d; }
        b
    }
    fn bits(v: &[f64]) -> Vec<u64> { v.iter().map(|x| x.to_bits()).collect() }

    /// expected: the methods of WallBreak itself, on these boxes, under bun with EXACT.on
    #[test]
    fn the_cells_geometry_is_the_typescripts() {
        std::thread::Builder::new().stack_size(1 << 28).spawn(|| {
            const C: V3 = [3.25, 1.4, 7.75]; const H: V3 = [2.1, 1.4, 0.06];
            let b = wall(&[3, 11], &[(12, 2.5), (4, 1.25)]);
            let mut g = game_with(&[(C, H, 0.3), ([3.0, 1.0, 7.7], [0.25, 0.35, 0.06], 0.3)], &b);
            let g = &mut *g;
            assert_eq!(bits(&wall_break_to_local(g, &b, [4.1, 0.9, 8.2])), [0x3fe5bacb19112b2b, 0xbfdffffffffffffe, 0x3fe5cb84cbeaed16]);
            assert_eq!(bits(&wall_break_to_world(g, &b, [0.7, -0.2, 0.03])), [0x400f6bba26a754f6, 0x3ff3333333333333, 0x401e4984e0a52087]);
            assert_eq!(wall_break_xb(g, &b, 3.0).to_bits(), 0xbfe0cccccccccccc);
            assert_eq!(wall_break_zb(g, &b, 1.0).to_bits(), 0x3faeb851eb851eb8);
            assert_eq!(wall_break_cell_at(g, &b, [3.4, 1.9, 7.8], None), [4.0, 1.0, 0.0]);
            assert_eq!(wall_break_cell_at(g, &b, [5.9, 2.2, 7.1], Some(BoxId(1))), [4.0, 1.0, 0.0]);
            assert_eq!(bits(&wall_break_cell_centre(g, &b, 2.0, 1.0, 0.0)), [0x4003fb3bba39f07c, 0x3ffc000000000000, 0x401fee4eb7f830c4]);
            let r = wall_break_rect_box(g, &b, [1.0, 4.0, 0.0, 1.0, 0.0, 0.0]);
            assert_eq!(bits(&r.c), [0x4005fcd27c26a052, 0x3ff6666666666666, 0x401f9edf25502083]);
            assert_eq!(bits(&r.h), [0x3ff0cccccccccccd, 0x3ff6666666666666, 0x3faeb851eb851eb8]);
            assert_eq!(wall_break_work_left(g, 0.0, [3.6, 1.8, 7.7], [0.8, 0.0, 0.6], None), Some(3.5));
            assert_eq!(wall_break_work_left(g, 0.0, [3.6, 1.8, 7.7], [1.0, 0.0, 0.0], Some(2.4)), Some(3.5));
            assert_eq!(wall_break_work_left(g, 1.0, [2.9, 0.4, 7.7], [0.8, 0.0, 0.6], None), None);
        }).unwrap().join().unwrap();
    }

    /// expected: WallBreak.workLeft under bun: the walk along a wall sums the cells it crosses, and a sill under a standing upper counts on the upper
    #[test]
    fn the_work_along_a_wall_is_the_typescripts() {
        std::thread::Builder::new().stack_size(1 << 28).spawn(|| {
            let b = wall(&[10], &[(9, 1.75)]);
            let mut g = game_with(&[([3.25, 1.4, 7.75], [2.1, 1.4, 0.06], 0.0)], &b);
            let g = &mut *g;
            assert_eq!(wall_break_work_left(g, 0.0, [2.0, 1.8, 7.75], [1.0, 0.0, 0.0], Some(2.4)).map(f64::to_bits), Some(0x403c400000000000));
            assert_eq!(wall_break_work_left(g, 0.0, [2.0, 0.3, 7.75], [1.0, 0.0, 0.0], None).map(f64::to_bits), Some(0x4011000000000000));
        }).unwrap().join().unwrap();
    }

    /// The feed: serials go up, the ring keeps its last rows, the subscriber hears each, `since` answers from the tail.
    #[test]
    fn the_feed_keeps_its_ring_and_tells_its_subscriber() {
        std::thread::Builder::new().stack_size(1 << 28).spawn(|| {
            let mut g = alloc::boxed::Box::new(Game::blank());
            let g = &mut *g;
            g.k.walls.max_changes = 3.0;
            wall_break_on_change(g, &ChangeListener::WallReact);
            let c = WallBreakNoteC { kind: LevelChangeKind::Port, cause: WallBreakNoteCCause::Fire, ..Default::default() };
            for _ in 0..4 { wall_break_note(g, &c); }
            let reset = WallBreakNoteC { kind: LevelChangeKind::Reset, cause: WallBreakNoteCCause::Reset, ..Default::default() };
            assert!(!g.m.wallreact.wall_react.stale);
            let row = wall_break_note(g, &reset);
            assert_eq!(row.serial, 5.0);
            assert!(g.m.wallreact.wall_react.stale);
            assert_eq!(g.m.wallreact.wall_react.queue.len(), 5);
            assert_eq!(g.destr.walls.changes.iter().map(|r| r.serial).collect::<Vec<_>>(), [3.0, 4.0, 5.0]);
            assert_eq!(wall_break_since(g, 5.0).map(|v| v.len()), Some(0));
            assert_eq!(wall_break_since(g, 3.0).map(|v| v.len()), Some(2));
            assert_eq!(wall_break_since(g, 2.0).map(|v| v.len()), Some(3));
            assert!(wall_break_since(g, 1.0).is_none());
        }).unwrap().join().unwrap();
    }

    /// expected: wallbreak.ts:336 pieces itself, run on these cells under node
    #[test]
    fn the_greedy_boxes_are_the_typescripts() {
        let cases: &[(usize, usize, usize, &[usize], &[[f64; 6]])] = &[
            (4, 2, 1, &[], &[[0., 3., 0., 1., 0., 0.]]),
            (4, 2, 1, &[5], &[[0., 0., 0., 1., 0., 0.], [1., 1., 0., 0., 0., 0.], [2., 3., 0., 1., 0., 0.]]),
            (4, 2, 1, &[1, 5], &[[0., 0., 0., 1., 0., 0.], [2., 3., 0., 1., 0., 0.]]),
            (4, 2, 1, &[5, 6], &[[0., 0., 0., 1., 0., 0.], [1., 2., 0., 0., 0., 0.], [3., 3., 0., 1., 0., 0.]]),
            (1, 2, 4, &[5], &[[0., 0., 0., 1., 0., 0.], [0., 0., 0., 0., 1., 1.], [0., 0., 0., 1., 2., 3.]]),
            (3, 1, 3, &[4], &[[0., 2., 0., 0., 0., 0.], [0., 0., 0., 0., 1., 2.], [2., 2., 0., 0., 1., 2.], [1., 1., 0., 0., 2., 2.]]),
            (3, 1, 3, &[0, 8], &[[1., 2., 0., 0., 0., 1.], [0., 0., 0., 0., 1., 2.], [1., 1., 0., 0., 2., 2.]]),
            (2, 2, 2, &[7], &[[0., 1., 0., 1., 0., 0.], [0., 0., 0., 1., 1., 1.], [1., 1., 0., 0., 1., 1.]]),
            (4, 2, 1, &[0, 1, 2, 3, 4, 5, 6, 7], &[]),
        ];
        for &(nx, ny, nz, opened, want) in cases {
            let mut open = vec![0u8; nx * ny * nz];
            for &c in opened { open[c] = 1; }
            assert_eq!(pieces_of(nx, ny, nz, &open), want.to_vec(), "{nx} {ny} {nz} {opened:?}");
        }
    }
}
