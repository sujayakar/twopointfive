//! The types of `startle`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct St {
    pub calm_t: f64,
    pub calm_torch: bool,
    pub by_sight: bool,
    pub at: f64,
    pub until: f64,
    pub toward: V3,
    pub pre: bool,
    pub torch: bool,
    pub choose: bool,
    pub call: bool,
    pub draw_until: f64,
    pub run: Option<V3>,
    pub run_at: f64,
    pub ref_: Option<V3>,
    pub ref_t: f64,
    pub script: Option<crate::game::GuardScript>,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes)]
pub struct StartleMod {
    /// (src/game/startle.ts:64 stateOf)
    pub state_of: JsMap<GuardId, crate::startle::St>,
}
impl StartleMod {
    pub fn blank() -> Self {
        StartleMod {
            state_of: JsMap::new(),
        }
    }
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct StartleStats {
    pub seen: i32,
    pub given: i32,
    pub drew: i32,
    pub ran: i32,
}
