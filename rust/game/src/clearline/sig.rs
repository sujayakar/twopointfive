#![allow(unused_variables)]
//! Every function and method of `clearline`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/clearline.ts:117 saveClearLine)
pub fn save_clear_line(g: &mut crate::game::Game) { todo!() }

/// (src/game/clearline.ts:132 memOf)
pub fn mem_of<'g>(g: &'g mut crate::game::Game, gd: GuardId) -> &'g mut crate::clearline::Mem { todo!() }

/// (src/game/clearline.ts:134 liveDoing)
pub fn live_doing(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::clearline::Doing> { todo!() }

/// (src/game/clearline.ts:135 chestOf)
pub fn chest_of(g: &mut crate::game::Game, h: GuardId) -> V3 { todo!() }

/// (src/game/clearline.ts:137 aimOf)
pub fn aim_of(g: &mut crate::game::Game, d: crate::clearline::Doing) -> V3 { todo!() }

/// (src/game/clearline.ts:138 flat)
pub fn flat(a: V3, b: V3) -> f64 { todo!() }

/// (src/game/clearline.ts:140 offSeg)
pub fn off_seg(p: V3, a: V3, b: V3) -> [f64; 2] { todo!() }

/// (src/game/clearline.ts:146 samAtEdge)
pub fn sam_at_edge(g: &mut crate::game::Game, gd: GuardId, to: V3) -> bool { todo!() }

/// (src/game/clearline.ts:155 leads)
pub fn leads(g: &mut crate::game::Game, gd: GuardId, by: crate::clearline::LeadsBy, tgt: GuardId, to: V3, secs: f64) -> bool { todo!() }

/// (src/game/clearline.ts:172 samMoves)
pub fn sam_moves(g: &mut crate::game::Game, secs: f64) -> bool { todo!() }

/// (src/game/clearline.ts:188 entering)
pub fn entering(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/clearline.ts:199 bodies)
pub fn bodies(g: &mut crate::game::Game, self_: GuardId) -> crate::Opaque { todo!() }

/// (src/game/clearline.ts:206 shotFrom)
pub fn shot_from(g: &mut crate::game::Game, gd: GuardId, p: V3, to: V3) -> bool { todo!() }

/// (src/game/clearline.ts:217 masked)
pub fn masked(g: &mut crate::game::Game, gd: GuardId, p: V3, to: V3) -> bool { todo!() }

/// (src/game/clearline.ts:223 spotOk)
pub fn spot_ok(g: &mut crate::game::Game, gd: GuardId, p: V3, to: V3) -> bool { todo!() }

/// (src/game/clearline.ts:242 findSpot)
pub fn find_spot(g: &mut crate::game::Game, gd: GuardId, to: V3) -> Option<crate::clearline::FindSpot> { todo!() }

/// (src/game/clearline.ts:262 note)
pub fn note(g: &mut crate::game::Game, gd: GuardId, what: crate::clearline::NoteWhat, by: &str, at: Option<V3>, walk: Option<f64>) { todo!() }

/// (src/game/clearline.ts:265 cue)
pub fn cue(g: &mut crate::game::Game, gd: GuardId, text: &str) { todo!() }

/// (src/game/clearline.ts:271 endIt)
pub fn end_it(g: &mut crate::game::Game, gd: GuardId, rest: Option<f64>) { todo!() }

/// (src/game/clearline.ts:279 doing)
pub fn doing<'g>(g: &'g mut crate::game::Game, gd: GuardId) -> Option<&'g mut crate::clearline::Doing> { todo!() }

/// (src/game/clearline.ts:295 clearLineLive)
pub fn clear_line_live(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/clearline.ts:297 clearLineFeet)
pub fn clear_line_feet(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::clearline::ClearLineFeet> { todo!() }

/// (src/game/clearline.ts:314 coverOk)
pub fn cover_ok(g: &mut crate::game::Game, gd: GuardId, p: V3) -> bool { todo!() }

/// (src/game/clearline.ts:322 clearLineBlocked)
pub fn clear_line_blocked(g: &mut crate::game::Game, gd: GuardId, tgt: GuardId, to: V3, by: crate::clearline::ClearLineBlockedBy) -> bool { todo!() }

/// (src/game/clearline.ts:400 clearLineSummary)
pub fn clear_line_summary(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/clearline.ts:406 clearLineOf)
pub fn clear_line_of(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::clearline::ClearLineOf> { todo!() }
