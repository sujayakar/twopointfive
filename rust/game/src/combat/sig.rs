#![allow(unused_variables)]
//! Every function and method of `combat`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/combat.ts:95 firearmOf)
pub fn firearm_of(g: &mut crate::game::Game, shooter: CharId, is_player: bool) -> FirearmId { todo!() }

/// (src/game/combat.ts:103 nearLamp)
pub fn near_lamp(g: &mut crate::game::Game, t: FixtureId, legs: &[crate::aimcost::RoundLeg], slack: f64) -> bool { todo!() }

/// (src/game/combat.ts:114 aiWide)
pub fn ai_wide(g: &mut crate::game::Game, sg: Option<GuardId>, shooter: CharId, def: &crate::weapons::WeaponDef) -> f64 { todo!() }

/// (src/game/combat.ts:120 fireWeapon)
pub fn fire_weapon(g: &mut crate::game::Game, shooter: CharId, target: V3, is_player: bool, o: Option<&crate::combat::FireOpts>) { todo!() }

/// (src/game/combat.ts:121 fireWeaponRaw)
pub fn fire_weapon_raw(g: &mut crate::game::Game, shooter: CharId, target: V3, is_player: bool, o: &crate::combat::FireOpts) { todo!() }

/// (src/game/combat.ts:324 acrossArm)
pub fn across_arm(g: &mut crate::game::Game, c_2: CharId) -> Option<crate::combat::AcrossArm> { todo!() }

/// (src/game/combat.ts:328 glassAcrossArm)
pub fn glass_across_arm(g: &mut crate::game::Game, c_2: CharId, dir: V3) { todo!() }

/// (src/game/combat.ts:337 wallAcrossArm)
pub fn wall_across_arm(g: &mut crate::game::Game, c_2: CharId, def: &crate::weapons::WeaponDef) { todo!() }

/// (src/game/combat.ts:346 armThrough)
pub fn arm_through(g: &mut crate::game::Game, c_2: CharId) -> bool { todo!() }

/// (src/game/combat.ts:351 solidBetween)
pub fn solid_between(g: &mut crate::game::Game, from: V3, to: V3) -> bool { todo!() }

/// (src/game/combat.ts:361 contactClear)
pub fn contact_clear(g: &mut crate::game::Game, a: CharId, b: CharId) -> bool { todo!() }

/// (src/game/combat.ts:372 bodyHit)
pub fn body_hit(g: &mut crate::game::Game, c_2: CharId, ro: V3, rd: V3, tmax: f64, contact: Option<crate::combat::BodyHitContact>) -> f64 { todo!() }

/// (src/game/combat.ts:387 killGuard)
pub fn kill_guard(g: &mut crate::game::Game, gd: GuardId, dir: V3, quiet: Option<bool>, how: Option<crate::game::DownKind>, by: Option<crate::game::DownBy>, line: Option<&str>, zone: Option<crate::combat::KillGuardZone>) { todo!() }

/// (src/game/combat.ts:417 dropFromHand)
pub fn drop_from_hand(g: &mut crate::game::Game, c_2: CharId, kind: crate::combat::DropFromHandKind, along: V3, dir: V3, suppressed: Option<bool>, quiet: Option<bool>, from: Option<V3>) -> ItemId { todo!() }

/// (src/game/combat.ts:428 hitPlayer)
pub fn hit_player(g: &mut crate::game::Game, from_dir: V3) { todo!() }

/// (src/game/combat.ts:446 handProps)
pub fn hand_props(g: &mut crate::game::Game, out: &mut Vec<BoxId>) { todo!() }

/// (src/game/combat.ts:454 rigKeys)
pub fn rig_keys(g: &mut crate::game::Game, out: &mut Vec<BoxId>, n0: f64) { todo!() }

/// (src/game/combat.ts:458 handPropsRaw)
pub fn hand_props_raw(g: &mut crate::game::Game, out: &mut Vec<BoxId>) { todo!() }

/// (src/game/combat.ts:467 playerProps)
pub fn player_props(g: &mut crate::game::Game, out: &mut Vec<BoxId>) { todo!() }

/// (src/game/combat.ts:504 hammerSide)
pub fn hammer_side(g: &mut crate::game::Game, c_2: CharId) -> Option<crate::combat::HammerSide> { todo!() }

/// (src/game/combat.ts:509 guardProps)
pub fn guard_props(g: &mut crate::game::Game, gd: GuardId, out: &mut Vec<BoxId>) { todo!() }

/// (src/game/combat.ts:523 freeCan)
pub fn free_can(g: &mut crate::game::Game, gd: GuardId, out: &mut Vec<BoxId>) { todo!() }

/// (src/game/combat.ts:525 hungSide)
pub fn hung_side(g: &mut crate::game::Game, gd: GuardId, out: &mut Vec<BoxId>) { todo!() }

/// (src/game/combat.ts:527 guardKit)
pub fn guard_kit(g: &mut crate::game::Game, gd: GuardId, out: &mut Vec<BoxId>, hang: bool) { todo!() }

/// (src/game/combat.ts:556 hiddenProps)
pub fn hidden_props(g: &mut crate::game::Game, out: &mut Vec<BoxId>) { todo!() }

/// (src/game/combat.ts:565 irBeams)
pub fn ir_beams(g: &mut crate::game::Game, out: &mut Vec<BoxId>) { todo!() }

/// (src/game/combat.ts:584 laserBoxes)
pub fn laser_boxes(g: &mut crate::game::Game, gd: GuardId, out: &mut Vec<BoxId>) { todo!() }

/// (src/game/combat.ts:602 laserReach)
pub fn laser_reach(g: &mut crate::game::Game, c_2: CharId, self_: Option<GuardId>, civ: Option<bool>) -> Option<crate::combat::LaserReach> { todo!() }

/// (src/game/combat.ts:617 beamBoxes)
pub fn beam_boxes(g: &mut crate::game::Game, from: V3, dir: V3, l: f64, out: &mut Vec<BoxId>, dot: bool, ir: Option<bool>, air: Option<bool>) { todo!() }

/// (src/game/combat.ts:629 updateItems)
pub fn update_items(g: &mut crate::game::Game, dt: f64) { todo!() }

/// (src/game/combat.ts:646 smokeParams)
pub fn smoke_params(g: &mut crate::game::Game) -> Option<crate::smoke_types::CanisterParams> { todo!() }

/// (src/game/combat.ts:648 startCanister)
pub fn start_canister(g: &mut crate::game::Game, it: ItemId) { todo!() }

/// (src/game/combat.ts:679 flashFuse)
pub fn flash_fuse(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/combat.ts:684 detonateStun)
pub fn detonate_stun(g: &mut crate::game::Game, it: ItemId, borrowed: Option<bool>, sam: Option<crate::combat::DetonateStunSam>) { todo!() }

/// (src/game/combat.ts:685 detonateStunRaw)
pub fn detonate_stun_raw(g: &mut crate::game::Game, it: ItemId, borrowed: bool, sam: Option<crate::combat::DetonateStunRawSam>) { todo!() }

/// (src/game/combat.ts:751 dropThrowable)
pub fn drop_throwable(g: &mut crate::game::Game, kind: crate::game::ThrowKind, p: V3) { todo!() }
