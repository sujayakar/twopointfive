#![allow(unused_variables)]
//! Every function and method of `shotgun`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/shotgun.ts:74 pelletDrops)
pub fn pellet_drops(g: &mut crate::game::Game, s: &mut crate::shotgun::Shell, who: Option<GuardId>, t: f64, dir: V3) -> bool { todo!() }

/// (src/game/shotgun.ts:81 pelletEnd)
pub fn pellet_end(s: &mut crate::shotgun::Shell, index: f64, t: f64, at: V3) { todo!() }

/// (src/game/shotgun.ts:84 fireShell)
pub fn fire_shell(g: &mut crate::game::Game, shooter: CharId, target: V3, is_player: bool, o: &crate::combat::FireOpts, def: &crate::weapons::WeaponDef) { todo!() }

/// (src/game/shotgun.ts:125 carriesBreacher)
pub fn carries_breacher(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/shotgun.ts:128 lockPoint)
pub fn lock_point(g: &mut crate::game::Game, d: DoorId, side: f64) -> V3 { todo!() }

/// (src/game/shotgun.ts:136 breachSpot)
pub fn breach_spot(g: &mut crate::game::Game, d: DoorId, side: f64, avoid: Option<&[V3]>, mark: Option<V3>, close: Option<crate::shotgun::BreachSpotClose>) -> Option<crate::shotgun::BreachSpot> { todo!() }
