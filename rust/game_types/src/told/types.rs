//! The types of `told`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum ToldRung {
    #[default] Bare,
    Guided,
    Standard,
}
impl ToldRung {
    pub const fn as_str(self) -> &'static str {
        match self {
            ToldRung::Bare => "bare",
            ToldRung::Guided => "guided",
            ToldRung::Standard => "standard",
        }
    }
}

