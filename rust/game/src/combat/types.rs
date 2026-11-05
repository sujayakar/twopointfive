//! The types of `combat`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// bodyHit's contact: what decides a round whose origin is already inside the man (combat.ts bodyHit). `true` is the parameter's None. Entering = false: the entering hit only. Muzzle = fireWeapon's test. LegCrown = a later leg through glass, ro[1] <= headTop's height (or pos + 1.8). Flight = a ring or a 40 mm round, ro[1] < headTop's height (or pos + 1.75) + radius
#[derive(Clone, Debug)]
pub enum BodyHitContact {
    Entering,
    Muzzle { st0_t: Option<f64>, muzzle: V3, glass_arm: bool, shooter: CharId },
    LegCrown,
    Flight { radius: f64 },
}

/// (TypeScript: FireOpts, src/game/combat.ts:76)
#[derive(Clone, Debug)]
pub struct FireOpts {
    pub spread: Option<f64>,
    pub weapon: Option<crate::weapons::WeaponDef>,
    pub steady: Option<bool>,
    pub blind: Option<bool>,
    pub pellet: Option<crate::shotgun::Shell>,
}
impl FireOpts {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FireOpts {
            spread: None,
            weapon: None,
            steady: None,
            blind: None,
            pellet: None,
        }
    }
}
impl Default for FireOpts { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/combat.ts:324 acrossArm ->, src/game/combat.ts:324)
#[derive(Clone, Copy, Debug)]
pub struct AcrossArm {
    pub index: f64,
    pub at: V3,
}
impl AcrossArm {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AcrossArm {
            index: 0.0,
            at: [0.0; 3],
        }
    }
}
impl Default for AcrossArm { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum KillGuardZone {
    Side,
    Leg,
}
impl KillGuardZone {
    pub const fn as_str(self) -> &'static str {
        match self {
            KillGuardZone::Side => "side",
            KillGuardZone::Leg => "leg",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum DropFromHandKind {
    Pistol,
    Rifle,
    Torch,
}
impl DropFromHandKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            DropFromHandKind::Pistol => "pistol",
            DropFromHandKind::Rifle => "rifle",
            DropFromHandKind::Torch => "torch",
        }
    }
}

/// (TypeScript: inline at src/game/combat.ts:504 hammerSide ->, src/game/combat.ts:504)
#[derive(Clone, Copy, Debug)]
pub struct HammerSide {
    pub o: V3,
    pub z: V3,
}
impl HammerSide {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        HammerSide {
            o: [0.0; 3],
            z: [0.0; 3],
        }
    }
}
impl Default for HammerSide { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/combat.ts:602 laserReach ->, src/game/combat.ts:602)
#[derive(Clone, Copy, Debug)]
pub struct LaserReach {
    pub from: V3,
    pub dir: V3,
    pub l: f64,
    pub dot: bool,
    pub on: Option<CharId>,
}
impl LaserReach {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LaserReach {
            from: [0.0; 3],
            dir: [0.0; 3],
            l: 0.0,
            dot: false,
            on: None,
        }
    }
}
impl Default for LaserReach { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/combat.ts:684 detonateStun(sam), src/game/combat.ts:684)
#[derive(Clone, Copy, Debug)]
pub struct DetonateStunSam {
    pub side: Option<crate::teams::Team>,
    pub spare: Option<bool>,
}
impl DetonateStunSam {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DetonateStunSam {
            side: None,
            spare: None,
        }
    }
}
impl Default for DetonateStunSam { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/combat.ts:685 detonateStunRaw(sam), src/game/combat.ts:685)
#[derive(Clone, Copy, Debug)]
pub struct DetonateStunRawSam {
    pub side: Option<crate::teams::Team>,
    pub spare: Option<bool>,
}
impl DetonateStunRawSam {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DetonateStunRawSam {
            side: None,
            spare: None,
        }
    }
}
impl Default for DetonateStunRawSam { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct CombatMod {
    /// (src/game/combat.ts:99 strayGun)
    pub stray_gun: Option<FirearmId>,
}
impl CombatMod {
    pub fn blank() -> Self {
        CombatMod {
            stray_gun: None,
        }
    }
}

/// (src/game/combat.ts:358 solidScratch: a table nothing writes)
pub fn solid_scratch() -> Vec<i32> { alloc::vec![0; 64] }

/// (src/game/combat.ts:426 GUN_ITEM_LIFE)
pub const GUN_ITEM_LIFE: f64 = 240.0;

/// (src/game/combat.ts:453 handOrd: a table nothing writes)
pub fn hand_ord() -> Vec<u16> { alloc::vec![0; 256] }
