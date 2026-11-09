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
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SettleTimeP {
    pub settle_secs: f64,
    pub range_ref: f64,
    pub near_k: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct GuardTerms {
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

/// (src/game/aim.ts:98 LOS_NOW)
pub const LOS_NOW: f64 = 0.05;

/// (src/game/aim.ts:249 BORE_FOUL_M)
pub const BORE_FOUL_M: f64 = 1.2;

