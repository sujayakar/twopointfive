//! The types of `wallcharge`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// a walk's answer kept for the open menu (wallcharge.ts keptWalks): a 'w|' key's length of walk (-1: no way), an 'r|' key's refusal from SquadOrders.reach (None: he can walk it)
#[derive(Clone, Debug)]
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

/// (TypeScript: WallSpot, src/game/wallcharge.ts:248)
#[derive(Clone, Debug)]
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
impl WallSpot {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WallSpot {
            index: 0.0,
            box_: BoxId(0),
            along_x: false,
            u: 0.0,
            side: 0.0,
            ok: false,
            why: String::new(),
            outer: None,
        }
    }
}
impl Default for WallSpot { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: WallCharge, src/game/wallcharge.ts:398)
#[derive(Clone, Debug)]
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
    pub nearby: Vec<i32>,
    pub walks: Option<crate::wallcharge::WallChargeWalks>,
    pub keep: crate::wallcharge::WallChargeKeep,
    pub pressed: bool,
    pub last_blow: Option<crate::wallcharge::WallChargeLastBlow>,
    pub marker: Option<crate::game::Interactable>,
}
impl WallCharge {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        WallCharge {
            knobs: crate::knobs::WallchargeK { on: true, menu: crate::knobs::WallchargeKMenu::Page, pick_m: 0.06, height: 1.05, end_m: 0.75, slide: alloc::vec![0.0, 0.5, -0.5, 1.0, -1.0, 1.5, -1.5], set_off: 0.5, set_near: 0.9, sam_reach: 1.35, sam_shift: 0.35, sam_marker_m: 3.0, standoff: alloc::vec![2.6, 2.0], post_off: 0.55, file_gap: 0.9, back_m: alloc::vec![3.0, 3.9, 4.8], clear_m: 0.45, brace: true, mate_m: 2.0, after_blow: 0.7, enter_cap: 16.0, sweep_secs: 1.5, mouth_in: 0.6, lane: 0.35, lane_watch: 3.0, pods: alloc::vec![[-1.3, 1.9], [1.3, 1.9], [-2.1, 1.0], [2.1, 1.0]], hole: false, doors: true, hold_late: true, cost: 2.0, carried: None, hold_each: true, hold_secs: 1.5, mate_line: 0.6, off_frame: 0.8, off_frame_pod: 0.5, off_holes: true, outer: false, outer_thick: 0.26, outer_min_len: 1.5, outer_yard: true, far_furn: false, far_slack: 0.5, far_reach: 0.375, far_mouth: true, safe: false, hung: false, hung_m: 0.35, safe_standoff: [4.0, 6.0], safe_reach: [3.0, 5.0], safe_secs: [3.0, 1.0], safe_hide_m: 9.0, safe_walk_m: 16.0, safe_back: true, safe_back_m: 9.0, out_of_fire: false, fire_known_s: 30.0, fire_m: 30.0, fire_step: 0.5, fire_grain: 0.03, men_clear: false, men_clear_in: 0.1, men_clear_s: 0.75, sam_secs: [2.0, 0.6], sam_sup: 0.5 },
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

/// (TypeScript: inline at src/game/wallcharge.ts:400 WallCharge.done, src/game/wallcharge.ts:400 WallCharge.done)
#[derive(Clone, Debug)]
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
impl WallChargeDoneItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WallChargeDoneItem {
            wall: crate::wallcharge::WallSpot::zeroed(),
            men: Vec::new(),
            lead: GuardId(0),
            phase: crate::wallcharge::Phase::Wait,
            t0: 0.0,
            spot: [0.0; 3],
            set_t: 0.0,
            stow_off_t: None,
            owes: None,
            mine: JsMap::new(),
            post: JsMap::new(),
            from: JsMap::new(),
            hole: None,
            queue: Vec::new(),
            stage: JsMap::new(),
            pod: JsMap::new(),
            crossed: JsMap::new(),
            last: JsMap::new(),
            in_t: 0.0,
            beats: Vec::new(),
            close: None,
            repick_t: None,
            repick_bad: None,
            redealt: None,
            repick_n: None,
            repick_kept: None,
            ended: String::new(),
        }
    }
}
impl Default for WallChargeDoneItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Phase {
    Wait,
    Enter,
    Place,
    Standoff,
    Armed,
}
impl Phase {
    pub const fn as_str(self) -> &'static str {
        match self {
            Phase::Wait => "wait",
            Phase::Enter => "enter",
            Phase::Place => "place",
            Phase::Standoff => "standoff",
            Phase::Armed => "armed",
        }
    }
}

/// (TypeScript: inline at src/game/wallcharge.ts:268 WallChargeDoneItem.hole, src/game/wallcharge.ts:268)
#[derive(Clone, Copy, Debug)]
pub struct WallChargeDoneItemHole {
    pub du: f64,
    pub half: f64,
}
impl WallChargeDoneItemHole {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WallChargeDoneItemHole {
            du: 0.0,
            half: 0.0,
        }
    }
}
impl Default for WallChargeDoneItemHole { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/wallcharge.ts:270 WallChargeDoneItem.queue, src/game/wallcharge.ts:270)
#[derive(Clone, Debug)]
pub struct WallChargeDoneItemQueueItem {
    pub gd: GuardId,
    pub at: f64,
    pub lane: V3,
    pub pod: crate::wallcharge::Pod,
}
impl WallChargeDoneItemQueueItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WallChargeDoneItemQueueItem {
            gd: GuardId(0),
            at: 0.0,
            lane: [0.0; 3],
            pod: crate::wallcharge::Pod::zeroed(),
        }
    }
}
impl Default for WallChargeDoneItemQueueItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Pod, src/game/wallcharge.ts:256)
#[derive(Clone, Debug)]
pub struct Pod {
    pub pos: V3,
    pub face: f64,
    pub watch: Option<V3>,
    pub door: Option<String>,
    pub hold: Option<crate::wallcharge::PodHold>,
    pub took: Option<bool>,
}
impl Pod {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Pod {
            pos: [0.0; 3],
            face: 0.0,
            watch: None,
            door: None,
            hold: None,
            took: None,
        }
    }
}
impl Default for Pod { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/wallcharge.ts:256 Pod.hold, src/game/wallcharge.ts:256)
#[derive(Clone, Debug)]
pub struct PodHold {
    pub face: f64,
    pub watch: V3,
    pub door: Option<String>,
}
impl PodHold {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PodHold {
            face: 0.0,
            watch: [0.0; 3],
            door: None,
        }
    }
}
impl Default for PodHold { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum WallChargeDoneItemStageValue {
    Lane,
    Pod,
}
impl WallChargeDoneItemStageValue {
    pub const fn as_str(self) -> &'static str {
        match self {
            WallChargeDoneItemStageValue::Lane => "lane",
            WallChargeDoneItemStageValue::Pod => "pod",
        }
    }
}

/// (TypeScript: inline at src/game/wallcharge.ts:271 WallChargeDoneItem.beats, src/game/wallcharge.ts:271)
#[derive(Clone, Debug)]
pub struct WallChargeDoneItemBeatsItem {
    pub t: f64,
    pub what: String,
}
impl WallChargeDoneItemBeatsItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WallChargeDoneItemBeatsItem {
            t: 0.0,
            what: String::new(),
        }
    }
}
impl Default for WallChargeDoneItemBeatsItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/wallcharge.ts:401 WallCharge.stats, src/game/wallcharge.ts:401)
#[derive(Clone, Copy, Debug)]
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
impl WallChargeStats {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WallChargeStats {
            orders: 0.0,
            set: 0.0,
            by_sam: 0.0,
            blown: 0.0,
            holes: 0.0,
            duds: 0.0,
            knocked: 0.0,
            stunned: 0.0,
            backblast: 0.0,
            entries: 0.0,
            back: 0.0,
            cleared: 0.0,
        }
    }
}
impl Default for WallChargeStats { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/wallcharge.ts:403 WallCharge.away, src/game/wallcharge.ts:403)
#[derive(Clone, Copy, Debug)]
pub struct WallChargeAwayValue {
    pub c: WallChargeId,
    pub at: V3,
    pub hit_t: f64,
    pub t: f64,
}
impl WallChargeAwayValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WallChargeAwayValue {
            c: WallChargeId(0),
            at: [0.0; 3],
            hit_t: 0.0,
            t: 0.0,
        }
    }
}
impl Default for WallChargeAwayValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/wallcharge.ts:404 WallCharge.awayMarks, src/game/wallcharge.ts:404)
#[derive(Clone, Debug)]
pub struct WallChargeAwayMarksValue {
    pub n: f64,
    pub marks: Vec<V3>,
    pub t: f64,
    pub sets: f64,
    pub r: crate::wallcharge::WallChargeAwayMarksValueR,
    pub pad: crate::wallcharge::WallChargeAwayMarksValueR,
}
impl WallChargeAwayMarksValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WallChargeAwayMarksValue {
            n: 0.0,
            marks: Vec::new(),
            t: 0.0,
            sets: 0.0,
            r: crate::wallcharge::WallChargeAwayMarksValueR::zeroed(),
            pad: crate::wallcharge::WallChargeAwayMarksValueR::zeroed(),
        }
    }
}
impl Default for WallChargeAwayMarksValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/wallcharge.ts:404 WallChargeAwayMarksValue.R, src/game/wallcharge.ts:342)
#[derive(Clone, Copy, Debug)]
pub struct WallChargeAwayMarksValueR {
    pub standoff: f64,
    pub reach: f64,
    pub indoors: bool,
}
impl WallChargeAwayMarksValueR {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WallChargeAwayMarksValueR {
            standoff: 0.0,
            reach: 0.0,
            indoors: false,
        }
    }
}
impl Default for WallChargeAwayMarksValueR { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/wallcharge.ts:406 WallCharge.log, src/game/wallcharge.ts:406)
#[derive(Clone, Debug)]
pub struct WallChargeLogItem {
    pub t: f64,
    pub what: String,
}
impl WallChargeLogItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WallChargeLogItem {
            t: 0.0,
            what: String::new(),
        }
    }
}
impl Default for WallChargeLogItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/wallcharge.ts:412 WallCharge.sam, src/game/wallcharge.ts:412)
#[derive(Clone, Debug)]
pub struct WallChargeSam {
    pub wall: crate::wallcharge::WallSpot,
    pub t0: f64,
    pub at: V3,
}
impl WallChargeSam {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WallChargeSam {
            wall: crate::wallcharge::WallSpot::zeroed(),
            t0: 0.0,
            at: [0.0; 3],
        }
    }
}
impl Default for WallChargeSam { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/wallcharge.ts:818 WallCharge.walks, src/game/wallcharge.ts:818)
#[derive(Clone, Debug)]
pub struct WallChargeWalks {
    pub menu: u32,
    pub stamp: String,
    pub map: JsMap<String, crate::wallcharge::WallChargeWalkAnswer>,
}
impl WallChargeWalks {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WallChargeWalks {
            menu: 0,
            stamp: String::new(),
            map: JsMap::new(),
        }
    }
}
impl Default for WallChargeWalks { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at quiet WallChargeWalks.menu, quiet WallChargeWalks.menu)
#[derive(Clone, Copy, Debug)]
pub struct WallChargeWalksMenu {
}
impl WallChargeWalksMenu {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WallChargeWalksMenu {
        }
    }
}
impl Default for WallChargeWalksMenu { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/wallcharge.ts:820 WallCharge.keep, src/game/wallcharge.ts:820)
#[derive(Clone, Copy, Debug)]
pub struct WallChargeKeep {
    pub on: bool,
    pub asks: f64,
    pub hits: f64,
}
impl WallChargeKeep {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WallChargeKeep {
            on: false,
            asks: 0.0,
            hits: 0.0,
        }
    }
}
impl Default for WallChargeKeep { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/wallcharge.ts:1130 WallCharge.lastBlow, src/game/wallcharge.ts:1130)
#[derive(Clone, Copy, Debug)]
pub struct WallChargeLastBlow {
    pub reach: f64,
    pub indoors: bool,
    pub sam: f64,
    pub lost: f64,
    pub sam_down: bool,
}
impl WallChargeLastBlow {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WallChargeLastBlow {
            reach: 0.0,
            indoors: false,
            sam: 0.0,
            lost: 0.0,
            sam_down: false,
        }
    }
}
impl Default for WallChargeLastBlow { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: SetWallCharge, src/game/wallcharge.ts:251)
#[derive(Clone, Debug)]
pub struct SetWallCharge {
    pub wall: crate::wallcharge::WallSpot,
    pub t: f64,
    pub by: String,
    pub paid: f64,
    pub free: Option<bool>,
}
impl SetWallCharge {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SetWallCharge {
            wall: crate::wallcharge::WallSpot::zeroed(),
            t: 0.0,
            by: String::new(),
            paid: 0.0,
            free: None,
        }
    }
}
impl Default for SetWallCharge { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: WallOp, src/game/wallcharge.ts:257)
#[derive(Clone, Debug)]
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
impl WallOp {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WallOp {
            wall: crate::wallcharge::WallSpot::zeroed(),
            men: Vec::new(),
            lead: GuardId(0),
            phase: crate::wallcharge::Phase::Wait,
            t0: 0.0,
            spot: [0.0; 3],
            set_t: 0.0,
            stow_off_t: None,
            owes: None,
            mine: JsMap::new(),
            post: JsMap::new(),
            from: JsMap::new(),
            hole: None,
            queue: Vec::new(),
            stage: JsMap::new(),
            pod: JsMap::new(),
            crossed: JsMap::new(),
            last: JsMap::new(),
            in_t: 0.0,
            beats: Vec::new(),
            close: None,
            repick_t: None,
            repick_bad: None,
            redealt: None,
            repick_n: None,
            repick_kept: None,
        }
    }
}
impl Default for WallOp { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum WallOpStageValue {
    Lane,
    Pod,
}
impl WallOpStageValue {
    pub const fn as_str(self) -> &'static str {
        match self {
            WallOpStageValue::Lane => "lane",
            WallOpStageValue::Pod => "pod",
        }
    }
}

/// (TypeScript: inline at src/game/wallcharge.ts:396 wallChargeRows(row), ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1583)
#[derive(Clone, Debug)]
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
impl WallChargeRowsRowOver {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WallChargeRowsRowOver {
            kind: None,
            pos: None,
            line1: None,
            line2: None,
            in_reach: None,
            off: None,
            progress: None,
            target: None,
            door: None,
            guard: None,
            verb: None,
            running: None,
            close: None,
            past: None,
            framed: None,
        }
    }
}
impl Default for WallChargeRowsRowOver { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
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

/// (TypeScript: inline at src/game/wallcharge.ts:670 underFire ->, src/game/wallcharge.ts:670)
#[derive(Clone, Debug)]
pub struct WallChargeUnderFire {
    pub score: crate::wallcharge::WallChargeFireAsk,
}
impl WallChargeUnderFire {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WallChargeUnderFire {
            score: crate::wallcharge::WallChargeFireAsk::Lines { wall: None, all: Vec::new(), memo: JsMap::new(), hit: JsMap::new() },
        }
    }
}
impl Default for WallChargeUnderFire { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: a union)
#[derive(Clone, Debug)]
pub enum WallChargePlan {
    Text(String),
    Arm1(crate::wallcharge::WallChargePlanArm1),
}

/// (TypeScript: inline at src/game/wallcharge.ts:765 plan ->, src/game/wallcharge.ts:765)
#[derive(Clone, Debug)]
pub struct WallChargePlanArm1 {
    pub lead: GuardId,
    pub spot: V3,
    pub deal: Vec<(GuardId, V3)>,
    pub close: bool,
}
impl WallChargePlanArm1 {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WallChargePlanArm1 {
            lead: GuardId(0),
            spot: [0.0; 3],
            deal: Vec::new(),
            close: false,
        }
    }
}
impl Default for WallChargePlanArm1 { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: a union)
#[derive(Clone, Debug)]
pub enum WallChargeCanOf {
    Text(String),
    Bool(bool),
}

/// (TypeScript: a union)
#[derive(Clone, Debug)]
pub enum WallChargeValid {
    Text(String),
    Bool(bool),
}

/// (TypeScript: inline at src/game/wallcharge.ts:832 assign(a), src/game/wallcharge.ts:832 assign(a))
#[derive(Clone, Debug)]
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
impl WallChargeAssignA {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WallChargeAssignA {
            post: None,
            going: String::new(),
            face: None,
            there: String::new(),
            order: None,
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
impl Default for WallChargeAssignA { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: a union)
#[derive(Clone, Debug)]
pub enum WallChargeIssue {
    Text(String),
    Arm1(()),
}

/// (TypeScript: a union)
#[derive(Clone, Copy, Debug)]
pub enum WallChargeOrderWho {
    Num(f64),
    All,
    Red,
    Blue,
}

/// (TypeScript: inline at src/game/wallcharge.ts:1063 blow ->, src/game/wallcharge.ts:1063)
#[derive(Clone, Copy, Debug)]
pub struct WallChargeBlow {
    pub hole: bool,
    pub knocked: f64,
    pub stunned: f64,
    pub backblast: f64,
    pub lost: f64,
}
impl WallChargeBlow {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WallChargeBlow {
            hole: false,
            knocked: 0.0,
            stunned: 0.0,
            backblast: 0.0,
            lost: 0.0,
        }
    }
}
impl Default for WallChargeBlow { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/wallcharge.ts:1064 blowRaw ->, src/game/wallcharge.ts:1064)
#[derive(Clone, Copy, Debug)]
pub struct WallChargeBlowRaw {
    pub hole: bool,
    pub knocked: f64,
    pub stunned: f64,
    pub backblast: f64,
    pub lost: f64,
}
impl WallChargeBlowRaw {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WallChargeBlowRaw {
            hole: false,
            knocked: 0.0,
            stunned: 0.0,
            backblast: 0.0,
            lost: 0.0,
        }
    }
}
impl Default for WallChargeBlowRaw { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/wallcharge.ts:1143 holeAt ->, src/game/wallcharge.ts:1143)
#[derive(Clone, Copy, Debug)]
pub struct WallChargeHoleAt {
    pub du: f64,
    pub half: f64,
}
impl WallChargeHoleAt {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WallChargeHoleAt {
            du: 0.0,
            half: 0.0,
        }
    }
}
impl Default for WallChargeHoleAt { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/wallcharge.ts:1191 samRows(row), ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1583)
#[derive(Clone, Debug)]
pub struct WallChargeSamRowsRowOver {
    pub kind: Option<crate::wallcharge::WallChargeSamRowsRowOverKind>,
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
impl WallChargeSamRowsRowOver {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WallChargeSamRowsRowOver {
            kind: None,
            pos: None,
            line1: None,
            line2: None,
            in_reach: None,
            off: None,
            progress: None,
            target: None,
            door: None,
            guard: None,
            verb: None,
            running: None,
            close: None,
            past: None,
            framed: None,
        }
    }
}
impl Default for WallChargeSamRowsRowOver { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum WallChargeSamRowsRowOverKind {
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
impl WallChargeSamRowsRowOverKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            WallChargeSamRowsRowOverKind::Cam => "cam",
            WallChargeSamRowsRowOverKind::Guard => "guard",
            WallChargeSamRowsRowOverKind::Breaker => "breaker",
            WallChargeSamRowsRowOverKind::Door => "door",
            WallChargeSamRowsRowOverKind::Light => "light",
            WallChargeSamRowsRowOverKind::Body => "body",
            WallChargeSamRowsRowOverKind::Objective => "objective",
            WallChargeSamRowsRowOverKind::Held => "held",
            WallChargeSamRowsRowOverKind::Wall => "wall",
            WallChargeSamRowsRowOverKind::Switch => "switch",
            WallChargeSamRowsRowOverKind::Charge => "charge",
            WallChargeSamRowsRowOverKind::Cancel => "cancel",
        }
    }
}

/// (TypeScript: inline at src/game/wallcharge.ts:1223 samJob ->, src/game/wallcharge.ts:1223)
#[derive(Clone, Copy, Debug)]
pub struct WallChargeSamJob {
    pub p: V3,
    pub n: V3,
    pub at: V3,
}
impl WallChargeSamJob {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WallChargeSamJob {
            p: [0.0; 3],
            n: [0.0; 3],
            at: [0.0; 3],
        }
    }
}
impl Default for WallChargeSamJob { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct WallchargeMod {
    /// (src/game/wallcharge.ts:301 skyMemo)
    pub sky_memo: Option<JsMap<crate::Num, f64>>,
    /// (src/game/wallcharge.ts:302 standIns)
    pub stand_ins: JsMap<OrdersOrderDefId, OrdersOrderDefId>,
}
impl WallchargeMod {
    pub fn blank() -> Self {
        WallchargeMod {
            sky_memo: None,
            stand_ins: JsMap::new(),
        }
    }
}

/// (src/game/wallcharge.ts:211 WALLCHARGE_DEFAULTS: a table nothing writes)
pub fn wallcharge_defaults() -> crate::knobs::WallchargeK { crate::knobs::page_wallcharge() }

/// (src/game/wallcharge.ts:241 WALL_PAGE)
pub const WALL_PAGE: &str = "walltools";

/// (src/game/wallcharge.ts:243 WALL_ORDER)
pub const WALL_ORDER: &str = "wallCharge";

/// (src/game/wallcharge.ts:283 FACE)
pub const FACE: f64 = 0.06;

/// (src/game/wallcharge.ts:300 SKY_TOP)
pub const SKY_TOP: f64 = 12.0;

/// (src/game/wallcharge.ts:300 SKY_OFF: a table nothing writes)
pub fn sky_off() -> Vec<f64> { alloc::vec![0.5, 1.0, 1.6] }

/// (src/game/wallcharge.ts:300 SKY_ALONG: a table nothing writes)
pub fn sky_along() -> Vec<f64> { alloc::vec![-0.5, 0.0, 0.5] }
