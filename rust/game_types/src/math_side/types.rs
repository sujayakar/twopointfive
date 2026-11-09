//! The types of `math_side`.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, Default)]
pub struct MathSideModPUTBACKS {
    pub n: f64,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct MathSideMod {
    pub put_backs: crate::math_side::MathSideModPUTBACKS,
}
impl MathSideMod {
    pub fn blank() -> Self {
        MathSideMod {
            put_backs: crate::math_side::MathSideModPUTBACKS { n: 0.0 },
        }
    }
}

