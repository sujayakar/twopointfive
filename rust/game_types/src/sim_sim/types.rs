//! The types of `sim_sim`.
#![allow(unused_imports)]
use crate::{Blank, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, Default)]
pub struct SimExports {
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, Blank)]
pub struct SimSimMod {
    #[is(true)] pub dice_on: bool,
}

pub const SIM_ABI: f64 = 34.0;

