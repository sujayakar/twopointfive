//! THE BRING-UP'S OWN PART of deathrecap: bodies for skeleton stubs the first tick reached (bringup-log.md says which is a boundary no-op and which is translated)
#![allow(unused_imports, unused_variables)]
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/deathrecap.ts:109 eachMan)
pub fn each_man(g: &mut crate::game::Game, fn_: &mut dyn FnMut(&mut crate::game::Game, CharId, bool, Option<GuardId>)) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/deathrecap.ts:125 recapRound)
pub fn recap_round(g: &mut crate::game::Game, victim: Option<CharId>, was_down: bool, shooter: CharId, muzzle: V3, hit: V3, def: &crate::weapons::WeaponDef, blind: Option<bool>) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/deathrecap.ts:169 sample)
pub fn sample(g: &mut crate::game::Game) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/deathrecap.ts:185 take)
pub fn take(g: &mut crate::game::Game, death: &crate::deathrecap::RecapDeath) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/deathrecap.ts:202 flush)
pub fn flush(g: &mut crate::game::Game, early: bool) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/deathrecap.ts:207 recapTick)
pub fn recap_tick(g: &mut crate::game::Game) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/deathrecap.ts:297 witness)
pub fn witness(g: &mut crate::game::Game) {}
