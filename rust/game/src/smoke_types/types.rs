//! The types of `smoke_types`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: SmokeEmitter, src/smoke/types.ts:6)
#[derive(Clone, Copy, Debug)]
pub struct SmokeEmitter {
    pub pos: V3,
    pub radius: f64,
    pub density: f64,
    pub ttl: f64,
    pub look: Option<bool>,
}
impl SmokeEmitter {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SmokeEmitter {
            pos: [0.0; 3],
            radius: 0.0,
            density: 0.0,
            ttl: 0.0,
            look: None,
        }
    }
}
impl Default for SmokeEmitter { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: SmokePush, src/smoke/types.ts:35)
#[derive(Clone, Copy, Debug)]
pub struct SmokePush {
    /// INVENTED by a review: see the proposals
    pub pos: V3,
    /// INVENTED by a review: see the proposals
    pub dir: V3,
    /// INVENTED by a review: see the proposals
    pub speed: f64,
    /// INVENTED by a review: see the proposals
    pub radius: f64,
}
impl SmokePush {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SmokePush {
            pos: [0.0; 3],
            dir: [0.0; 3],
            speed: 0.0,
            radius: 0.0,
        }
    }
}
impl Default for SmokePush { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: WindGpuSource, src/smoke/types.ts:54)
#[derive(Clone, Debug)]
pub struct WindGpuSource {
    pub kind: crate::smoke_types::WindGpuSourceKind,
    pub name: String,
    pub pos: V3,
    pub dir: V3,
    pub speed: f64,
    pub reach: f64,
    pub tan_spread: f64,
    pub r0: f64,
    pub d_max: f64,
    pub floor_y: Option<f64>,
    pub interior: Option<[f64; 4]>,
    pub annex: Option<[f64; 4]>,
    pub roofs: Option<Vec<Vec<f64>>>,
    pub field: Option<bool>,
    pub bound: crate::smoke_types::WindGpuSourceBound,
}
impl WindGpuSource {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WindGpuSource {
            kind: crate::smoke_types::WindGpuSourceKind::Jet,
            name: String::new(),
            pos: [0.0; 3],
            dir: [0.0; 3],
            speed: 0.0,
            reach: 0.0,
            tan_spread: 0.0,
            r0: 0.0,
            d_max: 0.0,
            floor_y: None,
            interior: None,
            annex: None,
            roofs: None,
            field: None,
            bound: crate::smoke_types::WindGpuSourceBound::zeroed(),
        }
    }
}
impl Default for WindGpuSource { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum WindGpuSourceKind {
    Jet,
    Storm,
}
impl WindGpuSourceKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            WindGpuSourceKind::Jet => "jet",
            WindGpuSourceKind::Storm => "storm",
        }
    }
}

/// (TypeScript: inline at src/smoke/types.ts:54 WindGpuSource.bound, src/smoke/types.ts:54)
#[derive(Clone, Copy, Debug)]
pub struct WindGpuSourceBound {
    pub c: V3,
    pub r: f64,
}
impl WindGpuSourceBound {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WindGpuSourceBound {
            c: [0.0; 3],
            r: 0.0,
        }
    }
}
impl Default for WindGpuSourceBound { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: CanisterParams, src/smoke/types.ts:59)
#[derive(Clone, Copy, Debug)]
pub struct CanisterParams {
    pub canister_duration: f64,
    pub canister_density: f64,
    pub canister_radius: f64,
    pub canister_temp: f64,
    pub canister_speed: f64,
}
impl CanisterParams {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CanisterParams {
            canister_duration: 0.0,
            canister_density: 0.0,
            canister_radius: 0.0,
            canister_temp: 0.0,
            canister_speed: 0.0,
        }
    }
}
impl Default for CanisterParams { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: SmokeSystemLike, src/smoke/types.ts:61)
#[derive(Clone, Copy, Debug)]
pub struct SmokeSystemLike {
    pub params: Option<crate::smoke_types::CanisterParams>,
}
impl SmokeSystemLike {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SmokeSystemLike {
            params: None,
        }
    }
}
impl Default for SmokeSystemLike { fn default() -> Self { Self::zeroed() } }

/// (src/smoke/types.ts:56 WIND_GPU_MAX)
pub const WIND_GPU_MAX: f64 = 8.0;
