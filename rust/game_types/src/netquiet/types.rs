//! The types of `netquiet`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum LineKind {
    Body,
    Name,
    Last,
    Why,
    Alone,
    Ask,
}
impl LineKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            LineKind::Body => "body",
            LineKind::Name => "name",
            LineKind::Last => "last",
            LineKind::Why => "why",
            LineKind::Alone => "alone",
            LineKind::Ask => "ask",
        }
    }
}

/// (TypeScript: Mind, src/game/netquiet.ts:122)
#[derive(Clone, Debug)]
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
impl Mind {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Mind {
            quiet: 0.0,
            asked: JsSet::new(),
            known: JsSet::new(),
            paid: JsMap::new(),
            say: 0.0,
            say_t: 0.0,
            say_name: String::new(),
            bark: 0.0,
            last_kind: 0.0,
            body_t: 0.0,
            roused: 0.0,
            heard_t: 0.0,
            cs: 0.0,
            cl: 0.0,
            fx: 0.0,
            fz: 0.0,
            whip_at: None,
            whip_in: 0.0,
            whip_t: 0.0,
            whip_held: 0.0,
            last: 0.0,
            t: 0.0,
            mate_t: 0.0,
            news: 0.0,
            news_t: 0.0,
            eye: 0.0,
            eye_half: 0.0,
            spot: None,
            yaw: 0.0,
            arc: 0.0,
            look: None,
            low: false,
        }
    }
}
impl Default for Mind { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: NetQuietSnap, src/game/netquiet.ts:495)
#[derive(Clone, Debug)]
pub struct NetQuietSnap {
    pub men: Vec<Option<Vec<f64>>>,
    pub pts: Vec<Option<Vec<f64>>>,
    pub names: Vec<Option<String>>,
    pub asked: Vec<Option<Vec<f64>>>,
    pub known: Option<Vec<Option<Vec<f64>>>>,
    pub paid: Option<Vec<Option<Vec<f64>>>>,
}
impl NetQuietSnap {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        NetQuietSnap {
            men: Vec::new(),
            pts: Vec::new(),
            names: Vec::new(),
            asked: Vec::new(),
            known: None,
            paid: None,
        }
    }
}
impl Default for NetQuietSnap { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/netquiet.ts:89 NETQUIET_LOG, src/game/netquiet.ts:89)
#[derive(Clone, Debug)]
pub struct NetquietModNETQUIETLOGItem {
    pub id: f64,
    pub t: f64,
    pub kind: crate::netquiet::LineKind,
    pub text: String,
    pub radio: bool,
}
impl NetquietModNETQUIETLOGItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        NetquietModNETQUIETLOGItem {
            id: 0.0,
            t: 0.0,
            kind: crate::netquiet::LineKind::Body,
            text: String::new(),
            radio: false,
        }
    }
}
impl Default for NetquietModNETQUIETLOGItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/netquiet.ts:93 NETQUIET_QA, src/game/netquiet.ts:93)
#[derive(Clone, Copy, Debug)]
pub struct NETQUIETQA {
    pub broken: bool,
}
impl NETQUIETQA {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        NETQUIETQA {
            broken: false,
        }
    }
}
impl Default for NETQUIETQA { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/netquiet.ts:262 arcAt ->, src/game/netquiet.ts:262)
#[derive(Clone, Copy, Debug)]
pub struct ArcAt {
    pub seen: f64,
    pub arc: f64,
    pub yaw: f64,
    pub near: f64,
}
impl ArcAt {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ArcAt {
            seen: 0.0,
            arc: 0.0,
            yaw: 0.0,
            near: 0.0,
        }
    }
}
impl Default for ArcAt { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/netquiet.ts:456 netQuietPose ->, src/game/netquiet.ts:456)
#[derive(Clone, Copy, Debug)]
pub struct NetQuietPose {
    pub aim: Option<V3>,
    pub crouch: f64,
}
impl NetQuietPose {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        NetQuietPose {
            aim: None,
            crouch: 0.0,
        }
    }
}
impl Default for NetQuietPose { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/netquiet.ts:469 netQuietSpot ->, src/game/netquiet.ts:469)
#[derive(Clone, Copy, Debug)]
pub struct NetQuietSpot {
    pub to: Option<V3>,
    pub arc: f64,
    pub seen: f64,
}
impl NetQuietSpot {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        NetQuietSpot {
            to: None,
            arc: 0.0,
            seen: 0.0,
        }
    }
}
impl Default for NetQuietSpot { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at quiet netQuietOf ->, ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1597)
#[derive(Clone, Debug)]
pub struct NetQuietOf {
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
impl NetQuietOf {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        NetQuietOf {
            quiet: 0.0,
            asked: JsSet::new(),
            known: JsSet::new(),
            paid: JsMap::new(),
            say: 0.0,
            say_t: 0.0,
            say_name: String::new(),
            bark: 0.0,
            last_kind: 0.0,
            body_t: 0.0,
            roused: 0.0,
            heard_t: 0.0,
            cs: 0.0,
            cl: 0.0,
            fx: 0.0,
            fz: 0.0,
            whip_at: None,
            whip_in: 0.0,
            whip_t: 0.0,
            whip_held: 0.0,
            last: 0.0,
            t: 0.0,
            mate_t: 0.0,
            news: 0.0,
            news_t: 0.0,
            eye: 0.0,
            eye_half: 0.0,
            spot: None,
            yaw: 0.0,
            arc: 0.0,
            look: None,
            low: false,
        }
    }
}
impl Default for NetQuietOf { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct NetquietMod {
    /// (src/game/netquiet.ts:89 NETQUIET_LOG)
    pub netquiet_log: Vec<crate::netquiet::NetquietModNETQUIETLOGItem>,
    /// (src/game/netquiet.ts:125 stateOf)
    pub state_of: JsMap<GuardId, crate::netquiet::Mind>,
}
impl NetquietMod {
    pub fn blank() -> Self {
        NetquietMod {
            netquiet_log: Vec::new(),
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

/// (src/game/netquiet.ts:484 VISIT)
pub const VISIT: &str = "twopointsix.netquiet.visit";
