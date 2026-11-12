//! What falls and what it leaves: the landings waiting, and the glass lying on the floor. The rules are `rust/sim/src/falls.rs`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Words, V3};
use alloc::vec::Vec;

/// is the floor told of a landing
#[derive(Clone, Copy, PartialEq, Debug, ToBytes)]
pub enum FallTold { Never, IfItSounds, Always }

/// each kind of thing that falls has its own list, so that one kind's cap, gap and going inert never move another's
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum FallKind { Glass, Pieces }

/// one thing on its way down: when and where it lands, how it sounds, the glass it leaves, whose it was, the hearing level it is told at (0: the rule's own) and the dust it raises (0: none)
#[derive(Clone, Copy, Debug, ToBytes)]
pub struct Landing {
    pub t: f64,
    pub pos: V3,
    pub sound: Option<crate::audio_iface::Sfx>,
    pub gain: f64,
    pub rate: f64,
    pub amount: f64,
    pub who: f64,
    pub told: FallTold,
    pub level: f64,
    pub dust: f64,
}

/// the landings of one kind, in the order they were scheduled, and when the floor was last told of one
#[derive(Clone, Debug, ToBytes)]
pub struct FallList {
    pub waiting: Vec<Landing>,
    pub last_told: f64,
}

/// the time a list's memory holds when nothing has landed yet
pub const FALL_NEVER: f64 = -1e9;

/// a list and the floor's glass
#[derive(Clone, Debug, ToBytes)]
pub struct Falls {
    pub glass: FallList,
    pub pieces: FallList,
    /// m² of glass in each half-metre cell of the floor, row by row; empty while none has been laid
    pub litter: Vec<f64>,
}
impl Falls {
    pub fn blank() -> Self {
        let list = || FallList { waiting: Vec::new(), last_told: FALL_NEVER };
        Falls { glass: list(), pieces: list(), litter: Vec::new() }
    }
}
impl Default for Falls { fn default() -> Self { Self::blank() } }

/// a landing that has come
#[derive(Clone, Copy, Debug)]
pub struct FallDue {
    pub sound: Option<crate::audio_iface::Sfx>,
    pub pos: V3,
    pub gain: f64,
    pub rate: f64,
    pub heard: bool,
    pub level: f64,
    pub who: f64,
    pub dust: f64,
}

/// a footfall: the level to file, and how much glass was underfoot (0‥1; 0 = no crunch)
#[derive(Clone, Copy, Debug, Default)]
pub struct FallStep {
    pub level: f64,
    pub k: f64,
}
