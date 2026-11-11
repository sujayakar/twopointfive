//! The types of `stalk`.
#![allow(unused_imports)]
use crate::{Blank, Words, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Words)]
pub enum Why {
    Keys,
    Wall,
    Let,
    Him,
    Turned,
    Hands,
    Far,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct St {
    pub on: Option<GuardId>,
    pub grab: Option<GuardId>,
    pub t: f64,
    pub wait: Option<bool>,
    pub armed: Option<bool>,
    pub orbit_r: Option<f64>,
    pub orbit_t: Option<f64>,
    pub orbit_ask: Option<[f64; 4]>,
    pub broke: Option<crate::stalk::StBroke>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct StBroke {
    pub gd: GuardId,
    pub t: f64,
    pub at: V3,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct StalkMove {
    pub mx: f64,
    pub mz: f64,
    pub pace: f64,
    pub at: V3,
    pub keep: Option<bool>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct StalkKeys {
    pub mx: f64,
    pub mz: f64,
    pub dt: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct StalkTell {
    pub gd: GuardId,
    pub at: V3,
    pub reach: f64,
    pub rise: f64,
    pub low: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct StalkWait {
    pub at: V3,
    pub reach: f64,
    pub coil: f64,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct StalkMod {
    pub state: JsMap<PlayerId, crate::stalk::St>,
}

