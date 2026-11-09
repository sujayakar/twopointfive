//! HIS ROOM IS LOST: enough of the men who held it with him are down in his view, and he is pressed. He goes: to his squad's rally point, out by a door nothing has come at, or
//! to cover. What counts, and where he may go, are the room fight's (roomfight/defender.rs); this file says it in the list's words.
use super::{Outcome, Reaction};
use crate::game::Game;
use crate::ids::*;
use crate::roomfight::{break_off_said, break_off_way, def_of, room_fight_room_lost, Leg, WayOut};

/// the way out he has found this tick
#[derive(Clone, Copy, Debug, Default, crate::bytes::ToBytes)]
pub struct Row { pub way: Option<WayOut> }

pub struct Rule;
impl Reaction for Rule {
    type Row = Row;
    type Ask = Leg;

    fn transition(g: &mut Game, gd: GuardId, _dt: f64) {
        let way = def_of(g, gd).filter(|&d| room_fight_room_lost(g, d)).and_then(|d| break_off_way(g, d));
        super::state_mut(g).rows.lost.at(gd).way = way;
    }

    fn propose(g: &Game, gd: GuardId) -> Option<Leg> {
        let way = super::state(g)?.rows.lost.get(gd)?.way?;
        Some(way.leg(g))
    }

    fn commit(g: &mut Game, gd: GuardId, _: Outcome) {
        let way = super::state_mut(g).rows.lost.at(gd).way.take();
        if let (Some(w), Some(d)) = (way, def_of(g, gd)) { break_off_said(g, d, w); }
    }
}
