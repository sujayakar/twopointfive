//! The types of `scene_shapes`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct ShapeFormArc {
    pub axis: f64,
    pub rise: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct ShapePiece {
    pub off: V3,
    pub h: V3,
    pub yaw: Option<f64>,
    pub albedo: Option<V3>,
    pub rough: Option<f64>,
    pub metal: Option<bool>,
    pub stuff: Option<crate::scene_stuff::BoxStuff>,
    pub does: Option<crate::scene_materials::DoesWord>,
    pub round: Option<f64>,
    pub arc: Option<crate::scene_shapes::ShapeFormArc>,
    pub cut: Option<Vec<[f64; 4]>>,
    pub old: Option<bool>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct BoxShape {
    pub with: Option<Vec<crate::scene_shapes::ShapePiece>>,
    pub round: Option<f64>,
    pub arc: Option<crate::scene_shapes::ShapeFormArc>,
    pub cut: Option<Vec<[f64; 4]>>,
    pub old: Option<bool>,
}
