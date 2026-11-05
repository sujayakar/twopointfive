#![allow(unused_variables)]
//! Every function and method of `close`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/close.ts:141 note)
pub fn note(g: &mut crate::game::Game, gd: GuardId, what: crate::close::NoteWhat, kind: Option<crate::close::CloseKind>, bearing: Option<f64>, dist: Option<f64>) { todo!() }

/// (src/game/close.ts:149 seatedOf)
pub fn seated_of(g: &mut crate::game::Game) -> JsMap<GuardId, SeatId> { todo!() }

/// (src/game/close.ts:150 stateOf)
pub fn state_of<'g>(g: &'g mut crate::game::Game) -> &'g mut crate::close::State { todo!() }

/// (src/game/close.ts:153 arc)
pub fn arc(id_2: crate::close::CloseId, kind: crate::close::CloseKind, mid: f64, half: f64, reach: f64, grab: bool, strike: bool) -> crate::close::CloseWindow { todo!() }

/// (src/game/close.ts:155 closeHasSeat)
pub fn close_has_seat(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/close.ts:157 closeSeated)
pub fn close_seated(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/close.ts:159 closeWindows)
pub fn close_windows(g: &mut crate::game::Game, gd: GuardId) -> Vec<crate::close::CloseWindow> { todo!() }

/// (src/game/close.ts:180 windowAt)
pub fn window_at(wins: &[crate::close::CloseWindow], b: f64) -> Option<crate::close::CloseWindow> { todo!() }

/// (src/game/close.ts:186 clearBetween)
pub fn clear_between(g: &mut crate::game::Game, p: &[f64], q: &[f64]) -> bool { todo!() }

/// (src/game/close.ts:193 closeSubject)
pub fn close_subject(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/close.ts:200 closeAt)
pub fn close_at(g: &mut crate::game::Game, gd: GuardId, p: &[f64]) -> crate::close::CloseOffer { todo!() }

/// (src/game/close.ts:222 closeLethal)
pub fn close_lethal(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/close.ts:228 closeRmb)
pub fn close_rmb(g: &mut crate::game::Game, on: bool, down: bool, hit: bool, dt: f64, near: bool, choke: bool, dropped: bool) -> Option<crate::close::CloseRmb> { todo!() }

/// (src/game/close.ts:241 closeBare)
pub fn close_bare(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/close.ts:243 closeLive)
pub fn close_live(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/close.ts:245 closeBlow)
pub fn close_blow(g: &mut crate::game::Game, lethal: bool) { todo!() }

/// (src/game/close.ts:246 closeKindOf)
pub fn close_kind_of(o: Option<&crate::close::CloseOffer>) -> crate::close::CloseKind { todo!() }

/// (src/game/close.ts:253 closeRows)
pub fn close_rows(g: &mut crate::game::Game, gd: GuardId, p: &[f64], busy: bool, row: &mut dyn FnMut(&mut crate::game::Game, crate::game::Interactable, crate::game::Verb, String, Option<String>, Option<crate::close::CloseRowsRowOver>) -> crate::game::Interactable, offer: &mut dyn FnMut(&mut crate::game::Game, crate::game::Interactable, Vec<crate::game::Interactable>)) { todo!() }

/// (src/game/close.ts:292 sneakHint)
pub fn sneak_hint(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/close.ts:294 noteBest)
pub fn note_best(g: &mut crate::game::Game, o: &crate::close::CloseOffer) { todo!() }

/// (src/game/close.ts:296 closeCivGate)
pub fn close_civ_gate(g: &mut crate::game::Game, gd: GuardId, p: &[f64]) -> crate::close::CloseCivGate { todo!() }

/// (src/game/close.ts:301 closeCivOffer)
pub fn close_civ_offer(g: &mut crate::game::Game, gd: GuardId, p: &[f64]) { todo!() }

/// (src/game/close.ts:307 closeReady)
pub fn close_ready(g: &mut crate::game::Game) -> Option<crate::close::CloseReady> { todo!() }

/// (src/game/close.ts:322 closeMode)
pub fn close_mode(g: &mut crate::game::Game) -> Option<GuardId> { todo!() }

/// (src/game/close.ts:323 modeNow)
pub fn mode_now(g: &mut crate::game::Game, pl: PlayerId) -> Option<GuardId> { todo!() }

/// (src/game/close.ts:338 closeTick)
pub fn close_tick(g: &mut crate::game::Game) { todo!() }

/// (src/game/close.ts:365 closeNearK)
pub fn close_near_k(g: &mut crate::game::Game, gd: GuardId, dist: f64) -> f64 { todo!() }

/// (src/game/close.ts:375 closeStation)
pub fn close_station(g: &mut crate::game::Game, gd: GuardId, kind: crate::close::CloseKind, bearing: f64) -> crate::close::CloseStation { todo!() }

/// (src/game/close.ts:381 closeNoise)
pub fn close_noise(g: &mut crate::game::Game, kind: crate::close::CloseKind, was_alert: bool) -> f64 { todo!() }

/// (src/game/close.ts:383 closeTook)
pub fn close_took(g: &mut crate::game::Game, gd: GuardId, what: crate::close::CloseTookWhat, kind: crate::close::CloseKind, bearing: f64, dist: f64) { todo!() }

/// (src/game/close.ts:398 clipReach)
pub fn clip_reach(g: &mut crate::game::Game, gd: GuardId, a: f64, r0: f64, r1: f64) -> f64 { todo!() }

/// (src/game/close.ts:407 labelOf)
pub fn label_of(g: &mut crate::game::Game, w: crate::close::CloseWindow) -> String { todo!() }

/// (src/game/close.ts:416 closeMarks)
pub fn close_marks(g: &mut crate::game::Game) -> Vec<crate::close::CloseMark> { todo!() }

/// (src/game/close.ts:447 calm)
pub fn calm(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/close.ts:450 closeSeat)
pub fn close_seat(g: &mut crate::game::Game, gd: GuardId, at: Option<&[f64]>, face: Option<f64>) -> bool { todo!() }

/// (src/game/close.ts:458 closeUnseat)
pub fn close_unseat(g: &mut crate::game::Game, gd: GuardId, back: Option<bool>) { todo!() }

/// (src/game/close.ts:465 closeSeatTick)
pub fn close_seat_tick(g: &mut crate::game::Game, dt: f64) { todo!() }

/// (src/game/close.ts:482 closeReset)
pub fn close_reset(g: &mut crate::game::Game) { todo!() }

/// (src/game/close.ts:483 closeSummary)
pub fn close_summary(g: &mut crate::game::Game) -> String { todo!() }
