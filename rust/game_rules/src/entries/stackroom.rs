pub use ::game_types::stackroom::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, V3};
use alloc::{vec, vec::Vec};

use crate::ask::Over;

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
    // only the doors kept, in the order kept
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
fn centre(g: &crate::game::Game, c_2: f64) -> V3 {
    let (w, cell) = (g.col.nav.w, g.col.nav.cell);
    [(js::rem(c_2, w) + 0.5) * cell, 0.0, (js::to_i32(c_2 / w) as f64 + 0.5) * cell]
}

/// some doorway's frame line lies between a and b inside its opening, or one of them is on it
fn door_between(g: &crate::game::Game, a: V3, b: V3) -> bool {
    for n in 0..g.doors.list.len() {
        let o = g.doors.list[n];
        let width = g.dr[o].def.width;
        if v3::dist_xz(a, g.dr[o].frame_centre) > width + 1.5 { continue; }
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
            let s = v3::dist_xz(c, front) + (if crate::scene_collision::low_blocked(g, front, c) { 10.0 } else { 0.0 });
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
fn steps_to(g: &crate::game::Game, over: &Over, f: &crate::stackroom::Floor, p: V3) -> f64 {
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
fn on_floor(g: &crate::game::Game, f: &crate::stackroom::Floor, p: V3) -> bool { on_floor_over(g, &Over::OWN, f, p) }
/// …of the world as the question takes it
fn on_floor_over(g: &Game, over: &Over, f: &crate::stackroom::Floor, p: V3) -> bool {
    let s = steps_to(g, over, f, p);
    if !(s >= 0.0) { return false; }
    let mid = centre(g, f.seed);
    s * g.col.nav.cell <= g.k.stackroom.round_m + g.k.stackroom.round_k * v3::dist_xz(mid, p)
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
        if g.tab.task[a].door != Some(d) { continue; }
        let Some(post) = g.tab.task[a].post else { continue };
        if v3::dist_xz(post, q) < SAME_M && v3::dist_xz(g.pos_of(man), post) < 0.6 { return true; }
    }
    false
}

/// a place a piece of furniture stands on, unless a man of the squad already stands on it at this door
fn furnished(g: &crate::game::Game, d: DoorId, q: V3) -> bool {
    crate::orders::squad_orders_on_furniture(g, q) && !stood_at(g, &crate::seats::here(g), d, q)
}

// ---------------------------------------------------------------- THE RULE

mod fix;
pub use fix::stack_room_fix;
mod view;
pub use view::{stack_view_fix, stack_view_open};
pub(super) use view::stack_view_reach;

/// off the opening's own lane
fn off_lane(g: &Game, d: DoorId, w: f64, q: V3) -> bool { m::abs(crate::doortools::along_of(g, d, q) - w / 2.0) >= w / 2.0 + 0.25 }
/// the hand of the opening a place is on
fn hand_of(g: &Game, d: DoorId, w: f64, q: V3) -> f64 { if crate::doortools::along_of(g, d, q) > w / 2.0 { 1.0 } else { -1.0 } }
/// a place along the door's wall: faced down the wall at the doorway, and the shoulder the wall is on (face, stackSide)
fn on_wall(th: f64, side: f64, hand: f64) -> (f64, f64) {
    let (fx, fz, nx, nz) = (-m::cos(th) * hand, -m::sin(th) * hand, -m::sin(th) * side, m::cos(th) * side);
    (m::atan2(fx, fz), if nx * fz - nz * fx >= 0.0 { 1.0 } else { -1.0 })
}

/// the floor of a face as the rule reads it (the rear watch asks it)
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
        g.boxes_changed();
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
        g.boxes_changed();
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
