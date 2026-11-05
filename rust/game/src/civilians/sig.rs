#![allow(unused_variables)]
//! Every function and method of `civilians`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/civilians.ts:71 civSpots)
pub fn civ_spots(g: &mut crate::game::Game) -> crate::civilians::CivSpots { todo!() }

/// (src/game/civilians.ts:128 withDice)
pub fn with_dice<T: Clone + core::fmt::Debug>(seed: f64, fn_: &mut dyn FnMut(&mut crate::game::Game) -> T) -> T { todo!() }

/// (src/game/civilians.ts:183 log)
pub fn log(g: &mut crate::game::Game, gd: GuardId, what: &str) { todo!() }

/// (src/game/civilians.ts:189 join)
pub fn join(g: &mut crate::game::Game, gd: GuardId) -> GuardId { todo!() }

/// (src/game/civilians.ts:194 spawnCivilians)
pub fn spawn_civilians(g: &mut crate::game::Game) { todo!() }

/// (src/game/civilians.ts:201 resetCivilians)
pub fn reset_civilians(g: &mut crate::game::Game) { todo!() }

/// (src/game/civilians.ts:333 civilianCompromise)
pub fn civilian_compromise(g: &mut crate::game::Game) -> Option<String> { todo!() }

/// (src/game/civilians.ts:345 makeCivilian)
pub fn make_civilian(g: &mut crate::game::Game, def: &crate::scene_level::CivilianDef, i: f64) -> GuardId { todo!() }

/// (src/game/civilians.ts:372 spawnHostage)
pub fn spawn_hostage(g: &mut crate::game::Game, def: &crate::civilians::SpawnHostageDef) -> GuardId { todo!() }

/// (src/game/civilians.ts:381 spawnTarget)
pub fn spawn_target(g: &mut crate::game::Game, def: &crate::scene_scenarios::SnatchDataMan) -> GuardId { todo!() }

/// (src/game/civilians.ts:389 civilianYield)
pub fn civilian_yield(g: &mut crate::game::Game, gd: GuardId, to: V3) { todo!() }

/// (src/game/civilians.ts:395 civilianBolt)
pub fn civilian_bolt(g: &mut crate::game::Game, gd: GuardId, at: V3) { todo!() }

/// (src/game/civilians.ts:403 civilianSees)
pub fn civilian_sees(g: &mut crate::game::Game, gd: GuardId, pc: CharId, mate: Option<GuardId>, bar: f64) -> bool { todo!() }

/// (src/game/civilians.ts:411 civilianTie)
pub fn civilian_tie(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/civilians.ts:419 hostageSay)
pub fn hostage_say(g: &mut crate::game::Game, gd: GuardId, moment: crate::dialogue::HostageMoment, other: Option<GuardId>) -> bool { todo!() }

/// (src/game/civilians.ts:428 forgetMeter)
pub fn forget_meter(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/civilians.ts:432 civiliansOnFloor)
pub fn civilians_on_floor(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/civilians.ts:434 civiliansShown)
pub fn civilians_shown(g: &mut crate::game::Game) -> Vec<GuardId> { todo!() }

/// (src/game/civilians.ts:437 civilianBodies)
pub fn civilian_bodies(g: &mut crate::game::Game) -> Vec<GuardId> { todo!() }

/// (src/game/civilians.ts:439 civilianDoorContacts)
pub fn civilian_door_contacts(g: &mut crate::game::Game, contacts: &mut Vec<crate::doors::DoorContact>) { todo!() }

/// (src/game/civilians.ts:444 floorOf)
pub fn floor_of<'g>(g: &'g mut crate::game::Game) -> &'g mut crate::civilians::CiviliansModFloorsValue { todo!() }

/// (src/game/civilians.ts:450 later)
pub fn later(g: &mut crate::game::Game, secs: f64, p: &crate::civilians::LaterP) { todo!() }

/// (src/game/civilians.ts:452 runPending)
pub fn run_pending(g: &mut crate::game::Game, p: &crate::civilians::Pending) { todo!() }

/// (src/game/civilians.ts:477 civSay)
pub fn civ_say(g: &mut crate::game::Game, gd: GuardId, text: &str, hold: Option<f64>, unheard: Option<bool>) { todo!() }

/// (src/game/civilians.ts:484 onScreenHead)
pub fn on_screen_head(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/civilians.ts:489 inView)
pub fn in_view(vp: Option<&crate::civilians::InViewVp>, p: V3) -> bool { todo!() }

/// (src/game/civilians.ts:495 pick)
pub fn pick(g: &mut crate::game::Game, gd: GuardId, pool: &[String]) -> String { todo!() }

/// (src/game/civilians.ts:498 away)
pub fn away(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/civilians.ts:500 reportPool)
pub fn report_pool(g: &mut crate::game::Game, cause: Option<&str>) -> Vec<String> { todo!() }

/// (src/game/civilians.ts:502 noSecurity)
pub fn no_security(g: &mut crate::game::Game, pool: &[String]) -> Vec<String> { todo!() }

/// (src/game/civilians.ts:504 fill)
pub fn fill(g: &mut crate::game::Game, gd: GuardId, text: &str, guard: Option<GuardId>) -> String { todo!() }

/// (src/game/civilians.ts:510 control)
pub fn control(g: &mut crate::game::Game, gd: GuardId, text: &str) { todo!() }

/// (src/game/civilians.ts:512 scream)
pub fn scream(g: &mut crate::game::Game, gd: GuardId, shout: Option<&str>) { todo!() }

/// (src/game/civilians.ts:521 updateCivilians)
pub fn update_civilians(g: &mut crate::game::Game, dt: f64) { todo!() }

/// (src/game/civilians.ts:533 setState)
pub fn set_state(g: &mut crate::game::Game, gd: GuardId, s: crate::civilians::CivState) { todo!() }

/// (src/game/civilians.ts:535 updateOne)
pub fn update_one(g: &mut crate::game::Game, gd: GuardId, dt: f64) { todo!() }

/// (src/game/civilians.ts:574 stepSound)
pub fn step_sound(g: &mut crate::game::Game, gd: GuardId, move_: f64) { todo!() }

/// (src/game/civilians.ts:581 eyeOf)
pub fn eye_of(g: &mut crate::game::Game, gd: GuardId) -> crate::knobs::CivKEye { todo!() }

/// (src/game/civilians.ts:589 see)
pub fn see(g: &mut crate::game::Game, gd: GuardId, dt: f64) { todo!() }

/// (src/game/civilians.ts:609 hear)
pub fn hear(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/civilians.ts:631 startleOrWorse)
pub fn startle_or_worse(g: &mut crate::game::Game, gd: GuardId, at: V3) { todo!() }

/// (src/game/civilians.ts:638 bodies)
pub fn bodies(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/civilians.ts:656 civVisibility)
pub fn civ_visibility(g: &mut crate::game::Game, gd: GuardId, dt: f64) -> f64 { todo!() }

/// (src/game/civilians.ts:664 taskAt)
pub fn task_at(g: &mut crate::game::Game, gd: GuardId) -> crate::scene_level::CivilianTask { todo!() }

/// (src/game/civilians.ts:672 civilianPushesProps)
pub fn civilian_pushes_props(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/civilians.ts:674 civilianSeated)
pub fn civilian_seated(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/civilians.ts:685 work)
pub fn work(g: &mut crate::game::Game, gd: GuardId, dt: f64) -> crate::civilians::Work { todo!() }

/// (src/game/civilians.ts:719 endTask)
pub fn end_task(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/civilians.ts:721 toWork)
pub fn to_work(g: &mut crate::game::Game, gd: GuardId, line: bool) { todo!() }

/// (src/game/civilians.ts:731 toStartled)
pub fn to_startled(g: &mut crate::game::Game, gd: GuardId, at: V3, sound: bool) { todo!() }

/// (src/game/civilians.ts:740 startled)
pub fn startled(g: &mut crate::game::Game, gd: GuardId, dt: f64) { todo!() }

/// (src/game/civilians.ts:754 toAlarmed)
pub fn to_alarmed(g: &mut crate::game::Game, gd: GuardId, cause: crate::civilians::CivCause, at: V3, armed: bool) { todo!() }

/// (src/game/civilians.ts:776 alarmed)
pub fn alarmed(g: &mut crate::game::Game, gd: GuardId, dt: f64) { todo!() }

/// (src/game/civilians.ts:787 released)
pub fn released(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/civilians.ts:796 pathMetres)
pub fn path_metres(g: &mut crate::game::Game, from: V3, to: V3) -> crate::civilians::PathMetres { todo!() }

/// (src/game/civilians.ts:802 tellable)
pub fn tellable(g: &mut crate::game::Game, o: GuardId) -> bool { todo!() }

/// (src/game/civilians.ts:806 pickTarget)
pub fn pick_target(g: &mut crate::game::Game, gd: GuardId, opts: Option<crate::civilians::PickTargetOpts>) -> Option<crate::civilians::CivTarget> { todo!() }

/// (src/game/civilians.ts:824 toFlee)
pub fn to_flee(g: &mut crate::game::Game, gd: GuardId, t: &crate::civilians::CivTarget, evac: bool) { todo!() }

/// (src/game/civilians.ts:832 flee)
pub fn flee(g: &mut crate::game::Game, gd: GuardId, dt: f64) -> f64 { todo!() }

/// (src/game/civilians.ts:867 firstHand)
pub fn first_hand(cause: Option<crate::civilians::CivCause>) -> bool { todo!() }

/// (src/game/civilians.ts:868 toReport)
pub fn to_report(g: &mut crate::game::Game, gd: GuardId, o: GuardId) { todo!() }

/// (src/game/civilians.ts:878 primeGuard)
pub fn prime_guard(g: &mut crate::game::Game, o: GuardId, pos: V3, beat: f64) { todo!() }

/// (src/game/civilians.ts:884 report)
pub fn report(g: &mut crate::game::Game, gd: GuardId, dt: f64) { todo!() }

/// (src/game/civilians.ts:898 deliverReport)
pub fn deliver_report(g: &mut crate::game::Game, gd: GuardId, o: GuardId, how: crate::civilians::DeliverReportHow) { todo!() }

/// (src/game/civilians.ts:915 toPhone)
pub fn to_phone(g: &mut crate::game::Game, gd: GuardId, s: &crate::civilians::CivSpot) { todo!() }

/// (src/game/civilians.ts:924 phone)
pub fn phone(g: &mut crate::game::Game, gd: GuardId, dt: f64) { todo!() }

/// (src/game/civilians.ts:935 phoneCall)
pub fn phone_call(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/civilians.ts:947 toGone)
pub fn to_gone(g: &mut crate::game::Game, gd: GuardId, evac: bool) { todo!() }

/// (src/game/civilians.ts:960 cornerSpot)
pub fn corner_spot(g: &mut crate::game::Game, gd: GuardId) -> V3 { todo!() }

/// (src/game/civilians.ts:976 toCower)
pub fn to_cower(g: &mut crate::game::Game, gd: GuardId, complied: bool) { todo!() }

/// (src/game/civilians.ts:987 hostageLead)
pub fn hostage_lead(g: &mut crate::game::Game, gd: GuardId, lead: Option<CharId>, gap: f64) { todo!() }

/// (src/game/civilians.ts:993 civilianLedTo)
pub fn civilian_led_to(g: &mut crate::game::Game, gd: GuardId, to: Option<V3>) { todo!() }

/// (src/game/civilians.ts:995 civilianOff)
pub fn civilian_off(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/civilians.ts:1001 follow)
pub fn follow(g: &mut crate::game::Game, gd: GuardId, dt: f64) -> crate::civilians::Follow { todo!() }

/// (src/game/civilians.ts:1020 cower)
pub fn cower(g: &mut crate::game::Game, gd: GuardId, dt: f64) -> crate::civilians::Cower { todo!() }

/// (src/game/civilians.ts:1044 civilianGrabbed)
pub fn civilian_grabbed(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/civilians.ts:1052 civilianAnswer)
pub fn civilian_answer(g: &mut crate::game::Game, gd: GuardId, rnd: Option<&mut dyn FnMut(&mut crate::game::Game) -> f64>) -> Option<crate::dialogue::DialoguePick> { todo!() }

/// (src/game/civilians.ts:1073 spellWords)
pub fn spell_words(code: &str) -> String { todo!() }

/// (src/game/civilians.ts:1075 civilianSamLine)
pub fn civilian_sam_line(g: &mut crate::game::Game, gd: GuardId) -> crate::civilians::CivilianSamLine { todo!() }

/// (src/game/civilians.ts:1079 civilianComply)
pub fn civilian_comply(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/civilians.ts:1098 civilianDown)
pub fn civilian_down(g: &mut crate::game::Game, gd: GuardId, dir: V3, quiet: bool, how: crate::game::DownKind, by: crate::game::DownBy, line: Option<&str>) { todo!() }

/// (src/game/civilians.ts:1121 civPointInView)
pub fn civ_point_in_view(g: &mut crate::game::Game, gd: GuardId, p: V3, range: f64) -> bool { todo!() }

/// (src/game/civilians.ts:1130 staffCalm)
pub fn staff_calm(g: &mut crate::game::Game, o: GuardId) -> bool { todo!() }

/// (src/game/civilians.ts:1136 guardsNotice)
pub fn guards_notice(g: &mut crate::game::Game, dt: f64) { todo!() }

/// (src/game/civilians.ts:1182 bodyInViewOf)
pub fn body_in_view_of(g: &mut crate::game::Game, o: GuardId, cv: GuardId) -> bool { todo!() }

/// (src/game/civilians.ts:1188 foundDown)
pub fn found_down(g: &mut crate::game::Game, cv: GuardId, o: GuardId) { todo!() }

/// (src/game/civilians.ts:1207 hearScream)
pub fn hear_scream(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/civilians.ts:1219 chattyEnough)
pub fn chatty_enough(g: &mut crate::game::Game, x: GuardId) -> bool { todo!() }

/// (src/game/civilians.ts:1220 greetings)
pub fn greetings(g: &mut crate::game::Game) { todo!() }

/// (src/game/civilians.ts:1238 evacuationCheck)
pub fn evacuation_check(g: &mut crate::game::Game) { todo!() }

/// (src/game/civilians.ts:1253 saveCivTune)
pub fn save_civ_tune(g: &mut crate::game::Game) { todo!() }

/// (src/game/civilians.ts:1254 loadCivTune)
pub fn load_civ_tune(g: &mut crate::game::Game) { todo!() }

/// (src/game/civilians.ts:1259 setCivilians)
pub fn set_civilians(g: &mut crate::game::Game, on: bool) { todo!() }

/// (src/game/civilians.ts:1263 civiliansSummary)
pub fn civilians_summary(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/civilians.ts:1281 civPanic)
pub fn civ_panic(g: &mut crate::game::Game, gd: Option<GuardId>, what: Option<crate::civilians::CivPanicWhat>) -> String { todo!() }

/// (src/game/civilians.ts:1298 checkCivSpots)
pub fn check_civ_spots(g: &mut crate::game::Game) -> Vec<String> { todo!() }


/// INVENTED: `T.guard ?? T.spot` (civilians.ts:809, 847, 857, 860; snatch.ts:179) as a key of Civ.bad_targets. Guard(o) for a man. For a spot, Spot(n): n is its place in civ_spots(g).phones, or phones.len() + its place in .exits, the list chosen by t.kind and the spot found by name and pos.
pub fn civ_target_key(g: &mut crate::game::Game, t: &crate::civilians::CivTarget) -> crate::civilians::CivBadTarget { todo!() }
