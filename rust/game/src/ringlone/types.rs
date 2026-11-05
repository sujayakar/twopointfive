//! The types of `ringlone`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: Wedge, src/game/ringlone.ts:43)
#[derive(Clone, Debug)]
pub struct Wedge {
    pub key: f64,
    pub text: String,
    pub opens: Option<bool>,
}
impl Wedge {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Wedge {
            key: 0.0,
            text: String::new(),
            opens: None,
        }
    }
}
impl Default for Wedge { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: RingPlace, src/game/ringlone.ts:53)
#[derive(Clone, Debug)]
pub struct RingPlace {
    pub d: String,
    pub nx: f64,
    pub ny: f64,
    pub tx: f64,
    pub ty: f64,
    pub anchor: crate::ringlone::RingPlaceAnchor,
}
impl RingPlace {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RingPlace {
            d: String::new(),
            nx: 0.0,
            ny: 0.0,
            tx: 0.0,
            ty: 0.0,
            anchor: crate::ringlone::RingPlaceAnchor::Start,
        }
    }
}
impl Default for RingPlace { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum RingPlaceAnchor {
    Start,
    End,
    Middle,
}
impl RingPlaceAnchor {
    pub const fn as_str(self) -> &'static str {
        match self {
            RingPlaceAnchor::Start => "start",
            RingPlaceAnchor::End => "end",
            RingPlaceAnchor::Middle => "middle",
        }
    }
}

/// (TypeScript: LonePill, src/game/ringlone.ts:54)
#[derive(Clone, Debug)]
pub struct LonePill {
    pub x0: f64,
    pub y0: f64,
    pub x1: f64,
    pub y1: f64,
    pub text_w: f64,
    pub lines: Vec<crate::ringlone::LonePillLinesItem>,
    pub d: String,
    pub nx: f64,
    pub ny: f64,
    pub tx: f64,
    pub ty: f64,
    pub anchor: crate::ringlone::LonePillAnchor,
}
impl LonePill {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LonePill {
            x0: 0.0,
            y0: 0.0,
            x1: 0.0,
            y1: 0.0,
            text_w: 0.0,
            lines: Vec::new(),
            d: String::new(),
            nx: 0.0,
            ny: 0.0,
            tx: 0.0,
            ty: 0.0,
            anchor: crate::ringlone::LonePillAnchor::Start,
        }
    }
}
impl Default for LonePill { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/ringlone.ts:55 LonePill.lines, src/game/ringlone.ts:55)
#[derive(Clone, Debug)]
pub struct LonePillLinesItem {
    pub s: String,
    pub y: f64,
    pub w: f64,
}
impl LonePillLinesItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LonePillLinesItem {
            s: String::new(),
            y: 0.0,
            w: 0.0,
        }
    }
}
impl Default for LonePillLinesItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum LonePillAnchor {
    Start,
    End,
    Middle,
}
impl LonePillAnchor {
    pub const fn as_str(self) -> &'static str {
        match self {
            LonePillAnchor::Start => "start",
            LonePillAnchor::End => "end",
            LonePillAnchor::Middle => "middle",
        }
    }
}

/// (TypeScript: inline at src/game/ringlone.ts:37 RING_BOX, src/game/ringlone.ts:37)
#[derive(Clone, Copy, Debug)]
pub struct RINGBOX {
    pub x: f64,
    pub y: f64,
}
impl RINGBOX {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RINGBOX {
            x: 0.0,
            y: 0.0,
        }
    }
}
impl Default for RINGBOX { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/ringlone.ts:42 LONE, src/game/ringlone.ts:42)
#[derive(Clone, Copy, Debug)]
pub struct LONE {
    pub r: f64,
    pub h: f64,
    pub pad: f64,
    pub char: f64,
    pub max: f64,
    pub lead: f64,
}
impl LONE {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LONE {
            r: 0.0,
            h: 0.0,
            pad: 0.0,
            char: 0.0,
            max: 0.0,
            lead: 0.0,
        }
    }
}
impl Default for LONE { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/ringlone.ts:58 boxOf ->, src/game/ringlone.ts:58)
#[derive(Clone, Copy, Debug)]
pub struct BoxOf {
    pub x0: f64,
    pub y0: f64,
    pub x1: f64,
    pub y1: f64,
    pub px: f64,
    pub py: f64,
    pub left: bool,
}
impl BoxOf {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BoxOf {
            x0: 0.0,
            y0: 0.0,
            x1: 0.0,
            y1: 0.0,
            px: 0.0,
            py: 0.0,
            left: false,
        }
    }
}
impl Default for BoxOf { fn default() -> Self { Self::zeroed() } }

/// (src/game/ringlone.ts:33 RING_HOLE)
pub const RING_HOLE: f64 = 17.0;

/// (src/game/ringlone.ts:35 RING_RIM)
pub const RING_RIM: f64 = 44.0;

/// (src/game/ringlone.ts:37 RING_BOX: a table nothing writes)
pub fn ring_box() -> crate::ringlone::RINGBOX { crate::ringlone::RINGBOX { x: 190.0, y: 80.0 } }

/// (src/game/ringlone.ts:42 LONE: a table nothing writes)
pub fn lone() -> crate::ringlone::LONE { crate::ringlone::LONE { r: 34.0, h: 22.0, pad: 10.0, char: 6.6, max: 20.0, lead: 12.0 } }
