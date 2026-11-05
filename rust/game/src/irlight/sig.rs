#![allow(unused_variables)]
//! Every function and method of `irlight`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/irlight.ts:100 irFlags)
pub fn ir_flags(g: &mut crate::game::Game) { todo!() }

/// (src/game/irlight.ts:117 kitOf)
pub fn kit_of(g: &mut crate::game::Game) -> crate::irlight::Kit { todo!() }

/// (src/game/irlight.ts:118 lamp)
pub fn lamp(g: &mut crate::game::Game, name: &str, owner: f64) -> LightId { todo!() }

/// (src/game/irlight.ts:122 pose)
pub fn pose(g: &mut crate::game::Game, l: LightId, c_2: CharId) { todo!() }

/// (src/game/irlight.ts:131 darkAt)
pub fn dark_at(g: &mut crate::game::Game, x: f64, z: f64) -> bool { todo!() }

/// (src/game/irlight.ts:137 fanDark)
pub fn fan_dark(g: &mut crate::game::Game, at: V3, yaw: f64) -> bool { todo!() }

/// (src/game/irlight.ts:148 skey)
pub fn skey(key: f64, i: f64) -> String { todo!() }

/// (src/game/irlight.ts:149 forget)
pub fn forget(g: &mut crate::game::Game, key: f64) { todo!() }

/// (src/game/irlight.ts:151 fanDarks)
pub fn fan_darks(g: &mut crate::game::Game, at: V3, yaw: f64) -> Vec<f64> { todo!() }

/// (src/game/irlight.ts:160 shownDark)
pub fn shown_dark(g: &mut crate::game::Game, cpu: f64, e: Option<f64>) -> Option<bool> { todo!() }

/// (src/game/irlight.ts:166 drop)
pub fn drop(g: &mut crate::game::Game, key: f64) { todo!() }

/// (src/game/irlight.ts:173 burn)
pub fn burn(g: &mut crate::game::Game, key: f64, at: V3, yaw: f64, each: bool, live: bool, own_key: Option<&str>) -> f64 { todo!() }

/// (src/game/irlight.ts:212 samOn)
pub fn sam_on(g: &mut crate::game::Game, pl: PlayerId) -> bool { todo!() }

/// (src/game/irlight.ts:214 out)
pub fn out(l: Option<LightId>) { todo!() }

/// (src/game/irlight.ts:220 nearK)
pub fn near_k(g: &mut crate::game::Game, key: f64, l: LightId) -> f64 { todo!() }

/// (src/game/irlight.ts:230 irNearOf)
pub fn ir_near_of(g: &mut crate::game::Game, key: f64) -> f64 { todo!() }

/// (src/game/irlight.ts:232 irLightTick)
pub fn ir_light_tick(g: &mut crate::game::Game) { todo!() }

/// (src/game/irlight.ts:267 irLightAt)
pub fn ir_light_at(g: &mut crate::game::Game, p: V3, skip: Option<f64>) -> f64 { todo!() }

/// (src/game/irlight.ts:285 irOnMan)
pub fn ir_on_man(g: &mut crate::game::Game, o: GuardId) -> f64 { todo!() }

/// (src/game/irlight.ts:294 irShowsAt)
pub fn ir_shows_at(g: &mut crate::game::Game, m: GuardId, p: V3) -> bool { todo!() }

/// (src/game/irlight.ts:302 irCarried)
pub fn ir_carried(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/irlight.ts:305 irLightState)
pub fn ir_light_state(g: &mut crate::game::Game) -> crate::irlight::IrLightState { todo!() }

/// (src/game/irlight.ts:310 irLightSummary)
pub fn ir_light_summary(g: &mut crate::game::Game) -> String { todo!() }
