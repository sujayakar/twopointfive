//! THE BRING-UP'S OWN PART of savestate: bodies for skeleton stubs the first tick reached (bringup-log.md says which is a boundary no-op and which is translated)
#![allow(unused_imports, unused_variables)]
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/savestate.ts:408 firearmApply)
pub fn firearm_apply(g: &mut crate::game::Game, w: FirearmId, s: &crate::savestate::FirearmSnap) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/savestate.ts:471 ownChargesBack)
pub fn own_charges_back(g: &mut crate::game::Game, own: Option<&crate::savestate::OwnCharges>) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/savestate.ts:669 loadState)
pub fn load_state(g: &mut crate::game::Game, s: &crate::savestate::SaveState, opts: Option<&crate::savestate::LoadStateOpts>) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/savestate.ts:688 applyState)
pub fn apply_state(g: &mut crate::game::Game, s: &crate::savestate::SaveState, prev_loads: f64, prev_saves: f64, opts: &crate::savestate::ApplyStateOpts) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/savestate.ts:860 SaveStore.remove)
pub fn save_store_remove(g: &mut crate::game::Game, key: &str) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/savestate.ts:885 MemShelf.set)
pub fn mem_shelf_set(g: &mut crate::game::Game, key: &str, e: &crate::savestate::MemSave) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/savestate.ts:888 MemShelf.clear)
pub fn mem_shelf_clear(g: &mut crate::game::Game) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/savestate.ts:934 Checkpoints.note)
pub fn checkpoints_note(g: &mut crate::game::Game, what: &str) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/savestate.ts:936 Checkpoints.adopt)
pub fn checkpoints_adopt(g: &mut crate::game::Game, a: Option<&crate::savestate::AutosaveSnap>) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/savestate.ts:951 Checkpoints.prime)
pub fn checkpoints_prime(g: &mut crate::game::Game) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/savestate.ts:964 Checkpoints.request)
pub fn checkpoints_request(g: &mut crate::game::Game, why: &str, label: &str) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/savestate.ts:979 Checkpoints.tick)
pub fn checkpoints_tick(g: &mut crate::game::Game, dt: f64) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/savestate.ts:1051 Checkpoints.markRegret)
pub fn checkpoints_mark_regret(g: &mut crate::game::Game, key: &str, why: &str) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/game/savestate.ts:1086 Checkpoints.keep)
pub fn checkpoints_keep(g: &mut crate::game::Game, label: &str) {}
