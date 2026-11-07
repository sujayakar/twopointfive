//! The bodies of levels/tut-squad.script.ts: its define pass, and a step function for each beat. Written by the skeleton's program.
#![allow(unused_variables, unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (levels/tut-squad.script.ts: `script(D)`) the define pass: everything the script names and declares, through director::manifest_* and program_*. Its locals a beat reads go in the answer.
pub fn tut_squad_define(m: &mut crate::director::Manifest) -> Result<crate::script_tut_squad::TutSquadVars, String> { todo!() }

/// the script's locals, as this run's define pass left them
pub fn tut_squad_vars(g: &crate::game::Game, r: DirectorRunId) -> &crate::script_tut_squad::TutSquadVars { match &g.tab.director_run[r].vars { crate::director::ScriptVars::TutSquad(v) => v, _ => panic!("the run is not of levels/tut-squad.script.ts") } }

/// (levels/tut-squad.script.ts:75-134 P.beat 'picket') 10 yields, 3 in a loop, 2 in a branch. Its closures are private fns: up, one, broke, out, there. One resume: `got` is what the last yield handed back; answer the next yield's Cond, or None when it is done, or Err as a throw.
pub fn tut_squad_picket_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_squad::TutSquadPicketFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> { todo!() }

/// (levels/tut-squad.script.ts:75-134) its 2 c.until closures, by an id of your own (CondTest::Script { id })
pub fn tut_squad_picket_test(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_squad::TutSquadPicketFrame, id: u32) -> Result<bool, String> { todo!() }

/// (levels/tut-squad.script.ts:143-149 P.beat 'store') 2 yields, 2 in a branch. Its closures are private fns: hurt. One resume: `got` is what the last yield handed back; answer the next yield's Cond, or None when it is done, or Err as a throw.
pub fn tut_squad_store_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_squad::TutSquadStoreFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> { todo!() }

/// (levels/tut-squad.script.ts:143-149) its 1 c.until closures, by an id of your own (CondTest::Script { id }); 1 of them read or write the frame
pub fn tut_squad_store_test(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_squad::TutSquadStoreFrame, id: u32) -> Result<bool, String> { todo!() }

/// (levels/tut-squad.script.ts:161-195 P.beat 'shop') 6 yields, 3 in a loop, 5 in a branch. Its closures are private fns: down, side, room, seen, hold, ends. One resume: `got` is what the last yield handed back; answer the next yield's Cond, or None when it is done, or Err as a throw.
pub fn tut_squad_shop_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_squad::TutSquadShopFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> { todo!() }

/// (levels/tut-squad.script.ts:161-195) its 4 c.until closures, by an id of your own (CondTest::Script { id }); 2 of them read or write the frame
pub fn tut_squad_shop_test(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_squad::TutSquadShopFrame, id: u32) -> Result<bool, String> { todo!() }

/// (levels/tut-squad.script.ts:205-235 P.beat 'hall') 2 yields, 1 in a loop, 1 in a branch. Its closures are private fns: up, mine, ends, post, slot, turn, cued. One resume: `got` is what the last yield handed back; answer the next yield's Cond, or None when it is done, or Err as a throw.
pub fn tut_squad_hall_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_squad::TutSquadHallFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> { todo!() }

/// (levels/tut-squad.script.ts:205-235) its 3 c.until closures, by an id of your own (CondTest::Script { id }); 1 of them read or write the frame
pub fn tut_squad_hall_test(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_squad::TutSquadHallFrame, id: u32) -> Result<bool, String> { todo!() }

/// (levels/tut-squad.script.ts:253-300 P.beat 'gun') 11 yields, 5 in a loop, 4 in a branch. Its closures are private fns: cue, on, left, fewer, gun, gunDown, burst, up, down. One resume: `got` is what the last yield handed back; answer the next yield's Cond, or None when it is done, or Err as a throw.
pub fn tut_squad_gun_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_squad::TutSquadGunFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> { todo!() }

/// (levels/tut-squad.script.ts:253-300) its 4 c.until closures, by an id of your own (CondTest::Script { id }); 2 of them read or write the frame
pub fn tut_squad_gun_test(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_squad::TutSquadGunFrame, id: u32) -> Result<bool, String> { todo!() }
