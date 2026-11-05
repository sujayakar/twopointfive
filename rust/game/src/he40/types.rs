//! The types of `he40`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: ShellEnd, src/game/he40.ts:42)
#[derive(Clone, Debug)]
pub struct ShellEnd {
    pub at: V3,
    pub flown: f64,
    pub armed: bool,
    pub met: crate::he40::ShellEndMet,
    pub report: Option<crate::frag::BlastReport>,
}
impl ShellEnd {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ShellEnd {
            at: [0.0; 3],
            flown: 0.0,
            armed: false,
            met: crate::he40::ShellEndMet::Prop,
            report: None,
        }
    }
}
impl Default for ShellEnd { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum ShellEndMet {
    Prop,
    Air,
    Man,
    Floor,
    Static,
}
impl ShellEndMet {
    pub const fn as_str(self) -> &'static str {
        match self {
            ShellEndMet::Prop => "prop",
            ShellEndMet::Air => "air",
            ShellEndMet::Man => "man",
            ShellEndMet::Floor => "floor",
            ShellEndMet::Static => "static",
        }
    }
}

/// (TypeScript: Shell, src/game/he40.ts:48)
#[derive(Clone, Copy, Debug)]
pub struct Shell {
    pub pos: V3,
    pub vel: V3,
    pub age: f64,
    pub flown: f64,
    pub shooter: CharId,
    pub spin: f64,
}
impl Shell {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Shell {
            pos: [0.0; 3],
            vel: [0.0; 3],
            age: 0.0,
            flown: 0.0,
            shooter: CharId(0),
            spin: 0.0,
        }
    }
}
impl Default for Shell { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum BurstMet {
    Prop,
    Air,
    Man,
    Floor,
    Static,
}
impl BurstMet {
    pub const fn as_str(self) -> &'static str {
        match self {
            BurstMet::Prop => "prop",
            BurstMet::Air => "air",
            BurstMet::Man => "man",
            BurstMet::Floor => "floor",
            BurstMet::Static => "static",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum DudMet {
    Prop,
    Air,
    Man,
    Floor,
    Static,
}
impl DudMet {
    pub const fn as_str(self) -> &'static str {
        match self {
            DudMet::Prop => "prop",
            DudMet::Air => "air",
            DudMet::Man => "man",
            DudMet::Floor => "floor",
            DudMet::Static => "static",
        }
    }
}

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
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
