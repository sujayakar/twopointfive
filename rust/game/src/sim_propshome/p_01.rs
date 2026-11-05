//! src/sim/propshome.ts lines 1 to 133
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// Where each column starts for columns `cap` rows long. (propshome.ts propCols)
pub fn prop_cols(cap: f64) -> crate::sim_propshome::PropCols {
    seam_cols_of(PROP_HEAD, cap)
}

/// The columns of the game's furniture, over its block: every thing in its own row, asleep, to be placed. (propshome.ts PropStore.constructor, colstore.ts ColStore.constructor)
pub fn prop_store_new(g: &mut crate::game::Game, st: crate::Opaque, total: f64) -> crate::sim_propshome::PropStore {
    if total > PROP_CAP {
        panic!("a level of {} loose things of furniture: the game holds {} (sim/propshome.ts PROP_CAP, rust/sim/src/props.rs CAP)", total, PROP_CAP);
    }
    let o = store_o(prop_cols(PROP_CAP));
    let at = js::to_u32(crate::sim_state::sim_part(crate::sim_state::P_PROPS));
    let p = &mut sim::state::state(g.sim).props;
    p.n = js::to_u32(total);
    p.total = js::to_u32(total);
    let rows = total as usize;
    for i in 0..rows {
        p.id[i] = i as u32;
        p.row[i] = i as u32;
        p.flags[i] = sim::props::F_ASLEEP | sim::props::F_DIRTY;
        p.noise_t[i] = -10.0;
        p.push_t[i] = -10.0;
        p.cs[i] = 1.0;
    }
    crate::sim_propshome::PropStore { epoch: -1.0, total, o, home: st, b8: (at >> 3) as f64, b4: (at >> 2) as f64, cap: PROP_CAP }
}

/// The system's clock. (propshome.ts PropStore.time)
pub fn prop_store_time(g: &mut crate::game::Game) -> f64 {
    sim::state::state(g.sim).props.time
}

/// Rows in the level now. (propshome.ts PropStore.n)
pub fn prop_store_n(g: &mut crate::game::Game) -> f64 {
    sim::state::state(g.sim).props.n as f64
}

/// Where thing `id` is. (propshome.ts PropStore.rowOf)
pub fn prop_store_row_of(g: &mut crate::game::Game, id_2: f64) -> f64 {
    sim::state::state(g.sim).props.row[id_2 as usize] as f64
}

/// Rows a and b change places in every column but `row`, the table that follows them. (propshome.ts PropStore.swap, colstore.ts swapCols)
pub fn prop_store_swap(g: &mut crate::game::Game, a: f64, b: f64) {
    if a == b { return; }
    let (a, b) = (a as usize, b as usize);
    let p = &mut sim::state::state(g.sim).props;
    p.x.swap(a, b);
    p.z.swap(a, b);
    p.yaw.swap(a, b);
    p.vx.swap(a, b);
    p.vz.swap(a, b);
    p.w.swap(a, b);
    p.cs.swap(a, b);
    p.sn.swap(a, b);
    p.idle.swap(a, b);
    p.push_t.swap(a, b);
    p.noise_t.swap(a, b);
    p.flags.swap(a, b);
    p.who.swap(a, b);
    p.id.swap(a, b);
    let (at_a, at_b) = (p.id[a] as usize, p.id[b] as usize);
    p.row[at_a] = a as u32;
    p.row[at_b] = b as u32;
}

/// A thing leaves the level: its row goes behind the live ones, those after it closing up in their order. False if it was not in. (propshome.ts PropStore.leave)
pub fn prop_store_leave(g: &mut crate::game::Game, id_2: f64) -> bool {
    let n = prop_store_n(g);
    let mut r = prop_store_row_of(g, id_2);
    if r >= n { return false; }
    while r < n - 1.0 {
        prop_store_swap(g, r, r + 1.0);
        r += 1.0;
    }
    sim::state::state(g.sim).props.n = js::to_u32(n - 1.0);
    true
}

/// The level put back: every thing in the level again, each in the row of its own id. (propshome.ts PropStore.homeRows)
pub fn prop_store_home_rows(g: &mut crate::game::Game) {
    let total = g.props.store.total;
    let mut i = 0.0;
    while i < total {
        let at = prop_store_row_of(g, i);
        prop_store_swap(g, i, at);
        i += 1.0;
    }
    sim::state::state(g.sim).props.n = js::to_u32(total);
}

/// Where the hand-over's tables are in the module's memory (indices into its doubles) and how much each holds, asked once; the layout worked out here held to the module's own. (propshome.ts propIo)
pub fn prop_io(g: &mut crate::game::Game) -> crate::sim_propshome::PropIo {
    if let Some(io) = g.m.sim_propshome.io { return io; }
    let x = crate::sim_sim::sim();
    let cap = |k: usize| sim::props::cap(k as u32) as f64;
    let o = prop_cols(PROP_CAP);
    let names: Vec<String> = prop_f64().into_iter().chain(prop_u32()).collect();
    let starts = [o.x, o.z, o.yaw, o.vx, o.vz, o.w, o.cs, o.sn, o.idle, o.push_t, o.noise_t, o.flags, o.who, o.id, o.row];
    let doubles = prop_f64().len();
    let mut off: Vec<String> = Vec::new();
    for (i, k) in names.iter().enumerate() {
        let start = if i < doubles { starts[i] * 8.0 } else { starts[i] * 4.0 };
        if cap(100 + i) != start { off.push(k.clone()); }
    }
    if cap(0) != PROP_CAP || cap(13) != o.bytes || !off.is_empty() || cap(100 + names.len()) != 0.0 || cap(6) != PROP_PARS
        || cap(9) != pd().chair + 1.0 || cap(10) != pk().quiet + 1.0 || cap(11) != py().half + 1.0 || cap(12) != PROP_EV || pe().rolling + 1.0 != PROP_EV {
        let columns = if off.is_empty() { String::new() } else { format!(" (columns not where the module has them: {})", off.join(", ")) };
        panic!("the Rust module lays the furniture out as {} rows in {} bytes and src/sim/propshome.ts as {} in {}{}, or the hand-over's tables are not the same width: bun run tools/sim/build.ts", cap(0), cap(13), PROP_CAP, o.bytes, columns);
    }
    let addr = |t: u32| sim::props::addr(t) as f64;
    let io = crate::sim_propshome::PropIo {
        x,
        def: addr(0) / 8.0,
        con: addr(2) / 8.0,
        leaf: addr(3) / 8.0,
        way: addr(4) / 8.0,
        ev: addr(5) / 8.0,
        par: addr(6) / 8.0,
        max_contacts: cap(2),
        max_leaves: cap(3),
        max_ways: cap(4),
        max_events: cap(5),
    };
    g.m.sim_propshome.io = Some(io);
    io
}

/// Where two takes of the store's bytes differ, in words: the clock, the counts, then column by column and row by row. At most `most` lines. (propshome.ts propDiff)
pub fn prop_diff(g: &mut crate::game::Game, a: &[u8], b: &[u8], most: Option<f64>) -> Vec<String> {
    let most = most.unwrap_or(4.0);
    let o = g.props.store.o;
    let total = g.props.store.total;
    let (da, db) = (double_at(a), double_at(b));
    let (ua2, ua3, ub2, ub3) = (word_at(a, 2), word_at(a, 3), word_at(b, 2), word_at(b, 3));
    let mut out: Vec<String> = Vec::new();
    if !object_is(da, db) { out.push(format!("the clock {} | {}", da, db)); }
    if ua2 != ub2 || ua3 != ub3 { out.push(format!("{} of {} things in the level | {} of {}", ua2, ua3, ub2, ub3)); }
    let id_at = o.id;
    let mut who = |u: Vec<u32>, r: f64| -> String { format!("thing {}", u[(id_at + r) as usize]) };
    seam_col_diff(o, a, b, total, &mut who, most, out)
}

/// PropCols and PropStoreO are one shape, two types by rule.
fn store_o(c: crate::sim_propshome::PropCols) -> crate::sim_propshome::PropStoreO {
    crate::sim_propshome::PropStoreO {
        x: c.x,
        z: c.z,
        yaw: c.yaw,
        flags: c.flags,
        idle: c.idle,
        bytes: c.bytes,
        who: c.who,
        w: c.w,
        id: c.id,
        vx: c.vx,
        vz: c.vz,
        cs: c.cs,
        sn: c.sn,
        push_t: c.push_t,
        noise_t: c.noise_t,
        row: c.row,
    }
}

/// `new Float64Array(bytes.buffer, bytes.byteOffset, 1)[0]`
fn double_at(bytes: &[u8]) -> f64 {
    let mut w = [0u8; 8];
    w.copy_from_slice(&bytes[..8]);
    f64::from_ne_bytes(w)
}

/// `new Uint32Array(bytes.buffer, bytes.byteOffset, 4)[i]`
fn word_at(bytes: &[u8], i: usize) -> u32 {
    let mut w = [0u8; 4];
    w.copy_from_slice(&bytes[i * 4..i * 4 + 4]);
    u32::from_ne_bytes(w)
}

/// `Object.is` of two doubles: a NaN is a NaN, −0 is not 0.
fn object_is(a: f64, b: f64) -> bool {
    (a.is_nan() && b.is_nan()) || a.to_bits() == b.to_bits()
}

/// propshome.ts:41 `colsOf(PROP_HEAD, PROP_F64, PROP_U32, cap)`.
fn seam_cols_of(head: f64, cap: f64) -> crate::sim_propshome::PropCols {
    todo!("needs: sim_colstore::cols_of to give every column's start (its ColsOf2 holds only `bytes`)")
}

/// propshome.ts:131 `colDiff(O, PROP_F64, PROP_U32, a, b, S.total, who, most, out)`.
fn seam_col_diff(o: crate::sim_propshome::PropStoreO, a: &[u8], b: &[u8], rows: f64, who: &mut dyn FnMut(Vec<u32>, f64) -> String, most: f64, out: Vec<String>) -> Vec<String> {
    todo!("needs: sim_colstore::col_diff's ColDiffO to hold every column's start (it holds only `bytes`)")
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A game whose furniture stands in rows of its own ids; x and who mark each thing.
    fn stood(total: usize) -> alloc::boxed::Box<Game> {
        let mut g = alloc::boxed::Box::new(Game::blank());
        g.props.store.total = total as f64;
        let p = &mut sim::state::state(g.sim).props;
        p.n = total as u32;
        p.total = total as u32;
        for i in 0..total {
            p.id[i] = i as u32;
            p.row[i] = i as u32;
            p.x[i] = 100.0 + i as f64;
            p.who[i] = -(i as i32);
        }
        g
    }

    fn ids(g: &mut Game, rows: usize) -> Vec<u32> {
        sim::state::state(g.sim).props.id[..rows].to_vec()
    }

    #[test]
    fn a_thing_leaves_and_the_level_is_put_back() {
        let mut g = stood(5);
        assert!(prop_store_leave(&mut g, 1.0));
        assert_eq!(prop_store_n(&mut g), 4.0);
        // those after it closed up in their order; it stands behind the live ones
        assert_eq!(ids(&mut g, 5), vec![0, 2, 3, 4, 1]);
        assert_eq!(prop_store_row_of(&mut g, 1.0), 4.0);
        assert_eq!(prop_store_row_of(&mut g, 4.0), 3.0);
        // every column went with it, and `row` is not one of those
        let p = &sim::state::state(g.sim).props;
        assert_eq!(p.x[..5], [100.0, 102.0, 103.0, 104.0, 101.0]);
        assert_eq!(p.who[..5], [0, -2, -3, -4, -1]);
        assert_eq!(p.row[..5], [0, 4, 1, 2, 3]);
        // not in: no change
        assert!(!prop_store_leave(&mut g, 1.0));
        assert_eq!(prop_store_n(&mut g), 4.0);

        assert!(prop_store_leave(&mut g, 3.0));
        assert_eq!(ids(&mut g, 5), vec![0, 2, 4, 3, 1]);
        prop_store_home_rows(&mut g);
        assert_eq!(prop_store_n(&mut g), 5.0);
        assert_eq!(ids(&mut g, 5), vec![0, 1, 2, 3, 4]);
        let p = &sim::state::state(g.sim).props;
        assert_eq!(p.row[..5], [0, 1, 2, 3, 4]);
        assert_eq!(p.x[..5], [100.0, 101.0, 102.0, 103.0, 104.0]);
    }

    #[test]
    fn object_is_tells_zeros_apart_and_nans_together() {
        assert!(object_is(f64::NAN, -f64::NAN));
        assert!(!object_is(0.0, -0.0));
        assert!(object_is(1.5, 1.5));
    }
}
