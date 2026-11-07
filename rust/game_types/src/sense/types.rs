//! The types of `sense`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: Blob, src/game/sense.ts:89)
#[derive(Clone, Copy, Debug)]
pub struct Blob {
    pub x: f64,
    pub y0: f64,
    pub z: f64,
    pub r2: f64,
    pub rate: f64,
    pub ttl: f64,
    pub age: f64,
    pub mass: f64,
}
impl Blob {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Blob {
            x: 0.0,
            y0: 0.0,
            z: 0.0,
            r2: 0.0,
            rate: 0.0,
            ttl: 0.0,
            age: 0.0,
            mass: 0.0,
        }
    }
}
impl Default for Blob { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: SmokeGrid, src/game/sense.ts:98)
#[derive(Clone, Debug)]
pub struct SmokeGrid {
    pub clock: f64,
    pub pending: Vec<f64>,
    pub stands: bool,
    pub fed: bool,
    pub sim: crate::Opaque,
}
impl SmokeGrid {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        SmokeGrid {
            clock: 0.0,
            pending: Vec::new(),
            stands: false,
            fed: false,
            sim: crate::Opaque,
        }
    }
}
impl Default for SmokeGrid { fn default() -> Self { Self::blank() } }

/// (TypeScript: GridOwner, src/game/sense.ts:167)
#[derive(Clone, Copy, Debug)]
pub struct GridOwner {
}
impl GridOwner {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GridOwner {
        }
    }
}
impl Default for GridOwner { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: LightField, src/game/sense.ts:188)
#[derive(Clone, Copy, Debug)]
pub struct LightField {
}
impl LightField {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LightField {
        }
    }
}
impl Default for LightField { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: CpuSmoke, src/game/sense.ts:192)
#[derive(Clone, Debug)]
pub struct CpuSmoke {
    pub blobs: Vec<crate::sense::Blob>,
    pub grid: Option<crate::sense::SmokeGrid>,
    pub lives: Vec<f64>,
}
impl CpuSmoke {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        CpuSmoke {
            blobs: Vec::new(),
            grid: None,
            lives: Vec::new(),
        }
    }
}
impl Default for CpuSmoke { fn default() -> Self { Self::blank() } }

/// (TypeScript: a union)
#[derive(Clone, Copy, Debug)]
pub enum SmokeGridStepLampsRet {
    Bool(bool),
    LampRows(crate::scene_collision::LampRows),
}

/// (src/game/sense.ts:90 PI15)
pub const PI15: f64 = 5.568327996831708;


