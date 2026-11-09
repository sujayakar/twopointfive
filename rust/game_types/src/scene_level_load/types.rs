//! The types of `scene_level_load`.
#![allow(unused_imports)]
use crate::{Blank, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct SceneLevelLoadMod {
    /// (tools/levels/lib/levelLoad.ts:47 levelNames)
    pub level_names: Option<String>,
}
