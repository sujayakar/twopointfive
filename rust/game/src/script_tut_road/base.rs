//! What the parts of `script_tut_road` share, and what it hands to the host.
#![allow(unused_variables, unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};


/// the script's locals, as this run's define pass left them
pub fn tut_road_vars(g: &crate::game::Game, r: DirectorRunId) -> &crate::script_tut_road::TutRoadVars { match &g.tab.director_run[r].vars { crate::director::ScriptVars::TutRoad(v) => v, _ => panic!("the run is not of levels/tut-road.script.ts") } }
