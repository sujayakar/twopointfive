//! The types of `sneak`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, ToBytes, Default)]
pub enum Why {
    #[default] Door,
    Near,
    Open,
    Lit,
    Long,
}
impl Why {
    pub const fn as_str(self) -> &'static str {
        match self {
            Why::Door => "door",
            Why::Near => "near",
            Why::Open => "open",
            Why::Lit => "lit",
            Why::Long => "long",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, ToBytes, Default)]
pub enum Mode {
    #[default] Cross,
    Back,
    Wait,
    Stand,
    Walk,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct Crumb {
    pub p: V3,
    pub lit: f64,
    pub t: f64,
    pub low: bool,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Man {
    pub i: f64,
    pub t: f64,
    pub lost_t: f64,
    pub mode: crate::sneak::Mode,
    pub plan_t: f64,
    pub wait_t: f64,
    pub why: crate::sneak::Why,
    pub nagged: bool,
    pub cue_t: f64,
    pub voice_t: f64,
    pub back_t: f64,
    pub hide_t: f64,
    pub hid_i: f64,
    pub mem: Option<crate::sneak::ManMem>,
    pub cross_from: f64,
    pub cross_to: f64,
    pub cross_t: f64,
    pub stand: f64,
    pub side: Option<V3>,
    pub side_t: f64,
    pub way: Option<Vec<V3>>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct ManMem {
    pub h: GuardId,
    pub a: f64,
    pub b: f64,
    pub t: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct State {
    pub trail: Vec<crate::sneak::Crumb>,
    pub off: f64,
    pub t: f64,
    pub men: JsMap<GuardId, crate::sneak::Man>,
    pub live: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Exp {
    pub seen: bool,
    pub watched: bool,
    pub near: bool,
    pub by: Option<GuardId>,
}

#[derive(Clone, Debug, Default)]
pub struct SneakWalk {
    pub by: Option<GuardId>,
    /// INVENTED by a review: see the proposals
    pub gd: GuardId,
    /// INVENTED by a review: see the proposals
    pub low: bool,
    /// INVENTED by a review: see the proposals
    pub keen: f64,
    /// INVENTED by a review: see the proposals
    pub card: f64,
    /// INVENTED by a review: see the proposals
    pub reach: f64,
    /// INVENTED by a review: see the proposals
    pub men: Vec<crate::sneak::SneakWalkMan>,
    /// INVENTED by a review: see the proposals
    pub memo: JsMap<crate::Num, bool>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Feet {
    pub goal: Option<V3>,
    pub speed: f64,
    pub crouch: bool,
    pub must: Option<bool>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct SneakModSNEAKLOGItem {
    pub t: f64,
    pub who: String,
    pub what: String,
    pub at: [f64; 2],
}

#[derive(Clone, Debug, Default)]
pub struct LINES {
    pub wait: crate::sneak::LINESWait,
    pub nag: crate::sneak::LINESNag,
    pub go: String,
}

#[derive(Clone, Debug, Default)]
pub struct LINESWait {
    pub lit: String,
    pub near: String,
    pub long: String,
    pub open: String,
    pub door: String,
}

#[derive(Clone, Debug, Default)]
pub struct LINESNag {
    pub lit: String,
    pub near: String,
    pub long: String,
    pub open: String,
    pub door: String,
}

#[derive(Clone, Debug, Default)]
pub struct LINESNOW {
    pub wait: crate::sneak::LINESNOWWait,
    pub nag: crate::sneak::LINESNOWNag,
}

#[derive(Clone, Debug, Default)]
pub struct LINESNOWWait {
    pub door: Option<String>,
    pub near: Option<String>,
    pub open: Option<String>,
    pub lit: Option<String>,
    pub long: Option<String>,
}

#[derive(Clone, Debug, Default)]
pub struct LINESNOWNag {
    pub door: Option<String>,
    pub near: Option<String>,
    pub open: Option<String>,
    pub lit: Option<String>,
    pub long: Option<String>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct SneakModLeafTValue {
    pub a: f64,
    pub t: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct SneakModLiveAtValue {
    pub t: f64,
    pub v: bool,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct SneakModKeptLinesValue {
    pub x: f64,
    pub z: f64,
    pub t: f64,
    pub low: bool,
    pub map: JsMap<crate::Num, f64>,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes)]
pub struct SneakMod {
    /// (src/game/sneak.ts:154 SNEAK_LOG)
    pub sneak_log: Vec<crate::sneak::SneakModSNEAKLOGItem>,
    /// (src/game/sneak.ts:188 leafT)
    pub leaf_t: JsMap<DoorId, crate::sneak::SneakModLeafTValue>,
    /// (src/game/sneak.ts:193 states)
    pub states: Option<crate::sneak::State>,
    /// (src/game/sneak.ts:199 liveAt)
    pub live_at: Option<crate::sneak::SneakModLiveAtValue>,
    /// (src/game/sneak.ts:324 cellLit)
    pub cell_lit: Option<JsMap<crate::Num, [f64; 2]>>,
    /// (src/game/sneak.ts:325 keptLines)
    pub kept_lines: JsMap<GuardId, crate::sneak::SneakModKeptLinesValue>,
}
impl SneakMod {
    pub fn blank() -> Self {
        SneakMod {
            sneak_log: Vec::new(),
            leaf_t: JsMap::new(),
            states: None,
            live_at: None,
            cell_lit: None,
            kept_lines: JsMap::new(),
        }
    }
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct SneakStats {
    pub waits: i32,
    pub wait_s: f64,
    pub crossings: i32,
    pub shade: i32,
    pub back_offs: i32,
    pub asides: i32,
    pub cues: i32,
    pub noise_walks: i32,
}

/// (src/game/sneak.ts:157 LINES: a table nothing writes)
pub fn lines() -> crate::sneak::LINES { crate::sneak::LINES { wait: crate::sneak::LINESWait { lit: String::from("holding. it's lit and he's looking."), near: String::from("holding. too close while he's looking."), long: String::from("holding. too much open light to cross."), open: String::from("holding in the dark. you're stood in the light."), door: String::from("i'm off the door. ready when you are.") }, nag: crate::sneak::LINESNag { lit: String::from("can't cross while it's lit. he's still on it."), near: String::from("he's still facing me. waiting for his back."), long: String::from("it's a long lit stretch. kill the light or go on."), open: String::from("you're stood in the light. i'll hold in the dark."), door: String::from("") }, go: String::from("moving.") } }

/// (src/game/sneak.ts:165 LINES_NOW: a table nothing writes)
pub fn lines_now() -> crate::sneak::LINESNOW { crate::sneak::LINESNOW { wait: crate::sneak::LINESNOWWait { door: None, near: None, open: Some(String::from("holding in the dark. you're standing in the light.")), lit: None, long: None }, nag: crate::sneak::LINESNOWNag { door: None, near: Some(String::from("he's still facing me. waiting for him to turn.")), open: Some(String::from("you're standing in the light. i'll hold in the dark.")), lit: None, long: None } } }

/// (src/game/sneak.ts:195 DEG)
pub const DEG: f64 = 0.017453292519943295;

/// (src/game/sneak.ts:251 LOW_Y: a table nothing writes)
pub fn low_y() -> Vec<f64> { alloc::vec![0.81, 0.98] }

/// (src/game/sneak.ts:251 UP_Y: a table nothing writes)
pub fn up_y() -> Vec<f64> { alloc::vec![1.27, 1.5] }

/// (src/game/sneak.ts:268 NONE: a table nothing writes)
pub fn none() -> crate::sneak::Exp { crate::sneak::Exp { seen: false, watched: false, near: false, by: None } }

/// (src/game/sneak.ts:297 MAP_SHORT)
pub const MAP_SHORT: f64 = 0.05;

/// INVENTED: what the eye test is asked about: a spot a man could stand on, or a man's own body (sneak.ts:266 standIn, :284 exposureOf)
#[derive(Clone, Copy, Debug)]
pub enum Target { Spot { chest: V3, head: V3 }, Man { c: CharId } }
/// INVENTED: one defender in a SneakWalk (sneak.ts:352, :362). His `kept` lines stay in g.m.sneak.kept_lines, read by his id
#[derive(Clone, Debug)]
pub struct SneakWalkMan { pub h: GuardId, pub x: f64, pub z: f64, pub open: crate::guards::Eye, pub looks: Vec<crate::sneak::SneakWalkLook>, pub line: JsMap<crate::Num, f64> }
/// INVENTED: one way a defender looks in a SneakWalk, as he looks now or as he mostly looks, each also WIDER (sneak.ts:352, :359)
#[derive(Clone, Copy, Debug)]
pub struct SneakWalkLook { pub e: crate::guards::Eye, pub ew: crate::guards::Eye, pub lx: f64, pub lz: f64 }
