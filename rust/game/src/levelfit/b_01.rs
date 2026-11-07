//! Bodies for skeleton stubs of levelfit that translated code calls and no part had (the stub round).
#![allow(unused_imports, unused_variables)]
use super::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

// ---- the terms of the count: called by the bodies below, stubs with no other caller, written here so no refusal panics on an empty stub

/// Men a game holds at once, at most.   (levelfit.ts menMost)
pub fn men_most(g: &mut crate::game::Game, civilians: f64) -> f64 {
    crate::guards::MAX_SKINNED + civilians + g.k.seats.max - 1.0
}

/// Men one step can be handed: the doors' hand-over in contacts, by the contacts one man is at most, and the furniture's in men. Infinity where there is no module.   (levelfit.ts menHeld)
pub fn men_held(g: &mut crate::game::Game) -> f64 {
    if !crate::sim_sim::sim_why_not().is_empty() { return f64::INFINITY; }
    let doors = crate::sim_doorshome::door_io(g).max_contacts;
    let held = m::floor(doors / (crate::game::body_door_particles().len() as f64 + 1.0));
    m::min(held, crate::sim_propshome::prop_io(g).max_contacts)
}

// ---- the functions translated code calls

/// Civilians a game holds at once, at most: what a step's hand-over leaves after every other man a game may hold.   (levelfit.ts civMost)
pub fn civ_most(g: &mut crate::game::Game) -> f64 {
    let held = men_held(g);
    held - men_most(g, 0.0)
}

/// A level the game will not load, said by its name where it has one.   (levelfit.ts levelTooBig)
pub fn level_too_big(g: &mut crate::game::Game, why: &str) -> String {
    let name = crate::scene_level_load::level_name_of(g);
    crate::scene_box_mirror::too_big(name.as_deref(), why)
}

