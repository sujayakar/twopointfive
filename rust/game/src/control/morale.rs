//! SHAKEN BY A BLAST: he is down, or backing off, until his nerve returns. The rule's own code and state are morale/ (moved by morale_tick).
//! This file says it in the list's words.
use super::{Intent, Outcome};
use crate::game::Game;
use crate::ids::*;

/// kept in the rule's own module for now
pub type Row = ();

pub fn transition(g: &mut Game, gd: GuardId, _dt: f64) { crate::morale::p_02::morale_transition(g, gd); }

pub fn propose(g: &Game, gd: GuardId) -> Option<Intent> { crate::morale::p_02::morale_propose(g, gd).map(Intent::Script) }

pub fn commit(g: &mut Game, gd: GuardId, o: Outcome) {
    if let Outcome::Script(ran) = o { crate::morale::morale_script_ran(g, gd, ran); }
}

pub fn lost(_: &mut Game, _: GuardId) {}
