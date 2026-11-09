//! THE ROOMS a fight is fought in: which room a place is in, and free floor in it.
#![allow(unused_imports)]
use super::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

pub(super) fn cell_free(g: &Game, dc: &JsMap<crate::Num, DoorId>, x: f64, z: f64) -> bool {
    let (w, h) = (g.col.nav.w, g.col.nav.h);
    if !(x >= 0.0 && z >= 0.0 && x < w && z < h) { return false; }
    let i = crate::scene_collision::nav_grid_idx(g, x, z);
    g.col.nav.blocked[i as usize] != 1 && !dc.has(crate::Num(i)) && !g.destr.walls.shell_gaps.has(crate::Num(i))
}

/// the room a point stands in: the nav cells reachable without crossing a doorway, and the doors on its edge (cached).   (roomfight.ts roomAt)
pub fn room_fight_room_at(g: &mut crate::game::Game, p: V3) -> Option<RoomfightRoomId> {
    let cs = g.col.nav.cell; let (w, h) = (g.col.nav.w, g.col.nav.h);
    let dc = crate::guards::door_cells(g);
    let (mut sx, mut sz) = (m::floor(p[0] / cs), m::floor(p[2] / cs));
    if !cell_free(g, &dc, sx, sz) {
        let mut found = false;
        'look: for r in 1..=3i32 { for dz in -r..=r { for dx in -r..=r {
            if cell_free(g, &dc, sx + dx as f64, sz + dz as f64) { sx += dx as f64; sz += dz as f64; found = true; break 'look; }
        } } }
        if !found { return None; }
    }
    let start = crate::scene_collision::nav_grid_idx(g, sx, sz);
    if let Some(&hit) = g.fight.room_of_cell.get(crate::Num(start)) { return Some(hit); }
    let mut cells: JsSet<crate::Num> = JsSet::default(); cells.add(crate::Num(start));
    let mut q: Vec<f64> = vec![start];
    let mut touched: JsMap<DoorId, f64> = JsMap::default();
    let size = (w * h) as usize;
    if g.fight.fill_mark.len() != size { g.fight.fill_mark = vec![0u8; size]; }
    g.fight.fill_mark[start as usize] = 1;
    let mut merge: Option<RoomfightRoomId> = None;
    let (dxs, dzs) = (room_dx(), room_dz());
    let mut qi = 0;
    while qi < q.len() && (cells.len() as f64) < ROOM_MAX_CELLS && merge.is_none() {
        let c = q[qi]; qi += 1;
        let x = js::rem(c, w); let z = js::to_i32(c / w) as f64;
        for k in 0..4 {
            let (nx, nz) = (x + dxs[k], z + dzs[k]);
            if nx < 0.0 || nz < 0.0 || nx >= w || nz >= h { continue; }
            let n = crate::scene_collision::nav_grid_idx(g, nx, nz);
            if let Some(&d) = dc.get(crate::Num(n)) { if !touched.has(d) { touched.set(d, c); } continue; }
            if g.col.nav.blocked[n as usize] == 1 || g.fight.fill_mark[n as usize] == 1 || g.destr.walls.shell_gaps.has(crate::Num(n)) { continue; }
            if let Some(&other) = g.fight.room_of_cell.get(crate::Num(n)) { merge = Some(other); break; }
            cells.add(crate::Num(n)); q.push(n); g.fight.fill_mark[n as usize] = 1;
        }
    }
    for &c in &q { g.fight.fill_mark[c as usize] = 0; }
    let room = match merge {
        Some(r) => {   // (a room wallreact absorbed holds this one's Set in the TypeScript: it gets the same cells)
            let same: Vec<_> = g.fight.rooms.iter().copied().filter(|o| *o != r && g.tab.roomfight_room[*o].cells.keys() == g.tab.roomfight_room[r].cells.keys()).collect();
            for c in cells.keys() { g.tab.roomfight_room[r].cells.add(c); for o in &same { g.tab.roomfight_room[*o].cells.add(c); } }
            r
        }
        None => { let id = g.fight.rooms.len() as f64; g.tab.roomfight_room.push(Room { id, cells: cells.clone(), entries: Vec::new() }) }
    };
    let met: Vec<(DoorId, f64)> = touched.iter().map(|(d, &c)| (d, c)).collect();
    for (d, c) in met {
        if g.tab.roomfight_room[room].entries.iter().any(|e| e.door == d) { continue; }
        let f = crate::guards::frame_of(g, d);
        let cx = (js::rem(c, w) + 0.5) * cs; let cz = (js::to_i32(c / w) as f64 + 0.5) * cs;
        let side = if (cx - f.hx) * f.nx + (cz - f.hz) * f.nz >= 0.0 { 1.0 } else { -1.0 };
        let fc = g.dr[d].frame_centre;
        g.tab.roomfight_room[room].entries.push(Entry { door: d, at: [fc[0], 0.0, fc[2]], out: [-side * f.nx, -side * f.nz] });
    }
    if merge.is_none() { g.fight.rooms.push(room); }
    for c in cells.keys() { g.fight.room_of_cell.set(c, room); }
    Some(room)
}

/// (roomfight.ts inRoom)
pub(in crate::defenders) fn room_fight_in_room(g: &crate::game::Game, r: Option<RoomfightRoomId>, p: V3) -> bool {
    let r = match r { Some(r) => r, None => return false };
    let cell = g.col.nav.cell;
    let i = crate::scene_collision::nav_grid_idx(g, m::floor(p[0] / cell), m::floor(p[2] / cell));
    g.tab.roomfight_room[r].cells.has(crate::Num(i))
}

/// (roomfight.ts freeNear)
pub(in crate::defenders) fn room_fight_free_near(g: &crate::game::Game, p: V3) -> Option<V3> {
    if !crate::scene_collision::nav_grid_is_blocked(g, p[0], p[2]) { return Some([p[0], 0.0, p[2]]); }
    for r in [0.5, 1.0, 1.5] { for a in 0..8 {
        let a = a as f64;
        let x = p[0] + m::cos(a * m::PI / 4.0) * r; let z = p[2] + m::sin(a * m::PI / 4.0) * r;
        if !crate::scene_collision::nav_grid_is_blocked(g, x, z) { return Some([x, 0.0, z]); }
    } }
    None
}

/// `p` lies outside the rectangle round every room of the level's structure with open sky over it.   (roomfight.ts outOfBuilding)
pub(super) fn room_fight_out_of_building(g: &mut crate::game::Game, p: V3) -> bool {
    let (mut x0, mut x1, mut z0, mut z1) = (f64::INFINITY, f64::NEG_INFINITY, f64::INFINITY, f64::NEG_INFINITY);
    {
        let rooms = g.level.rooms();
        if rooms.is_empty() { return false; }
        for r in rooms { x0 = m::min(x0, r.bounds.x0); x1 = m::max(x1, r.bounds.x1); z0 = m::min(z0, r.bounds.z0); z1 = m::max(z1, r.bounds.z1); }
    }
    if p[0] >= x0 && p[0] <= x1 && p[2] >= z0 && p[2] <= z1 { return false; }
    let mut roofed = 0;
    for (dx, dz) in [(0.0, 0.0), (0.35, 0.0), (-0.35, 0.0), (0.0, 0.35), (0.0, -0.35)] {
        if crate::scene_collision::solid_blocked(g, [p[0] + dx, 1.9, p[2] + dz], [p[0] + dx, 9.0, p[2] + dz], Some(0.02)) { roofed += 1; }
    }
    roofed < 3
}

/// The open doorway `p` stands in: its frame's span, within 0.75 m of the frame line; a shut leaf is a wall. (roomfight.ts funnelOf)
pub(super) fn room_fight_funnel_of(g: &crate::game::Game, p: V3) -> Option<DoorId> {
    let mut i = 0;
    while i < g.doors.list.len() {
        let d = g.doors.list[i];
        i += 1;
        if crate::doors::door_latched(g, d) || m::abs(crate::doors::door_angle(g, d)) < 0.6 { continue; }
        let f = crate::guards::frame_of(g, d);
        let along = (p[0] - f.hx) * f.ux + (p[2] - f.hz) * f.uz;
        let off = (p[0] - f.hx) * f.nx + (p[2] - f.hz) * f.nz;
        if along > -0.05 && along < f.w + 0.05 && m::abs(off) < 0.75 { return Some(d); }
    }
    None
}

/// A spot a man can step to: walkable, and nobody up within 0.8 m. (roomfight.ts spotFree)
pub(super) fn room_fight_spot_free(g: &crate::game::Game, p: V3, self_: GuardId) -> bool {
    if crate::scene_collision::nav_grid_is_blocked(g, p[0], p[2]) { return false; }
    for pl in crate::seats::men(g) {
        if !g.pl[pl].down && flat(g.ch[g.pl[pl].ch].pos, p) < 0.8 { return false; }
    }
    let mut i = 0;
    while i < g.guards.len() {
        let o = g.guards[i];
        i += 1;
        if o != self_ && g.gd[o].state != crate::game::GuardState::Dead && flat(g.ch[g.gd[o].ch].pos, p) < 0.8 { return false; }
    }
    true
}
