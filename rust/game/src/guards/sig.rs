#![allow(unused_variables)]
//! Every function and method of `guards`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/guards.ts:92 glassHeard)
pub fn glass_heard(g: &mut crate::game::Game, e: crate::game::GameEvent, ear: V3) -> crate::scene_collision::Propagation { todo!() }

/// (src/game/guards.ts:100 hearingCheck)
pub fn hearing_check(g: &mut crate::game::Game, gd: GuardId, ear: Option<f64>, eyes: Option<bool>, foot: Option<f64>) { todo!() }

/// (src/game/guards.ts:205 gunfireDue)
pub fn gunfire_due(g: &mut crate::game::Game, gd: GuardId, bangs: Option<bool>) -> bool { todo!() }

/// (src/game/guards.ts:219 shotRelation)
pub fn shot_relation(g: &mut crate::game::Game, gd: GuardId, e: &crate::guards::ShotRelationE) -> crate::guards::ShotRelation { todo!() }

/// (src/game/guards.ts:226 colleagueFirePos)
pub fn colleague_fire_pos(g: &mut crate::game::Game, gd: GuardId, e: crate::guards::ColleagueFirePosE) -> Option<V3> { todo!() }

/// (src/game/guards.ts:249 stepIsThreat)
pub fn step_is_threat(g: &mut crate::game::Game, gd: GuardId, e: crate::guards::StepIsThreatE) -> bool { todo!() }

/// (src/game/guards.ts:269 groundFresh)
pub fn ground_fresh(g: &mut crate::game::Game) { todo!() }

/// (src/game/guards.ts:270 playerGroundVel)
pub fn player_ground_vel(g: &mut crate::game::Game) -> V3 { todo!() }

/// (src/game/guards.ts:283 motionFactor)
pub fn motion_factor(g: &mut crate::game::Game, gd: GuardId, dist: f64) -> f64 { todo!() }

/// (src/game/guards.ts:288 motionCore)
pub fn motion_core(g: &mut crate::game::Game, gd: GuardId, pp: V3, v: V3, lit: f64, dist: f64) -> f64 { todo!() }

/// (src/game/guards.ts:303 sightGain)
pub fn sight_gain(g: &mut crate::game::Game, gd: GuardId, dist: f64) -> f64 { todo!() }

/// (src/game/guards.ts:316 sightGainOn)
pub fn sight_gain_on(g: &mut crate::game::Game, gd: GuardId, o: GuardId, dist: f64) -> f64 { todo!() }

/// (src/game/guards.ts:331 bodyInView)
pub fn body_in_view(g: &mut crate::game::Game, gd: GuardId) -> Option<GuardId> { todo!() }

/// (src/game/guards.ts:342 bodySeen)
pub fn body_seen(g: &mut crate::game::Game, gd: GuardId, o: GuardId, around: Option<f64>) -> bool { todo!() }

/// (src/game/guards.ts:373 bodyProbeSlot)
pub fn body_probe_slot(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/guards.ts:375 bodyVisibility)
pub fn body_visibility(g: &mut crate::game::Game, o: GuardId) -> f64 { todo!() }

/// (src/game/guards.ts:388 bodyLit)
pub fn body_lit(g: &mut crate::game::Game, gd: GuardId, o: GuardId, eye: V3, dist: f64) -> bool { todo!() }

/// (src/game/guards.ts:401 loadBodyLight)
pub fn load_body_light(g: &mut crate::game::Game, tune: &mut crate::guards::LoadBodyLightTune) { todo!() }

/// (src/game/guards.ts:408 saveBodyLight)
pub fn save_body_light(g: &mut crate::game::Game, tune: crate::guards::SaveBodyLightTune) { todo!() }

/// (src/game/guards.ts:409 resetBodyLight)
pub fn reset_body_light(g: &mut crate::game::Game, tune: &mut crate::guards::ResetBodyLightTune) { todo!() }

/// (src/game/guards.ts:411 bodyLightSummary)
pub fn body_light_summary(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/guards.ts:419 pointInView)
pub fn point_in_view(g: &mut crate::game::Game, gd: GuardId, p: V3, range: Option<f64>, half_deg: Option<f64>) -> bool { todo!() }

/// (src/game/guards.ts:431 lightOutSeen)
pub fn light_out_seen(g: &mut crate::game::Game, gd: GuardId, e: crate::guards::LightOutSeenE) -> bool { todo!() }

/// (src/game/guards.ts:443 witnessKill)
pub fn witness_kill(g: &mut crate::game::Game, victim: GuardId, how: crate::game::DownKind, by: crate::game::DownBy) { todo!() }

/// (src/game/guards.ts:454 alertCause)
pub fn alert_cause(g: &mut crate::game::Game, gd: GuardId, seen: bool) -> crate::guards::AlertCause { todo!() }

/// (src/game/guards.ts:462 witnessSummary)
pub fn witness_summary(g: &mut crate::game::Game, gd: GuardId) -> String { todo!() }

/// (src/game/guards.ts:478 bodyPos)
pub fn body_pos(g: &mut crate::game::Game, o: GuardId) -> V3 { todo!() }

/// (src/game/guards.ts:480 nearKnownBody)
pub fn near_known_body(g: &mut crate::game::Game, p: V3, r: Option<f64>) -> Option<GuardId> { todo!() }

/// (src/game/guards.ts:486 doorTamper)
pub fn door_tamper(g: &mut crate::game::Game, d: DoorId) -> Option<crate::guards::DoorTamper> { todo!() }

/// (src/game/guards.ts:493 coverSpot)
pub fn cover_spot(g: &mut crate::game::Game, gd: GuardId, bp: V3, stand: f64) -> V3 { todo!() }

/// (src/game/guards.ts:508 fanSpot)
pub fn fan_spot(g: &mut crate::game::Game, gd: GuardId, b: GuardId, bp: V3) -> Option<V3> { todo!() }

/// (src/game/guards.ts:527 notGettingAnywhere)
pub fn not_getting_anywhere(g: &mut crate::game::Game, gd: GuardId, dt: f64, secs: f64) -> bool { todo!() }

/// (src/game/guards.ts:534 dropToSearch)
pub fn drop_to_search(g: &mut crate::game::Game, gd: GuardId, bark: Option<&str>) { todo!() }

/// (src/game/guards.ts:543 resetChase)
pub fn reset_chase(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/guards.ts:551 bodyAftermath)
pub fn body_aftermath(g: &mut crate::game::Game, gd: GuardId, dt: f64) -> Option<crate::guards::BodyAftermath> { todo!() }

/// (src/game/guards.ts:605 leadFix)
pub fn lead_fix(g: &mut crate::game::Game, from: V3, vel: V3) -> V3 { todo!() }

/// (src/game/guards.ts:628 shotOrigin)
pub fn shot_origin(g: &mut crate::game::Game, c_2: CharId) -> V3 { todo!() }

/// (src/game/guards.ts:636 clearShot)
pub fn clear_shot(g: &mut crate::game::Game, c_2: CharId, to: V3) -> bool { todo!() }

/// (src/game/guards.ts:646 paneFirst)
pub fn pane_first(g: &mut crate::game::Game, c_2: CharId, to: V3) -> bool { todo!() }

/// (src/game/guards.ts:677 lineGeom)
pub fn line_geom(g: &mut crate::game::Game, from: V3, to: V3, o: GuardId) -> Option<crate::guards::LineGeom> { todo!() }

/// (src/game/guards.ts:692 colleagueInLine)
pub fn colleague_in_line(g: &mut crate::game::Game, gd: GuardId, to: V3, except: Option<GuardId>) -> Option<GuardId> { todo!() }

/// (src/game/guards.ts:709 roundPastColleague)
pub fn round_past_colleague(g: &mut crate::game::Game, gd: GuardId, to: V3, except: Option<GuardId>, cone_per_m: Option<f64>) -> bool { todo!() }

/// (src/game/guards.ts:725 aimAhead)
pub fn aim_ahead(g: &mut crate::game::Game, c_2: CharId) -> V3 { todo!() }

/// (src/game/guards.ts:734 muzzleCheck)
pub fn muzzle_check(g: &mut crate::game::Game, gd: GuardId, to: Option<V3>, dt: f64, except: Option<GuardId>) -> Option<GuardId> { todo!() }

/// (src/game/guards.ts:749 lineStep)
pub fn line_step(g: &mut crate::game::Game, gd: GuardId, aim_at: V3, o: Option<GuardId>, dt: f64) -> crate::guards::LineStep { todo!() }

/// (src/game/guards.ts:859 shieldUp)
pub fn shield_up(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/guards.ts:863 pairOf)
pub fn pair_of(g: &mut crate::game::Game) -> Option<crate::guards::Pair> { todo!() }

/// (src/game/guards.ts:873 forgetGuardClocks)
pub fn forget_guard_clocks(g: &mut crate::game::Game) { todo!() }

/// (src/game/guards.ts:877 pairInView)
pub fn pair_in_view(g: &mut crate::game::Game, gd: GuardId, p: &mut crate::guards::Pair) -> bool { todo!() }

/// (src/game/guards.ts:898 pairSightRate)
pub fn pair_sight_rate(g: &mut crate::game::Game, gd: GuardId, p: &mut crate::guards::Pair, dist: f64) -> f64 { todo!() }

/// (src/game/guards.ts:913 forgetHeldMeter)
pub fn forget_held_meter(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/guards.ts:914 heldVisibility)
pub fn held_visibility(g: &mut crate::game::Game, p: &mut crate::guards::Pair) -> f64 { todo!() }

/// (src/game/guards.ts:921 standoffPerceive)
pub fn standoff_perceive(g: &mut crate::game::Game, gd: GuardId, dt: Option<f64>) -> bool { todo!() }

/// (src/game/guards.ts:951 hostageCall)
pub fn hostage_call(g: &mut crate::game::Game, gd: GuardId, p: crate::guards::Pair) { todo!() }

/// (src/game/guards.ts:968 pickSide)
pub fn pick_side(g: &mut crate::game::Game, gd: GuardId, p: crate::guards::Pair) -> f64 { todo!() }

/// (src/game/guards.ts:978 axisToSegment)
pub fn axis_to_segment(g: &mut crate::game::Game, from: V3, to: V3, o: GuardId) -> f64 { todo!() }

/// (src/game/guards.ts:988 flankShot)
pub fn flank_shot(g: &mut crate::game::Game, gd: GuardId, p: crate::guards::Pair) -> crate::guards::FlankShot { todo!() }

/// (src/game/guards.ts:1021 standoffBark)
pub fn standoff_bark(g: &mut crate::game::Game, gd: GuardId, moment: crate::guards::StandoffMoment, force: Option<crate::guards::StandoffBarkForce>, held: Option<GuardId>) { todo!() }

/// (src/game/guards.ts:1036 stepAlongPath)
pub fn step_along_path(g: &mut crate::game::Game, gd: GuardId, dt: f64, speed: f64) -> bool { todo!() }

/// (src/game/guards.ts:1054 orbitGoalFor)
pub fn orbit_goal_for(g: &mut crate::game::Game, gd: GuardId, s: GuardsStandoffId, p: crate::guards::Pair) -> Option<V3> { todo!() }

/// (src/game/guards.ts:1097 approachPoint)
pub fn approach_point(g: &mut crate::game::Game, gd: GuardId, c_2: V3, need_sight: Option<bool>) -> Option<V3> { todo!() }

/// (src/game/guards.ts:1115 retreatGoal)
pub fn retreat_goal(g: &mut crate::game::Game, gd: GuardId, p: crate::guards::Pair) -> Option<V3> { todo!() }

/// (src/game/guards.ts:1136 standoffAlert)
pub fn standoff_alert(g: &mut crate::game::Game, gd: GuardId, dt: f64, p: crate::guards::Pair) -> crate::guards::StandoffAlert { todo!() }

/// (src/game/guards.ts:1204 standoffUpkeep)
pub fn standoff_upkeep(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::guards::Pair> { todo!() }

/// (src/game/guards.ts:1227 hostageHitBark)
pub fn hostage_hit_bark(g: &mut crate::game::Game, shooter: GuardId, victim: GuardId) { todo!() }

/// (src/game/guards.ts:1229 standoffSummary)
pub fn standoff_summary(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/guards.ts:1239 canSee)
pub fn can_see(g: &mut crate::game::Game, gd: GuardId) -> crate::guards::CanSee { todo!() }

/// (src/game/guards.ts:1245 canSeeChar)
pub fn can_see_char(g: &mut crate::game::Game, gd: GuardId, pc: CharId, is_player: bool) -> crate::guards::CanSeeChar { todo!() }

/// (src/game/guards.ts:1275 eyeReach)
pub fn eye_reach(g: &mut crate::game::Game, lit: f64) -> f64 { todo!() }

/// (src/game/guards.ts:1281 reachFor)
pub fn reach_for(g: &mut crate::game::Game, gd: GuardId, pc: CharId, is_player: bool, eye: V3) -> f64 { todo!() }

/// (src/game/guards.ts:1293 farFaint)
pub fn far_faint(g: &mut crate::game::Game, gd: GuardId, dist: f64, was: f64) -> bool { todo!() }

/// (src/game/guards.ts:1301 distTerm)
pub fn dist_term(g: &mut crate::game::Game, dist: f64) -> f64 { todo!() }

/// (src/game/guards.ts:1309 teamThrough)
pub fn team_through(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/guards.ts:1310 eyeThrough)
pub fn eye_through(g: &mut crate::game::Game, gd: GuardId) -> f64 { todo!() }

/// (src/game/guards.ts:1313 eyeBlocked)
pub fn eye_blocked(g: &mut crate::game::Game, gd: GuardId, a: V3, b: V3) -> bool { todo!() }

/// (src/game/guards.ts:1327 eyeContact)
pub fn eye_contact(g: &mut crate::game::Game, e: crate::guards::Eye, pc: CharId, is_player: bool, blind: bool, smoke_trans: f64, through: Option<f64>) -> crate::guards::EyeContact { todo!() }

/// (src/game/guards.ts:1342 eyeContactHere)
pub fn eye_contact_here(g: &mut crate::game::Game, e: crate::guards::Eye, pc: CharId, is_player: bool, tgt: V3, tgt2: V3, blind: bool, smoked: bool, through: f64) -> crate::guards::EyeContactHere { todo!() }

/// (src/game/guards.ts:1387 strandedAt)
pub fn stranded_at(g: &mut crate::game::Game, x: f64, z: f64) -> bool { todo!() }

/// (src/game/guards.ts:1395 legClear)
pub fn leg_clear(g: &mut crate::game::Game, from: V3, to: V3) -> bool { todo!() }

/// (src/game/guards.ts:1406 freeCellNear)
pub fn free_cell_near(g: &mut crate::game::Game, from: V3, reach: f64, test: crate::guards::FreeCellNearTest) -> Option<V3> { todo!() }

/// (src/game/guards.ts:1420 unstickSnap)
pub fn unstick_snap(g: &mut crate::game::Game, gd: GuardId, to: Option<V3>, why: &str) { todo!() }

/// (src/game/guards.ts:1429 liveLeg)
pub fn live_leg(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::guards::UnstickLeg> { todo!() }

/// (src/game/guards.ts:1437 unstickBegin)
pub fn unstick_begin(g: &mut crate::game::Game, gd: GuardId) -> crate::guards::UnstickBegin { todo!() }

/// (src/game/guards.ts:1455 unstickStep)
pub fn unstick_step(g: &mut crate::game::Game, gd: GuardId, dt: f64) -> crate::guards::UnstickStep { todo!() }

/// (src/game/guards.ts:1471 unstickFirst)
pub fn unstick_first(g: &mut crate::game::Game, gd: GuardId, dt: f64) -> bool { todo!() }

/// (src/game/guards.ts:1481 goTo)
pub fn go_to(g: &mut crate::game::Game, gd: GuardId, goal: V3, force: Option<bool>) { todo!() }

/// (src/game/guards.ts:1560 litPlace)
pub fn lit_place(g: &mut crate::game::Game, gd: GuardId, a: Option<OrdersAssignmentId>, tgt: Option<GuardId>, pt: Option<V3>) -> bool { todo!() }

/// (src/game/guards.ts:1577 looksIntoDark)
pub fn looks_into_dark(g: &mut crate::game::Game, gd: GuardId, a: Option<OrdersAssignmentId>) -> bool { todo!() }

/// (src/game/guards.ts:1588 placeLitSeen)
pub fn place_lit_seen(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/guards.ts:1593 beamFan)
pub fn beam_fan(d: V3) -> Vec<V3> { todo!() }

/// (src/game/guards.ts:1603 rayEnd)
pub fn ray_end(g: &mut crate::game::Game, o: V3, r: V3, m: f64) -> f64 { todo!() }

/// (src/game/guards.ts:1620 beamWalled)
pub fn beam_walled(g: &mut crate::game::Game, o: V3, fan: &[V3], m: f64, held: bool) -> bool { todo!() }

/// (src/game/guards.ts:1627 beamWalledAt)
pub fn beam_walled_at(g: &mut crate::game::Game, o: V3, d: V3, m: f64, held: Option<bool>) -> bool { todo!() }

/// (src/game/guards.ts:1628 beamRayEnd)
pub fn beam_ray_end(g: &mut crate::game::Game, o: V3, r: V3, m: f64) -> f64 { todo!() }

/// (src/game/guards.ts:1630 burnsFrom)
pub fn burns_from(g: &mut crate::game::Game, held: V3, p: V3) -> V3 { todo!() }

/// (src/game/guards.ts:1632 beamLens)
pub fn beam_lens(g: &mut crate::game::Game, gd: GuardId) -> Option<V3> { todo!() }

/// (src/game/guards.ts:1642 wallDark)
pub fn wall_dark(g: &mut crate::game::Game, gd: GuardId, held: Option<V3>) -> bool { todo!() }

/// (src/game/guards.ts:1661 torchLow)
pub fn torch_low(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/guards.ts:1669 torchLowSeenOf)
pub fn torch_low_seen_of(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/guards.ts:1671 wallDarkSeen)
pub fn wall_dark_seen(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/guards.ts:1674 wallPutOut)
pub fn wall_put_out(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/guards.ts:1676 lightRuled)
pub fn light_ruled(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/guards.ts:1679 darkOnLitMan)
pub fn dark_on_lit_man(g: &mut crate::game::Game, gd: GuardId, tgt: GuardId) -> bool { todo!() }

/// (src/game/guards.ts:1687 onLitMan)
pub fn on_lit_man(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/guards.ts:1694 teamLit)
pub fn team_lit(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/guards.ts:1706 teamPaced)
pub fn team_paced(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/guards.ts:1708 teamPace)
pub fn team_pace(g: &mut crate::game::Game, gd: GuardId, speed: f64) -> f64 { todo!() }

/// (src/game/guards.ts:1712 teamLegs)
pub fn team_legs(g: &mut crate::game::Game, gd: GuardId, speed: f64) -> f64 { todo!() }

/// (src/game/guards.ts:1715 teamTurn)
pub fn team_turn(g: &mut crate::game::Game, gd: GuardId, to: f64, dt: f64) -> f64 { todo!() }

/// (src/game/guards.ts:1721 teamLean)
pub fn team_lean(g: &mut crate::game::Game, gd: GuardId, was: f64, dt: f64) { todo!() }

/// (src/game/guards.ts:1723 followPath)
pub fn follow_path(g: &mut crate::game::Game, gd: GuardId, dt: f64, speed: f64) -> bool { todo!() }

/// (src/game/guards.ts:1814 doorCells)
pub fn door_cells(g: &mut crate::game::Game) -> JsMap<crate::Num, DoorId> { todo!() }

/// (src/game/guards.ts:1830 floodFrom)
pub fn flood_from(g: &mut crate::game::Game, from: V3, max_m: f64, keyed: Option<bool>) -> crate::guards::FloodFrom { todo!() }

/// (src/game/guards.ts:1866 dealSpots)
pub fn deal_spots(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/guards.ts:1918 searchPlanFor)
pub fn search_plan_for(g: &mut crate::game::Game, gd: GuardId) -> crate::guards::SearchPlan { todo!() }

/// (src/game/guards.ts:1933 searchProbes)
pub fn search_probes(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/guards.ts:1956 scoreSpots)
pub fn score_spots(g: &mut crate::game::Game, gd: GuardId, p: &crate::guards::SearchPlan, walk: Option<&[f32]>) { todo!() }

/// (src/game/guards.ts:1966 spotTaken)
pub fn spot_taken(g: &mut crate::game::Game, gd: GuardId, s: &crate::guards::SearchSpot) -> bool { todo!() }

/// (src/game/guards.ts:1975 pickNextSpot)
pub fn pick_next_spot(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/guards.ts:1986 searchSummary)
pub fn search_summary(g: &mut crate::game::Game, gd: GuardId) -> String { todo!() }

/// (src/game/guards.ts:2013 frameOf)
pub fn frame_of(g: &mut crate::game::Game, d: DoorId) -> crate::doorway::RelF { todo!() }

/// (src/game/guards.ts:2015 crossesDoor)
pub fn crosses_door(g: &mut crate::game::Game, a: V3, b: V3, d: DoorId) -> Option<crate::guards::CrossesDoor> { todo!() }

/// (src/game/guards.ts:2025 noteDoorCrossings)
pub fn note_door_crossings(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/guards.ts:2032 pieDoorway)
pub fn pie_doorway(g: &mut crate::game::Game, gd: GuardId, dt: f64, speed: f64, eyes_on: bool) -> crate::guards::PieDoorway { todo!() }

/// (src/game/guards.ts:2070 pathCrosses)
pub fn path_crosses(g: &mut crate::game::Game, gd: GuardId, d: DoorId) -> bool { todo!() }

/// (src/game/guards.ts:2072 refreshCrossing)
pub fn refresh_crossing(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/guards.ts:2077 bendApproach)
pub fn bend_approach(g: &mut crate::game::Game, gd: GuardId, d: DoorId, sg: f64) { todo!() }

/// (src/game/guards.ts:2099 chaseFix)
pub fn chase_fix(g: &mut crate::game::Game, gd: GuardId, dt: f64, pace: f64) -> crate::guards::ChaseFix { todo!() }

/// (src/game/guards.ts:2121 updateGuard)
pub fn update_guard(g: &mut crate::game::Game, gd: GuardId, dt: f64) { todo!() }

/// (src/game/guards.ts:2575 collideGuard)
pub fn collide_guard(g: &mut crate::game::Game, gd: GuardId, p0x: f64, p0z: f64) { todo!() }

/// (src/game/guards.ts:2601 sweepStatics)
pub fn sweep_statics(g: &mut crate::game::Game, c_2: CharId, fx: f64, fz: f64) { todo!() }

/// (src/game/guards.ts:2616 lightWindow)
pub fn light_window(d: f64, range: f64) -> f64 { todo!() }

/// (src/game/guards.ts:2628 lightEstimateAt)
pub fn light_estimate_at(g: &mut crate::game::Game, p: V3, skip_owner: Option<&mut dyn FnMut(&mut crate::game::Game, f64) -> bool>, steady_only: Option<bool>, field: Option<&mut crate::sense::LightField>, moon: Option<bool>) -> f64 { todo!() }

/// (src/game/guards.ts:2633 lightEstimateIn)
pub fn light_estimate_in(g: &mut crate::game::Game, p: V3, skip_owner: Option<&mut dyn FnMut(&mut crate::game::Game, f64) -> bool>, steady_only: bool, field: Option<&mut crate::sense::LightField>, moon: bool) -> f64 { todo!() }

/// (src/game/guards.ts:2646 lightField)
pub fn light_field(g: &mut crate::game::Game) -> Option<crate::sense::LightField> { todo!() }

/// (src/game/guards.ts:2651 lampAsked)
pub fn lamp_asked(g: &mut crate::game::Game, l: LightId, skip_owner: Option<&mut dyn FnMut(&mut crate::game::Game, f64) -> bool>, steady_only: bool) -> bool { todo!() }

/// (src/game/guards.ts:2658 writeLamps)
pub fn write_lamps(g: &mut crate::game::Game, p: Option<V3>, skip_owner: Option<&mut dyn FnMut(&mut crate::game::Game, f64) -> bool>, steady_only: bool) -> bool { todo!() }

/// (src/game/guards.ts:2668 CARRIED)
pub fn carried() -> bool { todo!() }

/// (src/game/guards.ts:2671 steadyLamps)
pub fn steady_lamps(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/guards.ts:2676 lampsThere)
pub fn lamps_there(g: &mut crate::game::Game, p: V3, skip_owner: Option<&mut dyn FnMut(&mut crate::game::Game, f64) -> bool>, steady_only: bool, acc: &mut Vec<f64>, field: Option<crate::sense::LightField>) -> bool { todo!() }

/// (src/game/guards.ts:2681 lampsHere)
pub fn lamps_here(g: &mut crate::game::Game, p: V3, skip_owner: Option<&mut dyn FnMut(&mut crate::game::Game, f64) -> bool>, steady_only: bool, acc: &mut Vec<f64>, field: Option<&mut crate::sense::LightField>) { todo!() }

/// (src/game/guards.ts:2716 meterOf)
pub fn meter_of(g: &mut crate::game::Game, c_2: CharId, prefix: &str) -> f64 { todo!() }

/// (src/game/guards.ts:2726 unitLightMeter)
pub fn unit_light_meter(g: &mut crate::game::Game, gd: GuardId, dt: f64) { todo!() }

/// (src/game/guards.ts:2736 acquireFoe)
pub fn acquire_foe(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/guards.ts:2780 rigOf)
pub fn rig_of(g: &mut crate::game::Game, gd: GuardId) -> crate::guards::RigOf { todo!() }

/// (src/game/guards.ts:2783 longRigOf)
pub fn long_rig_of(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::guards::LongRigOf> { todo!() }

/// (src/game/guards.ts:2790 handCarry)
pub fn hand_carry(g: &mut crate::game::Game, gd: GuardId) -> crate::guards::HandCarry { todo!() }

/// (src/game/guards.ts:2796 gunBack)
pub fn gun_back(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/guards.ts:2800 inHand)
pub fn in_hand(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::guards::InHand> { todo!() }

/// (src/game/guards.ts:2809 startReloadBeat)
pub fn start_reload_beat(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/guards.ts:2818 tickReloadBeat)
pub fn tick_reload_beat(g: &mut crate::game::Game, gd: GuardId, dt: f64) { todo!() }

/// (src/game/guards.ts:2851 burstTrigger)
pub fn burst_trigger(g: &mut crate::game::Game, gd: GuardId, dt: f64, aim_at: V3, start_ok: bool, round_ok: bool, except: Option<GuardId>) { todo!() }

/// (src/game/guards.ts:2890 burstSummary)
pub fn burst_summary(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/guards.ts:2900 attachFlashlight)
pub fn attach_flashlight(g: &mut crate::game::Game, gd: GuardId, dt: Option<f64>) { todo!() }

/// (src/game/guards.ts:2905 poseFlashlight)
pub fn pose_flashlight(g: &mut crate::game::Game, gd: GuardId, dt: f64) -> Option<V3> { todo!() }

/// (src/game/guards.ts:2941 onScreen)
pub fn on_screen(g: &mut crate::game::Game, p: V3) -> bool { todo!() }

/// (src/game/guards.ts:2945 inView)
pub fn in_view(vp: Option<&crate::guards::InViewVp>, p: V3) -> bool { todo!() }

/// (src/game/guards.ts:2953 bubbleSecs)
pub fn bubble_secs(text: &str) -> f64 { todo!() }

/// (src/game/guards.ts:2958 headOf)
pub fn head_of(g: &mut crate::game::Game, gd: GuardId) -> V3 { todo!() }

/// (src/game/guards.ts:2960 say)
pub fn say(g: &mut crate::game::Game, gd: GuardId, text: &str, radio: Option<bool>, force: Option<bool>, row: Option<bool>) { todo!() }

/// (src/game/guards.ts:2974 resetGuards)
pub fn reset_guards(g: &mut crate::game::Game) { todo!() }

/// (src/game/guards.ts:3004 escalationOf)
pub fn escalation_of(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/guards.ts:3005 escalationName)
pub fn escalation_name(l: f64) -> String { todo!() }

/// (src/game/guards.ts:3008 escalationSummary)
pub fn escalation_summary(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/guards.ts:3020 clearingSummary)
pub fn clearing_summary(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/guards.ts:3024 spokesman)
pub fn spokesman(g: &mut crate::game::Game, pos: Option<V3>, except: Option<GuardId>) -> Option<GuardId> { todo!() }

/// (src/game/guards.ts:3038 raiseEscalation)
pub fn raise_escalation(g: &mut crate::game::Game, to: f64, except: Option<GuardId>, cause: Option<crate::guards::RaiseEscalationCause>, another: Option<bool>) { todo!() }

/// (src/game/guards.ts:3053 standDownEscalation)
pub fn stand_down_escalation(g: &mut crate::game::Game) { todo!() }

/// (src/game/guards.ts:3078 offNet)
pub fn off_net(g: &mut crate::game::Game, x: GuardId) -> bool { todo!() }

/// (src/game/guards.ts:3080 rollcallState)
pub fn rollcall_state<'g>(g: &'g mut crate::game::Game) -> &'g mut crate::guards::Rollcall { todo!() }

/// (src/game/guards.ts:3084 rollcallInterval)
pub fn rollcall_interval(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/guards.ts:3087 radioCaller)
pub fn radio_caller(g: &mut crate::game::Game) -> Option<GuardId> { todo!() }

/// (src/game/guards.ts:3096 checkerFor)
pub fn checker_for(g: &mut crate::game::Game, m: GuardId) -> Option<GuardId> { todo!() }

/// (src/game/guards.ts:3099 stillLooking)
pub fn still_looking(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/guards.ts:3101 pickChecker)
pub fn pick_checker(g: &mut crate::game::Game, m: GuardId, prefer: Option<GuardId>) -> Option<GuardId> { todo!() }

/// (src/game/guards.ts:3110 sendCheckOn)
pub fn send_check_on(g: &mut crate::game::Game, gd: GuardId, m: GuardId, line: &str) { todo!() }

/// (src/game/guards.ts:3122 checkOnTick)
pub fn check_on_tick(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/guards.ts:3133 advanceCheckOn)
pub fn advance_check_on(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/guards.ts:3141 checkOnDone)
pub fn check_on_done(g: &mut crate::game::Game, gd: GuardId, m: GuardId, capped: bool) { todo!() }

/// (src/game/guards.ts:3152 updateRollcall)
pub fn update_rollcall(g: &mut crate::game::Game, dt: f64) { todo!() }

/// (src/game/guards.ts:3221 partnerWatch)
pub fn partner_watch(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/guards.ts:3253 planClears)
pub fn plan_clears(g: &mut crate::game::Game, force: Option<bool>, cause: Option<crate::guards::PlanClearsCause>) -> bool { todo!() }

/// (src/game/guards.ts:3279 clearQueueSummary)
pub fn clear_queue_summary(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/guards.ts:3287 forceClear)
pub fn force_clear(g: &mut crate::game::Game) { todo!() }

/// (src/game/guards.ts:3296 stepDownEscalation)
pub fn step_down_escalation(g: &mut crate::game::Game) { todo!() }

/// (src/game/guards.ts:3307 updateEscalation)
pub fn update_escalation(g: &mut crate::game::Game, dt: f64) { todo!() }

/// (src/game/guards.ts:3331 superviseLockdown)
pub fn supervise_lockdown(g: &mut crate::game::Game, dt: f64, living: f64) { todo!() }

/// (src/game/guards.ts:3355 assignLockdown)
pub fn assign_lockdown(g: &mut crate::game::Game, redeal: bool, except: Option<GuardId>) { todo!() }

/// (src/game/guards.ts:3375 clearLockdown)
pub fn clear_lockdown(g: &mut crate::game::Game) { todo!() }

/// (src/game/guards.ts:3376 routeLength)
pub fn route_length(g: &mut crate::game::Game, gd: GuardId) -> f64 { todo!() }

/// (src/game/guards.ts:3385 followLeader)
pub fn follow_leader(g: &mut crate::game::Game, gd: GuardId, dt: f64, walk: f64) -> bool { todo!() }

/// (src/game/guards.ts:3413 holdPost)
pub fn hold_post(g: &mut crate::game::Game, gd: GuardId, dt: f64, walk: f64) -> bool { todo!() }

/// (src/game/guards.ts:3433 lockdownDoors)
pub fn lockdown_doors(g: &mut crate::game::Game, gd: GuardId, calm: Option<bool>) { todo!() }

/// (src/game/guards.ts:3460 tension)
pub fn tension(g: &mut crate::game::Game, theirs: Option<bool>) -> f64 { todo!() }

/// (src/game/guards.ts:3515 spawnUnit)
pub fn spawn_unit(g: &mut crate::game::Game, o: &crate::guards::UnitOpts) -> GuardId { todo!() }

/// (src/game/guards.ts:3540 releaseUnit)
pub fn release_unit(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/guards.ts:3544 despawnUnit)
pub fn despawn_unit(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/guards.ts:3564 despawnUnits)
pub fn despawn_units(g: &mut crate::game::Game) { todo!() }

/// (src/game/guards.ts:3566 units)
pub fn units(g: &mut crate::game::Game) -> Vec<GuardId> { todo!() }

/// (src/game/guards.ts:3576 engagementOf)
pub fn engagement_of(g: &mut crate::game::Game) -> Option<crate::guards::Engagement> { todo!() }

/// (src/game/guards.ts:3578 armForFight)
pub fn arm_for_fight(g: &mut crate::game::Game, gd: GuardId, foes: &[GuardId], pin: bool, delay: Option<f64>) { todo!() }

/// (src/game/guards.ts:3595 engage)
pub fn engage(g: &mut crate::game::Game, operatives: &[GuardId], hostiles: &[GuardId], opts: Option<crate::guards::EngageOpts>) -> crate::guards::Engagement { todo!() }

/// (src/game/guards.ts:3606 nameOfId)
pub fn name_of_id(g: &mut crate::game::Game, id_2: f64) -> String { todo!() }

/// (src/game/guards.ts:3608 endEngagement)
pub fn end_engagement(g: &mut crate::game::Game, outcome: crate::guards::EndEngagementOutcome) { todo!() }

/// (src/game/guards.ts:3615 tickEngagement)
pub fn tick_engagement(g: &mut crate::game::Game) { todo!() }
