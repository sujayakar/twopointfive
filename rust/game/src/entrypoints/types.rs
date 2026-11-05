//! The types of `entrypoints`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: Pod, src/game/entrypoints.ts:64)
#[derive(Clone, Copy, Debug)]
pub struct Pod {
    pub pos: V3,
    pub face: f64,
    pub watch: Option<V3>,
    /// INVENTED: this point was moved off an opening or swapped in (entrypoints.ts:163, :170), read by dealt for the tallies (:172)
    pub moved: bool,
}
impl Pod {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Pod {
            pos: [0.0; 3],
            face: 0.0,
            watch: None,
            moved: false,
        }
    }
}
impl Default for Pod { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: EntryPoints, src/game/entrypoints.ts:67)
#[derive(Clone, Debug)]
pub struct EntryPoints {
    pub knobs: crate::knobs::EntryK,
    pub been: JsMap<CoverOpeningId, f64>,
    pub last: f64,
    pub stats: crate::entrypoints::EntryPointsStats,
    pub log: Vec<String>,
}
impl EntryPoints {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        EntryPoints {
            knobs: crate::knobs::EntryK { off_openings: true, move_: true, watch: true, near_m: 3.0, line_deg: 45.0, gaps: false, known_m: 4.0, move_m: 3.0, apart: 0.9, off_mouth: false, mouth_m: 1.8 },
            been: JsMap::new(),
            last: -1.0,
            stats: crate::entrypoints::EntryPointsStats { pods: 0.0, risky: 0.0, moved: 0.0, kept: 0.0 },
            log: Vec::new(),
        }
    }
}
impl Default for EntryPoints { fn default() -> Self { Self::blank() } }

/// (TypeScript: inline at src/game/entrypoints.ts:73 EntryPoints.stats, src/game/entrypoints.ts:73)
#[derive(Clone, Copy, Debug)]
pub struct EntryPointsStats {
    pub pods: f64,
    pub risky: f64,
    pub moved: f64,
    pub kept: f64,
}
impl EntryPointsStats {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        EntryPointsStats {
            pods: 0.0,
            risky: 0.0,
            moved: 0.0,
            kept: 0.0,
        }
    }
}
impl Default for EntryPointsStats { fn default() -> Self { Self::zeroed() } }

/// (src/game/entrypoints.ts:65 EYE)
pub const EYE: f64 = 1.55;

/// (src/game/entrypoints.ts:65 CHEST)
pub const CHEST: f64 = 1.1;
