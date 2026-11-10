//! The types of `wallcharge`.
#![allow(unused_imports)]
use crate::{Blank, Words, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// a walk's answer kept for the open menu: a 'w|' key's length of walk (-1: no way), an 'r|' key's refusal from SquadOrders.reach (None: he can walk it)
#[derive(Clone, Debug, ToBytes)]
pub enum WallChargeWalkAnswer {
    Walk { len: f64 },
    Reach { why: Option<String> },
}
/// whose store a walk's answer is kept in: the open menu's (g.wall_charge.walks, counted in g.wall_charge.keep), one order's re-deal (WallOp.repick_kept), or one call's own (clearOf)
#[derive(Clone, Debug)]
pub enum WallChargeKept {
    Menu,
    Repick { op: WallOpId },
    Local { memo: JsMap<String, f64> },
}
/// what underFire's two answers close over: the wall about to go, the foes as known when it was asked, and each place's answer so far (a place by its bits: the TypeScript keeps them by the array's identity)
#[derive(Clone, Debug)]
pub enum WallChargeFireAsk {
    Lines { wall: Option<crate::wallcharge::WallSpot>, all: Vec<V3>, memo: JsMap<[crate::Num; 3], f64>, hit: JsMap<[crate::Num; 3], bool> },
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct WallSpot {
    pub index: f64,
    pub box_: BoxId,
    pub along_x: bool,
    pub u: f64,
    pub side: f64,
    pub ok: bool,
    pub why: String,
    pub outer: Option<bool>,
}

#[derive(Clone, Debug, ToBytes, Blank)]
pub struct WallCharge {
    pub ops: Vec<WallOpId>,
    pub done: Vec<crate::wallcharge::WallChargeDoneItem>,
    pub set: Vec<WallChargeId>,
    #[is(crate::wallcharge::WallChargeStats { orders: 0.0, blown: 0.0, holes: 0.0, cleared: 0.0 })] pub stats: crate::wallcharge::WallChargeStats,
    pub away: JsMap<GuardId, crate::wallcharge::WallChargeAwayValue>,
    pub away_marks: JsMap<WallChargeId, crate::wallcharge::WallChargeAwayMarksValue>,
    pub away_none: JsMap<GuardId, f64>,
    #[is(OrdersOrderDefId(0))] pub opener: OrdersOrderDefId,
    #[is(OrdersOrderDefId(0))] pub row: OrdersOrderDefId,
    pub seated: bool,
    pub player: Option<PlayerId>,
    pub roster: Vec<GuardId>,
    pub alone: f64,
    pub sam: Option<crate::wallcharge::WallChargeSam>,
    pub offered: Option<crate::wallcharge::WallSpot>,
    pub carried: Option<f64>,
    pub brought: Option<f64>,
    pub free_set: bool,
    pub walks: Option<crate::wallcharge::WallChargeWalks>,
    #[is(crate::wallcharge::WallChargeKeep { on: true })] pub keep: crate::wallcharge::WallChargeKeep,
    pub pressed: bool,
    pub last_blow: Option<crate::wallcharge::WallChargeLastBlow>,
    pub marker: Option<crate::game::Interactable>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct WallChargeDoneItem {
    pub t0: f64,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, ToBytes, Default)]
pub enum Phase {
    #[default] Wait,
    Place,
    Standoff,
    Armed,
    Enter,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct WallChargeDoneItemHole {
    pub du: f64,
    pub half: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct WallChargeDoneItemQueueItem {
    pub gd: GuardId,
    pub at: f64,
    pub lane: V3,
    pub pod: crate::wallcharge::Pod,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Pod {
    pub pos: V3,
    pub face: f64,
    pub watch: Option<V3>,
    pub door: Option<String>,
    pub hold: Option<crate::wallcharge::PodHold>,
    pub took: Option<bool>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct PodHold {
    pub face: f64,
    pub watch: V3,
    pub door: Option<String>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct WallChargeStats {
    pub orders: f64,
    pub blown: f64,
    pub holes: f64,
    pub cleared: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct WallChargeAwayValue {
    pub c: WallChargeId,
    pub at: V3,
    pub hit_t: f64,
    pub t: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct WallChargeAwayMarksValue {
    pub n: f64,
    pub marks: Vec<V3>,
    pub t: f64,
    pub sets: f64,
    pub r: crate::wallcharge::WallChargeAwayMarksValueR,
    pub pad: crate::wallcharge::WallChargeAwayMarksValueR,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct WallChargeAwayMarksValueR {
    pub standoff: f64,
    pub reach: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct WallChargeSam {
    pub wall: crate::wallcharge::WallSpot,
    pub t0: f64,
    pub at: V3,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct WallChargeWalks {
    pub menu: u32,
    pub stamp: String,
    pub map: JsMap<String, crate::wallcharge::WallChargeWalkAnswer>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct WallChargeKeep {
    pub on: bool,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct WallChargeLastBlow {
    pub sam: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct SetWallCharge {
    pub wall: crate::wallcharge::WallSpot,
    pub t: f64,
    pub by: String,
    pub paid: f64,
    pub free: Option<bool>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct WallOp {
    pub wall: crate::wallcharge::WallSpot,
    pub men: Vec<GuardId>,
    pub lead: GuardId,
    pub phase: crate::wallcharge::Phase,
    pub t0: f64,
    pub spot: V3,
    pub set_t: f64,
    pub stow_off_t: Option<f64>,
    pub owes: Option<bool>,
    pub mine: JsMap<GuardId, OrdersAssignmentId>,
    pub post: JsMap<GuardId, V3>,
    pub from: JsMap<GuardId, V3>,
    pub hole: Option<crate::wallcharge::WallChargeDoneItemHole>,
    pub queue: Vec<crate::wallcharge::WallChargeDoneItemQueueItem>,
    pub stage: JsMap<GuardId, crate::wallcharge::WallStage>,
    pub pod: JsMap<GuardId, crate::wallcharge::Pod>,
    pub crossed: JsMap<GuardId, f64>,
    pub last: JsMap<GuardId, Option<V3>>,
    pub in_t: f64,
    pub close: Option<bool>,
    pub repick_t: Option<f64>,
    pub repick_bad: Option<JsSet<GuardId>>,
    pub redealt: Option<JsSet<GuardId>>,
    pub repick_n: Option<f64>,
    pub repick_kept: Option<JsMap<String, f64>>,
}

#[derive(Clone, Debug, Default)]
pub struct WallChargeRowsRowOver {
    pub kind: Option<crate::game::InteractableKind>,
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

#[derive(Clone, Debug)]
pub struct WallChargeUnderFire {
    pub score: crate::wallcharge::WallChargeFireAsk,
}
impl Default for WallChargeUnderFire {
    fn default() -> Self {
        WallChargeUnderFire {
            score: crate::wallcharge::WallChargeFireAsk::Lines { wall: None, all: Vec::new(), memo: JsMap::new(), hit: JsMap::new() },
        }
    }
}

#[derive(Clone, Debug)]
pub enum WallChargePlan {
    Text(String),
    Arm1(crate::wallcharge::WallChargePlanArm1),
}

#[derive(Clone, Debug, Default)]
pub struct WallChargePlanArm1 {
    pub lead: GuardId,
    pub spot: V3,
    pub deal: Vec<(GuardId, V3)>,
    pub close: bool,
}

#[derive(Clone, Debug)]
pub enum WallChargeCanOf {
    Text(String),
    Bool(bool),
}

#[derive(Clone, Debug)]
pub enum WallChargeValid {
    Text(String),
    Bool(bool),
}

#[derive(Clone, Debug, Default)]
pub struct WallChargeAssignA {
    pub post: Option<V3>,
    pub going: String,
    pub face: Option<f64>,
    pub there: String,
    pub order: Option<String>,
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

#[derive(Clone, Debug)]
pub enum WallChargeIssue {
    Text(String),
    Arm1(()),
}

#[derive(Clone, Copy, Debug)]
pub enum WallChargeOrderWho {
    Num(f64),
    All,
    Red,
    Blue,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct WallChargeBlow {
    pub hole: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct WallChargeBlowRaw {
    pub hole: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct WallChargeHoleAt {
    pub du: f64,
    pub half: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct WallChargeSamJob {
    pub p: V3,
    pub n: V3,
    pub at: V3,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct WallchargeMod {
    pub sky_memo: Option<JsMap<crate::Num, f64>>,
}

pub const FACE: f64 = 0.06;

pub const SKY_TOP: f64 = 12.0;

pub fn sky_off() -> Vec<f64> { alloc::vec![0.5, 1.0, 1.6] }

pub fn sky_along() -> Vec<f64> { alloc::vec![-0.5, 0.0, 0.5] }

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes)]
pub enum WallStage {
    Lane,
    Pod,
}
