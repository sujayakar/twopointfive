//! src/game/stackroom.ts lines 1 to 378
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// (stackroom.ts flat)
#[inline]
fn flat(a: V3, b: V3) -> f64 { v3::dist_xz(a, b) }

// ---------------------------------------------------------------- the seams: what the skeleton has yet to give

/// O.of(m) of a man of stackViewFix's `men`: the parameter's type holds a position, the caller hands Guards
fn seam_man_assignment(g: &mut Game, m: &crate::stackroom::StackViewFixMenItem) -> Option<OrdersAssignmentId> { todo!("needs: stackViewFix men to carry the GuardId (ts:275)") }

// ---------------------------------------------------------------- the book

/// keeps which of the book's floors and doorway cells have been read, for an ask made only to be looked at   (stackroom.ts stackRoomBookKeep)
pub fn stack_room_book_keep(g: &mut crate::game::Game) -> crate::stackroom::StackRoomBookKeep {
    let Some(k) = &g.m.stackroom.books else { return crate::stackroom::StackRoomBookKeep::Forget };
    let mut floors: JsMap<DoorId, [bool; 2]> = JsMap::new();
    for (d, pair) in k.floors.iter() { floors.set(d, [pair[0].is_some(), pair[1].is_some()]); }
    crate::stackroom::StackRoomBookKeep::Back { sig: k.sig, doors: k.doors, near: k.near.is_some(), floors }
}

/// this game's book of floors, new if the grid's cells or the doors changed   (stackroom.ts bookOf)
pub fn book_of<'g>(g: &'g mut crate::game::Game) -> &'g mut crate::stackroom::Book {
    let sig = g.engine.world.geom_serial;
    let doors = g.doors.list.len() as f64;
    let stale = match &g.m.stackroom.books { Some(k) => k.sig != sig || k.doors != doors, None => true };
    if stale {
        g.m.stackroom.books = Some(crate::stackroom::Book { sig, doors, near: None, floors: JsMap::new() });
    }
    g.m.stackroom.books.as_mut().unwrap()
}

/// the middle of a nav cell   (stackroom.ts centre)
pub fn centre(g: &mut crate::game::Game, c_2: f64) -> V3 {
    let (w, cell) = (g.col.nav.w, g.col.nav.cell);
    [(js::rem(c_2, w) + 0.5) * cell, 0.0, (js::to_i32(c_2 / w) as f64 + 0.5) * cell]
}

/// some doorway's frame line lies between a and b inside its opening, or one of them is on it   (stackroom.ts doorBetween)
pub fn door_between(g: &mut crate::game::Game, a: V3, b: V3) -> bool {
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
fn nav_idx(g: &mut Game, x: f64, z: f64) -> f64 { crate::scene_collision::nav_grid_idx(g, x, z) }
/// nav.isBlocked(x, z)
#[inline]
fn nav_blocked(g: &mut Game, x: f64, z: f64) -> bool { crate::scene_collision::nav_grid_is_blocked(g, x, z) }
/// a cell on the grid and not blocked   (stackroom.ts floorOf's free)
fn cell_free(g: &mut Game, x: f64, z: f64) -> bool {
    if !(x >= 0.0 && z >= 0.0 && x < g.col.nav.w && z < g.col.nav.h) { return false; }
    let i = nav_idx(g, x, z) as usize;
    g.col.nav.blocked[i] != 1
}

/// the cells a frame line can pass by, made once a book: the only steps asked about a doorway   (stackroom.ts floorOf)
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

/// the floor before face `side` of `d`; None: that face has none   (stackroom.ts floorOf)
pub fn floor_of(g: &mut crate::game::Game, d: DoorId, side: f64) -> Option<crate::stackroom::Floor> {
    let at = if side > 0.0 { 0 } else { 1 };
    {
        let k = book_of(g);
        match k.floors.get(d) {
            Some(pair) => { if let Some(f) = &pair[at] { return f.clone(); } }
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
        return None;
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
    g.m.stackroom.books.as_mut().unwrap().floors.get_mut(d).unwrap()[at] = Some(Some(f.clone()));
    Some(f)
}

/// steps of walking from the doorway to `p` on that floor (−1: not on it)   (stackroom.ts stepsTo)
pub fn steps_to(g: &mut crate::game::Game, f: &crate::stackroom::Floor, p: V3) -> f64 {
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
            if door_between(g, mid, p) || crate::scene_collision::solid_blocked(g, [mid[0], 0.45, mid[2]], [p[0], 0.45, p[2]], None) { continue; }
            best = here + 1.0;
        }
    }
    best
}

/// on the floor, and the walk there no way round a wall   (stackroom.ts onFloor)
pub fn on_floor(g: &mut crate::game::Game, f: &crate::stackroom::Floor, p: V3) -> bool {
    let s = steps_to(g, f, p);
    if !(s >= 0.0) { return false; }
    let mid = centre(g, f.seed);
    s * g.col.nav.cell <= g.k.stackroom.round_m + g.k.stackroom.round_k * flat(mid, p)
}

/// is `p` on the floor of that face of `d`: true for a face with no floor at all   (stackroom.ts stackRoomHas)
pub fn stack_room_has(g: &mut crate::game::Game, d: DoorId, side: f64, p: V3) -> bool {
    match floor_of(g, d, side) { None => true, Some(f) => on_floor(g, &f, p) }
}

/// … the same, counted: a tuck or a step back not taken for it   (stackroom.ts stackRoomKeeps)
pub fn stack_room_keeps(g: &mut crate::game::Game, d: DoorId, side: f64, p: V3) -> bool {
    let ok = stack_room_has(g, d, side, p);
    if !ok { g.stats.stackroom.kept += 1; }
    ok
}

/// is `q` the mark a man of the squad already stands on at this door   (stackroom.ts stoodAt)
pub fn stood_at(g: &mut crate::game::Game, d: DoorId, q: V3) -> bool {
    for man in crate::orders::squad_orders_squad(g) {
        let a = crate::orders::squad_orders_of(g, man);
        let Some(a) = a else { continue };
        if g.gd[man].state == crate::game::GuardState::Dead { continue; }
        if g.tab.orders_assignment[a].door != Some(d) { continue; }
        let Some(post) = g.tab.orders_assignment[a].post else { continue };
        if flat(post, q) < SAME_M && flat(g.ch[g.gd[man].ch].pos, post) < 0.6 { return true; }
    }
    false
}

/// a place a piece of furniture stands on, unless a man of the squad already stands on it at this door   (stackroom.ts furnished)
pub fn furnished(g: &mut crate::game::Game, d: DoorId, q: V3) -> bool {
    crate::orders::squad_orders_on_furniture(g, q) && !stood_at(g, d, q)
}

// ---------------------------------------------------------------- THE RULE

/// nothing solid between two places, at knee height   (stackroom.ts stackRoomFix's sees)
fn sees(g: &mut Game, a: V3, q: V3) -> bool { !crate::scene_collision::solid_blocked(g, [a[0], 0.45, a[2]], [q[0], 0.45, q[2]], None) }
/// off the opening's own lane   (stackroom.ts stackRoomFix's offLane)
fn off_lane(g: &mut Game, d: DoorId, w: f64, q: V3) -> bool { m::abs(crate::doortools::along_of(g, d, q) - w / 2.0) >= w / 2.0 + 0.25 }
/// the hand of the opening a place is on   (stackroom.ts handOf)
fn hand_of(g: &mut Game, d: DoorId, w: f64, q: V3) -> f64 { if crate::doortools::along_of(g, d, q) > w / 2.0 { 1.0 } else { -1.0 } }
/// a place along the door's wall: faced down the wall at the doorway, and the shoulder the wall is on (face, stackSide)   (stackroom.ts onWall)
fn on_wall(th: f64, side: f64, hand: f64) -> (f64, f64) {
    let (fx, fz, nx, nz) = (-m::cos(th) * hand, -m::sin(th) * hand, -m::sin(th) * side, m::cos(th) * side);
    (m::atan2(fx, fz), if nx * fz - nz * fx >= 0.0 { 1.0 } else { -1.0 })
}
/// a cell's middle not up against the furniture   (stackroom.ts stackRoomFix's clear)
fn clear_of_props(g: &mut Game, q: V3) -> bool {
    for n in 0..g.props.props.len() {
        let pr = g.props.props[n];
        let p = &g.tab.props_prop[pr];
        let gap = crate::roomclear::prop_gap(crate::roomclear::PropGapP { home: p.home, hx: p.hx, hz: p.hz }, q);
        if gap < 0.3 { return false; }
    }
    true
}

/// what `fits` of stackRoomFix closed over
struct FixFit<'a> { d: DoorId, side: f64, f: &'a crate::stackroom::Floor, c: V3, min_m: f64, used: &'a [V3], glass: bool, shy: bool }
/// a place a mark may be dealt   (stackroom.ts stackRoomFix's fits)
fn fix_fits(g: &mut Game, x: &FixFit, q: V3, apart: f64) -> bool {
    on_floor(g, x.f, q)
        && flat(q, x.c) >= x.min_m
        && !x.used.iter().any(|o| flat(*o, q) < apart)
        && !crate::doorway::doorway_in_way(g, q)
        && !(x.glass && crate::orders::squad_orders_behind_glass(g, x.d, x.side, q))
        && !(x.shy && furnished(g, x.d, q))
}

/// what a mark dealt again takes of its new place (Partial<StackPost>): no stackSide = its own stays; group always, none included
struct Got { pos: V3, face: f64, stack_side: Option<f64>, group: Option<f64> }

/// every post not on the floor of that face is dealt again, the rest stay; None: one has no room at all   (stackroom.ts stackRoomFix)
pub fn stack_room_fix(g: &mut crate::game::Game, d: DoorId, side: f64, posts: Option<&[crate::orders::StackPost]>, taken: &[V3], min_m: Option<f64>) -> Option<Vec<crate::orders::StackPost>> {
    let min_m = min_m.unwrap_or(0.0);
    let posts = posts?;
    let Some(f) = floor_of(g, d, side) else { return Some(posts.to_vec()) };
    let s = g.k.stackroom;
    g.stats.stackroom.asked += posts.len() as i32;
    let mut bad = Vec::with_capacity(posts.len());
    for p in posts { bad.push(!on_floor(g, &f, p.pos)); }
    if !bad.iter().any(|b| *b) { return Some(posts.to_vec()); }
    let mut out = posts.to_vec();
    let (w, c, th) = (g.dr[d].def.width, g.dr[d].frame_centre, g.dr[d].def.closed_dir);
    let (jamb_out, jamb_past, file_gap) = (g.orders.knobs.jamb_out, g.orders.knobs.jamb_past, g.orders.knobs.file_gap);
    let cell = g.col.nav.cell;
    let front = crate::doortools::door_pt(g, d, w / 2.0, jamb_out, side, Some(0.45));
    for i in 0..out.len() {
        if !bad[i] { continue; }
        g.stats.stackroom.off += 1;
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
                    g.stats.stackroom.other += 1;
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
                            g.stats.stackroom.behind += 1;
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
                        g.stats.stackroom.rear += 1;
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
                    g.stats.stackroom.floor += 1;
                }
            }
            // 5 the room is too small: by a doorway of a room next to it that the team already holds, never the man who has the door to open
            if got.is_none() && !pp.lead && !glass {
                if let Some(n) = next_room(g, d, side, &f, &used, min_m, shy) {
                    got = Some(Got { pos: n.pos, face: n.face, stack_side: None, group: if pp.group.is_none() { None } else { Some(2.0) } });
                    g.stats.stackroom.next += 1;
                }
            }
            // 6 anywhere on the face's floor, off the opening's lane, a body fits
            if got.is_none() && !glass {
                for &cc in &f.cells {
                    let p = centre(g, cc);
                    if off_lane(g, d, w, p) && fix_fits(g, &x, p, s.squeeze) && clear_of_props(g, p) {
                        got = Some(Got { pos: p, face: v3::yaw_to(p, c), stack_side: None, group: if pp.group.is_none() { None } else { Some(2.0) } });
                        g.stats.stackroom.squeezed += 1;
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
                            g.stats.stackroom.lane += 1;
                            break;
                        }
                    }
                }
            }
        }
        let Some(got) = got else {
            g.stats.stackroom.none += 1;
            return None;
        };
        // the door has ONE opener: with a man still on the other jamb the handle is that man's
        let lead = pp.lead && !out.iter().enumerate().any(|(j, o)| j != i && !bad[j] && o.lead);
        if pp.lead && !lead { g.stats.stackroom.unled += 1; }
        out[i] = crate::orders::StackPost { pos: got.pos, face: got.face, stack_side: got.stack_side.unwrap_or(pp.stack_side), lead, group: got.group };
        bad[i] = false;
    }
    Some(out)
}

// ---------------------------------------------------------------- OUT OF THE ROOM'S VIEW

/// is `p` (on face `side`) in view of the floor beyond the door through its opening   (stackroom.ts inView)
pub fn in_view(g: &mut crate::game::Game, d: DoorId, far: &[V3], p: V3) -> bool {
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

/// the nav cells within `viewFar` of the doorway on the face the men are NOT on   (stackroom.ts farCells)
pub fn far_cells(g: &mut crate::game::Game, d: DoorId, side: f64) -> Vec<V3> {
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

/// the leaf stands open: off its hinges, or unlatched and swung   (stackroom.ts stackViewOpen)
pub fn stack_view_open(g: &mut crate::game::Game, d: DoorId) -> bool {
    if crate::doors::door_off(g, d) { return true; }
    if crate::doors::door_latched(g, d) { return false; }
    m::abs(crate::doors::door_angle(g, d)) >= g.k.stackroom.view_ajar
}

/// how far from the frame's centre a plain stack's mark on this door can be   (stackroom.ts stackViewReach)
pub fn stack_view_reach(g: &mut crate::game::Game, d: DoorId) -> f64 {
    if stack_view_open(g, d) { m::max(2.6, g.dr[d].def.width / 2.0 + g.k.stackroom.view_past + 0.7) } else { 2.6 }
}

/// QA: is this place in the view the rule measures   (stackroom.ts stackViewSees)
pub fn stack_view_sees(g: &mut crate::game::Game, d: DoorId, side: f64, p: V3) -> bool {
    let far = far_cells(g, d, side);
    in_view(g, d, &far, p)
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

/// the places along hand `h` of the door's wall the floor beyond does not see, from the jamb out, made once a hand: (place, behind glass)   (stackroom.ts candsOf)
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

/// the first such place on hand `h` a body clear of every place that stands, not behind glass while another will do   (stackroom.ts stackViewFix's place)
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

/// a place for post `i`: across the opening first if it is across, else its own hand if it is in view   (stackroom.ts stackViewFix's deal)
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

/// a plain 'stack up' on a door standing open: the marks in view, and those across the opening from `men`, dealt again along the door's wall   (stackroom.ts stackViewFix)
pub fn stack_view_fix(g: &mut crate::game::Game, d: DoorId, side: f64, posts: Option<&[crate::orders::StackPost]>, taken: &[V3], men: &[crate::stackroom::StackViewFixMenItem]) -> Option<Vec<crate::orders::StackPost>> {
    let posts = posts?;
    let s = g.k.stackroom;
    if !stack_view_open(g, d) { return Some(posts.to_vec()); }
    let Some(f) = floor_of(g, d, side) else { return Some(posts.to_vec()) };
    let (jamb_out, jamb_past) = (g.orders.knobs.jamb_out, g.orders.knobs.jamb_past);
    let (w, c, th) = (g.dr[d].def.width, g.dr[d].frame_centre, g.dr[d].def.closed_dir);
    let far = far_cells(g, d, side);
    g.stats.stackroom.view_asked += 1;
    let front = crate::doortools::door_pt(g, d, w / 2.0, jamb_out, side, Some(0.45));
    // A STACK THAT STANDS DOES NOT CREEP: a place a man of this order already stands on, no mark as dealt now, is his while it still stands as a place
    let mut pre: JsMap<usize, V3> = JsMap::new();
    {
        let mut stood: Vec<V3> = Vec::new();
        for man in men {
            let a = seam_man_assignment(g, man);
            let Some(a) = a else { continue };
            if g.tab.orders_assignment[a].order != "stack" || g.tab.orders_assignment[a].door != Some(d) { continue; }
            let Some(post) = g.tab.orders_assignment[a].post else { continue };
            if flat(man.char.pos, post) < 0.6 && crate::doortools::off_of(g, d, post, side) > 0.0 { stood.push(post); }
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
        for man in men {
            let p = man.char.pos;
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
    g.stats.stackroom.view_left += stay.len() as i32;
    g.stats.stackroom.view_stood += pre.len() as i32;
    if got.is_empty() { return Some(posts.to_vec()); }
    let mut out = posts.to_vec();
    let mut led = false;
    for (i, dealt) in got.iter() {
        let pp = posts[i];
        let h = dealt.h;
        if dealt.over { g.stats.stackroom.view_over += 1; } else { g.stats.stackroom.view_out += 1; }
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

/// (5) a place by a doorway of a room next to the face's own, one the team holds, on that doorway's far face   (stackroom.ts nextRoom)
pub fn next_room(g: &mut crate::game::Game, d: DoorId, side: f64, f: &crate::stackroom::Floor, used: &[V3], min_m: f64, shy: bool) -> Option<crate::stackroom::NextRoom> {
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
    if !g.pl[pl].down { ours.push(g.ch[g.pl[pl].ch].pos); }
    for man in crate::orders::squad_orders_squad(g) {
        if g.gd[man].state != crate::game::GuardState::Dead { ours.push(g.ch[g.gd[man].ch].pos); }
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

/// a place by that doorway a mark may be dealt   (stackroom.ts nextRoom's fits)
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

/// the floor of a face as the rule reads it (rearwatch.ts asks it, as do the QA tools)   (stackroom.ts stackRoomProbe)
pub fn stack_room_probe(g: &mut crate::game::Game, d: DoorId, side: f64) -> crate::stackroom::StackRoomProbe {
    let f = floor_of(g, d, side);
    let seed = match &f { Some(f) => Some(centre(g, f.seed)), None => None };
    crate::stackroom::StackRoomProbe {
        floor: f.is_some(),
        cells: match &f { Some(f) => f.cells.len() as f64, None => 0.0 },
        seed,
    }
}

/// the counters as one line   (stackroom.ts stackRoomSummary)
pub fn stack_room_summary(g: &mut crate::game::Game) -> String {
    let t = &g.stats.stackroom;
    format!("stack rooms: {} of {} marks off the face's floor · dealt again {} other jamb, {} behind the file, {} back, {} floor, {} next room, {} squeezed, {} down the passage · {} no room · {} tucks/step-backs not taken · {} jamb men dealt again gave the door to the other jamb", t.off, t.asked, t.other, t.behind, t.rear, t.floor, t.next, t.squeezed, t.lane, t.none, t.kept, t.unled)
}

#[cfg(test)]
mod tests {
    use super::{book_of, floor_of, on_wall, stack_room_book_keep};
    use crate::game::Game;
    use crate::ids::DoorId;
    use crate::stackroom::StackRoomBookKeep;
    use alloc::vec;

    /// ts:119: a null in a pair is remembered. A flood would reach door_pt, a stub, and panic
    #[test]
    fn a_face_with_no_floor_is_remembered() {
        let mut g = Game::blank();
        let d = DoorId(0);
        book_of(&mut g).floors.set(d, [Some(None), None]);
        assert!(floor_of(&mut g, d, 1.0).is_none());
    }

    /// ts:98-103: the answer is the row, and a moved serial starts a new book
    #[test]
    fn book_of_is_the_row_until_the_grid_moves() {
        let mut g = Game::blank();
        book_of(&mut g).near = Some(vec![1]);
        assert!(g.m.stackroom.books.as_ref().unwrap().near.is_some());
        assert!(book_of(&mut g).near.is_some());
        g.engine.world.geom_serial += 1.0;
        assert!(book_of(&mut g).near.is_none());
    }

    /// ts:92-95: no book is Forget; a read face is marked, a null one too, an unread one not
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

    /// (th, side, hand, face's bits, stackSide) from the TypeScript's own onWall, under bun with EXACT.on
    const ON_WALL: [(f64, f64, f64, u64, f64); 20] = [
        (0.0, 1.0, 1.0, 0xbff921fb54442d18, 1.0),
        (0.0, 1.0, -1.0, 0x3ff921fb54442d18, -1.0),
        (0.0, -1.0, 1.0, 0xbff921fb54442d18, -1.0),
        (0.0, -1.0, -1.0, 0x3ff921fb54442d18, 1.0),
        (1.5707963267948966, 1.0, 1.0, 0xc00921fb54442d18, 1.0),
        (1.5707963267948966, 1.0, -1.0, 0x3c91a62633145c00, -1.0),
        (1.5707963267948966, -1.0, 1.0, 0xc00921fb54442d18, -1.0),
        (1.5707963267948966, -1.0, -1.0, 0x3c91a62633145c00, 1.0),
        (0.7, 1.0, 1.0, 0xc0022a9743bbb026, 1.0),
        (0.7, 1.0, -1.0, 0x3febdd904221f3ca, -1.0),
        (0.7, -1.0, 1.0, 0xc0022a9743bbb026, -1.0),
        (0.7, -1.0, -1.0, 0x3febdd904221f3ca, 1.0),
        (-2.3, 1.0, 1.0, 0x3fe755a2f1113f67, 1.0),
        (-2.3, 1.0, -1.0, 0xc0034c9297ffdd3e, -1.0),
        (-2.3, -1.0, 1.0, 0x3fe755a2f1113f67, -1.0),
        (-2.3, -1.0, -1.0, 0xc0034c9297ffdd3e, 1.0),
        (3.141592653589793, 1.0, 1.0, 0x3ff921fb54442d19, 1.0),
        (3.141592653589793, 1.0, -1.0, 0xbff921fb54442d17, -1.0),
        (3.141592653589793, -1.0, 1.0, 0x3ff921fb54442d19, -1.0),
        (3.141592653589793, -1.0, -1.0, 0xbff921fb54442d17, 1.0),
    ];
    #[test]
    fn on_wall_has_the_typescripts_bits() {
        for (th, side, hand, face, stack_side) in ON_WALL {
            let (f, s) = on_wall(th, side, hand);
            assert_eq!(f.to_bits(), face, "face at {th} {side} {hand}");
            assert_eq!(s.to_bits(), stack_side.to_bits(), "stackSide at {th} {side} {hand}");
        }
    }
}
