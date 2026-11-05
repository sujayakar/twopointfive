#![allow(unused_variables)]
//! Every function and method of `rescue`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/rescue.ts:157 doorName)
pub fn door_name(g: &mut crate::game::Game, id_2: &str) -> String { todo!() }

/// (src/game/rescue.ts:158 flat)
pub fn flat(a: V3, b: V3) -> f64 { todo!() }

/// (src/game/rescue.ts:159 wrap)
pub fn wrap(a: f64) -> f64 { todo!() }

/// (src/game/rescue.ts:166 checkRescue)
pub fn check_rescue(g: &mut crate::game::Game, s: Option<&crate::scene_scenarios::RescueData>) -> Vec<String> { todo!() }

/// (src/game/rescue.ts:269 Rescue.constructor)
pub fn rescue_new(g: &mut crate::game::Game) -> crate::rescue::Rescue { todo!() }

/// (src/game/rescue.ts:272 Rescue.data)
pub fn rescue_data(g: &mut crate::game::Game) -> Option<crate::scene_scenarios::RescueData> { todo!() }

/// (src/game/rescue.ts:274 Rescue.bounds)
pub fn rescue_bounds(g: &mut crate::game::Game) -> Option<crate::scene_scenarios::Rect> { todo!() }

/// (src/game/rescue.ts:276 Rescue.fromBounds)
pub fn rescue_from_bounds(g: &mut crate::game::Game) -> Option<crate::scene_scenarios::Rect> { todo!() }

/// (src/game/rescue.ts:278 Rescue.outOf)
pub fn rescue_out_of(g: &mut crate::game::Game, d: DoorId, room: Option<crate::scene_scenarios::Rect>, dist: f64) -> V3 { todo!() }

/// (src/game/rescue.ts:284 Rescue.roomPoint)
pub fn rescue_room_point(g: &mut crate::game::Game, which: crate::rescue::RescueRoomPointWhich) -> V3 { todo!() }

/// (src/game/rescue.ts:292 Rescue.canStart)
pub fn rescue_can_start(g: &mut crate::game::Game) -> Option<String> { todo!() }

/// (src/game/rescue.ts:309 Rescue.start)
pub fn rescue_start(g: &mut crate::game::Game) -> Option<String> { todo!() }

/// (src/game/rescue.ts:319 Rescue.stop)
pub fn rescue_stop(g: &mut crate::game::Game) { todo!() }

/// (src/game/rescue.ts:329 Rescue.restage)
pub fn rescue_restage(g: &mut crate::game::Game) { todo!() }

/// (src/game/rescue.ts:331 Rescue.onEncounterRestart)
pub fn rescue_on_encounter_restart(g: &mut crate::game::Game) { todo!() }

/// (src/game/rescue.ts:338 Rescue.mode)
pub fn rescue_mode(g: &mut crate::game::Game) -> crate::rescue::RescueMode { todo!() }

/// (src/game/rescue.ts:340 Rescue.owns)
pub fn rescue_owns(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/rescue.ts:342 Rescue.callOff)
pub fn rescue_call_off(g: &mut crate::game::Game) -> Option<String> { todo!() }

/// (src/game/rescue.ts:349 Rescue.isHostage)
pub fn rescue_is_hostage(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/rescue.ts:351 Rescue.bench)
pub fn rescue_bench(g: &mut crate::game::Game) { todo!() }

/// (src/game/rescue.ts:356 Rescue.clearCast)
pub fn rescue_clear_cast(g: &mut crate::game::Game) { todo!() }

/// (src/game/rescue.ts:365 Rescue.marks)
pub fn rescue_marks(g: &mut crate::game::Game, on: bool) { todo!() }

/// (src/game/rescue.ts:374 Rescue.faceOf)
pub fn rescue_face_of(g: &mut crate::game::Game, d: DoorId) -> f64 { todo!() }

/// (src/game/rescue.ts:377 Rescue.stage)
pub fn rescue_stage(g: &mut crate::game::Game) { todo!() }

/// (src/game/rescue.ts:438 Rescue.place)
pub fn rescue_place(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/rescue.ts:439 Rescue.note)
pub fn rescue_note(g: &mut crate::game::Game, what: &str) { todo!() }

/// (src/game/rescue.ts:440 Rescue.once)
pub fn rescue_once(g: &mut crate::game::Game, key: &str, text: &str) { todo!() }

/// (src/game/rescue.ts:441 Rescue.over)
pub fn rescue_over(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/rescue.ts:443 Rescue.dynamic)
pub fn rescue_dynamic(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/rescue.ts:447 Rescue.press)
pub fn rescue_press(g: &mut crate::game::Game) -> Option<crate::rescue::RescuePress> { todo!() }

/// (src/game/rescue.ts:455 Rescue.orderDoor)
pub fn rescue_order_door(g: &mut crate::game::Game) -> Option<crate::rescue::RescueOrderDoor> { todo!() }

/// (src/game/rescue.ts:483 Rescue.pick)
pub fn rescue_pick(g: &mut crate::game::Game) -> Option<crate::rescue::RescuePick> { todo!() }

/// (src/game/rescue.ts:504 Rescue.launch)
pub fn rescue_launch(g: &mut crate::game::Game, fd: DoorId, sd: Option<DoorId>, front: &[GuardId], side: &[GuardId], go: bool) -> Option<String> { todo!() }

/// (src/game/rescue.ts:525 Rescue.startClock)
pub fn rescue_start_clock(g: &mut crate::game::Game, why: &str) { todo!() }

/// (src/game/rescue.ts:540 Rescue.tick)
pub fn rescue_tick(g: &mut crate::game::Game, dt: f64, input: InputId, cam: Option<CameraId>) { todo!() }

/// (src/game/rescue.ts:564 Rescue.hear)
pub fn rescue_hear(g: &mut crate::game::Game) { todo!() }

/// (src/game/rescue.ts:580 Rescue.defenders)
pub fn rescue_defenders(g: &mut crate::game::Game) { todo!() }

/// (src/game/rescue.ts:625 Rescue.crewCall)
pub fn rescue_crew_call(g: &mut crate::game::Game) { todo!() }

/// (src/game/rescue.ts:642 Rescue.crew)
pub fn rescue_crew(g: &mut crate::game::Game, h: GuardId, d: &mut crate::rescue::Def, f: Option<RoomfightDefId>) -> bool { todo!() }

/// (src/game/rescue.ts:689 Rescue.crewMove)
pub fn rescue_crew_move(g: &mut crate::game::Game, h: GuardId, f: RoomfightDefId, goal: V3, by: Option<DoorId>) { todo!() }

/// (src/game/rescue.ts:694 Rescue.crewWay)
pub fn rescue_crew_way(g: &mut crate::game::Game, p: V3) -> Vec<V3> { todo!() }

/// (src/game/rescue.ts:702 Rescue.crewLegs)
pub fn rescue_crew_legs(g: &mut crate::game::Game, c_2: V3, path: &[V3], bad: &mut dyn FnMut(&mut crate::game::Game, V3) -> bool) -> bool { todo!() }

/// (src/game/rescue.ts:708 Rescue.crewSeen)
pub fn rescue_crew_seen(g: &mut crate::game::Game, p: V3, p_2: V3, near_m: f64) -> f64 { todo!() }

/// (src/game/rescue.ts:717 Rescue.crewCrossSpot)
pub fn rescue_crew_cross_spot(g: &mut crate::game::Game, h: GuardId, p: V3, walk: f64) -> Option<V3> { todo!() }

/// (src/game/rescue.ts:743 Rescue.crewHide)
pub fn rescue_crew_hide(g: &mut crate::game::Game, h: GuardId, f: RoomfightDefId, p: V3) -> Option<V3> { todo!() }

/// (src/game/rescue.ts:760 Rescue.crewLook)
pub fn rescue_crew_look(g: &mut crate::game::Game, gd: GuardId, pt: &mut V3) { todo!() }

/// (src/game/rescue.ts:762 Rescue.crewHides)
pub fn rescue_crew_hides(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/rescue.ts:764 Rescue.crewOn)
pub fn rescue_crew_on(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/rescue.ts:769 Rescue.roundBy)
pub fn rescue_round_by(g: &mut crate::game::Game, op: GuardId) { todo!() }

/// (src/game/rescue.ts:776 Rescue.post)
pub fn rescue_post(g: &mut crate::game::Game, op: GuardId, look: Option<V3>, door: Option<&str>) { todo!() }

/// (src/game/rescue.ts:787 Rescue.tickEntry)
pub fn rescue_tick_entry(g: &mut crate::game::Game) { todo!() }

/// (src/game/rescue.ts:828 Rescue.search)
pub fn rescue_search(g: &mut crate::game::Game) { todo!() }

/// (src/game/rescue.ts:844 Rescue.searched)
pub fn rescue_searched(g: &mut crate::game::Game, o: GuardId) { todo!() }

/// (src/game/rescue.ts:851 Rescue.endEntry)
pub fn rescue_end_entry(g: &mut crate::game::Game, called: bool) { todo!() }

/// (src/game/rescue.ts:862 Rescue.noiseAt)
pub fn rescue_noise_at(g: &mut crate::game::Game) -> Option<V3> { todo!() }

/// (src/game/rescue.ts:866 Rescue.cast)
pub fn rescue_cast(g: &mut crate::game::Game) -> Vec<crate::rescuecheck::CastItem> { todo!() }

/// (src/game/rescue.ts:871 Rescue.letGo)
pub fn rescue_let_go(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/rescue.ts:873 Rescue.outsidePoint)
pub fn rescue_outside_point(g: &mut crate::game::Game) -> V3 { todo!() }

/// (src/game/rescue.ts:875 Rescue.sideOutside)
pub fn rescue_side_outside(g: &mut crate::game::Game) -> Option<V3> { todo!() }

/// (src/game/rescue.ts:878 Rescue.sideWay)
pub fn rescue_side_way(g: &mut crate::game::Game) -> Vec<V3> { todo!() }

/// (src/game/rescue.ts:880 Rescue.captor)
pub fn rescue_captor(g: &mut crate::game::Game) -> Option<GuardId> { todo!() }

/// (src/game/rescue.ts:895 Rescue.lastWarning)
pub fn rescue_last_warning(g: &mut crate::game::Game, d: DoorId) -> bool { todo!() }

/// (src/game/rescue.ts:901 Rescue.clock)
pub fn rescue_clock(g: &mut crate::game::Game, dt: f64) { todo!() }

/// (src/game/rescue.ts:948 Rescue.endAct)
pub fn rescue_end_act(g: &mut crate::game::Game) { todo!() }

/// (src/game/rescue.ts:955 Rescue.setStep)
pub fn rescue_set_step(g: &mut crate::game::Game, s: crate::rescue::RescueStep) { todo!() }

/// (src/game/rescue.ts:962 Rescue.outcome)
pub fn rescue_outcome(g: &mut crate::game::Game) { todo!() }

/// (src/game/rescue.ts:989 Rescue.voices)
pub fn rescue_voices(g: &mut crate::game::Game) { todo!() }

/// (src/game/rescue.ts:1014 Rescue.end)
pub fn rescue_end(g: &mut crate::game::Game, state: crate::rescue::RescueEndState, why: &str) { todo!() }

/// (src/game/rescue.ts:1030 Rescue.state)
pub fn rescue_state(g: &mut crate::game::Game) -> crate::rescue::RescueState { todo!() }

/// (src/game/rescue.ts:1037 Rescue.objective)
pub fn rescue_objective(g: &mut crate::game::Game) -> Option<crate::rescue::RescueObjective> { todo!() }

/// (src/game/rescue.ts:1051 Rescue.clockStep)
pub fn rescue_clock_step(g: &mut crate::game::Game) -> crate::rescue::RescueStep { todo!() }

/// (src/game/rescue.ts:1053 Rescue.summary)
pub fn rescue_summary(g: &mut crate::game::Game) -> String { todo!() }
