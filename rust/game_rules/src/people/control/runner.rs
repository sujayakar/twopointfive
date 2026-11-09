//! BROKE AND RAN: he has dropped his gun and runs for the way out. The rule's own code and state are runner/ (moved by runner_tick).
//! This file says it in the list's words.
use super::{Outcome, Reaction};
use crate::game::{Game, GuardScript};
use crate::ids::*;

pub struct Rule;
impl Reaction for Rule {
    /// kept in the rule's own module for now
    type Row = ();
    type Ask = GuardScript;
    fn propose(g: &Game, gd: GuardId) -> Option<GuardScript> { crate::runner::runner_propose(g, gd) }
    fn commit(g: &mut Game, gd: GuardId, o: Outcome) {
        if let Outcome::Script(ran) = o { crate::runner::runner_script_ran(g, gd, ran); }
    }
}
