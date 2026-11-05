//! The types of `seats`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum SeatTone {
    You,
    Purple,
}
impl SeatTone {
    pub const fn as_str(self) -> &'static str {
        match self {
            SeatTone::You => "you",
            SeatTone::Purple => "purple",
        }
    }
}

/// (TypeScript: inline at src/game/seats.ts:48 SEAT_ASKS, src/game/seats.ts:48)
#[derive(Clone, Debug)]
pub struct SeatsModSEATASKS {
    pub strict: bool,
    pub shared: bool,
    pub misses: f64,
    pub missed: JsSet<String>,
    pub sites: Option<JsMap<String, f64>>,
}
impl SeatsModSEATASKS {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SeatsModSEATASKS {
            strict: false,
            shared: false,
            misses: 0.0,
            missed: JsSet::new(),
            sites: None,
        }
    }
}
impl Default for SeatsModSEATASKS { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/seats.ts:111 men(g), ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1604)
#[derive(Clone, Debug)]
pub struct MenG {
    pub player: PlayerId,
    pub seats: Vec<SeatId>,
}
impl MenG {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MenG {
            player: PlayerId(0),
            seats: Vec::new(),
        }
    }
}
impl Default for MenG { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/seats.ts:212 viewOfBody(c), src/game/seats.ts:212)
#[derive(Clone, Copy, Debug)]
pub struct ViewOfBodyC {
    pub id: f64,
}
impl ViewOfBodyC {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ViewOfBodyC {
            id: 0.0,
        }
    }
}
impl Default for ViewOfBodyC { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: seats.ts SEAT_KEYS: what a seat keeps while another seat is on Game; seats::with_seat swaps it)
#[derive(Clone, Debug)]
pub struct SeatSet {
    pub seat: SeatId,
    pub player: PlayerId,
    pub orders: crate::orders::SquadOrders,
    pub door_tools: crate::doortools::DoorTools,
    pub work: crate::workthings::WorkThings,
    pub wall_charge: crate::wallcharge::WallCharge,
    pub aim_point: V3,
    pub aim_target: Option<FixtureId>,
    pub aim_guard: Option<GuardId>,
    pub aim_friend: Option<GuardId>,
    pub aim_snap: Option<V3>,
    pub hover: Option<crate::game::Interactable>,
    pub interactables: Vec<crate::game::Interactable>,
    pub nearby: Vec<crate::game::Interactable>,
    pub act: Option<crate::game::Interactable>,
    pub use_door: Option<DoorId>,
    pub view_proj: Option<crate::game::GameViewProj>,
    pub threat_proximity: f64,
    pub torch_on_t: f64,
}
impl SeatSet {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SeatSet {
            seat: SeatId(0),
            player: PlayerId(0),
            orders: crate::orders::SquadOrders::blank(),
            door_tools: crate::doortools::DoorTools::blank(),
            work: crate::workthings::WorkThings::blank(),
            wall_charge: crate::wallcharge::WallCharge::blank(),
            aim_point: [0.0, 0.0, 0.0],
            aim_target: None,
            aim_guard: None,
            aim_friend: None,
            aim_snap: None,
            hover: None,
            interactables: Vec::new(),
            nearby: Vec::new(),
            act: None,
            use_door: None,
            view_proj: None,
            threat_proximity: 0.0,
            torch_on_t: -100.0,
        }
    }
}
impl Default for SeatSet { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct SeatsMod {
    /// (src/game/seats.ts:48 SEAT_ASKS)
    pub seat_asks: crate::seats::SeatsModSEATASKS,
}
impl SeatsMod {
    pub fn blank() -> Self {
        SeatsMod {
            seat_asks: crate::seats::SeatsModSEATASKS { strict: false, shared: false, misses: 0.0, missed: JsSet::new(), sites: None },
        }
    }
}

/// (src/game/seats.ts:44 SEAT_KINDS: a table nothing writes)
pub fn seat_kinds() -> Vec<String> { alloc::vec![String::from("roomclear"), String::from("nightbreach")] }

/// (src/game/seats.ts:54 SEAT_KEYS: a table nothing writes)
pub fn seat_keys() -> [String; 19] { [String::from("seat"), String::from("player"), String::from("orders"), String::from("doorTools"), String::from("work"), String::from("wallCharge"), String::from("aimPoint"), String::from("aimTarget"), String::from("aimGuard"), String::from("aimFriend"), String::from("aimSnap"), String::from("hover"), String::from("interactables"), String::from("nearby"), String::from("act"), String::from("useDoor"), String::from("viewProj"), String::from("threatProximity"), String::from("torchOnT")] }

/// (src/game/seats.ts:61 SEAT_TABLES: a table nothing writes)
pub fn seat_tables() -> Vec<String> { alloc::vec![String::from("blastFeel.STATE"), String::from("blastLook.STATE"), String::from("close.STATE"), String::from("guards.groundTrack"), String::from("holdfire.states"), String::from("player.lightAt"), String::from("rearwatch.books"), String::from("samlook.STATE"), String::from("showwhy.states"), String::from("silent.seats"), String::from("silent.states"), String::from("squad40.states"), String::from("squadsmoke.states"), String::from("squadthrow.states"), String::from("stalk.STATE"), String::from("tapstep.STATE"), String::from("unseen.states"), String::from("wandorder.states")] }

/// (src/game/seats.ts:151 SEAT_TONES: a table nothing writes)
pub fn seat_tones() -> Vec<String> { alloc::vec![String::from("you"), String::from("purple")] }
