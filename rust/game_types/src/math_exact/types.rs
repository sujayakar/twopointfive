//! The types of `math_exact`.
#![allow(unused_imports)]
use crate::{Blank, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// the state this module keeps at its top level
#[derive(Clone, Debug, Blank)]
pub struct MathExactMod {
    /// (src/math/exact.ts:314 mathsEpoch)
    pub maths_epoch: f64,
}

