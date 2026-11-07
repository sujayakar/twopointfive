//! The types of `kitload`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: KitLoad, src/game/kitload.ts:46)
#[derive(Clone, Copy, Debug)]
pub struct KitLoad {
    pub mob: f64,
    pub con: f64,
}
impl KitLoad {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        KitLoad {
            mob: 0.0,
            con: 0.0,
        }
    }
}
impl Default for KitLoad { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: KitWorn, src/game/kitload.ts:48)
#[derive(Clone, Copy, Debug)]
pub struct KitWorn {
    pub ref_mob: f64,
    pub ref_con: f64,
    pub mob: f64,
    pub con: f64,
}
impl KitWorn {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        KitWorn {
            ref_mob: 0.0,
            ref_con: 0.0,
            mob: 0.0,
            con: 0.0,
        }
    }
}
impl Default for KitWorn { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: a union)
#[derive(Clone, Copy, Debug)]
pub enum LoadOnWho {
    Guard(GuardId),
    Player,
}

/// (TypeScript: a union)
#[derive(Clone, Copy, Debug)]
pub enum LoadPaceOnWho {
    Guard(GuardId),
    Player,
}

/// (TypeScript: a union)
#[derive(Clone, Copy, Debug)]
pub enum LoadGaitWho {
    Guard(GuardId),
    Player,
}

/// (TypeScript: a union)
#[derive(Clone, Copy, Debug)]
pub enum LoadSeenWho {
    Guard(GuardId),
    Player,
}

/// (TypeScript: a union)
#[derive(Clone, Copy, Debug)]
pub enum LoadHeardWho {
    Guard(GuardId),
    Player,
}

/// (TypeScript: a union)
#[derive(Clone, Copy, Debug)]
pub enum LoadCrossWho {
    Guard(GuardId),
    Player,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct KitloadMod {
    /// (src/game/kitload.ts:125 manLoad)
    pub man_load: JsMap<GuardId, crate::kitload::KitWorn>,
}
impl KitloadMod {
    pub fn blank() -> Self {
        KitloadMod {
            man_load: JsMap::new(),
        }
    }
}

/// (src/game/kitload.ts:76 BREAK)
pub const BREAK: bool = false;
