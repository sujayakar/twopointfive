use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

// ---------------------------------------------------------------- seams: what skeleton 5 still cannot do

/// A box's row (Box.* in the TypeScript).
fn seam_box<'g>(g: &'g mut Game, b: BoxId) -> &'g mut crate::scene_boxes::Box { &mut g.tab.boxes[b] }
/// A box held by value, as a row, for a stub that takes an ID. `g.tab.boxes.push(cell.clone())`.
fn seam_box_push(g: &mut Game, cell: &crate::scene_boxes::Box) -> BoxId { crate::scene_boxes::box_new(g, cell.clone()) }

// ---------------------------------------------------------------- local helpers

/// a truthy `flags & mask`
fn any_bit(flags: f64, mask: f64) -> bool { js::to_i32(flags) & js::to_i32(mask) != 0 }
/// `flags | mask`
fn with_bits(flags: f64, mask: f64) -> f64 { (js::to_i32(flags) | js::to_i32(mask)) as f64 }
/// `flags & ~mask`
fn without_bits(flags: f64, mask: f64) -> f64 { (js::to_i32(flags) & !js::to_i32(mask)) as f64 }
/// g.col.boxes[i]: a place in the level's list, which holds the box's ID
fn box_at(g: &Game, i: f64) -> BoxId { g.col.boxes[i as usize] }
/// aabbOf(box), for a box named by its ID: aabb_of takes g and the row, which cannot both be lent
fn aabb(g: &mut Game, b: BoxId) -> AABB { let row = seam_box(g, b).clone(); aabb_of(g, &row) }
/// `owner.get(j)`: the breakable a static belongs to
fn owner_of(g: &Game, j: f64) -> Option<WallbreakBreakableId> { g.destr.walls.owner.get(crate::Num(j)).copied() }
/// `B.nx > 1 || B.box.h[0] >= B.box.h[2]`
fn along_x(g: &mut Game, nx: f64, bx: BoxId) -> bool {
    let h = seam_box(g, bx).h;
    nx > 1.0 || h[0] >= h[2]
}
/// `under` of restsOn: its top at a's bottom AND its middle below it
fn under(a: AABB, b: AABB) -> bool {
    b.y1 >= a.y0 - 0.06 && b.y1 <= a.y0 + 0.04 && (b.y0 + b.y1) / 2.0 < a.y0 + 0.01 && overlap_xz(a, b, None)
}
/// `touches` of held and near
fn touches(a: AABB, b: AABB, e: f64) -> bool {
    m::min(a.x1, b.x1) - m::max(a.x0, b.x0) > -e && m::min(a.y1, b.y1) - m::max(a.y0, b.y0) > -e && m::min(a.z1, b.z1) - m::max(a.z0, b.z0) > -e
}
/// the TypeScript's scratch Int32Array(96): a local, and its length still caps what gather lists
fn scratch() -> Vec<i32> { vec![0; 96] }

/// What a broken cell throws: this cell's pieces, chips and fines by its material's row.
pub(super) fn wall_break_shed(g: &mut crate::game::Game, b: WallbreakBreakableId, cell: &crate::scene_boxes::Box, dir: Option<V3>, who: f64, budget: &mut crate::break_table::ShedBudget) {
    let (surf, hard) = { let r = &g.tab.wallbreak_breakable[b]; (r.surf, r.hard) };
    let as_ = if hard == Some(true) { Some(crate::break_table::Mat::Blockwork) } else { None };
    let cell = seam_box_push(g, cell);
    let r = crate::break_table::breaks_shed_cell(g, surf, cell, dir, who, budget, as_);
    g.stats.wallbreak.chunks += r.thrown as i32;
    g.stats.wallbreak.chunks_demoted += r.refused as i32;
}

// ---------------------------------------------------------------- what stood on it / hung on it

/// What `a` (static j) rests on; `extra` counts as still standing.
fn wall_break_rests_on(g: &mut crate::game::Game, j: f64, a: crate::wallbreak::AABB, extra: Option<crate::wallbreak::AABB>) -> Vec<crate::wallbreak::AABB> {
    let mine = owner_of(g, j);
    let mut out: Vec<AABB> = Vec::new();
    let mut list = scratch();
    let n = crate::scene_collision::gather(g, a.x0, a.z0, a.x1, a.z1, a.y0 - 0.5, a.y0 + 0.05, &mut list) as usize;
    for k in 0..n {
        let i = list[k] as f64;
        if i == j || (mine.is_some() && owner_of(g, i) == mine) { continue; }
        let bx = box_at(g, i);
        let b = aabb(g, bx);
        if under(a, b) { out.push(b); }
    }
    let primary = crate::scene_boxes::box_flag().no_primary;
    for at in 0..g.destr.walls.ghosts.len() {
        let i = g.destr.walls.ghosts[at];
        if i == j { continue; }
        let o = box_at(g, i);
        if any_bit(seam_box(g, o).flags, primary) { continue; }
        let c = seam_box(g, o).c;
        if m::abs(c[0] - (a.x0 + a.x1) / 2.0) > 3.0 || m::abs(c[2] - (a.z0 + a.z1) / 2.0) > 3.0 { continue; }
        let b = aabb(g, o);
        if under(a, b) { out.push(b); }
    }
    if let Some(x) = extra { if under(a, x) { out.push(x); } }
    out
}

/// Is static j (bounds a) held up, by what it rests on or what it hangs from?
fn wall_break_held(g: &mut crate::game::Game, j: f64, a: crate::wallbreak::AABB, extra: Option<crate::wallbreak::AABB>) -> bool {
    if a.y0 <= 0.06 { return true; }
    let rest = wall_break_rests_on(g, j, a, extra);
    if !rest.is_empty() {
        let along = a.x1 - a.x0 >= a.z1 - a.z0;
        let c = if along { (a.x0 + a.x1) / 2.0 } else { (a.z0 + a.z1) / 2.0 };
        let (mut lo, mut hi) = (1e9, -1e9);
        for b in &rest {
            lo = m::min(lo, if along { b.x0 } else { b.z0 });
            hi = m::max(hi, if along { b.x1 } else { b.z1 });
        }
        return lo < c - 0.02 && hi > c + 0.02;
    }
    let mine = owner_of(g, j);
    let e = 0.04;
    if let Some(x) = extra { if touches(a, x, e) { return true; } }
    let mut list = scratch();
    let n = crate::scene_collision::gather(g, a.x0 - e, a.z0 - e, a.x1 + e, a.z1 + e, a.y0 - e, a.y1 + e, &mut list) as usize;
    for k in 0..n {
        let i = list[k] as f64;
        if i == j || (mine.is_some() && owner_of(g, i) == mine) { continue; }
        let bx = box_at(g, i);
        let b = aabb(g, bx);
        if !touches(a, b, e) { continue; }
        if b.y0 <= 0.06 || (b.x1 - b.x0) * (b.z1 - b.z0) > 6.0 || !wall_break_rests_on(g, i, b, None).is_empty() { return true; }
    }
    false
}

/// Authored statics touching `a`, ghosts and part-gone cover included, in index order.
fn wall_break_near(g: &mut crate::game::Game, a: crate::wallbreak::AABB) -> Vec<f64> {
    let e = 0.05;
    let mut out: Vec<f64> = Vec::new();
    let mut list = scratch();
    let n = crate::scene_collision::gather(g, a.x0 - e, a.z0 - e, a.x1 + e, a.z1 + e, a.y0 - e, a.y1 + e, &mut list) as usize;
    let authored = g.destr.walls.authored;
    for k in 0..n {
        let i = list[k] as f64;
        if i < authored {
            let bx = box_at(g, i);
            if touches(a, aabb(g, bx), e) && !out.contains(&i) { out.push(i); }
        }
    }
    let primary = crate::scene_boxes::box_flag().no_primary;
    for at in 0..g.destr.walls.ghosts.len() {
        let i = g.destr.walls.ghosts[at];
        let o = box_at(g, i);
        if any_bit(seam_box(g, o).flags, primary) { continue; }
        let c = seam_box(g, o).c;
        if m::abs(c[0] - (a.x0 + a.x1) / 2.0) > 4.0 || m::abs(c[2] - (a.z0 + a.z1) / 2.0) > 4.0 { continue; }
        if touches(a, aabb(g, o), e) && !out.contains(&i) { out.push(i); }
    }
    for at in 0..g.destr.walls.list.len() {
        let cid = g.destr.walls.list[at];
        let (kind, n_open, cells, id, bx) = { let c = &g.tab.wallbreak_breakable[cid]; (c.kind, c.n_open, c.open.len() as f64, c.id, c.box_) };
        if kind == BKind::Cover && n_open > 0.0 && n_open < cells && touches(a, aabb(g, bx), e) && !out.contains(&id) { out.push(id); }
    }
    js::sort_by(&mut out, |x, y| *x - *y);
    out
}

/// After cells of B opened: what B held up and nothing holds now comes down.
pub(super) fn wall_break_settle(g: &mut crate::game::Game, b: WallbreakBreakableId, rect: crate::wallbreak::AABB, dir: Option<V3>, who: f64, quiet: bool, rects: &mut Vec<[f64; 4]>) {
    let mut queue: Vec<AABB> = Vec::new();
    let (kind, bx) = { let r = &g.tab.wallbreak_breakable[b]; (r.kind, r.box_) };
    if kind == BKind::Wall { wall_break_hangers(g, b, rect, dir, who, quiet, &mut queue, rects); } else { queue.push(aabb(g, bx)); }
    wall_break_drop(g, Some(b), &mut queue, dir, who, quiet, rects);
}

/// Whatever the things in `queue` were holding, and what those held in turn, never B itself.
fn wall_break_drop(g: &mut crate::game::Game, b: Option<WallbreakBreakableId>, queue: &mut Vec<crate::wallbreak::AABB>, dir: Option<V3>, who: f64, quiet: bool, rects: &mut Vec<[f64; 4]>) {
    let mut falls = 0.0;
    let mut seen: JsSet<crate::Num> = JsSet::new();
    let (glass, primary) = { let f = crate::scene_boxes::box_flag(); (f.glass, f.no_primary) };
    while !queue.is_empty() && falls < g.k.walls.falls_per_change {
        let gone = queue.remove(0);
        for j in wall_break_near(g, gone) {
            if seen.has(crate::Num(j)) { continue; }
            let c = owner_of(g, j);
            if let Some(cid) = c {
                if b == Some(cid) { continue; }
                let row = &g.tab.wallbreak_breakable[cid];
                if row.n_open == row.open.len() as f64 { continue; }
            }
            let bx = box_at(g, j);
            let flags = seam_box(g, bx).flags;
            if any_bit(flags, glass) { continue; }
            if c.is_none() && any_bit(flags, primary) { continue; }   // (already gone)
            let a = aabb(g, bx);
            if a.y0 <= 0.06 || (a.x1 - a.x0) * (a.z1 - a.z0) > 6.0 { continue; }
            if !wall_break_held(g, j, a, Some(gone)) || wall_break_held(g, j, a, None) { continue; }   // was it held with that thing standing, and is it not now?
            seen.add(crate::Num(j));
            if wall_break_fall(g, j, a, dir, who, quiet, rects) { falls += 1.0; queue.push(a); }
        }
    }
}

/// The boards hung on the charge's face of `wall`, within `r` of the charge. Pure.
fn wall_break_hung_at(g: &mut crate::game::Game, wall: f64, at: V3, r: f64) -> Vec<f64> {
    let row = match owner_of(g, wall) { Some(b) => b, None => return Vec::new() };
    let (kind, nx, bx) = { let b = &g.tab.wallbreak_breakable[row]; (b.kind, b.nx, b.box_) };
    if kind != BKind::Wall { return Vec::new(); }
    let k_ = g.k.walls.hung;
    let wa = aabb(g, bx);
    let along = along_x(g, nx, bx);
    let bc = seam_box(g, bx).c;
    let side = if (if along { at[2] - bc[2] } else { at[0] - bc[0] }) >= 0.0 { 1.0 } else { -1.0 };
    let mut list = scratch();
    let n = crate::scene_collision::gather(g, at[0] - r - 0.4, at[2] - r - 0.4, at[0] + r + 0.4, at[2] + r + 0.4, 0.25, wa.y1, &mut list) as usize;
    let mut out: Vec<f64> = Vec::new();
    let primary = crate::scene_boxes::box_flag().no_primary;
    for k in 0..n {
        let j = list[k] as f64;
        if !wall_break_board_like(g, j) { continue; }
        let b = box_at(g, j);
        if any_bit(seam_box(g, b).flags, primary) { continue; }
        let a = aabb(g, b);
        if a.y0 > at[1] + 0.35 || a.y1 < at[1] - 0.05 { continue; }
        if (if along { a.z1 - a.z0 } else { a.x1 - a.x0 }) > k_.deep || (if along { a.x1 - a.x0 } else { a.z1 - a.z0 }) < k_.long { continue; }
        let gap = if side > 0.0 { if along { a.z0 - wa.z1 } else { a.x0 - wa.x1 } } else if along { wa.z0 - a.z1 } else { wa.x0 - a.x1 };
        if gap > 0.08 || gap < -0.02 { continue; }   // (on THIS face: the charge's)
        if m::min(if along { a.x1 } else { a.z1 }, if along { wa.x1 } else { wa.z1 }) - m::max(if along { a.x0 } else { a.z0 }, if along { wa.x0 } else { wa.z0 }) < 0.05 { continue; }
        if m::hyp2(m::max_n(&[a.x0 - at[0], 0.0, at[0] - a.x1]), m::max_n(&[a.z0 - at[2], 0.0, at[2] - a.z1])) > r { continue; }
        out.push(j);
    }
    js::sort_by(&mut out, |p, q| *p - *q);
    out
}

/// Could authored static `j` be a board hung on a wall, by what it was authored as?
fn wall_break_board_like(g: &mut crate::game::Game, j: f64) -> bool {
    if j < 0.0 || j >= g.col.boxes.len() as f64 || j >= g.destr.walls.authored || g.destr.walls.owner.has(crate::Num(j)) { return false; }
    let b = box_at(g, j);
    if seam_box(g, b).rot.is_some() { return false; }
    let flags = seam_box(g, b).flags;
    let f0 = match crate::destruction::destruction_flags0(g, j) {
        Some(f) => f,
        None => if crate::furnblast::furniture_broken(g, j) { without_bits(flags, HIDE) } else { flags },   // (as authored: one that a blast has since hidden is a board still)
    };
    let f = crate::scene_boxes::box_flag();
    if any_bit(f0, with_bits(f.glass, f.no_shadow)) || g.destr.walls.fixture_boxes.has(crate::Num(j)) || crate::destruction::destruction_item_of(g, j).is_some() || g.level.breaker.boxes.contains(&j) { return false; }
    let a = aabb(g, b);
    let k_ = g.k.walls.hung;
    let (dx, dz) = (a.x1 - a.x0, a.z1 - a.z0);
    a.y0 >= 0.25 && a.y1 - a.y0 >= k_.tall && m::min(dx, dz) <= k_.deep && m::max(dx, dz) >= k_.long
}

/// Things mounted on the stretch of wall B that opened: down once half of the cells behind them are gone.
fn wall_break_hangers(g: &mut crate::game::Game, b: WallbreakBreakableId, rect: crate::wallbreak::AABB, dir: Option<V3>, who: f64, quiet: bool, queue: &mut Vec<crate::wallbreak::AABB>, rects: &mut Vec<[f64; 4]>) {
    let (b_id, bx, nx, ny, nz) = { let r = &g.tab.wallbreak_breakable[b]; (r.id, r.box_, r.nx, r.ny, r.nz) };
    let wa = aabb(g, bx);
    let along = along_x(g, nx, bx);
    let mut list = scratch();
    let n = crate::scene_collision::gather(g, rect.x0 - 0.3, rect.z0 - 0.3, rect.x1 + 0.3, rect.z1 + 0.3, 0.25, wa.y1, &mut list) as usize;
    let mut cand: Vec<f64> = Vec::new();
    for k in 0..n { cand.push(list[k] as f64); }
    let (glass, primary) = { let f = crate::scene_boxes::box_flag(); (f.glass, f.no_primary) };
    for at in 0..g.destr.walls.ghosts.len() {
        let j = g.destr.walls.ghosts[at];
        let o = box_at(g, j);
        if any_bit(seam_box(g, o).flags, primary) { continue; }
        let c = seam_box(g, o).c;
        if c[0] > rect.x0 - 2.0 && c[0] < rect.x1 + 2.0 && c[2] > rect.z0 - 2.0 && c[2] < rect.z1 + 2.0 { cand.push(j); }
    }
    js::sort_by(&mut cand, |x, y| *x - *y);
    let bc1 = seam_box(g, bx).c[1];
    for j in cand {
        if g.destr.walls.owner.has(crate::Num(j)) { continue; }
        let bj = box_at(g, j);
        if any_bit(seam_box(g, bj).flags, with_bits(glass, primary)) { continue; }
        let a = aabb(g, bj);
        if a.y0 < 0.25 { continue; }
        if m::min(a.x1, rect.x1 + 0.3) - m::max(a.x0, rect.x0 - 0.3) < 0.0 || m::min(a.z1, rect.z1 + 0.3) - m::max(a.z0, rect.z0 - 0.3) < 0.0 { continue; }
        let depth = if along { a.z1 - a.z0 } else { a.x1 - a.x0 };
        if depth > 0.4 { continue; }
        let gap = if along { m::max(a.z0 - wa.z1, wa.z0 - a.z1) } else { m::max(a.x0 - wa.x1, wa.x0 - a.x1) };
        if gap > 0.08 { continue; }
        let (l0, l1, w0, w1) = if along { (a.x0, a.x1, wa.x0, wa.x1) } else { (a.z0, a.z1, wa.z0, wa.z1) };
        if m::min(l1, w1) - m::max(l0, w0) < 0.05 { continue; }
        // the cells behind it
        let nn = if along { nx } else { nz };
        let (mut tot, mut gone, mut sill) = (0.0, 0.0, 0.0);
        let mut k = 0.0;
        while k < nn {
            let (kk, step) = (k, k + 1.0);
            k = step;
            let c0 = w0 + (w1 - w0) * kk / nn;
            let c1 = w0 + (w1 - w0) * step / nn;
            if m::min(c1, l1) - m::max(c0, l0) <= 0.01 { continue; }
            let (cx, cz) = if along { (kk, 0.0) } else { (0.0, kk) };
            let mut up = 0.0;
            let mut iy = 0.0;
            while iy < ny {
                let cell = wall_break_ci(g, b, cx, iy, cz) as usize;
                if g.tab.wallbreak_breakable[b].open[cell] != 0 { up += 1.0; }
                iy += 1.0;
            }
            if up == ny { sill += 1.0; }   // (a column open floor to top)
            let mut iy = 0.0;
            while iy < ny {
                let at = iy as usize;
                let (y0, y1) = { let r = &g.tab.wallbreak_breakable[b]; (bc1 + r.yb[at], bc1 + r.yb[at + 1]) };
                if m::min(y1, a.y1) - m::max(y0, a.y0) > 0.01 {
                    tot += 1.0;
                    let cell = wall_break_ci(g, b, cx, iy, cz) as usize;
                    if g.tab.wallbreak_breakable[b].open[cell] != 0 { gone += 1.0; }
                }
                iy += 1.0;
            }
        }
        // (the charge's board goes with its wall once a column behind it is open from the floor up)
        let mine = !quiet && sill > 0.0 && match &g.destr.walls.hung_now { Some(h) => h.wall == b_id && h.set.contains(&j), None => false };
        let mut forced = false;
        if tot == 0.0 || gone * 2.0 < tot { if !mine { continue; } forced = true; }
        if !wall_break_rests_on(g, j, a, None).is_empty() { if !mine { continue; } forced = true; }   // (standing on something after all)
        if wall_break_fall(g, j, a, dir, who, quiet, rects) {
            queue.push(a);
            if forced { g.destr.walls.hung_gone.set(crate::Num(j), b_id); }
        }
    }
}

/// Static j has nothing left to stand on or hang from. True = it is gone from where it was.
fn wall_break_fall(g: &mut crate::game::Game, j: f64, a: crate::wallbreak::AABB, dir: Option<V3>, who: f64, quiet: bool, rects: &mut Vec<[f64; 4]>) -> bool {
    let b = box_at(g, j);
    let d: V3 = match dir { Some(x) => v3::normalize([x[0], 0.2, x[2]]), None => [0.0, -1.0, 0.0] };
    if let Some(cid) = owner_of(g, j) {   // cover that stood on it: all of it goes
        let cells: Vec<f64> = g.tab.wallbreak_breakable[cid].open.iter().enumerate().filter(|(_, o)| **o == 0).map(|(c, _)| c as f64).collect();
        let cause = if quiet { LevelChangeCause::Load } else { LevelChangeCause::Collapse };
        let mut budget = crate::break_table::ShedBudget { chunks: 2.0, heap: None, from: None, per: None };
        return wall_break_open_cells(g, cid, &cells, cause, who, dir, Some(quiet), Some(&mut budget)).is_some();
    }
    g.stats.wallbreak.falls += 1;
    if crate::destruction::destruction_item_of(g, j).is_some() {
        crate::destruction::destruction_drop_item(g, j, d, Some(who), Some(quiet));
        rects.push([a.x0, a.z0, a.x1, a.z1]);
        return true;
    }
    let h = seam_box(g, b).h;
    let vol = h[0] * h[1] * h[2] * 8.0;
    if vol > g.k.walls.fall_max_vol { g.stats.wallbreak.floating += 1; return false; }   // too big to throw about: it stays
    if g.destr.walls.fixture_boxes.has(crate::Num(j)) && !quiet { crate::destruction::destruction_break_fixture_box(g, j); }
    let flags = seam_box(g, b).flags;
    g.destr.walls.hidden.set(crate::Num(j), flags);
    seam_box(g, b).flags = with_bits(flags, HIDE);
    wall_break_bump(g);
    let c = seam_box(g, b).c;
    rects.push([a.x0, a.z0, a.x1, a.z1]);
    let hung = if quiet { None } else { g.destr.walls.hung_now.as_ref().filter(|n| n.set.contains(&j)).map(|n| (n.at, n.n)) };
    if let Some((at, n)) = hung {
        crate::furnblast::furniture_hung_goes(g, b, at, n, who);   // (what the charge was set on: in pieces, not let down whole)
    } else if !quiet {
        let v0 = d[0] * 0.8 + (wall_break_hash(g) - 0.5) * 0.4;
        let v2 = d[2] * 0.8 + (wall_break_hash(g) - 0.5) * 0.4;
        let v: V3 = [v0, 0.2, v2];
        let w0 = (wall_break_hash(g) - 0.5) * 3.0;
        let w1 = (wall_break_hash(g) - 0.5) * 2.0;
        let w2 = (wall_break_hash(g) - 0.5) * 3.0;
        let w: V3 = [w0, w1, w2];
        let got = crate::debris::debris_spawn(g, crate::debris::ChunkKind::Chunk, b, v, w, Some(who), None);
        if got.is_some() {
            g.stats.wallbreak.chunks += 1;
        } else {
            g.stats.wallbreak.chunks_demoted += 1;
            let albedo = seam_box(g, b).albedo;
            crate::render_particles::particles_emit(g, crate::render_particles::PKind::Debris, c, [0.0, 1.0, 0.0], 8.0, crate::render_particles::EmitOpts { speed: 1.8, speed_jit: Some(0.7), spread: Some(1.2), life: 16.0, life_jit: Some(0.3), size: 0.012, size_jit: Some(0.6), color: Some(albedo), owner: Some(who), ..Default::default() });
        }
    }
    true
}

// ---------------------------------------------------------------- the sites

/// A round stopped in static `index` (≥ 1e6 = a door leaf) at `hit_p` travelling `dir`.
pub fn wall_break_round(g: &mut crate::game::Game, who: f64, hit_p: V3, dir: V3, index: f64, def: &crate::weapons::WeaponDef, chaos: f64) {
    if !wall_break_on(g) { return; }
    let k_ = if def.cls == crate::weapons::WeaponClass::Pistol { g.k.walls.pistol_k } else { 1.0 };
    let w = k_ * chaos * 2.0;
    if index >= 1e6 { crate::doorblast::leaf_round(g, index, who, hit_p, dir, w); return; }   // a door leaf: the doors' own
    let b = match owner_of(g, index) { Some(b) => b, None => return };
    if g.tab.wallbreak_breakable[b].hard == Some(true) { return; }   // (a hard wall scars like the structure it is)
    g.stats.wallbreak.rounds += 1;
    let bx = box_at(g, index);
    let [ix, mut iy, iz] = wall_break_cell_at(g, b, v3::mad(hit_p, dir, 0.02), Some(bx));
    let c_hit = wall_break_ci(g, b, ix, iy, iz);   // (the cell the round met: the wear keys its marks by it)
    let (id, kind, surf, b_box) = { let r = &g.tab.wallbreak_breakable[b]; (r.id, r.kind, r.surf, r.box_) };
    if kind == BKind::Wall && iy == 0.0 {   // a sill never goes from under a standing upper
        let upper = wall_break_ci(g, b, ix, 1.0, iz) as usize;
        if g.tab.wallbreak_breakable[b].open[upper] == 0 { iy = 1.0; }
    }
    let c = wall_break_ci(g, b, ix, iy, iz);
    let r = &mut g.tab.wallbreak_breakable[b];
    if r.open[c as usize] != 0 { return; }
    let dmg = ((r.dmg[c as usize] as f64) + w) as f32;   // (a Float32Array: the sum is a double, the store rounds, the read below sees the rounded)
    r.dmg[c as usize] = dmg;
    g.destr.walls.dirty = true;
    let tough = g.k.walls.tough.get(String::from(surf.as_str())).copied().unwrap_or(8.0) * (if kind == BKind::Wall && iy == 0.0 { g.k.walls.sill_k } else { 1.0 });
    crate::wallwear::wall_wear_round(g, b_box, surf, id * 4096.0 + c_hit, id * 4096.0 + c, (dmg as f64) / tough, hit_p, dir);   // (look only: reads the count)
    if (dmg as f64) >= tough { wall_break_open_cells(g, b, &[c], LevelChangeCause::Fire, who, Some(dir), None, None); }
}

/// The plan of a blast at `at`, the boards the charge is set on out of its lines. Writes nothing it does not put back.
fn wall_break_plan_blast(g: &mut crate::game::Game, at: V3, opts: crate::wallbreak::BlastOpts, r: f64, hung: Option<&[f64]>) -> Vec<crate::wallbreak::WallBreakPlanBlastItem> {
    let own: Vec<f64> = match hung {
        Some(h) => Vec::from(h),
        None => match opts.hung { Some(x) => wall_break_hung_at(g, x.wall, at, x.r), None => Vec::new() },
    };
    let as_item = |p: WallBreakPlanOfItem| WallBreakPlanBlastItem { b: p.b, cells: p.cells };
    if own.is_empty() { return wall_break_plan_of(g, at, opts, r).into_iter().map(as_item).collect(); }
    // (the boards stand in no line of the plan: hidden as the engine hides a static, and put back)
    let own: Vec<BoxId> = own.iter().map(|&j| box_at(g, j)).collect();
    let mut was: Vec<f64> = Vec::new();
    for &b in &own { was.push(seam_box(g, b).flags); }
    for k in 0..own.len() { seam_box(g, own[k]).flags = with_bits(was[k], HIDE); }
    crate::scene_collision::wrote(g);
    let plan = wall_break_plan_of(g, at, opts, r);
    for k in 0..own.len() { seam_box(g, own[k]).flags = was[k]; }
    crate::scene_collision::wrote(g);
    plan.into_iter().map(as_item).collect()
}

/// `exposed` of planOf: does the blast's eye reach `p` on B, by a line that meets nothing but B?
fn exposed(g: &mut Game, b: WallbreakBreakableId, eye: V3, p: V3) -> bool {
    let d = v3::sub(p, eye);
    let l = v3::len(d);
    if l < 0.05 { return true; }
    match crate::scene_collision::raycast(g, eye, v3::scale(d, 1.0 / l), l) {
        None => true,
        Some(hit) => owner_of(g, hit.index) == Some(b) || hit.t >= l - 0.02,
    }
}

/// A hard wall's hole: the first column and how many, minHole half-metre columns at the charge. `u` is the charge's place in columns.
fn hard_hole(nn: f64, col_w: f64, min_hole: f64, col: f64, u: f64) -> (f64, f64) {
    let n = m::min(nn, m::max(1.0, m::ceil(min_hole * col / col_w - 1e-6)));
    let lo = m::max(0.0, m::min(nn - n, m::round(u - n / 2.0)));
    (lo, n)
}

/// The run of columns to open round the holed ones nearest the charge, widened to minHole.
fn widen_hole(full: &[f64], nearest: f64, min_hole: f64, nn: f64) -> (f64, f64) {
    let mut lo = full.iter().fold(full[0], |best, &k| if m::abs(k - nearest) < m::abs(best - nearest) { k } else { best });
    let mut hi = lo;
    while full.contains(&(lo - 1.0)) { lo -= 1.0; }
    while full.contains(&(hi + 1.0)) { hi += 1.0; }
    while hi - lo + 1.0 < m::min(min_hole, nn) {
        let cl = if lo > 0.0 { m::abs(lo - 1.0 - nearest) } else { 1e9 };
        let ch = if hi < nn - 1.0 { m::abs(hi + 1.0 - nearest) } else { 1e9 };
        if cl <= ch { lo -= 1.0; } else { hi += 1.0; }
    }
    (lo, hi)
}

/// The plan of a blast at `at`: which cells of which breakables it opens, nearest first. Pure.
fn wall_break_plan_of(g: &mut crate::game::Game, at: V3, opts: crate::wallbreak::BlastOpts, r: f64) -> Vec<crate::wallbreak::WallBreakPlanOfItem> {
    let k_ = g.k.walls.blast;
    let col = g.k.walls.col;
    let eye: V3 = [at[0], m::max(0.3, m::min(at[1] + 0.3, crate::scene_world::world().ceiling_y - 0.3)), at[2]];
    let mut plan: Vec<WallBreakPlanOfItem> = Vec::new();
    for row in 0..g.destr.walls.list.len() {
        let b = g.destr.walls.list[row];
        let (id, kind, hard, bx, nx, nz) = {
            let r = &g.tab.wallbreak_breakable[b];
            if r.n_open == r.open.len() as f64 { continue; }
            (r.id, r.kind, r.hard, r.box_, r.nx, r.nz)
        };
        let a = aabb(g, bx);
        let dx = m::max_n(&[a.x0 - at[0], 0.0, at[0] - a.x1]);
        let dz = m::max_n(&[a.z0 - at[2], 0.0, at[2] - a.z1]);
        let d_b = m::hyp2(dx, dz);
        if d_b > r { continue; }
        let mut cells: Vec<f64> = Vec::new();
        if hard == Some(true) {   // a hard wall: only the charge set on it opens it, and only its hole
            if opts.breach != Some(id) || d_b > k_.contact_m { continue; }
            let along = along_x(g, nx, bx);
            let nn = if along { nx } else { nz };
            let h = seam_box(g, bx).h;
            let col_w = 2.0 * (if along { h[0] } else { h[2] }) / nn;
            let u = (if along { (at[0] - a.x0) / (a.x1 - a.x0) } else { (at[2] - a.z0) / (a.z1 - a.z0) }) * nn;
            let (lo, n) = hard_hole(nn, col_w, k_.min_hole, col, u);
            let mut k = lo;
            while k < lo + n {
                for iy in [1.0, 0.0] { cells.push(wall_break_ci(g, b, if along { k } else { 0.0 }, iy, if along { 0.0 } else { k })); }
                k += 1.0;
            }
        } else if kind == BKind::Wall {
            let along = along_x(g, nx, bx);
            let nn = if along { nx } else { nz };
            let (bc1, bh1) = (seam_box(g, bx).c[1], seam_box(g, bx).h[1]);
            let mut full: Vec<f64> = Vec::new();
            let (mut nearest, mut nd) = (-1.0, 1e9);
            let mut k = 0.0;
            while k < nn {
                let here = k;
                k += 1.0;
                let (ix, iz) = if along { (here, 0.0) } else { (0.0, here) };
                let p = wall_break_cell_centre(g, b, ix, 1.0, iz);
                let d = m::hyp2(p[0] - at[0], p[2] - at[2]);
                if d < nd { nd = d; nearest = here; }
                if d > r * k_.port_k { continue; }
                let aim: V3 = [p[0], m::min(m::max(eye[1], 0.9), bc1 + bh1 - 0.1), p[2]];
                if !exposed(g, b, eye, aim) { continue; }
                cells.push(wall_break_ci(g, b, ix, 1.0, iz));
                if d <= r * k_.hole_k { cells.push(wall_break_ci(g, b, ix, 0.0, iz)); full.push(here); }
            }
            // the charge's own wall goes at the charge whatever stands on its face between them
            let charge_own = match opts.hung { Some(h) => owner_of(g, h.wall) == Some(b), None => false };
            if opts.hung.is_some() && d_b <= k_.contact_m && nearest >= 0.0 && !full.contains(&nearest) && charge_own {
                full.push(nearest);
                for iy in [1.0, 0.0] {
                    let c = wall_break_ci(g, b, if along { nearest } else { 0.0 }, iy, if along { 0.0 } else { nearest });
                    if !cells.contains(&c) { cells.push(c); }
                }
            }
            // a charge against the wall makes a hole a man fits through
            if !full.is_empty() && d_b <= k_.contact_m && nn > full.len() as f64 {
                let (lo, hi) = widen_hole(&full, nearest, k_.min_hole, nn);
                let mut k = lo;
                while k <= hi {
                    let (ix, iz) = if along { (k, 0.0) } else { (0.0, k) };
                    for iy in [0.0, 1.0] {
                        let c = wall_break_ci(g, b, ix, iy, iz);
                        if !cells.contains(&c) { cells.push(c); }
                    }
                    k += 1.0;
                }
            }
        } else {
            let mut iz = 0.0;
            while iz < nz {
                let mut ix = 0.0;
                while ix < nx {
                    let p = wall_break_cell_centre(g, b, ix, 0.0, iz);
                    if m::hyp2(p[0] - at[0], p[2] - at[2]) <= r * k_.cover_k && exposed(g, b, eye, p) { cells.push(wall_break_ci(g, b, ix, 0.0, iz)); }
                    ix += 1.0;
                }
                iz += 1.0;
            }
        }
        if !cells.is_empty() { plan.push(WallBreakPlanOfItem { b, cells, d: d_b }); }
    }
    js::sort_by(&mut plan, |x, y| js::or(x.d - y.d, g.tab.wallbreak_breakable[x.b].id - g.tab.wallbreak_breakable[y.b].id));
    plan
}

/// READ ONLY: what a blast at `at` would open, asked before it goes off.
pub(in crate::breakage) fn wall_break_would_open(g: &mut crate::game::Game, at: V3, opts: Option<crate::wallbreak::BlastOpts>) -> Vec<crate::wallbreak::WallOpening> {
    let opts = opts.unwrap_or_default();
    if !wall_break_on(g) { return Vec::new(); }
    let r = opts.radius.unwrap_or(g.k.walls.blast.radius) * opts.power.unwrap_or(1.0);
    if !(r > 0.0) { return Vec::new(); }
    let mut out: Vec<WallOpening> = Vec::new();
    for item in wall_break_plan_blast(g, at, opts, r, None) {
        let b = item.b;
        let (id, kind, bx, nx, nz, n_open, live, flags0) = { let r = &g.tab.wallbreak_breakable[b]; (r.id, r.kind, r.box_, r.nx, r.nz, r.n_open, r.live, r.flags0) };
        let mut open = g.tab.wallbreak_breakable[b].open.clone();
        let (mut n, mut hole) = (0.0, false);
        for &c in &item.cells {
            if open[c as usize] == 0 {
                open[c as usize] = 1;
                n += 1.0;
                if kind == BKind::Wall && c < nx * nz { hole = true; }
            }
        }
        if n == 0.0 { continue; }
        let standing: Vec<BoxId> = if n_open == 0.0 { vec![bx] } else { g.tab.wallbreak_breakable[b].slots[..live as usize].iter().map(|&i| box_at(g, i)).collect() };
        let mut left: Vec<BoxId> = Vec::new();
        if n_open + n < open.len() as f64 {
            let (yaw, albedo, stuff, does) = { let o = &*seam_box(g, bx); (o.yaw, o.albedo, o.stuff, o.does) };
            let word = crate::scene_boxes::word_of(crate::scene_boxes::WordOfB { stuff, does });
            for rect in wall_break_pieces(g, b, Some(&open)) {
                let rb = wall_break_rect_box(g, b, rect);
                let p = crate::scene_boxes::MakeBoxP { c: rb.c, h: rb.h, yaw: Some(yaw), albedo: Some(albedo), flags: Some(flags0), name: Some(String::from("piece")), stuff: word.stuff, does: word.does, ..Default::default() };
                left.push(crate::scene_boxes::make_box(g, &p));
            }
        }
        out.push(WallOpening { id, kind, standing, left, cells: n, hole });
    }
    out
}

/// The blast: open what it reaches; `hung_now` lasts as long as it does.
pub(in crate::breakage) fn wall_break_blast(g: &mut crate::game::Game, at: V3, opts: Option<crate::wallbreak::BlastOpts>) -> crate::wallbreak::BlastResult {
    let res = wall_break_blast_of(g, at, opts.unwrap_or_default());
    g.destr.walls.hung_now = None;
    res
}

/// Everything is judged against the level as it stood when it went off, then opened.
fn wall_break_blast_of(g: &mut crate::game::Game, at: V3, opts: crate::wallbreak::BlastOpts) -> crate::wallbreak::BlastResult {
    let mut res = BlastResult { walls: 0.0, cover: 0.0, doors: 0.0, panes: 0.0, opened: 0.0, serial: g.destr.walls.serial };
    if !wall_break_on(g) { return res; }
    let k_ = g.k.walls.blast;
    let r = opts.radius.unwrap_or(k_.radius) * opts.power.unwrap_or(1.0);
    let who = js::or(opts.who.unwrap_or(0.0), crate::consts::NOBODY_ID);
    if !(r > 0.0) { return res; }
    g.stats.wallbreak.blasts += 1;
    // HB: the wall the charge is set on, if this engine owns it
    let hb = match opts.hung { Some(h) => owner_of(g, h.wall), None => None };
    let hung: Vec<f64> = match (hb, opts.hung) { (Some(_), Some(h)) => wall_break_hung_at(g, h.wall, at, h.r), _ => Vec::new() };
    g.destr.walls.hung_now = match hb {
        Some(k) if !hung.is_empty() => {
            let (wall, nx, bx) = { let b = &g.tab.wallbreak_breakable[k]; (b.id, b.nx, b.box_) };
            let ax = along_x(g, nx, bx);
            let c = seam_box(g, bx).c;
            let n: V3 = if ax { [0.0, 0.0, if at[2] >= c[2] { 1.0 } else { -1.0 }] } else { [if at[0] >= c[0] { 1.0 } else { -1.0 }, 0.0, 0.0] };
            Some(WallBreakHungNow { wall, at, n, set: hung.clone() })
        }
        _ => None,
    };   // (read by hangers and fall until this blast is done)
    let plan = wall_break_plan_blast(g, at, opts, r, Some(&hung));
    let share = crate::break_table::breaks_share(g);   // (CUT.on is settled on the page)
    let mut budget = crate::break_table::ShedBudget { chunks: share.thrown, heap: Some(share.heap), from: Some(at), per: None };
    for item in plan {
        let b = item.b;
        let (b0, kind, surf, bx, nx, nz, hard) = { let r = &g.tab.wallbreak_breakable[b]; (r.n_open, r.kind, r.surf, r.box_, r.nx, r.nz, r.hard) };
        let c = seam_box(g, bx).c;
        let mut dir = v3::sub(c, at);
        dir[1] = 0.0;
        if hard == Some(true) {   // (what a long wall throws leaves through its far face)
            let ax = along_x(g, nx, bx);
            dir[if ax { 0 } else { 2 }] = 0.0;
        }
        let fresh: Vec<f64> = { let r = &g.tab.wallbreak_breakable[b]; item.cells.iter().copied().filter(|&c| r.open[c as usize] == 0).collect() };
        // (the share is dealt round the opening, nearest the charge first)
        let mut keyed: Vec<(f64, f64)> = Vec::new();
        for &c in &fresh {
            let p = wall_break_cell_box(g, b, js::rem(c, nx), m::floor(c / (nx * nz)), js::rem(m::floor(c / nx), nz)).c;
            keyed.push((c, m::pow(p[0] - at[0], 2.0) + m::pow(p[1] - at[1], 2.0) + m::pow(p[2] - at[2], 2.0)));
        }
        js::sort_by(&mut keyed, |x, y| js::or(x.1 - y.1, x.0 - y.0));
        let fresh: Vec<f64> = keyed.into_iter().map(|x| x.0).collect();
        budget.per = Some(if fresh.len() as f64 * 2.0 > budget.chunks { 1.0 } else { 2.0 });
        let aim = if v3::len(dir) > 0.05 { v3::normalize(dir) } else { [0.0, 0.0, 1.0] };
        let opened = wall_break_open_cells(g, b, &fresh, LevelChangeCause::Blast, who, Some(aim), Some(false), Some(&mut budget));
        let opened = match opened { Some(x) => x, None => continue };
        if g.tab.wallbreak_breakable[b].n_open == b0 { continue; }
        if kind == BKind::Wall { res.walls += 1.0; } else { res.cover += 1.0; }
        res.opened += opened.opened.len() as f64;
        if opened.kind == LevelChangeKind::Hole {   // (a few slabs dealt at rest at the foot of the hole, on the far side)
            let (mut mid0, mut mid2, mut w, mut n) = (0.0, 0.0, 0.0, 0.0);
            for &c in &fresh {
                if c < nx * nz {
                    let cb = wall_break_cell_box(g, b, js::rem(c, nx), 0.0, js::rem(m::floor(c / nx), nz));
                    mid0 += cb.c[0];
                    mid2 += cb.c[2];
                    w += m::max(cb.h[0], cb.h[2]);
                    n += 1.0;
                }
            }
            if n != 0.0 {
                let as_ = if hard == Some(true) { Some(crate::break_table::Mat::Blockwork) } else { None };
                crate::break_table::breaks_heap(g, surf, bx, [mid0 / n, 0.0, mid2 / n], at, w, Some(who), Some(&mut budget), as_);
            }
        }
    }
    res.serial = g.destr.walls.serial;
    res
}

// ---------------------------------------------------------------- reset

/// `letGo` of reset: a hard wall is the run's, not the level's.
fn let_go(g: &mut Game) {
    g.destr.walls.shell_gaps.clear();
    let hard: Vec<WallbreakBreakableId> = g.destr.walls.list.iter().copied().filter(|&b| g.tab.wallbreak_breakable[b].hard == Some(true)).collect();
    if hard.is_empty() { return; }
    for &b in &hard {
        let id = g.tab.wallbreak_breakable[b].id;
        g.destr.walls.owner.delete(crate::Num(id));
    }
    g.destr.walls.list.retain(|b| !hard.contains(b));
}

/// The level put back: authored boxes as they were, appended statics cut off, nav re-baked. Nothing at all when nothing broke.
pub(in crate::breakage) fn wall_break_reset(g: &mut crate::game::Game, silent: Option<bool>) {
    let silent = silent.unwrap_or(false);
    g.destr.walls.counter = 1.0;
    crate::wallwear::wall_wear_reset(g);
    crate::wallskin::wall_skin_clear(g);
    g.destr.walls.hung_now = None;
    g.destr.walls.hung_gone.clear();
    let authored = g.destr.walls.authored;
    if !g.destr.walls.dirty && g.col.boxes.len() as f64 == authored { let_go(g); return; }
    let hidden: Vec<(crate::Num, f64)> = g.destr.walls.hidden.iter().map(|(i, f)| (i, *f)).collect();
    for (i, f) in hidden { let b = box_at(g, i.0); seam_box(g, b).flags = f; }
    g.destr.walls.hidden.clear();
    let mut any = false;
    for k in 0..g.destr.walls.list.len() {
        let id = g.destr.walls.list[k];
        let (bx, flags0) = {
            let b = &g.tab.wallbreak_breakable[id];
            if b.n_open != 0.0 || !b.slots.is_empty() { any = true; }
            (b.box_, b.flags0)
        };
        seam_box(g, bx).flags = flags0;
        let b = &mut g.tab.wallbreak_breakable[id];
        b.open.fill(0);
        b.dmg.fill(0.0);
        b.n_open = 0.0;
        b.slots.clear();
        b.live = 0.0;
    }
    for i in g.destr.walls.owner.keys() { if i.0 >= authored { g.destr.walls.owner.delete(i); } }
    let_go(g);
    crate::scene_collision::truncate_statics(g, authored);
    if g.engine.world.statics.len() as f64 > authored { crate::scene_boxes::box_world_keep_statics(g, authored as usize); }
    g.destr.walls.dirty = false;
    wall_break_bump(g);
    if !any { return; }
    let world = crate::scene_world::world();
    crate::scene_collision::nav_grid_rebake(g, world.min[0], world.min[2], world.max[0], world.max[2]);
    crate::scene_collision::sound_paths_invalidate(g);
    if !silent {
        wall_break_note(g, &WallBreakNoteC {
            kind: LevelChangeKind::Reset,
            cause: LevelChangeCause::Reset,
            who: crate::consts::NOBODY_ID,
            pos: [0.0, 0.0, 0.0],
            id: -1.0,
            rect: [world.min[0], world.min[2], world.max[0], world.max[2]],
            opened: Vec::new(),
            walkable: false,
        });
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hard_hole_as_the_typescript() {
        // (expected values: planOf's lines 682-684 run under node)
        assert_eq!(hard_hole(8.0, 0.5, 3.0, 0.5, 4.0), (3.0, 3.0));
        assert_eq!(hard_hole(8.0, 0.5, 3.0, 0.5, 0.2), (0.0, 3.0));
        assert_eq!(hard_hole(8.0, 0.5, 3.0, 0.5, 7.9), (5.0, 3.0));
        assert_eq!(hard_hole(2.0, 1.0, 3.0, 0.5, 1.0), (0.0, 2.0));
        assert_eq!(hard_hole(10.0, 0.7, 3.0, 0.5, 5.5), (4.0, 3.0));
        assert_eq!(hard_hole(6.0, 0.5, 3.0, 0.5, 2.5), (1.0, 3.0));
    }

    #[test]
    fn widen_hole_as_the_typescript() {
        // (expected values: planOf's lines 701-703 run under node)
        assert_eq!(widen_hole(&[4.0], 4.0, 3.0, 8.0), (3.0, 5.0));
        assert_eq!(widen_hole(&[0.0], 0.0, 3.0, 8.0), (0.0, 2.0));
        assert_eq!(widen_hole(&[7.0], 7.0, 3.0, 8.0), (5.0, 7.0));
        assert_eq!(widen_hole(&[2.0, 3.0, 6.0], 6.0, 3.0, 8.0), (5.0, 7.0));
        assert_eq!(widen_hole(&[1.0, 2.0, 5.0], 3.0, 5.0, 8.0), (1.0, 5.0));
        assert_eq!(widen_hole(&[3.0], 3.0, 3.0, 4.0), (1.0, 3.0));
        assert_eq!(widen_hole(&[0.0, 1.0], 1.0, 3.0, 2.0), (0.0, 1.0));
    }
}
