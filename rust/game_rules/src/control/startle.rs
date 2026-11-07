//! STARTLED: a green man caught alone and close runs for cover. The rule's own code and state are startle/.
//! This file says it in the list's words.
use super::{Outcome, Reaction};
use crate::game::{Game, GuardScript};
use crate::ids::*;

pub struct Rule;
impl Reaction for Rule {
    /// kept in the rule's own module for now
    type Row = ();
    type Ask = GuardScript;
    fn transition(g: &mut Game, gd: GuardId, _dt: f64) { crate::startle::p_01::startle_transition(g, gd); }
    fn propose(g: &Game, gd: GuardId) -> Option<GuardScript> { crate::startle::p_01::startle_propose(g, gd) }
    fn commit(g: &mut Game, gd: GuardId, o: Outcome) {
        if let Outcome::Script(ran) = o { crate::startle::startle_script_ran(g, gd, ran); }
    }
}
