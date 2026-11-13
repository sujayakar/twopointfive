//! The types of `wallreact`.
use crate::{Blank, Words, bytes::ToBytes, ids::*, js::*, V3};
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
    pub cause: crate::wallbreak::LevelChangeCause,
    pub who: f64,
    pub blast_by: Option<f64>,
    pub ops: f64,
    pub men: JsMap<GuardId, f64>,
    pub said: JsSet<crate::teams::Team>,
    pub tell: Vec<crate::wallreact::BreachTellItem>,
    pub opening: Option<CoverOpeningId>,
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
    pub a: TaskId,
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

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct WallreactMod {
    pub wall_react: crate::wallreact::WallReact,
}

pub const EYE: f64 = 1.55;

pub const CHEST: f64 = 1.1;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Words)]
pub enum Reconsidered {
    Fine,
    Moved,
    Later,
}
