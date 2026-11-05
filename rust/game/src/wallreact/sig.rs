#![allow(unused_variables)]
//! Every function and method of `wallreact`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/wallreact.ts:139 wrap)
pub fn wrap(a: f64) -> f64 { todo!() }

/// (src/game/wallreact.ts:140 note)
pub fn note(g: &mut crate::game::Game, s: &str) { todo!() }

/// (src/game/wallreact.ts:142 segRect)
pub fn seg_rect(p: V3, q: V3, r: [f64; 4], pad: f64) -> bool { todo!() }

/// (src/game/wallreact.ts:151 rectDist)
pub fn rect_dist(p: V3, r: [f64; 4]) -> f64 { todo!() }

/// (src/game/wallreact.ts:165 WallReact.constructor)
pub fn wall_react_new(g: &mut crate::game::Game) -> crate::wallreact::WallReact { todo!() }

/// (src/game/wallreact.ts:169 WallReact.live)
pub fn wall_react_live(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/wallreact.ts:170 WallReact.fight)
pub fn wall_react_fight(g: &mut crate::game::Game) { todo!() }

/// (src/game/wallreact.ts:171 WallReact.defOf)
pub fn wall_react_def_of(g: &mut crate::game::Game, gd: GuardId) -> Option<RoomfightDefId> { todo!() }

/// (src/game/wallreact.ts:174 WallReact.reset)
pub fn wall_react_reset(g: &mut crate::game::Game) { todo!() }

/// (src/game/wallreact.ts:176 WallReact.tick)
pub fn wall_react_tick(g: &mut crate::game::Game) { todo!() }

/// (src/game/wallreact.ts:196 WallReact.take)
pub fn wall_react_take(g: &mut crate::game::Game, row: &crate::wallbreak::LevelChange) { todo!() }

/// (src/game/wallreact.ts:215 WallReact.kept)
pub fn wall_react_kept(g: &mut crate::game::Game, at: V3, list: &mut Vec<crate::wallreact::Gone>) -> Vec<crate::wallreact::Gone> { todo!() }

/// (src/game/wallreact.ts:222 WallReact.sweep)
pub fn wall_react_sweep(g: &mut crate::game::Game) { todo!() }

/// (src/game/wallreact.ts:238 WallReact.upsert)
pub fn wall_react_upsert(g: &mut crate::game::Game, row: &crate::wallbreak::LevelChange) -> Option<WallreactBreachId> { todo!() }

/// (src/game/wallreact.ts:269 WallReact.unflood)
pub fn wall_react_unflood(g: &mut crate::game::Game) { todo!() }

/// (src/game/wallreact.ts:276 WallReact.reflood)
pub fn wall_react_reflood(g: &mut crate::game::Game, opened: &[f64]) { todo!() }

/// (src/game/wallreact.ts:311 WallReact.head)
pub fn wall_react_head(g: &mut crate::game::Game, gd: crate::wallreact::WallReactHeadGd) -> V3 { todo!() }

/// (src/game/wallreact.ts:312 WallReact.faceOf)
pub fn wall_react_face_of(g: &mut crate::game::Game, b: WallreactBreachId, from: V3, y: Option<f64>) -> V3 { todo!() }

/// (src/game/wallreact.ts:313 WallReact.sees)
pub fn wall_react_sees(g: &mut crate::game::Game, b: WallreactBreachId, from: V3, yaw: Option<f64>) -> bool { todo!() }

/// (src/game/wallreact.ts:320 WallReact.opsKnow)
pub fn wall_react_ops_know(g: &mut crate::game::Game, b: WallreactBreachId, by: &str) { todo!() }

/// (src/game/wallreact.ts:321 WallReact.manKnows)
pub fn wall_react_man_knows(g: &mut crate::game::Game, b: WallreactBreachId, gd: GuardId, how: crate::wallreact::WallReactManKnowsHow) { todo!() }

/// (src/game/wallreact.ts:337 WallReact.heard)
pub fn wall_react_heard(g: &mut crate::game::Game, b: WallreactBreachId, row: &crate::wallbreak::LevelChange) { todo!() }

/// (src/game/wallreact.ts:349 WallReact.notice)
pub fn wall_react_notice(g: &mut crate::game::Game) { todo!() }

/// (src/game/wallreact.ts:363 WallReact.dropOpening)
pub fn wall_react_drop_opening(g: &mut crate::game::Game, o: CoverOpeningId) { todo!() }

/// (src/game/wallreact.ts:364 WallReact.dropOpenings)
pub fn wall_react_drop_openings(g: &mut crate::game::Game) { todo!() }

/// (src/game/wallreact.ts:366 WallReact.book)
pub fn wall_react_book(g: &mut crate::game::Game) { todo!() }

/// (src/game/wallreact.ts:378 WallReact.evidence)
pub fn wall_react_evidence(g: &mut crate::game::Game, dt: f64) { todo!() }

/// (src/game/wallreact.ts:392 WallReact.through)
pub fn wall_react_through(g: &mut crate::game::Game, b: WallreactBreachId, p: V3, q: V3) -> bool { todo!() }

/// (src/game/wallreact.ts:399 WallReact.look)
pub fn wall_react_look(g: &mut crate::game::Game, gd: GuardId, pt: &mut V3) { todo!() }

/// (src/game/wallreact.ts:418 WallReact.posts)
pub fn wall_react_posts(g: &mut crate::game::Game) -> Vec<crate::wallreact::WallReactPostsItem> { todo!() }

/// (src/game/wallreact.ts:423 WallReact.charOf)
pub fn wall_react_char_of(g: &mut crate::game::Game, id_2: f64) -> Option<crate::wallreact::WallReactCharOf> { todo!() }

/// (src/game/wallreact.ts:429 WallReact.threatFor)
pub fn wall_react_threat_for(g: &mut crate::game::Game, gd: GuardId, x: crate::wallreact::Gone, at: V3) -> Option<V3> { todo!() }

/// (src/game/wallreact.ts:441 WallReact.things)
pub fn wall_react_things(g: &mut crate::game::Game, p: &crate::wallreact::Pend) -> Vec<crate::wallreact::Gone> { todo!() }

/// (src/game/wallreact.ts:444 WallReact.asks)
pub fn wall_react_asks(g: &mut crate::game::Game, gd: GuardId, p: &crate::wallreact::Pend, at: V3, h: f64) -> crate::wallreact::WallReactAsks { todo!() }

/// (src/game/wallreact.ts:454 WallReact.beyond)
pub fn wall_react_beyond(g: &mut crate::game::Game, gd: GuardId, b: WallreactBreachId, at: V3) -> Option<V3> { todo!() }

/// (src/game/wallreact.ts:465 WallReact.openGone)
pub fn wall_react_open_gone(g: &mut crate::game::Game, threat: V3, at: V3, h: f64, x: crate::wallreact::Gone) -> bool { todo!() }

/// (src/game/wallreact.ts:469 WallReact.stepOut)
pub fn wall_react_step_out(g: &mut crate::game::Game, gd: GuardId, from: V3, threat: V3, h: f64, b: Option<WallreactBreachId>, taken: &[V3], loosely: Option<bool>) -> Option<V3> { todo!() }

/// (src/game/wallreact.ts:500 WallReact.openThrough)
pub fn wall_react_open_through(g: &mut crate::game::Game, b: WallreactBreachId, p: V3, h: f64) -> bool { todo!() }

/// (src/game/wallreact.ts:506 WallReact.known)
pub fn wall_react_known(g: &mut crate::game::Game, gd: GuardId) -> Vec<WallreactBreachId> { todo!() }

/// (src/game/wallreact.ts:509 WallReact.spot)
pub fn wall_react_spot(g: &mut crate::game::Game, gd: GuardId, threat: V3, b: Option<WallreactBreachId>, others: &[V3]) -> Option<V3> { todo!() }

/// (src/game/wallreact.ts:520 WallReact.runFor)
pub fn wall_react_run_for(g: &mut crate::game::Game, gd: GuardId, from: V3, threat: V3, taken: &[V3], kn: &[WallreactBreachId]) -> Option<V3> { todo!() }

/// (src/game/wallreact.ts:536 WallReact.reconsider)
pub fn wall_react_reconsider(g: &mut crate::game::Game, p: &mut crate::wallreact::Pend) -> crate::wallreact::WallReactReconsider { todo!() }

/// (src/game/wallreact.ts:539 WallReact.reconsider1)
pub fn wall_react_reconsider1(g: &mut crate::game::Game, p: &mut crate::wallreact::Pend) -> (crate::wallreact::WallReactReconsider10, String) { todo!() }

/// (src/game/wallreact.ts:565 WallReact.shift)
pub fn wall_react_shift(g: &mut crate::game::Game, gd: GuardId, threat: V3, b: Option<WallreactBreachId>, others: &[V3]) -> crate::wallreact::WallReactShift { todo!() }

/// (src/game/wallreact.ts:576 WallReact.opMove)
pub fn wall_react_op_move(g: &mut crate::game::Game, p: &mut crate::wallreact::Pend) -> crate::wallreact::WallReactOpMove { todo!() }

/// (src/game/wallreact.ts:594 WallReact.replan)
pub fn wall_react_replan(g: &mut crate::game::Game, team: crate::teams::Team, since: f64, men: &[GuardId]) -> bool { todo!() }

/// (src/game/wallreact.ts:599 WallReact.summary)
pub fn wall_react_summary(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/wallreact.ts:606 wallReactOf)
pub fn wall_react_of(g: &mut crate::game::Game) { todo!() }

/// (src/game/wallreact.ts:608 wallReactTick)
pub fn wall_react_tick_2(g: &mut crate::game::Game) { todo!() }

/// (src/game/wallreact.ts:610 wallReactReset)
pub fn wall_react_reset_2(g: &mut crate::game::Game) { todo!() }

/// (src/game/wallreact.ts:612 wallReactLook)
pub fn wall_react_look_2(g: &mut crate::game::Game, gd: GuardId, pt: &mut V3) { todo!() }

/// (src/game/wallreact.ts:614 wallReactReplan)
pub fn wall_react_replan_2(g: &mut crate::game::Game, team: crate::teams::Team, since: f64, men: &[GuardId]) -> bool { todo!() }
