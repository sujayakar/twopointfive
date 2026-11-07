#![allow(unused_variables)]
//! Every function and method of `wallreact`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/wallreact.ts:138 wrap)
pub fn wrap(a: f64) -> f64 { todo!() }

/// (src/game/wallreact.ts:139 note)
pub fn note(g: &mut crate::game::Game, s: &str) { todo!() }

/// (src/game/wallreact.ts:141 segRect)
pub fn seg_rect(p: V3, q: V3, r: [f64; 4], pad: f64) -> bool { todo!() }

/// (src/game/wallreact.ts:150 rectDist)
pub fn rect_dist(p: V3, r: [f64; 4]) -> f64 { todo!() }

/// (src/game/wallreact.ts:164 WallReact.constructor)
pub fn wall_react_new(g: &mut crate::game::Game) -> crate::wallreact::WallReact { todo!() }

/// (src/game/wallreact.ts:168 WallReact.live)
pub fn wall_react_live(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/wallreact.ts:169 WallReact.fight)
pub fn wall_react_fight(g: &mut crate::game::Game) { todo!() }

/// (src/game/wallreact.ts:170 WallReact.defOf)
pub fn wall_react_def_of(g: &mut crate::game::Game, gd: GuardId) -> Option<RoomfightDefId> { todo!() }

/// (src/game/wallreact.ts:173 WallReact.reset)
pub fn wall_react_reset(g: &mut crate::game::Game) { todo!() }

/// (src/game/wallreact.ts:175 WallReact.tick)
pub fn wall_react_tick(g: &mut crate::game::Game) { todo!() }

/// (src/game/wallreact.ts:195 WallReact.take)
pub fn wall_react_take(g: &mut crate::game::Game, row: &crate::wallbreak::LevelChange) { todo!() }

/// (src/game/wallreact.ts:214 WallReact.kept)
pub fn wall_react_kept(g: &mut crate::game::Game, at: V3, list: &mut Vec<crate::wallreact::Gone>) -> Vec<crate::wallreact::Gone> { todo!() }

/// (src/game/wallreact.ts:221 WallReact.sweep)
pub fn wall_react_sweep(g: &mut crate::game::Game) { todo!() }

/// (src/game/wallreact.ts:237 WallReact.upsert)
pub fn wall_react_upsert(g: &mut crate::game::Game, row: &crate::wallbreak::LevelChange) -> Option<WallreactBreachId> { todo!() }

/// (src/game/wallreact.ts:268 WallReact.unflood)
pub fn wall_react_unflood(g: &mut crate::game::Game) { todo!() }

/// (src/game/wallreact.ts:275 WallReact.reflood)
pub fn wall_react_reflood(g: &mut crate::game::Game, opened: &[f64]) { todo!() }

/// (src/game/wallreact.ts:310 WallReact.head)
pub fn wall_react_head(g: &mut crate::game::Game, gd: crate::wallreact::WallReactHeadGd) -> V3 { todo!() }

/// (src/game/wallreact.ts:311 WallReact.faceOf)
pub fn wall_react_face_of(g: &mut crate::game::Game, b: WallreactBreachId, from: V3, y: Option<f64>) -> V3 { todo!() }

/// (src/game/wallreact.ts:312 WallReact.sees)
pub fn wall_react_sees(g: &mut crate::game::Game, b: WallreactBreachId, from: V3, yaw: Option<f64>) -> bool { todo!() }

/// (src/game/wallreact.ts:319 WallReact.opsKnow)
pub fn wall_react_ops_know(g: &mut crate::game::Game, b: WallreactBreachId, by: &str) { todo!() }

/// (src/game/wallreact.ts:320 WallReact.manKnows)
pub fn wall_react_man_knows(g: &mut crate::game::Game, b: WallreactBreachId, gd: GuardId, how: crate::wallreact::WallReactManKnowsHow) { todo!() }

/// (src/game/wallreact.ts:336 WallReact.heard)
pub fn wall_react_heard(g: &mut crate::game::Game, b: WallreactBreachId, row: &crate::wallbreak::LevelChange) { todo!() }

/// (src/game/wallreact.ts:348 WallReact.notice)
pub fn wall_react_notice(g: &mut crate::game::Game) { todo!() }

/// (src/game/wallreact.ts:362 WallReact.dropOpening)
pub fn wall_react_drop_opening(g: &mut crate::game::Game, o: CoverOpeningId) { todo!() }

/// (src/game/wallreact.ts:363 WallReact.dropOpenings)
pub fn wall_react_drop_openings(g: &mut crate::game::Game) { todo!() }

/// (src/game/wallreact.ts:365 WallReact.book)
pub fn wall_react_book(g: &mut crate::game::Game) { todo!() }

/// (src/game/wallreact.ts:377 WallReact.evidence)
pub fn wall_react_evidence(g: &mut crate::game::Game, dt: f64) { todo!() }

/// (src/game/wallreact.ts:391 WallReact.through)
pub fn wall_react_through(g: &mut crate::game::Game, b: WallreactBreachId, p: V3, q: V3) -> bool { todo!() }

/// (src/game/wallreact.ts:398 WallReact.look)
pub fn wall_react_look(g: &mut crate::game::Game, gd: GuardId, pt: &mut V3) { todo!() }

/// (src/game/wallreact.ts:417 WallReact.posts)
pub fn wall_react_posts(g: &mut crate::game::Game) -> Vec<crate::wallreact::WallReactPostsItem> { todo!() }

/// (src/game/wallreact.ts:422 WallReact.charOf)
pub fn wall_react_char_of(g: &mut crate::game::Game, id_2: f64) -> Option<crate::wallreact::WallReactCharOf> { todo!() }

/// (src/game/wallreact.ts:428 WallReact.threatFor)
pub fn wall_react_threat_for(g: &mut crate::game::Game, gd: GuardId, x: crate::wallreact::Gone, at: V3) -> Option<V3> { todo!() }

/// (src/game/wallreact.ts:440 WallReact.things)
pub fn wall_react_things(g: &mut crate::game::Game, p: &crate::wallreact::Pend) -> Vec<crate::wallreact::Gone> { todo!() }

/// (src/game/wallreact.ts:443 WallReact.asks)
pub fn wall_react_asks(g: &mut crate::game::Game, gd: GuardId, p: &crate::wallreact::Pend, at: V3, h: f64) -> crate::wallreact::WallReactAsks { todo!() }

/// (src/game/wallreact.ts:453 WallReact.beyond)
pub fn wall_react_beyond(g: &mut crate::game::Game, gd: GuardId, b: WallreactBreachId, at: V3) -> Option<V3> { todo!() }

/// (src/game/wallreact.ts:464 WallReact.openGone)
pub fn wall_react_open_gone(g: &mut crate::game::Game, threat: V3, at: V3, h: f64, x: crate::wallreact::Gone) -> bool { todo!() }

/// (src/game/wallreact.ts:468 WallReact.stepOut)
pub fn wall_react_step_out(g: &mut crate::game::Game, gd: GuardId, from: V3, threat: V3, h: f64, b: Option<WallreactBreachId>, taken: &[V3], loosely: Option<bool>) -> Option<V3> { todo!() }

/// (src/game/wallreact.ts:499 WallReact.openThrough)
pub fn wall_react_open_through(g: &mut crate::game::Game, b: WallreactBreachId, p: V3, h: f64) -> bool { todo!() }

/// (src/game/wallreact.ts:505 WallReact.known)
pub fn wall_react_known(g: &mut crate::game::Game, gd: GuardId) -> Vec<WallreactBreachId> { todo!() }

/// (src/game/wallreact.ts:508 WallReact.spot)
pub fn wall_react_spot(g: &mut crate::game::Game, gd: GuardId, threat: V3, b: Option<WallreactBreachId>, others: &[V3]) -> Option<V3> { todo!() }

/// (src/game/wallreact.ts:519 WallReact.runFor)
pub fn wall_react_run_for(g: &mut crate::game::Game, gd: GuardId, from: V3, threat: V3, taken: &[V3], kn: &[WallreactBreachId]) -> Option<V3> { todo!() }

/// (src/game/wallreact.ts:535 WallReact.reconsider)
pub fn wall_react_reconsider(g: &mut crate::game::Game, p: &mut crate::wallreact::Pend) -> crate::wallreact::WallReactReconsider { todo!() }

/// (src/game/wallreact.ts:538 WallReact.reconsider1)
pub fn wall_react_reconsider1(g: &mut crate::game::Game, p: &mut crate::wallreact::Pend) -> (crate::wallreact::WallReactReconsider10, String) { todo!() }

/// (src/game/wallreact.ts:564 WallReact.shift)
pub fn wall_react_shift(g: &mut crate::game::Game, gd: GuardId, threat: V3, b: Option<WallreactBreachId>, others: &[V3]) -> crate::wallreact::WallReactShift { todo!() }

/// (src/game/wallreact.ts:575 WallReact.opMove)
pub fn wall_react_op_move(g: &mut crate::game::Game, p: &mut crate::wallreact::Pend) -> crate::wallreact::WallReactOpMove { todo!() }

/// (src/game/wallreact.ts:593 WallReact.replan)
pub fn wall_react_replan(g: &mut crate::game::Game, team: crate::teams::Team, since: f64, men: &[GuardId]) -> bool { todo!() }

/// (src/game/wallreact.ts:598 WallReact.summary)
pub fn wall_react_summary(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/wallreact.ts:605 wallReactOf)
pub fn wall_react_of(g: &mut crate::game::Game) { todo!() }

/// (src/game/wallreact.ts:607 wallReactTick)
pub fn wall_react_tick_2(g: &mut crate::game::Game) { todo!() }

/// (src/game/wallreact.ts:609 wallReactReset)
pub fn wall_react_reset_2(g: &mut crate::game::Game) { todo!() }

/// (src/game/wallreact.ts:611 wallReactLook)
pub fn wall_react_look_2(g: &mut crate::game::Game, gd: GuardId, pt: &mut V3) { todo!() }

/// (src/game/wallreact.ts:613 wallReactReplan)
pub fn wall_react_replan_2(g: &mut crate::game::Game, team: crate::teams::Team, since: f64, men: &[GuardId]) -> bool { todo!() }
