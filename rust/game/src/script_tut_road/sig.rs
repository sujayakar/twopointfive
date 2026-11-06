//! The bodies of levels/tut-road.script.ts: its define pass, and a step function for each beat. Written by the skeleton's program.
#![allow(unused_variables, unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (levels/tut-road.script.ts: `script(D)`) the define pass: everything the script names and declares, through director::manifest_* and program_*. Its locals a beat reads go in the answer.
pub fn tut_road_define(m: &mut crate::director::Manifest) -> Result<crate::script_tut_road::TutRoadVars, String> { todo!() }

/// the script's locals, as this run's define pass left them
pub fn tut_road_vars(g: &crate::game::Game, r: DirectorRunId) -> &crate::script_tut_road::TutRoadVars { match &g.tab.director_run[r].vars { crate::director::ScriptVars::TutRoad(v) => v, _ => panic!("the run is not of levels/tut-road.script.ts") } }

/// (levels/tut-road.script.ts:54-57 P.beat 'hold') 1 yields, a straight line. One resume: `got` is what the last yield handed back; answer the next yield's Cond, or None when it is done, or Err as a throw.
pub fn tut_road_hold_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_road::TutRoadHoldFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> { todo!() }

/// (levels/tut-road.script.ts:61-61 P.beat 'cease') 1 yields, a straight line. One resume: `got` is what the last yield handed back; answer the next yield's Cond, or None when it is done, or Err as a throw.
pub fn tut_road_cease_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_road::TutRoadCeaseFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> { todo!() }

/// (levels/tut-road.script.ts:62-77 P.beat 'road') 5 yields, 3 in a loop, 1 in a branch. Its closures are private fns: lane. One resume: `got` is what the last yield handed back; answer the next yield's Cond, or None when it is done, or Err as a throw.
pub fn tut_road_road_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_road::TutRoadRoadFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> { todo!() }

/// (levels/tut-road.script.ts:62-77) its 1 c.until closures, by an id of your own (CondTest::Script { id }); 1 of them read or write the frame
pub fn tut_road_road_test(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_road::TutRoadRoadFrame, id: u32) -> Result<bool, String> { todo!() }

/// (levels/tut-road.script.ts:81-81 P.beat 'way') 1 yields, a straight line. One resume: `got` is what the last yield handed back; answer the next yield's Cond, or None when it is done, or Err as a throw.
pub fn tut_road_way_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_road::TutRoadWayFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> { todo!() }

/// (levels/tut-road.script.ts:86-99 P.beat 'hut') 5 yields, 3 in a loop, 1 in a branch. One resume: `got` is what the last yield handed back; answer the next yield's Cond, or None when it is done, or Err as a throw.
pub fn tut_road_hut_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_road::TutRoadHutFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> { todo!() }

/// (levels/tut-road.script.ts:86-99) its 1 c.until closures, by an id of your own (CondTest::Script { id }); 1 of them read or write the frame
pub fn tut_road_hut_test(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_road::TutRoadHutFrame, id: u32) -> Result<bool, String> { todo!() }

/// (levels/tut-road.script.ts:104-123 P.beat 'gun') 5 yields, 4 in a loop, 4 in a branch. Its closures are private fns: out. One resume: `got` is what the last yield handed back; answer the next yield's Cond, or None when it is done, or Err as a throw.
pub fn tut_road_gun_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_road::TutRoadGunFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> { todo!() }

/// (levels/tut-road.script.ts:104-123) its 1 c.until closures, by an id of your own (CondTest::Script { id })
pub fn tut_road_gun_test(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_road::TutRoadGunFrame, id: u32) -> Result<bool, String> { todo!() }

/// (levels/tut-road.script.ts:124-124 P.beat 'bags') 1 yields, a straight line. One resume: `got` is what the last yield handed back; answer the next yield's Cond, or None when it is done, or Err as a throw.
pub fn tut_road_bags_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_road::TutRoadBagsFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> { todo!() }

/// (levels/tut-road.script.ts:125-125 P.beat 'ease') 1 yields, a straight line. One resume: `got` is what the last yield handed back; answer the next yield's Cond, or None when it is done, or Err as a throw.
pub fn tut_road_ease_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_road::TutRoadEaseFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> { todo!() }

/// (levels/tut-road.script.ts:130-148 P.beat 'gate') 6 yields, 3 in a loop, 1 in a branch. Its closures are private fns: up. One resume: `got` is what the last yield handed back; answer the next yield's Cond, or None when it is done, or Err as a throw.
pub fn tut_road_gate_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_road::TutRoadGateFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> { todo!() }

/// (levels/tut-road.script.ts:130-148) its 4 c.until closures, by an id of your own (CondTest::Script { id }); 4 of them read or write the frame
pub fn tut_road_gate_test(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_road::TutRoadGateFrame, id: u32) -> Result<bool, String> { todo!() }

/// (levels/tut-road.script.ts:152-152 P.beat 'open') 1 yields, a straight line. One resume: `got` is what the last yield handed back; answer the next yield's Cond, or None when it is done, or Err as a throw.
pub fn tut_road_open_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_road::TutRoadOpenFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> { todo!() }
