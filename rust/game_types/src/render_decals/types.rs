//! The types of `render_decals`.
#![allow(unused_imports)]
use crate::{Blank, bytes::ToBytes, ids::*, js::*, Quat, V3};
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

#[derive(Clone, Debug, Blank)]
pub struct Decals {
    #[is(true)] pub enabled: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct WEARMARK {
    pub look: f64,
    pub reach0: f64,
    pub reach1: f64,
    pub min_half: f64,
    pub max_half: f64,
}

/// (src/render/decals.ts:96 WEAR_MARK: a table nothing writes)
pub fn wear_mark() -> crate::render_decals::WEARMARK { crate::render_decals::WEARMARK { look: 3.0, reach0: 0.09, reach1: 0.21, min_half: 0.035, max_half: 0.09 } }

