//! The types of `doorswing`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: SwingDeal, src/game/doorswing.ts:31)
#[derive(Clone, Debug)]
pub struct SwingDeal {
    pub door: String,
    pub swing: f64,
    pub why: crate::doorswing::SwingWhy,
    pub into: Option<String>,
    pub out: Option<alloc::boxed::Box<crate::doorswing::SwingDeal>>,
    pub clear: Option<alloc::boxed::Box<crate::doorswing::SwingDeal>>,
}
impl SwingDeal {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SwingDeal {
            door: String::new(),
            swing: 0.0,
            why: crate::doorswing::SwingWhy::Mission,
            into: None,
            out: None,
            clear: None,
        }
    }
}
impl Default for SwingDeal { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum SwingWhy {
    Mission,
    Floor,
    Authored,
    Folds,
    Outside,
    Plan,
    Room,
    Hostages,
    Lock,
    StandsOpen,
    Wider,
    LeafRoom,
}
impl SwingWhy {
    pub const fn as_str(self) -> &'static str {
        match self {
            SwingWhy::Mission => "mission",
            SwingWhy::Floor => "floor",
            SwingWhy::Authored => "authored",
            SwingWhy::Folds => "folds",
            SwingWhy::Outside => "outside",
            SwingWhy::Plan => "plan",
            SwingWhy::Room => "room",
            SwingWhy::Hostages => "hostages",
            SwingWhy::Lock => "lock",
            SwingWhy::StandsOpen => "stands open",
            SwingWhy::Wider => "wider",
            SwingWhy::LeafRoom => "leaf room",
        }
    }
}

/// (TypeScript: Rect, src/game/doorswing.ts:36)
#[derive(Clone, Copy, Debug)]
pub struct Rect {
    pub x0: f64,
    pub x1: f64,
    pub z0: f64,
    pub z1: f64,
}
impl Rect {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Rect {
            x0: 0.0,
            x1: 0.0,
            z0: 0.0,
            z1: 0.0,
        }
    }
}
impl Default for Rect { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/doorswing.ts:70 floors ->, src/game/doorswing.ts:70)
#[derive(Clone, Copy, Debug)]
pub struct FloorsValue {
    pub plus: f64,
    pub minus: f64,
}
impl FloorsValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FloorsValue {
            plus: 0.0,
            minus: 0.0,
        }
    }
}
impl Default for FloorsValue { fn default() -> Self { Self::zeroed() } }

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
