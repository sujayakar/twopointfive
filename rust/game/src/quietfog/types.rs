//! The types of `quietfog`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum QuietRung {
    Team,
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

/// (TypeScript: Held, src/game/quietfog.ts:147)
#[derive(Clone, Copy, Debug)]
pub struct Held {
    pub cone: Option<crate::quietfog::HeldCone>,
    pub own: Option<crate::quietfog::HeldOwn>,
}
impl Held {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Held {
            cone: None,
            own: None,
        }
    }
}
impl Default for Held { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/quietfog.ts:147 Held.cone, src/game/quietfog.ts:147)
#[derive(Clone, Copy, Debug)]
pub struct HeldCone {
    pub was: Option<bool>,
    pub put: bool,
}
impl HeldCone {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        HeldCone {
            was: None,
            put: false,
        }
    }
}
impl Default for HeldCone { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/quietfog.ts:147 Held.own, src/game/quietfog.ts:147)
#[derive(Clone, Copy, Debug)]
pub struct HeldOwn {
    pub was: bool,
    pub put: bool,
}
impl HeldOwn {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        HeldOwn {
            was: false,
            put: false,
        }
    }
}
impl Default for HeldOwn { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/quietfog.ts:58 QUIET_RUNGS, src/game/quietfog.ts:58)
#[derive(Clone, Debug)]
pub struct QUIETRUNGSItem {
    pub id: String,
    pub label: String,
    pub words: String,
}
impl QUIETRUNGSItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        QUIETRUNGSItem {
            id: String::new(),
            label: String::new(),
            words: String::new(),
        }
    }
}
impl Default for QUIETRUNGSItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/quietfog.ts:123 quietFogRow(e), src/game/quietfog.ts:123)
#[derive(Clone, Debug)]
pub struct QuietFogRowE {
    pub kind: String,
    pub group: Option<String>,
}
impl QuietFogRowE {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        QuietFogRowE {
            kind: String::new(),
            group: None,
        }
    }
}
impl Default for QuietFogRowE { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct QuietfogMod {
    /// (src/game/quietfog.ts:69 PICK)
    pub pick: Option<crate::quietfog::QuietRung>,
    /// (src/game/quietfog.ts:70 PICK_DEF)
    pub pick_def: crate::quietfog::QuietRung,
    /// (src/game/quietfog.ts:80 KEPT)
    pub kept: bool,
    /// (src/game/quietfog.ts:86 CAMS)
    pub cams: Option<bool>,
    /// (src/game/quietfog.ts:87 CAMS_READ)
    pub cams_read: bool,
    /// (src/game/quietfog.ts:148 HELD)
    pub held: Option<crate::quietfog::Held>,
    /// (src/game/quietfog.ts:174 ASIDE)
    pub aside: bool,
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
            aside: false,
        }
    }
}

/// (src/game/quietfog.ts:48 NOT_SOLO: a table nothing writes)
pub fn not_solo() -> Vec<String> { alloc::vec![String::from("quiet"), String::from("roomclear"), String::from("nightbreach"), String::from("rescue"), String::from("squad4"), String::from("demo"), String::from("breach")] }

/// (src/game/quietfog.ts:58 QUIET_RUNGS: a table nothing writes)
pub fn quiet_rungs() -> Vec<crate::quietfog::QUIETRUNGSItem> { alloc::vec![crate::quietfog::QUIETRUNGSItem { id: String::from("off"), label: String::from("off"), words: String::from("no fog of war: the whole floor and every guard on it is drawn") }, crate::quietfog::QUIETRUNGSItem { id: String::from("team"), label: String::from("all round"), words: String::from("you see all around you, and around your partner if you have one. The rest of the floor is drawn as you last saw it") }, crate::quietfog::QUIETRUNGSItem { id: String::from("cones"), label: String::from("cones"), words: String::from("you see only the cone you're looking down, and your partner's if you have one. The floor behind your back is dark") }, crate::quietfog::QUIETRUNGSItem { id: String::from("own"), label: String::from("own eyes"), words: String::from("only the cone you're looking down lifts the fog, and a partner calls out what he sees") }] }

/// (src/game/quietfog.ts:68 LS)
pub const LS: &str = "twopointsix.quietfog";

/// (src/game/quietfog.ts:68 PARAM)
pub const PARAM: &str = "quietfog";

/// (src/game/quietfog.ts:85 LS_CAMS)
pub const LS_CAMS: &str = "twopointsix.quietcams";

/// (src/game/quietfog.ts:85 PARAM_CAMS)
pub const PARAM_CAMS: &str = "cams";
