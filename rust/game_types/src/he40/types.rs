//! The types of `he40`.
#![allow(unused_imports)]
use crate::{Blank, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, ToBytes, Default)]
pub struct ShellEnd {
    pub flown: f64,
    pub armed: bool,
    pub met: crate::he40::ShellEndMet,
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
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct He40Mod {
    pub he40_log: Vec<crate::he40::ShellEnd>,
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct He40Stats {
    pub duds: i32,
}
