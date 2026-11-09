pub use ::game_types::sim_propshome::types;
pub use types::*;
use crate::game::Game;
use crate::js::{self, JsMap, JsSet};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// Where each column starts for columns `cap` rows long.
fn prop_cols(cap: f64) -> crate::sim_propshome::PropCols {
    // colsOf(PROP_HEAD, PROP_F64, PROP_U32, cap): walked here
    let mut o = crate::sim_propshome::PropCols::default();
    let mut at = PROP_HEAD;
    for k in [&mut o.x, &mut o.z, &mut o.yaw, &mut o.vx, &mut o.vz, &mut o.w, &mut o.cs, &mut o.sn, &mut o.idle, &mut o.push_t, &mut o.noise_t] {
        *k = at / 8.0;
        at += cap * 8.0;
    }
    for k in [&mut o.flags, &mut o.who, &mut o.id, &mut o.row] {
        *k = at / 4.0;
        at += cap * 4.0;
    }
    o.bytes = at;
    o
}

/// The columns of the game's furniture, over its block: every thing in its own row, asleep, to be placed.
pub(super) fn prop_store_new(g: &crate::game::Game, st: crate::Opaque, total: f64) -> crate::sim_propshome::PropStore {
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
    crate::sim_propshome::PropStore { total, o }
}

/// Rows in the level now.
fn prop_store_n(g: &crate::game::Game) -> f64 {
    sim::state::state_read(g.sim).props.n as f64
}

/// Where thing `id` is.
fn prop_store_row_of(g: &crate::game::Game, id_2: f64) -> f64 {
    sim::state::state_read(g.sim).props.row[id_2 as usize] as f64
}

/// Rows a and b change places in every column but `row`, the table that follows them.
fn prop_store_swap(g: &crate::game::Game, a: f64, b: f64) {
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

/// A thing leaves the level: its row goes behind the live ones, those after it closing up in their order. False if it was not in.
pub(super) fn prop_store_leave(g: &crate::game::Game, id_2: f64) -> bool {
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

/// The level put back: every thing in the level again, each in the row of its own id.
pub(super) fn prop_store_home_rows(g: &crate::game::Game) {
    let total = g.props.store.total;
    let mut i = 0.0;
    while i < total {
        let at = prop_store_row_of(g, i);
        prop_store_swap(g, i, at);
        i += 1.0;
    }
    sim::state::state(g.sim).props.n = js::to_u32(total);
}

/// Where the hand-over's tables are in the module's memory (indices into its doubles) and how much each holds, asked once; the layout worked out here held to the module's own.
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
        let columns = if off.is_empty() { String::new() } else { alloc::format!(" (columns not where the module has them: {})", off.join(", ")) };
        panic!("rust/sim lays the furniture out as {} rows in {} bytes and the rules as {} in {}{}, or the hand-over's tables are not the same width", cap(0), cap(13), PROP_CAP, o.bytes, columns);
    }
    let io = crate::sim_propshome::PropIo {
        max_contacts: cap(2),
        max_leaves: cap(3),
        max_ways: cap(4),
    };
    g.m.sim_propshome.io = Some(io);
    io
}

/// Where two takes of the store's bytes differ, in words: the clock, the counts, then column by column and row by row. At most `most` lines.
fn prop_diff(g: &crate::game::Game, a: &[u8], b: &[u8], most: Option<f64>) -> Vec<String> {
    let most = most.unwrap_or(4.0);
    let o = g.props.store.o;
    let total = g.props.store.total;
    let (da, db) = (double_at(a, 0), double_at(b, 0));
    let (ua2, ua3, ub2, ub3) = (word_at(a, 2), word_at(a, 3), word_at(b, 2), word_at(b, 3));
    let mut out: Vec<String> = Vec::new();
    if !object_is(da, db) { out.push(alloc::format!("the clock {} | {}", js::num_str(da), js::num_str(db))); }
    if ua2 != ub2 || ua3 != ub3 { out.push(alloc::format!("{} of {} things in the level | {} of {}", ua2, ua3, ub2, ub3)); }
    col_diff_of(&o, a, b, total, most, out)
}

/// colsOf's `O[k]` of the doubles, in PROP_F64's order.
fn double_cols(o: &crate::sim_propshome::PropStoreO) -> [f64; 11] {
    [o.x, o.z, o.yaw, o.vx, o.vz, o.w, o.cs, o.sn, o.idle, o.push_t, o.noise_t]
}

/// colsOf's `O[k]` of the words, in PROP_U32's order.
fn word_cols(o: &crate::sim_propshome::PropStoreO) -> [f64; 4] {
    [o.flags, o.who, o.id, o.row]
}

/// colstore.ts colDiff over this family's columns, a row named by its id.
/// The views' lengths are taken as there, so bytes too short panic where the TypeScript throws.
fn col_diff_of(o: &crate::sim_propshome::PropStoreO, a: &[u8], b: &[u8], rows: f64, most: f64, mut out: Vec<String>) -> Vec<String> {
    let doubles = (js::to_u32(o.flags) >> 1) as usize * 8;
    let words = (js::to_u32(o.bytes) >> 2) as usize * 4;
    let (da, db, ua, ub) = (&a[..doubles], &b[..doubles], &a[..words], &b[..words]);
    for (k, col) in prop_f64().iter().zip(double_cols(o)) {
        let mut r = 0usize;
        while (r as f64) < rows && (out.len() as f64) < most {
            let at = col as usize + r;
            let (x, y) = (double_at(da, at), double_at(db, at));
            if !object_is(x, y) {
                let whom = word_at(ua, o.id as usize + r);
                out.push(alloc::format!("row {} (thing {}).{} {} | {}", r, whom, k, js::num_str(x), js::num_str(y)));
            }
            r += 1;
        }
    }
    for (k, col) in prop_u32().iter().zip(word_cols(o)) {
        let mut r = 0usize;
        while (r as f64) < rows && (out.len() as f64) < most {
            let at = col as usize + r;
            let (x, y) = (word_at(ua, at), word_at(ub, at));
            if x != y { out.push(alloc::format!("row {}.{} {} | {}", r, k, x, y)); }
            r += 1;
        }
    }
    out
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

/// `new Float64Array(bytes.buffer, bytes.byteOffset, …)[i]`
fn double_at(bytes: &[u8], i: usize) -> f64 {
    let mut w = [0u8; 8];
    w.copy_from_slice(&bytes[i * 8..i * 8 + 8]);
    f64::from_ne_bytes(w)
}

/// `new Uint32Array(bytes.buffer, bytes.byteOffset, …)[i]`
fn word_at(bytes: &[u8], i: usize) -> u32 {
    let mut w = [0u8; 4];
    w.copy_from_slice(&bytes[i * 4..i * 4 + 4]);
    u32::from_ne_bytes(w)
}

/// `Object.is` of two doubles: a NaN is a NaN, −0 is not 0.
fn object_is(a: f64, b: f64) -> bool {
    (a.is_nan() && b.is_nan()) || a.to_bits() == b.to_bits()
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

    /// colsOf(16, PROP_F64, PROP_U32, 128) as the TypeScript's own sum gives it, and held to the module's own layout as propIo holds it.
    #[test]
    fn the_columns_are_where_the_module_has_them() {
        let o = prop_cols(PROP_CAP);
        assert_eq!([o.x, o.z, o.noise_t, o.flags, o.who, o.id, o.row, o.bytes], [2.0, 130.0, 1282.0, 2820.0, 2948.0, 3076.0, 3204.0, 13328.0]);
        let store = store_o(o);
        for (i, col) in double_cols(&store).into_iter().enumerate() {
            assert_eq!(sim::props::cap(100 + i as u32) as f64, col * 8.0);
        }
        for (i, col) in word_cols(&store).into_iter().enumerate() {
            assert_eq!(sim::props::cap(111 + i as u32) as f64, col * 4.0);
        }
        assert_eq!(sim::props::cap(13) as f64, o.bytes);
        assert_eq!(sim::props::cap(115), 0);
    }

    fn put_double(bytes: &mut [u8], i: usize, x: f64) { bytes[i * 8..i * 8 + 8].copy_from_slice(&x.to_ne_bytes()); }
    fn put_word(bytes: &mut [u8], i: usize, x: u32) { bytes[i * 4..i * 4 + 4].copy_from_slice(&x.to_ne_bytes()); }

    #[test]
    fn two_takes_differ_in_these_words() {
        let mut g = alloc::boxed::Box::new(Game::blank());
        let o = store_o(prop_cols(PROP_CAP));
        g.props.store.o = o;
        g.props.store.total = 3.0;
        let mut a = vec![0u8; o.bytes as usize];
        for (r, id) in [7u32, 8, 9].into_iter().enumerate() { put_word(&mut a, o.id as usize + r, id); }
        put_word(&mut a, 2, 3);
        put_word(&mut a, 3, 3);
        put_double(&mut a, o.noise_t as usize, f64::NAN);
        let mut b = a.clone();
        assert!(prop_diff(&mut g, &a, &b, None).is_empty());

        put_double(&mut b, 0, 1.5);
        put_word(&mut b, 2, 2);
        put_double(&mut a, o.yaw as usize + 1, 0.25);
        put_double(&mut b, o.yaw as usize + 1, f64::NAN);
        put_double(&mut a, o.push_t as usize, -0.0);
        put_double(&mut b, o.noise_t as usize, f64::from_bits(0x7ff8_0000_0000_0001));
        put_word(&mut a, o.flags as usize + 2, 1);
        put_word(&mut b, o.flags as usize + 2, 3);
        let lines = prop_diff(&mut g, &a, &b, Some(9.0));
        assert_eq!(lines, [
            "the clock 0 | 1.5",
            "3 of 3 things in the level | 2 of 3",
            "row 1 (thing 8).yaw 0.25 | NaN",
            "row 0 (thing 7).pushT 0 | 0",
            "row 2.flags 1 | 3",
        ]);
        // at most `most`, four if none is said; what is past the most is not looked at
        assert_eq!(prop_diff(&mut g, &a, &b, None).len(), 4);
        assert_eq!(prop_diff(&mut g, &a, &b, Some(1.0)).len(), 2);
    }

}
