//! The types of `wallreact`.
#![allow(unused_imports)]
use crate::{Blank, Words, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Breach {
    pub key: String,
    pub wall: f64,
    pub u: [f64; 2],
    pub n: [f64; 2],
    pub lo: f64,
    pub hi: f64,
    pub at: V3,
    pub half: f64,
    pub into: Option<WallreactBreachId>,
    pub walk: bool,
    pub cause: crate::wallreact::BreachCause,
    pub who: f64,
    pub blast_by: Option<f64>,
    pub ops: f64,
    pub men: JsMap<GuardId, f64>,
    pub said: JsSet<crate::teams::Team>,
    pub tell: Vec<crate::wallreact::BreachTellItem>,
    pub opening: Option<CoverOpeningId>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum BreachCause {
    #[default] Blast,
    Fire,
    Load,
    Reset,
    Collapse,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct BreachTellItem {
    pub by: GuardId,
    pub at: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct Mind {
    pub ev: f64,
    pub line: bool,
    pub line_t: f64,
    pub t: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct Gone {
    pub rect: [f64; 4],
    pub b: Option<WallreactBreachId>,
    pub who: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Pend {
    pub gd: GuardId,
    pub also: Option<Vec<crate::wallreact::Gone>>,
    pub until: f64,
    pub next: f64,
    pub rect: [f64; 4],
    pub b: Option<WallreactBreachId>,
    pub who: f64,
}

#[derive(Clone, Debug, ToBytes, Blank)]
pub struct WallReact {
    pub breaches: Vec<WallreactBreachId>,
    pub queue: Vec<crate::wallbreak::LevelChange>,
    pub stale: bool,
    pub pend: Vec<crate::wallreact::Pend>,
    pub flood_all: Option<Vec<f64>>,
    pub minds: JsMap<GuardId, JsMap<WallreactBreachId, crate::wallreact::Mind>>,
    pub watching: JsMap<GuardId, WallreactBreachId>,
    #[is(-1.0)] pub last: f64,
    #[is(-1.0)] pub last_ev: f64,
    #[is(-1.0)] pub last_sweep: f64,
    pub moved_t: JsMap<GuardId, f64>,
    pub unsub: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Words)]
pub enum WallReactManKnowsHow {
    Heard,
    Saw,
    Told,
    Made,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct WallReactPostsItem {
    pub gd: GuardId,
    pub a: OrdersAssignmentId,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct WallReactCharOf {
    pub pos: V3,
    pub team: crate::teams::Team,
}

#[derive(Clone, Debug)]
pub enum WallReactAsks {
    Text(String),
    Arm1(crate::wallreact::WallReactAsksArm1),
}

#[derive(Clone, Copy, Debug, Default)]
pub struct WallReactAsksArm1 {
    pub x: crate::wallreact::Gone,
    pub threat: V3,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum WallReactReconsider {
    Fine,
    Moved,
    Later,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Words)]
pub enum WallReactReconsider10 {
    Fine,
    Moved,
    Later,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum WallReactShift {
    Fine,
    Moved,
    Later,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum WallReactOpMove {
    Fine,
    Moved,
    Later,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct WallreactMod {
    /// (src/game/wallreact.ts:102 WALLREACT_LOG)
    pub wallreact_log: Vec<String>,
    /// (src/game/wallreact.ts:152)
    pub wall_react: crate::wallreact::WallReact,
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct WallreactStats {
    pub rows: i32,
    pub breaches: i32,
    pub merged: i32,
    pub learned_ops: i32,
    pub learned_men: i32,
    pub told: i32,
    pub moved_def: i32,
    pub moved_ai: i32,
    pub moved_op: i32,
    pub no_cover: i32,
    pub watches: i32,
    pub replans: i32,
    pub openings: i32,
}

/// (src/game/wallreact.ts:137 EYE)
pub const EYE: f64 = 1.55;

/// (src/game/wallreact.ts:137 CHEST)
pub const CHEST: f64 = 1.1;
