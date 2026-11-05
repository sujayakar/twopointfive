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

/// AFTER AN IMPORT INTO A FRESH MODULE, before the first tick: the knobs' doubles as the TypeScript's module holds them (`knobs`: table 0, raw bytes; the
/// Rust game has no knobs_to_rust yet), every clip the state says the module holds written again in the order of its number (so the numbers come out the
/// same: checked), and the bench's rig and roles forgotten so that the first ask writes them.
pub fn bringup_module_state(g: &mut Game, knobs: &[u8]) {
    let n = (::sim::knobs::len(0) * 8).min(knobs.len());
    unsafe { core::ptr::copy_nonoverlapping(knobs.as_ptr(), ::sim::knobs::addr(0) as *mut u8, n); }
    g.m.anim_skeleton.pose_bench.rig = None;
    // the furniture's facts by row (props.ts PropSystem.facts: written into the module's hand-over once, as a level is stood up)
    if !g.props.props.is_empty() { crate::props::prop_system_facts(g); }
    g.m.anim_skeleton.layers.roles = None;
    let mut held: Vec<(f64, ClipId)> = g.m.anim_skeleton.clip_side.iter().filter(|(_, c)| c.at >= 0.0).map(|(k, c)| (c.at, k)).collect();
    held.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    for (at, clip) in held {
        let got = super::p_01::clip_write(&g.tab.clip[clip]);
        if got != Some(at) { panic!("bring-up: the clip {:?} is number {} in the state and {:?} in this module", g.tab.clip[clip].name, at, got); }
    }
}
