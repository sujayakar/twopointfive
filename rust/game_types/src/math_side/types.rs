//! The types of `math_side`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: a union)
#[derive(Clone, Debug)]
pub enum Table {
    SideMap(JsMap<crate::ObjKey, crate::Opaque>),
    SideSet(JsSet<crate::ObjKey>),
}

/// (TypeScript: Keys, src/math/side.ts:24)
#[derive(Clone, Debug)]
pub struct Keys {
    pub refs: Vec<crate::Opaque>,
    pub listed: JsSet<crate::ObjKey>,
    pub sweep_at: f64,
}
impl Keys {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        Keys {
            refs: Vec::new(),
            listed: JsSet::new(),
            sweep_at: 64.0,
        }
    }
}
impl Default for Keys { fn default() -> Self { Self::blank() } }

/// (TypeScript: SideVar, src/math/side.ts:107)
#[derive(Clone, Debug)]
pub struct SideVar {
    pub name: String,
}
impl SideVar {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SideVar {
            name: String::new(),
        }
    }
}
impl Default for SideVar { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: HostOwned, src/math/side.ts:130)
#[derive(Clone, Copy, Debug)]
pub struct HostOwned {
}
impl HostOwned {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        HostOwned {
        }
    }
}
impl Default for HostOwned { fn default() -> Self { Self::blank() } }

/// (TypeScript: inline at src/math/side.ts:134 PUT_BACKS, src/math/side.ts:134)
#[derive(Clone, Copy, Debug)]
pub struct MathSideModPUTBACKS {
    pub n: f64,
}
impl MathSideModPUTBACKS {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MathSideModPUTBACKS {
            n: 0.0,
        }
    }
}
impl Default for MathSideModPUTBACKS { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct MathSideMod {
    /// (src/math/side.ts:134 PUT_BACKS)
    pub put_backs: crate::math_side::MathSideModPUTBACKS,
}
impl MathSideMod {
    pub fn blank() -> Self {
        MathSideMod {
            put_backs: crate::math_side::MathSideModPUTBACKS { n: 0.0 },
        }
    }
}

/// (src/math/side.ts:75 RAW: a table nothing writes)
pub fn raw() -> crate::Opaque { crate::Opaque }

/// (src/math/side.ts:77 CAST_VIEW: a table nothing writes)
pub fn cast_view() -> crate::Opaque { crate::Opaque }
