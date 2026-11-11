//! The types of `civilians`.
#![allow(unused_imports)]
use crate::{Blank, Words, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// what a civilian has given up on: a man, or a CivSpot of the level by its place in the level's list
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, ToBytes)]
pub enum CivBadTarget { Guard(GuardId), Spot(u32) }

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default, Words)]
pub enum CivState {
    #[default] Held,
    Work,
    Startled,
    Alarmed,
    Flee,
    Report,
    Phone,
    Cower,
    Evacuate,
    Gone,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Words)]
pub enum CivCause {
    Shot,
    Bang,
    Body,
    Sam,
    Gun,
    Gunfire,
    Scream,
    Noise,
    Grabbed,
    Panel,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum PendKind {
    #[default] GreetReply,
    ReportRadio,
    Escalate,
    OutsideCall,
    ComplyLine,
    FoundReport,
    BodyEsc,
    Outrage,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct CivSpot {
    pub name: String,
    pub pos: V3,
    pub face: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct CivTarget {
    pub kind: crate::civilians::CivTargetKind,
    pub guard: Option<GuardId>,
    pub spot: Option<crate::civilians::CivSpot>,
    pub pos: V3,
    pub plan_pos: V3,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum CivTargetKind {
    #[default] Guard,
    Phone,
    Exit,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Civ {
    pub def: crate::scene_level::CivilianDef,
    pub name: String,
    pub label: String,
    pub she: String,
    pub her: String,
    pub state: crate::civilians::CivState,
    pub state_at: f64,
    pub rise_t: f64,
    pub seated: bool,
    pub settle_t: f64,
    pub look_at: Option<V3>,
    pub look_until: f64,
    pub shrug_at: f64,
    pub cause: Option<crate::civilians::CivCause>,
    pub cause_pos: Option<V3>,
    pub armed: bool,
    pub alarm_n: f64,
    pub seen_at: f64,
    pub seen_pos: Option<V3>,
    pub saw_gun: bool,
    pub target: Option<crate::civilians::CivTarget>,
    pub retarget_at: f64,
    pub bad_targets: JsSet<crate::civilians::CivBadTarget>,
    pub freeze_until: f64,
    pub pin_target: bool,
    pub report_to: Option<GuardId>,
    pub talk_t: f64,
    pub reported: bool,
    pub phoned: bool,
    pub acked: bool,
    pub cower_at: Option<V3>,
    pub complied: bool,
    pub comply_until: f64,
    pub refled: bool,
    pub found_t: f64,
    pub found_by: Option<GuardId>,
    pub found_line_at: f64,
    pub found_told: bool,
    pub lead: Option<CharId>,
    pub lead_gap: f64,
    pub scream_at: f64,
    pub whimper_at: f64,
    pub greet_at: f64,
    pub told: JsMap<GuardId, f64>,
    pub known_bodies: JsSet<GuardId>,
    pub intel_n: usize,
    pub vis: f64,
    pub dice: crate::dice::Die,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Pending {
    pub at: f64,
    pub kind: crate::civilians::PendKind,
    pub cv: GuardId,
    pub o: Option<GuardId>,
    pub key: Option<String>,
    pub armed: Option<bool>,
    pub pos: Option<V3>,
    pub lvl: Option<f64>,
    pub dead: Option<bool>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct CiviliansModFloorsValue {
    pub evac_told: bool,
    pub greet_at: f64,
    pub pending: Vec<crate::civilians::Pending>,
}

#[derive(Clone, Debug, Default)]
pub struct TASKCLIPValue {
    pub clip: String,
    pub loop_: Option<bool>,
    pub rate: Option<f64>,
    pub in_: Option<f64>,
    pub out: Option<f64>,
    pub rise: Option<f64>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct CivSpots {
    pub phones: Vec<crate::civilians::CivSpot>,
    pub exits: Vec<crate::civilians::CivSpot>,
}

#[derive(Clone, Debug, Default)]
pub struct SpawnHostageDef {
    pub name: String,
    pub kind: crate::scene_level::CivilianDefKind,
    pub tint: V3,
    pub tint2: Option<V3>,
    pub pos: V3,
    pub face: f64,
}

#[derive(Clone, Debug, Default)]
pub struct LaterP {
    pub dead: Option<bool>,
    pub kind: crate::civilians::PendKind,
    pub key: Option<String>,
    pub pos: Option<V3>,
    pub armed: Option<bool>,
    pub lvl: Option<f64>,
    pub o: Option<GuardId>,
    pub cv: GuardId,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct Work {
    pub move_: f64,
}

#[derive(Clone, Debug, Default)]
pub struct PathMetres {
    pub m: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct PickTargetOpts {
    pub exits_only: Option<bool>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Words)]
pub enum DeliverReportHow {
    Found,
    Told,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Follow {
    pub move_: f64,
    pub crouch: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct Cower {
    pub move_: f64,
    pub crouch: f64,
}

#[derive(Clone, Debug, Default)]
pub struct CivilianSamLine {
    pub text: String,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum CivPanicWhat {
    Phone,
    Panic,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct CiviliansMod {
    pub own_spots: Option<crate::civilians::CivSpots>,
    pub floors: Option<crate::civilians::CiviliansModFloorsValue>,
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct CiviliansStats {
    pub phoned: i32,
    pub found: i32,
}

pub fn civ_spots_table() -> crate::civilians::CivSpots { crate::civilians::CivSpots { phones: alloc::vec![crate::civilians::CivSpot { name: String::from("the reception phone"), pos: [8.9, 0.0, 13.8], face: 0.0 }, crate::civilians::CivSpot { name: String::from("the break-room phone"), pos: [34.6, 0.0, 16.2], face: 1.5707963267948966 }], exits: alloc::vec![crate::civilians::CivSpot { name: String::from("the fire exit"), pos: [38.2, 0.0, 23.2], face: 0.0 }, crate::civilians::CivSpot { name: String::from("the outside door"), pos: [34.0, 0.0, 25.0], face: 1.5707963267948966 }] } }

pub const CIV_ID0: f64 = 40.0;

pub fn none() -> Vec<GuardId> { Vec::new() }

pub const SIT_ADVANCE: f64 = 0.33;

pub fn task_clip() -> JsMap<String, Option<crate::civilians::TASKCLIPValue>> { { let mut t = JsMap::new(); t.set(String::from("stand"), None); t.set(String::from("wipe"), Some(crate::civilians::TASKCLIPValue { clip: String::from("Interact"), loop_: Some(true), rate: Some(0.85), in_: Some(0.4), out: Some(0.4), rise: None })); t.set(String::from("bin"), Some(crate::civilians::TASKCLIPValue { clip: String::from("Fixing_Kneeling"), loop_: Some(true), rate: Some(0.8), in_: Some(0.6), out: Some(0.7), rise: Some(0.8) })); t.set(String::from("sit"), Some(crate::civilians::TASKCLIPValue { clip: String::from("Sitting_Idle_Loop"), loop_: Some(true), rate: Some(1.0), in_: Some(0.7), out: Some(0.8), rise: Some(1.0) })); t.set(String::from("cooler"), Some(crate::civilians::TASKCLIPValue { clip: String::from("Interact"), loop_: Some(true), rate: Some(0.6), in_: Some(0.4), out: Some(0.4), rise: None })); t.set(String::from("printer"), Some(crate::civilians::TASKCLIPValue { clip: String::from("Interact"), loop_: Some(true), rate: Some(0.7), in_: Some(0.4), out: Some(0.4), rise: None })); t.set(String::from("shelf"), Some(crate::civilians::TASKCLIPValue { clip: String::from("PickUp_Table"), loop_: Some(true), rate: Some(0.7), in_: Some(0.4), out: Some(0.5), rise: None })); t } }

pub fn digit_words() -> Vec<String> { alloc::vec![String::from("oh"), String::from("one"), String::from("two"), String::from("three"), String::from("four"), String::from("five"), String::from("six"), String::from("seven"), String::from("eight"), String::from("nine")] }

