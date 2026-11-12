//! CAUGHT IN THE OPEN: rounds have a man at his post pinned, and getting down where he stands hides him from nobody. He goes for cover, if there is any, his gun on them. Whether it would hide
//! him, and where the cover is, are the suppression rule's (suppression.rs); this file says it in the list's words.
use super::{LeavePost, Outcome, Reaction};
use crate::game::Game;
use crate::ids::*;
use crate::roomfight::Leg;
use crate::v3::{self, V3};

/// the cover he has found this tick, and where the rounds come from
#[derive(Clone, Copy, Debug, Default, crate::bytes::ToBytes)]
pub struct Row { pub to: Option<V3>, pub from: V3 }

pub struct Rule;
impl Reaction for Rule {
    type Row = Row;
    type Ask = LeavePost;

    fn transition(g: &mut Game, gd: GuardId, _dt: f64) {
        let to = crate::suppression::caught_open(g, gd);
        let from = g.cond[g.gd[gd].ch].sup.src.unwrap_or(g.gd[gd].last_known.unwrap_or_default());
        *super::state_mut(g).rows.open.at(gd) = Row { to, from };
    }

    fn propose(g: &Game, gd: GuardId) -> Option<LeavePost> {
        let r = super::state(g)?.rows.open.get(gd)?;
        let pace = g.k.tune.guard_investigate * g.k.suppression.displace_pace;
        let at = g.pos_of(gd);
        let aim = Some([r.from[0], 1.1, r.from[2]]);
        Some(LeavePost::On(Leg { cover: true, face: Some(v3::yaw_to(at, r.from)), aim, ..Leg::to(r.to?, pace) }))
    }

    fn commit(g: &mut Game, gd: GuardId, _: Outcome) {
        if let Some(d) = crate::roomfight::def_of(g, gd) { crate::roomfight::took_cover_said(g, d); }
    }
}
