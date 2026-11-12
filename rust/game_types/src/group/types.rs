//! The types of `group`.
use crate::{bytes::ToBytes, ids::*, js::*};
use alloc::string::String;

/// A GROUP: men somebody commands together, whichever side they are on. A player's squad is one (in a game of two each player has his own), and so is the response team.
/// What a pass over the squad's code is about is a group (`g.group`): its book of orders, its door work.
#[derive(Clone, Debug, ToBytes)]
#[bytes(parts)]
pub struct Group {
    pub side: crate::teams::Team,
    pub commander: Commander,
    /// the keys its men try a locked door with, beside a player's own
    pub ring: JsSet<String>,
    /// what its men have been told
    pub orders: crate::orders::SquadOrders,
    /// what they are doing at doors
    pub tools: crate::doortools::DoorTools,
}

/// Who says what a group does.
#[derive(Clone, Copy, PartialEq, Eq, Debug, ToBytes)]
pub enum Commander {
    /// a player, by his keys
    Seat(SeatId),
    /// the squad itself, of those who hold a place (`enemysquads/`)
    Squad(EnemysquadsSquadId),
    /// the response team's own drill (`qrf/`)
    Response,
}

impl Group {
    /// its men are a player's own
    pub fn by_a_player(&self) -> bool { matches!(self.commander, Commander::Seat(_)) }
    /// one with nobody told anything yet
    pub fn new(side: crate::teams::Team, commander: Commander) -> Group {
        Group { side, commander, ring: JsSet::new(), orders: crate::orders::SquadOrders::blank(), tools: crate::doortools::DoorTools::blank() }
    }
}

/// a game's groups before it has more than its first seat's
pub fn groups_new() -> Tab<GroupId, Group> {
    let mut t = Tab::new();
    t.push(Group::new(crate::teams::Team::Ours, Commander::Seat(SeatId(0))));
    t
}
