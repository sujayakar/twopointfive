//! The types of `quiet`.
#![allow(unused_imports)]
use crate::{Blank, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct QuietDeal {
    pub start: f64,
    pub spot: f64,
    pub patrol: f64,
}

#[derive(Clone, Debug, ToBytes, Blank)]
pub struct QuietMission {
    pub squad: f64,
    pub pin: Option<f64>,
    #[is(crate::quiet::QuietDeal { start: 0.0, spot: 0.0, patrol: -1.0 })] pub deal: crate::quiet::QuietDeal,
    pub staged: f64,
    pub log: Vec<String>,
    pub data: Option<crate::scene_scenarios::QuietData>,
    pub authored: Option<crate::quiet::QuietMissionAuthored>,
    pub benched: Vec<GuardId>,
    pub was: Option<crate::quiet::Held>,
    pub mine: Option<crate::quiet::Held>,
    pub pending: Option<crate::quiet::QuietDeal>,
    pub fall_in: bool,
    pub cut: bool,
    pub failure: Option<String>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct QuietMissionAuthored {
    pub spawn: V3,
    pub errand: crate::mission::Errand,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct Held {
    pub pairup: bool,
    pub pair: bool,
    pub sneak: bool,
    pub spotted: bool,
    pub silent: bool,
    pub silent_start: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct HolderStandArea {
    pub x0: f64,
    pub x1: f64,
    pub z0: f64,
    pub z1: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct HolderStand {
    pub pos: Option<V3>,
    pub clear: bool,
    pub inside: bool,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct QuietMod {
    pub holder: bool,
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct QuietStats {
    pub starts: i32,
    pub deals: i32,
    pub mates: i32,
}

pub const EXIT_HALF_W: f64 = 1.8;

pub const EXIT_DEPTH: f64 = 12.0;

pub const FIRST_SIGHT_M: f64 = 16.0;
