//! src/game/kittree.ts lines 1 to 30
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// The switch as the card asks it: settled on the page, so always on.   (kittree.ts treeOn)
pub fn tree_on(g: &mut crate::game::Game) -> bool { true }
