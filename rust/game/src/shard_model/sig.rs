#![allow(unused_variables)]
//! Every function and method of `shard_model`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/shardModel.ts:57 pcg)
pub fn pcg(v: f64) -> f64 { todo!() }

/// (src/game/shardModel.ts:58 rnd)
pub fn rnd(seed: f64, k: f64) -> f64 { todo!() }

/// (src/game/shardModel.ts:61 shardDrag)
pub fn shard_drag(size: f64, k: Option<crate::shard_model::SHARD>) -> f64 { todo!() }

/// (src/game/shardModel.ts:64 shardAir)
pub fn shard_air(s: &mut crate::shard_model::ShardState, dt: f64, k: Option<crate::shard_model::SHARD>) { todo!() }

/// (src/game/shardModel.ts:76 shardTouch)
pub fn shard_touch(s: &mut crate::shard_model::ShardState, nrm: V3, k: Option<crate::shard_model::SHARD>) -> bool { todo!() }

/// (src/game/shardModel.ts:112 flyShard)
pub fn fly_shard(s: &mut crate::shard_model::ShardState, opts: Option<crate::shard_model::FlyShardOpts>, k: Option<crate::shard_model::SHARD>) -> crate::shard_model::FlyShard { todo!() }

/// (src/game/shardModel.ts:129 makeShard)
pub fn make_shard(seed: f64, p: V3, dir: V3, speed: f64, spread: f64, size: f64, tumble: f64) -> crate::shard_model::ShardState { todo!() }

/// (src/game/shardModel.ts:141 sprayShards)
pub fn spray_shards(r: crate::shard_model::ShardSpray) -> Vec<crate::shard_model::ShardState> { todo!() }
