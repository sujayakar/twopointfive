//! The bodies of levels/tut-sky.script.ts: its define pass, and a step function for each beat. Written by the skeleton's program.
#![allow(unused_variables, unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (levels/tut-sky.script.ts: `script(D)`) the define pass: everything the script names and declares, through director::manifest_* and program_*. Its locals a beat reads go in the answer.
pub fn tut_sky_define(m: &mut crate::director::Manifest) -> Result<crate::script_tut_sky::TutSkyVars, String> { todo!() }

/// the script's locals, as this run's define pass left them
pub fn tut_sky_vars(g: &crate::game::Game, r: DirectorRunId) -> &crate::script_tut_sky::TutSkyVars { match &g.tab.director_run[r].vars { crate::director::ScriptVars::TutSky(v) => v, _ => panic!("the run is not of levels/tut-sky.script.ts") } }

/// (levels/tut-sky.script.ts:107-114 P.beat 'hold') 2 yields, a straight line. One resume: `got` is what the last yield handed back; answer the next yield's Cond, or None when it is done, or Err as a throw.
pub fn tut_sky_hold_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_sky::TutSkyHoldFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> { todo!() }

/// (levels/tut-sky.script.ts:107-114) its 1 c.until closures, by an id of your own (CondTest::Script { id })
pub fn tut_sky_hold_test(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_sky::TutSkyHoldFrame, id: u32) -> Result<bool, String> { todo!() }

/// (levels/tut-sky.script.ts:119-126 P.beat 'ambush') 2 yields, yield* rake. One resume: `got` is what the last yield handed back; answer the next yield's Cond, or None when it is done, or Err as a throw.
pub fn tut_sky_ambush_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_sky::TutSkyAmbushFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> { todo!() }

/// (levels/tut-sky.script.ts:127-132 P.beat 'pinned') 1 yields, 1 in a loop. One resume: `got` is what the last yield handed back; answer the next yield's Cond, or None when it is done, or Err as a throw.
pub fn tut_sky_pinned_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_sky::TutSkyPinnedFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> { todo!() }

/// (levels/tut-sky.script.ts:127-132) its 1 c.until closures, by an id of your own (CondTest::Script { id })
pub fn tut_sky_pinned_test(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_sky::TutSkyPinnedFrame, id: u32) -> Result<bool, String> { todo!() }

/// (levels/tut-sky.script.ts:142-154 P.beat 'smoke') 4 yields, 2 in a loop, 1 in a branch, yield* smoke. Its closures are private fns: smoke. One resume: `got` is what the last yield handed back; answer the next yield's Cond, or None when it is done, or Err as a throw.
pub fn tut_sky_smoke_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_sky::TutSkySmokeFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> { todo!() }

/// (levels/tut-sky.script.ts:142-154) its 1 c.until closures, by an id of your own (CondTest::Script { id })
pub fn tut_sky_smoke_test(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_sky::TutSkySmokeFrame, id: u32) -> Result<bool, String> { todo!() }

/// (levels/tut-sky.script.ts:155-163 P.beat 'break') 3 yields, a straight line. One resume: `got` is what the last yield handed back; answer the next yield's Cond, or None when it is done, or Err as a throw.
pub fn tut_sky_break_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_sky::TutSkyBreakFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> { todo!() }

/// (levels/tut-sky.script.ts:155-163) its 2 c.until closures, by an id of your own (CondTest::Script { id })
pub fn tut_sky_break_test(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_sky::TutSkyBreakFrame, id: u32) -> Result<bool, String> { todo!() }

/// (levels/tut-sky.script.ts:164-164 P.beat 'ordered') 1 yields, a straight line. One resume: `got` is what the last yield handed back; answer the next yield's Cond, or None when it is done, or Err as a throw.
pub fn tut_sky_ordered_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_sky::TutSkyOrderedFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> { todo!() }

/// (levels/tut-sky.script.ts:167-167 P.beat 'blind') 1 yields, yield* harass. One resume: `got` is what the last yield handed back; answer the next yield's Cond, or None when it is done, or Err as a throw.
pub fn tut_sky_blind_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_sky::TutSkyBlindFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> { todo!() }

/// (levels/tut-sky.script.ts:173-187 P.beat 'push') 4 yields, 3 in a loop, 1 in a branch. One resume: `got` is what the last yield handed back; answer the next yield's Cond, or None when it is done, or Err as a throw.
pub fn tut_sky_push_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_sky::TutSkyPushFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> { todo!() }

/// (levels/tut-sky.script.ts:173-187) its 4 c.until closures, by an id of your own (CondTest::Script { id }); 1 of them read or write the frame
pub fn tut_sky_push_test(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_sky::TutSkyPushFrame, id: u32) -> Result<bool, String> { todo!() }

/// (levels/tut-sky.script.ts:190-195 P.beat 'indoors') 1 yields, a straight line. One resume: `got` is what the last yield handed back; answer the next yield's Cond, or None when it is done, or Err as a throw.
pub fn tut_sky_indoors_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_sky::TutSkyIndoorsFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> { todo!() }

/// (levels/tut-sky.script.ts:198-202 P.beat 'wall') 1 yields, a straight line. One resume: `got` is what the last yield handed back; answer the next yield's Cond, or None when it is done, or Err as a throw.
pub fn tut_sky_wall_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_sky::TutSkyWallFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> { todo!() }

/// (levels/tut-sky.script.ts:203-207 P.beat 'dark') 1 yields, a straight line. One resume: `got` is what the last yield handed back; answer the next yield's Cond, or None when it is done, or Err as a throw.
pub fn tut_sky_dark_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_sky::TutSkyDarkFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> { todo!() }

/// (levels/tut-sky.script.ts:208-212 P.beat 'nv') 1 yields, a straight line. One resume: `got` is what the last yield handed back; answer the next yield's Cond, or None when it is done, or Err as a throw.
pub fn tut_sky_nv_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_sky::TutSkyNvFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> { todo!() }

/// (levels/tut-sky.script.ts:224-243 P.beat 'torches') 5 yields, 1 in a loop, 5 in a branch. Its closures are private fns: down, gone. One resume: `got` is what the last yield handed back; answer the next yield's Cond, or None when it is done, or Err as a throw.
pub fn tut_sky_torches_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_sky::TutSkyTorchesFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> { todo!() }

/// (levels/tut-sky.script.ts:224-243) its 2 c.until closures, by an id of your own (CondTest::Script { id }); 1 of them read or write the frame
pub fn tut_sky_torches_test(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_sky::TutSkyTorchesFrame, id: u32) -> Result<bool, String> { todo!() }

/// (levels/tut-sky.script.ts:244-265 P.beat 'inside') 2 yields, 1 in a loop. Its closures are private fns: taken, shut, free. One resume: `got` is what the last yield handed back; answer the next yield's Cond, or None when it is done, or Err as a throw.
pub fn tut_sky_inside_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_sky::TutSkyInsideFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> { todo!() }

/// (levels/tut-sky.script.ts:244-265) its 1 c.until closures, by an id of your own (CondTest::Script { id }); 1 of them read or write the frame
pub fn tut_sky_inside_test(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_sky::TutSkyInsideFrame, id: u32) -> Result<bool, String> { todo!() }

/// (levels/tut-sky.script.ts:266-270 P.beat 'out') 1 yields, a straight line. One resume: `got` is what the last yield handed back; answer the next yield's Cond, or None when it is done, or Err as a throw.
pub fn tut_sky_out_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_sky::TutSkyOutFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> { todo!() }

/// (levels/tut-sky.script.ts:69-85 function* rake) 4 yields, 4 in a loop, 1 in a branch. Its closures are private fns: stop, mix. One resume: `got` is what the last yield handed back; answer the next yield's Cond, or None when it is done, or Err as a throw.
pub fn tut_sky_rake_helper_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_sky::TutSkyRakeHelperFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> { todo!() }

/// (levels/tut-sky.script.ts:87-97 function* harass) 2 yields, 2 in a loop. Its closures are private fns: stop. One resume: `got` is what the last yield handed back; answer the next yield's Cond, or None when it is done, or Err as a throw.
pub fn tut_sky_harass_helper_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_sky::TutSkyHarassHelperFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> { todo!() }

/// (levels/tut-sky.script.ts:144-149 function* smoke) 3 yields, a straight line. Its closures are private fns: call, toss. One resume: `got` is what the last yield handed back; answer the next yield's Cond, or None when it is done, or Err as a throw.
pub fn tut_sky_smoke_helper_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, parent: &mut crate::script_tut_sky::TutSkySmokeFrame, f: &mut crate::script_tut_sky::TutSkySmokeHelperFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> { todo!() }
