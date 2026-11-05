//! THE BRING-UP'S OWN PART of told: bodies for skeleton stubs the first tick reached (bringup-log.md says which is a boundary no-op and which is translated)
#![allow(unused_imports, unused_variables)]
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

// TRANSLATED BY THE BRING-UP (src/game/told.ts, which no part had): the fourth lever's questions. `TOLD.rung` is a ToldRung in the skeleton, so isRung's guard
// against a stray string has nothing to catch.

/// the rungs, the most told first (told.ts TOLD_RUNGS)
fn rung_ix(r: ToldRung) -> usize { match r { ToldRung::Guided => 0, ToldRung::Standard => 1, ToldRung::Bare => 2 } }

/// The rung in force: Bare while the lever is off.   (told.ts toldNow)
pub fn told_now(g: &mut crate::game::Game) -> crate::told::ToldRung {
    if g.k.told.on { g.k.told.rung } else { ToldRung::Bare }
}

/// Is the game telling at least as much as `rung`?   (told.ts toldAtLeast)
pub fn told_at_least(g: &mut crate::game::Game, rung: crate::told::ToldRung) -> bool {
    rung_ix(told_now(g)) <= rung_ix(rung)
}

/// Seconds a last-seen mark stays.   (told.ts toldLastSeen)
pub fn told_last_seen(g: &mut crate::game::Game) -> f64 {
    match told_now(g) {
        ToldRung::Guided => m::max(0.0, g.k.told.guided),
        ToldRung::Standard => m::max(0.0, g.k.told.standard),
        ToldRung::Bare => 0.0,
    }
}

/// The rung a level puts the lever on.   (told.ts toldOfLevel)
pub fn told_of_level(level: crate::difficulty::Difficulty) -> crate::told::ToldRung {
    if level == crate::difficulty::Difficulty::Hard { ToldRung::Bare } else { ToldRung::Standard }
}
