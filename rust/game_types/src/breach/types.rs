//! The types of `breach`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, ToBytes)]
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
    /// what a new one holds
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
            flows: core::array::from_fn(|_| crate::breach::OpFlow::default()),
            guarantee_downs: false,
            live: None,
            t_run: f64::NAN,
            plan: crate::scene_level::BreachPlan::default(),
            a: GuardId(0),
            b: GuardId(0),
            hostiles: Vec::new(),
            t_all_done: f64::NAN,
        }
    }
}
impl Default for BreachDrill { fn default() -> Self { Self::blank() } }

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, ToBytes)]
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

#[derive(Clone, Debug, ToBytes, Default)]
pub struct BreachDrillBeatsItem {
    pub what: String,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct OpFlow {
    pub st: crate::breach::FlowSt,
    pub t0: f64,
    pub sweep: crate::squad::Sweep,
    pub aim: V3,
    pub q: Vec<usize>,
    pub on: i32,
    pub since: f64,
    pub n: f64,
    pub next: f64,
    pub in_at: f64,
    pub corner_at: f64,
    pub hold_at: f64,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum FlowSt {
    #[default] Hold,
    Wait,
    Into,
    Corner,
    Centre,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct BreachLive {
    pub orders: f64,
    pub lights: bool,
    pub loud_boot: bool,
    pub stun: bool,
    pub set_hold: Option<f64>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct BreachDrillNewOpts {
    pub guarantee_downs: Option<bool>,
    pub live: Option<crate::breach::BreachLive>,
}

pub const RUN: f64 = 3.4;

pub const HOLD_OPEN: f64 = 2.6;

pub const SET_HOLD: f64 = 2.4;

pub const TOSS_AFTER_IMPACT: f64 = 0.15;

pub const FUSE_AFTER_LAND: f64 = 0.45;

pub const CHOP_LEAD: f64 = 0.4;

pub const GO_AFTER_BANG: f64 = 0.4;

pub const TWO_AFTER_ONE: f64 = 0.5;

pub const CENTRE_SECS: f64 = 0.32;

pub const SETTLE: f64 = 0.14;

pub const PAIR: f64 = 0.16;

pub const SHIFT: f64 = 0.3;

pub const FIRE_INSIDE_M: f64 = 0.3;

pub const NEXT_AFTER_CLEAR: f64 = 2.2;

pub const RESTACK_HOLD: f64 = 1.5;

pub const RESTACK_CAP: f64 = 7.5;

pub fn live_lines() -> JsMap<String, String> { { let mut t = JsMap::new(); t.set(String::from("clear1"), String::from("right side — clear.")); t.set(String::from("clear2"), String::from("left clear. i have the door.")); t.set(String::from("roomClear1"), String::from("room clear.")); t } }

pub fn live_guide() -> JsMap<String, Option<String>> { { let mut t = JsMap::new(); t.set(String::from("clear1"), Some(String::from("right side — clear. i've got the door."))); t } }

pub fn live_now() -> JsMap<String, String> { { let mut t = JsMap::new(); t.set(String::from("clear1"), String::from("right side — clear. i have the door.")); t.set(String::from("clear2"), String::from("left clear.")); t } }

pub fn live_no_charge() -> JsMap<String, String> { { let mut t = JsMap::new(); t.set(String::from("stack1"), String::from("stack on me — side door. on my boot.")); t.set(String::from("stack2"), String::from("set.")); t } }
