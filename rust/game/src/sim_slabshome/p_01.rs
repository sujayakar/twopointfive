//! src/sim/slabshome.ts lines 1 to 144
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// The slabs' columns in rust/sim's block for this game's slot. Never held across a call that gets them again.
fn cols(g: &Game) -> &'static mut ::sim::slabs::Slabs {
    &mut ::sim::state::state(g.sim).slabs
}

/// Rows a and b change places in the moved columns; the `row` table is the caller's. (colstore.ts ColStore.swapCols with SLAB_F64, SWAPPED)
fn swap_cols(s: &mut ::sim::slabs::Slabs, a: usize, b: usize) {
    s.px.swap(a, b); s.py.swap(a, b); s.pz.swap(a, b); s.vx.swap(a, b); s.vy.swap(a, b); s.vz.swap(a, b);
    s.a.swap(a, b); s.spin.swap(a, b); s.tip.swap(a, b); s.tip_v.swap(a, b); s.t.swap(a, b); s.ang.swap(a, b); s.wedged.swap(a, b);
    s.bw.swap(a, b); s.bl.swap(a, b); s.bx.swap(a, b); s.by.swap(a, b);
    s.flags.swap(a, b); s.who.swap(a, b); s.door.swap(a, b);
}

/// Slabs in all. (slabshome.ts SlabStore.n)
pub fn slab_store_n(g: &mut crate::game::Game) -> f64 {
    cols(g).n as f64
}

/// How many of them are in the air. (slabshome.ts SlabStore.fly)
pub fn slab_store_fly(g: &mut crate::game::Game) -> f64 {
    cols(g).fly as f64
}

/// Where door `door`'s slab is, or -1. (slabshome.ts SlabStore.rowOf)
pub fn slab_store_row_of(g: &mut crate::game::Game, door: f64) -> f64 {
    cols(g).row[door as usize] as f64 - 1.0
}

/// Whose leaf row `r` is. (slabshome.ts SlabStore.doorAt)
pub fn slab_store_door_at(g: &mut crate::game::Game, r: f64) -> f64 {
    cols(g).door[r as usize] as f64
}

/// Rows a and b change places, every column; the table follows. (slabshome.ts SlabStore.swap)
pub fn slab_store_swap(g: &mut crate::game::Game, a: f64, b: f64) {
    if a == b { return; }
    let (a, b) = (a as usize, b as usize);
    let s = cols(g);
    swap_cols(s, a, b);
    s.row[s.door[a] as usize] = a as u32 + 1;
    s.row[s.door[b] as usize] = b as u32 + 1;
}

/// A leaf comes off: a row for its slab, in its door's place among those in the air, or behind them if it lies at rest from the start. Returns the row. (slabshome.ts SlabStore.enter)
pub fn slab_store_enter(g: &mut crate::game::Game, door: f64, flying: bool) -> f64 {
    if slab_store_row_of(g, door) >= 0.0 { slab_store_leave(g, door); }
    let s = cols(g);
    let (n, fly) = (s.n, s.fly);
    s.door[n as usize] = js::to_u32(door);
    s.row[door as usize] = n + 1;
    s.n = n + 1;
    if !flying { return n as f64; }
    slab_store_swap(g, n as f64, fly as f64);
    cols(g).fly = fly + 1;
    let mut r = fly as usize;
    while r > 0 && cols(g).door[r - 1] as f64 > door {
        slab_store_swap(g, r as f64, (r - 1) as f64);
        r -= 1;
    }
    r as f64
}

/// A slab in the air came to rest: its row goes behind those still in the air, they closing up in their order. (slabshome.ts SlabStore.rest)
pub fn slab_store_rest(g: &mut crate::game::Game, door: f64) {
    let fly = slab_store_fly(g);
    let mut r = slab_store_row_of(g, door);
    if r < 0.0 || r >= fly { return; }
    while r < fly - 1.0 {
        slab_store_swap(g, r, r + 1.0);
        r += 1.0;
    }
    cols(g).fly = (fly - 1.0) as u32;
}

/// A door is hung again: its slab's row is given up and is zeros again. False if it had none. (slabshome.ts SlabStore.leave)
pub fn slab_store_leave(g: &mut crate::game::Game, door: f64) -> bool {
    if slab_store_row_of(g, door) < 0.0 { return false; }
    slab_store_rest(g, door);
    let n = slab_store_n(g);
    let r = slab_store_row_of(g, door);
    slab_store_swap(g, r, n - 1.0);
    let z = (n - 1.0) as usize;
    let s = cols(g);
    s.px[z] = 0.0; s.py[z] = 0.0; s.pz[z] = 0.0; s.vx[z] = 0.0; s.vy[z] = 0.0; s.vz[z] = 0.0;
    s.a[z] = 0.0; s.spin[z] = 0.0; s.tip[z] = 0.0; s.tip_v[z] = 0.0; s.t[z] = 0.0; s.ang[z] = 0.0; s.wedged[z] = 0.0;
    s.bw[z] = 0.0; s.bl[z] = 0.0; s.bx[z] = 0.0; s.by[z] = 0.0;
    s.flags[z] = 0; s.who[z] = 0; s.door[z] = 0; s.row[door as usize] = 0; s.n = z as u32;
    true
}

/// A slab's numbers out of row `r`. (slabshome.ts SlabStore.load)
pub fn slab_store_load(g: &mut crate::game::Game, r: f64) -> crate::sim_slabshome::SlabNow {
    let r = r as usize;
    let s = cols(g);
    let f = s.flags[r];
    SlabNow {
        p: [s.px[r], s.py[r], s.pz[r]],
        v: [s.vx[r], s.vy[r], s.vz[r]],
        a: s.a[r],
        spin: s.spin[r],
        tip: s.tip[r],
        tip_v: s.tip_v[r],
        side: if f & ::sim::slabs::F_BACK != 0 { -1.0 } else { 1.0 },
        rest: f & ::sim::slabs::F_REST != 0,
        landed: f & ::sim::slabs::F_LANDED != 0,
        who: s.who[r] as f64,
        t: s.t[r],
        ang: s.ang[r],
        bits: if f & ::sim::slabs::F_BITS != 0 { Some([s.bw[r], s.bl[r], s.bx[r], s.by[r]]) } else { None },
        wedged: Some(s.wedged[r]),
    }
}

/// A slab's numbers into row `r`; the row stays where it is. (slabshome.ts SlabStore.save)
pub fn slab_store_save(g: &mut crate::game::Game, r: f64, s: crate::sim_slabshome::SlabNow) {
    let r = r as usize;
    let c = cols(g);
    c.px[r] = s.p[0]; c.py[r] = s.p[1]; c.pz[r] = s.p[2]; c.vx[r] = s.v[0]; c.vy[r] = s.v[1]; c.vz[r] = s.v[2];
    c.a[r] = s.a; c.spin[r] = s.spin; c.tip[r] = s.tip; c.tip_v[r] = s.tip_v; c.t[r] = s.t; c.ang[r] = s.ang; c.wedged[r] = s.wedged.unwrap_or(0.0);
    let b = s.bits.unwrap_or([0.0; 4]);
    c.bw[r] = b[0]; c.bl[r] = b[1]; c.bx[r] = b[2]; c.by[r] = b[3];
    c.flags[r] = (if s.rest { ::sim::slabs::F_REST } else { 0 })
        | (if s.landed { ::sim::slabs::F_LANDED } else { 0 })
        | (if s.side < 0.0 { ::sim::slabs::F_BACK } else { 0 })
        | (if s.bits.is_some() { ::sim::slabs::F_BITS } else { 0 });
    c.who[r] = js::to_i32(s.who);
}

#[cfg(test)]
mod tests {
    use super::*;

    /// (door at each live row, row_of each of 16 doors, n, fly)
    fn view(g: &mut Game) -> (Vec<f64>, Vec<f64>, f64, f64) {
        let n = slab_store_n(g);
        let mut doors = Vec::new();
        let mut r = 0.0;
        while r < n { doors.push(slab_store_door_at(g, r)); r += 1.0; }
        let mut tab = Vec::new();
        let mut d = 0.0;
        while d < 16.0 { tab.push(slab_store_row_of(g, d)); d += 1.0; }
        (doors, tab, n, slab_store_fly(g))
    }

    fn mark(g: &mut Game, door: f64, x: f64) {
        let r = slab_store_row_of(g, door);
        let side = if door as i32 % 2 != 0 { -1.0 } else { 1.0 };
        let s = SlabNow { p: [x, x + 0.5, x + 0.25], v: [-x, 0.0, 1.0], a: x * 2.0, spin: 3.0, tip: 4.0, tip_v: 5.0, side, rest: false, landed: door as i32 % 3 == 0, who: -1.0, t: x, ang: 6.0, bits: None, wedged: None };
        slab_store_save(g, r, s);
    }

    #[track_caller]
    fn says(g: &mut Game, doors: &[f64], tab: [f64; 16], n: f64, fly: f64) {
        let (d, t, nn, ff) = view(g);
        assert_eq!(d, doors);
        assert_eq!(t, tab);
        assert_eq!((nn, ff), (n, fly));
    }

    #[test]
    fn rows_move_as_the_typescripts_do() {
        let g = &mut Game::blank();
        g.sim = 3; // (the only test on this slot)
        const N: f64 = -1.0;

        assert_eq!(slab_store_enter(g, 5.0, true), 0.0); mark(g, 5.0, 5.0);
        says(g, &[5.0], [N, N, N, N, N, 0.0, N, N, N, N, N, N, N, N, N, N], 1.0, 1.0);
        assert_eq!(slab_store_enter(g, 2.0, true), 0.0); mark(g, 2.0, 2.0);
        says(g, &[2.0, 5.0], [N, N, 0.0, N, N, 1.0, N, N, N, N, N, N, N, N, N, N], 2.0, 2.0);
        assert_eq!(slab_store_enter(g, 9.0, false), 2.0); mark(g, 9.0, 9.0);
        says(g, &[2.0, 5.0, 9.0], [N, N, 0.0, N, N, 1.0, N, N, N, 2.0, N, N, N, N, N, N], 3.0, 2.0);
        assert_eq!(slab_store_enter(g, 3.0, true), 1.0); mark(g, 3.0, 3.0);
        says(g, &[2.0, 3.0, 5.0, 9.0], [N, N, 0.0, 1.0, N, 2.0, N, N, N, 3.0, N, N, N, N, N, N], 4.0, 3.0);
        slab_store_rest(g, 3.0);
        says(g, &[2.0, 5.0, 3.0, 9.0], [N, N, 0.0, 2.0, N, 1.0, N, N, N, 3.0, N, N, N, N, N, N], 4.0, 2.0);
        slab_store_rest(g, 9.0);
        says(g, &[2.0, 5.0, 3.0, 9.0], [N, N, 0.0, 2.0, N, 1.0, N, N, N, 3.0, N, N, N, N, N, N], 4.0, 2.0);
        assert!(slab_store_leave(g, 2.0));
        says(g, &[5.0, 9.0, 3.0], [N, N, N, 2.0, N, 0.0, N, N, N, 1.0, N, N, N, N, N, N], 3.0, 1.0);
        assert!(!slab_store_leave(g, 2.0));
        says(g, &[5.0, 9.0, 3.0], [N, N, N, 2.0, N, 0.0, N, N, N, 1.0, N, N, N, N, N, N], 3.0, 1.0);
        assert_eq!(slab_store_enter(g, 9.0, true), 1.0); mark(g, 9.0, 9.0);
        says(g, &[5.0, 9.0, 3.0], [N, N, N, 2.0, N, 0.0, N, N, N, 1.0, N, N, N, N, N, N], 3.0, 2.0);
        assert_eq!(slab_store_enter(g, 1.0, false), 3.0);
        says(g, &[5.0, 9.0, 3.0, 1.0], [N, 3.0, N, 2.0, N, 0.0, N, N, N, 1.0, N, N, N, N, N, N], 4.0, 2.0);

        // each slab's numbers went with its door through the swaps
        let r = slab_store_row_of(g, 5.0);
        let s = slab_store_load(g, r);
        assert_eq!((s.p, s.v, s.a, s.t, s.who, s.side, s.landed, s.rest), ([5.0, 5.5, 5.25], [-5.0, 0.0, 1.0], 10.0, 5.0, -1.0, -1.0, false, false));
        assert_eq!((s.spin, s.tip, s.tip_v, s.ang, s.wedged, s.bits), (3.0, 4.0, 5.0, 6.0, Some(0.0), None));
        let r = slab_store_row_of(g, 3.0);
        let s = slab_store_load(g, r);
        assert_eq!((s.p, s.a, s.landed, s.side), ([3.0, 3.5, 3.25], 6.0, true, -1.0));
        // a row given up is zeros, and an entered one's numbers are `save`'s to write
        let r = slab_store_row_of(g, 1.0);
        let s = slab_store_load(g, r);
        assert_eq!((s.p, s.a, s.who, s.side, s.ang, s.wedged), ([0.0; 3], 0.0, 0.0, 1.0, 0.0, Some(0.0)));

        let all = SlabNow { p: [1.0, 2.0, 3.0], v: [4.0, 5.0, 6.0], a: 7.0, spin: 8.0, tip: 9.0, tip_v: 10.0, side: -1.0, rest: true, landed: true, who: 7.0, t: 11.0, ang: 12.0, bits: Some([0.1, 0.2, 0.3, 0.4]), wedged: Some(13.0) };
        slab_store_save(g, 0.0, all);
        assert_eq!(cols(g).flags[0], 15);
        let s = slab_store_load(g, 0.0);
        assert_eq!((s.p, s.v, s.a, s.spin, s.tip, s.tip_v), (all.p, all.v, 7.0, 8.0, 9.0, 10.0));
        assert_eq!((s.side, s.rest, s.landed, s.who, s.t, s.ang, s.wedged, s.bits), (-1.0, true, true, 7.0, 11.0, 12.0, Some(13.0), all.bits));
    }

}
