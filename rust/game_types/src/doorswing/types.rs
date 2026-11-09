//! The types of `doorswing`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, Words, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, ToBytes, Default)]
pub struct SwingDeal {
    pub swing: f64,
    pub why: crate::doorswing::SwingWhy,
    pub into: Option<String>,
    pub out: Option<alloc::boxed::Box<crate::doorswing::SwingDeal>>,
    pub clear: Option<alloc::boxed::Box<crate::doorswing::SwingDeal>>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default, Words)]
pub enum SwingWhy {
    #[default] Mission,
    Floor,
    Authored,
    Folds,
    Outside,
    Plan,
    Room,
    Hostages,
    Lock,
    #[word("stands open")] StandsOpen,
    Wider,
    #[word("leaf room")] LeafRoom,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Rect {
    pub x0: f64,
    pub x1: f64,
    pub z0: f64,
    pub z1: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct FloorsValue {
    pub plus: f64,
    pub minus: f64,
}

/// (src/game/doorswing.ts:38 MIN_OPEN)
pub const MIN_OPEN: f64 = 0.9;

/// (src/game/doorswing.ts:51 ROOF_PTS)
pub const ROOF_PTS: f64 = 6.0;

/// (src/game/doorswing.ts:51 ROOF_UNDER)
pub const ROOF_UNDER: f64 = 1.9;

/// (src/game/doorswing.ts:108 LEAF_MAN)
pub const LEAF_MAN: f64 = 0.3;

/// (src/game/doorswing.ts:108 LEAF_STEP)
pub const LEAF_STEP: f64 = 0.1;

/// (src/game/doorswing.ts:108 LEAF_WIN)
pub const LEAF_WIN: f64 = 4.0;

/// (src/game/doorswing.ts:108 LEAF_REST)
pub const LEAF_REST: f64 = 1.84;

/// (src/game/doorswing.ts:108 LEAF_CUT)
pub const LEAF_CUT: f64 = 1.0;

/// (src/game/doorswing.ts:108 LEAF_RING)
pub const LEAF_RING: f64 = 0.25;

/// (src/game/doorswing.ts:108 LEAF_WALL)
pub const LEAF_WALL: f64 = 0.5;
