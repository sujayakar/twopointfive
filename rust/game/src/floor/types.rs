//! The types of `floor`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum FloorMat {
    Carpet,
    Lino,
    Stone,
    Tile,
    Asphalt,
    Grass,
}
impl FloorMat {
    pub const fn as_str(self) -> &'static str {
        match self {
            FloorMat::Carpet => "carpet",
            FloorMat::Lino => "lino",
            FloorMat::Stone => "stone",
            FloorMat::Tile => "tile",
            FloorMat::Asphalt => "asphalt",
            FloorMat::Grass => "grass",
        }
    }
}

/// (TypeScript: inline at src/game/floor.ts:22 FLOOR, src/game/floor.ts:22)
#[derive(Clone, Debug)]
pub struct FLOORValue {
    pub loud: f64,
    pub hard: bool,
    pub rate: f64,
    pub label: String,
}
impl FLOORValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FLOORValue {
            loud: 0.0,
            hard: false,
            rate: 0.0,
            label: String::new(),
        }
    }
}
impl Default for FLOORValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/floor.ts:32 BUILDING, src/game/floor.ts:32)
#[derive(Clone, Copy, Debug)]
pub struct BUILDING {
    pub x0: f64,
    pub x1: f64,
    pub z0: f64,
    pub z1: f64,
}
impl BUILDING {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BUILDING {
            x0: 0.0,
            x1: 0.0,
            z0: 0.0,
            z1: 0.0,
        }
    }
}
impl Default for BUILDING { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/floor.ts:38 FINISHES, src/game/floor.ts:38)
#[derive(Clone, Debug)]
pub struct FINISHESItem {
    pub mat: String,
    pub x0: f64,
    pub z0: f64,
    pub x1: f64,
    pub z1: f64,
    pub room: String,
}
impl FINISHESItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FINISHESItem {
            mat: String::new(),
            x0: 0.0,
            z0: 0.0,
            x1: 0.0,
            z1: 0.0,
            room: String::new(),
        }
    }
}
impl Default for FINISHESItem { fn default() -> Self { Self::zeroed() } }

/// (src/game/floor.ts:22 FLOOR: a table nothing writes)
pub fn floor() -> JsMap<String, crate::floor::FLOORValue> { { let mut t = JsMap::new(); t.set(String::from("carpet"), crate::floor::FLOORValue { loud: 0.6, hard: false, rate: 0.94, label: String::from("carpet") }); t.set(String::from("lino"), crate::floor::FLOORValue { loud: 1.0, hard: true, rate: 1.0, label: String::from("lino") }); t.set(String::from("stone"), crate::floor::FLOORValue { loud: 1.1, hard: true, rate: 1.08, label: String::from("stone") }); t.set(String::from("tile"), crate::floor::FLOORValue { loud: 1.05, hard: true, rate: 1.04, label: String::from("raised floor") }); t.set(String::from("asphalt"), crate::floor::FLOORValue { loud: 0.9, hard: true, rate: 0.97, label: String::from("asphalt") }); t.set(String::from("grass"), crate::floor::FLOORValue { loud: 0.45, hard: false, rate: 0.88, label: String::from("grass") }); t } }

/// (src/game/floor.ts:32 BUILDING: a table nothing writes)
pub fn building() -> crate::floor::BUILDING { crate::floor::BUILDING { x0: 4.0, x1: 36.0, z0: 4.0, z1: 24.0 } }

/// (src/game/floor.ts:34 GRASS_EDGE)
pub const GRASS_EDGE: f64 = 3.2;

/// (src/game/floor.ts:38 FINISHES: a table nothing writes)
pub fn finishes() -> Vec<crate::floor::FINISHESItem> { alloc::vec![crate::floor::FINISHESItem { mat: String::from("lino"), x0: 4.0, z0: 10.0, x1: 36.0, z1: 12.2, room: String::from("corridor, wall to wall (level.ts floor(4, 10, 36, 12.2, lino); the runner down its middle is a decal)") }, crate::floor::FINISHESItem { mat: String::from("stone"), x0: 4.0, z0: 16.0, x1: 12.0, z1: 24.0, room: String::from("lobby south of the reception counter (floor(4, 16, 12, 24, stone))") }, crate::floor::FINISHESItem { mat: String::from("tile"), x0: 14.0, z0: 4.0, x1: 22.0, z1: 10.0, room: String::from("server room raised floor (floor(14, 4, 22, 10))") }, crate::floor::FINISHESItem { mat: String::from("lino"), x0: 28.0, z0: 12.2, x1: 36.0, z1: 24.0, room: String::from("break room (floor(28, 12.2, 36, 24, lino))") }] }
