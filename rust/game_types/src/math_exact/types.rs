//! The types of `math_exact`.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, Default)]
pub struct Maths {
    pub sin: crate::FnField,
    pub cos: crate::FnField,
    pub tan: crate::FnField,
    pub asin: crate::FnField,
    pub acos: crate::FnField,
    pub atan: crate::FnField,
    pub atan2: crate::FnField,
    pub exp: crate::FnField,
    pub log: crate::FnField,
    pub log2: crate::FnField,
    pub log10: crate::FnField,
    pub pow: crate::FnField,
    pub tanh: crate::FnField,
    pub hypot: crate::FnField,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct MathExactModEXACT {
}

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct MathExactMod {
    /// (src/math/exact.ts:312 installed)
    pub installed: bool,
    /// (src/math/exact.ts:314 mathsEpoch)
    pub maths_epoch: f64,
    /// (src/math/exact.ts:315 remade)
    pub remade: Vec<crate::FnField>,
    /// (src/math/exact.ts:322 other)
    pub other: Option<crate::math_exact::Maths>,
    /// (src/math/exact.ts:339 EXACT)
    pub exact: crate::math_exact::MathExactModEXACT,
}
impl MathExactMod {
    pub fn blank() -> Self {
        MathExactMod {
            installed: false,
            maths_epoch: 0.0,
            remade: Vec::new(),
            other: None,
            exact: crate::math_exact::MathExactModEXACT::default(),
        }
    }
}

