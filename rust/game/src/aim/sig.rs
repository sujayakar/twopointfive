#![allow(unused_variables)]
//! Every function and method of `aim`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/aim.ts:125 AimState.clear)
pub fn aim_state_clear(this: &mut crate::aim::AimState) { todo!() }

/// (src/game/aim.ts:129 moveCost)
pub fn move_cost(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/aim.ts:130 lampCost)
pub fn lamp_cost(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/aim.ts:133 feedFeet)
pub fn feed_feet(g: &mut crate::game::Game, a: &mut crate::aim::AimState, speed: f64, dt: f64, tau: f64, on: bool, hold: Option<f64>) { todo!() }

/// (src/game/aim.ts:139 feetOf)
pub fn feet_of(g: &mut crate::game::Game, a: &crate::aim::AimState, vel: &[f64], steady_pace: f64, on: bool) -> f64 { todo!() }

/// (src/game/aim.ts:152 feedTarget)
pub fn feed_target(a: &mut crate::aim::AimState, from: V3, at: V3, vel: &[f64], dt: f64, jump_deg: f64, tau: f64) -> bool { todo!() }

/// (src/game/aim.ts:172 trackGuard)
pub fn track_guard(g: &mut crate::game::Game, gd: GuardId, dt: f64) { todo!() }

/// (src/game/aim.ts:199 settleTime)
pub fn settle_time(p: crate::aim::SettleTimeP, dist: f64) -> f64 { todo!() }

/// (src/game/aim.ts:204 aimReady)
pub fn aim_ready(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/aim.ts:207 feetSettled)
pub fn feet_settled(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/aim.ts:209 guardAcquired)
pub fn guard_acquired(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/aim.ts:212 guardTerms)
pub fn guard_terms(g: &mut crate::game::Game, gd: GuardId, extra: f64) -> crate::aim::GuardTerms { todo!() }

/// (src/game/aim.ts:224 guardRound)
pub fn guard_round(g: &mut crate::game::Game, gd: GuardId, extra: f64, gated: Option<bool>) -> crate::combat::FireOpts { todo!() }

/// (src/game/aim.ts:232 handsCone)
pub fn hands_cone(g: &mut crate::game::Game, gd: GuardId, extra: f64) -> f64 { todo!() }

/// (src/game/aim.ts:239 aimBrake)
pub fn aim_brake(g: &mut crate::game::Game, gd: GuardId, dist: f64, blocked: bool) -> bool { todo!() }

/// (src/game/aim.ts:258 noteBore)
pub fn note_bore(g: &mut crate::game::Game, gd: GuardId, aim_at: V3) { todo!() }

/// (src/game/aim.ts:269 boreFouled)
pub fn bore_fouled(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/aim.ts:279 trackPlayer)
pub fn track_player(g: &mut crate::game::Game, dt: f64) { todo!() }

/// (src/game/aim.ts:306 playerAim)
pub fn player_aim(g: &mut crate::game::Game) -> crate::aim::PlayerAim { todo!() }

/// (src/game/aim.ts:318 samAim)
pub fn sam_aim(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/aim.ts:331 saveAim)
pub fn save_aim(g: &mut crate::game::Game) { todo!() }

/// (src/game/aim.ts:339 loadAim)
pub fn load_aim(g: &mut crate::game::Game) { todo!() }

/// (src/game/aim.ts:347 resetAim)
pub fn reset_aim(g: &mut crate::game::Game) { todo!() }

/// (src/game/aim.ts:355 aimSummary)
pub fn aim_summary(g: &mut crate::game::Game) -> String { todo!() }

pub mod aim_hooks {
    #[allow(unused_imports)]
    use crate::{ids::*, js::*, Quat, V3};
    #[allow(unused_imports)]
    use alloc::{string::String, vec::Vec};
    /// (src/game/aim.ts:145 AIM_HOOKS.playerExternalK)
    pub fn player_external_k() -> f64 { todo!() }

}
