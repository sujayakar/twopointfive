//! The types of `opening`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, ToBytes, Default)]
pub struct OpeningModSpotsValue {
    pub key: String,
    pub at: Option<crate::opening::OpeningModSpotsValueAt>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct OpeningModSpotsValueAt {
    pub pos: V3,
    pub yaw: f64,
    pub wp: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct OpeningSpot {
    pub pos: V3,
    pub yaw: f64,
    pub wp: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct FindSpot {
    pub pos: V3,
    pub yaw: f64,
    pub wp: f64,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes)]
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
