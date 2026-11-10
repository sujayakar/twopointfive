//! The types of `cast`.
#![allow(unused_imports)]
use crate::{Blank, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct CastMod {
    pub worn: JsMap<CharId, AnimGltfCastVariantId>,
    pub has_nvg: JsMap<AnimGltfCastVariantId, bool>,
}

pub fn field_parts() -> Vec<String> { alloc::vec![String::from("hat_straw"), String::from("hat_boonie"), String::from("bandana"), String::from("bandolier"), String::from("tank_top")] }
