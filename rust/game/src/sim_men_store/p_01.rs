//! src/sim/men/store.ts lines 1 to 119
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// The module's table is this file's table: a rust/sim of another schema is a mistake, said by name. (store.ts holdSchema)
pub fn hold_schema(g: &mut crate::game::Game) {
    if g.m.sim_men_store.held || !crate::sim_sim::sim_why_not().is_empty() { return; }
    g.m.sim_men_store.held = true;
    if let Some(fault) = schema_fault(sim::men::men_schema(), &|i| sim::men::men_col(i)) { panic!("{}", fault); }
}

/// holdSchema's test and its words, over what the module answers: `None` when the two sides agree.
fn schema_fault(schema: u32, col: &dyn Fn(u32) -> usize) -> Option<String> {
    let (doubles, words, at) = (crate::sim_men_cols::men_f64(), crate::sim_men_cols::men_u32(), men_at());
    let names: Vec<&String> = doubles.iter().chain(words.iter()).collect();
    let mut off: Vec<&str> = Vec::new();
    for (i, name) in names.iter().enumerate() {
        let want = if (i as f64) < NF { at[i] * 8.0 } else { at[i] * 4.0 };
        if col(i as u32) as f64 != want { off.push(name.as_str()); }
    }
    let rows = col(names.len() as u32);
    if schema as f64 == crate::sim_men_cols::MEN_SCHEMA && rows as f64 == crate::sim_men_schema::MEN_ROWS && off.is_empty() { return None; }
    let listed = if off.is_empty() { String::new() } else { format!(" (columns not where the module has them: {})", off.join(", ")) };
    Some(format!(
        "the Rust module lays the men out as schema {:x} in {} rows and src/sim/men/cols.ts as {:x} in {}{}: bun run tools/sim/men.ts, then bun run tools/sim/build.ts",
        schema, rows, crate::sim_men_cols::MEN_SCHEMA as u32, crate::sim_men_schema::MEN_ROWS as u32, listed
    ))
}

/// The words of a store's own doubles, low word first as the typed arrays of a little-endian machine have them: a copy, not a view. (store.ts ownWords)
pub fn own_words(g: &mut crate::game::Game, d: &[f64]) -> Vec<u32> {
    let own = &mut g.m.sim_men_store;
    let same = match &own.own_for {
        Some(f) => f.len() == d.len() && f.iter().zip(d).all(|(a, b)| a.to_bits() == b.to_bits()),
        None => false,
    };
    if !same {
        own.own_for = Some(d.to_vec());
        own.own_view = Some(words_of(d));
    }
    own.own_view.clone().unwrap()
}

fn words_of(d: &[f64]) -> Vec<u32> {
    let mut words = Vec::with_capacity(d.len() * 2);
    for x in d {
        let bits = x.to_bits();
        words.push(bits as u32);
        words.push((bits >> 32) as u32);
    }
    words
}

/// A man's row and store as he is made: nothing to write here. A man's numbers are plain fields of his record, zero in its blank, and MenStore is not in the game. (store.ts standMan)
pub fn stand_man(m: &mut crate::sim_men_cols::ManCols, s: &mut crate::Opaque) {}

/// The same row for a second object of the same man: nothing to write, for the same reason. (store.ts sameMan)
pub fn same_man(m: &mut crate::sim_men_cols::ManCols, of: crate::sim_men_cols::ManCols) {}

/// He has left the game: his numbers stay as they stood in his record, in a table that never shrinks, so nothing moves to a store of leavers. (store.ts manLeaves)
pub fn man_leaves(m: &mut crate::sim_men_cols::ManCols, also: &[crate::sim_men_cols::ManCols]) {}

/// The store for a man of no game: one until it is full, and then another. (store.ts loneMen)
pub fn lone_men(g: &mut crate::game::Game) -> crate::Opaque {
    if g.m.sim_men_store.lone.is_none() || g.m.sim_men_store.lone_rows == crate::sim_men_schema::MEN_ROWS {
        hold_schema(g);   // (all that `new MenStore()` still does)
        g.m.sim_men_store.lone = Some(crate::Opaque);
        g.m.sim_men_store.lone_rows = 0.0;
    }
    g.m.sim_men_store.lone_rows += 1.0;
    g.m.sim_men_store.lone.unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// the words bun gives for `new Uint32Array(d.buffer, d.byteOffset, d.byteLength >>> 2)`
    #[test]
    fn words_are_the_typed_arrays_words() {
        let d = [1.5, -0.0, 2717444977.0, 1e300, f64::NEG_INFINITY, f64::from_bits(0x7ff8000000000000), 5e-324, 0.1];
        let want: [u32; 16] = [
            0x00000000, 0x3ff80000, 0x00000000, 0x80000000, 0x6e200000, 0x41e43f1d, 0x8800759c, 0x7e37e43c,
            0x00000000, 0xfff00000, 0x00000000, 0x7ff80000, 0x00000001, 0x00000000, 0x9999999a, 0x3fb99999,
        ];
        assert_eq!(words_of(&d), want);
    }

    #[test]
    fn the_cached_words_follow_the_doubles() {
        let mut g = Game::blank();
        assert_eq!(own_words(&mut g, &[1.5]), [0, 0x3ff80000]);
        assert_eq!(own_words(&mut g, &[1.5]), [0, 0x3ff80000]);
        assert_eq!(own_words(&mut g, &[-0.0]), [0, 0x80000000]);
        assert_eq!(own_words(&mut g, &[0.0]), [0, 0]);
    }

    /// the skeleton's numbers against the rust/sim this crate links
    #[test]
    fn the_schema_is_the_modules() {
        assert_eq!(schema_fault(sim::men::men_schema(), &|i| sim::men::men_col(i)), None);
    }

    #[test]
    fn a_fault_is_said_by_name() {
        let moved = |i: u32| if i == 2 { 0 } else { sim::men::men_col(i) };
        assert_eq!(
            schema_fault(0xabc, &moved).unwrap(),
            "the Rust module lays the men out as schema abc in 64 rows and src/sim/men/cols.ts as a1f8eb71 in 64 (columns not where the module has them: owner): bun run tools/sim/men.ts, then bun run tools/sim/build.ts"
        );
    }

    #[test]
    fn a_full_lone_store_gives_way_to_another() {
        let mut g = Game::blank();
        g.m.sim_men_store.held = true;   // (past the module's question, which is another part's)
        for n in 1..=64 {
            lone_men(&mut g);
            assert_eq!(g.m.sim_men_store.lone_rows, n as f64);
        }
        lone_men(&mut g);
        assert_eq!(g.m.sim_men_store.lone_rows, 1.0);
    }
}
