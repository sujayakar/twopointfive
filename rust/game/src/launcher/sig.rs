#![allow(unused_variables)]
//! Every function and method of `launcher`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/launcher.ts:57 launchVel)
pub fn launch_vel(from: V3, target: V3, r: crate::launcher::Ballistics) -> V3 { todo!() }

/// (src/game/launcher.ts:81 ringSolve)
pub fn ring_solve(g: &mut crate::game::Game, from: V3, target: V3) -> crate::throwables::ThrowSolution { todo!() }

/// (src/game/launcher.ts:83 arcSolve)
pub fn arc_solve(g: &mut crate::game::Game, from: V3, target: V3, r: crate::launcher::Ballistics) -> crate::throwables::ThrowSolution { todo!() }

/// (src/game/launcher.ts:101 ringAimPoint)
pub fn ring_aim_point(g: &mut crate::game::Game, from: V3) -> V3 { todo!() }

/// (src/game/launcher.ts:115 fireRing)
pub fn fire_ring(g: &mut crate::game::Game, shooter: CharId, from: V3, target: V3) -> crate::launcher::Ring { todo!() }

/// (src/game/launcher.ts:128 updateRings)
pub fn update_rings(g: &mut crate::game::Game, dt: f64) { todo!() }

/// (src/game/launcher.ts:164 bounceOff)
pub fn bounce_off(g: &mut crate::game::Game, v: V3, n: V3) -> V3 { todo!() }

/// (src/game/launcher.ts:166 spend)
pub fn spend(g: &mut crate::game::Game, r: crate::launcher::Ring, at: V3, vel: V3) { todo!() }

/// (src/game/launcher.ts:172 clack)
pub fn clack(g: &mut crate::game::Game, at: V3, k: Option<f64>) { todo!() }

/// (src/game/launcher.ts:181 ringHitsCharacter)
pub fn ring_hits_character(g: &mut crate::game::Game, r: crate::launcher::Ring, c_2: CharId, gd: Option<GuardId>, dir: V3, at: V3) { todo!() }

/// (src/game/launcher.ts:206 staggerGuard)
pub fn stagger_guard(g: &mut crate::game::Game, gd: GuardId, dir: V3, shooter: CharId) { todo!() }

/// (src/game/launcher.ts:227 staggerLines)
pub fn stagger_lines() -> Vec<String> { todo!() }

/// (src/game/launcher.ts:232 ringBoxes)
pub fn ring_boxes(g: &mut crate::game::Game, out: &mut Vec<BoxId>) { todo!() }

/// (src/game/launcher.ts:250 ringSummary)
pub fn ring_summary(g: &mut crate::game::Game) -> String { todo!() }
