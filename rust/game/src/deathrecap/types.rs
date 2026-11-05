//! The types of `deathrecap`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum RecapCause {
    Blast,
    Round,
    Other,
    Unknown,
}
impl RecapCause {
    pub const fn as_str(self) -> &'static str {
        match self {
            RecapCause::Blast => "blast",
            RecapCause::Round => "round",
            RecapCause::Other => "other",
            RecapCause::Unknown => "unknown",
        }
    }
}

/// (TypeScript: RecapMan, src/game/deathrecap.ts:52)
#[derive(Clone, Debug)]
pub struct RecapMan {
    pub id: f64,
    pub name: String,
    pub side: String,
    pub kind: crate::deathrecap::RecapManKind,
}
impl RecapMan {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RecapMan {
            id: 0.0,
            name: String::new(),
            side: String::new(),
            kind: crate::deathrecap::RecapManKind::Player,
        }
    }
}
impl Default for RecapMan { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum RecapManKind {
    Player,
    Civ,
    Man,
}
impl RecapManKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            RecapManKind::Player => "player",
            RecapManKind::Civ => "civ",
            RecapManKind::Man => "man",
        }
    }
}

/// (TypeScript: RecapDeath, src/game/deathrecap.ts:54)
#[derive(Clone, Debug)]
pub struct RecapDeath {
    pub t: f64,
    pub victim: f64,
    pub name: String,
    pub side: String,
    pub player: bool,
    pub at: [f64; 2],
    pub gaze: f64,
    pub cause: crate::deathrecap::RecapCause,
    pub by: f64,
    pub by_name: String,
    pub by_side: String,
    pub by_at: Option<[f64; 2]>,
    pub from: Option<[f64; 2]>,
    pub to: [f64; 2],
    pub weapon: String,
    pub dist: f64,
    pub off: f64,
    pub blind: bool,
    pub loud: bool,
    /// INVENTED: g.serial() where explain makes the death (deathrecap.ts:146): take asks by identity whether this death already has a take (deathrecap.ts:195)
    pub serial: u32,
}
impl RecapDeath {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RecapDeath {
            t: 0.0,
            victim: 0.0,
            name: String::new(),
            side: String::new(),
            player: false,
            at: [0.0; 2],
            gaze: 0.0,
            cause: crate::deathrecap::RecapCause::Blast,
            by: 0.0,
            by_name: String::new(),
            by_side: String::new(),
            by_at: None,
            from: None,
            to: [0.0; 2],
            weapon: String::new(),
            dist: 0.0,
            off: 0.0,
            blind: false,
            loud: false,
            serial: 0,
        }
    }
}
impl Default for RecapDeath { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: RecapShot, src/game/deathrecap.ts:67)
#[derive(Clone, Debug)]
pub struct RecapShot {
    pub t: f64,
    pub kind: crate::deathrecap::RecapShotKind,
    pub who: f64,
    pub from: [f64; 2],
    pub to: Option<[f64; 2]>,
    pub at: Option<[f64; 2]>,
    pub weapon: Option<String>,
    pub loud: Option<bool>,
}
impl RecapShot {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RecapShot {
            t: 0.0,
            kind: crate::deathrecap::RecapShotKind::Blast,
            who: 0.0,
            from: [0.0; 2],
            to: None,
            at: None,
            weapon: None,
            loud: None,
        }
    }
}
impl Default for RecapShot { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum RecapShotKind {
    Blast,
    Bang,
    Round,
}
impl RecapShotKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            RecapShotKind::Blast => "blast",
            RecapShotKind::Bang => "bang",
            RecapShotKind::Round => "round",
        }
    }
}

/// (TypeScript: RecapFrame, src/game/deathrecap.ts:69)
#[derive(Clone, Debug)]
pub struct RecapFrame {
    pub t: f64,
    pub n: f64,
    pub men: Vec<f32>,
    pub doors: Vec<f32>,
}
impl RecapFrame {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RecapFrame {
            t: 0.0,
            n: 0.0,
            men: Vec::new(),
            doors: Vec::new(),
        }
    }
}
impl Default for RecapFrame { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: RecapTake, src/game/deathrecap.ts:72)
#[derive(Clone, Debug)]
pub struct RecapTake {
    pub death: crate::deathrecap::RecapDeath,
    pub t0: f64,
    pub t1: f64,
    pub cast: Vec<crate::deathrecap::RecapMan>,
    pub doors: Vec<crate::deathrecap::RecapDoor>,
    pub frames: Vec<crate::deathrecap::RecapFrame>,
    pub shots: Vec<crate::deathrecap::RecapShot>,
    pub deaths: Vec<crate::deathrecap::RecapDeath>,
}
impl RecapTake {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RecapTake {
            death: crate::deathrecap::RecapDeath::zeroed(),
            t0: 0.0,
            t1: 0.0,
            cast: Vec::new(),
            doors: Vec::new(),
            frames: Vec::new(),
            shots: Vec::new(),
            deaths: Vec::new(),
        }
    }
}
impl Default for RecapTake { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: RecapDoor, src/game/deathrecap.ts:70)
#[derive(Clone, Copy, Debug)]
pub struct RecapDoor {
    pub hinge: [f64; 2],
    pub width: f64,
}
impl RecapDoor {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RecapDoor {
            hinge: [0.0; 2],
            width: 0.0,
        }
    }
}
impl Default for RecapDoor { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: LostMan, src/game/deathrecap.ts:78)
#[derive(Clone, Debug)]
pub struct LostMan {
    pub id: f64,
    pub name: String,
    pub t: f64,
    pub at: [f64; 2],
    pub from: Option<[f64; 2]>,
    pub known_at: f64,
}
impl LostMan {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LostMan {
            id: 0.0,
            name: String::new(),
            t: 0.0,
            at: [0.0; 2],
            from: None,
            known_at: 0.0,
        }
    }
}
impl Default for LostMan { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Lost, src/game/deathrecap.ts:81)
#[derive(Clone, Debug)]
pub struct Lost {
    pub man: crate::deathrecap::LostMan,
    pub src: Option<[f64; 2]>,
    pub heard: bool,
    pub saw: Option<GuardId>,
    pub sam_saw: bool,
    pub done: bool,
}
impl Lost {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Lost {
            man: crate::deathrecap::LostMan::zeroed(),
            src: None,
            heard: false,
            saw: None,
            sam_saw: false,
            done: false,
        }
    }
}
impl Default for Lost { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Round, src/game/deathrecap.ts:86)
#[derive(Clone, Debug)]
pub struct Round {
    pub t: f64,
    pub by: f64,
    pub by_at: [f64; 2],
    pub from: [f64; 2],
    pub to: [f64; 2],
    pub weapon: String,
    pub blind: bool,
    pub loud: bool,
}
impl Round {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Round {
            t: 0.0,
            by: 0.0,
            by_at: [0.0; 2],
            from: [0.0; 2],
            to: [0.0; 2],
            weapon: String::new(),
            blind: false,
            loud: false,
        }
    }
}
impl Default for Round { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: State, src/game/deathrecap.ts:87)
#[derive(Clone, Debug)]
pub struct State {
    pub mission: u32,
    pub last_t: f64,
    pub next: f64,
    pub primed: bool,
    pub over_at: f64,
    pub frames: Vec<crate::deathrecap::RecapFrame>,
    pub spare: Vec<crate::deathrecap::RecapFrame>,
    pub shots: Vec<crate::deathrecap::RecapShot>,
    pub seen: JsSet<u32>,
    pub down: JsSet<crate::Num>,
    pub rounds: JsMap<crate::Num, crate::deathrecap::Round>,
    pub cast: JsMap<crate::Num, crate::deathrecap::RecapMan>,
    pub deaths: Vec<crate::deathrecap::RecapDeath>,
    pub due: Vec<crate::deathrecap::StateDueItem>,
    pub takes: Vec<crate::deathrecap::RecapTake>,
    pub lost: Vec<crate::deathrecap::Lost>,
    pub lost_pub: Vec<crate::deathrecap::LostMan>,
    pub lost_next: f64,
}
impl State {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        State {
            mission: 0,
            last_t: 0.0,
            next: 0.0,
            primed: false,
            over_at: 0.0,
            frames: Vec::new(),
            spare: Vec::new(),
            shots: Vec::new(),
            seen: JsSet::new(),
            down: JsSet::new(),
            rounds: JsMap::new(),
            cast: JsMap::new(),
            deaths: Vec::new(),
            due: Vec::new(),
            takes: Vec::new(),
            lost: Vec::new(),
            lost_pub: Vec::new(),
            lost_next: 0.0,
        }
    }
}
impl Default for State { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at quiet State.mission, quiet State.mission)
#[derive(Clone, Copy, Debug)]
pub struct StateMission {
}
impl StateMission {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StateMission {
        }
    }
}
impl Default for StateMission { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/deathrecap.ts:92 State.due, src/game/deathrecap.ts:92)
#[derive(Clone, Debug)]
pub struct StateDueItem {
    pub death: crate::deathrecap::RecapDeath,
    pub at: f64,
}
impl StateDueItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StateDueItem {
            death: crate::deathrecap::RecapDeath::zeroed(),
            at: 0.0,
        }
    }
}
impl Default for StateDueItem { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct DeathrecapMod {
    /// (src/game/deathrecap.ts:95 states)
    pub states: Option<crate::deathrecap::State>,
}
impl DeathrecapMod {
    pub fn blank() -> Self {
        DeathrecapMod {
            states: None,
        }
    }
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct DeathrecapStats {
    pub deaths: i32,
    pub takes: i32,
    pub rounds: i32,
    pub known: i32,
    pub calls: i32,
}

/// (src/game/deathrecap.ts:83 MAN_STRIDE)
pub const MAN_STRIDE: f64 = 5.0;

/// (src/game/deathrecap.ts:132 CAUSE_SECS)
pub const CAUSE_SECS: f64 = 0.3;

/// (src/game/deathrecap.ts:134 BLAST_M)
pub const BLAST_M: f64 = 9.0;

/// (src/game/deathrecap.ts:137 LINE_M)
pub const LINE_M: f64 = 0.6;

/// (src/game/deathrecap.ts:315 NONE: a table nothing writes)
pub fn none() -> Vec<crate::deathrecap::LostMan> { Vec::new() }
