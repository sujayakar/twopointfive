//! The types of `tracers`.
#![allow(unused_imports)]
use crate::{Blank, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, ToBytes, Blank)]
pub struct Tracers {
    pub pool: Vec<crate::tracers::Tracer>,
    pub live: f64,
    pub lit: f64,
    pub spawned: f64,
    #[is(crate::tracers::TracersParams { speed: 280.0, min_life: 0.042, shutter: 0.011, thickness: 0.012, emissive: 60.0, min_path: 0.5, light_intensity: 4.0, light_range: 4.0, light_radius: 0.06, ember_sparks: 3.0, ember_speed: 5.0, ember_cone: 0.9, ember_life: 0.3, ember_intensity: 2.5, ember_range: 2.5, ember_ttl: 0.06, max_live: 24.0, max_lights: 6.0 })] pub params: crate::tracers::TracersParams,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct Tracer {
    pub alive: bool,
    pub from: V3,
    pub dir: V3,
    pub l: f64,
    pub s: f64,
    pub v: f64,
    pub age: f64,
    pub frames: f64,
    pub color: V3,
    pub owner: f64,
    pub impact: crate::tracers::TracerImpact,
    pub arrived: bool,
    pub light: Option<LightId>,
    pub box_: BoxId,
    pub lamp: LightId,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum TracerImpact {
    #[default] None,
    Solid,
    Flesh,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct TracersParams {
    pub speed: f64,
    pub min_life: f64,
    pub shutter: f64,
    pub thickness: f64,
    pub emissive: f64,
    pub min_path: f64,
    pub light_intensity: f64,
    pub light_range: f64,
    pub light_radius: f64,
    pub ember_sparks: f64,
    pub ember_speed: f64,
    pub ember_cone: f64,
    pub ember_life: f64,
    pub ember_intensity: f64,
    pub ember_range: f64,
    pub ember_ttl: f64,
    pub max_live: f64,
    pub max_lights: f64,
}

/// (src/game/tracers.ts:56 LAMP_TTL)
pub const LAMP_TTL: f64 = 30.0;
