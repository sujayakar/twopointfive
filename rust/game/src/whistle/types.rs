//! The types of `whistle`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: WhistleMem, src/game/whistle.ts:75)
#[derive(Clone, Copy, Debug)]
pub struct WhistleMem {
    pub responded_at: f64,
    pub responses: f64,
    pub searches: f64,
    pub cues: f64,
    pub glances: f64,
    pub glance_until: f64,
    pub glance_at: Option<V3>,
    pub soft: bool,
    pub soft_at: f64,
    pub last_look: f64,
    pub last_twice: f64,
}
impl WhistleMem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WhistleMem {
            responded_at: 0.0,
            responses: 0.0,
            searches: 0.0,
            cues: 0.0,
            glances: 0.0,
            glance_until: 0.0,
            glance_at: None,
            soft: false,
            soft_at: 0.0,
            last_look: 0.0,
            last_twice: 0.0,
        }
    }
}
impl Default for WhistleMem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Blower, src/game/whistle.ts:99)
#[derive(Clone, Copy, Debug)]
pub struct Blower {
    pub ready_at: f64,
    pub blown: f64,
    pub last_at: f64,
}
impl Blower {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Blower {
            ready_at: 0.0,
            blown: 0.0,
            last_at: 0.0,
        }
    }
}
impl Default for Blower { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Heard, src/game/whistle.ts:127)
#[derive(Clone, Copy, Debug)]
pub struct Heard {
    pub d: f64,
    pub at: V3,
    pub direct: bool,
    pub carry: f64,
}
impl Heard {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Heard {
            d: 0.0,
            at: [0.0; 3],
            direct: false,
            carry: 0.0,
        }
    }
}
impl Default for Heard { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Resolution, src/game/whistle.ts:128)
#[derive(Clone, Debug)]
pub struct Resolution {
    pub hearers: JsMap<GuardId, crate::whistle::Heard>,
    pub responder: Option<GuardId>,
    pub answered_by: Option<GuardId>,
    pub t: f64,
}
impl Resolution {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Resolution {
            hearers: JsMap::new(),
            responder: None,
            answered_by: None,
            t: 0.0,
        }
    }
}
impl Default for Resolution { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: WhistleSnap, src/game/whistle.ts:244)
#[derive(Clone, Debug)]
pub struct WhistleSnap {
    pub blower: crate::whistle::Blower,
    pub men: Vec<Option<crate::whistle::WhistleMem>>,
}
impl WhistleSnap {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WhistleSnap {
            blower: crate::whistle::Blower::zeroed(),
            men: Vec::new(),
        }
    }
}
impl Default for WhistleSnap { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/whistle.ts:131 WHISTLE_LOG, src/game/whistle.ts:131)
#[derive(Clone, Debug)]
pub struct WhistleModWHISTLELOGItem {
    pub t: f64,
    pub pos: V3,
    pub hearers: Vec<String>,
    pub responder: Option<String>,
    pub answered: Option<String>,
    pub search: bool,
}
impl WhistleModWHISTLELOGItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WhistleModWHISTLELOGItem {
            t: 0.0,
            pos: [0.0; 3],
            hearers: Vec::new(),
            responder: None,
            answered: None,
            search: false,
        }
    }
}
impl Default for WhistleModWHISTLELOGItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum WhistleLinesKind {
    All,
    Look,
    Twice,
}
impl WhistleLinesKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            WhistleLinesKind::All => "all",
            WhistleLinesKind::Look => "look",
            WhistleLinesKind::Twice => "twice",
        }
    }
}

/// (TypeScript: inline at src/game/whistle.ts:96 whistleMem ->, ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1597)
#[derive(Clone, Copy, Debug)]
pub struct WhistleMem2 {
    pub responded_at: f64,
    pub responses: f64,
    pub searches: f64,
    pub cues: f64,
    pub glances: f64,
    pub glance_until: f64,
    pub glance_at: Option<V3>,
    pub soft: bool,
    pub soft_at: f64,
    pub last_look: f64,
    pub last_twice: f64,
}
impl WhistleMem2 {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WhistleMem2 {
            responded_at: 0.0,
            responses: 0.0,
            searches: 0.0,
            cues: 0.0,
            glances: 0.0,
            glance_until: 0.0,
            glance_at: None,
            soft: false,
            soft_at: 0.0,
            last_look: 0.0,
            last_twice: 0.0,
        }
    }
}
impl Default for WhistleMem2 { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum PickKey {
    LastLook,
    LastTwice,
}
impl PickKey {
    pub const fn as_str(self) -> &'static str {
        match self {
            PickKey::LastLook => "lastLook",
            PickKey::LastTwice => "lastTwice",
        }
    }
}

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct WhistleMod {
    /// (src/game/whistle.ts:87 memOf)
    pub mem_of: JsMap<GuardId, crate::whistle::WhistleMem>,
    /// (src/game/whistle.ts:100 blowerOf)
    pub blower_of: JsMap<PlayerId, crate::whistle::Blower>,
    /// (src/game/whistle.ts:129 resOf)
    pub res_of: JsMap<u32, crate::whistle::Resolution>,
    /// (src/game/whistle.ts:131 WHISTLE_LOG)
    pub whistle_log: Vec<crate::whistle::WhistleModWHISTLELOGItem>,
}
impl WhistleMod {
    pub fn blank() -> Self {
        WhistleMod {
            mem_of: JsMap::new(),
            blower_of: JsMap::new(),
            res_of: JsMap::new(),
            whistle_log: Vec::new(),
        }
    }
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct WhistleStats {
    pub blown: i32,
    pub resolved: i32,
    pub heard_by: i32,
    pub answered: i32,
    pub searched: i32,
    pub cued: i32,
    pub glanced: i32,
    pub unanswered: i32,
}

/// (src/game/whistle.ts:54 WHISTLE_DEFAULTS: a table nothing writes)
pub fn whistle_defaults() -> crate::knobs::WhistleK { crate::knobs::WhistleK { level: 0.35, reach_k: 8.0, horizon: 12.0, cooldown: 2.0, twice_window: 60.0, awareness: 0.45, react_t: 0.5, walk_floor: 0.32, glance_secs: 1.5, glance_max: 1.2, gain: 0.55 } }

/// (src/game/whistle.ts:59 WHISTLE_SOFT_UPPER)
pub const WHISTLE_SOFT_UPPER: &str = "torch";

/// (src/game/whistle.ts:62 LOOK_LINES: a table nothing writes)
pub fn look_lines() -> Vec<String> { alloc::vec![String::from("hello?"), String::from("someone there?"), String::from("…what was that?"), String::from("who's that whistling?"), String::from("somebody down there?")] }

/// (src/game/whistle.ts:63 TWICE_LINES: a table nothing writes)
pub fn twice_lines() -> Vec<String> { alloc::vec![String::from("right — who's messing about?"), String::from("that's twice now. come on out."), String::from("okay. not funny. where are you?")] }

/// (src/game/whistle.ts:65 TWICE_NOW: a table nothing writes)
pub fn twice_now() -> Vec<String> { alloc::vec![String::from("all right — who's messing around?"), String::from("that's twice now. come on out."), String::from("okay. not funny. where are you?")] }

/// (src/game/whistle.ts:67 LOOK_FOE: a table nothing writes)
pub fn look_foe() -> Vec<String> { alloc::vec![String::from("…quiet. listen."), String::from("you hear that?"), String::from("something's there.")] }

/// (src/game/whistle.ts:68 TWICE_FOE: a table nothing writes)
pub fn twice_foe() -> Vec<String> { alloc::vec![String::from("that's no bird. going to look."), String::from("someone's close. eyes up.")] }

/// (src/game/whistle.ts:74 NEVER)
pub const NEVER: f64 = -1000000000.0;

/// (src/game/whistle.ts:225 LS_KEY)
pub const LS_KEY: &str = "twopointsix.whistle";

/// (src/game/whistle.ts:226 KEPT: a table nothing writes)
pub fn kept() -> Vec<String> { alloc::vec![String::from("level"), String::from("reachK"), String::from("cooldown"), String::from("twiceWindow")] }
