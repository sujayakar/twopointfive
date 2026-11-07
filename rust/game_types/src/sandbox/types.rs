//! The types of `sandbox`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: SandboxFlags, src/game/sandbox.ts:5)
#[derive(Clone, Copy, Debug)]
pub struct SandboxFlags {
    pub god_mode: bool,
    pub ai_enabled: bool,
    pub infinite_ammo: bool,
}
impl SandboxFlags {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SandboxFlags {
            god_mode: false,
            ai_enabled: false,
            infinite_ammo: false,
        }
    }
}
impl Default for SandboxFlags { fn default() -> Self { Self::zeroed() } }

/// (src/game/sandbox.ts:6 SANDBOX_LS)
pub const SANDBOX_LS: &str = "twopointsix.sandbox";
