//! The types of `scene_look`.
#![allow(unused_imports)]
use crate::{Blank, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes)]
pub enum SceneLookModSuitAsked {
    Quiet,
    /// the men are dressed as no mission's: Sam alone in the suit
    NoMission,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum SuitAs {
    Quiet,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct SceneLookMod {
    pub suit_asked: Option<crate::scene_look::SceneLookModSuitAsked>,
    #[is(true)] pub look_place: bool,
}

