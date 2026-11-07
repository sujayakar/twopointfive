//! BROKE AND RAN: he has dropped his gun and runs for the way out. The rule's own code and state are runner/ (moved by runner_tick).
//! This file says it in the list's words.
use super::{Intent, Outcome};
use crate::game::Game;
use crate::ids::*;

/// kept in the rule's own module for now
pub type Row = ();

pub fn transition(g: &mut Game, gd: GuardId, _dt: f64) { let _ = (g, gd); }

pub fn propose(g: &Game, gd: GuardId) -> Option<Intent> { crate::runner::p_01::runner_propose(g, gd).map(Intent::Script) }

pub fn commit(g: &mut Game, gd: GuardId, o: Outcome) {
    if let Outcome::Script(ran) = o { crate::runner::runner_script_ran(g, gd, ran); }
}

pub fn lost(_: &mut Game, _: GuardId) {}
