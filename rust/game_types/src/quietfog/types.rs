//! The types of `quietfog`.
#![allow(unused_imports)]
use crate::{Blank, bytes::ToBytes, Words, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default, Words)]
pub enum QuietRung {
    #[default] Team,
    Own,
    Off,
    Cones,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct Held {
    pub cone: Option<crate::quietfog::HeldCone>,
    pub own: Option<crate::quietfog::HeldOwn>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct HeldCone {
    pub was: Option<bool>,
    pub put: bool,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct HeldOwn {
    pub was: bool,
    pub put: bool,
}

#[derive(Clone, Debug, Default)]
pub struct QUIETRUNGSItem {
    pub id: String,
    pub words: String,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct QuietfogMod {
    /// (src/game/quietfog.ts:69 PICK)
    pub pick: Option<crate::quietfog::QuietRung>,
    /// (src/game/quietfog.ts:70 PICK_DEF)
    #[is(crate::quietfog::QuietRung::Off)] pub pick_def: crate::quietfog::QuietRung,
    /// (src/game/quietfog.ts:80 KEPT)
    pub kept: bool,
    /// (src/game/quietfog.ts:86 CAMS)
    pub cams: Option<bool>,
    /// (src/game/quietfog.ts:87 CAMS_READ)
    pub cams_read: bool,
    /// (src/game/quietfog.ts:148 HELD)
    pub held: Option<crate::quietfog::Held>,
}

/// (src/game/quietfog.ts:48 NOT_SOLO: a table nothing writes)
pub fn not_solo() -> Vec<String> { alloc::vec![String::from("quiet"), String::from("roomclear"), String::from("nightbreach"), String::from("rescue"), String::from("squad4"), String::from("demo"), String::from("breach")] }

/// (src/game/quietfog.ts:58 QUIET_RUNGS: a table nothing writes)
pub fn quiet_rungs() -> Vec<crate::quietfog::QUIETRUNGSItem> { alloc::vec![crate::quietfog::QUIETRUNGSItem { id: String::from("off"), words: String::from("no fog of war: the whole floor and every guard on it is drawn") }, crate::quietfog::QUIETRUNGSItem { id: String::from("team"), words: String::from("you see all around you, and around your partner if you have one. The rest of the floor is drawn as you last saw it") }, crate::quietfog::QUIETRUNGSItem { id: String::from("cones"), words: String::from("you see only the cone you're looking down, and your partner's if you have one. The floor behind your back is dark") }, crate::quietfog::QUIETRUNGSItem { id: String::from("own"), words: String::from("only the cone you're looking down lifts the fog, and a partner calls out what he sees") }] }

/// (src/game/quietfog.ts:68 LS)
pub const LS: &str = "twopointsix.quietfog";

/// (src/game/quietfog.ts:68 PARAM)
pub const PARAM: &str = "quietfog";

/// (src/game/quietfog.ts:85 LS_CAMS)
pub const LS_CAMS: &str = "twopointsix.quietcams";

/// (src/game/quietfog.ts:85 PARAM_CAMS)
pub const PARAM_CAMS: &str = "cams";
