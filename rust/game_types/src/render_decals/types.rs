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

