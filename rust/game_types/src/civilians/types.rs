//! The types of `civilians`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// what a civilian has given up on (TypeScript: a Set of objects, compared by identity): a man, or a CivSpot of the level by its place in the level's list
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, ToBytes)]
pub enum CivBadTarget { Guard(GuardId), Spot(u32) }

#[derive(Clone, Debug, Default)]
pub struct CivSnap {
    pub time: f64,
    pub list: Vec<crate::civilians::CivSnapOne>,
    pub floor: crate::civilians::CivSnapFloor,
}

#[derive(Clone, Debug, Default)]
pub struct CivSnapOne {
    pub id: String,
    pub pos: V3,
    pub vel: V3,
    pub yaw: f64,
    pub aim_yaw: f64,
    pub crouch: f64,
    pub anim_phase: f64,
    pub speed: f64,
    pub step_phase: f64,
    pub look_phase: f64,
    pub wp: f64,
    pub wait: f64,
    pub awareness: f64,
    pub heard_up_to: f64,
    pub path: Vec<V3>,
    pub path_i: f64,
    pub path_goal: Option<V3>,
    pub repath_t: f64,
    pub last_pos: V3,
    pub stuck_t: f64,
    pub stuck_n: f64,
    pub stall_ref: Option<V3>,
    pub stall_t: f64,
    pub talk: crate::civilians::CivSnapOneTalk,
    pub bubble: Option<crate::civilians::CivSnapOneBubble>,
    pub down: Option<crate::civilians::CivSnapOneDown>,
    pub state: crate::civilians::CivState,
    pub state_at: f64,
    pub task_t: f64,
    pub rise_t: f64,
    pub settle_t: f64,
    pub seated: bool,
    pub look_at: Option<V3>,
    pub look_until: f64,
    pub shrug_at: f64,
    pub cause: Option<crate::civilians::CivCause>,
    pub cause_pos: Option<V3>,
    pub cause_at: f64,
    pub armed: bool,
    pub alarm_n: f64,
    pub seen_at: f64,
    pub seen_pos: Option<V3>,
    pub saw_gun: bool,
    pub target: Option<crate::civilians::CivSnapOneTarget>,
    pub retarget_at: f64,
    pub bad_targets: Vec<String>,
    pub freeze_until: f64,
    pub pin_target: bool,
    pub report_to: Option<String>,
    pub talk_t: f64,
    pub reported: bool,
    pub phoned: bool,
    pub acked: bool,
    pub cower_at: Option<V3>,
    pub complied: bool,
    pub comply_until: f64,
    pub refled: bool,
    pub found_t: f64,
    pub found_by: Option<String>,
    pub found_line_at: f64,
    pub found_told: bool,
    pub scream_at: f64,
    pub whimper_at: f64,
    pub greet_at: f64,
    pub last_line_at: f64,
    pub told: Vec<(String, f64)>,
    pub known_bodies: Vec<String>,
    pub intel_n: f64,
    pub outside_call_at: f64,
    pub vis: f64,
    pub dice: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct CivSnapOneTalk {
    pub stage: f64,
    pub presses: f64,
    pub last_t: f64,
    pub abort_t: f64,
}

#[derive(Clone, Debug, Default)]
pub struct CivSnapOneBubble {
    pub text: String,
    pub t: f64,
    pub dur: f64,
    pub radio: bool,
}

#[derive(Clone, Debug, Default)]
pub struct CivSnapOneDown {
    pub kind: crate::game::DownKind,
    pub by: crate::game::DownBy,
    pub at: f64,
    pub found: bool,
    pub rag: Option<crate::civilians::CivSnapOneDownRag>,
}

#[derive(Clone, Debug, Default)]
pub struct CivSnapOneDownRag {
    pub x: Vec<f64>,
    pub rot: [f64; 4],
    pub sleeping: bool,
    pub own: Option<bool>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
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
impl CivState {
    pub const fn as_str(self) -> &'static str {
        match self {
            CivState::Held => "held",
            CivState::Work => "work",
            CivState::Startled => "startled",
            CivState::Alarmed => "alarmed",
            CivState::Flee => "flee",
            CivState::Report => "report",
            CivState::Phone => "phone",
            CivState::Cower => "cower",
            CivState::Evacuate => "evacuate",
            CivState::Gone => "gone",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes)]
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
impl CivCause {
    pub const fn as_str(self) -> &'static str {
        match self {
            CivCause::Shot => "shot",
            CivCause::Bang => "bang",
            CivCause::Body => "body",
            CivCause::Sam => "sam",
            CivCause::Gun => "gun",
            CivCause::Gunfire => "gunfire",
            CivCause::Scream => "scream",
            CivCause::Noise => "noise",
            CivCause::Grabbed => "grabbed",
            CivCause::Panel => "panel",
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct CivSnapOneTarget {
    pub kind: crate::civilians::CivSnapOneTargetKind,
    pub guard: Option<String>,
    pub spot: Option<String>,
    pub pos: V3,
    pub plan_pos: V3,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum CivSnapOneTargetKind {
    #[default] Guard,
    Phone,
    Exit,
}
impl CivSnapOneTargetKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            CivSnapOneTargetKind::Guard => "guard",
            CivSnapOneTargetKind::Phone => "phone",
            CivSnapOneTargetKind::Exit => "exit",
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct CivSnapFloor {
    pub evac_told: bool,
    pub greet_at: f64,
    pub pending: Vec<crate::civilians::CivPendingSnap>,
}

#[derive(Clone, Debug, Default)]
pub struct CivPendingSnap {
    pub at: f64,
    pub kind: crate::civilians::PendKind,
    pub cv: String,
    pub o: Option<String>,
    pub key: Option<String>,
    pub armed: Option<bool>,
    pub pos: Option<V3>,
    pub lvl: Option<f64>,
    pub dead: Option<bool>,
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
impl PendKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            PendKind::GreetReply => "greetReply",
            PendKind::ReportRadio => "reportRadio",
            PendKind::Escalate => "escalate",
            PendKind::OutsideCall => "outsideCall",
            PendKind::ComplyLine => "complyLine",
            PendKind::FoundReport => "foundReport",
            PendKind::BodyEsc => "bodyEsc",
            PendKind::Outrage => "outrage",
        }
    }
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
    pub name: String,
    pub plan_pos: V3,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum CivTargetKind {
    #[default] Guard,
    Phone,
    Exit,
}
impl CivTargetKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            CivTargetKind::Guard => "guard",
            CivTargetKind::Phone => "phone",
            CivTargetKind::Exit => "exit",
        }
    }
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
    pub task_t: f64,
    pub rise_t: f64,
    pub seated: bool,
    pub settle_t: f64,
    pub look_at: Option<V3>,
    pub look_until: f64,
    pub shrug_at: f64,
    pub cause: Option<crate::civilians::CivCause>,
    pub cause_pos: Option<V3>,
    pub cause_at: f64,
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
    pub last_line_at: f64,
    pub told: JsMap<GuardId, f64>,
    pub known_bodies: JsSet<GuardId>,
    pub intel_n: usize,
    pub outside_call_at: f64,
    pub vis: f64,
    pub dice: crate::dice::Die,
    pub log: Vec<String>,
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

#[derive(Clone, Debug, Default)]
pub struct CIVSPOTS {
    pub phones: Vec<crate::civilians::CivSpot>,
    pub exits: Vec<crate::civilians::CivSpot>,
}

#[derive(Clone, Debug, Default)]
pub struct CiviliansModOwnSpotsValue {
    pub phones: Vec<crate::civilians::CivSpot>,
    pub exits: Vec<crate::civilians::CivSpot>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct CiviliansModCIVLOGItem {
    pub t: f64,
    pub who: String,
    pub what: String,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct CIVDEBUG {
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
    pub kind: crate::civilians::SpawnHostageDefKind,
    pub tint: V3,
    pub tint2: Option<V3>,
    pub pos: V3,
    pub face: f64,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum SpawnHostageDefKind {
    #[default] Cleaner,
    Worker,
}
impl SpawnHostageDefKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            SpawnHostageDefKind::Cleaner => "cleaner",
            SpawnHostageDefKind::Worker => "worker",
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct SpawnTargetDef {
    pub name: String,
    pub pos: V3,
    pub face: f64,
    pub tint: V3,
    pub tint2: Option<V3>,
    pub points: Option<Vec<V3>>,
    pub wait: Option<Vec<f64>>,
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

#[derive(Clone, Debug)]
pub enum InViewVp {
    Array(Vec<f64>),
    Float32Array(Vec<f32>),
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct Work {
    pub move_: f64,
}

#[derive(Clone, Debug, Default)]
pub struct PathMetres {
    pub m: f64,
    pub path: Option<Vec<V3>>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct PickTargetOpts {
    pub exits_only: Option<bool>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum DeliverReportHow {
    Found,
    Told,
}
impl DeliverReportHow {
    pub const fn as_str(self) -> &'static str {
        match self {
            DeliverReportHow::Found => "found",
            DeliverReportHow::Told => "told",
        }
    }
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
    pub id: String,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum CivPanicWhat {
    Phone,
    Panic,
}
impl CivPanicWhat {
    pub const fn as_str(self) -> &'static str {
        match self {
            CivPanicWhat::Phone => "phone",
            CivPanicWhat::Panic => "panic",
        }
    }
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes)]
pub struct CiviliansMod {
    /// (src/game/civilians.ts:78 ownSpots)
    pub own_spots: Option<crate::civilians::CivSpots>,
    /// (src/game/civilians.ts:181 CIV_LOG)
    pub civ_log: Vec<crate::civilians::CiviliansModCIVLOGItem>,
    /// (src/game/civilians.ts:443 floors)
    pub floors: Option<crate::civilians::CiviliansModFloorsValue>,
}
impl CiviliansMod {
    pub fn blank() -> Self {
        CiviliansMod {
            own_spots: None,
            civ_log: Vec::new(),
            floors: None,
        }
    }
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct CiviliansStats {
    pub spawned: i32,
    pub startled: i32,
    pub alarmed: i32,
    pub fled: i32,
    pub reports: i32,
    pub phoned: i32,
    pub outside_calls: i32,
    pub cowered: i32,
    pub complied: i32,
    pub ko: i32,
    pub killed: i32,
    pub found: i32,
    pub found_down: i32,
    pub evacuated: i32,
    pub greetings: i32,
    pub refled: i32,
    pub noticed: i32,
    pub sam_seen: i32,
}

/// (src/game/civilians.ts:57 CIV_SPOTS: a table nothing writes)
pub fn civ_spots_table() -> crate::civilians::CivSpots { crate::civilians::CivSpots { phones: alloc::vec![crate::civilians::CivSpot { name: String::from("the reception phone"), pos: [8.9, 0.0, 13.8], face: 0.0 }, crate::civilians::CivSpot { name: String::from("the break-room phone"), pos: [34.6, 0.0, 16.2], face: 1.5707963267948966 }], exits: alloc::vec![crate::civilians::CivSpot { name: String::from("the fire exit"), pos: [38.2, 0.0, 23.2], face: 0.0 }, crate::civilians::CivSpot { name: String::from("the outside door"), pos: [34.0, 0.0, 25.0], face: 1.5707963267948966 }] } }

/// (src/game/civilians.ts:122 CIV_ID0)
pub const CIV_ID0: f64 = 40.0;

/// (src/game/civilians.ts:182 CIV_DEBUG: a table nothing writes)
pub fn civ_debug() -> crate::civilians::CIVDEBUG { crate::civilians::CIVDEBUG {  } }

/// (src/game/civilians.ts:435 NONE: a table nothing writes)
pub fn none() -> Vec<GuardId> { Vec::new() }

/// (src/game/civilians.ts:667 SIT_ADVANCE)
pub const SIT_ADVANCE: f64 = 0.33;

/// (src/game/civilians.ts:676 TASK_CLIP: a table nothing writes)
pub fn task_clip() -> JsMap<String, Option<crate::civilians::TASKCLIPValue>> { { let mut t = JsMap::new(); t.set(String::from("stand"), None); t.set(String::from("wipe"), Some(crate::civilians::TASKCLIPValue { clip: String::from("Interact"), loop_: Some(true), rate: Some(0.85), in_: Some(0.4), out: Some(0.4), rise: None })); t.set(String::from("bin"), Some(crate::civilians::TASKCLIPValue { clip: String::from("Fixing_Kneeling"), loop_: Some(true), rate: Some(0.8), in_: Some(0.6), out: Some(0.7), rise: Some(0.8) })); t.set(String::from("sit"), Some(crate::civilians::TASKCLIPValue { clip: String::from("Sitting_Idle_Loop"), loop_: Some(true), rate: Some(1.0), in_: Some(0.7), out: Some(0.8), rise: Some(1.0) })); t.set(String::from("cooler"), Some(crate::civilians::TASKCLIPValue { clip: String::from("Interact"), loop_: Some(true), rate: Some(0.6), in_: Some(0.4), out: Some(0.4), rise: None })); t.set(String::from("printer"), Some(crate::civilians::TASKCLIPValue { clip: String::from("Interact"), loop_: Some(true), rate: Some(0.7), in_: Some(0.4), out: Some(0.4), rise: None })); t.set(String::from("shelf"), Some(crate::civilians::TASKCLIPValue { clip: String::from("PickUp_Table"), loop_: Some(true), rate: Some(0.7), in_: Some(0.4), out: Some(0.5), rise: None })); t } }

/// (src/game/civilians.ts:1072 DIGIT_WORDS: a table nothing writes)
pub fn digit_words() -> Vec<String> { alloc::vec![String::from("oh"), String::from("one"), String::from("two"), String::from("three"), String::from("four"), String::from("five"), String::from("six"), String::from("seven"), String::from("eight"), String::from("nine")] }

/// (src/game/civilians.ts:1252 LS_KEY)
pub const LS_KEY: &str = "twopointsix.civilians";
