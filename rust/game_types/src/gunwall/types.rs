//! The types of `gunwall`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum GunUp {
    Port,
    Aim,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct Hold {
    pub x: f64,
    pub z: f64,
    pub yaw: f64,
    pub pitch: f64,
    pub sh: f64,
    pub a: V3,
    pub b: V3,
    pub hot: bool,
    /// INVENTED: dealt when a hold starts; a new hold of the same man counts as new
    pub serial: u32,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct St {
    pub low: bool,
    pub t: f64,
    pub mine: bool,
    pub hold: Option<crate::gunwall::Hold>,
    pub wood: f64,
    pub rose: bool,
    pub px: f64,
    pub pz: f64,
    pub vt: f64,
    pub vx: f64,
    pub vz: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct GunwallModFIGHTValue {
    pub t: f64,
    pub until: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Frame {
    pub f: V3,
    pub r: V3,
    pub sh: f64,
    pub k: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct AtF {
    pub f: V3,
    pub r: V3,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct RaisedLine {
    pub a: V3,
    pub b: V3,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes)]
pub struct GunwallMod {
    /// (src/game/gunwall.ts:56 ST)
    pub st: JsMap<CharId, crate::gunwall::St>,
    /// (src/game/gunwall.ts:72 FIGHT)
    pub fight: Option<crate::gunwall::GunwallModFIGHTValue>,
}
impl GunwallMod {
    pub fn blank() -> Self {
        GunwallMod {
            st: JsMap::new(),
            fight: None,
        }
    }
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct GunwallStats {
    pub asked: i32,
    pub lowered: i32,
    pub lowered_s: f64,
    pub tucked: i32,
}

/// (src/game/gunwall.ts:59 STAND_SH)
pub const STAND_SH: f64 = 1.4;

/// (src/game/gunwall.ts:59 CROUCH_SH)
pub const CROUCH_SH: f64 = 0.99;

/// (src/game/gunwall.ts:60 TUCK_REACH)
pub const TUCK_REACH: f64 = 0.235;

/// (src/game/gunwall.ts:61 SH_REST)
pub const SH_REST: f64 = -0.07;

/// (src/game/gunwall.ts:61 SH_CARRY)
pub const SH_CARRY: f64 = 0.6;

/// (src/game/gunwall.ts:61 REACH_OVER)
pub const REACH_OVER: f64 = 0.2;

/// (src/game/gunwall.ts:62 GUN_HALF)
pub const GUN_HALF: f64 = 0.04;

/// (src/game/gunwall.ts:62 GUN_HALF_LOW)
pub const GUN_HALF_LOW: f64 = 0.06;

/// (src/game/gunwall.ts:63 SETTLED_M)
pub const SETTLED_M: f64 = 0.08;

/// (src/game/gunwall.ts:64 HOLD_M)
pub const HOLD_M: f64 = 0.1;

/// (src/game/gunwall.ts:64 HOLD_RAD)
pub const HOLD_RAD: f64 = 0.15;

/// (src/game/gunwall.ts:65 HOT_SECS)
pub const HOT_SECS: f64 = 0.1;

/// (src/game/gunwall.ts:66 FRESH_S)
pub const FRESH_S: f64 = 0.05;

/// (src/game/gunwall.ts:67 TUCK_HALF)
pub const TUCK_HALF: f64 = 0.07;

/// (src/game/gunwall.ts:68 TELEPORT_M)
pub const TELEPORT_M: f64 = 0.25;

/// (src/game/gunwall.ts:69 BREACH_AIM_M)
pub const BREACH_AIM_M: f64 = 1.4;

/// (src/game/gunwall.ts:70 AT_IN)
pub const AT_IN: f64 = 0.03;

/// (src/game/gunwall.ts:131 CLEAR_WIDE)
pub const CLEAR_WIDE: f64 = 0.1;

