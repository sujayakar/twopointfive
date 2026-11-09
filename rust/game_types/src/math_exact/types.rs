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
pub struct SYS {
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

/// (src/math/exact.ts:308 SYS: a table nothing writes)
pub fn sys() -> crate::math_exact::SYS { crate::math_exact::SYS { sin: crate::FnField, cos: crate::FnField, tan: crate::FnField, asin: crate::FnField, acos: crate::FnField, atan: crate::FnField, atan2: crate::FnField, exp: crate::FnField, log: crate::FnField, log2: crate::FnField, log10: crate::FnField, pow: crate::FnField, tanh: crate::FnField, hypot: crate::FnField } }

/// (src/math/exact.ts:309 OWN: a table nothing writes)
pub fn own() -> crate::math_exact::SYS { crate::math_exact::SYS { sin: crate::FnField, cos: crate::FnField, tan: crate::FnField, asin: crate::FnField, acos: crate::FnField, atan: crate::FnField, atan2: crate::FnField, exp: crate::FnField, log: crate::FnField, log2: crate::FnField, log10: crate::FnField, pow: crate::FnField, tanh: crate::FnField, hypot: crate::FnField } }
