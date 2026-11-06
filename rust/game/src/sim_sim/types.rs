//! The types of `sim_sim`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: SimExports, src/sim/sim.ts:24)
#[derive(Clone, Copy, Debug)]
pub struct SimExports {
    pub memory: crate::Opaque,
}
impl SimExports {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SimExports {
            memory: crate::Opaque,
        }
    }
}
impl Default for SimExports { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/sim/sim.ts:321 RUST, src/sim/sim.ts:321)
#[derive(Clone, Copy, Debug)]
pub struct SimSimModRUST {
}
impl SimSimModRUST {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SimSimModRUST {
        }
    }
}
impl Default for SimSimModRUST { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/sim/sim.ts:355 FELL, src/sim/sim.ts:355)
#[derive(Clone, Debug)]
pub struct SimSimModFELL {
    pub on: bool,
    pub hard: bool,
    pub who: f64,
    pub n: JsMap<String, f64>,
    pub red: bool,
    pub reds: JsMap<String, f64>,
}
impl SimSimModFELL {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SimSimModFELL {
            on: false,
            hard: false,
            who: 0.0,
            n: JsMap::new(),
            red: false,
            reds: JsMap::new(),
        }
    }
}
impl Default for SimSimModFELL { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: a union)
#[derive(Clone, Copy, Debug)]
pub enum ArrayBufferLike {
    ArrayBuffer(crate::Opaque),
    SharedArrayBuffer(crate::Opaque),
}

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct SimSimMod {
    /// (src/sim/sim.ts:193 X)
    pub x: Option<crate::sim_sim::SimExports>,
    /// (src/sim/sim.ts:193 why)
    pub why: Option<String>,
    /// (src/sim/sim.ts:195 simBootMs)
    pub sim_boot_ms: f64,
    /// (src/sim/sim.ts:233 F64)
    pub f64: Option<Vec<f64>>,
    /// (src/sim/sim.ts:233 U8)
    pub u8: Option<Vec<u8>>,
    /// (src/sim/sim.ts:233 U32)
    pub u32: Option<Vec<u32>>,
    /// (src/sim/sim.ts:233 I32)
    pub i32: Option<Vec<i32>>,
    /// (src/sim/sim.ts:233 F32)
    pub f32: Option<Vec<f32>>,
    /// (src/sim/sim.ts:247 rustMaths)
    pub rust_maths: Option<crate::math_exact::Maths>,
    /// (src/sim/sim.ts:268 mathsOn)
    pub maths_on: bool,
    /// (src/sim/sim.ts:268 linesOn)
    pub lines_on: bool,
    /// (src/sim/sim.ts:268 circlesOn)
    pub circles_on: bool,
    /// (src/sim/sim.ts:268 posesOn)
    pub poses_on: bool,
    /// (src/sim/sim.ts:268 diceOn)
    pub dice_on: bool,
    /// (src/sim/sim.ts:269 doorsOn)
    pub doors_on: bool,
    /// (src/sim/sim.ts:269 fallsOn)
    pub falls_on: bool,
    /// (src/sim/sim.ts:269 propsOn)
    pub props_on: bool,
    /// (src/sim/sim.ts:269 slabsOn)
    pub slabs_on: bool,
    /// (src/sim/sim.ts:269 itemsOn)
    pub items_on: bool,
    /// (src/sim/sim.ts:269 raysOn)
    pub rays_on: bool,
    /// (src/sim/sim.ts:321 RUST)
    pub rust: crate::sim_sim::SimSimModRUST,
    /// (src/sim/sim.ts:355 FELL)
    pub fell: crate::sim_sim::SimSimModFELL,
}
impl SimSimMod {
    pub fn blank() -> Self {
        SimSimMod {
            x: None,
            why: None,
            sim_boot_ms: 0.0,
            f64: None,
            u8: None,
            u32: None,
            i32: None,
            f32: None,
            rust_maths: None,
            maths_on: false,
            lines_on: true,
            circles_on: true,
            poses_on: true,
            dice_on: true,
            doors_on: true,
            falls_on: true,
            props_on: true,
            slabs_on: true,
            items_on: true,
            rays_on: true,
            rust: crate::sim_sim::SimSimModRUST::zeroed(),
            fell: crate::sim_sim::SimSimModFELL { on: false, hard: false, who: 0.0, n: JsMap::new(), red: false, reds: JsMap::new() },
        }
    }
}

/// (src/sim/sim.ts:21 SIM_ABI)
pub const SIM_ABI: f64 = 34.0;

/// (src/sim/sim.ts:373 RULES_ROW)
pub const RULES_ROW: &str = "a rule's step (doors, furniture, leaves, thrown things)";

/// (src/sim/sim.ts:399 OWN_MATHS: a table nothing writes)
pub fn own_maths() -> Vec<String> { alloc::vec![String::from("maths"), String::from("poses"), String::from("doors"), String::from("props"), String::from("slabs"), String::from("items")] }
