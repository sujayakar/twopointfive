//! The types of `wallcharge`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// a walk's answer kept for the open menu (wallcharge.ts keptWalks): a 'w|' key's length of walk (-1: no way), an 'r|' key's refusal from SquadOrders.reach (None: he can walk it)
#[derive(Clone, Debug, ToBytes)]
pub enum WallChargeWalkAnswer {
    Walk { len: f64 },
    Reach { why: Option<String> },
}
/// whose store a walk's answer is kept in (wallcharge.ts type Kept): the open menu's (g.wall_charge.walks, counted in g.wall_charge.keep), one order's re-deal (WallOp.repick_kept), or one call's own (clearOf)
#[derive(Clone, Debug)]
pub enum WallChargeKept {
    Menu,
    Repick { op: WallOpId },
    Local { memo: JsMap<String, f64> },
}
/// what underFire's two answers close over (wallcharge.ts underFire): the wall about to go, the foes as known when it was asked, and each place's answer so far (a place by its bits: the TypeScript keeps them by the array's identity)
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

#[derive(Clone, Debug, ToBytes)]
pub struct WallCharge {
    pub knobs: crate::knobs::WallchargeK,
    pub ops: Vec<WallOpId>,
    pub done: Vec<crate::wallcharge::WallChargeDoneItem>,
    pub set: Vec<WallChargeId>,
    pub stats: crate::wallcharge::WallChargeStats,
    pub away: JsMap<GuardId, crate::wallcharge::WallChargeAwayValue>,
    pub away_marks: JsMap<WallChargeId, crate::wallcharge::WallChargeAwayMarksValue>,
    pub away_none: JsMap<GuardId, f64>,
    pub log: Vec<crate::wallcharge::WallChargeLogItem>,
    pub opener: OrdersOrderDefId,
    pub row: OrdersOrderDefId,
    pub seated: bool,
    pub mode: String,
    pub player: Option<PlayerId>,
    pub roster: Vec<GuardId>,
    pub alone: f64,
    pub sam: Option<crate::wallcharge::WallChargeSam>,
    pub offered: Option<crate::wallcharge::WallSpot>,
    pub carried: Option<f64>,
    pub brought: Option<f64>,
    pub free_set: bool,
    /// (scratch, read by nobody who did not just write it: in no bytes)
    #[bytes(skip)]
    pub nearby: Vec<i32>,
    pub walks: Option<crate::wallcharge::WallChargeWalks>,
    pub keep: crate::wallcharge::WallChargeKeep,
    pub pressed: bool,
    pub last_blow: Option<crate::wallcharge::WallChargeLastBlow>,
    pub marker: Option<crate::game::Interactable>,
}
impl WallCharge {
    /// what a new one holds
    pub fn blank() -> Self {
        WallCharge {
            knobs: crate::knobs::WallchargeK { menu: crate::knobs::WallchargeKMenu::Page, pick_m: 0.06, height: 1.05, end_m: 0.75, slide: alloc::vec![0.0, 0.5, -0.5, 1.0, -1.0, 1.5, -1.5], set_off: 0.5, set_near: 0.9, sam_reach: 1.35, sam_shift: 0.35, sam_marker_m: 3.0, standoff: alloc::vec![2.6, 2.0], post_off: 0.55, file_gap: 0.9, back_m: alloc::vec![3.0, 3.9, 4.8], clear_m: 0.45, mate_m: 2.0, after_blow: 0.7, enter_cap: 16.0, sweep_secs: 1.5, mouth_in: 0.6, lane: 0.35, lane_watch: 3.0, pods: alloc::vec![[-1.3, 1.9], [1.3, 1.9], [-2.1, 1.0], [2.1, 1.0]], cost: 2.0, carried: None, hold_secs: 1.5, mate_line: 0.6, off_frame: 0.8, off_frame_pod: 0.5, outer_thick: 0.26, outer_min_len: 1.5, far_slack: 0.5, far_reach: 0.375, hung_m: 0.35, safe_standoff: [4.0, 6.0], safe_reach: [3.0, 5.0], safe_secs: [3.0, 1.0], safe_hide_m: 9.0, safe_walk_m: 16.0, safe_back_m: 9.0, fire_known_s: 30.0, fire_m: 30.0, fire_step: 0.5, fire_grain: 0.03, men_clear_in: 0.1, men_clear_s: 0.75, sam_secs: [2.0, 0.6], sam_sup: 0.5 },
            ops: Vec::new(),
            done: Vec::new(),
            set: Vec::new(),
            stats: crate::wallcharge::WallChargeStats { orders: 0.0, set: 0.0, by_sam: 0.0, blown: 0.0, holes: 0.0, duds: 0.0, knocked: 0.0, stunned: 0.0, backblast: 0.0, entries: 0.0, back: 0.0, cleared: 0.0 },
            away: JsMap::new(),
            away_marks: JsMap::new(),
            away_none: JsMap::new(),
            log: Vec::new(),
            opener: OrdersOrderDefId(0),
            row: OrdersOrderDefId(0),
            seated: false,
            mode: String::from(""),
            player: None,
            roster: Vec::new(),
            alone: 0.0,
            sam: None,
            offered: None,
            carried: None,
            brought: None,
            free_set: false,
            nearby: alloc::vec![0; 48],
            walks: None,
            keep: crate::wallcharge::WallChargeKeep { on: true, asks: 0.0, hits: 0.0 },
            pressed: false,
            last_blow: None,
            marker: None,
        }
    }
}
impl Default for WallCharge { fn default() -> Self { Self::blank() } }

#[derive(Clone, Debug, ToBytes, Default)]
pub struct WallChargeDoneItem {
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
    pub stage: JsMap<GuardId, crate::wallcharge::WallChargeDoneItemStageValue>,
    pub pod: JsMap<GuardId, crate::wallcharge::Pod>,
    pub crossed: JsMap<GuardId, f64>,
    pub last: JsMap<GuardId, Option<V3>>,
    pub in_t: f64,
    pub beats: Vec<crate::wallcharge::WallChargeDoneItemBeatsItem>,
    pub close: Option<bool>,
    pub repick_t: Option<f64>,
    pub repick_bad: Option<JsSet<GuardId>>,
    pub redealt: Option<JsSet<GuardId>>,
    pub repick_n: Option<f64>,
    pub repick_kept: Option<JsMap<String, f64>>,
    pub ended: String,
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

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes)]
pub enum WallChargeDoneItemStageValue {
    Lane,
    Pod,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct WallChargeDoneItemBeatsItem {
    pub t: f64,
    pub what: String,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct WallChargeStats {
    pub orders: f64,
    pub set: f64,
    pub by_sam: f64,
    pub blown: f64,
    pub holes: f64,
    pub duds: f64,
    pub knocked: f64,
    pub stunned: f64,
    pub backblast: f64,
    pub entries: f64,
    pub back: f64,
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
    pub indoors: bool,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct WallChargeLogItem {
    pub t: f64,
    pub what: String,
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
    pub asks: f64,
    pub hits: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct WallChargeLastBlow {
    pub reach: f64,
    pub indoors: bool,
    pub sam: f64,
    pub lost: f64,
    pub sam_down: bool,
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
    pub stage: JsMap<GuardId, crate::wallcharge::WallOpStageValue>,
    pub pod: JsMap<GuardId, crate::wallcharge::Pod>,
    pub crossed: JsMap<GuardId, f64>,
    pub last: JsMap<GuardId, Option<V3>>,
    pub in_t: f64,
    pub beats: Vec<crate::wallcharge::WallChargeDoneItemBeatsItem>,
    pub close: Option<bool>,
    pub repick_t: Option<f64>,
    pub repick_bad: Option<JsSet<GuardId>>,
    pub redealt: Option<JsSet<GuardId>>,
    pub repick_n: Option<f64>,
    pub repick_kept: Option<JsMap<String, f64>>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes)]
pub enum WallOpStageValue {
    Lane,
    Pod,
}

#[derive(Clone, Debug, Default)]
pub struct WallChargeRowsRowOver {
    pub kind: Option<crate::wallcharge::WallChargeRowsRowOverKind>,
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

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum WallChargeRowsRowOverKind {
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
impl WallChargeRowsRowOverKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            WallChargeRowsRowOverKind::Cam => "cam",
            WallChargeRowsRowOverKind::Guard => "guard",
            WallChargeRowsRowOverKind::Breaker => "breaker",
            WallChargeRowsRowOverKind::Door => "door",
            WallChargeRowsRowOverKind::Light => "light",
            WallChargeRowsRowOverKind::Body => "body",
            WallChargeRowsRowOverKind::Objective => "objective",
            WallChargeRowsRowOverKind::Held => "held",
            WallChargeRowsRowOverKind::Wall => "wall",
            WallChargeRowsRowOverKind::Switch => "switch",
            WallChargeRowsRowOverKind::Charge => "charge",
            WallChargeRowsRowOverKind::Cancel => "cancel",
        }
    }
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
    pub knocked: f64,
    pub stunned: f64,
    pub backblast: f64,
    pub lost: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct WallChargeBlowRaw {
    pub hole: bool,
    pub knocked: f64,
    pub stunned: f64,
    pub backblast: f64,
    pub lost: f64,
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
#[derive(Clone, Debug, ToBytes)]
pub struct WallchargeMod {
    /// (src/game/wallcharge.ts:302 skyMemo)
    pub sky_memo: Option<JsMap<crate::Num, f64>>,
}
impl WallchargeMod {
    pub fn blank() -> Self {
        WallchargeMod {
            sky_memo: None,
        }
    }
}

/// (src/game/wallcharge.ts:284 FACE)
pub const FACE: f64 = 0.06;

/// (src/game/wallcharge.ts:301 SKY_TOP)
pub const SKY_TOP: f64 = 12.0;

/// (src/game/wallcharge.ts:301 SKY_OFF: a table nothing writes)
pub fn sky_off() -> Vec<f64> { alloc::vec![0.5, 1.0, 1.6] }

/// (src/game/wallcharge.ts:301 SKY_ALONG: a table nothing writes)
pub fn sky_along() -> Vec<f64> { alloc::vec![-0.5, 0.0, 0.5] }
