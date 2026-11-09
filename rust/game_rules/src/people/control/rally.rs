//! A FIGHT NEXT DOOR: he can hear one at a post of his side's in another room, and after his own few seconds he goes to it, or a quarter of the time does not. The clock, and who may
//! go, are the room fight's (roomfight/defender.rs); this file says it in the list's words.
use super::{LeavePost, Outcome, Reaction};
use crate::game::Game;
use crate::ids::*;
use crate::roomfight::{def_of, room_fight_rally, room_fight_rallied};
use crate::V3;

/// where the fight is that he goes to this tick
#[derive(Clone, Copy, Debug, Default, crate::bytes::ToBytes)]
pub struct Row { pub at: Option<V3> }

pub struct Rule;
impl Reaction for Rule {
    type Row = Row;
    type Ask = LeavePost;

    fn transition(g: &mut Game, gd: GuardId, _dt: f64) {
        let at = def_of(g, gd).and_then(|d| room_fight_rally(g, d));
        super::state_mut(g).rows.rally.at(gd).at = at;
    }

    fn propose(g: &Game, gd: GuardId) -> Option<LeavePost> { super::state(g)?.rows.rally.get(gd)?.at.map(|_| LeavePost::Loose) }

    fn commit(g: &mut Game, gd: GuardId, _: Outcome) {
        let at = super::state_mut(g).rows.rally.at(gd).at.take();
        if let (Some(at), Some(d)) = (at, def_of(g, gd)) { room_fight_rallied(g, d, at); }
    }
}
