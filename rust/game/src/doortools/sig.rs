#![allow(unused_variables)]
//! Every function and method of `doortools`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/doortools.ts:749 doorPt)
pub fn door_pt(g: &mut crate::game::Game, d: DoorId, a: f64, b: f64, side: f64, y: Option<f64>) -> V3 { todo!() }

/// (src/game/doortools.ts:753 alongOf)
pub fn along_of(g: &mut crate::game::Game, d: DoorId, p: V3) -> f64 { todo!() }

/// (src/game/doortools.ts:755 offOf)
pub fn off_of(g: &mut crate::game::Game, d: DoorId, p: V3, side: f64) -> f64 { todo!() }

/// (src/game/doortools.ts:757 isOpen)
pub fn is_open(g: &mut crate::game::Game, d: DoorId) -> bool { todo!() }

/// (src/game/doortools.ts:761 underSky)
pub fn under_sky(g: &mut crate::game::Game, room: &crate::doortools::UnderSkyRoom) -> bool { todo!() }

/// (src/game/doortools.ts:768 openFloor)
pub fn open_floor(g: &mut crate::game::Game, room: &crate::doortools::OpenFloorRoom) -> bool { todo!() }

/// (src/game/doortools.ts:798 DoorTools.thrSigNow)
pub fn door_tools_thr_sig_now(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/doortools.ts:800 DoorTools.constructor)
pub fn door_tools_new(g: &mut crate::game::Game) -> crate::doortools::DoorTools { todo!() }

/// (src/game/doortools.ts:852 DoorTools.thrRows)
pub fn door_tools_thr_rows(g: &mut crate::game::Game) { todo!() }

/// (src/game/doortools.ts:881 DoorTools.take)
pub fn door_tools_take(g: &mut crate::game::Game, what: crate::doortools::DoorToolsTakeWhat) -> bool { todo!() }

/// (src/game/doortools.ts:888 DoorTools.refund)
pub fn door_tools_refund(g: &mut crate::game::Game, what: crate::doortools::DoorToolsRefundWhat) { todo!() }

/// (src/game/doortools.ts:890 DoorTools.sync)
pub fn door_tools_sync(g: &mut crate::game::Game) { todo!() }

/// (src/game/doortools.ts:940 DoorTools.reset)
pub fn door_tools_reset(g: &mut crate::game::Game) { todo!() }

/// (src/game/doortools.ts:947 DoorTools.alone)
pub fn door_tools_alone(g: &mut crate::game::Game, n: f64) { todo!() }

/// (src/game/doortools.ts:952 DoorTools.setBack)
pub fn door_tools_set_back(g: &mut crate::game::Game, d: DoorId, side: f64) { todo!() }

/// (src/game/doortools.ts:953 DoorTools.chargeOn)
pub fn door_tools_charge_on(g: &mut crate::game::Game, d: DoorId) -> Option<ChargeId> { todo!() }

/// (src/game/doortools.ts:955 DoorTools.stackable)
pub fn door_tools_stackable(g: &mut crate::game::Game, t: &crate::orders::OrderTarget, men: &[GuardId]) -> crate::doortools::DoorToolsStackable { todo!() }

/// (src/game/doortools.ts:965 DoorTools.lockedOut)
pub fn door_tools_locked_out(g: &mut crate::game::Game, d: DoorId, side: f64) -> bool { todo!() }

/// (src/game/doortools.ts:970 DoorTools.frameHeld)
pub fn door_tools_frame_held(g: &mut crate::game::Game, d: DoorId, side: f64) -> bool { todo!() }

/// (src/game/doortools.ts:973 DoorTools.noBoot)
pub fn door_tools_no_boot(g: &mut crate::game::Game, d: DoorId) -> String { todo!() }

/// (src/game/doortools.ts:976 DoorTools.frameAnswer)
pub fn door_tools_frame_answer(g: &mut crate::game::Game, op: DoorOpId) -> bool { todo!() }

/// (src/game/doortools.ts:984 DoorTools.willKick)
pub fn door_tools_will_kick(g: &mut crate::game::Game, op: DoorOpId) -> bool { todo!() }

/// (src/game/doortools.ts:990 DoorTools.bigOpens)
pub fn door_tools_big_opens(g: &mut crate::game::Game, d: DoorId, men: &[GuardId]) -> bool { todo!() }

/// (src/game/doortools.ts:993 DoorTools.bigWord)
pub fn door_tools_big_word(g: &mut crate::game::Game, t: &crate::orders::OrderTarget, men: &[GuardId]) -> crate::doortools::DoorToolsBigWord { todo!() }

/// (src/game/doortools.ts:998 DoorTools.bootSpot)
pub fn door_tools_boot_spot(g: &mut crate::game::Game, d: DoorId, side: f64) -> crate::doortools::DoorToolsBootSpot { todo!() }

/// (src/game/doortools.ts:1013 DoorTools.knows)
pub fn door_tools_knows(g: &mut crate::game::Game, d: DoorId, side: f64, free: bool) -> bool { todo!() }

/// (src/game/doortools.ts:1014 DoorTools.learn)
pub fn door_tools_learn(g: &mut crate::game::Game, d: DoorId, side: f64, free: Option<bool>) { todo!() }

/// (src/game/doortools.ts:1016 DoorTools.shutWhy)
pub fn door_tools_shut_why(g: &mut crate::game::Game, d: DoorId) -> String { todo!() }

/// (src/game/doortools.ts:1018 DoorTools.knownShut)
pub fn door_tools_known_shut(g: &mut crate::game::Game, d: DoorId, side: f64) -> bool { todo!() }

/// (src/game/doortools.ts:1025 DoorTools.untried)
pub fn door_tools_untried(g: &mut crate::game::Game, d: DoorId, side: f64) -> bool { todo!() }

/// (src/game/doortools.ts:1028 DoorTools.test)
pub fn door_tools_test(g: &mut crate::game::Game, d: DoorId, side: f64, gd: GuardId, say: bool) -> bool { todo!() }

/// (src/game/doortools.ts:1040 DoorTools.stackHeld)
pub fn door_tools_stack_held(g: &mut crate::game::Game, d: DoorId) -> bool { todo!() }

/// (src/game/doortools.ts:1043 DoorTools.testStacks)
pub fn door_tools_test_stacks(g: &mut crate::game::Game) { todo!() }

/// (src/game/doortools.ts:1057 DoorTools.haltAt)
pub fn door_tools_halt_at(g: &mut crate::game::Game, op: DoorOpId) -> bool { todo!() }

/// (src/game/doortools.ts:1070 DoorTools.shutSaid)
pub fn door_tools_shut_said(g: &mut crate::game::Game, lead: GuardId, d: DoorId) -> String { todo!() }

/// (src/game/doortools.ts:1077 DoorTools.walkTried)
pub fn door_tools_walk_tried(g: &mut crate::game::Game) { todo!() }

/// (src/game/doortools.ts:1104 DoorTools.haltBack)
pub fn door_tools_halt_back(g: &mut crate::game::Game, d: DoorId, side: f64, halted: &[GuardId]) { todo!() }

/// (src/game/doortools.ts:1112 DoorTools.plainStack)
pub fn door_tools_plain_stack(g: &mut crate::game::Game, op: DoorOpId, left: &[GuardId], note: Option<&str>) { todo!() }

/// (src/game/doortools.ts:1114 DoorTools.stackLeft)
pub fn door_tools_stack_left(g: &mut crate::game::Game, d: DoorId, post: &JsMap<GuardId, crate::doortools::DoorToolsStackLeftPostValue>, left: &[GuardId], note: Option<&str>) { todo!() }

/// (src/game/doortools.ts:1121 DoorTools.entryVerb)
pub fn door_tools_entry_verb(g: &mut crate::game::Game, t: &crate::orders::OrderTarget) -> crate::doortools::DoorToolsEntryVerb { todo!() }

/// (src/game/doortools.ts:1129 DoorTools.heldPlain)
pub fn door_tools_held_plain(g: &mut crate::game::Game, t: &crate::orders::OrderTarget, men: &[GuardId]) -> bool { todo!() }

/// (src/game/doortools.ts:1133 DoorTools.note)
pub fn door_tools_note(g: &mut crate::game::Game, what: &str) { todo!() }

/// (src/game/doortools.ts:1134 DoorTools.beat)
pub fn door_tools_beat(g: &mut crate::game::Game, op: DoorOpId, what: &str) { todo!() }

/// (src/game/doortools.ts:1138 DoorTools.assign)
pub fn door_tools_assign(g: &mut crate::game::Game, op: DoorOpId, gd: GuardId, a: &crate::doortools::DoorToolsAssignA) { todo!() }

/// (src/game/doortools.ts:1141 DoorTools.newOp)
pub fn door_tools_new_op(g: &mut crate::game::Game, kind: crate::doortools::DoorToolKind, d: DoorId, side: f64, men: &[GuardId], phase: crate::doortools::Phase) -> DoorOpId { todo!() }

/// (src/game/doortools.ts:1148 DoorTools.issue)
pub fn door_tools_issue(g: &mut crate::game::Game, kind: crate::doortools::DoorToolKind, men: &[GuardId], t: &crate::orders::OrderTarget, on_word: Option<bool>) -> crate::doortools::DoorToolsIssue { todo!() }

/// (src/game/doortools.ts:1150 DoorTools.together)
pub fn door_tools_together(g: &mut crate::game::Game, go: Option<bool>) -> DoorGateId { todo!() }

/// (src/game/doortools.ts:1154 DoorTools.order)
pub fn door_tools_order(g: &mut crate::game::Game, kind: crate::doortools::DoorToolsOrderKind, men: &[GuardId], door: DoorId, side: f64, gate: Option<DoorGateId>, hold: Option<bool>) -> crate::doortools::DoorToolsOrder { todo!() }

/// (src/game/doortools.ts:1168 DoorTools.threshold)
pub fn door_tools_threshold(g: &mut crate::game::Game, men: &[GuardId], t: &crate::orders::OrderTarget, on_word: Option<bool>, hold: Option<bool>) -> crate::doortools::DoorToolsThreshold { todo!() }

/// (src/game/doortools.ts:1169 DoorTools.thresholdHold)
pub fn door_tools_threshold_hold(g: &mut crate::game::Game, men: &[GuardId], t: &crate::orders::OrderTarget, on_word: Option<bool>) -> crate::doortools::DoorToolsThresholdHold { todo!() }

/// (src/game/doortools.ts:1170 DoorTools.canThreshold)
pub fn door_tools_can_threshold(g: &mut crate::game::Game, t: &crate::orders::OrderTarget, men: &[GuardId]) -> crate::doortools::DoorToolsCanThreshold { todo!() }

/// (src/game/doortools.ts:1174 DoorTools.thresholdOn)
pub fn door_tools_threshold_on(g: &mut crate::game::Game, door: &str, who: Option<crate::orders::Who>, side: Option<f64>, hold: Option<bool>) -> crate::doortools::DoorToolsThresholdOn { todo!() }

/// (src/game/doortools.ts:1183 DoorTools.thresholdHeld)
pub fn door_tools_threshold_held(g: &mut crate::game::Game, men: &[GuardId], door: Option<DoorId>) -> Vec<DoorOpId> { todo!() }

/// (src/game/doortools.ts:1185 DoorTools.heldOn)
pub fn door_tools_held_on(g: &mut crate::game::Game, t: &crate::orders::OrderTarget, men: &[GuardId]) -> Vec<DoorOpId> { todo!() }

/// (src/game/doortools.ts:1186 DoorTools.heldMen)
pub fn door_tools_held_men(g: &mut crate::game::Game, men: &[GuardId], t: &crate::orders::OrderTarget) -> Vec<GuardId> { todo!() }

/// (src/game/doortools.ts:1192 DoorTools.thresholdThen)
pub fn door_tools_threshold_then(g: &mut crate::game::Game, what: crate::doortools::DoorToolsThresholdThenWhat, men: &[GuardId], t: Option<&crate::doortools::DoorToolsThresholdThenT>) -> crate::doortools::DoorToolsThresholdThen { todo!() }

/// (src/game/doortools.ts:1214 DoorTools.thenOnWord)
pub fn door_tools_then_on_word(g: &mut crate::game::Game, what: crate::doortools::DoorToolsThenOnWordWhat, men: &[GuardId], t: &crate::orders::OrderTarget) -> crate::doortools::DoorToolsThenOnWord { todo!() }

/// (src/game/doortools.ts:1219 DoorTools.canThresholdThen)
pub fn door_tools_can_threshold_then(g: &mut crate::game::Game, what: crate::doortools::DoorToolsCanThresholdThenWhat, t: Option<&crate::doortools::DoorToolsCanThresholdThenT>, men: &[GuardId]) -> crate::doortools::DoorToolsCanThresholdThen { todo!() }

/// (src/game/doortools.ts:1227 DoorTools.toStep)
pub fn door_tools_to_step(g: &mut crate::game::Game, op: DoorOpId) { todo!() }

/// (src/game/doortools.ts:1230 DoorTools.thresholdTell)
pub fn door_tools_threshold_tell(g: &mut crate::game::Game, what: crate::doortools::DoorToolsThresholdTellWhat, door: Option<&str>, who: Option<crate::orders::Who>) -> Option<String> { todo!() }

/// (src/game/doortools.ts:1235 DoorTools.start)
pub fn door_tools_start(g: &mut crate::game::Game, kind: crate::doortools::DoorToolKind, men: &[GuardId], t: &crate::orders::OrderTarget, on_word: Option<bool>, hold: Option<bool>) -> crate::doortools::DoorToolsStart { todo!() }

/// (src/game/doortools.ts:1295 DoorTools.handOf)
pub fn door_tools_hand_of(g: &mut crate::game::Game, d: DoorId, side: f64, p: V3) -> f64 { todo!() }

/// (src/game/doortools.ts:1305 DoorTools.stayHands)
pub fn door_tools_stay_hands(g: &mut crate::game::Game, op: DoorOpId) { todo!() }

/// (src/game/doortools.ts:1314 DoorTools.crossesOpen)
pub fn door_tools_crosses_open(g: &mut crate::game::Game, d: DoorId, side: f64, from: V3, to: V3) -> bool { todo!() }

/// (src/game/doortools.ts:1320 DoorTools.keepHands)
pub fn door_tools_keep_hands(g: &mut crate::game::Game, op: DoorOpId, posts: &[crate::orders::StackPost], watch: V3) { todo!() }

/// (src/game/doortools.ts:1339 DoorTools.leadHand)
pub fn door_tools_lead_hand(g: &mut crate::game::Game, op: DoorOpId, posts: &[crate::orders::StackPost], watch: V3) { todo!() }

/// (src/game/doortools.ts:1357 DoorTools.laneCover)
pub fn door_tools_lane_cover(g: &mut crate::game::Game, op: DoorOpId) { todo!() }

/// (src/game/doortools.ts:1375 DoorTools.laneIn)
pub fn door_tools_lane_in(g: &mut crate::game::Game, op: DoorOpId) { todo!() }

/// (src/game/doortools.ts:1377 DoorTools.startCharge)
pub fn door_tools_start_charge(g: &mut crate::game::Game, men: &[GuardId], t: &crate::orders::OrderTarget, by_hand: Option<bool>) -> crate::doortools::DoorToolsStartCharge { todo!() }

/// (src/game/doortools.ts:1396 DoorTools.looseOn)
pub fn door_tools_loose_on(g: &mut crate::game::Game, t: &crate::orders::OrderTarget) -> bool { todo!() }

/// (src/game/doortools.ts:1403 DoorTools.standoffFits)
pub fn door_tools_standoff_fits(g: &mut crate::game::Game, men: &[GuardId], t: &crate::orders::OrderTarget, own: Option<bool>) -> crate::doortools::DoorToolsStandoffFits { todo!() }

/// (src/game/doortools.ts:1411 DoorTools.standoffWalk)
pub fn door_tools_standoff_walk(g: &mut crate::game::Game, men: &[GuardId], d: DoorId, posts: &[crate::orders::StackPost]) -> Option<String> { todo!() }

/// (src/game/doortools.ts:1420 DoorTools.closeWords)
pub fn door_tools_close_words(g: &mut crate::game::Game, c_2: ChargeId) -> String { todo!() }

/// (src/game/doortools.ts:1441 DoorTools.samFace)
pub fn door_tools_sam_face(g: &mut crate::game::Game, d: DoorId) -> Option<f64> { todo!() }

/// (src/game/doortools.ts:1449 DoorTools.samRows)
pub fn door_tools_sam_rows(g: &mut crate::game::Game, it: &mut crate::game::Interactable, d: DoorId, verbs: &mut Vec<crate::game::Interactable>, free: bool, row: &mut dyn FnMut(&mut crate::game::Game, crate::game::Interactable, crate::game::Verb, String, Option<String>) -> crate::game::Interactable) { todo!() }

/// (src/game/doortools.ts:1458 DoorTools.keyIs)
pub fn door_tools_key_is(g: &mut crate::game::Game, c_2: ChargeId) -> bool { todo!() }

/// (src/game/doortools.ts:1460 DoorTools.samSet)
pub fn door_tools_sam_set(g: &mut crate::game::Game) -> Option<ChargeId> { todo!() }

/// (src/game/doortools.ts:1462 DoorTools.samWhere)
pub fn door_tools_sam_where(g: &mut crate::game::Game, c_2: ChargeId) -> String { todo!() }

/// (src/game/doortools.ts:1464 DoorTools.samJob)
pub fn door_tools_sam_job(g: &mut crate::game::Game) -> Option<crate::doortools::DoorToolsSamJob> { todo!() }

/// (src/game/doortools.ts:1470 DoorTools.samProgress)
pub fn door_tools_sam_progress(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/doortools.ts:1472 DoorTools.cancelSam)
pub fn door_tools_cancel_sam(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/doortools.ts:1477 DoorTools.samTick)
pub fn door_tools_sam_tick(g: &mut crate::game::Game) { todo!() }

/// (src/game/doortools.ts:1503 DoorTools.tickSet)
pub fn door_tools_tick_set(g: &mut crate::game::Game) { todo!() }

/// (src/game/doortools.ts:1515 DoorTools.standoffDeal)
pub fn door_tools_standoff_deal(g: &mut crate::game::Game, men: &[GuardId], d: DoorId, posts: &[crate::orders::StackPost]) -> Vec<(GuardId, crate::orders::StackPost)> { todo!() }

/// (src/game/doortools.ts:1520 DoorTools.toLock)
pub fn door_tools_to_lock(g: &mut crate::game::Game, op: DoorOpId) { todo!() }

/// (src/game/doortools.ts:1525 DoorTools.toPick)
pub fn door_tools_to_pick(g: &mut crate::game::Game, op: DoorOpId) { todo!() }

/// (src/game/doortools.ts:1531 DoorTools.pickable)
pub fn door_tools_pickable(g: &mut crate::game::Game, d: DoorId) -> bool { todo!() }

/// (src/game/doortools.ts:1533 DoorTools.offLock)
pub fn door_tools_off_lock(g: &mut crate::game::Game, op: DoorOpId) { todo!() }

/// (src/game/doortools.ts:1539 DoorTools.pickStep)
pub fn door_tools_pick_step(g: &mut crate::game::Game, op: DoorOpId) { todo!() }

/// (src/game/doortools.ts:1567 DoorTools.toGun)
pub fn door_tools_to_gun(g: &mut crate::game::Game, op: DoorOpId) -> bool { todo!() }

/// (src/game/doortools.ts:1580 DoorTools.gunOff)
pub fn door_tools_gun_off(g: &mut crate::game::Game, op: DoorOpId, why: &str) { todo!() }

/// (src/game/doortools.ts:1589 DoorTools.gunStep)
pub fn door_tools_gun_step(g: &mut crate::game::Game, op: DoorOpId) { todo!() }

/// (src/game/doortools.ts:1625 DoorTools.hammerStep)
pub fn door_tools_hammer_step(g: &mut crate::game::Game, op: DoorOpId) { todo!() }

/// (src/game/doortools.ts:1644 DoorTools.slingTick)
pub fn door_tools_sling_tick(g: &mut crate::game::Game) { todo!() }

/// (src/game/doortools.ts:1647 DoorTools.newLead)
pub fn door_tools_new_lead(g: &mut crate::game::Game, op: DoorOpId) { todo!() }

/// (src/game/doortools.ts:1665 DoorTools.stackPosts)
pub fn door_tools_stack_posts(g: &mut crate::game::Game, d: DoorId, side: f64, men: &[GuardId], ignore: Option<&[GuardId]>) -> Option<Vec<crate::orders::StackPost>> { todo!() }

/// (src/game/doortools.ts:1668 DoorTools.stackTucked)
pub fn door_tools_stack_tucked(g: &mut crate::game::Game, d: DoorId, side: f64, men: &[GuardId], ignore: &[GuardId]) -> Option<Vec<crate::orders::StackPost>> { todo!() }

/// (src/game/doortools.ts:1685 DoorTools.holders)
pub fn door_tools_holders(g: &mut crate::game::Game, d: DoorId, side: f64, men: &[GuardId]) -> Vec<GuardId> { todo!() }

/// (src/game/doortools.ts:1695 DoorTools.stepBacks)
pub fn door_tools_step_backs(g: &mut crate::game::Game, d: DoorId, side: f64, men: &[GuardId], taken: &[V3], must: Option<bool>) -> Option<JsMap<GuardId, crate::orders::StackPost>> { todo!() }

/// (src/game/doortools.ts:1712 DoorTools.dealStack)
pub fn door_tools_deal_stack(g: &mut crate::game::Game, d: DoorId, side: f64, men: &[GuardId]) -> Option<crate::doortools::DoorToolsDealStack> { todo!() }

/// (src/game/doortools.ts:1722 DoorTools.makeRoom)
pub fn door_tools_make_room(g: &mut crate::game::Game, d: DoorId, back: &JsMap<GuardId, crate::orders::StackPost>) { todo!() }

/// (src/game/doortools.ts:1732 DoorTools.fragBehind)
pub fn door_tools_frag_behind(g: &mut crate::game::Game, d: DoorId, side: f64, men: &[GuardId]) -> Option<String> { todo!() }

/// (src/game/doortools.ts:1739 DoorTools.behind)
pub fn door_tools_behind(g: &mut crate::game::Game, d: DoorId, side: f64, men: &[GuardId]) -> Option<GuardId> { todo!() }

/// (src/game/doortools.ts:1747 DoorTools.canReach)
pub fn door_tools_can_reach(g: &mut crate::game::Game, d: DoorId, side: f64, p: V3) -> bool { todo!() }

/// (src/game/doortools.ts:1753 DoorTools.samBehind)
pub fn door_tools_sam_behind(g: &mut crate::game::Game, d: DoorId, side: f64) -> bool { todo!() }

/// (src/game/doortools.ts:1761 DoorTools.mateBehind)
pub fn door_tools_mate_behind(g: &mut crate::game::Game, op: DoorOpId) -> Option<GuardId> { todo!() }

/// (src/game/doortools.ts:1767 DoorTools.canBehind)
pub fn door_tools_can_behind(g: &mut crate::game::Game, d: DoorId, side: f64, men: &[GuardId]) -> Option<String> { todo!() }

/// (src/game/doortools.ts:1774 DoorTools.waiting4)
pub fn door_tools_waiting4(g: &mut crate::game::Game, men: &[GuardId]) -> Vec<crate::doortools::DoorToolsWaiting4Item> { todo!() }

/// (src/game/doortools.ts:1781 DoorTools.stacksOf)
pub fn door_tools_stacks_of(g: &mut crate::game::Game, men: &[GuardId]) -> Vec<crate::doortools::DoorToolsStacksOfItem> { todo!() }

/// (src/game/doortools.ts:1791 DoorTools.stackWaits)
pub fn door_tools_stack_waits(g: &mut crate::game::Game, men: &[GuardId]) -> Vec<crate::doortools::DoorToolsStackWaitsItem> { todo!() }

/// (src/game/doortools.ts:1792 DoorTools.stackWords)
pub fn door_tools_stack_words(g: &mut crate::game::Game, men: &[GuardId]) -> String { todo!() }

/// (src/game/doortools.ts:1794 DoorTools.waitingMen)
pub fn door_tools_waiting_men(g: &mut crate::game::Game, men: &[GuardId]) -> Vec<GuardId> { todo!() }

/// (src/game/doortools.ts:1798 DoorTools.wordMen)
pub fn door_tools_word_men(g: &mut crate::game::Game) -> Vec<GuardId> { todo!() }

/// (src/game/doortools.ts:1801 DoorTools.waitingAny)
pub fn door_tools_waiting_any(g: &mut crate::game::Game, men: &[GuardId]) -> Vec<crate::doortools::DoorToolsWaitingAnyItem> { todo!() }

/// (src/game/doortools.ts:1804 DoorTools.waitsOn)
pub fn door_tools_waits_on(g: &mut crate::game::Game, men: &[GuardId], d: DoorId) -> Option<crate::doortools::DoorToolsWaitsOn> { todo!() }

/// (src/game/doortools.ts:1810 DoorTools.wordPending)
pub fn door_tools_word_pending(g: &mut crate::game::Game, c_2: ChargeId) -> bool { todo!() }

/// (src/game/doortools.ts:1812 DoorTools.waitingWords)
pub fn door_tools_waiting_words(g: &mut crate::game::Game, men: &[GuardId]) -> String { todo!() }

/// (src/game/doortools.ts:1817 DoorTools.go)
pub fn door_tools_go(g: &mut crate::game::Game, who: Option<crate::orders::Who>) -> Option<String> { todo!() }

/// (src/game/doortools.ts:1821 DoorTools.wordQuiet)
pub fn door_tools_word_quiet(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/doortools.ts:1826 DoorTools.wordStepIn)
pub fn door_tools_word_step_in(g: &mut crate::game::Game, op: DoorOpId) { todo!() }

/// (src/game/doortools.ts:1835 DoorTools.wordStepGo)
pub fn door_tools_word_step_go(g: &mut crate::game::Game, op: DoorOpId) { todo!() }

/// (src/game/doortools.ts:1841 DoorTools.wordOwed)
pub fn door_tools_word_owed(g: &mut crate::game::Game, op: DoorOpId) -> bool { todo!() }

/// (src/game/doortools.ts:1849 DoorTools.wordSteps)
pub fn door_tools_word_steps(g: &mut crate::game::Game, op: DoorOpId) -> bool { todo!() }

/// (src/game/doortools.ts:1852 DoorTools.wordOnHold)
pub fn door_tools_word_on_hold(g: &mut crate::game::Game, men: &[GuardId]) -> bool { todo!() }

/// (src/game/doortools.ts:1861 DoorTools.wordEarly)
pub fn door_tools_word_early(g: &mut crate::game::Game, men: &[GuardId]) -> bool { todo!() }

/// (src/game/doortools.ts:1867 DoorTools.wordKeyHeld)
pub fn door_tools_word_key_held(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/doortools.ts:1872 DoorTools.pairWord)
pub fn door_tools_pair_word(g: &mut crate::game::Game, men: &[GuardId]) -> bool { todo!() }

/// (src/game/doortools.ts:1876 DoorTools.goMen)
pub fn door_tools_go_men(g: &mut crate::game::Game, men: &[GuardId], only: Option<&[crate::doortools::DoorToolsGoMenOnlyItem]>) -> Option<String> { todo!() }

/// (src/game/doortools.ts:1892 DoorTools.fromStack)
pub fn door_tools_from_stack(g: &mut crate::game::Game, men: &[GuardId], d: DoorId) -> Option<DoorOpId> { todo!() }

/// (src/game/doortools.ts:1908 DoorTools.wordTick)
pub fn door_tools_word_tick(g: &mut crate::game::Game) { todo!() }

/// (src/game/doortools.ts:1935 DoorTools.open)
pub fn door_tools_open(g: &mut crate::game::Game, op: DoorOpId) { todo!() }

/// (src/game/doortools.ts:1961 DoorTools.toHandle)
pub fn door_tools_to_handle(g: &mut crate::game::Game, op: DoorOpId, gate: bool) -> bool { todo!() }

/// (src/game/doortools.ts:1969 DoorTools.bootBeside)
pub fn door_tools_boot_beside(g: &mut crate::game::Game, op: DoorOpId) -> bool { todo!() }

/// (src/game/doortools.ts:1976 DoorTools.handStep)
pub fn door_tools_hand_step(g: &mut crate::game::Game, op: DoorOpId, el: f64) { todo!() }

/// (src/game/doortools.ts:1988 DoorTools.standoffPosts)
pub fn door_tools_standoff_posts(g: &mut crate::game::Game, d: DoorId, side: f64, n: f64, men: Option<&[GuardId]>) -> Option<Vec<crate::orders::StackPost>> { todo!() }

/// (src/game/doortools.ts:2002 DoorTools.standoffPostsAsWere)
pub fn door_tools_standoff_posts_as_were(g: &mut crate::game::Game, d: DoorId, side: f64, n: f64, men: Option<&[GuardId]>, files: Option<&mut Vec<Vec<crate::orders::StackPost>>>) -> Option<Vec<crate::orders::StackPost>> { todo!() }

/// (src/game/doortools.ts:2048 DoorTools.menuKey)
pub fn door_tools_menu_key(g: &mut crate::game::Game, input: InputId) -> bool { todo!() }

/// (src/game/doortools.ts:2056 DoorTools.bigKey)
pub fn door_tools_big_key(g: &mut crate::game::Game, input: InputId) -> bool { todo!() }

/// (src/game/doortools.ts:2062 DoorTools.input)
pub fn door_tools_input(g: &mut crate::game::Game, input: InputId) { todo!() }

/// (src/game/doortools.ts:2105 DoorTools.onPendingWord)
pub fn door_tools_on_pending_word(g: &mut crate::game::Game, c_2: ChargeId) -> bool { todo!() }

/// (src/game/doortools.ts:2108 DoorTools.trigger)
pub fn door_tools_trigger(g: &mut crate::game::Game, c_2: Option<ChargeId>) { todo!() }

/// (src/game/doortools.ts:2113 DoorTools.nearest)
pub fn door_tools_nearest(g: &mut crate::game::Game, of: Option<&mut dyn FnMut(&mut crate::game::Game, ChargeId) -> bool>) -> Option<ChargeId> { todo!() }

/// (src/game/doortools.ts:2115 DoorTools.tick)
pub fn door_tools_tick(g: &mut crate::game::Game) { todo!() }

/// (src/game/doortools.ts:2146 DoorTools.arrived)
pub fn door_tools_arrived(g: &mut crate::game::Game, op: DoorOpId, gd: GuardId) -> bool { todo!() }

/// (src/game/doortools.ts:2148 DoorTools.spareRear)
pub fn door_tools_spare_rear(g: &mut crate::game::Game, op: DoorOpId) { todo!() }

/// (src/game/doortools.ts:2154 DoorTools.waiting)
pub fn door_tools_waiting(g: &mut crate::game::Game, op: DoorOpId, el: f64) -> bool { todo!() }

/// (src/game/doortools.ts:2159 DoorTools.busy)
pub fn door_tools_busy(g: &mut crate::game::Game, op: DoorOpId, el: f64) -> bool { todo!() }

/// (src/game/doortools.ts:2162 DoorTools.end)
pub fn door_tools_end(g: &mut crate::game::Game, op: DoorOpId, why: &str) { todo!() }

/// (src/game/doortools.ts:2173 DoorTools.short)
pub fn door_tools_short(g: &mut crate::game::Game, op: DoorOpId, why: &str) { todo!() }

/// (src/game/doortools.ts:2186 DoorTools.chargeGone)
pub fn door_tools_charge_gone(g: &mut crate::game::Game, op: DoorOpId, hands: bool) { todo!() }

/// (src/game/doortools.ts:2192 DoorTools.knownOpen)
pub fn door_tools_known_open(g: &mut crate::game::Game, d: DoorId) -> bool { todo!() }

/// (src/game/doortools.ts:2194 DoorTools.defuse)
pub fn door_tools_defuse(g: &mut crate::game::Game, op: DoorOpId) { todo!() }

/// (src/game/doortools.ts:2197 DoorTools.step)
pub fn door_tools_step(g: &mut crate::game::Game, op: DoorOpId) { todo!() }

/// (src/game/doortools.ts:2583 DoorTools.closeUp)
pub fn door_tools_close_up(g: &mut crate::game::Game, op: DoorOpId, gd: Option<GuardId>) { todo!() }

/// (src/game/doortools.ts:2590 DoorTools.backToJamb)
pub fn door_tools_back_to_jamb(g: &mut crate::game::Game, op: DoorOpId, gd: GuardId) { todo!() }

/// (src/game/doortools.ts:2595 DoorTools.shove)
pub fn door_tools_shove(g: &mut crate::game::Game, op: DoorOpId) { todo!() }

/// (src/game/doortools.ts:2602 DoorTools.opened)
pub fn door_tools_opened(g: &mut crate::game::Game, op: DoorOpId) { todo!() }

/// (src/game/doortools.ts:2610 DoorTools.landAt)
pub fn door_tools_land_at(g: &mut crate::game::Game, op: DoorOpId, deep: Option<f64>) -> V3 { todo!() }

/// (src/game/doortools.ts:2613 DoorTools.bangSpot)
pub fn door_tools_bang_spot(g: &mut crate::game::Game, op: DoorOpId, from: V3) -> Option<crate::doortools::DoorToolsBangSpot> { todo!() }

/// (src/game/doortools.ts:2643 DoorTools.landFor)
pub fn door_tools_land_for(g: &mut crate::game::Game, d: DoorId, side: f64, deep: f64) -> V3 { todo!() }

/// (src/game/doortools.ts:2652 DoorTools.bang)
pub fn door_tools_bang(g: &mut crate::game::Game, op: DoorOpId, it: ItemId) { todo!() }

/// (src/game/doortools.ts:2668 DoorTools.cornerStep)
pub fn door_tools_corner_step(g: &mut crate::game::Game, op: DoorOpId, gd: GuardId, a: OrdersAssignmentId, pod: crate::entrypoints::Pod) -> bool { todo!() }

/// (src/game/doortools.ts:2684 DoorTools.onPoint)
pub fn door_tools_on_point(g: &mut crate::game::Game, gd: GuardId, a: OrdersAssignmentId, pod: crate::entrypoints::Pod) -> bool { todo!() }

/// (src/game/doortools.ts:2687 DoorTools.owesCorner)
pub fn door_tools_owes_corner(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/doortools.ts:2697 DoorTools.threatNote)
pub fn door_tools_threat_note(g: &mut crate::game::Game, op: DoorOpId) { todo!() }

/// (src/game/doortools.ts:2721 DoorTools.sees)
pub fn door_tools_sees(g: &mut crate::game::Game, op: DoorOpId, p: V3) -> bool { todo!() }

/// (src/game/doortools.ts:2725 DoorTools.threatDeal)
pub fn door_tools_threat_deal(g: &mut crate::game::Game, op: DoorOpId) { todo!() }

/// (src/game/doortools.ts:2734 DoorTools.threatStep)
pub fn door_tools_threat_step(g: &mut crate::game::Game, op: DoorOpId, gd: GuardId, a: OrdersAssignmentId, pod: crate::entrypoints::Pod) -> bool { todo!() }

/// (src/game/doortools.ts:2764 DoorTools.firstIn)
pub fn door_tools_first_in(g: &mut crate::game::Game, op: DoorOpId, at: Option<V3>, by_gun: Option<bool>) -> GuardId { todo!() }

/// (src/game/doortools.ts:2776 DoorTools.seenMost)
pub fn door_tools_seen_most(g: &mut crate::game::Game, lead: GuardId, men: &[GuardId], at: Option<V3>) -> GuardId { todo!() }

/// (src/game/doortools.ts:2785 DoorTools.inAt)
pub fn door_tools_in_at(g: &mut crate::game::Game, op: DoorOpId, gd: GuardId) -> f64 { todo!() }

/// (src/game/doortools.ts:2787 DoorTools.onSector)
pub fn door_tools_on_sector(g: &mut crate::game::Game, gd: GuardId, a: OrdersAssignmentId, pod: crate::entrypoints::Pod) -> f64 { todo!() }

/// (src/game/doortools.ts:2794 DoorTools.doorKeepers)
pub fn door_tools_door_keepers(g: &mut crate::game::Game) -> Vec<(GuardId, DoorId, OrdersAssignmentId)> { todo!() }

/// (src/game/doortools.ts:2799 DoorTools.keepSectors)
pub fn door_tools_keep_sectors(g: &mut crate::game::Game) { todo!() }

/// (src/game/doortools.ts:2821 DoorTools.onOther)
pub fn door_tools_on_other(g: &mut crate::game::Game, op: DoorOpId, gd: GuardId) -> bool { todo!() }

/// (src/game/doortools.ts:2829 DoorTools.otherDoors)
pub fn door_tools_other_doors(g: &mut crate::game::Game, op: DoorOpId, ended: Option<bool>, deal: Option<bool>, seen: Option<bool>) -> bool { todo!() }

/// (src/game/doortools.ts:2877 DoorTools.leafAcross)
pub fn door_tools_leaf_across(g: &mut crate::game::Game, o: DoorId, p: V3, at: V3) -> f64 { todo!() }

/// (src/game/doortools.ts:2890 DoorTools.holdDoors)
pub fn door_tools_hold_doors(g: &mut crate::game::Game, op: DoorOpId) { todo!() }

/// (src/game/doortools.ts:2942 DoorTools.roomIn)
pub fn door_tools_room_in(g: &mut crate::game::Game, op: DoorOpId) -> Option<RoomfightRoomId> { todo!() }

/// (src/game/doortools.ts:2943 DoorTools.cornersOf)
pub fn door_tools_corners_of(g: &mut crate::game::Game, op: DoorOpId, pod: crate::entrypoints::Pod) -> Vec<V3> { todo!() }

/// (src/game/doortools.ts:2955 DoorTools.searchOf)
pub fn door_tools_search_of(g: &mut crate::game::Game, op: DoorOpId, at: Option<V3>) -> Option<DoortoolsRoomSearchId> { todo!() }

/// (src/game/doortools.ts:2965 DoorTools.lineTo)
pub fn door_tools_line_to(g: &mut crate::game::Game, sr: DoortoolsRoomSearchId, gd: GuardId, i: f64) -> bool { todo!() }

/// (src/game/doortools.ts:2972 DoorTools.look)
pub fn door_tools_look(g: &mut crate::game::Game, op: DoorOpId, sr: DoortoolsRoomSearchId, out: Option<&[GuardId]>, ok: Option<&mut dyn FnMut(&mut crate::game::Game, GuardId, f64) -> bool>) { todo!() }

/// (src/game/doortools.ts:2983 DoorTools.scan)
pub fn door_tools_scan(g: &mut crate::game::Game, op: DoorOpId, sr: DoortoolsRoomSearchId, gd: GuardId, a: OrdersAssignmentId) -> bool { todo!() }

/// (src/game/doortools.ts:2996 DoorTools.walkLen)
pub fn door_tools_walk_len(g: &mut crate::game::Game, a: V3, b: V3, lim: f64) -> f64 { todo!() }

/// (src/game/doortools.ts:3001 DoorTools.walks)
pub fn door_tools_walks(g: &mut crate::game::Game) -> crate::FnField { todo!() }

/// (src/game/doortools.ts:3019 DoorTools.away)
pub fn door_tools_away(g: &mut crate::game::Game, gd: GuardId, p: V3, pool: &[GuardId], where_: &mut dyn FnMut(&mut crate::game::Game, GuardId) -> V3, len: &mut dyn FnMut(&mut crate::game::Game, V3, V3) -> f64, pair: Option<bool>, on_door: Option<&mut dyn FnMut(&mut crate::game::Game, GuardId) -> bool>) -> crate::doortools::DoorToolsAway { todo!() }

/// (src/game/doortools.ts:3043 DoorTools.besidePod)
pub fn door_tools_beside_pod(g: &mut crate::game::Game, p: V3, face: f64, mate: GuardId, taken: &[V3], in_room: &mut dyn FnMut(&mut crate::game::Game, V3) -> bool) -> Option<crate::entrypoints::Pod> { todo!() }

/// (src/game/doortools.ts:3059 DoorTools.sendOn)
pub fn door_tools_send_on(g: &mut crate::game::Game, op: DoorOpId, gd: GuardId, pod: &mut crate::entrypoints::Pod, going: &str) { todo!() }

/// (src/game/doortools.ts:3066 DoorTools.flows)
pub fn door_tools_flows(g: &mut crate::game::Game, op: DoorOpId, sr: DoortoolsRoomSearchId) -> bool { todo!() }

/// (src/game/doortools.ts:3075 DoorTools.flowIn)
pub fn door_tools_flow_in(g: &mut crate::game::Game, op: DoorOpId, sr: DoortoolsRoomSearchId, men: &[GuardId]) -> bool { todo!() }

/// (src/game/doortools.ts:3159 DoorTools.moveOn)
pub fn door_tools_move_on(g: &mut crate::game::Game, op: DoorOpId, sr: DoortoolsRoomSearchId) -> bool { todo!() }

/// (src/game/doortools.ts:3202 DoorTools.pods)
pub fn door_tools_pods(g: &mut crate::game::Game, d: DoorId, side: f64, n: Option<f64>) -> Vec<crate::entrypoints::Pod> { todo!() }

/// (src/game/doortools.ts:3212 DoorTools.offSwing)
pub fn door_tools_off_swing(g: &mut crate::game::Game, p: &mut crate::doortools::DoorToolsOffSwingP, d: Option<DoorId>, side: f64, taken: &[V3], see: Option<V3>) { todo!() }

/// (src/game/doortools.ts:3221 DoorTools.podsAsWere)
pub fn door_tools_pods_as_were(g: &mut crate::game::Game, d: DoorId, side: f64, n: Option<f64>) -> Vec<crate::entrypoints::Pod> { todo!() }

/// (src/game/doortools.ts:3233 DoorTools.inFrame)
pub fn door_tools_in_frame(g: &mut crate::game::Game, d: DoorId, mid: V3, p: V3) -> bool { todo!() }

/// (src/game/doortools.ts:3245 DoorTools.pods2)
pub fn door_tools_pods2(g: &mut crate::game::Game, d: DoorId, side: f64) -> Vec<crate::entrypoints::Pod> { todo!() }

/// (src/game/doortools.ts:3265 DoorTools.enter)
pub fn door_tools_enter(g: &mut crate::game::Game, op: DoorOpId) { todo!() }

/// (src/game/doortools.ts:3302 DoorTools.podsApart)
pub fn door_tools_pods_apart(g: &mut crate::game::Game, op: DoorOpId, pods: &[crate::entrypoints::Pod], n: f64) -> Vec<crate::entrypoints::Pod> { todo!() }

/// (src/game/doortools.ts:3312 DoorTools.blow)
pub fn door_tools_blow(g: &mut crate::game::Game, c_2: Option<ChargeId>) -> Option<crate::doortools::DoorToolsBlow> { todo!() }

/// (src/game/doortools.ts:3313 DoorTools.blowRaw)
pub fn door_tools_blow_raw(g: &mut crate::game::Game, c_2: Option<ChargeId>) -> Option<crate::doortools::DoorToolsBlowRaw> { todo!() }

/// (src/game/doortools.ts:3359 DoorTools.dazzle)
pub fn door_tools_dazzle(g: &mut crate::game::Game, gd: GuardId, secs: f64, at: V3) { todo!() }

/// (src/game/doortools.ts:3367 DoorTools.drawCharge)
pub fn door_tools_draw_charge(g: &mut crate::game::Game, c_2: ChargeId) { todo!() }

/// (src/game/doortools.ts:3378 DoorTools.pieNext)
pub fn door_tools_pie_next(g: &mut crate::game::Game, op: DoorOpId, gd: GuardId, s: &crate::doortools::DoorToolsPieNextS) -> f64 { todo!() }

/// (src/game/doortools.ts:3388 DoorTools.pieOff)
pub fn door_tools_pie_off(g: &mut crate::game::Game, op: DoorOpId) { todo!() }

/// (src/game/doortools.ts:3399 DoorTools.pieSet)
pub fn door_tools_pie_set(g: &mut crate::game::Game, op: DoorOpId, gd: GuardId, sgn: f64, first: &[V3], off: &[V3]) -> Vec<V3> { todo!() }

/// (src/game/doortools.ts:3417 DoorTools.sliceLooks)
pub fn door_tools_slice_looks(g: &mut crate::game::Game, op: DoorOpId, p: V3, sgn: f64) -> Vec<V3> { todo!() }

/// (src/game/doortools.ts:3426 DoorTools.toSlice)
pub fn door_tools_to_slice(g: &mut crate::game::Game, op: DoorOpId) { todo!() }

/// (src/game/doortools.ts:3461 DoorTools.leafOurs)
pub fn door_tools_leaf_ours(g: &mut crate::game::Game, d: DoorId, side: f64) -> bool { todo!() }

/// (src/game/doortools.ts:3464 DoorTools.thrSpot)
pub fn door_tools_thr_spot(g: &mut crate::game::Game, op: DoorOpId, sgn: f64, mark: V3) -> crate::doortools::DoorToolsThrSpot { todo!() }

/// (src/game/doortools.ts:3471 DoorTools.thrFar)
pub fn door_tools_thr_far(g: &mut crate::game::Game, op: DoorOpId) -> Option<crate::doortools::DoorToolsThrFar> { todo!() }

/// (src/game/doortools.ts:3480 DoorTools.thrSetFar)
pub fn door_tools_thr_set_far(g: &mut crate::game::Game, op: DoorOpId) { todo!() }

/// (src/game/doortools.ts:3489 DoorTools.lastCall)
pub fn door_tools_last_call(g: &mut crate::game::Game, op: DoorOpId) -> bool { todo!() }

/// (src/game/doortools.ts:3497 DoorTools.lastStack)
pub fn door_tools_last_stack(g: &mut crate::game::Game, op: DoorOpId) { todo!() }

/// (src/game/doortools.ts:3520 DoorTools.toThreshold)
pub fn door_tools_to_threshold(g: &mut crate::game::Game, op: DoorOpId) { todo!() }

/// (src/game/doortools.ts:3557 DoorTools.canFirst)
pub fn door_tools_can_first(g: &mut crate::game::Game, op: DoorOpId) -> bool { todo!() }

/// (src/game/doortools.ts:3567 DoorTools.burstDry)
pub fn door_tools_burst_dry(g: &mut crate::game::Game, op: DoorOpId) -> bool { todo!() }

/// (src/game/doortools.ts:3573 DoorTools.thrOut)
pub fn door_tools_thr_out(g: &mut crate::game::Game, op: DoorOpId, why: Option<&str>) { todo!() }

/// (src/game/doortools.ts:3583 DoorTools.thrMen)
pub fn door_tools_thr_men(g: &mut crate::game::Game, op: DoorOpId) { todo!() }

/// (src/game/doortools.ts:3595 DoorTools.thrWatch)
pub fn door_tools_thr_watch(g: &mut crate::game::Game, op: DoorOpId) { todo!() }

/// (src/game/doortools.ts:3616 DoorTools.thrCans)
pub fn door_tools_thr_cans(g: &mut crate::game::Game, t: &crate::doortools::Threshold) -> f64 { todo!() }

/// (src/game/doortools.ts:3623 DoorTools.toPockets)
pub fn door_tools_to_pockets(g: &mut crate::game::Game, op: DoorOpId) { todo!() }

/// (src/game/doortools.ts:3632 DoorTools.heldBeat)
pub fn door_tools_held_beat(g: &mut crate::game::Game, op: DoorOpId) { todo!() }

/// (src/game/doortools.ts:3637 DoorTools.thrThrower)
pub fn door_tools_thr_thrower(g: &mut crate::game::Game, op: DoorOpId) -> GuardId { todo!() }

/// (src/game/doortools.ts:3640 DoorTools.thrBack)
pub fn door_tools_thr_back(g: &mut crate::game::Game, op: DoorOpId, m: GuardId) -> bool { todo!() }

/// (src/game/doortools.ts:3643 DoorTools.cutsOwe)
pub fn door_tools_cuts_owe(g: &mut crate::game::Game, op: DoorOpId) -> bool { todo!() }

/// (src/game/doortools.ts:3652 DoorTools.thrFragWait)
pub fn door_tools_thr_frag_wait(g: &mut crate::game::Game, op: DoorOpId) -> bool { todo!() }

/// (src/game/doortools.ts:3659 DoorTools.fragWait)
pub fn door_tools_frag_wait(g: &mut crate::game::Game, op: DoorOpId) -> bool { todo!() }

/// (src/game/doortools.ts:3665 DoorTools.fragHeld)
pub fn door_tools_frag_held(g: &mut crate::game::Game, men: &[GuardId]) -> Vec<DoorOpId> { todo!() }

/// (src/game/doortools.ts:3673 DoorTools.fragHolds)
pub fn door_tools_frag_holds(g: &mut crate::game::Game, op: DoorOpId) -> bool { todo!() }

/// (src/game/doortools.ts:3687 DoorTools.fragBack)
pub fn door_tools_frag_back(g: &mut crate::game::Game, op: DoorOpId, m: GuardId) -> bool { todo!() }

/// (src/game/doortools.ts:3689 DoorTools.fragSaid)
pub fn door_tools_frag_said(g: &mut crate::game::Game, op: DoorOpId) { todo!() }

/// (src/game/doortools.ts:3694 DoorTools.fragOn)
pub fn door_tools_frag_on(g: &mut crate::game::Game, op: DoorOpId) { todo!() }

/// (src/game/doortools.ts:3698 DoorTools.fragStep)
pub fn door_tools_frag_step(g: &mut crate::game::Game, op: DoorOpId) -> bool { todo!() }

/// (src/game/doortools.ts:3713 DoorTools.thrAt)
pub fn door_tools_thr_at(g: &mut crate::game::Game, op: DoorOpId, m: GuardId) -> bool { todo!() }

/// (src/game/doortools.ts:3718 DoorTools.thrPick)
pub fn door_tools_thr_pick(g: &mut crate::game::Game, op: DoorOpId, hand: bool, fit: Option<&mut dyn FnMut(&mut crate::game::Game, GuardId) -> bool>) -> Option<GuardId> { todo!() }

/// (src/game/doortools.ts:3726 DoorTools.thrLitCell)
pub fn door_tools_thr_lit_cell(g: &mut crate::game::Game, t_key: DoorOpId, sr: DoortoolsRoomSearchId, i: f64) -> bool { todo!() }

/// (src/game/doortools.ts:3732 DoorTools.thrTubeCell)
pub fn door_tools_thr_tube_cell(g: &mut crate::game::Game, t_key: DoorOpId, sr: DoortoolsRoomSearchId, i: f64) -> bool { todo!() }

/// (src/game/doortools.ts:3738 DoorTools.handAt)
pub fn door_tools_hand_at(g: &mut crate::game::Game, op: DoorOpId, th: GuardId) -> V3 { todo!() }

/// (src/game/doortools.ts:3742 DoorTools.pocketOf)
pub fn door_tools_pocket_of(g: &mut crate::game::Game, op: DoorOpId, th: GuardId) -> Option<crate::doortools::DoorToolsPocketOf> { todo!() }

/// (src/game/doortools.ts:3759 DoorTools.thrBanged)
pub fn door_tools_thr_banged(g: &mut crate::game::Game, op: DoorOpId) { todo!() }

/// (src/game/doortools.ts:3771 DoorTools.shallowPods)
pub fn door_tools_shallow_pods(g: &mut crate::game::Game, d: DoorId, side: f64, n: Option<f64>) -> Vec<crate::entrypoints::Pod> { todo!() }

/// (src/game/doortools.ts:3790 DoorTools.stepIn)
pub fn door_tools_step_in(g: &mut crate::game::Game, op: DoorOpId) { todo!() }

/// (src/game/doortools.ts:3812 DoorTools.hudNotes)
pub fn door_tools_hud_notes(g: &mut crate::game::Game) -> Vec<String> { todo!() }

/// (src/game/doortools.ts:3827 DoorTools.cornerNotes)
pub fn door_tools_corner_notes(g: &mut crate::game::Game) -> Vec<crate::doortools::DoorToolsCornerNotesItem> { todo!() }

/// (src/game/doortools.ts:3838 DoorTools.wordNote)
pub fn door_tools_word_note(g: &mut crate::game::Game) -> Vec<crate::orders::OrderNote> { todo!() }

/// (src/game/doortools.ts:3849 DoorTools.summary)
pub fn door_tools_summary(g: &mut crate::game::Game) -> String { todo!() }
