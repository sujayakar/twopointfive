#![allow(unused_variables)]
//! Every function and method of `roomfight`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/roomfight.ts:287 drilledOn)
pub fn drilled_on(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/roomfight.ts:289 drilledK)
pub fn drilled_k(g: &mut crate::game::Game, gd: GuardId) -> f64 { todo!() }

/// (src/game/roomfight.ts:346 flat)
pub fn flat(a: V3, b: V3) -> f64 { todo!() }

/// (src/game/roomfight.ts:347 pick)
pub fn pick(g: &mut crate::game::Game, r: [f64; 2]) -> f64 { todo!() }

/// (src/game/roomfight.ts:348 eyeOf)
pub fn eye_of(g: &mut crate::game::Game, c_2: CharId) -> V3 { todo!() }

/// (src/game/roomfight.ts:349 chestOf)
pub fn chest_of(g: &mut crate::game::Game, c_2: CharId) -> V3 { todo!() }

/// (src/game/roomfight.ts:350 pairKey)
pub fn pair_key(a: f64, b: f64) -> String { todo!() }

/// (src/game/roomfight.ts:354 pairSmoke)
pub fn pair_smoke(g: &mut crate::game::Game, a: CharId, b: CharId) -> f64 { todo!() }

/// (src/game/roomfight.ts:362 lampGlare)
pub fn lamp_glare(g: &mut crate::game::Game, eye: CharId, o: GuardId) -> f64 { todo!() }

/// (src/game/roomfight.ts:370 fightLive)
pub fn fight_live(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/roomfight.ts:373 closeLane)
pub fn close_lane(g: &mut crate::game::Game, clear: f64, t: f64, l: f64) -> f64 { todo!() }

/// (src/game/roomfight.ts:400 RoomFight.samSpeedOf)
pub fn room_fight_sam_speed_of(g: &mut crate::game::Game, ch: CharId) -> f64 { todo!() }

/// (src/game/roomfight.ts:401 RoomFight.constructor)
pub fn room_fight_new(g: &mut crate::game::Game) -> crate::roomfight::RoomFight { todo!() }

/// (src/game/roomfight.ts:404 RoomFight.on)
pub fn room_fight_on(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/roomfight.ts:408 RoomFight.note)
pub fn room_fight_note(g: &mut crate::game::Game, gd: Option<GuardId>, what: &str) { todo!() }

/// (src/game/roomfight.ts:412 RoomFight.engaged)
pub fn room_fight_engaged(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/roomfight.ts:415 RoomFight.noShot)
pub fn room_fight_no_shot(g: &mut crate::game::Game, gd: GuardId) -> f64 { todo!() }

/// (src/game/roomfight.ts:417 RoomFight.busy)
pub fn room_fight_busy(g: &mut crate::game::Game, gd: GuardId, secs: f64) -> bool { todo!() }

/// (src/game/roomfight.ts:419 RoomFight.contact)
pub fn room_fight_contact(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/roomfight.ts:421 RoomFight.teamSees)
pub fn room_fight_team_sees(g: &mut crate::game::Game, h: GuardId) -> bool { todo!() }

/// (src/game/roomfight.ts:422 RoomFight.seenBy)
pub fn room_fight_seen_by(g: &mut crate::game::Game, h: GuardId) -> Vec<GuardId> { todo!() }

/// (src/game/roomfight.ts:429 RoomFight.sightOn)
pub fn room_fight_sight_on(g: &mut crate::game::Game, viewer: GuardId, target: GuardId) -> crate::roomfight::RoomFightSightOn { todo!() }

/// (src/game/roomfight.ts:436 RoomFight.rateOn)
pub fn room_fight_rate_on(g: &mut crate::game::Game, viewer: GuardId, target: GuardId, dist: f64, known: Option<bool>) -> f64 { todo!() }

/// (src/game/roomfight.ts:441 RoomFight.hotFor)
pub fn room_fight_hot_for(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/roomfight.ts:442 RoomFight.hotOf)
pub fn room_fight_hot_of(g: &mut crate::game::Game, gd: GuardId) -> f64 { todo!() }

/// (src/game/roomfight.ts:444 RoomFight.watchOf)
pub fn room_fight_watch_of(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::roomfight::RoomFightWatchOf> { todo!() }

/// (src/game/roomfight.ts:449 RoomFight.opOf)
pub fn room_fight_op_of(g: &mut crate::game::Game, gd: GuardId) -> Option<RoomfightOpId> { todo!() }

/// (src/game/roomfight.ts:451 RoomFight.short)
pub fn room_fight_short(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/roomfight.ts:452 RoomFight.summary)
pub fn room_fight_summary(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/roomfight.ts:461 RoomFight.tick)
pub fn room_fight_tick(g: &mut crate::game::Game, dt: f64) { todo!() }

/// (src/game/roomfight.ts:525 RoomFight.stop)
pub fn room_fight_stop(g: &mut crate::game::Game) { todo!() }

/// (src/game/roomfight.ts:539 RoomFight.reset)
pub fn room_fight_reset(g: &mut crate::game::Game) { todo!() }

/// (src/game/roomfight.ts:542 RoomFight.smoke)
pub fn room_fight_smoke(g: &mut crate::game::Game) { todo!() }

/// (src/game/roomfight.ts:559 RoomFight.roomAt)
pub fn room_fight_room_at(g: &mut crate::game::Game, p: V3) -> Option<RoomfightRoomId> { todo!() }

/// (src/game/roomfight.ts:593 RoomFight.inRoom)
pub fn room_fight_in_room(g: &mut crate::game::Game, r: Option<RoomfightRoomId>, p: V3) -> bool { todo!() }

/// (src/game/roomfight.ts:594 RoomFight.freeNear)
pub fn room_fight_free_near(g: &mut crate::game::Game, p: V3) -> Option<V3> { todo!() }

/// (src/game/roomfight.ts:601 RoomFight.adopt)
pub fn room_fight_adopt(g: &mut crate::game::Game, gd: GuardId) -> RoomfightDefId { todo!() }

/// (src/game/roomfight.ts:610 RoomFight.tickDef)
pub fn room_fight_tick_def(g: &mut crate::game::Game, d: RoomfightDefId, dt: f64, k: f64) { todo!() }

/// (src/game/roomfight.ts:638 RoomFight.evidence)
pub fn room_fight_evidence(g: &mut crate::game::Game, d: RoomfightDefId, dt: f64) { todo!() }

/// (src/game/roomfight.ts:658 RoomFight.told)
pub fn room_fight_told(g: &mut crate::game::Game, gd: GuardId, at: V3) -> bool { todo!() }

/// (src/game/roomfight.ts:665 RoomFight.entryToward)
pub fn room_fight_entry_toward(g: &mut crate::game::Game, d: RoomfightDefId, at: V3) -> f64 { todo!() }

/// (src/game/roomfight.ts:670 RoomFight.chooseWatch)
pub fn room_fight_choose_watch(g: &mut crate::game::Game, d: RoomfightDefId) { todo!() }

/// (src/game/roomfight.ts:689 RoomFight.applyWatch)
pub fn room_fight_apply_watch(g: &mut crate::game::Game, d: RoomfightDefId, dt: f64) { todo!() }

/// (src/game/roomfight.ts:703 RoomFight.reactions)
pub fn room_fight_reactions(g: &mut crate::game::Game, d: RoomfightDefId) { todo!() }

/// (src/game/roomfight.ts:723 RoomFight.blind)
pub fn room_fight_blind(g: &mut crate::game::Game, d: RoomfightDefId, k: f64) { todo!() }

/// (src/game/roomfight.ts:741 RoomFight.drilled)
pub fn room_fight_drilled(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/roomfight.ts:744 RoomFight.lost)
pub fn room_fight_lost(g: &mut crate::game::Game, d: RoomfightDefId) { todo!() }

/// (src/game/roomfight.ts:764 RoomFight.outOfBuilding)
pub fn room_fight_out_of_building(g: &mut crate::game::Game, p: V3) -> bool { todo!() }

/// (src/game/roomfight.ts:773 RoomFight.threatOf)
pub fn room_fight_threat_of(g: &mut crate::game::Game, d: RoomfightDefId) -> V3 { todo!() }

/// (src/game/roomfight.ts:782 RoomFight.bolt)
pub fn room_fight_bolt(g: &mut crate::game::Game, gd: GuardId, clear_secs: f64) -> Option<bool> { todo!() }

/// (src/game/roomfight.ts:791 RoomFight.breakOff)
pub fn room_fight_break_off(g: &mut crate::game::Game, d: RoomfightDefId) { todo!() }

/// (src/game/roomfight.ts:852 RoomFight.tickMove)
pub fn room_fight_tick_move(g: &mut crate::game::Game, d: RoomfightDefId, dt: f64) { todo!() }

/// (src/game/roomfight.ts:876 RoomFight.standOn)
pub fn room_fight_stand_on(g: &mut crate::game::Game, d: RoomfightDefId, set: bool) { todo!() }

/// (src/game/roomfight.ts:885 RoomFight.laneStop)
pub fn room_fight_lane_stop(g: &mut crate::game::Game, gd: GuardId, look: V3) -> bool { todo!() }

/// (src/game/roomfight.ts:894 RoomFight.plant)
pub fn room_fight_plant(g: &mut crate::game::Game, d: RoomfightDefId, why: &str) { todo!() }

/// (src/game/roomfight.ts:908 RoomFight.rally)
pub fn room_fight_rally(g: &mut crate::game::Game, d: RoomfightDefId) { todo!() }

/// (src/game/roomfight.ts:935 RoomFight.tickFree)
pub fn room_fight_tick_free(g: &mut crate::game::Game, d: RoomfightDefId) { todo!() }

/// (src/game/roomfight.ts:944 RoomFight.newOp)
pub fn room_fight_new_op(g: &mut crate::game::Game, gd: GuardId) -> RoomfightOpId { todo!() }

/// (src/game/roomfight.ts:950 RoomFight.friendInLine)
pub fn room_fight_friend_in_line(g: &mut crate::game::Game, from: V3, to: V3, self_: GuardId, sam_still: Option<bool>) -> Option<crate::roomfight::RoomFightFriendInLine> { todo!() }

/// (src/game/roomfight.ts:964 RoomFight.samInLine)
pub fn room_fight_sam_in_line(g: &mut crate::game::Game, o: RoomfightOpId, to: V3) -> bool { todo!() }

/// (src/game/roomfight.ts:969 RoomFight.speak)
pub fn room_fight_speak(g: &mut crate::game::Game, o: RoomfightOpId, text: &str, gap: Option<f64>) { todo!() }

/// (src/game/roomfight.ts:971 RoomFight.coverPasses)
pub fn room_fight_cover_passes(g: &mut crate::game::Game, o: RoomfightOpId, h: GuardId) -> bool { todo!() }

/// (src/game/roomfight.ts:984 RoomFight.tickOp)
pub fn room_fight_tick_op(g: &mut crate::game::Game, o: RoomfightOpId, dt: f64) { todo!() }

/// (src/game/roomfight.ts:1097 RoomFight.point)
pub fn room_fight_point(g: &mut crate::game::Game, o: RoomfightOpId, to: V3, dt: f64, speed: f64) -> f64 { todo!() }

/// (src/game/roomfight.ts:1112 RoomFight.cue)
pub fn room_fight_cue(g: &mut crate::game::Game, o: RoomfightOpId) { todo!() }

/// (src/game/roomfight.ts:1138 RoomFight.doorShot)
pub fn room_fight_door_shot(g: &mut crate::game::Game, o: RoomfightOpId, eye: V3, gone: Option<GuardId>, arc: Option<f64>, a: Option<OrdersAssignmentId>, only: Option<DoorId>) -> Option<crate::roomfight::RoomFightDoorShot> { todo!() }

/// (src/game/roomfight.ts:1155 RoomFight.holdShort)
pub fn room_fight_hold_short(g: &mut crate::game::Game, o: RoomfightOpId) { todo!() }

/// (src/game/roomfight.ts:1164 RoomFight.letGo)
pub fn room_fight_let_go(g: &mut crate::game::Game, o: RoomfightOpId) { todo!() }

/// (src/game/roomfight.ts:1174 RoomFight.walkOn)
pub fn room_fight_walk_on(g: &mut crate::game::Game, o: RoomfightOpId) { todo!() }

/// (src/game/roomfight.ts:1180 RoomFight.restore)
pub fn room_fight_restore(g: &mut crate::game::Game, o: RoomfightOpId) { todo!() }

/// (src/game/roomfight.ts:1182 RoomFight.stepTo)
pub fn room_fight_step_to(g: &mut crate::game::Game, o: RoomfightOpId, p: V3, door: bool, patch: crate::roomfight::RoomFightStepToPatch) { todo!() }

/// (src/game/roomfight.ts:1189 RoomFight.handBack)
pub fn room_fight_hand_back(g: &mut crate::game::Game, o: RoomfightOpId) { todo!() }

/// (src/game/roomfight.ts:1194 RoomFight.funnelOf)
pub fn room_fight_funnel_of(g: &mut crate::game::Game, p: V3) -> Option<DoorId> { todo!() }

/// (src/game/roomfight.ts:1202 RoomFight.spotFree)
pub fn room_fight_spot_free(g: &mut crate::game::Game, p: V3, self_: GuardId) -> bool { todo!() }

/// (src/game/roomfight.ts:1209 RoomFight.doorway)
pub fn room_fight_doorway(g: &mut crate::game::Game, o: RoomfightOpId, dt: f64, speed: f64) { todo!() }

/// (src/game/roomfight.ts:1226 RoomFight.sideStep)
pub fn room_fight_side_step(g: &mut crate::game::Game, o: RoomfightOpId, to: V3, speed: f64, waited: Option<f64>, why: Option<&str>) -> bool { todo!() }


/// INVENTED: lastKnown === D.pt (roomfight.ts:303): D.pt_stamp == Some(g.gd[D.gd].fix_stamp) && g.gd[D.gd].last_known == Some(D.pt). Read at roomfight.ts:643, 654, 698, 879, buddy.ts:96, enemysquads.ts:432, 494, 1397, laststand.ts:274. Ask it BEFORE a call that may write the fix (standOn :876 asks before plant)
pub fn def_fix_is_pt(g: &mut crate::game::Game, d: RoomfightDefId) -> bool { todo!() }

/// INVENTED: gd.lastKnown = D.pt (roomfight.ts:699, 728, 889, 902, enemysquads.ts:1048, 1491, powerout.ts:219): last_known = Some(pt). Only if def_fix_is_pt was false: bump fix_stamp and store it in pt_stamp (the same object written again is no new fix: doze.ts:244)
pub fn def_fix_to_pt(g: &mut crate::game::Game, d: RoomfightDefId) { todo!() }

/// INVENTED: D.pt moved in place (roomfight.ts:691-695, 879, 889, 902, buddy.ts:225, pressure.ts:597, powerout.ts:218, enemysquads.ts:771, 1047, 1491): pt = to, and if def_fix_is_pt was true, last_known = Some(to) with no bump: in the TypeScript the fix is this object and moves with it
pub fn def_move_pt(g: &mut crate::game::Game, d: RoomfightDefId, to: V3) { todo!() }
