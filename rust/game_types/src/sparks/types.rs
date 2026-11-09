//! The types of `sparks`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct SparkParams {
    pub gravity: f64,
    pub drag: f64,
    pub hold: f64,
    pub brightness: f64,
    pub max_live: f64,
    pub emissive: f64,
    pub thickness: f64,
    pub shutter: f64,
    pub min_streak: f64,
    pub collide: bool,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct Spark {
    pub pos: V3,
    pub vel: V3,
    pub age: f64,
    pub life: f64,
    pub alive: bool,
    pub gen_: f64,
}

#[derive(Clone, Debug, ToBytes)]
pub struct Sparks {
    pub pool: Vec<crate::sparks::Spark>,
    pub live: f64,
    pub params: crate::sparks::SparkParams,
}
impl Sparks {
    /// what a new one holds
    pub fn blank() -> Self {
        Sparks {
            pool: Vec::new(),
            live: 0.0,
            params: crate::sparks::SparkParams::default(),
        }
    }
}
impl Default for Sparks { fn default() -> Self { Self::blank() } }

#[derive(Clone, Copy, Debug, Default)]
pub struct SparksEmitBurstO {
    pub count: f64,
    pub speed: f64,
    pub spread: f64,
    pub lift: f64,
    pub life: f64,
}
