//! The bodies of levels/tut-pair.script.ts: its define pass, and a step function for each beat. Written by the skeleton's program.
#![allow(unused_variables, unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (levels/tut-pair.script.ts: `script(D)`) the define pass: everything the script names and declares, through director::manifest_* and program_*. Its locals a beat reads go in the answer.
pub fn tut_pair_define(m: &mut crate::director::Manifest) -> Result<crate::script_tut_pair::TutPairVars, String> { todo!() }

/// the script's locals, as this run's define pass left them
pub fn tut_pair_vars(g: &crate::game::Game, r: DirectorRunId) -> &crate::script_tut_pair::TutPairVars { match &g.tab.director_run[r].vars { crate::director::ScriptVars::TutPair(v) => v, _ => panic!("the run is not of levels/tut-pair.script.ts") } }

/// (levels/tut-pair.script.ts:18-24 P.beat 'gate') 2 yields, a straight line. One resume: `got` is what the last yield handed back; answer the next yield's Cond, or None when it is done, or Err as a throw.
pub fn tut_pair_gate_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_pair::TutPairGateFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> { todo!() }

/// (levels/tut-pair.script.ts:27-33 P.beat 'park') 2 yields, a straight line. One resume: `got` is what the last yield handed back; answer the next yield's Cond, or None when it is done, or Err as a throw.
pub fn tut_pair_park_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_pair::TutPairParkFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> { todo!() }

/// (levels/tut-pair.script.ts:36-39 P.beat 'yard') 1 yields, a straight line. One resume: `got` is what the last yield handed back; answer the next yield's Cond, or None when it is done, or Err as a throw.
pub fn tut_pair_yard_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_pair::TutPairYardFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> { todo!() }
