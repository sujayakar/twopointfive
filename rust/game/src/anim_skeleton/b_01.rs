//! THE BRING-UP'S OWN PART of anim_skeleton: bodies for skeleton stubs the first tick reached (bringup-log.md says which is a boundary no-op and which is translated)
#![allow(unused_imports, unused_variables)]
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

// TEMP-BRINGUP: everything in this file goes when poses are handles (the next skeleton: `Pose { at, nodes }` over the block's pool, the bridge carrying the place).
// ---- THE BRING-UP'S STAGE: a by-value Pose asked of a layer of rust/sim.
// SKELETON-GAP: `Pose.at` / `Pose.home` (skeleton.ts:142-149). In the TypeScript a man's poses LIVE in his game's block (posehome.ts) and a layer is handed
// their places. The skeleton's Pose owns its floats (`cols: Vec<f32>`) and has no place, and the bridge carries the floats and no place. Until Pose is a
// handle on the pool, each layer call is staged the way skeleton.ts stageOpen stages a figure who is in no game: the poses it names are copied into the pool
// of a slot nobody else uses, the layer runs there, and they are copied back ("the floats are the same floats either way, so it is the same pose to the
// bit"). WHAT THIS LEAVES WRONG: the game's own block keeps the poses as the host handed them in, so its `poses` part differs from the TypeScript's after a tick.
use core::sync::atomic::{AtomicUsize, Ordering::Relaxed};

/// the slot of rust/sim the stage's pool is in: the last one (the game stands in slot 0 under the face)
pub const STAGE_SLOT: usize = ::sim::state::GAMES - 1;
const STAGE_MOST: usize = 16;
static STAGED: AtomicUsize = AtomicUsize::new(0);
#[allow(clippy::declare_interior_mutable_const)]
const Z: AtomicUsize = AtomicUsize::new(0);
/// what stands on the stage: (the pose's own floats, by the address of its Vec's store · its place · how many singles)
static REG: [AtomicUsize; STAGE_MOST * 3] = [Z; STAGE_MOST * 3];

/// `pose.at` for a layer about to run in STAGE_SLOT: the pose's floats copied onto the stage (once: the same pose named twice has one place), and where.
/// A pose is known by its Vec's store, so one lent out of `g` for the call is still the same pose.
pub fn stage_pose(pose: &crate::anim_skeleton::Pose) -> usize {
    let (ptr, n) = (pose.cols.as_ptr() as usize, STAGED.load(Relaxed));
    for i in 0..n { if REG[i * 3].load(Relaxed) == ptr { return REG[i * 3 + 1].load(Relaxed); } }
    if n >= STAGE_MOST { panic!("posing: the bring-up's stage holds {} poses a layer and was asked for one more", STAGE_MOST); }
    let at = ::sim::poses::new_of(STAGE_SLOT, pose.num_nodes);
    if at == u32::MAX { panic!("posing: the bring-up's stage has no room for a pose of {} nodes", pose.num_nodes); }
    let at = at as usize;
    let d = &mut ::sim::state::state(STAGE_SLOT).poses.data[at - ::sim::poses::HEAD / 4..];
    for k in 0..pose.cols.len() { ::sim::single::put(d, k, pose.cols[k] as f64); }
    REG[n * 3].store(ptr, Relaxed); REG[n * 3 + 1].store(at, Relaxed); REG[n * 3 + 2].store(pose.cols.len(), Relaxed);
    STAGED.store(n + 1, Relaxed);
    at
}

/// AFTER THE LAYER: every staged pose's floats back into the pose they came from, and the stage empty. The poses are alive: they were borrowed or in `g`
/// when they were staged, a layer's call later.
pub fn stage_back() {
    let n = STAGED.load(Relaxed);
    for i in 0..n {
        let (ptr, at, len) = (REG[i * 3].load(Relaxed) as *mut f32, REG[i * 3 + 1].load(Relaxed), REG[i * 3 + 2].load(Relaxed));
        let d = &::sim::state::state(STAGE_SLOT).poses.data[at - ::sim::poses::HEAD / 4..];
        for k in 0..len { unsafe { *ptr.add(k) = ::sim::single::get(d, k) as f32; } }
    }
    STAGED.store(0, Relaxed);
    if n > 0 { ::sim::poses::clear_of(STAGE_SLOT); }
}

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
