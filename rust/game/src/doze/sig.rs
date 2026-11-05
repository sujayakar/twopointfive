#![allow(unused_variables)]
//! Every function and method of `doze`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/doze.ts:88 pulls)
pub fn pulls(g: &mut crate::game::Game) -> Vec<[f64; 2]> { todo!() }

/// (src/game/doze.ts:94 mapOf)
pub fn map_of(g: &mut crate::game::Game) -> JsMap<GuardId, crate::doze::Doze> { todo!() }

/// (src/game/doze.ts:99 flat)
pub fn flat(a: &[f64], x: f64, z: f64) -> f64 { todo!() }

/// (src/game/doze.ts:101 chairFace)
pub fn chair_face(g: &mut crate::game::Game, p: PropId) -> f64 { todo!() }

/// (src/game/doze.ts:103 live)
pub fn live(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/doze.ts:107 calm)
pub fn calm(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/doze.ts:110 put)
pub fn put(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/doze.ts:111 drop)
pub fn drop(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/doze.ts:113 dark)
pub fn dark(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/doze.ts:118 leftLit)
pub fn left_lit(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/doze.ts:119 leftOff)
pub fn left_off(g: &mut crate::game::Game) { todo!() }

/// (src/game/doze.ts:120 leftTick)
pub fn left_tick(g: &mut crate::game::Game) { todo!() }

/// (src/game/doze.ts:122 blank)
pub fn blank(phase: crate::doze::Phase, wp: f64, t: f64) -> crate::doze::Doze { todo!() }

/// (src/game/doze.ts:125 walkLen)
pub fn walk_len(g: &mut crate::game::Game, from: &[f64], to: V3) -> f64 { todo!() }

/// (src/game/doze.ts:126 seated)
pub fn seated(d: crate::doze::Doze) -> bool { todo!() }

/// (src/game/doze.ts:127 span)
pub fn span(g: &mut crate::game::Game, r: &[f64]) -> f64 { todo!() }

/// (src/game/doze.ts:143 dozeLines)
pub fn doze_lines() -> Vec<String> { todo!() }

/// (src/game/doze.ts:147 screenSeat)
pub fn screen_seat(g: &mut crate::game::Game, gd: GuardId, p: PropId) -> Option<SeatId> { todo!() }

/// (src/game/doze.ts:170 findChair)
pub fn find_chair(g: &mut crate::game::Game, gd: GuardId) -> Option<SeatId> { todo!() }

/// (src/game/doze.ts:194 dozePatrol)
pub fn doze_patrol(g: &mut crate::game::Game, gd: GuardId, dt: f64, walk: f64) -> bool { todo!() }

/// (src/game/doze.ts:222 up)
pub fn up(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/doze.ts:226 wake)
pub fn wake(g: &mut crate::game::Game, gd: GuardId, why: crate::doze::WakeWhy, at: Option<&[f64]>) { todo!() }

/// (src/game/doze.ts:239 dozeDeaf)
pub fn doze_deaf(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/doze.ts:251 dozeTick)
pub fn doze_tick(g: &mut crate::game::Game, dt: f64) { todo!() }

/// (src/game/doze.ts:304 faceLight)
pub fn face_light(g: &mut crate::game::Game, gd: GuardId, head: V3) -> f64 { todo!() }

/// (src/game/doze.ts:311 lampOn)
pub fn lamp_on(g: &mut crate::game::Game, gd: GuardId, head: V3) -> V3 { todo!() }

/// (src/game/doze.ts:322 dozeInChair)
pub fn doze_in_chair(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/doze.ts:327 dozeAsleep)
pub fn doze_asleep(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/doze.ts:329 dozeDark)
pub fn doze_dark(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/doze.ts:331 dozeAt)
pub fn doze_at(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::doze::DozeAt> { todo!() }

/// (src/game/doze.ts:333 dozeEar)
pub fn doze_ear(g: &mut crate::game::Game, gd: GuardId) -> [f64; 2] { todo!() }

/// (src/game/doze.ts:337 dozeWoke)
pub fn doze_woke(g: &mut crate::game::Game, gd: GuardId, secs: Option<f64>) -> Option<crate::doze::DozeWhy> { todo!() }

/// (src/game/doze.ts:339 dozeBreath)
pub fn doze_breath(g: &mut crate::game::Game, gd: GuardId) -> f64 { todo!() }

/// (src/game/doze.ts:341 dozeMark)
pub fn doze_mark(g: &mut crate::game::Game, gd: GuardId) -> String { todo!() }

/// (src/game/doze.ts:344 dozeOf)
pub fn doze_of(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::doze::DozeOf> { todo!() }

/// (src/game/doze.ts:346 dozeSummary)
pub fn doze_summary(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/doze.ts:352 dozeReset)
pub fn doze_reset(g: &mut crate::game::Game, load: Option<bool>) { todo!() }
