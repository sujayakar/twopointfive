//! The types of `quiet`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, Default)]
pub struct QuietSnap {
    pub deal: crate::quiet::QuietDeal,
    pub pin: Option<f64>,
    pub squad: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct QuietDeal {
    pub start: f64,
    pub spot: f64,
    pub patrol: f64,
}

#[derive(Clone, Debug, ToBytes)]
pub struct QuietMission {
    pub active: bool,
    pub ops: Vec<GuardId>,
    pub squad: f64,
    pub pin: Option<f64>,
    pub deal: crate::quiet::QuietDeal,
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
impl QuietMission {
    /// what a new one holds
    pub fn blank() -> Self {
        QuietMission {
            active: false,
            ops: Vec::new(),
            squad: 0.0,
            pin: None,
            deal: crate::quiet::QuietDeal { start: 0.0, spot: 0.0, patrol: -1.0 },
            staged: 0.0,
            log: Vec::new(),
            data: None,
            authored: None,
            benched: Vec::new(),
            was: None,
            mine: None,
            pending: None,
            fall_in: false,
            cut: false,
            failure: None,
        }
    }
}
impl Default for QuietMission { fn default() -> Self { Self::blank() } }

#[derive(Clone, Debug, ToBytes, Default)]
pub struct QuietMissionAuthored {
    pub spawn: V3,
    pub mission: crate::quiet::QuietMissionAuthoredMission,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct QuietMissionAuthoredMission {
    pub server_room: crate::quiet::QuietMissionAuthoredMissionServerRoom,
    pub entry_door: String,
    pub rack: crate::quiet::QuietMissionAuthoredMissionRack,
    pub exfil_door: String,
    pub exfil_x: f64,
    pub exfil_z: f64,
    pub exfil_out: Option<[f64; 2]>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct QuietMissionAuthoredMissionServerRoom {
    pub x0: f64,
    pub x1: f64,
    pub z0: f64,
    pub z1: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct QuietMissionAuthoredMissionRack {
    pub index: f64,
    pub front: V3,
    pub half_w: f64,
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
    pub ok: bool,
    pub clear: bool,
    pub inside: bool,
}

#[derive(Clone, Debug, Default)]
pub struct QuietWords {
    pub place: String,
    pub holder: String,
    pub thing: String,
    pub outs: String,
    pub bare: String,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes)]
pub struct QuietMod {
    /// (src/game/quiet.ts:60 holder)
    pub holder: bool,
}
impl QuietMod {
    pub fn blank() -> Self {
        QuietMod {
            holder: false,
        }
    }
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct QuietStats {
    pub starts: i32,
    pub deals: i32,
    pub mates: i32,
}

/// (src/game/quiet.ts:53 EXIT_HALF_W)
pub const EXIT_HALF_W: f64 = 1.8;

/// (src/game/quiet.ts:55 EXIT_DEPTH)
pub const EXIT_DEPTH: f64 = 12.0;

/// (src/game/quiet.ts:362 FIRST_SIGHT_M)
pub const FIRST_SIGHT_M: f64 = 16.0;
