//! The types of `scene_marking_defaults`.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, Default)]
pub struct P {
    pub tape: crate::scene_markings::MarkPal,
    pub ink: crate::scene_markings::MarkPal,
    pub label: crate::scene_markings::MarkPal,
    pub grey: crate::scene_markings::MarkPal,
    pub steel: crate::scene_markings::MarkPal,
}

/// (tools/levels/lib/markingDefaults.ts:11 T)
pub const T: f64 = 0.025;

/// (tools/levels/lib/markingDefaults.ts:31 P: a table nothing writes)
pub fn p() -> crate::scene_marking_defaults::P { crate::scene_marking_defaults::P { tape: crate::scene_markings::MarkPal { albedo: Some([0.36, 0.22, 0.1]), mul: None, rough: Some(0.35), emit: None }, ink: crate::scene_markings::MarkPal { albedo: Some([0.16, 0.12, 0.08]), mul: None, rough: None, emit: None }, label: crate::scene_markings::MarkPal { albedo: Some([0.86, 0.85, 0.8]), mul: None, rough: None, emit: None }, grey: crate::scene_markings::MarkPal { albedo: Some([0.42, 0.42, 0.41]), mul: None, rough: None, emit: None }, steel: crate::scene_markings::MarkPal { albedo: Some([0.5, 0.5, 0.48]), mul: None, rough: Some(0.32), emit: None } } }
