#![allow(unused_variables)]
//! Every function and method of `disarm`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/disarm.ts:43 lerp)
pub fn lerp(a: f64, b: f64, t: f64) -> f64 { todo!() }

/// (src/game/disarm.ts:43 sstep)
pub fn sstep(a: f64, b: f64, t: f64) -> f64 { todo!() }

/// (src/game/disarm.ts:46 disarmBegin)
pub fn disarm_begin(g: &mut crate::game::Game) { todo!() }

/// (src/game/disarm.ts:54 disarmCatch)
pub fn disarm_catch(g: &mut crate::game::Game, at: V3) { todo!() }

/// (src/game/disarm.ts:59 leads)
pub fn leads(g: &mut crate::game::Game, s: Option<crate::disarm::Strip>) -> bool { todo!() }

/// (src/game/disarm.ts:61 lead)
pub fn lead(g: &mut crate::game::Game, s: crate::disarm::Strip) -> f64 { todo!() }

/// (src/game/disarm.ts:67 disarmPose)
pub fn disarm_pose(s: crate::disarm::Strip) -> crate::character::StripPose { todo!() }

/// (src/game/disarm.ts:69 disarmHand)
pub fn disarm_hand(g: &mut crate::game::Game, h: &crate::player::Hold) -> bool { todo!() }

/// (src/game/disarm.ts:72 disarmTick)
pub fn disarm_tick(g: &mut crate::game::Game, dt: f64) { todo!() }

/// (src/game/disarm.ts:89 handOf)
pub fn hand_of(g: &mut crate::game::Game, gd: GuardId) -> V3 { todo!() }

/// (src/game/disarm.ts:90 long)
pub fn long(kind: &str) -> bool { todo!() }

/// (src/game/disarm.ts:94 take)
pub fn take(g: &mut crate::game::Game, pl: PlayerId, s: &mut crate::disarm::Strip) { todo!() }

/// (src/game/disarm.ts:103 carry)
pub fn carry(g: &mut crate::game::Game, s: &mut crate::disarm::Strip, sam: CharId) { todo!() }

/// (src/game/disarm.ts:112 letGo)
pub fn let_go(g: &mut crate::game::Game, it: ItemId, by: CharId, fall: Option<f64>) { todo!() }

/// (src/game/disarm.ts:133 gunDown)
pub fn gun_down(g: &mut crate::game::Game, sp: f64, p: V3, it: ItemId, n: V3) { todo!() }
