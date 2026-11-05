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

/// (src/game/lighting.ts:99 setBlackout)
pub fn set_blackout(g: &mut crate::game::Game, seconds: f64, cause: crate::lighting::SetBlackoutCause) { todo!() }

/// (src/game/lighting.ts:122 endBlackout)
pub fn end_blackout(g: &mut crate::game::Game) { todo!() }

/// (src/game/lighting.ts:126 mainsBack)
pub fn mains_back(g: &mut crate::game::Game) { todo!() }

/// (src/game/lighting.ts:131 lightEvent)
pub fn light_event(g: &mut crate::game::Game, t: FixtureId, cause: Option<crate::lighting::LightEventCause>, sw_at: Option<V3>) { todo!() }

/// (src/game/lighting.ts:139 ocpable)
pub fn ocpable(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/lighting.ts:145 ocp)
pub fn ocp(g: &mut crate::game::Game, t: Option<FixtureId>) { todo!() }

/// (src/game/lighting.ts:175 setGroup)
pub fn set_group(g: &mut crate::game::Game, group: &str, on: bool) { todo!() }

/// (src/game/lighting.ts:183 isDaylight)
pub fn is_daylight(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/lighting.ts:189 setDaylight)
pub fn set_daylight(g: &mut crate::game::Game, on: bool) { todo!() }

/// (src/game/lighting.ts:220 toggleTarget)
pub fn toggle_target(g: &mut crate::game::Game, t: FixtureId) { todo!() }

/// (src/game/lighting.ts:223 repairLights)
pub fn repair_lights(g: &mut crate::game::Game) { todo!() }

/// (src/game/lighting.ts:228 freshLights)
pub fn fresh_lights(g: &mut crate::game::Game) -> bool { todo!() }
