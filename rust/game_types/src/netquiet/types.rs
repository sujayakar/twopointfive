//! The types of `netquiet`.
#![allow(unused_imports)]
use crate::{Blank, Words, bytes::ToBytes, ids::*, js::*, Quat, V3};
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
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct NetquietMod {
    pub state_of: JsMap<GuardId, crate::netquiet::Mind>,
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct NetquietStats {
    pub falls: i32,
}

pub fn netquiet_qa() -> crate::netquiet::NETQUIETQA { crate::netquiet::NETQUIETQA { broken: false } }

pub fn lines() -> JsMap<String, Vec<String>> { { let mut t = JsMap::new(); t.set(String::from("name"), alloc::vec![String::from("{name}. … {name}."), String::from("{name}? … {name}."), String::from("{name}. {name}, answer up.")]); t.set(String::from("why"), alloc::vec![String::from("why's nobody answering?"), String::from("where is everybody? answer me."), String::from("that's another one of you not answering. come on.")]); t.set(String::from("last"), alloc::vec![String::from("somebody talk to me!"), String::from("anybody. anybody at all, talk to me!"), String::from("is anyone left on this net?")]); t.set(String::from("body"), alloc::vec![String::from("he's down. nobody heard a thing?"), String::from("man down here. not a sound out of him."), String::from("he's down, and nobody heard it?")]); t.set(String::from("alone"), alloc::vec![String::from("my whole squad's quiet. it's just me in here."), String::from("nobody left in here but me."), String::from("it's gone quiet in here. i'm on my own.")]); t.set(String::from("ask"), alloc::vec![String::from("all posts, answer up."), String::from("anyone on this net, come in."), String::from("who's still up? sound off.")]); t } }

pub const NONE: f64 = 0.0;

pub const SAID: f64 = 1.0;

pub const BACK: f64 = 2.0;

pub const LOW: f64 = 3.0;

pub const DONE: f64 = 4.0;

