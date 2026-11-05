#![allow(unused_variables)]
//! Every function and method of `workthings`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/workthings.ts:95 flat)
pub fn flat(a: V3, b: V3) -> f64 { todo!() }

/// (src/game/workthings.ts:101 standFor)
pub fn stand_for(g: &mut crate::game::Game, face: V3, n: Option<V3>, area: Option<crate::workthings::Area>) -> Option<V3> { todo!() }

/// (src/game/workthings.ts:115 panelFace)
pub fn panel_face(g: &mut crate::game::Game) -> crate::workthings::PanelFace { todo!() }

/// (src/game/workthings.ts:126 samPanelPoint)
pub fn sam_panel_point(g: &mut crate::game::Game, from: V3) -> Option<V3> { todo!() }

/// (src/game/workthings.ts:141 breakerErrand)
pub fn breaker_errand(g: &mut crate::game::Game) -> Option<crate::workthings::BreakerErrand> { todo!() }

/// (src/game/workthings.ts:165 WorkThings.askAside)
pub fn work_things_ask_aside<T: Clone + core::fmt::Debug>(g: &mut crate::game::Game, own: &mut crate::workthings::WorkThingsAskAsideOwn, fn_: &mut dyn FnMut(&mut crate::game::Game) -> T) -> T { todo!() }

/// (src/game/workthings.ts:171 WorkThings.constructor)
pub fn work_things_new(g: &mut crate::game::Game) -> crate::workthings::WorkThings { todo!() }

/// (src/game/workthings.ts:184 WorkThings.things)
pub fn work_things_things(g: &mut crate::game::Game) -> Vec<crate::workthings::WorkThing> { todo!() }

/// (src/game/workthings.ts:203 WorkThings.prune)
pub fn work_things_prune(g: &mut crate::game::Game) { todo!() }

/// (src/game/workthings.ts:209 WorkThings.verbOf)
pub fn work_things_verb_of(g: &mut crate::game::Game, th: &crate::workthings::WorkThing) -> crate::workthings::WorkVerb { todo!() }

/// (src/game/workthings.ts:211 WorkThings.words)
pub fn work_things_words(g: &mut crate::game::Game, th: &crate::workthings::WorkThing) -> String { todo!() }

/// (src/game/workthings.ts:217 WorkThings.moot)
pub fn work_things_moot(g: &mut crate::game::Game, th: &crate::workthings::WorkThing, verb: crate::workthings::WorkVerb) -> Option<String> { todo!() }

/// (src/game/workthings.ts:227 WorkThings.near)
pub fn work_things_near(g: &mut crate::game::Game, at: V3) -> Option<crate::workthings::WorkThingsNear> { todo!() }

/// (src/game/workthings.ts:238 WorkThings.on)
pub fn work_things_on(g: &mut crate::game::Game, box_: Option<BoxId>) -> Option<crate::workthings::WorkThingsOn> { todo!() }

/// (src/game/workthings.ts:244 WorkThings.rows)
pub fn work_things_rows(g: &mut crate::game::Game, t: &crate::orders::OrderTarget, page: Option<&str>, rows: &[OrdersOrderDefId]) -> Vec<OrdersOrderDefId> { todo!() }

/// (src/game/workthings.ts:251 WorkThings.coverPost)
pub fn work_things_cover_post(g: &mut crate::game::Game, mate: GuardId, th: &crate::workthings::WorkThing) -> Option<V3> { todo!() }

/// (src/game/workthings.ts:262 WorkThings.pick)
pub fn work_things_pick(g: &mut crate::game::Game, men: &[GuardId], t: &crate::orders::OrderTarget) -> Vec<GuardId> { todo!() }

/// (src/game/workthings.ts:268 WorkThings.issue)
pub fn work_things_issue(g: &mut crate::game::Game, men: &[GuardId], t: &crate::orders::OrderTarget) -> crate::workthings::WorkThingsIssue { todo!() }

/// (src/game/workthings.ts:283 WorkThings.tick)
pub fn work_things_tick(g: &mut crate::game::Game, gd: GuardId, a: OrdersAssignmentId) { todo!() }

/// (src/game/workthings.ts:320 WorkThings.handOff)
pub fn work_things_hand_off(g: &mut crate::game::Game, gd: GuardId, j: &mut crate::workthings::Job) { todo!() }

/// (src/game/workthings.ts:322 WorkThings.settle)
pub fn work_things_settle(g: &mut crate::game::Game, gd: GuardId, j: &mut crate::workthings::Job, line: Option<&str>, turn: bool) { todo!() }

/// (src/game/workthings.ts:328 WorkThings.hand)
pub fn work_things_hand(g: &mut crate::game::Game, c_2: CharId) -> Option<crate::character::DoorPush> { todo!() }

/// (src/game/workthings.ts:335 WorkThings.needOf)
pub fn work_things_need_of(g: &mut crate::game::Game, verb: crate::workthings::WorkVerb) -> f64 { todo!() }

/// (src/game/workthings.ts:338 WorkThings.job)
pub fn work_things_job(g: &mut crate::game::Game, c_2: CharId) -> Option<crate::workthings::WorkJob> { todo!() }

/// (src/game/workthings.ts:340 WorkThings.atThing)
pub fn work_things_at_thing(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/workthings.ts:342 WorkThings.working)
pub fn work_things_working(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/workthings.ts:346 WorkThings.onDevice)
pub fn work_things_on_device(g: &mut crate::game::Game, i: f64) -> Option<crate::workthings::WorkThingsOnDevice> { todo!() }

/// (src/game/workthings.ts:352 WorkThings.note)
pub fn work_things_note(g: &mut crate::game::Game, gd: GuardId, what: &str) { todo!() }

/// (src/game/workthings.ts:353 WorkThings.summary)
pub fn work_things_summary(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/workthings.ts:357 workNear)
pub fn work_near(g: &mut crate::game::Game, at: V3) -> Option<crate::workthings::WorkNear> { todo!() }

/// (src/game/workthings.ts:359 workOn)
pub fn work_on(g: &mut crate::game::Game, box_: Option<BoxId>) -> Option<crate::workthings::WorkOn> { todo!() }

/// (src/game/workthings.ts:361 workRows)
pub fn work_rows(g: &mut crate::game::Game, t: &crate::orders::OrderTarget, page: Option<&str>, rows: &[OrdersOrderDefId]) -> Vec<OrdersOrderDefId> { todo!() }

/// (src/game/workthings.ts:363 workHand)
pub fn work_hand(g: &mut crate::game::Game, c_2: CharId) -> Option<crate::character::DoorPush> { todo!() }

/// (src/game/workthings.ts:373 workPasses)
pub fn work_passes(g: &mut crate::game::Game, gd: GuardId, h: GuardId) -> bool { todo!() }

/// (src/game/workthings.ts:387 workComing)
pub fn work_coming(g: &mut crate::game::Game, gd: GuardId, h: GuardId) -> bool { todo!() }

/// (src/game/workthings.ts:400 workJob)
pub fn work_job(g: &mut crate::game::Game, c_2: CharId) -> Option<crate::workthings::WorkJob> { todo!() }

/// (src/game/workthings.ts:402 workNotHis)
pub fn work_not_his(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/workthings.ts:406 workStaysPut)
pub fn work_stays_put(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }
