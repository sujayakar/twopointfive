//! The types of `squadsmoke`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, ToBytes, Default)]
pub struct State {
    pub brought: f64,
    pub cans: f64,
    pub last: JsMap<GuardId, f64>,
}

#[derive(Clone, Debug, Default)]
pub struct SQUADSMOKE {
    pub order_key: String,
    pub speed: f64,
    pub near_m: f64,
    pub floor_m: f64,
    pub clear_h: Vec<f64>,
    pub roll_k: f64,
    pub hand_ahead: f64,
    pub hand_up: f64,
    pub head_drop: f64,
    pub again_secs: f64,
}

#[derive(Clone, Debug, Default)]
pub struct SquadArcTo {
    pub from: V3,
    pub sol: crate::throwables::ThrowSolution,
    pub rest: V3,
}

#[derive(Clone, Debug, Default)]
pub struct SquadSmokeThrower {
    pub gd: GuardId,
    pub from: V3,
    pub sol: crate::throwables::ThrowSolution,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes)]
pub struct SquadsmokeMod {
    pub states: JsMap<PlayerId, crate::squadsmoke::State>,
}
impl SquadsmokeMod {
    pub fn blank() -> Self {
        SquadsmokeMod {
            states: JsMap::new(),
        }
    }
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct SquadsmokeStats {
    pub staged: i32,
    pub thrown: i32,
}

pub fn squadsmoke() -> crate::squadsmoke::SQUADSMOKE { crate::squadsmoke::SQUADSMOKE { order_key: String::from("KeyH"), speed: 9.0, near_m: 1.0, floor_m: 0.035, clear_h: alloc::vec![0.07, 0.3], roll_k: 0.061, hand_ahead: 0.35, hand_up: 1.3, head_drop: 0.3, again_secs: 1.2 } }
