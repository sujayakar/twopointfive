//! The types of `rescueend`.
#![allow(unused_imports)]
use crate::{Blank, Words, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, ToBytes, Blank)]
pub struct RescueEnd {
    pub lead: Option<CharId>,
    #[is(f64::NAN)] pub clear_at: f64,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum RescueEndWon {
    Out,
    Cleared,
}

#[derive(Clone, Debug, Default)]
pub struct RescueEndObjective {
    pub text: String,
    pub pos: Option<V3>,
}

