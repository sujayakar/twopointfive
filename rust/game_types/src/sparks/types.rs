//! The types of `sparks`.
#![allow(unused_imports)]
use crate::{Blank, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct Spark {
    pub pos: V3,
    pub vel: V3,
    pub age: f64,
    pub life: f64,
    pub alive: bool,
    pub gen_: f64,
}

#[derive(Clone, Debug, ToBytes, Blank)]
pub struct Sparks {
    pub pool: Vec<crate::sparks::Spark>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SparksEmitBurstO {
    pub count: f64,
    pub speed: f64,
    pub spread: f64,
    pub lift: f64,
    pub life: f64,
}
