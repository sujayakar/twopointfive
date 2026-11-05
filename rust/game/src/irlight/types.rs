//! The types of `irlight`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: Need, src/game/irlight.ts:113)
#[derive(Clone, Debug)]
pub struct Need {
    pub read_t: f64,
    pub dark_t: f64,
    pub k: f64,
    pub t: f64,
    pub fresh: bool,
    pub t0: f64,
    pub own: f64,
    pub pts: Vec<f64>,
    pub since: Vec<f64>,
}
impl Need {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Need {
            read_t: 0.0,
            dark_t: 0.0,
            k: 0.0,
            t: 0.0,
            fresh: false,
            t0: 0.0,
            own: 0.0,
            pts: Vec::new(),
            since: Vec::new(),
        }
    }
}
impl Default for Need { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Kit, src/game/irlight.ts:114)
#[derive(Clone, Debug)]
pub struct Kit {
    pub sam: Option<LightId>,
    pub more: JsMap<crate::Num, LightId>,
    pub men: JsMap<GuardId, LightId>,
    pub seen: JsMap<GuardId, crate::irlight::KitSeenValue>,
    pub need: JsMap<crate::Num, crate::irlight::Need>,
    pub near: JsMap<crate::Num, crate::irlight::KitNearValue>,
}
impl Kit {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Kit {
            sam: None,
            more: JsMap::new(),
            men: JsMap::new(),
            seen: JsMap::new(),
            need: JsMap::new(),
            near: JsMap::new(),
        }
    }
}
impl Default for Kit { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/irlight.ts:114 Kit.seen, src/game/irlight.ts:114)
#[derive(Clone, Copy, Debug)]
pub struct KitSeenValue {
    pub t: f64,
    pub e: f64,
}
impl KitSeenValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        KitSeenValue {
            t: 0.0,
            e: 0.0,
        }
    }
}
impl Default for KitSeenValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/irlight.ts:116 Kit.near, src/game/irlight.ts:116)
#[derive(Clone, Copy, Debug)]
pub struct KitNearValue {
    pub t: f64,
    pub k: f64,
}
impl KitNearValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        KitNearValue {
            t: 0.0,
            k: 0.0,
        }
    }
}
impl Default for KitNearValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/irlight.ts:148 IRL_METER, src/game/irlight.ts:148)
#[derive(Clone, Copy, Debug)]
pub struct IRLMETER {
    pub slots: f64,
    pub wait: f64,
}
impl IRLMETER {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        IRLMETER {
            slots: 0.0,
            wait: 0.0,
        }
    }
}
impl Default for IRLMETER { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/irlight.ts:306 irLightState ->, src/game/irlight.ts:306)
#[derive(Clone, Debug)]
pub struct IrLightState {
    pub sam: bool,
    pub men: Vec<GuardId>,
    pub shown: bool,
}
impl IrLightState {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        IrLightState {
            sam: false,
            men: Vec::new(),
            shown: false,
        }
    }
}
impl Default for IrLightState { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
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

/// (src/game/irlight.ts:148 IRL_METER: a table nothing writes)
pub fn irl_meter() -> crate::irlight::IRLMETER { crate::irlight::IRLMETER { slots: 5.0, wait: 0.25 } }

/// (src/game/irlight.ts:220 NEAR_YAW: a table nothing writes)
pub fn near_yaw() -> Vec<f64> { alloc::vec![0.0, -0.17, 0.17] }
