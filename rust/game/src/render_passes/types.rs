//! The types of `render_passes`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: FireRecord, src/render/passes.ts:110)
#[derive(Clone, Debug)]
pub struct FireRecord {
    pub pos: Vec<f64>,
    pub r: f64,
    pub h: f64,
    pub heat: f64,
    pub seed: f64,
    pub time: f64,
    pub wind: Vec<f64>,
    pub flick: f64,
    pub bed: Option<f64>,
    pub fog: Option<f64>,
    pub gas: Option<f64>,
    pub soot: Option<f64>,
}
impl FireRecord {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FireRecord {
            pos: Vec::new(),
            r: 0.0,
            h: 0.0,
            heat: 0.0,
            seed: 0.0,
            time: 0.0,
            wind: Vec::new(),
            flick: 0.0,
            bed: None,
            fog: None,
            gas: None,
            soot: None,
        }
    }
}
impl Default for FireRecord { fn default() -> Self { Self::zeroed() } }
