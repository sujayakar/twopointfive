//! The types of `doortools`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// the step onto the diagonals, kept until the first can's burst has run dry (doortools.ts toThreshold's `out`)
#[derive(Clone, Debug)]
pub enum ThresholdOut {
    Step { lead: GuardId, f: Option<crate::doortools::DoorToolsThrFar>, sl: crate::doortools::DoorToolsDoneItemSliceValue, sf: Option<crate::doortools::DoorToolsDoneItemSliceValue>, light: bool, name: String },
}
/// walkLen for one deal's worth of questions, each pair of places measured once (doortools.ts walks); held in a local of the deal, never in a field
#[derive(Clone, Debug)]
pub enum Walks {
    Memo { memo: JsMap<String, f64>, lim: f64 },
}

/// (TypeScript: DoorTools, src/game/doortools.ts:770)
#[derive(Clone, Debug)]
pub struct DoorTools {
    pub knobs: crate::knobs::DoortoolsK,
    pub charges: f64,
    pub bangs: f64,
    pub frags: f64,
    pub frag_next: bool,
    pub big_next: bool,
    pub big_row: Option<bool>,
    pub ops: Vec<DoorOpId>,
    pub done: Vec<crate::doortools::DoorToolsDoneItem>,
    pub set: Vec<ChargeId>,
    pub linked: Vec<DoorId>,
    pub lights: Option<bool>,
    pub log: Vec<crate::doortools::DoorToolsLogItem>,
    pub stats: crate::doortools::DoorToolsStats,
    pub roster: Vec<GuardId>,
    pub seen: Option<crate::floorseen::FloorSeen>,
    pub seen_watch: bool,
    pub led: crate::doortools::DoorToolsLed,
    pub go_key: Option<String>,
    pub words: Vec<WordId>,
    pub word_key_t: f64,
    pub thr_key: Option<String>,
    pub thr_sig: f64,
    pub sam_of: Option<crate::doortools::DoorToolsSamOf>,
    pub sam_spent: f64,
    pub tried: JsMap<DoorId, f64>,
    pub close_at: JsMap<ChargeId, crate::doortools::DoorToolsCloseAtValue>,
    pub gate_asked: JsSet<DoorOpId>,
    pub held: JsMap<GuardId, crate::doortools::DoorToolsHeldValue>,
    pub watches: Vec<DoorOpId>,
    pub away_why: String,
    pub away_far: bool,
    pub big_said: JsSet<DoorOpId>,
}
impl DoorTools {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        DoorTools {
            knobs: crate::knobs::DoortoolsK { charges: 2.0, bangs: 3.0, blow_key: String::from("KeyF"), word_on_key: true, word_tap: 0.6, thr_hold_first: true, thr_bang_first: true, thr_outside_row: true, thr_leave_fall_in: false, thr_hands_off: false, thr_held_all: false, thr_menu_word: true, stack_cap: 60.0, still_secs: 2.5, kick_cap: 5.0, in_cap: 9.0, shove: 5.5, hold_open: 10.0, toss_after_open: 0.15, bang_in: 2.4, fuse_after_land: 0.35, air_in: 1.6, air_h: 1.2, toss_cap: 1.6, toss_after_kick: 0.05, second_throws: true, throw_reach: 4.0, toss_turn: 0.0, frag_way: true, frag_way_wait: 1.0, frag_way_open: 1.15, frag_way_ajar: 0.6, bang_way: true, open_hand: true, gate_hand: true, last_out: true, last_out_m: 3.0, after_bang: 0.6, quiet: 0.15, entry_cap: 2.8, second: 0.25, sweep_rad: 0.9, sweep_secs: 0.6, corners: true, corner_secs: 0.3, corner_rad: 0.3, corner_cap: 1.6, corner_owed: true, collapse: true, corner_turn: true, lead_seen: true, lead_seen_min: 12.0, lead_seen_secs: 30.0, lead_seen_m: 11.0, sectors: true, sector_secs: 0.6, search: true, scan_rate: 1.4, look_rad: 0.3, look_m: 11.0, dead_cells: 8.0, search_moves: 3.0, search_cap: 24.0, search_cells: 900.0, tuck: 0.4, edge_out: 0.4, bang_file: 1.1, set_secs: 2.0, standoff: alloc::vec![2.6, 2.0, 1.5], knock_m: 2.0, stun_m: 8.0, stun_secs: alloc::vec![6.0, 2.5], stagger_secs: 2.0, backblast_m: 1.6, backblast_secs: 1.5, blow_vel: 12.0, slice_back: 1.0, slice_look: 0.35, slice_edge: 0.12, slice_deep: 3.5, slice_pace: 0.9, word_stack: 20.0, word_in: 6.0, room_back: alloc::vec![2.8, 3.7, 4.6, 5.5], pod_apart: 1.2, thr_pull: true, thr_pull_at: alloc::vec![0.45, 1.8], thr_still: alloc::vec![0.45, 1.2], thr_arc: alloc::vec![alloc::vec![-0.45, 1.2], alloc::vec![0.25, 1.8]], thr_min: 4.0, thr_quiet: 2.0, thr_cap: 25.0, thr_bangs: 2.0, thr_pocket: 12.0, thr_shot_m: 8.0, thr_throw_m: 5.0, thr_flash_m: 7.0, thr_turn: 0.5, thr_next: 0.3, thr_cover: true, thr_slice: false, thr_light: 1.0, thr_in: 4.0, thr_pie: false, thr_pie_arc: alloc::vec![alloc::vec![-0.2, 1.4], alloc::vec![-0.05, 1.6], alloc::vec![0.08, 1.8]], thr_pie_apart: 0.65, thr_lean: false, thr_lean_m: 0.15, thr_knee_in: -0.1, thr_pie_one: false, thr_lean_look: 1.1, thr_doors: true, thr_doors_m: 5.5, thr_doors_lit: true, thr_doors_walk: 16.0, thr_doors_stay: 1.0, other_door_m: 9.0, own_can: true, buddy: true, buddy_m: 5.0, buddy_pair_m: 12.0, buddy_left_m: 8.0, flow_on: true, flow_cells: 28.0, flow_moves: 5.0, flow_reach: 5.5, flow_deep: 3.0, flow_bound: 8.0, flow_secs: 10.0, flow_fight: 20.0, open_air: true, flow_max: 400.0, test_door: false, outside_plain: false, word_no_can: false, word_cuts: false, word_cuts_cap: 8.0, word_small: false, bang_into: false, bang_into_min: 1.5, bang_into_lit: 0.9, bang_into_deep: 6.0, bang_into_wide: 4.2, bang_hand_m: 0.9, bang_pass_v: 6.5, halt_back: false, behind_sam: false, toss_mate: false, charge_answer: false, pod_frames: false, pod_frame_m: 0.55, thr_hand: false, thr_hand_wait: 6.0, thr_hand_every: 0.25, thr_no_can: false, thr_frag_hold: false, thr_doors_leaf: false, frag_hold: false, frag_hold_cap: 8.0, threat_first: true, threat_secs: 12.0, threat_cap: 1.5, threat_near_m: 1.5, threat_stuck: 1.0, buddy_doors: false, thr_set: false, thr_set_secs: 1.5, thr_tube: true, thr_dark: true, thr_first: 1.0, thr_first_min: 2.0, thr_first_stale: 0.0, thr_first_in: 0.0, thr_first_only: true, thr_last: true, thr_last_m: 2.0, thr_last_fresh: 3.0 },
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
            close_at: JsMap::new(),
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

/// (TypeScript: inline at src/game/doortools.ts:775 DoorTools.done, src/game/doortools.ts:775 DoorTools.done)
#[derive(Clone, Debug)]
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
    pub search: Option<crate::doortools::RoomSearch>,
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
impl DoorToolsDoneItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorToolsDoneItem {
            kind: crate::doortools::DoorToolKind::Charge,
            door: DoorId(0),
            side: 0.0,
            coded: None,
            men: Vec::new(),
            lead: GuardId(0),
            fled: None,
            fled_w: None,
            fled_t: None,
            fled_said: None,
            fled_go: None,
            rear: JsSet::new(),
            lane: None,
            from: JsMap::new(),
            phase: crate::doortools::Phase::Slice,
            t0: 0.0,
            bang_held: false,
            frag: None,
            frag_way_t: None,
            leaf_a: None,
            leaf_t: None,
            mine: JsMap::new(),
            post: JsMap::new(),
            kick: None,
            no_gun: None,
            big: None,
            open_t: 0.0,
            can: None,
            toss_t: 0.0,
            land_t: 0.0,
            bang_t: 0.0,
            bang_at: None,
            queue: Vec::new(),
            set_t: 0.0,
            stow_off_t: None,
            pod: JsMap::new(),
            on_pod: JsMap::new(),
            slice: JsMap::new(),
            crossed: JsMap::new(),
            corner: JsMap::new(),
            search: None,
            first: None,
            threw: None,
            aim: None,
            pass: None,
            beats: Vec::new(),
            on_word: false,
            word: None,
            hold: 0.0,
            ready_t: 0.0,
            gate: None,
            thr: None,
            thr_hold: None,
            thr_pre: None,
            worded: None,
            last_stack: None,
            last_hold: None,
            last_tossed: None,
            other: None,
            threat: None,
            ended: String::new(),
        }
    }
}
impl Default for DoorToolsDoneItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum DoorToolKind {
    Charge,
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

/// (TypeScript: inline at src/game/doortools.ts:636 DoorToolsDoneItem.lane, src/game/doortools.ts:636)
#[derive(Clone, Debug)]
pub struct DoorToolsDoneItemLane {
    pub gd: GuardId,
    pub door: DoorId,
    pub cover: OrdersAssignmentId,
    pub back: crate::doortools::DoorToolsDoneItemLaneBack,
    pub at: Option<f64>,
}
impl DoorToolsDoneItemLane {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorToolsDoneItemLane {
            gd: GuardId(0),
            door: DoorId(0),
            cover: OrdersAssignmentId(0),
            back: crate::doortools::DoorToolsDoneItemLaneBack::zeroed(),
            at: None,
        }
    }
}
impl Default for DoorToolsDoneItemLane { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/doortools.ts:636 DoorToolsDoneItemLane.back, src/game/doortools.ts:636 DoorToolsDoneItemLane.back)
#[derive(Clone, Debug)]
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
impl DoorToolsDoneItemLaneBack {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorToolsDoneItemLaneBack {
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
impl Default for DoorToolsDoneItemLaneBack { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/orders.ts:437 DoorToolsDoneItemLaneBack.move, src/game/orders.ts:437)
#[derive(Clone, Copy, Debug)]
pub struct DoorToolsDoneItemLaneBackMove {
    pub stance: crate::character::Stance,
    pub upper: crate::character::UpperMode,
}
impl DoorToolsDoneItemLaneBackMove {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorToolsDoneItemLaneBackMove {
            stance: crate::character::Stance::None,
            upper: crate::character::UpperMode::None,
        }
    }
}
impl Default for DoorToolsDoneItemLaneBackMove { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/orders.ts:480 DoorToolsDoneItemLaneBack.tries, src/game/orders.ts:480)
#[derive(Clone, Debug)]
pub struct DoorToolsDoneItemLaneBackTries {
    pub door: DoorId,
    pub side: f64,
    pub post: crate::orders::StackPost,
    pub at: V3,
    pub on: crate::doortools::DoorToolsDoneItemLaneBackTriesOn,
}
impl DoorToolsDoneItemLaneBackTries {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorToolsDoneItemLaneBackTries {
            door: DoorId(0),
            side: 0.0,
            post: crate::orders::StackPost::zeroed(),
            at: [0.0; 3],
            on: crate::doortools::DoorToolsDoneItemLaneBackTriesOn::zeroed(),
        }
    }
}
impl Default for DoorToolsDoneItemLaneBackTries { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/orders.ts:480 DoorToolsDoneItemLaneBackTries.on, src/game/orders.ts:480 DoorToolsDoneItemLaneBackTries.on)
#[derive(Clone, Debug)]
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
impl DoorToolsDoneItemLaneBackTriesOn {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorToolsDoneItemLaneBackTriesOn {
            post: None,
            going: String::new(),
            order: String::new(),
            face: None,
            there: String::new(),
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
impl Default for DoorToolsDoneItemLaneBackTriesOn { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Phase {
    Slice,
    Kick,
    Held,
    Stack,
    Wait,
    Ready,
    Toss,
    Fuse,
    Enter,
    Place,
    Standoff,
    Armed,
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
            Phase::Ready => "ready",
            Phase::Toss => "toss",
            Phase::Fuse => "fuse",
            Phase::Enter => "enter",
            Phase::Place => "place",
            Phase::Standoff => "standoff",
            Phase::Armed => "armed",
            Phase::Thr => "thr",
            Phase::Pocket => "pocket",
        }
    }
}

/// (TypeScript: inline at src/game/doortools.ts:654 DoorToolsDoneItem.kick, src/game/doortools.ts:654)
#[derive(Clone, Copy, Debug)]
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
impl DoorToolsDoneItemKick {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorToolsDoneItemKick {
            pos: [0.0; 3],
            yaw: 0.0,
            t: 0.0,
            pick: None,
            hand: None,
            gate: None,
            on_it: None,
            at: None,
            by: None,
            done_t: None,
            off_t: None,
            gun: None,
        }
    }
}
impl Default for DoorToolsDoneItemKick { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/doortools.ts:654 DoorToolsDoneItemKick.gun, src/game/doortools.ts:654)
#[derive(Clone, Copy, Debug)]
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
impl DoorToolsDoneItemKickGun {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorToolsDoneItemKickGun {
            by: GuardId(0),
            at: [0.0; 3],
            n: 0.0,
            next_t: 0.0,
            silent: false,
            own: None,
            hammer: None,
            shut: None,
        }
    }
}
impl Default for DoorToolsDoneItemKickGun { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/doortools.ts:663 DoorToolsDoneItem.queue, src/game/doortools.ts:663)
#[derive(Clone, Copy, Debug)]
pub struct DoorToolsDoneItemQueueItem {
    pub gd: GuardId,
    pub at: f64,
    pub pod: crate::entrypoints::Pod,
}
impl DoorToolsDoneItemQueueItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorToolsDoneItemQueueItem {
            gd: GuardId(0),
            at: 0.0,
            pod: crate::entrypoints::Pod::zeroed(),
        }
    }
}
impl Default for DoorToolsDoneItemQueueItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/doortools.ts:669 DoorToolsDoneItem.slice, src/game/doortools.ts:669)
#[derive(Clone, Debug)]
pub struct DoorToolsDoneItemSliceValue {
    pub pts: Vec<V3>,
    pub looks: Vec<Vec<V3>>,
    pub i: f64,
    pub j: f64,
    pub t: f64,
    pub ahead: Option<GuardId>,
    pub pie: Option<crate::doortools::DoorToolsDoneItemSliceValuePie>,
}
impl DoorToolsDoneItemSliceValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorToolsDoneItemSliceValue {
            pts: Vec::new(),
            looks: Vec::new(),
            i: 0.0,
            j: 0.0,
            t: 0.0,
            ahead: None,
            pie: None,
        }
    }
}
impl Default for DoorToolsDoneItemSliceValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/doortools.ts:669 DoorToolsDoneItemSliceValue.pie, src/game/doortools.ts:669)
#[derive(Clone, Debug)]
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
impl DoorToolsDoneItemSliceValuePie {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorToolsDoneItemSliceValuePie {
            eyes: Vec::new(),
            lean: 0.0,
            end: 0.0,
            end_on: false,
            knee: Vec::new(),
            fin: false,
            begun: false,
            at: 0.0,
            inw: [0.0; 3],
            gun: 0.0,
            gun_at: 0.0,
            on: false,
            to: None,
        }
    }
}
impl Default for DoorToolsDoneItemSliceValuePie { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/doortools.ts:674 DoorToolsDoneItem.corner, src/game/doortools.ts:674)
#[derive(Clone, Debug)]
pub struct DoorToolsDoneItemCornerValue {
    pub pts: Vec<V3>,
    pub i: f64,
    pub on: f64,
    pub fight: Option<f64>,
    pub last: Option<f64>,
    pub turned: Option<bool>,
}
impl DoorToolsDoneItemCornerValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorToolsDoneItemCornerValue {
            pts: Vec::new(),
            i: 0.0,
            on: 0.0,
            fight: None,
            last: None,
            turned: None,
        }
    }
}
impl Default for DoorToolsDoneItemCornerValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: RoomSearch, src/game/doortools.ts:740)
#[derive(Clone, Debug)]
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
impl RoomSearch {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RoomSearch {
            room: RoomfightRoomId(0),
            pts: Vec::new(),
            looked: Vec::new(),
            los: JsMap::new(),
            yaw: JsMap::new(),
            at: JsMap::new(),
            moves: 0.0,
            left_at_move: 0.0,
            left: 0.0,
            flows: None,
            closes: None,
            flow_said: None,
            late: None,
            cell_at: None,
            fight_t: None,
            fight_at: None,
        }
    }
}
impl Default for RoomSearch { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/doortools.ts:740 RoomSearch.los, src/game/doortools.ts:740)
#[derive(Clone, Debug)]
pub struct RoomSearchLosValue {
    pub at: V3,
    pub ok: Vec<i8>,
}
impl RoomSearchLosValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RoomSearchLosValue {
            at: [0.0; 3],
            ok: Vec::new(),
        }
    }
}
impl Default for RoomSearchLosValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/doortools.ts:678 DoorToolsDoneItem.aim, src/game/doortools.ts:678)
#[derive(Clone, Copy, Debug)]
pub struct DoorToolsDoneItemAim {
    pub at: V3,
    pub from: V3,
}
impl DoorToolsDoneItemAim {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorToolsDoneItemAim {
            at: [0.0; 3],
            from: [0.0; 3],
        }
    }
}
impl Default for DoorToolsDoneItemAim { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/doortools.ts:678 DoorToolsDoneItem.pass, src/game/doortools.ts:678)
#[derive(Clone, Copy, Debug)]
pub struct DoorToolsDoneItemPass {
    pub to: V3,
    pub v0: V3,
    pub t: f64,
}
impl DoorToolsDoneItemPass {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorToolsDoneItemPass {
            to: [0.0; 3],
            v0: [0.0; 3],
            t: 0.0,
        }
    }
}
impl Default for DoorToolsDoneItemPass { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/doortools.ts:679 DoorToolsDoneItem.beats, src/game/doortools.ts:679)
#[derive(Clone, Debug)]
pub struct DoorToolsDoneItemBeatsItem {
    pub t: f64,
    pub what: String,
}
impl DoorToolsDoneItemBeatsItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorToolsDoneItemBeatsItem {
            t: 0.0,
            what: String::new(),
        }
    }
}
impl Default for DoorToolsDoneItemBeatsItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Threshold, src/game/doortools.ts:711)
#[derive(Clone, Debug)]
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
impl Threshold {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Threshold {
            jamb: Vec::new(),
            pods: Vec::new(),
            quiet_t: 0.0,
            ev_t: 0.0,
            contact: JsSet::new(),
            up: Vec::new(),
            down: 0.0,
            sliced: false,
            thrower: None,
            tossed: 0.0,
            next_t: 0.0,
            aim: None,
            from: None,
            banged: None,
            handed: false,
            los_t: 0.0,
            lit_c: None,
            tube_c: None,
            hold: false,
            told: false,
            force: false,
            hand_w: None,
            hand_t: None,
            hand_done: None,
            fled: None,
            frag_t: None,
            frag_said: None,
            step_owed: None,
            owed_t: None,
            cut_owed: None,
            first: None,
            out: None,
            wide: None,
            early: None,
            asked: None,
            word_step: None,
            shut_t: None,
        }
    }
}
impl Default for Threshold { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
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

/// (TypeScript: inline at src/game/doortools.ts:688 DoorToolsDoneItem.thrPre, src/game/doortools.ts:688)
#[derive(Clone, Copy, Debug)]
pub struct DoorToolsDoneItemThrPre {
    pub far: GuardId,
    pub t: f64,
}
impl DoorToolsDoneItemThrPre {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorToolsDoneItemThrPre {
            far: GuardId(0),
            t: 0.0,
        }
    }
}
impl Default for DoorToolsDoneItemThrPre { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/doortools.ts:694 DoorToolsDoneItem.other, src/game/doortools.ts:694)
#[derive(Clone, Copy, Debug)]
pub struct DoorToolsDoneItemOtherValue {
    pub gd: GuardId,
    pub w: V3,
}
impl DoorToolsDoneItemOtherValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorToolsDoneItemOtherValue {
            gd: GuardId(0),
            w: [0.0; 3],
        }
    }
}
impl Default for DoorToolsDoneItemOtherValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/doortools.ts:697 DoorToolsDoneItem.threat, src/game/doortools.ts:697)
#[derive(Clone, Debug)]
pub struct DoorToolsDoneItemThreat {
    pub known: Vec<crate::doortools::KnownThreat>,
    pub ev_t: f64,
    pub room: Option<crate::doortools::DoorToolsDoneItemThreatRoom>,
    pub room_t: Option<f64>,
    pub deal: Option<JsMap<GuardId, crate::doortools::ThreatWatch>>,
}
impl DoorToolsDoneItemThreat {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorToolsDoneItemThreat {
            known: Vec::new(),
            ev_t: 0.0,
            room: None,
            room_t: None,
            deal: None,
        }
    }
}
impl Default for DoorToolsDoneItemThreat { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: KnownThreat, src/game/doortools.ts:700)
#[derive(Clone, Copy, Debug)]
pub struct KnownThreat {
    pub at: V3,
    pub t: f64,
    pub by: Option<GuardId>,
    pub how: crate::doortools::KnownThreatHow,
}
impl KnownThreat {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        KnownThreat {
            at: [0.0; 3],
            t: 0.0,
            by: None,
            how: crate::doortools::KnownThreatHow::Shot,
        }
    }
}
impl Default for KnownThreat { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum KnownThreatHow {
    Shot,
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

/// (TypeScript: inline at src/game/doortools.ts:697 DoorToolsDoneItemThreat.room, src/game/doortools.ts:697)
#[derive(Clone, Debug)]
pub struct DoorToolsDoneItemThreatRoom {
    pub cells: JsSet<crate::Num>,
}
impl DoorToolsDoneItemThreatRoom {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorToolsDoneItemThreatRoom {
            cells: JsSet::new(),
        }
    }
}
impl Default for DoorToolsDoneItemThreatRoom { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: ThreatWatch, src/game/doortools.ts:703)
#[derive(Clone, Copy, Debug)]
pub struct ThreatWatch {
    pub th: crate::doortools::KnownThreat,
    pub lead: Option<GuardId>,
    pub on: f64,
    pub fight: f64,
    pub last: f64,
    pub done_t: f64,
    pub faced: Option<bool>,
    pub at: Option<V3>,
    pub still: Option<f64>,
}
impl ThreatWatch {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ThreatWatch {
            th: crate::doortools::KnownThreat::zeroed(),
            lead: None,
            on: 0.0,
            fight: 0.0,
            last: 0.0,
            done_t: 0.0,
            faced: None,
            at: None,
            still: None,
        }
    }
}
impl Default for ThreatWatch { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/doortools.ts:781 DoorTools.log, src/game/doortools.ts:781)
#[derive(Clone, Debug)]
pub struct DoorToolsLogItem {
    pub t: f64,
    pub what: String,
}
impl DoorToolsLogItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorToolsLogItem {
            t: 0.0,
            what: String::new(),
        }
    }
}
impl Default for DoorToolsLogItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/doortools.ts:782 DoorTools.stats, src/game/doortools.ts:782)
#[derive(Clone, Copy, Debug)]
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
impl DoorToolsStats {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorToolsStats {
            orders: 0.0,
            opened: 0.0,
            kicks: 0.0,
            shot: 0.0,
            picks: 0.0,
            no_boot: 0.0,
            hand_kept: 0.0,
            tested: 0.0,
            halted: 0.0,
            bangs: 0.0,
            own_flashed: 0.0,
            frags: 0.0,
            blown: 0.0,
            knocked: 0.0,
            stunned: 0.0,
            backblast: 0.0,
            entries: 0.0,
            words: 0.0,
            stepped_back: 0.0,
            pods_moved: 0.0,
            sectors: 0.0,
            sent_on: 0.0,
            sent_alone: 0.0,
            door_alone: 0.0,
            sent_pair: 0.0,
            not_sent: 0.0,
            flowed: 0.0,
            closed_up: 0.0,
            flow_late: 0.0,
        }
    }
}
impl Default for DoorToolsStats { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/doortools.ts:789 DoorTools.led, src/game/doortools.ts:789)
#[derive(Clone, Copy, Debug)]
pub struct DoorToolsLed {
    pub entries: f64,
    pub by_seen: f64,
}
impl DoorToolsLed {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorToolsLed {
            entries: 0.0,
            by_seen: 0.0,
        }
    }
}
impl Default for DoorToolsLed { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/doortools.ts:949 DoorTools.samOf, src/game/doortools.ts:949 DoorTools.samOf)
#[derive(Clone, Copy, Debug)]
pub struct DoorToolsSamOf {
}
impl DoorToolsSamOf {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorToolsSamOf {
        }
    }
}
impl Default for DoorToolsSamOf { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/doortools.ts:1438 DoorTools.closeAt, src/game/doortools.ts:1438)
#[derive(Clone, Debug)]
pub struct DoorToolsCloseAtValue {
    pub t: f64,
    pub w: String,
}
impl DoorToolsCloseAtValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorToolsCloseAtValue {
            t: 0.0,
            w: String::new(),
        }
    }
}
impl Default for DoorToolsCloseAtValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/doortools.ts:2792 DoorTools.held, src/game/doortools.ts:2792)
#[derive(Clone, Copy, Debug)]
pub struct DoorToolsHeldValue {
    pub a: OrdersAssignmentId,
    pub face: f64,
    pub d: f64,
    pub hit_t: f64,
    pub arr_t: f64,
    pub out: Option<bool>,
    pub door: Option<DoorId>,
}
impl DoorToolsHeldValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorToolsHeldValue {
            a: OrdersAssignmentId(0),
            face: 0.0,
            d: 0.0,
            hit_t: 0.0,
            arr_t: 0.0,
            out: None,
            door: None,
        }
    }
}
impl Default for DoorToolsHeldValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Word, src/game/doortools.ts:617)
#[derive(Clone, Copy, Debug)]
pub struct Word {
    pub t0: f64,
    pub go1: f64,
    pub fled_t: Option<f64>,
}
impl Word {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Word {
            t0: 0.0,
            go1: 0.0,
            fled_t: None,
        }
    }
}
impl Default for Word { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: DoorOp, src/game/doortools.ts:623)
#[derive(Clone, Debug)]
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
    pub search: Option<crate::doortools::RoomSearch>,
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
}
impl DoorOp {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorOp {
            kind: crate::doortools::DoorToolKind::Charge,
            door: DoorId(0),
            side: 0.0,
            coded: None,
            men: Vec::new(),
            lead: GuardId(0),
            fled: None,
            fled_w: None,
            fled_t: None,
            fled_said: None,
            fled_go: None,
            rear: JsSet::new(),
            lane: None,
            from: JsMap::new(),
            phase: crate::doortools::Phase::Slice,
            t0: 0.0,
            bang_held: false,
            frag: None,
            frag_way_t: None,
            leaf_a: None,
            leaf_t: None,
            mine: JsMap::new(),
            post: JsMap::new(),
            kick: None,
            no_gun: None,
            big: None,
            open_t: 0.0,
            can: None,
            toss_t: 0.0,
            land_t: 0.0,
            bang_t: 0.0,
            bang_at: None,
            queue: Vec::new(),
            set_t: 0.0,
            stow_off_t: None,
            pod: JsMap::new(),
            on_pod: JsMap::new(),
            slice: JsMap::new(),
            crossed: JsMap::new(),
            corner: JsMap::new(),
            search: None,
            first: None,
            threw: None,
            aim: None,
            pass: None,
            beats: Vec::new(),
            on_word: false,
            word: None,
            hold: 0.0,
            ready_t: 0.0,
            gate: None,
            thr: None,
            thr_hold: None,
            thr_pre: None,
            worded: None,
            last_stack: None,
            last_hold: None,
            last_tossed: None,
            other: None,
            threat: None,
        }
    }
}
impl Default for DoorOp { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: DoorGate, src/game/doortools.ts:744)
#[derive(Clone, Debug)]
pub struct DoorGate {
    pub ops: Vec<DoorOpId>,
    pub go: bool,
    pub open_t: f64,
}
impl DoorGate {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorGate {
            ops: Vec::new(),
            go: false,
            open_t: 0.0,
        }
    }
}
impl Default for DoorGate { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: SetCharge, src/game/doortools.ts:746)
#[derive(Clone, Debug)]
pub struct SetCharge {
    pub door: DoorId,
    pub side: f64,
    pub t: f64,
    pub by: String,
}
impl SetCharge {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SetCharge {
            door: DoorId(0),
            side: 0.0,
            t: 0.0,
            by: String::new(),
        }
    }
}
impl Default for SetCharge { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/doortools.ts:613 PIE_GUN, src/game/doortools.ts:613)
#[derive(Clone, Copy, Debug)]
pub struct PIEGUN {
    pub after: f64,
    pub step: f64,
    pub most: f64,
    pub every: f64,
}
impl PIEGUN {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PIEGUN {
            after: 0.0,
            step: 0.0,
            most: 0.0,
            every: 0.0,
        }
    }
}
impl Default for PIEGUN { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/doortools.ts:760 SKY, src/game/doortools.ts:760)
#[derive(Clone, Copy, Debug)]
pub struct DoortoolsModSKYValue {
    pub n: f64,
    pub open: bool,
}
impl DoortoolsModSKYValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoortoolsModSKYValue {
            n: 0.0,
            open: false,
        }
    }
}
impl Default for DoortoolsModSKYValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/doortools.ts:761 underSky(room), src/game/doortools.ts:761)
#[derive(Clone, Debug)]
pub struct UnderSkyRoom {
    pub cells: JsSet<crate::Num>,
}
impl UnderSkyRoom {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        UnderSkyRoom {
            cells: JsSet::new(),
        }
    }
}
impl Default for UnderSkyRoom { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/doortools.ts:768 openFloor(room), src/game/doortools.ts:768)
#[derive(Clone, Debug)]
pub struct OpenFloorRoom {
    pub cells: JsSet<crate::Num>,
}
impl OpenFloorRoom {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        OpenFloorRoom {
            cells: JsSet::new(),
        }
    }
}
impl Default for OpenFloorRoom { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
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

/// (TypeScript: words)
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

/// (TypeScript: a union)
#[derive(Clone, Debug)]
pub enum DoorToolsStackable {
    Text(String),
    Bool(bool),
}

/// (TypeScript: words)
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

/// (TypeScript: inline at src/game/doortools.ts:998 bootSpot ->, src/game/doortools.ts:998)
#[derive(Clone, Copy, Debug)]
pub struct DoorToolsBootSpot {
    pub pos: V3,
    pub yaw: f64,
}
impl DoorToolsBootSpot {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorToolsBootSpot {
            pos: [0.0; 3],
            yaw: 0.0,
        }
    }
}
impl Default for DoorToolsBootSpot { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/doortools.ts:1114 stackLeft(post), ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1604)
#[derive(Clone, Copy, Debug)]
pub struct DoorToolsStackLeftPostValue {
    pub lead: bool,
    pub pos: V3,
    pub face: f64,
    pub stack_side: f64,
}
impl DoorToolsStackLeftPostValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorToolsStackLeftPostValue {
            lead: false,
            pos: [0.0; 3],
            face: 0.0,
            stack_side: 0.0,
        }
    }
}
impl Default for DoorToolsStackLeftPostValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
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

/// (TypeScript: inline at src/game/doortools.ts:1138 assign(a), src/game/doortools.ts:1138 assign(a))
#[derive(Clone, Debug)]
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
impl DoorToolsAssignA {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorToolsAssignA {
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
impl Default for DoorToolsAssignA { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: a union)
#[derive(Clone, Debug)]
pub enum DoorToolsIssue {
    Text(String),
    Arm1(()),
}

/// (TypeScript: words)
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

/// (TypeScript: a union)
#[derive(Clone, Debug)]
pub enum DoorToolsOrder {
    Text(String),
    DoorOp(DoorOpId),
}

/// (TypeScript: a union)
#[derive(Clone, Debug)]
pub enum DoorToolsThreshold {
    Text(String),
    Arm1(()),
}

/// (TypeScript: a union)
#[derive(Clone, Debug)]
pub enum DoorToolsThresholdHold {
    Text(String),
    Arm1(()),
}

/// (TypeScript: a union)
#[derive(Clone, Debug)]
pub enum DoorToolsCanThreshold {
    Text(String),
    Bool(bool),
}

/// (TypeScript: a union)
#[derive(Clone, Debug)]
pub enum DoorToolsThresholdOn {
    Text(String),
    DoorOp(DoorOpId),
}

/// (TypeScript: words)
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

/// (TypeScript: a union)
#[derive(Clone, Debug)]
pub enum DoorToolsThresholdThenT {
    Floor(crate::orders::OrderTargetFloor),
    Door(crate::orders::OrderTargetDoor),
    None(crate::orders::OrderTargetNone),
    Foe(crate::orders::OrderTargetFoe),
}

/// (TypeScript: a union)
#[derive(Clone, Debug)]
pub enum DoorToolsThresholdThen {
    Text(String),
    Arm1(()),
}

/// (TypeScript: words)
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

/// (TypeScript: a union)
#[derive(Clone, Debug)]
pub enum DoorToolsThenOnWord {
    Text(String),
    Arm1(()),
}

/// (TypeScript: words)
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

/// (TypeScript: a union)
#[derive(Clone, Debug)]
pub enum DoorToolsCanThresholdThenT {
    Floor(crate::orders::OrderTargetFloor),
    Door(crate::orders::OrderTargetDoor),
    None(crate::orders::OrderTargetNone),
    Foe(crate::orders::OrderTargetFoe),
}

/// (TypeScript: a union)
#[derive(Clone, Debug)]
pub enum DoorToolsCanThresholdThen {
    Text(String),
    Bool(bool),
}

/// (TypeScript: words)
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

/// (TypeScript: a union)
#[derive(Clone, Debug)]
pub enum DoorToolsStart {
    Text(String),
    DoorOp(DoorOpId),
}

/// (TypeScript: a union)
#[derive(Clone, Debug)]
pub enum DoorToolsStartCharge {
    Text(String),
    Arm1(()),
}

/// (TypeScript: a union)
#[derive(Clone, Debug)]
pub enum DoorToolsStandoffFits {
    Text(String),
    Bool(bool),
}

/// (TypeScript: inline at src/game/doortools.ts:1464 samJob ->, src/game/doortools.ts:1464)
#[derive(Clone, Copy, Debug)]
pub struct DoorToolsSamJob {
    pub p: V3,
    pub n: V3,
    pub at: V3,
}
impl DoorToolsSamJob {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorToolsSamJob {
            p: [0.0; 3],
            n: [0.0; 3],
            at: [0.0; 3],
        }
    }
}
impl Default for DoorToolsSamJob { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/doortools.ts:1712 dealStack ->, src/game/doortools.ts:1712)
#[derive(Clone, Debug)]
pub struct DoorToolsDealStack {
    pub posts: Vec<crate::orders::StackPost>,
    pub back: JsMap<GuardId, crate::orders::StackPost>,
}
impl DoorToolsDealStack {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorToolsDealStack {
            posts: Vec::new(),
            back: JsMap::new(),
        }
    }
}
impl Default for DoorToolsDealStack { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: a union)
#[derive(Clone, Debug)]
pub enum DoorToolsWaiting4Item {
    DoorOp(DoorOpId),
    Arm1(crate::doortools::DoorToolsWaiting4ItemArm1),
}

/// (TypeScript: inline at src/game/doortools.ts:1774 waiting4 ->, src/game/doortools.ts:1774)
#[derive(Clone, Debug)]
pub struct DoorToolsWaiting4ItemArm1 {
    pub stack: DoorId,
    pub men: Vec<GuardId>,
}
impl DoorToolsWaiting4ItemArm1 {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorToolsWaiting4ItemArm1 {
            stack: DoorId(0),
            men: Vec::new(),
        }
    }
}
impl Default for DoorToolsWaiting4ItemArm1 { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/doortools.ts:1781 stacksOf ->, src/game/doortools.ts:1781)
#[derive(Clone, Debug)]
pub struct DoorToolsStacksOfItem {
    pub stack: DoorId,
    pub men: Vec<GuardId>,
}
impl DoorToolsStacksOfItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorToolsStacksOfItem {
            stack: DoorId(0),
            men: Vec::new(),
        }
    }
}
impl Default for DoorToolsStacksOfItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/doortools.ts:1791 stackWaits ->, src/game/doortools.ts:1791)
#[derive(Clone, Debug)]
pub struct DoorToolsStackWaitsItem {
    pub stack: DoorId,
    pub men: Vec<GuardId>,
}
impl DoorToolsStackWaitsItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorToolsStackWaitsItem {
            stack: DoorId(0),
            men: Vec::new(),
        }
    }
}
impl Default for DoorToolsStackWaitsItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: a union)
#[derive(Clone, Debug)]
pub enum DoorToolsWaitingAnyItem {
    DoorOp(DoorOpId),
    Arm1(crate::doortools::DoorToolsWaitingAnyItemArm1),
}

/// (TypeScript: inline at src/game/doortools.ts:1801 waitingAny ->, src/game/doortools.ts:1801)
#[derive(Clone, Debug)]
pub struct DoorToolsWaitingAnyItemArm1 {
    pub stack: DoorId,
    pub men: Vec<GuardId>,
}
impl DoorToolsWaitingAnyItemArm1 {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorToolsWaitingAnyItemArm1 {
            stack: DoorId(0),
            men: Vec::new(),
        }
    }
}
impl Default for DoorToolsWaitingAnyItemArm1 { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/doortools.ts:1804 waitsOn ->, src/game/doortools.ts:1804)
#[derive(Clone, Debug)]
pub struct DoorToolsWaitsOn {
    pub kind: crate::doortools::DoorToolsWaitsOnKind,
    pub men: Vec<GuardId>,
    pub set: bool,
}
impl DoorToolsWaitsOn {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorToolsWaitsOn {
            kind: crate::doortools::DoorToolsWaitsOnKind::Stack,
            men: Vec::new(),
            set: false,
        }
    }
}
impl Default for DoorToolsWaitsOn { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum DoorToolsWaitsOnKind {
    Stack,
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

/// (TypeScript: inline at src/game/doortools.ts:1876 goMen(only), src/game/doortools.ts:1876)
#[derive(Clone, Debug)]
pub struct DoorToolsGoMenOnlyItem {
    pub stack: DoorId,
    pub men: Vec<GuardId>,
}
impl DoorToolsGoMenOnlyItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorToolsGoMenOnlyItem {
            stack: DoorId(0),
            men: Vec::new(),
        }
    }
}
impl Default for DoorToolsGoMenOnlyItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/doortools.ts:2613 bangSpot ->, src/game/doortools.ts:2613)
#[derive(Clone, Copy, Debug)]
pub struct DoorToolsBangSpot {
    pub at: V3,
    pub n: f64,
    pub of: f64,
}
impl DoorToolsBangSpot {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorToolsBangSpot {
            at: [0.0; 3],
            n: 0.0,
            of: 0.0,
        }
    }
}
impl Default for DoorToolsBangSpot { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: a union)
#[derive(Clone, Copy, Debug)]
pub enum DoorToolsAway {
    Guard(GuardId),
    Alone,
    No,
}

/// (TypeScript: inline at src/game/doortools.ts:3212 offSwing(P), src/game/doortools.ts:3212)
#[derive(Clone, Copy, Debug)]
pub struct DoorToolsOffSwingP {
    pub pos: V3,
    pub watch: Option<V3>,
}
impl DoorToolsOffSwingP {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorToolsOffSwingP {
            pos: [0.0; 3],
            watch: None,
        }
    }
}
impl Default for DoorToolsOffSwingP { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/doortools.ts:3312 blow ->, src/game/doortools.ts:3312)
#[derive(Clone, Copy, Debug)]
pub struct DoorToolsBlow {
    pub knocked: f64,
    pub stunned: f64,
    pub backblast: f64,
}
impl DoorToolsBlow {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorToolsBlow {
            knocked: 0.0,
            stunned: 0.0,
            backblast: 0.0,
        }
    }
}
impl Default for DoorToolsBlow { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/doortools.ts:3313 blowRaw ->, src/game/doortools.ts:3313)
#[derive(Clone, Copy, Debug)]
pub struct DoorToolsBlowRaw {
    pub knocked: f64,
    pub stunned: f64,
    pub backblast: f64,
}
impl DoorToolsBlowRaw {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorToolsBlowRaw {
            knocked: 0.0,
            stunned: 0.0,
            backblast: 0.0,
        }
    }
}
impl Default for DoorToolsBlowRaw { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/doortools.ts:3378 pieNext(s), src/game/doortools.ts:3378)
#[derive(Clone, Debug)]
pub struct DoorToolsPieNextS {
    pub pts: Vec<V3>,
    pub i: f64,
    pub pie: Option<crate::doortools::DoorToolsPieNextSPie>,
}
impl DoorToolsPieNextS {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorToolsPieNextS {
            pts: Vec::new(),
            i: 0.0,
            pie: None,
        }
    }
}
impl Default for DoorToolsPieNextS { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/doortools.ts:3378 DoorToolsPieNextS.pie, src/game/doortools.ts:3378)
#[derive(Clone, Debug)]
pub struct DoorToolsPieNextSPie {
    pub eyes: Vec<V3>,
    pub knee: Vec<bool>,
}
impl DoorToolsPieNextSPie {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorToolsPieNextSPie {
            eyes: Vec::new(),
            knee: Vec::new(),
        }
    }
}
impl Default for DoorToolsPieNextSPie { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/doortools.ts:3464 thrSpot ->, src/game/doortools.ts:3464)
#[derive(Clone, Copy, Debug)]
pub struct DoorToolsThrSpot {
    pub q: V3,
    pub from: V3,
}
impl DoorToolsThrSpot {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorToolsThrSpot {
            q: [0.0; 3],
            from: [0.0; 3],
        }
    }
}
impl Default for DoorToolsThrSpot { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/doortools.ts:3471 thrFar ->, src/game/doortools.ts:3471)
#[derive(Clone, Copy, Debug)]
pub struct DoorToolsThrFar {
    pub far: GuardId,
    pub fs: f64,
    pub here: V3,
}
impl DoorToolsThrFar {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorToolsThrFar {
            far: GuardId(0),
            fs: 0.0,
            here: [0.0; 3],
        }
    }
}
impl Default for DoorToolsThrFar { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/doortools.ts:3742 pocketOf ->, src/game/doortools.ts:3742)
#[derive(Clone, Copy, Debug)]
pub struct DoorToolsPocketOf {
    pub at: Option<V3>,
    pub n: f64,
}
impl DoorToolsPocketOf {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorToolsPocketOf {
            at: None,
            n: 0.0,
        }
    }
}
impl Default for DoorToolsPocketOf { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/doortools.ts:3827 cornerNotes ->, src/game/doortools.ts:3827)
#[derive(Clone, Debug)]
pub struct DoorToolsCornerNotesItem {
    pub key: String,
    pub text: String,
}
impl DoorToolsCornerNotesItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorToolsCornerNotesItem {
            key: String::new(),
            text: String::new(),
        }
    }
}
impl Default for DoorToolsCornerNotesItem { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct DoortoolsMod {
    /// (src/game/doortools.ts:760 SKY)
    pub sky: JsMap<RoomfightRoomId, crate::doortools::DoortoolsModSKYValue>,
}
impl DoortoolsMod {
    pub fn blank() -> Self {
        DoortoolsMod {
            sky: JsMap::new(),
        }
    }
}

/// (src/game/doortools.ts:113 WORDCUTS_BREAK)
pub const WORDCUTS_BREAK: bool = false;

/// (src/game/doortools.ts:600 LEAF_EDGE)
pub const LEAF_EDGE: f64 = 0.1;

/// (src/game/doortools.ts:600 LEAF_FROM)
pub const LEAF_FROM: f64 = 1.45;

/// (src/game/doortools.ts:600 LEAF_STEP)
pub const LEAF_STEP: f64 = 0.1;

/// (src/game/doortools.ts:600 LEAF_COST)
pub const LEAF_COST: f64 = 4.0;

/// (src/game/doortools.ts:600 LEAF_POSTS: a table nothing writes)
pub fn leaf_posts() -> Vec<Vec<f64>> { alloc::vec![alloc::vec![0.2, 2.4], alloc::vec![0.5, 2.4]] }

/// (src/game/doortools.ts:602 SAM_SHIFT)
pub const SAM_SHIFT: f64 = 0.35;

/// (src/game/doortools.ts:606 TOOLS_PAGE)
pub const TOOLS_PAGE: &str = "tools";

/// (src/game/doortools.ts:608 BIG_ORDER)
pub const BIG_ORDER: &str = "bigClear";

/// (src/game/doortools.ts:608 BIG_KEY)
pub const BIG_KEY: &str = "KeyH";

/// (src/game/doortools.ts:611 THR_PAGE)
pub const THR_PAGE: &str = "thr";

/// (src/game/doortools.ts:611 HELD_PAGE)
pub const HELD_PAGE: &str = "held";

/// (src/game/doortools.ts:613 PIE_WAIT)
pub const PIE_WAIT: f64 = 2.5;

/// (src/game/doortools.ts:613 KNEE_EYE)
pub const KNEE_EYE: f64 = 0.95;

/// (src/game/doortools.ts:613 PIE_GUN: a table nothing writes)
pub fn pie_gun() -> crate::doortools::PIEGUN { crate::doortools::PIEGUN { after: 0.2, step: 0.12, most: 3.0, every: 0.4 } }

/// (src/game/doortools.ts:621 PRE: a table nothing writes)
pub fn pre() -> Vec<String> { alloc::vec![String::from("stack"), String::from("ready"), String::from("place"), String::from("standoff"), String::from("armed")] }

/// (src/game/doortools.ts:756 OPEN_RAD)
pub const OPEN_RAD: f64 = 0.6;

/// (src/game/doortools.ts:758 N4: a table nothing writes)
pub fn n4() -> Vec<[f64; 2]> { alloc::vec![[1.0, 0.0], [-1.0, 0.0], [0.0, 1.0], [0.0, -1.0]] }

/// (src/game/doortools.ts:3854 WORD_LEAD)
pub const WORD_LEAD: f64 = 0.15;
