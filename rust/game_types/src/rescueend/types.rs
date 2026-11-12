//! The types of `rescueend`.
use crate::{Blank, bytes::ToBytes, ids::*, V3};
use alloc::string::String;

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

