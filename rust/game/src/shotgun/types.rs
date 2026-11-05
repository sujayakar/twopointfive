//! The types of `shotgun`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// a man this shell puts down, settled once the whole pattern has flown: combat.ts fireWeaponRaw's two blocks, run by fire_shell
#[derive(Clone, Debug)]
pub enum ShellLate {
    ManDown { hit_guard: GuardId, sg: Option<GuardId>, shooter: CharId, is_player: bool, ff_ours: bool, hostage_hit: bool, muzzle: V3, dir: V3, t_hit: f64, hit_p: V3 },
    SamHit { hit_view: PlayerId, sg: Option<GuardId>, dir: V3, hit_p: V3 },
}

/// (TypeScript: Shell, src/game/shotgun.ts:61)
#[derive(Clone, Debug)]
pub struct Shell {
    pub def: crate::weapons::WeaponDef,
    pub i: f64,
    pub dir: Option<V3>,
    pub men: JsMap<GuardId, crate::shotgun::ShellMenValue>,
    pub ends: Vec<crate::shotgun::ShellEndsItem>,
    pub glass: JsMap<crate::Num, Vec<f64>>,
    pub late: Vec<crate::shotgun::ShellLate>,
}
impl Shell {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Shell {
            def: crate::weapons::WeaponDef::zeroed(),
            i: 0.0,
            dir: None,
            men: JsMap::new(),
            ends: Vec::new(),
            glass: JsMap::new(),
            late: Vec::new(),
        }
    }
}
impl Default for Shell { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/shotgun.ts:64 Shell.men, src/game/shotgun.ts:64)
#[derive(Clone, Copy, Debug)]
pub struct ShellMenValue {
    pub n: f64,
    pub near: f64,
    pub dir: V3,
    pub down: bool,
}
impl ShellMenValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ShellMenValue {
            n: 0.0,
            near: 0.0,
            dir: [0.0; 3],
            down: false,
        }
    }
}
impl Default for ShellMenValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/shotgun.ts:65 Shell.ends, src/game/shotgun.ts:65)
#[derive(Clone, Copy, Debug)]
pub struct ShellEndsItem {
    pub index: f64,
    pub t: f64,
    pub at: V3,
}
impl ShellEndsItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ShellEndsItem {
            index: 0.0,
            t: 0.0,
            at: [0.0; 3],
        }
    }
}
impl Default for ShellEndsItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/shotgun.ts:38 SHOTGUN_LAST, src/game/shotgun.ts:38)
#[derive(Clone, Debug)]
pub struct ShotgunModSHOTGUNLAST {
    pub ends: Vec<crate::shotgun::ShellEndsItem>,
    pub glass: JsMap<crate::Num, Vec<f64>>,
}
impl ShotgunModSHOTGUNLAST {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ShotgunModSHOTGUNLAST {
            ends: Vec::new(),
            glass: JsMap::new(),
        }
    }
}
impl Default for ShotgunModSHOTGUNLAST { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum BreachSpotClose {
    Shotgun,
    Hammer,
}
impl BreachSpotClose {
    pub const fn as_str(self) -> &'static str {
        match self {
            BreachSpotClose::Shotgun => "shotgun",
            BreachSpotClose::Hammer => "hammer",
        }
    }
}

/// (TypeScript: inline at src/game/shotgun.ts:136 breachSpot ->, src/game/shotgun.ts:136)
#[derive(Clone, Copy, Debug)]
pub struct BreachSpot {
    pub pos: V3,
    pub yaw: f64,
    pub at: V3,
    pub beside: bool,
    pub own: bool,
}
impl BreachSpot {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BreachSpot {
            pos: [0.0; 3],
            yaw: 0.0,
            at: [0.0; 3],
            beside: false,
            own: false,
        }
    }
}
impl Default for BreachSpot { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct ShotgunMod {
    /// (src/game/shotgun.ts:38 SHOTGUN_LAST)
    pub shotgun_last: crate::shotgun::ShotgunModSHOTGUNLAST,
}
impl ShotgunMod {
    pub fn blank() -> Self {
        ShotgunMod {
            shotgun_last: crate::shotgun::ShotgunModSHOTGUNLAST { ends: Vec::new(), glass: JsMap::new() },
        }
    }
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct ShotgunStats {
    pub shells: i32,
    pub pellets: i32,
    pub in_men: i32,
    pub dropped: i32,
    pub peppered: i32,
    pub locks: i32,
}

/// (src/game/shotgun.ts:126 LOCK_PROUD)
pub const LOCK_PROUD: f64 = 0.06;
