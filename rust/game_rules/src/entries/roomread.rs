pub use ::game_types::roomread::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{string::String, vec, vec::Vec};

use crate::Num;
use crate::scene_collision::{nav_grid_idx, nav_grid_is_blocked};

/// a nav cell on the grid and not blocked
fn free(g: &Game, x: f64, z: f64) -> bool {
    let (w, h) = (g.col.nav.w, g.col.nav.h);
    if !(x >= 0.0 && z >= 0.0 && x < w && z < h) { return false; }
    let i = nav_grid_idx(g,x, z);
    g.col.nav.blocked.get(i as usize).copied() != Some(1)
}

/// The space round `p` and its ways out; pure, draws no dice. `reach`: metres of walking followed; `hint`: whose side, with `p` on a line; `ring`:
/// the keys a locked leaf is judged against, default the team's.
pub(super) fn room_read(g: &mut crate::game::Game, p: V3, reach: Option<f64>, hint: Option<V3>, ring: Option<&JsSet<String>>) -> crate::roomread::RoomRead {
    let reach = reach.unwrap_or(9.0);
    let doors = crate::guards::door_cells(g);
    let opens = crate::cover::cover_ready(g);
    let x = Lines { lines: opening_lines(g, &opens, p, reach), opens, doors };
    let Some(from) = start_cell(g, &x, p, hint.unwrap_or(p)) else { return RoomRead { cells: Vec::new(), steps: JsMap::new(), ways: Vec::new() } };
    let (cells, steps, edge) = flooded(g, &x, from, reach);
    let own_ring = match ring { Some(_) => JsSet::new(), None => crate::orders::squad_orders_ring(g, &crate::seats::here(g)) };
    let ring = ring.unwrap_or(&own_ring);
    let mut ways: Vec<RoomWay> = edge.iter().map(|(o, &c)| way_out(g, o, c, p, ring)).collect();
    let tab = &g.tab.cover_opening;
    js::sort_by(&mut ways, |a, b| js::or(a.dist - b.dist, if tab[a.o].id < tab[b.o].id { -1.0 } else { 1.0 }));
    RoomRead { cells, steps, ways }
}

/// Where a space ends: the cells of every door's frame, and of every leafless opening's own line near enough to matter.
struct Lines { opens: Vec<CoverOpeningId>, doors: JsMap<Num, DoorId>, lines: JsMap<Num, CoverOpeningId> }
impl Lines {
    /// A cell of the space: on none of them.
    fn inner(&self, i: f64) -> bool { !self.doors.has(crate::Num::of(i)) && !self.lines.has(crate::Num::of(i)) }
    /// The way out whose line a cell is on.
    fn way(&self, g: &Game, i: f64) -> Option<CoverOpeningId> {
        match self.doors.get(crate::Num::of(i)) {
            Some(&d) => self.opens.iter().copied().find(|&q| g.tab.cover_opening[q].door == Some(d)),
            None => self.lines.get(crate::Num::of(i)).copied(),
        }
    }
}
fn opening_lines(g: &Game, opens: &[CoverOpeningId], p: V3, reach: f64) -> JsMap<Num, CoverOpeningId> {
    let cs = g.col.nav.cell;
    let mut lines: JsMap<Num, CoverOpeningId> = JsMap::new();
    for &o in opens {
        let q = &g.tab.cover_opening[o];
        if q.door.is_some() || v3::dist_xz(q.at, p) > reach + 3.0 { continue; }
        let (tx, tz, half) = (-q.n[1], q.n[0], m::max(0.0, q.half - 0.34));
        let k = m::ceil(half / cs) + 1.0;
        let mut a = -k;
        while a <= k {
            let (x, z) = (q.at[0] + tx * a * cs * 0.5, q.at[2] + tz * a * cs * 0.5);
            if !(m::abs(a * cs * 0.5) > half + 1e-6 || nav_grid_is_blocked(g, x, z)) { lines.set(crate::Num::of(nav_grid_idx(g, m::floor(x / cs), m::floor(z / cs))), o); }
            a += 1.0;
        }
    }
    lines
}
/// Where the walk starts: the point's own cell, else the free cell by it nearest the hint.
fn start_cell(g: &Game, x: &Lines, p: V3, hint: V3) -> Option<f64> {
    let cs = g.col.nav.cell;
    let (sx, sz) = (m::floor(p[0] / cs), m::floor(p[2] / cs));
    let of_space = |x_: f64, z: f64| if free(g, x_, z) { Some(nav_grid_idx(g, x_, z)).filter(|&i| x.inner(i)) } else { None };
    if let Some(i) = of_space(sx, sz) { return Some(i); }
    for r in 1..=3_i32 {
        let mut best: Option<(f64, f64)> = None;
        for dz in -r..=r {
            for dx in -r..=r {
                let (cx, cz) = (sx + dx as f64, sz + dz as f64);
                let Some(i) = of_space(cx, cz) else { continue };
                let d = m::hyp2((cx + 0.5) * cs - hint[0], (cz + 0.5) * cs - hint[2]) + 2.0 * m::hyp2((cx + 0.5) * cs - p[0], (cz + 0.5) * cs - p[2]);
                if best.map_or(true, |b| d < b.0) { best = Some((d, i)); }
            }
        }
        if let Some((_, i)) = best { return Some(i); }
    }
    None
}
/// The space by steps from there, and its cell by each way's line, the first met.
fn flooded(g: &Game, x: &Lines, from: f64, reach: f64) -> (Vec<f64>, JsMap<Num, f64>, JsMap<CoverOpeningId, f64>) {
    let (w, max) = (g.col.nav.w, m::ceil(reach / g.col.nav.cell));
    let mut cells: Vec<f64> = vec![from];
    let mut steps: JsMap<Num, f64> = JsMap::new();
    let mut edge: JsMap<CoverOpeningId, f64> = JsMap::new();
    steps.set(crate::Num::of(from), 0.0);
    let mut at = 0;
    while at < cells.len() {
        let c = cells[at];
        at += 1;
        let (cx, cz) = (js::rem(c, w), js::to_i32(c / w) as f64);
        let s = *steps.get(crate::Num::of(c)).unwrap();
        for (dx, dz) in [(1.0, 0.0), (-1.0, 0.0), (0.0, 1.0), (0.0, -1.0)] {
            if !free(g, cx + dx, cz + dz) { continue; }
            let n = nav_grid_idx(g, cx + dx, cz + dz);
            if let Some(o) = x.way(g, n) { if !edge.has(o) { edge.set(o, c); } continue; }
            if s >= max || steps.has(crate::Num::of(n)) { continue; }
            steps.set(crate::Num::of(n), s + 1.0);
            cells.push(n);
        }
    }
    (cells, steps, edge)
}
/// One way out, from the space's cell by its line: what is beyond it, whether that is known, and whether its leaf is shut or locked to them.
fn way_out(g: &mut Game, o: CoverOpeningId, c: f64, p: V3, ring: &JsSet<String>) -> RoomWay {
    let (w, cs) = (g.col.nav.w, g.col.nav.cell);
    let (at, n, door) = { let q = &g.tab.cover_opening[o]; (q.at, q.n, q.door) };
    let e: V3 = [(js::rem(c, w) + 0.5) * cs, 0.0, (js::to_i32(c / w) as f64 + 0.5) * cs];
    let s = if (e[0] - at[0]) * n[0] + (e[2] - at[2]) * n[1] >= 0.0 { 1.0 } else { -1.0 };
    let in_n = [n[0] * s, n[1] * s];
    let beyond: V3 = [at[0] - in_n[0], 0.0, at[2] - in_n[1]];
    let dist = v3::dist_xz(at, p);
    let Some(d) = door else {
        return RoomWay { o, in_n, beyond, far_known: crate::cleared::cleared_rooms_known_at(g, beyond), shut: false, locked: false, dist };
    };
    let far = crate::cleared::cleared_rooms_room_by(g, d, crate::orders::side_of(g, d, beyond));
    let leaf = &g.dr[d].leaf;
    let shut = !leaf.off && (leaf.latched || m::abs(leaf.angle) < 0.2);
    let kept = shut && leaf.locked && leaf.latched;
    let far_known = crate::cleared::cleared_rooms_is_known(g, far);
    RoomWay { o, in_n, beyond, far_known, shut, locked: kept && crate::doors::doors_locked_out(g, d, e, Some(ring)), dist }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A cell's number on the grid is one key whichever sign its nought has.
    #[test]
    fn a_map_key_holds_minus_zero_as_zero() {
        assert!(crate::Num::of(-0.0) == crate::Num::of(0.0));
    }
}
