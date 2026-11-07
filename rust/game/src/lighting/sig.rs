#![allow(unused_variables)]
//! Every function and method of `lighting`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/lighting.ts:40 lensClear)
pub fn lens_clear(g: &mut crate::game::Game, from: V3, lens: V3, radius: f64, light: Option<LightId>) -> V3 { todo!() }

/// (src/game/lighting.ts:54 lensRecheck)
pub fn lens_recheck(g: &mut crate::game::Game) { todo!() }

/// (src/game/lighting.ts:63 theirMains)
pub fn their_mains(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/lighting.ts:66 breakerResetNear)
pub fn breaker_reset_near(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/lighting.ts:75 updateTargets)
pub fn update_targets(g: &mut crate::game::Game, dt: f64) { todo!() }

/// (src/game/lighting.ts:96 setBlackout)
pub fn set_blackout(g: &mut crate::game::Game, seconds: f64, cause: crate::lighting::SetBlackoutCause) { todo!() }

/// (src/game/lighting.ts:119 endBlackout)
pub fn end_blackout(g: &mut crate::game::Game) { todo!() }

/// (src/game/lighting.ts:123 mainsBack)
pub fn mains_back(g: &mut crate::game::Game) { todo!() }

/// (src/game/lighting.ts:128 lightEvent)
pub fn light_event(g: &mut crate::game::Game, t: FixtureId, cause: Option<crate::lighting::LightEventCause>, sw_at: Option<V3>) { todo!() }

/// (src/game/lighting.ts:136 ocpable)
pub fn ocpable(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/lighting.ts:142 ocp)
pub fn ocp(g: &mut crate::game::Game, t: Option<FixtureId>) { todo!() }

/// (src/game/lighting.ts:172 setGroup)
pub fn set_group(g: &mut crate::game::Game, group: &str, on: bool) { todo!() }

/// (src/game/lighting.ts:180 isDaylight)
pub fn is_daylight(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/lighting.ts:186 setDaylight)
pub fn set_daylight(g: &mut crate::game::Game, on: bool) { todo!() }

/// (src/game/lighting.ts:217 toggleTarget)
pub fn toggle_target(g: &mut crate::game::Game, t: FixtureId) { todo!() }

/// (src/game/lighting.ts:220 repairLights)
pub fn repair_lights(g: &mut crate::game::Game) { todo!() }

/// (src/game/lighting.ts:225 freshLights)
pub fn fresh_lights(g: &mut crate::game::Game) -> bool { todo!() }
