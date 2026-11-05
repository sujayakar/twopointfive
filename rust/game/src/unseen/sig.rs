#![allow(unused_variables)]
//! Every function and method of `unseen`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/unseen.ts:55 clamp)
pub fn clamp(x: f64, a: f64, b: f64) -> f64 { todo!() }

/// (src/game/unseen.ts:103 stateOf)
pub fn state_of<'g>(g: &'g mut crate::game::Game) -> &'g mut crate::unseen::State { todo!() }

/// (src/game/unseen.ts:105 lead)
pub fn lead(g: &mut crate::game::Game) -> PlayerId { todo!() }

/// (src/game/unseen.ts:106 mate)
pub fn mate(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/unseen.ts:110 onWall)
pub fn on_wall(g: &mut crate::game::Game, pl: PlayerId) -> bool { todo!() }

/// (src/game/unseen.ts:112 samSneaks)
pub fn sam_sneaks(g: &mut crate::game::Game, pl: PlayerId) -> bool { todo!() }

/// (src/game/unseen.ts:114 squadFound)
pub fn squad_found(g: &mut crate::game::Game, men: &[GuardId]) -> Option<String> { todo!() }

/// (src/game/unseen.ts:122 unseenLive)
pub fn unseen_live(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/unseen.ts:141 cue)
pub fn cue(g: &mut crate::game::Game, pl: PlayerId) { todo!() }

/// (src/game/unseen.ts:147 unseenMan)
pub fn unseen_man(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/unseen.ts:150 entering)
pub fn entering(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/unseen.ts:155 offWall)
pub fn off_wall(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/unseen.ts:156 lowNow)
pub fn low_now(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/unseen.ts:157 unseenLow)
pub fn unseen_low(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/unseen.ts:162 unseenFollow)
pub fn unseen_follow(g: &mut crate::game::Game, gd: GuardId, sam_v: f64, d: f64, walk: f64, speed0: f64) -> Option<crate::unseen::UnseenFollow> { todo!() }

/// (src/game/unseen.ts:168 unseenPost)
pub fn unseen_post(g: &mut crate::game::Game, gd: GuardId, order: &str) -> Option<f64> { todo!() }

/// (src/game/unseen.ts:172 unseenWallSlot)
pub fn unseen_wall_slot(g: &mut crate::game::Game, gd: GuardId, back: f64, file: &[GuardId]) -> Option<V3> { todo!() }

/// (src/game/unseen.ts:179 unseenWallLost)
pub fn unseen_wall_lost(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/unseen.ts:180 wallSlot)
pub fn wall_slot(g: &mut crate::game::Game, gd: GuardId, back: f64, file: &[GuardId]) -> Option<V3> { todo!() }

/// (src/game/unseen.ts:208 unseenPace)
pub fn unseen_pace(g: &mut crate::game::Game, gd: GuardId) -> Option<f64> { todo!() }

/// (src/game/unseen.ts:210 unseenStep)
pub fn unseen_step(g: &mut crate::game::Game, gd: GuardId) -> Option<f64> { todo!() }

/// (src/game/unseen.ts:212 unseenDark)
pub fn unseen_dark(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/unseen.ts:216 unseenHush)
pub fn unseen_hush(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/unseen.ts:217 unseenEase)
pub fn unseen_ease(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/unseen.ts:220 unseenHolds)
pub fn unseen_holds(g: &mut crate::game::Game, gd: GuardId, h: GuardId, noticed: f64, quiet: bool, urgent: &mut dyn FnMut(&mut crate::game::Game) -> bool) -> bool { todo!() }

/// (src/game/unseen.ts:226 unseenGlass)
pub fn unseen_glass(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/unseen.ts:228 unseenGlassHeld)
pub fn unseen_glass_held(g: &mut crate::game::Game) { todo!() }

/// (src/game/unseen.ts:232 unseenWord)
pub fn unseen_word(g: &mut crate::game::Game) -> Option<String> { todo!() }

/// (src/game/unseen.ts:233 unseenSummary)
pub fn unseen_summary(g: &mut crate::game::Game) -> String { todo!() }
