//! The types of `rescue`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, ToBytes)]
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
    /// INVENTED by a review: see the proposals
    pub noise_serial: Option<u32>,
}
impl Rescue {
    /// what a new one holds
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
            noise_serial: None,
        }
    }
}
impl Default for Rescue { fn default() -> Self { Self::blank() } }

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Entry {
    pub ops: Vec<crate::rescue::EntryOpsItem>,
    pub gate: DoorGateId,
    pub men: Vec<GuardId>,
    pub split: bool,
    pub set_t: f64,
    pub go_t: f64,
    pub in_t: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct EntryOpsItem {
    pub op: DoorOpId,
    pub side: bool,
    pub seen: f64,
    pub kicked: bool,
    pub ended: Option<String>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct RescueLogItem {
    pub t: f64,
    pub what: String,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct RescueResult {
    pub state: crate::rescue::RescueResultState,
    pub grade: crate::rescue::RescueResultGrade,
    pub why: String,
    pub hostages: f64,
    pub team: f64,
    pub secs: f64,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum RescueResultState {
    #[default] Lost,
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

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum RescueResultGrade {
    #[default] Failed,
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

#[derive(Clone, Copy, Debug, ToBytes, Default)]
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
    pub noise0: Option<u32>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum Role {
    #[default] Guard,
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

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct DefMark {
    pub pos: V3,
    pub aim_at: V3,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum DefCrew {
    #[default] Empty,
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

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct RescueAct {
    pub by: GuardId,
    pub victim: GuardId,
    pub at: f64,
    pub def: Option<RoomfightDefId>,
    pub hold: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes)]
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

#[derive(Clone, Copy, Debug, Default)]
pub struct RescueLastOrder {
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct RescueSearchAValue {
    pub a: OrdersAssignmentId,
    pub face: Option<f64>,
    pub watch: Option<V3>,
    pub there: String,
}

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

#[derive(Clone, Debug, Default)]
pub struct RescuePick {
    pub front: Vec<GuardId>,
    pub side: Vec<GuardId>,
}

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

#[derive(Clone, Debug, Default)]
pub struct RescueObjective {
    pub text: String,
    pub pos: Option<V3>,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes)]
pub struct RescueMod {
    /// (src/game/rescue.ts:150 applied)
    pub applied: Option<crate::scene_scenarios::TeamStart>,
}
impl RescueMod {
    pub fn blank() -> Self {
        RescueMod {
            // (the team's start the knob table holds as it is made: a start whose level names another writes its own over the knobs)
            applied: { let k = crate::knobs::page_rescue(); Some(crate::scene_scenarios::TeamStart { sam: k.sam_at, sam_yaw: k.sam_yaw, ops: Some(k.ops_at.to_vec()), ops_yaw: None }) },
        }
    }
}

/// (src/game/rescue.ts:156 NUMBER: a table nothing writes)
pub fn number() -> Vec<String> { alloc::vec![String::from("no"), String::from("one"), String::from("two"), String::from("three"), String::from("four"), String::from("five"), String::from("six")] }

/// (src/game/rescue.ts:161 SEARCH_RATE)
pub const SEARCH_RATE: f64 = 1.3;

/// (src/game/rescue.ts:161 SEARCH_HALF)
pub const SEARCH_HALF: f64 = 1.75;

/// (src/game/rescue.ts:218 CREW_SEEN)
pub const CREW_SEEN: f64 = 4.0;
