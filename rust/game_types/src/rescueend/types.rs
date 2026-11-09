//! The types of `rescueend`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, Words, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, ToBytes)]
pub struct RescueEnd {
    pub lead: Option<CharId>,
    pub led: bool,
    pub clear_at: f64,
}
impl RescueEnd {
    /// what a new one holds
    pub fn blank() -> Self {
        RescueEnd {
            lead: None,
            led: false,
            clear_at: f64::NAN,
        }
    }
}
impl Default for RescueEnd { fn default() -> Self { Self::blank() } }

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum RescueEndWon {
    Out,
    Cleared,
}

#[derive(Clone, Debug, Default)]
pub struct RescueEndObjective {
    pub text: String,
    pub pos: Option<V3>,
}

#[derive(Clone, Debug, Default)]
pub struct RescueEndRows2RowOver {
    pub kind: Option<crate::rescueend::RescueEndRows2RowOverKind>,
    pub pos: Option<V3>,
    pub line1: Option<String>,
    pub line2: Option<String>,
    pub in_reach: Option<bool>,
    pub off: Option<bool>,
    pub progress: Option<f64>,
    pub target: Option<FixtureId>,
    pub door: Option<DoorId>,
    pub guard: Option<GuardId>,
    pub verb: Option<crate::game::Verb>,
    pub running: Option<bool>,
    pub close: Option<crate::close::CloseKind>,
    pub past: Option<bool>,
    pub framed: Option<bool>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Words)]
pub enum RescueEndRows2RowOverKind {
    Cam,
    Guard,
    Breaker,
    Door,
    Light,
    Body,
    Objective,
    Held,
    Wall,
    Switch,
    Charge,
    Cancel,
}
