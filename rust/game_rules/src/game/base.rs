//! What the parts of `game` share, and what it hands to the host.
#![allow(unused_variables)]
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};


/// INVENTED: THE whole write of a man's path. Every `gd.path = x` of the TypeScript (a new array, even `[]`) is this call: identity readers (doorpath.ts:336-337, doorway.ts:328, guards.ts:2037) compare path_serial. An in-place edit (splice, push, index, `length = n`) is not a whole write and does not call it. ONE exception: runner.ts:294 puts the same array back, so it restores path and path_serial together and does not call this
pub fn guard_set_path(g: &mut crate::game::Game, gd: GuardId, path: Vec<V3>) { let o = &mut g.gd[gd]; o.path = path; o.path_serial = o.path_serial.wrapping_add(1); }

/// A game stands in this slot of rust/sim's: its own word says so, and the box mirror's, which the world's lookups read.
pub fn stand_in(g: &mut crate::game::Game, slot: usize) { g.sim = slot; g.col.mirror.slot = slot as f64; }
