//! The types of `stature`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: Fit, src/game/stature.ts:84)
#[derive(Clone, Debug)]
pub struct Fit {
    pub r0: Vec<f32>,
    pub r1: Vec<f32>,
    pub pad: Option<f64>,
}
impl Fit {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Fit {
            r0: Vec::new(),
            r1: Vec::new(),
            pad: None,
        }
    }
}
impl Default for Fit { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/stature.ts:31 STATURE_TUNE, src/game/stature.ts:31)
#[derive(Clone, Copy, Debug)]
pub struct STATURETUNE {
    pub jitter: f64,
    pub min: f64,
    pub max: f64,
    pub ease_per_sec: f64,
}
impl STATURETUNE {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        STATURETUNE {
            jitter: 0.0,
            min: 0.0,
            max: 0.0,
            ease_per_sec: 0.0,
        }
    }
}
impl Default for STATURETUNE { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/stature.ts:85 fits, src/game/stature.ts:85)
#[derive(Clone, Debug)]
pub struct StatureModFitsValue {
    pub of: crate::stature::Fit,
    pub s: f64,
    pub fit: crate::stature::Fit,
}
impl StatureModFitsValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StatureModFitsValue {
            of: crate::stature::Fit::zeroed(),
            s: 0.0,
            fit: crate::stature::Fit::zeroed(),
        }
    }
}
impl Default for StatureModFitsValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/stature.ts:79 drawnCapsules(P), src/game/stature.ts:79)
#[derive(Clone, Debug)]
pub struct DrawnCapsulesP {
    pub caps: Vec<Vec<f64>>,
    pub bound: [f64; 4],
}
impl DrawnCapsulesP {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DrawnCapsulesP {
            caps: Vec::new(),
            bound: [0.0; 4],
        }
    }
}
impl Default for DrawnCapsulesP { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/stature.ts:79 drawnCapsules ->, src/game/stature.ts:79)
#[derive(Clone, Debug)]
pub struct DrawnCapsules {
    pub caps: Vec<Vec<f64>>,
    pub bound: [f64; 4],
}
impl DrawnCapsules {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DrawnCapsules {
            caps: Vec::new(),
            bound: [0.0; 4],
        }
    }
}
impl Default for DrawnCapsules { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/stature.ts:105 drawnBeam ->, src/game/stature.ts:105)
#[derive(Clone, Copy, Debug)]
pub struct DrawnBeam {
    pub from: V3,
    pub dir: V3,
    pub k: f64,
}
impl DrawnBeam {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DrawnBeam {
            from: [0.0; 3],
            dir: [0.0; 3],
            k: 0.0,
        }
    }
}
impl Default for DrawnBeam { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct StatureMod {
    /// (src/game/stature.ts:47 shown)
    pub shown: JsMap<CharId, f64>,
    /// (src/game/stature.ts:85 fits)
    pub fits: JsMap<CharId, crate::stature::StatureModFitsValue>,
    /// (src/game/stature.ts:103 byId)
    pub by_id: Vec<Option<CharId>>,
}
impl StatureMod {
    pub fn blank() -> Self {
        StatureMod {
            shown: JsMap::new(),
            fits: JsMap::new(),
            by_id: alloc::vec![None; 256],
        }
    }
}

/// (src/game/stature.ts:26 STATURE: a table nothing writes)
pub fn stature() -> JsMap<String, f64> { { let mut t = JsMap::new(); t.set(String::from("civilian_1"), 0.93); t.set(String::from("defender_2"), 0.97); t.set(String::from("defender_3"), 1.04); t.set(String::from("operator_2"), 1.02); t.set(String::from("operator_3"), 0.99); t.set(String::from("operator_4"), 1.03); t.set(String::from("operator_5"), 1.0); t } }

/// (src/game/stature.ts:31 STATURE_TUNE: a table nothing writes)
pub fn stature_tune() -> crate::stature::STATURETUNE { crate::stature::STATURETUNE { jitter: 0.015, min: 0.92, max: 1.05, ease_per_sec: 0.25 } }

/// (src/game/stature.ts:65 palette: a table nothing writes)
pub fn palette() -> Vec<f32> { alloc::vec![0.0; 1024] }
