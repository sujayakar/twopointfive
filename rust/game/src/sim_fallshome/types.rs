//! The types of `sim_fallshome`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum FallSound {
    Empty,
    GlassTinkle,
    GlassShatter,
}
impl FallSound {
    pub const fn as_str(self) -> &'static str {
        match self {
            FallSound::Empty => "",
            FallSound::GlassTinkle => "glassTinkle",
            FallSound::GlassShatter => "glassShatter",
        }
    }
}

/// (TypeScript: FallDue, src/sim/fallshome.ts:37)
#[derive(Clone, Copy, Debug)]
pub struct FallDue {
    pub name: crate::sim_fallshome::FallSound,
    pub pos: [f64; 3],
    pub gain: f64,
    pub rate: f64,
    pub heard: bool,
    pub level: f64,
    pub who: f64,
    pub dust: f64,
}
impl FallDue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FallDue {
            name: crate::sim_fallshome::FallSound::Empty,
            pos: [0.0; 3],
            gain: 0.0,
            rate: 0.0,
            heard: false,
            level: 0.0,
            who: 0.0,
            dust: 0.0,
        }
    }
}
impl Default for FallDue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: FallStore, src/sim/fallshome.ts:48)
#[derive(Clone, Copy, Debug)]
pub struct FallStore {
    pub base: f64,
    pub home: crate::Opaque,
}
impl FallStore {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        FallStore {
            base: 0.0,
            home: crate::Opaque,
        }
    }
}
impl Default for FallStore { fn default() -> Self { Self::blank() } }

/// (TypeScript: inline at src/sim/fallshome.ts:25 FP, src/sim/fallshome.ts:25)
#[derive(Clone, Copy, Debug)]
pub struct FP {
    pub t: f64,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub name: f64,
    pub gain: f64,
    pub rate: f64,
    pub amount: f64,
    pub who: f64,
    pub event: f64,
    pub level: f64,
    pub dust: f64,
}
impl FP {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FP {
            t: 0.0,
            x: 0.0,
            y: 0.0,
            z: 0.0,
            name: 0.0,
            gain: 0.0,
            rate: 0.0,
            amount: 0.0,
            who: 0.0,
            event: 0.0,
            level: 0.0,
            dust: 0.0,
        }
    }
}
impl Default for FP { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/sim/fallshome.ts:43 FALLS, src/sim/fallshome.ts:43)
#[derive(Clone, Debug)]
pub struct SimFallshomeModFALLS {
    pub both: bool,
    pub there: f64,
    pub here: f64,
    pub asked: f64,
    pub differ: f64,
    pub first: String,
    pub dues: f64,
    pub steps: f64,
    pub crunches: f64,
}
impl SimFallshomeModFALLS {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SimFallshomeModFALLS {
            both: false,
            there: 0.0,
            here: 0.0,
            asked: 0.0,
            differ: 0.0,
            first: String::new(),
            dues: 0.0,
            steps: 0.0,
            crunches: 0.0,
        }
    }
}
impl Default for SimFallshomeModFALLS { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum FallStoreLandingKey {
    Dust,
    X,
    T,
    Y,
    Z,
    Name,
    Level,
    Who,
    Event,
    Gain,
    Rate,
    Amount,
}
impl FallStoreLandingKey {
    pub const fn as_str(self) -> &'static str {
        match self {
            FallStoreLandingKey::Dust => "dust",
            FallStoreLandingKey::X => "x",
            FallStoreLandingKey::T => "t",
            FallStoreLandingKey::Y => "y",
            FallStoreLandingKey::Z => "z",
            FallStoreLandingKey::Name => "name",
            FallStoreLandingKey::Level => "level",
            FallStoreLandingKey::Who => "who",
            FallStoreLandingKey::Event => "event",
            FallStoreLandingKey::Gain => "gain",
            FallStoreLandingKey::Rate => "rate",
            FallStoreLandingKey::Amount => "amount",
        }
    }
}

/// (TypeScript: inline at src/sim/fallshome.ts:86 step ->, src/sim/fallshome.ts:86)
#[derive(Clone, Copy, Debug)]
pub struct FallStoreStep {
    pub level: f64,
    pub k: f64,
}
impl FallStoreStep {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FallStoreStep {
            level: 0.0,
            k: 0.0,
        }
    }
}
impl Default for FallStoreStep { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/sim/fallshome.ts:148 stepHere ->, src/sim/fallshome.ts:148)
#[derive(Clone, Copy, Debug)]
pub struct StepHere {
    pub level: f64,
    pub k: f64,
}
impl StepHere {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StepHere {
            level: 0.0,
            k: 0.0,
        }
    }
}
impl Default for StepHere { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct SimFallshomeMod {
    /// (src/sim/fallshome.ts:43 FALLS)
    pub falls: crate::sim_fallshome::SimFallshomeModFALLS,
}
impl SimFallshomeMod {
    pub fn blank() -> Self {
        SimFallshomeMod {
            falls: crate::sim_fallshome::SimFallshomeModFALLS { both: false, there: 0.0, here: 0.0, asked: 0.0, differ: 0.0, first: String::from(""), dues: 0.0, steps: 0.0, crunches: 0.0 },
        }
    }
}

/// (src/sim/fallshome.ts:20 FALL_CELL)
pub const FALL_CELL: f64 = 0.5;

/// (src/sim/fallshome.ts:20 FALL_CELLS_X)
pub const FALL_CELLS_X: f64 = 80.0;

/// (src/sim/fallshome.ts:20 FALL_CELLS_Z)
pub const FALL_CELLS_Z: f64 = 56.0;

/// (src/sim/fallshome.ts:20 FALL_CELL_MAX)
pub const FALL_CELL_MAX: f64 = 0.25;

/// (src/sim/fallshome.ts:20 FALL_MAXP)
pub const FALL_MAXP: f64 = 96.0;

/// (src/sim/fallshome.ts:22 FALL_GLASS)
pub const FALL_GLASS: f64 = 0.0;

/// (src/sim/fallshome.ts:22 FALL_PIECES)
pub const FALL_PIECES: f64 = 1.0;

/// (src/sim/fallshome.ts:22 FALL_LISTS)
pub const FALL_LISTS: f64 = 2.0;

/// (src/sim/fallshome.ts:23 NCELLS)
pub const NCELLS: f64 = 4480.0;

/// (src/sim/fallshome.ts:25 FP: a table nothing writes)
pub fn fp() -> crate::sim_fallshome::FP { crate::sim_fallshome::FP { t: 0.0, x: 1.0, y: 2.0, z: 3.0, name: 4.0, gain: 5.0, rate: 6.0, amount: 7.0, who: 8.0, event: 9.0, level: 10.0, dust: 11.0 } }

/// (src/sim/fallshome.ts:26 NP)
pub const NP: f64 = 12.0;

/// (src/sim/fallshome.ts:26 DUE)
pub const DUE: f64 = 10.0;

/// (src/sim/fallshome.ts:29 LIST)
pub const LIST: f64 = 1152.0;

/// (src/sim/fallshome.ts:29 O_P)
pub const O_P: f64 = 4480.0;

/// (src/sim/fallshome.ts:29 O_N)
pub const O_N: f64 = 6784.0;

/// (src/sim/fallshome.ts:29 O_ANY)
pub const O_ANY: f64 = 6786.0;

/// (src/sim/fallshome.ts:29 O_LAST)
pub const O_LAST: f64 = 6787.0;

/// (src/sim/fallshome.ts:30 FALL_LEN)
pub const FALL_LEN: f64 = 6790.0;

/// (src/sim/fallshome.ts:32 FALL_SOUNDS: a table nothing writes)
pub fn fall_sounds() -> [String; 2] { [String::from("glassTinkle"), String::from("glassShatter")] }

/// (src/sim/fallshome.ts:35 FALL_UNHEARD)
pub const FALL_UNHEARD: f64 = 0.0;

/// (src/sim/fallshome.ts:35 FALL_IF_SOUNDED)
pub const FALL_IF_SOUNDED: f64 = 1.0;

/// (src/sim/fallshome.ts:35 FALL_HEARD)
pub const FALL_HEARD: f64 = 2.0;

/// (src/sim/fallshome.ts:46 FALL_BIT)
pub const FALL_BIT: f64 = 8.0;

/// (src/sim/fallshome.ts:112 NONE: a table nothing writes)
pub fn none() -> Vec<crate::sim_fallshome::FallDue> { Vec::new() }
