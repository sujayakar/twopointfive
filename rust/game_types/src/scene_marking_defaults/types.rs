//! The types of `scene_marking_defaults`.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, Default)]
pub struct P {
    pub steel: crate::scene_markings::MarkPal,
}

/// (tools/levels/lib/markingDefaults.ts:11 T)
pub const T: f64 = 0.025;

/// (tools/levels/lib/markingDefaults.ts:31 P: a table nothing writes)
pub fn p() -> crate::scene_marking_defaults::P { crate::scene_marking_defaults::P { steel: crate::scene_markings::MarkPal { albedo: Some([0.5, 0.5, 0.48]), mul: None, rough: Some(0.32), emit: None } } }
