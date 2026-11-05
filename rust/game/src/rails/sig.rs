#![allow(unused_variables)]
//! Every function and method of `rails`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/rails.ts:166 said)
pub fn said(a: &crate::rails::SaidA, gave: f64) -> Option<String> { todo!() }

/// (src/game/rails.ts:169 lookoutEye)
pub fn lookout_eye(id_2: f64, e: crate::rails::LookoutEyeE) -> crate::fogofwar::FogEye { todo!() }

/// (src/game/rails.ts:181 rowsWithheld)
pub fn rows_withheld(l: Option<&crate::scene_scenarios::LessonData>) -> Vec<String> { todo!() }

/// (src/game/rails.ts:186 railsFaults)
pub fn rails_faults(l: &crate::scene_scenarios::LessonData) -> Vec<String> { todo!() }

/// (src/game/rails.ts:242 Rails.fight)
pub fn rails_fight(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/rails.ts:248 Rails.constructor)
pub fn rails_new(g: &mut crate::game::Game) -> crate::rails::Rails { todo!() }

/// (src/game/rails.ts:251 Rails.data)
pub fn rails_data(g: &mut crate::game::Game) -> Option<crate::scene_scenarios::RailsData> { todo!() }

/// (src/game/rails.ts:252 Rails.live)
pub fn rails_live(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/rails.ts:253 Rails.skyOn)
pub fn rails_sky_on(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/rails.ts:255 Rails.flatRing)
pub fn rails_flat_ring(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/rails.ts:256 Rails.lesson)
pub fn rails_lesson(g: &mut crate::game::Game) -> Option<crate::rails::RailsLesson> { todo!() }

/// (src/game/rails.ts:259 Rails.allowed)
pub fn rails_allowed(g: &mut crate::game::Game) -> Option<JsSet<String>> { todo!() }

/// (src/game/rails.ts:266 Rails.locks)
pub fn rails_locks(g: &mut crate::game::Game, action: &str) -> bool { todo!() }

/// (src/game/rails.ts:271 Rails.shuts)
pub fn rails_shuts(g: &mut crate::game::Game, action: &str) -> bool { todo!() }

/// (src/game/rails.ts:278 Rails.hushes)
pub fn rails_hushes(g: &mut crate::game::Game, action: &str) -> bool { todo!() }

/// (src/game/rails.ts:281 Rails.locksWall)
pub fn rails_locks_wall(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/rails.ts:286 Rails.card)
pub fn rails_card(g: &mut crate::game::Game) -> Option<crate::rails::RailsCard> { todo!() }

/// (src/game/rails.ts:289 Rails.settles)
pub fn rails_settles(g: &mut crate::game::Game, l: &crate::scene_scenarios::LessonData, i: f64, done_t: f64) -> bool { todo!() }

/// (src/game/rails.ts:294 Rails.arrow)
pub fn rails_arrow(g: &mut crate::game::Game) -> Option<V3> { todo!() }

/// (src/game/rails.ts:297 Rails.noLasers)
pub fn rails_no_lasers(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/rails.ts:299 Rails.tapOff)
pub fn rails_tap_off(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/rails.ts:301 Rails.teachesHold)
pub fn rails_teaches_hold(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/rails.ts:303 Rails.clean)
pub fn rails_clean(g: &mut crate::game::Game) { todo!() }

/// (src/game/rails.ts:305 Rails.noteGiven)
pub fn rails_note_given(g: &mut crate::game::Game, o: &str) { todo!() }

/// (src/game/rails.ts:318 Rails.runs)
pub fn rails_runs(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/rails.ts:325 Rails.onWay)
pub fn rails_on_way(g: &mut crate::game::Game, s: Option<&crate::rails::RailsOnWayS>) -> bool { todo!() }

/// (src/game/rails.ts:333 Rails.offers)
pub fn rails_offers(g: &mut crate::game::Game, verb: &str) -> bool { todo!() }

/// (src/game/rails.ts:336 Rails.gate)
pub fn rails_gate(g: &mut crate::game::Game, input: InputId, cam: CameraId, dt: f64) { todo!() }

/// (src/game/rails.ts:387 Rails.markUnder)
pub fn rails_mark_under(g: &mut crate::game::Game, input: InputId, cam: CameraId, canvas: crate::rails::RailsMarkUnderCanvas) -> Option<V3> { todo!() }

/// (src/game/rails.ts:394 Rails.orderRows)
pub fn rails_order_rows(g: &mut crate::game::Game, rows: &[crate::rails::RailsOrderRowsRowsItem]) -> Vec<crate::rails::RailsOrderRowsRowsItem> { todo!() }

/// (src/game/rails.ts:396 Rails.eye)
pub fn rails_eye(g: &mut crate::game::Game) -> Option<crate::fogofwar::FogEye> { todo!() }

/// (src/game/rails.ts:402 Rails.eyeFirst)
pub fn rails_eye_first(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/rails.ts:404 Rails.eyeView)
pub fn rails_eye_view(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/rails.ts:406 Rails.stepEye)
pub fn rails_step_eye(g: &mut crate::game::Game) -> Option<crate::fogofwar::FogEye> { todo!() }

/// (src/game/rails.ts:417 Rails.hostView)
pub fn rails_host_view(g: &mut crate::game::Game, cam: CameraId) { todo!() }

/// (src/game/rails.ts:424 Rails.tick)
pub fn rails_tick(g: &mut crate::game::Game, dt: f64) { todo!() }

/// (src/game/rails.ts:448 Rails.answer)
pub fn rails_answer(g: &mut crate::game::Game, s: &crate::rails::RailsAnswerS, again: bool) { todo!() }

/// (src/game/rails.ts:456 Rails.startProgram)
pub fn rails_start_program(g: &mut crate::game::Game, why: &str) { todo!() }

/// (src/game/rails.ts:463 Rails.putBack)
pub fn rails_put_back(g: &mut crate::game::Game) { todo!() }

/// (src/game/rails.ts:465 Rails.resume)
pub fn rails_resume(g: &mut crate::game::Game) { todo!() }

/// (src/game/rails.ts:468 Rails.cued)
pub fn rails_cued(g: &mut crate::game::Game, name: &str) -> bool { todo!() }

/// (src/game/rails.ts:471 Rails.holdsEnd)
pub fn rails_holds_end(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/rails.ts:472 Rails.endsNow)
pub fn rails_ends_now(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/rails.ts:476 Rails.prompt)
pub fn rails_prompt(g: &mut crate::game::Game) -> Option<crate::rails::RailsPrompt> { todo!() }

/// (src/game/rails.ts:505 Rails.spare)
pub fn rails_spare(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/rails.ts:515 Rails.isSpared)
pub fn rails_is_spared(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/rails.ts:519 Rails.lineMen)
pub fn rails_line_men(g: &mut crate::game::Game) -> Vec<crate::rails::RailsLineMenItem> { todo!() }

/// (src/game/rails.ts:525 Rails.squad)
pub fn rails_squad(g: &mut crate::game::Game) -> Vec<GuardId> { todo!() }

/// (src/game/rails.ts:526 Rails.park)
pub fn rails_park(g: &mut crate::game::Game, r: &crate::scene_scenarios::RailsData) { todo!() }

/// (src/game/rails.ts:533 Rails.holdHere)
pub fn rails_hold_here(g: &mut crate::game::Game, gd: GuardId, low: Option<bool>) { todo!() }

/// (src/game/rails.ts:537 Rails.waiting)
pub fn rails_waiting(g: &mut crate::game::Game, s: &crate::rails::RailsWaitingS) -> bool { todo!() }

/// (src/game/rails.ts:544 Rails.holdWaiting)
pub fn rails_hold_waiting(g: &mut crate::game::Game, s: &crate::rails::RailsHoldWaitingS) { todo!() }

/// (src/game/rails.ts:548 Rails.unpark)
pub fn rails_unpark(g: &mut crate::game::Game) { todo!() }

/// (src/game/rails.ts:550 Rails.pan)
pub fn rails_pan(g: &mut crate::game::Game, input: InputId, cam: CameraId, dt: f64) { todo!() }

/// (src/game/rails.ts:558 Rails.keep)
pub fn rails_keep(g: &mut crate::game::Game, input: InputId, cam: CameraId, dt: f64) { todo!() }

/// (src/game/rails.ts:571 Rails.panned)
pub fn rails_panned(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/rails.ts:573 Rails.look)
pub fn rails_look(g: &mut crate::game::Game) -> Option<V3> { todo!() }

/// (src/game/rails.ts:582 Rails.summary)
pub fn rails_summary(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/rails.ts:587 Rails.note)
pub fn rails_note(g: &mut crate::game::Game, s: &str) { todo!() }

/// (src/game/rails.ts:591 railsSpare)
pub fn rails_spare_2(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/rails.ts:593 railsDown)
pub fn rails_down(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }
