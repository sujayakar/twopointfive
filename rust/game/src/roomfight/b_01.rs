//! THE BRING-UP'S OWN PART of roomfight: bodies for skeleton stubs a tick reached (bringup2-log.md says which is a boundary no-op and which is translated)
#![allow(unused_imports, unused_variables)]
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// A DEFENDER'S WATCHED POINT MOVES (the one place it is written in place: roomfight's parts, the enemy squads and the lights-out rule all come
/// here). What his duty has his gun on (`covers`) is that point, so it moves with it; kept as a copy, equal numbers stand for "the same point".
/// His fix is not touched: where he thinks his man is, is his own, and no rule makes the watched point his fix any more.
pub fn def_move_pt(g: &mut crate::game::Game, d: RoomfightDefId, to: V3) {
    let (gd, old) = (g.tab.roomfight_def[d].gd, g.tab.roomfight_def[d].pt);
    g.tab.roomfight_def[d].pt = to;
    if g.gd[gd].covers == Some(old) { g.gd[gd].covers = Some(to); }
}
