//! The types of `tracers`.
use crate::{Blank, bytes::ToBytes, ids::*, V3};
use alloc::vec::Vec;

#[derive(Clone, Debug, ToBytes, Blank)]
pub struct Tracers {
    pub pool: Vec<crate::tracers::Tracer>,
    pub live: f64,
    pub lit: f64,
    pub spawned: f64,
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

pub const LAMP_TTL: f64 = 30.0;
