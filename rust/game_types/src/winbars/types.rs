//! The types of `winbars`.
#![allow(unused_imports)]
use crate::{Blank, Words, bytes::ToBytes, ids::*, js::*, Quat, V3};
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

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Words)]
pub enum BreakBarCause {
    Blast,
    Round,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct WinbarsMod {
    pub log: Vec<String>,
    pub state: Option<crate::winbars::BarState>,
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

pub const HIDE: f64 = 3.0;
