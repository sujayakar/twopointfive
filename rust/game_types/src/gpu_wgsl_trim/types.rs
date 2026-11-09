//! The types of `gpu_wgsl_trim`.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, Default)]
pub struct Decl {
    pub text: String,
    pub name: Option<String>,
    pub root: bool,
    pub entry: bool,
    pub fn_: bool,
    pub ids: Vec<String>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SHADERTRIM {
    pub on: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SHADERFOLD {
    pub on: bool,
}

#[derive(Clone, Debug, Default)]
pub struct GpuWgslTrimModFoldedValue {
    pub own: JsMap<String, bool>,
    pub by: JsMap<String, crate::gpu_wgsl_trim::Decl>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct GpuWgslTrimModTrimTotals {
    pub modules: f64,
    pub before: f64,
    pub after: f64,
    pub ms: f64,
}

#[derive(Clone, Debug, Default)]
pub struct ChainAt {
    pub arms: Vec<crate::gpu_wgsl_trim::ChainAtArmsItem>,
    pub last: Option<String>,
    pub end: f64,
}

#[derive(Clone, Debug, Default)]
pub struct ChainAtArmsItem {
    pub test: String,
    pub block: String,
}

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
