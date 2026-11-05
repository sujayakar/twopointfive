//! The types of `snow_bog`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: Ground, src/game/snowBog.ts:38)
#[derive(Clone, Debug)]
pub struct Ground {
    pub kinds: Vec<u8>,
    pub lays: Vec<f64>,
    pub banks: Vec<f64>,
}
impl Ground {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Ground {
            kinds: Vec::new(),
            lays: Vec::new(),
            banks: Vec::new(),
        }
    }
}
impl Default for Ground { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/snowBog.ts:37 SNOW_KIND, src/game/snowBog.ts:37)
#[derive(Clone, Copy, Debug)]
pub struct SNOWKIND {
    pub none: f64,
    pub bare: f64,
    pub packed: f64,
    pub old: f64,
    pub bank: f64,
}
impl SNOWKIND {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SNOWKIND {
            none: 0.0,
            bare: 0.0,
            packed: 0.0,
            old: 0.0,
            bank: 0.0,
        }
    }
}
impl Default for SNOWKIND { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct SnowBogMod {
    /// (src/game/snowBog.ts:39 groundOf)
    pub ground_of: Option<crate::snow_bog::Ground>,
}
impl SnowBogMod {
    pub fn blank() -> Self {
        SnowBogMod {
            ground_of: None,
        }
    }
}

/// (src/game/snowBog.ts:37 SNOW_KIND: a table nothing writes)
pub fn snow_kind() -> crate::snow_bog::SNOWKIND { crate::snow_bog::SNOWKIND { none: 0.0, bare: 1.0, packed: 2.0, old: 3.0, bank: 4.0 } }
