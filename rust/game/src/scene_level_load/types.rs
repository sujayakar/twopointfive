//! The types of `scene_level_load`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct SceneLevelLoadMod {
    /// (src/scene/levelLoad.ts:47 levelNames)
    pub level_names: JsMap<u32, String>,
}
impl SceneLevelLoadMod {
    pub fn blank() -> Self {
        SceneLevelLoadMod {
            level_names: JsMap::new(),
        }
    }
}
