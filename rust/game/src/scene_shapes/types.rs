//! The types of `scene_shapes`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: ShapeForm, src/scene/shapes.ts:73)
#[derive(Clone, Debug)]
pub struct ShapeForm {
    pub round: Option<f64>,
    pub arc: Option<crate::scene_shapes::ShapeFormArc>,
    pub cut: Option<Vec<[f64; 4]>>,
    pub old: Option<bool>,
}
impl ShapeForm {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ShapeForm {
            round: None,
            arc: None,
            cut: None,
            old: None,
        }
    }
}
impl Default for ShapeForm { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/shapes.ts:73 ShapeForm.arc, src/scene/shapes.ts:73)
#[derive(Clone, Copy, Debug)]
pub struct ShapeFormArc {
    pub axis: f64,
    pub rise: f64,
}
impl ShapeFormArc {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ShapeFormArc {
            axis: 0.0,
            rise: 0.0,
        }
    }
}
impl Default for ShapeFormArc { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: ShapePiece, src/scene/shapes.ts:75)
#[derive(Clone, Debug)]
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
impl ShapePiece {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ShapePiece {
            off: [0.0; 3],
            h: [0.0; 3],
            yaw: None,
            albedo: None,
            rough: None,
            metal: None,
            stuff: None,
            does: None,
            round: None,
            arc: None,
            cut: None,
            old: None,
        }
    }
}
impl Default for ShapePiece { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: BoxShape, src/scene/shapes.ts:77)
#[derive(Clone, Debug)]
pub struct BoxShape {
    pub with: Option<Vec<crate::scene_shapes::ShapePiece>>,
    pub round: Option<f64>,
    pub arc: Option<crate::scene_shapes::ShapeFormArc>,
    pub cut: Option<Vec<[f64; 4]>>,
    pub old: Option<bool>,
}
impl BoxShape {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BoxShape {
            with: None,
            round: None,
            arc: None,
            cut: None,
            old: None,
        }
    }
}
impl Default for BoxShape { fn default() -> Self { Self::zeroed() } }
