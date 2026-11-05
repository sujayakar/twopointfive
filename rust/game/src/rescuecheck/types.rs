//! The types of `rescuecheck`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Phase {
    Out,
    Back,
    There,
}
impl Phase {
    pub const fn as_str(self) -> &'static str {
        match self {
            Phase::Out => "out",
            Phase::Back => "back",
            Phase::There => "there",
        }
    }
}

/// (TypeScript: Out, src/game/rescuecheck.ts:73)
#[derive(Clone, Debug)]
pub struct Out {
    pub gd: GuardId,
    pub about: GuardId,
    pub goal: V3,
    pub from: V3,
    pub phase: crate::rescuecheck::Phase,
    pub there_t: f64,
    pub tries: f64,
    pub step_t: f64,
    pub heard0: f64,
    pub calm: crate::powerout::Calm,
}
impl Out {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Out {
            gd: GuardId(0),
            about: GuardId(0),
            goal: [0.0; 3],
            from: [0.0; 3],
            phase: crate::rescuecheck::Phase::Out,
            there_t: 0.0,
            tries: 0.0,
            step_t: 0.0,
            heard0: 0.0,
            calm: crate::powerout::Calm::zeroed(),
        }
    }
}
impl Default for Out { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: RescueCheckEvent, src/game/rescuecheck.ts:74)
#[derive(Clone, Debug)]
pub struct RescueCheckEvent {
    pub t: f64,
    pub what: crate::rescuecheck::RescueCheckEventWhat,
    pub who: String,
    pub why: String,
}
impl RescueCheckEvent {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RescueCheckEvent {
            t: 0.0,
            what: crate::rescuecheck::RescueCheckEventWhat::Lost,
            who: String::new(),
            why: String::new(),
        }
    }
}
impl Default for RescueCheckEvent { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum RescueCheckEventWhat {
    Lost,
    Home,
    There,
    Word,
    Found,
    Kept,
    Nobody,
    Empty,
    Check,
    Asks,
    Abort,
    Goes,
}
impl RescueCheckEventWhat {
    pub const fn as_str(self) -> &'static str {
        match self {
            RescueCheckEventWhat::Lost => "lost",
            RescueCheckEventWhat::Home => "home",
            RescueCheckEventWhat::There => "there",
            RescueCheckEventWhat::Word => "word",
            RescueCheckEventWhat::Found => "found",
            RescueCheckEventWhat::Kept => "kept",
            RescueCheckEventWhat::Nobody => "nobody",
            RescueCheckEventWhat::Empty => "empty",
            RescueCheckEventWhat::Check => "check",
            RescueCheckEventWhat::Asks => "asks",
            RescueCheckEventWhat::Abort => "abort",
            RescueCheckEventWhat::Goes => "goes",
        }
    }
}

/// (TypeScript: inline at src/game/rescuecheck.ts:75 Cast, src/game/rescue.ts:863)
#[derive(Clone, Copy, Debug)]
pub struct CastItem {
    pub gd: GuardId,
    pub role: crate::rescue::Role,
    pub in_room: bool,
    pub post: V3,
    pub learned_at: f64,
    pub released: bool,
    pub crewed: bool,
}
impl CastItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CastItem {
            gd: GuardId(0),
            role: crate::rescue::Role::Guard,
            in_room: false,
            post: [0.0; 3],
            learned_at: 0.0,
            released: false,
            crewed: false,
        }
    }
}
impl Default for CastItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: State, src/game/rescuecheck.ts:76)
#[derive(Clone, Debug)]
pub struct State {
    pub next_at: f64,
    pub n: f64,
    pub stage: crate::rescuecheck::StateStage,
    pub t0: f64,
    pub caller: Option<GuardId>,
    pub about: Option<GuardId>,
    pub known: JsSet<GuardId>,
    pub where_: JsMap<GuardId, V3>,
    pub worded: JsMap<GuardId, crate::rescuecheck::StateWordedValue>,
    pub kept: JsSet<GuardId>,
    pub out: Option<crate::rescuecheck::Out>,
    pub events: Vec<crate::rescuecheck::RescueCheckEvent>,
}
impl State {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        State {
            next_at: 0.0,
            n: 0.0,
            stage: crate::rescuecheck::StateStage::Idle,
            t0: 0.0,
            caller: None,
            about: None,
            known: JsSet::new(),
            where_: JsMap::new(),
            worded: JsMap::new(),
            kept: JsSet::new(),
            out: None,
            events: Vec::new(),
        }
    }
}
impl Default for State { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum StateStage {
    Idle,
    Called,
    Asked,
}
impl StateStage {
    pub const fn as_str(self) -> &'static str {
        match self {
            StateStage::Idle => "idle",
            StateStage::Called => "called",
            StateStage::Asked => "asked",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum StateWordedValue {
    Door,
    Noise,
}
impl StateWordedValue {
    pub const fn as_str(self) -> &'static str {
        match self {
            StateWordedValue::Door => "door",
            StateWordedValue::Noise => "noise",
        }
    }
}

/// (TypeScript: RescueCheckSnap, src/game/rescuecheck.ts:230)
#[derive(Clone, Debug)]
pub struct RescueCheckSnap {
    pub next_at: f64,
    pub n: f64,
    pub stage: crate::rescuecheck::RescueCheckSnapStage,
    pub t0: f64,
    pub caller: f64,
    pub about: f64,
    pub known: Vec<f64>,
    pub where_: Vec<(f64, V3)>,
    pub worded: Vec<(f64, crate::rescuecheck::RescueCheckSnapWordedItem1)>,
    pub kept: Vec<f64>,
    pub out: Option<crate::rescuecheck::RescueCheckSnapOut>,
}
impl RescueCheckSnap {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RescueCheckSnap {
            next_at: 0.0,
            n: 0.0,
            stage: crate::rescuecheck::RescueCheckSnapStage::Idle,
            t0: 0.0,
            caller: 0.0,
            about: 0.0,
            known: Vec::new(),
            where_: Vec::new(),
            worded: Vec::new(),
            kept: Vec::new(),
            out: None,
        }
    }
}
impl Default for RescueCheckSnap { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum RescueCheckSnapStage {
    Idle,
    Called,
    Asked,
}
impl RescueCheckSnapStage {
    pub const fn as_str(self) -> &'static str {
        match self {
            RescueCheckSnapStage::Idle => "idle",
            RescueCheckSnapStage::Called => "called",
            RescueCheckSnapStage::Asked => "asked",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum RescueCheckSnapWordedItem1 {
    Door,
    Noise,
}
impl RescueCheckSnapWordedItem1 {
    pub const fn as_str(self) -> &'static str {
        match self {
            RescueCheckSnapWordedItem1::Door => "door",
            RescueCheckSnapWordedItem1::Noise => "noise",
        }
    }
}

/// (TypeScript: inline at src/game/rescuecheck.ts:231 RescueCheckSnap.out, src/game/rescuecheck.ts:231)
#[derive(Clone, Debug)]
pub struct RescueCheckSnapOut {
    pub i: f64,
    pub about: f64,
    pub goal: V3,
    pub from: V3,
    pub phase: crate::rescuecheck::Phase,
    pub there_t: f64,
    pub tries: f64,
    pub step_t: f64,
    pub heard0: f64,
    pub calm: crate::powerout::CalmSnap,
}
impl RescueCheckSnapOut {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RescueCheckSnapOut {
            i: 0.0,
            about: 0.0,
            goal: [0.0; 3],
            from: [0.0; 3],
            phase: crate::rescuecheck::Phase::Out,
            there_t: 0.0,
            tries: 0.0,
            step_t: 0.0,
            heard0: 0.0,
            calm: crate::powerout::CalmSnap::zeroed(),
        }
    }
}
impl Default for RescueCheckSnapOut { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum EventWhat {
    Lost,
    Home,
    There,
    Word,
    Found,
    Kept,
    Nobody,
    Empty,
    Check,
    Asks,
    Abort,
    Goes,
}
impl EventWhat {
    pub const fn as_str(self) -> &'static str {
        match self {
            EventWhat::Lost => "lost",
            EventWhat::Home => "home",
            EventWhat::There => "there",
            EventWhat::Word => "word",
            EventWhat::Found => "found",
            EventWhat::Kept => "kept",
            EventWhat::Nobody => "nobody",
            EventWhat::Empty => "empty",
            EventWhat::Check => "check",
            EventWhat::Asks => "asks",
            EventWhat::Abort => "abort",
            EventWhat::Goes => "goes",
        }
    }
}

/// (TypeScript: inline at src/game/rescuecheck.ts:114 goer ->, src/game/rescuecheck.ts:114)
#[derive(Clone, Copy, Debug)]
pub struct Goer {
    pub d: RoomfightDefId,
    pub gd: GuardId,
}
impl Goer {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Goer {
            d: RoomfightDefId(0),
            gd: GuardId(0),
        }
    }
}
impl Default for Goer { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct RescuecheckMod {
    /// (src/game/rescuecheck.ts:79 STATES)
    pub states: Option<crate::rescuecheck::State>,
}
impl RescuecheckMod {
    pub fn blank() -> Self {
        RescuecheckMod {
            states: None,
        }
    }
}
