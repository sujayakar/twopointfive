#![allow(unused_variables)]
//! Every function and method of `wand`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/wand.ts:90 wandFits)
pub fn wand_fits(g: &mut crate::game::Game, d: DoorId) -> bool { todo!() }

/// (src/game/wand.ts:92 wandOffered)
pub fn wand_offered(g: &mut crate::game::Game, pl: PlayerId, d: DoorId) -> bool { todo!() }

/// (src/game/wand.ts:96 wandFeedSecs)
pub fn wand_feed_secs(g: &mut crate::game::Game, squat: Option<bool>) -> f64 { todo!() }

/// (src/game/wand.ts:98 wandThrough)
pub fn wand_through(g: &mut crate::game::Game, t: f64, age: f64, crouch: f64, squat: Option<bool>) -> bool { todo!() }

/// (src/game/wand.ts:103 wandFov)
pub fn wand_fov(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/wand.ts:105 wandPanMax)
pub fn wand_pan_max(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/wand.ts:108 wandLens)
pub fn wand_lens(g: &mut crate::game::Game, d: DoorId, side: f64) -> crate::wand::WandLens { todo!() }

/// (src/game/wand.ts:114 wandTurn)
pub fn wand_turn(g: &mut crate::game::Game, pl: PlayerId, aim: V3, dt: f64) { todo!() }

/// (src/game/wand.ts:121 wandEye)
pub fn wand_eye(g: &mut crate::game::Game) -> Option<crate::fogofwar::FogEye> { todo!() }

/// (src/game/wand.ts:128 wandDrop)
pub fn wand_drop(g: &mut crate::game::Game, pl: PlayerId) { todo!() }

/// (src/game/wand.ts:133 wandSummary)
pub fn wand_summary(g: &mut crate::game::Game) -> String { todo!() }
