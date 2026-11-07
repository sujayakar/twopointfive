//! What the parts of `script_tut_team` share, and what it hands to the host.
#![allow(unused_variables, unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};


/// the script's locals, as this run's define pass left them
pub fn tut_team_vars(g: &crate::game::Game, r: DirectorRunId) -> &crate::script_tut_team::TutTeamVars { match &g.tab.director_run[r].vars { crate::director::ScriptVars::TutTeam(v) => v, _ => panic!("the run is not of levels/tut-team.script.ts") } }
