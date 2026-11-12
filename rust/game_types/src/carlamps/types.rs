//! The types of `carlamps`.
use crate::{Words, bytes::ToBytes};

/// a vehicle's lamp, as the level's file says it: which box its lens is
#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct VehicleLamp {
    pub box_: f64,
    pub what: crate::carlamps::CarLamp,
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct CarlampsStats {
    pub found: i32,
}

pub const VEHICLE_GROUP: &str = "vehicle";

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default, Words)]
pub enum CarLamp {
    #[word("head lamp")] #[default] HeadLamp,
    #[word("tail lamp")] TailLamp,
}
