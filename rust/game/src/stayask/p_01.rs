//! src/game/stayask.ts lines 1 to 36
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// Does the rule hold on this floor: two squads or more, every one told to stay in its rooms, no lesson on the level. (stayask.ts stayAskHere)
pub fn stay_ask_here(g: &mut crate::game::Game, squads: &[crate::scene_scenarios::SquadData]) -> bool {
    squads.len() > 1
        && squads.iter().all(|q| q.may_leave == Some(false))
        && !g.level.scenarios.as_ref().is_some_and(|s| s.lesson.is_some())
}
