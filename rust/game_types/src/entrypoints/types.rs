//! The types of `entrypoints`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct Pod {
    pub pos: V3,
    pub face: f64,
    pub watch: Option<V3>,
    /// INVENTED: this point was moved off an opening or swapped in, read by dealt for the tallies (:172)
    pub moved: bool,
}

#[derive(Clone, Debug, ToBytes)]
pub struct EntryPoints {
    pub been: JsMap<CoverOpeningId, f64>,
    pub last: f64,
    pub stats: crate::entrypoints::EntryPointsStats,
    pub log: Vec<String>,
}
impl EntryPoints {
    /// what a new one holds
    pub fn blank() -> Self {
        EntryPoints {
            been: JsMap::new(),
            last: -1.0,
            stats: crate::entrypoints::EntryPointsStats { pods: 0.0, risky: 0.0, moved: 0.0, kept: 0.0 },
            log: Vec::new(),
        }
    }
}
impl Default for EntryPoints { fn default() -> Self { Self::blank() } }

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct EntryPointsStats {
    pub pods: f64,
    pub risky: f64,
    pub moved: f64,
    pub kept: f64,
}

pub const EYE: f64 = 1.55;

pub const CHEST: f64 = 1.1;
