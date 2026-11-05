#![allow(unused_variables)]
//! Every function and method of `input`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/input.ts:42 Input.detached)
pub fn input_detached(g: &mut crate::game::Game) -> InputId { todo!() }

/// (src/game/input.ts:44 Input.constructor)
pub fn input_new(g: &mut crate::game::Game, el: Option<crate::Opaque>) -> InputId { todo!() }

/// (src/game/input.ts:91 Input.modsUp)
pub fn input_mods_up(g: &mut crate::game::Game, id: InputId, e: crate::input::InputModsUpE) { todo!() }

/// (src/game/input.ts:96 Input.modUp)
pub fn input_mod_up(g: &mut crate::game::Game, id: InputId, phys: &str) { todo!() }

/// (src/game/input.ts:98 Input.focusGame)
pub fn input_focus_game(g: &mut crate::game::Game, id: InputId) { todo!() }

/// (src/game/input.ts:99 Input.down)
pub fn input_down(g: &mut crate::game::Game, id: InputId, code: &str) -> bool { todo!() }

/// (src/game/input.ts:100 Input.hit)
pub fn input_hit(g: &mut crate::game::Game, id: InputId, code: &str) -> bool { todo!() }

/// (src/game/input.ts:101 Input.lmb)
pub fn input_lmb(g: &mut crate::game::Game, id: InputId) -> bool { todo!() }

/// (src/game/input.ts:102 Input.rmb)
pub fn input_rmb(g: &mut crate::game::Game, id: InputId) -> bool { todo!() }

/// (src/game/input.ts:103 Input.lmbHit)
pub fn input_lmb_hit(g: &mut crate::game::Game, id: InputId) -> bool { todo!() }

/// (src/game/input.ts:104 Input.rmbHit)
pub fn input_rmb_hit(g: &mut crate::game::Game, id: InputId) -> bool { todo!() }

/// (src/game/input.ts:105 Input.mmb)
pub fn input_mmb(g: &mut crate::game::Game, id: InputId) -> bool { todo!() }

/// (src/game/input.ts:106 Input.mmbHit)
pub fn input_mmb_hit(g: &mut crate::game::Game, id: InputId) -> bool { todo!() }

/// (src/game/input.ts:107 Input.endFrame)
pub fn input_end_frame(g: &mut crate::game::Game, id: InputId) { todo!() }

/// (src/game/input.ts:109 Input.endTick)
pub fn input_end_tick(g: &mut crate::game::Game, id: InputId) { todo!() }
