//! The types of `glass`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
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
pub struct GLASSDEFAULTS {
    pub crack_gap: f64,
    pub live_gap: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct NearestOnBox {
    pub q: V3,
    pub d: f64,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum GlassImpactCause {
    Shot,
    Bang,
    Hand,
}
impl GlassImpactCause {
    pub const fn as_str(self) -> &'static str {
        match self {
            GlassImpactCause::Shot => "shot",
            GlassImpactCause::Bang => "bang",
            GlassImpactCause::Hand => "hand",
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct GlassImpactOpts {
    pub through: Option<bool>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum GlassHitCause {
    Shot,
    Bang,
    Hand,
}
impl GlassHitCause {
    pub const fn as_str(self) -> &'static str {
        match self {
            GlassHitCause::Shot => "shot",
            GlassHitCause::Bang => "bang",
            GlassHitCause::Hand => "hand",
        }
    }
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes)]
pub struct GlassMod {
    /// (src/game/glass.ts:45 fxForced)
    pub fx_forced: Option<bool>,
    /// (src/game/glass.ts:59 states)
    pub states: Option<crate::glass::GlassState>,
}
impl GlassMod {
    pub fn blank() -> Self {
        GlassMod {
            fx_forced: None,
            states: None,
        }
    }
}

/// (src/game/glass.ts:43 GLASS_DEFAULTS: a table nothing writes)
pub fn glass_defaults() -> crate::glass::GLASSDEFAULTS { crate::glass::GLASSDEFAULTS { crack_gap: 0.045, live_gap: 0.2 } }

/// (src/game/glass.ts:46 FX_OFF)
pub const FX_OFF: &str = "/^(0|off|false|no)$/i";

/// (src/game/glass.ts:46 FX_KEY)
pub const FX_KEY: &str = "twopointsix.glassfx";

/// (src/game/glass.ts:74 THICK_GLASS_HALF)
pub const THICK_GLASS_HALF: f64 = 0.05;
