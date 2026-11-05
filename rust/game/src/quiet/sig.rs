#![allow(unused_variables)]
//! Every function and method of `quiet`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/quiet.ts:62 heldNow)
pub fn held_now(g: &mut crate::game::Game) -> crate::quiet::Held { todo!() }

/// (src/game/quiet.ts:63 heldSet)
pub fn held_set(g: &mut crate::game::Game, h: crate::quiet::Held) { todo!() }

/// (src/game/quiet.ts:66 outThrough)
pub fn out_through(x: &crate::scene_scenarios::QuietExit, p: V3) -> bool { todo!() }

/// (src/game/quiet.ts:71 exitPoint)
pub fn exit_point(x: &crate::scene_scenarios::QuietExit, past: Option<f64>) -> V3 { todo!() }

/// (src/game/quiet.ts:93 QuietMission.constructor)
pub fn quiet_mission_new(g: &mut crate::game::Game) -> crate::quiet::QuietMission { todo!() }

/// (src/game/quiet.ts:96 QuietMission.block)
pub fn quiet_mission_block(g: &mut crate::game::Game) -> Option<crate::scene_scenarios::QuietData> { todo!() }

/// (src/game/quiet.ts:97 QuietMission.start_)
pub fn quiet_mission_start_(g: &mut crate::game::Game) -> Option<crate::scene_scenarios::QuietStart> { todo!() }

/// (src/game/quiet.ts:98 QuietMission.spot)
pub fn quiet_mission_spot(g: &mut crate::game::Game) -> Option<crate::scene_scenarios::QuietSpot> { todo!() }

/// (src/game/quiet.ts:99 QuietMission.exits)
pub fn quiet_mission_exits(g: &mut crate::game::Game) -> Vec<crate::scene_scenarios::QuietExit> { todo!() }

/// (src/game/quiet.ts:100 QuietMission.thing)
pub fn quiet_mission_thing(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/quiet.ts:102 QuietMission.patrol)
pub fn quiet_mission_patrol(g: &mut crate::game::Game) -> Vec<String> { todo!() }

/// (src/game/quiet.ts:105 QuietMission.canStart)
pub fn quiet_mission_can_start(g: &mut crate::game::Game, squad: Option<f64>) -> Option<String> { todo!() }

/// (src/game/quiet.ts:118 QuietMission.start)
pub fn quiet_mission_start(g: &mut crate::game::Game, pin: Option<f64>, squad: Option<f64>) -> Option<String> { todo!() }

/// (src/game/quiet.ts:142 QuietMission.stop)
pub fn quiet_mission_stop(g: &mut crate::game::Game) { todo!() }

/// (src/game/quiet.ts:150 QuietMission.putBack)
pub fn quiet_mission_put_back(g: &mut crate::game::Game) { todo!() }

/// (src/game/quiet.ts:157 QuietMission.release)
pub fn quiet_mission_release(g: &mut crate::game::Game) { todo!() }

/// (src/game/quiet.ts:164 QuietMission.current)
pub fn quiet_mission_current(g: &mut crate::game::Game) { todo!() }

/// (src/game/quiet.ts:170 QuietMission.restage)
pub fn quiet_mission_restage(g: &mut crate::game::Game) { todo!() }

/// (src/game/quiet.ts:173 QuietMission.beforeRestart)
pub fn quiet_mission_before_restart(g: &mut crate::game::Game) { todo!() }

/// (src/game/quiet.ts:189 QuietMission.onEncounterRestart)
pub fn quiet_mission_on_encounter_restart(g: &mut crate::game::Game) { todo!() }

/// (src/game/quiet.ts:195 QuietMission.tick)
pub fn quiet_mission_tick(g: &mut crate::game::Game) { todo!() }

/// (src/game/quiet.ts:211 QuietMission.snapshot)
pub fn quiet_mission_snapshot(g: &mut crate::game::Game) -> Option<crate::quiet::QuietSnap> { todo!() }

/// (src/game/quiet.ts:213 QuietMission.restoreDeal)
pub fn quiet_mission_restore_deal(g: &mut crate::game::Game, s: Option<crate::quiet::QuietSnap>) { todo!() }

/// (src/game/quiet.ts:220 QuietMission.rosterOf)
pub fn quiet_mission_roster_of(g: &mut crate::game::Game, d: Option<crate::quiet::QuietDeal>) -> f64 { todo!() }

/// (src/game/quiet.ts:227 QuietMission.summary)
pub fn quiet_mission_summary(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/quiet.ts:234 QuietMission.note)
pub fn quiet_mission_note(g: &mut crate::game::Game, s: &str) { todo!() }

/// (src/game/quiet.ts:235 QuietMission.dealOne)
pub fn quiet_mission_deal_one(g: &mut crate::game::Game, q: &crate::scene_scenarios::QuietData) -> crate::quiet::QuietDeal { todo!() }

/// (src/game/quiet.ts:244 QuietMission.unbench)
pub fn quiet_mission_unbench(g: &mut crate::game::Game) { todo!() }

/// (src/game/quiet.ts:251 QuietMission.rack)
pub fn quiet_mission_rack(g: &mut crate::game::Game) { todo!() }

/// (src/game/quiet.ts:252 QuietMission.stage)
pub fn quiet_mission_stage(g: &mut crate::game::Game) { todo!() }

/// (src/game/quiet.ts:270 QuietMission.behind)
pub fn quiet_mission_behind(g: &mut crate::game::Game, s: &crate::scene_scenarios::QuietStart) -> V3 { todo!() }

/// (src/game/quiet.ts:287 holderStand)
pub fn holder_stand(g: &mut crate::game::Game, front: V3, area: Option<crate::quiet::HolderStandArea>) -> crate::quiet::HolderStand { todo!() }

/// (src/game/quiet.ts:299 quietOf)
pub fn quiet_of(g: &mut crate::game::Game) { todo!() }

/// (src/game/quiet.ts:302 quietWords)
pub fn quiet_words(g: &mut crate::game::Game) -> Option<crate::quiet::QuietWords> { todo!() }

/// (src/game/quiet.ts:309 exitTaken)
pub fn exit_taken(g: &mut crate::game::Game, p: V3) -> Option<crate::scene_scenarios::QuietExit> { todo!() }

/// (src/game/quiet.ts:316 nearestExit)
pub fn nearest_exit(g: &mut crate::game::Game, p: V3) -> Option<crate::scene_scenarios::QuietExit> { todo!() }

/// (src/game/quiet.ts:326 checkQuiet)
pub fn check_quiet(g: &mut crate::game::Game) -> Vec<String> { todo!() }
