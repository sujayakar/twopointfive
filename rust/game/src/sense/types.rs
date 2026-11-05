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

/// (TypeScript: SmokeGrid, src/game/sense.ts:104)
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

/// (TypeScript: GridOwner, src/game/sense.ts:172)
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

/// (TypeScript: LightField, src/game/sense.ts:193)
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

/// (TypeScript: CpuSmoke, src/game/sense.ts:197)
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

/// (TypeScript: inline at src/game/sense.ts:184 GRID, src/game/sense.ts:184)
#[derive(Clone, Copy, Debug)]
pub struct GRID {
    pub step: f64,
    pub head: f64,
    pub emitters: f64,
    pub emit: f64,
    pub bricks: f64,
    pub cells: f64,
    pub vol: f64,
    pub out: f64,
    pub given: f64,
    pub lost: f64,
    pub nbx: f64,
    pub nbz: f64,
    pub side: f64,
    pub glow_max: f64,
    pub chest: f64,
    pub glow_d: f64,
    pub glow_b: f64,
    pub glow_x: f64,
    pub glow_z: f64,
    pub glow_lamps: f64,
    pub rho: f64,
}
impl GRID {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GRID {
            step: 0.0,
            head: 0.0,
            emitters: 0.0,
            emit: 0.0,
            bricks: 0.0,
            cells: 0.0,
            vol: 0.0,
            out: 0.0,
            given: 0.0,
            lost: 0.0,
            nbx: 0.0,
            nbz: 0.0,
            side: 0.0,
            glow_max: 0.0,
            chest: 0.0,
            glow_d: 0.0,
            glow_b: 0.0,
            glow_x: 0.0,
            glow_z: 0.0,
            glow_lamps: 0.0,
            rho: 0.0,
        }
    }
}
impl Default for GRID { fn default() -> Self { Self::zeroed() } }

/// (src/game/sense.ts:90 PI15)
pub const PI15: f64 = 5.568327996831708;

/// (src/game/sense.ts:184 GRID: a table nothing writes)
pub fn grid() -> crate::sense::GRID { crate::sense::GRID { step: 0.1, head: 8.0, emitters: 64.0, emit: 7.0, bricks: 160.0, cells: 128.0, vol: 0.125, out: 4.0, given: 5.0, lost: 6.0, nbx: 2.0, nbz: 3.0, side: 2.0, glow_max: 2048.0, chest: 1.2, glow_d: 776.0, glow_b: 2824.0, glow_x: 4872.0, glow_z: 6920.0, glow_lamps: 8968.0, rho: 8969.0 } }

/// (src/game/sense.ts:194 GRID_STEP)
pub const GRID_STEP: f64 = 0.1;
