//! The types of `civilians`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// what a civilian has given up on (TypeScript: a Set of objects, compared by identity): a man, or a CivSpot of the level by its place in the level's list
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum CivBadTarget { Guard(GuardId), Spot(u32) }

/// (TypeScript: CivSnap, src/game/civilians.ts:235)
#[derive(Clone, Debug)]
pub struct CivSnap {
    pub time: f64,
    pub list: Vec<crate::civilians::CivSnapOne>,
    pub floor: crate::civilians::CivSnapFloor,
}
impl CivSnap {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CivSnap {
            time: 0.0,
            list: Vec::new(),
            floor: crate::civilians::CivSnapFloor::zeroed(),
        }
    }
}
impl Default for CivSnap { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: CivSnapOne, src/game/civilians.ts:219)
#[derive(Clone, Debug)]
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
impl CivSnapOne {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CivSnapOne {
            id: String::new(),
            pos: [0.0; 3],
            vel: [0.0; 3],
            yaw: 0.0,
            aim_yaw: 0.0,
            crouch: 0.0,
            anim_phase: 0.0,
            speed: 0.0,
            step_phase: 0.0,
            look_phase: 0.0,
            wp: 0.0,
            wait: 0.0,
            awareness: 0.0,
            heard_up_to: 0.0,
            path: Vec::new(),
            path_i: 0.0,
            path_goal: None,
            repath_t: 0.0,
            last_pos: [0.0; 3],
            stuck_t: 0.0,
            stuck_n: 0.0,
            stall_ref: None,
            stall_t: 0.0,
            talk: crate::civilians::CivSnapOneTalk::zeroed(),
            bubble: None,
            down: None,
            state: crate::civilians::CivState::Held,
            state_at: 0.0,
            task_t: 0.0,
            rise_t: 0.0,
            settle_t: 0.0,
            seated: false,
            look_at: None,
            look_until: 0.0,
            shrug_at: 0.0,
            cause: None,
            cause_pos: None,
            cause_at: 0.0,
            armed: false,
            alarm_n: 0.0,
            seen_at: 0.0,
            seen_pos: None,
            saw_gun: false,
            target: None,
            retarget_at: 0.0,
            bad_targets: Vec::new(),
            freeze_until: 0.0,
            pin_target: false,
            report_to: None,
            talk_t: 0.0,
            reported: false,
            phoned: false,
            acked: false,
            cower_at: None,
            complied: false,
            comply_until: 0.0,
            refled: false,
            found_t: 0.0,
            found_by: None,
            found_line_at: 0.0,
            found_told: false,
            scream_at: 0.0,
            whimper_at: 0.0,
            greet_at: 0.0,
            last_line_at: 0.0,
            told: Vec::new(),
            known_bodies: Vec::new(),
            intel_n: 0.0,
            outside_call_at: 0.0,
            vis: 0.0,
            dice: 0.0,
        }
    }
}
impl Default for CivSnapOne { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/civilians.ts:224 CivSnapOne.talk, src/game/civilians.ts:224)
#[derive(Clone, Copy, Debug)]
pub struct CivSnapOneTalk {
    pub stage: f64,
    pub presses: f64,
    pub last_t: f64,
    pub abort_t: f64,
}
impl CivSnapOneTalk {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CivSnapOneTalk {
            stage: 0.0,
            presses: 0.0,
            last_t: 0.0,
            abort_t: 0.0,
        }
    }
}
impl Default for CivSnapOneTalk { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/civilians.ts:224 CivSnapOne.bubble, src/game/civilians.ts:224)
#[derive(Clone, Debug)]
pub struct CivSnapOneBubble {
    pub text: String,
    pub t: f64,
    pub dur: f64,
    pub radio: bool,
}
impl CivSnapOneBubble {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CivSnapOneBubble {
            text: String::new(),
            t: 0.0,
            dur: 0.0,
            radio: false,
        }
    }
}
impl Default for CivSnapOneBubble { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/civilians.ts:225 CivSnapOne.down, src/game/civilians.ts:225)
#[derive(Clone, Debug)]
pub struct CivSnapOneDown {
    pub kind: crate::game::DownKind,
    pub by: crate::game::DownBy,
    pub at: f64,
    pub found: bool,
    pub rag: Option<crate::civilians::CivSnapOneDownRag>,
}
impl CivSnapOneDown {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CivSnapOneDown {
            kind: crate::game::DownKind::Shot,
            by: crate::game::DownBy::Guard,
            at: 0.0,
            found: false,
            rag: None,
        }
    }
}
impl Default for CivSnapOneDown { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/civilians.ts:225 CivSnapOneDown.rag, src/game/civilians.ts:225)
#[derive(Clone, Debug)]
pub struct CivSnapOneDownRag {
    pub x: Vec<f64>,
    pub rot: [f64; 4],
    pub sleeping: bool,
    pub own: Option<bool>,
}
impl CivSnapOneDownRag {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CivSnapOneDownRag {
            x: Vec::new(),
            rot: [0.0; 4],
            sleeping: false,
            own: None,
        }
    }
}
impl Default for CivSnapOneDownRag { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum CivState {
    Held,
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

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
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

/// (TypeScript: inline at src/game/civilians.ts:228 CivSnapOne.target, src/game/civilians.ts:228)
#[derive(Clone, Debug)]
pub struct CivSnapOneTarget {
    pub kind: crate::civilians::CivSnapOneTargetKind,
    pub guard: Option<String>,
    pub spot: Option<String>,
    pub pos: V3,
    pub plan_pos: V3,
}
impl CivSnapOneTarget {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CivSnapOneTarget {
            kind: crate::civilians::CivSnapOneTargetKind::Guard,
            guard: None,
            spot: None,
            pos: [0.0; 3],
            plan_pos: [0.0; 3],
        }
    }
}
impl Default for CivSnapOneTarget { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum CivSnapOneTargetKind {
    Guard,
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

/// (TypeScript: inline at src/game/civilians.ts:235 CivSnap.floor, src/game/civilians.ts:235)
#[derive(Clone, Debug)]
pub struct CivSnapFloor {
    pub evac_told: bool,
    pub greet_at: f64,
    pub pending: Vec<crate::civilians::CivPendingSnap>,
}
impl CivSnapFloor {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CivSnapFloor {
            evac_told: false,
            greet_at: 0.0,
            pending: Vec::new(),
        }
    }
}
impl Default for CivSnapFloor { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: CivPendingSnap, src/game/civilians.ts:234)
#[derive(Clone, Debug)]
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
impl CivPendingSnap {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CivPendingSnap {
            at: 0.0,
            kind: crate::civilians::PendKind::GreetReply,
            cv: String::new(),
            o: None,
            key: None,
            armed: None,
            pos: None,
            lvl: None,
            dead: None,
        }
    }
}
impl Default for CivPendingSnap { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum PendKind {
    GreetReply,
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

/// (TypeScript: CivSpot, src/game/civilians.ts:55)
#[derive(Clone, Debug)]
pub struct CivSpot {
    pub name: String,
    pub pos: V3,
    pub face: f64,
}
impl CivSpot {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CivSpot {
            name: String::new(),
            pos: [0.0; 3],
            face: 0.0,
        }
    }
}
impl Default for CivSpot { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: CivTarget, src/game/civilians.ts:135)
#[derive(Clone, Debug)]
pub struct CivTarget {
    pub kind: crate::civilians::CivTargetKind,
    pub guard: Option<GuardId>,
    pub spot: Option<crate::civilians::CivSpot>,
    pub pos: V3,
    pub name: String,
    pub plan_pos: V3,
}
impl CivTarget {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CivTarget {
            kind: crate::civilians::CivTargetKind::Guard,
            guard: None,
            spot: None,
            pos: [0.0; 3],
            name: String::new(),
            plan_pos: [0.0; 3],
        }
    }
}
impl Default for CivTarget { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum CivTargetKind {
    Guard,
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

/// (TypeScript: Civ, src/game/civilians.ts:136)
#[derive(Clone, Debug)]
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
    pub lead_to: Option<V3>,
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
impl Civ {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Civ {
            def: crate::scene_level::CivilianDef::zeroed(),
            name: String::new(),
            label: String::new(),
            she: String::new(),
            her: String::new(),
            state: crate::civilians::CivState::Held,
            state_at: 0.0,
            task_t: 0.0,
            rise_t: 0.0,
            seated: false,
            settle_t: 0.0,
            look_at: None,
            look_until: 0.0,
            shrug_at: 0.0,
            cause: None,
            cause_pos: None,
            cause_at: 0.0,
            armed: false,
            alarm_n: 0.0,
            seen_at: 0.0,
            seen_pos: None,
            saw_gun: false,
            target: None,
            retarget_at: 0.0,
            bad_targets: JsSet::new(),
            freeze_until: 0.0,
            pin_target: false,
            report_to: None,
            talk_t: 0.0,
            reported: false,
            phoned: false,
            acked: false,
            cower_at: None,
            complied: false,
            comply_until: 0.0,
            refled: false,
            found_t: 0.0,
            found_by: None,
            found_line_at: 0.0,
            found_told: false,
            lead: None,
            lead_gap: 0.0,
            lead_to: None,
            scream_at: 0.0,
            whimper_at: 0.0,
            greet_at: 0.0,
            last_line_at: 0.0,
            told: JsMap::new(),
            known_bodies: JsSet::new(),
            intel_n: 0,
            outside_call_at: 0.0,
            vis: 0.0,
            dice: crate::dice::Die::default(),
            log: Vec::new(),
        }
    }
}
impl Default for Civ { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Pending, src/game/civilians.ts:448)
#[derive(Clone, Debug)]
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
impl Pending {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Pending {
            at: 0.0,
            kind: crate::civilians::PendKind::GreetReply,
            cv: GuardId(0),
            o: None,
            key: None,
            armed: None,
            pos: None,
            lvl: None,
            dead: None,
        }
    }
}
impl Default for Pending { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/civilians.ts:56 CIV_SPOTS, src/game/civilians.ts:56)
#[derive(Clone, Debug)]
pub struct CIVSPOTS {
    pub phones: Vec<crate::civilians::CivSpot>,
    pub exits: Vec<crate::civilians::CivSpot>,
}
impl CIVSPOTS {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CIVSPOTS {
            phones: Vec::new(),
            exits: Vec::new(),
        }
    }
}
impl Default for CIVSPOTS { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/civilians.ts:77 ownSpots, src/game/civilians.ts:77)
#[derive(Clone, Debug)]
pub struct CiviliansModOwnSpotsValue {
    pub phones: Vec<crate::civilians::CivSpot>,
    pub exits: Vec<crate::civilians::CivSpot>,
}
impl CiviliansModOwnSpotsValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CiviliansModOwnSpotsValue {
            phones: Vec::new(),
            exits: Vec::new(),
        }
    }
}
impl Default for CiviliansModOwnSpotsValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/civilians.ts:180 CIV_LOG, src/game/civilians.ts:180)
#[derive(Clone, Debug)]
pub struct CiviliansModCIVLOGItem {
    pub t: f64,
    pub who: String,
    pub what: String,
}
impl CiviliansModCIVLOGItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CiviliansModCIVLOGItem {
            t: 0.0,
            who: String::new(),
            what: String::new(),
        }
    }
}
impl Default for CiviliansModCIVLOGItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/civilians.ts:181 CIV_DEBUG, src/game/civilians.ts:181)
#[derive(Clone, Copy, Debug)]
pub struct CIVDEBUG {
}
impl CIVDEBUG {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CIVDEBUG {
        }
    }
}
impl Default for CIVDEBUG { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/civilians.ts:442 floors, src/game/civilians.ts:442)
#[derive(Clone, Debug)]
pub struct CiviliansModFloorsValue {
    pub evac_told: bool,
    pub greet_at: f64,
    pub pending: Vec<crate::civilians::Pending>,
}
impl CiviliansModFloorsValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CiviliansModFloorsValue {
            evac_told: false,
            greet_at: 0.0,
            pending: Vec::new(),
        }
    }
}
impl Default for CiviliansModFloorsValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/civilians.ts:675 TASK_CLIP, src/game/civilians.ts:675)
#[derive(Clone, Debug)]
pub struct TASKCLIPValue {
    pub clip: String,
    pub loop_: Option<bool>,
    pub rate: Option<f64>,
    pub in_: Option<f64>,
    pub out: Option<f64>,
    pub rise: Option<f64>,
}
impl TASKCLIPValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        TASKCLIPValue {
            clip: String::new(),
            loop_: None,
            rate: None,
            in_: None,
            out: None,
            rise: None,
        }
    }
}
impl Default for TASKCLIPValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/civilians.ts:70 civSpots ->, src/game/civilians.ts:70)
#[derive(Clone, Debug)]
pub struct CivSpots {
    pub phones: Vec<crate::civilians::CivSpot>,
    pub exits: Vec<crate::civilians::CivSpot>,
}
impl CivSpots {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CivSpots {
            phones: Vec::new(),
            exits: Vec::new(),
        }
    }
}
impl Default for CivSpots { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/civilians.ts:371 spawnHostage(def), src/game/civilians.ts:371)
#[derive(Clone, Debug)]
pub struct SpawnHostageDef {
    pub name: String,
    pub kind: crate::civilians::SpawnHostageDefKind,
    pub tint: V3,
    pub tint2: Option<V3>,
    pub pos: V3,
    pub face: f64,
}
impl SpawnHostageDef {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SpawnHostageDef {
            name: String::new(),
            kind: crate::civilians::SpawnHostageDefKind::Cleaner,
            tint: [0.0; 3],
            tint2: None,
            pos: [0.0; 3],
            face: 0.0,
        }
    }
}
impl Default for SpawnHostageDef { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum SpawnHostageDefKind {
    Cleaner,
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

/// (TypeScript: inline at src/game/civilians.ts:380 spawnTarget(def), src/game/civilians.ts:380)
#[derive(Clone, Debug)]
pub struct SpawnTargetDef {
    pub name: String,
    pub pos: V3,
    pub face: f64,
    pub tint: V3,
    pub tint2: Option<V3>,
    pub points: Option<Vec<V3>>,
    pub wait: Option<Vec<f64>>,
}
impl SpawnTargetDef {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SpawnTargetDef {
            name: String::new(),
            pos: [0.0; 3],
            face: 0.0,
            tint: [0.0; 3],
            tint2: None,
            points: None,
            wait: None,
        }
    }
}
impl Default for SpawnTargetDef { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/civilians.ts:449 later(P), ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1604)
#[derive(Clone, Debug)]
pub struct LaterP {
    pub dead: Option<bool>,
    pub kind: crate::civilians::PendKind,
    pub key: Option<String>,
    pub pos: Option<V3>,
    pub armed: Option<bool>,
    pub o: Option<GuardId>,
    pub lvl: Option<f64>,
    pub cv: GuardId,
}
impl LaterP {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LaterP {
            dead: None,
            kind: crate::civilians::PendKind::GreetReply,
            key: None,
            pos: None,
            armed: None,
            o: None,
            lvl: None,
            cv: GuardId(0),
        }
    }
}
impl Default for LaterP { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: a union)
#[derive(Clone, Debug)]
pub enum InViewVp {
    Array(Vec<f64>),
    Float32Array(Vec<f32>),
}

/// (TypeScript: inline at src/game/civilians.ts:684 work ->, src/game/civilians.ts:684)
#[derive(Clone, Copy, Debug)]
pub struct Work {
    pub move_: f64,
}
impl Work {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Work {
            move_: 0.0,
        }
    }
}
impl Default for Work { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/civilians.ts:795 pathMetres ->, src/game/civilians.ts:795)
#[derive(Clone, Debug)]
pub struct PathMetres {
    pub m: f64,
    pub path: Option<Vec<V3>>,
}
impl PathMetres {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PathMetres {
            m: 0.0,
            path: None,
        }
    }
}
impl Default for PathMetres { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/civilians.ts:805 pickTarget(opts), src/game/civilians.ts:805)
#[derive(Clone, Copy, Debug)]
pub struct PickTargetOpts {
    pub exits_only: Option<bool>,
}
impl PickTargetOpts {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PickTargetOpts {
            exits_only: None,
        }
    }
}
impl Default for PickTargetOpts { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
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

/// (TypeScript: inline at src/game/civilians.ts:1000 follow ->, src/game/civilians.ts:1000)
#[derive(Clone, Copy, Debug)]
pub struct Follow {
    pub move_: f64,
    pub crouch: f64,
}
impl Follow {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Follow {
            move_: 0.0,
            crouch: 0.0,
        }
    }
}
impl Default for Follow { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/civilians.ts:1019 cower ->, src/game/civilians.ts:1019)
#[derive(Clone, Copy, Debug)]
pub struct Cower {
    pub move_: f64,
    pub crouch: f64,
}
impl Cower {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Cower {
            move_: 0.0,
            crouch: 0.0,
        }
    }
}
impl Default for Cower { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/civilians.ts:1074 civilianSamLine ->, src/game/civilians.ts:1074)
#[derive(Clone, Debug)]
pub struct CivilianSamLine {
    pub text: String,
    pub id: String,
}
impl CivilianSamLine {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CivilianSamLine {
            text: String::new(),
            id: String::new(),
        }
    }
}
impl Default for CivilianSamLine { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
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
#[derive(Clone, Debug)]
pub struct CiviliansMod {
    /// (src/game/civilians.ts:77 ownSpots)
    pub own_spots: Option<crate::civilians::CivSpots>,
    /// (src/game/civilians.ts:180 CIV_LOG)
    pub civ_log: Vec<crate::civilians::CiviliansModCIVLOGItem>,
    /// (src/game/civilians.ts:442 floors)
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

/// (src/game/civilians.ts:56 CIV_SPOTS: a table nothing writes)
pub fn civ_spots_table() -> crate::civilians::CIVSPOTS { crate::civilians::CIVSPOTS { phones: alloc::vec![crate::civilians::CivSpot { name: String::from("the reception phone"), pos: [8.9, 0.0, 13.8], face: 0.0 }, crate::civilians::CivSpot { name: String::from("the break-room phone"), pos: [34.6, 0.0, 16.2], face: 1.5707963267948966 }], exits: alloc::vec![crate::civilians::CivSpot { name: String::from("the fire exit"), pos: [38.2, 0.0, 23.2], face: 0.0 }, crate::civilians::CivSpot { name: String::from("the outside door"), pos: [34.0, 0.0, 25.0], face: 1.5707963267948966 }] } }

/// (src/game/civilians.ts:121 CIV_ID0)
pub const CIV_ID0: f64 = 40.0;

/// (src/game/civilians.ts:181 CIV_DEBUG: a table nothing writes)
pub fn civ_debug() -> crate::civilians::CIVDEBUG { crate::civilians::CIVDEBUG {  } }

/// (src/game/civilians.ts:434 NONE: a table nothing writes)
pub fn none() -> Vec<GuardId> { Vec::new() }

/// (src/game/civilians.ts:666 SIT_ADVANCE)
pub const SIT_ADVANCE: f64 = 0.33;

/// (src/game/civilians.ts:675 TASK_CLIP: a table nothing writes)
pub fn task_clip() -> JsMap<String, Option<crate::civilians::TASKCLIPValue>> { { let mut t = JsMap::new(); t.set(String::from("stand"), None); t.set(String::from("wipe"), Some(crate::civilians::TASKCLIPValue { clip: String::from("Interact"), loop_: Some(true), rate: Some(0.85), in_: Some(0.4), out: Some(0.4), rise: None })); t.set(String::from("bin"), Some(crate::civilians::TASKCLIPValue { clip: String::from("Fixing_Kneeling"), loop_: Some(true), rate: Some(0.8), in_: Some(0.6), out: Some(0.7), rise: Some(0.8) })); t.set(String::from("sit"), Some(crate::civilians::TASKCLIPValue { clip: String::from("Sitting_Idle_Loop"), loop_: Some(true), rate: Some(1.0), in_: Some(0.7), out: Some(0.8), rise: Some(1.0) })); t.set(String::from("cooler"), Some(crate::civilians::TASKCLIPValue { clip: String::from("Interact"), loop_: Some(true), rate: Some(0.6), in_: Some(0.4), out: Some(0.4), rise: None })); t.set(String::from("printer"), Some(crate::civilians::TASKCLIPValue { clip: String::from("Interact"), loop_: Some(true), rate: Some(0.7), in_: Some(0.4), out: Some(0.4), rise: None })); t.set(String::from("shelf"), Some(crate::civilians::TASKCLIPValue { clip: String::from("PickUp_Table"), loop_: Some(true), rate: Some(0.7), in_: Some(0.4), out: Some(0.5), rise: None })); t } }

/// (src/game/civilians.ts:1071 DIGIT_WORDS: a table nothing writes)
pub fn digit_words() -> Vec<String> { alloc::vec![String::from("oh"), String::from("one"), String::from("two"), String::from("three"), String::from("four"), String::from("five"), String::from("six"), String::from("seven"), String::from("eight"), String::from("nine")] }

/// (src/game/civilians.ts:1251 LS_KEY)
pub const LS_KEY: &str = "twopointsix.civilians";
