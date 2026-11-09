//! The types of `math_side`.
#![allow(unused_imports)]
use crate::{Blank, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, Default)]
pub struct MathSideModPUTBACKS {
    pub n: f64,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, Blank)]
pub struct MathSideMod {
    /// (src/math/side.ts:134 PUT_BACKS)
    #[is(crate::math_side::MathSideModPUTBACKS { n: 0.0 })] pub put_backs: crate::math_side::MathSideModPUTBACKS,
}

