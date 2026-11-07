#![allow(unused_variables)]
//! Every function and method of `wallcharge`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/wallcharge.ts:282 nearSeg)
pub fn near_seg(q: V3, a: V3, b: V3) -> f64 { todo!() }

/// (src/game/wallcharge.ts:285 thick)
pub fn thick(g: &mut crate::game::Game, w: &crate::wallcharge::WallSpot) -> f64 { todo!() }

/// (src/game/wallcharge.ts:287 chargeAt)
pub fn charge_at(g: &mut crate::game::Game, w: &crate::wallcharge::WallSpot) -> V3 { todo!() }

/// (src/game/wallcharge.ts:289 wallPt)
pub fn wall_pt(g: &mut crate::game::Game, w: &crate::wallcharge::WallSpot, du: f64, off: f64, y: Option<f64>) -> V3 { todo!() }

/// (src/game/wallcharge.ts:292 wallN)
pub fn wall_n(w: &crate::wallcharge::WallSpot) -> V3 { todo!() }

/// (src/game/wallcharge.ts:293 wallOff)
pub fn wall_off(g: &mut crate::game::Game, w: &crate::wallcharge::WallSpot, p: V3) -> f64 { todo!() }

/// (src/game/wallcharge.ts:294 wallAlong)
pub fn wall_along(w: &crate::wallcharge::WallSpot, p: V3) -> f64 { todo!() }

/// (src/game/wallcharge.ts:295 sameWall)
pub fn same_wall(a: &crate::wallcharge::WallSpot, b: &crate::wallcharge::WallSpot) -> bool { todo!() }

/// (src/game/wallcharge.ts:298 wallChargeOn)
pub fn wall_charge_on(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/wallcharge.ts:307 skyFaces)
pub fn sky_faces(g: &mut crate::game::Game, index: f64) -> f64 { todo!() }

/// (src/game/wallcharge.ts:318 skyAside)
pub fn sky_aside<T: Clone + core::fmt::Debug>(g: &mut crate::game::Game, fn_: &mut dyn FnMut(&mut crate::game::Game) -> T) -> T { todo!() }

/// (src/game/wallcharge.ts:326 outerWall)
pub fn outer_wall(g: &mut crate::game::Game, index: f64) -> bool { todo!() }

/// (src/game/wallcharge.ts:336 chargeIndoors)
pub fn charge_indoors(g: &mut crate::game::Game, w: &crate::wallcharge::WallSpot) -> bool { todo!() }

/// (src/game/wallcharge.ts:343 safeRadius)
pub fn safe_radius(g: &mut crate::game::Game, w: &crate::wallcharge::WallSpot) -> crate::wallcharge::WallChargeAwayMarksValueR { todo!() }

/// (src/game/wallcharge.ts:352 roomBeyond)
pub fn room_beyond(g: &mut crate::game::Game, w: &crate::wallcharge::WallSpot, du: f64, t: f64) -> f64 { todo!() }

/// (src/game/wallcharge.ts:356 wallAt)
pub fn wall_at(g: &mut crate::game::Game, at: V3, from: Option<V3>) -> Option<crate::wallcharge::WallSpot> { todo!() }

/// (src/game/wallcharge.ts:382 wallTarget)
pub fn wall_target(g: &mut crate::game::Game, at: V3, base: &crate::orders::OrderTarget) -> crate::orders::OrderTarget { todo!() }

/// (src/game/wallcharge.ts:388 wallRows)
pub fn wall_rows(g: &mut crate::game::Game, t: &crate::orders::OrderTarget, page: Option<&str>, rows: &[OrdersOrderDefId]) -> Vec<OrdersOrderDefId> { todo!() }

/// (src/game/wallcharge.ts:397 wallChargeRows)
pub fn wall_charge_rows(g: &mut crate::game::Game, p: V3, busy: bool, row: &mut dyn FnMut(&mut crate::game::Game, crate::game::Interactable, crate::game::Verb, String, Option<String>, Option<crate::wallcharge::WallChargeRowsRowOver>) -> crate::game::Interactable, offer: &mut dyn FnMut(&mut crate::game::Game, crate::game::Interactable, Vec<crate::game::Interactable>)) { todo!() }

/// (src/game/wallcharge.ts:415 WallCharge.constructor)
pub fn wall_charge_new(g: &mut crate::game::Game) -> crate::wallcharge::WallCharge { todo!() }

/// (src/game/wallcharge.ts:425 WallCharge.sync)
pub fn wall_charge_sync(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/wallcharge.ts:439 WallCharge.undigit)
pub fn wall_charge_undigit(g: &mut crate::game::Game, d: OrdersOrderDefId) -> OrdersOrderDefId { todo!() }

/// (src/game/wallcharge.ts:440 WallCharge.seat)
pub fn wall_charge_seat(g: &mut crate::game::Game) { todo!() }

/// (src/game/wallcharge.ts:447 WallCharge.reset)
pub fn wall_charge_reset(g: &mut crate::game::Game) { todo!() }

/// (src/game/wallcharge.ts:449 WallCharge.stop)
pub fn wall_charge_stop(g: &mut crate::game::Game) { todo!() }

/// (src/game/wallcharge.ts:450 WallCharge.note)
pub fn wall_charge_note(g: &mut crate::game::Game, what: &str) { todo!() }

/// (src/game/wallcharge.ts:451 WallCharge.beat)
pub fn wall_charge_beat(g: &mut crate::game::Game, op: WallOpId, what: &str) { todo!() }

/// (src/game/wallcharge.ts:456 WallCharge.setCarried)
pub fn wall_charge_set_carried(g: &mut crate::game::Game, n: Option<f64>) { todo!() }

/// (src/game/wallcharge.ts:458 WallCharge.raiseCarried)
pub fn wall_charge_raise_carried(g: &mut crate::game::Game, n: f64) { todo!() }

/// (src/game/wallcharge.ts:462 WallCharge.price)
pub fn wall_charge_price(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/wallcharge.ts:463 WallCharge.affordable)
pub fn wall_charge_affordable(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/wallcharge.ts:464 WallCharge.wallsLeft)
pub fn wall_charge_walls_left(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/wallcharge.ts:466 WallCharge.rowWords)
pub fn wall_charge_row_words(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/wallcharge.ts:468 WallCharge.noneWords)
pub fn wall_charge_none_words(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/wallcharge.ts:470 WallCharge.take)
pub fn wall_charge_take(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/wallcharge.ts:478 WallCharge.giveBack)
pub fn wall_charge_give_back(g: &mut crate::game::Game, c_2: WallChargeId, why: &str) { todo!() }

/// (src/game/wallcharge.ts:485 WallCharge.hudNotes)
pub fn wall_charge_hud_notes(g: &mut crate::game::Game) -> Vec<crate::orders::OrderNote> { todo!() }

/// (src/game/wallcharge.ts:492 WallCharge.summary)
pub fn wall_charge_summary(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/wallcharge.ts:495 WallCharge.chargeOn)
pub fn wall_charge_charge_on(g: &mut crate::game::Game, w: &crate::wallcharge::WallSpot) -> Option<WallChargeId> { todo!() }

/// (src/game/wallcharge.ts:496 WallCharge.chargeOnTarget)
pub fn wall_charge_charge_on_target(g: &mut crate::game::Game, t: &crate::orders::OrderTarget) -> Option<WallChargeId> { todo!() }

/// (src/game/wallcharge.ts:499 WallCharge.spot)
pub fn wall_charge_spot(g: &mut crate::game::Game, t: &crate::orders::OrderTarget) -> Option<crate::wallcharge::WallSpot> { todo!() }

/// (src/game/wallcharge.ts:504 WallCharge.settle)
pub fn wall_charge_settle(g: &mut crate::game::Game, w: &crate::wallcharge::WallSpot) -> crate::wallcharge::WallSpot { todo!() }

/// (src/game/wallcharge.ts:511 WallCharge.openAt)
pub fn wall_charge_open_at(g: &mut crate::game::Game, w: &crate::wallcharge::WallSpot) -> bool { todo!() }

/// (src/game/wallcharge.ts:513 WallCharge.setSpot)
pub fn wall_charge_set_spot(g: &mut crate::game::Game, w: &crate::wallcharge::WallSpot) -> Option<V3> { todo!() }

/// (src/game/wallcharge.ts:520 WallCharge.farFloor)
pub fn wall_charge_far_floor(g: &mut crate::game::Game, w: &crate::wallcharge::WallSpot, furn: Option<bool>) -> Option<V3> { todo!() }

/// (src/game/wallcharge.ts:532 WallCharge.mouthAfter)
pub fn wall_charge_mouth_after(g: &mut crate::game::Game, w: &crate::wallcharge::WallSpot) -> Option<V3> { todo!() }

/// (src/game/wallcharge.ts:569 WallCharge.floorAfter)
pub fn wall_charge_floor_after(g: &mut crate::game::Game, w: &crate::wallcharge::WallSpot, m: f64, half: f64, q: V3, wall: f64, first: Option<crate::furnblast::BlastFirst>, mouth: &[V3], goes: Option<&mut dyn FnMut(&mut crate::game::Game, f64) -> bool>) -> bool { todo!() }

/// (src/game/wallcharge.ts:583 WallCharge.inFrame)
pub fn wall_charge_in_frame(g: &mut crate::game::Game, q: V3, m: Option<f64>) -> bool { todo!() }

/// (src/game/wallcharge.ts:585 WallCharge.openFoot)
pub fn wall_charge_open_foot(g: &mut crate::game::Game, w: &crate::wallcharge::WallSpot, du: f64) -> bool { todo!() }

/// (src/game/wallcharge.ts:590 WallCharge.standoffPosts)
pub fn wall_charge_standoff_posts(g: &mut crate::game::Game, w: &crate::wallcharge::WallSpot, n: f64, spot: Option<V3>, old: Option<bool>) -> Option<Vec<V3>> { todo!() }

/// (src/game/wallcharge.ts:614 WallCharge.safePosts)
pub fn wall_charge_safe_posts(g: &mut crate::game::Game, w: &crate::wallcharge::WallSpot, n: f64, spot: V3, can: Option<&mut dyn FnMut(&mut crate::game::Game, V3) -> bool>, kept: Option<&mut crate::wallcharge::WallChargeKept>) -> Option<Vec<V3>> { todo!() }

/// (src/game/wallcharge.ts:654 WallCharge.knownFoes)
pub fn wall_charge_known_foes(g: &mut crate::game::Game) -> Vec<V3> { todo!() }

/// (src/game/wallcharge.ts:671 WallCharge.underFire)
pub fn wall_charge_under_fire(g: &mut crate::game::Game, w: Option<&crate::wallcharge::WallSpot>, all: Option<&[V3]>) -> crate::wallcharge::WallChargeUnderFire { todo!() }

/// (src/game/wallcharge.ts:688 WallCharge.fireAt)
pub fn wall_charge_fire_at(g: &mut crate::game::Game, q: V3, w: Option<&crate::wallcharge::WallSpot>) -> f64 { todo!() }

/// (src/game/wallcharge.ts:691 WallCharge.blastShare)
pub fn wall_charge_blast_share(g: &mut crate::game::Game, w: &crate::wallcharge::WallSpot, p: V3, r: Option<crate::wallcharge::WallChargeAwayMarksValueR>, leaves: Option<bool>) -> f64 { todo!() }

/// (src/game/wallcharge.ts:699 WallCharge.clearOf)
pub fn wall_charge_clear_of(g: &mut crate::game::Game, gd: GuardId, where_: &mut dyn FnMut(&mut crate::game::Game) -> V3) -> Option<V3> { todo!() }

/// (src/game/wallcharge.ts:732 WallCharge.noLeaves)
pub fn wall_charge_no_leaves<T: Clone + core::fmt::Debug>(g: &mut crate::game::Game, f: &mut dyn FnMut(&mut crate::game::Game) -> T) -> T { todo!() }

/// (src/game/wallcharge.ts:734 WallCharge.tooClose)
pub fn wall_charge_too_close(g: &mut crate::game::Game, c_2: Option<WallChargeId>) -> Vec<String> { todo!() }

/// (src/game/wallcharge.ts:746 WallCharge.behind)
pub fn wall_charge_behind(g: &mut crate::game::Game, c_2: Option<WallChargeId>) -> Vec<String> { todo!() }

/// (src/game/wallcharge.ts:755 WallCharge.closeWords)
pub fn wall_charge_close_words(g: &mut crate::game::Game, c_2: Option<WallChargeId>, caps: Option<bool>) -> String { todo!() }

/// (src/game/wallcharge.ts:758 WallCharge.inBlast)
pub fn wall_charge_in_blast(g: &mut crate::game::Game, w: &crate::wallcharge::WallSpot, men: &[GuardId]) -> Option<String> { todo!() }

/// (src/game/wallcharge.ts:766 WallCharge.plan)
pub fn wall_charge_plan(g: &mut crate::game::Game, w: &crate::wallcharge::WallSpot, men: &[GuardId], kept: Option<&crate::wallcharge::WallChargeKept>) -> crate::wallcharge::WallChargePlan { todo!() }

/// (src/game/wallcharge.ts:786 WallCharge.keptFor)
pub fn wall_charge_kept_for(g: &mut crate::game::Game, t: &crate::orders::OrderTarget) -> Option<crate::wallcharge::WallChargeKept> { todo!() }

/// (src/game/wallcharge.ts:790 WallCharge.canOf)
pub fn wall_charge_can_of(g: &mut crate::game::Game, gd: GuardId, t: &crate::orders::OrderTarget) -> crate::wallcharge::WallChargeCanOf { todo!() }

/// (src/game/wallcharge.ts:796 WallCharge.valid)
pub fn wall_charge_valid(g: &mut crate::game::Game, t: &crate::orders::OrderTarget, men: &[GuardId]) -> crate::wallcharge::WallChargeValid { todo!() }

/// (src/game/wallcharge.ts:822 WallCharge.keptWalks)
pub fn wall_charge_kept_walks(g: &mut crate::game::Game, m: u32) -> crate::wallcharge::WallChargeKept { todo!() }

/// (src/game/wallcharge.ts:833 WallCharge.assign)
pub fn wall_charge_assign(g: &mut crate::game::Game, op: WallOpId, gd: GuardId, a: &crate::wallcharge::WallChargeAssignA) { todo!() }

/// (src/game/wallcharge.ts:834 WallCharge.standOff)
pub fn wall_charge_stand_off(g: &mut crate::game::Game, op: WallOpId, gd: GuardId) { todo!() }

/// (src/game/wallcharge.ts:839 WallCharge.issue)
pub fn wall_charge_issue(g: &mut crate::game::Game, men: &[GuardId], t: &crate::orders::OrderTarget) -> crate::wallcharge::WallChargeIssue { todo!() }

/// (src/game/wallcharge.ts:851 WallCharge.order)
pub fn wall_charge_order(g: &mut crate::game::Game, at: V3, who: Option<crate::wallcharge::WallChargeOrderWho>) -> Option<String> { todo!() }

/// (src/game/wallcharge.ts:852 WallCharge.end)
pub fn wall_charge_end(g: &mut crate::game::Game, op: WallOpId, why: &str) { todo!() }

/// (src/game/wallcharge.ts:866 WallCharge.repick)
pub fn wall_charge_repick(g: &mut crate::game::Game, op: WallOpId) { todo!() }

/// (src/game/wallcharge.ts:890 WallCharge.spareRear)
pub fn wall_charge_spare_rear(g: &mut crate::game::Game, op: WallOpId) { todo!() }

/// (src/game/wallcharge.ts:897 WallCharge.waiting)
pub fn wall_charge_waiting(g: &mut crate::game::Game, op: WallOpId, el: f64) -> bool { todo!() }

/// (src/game/wallcharge.ts:904 WallCharge.nearest)
pub fn wall_charge_nearest(g: &mut crate::game::Game, of: Option<&mut dyn FnMut(&mut crate::game::Game, WallChargeId) -> bool>) -> Option<WallChargeId> { todo!() }

/// (src/game/wallcharge.ts:907 WallCharge.input)
pub fn wall_charge_input(g: &mut crate::game::Game, input: InputId) { todo!() }

/// (src/game/wallcharge.ts:917 WallCharge.inputAfter)
pub fn wall_charge_input_after(g: &mut crate::game::Game, input: InputId) { todo!() }

/// (src/game/wallcharge.ts:919 WallCharge.tick)
pub fn wall_charge_tick(g: &mut crate::game::Game) { todo!() }

/// (src/game/wallcharge.ts:926 WallCharge.step)
pub fn wall_charge_step(g: &mut crate::game::Game, op: WallOpId) { todo!() }

/// (src/game/wallcharge.ts:990 WallCharge.takeHold)
pub fn wall_charge_take_hold(g: &mut crate::game::Game, op: WallOpId, gd: GuardId, light: Option<bool>) { todo!() }

/// (src/game/wallcharge.ts:1005 WallCharge.blowWait)
pub fn wall_charge_blow_wait(g: &mut crate::game::Game, op: WallOpId) -> f64 { todo!() }

/// (src/game/wallcharge.ts:1011 WallCharge.enter)
pub fn wall_charge_enter(g: &mut crate::game::Game, op: WallOpId) { todo!() }

/// (src/game/wallcharge.ts:1046 WallCharge.cross)
pub fn wall_charge_cross(g: &mut crate::game::Game, op: WallOpId, gd: GuardId, p: V3) { todo!() }

/// (src/game/wallcharge.ts:1053 WallCharge.lookFrom)
pub fn wall_charge_look_from(g: &mut crate::game::Game, p: V3, into: f64, want: f64) -> f64 { todo!() }

/// (src/game/wallcharge.ts:1064 WallCharge.blow)
pub fn wall_charge_blow(g: &mut crate::game::Game, c_2: Option<WallChargeId>, pressed: Option<bool>) -> Option<crate::wallcharge::WallChargeBlow> { todo!() }

/// (src/game/wallcharge.ts:1065 WallCharge.blowRaw)
pub fn wall_charge_blow_raw(g: &mut crate::game::Game, c_2: Option<WallChargeId>, pressed: bool) -> Option<crate::wallcharge::WallChargeBlowRaw> { todo!() }

/// (src/game/wallcharge.ts:1136 WallCharge.samCaught)
pub fn wall_charge_sam_caught(g: &mut crate::game::Game, pl: PlayerId, k: f64, from: V3) { todo!() }

/// (src/game/wallcharge.ts:1144 WallCharge.holeAt)
pub fn wall_charge_hole_at(g: &mut crate::game::Game, w: &crate::wallcharge::WallSpot) -> Option<crate::wallcharge::WallChargeHoleAt> { todo!() }

/// (src/game/wallcharge.ts:1159 WallCharge.shock)
pub fn wall_charge_shock(g: &mut crate::game::Game, gd: GuardId, secs: f64, at: V3) { todo!() }

/// (src/game/wallcharge.ts:1165 WallCharge.dazzle)
pub fn wall_charge_dazzle(g: &mut crate::game::Game, gd: GuardId, secs: f64, at: V3) { todo!() }

/// (src/game/wallcharge.ts:1173 WallCharge.draw)
pub fn wall_charge_draw(g: &mut crate::game::Game, c_2: WallChargeId) { todo!() }

/// (src/game/wallcharge.ts:1182 WallCharge.keyIs)
pub fn wall_charge_key_is(g: &mut crate::game::Game, c_2: WallChargeId) -> bool { todo!() }

/// (src/game/wallcharge.ts:1185 WallCharge.samSet)
pub fn wall_charge_sam_set(g: &mut crate::game::Game) -> Option<WallChargeId> { todo!() }

/// (src/game/wallcharge.ts:1186 WallCharge.samAt)
pub fn wall_charge_sam_at(g: &mut crate::game::Game, c_2: WallChargeId) -> V3 { todo!() }

/// (src/game/wallcharge.ts:1187 WallCharge.samWords)
pub fn wall_charge_sam_words(g: &mut crate::game::Game, c_2: WallChargeId) -> String { todo!() }

/// (src/game/wallcharge.ts:1190 WallCharge.samFatal)
pub fn wall_charge_sam_fatal(g: &mut crate::game::Game, c_2: WallChargeId) -> bool { todo!() }

/// (src/game/wallcharge.ts:1192 WallCharge.samRows)
pub fn wall_charge_sam_rows(g: &mut crate::game::Game, p: V3, busy: bool, row: &mut dyn FnMut(&mut crate::game::Game, crate::game::Interactable, crate::game::Verb, String, Option<String>, Option<crate::wallcharge::WallChargeRowsRowOver>) -> crate::game::Interactable, offer: &mut dyn FnMut(&mut crate::game::Game, crate::game::Interactable, Vec<crate::game::Interactable>)) { todo!() }

/// (src/game/wallcharge.ts:1207 WallCharge.samTick)
pub fn wall_charge_sam_tick(g: &mut crate::game::Game) { todo!() }

/// (src/game/wallcharge.ts:1222 WallCharge.setting)
pub fn wall_charge_setting(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/wallcharge.ts:1224 WallCharge.samJob)
pub fn wall_charge_sam_job(g: &mut crate::game::Game) -> Option<crate::wallcharge::WallChargeSamJob> { todo!() }

/// (src/game/wallcharge.ts:1226 WallCharge.cancel)
pub fn wall_charge_cancel(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/wallcharge.ts:1228 WallCharge.runningRow)
pub fn wall_charge_running_row(g: &mut crate::game::Game, running: &mut dyn FnMut(&mut crate::game::Game, crate::game::Interactable, crate::game::Verb, String, f64) -> crate::game::Interactable) -> crate::game::Interactable { todo!() }

/// (src/game/wallcharge.ts:1234 xz)
pub fn xz(p: V3) -> [f64; 2] { todo!() }


/// INVENTED: `fire.score(q)` of underFire (wallcharge.ts:680), the match over WallChargeFireAsk: how much under known fire a place is, 0 = not at all; each place asked once
pub fn wall_charge_fire_score(g: &mut crate::game::Game, fire: &mut crate::wallcharge::WallChargeFireAsk, q: V3) -> f64 { todo!() }

/// INVENTED: `fire.any(q)` of underFire (wallcharge.ts:682), the match over WallChargeFireAsk: is a place under fire at all, the first clear line ending the asking
pub fn wall_charge_fire_any(g: &mut crate::game::Game, fire: &mut crate::wallcharge::WallChargeFireAsk, q: V3) -> bool { todo!() }
