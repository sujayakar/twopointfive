//! The types of `squadthrow`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// a throw search's test: a frag thrown so catches nobody of ours; a stun always passes (squadthrow.ts Lob.clean)
#[derive(Clone, Debug)]
pub enum LobClean {
    Plan { kind: crate::squadthrow::SquadThrowKind, at: V3, fresh: bool },
    LookAgain { kind: crate::squadthrow::SquadThrowKind, at: V3, gd: GuardId },
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum SquadThrowKind {
    Frag,
    Stun,
}
impl SquadThrowKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            SquadThrowKind::Frag => "frag",
            SquadThrowKind::Stun => "stun",
        }
    }
}

/// (TypeScript: Arc, src/game/squadthrow.ts:119)
#[derive(Clone, Debug)]
pub struct Arc {
    pub from: V3,
    pub sol: crate::throwables::ThrowSolution,
    pub rest: V3,
}
impl Arc {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Arc {
            from: [0.0; 3],
            sol: crate::throwables::ThrowSolution::zeroed(),
            rest: [0.0; 3],
        }
    }
}
impl Default for Arc { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: SquadThrower, src/game/squadthrow.ts:121)
#[derive(Clone, Debug)]
pub struct SquadThrower {
    pub from: V3,
    pub sol: crate::throwables::ThrowSolution,
    pub rest: V3,
    pub gd: GuardId,
    pub post: Option<V3>,
}
impl SquadThrower {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SquadThrower {
            from: [0.0; 3],
            sol: crate::throwables::ThrowSolution::zeroed(),
            rest: [0.0; 3],
            gd: GuardId(0),
            post: None,
        }
    }
}
impl Default for SquadThrower { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Walk, src/game/squadthrow.ts:124)
#[derive(Clone, Debug)]
pub struct Walk {
    pub kind: crate::squadthrow::SquadThrowKind,
    pub at: V3,
    pub post: V3,
    pub arc: crate::squadthrow::Arc,
    pub spent: f64,
    pub last_t: f64,
    pub on_t: f64,
    pub thrown_t: f64,
    pub again: f64,
    pub held: Option<crate::squadthrow::Arc>,
    pub held_at: Option<V3>,
    pub held_t: Option<f64>,
    pub line: Vec<V3>,
    pub mates: Vec<GuardId>,
    pub gd: GuardId,
    pub t: crate::orders::OrderTarget,
    pub a: Option<OrdersAssignmentId>,
    pub home: V3,
    pub home_yaw: f64,
    pub back: Option<crate::squadthrow::WalkBack>,
}
impl Walk {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Walk {
            kind: crate::squadthrow::SquadThrowKind::Frag,
            at: [0.0; 3],
            post: [0.0; 3],
            arc: crate::squadthrow::Arc::zeroed(),
            spent: 0.0,
            last_t: 0.0,
            on_t: 0.0,
            thrown_t: 0.0,
            again: 0.0,
            held: None,
            held_at: None,
            held_t: None,
            line: Vec::new(),
            mates: Vec::new(),
            gd: GuardId(0),
            t: crate::orders::OrderTarget::Floor(crate::orders::OrderTargetFloor::zeroed()),
            a: None,
            home: [0.0; 3],
            home_yaw: 0.0,
            back: None,
        }
    }
}
impl Default for Walk { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/squadthrow.ts:128 Walk.back, src/game/squadthrow.ts:128 Walk.back)
#[derive(Clone, Debug)]
pub struct WalkBack {
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
impl WalkBack {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WalkBack {
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
impl Default for WalkBack { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Lent, src/game/squadthrow.ts:128)
#[derive(Clone, Debug)]
pub struct Lent {
    pub mates: Vec<GuardId>,
    pub gd: GuardId,
    pub t: crate::orders::OrderTarget,
    pub a: Option<OrdersAssignmentId>,
    pub home: V3,
    pub home_yaw: f64,
    pub back: Option<crate::squadthrow::LentBack>,
}
impl Lent {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Lent {
            mates: Vec::new(),
            gd: GuardId(0),
            t: crate::orders::OrderTarget::Floor(crate::orders::OrderTargetFloor::zeroed()),
            a: None,
            home: [0.0; 3],
            home_yaw: 0.0,
            back: None,
        }
    }
}
impl Default for Lent { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/squadthrow.ts:128 Lent.back, src/game/squadthrow.ts:128 Lent.back)
#[derive(Clone, Debug)]
pub struct LentBack {
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
impl LentBack {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LentBack {
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
impl Default for LentBack { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Scratch, src/game/squadthrow.ts:129)
#[derive(Clone, Debug)]
pub struct Scratch {
    pub n: f64,
    pub gen_: f64,
    pub seen: Vec<i32>,
    pub leaf: Vec<i32>,
    pub dep: Vec<u8>,
    pub who: Vec<u8>,
    pub q: Vec<i32>,
}
impl Scratch {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Scratch {
            n: 0.0,
            gen_: 0.0,
            seen: Vec::new(),
            leaf: Vec::new(),
            dep: Vec::new(),
            who: Vec::new(),
            q: Vec::new(),
        }
    }
}
impl Default for Scratch { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: State, src/game/squadthrow.ts:130)
#[derive(Clone, Debug)]
pub struct State {
    pub last: JsMap<GuardId, f64>,
    pub noted: bool,
    pub shown: Option<crate::squadthrow::StateShown>,
    pub warn: Option<crate::squadthrow::StateWarn>,
    pub warn_frag: Option<crate::squadthrow::StateWarnFrag>,
    pub caught: Option<crate::squadthrow::StateCaught>,
    pub asked: JsMap<crate::squadthrow::SquadThrowKind, crate::squadthrow::StateAskedValue>,
    pub walks: JsMap<GuardId, crate::squadthrow::Walk>,
    pub kept: JsMap<String, crate::squadthrow::StateKeptValue>,
    pub scratch: Option<crate::squadthrow::Scratch>,
}
impl State {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        State {
            last: JsMap::new(),
            noted: false,
            shown: None,
            warn: None,
            warn_frag: None,
            caught: None,
            asked: JsMap::new(),
            walks: JsMap::new(),
            kept: JsMap::new(),
            scratch: None,
        }
    }
}
impl Default for State { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/squadthrow.ts:130 State.shown, src/game/squadthrow.ts:130)
#[derive(Clone, Debug)]
pub struct StateShown {
    pub t: f64,
    pub text: String,
}
impl StateShown {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StateShown {
            t: 0.0,
            text: String::new(),
        }
    }
}
impl Default for StateShown { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/squadthrow.ts:130 State.warn, src/game/squadthrow.ts:130)
#[derive(Clone, Debug)]
pub struct StateWarn {
    pub key: String,
    pub text: String,
}
impl StateWarn {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StateWarn {
            key: String::new(),
            text: String::new(),
        }
    }
}
impl Default for StateWarn { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/squadthrow.ts:130 State.warnFrag, src/game/squadthrow.ts:130)
#[derive(Clone, Debug)]
pub struct StateWarnFrag {
    pub key: String,
    pub text: String,
}
impl StateWarnFrag {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StateWarnFrag {
            key: String::new(),
            text: String::new(),
        }
    }
}
impl Default for StateWarnFrag { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/squadthrow.ts:130 State.caught, src/game/squadthrow.ts:130)
#[derive(Clone, Debug)]
pub struct StateCaught {
    pub key: String,
    pub t: f64,
    pub who: Option<String>,
}
impl StateCaught {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StateCaught {
            key: String::new(),
            t: 0.0,
            who: None,
        }
    }
}
impl Default for StateCaught { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/squadthrow.ts:131 State.asked, src/game/squadthrow.ts:131)
#[derive(Clone, Debug)]
pub struct StateAskedValue {
    pub key: String,
    pub men: Vec<GuardId>,
}
impl StateAskedValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StateAskedValue {
            key: String::new(),
            men: Vec::new(),
        }
    }
}
impl Default for StateAskedValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/squadthrow.ts:132 State.kept, src/game/squadthrow.ts:132)
#[derive(Clone, Debug)]
pub struct StateKeptValue {
    pub key: String,
    pub t: f64,
    pub th: Option<crate::squadthrow::SquadThrower>,
    pub line: Option<Vec<V3>>,
}
impl StateKeptValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StateKeptValue {
            key: String::new(),
            t: 0.0,
            th: None,
            line: None,
        }
    }
}
impl Default for StateKeptValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: SquadThrowKept, src/game/squadthrow.ts:138)
#[derive(Clone, Debug)]
pub struct SquadThrowKept {
    pub kept: JsMap<String, crate::squadthrow::StateKeptValue>,
    pub caught: Option<crate::squadthrow::StateCaught>,
    pub scratch: Option<crate::squadthrow::Scratch>,
    pub col: crate::scene_collision::AsideOwn,
    pub orders: crate::orders::SquadOrdersAskAsideOwn,
    pub duck: Option<crate::squadthrow::SquadThrowKeptDuck>,
}
impl SquadThrowKept {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SquadThrowKept {
            kept: JsMap::new(),
            caught: None,
            scratch: None,
            col: crate::scene_collision::AsideOwn::zeroed(),
            orders: crate::orders::SquadOrdersAskAsideOwn::zeroed(),
            duck: None,
        }
    }
}
impl Default for SquadThrowKept { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at quiet SquadThrowKept.col, src/game/squadthrow.ts:138)
#[derive(Clone, Copy, Debug)]
pub struct SquadThrowKeptCol {
    pub s: Option<crate::squadthrow::SquadThrowKeptColS>,
    pub q: Option<crate::squadthrow::SquadThrowKeptColQ>,
}
impl SquadThrowKeptCol {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SquadThrowKeptCol {
            s: None,
            q: None,
        }
    }
}
impl Default for SquadThrowKeptCol { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/squadthrow.ts:138 SquadThrowKeptCol.s, src/game/squadthrow.ts:138 SquadThrowKeptCol.s)
#[derive(Clone, Copy, Debug)]
pub struct SquadThrowKeptColS {
}
impl SquadThrowKeptColS {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SquadThrowKeptColS {
        }
    }
}
impl Default for SquadThrowKeptColS { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/squadthrow.ts:138 SquadThrowKeptCol.q, src/game/squadthrow.ts:138 SquadThrowKeptCol.q)
#[derive(Clone, Copy, Debug)]
pub struct SquadThrowKeptColQ {
}
impl SquadThrowKeptColQ {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SquadThrowKeptColQ {
        }
    }
}
impl Default for SquadThrowKeptColQ { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at quiet SquadThrowKept.orders, src/game/squadthrow.ts:138)
#[derive(Clone, Copy, Debug)]
pub struct SquadThrowKeptOrders {
    pub r: Option<crate::squadthrow::SquadThrowKeptOrdersR>,
}
impl SquadThrowKeptOrders {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SquadThrowKeptOrders {
            r: None,
        }
    }
}
impl Default for SquadThrowKeptOrders { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/squadthrow.ts:138 SquadThrowKeptOrders.r, src/game/squadthrow.ts:138 SquadThrowKeptOrders.r)
#[derive(Clone, Copy, Debug)]
pub struct SquadThrowKeptOrdersR {
}
impl SquadThrowKeptOrdersR {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SquadThrowKeptOrdersR {
        }
    }
}
impl Default for SquadThrowKeptOrdersR { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/squadthrow.ts:138 SquadThrowKept.duck, src/game/squadthrow.ts:201)
#[derive(Clone, Debug)]
pub struct SquadThrowKeptDuck {
    pub gd: GuardId,
    pub t: f64,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub m: Option<crate::squadthrow::SquadThrowKeptDuckM>,
}
impl SquadThrowKeptDuck {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SquadThrowKeptDuck {
            gd: GuardId(0),
            t: 0.0,
            x: 0.0,
            y: 0.0,
            z: 0.0,
            m: None,
        }
    }
}
impl Default for SquadThrowKeptDuck { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/squadthrow.ts:201 SquadThrowKeptDuck.m, src/game/frag.ts:697)
#[derive(Clone, Debug)]
pub struct SquadThrowKeptDuckM {
    pub stay: Vec<V3>,
    pub go: Vec<V3>,
}
impl SquadThrowKeptDuckM {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SquadThrowKeptDuckM {
            stay: Vec::new(),
            go: Vec::new(),
        }
    }
}
impl Default for SquadThrowKeptDuckM { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Lob, src/game/squadthrow.ts:184)
#[derive(Clone, Debug)]
pub struct Lob {
    pub walls: Vec<V3>,
    pub lim: crate::squadthrow::LobLim,
    pub clean: crate::squadthrow::LobClean,
    pub dirty: Option<crate::squadthrow::SquadThrower>,
}
impl Lob {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Lob {
            walls: Vec::new(),
            lim: crate::squadthrow::LobLim::zeroed(),
            clean: crate::squadthrow::LobClean::Plan { kind: crate::squadthrow::SquadThrowKind::Frag, at: [0.0; 3], fresh: false },
            dirty: None,
        }
    }
}
impl Default for Lob { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/squadthrow.ts:184 Lob.lim, src/game/squadthrow.ts:184)
#[derive(Clone, Copy, Debug)]
pub struct LobLim {
    pub n: f64,
}
impl LobLim {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LobLim {
            n: 0.0,
        }
    }
}
impl Default for LobLim { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: SquadThrowPlan, src/game/squadthrow.ts:518)
#[derive(Clone, Debug)]
pub struct SquadThrowPlan {
    pub gd: GuardId,
    pub from: V3,
    pub points: Vec<V3>,
    pub land: V3,
    pub kind: crate::squadthrow::SquadThrowKind,
    pub preview: Option<bool>,
    pub walk: Option<Vec<V3>>,
}
impl SquadThrowPlan {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SquadThrowPlan {
            gd: GuardId(0),
            from: [0.0; 3],
            points: Vec::new(),
            land: [0.0; 3],
            kind: crate::squadthrow::SquadThrowKind::Frag,
            preview: None,
            walk: None,
        }
    }
}
impl Default for SquadThrowPlan { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/squadthrow.ts:159 SQUADTHROW_LOG, src/game/squadthrow.ts:159)
#[derive(Clone, Debug)]
pub struct SquadthrowModSQUADTHROWLOGItem {
    pub t: f64,
    pub kind: crate::squadthrow::SquadThrowKind,
    pub who: String,
    pub from: V3,
    pub at: V3,
    pub land: V3,
    pub rest: V3,
}
impl SquadthrowModSQUADTHROWLOGItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SquadthrowModSQUADTHROWLOGItem {
            t: 0.0,
            kind: crate::squadthrow::SquadThrowKind::Frag,
            who: String::new(),
            from: [0.0; 3],
            at: [0.0; 3],
            land: [0.0; 3],
            rest: [0.0; 3],
        }
    }
}
impl Default for SquadthrowModSQUADTHROWLOGItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Word {
    Frag,
    Bang,
}
impl Word {
    pub const fn as_str(self) -> &'static str {
        match self {
            Word::Frag => "frag",
            Word::Bang => "bang",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum None {
    NoFragsLeft,
    NoFragsInThisKit,
    NoBangsLeft,
}
impl None {
    pub const fn as_str(self) -> &'static str {
        match self {
            None::NoFragsLeft => "no frags left.",
            None::NoFragsInThisKit => "no frags in this kit.",
            None::NoBangsLeft => "no bangs left.",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum RowOf {
    FragThere,
    StunThere,
}
impl RowOf {
    pub const fn as_str(self) -> &'static str {
        match self {
            RowOf::FragThere => "fragThere",
            RowOf::StunThere => "stunThere",
        }
    }
}

/// (TypeScript: inline at src/game/squadthrow.ts:181 squadThrower ->, src/game/squadthrow.ts:181)
#[derive(Clone, Debug)]
pub struct SquadThrower2 {
    pub gd: GuardId,
    pub from: V3,
    pub sol: crate::throwables::ThrowSolution,
    pub rest: V3,
}
impl SquadThrower2 {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SquadThrower2 {
            gd: GuardId(0),
            from: [0.0; 3],
            sol: crate::throwables::ThrowSolution::zeroed(),
            rest: [0.0; 3],
        }
    }
}
impl Default for SquadThrower2 { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/squadthrow.ts:187 standThrow ->, src/game/squadthrow.ts:187)
#[derive(Clone, Debug)]
pub struct StandThrow {
    pub gd: GuardId,
    pub from: V3,
    pub sol: crate::throwables::ThrowSolution,
    pub rest: V3,
}
impl StandThrow {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StandThrow {
            gd: GuardId(0),
            from: [0.0; 3],
            sol: crate::throwables::ThrowSolution::zeroed(),
            rest: [0.0; 3],
        }
    }
}
impl Default for StandThrow { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/squadthrow.ts:380 squadFragCatches(moved), src/game/squadthrow.ts:380)
#[derive(Clone, Copy, Debug)]
pub struct SquadFragCatchesMoved {
    pub gd: GuardId,
    pub at: V3,
}
impl SquadFragCatchesMoved {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SquadFragCatchesMoved {
            gd: GuardId(0),
            at: [0.0; 3],
        }
    }
}
impl Default for SquadFragCatchesMoved { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/squadthrow.ts:380 squadFragCatches(burst), src/game/squadthrow.ts:380)
#[derive(Clone, Debug)]
pub struct SquadFragCatchesBurst {
    pub reach: f64,
    pub no: String,
}
impl SquadFragCatchesBurst {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SquadFragCatchesBurst {
            reach: 0.0,
            no: String::new(),
        }
    }
}
impl Default for SquadFragCatchesBurst { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/squadthrow.ts:387 fragCatches(moved), src/game/squadthrow.ts:387)
#[derive(Clone, Copy, Debug)]
pub struct FragCatchesMoved {
    pub gd: GuardId,
    pub at: V3,
}
impl FragCatchesMoved {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FragCatchesMoved {
            gd: GuardId(0),
            at: [0.0; 3],
        }
    }
}
impl Default for FragCatchesMoved { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/squadthrow.ts:387 fragCatches(burst), src/game/squadthrow.ts:387)
#[derive(Clone, Debug)]
pub struct FragCatchesBurst {
    pub reach: f64,
    pub no: String,
}
impl FragCatchesBurst {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FragCatchesBurst {
            reach: 0.0,
            no: String::new(),
        }
    }
}
impl Default for FragCatchesBurst { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct SquadthrowMod {
    /// (src/game/squadthrow.ts:133 states)
    pub states: JsMap<PlayerId, crate::squadthrow::State>,
    /// (src/game/squadthrow.ts:159 SQUADTHROW_LOG)
    pub squadthrow_log: Vec<crate::squadthrow::SquadthrowModSQUADTHROWLOGItem>,
    /// (src/game/squadthrow.ts:201 duckMates)
    pub duck_mates: Option<crate::squadthrow::SquadThrowKeptDuck>,
}
impl SquadthrowMod {
    pub fn blank() -> Self {
        SquadthrowMod {
            states: JsMap::new(),
            squadthrow_log: Vec::new(),
            duck_mates: None,
        }
    }
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct SquadthrowStats {
    pub thrown: i32,
    pub frags: i32,
    pub stuns: i32,
    pub refused: i32,
    pub walks: i32,
    pub walked: i32,
    pub gave_up: i32,
    pub searches: i32,
    pub arcs: i32,
    pub ducks: i32,
    pub cells: i32,
}

/// (src/game/squadthrow.ts:117 THROW_PAGE)
pub const THROW_PAGE: &str = "throw";

/// (src/game/squadthrow.ts:117 THROW_OPEN)
pub const THROW_OPEN: &str = "throwThere";

/// (src/game/squadthrow.ts:117 FRAG_THERE)
pub const FRAG_THERE: &str = "fragThere";

/// (src/game/squadthrow.ts:117 STUN_THERE)
pub const STUN_THERE: &str = "stunThere";

/// (src/game/squadthrow.ts:143 TICK_KEPT: a table nothing writes)
pub fn tick_kept() -> Vec<String> { alloc::vec![String::from("frag.WALLS_FOR"), String::from("seenwalk.shared")] }

/// (src/game/squadthrow.ts:244 RESUMED: a table nothing writes)
pub fn resumed() -> Vec<String> { alloc::vec![String::from("move"), String::from("stack"), String::from("cover"), String::from("hold"), String::from("fallIn")] }
