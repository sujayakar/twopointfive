//! The types of `breach`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: BreachDrill, src/game/breach.ts:75)
#[derive(Clone, Debug)]
pub struct BreachDrill {
    pub phase: crate::breach::Phase,
    pub t_stack: f64,
    pub t_set: f64,
    pub t_go: f64,
    pub t_kick: f64,
    pub t_flung: f64,
    pub t_toss: f64,
    pub t_land: f64,
    pub t_bang: f64,
    pub t_enter: f64,
    pub t_corner: f64,
    pub t_clear: f64,
    pub t_next: f64,
    pub t_stacked2: f64,
    pub t_done: f64,
    pub beats: Vec<crate::breach::BreachDrillBeatsItem>,
    pub rounds: Vec<f64>,
    pub t: f64,
    pub done: JsSet<String>,
    pub aim_a: V3,
    pub aim_b: V3,
    pub wp_a: Vec<V3>,
    pub wp_b: Vec<V3>,
    pub can: Option<ItemId>,
    pub chopped: bool,
    pub down_logged: Vec<bool>,
    pub flows: [crate::breach::OpFlow; 2],
    pub guarantee_downs: bool,
    pub live: Option<crate::breach::BreachLive>,
    pub t_run: f64,
    pub plan: crate::scene_level::BreachPlan,
    pub a: GuardId,
    pub b: GuardId,
    pub hostiles: Vec<GuardId>,
    pub t_all_done: f64,
}
impl BreachDrill {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        BreachDrill {
            phase: crate::breach::Phase::Wait,
            t_stack: f64::NAN,
            t_set: f64::NAN,
            t_go: f64::NAN,
            t_kick: f64::NAN,
            t_flung: f64::NAN,
            t_toss: f64::NAN,
            t_land: f64::NAN,
            t_bang: f64::NAN,
            t_enter: f64::NAN,
            t_corner: f64::NAN,
            t_clear: f64::NAN,
            t_next: f64::NAN,
            t_stacked2: f64::NAN,
            t_done: f64::NAN,
            beats: Vec::new(),
            rounds: alloc::vec![0.0, 0.0],
            t: 0.0,
            done: JsSet::new(),
            aim_a: [0.0; 3],
            aim_b: [0.0; 3],
            wp_a: Vec::new(),
            wp_b: Vec::new(),
            can: None,
            chopped: false,
            down_logged: Vec::new(),
            flows: core::array::from_fn(|_| crate::breach::OpFlow::zeroed()),
            guarantee_downs: false,
            live: None,
            t_run: f64::NAN,
            plan: crate::scene_level::BreachPlan::zeroed(),
            a: GuardId(0),
            b: GuardId(0),
            hostiles: Vec::new(),
            t_all_done: f64::NAN,
        }
    }
}
impl Default for BreachDrill { fn default() -> Self { Self::blank() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Phase {
    Clear,
    Bang,
    Kick,
    Approach,
    Breach,
    Stack,
    Done,
    Wait,
    Enter,
    Restack,
}
impl Phase {
    pub const fn as_str(self) -> &'static str {
        match self {
            Phase::Clear => "clear",
            Phase::Bang => "bang",
            Phase::Kick => "kick",
            Phase::Approach => "approach",
            Phase::Breach => "breach",
            Phase::Stack => "stack",
            Phase::Done => "done",
            Phase::Wait => "wait",
            Phase::Enter => "enter",
            Phase::Restack => "restack",
        }
    }
}

/// (TypeScript: inline at src/game/breach.ts:78 BreachDrill.beats, src/game/breach.ts:78)
#[derive(Clone, Debug)]
pub struct BreachDrillBeatsItem {
    pub t: f64,
    pub what: String,
}
impl BreachDrillBeatsItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BreachDrillBeatsItem {
            t: 0.0,
            what: String::new(),
        }
    }
}
impl Default for BreachDrillBeatsItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: OpFlow, src/game/breach.ts:65)
#[derive(Clone, Debug)]
pub struct OpFlow {
    pub st: crate::breach::FlowSt,
    pub t0: f64,
    pub wps: Vec<V3>,
    pub sweep: crate::squad::Sweep,
    pub aim: V3,
    pub q: Vec<f64>,
    pub on: f64,
    pub since: f64,
    pub n: f64,
    pub next: f64,
    pub said: bool,
    pub in_at: f64,
    pub corner_at: f64,
    pub hold_at: f64,
}
impl OpFlow {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        OpFlow {
            st: crate::breach::FlowSt::Hold,
            t0: 0.0,
            wps: Vec::new(),
            sweep: crate::squad::Sweep::blank(),
            aim: [0.0; 3],
            q: Vec::new(),
            on: 0.0,
            since: 0.0,
            n: 0.0,
            next: 0.0,
            said: false,
            in_at: 0.0,
            corner_at: 0.0,
            hold_at: 0.0,
        }
    }
}
impl Default for OpFlow { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum FlowSt {
    Hold,
    Wait,
    Into,
    Corner,
    Centre,
}
impl FlowSt {
    pub const fn as_str(self) -> &'static str {
        match self {
            FlowSt::Hold => "hold",
            FlowSt::Wait => "wait",
            FlowSt::Into => "into",
            FlowSt::Corner => "corner",
            FlowSt::Centre => "centre",
        }
    }
}

/// (TypeScript: BreachLive, src/game/breach.ts:54)
#[derive(Clone, Copy, Debug)]
pub struct BreachLive {
    pub orders: f64,
    pub lights: bool,
    pub loud_boot: bool,
    pub stun: bool,
    pub set_hold: Option<f64>,
}
impl BreachLive {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BreachLive {
            orders: 0.0,
            lights: false,
            loud_boot: false,
            stun: false,
            set_hold: None,
        }
    }
}
impl Default for BreachLive { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/breach.ts:98 constructor(opts), src/game/breach.ts:98)
#[derive(Clone, Copy, Debug)]
pub struct BreachDrillNewOpts {
    pub guarantee_downs: Option<bool>,
    pub live: Option<crate::breach::BreachLive>,
}
impl BreachDrillNewOpts {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BreachDrillNewOpts {
            guarantee_downs: None,
            live: None,
        }
    }
}
impl Default for BreachDrillNewOpts { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/breach.ts:116 route(patch), ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1583)
#[derive(Clone, Copy, Debug)]
pub struct BreachDrillRoutePatch {
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
impl BreachDrillRoutePatch {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BreachDrillRoutePatch {
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
impl Default for BreachDrillRoutePatch { fn default() -> Self { Self::zeroed() } }

/// (src/game/breach.ts:27 RUN)
pub const RUN: f64 = 3.4;

/// (src/game/breach.ts:27 HOLD_OPEN)
pub const HOLD_OPEN: f64 = 2.6;

/// (src/game/breach.ts:29 SET_HOLD)
pub const SET_HOLD: f64 = 2.4;

/// (src/game/breach.ts:31 TOSS_AFTER_IMPACT)
pub const TOSS_AFTER_IMPACT: f64 = 0.15;

/// (src/game/breach.ts:31 FUSE_AFTER_LAND)
pub const FUSE_AFTER_LAND: f64 = 0.45;

/// (src/game/breach.ts:34 CHOP_LEAD)
pub const CHOP_LEAD: f64 = 0.4;

/// (src/game/breach.ts:36 GO_AFTER_BANG)
pub const GO_AFTER_BANG: f64 = 0.4;

/// (src/game/breach.ts:36 TWO_AFTER_ONE)
pub const TWO_AFTER_ONE: f64 = 0.5;

/// (src/game/breach.ts:38 CENTRE_SECS)
pub const CENTRE_SECS: f64 = 0.32;

/// (src/game/breach.ts:40 SETTLE)
pub const SETTLE: f64 = 0.14;

/// (src/game/breach.ts:40 PAIR)
pub const PAIR: f64 = 0.16;

/// (src/game/breach.ts:40 SHIFT)
pub const SHIFT: f64 = 0.3;

/// (src/game/breach.ts:42 FIRE_INSIDE_M)
pub const FIRE_INSIDE_M: f64 = 0.3;

/// (src/game/breach.ts:44 NEXT_AFTER_CLEAR)
pub const NEXT_AFTER_CLEAR: f64 = 2.2;

/// (src/game/breach.ts:44 RESTACK_HOLD)
pub const RESTACK_HOLD: f64 = 1.5;

/// (src/game/breach.ts:44 RESTACK_CAP)
pub const RESTACK_CAP: f64 = 7.5;

/// (src/game/breach.ts:58 LIVE_LINES: a table nothing writes)
pub fn live_lines() -> JsMap<String, String> { { let mut t = JsMap::new(); t.set(String::from("clear1"), String::from("right side — clear.")); t.set(String::from("clear2"), String::from("left clear. i have the door.")); t.set(String::from("roomClear1"), String::from("room clear.")); t } }

/// (src/game/breach.ts:61 LIVE_GUIDE: a table nothing writes)
pub fn live_guide() -> JsMap<String, Option<String>> { { let mut t = JsMap::new(); t.set(String::from("clear1"), Some(String::from("right side — clear. i've got the door."))); t } }

/// (src/game/breach.ts:62 LIVE_NOW: a table nothing writes)
pub fn live_now() -> JsMap<String, String> { { let mut t = JsMap::new(); t.set(String::from("clear1"), String::from("right side — clear. i have the door.")); t.set(String::from("clear2"), String::from("left clear.")); t } }

/// (src/game/breach.ts:64 LIVE_NO_CHARGE: a table nothing writes)
pub fn live_no_charge() -> JsMap<String, String> { { let mut t = JsMap::new(); t.set(String::from("stack1"), String::from("stack on me — side door. on my boot.")); t.set(String::from("stack2"), String::from("set.")); t } }
