//! The types of `aim`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, ToBytes)]
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
    /// what a new one holds
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

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct AimStateFouled {
    pub n: f64,
    pub pos: V3,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
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

#[derive(Clone, Copy, Debug, Default)]
pub struct AIMHOOKS {
}

#[derive(Clone, Copy, Debug, Default)]
pub struct AIMDEBUG {
}

#[derive(Clone, Debug, Default)]
pub struct KEPT {
    pub guard: Vec<String>,
    pub player: Vec<String>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SettleTimeP {
    pub settle_secs: f64,
    pub range_ref: f64,
    pub near_k: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct GuardTerms {
    pub hands: f64,
    pub settle: f64,
    pub tgt: f64,
    pub own: f64,
    pub speed: f64,
    pub spread: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct PlayerAim {
    pub spread: f64,
    pub track: f64,
    pub base: f64,
    pub settle: f64,
    pub tgt: f64,
    pub own: f64,
}

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
