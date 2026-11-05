//! The types of `squad`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: Sweep, src/game/squad.ts:153)
#[derive(Clone, Debug)]
pub struct Sweep {
    pub pts: Vec<V3>,
    pub t0: f64,
    pub dur: f64,
    pub active: bool,
}
impl Sweep {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        Sweep {
            pts: Vec::new(),
            t0: 0.0,
            dur: 1.0,
            active: false,
        }
    }
}
impl Default for Sweep { fn default() -> Self { Self::blank() } }

/// (TypeScript: RoomClear, src/game/squad.ts:183)
#[derive(Clone, Debug)]
pub struct RoomClear {
    pub phase: crate::squad::RoomClearPhase,
    pub t_kick: f64,
    pub t_enter: f64,
    pub t_enter2: f64,
    pub t_search: f64,
    pub t_resolve: f64,
    pub door: Option<DoorId>,
    pub next_door: Option<DoorId>,
    pub live: bool,
    pub done: JsSet<String>,
    pub flung: bool,
    pub b_go: bool,
    pub no_mark: bool,
    pub t_set: f64,
    pub t_go: f64,
    pub t_now: f64,
    pub a_home: bool,
    pub b_home: bool,
    pub aim_a: V3,
    pub aim_b: V3,
    pub sw_a: crate::squad::Sweep,
    pub sw_b: crate::squad::Sweep,
    pub rest_a: Option<Vec<V3>>,
    pub rest_b: Option<Vec<V3>>,
    pub wp_a: Vec<V3>,
    pub wp_b: Vec<V3>,
    pub plan: crate::scene_level::RoomClearPlan,
    pub a: GuardId,
    pub b: Option<GuardId>,
}
impl RoomClear {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        RoomClear {
            phase: crate::squad::RoomClearPhase::Stack,
            t_kick: f64::NAN,
            t_enter: f64::NAN,
            t_enter2: f64::NAN,
            t_search: f64::NAN,
            t_resolve: f64::NAN,
            door: None,
            next_door: None,
            live: false,
            done: JsSet::new(),
            flung: false,
            b_go: false,
            no_mark: false,
            t_set: f64::NAN,
            t_go: f64::NAN,
            t_now: 0.0,
            a_home: false,
            b_home: false,
            aim_a: [0.0, 0.0, 0.0],
            aim_b: [0.0, 0.0, 0.0],
            sw_a: crate::squad::Sweep::blank(),
            sw_b: crate::squad::Sweep::blank(),
            rest_a: None,
            rest_b: None,
            wp_a: Vec::new(),
            wp_b: Vec::new(),
            plan: crate::scene_level::RoomClearPlan::zeroed(),
            a: GuardId(0),
            b: None,
        }
    }
}
impl Default for RoomClear { fn default() -> Self { Self::blank() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum RoomClearPhase {
    Search,
    Kick,
    Breach,
    Stack,
    Enter,
    Resolve,
}
impl RoomClearPhase {
    pub const fn as_str(self) -> &'static str {
        match self {
            RoomClearPhase::Search => "search",
            RoomClearPhase::Kick => "kick",
            RoomClearPhase::Breach => "breach",
            RoomClearPhase::Stack => "stack",
            RoomClearPhase::Enter => "enter",
            RoomClearPhase::Resolve => "resolve",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum ClearStage {
    Approach,
    Done,
    Muster,
    Drill,
}
impl ClearStage {
    pub const fn as_str(self) -> &'static str {
        match self {
            ClearStage::Approach => "approach",
            ClearStage::Done => "done",
            ClearStage::Muster => "muster",
            ClearStage::Drill => "drill",
        }
    }
}

/// (TypeScript: Clearing, src/game/squad.ts:489)
#[derive(Clone, Debug)]
pub struct Clearing {
    pub stage: crate::squad::ClearStage,
    pub room_i: f64,
    pub drill: Option<RoomClearId>,
    pub cur: Option<crate::scene_level::RoomClearPlan>,
    pub t: f64,
    pub total: f64,
    pub eta: f64,
    pub t_drill: f64,
    pub t_near: f64,
    pub aw: Vec<f64>,
    pub wps: [Vec<V3>; 2],
    pub dark: bool,
    pub rooms: Vec<Vec<crate::scene_level::RoomClearPlan>>,
    pub a: GuardId,
    pub b: Option<GuardId>,
}
impl Clearing {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        Clearing {
            stage: crate::squad::ClearStage::Muster,
            room_i: 0.0,
            drill: None,
            cur: None,
            t: 0.0,
            total: 0.0,
            eta: 0.0,
            t_drill: 0.0,
            t_near: f64::NAN,
            aw: alloc::vec![0.0, 0.0],
            wps: [Vec::new(), Vec::new()],
            dark: false,
            rooms: Vec::new(),
            a: GuardId(0),
            b: None,
        }
    }
}
impl Default for Clearing { fn default() -> Self { Self::blank() } }

/// (TypeScript: inline at src/game/squad.ts:38 settleWatch, src/game/squad.ts:38)
#[derive(Clone, Copy, Debug)]
pub struct SquadModSettleWatchValue {
    pub goal: V3,
    pub best: f64,
    pub t: f64,
}
impl SquadModSettleWatchValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SquadModSettleWatchValue {
            goal: [0.0; 3],
            best: 0.0,
            t: 0.0,
        }
    }
}
impl Default for SquadModSettleWatchValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/squad.ts:41 walkWatch, src/game/squad.ts:41)
#[derive(Clone, Copy, Debug)]
pub struct SquadModWalkWatchValue {
    pub x: f64,
    pub z: f64,
    pub t: f64,
}
impl SquadModWalkWatchValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SquadModWalkWatchValue {
            x: 0.0,
            z: 0.0,
            t: 0.0,
        }
    }
}
impl Default for SquadModWalkWatchValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/squad.ts:399 CENTRE_CHECK, src/game/squad.ts:399)
#[derive(Clone, Copy, Debug)]
pub struct CENTRECHECK {
    pub corner: f64,
    pub total: f64,
    pub min_leg: f64,
}
impl CENTRECHECK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CENTRECHECK {
            corner: 0.0,
            total: 0.0,
            min_leg: 0.0,
        }
    }
}
impl Default for CENTRECHECK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/squad.ts:422 BODY_LINES, ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1583)
#[derive(Clone, Debug)]
pub struct BODYLINES {
    pub stack1: Option<String>,
    pub stack2: Option<String>,
    pub go2: Option<String>,
    pub in1: Option<String>,
    pub side2: Option<String>,
    pub side1: Option<String>,
    pub push1: Option<String>,
    pub under2: Option<String>,
    pub nothing2: Option<String>,
    pub corner1: Option<String>,
    pub empty2: Option<String>,
    pub radio1: Option<String>,
}
impl BODYLINES {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BODYLINES {
            stack1: None,
            stack2: None,
            go2: None,
            in1: None,
            side2: None,
            side1: None,
            push1: None,
            under2: None,
            nothing2: None,
            corner1: None,
            empty2: None,
            radio1: None,
        }
    }
}
impl Default for BODYLINES { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/squad.ts:428 BODY_LINES_NOW, ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1583)
#[derive(Clone, Debug)]
pub struct BODYLINESNOW {
    pub stack1: Option<String>,
    pub stack2: Option<String>,
    pub go2: Option<String>,
    pub in1: Option<String>,
    pub side2: Option<String>,
    pub side1: Option<String>,
    pub push1: Option<String>,
    pub under2: Option<String>,
    pub nothing2: Option<String>,
    pub corner1: Option<String>,
    pub empty2: Option<String>,
    pub radio1: Option<String>,
}
impl BODYLINESNOW {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BODYLINESNOW {
            stack1: None,
            stack2: None,
            go2: None,
            in1: None,
            side2: None,
            side1: None,
            push1: None,
            under2: None,
            nothing2: None,
            corner1: None,
            empty2: None,
            radio1: None,
        }
    }
}
impl Default for BODYLINESNOW { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/squad.ts:429 BODY_LINES_COLD_NOW, ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1583)
#[derive(Clone, Debug)]
pub struct BODYLINESCOLDNOW {
    pub stack1: Option<String>,
    pub stack2: Option<String>,
    pub go2: Option<String>,
    pub in1: Option<String>,
    pub side2: Option<String>,
    pub side1: Option<String>,
    pub push1: Option<String>,
    pub under2: Option<String>,
    pub nothing2: Option<String>,
    pub corner1: Option<String>,
    pub empty2: Option<String>,
    pub radio1: Option<String>,
}
impl BODYLINESCOLDNOW {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BODYLINESCOLDNOW {
            stack1: None,
            stack2: None,
            go2: None,
            in1: None,
            side2: None,
            side1: None,
            push1: None,
            under2: None,
            nothing2: None,
            corner1: None,
            empty2: None,
            radio1: None,
        }
    }
}
impl Default for BODYLINESCOLDNOW { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/squad.ts:431 BODY_LINES_COLD, ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1583)
#[derive(Clone, Debug)]
pub struct BODYLINESCOLD {
    pub stack1: Option<String>,
    pub stack2: Option<String>,
    pub go2: Option<String>,
    pub in1: Option<String>,
    pub side2: Option<String>,
    pub side1: Option<String>,
    pub push1: Option<String>,
    pub under2: Option<String>,
    pub nothing2: Option<String>,
    pub corner1: Option<String>,
    pub empty2: Option<String>,
    pub radio1: Option<String>,
}
impl BODYLINESCOLD {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BODYLINESCOLD {
            stack1: None,
            stack2: None,
            go2: None,
            in1: None,
            side2: None,
            side1: None,
            push1: None,
            under2: None,
            nothing2: None,
            corner1: None,
            empty2: None,
            radio1: None,
        }
    }
}
impl Default for BODYLINESCOLD { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum EndGuardScriptState {
    Patrol,
    Suspicious,
    Alert,
    Search,
}
impl EndGuardScriptState {
    pub const fn as_str(self) -> &'static str {
        match self {
            EndGuardScriptState::Patrol => "patrol",
            EndGuardScriptState::Suspicious => "suspicious",
            EndGuardScriptState::Alert => "alert",
            EndGuardScriptState::Search => "search",
        }
    }
}

/// (TypeScript: inline at src/game/squad.ts:144 order(patch), ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1583)
#[derive(Clone, Copy, Debug)]
pub struct OrderPatch {
    pub goal: Option<Option<V3>>,
    pub speed: Option<Option<f64>>,
    pub face: Option<Option<f64>>,
    pub strafe: Option<Option<bool>>,
    pub aim_at: Option<Option<V3>>,
    pub stance: Option<Option<crate::character::Stance>>,
    pub stack_side: Option<Option<f64>>,
    pub crouch: Option<Option<bool>>,
    pub lean: Option<Option<f64>>,
    pub upper: Option<Option<crate::character::UpperMode>>,
    pub light: Option<Option<bool>>,
    pub arrived: Option<Option<bool>>,
    pub hot: Option<Option<bool>>,
    pub flinch: Option<Option<bool>>,
    pub direct: Option<Option<bool>>,
    pub smoke: Option<Option<bool>>,
    pub over: Option<Option<crate::game::GuardScriptOver>>,
    /// INVENTED: true only from a caller that passes a goal it keeps and moves (orders.ts:1519, qrf.ts:828, :851): that one compares by value. goal Some(Some(_)) from anyone else is a new leg, even at the same spot (squad.ts:146)
    pub held: bool,
}
impl OrderPatch {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        OrderPatch {
            goal: None,
            speed: None,
            face: None,
            strafe: None,
            aim_at: None,
            stance: None,
            stack_side: None,
            crouch: None,
            lean: None,
            upper: None,
            light: None,
            arrived: None,
            hot: None,
            flinch: None,
            direct: None,
            smoke: None,
            over: None,
            held: false,
        }
    }
}
impl Default for OrderPatch { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/squad.ts:200 constructor(opts), src/game/squad.ts:200)
#[derive(Clone, Copy, Debug)]
pub struct RoomClearNewOpts {
    pub live: Option<bool>,
}
impl RoomClearNewOpts {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RoomClearNewOpts {
            live: None,
        }
    }
}
impl Default for RoomClearNewOpts { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum RoomClearEndState {
    Patrol,
    Search,
}
impl RoomClearEndState {
    pub const fn as_str(self) -> &'static str {
        match self {
            RoomClearEndState::Patrol => "patrol",
            RoomClearEndState::Search => "search",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum RoomClearSayKey {
    Stack1,
    Stack2,
    Go2,
    In1,
    Side2,
    Side1,
    Push1,
    Under2,
    Nothing2,
    Corner1,
    Empty2,
    Radio1,
}
impl RoomClearSayKey {
    pub const fn as_str(self) -> &'static str {
        match self {
            RoomClearSayKey::Stack1 => "stack1",
            RoomClearSayKey::Stack2 => "stack2",
            RoomClearSayKey::Go2 => "go2",
            RoomClearSayKey::In1 => "in1",
            RoomClearSayKey::Side2 => "side2",
            RoomClearSayKey::Side1 => "side1",
            RoomClearSayKey::Push1 => "push1",
            RoomClearSayKey::Under2 => "under2",
            RoomClearSayKey::Nothing2 => "nothing2",
            RoomClearSayKey::Corner1 => "corner1",
            RoomClearSayKey::Empty2 => "empty2",
            RoomClearSayKey::Radio1 => "radio1",
        }
    }
}

/// (TypeScript: inline at src/game/squad.ts:247 route(patch), ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1583)
#[derive(Clone, Copy, Debug)]
pub struct RoomClearRoutePatch {
    pub goal: Option<V3>,
    pub speed: Option<f64>,
    pub face: Option<f64>,
    pub strafe: Option<bool>,
    pub aim_at: Option<V3>,
    pub stance: Option<crate::character::Stance>,
    pub stack_side: Option<f64>,
    pub crouch: Option<bool>,
    pub lean: Option<f64>,
    pub upper: Option<crate::character::UpperMode>,
    pub light: Option<bool>,
    pub arrived: Option<bool>,
    pub hot: Option<bool>,
    pub flinch: Option<bool>,
    pub direct: Option<bool>,
    pub smoke: Option<bool>,
    pub over: Option<crate::game::GuardScriptOver>,
}
impl RoomClearRoutePatch {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RoomClearRoutePatch {
            goal: None,
            speed: None,
            face: None,
            strafe: None,
            aim_at: None,
            stance: None,
            stack_side: None,
            crouch: None,
            lean: None,
            upper: None,
            light: None,
            arrived: None,
            hot: None,
            flinch: None,
            direct: None,
            smoke: None,
            over: None,
        }
    }
}
impl Default for RoomClearRoutePatch { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Perceive {
    Suspicious,
    Alert,
    Body,
}
impl Perceive {
    pub const fn as_str(self) -> &'static str {
        match self {
            Perceive::Suspicious => "suspicious",
            Perceive::Alert => "alert",
            Perceive::Body => "body",
        }
    }
}

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct SquadMod {
    /// (src/game/squad.ts:38 settleWatch)
    pub settle_watch: JsMap<GuardId, crate::squad::SquadModSettleWatchValue>,
    /// (src/game/squad.ts:41 walkWatch)
    pub walk_watch: JsMap<GuardId, crate::squad::SquadModWalkWatchValue>,
}
impl SquadMod {
    pub fn blank() -> Self {
        SquadMod {
            settle_watch: JsMap::new(),
            walk_watch: JsMap::new(),
        }
    }
}

/// (src/game/squad.ts:30 SETTLE)
pub const SETTLE: f64 = 0.6;

/// (src/game/squad.ts:34 FACE_WITHIN)
pub const FACE_WITHIN: f64 = 1.0;

/// (src/game/squad.ts:394 STACK_CAP)
pub const STACK_CAP: f64 = 6.0;

/// (src/game/squad.ts:396 KICK_BACK)
pub const KICK_BACK: f64 = 0.7;

/// (src/game/squad.ts:399 CENTRE_CHECK: a table nothing writes)
pub fn centre_check() -> crate::squad::CENTRECHECK { crate::squad::CENTRECHECK { corner: 0.95, total: 3.0, min_leg: 0.6 } }

/// (src/game/squad.ts:422 BODY_LINES: a table nothing writes)
pub fn body_lines() -> crate::squad::BODYLINES { crate::squad::BODYLINES { stack1: None, stack2: None, go2: None, in1: None, side2: Some(String::from("door side clear — apart from him.")), side1: Some(String::from("my side's clear. …he's right here.")), push1: None, under2: Some(String::from("checking under… just him.")), nothing2: Some(String::from("nothing else down here.")), corner1: Some(String::from("corner's clear. only him.")), empty2: Some(String::from("room's clear — nobody but him.")), radio1: None } }

/// (src/game/squad.ts:428 BODY_LINES_NOW: a table nothing writes)
pub fn body_lines_now() -> crate::squad::BODYLINESNOW { crate::squad::BODYLINESNOW { stack1: None, stack2: None, go2: None, in1: None, side2: Some(String::from("door side clear — except for him.")), side1: None, push1: None, under2: None, nothing2: None, corner1: None, empty2: None, radio1: None } }

/// (src/game/squad.ts:429 BODY_LINES_COLD_NOW: a table nothing writes)
pub fn body_lines_cold_now() -> crate::squad::BODYLINESCOLDNOW { crate::squad::BODYLINESCOLDNOW { stack1: None, stack2: None, go2: None, in1: None, side2: None, side1: None, push1: None, under2: None, nothing2: Some(String::from("nothing else down here. he took a hard hit.")), corner1: None, empty2: None, radio1: None } }

/// (src/game/squad.ts:431 BODY_LINES_COLD: a table nothing writes)
pub fn body_lines_cold() -> crate::squad::BODYLINESCOLD { crate::squad::BODYLINESCOLD { stack1: None, stack2: None, go2: None, in1: None, side2: Some(String::from("door side clear — just him, out cold.")), side1: Some(String::from("my side's clear. …he's here — still breathing.")), push1: None, under2: Some(String::from("checking under… just him. breathing.")), nothing2: Some(String::from("nothing else down here. he took a proper hit.")), corner1: Some(String::from("corner's clear. only him.")), empty2: Some(String::from("room's clear — nobody but him, and he's breathing.")), radio1: None } }

/// (src/game/squad.ts:449 SUSPECT)
pub const SUSPECT: f64 = 0.4;

/// (src/game/squad.ts:472 APPROACH_SPEED)
pub const APPROACH_SPEED: f64 = 1.6;

/// (src/game/squad.ts:474 DRILL_CAP)
pub const DRILL_CAP: f64 = 32.0;
