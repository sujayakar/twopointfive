#![allow(unused_variables)]
//! Every function and method of `irlaser`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/irlaser.ts:70 irViewer)
pub fn ir_viewer(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/irlaser.ts:74 irViewAs)
pub fn ir_view_as<T: Clone + core::fmt::Debug>(g: &mut crate::game::Game, tube: bool, fn_: &mut dyn FnMut(&mut crate::game::Game) -> T) -> T { todo!() }

/// (src/game/irlaser.ts:78 irNote)
pub fn ir_note(g: &mut crate::game::Game, b: BoxId) -> BoxId { todo!() }

/// (src/game/irlaser.ts:79 irMade)
pub fn ir_made(g: &mut crate::game::Game, b: BoxId) -> bool { todo!() }

/// (src/game/irlaser.ts:82 laserBand)
pub fn laser_band(g: &mut crate::game::Game, gd: GuardId) -> crate::irlaser::LaserBand { todo!() }

/// (src/game/irlaser.ts:87 samLaserOn)
pub fn sam_laser_on(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/irlaser.ts:92 samLaserBoxes)
pub fn sam_laser_boxes(g: &mut crate::game::Game, out: &mut Vec<BoxId>) { todo!() }

/// (src/game/irlaser.ts:98 laserOn)
pub fn laser_on(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/irlaser.ts:107 paintsOwn)
pub fn paints_own(g: &mut crate::game::Game, gd: GuardId, on: CharId) -> bool { todo!() }
