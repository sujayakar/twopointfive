//! The types of `squadsmoke`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: State, src/game/squadsmoke.ts:56)
#[derive(Clone, Debug)]
pub struct State {
    pub brought: f64,
    pub cans: f64,
    pub noted: bool,
    pub last: JsMap<GuardId, f64>,
}
impl State {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        State {
            brought: 0.0,
            cans: 0.0,
            noted: false,
            last: JsMap::new(),
        }
    }
}
impl Default for State { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/squadsmoke.ts:39 SQUADSMOKE, src/game/squadsmoke.ts:39)
#[derive(Clone, Debug)]
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
impl SQUADSMOKE {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SQUADSMOKE {
            order_key: String::new(),
            speed: 0.0,
            near_m: 0.0,
            floor_m: 0.0,
            clear_h: Vec::new(),
            roll_k: 0.0,
            hand_ahead: 0.0,
            hand_up: 0.0,
            head_drop: 0.0,
            again_secs: 0.0,
        }
    }
}
impl Default for SQUADSMOKE { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/squadsmoke.ts:60 SQUADSMOKE_LOG, src/game/squadsmoke.ts:60)
#[derive(Clone, Debug)]
pub struct SquadsmokeModSQUADSMOKELOGItem {
    pub t: f64,
    pub who: String,
    pub from: V3,
    pub at: V3,
    pub land: V3,
    pub rest: V3,
    pub left: f64,
}
impl SquadsmokeModSQUADSMOKELOGItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SquadsmokeModSQUADSMOKELOGItem {
            t: 0.0,
            who: String::new(),
            from: [0.0; 3],
            at: [0.0; 3],
            land: [0.0; 3],
            rest: [0.0; 3],
            left: 0.0,
        }
    }
}
impl Default for SquadsmokeModSQUADSMOKELOGItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/squadsmoke.ts:101 squadArcTo ->, src/game/squadsmoke.ts:101)
#[derive(Clone, Debug)]
pub struct SquadArcTo {
    pub from: V3,
    pub sol: crate::throwables::ThrowSolution,
    pub rest: V3,
}
impl SquadArcTo {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SquadArcTo {
            from: [0.0; 3],
            sol: crate::throwables::ThrowSolution::zeroed(),
            rest: [0.0; 3],
        }
    }
}
impl Default for SquadArcTo { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/squadsmoke.ts:120 squadSmokeThrower ->, src/game/squadsmoke.ts:120)
#[derive(Clone, Debug)]
pub struct SquadSmokeThrower {
    pub gd: GuardId,
    pub from: V3,
    pub sol: crate::throwables::ThrowSolution,
    pub rest: V3,
}
impl SquadSmokeThrower {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SquadSmokeThrower {
            gd: GuardId(0),
            from: [0.0; 3],
            sol: crate::throwables::ThrowSolution::zeroed(),
            rest: [0.0; 3],
        }
    }
}
impl Default for SquadSmokeThrower { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct SquadsmokeMod {
    /// (src/game/squadsmoke.ts:57 states)
    pub states: JsMap<PlayerId, crate::squadsmoke::State>,
    /// (src/game/squadsmoke.ts:60 SQUADSMOKE_LOG)
    pub squadsmoke_log: Vec<crate::squadsmoke::SquadsmokeModSQUADSMOKELOGItem>,
}
impl SquadsmokeMod {
    pub fn blank() -> Self {
        SquadsmokeMod {
            states: JsMap::new(),
            squadsmoke_log: Vec::new(),
        }
    }
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct SquadsmokeStats {
    pub staged: i32,
    pub thrown: i32,
}

/// (src/game/squadsmoke.ts:39 SQUADSMOKE: a table nothing writes)
pub fn squadsmoke() -> crate::squadsmoke::SQUADSMOKE { crate::squadsmoke::SQUADSMOKE { order_key: String::from("KeyH"), speed: 9.0, near_m: 1.0, floor_m: 0.035, clear_h: alloc::vec![0.07, 0.3], roll_k: 0.061, hand_ahead: 0.35, hand_up: 1.3, head_drop: 0.3, again_secs: 1.2 } }

/// (src/game/squadsmoke.ts:55 SMOKE_ORDER)
pub const SMOKE_ORDER: &str = "smokeThere";
