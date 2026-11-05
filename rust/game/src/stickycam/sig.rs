#![allow(unused_variables)]
//! Every function and method of `stickycam`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/stickycam.ts:61 stickyStatsReset)
pub fn sticky_stats_reset(g: &mut crate::game::Game) { todo!() }

/// (src/game/stickycam.ts:69 B)
pub fn b(g: &mut crate::game::Game) -> crate::launcher::Ballistics { todo!() }

/// (src/game/stickycam.ts:77 stickyVisit)
pub fn sticky_visit(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/stickycam.ts:78 stickyVisitSet)
pub fn sticky_visit_set(g: &mut crate::game::Game, n: f64) { todo!() }

/// (src/game/stickycam.ts:83 stickyIssue)
pub fn sticky_issue(g: &mut crate::game::Game, pl: PlayerId, n: f64, listed: Option<bool>) { todo!() }

/// (src/game/stickycam.ts:93 camAimPoint)
pub fn cam_aim_point(g: &mut crate::game::Game, from: V3) -> V3 { todo!() }

/// (src/game/stickycam.ts:100 camSolve)
pub fn cam_solve(g: &mut crate::game::Game, from: V3, target: V3) -> crate::throwables::ThrowSolution { todo!() }

/// (src/game/stickycam.ts:102 camVel)
pub fn cam_vel(g: &mut crate::game::Game, from: V3, target: V3) -> V3 { todo!() }

/// (src/game/stickycam.ts:106 fireCam)
pub fn fire_cam(g: &mut crate::game::Game, pl: PlayerId, from: V3, target: V3) -> StickyCamId { todo!() }

/// (src/game/stickycam.ts:117 settle)
pub fn settle(g: &mut crate::game::Game, pl: PlayerId, cam: StickyCamId, at: V3, n: V3, si: f64) { todo!() }

/// (src/game/stickycam.ts:137 updateStickies)
pub fn update_stickies(g: &mut crate::game::Game, dt: f64) { todo!() }

/// (src/game/stickycam.ts:178 flat)
pub fn flat(g: &mut crate::game::Game, c_2: StickyCamId) -> bool { todo!() }

/// (src/game/stickycam.ts:180 camLens)
pub fn cam_lens(g: &mut crate::game::Game, c_2: StickyCamId) -> V3 { todo!() }

/// (src/game/stickycam.ts:183 stickyLive)
pub fn sticky_live(g: &mut crate::game::Game, pl: &crate::stickycam::StickyLivePl) -> Vec<StickyCamId> { todo!() }

/// (src/game/stickycam.ts:192 stickyEyes)
pub fn sticky_eyes(g: &mut crate::game::Game) -> Vec<crate::fogofwar::FogEye> { todo!() }

/// (src/game/stickycam.ts:207 canTake)
pub fn can_take(g: &mut crate::game::Game, pl: PlayerId) -> bool { todo!() }

/// (src/game/stickycam.ts:210 camNear)
pub fn cam_near(g: &mut crate::game::Game, pl: PlayerId, r: f64) -> Option<StickyCamId> { todo!() }

/// (src/game/stickycam.ts:216 handsFree)
pub fn hands_free(g: &mut crate::game::Game, pl: PlayerId) -> bool { todo!() }

/// (src/game/stickycam.ts:218 camToTake)
pub fn cam_to_take(g: &mut crate::game::Game, pl: PlayerId) -> Option<StickyCamId> { todo!() }

/// (src/game/stickycam.ts:221 stickyRows)
pub fn sticky_rows(g: &mut crate::game::Game, pl: PlayerId, busy: bool, row: &mut dyn FnMut(&mut crate::game::Game, crate::game::Interactable, crate::game::Verb, String, Option<String>, Option<crate::stickycam::StickyRowsRowOver>) -> crate::game::Interactable, offer: &mut dyn FnMut(&mut crate::game::Game, crate::game::Interactable, Vec<crate::game::Interactable>)) { todo!() }

/// (src/game/stickycam.ts:230 takeCam)
pub fn take_cam(g: &mut crate::game::Game, pl: PlayerId) -> bool { todo!() }

/// (src/game/stickycam.ts:246 stickyBoxes)
pub fn sticky_boxes(g: &mut crate::game::Game, out: &mut Vec<BoxId>) { todo!() }

/// (src/game/stickycam.ts:263 sending)
pub fn sending(g: &mut crate::game::Game) -> crate::FnField { todo!() }

/// (src/game/stickycam.ts:271 stickyMarks)
pub fn sticky_marks(g: &mut crate::game::Game) -> Vec<crate::stickycam::CamMark> { todo!() }

/// (src/game/stickycam.ts:280 stickyFlying)
pub fn sticky_flying(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/stickycam.ts:283 stickyLeft)
pub fn sticky_left(g: &mut crate::game::Game, pl: PlayerId) -> f64 { todo!() }

/// (src/game/stickycam.ts:298 stickySummary)
pub fn sticky_summary(g: &mut crate::game::Game) -> String { todo!() }
