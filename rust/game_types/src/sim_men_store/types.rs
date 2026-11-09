//! The types of `sim_men_store`.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (src/sim/men/store.ts:19 MEN_AT: a table nothing writes)
pub fn men_at() -> Vec<f64> { alloc::vec![2.0, 132.0, 196.0, 260.0, 324.0] }

/// (src/sim/men/store.ts:20 MEN_BYTES)
pub const MEN_BYTES: f64 = 1552.0;

/// (src/sim/men/store.ts:21 NF)
pub const NF: f64 = 1.0;
