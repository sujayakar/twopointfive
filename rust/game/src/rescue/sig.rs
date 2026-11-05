#![allow(unused_variables)]
//! Every function and method of `rescue`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/rescue.ts:155 doorName)
pub fn door_name(g: &mut crate::game::Game, id_2: &str) -> String { todo!() }

/// (src/game/rescue.ts:156 flat)
pub fn flat(a: V3, b: V3) -> f64 { todo!() }

/// (src/game/rescue.ts:157 wrap)
pub fn wrap(a: f64) -> f64 { todo!() }

/// (src/game/rescue.ts:164 checkRescue)
pub fn check_rescue(g: &mut crate::game::Game, s: Option<&crate::scene_scenarios::RescueData>) -> Vec<String> { todo!() }

/// (src/game/rescue.ts:267 Rescue.constructor)
pub fn rescue_new(g: &mut crate::game::Game) -> crate::rescue::Rescue { todo!() }

/// (src/game/rescue.ts:270 Rescue.data)
pub fn rescue_data(g: &mut crate::game::Game) -> Option<crate::scene_scenarios::RescueData> { todo!() }

/// (src/game/rescue.ts:272 Rescue.bounds)
pub fn rescue_bounds(g: &mut crate::game::Game) -> Option<crate::scene_scenarios::Rect> { todo!() }

/// (src/game/rescue.ts:274 Rescue.fromBounds)
pub fn rescue_from_bounds(g: &mut crate::game::Game) -> Option<crate::scene_scenarios::Rect> { todo!() }

/// (src/game/rescue.ts:276 Rescue.outOf)
pub fn rescue_out_of(g: &mut crate::game::Game, d: DoorId, room: Option<crate::scene_scenarios::Rect>, dist: f64) -> V3 { todo!() }

/// (src/game/rescue.ts:282 Rescue.roomPoint)
pub fn rescue_room_point(g: &mut crate::game::Game, which: crate::rescue::RescueRoomPointWhich) -> V3 { todo!() }

/// (src/game/rescue.ts:290 Rescue.canStart)
pub fn rescue_can_start(g: &mut crate::game::Game) -> Option<String> { todo!() }

/// (src/game/rescue.ts:307 Rescue.start)
pub fn rescue_start(g: &mut crate::game::Game) -> Option<String> { todo!() }

/// (src/game/rescue.ts:317 Rescue.stop)
pub fn rescue_stop(g: &mut crate::game::Game) { todo!() }

/// (src/game/rescue.ts:327 Rescue.restage)
pub fn rescue_restage(g: &mut crate::game::Game) { todo!() }

/// (src/game/rescue.ts:329 Rescue.onEncounterRestart)
pub fn rescue_on_encounter_restart(g: &mut crate::game::Game) { todo!() }

/// (src/game/rescue.ts:336 Rescue.mode)
pub fn rescue_mode(g: &mut crate::game::Game) -> crate::rescue::RescueMode { todo!() }

/// (src/game/rescue.ts:338 Rescue.owns)
pub fn rescue_owns(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/rescue.ts:340 Rescue.callOff)
pub fn rescue_call_off(g: &mut crate::game::Game) -> Option<String> { todo!() }

/// (src/game/rescue.ts:347 Rescue.isHostage)
pub fn rescue_is_hostage(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/rescue.ts:349 Rescue.bench)
pub fn rescue_bench(g: &mut crate::game::Game) { todo!() }

/// (src/game/rescue.ts:354 Rescue.clearCast)
pub fn rescue_clear_cast(g: &mut crate::game::Game) { todo!() }

/// (src/game/rescue.ts:363 Rescue.marks)
pub fn rescue_marks(g: &mut crate::game::Game, on: bool) { todo!() }

/// (src/game/rescue.ts:372 Rescue.faceOf)
pub fn rescue_face_of(g: &mut crate::game::Game, d: DoorId) -> f64 { todo!() }

/// (src/game/rescue.ts:375 Rescue.stage)
pub fn rescue_stage(g: &mut crate::game::Game) { todo!() }

/// (src/game/rescue.ts:436 Rescue.place)
pub fn rescue_place(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/rescue.ts:437 Rescue.note)
pub fn rescue_note(g: &mut crate::game::Game, what: &str) { todo!() }

/// (src/game/rescue.ts:438 Rescue.once)
pub fn rescue_once(g: &mut crate::game::Game, key: &str, text: &str) { todo!() }

/// (src/game/rescue.ts:439 Rescue.over)
pub fn rescue_over(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/rescue.ts:441 Rescue.dynamic)
pub fn rescue_dynamic(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/rescue.ts:445 Rescue.press)
pub fn rescue_press(g: &mut crate::game::Game) -> Option<crate::rescue::RescuePress> { todo!() }

/// (src/game/rescue.ts:453 Rescue.orderDoor)
pub fn rescue_order_door(g: &mut crate::game::Game) -> Option<crate::rescue::RescueOrderDoor> { todo!() }

/// (src/game/rescue.ts:481 Rescue.pick)
pub fn rescue_pick(g: &mut crate::game::Game) -> Option<crate::rescue::RescuePick> { todo!() }

/// (src/game/rescue.ts:502 Rescue.launch)
pub fn rescue_launch(g: &mut crate::game::Game, fd: DoorId, sd: Option<DoorId>, front: &[GuardId], side: &[GuardId], go: bool) -> Option<String> { todo!() }

/// (src/game/rescue.ts:523 Rescue.startClock)
pub fn rescue_start_clock(g: &mut crate::game::Game, why: &str) { todo!() }

/// (src/game/rescue.ts:538 Rescue.tick)
pub fn rescue_tick(g: &mut crate::game::Game, dt: f64, input: InputId) { todo!() }

/// (src/game/rescue.ts:561 Rescue.hear)
pub fn rescue_hear(g: &mut crate::game::Game) { todo!() }

/// (src/game/rescue.ts:577 Rescue.defenders)
pub fn rescue_defenders(g: &mut crate::game::Game) { todo!() }

/// (src/game/rescue.ts:622 Rescue.crewCall)
pub fn rescue_crew_call(g: &mut crate::game::Game) { todo!() }

/// (src/game/rescue.ts:639 Rescue.crew)
pub fn rescue_crew(g: &mut crate::game::Game, h: GuardId, d: &mut crate::rescue::Def, f: Option<RoomfightDefId>) -> bool { todo!() }

/// (src/game/rescue.ts:686 Rescue.crewMove)
pub fn rescue_crew_move(g: &mut crate::game::Game, h: GuardId, f: RoomfightDefId, goal: V3, by: Option<DoorId>) { todo!() }

/// (src/game/rescue.ts:691 Rescue.crewWay)
pub fn rescue_crew_way(g: &mut crate::game::Game, p: V3) -> Vec<V3> { todo!() }

/// (src/game/rescue.ts:699 Rescue.crewLegs)
pub fn rescue_crew_legs(g: &mut crate::game::Game, c_2: V3, path: &[V3], bad: &mut dyn FnMut(&mut crate::game::Game, V3) -> bool) -> bool { todo!() }

/// (src/game/rescue.ts:705 Rescue.crewSeen)
pub fn rescue_crew_seen(g: &mut crate::game::Game, p: V3, p_2: V3, near_m: f64) -> f64 { todo!() }

/// (src/game/rescue.ts:714 Rescue.crewCrossSpot)
pub fn rescue_crew_cross_spot(g: &mut crate::game::Game, h: GuardId, p: V3, walk: f64) -> Option<V3> { todo!() }

/// (src/game/rescue.ts:740 Rescue.crewHide)
pub fn rescue_crew_hide(g: &mut crate::game::Game, h: GuardId, f: RoomfightDefId, p: V3) -> Option<V3> { todo!() }

/// (src/game/rescue.ts:757 Rescue.crewLook)
pub fn rescue_crew_look(g: &mut crate::game::Game, gd: GuardId, pt: &mut V3) { todo!() }

/// (src/game/rescue.ts:759 Rescue.crewHides)
pub fn rescue_crew_hides(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/rescue.ts:761 Rescue.crewOn)
pub fn rescue_crew_on(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/rescue.ts:766 Rescue.roundBy)
pub fn rescue_round_by(g: &mut crate::game::Game, op: GuardId) { todo!() }

/// (src/game/rescue.ts:773 Rescue.post)
pub fn rescue_post(g: &mut crate::game::Game, op: GuardId, look: Option<V3>, door: Option<&str>) { todo!() }

/// (src/game/rescue.ts:784 Rescue.tickEntry)
pub fn rescue_tick_entry(g: &mut crate::game::Game) { todo!() }

/// (src/game/rescue.ts:825 Rescue.search)
pub fn rescue_search(g: &mut crate::game::Game) { todo!() }

/// (src/game/rescue.ts:841 Rescue.searched)
pub fn rescue_searched(g: &mut crate::game::Game, o: GuardId) { todo!() }

/// (src/game/rescue.ts:848 Rescue.endEntry)
pub fn rescue_end_entry(g: &mut crate::game::Game, called: bool) { todo!() }

/// (src/game/rescue.ts:859 Rescue.noiseAt)
pub fn rescue_noise_at(g: &mut crate::game::Game) -> Option<V3> { todo!() }

/// (src/game/rescue.ts:863 Rescue.cast)
pub fn rescue_cast(g: &mut crate::game::Game) -> Vec<crate::rescuecheck::CastItem> { todo!() }

/// (src/game/rescue.ts:868 Rescue.letGo)
pub fn rescue_let_go(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/rescue.ts:870 Rescue.outsidePoint)
pub fn rescue_outside_point(g: &mut crate::game::Game) -> V3 { todo!() }

/// (src/game/rescue.ts:872 Rescue.sideOutside)
pub fn rescue_side_outside(g: &mut crate::game::Game) -> Option<V3> { todo!() }

/// (src/game/rescue.ts:875 Rescue.sideWay)
pub fn rescue_side_way(g: &mut crate::game::Game) -> Vec<V3> { todo!() }

/// (src/game/rescue.ts:877 Rescue.captor)
pub fn rescue_captor(g: &mut crate::game::Game) -> Option<GuardId> { todo!() }

/// (src/game/rescue.ts:892 Rescue.lastWarning)
pub fn rescue_last_warning(g: &mut crate::game::Game, d: DoorId) -> bool { todo!() }

/// (src/game/rescue.ts:898 Rescue.clock)
pub fn rescue_clock(g: &mut crate::game::Game, dt: f64) { todo!() }

/// (src/game/rescue.ts:945 Rescue.endAct)
pub fn rescue_end_act(g: &mut crate::game::Game) { todo!() }

/// (src/game/rescue.ts:952 Rescue.setStep)
pub fn rescue_set_step(g: &mut crate::game::Game, s: crate::rescue::RescueStep) { todo!() }

/// (src/game/rescue.ts:959 Rescue.outcome)
pub fn rescue_outcome(g: &mut crate::game::Game) { todo!() }

/// (src/game/rescue.ts:986 Rescue.voices)
pub fn rescue_voices(g: &mut crate::game::Game) { todo!() }

/// (src/game/rescue.ts:1011 Rescue.end)
pub fn rescue_end(g: &mut crate::game::Game, state: crate::rescue::RescueEndState, why: &str) { todo!() }

/// (src/game/rescue.ts:1027 Rescue.state)
pub fn rescue_state(g: &mut crate::game::Game) -> crate::rescue::RescueState { todo!() }

/// (src/game/rescue.ts:1034 Rescue.objective)
pub fn rescue_objective(g: &mut crate::game::Game) -> Option<crate::rescue::RescueObjective> { todo!() }

/// (src/game/rescue.ts:1048 Rescue.clockStep)
pub fn rescue_clock_step(g: &mut crate::game::Game) -> crate::rescue::RescueStep { todo!() }

/// (src/game/rescue.ts:1050 Rescue.summary)
pub fn rescue_summary(g: &mut crate::game::Game) -> String { todo!() }
