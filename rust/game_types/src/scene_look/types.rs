//! The types of `scene_look`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum SceneLookModSuitAsked {
    Quiet,
}
impl SceneLookModSuitAsked {
    pub const fn as_str(self) -> &'static str {
        match self {
            SceneLookModSuitAsked::Quiet => "quiet",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum SuitAs {
    Quiet,
}
impl SuitAs {
    pub const fn as_str(self) -> &'static str {
        match self {
            SuitAs::Quiet => "quiet",
        }
    }
}

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct SceneLookMod {
    /// (src/scene/look.ts:24 suitAsked)
    pub suit_asked: Option<crate::scene_look::SceneLookModSuitAsked>,
    /// (src/scene/look.ts:35 lookPlace)
    pub look_place: bool,
}
impl SceneLookMod {
    pub fn blank() -> Self {
        SceneLookMod {
            suit_asked: None,
            look_place: true,
        }
    }
}

