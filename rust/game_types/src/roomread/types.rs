//! The types of `roomread`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: RoomWay, src/game/roomread.ts:20)
#[derive(Clone, Copy, Debug)]
pub struct RoomWay {
    pub o: CoverOpeningId,
    pub in_n: [f64; 2],
    pub beyond: V3,
    pub far: f64,
    pub far_known: bool,
    pub shut: bool,
    pub locked: bool,
    pub dist: f64,
}
impl RoomWay {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RoomWay {
            o: CoverOpeningId(0),
            in_n: [0.0; 2],
            beyond: [0.0; 3],
            far: 0.0,
            far_known: false,
            shut: false,
            locked: false,
            dist: 0.0,
        }
    }
}
impl Default for RoomWay { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: RoomRead, src/game/roomread.ts:34)
#[derive(Clone, Debug)]
pub struct RoomRead {
    pub at: V3,
    pub room: f64,
    pub known: bool,
    pub cells: Vec<f64>,
    pub steps: JsMap<crate::Num, f64>,
    pub ways: Vec<crate::roomread::RoomWay>,
}
impl RoomRead {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RoomRead {
            at: [0.0; 3],
            room: 0.0,
            known: false,
            cells: Vec::new(),
            steps: JsMap::new(),
            ways: Vec::new(),
        }
    }
}
impl Default for RoomRead { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/roomread.ts:80 sideSign(d), src/game/roomread.ts:80)
#[derive(Clone, Debug)]
pub struct SideSignD {
    pub def: crate::roomread::SideSignDDef,
}
impl SideSignD {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SideSignD {
            def: crate::roomread::SideSignDDef::zeroed(),
        }
    }
}
impl Default for SideSignD { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/roomread.ts:80 SideSignD.def, src/game/roomread.ts:80)
#[derive(Clone, Debug)]
pub struct SideSignDDef {
    pub closed_dir: f64,
    pub hinge: crate::roomread::SideSignDDefHinge,
}
impl SideSignDDef {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SideSignDDef {
            closed_dir: 0.0,
            hinge: crate::roomread::SideSignDDefHinge::Array(Vec::new()),
        }
    }
}
impl Default for SideSignDDef { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: a union)
#[derive(Clone, Debug)]
pub enum SideSignDDefHinge {
    Array(Vec<f64>),
    List([f64; 2]),
}
