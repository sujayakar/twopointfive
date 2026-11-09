//! The types of `savestate`.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, Default)]
pub struct SlotInfo {
    pub key: String,
    pub slot: crate::savestate::SlotInfoSlot,
    pub label: String,
    pub saved_at: f64,
    pub mission_t: f64,
    pub run_id: String,
    pub regret: Option<String>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum SlotInfoSlot {
    #[default] Auto,
    Manual,
}

#[derive(Clone, Debug, Default)]
pub struct MemSave {
    pub info: crate::savestate::SlotInfo,
}

#[derive(Clone, Debug)]
pub struct MemShelf {
    pub m: JsMap<String, crate::savestate::MemSave>,
}
impl MemShelf {
    /// what a new one holds
    pub fn blank() -> Self {
        MemShelf {
            m: JsMap::new(),
        }
    }
}
impl Default for MemShelf { fn default() -> Self { Self::blank() } }

#[derive(Clone, Debug)]
pub struct Checkpoints {
    pub enabled: bool,
    pub run_id: String,
    pub visited: JsSet<String>,
    pub quiet_t: f64,
    pub body_from: Option<V3>,
    pub last_save_t: f64,
    pub pending: Option<crate::savestate::CheckpointsPending>,
    pub flash: Option<crate::savestate::CheckpointsFlash>,
    pub log: Vec<crate::savestate::CheckpointsLogItem>,
    pub stats: crate::savestate::CheckpointsStats,
    pub mission_ref: Option<u32>,
    pub stage_ref: crate::game::MissionStage,
    pub room_ref: String,
    pub drag_ref: Option<GuardId>,
    pub last_auto: Option<crate::savestate::CheckpointsLastAuto>,
    pub shelf: crate::savestate::MemShelf,
    pub hand: bool,
    pub due: Option<String>,
    pub authored: Option<String>,
    /// a step asked for the mission over, as a load in a step asks
    pub again: bool,
    /// which stand this game is of, under which the control holds what it keeps (0: none yet)
    pub stood: u32,
}
impl Checkpoints {
    /// what a new one holds
    pub fn blank() -> Self {
        Checkpoints {
            enabled: true,
            run_id: String::from(""),
            visited: JsSet::new(),
            quiet_t: 0.0,
            body_from: None,
            last_save_t: -1000000000.0,
            pending: None,
            flash: None,
            log: Vec::new(),
            stats: crate::savestate::CheckpointsStats { requests: 0.0, autosaves: 0.0, manual: 0.0, loads: 0.0, dropped: 0.0, refused: 0.0, deferred_frames: 0.0, regretted: 0.0 },
            mission_ref: None,
            stage_ref: crate::game::MissionStage::Infiltrate,
            room_ref: String::from(""),
            drag_ref: None,
            last_auto: None,
            shelf: crate::savestate::MemShelf::blank(),
            hand: false,
            due: None,
            authored: None,
            again: false,
            stood: 0,
        }
    }
}
impl Default for Checkpoints { fn default() -> Self { Self::blank() } }

#[derive(Clone, Debug, Default)]
pub struct CheckpointsPending {
    pub why: String,
    pub label: String,
    pub at: f64,
}

#[derive(Clone, Debug, Default)]
pub struct CheckpointsFlash {
    pub text: String,
    pub t: f64,
}

#[derive(Clone, Debug, Default)]
pub struct CheckpointsLogItem {
}

#[derive(Clone, Copy, Debug, Default)]
pub struct CheckpointsStats {
    pub requests: f64,
    pub autosaves: f64,
    pub manual: f64,
    pub loads: f64,
    pub dropped: f64,
    pub refused: f64,
    pub deferred_frames: f64,
    pub regretted: f64,
}

#[derive(Clone, Debug, Default)]
pub struct CheckpointsLastAuto {
    pub key: String,
    pub t: f64,
}

