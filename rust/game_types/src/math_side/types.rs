//! The types of `math_side`.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug)]
pub enum Table {
    SideMap(JsMap<crate::ObjKey, crate::Opaque>),
    SideSet(JsSet<crate::ObjKey>),
}

#[derive(Clone, Debug)]
pub struct Keys {
    pub refs: Vec<crate::Opaque>,
    pub listed: JsSet<crate::ObjKey>,
    pub sweep_at: f64,
}
impl Keys {
    /// what a new one holds
    pub fn blank() -> Self {
        Keys {
            refs: Vec::new(),
            listed: JsSet::new(),
            sweep_at: 64.0,
        }
    }
}
impl Default for Keys { fn default() -> Self { Self::blank() } }

#[derive(Clone, Debug, Default)]
pub struct SideVar {
    pub name: String,
}

#[derive(Clone, Copy, Debug)]
pub struct HostOwned {
}
impl HostOwned {
    /// what a new one holds
    pub fn blank() -> Self {
        HostOwned {
        }
    }
}
impl Default for HostOwned { fn default() -> Self { Self::blank() } }

#[derive(Clone, Copy, Debug, Default)]
pub struct MathSideModPUTBACKS {
    pub n: f64,
}

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
