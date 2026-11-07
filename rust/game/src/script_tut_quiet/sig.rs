//! The bodies of levels/tut-quiet.script.ts: its define pass, and a step function for each beat. Written by the skeleton's program.
#![allow(unused_variables, unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (levels/tut-quiet.script.ts: `script(D)`) the define pass: everything the script names and declares, through director::manifest_* and program_*. Its locals a beat reads go in the answer.
pub fn tut_quiet_define(m: &mut crate::director::Manifest) -> Result<crate::script_tut_quiet::TutQuietVars, String> { todo!() }

/// the script's locals, as this run's define pass left them
pub fn tut_quiet_vars(g: &crate::game::Game, r: DirectorRunId) -> &crate::script_tut_quiet::TutQuietVars { match &g.tab.director_run[r].vars { crate::director::ScriptVars::TutQuiet(v) => v, _ => panic!("the run is not of levels/tut-quiet.script.ts") } }

/// (levels/tut-quiet.script.ts:34-43 P.beat 'lane') 3 yields, a straight line. One resume: `got` is what the last yield handed back; answer the next yield's Cond, or None when it is done, or Err as a throw.
pub fn tut_quiet_lane_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_quiet::TutQuietLaneFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> { todo!() }

/// (levels/tut-quiet.script.ts:46-93 P.beat 'night') 6 yields, 6 in a loop, 3 in a branch. Its closures are private fns: taken, gone, inStrip, met, lit, heard, turns. One resume: `got` is what the last yield handed back; answer the next yield's Cond, or None when it is done, or Err as a throw.
pub fn tut_quiet_night_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_quiet::TutQuietNightFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> { todo!() }

/// (levels/tut-quiet.script.ts:46-93) its 6 c.until closures, by an id of your own (CondTest::Script { id }); 3 of them read or write the frame
pub fn tut_quiet_night_test(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_quiet::TutQuietNightFrame, id: u32) -> Result<bool, String> { todo!() }

/// (levels/tut-quiet.script.ts:96-102 P.beat 'store') 2 yields, a straight line. One resume: `got` is what the last yield handed back; answer the next yield's Cond, or None when it is done, or Err as a throw.
pub fn tut_quiet_store_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_quiet::TutQuietStoreFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> { todo!() }
