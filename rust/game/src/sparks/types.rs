//! The types of `sparks`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: SparkParams, src/game/sparks.ts:10)
#[derive(Clone, Copy, Debug)]
pub struct SparkParams {
    pub gravity: f64,
    pub drag: f64,
    pub hold: f64,
    pub brightness: f64,
    pub max_live: f64,
    pub emissive: f64,
    pub thickness: f64,
    pub shutter: f64,
    pub min_streak: f64,
    pub collide: bool,
}
impl SparkParams {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SparkParams {
            gravity: 0.0,
            drag: 0.0,
            hold: 0.0,
            brightness: 0.0,
            max_live: 0.0,
            emissive: 0.0,
            thickness: 0.0,
            shutter: 0.0,
            min_streak: 0.0,
            collide: false,
        }
    }
}
impl Default for SparkParams { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Spark, src/game/sparks.ts:23)
#[derive(Clone, Copy, Debug)]
pub struct Spark {
    pub pos: V3,
    pub vel: V3,
    pub age: f64,
    pub life: f64,
    pub alive: bool,
    pub gen_: f64,
}
impl Spark {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Spark {
            pos: [0.0; 3],
            vel: [0.0; 3],
            age: 0.0,
            life: 0.0,
            alive: false,
            gen_: 0.0,
        }
    }
}
impl Default for Spark { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Sparks, src/game/sparks.ts:25)
#[derive(Clone, Debug)]
pub struct Sparks {
    pub pool: Vec<crate::sparks::Spark>,
    pub live: f64,
    pub params: crate::sparks::SparkParams,
}
impl Sparks {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        Sparks {
            pool: Vec::new(),
            live: 0.0,
            params: crate::sparks::SparkParams::zeroed(),
        }
    }
}
impl Default for Sparks { fn default() -> Self { Self::blank() } }

/// (TypeScript: inline at src/game/sparks.ts:48 emitCone(o), src/game/sparks.ts:48)
#[derive(Clone, Copy, Debug)]
pub struct SparksEmitConeO {
    pub count: f64,
    pub speed: f64,
    pub cone: f64,
    pub life: f64,
}
impl SparksEmitConeO {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SparksEmitConeO {
            count: 0.0,
            speed: 0.0,
            cone: 0.0,
            life: 0.0,
        }
    }
}
impl Default for SparksEmitConeO { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/sparks.ts:61 emitBurst(o), src/game/sparks.ts:61)
#[derive(Clone, Copy, Debug)]
pub struct SparksEmitBurstO {
    pub count: f64,
    pub speed: f64,
    pub spread: f64,
    pub lift: f64,
    pub life: f64,
}
impl SparksEmitBurstO {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SparksEmitBurstO {
            count: 0.0,
            speed: 0.0,
            spread: 0.0,
            lift: 0.0,
            life: 0.0,
        }
    }
}
impl Default for SparksEmitBurstO { fn default() -> Self { Self::zeroed() } }
