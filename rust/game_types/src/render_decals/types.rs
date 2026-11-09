//! The types of `render_decals`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum DecalKind {
    #[default] Bullet,
    Scorch,
    Scuff,
    Chip,
    GlassHit,
    Wetprint,
    Damage,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct Decal {
    pub kind: crate::render_decals::DecalKind,
    pub pos: V3,
    pub n: V3,
    pub t: V3,
    pub radius: f64,
    pub param: f64,
    pub seed: f64,
    pub serial: f64,
    pub pane: f64,
    pub box_: Option<BoxId>,
    pub local: Option<crate::render_decals::DecalLocal>,
    pub look: Option<bool>,
    pub box_idx: f64,
    pub hidden: bool,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct DecalLocal {
    pub p: V3,
    pub n: V3,
    pub t: V3,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct DecalOpts {
    pub tangent: Option<V3>,
    pub param: Option<f64>,
    pub look: Option<bool>,
    pub attach: Option<BoxId>,
    pub clamp_to: Option<BoxId>,
    pub seed: Option<f64>,
    pub pane: Option<f64>,
}

#[derive(Clone, Debug)]
pub struct Decals {
    pub list: Vec<crate::render_decals::Decal>,
    pub enabled: bool,
    pub buf: crate::Opaque,
    pub dirty: bool,
    pub serial: f64,
    pub device: crate::Opaque,
}
impl Decals {
    /// what a new one holds
    pub fn blank() -> Self {
        Decals {
            list: Vec::new(),
            enabled: true,
            buf: crate::Opaque,
            dirty: true,
            serial: 0.0,
            device: crate::Opaque,
        }
    }
}
impl Default for Decals { fn default() -> Self { Self::blank() } }

#[derive(Clone, Copy, Debug, Default)]
pub struct WEARMARK {
    pub look: f64,
    pub materials: f64,
    pub reach0: f64,
    pub reach1: f64,
    pub min_half: f64,
    pub max_half: f64,
    pub face_tol: f64,
}

/// (src/render/decals.ts:96 WEAR_MARK: a table nothing writes)
pub fn wear_mark() -> crate::render_decals::WEARMARK { crate::render_decals::WEARMARK { look: 3.0, materials: 3.0, reach0: 0.09, reach1: 0.21, min_half: 0.035, max_half: 0.09, face_tol: 0.015 } }

