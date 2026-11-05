//! The types of `winbars`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: BarState, src/game/winbars.ts:61)
#[derive(Clone, Debug)]
pub struct BarState {
    pub gone: JsMap<crate::Num, f64>,
    pub met: Vec<crate::winbars::BarMet>,
}
impl BarState {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BarState {
            gone: JsMap::new(),
            met: Vec::new(),
        }
    }
}
impl Default for BarState { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: BarMet, src/game/winbars.ts:85)
#[derive(Clone, Copy, Debug)]
pub struct BarMet {
    pub index: f64,
    pub t: f64,
    pub at: V3,
    pub dir: V3,
    pub flags: f64,
}
impl BarMet {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BarMet {
            index: 0.0,
            t: 0.0,
            at: [0.0; 3],
            dir: [0.0; 3],
            flags: 0.0,
        }
    }
}
impl Default for BarMet { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: BarsSave, src/game/winbars.ts:174)
#[derive(Clone, Debug)]
pub struct BarsSave {
    pub v: f64,
    pub s: Vec<f64>,
}
impl BarsSave {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BarsSave {
            v: 0.0,
            s: Vec::new(),
        }
    }
}
impl Default for BarsSave { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum BreakBarCause {
    Blast,
    Round,
}
impl BreakBarCause {
    pub const fn as_str(self) -> &'static str {
        match self {
            BreakBarCause::Blast => "blast",
            BreakBarCause::Round => "round",
        }
    }
}

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct WinbarsMod {
    /// (src/game/winbars.ts:55 LOG)
    pub log: Vec<String>,
    /// (src/game/winbars.ts:62 STATE)
    pub state: Option<crate::winbars::BarState>,
}
impl WinbarsMod {
    pub fn blank() -> Self {
        WinbarsMod {
            log: Vec::new(),
            state: None,
        }
    }
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct WinbarsStats {
    pub broken: i32,
    pub by_round: i32,
    pub by_blast: i32,
    pub chunks: i32,
    pub demoted: i32,
    pub spared: i32,
}

/// (src/game/winbars.ts:57 WINBARS_BREAK)
pub const WINBARS_BREAK: bool = false;

/// (src/game/winbars.ts:58 HIDE)
pub const HIDE: f64 = 3.0;
