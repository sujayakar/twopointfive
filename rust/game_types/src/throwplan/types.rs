//! The types of `throwplan`.
use crate::V3;
use alloc::vec::Vec;

#[derive(Clone, Debug, Default)]
pub struct ThrowFlight {
    pub end: V3,
    pub land: V3,
    pub points: Vec<V3>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ThrowToLim {
    pub n: f64,
}

#[derive(Clone, Debug, Default)]
pub struct ThrowTo {
    pub from: V3,
    pub sol: crate::throwables::ThrowSolution,
    pub rest: V3,
}

