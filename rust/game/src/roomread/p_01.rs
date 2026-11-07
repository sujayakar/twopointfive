//! src/game/roomread.ts lines 1 to 81
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

use crate::Num;
use crate::scene_collision::{nav_grid_idx, nav_grid_is_blocked};

/// a Map's key: the language's Map holds −0 and 0 as one key, `Num` tells them apart by bits
#[inline]
fn key(i: f64) -> Num { Num(i + 0.0) }

/// Distance in plan. (roomread.ts flat)
pub fn flat(a: V3, b: V3) -> f64 { m::hyp2(a[0] - b[0], a[2] - b[2]) }

/// a nav cell on the grid and not blocked (roomread.ts roomRead's free)
fn free(g: &mut Game, x: f64, z: f64) -> bool {
    let (w, h) = (g.col.nav.w, g.col.nav.h);
    if !(x >= 0.0 && z >= 0.0 && x < w && z < h) { return false; }
    let i = nav_grid_idx(g,x, z);
    g.col.nav.blocked.get(i as usize).copied() != Some(1)
}

/// a cell of the space: no door's frame and no leafless opening's line (roomread.ts roomRead's inner)
fn inner(i: f64, dc: &JsMap<Num, DoorId>, line_of: &JsMap<Num, CoverOpeningId>) -> bool {
    !dc.has(key(i)) && !line_of.has(key(i))
}

/// The space round `p` and its ways out; pure, draws no dice. `reach`: metres of walking followed; `hint`: whose side, with `p` on a line; `ring`:
/// the keys a locked leaf is judged against, default the team's. (roomread.ts roomRead)
pub fn room_read(g: &mut crate::game::Game, p: V3, reach: Option<f64>, hint: Option<V3>, ring: Option<&JsSet<String>>) -> crate::roomread::RoomRead {
    let reach = reach.unwrap_or(9.0);
    let hint = hint.unwrap_or(p);
    let (w, cs) = (g.col.nav.w, g.col.nav.cell);
    let dc = crate::guards::door_cells(g);
    let opens = crate::cover::cover_ready(g);
    // the cells of every leafless opening's own line, near enough to matter
    let mut line_of: JsMap<Num, CoverOpeningId> = JsMap::new();
    for &o in opens.iter() {
        let (o_at, o_n, o_half, o_door) = { let q = &g.tab.cover_opening[o]; (q.at, q.n, q.half, q.door) };
        if o_door.is_some() || flat(o_at, p) > reach + 3.0 { continue; }
        let (tx, tz, w_) = (-o_n[1], o_n[0], m::max(0.0, o_half - 0.34));
        let k = m::ceil(w_ / cs) + 1.0;
        let mut a = -k;
        while a <= k {
            let x = o_at[0] + tx * a * cs * 0.5;
            let z = o_at[2] + tz * a * cs * 0.5;
            if !(m::abs(a * cs * 0.5) > w_ + 1e-6 || nav_grid_is_blocked(g,x, z)) {
                let i = nav_grid_idx(g,m::floor(x / cs), m::floor(z / cs));
                line_of.set(key(i), o);
            }
            a += 1.0;
        }
    }
    // where the walk starts: the point's own cell, else the free cell by it nearest the hint
    let mut sx = m::floor(p[0] / cs);
    let mut sz = m::floor(p[2] / cs);
    let start_ok = free(g, sx, sz) && { let i = nav_grid_idx(g,sx, sz); inner(i, &dc, &line_of) };
    if !start_ok {
        let mut best = -1.0_f64;
        let mut bd = f64::INFINITY;
        let mut r = 1_i32;
        while r <= 3 && best < 0.0 {
            for dz in -r..=r {
                for dx in -r..=r {
                    let x = sx + dx as f64;
                    let z = sz + dz as f64;
                    if !free(g, x, z) { continue; }
                    let i = nav_grid_idx(g,x, z);
                    if !inner(i, &dc, &line_of) { continue; }
                    let d = m::hyp2((x + 0.5) * cs - hint[0], (z + 0.5) * cs - hint[2]) + 2.0 * m::hyp2((x + 0.5) * cs - p[0], (z + 0.5) * cs - p[2]);
                    if d < bd { bd = d; best = i; }
                }
            }
            r += 1;
        }
        if best < 0.0 {
            let room = crate::cleared::cleared_rooms_room_at(g, p);
            let known = crate::cleared::cleared_rooms_known_at(g, p);
            return RoomRead { at: [p[0], 0.0, p[2]], room, known, cells: Vec::new(), steps: JsMap::new(), ways: Vec::new() };
        }
        sx = js::rem(best, w);
        sz = js::to_i32(best / w) as f64;
    }
    let max = m::ceil(reach / cs);
    let mut steps: JsMap<Num, f64> = JsMap::new();
    let mut cells: Vec<f64> = vec![nav_grid_idx(g,sx, sz)];
    let mut edge: JsMap<CoverOpeningId, f64> = JsMap::new(); // the space's cell by each way's line, the first met
    steps.set(key(cells[0]), 0.0);
    let mut qi = 0;
    while qi < cells.len() {
        let c = cells[qi];
        let x = js::rem(c, w);
        let z = js::to_i32(c / w) as f64;
        let s = *steps.get(key(c)).unwrap();
        for (dx, dz) in [(1.0, 0.0), (-1.0, 0.0), (0.0, 1.0), (0.0, -1.0)] {
            let (nx, nz) = (x + dx, z + dz);
            if !free(g, nx, nz) { continue; }
            let n = nav_grid_idx(g,nx, nz);
            let o = match dc.get(key(n)) {
                Some(&d) => opens.iter().copied().find(|&q| g.tab.cover_opening[q].door == Some(d)),
                None => line_of.get(key(n)).copied(),
            };
            if let Some(o) = o {
                if !edge.has(o) { edge.set(o, c); }
                continue;
            }
            if s >= max || steps.has(key(n)) { continue; }
            steps.set(key(n), s + 1.0);
            cells.push(n);
        }
        qi += 1;
    }
    let own_ring: JsSet<String>;
    let ring: &JsSet<String> = match ring {
        Some(r) => r,
        None => {
            let pl = &g.pl[g.player];
            own_ring = if !pl.codes.is_empty() {
                let mut both = pl.keys.clone();
                for c in pl.codes.iter() { both.add(c); }
                both
            } else {
                pl.keys.clone()
            };
            &own_ring
        }
    };
    let mut ways: Vec<RoomWay> = Vec::new();
    for (o, &c) in edge.iter() {
        let (o_at, o_n, d) = { let q = &g.tab.cover_opening[o]; (q.at, q.n, q.door) };
        let ex = (js::rem(c, w) + 0.5) * cs;
        let ez = (js::to_i32(c / w) as f64 + 0.5) * cs;
        let s = if (ex - o_at[0]) * o_n[0] + (ez - o_at[2]) * o_n[1] >= 0.0 { 1.0 } else { -1.0 };
        let in_n = [o_n[0] * s, o_n[1] * s];
        let beyond: V3 = [o_at[0] - in_n[0], 0.0, o_at[2] - in_n[1]];
        let far = match d {
            Some(d) => { let side = side_sign(&door_face(g, d), beyond); crate::cleared::cleared_rooms_room_by(g, d, side) }
            None => crate::cleared::cleared_rooms_room_at(g, beyond),
        };
        let shut = match d {
            Some(d) => !crate::doors::door_off(g, d) && (crate::doors::door_latched(g, d) || m::abs(crate::doors::door_angle(g, d)) < 0.2),
            None => false,
        };
        let far_known = match d {
            Some(_) => crate::cleared::cleared_rooms_is_known(g, far),
            None => crate::cleared::cleared_rooms_known_at(g, beyond),
        };
        let locked = match d {
            Some(d) => shut && crate::doors::door_locked(g, d) && crate::doors::door_latched(g, d) && crate::doors::doors_locked_out(g, d, [ex, 0.0, ez], Some(ring)),
            None => false,
        };
        ways.push(RoomWay { o, in_n, beyond, far, far_known, shut, locked, dist: flat(o_at, p) });
    }
    let tab = &g.tab.cover_opening;
    js::sort_by(&mut ways, |a, b| js::or(a.dist - b.dist, if tab[a.o].id < tab[b.o].id { -1.0 } else { 1.0 }));
    let room = crate::cleared::cleared_rooms_room_at(g, p);
    let known = crate::cleared::cleared_rooms_known_at(g, p);
    RoomRead { at: [(sx + 0.5) * cs, 0.0, (sz + 0.5) * cs], room, known, cells, steps, ways }
}

/// the part of a door `side_sign` reads (the TypeScript's structural parameter)
fn door_face(g: &Game, d: DoorId) -> SideSignD {
    let def = &g.dr[d].def;
    SideSignD { def: SideSignDDef { closed_dir: def.closed_dir, hinge: SideSignDDefHinge::List(def.hinge) } }
}

/// Which face of a door a point is on, in orders.ts sideOf's sign. (roomread.ts sideSign)
pub fn side_sign(d: &crate::roomread::SideSignD, p: V3) -> f64 {
    let th = d.def.closed_dir;
    let (h0, h1) = match &d.def.hinge {
        SideSignDDefHinge::List(h) => (h[0], h[1]),
        // a short array reads `undefined`, which is NaN in the sums
        SideSignDDefHinge::Array(h) => (h.first().copied().unwrap_or(f64::NAN), h.get(1).copied().unwrap_or(f64::NAN)),
    };
    if m::cos(th) * (p[2] - h1) - m::sin(th) * (p[0] - h0) >= 0.0 { 1.0 } else { -1.0 }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn door(closed_dir: f64, hinge: SideSignDDefHinge) -> SideSignD { SideSignD { def: SideSignDDef { closed_dir, hinge } } }

    #[test]
    fn flat_is_the_typescripts_bits() {
        assert_eq!(flat([1.25, 9.0, -3.7], [17.1, 0.0, 4.4]).to_bits(), 0x4031ccbefe40d45e);
        assert_eq!(flat([0.1, 0.0, 0.2], [0.3, 0.0, 0.7]).to_bits(), 0x3fe13b86ea20ae5c);
    }

    #[test]
    fn side_sign_is_the_typescripts_faces() {
        let up = door(1.5707963267948966, SideSignDDefHinge::List([10.0, 5.0]));
        assert_eq!(side_sign(&up, [9.0, 0.0, 5.0]), 1.0);
        assert_eq!(side_sign(&up, [11.0, 0.0, 5.0]), -1.0);
        // exactly on the line counts as the positive face
        assert_eq!(side_sign(&door(0.0, SideSignDDefHinge::List([10.0, 5.0])), [12.0, 0.0, 5.0]), 1.0);
        // a hinge array too short: NaN in the sum, the negative face
        assert_eq!(side_sign(&door(0.0, SideSignDDefHinge::Array(vec![10.0])), [12.0, 0.0, 5.0]), -1.0);
    }

    #[test]
    fn a_map_key_holds_minus_zero_as_zero() {
        assert!(key(-0.0) == key(0.0));
    }
}
