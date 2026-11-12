//! The types of `breach`.
use crate::{Blank, bytes::ToBytes, ids::*, js::*, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, ToBytes, Blank)]
pub struct BreachDrill {
    #[is(crate::breach::Phase::Wait)] pub phase: crate::breach::Phase,
    #[is(f64::NAN)] pub t_stack: f64,
    #[is(f64::NAN)] pub t_set: f64,
    #[is(f64::NAN)] pub t_go: f64,
    #[is(f64::NAN)] pub t_kick: f64,
    #[is(f64::NAN)] pub t_flung: f64,
    #[is(f64::NAN)] pub t_toss: f64,
    #[is(f64::NAN)] pub t_land: f64,
    #[is(f64::NAN)] pub t_bang: f64,
    #[is(f64::NAN)] pub t_enter: f64,
    #[is(f64::NAN)] pub t_corner: f64,
    #[is(f64::NAN)] pub t_clear: f64,
    /// called off with men inside
    pub recalled: bool,
    pub beats: Vec<crate::breach::BreachDrillBeatsItem>,
    pub t: f64,
    pub done: JsSet<String>,
    pub aim_a: V3,
    pub aim_b: V3,
    pub wp_a: Vec<V3>,
    pub wp_b: Vec<V3>,
    pub can: Option<ItemId>,
    pub chopped: bool,
    pub down_logged: Vec<bool>,
    #[is(core::array::from_fn(|_| crate::breach::OpFlow::default()))] pub flows: [crate::breach::OpFlow; 2],
    pub live: crate::breach::BreachLive,
    #[is(f64::NAN)] pub t_run: f64,
    pub plan: crate::scene_level::BreachPlan,
    #[is(GuardId(0))] pub a: GuardId,
    #[is(GuardId(0))] pub b: GuardId,
    pub hostiles: Vec<GuardId>,
    #[is(f64::NAN)] pub t_all_done: f64,
}

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
    pub stun: bool,
}

pub const RUN: f64 = 3.4;

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
