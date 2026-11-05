//! The types of `glass`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: GlassState, src/game/glass.ts:56)
#[derive(Clone, Debug)]
pub struct GlassState {
    pub panes: Vec<f64>,
    pub seen: JsSet<u32>,
    pub hits: f64,
    pub last_hit_t: f64,
    pub log: Vec<String>,
    pub event_at: JsMap<crate::Num, f64>,
    pub crack_at: f64,
}
impl GlassState {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GlassState {
            panes: Vec::new(),
            seen: JsSet::new(),
            hits: 0.0,
            last_hit_t: 0.0,
            log: Vec::new(),
            event_at: JsMap::new(),
            crack_at: 0.0,
        }
    }
}
impl Default for GlassState { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/glass.ts:41 GLASS_DEFAULTS, src/game/glass.ts:41)
#[derive(Clone, Copy, Debug)]
pub struct GLASSDEFAULTS {
    pub crack_gap: f64,
    pub live_gap: f64,
}
impl GLASSDEFAULTS {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GLASSDEFAULTS {
            crack_gap: 0.0,
            live_gap: 0.0,
        }
    }
}
impl Default for GLASSDEFAULTS { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/glass.ts:93 nearestOnBox ->, src/game/glass.ts:93)
#[derive(Clone, Copy, Debug)]
pub struct NearestOnBox {
    pub q: V3,
    pub d: f64,
}
impl NearestOnBox {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        NearestOnBox {
            q: [0.0; 3],
            d: 0.0,
        }
    }
}
impl Default for NearestOnBox { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
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

/// (TypeScript: inline at src/game/glass.ts:139 glassImpact(opts), src/game/glass.ts:139)
#[derive(Clone, Copy, Debug)]
pub struct GlassImpactOpts {
    pub through: Option<bool>,
}
impl GlassImpactOpts {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GlassImpactOpts {
            through: None,
        }
    }
}
impl Default for GlassImpactOpts { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
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
#[derive(Clone, Debug)]
pub struct GlassMod {
    /// (src/game/glass.ts:43 fxForced)
    pub fx_forced: Option<bool>,
    /// (src/game/glass.ts:57 states)
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

/// (src/game/glass.ts:41 GLASS_DEFAULTS: a table nothing writes)
pub fn glass_defaults() -> crate::glass::GLASSDEFAULTS { crate::glass::GLASSDEFAULTS { crack_gap: 0.045, live_gap: 0.2 } }

/// (src/game/glass.ts:44 FX_OFF)
pub const FX_OFF: &str = "/^(0|off|false|no)$/i";

/// (src/game/glass.ts:44 FX_KEY)
pub const FX_KEY: &str = "twopointsix.glassfx";

/// (src/game/glass.ts:72 THICK_GLASS_HALF)
pub const THICK_GLASS_HALF: f64 = 0.05;
