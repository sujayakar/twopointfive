//! A SEAT AS A VALUE. A seat's own things stand in its own row (`g.sets[s]`), the seat a pass is about is named by `g.seat`. `SeatRef` is one seat's things,
//! borrowed: a function that holds the game SHARED and depends on the seat takes one (`seat: &SeatRef`), so it can be asked about any seat. Two ways to get one:
//!   `seats::seat(g, s)`   any seat
//!   `seats::here(g)`      the seat a pass is about. A tick function that calls a reader passes `&seats::here(g)`, and each of those is a place where the tick SAYS whose
//!                         seat it means.
use crate::game::{Game, Interactable};
use crate::ids::*;
use crate::V3;
use alloc::vec::Vec;

/// one seat's own things, by the names the game has them under
#[derive(Clone, Copy)]
pub struct SeatRef<'a> {
    pub id: SeatId,
    /// the seat's man
    pub player: PlayerId,
    /// the group the pass is about, where this is the seat it is about; else the seat's own squad
    pub group: &'a crate::group::Group,
    pub group_id: GroupId,
    pub orders: &'a crate::orders::SquadOrders,
    pub door_tools: &'a crate::doortools::DoorTools,
    pub work: &'a crate::workthings::WorkThings,
    pub wall_charge: &'a crate::wallcharge::WallCharge,
    pub aim_point: V3,
    pub aim_target: Option<FixtureId>,
    pub aim_guard: Option<GuardId>,
    pub aim_snap: Option<V3>,
    pub hover: &'a Option<Interactable>,
    pub interactables: &'a Vec<Interactable>,
    pub nearby: &'a Vec<Interactable>,
    pub act: &'a Option<Interactable>,
    pub view_proj: &'a Option<crate::game::GameViewProj>,
    pub hand: &'a crate::seats::SeatHand,
}

/// the seat a pass is about
pub fn here(g: &Game) -> SeatRef<'_> {
    seat(g, g.seat)
}

/// any seat
pub fn seat(g: &Game, s: SeatId) -> SeatRef<'_> {
    let o = &g.sets[s];
    let group_id = if g.seat == s { g.group } else { o.group };
    let group = &g.groups[group_id];
    SeatRef {
        id: s, player: if g.seat == s { g.player } else { o.player }, group, group_id, orders: &group.orders, door_tools: &group.tools, work: &o.work, wall_charge: &o.wall_charge,
        aim_point: o.aim_point, aim_target: o.aim_target, aim_guard: o.aim_guard, aim_snap: o.aim_snap,
        hover: &o.hover, interactables: &o.interactables, nearby: &o.nearby, act: &o.act, view_proj: &o.view_proj, hand: &o.hand,
    }
}
