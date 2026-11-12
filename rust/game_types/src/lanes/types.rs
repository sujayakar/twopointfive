//! The types of `lanes`.
use crate::{Blank, bytes::ToBytes, ids::*, js::*, V3};
use alloc::vec::Vec;

/// a walk's price list, nav.findPath's penalty: a cell of the mask costs `dear`, any other 1, or at least the pursuit tactics' price where there is one; a man the rule leaves alone pays the tactics' price alone
#[derive(Clone, Debug)]
pub enum LanePenalty {
    Dear { m: Vec<u8>, w: f64, dear: f64 },
    DearOver { base: crate::tactics::TacticsPenalty, m: Vec<u8>, w: f64, dear: f64 },
    Base { base: crate::tactics::TacticsPenalty },
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Lane {
    pub id: f64,
    pub from: V3,
    pub at: V3,
    pub dx: f64,
    pub dz: f64,
    pub last_t: f64,
    pub until: f64,
    pub weight: f64,
    pub downs: f64,
    pub known: bool,
    pub known_at: f64,
    pub guns: Vec<V3>,
    pub saw: JsMap<GuardId, f64>,
    pub knowers: JsSet<GuardId>,
    pub cells: Option<JsSet<crate::Num>>,
    pub wedges: Vec<V3>,
    pub re: Option<crate::lanes::LaneRe>,
    pub who: JsSet<crate::Num>,
    pub moved: JsSet<crate::Num>,
    pub round_at: f64,
    pub stun_at: f64,
    pub push_until: f64,
    pub can: Option<crate::lanes::LaneCan>,
    pub peek_at: f64,
    pub peek_n: f64,
    pub peek_part: f64,
    pub pin_at: f64,
    pub pin_n: f64,
    pub pin_by: f64,
    pub frag_at: f64,
    pub e: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct LaneRe {
    pub set: JsSet<crate::Num>,
    pub i: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct LaneCan {
    pub t: f64,
    pub land: V3,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct Flank {
    pub lane: f64,
    pub phase: crate::lanes::FlankPhase,
    pub at: V3,
    pub t: f64,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum FlankPhase {
    #[default] Set,
    Go,
    Walk,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct Peek {
    pub lane: f64,
    pub phase: crate::lanes::PeekPhase,
    pub t: f64,
    pub home: V3,
    pub look: V3,
    pub to: V3,
    pub line_t: f64,
    pub eye: Option<V3>,
    pub fire: Option<crate::lanes::PeekFire>,
    pub frag: Option<crate::lanes::PeekFrag>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum PeekPhase {
    #[default] Out,
    Back,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct PeekFire {
    pub n: f64,
    pub t: f64,
    pub next: f64,
    pub saw: bool,
    pub line_t: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct PeekFrag {
    pub at: Option<V3>,
    pub k: f64,
    pub next: f64,
    pub walk: bool,
    pub going: bool,
    pub edge: bool,
    pub own: bool,
    pub until: f64,
    pub had: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct State {
    pub lanes: Vec<LanesLaneId>,
    pub serial: f64,
    pub mask: Option<Vec<u8>>,
    pub ver: f64,
    pub own: JsMap<GuardId, crate::lanes::StateOwnValue>,
    pub seen: JsSet<u32>,
    pub banged: JsSet<u32>,
    pub push: JsMap<GuardId, f64>,
    pub flank: JsMap<GuardId, f64>,
    pub flank_of: JsMap<GuardId, crate::lanes::Flank>,
    pub dropped: JsSet<GuardId>,
    pub fail_t: JsMap<GuardId, f64>,
    pub skip: bool,
    pub held: JsMap<GuardId, crate::lanes::StateHeldValue>,
    pub cans: Vec<ItemId>,
    pub roll: f64,
    pub due: Vec<crate::lanes::StateDueItem>,
    pub look: JsMap<GuardId, f64>,
    pub post: JsMap<GuardId, V3>,
    pub back_t: JsMap<GuardId, f64>,
    pub said_t: f64,
    pub t: f64,
    pub tick_t: f64,
    pub stats: crate::lanes::StateStats,
    pub peek: JsMap<GuardId, crate::lanes::Peek>,
    pub frag_no: JsMap<GuardId, crate::lanes::StateFragNoValue>,
    pub near: Vec<crate::lanes::StateNearItem>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct StateOwnValue {
    pub ver: f64,
    pub m: Vec<u8>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct StateHeldValue {
    pub lane: f64,
    pub t: f64,
    pub look: V3,
    pub aimed: bool,
    pub at: V3,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct StateDueItem {
    pub gd: GuardId,
    pub lane: f64,
    pub look: V3,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct StateStats {
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct StateFragNoValue {
    pub lane: f64,
    pub at: V3,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct StateNearItem {
    pub gd: GuardId,
    pub from: V3,
    pub at: V3,
    pub who: f64,
    pub t: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Stepper {
    pub gd: GuardId,
    pub to: V3,
}

#[derive(Clone, Copy, Debug)]
pub enum FragPlace {
    List(V3),
    Edge,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct LanesMod {
    pub states: Option<crate::lanes::State>,
}

pub const SPOT_RING: f64 = 0.7;
