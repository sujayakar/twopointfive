//! The types of `aimcost`.
use crate::V3;

#[derive(Clone, Copy, Debug, Default)]
pub struct RoundLeg {
    pub o: V3,
    pub d: V3,
    pub end: f64,
}

