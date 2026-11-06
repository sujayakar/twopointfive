//! The bodies of levels/tut-kit.script.ts: its define pass, and a step function for each beat. Written by the skeleton's program.
#![allow(unused_variables, unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (levels/tut-kit.script.ts: `script(D)`) the define pass: everything the script names and declares, through director::manifest_* and program_*. Its locals a beat reads go in the answer.
pub fn tut_kit_define(m: &mut crate::director::Manifest) -> Result<crate::script_tut_kit::TutKitVars, String> { todo!() }

/// the script's locals, as this run's define pass left them
pub fn tut_kit_vars(g: &crate::game::Game, r: DirectorRunId) -> &crate::script_tut_kit::TutKitVars { match &g.tab.director_run[r].vars { crate::director::ScriptVars::TutKit(v) => v, _ => panic!("the run is not of levels/tut-kit.script.ts") } }

/// (levels/tut-kit.script.ts:71-78 P.beat 'hold') 2 yields, 1 in a branch. One resume: `got` is what the last yield handed back; answer the next yield's Cond, or None when it is done, or Err as a throw.
pub fn tut_kit_hold_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_kit::TutKitHoldFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> { todo!() }

/// (levels/tut-kit.script.ts:81-101 P.beat 'gun') 7 yields, 6 in a loop, 6 in a branch. One resume: `got` is what the last yield handed back; answer the next yield's Cond, or None when it is done, or Err as a throw.
pub fn tut_kit_gun_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_kit::TutKitGunFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> { todo!() }

/// (levels/tut-kit.script.ts:102-102 P.beat 'bags') 1 yields, a straight line. One resume: `got` is what the last yield handed back; answer the next yield's Cond, or None when it is done, or Err as a throw.
pub fn tut_kit_bags_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_kit::TutKitBagsFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> { todo!() }

/// (levels/tut-kit.script.ts:103-103 P.beat 'gap') 1 yields, a straight line. One resume: `got` is what the last yield handed back; answer the next yield's Cond, or None when it is done, or Err as a throw.
pub fn tut_kit_gap_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_kit::TutKitGapFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> { todo!() }

/// (levels/tut-kit.script.ts:104-104 P.beat 'blind') 1 yields, a straight line. One resume: `got` is what the last yield handed back; answer the next yield's Cond, or None when it is done, or Err as a throw.
pub fn tut_kit_blind_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_kit::TutKitBlindFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> { todo!() }

/// (levels/tut-kit.script.ts:108-114 P.beat 'shop') 2 yields, 1 in a branch. Its closures are private fns: down. One resume: `got` is what the last yield handed back; answer the next yield's Cond, or None when it is done, or Err as a throw.
pub fn tut_kit_shop_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_kit::TutKitShopFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> { todo!() }

/// (levels/tut-kit.script.ts:108-114) its 2 c.until closures, by an id of your own (CondTest::Script { id }); 1 of them read or write the frame
pub fn tut_kit_shop_test(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_kit::TutKitShopFrame, id: u32) -> Result<bool, String> { todo!() }

/// (levels/tut-kit.script.ts:118-126 P.beat 'room') 4 yields, 3 in a branch. Its closures are private fns: hit, inside. One resume: `got` is what the last yield handed back; answer the next yield's Cond, or None when it is done, or Err as a throw.
pub fn tut_kit_room_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_kit::TutKitRoomFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> { todo!() }

/// (levels/tut-kit.script.ts:118-126) its 2 c.until closures, by an id of your own (CondTest::Script { id }); 1 of them read or write the frame
pub fn tut_kit_room_test(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_kit::TutKitRoomFrame, id: u32) -> Result<bool, String> { todo!() }

/// (levels/tut-kit.script.ts:131-131 P.beat 'loose') 1 yields, a straight line. One resume: `got` is what the last yield handed back; answer the next yield's Cond, or None when it is done, or Err as a throw.
pub fn tut_kit_loose_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_kit::TutKitLooseFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> { todo!() }

/// (levels/tut-kit.script.ts:132-150 P.beat 'trucks') 5 yields, 2 in a loop. Its closures are private fns: up. One resume: `got` is what the last yield handed back; answer the next yield's Cond, or None when it is done, or Err as a throw.
pub fn tut_kit_trucks_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_kit::TutKitTrucksFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> { todo!() }

/// (levels/tut-kit.script.ts:132-150) its 3 c.until closures, by an id of your own (CondTest::Script { id }); 3 of them read or write the frame
pub fn tut_kit_trucks_test(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_kit::TutKitTrucksFrame, id: u32) -> Result<bool, String> { todo!() }

/// (levels/tut-kit.script.ts:153-153 P.beat 'call') 1 yields, a straight line. One resume: `got` is what the last yield handed back; answer the next yield's Cond, or None when it is done, or Err as a throw.
pub fn tut_kit_call_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_kit::TutKitCallFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> { todo!() }

/// (levels/tut-kit.script.ts:154-154 P.beat 'box') 1 yields, a straight line. One resume: `got` is what the last yield handed back; answer the next yield's Cond, or None when it is done, or Err as a throw.
pub fn tut_kit_box_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_kit::TutKitBoxFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> { todo!() }

/// (levels/tut-kit.script.ts:156-156 P.beat 'dark') 1 yields, a straight line. One resume: `got` is what the last yield handed back; answer the next yield's Cond, or None when it is done, or Err as a throw.
pub fn tut_kit_dark_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_kit::TutKitDarkFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> { todo!() }

/// (levels/tut-kit.script.ts:157-157 P.beat 'bunk') 1 yields, a straight line. One resume: `got` is what the last yield handed back; answer the next yield's Cond, or None when it is done, or Err as a throw.
pub fn tut_kit_bunk_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_kit::TutKitBunkFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> { todo!() }

/// (levels/tut-kit.script.ts:158-158 P.beat 'wall') 1 yields, a straight line. One resume: `got` is what the last yield handed back; answer the next yield's Cond, or None when it is done, or Err as a throw.
pub fn tut_kit_wall_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_kit::TutKitWallFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> { todo!() }

/// (levels/tut-kit.script.ts:160-160 P.beat 'bunkroom') 1 yields, yield* room. One resume: `got` is what the last yield handed back; answer the next yield's Cond, or None when it is done, or Err as a throw.
pub fn tut_kit_bunkroom_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_kit::TutKitBunkroomFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> { todo!() }

/// (levels/tut-kit.script.ts:161-161 P.beat 'fuelroom') 1 yields, yield* room. One resume: `got` is what the last yield handed back; answer the next yield's Cond, or None when it is done, or Err as a throw.
pub fn tut_kit_fuelroom_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_kit::TutKitFuelroomFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> { todo!() }

/// (levels/tut-kit.script.ts:166-166 P.beat 'free') 1 yields, a straight line. One resume: `got` is what the last yield handed back; answer the next yield's Cond, or None when it is done, or Err as a throw.
pub fn tut_kit_free_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_kit::TutKitFreeFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> { todo!() }

/// (levels/tut-kit.script.ts:167-170 P.beat 'seen') 1 yields, 1 in a branch. One resume: `got` is what the last yield handed back; answer the next yield's Cond, or None when it is done, or Err as a throw.
pub fn tut_kit_seen_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_kit::TutKitSeenFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> { todo!() }

/// (levels/tut-kit.script.ts:171-185 P.beat 'yard') 4 yields, a straight line. Its closures are private fns: hit. One resume: `got` is what the last yield handed back; answer the next yield's Cond, or None when it is done, or Err as a throw.
pub fn tut_kit_yard_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_kit::TutKitYardFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> { todo!() }

/// (levels/tut-kit.script.ts:171-185) its 3 c.until closures, by an id of your own (CondTest::Script { id }); 2 of them read or write the frame
pub fn tut_kit_yard_test(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_kit::TutKitYardFrame, id: u32) -> Result<bool, String> { todo!() }

/// (levels/tut-kit.script.ts:55-63 function* room) 3 yields, 1 in a branch. Its closures are private fns: hit, inRoom. One resume: `got` is what the last yield handed back; answer the next yield's Cond, or None when it is done, or Err as a throw.
pub fn tut_kit_room_helper_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_kit::TutKitRoomHelperFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> { todo!() }

/// (levels/tut-kit.script.ts:55-63) its 2 c.until closures, by an id of your own (CondTest::Script { id }); 1 of them read or write the frame
pub fn tut_kit_room_helper_test(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_kit::TutKitRoomHelperFrame, id: u32) -> Result<bool, String> { todo!() }
