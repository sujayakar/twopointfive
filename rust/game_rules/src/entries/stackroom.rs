pub use ::game_types::stackroom::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{string::String, vec, vec::Vec};

use crate::ask::Over;

#[inline]
fn flat(a: V3, b: V3) -> f64 { v3::dist_xz(a, b) }

// ---------------------------------------------------------------- the book

/// keeps which of the book's floors and doorway cells have been read, for an ask made only to be looked at
pub fn stack_room_book_keep(g: &crate::game::Game) -> crate::stackroom::StackRoomBookKeep {
    let Some(k) = &g.m.stackroom.books else { return crate::stackroom::StackRoomBookKeep::Forget };
    let mut floors: JsMap<DoorId, [bool; 2]> = JsMap::new();
    for (d, pair) in k.floors.iter() { floors.set(d, [pair[0].is_some(), pair[1].is_some()]); }
    crate::stackroom::StackRoomBookKeep::Back { sig: k.sig, doors: k.doors, near: k.near.is_some(), floors }
}

/// puts back what the keep took: the closure stackRoomBookKeep answers
pub fn stack_room_book_put_back(g: &mut crate::game::Game, keep: crate::stackroom::StackRoomBookKeep) {
    let (sig, doors, near, floors) = match keep {
        crate::stackroom::StackRoomBookKeep::Forget => { g.m.stackroom.books = None; return; }
        crate::stackroom::StackRoomBookKeep::Back { sig, doors, near, floors } => (sig, doors, near, floors),
    };
    // the book was rebuilt during the ask: the old one put back would be rebuilt empty at the next bookOf
    let same = match &g.m.stackroom.books { Some(k) => k.sig == sig && k.doors == doors, None => false };
    if !same { g.m.stackroom.books = None; return; }
    let k = g.m.stackroom.books.as_mut().unwrap();
    if !near { k.near = None; }
    // only the doors kept, in the order kept, as the copied map of ts:94 has them
    let mut back = JsMap::new();
    for (d, read) in floors.iter() {
        let mut pair = k.floors.delete(d).unwrap_or([None, None]);
        for at in 0..2 { if !read[at] { pair[at] = None; } }
        back.set(d, pair);
    }
    k.floors = back;
}

/// this game's book of floors, new if the grid's cells or the doors changed
fn book_of<'g>(g: &'g mut crate::game::Game) -> &'g mut crate::stackroom::Book {
    let sig = g.engine.world.geom_serial;
    let doors = g.doors.list.len() as f64;
    let stale = match &g.m.stackroom.books { Some(k) => k.sig != sig || k.doors != doors, None => true };
    if stale {
        g.m.stackroom.books = Some(crate::stackroom::Book { sig, doors, near: None, floors: JsMap::new() });
    }
    g.m.stackroom.books.as_mut().unwrap()
}

/// the middle of a nav cell
fn centre(g: &mut crate::game::Game, c_2: f64) -> V3 {
    let (w, cell) = (g.col.nav.w, g.col.nav.cell);
    [(js::rem(c_2, w) + 0.5) * cell, 0.0, (js::to_i32(c_2 / w) as f64 + 0.5) * cell]
}

/// some doorway's frame line lies between a and b inside its opening, or one of them is on it
fn door_between(g: &crate::game::Game, a: V3, b: V3) -> bool {
    for n in 0..g.doors.list.len() {
        let o = g.doors.list[n];
        let width = g.dr[o].def.width;
        if flat(a, g.dr[o].frame_centre) > width + 1.5 { continue; }
        let pa = crate::doortools::off_of(g, o, a, 1.0);
        let pb = crate::doortools::off_of(g, o, b, 1.0);
        if (pa > ON_LINE && pb > ON_LINE) || (pa < -ON_LINE && pb < -ON_LINE) { continue; }
        let t = if m::abs(pa - pb) < 1e-9 { 0.5 } else { m::min(1.0, m::max(0.0, pa / (pa - pb))) };
        let at = crate::doortools::along_of(g, o, [a[0] + (b[0] - a[0]) * t, 0.0, a[2] + (b[2] - a[2]) * t]);
        if at >= -0.1 && at <= width + 0.1 { return true; }
    }
    false
}

/// nav.idx(x, z)
#[inline]
fn nav_idx(g: &Game, x: f64, z: f64) -> f64 { crate::scene_collision::nav_grid_idx(g, x, z) }
/// nav.isBlocked(x, z)
#[inline]
fn nav_blocked(g: &Game, x: f64, z: f64) -> bool { crate::scene_collision::nav_grid_is_blocked(g, x, z) }
/// a cell on the grid and not blocked
fn cell_free(g: &Game, x: f64, z: f64) -> bool {
    if !(x >= 0.0 && z >= 0.0 && x < g.col.nav.w && z < g.col.nav.h) { return false; }
    let i = nav_idx(g, x, z) as usize;
    g.col.nav.blocked[i] != 1
}

/// the cells a frame line can pass by, made once a book: the only steps asked about a doorway
fn near_cells(g: &mut Game) -> Vec<u8> {
    if let Some(n) = &g.m.stackroom.books.as_ref().unwrap().near { return n.clone(); }
    let (nw, nh, cs) = (g.col.nav.w, g.col.nav.h, g.col.nav.cell);
    let mut near = vec![0u8; (nw * nh) as usize];
    for n in 0..g.doors.list.len() {
        let o = g.doors.list[n];
        let r = g.dr[o].def.width / 2.0 + 1.0;
        let c = g.dr[o].frame_centre;
        let mut z = m::max(0.0, m::floor((c[2] - r) / cs));
        while z <= m::min(nh - 1.0, m::floor((c[2] + r) / cs)) {
            let mut x = m::max(0.0, m::floor((c[0] - r) / cs));
            while x <= m::min(nw - 1.0, m::floor((c[0] + r) / cs)) {
                let i = nav_idx(g, x, z) as usize;
                near[i] = 1;
                x += 1.0;
            }
            z += 1.0;
        }
    }
    g.m.stackroom.books.as_mut().unwrap().near = Some(near.clone());
    near
}

/// the floor before face `side` of `d`; None: that face has none
fn floor_of(g: &mut crate::game::Game, d: DoorId, side: f64) -> Option<crate::stackroom::Floor> {
    floor_read(g, d, side);
    let at = if side > 0.0 { 0 } else { 1 };
    g.m.stackroom.books.as_ref().unwrap().floors.get(d).unwrap()[at].as_ref().unwrap().clone()
}

/// floods that face if it has not been read, leaving the answer in the book's row
fn floor_read(g: &mut Game, d: DoorId, side: f64) {
    let at = if side > 0.0 { 0 } else { 1 };
    {
        let k = book_of(g);
        match k.floors.get(d) {
            Some(pair) => { if pair[at].is_some() { return; } }
            None => { k.floors.set(d, [None, None]); }
        }
    }
    let (nw, nh, cs) = (g.col.nav.w, g.col.nav.h, g.col.nav.cell);
    let near = near_cells(g);
    // the cell it is flooded from: the free cell nearest the doorway's front whose middle is on this face, before the opening
    let (mut seed, mut best) = (-1.0, m::INF);
    let w = g.dr[d].def.width;
    let front = crate::doortools::door_pt(g, d, w / 2.0, 0.55, side, None);
    let (fx, fz) = (m::floor(front[0] / cs), m::floor(front[2] / cs));
    for dz in -3..=3 {
        for dx in -3..=3 {
            let (x, z) = (fx + dx as f64, fz + dz as f64);
            if !cell_free(g, x, z) { continue; }
            let i = nav_idx(g, x, z);
            let c = centre(g, i);
            let b = crate::doortools::off_of(g, d, c, side);
            let a = crate::doortools::along_of(g, d, c);
            if b < 0.05 || b > 1.8 || a < -0.3 || a > w + 0.3 { continue; }
            let s = flat(c, front) + (if crate::scene_collision::solid_blocked(g, [front[0], 0.45, front[2]], [c[0], 0.45, c[2]], None) { 10.0 } else { 0.0 });
            if s < best { best = s; seed = i; }
        }
    }
    if seed < 0.0 {
        g.m.stackroom.books.as_mut().unwrap().floors.get_mut(d).unwrap()[at] = Some(None);
        return;
    }
    let mut dist = vec![-1i16; (nw * nh) as usize];
    let mut cells = vec![seed];
    dist[seed as usize] = 0;
    let mut qi = 0;
    while qi < cells.len() {
        let c = cells[qi];
        let (x, z) = (js::rem(c, nw), js::to_i32(c / nw) as f64);
        let here = centre(g, c);
        for (dx, dz) in [(1.0, 0.0), (-1.0, 0.0), (0.0, 1.0), (0.0, -1.0)] {
            let (nx, nz) = (x + dx, z + dz);
            if !cell_free(g, nx, nz) { continue; }
            let n = nav_idx(g, nx, nz);
            if dist[n as usize] >= 0 { continue; }
            if near[c as usize] != 0 || near[n as usize] != 0 {
                let there = centre(g, n);
                if door_between(g, here, there) { continue; }   // (no step through a doorway: this one's or any other's)
            }
            dist[n as usize] = core::cmp::min(32000, dist[c as usize] + 1);
            cells.push(n);
        }
        qi += 1;
    }
    let f = crate::stackroom::Floor { seed, dist, cells };
    g.m.stackroom.books.as_mut().unwrap().floors.get_mut(d).unwrap()[at] = Some(Some(f));
}

/// steps of walking from the doorway to `p` on that floor (−1: not on it)
fn steps_to(g: &mut crate::game::Game, over: &Over, f: &crate::stackroom::Floor, p: V3) -> f64 {
    let (nw, nh, cell) = (g.col.nav.w, g.col.nav.h, g.col.nav.cell);
    let (x, z) = (m::floor(p[0] / cell), m::floor(p[2] / cell));
    if x < 0.0 || z < 0.0 || x >= nw || z >= nh { return -1.0; }
    let own = nav_idx(g, x, z);
    if g.col.nav.blocked[own as usize] == 1 { return -1.0; }
    if f.dist[own as usize] >= 0 {
        let mid = centre(g, own);
        if !door_between(g, mid, p) { return f.dist[own as usize] as f64; }
    }
    let mut best = -1.0;   // (a mark by a frame line: its cell's middle can be the other face's)
    for dz in -1..=1 {
        for dx in -1..=1 {
            let (nx, nz) = (x + dx as f64, z + dz as f64);
            if (dx == 0 && dz == 0) || nx < 0.0 || nz < 0.0 || nx >= nw || nz >= nh { continue; }
            let c = nav_idx(g, nx, nz);
            let here = f.dist[c as usize] as f64;
            if here < 0.0 || (best >= 0.0 && here + 1.0 >= best) { continue; }
            let mid = centre(g, c);
            if door_between(g, mid, p) || crate::scene_collision::solid_blocked_over(g, over, [mid[0], 0.45, mid[2]], [p[0], 0.45, p[2]], None) { continue; }
            best = here + 1.0;
        }
    }
    best
}

/// on the floor, and the walk there no way round a wall
fn on_floor(g: &mut crate::game::Game, f: &crate::stackroom::Floor, p: V3) -> bool { on_floor_over(g, &Over::OWN, f, p) }
/// …of the world as the question takes it
fn on_floor_over(g: &mut Game, over: &Over, f: &crate::stackroom::Floor, p: V3) -> bool {
    let s = steps_to(g, over, f, p);
    if !(s >= 0.0) { return false; }
    let mid = centre(g, f.seed);
    s * g.col.nav.cell <= g.k.stackroom.round_m + g.k.stackroom.round_k * flat(mid, p)
}

/// is `p` on the floor of that face of `d`: true for a face with no floor at all
pub fn stack_room_has(g: &mut crate::game::Game, d: DoorId, side: f64, p: V3) -> bool { stack_room_has_over(g, &Over::OWN, d, side, p) }
/// …of the world as the question takes it (the floor itself is the book's, read of the level as it stands)
pub fn stack_room_has_over(g: &mut Game, over: &Over, d: DoorId, side: f64, p: V3) -> bool {
    floor_read(g, d, side);
    // the face out of its row for the one question, not a copy of its W*H cells: on_floor never reads the book
    let at = if side > 0.0 { 0 } else { 1 };
    let f = g.m.stackroom.books.as_mut().unwrap().floors.get_mut(d).unwrap()[at].take();
    let ok = match &f { Some(Some(f)) => on_floor_over(g, over, f, p), _ => true };
    g.m.stackroom.books.as_mut().unwrap().floors.get_mut(d).unwrap()[at] = f;
    ok
}

/// … the same, counted: a tuck or a step back not taken for it
pub(super) fn stack_room_keeps(g: &mut crate::game::Game, d: DoorId, side: f64, p: V3) -> bool {
    let ok = stack_room_has(g, d, side, p);
    ok
}

/// is `q` the mark a man of the squad already stands on at this door
fn stood_at(g: &crate::game::Game, seat: &crate::seats::SeatRef, d: DoorId, q: V3) -> bool {
    for man in crate::orders::squad_orders_squad(g, seat) {
        let a = crate::orders::squad_orders_of(seat, man);
        let Some(a) = a else { continue };
        if crate::condition::down(g, g.gd[man].ch) { continue; }
        if g.tab.orders_assignment[a].door != Some(d) { continue; }
        let Some(post) = g.tab.orders_assignment[a].post else { continue };
        if flat(post, q) < SAME_M && flat(g.ch[g.gd[man].ch].pos, post) < 0.6 { return true; }
    }
    false
}

/// a place a piece of furniture stands on, unless a man of the squad already stands on it at this door
fn furnished(g: &mut crate::game::Game, d: DoorId, q: V3) -> bool {
    crate::orders::squad_orders_on_furniture(g, q) && !stood_at(g, &crate::seats::here(g), d, q)
}

// ---------------------------------------------------------------- THE RULE

/// nothing solid between two places, at knee height
fn sees(g: &mut Game, a: V3, q: V3) -> bool { !crate::scene_collision::solid_blocked(g, [a[0], 0.45, a[2]], [q[0], 0.45, q[2]], None) }
/// off the opening's own lane
fn off_lane(g: &Game, d: DoorId, w: f64, q: V3) -> bool { m::abs(crate::doortools::along_of(g, d, q) - w / 2.0) >= w / 2.0 + 0.25 }
/// the hand of the opening a place is on
fn hand_of(g: &Game, d: DoorId, w: f64, q: V3) -> f64 { if crate::doortools::along_of(g, d, q) > w / 2.0 { 1.0 } else { -1.0 } }
/// a place along the door's wall: faced down the wall at the doorway, and the shoulder the wall is on (face, stackSide)
fn on_wall(th: f64, side: f64, hand: f64) -> (f64, f64) {
    let (fx, fz, nx, nz) = (-m::cos(th) * hand, -m::sin(th) * hand, -m::sin(th) * side, m::cos(th) * side);
    (m::atan2(fx, fz), if nx * fz - nz * fx >= 0.0 { 1.0 } else { -1.0 })
}
/// a cell's middle not up against the furniture
fn clear_of_props(g: &Game, q: V3) -> bool {
    for n in 0..g.props.props.len() {
        let pr = g.props.props[n];
        let p = &g.tab.props_prop[pr];
        let gap = crate::props::prop_gap(p.home, p.hx, p.hz, q);
        if gap < 0.3 { return false; }
    }
    true
}

/// what `fits` of stackRoomFix closed over
struct FixFit<'a> { d: DoorId, side: f64, f: &'a crate::stackroom::Floor, c: V3, min_m: f64, used: &'a [V3], glass: bool, shy: bool }
/// Is this place another door's: before its leaf, or within the blast a charge on it throws back.
fn anothers(g: &Game, d: DoorId, q: V3) -> bool { crate::doorway::before_another_door(g, q, d) || crate::doortools::in_anothers_blast(g, &crate::seats::here(g), d, q) }

/// a place a mark may be dealt
fn fix_fits(g: &mut Game, x: &FixFit, q: V3, apart: f64) -> bool {
    on_floor(g, x.f, q)
        && !anothers(g, x.d, q)
        && flat(q, x.c) >= x.min_m
        && !x.used.iter().any(|o| flat(*o, q) < apart)
        && !crate::doorway::doorway_in_way(g, q)
        && !(x.glass && crate::orders::squad_orders_behind_glass(g, x.d, x.side, q))
        && !(x.shy && furnished(g, x.d, q))
}

/// what a mark dealt again takes of its new place (Partial<StackPost>): no stackSide = its own stays; group always, none included
struct Got { pos: V3, face: f64, stack_side: Option<f64>, group: Option<f64> }

/// every post not on the floor of that face, or at another door, is dealt again, the rest stay; None: one has no room at all
pub fn stack_room_fix(g: &mut crate::game::Game, d: DoorId, side: f64, posts: Option<&[crate::orders::StackPost]>, taken: &[V3], min_m: Option<f64>) -> Option<Vec<crate::orders::StackPost>> {
    let min_m = min_m.unwrap_or(0.0);
    let posts = posts?;
    let Some(f) = floor_of(g, d, side) else { return Some(posts.to_vec()) };
    let s = g.k.stackroom;
    let mut bad = Vec::with_capacity(posts.len());
    for p in posts { bad.push(!on_floor(g, &f, p.pos) || anothers(g, d, p.pos)); }
    if !bad.iter().any(|b| *b) { return Some(posts.to_vec()); }
    let mut out = posts.to_vec();
    let (w, c, th) = (g.dr[d].def.width, g.dr[d].frame_centre, g.dr[d].def.closed_dir);
    let (jamb_out, jamb_past, file_gap) = (g.orders.knobs.jamb_out, g.orders.knobs.jamb_past, g.orders.knobs.file_gap);
    let cell = g.col.nav.cell;
    let front = crate::doortools::door_pt(g, d, w / 2.0, jamb_out, side, Some(0.45));
    for i in 0..out.len() {
        if !bad[i] { continue; }
        let pp = out[i];
        let mut used: Vec<V3> = taken.to_vec();
        for j in 0..out.len() { if j != i && !bad[j] { used.push(out[j].pos); } }
        let hand = hand_of(g, d, w, pp.pos);
        let mut got: Option<Got> = None;
        // GLASS IS NOT COVER: first with the places a round reaches through a pane counted as no floor; and each of those first with the places furniture stands on left to it
        for (glass, shy) in [(true, true), (true, false), (false, true), (false, false)] {
            if got.is_some() { break; }
            let x = FixFit { d, side, f: &f, c, min_m, used: &used, glass, shy };
            // 1 the other jamb's wall, the next place past whoever stands there
            for k in 0..4 {
                if got.is_some() { break; }
                let k = k as f64;
                let a = if hand > 0.0 { -jamb_past - k * file_gap } else { w + jamb_past + k * file_gap };
                let p = crate::doortools::door_pt(g, d, a, jamb_out, side, None);
                if nav_blocked(g, p[0], p[2]) || !sees(g, front, p) { break; }   // (the wall, or the floor, runs out)
                if fix_fits(g, &x, p, s.apart) {
                    let (face, stack_side) = on_wall(th, side, -hand);
                    got = Some(Got { pos: p, face, stack_side: Some(stack_side), group: if pp.group.is_none() { None } else if hand > 0.0 { Some(1.0) } else { Some(0.0) } });
                }
            }
            // 2 the file turns the corner: behind the nearest man of his own file, straight off the door's wall
            if got.is_none() {
                let mut file: Vec<crate::orders::StackPost> = Vec::new();
                for j in 0..out.len() { if j != i && !bad[j] && hand_of(g, d, w, out[j].pos) == hand { file.push(out[j]); } }
                js::sort_by(&mut file, |a, b| flat(a.pos, pp.pos) - flat(b.pos, pp.pos));
                for mm in file {
                    let mut prev = mm.pos;
                    for k in 1..=3 {
                        if got.is_some() { break; }
                        let along = crate::doortools::along_of(g, d, mm.pos);
                        let off = crate::doortools::off_of(g, d, mm.pos, side);
                        let p = crate::doortools::door_pt(g, d, along, off + k as f64 * file_gap, side, None);
                        if nav_blocked(g, p[0], p[2]) || !sees(g, prev, p) { break; }
                        prev = p;
                        if fix_fits(g, &x, p, s.apart) && off_lane(g, d, w, p) {
                            got = Some(Got { pos: p, face: v3::yaw_to(p, c), stack_side: Some(mm.stack_side), group: mm.group });
                        }
                    }
                    if got.is_some() { break; }
                }
            }
            // 3 back from the door, to a hand of it: not the man who has the door to open
            if got.is_none() && !pp.lead {
                for r in crate::orders::squad_orders_rear_posts(g, d, side, 6.0, &[]) {
                    if fix_fits(g, &x, r.pos, s.apart) {
                        got = Some(Got { pos: r.pos, face: r.face, stack_side: Some(r.stack_side), group: r.group });
                        break;
                    }
                }
            }
            // 4 any floor of the face within a walk, the nearest with a line to the doorway first
            if got.is_none() {
                let (mut bs, mut q) = (m::INF, None);
                for &cc in &f.cells {
                    let steps = f.dist[cc as usize] as f64 * cell;
                    if steps > s.walk_m || steps >= bs { break; }
                    let p = centre(g, cc);
                    if crate::doortools::off_of(g, d, p, side) < jamb_out - 0.1 || !off_lane(g, d, w, p) || !fix_fits(g, &x, p, s.apart) || !clear_of_props(g, p) { continue; }
                    let score = steps + (if sees(g, front, p) { 0.0 } else { 3.0 });
                    if score < bs { bs = score; q = Some(p); }
                }
                if let Some(q) = q {
                    got = Some(Got { pos: q, face: v3::yaw_to(q, c), stack_side: None, group: if pp.group.is_none() { None } else { Some(2.0) } });
                }
            }
            // 5 the room is too small: by a doorway of a room next to it that the team already holds, never the man who has the door to open
            if got.is_none() && !pp.lead && !glass {
                if let Some(n) = next_room(g, d, side, &f, &used, min_m, shy) {
                    got = Some(Got { pos: n.pos, face: n.face, stack_side: None, group: if pp.group.is_none() { None } else { Some(2.0) } });
                }
            }
            // 6 anywhere on the face's floor, off the opening's lane, a body fits
            if got.is_none() && !glass {
                for &cc in &f.cells {
                    let p = centre(g, cc);
                    if off_lane(g, d, w, p) && fix_fits(g, &x, p, s.squeeze) && clear_of_props(g, p) {
                        got = Some(Got { pos: p, face: v3::yaw_to(p, c), stack_side: None, group: if pp.group.is_none() { None } else { Some(2.0) } });
                        break;
                    }
                }
            }
            // 7 a passage no wider than its door: down it in file, stood back past where a leaf swings, the nearest first
            if got.is_none() && !glass {
                for apart in [s.apart, s.squeeze] {
                    if got.is_some() { break; }
                    for &cc in &f.cells {
                        if f.dist[cc as usize] as f64 * cell > s.walk_m { break; }
                        let p = centre(g, cc);
                        if crate::doortools::off_of(g, d, p, side) >= s.lane_back && fix_fits(g, &x, p, apart) && clear_of_props(g, p) {
                            got = Some(Got { pos: p, face: v3::yaw_to(p, c), stack_side: None, group: if pp.group.is_none() { None } else { Some(2.0) } });
                            break;
                        }
                    }
                }
            }
        }
        let Some(got) = got else {
            if on_floor(g, &f, pp.pos) { continue; }   
            return None;
        };
        // the door has ONE opener: with a man still on the other jamb the handle is that man's
        let lead = pp.lead && !out.iter().enumerate().any(|(j, o)| j != i && !bad[j] && o.lead);
        out[i] = crate::orders::StackPost { pos: got.pos, face: got.face, stack_side: got.stack_side.unwrap_or(pp.stack_side), lead, group: got.group };
        bad[i] = false;
    }
    Some(out)
}

// ---------------------------------------------------------------- OUT OF THE ROOM'S VIEW

/// is `p` (on face `side`) in view of the floor beyond the door through its opening
fn in_view(g: &mut crate::game::Game, d: DoorId, far: &[V3], p: V3) -> bool {
    let (eye, need) = (g.k.stackroom.view_eye, g.k.stackroom.view_cells);
    let pp = [p[0], eye, p[2]];
    let mut n = 0.0;
    for q in far {
        if crate::guards::crosses_door(g, *q, p, d).is_some() && !crate::scene_collision::segment_blocked(g, [q[0], eye, q[2]], pp, None, None, None) {
            n += 1.0;
            if n >= need { return true; }
        }
    }
    false
}

/// the nav cells within `viewFar` of the doorway on the face the men are NOT on
fn far_cells(g: &crate::game::Game, d: DoorId, side: f64) -> Vec<V3> {
    let view_far = g.k.stackroom.view_far;
    let cell = g.col.nav.cell;
    let c = g.dr[d].frame_centre;
    let n = m::ceil(view_far / cell);
    let (cx, cz) = (m::floor(c[0] / cell), m::floor(c[2] / cell));
    let mut l = Vec::new();
    let mut dz = -n;
    while dz <= n {
        let mut dx = -n;
        while dx <= n {
            let q = [(cx + dx + 0.5) * cell, 0.0, (cz + dz + 0.5) * cell];
            if flat(q, c) <= view_far && !nav_blocked(g, q[0], q[2]) {
                let face = if crate::doortools::off_of(g, d, q, 1.0) >= 0.0 { 1.0 } else { -1.0 };
                if face != side { l.push(q); }
            }
            dx += 1.0;
        }
        dz += 1.0;
    }
    l
}

/// the leaf stands open: off its hinges, or unlatched and swung
pub fn stack_view_open(g: &crate::game::Game, d: DoorId) -> bool {
    if crate::doors::door_off(g, d) { return true; }
    if crate::doors::door_latched(g, d) { return false; }
    m::abs(crate::doors::door_angle(g, d)) >= g.k.stackroom.view_ajar
}

/// how far from the frame's centre a plain stack's mark on this door can be
pub(super) fn stack_view_reach(g: &crate::game::Game, d: DoorId) -> f64 {
    if stack_view_open(g, d) { m::max(2.6, g.dr[d].def.width / 2.0 + g.k.stackroom.view_past + 0.7) } else { 2.6 }
}

/// what the closures of stackViewFix closed over, and its memo of each hand's places
struct ViewDeal<'a> {
    d: DoorId, side: f64, w: f64, jamb_out: f64, jamb_past: f64, front: V3,
    f: &'a crate::stackroom::Floor, far: &'a [V3], taken: &'a [V3], keeps: &'a [Vec<crate::rearwatch::RearWaysItem>],
    posts: &'a [crate::orders::StackPost], seen: &'a [bool], across: &'a [bool],
    cands: [Option<Vec<(V3, bool)>>; 2],
}
/// a place dealt: where, on which hand, on the other hand from the mark as dealt
#[derive(Clone, Copy)]
struct Dealt { q: V3, h: f64, over: bool }

/// the places along hand `h` of the door's wall the floor beyond does not see, from the jamb out, made once a hand: (place, behind glass)
fn cands_of(g: &mut Game, r: &mut ViewDeal, h: f64) {
    let at = if h > 0.0 { 0 } else { 1 };
    if r.cands[at].is_some() { return; }
    let s = g.k.stackroom;
    let (d, side, w, jamb_out) = (r.d, r.side, r.w, r.jamb_out);
    let mut l = Vec::new();
    let mut past = r.jamb_past;
    while past <= s.view_past + 1e-6 {
        let a = if h > 0.0 { w + past } else { -past };
        // the door's own wall must stand behind the place: where it ends, or another doorway opens in it, the file ends too
        let p0 = crate::doortools::door_pt(g, d, a, jamb_out, side, Some(0.45));
        let p1 = crate::doortools::door_pt(g, d, a, -s.view_wall, side, Some(0.45));
        if !crate::scene_collision::solid_blocked(g, p0, p1, None) { break; }
        for off in [jamb_out, s.view_off] {
            let q = crate::doortools::door_pt(g, d, a, off, side, None);
            if !nav_blocked(g, q[0], q[2]) && on_floor(g, r.f, q) && !crate::scene_collision::solid_blocked(g, r.front, [q[0], 0.45, q[2]], None) && !crate::doorway::doorway_in_way(g, q)
                && !r.taken.iter().any(|o| flat(*o, q) < s.apart) && !in_view(g, d, r.far, q) && !furnished(g, d, q) {
                let glass = crate::orders::squad_orders_behind_glass(g, d, side, q);
                l.push((q, glass));
            }
        }
        past += s.view_step;
    }
    r.cands[at] = Some(l);
}

/// the first such place on hand `h` a body clear of every place that stands, not behind glass while another will do
fn view_place(g: &mut Game, r: &mut ViewDeal, i: usize, h: f64, fixed: &[V3]) -> Option<V3> {
    cands_of(g, r, h);
    let view_apart = g.k.stackroom.view_apart;
    let list = r.cands[if h > 0.0 { 0 } else { 1 }].as_ref().unwrap();
    for shy in [true, false] {
        for &(q, glass) in list {
            if shy && glass { continue; }
            if fixed.iter().any(|o| flat(*o, q) < view_apart) { continue; }
            let mut keeps = true;
            for way in &r.keeps[i] { if crate::rearwatch::rear_sees(g, q, way.o, way.front).is_none() { keeps = false; break; } }
            if keeps { return Some(q); }
        }
    }
    None
}

/// a place for post `i`: across the opening first if it is across, else its own hand if it is in view
fn view_deal(g: &mut Game, r: &mut ViewDeal, i: usize, fixed: &[V3]) -> Option<Dealt> {
    let h0 = hand_of(g, r.d, r.w, r.posts[i].pos);
    let mut h = -h0;
    let mut q = if r.across[i] { view_place(g, r, i, h, fixed) } else { None };
    if q.is_none() && r.seen[i] {
        h = h0;
        q = view_place(g, r, i, h, fixed);
    }
    q.map(|q| Dealt { q, h, over: h != h0 })
}

/// a plain 'stack up' on a door standing open: the marks in view, and those across the opening from `men`, dealt again along the door's wall
pub fn stack_view_fix(g: &mut crate::game::Game, d: DoorId, side: f64, posts: Option<&[crate::orders::StackPost]>, taken: &[V3], men: &[GuardId]) -> Option<Vec<crate::orders::StackPost>> {
    let posts = posts?;
    let s = g.k.stackroom;
    if !stack_view_open(g, d) { return Some(posts.to_vec()); }
    let Some(f) = floor_of(g, d, side) else { return Some(posts.to_vec()) };
    let (jamb_out, jamb_past) = (g.orders.knobs.jamb_out, g.orders.knobs.jamb_past);
    let (w, c, th) = (g.dr[d].def.width, g.dr[d].frame_centre, g.dr[d].def.closed_dir);
    let far = far_cells(g, d, side);
    let front = crate::doortools::door_pt(g, d, w / 2.0, jamb_out, side, Some(0.45));
    // A STACK THAT STANDS DOES NOT CREEP: a place a man of this order already stands on, no mark as dealt now, is his while it still stands as a place
    let mut pre: JsMap<usize, V3> = JsMap::new();
    {
        let mut stood: Vec<V3> = Vec::new();
        for &man in men {
            let a = crate::orders::squad_orders_of(&crate::seats::here(g), man);
            let Some(a) = a else { continue };
            if !crate::orders::rows::is(g, a, crate::orders::Is::Stacks) || g.tab.orders_assignment[a].door != Some(d) { continue; }
            let Some(post) = g.tab.orders_assignment[a].post else { continue };
            if flat(g.ch[g.gd[man].ch].pos, post) < 0.6 && crate::doortools::off_of(g, d, post, side) > 0.0 { stood.push(post); }
        }
        let loose: Vec<usize> = (0..posts.len()).filter(|i| !stood.iter().any(|q| flat(*q, posts[*i].pos) < SAME_M)).collect();
        // only a place this rule could have dealt: by the door's own wall, no further along it than the rule reaches
        let mut mine: Vec<V3> = Vec::new();
        for &q in &stood {
            if crate::doortools::off_of(g, d, q, side) < jamb_out + 0.5
                && m::abs(crate::doortools::along_of(g, d, q) - w / 2.0) <= w / 2.0 + s.view_past + 0.05
                && !posts.iter().any(|pp| flat(pp.pos, q) < SAME_M) {
                mine.push(q);
            }
        }
        js::sort_by(&mut mine, |p, q| flat(*p, c) - flat(*q, c));
        if !loose.is_empty() {
            for &q in &mine {
                if nav_blocked(g, q[0], q[2]) || !on_floor(g, &f, q) || crate::scene_collision::solid_blocked(g, front, [q[0], 0.45, q[2]], None) || crate::doorway::doorway_in_way(g, q)
                    || taken.iter().any(|o| flat(*o, q) < s.apart) || in_view(g, d, &far, q) { continue; }
                let stands = (0..posts.len()).filter(|i| !loose.contains(i)).any(|i| flat(posts[i].pos, q) < s.apart);
                if stands || pre.values().any(|o| flat(*o, q) < s.apart) { continue; }
                let h = hand_of(g, d, w, q);
                let mut free: Vec<(usize, f64)> = Vec::new();
                for &i in &loose { if !pre.has(i) { free.push((i, if hand_of(g, d, w, posts[i].pos) == h { 0.0 } else { 1.0 })); } }
                js::sort_by(&mut free, |x, y| js::or(x.1 - y.1, flat(posts[x.0].pos, q) - flat(posts[y.0].pos, q)));
                let Some(&(u, _)) = free.first() else { break };
                // nor a body's width from a dealt mark that will stand where it is
                let mut crowded = false;
                for &j in &loose {
                    if j != u && !pre.has(j) && flat(posts[j].pos, q) < s.apart && !in_view(g, d, &far, posts[j].pos) { crowded = true; break; }
                }
                if crowded { continue; }
                pre.set(u, q);
            }
        }
    }
    let at = |i: usize| -> V3 { match pre.get(i) { Some(q) => *q, None => posts[i].pos } };
    let mut seen = Vec::with_capacity(posts.len());
    for i in 0..posts.len() { seen.push(!pre.has(i) && in_view(g, d, &far, posts[i].pos)); }
    let mut across = vec![false; posts.len()];
    // ACROSS: a hand dealt more places than it has men to take them without crossing gives up the ones nearest the frame
    {
        let mut hs: Vec<f64> = Vec::new();
        for &man in men {
            let p = g.ch[g.gd[man].ch].pos;
            if crate::doortools::off_of(g, d, p, side) > 0.0 && flat(p, c) <= s.view_men {
                let a = crate::doortools::along_of(g, d, p);
                hs.push(if a >= w + s.view_out { 1.0 } else if a <= -s.view_out { -1.0 } else { 0.0 });
            }
        }
        for h in [1.0, -1.0] {
            let room = men.len() as i32 - hs.iter().filter(|x| **x == -h).count() as i32;
            let mut mine: Vec<usize> = Vec::new();
            for i in 0..posts.len() {
                if hand_of(g, d, w, at(i)) == h && m::abs(crate::doortools::along_of(g, d, at(i)) - w / 2.0) < w / 2.0 + s.view_past + 1.0 { mine.push(i); }
            }
            js::sort_by(&mut mine, |a, b| flat(at(*a), c) - flat(at(*b), c));
            let mut over = mine.len() as i32 - room;
            for &i in &mine {
                if over <= 0 { break; }
                if pre.has(i) { continue; }
                across[i] = true;
                over -= 1;
            }
        }
    }
    g.stats.stackroom.view_in += seen.iter().filter(|b| **b).count() as i32;
    g.stats.stackroom.view_across += across.iter().filter(|b| **b).count() as i32;
    if !seen.iter().any(|b| *b) && !across.iter().any(|b| *b) && pre.is_empty() { return Some(posts.to_vec()); }
    // the ways at the stack's back as the marks stand: a place that has a line to one keeps a line to it
    let marks: Vec<V3> = posts.iter().map(|pp| pp.pos).collect();
    let ways = crate::rearwatch::rear_ways(g, d, side, &marks);
    let mut keeps: Vec<Vec<crate::rearwatch::RearWaysItem>> = Vec::with_capacity(posts.len());
    for pp in posts {
        let mut l = Vec::new();
        for way in &ways { if crate::rearwatch::rear_sees(g, pp.pos, way.o, way.front).is_some() { l.push(*way); } }
        keeps.push(l);
    }
    let moved: Vec<bool> = (0..posts.len()).map(|i| seen[i] || across[i]).collect();
    let mut r = ViewDeal { d, side, w, jamb_out, jamb_past, front, f: &f, far: &far, taken, keeps: &keeps, posts, seen: &seen, across: &across, cands: [None, None] };
    // dealt nearest the frame first. A place with nowhere to go STAYS and stands in the others' way: the deal is run again with it standing
    let mut order: Vec<usize> = (0..posts.len()).filter(|i| moved[*i]).collect();
    js::sort_by(&mut order, |a, b| flat(posts[*a].pos, c) - flat(posts[*b].pos, c));
    let mut stay: JsSet<usize> = JsSet::new();
    let mut got: JsMap<usize, Dealt>;
    loop {
        let mut fixed: Vec<V3> = (0..posts.len()).filter(|i| !moved[*i] || stay.has(*i)).map(|i| at(i)).collect();
        let mut failed: Option<usize> = None;
        got = JsMap::new();
        for (i, q) in pre.iter() {
            let h = hand_of(g, d, w, *q);
            let h0 = hand_of(g, d, w, posts[i].pos);
            got.set(i, Dealt { q: *q, h, over: h != h0 });
        }
        for &i in &order {
            if stay.has(i) { continue; }
            match view_deal(g, &mut r, i, &fixed) {
                None => { failed = Some(i); break; }
                Some(dealt) => { fixed.push(dealt.q); got.set(i, dealt); }
            }
        }
        match failed { None => break, Some(i) => { stay.add(i); } }
    }
    // every place that stayed is asked again with all the others where they ended up, nearest the frame first, until none of them moves
    let mut again = !stay.is_empty();
    while again {
        again = false;
        for &i in &order {
            if !stay.has(i) { continue; }
            let others: Vec<V3> = (0..posts.len()).filter(|j| *j != i).map(|j| match got.get(j) { Some(dealt) => dealt.q, None => at(j) }).collect();
            if let Some(dealt) = view_deal(g, &mut r, i, &others) {
                stay.delete(i);
                got.set(i, dealt);
                again = true;
                break;
            }
        }
    }
    if got.is_empty() { return Some(posts.to_vec()); }
    let mut out = posts.to_vec();
    let mut led = false;
    for (i, dealt) in got.iter() {
        let pp = posts[i];
        let h = dealt.h;
        if dealt.over {  } else {  }
        let (face, stack_side) = on_wall(th, side, h);   // (as a wall mark is faced: down the wall at the doorway)
        // a place given to the other hand joins that hand's file: 0 past the far jamb, 1 before the hinge
        let group = if dealt.over && pp.group.is_some() && pp.group != Some(2.0) { Some(if h > 0.0 { 0.0 } else { 1.0 }) } else { pp.group };
        out[i] = crate::orders::StackPost { pos: dealt.q, face, stack_side, group, ..pp };
        led = led || pp.lead;
    }
    // the door has one opener, the man nearest its frame
    if led {
        let mut best = 0;
        for i in 1..out.len() { if flat(out[i].pos, c) < flat(out[best].pos, c) { best = i; } }
        for i in 0..out.len() { out[i].lead = i == best; }
    }
    Some(out)
}

/// a way through to a held room next door
struct Way { o: DoorId, far: f64, floor: crate::stackroom::Floor, cost: f64 }

/// (5) a place by a doorway of a room next to the face's own, one the team holds, on that doorway's far face
fn next_room(g: &mut crate::game::Game, d: DoorId, side: f64, f: &crate::stackroom::Floor, used: &[V3], min_m: f64, shy: bool) -> Option<crate::stackroom::NextRoom> {
    let s = g.k.stackroom;
    let (jamb_out, jamb_past, file_gap) = (g.orders.knobs.jamb_out, g.orders.knobs.jamb_past, g.orders.knobs.file_gap);
    let cs = g.col.nav.cell;
    let into = floor_of(g, d, -side);
    let mut ways: Vec<Way> = Vec::new();
    let pl = g.player;
    let ring: JsSet<String> = {
        let mut ring = g.pl[pl].keys.clone();
        for code in g.pl[pl].codes.keys() { ring.add(code); }
        ring
    };
    let mut ours: Vec<V3> = Vec::new();
    if !crate::condition::down(g, g.pl[pl].ch) { ours.push(g.ch[g.pl[pl].ch].pos); }
    for man in crate::orders::squad_orders_squad(g, &crate::seats::here(g)) {
        if !crate::condition::down(g, g.gd[man].ch) { ours.push(g.ch[g.gd[man].ch].pos); }
    }
    for n in 0..g.doors.list.len() {
        let o = g.doors.list[n];
        if o == d { continue; }
        for s_o in [1.0, -1.0] {
            let near = floor_of(g, o, s_o);
            let far = -s_o;
            let other = floor_of(g, o, far);
            let (Some(near), Some(other)) = (near, other) else { continue };
            if f.dist[near.seed as usize] < 0 { continue; }
            let walk = f.dist[near.seed as usize] as f64 * cs;
            if walk > s.next_walk_m { continue; }
            // its far face is this very floor, or the room about to be entered
            if other.dist[f.seed as usize] >= 0 || into.as_ref().map_or(false, |i| other.dist[i.seed as usize] >= 0) { continue; }
            let room = if crate::cleared::cleared_rooms_has_floor(g, o, far) { crate::cleared::cleared_rooms_room_by(g, o, far) } else { -1.0 };
            if room < 0.0 { continue; }
            let unheld = if js::truthy(room) { !crate::cleared::cleared_rooms_is_known(g, room) } else { !ours.iter().any(|p| on_floor(g, &other, *p)) };   // (held: one of ours has stood in it)
            if unheld { continue; }
            let latched = crate::doors::door_latched(g, o);
            if latched {
                let width = g.dr[o].def.width;
                let at = crate::doortools::door_pt(g, o, width / 2.0, 0.6, s_o, None);
                if crate::doors::doors_locked_out(g, o, at, Some(&ring)) { continue; }
            }
            ways.push(Way { o, far, floor: other, cost: walk + (if latched { 6.0 } else { 0.0 }) });
        }
    }
    js::sort_by(&mut ways, |a, b| a.cost - b.cost);
    let d_c = g.dr[d].frame_centre;
    for way in &ways {
        let (o, far) = (way.o, way.far);
        let (w, c) = (g.dr[o].def.width, g.dr[o].frame_centre);
        let front = crate::doortools::door_pt(g, o, w / 2.0, jamb_out, far, Some(0.45));
        for k in 0..3 {
            let k = k as f64;
            for a in [w + jamb_past + k * file_gap, -jamb_past - k * file_gap] {
                let q = crate::doortools::door_pt(g, o, a, jamb_out, far, None);
                if next_fits(g, d, &way.floor, c, d_c, front, used, min_m, shy, q) { return Some(crate::stackroom::NextRoom { pos: q, face: v3::yaw_to(q, c) }); }
            }
        }
        for back in [1.6, 2.5] {
            for lat in [w / 2.0 + 0.8, -w / 2.0 - 0.8, w / 2.0 + 1.5, -w / 2.0 - 1.5] {
                let q = crate::doortools::door_pt(g, o, w / 2.0 + lat, back, far, None);
                if next_fits(g, d, &way.floor, c, d_c, front, used, min_m, shy, q) { return Some(crate::stackroom::NextRoom { pos: q, face: v3::yaw_to(q, c) }); }
            }
        }
    }
    None
}

/// a place by that doorway a mark may be dealt
fn next_fits(g: &mut Game, d: DoorId, floor: &crate::stackroom::Floor, c: V3, d_c: V3, front: V3, used: &[V3], min_m: f64, shy: bool, q: V3) -> bool {
    let s = g.k.stackroom;
    on_floor(g, floor, q)
        && flat(q, c) <= s.next_m
        && flat(q, d_c) >= min_m
        && !used.iter().any(|u| flat(*u, q) < s.apart)
        && !crate::doorway::doorway_in_way(g, q)
        && !crate::scene_collision::solid_blocked(g, front, [q[0], 0.45, q[2]], None)
        && !(shy && furnished(g, d, q))
}

/// the floor of a face as the rule reads it (rearwatch.ts asks it, as do the QA tools)
pub fn stack_room_probe(g: &mut crate::game::Game, d: DoorId, side: f64) -> crate::stackroom::StackRoomProbe {
    let f = floor_of(g, d, side);
    crate::stackroom::StackRoomProbe {
        floor: f.is_some(),
    }
}

#[cfg(test)]
mod tests {
    use super::{book_of, floor_of, on_wall, stack_room_book_keep, stack_room_book_put_back, stack_room_has};
    use crate::game::Game;
    use crate::ids::DoorId;
    use crate::stackroom::{Floor, StackRoomBookKeep};
    use alloc::vec;

    /// A face found to have no floor is remembered, and no floor is flooded for it again.
    #[test]
    fn a_face_with_no_floor_is_remembered() {
        let mut g = Game::blank();
        let d = DoorId(0);
        book_of(&mut g).floors.set(d, [Some(None), None]);
        assert!(floor_of(&mut g, d, 1.0).is_none());
    }

    /// The answer is the row, and a grid that moved starts a new book.
    #[test]
    fn book_of_is_the_row_until_the_grid_moves() {
        let mut g = Game::blank();
        book_of(&mut g).near = Some(vec![1]);
        assert!(g.m.stackroom.books.as_ref().unwrap().near.is_some());
        assert!(book_of(&mut g).near.is_some());
        g.engine.world.geom_serial += 1.0;
        assert!(book_of(&mut g).near.is_none());
    }

    /// No book keeps nothing; a read face is marked, one found to have no floor too, an unread one not.
    #[test]
    fn the_keep_marks_what_has_been_read() {
        let mut g = Game::blank();
        assert!(matches!(stack_room_book_keep(&mut g), StackRoomBookKeep::Forget));
        let d = DoorId(0);
        book_of(&mut g).floors.set(d, [Some(None), None]);
        match stack_room_book_keep(&mut g) {
            StackRoomBookKeep::Back { near, floors, .. } => {
                assert!(!near);
                assert_eq!(floors.get(d), Some(&[true, false]));
            }
            StackRoomBookKeep::Forget => panic!("there is a book"),
        }
    }

    fn a_floor() -> Floor { Floor { seed: 0.0, dist: vec![-1], cells: vec![0.0] } }

    /// No book when kept, none after.
    #[test]
    fn put_back_of_no_book_drops_the_book() {
        let mut g = Game::blank();
        let keep = stack_room_book_keep(&mut g);
        book_of(&mut g).near = Some(vec![1]);
        stack_room_book_put_back(&mut g, keep);
        assert!(g.m.stackroom.books.is_none());
    }

    /// What was read stays, what was read since is forgotten: a face, a door, the doorway cells.
    #[test]
    fn put_back_forgets_only_what_was_read_since() {
        let mut g = Game::blank();
        let (d0, d1) = (DoorId(0), DoorId(1));
        book_of(&mut g).floors.set(d0, [Some(Some(a_floor())), None]);
        let keep = stack_room_book_keep(&mut g);
        let k = book_of(&mut g);
        k.near = Some(vec![1]);
        k.floors.get_mut(d0).unwrap()[1] = Some(None);
        k.floors.set(d1, [Some(None), None]);
        stack_room_book_put_back(&mut g, keep);
        let k = g.m.stackroom.books.as_ref().unwrap();
        assert!(k.near.is_none());
        assert!(k.floors.get(d1).is_none());
        let pair = k.floors.get(d0).unwrap();
        assert!(matches!(&pair[0], Some(Some(_))));
        assert!(pair[1].is_none());
    }

    /// Doorway cells made before the keep stay made.
    #[test]
    fn put_back_keeps_doorway_cells_already_made() {
        let mut g = Game::blank();
        book_of(&mut g).near = Some(vec![1]);
        let keep = stack_room_book_keep(&mut g);
        stack_room_book_put_back(&mut g, keep);
        assert!(g.m.stackroom.books.as_ref().unwrap().near.is_some());
    }

    /// Rebuilt during the ask, so the book kept would be stale at the next look.
    #[test]
    fn put_back_after_a_rebuild_drops_the_book() {
        let mut g = Game::blank();
        book_of(&mut g);
        let keep = stack_room_book_keep(&mut g);
        g.engine.world.geom_serial += 1.0;
        book_of(&mut g);
        stack_room_book_put_back(&mut g, keep);
        assert!(g.m.stackroom.books.is_none());
    }

    /// No floor is true, and what is remembered of it is still in its row after the ask.
    #[test]
    fn has_with_no_floor_is_true_and_leaves_the_row() {
        let mut g = Game::blank();
        let d = DoorId(0);
        book_of(&mut g).floors.set(d, [Some(None), None]);
        assert!(stack_room_has(&mut g, d, 1.0, [0.0; 3]));
        assert!(matches!(&g.m.stackroom.books.as_ref().unwrap().floors.get(d).unwrap()[0], Some(None)));
    }

    /// A point off the grid is off the floor, and the floor taken out for the question is back in its row.
    #[test]
    fn has_puts_the_floor_back_after_the_question() {
        let mut g = Game::blank();
        g.col.nav.w = 1.0;
        g.col.nav.h = 1.0;
        g.col.nav.cell = 1.0;
        let d = DoorId(0);
        book_of(&mut g).floors.set(d, [None, Some(Some(a_floor()))]);
        assert!(!stack_room_has(&mut g, d, -1.0, [-5.0, 0.0, 0.0]));
        assert!(matches!(&g.m.stackroom.books.as_ref().unwrap().floors.get(d).unwrap()[1], Some(Some(_))));
    }

    /// `hand` is the side of the doorway he stands (+1 past the latch), `side` the face of the wall. A failure means a man on a wall mark looks away from the doorway, or has the wall on his gun's shoulder.
    #[test]
    fn a_man_on_the_door_s_wall_looks_down_it_at_the_doorway() {
        for th in [0.0, 0.7, -2.3, 3.141592653589793] {
            for (side, hand) in [(1.0, 1.0), (1.0, -1.0), (-1.0, 1.0), (-1.0, -1.0)] {
                let (face, stack_side) = on_wall(th, side, hand);
                let (fx, fz) = (super::m::sin(face), super::m::cos(face));
                let (ux, uz) = (super::m::cos(th) * hand, super::m::sin(th) * hand);
                assert!((fx + ux).abs() < 1e-12 && (fz + uz).abs() < 1e-12, "he faces back along the door's line, at {th} {side} {hand}");
                assert_eq!(stack_side, side * hand, "the shoulder turns over with the hand and with the face, however the door lies: {th}");
            }
        }
    }
}
