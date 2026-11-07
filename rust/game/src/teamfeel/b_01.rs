//! Bodies for skeleton stubs of teamfeel that translated code calls and no part had (the stub round).
#![allow(unused_imports, unused_variables)]
use super::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// Read a machine's kept feel rows back over the knob tables. Headless there is no stored copy: nothing is written.   (teamfeel.ts loadTeamFeel)
/// BOUNDARY: at the swap the host reads localStorage `twopointsix.teamfeel` and applies its rows to `g.k`, under TypeScript's checks (key kept, same type as shipped, number in range).
pub fn load_team_feel(g: &mut crate::game::Game) {}
