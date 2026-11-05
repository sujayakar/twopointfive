#![allow(unused_variables)]
//! Every function and method of `fireline`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/fireline.ts:86 live)
pub fn live(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/fireline.ts:90 fireLineTick)
pub fn fire_line_tick(g: &mut crate::game::Game) { todo!() }

/// (src/game/fireline.ts:105 laneOf)
pub fn lane_of(g: &mut crate::game::Game) -> Option<crate::fireline::Lane> { todo!() }

/// (src/game/fireline.ts:113 inShape)
pub fn in_shape(g: &mut crate::game::Game, l: crate::fireline::Lane, x: f64, z: f64, more: Option<f64>) -> bool { todo!() }

/// (src/game/fireline.ts:118 inLane)
pub fn in_lane(g: &mut crate::game::Game, l: crate::fireline::Lane, x: f64, z: f64, more: Option<f64>) -> bool { todo!() }

/// (src/game/fireline.ts:124 along)
pub fn along(from: V3, pts: &[V3], far: f64, hit: &mut dyn FnMut(f64, f64) -> bool) -> f64 { todo!() }

/// (src/game/fireline.ts:135 crossAt)
pub fn cross_at(g: &mut crate::game::Game, l: crate::fireline::Lane, from: V3, pts: &[V3], far: f64, more: Option<f64>) -> f64 { todo!() }

/// (src/game/fireline.ts:137 doorAt)
pub fn door_at(g: &mut crate::game::Game, from: V3, pts: &[V3], far: f64) -> f64 { todo!() }

/// (src/game/fireline.ts:138 inDoorway)
pub fn in_doorway(g: &mut crate::game::Game, x: f64, z: f64) -> bool { todo!() }

/// (src/game/fireline.ts:145 walkOf)
pub fn walk_of(g: &mut crate::game::Game, gd: GuardId, goal: V3) -> Option<Vec<V3>> { todo!() }

/// (src/game/fireline.ts:152 inEnemyLine)
pub fn in_enemy_line(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/fireline.ts:154 wayRound)
pub fn way_round(g: &mut crate::game::Game, gd: GuardId, l: crate::fireline::Lane, goal: V3) -> Option<Vec<V3>> { todo!() }

/// (src/game/fireline.ts:164 end)
pub fn end(g: &mut crate::game::Game, gd: GuardId, st: Option<&crate::fireline::St>) { todo!() }

/// (src/game/fireline.ts:166 hold)
pub fn hold(g: &mut crate::game::Game, gd: GuardId) -> crate::fireline::Hold { todo!() }

/// (src/game/fireline.ts:171 fireLineFeet)
pub fn fire_line_feet(g: &mut crate::game::Game, gd: GuardId, goal: Option<V3>, speed: f64, mine: bool) -> Option<crate::fireline::FireLineFeet> { todo!() }

/// (src/game/fireline.ts:221 fireLineLive)
pub fn fire_line_live(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }
