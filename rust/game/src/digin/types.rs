//! The types of `digin`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Phase {
    Set,
    Idle,
    Shut,
    Haul,
    Think,
    ToRoom,
    ToProp,
    ToDoor,
    Wedge,
    ToCover,
}
impl Phase {
    pub const fn as_str(self) -> &'static str {
        match self {
            Phase::Set => "set",
            Phase::Idle => "idle",
            Phase::Shut => "shut",
            Phase::Haul => "haul",
            Phase::Think => "think",
            Phase::ToRoom => "toRoom",
            Phase::ToProp => "toProp",
            Phase::ToDoor => "toDoor",
            Phase::Wedge => "wedge",
            Phase::ToCover => "toCover",
        }
    }
}

/// (TypeScript: Man, src/game/digin.ts:216)
#[derive(Clone, Copy, Debug)]
pub struct Man {
    pub far: bool,
    pub owe: f64,
    pub haul_by: f64,
    pub deep: bool,
    pub look: Option<V3>,
    pub tried_deep: bool,
    pub gathered: bool,
    pub gather_to: Option<RoomfightRoomId>,
    pub dug_once: bool,
    pub ordered: bool,
    pub phase: crate::digin::Phase,
    pub until: f64,
    pub know: V3,
    pub know_t: f64,
    pub room: Option<RoomfightRoomId>,
    pub entry: f64,
    pub brace: f64,
    pub prop: Option<PropId>,
    pub stand: V3,
    pub rest: V3,
    pub scrape_t: f64,
    pub set_at: f64,
    pub spot: Option<V3>,
    pub kept: bool,
    pub eyes_t: Option<f64>,
    pub lane_t: Option<f64>,
    pub also: Option<f64>,
    pub also_n: Option<f64>,
    pub home_t: Option<f64>,
    pub home_d: Option<f64>,
    pub home_at: Option<f64>,
    pub hand_t: Option<f64>,
    pub pull_t: Option<f64>,
    pub pushes: Option<f64>,
    pub pull_a: Option<f64>,
    pub stalls: Option<f64>,
}
impl Man {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Man {
            far: false,
            owe: 0.0,
            haul_by: 0.0,
            deep: false,
            look: None,
            tried_deep: false,
            gathered: false,
            gather_to: None,
            dug_once: false,
            ordered: false,
            phase: crate::digin::Phase::Set,
            until: 0.0,
            know: [0.0; 3],
            know_t: 0.0,
            room: None,
            entry: 0.0,
            brace: 0.0,
            prop: None,
            stand: [0.0; 3],
            rest: [0.0; 3],
            scrape_t: 0.0,
            set_at: 0.0,
            spot: None,
            kept: false,
            eyes_t: None,
            lane_t: None,
            also: None,
            also_n: None,
            home_t: None,
            home_d: None,
            home_at: None,
            hand_t: None,
            pull_t: None,
            pushes: None,
            pull_a: None,
            stalls: None,
        }
    }
}
impl Default for Man { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Cue {
    Kick,
    Scrape,
    Radio,
    Shout,
    Tried,
}
impl Cue {
    pub const fn as_str(self) -> &'static str {
        match self {
            Cue::Kick => "kick",
            Cue::Scrape => "scrape",
            Cue::Radio => "radio",
            Cue::Shout => "shout",
            Cue::Tried => "tried",
        }
    }
}

/// (TypeScript: RoomSt, src/game/digin.ts:218)
#[derive(Clone, Debug)]
pub struct RoomSt {
    pub room: RoomfightRoomId,
    pub expected: f64,
    pub second: f64,
    pub at: f64,
    pub t0: f64,
    pub men: Vec<GuardId>,
    pub bracer: Option<GuardId>,
    pub wedged: f64,
    pub cue_at: f64,
    pub cues: JsSet<crate::digin::Cue>,
    pub entered_at: f64,
    pub ready_at: f64,
    pub callout: bool,
}
impl RoomSt {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RoomSt {
            room: RoomfightRoomId(0),
            expected: 0.0,
            second: 0.0,
            at: 0.0,
            t0: 0.0,
            men: Vec::new(),
            bracer: None,
            wedged: 0.0,
            cue_at: 0.0,
            cues: JsSet::new(),
            entered_at: 0.0,
            ready_at: 0.0,
            callout: false,
        }
    }
}
impl Default for RoomSt { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: St, src/game/digin.ts:219)
#[derive(Clone, Debug)]
pub struct St {
    pub men: JsMap<GuardId, crate::digin::Man>,
    pub rooms: JsMap<RoomfightRoomId, crate::digin::RoomSt>,
    pub wedges: JsMap<DoorId, crate::digin::StWedgesValue>,
    pub fix: Option<crate::digin::StFix>,
    pub called: JsSet<GuardId>,
    pub used: JsSet<PropId>,
    pub away: JsSet<DoorId>,
    pub frame_t: f64,
    pub bang_call: f64,
}
impl St {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        St {
            men: JsMap::new(),
            rooms: JsMap::new(),
            wedges: JsMap::new(),
            fix: None,
            called: JsSet::new(),
            used: JsSet::new(),
            away: JsSet::new(),
            frame_t: 0.0,
            bang_call: 0.0,
        }
    }
}
impl Default for St { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/digin.ts:219 St.wedges, src/game/digin.ts:219)
#[derive(Clone, Debug)]
pub struct StWedgesValue {
    pub key_side: f64,
    pub lock_id: String,
    pub kicked: bool,
    pub tried: bool,
    pub by_t: f64,
}
impl StWedgesValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StWedgesValue {
            key_side: 0.0,
            lock_id: String::new(),
            kicked: false,
            tried: false,
            by_t: 0.0,
        }
    }
}
impl Default for StWedgesValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/digin.ts:219 St.fix, src/game/digin.ts:219)
#[derive(Clone, Copy, Debug)]
pub struct StFix {
    pub at: V3,
    pub t: f64,
    pub room: Option<RoomfightRoomId>,
    pub heard: bool,
}
impl StFix {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StFix {
            at: [0.0; 3],
            t: 0.0,
            room: None,
            heard: false,
        }
    }
}
impl Default for StFix { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/digin.ts:189 DIGIN_LOG, src/game/digin.ts:189)
#[derive(Clone, Debug)]
pub struct DiginModDIGINLOGItem {
    pub t: f64,
    pub who: String,
    pub what: String,
}
impl DiginModDIGINLOGItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DiginModDIGINLOGItem {
            t: 0.0,
            who: String::new(),
            what: String::new(),
        }
    }
}
impl Default for DiginModDIGINLOGItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/digin.ts:192 DIGIN_SHUT, src/game/digin.ts:192)
#[derive(Clone, Copy, Debug)]
pub struct DiginModDIGINSHUT {
    pub home: f64,
    pub held: f64,
    pub again: f64,
    pub more: f64,
    pub left: f64,
}
impl DiginModDIGINSHUT {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DiginModDIGINSHUT {
            home: 0.0,
            held: 0.0,
            again: 0.0,
            more: 0.0,
            left: 0.0,
        }
    }
}
impl Default for DiginModDIGINSHUT { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/digin.ts:197 NUM_MAX, src/game/digin.ts:197)
#[derive(Clone, Copy, Debug)]
pub struct NUMMAX {
    pub wedges: f64,
    pub pace: f64,
    pub reach_m: f64,
}
impl NUMMAX {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        NUMMAX {
            wedges: 0.0,
            pace: 0.0,
            reach_m: 0.0,
        }
    }
}
impl Default for NUMMAX { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/digin.ts:231 listeners ->, src/game/digin.ts:231)
#[derive(Clone, Copy, Debug)]
pub struct ListenersItem {
    pub gd: Option<GuardId>,
    pub head: V3,
    pub vel: V3,
}
impl ListenersItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ListenersItem {
            gd: None,
            head: [0.0; 3],
            vel: [0.0; 3],
        }
    }
}
impl Default for ListenersItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/digin.ts:238 heardBy ->, src/game/digin.ts:238)
#[derive(Clone, Copy, Debug)]
pub struct HeardBy {
    pub gd: Option<GuardId>,
}
impl HeardBy {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        HeardBy {
            gd: None,
        }
    }
}
impl Default for HeardBy { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/digin.ts:244 heardNoise ->, src/game/digin.ts:244)
#[derive(Clone, Copy, Debug)]
pub struct HeardNoise {
    pub gd: Option<GuardId>,
}
impl HeardNoise {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        HeardNoise {
            gd: None,
        }
    }
}
impl Default for HeardNoise { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/digin.ts:277 coverSpot ->, src/game/digin.ts:277)
#[derive(Clone, Copy, Debug)]
pub struct CoverSpot {
    pub spot: Option<V3>,
    pub here: bool,
    pub look: Option<V3>,
}
impl CoverSpot {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CoverSpot {
            spot: None,
            here: false,
            look: None,
        }
    }
}
impl Default for CoverSpot { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/digin.ts:520 gatherTo ->, src/game/digin.ts:520)
#[derive(Clone, Copy, Debug)]
pub struct GatherTo {
    pub room: RoomfightRoomId,
    pub spot: V3,
}
impl GatherTo {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GatherTo {
            room: RoomfightRoomId(0),
            spot: [0.0; 3],
        }
    }
}
impl Default for GatherTo { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/digin.ts:859 digInWedged ->, src/game/digin.ts:859)
#[derive(Clone, Copy, Debug)]
pub struct DigInWedgedItem {
    pub door: DoorId,
    pub at: V3,
}
impl DigInWedgedItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DigInWedgedItem {
            door: DoorId(0),
            at: [0.0; 3],
        }
    }
}
impl Default for DigInWedgedItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/digin.ts:881 digInRefuge ->, src/game/digin.ts:881)
#[derive(Clone, Copy, Debug)]
pub struct DigInRefuge {
    pub spot: V3,
    pub door: Option<DoorId>,
}
impl DigInRefuge {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DigInRefuge {
            spot: [0.0; 3],
            door: None,
        }
    }
}
impl Default for DigInRefuge { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct DiginMod {
    /// (src/game/digin.ts:189 DIGIN_LOG)
    pub digin_log: Vec<crate::digin::DiginModDIGINLOGItem>,
    /// (src/game/digin.ts:192 DIGIN_SHUT)
    pub digin_shut: crate::digin::DiginModDIGINSHUT,
    /// (src/game/digin.ts:220 states)
    pub states: Option<crate::digin::St>,
}
impl DiginMod {
    pub fn blank() -> Self {
        DiginMod {
            digin_log: Vec::new(),
            digin_shut: crate::digin::DiginModDIGINSHUT { home: 0.0, held: 0.0, again: 0.0, more: 0.0, left: 0.0 },
            states: None,
        }
    }
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct DiginStats {
    pub heard: i32,
    pub far: i32,
    pub dug: i32,
    pub moved: i32,
    pub stayed: i32,
    pub hauls: i32,
    pub brace_beside: i32,
    pub brace_passed: i32,
    pub braced: i32,
    pub kicked: i32,
    pub pulled_to: i32,
    pub radio: i32,
    pub shouts: i32,
    pub scrapes: i32,
    pub callouts: i32,
    pub no_rally: i32,
    pub aborted: i32,
    pub caught: i32,
    pub steady: i32,
    pub tried: i32,
    pub deep: i32,
    pub bang_calls: i32,
    pub gathered: i32,
    pub gather_lost: i32,
    pub refuge: i32,
    pub lane_moved: i32,
    pub lane_kept: i32,
    pub rooms: i32,
    pub entered: i32,
    pub cue_first: i32,
    pub ready_first: i32,
    pub half_ready: i32,
    pub cue_radio: i32,
    pub cue_shout: i32,
    pub cue_scrape: i32,
    pub cue_tried: i32,
    pub cue_kick: i32,
    pub wedged_at_entry: i32,
    pub know_to_entry: i32,
    pub know_to_ready: i32,
    pub ready_n: i32,
    pub opens_away: i32,
    pub wedged_away: i32,
}

/// (src/game/digin.ts:163 DIGIN_DEFAULTS: a table nothing writes)
pub fn digin_defaults() -> crate::knobs::DiginK { crate::knobs::page_digin() }

/// (src/game/digin.ts:194 LS_KEY)
pub const LS_KEY: &str = "twopointsix.digin";

/// (src/game/digin.ts:195 KEPT: a table nothing writes)
pub fn kept() -> [String; 15] { [String::from("on"), String::from("stay"), String::from("cover"), String::from("brace"), String::from("doors"), String::from("radio"), String::from("deep"), String::from("far"), String::from("learn"), String::from("gather"), String::from("steady"), String::from("holdSet"), String::from("refuge"), String::from("keeper"), String::from("shutOpen")] }

/// (src/game/digin.ts:196 KEPT_NUM: a table nothing writes)
pub fn kept_num() -> [String; 3] { [String::from("wedges"), String::from("pace"), String::from("reachM")] }

/// (src/game/digin.ts:197 NUM_MAX: a table nothing writes)
pub fn num_max() -> crate::digin::NUMMAX { crate::digin::NUMMAX { wedges: 3.0, pace: 3.0, reach_m: 60.0 } }

/// (src/game/digin.ts:639 HAUL_NEAR)
pub const HAUL_NEAR: f64 = 0.9;

/// (src/game/digin.ts:639 HOME_REACH)
pub const HOME_REACH: f64 = 1.2;
