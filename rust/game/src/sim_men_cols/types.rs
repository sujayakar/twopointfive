//! The types of `sim_men_cols`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: ManCols, src/sim/men/cols.ts:14)
#[derive(Clone, Copy, Debug)]
pub struct ManCols {
}
impl ManCols {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        ManCols {
        }
    }
}
impl Default for ManCols { fn default() -> Self { Self::blank() } }

/// (TypeScript: GuardCols, src/sim/men/cols.ts:26)
#[derive(Clone, Copy, Debug)]
pub struct GuardCols {
}
impl GuardCols {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        GuardCols {
        }
    }
}
impl Default for GuardCols { fn default() -> Self { Self::blank() } }

/// (TypeScript: PlayerCols, src/sim/men/cols.ts:29)
#[derive(Clone, Copy, Debug)]
pub struct PlayerCols {
}
impl PlayerCols {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        PlayerCols {
        }
    }
}
impl Default for PlayerCols { fn default() -> Self { Self::blank() } }

/// (src/sim/men/cols.ts:6 MEN_SCHEMA)
pub const MEN_SCHEMA: f64 = 2717444977.0;

/// (src/sim/men/cols.ts:7 MEN_HEAD)
pub const MEN_HEAD: f64 = 16.0;

/// (src/sim/men/cols.ts:8 MEN_F64: a table nothing writes)
pub fn men_f64() -> [String; 1] { [String::from("since")] }

/// (src/sim/men/cols.ts:9 MEN_U32: a table nothing writes)
pub fn men_u32() -> [String; 4] { [String::from("gen"), String::from("owner"), String::from("rank"), String::from("reason")] }

/// (src/sim/men/cols.ts:11 MEN_GEN)
pub const MEN_GEN: f64 = 132.0;
