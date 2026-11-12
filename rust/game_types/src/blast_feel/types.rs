//! The types of `blast_feel`.
#![allow(unused_imports)]
use crate::{Blank, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, Default)]
pub struct Face {
    pub o: V3,
    pub d: V3,
    pub mask: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct Kick {
    pub t0: f64,
    pub at: V3,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct State {
    pub seen: Vec<WallChargeId>,
    pub kicks: Vec<crate::blast_feel::Kick>,
    pub serial: f64,
    pub last: f64,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct BlastFeelMod {
    pub state: JsMap<PlayerId, crate::blast_feel::State>,
}

