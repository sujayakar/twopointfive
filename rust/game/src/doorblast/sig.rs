#![allow(unused_variables)]
//! Every function and method of `doorblast`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/doorblast.ts:71 note)
pub fn note(g: &mut crate::game::Game, s: &str) { todo!() }

/// (src/game/doorblast.ts:116 support)
pub fn support(g: &mut crate::game::Game, d: DoorId, tip: f64) -> f64 { todo!() }

/// (src/game/doorblast.ts:123 hitAt)
pub fn hit_at(g: &mut crate::game::Game, x: f64, y: f64, z: f64, ex: f64, ez: f64, tmax: f64) -> Option<crate::doorblast::HitAt> { todo!() }

/// (src/game/doorblast.ts:126 withoutLeaf)
pub fn without_leaf<T: Clone + core::fmt::Debug>(g: &mut crate::game::Game, d: DoorId, fn_: &mut dyn FnMut(&mut crate::game::Game) -> T) -> T { todo!() }

/// (src/game/doorblast.ts:136 overlaps)
pub fn overlaps(g: &mut crate::game::Game, x: f64, z: f64, a: f64, hu: f64, hn: f64, y0: f64, y1: f64) -> bool { todo!() }

/// (src/game/doorblast.ts:151 freeSpot)
pub fn free_spot(g: &mut crate::game::Game, x: f64, z: f64, a: f64, hu: f64, hn: f64, y0: f64, y1: f64, max_r: f64) -> Option<[f64; 2]> { todo!() }

/// (src/game/doorblast.ts:161 nothingMet)
pub fn nothing_met() -> crate::doorblast::SlabMet { todo!() }

/// (src/game/doorblast.ts:186 doorBlast)
pub fn door_blast(g: &mut crate::game::Game, at: V3, tear_m: f64, who: f64) -> Vec<DoorId> { todo!() }

/// (src/game/doorblast.ts:194 doorBlastReach)
pub fn door_blast_reach(g: &mut crate::game::Game, at: V3, tear_m: f64) -> Vec<crate::doorblast::DoorBlastReachItem> { todo!() }

/// (src/game/doorblast.ts:208 reaches)
pub fn reaches(g: &mut crate::game::Game, d: DoorId, at: V3) -> bool { todo!() }

/// (src/game/doorblast.ts:224 handsOff)
pub fn hands_off(g: &mut crate::game::Game, d: DoorId) { todo!() }

/// (src/game/doorblast.ts:235 tearOff)
pub fn tear_off(g: &mut crate::game::Game, d: DoorId, from: V3, who: f64, k: Option<f64>) -> bool { todo!() }

/// (src/game/doorblast.ts:263 leafTough)
pub fn leaf_tough(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/doorblast.ts:267 leafRound)
pub fn leaf_round(g: &mut crate::game::Game, index: f64, who: f64, hit_p: V3, dir: V3, w: f64) { todo!() }

/// (src/game/doorblast.ts:280 splinter)
pub fn splinter(g: &mut crate::game::Game, d: DoorId, dir: V3, who: f64, at: Option<V3>) -> bool { todo!() }

/// (src/game/doorblast.ts:331 doorSlabs)
pub fn door_slabs(g: &mut crate::game::Game, dt: f64) { todo!() }

/// (src/game/doorblast.ts:340 flySlab)
pub fn fly_slab(g: &mut crate::game::Game, d: DoorId, dt: f64) { todo!() }

/// (src/game/doorblast.ts:352 stepHere) TWIN → sim::slabs::step(g, w) -> u32 (SLAB_STEPPED, or SLAB_HOST)
pub fn step_here(g: &mut crate::game::Game, d: DoorId, r: f64, dt: f64) -> crate::doorblast::SlabMet { todo!() }

/// (src/game/doorblast.ts:360 slabThere)
pub fn slab_there(g: &mut crate::game::Game, d: DoorId) -> Option<crate::doorblast::SlabThere> { todo!() }

/// (src/game/doorblast.ts:370 stepThere)
pub fn step_there(g: &mut crate::game::Game, d: DoorId, r: f64, dt: f64, w: crate::doorblast::StepThereW) -> crate::doorblast::SlabMet { todo!() }

/// (src/game/doorblast.ts:396 stepSlab) TWIN → sim::slabs::step(g, w) -> u32
pub fn step_slab(g: &mut crate::game::Game, d: DoorId, s: &mut crate::sim_slabshome::SlabNow, dt: f64, t: &mut crate::doorblast::SlabMet) { todo!() }

/// (src/game/doorblast.ts:448 poseSlab)
pub fn pose_slab(g: &mut crate::game::Game, d: DoorId) { todo!() }

/// (src/game/doorblast.ts:469 runOn)
pub fn run_on(g: &mut crate::game::Game, d: DoorId) -> crate::sim_slabshome::SlabNow { todo!() }

/// (src/game/doorblast.ts:507 doorBlastSummary)
pub fn door_blast_summary(g: &mut crate::game::Game) -> String { todo!() }
