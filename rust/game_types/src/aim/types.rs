//! The types of `aim`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: AimState, src/game/aim.ts:102)
#[derive(Clone, Debug)]
pub struct AimState {
    pub track: f64,
    pub tgt_key: String,
    pub last_bearing: f64,
    pub bearing_rate: f64,
    pub speed_sm: f64,
    pub move_sm: f64,
    pub last_man: String,
    pub left_t: f64,
    pub left_track: f64,
    pub braking: bool,
    pub fouled: Option<crate::aim::AimStateFouled>,
    pub last_shot: Option<crate::aim::AimStateLastShot>,
}
impl AimState {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        AimState {
            track: 0.0,
            tgt_key: String::from(""),
            last_bearing: f64::NAN,
            bearing_rate: 0.0,
            speed_sm: 0.0,
            move_sm: 0.0,
            last_man: String::from(""),
            left_t: -100.0,
            left_track: 0.0,
            braking: false,
            fouled: None,
            last_shot: None,
        }
    }
}
impl Default for AimState { fn default() -> Self { Self::blank() } }

/// (TypeScript: inline at src/game/aim.ts:121 AimState.fouled, src/game/aim.ts:121)
#[derive(Clone, Copy, Debug)]
pub struct AimStateFouled {
    pub n: f64,
    pub pos: V3,
}
impl AimStateFouled {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AimStateFouled {
            n: 0.0,
            pos: [0.0; 3],
        }
    }
}
impl Default for AimStateFouled { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/aim.ts:124 AimState.lastShot, src/game/aim.ts:124)
#[derive(Clone, Copy, Debug)]
pub struct AimStateLastShot {
    pub t: f64,
    pub track: f64,
    pub spread: f64,
    pub speed: f64,
    pub gated: bool,
    pub hands: f64,
    pub settle: f64,
    pub tgt: f64,
    pub own: f64,
}
impl AimStateLastShot {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AimStateLastShot {
            t: 0.0,
            track: 0.0,
            spread: 0.0,
            speed: 0.0,
            gated: false,
            hands: 0.0,
            settle: 0.0,
            tgt: 0.0,
            own: 0.0,
        }
    }
}
impl Default for AimStateLastShot { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/aim.ts:145 AIM_HOOKS, src/game/aim.ts:145)
#[derive(Clone, Copy, Debug)]
pub struct AIMHOOKS {
}
impl AIMHOOKS {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AIMHOOKS {
        }
    }
}
impl Default for AIMHOOKS { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/aim.ts:147 AIM_DEBUG, src/game/aim.ts:147)
#[derive(Clone, Copy, Debug)]
pub struct AIMDEBUG {
}
impl AIMDEBUG {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AIMDEBUG {
        }
    }
}
impl Default for AIMDEBUG { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/aim.ts:326 KEPT, src/game/aim.ts:326)
#[derive(Clone, Debug)]
pub struct KEPT {
    pub guard: Vec<String>,
    pub player: Vec<String>,
}
impl KEPT {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        KEPT {
            guard: Vec::new(),
            player: Vec::new(),
        }
    }
}
impl Default for KEPT { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/aim.ts:199 settleTime(P), src/game/aim.ts:199)
#[derive(Clone, Copy, Debug)]
pub struct SettleTimeP {
    pub settle_secs: f64,
    pub range_ref: f64,
    pub near_k: f64,
}
impl SettleTimeP {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SettleTimeP {
            settle_secs: 0.0,
            range_ref: 0.0,
            near_k: 0.0,
        }
    }
}
impl Default for SettleTimeP { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/aim.ts:212 guardTerms ->, src/game/aim.ts:219)
#[derive(Clone, Copy, Debug)]
pub struct GuardTerms {
    pub hands: f64,
    pub settle: f64,
    pub tgt: f64,
    pub own: f64,
    pub speed: f64,
    pub spread: f64,
}
impl GuardTerms {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GuardTerms {
            hands: 0.0,
            settle: 0.0,
            tgt: 0.0,
            own: 0.0,
            speed: 0.0,
            spread: 0.0,
        }
    }
}
impl Default for GuardTerms { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/aim.ts:306 playerAim ->, src/game/aim.ts:306)
#[derive(Clone, Copy, Debug)]
pub struct PlayerAim {
    pub spread: f64,
    pub track: f64,
    pub base: f64,
    pub settle: f64,
    pub tgt: f64,
    pub own: f64,
}
impl PlayerAim {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PlayerAim {
            spread: 0.0,
            track: 0.0,
            base: 0.0,
            settle: 0.0,
            tgt: 0.0,
            own: 0.0,
        }
    }
}
impl Default for PlayerAim { fn default() -> Self { Self::zeroed() } }

/// (src/game/aim.ts:29 SNAP_PX)
pub const SNAP_PX: f64 = 60.0;

/// (src/game/aim.ts:91 AIM_DEFAULTS: a table nothing writes)
pub fn aim_defaults() -> crate::knobs::AimK { crate::knobs::page_aim() }

/// (src/game/aim.ts:98 LOS_NOW)
pub const LOS_NOW: f64 = 0.05;

/// (src/game/aim.ts:145 AIM_HOOKS: a table nothing writes)
pub fn aim_hooks() -> crate::aim::AIMHOOKS { crate::aim::AIMHOOKS {  } }

/// (src/game/aim.ts:147 AIM_DEBUG: a table nothing writes)
pub fn aim_debug() -> crate::aim::AIMDEBUG { crate::aim::AIMDEBUG {  } }

/// (src/game/aim.ts:249 BORE_FOUL_M)
pub const BORE_FOUL_M: f64 = 1.2;

/// (src/game/aim.ts:325 LS_KEY)
pub const LS_KEY: &str = "twopointsix.aim";

/// (src/game/aim.ts:326 KEPT: a table nothing writes)
pub fn kept() -> crate::aim::KEPT { crate::aim::KEPT { guard: alloc::vec![String::from("settleSecs"), String::from("acquireMin"), String::from("unsettledK"), String::from("targetMoveK"), String::from("shooterMoveK"), String::from("stopToShoot")], player: alloc::vec![String::from("settleSecs"), String::from("unsettledK"), String::from("targetMoveK"), String::from("shooterMoveK"), String::from("lightSnapPx"), String::from("level")] } }
