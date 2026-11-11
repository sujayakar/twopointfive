//! The types of `render_particles`.
#![allow(unused_imports)]
use crate::{Words, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// how an emit's particles are born; absent on a record reads as Cone (:471)
#[derive(Clone, Copy, Debug, Words)]
pub enum EmitShape {
    Cone,
    Burst,
    Ring,
    Puff,
    RainDisc,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Default, Words)]
pub enum PKind {
    #[default] Rain,
    Dust,
    Casing,
    Sheet,
    Spark,
    Debris,
    Shard,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct EmitOpts {
    pub shape: Option<crate::render_particles::EmitShape>,
    pub speed: f64,
    pub spread: Option<f64>,
    pub life: f64,
    pub size: f64,
    pub speed_jit: Option<f64>,
    pub life_jit: Option<f64>,
    pub size_jit: Option<f64>,
    pub lift: Option<f64>,
    pub radius: Option<f64>,
    pub pos_jit: Option<f64>,
    pub vadd: Option<V3>,
    pub owner: Option<f64>,
    pub color: Option<V3>,
    pub top: Option<f64>,
    pub aspect: Option<f64>,
    pub tumble: Option<f64>,
    pub nrm_jit: Option<f64>,
    pub seed: Option<f64>,
    pub steam: Option<f64>,
    pub ember: Option<f64>,
}

#[derive(Clone, Debug, Default)]
pub struct ImpulseOpts {
    pub up: Option<f64>,
    pub bias: Option<V3>,
    pub kinds: Option<Vec<crate::render_particles::PKind>>,
    pub occlude: Option<bool>,
    pub eye: Option<V3>,
    pub seed: Option<f64>,
}

pub const RAIN_MAX_BIRTHS: f64 = 640.0;

pub const MAX_NEW_PER_FRAME: f64 = 6144.0;

