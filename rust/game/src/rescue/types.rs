//! The types of `rescue`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: Rescue, src/game/rescue.ts:219)
#[derive(Clone, Debug)]
pub struct Rescue {
    pub active: bool,
    pub ops: Vec<GuardId>,
    pub hostiles: Vec<GuardId>,
    pub hostages: Vec<GuardId>,
    pub plan: Option<crate::scene_level::RoomClearPlan>,
    pub entry: Option<crate::rescue::Entry>,
    pub staged: f64,
    pub log: Vec<crate::rescue::RescueLogItem>,
    pub resolve: f64,
    pub resolve_secs: f64,
    pub t0: f64,
    pub door_orders: f64,
    pub blind_rounds: f64,
    pub result: Option<crate::rescue::RescueResult>,
    pub defs: JsMap<GuardId, crate::rescue::Def>,
    pub act: Option<crate::rescue::RescueAct>,
    pub cap_by: Option<GuardId>,
    pub executed: JsSet<GuardId>,
    pub counted: JsSet<GuardId>,
    pub secured_at: f64,
    pub step: crate::rescue::RescueStep,
    pub bark_at: f64,
    pub noise: Option<V3>,
    pub heard: JsSet<u32>,
    pub last_order: Option<u32>,
    pub alarm_at: f64,
    pub way: Option<Vec<V3>>,
    pub benched: Option<Vec<GuardId>>,
    pub said: JsSet<String>,
    pub cut: bool,
    pub failure: Option<String>,
    pub murmur_at: f64,
    pub murmur_n: f64,
    pub ack_at: f64,
    pub flow: alloc::boxed::Box<crate::rescueflow::RescueFlow>,
    pub called: bool,
    pub search_t: JsMap<GuardId, f64>,
    pub search_a: JsMap<GuardId, crate::rescue::RescueSearchAValue>,
    pub auto: JsSet<OrdersAssignmentId>,
    pub server_out: Option<V3>,
    pub ending: crate::rescueend::RescueEnd,
    pub flush: crate::rescueflush::RescueFlush,
}
impl Rescue {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        Rescue {
            active: false,
            ops: Vec::new(),
            hostiles: Vec::new(),
            hostages: Vec::new(),
            plan: None,
            entry: None,
            staged: 0.0,
            log: Vec::new(),
            resolve: 0.0,
            resolve_secs: 35.0,
            t0: f64::NAN,
            door_orders: 0.0,
            blind_rounds: 0.0,
            result: None,
            defs: JsMap::new(),
            act: None,
            cap_by: None,
            executed: JsSet::new(),
            counted: JsSet::new(),
            secured_at: f64::NAN,
            step: crate::rescue::RescueStep::Waiting,
            bark_at: 0.0,
            noise: None,
            heard: JsSet::new(),
            last_order: None,
            alarm_at: f64::NAN,
            way: None,
            benched: None,
            said: JsSet::new(),
            cut: false,
            failure: None,
            murmur_at: 0.0,
            murmur_n: 0.0,
            ack_at: f64::NAN,
            flow: alloc::boxed::Box::new(crate::rescueflow::RescueFlow::blank()),
            called: false,
            search_t: JsMap::new(),
            search_a: JsMap::new(),
            auto: JsSet::new(),
            server_out: None,
            ending: crate::rescueend::RescueEnd::blank(),
            flush: crate::rescueflush::RescueFlush::blank(),
        }
    }
}
impl Default for Rescue { fn default() -> Self { Self::blank() } }

/// (TypeScript: Entry, src/game/rescue.ts:211)
#[derive(Clone, Debug)]
pub struct Entry {
    pub ops: Vec<crate::rescue::EntryOpsItem>,
    pub gate: DoorGateId,
    pub men: Vec<GuardId>,
    pub split: bool,
    pub set_t: f64,
    pub go_t: f64,
    pub in_t: f64,
}
impl Entry {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Entry {
            ops: Vec::new(),
            gate: DoorGateId(0),
            men: Vec::new(),
            split: false,
            set_t: 0.0,
            go_t: 0.0,
            in_t: 0.0,
        }
    }
}
impl Default for Entry { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/rescue.ts:211 Entry.ops, src/game/rescue.ts:211)
#[derive(Clone, Debug)]
pub struct EntryOpsItem {
    pub op: DoorOpId,
    pub side: bool,
    pub seen: f64,
    pub kicked: bool,
    pub ended: Option<String>,
}
impl EntryOpsItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        EntryOpsItem {
            op: DoorOpId(0),
            side: false,
            seen: 0.0,
            kicked: false,
            ended: None,
        }
    }
}
impl Default for EntryOpsItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/rescue.ts:234 Rescue.log, src/game/rescue.ts:234)
#[derive(Clone, Debug)]
pub struct RescueLogItem {
    pub t: f64,
    pub what: String,
}
impl RescueLogItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RescueLogItem {
            t: 0.0,
            what: String::new(),
        }
    }
}
impl Default for RescueLogItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/rescue.ts:242 Rescue.result, src/game/rescue.ts:242)
#[derive(Clone, Debug)]
pub struct RescueResult {
    pub state: crate::rescue::RescueResultState,
    pub grade: crate::rescue::RescueResultGrade,
    pub why: String,
    pub hostages: f64,
    pub team: f64,
    pub secs: f64,
}
impl RescueResult {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RescueResult {
            state: crate::rescue::RescueResultState::Lost,
            grade: crate::rescue::RescueResultGrade::Failed,
            why: String::new(),
            hostages: 0.0,
            team: 0.0,
            secs: 0.0,
        }
    }
}
impl Default for RescueResult { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum RescueResultState {
    Lost,
    Rescued,
}
impl RescueResultState {
    pub const fn as_str(self) -> &'static str {
        match self {
            RescueResultState::Lost => "lost",
            RescueResultState::Rescued => "rescued",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum RescueResultGrade {
    Failed,
    Clean,
    Costly,
}
impl RescueResultGrade {
    pub const fn as_str(self) -> &'static str {
        match self {
            RescueResultGrade::Failed => "failed",
            RescueResultGrade::Clean => "clean",
            RescueResultGrade::Costly => "costly",
        }
    }
}

/// (TypeScript: Def, src/game/rescue.ts:214)
#[derive(Clone, Copy, Debug)]
pub struct Def {
    pub role: crate::rescue::Role,
    pub in_room: bool,
    pub mark: crate::rescue::DefMark,
    pub learned: f64,
    pub set: bool,
    pub saw_foe: bool,
    pub released: bool,
    pub burst: crate::nightbreach::BlindBurst,
    pub k: f64,
    pub crew: crate::rescue::DefCrew,
    pub look: Option<V3>,
    pub spot: Option<V3>,
    pub noise0: Option<V3>,
}
impl Def {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Def {
            role: crate::rescue::Role::Guard,
            in_room: false,
            mark: crate::rescue::DefMark::zeroed(),
            learned: 0.0,
            set: false,
            saw_foe: false,
            released: false,
            burst: crate::nightbreach::BlindBurst::zeroed(),
            k: 0.0,
            crew: crate::rescue::DefCrew::Empty,
            look: None,
            spot: None,
            noise0: None,
        }
    }
}
impl Default for Def { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Role {
    Guard,
    Door,
    Captor,
    Roamer,
}
impl Role {
    pub const fn as_str(self) -> &'static str {
        match self {
            Role::Guard => "guard",
            Role::Door => "door",
            Role::Captor => "captor",
            Role::Roamer => "roamer",
        }
    }
}

/// (TypeScript: inline at src/game/rescue.ts:214 Def.mark, src/game/rescue.ts:214)
#[derive(Clone, Copy, Debug)]
pub struct DefMark {
    pub pos: V3,
    pub aim_at: V3,
}
impl DefMark {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DefMark {
            pos: [0.0; 3],
            aim_at: [0.0; 3],
        }
    }
}
impl Default for DefMark { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum DefCrew {
    Empty,
    Cross,
    Out,
    Wait,
    Going,
    In,
}
impl DefCrew {
    pub const fn as_str(self) -> &'static str {
        match self {
            DefCrew::Empty => "",
            DefCrew::Cross => "cross",
            DefCrew::Out => "out",
            DefCrew::Wait => "wait",
            DefCrew::Going => "going",
            DefCrew::In => "in",
        }
    }
}

/// (TypeScript: inline at src/game/rescue.ts:244 Rescue.act, src/game/rescue.ts:244)
#[derive(Clone, Copy, Debug)]
pub struct RescueAct {
    pub by: GuardId,
    pub victim: GuardId,
    pub at: f64,
    pub def: Option<RoomfightDefId>,
    pub hold: bool,
}
impl RescueAct {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RescueAct {
            by: GuardId(0),
            victim: GuardId(0),
            at: 0.0,
            def: None,
            hold: false,
        }
    }
}
impl Default for RescueAct { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum RescueStep {
    Waiting,
    Shouting,
    Counting,
    AboutTo,
    Stopped,
}
impl RescueStep {
    pub const fn as_str(self) -> &'static str {
        match self {
            RescueStep::Waiting => "waiting",
            RescueStep::Shouting => "shouting",
            RescueStep::Counting => "counting",
            RescueStep::AboutTo => "about to",
            RescueStep::Stopped => "stopped",
        }
    }
}

/// (TypeScript: inline at quiet Rescue.lastOrder, quiet Rescue.lastOrder)
#[derive(Clone, Copy, Debug)]
pub struct RescueLastOrder {
}
impl RescueLastOrder {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RescueLastOrder {
        }
    }
}
impl Default for RescueLastOrder { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/rescue.ts:260 Rescue.searchA, src/game/rescue.ts:260)
#[derive(Clone, Debug)]
pub struct RescueSearchAValue {
    pub a: OrdersAssignmentId,
    pub face: Option<f64>,
    pub watch: Option<V3>,
    pub there: String,
}
impl RescueSearchAValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RescueSearchAValue {
            a: OrdersAssignmentId(0),
            face: None,
            watch: None,
            there: String::new(),
        }
    }
}
impl Default for RescueSearchAValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum RescueState {
    Lost,
    Off,
    Rescued,
    Staged,
    Secured,
    Running,
}
impl RescueState {
    pub const fn as_str(self) -> &'static str {
        match self {
            RescueState::Lost => "lost",
            RescueState::Off => "off",
            RescueState::Rescued => "rescued",
            RescueState::Staged => "staged",
            RescueState::Secured => "secured",
            RescueState::Running => "running",
        }
    }
}

/// (TypeScript: inline at src/game/rescue.ts:153 SIDE_MARKS, src/game/rescue.ts:153)
#[derive(Clone, Debug)]
pub struct SIDEMARKS {
    pub stack: Vec<V3>,
    pub standoff: Vec<V3>,
}
impl SIDEMARKS {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SIDEMARKS {
            stack: Vec::new(),
            standoff: Vec::new(),
        }
    }
}
impl Default for SIDEMARKS { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum RescueRoomPointWhich {
    Hold,
    Outside,
}
impl RescueRoomPointWhich {
    pub const fn as_str(self) -> &'static str {
        match self {
            RescueRoomPointWhich::Hold => "hold",
            RescueRoomPointWhich::Outside => "outside",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum RescueMode {
    Door,
    Free,
}
impl RescueMode {
    pub const fn as_str(self) -> &'static str {
        match self {
            RescueMode::Door => "door",
            RescueMode::Free => "free",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum RescuePress {
    Bang,
    Blow,
    Stack,
    Go,
}
impl RescuePress {
    pub const fn as_str(self) -> &'static str {
        match self {
            RescuePress::Bang => "bang",
            RescuePress::Blow => "blow",
            RescuePress::Stack => "stack",
            RescuePress::Go => "go",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum RescueOrderDoor {
    Bang,
    Stack,
    Go,
}
impl RescueOrderDoor {
    pub const fn as_str(self) -> &'static str {
        match self {
            RescueOrderDoor::Bang => "bang",
            RescueOrderDoor::Stack => "stack",
            RescueOrderDoor::Go => "go",
        }
    }
}

/// (TypeScript: inline at src/game/rescue.ts:481 pick ->, src/game/rescue.ts:481)
#[derive(Clone, Debug)]
pub struct RescuePick {
    pub front: Vec<GuardId>,
    pub side: Vec<GuardId>,
}
impl RescuePick {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RescuePick {
            front: Vec::new(),
            side: Vec::new(),
        }
    }
}
impl Default for RescuePick { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum RescueEndState {
    Lost,
    Rescued,
}
impl RescueEndState {
    pub const fn as_str(self) -> &'static str {
        match self {
            RescueEndState::Lost => "lost",
            RescueEndState::Rescued => "rescued",
        }
    }
}

/// (TypeScript: inline at src/game/rescue.ts:1034 objective ->, src/game/rescue.ts:1034)
#[derive(Clone, Debug)]
pub struct RescueObjective {
    pub text: String,
    pub pos: Option<V3>,
}
impl RescueObjective {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RescueObjective {
            text: String::new(),
            pos: None,
        }
    }
}
impl Default for RescueObjective { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct RescueMod {
    /// (src/game/rescue.ts:148 applied)
    pub applied: Option<crate::scene_scenarios::TeamStart>,
    /// (src/game/rescue.ts:159 DOOR_ORDERS)
    pub door_orders: JsSet<String>,
}
impl RescueMod {
    pub fn blank() -> Self {
        RescueMod {
            applied: Some(crate::scene_scenarios::TeamStart { sam: [7.6, 0.0, 13.4], sam_yaw: 0.9, ops: Some(alloc::vec![[6.6, 0.0, 14.25], [8.5, 0.0, 14.3]]), ops_yaw: None }),
            door_orders: { let mut s = JsSet::new(); for w in ["openClear", "bangClear", "charge", "outside", "threshold"] { s.add(String::from(w)); } s },
        }
    }
}

/// (src/game/rescue.ts:153 SIDE_MARKS: a table nothing writes)
pub fn side_marks() -> crate::rescue::SIDEMARKS { crate::rescue::SIDEMARKS { stack: alloc::vec![[20.85, 0.0, 7.55], [20.15, 0.0, 7.55]], standoff: alloc::vec![[20.5, 0.0, 7.55], [19.8, 0.0, 7.55]] } }

/// (src/game/rescue.ts:154 NUMBER: a table nothing writes)
pub fn number() -> Vec<String> { alloc::vec![String::from("no"), String::from("one"), String::from("two"), String::from("three"), String::from("four"), String::from("five"), String::from("six")] }

/// (src/game/rescue.ts:159 SEARCH_RATE)
pub const SEARCH_RATE: f64 = 1.3;

/// (src/game/rescue.ts:159 SEARCH_HALF)
pub const SEARCH_HALF: f64 = 1.75;

/// (src/game/rescue.ts:216 CREW_SEEN)
pub const CREW_SEEN: f64 = 4.0;
