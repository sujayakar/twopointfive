//! The types of `inhand`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, ToBytes)]
pub enum InHandWord {
    Pistol,
    Torch(crate::inhand::InHandWordTorch),
    Long(crate::inhand::InHandWordLong),
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct InHandWordTorch {
    pub beam: V3,
    pub at: V3,
    pub axis: V3,
    pub fwd: f64,
    pub palm: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct InHandWordLong {
    pub by: V3,
}

#[derive(Clone, Debug, Default)]
pub struct BELTTORCH {
    pub was: Vec<f64>,
    pub at: Vec<f64>,
}

pub fn in_pistol() -> crate::inhand::InHandWord { crate::inhand::InHandWord::Pistol }

pub fn belt_torch() -> crate::inhand::BELTTORCH { crate::inhand::BELTTORCH { was: alloc::vec![0.15, 0.115], at: alloc::vec![0.21, 0.16] } }
