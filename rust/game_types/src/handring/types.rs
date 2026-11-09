//! The types of `handring`.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, Default)]
pub struct EMPTYWHY {
    pub far: String,
    pub run: String,
    pub held: String,
    pub no: String,
}

/// (src/game/handring.ts:89 EMPTY_WORD)
pub const EMPTY_WORD: &str = "nothing in reach";


/// (src/game/handring.ts:89 EMPTY_SHOW_SECS)
pub const EMPTY_SHOW_SECS: f64 = 1.6;

/// (src/game/handring.ts:93 EMPTY_NEAR_M)
pub const EMPTY_NEAR_M: f64 = 2.2;

/// (src/game/handring.ts:95 EMPTY_SHY_M)
pub const EMPTY_SHY_M: f64 = 0.2;

/// (src/game/handring.ts:96 EMPTY_WHY: a table nothing writes)
pub fn empty_why() -> crate::handring::EMPTYWHY { crate::handring::EMPTYWHY { far: String::from("step closer"), run: String::from("stop sprinting"), held: String::from("not in this mission"), no: String::from("not now") } }
