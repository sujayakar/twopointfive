//! The bodies of levels/tut-queue.script.ts: its define pass, and a step function for each beat. Written by the skeleton's program.
#![allow(unused_variables, unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (levels/tut-queue.script.ts: `script(D)`) the define pass: everything the script names and declares, through director::manifest_* and program_*. Its locals a beat reads go in the answer.
pub fn tut_queue_define(m: &mut crate::director::Manifest) -> Result<crate::script_tut_queue::TutQueueVars, String> { todo!() }

/// the script's locals, as this run's define pass left them
pub fn tut_queue_vars(g: &crate::game::Game, r: DirectorRunId) -> &crate::script_tut_queue::TutQueueVars { match &g.tab.director_run[r].vars { crate::director::ScriptVars::TutQueue(v) => v, _ => panic!("the run is not of levels/tut-queue.script.ts") } }

/// (levels/tut-queue.script.ts:44-82 P.beat 'gun') 8 yields, 2 in a loop, 7 in a branch. Its closures are private fns: up, too. One resume: `got` is what the last yield handed back; answer the next yield's Cond, or None when it is done, or Err as a throw.
pub fn tut_queue_gun_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_queue::TutQueueGunFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> { todo!() }

/// (levels/tut-queue.script.ts:44-82) its 5 c.until closures, by an id of your own (CondTest::Script { id }); 2 of them read or write the frame
pub fn tut_queue_gun_test(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_queue::TutQueueGunFrame, id: u32) -> Result<bool, String> { todo!() }

/// (levels/tut-queue.script.ts:88-124 P.beat 'gates') 7 yields, a straight line. Its closures are private fns: left, one, broke, walk, on. One resume: `got` is what the last yield handed back; answer the next yield's Cond, or None when it is done, or Err as a throw.
pub fn tut_queue_gates_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_queue::TutQueueGatesFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> { todo!() }

/// (levels/tut-queue.script.ts:88-124) its 3 c.until closures, by an id of your own (CondTest::Script { id }); 3 of them read or write the frame
pub fn tut_queue_gates_test(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_queue::TutQueueGatesFrame, id: u32) -> Result<bool, String> { todo!() }
