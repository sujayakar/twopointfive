#![allow(unused_variables)]
//! Every function and method of `sneak`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/sneak.ts:62 clamp)
pub fn clamp(x: f64, a: f64, b: f64) -> f64 { todo!() }

/// (src/game/sneak.ts:147 saveSneak)
pub fn save_sneak(g: &mut crate::game::Game) { todo!() }

/// (src/game/sneak.ts:189 leafMoved)
pub fn leaf_moved(g: &mut crate::game::Game, d: crate::sneak::LeafMovedD) -> bool { todo!() }

/// (src/game/sneak.ts:194 flat)
pub fn flat(a: V3, b: V3) -> f64 { todo!() }

/// (src/game/sneak.ts:196 note)
pub fn note(g: &mut crate::game::Game, gd: GuardId, what: &str) { todo!() }

/// (src/game/sneak.ts:200 sneakLive)
pub fn sneak_live(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/sneak.ts:210 mate)
pub fn mate(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/sneak.ts:212 sneaking)
pub fn sneaking(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/sneak.ts:214 sneakLow)
pub fn sneak_low(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/sneak.ts:216 sneakPost)
pub fn sneak_post(g: &mut crate::game::Game, gd: GuardId, order: &str) -> Option<f64> { todo!() }

/// (src/game/sneak.ts:219 sneakStep)
pub fn sneak_step(g: &mut crate::game::Game, gd: GuardId) -> Option<f64> { todo!() }

/// (src/game/sneak.ts:223 sneakComing)
pub fn sneak_coming(g: &mut crate::game::Game, gd: GuardId, h: GuardId) -> bool { todo!() }

/// (src/game/sneak.ts:238 sneakWord)
pub fn sneak_word(g: &mut crate::game::Game, gd: GuardId) -> Option<String> { todo!() }

/// (src/game/sneak.ts:245 eyesFor)
pub fn eyes_for(g: &mut crate::game::Game, gd: GuardId) -> Vec<GuardId> { todo!() }

/// (src/game/sneak.ts:252 cpuLit)
pub fn cpu_lit(g: &mut crate::game::Game, a: V3, b: V3, own: Option<&mut dyn FnMut(&mut crate::game::Game, f64) -> bool>, steady: Option<bool>) -> f64 { todo!() }

/// (src/game/sneak.ts:255 litAt)
pub fn lit_at(g: &mut crate::game::Game, c_2: &mut crate::sneak::Crumb, low: bool) -> f64 { todo!() }

/// (src/game/sneak.ts:261 eyeFor)
pub fn eye_for(g: &mut crate::game::Game, h: GuardId, wide: f64) -> crate::guards::Eye { todo!() }

/// (src/game/sneak.ts:266 standIn)
pub fn stand_in(p: V3, low: bool) -> CharId { todo!() }

/// (src/game/sneak.ts:271 read)
pub fn read(g: &mut crate::game::Game, eyes: &[GuardId], pc: CharId, lit: &mut dyn FnMut(&mut crate::game::Game) -> f64, low: Option<bool>, card: Option<f64>) -> crate::sneak::Exp { todo!() }

/// (src/game/sneak.ts:283 exposure)
pub fn exposure(g: &mut crate::game::Game, eyes: &[GuardId], c_2: &mut crate::sneak::Crumb, low: bool, card: Option<f64>) -> crate::sneak::Exp { todo!() }

/// (src/game/sneak.ts:284 exposureOf)
pub fn exposure_of(g: &mut crate::game::Game, eyes: &[GuardId], gd: GuardId) -> crate::sneak::Exp { todo!() }

/// (src/game/sneak.ts:291 eyeRate)
pub fn eye_rate(g: &mut crate::game::Game, h: GuardId, lit: f64, dist: f64, keen: f64, low: Option<bool>) -> f64 { todo!() }

/// (src/game/sneak.ts:302 eyeGains)
pub fn eye_gains(g: &mut crate::game::Game, h: GuardId, lit: f64, dist: f64, keen: f64, low: bool, card: Option<f64>, pad: Option<f64>) -> bool { todo!() }

/// (src/game/sneak.ts:306 litSeen)
pub fn lit_seen(g: &mut crate::game::Game, h: GuardId, lit: f64, dist: f64, keen: f64, low: bool, card: Option<f64>, pad: Option<f64>) -> bool { todo!() }

/// (src/game/sneak.ts:310 crossCost)
pub fn cross_cost(g: &mut crate::game::Game, eyes: &[GuardId], t: &[crate::sneak::Crumb], first: f64, end: f64, card: Option<f64>) -> f64 { todo!() }

/// (src/game/sneak.ts:329 litCell)
pub fn lit_cell(g: &mut crate::game::Game, x: f64, z: f64, low: bool, age: Option<f64>) -> f64 { todo!() }

/// (src/game/sneak.ts:348 sneakWalk)
pub fn sneak_walk(g: &mut crate::game::Game, gd: GuardId, low: bool, habit: Option<&mut dyn FnMut(&mut crate::game::Game, GuardId) -> Option<[f64; 2]>>) -> crate::sneak::SneakWalk { todo!() }

/// (src/game/sneak.ts:418 track)
pub fn track(g: &mut crate::game::Game) { todo!() }

/// (src/game/sneak.ts:434 sneakTick)
pub fn sneak_tick(g: &mut crate::game::Game) -> Option<crate::sneak::State> { todo!() }

/// (src/game/sneak.ts:444 sneakFollow)
pub fn sneak_follow(g: &mut crate::game::Game, gd: GuardId, back: f64, sam_v: f64, stopped: bool, speed0: f64) -> Option<crate::sneak::Feet> { todo!() }

/// (src/game/sneak.ts:627 sneakWay)
pub fn sneak_way(g: &mut crate::game::Game, gd: GuardId, goal: V3) { todo!() }

/// (src/game/sneak.ts:633 cue)
pub fn cue(g: &mut crate::game::Game, gd: GuardId, m: &mut crate::sneak::Man, line: &str, force: Option<bool>) -> bool { todo!() }

/// (src/game/sneak.ts:639 sneakSummary)
pub fn sneak_summary(g: &mut crate::game::Game) -> String { todo!() }
