#![allow(unused_variables)]
//! Every function and method of `cmd`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/cmd.ts:40 sampleCmd)
pub fn sample_cmd(g: &mut crate::game::Game, input: InputId, canvas: crate::host_hud::View, into: Option<&crate::cmd::PlayerCmd>) -> crate::cmd::PlayerCmd { todo!() }

/// (src/game/cmd.ts:60 cmdText)
pub fn cmd_text(c_2: &crate::cmd::PlayerCmd) -> String { todo!() }

/// (src/game/cmd.ts:61 cmdFromText)
pub fn cmd_from_text(t: &str) -> crate::cmd::PlayerCmd { todo!() }

/// (src/game/cmd.ts:63 numWord)
pub fn num_word(_k: &str, x: f64) -> Option<&'static str> { todo!() }

/// (src/game/cmd.ts:64 wordNum)
pub fn word_num(_k: &str, x: &str) -> Option<f64> { todo!() }

/// (src/game/cmd.ts:80 checkCmd)
pub fn check_cmd(g: &mut crate::game::Game, raw: Option<&crate::cmd::PlayerCmd>, near: Option<V3>) -> crate::cmd::CheckCmd { todo!() }

/// (src/game/cmd.ts:83 checkCmdIn)
pub fn check_cmd_in(g: &mut crate::game::Game, raw: Option<&crate::cmd::PlayerCmd>, near: Option<V3>) -> crate::cmd::CheckCmdIn { todo!() }

/// (src/game/cmd.ts:128 stillCmd)
pub fn still_cmd(c_2: &crate::cmd::PlayerCmd) -> crate::cmd::PlayerCmd { todo!() }

/// (src/game/cmd.ts:133 honestCopy)
pub fn honest_copy(g: &mut crate::game::Game, c_2: &crate::cmd::PlayerCmd, near: Option<V3>) -> crate::cmd::PlayerCmd { todo!() }

/// (src/game/cmd.ts:139 loadLens)
pub fn load_lens(g: &mut crate::game::Game, v: &crate::cmd::PlayerCmdView) { todo!() }

/// (src/game/cmd.ts:154 Seat.constructor)
pub fn seat_new(g: &mut crate::game::Game, index: Option<f64>) -> SeatId { todo!() }

/// (src/game/cmd.ts:156 Seat.man)
pub fn seat_man(g: &mut crate::game::Game, id: SeatId) -> PlayerId { todo!() }

/// (src/game/cmd.ts:172 Seat.load)
pub fn seat_load(g: &mut crate::game::Game, id: SeatId, c_2: &crate::cmd::PlayerCmd) { todo!() }

/// (src/game/cmd.ts:184 Seat.hand)
pub fn seat_hand(g: &mut crate::game::Game, id: SeatId, c_2: &crate::cmd::PlayerCmd) { todo!() }

/// (src/game/cmd.ts:185 Seat.handed)
pub fn seat_handed(g: &mut crate::game::Game, id: SeatId, like: &crate::cmd::PlayerCmd) -> crate::cmd::PlayerCmd { todo!() }

/// (src/game/cmd.ts:188 Seat.swallow)
pub fn seat_swallow(g: &mut crate::game::Game, id: SeatId, bits: f64) { todo!() }

/// (src/game/cmd.ts:191 Seat.takeCut)
pub fn seat_take_cut(g: &mut crate::game::Game, id: SeatId) -> Option<V3> { todo!() }
