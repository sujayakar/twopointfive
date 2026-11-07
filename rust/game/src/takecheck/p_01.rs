//! src/game/takecheck.ts lines 1 to 23
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// Is somebody told that the thing is gone (so that a man is sent to look at its holder)?   (takecheck.ts takeTold)
pub fn take_told(g: &mut crate::game::Game) -> bool {
    if !g.quiet.active { return true; }
    let spot = crate::quiet::quiet_mission_spot(g);
    spot.map_or(false, |s| s.wired.unwrap_or(false))
}
