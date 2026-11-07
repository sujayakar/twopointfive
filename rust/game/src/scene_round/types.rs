//! The types of `scene_round`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: inline at src/scene/round.ts:42 roundConfig, src/scene/round.ts:42)
#[derive(Clone, Copy, Debug)]
pub struct SceneRoundModRoundConfig {
    pub gpu: bool,
    pub exact: bool,
}
impl SceneRoundModRoundConfig {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SceneRoundModRoundConfig {
            gpu: false,
            exact: false,
        }
    }
}
impl Default for SceneRoundModRoundConfig { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct SceneRoundMod {
    /// (src/scene/round.ts:42 roundConfig)
    pub round_config: crate::scene_round::SceneRoundModRoundConfig,
}
impl SceneRoundMod {
    pub fn blank() -> Self {
        SceneRoundMod {
            round_config: crate::scene_round::SceneRoundModRoundConfig { gpu: false, exact: false },
        }
    }
}
