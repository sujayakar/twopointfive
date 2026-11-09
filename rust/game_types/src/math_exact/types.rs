//! The types of `math_exact`.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct MathExactMod {
    /// (src/math/exact.ts:314 mathsEpoch)
    pub maths_epoch: f64,
}
impl MathExactMod {
    pub fn blank() -> Self {
        MathExactMod {
            maths_epoch: 0.0,
        }
    }
}

