//! The types of `digest`.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug)]
pub enum P3P {
    List(V3),
    Array(Vec<f64>),
}
