//! The types of `render_fogofwar`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum FogPassMode {
    Hint,
    Old,
}
impl FogPassMode {
    pub const fn as_str(self) -> &'static str {
        match self {
            FogPassMode::Hint => "hint",
            FogPassMode::Old => "old",
        }
    }
}

/// (TypeScript: FogFrame, src/render/fogofwar.ts:31)
#[derive(Clone, Debug)]
pub struct FogFrame {
    pub on: bool,
    pub count: f64,
    pub eyes: Vec<f32>,
    pub looks: Vec<f32>,
    pub dim: f64,
    pub desat: f64,
    pub top_pad: f64,
    pub edge: f64,
    pub soft_in: f64,
    pub soft_out: f64,
    pub near_soft: f64,
    pub flat: f64,
    pub lit: f64,
    pub lit_fill: f64,
    pub lit_cut: f64,
    pub lit_ao: f64,
    pub lit_ao_m: f64,
    pub light_count: f64,
    pub lights: Vec<f32>,
    pub lights_u: Vec<u32>,
    pub full: bool,
    pub full_flat: f64,
    pub air: bool,
    pub thin: bool,
    pub kept: bool,
    pub thin_bits: Vec<u32>,
    pub thin_serial: f64,
    pub thin_count: f64,
}
impl FogFrame {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FogFrame {
            on: false,
            count: 0.0,
            eyes: Vec::new(),
            looks: Vec::new(),
            dim: 0.0,
            desat: 0.0,
            top_pad: 0.0,
            edge: 0.0,
            soft_in: 0.0,
            soft_out: 0.0,
            near_soft: 0.0,
            flat: 0.0,
            lit: 0.0,
            lit_fill: 0.0,
            lit_cut: 0.0,
            lit_ao: 0.0,
            lit_ao_m: 0.0,
            light_count: 0.0,
            lights: Vec::new(),
            lights_u: Vec::new(),
            full: false,
            full_flat: 0.0,
            air: false,
            thin: false,
            kept: false,
            thin_bits: Vec::new(),
            thin_serial: 0.0,
            thin_count: 0.0,
        }
    }
}
impl Default for FogFrame { fn default() -> Self { Self::zeroed() } }

/// (src/render/fogofwar.ts:21 FOG_EYES)
pub const FOG_EYES: f64 = 7.0;

/// (src/render/fogofwar.ts:25 FOG_LIGHTS)
pub const FOG_LIGHTS: f64 = 48.0;

/// (src/render/fogofwar.ts:261 FOG_THIN_WORDS)
pub const FOG_THIN_WORDS: f64 = 128.0;
