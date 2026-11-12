//! The types of `silent`.
use crate::{Blank, bytes::ToBytes, ids::*, js::*};
use alloc::vec::Vec;

#[derive(Clone, Debug, ToBytes, Default)]
pub struct State {
    pub cast: Vec<GuardId>,
    pub silent: bool,
    pub since: f64,
    pub hot: bool,
    pub was: Option<bool>,
    pub booted: Option<JsSet<DoorId>>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct SilentModSeatsValue {
    pub on: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum SilentBoot {
    First,
    Again,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct SilentMod {
    pub states: JsMap<GroupId, crate::silent::State>,
    pub seats: JsMap<GroupId, crate::silent::SilentModSeatsValue>,
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct SilentStats {
    pub tools: i32,
}

pub fn targets() -> Vec<crate::orders::TargetKind> { alloc::vec![crate::orders::TargetKind::Floor, crate::orders::TargetKind::Door, crate::orders::TargetKind::None] }

pub fn listed() -> Option<Vec<crate::orders::TargetKind>> { Some(alloc::vec![crate::orders::TargetKind::Floor, crate::orders::TargetKind::None]) }

