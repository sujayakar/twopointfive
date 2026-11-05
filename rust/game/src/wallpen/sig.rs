#![allow(unused_variables)]
//! Every function and method of `wallpen`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/wallpen.ts:90 wallPenLive)
pub fn wall_pen_live(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/wallpen.ts:94 thinN)
pub fn thin_n(g: &mut crate::game::Game, b: BoxId) -> V3 { todo!() }

/// (src/game/wallpen.ts:96 faceN)
pub fn face_n(g: &mut crate::game::Game, b: BoxId, p: V3) -> V3 { todo!() }

/// (src/game/wallpen.ts:103 roofed)
pub fn roofed(g: &mut crate::game::Game, x: f64, z: f64) -> bool { todo!() }

/// (src/game/wallpen.ts:113 stuffOf)
pub fn stuff_of(g: &mut crate::game::Game, index: f64, b: BoxId) -> Option<crate::wallpen::ThinStuff> { todo!() }

/// (src/game/wallpen.ts:129 thingAt)
pub fn thing_at(g: &mut crate::game::Game, index: f64) -> Option<crate::wallpen::ThingAt> { todo!() }

/// (src/game/wallpen.ts:134 thinThing)
pub fn thin_thing(g: &mut crate::game::Game, index: f64) -> Option<crate::wallpen::ThinThing> { todo!() }

/// (src/game/wallpen.ts:143 thinToll)
pub fn thin_toll(g: &mut crate::game::Game, index: f64, at: V3, d: V3, e: f64) -> Option<crate::wallpen::ThinToll> { todo!() }

/// (src/game/wallpen.ts:173 bornIn)
pub fn born_in(g: &mut crate::game::Game, p: V3, d: V3, self_: f64) -> f64 { todo!() }

/// (src/game/wallpen.ts:186 deflect)
pub fn deflect(g: &mut crate::game::Game, b: BoxId, at: V3, d: V3, take: f64, e_in: f64) -> V3 { todo!() }

/// (src/game/wallpen.ts:194 crossing)
pub fn crossing(g: &mut crate::game::Game, index: f64, o: V3, d: V3, t: f64, n: V3, e: f64) -> Option<crate::wallpen::WallCross> { todo!() }

/// (src/game/wallpen.ts:209 thinPath)
pub fn thin_path(g: &mut crate::game::Game, from: V3, dir: V3, range: f64, energy: f64, first: Option<crate::wallpen::StaticHit2>, nth0: Option<f64>) -> Option<crate::wallpen::ThinPath> { todo!() }

/// (src/game/wallpen.ts:234 commitWalls)
pub fn commit_walls(g: &mut crate::game::Game, walls: &[crate::wallpen::WallCross], t_hit: f64, shooter: CharId, def: &crate::weapons::WeaponDef, lead: bool, third: bool, pel: Option<&mut crate::shotgun::Shell>) { todo!() }

/// (src/game/wallpen.ts:251 wallMenLive)
pub fn wall_men_live(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/wallpen.ts:258 thinLine)
pub fn thin_line(g: &mut crate::game::Game, from: V3, to: V3, energy: f64, short: Option<f64>, first: Option<crate::wallpen::ThinLineFirst>) -> Option<crate::wallpen::ThinLine2> { todo!() }

/// (src/game/wallpen.ts:268 topEnergy)
pub fn top_energy(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/wallpen.ts:270 flightLen)
pub fn flight_len(g: &mut crate::game::Game, from: V3, dir: V3, range: f64, energy: f64) -> f64 { todo!() }

/// (src/game/wallpen.ts:277 spentInWall)
pub fn spent_in_wall(g: &mut crate::game::Game, index: f64) { todo!() }

/// (src/game/wallpen.ts:280 wallPenSummary)
pub fn wall_pen_summary(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/wallpen.ts:285 wallPenLog)
pub fn wall_pen_log(g: &mut crate::game::Game) -> Vec<String> { todo!() }
