//! The types of `doortools`.
#![allow(unused_imports)]
use crate::{Blank, Words, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// the word's own row as the squad's size has it: none, a squad of one or two (it's on no list and waits stacked), a bigger one
#[derive(Clone, Copy, PartialEq, Eq, Debug, ToBytes)]
pub enum GoRow { None, Small, Big }

/// the step onto the diagonals, kept until the first can's burst has run dry
#[derive(Clone, Debug, ToBytes)]
pub enum ThresholdOut {
    Step { lead: GuardId, f: Option<crate::doortools::DoorToolsThrFar>, sl: crate::doortools::DoorToolsDoneItemSliceValue, sf: Option<crate::doortools::DoorToolsDoneItemSliceValue>, light: bool, name: String },
}
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct DoorTools {
    #[is(2.0)] pub charges: f64,
    #[is(3.0)] pub bangs: f64,
    #[is(2.0)] pub frags: f64,
    pub frag_next: bool,
    pub big_next: bool,
    pub big_row: Option<bool>,
    pub ops: Vec<DoorOpId>,
    pub done: Vec<DoorOpId>,
    pub set: Vec<ChargeId>,
    pub linked: Vec<DoorId>,
    pub lights: Option<bool>,
    #[is(crate::doortools::DoorToolsStats { orders: 0.0, kicks: 0.0, picks: 0.0, tested: 0.0, halted: 0.0, bangs: 0.0, frags: 0.0, blown: 0.0, backblast: 0.0 })] pub stats: crate::doortools::DoorToolsStats,
    pub roster: Vec<GuardId>,
    pub seen: Option<crate::floorseen::FloorSeen>,
    pub go_row: Option<GoRow>,
    pub words: Vec<WordId>,
    #[is(-1000000000.0)] pub word_key_t: f64,
    pub thr_row: bool,
    #[is(-1.0)] pub thr_sig: f64,
    pub sam_of: Option<PlayerId>,
    pub sam_spent: f64,
    pub tried: JsMap<DoorId, f64>,
    pub gate_asked: JsSet<DoorOpId>,
    pub held: JsMap<GuardId, crate::doortools::DoorToolsHeldValue>,
    pub watches: Vec<DoorOpId>,
    #[is(String::from(""))] pub away_why: String,
    pub away_far: bool,
    pub big_said: JsSet<DoorOpId>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default, Words)]
pub enum DoorToolKind {
    #[default] Charge,
    BangClear,
    OpenClear,
    Threshold,
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
    pub going: crate::orders::Doing,
    pub face: Option<f64>,
    pub there: crate::orders::Doing,
    pub watch: Option<V3>,
    pub stance: Option<crate::character::Stance>,
    pub upper: Option<crate::character::UpperMode>,
    pub stack_side: Option<f64>,
    pub move_: Option<crate::doortools::DoorToolsDoneItemLaneBackMove>,
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
    pub going: crate::orders::Doing,
    pub order: String,
    pub face: Option<f64>,
    pub there: crate::orders::Doing,
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
    #[default] Kick,
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
    pub pod: crate::entrypoints::Pod,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct DoorToolsDoneItemSliceValue {
    pub pts: Vec<V3>,
    pub looks: Vec<Vec<V3>>,
    pub i: f64,
    pub j: f64,
    pub t: f64,
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
    pub is: crate::doortools::Beat,
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
    pub shut_t: Option<f64>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes)]
pub enum ThresholdFirst {
    Burst,
    Can,
    Done,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct DoorToolsDoneItemThrPre {
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

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct DoorToolsStats {
    pub orders: f64,
    pub kicks: f64,
    pub picks: f64,
    pub tested: f64,
    pub halted: f64,
    pub bangs: f64,
    pub frags: f64,
    pub blown: f64,
    pub backblast: f64,
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
    /// why it ended (`door_tools_end`); None while it runs
    pub ended: Option<crate::doortools::Ended>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct KnownThreat {
    pub at: V3,
    pub t: f64,
    pub by: Option<GuardId>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default, Words)]
pub enum KnownThreatHow {
    #[default] Shot,
    Seen,
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

#[derive(Clone, Debug)]
pub enum DoorToolsStackable {
    Text(String),
    Bool(bool),
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

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Words)]
pub enum DoorToolsEntryVerb {
    Pick,
    Kick,
    Open,
}

#[derive(Clone, Debug, Default)]
pub struct DoorToolsAssignA {
    pub post: Option<V3>,
    pub going: crate::orders::Doing,
    pub face: Option<f64>,
    pub there: crate::orders::Doing,
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

#[derive(Clone, Debug, Default)]
pub struct DoorToolsPieNextS {
    pub pts: Vec<V3>,
    pub i: f64,
    pub pie: Option<crate::doortools::DoorToolsPieNextSPie>,
}

#[derive(Clone, Debug, Default)]
pub struct DoorToolsPieNextSPie {
    pub eyes: Vec<V3>,
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
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct DoortoolsMod {
    pub sky: JsMap<RoomfightRoomId, crate::doortools::DoortoolsModSKYValue>,
}

pub const LEAF_EDGE: f64 = 0.1;

pub const LEAF_FROM: f64 = 1.45;

pub const LEAF_STEP: f64 = 0.1;

pub const LEAF_COST: f64 = 4.0;


pub const SAM_SHIFT: f64 = 0.35;

pub const THR_PAGE: &str = "thr";

pub const PIE_WAIT: f64 = 2.5;

pub fn pie_gun() -> crate::doortools::PIEGUN { crate::doortools::PIEGUN { after: 0.2, step: 0.12, most: 3.0, every: 0.4 } }

pub const OPEN_RAD: f64 = 0.6;

pub fn n4() -> Vec<[f64; 2]> { alloc::vec![[1.0, 0.0], [-1.0, 0.0], [0.0, 1.0], [0.0, -1.0]] }

pub const WORD_LEAD: f64 = 0.15;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum ThresholdThen {
    Bang,
    Step,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Words)]
pub enum DoorToolsCan {
    Frag,
    Bang,
    Charge,
}

/// how an order at a door was over
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Words)]
pub enum Ended {
    #[word("steel and locked")] SteelAndLocked,
    #[word("locked · silent: no boot")] SilentNoBoot,
    #[word("nobody left on it")] NobodyLeft,
    #[word("never made the lock")] NoLock,
    #[word("opens toward us · no boot")] OpensTowardUs,
    #[word("found shut")] FoundShut,
    #[word("re-ordered")] ReOrdered,
    #[word("no floor inside")] NoFloorInside,
    #[word("never made the stack")] NoStack,
    #[word("never made the kick spot")] NoKickSpot,
    #[word("the door held the boot")] HeldTheBoot,
    #[word("no bang to throw")] NoBang,
    #[word("the frag is gone")] FragGone,
    #[word("in")] In,
    #[word("the charge is gone")] ChargeGone,
    #[word("the door opened")] DoorOpened,
    #[word("never made the door")] NoDoor,
    #[word("no charges")] NoCharges,
}

/// the beats of an order at a door that a rule asks for; the rest are words for its timeline
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum Beat { #[default] Other, Stacked, Kick, Shove, OpenAlready, Toss, Bang, Go, Word }
