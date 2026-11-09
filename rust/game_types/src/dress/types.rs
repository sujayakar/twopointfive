//! The types of `dress`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct DressModDRESS {
    pub moths: bool,
    pub cues: bool,
    pub blast_feel: bool,
    pub fixtures: bool,
    pub fx: bool,
    pub stature: bool,
    pub net: bool,
    pub wind: bool,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes)]
pub struct DressMod {
    /// (src/game/dress.ts:11 DRESS)
    pub dress: crate::dress::DressModDRESS,
}
impl DressMod {
    pub fn blank() -> Self {
        DressMod {
            dress: crate::dress::DressModDRESS { moths: true, cues: true, blast_feel: true, fixtures: true, fx: true, stature: true, net: true, wind: true },
        }
    }
}
