//! The types of `shard_model`.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, Default)]
pub struct SHARD {
    pub gravity: f64,
    pub drag: f64,
    pub restitution: f64,
    pub keep: f64,
    pub sleep_v: f64,
    pub ref_size: f64,
    pub drag_max: f64,
    pub ang_damp: f64,
    pub e_jit: f64,
    pub skitter: f64,
    pub spin_k: f64,
    pub spin_max: f64,
    pub wall_e: f64,
    pub wall_keep: f64,
    pub rest_tilt: f64,
}

pub fn shard() -> crate::shard_model::SHARD { crate::shard_model::SHARD { gravity: 9.8, drag: 0.5, restitution: 0.4, keep: 0.78, sleep_v: 0.3, ref_size: 0.01, drag_max: 6.0, ang_damp: 0.3, e_jit: 0.8, skitter: 0.45, spin_k: 28.0, spin_max: 60.0, wall_e: 0.3, wall_keep: 0.8, rest_tilt: 0.35 } }

