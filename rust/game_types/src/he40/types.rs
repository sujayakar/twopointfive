//! The types of `he40`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, ToBytes, Default)]
pub struct ShellEnd {
    pub at: V3,
    pub flown: f64,
    pub armed: bool,
    pub met: crate::he40::ShellEndMet,
    pub report: Option<crate::frag::BlastReport>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum ShellEndMet {
    #[default] Prop,
    Air,
    Man,
    Floor,
    Static,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct Shell {
    pub pos: V3,
    pub vel: V3,
    pub age: f64,
    pub flown: f64,
    pub shooter: CharId,
    pub spin: f64,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes)]
pub struct He40Mod {
    /// (src/game/he40.ts:43 HE40_LOG)
    pub he40_log: Vec<crate::he40::ShellEnd>,
}
impl He40Mod {
    pub fn blank() -> Self {
        He40Mod {
            he40_log: Vec::new(),
        }
    }
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct He40Stats {
    pub fired: i32,
    pub bursts: i32,
    pub duds: i32,
    pub gone: i32,
}
