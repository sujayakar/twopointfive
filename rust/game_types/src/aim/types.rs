//! The types of `aim`.
#![allow(unused_imports)]
use crate::{Blank, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// A MAN BEHIND A GUN, whoever he is
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct Arms {
    pub aim: crate::aim::AimState,
    /// seconds until he may fire again
    pub fire_cd: f64,
}

#[derive(Clone, Debug, ToBytes, Blank)]
pub struct AimState {
    pub track: f64,
    #[is(String::from(""))] pub tgt_key: String,
    #[is(f64::NAN)] pub last_bearing: f64,
    pub bearing_rate: f64,
    pub speed_sm: f64,
    pub move_sm: f64,
    #[is(String::from(""))] pub last_man: String,
    #[is(-100.0)] pub left_t: f64,
    pub left_track: f64,
    pub braking: bool,
    pub fouled: Option<crate::aim::AimStateFouled>,
    pub last_shot: Option<crate::aim::AimStateLastShot>,
}

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

pub const SNAP_PX: f64 = 60.0;

pub const LOS_NOW: f64 = 0.05;

pub const BORE_FOUL_M: f64 = 1.2;

