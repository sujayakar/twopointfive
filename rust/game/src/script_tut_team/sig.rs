//! The bodies of levels/tut-team.script.ts: its define pass, and a step function for each beat. Written by the skeleton's program.
#![allow(unused_variables, unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (levels/tut-team.script.ts: `script(D)`) the define pass: everything the script names and declares, through director::manifest_* and program_*. Its locals a beat reads go in the answer.
pub fn tut_team_define(m: &mut crate::director::Manifest) -> Result<crate::script_tut_team::TutTeamVars, String> { todo!() }

/// the script's locals, as this run's define pass left them
pub fn tut_team_vars(g: &crate::game::Game, r: DirectorRunId) -> &crate::script_tut_team::TutTeamVars { match &g.tab.director_run[r].vars { crate::director::ScriptVars::TutTeam(v) => v, _ => panic!("the run is not of levels/tut-team.script.ts") } }

/// (levels/tut-team.script.ts:111-119 P.beat 'hold') 2 yields, a straight line. Its closures are private fns: met. One resume: `got` is what the last yield handed back; answer the next yield's Cond, or None when it is done, or Err as a throw.
pub fn tut_team_hold_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_team::TutTeamHoldFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> { todo!() }

/// (levels/tut-team.script.ts:111-119) its 1 c.until closures, by an id of your own (CondTest::Script { id })
pub fn tut_team_hold_test(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_team::TutTeamHoldFrame, id: u32) -> Result<bool, String> { todo!() }

/// (levels/tut-team.script.ts:124-132 P.beat 'cook') 2 yields, a straight line. Its closures are private fns: met. One resume: `got` is what the last yield handed back; answer the next yield's Cond, or None when it is done, or Err as a throw.
pub fn tut_team_cook_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_team::TutTeamCookFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> { todo!() }

/// (levels/tut-team.script.ts:124-132) its 1 c.until closures, by an id of your own (CondTest::Script { id }); 1 of them read or write the frame
pub fn tut_team_cook_test(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_team::TutTeamCookFrame, id: u32) -> Result<bool, String> { todo!() }

/// (levels/tut-team.script.ts:136-143 P.beat 'ambush') 2 yields, yield* rake. One resume: `got` is what the last yield handed back; answer the next yield's Cond, or None when it is done, or Err as a throw.
pub fn tut_team_ambush_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_team::TutTeamAmbushFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> { todo!() }

/// (levels/tut-team.script.ts:144-149 P.beat 'pinned') 1 yields, 1 in a loop. One resume: `got` is what the last yield handed back; answer the next yield's Cond, or None when it is done, or Err as a throw.
pub fn tut_team_pinned_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_team::TutTeamPinnedFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> { todo!() }

/// (levels/tut-team.script.ts:144-149) its 1 c.until closures, by an id of your own (CondTest::Script { id })
pub fn tut_team_pinned_test(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_team::TutTeamPinnedFrame, id: u32) -> Result<bool, String> { todo!() }

/// (levels/tut-team.script.ts:154-168 P.beat 'smoke') 5 yields, 2 in a loop, 1 in a branch, yield* smoke. Its closures are private fns: smoke. One resume: `got` is what the last yield handed back; answer the next yield's Cond, or None when it is done, or Err as a throw.
pub fn tut_team_smoke_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_team::TutTeamSmokeFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> { todo!() }

/// (levels/tut-team.script.ts:154-168) its 1 c.until closures, by an id of your own (CondTest::Script { id })
pub fn tut_team_smoke_test(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_team::TutTeamSmokeFrame, id: u32) -> Result<bool, String> { todo!() }

/// (levels/tut-team.script.ts:169-175 P.beat 'break') 3 yields, a straight line. One resume: `got` is what the last yield handed back; answer the next yield's Cond, or None when it is done, or Err as a throw.
pub fn tut_team_break_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_team::TutTeamBreakFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> { todo!() }

/// (levels/tut-team.script.ts:169-175) its 2 c.until closures, by an id of your own (CondTest::Script { id })
pub fn tut_team_break_test(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_team::TutTeamBreakFrame, id: u32) -> Result<bool, String> { todo!() }

/// (levels/tut-team.script.ts:176-176 P.beat 'ordered') 1 yields, a straight line. One resume: `got` is what the last yield handed back; answer the next yield's Cond, or None when it is done, or Err as a throw.
pub fn tut_team_ordered_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_team::TutTeamOrderedFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> { todo!() }

/// (levels/tut-team.script.ts:178-178 P.beat 'blind') 1 yields, yield* harass. One resume: `got` is what the last yield handed back; answer the next yield's Cond, or None when it is done, or Err as a throw.
pub fn tut_team_blind_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_team::TutTeamBlindFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> { todo!() }

/// (levels/tut-team.script.ts:181-185 P.beat 'linger') 3 yields, yield* harass. One resume: `got` is what the last yield handed back; answer the next yield's Cond, or None when it is done, or Err as a throw.
pub fn tut_team_linger_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_team::TutTeamLingerFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> { todo!() }

/// (levels/tut-team.script.ts:181-185) its 2 c.until closures, by an id of your own (CondTest::Script { id })
pub fn tut_team_linger_test(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_team::TutTeamLingerFrame, id: u32) -> Result<bool, String> { todo!() }

/// (levels/tut-team.script.ts:190-205 P.beat 'push') 4 yields, 3 in a loop, 1 in a branch. One resume: `got` is what the last yield handed back; answer the next yield's Cond, or None when it is done, or Err as a throw.
pub fn tut_team_push_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_team::TutTeamPushFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> { todo!() }

/// (levels/tut-team.script.ts:190-205) its 3 c.until closures, by an id of your own (CondTest::Script { id }); 1 of them read or write the frame
pub fn tut_team_push_test(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_team::TutTeamPushFrame, id: u32) -> Result<bool, String> { todo!() }

/// (levels/tut-team.script.ts:208-213 P.beat 'indoors') 1 yields, a straight line. One resume: `got` is what the last yield handed back; answer the next yield's Cond, or None when it is done, or Err as a throw.
pub fn tut_team_indoors_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_team::TutTeamIndoorsFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> { todo!() }

/// (levels/tut-team.script.ts:216-220 P.beat 'house') 1 yields, a straight line. One resume: `got` is what the last yield handed back; answer the next yield's Cond, or None when it is done, or Err as a throw.
pub fn tut_team_house_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_team::TutTeamHouseFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> { todo!() }

/// (levels/tut-team.script.ts:221-225 P.beat 'out') 1 yields, a straight line. One resume: `got` is what the last yield handed back; answer the next yield's Cond, or None when it is done, or Err as a throw.
pub fn tut_team_out_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_team::TutTeamOutFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> { todo!() }

/// (levels/tut-team.script.ts:73-89 function* rake) 4 yields, 4 in a loop, 1 in a branch. Its closures are private fns: stop, mix. One resume: `got` is what the last yield handed back; answer the next yield's Cond, or None when it is done, or Err as a throw.
pub fn tut_team_rake_helper_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_team::TutTeamRakeHelperFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> { todo!() }

/// (levels/tut-team.script.ts:91-101 function* harass) 2 yields, 2 in a loop. Its closures are private fns: stop. One resume: `got` is what the last yield handed back; answer the next yield's Cond, or None when it is done, or Err as a throw.
pub fn tut_team_harass_helper_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_team::TutTeamHarassHelperFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> { todo!() }

/// (levels/tut-team.script.ts:156-161 function* smoke) 3 yields, a straight line. Its closures are private fns: call, toss. One resume: `got` is what the last yield handed back; answer the next yield's Cond, or None when it is done, or Err as a throw.
pub fn tut_team_smoke_helper_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, parent: &mut crate::script_tut_team::TutTeamSmokeFrame, f: &mut crate::script_tut_team::TutTeamSmokeHelperFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> { todo!() }
