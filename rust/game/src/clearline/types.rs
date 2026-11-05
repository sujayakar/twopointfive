//! The types of `clearline`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: ClearNote, src/game/clearline.ts:112)
#[derive(Clone, Debug)]
pub struct ClearNote {
    pub t: f64,
    pub who: String,
    pub what: crate::clearline::ClearNoteWhat,
    pub by: String,
    pub at: Option<[f64; 2]>,
    pub walk: f64,
}
impl ClearNote {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ClearNote {
            t: 0.0,
            who: String::new(),
            what: crate::clearline::ClearNoteWhat::Cover,
            by: String::new(),
            at: None,
            walk: 0.0,
        }
    }
}
impl Default for ClearNote { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum ClearNoteWhat {
    Cover,
    Step,
    End,
    Kneel,
}
impl ClearNoteWhat {
    pub const fn as_str(self) -> &'static str {
        match self {
            ClearNoteWhat::Cover => "cover",
            ClearNoteWhat::Step => "step",
            ClearNoteWhat::End => "end",
            ClearNoteWhat::Kneel => "kneel",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum What {
    Cover,
    Step,
    Kneel,
}
impl What {
    pub const fn as_str(self) -> &'static str {
        match self {
            What::Cover => "cover",
            What::Step => "step",
            What::Kneel => "kneel",
        }
    }
}

/// (TypeScript: Doing, src/game/clearline.ts:124)
#[derive(Clone, Copy, Debug)]
pub struct Doing {
    pub what: crate::clearline::What,
    pub spot: V3,
    pub of: GuardId,
    pub t0: f64,
    pub on_t: f64,
    pub until: f64,
    pub ask_t: f64,
    pub look_t: f64,
    pub from: V3,
    pub order_t: f64,
    pub eta: f64,
    pub good_t: f64,
    pub head: bool,
}
impl Doing {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Doing {
            what: crate::clearline::What::Cover,
            spot: [0.0; 3],
            of: GuardId(0),
            t0: 0.0,
            on_t: 0.0,
            until: 0.0,
            ask_t: 0.0,
            look_t: 0.0,
            from: [0.0; 3],
            order_t: 0.0,
            eta: 0.0,
            good_t: 0.0,
            head: false,
        }
    }
}
impl Default for Doing { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Mem, src/game/clearline.ts:129)
#[derive(Clone, Debug)]
pub struct Mem {
    pub since: f64,
    pub seen_t: f64,
    pub said_t: f64,
    pub said: String,
    pub doing: Option<crate::clearline::Doing>,
    pub rest_until: f64,
    pub try_t: f64,
    pub run_t: f64,
    pub run_of: Option<GuardId>,
    pub lead_t: f64,
    pub lead_p: Option<[f64; 3]>,
    pub lead_of: Option<crate::clearline::LeadsBy>,
    pub lead_for: Option<GuardId>,
}
impl Mem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Mem {
            since: 0.0,
            seen_t: 0.0,
            said_t: 0.0,
            said: String::new(),
            doing: None,
            rest_until: 0.0,
            try_t: 0.0,
            run_t: 0.0,
            run_of: None,
            lead_t: 0.0,
            lead_p: None,
            lead_of: None,
            lead_for: None,
        }
    }
}
impl Default for Mem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/clearline.ts:171 samAt, src/game/clearline.ts:171)
#[derive(Clone, Copy, Debug)]
pub struct ClearlineModSamAtValue {
    pub p: [f64; 3],
    pub move_t: f64,
}
impl ClearlineModSamAtValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ClearlineModSamAtValue {
            p: [0.0; 3],
            move_t: 0.0,
        }
    }
}
impl Default for ClearlineModSamAtValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/clearline.ts:180 entered, src/game/clearline.ts:180)
#[derive(Clone, Copy, Debug)]
pub struct ClearlineModEnteredValue {
    pub op: DoorOpId,
    pub a: OrdersAssignmentId,
    pub past: bool,
}
impl ClearlineModEnteredValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ClearlineModEnteredValue {
            op: DoorOpId(0),
            a: OrdersAssignmentId(0),
            past: false,
        }
    }
}
impl Default for ClearlineModEnteredValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: a union)
#[derive(Clone, Copy, Debug)]
pub enum LeadsBy {
    Guard(GuardId),
    Player,
}

/// (TypeScript: inline at src/game/clearline.ts:242 findSpot ->, src/game/clearline.ts:242)
#[derive(Clone, Copy, Debug)]
pub struct FindSpot {
    pub p: V3,
    pub along: f64,
    pub side: f64,
}
impl FindSpot {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FindSpot {
            p: [0.0; 3],
            along: 0.0,
            side: 0.0,
        }
    }
}
impl Default for FindSpot { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum NoteWhat {
    Cover,
    Step,
    End,
    Kneel,
}
impl NoteWhat {
    pub const fn as_str(self) -> &'static str {
        match self {
            NoteWhat::Cover => "cover",
            NoteWhat::Step => "step",
            NoteWhat::End => "end",
            NoteWhat::Kneel => "kneel",
        }
    }
}

/// (TypeScript: inline at src/game/clearline.ts:297 clearLineFeet ->, src/game/clearline.ts:297)
#[derive(Clone, Copy, Debug)]
pub struct ClearLineFeet {
    pub goal: V3,
    pub speed: f64,
    pub face: Option<f64>,
    pub strafe: bool,
    pub crouch: bool,
}
impl ClearLineFeet {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ClearLineFeet {
            goal: [0.0; 3],
            speed: 0.0,
            face: None,
            strafe: false,
            crouch: false,
        }
    }
}
impl Default for ClearLineFeet { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: a union)
#[derive(Clone, Copy, Debug)]
pub enum ClearLineBlockedBy {
    Guard(GuardId),
    Player,
}

/// (TypeScript: inline at src/game/clearline.ts:406 clearLineOf ->, src/game/clearline.ts:406)
#[derive(Clone, Copy, Debug)]
pub struct ClearLineOf {
    pub what: crate::clearline::What,
    pub spot: V3,
    pub of: GuardId,
}
impl ClearLineOf {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ClearLineOf {
            what: crate::clearline::What::Cover,
            spot: [0.0; 3],
            of: GuardId(0),
        }
    }
}
impl Default for ClearLineOf { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct ClearlineMod {
    /// (src/game/clearline.ts:113 CLEARLINE_LOG)
    pub clearline_log: Vec<crate::clearline::ClearNote>,
    /// (src/game/clearline.ts:131 mem)
    pub mem: JsMap<GuardId, crate::clearline::Mem>,
    /// (src/game/clearline.ts:171 samAt)
    pub sam_at: Option<crate::clearline::ClearlineModSamAtValue>,
    /// (src/game/clearline.ts:180 entered)
    pub entered: JsMap<GuardId, crate::clearline::ClearlineModEnteredValue>,
}
impl ClearlineMod {
    pub fn blank() -> Self {
        ClearlineMod {
            clearline_log: Vec::new(),
            mem: JsMap::new(),
            sam_at: None,
            entered: JsMap::new(),
        }
    }
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct ClearlineStats {
    pub blocked: i32,
    pub steps: i32,
    pub up: i32,
    pub planted: i32,
    pub covers: i32,
    pub kneels: i32,
    pub cues: i32,
    pub ended: i32,
    pub nothing: i32,
    pub behind: i32,
    pub moving: i32,
}

/// (src/game/clearline.ts:110 CLEARLINE_DEFAULTS: a table nothing writes)
pub fn clearline_defaults() -> crate::knobs::ClearlineK { crate::knobs::ClearlineK { on: false, after: 0.3, reach_m: 3.0, pace: 2.0, body_m: 0.6, spare_m: 0.15, cover: true, cover_secs: 3.0, kneel: true, cue: true, again: 0.5, lose: 1.0, hold_max: 10.0, rest: 8.0, stale: 1.5, after_edge: 0.0, in_entry: true, hold_behind: 2.0, hold_step: true, still: 0.0, still_far: 0.0 } }

/// (src/game/clearline.ts:115 LS)
pub const LS: &str = "twopointsix.clearline";

/// (src/game/clearline.ts:116 KEPT: a table nothing writes)
pub fn kept() -> [String; 14] { [String::from("on"), String::from("after"), String::from("afterEdge"), String::from("inEntry"), String::from("holdBehind"), String::from("holdStep"), String::from("still"), String::from("stillFar"), String::from("reachM"), String::from("pace"), String::from("cover"), String::from("kneel"), String::from("cue"), String::from("holdMax")] }

/// (src/game/clearline.ts:154 LEAD_V)
pub const LEAD_V: f64 = 0.5;

/// (src/game/clearline.ts:220 SHOULDER)
pub const SHOULDER: f64 = 0.24;

/// (src/game/clearline.ts:221 ROOM: a table nothing writes)
pub fn room() -> Vec<[f64; 2]> { alloc::vec![[0.3, 0.0], [-0.3, 0.0], [0.0, 0.3], [0.0, -0.3]] }
