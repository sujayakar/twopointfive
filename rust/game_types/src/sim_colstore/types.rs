//! The types of `sim_colstore`.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, Default)]
pub struct ColsOf2 {
    pub bytes: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ColDiffO {
    pub bytes: f64,
}
