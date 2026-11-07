//! The types of `ui_shaderrows`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum ShaderRowKey {
    Compact,
    FirstFrame,
    SplitRays,
}
impl ShaderRowKey {
    pub const fn as_str(self) -> &'static str {
        match self {
            ShaderRowKey::Compact => "compact",
            ShaderRowKey::FirstFrame => "firstFrame",
            ShaderRowKey::SplitRays => "splitRays",
        }
    }
}

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct UiShaderrowsMod {
    /// (src/ui/shaderrows.ts:67 used)
    pub used: JsSet<crate::ui_shaderrows::ShaderRowKey>,
}
impl UiShaderrowsMod {
    pub fn blank() -> Self {
        UiShaderrowsMod {
            used: JsSet::new(),
        }
    }
}

/// (src/ui/shaderrows.ts:64 SHADER_ROWS: a table nothing writes)
pub fn shader_rows() -> JsMap<String, bool> { { let mut t = JsMap::new(); t.set(String::from("compact"), false); t.set(String::from("firstFrame"), false); t.set(String::from("splitRays"), false); t } }
