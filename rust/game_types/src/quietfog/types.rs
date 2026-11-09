//! The types of `quietfog`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum QuietRung {
    #[default] Team,
    Own,
    Off,
    Cones,
}
impl QuietRung {
    pub const fn as_str(self) -> &'static str {
        match self {
            QuietRung::Team => "team",
            QuietRung::Own => "own",
            QuietRung::Off => "off",
            QuietRung::Cones => "cones",
        }
    }
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
#[derive(Clone, Debug, ToBytes)]
pub struct QuietfogMod {
    pub pick: Option<crate::quietfog::QuietRung>,
    pub pick_def: crate::quietfog::QuietRung,
    pub kept: bool,
    pub cams: Option<bool>,
    pub cams_read: bool,
    pub held: Option<crate::quietfog::Held>,
}
impl QuietfogMod {
    pub fn blank() -> Self {
        QuietfogMod {
            pick: None,
            pick_def: crate::quietfog::QuietRung::Off,
            kept: false,
            cams: None,
            cams_read: false,
            held: None,
        }
    }
}

pub fn not_solo() -> Vec<String> { alloc::vec![String::from("quiet"), String::from("roomclear"), String::from("nightbreach"), String::from("rescue"), String::from("squad4"), String::from("demo"), String::from("breach")] }

pub fn quiet_rungs() -> Vec<crate::quietfog::QUIETRUNGSItem> { alloc::vec![crate::quietfog::QUIETRUNGSItem { id: String::from("off"), words: String::from("no fog of war: the whole floor and every guard on it is drawn") }, crate::quietfog::QUIETRUNGSItem { id: String::from("team"), words: String::from("you see all around you, and around your partner if you have one. The rest of the floor is drawn as you last saw it") }, crate::quietfog::QUIETRUNGSItem { id: String::from("cones"), words: String::from("you see only the cone you're looking down, and your partner's if you have one. The floor behind your back is dark") }, crate::quietfog::QUIETRUNGSItem { id: String::from("own"), words: String::from("only the cone you're looking down lifts the fog, and a partner calls out what he sees") }] }

pub const LS: &str = "twopointsix.quietfog";

pub const PARAM: &str = "quietfog";

pub const LS_CAMS: &str = "twopointsix.quietcams";

pub const PARAM_CAMS: &str = "cams";
