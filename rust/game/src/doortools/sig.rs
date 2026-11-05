#![allow(unused_variables)]
//! Every function and method of `doortools`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/doortools.ts:750 doorPt)
pub fn door_pt(g: &mut crate::game::Game, d: DoorId, a: f64, b: f64, side: f64, y: Option<f64>) -> V3 { todo!() }

/// (src/game/doortools.ts:754 alongOf)
pub fn along_of(g: &mut crate::game::Game, d: DoorId, p: V3) -> f64 { todo!() }

/// (src/game/doortools.ts:756 offOf)
pub fn off_of(g: &mut crate::game::Game, d: DoorId, p: V3, side: f64) -> f64 { todo!() }

/// (src/game/doortools.ts:758 isOpen)
pub fn is_open(g: &mut crate::game::Game, d: DoorId) -> bool { todo!() }

/// (src/game/doortools.ts:762 underSky)
pub fn under_sky(g: &mut crate::game::Game, room: RoomfightRoomId) -> bool { todo!() }

/// (src/game/doortools.ts:769 openFloor)
pub fn open_floor(g: &mut crate::game::Game, room: RoomfightRoomId) -> bool { todo!() }

/// (src/game/doortools.ts:799 DoorTools.thrSigNow)
pub fn door_tools_thr_sig_now(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/doortools.ts:801 DoorTools.constructor)
pub fn door_tools_new(g: &mut crate::game::Game) -> crate::doortools::DoorTools { todo!() }

/// (src/game/doortools.ts:853 DoorTools.thrRows)
pub fn door_tools_thr_rows(g: &mut crate::game::Game) { todo!() }

/// (src/game/doortools.ts:882 DoorTools.take)
pub fn door_tools_take(g: &mut crate::game::Game, what: crate::doortools::DoorToolsTakeWhat) -> bool { todo!() }

/// (src/game/doortools.ts:889 DoorTools.refund)
pub fn door_tools_refund(g: &mut crate::game::Game, what: crate::doortools::DoorToolsRefundWhat) { todo!() }

/// (src/game/doortools.ts:891 DoorTools.sync)
pub fn door_tools_sync(g: &mut crate::game::Game) { todo!() }

/// (src/game/doortools.ts:941 DoorTools.reset)
pub fn door_tools_reset(g: &mut crate::game::Game) { todo!() }

/// (src/game/doortools.ts:948 DoorTools.alone)
pub fn door_tools_alone(g: &mut crate::game::Game, n: f64) { todo!() }

/// (src/game/doortools.ts:953 DoorTools.setBack)
pub fn door_tools_set_back(g: &mut crate::game::Game, d: DoorId, side: f64) { todo!() }

/// (src/game/doortools.ts:954 DoorTools.chargeOn)
pub fn door_tools_charge_on(g: &mut crate::game::Game, d: DoorId) -> Option<ChargeId> { todo!() }

/// (src/game/doortools.ts:956 DoorTools.stackable)
pub fn door_tools_stackable(g: &mut crate::game::Game, t: &crate::orders::OrderTarget, men: &[GuardId]) -> crate::doortools::DoorToolsStackable { todo!() }

/// (src/game/doortools.ts:966 DoorTools.lockedOut)
pub fn door_tools_locked_out(g: &mut crate::game::Game, d: DoorId, side: f64) -> bool { todo!() }

/// (src/game/doortools.ts:971 DoorTools.frameHeld)
pub fn door_tools_frame_held(g: &mut crate::game::Game, d: DoorId, side: f64) -> bool { todo!() }

/// (src/game/doortools.ts:974 DoorTools.noBoot)
pub fn door_tools_no_boot(g: &mut crate::game::Game, d: DoorId) -> String { todo!() }

/// (src/game/doortools.ts:977 DoorTools.frameAnswer)
pub fn door_tools_frame_answer(g: &mut crate::game::Game, op: DoorOpId) -> bool { todo!() }

/// (src/game/doortools.ts:985 DoorTools.willKick)
pub fn door_tools_will_kick(g: &mut crate::game::Game, op: DoorOpId) -> bool { todo!() }

/// (src/game/doortools.ts:991 DoorTools.bigOpens)
pub fn door_tools_big_opens(g: &mut crate::game::Game, d: DoorId, men: &[GuardId]) -> bool { todo!() }

/// (src/game/doortools.ts:994 DoorTools.bigWord)
pub fn door_tools_big_word(g: &mut crate::game::Game, t: &crate::orders::OrderTarget, men: &[GuardId]) -> crate::doortools::DoorToolsBigWord { todo!() }

/// (src/game/doortools.ts:999 DoorTools.bootSpot)
pub fn door_tools_boot_spot(g: &mut crate::game::Game, d: DoorId, side: f64) -> crate::doortools::DoorToolsBootSpot { todo!() }

/// (src/game/doortools.ts:1014 DoorTools.knows)
pub fn door_tools_knows(g: &mut crate::game::Game, d: DoorId, side: f64, free: bool) -> bool { todo!() }

/// (src/game/doortools.ts:1015 DoorTools.learn)
pub fn door_tools_learn(g: &mut crate::game::Game, d: DoorId, side: f64, free: Option<bool>) { todo!() }

/// (src/game/doortools.ts:1017 DoorTools.shutWhy)
pub fn door_tools_shut_why(g: &mut crate::game::Game, d: DoorId) -> String { todo!() }

/// (src/game/doortools.ts:1019 DoorTools.knownShut)
pub fn door_tools_known_shut(g: &mut crate::game::Game, d: DoorId, side: f64) -> bool { todo!() }

/// (src/game/doortools.ts:1026 DoorTools.untried)
pub fn door_tools_untried(g: &mut crate::game::Game, d: DoorId, side: f64) -> bool { todo!() }

/// (src/game/doortools.ts:1029 DoorTools.test)
pub fn door_tools_test(g: &mut crate::game::Game, d: DoorId, side: f64, gd: GuardId, say: bool) -> bool { todo!() }

/// (src/game/doortools.ts:1041 DoorTools.stackHeld)
pub fn door_tools_stack_held(g: &mut crate::game::Game, d: DoorId) -> bool { todo!() }

/// (src/game/doortools.ts:1044 DoorTools.testStacks)
pub fn door_tools_test_stacks(g: &mut crate::game::Game) { todo!() }

/// (src/game/doortools.ts:1058 DoorTools.haltAt)
pub fn door_tools_halt_at(g: &mut crate::game::Game, op: DoorOpId) -> bool { todo!() }

/// (src/game/doortools.ts:1071 DoorTools.shutSaid)
pub fn door_tools_shut_said(g: &mut crate::game::Game, lead: GuardId, d: DoorId) -> String { todo!() }

/// (src/game/doortools.ts:1078 DoorTools.walkTried)
pub fn door_tools_walk_tried(g: &mut crate::game::Game) { todo!() }

/// (src/game/doortools.ts:1105 DoorTools.haltBack)
pub fn door_tools_halt_back(g: &mut crate::game::Game, d: DoorId, side: f64, halted: &[GuardId]) { todo!() }

/// (src/game/doortools.ts:1113 DoorTools.plainStack)
pub fn door_tools_plain_stack(g: &mut crate::game::Game, op: DoorOpId, left: &[GuardId], note: Option<&str>) { todo!() }

/// (src/game/doortools.ts:1115 DoorTools.stackLeft)
pub fn door_tools_stack_left(g: &mut crate::game::Game, d: DoorId, post: &JsMap<GuardId, crate::doortools::DoorToolsStackLeftPostValue>, left: &[GuardId], note: Option<&str>) { todo!() }

/// (src/game/doortools.ts:1122 DoorTools.entryVerb)
pub fn door_tools_entry_verb(g: &mut crate::game::Game, t: &crate::orders::OrderTarget) -> crate::doortools::DoorToolsEntryVerb { todo!() }

/// (src/game/doortools.ts:1130 DoorTools.heldPlain)
pub fn door_tools_held_plain(g: &mut crate::game::Game, t: &crate::orders::OrderTarget, men: &[GuardId]) -> bool { todo!() }

/// (src/game/doortools.ts:1134 DoorTools.note)
pub fn door_tools_note(g: &mut crate::game::Game, what: &str) { todo!() }

/// (src/game/doortools.ts:1135 DoorTools.beat)
pub fn door_tools_beat(g: &mut crate::game::Game, op: DoorOpId, what: &str) { todo!() }

/// (src/game/doortools.ts:1139 DoorTools.assign)
pub fn door_tools_assign(g: &mut crate::game::Game, op: DoorOpId, gd: GuardId, a: &crate::doortools::DoorToolsAssignA) { todo!() }

/// (src/game/doortools.ts:1142 DoorTools.newOp)
pub fn door_tools_new_op(g: &mut crate::game::Game, kind: crate::doortools::DoorToolKind, d: DoorId, side: f64, men: &[GuardId], phase: crate::doortools::Phase) -> DoorOpId { todo!() }

/// (src/game/doortools.ts:1149 DoorTools.issue)
pub fn door_tools_issue(g: &mut crate::game::Game, kind: crate::doortools::DoorToolKind, men: &[GuardId], t: &crate::orders::OrderTarget, on_word: Option<bool>) -> crate::doortools::DoorToolsIssue { todo!() }

/// (src/game/doortools.ts:1151 DoorTools.together)
pub fn door_tools_together(g: &mut crate::game::Game, go: Option<bool>) -> DoorGateId { todo!() }

/// (src/game/doortools.ts:1155 DoorTools.order)
pub fn door_tools_order(g: &mut crate::game::Game, kind: crate::doortools::DoorToolsOrderKind, men: &[GuardId], door: DoorId, side: f64, gate: Option<DoorGateId>, hold: Option<bool>) -> crate::doortools::DoorToolsOrder { todo!() }

/// (src/game/doortools.ts:1169 DoorTools.threshold)
pub fn door_tools_threshold(g: &mut crate::game::Game, men: &[GuardId], t: &crate::orders::OrderTarget, on_word: Option<bool>, hold: Option<bool>) -> crate::doortools::DoorToolsThreshold { todo!() }

/// (src/game/doortools.ts:1170 DoorTools.thresholdHold)
pub fn door_tools_threshold_hold(g: &mut crate::game::Game, men: &[GuardId], t: &crate::orders::OrderTarget, on_word: Option<bool>) -> crate::doortools::DoorToolsThresholdHold { todo!() }

/// (src/game/doortools.ts:1171 DoorTools.canThreshold)
pub fn door_tools_can_threshold(g: &mut crate::game::Game, t: &crate::orders::OrderTarget, men: &[GuardId]) -> crate::doortools::DoorToolsCanThreshold { todo!() }

/// (src/game/doortools.ts:1175 DoorTools.thresholdOn)
pub fn door_tools_threshold_on(g: &mut crate::game::Game, door: &str, who: Option<crate::orders::Who>, side: Option<f64>, hold: Option<bool>) -> crate::doortools::DoorToolsThresholdOn { todo!() }

/// (src/game/doortools.ts:1184 DoorTools.thresholdHeld)
pub fn door_tools_threshold_held(g: &mut crate::game::Game, men: &[GuardId], door: Option<DoorId>) -> Vec<DoorOpId> { todo!() }

/// (src/game/doortools.ts:1186 DoorTools.heldOn)
pub fn door_tools_held_on(g: &mut crate::game::Game, t: &crate::orders::OrderTarget, men: &[GuardId]) -> Vec<DoorOpId> { todo!() }

/// (src/game/doortools.ts:1187 DoorTools.heldMen)
pub fn door_tools_held_men(g: &mut crate::game::Game, men: &[GuardId], t: &crate::orders::OrderTarget) -> Vec<GuardId> { todo!() }

/// (src/game/doortools.ts:1193 DoorTools.thresholdThen)
pub fn door_tools_threshold_then(g: &mut crate::game::Game, what: crate::doortools::DoorToolsThresholdThenWhat, men: &[GuardId], t: Option<&crate::doortools::DoorToolsThresholdThenT>) -> crate::doortools::DoorToolsThresholdThen { todo!() }

/// (src/game/doortools.ts:1215 DoorTools.thenOnWord)
pub fn door_tools_then_on_word(g: &mut crate::game::Game, what: crate::doortools::DoorToolsThenOnWordWhat, men: &[GuardId], t: &crate::orders::OrderTarget) -> crate::doortools::DoorToolsThenOnWord { todo!() }

/// (src/game/doortools.ts:1220 DoorTools.canThresholdThen)
pub fn door_tools_can_threshold_then(g: &mut crate::game::Game, what: crate::doortools::DoorToolsCanThresholdThenWhat, t: Option<&crate::doortools::DoorToolsCanThresholdThenT>, men: &[GuardId]) -> crate::doortools::DoorToolsCanThresholdThen { todo!() }

/// (src/game/doortools.ts:1228 DoorTools.toStep)
pub fn door_tools_to_step(g: &mut crate::game::Game, op: DoorOpId) { todo!() }

/// (src/game/doortools.ts:1231 DoorTools.thresholdTell)
pub fn door_tools_threshold_tell(g: &mut crate::game::Game, what: crate::doortools::DoorToolsThresholdTellWhat, door: Option<&str>, who: Option<crate::orders::Who>) -> Option<String> { todo!() }

/// (src/game/doortools.ts:1236 DoorTools.start)
pub fn door_tools_start(g: &mut crate::game::Game, kind: crate::doortools::DoorToolKind, men: &[GuardId], t: &crate::orders::OrderTarget, on_word: Option<bool>, hold: Option<bool>) -> crate::doortools::DoorToolsStart { todo!() }

/// (src/game/doortools.ts:1296 DoorTools.handOf)
pub fn door_tools_hand_of(g: &mut crate::game::Game, d: DoorId, side: f64, p: V3) -> f64 { todo!() }

/// (src/game/doortools.ts:1306 DoorTools.stayHands)
pub fn door_tools_stay_hands(g: &mut crate::game::Game, op: DoorOpId) { todo!() }

/// (src/game/doortools.ts:1315 DoorTools.crossesOpen)
pub fn door_tools_crosses_open(g: &mut crate::game::Game, d: DoorId, side: f64, from: V3, to: V3) -> bool { todo!() }

/// (src/game/doortools.ts:1321 DoorTools.keepHands)
pub fn door_tools_keep_hands(g: &mut crate::game::Game, op: DoorOpId, posts: &[crate::orders::StackPost], watch: V3) { todo!() }

/// (src/game/doortools.ts:1340 DoorTools.leadHand)
pub fn door_tools_lead_hand(g: &mut crate::game::Game, op: DoorOpId, posts: &[crate::orders::StackPost], watch: V3) { todo!() }

/// (src/game/doortools.ts:1358 DoorTools.laneCover)
pub fn door_tools_lane_cover(g: &mut crate::game::Game, op: DoorOpId) { todo!() }

/// (src/game/doortools.ts:1376 DoorTools.laneIn)
pub fn door_tools_lane_in(g: &mut crate::game::Game, op: DoorOpId) { todo!() }

/// (src/game/doortools.ts:1378 DoorTools.startCharge)
pub fn door_tools_start_charge(g: &mut crate::game::Game, men: &[GuardId], t: &crate::orders::OrderTarget, by_hand: Option<bool>) -> crate::doortools::DoorToolsStartCharge { todo!() }

/// (src/game/doortools.ts:1397 DoorTools.looseOn)
pub fn door_tools_loose_on(g: &mut crate::game::Game, t: &crate::orders::OrderTarget) -> bool { todo!() }

/// (src/game/doortools.ts:1404 DoorTools.standoffFits)
pub fn door_tools_standoff_fits(g: &mut crate::game::Game, men: &[GuardId], t: &crate::orders::OrderTarget, own: Option<bool>) -> crate::doortools::DoorToolsStandoffFits { todo!() }

/// (src/game/doortools.ts:1412 DoorTools.standoffWalk)
pub fn door_tools_standoff_walk(g: &mut crate::game::Game, men: &[GuardId], d: DoorId, posts: &[crate::orders::StackPost]) -> Option<String> { todo!() }

/// (src/game/doortools.ts:1421 DoorTools.closeWords)
pub fn door_tools_close_words(g: &mut crate::game::Game, c_2: ChargeId) -> String { todo!() }

/// (src/game/doortools.ts:1442 DoorTools.samFace)
pub fn door_tools_sam_face(g: &mut crate::game::Game, d: DoorId) -> Option<f64> { todo!() }

/// (src/game/doortools.ts:1450 DoorTools.samRows)
pub fn door_tools_sam_rows(g: &mut crate::game::Game, it: &mut crate::game::Interactable, d: DoorId, verbs: &mut Vec<crate::game::Interactable>, free: bool, row: &mut dyn FnMut(&mut crate::game::Game, crate::game::Interactable, crate::game::Verb, String, Option<String>) -> crate::game::Interactable) { todo!() }

/// (src/game/doortools.ts:1459 DoorTools.keyIs)
pub fn door_tools_key_is(g: &mut crate::game::Game, c_2: ChargeId) -> bool { todo!() }

/// (src/game/doortools.ts:1461 DoorTools.samSet)
pub fn door_tools_sam_set(g: &mut crate::game::Game) -> Option<ChargeId> { todo!() }

/// (src/game/doortools.ts:1463 DoorTools.samWhere)
pub fn door_tools_sam_where(g: &mut crate::game::Game, c_2: ChargeId) -> String { todo!() }

/// (src/game/doortools.ts:1465 DoorTools.samJob)
pub fn door_tools_sam_job(g: &mut crate::game::Game) -> Option<crate::doortools::DoorToolsSamJob> { todo!() }

/// (src/game/doortools.ts:1471 DoorTools.samProgress)
pub fn door_tools_sam_progress(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/doortools.ts:1473 DoorTools.cancelSam)
pub fn door_tools_cancel_sam(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/doortools.ts:1478 DoorTools.samTick)
pub fn door_tools_sam_tick(g: &mut crate::game::Game) { todo!() }

/// (src/game/doortools.ts:1504 DoorTools.tickSet)
pub fn door_tools_tick_set(g: &mut crate::game::Game) { todo!() }

/// (src/game/doortools.ts:1516 DoorTools.standoffDeal)
pub fn door_tools_standoff_deal(g: &mut crate::game::Game, men: &[GuardId], d: DoorId, posts: &[crate::orders::StackPost]) -> Vec<(GuardId, crate::orders::StackPost)> { todo!() }

/// (src/game/doortools.ts:1521 DoorTools.toLock)
pub fn door_tools_to_lock(g: &mut crate::game::Game, op: DoorOpId) { todo!() }

/// (src/game/doortools.ts:1526 DoorTools.toPick)
pub fn door_tools_to_pick(g: &mut crate::game::Game, op: DoorOpId) { todo!() }

/// (src/game/doortools.ts:1532 DoorTools.pickable)
pub fn door_tools_pickable(g: &mut crate::game::Game, d: DoorId) -> bool { todo!() }

/// (src/game/doortools.ts:1534 DoorTools.offLock)
pub fn door_tools_off_lock(g: &mut crate::game::Game, op: DoorOpId) { todo!() }

/// (src/game/doortools.ts:1540 DoorTools.pickStep)
pub fn door_tools_pick_step(g: &mut crate::game::Game, op: DoorOpId) { todo!() }

/// (src/game/doortools.ts:1568 DoorTools.toGun)
pub fn door_tools_to_gun(g: &mut crate::game::Game, op: DoorOpId) -> bool { todo!() }

/// (src/game/doortools.ts:1581 DoorTools.gunOff)
pub fn door_tools_gun_off(g: &mut crate::game::Game, op: DoorOpId, why: &str) { todo!() }

/// (src/game/doortools.ts:1590 DoorTools.gunStep)
pub fn door_tools_gun_step(g: &mut crate::game::Game, op: DoorOpId) { todo!() }

/// (src/game/doortools.ts:1626 DoorTools.hammerStep)
pub fn door_tools_hammer_step(g: &mut crate::game::Game, op: DoorOpId) { todo!() }

/// (src/game/doortools.ts:1645 DoorTools.slingTick)
pub fn door_tools_sling_tick(g: &mut crate::game::Game) { todo!() }

/// (src/game/doortools.ts:1648 DoorTools.newLead)
pub fn door_tools_new_lead(g: &mut crate::game::Game, op: DoorOpId) { todo!() }

/// (src/game/doortools.ts:1666 DoorTools.stackPosts)
pub fn door_tools_stack_posts(g: &mut crate::game::Game, d: DoorId, side: f64, men: &[GuardId], ignore: Option<&[GuardId]>) -> Option<Vec<crate::orders::StackPost>> { todo!() }

/// (src/game/doortools.ts:1669 DoorTools.stackTucked)
pub fn door_tools_stack_tucked(g: &mut crate::game::Game, d: DoorId, side: f64, men: &[GuardId], ignore: &[GuardId]) -> Option<Vec<crate::orders::StackPost>> { todo!() }

/// (src/game/doortools.ts:1686 DoorTools.holders)
pub fn door_tools_holders(g: &mut crate::game::Game, d: DoorId, side: f64, men: &[GuardId]) -> Vec<GuardId> { todo!() }

/// (src/game/doortools.ts:1696 DoorTools.stepBacks)
pub fn door_tools_step_backs(g: &mut crate::game::Game, d: DoorId, side: f64, men: &[GuardId], taken: &[V3], must: Option<bool>) -> Option<JsMap<GuardId, crate::orders::StackPost>> { todo!() }

/// (src/game/doortools.ts:1713 DoorTools.dealStack)
pub fn door_tools_deal_stack(g: &mut crate::game::Game, d: DoorId, side: f64, men: &[GuardId]) -> Option<crate::doortools::DoorToolsDealStack> { todo!() }

/// (src/game/doortools.ts:1723 DoorTools.makeRoom)
pub fn door_tools_make_room(g: &mut crate::game::Game, d: DoorId, back: &JsMap<GuardId, crate::orders::StackPost>) { todo!() }

/// (src/game/doortools.ts:1733 DoorTools.fragBehind)
pub fn door_tools_frag_behind(g: &mut crate::game::Game, d: DoorId, side: f64, men: &[GuardId]) -> Option<String> { todo!() }

/// (src/game/doortools.ts:1740 DoorTools.behind)
pub fn door_tools_behind(g: &mut crate::game::Game, d: DoorId, side: f64, men: &[GuardId]) -> Option<GuardId> { todo!() }

/// (src/game/doortools.ts:1748 DoorTools.canReach)
pub fn door_tools_can_reach(g: &mut crate::game::Game, d: DoorId, side: f64, p: V3) -> bool { todo!() }

/// (src/game/doortools.ts:1754 DoorTools.samBehind)
pub fn door_tools_sam_behind(g: &mut crate::game::Game, d: DoorId, side: f64) -> bool { todo!() }

/// (src/game/doortools.ts:1762 DoorTools.mateBehind)
pub fn door_tools_mate_behind(g: &mut crate::game::Game, op: DoorOpId) -> Option<GuardId> { todo!() }

/// (src/game/doortools.ts:1768 DoorTools.canBehind)
pub fn door_tools_can_behind(g: &mut crate::game::Game, d: DoorId, side: f64, men: &[GuardId]) -> Option<String> { todo!() }

/// (src/game/doortools.ts:1775 DoorTools.waiting4)
pub fn door_tools_waiting4(g: &mut crate::game::Game, men: &[GuardId]) -> Vec<crate::doortools::DoorToolsWaiting4Item> { todo!() }

/// (src/game/doortools.ts:1782 DoorTools.stacksOf)
pub fn door_tools_stacks_of(g: &mut crate::game::Game, men: &[GuardId]) -> Vec<crate::doortools::DoorToolsStacksOfItem> { todo!() }

/// (src/game/doortools.ts:1792 DoorTools.stackWaits)
pub fn door_tools_stack_waits(g: &mut crate::game::Game, men: &[GuardId]) -> Vec<crate::doortools::DoorToolsStackWaitsItem> { todo!() }

/// (src/game/doortools.ts:1793 DoorTools.stackWords)
pub fn door_tools_stack_words(g: &mut crate::game::Game, men: &[GuardId]) -> String { todo!() }

/// (src/game/doortools.ts:1795 DoorTools.waitingMen)
pub fn door_tools_waiting_men(g: &mut crate::game::Game, men: &[GuardId]) -> Vec<GuardId> { todo!() }

/// (src/game/doortools.ts:1799 DoorTools.wordMen)
pub fn door_tools_word_men(g: &mut crate::game::Game) -> Vec<GuardId> { todo!() }

/// (src/game/doortools.ts:1802 DoorTools.waitingAny)
pub fn door_tools_waiting_any(g: &mut crate::game::Game, men: &[GuardId]) -> Vec<crate::doortools::DoorToolsWaitingAnyItem> { todo!() }

/// (src/game/doortools.ts:1805 DoorTools.waitsOn)
pub fn door_tools_waits_on(g: &mut crate::game::Game, men: &[GuardId], d: DoorId) -> Option<crate::doortools::DoorToolsWaitsOn> { todo!() }

/// (src/game/doortools.ts:1811 DoorTools.wordPending)
pub fn door_tools_word_pending(g: &mut crate::game::Game, c_2: ChargeId) -> bool { todo!() }

/// (src/game/doortools.ts:1813 DoorTools.waitingWords)
pub fn door_tools_waiting_words(g: &mut crate::game::Game, men: &[GuardId]) -> String { todo!() }

/// (src/game/doortools.ts:1818 DoorTools.go)
pub fn door_tools_go(g: &mut crate::game::Game, who: Option<crate::orders::Who>) -> Option<String> { todo!() }

/// (src/game/doortools.ts:1822 DoorTools.wordQuiet)
pub fn door_tools_word_quiet(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/doortools.ts:1827 DoorTools.wordStepIn)
pub fn door_tools_word_step_in(g: &mut crate::game::Game, op: DoorOpId) { todo!() }

/// (src/game/doortools.ts:1836 DoorTools.wordStepGo)
pub fn door_tools_word_step_go(g: &mut crate::game::Game, op: DoorOpId) { todo!() }

/// (src/game/doortools.ts:1842 DoorTools.wordOwed)
pub fn door_tools_word_owed(g: &mut crate::game::Game, op: DoorOpId) -> bool { todo!() }

/// (src/game/doortools.ts:1850 DoorTools.wordSteps)
pub fn door_tools_word_steps(g: &mut crate::game::Game, op: DoorOpId) -> bool { todo!() }

/// (src/game/doortools.ts:1853 DoorTools.wordOnHold)
pub fn door_tools_word_on_hold(g: &mut crate::game::Game, men: &[GuardId]) -> bool { todo!() }

/// (src/game/doortools.ts:1862 DoorTools.wordEarly)
pub fn door_tools_word_early(g: &mut crate::game::Game, men: &[GuardId]) -> bool { todo!() }

/// (src/game/doortools.ts:1868 DoorTools.wordKeyHeld)
pub fn door_tools_word_key_held(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/doortools.ts:1873 DoorTools.pairWord)
pub fn door_tools_pair_word(g: &mut crate::game::Game, men: &[GuardId]) -> bool { todo!() }

/// (src/game/doortools.ts:1877 DoorTools.goMen)
pub fn door_tools_go_men(g: &mut crate::game::Game, men: &[GuardId], only: Option<&[crate::doortools::DoorToolsGoMenOnlyItem]>) -> Option<String> { todo!() }

/// (src/game/doortools.ts:1893 DoorTools.fromStack)
pub fn door_tools_from_stack(g: &mut crate::game::Game, men: &[GuardId], d: DoorId) -> Option<DoorOpId> { todo!() }

/// (src/game/doortools.ts:1909 DoorTools.wordTick)
pub fn door_tools_word_tick(g: &mut crate::game::Game) { todo!() }

/// (src/game/doortools.ts:1936 DoorTools.open)
pub fn door_tools_open(g: &mut crate::game::Game, op: DoorOpId) { todo!() }

/// (src/game/doortools.ts:1962 DoorTools.toHandle)
pub fn door_tools_to_handle(g: &mut crate::game::Game, op: DoorOpId, gate: bool) -> bool { todo!() }

/// (src/game/doortools.ts:1970 DoorTools.bootBeside)
pub fn door_tools_boot_beside(g: &mut crate::game::Game, op: DoorOpId) -> bool { todo!() }

/// (src/game/doortools.ts:1977 DoorTools.handStep)
pub fn door_tools_hand_step(g: &mut crate::game::Game, op: DoorOpId, el: f64) { todo!() }

/// (src/game/doortools.ts:1989 DoorTools.standoffPosts)
pub fn door_tools_standoff_posts(g: &mut crate::game::Game, d: DoorId, side: f64, n: f64, men: Option<&[GuardId]>) -> Option<Vec<crate::orders::StackPost>> { todo!() }

/// (src/game/doortools.ts:2003 DoorTools.standoffPostsAsWere)
pub fn door_tools_standoff_posts_as_were(g: &mut crate::game::Game, d: DoorId, side: f64, n: f64, men: Option<&[GuardId]>, files: Option<&mut Vec<Vec<crate::orders::StackPost>>>) -> Option<Vec<crate::orders::StackPost>> { todo!() }

/// (src/game/doortools.ts:2049 DoorTools.menuKey)
pub fn door_tools_menu_key(g: &mut crate::game::Game, input: InputId) -> bool { todo!() }

/// (src/game/doortools.ts:2057 DoorTools.bigKey)
pub fn door_tools_big_key(g: &mut crate::game::Game, input: InputId) -> bool { todo!() }

/// (src/game/doortools.ts:2063 DoorTools.input)
pub fn door_tools_input(g: &mut crate::game::Game, input: InputId) { todo!() }

/// (src/game/doortools.ts:2106 DoorTools.onPendingWord)
pub fn door_tools_on_pending_word(g: &mut crate::game::Game, c_2: ChargeId) -> bool { todo!() }

/// (src/game/doortools.ts:2109 DoorTools.trigger)
pub fn door_tools_trigger(g: &mut crate::game::Game, c_2: Option<ChargeId>) { todo!() }

/// (src/game/doortools.ts:2114 DoorTools.nearest)
pub fn door_tools_nearest(g: &mut crate::game::Game, of: Option<&mut dyn FnMut(&mut crate::game::Game, ChargeId) -> bool>) -> Option<ChargeId> { todo!() }

/// (src/game/doortools.ts:2116 DoorTools.tick)
pub fn door_tools_tick(g: &mut crate::game::Game) { todo!() }

/// (src/game/doortools.ts:2147 DoorTools.arrived)
pub fn door_tools_arrived(g: &mut crate::game::Game, op: DoorOpId, gd: GuardId) -> bool { todo!() }

/// (src/game/doortools.ts:2149 DoorTools.spareRear)
pub fn door_tools_spare_rear(g: &mut crate::game::Game, op: DoorOpId) { todo!() }

/// (src/game/doortools.ts:2155 DoorTools.waiting)
pub fn door_tools_waiting(g: &mut crate::game::Game, op: DoorOpId, el: f64) -> bool { todo!() }

/// (src/game/doortools.ts:2160 DoorTools.busy)
pub fn door_tools_busy(g: &mut crate::game::Game, op: DoorOpId, el: f64) -> bool { todo!() }

/// (src/game/doortools.ts:2163 DoorTools.end)
pub fn door_tools_end(g: &mut crate::game::Game, op: DoorOpId, why: &str) { todo!() }

/// (src/game/doortools.ts:2174 DoorTools.short)
pub fn door_tools_short(g: &mut crate::game::Game, op: DoorOpId, why: &str) { todo!() }

/// (src/game/doortools.ts:2187 DoorTools.chargeGone)
pub fn door_tools_charge_gone(g: &mut crate::game::Game, op: DoorOpId, hands: bool) { todo!() }

/// (src/game/doortools.ts:2193 DoorTools.knownOpen)
pub fn door_tools_known_open(g: &mut crate::game::Game, d: DoorId) -> bool { todo!() }

/// (src/game/doortools.ts:2195 DoorTools.defuse)
pub fn door_tools_defuse(g: &mut crate::game::Game, op: DoorOpId) { todo!() }

/// (src/game/doortools.ts:2198 DoorTools.step)
pub fn door_tools_step(g: &mut crate::game::Game, op: DoorOpId) { todo!() }

/// (src/game/doortools.ts:2584 DoorTools.closeUp)
pub fn door_tools_close_up(g: &mut crate::game::Game, op: DoorOpId, gd: Option<GuardId>) { todo!() }

/// (src/game/doortools.ts:2591 DoorTools.backToJamb)
pub fn door_tools_back_to_jamb(g: &mut crate::game::Game, op: DoorOpId, gd: GuardId) { todo!() }

/// (src/game/doortools.ts:2596 DoorTools.shove)
pub fn door_tools_shove(g: &mut crate::game::Game, op: DoorOpId) { todo!() }

/// (src/game/doortools.ts:2603 DoorTools.opened)
pub fn door_tools_opened(g: &mut crate::game::Game, op: DoorOpId) { todo!() }

/// (src/game/doortools.ts:2611 DoorTools.landAt)
pub fn door_tools_land_at(g: &mut crate::game::Game, op: DoorOpId, deep: Option<f64>) -> V3 { todo!() }

/// (src/game/doortools.ts:2614 DoorTools.bangSpot)
pub fn door_tools_bang_spot(g: &mut crate::game::Game, op: DoorOpId, from: V3) -> Option<crate::doortools::DoorToolsBangSpot> { todo!() }

/// (src/game/doortools.ts:2644 DoorTools.landFor)
pub fn door_tools_land_for(g: &mut crate::game::Game, d: DoorId, side: f64, deep: f64) -> V3 { todo!() }

/// (src/game/doortools.ts:2653 DoorTools.bang)
pub fn door_tools_bang(g: &mut crate::game::Game, op: DoorOpId, it: ItemId) { todo!() }

/// (src/game/doortools.ts:2669 DoorTools.cornerStep)
pub fn door_tools_corner_step(g: &mut crate::game::Game, op: DoorOpId, gd: GuardId, a: OrdersAssignmentId, pod: crate::entrypoints::Pod) -> bool { todo!() }

/// (src/game/doortools.ts:2685 DoorTools.onPoint)
pub fn door_tools_on_point(g: &mut crate::game::Game, gd: GuardId, a: OrdersAssignmentId, pod: crate::entrypoints::Pod) -> bool { todo!() }

/// (src/game/doortools.ts:2688 DoorTools.owesCorner)
pub fn door_tools_owes_corner(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/doortools.ts:2698 DoorTools.threatNote)
pub fn door_tools_threat_note(g: &mut crate::game::Game, op: DoorOpId) { todo!() }

/// (src/game/doortools.ts:2722 DoorTools.sees)
pub fn door_tools_sees(g: &mut crate::game::Game, op: DoorOpId, p: V3) -> bool { todo!() }

/// (src/game/doortools.ts:2726 DoorTools.threatDeal)
pub fn door_tools_threat_deal(g: &mut crate::game::Game, op: DoorOpId) { todo!() }

/// (src/game/doortools.ts:2735 DoorTools.threatStep)
pub fn door_tools_threat_step(g: &mut crate::game::Game, op: DoorOpId, gd: GuardId, a: OrdersAssignmentId, pod: crate::entrypoints::Pod) -> bool { todo!() }

/// (src/game/doortools.ts:2765 DoorTools.firstIn)
pub fn door_tools_first_in(g: &mut crate::game::Game, op: DoorOpId, at: Option<V3>, by_gun: Option<bool>) -> GuardId { todo!() }

/// (src/game/doortools.ts:2777 DoorTools.seenMost)
pub fn door_tools_seen_most(g: &mut crate::game::Game, lead: GuardId, men: &[GuardId], at: Option<V3>) -> GuardId { todo!() }

/// (src/game/doortools.ts:2786 DoorTools.inAt)
pub fn door_tools_in_at(g: &mut crate::game::Game, op: DoorOpId, gd: GuardId) -> f64 { todo!() }

/// (src/game/doortools.ts:2788 DoorTools.onSector)
pub fn door_tools_on_sector(g: &mut crate::game::Game, gd: GuardId, a: OrdersAssignmentId, pod: crate::entrypoints::Pod) -> f64 { todo!() }

/// (src/game/doortools.ts:2795 DoorTools.doorKeepers)
pub fn door_tools_door_keepers(g: &mut crate::game::Game) -> Vec<(GuardId, DoorId, OrdersAssignmentId)> { todo!() }

/// (src/game/doortools.ts:2800 DoorTools.keepSectors)
pub fn door_tools_keep_sectors(g: &mut crate::game::Game) { todo!() }

/// (src/game/doortools.ts:2822 DoorTools.onOther)
pub fn door_tools_on_other(g: &mut crate::game::Game, op: DoorOpId, gd: GuardId) -> bool { todo!() }

/// (src/game/doortools.ts:2830 DoorTools.otherDoors)
pub fn door_tools_other_doors(g: &mut crate::game::Game, op: DoorOpId, ended: Option<bool>, deal: Option<bool>, seen: Option<bool>) -> bool { todo!() }

/// (src/game/doortools.ts:2878 DoorTools.leafAcross)
pub fn door_tools_leaf_across(g: &mut crate::game::Game, o: DoorId, p: V3, at: V3) -> f64 { todo!() }

/// (src/game/doortools.ts:2891 DoorTools.holdDoors)
pub fn door_tools_hold_doors(g: &mut crate::game::Game, op: DoorOpId) { todo!() }

/// (src/game/doortools.ts:2943 DoorTools.roomIn)
pub fn door_tools_room_in(g: &mut crate::game::Game, op: DoorOpId) -> Option<RoomfightRoomId> { todo!() }

/// (src/game/doortools.ts:2944 DoorTools.cornersOf)
pub fn door_tools_corners_of(g: &mut crate::game::Game, op: DoorOpId, pod: crate::entrypoints::Pod) -> Vec<V3> { todo!() }

/// (src/game/doortools.ts:2956 DoorTools.searchOf)
pub fn door_tools_search_of(g: &mut crate::game::Game, op: DoorOpId, at: Option<V3>) -> Option<DoortoolsRoomSearchId> { todo!() }

/// (src/game/doortools.ts:2966 DoorTools.lineTo)
pub fn door_tools_line_to(g: &mut crate::game::Game, sr: DoortoolsRoomSearchId, gd: GuardId, i: f64) -> bool { todo!() }

/// (src/game/doortools.ts:2973 DoorTools.look)
pub fn door_tools_look(g: &mut crate::game::Game, op: DoorOpId, sr: DoortoolsRoomSearchId, out: Option<&[GuardId]>, ok: Option<&mut dyn FnMut(&mut crate::game::Game, GuardId, f64) -> bool>) { todo!() }

/// (src/game/doortools.ts:2984 DoorTools.scan)
pub fn door_tools_scan(g: &mut crate::game::Game, op: DoorOpId, sr: DoortoolsRoomSearchId, gd: GuardId, a: OrdersAssignmentId) -> bool { todo!() }

/// (src/game/doortools.ts:2997 DoorTools.walkLen)
pub fn door_tools_walk_len(g: &mut crate::game::Game, a: V3, b: V3, lim: f64) -> f64 { todo!() }

/// (src/game/doortools.ts:3002 DoorTools.walks)
pub fn door_tools_walks(g: &mut crate::game::Game) -> crate::doortools::Walks { todo!() }

/// (src/game/doortools.ts:3020 DoorTools.away)
pub fn door_tools_away(g: &mut crate::game::Game, gd: GuardId, p: V3, pool: &[GuardId], where_: &mut dyn FnMut(&mut crate::game::Game, GuardId) -> V3, len: &mut dyn FnMut(&mut crate::game::Game, V3, V3) -> f64, pair: Option<bool>, on_door: Option<&mut dyn FnMut(&mut crate::game::Game, GuardId) -> bool>) -> crate::doortools::DoorToolsAway { todo!() }

/// (src/game/doortools.ts:3044 DoorTools.besidePod)
pub fn door_tools_beside_pod(g: &mut crate::game::Game, p: V3, face: f64, mate: GuardId, taken: &[V3], in_room: &mut dyn FnMut(&mut crate::game::Game, V3) -> bool) -> Option<crate::entrypoints::Pod> { todo!() }

/// (src/game/doortools.ts:3060 DoorTools.sendOn)
pub fn door_tools_send_on(g: &mut crate::game::Game, op: DoorOpId, gd: GuardId, pod: &mut crate::entrypoints::Pod, going: &str) { todo!() }

/// (src/game/doortools.ts:3067 DoorTools.flows)
pub fn door_tools_flows(g: &mut crate::game::Game, op: DoorOpId, sr: DoortoolsRoomSearchId) -> bool { todo!() }

/// (src/game/doortools.ts:3076 DoorTools.flowIn)
pub fn door_tools_flow_in(g: &mut crate::game::Game, op: DoorOpId, sr: DoortoolsRoomSearchId, men: &[GuardId]) -> bool { todo!() }

/// (src/game/doortools.ts:3160 DoorTools.moveOn)
pub fn door_tools_move_on(g: &mut crate::game::Game, op: DoorOpId, sr: DoortoolsRoomSearchId) -> bool { todo!() }

/// (src/game/doortools.ts:3203 DoorTools.pods)
pub fn door_tools_pods(g: &mut crate::game::Game, d: DoorId, side: f64, n: Option<f64>) -> Vec<crate::entrypoints::Pod> { todo!() }

/// (src/game/doortools.ts:3213 DoorTools.offSwing)
pub fn door_tools_off_swing(g: &mut crate::game::Game, p: &mut crate::doortools::DoorToolsOffSwingP, d: Option<DoorId>, side: f64, taken: &[V3], see: Option<V3>) { todo!() }

/// (src/game/doortools.ts:3222 DoorTools.podsAsWere)
pub fn door_tools_pods_as_were(g: &mut crate::game::Game, d: DoorId, side: f64, n: Option<f64>) -> Vec<crate::entrypoints::Pod> { todo!() }

/// (src/game/doortools.ts:3234 DoorTools.inFrame)
pub fn door_tools_in_frame(g: &mut crate::game::Game, d: DoorId, mid: V3, p: V3) -> bool { todo!() }

/// (src/game/doortools.ts:3246 DoorTools.pods2)
pub fn door_tools_pods2(g: &mut crate::game::Game, d: DoorId, side: f64) -> Vec<crate::entrypoints::Pod> { todo!() }

/// (src/game/doortools.ts:3266 DoorTools.enter)
pub fn door_tools_enter(g: &mut crate::game::Game, op: DoorOpId) { todo!() }

/// (src/game/doortools.ts:3303 DoorTools.podsApart)
pub fn door_tools_pods_apart(g: &mut crate::game::Game, op: DoorOpId, pods: &[crate::entrypoints::Pod], n: f64) -> Vec<crate::entrypoints::Pod> { todo!() }

/// (src/game/doortools.ts:3313 DoorTools.blow)
pub fn door_tools_blow(g: &mut crate::game::Game, c_2: Option<ChargeId>) -> Option<crate::doortools::DoorToolsBlow> { todo!() }

/// (src/game/doortools.ts:3314 DoorTools.blowRaw)
pub fn door_tools_blow_raw(g: &mut crate::game::Game, c_2: Option<ChargeId>) -> Option<crate::doortools::DoorToolsBlowRaw> { todo!() }

/// (src/game/doortools.ts:3360 DoorTools.dazzle)
pub fn door_tools_dazzle(g: &mut crate::game::Game, gd: GuardId, secs: f64, at: V3) { todo!() }

/// (src/game/doortools.ts:3368 DoorTools.drawCharge)
pub fn door_tools_draw_charge(g: &mut crate::game::Game, c_2: ChargeId) { todo!() }

/// (src/game/doortools.ts:3379 DoorTools.pieNext)
pub fn door_tools_pie_next(g: &mut crate::game::Game, op: DoorOpId, gd: GuardId, s: &crate::doortools::DoorToolsPieNextS) -> f64 { todo!() }

/// (src/game/doortools.ts:3389 DoorTools.pieOff)
pub fn door_tools_pie_off(g: &mut crate::game::Game, op: DoorOpId) { todo!() }

/// (src/game/doortools.ts:3400 DoorTools.pieSet)
pub fn door_tools_pie_set(g: &mut crate::game::Game, op: DoorOpId, gd: GuardId, sgn: f64, first: &[V3], off: &[V3]) -> Vec<V3> { todo!() }

/// (src/game/doortools.ts:3418 DoorTools.sliceLooks)
pub fn door_tools_slice_looks(g: &mut crate::game::Game, op: DoorOpId, p: V3, sgn: f64) -> Vec<V3> { todo!() }

/// (src/game/doortools.ts:3427 DoorTools.toSlice)
pub fn door_tools_to_slice(g: &mut crate::game::Game, op: DoorOpId) { todo!() }

/// (src/game/doortools.ts:3462 DoorTools.leafOurs)
pub fn door_tools_leaf_ours(g: &mut crate::game::Game, d: DoorId, side: f64) -> bool { todo!() }

/// (src/game/doortools.ts:3465 DoorTools.thrSpot)
pub fn door_tools_thr_spot(g: &mut crate::game::Game, op: DoorOpId, sgn: f64, mark: V3) -> crate::doortools::DoorToolsThrSpot { todo!() }

/// (src/game/doortools.ts:3472 DoorTools.thrFar)
pub fn door_tools_thr_far(g: &mut crate::game::Game, op: DoorOpId) -> Option<crate::doortools::DoorToolsThrFar> { todo!() }

/// (src/game/doortools.ts:3481 DoorTools.thrSetFar)
pub fn door_tools_thr_set_far(g: &mut crate::game::Game, op: DoorOpId) { todo!() }

/// (src/game/doortools.ts:3490 DoorTools.lastCall)
pub fn door_tools_last_call(g: &mut crate::game::Game, op: DoorOpId) -> bool { todo!() }

/// (src/game/doortools.ts:3498 DoorTools.lastStack)
pub fn door_tools_last_stack(g: &mut crate::game::Game, op: DoorOpId) { todo!() }

/// (src/game/doortools.ts:3521 DoorTools.toThreshold)
pub fn door_tools_to_threshold(g: &mut crate::game::Game, op: DoorOpId) { todo!() }

/// (src/game/doortools.ts:3558 DoorTools.canFirst)
pub fn door_tools_can_first(g: &mut crate::game::Game, op: DoorOpId) -> bool { todo!() }

/// (src/game/doortools.ts:3568 DoorTools.burstDry)
pub fn door_tools_burst_dry(g: &mut crate::game::Game, op: DoorOpId) -> bool { todo!() }

/// (src/game/doortools.ts:3574 DoorTools.thrOut)
pub fn door_tools_thr_out(g: &mut crate::game::Game, op: DoorOpId, why: Option<&str>) { todo!() }

/// (src/game/doortools.ts:3584 DoorTools.thrMen)
pub fn door_tools_thr_men(g: &mut crate::game::Game, op: DoorOpId) { todo!() }

/// (src/game/doortools.ts:3596 DoorTools.thrWatch)
pub fn door_tools_thr_watch(g: &mut crate::game::Game, op: DoorOpId) { todo!() }

/// (src/game/doortools.ts:3617 DoorTools.thrCans)
pub fn door_tools_thr_cans(g: &mut crate::game::Game, t: &crate::doortools::Threshold) -> f64 { todo!() }

/// (src/game/doortools.ts:3624 DoorTools.toPockets)
pub fn door_tools_to_pockets(g: &mut crate::game::Game, op: DoorOpId) { todo!() }

/// (src/game/doortools.ts:3633 DoorTools.heldBeat)
pub fn door_tools_held_beat(g: &mut crate::game::Game, op: DoorOpId) { todo!() }

/// (src/game/doortools.ts:3638 DoorTools.thrThrower)
pub fn door_tools_thr_thrower(g: &mut crate::game::Game, op: DoorOpId) -> GuardId { todo!() }

/// (src/game/doortools.ts:3641 DoorTools.thrBack)
pub fn door_tools_thr_back(g: &mut crate::game::Game, op: DoorOpId, m: GuardId) -> bool { todo!() }

/// (src/game/doortools.ts:3644 DoorTools.cutsOwe)
pub fn door_tools_cuts_owe(g: &mut crate::game::Game, op: DoorOpId) -> bool { todo!() }

/// (src/game/doortools.ts:3653 DoorTools.thrFragWait)
pub fn door_tools_thr_frag_wait(g: &mut crate::game::Game, op: DoorOpId) -> bool { todo!() }

/// (src/game/doortools.ts:3660 DoorTools.fragWait)
pub fn door_tools_frag_wait(g: &mut crate::game::Game, op: DoorOpId) -> bool { todo!() }

/// (src/game/doortools.ts:3666 DoorTools.fragHeld)
pub fn door_tools_frag_held(g: &mut crate::game::Game, men: &[GuardId]) -> Vec<DoorOpId> { todo!() }

/// (src/game/doortools.ts:3674 DoorTools.fragHolds)
pub fn door_tools_frag_holds(g: &mut crate::game::Game, op: DoorOpId) -> bool { todo!() }

/// (src/game/doortools.ts:3688 DoorTools.fragBack)
pub fn door_tools_frag_back(g: &mut crate::game::Game, op: DoorOpId, m: GuardId) -> bool { todo!() }

/// (src/game/doortools.ts:3690 DoorTools.fragSaid)
pub fn door_tools_frag_said(g: &mut crate::game::Game, op: DoorOpId) { todo!() }

/// (src/game/doortools.ts:3695 DoorTools.fragOn)
pub fn door_tools_frag_on(g: &mut crate::game::Game, op: DoorOpId) { todo!() }

/// (src/game/doortools.ts:3699 DoorTools.fragStep)
pub fn door_tools_frag_step(g: &mut crate::game::Game, op: DoorOpId) -> bool { todo!() }

/// (src/game/doortools.ts:3714 DoorTools.thrAt)
pub fn door_tools_thr_at(g: &mut crate::game::Game, op: DoorOpId, m: GuardId) -> bool { todo!() }

/// (src/game/doortools.ts:3719 DoorTools.thrPick)
pub fn door_tools_thr_pick(g: &mut crate::game::Game, op: DoorOpId, hand: bool, fit: Option<&mut dyn FnMut(&mut crate::game::Game, GuardId) -> bool>) -> Option<GuardId> { todo!() }

/// (src/game/doortools.ts:3727 DoorTools.thrLitCell)
pub fn door_tools_thr_lit_cell(g: &mut crate::game::Game, t_key: DoorOpId, sr: DoortoolsRoomSearchId, i: f64) -> bool { todo!() }

/// (src/game/doortools.ts:3733 DoorTools.thrTubeCell)
pub fn door_tools_thr_tube_cell(g: &mut crate::game::Game, t_key: DoorOpId, sr: DoortoolsRoomSearchId, i: f64) -> bool { todo!() }

/// (src/game/doortools.ts:3739 DoorTools.handAt)
pub fn door_tools_hand_at(g: &mut crate::game::Game, op: DoorOpId, th: GuardId) -> V3 { todo!() }

/// (src/game/doortools.ts:3743 DoorTools.pocketOf)
pub fn door_tools_pocket_of(g: &mut crate::game::Game, op: DoorOpId, th: GuardId) -> Option<crate::doortools::DoorToolsPocketOf> { todo!() }

/// (src/game/doortools.ts:3760 DoorTools.thrBanged)
pub fn door_tools_thr_banged(g: &mut crate::game::Game, op: DoorOpId) { todo!() }

/// (src/game/doortools.ts:3772 DoorTools.shallowPods)
pub fn door_tools_shallow_pods(g: &mut crate::game::Game, d: DoorId, side: f64, n: Option<f64>) -> Vec<crate::entrypoints::Pod> { todo!() }

/// (src/game/doortools.ts:3791 DoorTools.stepIn)
pub fn door_tools_step_in(g: &mut crate::game::Game, op: DoorOpId) { todo!() }

/// (src/game/doortools.ts:3813 DoorTools.hudNotes)
pub fn door_tools_hud_notes(g: &mut crate::game::Game) -> Vec<String> { todo!() }

/// (src/game/doortools.ts:3828 DoorTools.cornerNotes)
pub fn door_tools_corner_notes(g: &mut crate::game::Game) -> Vec<crate::doortools::DoorToolsCornerNotesItem> { todo!() }

/// (src/game/doortools.ts:3839 DoorTools.wordNote)
pub fn door_tools_word_note(g: &mut crate::game::Game) -> Vec<crate::orders::OrderNote> { todo!() }

/// (src/game/doortools.ts:3850 DoorTools.summary)
pub fn door_tools_summary(g: &mut crate::game::Game) -> String { todo!() }


/// INVENTED: the call of what DoorTools.walks returned, a memo held in a local of the deal: the key is `${a[0].toFixed(2)},${a[2].toFixed(2)}>${b[0].toFixed(2)},${b[2].toFixed(2)}`, a miss is door_tools_walk_len(g, a, b, lim) (doortools.ts:3001 to 3004). A caller of away passes `&mut |g, a, b| door_tools_walks_len(g, &mut walks, a, b)`.
pub fn door_tools_walks_len(g: &mut crate::game::Game, w: &mut crate::doortools::Walks, a: V3, b: V3) -> f64 { todo!() }
