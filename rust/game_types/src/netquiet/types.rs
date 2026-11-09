//! The types of `netquiet`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, Words, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, ToBytes, Default, Words)]
pub enum LineKind {
    #[default] Body,
    Name,
    Last,
    Why,
    Alone,
    Ask,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Mind {
    pub quiet: f64,
    pub asked: JsSet<crate::Num>,
    pub known: JsSet<crate::Num>,
    pub paid: JsMap<crate::Num, f64>,
    pub say: f64,
    pub say_t: f64,
    pub say_name: String,
    pub bark: f64,
    pub last_kind: f64,
    pub body_t: f64,
    pub roused: f64,
    pub heard_t: f64,
    pub cs: f64,
    pub cl: f64,
    pub fx: f64,
    pub fz: f64,
    pub whip_at: Option<V3>,
    pub whip_in: f64,
    pub whip_t: f64,
    pub whip_held: f64,
    pub last: f64,
    pub t: f64,
    pub mate_t: f64,
    pub news: f64,
    pub news_t: f64,
    pub eye: f64,
    pub eye_half: f64,
    pub spot: Option<V3>,
    pub yaw: f64,
    pub arc: f64,
    pub look: Option<V3>,
    pub low: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct NETQUIETQA {
    pub broken: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ArcAt {
    pub seen: f64,
    pub arc: f64,
    pub yaw: f64,
    pub near: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct NetQuietPose {
    pub aim: Option<V3>,
    pub crouch: f64,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes)]
pub struct NetquietMod {
    /// (src/game/netquiet.ts:125 stateOf)
    pub state_of: JsMap<GuardId, crate::netquiet::Mind>,
}
impl NetquietMod {
    pub fn blank() -> Self {
        NetquietMod {
            state_of: JsMap::new(),
        }
    }
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct NetquietStats {
    pub asks: i32,
    pub quiet: i32,
    pub bodies: i32,
    pub falls: i32,
    pub unseen: i32,
    pub lines: i32,
    pub whips: i32,
    pub last: i32,
    pub backs: i32,
    pub low: i32,
    pub self_asks: i32,
    pub released: i32,
    pub eased: i32,
    pub squad_asks: i32,
    pub squad_bodies: i32,
    pub stand_checks: i32,
    pub staff_asks: i32,
    pub staff_bodies: i32,
}

/// (src/game/netquiet.ts:93 NETQUIET_QA: a table nothing writes)
pub fn netquiet_qa() -> crate::netquiet::NETQUIETQA { crate::netquiet::NETQUIETQA { broken: false } }

/// (src/game/netquiet.ts:100 KINDS: a table nothing writes)
pub fn kinds() -> Vec<String> { alloc::vec![String::from("name"), String::from("why"), String::from("last"), String::from("body"), String::from("alone"), String::from("ask")] }

/// (src/game/netquiet.ts:101 LINES: a table nothing writes)
pub fn lines() -> JsMap<String, Vec<String>> { { let mut t = JsMap::new(); t.set(String::from("name"), alloc::vec![String::from("{name}. … {name}."), String::from("{name}? … {name}."), String::from("{name}. {name}, answer up.")]); t.set(String::from("why"), alloc::vec![String::from("why's nobody answering?"), String::from("where is everybody? answer me."), String::from("that's another of you quiet. come on.")]); t.set(String::from("last"), alloc::vec![String::from("somebody talk to me!"), String::from("anybody. anybody at all, talk to me!"), String::from("is anyone left on this net?")]); t.set(String::from("body"), alloc::vec![String::from("he's down. nobody heard a thing?"), String::from("man down here. not a sound out of him."), String::from("he's down, and nobody heard it?")]); t.set(String::from("alone"), alloc::vec![String::from("that's my whole squad quiet. it's just me in here."), String::from("nobody left in here but me."), String::from("it's gone quiet in here. i'm on my own.")]); t.set(String::from("ask"), alloc::vec![String::from("all posts, answer up."), String::from("anyone on this net, come in."), String::from("who's still up? sound off.")]); t } }

/// (src/game/netquiet.ts:110 LINES_NOW: a table nothing writes)
pub fn lines_now() -> JsMap<String, String> { { let mut t = JsMap::new(); t.set(String::from("that's another of you quiet. come on."), String::from("that's another one of you not answering. come on.")); t.set(String::from("that's my whole squad quiet. it's just me in here."), String::from("my whole squad's quiet. it's just me in here.")); t } }

/// (src/game/netquiet.ts:115 NONE)
pub const NONE: f64 = 0.0;

/// (src/game/netquiet.ts:115 SAID)
pub const SAID: f64 = 1.0;

/// (src/game/netquiet.ts:115 BACK)
pub const BACK: f64 = 2.0;

/// (src/game/netquiet.ts:115 LOW)
pub const LOW: f64 = 3.0;

/// (src/game/netquiet.ts:115 DONE)
pub const DONE: f64 = 4.0;

