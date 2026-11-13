//! The types of `disarm`.
use crate::{bytes::ToBytes, ids::*, V3};

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct Strip {
    pub t: f64,
    pub kind: crate::slotsetup::SetupGun,
    pub supp: bool,
    pub at: V3,
    pub item: Option<ItemId>,
    pub rap: bool,
    pub gone: bool,
    pub yaw: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct DISARMGAIN {
    pub catch: f64,
    pub strip: f64,
    pub down: f64,
    pub long: f64,
}

pub fn disarm_gain() -> crate::disarm::DISARMGAIN { crate::disarm::DISARMGAIN { catch: 0.5, strip: 0.45, down: 0.4, long: 0.8 } }

pub const LIFE: f64 = 240.0;

pub const DEG: f64 = 0.017453292519943295;
