//! The types of `snatchout`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: SnatchOutSnap, src/game/snatchout.ts:57)
#[derive(Clone, Copy, Debug)]
pub struct SnatchOutSnap {
    pub lead: Option<f64>,
    pub led: bool,
    pub sent: bool,
    pub in_at: f64,
}
impl SnatchOutSnap {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SnatchOutSnap {
            lead: None,
            led: false,
            sent: false,
            in_at: 0.0,
        }
    }
}
impl Default for SnatchOutSnap { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: SnatchOut, src/game/snatchout.ts:59)
#[derive(Clone, Debug)]
pub struct SnatchOut {
    pub lead: Option<CharId>,
    pub led: bool,
    pub sent: bool,
    pub in_at: f64,
    pub jobs: JsMap<GuardId, OrdersAssignmentId>,
    pub coming: Option<String>,
    pub moving: bool,
    pub bounds: f64,
    pub last: JsMap<GuardId, Option<OrdersAssignmentId>>,
    pub told: JsSet<GuardId>,
    pub hot: Option<crate::snatchout::SnatchOutHot>,
    pub turned: JsMap<GuardId, crate::snatchout::SnatchOutTurnedValue>,
}
impl SnatchOut {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        SnatchOut {
            lead: None,
            led: false,
            sent: false,
            in_at: -1.0,
            jobs: JsMap::new(),
            coming: None,
            moving: false,
            bounds: 0.0,
            last: JsMap::new(),
            told: JsSet::new(),
            hot: None,
            turned: JsMap::new(),
        }
    }
}
impl Default for SnatchOut { fn default() -> Self { Self::blank() } }

/// (TypeScript: inline at src/game/snatchout.ts:74 SnatchOut.hot, src/game/snatchout.ts:74)
#[derive(Clone, Copy, Debug)]
pub struct SnatchOutHot {
    pub at: V3,
    pub until: f64,
}
impl SnatchOutHot {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SnatchOutHot {
            at: [0.0; 3],
            until: 0.0,
        }
    }
}
impl Default for SnatchOutHot { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/snatchout.ts:74 SnatchOut.turned, src/game/snatchout.ts:74)
#[derive(Clone, Copy, Debug)]
pub struct SnatchOutTurnedValue {
    pub watch: Option<V3>,
    pub face: Option<f64>,
}
impl SnatchOutTurnedValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SnatchOutTurnedValue {
            watch: None,
            face: None,
        }
    }
}
impl Default for SnatchOutTurnedValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/snatchout.ts:93 rows(row), ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1583)
#[derive(Clone, Debug)]
pub struct SnatchOutRowsRowOver {
    pub kind: Option<crate::snatchout::SnatchOutRowsRowOverKind>,
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
impl SnatchOutRowsRowOver {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SnatchOutRowsRowOver {
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
impl Default for SnatchOutRowsRowOver { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum SnatchOutRowsRowOverKind {
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
impl SnatchOutRowsRowOverKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            SnatchOutRowsRowOverKind::Cam => "cam",
            SnatchOutRowsRowOverKind::Guard => "guard",
            SnatchOutRowsRowOverKind::Breaker => "breaker",
            SnatchOutRowsRowOverKind::Door => "door",
            SnatchOutRowsRowOverKind::Light => "light",
            SnatchOutRowsRowOverKind::Body => "body",
            SnatchOutRowsRowOverKind::Objective => "objective",
            SnatchOutRowsRowOverKind::Held => "held",
            SnatchOutRowsRowOverKind::Wall => "wall",
            SnatchOutRowsRowOverKind::Switch => "switch",
            SnatchOutRowsRowOverKind::Charge => "charge",
            SnatchOutRowsRowOverKind::Cancel => "cancel",
        }
    }
}

/// (TypeScript: inline at src/game/snatchout.ts:147 mine(spec), src/game/snatchout.ts:147 mine(spec))
#[derive(Clone, Debug)]
pub struct SnatchOutMineSpec {
    pub post: Option<V3>,
    pub going: String,
    pub order: String,
    pub face: Option<f64>,
    pub there: String,
    pub t0: Option<f64>,
    pub follow: Option<f64>,
    pub watch: Option<V3>,
    pub door: Option<DoorId>,
    pub strafe: Option<bool>,
    pub stance: Option<crate::character::Stance>,
    pub upper: Option<crate::character::UpperMode>,
    pub stack_side: Option<f64>,
    pub move_: Option<crate::doortools::DoorToolsDoneItemLaneBackMove>,
    pub set_line: Option<String>,
    pub group: Option<f64>,
    pub set_said: Option<bool>,
    pub goal: Option<V3>,
    pub arrived: Option<bool>,
    pub held_set: Option<bool>,
    pub held_fire: Option<bool>,
    pub arr_t: Option<f64>,
    pub moved_t: Option<f64>,
    pub light: Option<bool>,
    pub low: Option<bool>,
    pub yield_t: Option<f64>,
    pub face_now: Option<f64>,
    pub search: Option<bool>,
    pub search_t: Option<f64>,
    pub via: Option<Vec<V3>>,
    pub anchor: Option<V3>,
    pub mark: Option<V3>,
    pub mark_t: Option<f64>,
    pub had: Option<V3>,
    pub back_n: Option<f64>,
    pub shy: Option<DoorId>,
    pub thru: Option<DoorId>,
    pub via_for: Option<V3>,
    pub via_t: Option<f64>,
    pub pace: Option<f64>,
    pub spare: Option<bool>,
    pub from: Option<V3>,
    pub foe: Option<GuardId>,
    pub crouch: Option<bool>,
    pub lean: Option<f64>,
    pub fit: Option<crate::roomfit::Fit>,
    pub tries: Option<crate::doortools::DoorToolsDoneItemLaneBackTries>,
}
impl SnatchOutMineSpec {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SnatchOutMineSpec {
            post: None,
            going: String::new(),
            order: String::new(),
            face: None,
            there: String::new(),
            t0: None,
            follow: None,
            watch: None,
            door: None,
            strafe: None,
            stance: None,
            upper: None,
            stack_side: None,
            move_: None,
            set_line: None,
            group: None,
            set_said: None,
            goal: None,
            arrived: None,
            held_set: None,
            held_fire: None,
            arr_t: None,
            moved_t: None,
            light: None,
            low: None,
            yield_t: None,
            face_now: None,
            search: None,
            search_t: None,
            via: None,
            anchor: None,
            mark: None,
            mark_t: None,
            had: None,
            back_n: None,
            shy: None,
            thru: None,
            via_for: None,
            via_t: None,
            pace: None,
            spare: None,
            from: None,
            foe: None,
            crouch: None,
            lean: None,
            fit: None,
            tries: None,
        }
    }
}
impl Default for SnatchOutMineSpec { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/snatchout.ts:242 objective ->, src/game/snatchout.ts:242)
#[derive(Clone, Debug)]
pub struct SnatchOutObjective {
    pub text: String,
    pub pos: Option<V3>,
}
impl SnatchOutObjective {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SnatchOutObjective {
            text: String::new(),
            pos: None,
        }
    }
}
impl Default for SnatchOutObjective { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/snatchout.ts:261 snatchOutRows(row), ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1583)
#[derive(Clone, Debug)]
pub struct SnatchOutRows2RowOver {
    pub kind: Option<crate::snatchout::SnatchOutRows2RowOverKind>,
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
impl SnatchOutRows2RowOver {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SnatchOutRows2RowOver {
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
impl Default for SnatchOutRows2RowOver { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum SnatchOutRows2RowOverKind {
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
impl SnatchOutRows2RowOverKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            SnatchOutRows2RowOverKind::Cam => "cam",
            SnatchOutRows2RowOverKind::Guard => "guard",
            SnatchOutRows2RowOverKind::Breaker => "breaker",
            SnatchOutRows2RowOverKind::Door => "door",
            SnatchOutRows2RowOverKind::Light => "light",
            SnatchOutRows2RowOverKind::Body => "body",
            SnatchOutRows2RowOverKind::Objective => "objective",
            SnatchOutRows2RowOverKind::Held => "held",
            SnatchOutRows2RowOverKind::Wall => "wall",
            SnatchOutRows2RowOverKind::Switch => "switch",
            SnatchOutRows2RowOverKind::Charge => "charge",
            SnatchOutRows2RowOverKind::Cancel => "cancel",
        }
    }
}
