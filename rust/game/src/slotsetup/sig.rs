#![allow(unused_variables)]
//! Every function and method of `slotsetup`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/slotsetup.ts:88 setupGun)
pub fn setup_gun(g: &mut crate::game::Game, pl: PlayerId) -> Option<crate::slotsetup::SetupGun> { todo!() }

/// (src/game/slotsetup.ts:90 setupRows)
pub fn setup_rows(g: &mut crate::game::Game, pl: PlayerId, gun: crate::slotsetup::SetupGun) -> Vec<crate::slotsetup::SetupRow> { todo!() }

/// (src/game/slotsetup.ts:98 slotSetupOpen)
pub fn slot_setup_open(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/slotsetup.ts:103 slotSetupLeft)
pub fn slot_setup_left(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/slotsetup.ts:105 slotSetupShut)
pub fn slot_setup_shut(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/slotsetup.ts:107 loadKeys)
pub fn load_keys(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/slotsetup.ts:109 lightKeys)
pub fn light_keys(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/slotsetup.ts:112 slotSetupTakes)
pub fn slot_setup_takes(g: &mut crate::game::Game, code: &str) -> bool { todo!() }

/// (src/game/slotsetup.ts:118 clocked)
pub fn clocked(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/slotsetup.ts:119 lapsed)
pub fn lapsed(g: &mut crate::game::Game, t: f64) -> bool { todo!() }

/// (src/game/slotsetup.ts:122 loadSteps)
pub fn load_steps(g: &mut crate::game::Game, pl: PlayerId) -> f64 { todo!() }

/// (src/game/slotsetup.ts:124 stepRow)
pub fn step_row(g: &mut crate::game::Game, gun: crate::slotsetup::SetupGun, row: &crate::slotsetup::SetupRow) { todo!() }

/// (src/game/slotsetup.ts:133 drawPress)
pub fn draw_press(g: &mut crate::game::Game, input: InputId, in_hand: Option<crate::slotsetup::SetupGun>) { todo!() }

/// (src/game/slotsetup.ts:139 slotSetupKeys)
pub fn slot_setup_keys(g: &mut crate::game::Game, input: InputId) { todo!() }
