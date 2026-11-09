//! The types of `dress`.
#![allow(unused_imports)]
use crate::{Blank, bytes::ToBytes, ids::*, js::*, Quat, V3};
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
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct DressMod {
    #[is(crate::dress::DressModDRESS { moths: true, cues: true, blast_feel: true, fixtures: true, fx: true, stature: true, net: true, wind: true })] pub dress: crate::dress::DressModDRESS,
}
