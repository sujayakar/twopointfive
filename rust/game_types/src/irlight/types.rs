//! The types of `irlight`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Need {
    pub read_t: f64,
    pub dark_t: f64,
    pub k: f64,
    pub t: f64,
    pub fresh: bool,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Kit {
    pub sam: Option<LightId>,
    pub more: JsMap<crate::Num, LightId>,
    pub men: JsMap<GuardId, LightId>,
    pub seen: JsMap<GuardId, crate::irlight::KitSeenValue>,
    pub need: JsMap<crate::Num, crate::irlight::Need>,
    pub near: JsMap<crate::Num, crate::irlight::KitNearValue>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct KitSeenValue {
    pub t: f64,
    pub e: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct KitNearValue {
    pub t: f64,
    pub k: f64,
}

#[derive(Clone, Debug, Default)]
pub struct IrLightState {
    pub sam: bool,
    pub men: Vec<GuardId>,
    pub shown: bool,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes)]
pub struct IrlightMod {
    /// (src/game/irlight.ts:117 kits)
    pub kits: Option<crate::irlight::Kit>,
}
impl IrlightMod {
    pub fn blank() -> Self {
        IrlightMod {
            kits: None,
        }
    }
}

/// (src/game/irlight.ts:104 NEAR_BREAK)
pub const NEAR_BREAK: bool = false;

/// (src/game/irlight.ts:107 WAND_KEY0)
pub const WAND_KEY0: f64 = -1.0;

/// (src/game/irlight.ts:108 HEAD_Y)
pub const HEAD_Y: f64 = 1.62;

/// (src/game/irlight.ts:108 LENS_AHEAD)
pub const LENS_AHEAD: f64 = 0.14;

/// (src/game/irlight.ts:108 LENS_CLEAR)
pub const LENS_CLEAR: f64 = 0.3;

/// (src/game/irlight.ts:108 LENS_R)
pub const LENS_R: f64 = 0.04;

/// (src/game/irlight.ts:137 FAN_NEED)
pub const FAN_NEED: f64 = 2.0;

/// (src/game/irlight.ts:137 READ_EVERY)
pub const READ_EVERY: f64 = 0.25;

/// (src/game/irlight.ts:137 FAN_DEG: a table nothing writes)
pub fn fan_deg() -> Vec<f64> { alloc::vec![0.0, 12.0, -12.0, 24.0, -24.0, 36.0, -36.0, 48.0, -48.0, 60.0, -60.0] }

/// (src/game/irlight.ts:137 FAN_M: a table nothing writes)
pub fn fan_m() -> Vec<f64> { alloc::vec![1.5, 3.0, 4.5, 6.0, 8.0, 10.0, 12.0] }

/// (src/game/irlight.ts:137 LOOK_MAX)
pub const LOOK_MAX: f64 = 12.0;

/// (src/game/irlight.ts:137 FADE_IN)
pub const FADE_IN: f64 = 0.15;

/// (src/game/irlight.ts:137 FADE_OUT)
pub const FADE_OUT: f64 = 0.5;

/// (src/game/irlight.ts:220 NEAR_YAW: a table nothing writes)
pub fn near_yaw() -> Vec<f64> { alloc::vec![0.0, -0.17, 0.17] }
