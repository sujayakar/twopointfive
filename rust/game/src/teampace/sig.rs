#![allow(unused_variables)]
//! Every function and method of `teampace`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/teampace.ts:123 teamSlow)
pub fn team_slow(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/teampace.ts:134 walkLead)
pub fn walk_lead(g: &mut crate::game::Game, gd: GuardId, aim: V3) -> Option<V3> { todo!() }

/// (src/game/teampace.ts:144 teamWalk)
pub fn team_walk(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/teampace.ts:146 teamWalkOf)
pub fn team_walk_of(g: &mut crate::game::Game, gd: GuardId) -> f64 { todo!() }

/// (src/game/teampace.ts:157 quiet)
pub fn quiet(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/teampace.ts:158 isRun)
pub fn is_run(why: crate::teampace::IsRunWhy) -> bool { todo!() }

/// (src/game/teampace.ts:161 teamLeave)
pub fn team_leave(g: &mut crate::game::Game, gd: GuardId, why: crate::teampace::TeamLeaveWhy, top: f64, secs: Option<f64>) -> bool { todo!() }

/// (src/game/teampace.ts:169 live)
pub fn live(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::teampace::Leave> { todo!() }

/// (src/game/teampace.ts:171 fighting)
pub fn fighting(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/teampace.ts:173 blastRun)
pub fn blast_run(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/teampace.ts:176 teamCap)
pub fn team_cap(g: &mut crate::game::Game, gd: GuardId) -> f64 { todo!() }

/// (src/game/teampace.ts:186 teamAtWalk)
pub fn team_at_walk(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/teampace.ts:188 runWhy)
pub fn run_why(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::teampace::RunWhy> { todo!() }

/// (src/game/teampace.ts:190 teamRan)
pub fn team_ran(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/teampace.ts:202 teamRunAgo)
pub fn team_run_ago(g: &mut crate::game::Game, gd: GuardId) -> f64 { todo!() }

/// (src/game/teampace.ts:204 teamRunLegs)
pub fn team_run_legs(g: &mut crate::game::Game, gd: GuardId, speed: f64) -> Option<f64> { todo!() }

/// (src/game/teampace.ts:209 teamAccel)
pub fn team_accel(g: &mut crate::game::Game, gd: GuardId) -> f64 { todo!() }

/// (src/game/teampace.ts:214 teamSprinting)
pub fn team_sprinting(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/teampace.ts:217 teamTread)
pub fn team_tread(g: &mut crate::game::Game, gd: GuardId, speed: f64, base: f64) -> f64 { todo!() }

/// (src/game/teampace.ts:222 teamFollow)
pub fn team_follow(g: &mut crate::game::Game, gd: GuardId, sam_v: f64, d: f64, gain: f64) -> Option<f64> { todo!() }

/// (src/game/teampace.ts:235 teamFight)
pub fn team_fight(g: &mut crate::game::Game, gd: GuardId, secs: Option<f64>) { todo!() }

/// (src/game/teampace.ts:240 teamDashNow)
pub fn team_dash_now(g: &mut crate::game::Game, gd: GuardId, to: V3, d: f64, least: Option<f64>, running: Option<bool>) -> bool { todo!() }

/// (src/game/teampace.ts:250 teamDash)
pub fn team_dash(g: &mut crate::game::Game, gd: GuardId, to: V3, d: f64, stack: Option<bool>) -> Option<f64> { todo!() }

/// (src/game/teampace.ts:264 teamTold)
pub fn team_told(g: &mut crate::game::Game, gd: GuardId) -> Option<f64> { todo!() }

/// (src/game/teampace.ts:270 teamBlastPace)
pub fn team_blast_pace(g: &mut crate::game::Game, gd: GuardId, run: f64) -> f64 { todo!() }
