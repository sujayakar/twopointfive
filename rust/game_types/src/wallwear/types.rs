//! The types of `wallwear`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, ToBytes)]
pub struct WallWear {
    pub on: bool,
}
impl WallWear {
    /// what a new one holds
    pub fn blank() -> Self {
        WallWear {
            on: true,
        }
    }
}
impl Default for WallWear { fn default() -> Self { Self::blank() } }

#[derive(Clone, Copy, Debug, Default)]
pub struct WEAR {
    pub curve: f64,
    pub vary: f64,
    pub exit_chips_from: f64,
    pub max_births: f64,
    pub max_cells: f64,
}

/// (src/game/wallwear.ts:32 WEAR: a table nothing writes)
pub fn wear() -> crate::wallwear::WEAR { crate::wallwear::WEAR { curve: 0.4, vary: 0.25, exit_chips_from: 0.3, max_births: 16.0, max_cells: 256.0 } }

