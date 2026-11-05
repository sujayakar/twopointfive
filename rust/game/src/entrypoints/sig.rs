#![allow(unused_variables)]
//! Every function and method of `entrypoints`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/entrypoints.ts:77 EntryPoints.constructor)
pub fn entry_points_new(g: &mut crate::game::Game) -> crate::entrypoints::EntryPoints { todo!() }

/// (src/game/entrypoints.ts:80 EntryPoints.reset)
pub fn entry_points_reset(g: &mut crate::game::Game) { todo!() }

/// (src/game/entrypoints.ts:83 EntryPoints.openings)
pub fn entry_points_openings(g: &mut crate::game::Game) -> Vec<CoverOpeningId> { todo!() }

/// (src/game/entrypoints.ts:87 EntryPoints.tick)
pub fn entry_points_tick(g: &mut crate::game::Game) { todo!() }

/// (src/game/entrypoints.ts:94 EntryPoints.mark)
pub fn entry_points_mark(g: &mut crate::game::Game, p: V3) { todo!() }

/// (src/game/entrypoints.ts:106 EntryPoints.sees)
pub fn entry_points_sees(g: &mut crate::game::Game, p: V3, o: CoverOpeningId, sg: f64, open: bool) -> bool { todo!() }

/// (src/game/entrypoints.ts:112 EntryPoints.farKnown)
pub fn entry_points_far_known(g: &mut crate::game::Game, o: CoverOpeningId, p: V3) -> bool { todo!() }

/// (src/game/entrypoints.ts:116 EntryPoints.riskAt)
pub fn entry_points_risk_at(g: &mut crate::game::Game, p: V3, skip: Option<DoorId>, gaps: Option<bool>) -> Option<CoverOpeningId> { todo!() }

/// (src/game/entrypoints.ts:138 EntryPoints.clearOf)
pub fn entry_points_clear_of(g: &mut crate::game::Game, d: DoorId, pods: &[crate::entrypoints::Pod], frame: &mut dyn FnMut(&mut crate::game::Game, V3) -> [f64; 2]) -> Vec<crate::entrypoints::Pod> { todo!() }

/// (src/game/entrypoints.ts:170 EntryPoints.swapped)
pub fn entry_points_swapped(g: &mut crate::game::Game, pod: crate::entrypoints::Pod) { todo!() }

/// (src/game/entrypoints.ts:172 EntryPoints.dealt)
pub fn entry_points_dealt(g: &mut crate::game::Game, pod: crate::entrypoints::Pod, skip: Option<DoorId>) { todo!() }

/// (src/game/entrypoints.ts:174 EntryPoints.summary)
pub fn entry_points_summary(g: &mut crate::game::Game) -> String { todo!() }
