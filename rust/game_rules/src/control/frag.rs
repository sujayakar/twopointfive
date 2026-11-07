//! A LIVE GRENADE he knows of has a line to him: he runs from it and gets down. The rule's own code and state are frag/ (the dealing is frag_tick's).
//! This file says it in the list's words.
use super::{Intent, Outcome};
use crate::game::Game;
use crate::ids::*;

/// kept in the rule's own module for now
pub type Row = ();

pub fn transition(g: &mut Game, gd: GuardId, _dt: f64) { crate::frag::p_03::frag_flee_transition(g, gd); }

pub fn propose(g: &Game, gd: GuardId) -> Option<Intent> { crate::frag::p_03::frag_flee_propose(g, gd).map(Intent::Script) }

pub fn commit(g: &mut Game, gd: GuardId, o: Outcome) {
    if let Outcome::Script(ran) = o { crate::frag::frag_script_ran(g, gd, ran); }
}

pub fn lost(_: &mut Game, _: GuardId) {}
