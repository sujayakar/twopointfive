pub use ::game_types::sim_itemshome::types;
pub use types::*;
use crate::game::Game;
use crate::js::{self, JsMap, JsSet};
use crate::m;
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

/// Where each column starts and how long the part is.
pub(in crate::air_and_light) fn item_cols() -> crate::sim_itemshome::ItemCols {
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

/// The columns of one pool over its game's block; ColStore's constructor folded in.
pub fn item_store_new(g: &crate::game::Game, st: crate::Opaque) -> crate::sim_itemshome::ItemStore {
    // `st.keep(ITEM_BIT, 'thrown things')` is not in the game (notInTheGame.txt); rust/sim's State holds the part from the start.
    let o = o_of(item_cols());
    let part = crate::sim_state::sim_part(crate::sim_state::P_ITEMS);
    ItemStore {
        o,
        cap: ITEM_ROWS,
    }
}

/// `n` rows fit, or it is a fault.
pub(in crate::air_and_light) fn item_store_room(g: &crate::game::Game, n: f64) {
    if n > g.items.store.cap {
        panic!("a pool of {} thrown things: its columns hold {} (sim/itemshome.ts ITEM_ROWS, rust/sim/src/items.rs CAP; game/throwables.ts spawn gives one up for each new one past {})", n, g.items.store.cap, ITEM_CAP);
    }
}

/// Things in the pool.
pub(in crate::air_and_light) fn item_store_n(g: &crate::game::Game) -> f64 {
    sim::state::state_read(g.sim).items.n as f64
}

/// One more thing, behind the last: its row (zeros, the caller's to write).
pub fn item_store_enter(g: &crate::game::Game) -> f64 {
    let n = item_store_n(g);
    item_store_room(g, n + 1.0);
    sim::state::state(g.sim).items.n = js::to_u32(n + 1.0);
    n
}

/// Row `r`'s numbers, by their places (IC).
pub fn item_store_row_out(g: &crate::game::Game, r: f64) -> Vec<f64> {
    let c = &mut sim::state::state(g.sim).items;
    let row = r as usize;
    let mut out = vec![0.0; COLS_ALL];
    for k in 0..COLS_F { out[k] = double_col(c, k)[row]; }
    for k in COLS_F..COLS_ALL { out[k] = word_at(c, k, row) as f64; }
    out
}

/// …and into it; a word takes its number as a Uint32Array does.
pub(in crate::air_and_light) fn item_store_row_in(g: &crate::game::Game, r: f64, v: &[f64]) {
    let c = &mut sim::state::state(g.sim).items;
    let row = r as usize;
    for k in 0..COLS_F { double_col(c, k)[row] = v[k]; }
    for k in COLS_F..COLS_ALL { set_word_at(c, k, row, js::to_u32(v[k])); }
}

/// A row is given up: those behind it close up in their order, the last is zeros again.
pub fn item_store_close(g: &crate::game::Game, r: f64) {
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

/// The pool laid out anew: these rows, in this order, and zeros behind them.
pub fn item_store_lay(g: &crate::game::Game, rows: &[Vec<f64>]) {
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

/// A double of a take of the store's bytes, by its index into the doubles.
fn double_of(b: &[u8], i: usize) -> f64 {
    f64::from_ne_bytes(b[i * 8..i * 8 + 8].try_into().unwrap())
}

/// A word of a take of the store's bytes, by its index into the words.
fn word_of(b: &[u8], i: usize) -> u32 {
    u32::from_ne_bytes(b[i * 4..i * 4 + 4].try_into().unwrap())
}

/// Object.is of two doubles: a NaN is a NaN whatever its bits, -0 is not 0.
fn same_double(x: f64, y: f64) -> bool {
    (x.is_nan() && y.is_nan()) || x.to_bits() == y.to_bits()
}

/// Where two takes of the store's bytes differ, in words: at most `most` lines, none if they are the same state.
pub(in crate::air_and_light) fn item_diff(g: &crate::game::Game, a: &[u8], b: &[u8], most: Option<f64>) -> Vec<String> {
    let most = most.unwrap_or(4.0);
    let o = g.items.store.o;
    let (ua0, ub0) = (word_of(a, 0), word_of(b, 0));
    let mut out: Vec<String> = Vec::new();
    if ua0 != ub0 { out.push(format!("{} things in the pool | {}", ua0, ub0)); }
    col_diff_items(&o, a, b, m::max(ua0 as f64, ub0 as f64) as usize, most, &mut out);
    out
}

/// colDiff(O, ITEM_F64, ITEM_U32, a, b, rows, who, most, out) over this store's columns, `who` being the row's serial.
fn col_diff_items(o: &ItemStoreO, a: &[u8], b: &[u8], rows: usize, most: f64, out: &mut Vec<String>) {
    let at = places_of(o);
    for (c, k) in item_f64().iter().enumerate() {
        let col = at[c] as usize;
        let mut r = 0;
        while r < rows && (out.len() as f64) < most {
            let (x, y) = (double_of(a, col + r), double_of(b, col + r));
            if !same_double(x, y) {
                out.push(format!("row {} (serial {}).{} {} | {}", r, word_of(a, o.serial as usize + r), k, js::num_str(x), js::num_str(y)));
            }
            r += 1;
        }
    }
    for (c, k) in item_u32().iter().enumerate() {
        let col = at[COLS_F + c] as usize;
        let mut r = 0;
        while r < rows && (out.len() as f64) < most {
            let (x, y) = (word_of(a, col + r), word_of(b, col + r));
            if x != y { out.push(format!("row {}.{} {} | {}", r, k, x, y)); }
            r += 1;
        }
    }
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

    /// the words itemDiff gives, held to the TypeScript's own output for these same bytes
    #[test]
    fn two_takes_differ_in_the_typescripts_words() {
        let mut g = Box::new(crate::game::Game::blank());
        g.items.store.o = o_of(item_cols());
        let o = g.items.store.o;
        let size = o.bytes as usize;
        let put_f = |t: &mut [u8], i: f64, r: usize, v: f64| { let at = (i as usize + r) * 8; t[at..at + 8].copy_from_slice(&v.to_ne_bytes()); };
        let put_w = |t: &mut [u8], i: f64, r: usize, v: u32| { let at = (i as usize + r) * 4; t[at..at + 4].copy_from_slice(&v.to_ne_bytes()); };
        let mut a = vec![0u8; size];
        put_w(&mut a, 0.0, 0, 2);
        put_w(&mut a, o.serial, 0, 7);
        put_w(&mut a, o.serial, 1, 9);
        put_f(&mut a, o.fuse, 1, f64::NAN);
        assert!(item_diff(&mut g, &a, &a.clone(), None).is_empty(), "the same state");

        let mut b = a.clone();
        put_f(&mut b, o.fuse, 1, f64::from_bits(0x7ff8_0000_0000_0001));
        assert!(item_diff(&mut g, &a, &b, None).is_empty(), "a NaN is a NaN");

        put_f(&mut b, o.px, 0, -0.0);
        put_f(&mut b, o.tumble_rate, 1, 0.1);
        put_w(&mut b, o.who, 1, 4294967295);
        put_w(&mut b, 0.0, 0, 3);
        assert_eq!(item_diff(&mut g, &a, &b, None), vec![
            String::from("2 things in the pool | 3"),
            String::from("row 0 (serial 7).px 0 | 0"),
            String::from("row 1 (serial 9).tumbleRate 0 | 0.1"),
            String::from("row 1.who 0 | 4294967295"),
        ]);
        assert_eq!(item_diff(&mut g, &a, &b, Some(2.0)).len(), 2);
    }
}
