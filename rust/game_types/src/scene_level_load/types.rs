//! The types of `scene_level_load`.
use crate::{Blank, bytes::ToBytes};
use alloc::string::String;

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct SceneLevelLoadMod {
    pub level_names: Option<String>,
}
