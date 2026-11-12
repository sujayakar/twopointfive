//! The types of `lightswitch`.
use crate::{Blank, bytes::ToBytes, ids::*, V3};
use alloc::vec::Vec;

#[derive(Clone, Debug, ToBytes, Default)]
pub struct WallSwitch {
    pub id: f64,
    pub pos: V3,
    pub n: V3,
    pub stand: V3,
    pub floor: V3,
    pub door: DoorId,
    pub targets: Vec<FixtureId>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Hand {
    pub c: CharId,
    pub sw: crate::lightswitch::WallSwitch,
    pub at: f64,
    pub dir: f64,
    pub dur: f64,
    pub plant: bool,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct St {
    pub all: Vec<crate::lightswitch::WallSwitch>,
    pub list: Vec<crate::lightswitch::WallSwitch>,
    pub hands: Vec<crate::lightswitch::Hand>,
    pub serial: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Plate {
    pub door: DoorId,
    pub n: V3,
    pub pos: V3,
    pub floor: V3,
    pub stand: V3,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SwitchHand {
    pub p: V3,
    pub n: V3,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct LightswitchMod {
    pub states: Option<crate::lightswitch::St>,
}

pub const WALL_TOP: f64 = 2.2;

pub const SW_KEY: f64 = 4194304.0;
