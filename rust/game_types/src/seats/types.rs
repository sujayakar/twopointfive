//! The types of `seats`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, Default)]
pub struct ViewOfBodyC {
    pub id: f64,
}

#[derive(Clone, Debug, ToBytes)]
#[bytes(parts)]
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
    /// (the host's camera: in no bytes)
    #[bytes(skip)]
    pub view_proj: Option<crate::game::GameViewProj>,
    pub threat_proximity: f64,
    pub torch_on_t: f64,
}
impl Default for SeatSet {
    fn default() -> Self {
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

pub fn seat_kinds() -> Vec<String> { alloc::vec![String::from("roomclear"), String::from("nightbreach")] }

