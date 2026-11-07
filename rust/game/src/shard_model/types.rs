//! The types of `shard_model`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: ShardState, src/game/shardModel.ts:54)
#[derive(Clone, Copy, Debug)]
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
impl ShardState {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ShardState {
            p: [0.0; 3],
            v: [0.0; 3],
            n: [0.0; 3],
            w: [0.0; 3],
            size: 0.0,
            asleep: false,
            age: 0.0,
            touches: 0.0,
            seed: 0.0,
        }
    }
}
impl Default for ShardState { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: ShardSpray, src/game/shardModel.ts:138)
#[derive(Clone, Copy, Debug)]
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
impl ShardSpray {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ShardSpray {
            pos: [0.0; 3],
            dir: [0.0; 3],
            count: 0.0,
            speed: 0.0,
            speed_jit: 0.0,
            spread: 0.0,
            size: 0.0,
            size_jit: 0.0,
            tumble: 0.0,
            pos_jit: 0.0,
            vadd: None,
            seed: 0.0,
        }
    }
}
impl Default for ShardSpray { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/shardModel.ts:28 SHARD, src/game/shardModel.ts:28)
#[derive(Clone, Copy, Debug)]
pub struct SHARD {
    pub gravity: f64,
    pub drag: f64,
    pub restitution: f64,
    pub keep: f64,
    pub sleep_v: f64,
    pub fade: f64,
    pub rough: f64,
    pub scatter: f64,
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
    pub glint_cap: f64,
    pub f0: f64,
    pub life: f64,
}
impl SHARD {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SHARD {
            gravity: 0.0,
            drag: 0.0,
            restitution: 0.0,
            keep: 0.0,
            sleep_v: 0.0,
            fade: 0.0,
            rough: 0.0,
            scatter: 0.0,
            ref_size: 0.0,
            drag_max: 0.0,
            ang_damp: 0.0,
            e_jit: 0.0,
            skitter: 0.0,
            spin_k: 0.0,
            spin_max: 0.0,
            wall_e: 0.0,
            wall_keep: 0.0,
            rest_tilt: 0.0,
            glint_cap: 0.0,
            f0: 0.0,
            life: 0.0,
        }
    }
}
impl Default for SHARD { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/shardModel.ts:112 flyShard(opts), src/game/shardModel.ts:112)
#[derive(Clone, Copy, Debug)]
pub struct FlyShardOpts {
    pub dt: Option<f64>,
    pub max_t: Option<f64>,
    pub floor_y: Option<f64>,
    pub every: Option<f64>,
    pub wall: Option<crate::shard_model::FlyShardOptsWall>,
}
impl FlyShardOpts {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FlyShardOpts {
            dt: None,
            max_t: None,
            floor_y: None,
            every: None,
            wall: None,
        }
    }
}
impl Default for FlyShardOpts { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/shardModel.ts:112 FlyShardOpts.wall, src/game/shardModel.ts:112)
#[derive(Clone, Copy, Debug)]
pub struct FlyShardOptsWall {
    pub n: V3,
    pub d: f64,
}
impl FlyShardOptsWall {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FlyShardOptsWall {
            n: [0.0; 3],
            d: 0.0,
        }
    }
}
impl Default for FlyShardOptsWall { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/shardModel.ts:112 flyShard ->, src/game/shardModel.ts:112)
#[derive(Clone, Debug)]
pub struct FlyShard {
    pub path: Vec<V3>,
    pub rest_t: f64,
    pub hops: f64,
    pub max_h: f64,
    pub reach: f64,
}
impl FlyShard {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FlyShard {
            path: Vec::new(),
            rest_t: 0.0,
            hops: 0.0,
            max_h: 0.0,
            reach: 0.0,
        }
    }
}
impl Default for FlyShard { fn default() -> Self { Self::zeroed() } }

/// (src/game/shardModel.ts:28 SHARD: a table nothing writes)
pub fn shard() -> crate::shard_model::SHARD { crate::shard_model::SHARD { gravity: 9.8, drag: 0.5, restitution: 0.4, keep: 0.78, sleep_v: 0.3, fade: 5.0, rough: 0.07, scatter: 0.1, ref_size: 0.01, drag_max: 6.0, ang_damp: 0.3, e_jit: 0.8, skitter: 0.45, spin_k: 28.0, spin_max: 60.0, wall_e: 0.3, wall_keep: 0.8, rest_tilt: 0.35, glint_cap: 14.0, f0: 0.04, life: 60.0 } }

/// (src/game/shardModel.ts:52 SHARD_IMP: a table nothing writes)
pub fn shard_imp() -> crate::sheet_model::ImpulseResponse { crate::sheet_model::ImpulseResponse { gain: 0.45, up_k: 0.5, v_max: 4.5, wake: 0.25 } }
