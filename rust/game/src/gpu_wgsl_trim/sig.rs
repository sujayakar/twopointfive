#![allow(unused_variables)]
//! Every function and method of `gpu_wgsl_trim`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/gpu/wgslTrim.ts:50 read)
pub fn read(g: &mut crate::game::Game, text: &str) -> crate::gpu_wgsl_trim::Decl { todo!() }

/// (src/gpu/wgslTrim.ts:66 wgslDecls)
pub fn wgsl_decls(code: &str) -> Vec<String> { todo!() }

/// (src/gpu/wgslTrim.ts:89 foldTest)
pub fn fold_test(expr: &str, val: &mut dyn FnMut(String) -> Option<bool>) -> Option<bool> { todo!() }

/// (src/gpu/wgslTrim.ts:110 boolConsts)
pub fn bool_consts(decls: &[String]) -> JsMap<String, bool> { todo!() }

/// (src/gpu/wgslTrim.ts:117 closing)
pub fn closing(t: &str, open: f64) -> f64 { todo!() }

/// (src/gpu/wgslTrim.ts:118 blank)
pub fn blank(t: &str, i: f64) -> f64 { todo!() }

/// (src/gpu/wgslTrim.ts:121 chainAt)
pub fn chain_at(t: &str, at: f64) -> Option<crate::gpu_wgsl_trim::ChainAt> { todo!() }

/// (src/gpu/wgslTrim.ts:138 stmtEnd)
pub fn stmt_end(t: &str, i: f64) -> f64 { todo!() }

/// (src/gpu/wgslTrim.ts:149 leaves)
pub fn leaves(st: &str) -> bool { todo!() }

/// (src/gpu/wgslTrim.ts:155 leavesAt)
pub fn leaves_at(block: &str) -> f64 { todo!() }

/// (src/gpu/wgslTrim.ts:161 cutTails)
pub fn cut_tails(t: &str) -> String { todo!() }

/// (src/gpu/wgslTrim.ts:173 foldFn)
pub fn fold_fn(text: &str, val: &mut dyn FnMut(String) -> Option<bool>, used: Option<&mut JsSet<String>>) -> String { todo!() }

/// (src/gpu/wgslTrim.ts:197 foldOne)
pub fn fold_one(g: &mut crate::game::Game, d: &crate::gpu_wgsl_trim::Decl, val: &JsMap<String, bool>) -> crate::gpu_wgsl_trim::Decl { todo!() }

/// (src/gpu/wgslTrim.ts:219 trimWgsl)
pub fn trim_wgsl(g: &mut crate::game::Game, code: &str, fold: Option<bool>) -> String { todo!() }

/// (src/gpu/wgslTrim.ts:235 shaderText)
pub fn shader_text(g: &mut crate::game::Game, code: &str) -> String { todo!() }
