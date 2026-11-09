//! The types of `env`.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, Default)]
pub struct BUILT {
    pub exposure: f64,
    pub tube_gain: f64,
}

/// (src/game/env.ts:18 BUILT: a table nothing writes)
pub fn built() -> crate::env::BUILT { crate::env::BUILT { exposure: 2.0, tube_gain: 475.0 } }
