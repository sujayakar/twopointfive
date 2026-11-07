#![allow(unused_variables)]
//! Every function and method of `combat`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/combat.ts:96 firearmOf)
pub fn firearm_of(g: &mut crate::game::Game, shooter: CharId, is_player: bool) -> FirearmId { todo!() }

/// (src/game/combat.ts:104 nearLamp)
pub fn near_lamp(g: &mut crate::game::Game, t: FixtureId, legs: &[crate::aimcost::RoundLeg], slack: f64) -> bool { todo!() }

/// (src/game/combat.ts:115 aiWide)
pub fn ai_wide(g: &mut crate::game::Game, sg: Option<GuardId>, shooter: CharId, def: &crate::weapons::WeaponDef) -> f64 { todo!() }

/// (src/game/combat.ts:121 fireWeapon)
pub fn fire_weapon(g: &mut crate::game::Game, shooter: CharId, target: V3, is_player: bool, o: Option<&crate::combat::FireOpts>) { todo!() }

/// (src/game/combat.ts:122 fireWeaponRaw)
pub fn fire_weapon_raw(g: &mut crate::game::Game, shooter: CharId, target: V3, is_player: bool, o: &crate::combat::FireOpts, pel: Option<&mut crate::shotgun::Shell>) { todo!() }

/// (src/game/combat.ts:325 acrossArm)
pub fn across_arm(g: &mut crate::game::Game, c_2: CharId) -> Option<crate::combat::AcrossArm> { todo!() }

/// (src/game/combat.ts:329 glassAcrossArm)
pub fn glass_across_arm(g: &mut crate::game::Game, c_2: CharId, dir: V3) { todo!() }

/// (src/game/combat.ts:338 wallAcrossArm)
pub fn wall_across_arm(g: &mut crate::game::Game, c_2: CharId, def: &crate::weapons::WeaponDef) { todo!() }

/// (src/game/combat.ts:347 armThrough)
pub fn arm_through(g: &mut crate::game::Game, c_2: CharId) -> bool { todo!() }

/// (src/game/combat.ts:352 solidBetween)
pub fn solid_between(g: &mut crate::game::Game, from: V3, to: V3) -> bool { todo!() }

/// (src/game/combat.ts:362 contactClear)
pub fn contact_clear(g: &mut crate::game::Game, a: CharId, b: CharId) -> bool { todo!() }

/// (src/game/combat.ts:373 bodyHit)
pub fn body_hit(g: &mut crate::game::Game, c_2: CharId, ro: V3, rd: V3, tmax: f64, contact: Option<crate::combat::BodyHitContact>) -> f64 { todo!() }

/// (src/game/combat.ts:388 killGuard)
pub fn kill_guard(g: &mut crate::game::Game, gd: GuardId, dir: V3, quiet: Option<bool>, how: Option<crate::game::DownKind>, by: Option<crate::game::DownBy>, line: Option<&str>, zone: Option<crate::combat::KillGuardZone>) { todo!() }

/// (src/game/combat.ts:418 dropFromHand)
pub fn drop_from_hand(g: &mut crate::game::Game, c_2: CharId, kind: crate::combat::DropFromHandKind, along: V3, dir: V3, suppressed: Option<bool>, quiet: Option<bool>, from: Option<V3>) -> ItemId { todo!() }

/// (src/game/combat.ts:429 hitPlayer)
pub fn hit_player(g: &mut crate::game::Game, from_dir: V3) { todo!() }

/// (src/game/combat.ts:447 handProps)
pub fn hand_props(g: &mut crate::game::Game, out: &mut Vec<BoxId>) { todo!() }

/// (src/game/combat.ts:455 rigKeys)
pub fn rig_keys(g: &mut crate::game::Game, out: &mut Vec<BoxId>, n0: f64) { todo!() }

/// (src/game/combat.ts:459 handPropsRaw)
pub fn hand_props_raw(g: &mut crate::game::Game, out: &mut Vec<BoxId>) { todo!() }

/// (src/game/combat.ts:468 playerProps)
pub fn player_props(g: &mut crate::game::Game, out: &mut Vec<BoxId>) { todo!() }

/// (src/game/combat.ts:505 hammerSide)
pub fn hammer_side(g: &mut crate::game::Game, c_2: CharId) -> Option<crate::combat::HammerSide> { todo!() }

/// (src/game/combat.ts:510 guardProps)
pub fn guard_props(g: &mut crate::game::Game, gd: GuardId, out: &mut Vec<BoxId>) { todo!() }

/// (src/game/combat.ts:524 freeCan)
pub fn free_can(g: &mut crate::game::Game, gd: GuardId, out: &mut Vec<BoxId>) { todo!() }

/// (src/game/combat.ts:526 hungSide)
pub fn hung_side(g: &mut crate::game::Game, gd: GuardId, out: &mut Vec<BoxId>) { todo!() }

/// (src/game/combat.ts:528 guardKit)
pub fn guard_kit(g: &mut crate::game::Game, gd: GuardId, out: &mut Vec<BoxId>, hang: bool) { todo!() }

/// (src/game/combat.ts:557 hiddenProps)
pub fn hidden_props(g: &mut crate::game::Game, out: &mut Vec<BoxId>) { todo!() }

/// (src/game/combat.ts:566 irBeams)
pub fn ir_beams(g: &mut crate::game::Game, out: &mut Vec<BoxId>) { todo!() }

/// (src/game/combat.ts:585 laserBoxes)
pub fn laser_boxes(g: &mut crate::game::Game, gd: GuardId, out: &mut Vec<BoxId>) { todo!() }

/// (src/game/combat.ts:603 laserReach)
pub fn laser_reach(g: &mut crate::game::Game, c_2: CharId, self_: Option<GuardId>, civ: Option<bool>) -> Option<crate::combat::LaserReach> { todo!() }

/// (src/game/combat.ts:618 beamBoxes)
pub fn beam_boxes(g: &mut crate::game::Game, from: V3, dir: V3, l: f64, out: &mut Vec<BoxId>, dot: bool, ir: Option<bool>, air: Option<bool>) { todo!() }

/// (src/game/combat.ts:630 updateItems)
pub fn update_items(g: &mut crate::game::Game, dt: f64) { todo!() }

/// (src/game/combat.ts:647 smokeParams)
pub fn smoke_params(g: &mut crate::game::Game) -> Option<crate::smoke_types::CanisterParams> { todo!() }

/// (src/game/combat.ts:649 startCanister)
pub fn start_canister(g: &mut crate::game::Game, it: ItemId) { todo!() }

/// (src/game/combat.ts:680 flashFuse)
pub fn flash_fuse(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/combat.ts:685 detonateStun)
pub fn detonate_stun(g: &mut crate::game::Game, it: ItemId, borrowed: Option<bool>, sam: Option<crate::combat::DetonateStunSam>) { todo!() }

/// (src/game/combat.ts:686 detonateStunRaw)
pub fn detonate_stun_raw(g: &mut crate::game::Game, it: ItemId, borrowed: bool, sam: Option<crate::combat::DetonateStunRawSam>) { todo!() }

/// (src/game/combat.ts:752 dropThrowable)
pub fn drop_throwable(g: &mut crate::game::Game, kind: crate::game::ThrowKind, p: V3) { todo!() }


/// INVENTED: fireWeapon for one pellet of a shell, the shell lent so the round writes on it (dir, ends, men, glass, late). The same wrapper as fire_weapon (view, fog begin, the round, bars unmeet, fog done), around fire_weapon_raw with pel = Some(shell). (shotgun.ts:88, :95; combat.ts:120)
pub fn fire_pellet(g: &mut crate::game::Game, shooter: CharId, target: V3, is_player: bool, o: &crate::combat::FireOpts, shell: &mut crate::shotgun::Shell) { todo!() }

/// INVENTED: runs one row of Shell.late, what a round in a man does (fireWeaponRaw's manDown and samHit closures, combat.ts:256, :271). Called in the shooter's seat: by fire_shell's walk of the shell, and by the round itself when it is not a pellet.
pub fn run_shell_late(g: &mut crate::game::Game, late: &crate::shotgun::ShellLate) { todo!() }
