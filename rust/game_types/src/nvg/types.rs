//! The types of `nvg`.
#![allow(unused_imports)]
use crate::{Blank, Words, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, ToBytes, Default)]
pub enum Mode {
    #[default] Off,
    Free,
    Dark,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, ToBytes, Default, Words)]
pub enum Why {
    #[word("your light")] #[default] YourLight,
    #[word("your goggles")] YourGoggles,
    #[word("not compromised")] NotCompromised,
    Compromised,
    #[word("no rule")] NoRule,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct Man {
    pub down: bool,
    pub dark: bool,
    pub agree_t: f64,
    pub place_t: f64,
    pub dim: bool,
    pub black: bool,
    pub run: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Team {
    pub t: f64,
    pub mode: crate::nvg::Mode,
    pub why: crate::nvg::Why,
    pub men: JsMap<GuardId, crate::nvg::Man>,
    pub lead_lit_t: f64,
    pub hunt_t: f64,
    pub alarm_t: f64,
    pub called_t: f64,
    pub call_due: bool,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct NvgMod {
    /// (src/game/nvg.ts:94 teams)
    pub teams: Option<crate::nvg::Team>,
}

/// (src/game/nvg.ts:138 LOOK_M: a table nothing writes)
pub fn look_m() -> Vec<f64> { alloc::vec![2.0, 5.0, 8.0, 12.0, 16.0] }

/// (src/game/nvg.ts:138 TUBE_M)
pub const TUBE_M: f64 = 8.0;
