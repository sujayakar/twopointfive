//! The types of `winbars`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, ToBytes, Default)]
pub struct BarState {
    pub met: Vec<crate::winbars::BarMet>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct BarMet {
    pub index: f64,
    pub t: f64,
    pub at: V3,
    pub dir: V3,
    pub flags: f64,
}

#[derive(Clone, Debug, Default)]
pub struct BarsSave {
    pub v: f64,
    pub s: Vec<f64>,
}

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
#[derive(Clone, Debug, ToBytes)]
pub struct WinbarsMod {
    /// (src/game/winbars.ts:57 LOG)
    pub log: Vec<String>,
    /// (src/game/winbars.ts:64 STATE)
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

/// (src/game/winbars.ts:59 WINBARS_BREAK)
pub const WINBARS_BREAK: bool = false;

/// (src/game/winbars.ts:60 HIDE)
pub const HIDE: f64 = 3.0;
