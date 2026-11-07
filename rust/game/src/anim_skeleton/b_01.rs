//! THE BRING-UP'S OWN PART of anim_skeleton: bodies for skeleton stubs the first tick reached (bringup-log.md says which is a boundary no-op and which is translated)
#![allow(unused_imports, unused_variables)]
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

// ---- THE BRING-UP'S HAND-OVER OF rust/sim's MODULE-LEVEL TABLES. A game's block is not all of rust/sim's state: the knobs' values, the clip store, the
// rig and the roles on the bench are statics of the module, written by the host as it goes, and the game's own state remembers THAT it wrote them
// (pose_bench.rig, layers.roles, clip_side[..].at). A game imported into a fresh module remembers writes this module never had.

/// HARNESS STAND-IN FOR A BRIDGE GAP (tools/blitz/README.md "Host steps"): called by the tick loop's runner after an import, never by the game.
/// AFTER AN IMPORT INTO A FRESH MODULE, before the first tick. `knobs` (table 0 of the TypeScript module's knobs, raw bytes) is only for checking
/// rustknobs::knobs_to_rust against: a tick writes them itself. The furniture's facts by row are written from the state. The clip store, the bench's rig
/// and the roles need nothing since skeleton 5.1 (the bridge leaves their marks blank, so the first use writes them).
pub fn bringup_module_state(g: &mut Game, knobs: Option<&[u8]>) {
    if let Some(knobs) = knobs {
        let n = (::sim::knobs::len(0) * 8).min(knobs.len());
        unsafe { core::ptr::copy_nonoverlapping(knobs.as_ptr(), ::sim::knobs::addr(0) as *mut u8, n); }
    }
    // BRIDGE-ASK: the bridge writes `ClipSide.at` as -1, which skeleton.ts:247 reads as ASKED AND REFUSED; not yet asked is -2 (clipIn writes the store on -2 alone)
    for (_, c) in g.m.anim_skeleton.clip_side.iter_mut() { c.at = -2.0; }
    // the furniture's facts by row (props.ts PropSystem.facts: written into the module's hand-over once, as a level is stood up)
    if !g.props.props.is_empty() { crate::props::prop_system_facts(g); }
}
