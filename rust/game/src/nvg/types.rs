//! The types of `nvg`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Mode {
    Off,
    Free,
    Dark,
}
impl Mode {
    pub const fn as_str(self) -> &'static str {
        match self {
            Mode::Off => "off",
            Mode::Free => "free",
            Mode::Dark => "dark",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Why {
    YourLight,
    YourGoggles,
    NotCompromised,
    Compromised,
    NoRule,
}
impl Why {
    pub const fn as_str(self) -> &'static str {
        match self {
            Why::YourLight => "your light",
            Why::YourGoggles => "your goggles",
            Why::NotCompromised => "not compromised",
            Why::Compromised => "compromised",
            Why::NoRule => "no rule",
        }
    }
}

/// (TypeScript: Man, src/game/nvg.ts:73)
#[derive(Clone, Copy, Debug)]
pub struct Man {
    pub down: bool,
    pub dark: bool,
    pub agree_t: f64,
    pub place_t: f64,
    pub dim: bool,
    pub black: bool,
    pub run: f64,
}
impl Man {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Man {
            down: false,
            dark: false,
            agree_t: 0.0,
            place_t: 0.0,
            dim: false,
            black: false,
            run: 0.0,
        }
    }
}
impl Default for Man { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Team, src/game/nvg.ts:83)
#[derive(Clone, Debug)]
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
impl Team {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Team {
            t: 0.0,
            mode: crate::nvg::Mode::Off,
            why: crate::nvg::Why::YourLight,
            men: JsMap::new(),
            lead_lit_t: 0.0,
            hunt_t: 0.0,
            alarm_t: 0.0,
            called_t: 0.0,
            call_due: false,
        }
    }
}
impl Default for Team { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: NvgNote, src/game/nvg.ts:90)
#[derive(Clone, Debug)]
pub struct NvgNote {
    pub t: f64,
    pub who: String,
    pub what: String,
}
impl NvgNote {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        NvgNote {
            t: 0.0,
            who: String::new(),
            what: String::new(),
        }
    }
}
impl Default for NvgNote { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/nvg.ts:200 nvgState ->, src/game/nvg.ts:200)
#[derive(Clone, Debug)]
pub struct NvgState {
    pub mode: crate::nvg::Mode,
    pub why: crate::nvg::Why,
    pub quiet_for: f64,
    pub men: Vec<crate::nvg::NvgStateMenItem>,
}
impl NvgState {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        NvgState {
            mode: crate::nvg::Mode::Off,
            why: crate::nvg::Why::YourLight,
            quiet_for: 0.0,
            men: Vec::new(),
        }
    }
}
impl Default for NvgState { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/nvg.ts:200 NvgState.men, src/game/nvg.ts:200)
#[derive(Clone, Copy, Debug)]
pub struct NvgStateMenItem {
    pub gd: GuardId,
    pub down: bool,
    pub dark: bool,
    pub dim: bool,
    pub black: bool,
}
impl NvgStateMenItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        NvgStateMenItem {
            gd: GuardId(0),
            down: false,
            dark: false,
            dim: false,
            black: false,
        }
    }
}
impl Default for NvgStateMenItem { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct NvgMod {
    /// (src/game/nvg.ts:92 NVG_LOG)
    pub nvg_log: Vec<crate::nvg::NvgNote>,
    /// (src/game/nvg.ts:93 teams)
    pub teams: Option<crate::nvg::Team>,
}
impl NvgMod {
    pub fn blank() -> Self {
        NvgMod {
            nvg_log: Vec::new(),
            teams: None,
        }
    }
}

/// (src/game/nvg.ts:137 LOOK_M: a table nothing writes)
pub fn look_m() -> Vec<f64> { alloc::vec![2.0, 5.0, 8.0, 12.0, 16.0] }

/// (src/game/nvg.ts:137 TUBE_M)
pub const TUBE_M: f64 = 8.0;
