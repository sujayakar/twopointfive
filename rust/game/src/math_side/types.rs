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
    pub get: crate::FnField,
    pub set: crate::FnField,
}
impl SideVar {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SideVar {
            name: String::new(),
            get: crate::FnField,
            set: crate::FnField,
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

/// (TypeScript: inline at src/math/side.ts:114 ROOTS, src/math/side.ts:114)
#[derive(Clone, Debug)]
pub struct MathSideModROOTSItem {
    pub name: String,
}
impl MathSideModROOTSItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MathSideModROOTSItem {
            name: String::new(),
        }
    }
}
impl Default for MathSideModROOTSItem { fn default() -> Self { Self::zeroed() } }

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

/// (TypeScript: inline at src/math/side.ts:124 wallClock(keys), src/math/side.ts:124 wallClock(keys))
#[derive(Clone, Copy, Debug)]
pub struct WallClockKeys {
    pub length: f64,
}
impl WallClockKeys {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WallClockKeys {
            length: 0.0,
        }
    }
}
impl Default for WallClockKeys { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/math/side.ts:137 sideRegistry ->, src/math/side.ts:137)
#[derive(Clone, Debug)]
pub struct SideRegistry {
    pub tables: Vec<crate::math_side::Table>,
    pub vars: Vec<crate::math_side::SideVar>,
    pub roots: Vec<crate::math_side::SideRegistryRootsItem>,
}
impl SideRegistry {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SideRegistry {
            tables: Vec::new(),
            vars: Vec::new(),
            roots: Vec::new(),
        }
    }
}
impl Default for SideRegistry { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/math/side.ts:137 SideRegistry.roots, src/math/side.ts:137)
#[derive(Clone, Debug)]
pub struct SideRegistryRootsItem {
    pub name: String,
}
impl SideRegistryRootsItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SideRegistryRootsItem {
            name: String::new(),
        }
    }
}
impl Default for SideRegistryRootsItem { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct MathSideMod {
    /// (src/math/side.ts:15 NAMED)
    pub named: Vec<crate::math_side::Table>,
    /// (src/math/side.ts:16 memos)
    pub memos: f64,
    /// (src/math/side.ts:17 NAMES)
    pub names: JsSet<String>,
    /// (src/math/side.ts:108 VARS)
    pub vars: Vec<crate::math_side::SideVar>,
    /// (src/math/side.ts:114 ROOTS)
    pub roots: Vec<crate::math_side::MathSideModROOTSItem>,
    /// (src/math/side.ts:123 CLOCKED)
    pub clocked: JsMap<crate::ObjKey, Vec<String>>,
    /// (src/math/side.ts:134 PUT_BACKS)
    pub put_backs: crate::math_side::MathSideModPUTBACKS,
    /// (src/math/side.ts:144 sorted)
    pub sorted: f64,
    /// (src/math/side.ts:144 sortedMemos)
    pub sorted_memos: f64,
    /// (src/math/side.ts:144 STATE)
    pub state: Vec<crate::math_side::Table>,
}
impl MathSideMod {
    pub fn blank() -> Self {
        MathSideMod {
            named: Vec::new(),
            memos: 0.0,
            names: JsSet::new(),
            vars: Vec::new(),
            roots: Vec::new(),
            clocked: JsMap::new(),
            put_backs: crate::math_side::MathSideModPUTBACKS { n: 0.0 },
            sorted: 0.0,
            sorted_memos: 0.0,
            state: Vec::new(),
        }
    }
}

/// (src/math/side.ts:75 RAW: a table nothing writes)
pub fn raw() -> crate::Opaque { crate::Opaque }

/// (src/math/side.ts:77 CAST_VIEW: a table nothing writes)
pub fn cast_view() -> crate::Opaque { crate::Opaque }
