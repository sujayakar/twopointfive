#![allow(unused_variables)]
//! Every function and method of `spotted`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/spotted.ts:124 flat)
pub fn flat(a: &[f64], b: &[f64]) -> f64 { todo!() }

/// (src/game/spotted.ts:125 eyeOf)
pub fn eye_of(g: &mut crate::game::Game, gd: GuardId) -> V3 { todo!() }

/// (src/game/spotted.ts:129 inLine)
pub fn in_line(g: &mut crate::game::Game, h: GuardId, q: &[f64], low: bool) -> bool { todo!() }

/// (src/game/spotted.ts:134 litAt)
pub fn lit_at(g: &mut crate::game::Game, q: &[f64]) -> f64 { todo!() }

/// (src/game/spotted.ts:135 ours)
pub fn ours(g: &mut crate::game::Game, h: GuardId) -> bool { todo!() }

/// (src/game/spotted.ts:137 hasThem)
pub fn has_them(g: &mut crate::game::Game, h: GuardId) -> bool { todo!() }

/// (src/game/spotted.ts:138 live)
pub fn live(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/spotted.ts:142 spottedMode)
pub fn spotted_mode(g: &mut crate::game::Game, gd: GuardId) -> crate::spotted::SpottedMode { todo!() }

/// (src/game/spotted.ts:144 spottedBy)
pub fn spotted_by(g: &mut crate::game::Game, gd: GuardId) -> Option<GuardId> { todo!() }

/// (src/game/spotted.ts:146 spottedHolds)
pub fn spotted_holds(g: &mut crate::game::Game, gd: GuardId, h: GuardId) -> bool { todo!() }

/// (src/game/spotted.ts:155 dangerNow)
pub fn danger_now(g: &mut crate::game::Game, h: GuardId, gd: GuardId) -> bool { todo!() }

/// (src/game/spotted.ts:164 spottedKeen)
pub fn spotted_keen(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/spotted.ts:170 spottedFeet)
pub fn spotted_feet(g: &mut crate::game::Game, gd: GuardId, follower: bool, aside: bool) -> Option<crate::spotted::SpottedFeet> { todo!() }

/// (src/game/spotted.ts:186 set)
pub fn set(g: &mut crate::game::Game, gd: GuardId, to: crate::spotted::SpottedMode, of: Option<GuardId>) { todo!() }

/// (src/game/spotted.ts:191 cue)
pub fn cue(g: &mut crate::game::Game, gd: GuardId, text: &str, gap: Option<f64>) { todo!() }

/// (src/game/spotted.ts:196 ring)
pub fn ring(g: &mut crate::game::Game, p: V3, max_m: f64) -> crate::Opaque { todo!() }

/// (src/game/spotted.ts:206 inSamsLine)
pub fn in_sams_line(g: &mut crate::game::Game, q: V3, h: GuardId) -> bool { todo!() }

/// (src/game/spotted.ts:213 hide)
pub fn hide(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/spotted.ts:235 shotLine)
pub fn shot_line(g: &mut crate::game::Game, q: &[f64], hp: &[f64], y: f64) -> bool { todo!() }

/// (src/game/spotted.ts:241 cover)
pub fn cover(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/spotted.ts:257 onHim)
pub fn on_him(g: &mut crate::game::Game, h: GuardId, gd: GuardId) -> bool { todo!() }

/// (src/game/spotted.ts:266 hands)
pub fn hands(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/spotted.ts:296 spottedTake)
pub fn spotted_take(g: &mut crate::game::Game, gd: GuardId, foe: GuardId) -> bool { todo!() }

/// (src/game/spotted.ts:307 spottedGripped)
pub fn spotted_gripped(g: &mut crate::game::Game, h: GuardId) -> bool { todo!() }

/// (src/game/spotted.ts:313 fresh)
pub fn fresh(g: &mut crate::game::Game) -> Vec<crate::spotted::FreshItem> { todo!() }

/// (src/game/spotted.ts:323 spottedTick)
pub fn spotted_tick(g: &mut crate::game::Game) { todo!() }

/// (src/game/spotted.ts:355 spottedWord)
pub fn spotted_word(g: &mut crate::game::Game) -> Option<String> { todo!() }

/// (src/game/spotted.ts:361 spottedSummary)
pub fn spotted_summary(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/spotted.ts:368 spottedOf)
pub fn spotted_of(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::spotted::SpottedOf> { todo!() }

/// (src/game/spotted.ts:375 saveSpotted)
pub fn save_spotted(g: &mut crate::game::Game) { todo!() }
