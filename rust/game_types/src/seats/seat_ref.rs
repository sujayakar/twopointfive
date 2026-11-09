//! A SEAT AS A VALUE. A seat's own things are nineteen fields of the game, and they are those of whichever seat STANDS on it; every other seat's
//! are kept aside (`cmd_seat[s].own`) and changed over for a pass about that seat (`with_seat`, under `&mut`). A function that reads `g.orders`
//! therefore answers for the seat that happens to stand, and nothing in its signature says so.
//!
//! `SeatRef` is one seat's things, borrowed from wherever they are. A function that holds the game SHARED and depends on the seat takes one
//! (`seat: &SeatRef`) and reads `seat.orders`, never `g.orders`: so it can be asked about any seat, on a game nobody may write to. Two ways to get one:
//!   `seats::seat(g, s)`   any seat
//!   `seats::here(g)`      the seat a pass is about: the one standing. A tick function that calls a reader passes `&seats::here(g)`, and each of
//!                         those is a place where the tick SAYS whose seat it means.
//! Only this folder names the nineteen through a shared game (tools/blitz/sweeps/seat-ref.py --check holds that).
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
}

/// the seat a pass is about: the one standing on the game
pub fn here(g: &Game) -> SeatRef<'_> {
    SeatRef {
        id: g.seat, player: g.player, orders: &g.orders, door_tools: &g.door_tools, work: &g.work, wall_charge: &g.wall_charge,
        aim_point: g.aim_point, aim_target: g.aim_target, aim_guard: g.aim_guard, aim_snap: g.aim_snap,
        hover: &g.hover, interactables: &g.interactables, nearby: &g.nearby, act: &g.act, view_proj: &g.view_proj,
    }
}

/// any seat: from the game where it stands, from where it is kept where it does not
pub fn seat(g: &Game, s: SeatId) -> SeatRef<'_> {
    if g.seat == s { return here(g); }
    let o = g.tab.cmd_seat[s].own.as_ref().unwrap_or_else(|| panic!("seats::seat: {:?} neither stands on the game nor keeps a set", s));
    SeatRef {
        id: o.seat, player: o.player, orders: &o.orders, door_tools: &o.door_tools, work: &o.work, wall_charge: &o.wall_charge,
        aim_point: o.aim_point, aim_target: o.aim_target, aim_guard: o.aim_guard, aim_snap: o.aim_snap,
        hover: &o.hover, interactables: &o.interactables, nearby: &o.nearby, act: &o.act, view_proj: &o.view_proj,
    }
}
