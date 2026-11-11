//! The types of `shard_model`.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, Default)]
pub struct SHARD {
    pub gravity: f64,
}

pub fn shard() -> crate::shard_model::SHARD { crate::shard_model::SHARD { gravity: 9.8 } }

