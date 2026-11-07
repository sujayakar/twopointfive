#![allow(unused_variables)]
//! Every function and method of `sim_slabshome`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/sim/slabshome.ts:39 slabCols)
pub fn slab_cols(cap: f64) -> crate::sim_slabshome::SlabCols { todo!() }

/// (src/sim/slabshome.ts:50 SlabStore.constructor)
pub fn slab_store_new(g: &mut crate::game::Game, st: crate::Opaque, doors: f64) -> crate::sim_slabshome::SlabStore { todo!() }

/// (src/sim/slabshome.ts:55 SlabStore.n)
pub fn slab_store_n(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/sim/slabshome.ts:56 SlabStore.fly)
pub fn slab_store_fly(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/sim/slabshome.ts:58 SlabStore.rowOf)
pub fn slab_store_row_of(g: &mut crate::game::Game, door: f64) -> f64 { todo!() }

/// (src/sim/slabshome.ts:60 SlabStore.doorAt)
pub fn slab_store_door_at(g: &mut crate::game::Game, r: f64) -> f64 { todo!() }

/// (src/sim/slabshome.ts:61 SlabStore.swap)
pub fn slab_store_swap(g: &mut crate::game::Game, a: f64, b: f64) { todo!() }

/// (src/sim/slabshome.ts:68 SlabStore.enter)
pub fn slab_store_enter(g: &mut crate::game::Game, door: f64, flying: bool) -> f64 { todo!() }

/// (src/sim/slabshome.ts:78 SlabStore.rest)
pub fn slab_store_rest(g: &mut crate::game::Game, door: f64) { todo!() }

/// (src/sim/slabshome.ts:84 SlabStore.leave)
pub fn slab_store_leave(g: &mut crate::game::Game, door: f64) -> bool { todo!() }

/// (src/sim/slabshome.ts:94 SlabStore.load)
pub fn slab_store_load(g: &mut crate::game::Game, r: f64) -> crate::sim_slabshome::SlabNow { todo!() }

/// (src/sim/slabshome.ts:102 SlabStore.save)
pub fn slab_store_save(g: &mut crate::game::Game, r: f64, s: crate::sim_slabshome::SlabNow) { todo!() }

/// (src/sim/slabshome.ts:121 slabIo)
pub fn slab_io(g: &mut crate::game::Game) -> crate::sim_slabshome::SlabIo { todo!() }

/// (src/sim/slabshome.ts:138 slabDiff)
pub fn slab_diff(g: &mut crate::game::Game, a: &[u8], b: &[u8], most: Option<f64>) -> Vec<String> { todo!() }
