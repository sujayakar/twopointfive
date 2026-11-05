#![allow(unused_variables)]
//! Every function and method of `he40`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/he40.ts:45 he40StatsReset)
pub fn he40_stats_reset(g: &mut crate::game::Game) { todo!() }

/// (src/game/he40.ts:51 arcLength)
pub fn arc_length(sol: &crate::throwables::ThrowSolution) -> f64 { todo!() }

/// (src/game/he40.ts:52 arcArmed)
pub fn arc_armed(g: &mut crate::game::Game, sol: &crate::throwables::ThrowSolution) -> bool { todo!() }

/// (src/game/he40.ts:54 insideAt)
pub fn inside_at(g: &mut crate::game::Game, pos: V3) { todo!() }

/// (src/game/he40.ts:57 shellMan)
pub fn shell_man(g: &mut crate::game::Game) -> Option<GuardId> { todo!() }

/// (src/game/he40.ts:63 shellBodies)
pub fn shell_bodies(g: &mut crate::game::Game) -> Vec<CharId> { todo!() }

/// (src/game/he40.ts:74 shellSolve)
pub fn shell_solve(g: &mut crate::game::Game, from: V3, target: V3, bodies: Option<&[CharId]>) -> crate::throwables::ThrowSolution { todo!() }

/// (src/game/he40.ts:86 previewTooClose)
pub fn preview_too_close(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/he40.ts:90 shellAimPoint)
pub fn shell_aim_point(g: &mut crate::game::Game, from: V3) -> V3 { todo!() }

/// (src/game/he40.ts:102 fireShell)
pub fn fire_shell(g: &mut crate::game::Game, shooter: CharId, from: V3, target: V3) -> crate::he40::Shell { todo!() }

/// (src/game/he40.ts:111 logEnd)
pub fn log_end(g: &mut crate::game::Game, e: &crate::he40::ShellEnd) { todo!() }

/// (src/game/he40.ts:113 burst)
pub fn burst(g: &mut crate::game::Game, sh: crate::he40::Shell, pt: V3, met: crate::he40::BurstMet) { todo!() }

/// (src/game/he40.ts:118 bounceOff)
pub fn bounce_off(g: &mut crate::game::Game, v: V3, n: V3) -> V3 { todo!() }

/// (src/game/he40.ts:120 dud)
pub fn dud(g: &mut crate::game::Game, sh: crate::he40::Shell, at: V3, vel: V3, met: crate::he40::DudMet, noise: Option<f64>, who: Option<f64>) { todo!() }

/// (src/game/he40.ts:133 updateShells)
pub fn update_shells(g: &mut crate::game::Game, dt: f64) { todo!() }

/// (src/game/he40.ts:185 dudHitsCharacter)
pub fn dud_hits_character(g: &mut crate::game::Game, r: crate::he40::Shell, c_2: CharId, gd: Option<GuardId>, dir: V3, at: V3) { todo!() }

/// (src/game/he40.ts:196 shellBoxes)
pub fn shell_boxes(g: &mut crate::game::Game, out: &mut Vec<BoxId>) { todo!() }
