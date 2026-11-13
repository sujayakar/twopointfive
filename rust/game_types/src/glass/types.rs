//! The types of `glass`.
use crate::{Blank, bytes::ToBytes, js::*};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, ToBytes, Default)]
pub struct GlassState {
    pub panes: Vec<f64>,
    pub seen: JsSet<u32>,
    pub hits: f64,
    pub last_hit_t: f64,
    pub log: Vec<String>,
    pub crack_at: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct GlassImpactOpts {
    pub through: Option<bool>,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct GlassMod {
    pub states: Option<crate::glass::GlassState>,
}

pub const THICK_GLASS_HALF: f64 = 0.05;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum GlassImpactCause {
    Shot,
    Bang,
    Hand,
}
