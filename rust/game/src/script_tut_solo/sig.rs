//! The bodies of levels/tut-solo.script.ts: its define pass, and a step function for each beat. Written by the skeleton's program.
#![allow(unused_variables, unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (levels/tut-solo.script.ts: `script(D)`) the define pass: everything the script names and declares, through director::manifest_* and program_*. Its locals a beat reads go in the answer.
pub fn tut_solo_define(m: &mut crate::director::Manifest) -> Result<crate::script_tut_solo::TutSoloVars, String> { todo!() }

/// the script's locals, as this run's define pass left them
pub fn tut_solo_vars(g: &crate::game::Game, r: DirectorRunId) -> &crate::script_tut_solo::TutSoloVars { match &g.tab.director_run[r].vars { crate::director::ScriptVars::TutSolo(v) => v, _ => panic!("the run is not of levels/tut-solo.script.ts") } }

/// (levels/tut-solo.script.ts:52-55 P.beat 'hold') 1 yields, a straight line. One resume: `got` is what the last yield handed back; answer the next yield's Cond, or None when it is done, or Err as a throw.
pub fn tut_solo_hold_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_solo::TutSoloHoldFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> { todo!() }

/// (levels/tut-solo.script.ts:59-59 P.beat 'cease') 1 yields, a straight line. One resume: `got` is what the last yield handed back; answer the next yield's Cond, or None when it is done, or Err as a throw.
pub fn tut_solo_cease_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_solo::TutSoloCeaseFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> { todo!() }

/// (levels/tut-solo.script.ts:60-73 P.beat 'road') 5 yields, 3 in a loop, 1 in a branch. Its closures are private fns: lane. One resume: `got` is what the last yield handed back; answer the next yield's Cond, or None when it is done, or Err as a throw.
pub fn tut_solo_road_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_solo::TutSoloRoadFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> { todo!() }

/// (levels/tut-solo.script.ts:60-73) its 1 c.until closures, by an id of your own (CondTest::Script { id }); 1 of them read or write the frame
pub fn tut_solo_road_test(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_solo::TutSoloRoadFrame, id: u32) -> Result<bool, String> { todo!() }

/// (levels/tut-solo.script.ts:77-77 P.beat 'way') 1 yields, a straight line. One resume: `got` is what the last yield handed back; answer the next yield's Cond, or None when it is done, or Err as a throw.
pub fn tut_solo_way_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_solo::TutSoloWayFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> { todo!() }

/// (levels/tut-solo.script.ts:80-100 P.beat 'gun') 7 yields, 6 in a loop, 6 in a branch. One resume: `got` is what the last yield handed back; answer the next yield's Cond, or None when it is done, or Err as a throw.
pub fn tut_solo_gun_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_solo::TutSoloGunFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> { todo!() }

/// (levels/tut-solo.script.ts:101-101 P.beat 'bags') 1 yields, a straight line. One resume: `got` is what the last yield handed back; answer the next yield's Cond, or None when it is done, or Err as a throw.
pub fn tut_solo_bags_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_solo::TutSoloBagsFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> { todo!() }

/// (levels/tut-solo.script.ts:102-102 P.beat 'gap') 1 yields, a straight line. One resume: `got` is what the last yield handed back; answer the next yield's Cond, or None when it is done, or Err as a throw.
pub fn tut_solo_gap_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_solo::TutSoloGapFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> { todo!() }

/// (levels/tut-solo.script.ts:103-103 P.beat 'blind') 1 yields, a straight line. One resume: `got` is what the last yield handed back; answer the next yield's Cond, or None when it is done, or Err as a throw.
pub fn tut_solo_blind_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_solo::TutSoloBlindFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> { todo!() }

/// (levels/tut-solo.script.ts:107-113 P.beat 'shop') 2 yields, 1 in a branch. Its closures are private fns: down. One resume: `got` is what the last yield handed back; answer the next yield's Cond, or None when it is done, or Err as a throw.
pub fn tut_solo_shop_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_solo::TutSoloShopFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> { todo!() }

/// (levels/tut-solo.script.ts:107-113) its 2 c.until closures, by an id of your own (CondTest::Script { id }); 1 of them read or write the frame
pub fn tut_solo_shop_test(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_solo::TutSoloShopFrame, id: u32) -> Result<bool, String> { todo!() }

/// (levels/tut-solo.script.ts:118-126 P.beat 'room') 4 yields, 3 in a branch. Its closures are private fns: hit, inside. One resume: `got` is what the last yield handed back; answer the next yield's Cond, or None when it is done, or Err as a throw.
pub fn tut_solo_room_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_solo::TutSoloRoomFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> { todo!() }

/// (levels/tut-solo.script.ts:118-126) its 2 c.until closures, by an id of your own (CondTest::Script { id }); 1 of them read or write the frame
pub fn tut_solo_room_test(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_solo::TutSoloRoomFrame, id: u32) -> Result<bool, String> { todo!() }

/// (levels/tut-solo.script.ts:131-131 P.beat 'loose') 1 yields, a straight line. One resume: `got` is what the last yield handed back; answer the next yield's Cond, or None when it is done, or Err as a throw.
pub fn tut_solo_loose_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_solo::TutSoloLooseFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> { todo!() }

/// (levels/tut-solo.script.ts:132-150 P.beat 'trucks') 5 yields, 2 in a loop. Its closures are private fns: up. One resume: `got` is what the last yield handed back; answer the next yield's Cond, or None when it is done, or Err as a throw.
pub fn tut_solo_trucks_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_solo::TutSoloTrucksFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> { todo!() }

/// (levels/tut-solo.script.ts:132-150) its 3 c.until closures, by an id of your own (CondTest::Script { id }); 3 of them read or write the frame
pub fn tut_solo_trucks_test(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_solo::TutSoloTrucksFrame, id: u32) -> Result<bool, String> { todo!() }
