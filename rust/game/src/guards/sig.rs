#![allow(unused_variables)]
//! Every function and method of `guards`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/guards.ts:92 glassHeard)
pub fn glass_heard(g: &mut crate::game::Game, e: crate::game::GameEvent, ear: V3) -> crate::scene_collision::Propagation { todo!() }

/// (src/game/guards.ts:101 eyesOn)
pub fn eyes_on(g: &mut crate::game::Game, gd: GuardId, at: V3) { todo!() }

/// (src/game/guards.ts:102 hearingCheck)
pub fn hearing_check(g: &mut crate::game::Game, gd: GuardId, ear: Option<f64>, eyes: Option<bool>, foot: Option<f64>) { todo!() }

/// (src/game/guards.ts:208 gunfireDue)
pub fn gunfire_due(g: &mut crate::game::Game, gd: GuardId, bangs: Option<bool>) -> bool { todo!() }

/// (src/game/guards.ts:222 shotRelation)
pub fn shot_relation(g: &mut crate::game::Game, gd: GuardId, e: crate::guards::ShotRelationE) -> crate::guards::ShotRelation { todo!() }

/// (src/game/guards.ts:229 colleagueFirePos)
pub fn colleague_fire_pos(g: &mut crate::game::Game, gd: GuardId, e: crate::guards::ColleagueFirePosE) -> Option<V3> { todo!() }

/// (src/game/guards.ts:252 stepIsThreat)
pub fn step_is_threat(g: &mut crate::game::Game, gd: GuardId, e: crate::guards::StepIsThreatE) -> bool { todo!() }

/// (src/game/guards.ts:272 groundFresh)
pub fn ground_fresh(g: &mut crate::game::Game) { todo!() }

/// (src/game/guards.ts:273 playerGroundVel)
pub fn player_ground_vel(g: &mut crate::game::Game) -> V3 { todo!() }

/// (src/game/guards.ts:286 motionFactor)
pub fn motion_factor(g: &mut crate::game::Game, gd: GuardId, dist: f64) -> f64 { todo!() }

/// (src/game/guards.ts:291 motionCore)
pub fn motion_core(g: &mut crate::game::Game, gd: GuardId, pp: V3, v: V3, lit: f64, dist: f64) -> f64 { todo!() }

/// (src/game/guards.ts:306 sightGain)
pub fn sight_gain(g: &mut crate::game::Game, gd: GuardId, dist: f64) -> f64 { todo!() }

/// (src/game/guards.ts:319 sightGainOn)
pub fn sight_gain_on(g: &mut crate::game::Game, gd: GuardId, o: GuardId, dist: f64) -> f64 { todo!() }

/// (src/game/guards.ts:334 bodyInView)
pub fn body_in_view(g: &mut crate::game::Game, gd: GuardId) -> Option<GuardId> { todo!() }

/// (src/game/guards.ts:345 bodySeen)
pub fn body_seen(g: &mut crate::game::Game, gd: GuardId, o: GuardId, around: Option<f64>) -> bool { todo!() }

/// (src/game/guards.ts:376 bodyProbeSlot)
pub fn body_probe_slot(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/guards.ts:378 bodyVisibility)
pub fn body_visibility(g: &mut crate::game::Game, o: GuardId) -> f64 { todo!() }

/// (src/game/guards.ts:391 bodyLit)
pub fn body_lit(g: &mut crate::game::Game, gd: GuardId, o: GuardId, eye: V3, dist: f64) -> bool { todo!() }

/// (src/game/guards.ts:404 loadBodyLight)
pub fn load_body_light(g: &mut crate::game::Game) { todo!() }

/// (src/game/guards.ts:411 saveBodyLight)
pub fn save_body_light(g: &mut crate::game::Game) { todo!() }

/// (src/game/guards.ts:412 resetBodyLight)
pub fn reset_body_light(g: &mut crate::game::Game) { todo!() }

/// (src/game/guards.ts:414 bodyLightSummary)
pub fn body_light_summary(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/guards.ts:422 pointInView)
pub fn point_in_view(g: &mut crate::game::Game, gd: GuardId, p: V3, range: Option<f64>, half_deg: Option<f64>) -> bool { todo!() }

/// (src/game/guards.ts:434 lightOutSeen)
pub fn light_out_seen(g: &mut crate::game::Game, gd: GuardId, e: crate::guards::LightOutSeenE) -> bool { todo!() }

/// (src/game/guards.ts:446 witnessKill)
pub fn witness_kill(g: &mut crate::game::Game, victim: GuardId, how: crate::game::DownKind, by: crate::game::DownBy) { todo!() }

/// (src/game/guards.ts:457 alertCause)
pub fn alert_cause(g: &mut crate::game::Game, gd: GuardId, seen: bool) -> crate::guards::AlertCause { todo!() }

/// (src/game/guards.ts:465 witnessSummary)
pub fn witness_summary(g: &mut crate::game::Game, gd: GuardId) -> String { todo!() }

/// (src/game/guards.ts:481 bodyPos)
pub fn body_pos(g: &mut crate::game::Game, o: GuardId) -> V3 { todo!() }

/// (src/game/guards.ts:483 nearKnownBody)
pub fn near_known_body(g: &mut crate::game::Game, p: V3, r: Option<f64>) -> Option<GuardId> { todo!() }

/// (src/game/guards.ts:489 doorTamper)
pub fn door_tamper(g: &mut crate::game::Game, d: DoorId) -> Option<crate::guards::DoorTamper> { todo!() }

/// (src/game/guards.ts:496 coverSpot)
pub fn cover_spot(g: &mut crate::game::Game, gd: GuardId, bp: V3, stand: f64) -> V3 { todo!() }

/// (src/game/guards.ts:511 fanSpot)
pub fn fan_spot(g: &mut crate::game::Game, gd: GuardId, b: GuardId, bp: V3) -> Option<V3> { todo!() }

/// (src/game/guards.ts:530 notGettingAnywhere)
pub fn not_getting_anywhere(g: &mut crate::game::Game, gd: GuardId, dt: f64, secs: f64) -> bool { todo!() }

/// (src/game/guards.ts:537 dropToSearch)
pub fn drop_to_search(g: &mut crate::game::Game, gd: GuardId, bark: Option<&str>) { todo!() }

/// (src/game/guards.ts:546 resetChase)
pub fn reset_chase(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/guards.ts:554 bodyAftermath)
pub fn body_aftermath(g: &mut crate::game::Game, gd: GuardId, dt: f64) -> Option<crate::guards::BodyAftermath> { todo!() }

/// (src/game/guards.ts:608 leadFix)
pub fn lead_fix(g: &mut crate::game::Game, from: V3, vel: V3) -> V3 { todo!() }

/// (src/game/guards.ts:631 shotOrigin)
pub fn shot_origin(g: &mut crate::game::Game, c_2: CharId) -> V3 { todo!() }

/// (src/game/guards.ts:639 clearShot)
pub fn clear_shot(g: &mut crate::game::Game, c_2: CharId, to: V3) -> bool { todo!() }

/// (src/game/guards.ts:649 paneFirst)
pub fn pane_first(g: &mut crate::game::Game, c_2: CharId, to: V3) -> bool { todo!() }

/// (src/game/guards.ts:680 lineGeom)
pub fn line_geom(g: &mut crate::game::Game, from: V3, to: V3, o: GuardId) -> Option<crate::guards::LineGeom> { todo!() }

/// (src/game/guards.ts:695 colleagueInLine)
pub fn colleague_in_line(g: &mut crate::game::Game, gd: GuardId, to: V3, except: Option<GuardId>) -> Option<GuardId> { todo!() }

/// (src/game/guards.ts:712 roundPastColleague)
pub fn round_past_colleague(g: &mut crate::game::Game, gd: GuardId, to: V3, except: Option<GuardId>, cone_per_m: Option<f64>) -> bool { todo!() }

/// (src/game/guards.ts:728 aimAhead)
pub fn aim_ahead(g: &mut crate::game::Game, c_2: CharId) -> V3 { todo!() }

/// (src/game/guards.ts:737 muzzleCheck)
pub fn muzzle_check(g: &mut crate::game::Game, gd: GuardId, to: Option<V3>, dt: f64, except: Option<GuardId>) -> Option<GuardId> { todo!() }

/// (src/game/guards.ts:752 lineStep)
pub fn line_step(g: &mut crate::game::Game, gd: GuardId, aim_at: V3, o: Option<GuardId>, dt: f64) -> crate::guards::LineStep { todo!() }

/// (src/game/guards.ts:862 shieldUp)
pub fn shield_up(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/guards.ts:866 pairOf)
pub fn pair_of(g: &mut crate::game::Game) -> Option<crate::guards::Pair> { todo!() }

/// (src/game/guards.ts:876 forgetGuardClocks)
pub fn forget_guard_clocks(g: &mut crate::game::Game) { todo!() }

/// (src/game/guards.ts:880 pairInView)
pub fn pair_in_view(g: &mut crate::game::Game, gd: GuardId, p: &crate::guards::Pair) -> bool { todo!() }

/// (src/game/guards.ts:901 pairSightRate)
pub fn pair_sight_rate(g: &mut crate::game::Game, gd: GuardId, p: &crate::guards::Pair, dist: f64) -> f64 { todo!() }

/// (src/game/guards.ts:916 forgetHeldMeter)
pub fn forget_held_meter(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/guards.ts:917 heldVisibility)
pub fn held_visibility(g: &mut crate::game::Game, p: &crate::guards::Pair) -> f64 { todo!() }

/// (src/game/guards.ts:924 standoffPerceive)
pub fn standoff_perceive(g: &mut crate::game::Game, gd: GuardId, dt: Option<f64>) -> bool { todo!() }

/// (src/game/guards.ts:954 hostageCall)
pub fn hostage_call(g: &mut crate::game::Game, gd: GuardId, p: crate::guards::Pair) { todo!() }

/// (src/game/guards.ts:971 pickSide)
pub fn pick_side(g: &mut crate::game::Game, gd: GuardId, p: crate::guards::Pair) -> f64 { todo!() }

/// (src/game/guards.ts:981 axisToSegment)
pub fn axis_to_segment(g: &mut crate::game::Game, from: V3, to: V3, o: GuardId) -> f64 { todo!() }

/// (src/game/guards.ts:991 flankShot)
pub fn flank_shot(g: &mut crate::game::Game, gd: GuardId, p: crate::guards::Pair) -> crate::guards::FlankShot { todo!() }

/// (src/game/guards.ts:1024 standoffBark)
pub fn standoff_bark(g: &mut crate::game::Game, gd: GuardId, moment: crate::guards::StandoffMoment, force: Option<crate::guards::StandoffBarkForce>, held: Option<GuardId>) { todo!() }

/// (src/game/guards.ts:1039 stepAlongPath)
pub fn step_along_path(g: &mut crate::game::Game, gd: GuardId, dt: f64, speed: f64) -> bool { todo!() }

/// (src/game/guards.ts:1057 orbitGoalFor)
pub fn orbit_goal_for(g: &mut crate::game::Game, gd: GuardId, s: GuardsStandoffId, p: crate::guards::Pair) -> Option<V3> { todo!() }

/// (src/game/guards.ts:1100 approachPoint)
pub fn approach_point(g: &mut crate::game::Game, gd: GuardId, c_2: V3, need_sight: Option<bool>) -> Option<V3> { todo!() }

/// (src/game/guards.ts:1118 retreatGoal)
pub fn retreat_goal(g: &mut crate::game::Game, gd: GuardId, p: crate::guards::Pair) -> Option<V3> { todo!() }

/// (src/game/guards.ts:1139 standoffAlert)
pub fn standoff_alert(g: &mut crate::game::Game, gd: GuardId, dt: f64, p: crate::guards::Pair) -> crate::guards::StandoffAlert { todo!() }

/// (src/game/guards.ts:1207 standoffUpkeep)
pub fn standoff_upkeep(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::guards::Pair> { todo!() }

/// (src/game/guards.ts:1230 hostageHitBark)
pub fn hostage_hit_bark(g: &mut crate::game::Game, shooter: GuardId, victim: GuardId) { todo!() }

/// (src/game/guards.ts:1232 standoffSummary)
pub fn standoff_summary(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/guards.ts:1242 canSee)
pub fn can_see(g: &mut crate::game::Game, gd: GuardId) -> crate::guards::CanSee { todo!() }

/// (src/game/guards.ts:1248 canSeeChar)
pub fn can_see_char(g: &mut crate::game::Game, gd: GuardId, pc: CharId, is_player: bool) -> crate::guards::CanSeeChar { todo!() }

/// (src/game/guards.ts:1278 eyeReach)
pub fn eye_reach(g: &mut crate::game::Game, lit: f64) -> f64 { todo!() }

/// (src/game/guards.ts:1284 reachFor)
pub fn reach_for(g: &mut crate::game::Game, gd: GuardId, pc: CharId, is_player: bool, eye: V3) -> f64 { todo!() }

/// (src/game/guards.ts:1296 farFaint)
pub fn far_faint(g: &mut crate::game::Game, gd: GuardId, dist: f64, was: f64) -> bool { todo!() }

/// (src/game/guards.ts:1304 distTerm)
pub fn dist_term(g: &mut crate::game::Game, dist: f64) -> f64 { todo!() }

/// (src/game/guards.ts:1312 teamThrough)
pub fn team_through(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/guards.ts:1313 eyeThrough)
pub fn eye_through(g: &mut crate::game::Game, gd: GuardId) -> f64 { todo!() }

/// (src/game/guards.ts:1316 eyeBlocked)
pub fn eye_blocked(g: &mut crate::game::Game, gd: GuardId, a: V3, b: V3) -> bool { todo!() }

/// (src/game/guards.ts:1330 eyeContact)
pub fn eye_contact(g: &mut crate::game::Game, e: crate::guards::Eye, pc: CharId, is_player: bool, blind: bool, smoke_trans: f64, through: Option<f64>) -> crate::guards::EyeContact { todo!() }

/// (src/game/guards.ts:1375 strandedAt)
pub fn stranded_at(g: &mut crate::game::Game, x: f64, z: f64) -> bool { todo!() }

/// (src/game/guards.ts:1383 legClear)
pub fn leg_clear(g: &mut crate::game::Game, from: V3, to: V3) -> bool { todo!() }

/// (src/game/guards.ts:1394 freeCellNear)
pub fn free_cell_near(g: &mut crate::game::Game, from: V3, reach: f64, test: crate::guards::FreeCellNearTest) -> Option<V3> { todo!() }

/// (src/game/guards.ts:1408 unstickSnap)
pub fn unstick_snap(g: &mut crate::game::Game, gd: GuardId, to: Option<V3>, why: &str) { todo!() }

/// (src/game/guards.ts:1417 liveLeg)
pub fn live_leg(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::guards::UnstickLeg> { todo!() }

/// (src/game/guards.ts:1425 unstickBegin)
pub fn unstick_begin(g: &mut crate::game::Game, gd: GuardId) -> crate::guards::UnstickBegin { todo!() }

/// (src/game/guards.ts:1443 unstickStep)
pub fn unstick_step(g: &mut crate::game::Game, gd: GuardId, dt: f64) -> crate::guards::UnstickStep { todo!() }

/// (src/game/guards.ts:1459 unstickFirst)
pub fn unstick_first(g: &mut crate::game::Game, gd: GuardId, dt: f64) -> bool { todo!() }

/// (src/game/guards.ts:1469 goTo)
pub fn go_to(g: &mut crate::game::Game, gd: GuardId, goal: V3, force: Option<bool>) { todo!() }

/// (src/game/guards.ts:1548 litPlace)
pub fn lit_place(g: &mut crate::game::Game, gd: GuardId, a: Option<OrdersAssignmentId>, tgt: Option<GuardId>, pt: Option<V3>) -> bool { todo!() }

/// (src/game/guards.ts:1565 looksIntoDark)
pub fn looks_into_dark(g: &mut crate::game::Game, gd: GuardId, a: Option<OrdersAssignmentId>) -> bool { todo!() }

/// (src/game/guards.ts:1576 placeLitSeen)
pub fn place_lit_seen(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/guards.ts:1581 beamFan)
pub fn beam_fan(d: V3) -> Vec<V3> { todo!() }

/// (src/game/guards.ts:1591 rayEnd)
pub fn ray_end(g: &mut crate::game::Game, o: V3, r: V3, m: f64) -> f64 { todo!() }

/// (src/game/guards.ts:1608 beamWalled)
pub fn beam_walled(g: &mut crate::game::Game, o: V3, fan: &[V3], m: f64, held: bool) -> bool { todo!() }

/// (src/game/guards.ts:1615 beamWalledAt)
pub fn beam_walled_at(g: &mut crate::game::Game, o: V3, d: V3, m: f64, held: Option<bool>) -> bool { todo!() }

/// (src/game/guards.ts:1616 beamRayEnd)
pub fn beam_ray_end(g: &mut crate::game::Game, o: V3, r: V3, m: f64) -> f64 { todo!() }

/// (src/game/guards.ts:1618 burnsFrom)
pub fn burns_from(g: &mut crate::game::Game, held: V3, p: V3) -> V3 { todo!() }

/// (src/game/guards.ts:1620 beamLens)
pub fn beam_lens(g: &mut crate::game::Game, gd: GuardId) -> Option<V3> { todo!() }

/// (src/game/guards.ts:1630 wallDark)
pub fn wall_dark(g: &mut crate::game::Game, gd: GuardId, held: Option<V3>) -> bool { todo!() }

/// (src/game/guards.ts:1649 torchLow)
pub fn torch_low(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/guards.ts:1657 torchLowSeenOf)
pub fn torch_low_seen_of(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/guards.ts:1659 wallDarkSeen)
pub fn wall_dark_seen(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/guards.ts:1662 wallPutOut)
pub fn wall_put_out(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/guards.ts:1664 lightRuled)
pub fn light_ruled(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/guards.ts:1667 darkOnLitMan)
pub fn dark_on_lit_man(g: &mut crate::game::Game, gd: GuardId, tgt: GuardId) -> bool { todo!() }

/// (src/game/guards.ts:1675 gunOnLitMan)
pub fn gun_on_lit_man(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/guards.ts:1682 teamLit)
pub fn team_lit(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/guards.ts:1694 teamPaced)
pub fn team_paced(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/guards.ts:1696 teamPace)
pub fn team_pace(g: &mut crate::game::Game, gd: GuardId, speed: f64) -> f64 { todo!() }

/// (src/game/guards.ts:1700 teamLegs)
pub fn team_legs(g: &mut crate::game::Game, gd: GuardId, speed: f64) -> f64 { todo!() }

/// (src/game/guards.ts:1703 teamTurn)
pub fn team_turn(g: &mut crate::game::Game, gd: GuardId, to: f64, dt: f64) -> f64 { todo!() }

/// (src/game/guards.ts:1709 teamLean)
pub fn team_lean(g: &mut crate::game::Game, gd: GuardId, was: f64, dt: f64) { todo!() }

/// (src/game/guards.ts:1711 followPath)
pub fn follow_path(g: &mut crate::game::Game, gd: GuardId, dt: f64, speed: f64) -> bool { todo!() }

/// (src/game/guards.ts:1802 doorCells)
pub fn door_cells(g: &mut crate::game::Game) -> JsMap<crate::Num, DoorId> { todo!() }

/// (src/game/guards.ts:1818 floodFrom)
pub fn flood_from(g: &mut crate::game::Game, from: V3, max_m: f64, keyed: Option<bool>) -> crate::guards::FloodFrom { todo!() }

/// (src/game/guards.ts:1854 dealSpots)
pub fn deal_spots(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/guards.ts:1906 searchPlanFor)
pub fn search_plan_for(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/guards.ts:1921 searchProbes)
pub fn search_probes(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/guards.ts:1944 scoreSpots)
pub fn score_spots(g: &mut crate::game::Game, gd: GuardId, walk: Option<&[f32]>) { todo!() }

/// (src/game/guards.ts:1954 spotTaken)
pub fn spot_taken(g: &mut crate::game::Game, gd: GuardId, s: &crate::guards::SearchSpot) -> bool { todo!() }

/// (src/game/guards.ts:1963 pickNextSpot)
pub fn pick_next_spot(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/guards.ts:1974 searchSummary)
pub fn search_summary(g: &mut crate::game::Game, gd: GuardId) -> String { todo!() }

/// (src/game/guards.ts:2001 frameOf)
pub fn frame_of(g: &mut crate::game::Game, d: DoorId) -> crate::doorway::RelF { todo!() }

/// (src/game/guards.ts:2003 crossesDoor)
pub fn crosses_door(g: &mut crate::game::Game, a: V3, b: V3, d: DoorId) -> Option<crate::guards::CrossesDoor> { todo!() }

/// (src/game/guards.ts:2013 noteDoorCrossings)
pub fn note_door_crossings(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/guards.ts:2020 pieDoorway)
pub fn pie_doorway(g: &mut crate::game::Game, gd: GuardId, dt: f64, speed: f64, eyes_on: bool) -> crate::guards::PieDoorway { todo!() }

/// (src/game/guards.ts:2058 pathCrosses)
pub fn path_crosses(g: &mut crate::game::Game, gd: GuardId, d: DoorId) -> bool { todo!() }

/// (src/game/guards.ts:2060 refreshCrossing)
pub fn refresh_crossing(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/guards.ts:2065 bendApproach)
pub fn bend_approach(g: &mut crate::game::Game, gd: GuardId, d: DoorId, sg: f64) { todo!() }

/// (src/game/guards.ts:2087 chaseFix)
pub fn chase_fix(g: &mut crate::game::Game, gd: GuardId, dt: f64, pace: f64) -> crate::guards::ChaseFix { todo!() }

/// (src/game/guards.ts:2109 updateGuard)
pub fn update_guard(g: &mut crate::game::Game, gd: GuardId, dt: f64) { todo!() }

/// (src/game/guards.ts:2564 collideGuard)
pub fn collide_guard(g: &mut crate::game::Game, gd: GuardId, p0x: f64, p0z: f64) { todo!() }

/// (src/game/guards.ts:2590 sweepStatics)
pub fn sweep_statics(g: &mut crate::game::Game, c_2: CharId, fx: f64, fz: f64) { todo!() }

/// (src/game/guards.ts:2614 lightEstimateAt)
pub fn light_estimate_at(g: &mut crate::game::Game, p: V3, skip_owner: Option<&mut dyn FnMut(&mut crate::game::Game, f64) -> bool>, steady_only: Option<bool>, field: Option<&mut crate::sense::LightField>, moon: Option<bool>, leave: Option<f64>) -> f64 { todo!() }

/// (src/game/guards.ts:2619 lightEstimateIn)
pub fn light_estimate_in(g: &mut crate::game::Game, p: V3, skip_owner: Option<&mut dyn FnMut(&mut crate::game::Game, f64) -> bool>, steady_only: bool, field: Option<&mut crate::sense::LightField>, moon: bool, leave: f64) -> f64 { todo!() }

/// (src/game/guards.ts:2630 lightField)
pub fn light_field(g: &mut crate::game::Game) -> Option<crate::sense::LightField> { todo!() }

/// (src/game/guards.ts:2636 lampAsked)
pub fn lamp_asked(g: &mut crate::game::Game, l: LightId, skip_owner: Option<&mut dyn FnMut(&mut crate::game::Game, f64) -> bool>, steady_only: bool) -> bool { todo!() }

/// (src/game/guards.ts:2642 rowsOf)
pub fn rows_of(g: &mut crate::game::Game, steady_only: bool, leave: f64) -> crate::scene_collision::LampRows { todo!() }

/// (src/game/guards.ts:2648 writeLamps)
pub fn write_lamps(g: &mut crate::game::Game, p: Option<V3>, skip_owner: Option<&mut dyn FnMut(&mut crate::game::Game, f64) -> bool>, steady_only: bool) -> bool { todo!() }

/// (src/game/guards.ts:2660 CARRIED)
pub fn carried() -> bool { todo!() }

/// (src/game/guards.ts:2664 steadyLamps)
pub fn steady_lamps(g: &mut crate::game::Game) -> crate::guards::SteadyLamps { todo!() }

/// (src/game/guards.ts:2671 lampsOn)
pub fn lamps_on(g: &mut crate::game::Game, p: V3, skip_owner: Option<&mut dyn FnMut(&mut crate::game::Game, f64) -> bool>, steady_only: bool, acc: &mut Vec<f64>, field: Option<crate::sense::LightField>, leave: f64) { todo!() }

/// (src/game/guards.ts:2677 meterOf)
pub fn meter_of(g: &mut crate::game::Game, c_2: CharId, prefix: &str) -> f64 { todo!() }

/// (src/game/guards.ts:2687 unitLightMeter)
pub fn unit_light_meter(g: &mut crate::game::Game, gd: GuardId, dt: f64) { todo!() }

/// (src/game/guards.ts:2697 acquireFoe)
pub fn acquire_foe(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/guards.ts:2741 rigOf)
pub fn rig_of(g: &mut crate::game::Game, gd: GuardId) -> crate::guards::RigOf { todo!() }

/// (src/game/guards.ts:2744 longRigOf)
pub fn long_rig_of(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::guards::LongRigOf> { todo!() }

/// (src/game/guards.ts:2751 handCarry)
pub fn hand_carry(g: &mut crate::game::Game, gd: GuardId) -> crate::guards::HandCarry { todo!() }

/// (src/game/guards.ts:2757 gunBack)
pub fn gun_back(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/guards.ts:2761 inHand)
pub fn in_hand(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::guards::InHand> { todo!() }

/// (src/game/guards.ts:2770 startReloadBeat)
pub fn start_reload_beat(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/guards.ts:2779 tickReloadBeat)
pub fn tick_reload_beat(g: &mut crate::game::Game, gd: GuardId, dt: f64) { todo!() }

/// (src/game/guards.ts:2812 burstTrigger)
pub fn burst_trigger(g: &mut crate::game::Game, gd: GuardId, dt: f64, aim_at: V3, start_ok: bool, round_ok: bool, except: Option<GuardId>) { todo!() }

/// (src/game/guards.ts:2851 burstSummary)
pub fn burst_summary(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/guards.ts:2861 attachFlashlight)
pub fn attach_flashlight(g: &mut crate::game::Game, gd: GuardId, dt: Option<f64>) { todo!() }

/// (src/game/guards.ts:2866 poseFlashlight)
pub fn pose_flashlight(g: &mut crate::game::Game, gd: GuardId, dt: f64) -> Option<V3> { todo!() }

/// (src/game/guards.ts:2902 onScreen)
pub fn on_screen(g: &mut crate::game::Game, p: V3) -> bool { todo!() }

/// (src/game/guards.ts:2906 inView)
pub fn in_view(vp: Option<&crate::guards::InViewVp>, p: V3) -> bool { todo!() }

/// (src/game/guards.ts:2914 bubbleSecs)
pub fn bubble_secs(text: &str) -> f64 { todo!() }

/// (src/game/guards.ts:2919 headOf)
pub fn head_of(g: &mut crate::game::Game, gd: GuardId) -> V3 { todo!() }

/// (src/game/guards.ts:2921 say)
pub fn say(g: &mut crate::game::Game, gd: GuardId, text: &str, radio: Option<bool>, force: Option<bool>, row: Option<bool>) { todo!() }

/// (src/game/guards.ts:2935 resetGuards)
pub fn reset_guards(g: &mut crate::game::Game) { todo!() }

/// (src/game/guards.ts:2965 escalationOf)
pub fn escalation_of(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/guards.ts:2966 escalationName)
pub fn escalation_name(l: f64) -> String { todo!() }

/// (src/game/guards.ts:2969 escalationSummary)
pub fn escalation_summary(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/guards.ts:2981 clearingSummary)
pub fn clearing_summary(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/guards.ts:2985 spokesman)
pub fn spokesman(g: &mut crate::game::Game, pos: Option<V3>, except: Option<GuardId>) -> Option<GuardId> { todo!() }

/// (src/game/guards.ts:2999 raiseEscalation)
pub fn raise_escalation(g: &mut crate::game::Game, to: f64, except: Option<GuardId>, cause: Option<crate::guards::RaiseEscalationCause>, another: Option<bool>) { todo!() }

/// (src/game/guards.ts:3014 standDownEscalation)
pub fn stand_down_escalation(g: &mut crate::game::Game) { todo!() }

/// (src/game/guards.ts:3039 offNet)
pub fn off_net(g: &mut crate::game::Game, x: GuardId) -> bool { todo!() }

/// (src/game/guards.ts:3041 rollcallState)
pub fn rollcall_state<'g>(g: &'g mut crate::game::Game) -> &'g mut crate::guards::Rollcall { todo!() }

/// (src/game/guards.ts:3045 rollcallInterval)
pub fn rollcall_interval(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/guards.ts:3048 radioCaller)
pub fn radio_caller(g: &mut crate::game::Game) -> Option<GuardId> { todo!() }

/// (src/game/guards.ts:3057 checkerFor)
pub fn checker_for(g: &mut crate::game::Game, m: GuardId) -> Option<GuardId> { todo!() }

/// (src/game/guards.ts:3060 stillLooking)
pub fn still_looking(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/guards.ts:3062 pickChecker)
pub fn pick_checker(g: &mut crate::game::Game, m: GuardId, prefer: Option<GuardId>) -> Option<GuardId> { todo!() }

/// (src/game/guards.ts:3071 sendCheckOn)
pub fn send_check_on(g: &mut crate::game::Game, gd: GuardId, m: GuardId, line: &str) { todo!() }

/// (src/game/guards.ts:3083 checkOnTick)
pub fn check_on_tick(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/guards.ts:3094 advanceCheckOn)
pub fn advance_check_on(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/guards.ts:3102 checkOnDone)
pub fn check_on_done(g: &mut crate::game::Game, gd: GuardId, m: GuardId, capped: bool) { todo!() }

/// (src/game/guards.ts:3113 updateRollcall)
pub fn update_rollcall(g: &mut crate::game::Game, dt: f64) { todo!() }

/// (src/game/guards.ts:3182 partnerWatch)
pub fn partner_watch(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/guards.ts:3214 planClears)
pub fn plan_clears(g: &mut crate::game::Game, force: Option<bool>, cause: Option<crate::guards::PlanClearsCause>) -> bool { todo!() }

/// (src/game/guards.ts:3240 clearQueueSummary)
pub fn clear_queue_summary(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/guards.ts:3248 forceClear)
pub fn force_clear(g: &mut crate::game::Game) { todo!() }

/// (src/game/guards.ts:3257 stepDownEscalation)
pub fn step_down_escalation(g: &mut crate::game::Game) { todo!() }

/// (src/game/guards.ts:3268 updateEscalation)
pub fn update_escalation(g: &mut crate::game::Game, dt: f64) { todo!() }

/// (src/game/guards.ts:3292 superviseLockdown)
pub fn supervise_lockdown(g: &mut crate::game::Game, dt: f64, living: f64) { todo!() }

/// (src/game/guards.ts:3316 assignLockdown)
pub fn assign_lockdown(g: &mut crate::game::Game, redeal: bool, except: Option<GuardId>) { todo!() }

/// (src/game/guards.ts:3336 clearLockdown)
pub fn clear_lockdown(g: &mut crate::game::Game) { todo!() }

/// (src/game/guards.ts:3337 routeLength)
pub fn route_length(g: &mut crate::game::Game, gd: GuardId) -> f64 { todo!() }

/// (src/game/guards.ts:3346 followLeader)
pub fn follow_leader(g: &mut crate::game::Game, gd: GuardId, dt: f64, walk: f64) -> bool { todo!() }

/// (src/game/guards.ts:3374 holdPost)
pub fn hold_post(g: &mut crate::game::Game, gd: GuardId, dt: f64, walk: f64) -> bool { todo!() }

/// (src/game/guards.ts:3394 lockdownDoors)
pub fn lockdown_doors(g: &mut crate::game::Game, gd: GuardId, calm: Option<bool>) { todo!() }

/// (src/game/guards.ts:3421 tension)
pub fn tension(g: &mut crate::game::Game, theirs: Option<bool>) -> f64 { todo!() }

/// (src/game/guards.ts:3476 spawnUnit)
pub fn spawn_unit(g: &mut crate::game::Game, o: &crate::guards::UnitOpts) -> Result<GuardId, String> { todo!() }

/// (src/game/guards.ts:3501 releaseUnit)
pub fn release_unit(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/guards.ts:3505 despawnUnit)
pub fn despawn_unit(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/guards.ts:3525 despawnUnits)
pub fn despawn_units(g: &mut crate::game::Game) { todo!() }

/// (src/game/guards.ts:3527 units)
pub fn units(g: &mut crate::game::Game) -> Vec<GuardId> { todo!() }

/// (src/game/guards.ts:3537 engagementOf)
pub fn engagement_of(g: &mut crate::game::Game) -> Option<crate::guards::Engagement> { todo!() }

/// (src/game/guards.ts:3567 nameOfId)
pub fn name_of_id(g: &mut crate::game::Game, id_2: f64) -> String { todo!() }

/// (src/game/guards.ts:3569 endEngagement)
pub fn end_engagement(g: &mut crate::game::Game, outcome: crate::guards::EndEngagementOutcome) { todo!() }

/// (src/game/guards.ts:3576 tickEngagement)
pub fn tick_engagement(g: &mut crate::game::Game) { todo!() }


/// INVENTED, beside game::guard_set_path: `gd.pathGoal = goal` and what says it is a new object (guards.ts:1163 `gd.pathGoal !== S.orbitGoal`)
pub fn guard_set_path_goal(g: &mut crate::game::Game, gd: GuardId, goal: Option<V3>) { todo!() }

