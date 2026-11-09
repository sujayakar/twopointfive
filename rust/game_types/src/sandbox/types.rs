//! The types of `sandbox`.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, Default)]
pub struct SandboxFlags {
    pub god_mode: bool,
    pub ai_enabled: bool,
    pub infinite_ammo: bool,
}

/// (src/game/sandbox.ts:6 SANDBOX_LS)
pub const SANDBOX_LS: &str = "twopointsix.sandbox";
