//! The types of `sim_fallshome`.
#![allow(unused_imports)]
use crate::{Blank, Words, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Words)]
pub enum FallSound {
    #[word("")] #[default] Empty,
    GlassTinkle,
    GlassShatter,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct FallDue {
    pub name: crate::sim_fallshome::FallSound,
    pub pos: [f64; 3],
    pub gain: f64,
    pub rate: f64,
    pub heard: bool,
    pub level: f64,
    pub who: f64,
    pub dust: f64,
}

#[derive(Clone, Copy, Debug, ToBytes)]
pub struct FallStore {
}
impl FallStore {
    /// what a new one holds
    pub fn blank() -> Self {
        FallStore {
        }
    }
}
impl Default for FallStore { fn default() -> Self { Self::blank() } }

#[derive(Clone, Debug, Default)]
pub struct SimFallshomeModFALLS {
    pub there: f64,
    pub dues: f64,
    pub steps: f64,
    pub crunches: f64,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum FallStoreLandingKey {
    Dust,
    X,
    T,
    Y,
    Z,
    Name,
    Level,
    Who,
    Event,
    Gain,
    Rate,
    Amount,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct FallStoreStep {
    pub level: f64,
    pub k: f64,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, Blank)]
pub struct SimFallshomeMod {
    #[is(crate::sim_fallshome::SimFallshomeModFALLS { there: 0.0, dues: 0.0, steps: 0.0, crunches: 0.0 })] pub falls: crate::sim_fallshome::SimFallshomeModFALLS,
}

pub const FALL_CELL: f64 = 0.5;

pub const FALL_CELLS_X: f64 = 80.0;

pub const FALL_CELLS_Z: f64 = 56.0;

pub const FALL_MAXP: f64 = 96.0;

pub const FALL_GLASS: f64 = 0.0;

pub const FALL_PIECES: f64 = 1.0;

pub const FALL_LISTS: f64 = 2.0;

pub const NCELLS: f64 = 4480.0;

pub const FALL_UNHEARD: f64 = 0.0;

pub const FALL_IF_SOUNDED: f64 = 1.0;

pub const FALL_HEARD: f64 = 2.0;

pub const FALL_BIT: f64 = 8.0;

pub fn none() -> Vec<crate::sim_fallshome::FallDue> { Vec::new() }
