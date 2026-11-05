//! The types of `gunwall`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum GunUp {
    Port,
    Aim,
}
impl GunUp {
    pub const fn as_str(self) -> &'static str {
        match self {
            GunUp::Port => "port",
            GunUp::Aim => "aim",
        }
    }
}

/// (TypeScript: Hold, src/game/gunwall.ts:52)
#[derive(Clone, Copy, Debug)]
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
impl Hold {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Hold {
            x: 0.0,
            z: 0.0,
            yaw: 0.0,
            pitch: 0.0,
            sh: 0.0,
            a: [0.0; 3],
            b: [0.0; 3],
            hot: false,
            serial: 0,
        }
    }
}
impl Default for Hold { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: St, src/game/gunwall.ts:53)
#[derive(Clone, Copy, Debug)]
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
impl St {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        St {
            low: false,
            t: 0.0,
            mine: false,
            hold: None,
            wood: 0.0,
            rose: false,
            px: 0.0,
            pz: 0.0,
            vt: 0.0,
            vx: 0.0,
            vz: 0.0,
        }
    }
}
impl Default for St { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/gunwall.ts:72 FIGHT, src/game/gunwall.ts:72)
#[derive(Clone, Copy, Debug)]
pub struct GunwallModFIGHTValue {
    pub t: f64,
    pub until: f64,
}
impl GunwallModFIGHTValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GunwallModFIGHTValue {
            t: 0.0,
            until: 0.0,
        }
    }
}
impl Default for GunwallModFIGHTValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/gunwall.ts:135 gunWallModel, src/game/gunwall.ts:135)
#[derive(Clone, Copy, Debug)]
pub struct GunWallModel {
    pub tuck_reach: f64,
}
impl GunWallModel {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GunWallModel {
            tuck_reach: 0.0,
        }
    }
}
impl Default for GunWallModel { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/gunwall.ts:86 frame ->, src/game/gunwall.ts:89)
#[derive(Clone, Copy, Debug)]
pub struct Frame {
    pub f: V3,
    pub r: V3,
    pub sh: f64,
    pub k: f64,
}
impl Frame {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Frame {
            f: [0.0; 3],
            r: [0.0; 3],
            sh: 0.0,
            k: 0.0,
        }
    }
}
impl Default for Frame { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/gunwall.ts:91 at(F), src/game/gunwall.ts:91)
#[derive(Clone, Copy, Debug)]
pub struct AtF {
    pub f: V3,
    pub r: V3,
}
impl AtF {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AtF {
            f: [0.0; 3],
            r: [0.0; 3],
        }
    }
}
impl Default for AtF { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/gunwall.ts:108 raisedLine ->, src/game/gunwall.ts:108)
#[derive(Clone, Copy, Debug)]
pub struct RaisedLine {
    pub a: V3,
    pub b: V3,
}
impl RaisedLine {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RaisedLine {
            a: [0.0; 3],
            b: [0.0; 3],
        }
    }
}
impl Default for RaisedLine { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
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
    pub lowered_s: i32,
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

/// (src/game/gunwall.ts:135 gunWallModel: a table nothing writes)
pub fn gun_wall_model() -> crate::gunwall::GunWallModel { crate::gunwall::GunWallModel { tuck_reach: crate::gunwall::TUCK_REACH } }
