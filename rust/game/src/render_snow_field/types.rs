//! The types of `render_snow_field`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: SnowField, src/render/snowField.ts:45)
#[derive(Clone, Debug)]
pub struct SnowField {
    pub data: Vec<u16>,
    pub w: f64,
    pub h: f64,
    pub quads: Vec<u32>,
    pub slab: f64,
    pub statics: f64,
    pub paint: [f64; 3],
    pub wind: [f64; 2],
    pub mound: Vec<f32>,
    pub cover: Vec<f32>,
    pub packed: Vec<f32>,
    pub old: Vec<f32>,
    pub roof: Vec<f32>,
    pub wall: Vec<u8>,
    pub hard: Vec<f32>,
    pub base: Option<Vec<f32>>,
    pub kept: Option<Vec<crate::render_snow_field::SnowFieldKeptItem>>,
    pub walks: Option<f64>,
    pub stats: crate::render_snow_field::SnowFieldStats,
}
impl SnowField {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SnowField {
            data: Vec::new(),
            w: 0.0,
            h: 0.0,
            quads: Vec::new(),
            slab: 0.0,
            statics: 0.0,
            paint: [0.0; 3],
            wind: [0.0; 2],
            mound: Vec::new(),
            cover: Vec::new(),
            packed: Vec::new(),
            old: Vec::new(),
            roof: Vec::new(),
            wall: Vec::new(),
            hard: Vec::new(),
            base: None,
            kept: None,
            walks: None,
            stats: crate::render_snow_field::SnowFieldStats::zeroed(),
        }
    }
}
impl Default for SnowField { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/render/snowField.ts:65 SnowField.kept, src/render/snowField.ts:65)
#[derive(Clone, Debug)]
pub struct SnowFieldKeptItem {
    pub x: f64,
    pub z: f64,
    pub r: f64,
    pub seed: f64,
    pub stood: Vec<f64>,
}
impl SnowFieldKeptItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SnowFieldKeptItem {
            x: 0.0,
            z: 0.0,
            r: 0.0,
            seed: 0.0,
            stood: Vec::new(),
        }
    }
}
impl Default for SnowFieldKeptItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/render/snowField.ts:66 SnowField.stats, src/render/snowField.ts:66)
#[derive(Clone, Copy, Debug)]
pub struct SnowFieldStats {
    pub mounds: f64,
    pub lays: f64,
    pub bare: f64,
    pub roofs: f64,
    pub quads: f64,
    pub max_mound: f64,
}
impl SnowFieldStats {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SnowFieldStats {
            mounds: 0.0,
            lays: 0.0,
            bare: 0.0,
            roofs: 0.0,
            quads: 0.0,
            max_mound: 0.0,
        }
    }
}
impl Default for SnowFieldStats { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/render/snowField.ts:25 SNOW_SURF, src/render/snowField.ts:25)
#[derive(Clone, Copy, Debug)]
pub struct SNOWSURF {
    pub fresh: [f64; 3],
    pub packed: [f64; 3],
    pub old: [f64; 3],
    pub relief: f64,
    pub roll: f64,
    pub ripple: f64,
    pub dimple: f64,
    pub hollow_blue: f64,
    pub sparkle: f64,
    pub sparkle_share: f64,
    pub sparkle_rough: f64,
    pub sparkle_lean: f64,
    pub rough: f64,
    pub pillow: f64,
    pub cap_wave: f64,
    pub track_deep: f64,
    pub track_rim: f64,
}
impl SNOWSURF {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SNOWSURF {
            fresh: [0.0; 3],
            packed: [0.0; 3],
            old: [0.0; 3],
            relief: 0.0,
            roll: 0.0,
            ripple: 0.0,
            dimple: 0.0,
            hollow_blue: 0.0,
            sparkle: 0.0,
            sparkle_share: 0.0,
            sparkle_rough: 0.0,
            sparkle_lean: 0.0,
            rough: 0.0,
            pillow: 0.0,
            cap_wave: 0.0,
            track_deep: 0.0,
            track_rim: 0.0,
        }
    }
}
impl Default for SNOWSURF { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/render/snowField.ts:79 snowPaintOf(b), src/render/snowField.ts:79)
#[derive(Clone, Debug)]
pub struct SnowPaintOfB {
    pub does: Option<String>,
}
impl SnowPaintOfB {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SnowPaintOfB {
            does: None,
        }
    }
}
impl Default for SnowPaintOfB { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct RenderSnowFieldMod {
    /// (src/render/snowField.ts:95 f32)
    pub f32: Vec<f32>,
}
impl RenderSnowFieldMod {
    pub fn blank() -> Self {
        RenderSnowFieldMod {
            f32: alloc::vec![0.0; 1],
        }
    }
}

/// (src/render/snowField.ts:23 SNOW_RES)
pub const SNOW_RES: f64 = 10.0;

/// (src/render/snowField.ts:25 SNOW_SURF: a table nothing writes)
pub fn snow_surf() -> crate::render_snow_field::SNOWSURF { crate::render_snow_field::SNOWSURF { fresh: [0.76, 0.83, 0.95], packed: [0.55, 0.61, 0.71], old: [0.36, 0.39, 0.44], relief: 2.4, roll: 0.035, ripple: 0.03, dimple: 0.014, hollow_blue: 0.5, sparkle: 1.0, sparkle_share: 0.08, sparkle_rough: 0.28, sparkle_lean: 0.06, rough: 0.92, pillow: 0.22, cap_wave: 0.06, track_deep: 0.08, track_rim: 0.012 } }

/// (src/render/snowField.ts:95 u32: a table nothing writes)
pub fn u32() -> Vec<u32> { todo!() }

/// (src/render/snowField.ts:311 SNOW_CFG_FLOATS)
pub const SNOW_CFG_FLOATS: f64 = 44.0;
