//! The types of `combat`.
#![allow(unused_imports)]
use crate::{Blank, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// bodyHit's contact: what decides a round whose origin is already inside the man (combat.ts bodyHit). `true` is the parameter's None. Entering = false: the entering hit only. Muzzle = fireWeapon's test. LegCrown = a later leg through glass, ro[1] <= headTop's height (or pos + 1.8). Flight = a ring or a 40 mm round, ro[1] < headTop's height (or pos + 1.75) + radius
#[derive(Clone, Debug)]
pub enum BodyHitContact {
    Entering,
    Muzzle { st0_t: Option<f64>, muzzle: V3, glass_arm: bool, shooter: CharId },
    LegCrown,
    Flight { radius: f64 },
}

#[derive(Clone, Debug, Default)]
pub struct FireOpts {
    pub spread: Option<f64>,
    pub weapon: Option<crate::weapons::WeaponDef>,
    pub steady: Option<bool>,
    pub blind: Option<bool>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct AcrossArm {
    pub index: f64,
    pub at: V3,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum DropFromHandKind {
    Pistol,
    Rifle,
    Torch,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct HammerSide {
    pub o: V3,
    pub z: V3,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct LaserReach {
    pub from: V3,
    pub dir: V3,
    pub l: f64,
    pub dot: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct DetonateStunSam {
    pub side: Option<crate::teams::Team>,
    pub spare: Option<bool>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct DetonateStunRawSam {
    pub side: Option<crate::teams::Team>,
    pub spare: Option<bool>,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct CombatMod {
    /// (src/game/combat.ts:100 strayGun)
    pub stray_gun: Option<FirearmId>,
}

/// (src/game/combat.ts:359 solidScratch: a table nothing writes)
pub fn solid_scratch() -> Vec<i32> { alloc::vec![0; 64] }

/// (src/game/combat.ts:427 GUN_ITEM_LIFE)
pub const GUN_ITEM_LIFE: f64 = 240.0;

/// (src/game/combat.ts:454 handOrd: a table nothing writes)
pub fn hand_ord() -> Vec<u16> { alloc::vec![0; 256] }
