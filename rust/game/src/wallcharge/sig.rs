#![allow(unused_variables)]
//! Every function and method of `wallcharge`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/wallcharge.ts:281 nearSeg)
pub fn near_seg(q: V3, a: V3, b: V3) -> f64 { todo!() }

/// (src/game/wallcharge.ts:284 thick)
pub fn thick(g: &mut crate::game::Game, w: &crate::wallcharge::WallSpot) -> f64 { todo!() }

/// (src/game/wallcharge.ts:286 chargeAt)
pub fn charge_at(g: &mut crate::game::Game, w: &crate::wallcharge::WallSpot) -> V3 { todo!() }

/// (src/game/wallcharge.ts:288 wallPt)
pub fn wall_pt(g: &mut crate::game::Game, w: &crate::wallcharge::WallSpot, du: f64, off: f64, y: Option<f64>) -> V3 { todo!() }

/// (src/game/wallcharge.ts:291 wallN)
pub fn wall_n(w: &crate::wallcharge::WallSpot) -> V3 { todo!() }

/// (src/game/wallcharge.ts:292 wallOff)
pub fn wall_off(g: &mut crate::game::Game, w: &crate::wallcharge::WallSpot, p: V3) -> f64 { todo!() }

/// (src/game/wallcharge.ts:293 wallAlong)
pub fn wall_along(w: &crate::wallcharge::WallSpot, p: V3) -> f64 { todo!() }

/// (src/game/wallcharge.ts:294 sameWall)
pub fn same_wall(a: &crate::wallcharge::WallSpot, b: &crate::wallcharge::WallSpot) -> bool { todo!() }

/// (src/game/wallcharge.ts:297 wallChargeOn)
pub fn wall_charge_on(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/wallcharge.ts:306 skyFaces)
pub fn sky_faces(g: &mut crate::game::Game, index: f64) -> f64 { todo!() }

/// (src/game/wallcharge.ts:317 skyAside)
pub fn sky_aside<T: Clone + core::fmt::Debug>(g: &mut crate::game::Game, fn_: &mut dyn FnMut(&mut crate::game::Game) -> T) -> T { todo!() }

/// (src/game/wallcharge.ts:325 outerWall)
pub fn outer_wall(g: &mut crate::game::Game, index: f64) -> bool { todo!() }

/// (src/game/wallcharge.ts:335 chargeIndoors)
pub fn charge_indoors(g: &mut crate::game::Game, w: &crate::wallcharge::WallSpot) -> bool { todo!() }

/// (src/game/wallcharge.ts:342 safeRadius)
pub fn safe_radius(g: &mut crate::game::Game, w: &crate::wallcharge::WallSpot) -> crate::wallcharge::WallChargeAwayMarksValueR { todo!() }

/// (src/game/wallcharge.ts:351 roomBeyond)
pub fn room_beyond(g: &mut crate::game::Game, w: &crate::wallcharge::WallSpot, du: f64, t: f64) -> f64 { todo!() }

/// (src/game/wallcharge.ts:355 wallAt)
pub fn wall_at(g: &mut crate::game::Game, at: V3, from: Option<V3>) -> Option<crate::wallcharge::WallSpot> { todo!() }

/// (src/game/wallcharge.ts:381 wallTarget)
pub fn wall_target(g: &mut crate::game::Game, at: V3, base: &crate::orders::OrderTarget) -> crate::orders::OrderTarget { todo!() }

/// (src/game/wallcharge.ts:387 wallRows)
pub fn wall_rows(g: &mut crate::game::Game, t: &crate::orders::OrderTarget, page: Option<&str>, rows: &[OrdersOrderDefId]) -> Vec<OrdersOrderDefId> { todo!() }

/// (src/game/wallcharge.ts:396 wallChargeRows)
pub fn wall_charge_rows(g: &mut crate::game::Game, p: V3, busy: bool, row: &mut dyn FnMut(&mut crate::game::Game, crate::game::Interactable, crate::game::Verb, String, Option<String>, Option<crate::wallcharge::WallChargeRowsRowOver>) -> crate::game::Interactable, offer: &mut dyn FnMut(&mut crate::game::Game, crate::game::Interactable, Vec<crate::game::Interactable>)) { todo!() }

/// (src/game/wallcharge.ts:414 WallCharge.constructor)
pub fn wall_charge_new(g: &mut crate::game::Game) -> crate::wallcharge::WallCharge { todo!() }

/// (src/game/wallcharge.ts:424 WallCharge.sync)
pub fn wall_charge_sync(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/wallcharge.ts:438 WallCharge.undigit)
pub fn wall_charge_undigit(g: &mut crate::game::Game, d: OrdersOrderDefId) -> OrdersOrderDefId { todo!() }

/// (src/game/wallcharge.ts:439 WallCharge.seat)
pub fn wall_charge_seat(g: &mut crate::game::Game) { todo!() }

/// (src/game/wallcharge.ts:446 WallCharge.reset)
pub fn wall_charge_reset(g: &mut crate::game::Game) { todo!() }

/// (src/game/wallcharge.ts:448 WallCharge.stop)
pub fn wall_charge_stop(g: &mut crate::game::Game) { todo!() }

/// (src/game/wallcharge.ts:449 WallCharge.note)
pub fn wall_charge_note(g: &mut crate::game::Game, what: &str) { todo!() }

/// (src/game/wallcharge.ts:450 WallCharge.beat)
pub fn wall_charge_beat(g: &mut crate::game::Game, op: WallOpId, what: &str) { todo!() }

/// (src/game/wallcharge.ts:455 WallCharge.setCarried)
pub fn wall_charge_set_carried(g: &mut crate::game::Game, n: Option<f64>) { todo!() }

/// (src/game/wallcharge.ts:457 WallCharge.raiseCarried)
pub fn wall_charge_raise_carried(g: &mut crate::game::Game, n: f64) { todo!() }

/// (src/game/wallcharge.ts:461 WallCharge.price)
pub fn wall_charge_price(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/wallcharge.ts:462 WallCharge.affordable)
pub fn wall_charge_affordable(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/wallcharge.ts:463 WallCharge.wallsLeft)
pub fn wall_charge_walls_left(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/wallcharge.ts:465 WallCharge.rowWords)
pub fn wall_charge_row_words(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/wallcharge.ts:467 WallCharge.noneWords)
pub fn wall_charge_none_words(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/wallcharge.ts:469 WallCharge.take)
pub fn wall_charge_take(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/wallcharge.ts:477 WallCharge.giveBack)
pub fn wall_charge_give_back(g: &mut crate::game::Game, c_2: WallChargeId, why: &str) { todo!() }

/// (src/game/wallcharge.ts:484 WallCharge.hudNotes)
pub fn wall_charge_hud_notes(g: &mut crate::game::Game) -> Vec<crate::orders::OrderNote> { todo!() }

/// (src/game/wallcharge.ts:491 WallCharge.summary)
pub fn wall_charge_summary(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/wallcharge.ts:494 WallCharge.chargeOn)
pub fn wall_charge_charge_on(g: &mut crate::game::Game, w: &crate::wallcharge::WallSpot) -> Option<WallChargeId> { todo!() }

/// (src/game/wallcharge.ts:495 WallCharge.chargeOnTarget)
pub fn wall_charge_charge_on_target(g: &mut crate::game::Game, t: &crate::orders::OrderTarget) -> Option<WallChargeId> { todo!() }

/// (src/game/wallcharge.ts:498 WallCharge.spot)
pub fn wall_charge_spot(g: &mut crate::game::Game, t: &crate::orders::OrderTarget) -> Option<crate::wallcharge::WallSpot> { todo!() }

/// (src/game/wallcharge.ts:503 WallCharge.settle)
pub fn wall_charge_settle(g: &mut crate::game::Game, w: &crate::wallcharge::WallSpot) -> crate::wallcharge::WallSpot { todo!() }

/// (src/game/wallcharge.ts:510 WallCharge.openAt)
pub fn wall_charge_open_at(g: &mut crate::game::Game, w: &crate::wallcharge::WallSpot) -> bool { todo!() }

/// (src/game/wallcharge.ts:512 WallCharge.setSpot)
pub fn wall_charge_set_spot(g: &mut crate::game::Game, w: &crate::wallcharge::WallSpot) -> Option<V3> { todo!() }

/// (src/game/wallcharge.ts:519 WallCharge.farFloor)
pub fn wall_charge_far_floor(g: &mut crate::game::Game, w: &crate::wallcharge::WallSpot, furn: Option<bool>) -> Option<V3> { todo!() }

/// (src/game/wallcharge.ts:531 WallCharge.mouthAfter)
pub fn wall_charge_mouth_after(g: &mut crate::game::Game, w: &crate::wallcharge::WallSpot) -> Option<V3> { todo!() }

/// (src/game/wallcharge.ts:568 WallCharge.floorAfter)
pub fn wall_charge_floor_after(g: &mut crate::game::Game, w: &crate::wallcharge::WallSpot, m: f64, half: f64, q: V3, wall: f64, first: Option<crate::furnblast::BlastFirst>, mouth: &[V3], goes: Option<&mut dyn FnMut(&mut crate::game::Game, f64) -> bool>) -> bool { todo!() }

/// (src/game/wallcharge.ts:582 WallCharge.inFrame)
pub fn wall_charge_in_frame(g: &mut crate::game::Game, q: V3, m: Option<f64>) -> bool { todo!() }

/// (src/game/wallcharge.ts:584 WallCharge.openFoot)
pub fn wall_charge_open_foot(g: &mut crate::game::Game, w: &crate::wallcharge::WallSpot, du: f64) -> bool { todo!() }

/// (src/game/wallcharge.ts:589 WallCharge.standoffPosts)
pub fn wall_charge_standoff_posts(g: &mut crate::game::Game, w: &crate::wallcharge::WallSpot, n: f64, spot: Option<V3>, old: Option<bool>) -> Option<Vec<V3>> { todo!() }

/// (src/game/wallcharge.ts:613 WallCharge.safePosts)
pub fn wall_charge_safe_posts(g: &mut crate::game::Game, w: &crate::wallcharge::WallSpot, n: f64, spot: V3, can: Option<&mut dyn FnMut(&mut crate::game::Game, V3) -> bool>, kept: Option<&mut dyn FnMut(&mut crate::game::Game, String, crate::FnField) -> crate::Opaque>) -> Option<Vec<V3>> { todo!() }

/// (src/game/wallcharge.ts:653 WallCharge.knownFoes)
pub fn wall_charge_known_foes(g: &mut crate::game::Game) -> Vec<V3> { todo!() }

/// (src/game/wallcharge.ts:670 WallCharge.underFire)
pub fn wall_charge_under_fire(g: &mut crate::game::Game, w: Option<&crate::wallcharge::WallSpot>, all: Option<&[V3]>) -> crate::wallcharge::WallChargeUnderFire { todo!() }

/// (src/game/wallcharge.ts:687 WallCharge.fireAt)
pub fn wall_charge_fire_at(g: &mut crate::game::Game, q: V3, w: Option<&crate::wallcharge::WallSpot>) -> f64 { todo!() }

/// (src/game/wallcharge.ts:690 WallCharge.blastShare)
pub fn wall_charge_blast_share(g: &mut crate::game::Game, w: &crate::wallcharge::WallSpot, p: V3, r: Option<crate::wallcharge::WallChargeAwayMarksValueR>, leaves: Option<bool>) -> f64 { todo!() }

/// (src/game/wallcharge.ts:698 WallCharge.clearOf)
pub fn wall_charge_clear_of(g: &mut crate::game::Game, gd: GuardId, where_: &mut dyn FnMut(&mut crate::game::Game) -> V3) -> Option<V3> { todo!() }

/// (src/game/wallcharge.ts:731 WallCharge.noLeaves)
pub fn wall_charge_no_leaves<T: Clone + core::fmt::Debug>(g: &mut crate::game::Game, f: &mut dyn FnMut(&mut crate::game::Game) -> T) -> T { todo!() }

/// (src/game/wallcharge.ts:733 WallCharge.tooClose)
pub fn wall_charge_too_close(g: &mut crate::game::Game, c_2: Option<WallChargeId>) -> Vec<String> { todo!() }

/// (src/game/wallcharge.ts:745 WallCharge.behind)
pub fn wall_charge_behind(g: &mut crate::game::Game, c_2: Option<WallChargeId>) -> Vec<String> { todo!() }

/// (src/game/wallcharge.ts:754 WallCharge.closeWords)
pub fn wall_charge_close_words(g: &mut crate::game::Game, c_2: Option<WallChargeId>, caps: Option<bool>) -> String { todo!() }

/// (src/game/wallcharge.ts:757 WallCharge.inBlast)
pub fn wall_charge_in_blast(g: &mut crate::game::Game, w: &crate::wallcharge::WallSpot, men: &[GuardId]) -> Option<String> { todo!() }

/// (src/game/wallcharge.ts:765 WallCharge.plan)
pub fn wall_charge_plan(g: &mut crate::game::Game, w: &crate::wallcharge::WallSpot, men: &[GuardId], kept: Option<&mut dyn FnMut(&mut crate::game::Game, String, crate::FnField) -> crate::Opaque>) -> crate::wallcharge::WallChargePlan { todo!() }

/// (src/game/wallcharge.ts:785 WallCharge.keptFor)
pub fn wall_charge_kept_for(g: &mut crate::game::Game, t: &crate::orders::OrderTarget) -> Option<crate::FnField> { todo!() }

/// (src/game/wallcharge.ts:789 WallCharge.canOf)
pub fn wall_charge_can_of(g: &mut crate::game::Game, gd: GuardId, t: &crate::orders::OrderTarget) -> crate::wallcharge::WallChargeCanOf { todo!() }

/// (src/game/wallcharge.ts:795 WallCharge.valid)
pub fn wall_charge_valid(g: &mut crate::game::Game, t: &crate::orders::OrderTarget, men: &[GuardId]) -> crate::wallcharge::WallChargeValid { todo!() }

/// (src/game/wallcharge.ts:821 WallCharge.keptWalks)
pub fn wall_charge_kept_walks(g: &mut crate::game::Game, m: u32) -> crate::FnField { todo!() }

/// (src/game/wallcharge.ts:832 WallCharge.assign)
pub fn wall_charge_assign(g: &mut crate::game::Game, op: WallOpId, gd: GuardId, a: &crate::wallcharge::WallChargeAssignA) { todo!() }

/// (src/game/wallcharge.ts:833 WallCharge.standOff)
pub fn wall_charge_stand_off(g: &mut crate::game::Game, op: WallOpId, gd: GuardId) { todo!() }

/// (src/game/wallcharge.ts:838 WallCharge.issue)
pub fn wall_charge_issue(g: &mut crate::game::Game, men: &[GuardId], t: &crate::orders::OrderTarget) -> crate::wallcharge::WallChargeIssue { todo!() }

/// (src/game/wallcharge.ts:850 WallCharge.order)
pub fn wall_charge_order(g: &mut crate::game::Game, at: V3, who: Option<crate::wallcharge::WallChargeOrderWho>) -> Option<String> { todo!() }

/// (src/game/wallcharge.ts:851 WallCharge.end)
pub fn wall_charge_end(g: &mut crate::game::Game, op: WallOpId, why: &str) { todo!() }

/// (src/game/wallcharge.ts:865 WallCharge.repick)
pub fn wall_charge_repick(g: &mut crate::game::Game, op: WallOpId) { todo!() }

/// (src/game/wallcharge.ts:889 WallCharge.spareRear)
pub fn wall_charge_spare_rear(g: &mut crate::game::Game, op: WallOpId) { todo!() }

/// (src/game/wallcharge.ts:896 WallCharge.waiting)
pub fn wall_charge_waiting(g: &mut crate::game::Game, op: WallOpId, el: f64) -> bool { todo!() }

/// (src/game/wallcharge.ts:903 WallCharge.nearest)
pub fn wall_charge_nearest(g: &mut crate::game::Game, of: Option<&mut dyn FnMut(&mut crate::game::Game, WallChargeId) -> bool>) -> Option<WallChargeId> { todo!() }

/// (src/game/wallcharge.ts:906 WallCharge.input)
pub fn wall_charge_input(g: &mut crate::game::Game, input: InputId) { todo!() }

/// (src/game/wallcharge.ts:916 WallCharge.inputAfter)
pub fn wall_charge_input_after(g: &mut crate::game::Game, input: InputId) { todo!() }

/// (src/game/wallcharge.ts:918 WallCharge.tick)
pub fn wall_charge_tick(g: &mut crate::game::Game) { todo!() }

/// (src/game/wallcharge.ts:925 WallCharge.step)
pub fn wall_charge_step(g: &mut crate::game::Game, op: WallOpId) { todo!() }

/// (src/game/wallcharge.ts:989 WallCharge.takeHold)
pub fn wall_charge_take_hold(g: &mut crate::game::Game, op: WallOpId, gd: GuardId, light: Option<bool>) { todo!() }

/// (src/game/wallcharge.ts:1004 WallCharge.blowWait)
pub fn wall_charge_blow_wait(g: &mut crate::game::Game, op: WallOpId) -> f64 { todo!() }

/// (src/game/wallcharge.ts:1010 WallCharge.enter)
pub fn wall_charge_enter(g: &mut crate::game::Game, op: WallOpId) { todo!() }

/// (src/game/wallcharge.ts:1045 WallCharge.cross)
pub fn wall_charge_cross(g: &mut crate::game::Game, op: WallOpId, gd: GuardId, p: V3) { todo!() }

/// (src/game/wallcharge.ts:1052 WallCharge.lookFrom)
pub fn wall_charge_look_from(g: &mut crate::game::Game, p: V3, into: f64, want: f64) -> f64 { todo!() }

/// (src/game/wallcharge.ts:1063 WallCharge.blow)
pub fn wall_charge_blow(g: &mut crate::game::Game, c_2: Option<WallChargeId>, pressed: Option<bool>) -> Option<crate::wallcharge::WallChargeBlow> { todo!() }

/// (src/game/wallcharge.ts:1064 WallCharge.blowRaw)
pub fn wall_charge_blow_raw(g: &mut crate::game::Game, c_2: Option<WallChargeId>, pressed: bool) -> Option<crate::wallcharge::WallChargeBlowRaw> { todo!() }

/// (src/game/wallcharge.ts:1135 WallCharge.samCaught)
pub fn wall_charge_sam_caught(g: &mut crate::game::Game, pl: PlayerId, k: f64, from: V3) { todo!() }

/// (src/game/wallcharge.ts:1143 WallCharge.holeAt)
pub fn wall_charge_hole_at(g: &mut crate::game::Game, w: &crate::wallcharge::WallSpot) -> Option<crate::wallcharge::WallChargeHoleAt> { todo!() }

/// (src/game/wallcharge.ts:1158 WallCharge.shock)
pub fn wall_charge_shock(g: &mut crate::game::Game, gd: GuardId, secs: f64, at: V3) { todo!() }

/// (src/game/wallcharge.ts:1164 WallCharge.dazzle)
pub fn wall_charge_dazzle(g: &mut crate::game::Game, gd: GuardId, secs: f64, at: V3) { todo!() }

/// (src/game/wallcharge.ts:1172 WallCharge.draw)
pub fn wall_charge_draw(g: &mut crate::game::Game, c_2: WallChargeId) { todo!() }

/// (src/game/wallcharge.ts:1181 WallCharge.keyIs)
pub fn wall_charge_key_is(g: &mut crate::game::Game, c_2: WallChargeId) -> bool { todo!() }

/// (src/game/wallcharge.ts:1184 WallCharge.samSet)
pub fn wall_charge_sam_set(g: &mut crate::game::Game) -> Option<WallChargeId> { todo!() }

/// (src/game/wallcharge.ts:1185 WallCharge.samAt)
pub fn wall_charge_sam_at(g: &mut crate::game::Game, c_2: WallChargeId) -> V3 { todo!() }

/// (src/game/wallcharge.ts:1186 WallCharge.samWords)
pub fn wall_charge_sam_words(g: &mut crate::game::Game, c_2: WallChargeId) -> String { todo!() }

/// (src/game/wallcharge.ts:1189 WallCharge.samFatal)
pub fn wall_charge_sam_fatal(g: &mut crate::game::Game, c_2: WallChargeId) -> bool { todo!() }

/// (src/game/wallcharge.ts:1191 WallCharge.samRows)
pub fn wall_charge_sam_rows(g: &mut crate::game::Game, p: V3, busy: bool, row: &mut dyn FnMut(&mut crate::game::Game, crate::game::Interactable, crate::game::Verb, String, Option<String>, Option<crate::wallcharge::WallChargeSamRowsRowOver>) -> crate::game::Interactable, offer: &mut dyn FnMut(&mut crate::game::Game, crate::game::Interactable, Vec<crate::game::Interactable>)) { todo!() }

/// (src/game/wallcharge.ts:1206 WallCharge.samTick)
pub fn wall_charge_sam_tick(g: &mut crate::game::Game) { todo!() }

/// (src/game/wallcharge.ts:1221 WallCharge.setting)
pub fn wall_charge_setting(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/wallcharge.ts:1223 WallCharge.samJob)
pub fn wall_charge_sam_job(g: &mut crate::game::Game) -> Option<crate::wallcharge::WallChargeSamJob> { todo!() }

/// (src/game/wallcharge.ts:1225 WallCharge.cancel)
pub fn wall_charge_cancel(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/wallcharge.ts:1227 WallCharge.runningRow)
pub fn wall_charge_running_row(g: &mut crate::game::Game, running: &mut dyn FnMut(&mut crate::game::Game, crate::game::Interactable, crate::game::Verb, String, f64) -> crate::game::Interactable) -> crate::game::Interactable { todo!() }

/// (src/game/wallcharge.ts:1233 xz)
pub fn xz(p: V3) -> [f64; 2] { todo!() }
