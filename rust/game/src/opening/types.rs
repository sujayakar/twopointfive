//! The types of `opening`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: inline at src/game/opening.ts:42 spots, src/game/opening.ts:42)
#[derive(Clone, Debug)]
pub struct OpeningModSpotsValue {
    pub key: String,
    pub at: Option<crate::opening::OpeningModSpotsValueAt>,
}
impl OpeningModSpotsValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        OpeningModSpotsValue {
            key: String::new(),
            at: None,
        }
    }
}
impl Default for OpeningModSpotsValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/opening.ts:42 OpeningModSpotsValue.at, src/game/opening.ts:42)
#[derive(Clone, Copy, Debug)]
pub struct OpeningModSpotsValueAt {
    pub pos: V3,
    pub yaw: f64,
    pub wp: f64,
}
impl OpeningModSpotsValueAt {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        OpeningModSpotsValueAt {
            pos: [0.0; 3],
            yaw: 0.0,
            wp: 0.0,
        }
    }
}
impl Default for OpeningModSpotsValueAt { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/opening.ts:52 openingSpot ->, src/game/opening.ts:52)
#[derive(Clone, Copy, Debug)]
pub struct OpeningSpot {
    pub pos: V3,
    pub yaw: f64,
    pub wp: f64,
}
impl OpeningSpot {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        OpeningSpot {
            pos: [0.0; 3],
            yaw: 0.0,
            wp: 0.0,
        }
    }
}
impl Default for OpeningSpot { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/opening.ts:57 findSpot ->, src/game/opening.ts:57)
#[derive(Clone, Copy, Debug)]
pub struct FindSpot {
    pub pos: V3,
    pub yaw: f64,
    pub wp: f64,
}
impl FindSpot {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FindSpot {
            pos: [0.0; 3],
            yaw: 0.0,
            wp: 0.0,
        }
    }
}
impl Default for FindSpot { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct OpeningMod {
    /// (src/game/opening.ts:30 urlRead)
    pub url_read: bool,
    /// (src/game/opening.ts:38 seen)
    pub seen: JsSet<GuardId>,
    /// (src/game/opening.ts:40 born)
    pub born: JsMap<GuardId, f64>,
    /// (src/game/opening.ts:42 spots)
    pub spots: JsMap<RouteId, crate::opening::OpeningModSpotsValue>,
}
impl OpeningMod {
    pub fn blank() -> Self {
        OpeningMod {
            url_read: false,
            seen: JsSet::new(),
            born: JsMap::new(),
            spots: JsMap::new(),
        }
    }
}

/// (src/game/opening.ts:40 FRESH_SECS)
pub const FRESH_SECS: f64 = 0.25;
