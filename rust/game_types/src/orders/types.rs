//! The types of `orders`.
#![allow(unused_imports)]
use crate::{Blank, Words, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// what marksKeep took, to put back once: the makers' counters, the room rule's book, the leaf's answer. Not kept past the one trial.
#[derive(Clone, Debug)]
pub enum MarksKeep {
    Kept { leaf: bool, stack_own: crate::doorway::DoorwayModSTACKOWN, doorway: crate::doorway::DoorwayStats, stackroom: crate::stackroom::StackroomStats, glasspen: crate::glasspen::GlasspenStats, book: crate::stackroom::StackRoomBookKeep },
}
/// what one deal of a stack costs in men squeezing past one another: `file` = each spot's file, `tight` = by a spot's file, is it tight, `l` = man by spot, his walk (0 off a tight file), `cost` = ORDERS.fileQueueCost. Needs no g to price a deal.
#[derive(Clone, Debug)]
pub enum FileCross {
    Tight { file: Vec<usize>, tight: Vec<bool>, l: Vec<Vec<f64>>, spots: Vec<V3>, cost: f64 },
}
/// a teammate's way, nav.findPath's penalty: a cell of `dear` costs `cost`, any other 1, times what `pen` asks. `dear` is by nav.idx; the nav is read from g when priced.
#[derive(Clone, Debug)]
pub enum PropPenalty {
    Dear { dear: JsSet<crate::Num>, cost: f64, pen: Option<crate::lanes::LanePenalty> },
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct StackPost {
    pub pos: V3,
    pub face: f64,
    pub stack_side: f64,
    pub lead: bool,
    pub group: Option<f64>,
}

#[derive(Clone, Debug, ToBytes, Blank)]
pub struct SquadOrders {
    #[is(crate::knobs::OrdersK { who_key: String::from("KeyT"), quick_key: String::from("KeyB"), refresh_secs: 1.0, door_pick_m: 1.0, self_m: 1.0, man_m: 0.8, door_ray_pad: 0.08, door_ray_reach_m: 3.0, floor_snap_m: 1.0, jamb_out: 0.55, jamb_past: 0.45, file_gap: 0.9, seam_m: 0.03, work_seen_m: 4.0, follow_back: alloc::vec![1.3, 2.4], crumb: 0.4, trail_m: 10.0, run: 2.2, walk: 1.5, gain: 1.5, near_m: 0.6, band_m: 0.6, word_far_m: 2.0, word_hold_s: 2.5, file_apart: 0.9, file_tight: 0.75, off_door: 0.8, aside_m: 2.6, aside_lane: 0.8, aside_off: 0.95, file_queue_gap: 1.3, file_queue_cost: 500.0, prop_round_pad: 0.25, prop_round_cost: 6.0, prop_round_more: 2.0, prop_round_door: 2.5, aside_hold: 1.5, brisk: 2.0, brisk_in: 1.8, onto: 1.3, entry: 2.2, move_spread: 0.9, cold_secs: 2.5, lane_in: 0.45, lane_out: 0.7, hold_stride: 0.15, stuck_secs: 4.0, hold_group_m: 4.0, hold_door_m: 2.0, cleared_dear: 20.0, round_cap: 0.0, same_near_m: 5.0, same_apart_m: 6.0, same_long: 1.5, trail_reach_m: 3.0, trail_long: 1.5 })] pub knobs: crate::knobs::OrdersK,
    pub defs: Vec<OrdersOrderDefId>,
    /// the row of the table this seat's menu has for an order's id, on the menu or off it (`squad_orders_row`; Rust's own: not on the wire)
    pub made: JsMap<String, OrdersOrderDefId>,
    /// what the squad's panel says of this seat, as its last tick left it (orders/shown.rs; Rust's own: not on the wire, so blank after a load until the next tick)
    pub shown: crate::orders::Shown,
    pub posture: Option<crate::orders::SquadOrdersPosture>,
    pub people: Option<crate::orders::SquadOrdersPeople>,
    #[is(crate::orders::Who::All)] pub who: crate::orders::Who,
    pub menu: Option<crate::orders::Menu>,
    pub cmd_ring: Option<crate::cmdring::RingState>,
    pub cmd_press: Option<crate::orders::SquadOrdersCmdPress>,
    pub tap_was: Option<crate::orders::SquadOrdersTapWas>,
    pub log: Vec<crate::orders::SquadOrdersLogItem>,
    pub shows: Vec<crate::orders::SquadOrdersShowsItem>,
    pub asg: JsMap<GuardId, OrdersAssignmentId>,
    pub trail: Vec<V3>,
    #[is([0.0, 0.0, -1.0])] pub back: V3,
    #[is(crate::orders::SquadOrdersView { input: None })] pub view: crate::orders::SquadOrdersView,
    pub esc_used: bool,
    pub ours: JsSet<GuardId>,
    pub acks: f64,
    pub group: f64,
    pub roster: Vec<GuardId>,
    pub sam_p: V3,
    #[is(-1.0)] pub sam_t: f64,
    pub sam_v: f64,
    #[is([0.0, 0.0, 1.0])] pub sam_dir: V3,
    pub door_marks: Vec<crate::orders::SquadOrdersDoorMarksItem>,
    pub cast: Option<crate::orders::OtherCast>,
    pub aside_mem: JsMap<GuardId, crate::orders::SquadOrdersAsideMemValue>,
    pub back_fix: JsMap<OrdersAssignmentId, crate::orders::SquadOrdersBackFixValue>,
    pub rest_of: JsMap<OrdersAssignmentId, crate::orders::SquadOrdersRestOfValue>,
    pub held_of: JsMap<OrdersAssignmentId, crate::orders::SquadOrdersHeldOfValue>,
    pub short_of: JsMap<OrdersAssignmentId, crate::orders::SquadOrdersShortOfValue>,
    #[is(-1.0)] pub clear_t: f64,
    pub clear_by: Vec<f64>,
    pub apart_of: JsMap<OrdersAssignmentId, crate::orders::SquadOrdersApartOfValue>,
    pub file_hand: f64,
    pub file_gave: bool,
    pub sam_out: bool,
    pub leaf_out_gave: bool,
    pub glass_at: JsMap<String, f64>,
    pub glass_sig: f64,
    pub fur_in: bool,
    pub worked: JsMap<DoorId, Vec<Option<crate::orders::SquadOrdersWorkedValueItem>>>,
    pub reached: JsMap<String, crate::orders::SquadOrdersReachedValue>,
}


#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct SquadOrdersPosture {
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct SquadOrdersPeople {
}

#[derive(Clone, Copy, Debug, ToBytes)]
pub enum Who {
    Num(f64),
    All,
    Red,
    Blue,
}

#[derive(Clone, Debug, ToBytes)]
pub struct Menu {
    pub target: crate::orders::OrderTarget,
    pub men: Vec<GuardId>,
    pub rows: Vec<OrdersOrderDefId>,
    pub why: Vec<Option<String>>,
    pub sel: f64,
    pub read_t: f64,
    pub part: Option<Vec<GuardId>>,
    pub men_side: Option<f64>,
    pub side_key: String,
    /// each row's words, read beside its why (orders/words.rs): no part of the wire
    pub words: Vec<String>,
    /// the first wedge's word, the quick key's there (the same)
    pub quick: Option<String>,
}
impl Default for Menu {
    fn default() -> Self {
        Menu {
            target: crate::orders::OrderTarget::Floor(crate::orders::OrderTargetFloor::default()),
            men: Vec::new(),
            rows: Vec::new(),
            why: Vec::new(),
            sel: 0.0,
            read_t: 0.0,
            part: None,
            men_side: None,
            side_key: String::new(),
            words: Vec::new(),
            quick: None,
        }
    }
}

#[derive(Clone, Debug, ToBytes)]
pub enum OrderTarget {
    Floor(crate::orders::OrderTargetFloor),
    Door(crate::orders::OrderTargetDoor),
    None(crate::orders::OrderTargetNone),
    Foe(crate::orders::OrderTargetFoe),
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct OrderTargetFloor {
    pub at: V3,
    pub wall: Option<crate::wallcharge::WallSpot>,
    pub thing: Option<crate::workthings::WorkThing>,
    /// INVENTED by a review: see the proposals
    pub serial: u32,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct OrderTargetDoor {
    pub at: V3,
    pub door: DoorId,
    pub side: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct OrderTargetNone {
    pub at: V3,
    pub man: Option<GuardId>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct OrderTargetFoe {
    pub at: V3,
    pub foe: GuardId,
}

#[derive(Clone, Debug, ToBytes)]
pub struct SquadOrdersCmdPress {
    pub t0: f64,
    pub t: crate::orders::OrderTarget,
    pub add: Option<bool>,
}
impl Default for SquadOrdersCmdPress {
    fn default() -> Self {
        SquadOrdersCmdPress {
            t0: 0.0,
            t: crate::orders::OrderTarget::Floor(crate::orders::OrderTargetFloor::default()),
            add: None,
        }
    }
}

#[derive(Clone, Debug, ToBytes)]
pub struct SquadOrdersTapWas {
    pub at: f64,
    pub t: crate::orders::OrderTarget,
    pub x: f64,
    pub y: f64,
}
impl Default for SquadOrdersTapWas {
    fn default() -> Self {
        SquadOrdersTapWas {
            at: 0.0,
            t: crate::orders::OrderTarget::Floor(crate::orders::OrderTargetFloor::default()),
            x: 0.0,
            y: 0.0,
        }
    }
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct SquadOrdersLogItem {
    pub t: f64,
    pub order: String,
    pub who: Vec<String>,
    pub target: String,
    pub auto: Option<bool>,
    /// INVENTED: g.serial() at the push: rescue.ts:570 and lesson.ts:113 hold a row as a cursor by identity
    pub serial: u32,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct SquadOrdersShowsItem {
    pub t: f64,
    pub who: Vec<String>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct SquadOrdersView {
    pub input: Option<InputId>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct SquadOrdersDoorMarksItem {
    pub door: DoorId,
    pub side: f64,
    pub stack: Vec<V3>,
    pub standoff: Vec<V3>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct OtherCast {
    pub team: crate::teams::Team,
    pub ring: JsSet<String>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct SquadOrdersAsideMemValue {
    pub q: V3,
    pub t: f64,
    pub dir: V3,
    pub side: Option<f64>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct SquadOrdersBackFixValue {
    pub plain: f64,
    pub path: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct SquadOrdersRestOfValue {
    pub x: f64,
    pub z: f64,
    pub t: f64,
    pub step: Option<V3>,
    pub until: f64,
    pub from: V3,
    pub n: f64,
    pub hold: Option<V3>,
    pub hold_for: V3,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct SquadOrdersHeldOfValue {
    pub last: f64,
    pub total: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct SquadOrdersShortOfValue {
    pub x: f64,
    pub z: f64,
    pub n: f64,
    pub k: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct SquadOrdersApartOfValue {
    pub at: V3,
    pub q: Option<V3>,
    pub t: f64,
    pub m: f64,
    pub short: Option<bool>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct SquadOrdersWorkedValueItem {
    pub jambless: bool,
    pub marks: Vec<crate::orders::SquadOrdersWorkedValueItemMarksItem>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct SquadOrdersWorkedValueItemMarksItem {
    pub pos: V3,
    pub walk: f64,
    pub seen: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct SquadOrdersReachedValue {
    pub t: f64,
    pub why: Option<String>,
    pub lock: Option<crate::orders::SquadOrdersReachedValueLock>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct SquadOrdersReachedValueLock {
    pub door: DoorId,
    pub side: f64,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Words)]
pub enum TargetKind {
    None,
    Door,
    Floor,
    Foe,
}

#[derive(Clone, Debug, Default)]
pub struct GiveOpts {
    pub ack: Option<String>,
    pub men: Option<Vec<GuardId>>,
}

#[derive(Clone, Debug, ToBytes)]
pub struct OrderDef {
    pub id: String,
    /// what it does: its label, who it is for, what it asks and hands out (orders/rows/)
    /// (code: in no bytes)
    #[bytes(skip)]
    pub row: &'static crate::orders::Row,
    pub targets: Vec<crate::orders::TargetKind>,
    /// the name of the rows this one is among: they're listed where a row that opens the name is
    pub page: Option<String>,
    pub opens: Option<String>,
    pub listed: Option<Vec<crate::orders::TargetKind>>,
    /// the word's own row: F gives it with the ring up, and it has no wedge
    pub word: bool,
    pub ack: Option<Vec<String>>,
    pub ack_now: Option<Vec<String>>,
    pub free: Option<bool>,
    pub stands_in: Option<crate::orders::OrderDefStandsIn>,
}
impl Default for OrderDef {
    fn default() -> Self {
        OrderDef {
            id: String::new(),
            row: &crate::orders::Row::NONE,
            targets: Vec::new(),
            page: None,
            opens: None,
            listed: None,
            word: false,
            ack: None,
            ack_now: None,
            free: None,
            stands_in: None,
        }
    }
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct OrderDefStandsIn {
    pub for_: String,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Assignment {
    pub order: String,
    pub t0: f64,
    pub post: Option<V3>,
    pub follow: f64,
    pub face: Option<f64>,
    pub watch: Option<V3>,
    pub door: Option<DoorId>,
    pub strafe: Option<bool>,
    pub stance: crate::character::Stance,
    pub upper: crate::character::UpperMode,
    pub stack_side: f64,
    pub move_: Option<crate::doortools::DoorToolsDoneItemLaneBackMove>,
    pub going: String,
    pub there: String,
    pub set_line: Option<String>,
    pub group: f64,
    pub set_said: bool,
    pub goal: V3,
    pub arrived: bool,
    pub held_set: Option<bool>,
    pub held_fire: Option<bool>,
    pub arr_t: f64,
    pub moved_t: f64,
    pub light: Option<bool>,
    pub low: bool,
    pub yield_t: f64,
    pub face_now: Option<f64>,
    pub search: Option<bool>,
    pub search_t: Option<f64>,
    pub via: Vec<V3>,
    pub anchor: V3,
    pub mark: V3,
    pub mark_t: f64,
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
    /// INVENTED: true at assign and at the unguarded writes made here, not for doorpath.ts:273, :316, which alias s.goal. orders.ts:1519 passes held: !(a.new_leg && the A.goal arm was chosen) and clears it then
    pub new_leg: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Tone {
    Red,
    Blue,
    Gold,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct OrdersModFOLLOWWORDValue {
    pub on: bool,
    pub out_t: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct LOW {
    pub stance: crate::character::Stance,
    pub upper: crate::character::UpperMode,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct AIM {
    pub stance: crate::character::Stance,
    pub upper: crate::character::UpperMode,
}

#[derive(Clone, Copy, Debug, ToBytes)]
pub enum SquadOrdersMenSel {
    Num(f64),
    Guard(GuardId),
    All,
    Red,
    Blue,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SquadOrdersMenOpts {
    pub down: Option<bool>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SquadOrdersSceneHitFirst {
    pub t: f64,
    pub index: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SquadOrdersSceneHit {
    pub t: f64,
    pub index: f64,
}

#[derive(Clone, Debug, Default)]
pub struct SquadOrdersAssignA {
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
    pub tries: Option<crate::doortools::DoorToolsDoneItemLaneBackTries>,
}

#[derive(Clone, Debug, Default)]
pub struct SquadOrdersRingUpG {
    pub men: Vec<GuardId>,
}

#[derive(Clone, Debug, Default)]
pub struct SquadOrdersGoalPart {
    pub men: Vec<GuardId>,
}

#[derive(Clone, Debug, Default)]
pub struct SquadOrdersSeats {
    pub row: Vec<f64>,
    pub more: f64,
    pub over: Vec<f64>,
    pub same: Vec<f64>,
}

#[derive(Clone, Debug, Default)]
pub struct SquadOrdersWorkedMarks {
    pub jambless: bool,
    pub marks: Vec<crate::orders::SquadOrdersWorkedMarksMarksItem>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SquadOrdersWorkedMarksMarksItem {
    pub pos: V3,
    pub walk: f64,
    pub seen: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SquadOrdersHoldArcsItem {
    pub yaw: f64,
    pub door: Option<DoorId>,
    pub reach: f64,
}

#[derive(Clone, Debug, Default)]
pub struct SquadOrdersRoute {
    pub why: Option<String>,
    pub via: Vec<V3>,
    pub how: Option<crate::orders::SquadOrdersRouteHow>,
    pub lock: Option<crate::orders::SquadOrdersRouteLock>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum SquadOrdersRouteHow {
    Round,
    Through,
    Mouth,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SquadOrdersRouteLock {
    pub door: DoorId,
    pub side: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SquadOrdersWalkTry {
    pub door: DoorId,
    pub side: f64,
}

#[derive(Clone, Debug)]
pub enum SquadOrdersWalkStacks {
    Text(String),
    Map(JsMap<GuardId, crate::orders::SquadOrdersWalkStacksMapValue>),
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SquadOrdersWalkStacksMapValue {
    pub door: DoorId,
    pub side: f64,
    pub post: crate::orders::StackPost,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SquadOrdersWalkSendT {
    pub door: DoorId,
    pub side: f64,
    pub post: crate::orders::StackPost,
}

#[derive(Clone, Debug, Default)]
pub struct SquadOrdersAsked {
    pub why: Option<String>,
    pub lock: Option<crate::orders::SquadOrdersAskedLock>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SquadOrdersAskedLock {
    pub door: DoorId,
    pub side: f64,
}

#[derive(Clone, Debug, Default)]
pub struct SquadOrdersWhoTag {
    pub tag: String,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct FarOff {
    pub d: f64,
    pub at: V3,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct OrdersMod {
    pub follow_word: JsMap<OrdersAssignmentId, crate::orders::OrdersModFOLLOWWORDValue>,
}

pub fn nowhere() -> V3 { [10000000.0, 0.0, 10000000.0] }

pub const OWN_EPS: f64 = 0.15;

pub const IN_WAY_WORDS: &str = "you're in the way.";

pub const FUR_M: f64 = 0.12;

pub const BACK_TRIES: f64 = 2.0;

pub fn low() -> crate::orders::LOW { crate::orders::LOW { stance: crate::character::Stance::LowReady, upper: crate::character::UpperMode::None } }

pub fn aim() -> crate::orders::AIM { crate::orders::AIM { stance: crate::character::Stance::None, upper: crate::character::UpperMode::Aim } }

pub const OFF_M: f64 = 0.3;

pub const ROUND_ITER: f64 = 30000.0;

pub const MOUTH_M: f64 = 0.8;

pub const SAME_PLUS_M: f64 = 2.0;

pub const ROUND_PLUS_M: f64 = 3.0;

pub const ROUND_LESS: f64 = 0.5;

pub fn door_said() -> JsMap<String, String> { { let mut t = JsMap::new(); t.set(String::from("n"), String::from("north")); t.set(String::from("s"), String::from("south")); t.set(String::from("e"), String::from("east")); t.set(String::from("w"), String::from("west")); t.set(String::from("ne"), String::from("north-east")); t.set(String::from("nw"), String::from("north-west")); t.set(String::from("se"), String::from("south-east")); t.set(String::from("sw"), String::from("south-west")); t.set(String::from("ext"), String::from("outside")); t.set(String::from("breakroom"), String::from("break room")); t.set(String::from("mgr"), String::from("manager's")); t } }

use crate::game::Game;
pub struct Row {
    /// its id on the menu and in a man's assignment
    pub id: &'static str,
    /// what the rules that do not know it by name need to know of it
    pub is: &'static [Is],
    pub label: fn(&mut Game, &OrderTarget) -> String,
    pub can: Option<fn(&mut Game, GuardId, &OrderTarget) -> Option<String>>,
    pub only: Option<fn(&mut Game, &[GuardId], &OrderTarget) -> Vec<GuardId>>,
    pub valid: Option<fn(&mut Game, &OrderTarget, &[GuardId]) -> Result<(), Option<String>>>,
    pub issue: fn(&mut Game, &[GuardId], &OrderTarget, Option<&GiveOpts>) -> Option<String>,
    /// the ring does not offer the row on this target at all
    pub gone: Option<fn(&Game, &crate::seats::SeatRef, &OrderTarget, &[GuardId]) -> bool>,
    /// it stands in for the row its data names (`stands_in`) now
    pub when: Option<fn(&Game, &crate::seats::SeatRef, &OrderTarget, &[GuardId]) -> bool>,
    pub tick: Option<fn(&mut Game, GuardId, OrdersAssignmentId)>,
    pub frame: Option<fn(&mut Game)>,
    /// its lines in the squad's HUD block, for this seat: whether it has anything to say is its own to answer
    pub notes: Option<fn(&Game, &crate::seats::SeatRef) -> Vec<crate::view::Note>>,
}
impl Row {
    /// An order with the three things every order has. The rest are `None` until its file says otherwise:
    /// `Row { frame: Some(frame), ..Row::new(ID, label, issue) }`.
    pub const fn new(id: &'static str, label: fn(&mut Game, &OrderTarget) -> String, issue: fn(&mut Game, &[GuardId], &OrderTarget, Option<&GiveOpts>) -> Option<String>) -> Row {
        Row { id, is: &[], label, can: None, only: None, valid: None, issue, gone: None, when: None, tick: None, frame: None, notes: None }
    }
}
fn none_label(_g: &mut Game, _t: &OrderTarget) -> String { String::new() }
fn none_issue(_g: &mut Game, _men: &[GuardId], _t: &OrderTarget, _o: Option<&GiveOpts>) -> Option<String> { Some(String::from("no such order.")) }
impl Row {
    /// The row a blank OrderDef holds until it is dealt its own: it says nothing and refuses.
    pub const NONE: Row = Row::new("", none_label, none_issue);
}
/// WHAT AN ORDER IS, to a rule that has no reason to know it by name. An order says these of itself (`Row.is`); a rule asks
/// (`rows::is`, of a man's assignment; `rows::id_is`, of an id).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Is {
    /// he follows the player: after a throw he falls in again, and a rule that looks for a free man counts him
    Follows,
    /// he stays where he was told: after a throw he comes back to the spot alone, and with no mark to watch he is walked to where he sees
    Holds,
    /// he watches a side from a post: the room fight and the rescue read him as watching it
    Covers,
    /// he is on a door, waiting for the word: the door's tools and the room fight read his place in the stack
    Stacks,
    /// his eyes are his own on his mark too, wherever it has named nothing to watch (on the walk to a mark they are under every order)
    LooksAbout,
    /// a man under it who is sent to throw goes back to it afterwards
    Resumed,
    /// once he has arrived he goes low when a man of the other pair stands in his line
    KeepsLane,
    /// his walk to a mark goes low while the player is low
    WalksLow,
    /// giving it starts the fight: the squad's held fire lifts (at the word if it waits, when it goes off if it `Blows`)
    LiftsFire,
    /// it lifts the held fire only when it goes off, not when it is given
    Blows,
    /// it gives a silent squad away, so its row says so and giving it spends the quiet
    Loud,
    /// he is going in by a door: the rescue reads a man at his post for one as standing to it
    Enters,
    /// it is one of the two pairs' rows: they come and go with the pairs
    Pair,
    /// a man under it has a marked man for his own
    Take,
}
impl core::fmt::Debug for Row {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result { write!(f, "Row({})", self.id) }
}

/// WHAT THE SQUAD'S PANEL SAYS OF A SEAT BESIDE ITS MENU: kept by the tick, read by whoever draws it (orders/shown.rs).
#[derive(Clone, Debug, Default, PartialEq, ToBytes)]
pub struct Shown {
    /// the quick key's line for what the seat's cursor is on: the row it would give, and what it is on. None: a menu is up, or the key gives nothing there
    pub quick: Option<String>,
}
