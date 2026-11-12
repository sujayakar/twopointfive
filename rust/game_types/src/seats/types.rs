//! The types of `seats`.
#![allow(unused_imports)]
use crate::{Blank, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, Default)]
pub struct ViewOfBodyC {
    pub id: f64,
}

/// What a seat's hand is doing: the keys held, what it is choosing, and what the fingers are waiting on. A fresh man starts them again.
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct SeatHand {
    pub near_sel: i32,
    #[is(String::from(""))] pub near_key: String,
    #[is(crate::game::PlayerSel { down_t: -1.0, open: false, run_key: String::from(""), move_latch: 0.0, word: String::from(""), word_t: -1000000000.0, word_kind: None, ring: None })] pub sel: crate::game::PlayerSel,
    pub setup: Option<crate::game::PlayerSetup>,
    pub setup_key: Option<crate::game::PlayerSetupKey>,
    pub f_held: bool,
    pub throw_preview: Option<crate::throwables::ThrowSolution>,
    pub friend_low: bool,
    #[is(-10.0)] pub friend_t: f64,
    pub catch_due: Option<f64>,
}

#[derive(Clone, Debug, ToBytes)]
#[bytes(parts)]
pub struct SeatSet {
    pub player: PlayerId,
    /// his squad
    pub group: GroupId,
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
    /// (the host's camera: in no bytes)
    #[bytes(skip)]
    pub view_proj: Option<crate::game::GameViewProj>,
    pub threat_proximity: f64,
    pub torch_on_t: f64,
    pub hand: SeatHand,
}
impl Default for SeatSet {
    fn default() -> Self {
        SeatSet {
            player: PlayerId(0),
            group: GroupId(0),
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
            view_proj: None,
            threat_proximity: 0.0,
            torch_on_t: -100.0,
            hand: SeatHand::blank(),
        }
    }
}

/// a game's seats' sets before it has more than its first
pub fn sets_new() -> Tab<SeatId, SeatSet> {
    let mut t = Tab::new();
    t.push(SeatSet::default());
    t
}

pub fn seat_kinds() -> Vec<String> { alloc::vec![String::from("roomclear"), String::from("nightbreach")] }

