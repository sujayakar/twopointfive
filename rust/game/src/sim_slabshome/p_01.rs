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

/// The column starts in SLAB_F64 then SLAB_U32 order, as `names` in slab_io walks them.
fn in_order(o: &SlabCols) -> [f64; 21] {
    [o.px, o.py, o.pz, o.vx, o.vy, o.vz, o.a, o.spin, o.tip, o.tip_v, o.t, o.ang, o.wedged, o.bw, o.bl, o.bx, o.by, o.flags, o.who, o.door, o.row]
}

/// The store's own copy of the layout. (colstore.ts ColStore.O)
fn store_o(c: &SlabCols) -> SlabStoreO {
    SlabStoreO {
        door: c.door, t: c.t, a: c.a, flags: c.flags, bytes: c.bytes, who: c.who, vx: c.vx, vz: c.vz, row: c.row, by: c.by, spin: c.spin,
        px: c.px, py: c.py, pz: c.pz, vy: c.vy, wedged: c.wedged, tip: c.tip, tip_v: c.tip_v, ang: c.ang, bw: c.bw, bl: c.bl, bx: c.bx,
    }
}

/// A family's state stays in the block for the game's life; one store a part. (state.ts SimState.keep; `born` is the slot's owner's, see notes)
fn keep_home(g: &Game, bit: u32, name: &str) {
    let head = &mut ::sim::state::state(g.sim).head;
    if head.homes & bit != 0 { panic!("two stores of the {} over one game's block: a part of a block is one store's", name); }
    head.homes |= bit;
}

/// Rows a and b change places in the moved columns; the `row` table is the caller's. (colstore.ts ColStore.swapCols with SLAB_F64, SWAPPED)
fn swap_cols(s: &mut ::sim::slabs::Slabs, a: usize, b: usize) {
    s.px.swap(a, b); s.py.swap(a, b); s.pz.swap(a, b); s.vx.swap(a, b); s.vy.swap(a, b); s.vz.swap(a, b);
    s.a.swap(a, b); s.spin.swap(a, b); s.tip.swap(a, b); s.tip_v.swap(a, b); s.t.swap(a, b); s.ang.swap(a, b); s.wedged.swap(a, b);
    s.bw.swap(a, b); s.bl.swap(a, b); s.bx.swap(a, b); s.by.swap(a, b);
    s.flags.swap(a, b); s.who.swap(a, b); s.door.swap(a, b);
}

/// Where each column starts for columns `cap` rows long: doubles index doubles, words index words, from the part's start. (slabshome.ts slabCols, colstore.ts colsOf)
pub fn slab_cols(cap: f64) -> crate::sim_slabshome::SlabCols {
    let mut o = SlabCols::zeroed();
    let mut at = SLAB_HEAD;
    for k in [&mut o.px, &mut o.py, &mut o.pz, &mut o.vx, &mut o.vy, &mut o.vz, &mut o.a, &mut o.spin, &mut o.tip, &mut o.tip_v, &mut o.t, &mut o.ang, &mut o.wedged, &mut o.bw, &mut o.bl, &mut o.bx, &mut o.by] {
        *k = at / 8.0;
        at += cap * 8.0;
    }
    for k in [&mut o.flags, &mut o.who, &mut o.door, &mut o.row] {
        *k = at / 4.0;
        at += cap * 4.0;
    }
    o.bytes = at;
    o
}

/// The columns of a level's slabs, over its game's block; a level of more doors than SLAB_CAP is not loaded. (slabshome.ts SlabStore.constructor)
pub fn slab_store_new(g: &mut crate::game::Game, st: crate::Opaque, doors: f64) -> crate::sim_slabshome::SlabStore {
    if doors > SLAB_CAP {
        panic!("a level of {} doors: the game holds {} leaves off their hinges (sim/slabshome.ts SLAB_CAP, rust/sim/src/slabs.rs CAP)", doors, SLAB_CAP);
    }
    let o = store_o(&slab_cols(SLAB_CAP));
    keep_home(g, js::to_u32(SLAB_BIT), "leaves off their hinges");
    let at = crate::sim_state::sim_part(crate::sim_state::P_SLABS);
    SlabStore { doors, o, home: st, b8: (js::to_u32(at) >> 3) as f64, b4: (js::to_u32(at) >> 2) as f64, cap: SLAB_CAP }
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

/// Where the hand-over is, asked once, after checking this file's layout is the module's own. (slabshome.ts slabIo)
pub fn slab_io(g: &mut crate::game::Game) -> crate::sim_slabshome::SlabIo {
    if let Some(io) = g.m.sim_slabshome.io { return io; }
    let x = crate::sim_sim::sim();
    let o = slab_cols(SLAB_CAP);
    let at = in_order(&o);
    let doubles = slab_f64().len();
    let names: Vec<String> = slab_f64().into_iter().chain(slab_u32()).collect();
    let mut off: Vec<String> = Vec::new();
    for (i, k) in names.iter().enumerate() {
        let want = if i < doubles { at[i] * 8.0 } else { at[i] * 4.0 };
        if ::sim::slabs::cap(100 + i as u32) as f64 != want { off.push(k.clone()); }
    }
    let cap = |t: u32| ::sim::slabs::cap(t) as f64;
    if cap(0) != SLAB_CAP || cap(13) != o.bytes || !off.is_empty() || cap(100 + names.len() as u32) != 0.0 || cap(1) != SLAB_PARS {
        let where_ = if off.is_empty() { String::new() } else { format!(" (columns not where the module has them: {})", off.join(", ")) };
        panic!("the Rust module lays the slabs out as {} rows in {} bytes and src/sim/slabshome.ts as {} in {}{}, or the hand-over is not the same length: bun run tools/sim/build.ts", cap(0), cap(13), SLAB_CAP, o.bytes, where_);
    }
    let io = SlabIo { x, par: (::sim::slabs::addr(0) / 8) as f64 };
    g.m.sim_slabshome.io = Some(io);
    io
}

/// Word `i` of a store's bytes. (the Uint32Array views of slabDiff)
fn word(a: &[u8], i: usize) -> u32 {
    let at = i * 4;
    u32::from_le_bytes([a[at], a[at + 1], a[at + 2], a[at + 3]])
}

/// colDiff over the moved columns, naming a row `door ${u[O.door + r]}`, appending to `out`.
fn seam_col_diff(g: &mut Game, a: &[u8], b: &[u8], rows: f64, most: f64, out: Vec<String>) -> Vec<String> {
    todo!("needs: sim_colstore::col_diff's ColDiffO to name its columns (it holds only `bytes`) (slabshome.ts:142)")
}

/// Where two takes of a store's bytes differ, in words: at most `most` lines, none if they are the same state. (slabshome.ts slabDiff)
pub fn slab_diff(g: &mut crate::game::Game, a: &[u8], b: &[u8], most: Option<f64>) -> Vec<String> {
    let most = most.unwrap_or(4.0);
    let (o_row, doors) = (g.doors.slabs.o.row, g.doors.slabs.doors);
    let (ua0, ub0) = (word(a, 0), word(b, 0));
    let mut out: Vec<String> = Vec::new();
    if ua0 != ub0 || word(a, 1) != word(b, 1) {
        out.push(format!("{} slabs, {} in the air | {}, {}", ua0, word(a, 1), ub0, word(b, 1)));
    }
    let mut k = 0.0;
    while k < doors && (out.len() as f64) < most {
        let at = (o_row + k) as usize;
        if word(a, at) != word(b, at) {
            out.push(format!("door {}'s row (plus one) {} | {}", k as usize, word(a, at), word(b, at)));
        }
        k += 1.0;
    }
    seam_col_diff(g, a, b, m::max(ua0 as f64, ub0 as f64), most, out)
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

    // The numbers below are the TypeScript store's own, printed under bun.
    #[test]
    fn cols_are_the_typescripts_and_the_modules() {
        let o = slab_cols(64.0);
        let want = [2.0, 66.0, 130.0, 194.0, 258.0, 322.0, 386.0, 450.0, 514.0, 578.0, 642.0, 706.0, 770.0, 834.0, 898.0, 962.0, 1026.0, 2180.0, 2244.0, 2308.0, 2372.0];
        assert_eq!(in_order(&o), want);
        assert_eq!(o.bytes, 9744.0);
        let small = slab_cols(3.0);
        assert_eq!(in_order(&small), [2.0, 5.0, 8.0, 11.0, 14.0, 17.0, 20.0, 23.0, 26.0, 29.0, 32.0, 35.0, 38.0, 41.0, 44.0, 47.0, 50.0, 106.0, 109.0, 112.0, 115.0]);
        assert_eq!(small.bytes, 472.0);
        // the module's own layout, as slab_io holds it to
        for (i, c) in in_order(&o).iter().enumerate() {
            let scale = if i < 17 { 8.0 } else { 4.0 };
            assert_eq!(::sim::slabs::COLS[i] as f64, c * scale);
        }
        assert_eq!(::sim::slabs::cap(13) as f64, o.bytes);
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
