//! The types of `handring`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: inline at src/game/handring.ts:96 EMPTY_WHY, src/game/handring.ts:96)
#[derive(Clone, Debug)]
pub struct EMPTYWHY {
    pub far: String,
    pub run: String,
    pub held: String,
    pub no: String,
}
impl EMPTYWHY {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        EMPTYWHY {
            far: String::new(),
            run: String::new(),
            held: String::new(),
            no: String::new(),
        }
    }
}
impl Default for EMPTYWHY { fn default() -> Self { Self::zeroed() } }

/// (src/game/handring.ts:89 EMPTY_WORD)
pub const EMPTY_WORD: &str = "nothing in reach";

/// (src/game/handring.ts:89 EMPTY_AGAIN_SECS)
pub const EMPTY_AGAIN_SECS: f64 = 3.0;

/// (src/game/handring.ts:89 EMPTY_SHOW_SECS)
pub const EMPTY_SHOW_SECS: f64 = 1.6;

/// (src/game/handring.ts:93 EMPTY_NEAR_M)
pub const EMPTY_NEAR_M: f64 = 2.2;

/// (src/game/handring.ts:95 EMPTY_SHY_M)
pub const EMPTY_SHY_M: f64 = 0.2;

/// (src/game/handring.ts:96 EMPTY_WHY: a table nothing writes)
pub fn empty_why() -> crate::handring::EMPTYWHY { crate::handring::EMPTYWHY { far: String::from("step closer"), run: String::from("stop sprinting"), held: String::from("not in this mission"), no: String::from("not now") } }
