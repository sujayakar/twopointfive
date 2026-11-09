//! The types of `sim_sim`.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, Default)]
pub struct SimExports {
}

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct SimSimMod {
    pub dice_on: bool,
}
impl SimSimMod {
    pub fn blank() -> Self {
        SimSimMod {
            dice_on: true,
        }
    }
}

pub const SIM_ABI: f64 = 34.0;

