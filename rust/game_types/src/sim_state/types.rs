//! The types of `sim_state`.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, Default)]
pub struct Layout {
    pub words: f64,
    pub games: f64,
    pub word: f64,
    pub off: Vec<f64>,
    pub len: Vec<f64>,
    pub pool: Vec<bool>,
}

#[derive(Clone, Debug, Default)]
pub struct SimTenant {
    pub name: String,
    pub bit: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SimStateModCLOCK {
    pub n: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SimStateModSIMSTATETALLY {
    pub parked: f64,
    pub bound: f64,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct SimStateMod {
    /// (src/sim/state.ts:53 L)
    pub l: Option<crate::sim_state::Layout>,
    /// (src/sim/state.ts:55 ROOM)
    pub room: Vec<f64>,
    /// (src/sim/state.ts:100 TENANTS)
    pub tenants: Vec<crate::sim_state::SimTenant>,
    /// (src/sim/state.ts:108 HELD)
    pub held: Vec<Option<crate::Opaque>>,
    /// (src/sim/state.ts:109 CLOCK)
    pub clock: crate::sim_state::SimStateModCLOCK,
    /// (src/sim/state.ts:111 SIM_STATE_TALLY)
    pub sim_state_tally: crate::sim_state::SimStateModSIMSTATETALLY,
}
impl SimStateMod {
    pub fn blank() -> Self {
        SimStateMod {
            l: None,
            room: Vec::new(),
            tenants: Vec::new(),
            held: Vec::new(),
            clock: crate::sim_state::SimStateModCLOCK { n: 0.0 },
            sim_state_tally: crate::sim_state::SimStateModSIMSTATETALLY { parked: 0.0, bound: 0.0 },
        }
    }
}

/// (src/sim/state.ts:39 SIM_PARTS: a table nothing writes)
pub fn sim_parts() -> [String; 13] { [String::from("head"), String::from("dice"), String::from("doors"), String::from("falls"), String::from("props"), String::from("slabs"), String::from("items"), String::from("men"), String::from("lights"), String::from("weather"), String::from("smoke"), String::from("poses"), String::from("panes")] }

/// (src/sim/state.ts:40 P_DOORS)
pub const P_DOORS: f64 = 2.0;

/// (src/sim/state.ts:40 P_FALLS)
pub const P_FALLS: f64 = 3.0;

/// (src/sim/state.ts:40 P_PROPS)
pub const P_PROPS: f64 = 4.0;

/// (src/sim/state.ts:40 P_SLABS)
pub const P_SLABS: f64 = 5.0;

/// (src/sim/state.ts:40 P_ITEMS)
pub const P_ITEMS: f64 = 6.0;

/// (src/sim/state.ts:40 P_LIGHTS)
pub const P_LIGHTS: f64 = 8.0;

/// (src/sim/state.ts:40 P_WEATHER)
pub const P_WEATHER: f64 = 9.0;

/// (src/sim/state.ts:40 P_SMOKE)
pub const P_SMOKE: f64 = 10.0;

/// (src/sim/state.ts:40 P_POSES)
pub const P_POSES: f64 = 11.0;

/// (src/sim/state.ts:40 P_PANES)
pub const P_PANES: f64 = 12.0;

/// (src/sim/state.ts:42 H_LAYOUT)
pub const H_LAYOUT: f64 = 0.0;

/// (src/sim/state.ts:42 H_HOMES)
pub const H_HOMES: f64 = 1.0;

/// (src/sim/state.ts:45 PIECE)
pub const PIECE: f64 = 1024.0;

/// (src/sim/state.ts:48 NO_BLOCK: a table nothing writes)
pub fn no_block() -> Vec<Vec<u32>> { alloc::vec![] }

/// (src/sim/state.ts:49 NO_WORDS: a table nothing writes)
pub fn no_words() -> Vec<u32> { alloc::vec![0; 0] }

/// (src/sim/state.ts:49 NO_DOUBLES: a table nothing writes)
pub fn no_doubles() -> Vec<f64> { alloc::vec![0.0; 0] }
