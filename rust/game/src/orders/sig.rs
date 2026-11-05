#![allow(unused_variables)]
//! Every function and method of `orders`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/orders.ts:337 digitOf)
pub fn digit_of(code: &str) -> f64 { todo!() }

/// (src/game/orders.ts:338 keyName)
pub fn key_name(code: &str) -> String { todo!() }

/// (src/game/orders.ts:341 compass)
pub fn compass(yaw: f64) -> String { todo!() }

/// (src/game/orders.ts:349 hoverStore)
pub fn hover_store() -> crate::orders::HoverKept { todo!() }

/// (src/game/orders.ts:562 SquadOrders.show)
pub fn squad_orders_show(g: &mut crate::game::Game, men: &[GuardId]) { todo!() }

/// (src/game/orders.ts:578 SquadOrders.constructor)
pub fn squad_orders_new(g: &mut crate::game::Game, cast: Option<&crate::orders::OtherCast>) -> crate::orders::SquadOrders { todo!() }

/// (src/game/orders.ts:642 SquadOrders.active)
pub fn squad_orders_active(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/orders.ts:648 SquadOrders.squad)
pub fn squad_orders_squad(g: &mut crate::game::Game) -> Vec<GuardId> { todo!() }

/// (src/game/orders.ts:652 SquadOrders.mine)
pub fn squad_orders_mine(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/orders.ts:654 SquadOrders.sams)
pub fn squad_orders_sams(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/orders.ts:656 SquadOrders.team)
pub fn squad_orders_team(g: &mut crate::game::Game) -> crate::teams::Team { todo!() }

/// (src/game/orders.ts:658 SquadOrders.ring)
pub fn squad_orders_ring(g: &mut crate::game::Game) -> JsSet<String> { todo!() }

/// (src/game/orders.ts:660 SquadOrders.recipients)
pub fn squad_orders_recipients(g: &mut crate::game::Game, t: Option<&crate::orders::SquadOrdersRecipientsT>, who: Option<crate::orders::SquadOrdersRecipientsWho>) -> Vec<GuardId> { todo!() }

/// (src/game/orders.ts:667 SquadOrders.men)
pub fn squad_orders_men(g: &mut crate::game::Game, sel: Option<crate::orders::SquadOrdersMenSel>, opts: Option<crate::orders::SquadOrdersMenOpts>) -> Vec<GuardId> { todo!() }

/// (src/game/orders.ts:674 SquadOrders.big)
pub fn squad_orders_big(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/orders.ts:676 SquadOrders.pairOf)
pub fn squad_orders_pair_of(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::game::PairId> { todo!() }

/// (src/game/orders.ts:678 SquadOrders.pairs)
pub fn squad_orders_pairs(g: &mut crate::game::Game) -> Vec<crate::game::PairId> { todo!() }

/// (src/game/orders.ts:680 SquadOrders.paired)
pub fn squad_orders_paired(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/orders.ts:683 SquadOrders.cycleWho)
pub fn squad_orders_cycle_who(g: &mut crate::game::Game, dir: Option<f64>) -> crate::orders::Who { todo!() }

/// (src/game/orders.ts:690 SquadOrders.whole)
pub fn squad_orders_whole(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/orders.ts:692 SquadOrders.address)
pub fn squad_orders_address(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/orders.ts:694 SquadOrders.whoWords)
pub fn squad_orders_who_words(g: &mut crate::game::Game, men: Option<&[GuardId]>) -> String { todo!() }

/// (src/game/orders.ts:698 SquadOrders.toneOf)
pub fn squad_orders_tone_of(g: &mut crate::game::Game, men: &[GuardId]) -> Option<crate::orders::Tone> { todo!() }

/// (src/game/orders.ts:699 SquadOrders.has)
pub fn squad_orders_has(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/orders.ts:700 SquadOrders.of)
pub fn squad_orders_of(g: &mut crate::game::Game, gd: GuardId) -> Option<OrdersAssignmentId> { todo!() }

/// (src/game/orders.ts:702 SquadOrders.followWord)
pub fn squad_orders_follow_word(g: &mut crate::game::Game, a: OrdersAssignmentId, d: f64) { todo!() }

/// (src/game/orders.ts:708 SquadOrders.shownThere)
pub fn squad_orders_shown_there(g: &mut crate::game::Game, a: OrdersAssignmentId) -> bool { todo!() }

/// (src/game/orders.ts:712 SquadOrders.register)
pub fn squad_orders_register(g: &mut crate::game::Game, def: OrdersOrderDefId, before: Option<&str>) { todo!() }

/// (src/game/orders.ts:726 SquadOrders.newGroup)
pub fn squad_orders_new_group(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/orders.ts:727 SquadOrders.unregister)
pub fn squad_orders_unregister(g: &mut crate::game::Game, id_2: &str) { todo!() }

/// (src/game/orders.ts:729 SquadOrders.clear)
pub fn squad_orders_clear(g: &mut crate::game::Game, men: Option<&[GuardId]>) { todo!() }

/// (src/game/orders.ts:735 SquadOrders.targetAt)
pub fn squad_orders_target_at(g: &mut crate::game::Game, p: V3) -> crate::orders::OrderTarget { todo!() }

/// (src/game/orders.ts:754 SquadOrders.doorOnRay)
pub fn squad_orders_door_on_ray(g: &mut crate::game::Game, ro: V3, rd: V3, t_scene: Option<f64>) -> Option<DoorId> { todo!() }

/// (src/game/orders.ts:771 SquadOrders.targetUnder)
pub fn squad_orders_target_under(g: &mut crate::game::Game, input: InputId, canvas: crate::host_hud::View) -> crate::orders::OrderTarget { todo!() }

/// (src/game/orders.ts:775 SquadOrders.targetUnderOf)
pub fn squad_orders_target_under_of(g: &mut crate::game::Game, input: InputId, canvas: crate::host_hud::View) -> crate::orders::OrderTarget { todo!() }

/// (src/game/orders.ts:803 SquadOrders.sceneHit)
pub fn squad_orders_scene_hit(g: &mut crate::game::Game, ro: V3, rd: V3, first: Option<crate::orders::SquadOrdersSceneHitFirst>) -> crate::orders::SquadOrdersSceneHit { todo!() }

/// (src/game/orders.ts:815 SquadOrders.rowsFor)
pub fn squad_orders_rows_for(g: &mut crate::game::Game, t: &crate::orders::OrderTarget, page: Option<&str>, men: Option<&[GuardId]>) -> Vec<OrdersOrderDefId> { todo!() }

/// (src/game/orders.ts:824 SquadOrders.flatRing)
pub fn squad_orders_flat_ring(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/orders.ts:828 SquadOrders.flatRows)
pub fn squad_orders_flat_rows(g: &mut crate::game::Game, t: &crate::orders::OrderTarget, men: Option<&[GuardId]>) -> Vec<OrdersOrderDefId> { todo!() }

/// (src/game/orders.ts:838 SquadOrders.menuRows)
pub fn squad_orders_menu_rows(g: &mut crate::game::Game, m: &crate::orders::Menu, t: &crate::orders::OrderTarget) -> Vec<OrdersOrderDefId> { todo!() }

/// (src/game/orders.ts:840 SquadOrders.seat)
pub fn squad_orders_seat(g: &mut crate::game::Game, def: OrdersOrderDefId, stood: OrdersOrderDefId) { todo!() }

/// (src/game/orders.ts:842 SquadOrders.wordRowOf)
pub fn squad_orders_word_row_of(g: &mut crate::game::Game, t: &crate::orders::OrderTarget, rows: &[OrdersOrderDefId]) -> bool { todo!() }

/// (src/game/orders.ts:844 SquadOrders.wordPage)
pub fn squad_orders_word_page(g: &mut crate::game::Game, m: &crate::orders::Menu) -> Option<String> { todo!() }

/// (src/game/orders.ts:846 SquadOrders.waitDigit)
pub fn squad_orders_wait_digit(g: &mut crate::game::Game, m: &crate::orders::Menu) -> f64 { todo!() }

/// (src/game/orders.ts:848 SquadOrders.whyNot)
pub fn squad_orders_why_not(g: &mut crate::game::Game, d: OrdersOrderDefId, t: &crate::orders::OrderTarget, men: Option<&[GuardId]>) -> Option<String> { todo!() }

/// (src/game/orders.ts:854 SquadOrders.labelOf)
pub fn squad_orders_label_of(g: &mut crate::game::Game, d: OrdersOrderDefId, t: &crate::orders::OrderTarget) -> String { todo!() }

/// (src/game/orders.ts:859 SquadOrders.give)
pub fn squad_orders_give(g: &mut crate::game::Game, id_2: &str, t: Option<&crate::orders::OrderTarget>, who: Option<crate::orders::SquadOrdersGiveWho>, opts: Option<&crate::orders::GiveOpts>) -> Option<String> { todo!() }

/// (src/game/orders.ts:891 SquadOrders.givenLine)
pub fn squad_orders_given_line(g: &mut crate::game::Game, def: OrdersOrderDefId, t: &crate::orders::OrderTarget, men: &[GuardId]) -> String { todo!() }

/// (src/game/orders.ts:897 SquadOrders.again)
pub fn squad_orders_again(g: &mut crate::game::Game, id_2: &str, t: &crate::orders::OrderTarget, men: &[GuardId], lines: &[String]) { todo!() }

/// (src/game/orders.ts:903 SquadOrders.assign)
pub fn squad_orders_assign(g: &mut crate::game::Game, gd: GuardId, a: &crate::orders::SquadOrdersAssignA) { todo!() }

/// (src/game/orders.ts:915 SquadOrders.spareRear)
pub fn squad_orders_spare_rear(g: &mut crate::game::Game, men: &[GuardId], door: DoorId, lead: Option<GuardId>, from: Option<&JsMap<GuardId, V3>>) -> Vec<GuardId> { todo!() }

/// (src/game/orders.ts:928 SquadOrders.loaded)
pub fn squad_orders_loaded(g: &mut crate::game::Game) { todo!() }

/// (src/game/orders.ts:929 SquadOrders.escape)
pub fn squad_orders_escape(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/orders.ts:934 SquadOrders.open)
pub fn squad_orders_open(g: &mut crate::game::Game, target: Option<&crate::orders::OrderTarget>, flat: Option<bool>, part: Option<&[GuardId]>) -> bool { todo!() }

/// (src/game/orders.ts:941 SquadOrders.close)
pub fn squad_orders_close(g: &mut crate::game::Game) { todo!() }

/// (src/game/orders.ts:945 SquadOrders.turn)
pub fn squad_orders_turn(g: &mut crate::game::Game, page: Option<&str>, on_word: Option<bool>) { todo!() }

/// (src/game/orders.ts:956 SquadOrders.pageBack)
pub fn squad_orders_page_back(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/orders.ts:960 SquadOrders.pick)
pub fn squad_orders_pick(g: &mut crate::game::Game, i: Option<f64>) -> Option<String> { todo!() }

/// (src/game/orders.ts:973 SquadOrders.partGive)
pub fn squad_orders_part_give(g: &mut crate::game::Game, m: &crate::orders::Menu, give: &mut dyn FnMut(&mut crate::game::Game) -> Option<String>) -> Option<String> { todo!() }

/// (src/game/orders.ts:979 SquadOrders.menuTarget)
pub fn squad_orders_menu_target(g: &mut crate::game::Game) -> Option<crate::orders::SquadOrdersMenuTarget> { todo!() }

/// (src/game/orders.ts:982 SquadOrders.reread)
pub fn squad_orders_reread(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/orders.ts:991 SquadOrders.refresh)
pub fn squad_orders_refresh(g: &mut crate::game::Game) { todo!() }

/// (src/game/orders.ts:1010 SquadOrders.due)
pub fn squad_orders_due(g: &mut crate::game::Game, m: &crate::orders::Menu) -> bool { todo!() }

/// (src/game/orders.ts:1021 SquadOrders.hover)
pub fn squad_orders_hover(g: &mut crate::game::Game, kept: Option<&mut crate::orders::HoverKept>) -> Option<crate::orders::SquadOrdersHover> { todo!() }

/// (src/game/orders.ts:1030 SquadOrders.defaultFor)
pub fn squad_orders_default_for(g: &mut crate::game::Game, t: &crate::orders::OrderTarget) -> Option<OrdersOrderDefId> { todo!() }

/// (src/game/orders.ts:1033 SquadOrders.whoAfterT)
pub fn squad_orders_who_after_t(g: &mut crate::game::Game, point: Option<&crate::orders::SquadOrdersWhoAfterTPoint>, dir: Option<f64>) -> crate::orders::Who { todo!() }

/// (src/game/orders.ts:1041 SquadOrders.stepWho)
pub fn squad_orders_step_who(g: &mut crate::game::Game, dir: f64, point: Option<&crate::orders::SquadOrdersStepWhoPoint>) { todo!() }

/// (src/game/orders.ts:1051 SquadOrders.hostView)
pub fn squad_orders_host_view(g: &mut crate::game::Game, input: InputId, canvas: crate::host_hud::View) { todo!() }

/// (src/game/orders.ts:1052 SquadOrders.input)
pub fn squad_orders_input(g: &mut crate::game::Game, _dt: f64, input: InputId, canvas: crate::host_hud::View) { todo!() }

/// (src/game/orders.ts:1104 SquadOrders.ringInput)
pub fn squad_orders_ring_input(g: &mut crate::game::Game, input: InputId, canvas: crate::host_hud::View) -> bool { todo!() }

/// (src/game/orders.ts:1196 SquadOrders.ringDigit)
pub fn squad_orders_ring_digit(g: &mut crate::game::Game, input: InputId) -> bool { todo!() }

/// (src/game/orders.ts:1198 SquadOrders.ringUp)
pub fn squad_orders_ring_up(g: &mut crate::game::Game, t: &crate::orders::OrderTarget, input: InputId, canvas: crate::host_hud::View, t0: f64, g_2: Option<&crate::orders::SquadOrdersRingUpG>) -> bool { todo!() }

/// (src/game/orders.ts:1211 SquadOrders.goalPart)
pub fn squad_orders_goal_part(g: &mut crate::game::Game, t: &crate::orders::OrderTarget, add: bool) -> Option<crate::orders::SquadOrdersGoalPart> { todo!() }

/// (src/game/orders.ts:1218 SquadOrders.ringFree)
pub fn squad_orders_ring_free(g: &mut crate::game::Game, input: InputId) -> bool { todo!() }

/// (src/game/orders.ts:1220 SquadOrders.backFree)
pub fn squad_orders_back_free(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/orders.ts:1225 SquadOrders.hintView)
pub fn squad_orders_hint_view(g: &mut crate::game::Game, kept: Option<&mut crate::orders::HoverKept>) -> Option<crate::orders::SquadOrdersHintView> { todo!() }

/// (src/game/orders.ts:1249 SquadOrders.ringSel)
pub fn squad_orders_ring_sel(g: &mut crate::game::Game) { todo!() }

/// (src/game/orders.ts:1251 SquadOrders.seats)
pub fn squad_orders_seats(g: &mut crate::game::Game, m: &crate::orders::Menu, r: Option<crate::cmdring::RingState>) -> crate::orders::SquadOrdersSeats { todo!() }

/// (src/game/orders.ts:1264 SquadOrders.tapGive)
pub fn squad_orders_tap_give(g: &mut crate::game::Game, t: &crate::orders::OrderTarget, input: InputId, goal: Option<bool>) { todo!() }

/// (src/game/orders.ts:1276 SquadOrders.twiceOn)
pub fn squad_orders_twice_on(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/orders.ts:1278 SquadOrders.allIn)
pub fn squad_orders_all_in(g: &mut crate::game::Game) { todo!() }

/// (src/game/orders.ts:1282 SquadOrders.quickGive)
pub fn squad_orders_quick_give(g: &mut crate::game::Game, t: &crate::orders::OrderTarget) { todo!() }

/// (src/game/orders.ts:1286 SquadOrders.quickFor)
pub fn squad_orders_quick_for(g: &mut crate::game::Game, men: &[GuardId], t: &crate::orders::OrderTarget) -> Option<crate::orders::SquadOrdersQuickFor> { todo!() }

/// (src/game/orders.ts:1296 SquadOrders.coverFor)
pub fn squad_orders_cover_for(g: &mut crate::game::Game, men: &[GuardId], at: V3) -> bool { todo!() }

/// (src/game/orders.ts:1302 SquadOrders.quickWords)
pub fn squad_orders_quick_words(g: &mut crate::game::Game, t: &crate::orders::OrderTarget, p: Option<&crate::autopilot::AutoPick>) -> Option<String> { todo!() }

/// (src/game/orders.ts:1305 SquadOrders.menuQuick)
pub fn squad_orders_menu_quick(g: &mut crate::game::Game, m: &crate::orders::Menu) -> Option<String> { todo!() }

/// (src/game/orders.ts:1308 SquadOrders.manAt)
pub fn squad_orders_man_at(g: &mut crate::game::Game, input: InputId, canvas: crate::host_hud::View) -> f64 { todo!() }

/// (src/game/orders.ts:1318 SquadOrders.ringKeys)
pub fn squad_orders_ring_keys(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/orders.ts:1326 SquadOrders.ringView)
pub fn squad_orders_ring_view(g: &mut crate::game::Game) -> Option<crate::cmdring::RingView> { todo!() }

/// (src/game/orders.ts:1343 SquadOrders.sync)
pub fn squad_orders_sync(g: &mut crate::game::Game) -> Vec<GuardId> { todo!() }

/// (src/game/orders.ts:1352 SquadOrders.samStopped)
pub fn squad_orders_sam_stopped(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/orders.ts:1354 SquadOrders.drop)
pub fn squad_orders_drop(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/orders.ts:1356 SquadOrders.tick)
pub fn squad_orders_tick(g: &mut crate::game::Game) { todo!() }

/// (src/game/orders.ts:1535 SquadOrders.aside)
pub fn squad_orders_aside(g: &mut crate::game::Game, gd: GuardId) -> Option<V3> { todo!() }

/// (src/game/orders.ts:1556 SquadOrders.crumb)
pub fn squad_orders_crumb(g: &mut crate::game::Game, p: V3, men: &[GuardId]) { todo!() }

/// (src/game/orders.ts:1571 SquadOrders.ahead)
pub fn squad_orders_ahead(g: &mut crate::game::Game, gd: GuardId, a: OrdersAssignmentId) -> Vec<V3> { todo!() }

/// (src/game/orders.ts:1572 SquadOrders.onTop)
pub fn squad_orders_on_top(g: &mut crate::game::Game, gd: GuardId, a: OrdersAssignmentId) -> bool { todo!() }

/// (src/game/orders.ts:1577 SquadOrders.inFrame)
pub fn squad_orders_in_frame(g: &mut crate::game::Game, p: V3) -> bool { todo!() }

/// (src/game/orders.ts:1592 SquadOrders.restClear)
pub fn squad_orders_rest_clear(g: &mut crate::game::Game, gd: GuardId, a: OrdersAssignmentId, place: V3, trail: V3, stopped: bool) { todo!() }

/// (src/game/orders.ts:1617 SquadOrders.heldWait)
pub fn squad_orders_held_wait(g: &mut crate::game::Game, a: OrdersAssignmentId) -> bool { todo!() }

/// (src/game/orders.ts:1628 SquadOrders.restShort)
pub fn squad_orders_rest_short(g: &mut crate::game::Game, gd: GuardId, a: OrdersAssignmentId, d: f64) -> bool { todo!() }

/// (src/game/orders.ts:1645 SquadOrders.clearBack)
pub fn squad_orders_clear_back(g: &mut crate::game::Game, follow: f64) -> f64 { todo!() }

/// (src/game/orders.ts:1659 SquadOrders.backOf)
pub fn squad_orders_back_of(g: &mut crate::game::Game, follow: f64) -> f64 { todo!() }

/// (src/game/orders.ts:1662 SquadOrders.trailBack)
pub fn squad_orders_trail_back(g: &mut crate::game::Game, gd: GuardId, extra: f64) -> Option<V3> { todo!() }

/// (src/game/orders.ts:1669 SquadOrders.apartSlot)
pub fn squad_orders_apart_slot(g: &mut crate::game::Game, gd: GuardId, a: OrdersAssignmentId, slot: V3) -> Option<V3> { todo!() }

/// (src/game/orders.ts:1693 SquadOrders.slot)
pub fn squad_orders_slot(g: &mut crate::game::Game, back: f64) -> V3 { todo!() }

/// (src/game/orders.ts:1720 SquadOrders.freeSpot)
pub fn squad_orders_free_spot(g: &mut crate::game::Game, p: V3) -> Option<V3> { todo!() }

/// (src/game/orders.ts:1728 SquadOrders.spotsAt)
pub fn squad_orders_spots_at(g: &mut crate::game::Game, at: V3, face: f64, n: f64) -> Vec<V3> { todo!() }

/// (src/game/orders.ts:1754 SquadOrders.stackPosts)
pub fn squad_orders_stack_posts(g: &mut crate::game::Game, d: DoorId, side: f64, men: &[GuardId], ignore: Option<&[GuardId]>) -> Option<Vec<crate::orders::StackPost>> { todo!() }

/// (src/game/orders.ts:1760 SquadOrders.stackPostsWait)
pub fn squad_orders_stack_posts_wait(g: &mut crate::game::Game, d: DoorId, side: f64, men: &[GuardId]) -> Option<Vec<crate::orders::StackPost>> { todo!() }

/// (src/game/orders.ts:1773 SquadOrders.stackSeen)
pub fn squad_orders_stack_seen(g: &mut crate::game::Game, d: DoorId, side: f64, men: &[GuardId], ignore: &[GuardId], ask: &mut dyn FnMut(&mut crate::game::Game) -> Option<Vec<crate::orders::StackPost>>) -> Option<Vec<crate::orders::StackPost>> { todo!() }

/// (src/game/orders.ts:1802 SquadOrders.marksKeep)
pub fn squad_orders_marks_keep(g: &mut crate::game::Game) -> crate::orders::MarksKeep { todo!() }

/// (src/game/orders.ts:1813 SquadOrders.stackFileOn)
pub fn squad_orders_stack_file_on(g: &mut crate::game::Game, d: DoorId, side: f64, men: &[GuardId], ignore: &[GuardId], latch: bool, replaced: &[crate::orders::StackPost]) -> Option<Vec<crate::orders::StackPost>> { todo!() }

/// (src/game/orders.ts:1835 SquadOrders.stackPostsRoom)
pub fn squad_orders_stack_posts_room(g: &mut crate::game::Game, d: DoorId, side: f64, men: &[GuardId], ignore: Option<&[GuardId]>) -> Option<Vec<crate::orders::StackPost>> { todo!() }

/// (src/game/orders.ts:1844 SquadOrders.ownMarkFirst)
pub fn squad_orders_own_mark_first(g: &mut crate::game::Game, d: DoorId, side: f64, men: &[GuardId], ask: &mut dyn FnMut(&mut crate::game::Game) -> Option<Vec<crate::orders::SquadOrdersOwnMarkFirstAskRetItem>>) -> Option<Vec<crate::orders::SquadOrdersOwnMarkFirstAskRetItem>> { todo!() }

/// (src/game/orders.ts:1858 SquadOrders.ownMarks)
pub fn squad_orders_own_marks(g: &mut crate::game::Game, d: DoorId, side: f64, men: &[GuardId]) -> Vec<V3> { todo!() }

/// (src/game/orders.ts:1869 SquadOrders.noRoomWords)
pub fn squad_orders_no_room_words(g: &mut crate::game::Game, plain: &str, ask: &mut dyn FnMut(&mut crate::game::Game, &mut crate::game::Game) -> bool) -> String { todo!() }

/// (src/game/orders.ts:1878 SquadOrders.samPlace)
pub fn squad_orders_sam_place(g: &mut crate::game::Game) -> V3 { todo!() }

/// (src/game/orders.ts:1881 SquadOrders.samStood)
pub fn squad_orders_sam_stood(g: &mut crate::game::Game) -> V3 { todo!() }

/// (src/game/orders.ts:1882 SquadOrders.stackPostsMarks)
pub fn squad_orders_stack_posts_marks(g: &mut crate::game::Game, d: DoorId, side: f64, men: &[GuardId], ignore: Option<&[GuardId]>) -> Option<Vec<crate::orders::StackPost>> { todo!() }

/// (src/game/orders.ts:1896 SquadOrders.stackPostsAsWere)
pub fn squad_orders_stack_posts_as_were(g: &mut crate::game::Game, d: DoorId, side: f64, men: &[GuardId], ignore: Option<&[GuardId]>) -> Option<Vec<crate::orders::StackPost>> { todo!() }

/// (src/game/orders.ts:1907 SquadOrders.stackPostsLeafIn)
pub fn squad_orders_stack_posts_leaf_in(g: &mut crate::game::Game, d: DoorId, side: f64, men: &[GuardId], ignore: Option<&[GuardId]>) -> Option<Vec<crate::orders::StackPost>> { todo!() }

/// (src/game/orders.ts:1918 SquadOrders.behindGlass)
pub fn squad_orders_behind_glass(g: &mut crate::game::Game, d: DoorId, side: f64, pos: V3) -> bool { todo!() }

/// (src/game/orders.ts:1920 SquadOrders.glassEyes)
pub fn squad_orders_glass_eyes(g: &mut crate::game::Game, d: DoorId, side: f64, pos: V3, kept: Option<bool>) -> f64 { todo!() }

/// (src/game/orders.ts:1936 SquadOrders.stackLeastGlass)
pub fn squad_orders_stack_least_glass(g: &mut crate::game::Game, d: DoorId, side: f64, men: &[GuardId], ignore: &[GuardId]) -> Option<Vec<crate::orders::StackPost>> { todo!() }

/// (src/game/orders.ts:1959 SquadOrders.hallStands)
pub fn squad_orders_hall_stands(g: &mut crate::game::Game, d: DoorId, side: f64, pos: V3) -> bool { todo!() }

/// (src/game/orders.ts:1977 SquadOrders.hallPlaces)
pub fn squad_orders_hall_places(g: &mut crate::game::Game, d: DoorId, side: f64, k: f64, from: f64, taken: &mut dyn FnMut(&mut crate::game::Game, V3) -> bool) -> Vec<crate::orders::StackPost> { todo!() }

/// (src/game/orders.ts:2007 SquadOrders.stackPostsOn)
pub fn squad_orders_stack_posts_on(g: &mut crate::game::Game, d: DoorId, side: f64, men: &[GuardId], ignore: &[GuardId], glass: bool, hall: Option<bool>) -> Option<Vec<crate::orders::StackPost>> { todo!() }

/// (src/game/orders.ts:2012 SquadOrders.stackPlaces)
pub fn squad_orders_stack_places(g: &mut crate::game::Game, d: DoorId, side: f64, men: &[GuardId], ignore: &[GuardId], glass: bool, hall: bool) -> Option<Vec<crate::orders::StackPost>> { todo!() }

/// (src/game/orders.ts:2106 SquadOrders.workedMarks)
pub fn squad_orders_worked_marks(g: &mut crate::game::Game, d: DoorId, side: f64) -> crate::orders::SquadOrdersWorkedMarks { todo!() }

/// (src/game/orders.ts:2129 SquadOrders.heldOn)
pub fn squad_orders_held_on(g: &mut crate::game::Game, d: DoorId, men: &[GuardId], ignore: Option<&[GuardId]>) -> Vec<V3> { todo!() }

/// (src/game/orders.ts:2133 SquadOrders.workedPick)
pub fn squad_orders_worked_pick(g: &mut crate::game::Game, d: DoorId, side: f64, n: f64, taken: &[V3], min_m: Option<f64>) -> Option<Vec<V3>> { todo!() }

/// (src/game/orders.ts:2143 SquadOrders.workedStack)
pub fn squad_orders_worked_stack(g: &mut crate::game::Game, d: DoorId, side: f64, n: f64, taken: &[V3]) -> Option<Vec<crate::orders::StackPost>> { todo!() }

/// (src/game/orders.ts:2150 SquadOrders.stoodOn)
pub fn squad_orders_stood_on(g: &mut crate::game::Game, d: DoorId, men: &[GuardId], p: V3) -> bool { todo!() }

/// (src/game/orders.ts:2157 SquadOrders.onFurniture)
pub fn squad_orders_on_furniture(g: &mut crate::game::Game, p: V3) -> bool { todo!() }

/// (src/game/orders.ts:2168 SquadOrders.rearPosts)
pub fn squad_orders_rear_posts(g: &mut crate::game::Game, d: DoorId, side: f64, k: f64, taken: &[V3]) -> Vec<crate::orders::StackPost> { todo!() }

/// (src/game/orders.ts:2185 SquadOrders.forMen)
pub fn squad_orders_for_men(g: &mut crate::game::Game, t: &crate::orders::OrderTarget, men: &[GuardId]) -> crate::orders::OrderTarget { todo!() }

/// (src/game/orders.ts:2186 SquadOrders.sideFor)
pub fn squad_orders_side_for(g: &mut crate::game::Game, d: DoorId, men: &[GuardId], sam_side: f64) -> f64 { todo!() }

/// (src/game/orders.ts:2214 SquadOrders.holdArcs)
pub fn squad_orders_hold_arcs(g: &mut crate::game::Game, at: &[V3]) -> Vec<Option<crate::orders::SquadOrdersHoldArcsItem>> { todo!() }

/// (src/game/orders.ts:2250 SquadOrders.route)
pub fn squad_orders_route(g: &mut crate::game::Game, gd: GuardId, to: V3, shy: Option<DoorId>, thru: Option<DoorId>, why_only: Option<bool>) -> crate::orders::SquadOrdersRoute { todo!() }

/// (src/game/orders.ts:2329 SquadOrders.sameWays)
pub fn squad_orders_same_ways(g: &mut crate::game::Game, men: &[GuardId]) { todo!() }

/// (src/game/orders.ts:2366 SquadOrders.byDoor)
pub fn squad_orders_by_door(g: &mut crate::game::Game, from: V3, to: V3, d: DoorId, plain: &[V3]) -> Option<Vec<V3>> { todo!() }

/// (src/game/orders.ts:2385 SquadOrders.roundRooms)
pub fn squad_orders_round_rooms(g: &mut crate::game::Game, gd: GuardId, to: V3) -> Vec<V3> { todo!() }

/// (src/game/orders.ts:2398 SquadOrders.slotCrumb)
pub fn squad_orders_slot_crumb(g: &mut crate::game::Game, back: f64) -> f64 { todo!() }

/// (src/game/orders.ts:2413 SquadOrders.trailWay)
pub fn squad_orders_trail_way(g: &mut crate::game::Game, gd: GuardId, slot: V3, on_trail: V3, j: f64) -> Vec<V3> { todo!() }

/// (src/game/orders.ts:2434 SquadOrders.reach)
pub fn squad_orders_reach(g: &mut crate::game::Game, gd: GuardId, to: V3) -> Option<String> { todo!() }

/// (src/game/orders.ts:2437 SquadOrders.walkTry)
pub fn squad_orders_walk_try(g: &mut crate::game::Game, gd: GuardId, to: V3) -> Option<crate::orders::SquadOrdersWalkTry> { todo!() }

/// (src/game/orders.ts:2444 SquadOrders.walkStacks)
pub fn squad_orders_walk_stacks(g: &mut crate::game::Game, men: &[GuardId], to: V3) -> Option<crate::orders::SquadOrdersWalkStacks> { todo!() }

/// (src/game/orders.ts:2454 SquadOrders.walkSend)
pub fn squad_orders_walk_send(g: &mut crate::game::Game, gd: GuardId, t: crate::orders::SquadOrdersWalkSendT, at: V3, on: &crate::orders::SquadOrdersWalkSendOn) { todo!() }

/// (src/game/orders.ts:2461 SquadOrders.walkOn)
pub fn squad_orders_walk_on(g: &mut crate::game::Game, men: &[GuardId]) { todo!() }

/// (src/game/orders.ts:2477 SquadOrders.asked)
pub fn squad_orders_asked(g: &mut crate::game::Game, gd: GuardId, to: V3) -> crate::orders::SquadOrdersAsked { todo!() }

/// (src/game/orders.ts:2489 SquadOrders.askAside)
pub fn squad_orders_ask_aside<T: Clone + core::fmt::Debug>(g: &mut crate::game::Game, own: &mut crate::orders::SquadOrdersAskAsideOwn, fn_: &mut dyn FnMut(&mut crate::game::Game) -> T) -> T { todo!() }

/// (src/game/orders.ts:2494 SquadOrders.hasLine)
pub fn squad_orders_has_line(g: &mut crate::game::Game, gd: GuardId, t: &crate::orders::OrderTarget) -> bool { todo!() }

/// (src/game/orders.ts:2504 SquadOrders.fileCross)
pub fn squad_orders_file_cross(g: &mut crate::game::Game, men: &[GuardId], spots: &[V3]) -> Option<crate::orders::FileCross> { todo!() }

/// (src/game/orders.ts:2527 SquadOrders.pair)
pub fn squad_orders_pair(g: &mut crate::game::Game, men: &[GuardId], spots: &[V3], groups: Option<&[f64]>, front: Option<V3>) -> Vec<(GuardId, V3)> { todo!() }

/// (src/game/orders.ts:2551 SquadOrders.statusOf)
pub fn squad_orders_status_of(g: &mut crate::game::Game, gd: GuardId) -> String { todo!() }

/// (src/game/orders.ts:2562 SquadOrders.whoTag)
pub fn squad_orders_who_tag(g: &mut crate::game::Game) -> crate::orders::SquadOrdersWhoTag { todo!() }

/// (src/game/orders.ts:2570 SquadOrders.whoStrip)
pub fn squad_orders_who_strip(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/orders.ts:2577 SquadOrders.hoverWords)
pub fn squad_orders_hover_words(g: &mut crate::game::Game, t: Option<&crate::orders::SquadOrdersHoverWordsT>) -> String { todo!() }

/// (src/game/orders.ts:2583 SquadOrders.hud)
pub fn squad_orders_hud(g: &mut crate::game::Game, kept: Option<&mut crate::orders::HoverKept>) -> crate::orders::SquadOrdersHud { todo!() }

/// (src/game/orders.ts:2619 SquadOrders.mark)
pub fn squad_orders_mark(g: &mut crate::game::Game) -> Option<crate::orders::SquadOrdersMark> { todo!() }

/// (src/game/orders.ts:2626 SquadOrders.summary)
pub fn squad_orders_summary(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/orders.ts:2643 propWay)
pub fn prop_way(g: &mut crate::game::Game, from: V3, plain: &[V3], round: Option<&[V3]>) -> Vec<V3> { todo!() }

/// (src/game/orders.ts:2652 propPenalty)
pub fn prop_penalty(g: &mut crate::game::Game, gd: GuardId, pen: Option<&crate::lanes::LanePenalty>) -> Option<crate::orders::PropPenalty> { todo!() }

/// (src/game/orders.ts:2665 farOff)
pub fn far_off(a: &[V3], b: &[V3]) -> crate::orders::FarOff { todo!() }

/// (src/game/orders.ts:2683 offLine)
pub fn off_line(g: &mut crate::game::Game, d: DoorId, p: V3) -> f64 { todo!() }

/// (src/game/orders.ts:2685 sideOf)
pub fn side_of(g: &mut crate::game::Game, d: DoorId, p: V3) -> f64 { todo!() }

/// (src/game/orders.ts:2688 doorWords)
pub fn door_words(g: &mut crate::game::Game, d: DoorId) -> String { todo!() }

/// (src/game/orders.ts:2689 doorIdWords)
pub fn door_id_words(g: &mut crate::game::Game, id_2: &str) -> String { todo!() }
