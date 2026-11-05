#![allow(unused_variables)]
//! Every function and method of `effects`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/effects.ts:195 muzzleFrame)
pub fn muzzle_frame(o: V3, d: V3) -> crate::effects::MuzzleFrame { todo!() }

/// (src/game/effects.ts:201 spawnShot)
pub fn spawn_shot(g: &mut crate::game::Game, muzzle: V3, dir: V3, o: &crate::effects::SpawnShotO) -> Vec<crate::effects::FxLight> { todo!() }

/// (src/game/effects.ts:230 spawnBurst)
pub fn spawn_burst(g: &mut crate::game::Game, at: V3, seed: f64, o: Option<crate::effects::SpawnBurstO>) -> crate::effects::FxLight { todo!() }

/// (src/game/effects.ts:274 spawnVent)
pub fn spawn_vent(g: &mut crate::game::Game, at: V3, yaw: f64, o: Option<crate::effects::SpawnVentO>) -> f64 { todo!() }

/// (src/game/effects.ts:282 ventMouth)
pub fn vent_mouth(item_pos: V3, yaw: f64) -> V3 { todo!() }

pub mod no_gas {
    #[allow(unused_imports)]
    use crate::{ids::*, js::*, Quat, V3};
    #[allow(unused_imports)]
    use alloc::{string::String, vec::Vec};
    /// (src/game/effects.ts:200 NO_GAS.emit)
    pub fn emit() { todo!() }

    /// (src/game/effects.ts:200 NO_GAS.push)
    pub fn push() { todo!() }

    /// (src/game/effects.ts:200 NO_GAS.inSmokeDomain)
    pub fn in_smoke_domain() -> bool { todo!() }

}
