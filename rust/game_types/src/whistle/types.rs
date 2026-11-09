//! The types of `whistle`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, ToBytes, Default)]
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

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct Blower {
    pub ready_at: f64,
    pub blown: f64,
    pub last_at: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct Heard {
    pub d: f64,
    pub at: V3,
    pub direct: bool,
    pub carry: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Resolution {
    pub hearers: JsMap<GuardId, crate::whistle::Heard>,
    pub responder: Option<GuardId>,
    pub answered_by: Option<GuardId>,
    pub t: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct WhistleModWHISTLELOGItem {
    pub t: f64,
    pub pos: V3,
    pub hearers: Vec<String>,
    pub responder: Option<String>,
    pub answered: Option<String>,
    pub search: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum PickKey {
    LastLook,
    LastTwice,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes)]
pub struct WhistleMod {
    /// (src/game/whistle.ts:88 memOf)
    pub mem_of: JsMap<GuardId, crate::whistle::WhistleMem>,
    /// (src/game/whistle.ts:101 blowerOf)
    pub blower_of: JsMap<PlayerId, crate::whistle::Blower>,
    /// (src/game/whistle.ts:130 resOf)
    pub res_of: JsMap<u32, crate::whistle::Resolution>,
    /// (src/game/whistle.ts:132 WHISTLE_LOG)
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

/// (src/game/whistle.ts:63 LOOK_LINES: a table nothing writes)
pub fn look_lines() -> Vec<String> { alloc::vec![String::from("hello?"), String::from("someone there?"), String::from("…what was that?"), String::from("who's that whistling?"), String::from("somebody down there?")] }

/// (src/game/whistle.ts:66 TWICE_NOW: a table nothing writes)
pub fn twice_now() -> Vec<String> { alloc::vec![String::from("all right — who's messing around?"), String::from("that's twice now. come on out."), String::from("okay. not funny. where are you?")] }

/// (src/game/whistle.ts:68 LOOK_FOE: a table nothing writes)
pub fn look_foe() -> Vec<String> { alloc::vec![String::from("…quiet. listen."), String::from("you hear that?"), String::from("something's there.")] }

/// (src/game/whistle.ts:69 TWICE_FOE: a table nothing writes)
pub fn twice_foe() -> Vec<String> { alloc::vec![String::from("that's no bird. going to look."), String::from("someone's close. eyes up.")] }

/// (src/game/whistle.ts:75 NEVER)
pub const NEVER: f64 = -1000000000.0;

