//! src/sim/men/store.ts lines 1 to 119: nothing of this file runs in Rust. A man's numbers are plain fields of his record (`g.gd[id].owner`, `rank`, `reason`, `since`), so no store, row or schema latch is left; the skeleton's `notInGameFns` lists all six functions. What is left is the test below.
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

#[cfg(test)]
mod tests {
    use super::*;

    /// what holdSchema (store.ts:25) asked as a game started, asked of the build: the skeleton's layout is the layout of the rust/sim this crate links
    #[test]
    fn the_schema_is_the_modules() {
        let at = men_at();
        let n = crate::sim_men_cols::men_f64().len() + crate::sim_men_cols::men_u32().len();
        assert_eq!(at.len(), n);
        for i in 0..n {
            let want = if (i as f64) < NF { at[i] * 8.0 } else { at[i] * 4.0 };
            assert_eq!(sim::men::men_col(i as u32) as f64, want, "column {}", i);
        }
        assert_eq!(sim::men::men_col(n as u32) as f64, crate::sim_men_schema::MEN_ROWS);
        assert_eq!(sim::men::men_schema() as f64, crate::sim_men_cols::MEN_SCHEMA);
    }
}
