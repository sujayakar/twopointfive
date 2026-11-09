//! The types of `doortools`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// the step onto the diagonals, kept until the first can's burst has run dry (doortools.ts toThreshold's `out`)
#[derive(Clone, Debug, ToBytes)]
pub enum ThresholdOut {
    Step { lead: GuardId, f: Option<crate::doortools::DoorToolsThrFar>, sl: crate::doortools::DoorToolsDoneItemSliceValue, sf: Option<crate::doortools::DoorToolsDoneItemSliceValue>, light: bool, name: String },
}
/// walkLen for one deal's worth of questions, each pair of places measured once (doortools.ts walks); held in a local of the deal, never in a field
#[derive(Clone, Debug)]
pub enum Walks {
    Memo { memo: JsMap<String, f64>, lim: f64 },
}

#[derive(Clone, Debug, ToBytes)]
pub struct DoorTools {
    pub charges: f64,
    pub bangs: f64,
    pub frags: f64,
    pub frag_next: bool,
    pub big_next: bool,
    pub big_row: Option<bool>,
    pub ops: Vec<DoorOpId>,
    pub done: Vec<DoorOpId>,
    pub set: Vec<ChargeId>,
    pub linked: Vec<DoorId>,
    pub lights: Option<bool>,
    pub log: Vec<crate::doortools::DoorToolsLogItem>,
    pub stats: crate::doortools::DoorToolsStats,
    pub roster: Vec<GuardId>,
    pub seen: Option<crate::floorseen::FloorSeen>,
    pub seen_watch: bool,
    pub led: crate::doortools::DoorToolsLed,
    pub go_key: Option<Option<String>>,
    pub words: Vec<WordId>,
    pub word_key_t: f64,
    pub thr_key: Option<String>,
    pub thr_sig: f64,
    pub sam_of: Option<PlayerId>,
    pub sam_spent: f64,
    pub tried: JsMap<DoorId, f64>,
    pub gate_asked: JsSet<DoorOpId>,
    pub held: JsMap<GuardId, crate::doortools::DoorToolsHeldValue>,
    pub watches: Vec<DoorOpId>,
    pub away_why: String,
    pub away_far: bool,
    pub big_said: JsSet<DoorOpId>,
}
impl DoorTools {
    /// what a new one holds
    pub fn blank() -> Self {
        DoorTools {
            charges: 2.0,
            bangs: 3.0,
            frags: 2.0,
            frag_next: false,
            big_next: false,
            big_row: None,
            ops: Vec::new(),
            done: Vec::new(),
            set: Vec::new(),
            linked: Vec::new(),
            lights: None,
            log: Vec::new(),
            stats: crate::doortools::DoorToolsStats { orders: 0.0, opened: 0.0, kicks: 0.0, shot: 0.0, picks: 0.0, no_boot: 0.0, hand_kept: 0.0, tested: 0.0, halted: 0.0, bangs: 0.0, own_flashed: 0.0, frags: 0.0, blown: 0.0, knocked: 0.0, stunned: 0.0, backblast: 0.0, entries: 0.0, words: 0.0, stepped_back: 0.0, pods_moved: 0.0, sectors: 0.0, sent_on: 0.0, sent_alone: 0.0, door_alone: 0.0, sent_pair: 0.0, not_sent: 0.0, flowed: 0.0, closed_up: 0.0, flow_late: 0.0 },
            roster: Vec::new(),
            seen: None,
            seen_watch: false,
            led: crate::doortools::DoorToolsLed { entries: 0.0, by_seen: 0.0 },
            go_key: None,
            words: Vec::new(),
            word_key_t: -1000000000.0,
            thr_key: None,
            thr_sig: -1.0,
            sam_of: None,
            sam_spent: 0.0,
            tried: JsMap::new(),
            gate_asked: JsSet::new(),
            held: JsMap::new(),
            watches: Vec::new(),
            away_why: String::from(""),
            away_far: false,
            big_said: JsSet::new(),
        }
    }
}
impl Default for DoorTools { fn default() -> Self { Self::blank() } }

#[derive(Clone, Debug, Default)]
pub struct DoorToolsDoneItem {
    pub kind: crate::doortools::DoorToolKind,
    pub door: DoorId,
    pub side: f64,
    pub coded: Option<bool>,
    pub men: Vec<GuardId>,
    pub lead: GuardId,
    pub fled: Option<JsSet<GuardId>>,
    pub fled_w: Option<f64>,
    pub fled_t: Option<f64>,
    pub fled_said: Option<bool>,
    pub fled_go: Option<bool>,
    pub rear: JsSet<GuardId>,
    pub lane: Option<crate::doortools::DoorToolsDoneItemLane>,
    pub from: JsMap<GuardId, V3>,
    pub phase: crate::doortools::Phase,
    pub t0: f64,
    pub bang_held: bool,
    pub frag: Option<bool>,
    pub frag_way_t: Option<f64>,
    pub leaf_a: Option<f64>,
    pub leaf_t: Option<f64>,
    pub mine: JsMap<GuardId, OrdersAssignmentId>,
    pub post: JsMap<GuardId, crate::orders::StackPost>,
    pub kick: Option<crate::doortools::DoorToolsDoneItemKick>,
    pub no_gun: Option<bool>,
    pub big: Option<bool>,
    pub open_t: f64,
    pub can: Option<ItemId>,
    pub toss_t: f64,
    pub land_t: f64,
    pub bang_t: f64,
    pub bang_at: Option<V3>,
    pub queue: Vec<crate::doortools::DoorToolsDoneItemQueueItem>,
    pub set_t: f64,
    pub stow_off_t: Option<f64>,
    pub pod: JsMap<GuardId, crate::entrypoints::Pod>,
    pub on_pod: JsMap<GuardId, f64>,
    pub slice: JsMap<GuardId, crate::doortools::DoorToolsDoneItemSliceValue>,
    pub crossed: JsMap<GuardId, f64>,
    pub corner: JsMap<GuardId, crate::doortools::DoorToolsDoneItemCornerValue>,
    pub search: Option<Option<DoortoolsRoomSearchId>>,
    pub first: Option<GuardId>,
    pub threw: Option<GuardId>,
    pub aim: Option<crate::doortools::DoorToolsDoneItemAim>,
    pub pass: Option<crate::doortools::DoorToolsDoneItemPass>,
    pub beats: Vec<crate::doortools::DoorToolsDoneItemBeatsItem>,
    pub on_word: bool,
    pub word: Option<WordId>,
    pub hold: f64,
    pub ready_t: f64,
    pub gate: Option<DoorGateId>,
    pub thr: Option<crate::doortools::Threshold>,
    pub thr_hold: Option<bool>,
    pub thr_pre: Option<crate::doortools::DoorToolsDoneItemThrPre>,
    pub worded: Option<bool>,
    pub last_stack: Option<bool>,
    pub last_hold: Option<bool>,
    pub last_tossed: Option<f64>,
    pub other: Option<JsMap<DoorId, crate::doortools::DoorToolsDoneItemOtherValue>>,
    pub threat: Option<crate::doortools::DoorToolsDoneItemThreat>,
    pub ended: String,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum DoorToolKind {
    #[default] Charge,
    Outside,
    BangClear,
    OpenClear,
    Threshold,
}
impl DoorToolKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            DoorToolKind::Charge => "charge",
            DoorToolKind::Outside => "outside",
            DoorToolKind::BangClear => "bangClear",
            DoorToolKind::OpenClear => "openClear",
            DoorToolKind::Threshold => "threshold",
        }
    }
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct DoorToolsDoneItemLane {
    pub gd: GuardId,
    pub door: DoorId,
    pub cover: OrdersAssignmentId,
    pub back: crate::doortools::DoorToolsDoneItemLaneBack,
    pub at: Option<f64>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct DoorToolsDoneItemLaneBack {
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

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct DoorToolsDoneItemLaneBackMove {
    pub stance: crate::character::Stance,
    pub upper: crate::character::UpperMode,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct DoorToolsDoneItemLaneBackTries {
    pub door: DoorId,
    pub side: f64,
    pub post: crate::orders::StackPost,
    pub at: V3,
    pub on: crate::doortools::DoorToolsDoneItemLaneBackTriesOn,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct DoorToolsDoneItemLaneBackTriesOn {
    pub post: Option<V3>,
    pub going: String,
    pub order: String,
    pub face: Option<f64>,
    pub there: String,
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
    pub tries: Option<alloc::boxed::Box<crate::doortools::DoorToolsDoneItemLaneBackTries>>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, ToBytes, Default)]
pub enum Phase {
    #[default] Slice,
    Kick,
    Held,
    Stack,
    Wait,
    Place,
    Standoff,
    Armed,
    Enter,
    Fuse,
    Ready,
    Toss,
    Thr,
    Pocket,
}
impl Phase {
    pub const fn as_str(self) -> &'static str {
        match self {
            Phase::Slice => "slice",
            Phase::Kick => "kick",
            Phase::Held => "held",
            Phase::Stack => "stack",
            Phase::Wait => "wait",
            Phase::Place => "place",
            Phase::Standoff => "standoff",
            Phase::Armed => "armed",
            Phase::Enter => "enter",
            Phase::Fuse => "fuse",
            Phase::Ready => "ready",
            Phase::Toss => "toss",
            Phase::Thr => "thr",
            Phase::Pocket => "pocket",
        }
    }
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct DoorToolsDoneItemKick {
    pub pos: V3,
    pub yaw: f64,
    pub t: f64,
    pub pick: Option<bool>,
    pub hand: Option<bool>,
    pub gate: Option<bool>,
    pub on_it: Option<bool>,
    pub at: Option<f64>,
    pub by: Option<GuardId>,
    pub done_t: Option<f64>,
    pub off_t: Option<f64>,
    pub gun: Option<crate::doortools::DoorToolsDoneItemKickGun>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct DoorToolsDoneItemKickGun {
    pub by: GuardId,
    pub at: V3,
    pub n: f64,
    pub next_t: f64,
    pub silent: bool,
    pub own: Option<bool>,
    pub hammer: Option<bool>,
    pub shut: Option<bool>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct DoorToolsDoneItemQueueItem {
    pub gd: GuardId,
    pub at: f64,
    pub pod: crate::entrypoints::Pod,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct DoorToolsDoneItemSliceValue {
    pub pts: Vec<V3>,
    pub looks: Vec<Vec<V3>>,
    pub i: f64,
    pub j: f64,
    pub t: f64,
    pub ahead: Option<GuardId>,
    pub pie: Option<crate::doortools::DoorToolsDoneItemSliceValuePie>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct DoorToolsDoneItemSliceValuePie {
    pub eyes: Vec<V3>,
    pub lean: f64,
    pub end: f64,
    pub end_on: bool,
    pub knee: Vec<bool>,
    pub fin: bool,
    pub begun: bool,
    pub at: f64,
    pub inw: V3,
    pub gun: f64,
    pub gun_at: f64,
    pub on: bool,
    pub to: Option<V3>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct DoorToolsDoneItemCornerValue {
    pub pts: Vec<V3>,
    pub i: f64,
    pub on: f64,
    pub fight: Option<f64>,
    pub last: Option<f64>,
    pub turned: Option<bool>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct DoorToolsDoneItemAim {
    pub at: V3,
    pub from: V3,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct DoorToolsDoneItemPass {
    pub to: V3,
    pub v0: V3,
    pub t: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct DoorToolsDoneItemBeatsItem {
    pub t: f64,
    pub what: String,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Threshold {
    pub jamb: Vec<GuardId>,
    pub pods: Vec<crate::entrypoints::Pod>,
    pub quiet_t: f64,
    pub ev_t: f64,
    pub contact: JsSet<GuardId>,
    pub up: Vec<GuardId>,
    pub down: f64,
    pub sliced: bool,
    pub thrower: Option<GuardId>,
    pub tossed: f64,
    pub next_t: f64,
    pub aim: Option<V3>,
    pub from: Option<V3>,
    pub banged: Option<Vec<u8>>,
    pub handed: bool,
    pub los_t: f64,
    pub lit_c: Option<Vec<i8>>,
    pub tube_c: Option<Vec<i8>>,
    pub hold: bool,
    pub told: bool,
    pub force: bool,
    pub hand_w: Option<f64>,
    pub hand_t: Option<f64>,
    pub hand_done: Option<bool>,
    pub fled: Option<JsSet<GuardId>>,
    pub frag_t: Option<f64>,
    pub frag_said: Option<bool>,
    pub step_owed: Option<bool>,
    pub owed_t: Option<f64>,
    pub cut_owed: Option<bool>,
    pub first: Option<crate::doortools::ThresholdFirst>,
    pub out: Option<crate::doortools::ThresholdOut>,
    pub wide: Option<bool>,
    pub early: Option<bool>,
    pub asked: Option<bool>,
    pub word_step: Option<bool>,
    pub shut_t: Option<f64>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes)]
pub enum ThresholdFirst {
    Burst,
    Can,
    Done,
}
impl ThresholdFirst {
    pub const fn as_str(self) -> &'static str {
        match self {
            ThresholdFirst::Burst => "burst",
            ThresholdFirst::Can => "can",
            ThresholdFirst::Done => "done",
        }
    }
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct DoorToolsDoneItemThrPre {
    pub far: GuardId,
    pub t: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct DoorToolsDoneItemOtherValue {
    pub gd: GuardId,
    pub w: V3,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct DoorToolsDoneItemThreat {
    pub known: Vec<DoortoolsKnownThreatId>,
    pub ev_t: f64,
    pub room: Option<RoomfightRoomId>,
    pub room_t: Option<f64>,
    pub deal: Option<JsMap<GuardId, crate::doortools::ThreatWatch>>,
}

#[derive(Clone, Debug, Default)]
pub struct DoorToolsDoneItemThreatRoom {
    pub cells: JsSet<crate::Num>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct ThreatWatch {
    pub th: DoortoolsKnownThreatId,
    pub lead: Option<GuardId>,
    pub on: f64,
    pub fight: f64,
    pub last: f64,
    pub done_t: f64,
    pub faced: Option<bool>,
    pub at: Option<V3>,
    pub still: Option<f64>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct DoorToolsLogItem {
    pub t: f64,
    pub what: String,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct DoorToolsStats {
    pub orders: f64,
    pub opened: f64,
    pub kicks: f64,
    pub shot: f64,
    pub picks: f64,
    pub no_boot: f64,
    pub hand_kept: f64,
    pub tested: f64,
    pub halted: f64,
    pub bangs: f64,
    pub own_flashed: f64,
    pub frags: f64,
    pub blown: f64,
    pub knocked: f64,
    pub stunned: f64,
    pub backblast: f64,
    pub entries: f64,
    pub words: f64,
    pub stepped_back: f64,
    pub pods_moved: f64,
    pub sectors: f64,
    pub sent_on: f64,
    pub sent_alone: f64,
    pub door_alone: f64,
    pub sent_pair: f64,
    pub not_sent: f64,
    pub flowed: f64,
    pub closed_up: f64,
    pub flow_late: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct DoorToolsLed {
    pub entries: f64,
    pub by_seen: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct DoorToolsSamOf {
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct DoorToolsHeldValue {
    pub a: OrdersAssignmentId,
    pub face: f64,
    pub d: f64,
    pub hit_t: f64,
    pub arr_t: f64,
    pub out: Option<bool>,
    pub door: Option<DoorId>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct Word {
    pub t0: f64,
    pub go1: f64,
    pub fled_t: Option<f64>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct DoorOp {
    pub kind: crate::doortools::DoorToolKind,
    pub door: DoorId,
    pub side: f64,
    pub coded: Option<bool>,
    pub men: Vec<GuardId>,
    pub lead: GuardId,
    pub fled: Option<JsSet<GuardId>>,
    pub fled_w: Option<f64>,
    pub fled_t: Option<f64>,
    pub fled_said: Option<bool>,
    pub fled_go: Option<bool>,
    pub rear: JsSet<GuardId>,
    pub lane: Option<crate::doortools::DoorToolsDoneItemLane>,
    pub from: JsMap<GuardId, V3>,
    pub phase: crate::doortools::Phase,
    pub t0: f64,
    pub bang_held: bool,
    pub frag: Option<bool>,
    pub frag_way_t: Option<f64>,
    pub leaf_a: Option<f64>,
    pub leaf_t: Option<f64>,
    pub mine: JsMap<GuardId, OrdersAssignmentId>,
    pub post: JsMap<GuardId, crate::orders::StackPost>,
    pub kick: Option<crate::doortools::DoorToolsDoneItemKick>,
    pub no_gun: Option<bool>,
    pub big: Option<bool>,
    pub open_t: f64,
    pub can: Option<ItemId>,
    pub toss_t: f64,
    pub land_t: f64,
    pub bang_t: f64,
    pub bang_at: Option<V3>,
    pub queue: Vec<crate::doortools::DoorToolsDoneItemQueueItem>,
    pub set_t: f64,
    pub stow_off_t: Option<f64>,
    pub pod: JsMap<GuardId, crate::entrypoints::Pod>,
    pub on_pod: JsMap<GuardId, f64>,
    pub slice: JsMap<GuardId, crate::doortools::DoorToolsDoneItemSliceValue>,
    pub crossed: JsMap<GuardId, f64>,
    pub corner: JsMap<GuardId, crate::doortools::DoorToolsDoneItemCornerValue>,
    pub search: Option<Option<DoortoolsRoomSearchId>>,
    pub first: Option<GuardId>,
    pub threw: Option<GuardId>,
    pub aim: Option<crate::doortools::DoorToolsDoneItemAim>,
    pub pass: Option<crate::doortools::DoorToolsDoneItemPass>,
    pub beats: Vec<crate::doortools::DoorToolsDoneItemBeatsItem>,
    pub on_word: bool,
    pub word: Option<WordId>,
    pub hold: f64,
    pub ready_t: f64,
    pub gate: Option<DoorGateId>,
    pub thr: Option<crate::doortools::Threshold>,
    pub thr_hold: Option<bool>,
    pub thr_pre: Option<crate::doortools::DoorToolsDoneItemThrPre>,
    pub worded: Option<bool>,
    pub last_stack: Option<bool>,
    pub last_hold: Option<bool>,
    pub last_tossed: Option<f64>,
    pub other: Option<JsMap<DoorId, crate::doortools::DoorToolsDoneItemOtherValue>>,
    pub threat: Option<crate::doortools::DoorToolsDoneItemThreat>,
    /// INVENTED by a review: see the proposals
    pub ended: Option<String>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct KnownThreat {
    pub at: V3,
    pub t: f64,
    pub by: Option<GuardId>,
    pub how: crate::doortools::KnownThreatHow,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum KnownThreatHow {
    #[default] Shot,
    Seen,
}
impl KnownThreatHow {
    pub const fn as_str(self) -> &'static str {
        match self {
            KnownThreatHow::Shot => "shot",
            KnownThreatHow::Seen => "seen",
        }
    }
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct RoomSearch {
    pub room: RoomfightRoomId,
    pub pts: Vec<V3>,
    pub looked: Vec<u8>,
    pub los: JsMap<GuardId, crate::doortools::RoomSearchLosValue>,
    pub yaw: JsMap<GuardId, f64>,
    pub at: JsMap<GuardId, f64>,
    pub moves: f64,
    pub left_at_move: f64,
    pub left: f64,
    pub flows: Option<f64>,
    pub closes: Option<f64>,
    pub flow_said: Option<bool>,
    pub late: Option<bool>,
    pub cell_at: Option<JsMap<crate::Num, f64>>,
    pub fight_t: Option<f64>,
    pub fight_at: Option<f64>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct RoomSearchLosValue {
    pub at: V3,
    pub ok: Vec<i8>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct DoorGate {
    pub ops: Vec<DoorOpId>,
    pub go: bool,
    pub open_t: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct SetCharge {
    pub door: DoorId,
    pub side: f64,
    pub t: f64,
    pub by: String,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct PIEGUN {
    pub after: f64,
    pub step: f64,
    pub most: f64,
    pub every: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct DoortoolsModSKYValue {
    pub n: f64,
    pub open: bool,
}

#[derive(Clone, Debug, Default)]
pub struct UnderSkyRoom {
    pub cells: JsSet<crate::Num>,
}

#[derive(Clone, Debug, Default)]
pub struct OpenFloorRoom {
    pub cells: JsSet<crate::Num>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum DoorToolsTakeWhat {
    Frag,
    Bang,
    Charge,
}
impl DoorToolsTakeWhat {
    pub const fn as_str(self) -> &'static str {
        match self {
            DoorToolsTakeWhat::Frag => "frag",
            DoorToolsTakeWhat::Bang => "bang",
            DoorToolsTakeWhat::Charge => "charge",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum DoorToolsRefundWhat {
    Frag,
    Bang,
    Charge,
}
impl DoorToolsRefundWhat {
    pub const fn as_str(self) -> &'static str {
        match self {
            DoorToolsRefundWhat::Frag => "frag",
            DoorToolsRefundWhat::Bang => "bang",
            DoorToolsRefundWhat::Charge => "charge",
        }
    }
}

#[derive(Clone, Debug)]
pub enum DoorToolsStackable {
    Text(String),
    Bool(bool),
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum DoorToolsBigWord {
    Shotgun,
    Hammer,
}
impl DoorToolsBigWord {
    pub const fn as_str(self) -> &'static str {
        match self {
            DoorToolsBigWord::Shotgun => "shotgun",
            DoorToolsBigWord::Hammer => "hammer",
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct DoorToolsBootSpot {
    pub pos: V3,
    pub yaw: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct DoorToolsStackLeftPostValue {
    pub lead: bool,
    pub pos: V3,
    pub face: f64,
    pub stack_side: f64,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum DoorToolsEntryVerb {
    Pick,
    Kick,
    Open,
}
impl DoorToolsEntryVerb {
    pub const fn as_str(self) -> &'static str {
        match self {
            DoorToolsEntryVerb::Pick => "pick",
            DoorToolsEntryVerb::Kick => "kick",
            DoorToolsEntryVerb::Open => "open",
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct DoorToolsAssignA {
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
    pub shy: Option<Option<DoorId>>,
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
pub enum DoorToolsIssue {
    Text(String),
    Arm1(()),
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum DoorToolsOrderKind {
    Outside,
    BangClear,
    OpenClear,
    Threshold,
}
impl DoorToolsOrderKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            DoorToolsOrderKind::Outside => "outside",
            DoorToolsOrderKind::BangClear => "bangClear",
            DoorToolsOrderKind::OpenClear => "openClear",
            DoorToolsOrderKind::Threshold => "threshold",
        }
    }
}

#[derive(Clone, Debug)]
pub enum DoorToolsOrder {
    Text(String),
    DoorOp(DoorOpId),
}

#[derive(Clone, Debug)]
pub enum DoorToolsThreshold {
    Text(String),
    Arm1(()),
}

#[derive(Clone, Debug)]
pub enum DoorToolsThresholdHold {
    Text(String),
    Arm1(()),
}

#[derive(Clone, Debug)]
pub enum DoorToolsCanThreshold {
    Text(String),
    Bool(bool),
}

#[derive(Clone, Debug)]
pub enum DoorToolsThresholdOn {
    Text(String),
    DoorOp(DoorOpId),
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum DoorToolsThresholdThenWhat {
    Bang,
    Step,
    Leave,
}
impl DoorToolsThresholdThenWhat {
    pub const fn as_str(self) -> &'static str {
        match self {
            DoorToolsThresholdThenWhat::Bang => "bang",
            DoorToolsThresholdThenWhat::Step => "step",
            DoorToolsThresholdThenWhat::Leave => "leave",
        }
    }
}

#[derive(Clone, Debug)]
pub enum DoorToolsThresholdThenT {
    Floor(crate::orders::OrderTargetFloor),
    Door(crate::orders::OrderTargetDoor),
    None(crate::orders::OrderTargetNone),
    Foe(crate::orders::OrderTargetFoe),
}

#[derive(Clone, Debug)]
pub enum DoorToolsThresholdThen {
    Text(String),
    Arm1(()),
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum DoorToolsThenOnWordWhat {
    Bang,
    Step,
}
impl DoorToolsThenOnWordWhat {
    pub const fn as_str(self) -> &'static str {
        match self {
            DoorToolsThenOnWordWhat::Bang => "bang",
            DoorToolsThenOnWordWhat::Step => "step",
        }
    }
}

#[derive(Clone, Debug)]
pub enum DoorToolsThenOnWord {
    Text(String),
    Arm1(()),
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum DoorToolsCanThresholdThenWhat {
    Bang,
    Step,
    Leave,
}
impl DoorToolsCanThresholdThenWhat {
    pub const fn as_str(self) -> &'static str {
        match self {
            DoorToolsCanThresholdThenWhat::Bang => "bang",
            DoorToolsCanThresholdThenWhat::Step => "step",
            DoorToolsCanThresholdThenWhat::Leave => "leave",
        }
    }
}

#[derive(Clone, Debug)]
pub enum DoorToolsCanThresholdThenT {
    Floor(crate::orders::OrderTargetFloor),
    Door(crate::orders::OrderTargetDoor),
    None(crate::orders::OrderTargetNone),
    Foe(crate::orders::OrderTargetFoe),
}

#[derive(Clone, Debug)]
pub enum DoorToolsCanThresholdThen {
    Text(String),
    Bool(bool),
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum DoorToolsThresholdTellWhat {
    Bang,
    Step,
    Leave,
}
impl DoorToolsThresholdTellWhat {
    pub const fn as_str(self) -> &'static str {
        match self {
            DoorToolsThresholdTellWhat::Bang => "bang",
            DoorToolsThresholdTellWhat::Step => "step",
            DoorToolsThresholdTellWhat::Leave => "leave",
        }
    }
}

#[derive(Clone, Debug)]
pub enum DoorToolsStart {
    Text(String),
    DoorOp(DoorOpId),
}

#[derive(Clone, Debug)]
pub enum DoorToolsStartCharge {
    Text(String),
    Arm1(()),
}

#[derive(Clone, Debug)]
pub enum DoorToolsStandoffFits {
    Text(String),
    Bool(bool),
}

#[derive(Clone, Copy, Debug, Default)]
pub struct DoorToolsSamJob {
    pub p: V3,
    pub n: V3,
    pub at: V3,
}

#[derive(Clone, Debug, Default)]
pub struct DoorToolsDealStack {
    pub posts: Vec<crate::orders::StackPost>,
    pub back: JsMap<GuardId, crate::orders::StackPost>,
}

#[derive(Clone, Debug)]
pub enum DoorToolsWaiting4Item {
    DoorOp(DoorOpId),
    Arm1(crate::doortools::DoorToolsWaiting4ItemArm1),
}

#[derive(Clone, Debug, Default)]
pub struct DoorToolsWaiting4ItemArm1 {
    pub stack: DoorId,
    pub men: Vec<GuardId>,
}

#[derive(Clone, Debug, Default)]
pub struct DoorToolsStacksOfItem {
    pub stack: DoorId,
    pub men: Vec<GuardId>,
}

#[derive(Clone, Debug, Default)]
pub struct DoorToolsStackWaitsItem {
    pub stack: DoorId,
    pub men: Vec<GuardId>,
}

#[derive(Clone, Debug)]
pub enum DoorToolsWaitingAnyItem {
    DoorOp(DoorOpId),
    Arm1(crate::doortools::DoorToolsWaitingAnyItemArm1),
}

#[derive(Clone, Debug, Default)]
pub struct DoorToolsWaitingAnyItemArm1 {
    pub stack: DoorId,
    pub men: Vec<GuardId>,
}

#[derive(Clone, Debug, Default)]
pub struct DoorToolsWaitsOn {
    pub kind: crate::doortools::DoorToolsWaitsOnKind,
    pub men: Vec<GuardId>,
    pub set: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum DoorToolsWaitsOnKind {
    #[default] Stack,
    Word,
}
impl DoorToolsWaitsOnKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            DoorToolsWaitsOnKind::Stack => "stack",
            DoorToolsWaitsOnKind::Word => "word",
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct DoorToolsGoMenOnlyItem {
    pub stack: DoorId,
    pub men: Vec<GuardId>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct DoorToolsBangSpot {
    pub at: V3,
    pub n: f64,
    pub of: f64,
}

#[derive(Clone, Copy, Debug)]
pub enum DoorToolsAway {
    Guard(GuardId),
    Alone,
    No,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct DoorToolsOffSwingP {
    pub pos: V3,
    pub watch: Option<V3>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct DoorToolsBlow {
    pub knocked: f64,
    pub stunned: f64,
    pub backblast: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct DoorToolsBlowRaw {
    pub knocked: f64,
    pub stunned: f64,
    pub backblast: f64,
}

#[derive(Clone, Debug, Default)]
pub struct DoorToolsPieNextS {
    pub pts: Vec<V3>,
    pub i: f64,
    pub pie: Option<crate::doortools::DoorToolsPieNextSPie>,
}

#[derive(Clone, Debug, Default)]
pub struct DoorToolsPieNextSPie {
    pub eyes: Vec<V3>,
    pub knee: Vec<bool>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct DoorToolsThrSpot {
    pub q: V3,
    pub from: V3,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct DoorToolsThrFar {
    pub far: GuardId,
    pub fs: f64,
    pub here: V3,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct DoorToolsPocketOf {
    pub at: Option<V3>,
    pub n: f64,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes)]
pub struct DoortoolsMod {
    /// (src/game/doortools.ts:761 SKY)
    pub sky: JsMap<RoomfightRoomId, crate::doortools::DoortoolsModSKYValue>,
}
impl DoortoolsMod {
    pub fn blank() -> Self {
        DoortoolsMod {
            sky: JsMap::new(),
        }
    }
}

/// (src/game/doortools.ts:114 WORDCUTS_BREAK)
pub const WORDCUTS_BREAK: bool = false;

/// (src/game/doortools.ts:601 LEAF_EDGE)
pub const LEAF_EDGE: f64 = 0.1;

/// (src/game/doortools.ts:601 LEAF_FROM)
pub const LEAF_FROM: f64 = 1.45;

/// (src/game/doortools.ts:601 LEAF_STEP)
pub const LEAF_STEP: f64 = 0.1;

/// (src/game/doortools.ts:601 LEAF_COST)
pub const LEAF_COST: f64 = 4.0;

/// (src/game/doortools.ts:601 LEAF_POSTS: a table nothing writes)
pub fn leaf_posts() -> Vec<Vec<f64>> { alloc::vec![alloc::vec![0.2, 2.4], alloc::vec![0.5, 2.4]] }

/// (src/game/doortools.ts:603 SAM_SHIFT)
pub const SAM_SHIFT: f64 = 0.35;

/// (src/game/doortools.ts:609 BIG_KEY)
pub const BIG_KEY: &str = "KeyH";

/// (src/game/doortools.ts:612 THR_PAGE)
pub const THR_PAGE: &str = "thr";

/// (src/game/doortools.ts:614 PIE_WAIT)
pub const PIE_WAIT: f64 = 2.5;

/// (src/game/doortools.ts:614 KNEE_EYE)
pub const KNEE_EYE: f64 = 0.95;

/// (src/game/doortools.ts:614 PIE_GUN: a table nothing writes)
pub fn pie_gun() -> crate::doortools::PIEGUN { crate::doortools::PIEGUN { after: 0.2, step: 0.12, most: 3.0, every: 0.4 } }

/// (src/game/doortools.ts:622 PRE: a table nothing writes)
pub fn pre() -> Vec<String> { alloc::vec![String::from("stack"), String::from("ready"), String::from("place"), String::from("standoff"), String::from("armed")] }

/// (src/game/doortools.ts:757 OPEN_RAD)
pub const OPEN_RAD: f64 = 0.6;

/// (src/game/doortools.ts:759 N4: a table nothing writes)
pub fn n4() -> Vec<[f64; 2]> { alloc::vec![[1.0, 0.0], [-1.0, 0.0], [0.0, 1.0], [0.0, -1.0]] }

/// (src/game/doortools.ts:3855 WORD_LEAD)
pub const WORD_LEAD: f64 = 0.15;
