//! src/sim/itemshome.ts lines 1 to 145
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

// The columns' places, as IC numbers them: the doubles 0 .. 19, then the words.
const COLS_F: usize = NF as usize;
const COLS_ALL: usize = NC as usize;

/// A double column of rust/sim's part by its place (ITEM_F64's order, which is COLS's).
fn double_col(c: &mut sim::items::Items, k: usize) -> &mut [f64; sim::items::CAP] {
    match k {
        0 => &mut c.px,
        1 => &mut c.py,
        2 => &mut c.pz,
        3 => &mut c.vx,
        4 => &mut c.vy,
        5 => &mut c.vz,
        6 => &mut c.yaw,
        7 => &mut c.spin,
        8 => &mut c.tumble,
        9 => &mut c.tumble_rate,
        10 => &mut c.age,
        11 => &mut c.life,
        12 => &mut c.radius,
        13 => &mut c.restitution,
        14 => &mut c.friction,
        15 => &mut c.heading,
        16 => &mut c.on_pose,
        17 => &mut c.fuse,
        18 => &mut c.emit_t,
        19 => &mut c.fill,
        _ => unreachable!("no double column {}", k),
    }
}

/// A word at its place (ITEM_U32's order, numbered from COLS_F), read as the TypeScript's Uint32Array reads it: `who` is an i32 kept in a word, so -1 is 4294967295.
fn word_at(c: &sim::items::Items, k: usize, r: usize) -> u32 {
    match k {
        20 => c.flags[r],
        21 => c.kind[r],
        22 => c.bounces[r],
        23 => c.who[r] as u32,
        24 => c.on[r],
        25 => c.serial[r],
        _ => unreachable!("no word column {}", k),
    }
}

/// …and written.
fn set_word_at(c: &mut sim::items::Items, k: usize, r: usize, v: u32) {
    match k {
        20 => c.flags[r] = v,
        21 => c.kind[r] = v,
        22 => c.bounces[r] = v,
        23 => c.who[r] = v as i32,
        24 => c.on[r] = v,
        25 => c.serial[r] = v,
        _ => unreachable!("no word column {}", k),
    }
}

/// The store's `O` from the columns' starts (the same numbers, two generated types).
fn o_of(c: ItemCols) -> ItemStoreO {
    ItemStoreO {
        kind: c.kind, fill: c.fill, radius: c.radius, yaw: c.yaw, flags: c.flags, bytes: c.bytes, who: c.who, vx: c.vx, vz: c.vz, serial: c.serial,
        fuse: c.fuse, on: c.on, restitution: c.restitution, friction: c.friction, spin: c.spin, age: c.age, life: c.life, tumble: c.tumble,
        tumble_rate: c.tumble_rate, emit_t: c.emit_t, bounces: c.bounces, heading: c.heading, on_pose: c.on_pose, px: c.px, py: c.py, pz: c.pz, vy: c.vy,
    }
}

/// Where each column starts, by its place (the body of ItemStore.places, usable before the store stands at its path).
fn places_of(o: &ItemStoreO) -> Vec<f64> {
    vec![
        o.px, o.py, o.pz, o.vx, o.vy, o.vz, o.yaw, o.spin, o.tumble, o.tumble_rate, o.age, o.life, o.radius, o.restitution, o.friction, o.heading,
        o.on_pose, o.fuse, o.emit_t, o.fill, o.flags, o.kind, o.bounces, o.who, o.on, o.serial,
    ]
}

/// Where each column starts and how long the part is. (itemshome.ts itemCols)
pub fn item_cols() -> crate::sim_itemshome::ItemCols {
    // colsOf(ITEM_HEAD, ITEM_F64, ITEM_U32, ITEM_ROWS), walked here: the skeleton's cols_of hands back only `bytes`.
    let mut s = [0.0; COLS_ALL];
    let mut at = ITEM_HEAD;
    for c in 0..COLS_F { s[c] = at / 8.0; at += ITEM_ROWS * 8.0; }
    for c in COLS_F..COLS_ALL { s[c] = at / 4.0; at += ITEM_ROWS * 4.0; }
    ItemCols {
        px: s[0], py: s[1], pz: s[2], vx: s[3], vy: s[4], vz: s[5], yaw: s[6], spin: s[7], tumble: s[8], tumble_rate: s[9], age: s[10], life: s[11],
        radius: s[12], restitution: s[13], friction: s[14], heading: s[15], on_pose: s[16], fuse: s[17], emit_t: s[18], fill: s[19],
        flags: s[20], kind: s[21], bounces: s[22], who: s[23], on: s[24], serial: s[25],
        bytes: at,
    }
}

/// The columns of one pool over its game's block; ColStore's constructor folded in. (itemshome.ts ItemStore.constructor)
pub fn item_store_new(g: &mut crate::game::Game, st: crate::Opaque) -> crate::sim_itemshome::ItemStore {
    // `st.keep(ITEM_BIT, 'thrown things')` is not in the game (notInTheGame.txt); rust/sim's State holds the part from the start.
    let o = o_of(item_cols());
    let part = crate::sim_state::sim_part(crate::sim_state::P_ITEMS);
    ItemStore {
        at: places_of(&o),
        o,
        home: st,
        b8: (js::to_u32(part) >> 3) as f64,
        b4: (js::to_u32(part) >> 2) as f64,
        cap: ITEM_ROWS,
    }
}

/// Where each column starts, by its place (IC). (itemshome.ts ItemStore.places)
pub fn item_store_places(g: &mut crate::game::Game) -> Vec<f64> {
    places_of(&g.items.store.o)
}

/// `n` rows fit, or it is a fault. (itemshome.ts ItemStore.room)
pub fn item_store_room(g: &mut crate::game::Game, n: f64) {
    if n > g.items.store.cap {
        panic!("a pool of {} thrown things: its columns hold {} (sim/itemshome.ts ITEM_ROWS, rust/sim/src/items.rs CAP; game/throwables.ts spawn gives one up for each new one past {})", n, g.items.store.cap, ITEM_CAP);
    }
}

/// Things in the pool. (itemshome.ts ItemStore.n)
pub fn item_store_n(g: &mut crate::game::Game) -> f64 {
    sim::state::state(g.sim).items.n as f64
}

/// One more thing, behind the last: its row (zeros, the caller's to write). (itemshome.ts ItemStore.enter)
pub fn item_store_enter(g: &mut crate::game::Game) -> f64 {
    let n = item_store_n(g);
    item_store_room(g, n + 1.0);
    sim::state::state(g.sim).items.n = js::to_u32(n + 1.0);
    n
}

/// Row `r`'s numbers, by their places (IC). (itemshome.ts ItemStore.rowOut)
pub fn item_store_row_out(g: &mut crate::game::Game, r: f64) -> Vec<f64> {
    let c = &mut sim::state::state(g.sim).items;
    let row = r as usize;
    let mut out = vec![0.0; COLS_ALL];
    for k in 0..COLS_F { out[k] = double_col(c, k)[row]; }
    for k in COLS_F..COLS_ALL { out[k] = word_at(c, k, row) as f64; }
    out
}

/// …and into it; a word takes its number as a Uint32Array does. (itemshome.ts ItemStore.rowIn)
pub fn item_store_row_in(g: &mut crate::game::Game, r: f64, v: &[f64]) {
    let c = &mut sim::state::state(g.sim).items;
    let row = r as usize;
    for k in 0..COLS_F { double_col(c, k)[row] = v[k]; }
    for k in COLS_F..COLS_ALL { set_word_at(c, k, row, js::to_u32(v[k])); }
}

/// A row is given up: those behind it close up in their order, the last is zeros again. (itemshome.ts ItemStore.close)
pub fn item_store_close(g: &mut crate::game::Game, r: f64) {
    let n = item_store_n(g);
    if r < 0.0 || r >= n { return; }
    let c = &mut sim::state::state(g.sim).items;
    let (row, count) = (r as usize, n as usize);
    for k in 0..COLS_F {
        let col = double_col(c, k);
        col.copy_within(row + 1..count, row);
        col[count - 1] = 0.0;
    }
    for k in COLS_F..COLS_ALL {
        for i in row + 1..count {
            let w = word_at(c, k, i);
            set_word_at(c, k, i - 1, w);
        }
        set_word_at(c, k, count - 1, 0);
    }
    c.n = js::to_u32(n - 1.0);
}

/// The pool laid out anew: these rows, in this order, and zeros behind them. (itemshome.ts ItemStore.lay)
pub fn item_store_lay(g: &mut crate::game::Game, rows: &[Vec<f64>]) {
    let was = item_store_n(g) as usize;
    item_store_room(g, rows.len() as f64);
    for (r, row) in rows.iter().enumerate() {
        item_store_row_in(g, r as f64, row);
    }
    let c = &mut sim::state::state(g.sim).items;
    for r in rows.len()..was {
        for k in 0..COLS_F { double_col(c, k)[r] = 0.0; }
        for k in COLS_F..COLS_ALL { set_word_at(c, k, r, 0); }
    }
    c.n = rows.len() as u32;
}

/// Where the hand-over's tables are in the module's memory, asked once; the layout here is held to rust/sim's. (itemshome.ts itemIo)
pub fn item_io(g: &mut crate::game::Game) -> crate::sim_itemshome::ItemIo {
    if let Some(io) = g.m.sim_itemshome.io { return io; }
    let x = crate::sim_sim::sim();
    let o = item_cols();
    let at = places_of(&o_of(o));
    let names: Vec<String> = item_f64().into_iter().chain(item_u32()).collect();
    let mut off: Vec<&str> = Vec::new();
    for (i, k) in names.iter().enumerate() {
        let here = if i < COLS_F { at[i] * 8.0 } else { at[i] * 4.0 };
        if sim::items::cap(100 + i as u32) as f64 != here { off.push(k.as_str()); }
    }
    if sim::items::cap(0) as f64 != ITEM_ROWS
        || sim::items::cap(13) as f64 != o.bytes
        || !off.is_empty()
        || sim::items::cap(100 + names.len() as u32) != 0
        || sim::items::cap(1) as f64 != ITEM_PARS
        || sim::items::cap(4) as f64 != ITEM_ROOFS
        || sim::items::cap(3) as f64 != ITEM_EV
        || sim::items::cap(2) as f64 != ITEM_ROWS * ITEM_ROW_EVENTS
    {
        let columns = if off.is_empty() { String::new() } else { format!(" (columns not where the module has them: {})", off.join(", ")) };
        panic!("the Rust module lays the thrown things out as {} rows in {} bytes and src/sim/itemshome.ts as {} in {}{}, or the hand-over is not the same shape: bun run tools/sim/build.ts", sim::items::cap(0), sim::items::cap(13), ITEM_ROWS, o.bytes, columns);
    }
    let io = ItemIo { x, par: (sim::items::addr(0) / 8) as f64, ev: (sim::items::addr(1) / 8) as f64, max_events: sim::items::cap(2) as f64 };
    g.m.sim_itemshome.io = Some(io);
    io
}

/// Where two takes of the store's bytes differ, in words: at most `most` lines, none if they are the same state. (itemshome.ts itemDiff)
pub fn item_diff(g: &mut crate::game::Game, a: &[u8], b: &[u8], most: Option<f64>) -> Vec<String> {
    let most = most.unwrap_or(4.0);
    let o = g.items.store.o;
    let ua0 = u32::from_ne_bytes([a[0], a[1], a[2], a[3]]);
    let ub0 = u32::from_ne_bytes([b[0], b[1], b[2], b[3]]);
    let mut out: Vec<String> = Vec::new();
    if ua0 != ub0 { out.push(format!("{} things in the pool | {}", ua0, ub0)); }
    let serial = o.serial;
    let mut who = |u: Vec<u32>, r: f64| format!("serial {}", u[(serial + r) as usize]);
    seam_col_diff(o, a, b, m::max(ua0 as f64, ub0 as f64), &mut who, most, &mut out)
}

/// colDiff(O, ITEM_F64, ITEM_U32, a, b, rows, who, most, out) (itemshome.ts:143): the skeleton's ColDiffO carries only `bytes`, so the columns' starts cannot reach it.
fn seam_col_diff(o: ItemStoreO, a: &[u8], b: &[u8], rows: f64, who: &mut dyn FnMut(Vec<u32>, f64) -> String, most: f64, out: &mut Vec<String>) -> Vec<String> {
    todo!("needs: sim_colstore::ColDiffO to carry every column's start (it has only bytes)")
}

#[cfg(test)]
mod tests {
    extern crate std;
    use super::*;

    /// a row with every column its own number, who = -1 and fractions and a negative zero among the doubles
    fn row(tag: f64) -> Vec<f64> {
        let mut v: Vec<f64> = (0..COLS_ALL).map(|k| tag * 100.0 + k as f64).collect();
        v[3] = -0.0;
        v[4] = tag + 0.25;
        v[23] = -1.0;
        v
    }

    #[test]
    fn the_columns_stand_where_rust_sim_has_them() {
        let o = item_cols();
        let at = places_of(&o_of(o));
        for k in 0..COLS_ALL {
            let bytes = if k < COLS_F { at[k] * 8.0 } else { at[k] * 4.0 };
            assert_eq!(bytes as usize, sim::items::COLS[k], "column {}", k);
        }
        assert_eq!(o.bytes as usize, core::mem::size_of::<sim::items::Items>());
        assert_eq!(item_f64().len(), COLS_F);
        assert_eq!(item_f64().len() + item_u32().len(), COLS_ALL);
    }

    // (one test: every step below works on the same slot of rust/sim's block, a process-wide static)
    #[test]
    fn a_pool_enters_closes_and_lays() {
        let mut g = Box::new(crate::game::Game::blank());
        g.sim = 0;
        g.items.store.cap = ITEM_ROWS;
        sim::state::state(g.sim).items.n = 0;

        // enter: rows by count, and row_in/row_out hold every number by its place
        assert_eq!(item_store_enter(&mut g), 0.0);
        assert_eq!(item_store_enter(&mut g), 1.0);
        assert_eq!(item_store_enter(&mut g), 2.0);
        assert_eq!(item_store_n(&mut g), 3.0);
        for r in 0..3 { item_store_row_in(&mut g, r as f64, &row(r as f64 + 1.0)); }
        let out = item_store_row_out(&mut g, 1.0);
        assert_eq!(out[0], 200.0);
        assert_eq!(out[3].to_bits(), (-0.0f64).to_bits());
        assert_eq!(out[4], 2.25);
        assert_eq!(out[23], 4294967295.0, "`who` reads as a Uint32Array's word");
        assert_eq!(out[25], 225.0);
        assert_eq!(sim::state::state(g.sim).items.who[1], -1);

        // close: a row out of range is nothing; the rows behind close up, the last is zeros
        item_store_close(&mut g, 3.0);
        item_store_close(&mut g, -1.0);
        assert_eq!(item_store_n(&mut g), 3.0);
        item_store_close(&mut g, 0.0);
        assert_eq!(item_store_n(&mut g), 2.0);
        assert_eq!(item_store_row_out(&mut g, 0.0)[0], 200.0);
        assert_eq!(item_store_row_out(&mut g, 1.0)[25], 325.0);
        let gone = item_store_row_out(&mut g, 2.0);
        assert!(gone.iter().all(|x| x.to_bits() == 0), "{:?}", gone);

        // lay: fewer rows than there were, zeros behind them
        item_store_lay(&mut g, &[row(9.0)]);
        assert_eq!(item_store_n(&mut g), 1.0);
        assert_eq!(item_store_row_out(&mut g, 0.0)[0], 900.0);
        assert!(item_store_row_out(&mut g, 1.0).iter().all(|x| x.to_bits() == 0));

        // room: a pool past its columns is a fault
        let many: Vec<Vec<f64>> = (0..ITEM_ROWS as usize + 1).map(|r| row(r as f64)).collect();
        let g2 = &mut *g;
        let fault = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| item_store_lay(g2, &many)));
        assert!(fault.is_err());
        assert_eq!(item_store_n(&mut g), 1.0, "nothing laid by a pool that does not fit");
    }
}
