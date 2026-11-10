//! SHAKEN BY A BLAST: he is down, or backing off, until his nerve returns. The rule's own code and state are morale/ (moved by morale_tick).
//! This file says it in the list's words.
use super::{Outcome, Reaction};
use crate::game::{Game, GuardScript};
use crate::ids::*;

pub struct Rule;
impl Reaction for Rule {
    /// kept in the rule's own module for now
    type Row = ();
    type Ask = GuardScript;
    fn transition(g: &mut Game, gd: GuardId, _dt: f64) { crate::morale::morale_transition(g, gd); }
    fn propose(g: &Game, gd: GuardId) -> Option<GuardScript> { crate::morale::morale_propose(g, gd) }
    fn commit(g: &mut Game, gd: GuardId, o: Outcome) {
        if let Outcome::Script(ran) = o { crate::morale::morale_script_ran(g, gd, ran); }
    }
}
