//! The types of `orders`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// OrderDef.label: the row's words, written out or read off the target (returns the String). The arm's body lives in the module that wrote the closure; the match sits where orders.ts calls the field. Rows off the page's path have no arm.
#[derive(Clone, Debug)]
pub enum OrderLabel {
    Text { text: String },
    Cover,
    DoorOpenClear,
    DoorBangClear,
    DoorCharge,
    DoorFragClear,
    DoorThresholdHold,
    DoorGoSmall,
    DoorGoBig,
    DoorBigClear,
    WallRow,
    ThrowRow { kind: crate::squadthrow::SquadThrowKind },
    ThrowOpen,
    Shell40,
    Work,
    Smoke,
    PairGo,
}
/// OrderDef.can: this man can do his part (true in the TypeScript), or the word for why not. The arm's body lives in the module that wrote the closure; the match sits where orders.ts calls the field. Rows off the page's path have no arm.
#[derive(Clone, Debug)]
pub enum OrderCan {
    Move,
    Cover,
    WallRow,
    Work,
    PairTake,
    PairWatch,
    PairDark,
}
/// OrderDef.only: the men among those addressed this order is for. The arm's body lives in the module that wrote the closure; the match sits where orders.ts calls the field. Rows off the page's path have no arm.
#[derive(Clone, Debug)]
pub enum OrderOnly {
    DoorHeldOrAll,
    DoorGoSmall,
    DoorGoBig,
    SilentAll,
    ShutDoor,
    ThrowRow { kind: crate::squadthrow::SquadThrowKind },
    Wand,
    Shell40,
    Work,
    Smoke,
    PairTake,
    PairGo,
}
/// OrderDef.valid: these men together can carry it out here (true), or not (false, or the word for why). The arm's body lives in the module that wrote the closure; the match sits where orders.ts calls the field. Rows off the page's path have no arm.
#[derive(Clone, Debug)]
pub enum OrderValid {
    Move,
    Stack,
    DoorOpenClear,
    DoorBangClear,
    DoorCharge,
    DoorFragClear,
    DoorThresholdHold,
    DoorGoSmall,
    DoorGoBig,
    DoorBigClear,
    WallRow,
    ShutDoor,
    ThrowRow { kind: crate::squadthrow::SquadThrowKind },
    Wand,
    Shell40,
    Work,
    Smoke,
    PairTake,
    PairWatch,
    PairGo,
}
/// OrderDef.issue: hand each man his assignment; a String back is a refusal. Nothing: a row that only opens a page (give() refuses it at orders.ts:862, so it is never run). The arm's body lives in the module that wrote the closure; the match sits where orders.ts calls the field. Rows off the page's path have no arm.
#[derive(Clone, Debug)]
pub enum OrderIssue {
    Nothing,
    Move,
    Stack,
    Cover,
    FallIn,
    Hold,
    DoorOpenClear,
    DoorBangClear,
    DoorCharge,
    DoorFragClear,
    DoorThresholdHold,
    DoorGoSmall,
    DoorGoBig,
    DoorBigClear,
    WallRow,
    Silent,
    Loud,
    ShutDoor,
    ThrowRow { kind: crate::squadthrow::SquadThrowKind },
    Wand,
    Shell40,
    Work,
    Smoke,
    PairTake { mode: crate::pairorders::JobMode },
    PairWatch,
    PairDark,
    PairGo,
}
/// OrderDef.tick: once a frame for each man under the order, before he is posed. The arm's body lives in the module that wrote the closure; the match sits where orders.ts calls the field. Rows off the page's path have no arm.
#[derive(Clone, Debug)]
pub enum OrderTick {
    Move,
    ShutDoor,
    ThrowRow,
    Wand,
    Shell40,
    Work,
    PairTake,
    PairWatch,
    PairDark,
}
/// OrderDef.standsIn.when: the row stands in for the other one now. The arm's body lives in the module that wrote the closure; the match sits where orders.ts calls the field. Rows off the page's path have no arm.
#[derive(Clone, Debug)]
pub enum OrderWhen {
    ShutDoor,
    Loud,
    PairDark,
}
/// OrderDef.gone: the flat ring does not offer the row on this target at all. The arm's body lives in the module that wrote the closure; the match sits where orders.ts calls the field. Rows off the page's path have no arm.
#[derive(Clone, Debug)]
pub enum OrderGone {
    DoorCharge,
    DoorBigClear,
    WallRow,
}
/// SquadOrders.notes: who has a line for the squad's HUD block, in the order they were pushed. Read only by hud() (triage: dress).
#[derive(Clone, Debug)]
pub enum OrderNoteFn {
    DoorToolsHud,
    DoorToolsWord,
    WallCharge,
    Pair,
    Squad40,
    SquadThrow,
    SquadSmoke,
}
/// what marksKeep took, to put back once: the makers' counters, the room rule's book, the leaf's answer (orders.ts marksKeep). Not kept past the one trial.
#[derive(Clone, Debug)]
pub enum MarksKeep {
    Kept { leaf: bool, stack_own: crate::doorway::DoorwayModSTACKOWN, doorway: crate::doorway::DoorwayStats, stackroom: crate::stackroom::StackroomStats, glasspen: crate::glasspen::GlasspenStats, book: crate::stackroom::StackRoomBookKeep },
}
/// what one deal of a stack costs in men squeezing past one another (orders.ts fileCross): `file` = each spot's file, `tight` = by a spot's file, is it tight, `l` = man by spot, his walk (0 off a tight file), `cost` = ORDERS.fileQueueCost. Needs no g to price a deal.
#[derive(Clone, Debug)]
pub enum FileCross {
    Tight { file: Vec<usize>, tight: Vec<bool>, l: Vec<Vec<f64>>, spots: Vec<V3>, cost: f64 },
}
/// a teammate's way, nav.findPath's penalty: a cell of `dear` costs `cost`, any other 1, times what `pen` asks (orders.ts propPenalty). `dear` is by nav.idx; the nav is read from g when priced.
#[derive(Clone, Debug)]
pub enum PropPenalty {
    Dear { dear: JsSet<crate::Num>, cost: f64, pen: Option<crate::lanes::LanePenalty> },
}

/// (TypeScript: StackPost, src/game/orders.ts:366)
#[derive(Clone, Copy, Debug)]
pub struct StackPost {
    pub pos: V3,
    pub face: f64,
    pub stack_side: f64,
    pub lead: bool,
    pub group: Option<f64>,
}
impl StackPost {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StackPost {
            pos: [0.0; 3],
            face: 0.0,
            stack_side: 0.0,
            lead: false,
            group: None,
        }
    }
}
impl Default for StackPost { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: SquadOrders, src/game/orders.ts:531)
#[derive(Clone, Debug)]
pub struct SquadOrders {
    pub knobs: crate::knobs::OrdersK,
    pub defs: Vec<OrdersOrderDefId>,
    pub owners: Vec<crate::orders::SquadOwner>,
    pub notes: Vec<crate::orders::OrderNoteFn>,
    pub posture: Option<crate::orders::SquadOrdersPosture>,
    pub people: Option<crate::orders::SquadOrdersPeople>,
    pub who: crate::orders::Who,
    pub menu: Option<crate::orders::Menu>,
    pub cmd_ring: Option<crate::cmdring::RingState>,
    pub cmd_press: Option<crate::orders::SquadOrdersCmdPress>,
    pub tap_was: Option<crate::orders::SquadOrdersTapWas>,
    pub log: Vec<crate::orders::SquadOrdersLogItem>,
    pub lay: crate::planlay::LayState,
    pub by_hand: bool,
    pub shows: Vec<crate::orders::SquadOrdersShowsItem>,
    pub asg: JsMap<GuardId, OrdersAssignmentId>,
    pub trail: Vec<V3>,
    pub back: V3,
    pub view: crate::orders::SquadOrdersView,
    pub esc_used: bool,
    pub ours: JsSet<GuardId>,
    pub acks: f64,
    pub group: f64,
    pub roster: Vec<GuardId>,
    pub sam_p: V3,
    pub sam_t: f64,
    pub sam_v: f64,
    pub sam_dir: V3,
    pub door_marks: Vec<crate::orders::SquadOrdersDoorMarksItem>,
    pub cast: Option<crate::orders::OtherCast>,
    pub aside_mem: JsMap<GuardId, crate::orders::SquadOrdersAsideMemValue>,
    pub back_fix: JsMap<OrdersAssignmentId, crate::orders::SquadOrdersBackFixValue>,
    pub rest_of: JsMap<OrdersAssignmentId, crate::orders::SquadOrdersRestOfValue>,
    pub held_of: JsMap<OrdersAssignmentId, crate::orders::SquadOrdersHeldOfValue>,
    pub short_of: JsMap<OrdersAssignmentId, crate::orders::SquadOrdersShortOfValue>,
    pub clear_t: f64,
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
    pub face_flip: bool,
    pub reached: JsMap<String, crate::orders::SquadOrdersReachedValue>,
}
impl SquadOrders {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        SquadOrders {
            knobs: crate::knobs::OrdersK { key: String::from("KeyX"), mouse_opens: true, click_gives: false, who_key: String::from("KeyT"), quick_key: String::from("KeyB"), wheel_element: false, wait_key: String::from("Digit5"), wait_alt: String::from("KeyF"), refresh_secs: 1.0, door_pick_m: 1.0, self_m: 1.0, man_m: 0.8, door_ray: true, door_ray_pad: 0.08, door_ray_reach_m: 3.0, self_body: false, floor_snap_m: 1.0, jamb_out: 0.55, jamb_past: 0.45, file_gap: 0.9, seam_m: 0.03, second_floor: true, work_out: true, work_seen_m: 4.0, hall_file: true, follow_back: alloc::vec![1.3, 2.4], crumb: 0.4, trail_m: 10.0, run: 2.2, walk: 1.5, gain: 1.5, near_m: 0.6, band_m: 0.6, word_far_m: 2.0, word_hold_s: 2.5, file_hold: false, file_spread: false, file_apart: 0.9, file_tight: 0.75, off_door: 0.8, back_end: false, in_way: false, retake_goal: false, walk_tries: false, aside_m: 2.6, aside_lane: 0.8, aside_off: 0.95, aside_clear: false, file_queue: false, file_queue_gap: 1.3, file_queue_cost: 500.0, prop_round: false, prop_round_pad: 0.25, prop_round_cost: 6.0, prop_round_more: 2.0, prop_round_door: 2.5, aside_hold: 1.5, brisk: 2.0, brisk_in: 1.8, onto: 1.3, entry: 2.2, move_spread: 0.9, cold_secs: 2.5, pin_posted: true, lane_in: 0.45, lane_out: 0.7, hold_stride: 0.15, stuck_secs: 4.0, hold_group_m: 4.0, hold_door_m: 2.0, cleared: true, cleared_dear: 20.0, known_face: true, own_door: true, round_cap: 0.0, round_own: true, same_way: false, same_near_m: 5.0, same_apart_m: 6.0, same_long: 1.5, trail_doors: false, trail_reach_m: 3.0, trail_long: 1.5 },
            defs: Vec::new(),
            owners: Vec::new(),
            notes: Vec::new(),
            posture: None,
            people: None,
            who: crate::orders::Who::All,
            menu: None,
            cmd_ring: None,
            cmd_press: None,
            tap_was: None,
            log: Vec::new(),
            lay: crate::planlay::LayState { laid: Vec::new(), kept: None, kept_for: String::from(""), try_key: String::from(""), started: false, sending: false, restored_t: -1.0 },
            by_hand: false,
            shows: Vec::new(),
            asg: JsMap::new(),
            trail: Vec::new(),
            back: [0.0, 0.0, -1.0],
            view: crate::orders::SquadOrdersView { input: None, cam: None, canvas: None },
            esc_used: false,
            ours: JsSet::new(),
            acks: 0.0,
            group: 0.0,
            roster: Vec::new(),
            sam_p: [0.0, 0.0, 0.0],
            sam_t: -1.0,
            sam_v: 0.0,
            sam_dir: [0.0, 0.0, 1.0],
            door_marks: Vec::new(),
            cast: None,
            aside_mem: JsMap::new(),
            back_fix: JsMap::new(),
            rest_of: JsMap::new(),
            held_of: JsMap::new(),
            short_of: JsMap::new(),
            clear_t: -1.0,
            clear_by: Vec::new(),
            apart_of: JsMap::new(),
            file_hand: 0.0,
            file_gave: false,
            sam_out: false,
            leaf_out_gave: false,
            glass_at: JsMap::new(),
            glass_sig: 0.0,
            fur_in: false,
            worked: JsMap::new(),
            face_flip: false,
            reached: JsMap::new(),
        }
    }
}
impl Default for SquadOrders { fn default() -> Self { Self::blank() } }

/// (TypeScript: SquadOwner, src/game/orders.ts:486)
#[derive(Clone, Debug)]
pub struct SquadOwner {
    pub name: String,
}
impl SquadOwner {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SquadOwner {
            name: String::new(),
        }
    }
}
impl Default for SquadOwner { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/orders.ts:540 SquadOrders.posture, src/game/orders.ts:540)
#[derive(Clone, Copy, Debug)]
pub struct SquadOrdersPosture {
}
impl SquadOrdersPosture {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SquadOrdersPosture {
        }
    }
}
impl Default for SquadOrdersPosture { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/orders.ts:543 SquadOrders.people, src/game/orders.ts:543)
#[derive(Clone, Copy, Debug)]
pub struct SquadOrdersPeople {
}
impl SquadOrdersPeople {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SquadOrdersPeople {
        }
    }
}
impl Default for SquadOrdersPeople { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: a union)
#[derive(Clone, Copy, Debug)]
pub enum Who {
    Num(f64),
    All,
    Red,
    Blue,
}

/// (TypeScript: Menu, src/game/orders.ts:506)
#[derive(Clone, Debug)]
pub struct Menu {
    pub target: crate::orders::OrderTarget,
    pub men: Vec<GuardId>,
    pub rows: Vec<OrdersOrderDefId>,
    pub why: Vec<Option<String>>,
    pub sel: f64,
    pub wheel_acc: f64,
    pub read_t: f64,
    pub part: Option<Vec<GuardId>>,
    pub split: Option<bool>,
    pub page: Option<String>,
    pub word_row: bool,
    pub on_word: bool,
    pub men_side: Option<f64>,
    pub side_key: String,
    pub flat: Option<bool>,
}
impl Menu {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Menu {
            target: crate::orders::OrderTarget::Floor(crate::orders::OrderTargetFloor::zeroed()),
            men: Vec::new(),
            rows: Vec::new(),
            why: Vec::new(),
            sel: 0.0,
            wheel_acc: 0.0,
            read_t: 0.0,
            part: None,
            split: None,
            page: None,
            word_row: false,
            on_word: false,
            men_side: None,
            side_key: String::new(),
            flat: None,
        }
    }
}
impl Default for Menu { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: a union)
#[derive(Clone, Debug)]
pub enum OrderTarget {
    Floor(crate::orders::OrderTargetFloor),
    Door(crate::orders::OrderTargetDoor),
    None(crate::orders::OrderTargetNone),
    Foe(crate::orders::OrderTargetFoe),
}

/// (TypeScript: inline at src/game/orders.ts:506 Menu.target, src/game/orders.ts:352)
#[derive(Clone, Debug)]
pub struct OrderTargetFloor {
    pub at: V3,
    pub wall: Option<crate::wallcharge::WallSpot>,
    pub thing: Option<crate::workthings::WorkThing>,
    /// INVENTED by a review: see the proposals
    pub serial: u32,
}
impl OrderTargetFloor {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        OrderTargetFloor {
            at: [0.0; 3],
            wall: None,
            thing: None,
            serial: 0,
        }
    }
}
impl Default for OrderTargetFloor { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/orders.ts:506 Menu.target, src/game/orders.ts:353)
#[derive(Clone, Copy, Debug)]
pub struct OrderTargetDoor {
    pub at: V3,
    pub door: DoorId,
    pub side: f64,
}
impl OrderTargetDoor {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        OrderTargetDoor {
            at: [0.0; 3],
            door: DoorId(0),
            side: 0.0,
        }
    }
}
impl Default for OrderTargetDoor { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/orders.ts:506 Menu.target, src/game/orders.ts:354)
#[derive(Clone, Copy, Debug)]
pub struct OrderTargetNone {
    pub at: V3,
    pub man: Option<GuardId>,
}
impl OrderTargetNone {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        OrderTargetNone {
            at: [0.0; 3],
            man: None,
        }
    }
}
impl Default for OrderTargetNone { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/orders.ts:506 Menu.target, src/game/orders.ts:356)
#[derive(Clone, Copy, Debug)]
pub struct OrderTargetFoe {
    pub at: V3,
    pub foe: GuardId,
}
impl OrderTargetFoe {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        OrderTargetFoe {
            at: [0.0; 3],
            foe: GuardId(0),
        }
    }
}
impl Default for OrderTargetFoe { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/orders.ts:549 SquadOrders.cmdPress, src/game/orders.ts:549)
#[derive(Clone, Debug)]
pub struct SquadOrdersCmdPress {
    pub t0: f64,
    pub t: crate::orders::OrderTarget,
    pub add: Option<bool>,
}
impl SquadOrdersCmdPress {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SquadOrdersCmdPress {
            t0: 0.0,
            t: crate::orders::OrderTarget::Floor(crate::orders::OrderTargetFloor::zeroed()),
            add: None,
        }
    }
}
impl Default for SquadOrdersCmdPress { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/orders.ts:551 SquadOrders.tapWas, src/game/orders.ts:551)
#[derive(Clone, Debug)]
pub struct SquadOrdersTapWas {
    pub at: f64,
    pub t: crate::orders::OrderTarget,
    pub x: f64,
    pub y: f64,
}
impl SquadOrdersTapWas {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SquadOrdersTapWas {
            at: 0.0,
            t: crate::orders::OrderTarget::Floor(crate::orders::OrderTargetFloor::zeroed()),
            x: 0.0,
            y: 0.0,
        }
    }
}
impl Default for SquadOrdersTapWas { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/orders.ts:553 SquadOrders.log, src/game/orders.ts:553)
#[derive(Clone, Debug)]
pub struct SquadOrdersLogItem {
    pub t: f64,
    pub order: String,
    pub who: Vec<String>,
    pub target: String,
    pub auto: Option<bool>,
    pub laid: Option<bool>,
    /// INVENTED: g.serial() at the push (orders.ts:881 cuts the front): rescue.ts:570 and lesson.ts:113 hold a row as a cursor by identity
    pub serial: u32,
}
impl SquadOrdersLogItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SquadOrdersLogItem {
            t: 0.0,
            order: String::new(),
            who: Vec::new(),
            target: String::new(),
            auto: None,
            laid: None,
            serial: 0,
        }
    }
}
impl Default for SquadOrdersLogItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/orders.ts:560 SquadOrders.shows, src/game/orders.ts:560)
#[derive(Clone, Debug)]
pub struct SquadOrdersShowsItem {
    pub t: f64,
    pub who: Vec<String>,
}
impl SquadOrdersShowsItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SquadOrdersShowsItem {
            t: 0.0,
            who: Vec::new(),
        }
    }
}
impl Default for SquadOrdersShowsItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/orders.ts:566 SquadOrders.view, src/game/orders.ts:566)
#[derive(Clone, Copy, Debug)]
pub struct SquadOrdersView {
    pub input: Option<InputId>,
    pub cam: Option<CameraId>,
    pub canvas: Option<crate::host_hud::View>,
}
impl SquadOrdersView {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SquadOrdersView {
            input: None,
            cam: None,
            canvas: None,
        }
    }
}
impl Default for SquadOrdersView { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/orders.ts:577 SquadOrders.doorMarks, src/game/orders.ts:577)
#[derive(Clone, Debug)]
pub struct SquadOrdersDoorMarksItem {
    pub door: DoorId,
    pub side: f64,
    pub stack: Vec<V3>,
    pub standoff: Vec<V3>,
}
impl SquadOrdersDoorMarksItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SquadOrdersDoorMarksItem {
            door: DoorId(0),
            side: 0.0,
            stack: Vec::new(),
            standoff: Vec::new(),
        }
    }
}
impl Default for SquadOrdersDoorMarksItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: OtherCast, src/game/orders.ts:484)
#[derive(Clone, Debug)]
pub struct OtherCast {
    pub team: crate::teams::Team,
    pub ring: JsSet<String>,
}
impl OtherCast {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        OtherCast {
            team: crate::teams::Team::Guard,
            ring: JsSet::new(),
        }
    }
}
impl Default for OtherCast { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/orders.ts:1554 SquadOrders.asideMem, src/game/orders.ts:1554)
#[derive(Clone, Copy, Debug)]
pub struct SquadOrdersAsideMemValue {
    pub q: V3,
    pub t: f64,
    pub dir: V3,
    pub side: Option<f64>,
}
impl SquadOrdersAsideMemValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SquadOrdersAsideMemValue {
            q: [0.0; 3],
            t: 0.0,
            dir: [0.0; 3],
            side: None,
        }
    }
}
impl Default for SquadOrdersAsideMemValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/orders.ts:1570 SquadOrders.backFix, src/game/orders.ts:1570)
#[derive(Clone, Copy, Debug)]
pub struct SquadOrdersBackFixValue {
    pub plain: f64,
    pub path: f64,
}
impl SquadOrdersBackFixValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SquadOrdersBackFixValue {
            plain: 0.0,
            path: 0.0,
        }
    }
}
impl Default for SquadOrdersBackFixValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/orders.ts:1591 SquadOrders.restOf, src/game/orders.ts:1591)
#[derive(Clone, Copy, Debug)]
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
impl SquadOrdersRestOfValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SquadOrdersRestOfValue {
            x: 0.0,
            z: 0.0,
            t: 0.0,
            step: None,
            until: 0.0,
            from: [0.0; 3],
            n: 0.0,
            hold: None,
            hold_for: [0.0; 3],
        }
    }
}
impl Default for SquadOrdersRestOfValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/orders.ts:1616 SquadOrders.heldOf, src/game/orders.ts:1616)
#[derive(Clone, Copy, Debug)]
pub struct SquadOrdersHeldOfValue {
    pub last: f64,
    pub total: f64,
}
impl SquadOrdersHeldOfValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SquadOrdersHeldOfValue {
            last: 0.0,
            total: 0.0,
        }
    }
}
impl Default for SquadOrdersHeldOfValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/orders.ts:1627 SquadOrders.shortOf, src/game/orders.ts:1627)
#[derive(Clone, Copy, Debug)]
pub struct SquadOrdersShortOfValue {
    pub x: f64,
    pub z: f64,
    pub n: f64,
    pub k: f64,
}
impl SquadOrdersShortOfValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SquadOrdersShortOfValue {
            x: 0.0,
            z: 0.0,
            n: 0.0,
            k: 0.0,
        }
    }
}
impl Default for SquadOrdersShortOfValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/orders.ts:1668 SquadOrders.apartOf, src/game/orders.ts:1668)
#[derive(Clone, Copy, Debug)]
pub struct SquadOrdersApartOfValue {
    pub at: V3,
    pub q: Option<V3>,
    pub t: f64,
    pub m: f64,
    pub short: Option<bool>,
}
impl SquadOrdersApartOfValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SquadOrdersApartOfValue {
            at: [0.0; 3],
            q: None,
            t: 0.0,
            m: 0.0,
            short: None,
        }
    }
}
impl Default for SquadOrdersApartOfValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/orders.ts:2127 SquadOrders.worked, src/game/orders.ts:2127)
#[derive(Clone, Debug)]
pub struct SquadOrdersWorkedValueItem {
    pub jambless: bool,
    pub marks: Vec<crate::orders::SquadOrdersWorkedValueItemMarksItem>,
}
impl SquadOrdersWorkedValueItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SquadOrdersWorkedValueItem {
            jambless: false,
            marks: Vec::new(),
        }
    }
}
impl Default for SquadOrdersWorkedValueItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/orders.ts:2127 SquadOrdersWorkedValueItem.marks, src/game/orders.ts:2127)
#[derive(Clone, Copy, Debug)]
pub struct SquadOrdersWorkedValueItemMarksItem {
    pub pos: V3,
    pub walk: f64,
    pub seen: f64,
}
impl SquadOrdersWorkedValueItemMarksItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SquadOrdersWorkedValueItemMarksItem {
            pos: [0.0; 3],
            walk: 0.0,
            seen: 0.0,
        }
    }
}
impl Default for SquadOrdersWorkedValueItemMarksItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/orders.ts:2486 SquadOrders.reached, src/game/orders.ts:2486)
#[derive(Clone, Debug)]
pub struct SquadOrdersReachedValue {
    pub t: f64,
    pub why: Option<String>,
    pub lock: Option<crate::orders::SquadOrdersReachedValueLock>,
}
impl SquadOrdersReachedValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SquadOrdersReachedValue {
            t: 0.0,
            why: None,
            lock: None,
        }
    }
}
impl Default for SquadOrdersReachedValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/orders.ts:2486 SquadOrdersReachedValue.lock, src/game/orders.ts:2486)
#[derive(Clone, Copy, Debug)]
pub struct SquadOrdersReachedValueLock {
    pub door: DoorId,
    pub side: f64,
}
impl SquadOrdersReachedValueLock {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SquadOrdersReachedValueLock {
            door: DoorId(0),
            side: 0.0,
        }
    }
}
impl Default for SquadOrdersReachedValueLock { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: HoverKept, src/game/orders.ts:347)
#[derive(Clone, Debug)]
pub struct HoverKept {
    pub hov: Option<crate::orders::HoverKeptHov>,
    pub cam: Vec<f64>,
}
impl HoverKept {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        HoverKept {
            hov: None,
            cam: Vec::new(),
        }
    }
}
impl Default for HoverKept { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/orders.ts:347 HoverKept.hov, src/game/orders.ts:347)
#[derive(Clone, Debug)]
pub struct HoverKeptHov {
    pub x: f64,
    pub y: f64,
    pub t: f64,
    pub v: crate::orders::OrderTarget,
}
impl HoverKeptHov {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        HoverKeptHov {
            x: 0.0,
            y: 0.0,
            t: 0.0,
            v: crate::orders::OrderTarget::Floor(crate::orders::OrderTargetFloor::zeroed()),
        }
    }
}
impl Default for HoverKeptHov { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum TargetKind {
    None,
    Door,
    Floor,
    Foe,
}
impl TargetKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            TargetKind::None => "none",
            TargetKind::Door => "door",
            TargetKind::Floor => "floor",
            TargetKind::Foe => "foe",
        }
    }
}

/// (TypeScript: GiveOpts, src/game/orders.ts:363)
#[derive(Clone, Debug)]
pub struct GiveOpts {
    pub ack: Option<String>,
    pub on_word: Option<bool>,
    pub men: Option<Vec<GuardId>>,
}
impl GiveOpts {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GiveOpts {
            ack: None,
            on_word: None,
            men: None,
        }
    }
}
impl Default for GiveOpts { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: OrderDef, src/game/orders.ts:369)
#[derive(Clone, Debug)]
pub struct OrderDef {
    pub id: String,
    pub label: crate::orders::OrderLabel,
    pub targets: Vec<crate::orders::TargetKind>,
    pub key: Option<f64>,
    pub page: Option<String>,
    pub opens: Option<String>,
    pub listed: Option<Vec<crate::orders::TargetKind>>,
    pub hot_key: Option<String>,
    pub page_only: Option<bool>,
    pub can: Option<crate::orders::OrderCan>,
    pub only: Option<crate::orders::OrderOnly>,
    pub valid: Option<crate::orders::OrderValid>,
    pub issue: crate::orders::OrderIssue,
    pub tick: Option<crate::orders::OrderTick>,
    pub ack: Option<Vec<String>>,
    pub ack_now: Option<Vec<String>>,
    pub on_word: Option<bool>,
    pub free: Option<bool>,
    pub head: Option<String>,
    pub stands_in: Option<crate::orders::OrderDefStandsIn>,
    pub gone: Option<crate::orders::OrderGone>,
}
impl OrderDef {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        OrderDef {
            id: String::new(),
            label: crate::orders::OrderLabel::Text { text: String::new() },
            targets: Vec::new(),
            key: None,
            page: None,
            opens: None,
            listed: None,
            hot_key: None,
            page_only: None,
            can: None,
            only: None,
            valid: None,
            issue: crate::orders::OrderIssue::Nothing,
            tick: None,
            ack: None,
            ack_now: None,
            on_word: None,
            free: None,
            head: None,
            stands_in: None,
            gone: None,
        }
    }
}
impl Default for OrderDef { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: a union)
#[derive(Clone, Debug)]
pub enum OrderDefLabel {
    Text(String),
    Arm1(crate::FnField),
}

/// (TypeScript: inline at src/game/orders.ts:418 OrderDef.standsIn, src/game/orders.ts:418)
#[derive(Clone, Debug)]
pub struct OrderDefStandsIn {
    pub for_: String,
    pub when: crate::orders::OrderWhen,
}
impl OrderDefStandsIn {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        OrderDefStandsIn {
            for_: String::new(),
            when: crate::orders::OrderWhen::ShutDoor,
        }
    }
}
impl Default for OrderDefStandsIn { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Assignment, src/game/orders.ts:428)
#[derive(Clone, Debug)]
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
    /// INVENTED: true at assign and at the unguarded writes made here (orders.ts:1473, :1475, :1640, :2346, :1457, wallreact.ts:587, pairorders.ts:439, :486, rescue.ts:769), not for doorpath.ts:273, :316, which alias s.goal. orders.ts:1519 passes held: !(a.new_leg && the A.goal arm was chosen) and clears it then
    pub new_leg: bool,
}
impl Assignment {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Assignment {
            order: String::new(),
            t0: 0.0,
            post: None,
            follow: 0.0,
            face: None,
            watch: None,
            door: None,
            strafe: None,
            stance: crate::character::Stance::None,
            upper: crate::character::UpperMode::None,
            stack_side: 0.0,
            move_: None,
            going: String::new(),
            there: String::new(),
            set_line: None,
            group: 0.0,
            set_said: false,
            goal: [0.0; 3],
            arrived: false,
            held_set: None,
            held_fire: None,
            arr_t: 0.0,
            moved_t: 0.0,
            light: None,
            low: false,
            yield_t: 0.0,
            face_now: None,
            search: None,
            search_t: None,
            via: Vec::new(),
            anchor: [0.0; 3],
            mark: [0.0; 3],
            mark_t: 0.0,
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
            new_leg: false,
        }
    }
}
impl Default for Assignment { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Tone {
    Red,
    Blue,
    Gold,
}
impl Tone {
    pub const fn as_str(self) -> &'static str {
        match self {
            Tone::Red => "red",
            Tone::Blue => "blue",
            Tone::Gold => "gold",
        }
    }
}

/// (TypeScript: OrderHudRow, src/game/orders.ts:499)
#[derive(Clone, Debug)]
pub struct OrderHudRow {
    pub kind: crate::orders::OrderHudRowKind,
    pub label: String,
    pub opts: Vec<crate::orders::OrderHudRowOptsItem>,
    pub in_reach: bool,
    pub off: bool,
    pub cont: Option<bool>,
    pub tone: Option<crate::orders::Tone>,
}
impl OrderHudRow {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        OrderHudRow {
            kind: crate::orders::OrderHudRowKind::Cam,
            label: String::new(),
            opts: Vec::new(),
            in_reach: false,
            off: false,
            cont: None,
            tone: None,
        }
    }
}
impl Default for OrderHudRow { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum OrderHudRowKind {
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
impl OrderHudRowKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            OrderHudRowKind::Cam => "cam",
            OrderHudRowKind::Guard => "guard",
            OrderHudRowKind::Breaker => "breaker",
            OrderHudRowKind::Door => "door",
            OrderHudRowKind::Light => "light",
            OrderHudRowKind::Body => "body",
            OrderHudRowKind::Objective => "objective",
            OrderHudRowKind::Held => "held",
            OrderHudRowKind::Wall => "wall",
            OrderHudRowKind::Switch => "switch",
            OrderHudRowKind::Charge => "charge",
            OrderHudRowKind::Cancel => "cancel",
        }
    }
}

/// (TypeScript: inline at src/game/orders.ts:499 OrderHudRow.opts, src/game/orders.ts:499)
#[derive(Clone, Debug)]
pub struct OrderHudRowOptsItem {
    pub key: String,
    pub text: String,
    pub tone: Option<crate::orders::Tone>,
    pub gone: Option<bool>,
    pub lost: Option<crate::deathrecap::LostMan>,
}
impl OrderHudRowOptsItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        OrderHudRowOptsItem {
            key: String::new(),
            text: String::new(),
            tone: None,
            gone: None,
            lost: None,
        }
    }
}
impl Default for OrderHudRowOptsItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: OrderHudMenu, src/game/orders.ts:501)
#[derive(Clone, Debug)]
pub struct OrderHudMenu {
    pub rows: Vec<crate::orders::OrderHudRow>,
    pub primary: f64,
    pub more: f64,
    pub open: bool,
    pub hint: String,
    pub cancel: String,
}
impl OrderHudMenu {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        OrderHudMenu {
            rows: Vec::new(),
            primary: 0.0,
            more: 0.0,
            open: false,
            hint: String::new(),
            cancel: String::new(),
        }
    }
}
impl Default for OrderHudMenu { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: a union)
#[derive(Clone, Debug)]
pub enum OrderNote {
    Text(String),
    Arm1(crate::orders::OrderNoteArm1),
}

/// (TypeScript: inline at src/game/orders.ts:503 OrderNote, src/game/orders.ts:503)
#[derive(Clone, Debug)]
pub struct OrderNoteArm1 {
    pub key: String,
    pub text: String,
}
impl OrderNoteArm1 {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        OrderNoteArm1 {
            key: String::new(),
            text: String::new(),
        }
    }
}
impl Default for OrderNoteArm1 { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: FightLease, src/game/orders.ts:520)
#[derive(Clone, Copy, Debug)]
pub struct FightLease {
}
impl FightLease {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FightLease {
        }
    }
}
impl Default for FightLease { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/orders.ts:529 FOLLOW_WORD, src/game/orders.ts:529)
#[derive(Clone, Copy, Debug)]
pub struct OrdersModFOLLOWWORDValue {
    pub on: bool,
    pub out_t: f64,
}
impl OrdersModFOLLOWWORDValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        OrdersModFOLLOWWORDValue {
            on: false,
            out_t: 0.0,
        }
    }
}
impl Default for OrdersModFOLLOWWORDValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/orders.ts:2629 LOW, src/game/orders.ts:2629)
#[derive(Clone, Debug)]
pub struct LOW {
    pub stance: String,
    pub upper: String,
}
impl LOW {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LOW {
            stance: String::new(),
            upper: String::new(),
        }
    }
}
impl Default for LOW { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/orders.ts:2630 AIM, src/game/orders.ts:2630)
#[derive(Clone, Debug)]
pub struct AIM {
    pub stance: String,
    pub upper: String,
}
impl AIM {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AIM {
            stance: String::new(),
            upper: String::new(),
        }
    }
}
impl Default for AIM { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: a union)
#[derive(Clone, Debug)]
pub enum SquadOrdersRecipientsT {
    Floor(crate::orders::OrderTargetFloor),
    Door(crate::orders::OrderTargetDoor),
    None(crate::orders::OrderTargetNone),
    Foe(crate::orders::OrderTargetFoe),
}

/// (TypeScript: a union)
#[derive(Clone, Copy, Debug)]
pub enum SquadOrdersRecipientsWho {
    Num(f64),
    All,
    Red,
    Blue,
}

/// (TypeScript: a union)
#[derive(Clone, Copy, Debug)]
pub enum SquadOrdersMenSel {
    Num(f64),
    Guard(GuardId),
    All,
    Red,
    Blue,
}

/// (TypeScript: inline at src/game/orders.ts:667 men(opts), src/game/orders.ts:667)
#[derive(Clone, Copy, Debug)]
pub struct SquadOrdersMenOpts {
    pub down: Option<bool>,
}
impl SquadOrdersMenOpts {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SquadOrdersMenOpts {
            down: None,
        }
    }
}
impl Default for SquadOrdersMenOpts { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/orders.ts:803 sceneHit(first), src/game/orders.ts:803)
#[derive(Clone, Copy, Debug)]
pub struct SquadOrdersSceneHitFirst {
    pub t: f64,
    pub index: f64,
}
impl SquadOrdersSceneHitFirst {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SquadOrdersSceneHitFirst {
            t: 0.0,
            index: 0.0,
        }
    }
}
impl Default for SquadOrdersSceneHitFirst { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/orders.ts:803 sceneHit ->, src/game/orders.ts:803)
#[derive(Clone, Copy, Debug)]
pub struct SquadOrdersSceneHit {
    pub t: f64,
    pub index: f64,
}
impl SquadOrdersSceneHit {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SquadOrdersSceneHit {
            t: 0.0,
            index: 0.0,
        }
    }
}
impl Default for SquadOrdersSceneHit { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: a union)
#[derive(Clone, Copy, Debug)]
pub enum SquadOrdersGiveWho {
    Num(f64),
    All,
    Red,
    Blue,
}

/// (TypeScript: inline at src/game/orders.ts:903 assign(a), src/game/orders.ts:903 assign(a))
#[derive(Clone, Debug)]
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
impl SquadOrdersAssignA {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SquadOrdersAssignA {
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
impl Default for SquadOrdersAssignA { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: a union)
#[derive(Clone, Debug)]
pub enum SquadOrdersMenuTarget {
    Floor(crate::orders::OrderTargetFloor),
    Door(crate::orders::OrderTargetDoor),
    None(crate::orders::OrderTargetNone),
    Foe(crate::orders::OrderTargetFoe),
}

/// (TypeScript: a union)
#[derive(Clone, Debug)]
pub enum SquadOrdersHover {
    Floor(crate::orders::OrderTargetFloor),
    Door(crate::orders::OrderTargetDoor),
    None(crate::orders::OrderTargetNone),
    Foe(crate::orders::OrderTargetFoe),
}

/// (TypeScript: a union)
#[derive(Clone, Debug)]
pub enum SquadOrdersWhoAfterTPoint {
    Floor(crate::orders::OrderTargetFloor),
    Door(crate::orders::OrderTargetDoor),
    None(crate::orders::OrderTargetNone),
    Foe(crate::orders::OrderTargetFoe),
}

/// (TypeScript: a union)
#[derive(Clone, Debug)]
pub enum SquadOrdersStepWhoPoint {
    Floor(crate::orders::OrderTargetFloor),
    Door(crate::orders::OrderTargetDoor),
    None(crate::orders::OrderTargetNone),
    Foe(crate::orders::OrderTargetFoe),
}

/// (TypeScript: inline at src/game/orders.ts:1198 ringUp(G), src/game/orders.ts:1198)
#[derive(Clone, Debug)]
pub struct SquadOrdersRingUpG {
    pub men: Vec<GuardId>,
    pub split: bool,
}
impl SquadOrdersRingUpG {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SquadOrdersRingUpG {
            men: Vec::new(),
            split: false,
        }
    }
}
impl Default for SquadOrdersRingUpG { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/orders.ts:1211 goalPart ->, src/game/orders.ts:1211)
#[derive(Clone, Debug)]
pub struct SquadOrdersGoalPart {
    pub men: Vec<GuardId>,
    pub split: bool,
}
impl SquadOrdersGoalPart {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SquadOrdersGoalPart {
            men: Vec::new(),
            split: false,
        }
    }
}
impl Default for SquadOrdersGoalPart { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/orders.ts:1225 hintView ->, src/game/orders.ts:1225)
#[derive(Clone, Copy, Debug)]
pub struct SquadOrdersHintView {
    pub kind: crate::cmdring::RingHint,
    pub x: f64,
    pub y: f64,
    pub zoom: f64,
    pub held: bool,
    pub press: bool,
    pub tone: Option<crate::orders::Tone>,
    pub seg: Option<[V3; 2]>,
    pub ring: Option<crate::orders::SquadOrdersHintViewRing>,
    pub glyph: bool,
}
impl SquadOrdersHintView {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SquadOrdersHintView {
            kind: crate::cmdring::RingHint::Door,
            x: 0.0,
            y: 0.0,
            zoom: 0.0,
            held: false,
            press: false,
            tone: None,
            seg: None,
            ring: None,
            glyph: false,
        }
    }
}
impl Default for SquadOrdersHintView { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/orders.ts:1225 SquadOrdersHintView.ring, src/game/orders.ts:1225)
#[derive(Clone, Copy, Debug)]
pub struct SquadOrdersHintViewRing {
    pub at: V3,
    pub r: f64,
}
impl SquadOrdersHintViewRing {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SquadOrdersHintViewRing {
            at: [0.0; 3],
            r: 0.0,
        }
    }
}
impl Default for SquadOrdersHintViewRing { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/orders.ts:1251 seats ->, src/game/orders.ts:1251)
#[derive(Clone, Debug)]
pub struct SquadOrdersSeats {
    pub row: Vec<f64>,
    pub more: f64,
    pub over: Vec<f64>,
    pub same: Vec<f64>,
}
impl SquadOrdersSeats {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SquadOrdersSeats {
            row: Vec::new(),
            more: 0.0,
            over: Vec::new(),
            same: Vec::new(),
        }
    }
}
impl Default for SquadOrdersSeats { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum SquadOrdersQuickFor {
    First,
    Entry,
}
impl SquadOrdersQuickFor {
    pub const fn as_str(self) -> &'static str {
        match self {
            SquadOrdersQuickFor::First => "first",
            SquadOrdersQuickFor::Entry => "entry",
        }
    }
}

/// (TypeScript: inline at src/game/orders.ts:1844 ownMarkFirst(ask), src/game/orders.ts:1844)
#[derive(Clone, Copy, Debug)]
pub struct SquadOrdersOwnMarkFirstAskRetItem {
    pub pos: V3,
}
impl SquadOrdersOwnMarkFirstAskRetItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SquadOrdersOwnMarkFirstAskRetItem {
            pos: [0.0; 3],
        }
    }
}
impl Default for SquadOrdersOwnMarkFirstAskRetItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/orders.ts:2106 workedMarks ->, src/game/orders.ts:2106)
#[derive(Clone, Debug)]
pub struct SquadOrdersWorkedMarks {
    pub jambless: bool,
    pub marks: Vec<crate::orders::SquadOrdersWorkedMarksMarksItem>,
}
impl SquadOrdersWorkedMarks {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SquadOrdersWorkedMarks {
            jambless: false,
            marks: Vec::new(),
        }
    }
}
impl Default for SquadOrdersWorkedMarks { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/orders.ts:2106 SquadOrdersWorkedMarks.marks, src/game/orders.ts:2106)
#[derive(Clone, Copy, Debug)]
pub struct SquadOrdersWorkedMarksMarksItem {
    pub pos: V3,
    pub walk: f64,
    pub seen: f64,
}
impl SquadOrdersWorkedMarksMarksItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SquadOrdersWorkedMarksMarksItem {
            pos: [0.0; 3],
            walk: 0.0,
            seen: 0.0,
        }
    }
}
impl Default for SquadOrdersWorkedMarksMarksItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/orders.ts:2214 holdArcs ->, src/game/orders.ts:2214)
#[derive(Clone, Copy, Debug)]
pub struct SquadOrdersHoldArcsItem {
    pub yaw: f64,
    pub door: Option<DoorId>,
    pub reach: f64,
}
impl SquadOrdersHoldArcsItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SquadOrdersHoldArcsItem {
            yaw: 0.0,
            door: None,
            reach: 0.0,
        }
    }
}
impl Default for SquadOrdersHoldArcsItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/orders.ts:2250 route ->, src/game/orders.ts:2250)
#[derive(Clone, Debug)]
pub struct SquadOrdersRoute {
    pub why: Option<String>,
    pub via: Vec<V3>,
    pub how: Option<crate::orders::SquadOrdersRouteHow>,
    pub lock: Option<crate::orders::SquadOrdersRouteLock>,
}
impl SquadOrdersRoute {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SquadOrdersRoute {
            why: None,
            via: Vec::new(),
            how: None,
            lock: None,
        }
    }
}
impl Default for SquadOrdersRoute { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum SquadOrdersRouteHow {
    Round,
    Through,
    Mouth,
}
impl SquadOrdersRouteHow {
    pub const fn as_str(self) -> &'static str {
        match self {
            SquadOrdersRouteHow::Round => "round",
            SquadOrdersRouteHow::Through => "through",
            SquadOrdersRouteHow::Mouth => "mouth",
        }
    }
}

/// (TypeScript: inline at src/game/orders.ts:2250 SquadOrdersRoute.lock, src/game/orders.ts:2250)
#[derive(Clone, Copy, Debug)]
pub struct SquadOrdersRouteLock {
    pub door: DoorId,
    pub side: f64,
}
impl SquadOrdersRouteLock {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SquadOrdersRouteLock {
            door: DoorId(0),
            side: 0.0,
        }
    }
}
impl Default for SquadOrdersRouteLock { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/orders.ts:2437 walkTry ->, src/game/orders.ts:2437)
#[derive(Clone, Copy, Debug)]
pub struct SquadOrdersWalkTry {
    pub door: DoorId,
    pub side: f64,
}
impl SquadOrdersWalkTry {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SquadOrdersWalkTry {
            door: DoorId(0),
            side: 0.0,
        }
    }
}
impl Default for SquadOrdersWalkTry { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: a union)
#[derive(Clone, Debug)]
pub enum SquadOrdersWalkStacks {
    Text(String),
    Map(JsMap<GuardId, crate::orders::SquadOrdersWalkStacksMapValue>),
}

/// (TypeScript: inline at src/game/orders.ts:2444 walkStacks ->, src/game/orders.ts:2444)
#[derive(Clone, Copy, Debug)]
pub struct SquadOrdersWalkStacksMapValue {
    pub door: DoorId,
    pub side: f64,
    pub post: crate::orders::StackPost,
}
impl SquadOrdersWalkStacksMapValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SquadOrdersWalkStacksMapValue {
            door: DoorId(0),
            side: 0.0,
            post: crate::orders::StackPost::zeroed(),
        }
    }
}
impl Default for SquadOrdersWalkStacksMapValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/orders.ts:2454 walkSend(T), src/game/orders.ts:2454)
#[derive(Clone, Copy, Debug)]
pub struct SquadOrdersWalkSendT {
    pub door: DoorId,
    pub side: f64,
    pub post: crate::orders::StackPost,
}
impl SquadOrdersWalkSendT {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SquadOrdersWalkSendT {
            door: DoorId(0),
            side: 0.0,
            post: crate::orders::StackPost::zeroed(),
        }
    }
}
impl Default for SquadOrdersWalkSendT { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/orders.ts:2454 walkSend(on), src/game/orders.ts:2454 walkSend(on))
#[derive(Clone, Debug)]
pub struct SquadOrdersWalkSendOn {
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
impl SquadOrdersWalkSendOn {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SquadOrdersWalkSendOn {
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
impl Default for SquadOrdersWalkSendOn { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/orders.ts:2477 asked ->, src/game/orders.ts:2477)
#[derive(Clone, Debug)]
pub struct SquadOrdersAsked {
    pub why: Option<String>,
    pub lock: Option<crate::orders::SquadOrdersAskedLock>,
}
impl SquadOrdersAsked {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SquadOrdersAsked {
            why: None,
            lock: None,
        }
    }
}
impl Default for SquadOrdersAsked { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/orders.ts:2477 SquadOrdersAsked.lock, src/game/orders.ts:2477)
#[derive(Clone, Copy, Debug)]
pub struct SquadOrdersAskedLock {
    pub door: DoorId,
    pub side: f64,
}
impl SquadOrdersAskedLock {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SquadOrdersAskedLock {
            door: DoorId(0),
            side: 0.0,
        }
    }
}
impl Default for SquadOrdersAskedLock { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/orders.ts:2489 askAside(own), src/game/orders.ts:2489)
#[derive(Clone, Debug)]
pub struct SquadOrdersAskAsideOwn {
    pub r: Option<JsMap<String, crate::orders::SquadOrdersReachedValue>>,
}
impl SquadOrdersAskAsideOwn {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SquadOrdersAskAsideOwn {
            r: None,
        }
    }
}
impl Default for SquadOrdersAskAsideOwn { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at quiet SquadOrdersAskAsideOwn.r, quiet SquadOrdersAskAsideOwn.r)
#[derive(Clone, Copy, Debug)]
pub struct SquadOrdersAskAsideOwnR {
}
impl SquadOrdersAskAsideOwnR {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SquadOrdersAskAsideOwnR {
        }
    }
}
impl Default for SquadOrdersAskAsideOwnR { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/orders.ts:2562 whoTag ->, src/game/orders.ts:2562)
#[derive(Clone, Debug)]
pub struct SquadOrdersWhoTag {
    pub tag: String,
    pub one: bool,
}
impl SquadOrdersWhoTag {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SquadOrdersWhoTag {
            tag: String::new(),
            one: false,
        }
    }
}
impl Default for SquadOrdersWhoTag { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: a union)
#[derive(Clone, Debug)]
pub enum SquadOrdersHoverWordsT {
    Floor(crate::orders::OrderTargetFloor),
    Door(crate::orders::OrderTargetDoor),
    None(crate::orders::OrderTargetNone),
    Foe(crate::orders::OrderTargetFoe),
}

/// (TypeScript: inline at src/game/orders.ts:2583 hud ->, src/game/orders.ts:2583)
#[derive(Clone, Debug)]
pub struct SquadOrdersHud {
    pub menu: Option<crate::orders::OrderHudMenu>,
    pub status: Vec<crate::orders::OrderHudRow>,
}
impl SquadOrdersHud {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SquadOrdersHud {
            menu: None,
            status: Vec::new(),
        }
    }
}
impl Default for SquadOrdersHud { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/orders.ts:2619 mark ->, src/game/orders.ts:2619)
#[derive(Clone, Debug)]
pub struct SquadOrdersMark {
    pub at: V3,
    pub text: String,
    pub tone: Option<crate::orders::Tone>,
}
impl SquadOrdersMark {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SquadOrdersMark {
            at: [0.0; 3],
            text: String::new(),
            tone: None,
        }
    }
}
impl Default for SquadOrdersMark { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/orders.ts:2665 farOff ->, src/game/orders.ts:2665)
#[derive(Clone, Copy, Debug)]
pub struct FarOff {
    pub d: f64,
    pub at: V3,
}
impl FarOff {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FarOff {
            d: 0.0,
            at: [0.0; 3],
        }
    }
}
impl Default for FarOff { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct OrdersMod {
    /// (src/game/orders.ts:529 FOLLOW_WORD)
    pub follow_word: JsMap<OrdersAssignmentId, crate::orders::OrdersModFOLLOWWORDValue>,
}
impl OrdersMod {
    pub fn blank() -> Self {
        OrdersMod {
            follow_word: JsMap::new(),
        }
    }
}

/// (src/game/orders.ts:114 NOWHERE: a table nothing writes)
pub fn nowhere() -> V3 { [10000000.0, 0.0, 10000000.0] }

/// (src/game/orders.ts:116 OWN_EPS)
pub const OWN_EPS: f64 = 0.15;

/// (src/game/orders.ts:118 IN_WAY_WORDS)
pub const IN_WAY_WORDS: &str = "you're in the way.";

/// (src/game/orders.ts:121 FUR_M)
pub const FUR_M: f64 = 0.12;

/// (src/game/orders.ts:123 BACK_TRIES)
pub const BACK_TRIES: f64 = 2.0;

/// (src/game/orders.ts:334 WHEEL_STEP)
pub const WHEEL_STEP: f64 = 60.0;

/// (src/game/orders.ts:336 ORDER_DIGITS)
pub const ORDER_DIGITS: f64 = 5.0;

/// (src/game/orders.ts:339 COMPASS: a table nothing writes)
pub fn compass_table() -> Vec<String> { alloc::vec![String::from("south"), String::from("south-east"), String::from("east"), String::from("north-east"), String::from("north"), String::from("north-west"), String::from("west"), String::from("south-west")] }

/// (src/game/orders.ts:425 WORD_HEAD)
pub const WORD_HEAD: &str = "on your word";

/// (src/game/orders.ts:2629 LOW: a table nothing writes)
pub fn low() -> crate::orders::LOW { crate::orders::LOW { stance: String::from("lowReady"), upper: String::from("none") } }

/// (src/game/orders.ts:2630 AIM: a table nothing writes)
pub fn aim() -> crate::orders::AIM { crate::orders::AIM { stance: String::from("none"), upper: String::from("aim") } }

/// (src/game/orders.ts:2631 LANE_ORDERS: a table nothing writes)
pub fn lane_orders() -> Vec<String> { alloc::vec![String::from("cover"), String::from("stack"), String::from("move"), String::from("hold")] }

/// (src/game/orders.ts:2634 OFF_M)
pub const OFF_M: f64 = 0.3;

/// (src/game/orders.ts:2634 ROUND_ITER)
pub const ROUND_ITER: f64 = 30000.0;

/// (src/game/orders.ts:2634 MOUTH_M)
pub const MOUTH_M: f64 = 0.8;

/// (src/game/orders.ts:2636 SAME_PLUS_M)
pub const SAME_PLUS_M: f64 = 2.0;

/// (src/game/orders.ts:2637 SAMEWAY_BREAK)
pub const SAMEWAY_BREAK: bool = false;

/// (src/game/orders.ts:2638 SELFBODY_BREAK)
pub const SELFBODY_BREAK: bool = false;

/// (src/game/orders.ts:2639 FILEQUEUE_BREAK)
pub const FILEQUEUE_BREAK: bool = false;

/// (src/game/orders.ts:2640 PROPROUND_BREAK)
pub const PROPROUND_BREAK: bool = false;

/// (src/game/orders.ts:2662 ASIDECLEAR_BREAK)
pub const ASIDECLEAR_BREAK: bool = false;

/// (src/game/orders.ts:2663 TRAILDOORS_BREAK)
pub const TRAILDOORS_BREAK: bool = false;

/// (src/game/orders.ts:2679 ROUND_CAP)
pub const ROUND_CAP: f64 = 1.6;

/// (src/game/orders.ts:2679 ROUND_PLUS_M)
pub const ROUND_PLUS_M: f64 = 3.0;

/// (src/game/orders.ts:2681 ROUND_LESS)
pub const ROUND_LESS: f64 = 0.5;

/// (src/game/orders.ts:2695 DOOR_SAID: a table nothing writes)
pub fn door_said() -> JsMap<String, String> { { let mut t = JsMap::new(); t.set(String::from("n"), String::from("north")); t.set(String::from("s"), String::from("south")); t.set(String::from("e"), String::from("east")); t.set(String::from("w"), String::from("west")); t.set(String::from("ne"), String::from("north-east")); t.set(String::from("nw"), String::from("north-west")); t.set(String::from("se"), String::from("south-east")); t.set(String::from("sw"), String::from("south-west")); t.set(String::from("ext"), String::from("outside")); t.set(String::from("breakroom"), String::from("break room")); t.set(String::from("mgr"), String::from("manager's")); t } }
