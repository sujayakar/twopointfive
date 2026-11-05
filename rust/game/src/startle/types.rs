//! The types of `startle`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: StartleSnap, src/game/startle.ts:155)
#[derive(Clone, Debug)]
pub struct StartleSnap {
    pub men: Vec<Option<Vec<f64>>>,
}
impl StartleSnap {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StartleSnap {
            men: Vec::new(),
        }
    }
}
impl Default for StartleSnap { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: St, src/game/startle.ts:44)
#[derive(Clone, Copy, Debug)]
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
impl St {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        St {
            calm_t: 0.0,
            calm_torch: false,
            by_sight: false,
            at: 0.0,
            until: 0.0,
            toward: [0.0; 3],
            pre: false,
            torch: false,
            choose: false,
            call: false,
            draw_until: 0.0,
            run: None,
            run_at: 0.0,
            ref_: None,
            ref_t: 0.0,
            script: None,
        }
    }
}
impl Default for St { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
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
