//! The types of `roomfit`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Fit {
    pub at: V3,
    pub pos: V3,
    pub watch: Option<V3>,
    pub face: f64,
    pub words: String,
    pub key: Option<String>,
    pub w: f64,
    pub front: f64,
    pub on: bool,
    pub next_t: f64,
    pub sig: String,
}

#[derive(Clone, Debug, Default)]
pub struct Threat {
    pub at: V3,
    pub w: f64,
    pub way: Option<crate::roomread::RoomWay>,
    pub key: String,
    pub words: String,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Watchable {
    pub at: V3,
    pub w: f64,
    pub key: String,
    pub yaw: f64,
    pub way: bool,
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct RoomfitStats {
    pub deals: i32,
    pub way: i32,
    pub sector: i32,
    pub plain: i32,
    pub moved: i32,
}

pub const EYE: f64 = 1.55;

pub const CHEST: f64 = 1.1;

pub const DEG: f64 = 0.017453292519943295;

