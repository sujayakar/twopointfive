//! The types of `sim_men_store`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct SimMenStoreMod {
    /// (src/sim/men/store.ts:23 held)
    pub held: bool,
    /// (src/sim/men/store.ts:35 ownFor)
    pub own_for: Option<Vec<f64>>,
    /// (src/sim/men/store.ts:35 ownView)
    pub own_view: Option<Vec<u32>>,
    /// (src/sim/men/store.ts:105 lone)
    pub lone: Option<crate::Opaque>,
    /// (src/sim/men/store.ts:105 loneRows)
    pub lone_rows: f64,
}
impl SimMenStoreMod {
    pub fn blank() -> Self {
        SimMenStoreMod {
            held: false,
            own_for: None,
            own_view: None,
            lone: None,
            lone_rows: 0.0,
        }
    }
}

/// (src/sim/men/store.ts:19 MEN_AT: a table nothing writes)
pub fn men_at() -> Vec<f64> { alloc::vec![2.0, 132.0, 196.0, 260.0, 324.0] }

/// (src/sim/men/store.ts:20 MEN_BYTES)
pub const MEN_BYTES: f64 = 1552.0;

/// (src/sim/men/store.ts:21 NF)
pub const NF: f64 = 1.0;
