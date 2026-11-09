//! The types of `shard_model`.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, Default)]
pub struct ShardState {
    pub p: V3,
    pub v: V3,
    pub n: V3,
    pub w: V3,
    pub size: f64,
    pub asleep: bool,
    pub age: f64,
    pub touches: f64,
    pub seed: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ShardSpray {
    pub pos: V3,
    pub dir: V3,
    pub count: f64,
    pub speed: f64,
    pub speed_jit: f64,
    pub spread: f64,
    pub size: f64,
    pub size_jit: f64,
    pub tumble: f64,
    pub pos_jit: f64,
    pub vadd: Option<V3>,
    pub seed: f64,
}

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

#[derive(Clone, Copy, Debug, Default)]
pub struct FlyShardOpts {
    pub dt: Option<f64>,
    pub max_t: Option<f64>,
    pub floor_y: Option<f64>,
    pub every: Option<f64>,
    pub wall: Option<crate::shard_model::FlyShardOptsWall>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct FlyShardOptsWall {
    pub n: V3,
    pub d: f64,
}

#[derive(Clone, Debug, Default)]
pub struct FlyShard {
    pub path: Vec<V3>,
    pub rest_t: f64,
    pub hops: f64,
    pub max_h: f64,
    pub reach: f64,
}

/// (src/game/shardModel.ts:28 SHARD: a table nothing writes)
pub fn shard() -> crate::shard_model::SHARD { crate::shard_model::SHARD { gravity: 9.8, drag: 0.5, restitution: 0.4, keep: 0.78, sleep_v: 0.3, ref_size: 0.01, drag_max: 6.0, ang_damp: 0.3, e_jit: 0.8, skitter: 0.45, spin_k: 28.0, spin_max: 60.0, wall_e: 0.3, wall_keep: 0.8, rest_tilt: 0.35 } }

