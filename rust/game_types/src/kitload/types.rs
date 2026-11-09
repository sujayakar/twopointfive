//! The types of `kitload`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, Default)]
pub struct KitLoad {
    pub mob: f64,
    pub con: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct KitWorn {
    pub ref_mob: f64,
    pub ref_con: f64,
    pub mob: f64,
    pub con: f64,
}

#[derive(Clone, Copy, Debug)]
pub enum LoadOnWho {
    Guard(GuardId),
    Player,
}

#[derive(Clone, Copy, Debug)]
pub enum LoadPaceOnWho {
    Guard(GuardId),
    Player,
}

#[derive(Clone, Copy, Debug)]
pub enum LoadGaitWho {
    Guard(GuardId),
    Player,
}

#[derive(Clone, Copy, Debug)]
pub enum LoadSeenWho {
    Guard(GuardId),
    Player,
}

#[derive(Clone, Copy, Debug)]
pub enum LoadHeardWho {
    Guard(GuardId),
    Player,
}

#[derive(Clone, Copy, Debug)]
pub enum LoadCrossWho {
    Guard(GuardId),
    Player,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes)]
pub struct KitloadMod {
    pub man_load: JsMap<GuardId, crate::kitload::KitWorn>,
}
impl KitloadMod {
    pub fn blank() -> Self {
        KitloadMod {
            man_load: JsMap::new(),
        }
    }
}

