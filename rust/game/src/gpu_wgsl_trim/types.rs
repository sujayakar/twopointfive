//! The types of `gpu_wgsl_trim`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: Decl, src/gpu/wgslTrim.ts:41)
#[derive(Clone, Debug)]
pub struct Decl {
    pub text: String,
    pub name: Option<String>,
    pub root: bool,
    pub entry: bool,
    pub fn_: bool,
    pub ids: Vec<String>,
}
impl Decl {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Decl {
            text: String::new(),
            name: None,
            root: false,
            entry: false,
            fn_: false,
            ids: Vec::new(),
        }
    }
}
impl Default for Decl { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/gpu/wgslTrim.ts:16 SHADER_TRIM, src/gpu/wgslTrim.ts:16)
#[derive(Clone, Copy, Debug)]
pub struct SHADERTRIM {
    pub on: bool,
}
impl SHADERTRIM {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SHADERTRIM {
            on: false,
        }
    }
}
impl Default for SHADERTRIM { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/gpu/wgslTrim.ts:38 SHADER_FOLD, src/gpu/wgslTrim.ts:38)
#[derive(Clone, Copy, Debug)]
pub struct SHADERFOLD {
    pub on: bool,
}
impl SHADERFOLD {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SHADERFOLD {
            on: false,
        }
    }
}
impl Default for SHADERFOLD { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/gpu/wgslTrim.ts:195 folded, src/gpu/wgslTrim.ts:195)
#[derive(Clone, Debug)]
pub struct GpuWgslTrimModFoldedValue {
    pub own: JsMap<String, bool>,
    pub by: JsMap<String, crate::gpu_wgsl_trim::Decl>,
}
impl GpuWgslTrimModFoldedValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GpuWgslTrimModFoldedValue {
            own: JsMap::new(),
            by: JsMap::new(),
        }
    }
}
impl Default for GpuWgslTrimModFoldedValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/gpu/wgslTrim.ts:233 trimTotals, src/gpu/wgslTrim.ts:233)
#[derive(Clone, Copy, Debug)]
pub struct GpuWgslTrimModTrimTotals {
    pub modules: f64,
    pub before: f64,
    pub after: f64,
    pub ms: f64,
}
impl GpuWgslTrimModTrimTotals {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GpuWgslTrimModTrimTotals {
            modules: 0.0,
            before: 0.0,
            after: 0.0,
            ms: 0.0,
        }
    }
}
impl Default for GpuWgslTrimModTrimTotals { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/gpu/wgslTrim.ts:121 chainAt ->, src/gpu/wgslTrim.ts:121)
#[derive(Clone, Debug)]
pub struct ChainAt {
    pub arms: Vec<crate::gpu_wgsl_trim::ChainAtArmsItem>,
    pub last: Option<String>,
    pub end: f64,
}
impl ChainAt {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ChainAt {
            arms: Vec::new(),
            last: None,
            end: 0.0,
        }
    }
}
impl Default for ChainAt { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/gpu/wgslTrim.ts:121 ChainAt.arms, src/gpu/wgslTrim.ts:121)
#[derive(Clone, Debug)]
pub struct ChainAtArmsItem {
    pub test: String,
    pub block: String,
}
impl ChainAtArmsItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ChainAtArmsItem {
            test: String::new(),
            block: String::new(),
        }
    }
}
impl Default for ChainAtArmsItem { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct GpuWgslTrimMod {
    /// (src/gpu/wgslTrim.ts:44 seen)
    pub seen: JsMap<String, crate::gpu_wgsl_trim::Decl>,
    /// (src/gpu/wgslTrim.ts:47 NAMED)
    pub named: JsSet<String>,
    /// (src/gpu/wgslTrim.ts:48 STAGE)
    pub stage: JsSet<String>,
    /// (src/gpu/wgslTrim.ts:195 folded)
    pub folded: JsMap<String, crate::gpu_wgsl_trim::GpuWgslTrimModFoldedValue>,
    /// (src/gpu/wgslTrim.ts:233 trimTotals)
    pub trim_totals: crate::gpu_wgsl_trim::GpuWgslTrimModTrimTotals,
}
impl GpuWgslTrimMod {
    pub fn blank() -> Self {
        GpuWgslTrimMod {
            seen: JsMap::new(),
            named: { let mut s = JsSet::new(); for w in ["fn", "struct", "alias", "const", "override", "var", "let"] { s.add(String::from(w)); } s },
            stage: { let mut s = JsSet::new(); for w in ["compute", "vertex", "fragment"] { s.add(String::from(w)); } s },
            folded: JsMap::new(),
            trim_totals: crate::gpu_wgsl_trim::GpuWgslTrimModTrimTotals { modules: 0.0, before: 0.0, after: 0.0, ms: 0.0 },
        }
    }
}

/// (src/gpu/wgslTrim.ts:16 SHADER_TRIM: a table nothing writes)
pub fn shader_trim() -> crate::gpu_wgsl_trim::SHADERTRIM { crate::gpu_wgsl_trim::SHADERTRIM { on: true } }

/// (src/gpu/wgslTrim.ts:38 SHADER_FOLD: a table nothing writes)
pub fn shader_fold() -> crate::gpu_wgsl_trim::SHADERFOLD { crate::gpu_wgsl_trim::SHADERFOLD { on: true } }

/// (src/gpu/wgslTrim.ts:46 WORD)
pub const WORD: &str = "/[A-Za-z_][A-Za-z0-9_]*/g";
