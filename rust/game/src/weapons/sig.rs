#![allow(unused_variables)]
//! Every function and method of `weapons`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/weapons.ts:324 bareRow)
pub fn bare_row(of: &crate::weapons::WeaponDef, over: &crate::weapons::BareRowOver) -> crate::weapons::WeaponDef { todo!() }

/// (src/game/weapons.ts:345 handlingOf)
pub fn handling_of(def: &crate::weapons::WeaponDef) -> f64 { todo!() }

/// (src/game/weapons.ts:374 Firearm.shellFed)
pub fn firearm_shell_fed(g: &mut crate::game::Game, id: FirearmId) -> bool { todo!() }

/// (src/game/weapons.ts:376 Firearm.reloadRow)
pub fn firearm_reload_row(g: &mut crate::game::Game, id: FirearmId) -> crate::weapons::WeaponDefReload { todo!() }

/// (src/game/weapons.ts:389 Firearm.constructor)
pub fn firearm_new(g: &mut crate::game::Game, def: &crate::weapons::WeaponDef) -> FirearmId { todo!() }

/// (src/game/weapons.ts:395 Firearm.name)
pub fn firearm_name(g: &mut crate::game::Game, id: FirearmId) -> String { todo!() }

/// (src/game/weapons.ts:397 Firearm.cycleMode)
pub fn firearm_cycle_mode(g: &mut crate::game::Game, id: FirearmId) -> crate::weapons::FireMode { todo!() }

/// (src/game/weapons.ts:399 Firearm.automatic)
pub fn firearm_automatic(g: &mut crate::game::Game, id: FirearmId) -> bool { todo!() }

/// (src/game/weapons.ts:401 Firearm.autoSpread)
pub fn firearm_auto_spread(g: &mut crate::game::Game, id: FirearmId) -> f64 { todo!() }

/// (src/game/weapons.ts:403 Firearm.autoFired)
pub fn firearm_auto_fired(g: &mut crate::game::Game, id: FirearmId) { todo!() }

/// (src/game/weapons.ts:404 Firearm.autoIdle)
pub fn firearm_auto_idle(g: &mut crate::game::Game, id: FirearmId, dt: f64, trigger_down: bool) { todo!() }

/// (src/game/weapons.ts:406 Firearm.bottomless)
pub fn firearm_bottomless(g: &mut crate::game::Game, id: FirearmId) -> bool { todo!() }

/// (src/game/weapons.ts:407 Firearm.reloading)
pub fn firearm_reloading(g: &mut crate::game::Game, id: FirearmId) -> bool { todo!() }

/// (src/game/weapons.ts:408 Firearm.roundsReady)
pub fn firearm_rounds_ready(g: &mut crate::game::Game, id: FirearmId) -> f64 { todo!() }

/// (src/game/weapons.ts:410 Firearm.spent)
pub fn firearm_spent(g: &mut crate::game::Game, id: FirearmId) -> f64 { todo!() }

/// (src/game/weapons.ts:411 Firearm.canFire)
pub fn firearm_can_fire(g: &mut crate::game::Game, id: FirearmId) -> bool { todo!() }

/// (src/game/weapons.ts:413 Firearm.spareSorted)
pub fn firearm_spare_sorted(g: &mut crate::game::Game, id: FirearmId) -> Vec<f64> { todo!() }

/// (src/game/weapons.ts:415 Firearm.fire)
pub fn firearm_fire(g: &mut crate::game::Game, id: FirearmId) -> bool { todo!() }

/// (src/game/weapons.ts:425 Firearm.magRoom)
pub fn firearm_mag_room(g: &mut crate::game::Game, id: FirearmId) -> f64 { todo!() }

/// (src/game/weapons.ts:427 Firearm.startReload)
pub fn firearm_start_reload(g: &mut crate::game::Game, id: FirearmId, ev: Option<&crate::weapons::ReloadEvents>, beat: Option<crate::weapons::WeaponDefReload>, cues: Option<&[f64]>, shells: Option<&crate::weapons::FirearmStartReloadShells>) -> bool { todo!() }

/// (src/game/weapons.ts:441 Firearm.cancelReload)
pub fn firearm_cancel_reload(g: &mut crate::game::Game, id: FirearmId) { todo!() }

/// (src/game/weapons.ts:442 Firearm.planShells)
pub fn firearm_plan_shells(g: &mut crate::game::Game, id: FirearmId, times: &[f64], port: bool) { todo!() }

/// (src/game/weapons.ts:448 Firearm.planFeed)
pub fn firearm_plan_feed(g: &mut crate::game::Game, id: FirearmId, row: crate::weapons::WeaponDefReload, times: &[f64], port: bool) { todo!() }

/// (src/game/weapons.ts:452 Firearm.feedTo)
pub fn firearm_feed_to(g: &mut crate::game::Game, id: FirearmId, t: f64, free: Option<bool>) { todo!() }

/// (src/game/weapons.ts:455 Firearm.feedShell)
pub fn firearm_feed_shell(g: &mut crate::game::Game, id: FirearmId, port: Option<bool>, free: Option<bool>) -> bool { todo!() }

/// (src/game/weapons.ts:468 Firearm.rearm)
pub fn firearm_rearm(g: &mut crate::game::Game, id: FirearmId) { todo!() }

/// (src/game/weapons.ts:471 Firearm.fit)
pub fn firearm_fit(g: &mut crate::game::Game, id: FirearmId) { todo!() }

/// (src/game/weapons.ts:473 Firearm.update)
pub fn firearm_update(g: &mut crate::game::Game, id: FirearmId, dt: f64) { todo!() }

/// (src/game/weapons.ts:511 Pistol.constructor)
pub fn pistol_new(def: Option<&crate::weapons::WeaponDef>) -> crate::Opaque { todo!() }

/// (src/game/weapons.ts:531 Rifle.constructor)
pub fn rifle_new(def: Option<&crate::weapons::WeaponDef>) -> crate::Opaque { todo!() }

/// (src/game/weapons.ts:533 Rifle.attachment)
pub fn rifle_attachment() -> Option<crate::weapons::RifleAttachment> { todo!() }

/// (src/game/weapons.ts:535 Rifle.left)
pub fn rifle_left(a: crate::weapons::RifleAttachment) -> f64 { todo!() }

/// (src/game/weapons.ts:538 Rifle.cycleAttachment)
pub fn rifle_cycle_attachment() -> Option<crate::weapons::RifleAttachment> { todo!() }

/// (src/game/weapons.ts:544 Ocp.ready)
pub fn ocp_ready(this: &mut crate::weapons::Ocp) -> bool { todo!() }

/// (src/game/weapons.ts:545 Ocp.use)
pub fn ocp_use(this: &mut crate::weapons::Ocp) -> bool { todo!() }

/// (src/game/weapons.ts:546 Ocp.update)
pub fn ocp_update(this: &mut crate::weapons::Ocp, dt: f64) { todo!() }
