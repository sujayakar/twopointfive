pub use ::game_types::cleared::types;
pub use types::*;
pub mod looked;
pub use looked::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{vec, vec::Vec};

/// `M[i]` of a Uint16Array: `None` is JavaScript's undefined (a negative, NaN or past-the-end index).
fn cell_at(m: &[u16], i: f64) -> Option<f64> {
    if i >= 0.0 && i < m.len() as f64 { Some(m[i as usize] as f64) } else { None }
}

/// `!!knownRoom[r]`, an index past the array being undefined.
fn known_flag(g: &Game, r: f64) -> bool {
    r >= 0.0 && g.cleared.known_room.get(r as usize).copied().unwrap_or(0) != 0
}

/// The built grid, once `cleared_rooms_build` has run.
fn grid(g: &Game) -> &[u16] {
    g.cleared.room_of_cell.as_deref().unwrap()
}

/// May a flood stand on this cell: on the grid, not shut, not a doorway.
fn free(g: &Game, dc: &JsMap<crate::Num, DoorId>, x: f64, z: f64) -> bool {
    if !(x >= 0.0 && z >= 0.0 && x < g.col.nav.w && z < g.col.nav.h) { return false; }
    let i = crate::scene_collision::nav_grid_idx(g, x, z);
    // (`+ 0.0`: a Map's key is the same for −0 and 0, Num's is by bits)
    g.col.nav.blocked[i as usize] != 1 && !dc.has(crate::Num(i + 0.0))
}

/// The rooms of the level: a flood from the floor on either face of every door. Built once: a game has one nav grid, rebaked in place.
fn cleared_rooms_build(g: &mut crate::game::Game) {
    if g.cleared.room_of_cell.is_some() { return; }
    let (w, h, cs) = (g.col.nav.w, g.col.nav.h, g.col.nav.cell);
    let dc = crate::guards::door_cells(g);
    let mut grid_new = vec![0u16; (w * h) as usize];
    let mut seen = vec![0u8; (w * h) as usize];
    g.cleared.sizes = vec![0.0];
    g.cleared.seeds.clear();
    let list = g.doors.list.clone();
    for d in list {
        g.cleared.seeds.set(d, [-1.0, -1.0]);
        for b in [1.0, -1.0] {
            let th = g.dr[d].def.closed_dir;
            let c = g.dr[d].frame_centre;
            let mut sx = m::floor((c[0] - m::sin(th) * b) / cs);
            let mut sz = m::floor((c[2] + m::cos(th) * b) / cs);
            if !free(g, &dc, sx, sz) {
                let mut found = false;
                'search: for r in 1..=2i32 {
                    for dz in -r..=r {
                        for dx in -r..=r {
                            let x = sx + dx as f64;
                            let z = sz + dz as f64;
                            if free(g, &dc, x, z)
                                && ((x + 0.5) * cs - c[0]) * -m::sin(th) * b + ((z + 0.5) * cs - c[2]) * m::cos(th) * b > 0.0
                            {
                                sx = x;
                                sz = z;
                                found = true;
                                break 'search;
                            }
                        }
                    }
                }
                if !found { continue; }
            }
            let s0 = crate::scene_collision::nav_grid_idx(g, sx, sz);
            g.cleared.seeds.get_mut(d).unwrap()[if b > 0.0 { 0 } else { 1 }] = s0;
            if seen[s0 as usize] != 0 { continue; }
            let mut q = vec![s0];
            seen[s0 as usize] = 1;
            // (filled to its end, whatever its size: a fill cut short would leave a pocket the next door floods as a room of its own)
            let mut qi = 0;
            while qi < q.len() {
                let cc = q[qi];
                let x = js::rem(cc, w);
                let z = js::to_i32(cc / w) as f64;
                for (dx, dz) in [(1.0, 0.0), (-1.0, 0.0), (0.0, 1.0), (0.0, -1.0)] {
                    let nx = x + dx;
                    let nz = z + dz;
                    if !free(g, &dc, nx, nz) { continue; }
                    let n = crate::scene_collision::nav_grid_idx(g, nx, nz);
                    if seen[n as usize] == 0 {
                        seen[n as usize] = 1;
                        q.push(n);
                    }
                }
                qi += 1;
            }
            if q.len() as f64 > ROOM_MAX_CELLS { continue; } // (open floor: its cells stay 0)
            let id = g.cleared.sizes.len();
            g.cleared.sizes.push(q.len() as f64);
            for &cc in &q { grid_new[cc as usize] = id as u16; }
        }
    }
    g.cleared.known_room = vec![0; g.cleared.sizes.len()];
    g.cleared.known = 0.0;
    g.cleared.room_of_cell = Some(grid_new);
}

/// What `dearFor`'s price list answers for a cell. The room map and what is known are read live, as the closure's shared arrays were.
pub fn dear_price(g: &crate::game::Game, price: &crate::cleared::DearPrice, ix: f64, iz: f64) -> f64 {
    let crate::cleared::DearPrice::Rooms { dear, from_room, mark_rooms, also } = price;
    let i = crate::scene_collision::nav_grid_idx(g, ix, iz);
    match cell_at(grid(g), i) {
        Some(r) if r != 0.0 && (also.contains(&r) || (!known_flag(g, r) && r != *from_room && !mark_rooms.contains(&r))) => *dear,
        _ => 1.0,
    }
}

/// A new one: the field initialisers, nothing more.
pub fn cleared_rooms_new() -> crate::cleared::ClearedRooms {
    crate::cleared::ClearedRooms::blank()
}

/// The room `p` stands in (0: none, a doorway, open floor).
pub fn cleared_rooms_room_at(g: &mut crate::game::Game, p: V3) -> f64 {
    cleared_rooms_build(g);
    let cell = g.col.nav.cell;
    let x = m::floor(p[0] / cell);
    let z = m::floor(p[2] / cell);
    if x < 0.0 || z < 0.0 || x >= g.col.nav.w || z >= g.col.nav.h { return 0.0; }
    let i = crate::scene_collision::nav_grid_idx(g, x, z);
    cell_at(grid(g), i).unwrap_or(0.0)
}

/// Has the team been in the room `p` stands in (open floor and doorways: yes).
pub fn cleared_rooms_known_at(g: &mut crate::game::Game, p: V3) -> bool {
    let r = cleared_rooms_room_at(g, p);
    !js::truthy(r) || known_flag(g, r)
}

/// The room on a door's `side` face, by the cell its floor was flooded from (0: open floor, or no floor there).
pub fn cleared_rooms_room_by(g: &mut crate::game::Game, d: DoorId, side: f64) -> f64 {
    cleared_rooms_build(g);
    let s = g.cleared.seeds.get(d).map_or(-1.0, |p| p[if side > 0.0 { 0 } else { 1 }]);
    if s < 0.0 { return 0.0; }
    cell_at(grid(g), s).unwrap_or(0.0)
}

/// Is there floor at all on that face of the door.
pub fn cleared_rooms_has_floor(g: &mut crate::game::Game, d: DoorId, side: f64) -> bool {
    cleared_rooms_build(g);
    g.cleared.seeds.get(d).map_or(-1.0, |p| p[if side > 0.0 { 0 } else { 1 }]) >= 0.0
}

/// The room a mark is in; a mark on a cell that is no room's is in every room a cell away.
fn cleared_rooms_rooms_of(g: &mut crate::game::Game, p: V3) -> Vec<f64> {
    let r = cleared_rooms_room_at(g, p);
    if js::truthy(r) { return vec![r]; }
    cleared_rooms_build(g);
    let cell = g.col.nav.cell;
    let x = m::floor(p[0] / cell);
    let z = m::floor(p[2] / cell);
    let mut out: Vec<f64> = Vec::new();
    for dz in -1..=1i32 {
        for dx in -1..=1i32 {
            let nx = x + dx as f64;
            let nz = z + dz as f64;
            if nx < 0.0 || nz < 0.0 || nx >= g.col.nav.w || nz >= g.col.nav.h { continue; }
            let i = crate::scene_collision::nav_grid_idx(g, nx, nz);
            if let Some(q) = cell_at(grid(g), i) {
                if q != 0.0 && !out.contains(&q) { out.push(q); }
            }
        }
    }
    out
}

/// Has the team been in this room (no room: yes).
pub fn cleared_rooms_is_known(g: &crate::game::Game, room: f64) -> bool {
    !js::truthy(room) || known_flag(g, room)
}

/// Where somebody might stand in the rooms nobody of the team has been in: one point of their floor every `step` nav cells each way.
pub fn cleared_rooms_unknown_eyes(g: &mut crate::game::Game, step: f64) -> Vec<crate::cleared::ClearedRoomsUnknownEyesItem> {
    cleared_rooms_build(g);
    let (w, h, cell) = (g.col.nav.w, g.col.nav.h, g.col.nav.cell);
    let mut out = Vec::new();
    let n = m::max(1.0, js::to_i32(step) as f64);
    let mut iz = 0.0;
    while iz < h {
        let mut ix = 0.0;
        while ix < w {
            let i = crate::scene_collision::nav_grid_idx(g, ix, iz);
            if let Some(r) = cell_at(grid(g), i) {
                if r != 0.0 && !known_flag(g, r) {
                    out.push(crate::cleared::ClearedRoomsUnknownEyesItem { x: (ix + 0.5) * cell, z: (iz + 0.5) * cell });
                }
            }
            ix += n;
        }
        iz += n;
    }
    out
}

/// One of ours stands at `p`.
fn cleared_rooms_mark(g: &mut crate::game::Game, p: V3) {
    let r = cleared_rooms_room_at(g, p);
    if js::truthy(r) && !known_flag(g, r) {
        g.cleared.known_room[r as usize] = 1;
        g.cleared.known += 1.0;
    }
}

/// Four times a second: where the team stands is known ground.
pub fn cleared_rooms_tick(g: &mut crate::game::Game) {
    if !crate::seats::team_active(g) { return; }
    if g.cleared.last >= 0.0 && m::abs(g.time - g.cleared.last) < 0.25 { return; }
    g.cleared.last = g.time;
    for gd in crate::seats::team_squad(g, &crate::seats::here(g)) {
        if !crate::condition::down(g, g.gd[gd].ch) {
            let p = g.ch[g.gd[gd].ch].pos;
            cleared_rooms_mark(g, p);
        }
    }
    for pl in crate::seats::men(g) {
        if !crate::condition::down(g, g.pl[pl].ch) {
            let p = g.ch[g.pl[pl].ch].pos;
            cleared_rooms_mark(g, p);
        }
    }
    look_over(g);
}

/// A walk's price list for nav.findPath: every cell of a room nobody of the team has been in is `dear`, bar the room he stands in and the room his mark is in; `also`: rooms dear whatever is known of them. None: nothing on this floor is dear for this walk.
pub fn cleared_rooms_dear_for(g: &mut crate::game::Game, from: V3, to: V3, dear: f64, also: Option<&[f64]>) -> Option<crate::cleared::DearPrice> {
    let also = also.unwrap_or(&[]);
    cleared_rooms_build(g);
    let a = cleared_rooms_room_at(g, from);
    let b = cleared_rooms_rooms_of(g, to);
    let mut some = also.iter().any(|&r| r > 0.0);
    if !some {
        for i in 1..g.cleared.known_room.len() {
            let room = i as f64;
            if g.cleared.known_room[i] == 0 && room != a && !b.contains(&room) {
                some = true;
                break;
            }
        }
    }
    if !some { return None; }
    Some(crate::cleared::DearPrice::Rooms { dear, from_room: a, mark_rooms: b, also: also.to_vec() })
}

/// Metres of a path (from `from`, by its corners) that lie on cells `pen` prices dear.
pub fn cleared_rooms_dear_along(g: &mut crate::game::Game, from: V3, path: &[V3], pen: &mut dyn FnMut(&mut crate::game::Game, f64, f64) -> f64) -> f64 {
    let cell = g.col.nav.cell;
    let mut metres = 0.0;
    let mut a = from;
    for &b in path {
        let l = m::hyp2(b[0] - a[0], b[2] - a[2]);
        let n = m::max(1.0, m::ceil(l / 0.25));
        let mut i = 1.0;
        while i <= n {
            let x = a[0] + (b[0] - a[0]) * i / n;
            let z = a[2] + (b[2] - a[2]) * i / n;
            if pen(g, m::floor(x / cell), m::floor(z / cell)) > 1.0 { metres += l / n; }
            i += 1.0;
        }
        a = b;
    }
    metres
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The metres against the TypeScript's own bits (cleared.ts dearAlong under EXACT, a cell dear where its x is even).
    #[test]
    fn dear_along_keeps_the_typescripts_bits() {
        let mut g = Game::blank();
        let cell = g.col.nav.cell;
        assert_eq!(cell, 0.5);
        let path = [[3.1, 0.0, -2.2], [3.1, 0.0, 9.9], [-4.7, 0.0, 1.3]];
        let mut pen = |_: &mut Game, ix: f64, _iz: f64| if js::rem(ix, 2.0) == 0.0 { 2.0 } else { 1.0 };
        let metres = cleared_rooms_dear_along(&mut g, [0.3, 0.0, 0.7], &path, &mut pen);
        assert_eq!(metres.to_bits(), 0x4033adbc8267228b);
    }

    /// An index a typed array has no element for reads as undefined, not a panic.
    #[test]
    fn cell_at_reads_as_a_typed_array_does() {
        let cells = [0u16, 7, 65535];
        assert_eq!(cell_at(&cells, 0.0), Some(0.0));
        assert_eq!(cell_at(&cells, -0.0), Some(0.0));
        assert_eq!(cell_at(&cells, 2.0), Some(65535.0));
        assert_eq!(cell_at(&cells, 3.0), None);
        assert_eq!(cell_at(&cells, -1.0), None);
        assert_eq!(cell_at(&cells, f64::NAN), None);
    }
}
