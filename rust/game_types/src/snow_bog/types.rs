//! The types of `snow_bog`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, Default)]
pub struct Ground {
    pub kinds: Vec<u8>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SNOWKIND {
    pub bare: f64,
    pub packed: f64,
    pub old: f64,
    pub bank: f64,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes)]
pub struct SnowBogMod {
    /// (scratch, read by nobody who did not just write it: in no bytes)
    #[bytes(skip)]
    pub ground_of: Option<crate::snow_bog::Ground>,
}
impl SnowBogMod {
    pub fn blank() -> Self {
        SnowBogMod {
            ground_of: None,
        }
    }
}

pub fn snow_kind() -> crate::snow_bog::SNOWKIND { crate::snow_bog::SNOWKIND { bare: 1.0, packed: 2.0, old: 3.0, bank: 4.0 } }
