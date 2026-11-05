//! The types of `workthings`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: WorkThing, src/game/workthings.ts:89)
#[derive(Clone, Debug)]
pub struct WorkThing {
    pub kind: crate::workthings::WorkThingKind,
    pub face: V3,
    pub n: V3,
    pub stand: Option<V3>,
    pub name: String,
    pub dev: Option<f64>,
}
impl WorkThing {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WorkThing {
            kind: crate::workthings::WorkThingKind::Breaker,
            face: [0.0; 3],
            n: [0.0; 3],
            stand: None,
            name: String::new(),
            dev: None,
        }
    }
}
impl Default for WorkThing { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum WorkThingKind {
    Breaker,
    Device,
    Objective,
}
impl WorkThingKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            WorkThingKind::Breaker => "breaker",
            WorkThingKind::Device => "device",
            WorkThingKind::Objective => "objective",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum WorkVerb {
    Drive,
    BreakerOff,
    BreakerOn,
    Defuse,
}
impl WorkVerb {
    pub const fn as_str(self) -> &'static str {
        match self {
            WorkVerb::Drive => "drive",
            WorkVerb::BreakerOff => "breakerOff",
            WorkVerb::BreakerOn => "breakerOn",
            WorkVerb::Defuse => "defuse",
        }
    }
}

/// (TypeScript: WorkJob, src/game/workthings.ts:93)
#[derive(Clone, Copy, Debug)]
pub struct WorkJob {
    pub verb: crate::workthings::WorkVerb,
    pub secs: f64,
    pub need: f64,
}
impl WorkJob {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WorkJob {
            verb: crate::workthings::WorkVerb::Drive,
            secs: 0.0,
            need: 0.0,
        }
    }
}
impl Default for WorkJob { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Job, src/game/workthings.ts:94)
#[derive(Clone, Debug)]
pub struct Job {
    pub thing: crate::workthings::WorkThing,
    pub verb: crate::workthings::WorkVerb,
    pub a: OrdersAssignmentId,
    pub t0: f64,
    pub last_t: f64,
}
impl Job {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Job {
            thing: crate::workthings::WorkThing::zeroed(),
            verb: crate::workthings::WorkVerb::Drive,
            a: OrdersAssignmentId(0),
            t0: 0.0,
            last_t: 0.0,
        }
    }
}
impl Default for Job { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Area, src/game/workthings.ts:100)
#[derive(Clone, Copy, Debug)]
pub struct Area {
    pub x0: f64,
    pub x1: f64,
    pub z0: f64,
    pub z1: f64,
}
impl Area {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Area {
            x0: 0.0,
            x1: 0.0,
            z0: 0.0,
            z1: 0.0,
        }
    }
}
impl Default for Area { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: BreakerErrand, src/game/workthings.ts:135)
#[derive(Clone, Copy, Debug)]
pub struct BreakerErrand {
    pub goal: V3,
    pub mid: V3,
    pub at: V3,
}
impl BreakerErrand {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BreakerErrand {
            goal: [0.0; 3],
            mid: [0.0; 3],
            at: [0.0; 3],
        }
    }
}
impl Default for BreakerErrand { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: WorkThings, src/game/workthings.ts:154)
#[derive(Clone, Debug)]
pub struct WorkThings {
    pub row: OrdersOrderDefId,
    pub jobs: JsMap<GuardId, crate::workthings::Job>,
    pub hands: JsMap<CharId, crate::workthings::WorkThingsHandsValue>,
    pub cache: Option<crate::workthings::WorkThingsCache>,
    pub asking: bool,
    pub log: Vec<crate::workthings::WorkThingsLogItem>,
}
impl WorkThings {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        WorkThings {
            row: OrdersOrderDefId(0),
            jobs: JsMap::new(),
            hands: JsMap::new(),
            cache: None,
            asking: false,
            log: Vec::new(),
        }
    }
}
impl Default for WorkThings { fn default() -> Self { Self::blank() } }

/// (TypeScript: inline at src/game/workthings.ts:160 WorkThings.hands, src/game/workthings.ts:160)
#[derive(Clone, Copy, Debug)]
pub struct WorkThingsHandsValue {
    pub push: crate::character::DoorPush,
    pub until: f64,
    pub seen: f64,
    pub verb: crate::workthings::WorkVerb,
    pub t0: f64,
    pub need: f64,
}
impl WorkThingsHandsValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WorkThingsHandsValue {
            push: crate::character::DoorPush::zeroed(),
            until: 0.0,
            seen: 0.0,
            verb: crate::workthings::WorkVerb::Drive,
            t0: 0.0,
            need: 0.0,
        }
    }
}
impl Default for WorkThingsHandsValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/workthings.ts:161 WorkThings.cache, src/game/workthings.ts:161)
#[derive(Clone, Debug)]
pub struct WorkThingsCache {
    pub key: String,
    pub t: f64,
    pub list: Vec<crate::workthings::WorkThing>,
}
impl WorkThingsCache {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WorkThingsCache {
            key: String::new(),
            t: 0.0,
            list: Vec::new(),
        }
    }
}
impl Default for WorkThingsCache { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/workthings.ts:169 WorkThings.log, src/game/workthings.ts:169)
#[derive(Clone, Debug)]
pub struct WorkThingsLogItem {
    pub t: f64,
    pub who: String,
    pub what: String,
}
impl WorkThingsLogItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WorkThingsLogItem {
            t: 0.0,
            who: String::new(),
            what: String::new(),
        }
    }
}
impl Default for WorkThingsLogItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/workthings.ts:136 errands, src/game/workthings.ts:136)
#[derive(Clone, Debug)]
pub struct WorkthingsModErrandsValue {
    pub t: f64,
    pub key: String,
    pub e: Option<crate::workthings::BreakerErrand>,
}
impl WorkthingsModErrandsValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WorkthingsModErrandsValue {
            t: 0.0,
            key: String::new(),
            e: None,
        }
    }
}
impl Default for WorkthingsModErrandsValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/workthings.ts:115 panelFace ->, src/game/workthings.ts:115)
#[derive(Clone, Copy, Debug)]
pub struct PanelFace {
    pub face: V3,
    pub n: V3,
}
impl PanelFace {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PanelFace {
            face: [0.0; 3],
            n: [0.0; 3],
        }
    }
}
impl Default for PanelFace { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/workthings.ts:165 askAside(own), src/game/workthings.ts:165)
#[derive(Clone, Debug)]
pub struct WorkThingsAskAsideOwn {
    pub c: Option<crate::workthings::WorkThingsCache>,
}
impl WorkThingsAskAsideOwn {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WorkThingsAskAsideOwn {
            c: None,
        }
    }
}
impl Default for WorkThingsAskAsideOwn { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at quiet WorkThingsAskAsideOwn.c, quiet WorkThingsAskAsideOwn.c)
#[derive(Clone, Copy, Debug)]
pub struct WorkThingsAskAsideOwnC {
}
impl WorkThingsAskAsideOwnC {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WorkThingsAskAsideOwnC {
        }
    }
}
impl Default for WorkThingsAskAsideOwnC { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/workthings.ts:227 near ->, src/game/workthings.ts:227)
#[derive(Clone, Debug)]
pub struct WorkThingsNear {
    pub d: f64,
    pub target: crate::orders::OrderTarget,
}
impl WorkThingsNear {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WorkThingsNear {
            d: 0.0,
            target: crate::orders::OrderTarget::Floor(crate::orders::OrderTargetFloor::zeroed()),
        }
    }
}
impl Default for WorkThingsNear { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: a union)
#[derive(Clone, Debug)]
pub enum WorkThingsOn {
    Floor(crate::orders::OrderTargetFloor),
    Door(crate::orders::OrderTargetDoor),
    None(crate::orders::OrderTargetNone),
    Foe(crate::orders::OrderTargetFoe),
}

/// (TypeScript: a union)
#[derive(Clone, Debug)]
pub enum WorkThingsIssue {
    Text(String),
    Arm1(()),
}

/// (TypeScript: inline at src/game/workthings.ts:346 onDevice ->, src/game/workthings.ts:346)
#[derive(Clone, Copy, Debug)]
pub struct WorkThingsOnDevice {
    pub gd: GuardId,
    pub on: bool,
}
impl WorkThingsOnDevice {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WorkThingsOnDevice {
            gd: GuardId(0),
            on: false,
        }
    }
}
impl Default for WorkThingsOnDevice { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/workthings.ts:357 workNear ->, src/game/workthings.ts:357)
#[derive(Clone, Debug)]
pub struct WorkNear {
    pub d: f64,
    pub target: crate::orders::OrderTarget,
}
impl WorkNear {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WorkNear {
            d: 0.0,
            target: crate::orders::OrderTarget::Floor(crate::orders::OrderTargetFloor::zeroed()),
        }
    }
}
impl Default for WorkNear { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: a union)
#[derive(Clone, Debug)]
pub enum WorkOn {
    Floor(crate::orders::OrderTargetFloor),
    Door(crate::orders::OrderTargetDoor),
    None(crate::orders::OrderTargetNone),
    Foe(crate::orders::OrderTargetFoe),
}

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct WorkthingsMod {
    /// (src/game/workthings.ts:136 errands)
    pub errands: Option<crate::workthings::WorkthingsModErrandsValue>,
}
impl WorkthingsMod {
    pub fn blank() -> Self {
        WorkthingsMod {
            errands: None,
        }
    }
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct WorkthingsStats {
    pub given: i32,
    pub done: i32,
    pub moot: i32,
}

/// (src/game/workthings.ts:84 WORK_ORDER)
pub const WORK_ORDER: &str = "work";
