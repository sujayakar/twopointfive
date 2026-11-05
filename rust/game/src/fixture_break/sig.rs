#![allow(unused_variables)]
//! Every function and method of `fixture_break`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/fixtureBreak.ts:67 h01)
pub fn h01(x: f64) -> f64 { todo!() }

/// (src/game/fixtureBreak.ts:72 kindOf)
pub fn kind_of(g: &mut crate::game::Game, t: FixtureId) -> f64 { todo!() }

/// (src/game/fixtureBreak.ts:82 floorAt)
pub fn floor_at(g: &mut crate::game::Game, x: f64, z: f64, from: f64) -> f64 { todo!() }

/// (src/game/fixtureBreak.ts:94 restOn)
pub fn rest_on(g: &mut crate::game::Game, x: f64, z: f64, ux: f64, uz: f64, l: f64, from: f64) -> crate::fixture_break::RestOn { todo!() }

/// (src/game/fixtureBreak.ts:102 causeOf)
pub fn cause_of(g: &mut crate::game::Game, at: V3) -> Option<V3> { todo!() }

/// (src/game/fixtureBreak.ts:109 add)
pub fn add(g: &mut crate::game::Game, t: FixtureId, kind: f64, born: f64) -> crate::fixture_break::Rec { todo!() }

/// (src/game/fixtureBreak.ts:166 pose)
pub fn pose(g: &mut crate::game::Game, r: &mut crate::fixture_break::Rec) { todo!() }

/// (src/game/fixtureBreak.ts:179 land)
pub fn land(r: &mut crate::fixture_break::Rec, s: f64) { todo!() }

/// (src/game/fixtureBreak.ts:185 glowAt)
pub fn glow_at(g: &mut crate::game::Game, r: &mut crate::fixture_break::Rec, a: f64, out: &mut crate::fixture_break::GlowAtOut) -> f64 { todo!() }

/// (src/game/fixtureBreak.ts:194 seedOf)
pub fn seed_of(r: &mut crate::fixture_break::Rec) -> f64 { todo!() }

/// (src/game/fixtureBreak.ts:197 fixtureBreakBoxes)
pub fn fixture_break_boxes(g: &mut crate::game::Game, out: &mut Vec<BoxId>, dt: f64) { todo!() }
