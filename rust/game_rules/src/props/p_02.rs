//! src/game/props.ts lines 585 to 588
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

// Nothing to translate: lines 585 to 588 are the end of PropSystem and the scratch `cc`. Its only users, `step` and `circleContact`, are helpers of
// the twin sim::props::update (notInTheGame.txt), so the game holds no `cc`.
