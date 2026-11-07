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
