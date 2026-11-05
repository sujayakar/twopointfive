#![allow(unused_variables)]
//! Every function and method of `gunlook`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/gunlook.ts:27 gunLookOn)
pub fn gun_look_on(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/gunlook.ts:29 gunShellsOn)
pub fn gun_shells_on(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/gunlook.ts:31 gunOwn)
pub fn gun_own(g: &mut crate::game::Game, def: &crate::weapons::WeaponDef) -> Option<crate::weapons::WeaponDefOwn> { todo!() }

/// (src/game/gunlook.ts:33 voiceOf)
pub fn voice_of(g: &mut crate::game::Game, def: &crate::weapons::WeaponDef) -> String { todo!() }

/// (src/game/gunlook.ts:35 flameOf)
pub fn flame_of(g: &mut crate::game::Game, def: &crate::weapons::WeaponDef) -> crate::effects::MuzzleFxKey { todo!() }

/// (src/game/gunlook.ts:79 under)
pub fn under(z: f64, x: Option<f64>, y: Option<f64>) -> crate::gunlook::LongModelGripL { todo!() }

/// (src/game/gunlook.ts:189 longModel)
pub fn long_model(g: &mut crate::game::Game, id_2: Option<crate::weapons::GunModelId>) -> Option<crate::gunlook::LongModel> { todo!() }

/// (src/game/gunlook.ts:190 pistolModel)
pub fn pistol_model(g: &mut crate::game::Game, id_2: Option<crate::weapons::GunModelId>) -> Option<crate::gunlook::PistolModel> { todo!() }

/// (src/game/gunlook.ts:201 gunLookOf)
pub fn gun_look_of(g: &mut crate::game::Game, into: Option<crate::gunlook::GunLook>, in_hands: Option<&crate::weapons::WeaponDef>, other: Option<&crate::weapons::WeaponDef>, third: Option<&crate::weapons::WeaponDef>) -> Option<crate::gunlook::GunLook> { todo!() }
