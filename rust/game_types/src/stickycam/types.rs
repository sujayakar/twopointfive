//! The types of `stickycam`.
#![allow(unused_imports)]
use crate::{Blank, Words, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct StickyCam {
    pub pos: V3,
    pub n: V3,
    pub vel: Option<V3>,
    pub age: f64,
    pub si: f64,
    pub id: f64,
}

#[derive(Clone, Debug, Default)]
pub struct StickyLivePl {
    pub cams: Vec<StickyCamId>,
}

#[derive(Clone, Debug, Default)]
pub struct StickyRowsRowOver {
    pub kind: Option<crate::stickycam::StickyRowsRowOverKind>,
    pub pos: Option<V3>,
    pub line1: Option<String>,
    pub line2: Option<String>,
    pub in_reach: Option<bool>,
    pub off: Option<bool>,
    pub progress: Option<f64>,
    pub target: Option<FixtureId>,
    pub door: Option<DoorId>,
    pub guard: Option<GuardId>,
    pub verb: Option<crate::game::Verb>,
    pub running: Option<bool>,
    pub close: Option<crate::close::CloseKind>,
    pub past: Option<bool>,
    pub framed: Option<bool>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Words)]
pub enum StickyRowsRowOverKind {
    Cam,
    Guard,
    Breaker,
    Door,
    Light,
    Body,
    Objective,
    Held,
    Wall,
    Switch,
    Charge,
    Cancel,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct StickycamMod {
    pub visit: Option<f64>,
}

pub const EYE_ID: f64 = 200.0;

pub fn none() -> Vec<crate::fogofwar::FogEye> { Vec::new() }

pub fn none_cams() -> Vec<StickyCamId> { Vec::new() }

pub fn body() -> V3 { [0.05, 0.05, 0.055] }

pub fn lens() -> V3 { [0.012, 0.012, 0.014] }

pub fn lens_live() -> V3 { [0.42, 0.05, 0.04] }

pub const CAM_KEY: f64 = 8388608.0;

