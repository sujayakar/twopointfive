//! What the parts of `seats` share, and what it hands to the host.
#![allow(unused_variables)]
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};


/// the row a second cast of the squad's code keeps in a table kept per view
pub const CAST_KEY: PlayerId = PlayerId(u32::MAX);

/// is this call inside a seat's view (seats.ts `rawGame(g) !== g`), and not on the bare Game. with_seat raises Game.view_depth for the length of its call
pub fn in_view(g: &crate::game::Game) -> bool { g.view_depth > 0 }
