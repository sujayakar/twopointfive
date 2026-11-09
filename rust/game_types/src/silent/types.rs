//! The types of `silent`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum By {
    #[default] Start,
    Order,
    Tool,
    Api,
}

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
    pub go_digit: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum SilentBoot {
    First,
    Again,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes)]
pub struct SilentMod {
    /// (src/game/silent.ts:110 states)
    pub states: JsMap<PlayerId, crate::silent::State>,
    /// (src/game/silent.ts:233 seats)
    pub seats: JsMap<PlayerId, crate::silent::SilentModSeatsValue>,
}
impl SilentMod {
    pub fn blank() -> Self {
        SilentMod {
            states: JsMap::new(),
            seats: JsMap::new(),
        }
    }
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct SilentStats {
    pub silent: i32,
    pub loud: i32,
    pub locked: i32,
    pub picked: i32,
    pub booted: i32,
    pub held: i32,
    pub held_glass: i32,
    pub compromised: i32,
    pub tools: i32,
}

/// (src/game/silent.ts:222 TARGETS: a table nothing writes)
pub fn targets() -> Vec<crate::orders::TargetKind> { alloc::vec![crate::orders::TargetKind::Floor, crate::orders::TargetKind::Door, crate::orders::TargetKind::None] }

/// (src/game/silent.ts:222 LISTED: a table nothing writes)
pub fn listed() -> Option<Vec<crate::orders::TargetKind>> { Some(alloc::vec![crate::orders::TargetKind::Floor, crate::orders::TargetKind::None]) }

