//! The types of `dress`.
use crate::{Blank, bytes::ToBytes};

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
