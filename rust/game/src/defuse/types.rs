//! The types of `defuse`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum ChargeState {
    Safe,
    Armed,
    Blown,
}
impl ChargeState {
    pub const fn as_str(self) -> &'static str {
        match self {
            ChargeState::Safe => "safe",
            ChargeState::Armed => "armed",
            ChargeState::Blown => "blown",
        }
    }
}

/// (TypeScript: DefuseSnap, src/game/defuse.ts:50)
#[derive(Clone, Debug)]
pub struct DefuseSnap {
    pub ran: f64,
    pub st: Vec<crate::defuse::ChargeState>,
    pub by: Option<Vec<Option<String>>>,
    pub end: Option<crate::defuse::DefuseSnapEnd>,
    pub sam: Option<crate::defuse::DefuseSnapSam>,
}
impl DefuseSnap {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DefuseSnap {
            ran: 0.0,
            st: Vec::new(),
            by: None,
            end: None,
            sam: None,
        }
    }
}
impl Default for DefuseSnap { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum DefuseSnapEnd {
    Failed,
    Safe,
}
impl DefuseSnapEnd {
    pub const fn as_str(self) -> &'static str {
        match self {
            DefuseSnapEnd::Failed => "failed",
            DefuseSnapEnd::Safe => "safe",
        }
    }
}

/// (TypeScript: inline at src/game/defuse.ts:50 DefuseSnap.sam, src/game/defuse.ts:50)
#[derive(Clone, Copy, Debug)]
pub struct DefuseSnapSam {
    pub dev: f64,
    pub ran: f64,
}
impl DefuseSnapSam {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DefuseSnapSam {
            dev: 0.0,
            ran: 0.0,
        }
    }
}
impl Default for DefuseSnapSam { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Defuse, src/game/defuse.ts:60)
#[derive(Clone, Debug)]
pub struct Defuse {
    pub states: Vec<crate::defuse::ChargeState>,
    pub safe_by: Vec<Option<String>>,
    pub end: Option<crate::defuse::DefuseEnd>,
    pub t0: f64,
    pub t_end: f64,
    pub log: Vec<crate::defuse::DefuseLogItem>,
    pub run: Option<u32>,
    pub from: Option<V3>,
    pub last_order: Option<u32>,
    pub sam: Option<crate::defuse::DefuseSam>,
    pub offered: f64,
    pub marker: Option<crate::game::Interactable>,
    pub beat: f64,
    pub won_said: bool,
    pub took_said: bool,
}
impl Defuse {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        Defuse {
            states: Vec::new(),
            safe_by: Vec::new(),
            end: None,
            t0: f64::NAN,
            t_end: f64::NAN,
            log: Vec::new(),
            run: None,
            from: None,
            last_order: None,
            sam: None,
            offered: -1.0,
            marker: None,
            beat: -1.0,
            won_said: false,
            took_said: false,
        }
    }
}
impl Default for Defuse { fn default() -> Self { Self::blank() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum DefuseEnd {
    Failed,
    Safe,
    Over,
}
impl DefuseEnd {
    pub const fn as_str(self) -> &'static str {
        match self {
            DefuseEnd::Failed => "failed",
            DefuseEnd::Safe => "safe",
            DefuseEnd::Over => "over",
        }
    }
}

/// (TypeScript: inline at src/game/defuse.ts:71 Defuse.log, src/game/defuse.ts:71)
#[derive(Clone, Debug)]
pub struct DefuseLogItem {
    pub t: f64,
    pub what: String,
}
impl DefuseLogItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DefuseLogItem {
            t: 0.0,
            what: String::new(),
        }
    }
}
impl Default for DefuseLogItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at quiet Defuse.run, quiet Defuse.run)
#[derive(Clone, Copy, Debug)]
pub struct DefuseRun {
}
impl DefuseRun {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DefuseRun {
        }
    }
}
impl Default for DefuseRun { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at quiet Defuse.lastOrder, quiet Defuse.lastOrder)
#[derive(Clone, Copy, Debug)]
pub struct DefuseLastOrder {
}
impl DefuseLastOrder {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DefuseLastOrder {
        }
    }
}
impl Default for DefuseLastOrder { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/defuse.ts:75 Defuse.sam, src/game/defuse.ts:75)
#[derive(Clone, Copy, Debug)]
pub struct DefuseSam {
    pub dev: f64,
    pub t0: f64,
    pub at: V3,
}
impl DefuseSam {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DefuseSam {
            dev: 0.0,
            t0: 0.0,
            at: [0.0; 3],
        }
    }
}
impl Default for DefuseSam { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/defuse.ts:105 armed ->, src/game/defuse.ts:105)
#[derive(Clone, Debug)]
pub struct DefuseArmedItem {
    pub i: f64,
    pub face: V3,
    pub n: V3,
    pub name: String,
}
impl DefuseArmedItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DefuseArmedItem {
            i: 0.0,
            face: [0.0; 3],
            n: [0.0; 3],
            name: String::new(),
        }
    }
}
impl Default for DefuseArmedItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: a union)
#[derive(Clone, Copy, Debug)]
pub enum DefuseMakeSafeBy {
    Guard(GuardId),
    Player,
}

/// (TypeScript: inline at src/game/defuse.ts:185 samRows(row), ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1583)
#[derive(Clone, Debug)]
pub struct DefuseSamRowsRowOver {
    pub kind: Option<crate::defuse::DefuseSamRowsRowOverKind>,
    pub pos: Option<V3>,
    pub line1: Option<String>,
    pub line2: Option<String>,
    pub in_reach: Option<bool>,
    pub off: Option<bool>,
    pub progress: Option<f64>,
    pub target: Option<FixtureId>,
    pub door: Option<DoorId>,
    pub guard: Option<GuardId>,
    pub verb: Option<crate::game::Verb>,
    pub running: Option<bool>,
    pub close: Option<crate::close::CloseKind>,
    pub past: Option<bool>,
    pub framed: Option<bool>,
}
impl DefuseSamRowsRowOver {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DefuseSamRowsRowOver {
            kind: None,
            pos: None,
            line1: None,
            line2: None,
            in_reach: None,
            off: None,
            progress: None,
            target: None,
            door: None,
            guard: None,
            verb: None,
            running: None,
            close: None,
            past: None,
            framed: None,
        }
    }
}
impl Default for DefuseSamRowsRowOver { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum DefuseSamRowsRowOverKind {
    Cam,
    Guard,
    Breaker,
    Door,
    Light,
    Body,
    Objective,
    Held,
    Wall,
    Switch,
    Charge,
    Cancel,
}
impl DefuseSamRowsRowOverKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            DefuseSamRowsRowOverKind::Cam => "cam",
            DefuseSamRowsRowOverKind::Guard => "guard",
            DefuseSamRowsRowOverKind::Breaker => "breaker",
            DefuseSamRowsRowOverKind::Door => "door",
            DefuseSamRowsRowOverKind::Light => "light",
            DefuseSamRowsRowOverKind::Body => "body",
            DefuseSamRowsRowOverKind::Objective => "objective",
            DefuseSamRowsRowOverKind::Held => "held",
            DefuseSamRowsRowOverKind::Wall => "wall",
            DefuseSamRowsRowOverKind::Switch => "switch",
            DefuseSamRowsRowOverKind::Charge => "charge",
            DefuseSamRowsRowOverKind::Cancel => "cancel",
        }
    }
}

/// (TypeScript: inline at src/game/defuse.ts:218 samJob ->, src/game/defuse.ts:218)
#[derive(Clone, Copy, Debug)]
pub struct DefuseSamJob {
    pub p: V3,
    pub n: V3,
    pub at: V3,
}
impl DefuseSamJob {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DefuseSamJob {
            p: [0.0; 3],
            n: [0.0; 3],
            at: [0.0; 3],
        }
    }
}
impl Default for DefuseSamJob { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/defuse.ts:230 objective ->, src/game/defuse.ts:230)
#[derive(Clone, Debug)]
pub struct DefuseObjective {
    pub text: String,
    pub pos: Option<V3>,
}
impl DefuseObjective {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DefuseObjective {
            text: String::new(),
            pos: None,
        }
    }
}
impl Default for DefuseObjective { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/defuse.ts:285 defuseRows(row), ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1583)
#[derive(Clone, Debug)]
pub struct DefuseRowsRowOver {
    pub kind: Option<crate::defuse::DefuseRowsRowOverKind>,
    pub pos: Option<V3>,
    pub line1: Option<String>,
    pub line2: Option<String>,
    pub in_reach: Option<bool>,
    pub off: Option<bool>,
    pub progress: Option<f64>,
    pub target: Option<FixtureId>,
    pub door: Option<DoorId>,
    pub guard: Option<GuardId>,
    pub verb: Option<crate::game::Verb>,
    pub running: Option<bool>,
    pub close: Option<crate::close::CloseKind>,
    pub past: Option<bool>,
    pub framed: Option<bool>,
}
impl DefuseRowsRowOver {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DefuseRowsRowOver {
            kind: None,
            pos: None,
            line1: None,
            line2: None,
            in_reach: None,
            off: None,
            progress: None,
            target: None,
            door: None,
            guard: None,
            verb: None,
            running: None,
            close: None,
            past: None,
            framed: None,
        }
    }
}
impl Default for DefuseRowsRowOver { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum DefuseRowsRowOverKind {
    Cam,
    Guard,
    Breaker,
    Door,
    Light,
    Body,
    Objective,
    Held,
    Wall,
    Switch,
    Charge,
    Cancel,
}
impl DefuseRowsRowOverKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            DefuseRowsRowOverKind::Cam => "cam",
            DefuseRowsRowOverKind::Guard => "guard",
            DefuseRowsRowOverKind::Breaker => "breaker",
            DefuseRowsRowOverKind::Door => "door",
            DefuseRowsRowOverKind::Light => "light",
            DefuseRowsRowOverKind::Body => "body",
            DefuseRowsRowOverKind::Objective => "objective",
            DefuseRowsRowOverKind::Held => "held",
            DefuseRowsRowOverKind::Wall => "wall",
            DefuseRowsRowOverKind::Switch => "switch",
            DefuseRowsRowOverKind::Charge => "charge",
            DefuseRowsRowOverKind::Cancel => "cancel",
        }
    }
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct DefuseStats {
    pub started: i32,
    pub safe: i32,
    pub by_sam: i32,
    pub blown: i32,
    pub won: i32,
    pub failed: i32,
}
