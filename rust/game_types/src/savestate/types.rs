//! The types of `savestate`.
#![allow(unused_imports)]
use crate::{Blank, Words, bytes::ToBytes, crosses::Crosses, ids::*, js::*, Quat, V3};
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

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default, ToBytes, Words, Crosses)]
pub enum SlotInfoSlot {
    #[default] Auto,
    Manual,
}

#[derive(Clone, Debug, Default)]
pub struct MemSave {
    pub info: crate::savestate::SlotInfo,
}

#[derive(Clone, Debug, Blank)]
pub struct MemShelf {
    pub m: JsMap<String, crate::savestate::MemSave>,
}

#[derive(Clone, Debug, Blank)]
pub struct Checkpoints {
    #[is(true)] pub enabled: bool,
    #[is(String::from(""))] pub run_id: String,
    pub visited: JsSet<String>,
    pub quiet_t: f64,
    pub body_from: Option<V3>,
    #[is(-1000000000.0)] pub last_save_t: f64,
    pub pending: Option<crate::savestate::CheckpointsPending>,
    pub flash: Option<crate::savestate::CheckpointsFlash>,
    #[is(crate::savestate::CheckpointsStats { autosaves: 0.0, manual: 0.0, loads: 0.0, refused: 0.0 })] pub stats: crate::savestate::CheckpointsStats,
    pub mission_ref: Option<u32>,
    #[is(crate::game::MissionStage::Infiltrate)] pub stage_ref: crate::game::MissionStage,
    #[is(String::from(""))] pub room_ref: String,
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

#[derive(Clone, Copy, Debug, Default)]
pub struct CheckpointsStats {
    pub autosaves: f64,
    pub manual: f64,
    pub loads: f64,
    pub refused: f64,
}

#[derive(Clone, Debug, Default)]
pub struct CheckpointsLastAuto {
    pub key: String,
    pub t: f64,
}

