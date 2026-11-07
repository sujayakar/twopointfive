//! Functions of `savestate` that a caller names and nobody has written: a call says so by name.
#![allow(unused_variables)]
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};


/// (src/game/savestate.ts:644 canLoad)
pub fn can_load(g: &mut crate::game::Game, s: Option<&crate::savestate::SaveState>) -> Option<String> { crate::no_body("savestate::can_load") }
